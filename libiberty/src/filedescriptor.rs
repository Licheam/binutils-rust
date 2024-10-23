use ::libc;
extern "C" {
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn is_valid_fd(mut fd: libc::c_int) -> libc::c_int {
    return (fcntl(fd, 1 as libc::c_int) >= 0 as libc::c_int) as libc::c_int;
}
