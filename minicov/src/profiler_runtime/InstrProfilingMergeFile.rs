use ::core::ffi::{c_uint, c_void};

use super::InstrProfData::{
    __llvm_profile_data, getFirstValueProfRecord, getValueProfRecordNext,
    getValueProfRecordValueData, InstrProfValueData, ValueProfData,
};
use super::InstrProfilingValue::__llvm_profile_instrument_target_value;

pub unsafe extern "C" fn lprofMergeValueProfData(
    SrcValueProfData: *mut ValueProfData,
    DstData: *mut __llvm_profile_data,
) {
    let mut I: c_uint;
    let mut S: c_uint;
    let mut V: c_uint;
    let mut DstIndex: c_uint = 0;
    let mut VData: *mut InstrProfValueData;
    let mut VR = getFirstValueProfRecord(SrcValueProfData);
    I = 0;
    while (I as u32) < (*SrcValueProfData).NumValueKinds {
        VData = getValueProfRecordValueData(VR);
        let mut SrcIndex: c_uint = 0;
        S = 0;
        while (S as u32) < (*VR).NumValueSites {
            let NV: u8 = *(*VR).SiteCountArray.as_mut_ptr().offset(S as isize);
            V = 0;
            while V < NV as c_uint {
                __llvm_profile_instrument_target_value(
                    (*VData.offset(SrcIndex as isize)).Value,
                    DstData as *mut c_void,
                    DstIndex as u32,
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
