use core::ffi::{c_char, c_int, c_uint, c_void};
use core::{mem, ptr};

use super::InstrProfData::{
    __llvm_profile_data, __llvm_profile_header, VTableProfData, VARIANT_MASK_BYTE_COVERAGE,
};
use super::InstrProfiling::{__llvm_profile_get_num_padding_bytes, __llvm_profile_get_version};
use super::InstrProfilingInternal::ProfDataWriter;
use super::InstrProfilingPlatformLinux::{
    __llvm_profile_begin_bitmap, __llvm_profile_begin_counters, __llvm_profile_begin_data,
    __llvm_profile_begin_names, __llvm_profile_begin_vtables, __llvm_profile_begin_vtabnames,
    __llvm_profile_end_bitmap, __llvm_profile_end_counters, __llvm_profile_end_data,
    __llvm_profile_end_names, __llvm_profile_end_vtables, __llvm_profile_end_vtabnames,
    __llvm_write_binary_ids,
};
use super::InstrProfilingWriter::{lprofBufferWriter, lprofWriteData, lprofWriteDataImpl};

static mut ContinuouslySyncProfile: c_int = 0;
static mut PageSize: c_uint = 0;

pub unsafe fn __llvm_profile_is_continuous_mode_enabled() -> c_int {
    (ContinuouslySyncProfile != 0 && PageSize != 0) as c_int
}

pub unsafe fn __llvm_profile_enable_continuous_mode() {
    ContinuouslySyncProfile = 1;
}

pub unsafe fn __llvm_profile_disable_continuous_mode() {
    ContinuouslySyncProfile = 0;
}

pub unsafe fn __llvm_profile_set_page_size(PS: c_uint) {
    PageSize = PS;
}

