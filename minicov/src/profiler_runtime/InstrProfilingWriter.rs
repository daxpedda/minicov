use core::ffi::{c_char, c_int, c_uint, c_void};
use core::{mem, ptr};

use alloc::vec::{self, Vec};

use super::InstrProfData::{
    __llvm_profile_data, __llvm_profile_header, IPVK_Last, InstrProfValueData, VTableProfData,
    ValueProfData, ValueProfNode, ValueProfRecord,
};
use super::InstrProfiling::{__llvm_profile_get_magic, __llvm_profile_get_version};
use super::InstrProfilingBuffer::{
    __llvm_profile_get_counters_size, __llvm_profile_get_data_size, __llvm_profile_get_name_size,
    __llvm_profile_get_num_bitmap_bytes, __llvm_profile_get_num_counters,
    __llvm_profile_get_num_data, __llvm_profile_get_num_vtable,
    __llvm_profile_get_padding_sizes_for_counters, __llvm_profile_get_vtable_section_size,
    __llvm_profile_is_continuous_mode_enabled, initBufferWriter,
};
use super::InstrProfilingInternal::{ProfDataIOVec, ProfDataWriter, VPDataReaderType};
use super::InstrProfilingPlatformLinux::{
    __llvm_profile_begin_bitmap, __llvm_profile_begin_counters, __llvm_profile_begin_data,
    __llvm_profile_begin_names, __llvm_profile_begin_vtables, __llvm_profile_begin_vtabnames,
    __llvm_profile_end_bitmap, __llvm_profile_end_counters, __llvm_profile_end_data,
    __llvm_profile_end_names, __llvm_profile_end_vtables, __llvm_profile_end_vtabnames,
    __llvm_write_binary_ids,
};

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ProfBufferIO {
    pub FileWriter: *mut ProfDataWriter,
    pub OwnFileWriter: u32,
    pub BufferStart: *mut u8,
    pub BufferSz: u32,
    pub CurOffset: u32,
}

pub static mut FreeHook: Option<unsafe extern "C" fn(*mut c_void) -> ()> = None;
static mut TheBufferIO: ProfBufferIO = ProfBufferIO {
    FileWriter: ptr::null_mut(),
    OwnFileWriter: 0,
    BufferStart: ptr::null_mut(),
    BufferSz: 0,
    CurOffset: 0,
};
static mut BufferIOBuffer: [u8; 8192] = [0; 8192];
static mut VPDataArray: [InstrProfValueData; 16] = [InstrProfValueData { Value: 0, Count: 0 }; 16];
static mut VPDataArraySize: u32 = (mem::size_of::<[InstrProfValueData; 16]>())
    .wrapping_div(mem::size_of::<InstrProfValueData>()) as u32;

pub static mut DynamicBufferIOBuffer: *mut u8 = ptr::null_mut();
pub static mut VPBufferSize: u32 = 0;

pub unsafe extern "C" fn lprofBufferWriter(
    This: *mut ProfDataWriter,
    IOVecs: *mut ProfDataIOVec,
    NumIOVecs: u32,
) -> u32 {
    let mut I: u32 = 0;
    let Buffer = &raw mut (*This).WriterCtx as *mut *mut c_char;
    while I < NumIOVecs {
        let Length: usize = (*IOVecs.offset(I as isize))
            .ElmSize
            .wrapping_mul((*IOVecs.offset(I as isize)).NumElm);
        if !(*IOVecs.offset(I as isize)).Data.is_null() {
            ptr::copy_nonoverlapping(
                (*IOVecs.offset(I as isize)).Data,
                *Buffer as *mut c_void,
                Length,
            );
        }
        *Buffer = (*Buffer).add(Length);
        I = I.wrapping_add(1);
    }
    0
}

unsafe fn llvmInitBufferIO(
    BufferIO: *mut ProfBufferIO,
    FileWriter: *mut ProfDataWriter,
    Buffer: *mut u8,
    BufferSz: u32,
) {
    (*BufferIO).FileWriter = FileWriter;
    (*BufferIO).OwnFileWriter = 0;
    (*BufferIO).BufferStart = Buffer;
    (*BufferIO).BufferSz = BufferSz;
    (*BufferIO).CurOffset = 0;
}

