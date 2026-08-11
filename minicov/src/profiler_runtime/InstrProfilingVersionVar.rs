use super::InstrProfData::INSTR_PROF_RAW_VERSION;

pub type uint64_t = u64;

#[no_mangle]
#[linkage = "weak"]
pub static mut __llvm_profile_raw_version: uint64_t = INSTR_PROF_RAW_VERSION as uint64_t;
