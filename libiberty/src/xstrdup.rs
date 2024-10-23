use ::libc;
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn xmalloc(_: size_t) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
#[no_mangle]
pub unsafe extern "C" fn xstrdup(mut s: *const libc::c_char) -> *mut libc::c_char {
    let mut len: size_t = (strlen(s)).wrapping_add(1 as libc::c_int as libc::c_ulong);
    let mut ret: *mut libc::c_char = xmalloc(
        (::core::mem::size_of::<libc::c_char>() as libc::c_ulong).wrapping_mul(len),
    ) as *mut libc::c_char;
    return memcpy(ret as *mut libc::c_void, s as *const libc::c_void, len)
        as *mut libc::c_char;
}