pub unsafe fn lprofCreateBufferIO(FileWriter: *mut ProfDataWriter) -> *mut ProfBufferIO {
    let mut Buffer = DynamicBufferIOBuffer;
    let mut BufferSize: u32 = VPBufferSize;
    #[expect(static_mut_refs)]
    if Buffer.is_null() {
        Buffer = BufferIOBuffer.as_mut_ptr();
        BufferSize = mem::size_of::<[u8; 8192]>() as u32;
    }
    llvmInitBufferIO(&raw mut TheBufferIO, FileWriter, Buffer, BufferSize);
    &raw mut TheBufferIO
}

pub unsafe fn lprofDeleteBufferIO(BufferIO: *mut ProfBufferIO) {
    if (*BufferIO).OwnFileWriter != 0 {
        FreeHook.expect("non-null function pointer")((*BufferIO).FileWriter as *mut c_void);
    }
    if !DynamicBufferIOBuffer.is_null() {
        FreeHook.expect("non-null function pointer")(DynamicBufferIOBuffer as *mut c_void);
        DynamicBufferIOBuffer = ptr::null_mut();
        VPBufferSize = 0;
    }
}

pub unsafe fn lprofBufferIOWrite(BufferIO: *mut ProfBufferIO, Data: *const u8, Size: u32) -> c_int {
    if Size.wrapping_add((*BufferIO).CurOffset) > (*BufferIO).BufferSz
        && lprofBufferIOFlush(BufferIO) != 0
    {
        return -1;
    }
    let mut IO: [ProfDataIOVec; 1] = [ProfDataIOVec {
        Data: Data as *const c_void,
        ElmSize: mem::size_of::<u8>(),
        NumElm: Size as usize,
        UseZeroPadding: 0,
    }];
    if Size > (*BufferIO).BufferSz {
        if (*(*BufferIO).FileWriter)
            .Write
            .expect("non-null function pointer")(
            (*BufferIO).FileWriter,
            &raw mut IO as *mut ProfDataIOVec,
            1,
        ) != 0
        {
            return -1;
        }
    } else {
        let Buffer = (*BufferIO)
            .BufferStart
            .offset((*BufferIO).CurOffset as isize);
        let mut BufferWriter = ProfDataWriter {
            Write: None,
            WriterCtx: ptr::null_mut(),
        };
        initBufferWriter(&raw mut BufferWriter, Buffer as *mut c_char);
        lprofBufferWriter(&raw mut BufferWriter, &raw mut IO as *mut ProfDataIOVec, 1);
        (*BufferIO).CurOffset =
            (BufferWriter.WriterCtx as *mut u8).offset_from((*BufferIO).BufferStart) as u32;
    }
    0
}

pub unsafe fn lprofBufferIOFlush(BufferIO: *mut ProfBufferIO) -> c_int {
    if (*BufferIO).CurOffset != 0 {
        let mut IO: [ProfDataIOVec; 1] = [ProfDataIOVec {
            Data: (*BufferIO).BufferStart as *const c_void,
            ElmSize: mem::size_of::<u8>(),
            NumElm: (*BufferIO).CurOffset as usize,
            UseZeroPadding: 0,
        }];
        if (*(*BufferIO).FileWriter)
            .Write
            .expect("non-null function pointer")(
            (*BufferIO).FileWriter,
            &raw mut IO as *mut ProfDataIOVec,
            1,
        ) != 0
        {
            return -1;
        }
        (*BufferIO).CurOffset = 0;
    }
    0 as c_int
}

