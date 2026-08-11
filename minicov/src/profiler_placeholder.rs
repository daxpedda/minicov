#![allow(non_snake_case)]

use core::ffi::{c_int, c_void};
use core::ptr;

use self::Internal::VPDataReaderType;

pub const INSTR_PROF_RAW_VERSION: c_int = 11;
pub const VARIANT_MASKS_ALL: u64 = 0xffffffff00000000;

pub unsafe fn __llvm_profile_begin_counters() -> *const u8 {
    ptr::null()
}

pub unsafe fn __llvm_profile_end_counters() -> *const u8 {
    ptr::null()
}

pub unsafe fn __llvm_profile_reset_counters() {}

pub unsafe fn __llvm_profile_merge_from_buffer(_: *const u8, _: u64) -> i32 {
    0
}

pub unsafe fn __llvm_profile_check_compatibility(_: *const u8, _: u64) -> i32 {
    0
}

pub unsafe fn __llvm_profile_get_version() -> u64 {
    0
}

pub unsafe fn lprofWriteData(_: *mut ProfDataWriter, _: *mut VPDataReaderType, _: i32) -> i32 {
    0
}

pub unsafe fn lprofGetVPDataReader() -> *mut VPDataReaderType {
    ptr::null_mut()
}

pub unsafe fn lprofGetLoadModuleSignature() -> u64 {
    0
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ProfDataWriter {
    pub Write: WriterCallback,
    pub WriterCtx: *mut c_void,
}

pub type WriterCallback =
    Option<unsafe extern "C" fn(*mut ProfDataWriter, *mut ProfDataIOVec, u32) -> u32>;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ProfDataIOVec {
    pub Data: *const c_void,
    pub ElmSize: usize,
    pub NumElm: usize,
    pub UseZeroPadding: c_int,
}

mod Internal {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct VPDataReaderType {}
}
