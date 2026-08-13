use core::ffi::{c_int, c_uchar, c_uint, c_void};
use core::sync::atomic::{AtomicPtr, Ordering};
use core::{mem, ptr};

use super::InstrProfData::{
    __llvm_profile_data, getFirstValueProfRecord, getValueProfDataSize,
    getValueProfRecordHeaderSize, IPVK_First, IPVK_Last, InstrProfGetRangeRepValue,
    InstrProfValueData, ValueProfNode, ValueProfRecordClosure, INSTR_PROF_MAX_NUM_VAL_PER_SITE,
};
use super::InstrProfilingInternal::VPDataReaderType;
use super::InstrProfilingPlatform::{CurrentVNode, EndVNode};
use super::{minicov_alloc_zeroed, minicov_dealloc};

static mut hasStaticCounters: c_int = 1;
static mut OutOfNodesWarnings: c_int = 0;
static mut hasNonDefaultValsPerSite: c_int = 0;
pub const INSTR_PROF_DEFAULT_NUM_VAL_PER_SITE: c_int = 24;

#[cfg(not(target_env = "msvc"))]
#[used]
#[cfg_attr(
    not(any(target_vendor = "apple", target_os = "windows")),
    link_section = "__llvm_prf_vnds"
)]
#[cfg_attr(target_vendor = "apple", link_section = "__DATA,__llvm_prf_vnds")]
#[cfg_attr(target_os = "windows", link_section = ".lprfnd$M")]
pub static mut lprofValueProfNodes: [ValueProfNode; 1024] = [ValueProfNode {
    Value: 0,
    Count: 0,
    Next: ptr::null_mut(),
}; 1024];
pub static mut VPMaxNumValsPerSite: u32 = INSTR_PROF_DEFAULT_NUM_VAL_PER_SITE as u32;

pub unsafe fn __llvm_profile_set_num_value_sites(
    Data: *mut __llvm_profile_data,
    ValueKind: u32,
    NumValueSites: u16,
) {
    *(*Data)
        .0
        .NumValueSites
        .as_mut_ptr()
        .offset(ValueKind as isize) = NumValueSites;
}

pub unsafe fn __llvm_profile_iterate_data(
    Data: *const __llvm_profile_data,
) -> *const __llvm_profile_data {
    Data.offset(1)
}

pub unsafe fn __llvm_get_function_addr(Data: *const __llvm_profile_data) -> *mut c_void {
    (*Data).0.FunctionPointer
}

unsafe fn allocateValueProfileCounters(Data: *mut __llvm_profile_data) -> c_int {
    let mut NumVSites: u64 = 0;
    let mut VKI: u32;
    hasStaticCounters = 0;
    if hasNonDefaultValsPerSite == 0 {
        VPMaxNumValsPerSite = INSTR_PROF_MAX_NUM_VAL_PER_SITE as u32;
    }
    VKI = IPVK_First as c_int as u32;
    while VKI <= IPVK_Last as c_int as u32 {
        NumVSites = NumVSites.wrapping_add((*Data).0.NumValueSites[VKI as usize] as u64);
        VKI = VKI.wrapping_add(1);
    }
    let Size = NumVSites.wrapping_mul(mem::size_of::<*mut ValueProfNode>() as u64) as usize;
    let Alignment = mem::align_of::<*mut ValueProfNode>();
    let Mem = minicov_alloc_zeroed(Size, Alignment) as *mut *mut ValueProfNode;
    if Mem.is_null() {
        return 0;
    }
    if AtomicPtr::from_ptr(&raw mut (*Data).0.Values)
        .compare_exchange(
            ptr::null_mut(),
            Mem as *mut c_void,
            Ordering::SeqCst,
            Ordering::SeqCst,
        )
        .is_err()
    {
        minicov_dealloc(Mem as *mut u8, Size, Alignment);
        return 0;
    }
    1
}

unsafe fn allocateOneNode() -> *mut ValueProfNode {
    if hasStaticCounters == 0 {
        return minicov_alloc_zeroed(
            mem::size_of::<ValueProfNode>(),
            mem::align_of::<ValueProfNode>(),
        ) as *mut ValueProfNode;
    }
    if CurrentVNode.offset(1) > EndVNode {
        OutOfNodesWarnings += 1;
        return ptr::null_mut();
    }
    let Node = AtomicPtr::from_ptr(&raw mut CurrentVNode as *mut *mut isize).fetch_ptr_add(
        (mem::size_of::<ValueProfNode>()).wrapping_mul(1),
        Ordering::SeqCst,
    ) as *mut ValueProfNode;
    if Node.offset(1) > EndVNode {
        return ptr::null_mut();
    }
    Node
}

