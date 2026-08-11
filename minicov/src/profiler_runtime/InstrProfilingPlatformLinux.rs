extern "C" {
    #[linkage = "extern_weak"]
    static __start___llvm_prf_data: *const __llvm_profile_data;
    #[linkage = "extern_weak"]
    static __stop___llvm_prf_data: *const __llvm_profile_data;
    #[linkage = "extern_weak"]
    static __start___llvm_prf_cnts: *mut ::core::ffi::c_char;
    #[linkage = "extern_weak"]
    static __stop___llvm_prf_cnts: *mut ::core::ffi::c_char;
    #[linkage = "extern_weak"]
    static __start___llvm_prf_vtab: *const VTableProfData;
    #[linkage = "extern_weak"]
    static __stop___llvm_prf_vtab: *const VTableProfData;
    #[linkage = "extern_weak"]
    static __start___llvm_prf_vns: *const ::core::ffi::c_char;
    #[linkage = "extern_weak"]
    static __stop___llvm_prf_vns: *const ::core::ffi::c_char;
    #[linkage = "extern_weak"]
    static __start___llvm_prf_bits: *mut ::core::ffi::c_char;
    #[linkage = "extern_weak"]
    static __stop___llvm_prf_bits: *mut ::core::ffi::c_char;
    #[linkage = "extern_weak"]
    static __start___llvm_prf_names: *const ::core::ffi::c_char;
    #[linkage = "extern_weak"]
    static __stop___llvm_prf_names: *const ::core::ffi::c_char;
    #[linkage = "extern_weak"]
    static __start___llvm_prf_vnds: *mut ValueProfNode;
    #[linkage = "extern_weak"]
    static __stop___llvm_prf_vnds: *mut ValueProfNode;
    #[linkage = "extern_weak"]
    static __start___llvm_covinit: *const __llvm_gcov_init_func_struct;
    #[linkage = "extern_weak"]
    static __stop___llvm_covinit: *const __llvm_gcov_init_func_struct;
}
pub type size_t = usize;
pub type uint64_t = u64;
pub type uint32_t = u32;
pub type uint16_t = u16;
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
#[repr(C, align(8))]
pub struct __llvm_gcov_init_func_struct(pub __llvm_gcov_init_func_struct_Inner);
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __llvm_gcov_init_func_struct_Inner {
    pub WriteoutFunction: IntPtrT,
    pub ResetFunction: IntPtrT,
}
#[allow(dead_code, non_upper_case_globals)]
const __llvm_gcov_init_func_struct_PADDING: usize =
    ::core::mem::size_of::<__llvm_gcov_init_func_struct>()
        - ::core::mem::size_of::<__llvm_gcov_init_func_struct_Inner>();
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
#[no_mangle]
pub unsafe extern "C" fn __llvm_profile_begin_data() -> *const __llvm_profile_data {
    return __start___llvm_prf_data;
}
#[no_mangle]
pub unsafe extern "C" fn __llvm_profile_end_data() -> *const __llvm_profile_data {
    return __stop___llvm_prf_data;
}
#[no_mangle]
pub unsafe extern "C" fn __llvm_profile_begin_names() -> *const ::core::ffi::c_char {
    return __start___llvm_prf_names;
}
#[no_mangle]
pub unsafe extern "C" fn __llvm_profile_end_names() -> *const ::core::ffi::c_char {
    return __stop___llvm_prf_names;
}
#[no_mangle]
pub unsafe extern "C" fn __llvm_profile_begin_vtabnames() -> *const ::core::ffi::c_char {
    return __start___llvm_prf_vns;
}
#[no_mangle]
pub unsafe extern "C" fn __llvm_profile_end_vtabnames() -> *const ::core::ffi::c_char {
    return __stop___llvm_prf_vns;
}
#[no_mangle]
pub unsafe extern "C" fn __llvm_profile_begin_vtables() -> *const VTableProfData {
    return __start___llvm_prf_vtab;
}
#[no_mangle]
pub unsafe extern "C" fn __llvm_profile_end_vtables() -> *const VTableProfData {
    return __stop___llvm_prf_vtab;
}
#[no_mangle]
pub unsafe extern "C" fn __llvm_profile_begin_counters() -> *mut ::core::ffi::c_char {
    return __start___llvm_prf_cnts;
}
#[no_mangle]
pub unsafe extern "C" fn __llvm_profile_end_counters() -> *mut ::core::ffi::c_char {
    return __stop___llvm_prf_cnts;
}
#[no_mangle]
pub unsafe extern "C" fn __llvm_profile_begin_bitmap() -> *mut ::core::ffi::c_char {
    return __start___llvm_prf_bits;
}
#[no_mangle]
pub unsafe extern "C" fn __llvm_profile_end_bitmap() -> *mut ::core::ffi::c_char {
    return __stop___llvm_prf_bits;
}
#[no_mangle]
pub unsafe extern "C" fn __llvm_profile_begin_vnodes() -> *mut ValueProfNode {
    return __start___llvm_prf_vnds;
}
#[no_mangle]
pub unsafe extern "C" fn __llvm_profile_end_vnodes() -> *mut ValueProfNode {
    return __stop___llvm_prf_vnds;
}
#[no_mangle]
pub static mut CurrentVNode: *mut ValueProfNode =
    unsafe { &raw const __start___llvm_prf_vnds as *mut ValueProfNode };
#[no_mangle]
pub static mut EndVNode: *mut ValueProfNode =
    unsafe { &raw const __stop___llvm_prf_vnds as *mut ValueProfNode };
#[no_mangle]
pub unsafe extern "C" fn __llvm_profile_begin_covinit() -> *const __llvm_gcov_init_func_struct {
    return __start___llvm_covinit;
}
#[no_mangle]
pub unsafe extern "C" fn __llvm_profile_end_covinit() -> *const __llvm_gcov_init_func_struct {
    return __stop___llvm_covinit;
}
#[no_mangle]
pub unsafe extern "C" fn __llvm_write_binary_ids(
    mut Writer: *mut ProfDataWriter,
) -> ::core::ffi::c_int {
    return 0 as ::core::ffi::c_int;
}
