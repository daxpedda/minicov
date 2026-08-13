use core::ffi::{c_char, c_int};

use super::InstrProfData::{
    __llvm_gcov_init_func_struct, __llvm_profile_data, VTableProfData, ValueProfNode,
};
use super::InstrProfilingInternal::ProfDataWriter;

extern "C" {
    #[linkage = "extern_weak"]
    static __start___llvm_prf_data: *const __llvm_profile_data;
    #[linkage = "extern_weak"]
    static __stop___llvm_prf_data: *const __llvm_profile_data;
    #[linkage = "extern_weak"]
    static __start___llvm_prf_cnts: *mut c_char;
    #[linkage = "extern_weak"]
    static __stop___llvm_prf_cnts: *mut c_char;
    #[linkage = "extern_weak"]
    static __start___llvm_prf_vtab: *const VTableProfData;
    #[linkage = "extern_weak"]
    static __stop___llvm_prf_vtab: *const VTableProfData;
    #[linkage = "extern_weak"]
    static __start___llvm_prf_vns: *const c_char;
    #[linkage = "extern_weak"]
    static __stop___llvm_prf_vns: *const c_char;
    #[linkage = "extern_weak"]
    static __start___llvm_prf_bits: *mut c_char;
    #[linkage = "extern_weak"]
    static __stop___llvm_prf_bits: *mut c_char;
    #[linkage = "extern_weak"]
    static __start___llvm_prf_names: *const c_char;
    #[linkage = "extern_weak"]
    static __stop___llvm_prf_names: *const c_char;
    #[linkage = "extern_weak"]
    static __start___llvm_prf_vnds: *mut ValueProfNode;
    #[linkage = "extern_weak"]
    static __stop___llvm_prf_vnds: *mut ValueProfNode;
    #[linkage = "extern_weak"]
    static __start___llvm_covinit: *const __llvm_gcov_init_func_struct;
    #[linkage = "extern_weak"]
    static __stop___llvm_covinit: *const __llvm_gcov_init_func_struct;
}

#[no_mangle]
pub unsafe extern "C" fn __llvm_profile_begin_data() -> *const __llvm_profile_data {
    __start___llvm_prf_data
}

#[no_mangle]
pub unsafe extern "C" fn __llvm_profile_end_data() -> *const __llvm_profile_data {
    __stop___llvm_prf_data
}

pub unsafe fn __llvm_profile_begin_names() -> *const c_char {
    __start___llvm_prf_names
}

pub unsafe fn __llvm_profile_end_names() -> *const c_char {
    __stop___llvm_prf_names
}

pub unsafe fn __llvm_profile_begin_vtabnames() -> *const c_char {
    __start___llvm_prf_vns
}

pub unsafe fn __llvm_profile_end_vtabnames() -> *const c_char {
    __stop___llvm_prf_vns
}

pub unsafe fn __llvm_profile_begin_vtables() -> *const VTableProfData {
    __start___llvm_prf_vtab
}

pub unsafe fn __llvm_profile_end_vtables() -> *const VTableProfData {
    __stop___llvm_prf_vtab
}

pub unsafe fn __llvm_profile_begin_counters() -> *mut c_char {
    __start___llvm_prf_cnts
}

pub unsafe fn __llvm_profile_end_counters() -> *mut c_char {
    __stop___llvm_prf_cnts
}

pub unsafe fn __llvm_profile_begin_bitmap() -> *mut c_char {
    __start___llvm_prf_bits
}

pub unsafe fn __llvm_profile_end_bitmap() -> *mut c_char {
    __stop___llvm_prf_bits
}

pub unsafe fn __llvm_profile_begin_vnodes() -> *mut ValueProfNode {
    __start___llvm_prf_vnds
}

pub unsafe fn __llvm_profile_end_vnodes() -> *mut ValueProfNode {
    __stop___llvm_prf_vnds
}

pub static mut CurrentVNode: *mut ValueProfNode =
    &raw const __start___llvm_prf_vnds as *mut ValueProfNode;
pub static mut EndVNode: *mut ValueProfNode =
    &raw const __stop___llvm_prf_vnds as *mut ValueProfNode;

pub unsafe fn __llvm_profile_begin_covinit() -> *const __llvm_gcov_init_func_struct {
    __start___llvm_covinit
}

pub unsafe fn __llvm_profile_end_covinit() -> *const __llvm_gcov_init_func_struct {
    __stop___llvm_covinit
}

pub unsafe fn __llvm_write_binary_ids(_Writer: *mut ProfDataWriter) -> c_int {
    0
}