#[inline(always)]
unsafe fn instrumentTargetValueImpl(
    TargetValue: u64,
    Data: *mut c_void,
    CounterIndex: u32,
    CountValue: u64,
) {
    let PData = Data as *mut __llvm_profile_data;
    if PData.is_null() {
        return;
    }
    if CountValue == 0 {
        return;
    }
    if (*PData).0.Values.is_null() && allocateValueProfileCounters(PData) == 0 {
        return;
    }
    let ValueCounters = (*PData).0.Values as *mut *mut ValueProfNode;
    let mut PrevVNode: *mut ValueProfNode = ptr::null_mut();
    let mut MinCountVNode: *mut ValueProfNode = ptr::null_mut();
    let mut CurVNode = *ValueCounters.offset(CounterIndex as isize);
    let mut MinCount = u64::MAX;
    let mut VDataCount: u8 = 0;
    while !CurVNode.is_null() {
        if TargetValue == (*CurVNode).Value {
            (*CurVNode).Count = (*CurVNode).Count.wrapping_add(CountValue);
            return;
        }
        if (*CurVNode).Count < MinCount {
            MinCount = (*CurVNode).Count;
            MinCountVNode = CurVNode;
        }
        PrevVNode = CurVNode;
        CurVNode = (*CurVNode).Next;
        VDataCount = VDataCount.wrapping_add(1);
    }
    if VDataCount as u32 >= VPMaxNumValsPerSite {
        if (*MinCountVNode).Count <= CountValue {
            CurVNode = MinCountVNode;
            (*CurVNode).Value = TargetValue;
            (*CurVNode).Count = CountValue;
        } else {
            (*MinCountVNode).Count = (*MinCountVNode).Count.wrapping_sub(CountValue);
        }
        return;
    }
    CurVNode = allocateOneNode();
    if CurVNode.is_null() {
        return;
    }
    (*CurVNode).Value = TargetValue;
    (*CurVNode).Count = (*CurVNode).Count.wrapping_add(CountValue);
    let mut Success: u32 = 0;
    if (*ValueCounters.offset(CounterIndex as isize)).is_null() {
        Success = AtomicPtr::from_ptr(ValueCounters.offset(CounterIndex as isize))
            .compare_exchange(
                ptr::null_mut(),
                CurVNode,
                Ordering::SeqCst,
                Ordering::SeqCst,
            )
            .is_ok() as u32;
    } else if !PrevVNode.is_null() && (*PrevVNode).Next.is_null() {
        Success = AtomicPtr::from_ptr(&raw mut (*PrevVNode).Next)
            .compare_exchange(
                ptr::null_mut(),
                CurVNode,
                Ordering::SeqCst,
                Ordering::SeqCst,
            )
            .is_ok() as u32;
    }
    if Success == 0 && hasStaticCounters == 0 {
        minicov_dealloc(
            CurVNode as *mut u8,
            mem::size_of::<ValueProfNode>(),
            mem::align_of::<ValueProfNode>(),
        );
    }
}

#[no_mangle]
pub unsafe extern "C" fn __llvm_profile_instrument_target(
    TargetValue: u64,
    Data: *mut c_void,
    CounterIndex: u32,
) {
    instrumentTargetValueImpl(TargetValue, Data, CounterIndex, 1);
}

pub unsafe fn __llvm_profile_instrument_target_value(
    TargetValue: u64,
    Data: *mut c_void,
    CounterIndex: u32,
    CountValue: u64,
) {
    instrumentTargetValueImpl(TargetValue, Data, CounterIndex, CountValue);
}

