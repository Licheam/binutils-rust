use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn access(__name: *const libc::c_char, __type: libc::c_int) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn abort() -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn mkstemps(__template: *mut libc::c_char, __suffixlen: libc::c_int) -> libc::c_int;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn xmalloc(_: size_t) -> *mut libc::c_void;
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
#[inline]
unsafe extern "C" fn try_dir(
    mut dir: *const libc::c_char,
    mut base: *const libc::c_char,
) -> *const libc::c_char {
    if !base.is_null() {
        return base;
    }
    if !dir.is_null()
        && access(dir, 4 as libc::c_int | 2 as libc::c_int | 1 as libc::c_int)
            == 0 as libc::c_int
    {
        return dir;
    }
    return 0 as *const libc::c_char;
}
static mut tmp: [libc::c_char; 5] = [
    '/' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
];
static mut vartmp: [libc::c_char; 9] = [
    '/' as i32 as libc::c_char,
    'v' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    '/' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
];
static mut memoized_tmpdir: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub unsafe extern "C" fn choose_tmpdir() -> *const libc::c_char {
    if memoized_tmpdir.is_null() {
        let mut base: *const libc::c_char = 0 as *const libc::c_char;
        let mut tmpdir: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut len: libc::c_uint = 0;
        base = try_dir(getenv(b"TMPDIR\0" as *const u8 as *const libc::c_char), base);
        base = try_dir(getenv(b"TMP\0" as *const u8 as *const libc::c_char), base);
        base = try_dir(getenv(b"TEMP\0" as *const u8 as *const libc::c_char), base);
        if strcmp(
            b"/tmp\0" as *const u8 as *const libc::c_char,
            b"\\\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            base = try_dir(b"\\.\0" as *const u8 as *const libc::c_char, base);
        } else {
            base = try_dir(b"/tmp\0" as *const u8 as *const libc::c_char, base);
        }
        base = try_dir(vartmp.as_ptr(), base);
        base = try_dir(tmp.as_ptr(), base);
        if base.is_null() {
            base = b".\0" as *const u8 as *const libc::c_char;
        }
        len = strlen(base) as libc::c_uint;
        tmpdir = xmalloc(
            (::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
                .wrapping_mul(
                    len.wrapping_add(2 as libc::c_int as libc::c_uint) as libc::c_ulong,
                ),
        ) as *mut libc::c_char;
        strcpy(tmpdir, base);
        *tmpdir.offset(len as isize) = '/' as i32 as libc::c_char;
        *tmpdir
            .offset(
                len.wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
            ) = '\0' as i32 as libc::c_char;
        memoized_tmpdir = tmpdir;
    }
    return memoized_tmpdir;
}
#[no_mangle]
pub unsafe extern "C" fn make_temp_file_with_prefix(
    mut prefix: *const libc::c_char,
    mut suffix: *const libc::c_char,
) -> *mut libc::c_char {
    let mut base: *const libc::c_char = choose_tmpdir();
    let mut temp_filename: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut base_len: libc::c_int = 0;
    let mut suffix_len: libc::c_int = 0;
    let mut prefix_len: libc::c_int = 0;
    let mut fd: libc::c_int = 0;
    if prefix.is_null() {
        prefix = b"cc\0" as *const u8 as *const libc::c_char;
    }
    if suffix.is_null() {
        suffix = b"\0" as *const u8 as *const libc::c_char;
    }
    base_len = strlen(base) as libc::c_int;
    prefix_len = strlen(prefix) as libc::c_int;
    suffix_len = strlen(suffix) as libc::c_int;
    temp_filename = xmalloc(
        (::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
            .wrapping_mul(
                (base_len as libc::c_ulong)
                    .wrapping_add(
                        (::core::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    )
                    .wrapping_add(suffix_len as libc::c_ulong)
                    .wrapping_add(prefix_len as libc::c_ulong)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            ),
    ) as *mut libc::c_char;
    strcpy(temp_filename, base);
    strcpy(temp_filename.offset(base_len as isize), prefix);
    strcpy(
        temp_filename.offset(base_len as isize).offset(prefix_len as isize),
        b"XXXXXX\0" as *const u8 as *const libc::c_char,
    );
    strcpy(
        temp_filename
            .offset(base_len as isize)
            .offset(prefix_len as isize)
            .offset(
                (::core::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
            ),
        suffix,
    );
    fd = mkstemps(temp_filename, suffix_len);
    if fd == -(1 as libc::c_int) {
        fprintf(
            stderr,
            b"Cannot create temporary file in %s: %s\n\0" as *const u8
                as *const libc::c_char,
            base,
            strerror(*__errno_location()),
        );
        abort();
    }
    if close(fd) != 0 {
        abort();
    }
    return temp_filename;
}
#[no_mangle]
pub unsafe extern "C" fn make_temp_file(
    mut suffix: *const libc::c_char,
) -> *mut libc::c_char {
    return make_temp_file_with_prefix(0 as *const libc::c_char, suffix);
}
