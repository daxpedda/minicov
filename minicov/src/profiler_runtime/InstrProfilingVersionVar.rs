use super::InstrProfData::INSTR_PROF_RAW_VERSION;

#[no_mangle]
#[linkage = "weak"]
pub static mut __llvm_profile_raw_version: u64 = INSTR_PROF_RAW_VERSION as u64;