#[no_mangle]
pub unsafe extern "C" fn __llvm_profile_instrument_memop(
    TargetValue: u64,
    Data: *mut c_void,
    CounterIndex: u32,
) {
    let RepValue = InstrProfGetRangeRepValue(TargetValue);
    __llvm_profile_instrument_target(RepValue, Data, CounterIndex);
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ValueProfRuntimeRecord {
    pub Data: *const __llvm_profile_data,
    pub NodesKind: [*mut *mut ValueProfNode; 3],
    pub SiteCountArray: *mut *mut u8,
}

unsafe extern "C" fn getNumValueSitesRT(R: *const c_void, VK: u32) -> u32 {
    (*(*(R as *const ValueProfRuntimeRecord)).Data)
        .0
        .NumValueSites[VK as usize] as u32
}

unsafe extern "C" fn getNumValueDataRT(R: *const c_void, VK: u32) -> u32 {
    let mut S: u32 = 0;
    let mut I: u32;
    let Record = R as *const ValueProfRuntimeRecord;
    if (*(*Record).SiteCountArray.offset(VK as isize)).is_null() {
        return 0;
    }
    I = 0;
    while I < (*(*Record).Data).0.NumValueSites[VK as usize] as u32 {
        S = S.wrapping_add(
            *(*(*Record).SiteCountArray.offset(VK as isize)).offset(I as isize) as u32,
        );
        I = I.wrapping_add(1);
    }
    S
}

unsafe extern "C" fn getNumValueDataForSiteRT(R: *const c_void, VK: u32, S: u32) -> u32 {
    let Record = R as *const ValueProfRuntimeRecord;
    *(*(*Record).SiteCountArray.offset(VK as isize)).offset(S as isize) as u32
}

static mut RTRecord: ValueProfRuntimeRecord = ValueProfRuntimeRecord {
    Data: ptr::null(),
    NodesKind: [ptr::null_mut(); 3],
    SiteCountArray: ptr::null_mut(),
};
static mut RTRecordClosure: ValueProfRecordClosure = {
    ValueProfRecordClosure {
        Record: &raw const RTRecord as *const c_void,
        GetNumValueKinds: None,
        GetNumValueSites: Some(getNumValueSitesRT),
        GetNumValueData: Some(getNumValueDataRT),
        GetNumValueDataForSite: Some(getNumValueDataForSiteRT),
        RemapValueData: None,
        GetValueForSite: None,
        AllocValueProfData: None,
    }
};

unsafe extern "C" fn initializeValueProfRuntimeRecord(
    Data: *const __llvm_profile_data,
    SiteCountArray: *mut *mut u8,
) -> u32 {
    let mut I: c_uint = 0;
    let mut J: c_uint;
    let mut S: c_uint = 0;
    let mut NumValueKinds: c_uint = 0;
    let Nodes = (*Data).0.Values as *mut *mut ValueProfNode;
    RTRecord.Data = Data;
    RTRecord.SiteCountArray = SiteCountArray;
    while I <= IPVK_Last as c_int as c_uint {
        let N: u16 = (*Data).0.NumValueSites[I as usize];
        if !(N == 0) {
            NumValueKinds = NumValueKinds.wrapping_add(1);
            RTRecord.NodesKind[I as usize] = if !Nodes.is_null() {
                Nodes.offset(S as isize)
            } else {
                ptr::null_mut()
            };
            J = 0;
            while J < N as c_uint {
                let mut C: u32 = 0;
                let mut Site = if !Nodes.is_null() {
                    *RTRecord.NodesKind[I as usize].offset(J as isize)
                } else {
                    ptr::null_mut()
                };
                while !Site.is_null() {
                    C = C.wrapping_add(1);
                    Site = (*Site).Next;
                }
                if C > c_uchar::MAX as u32 {
                    C = c_uchar::MAX as u32;
                }
                *(*RTRecord.SiteCountArray.offset(I as isize)).offset(J as isize) = C as u8;
                J = J.wrapping_add(1);
            }
            S = S.wrapping_add(N as c_uint);
        }
        I = I.wrapping_add(1);
    }
    NumValueKinds as u32
}

unsafe extern "C" fn getNextNValueData(
    VK: u32,
    Site: u32,
    Dst: *mut InstrProfValueData,
    StartNode: *mut ValueProfNode,
    N: u32,
) -> *mut ValueProfNode {
    let mut I: c_uint = 0;
    let mut VNode = if !StartNode.is_null() {
        StartNode
    } else {
        *RTRecord.NodesKind[VK as usize].offset(Site as isize)
    };
    while (I as u32) < N {
        (*Dst.offset(I as isize)).Value = (*VNode).Value;
        (*Dst.offset(I as isize)).Count = (*VNode).Count;
        VNode = (*VNode).Next;
        I = I.wrapping_add(1);
    }
    VNode
}

unsafe extern "C" fn getValueProfDataSizeWrapper() -> u32 {
    getValueProfDataSize(&raw mut RTRecordClosure)
}

unsafe extern "C" fn getNumValueDataForSiteWrapper(VK: u32, S: u32) -> u32 {
    getNumValueDataForSiteRT(&raw mut RTRecord as *const c_void, VK, S)
}

static mut TheVPDataReader: VPDataReaderType = {
    VPDataReaderType {
        InitRTRecord: Some(initializeValueProfRuntimeRecord),
        GetValueProfRecordHeaderSize: Some(getValueProfRecordHeaderSize),
        GetFirstValueProfRecord: Some(getFirstValueProfRecord),
        GetNumValueDataForSite: Some(getNumValueDataForSiteWrapper),
        GetValueProfDataSize: Some(getValueProfDataSizeWrapper),
        GetValueData: Some(getNextNValueData),
    }
};

pub unsafe fn lprofGetVPDataReader() -> *mut VPDataReaderType {
    &raw mut TheVPDataReader
}
