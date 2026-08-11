use super::InstrProfData::{
    __llvm_profile_data, __llvm_profile_header, VTableProfData, VARIANT_MASK_BYTE_COVERAGE,
};
use super::InstrProfiling::{__llvm_profile_get_num_padding_bytes, __llvm_profile_get_version};
use super::InstrProfilingInternal::{
    ProfDataIOVec, ProfDataWriter, VPDataReaderType, WriterCallback,
};
use super::InstrProfilingPlatformLinux::{
    __llvm_profile_begin_bitmap, __llvm_profile_begin_counters, __llvm_profile_begin_data,
    __llvm_profile_begin_names, __llvm_profile_begin_vtables, __llvm_profile_begin_vtabnames,
    __llvm_profile_end_bitmap, __llvm_profile_end_counters, __llvm_profile_end_data,
    __llvm_profile_end_names, __llvm_profile_end_vtables, __llvm_profile_end_vtabnames,
    __llvm_write_binary_ids,
};
use super::InstrProfilingWriter::{lprofBufferWriter, lprofWriteData, lprofWriteDataImpl};

pub type size_t = usize;
pub type uint64_t = u64;
pub type uint32_t = u32;
pub type uint8_t = u8;
pub type intptr_t = isize;

static mut ContinuouslySyncProfile: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut PageSize: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;

