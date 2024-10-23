use ::libc;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[no_mangle]
pub unsafe extern "C" fn timeval_add(
    mut result: *mut timeval,
    mut a: *const timeval,
    mut b: *const timeval,
) {
    (*result).tv_sec = (*a).tv_sec + (*b).tv_sec;
    (*result).tv_usec = (*a).tv_usec + (*b).tv_usec;
    if (*result).tv_usec >= 1000000 as libc::c_int as libc::c_long {
        (*result).tv_sec += 1;
        (*result).tv_sec;
        (*result).tv_usec -= 1000000 as libc::c_int as libc::c_long;
    }
}
#[no_mangle]
pub unsafe extern "C" fn timeval_sub(
    mut result: *mut timeval,
    mut a: *const timeval,
    mut b: *const timeval,
) {
    (*result).tv_sec = (*a).tv_sec - (*b).tv_sec;
    (*result).tv_usec = (*a).tv_usec - (*b).tv_usec;
    if (*result).tv_usec < 0 as libc::c_int as libc::c_long {
        (*result).tv_sec -= 1;
        (*result).tv_sec;
        (*result).tv_usec += 1000000 as libc::c_int as libc::c_long;
    }
}
