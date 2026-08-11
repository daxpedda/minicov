extern "C" {
    fn lprofMergeValueProfData(
        SrcValueProfData: *mut ValueProfData,
        DstData: *mut __llvm_profile_data,
    );
    fn __llvm_profile_get_num_padding_bytes(SizeInBytes: uint64_t) -> uint8_t;
    fn __llvm_profile_begin_data() -> *const __llvm_profile_data;
    fn __llvm_profile_end_data() -> *const __llvm_profile_data;
    fn __llvm_profile_begin_names() -> *const ::core::ffi::c_char;
    fn __llvm_profile_end_names() -> *const ::core::ffi::c_char;
    fn __llvm_profile_begin_counters() -> *mut ::core::ffi::c_char;
    fn __llvm_profile_end_counters() -> *mut ::core::ffi::c_char;
    fn __llvm_profile_begin_bitmap() -> *mut ::core::ffi::c_char;
    fn __llvm_profile_end_bitmap() -> *mut ::core::ffi::c_char;
    fn __llvm_profile_begin_vnodes() -> *mut ValueProfNode;
    fn __llvm_profile_end_vnodes() -> *mut ValueProfNode;
    fn __llvm_profile_get_magic() -> uint64_t;
    fn __llvm_profile_get_version() -> uint64_t;
    fn __llvm_profile_get_num_data(
        Begin: *const __llvm_profile_data,
        End: *const __llvm_profile_data,
    ) -> uint64_t;
    fn __llvm_profile_counter_entry_size() -> size_t;
    fn __llvm_profile_get_num_counters(
        Begin: *const ::core::ffi::c_char,
        End: *const ::core::ffi::c_char,
    ) -> uint64_t;
    fn __llvm_profile_get_num_bitmap_bytes(
        Begin: *const ::core::ffi::c_char,
        End: *const ::core::ffi::c_char,
    ) -> uint64_t;
    fn __llvm_profile_get_name_size(
        Begin: *const ::core::ffi::c_char,
        End: *const ::core::ffi::c_char,
    ) -> uint64_t;
}
pub type size_t = usize;
pub type uint64_t = u64;
pub type uint32_t = u32;
pub type uint16_t = u16;
pub type uint8_t = u8;
pub type uintptr_t = usize;
pub type ValueKind = ::core::ffi::c_uint;
pub const IPVK_Last: ValueKind = 2;
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
pub struct __llvm_profile_header {
    pub Magic: uint64_t,
    pub Version: uint64_t,
    pub BinaryIdsSize: uint64_t,
    pub NumData: uint64_t,
    pub PaddingBytesBeforeCounters: uint64_t,
    pub NumCounters: uint64_t,
    pub PaddingBytesAfterCounters: uint64_t,
    pub NumBitmapBytes: uint64_t,
    pub PaddingBytesAfterBitmapBytes: uint64_t,
    pub NumUniformCounters: uint64_t,
    pub PaddingBytesAfterUniformCounters: uint64_t,
    pub UniformCountersDelta: uint64_t,
    pub NamesSize: uint64_t,
    pub CountersDelta: uint64_t,
    pub BitmapDelta: uint64_t,
    pub NamesDelta: uint64_t,
    pub NumVTables: uint64_t,
    pub VNamesSize: uint64_t,
    pub ValueKindLast: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ValueProfNode {
    pub Value: uint64_t,
    pub Count: uint64_t,
    pub Next: PtrToNodeT,
}
pub type PtrToNodeT = *mut ValueProfNode;
#[derive(Copy, Clone)]
#[repr(C, align(8))]
pub struct VTableProfData(pub VTableProfData_Inner);
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VTableProfData_Inner {
    pub VTableNameHash: uint64_t,
    pub VTablePointer: IntPtrT,
    pub VTableSize: uint32_t,
}
#[allow(dead_code, non_upper_case_globals)]
const VTableProfData_PADDING: usize =
    ::core::mem::size_of::<VTableProfData>() - ::core::mem::size_of::<VTableProfData_Inner>();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ValueProfData {
    pub TotalSize: uint32_t,
    pub NumValueKinds: uint32_t,
}
pub const VARIANT_MASK_BYTE_COVERAGE: ::core::ffi::c_ulonglong =
    (0x1 as ::core::ffi::c_ulonglong) << 60 as ::core::ffi::c_int;
pub const VARIANT_MASK_TEMPORAL_PROF: ::core::ffi::c_ulonglong =
    (0x1 as ::core::ffi::c_ulonglong) << 63 as ::core::ffi::c_int;
#[no_mangle]
pub static VPMergeHook: Option<
    unsafe extern "C" fn(*mut ValueProfData, *mut __llvm_profile_data) -> (),
> = Some(
    lprofMergeValueProfData
        as unsafe extern "C" fn(*mut ValueProfData, *mut __llvm_profile_data) -> (),
);
#[no_mangle]
pub unsafe extern "C" fn lprofGetLoadModuleSignature() -> uint64_t {
    let Version = __llvm_profile_get_version();
    let NumCounters = __llvm_profile_get_num_counters(
        __llvm_profile_begin_counters(),
        __llvm_profile_end_counters(),
    );
    let NumData =
        __llvm_profile_get_num_data(__llvm_profile_begin_data(), __llvm_profile_end_data());
    let NamesSize: uint64_t = __llvm_profile_end_names().offset_from(__llvm_profile_begin_names())
        as ::core::ffi::c_long as uint64_t;
    let NumVnodes: uint64_t = __llvm_profile_end_vnodes().offset_from(__llvm_profile_begin_vnodes())
        as ::core::ffi::c_long as uint64_t;
    let FirstD = __llvm_profile_begin_data();
    (NamesSize << 40 as ::core::ffi::c_int)
        .wrapping_add(NumCounters << 30 as ::core::ffi::c_int)
        .wrapping_add(NumData << 20 as ::core::ffi::c_int)
        .wrapping_add(NumVnodes << 10 as ::core::ffi::c_int)
        .wrapping_add(if NumData > 0 as uint64_t {
            (*FirstD).0.NameRef
        } else {
            0 as uint64_t
        })
        .wrapping_add(Version)
        .wrapping_add(__llvm_profile_get_magic())
}
#[no_mangle]
pub unsafe extern "C" fn __llvm_profile_check_compatibility(
    ProfileData: *const ::core::ffi::c_char,
    ProfileSize: uint64_t,
) -> ::core::ffi::c_int {
    let Header = ProfileData as *mut __llvm_profile_header;
    let mut SrcData: *mut __llvm_profile_data;
    let mut DstData: *mut __llvm_profile_data;
    let SrcDataStart: *mut __llvm_profile_data = ProfileData
        .add(::core::mem::size_of::<__llvm_profile_header>() as usize)
        .offset((*Header).BinaryIdsSize as isize)
        as *mut __llvm_profile_data;
    let SrcDataEnd: *mut __llvm_profile_data = SrcDataStart.offset((*Header).NumData as isize);
    if ProfileSize < ::core::mem::size_of::<__llvm_profile_header>() as uint64_t {
        return 1 as ::core::ffi::c_int;
    }
    if (*Header).Magic != __llvm_profile_get_magic()
        || (*Header).Version != __llvm_profile_get_version()
        || (*Header).NumData
            != __llvm_profile_get_num_data(__llvm_profile_begin_data(), __llvm_profile_end_data())
        || (*Header).NumCounters
            != __llvm_profile_get_num_counters(
                __llvm_profile_begin_counters(),
                __llvm_profile_end_counters(),
            )
        || (*Header).NumBitmapBytes
            != __llvm_profile_get_num_bitmap_bytes(
                __llvm_profile_begin_bitmap(),
                __llvm_profile_end_bitmap(),
            )
        || (*Header).NamesSize
            != __llvm_profile_get_name_size(
                __llvm_profile_begin_names(),
                __llvm_profile_end_names(),
            )
        || (*Header).ValueKindLast != IPVK_Last as ::core::ffi::c_int as uint64_t
    {
        return 1 as ::core::ffi::c_int;
    }
    if ProfileSize
        < (::core::mem::size_of::<__llvm_profile_header>() as uint64_t)
            .wrapping_add((*Header).BinaryIdsSize)
            .wrapping_add(
                (*Header)
                    .NumData
                    .wrapping_mul(::core::mem::size_of::<__llvm_profile_data>() as uint64_t),
            )
            .wrapping_add((*Header).NamesSize)
            .wrapping_add(
                (*Header)
                    .NumCounters
                    .wrapping_mul(__llvm_profile_counter_entry_size() as uint64_t),
            )
            .wrapping_add((*Header).NumBitmapBytes)
    {
        return 1 as ::core::ffi::c_int;
    }
    SrcData = SrcDataStart;
    DstData = __llvm_profile_begin_data() as *mut __llvm_profile_data;
    while SrcData < SrcDataEnd {
        if (*SrcData).0.NameRef != (*DstData).0.NameRef
            || (*SrcData).0.FuncHash != (*DstData).0.FuncHash
            || (*SrcData).0.NumCounters != (*DstData).0.NumCounters
            || (*SrcData).0.NumBitmapBytes != (*DstData).0.NumBitmapBytes
        {
            return 1 as ::core::ffi::c_int;
        }
        SrcData = SrcData.offset(1);
        DstData = DstData.offset(1);
    }
    0 as ::core::ffi::c_int
}
unsafe extern "C" fn signextIfWin64(V: *mut ::core::ffi::c_void) -> uintptr_t {
    V as uintptr_t
}
unsafe extern "C" fn getDistanceFromCounterToValueProf(
    Header: *const __llvm_profile_header,
) -> uint64_t {
    let VTableSectionSize: uint64_t = (*Header)
        .NumVTables
        .wrapping_mul(::core::mem::size_of::<VTableProfData>() as uint64_t);
    let PaddingBytesAfterVTableSection: uint64_t =
        __llvm_profile_get_num_padding_bytes(VTableSectionSize) as uint64_t;
    let VNamesSize: uint64_t = (*Header).VNamesSize;
    let PaddingBytesAfterVNamesSize: uint64_t =
        __llvm_profile_get_num_padding_bytes(VNamesSize) as uint64_t;
    (*Header)
        .NamesSize
        .wrapping_add(__llvm_profile_get_num_padding_bytes((*Header).NamesSize) as uint64_t)
        .wrapping_add(VTableSectionSize)
        .wrapping_add(PaddingBytesAfterVTableSection)
        .wrapping_add(VNamesSize)
        .wrapping_add(PaddingBytesAfterVNamesSize)
}
#[no_mangle]
pub unsafe extern "C" fn __llvm_profile_merge_from_buffer(
    ProfileData: *const ::core::ffi::c_char,
    ProfileSize: uint64_t,
) -> ::core::ffi::c_int {
    if __llvm_profile_get_version() & VARIANT_MASK_TEMPORAL_PROF as uint64_t != 0 {
        return 1 as ::core::ffi::c_int;
    }
    let Header = ProfileData as *mut __llvm_profile_header;
    let mut CountersDelta: uintptr_t = (*Header).CountersDelta as uintptr_t;
    let mut BitmapDelta: uintptr_t = (*Header).BitmapDelta as uintptr_t;
    let SrcDataStart = ProfileData
        .add(::core::mem::size_of::<__llvm_profile_header>() as usize)
        .offset((*Header).BinaryIdsSize as isize)
        as *mut __llvm_profile_data;
    let SrcDataEnd = SrcDataStart.offset((*Header).NumData as isize);
    let SrcCountersStart: uintptr_t = SrcDataEnd as uintptr_t;
    let SrcCountersEnd: uintptr_t = (SrcCountersStart as uint64_t).wrapping_add(
        (*Header)
            .NumCounters
            .wrapping_mul(__llvm_profile_counter_entry_size() as uint64_t),
    ) as uintptr_t;
    let SrcBitmapStart: uintptr_t =
        SrcCountersEnd.wrapping_add(__llvm_profile_get_num_padding_bytes(
            SrcCountersEnd.wrapping_sub(SrcCountersStart) as uint64_t,
        ) as uintptr_t);
    let SrcNameStart: uintptr_t =
        (SrcBitmapStart as uint64_t).wrapping_add((*Header).NumBitmapBytes) as uintptr_t;
    let SrcValueProfDataStart: uintptr_t = (SrcNameStart as uint64_t)
        .wrapping_add(getDistanceFromCounterToValueProf(Header))
        as uintptr_t;
    if SrcNameStart < SrcCountersStart || SrcNameStart < SrcBitmapStart {
        return 1 as ::core::ffi::c_int;
    }
    if (*Header).NumData == 0 as uint64_t {
        let mut SrcCounter: uintptr_t = SrcCountersStart;
        let mut DstCounter: uintptr_t = __llvm_profile_begin_counters() as uintptr_t;
        while SrcCounter < SrcCountersEnd {
            if __llvm_profile_get_version() & VARIANT_MASK_BYTE_COVERAGE as uint64_t != 0 {
                let fresh0 = &mut *(DstCounter as *mut ::core::ffi::c_char);
                *fresh0 = (*fresh0 as ::core::ffi::c_int
                    & *(SrcCounter as *const ::core::ffi::c_char) as ::core::ffi::c_int)
                    as ::core::ffi::c_char;
            } else {
                let fresh1 = &mut *(DstCounter as *mut uint64_t);
                *fresh1 = (*fresh1).wrapping_add(*(SrcCounter as *const uint64_t));
            }
            SrcCounter = (SrcCounter as ::core::ffi::c_ulong)
                .wrapping_add(__llvm_profile_counter_entry_size() as ::core::ffi::c_ulong)
                as uintptr_t as uintptr_t;
            DstCounter = (DstCounter as ::core::ffi::c_ulong)
                .wrapping_add(__llvm_profile_counter_entry_size() as ::core::ffi::c_ulong)
                as uintptr_t as uintptr_t;
        }
        return 0 as ::core::ffi::c_int;
    }
    let mut SrcData: *mut __llvm_profile_data;
    let mut DstData: *mut __llvm_profile_data;
    let mut SrcValueProfData: uintptr_t;
    SrcData = SrcDataStart;
    DstData = __llvm_profile_begin_data() as *mut __llvm_profile_data;
    SrcValueProfData = SrcValueProfDataStart;
    while SrcData < SrcDataEnd {
        let DstCounters: uintptr_t =
            (DstData as uintptr_t).wrapping_add(signextIfWin64((*DstData).0.CounterPtr));
        let DstBitmap: uintptr_t =
            (DstData as uintptr_t).wrapping_add(signextIfWin64((*DstData).0.BitmapPtr));
        let mut NVK = 0 as ::core::ffi::c_uint;
        let SrcCounters: uintptr_t = SrcCountersStart
            .wrapping_add(((*SrcData).0.CounterPtr as uintptr_t).wrapping_sub(CountersDelta));
        CountersDelta =
            (CountersDelta as ::core::ffi::c_ulong)
                .wrapping_sub(
                    ::core::mem::size_of::<__llvm_profile_data>() as usize as ::core::ffi::c_ulong
                ) as uintptr_t as uintptr_t;
        let NC = (*SrcData).0.NumCounters as ::core::ffi::c_uint;
        if NC == 0 as ::core::ffi::c_uint {
            return 1 as ::core::ffi::c_int;
        }
        if SrcCounters < SrcCountersStart
            || SrcCounters >= SrcNameStart
            || SrcCounters.wrapping_add(
                (__llvm_profile_counter_entry_size() as uintptr_t).wrapping_mul(NC as uintptr_t),
            ) > SrcNameStart
        {
            return 1 as ::core::ffi::c_int;
        }
        let mut I = 0 as ::core::ffi::c_uint;
        while I < NC {
            if __llvm_profile_get_version() & VARIANT_MASK_BYTE_COVERAGE as uint64_t != 0 {
                let fresh2 = &mut *(DstCounters as *mut ::core::ffi::c_char).offset(I as isize);
                *fresh2 = (*fresh2 as ::core::ffi::c_int
                    & *(SrcCounters as *const ::core::ffi::c_char).offset(I as isize)
                        as ::core::ffi::c_int) as ::core::ffi::c_char;
            } else {
                let fresh3 = &mut *(DstCounters as *mut uint64_t).offset(I as isize);
                *fresh3 =
                    (*fresh3).wrapping_add(*(SrcCounters as *const uint64_t).offset(I as isize));
            }
            I = I.wrapping_add(1);
        }
        let SrcBitmap: uintptr_t = SrcBitmapStart
            .wrapping_add(((*SrcData).0.BitmapPtr as uintptr_t).wrapping_sub(BitmapDelta));
        BitmapDelta = (BitmapDelta as ::core::ffi::c_ulong)
            .wrapping_sub(
                ::core::mem::size_of::<__llvm_profile_data>() as usize as ::core::ffi::c_ulong
            ) as uintptr_t as uintptr_t;
        let NB = (*SrcData).0.NumBitmapBytes as ::core::ffi::c_uint;
        if NB != 0 as ::core::ffi::c_uint {
            if SrcBitmap < SrcBitmapStart || SrcBitmap.wrapping_add(NB as uintptr_t) > SrcNameStart
            {
                return 1 as ::core::ffi::c_int;
            }
            let mut I_0 = 0 as ::core::ffi::c_uint;
            while I_0 < NB {
                let fresh4 = &mut *(DstBitmap as *mut ::core::ffi::c_char).offset(I_0 as isize);
                *fresh4 = (*fresh4 as ::core::ffi::c_int
                    | *(SrcBitmap as *const ::core::ffi::c_char).offset(I_0 as isize)
                        as ::core::ffi::c_int) as ::core::ffi::c_char;
                I_0 = I_0.wrapping_add(1);
            }
        }
        if !VPMergeHook.is_none() {
            let mut I_1 = 0 as ::core::ffi::c_uint;
            while I_1 <= IPVK_Last as ::core::ffi::c_int as ::core::ffi::c_uint {
                NVK = NVK.wrapping_add(
                    ((*SrcData).0.NumValueSites[I_1 as usize] as ::core::ffi::c_int
                        != 0 as ::core::ffi::c_int) as ::core::ffi::c_int
                        as ::core::ffi::c_uint,
                );
                I_1 = I_1.wrapping_add(1);
            }
            if !(NVK == 0) {
                if SrcValueProfData as uint64_t
                    >= (ProfileData as uintptr_t as uint64_t).wrapping_add(ProfileSize)
                {
                    return 1 as ::core::ffi::c_int;
                }
                VPMergeHook.expect("non-null function pointer")(
                    SrcValueProfData as *mut ValueProfData,
                    DstData,
                );
                SrcValueProfData = SrcValueProfData.wrapping_add(
                    (*(SrcValueProfData as *mut ValueProfData)).TotalSize as uintptr_t,
                );
            }
        }
        SrcData = SrcData.offset(1);
        DstData = DstData.offset(1);
    }
    0 as ::core::ffi::c_int
}