#[no_mangle]
pub unsafe extern "C" fn __llvm_profile_is_continuous_mode_enabled() -> ::core::ffi::c_int {
    (ContinuouslySyncProfile != 0 && PageSize != 0) as ::core::ffi::c_int
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
pub unsafe extern "C" fn __llvm_profile_set_page_size(PS: ::core::ffi::c_uint) {
    PageSize = PS;
}

#[no_mangle]
pub unsafe extern "C" fn __llvm_profile_get_size_for_buffer() -> uint64_t {
    let DataBegin = __llvm_profile_begin_data();
    let DataEnd = __llvm_profile_end_data();
    let CountersBegin: *const ::core::ffi::c_char = __llvm_profile_begin_counters();
    let CountersEnd: *const ::core::ffi::c_char = __llvm_profile_end_counters();
    let BitmapBegin: *const ::core::ffi::c_char = __llvm_profile_begin_bitmap();
    let BitmapEnd: *const ::core::ffi::c_char = __llvm_profile_end_bitmap();
    let NamesBegin = __llvm_profile_begin_names();
    let NamesEnd = __llvm_profile_end_names();
    let VTableBegin = __llvm_profile_begin_vtables();
    let VTableEnd = __llvm_profile_end_vtables();
    let VNamesBegin = __llvm_profile_begin_vtabnames();
    let VNamesEnd = __llvm_profile_end_vtabnames();
    __llvm_profile_get_size_for_buffer_internal(
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
    )
}

#[no_mangle]
pub unsafe extern "C" fn __llvm_profile_get_num_data(
    Begin: *const __llvm_profile_data,
    End: *const __llvm_profile_data,
) -> uint64_t {
    let BeginI: intptr_t = Begin as intptr_t;
    let EndI: intptr_t = End as intptr_t;
    (EndI as usize)
        .wrapping_add(::core::mem::size_of::<__llvm_profile_data>() as usize)
        .wrapping_sub(1)
        .wrapping_sub(BeginI as usize)
        .wrapping_div(::core::mem::size_of::<__llvm_profile_data>() as usize) as uint64_t
}

#[no_mangle]
pub unsafe extern "C" fn __llvm_profile_get_data_size(
    Begin: *const __llvm_profile_data,
    End: *const __llvm_profile_data,
) -> uint64_t {
    __llvm_profile_get_num_data(Begin, End)
        .wrapping_mul(::core::mem::size_of::<__llvm_profile_data>() as uint64_t)
}

#[no_mangle]
pub unsafe extern "C" fn __llvm_profile_get_num_vtable(
    Begin: *const VTableProfData,
    End: *const VTableProfData,
) -> uint64_t {
    let EndI: intptr_t = End as intptr_t;
    let BeginI: intptr_t = Begin as intptr_t;
    ((EndI - BeginI) as usize).wrapping_div(::core::mem::size_of::<VTableProfData>() as usize)
        as uint64_t
}

#[no_mangle]
pub unsafe extern "C" fn __llvm_profile_get_vtable_section_size(
    Begin: *const VTableProfData,
    End: *const VTableProfData,
) -> uint64_t {
    (End as intptr_t - Begin as intptr_t) as uint64_t
}

#[no_mangle]
pub unsafe extern "C" fn __llvm_profile_counter_entry_size() -> size_t {
    if __llvm_profile_get_version() & VARIANT_MASK_BYTE_COVERAGE as uint64_t != 0 {
        return ::core::mem::size_of::<uint8_t>() as size_t;
    }
    ::core::mem::size_of::<uint64_t>() as size_t
}

#[no_mangle]
pub unsafe extern "C" fn __llvm_profile_get_num_counters(
    Begin: *const ::core::ffi::c_char,
    End: *const ::core::ffi::c_char,
) -> uint64_t {
    let BeginI: intptr_t = Begin as intptr_t;
    let EndI: intptr_t = End as intptr_t;
    (EndI as size_t)
        .wrapping_add(__llvm_profile_counter_entry_size())
        .wrapping_sub(1 as size_t)
        .wrapping_sub(BeginI as size_t)
        .wrapping_div(__llvm_profile_counter_entry_size()) as uint64_t
}

#[no_mangle]
pub unsafe extern "C" fn __llvm_profile_get_counters_size(
    Begin: *const ::core::ffi::c_char,
    End: *const ::core::ffi::c_char,
) -> uint64_t {
    __llvm_profile_get_num_counters(Begin, End)
        .wrapping_mul(__llvm_profile_counter_entry_size() as uint64_t)
}

#[no_mangle]
pub unsafe extern "C" fn __llvm_profile_get_num_bitmap_bytes(
    Begin: *const ::core::ffi::c_char,
    End: *const ::core::ffi::c_char,
) -> uint64_t {
    End.offset_from(Begin) as ::core::ffi::c_long as uint64_t
}

#[no_mangle]
pub unsafe extern "C" fn __llvm_profile_get_name_size(
    Begin: *const ::core::ffi::c_char,
    End: *const ::core::ffi::c_char,
) -> uint64_t {
    End.offset_from(Begin) as ::core::ffi::c_long as uint64_t
}

unsafe fn calculateBytesNeededToPageAlign(Offset: uint64_t) -> uint64_t {
    let OffsetModPage: uint64_t = Offset.wrapping_rem(PageSize as uint64_t);
    if OffsetModPage > 0 as uint64_t {
        return (PageSize as uint64_t).wrapping_sub(OffsetModPage);
    }
    0 as uint64_t
}

unsafe fn needsCounterPadding() -> ::core::ffi::c_int {
    0 as ::core::ffi::c_int
}

#[no_mangle]
pub unsafe extern "C" fn __llvm_profile_get_padding_sizes_for_counters(
    DataSize: uint64_t,
    CountersSize: uint64_t,
    NumBitmapBytes: uint64_t,
    NumUniformCounters: uint64_t,
    NamesSize: uint64_t,
    VTableSize: uint64_t,
    VNameSize: uint64_t,
    PaddingBytesBeforeCounters: *mut uint64_t,
    PaddingBytesAfterCounters: *mut uint64_t,
    PaddingBytesAfterBitmapBytes: *mut uint64_t,
    PaddingBytesAfterUniformCounters: *mut uint64_t,
    PaddingBytesAfterNames: *mut uint64_t,
    PaddingBytesAfterVTable: *mut uint64_t,
    PaddingBytesAfterVName: *mut uint64_t,
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
    0 as ::core::ffi::c_int
}

#[no_mangle]
pub unsafe extern "C" fn __llvm_profile_get_size_for_buffer_internal(
    DataBegin: *const __llvm_profile_data,
    DataEnd: *const __llvm_profile_data,
    CountersBegin: *const ::core::ffi::c_char,
    CountersEnd: *const ::core::ffi::c_char,
    BitmapBegin: *const ::core::ffi::c_char,
    BitmapEnd: *const ::core::ffi::c_char,
    NamesBegin: *const ::core::ffi::c_char,
    NamesEnd: *const ::core::ffi::c_char,
    VTableBegin: *const VTableProfData,
    VTableEnd: *const VTableProfData,
    VNamesBegin: *const ::core::ffi::c_char,
    VNamesEnd: *const ::core::ffi::c_char,
) -> uint64_t {
    let NamesSize: uint64_t = (NamesEnd.offset_from(NamesBegin) as ::core::ffi::c_long as usize)
        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
        as uint64_t;
    let DataSize = __llvm_profile_get_data_size(DataBegin, DataEnd);
    let CountersSize = __llvm_profile_get_counters_size(CountersBegin, CountersEnd);
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
    ((::core::mem::size_of::<__llvm_profile_header>() as usize)
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
        .wrapping_add(PaddingBytesAfterVNames)
}

#[no_mangle]
pub unsafe extern "C" fn initBufferWriter(
    BufferWriter: *mut ProfDataWriter,
    Buffer: *mut ::core::ffi::c_char,
) {
    (*BufferWriter).Write = Some(
        lprofBufferWriter
            as unsafe extern "C" fn(*mut ProfDataWriter, *mut ProfDataIOVec, uint32_t) -> uint32_t,
    ) as WriterCallback;
    (*BufferWriter).WriterCtx = Buffer as *mut ::core::ffi::c_void;
}

#[no_mangle]
pub unsafe extern "C" fn __llvm_profile_write_buffer(
    Buffer: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut BufferWriter = ProfDataWriter {
        Write: None,
        WriterCtx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    initBufferWriter(&raw mut BufferWriter, Buffer);
    lprofWriteData(
        &raw mut BufferWriter,
        ::core::ptr::null_mut::<VPDataReaderType>(),
        0 as ::core::ffi::c_int,
    )
}

#[no_mangle]
pub unsafe extern "C" fn __llvm_profile_write_buffer_internal(
    Buffer: *mut ::core::ffi::c_char,
    DataBegin: *const __llvm_profile_data,
    DataEnd: *const __llvm_profile_data,
    CountersBegin: *const ::core::ffi::c_char,
    CountersEnd: *const ::core::ffi::c_char,
    BitmapBegin: *const ::core::ffi::c_char,
    BitmapEnd: *const ::core::ffi::c_char,
    NamesBegin: *const ::core::ffi::c_char,
    NamesEnd: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut BufferWriter = ProfDataWriter {
        Write: None,
        WriterCtx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    initBufferWriter(&raw mut BufferWriter, Buffer);
    lprofWriteDataImpl(
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
    )
}
