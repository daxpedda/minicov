use core::ptr;

use alloc::vec::{self, Vec};

extern "C" {
    fn initBufferWriter(BufferWriter: *mut ProfDataWriter, Buffer: *mut ::core::ffi::c_char);
    fn __llvm_write_binary_ids(Writer: *mut ProfDataWriter) -> ::core::ffi::c_int;
    fn __llvm_profile_is_continuous_mode_enabled() -> ::core::ffi::c_int;
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
    fn __llvm_profile_get_magic() -> uint64_t;
    fn __llvm_profile_get_version() -> uint64_t;
    fn __llvm_profile_get_num_data(
        Begin: *const __llvm_profile_data,
        End: *const __llvm_profile_data,
    ) -> uint64_t;
    fn __llvm_profile_get_data_size(
        Begin: *const __llvm_profile_data,
        End: *const __llvm_profile_data,
    ) -> uint64_t;
    fn __llvm_profile_get_num_counters(
        Begin: *const ::core::ffi::c_char,
        End: *const ::core::ffi::c_char,
    ) -> uint64_t;
    fn __llvm_profile_get_counters_size(
        Begin: *const ::core::ffi::c_char,
        End: *const ::core::ffi::c_char,
    ) -> uint64_t;
    fn __llvm_profile_get_num_bitmap_bytes(
        Begin: *const ::core::ffi::c_char,
        End: *const ::core::ffi::c_char,
    ) -> uint64_t;
    fn __llvm_profile_get_name_size(
        Begin: *const ::core::ffi::c_char,
        End: *const ::core::ffi::c_char,
    ) -> uint64_t;
    fn __llvm_profile_get_num_vtable(
        Begin: *const VTableProfData,
        End: *const VTableProfData,
    ) -> uint64_t;
    fn __llvm_profile_get_vtable_section_size(
        Begin: *const VTableProfData,
        End: *const VTableProfData,
    ) -> uint64_t;
    fn __llvm_profile_get_padding_sizes_for_counters(
        DataSize: uint64_t,
        CountersSize: uint64_t,
        NumBitmapBytes: uint64_t,
        NumUniformCounters: uint64_t,
        NamesSize: uint64_t,
        VTableSize: uint64_t,
        VNameSize: uint64_t,
        PaddingBytesBeforeCounters: *mut uint64_t,
        PaddingBytesAfterCounters: *mut uint64_t,
        PaddingBytesAfterBitmap: *mut uint64_t,
        PaddingBytesAfterUniformCounters: *mut uint64_t,
        PaddingBytesAfterNames: *mut uint64_t,
        PaddingBytesAfterVTable: *mut uint64_t,
        PaddingBytesAfterVNames: *mut uint64_t,
    ) -> ::core::ffi::c_int;
}
pub type size_t = usize;
pub type uint64_t = u64;
pub type uint32_t = u32;
pub type uint16_t = u16;
pub type uint8_t = u8;
pub type uintptr_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct InstrProfValueData {
    pub Value: uint64_t,
    pub Count: uint64_t,
}
pub type ValueKind = ::core::ffi::c_uint;
pub const IPVK_Last: ValueKind = 2;
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
pub struct ProfDataIOVec {
    pub Data: *const ::core::ffi::c_void,
    pub ElmSize: size_t,
    pub NumElm: size_t,
    pub UseZeroPadding: ::core::ffi::c_int,
}
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
pub struct ProfBufferIO {
    pub FileWriter: *mut ProfDataWriter,
    pub OwnFileWriter: uint32_t,
    pub BufferStart: *mut uint8_t,
    pub BufferSz: uint32_t,
    pub CurOffset: uint32_t,
}
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
#[no_mangle]
pub static mut FreeHook: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()> = None;
static mut TheBufferIO: ProfBufferIO = ProfBufferIO {
    FileWriter: ::core::ptr::null::<ProfDataWriter>() as *mut ProfDataWriter,
    OwnFileWriter: 0,
    BufferStart: ::core::ptr::null::<uint8_t>() as *mut uint8_t,
    BufferSz: 0,
    CurOffset: 0,
};
static mut BufferIOBuffer: [uint8_t; 8192] = [0; 8192];
static mut VPDataArray: [InstrProfValueData; 16] = [InstrProfValueData { Value: 0, Count: 0 }; 16];
static mut VPDataArraySize: uint32_t = (::core::mem::size_of::<[InstrProfValueData; 16]>() as usize)
    .wrapping_div(::core::mem::size_of::<InstrProfValueData>() as usize)
    as uint32_t;