unsafe fn writeOneValueProfData(
    BufferIO: *mut ProfBufferIO,
    VPDataReader: *mut VPDataReaderType,
    Data: *const __llvm_profile_data,
) -> c_int {
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    let mut I: c_uint = 0;

    let mut VPHeader = ValueProfData {
        TotalSize: 0,
        NumValueKinds: 0,
    };
    let mut SiteCountArray: [*mut u8; 3] = [ptr::null_mut(); 3];
    while I <= IPVK_Last as c_int as c_uint {
        if (*Data).0.NumValueSites[I as usize] == 0 {
            SiteCountArray[I as usize] = ptr::null_mut();
        } else {
            let Sz: u32 = ((*VPDataReader)
                .GetValueProfRecordHeaderSize
                .expect("non-null function pointer")(
                (*Data).0.NumValueSites[I as usize] as u32
            ))
            .wrapping_sub(8) as u32;
            alloca_allocations.push(vec::from_elem(0, Sz as usize));
            SiteCountArray[I as usize] = alloca_allocations.last_mut().unwrap().as_mut_ptr();
            ptr::write_bytes(SiteCountArray[I as usize] as *mut c_void, 0, Sz as usize);
        }
        I = I.wrapping_add(1);
    }
    let NumValueKinds: c_uint = (*VPDataReader)
        .InitRTRecord
        .expect("non-null function pointer")(
        Data, SiteCountArray.as_mut_ptr()
    ) as c_uint;
    if NumValueKinds == 0 {
        return 0;
    }
    VPHeader.TotalSize = (*VPDataReader)
        .GetValueProfDataSize
        .expect("non-null function pointer")();
    VPHeader.NumValueKinds = NumValueKinds as u32;
    if lprofBufferIOWrite(
        BufferIO,
        &raw mut VPHeader as *const u8,
        mem::size_of::<ValueProfData>() as u32,
    ) != 0
    {
        return -1;
    }
    if (*VPDataReader)
        .GetFirstValueProfRecord
        .expect("non-null function pointer")(&raw mut VPHeader) as *mut c_void
        != (&raw mut VPHeader).offset(1) as *mut c_void
    {
        return -1;
    }
    I = 0;
    while I <= IPVK_Last as c_int as c_uint {
        let mut J: u32;
        let mut RecordHeader = ValueProfRecord {
            Kind: 0,
            NumValueSites: 0,
            SiteCountArray: [0; 1],
        };
        let RecordHeaderSize: u32 = 8;
        let SiteCountArraySize: u32;
        if !((*Data).0.NumValueSites[I as usize] == 0) {
            RecordHeader.Kind = I as u32;
            RecordHeader.NumValueSites = (*Data).0.NumValueSites[I as usize] as u32;
            if lprofBufferIOWrite(
                BufferIO,
                &raw mut RecordHeader as *const u8,
                RecordHeaderSize,
            ) != 0
            {
                return -1;
            }
            SiteCountArraySize = (*VPDataReader)
                .GetValueProfRecordHeaderSize
                .expect("non-null function pointer")(
                (*Data).0.NumValueSites[I as usize] as u32
            )
            .wrapping_sub(RecordHeaderSize);
            if lprofBufferIOWrite(BufferIO, SiteCountArray[I as usize], SiteCountArraySize) != 0 {
                return -1;
            }
            J = 0;
            while J < (*Data).0.NumValueSites[I as usize] as u32 {
                let mut NRead: u32;
                let mut NRemain: u32;
                let mut NextStartNode = ptr::null_mut::<ValueProfNode>();
                NRemain = (*VPDataReader)
                    .GetNumValueDataForSite
                    .expect("non-null function pointer")(I as u32, J);
                if !(NRemain == 0) {
                    loop {
                        NRead = if NRemain > VPDataArraySize {
                            VPDataArraySize
                        } else {
                            NRemain
                        };
                        NextStartNode = (*VPDataReader)
                            .GetValueData
                            .expect("non-null function pointer")(
                            I as u32,
                            J,
                            #[expect(static_mut_refs)]
                            VPDataArray.as_mut_ptr(),
                            NextStartNode,
                            NRead,
                        );
                        #[expect(static_mut_refs)]
                        if lprofBufferIOWrite(
                            BufferIO,
                            VPDataArray.as_mut_ptr() as *const u8,
                            (NRead as usize).wrapping_mul(mem::size_of::<InstrProfValueData>())
                                as u32,
                        ) != 0
                        {
                            return -1;
                        }
                        NRemain = NRemain.wrapping_sub(NRead);
                        if NRemain == 0 {
                            break;
                        }
                    }
                }
                J = J.wrapping_add(1);
            }
        }
        I = I.wrapping_add(1);
    }
    0
}

