use core::ffi::{c_char, c_int};
use core::{arch, mem, ptr};

use super::InstrProfData::{
    __llvm_profile_data, __llvm_profile_data_Inner, VTableProfData, ValueProfNode,
};
use super::InstrProfilingWriter::lprofWriteOneBinaryId;
use super::ProfDataWriter;

#[cfg(target_env = "msvc")]
#[link_section = ".drectve"]
#[used]
static DIRECTIVE_1: ([u8; 21], [u8; 21], [u8; 22]) = (
    *b" /MERGE:.lprfd=.data ",
    *b" /MERGE:.lprfv=.data ",
    *b" /MERGE:.lprfnd=.data ",
);

#[cfg(target_env = "msvc")]
arch::global_asm! {
    ".section .lprfn$A, \"r\"",
    ".section .lprfn$Z, \"r\"",
    ".section .lprfd$A, \"rw\"",
    ".section .lprfd$Z, \"rw\"",
    ".section .lprfc$A, \"rw\"",
    ".section .lprfc$Z, \"rw\"",
    ".section .lprfb$A, \"rw\"",
    ".section .lprfb$Z, \"rw\"",
    ".section .lprfnd$A, \"rw\"",
    ".section .lprfnd$Z, \"rw\"",
}

#[link_section = ".lprfd$A"]
pub static mut DataStart: __llvm_profile_data = __llvm_profile_data(__llvm_profile_data_Inner {
    NameRef: 0,
    FuncHash: 0,
    CounterPtr: ptr::null_mut(),
    UniformCounterPtr: ptr::null_mut(),
    BitmapPtr: ptr::null_mut(),
    FunctionPointer: ptr::null_mut(),
    Values: ptr::null_mut(),
    NumCounters: 0,
    NumValueSites: [0; 3],
    OffloadDeviceWaveSize: 0,
    NumBitmapBytes: 0,
});
#[link_section = ".lprfd$Z"]
pub static mut DataEnd: __llvm_profile_data = __llvm_profile_data(__llvm_profile_data_Inner {
    NameRef: 0,
    FuncHash: 0,
    CounterPtr: ptr::null_mut(),
    UniformCounterPtr: ptr::null_mut(),
    BitmapPtr: ptr::null_mut(),
    FunctionPointer: ptr::null_mut(),
    Values: ptr::null_mut(),
    NumCounters: 0,
    NumValueSites: [0; 3],
    OffloadDeviceWaveSize: 0,
    NumBitmapBytes: 0,
});

#[link_section = ".lprfn$A"]
pub static mut NamesStart: c_char = '\0' as c_char;
#[link_section = ".lprfn$Z"]
pub static mut NamesEnd: c_char = '\0' as c_char;

#[link_section = ".lprfc$A"]
pub static mut CountersStart: c_char = 0;
#[link_section = ".lprfc$Z"]
pub static mut CountersEnd: c_char = 0;
#[link_section = ".lprfb$A"]
pub static mut BitmapStart: c_char = 0;
#[link_section = ".lprfb$Z"]
pub static mut BitmapEnd: c_char = 0;

#[link_section = ".lprfnd$A"]
pub static mut VNodesStart: ValueProfNode = ValueProfNode {
    Value: 0,
    Count: 0,
    Next: ptr::null_mut(),
};
#[link_section = ".lprfnd$Z"]
pub static mut VNodesEnd: ValueProfNode = ValueProfNode {
    Value: 0,
    Count: 0,
    Next: ptr::null_mut(),
};

#[no_mangle]
pub unsafe extern "C" fn __llvm_profile_begin_data() -> *const __llvm_profile_data {
    (&raw mut DataStart).offset(1)
}

#[no_mangle]
pub unsafe extern "C" fn __llvm_profile_end_data() -> *const __llvm_profile_data {
    &raw mut DataEnd
}

pub unsafe fn __llvm_profile_begin_vtables() -> *const VTableProfData {
    ptr::null()
}

pub unsafe fn __llvm_profile_end_vtables() -> *const VTableProfData {
    ptr::null()
}

pub unsafe fn __llvm_profile_begin_names() -> *const c_char {
    (&raw const NamesStart).offset(1)
}

pub unsafe fn __llvm_profile_end_names() -> *const c_char {
    &raw const NamesEnd
}

pub unsafe fn __llvm_profile_begin_vtabnames() -> *const c_char {
    ptr::null()
}

pub unsafe fn __llvm_profile_end_vtabnames() -> *const c_char {
    ptr::null()
}

pub unsafe fn __llvm_profile_begin_counters() -> *mut c_char {
    (&raw mut CountersStart).offset(1)
}

pub unsafe fn __llvm_profile_end_counters() -> *mut c_char {
    &raw mut CountersEnd
}

pub unsafe fn __llvm_profile_begin_bitmap() -> *mut c_char {
    (&raw mut BitmapStart).offset(1)
}

pub unsafe fn __llvm_profile_end_bitmap() -> *mut c_char {
    &raw mut BitmapEnd
}

pub unsafe fn __llvm_profile_begin_vnodes() -> *mut ValueProfNode {
    (&raw mut VNodesStart).offset(1)
}

pub unsafe fn __llvm_profile_end_vnodes() -> *mut ValueProfNode {
    &raw mut VNodesEnd
}

pub static mut CurrentVNode: *mut ValueProfNode = unsafe { (&raw mut VNodesStart).offset(1) };
pub static mut EndVNode: *mut ValueProfNode = &raw mut VNodesEnd;

pub const BUILD_ID_LEN: usize = 16;
#[no_mangle]
#[linkage = "weak"]
pub static mut __buildid: [u8; BUILD_ID_LEN] = [0; BUILD_ID_LEN];

pub unsafe fn __llvm_write_binary_ids(Writer: *mut ProfDataWriter) -> c_int {
    if __buildid != [0; BUILD_ID_LEN] {
        #[expect(static_mut_refs)]
        if !Writer.is_null()
            && lprofWriteOneBinaryId(Writer, BUILD_ID_LEN as u64, __buildid.as_mut_ptr(), 0) == -1
        {
            return -1;
        }
        return (mem::size_of::<u64>()).wrapping_add(BUILD_ID_LEN) as c_int;
    }
    0
}
