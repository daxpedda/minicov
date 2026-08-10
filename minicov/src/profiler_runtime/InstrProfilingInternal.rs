static mut ProfileDumped: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
#[no_mangle]
pub unsafe extern "C" fn lprofProfileDumped() -> ::core::ffi::c_uint {
    return ProfileDumped;
}
#[no_mangle]
pub unsafe extern "C" fn lprofSetProfileDumped(mut Value: ::core::ffi::c_uint) {
    ProfileDumped = Value;
}
