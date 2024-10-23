use ::libc;
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strnlen(__string: *const libc::c_char, __maxlen: size_t) -> size_t;
    fn xmalloc(_: size_t) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
#[no_mangle]
pub unsafe extern "C" fn xstrndup(
    mut s: *const libc::c_char,
    mut n: size_t,
) -> *mut libc::c_char {
    let mut result: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = strnlen(s, n);
    result = xmalloc(
        (::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
            .wrapping_mul(len.wrapping_add(1 as libc::c_int as libc::c_ulong)),
    ) as *mut libc::c_char;
    *result.offset(len as isize) = '\0' as i32 as libc::c_char;
    return memcpy(result as *mut libc::c_void, s as *const libc::c_void, len)
        as *mut libc::c_char;
}
