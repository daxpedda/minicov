use core::sync::atomic::{AtomicPtr, Ordering};

use super::InstrProfData::{
    __llvm_profile_data, getFirstValueProfRecord, getValueProfDataSize,
    getValueProfRecordHeaderSize, IPVK_First, IPVK_Last, InstrProfGetRangeRepValue,
    InstrProfValueData, ValueProfData, ValueProfNode, ValueProfRecord, ValueProfRecordClosure,
    INSTR_PROF_MAX_NUM_VAL_PER_SITE,
};
use super::InstrProfilingInternal::VPDataReaderType;
use super::InstrProfilingPlatformLinux::{CurrentVNode, EndVNode};
use super::{minicov_alloc_zeroed, minicov_dealloc};

pub type size_t = usize;
pub type uint64_t = u64;
pub type uint32_t = u32;
pub type uint16_t = u16;
pub type uint8_t = u8;
pub type intptr_t = isize;
pub type IntPtrT = *mut ::core::ffi::c_void;

pub const UCHAR_MAX: ::core::ffi::c_int =
    __SCHAR_MAX__ * 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int;
pub const __SCHAR_MAX__: ::core::ffi::c_int = 127 as ::core::ffi::c_int;

static mut hasStaticCounters: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
static mut OutOfNodesWarnings: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut hasNonDefaultValsPerSite: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const INSTR_PROF_DEFAULT_NUM_VAL_PER_SITE: ::core::ffi::c_int = 24 as ::core::ffi::c_int;

#[no_mangle]
#[link_section = "__llvm_prf_vnds"]
pub static mut lprofValueProfNodes: [ValueProfNode; 1024] = [ValueProfNode {
    Value: 0,
    Count: 0,
    Next: ::core::ptr::null::<ValueProfNode>() as *mut ValueProfNode,
}; 1024];

#[no_mangle]
pub static mut VPMaxNumValsPerSite: uint32_t = INSTR_PROF_DEFAULT_NUM_VAL_PER_SITE as uint32_t;

#[no_mangle]
pub unsafe extern "C" fn lprofSetupValueProfiler() {}

