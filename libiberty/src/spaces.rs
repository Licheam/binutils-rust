use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn spaces(mut count: libc::c_int) -> *const libc::c_char {
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    static mut buf: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
    static mut maxsize: libc::c_int = 0;
    if count > maxsize {
        free(buf as *mut libc::c_void);
        buf = malloc((count + 1 as libc::c_int) as libc::c_ulong) as *mut libc::c_char;
        if buf.is_null() {
            return 0 as *const libc::c_char;
        }
        t = buf.offset(count as isize);
        while t != buf {
            t = t.offset(-1);
            *t = ' ' as i32 as libc::c_char;
        }
        maxsize = count;
        *buf.offset(count as isize) = '\0' as i32 as libc::c_char;
    }
    return buf.offset(maxsize as isize).offset(-(count as isize)) as *const libc::c_char;
}