unsafe fn writeValueProfData(
    Writer: *mut ProfDataWriter,
    VPDataReader: *mut VPDataReaderType,
    DataBegin: *const __llvm_profile_data,
    DataEnd: *const __llvm_profile_data,
) -> c_int {
    let mut DI: *const __llvm_profile_data;
    if VPDataReader.is_null() {
        return 0;
    }
    let BufferIO: *mut ProfBufferIO = lprofCreateBufferIO(Writer);
    DI = DataBegin;
    while DI < DataEnd {
        if writeOneValueProfData(BufferIO, VPDataReader, DI) != 0 {
            return -1;
        }
        DI = DI.offset(1);
    }
    if lprofBufferIOFlush(BufferIO) != 0 {
        return -1;
    }
    lprofDeleteBufferIO(BufferIO);
    0
}

pub unsafe fn lprofWriteData(
    Writer: *mut ProfDataWriter,
    VPDataReader: *mut VPDataReaderType,
    SkipNameDataWrite: c_int,
) -> c_int {
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
    let Version = __llvm_profile_get_version();
    lprofWriteDataImpl(
        Writer,
        DataBegin,
        DataEnd,
        CountersBegin,
        CountersEnd,
        BitmapBegin,
        BitmapEnd,
        ptr::null(),
        ptr::null(),
        VPDataReader,
        NamesBegin,
        NamesEnd,
        VTableBegin,
        VTableEnd,
        VNamesBegin,
        VNamesEnd,
        SkipNameDataWrite,
        Version,
    )
}

