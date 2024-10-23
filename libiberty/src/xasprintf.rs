use ::libc;
extern "C" {
    fn xvasprintf(_: *const libc::c_char, _: ::core::ffi::VaList) -> *mut libc::c_char;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type va_list = __builtin_va_list;
#[no_mangle]
pub unsafe extern "C" fn xasprintf(
    mut fmt: *const libc::c_char,
    mut args: ...
) -> *mut libc::c_char {
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    buf = xvasprintf(fmt, ap.as_va_list());
    return buf;
}
