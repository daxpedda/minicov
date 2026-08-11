extern "C" {
    fn getFirstValueProfRecord(VPD: *mut ValueProfData) -> *mut ValueProfRecord;
    fn getValueProfRecordNext(VPR: *mut ValueProfRecord) -> *mut ValueProfRecord;
    fn getValueProfRecordValueData(VPR: *mut ValueProfRecord) -> *mut InstrProfValueData;
    fn __llvm_profile_instrument_target_value(
        TargetValue: uint64_t,
        Data: *mut ::core::ffi::c_void,
        CounterIndex: uint32_t,
        CounterValue: uint64_t,
    );
}
pub type uint64_t = u64;
pub type uint32_t = u32;
pub type uint16_t = u16;
pub type uint8_t = u8;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct InstrProfValueData {
    pub Value: uint64_t,
    pub Count: uint64_t,
}
pub type IntPtrT = *mut ::core::ffi::c_void;
#[derive(Copy, Clone)]
#[repr(C, align(8))]
pub struct __llvm_profile_data(pub __llvm_profile_data_Inner);
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __llvm_profile_data_Inner {
    pub NameRef: uint64_t,
    pub FuncHash: uint64_t,
    pub CounterPtr: IntPtrT,
    pub UniformCounterPtr: IntPtrT,
    pub BitmapPtr: IntPtrT,
    pub FunctionPointer: IntPtrT,
    pub Values: IntPtrT,
    pub NumCounters: uint32_t,
    pub NumValueSites: [uint16_t; 3],
    pub OffloadDeviceWaveSize: uint16_t,
    pub NumBitmapBytes: uint32_t,
}
#[allow(dead_code, non_upper_case_globals)]
const __llvm_profile_data_PADDING: usize = ::core::mem::size_of::<__llvm_profile_data>()
    - ::core::mem::size_of::<__llvm_profile_data_Inner>();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ValueProfData {
    pub TotalSize: uint32_t,
    pub NumValueKinds: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ValueProfRecord {
    pub Kind: uint32_t,
    pub NumValueSites: uint32_t,
    pub SiteCountArray: [uint8_t; 1],
}
#[no_mangle]
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
