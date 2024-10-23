use ::libc;
extern "C" {
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn free(_: *mut libc::c_void);
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
pub type size_t = libc::c_ulong;
pub type va_list = __builtin_va_list;
#[inline]
unsafe extern "C" fn vconcat_length(
    mut first: *const libc::c_char,
    mut args: ::core::ffi::VaList,
) -> libc::c_ulong {
    let mut length: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut arg: *const libc::c_char = 0 as *const libc::c_char;
    arg = first;
    while !arg.is_null() {
        length = length.wrapping_add(strlen(arg));
        arg = args.arg::<*const libc::c_char>();
    }
    return length;
}
#[inline]
unsafe extern "C" fn vconcat_copy(
    mut dst: *mut libc::c_char,
    mut first: *const libc::c_char,
    mut args: ::core::ffi::VaList,
) -> *mut libc::c_char {
    let mut end: *mut libc::c_char = dst;
    let mut arg: *const libc::c_char = 0 as *const libc::c_char;
    arg = first;
    while !arg.is_null() {
        let mut length: libc::c_ulong = strlen(arg);
        memcpy(end as *mut libc::c_void, arg as *const libc::c_void, length);
        end = end.offset(length as isize);
        arg = args.arg::<*const libc::c_char>();
    }
    *end = '\0' as i32 as libc::c_char;
    return dst;
}
#[no_mangle]
pub unsafe extern "C" fn concat_length(
    mut first: *const libc::c_char,
    mut args: ...
) -> libc::c_ulong {
    let mut length: libc::c_ulong = 0;
    let mut args_0: ::core::ffi::VaListImpl;
    args_0 = args.clone();
    length = vconcat_length(first, args_0.as_va_list());
    return length;
}
#[no_mangle]
pub unsafe extern "C" fn concat_copy(
    mut dst: *mut libc::c_char,
    mut first: *const libc::c_char,
    mut args: ...
) -> *mut libc::c_char {
    let mut save_dst: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut args_0: ::core::ffi::VaListImpl;
    args_0 = args.clone();
    vconcat_copy(dst, first, args_0.as_va_list());
    save_dst = dst;
    return save_dst;
}
#[no_mangle]
pub static mut libiberty_concat_ptr: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub unsafe extern "C" fn concat_copy2(
    mut first: *const libc::c_char,
    mut args: ...
) -> *mut libc::c_char {
    let mut args_0: ::core::ffi::VaListImpl;
    args_0 = args.clone();
    vconcat_copy(libiberty_concat_ptr, first, args_0.as_va_list());
    return libiberty_concat_ptr;
}
#[no_mangle]
pub unsafe extern "C" fn concat(
    mut first: *const libc::c_char,
    mut args: ...
) -> *mut libc::c_char {
    let mut newstr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut args_0: ::core::ffi::VaListImpl;
    args_0 = args.clone();
    newstr = xmalloc(
        (::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
            .wrapping_mul(
                (vconcat_length(first, args_0.as_va_list()))
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            ),
    ) as *mut libc::c_char;
    args_0 = args.clone();
    vconcat_copy(newstr, first, args_0.as_va_list());
    return newstr;
}
#[no_mangle]
pub unsafe extern "C" fn reconcat(
    mut optr: *mut libc::c_char,
    mut first: *const libc::c_char,
    mut args: ...
) -> *mut libc::c_char {
    let mut newstr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut args_0: ::core::ffi::VaListImpl;
    args_0 = args.clone();
    newstr = xmalloc(
        (::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
            .wrapping_mul(
                (vconcat_length(first, args_0.as_va_list()))
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            ),
    ) as *mut libc::c_char;
    args_0 = args.clone();
    vconcat_copy(newstr, first, args_0.as_va_list());
    if !optr.is_null() {
        free(optr as *mut libc::c_void);
    }
    return newstr;
}
