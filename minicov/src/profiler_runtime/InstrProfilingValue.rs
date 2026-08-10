extern "C" {
    fn minicov_alloc_zeroed(Size: size_t, Alignment: size_t) -> *mut ::core::ffi::c_void;
    fn minicov_dealloc(Ptr: *mut ::core::ffi::c_void, Size: size_t, Alignment: size_t);
    static mut CurrentVNode: *mut ValueProfNode;
    static mut EndVNode: *mut ValueProfNode;
}
pub type size_t = usize;
pub type uint64_t = u64;
pub type uint32_t = u32;
pub type uint16_t = u16;
pub type uint8_t = u8;
pub type intptr_t = isize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct InstrProfValueData {
    pub Value: uint64_t,
    pub Count: uint64_t,
}
pub type ValueKind = ::core::ffi::c_uint;
pub const IPVK_Last: ValueKind = 2;
pub const IPVK_First: ValueKind = 0;
pub const IPVK_VTableTarget: ValueKind = 2;
pub const IPVK_MemOPSize: ValueKind = 1;
pub const IPVK_IndirectCallTarget: ValueKind = 0;
pub type IntPtrT = *mut ::core::ffi::c_void;
#[derive(Copy, Clone)]
#[repr(C, align(8))]
pub struct __llvm_profile_data(pub __llvm_profile_data_Inner);
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __llvm_profile_data_Inner {
    pub NameRef: uint64_t,
    pub FuncHash: uint64_t,
    pub CounterPtr: IntPtrT,
    pub UniformCounterPtr: IntPtrT,
    pub BitmapPtr: IntPtrT,
    pub FunctionPointer: IntPtrT,
    pub Values: IntPtrT,
    pub NumCounters: uint32_t,
    pub NumValueSites: [uint16_t; 3],
    pub OffloadDeviceWaveSize: uint16_t,
    pub NumBitmapBytes: uint32_t,
}
#[allow(dead_code, non_upper_case_globals)]
const __llvm_profile_data_PADDING: usize = ::core::mem::size_of::<__llvm_profile_data>()
    - ::core::mem::size_of::<__llvm_profile_data_Inner>();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ValueProfNode {
    pub Value: uint64_t,
    pub Count: uint64_t,
    pub Next: PtrToNodeT,
}
pub type PtrToNodeT = *mut ValueProfNode;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ValueProfData {
    pub TotalSize: uint32_t,
    pub NumValueKinds: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ValueProfRecord {
    pub Kind: uint32_t,
    pub NumValueSites: uint32_t,
    pub SiteCountArray: [uint8_t; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VPDataReaderType {
    pub InitRTRecord:
        Option<unsafe extern "C" fn(*const __llvm_profile_data, *mut *mut uint8_t) -> uint32_t>,
    pub GetValueProfRecordHeaderSize: Option<unsafe extern "C" fn(uint32_t) -> uint32_t>,
    pub GetFirstValueProfRecord:
        Option<unsafe extern "C" fn(*mut ValueProfData) -> *mut ValueProfRecord>,
    pub GetNumValueDataForSite: Option<unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t>,
    pub GetValueProfDataSize: Option<unsafe extern "C" fn() -> uint32_t>,
    pub GetValueData: Option<
        unsafe extern "C" fn(
            uint32_t,
            uint32_t,
            *mut InstrProfValueData,
            *mut ValueProfNode,
            uint32_t,
        ) -> *mut ValueProfNode,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ValueProfRuntimeRecord {
    pub Data: *const __llvm_profile_data,
    pub NodesKind: [*mut *mut ValueProfNode; 3],
    pub SiteCountArray: *mut *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ValueProfRecordClosure {
    pub Record: *const ::core::ffi::c_void,
    pub GetNumValueKinds: Option<unsafe extern "C" fn(*const ::core::ffi::c_void) -> uint32_t>,
    pub GetNumValueSites:
        Option<unsafe extern "C" fn(*const ::core::ffi::c_void, uint32_t) -> uint32_t>,
    pub GetNumValueData:
        Option<unsafe extern "C" fn(*const ::core::ffi::c_void, uint32_t) -> uint32_t>,
    pub GetNumValueDataForSite:
        Option<unsafe extern "C" fn(*const ::core::ffi::c_void, uint32_t, uint32_t) -> uint32_t>,
    pub RemapValueData: Option<unsafe extern "C" fn(uint32_t, uint64_t) -> uint64_t>,
    pub GetValueForSite: Option<
        unsafe extern "C" fn(
            *const ::core::ffi::c_void,
            *mut InstrProfValueData,
            uint32_t,
            uint32_t,
        ) -> (),
    >,
    pub AllocValueProfData: Option<unsafe extern "C" fn(size_t) -> *mut ValueProfData>,
}
pub const INSTR_PROF_MAX_NUM_VAL_PER_SITE: ::core::ffi::c_int = 255 as ::core::ffi::c_int;
pub const INSTR_PROF_NULLPTR: *mut ::core::ffi::c_void = NULL;
#[no_mangle]
pub unsafe extern "C" fn getValueProfRecordHeaderSize(mut NumValueSites: uint32_t) -> uint32_t {
    let mut Size: uint32_t = (8 as usize).wrapping_add(
        (::core::mem::size_of::<uint8_t>() as usize).wrapping_mul(NumValueSites as usize),
    ) as uint32_t;
    Size = Size.wrapping_add(7 as uint32_t) & !(7 as ::core::ffi::c_int) as uint32_t;
    return Size;
}
#[no_mangle]
pub unsafe extern "C" fn getValueProfRecordSize(
    mut NumValueSites: uint32_t,
    mut NumValueData: uint32_t,
) -> uint32_t {
    return (getValueProfRecordHeaderSize(NumValueSites) as usize).wrapping_add(
        (::core::mem::size_of::<InstrProfValueData>() as usize).wrapping_mul(NumValueData as usize),
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C" fn getValueProfRecordValueData(
    mut This: *mut ValueProfRecord,
) -> *mut InstrProfValueData {
    return (This as *mut ::core::ffi::c_char)
        .offset(getValueProfRecordHeaderSize((*This).NumValueSites) as isize)
        as *mut InstrProfValueData;
}
#[no_mangle]
pub unsafe extern "C" fn getValueProfRecordNumValueData(
    mut This: *mut ValueProfRecord,
) -> uint32_t {
    let mut NumValueData: uint32_t = 0 as uint32_t;
    let mut I: uint32_t = 0;
    I = 0 as uint32_t;
    while I < (*This).NumValueSites {
        NumValueData = NumValueData.wrapping_add(
            *(&raw mut (*This).SiteCountArray as *mut uint8_t).offset(I as isize) as uint32_t,
        );
        I = I.wrapping_add(1);
    }
    return NumValueData;
}
#[no_mangle]
pub unsafe extern "C" fn getValueProfRecordNext(
    mut This: *mut ValueProfRecord,
) -> *mut ValueProfRecord {
    let mut NumValueData = getValueProfRecordNumValueData(This);
    return (This as *mut ::core::ffi::c_char)
        .offset(getValueProfRecordSize((*This).NumValueSites, NumValueData) as isize)
        as *mut ValueProfRecord;
}
#[no_mangle]
pub unsafe extern "C" fn getFirstValueProfRecord(
    mut This: *mut ValueProfData,
) -> *mut ValueProfRecord {
    return (This as *mut ::core::ffi::c_char)
        .offset(::core::mem::size_of::<ValueProfData>() as usize as isize)
        as *mut ValueProfRecord;
}
#[no_mangle]
pub unsafe extern "C" fn getValueProfDataSize(
    mut Closure: *mut ValueProfRecordClosure,
) -> uint32_t {
    let mut Kind: uint32_t = 0;
    let mut TotalSize: uint32_t = ::core::mem::size_of::<ValueProfData>() as uint32_t;
    let mut Record = (*Closure).Record;
    Kind = IPVK_First as ::core::ffi::c_int as uint32_t;
    while Kind <= IPVK_Last as ::core::ffi::c_int as uint32_t {
        let mut NumValueSites = (*Closure)
            .GetNumValueSites
            .expect("non-null function pointer")(Record, Kind);
        if !(NumValueSites == 0) {
            TotalSize = TotalSize.wrapping_add(getValueProfRecordSize(
                NumValueSites,
                (*Closure)
                    .GetNumValueData
                    .expect("non-null function pointer")(Record, Kind),
            ));
        }
        Kind = Kind.wrapping_add(1);
    }
    return TotalSize;
}
#[no_mangle]
pub unsafe extern "C" fn serializeValueProfRecordFrom(
    mut This: *mut ValueProfRecord,
    mut Closure: *mut ValueProfRecordClosure,
    mut ValueKind: uint32_t,
    mut NumValueSites: uint32_t,
) {
    let mut S: uint32_t = 0;
    let mut Record = (*Closure).Record;
    (*This).Kind = ValueKind;
    (*This).NumValueSites = NumValueSites;
    let mut DstVD = getValueProfRecordValueData(This);
    S = 0 as uint32_t;
    while S < NumValueSites {
        let mut ND = (*Closure)
            .GetNumValueDataForSite
            .expect("non-null function pointer")(Record, ValueKind, S);
        *(&raw mut (*This).SiteCountArray as *mut uint8_t).offset(S as isize) = ND as uint8_t;
        (*Closure)
            .GetValueForSite
            .expect("non-null function pointer")(Record, DstVD, ValueKind, S);
        DstVD = DstVD.offset(ND as isize);
        S = S.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn serializeValueProfDataFrom(
    mut Closure: *mut ValueProfRecordClosure,
    mut DstData: *mut ValueProfData,
) -> *mut ValueProfData {
    let mut Kind: uint32_t = 0;
    let mut TotalSize: uint32_t = if !DstData.is_null() {
        (*DstData).TotalSize
    } else {
        getValueProfDataSize(Closure)
    };
    let mut VPD = if !DstData.is_null() {
        DstData
    } else {
        (*Closure)
            .AllocValueProfData
            .expect("non-null function pointer")(TotalSize as size_t)
    };
    (*VPD).TotalSize = TotalSize;
    (*VPD).NumValueKinds = (*Closure)
        .GetNumValueKinds
        .expect("non-null function pointer")((*Closure).Record);
    let mut VR = getFirstValueProfRecord(VPD);
    Kind = IPVK_First as ::core::ffi::c_int as uint32_t;
    while Kind <= IPVK_Last as ::core::ffi::c_int as uint32_t {
        let mut NumValueSites =
            (*Closure)
                .GetNumValueSites
                .expect("non-null function pointer")((*Closure).Record, Kind);
        if !(NumValueSites == 0) {
            serializeValueProfRecordFrom(VR, Closure, Kind, NumValueSites);
            VR = getValueProfRecordNext(VR);
        }
        Kind = Kind.wrapping_add(1);
    }
    return VPD;
}
#[no_mangle]
pub unsafe extern "C" fn InstProfClzll(mut X: ::core::ffi::c_ulonglong) -> ::core::ffi::c_int {
    return X.leading_zeros() as i32;
}
#[no_mangle]
pub unsafe extern "C" fn InstProfPopcountll(mut X: ::core::ffi::c_ulonglong) -> ::core::ffi::c_int {
    return X.count_ones() as i32;
}
#[no_mangle]
pub unsafe extern "C" fn InstrProfGetRangeRepValue(mut Value: uint64_t) -> uint64_t {
    if Value <= 8 as uint64_t {
        return Value;
    } else if Value >= 513 as uint64_t {
        return 513 as uint64_t;
    } else if InstProfPopcountll(Value as ::core::ffi::c_ulonglong) == 1 as ::core::ffi::c_int {
        return Value;
    } else {
        return ((1 as uint64_t)
            << 64 as ::core::ffi::c_int
                - InstProfClzll(Value as ::core::ffi::c_ulonglong)
                - 1 as ::core::ffi::c_int)
            .wrapping_add(1 as uint64_t);
    };
}
#[no_mangle]
pub unsafe extern "C" fn InstrProfIsSingleValRange(mut Value: uint64_t) -> ::core::ffi::c_uint {
    if Value <= 8 as uint64_t {
        return 1 as ::core::ffi::c_uint;
    } else if InstProfPopcountll(Value as ::core::ffi::c_ulonglong) == 1 as ::core::ffi::c_int {
        return 1 as ::core::ffi::c_uint;
    } else {
        return 0 as ::core::ffi::c_uint;
    };
}
static mut hasStaticCounters: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
static mut OutOfNodesWarnings: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut hasNonDefaultValsPerSite: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const INSTR_PROF_MAX_VP_WARNS: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
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
pub unsafe extern "C" fn lprofSetMaxValsPerSite(mut MaxVals: uint32_t) {
    VPMaxNumValsPerSite = MaxVals;
    hasNonDefaultValsPerSite = 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn __llvm_profile_set_num_value_sites(
    mut Data: *mut __llvm_profile_data,
    mut ValueKind: uint32_t,
    mut NumValueSites: uint16_t,
) {
    *((&raw const (*Data).0.NumValueSites as *const uint16_t).offset(ValueKind as isize)
        as *const uint16_t as *mut uint16_t) = NumValueSites;
}
#[no_mangle]
pub unsafe extern "C" fn __llvm_profile_iterate_data(
    mut Data: *const __llvm_profile_data,
) -> *const __llvm_profile_data {
    return Data.offset(1 as ::core::ffi::c_int as isize);
}
#[no_mangle]
pub unsafe extern "C" fn __llvm_get_function_addr(
    mut Data: *const __llvm_profile_data,
) -> *mut ::core::ffi::c_void {
    return (*Data).0.FunctionPointer as *mut ::core::ffi::c_void;
}
unsafe extern "C" fn allocateValueProfileCounters(
    mut Data: *mut __llvm_profile_data,
) -> ::core::ffi::c_int {
    let mut NumVSites: uint64_t = 0 as uint64_t;
    let mut VKI: uint32_t = 0;
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
    let mut Mem = minicov_alloc_zeroed(Size, Alignment) as *mut *mut ValueProfNode;
    if Mem.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if !::core::intrinsics::atomic_cxchg_seqcst_seqcst(
        &raw mut (*Data).0.Values,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
        Mem as IntPtrT,
    )
    .1
    {
        minicov_dealloc(Mem as *mut ::core::ffi::c_void, Size, Alignment);
        return 0 as ::core::ffi::c_int;
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn allocateOneNode() -> *mut ValueProfNode {
    let mut Node = ::core::ptr::null_mut::<ValueProfNode>();
    if hasStaticCounters == 0 {
        return minicov_alloc_zeroed(
            ::core::mem::size_of::<ValueProfNode>() as size_t,
            ::core::mem::align_of::<ValueProfNode>(),
        ) as *mut ValueProfNode;
    }
    if CurrentVNode.offset(1 as ::core::ffi::c_int as isize) > EndVNode {
        let fresh0 = OutOfNodesWarnings;
        OutOfNodesWarnings = OutOfNodesWarnings + 1;
        fresh0 < INSTR_PROF_MAX_VP_WARNS;
        return ::core::ptr::null_mut::<ValueProfNode>();
    }
    Node = ::core::intrinsics::atomic_xadd_seqcst(
        &raw mut CurrentVNode as *mut intptr_t,
        (::core::mem::size_of::<ValueProfNode>() as usize).wrapping_mul(1 as usize) as intptr_t,
    ) as *mut ValueProfNode;
    if Node.offset(1 as ::core::ffi::c_int as isize) > EndVNode {
        return ::core::ptr::null_mut::<ValueProfNode>();
    }
    return Node;
}
#[inline(always)]
unsafe extern "C" fn instrumentTargetValueImpl(
    mut TargetValue: uint64_t,
    mut Data: *mut ::core::ffi::c_void,
    mut CounterIndex: uint32_t,
    mut CountValue: uint64_t,
) {
    let mut PData = Data as *mut __llvm_profile_data;
    if PData.is_null() {
        return;
    }
    if CountValue == 0 {
        return;
    }
    if (*PData).0.Values.is_null() {
        if allocateValueProfileCounters(PData) == 0 {
            return;
        }
    }
    let mut ValueCounters = (*PData).0.Values as *mut *mut ValueProfNode;
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
        CurVNode = (*CurVNode).Next as *mut ValueProfNode;
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
        Success = ::core::intrinsics::atomic_cxchg_seqcst_seqcst(
            ValueCounters.offset(CounterIndex as isize) as *mut *mut ValueProfNode,
            ::core::ptr::null_mut::<ValueProfNode>(),
            CurVNode,
        )
        .1 as uint32_t;
    } else if !PrevVNode.is_null() && (*PrevVNode).Next.is_null() {
        Success = ::core::intrinsics::atomic_cxchg_seqcst_seqcst(
            &raw mut (*PrevVNode).Next,
            ::core::ptr::null_mut::<ValueProfNode>(),
            CurVNode,
        )
        .1 as uint32_t;
    }
    if Success == 0 && hasStaticCounters == 0 {
        minicov_dealloc(
            CurVNode as *mut ::core::ffi::c_void,
            ::core::mem::size_of::<ValueProfNode>() as size_t,
            ::core::mem::align_of::<ValueProfNode>(),
        );
        return;
    }
}
#[no_mangle]
pub unsafe extern "C" fn __llvm_profile_instrument_target(
    mut TargetValue: uint64_t,
    mut Data: *mut ::core::ffi::c_void,
    mut CounterIndex: uint32_t,
) {
    instrumentTargetValueImpl(TargetValue, Data, CounterIndex, 1 as uint64_t);
}
#[no_mangle]
pub unsafe extern "C" fn __llvm_profile_instrument_target_value(
    mut TargetValue: uint64_t,
    mut Data: *mut ::core::ffi::c_void,
    mut CounterIndex: uint32_t,
    mut CountValue: uint64_t,
) {
    instrumentTargetValueImpl(TargetValue, Data, CounterIndex, CountValue);
}
#[no_mangle]
pub unsafe extern "C" fn __llvm_profile_instrument_memop(
    mut TargetValue: uint64_t,
    mut Data: *mut ::core::ffi::c_void,
    mut CounterIndex: uint32_t,
) {
    let mut RepValue = InstrProfGetRangeRepValue(TargetValue);
    __llvm_profile_instrument_target(RepValue, Data, CounterIndex);
}
unsafe extern "C" fn getNumValueSitesRT(
    mut R: *const ::core::ffi::c_void,
    mut VK: uint32_t,
) -> uint32_t {
    return (*(*(R as *const ValueProfRuntimeRecord)).Data)
        .0
        .NumValueSites[VK as usize] as uint32_t;
}
unsafe extern "C" fn getNumValueDataRT(
    mut R: *const ::core::ffi::c_void,
    mut VK: uint32_t,
) -> uint32_t {
    let mut S: uint32_t = 0 as uint32_t;
    let mut I: uint32_t = 0;
    let mut Record = R as *const ValueProfRuntimeRecord;
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
    return S;
}
unsafe extern "C" fn getNumValueDataForSiteRT(
    mut R: *const ::core::ffi::c_void,
    mut VK: uint32_t,
    mut S: uint32_t,
) -> uint32_t {
    let mut Record = R as *const ValueProfRuntimeRecord;
    return *(*(*Record).SiteCountArray.offset(VK as isize)).offset(S as isize) as uint32_t;
}
static mut RTRecord: ValueProfRuntimeRecord = ValueProfRuntimeRecord {
    Data: ::core::ptr::null::<__llvm_profile_data>(),
    NodesKind: [::core::ptr::null::<*mut ValueProfNode>() as *mut *mut ValueProfNode; 3],
    SiteCountArray: ::core::ptr::null::<*mut uint8_t>() as *mut *mut uint8_t,
};
static mut RTRecordClosure: ValueProfRecordClosure = unsafe {
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
    mut Data: *const __llvm_profile_data,
    mut SiteCountArray: *mut *mut uint8_t,
) -> uint32_t {
    let mut I: ::core::ffi::c_uint = 0;
    let mut J: ::core::ffi::c_uint = 0;
    let mut S = 0 as ::core::ffi::c_uint;
    let mut NumValueKinds = 0 as ::core::ffi::c_uint;
    let mut Nodes = (*Data).0.Values as *mut *mut ValueProfNode;
    RTRecord.Data = Data;
    RTRecord.SiteCountArray = SiteCountArray as *mut *mut uint8_t;
    I = 0 as ::core::ffi::c_uint;
    while I <= IPVK_Last as ::core::ffi::c_int as ::core::ffi::c_uint {
        let mut N: uint16_t = (*Data).0.NumValueSites[I as usize];
        if !(N == 0) {
            NumValueKinds = NumValueKinds.wrapping_add(1);
            RTRecord.NodesKind[I as usize] = if !Nodes.is_null() {
                Nodes.offset(S as isize) as *mut *mut ValueProfNode
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
                    Site = (*Site).Next as *mut ValueProfNode;
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
    return NumValueKinds as uint32_t;
}
unsafe extern "C" fn getNextNValueData(
    mut VK: uint32_t,
    mut Site: uint32_t,
    mut Dst: *mut InstrProfValueData,
    mut StartNode: *mut ValueProfNode,
    mut N: uint32_t,
) -> *mut ValueProfNode {
    let mut I: ::core::ffi::c_uint = 0;
    let mut VNode = if !StartNode.is_null() {
        StartNode
    } else {
        *RTRecord.NodesKind[VK as usize].offset(Site as isize)
    };
    I = 0 as ::core::ffi::c_uint;
    while (I as uint32_t) < N {
        (*Dst.offset(I as isize)).Value = (*VNode).Value;
        (*Dst.offset(I as isize)).Count = (*VNode).Count;
        VNode = (*VNode).Next as *mut ValueProfNode;
        I = I.wrapping_add(1);
    }
    return VNode;
}
unsafe extern "C" fn getValueProfDataSizeWrapper() -> uint32_t {
    return getValueProfDataSize(&raw mut RTRecordClosure);
}
unsafe extern "C" fn getNumValueDataForSiteWrapper(mut VK: uint32_t, mut S: uint32_t) -> uint32_t {
    return getNumValueDataForSiteRT(&raw mut RTRecord as *const ::core::ffi::c_void, VK, S);
}
static mut TheVPDataReader: VPDataReaderType = unsafe {
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
    return &raw mut TheVPDataReader;
}
pub const UCHAR_MAX: ::core::ffi::c_int =
    __SCHAR_MAX__ * 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const __SCHAR_MAX__: ::core::ffi::c_int = 127 as ::core::ffi::c_int;
