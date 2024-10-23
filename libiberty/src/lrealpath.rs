use ::libc;
extern "C" {
    fn canonicalize_file_name(__name: *const libc::c_char) -> *mut libc::c_char;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realpath(
        __name: *const libc::c_char,
        __resolved: *mut libc::c_char,
    ) -> *mut libc::c_char;
    fn free(_: *mut libc::c_void);
    fn pathconf(__path: *const libc::c_char, __name: libc::c_int) -> libc::c_long;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
}
pub const _PC_PATH_MAX: C2RustUnnamed = 4;
pub type C2RustUnnamed = libc::c_uint;
pub const _PC_2_SYMLINKS: C2RustUnnamed = 20;
pub const _PC_SYMLINK_MAX: C2RustUnnamed = 19;
pub const _PC_ALLOC_SIZE_MIN: C2RustUnnamed = 18;
pub const _PC_REC_XFER_ALIGN: C2RustUnnamed = 17;
pub const _PC_REC_MIN_XFER_SIZE: C2RustUnnamed = 16;
pub const _PC_REC_MAX_XFER_SIZE: C2RustUnnamed = 15;
pub const _PC_REC_INCR_XFER_SIZE: C2RustUnnamed = 14;
pub const _PC_FILESIZEBITS: C2RustUnnamed = 13;
pub const _PC_SOCK_MAXBUF: C2RustUnnamed = 12;
pub const _PC_PRIO_IO: C2RustUnnamed = 11;
pub const _PC_ASYNC_IO: C2RustUnnamed = 10;
pub const _PC_SYNC_IO: C2RustUnnamed = 9;
pub const _PC_VDISABLE: C2RustUnnamed = 8;
pub const _PC_NO_TRUNC: C2RustUnnamed = 7;
pub const _PC_CHOWN_RESTRICTED: C2RustUnnamed = 6;
pub const _PC_PIPE_BUF: C2RustUnnamed = 5;
pub const _PC_NAME_MAX: C2RustUnnamed = 3;
pub const _PC_MAX_INPUT: C2RustUnnamed = 2;
pub const _PC_MAX_CANON: C2RustUnnamed = 1;
pub const _PC_LINK_MAX: C2RustUnnamed = 0;
#[no_mangle]
pub unsafe extern "C" fn lrealpath(
    mut filename: *const libc::c_char,
) -> *mut libc::c_char {
    let mut buf: [libc::c_char; 4096] = [0; 4096];
    let mut rp: *const libc::c_char = realpath(filename, buf.as_mut_ptr());
    if rp.is_null() {
        rp = filename;
    }
    return strdup(rp);
}
