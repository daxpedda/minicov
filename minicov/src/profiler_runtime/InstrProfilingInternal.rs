use core::ffi::{c_int, c_uint, c_void};

use super::InstrProfData::{
    __llvm_profile_data, InstrProfValueData, ValueProfData, ValueProfNode, ValueProfRecord,
};

static mut ProfileDumped: c_uint = 0;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ProfDataIOVec {
    pub Data: *const c_void,
    pub ElmSize: usize,
    pub NumElm: usize,
    pub UseZeroPadding: c_int,
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
pub struct VPDataReaderType {
    pub InitRTRecord: Option<unsafe extern "C" fn(*const __llvm_profile_data, *mut *mut u8) -> u32>,
    pub GetValueProfRecordHeaderSize: Option<unsafe extern "C" fn(u32) -> u32>,
    pub GetFirstValueProfRecord:
        Option<unsafe extern "C" fn(*mut ValueProfData) -> *mut ValueProfRecord>,
    pub GetNumValueDataForSite: Option<unsafe extern "C" fn(u32, u32) -> u32>,
    pub GetValueProfDataSize: Option<unsafe extern "C" fn() -> u32>,
    pub GetValueData: Option<
        unsafe extern "C" fn(
            u32,
            u32,
            *mut InstrProfValueData,
            *mut ValueProfNode,
            u32,
        ) -> *mut ValueProfNode,
    >,
}

pub unsafe fn lprofSetProfileDumped(Value: c_uint) {
    ProfileDumped = Value;
}