#[expect(clippy::too_many_arguments)]
pub unsafe fn lprofWriteDataImpl(
    Writer: *mut ProfDataWriter,
    DataBegin: *const __llvm_profile_data,
    DataEnd: *const __llvm_profile_data,
    CountersBegin: *const c_char,
    CountersEnd: *const c_char,
    BitmapBegin: *const c_char,
    BitmapEnd: *const c_char,
    UniformCountersBegin: *const c_char,
    UniformCountersEnd: *const c_char,
    VPDataReader: *mut VPDataReaderType,
    NamesBegin: *const c_char,
    NamesEnd: *const c_char,
    VTableBegin: *const VTableProfData,
    VTableEnd: *const VTableProfData,
    VNamesBegin: *const c_char,
    VNamesEnd: *const c_char,
    SkipNameDataWrite: c_int,
    Version: u64,
) -> c_int {
    let DataSectionSize = __llvm_profile_get_data_size(DataBegin, DataEnd);
    let NumData = __llvm_profile_get_num_data(DataBegin, DataEnd);
    let CountersSectionSize = __llvm_profile_get_counters_size(CountersBegin, CountersEnd);
    let NumCounters = __llvm_profile_get_num_counters(CountersBegin, CountersEnd);
    let NumBitmapBytes = __llvm_profile_get_num_bitmap_bytes(BitmapBegin, BitmapEnd);
    let NamesSize = __llvm_profile_get_name_size(NamesBegin, NamesEnd);
    let NumVTables = __llvm_profile_get_num_vtable(VTableBegin, VTableEnd);
    let VTableSectionSize = __llvm_profile_get_vtable_section_size(VTableBegin, VTableEnd);
    let VNamesSize = __llvm_profile_get_name_size(VNamesBegin, VNamesEnd);
    let NumUniformCounters: u64 = if !UniformCountersBegin.is_null()
        && !UniformCountersEnd.is_null()
        && UniformCountersEnd > UniformCountersBegin
    {
        (UniformCountersEnd.offset_from(UniformCountersBegin) as usize)
            .wrapping_div(mem::size_of::<u64>()) as u64
    } else {
        0
    };
    let UniformCountersSectionSize: u64 =
        NumUniformCounters.wrapping_mul(mem::size_of::<u64>() as u64);
    let mut Header = __llvm_profile_header {
        Magic: 0,
        Version: 0,
        BinaryIdsSize: 0,
        NumData: 0,
        PaddingBytesBeforeCounters: 0,
        NumCounters: 0,
        PaddingBytesAfterCounters: 0,
        NumBitmapBytes: 0,
        PaddingBytesAfterBitmapBytes: 0,
        NumUniformCounters: 0,
        PaddingBytesAfterUniformCounters: 0,
        UniformCountersDelta: 0,
        NamesSize: 0,
        CountersDelta: 0,
        BitmapDelta: 0,
        NamesDelta: 0,
        NumVTables: 0,
        VNamesSize: 0,
        ValueKindLast: 0,
    };
    let mut PaddingBytesBeforeCounters: u64 = 0;
    let mut PaddingBytesAfterCounters: u64 = 0;
    let mut PaddingBytesAfterBitmapBytes: u64 = 0;
    let mut PaddingBytesAfterUniformCounters: u64 = 0;
    let mut PaddingBytesAfterNames: u64 = 0;
    let mut PaddingBytesAfterVTable: u64 = 0;
    let mut PaddingBytesAfterVNames: u64 = 0;
    if __llvm_profile_get_padding_sizes_for_counters(
        DataSectionSize,
        CountersSectionSize,
        NumBitmapBytes,
        NumUniformCounters,
        NamesSize,
        VTableSectionSize,
        VNamesSize,
        &raw mut PaddingBytesBeforeCounters,
        &raw mut PaddingBytesAfterCounters,
        &raw mut PaddingBytesAfterBitmapBytes,
        &raw mut PaddingBytesAfterUniformCounters,
        &raw mut PaddingBytesAfterNames,
        &raw mut PaddingBytesAfterVTable,
        &raw mut PaddingBytesAfterVNames,
    ) == -1
    {
        return -1;
    }
    Header.Magic = __llvm_profile_get_magic();
    Header.Version = __llvm_profile_get_version();
    Header.BinaryIdsSize = __llvm_write_binary_ids(ptr::null_mut()) as u64;
    Header.NumData = NumData;
    Header.PaddingBytesBeforeCounters = PaddingBytesBeforeCounters;
    Header.NumCounters = NumCounters;
    Header.PaddingBytesAfterCounters = PaddingBytesAfterCounters;
    Header.NumBitmapBytes = NumBitmapBytes;
    Header.PaddingBytesAfterBitmapBytes = PaddingBytesAfterBitmapBytes;
    Header.NumUniformCounters = NumUniformCounters;
    Header.PaddingBytesAfterUniformCounters = PaddingBytesAfterUniformCounters;
    Header.UniformCountersDelta = (if !UniformCountersBegin.is_null() {
        (UniformCountersBegin as usize).wrapping_sub(DataBegin as usize)
    } else {
        0
    }) as u64;
    Header.NamesSize = NamesSize;
    Header.CountersDelta = (CountersBegin as usize).wrapping_sub(DataBegin as usize) as u64;
    Header.BitmapDelta = (BitmapBegin as usize).wrapping_sub(DataBegin as usize) as u64;
    Header.NamesDelta = NamesBegin as usize as u64;
    Header.NumVTables = NumVTables;
    Header.VNamesSize = VNamesSize;
    Header.ValueKindLast = IPVK_Last as c_int as u64;
    Header.Version = Version;
    if NumUniformCounters > 0 {
        Header.UniformCountersDelta = DataSectionSize
            .wrapping_add(PaddingBytesBeforeCounters)
            .wrapping_add(CountersSectionSize)
            .wrapping_add(PaddingBytesAfterCounters)
            .wrapping_add(NumBitmapBytes)
            .wrapping_add(PaddingBytesAfterBitmapBytes);
    } else {
        Header.UniformCountersDelta = 0;
    }
    if NumData == 0 && NamesSize == 0 {
        Header.CountersDelta = 0;
        Header.BitmapDelta = 0;
        Header.NamesDelta = 0;
        Header.UniformCountersDelta = 0;
    }
    let mut IOVec: [ProfDataIOVec; 1] = [ProfDataIOVec {
        Data: &raw mut Header as *const c_void,
        ElmSize: mem::size_of::<__llvm_profile_header>(),
        NumElm: 1,
        UseZeroPadding: 0 as c_int,
    }];
    if (*Writer).Write.expect("non-null function pointer")(
        Writer,
        &raw mut IOVec as *mut ProfDataIOVec,
        (mem::size_of::<[ProfDataIOVec; 1]>()).wrapping_div(mem::size_of::<ProfDataIOVec>()) as u32,
    ) != 0
    {
        return -1;
    }
    if __llvm_write_binary_ids(Writer) == -1 {
        return -1;
    }
    let mut IOVecData: [ProfDataIOVec; 14] = [
        ProfDataIOVec {
            Data: DataBegin as *const c_void,
            ElmSize: mem::size_of::<u8>(),
            NumElm: DataSectionSize as usize,
            UseZeroPadding: 0,
        },
        ProfDataIOVec {
            Data: ptr::null(),
            ElmSize: mem::size_of::<u8>(),
            NumElm: PaddingBytesBeforeCounters as usize,
            UseZeroPadding: 1,
        },
        ProfDataIOVec {
            Data: CountersBegin as *const c_void,
            ElmSize: mem::size_of::<u8>(),
            NumElm: CountersSectionSize as usize,
            UseZeroPadding: 0,
        },
        ProfDataIOVec {
            Data: ptr::null(),
            ElmSize: mem::size_of::<u8>(),
            NumElm: PaddingBytesAfterCounters as usize,
            UseZeroPadding: 1,
        },
        ProfDataIOVec {
            Data: BitmapBegin as *const c_void,
            ElmSize: mem::size_of::<u8>(),
            NumElm: NumBitmapBytes as usize,
            UseZeroPadding: 0,
        },
        ProfDataIOVec {
            Data: ptr::null(),
            ElmSize: mem::size_of::<u8>(),
            NumElm: PaddingBytesAfterBitmapBytes as usize,
            UseZeroPadding: 1,
        },
        ProfDataIOVec {
            Data: UniformCountersBegin as *const c_void,
            ElmSize: mem::size_of::<u8>(),
            NumElm: UniformCountersSectionSize as usize,
            UseZeroPadding: 0,
        },
        ProfDataIOVec {
            Data: ptr::null(),
            ElmSize: mem::size_of::<u8>(),
            NumElm: PaddingBytesAfterUniformCounters as usize,
            UseZeroPadding: 1,
        },
        ProfDataIOVec {
            Data: (if SkipNameDataWrite != 0 {
                ptr::null()
            } else {
                NamesBegin
            }) as *const c_void,
            ElmSize: mem::size_of::<u8>(),
            NumElm: NamesSize as usize,
            UseZeroPadding: 0,
        },
        ProfDataIOVec {
            Data: ptr::null(),
            ElmSize: mem::size_of::<u8>(),
            NumElm: PaddingBytesAfterNames as usize,
            UseZeroPadding: 1,
        },
        ProfDataIOVec {
            Data: VTableBegin as *const c_void,
            ElmSize: mem::size_of::<u8>(),
            NumElm: VTableSectionSize as usize,
            UseZeroPadding: 0,
        },
        ProfDataIOVec {
            Data: ptr::null(),
            ElmSize: mem::size_of::<u8>(),
            NumElm: PaddingBytesAfterVTable as usize,
            UseZeroPadding: 1,
        },
        ProfDataIOVec {
            Data: (if SkipNameDataWrite != 0 {
                ptr::null()
            } else {
                VNamesBegin
            }) as *const c_void,
            ElmSize: mem::size_of::<u8>(),
            NumElm: VNamesSize as usize,
            UseZeroPadding: 0,
        },
        ProfDataIOVec {
            Data: ptr::null(),
            ElmSize: mem::size_of::<u8>(),
            NumElm: PaddingBytesAfterVNames as usize,
            UseZeroPadding: 1,
        },
    ];
    if (*Writer).Write.expect("non-null function pointer")(
        Writer,
        IOVecData.as_mut_ptr(),
        (mem::size_of::<[ProfDataIOVec; 14]>()).wrapping_div(mem::size_of::<ProfDataIOVec>())
            as u32,
    ) != 0
    {
        return -1;
    }
    if __llvm_profile_is_continuous_mode_enabled() != 0 || NumData == 0 && NamesSize == 0 {
        return 0;
    }
    writeValueProfData(Writer, VPDataReader, DataBegin, DataEnd)
}
