pub type uint64_t = u64;
pub type uint32_t = u32;
pub type uint16_t = u16;
pub type uint8_t = u8;
pub type size_t = usize;
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

#[derive(Copy, Clone)]
#[repr(C, align(8))]
pub struct VTableProfData(pub VTableProfData_Inner);

#[derive(Copy, Clone)]
#[repr(C)]
pub struct VTableProfData_Inner {
    pub VTableNameHash: uint64_t,
    pub VTablePointer: IntPtrT,
    pub VTableSize: uint32_t,
}

pub type PtrToNodeT = *mut ValueProfNode;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ValueProfNode {
    pub Value: uint64_t,
    pub Count: uint64_t,
    pub Next: PtrToNodeT,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct __llvm_profile_header {
    pub Magic: uint64_t,
    pub Version: uint64_t,
    pub BinaryIdsSize: uint64_t,
    pub NumData: uint64_t,
    pub PaddingBytesBeforeCounters: uint64_t,
    pub NumCounters: uint64_t,
    pub PaddingBytesAfterCounters: uint64_t,
    pub NumBitmapBytes: uint64_t,
    pub PaddingBytesAfterBitmapBytes: uint64_t,
    pub NumUniformCounters: uint64_t,
    pub PaddingBytesAfterUniformCounters: uint64_t,
    pub UniformCountersDelta: uint64_t,
    pub NamesSize: uint64_t,
    pub CountersDelta: uint64_t,
    pub BitmapDelta: uint64_t,
    pub NamesDelta: uint64_t,
    pub NumVTables: uint64_t,
    pub VNamesSize: uint64_t,
    pub ValueKindLast: uint64_t,
}

pub type ValueKind = ::core::ffi::c_uint;
pub const IPVK_First: ValueKind = 0;
pub const IPVK_Last: ValueKind = 2;

#[derive(Copy, Clone)]
#[repr(C, align(8))]
pub struct __llvm_gcov_init_func_struct(pub __llvm_gcov_init_func_struct_Inner);

#[derive(Copy, Clone)]
#[repr(C)]
pub struct __llvm_gcov_init_func_struct_Inner {
    pub WriteoutFunction: IntPtrT,
    pub ResetFunction: IntPtrT,
}

pub const INSTR_PROF_MAX_NUM_VAL_PER_SITE: ::core::ffi::c_int = 255 as ::core::ffi::c_int;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ValueProfRecord {
    pub Kind: uint32_t,
    pub NumValueSites: uint32_t,
    pub SiteCountArray: [uint8_t; 1],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ValueProfData {
    pub TotalSize: uint32_t,
    pub NumValueKinds: uint32_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ValueProfRecordClosure {
    pub Record: *const ::core::ffi::c_void,
    pub GetNumValueKinds: Option<unsafe extern "C" fn(*const ::core::ffi::c_void) -> uint32_t>,
    pub GetNumValueSites:
        Option<unsafe extern "C" fn(*const ::core::ffi::c_void, uint32_t) -> uint32_t>,
    pub GetNumValueData:
        Option<unsafe extern "C" fn(*const ::core::ffi::c_void, uint32_t) -> uint32_t>,
    pub GetNumValueDataForSite:
        Option<unsafe extern "C" fn(*const ::core::ffi::c_void, uint32_t, uint32_t) -> uint32_t>,
    pub RemapValueData: Option<unsafe extern "C" fn(uint32_t, uint64_t) -> uint64_t>,
    pub GetValueForSite: Option<
        unsafe extern "C" fn(
            *const ::core::ffi::c_void,
            *mut InstrProfValueData,
            uint32_t,
            uint32_t,
        ) -> (),
    >,
    pub AllocValueProfData: Option<unsafe extern "C" fn(size_t) -> *mut ValueProfData>,
}

pub unsafe extern "C" fn getValueProfRecordHeaderSize(NumValueSites: uint32_t) -> uint32_t {
    let mut Size: uint32_t = 8_usize.wrapping_add(
        (::core::mem::size_of::<uint8_t>() as usize).wrapping_mul(NumValueSites as usize),
    ) as uint32_t;
    Size = Size.wrapping_add(7 as uint32_t) & !(7 as ::core::ffi::c_int) as uint32_t;
    Size
}

pub unsafe fn getValueProfRecordSize(NumValueSites: uint32_t, NumValueData: uint32_t) -> uint32_t {
    (getValueProfRecordHeaderSize(NumValueSites) as usize).wrapping_add(
        (::core::mem::size_of::<InstrProfValueData>() as usize).wrapping_mul(NumValueData as usize),
    ) as uint32_t
}

pub unsafe fn getValueProfRecordValueData(This: *mut ValueProfRecord) -> *mut InstrProfValueData {
    (This as *mut ::core::ffi::c_char)
        .offset(getValueProfRecordHeaderSize((*This).NumValueSites) as isize)
        as *mut InstrProfValueData
}

pub unsafe fn getValueProfRecordNumValueData(This: *mut ValueProfRecord) -> uint32_t {
    let mut NumValueData: uint32_t = 0 as uint32_t;
    let mut I: uint32_t;
    I = 0 as uint32_t;
    while I < (*This).NumValueSites {
        NumValueData = NumValueData.wrapping_add(
            *(&raw mut (*This).SiteCountArray as *mut uint8_t).offset(I as isize) as uint32_t,
        );
        I = I.wrapping_add(1);
    }
    NumValueData
}

pub unsafe fn getValueProfRecordNext(This: *mut ValueProfRecord) -> *mut ValueProfRecord {
    let NumValueData = getValueProfRecordNumValueData(This);
    (This as *mut ::core::ffi::c_char)
        .offset(getValueProfRecordSize((*This).NumValueSites, NumValueData) as isize)
        as *mut ValueProfRecord
}

pub unsafe extern "C" fn getFirstValueProfRecord(This: *mut ValueProfData) -> *mut ValueProfRecord {
    (This as *mut ::core::ffi::c_char).add(::core::mem::size_of::<ValueProfData>() as usize)
        as *mut ValueProfRecord
}

pub unsafe fn getValueProfDataSize(Closure: *mut ValueProfRecordClosure) -> uint32_t {
    let mut Kind: uint32_t;
    let mut TotalSize: uint32_t = ::core::mem::size_of::<ValueProfData>() as uint32_t;
    let Record = (*Closure).Record;
    Kind = IPVK_First as ::core::ffi::c_int as uint32_t;
    while Kind <= IPVK_Last as ::core::ffi::c_int as uint32_t {
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

pub const INSTR_PROF_RAW_VERSION: ::core::ffi::c_int = 11;

pub const VARIANT_MASKS_ALL: u64 = 0xffffffff00000000;
pub const VARIANT_MASK_BYTE_COVERAGE: ::core::ffi::c_ulonglong =
    (0x1 as ::core::ffi::c_ulonglong) << 60 as ::core::ffi::c_int;
pub const VARIANT_MASK_TEMPORAL_PROF: ::core::ffi::c_ulonglong =
    (0x1 as ::core::ffi::c_ulonglong) << 63 as ::core::ffi::c_int;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct InstrProfValueData {
    pub Value: uint64_t,
    pub Count: uint64_t,
}

pub unsafe fn InstProfClzll(X: ::core::ffi::c_ulonglong) -> ::core::ffi::c_int {
    X.leading_zeros() as i32
}

pub unsafe fn InstProfPopcountll(X: ::core::ffi::c_ulonglong) -> ::core::ffi::c_int {
    X.count_ones() as i32
}

pub unsafe fn InstrProfGetRangeRepValue(Value: uint64_t) -> uint64_t {
    if Value <= 8 as uint64_t {
        Value
    } else if Value >= 513 as uint64_t {
        513 as uint64_t
    } else if InstProfPopcountll(Value as ::core::ffi::c_ulonglong) == 1 as ::core::ffi::c_int {
        Value
    } else {
        ((1 as uint64_t)
            << (64 as ::core::ffi::c_int
                - InstProfClzll(Value as ::core::ffi::c_ulonglong)
                - 1 as ::core::ffi::c_int))
            .wrapping_add(1 as uint64_t)
    }
}
