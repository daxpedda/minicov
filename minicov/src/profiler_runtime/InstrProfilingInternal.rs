use super::InstrProfData::{
    __llvm_profile_data, InstrProfValueData, ValueProfData, ValueProfNode, ValueProfRecord,
};

pub type size_t = usize;
pub type uint32_t = u32;
pub type uint8_t = u8;

static mut ProfileDumped: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ProfDataIOVec {
    pub Data: *const ::core::ffi::c_void,
    pub ElmSize: size_t,
    pub NumElm: size_t,
    pub UseZeroPadding: ::core::ffi::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ProfDataWriter {
    pub Write: WriterCallback,
    pub WriterCtx: *mut ::core::ffi::c_void,
}

pub type WriterCallback =
    Option<unsafe extern "C" fn(*mut ProfDataWriter, *mut ProfDataIOVec, uint32_t) -> uint32_t>;

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

#[no_mangle]
pub unsafe extern "C" fn lprofProfileDumped() -> ::core::ffi::c_uint {
    ProfileDumped
}

#[no_mangle]
pub unsafe extern "C" fn lprofSetProfileDumped(Value: ::core::ffi::c_uint) {
    ProfileDumped = Value;
}
