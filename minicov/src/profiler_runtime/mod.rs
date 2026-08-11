#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

mod InstrProfData;
mod InstrProfiling;
mod InstrProfilingBuffer;
mod InstrProfilingInternal;
mod InstrProfilingMerge;
mod InstrProfilingMergeFile;
mod InstrProfilingPlatformLinux;
mod InstrProfilingValue;
mod InstrProfilingVersionVar;
mod InstrProfilingWriter;

#[cfg(feature = "alloc")]
use core::alloc::Layout;

pub use InstrProfData::{INSTR_PROF_RAW_VERSION, VARIANT_MASKS_ALL};
pub use InstrProfiling::{__llvm_profile_get_version, __llvm_profile_reset_counters};
pub use InstrProfilingInternal::{ProfDataIOVec, ProfDataWriter};
pub use InstrProfilingMerge::{
    __llvm_profile_check_compatibility, __llvm_profile_merge_from_buffer,
    lprofGetLoadModuleSignature,
};
pub use InstrProfilingPlatformLinux::{__llvm_profile_begin_counters, __llvm_profile_end_counters};
pub use InstrProfilingValue::lprofGetVPDataReader;
pub use InstrProfilingWriter::lprofWriteData;

// Memory allocation functions used by value profiling. If the "alloc" feature
// is disabled then value profiling will also be disabled.
#[cfg(feature = "alloc")]
unsafe fn minicov_alloc_zeroed(size: usize, align: usize) -> *mut u8 {
    alloc::alloc::alloc_zeroed(Layout::from_size_align(size, align).unwrap())
}
#[cfg(feature = "alloc")]
unsafe fn minicov_dealloc(ptr: *mut u8, size: usize, align: usize) {
    alloc::alloc::dealloc(ptr, Layout::from_size_align(size, align).unwrap())
}
#[cfg(not(feature = "alloc"))]
unsafe fn minicov_alloc_zeroed(_size: usize, _align: usize) -> *mut u8 {
    core::ptr::null_mut()
}
#[cfg(not(feature = "alloc"))]
unsafe fn minicov_dealloc(_ptr: *mut u8, _size: usize, _align: usize) {}
