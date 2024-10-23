use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn freopen(
        __filename: *const libc::c_char,
        __modes: *const libc::c_char,
        __stream: *mut FILE,
    ) -> *mut FILE;
    fn fdopen(__fd: libc::c_int, __modes: *const libc::c_char) -> *mut FILE;
    fn __fsetlocking(__fp: *mut FILE, __type: libc::c_int) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type C2RustUnnamed = libc::c_uint;
pub const FSETLOCKING_BYCALLER: C2RustUnnamed = 2;
pub const FSETLOCKING_INTERNAL: C2RustUnnamed = 1;
pub const FSETLOCKING_QUERY: C2RustUnnamed = 0;
#[inline]
unsafe extern "C" fn unlock_1(fp: *mut FILE) {
    if !fp.is_null() {
        __fsetlocking(fp, FSETLOCKING_BYCALLER as libc::c_int);
    }
}
#[no_mangle]
pub unsafe extern "C" fn unlock_stream(mut fp: *mut FILE) {
    unlock_1(fp);
}
#[no_mangle]
pub unsafe extern "C" fn unlock_std_streams() {
    unlock_1(stdin);
    unlock_1(stdout);
    unlock_1(stderr);
}
#[no_mangle]
pub unsafe extern "C" fn fopen_unlocked(
    mut path: *const libc::c_char,
    mut mode: *const libc::c_char,
) -> *mut FILE {
    let fp: *mut FILE = fopen(path, mode);
    unlock_1(fp);
    return fp;
}
#[no_mangle]
pub unsafe extern "C" fn fdopen_unlocked(
    mut fildes: libc::c_int,
    mut mode: *const libc::c_char,
) -> *mut FILE {
    let fp: *mut FILE = fdopen(fildes, mode);
    unlock_1(fp);
    return fp;
}
#[no_mangle]
pub unsafe extern "C" fn freopen_unlocked(
    mut path: *const libc::c_char,
    mut mode: *const libc::c_char,
    mut stream: *mut FILE,
) -> *mut FILE {
    let fp: *mut FILE = freopen(path, mode, stream);
    unlock_1(fp);
    return fp;
}
