use ::libc;
extern "C" {
    fn vsprintf(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: ::core::ffi::VaList,
    ) -> libc::c_int;
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    fn libiberty_vprintf_buffer_size(
        _: *const libc::c_char,
        _: ::core::ffi::VaList,
    ) -> libc::c_int;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type size_t = libc::c_ulong;
#[no_mangle]
pub unsafe extern "C" fn xvasprintf(
    mut format: *const libc::c_char,
    mut args: ::core::ffi::VaList,
) -> *mut libc::c_char {
    let mut result: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut total_width: libc::c_int = libiberty_vprintf_buffer_size(
        format,
        args.as_va_list(),
    );
    result = xmalloc(total_width as size_t) as *mut libc::c_char;
    vsprintf(result, format, args.as_va_list());
    return result;
}
