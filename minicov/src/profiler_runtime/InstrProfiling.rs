use core::ptr;

extern "C" {
    fn lprofSetProfileDumped(_: ::core::ffi::c_uint);
    fn __llvm_profile_begin_data() -> *const __llvm_profile_data;
    fn __llvm_profile_end_data() -> *const __llvm_profile_data;
    fn __llvm_profile_begin_counters() -> *mut ::core::ffi::c_char;
    fn __llvm_profile_end_counters() -> *mut ::core::ffi::c_char;
    fn __llvm_profile_begin_bitmap() -> *mut ::core::ffi::c_char;
    fn __llvm_profile_end_bitmap() -> *mut ::core::ffi::c_char;
    static mut __llvm_profile_raw_version: uint64_t;
}
pub type uint64_t = u64;
pub type uint32_t = u32;
pub type uint16_t = u16;
pub type uint8_t = u8;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ValueProfNode {
    pub Value: uint64_t,
    pub Count: uint64_t,
    pub Next: PtrToNodeT,
}
pub type PtrToNodeT = *mut ValueProfNode;
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
pub const IPVK_Last: ValueKind = 2;
pub const IPVK_First: ValueKind = 0;
pub type ValueKind = ::core::ffi::c_uint;
pub const IPVK_VTableTarget: ValueKind = 2;
pub const IPVK_MemOPSize: ValueKind = 1;
pub const IPVK_IndirectCallTarget: ValueKind = 0;
pub const INSTR_PROF_RAW_MAGIC_64: uint64_t = (255 as ::core::ffi::c_int as uint64_t)
    << 56 as ::core::ffi::c_int
    | ('l' as i32 as uint64_t) << 48 as ::core::ffi::c_int
    | ('p' as i32 as uint64_t) << 40 as ::core::ffi::c_int
    | ('r' as i32 as uint64_t) << 32 as ::core::ffi::c_int
    | ('o' as i32 as uint64_t) << 24 as ::core::ffi::c_int
    | ('f' as i32 as uint64_t) << 16 as ::core::ffi::c_int
    | ('r' as i32 as uint64_t) << 8 as ::core::ffi::c_int
    | 129 as ::core::ffi::c_int as uint64_t;
pub const INSTR_PROF_RAW_MAGIC_32: uint64_t = (255 as ::core::ffi::c_int as uint64_t)
    << 56 as ::core::ffi::c_int
    | ('l' as i32 as uint64_t) << 48 as ::core::ffi::c_int
    | ('p' as i32 as uint64_t) << 40 as ::core::ffi::c_int
    | ('r' as i32 as uint64_t) << 32 as ::core::ffi::c_int
    | ('o' as i32 as uint64_t) << 24 as ::core::ffi::c_int
    | ('f' as i32 as uint64_t) << 16 as ::core::ffi::c_int
    | ('R' as i32 as uint64_t) << 8 as ::core::ffi::c_int
    | 129 as ::core::ffi::c_int as uint64_t;
pub const VARIANT_MASK_BYTE_COVERAGE: ::core::ffi::c_ulonglong =
    (0x1 as ::core::ffi::c_ulonglong) << 60 as ::core::ffi::c_int;
pub const VARIANT_MASK_TEMPORAL_PROF: ::core::ffi::c_ulonglong =
    (0x1 as ::core::ffi::c_ulonglong) << 63 as ::core::ffi::c_int;
