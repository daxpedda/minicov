use super::InstrProfilingMerge::ValueProfData;
use super::InstrProfilingValue::ValueProfRecord;

extern "C" {
    fn lprofBufferWriter(
        This: *mut ProfDataWriter,
        IOVecs: *mut ProfDataIOVec,
        NumIOVecs: uint32_t,
    ) -> uint32_t;
    fn lprofWriteData(
        Writer: *mut ProfDataWriter,
        VPDataReader: *mut VPDataReaderType,
        SkipNameDataWrite: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn lprofWriteDataImpl(
        Writer: *mut ProfDataWriter,
        DataBegin: *const __llvm_profile_data,
        DataEnd: *const __llvm_profile_data,
        CountersBegin: *const ::core::ffi::c_char,
        CountersEnd: *const ::core::ffi::c_char,
        BitmapBegin: *const ::core::ffi::c_char,
        BitmapEnd: *const ::core::ffi::c_char,
        UniformCountersBegin: *const ::core::ffi::c_char,
        UniformCountersEnd: *const ::core::ffi::c_char,
        VPDataReader: *mut VPDataReaderType,
        NamesBegin: *const ::core::ffi::c_char,
        NamesEnd: *const ::core::ffi::c_char,
        VTableBegin: *const VTableProfData,
        VTableEnd: *const VTableProfData,
        VNamesBegin: *const ::core::ffi::c_char,
        VNamesEnd: *const ::core::ffi::c_char,
        SkipNameDataWrite: ::core::ffi::c_int,
        Version: uint64_t,
    ) -> ::core::ffi::c_int;
    fn __llvm_write_binary_ids(Writer: *mut ProfDataWriter) -> ::core::ffi::c_int;
    fn __llvm_profile_get_num_padding_bytes(SizeInBytes: uint64_t) -> uint8_t;
    fn __llvm_profile_begin_data() -> *const __llvm_profile_data;
    fn __llvm_profile_end_data() -> *const __llvm_profile_data;
    fn __llvm_profile_begin_names() -> *const ::core::ffi::c_char;
    fn __llvm_profile_end_names() -> *const ::core::ffi::c_char;
    fn __llvm_profile_begin_vtabnames() -> *const ::core::ffi::c_char;
    fn __llvm_profile_end_vtabnames() -> *const ::core::ffi::c_char;
    fn __llvm_profile_begin_counters() -> *mut ::core::ffi::c_char;
    fn __llvm_profile_end_counters() -> *mut ::core::ffi::c_char;
    fn __llvm_profile_begin_bitmap() -> *mut ::core::ffi::c_char;
    fn __llvm_profile_end_bitmap() -> *mut ::core::ffi::c_char;
    fn __llvm_profile_begin_vtables() -> *const VTableProfData;
    fn __llvm_profile_end_vtables() -> *const VTableProfData;
    fn __llvm_profile_get_version() -> uint64_t;
}
pub type size_t = usize;
pub type uint64_t = u64;
pub type uint32_t = u32;
pub type uint16_t = u16;
pub type uint8_t = u8;
pub type intptr_t = isize;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ValueProfNode {
    pub Value: uint64_t,
    pub Count: uint64_t,
    pub Next: PtrToNodeT,
}
pub type PtrToNodeT = *mut ValueProfNode;
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
#[allow(dead_code, non_upper_case_globals)]
const VTableProfData_PADDING: usize =
    ::core::mem::size_of::<VTableProfData>() - ::core::mem::size_of::<VTableProfData_Inner>();
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
pub struct ProfDataIOVec {
    pub Data: *const ::core::ffi::c_void,
    pub ElmSize: size_t,
    pub NumElm: size_t,
    pub UseZeroPadding: ::core::ffi::c_int,
}
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
pub const VARIANT_MASK_BYTE_COVERAGE: ::core::ffi::c_ulonglong =
    (0x1 as ::core::ffi::c_ulonglong) << 60 as ::core::ffi::c_int;
static mut ContinuouslySyncProfile: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut PageSize: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
#[no_mangle]
pub unsafe extern "C" fn __llvm_profile_is_continuous_mode_enabled() -> ::core::ffi::c_int {
    return (ContinuouslySyncProfile != 0 && PageSize != 0) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn __llvm_profile_enable_continuous_mode() {
    ContinuouslySyncProfile = 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn __llvm_profile_disable_continuous_mode() {
    ContinuouslySyncProfile = 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn __llvm_profile_set_page_size(mut PS: ::core::ffi::c_uint) {
    PageSize = PS;
}
#[no_mangle]
pub unsafe extern "C" fn __llvm_profile_get_size_for_buffer() -> uint64_t {
    let mut DataBegin = __llvm_profile_begin_data();
    let mut DataEnd = __llvm_profile_end_data();
    let mut CountersBegin: *const ::core::ffi::c_char = __llvm_profile_begin_counters();
    let mut CountersEnd: *const ::core::ffi::c_char = __llvm_profile_end_counters();
    let mut BitmapBegin: *const ::core::ffi::c_char = __llvm_profile_begin_bitmap();
    let mut BitmapEnd: *const ::core::ffi::c_char = __llvm_profile_end_bitmap();
    let mut NamesBegin = __llvm_profile_begin_names();
    let mut NamesEnd = __llvm_profile_end_names();
    let mut VTableBegin = __llvm_profile_begin_vtables();
    let mut VTableEnd = __llvm_profile_end_vtables();
    let mut VNamesBegin = __llvm_profile_begin_vtabnames();
    let mut VNamesEnd = __llvm_profile_end_vtabnames();
    return __llvm_profile_get_size_for_buffer_internal(
        DataBegin,
        DataEnd,
        CountersBegin,
        CountersEnd,
        BitmapBegin,
        BitmapEnd,
        NamesBegin,
        NamesEnd,
        VTableBegin,
        VTableEnd,
        VNamesBegin,
        VNamesEnd,
    );
}
#[no_mangle]
pub unsafe extern "C" fn __llvm_profile_get_num_data(
    mut Begin: *const __llvm_profile_data,
    mut End: *const __llvm_profile_data,
) -> uint64_t {
    let mut BeginI: intptr_t = Begin as intptr_t;
    let mut EndI: intptr_t = End as intptr_t;
    return (EndI as usize)
        .wrapping_add(::core::mem::size_of::<__llvm_profile_data>() as usize)
        .wrapping_sub(1 as usize)
        .wrapping_sub(BeginI as usize)
        .wrapping_div(::core::mem::size_of::<__llvm_profile_data>() as usize)
        as uint64_t;
}
#[no_mangle]
pub unsafe extern "C" fn __llvm_profile_get_data_size(
    mut Begin: *const __llvm_profile_data,
    mut End: *const __llvm_profile_data,
) -> uint64_t {
    return __llvm_profile_get_num_data(Begin, End)
        .wrapping_mul(::core::mem::size_of::<__llvm_profile_data>() as uint64_t);
}
#[no_mangle]
pub unsafe extern "C" fn __llvm_profile_get_num_vtable(
    mut Begin: *const VTableProfData,
    mut End: *const VTableProfData,
) -> uint64_t {
    let mut EndI: intptr_t = End as intptr_t;
    let mut BeginI: intptr_t = Begin as intptr_t;
    return ((EndI - BeginI) as usize)
        .wrapping_div(::core::mem::size_of::<VTableProfData>() as usize) as uint64_t;
}
#[no_mangle]
pub unsafe extern "C" fn __llvm_profile_get_vtable_section_size(
    mut Begin: *const VTableProfData,
    mut End: *const VTableProfData,
) -> uint64_t {
    return (End as intptr_t - Begin as intptr_t) as uint64_t;
}
#[no_mangle]
pub unsafe extern "C" fn __llvm_profile_counter_entry_size() -> size_t {
    if __llvm_profile_get_version() & VARIANT_MASK_BYTE_COVERAGE as uint64_t != 0 {
        return ::core::mem::size_of::<uint8_t>() as size_t;
    }
    return ::core::mem::size_of::<uint64_t>() as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn __llvm_profile_get_num_counters(
    mut Begin: *const ::core::ffi::c_char,
    mut End: *const ::core::ffi::c_char,
) -> uint64_t {
    let mut BeginI: intptr_t = Begin as intptr_t;
    let mut EndI: intptr_t = End as intptr_t;
    return (EndI as size_t)
        .wrapping_add(__llvm_profile_counter_entry_size())
        .wrapping_sub(1 as size_t)
        .wrapping_sub(BeginI as size_t)
        .wrapping_div(__llvm_profile_counter_entry_size()) as uint64_t;
}
#[no_mangle]
pub unsafe extern "C" fn __llvm_profile_get_counters_size(
    mut Begin: *const ::core::ffi::c_char,
    mut End: *const ::core::ffi::c_char,
) -> uint64_t {
    return __llvm_profile_get_num_counters(Begin, End)
        .wrapping_mul(__llvm_profile_counter_entry_size() as uint64_t);
}
#[no_mangle]
pub unsafe extern "C" fn __llvm_profile_get_num_bitmap_bytes(
    mut Begin: *const ::core::ffi::c_char,
    mut End: *const ::core::ffi::c_char,
) -> uint64_t {
    return End.offset_from(Begin) as ::core::ffi::c_long as uint64_t;
}
#[no_mangle]
pub unsafe extern "C" fn __llvm_profile_get_name_size(
    mut Begin: *const ::core::ffi::c_char,
    mut End: *const ::core::ffi::c_char,
) -> uint64_t {
    return End.offset_from(Begin) as ::core::ffi::c_long as uint64_t;
}
unsafe extern "C" fn calculateBytesNeededToPageAlign(mut Offset: uint64_t) -> uint64_t {
    let mut OffsetModPage: uint64_t = Offset.wrapping_rem(PageSize as uint64_t);
    if OffsetModPage > 0 as uint64_t {
        return (PageSize as uint64_t).wrapping_sub(OffsetModPage);
    }
    return 0 as uint64_t;
}
unsafe extern "C" fn needsCounterPadding() -> ::core::ffi::c_int {
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn __llvm_profile_get_padding_sizes_for_counters(
    mut DataSize: uint64_t,
    mut CountersSize: uint64_t,
    mut NumBitmapBytes: uint64_t,
    mut NumUniformCounters: uint64_t,
    mut NamesSize: uint64_t,
    mut VTableSize: uint64_t,
    mut VNameSize: uint64_t,
    mut PaddingBytesBeforeCounters: *mut uint64_t,
    mut PaddingBytesAfterCounters: *mut uint64_t,
    mut PaddingBytesAfterBitmapBytes: *mut uint64_t,
    mut PaddingBytesAfterUniformCounters: *mut uint64_t,
    mut PaddingBytesAfterNames: *mut uint64_t,
    mut PaddingBytesAfterVTable: *mut uint64_t,
    mut PaddingBytesAfterVName: *mut uint64_t,
) -> ::core::ffi::c_int {
    if needsCounterPadding() == 0 {
        *PaddingBytesBeforeCounters = 0 as uint64_t;
        *PaddingBytesAfterCounters = __llvm_profile_get_num_padding_bytes(CountersSize) as uint64_t;
        *PaddingBytesAfterBitmapBytes =
            __llvm_profile_get_num_padding_bytes(NumBitmapBytes) as uint64_t;
        if !PaddingBytesAfterUniformCounters.is_null() {
            *PaddingBytesAfterUniformCounters = __llvm_profile_get_num_padding_bytes(
                NumUniformCounters.wrapping_mul(::core::mem::size_of::<uint64_t>() as uint64_t),
            ) as uint64_t;
        }
        *PaddingBytesAfterNames = __llvm_profile_get_num_padding_bytes(NamesSize) as uint64_t;
        if !PaddingBytesAfterVTable.is_null() {
            *PaddingBytesAfterVTable = __llvm_profile_get_num_padding_bytes(VTableSize) as uint64_t;
        }
        if !PaddingBytesAfterVName.is_null() {
            *PaddingBytesAfterVName = __llvm_profile_get_num_padding_bytes(VNameSize) as uint64_t;
        }
        return 0 as ::core::ffi::c_int;
    }
    if VTableSize != 0 as uint64_t || VNameSize != 0 as uint64_t {
        return -(1 as ::core::ffi::c_int);
    }
    *PaddingBytesBeforeCounters = calculateBytesNeededToPageAlign(
        (::core::mem::size_of::<__llvm_profile_header>() as uint64_t).wrapping_add(DataSize),
    );
    *PaddingBytesAfterCounters = calculateBytesNeededToPageAlign(CountersSize);
    *PaddingBytesAfterBitmapBytes = calculateBytesNeededToPageAlign(NumBitmapBytes);
    if !PaddingBytesAfterUniformCounters.is_null() {
        *PaddingBytesAfterUniformCounters = 0 as uint64_t;
    }
    *PaddingBytesAfterNames = calculateBytesNeededToPageAlign(NamesSize);
    if !PaddingBytesAfterVTable.is_null() {
        *PaddingBytesAfterVTable = 0 as uint64_t;
    }
    if !PaddingBytesAfterVName.is_null() {
        *PaddingBytesAfterVName = 0 as uint64_t;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn __llvm_profile_get_size_for_buffer_internal(
    mut DataBegin: *const __llvm_profile_data,
    mut DataEnd: *const __llvm_profile_data,
    mut CountersBegin: *const ::core::ffi::c_char,
    mut CountersEnd: *const ::core::ffi::c_char,
    mut BitmapBegin: *const ::core::ffi::c_char,
    mut BitmapEnd: *const ::core::ffi::c_char,
    mut NamesBegin: *const ::core::ffi::c_char,
    mut NamesEnd: *const ::core::ffi::c_char,
    mut VTableBegin: *const VTableProfData,
    mut VTableEnd: *const VTableProfData,
    mut VNamesBegin: *const ::core::ffi::c_char,
    mut VNamesEnd: *const ::core::ffi::c_char,
) -> uint64_t {
    let NamesSize: uint64_t = (NamesEnd.offset_from(NamesBegin) as ::core::ffi::c_long as usize)
        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
        as uint64_t;
    let mut DataSize = __llvm_profile_get_data_size(DataBegin, DataEnd);
    let mut CountersSize = __llvm_profile_get_counters_size(CountersBegin, CountersEnd);
    let NumBitmapBytes = __llvm_profile_get_num_bitmap_bytes(BitmapBegin, BitmapEnd) as uint64_t;
    let VTableSize = __llvm_profile_get_vtable_section_size(VTableBegin, VTableEnd) as uint64_t;
    let VNameSize = __llvm_profile_get_name_size(VNamesBegin, VNamesEnd) as uint64_t;
    let mut PaddingBytesBeforeCounters: uint64_t = 0;
    let mut PaddingBytesAfterCounters: uint64_t = 0;
    let mut PaddingBytesAfterNames: uint64_t = 0;
    let mut PaddingBytesAfterBitmapBytes: uint64_t = 0;
    let mut PaddingBytesAfterUniformCounters: uint64_t = 0;
    let mut PaddingBytesAfterVTable: uint64_t = 0;
    let mut PaddingBytesAfterVNames: uint64_t = 0;
    __llvm_profile_get_padding_sizes_for_counters(
        DataSize,
        CountersSize,
        NumBitmapBytes,
        0 as uint64_t,
        NamesSize,
        0 as uint64_t,
        0 as uint64_t,
        &raw mut PaddingBytesBeforeCounters,
        &raw mut PaddingBytesAfterCounters,
        &raw mut PaddingBytesAfterBitmapBytes,
        &raw mut PaddingBytesAfterUniformCounters,
        &raw mut PaddingBytesAfterNames,
        &raw mut PaddingBytesAfterVTable,
        &raw mut PaddingBytesAfterVNames,
    );
    return ((::core::mem::size_of::<__llvm_profile_header>() as usize)
        .wrapping_add(__llvm_write_binary_ids(::core::ptr::null_mut::<ProfDataWriter>()) as usize)
        as uint64_t)
        .wrapping_add(DataSize)
        .wrapping_add(PaddingBytesBeforeCounters)
        .wrapping_add(CountersSize)
        .wrapping_add(PaddingBytesAfterCounters)
        .wrapping_add(NumBitmapBytes)
        .wrapping_add(PaddingBytesAfterBitmapBytes)
        .wrapping_add(PaddingBytesAfterUniformCounters)
        .wrapping_add(NamesSize)
        .wrapping_add(PaddingBytesAfterNames)
        .wrapping_add(VTableSize)
        .wrapping_add(PaddingBytesAfterVTable)
        .wrapping_add(VNameSize)
        .wrapping_add(PaddingBytesAfterVNames);
}
#[no_mangle]
pub unsafe extern "C" fn initBufferWriter(
    mut BufferWriter: *mut ProfDataWriter,
    mut Buffer: *mut ::core::ffi::c_char,
) {
    (*BufferWriter).Write = Some(
        lprofBufferWriter
            as unsafe extern "C" fn(*mut ProfDataWriter, *mut ProfDataIOVec, uint32_t) -> uint32_t,
    ) as WriterCallback;
    (*BufferWriter).WriterCtx = Buffer as *mut ::core::ffi::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn __llvm_profile_write_buffer(
    mut Buffer: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut BufferWriter = ProfDataWriter {
        Write: None,
        WriterCtx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    initBufferWriter(&raw mut BufferWriter, Buffer);
    return lprofWriteData(
        &raw mut BufferWriter,
        ::core::ptr::null_mut::<VPDataReaderType>(),
        0 as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn __llvm_profile_write_buffer_internal(
    mut Buffer: *mut ::core::ffi::c_char,
    mut DataBegin: *const __llvm_profile_data,
    mut DataEnd: *const __llvm_profile_data,
    mut CountersBegin: *const ::core::ffi::c_char,
    mut CountersEnd: *const ::core::ffi::c_char,
    mut BitmapBegin: *const ::core::ffi::c_char,
    mut BitmapEnd: *const ::core::ffi::c_char,
    mut NamesBegin: *const ::core::ffi::c_char,
    mut NamesEnd: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut BufferWriter = ProfDataWriter {
        Write: None,
        WriterCtx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    initBufferWriter(&raw mut BufferWriter, Buffer);
    return lprofWriteDataImpl(
        &raw mut BufferWriter,
        DataBegin,
        DataEnd,
        CountersBegin,
        CountersEnd,
        BitmapBegin,
        BitmapEnd,
        ::core::ptr::null::<::core::ffi::c_char>(),
        ::core::ptr::null::<::core::ffi::c_char>(),
        ::core::ptr::null_mut::<VPDataReaderType>(),
        NamesBegin,
        NamesEnd,
        ::core::ptr::null::<VTableProfData>(),
        ::core::ptr::null::<VTableProfData>(),
        ::core::ptr::null::<::core::ffi::c_char>(),
        ::core::ptr::null::<::core::ffi::c_char>(),
        0 as ::core::ffi::c_int,
        __llvm_profile_get_version(),
    );
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
