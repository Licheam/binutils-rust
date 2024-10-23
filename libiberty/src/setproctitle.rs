use ::libc;
extern "C" {
    fn prctl(__option: libc::c_int, _: ...) -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn setproctitle(mut name: *const libc::c_char, mut args: ...) {
    prctl(15 as libc::c_int, name);
}
