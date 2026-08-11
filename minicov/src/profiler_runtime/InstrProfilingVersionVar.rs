pub type uint64_t = u64;
pub const INSTR_PROF_RAW_VERSION: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
#[no_mangle]
#[linkage = "weak"]
pub static mut __llvm_profile_raw_version: uint64_t = INSTR_PROF_RAW_VERSION as uint64_t;
