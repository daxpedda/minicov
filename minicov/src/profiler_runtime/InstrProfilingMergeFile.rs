use super::InstrProfData::{
    __llvm_profile_data, getFirstValueProfRecord, getValueProfRecordNext,
    getValueProfRecordValueData, InstrProfValueData, ValueProfData,
};
use super::InstrProfilingValue::__llvm_profile_instrument_target_value;

pub type uint32_t = u32;
pub type uint8_t = u8;

pub unsafe extern "C" fn lprofMergeValueProfData(
    SrcValueProfData: *mut ValueProfData,
    DstData: *mut __llvm_profile_data,
) {
    let mut I: ::core::ffi::c_uint;
    let mut S: ::core::ffi::c_uint;
    let mut V: ::core::ffi::c_uint;
    let mut DstIndex = 0 as ::core::ffi::c_uint;
    let mut VData: *mut InstrProfValueData;
    let mut VR = getFirstValueProfRecord(SrcValueProfData);
    I = 0 as ::core::ffi::c_uint;
    while (I as uint32_t) < (*SrcValueProfData).NumValueKinds {
        VData = getValueProfRecordValueData(VR);
        let mut SrcIndex = 0 as ::core::ffi::c_uint;
        S = 0 as ::core::ffi::c_uint;
        while (S as uint32_t) < (*VR).NumValueSites {
            let NV: uint8_t = *(&raw mut (*VR).SiteCountArray as *mut uint8_t).offset(S as isize);
            V = 0 as ::core::ffi::c_uint;
            while V < NV as ::core::ffi::c_uint {
                __llvm_profile_instrument_target_value(
                    (*VData.offset(SrcIndex as isize)).Value,
                    DstData as *mut ::core::ffi::c_void,
                    DstIndex as uint32_t,
                    (*VData.offset(SrcIndex as isize)).Count,
                );
                SrcIndex = SrcIndex.wrapping_add(1);
                V = V.wrapping_add(1);
            }
            DstIndex = DstIndex.wrapping_add(1);
            S = S.wrapping_add(1);
        }
        VR = getValueProfRecordNext(VR);
        I = I.wrapping_add(1);
    }
}