#[no_mangle]
pub unsafe extern "C" fn lprofSetMaxValsPerSite(MaxVals: uint32_t) {
    VPMaxNumValsPerSite = MaxVals;
    hasNonDefaultValsPerSite = 1 as ::core::ffi::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn __llvm_profile_set_num_value_sites(
    Data: *mut __llvm_profile_data,
    ValueKind: uint32_t,
    NumValueSites: uint16_t,
) {
    *((&raw const (*Data).0.NumValueSites as *const uint16_t).offset(ValueKind as isize)
        as *const uint16_t as *mut uint16_t) = NumValueSites;
}

#[no_mangle]
pub unsafe extern "C" fn __llvm_profile_iterate_data(
    Data: *const __llvm_profile_data,
) -> *const __llvm_profile_data {
    Data.offset(1 as ::core::ffi::c_int as isize)
}

#[no_mangle]
pub unsafe extern "C" fn __llvm_get_function_addr(
    Data: *const __llvm_profile_data,
) -> *mut ::core::ffi::c_void {
    (*Data).0.FunctionPointer
}

unsafe fn allocateValueProfileCounters(Data: *mut __llvm_profile_data) -> ::core::ffi::c_int {
    let mut NumVSites: uint64_t = 0 as uint64_t;
    let mut VKI: uint32_t;
    hasStaticCounters = 0 as ::core::ffi::c_int;
    if hasNonDefaultValsPerSite == 0 {
        VPMaxNumValsPerSite = INSTR_PROF_MAX_NUM_VAL_PER_SITE as uint32_t;
    }
    VKI = IPVK_First as ::core::ffi::c_int as uint32_t;
    while VKI <= IPVK_Last as ::core::ffi::c_int as uint32_t {
        NumVSites = NumVSites.wrapping_add((*Data).0.NumValueSites[VKI as usize] as uint64_t);
        VKI = VKI.wrapping_add(1);
    }
    let Size: size_t =
        NumVSites.wrapping_mul(::core::mem::size_of::<*mut ValueProfNode>() as uint64_t) as size_t;
    let Alignment: size_t = ::core::mem::align_of::<*mut ValueProfNode>();
    let Mem = minicov_alloc_zeroed(Size, Alignment) as *mut *mut ValueProfNode;
    if Mem.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if AtomicPtr::from_ptr(&raw mut (*Data).0.Values)
        .compare_exchange(
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            Mem as IntPtrT,
            Ordering::SeqCst,
            Ordering::SeqCst,
        )
        .is_err()
    {
        minicov_dealloc(Mem as *mut u8, Size, Alignment);
        return 0 as ::core::ffi::c_int;
    }
    1 as ::core::ffi::c_int
}

unsafe fn allocateOneNode() -> *mut ValueProfNode {
    if hasStaticCounters == 0 {
        return minicov_alloc_zeroed(
            ::core::mem::size_of::<ValueProfNode>() as size_t,
            ::core::mem::align_of::<ValueProfNode>(),
        ) as *mut ValueProfNode;
    }
    if CurrentVNode.offset(1 as ::core::ffi::c_int as isize) > EndVNode {
        OutOfNodesWarnings += 1;
        return ::core::ptr::null_mut::<ValueProfNode>();
    }
    let Node = AtomicPtr::from_ptr(&raw mut CurrentVNode as *mut *mut intptr_t).fetch_ptr_add(
        (::core::mem::size_of::<ValueProfNode>() as usize).wrapping_mul(1) as usize,
        Ordering::SeqCst,
    ) as *mut ValueProfNode;
    if Node.offset(1 as ::core::ffi::c_int as isize) > EndVNode {
        return ::core::ptr::null_mut::<ValueProfNode>();
    }
    Node
}

#[inline(always)]
unsafe fn instrumentTargetValueImpl(
    TargetValue: uint64_t,
    Data: *mut ::core::ffi::c_void,
    CounterIndex: uint32_t,
    CountValue: uint64_t,
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
    let mut PrevVNode = ::core::ptr::null_mut::<ValueProfNode>();
    let mut MinCountVNode = ::core::ptr::null_mut::<ValueProfNode>();
    let mut CurVNode = *ValueCounters.offset(CounterIndex as isize);
    let mut MinCount: uint64_t = 18446744073709551615 as uint64_t;
    let mut VDataCount: uint8_t = 0 as uint8_t;
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
    if VDataCount as uint32_t >= VPMaxNumValsPerSite {
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
    let mut Success: uint32_t = 0 as uint32_t;
    if (*ValueCounters.offset(CounterIndex as isize)).is_null() {
        Success = AtomicPtr::from_ptr(ValueCounters.offset(CounterIndex as isize))
            .compare_exchange(
                ::core::ptr::null_mut::<ValueProfNode>(),
                CurVNode,
                Ordering::SeqCst,
                Ordering::SeqCst,
            )
            .is_ok() as uint32_t;
    } else if !PrevVNode.is_null() && (*PrevVNode).Next.is_null() {
        Success = AtomicPtr::from_ptr(&raw mut (*PrevVNode).Next)
            .compare_exchange(
                ::core::ptr::null_mut::<ValueProfNode>(),
                CurVNode,
                Ordering::SeqCst,
                Ordering::SeqCst,
            )
            .is_ok() as uint32_t;
    }
    if Success == 0 && hasStaticCounters == 0 {
        minicov_dealloc(
            CurVNode as *mut u8,
            ::core::mem::size_of::<ValueProfNode>() as size_t,
            ::core::mem::align_of::<ValueProfNode>(),
        );
    }
}

#[no_mangle]
pub unsafe extern "C" fn __llvm_profile_instrument_target(
    TargetValue: uint64_t,
    Data: *mut ::core::ffi::c_void,
    CounterIndex: uint32_t,
) {
    instrumentTargetValueImpl(TargetValue, Data, CounterIndex, 1 as uint64_t);
}

#[no_mangle]
pub unsafe extern "C" fn __llvm_profile_instrument_target_value(
    TargetValue: uint64_t,
    Data: *mut ::core::ffi::c_void,
    CounterIndex: uint32_t,
    CountValue: uint64_t,
) {
    instrumentTargetValueImpl(TargetValue, Data, CounterIndex, CountValue);
}

#[no_mangle]
pub unsafe extern "C" fn __llvm_profile_instrument_memop(
    TargetValue: uint64_t,
    Data: *mut ::core::ffi::c_void,
    CounterIndex: uint32_t,
) {
    let RepValue = InstrProfGetRangeRepValue(TargetValue);
    __llvm_profile_instrument_target(RepValue, Data, CounterIndex);
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ValueProfRuntimeRecord {
    pub Data: *const __llvm_profile_data,
    pub NodesKind: [*mut *mut ValueProfNode; 3],
    pub SiteCountArray: *mut *mut uint8_t,
}

unsafe extern "C" fn getNumValueSitesRT(R: *const ::core::ffi::c_void, VK: uint32_t) -> uint32_t {
    (*(*(R as *const ValueProfRuntimeRecord)).Data)
        .0
        .NumValueSites[VK as usize] as uint32_t
}

unsafe extern "C" fn getNumValueDataRT(R: *const ::core::ffi::c_void, VK: uint32_t) -> uint32_t {
    let mut S: uint32_t = 0 as uint32_t;
    let mut I: uint32_t;
    let Record = R as *const ValueProfRuntimeRecord;
    if (*(*Record).SiteCountArray.offset(VK as isize)).is_null() {
        return 0 as uint32_t;
    }
    I = 0 as uint32_t;
    while I < (*(*Record).Data).0.NumValueSites[VK as usize] as uint32_t {
        S = S.wrapping_add(
            *(*(*Record).SiteCountArray.offset(VK as isize)).offset(I as isize) as uint32_t,
        );
        I = I.wrapping_add(1);
    }
    S
}

unsafe extern "C" fn getNumValueDataForSiteRT(
    R: *const ::core::ffi::c_void,
    VK: uint32_t,
    S: uint32_t,
) -> uint32_t {
    let Record = R as *const ValueProfRuntimeRecord;
    *(*(*Record).SiteCountArray.offset(VK as isize)).offset(S as isize) as uint32_t
}

static mut RTRecord: ValueProfRuntimeRecord = ValueProfRuntimeRecord {
    Data: ::core::ptr::null::<__llvm_profile_data>(),
    NodesKind: [::core::ptr::null::<*mut ValueProfNode>() as *mut *mut ValueProfNode; 3],
    SiteCountArray: ::core::ptr::null::<*mut uint8_t>() as *mut *mut uint8_t,
};
static mut RTRecordClosure: ValueProfRecordClosure = {
    ValueProfRecordClosure {
        Record: &raw const RTRecord as *mut ValueProfRuntimeRecord as *const ::core::ffi::c_void,
        GetNumValueKinds: None,
        GetNumValueSites: Some(
            getNumValueSitesRT
                as unsafe extern "C" fn(*const ::core::ffi::c_void, uint32_t) -> uint32_t,
        ),
        GetNumValueData: Some(
            getNumValueDataRT
                as unsafe extern "C" fn(*const ::core::ffi::c_void, uint32_t) -> uint32_t,
        ),
        GetNumValueDataForSite: Some(
            getNumValueDataForSiteRT
                as unsafe extern "C" fn(*const ::core::ffi::c_void, uint32_t, uint32_t) -> uint32_t,
        ),
        RemapValueData: None,
        GetValueForSite: None,
        AllocValueProfData: None,
    }
};

unsafe extern "C" fn initializeValueProfRuntimeRecord(
    Data: *const __llvm_profile_data,
    SiteCountArray: *mut *mut uint8_t,
) -> uint32_t {
    let mut I: ::core::ffi::c_uint;
    let mut J: ::core::ffi::c_uint;
    let mut S = 0 as ::core::ffi::c_uint;
    let mut NumValueKinds = 0 as ::core::ffi::c_uint;
    let Nodes = (*Data).0.Values as *mut *mut ValueProfNode;
    RTRecord.Data = Data;
    RTRecord.SiteCountArray = SiteCountArray;
    I = 0 as ::core::ffi::c_uint;
    while I <= IPVK_Last as ::core::ffi::c_int as ::core::ffi::c_uint {
        let N: uint16_t = (*Data).0.NumValueSites[I as usize];
        if !(N == 0) {
            NumValueKinds = NumValueKinds.wrapping_add(1);
            RTRecord.NodesKind[I as usize] = if !Nodes.is_null() {
                Nodes.offset(S as isize)
            } else {
                ::core::ptr::null_mut::<*mut ValueProfNode>()
            };
            J = 0 as ::core::ffi::c_uint;
            while J < N as ::core::ffi::c_uint {
                let mut C: uint32_t = 0 as uint32_t;
                let mut Site = if !Nodes.is_null() {
                    *RTRecord.NodesKind[I as usize].offset(J as isize)
                } else {
                    ::core::ptr::null_mut::<ValueProfNode>()
                };
                while !Site.is_null() {
                    C = C.wrapping_add(1);
                    Site = (*Site).Next;
                }
                if C > UCHAR_MAX as uint32_t {
                    C = UCHAR_MAX as uint32_t;
                }
                *(*RTRecord.SiteCountArray.offset(I as isize)).offset(J as isize) = C as uint8_t;
                J = J.wrapping_add(1);
            }
            S = S.wrapping_add(N as ::core::ffi::c_uint);
        }
        I = I.wrapping_add(1);
    }
    NumValueKinds as uint32_t
}

unsafe extern "C" fn getNextNValueData(
    VK: uint32_t,
    Site: uint32_t,
    Dst: *mut InstrProfValueData,
    StartNode: *mut ValueProfNode,
    N: uint32_t,
) -> *mut ValueProfNode {
    let mut I: ::core::ffi::c_uint;
    let mut VNode = if !StartNode.is_null() {
        StartNode
    } else {
        *RTRecord.NodesKind[VK as usize].offset(Site as isize)
    };
    I = 0 as ::core::ffi::c_uint;
    while (I as uint32_t) < N {
        (*Dst.offset(I as isize)).Value = (*VNode).Value;
        (*Dst.offset(I as isize)).Count = (*VNode).Count;
        VNode = (*VNode).Next;
        I = I.wrapping_add(1);
    }
    VNode
}

unsafe extern "C" fn getValueProfDataSizeWrapper() -> uint32_t {
    getValueProfDataSize(&raw mut RTRecordClosure)
}

unsafe extern "C" fn getNumValueDataForSiteWrapper(VK: uint32_t, S: uint32_t) -> uint32_t {
    getNumValueDataForSiteRT(&raw mut RTRecord as *const ::core::ffi::c_void, VK, S)
}

static mut TheVPDataReader: VPDataReaderType = {
    VPDataReaderType {
        InitRTRecord: Some(
            initializeValueProfRuntimeRecord
                as unsafe extern "C" fn(*const __llvm_profile_data, *mut *mut uint8_t) -> uint32_t,
        ),
        GetValueProfRecordHeaderSize: Some(
            getValueProfRecordHeaderSize as unsafe extern "C" fn(uint32_t) -> uint32_t,
        ),
        GetFirstValueProfRecord: Some(
            getFirstValueProfRecord
                as unsafe extern "C" fn(*mut ValueProfData) -> *mut ValueProfRecord,
        ),
        GetNumValueDataForSite: Some(
            getNumValueDataForSiteWrapper as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t,
        ),
        GetValueProfDataSize: Some(
            getValueProfDataSizeWrapper as unsafe extern "C" fn() -> uint32_t,
        ),
        GetValueData: Some(
            getNextNValueData
                as unsafe extern "C" fn(
                    uint32_t,
                    uint32_t,
                    *mut InstrProfValueData,
                    *mut ValueProfNode,
                    uint32_t,
                ) -> *mut ValueProfNode,
        ),
    }
};

#[no_mangle]
pub unsafe extern "C" fn lprofGetVPDataReader() -> *mut VPDataReaderType {
    &raw mut TheVPDataReader
}
