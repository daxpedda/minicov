use core::ptr;

use super::InstrProfData::{
    __llvm_profile_data, IPVK_First, IPVK_Last, ValueProfNode, VARIANT_MASK_BYTE_COVERAGE,
    VARIANT_MASK_TEMPORAL_PROF,
};
use super::InstrProfilingInternal::lprofSetProfileDumped;
use super::InstrProfilingPlatformLinux::{
    __llvm_profile_begin_bitmap, __llvm_profile_begin_counters, __llvm_profile_begin_data,
    __llvm_profile_end_bitmap, __llvm_profile_end_counters, __llvm_profile_end_data,
};
use super::InstrProfilingVersionVar::__llvm_profile_raw_version;

pub type uint64_t = u64;
pub type uint32_t = u32;
pub type uint8_t = u8;

static mut __llvm_profile_global_timestamp: uint32_t = 1 as uint32_t;

#[no_mangle]
pub unsafe extern "C" fn __llvm_profile_set_timestamp(Probe: *mut uint64_t) {
    if *Probe == 0 as uint64_t || *Probe == -(1 as ::core::ffi::c_int) as uint64_t {
        let fresh0 = __llvm_profile_global_timestamp;
        __llvm_profile_global_timestamp = __llvm_profile_global_timestamp.wrapping_add(1);
        *Probe = fresh0 as uint64_t;
    }
}

#[no_mangle]
pub unsafe extern "C" fn __llvm_profile_get_magic() -> uint64_t {
    if ::core::mem::size_of::<*mut ::core::ffi::c_void>() as usize
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
    }
}

#[no_mangle]
pub unsafe extern "C" fn __llvm_profile_set_dumped() {
    lprofSetProfileDumped(1 as ::core::ffi::c_uint);
}

#[no_mangle]
pub unsafe extern "C" fn __llvm_profile_get_num_padding_bytes(SizeInBytes: uint64_t) -> uint8_t {
    (7 as uint64_t
        & (::core::mem::size_of::<uint64_t>() as uint64_t)
            .wrapping_sub(SizeInBytes.wrapping_rem(::core::mem::size_of::<uint64_t>() as uint64_t)))
        as uint8_t
}

#[no_mangle]
pub unsafe extern "C" fn __llvm_profile_get_version() -> uint64_t {
    __llvm_profile_raw_version
}

#[no_mangle]
pub unsafe extern "C" fn __llvm_profile_reset_counters() {
    if __llvm_profile_get_version() & VARIANT_MASK_TEMPORAL_PROF as uint64_t != 0 {
        __llvm_profile_global_timestamp = 1 as uint32_t;
    }
    let mut I = __llvm_profile_begin_counters();
    let mut E = __llvm_profile_end_counters();
    let ResetValue = (if __llvm_profile_get_version() & VARIANT_MASK_BYTE_COVERAGE as uint64_t != 0
    {
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
    let DataBegin = __llvm_profile_begin_data();
    let DataEnd = __llvm_profile_end_data();
    let mut DI: *const __llvm_profile_data;
    DI = DataBegin;
    while DI < DataEnd {
        let mut CurrentVSiteCount: uint64_t = 0 as uint64_t;
        let mut VKI: uint32_t;
        let mut i: uint32_t;
        if !(*DI).0.Values.is_null() {
            let ValueCounters = (*DI).0.Values as *mut *mut ValueProfNode;
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
                    CurrVNode = (*CurrVNode).Next;
                }
                i = i.wrapping_add(1);
            }
        }
        DI = DI.offset(1);
    }
    lprofSetProfileDumped(0 as ::core::ffi::c_uint);
}