pub unsafe fn __llvm_profile_get_size_for_buffer() -> u64 {
    let DataBegin = __llvm_profile_begin_data();
    let DataEnd = __llvm_profile_end_data();
    let CountersBegin = __llvm_profile_begin_counters();
    let CountersEnd = __llvm_profile_end_counters();
    let BitmapBegin = __llvm_profile_begin_bitmap();
    let BitmapEnd = __llvm_profile_end_bitmap();
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

pub unsafe fn __llvm_profile_get_num_data(
    Begin: *const __llvm_profile_data,
    End: *const __llvm_profile_data,
) -> u64 {
    let BeginI = Begin as isize;
    let EndI = End as isize;
    (EndI as usize)
        .wrapping_add(mem::size_of::<__llvm_profile_data>())
        .wrapping_sub(1)
        .wrapping_sub(BeginI as usize)
        .wrapping_div(mem::size_of::<__llvm_profile_data>()) as u64
}

pub unsafe fn __llvm_profile_get_data_size(
    Begin: *const __llvm_profile_data,
    End: *const __llvm_profile_data,
) -> u64 {
    __llvm_profile_get_num_data(Begin, End)
        .wrapping_mul(mem::size_of::<__llvm_profile_data>() as u64)
}

pub unsafe fn __llvm_profile_get_num_vtable(
    Begin: *const VTableProfData,
    End: *const VTableProfData,
) -> u64 {
    let EndI: isize = End as isize;
    let BeginI: isize = Begin as isize;
    ((EndI - BeginI) as usize).wrapping_div(mem::size_of::<VTableProfData>()) as u64
}

pub unsafe fn __llvm_profile_get_vtable_section_size(
    Begin: *const VTableProfData,
    End: *const VTableProfData,
) -> u64 {
    (End as isize - Begin as isize) as u64
}

pub unsafe fn __llvm_profile_counter_entry_size() -> usize {
    #[expect(clippy::unnecessary_cast)]
    if __llvm_profile_get_version() & VARIANT_MASK_BYTE_COVERAGE as u64 != 0 {
        return mem::size_of::<u8>();
    }
    mem::size_of::<u64>()
}

pub unsafe fn __llvm_profile_get_num_counters(Begin: *const c_char, End: *const c_char) -> u64 {
    let BeginI: isize = Begin as isize;
    let EndI: isize = End as isize;
    (EndI as usize)
        .wrapping_add(__llvm_profile_counter_entry_size())
        .wrapping_sub(1)
        .wrapping_sub(BeginI as usize)
        .wrapping_div(__llvm_profile_counter_entry_size()) as u64
}

pub unsafe fn __llvm_profile_get_counters_size(Begin: *const c_char, End: *const c_char) -> u64 {
    __llvm_profile_get_num_counters(Begin, End)
        .wrapping_mul(__llvm_profile_counter_entry_size() as u64)
}

pub unsafe fn __llvm_profile_get_num_bitmap_bytes(Begin: *const c_char, End: *const c_char) -> u64 {
    End.offset_from(Begin) as u64
}

pub unsafe fn __llvm_profile_get_name_size(Begin: *const c_char, End: *const c_char) -> u64 {
    End.offset_from(Begin) as u64
}

unsafe fn calculateBytesNeededToPageAlign(Offset: u64) -> u64 {
    let OffsetModPage = Offset.wrapping_rem(PageSize as u64);
    if OffsetModPage > 0 {
        return (PageSize as u64).wrapping_sub(OffsetModPage);
    }
    0
}

unsafe fn needsCounterPadding() -> c_int {
    0
}

#[expect(clippy::too_many_arguments)]
pub unsafe fn __llvm_profile_get_padding_sizes_for_counters(
    DataSize: u64,
    CountersSize: u64,
    NumBitmapBytes: u64,
    NumUniformCounters: u64,
    NamesSize: u64,
    VTableSize: u64,
    VNameSize: u64,
    PaddingBytesBeforeCounters: *mut u64,
    PaddingBytesAfterCounters: *mut u64,
    PaddingBytesAfterBitmapBytes: *mut u64,
    PaddingBytesAfterUniformCounters: *mut u64,
    PaddingBytesAfterNames: *mut u64,
    PaddingBytesAfterVTable: *mut u64,
    PaddingBytesAfterVName: *mut u64,
) -> c_int {
    if needsCounterPadding() == 0 {
        *PaddingBytesBeforeCounters = 0;
        *PaddingBytesAfterCounters = __llvm_profile_get_num_padding_bytes(CountersSize) as u64;
        *PaddingBytesAfterBitmapBytes = __llvm_profile_get_num_padding_bytes(NumBitmapBytes) as u64;
        if !PaddingBytesAfterUniformCounters.is_null() {
            *PaddingBytesAfterUniformCounters = __llvm_profile_get_num_padding_bytes(
                NumUniformCounters.wrapping_mul(mem::size_of::<u64>() as u64),
            ) as u64;
        }
        *PaddingBytesAfterNames = __llvm_profile_get_num_padding_bytes(NamesSize) as u64;
        if !PaddingBytesAfterVTable.is_null() {
            *PaddingBytesAfterVTable = __llvm_profile_get_num_padding_bytes(VTableSize) as u64;
        }
        if !PaddingBytesAfterVName.is_null() {
            *PaddingBytesAfterVName = __llvm_profile_get_num_padding_bytes(VNameSize) as u64;
        }
        return 0;
    }
    if VTableSize != 0 || VNameSize != 0 {
        return -1;
    }
    *PaddingBytesBeforeCounters = calculateBytesNeededToPageAlign(
        (mem::size_of::<__llvm_profile_header>() as u64).wrapping_add(DataSize),
    );
    *PaddingBytesAfterCounters = calculateBytesNeededToPageAlign(CountersSize);
    *PaddingBytesAfterBitmapBytes = calculateBytesNeededToPageAlign(NumBitmapBytes);
    if !PaddingBytesAfterUniformCounters.is_null() {
        *PaddingBytesAfterUniformCounters = 0;
    }
    *PaddingBytesAfterNames = calculateBytesNeededToPageAlign(NamesSize);
    if !PaddingBytesAfterVTable.is_null() {
        *PaddingBytesAfterVTable = 0;
    }
    if !PaddingBytesAfterVName.is_null() {
        *PaddingBytesAfterVName = 0;
    }
    0
}

#[expect(clippy::too_many_arguments)]
pub unsafe fn __llvm_profile_get_size_for_buffer_internal(
    DataBegin: *const __llvm_profile_data,
    DataEnd: *const __llvm_profile_data,
    CountersBegin: *const c_char,
    CountersEnd: *const c_char,
    BitmapBegin: *const c_char,
    BitmapEnd: *const c_char,
    NamesBegin: *const c_char,
    NamesEnd: *const c_char,
    VTableBegin: *const VTableProfData,
    VTableEnd: *const VTableProfData,
    VNamesBegin: *const c_char,
    VNamesEnd: *const c_char,
) -> u64 {
    let NamesSize: u64 = (NamesEnd.offset_from(NamesBegin) as usize)
        .wrapping_mul(mem::size_of::<c_char>() as usize) as u64;
    let DataSize = __llvm_profile_get_data_size(DataBegin, DataEnd);
    let CountersSize = __llvm_profile_get_counters_size(CountersBegin, CountersEnd);
    let NumBitmapBytes = __llvm_profile_get_num_bitmap_bytes(BitmapBegin, BitmapEnd);
    let VTableSize = __llvm_profile_get_vtable_section_size(VTableBegin, VTableEnd);
    let VNameSize = __llvm_profile_get_name_size(VNamesBegin, VNamesEnd);
    let mut PaddingBytesBeforeCounters: u64 = 0;
    let mut PaddingBytesAfterCounters: u64 = 0;
    let mut PaddingBytesAfterNames: u64 = 0;
    let mut PaddingBytesAfterBitmapBytes: u64 = 0;
    let mut PaddingBytesAfterUniformCounters: u64 = 0;
    let mut PaddingBytesAfterVTable: u64 = 0;
    let mut PaddingBytesAfterVNames: u64 = 0;
    __llvm_profile_get_padding_sizes_for_counters(
        DataSize,
        CountersSize,
        NumBitmapBytes,
        0,
        NamesSize,
        0,
        0,
        &raw mut PaddingBytesBeforeCounters,
        &raw mut PaddingBytesAfterCounters,
        &raw mut PaddingBytesAfterBitmapBytes,
        &raw mut PaddingBytesAfterUniformCounters,
        &raw mut PaddingBytesAfterNames,
        &raw mut PaddingBytesAfterVTable,
        &raw mut PaddingBytesAfterVNames,
    );
    ((mem::size_of::<__llvm_profile_header>())
        .wrapping_add(__llvm_write_binary_ids(ptr::null_mut::<ProfDataWriter>()) as usize)
        as u64)
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

pub unsafe fn initBufferWriter(BufferWriter: *mut ProfDataWriter, Buffer: *mut c_char) {
    (*BufferWriter).Write = Some(lprofBufferWriter);
    (*BufferWriter).WriterCtx = Buffer as *mut c_void;
}

pub unsafe fn __llvm_profile_write_buffer(Buffer: *mut c_char) -> c_int {
    let mut BufferWriter = ProfDataWriter {
        Write: None,
        WriterCtx: ptr::null_mut(),
    };
    initBufferWriter(&raw mut BufferWriter, Buffer);
    lprofWriteData(&raw mut BufferWriter, ptr::null_mut(), 0)
}

#[expect(clippy::too_many_arguments)]
pub unsafe fn __llvm_profile_write_buffer_internal(
    Buffer: *mut c_char,
    DataBegin: *const __llvm_profile_data,
    DataEnd: *const __llvm_profile_data,
    CountersBegin: *const c_char,
    CountersEnd: *const c_char,
    BitmapBegin: *const c_char,
    BitmapEnd: *const c_char,
    NamesBegin: *const c_char,
    NamesEnd: *const c_char,
) -> c_int {
    let mut BufferWriter = ProfDataWriter {
        Write: None,
        WriterCtx: ptr::null_mut(),
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
        ptr::null(),
        ptr::null(),
        ptr::null_mut(),
        NamesBegin,
        NamesEnd,
        ptr::null(),
        ptr::null(),
        ptr::null(),
        ptr::null(),
        0,
        __llvm_profile_get_version(),
    )
}