#[no_mangle]
pub static mut DynamicBufferIOBuffer: *mut uint8_t = ::core::ptr::null::<uint8_t>() as *mut uint8_t;
#[no_mangle]
pub static mut VPBufferSize: uint32_t = 0 as uint32_t;
#[no_mangle]
pub unsafe extern "C" fn lprofBufferWriter(
    This: *mut ProfDataWriter,
    IOVecs: *mut ProfDataIOVec,
    NumIOVecs: uint32_t,
) -> uint32_t {
    let mut I: uint32_t;
    let Buffer = &raw mut (*This).WriterCtx as *mut *mut ::core::ffi::c_char;
    I = 0 as uint32_t;
    while I < NumIOVecs {
        let Length: size_t = (*IOVecs.offset(I as isize))
            .ElmSize
            .wrapping_mul((*IOVecs.offset(I as isize)).NumElm);
        if !(*IOVecs.offset(I as isize)).Data.is_null() {
            ptr::copy_nonoverlapping(
                (*IOVecs.offset(I as isize)).Data,
                *Buffer as *mut ::core::ffi::c_void,
                Length,
            );
        }
        *Buffer = (*Buffer).add(Length);
        I = I.wrapping_add(1);
    }
    0 as uint32_t
}
unsafe extern "C" fn llvmInitBufferIO(
    BufferIO: *mut ProfBufferIO,
    FileWriter: *mut ProfDataWriter,
    Buffer: *mut uint8_t,
    BufferSz: uint32_t,
) {
    (*BufferIO).FileWriter = FileWriter;
    (*BufferIO).OwnFileWriter = 0 as uint32_t;
    (*BufferIO).BufferStart = Buffer;
    (*BufferIO).BufferSz = BufferSz;
    (*BufferIO).CurOffset = 0 as uint32_t;
}
#[no_mangle]
pub unsafe extern "C" fn lprofCreateBufferIO(FileWriter: *mut ProfDataWriter) -> *mut ProfBufferIO {
    let mut Buffer = DynamicBufferIOBuffer;
    let mut BufferSize: uint32_t = VPBufferSize;
    if Buffer.is_null() {
        Buffer = (&raw mut BufferIOBuffer as *mut uint8_t).offset(0 as ::core::ffi::c_int as isize)
            as *mut uint8_t;
        BufferSize = ::core::mem::size_of::<[uint8_t; 8192]>() as uint32_t;
    }
    llvmInitBufferIO(&raw mut TheBufferIO, FileWriter, Buffer, BufferSize);
    &raw mut TheBufferIO
}
#[no_mangle]
pub unsafe extern "C" fn lprofDeleteBufferIO(BufferIO: *mut ProfBufferIO) {
    if (*BufferIO).OwnFileWriter != 0 {
        FreeHook.expect("non-null function pointer")(
            (*BufferIO).FileWriter as *mut ::core::ffi::c_void,
        );
    }
    if !DynamicBufferIOBuffer.is_null() {
        FreeHook.expect("non-null function pointer")(
            DynamicBufferIOBuffer as *mut ::core::ffi::c_void,
        );
        DynamicBufferIOBuffer = ::core::ptr::null_mut::<uint8_t>();
        VPBufferSize = 0 as uint32_t;
    }
}
#[no_mangle]
pub unsafe extern "C" fn lprofBufferIOWrite(
    BufferIO: *mut ProfBufferIO,
    Data: *const uint8_t,
    Size: uint32_t,
) -> ::core::ffi::c_int {
    if Size.wrapping_add((*BufferIO).CurOffset) > (*BufferIO).BufferSz
        && lprofBufferIOFlush(BufferIO) != 0 as ::core::ffi::c_int
    {
        return -(1 as ::core::ffi::c_int);
    }
    let mut IO: [ProfDataIOVec; 1] = [ProfDataIOVec {
        Data: Data as *const ::core::ffi::c_void,
        ElmSize: ::core::mem::size_of::<uint8_t>() as size_t,
        NumElm: Size as size_t,
        UseZeroPadding: 0 as ::core::ffi::c_int,
    }];
    if Size > (*BufferIO).BufferSz {
        if (*(*BufferIO).FileWriter)
            .Write
            .expect("non-null function pointer")(
            (*BufferIO).FileWriter,
            &raw mut IO as *mut ProfDataIOVec,
            1 as uint32_t,
        ) != 0
        {
            return -(1 as ::core::ffi::c_int);
        }
    } else {
        let Buffer = (*BufferIO)
            .BufferStart
            .offset((*BufferIO).CurOffset as isize);
        let mut BufferWriter = ProfDataWriter {
            Write: None,
            WriterCtx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        };
        initBufferWriter(&raw mut BufferWriter, Buffer as *mut ::core::ffi::c_char);
        lprofBufferWriter(
            &raw mut BufferWriter,
            &raw mut IO as *mut ProfDataIOVec,
            1 as uint32_t,
        );
        (*BufferIO).CurOffset = (BufferWriter.WriterCtx as *mut uint8_t)
            .offset_from((*BufferIO).BufferStart)
            as ::core::ffi::c_long as uint32_t;
    }
    0 as ::core::ffi::c_int
}
#[no_mangle]
pub unsafe extern "C" fn lprofBufferIOFlush(BufferIO: *mut ProfBufferIO) -> ::core::ffi::c_int {
    if (*BufferIO).CurOffset != 0 {
        let mut IO: [ProfDataIOVec; 1] = [ProfDataIOVec {
            Data: (*BufferIO).BufferStart as *const ::core::ffi::c_void,
            ElmSize: ::core::mem::size_of::<uint8_t>() as size_t,
            NumElm: (*BufferIO).CurOffset as size_t,
            UseZeroPadding: 0 as ::core::ffi::c_int,
        }];
        if (*(*BufferIO).FileWriter)
            .Write
            .expect("non-null function pointer")(
            (*BufferIO).FileWriter,
            &raw mut IO as *mut ProfDataIOVec,
            1 as uint32_t,
        ) != 0
        {
            return -(1 as ::core::ffi::c_int);
        }
        (*BufferIO).CurOffset = 0 as uint32_t;
    }
    0 as ::core::ffi::c_int
}
unsafe extern "C" fn writeOneValueProfData(
    BufferIO: *mut ProfBufferIO,
    VPDataReader: *mut VPDataReaderType,
    Data: *const __llvm_profile_data,
) -> ::core::ffi::c_int {
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    let mut I: ::core::ffi::c_uint;

    let mut VPHeader = ValueProfData {
        TotalSize: 0,
        NumValueKinds: 0,
    };
    let mut SiteCountArray: [*mut uint8_t; 3] = [::core::ptr::null_mut::<uint8_t>(); 3];
    I = 0 as ::core::ffi::c_uint;
    while I <= IPVK_Last as ::core::ffi::c_int as ::core::ffi::c_uint {
        if (*Data).0.NumValueSites[I as usize] == 0 {
            SiteCountArray[I as usize] = ::core::ptr::null_mut::<uint8_t>();
        } else {
            let Sz: uint32_t = ((*VPDataReader)
                .GetValueProfRecordHeaderSize
                .expect("non-null function pointer")(
                (*Data).0.NumValueSites[I as usize] as uint32_t,
            ) as ::core::ffi::c_ulong)
                .wrapping_sub(8 as ::core::ffi::c_ulong) as uint32_t;
            alloca_allocations.push(vec::from_elem(0, Sz as ::core::ffi::c_ulong as usize));
            SiteCountArray[I as usize] =
                alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut uint8_t;
            ptr::write_bytes(
                SiteCountArray[I as usize] as *mut ::core::ffi::c_void,
                0,
                Sz as ::core::ffi::c_ulong as usize,
            );
        }
        I = I.wrapping_add(1);
    }
    let NumValueKinds: ::core::ffi::c_uint = (*VPDataReader)
        .InitRTRecord
        .expect("non-null function pointer")(
        Data, &raw mut SiteCountArray as *mut *mut uint8_t
    ) as ::core::ffi::c_uint;
    if NumValueKinds == 0 {
        return 0 as ::core::ffi::c_int;
    }
    VPHeader.TotalSize = (*VPDataReader)
        .GetValueProfDataSize
        .expect("non-null function pointer")();
    VPHeader.NumValueKinds = NumValueKinds as uint32_t;
    if lprofBufferIOWrite(
        BufferIO,
        &raw mut VPHeader as *const uint8_t,
        ::core::mem::size_of::<ValueProfData>() as uint32_t,
    ) != 0
    {
        return -(1 as ::core::ffi::c_int);
    }
    if (*VPDataReader)
        .GetFirstValueProfRecord
        .expect("non-null function pointer")(&raw mut VPHeader) as *mut ::core::ffi::c_void
        != (&raw mut VPHeader).offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void
    {
        return -(1 as ::core::ffi::c_int);
    }
    I = 0 as ::core::ffi::c_uint;
    while I <= IPVK_Last as ::core::ffi::c_int as ::core::ffi::c_uint {
        let mut J: uint32_t;
        let mut RecordHeader = ValueProfRecord {
            Kind: 0,
            NumValueSites: 0,
            SiteCountArray: [0; 1],
        };
        let RecordHeaderSize: uint32_t = 8 as ::core::ffi::c_ulong as uint32_t;
        let SiteCountArraySize: uint32_t;
        if !((*Data).0.NumValueSites[I as usize] == 0) {
            RecordHeader.Kind = I as uint32_t;
            RecordHeader.NumValueSites = (*Data).0.NumValueSites[I as usize] as uint32_t;
            if lprofBufferIOWrite(
                BufferIO,
                &raw mut RecordHeader as *const uint8_t,
                RecordHeaderSize,
            ) != 0
            {
                return -(1 as ::core::ffi::c_int);
            }
            SiteCountArraySize = (*VPDataReader)
                .GetValueProfRecordHeaderSize
                .expect("non-null function pointer")(
                (*Data).0.NumValueSites[I as usize] as uint32_t,
            )
            .wrapping_sub(RecordHeaderSize);
            if lprofBufferIOWrite(BufferIO, SiteCountArray[I as usize], SiteCountArraySize) != 0 {
                return -(1 as ::core::ffi::c_int);
            }
            J = 0 as uint32_t;
            while J < (*Data).0.NumValueSites[I as usize] as uint32_t {
                let mut NRead: uint32_t;
                let mut NRemain: uint32_t;
                let mut NextStartNode = ::core::ptr::null_mut::<ValueProfNode>();
                NRemain = (*VPDataReader)
                    .GetNumValueDataForSite
                    .expect("non-null function pointer")(I as uint32_t, J);
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
                            I as uint32_t,
                            J,
                            (&raw mut VPDataArray as *mut InstrProfValueData)
                                .offset(0 as ::core::ffi::c_int as isize),
                            NextStartNode,
                            NRead,
                        );
                        if lprofBufferIOWrite(
                            BufferIO,
                            (&raw mut VPDataArray as *mut InstrProfValueData)
                                .offset(0 as ::core::ffi::c_int as isize)
                                as *const uint8_t,
                            (NRead as usize)
                                .wrapping_mul(::core::mem::size_of::<InstrProfValueData>() as usize)
                                as uint32_t,
                        ) != 0
                        {
                            return -(1 as ::core::ffi::c_int);
                        }
                        NRemain = NRemain.wrapping_sub(NRead);
                        if !(NRemain != 0 as uint32_t) {
                            break;
                        }
                    }
                }
                J = J.wrapping_add(1);
            }
        }
        I = I.wrapping_add(1);
    }
    0 as ::core::ffi::c_int
}
unsafe extern "C" fn writeValueProfData(
    Writer: *mut ProfDataWriter,
    VPDataReader: *mut VPDataReaderType,
    DataBegin: *const __llvm_profile_data,
    DataEnd: *const __llvm_profile_data,
) -> ::core::ffi::c_int {
    let mut DI: *const __llvm_profile_data;
    if VPDataReader.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    let BufferIO: *mut ProfBufferIO = lprofCreateBufferIO(Writer);
    DI = DataBegin;
    while DI < DataEnd {
        if writeOneValueProfData(BufferIO, VPDataReader, DI) != 0 {
            return -(1 as ::core::ffi::c_int);
        }
        DI = DI.offset(1);
    }
    if lprofBufferIOFlush(BufferIO) != 0 as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    lprofDeleteBufferIO(BufferIO);
    0 as ::core::ffi::c_int
}
#[no_mangle]
pub unsafe extern "C" fn lprofWriteData(
    Writer: *mut ProfDataWriter,
    VPDataReader: *mut VPDataReaderType,
    SkipNameDataWrite: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
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
    let Version = __llvm_profile_get_version();
    lprofWriteDataImpl(
        Writer,
        DataBegin,
        DataEnd,
        CountersBegin,
        CountersEnd,
        BitmapBegin,
        BitmapEnd,
        ::core::ptr::null::<::core::ffi::c_char>(),
        ::core::ptr::null::<::core::ffi::c_char>(),
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
#[no_mangle]
pub unsafe extern "C" fn lprofWriteDataImpl(
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
) -> ::core::ffi::c_int {
    let DataSectionSize = __llvm_profile_get_data_size(DataBegin, DataEnd) as uint64_t;
    let NumData = __llvm_profile_get_num_data(DataBegin, DataEnd) as uint64_t;
    let CountersSectionSize =
        __llvm_profile_get_counters_size(CountersBegin, CountersEnd) as uint64_t;
    let NumCounters = __llvm_profile_get_num_counters(CountersBegin, CountersEnd) as uint64_t;
    let NumBitmapBytes = __llvm_profile_get_num_bitmap_bytes(BitmapBegin, BitmapEnd) as uint64_t;
    let NamesSize = __llvm_profile_get_name_size(NamesBegin, NamesEnd) as uint64_t;
    let NumVTables = __llvm_profile_get_num_vtable(VTableBegin, VTableEnd) as uint64_t;
    let VTableSectionSize =
        __llvm_profile_get_vtable_section_size(VTableBegin, VTableEnd) as uint64_t;
    let VNamesSize = __llvm_profile_get_name_size(VNamesBegin, VNamesEnd) as uint64_t;
    let NumUniformCounters: uint64_t = (if !UniformCountersBegin.is_null()
        && !UniformCountersEnd.is_null()
        && UniformCountersEnd > UniformCountersBegin
    {
        (UniformCountersEnd.offset_from(UniformCountersBegin) as ::core::ffi::c_long as usize)
            .wrapping_div(::core::mem::size_of::<uint64_t>() as usize)
    } else {
        0
    }) as uint64_t;
    let UniformCountersSectionSize: uint64_t =
        NumUniformCounters.wrapping_mul(::core::mem::size_of::<uint64_t>() as uint64_t);
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
    let mut PaddingBytesBeforeCounters: uint64_t = 0;
    let mut PaddingBytesAfterCounters: uint64_t = 0;
    let mut PaddingBytesAfterBitmapBytes: uint64_t = 0;
    let mut PaddingBytesAfterUniformCounters: uint64_t = 0;
    let mut PaddingBytesAfterNames: uint64_t = 0;
    let mut PaddingBytesAfterVTable: uint64_t = 0;
    let mut PaddingBytesAfterVNames: uint64_t = 0;
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
    ) == -(1 as ::core::ffi::c_int)
    {
        return -(1 as ::core::ffi::c_int);
    }
    Header.Magic = __llvm_profile_get_magic();
    Header.Version = __llvm_profile_get_version();
    Header.BinaryIdsSize =
        __llvm_write_binary_ids(::core::ptr::null_mut::<ProfDataWriter>()) as uint64_t;
    Header.NumData = NumData;
    Header.PaddingBytesBeforeCounters = PaddingBytesBeforeCounters;
    Header.NumCounters = NumCounters;
    Header.PaddingBytesAfterCounters = PaddingBytesAfterCounters;
    Header.NumBitmapBytes = NumBitmapBytes;
    Header.PaddingBytesAfterBitmapBytes = PaddingBytesAfterBitmapBytes;
    Header.NumUniformCounters = NumUniformCounters;
    Header.PaddingBytesAfterUniformCounters = PaddingBytesAfterUniformCounters;
    Header.UniformCountersDelta = (if !UniformCountersBegin.is_null() {
        (UniformCountersBegin as uintptr_t).wrapping_sub(DataBegin as uintptr_t)
    } else {
        0 as uintptr_t
    }) as uint64_t;
    Header.NamesSize = NamesSize;
    Header.CountersDelta =
        (CountersBegin as uintptr_t).wrapping_sub(DataBegin as uintptr_t) as uint64_t;
    Header.BitmapDelta =
        (BitmapBegin as uintptr_t).wrapping_sub(DataBegin as uintptr_t) as uint64_t;
    Header.NamesDelta = NamesBegin as uintptr_t as uint64_t;
    Header.NumVTables = NumVTables;
    Header.VNamesSize = VNamesSize;
    Header.ValueKindLast = IPVK_Last as ::core::ffi::c_int as uint64_t;
    Header.Version = Version;
    if NumUniformCounters > 0 as uint64_t {
        Header.UniformCountersDelta = DataSectionSize
            .wrapping_add(PaddingBytesBeforeCounters)
            .wrapping_add(CountersSectionSize)
            .wrapping_add(PaddingBytesAfterCounters)
            .wrapping_add(NumBitmapBytes)
            .wrapping_add(PaddingBytesAfterBitmapBytes);
    } else {
        Header.UniformCountersDelta = 0 as uint64_t;
    }
    if NumData == 0 as uint64_t && NamesSize == 0 as uint64_t {
        Header.CountersDelta = 0 as uint64_t;
        Header.BitmapDelta = 0 as uint64_t;
        Header.NamesDelta = 0 as uint64_t;
        Header.UniformCountersDelta = 0 as uint64_t;
    }
    let mut IOVec: [ProfDataIOVec; 1] = [ProfDataIOVec {
        Data: &raw mut Header as *const ::core::ffi::c_void,
        ElmSize: ::core::mem::size_of::<__llvm_profile_header>() as size_t,
        NumElm: 1 as size_t,
        UseZeroPadding: 0 as ::core::ffi::c_int,
    }];
    if (*Writer).Write.expect("non-null function pointer")(
        Writer,
        &raw mut IOVec as *mut ProfDataIOVec,
        (::core::mem::size_of::<[ProfDataIOVec; 1]>() as usize)
            .wrapping_div(::core::mem::size_of::<ProfDataIOVec>() as usize) as uint32_t,
    ) != 0
    {
        return -(1 as ::core::ffi::c_int);
    }
    if __llvm_write_binary_ids(Writer) == -(1 as ::core::ffi::c_int) {
        return -(1 as ::core::ffi::c_int);
    }
    let mut IOVecData: [ProfDataIOVec; 14] = [
        ProfDataIOVec {
            Data: DataBegin as *const ::core::ffi::c_void,
            ElmSize: ::core::mem::size_of::<uint8_t>() as size_t,
            NumElm: DataSectionSize as size_t,
            UseZeroPadding: 0 as ::core::ffi::c_int,
        },
        ProfDataIOVec {
            Data: ::core::ptr::null::<::core::ffi::c_void>(),
            ElmSize: ::core::mem::size_of::<uint8_t>() as size_t,
            NumElm: PaddingBytesBeforeCounters as size_t,
            UseZeroPadding: 1 as ::core::ffi::c_int,
        },
        ProfDataIOVec {
            Data: CountersBegin as *const ::core::ffi::c_void,
            ElmSize: ::core::mem::size_of::<uint8_t>() as size_t,
            NumElm: CountersSectionSize as size_t,
            UseZeroPadding: 0 as ::core::ffi::c_int,
        },
        ProfDataIOVec {
            Data: ::core::ptr::null::<::core::ffi::c_void>(),
            ElmSize: ::core::mem::size_of::<uint8_t>() as size_t,
            NumElm: PaddingBytesAfterCounters as size_t,
            UseZeroPadding: 1 as ::core::ffi::c_int,
        },
        ProfDataIOVec {
            Data: BitmapBegin as *const ::core::ffi::c_void,
            ElmSize: ::core::mem::size_of::<uint8_t>() as size_t,
            NumElm: NumBitmapBytes as size_t,
            UseZeroPadding: 0 as ::core::ffi::c_int,
        },
        ProfDataIOVec {
            Data: ::core::ptr::null::<::core::ffi::c_void>(),
            ElmSize: ::core::mem::size_of::<uint8_t>() as size_t,
            NumElm: PaddingBytesAfterBitmapBytes as size_t,
            UseZeroPadding: 1 as ::core::ffi::c_int,
        },
        ProfDataIOVec {
            Data: UniformCountersBegin as *const ::core::ffi::c_void,
            ElmSize: ::core::mem::size_of::<uint8_t>() as size_t,
            NumElm: UniformCountersSectionSize as size_t,
            UseZeroPadding: 0 as ::core::ffi::c_int,
        },
        ProfDataIOVec {
            Data: ::core::ptr::null::<::core::ffi::c_void>(),
            ElmSize: ::core::mem::size_of::<uint8_t>() as size_t,
            NumElm: PaddingBytesAfterUniformCounters as size_t,
            UseZeroPadding: 1 as ::core::ffi::c_int,
        },
        ProfDataIOVec {
            Data: (if SkipNameDataWrite != 0 {
                ::core::ptr::null::<::core::ffi::c_char>()
            } else {
                NamesBegin
            }) as *const ::core::ffi::c_void,
            ElmSize: ::core::mem::size_of::<uint8_t>() as size_t,
            NumElm: NamesSize as size_t,
            UseZeroPadding: 0 as ::core::ffi::c_int,
        },
        ProfDataIOVec {
            Data: ::core::ptr::null::<::core::ffi::c_void>(),
            ElmSize: ::core::mem::size_of::<uint8_t>() as size_t,
            NumElm: PaddingBytesAfterNames as size_t,
            UseZeroPadding: 1 as ::core::ffi::c_int,
        },
        ProfDataIOVec {
            Data: VTableBegin as *const ::core::ffi::c_void,
            ElmSize: ::core::mem::size_of::<uint8_t>() as size_t,
            NumElm: VTableSectionSize as size_t,
            UseZeroPadding: 0 as ::core::ffi::c_int,
        },
        ProfDataIOVec {
            Data: ::core::ptr::null::<::core::ffi::c_void>(),
            ElmSize: ::core::mem::size_of::<uint8_t>() as size_t,
            NumElm: PaddingBytesAfterVTable as size_t,
            UseZeroPadding: 1 as ::core::ffi::c_int,
        },
        ProfDataIOVec {
            Data: (if SkipNameDataWrite != 0 {
                ::core::ptr::null::<::core::ffi::c_char>()
            } else {
                VNamesBegin
            }) as *const ::core::ffi::c_void,
            ElmSize: ::core::mem::size_of::<uint8_t>() as size_t,
            NumElm: VNamesSize as size_t,
            UseZeroPadding: 0 as ::core::ffi::c_int,
        },
        ProfDataIOVec {
            Data: ::core::ptr::null::<::core::ffi::c_void>(),
            ElmSize: ::core::mem::size_of::<uint8_t>() as size_t,
            NumElm: PaddingBytesAfterVNames as size_t,
            UseZeroPadding: 1 as ::core::ffi::c_int,
        },
    ];
    if (*Writer).Write.expect("non-null function pointer")(
        Writer,
        &raw mut IOVecData as *mut ProfDataIOVec,
        (::core::mem::size_of::<[ProfDataIOVec; 14]>() as usize)
            .wrapping_div(::core::mem::size_of::<ProfDataIOVec>() as usize) as uint32_t,
    ) != 0
    {
        return -(1 as ::core::ffi::c_int);
    }
    if __llvm_profile_is_continuous_mode_enabled() != 0
        || NumData == 0 as uint64_t && NamesSize == 0 as uint64_t
    {
        return 0 as ::core::ffi::c_int;
    }
    writeValueProfData(Writer, VPDataReader, DataBegin, DataEnd)
}
#[no_mangle]
pub unsafe extern "C" fn lprofWriteOneBinaryId(
    Writer: *mut ProfDataWriter,
    mut BinaryIdLen: uint64_t,
    BinaryIdData: *const uint8_t,
    BinaryIdPadding: uint64_t,
) -> ::core::ffi::c_int {
    let mut BinaryIdIOVec: [ProfDataIOVec; 3] = [
        ProfDataIOVec {
            Data: &raw mut BinaryIdLen as *const ::core::ffi::c_void,
            ElmSize: ::core::mem::size_of::<uint64_t>() as size_t,
            NumElm: 1 as size_t,
            UseZeroPadding: 0 as ::core::ffi::c_int,
        },
        ProfDataIOVec {
            Data: BinaryIdData as *const ::core::ffi::c_void,
            ElmSize: ::core::mem::size_of::<uint8_t>() as size_t,
            NumElm: BinaryIdLen as size_t,
            UseZeroPadding: 0 as ::core::ffi::c_int,
        },
        ProfDataIOVec {
            Data: ::core::ptr::null::<::core::ffi::c_void>(),
            ElmSize: ::core::mem::size_of::<uint8_t>() as size_t,
            NumElm: BinaryIdPadding as size_t,
            UseZeroPadding: 1 as ::core::ffi::c_int,
        },
    ];
    if (*Writer).Write.expect("non-null function pointer")(
        Writer,
        &raw mut BinaryIdIOVec as *mut ProfDataIOVec,
        (::core::mem::size_of::<[ProfDataIOVec; 3]>() as usize)
            .wrapping_div(::core::mem::size_of::<ProfDataIOVec>() as usize) as uint32_t,
    ) != 0
    {
        return -(1 as ::core::ffi::c_int);
    }
    0 as ::core::ffi::c_int
}