static mut __llvm_profile_global_timestamp: uint32_t = 1 as uint32_t;
#[no_mangle]
pub unsafe extern "C" fn __llvm_profile_set_timestamp(mut Probe: *mut uint64_t) {
    if *Probe == 0 as uint64_t || *Probe == -(1 as ::core::ffi::c_int) as uint64_t {
        let fresh0 = __llvm_profile_global_timestamp;
        __llvm_profile_global_timestamp = __llvm_profile_global_timestamp.wrapping_add(1);
        *Probe = fresh0 as uint64_t;
    }
}
#[no_mangle]
pub unsafe extern "C" fn __llvm_profile_get_magic() -> uint64_t {
    return if ::core::mem::size_of::<*mut ::core::ffi::c_void>() as usize
        == ::core::mem::size_of::<uint64_t>() as usize
    {
        (255 as ::core::ffi::c_int as uint64_t) << 56 as ::core::ffi::c_int
            | ('l' as i32 as uint64_t) << 48 as ::core::ffi::c_int
            | ('p' as i32 as uint64_t) << 40 as ::core::ffi::c_int
            | ('r' as i32 as uint64_t) << 32 as ::core::ffi::c_int
            | ('o' as i32 as uint64_t) << 24 as ::core::ffi::c_int
            | ('f' as i32 as uint64_t) << 16 as ::core::ffi::c_int
            | ('r' as i32 as uint64_t) << 8 as ::core::ffi::c_int
            | 129 as ::core::ffi::c_int as uint64_t
    } else {
        (255 as ::core::ffi::c_int as uint64_t) << 56 as ::core::ffi::c_int
            | ('l' as i32 as uint64_t) << 48 as ::core::ffi::c_int
            | ('p' as i32 as uint64_t) << 40 as ::core::ffi::c_int
            | ('r' as i32 as uint64_t) << 32 as ::core::ffi::c_int
            | ('o' as i32 as uint64_t) << 24 as ::core::ffi::c_int
            | ('f' as i32 as uint64_t) << 16 as ::core::ffi::c_int
            | ('R' as i32 as uint64_t) << 8 as ::core::ffi::c_int
            | 129 as ::core::ffi::c_int as uint64_t
    };
}
#[no_mangle]
pub unsafe extern "C" fn __llvm_profile_set_dumped() {
    lprofSetProfileDumped(1 as ::core::ffi::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn __llvm_profile_get_num_padding_bytes(
    mut SizeInBytes: uint64_t,
) -> uint8_t {
    return (7 as uint64_t
        & (::core::mem::size_of::<uint64_t>() as uint64_t)
            .wrapping_sub(SizeInBytes.wrapping_rem(::core::mem::size_of::<uint64_t>() as uint64_t)))
        as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn __llvm_profile_get_version() -> uint64_t {
    return __llvm_profile_raw_version;
}
#[no_mangle]
pub unsafe extern "C" fn __llvm_profile_reset_counters() {
    if __llvm_profile_get_version() & VARIANT_MASK_TEMPORAL_PROF as uint64_t != 0 {
        __llvm_profile_global_timestamp = 1 as uint32_t;
    }
    let mut I = __llvm_profile_begin_counters();
    let mut E = __llvm_profile_end_counters();
    let mut ResetValue =
        (if __llvm_profile_get_version() & VARIANT_MASK_BYTE_COVERAGE as uint64_t != 0 {
            0xff as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as ::core::ffi::c_char;
    ptr::write_bytes(
        I,
        ResetValue as u8,
        E.offset_from(I) as ::core::ffi::c_long as ::core::ffi::c_ulong as usize,
    );
    I = __llvm_profile_begin_bitmap();
    E = __llvm_profile_end_bitmap();
    ptr::write_bytes(
        I as *mut ::core::ffi::c_void,
        0,
        E.offset_from(I) as ::core::ffi::c_long as ::core::ffi::c_ulong as usize,
    );
    let mut DataBegin = __llvm_profile_begin_data();
    let mut DataEnd = __llvm_profile_end_data();
    let mut DI = ::core::ptr::null::<__llvm_profile_data>();
    DI = DataBegin;
    while DI < DataEnd {
        let mut CurrentVSiteCount: uint64_t = 0 as uint64_t;
        let mut VKI: uint32_t = 0;
        let mut i: uint32_t = 0;
        if !(*DI).0.Values.is_null() {
            let mut ValueCounters = (*DI).0.Values as *mut *mut ValueProfNode;
            VKI = IPVK_First as ::core::ffi::c_int as uint32_t;
            while VKI <= IPVK_Last as ::core::ffi::c_int as uint32_t {
                CurrentVSiteCount =
                    CurrentVSiteCount.wrapping_add((*DI).0.NumValueSites[VKI as usize] as uint64_t);
                VKI = VKI.wrapping_add(1);
            }
            i = 0 as uint32_t;
            while (i as uint64_t) < CurrentVSiteCount {
                let mut CurrVNode = *ValueCounters.offset(i as isize);
                while !CurrVNode.is_null() {
                    (*CurrVNode).Count = 0 as uint64_t;
                    CurrVNode = (*CurrVNode).Next as *mut ValueProfNode;
                }
                i = i.wrapping_add(1);
            }
        }
        DI = DI.offset(1);
    }
    lprofSetProfileDumped(0 as ::core::ffi::c_uint);
}
