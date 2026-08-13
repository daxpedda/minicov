use core::ffi::{c_char, c_int, c_void};
use core::mem;
use core::ptr;

use super::InstrProfData::{
    __llvm_profile_data, IPVK_First, IPVK_Last, ValueProfNode, VARIANT_MASK_BYTE_COVERAGE,
    VARIANT_MASK_TEMPORAL_PROF,
};
use super::InstrProfilingInternal::lprofSetProfileDumped;
use super::InstrProfilingPlatform::{
    __llvm_profile_begin_bitmap, __llvm_profile_begin_counters, __llvm_profile_begin_data,
    __llvm_profile_end_bitmap, __llvm_profile_end_counters, __llvm_profile_end_data,
};
use super::InstrProfilingVersionVar::__llvm_profile_raw_version;

static mut __llvm_profile_global_timestamp: u32 = 1;

pub unsafe fn __llvm_profile_set_timestamp(Probe: *mut u64) {
    if *Probe == 0 || *Probe == -(1 as c_int) as u64 {
        let fresh0 = __llvm_profile_global_timestamp;
        __llvm_profile_global_timestamp = __llvm_profile_global_timestamp.wrapping_add(1);
        *Probe = fresh0 as u64;
    }
}

pub unsafe fn __llvm_profile_get_magic() -> u64 {
    if mem::size_of::<*mut c_void>() == mem::size_of::<u64>() {
        255_u64 << 56
            | ('l' as u64) << 48
            | ('p' as u64) << 40
            | ('r' as u64) << 32
            | ('o' as u64) << 24
            | ('f' as u64) << 16
            | ('r' as u64) << 8
            | 129_u64
    } else {
        255_u64 << 56
            | ('l' as u64) << 48
            | ('p' as u64) << 40
            | ('r' as u64) << 32
            | ('o' as u64) << 24
            | ('f' as u64) << 16
            | ('R' as u64) << 8
            | 129_u64
    }
}

pub unsafe fn __llvm_profile_set_dumped() {
    lprofSetProfileDumped(1);
}

pub unsafe fn __llvm_profile_get_num_padding_bytes(SizeInBytes: u64) -> u8 {
    (7_u64
        & (mem::size_of::<u64>() as u64)
            .wrapping_sub(SizeInBytes.wrapping_rem(mem::size_of::<u64>() as u64))) as u8
}

pub unsafe fn __llvm_profile_get_version() -> u64 {
    __llvm_profile_raw_version
}

pub unsafe fn __llvm_profile_reset_counters() {
    #[expect(clippy::unnecessary_cast)]
    if __llvm_profile_get_version() & VARIANT_MASK_TEMPORAL_PROF as u64 != 0 {
        __llvm_profile_global_timestamp = 1;
    }
    let mut I = __llvm_profile_begin_counters();
    let mut E = __llvm_profile_end_counters();
    #[expect(clippy::unnecessary_cast)]
    let ResetValue = (if __llvm_profile_get_version() & VARIANT_MASK_BYTE_COVERAGE as u64 != 0 {
        0xff
    } else {
        0
    }) as c_char;
    ptr::write_bytes(I, ResetValue as u8, E.offset_from(I) as usize);
    I = __llvm_profile_begin_bitmap();
    E = __llvm_profile_end_bitmap();
    ptr::write_bytes(I as *mut c_void, 0, E.offset_from(I) as usize);
    let DataBegin = __llvm_profile_begin_data();
    let DataEnd = __llvm_profile_end_data();
    let mut DI: *const __llvm_profile_data;
    DI = DataBegin;
    while DI < DataEnd {
        let mut CurrentVSiteCount: u64 = 0;
        let mut VKI: u32;
        let mut i: u32;
        if !(*DI).0.Values.is_null() {
            let ValueCounters = (*DI).0.Values as *mut *mut ValueProfNode;
            VKI = IPVK_First as c_int as u32;
            while VKI <= IPVK_Last as c_int as u32 {
                CurrentVSiteCount =
                    CurrentVSiteCount.wrapping_add((*DI).0.NumValueSites[VKI as usize] as u64);
                VKI = VKI.wrapping_add(1);
            }
            i = 0;
            while (i as u64) < CurrentVSiteCount {
                let mut CurrVNode = *ValueCounters.offset(i as isize);
                while !CurrVNode.is_null() {
                    (*CurrVNode).Count = 0;
                    CurrVNode = (*CurrVNode).Next;
                }
                i = i.wrapping_add(1);
            }
        }
        DI = DI.offset(1);
    }
    lprofSetProfileDumped(0);
}
