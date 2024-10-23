use ::libc;
extern "C" {
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
}
static mut xstrerror_buf: [libc::c_char; 43] = [0; 43];
#[no_mangle]
pub unsafe extern "C" fn xstrerror(mut errnum: libc::c_int) -> *mut libc::c_char {
    let mut errstr: *mut libc::c_char = 0 as *mut libc::c_char;
    errstr = strerror(errnum);
    if errstr.is_null() {
        sprintf(
            xstrerror_buf.as_mut_ptr(),
            b"undocumented error #%d\0" as *const u8 as *const libc::c_char,
            errnum,
        );
        errstr = xstrerror_buf.as_mut_ptr();
    }
    return errstr;
}
