use core::ffi::{c_char, c_int, c_uint, c_ulonglong, c_void};
use core::mem;

#[derive(Copy, Clone)]
#[repr(C, align(8))]
pub struct __llvm_profile_data(pub __llvm_profile_data_Inner);

#[derive(Copy, Clone)]
#[repr(C)]
pub struct __llvm_profile_data_Inner {
    pub NameRef: u64,
    pub FuncHash: u64,
    pub CounterPtr: *mut c_void,
    pub UniformCounterPtr: *mut c_void,
    pub BitmapPtr: *mut c_void,
    pub FunctionPointer: *mut c_void,
    pub Values: *mut c_void,
    pub NumCounters: u32,
    pub NumValueSites: [u16; 3],
    pub OffloadDeviceWaveSize: u16,
    pub NumBitmapBytes: u32,
}

#[derive(Copy, Clone)]
#[repr(C, align(8))]
pub struct VTableProfData(pub VTableProfData_Inner);

#[derive(Copy, Clone)]
#[repr(C)]
pub struct VTableProfData_Inner {
    pub VTableNameHash: u64,
    pub VTablePointer: *mut c_void,
    pub VTableSize: u32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ValueProfNode {
    pub Value: u64,
    pub Count: u64,
    pub Next: *mut ValueProfNode,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct __llvm_profile_header {
    pub Magic: u64,
    pub Version: u64,
    pub BinaryIdsSize: u64,
    pub NumData: u64,
    pub PaddingBytesBeforeCounters: u64,
    pub NumCounters: u64,
    pub PaddingBytesAfterCounters: u64,
    pub NumBitmapBytes: u64,
    pub PaddingBytesAfterBitmapBytes: u64,
    pub NumUniformCounters: u64,
    pub PaddingBytesAfterUniformCounters: u64,
    pub UniformCountersDelta: u64,
    pub NamesSize: u64,
    pub CountersDelta: u64,
    pub BitmapDelta: u64,
    pub NamesDelta: u64,
    pub NumVTables: u64,
    pub VNamesSize: u64,
    pub ValueKindLast: u64,
}

pub const IPVK_First: c_uint = 0;
pub const IPVK_Last: c_uint = 2;

#[derive(Copy, Clone)]
#[repr(C, align(8))]
pub struct __llvm_gcov_init_func_struct(pub __llvm_gcov_init_func_struct_Inner);

#[derive(Copy, Clone)]
#[repr(C)]
pub struct __llvm_gcov_init_func_struct_Inner {
    pub WriteoutFunction: *mut c_void,
    pub ResetFunction: *mut c_void,
}

pub const INSTR_PROF_MAX_NUM_VAL_PER_SITE: c_int = 255;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ValueProfRecord {
    pub Kind: u32,
    pub NumValueSites: u32,
    pub SiteCountArray: [u8; 1],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ValueProfData {
    pub TotalSize: u32,
    pub NumValueKinds: u32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ValueProfRecordClosure {
    pub Record: *const c_void,
    pub GetNumValueKinds: Option<unsafe extern "C" fn(*const c_void) -> u32>,
    pub GetNumValueSites: Option<unsafe extern "C" fn(*const c_void, u32) -> u32>,
    pub GetNumValueData: Option<unsafe extern "C" fn(*const c_void, u32) -> u32>,
    pub GetNumValueDataForSite: Option<unsafe extern "C" fn(*const c_void, u32, u32) -> u32>,
    pub RemapValueData: Option<unsafe extern "C" fn(u32, u64) -> u64>,
    pub GetValueForSite:
        Option<unsafe extern "C" fn(*const c_void, *mut InstrProfValueData, u32, u32) -> ()>,
    pub AllocValueProfData: Option<unsafe extern "C" fn(usize) -> *mut ValueProfData>,
}

pub unsafe extern "C" fn getValueProfRecordHeaderSize(NumValueSites: u32) -> u32 {
    let mut Size: u32 =
        8_usize.wrapping_add((mem::size_of::<u8>()).wrapping_mul(NumValueSites as usize)) as u32;
    Size = Size.wrapping_add(7_u32) & !(7 as c_int) as u32;
    Size
}

pub unsafe fn getValueProfRecordSize(NumValueSites: u32, NumValueData: u32) -> u32 {
    (getValueProfRecordHeaderSize(NumValueSites) as usize)
        .wrapping_add((mem::size_of::<InstrProfValueData>()).wrapping_mul(NumValueData as usize))
        as u32
}

pub unsafe fn getValueProfRecordValueData(This: *mut ValueProfRecord) -> *mut InstrProfValueData {
    (This as *mut c_char).offset(getValueProfRecordHeaderSize((*This).NumValueSites) as isize)
        as *mut InstrProfValueData
}

pub unsafe fn getValueProfRecordNumValueData(This: *mut ValueProfRecord) -> u32 {
    let mut NumValueData: u32 = 0;
    let mut I: u32 = 0;
    while I < (*This).NumValueSites {
        NumValueData = NumValueData
            .wrapping_add(*(*This).SiteCountArray.as_mut_ptr().offset(I as isize) as u32);
        I = I.wrapping_add(1);
    }
    NumValueData
}

pub unsafe fn getValueProfRecordNext(This: *mut ValueProfRecord) -> *mut ValueProfRecord {
    let NumValueData = getValueProfRecordNumValueData(This);
    (This as *mut c_char)
        .offset(getValueProfRecordSize((*This).NumValueSites, NumValueData) as isize)
        as *mut ValueProfRecord
}

pub unsafe extern "C" fn getFirstValueProfRecord(This: *mut ValueProfData) -> *mut ValueProfRecord {
    (This as *mut c_char).add(mem::size_of::<ValueProfData>()) as *mut ValueProfRecord
}

pub unsafe fn getValueProfDataSize(Closure: *mut ValueProfRecordClosure) -> u32 {
    let mut Kind: u32;
    let mut TotalSize: u32 = mem::size_of::<ValueProfData>() as u32;
    let Record = (*Closure).Record;
    Kind = IPVK_First as c_int as u32;
    while Kind <= IPVK_Last as c_int as u32 {
        let NumValueSites = (*Closure)
            .GetNumValueSites
            .expect("non-null function pointer")(Record, Kind);
        if !(NumValueSites == 0) {
            TotalSize = TotalSize.wrapping_add(getValueProfRecordSize(
                NumValueSites,
                (*Closure)
                    .GetNumValueData
                    .expect("non-null function pointer")(Record, Kind),
            ));
        }
        Kind = Kind.wrapping_add(1);
    }
    TotalSize
}

pub const INSTR_PROF_RAW_VERSION: c_int = 11;

pub const VARIANT_MASKS_ALL: u64 = 0xffffffff00000000;
pub const VARIANT_MASK_BYTE_COVERAGE: c_ulonglong = 0x1 << 60 as c_int;
pub const VARIANT_MASK_TEMPORAL_PROF: c_ulonglong = 0x1 << 63 as c_int;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct InstrProfValueData {
    pub Value: u64,
    pub Count: u64,
}

pub unsafe fn InstProfClzll(X: c_ulonglong) -> c_int {
    X.leading_zeros() as c_int
}

pub unsafe fn InstProfPopcountll(X: c_ulonglong) -> c_int {
    X.count_ones() as c_int
}

pub unsafe fn InstrProfGetRangeRepValue(Value: u64) -> u64 {
    if Value <= 8 {
        Value
    } else if Value >= 513 {
        513
    } else if InstProfPopcountll(Value as c_ulonglong) == 1 {
        Value
    } else {
        (1_u64 << (64 - InstProfClzll(Value as c_ulonglong) - 1)).wrapping_add(1)
    }
}
