extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdin: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn getc(__stream: *mut FILE) -> libc::c_int;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn ungetc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn feof(__stream: *mut FILE) -> libc::c_int;
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn free(_: *mut libc::c_void);
    fn __errno_location() -> *mut libc::c_int;
    fn xstrerror(_: libc::c_int) -> *mut libc::c_char;
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn as_bad(format: *const libc::c_char, _: ...);
    fn as_warn(format: *const libc::c_char, _: ...);
    fn as_abort(_: *const libc::c_char, _: libc::c_int, _: *const libc::c_char) -> !;
    fn app_push() -> *mut libc::c_char;
    fn do_scrub_chars(
        get: Option::<unsafe extern "C" fn(*mut libc::c_char, size_t) -> size_t>,
        _: *mut libc::c_char,
        _: size_t,
    ) -> size_t;
    fn app_pop(_: *mut libc::c_char);
    static _sch_istable: [libc::c_ushort; 256];
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct saved_file {
    pub f_in: *mut FILE,
    pub file_name: *const libc::c_char,
    pub preprocess: libc::c_int,
    pub app_save: *mut libc::c_char,
}
pub const _sch_isspace: C2RustUnnamed = 64;
pub type C2RustUnnamed = libc::c_uint;
pub const _sch_isbasic: C2RustUnnamed = 3088;
pub const _sch_iscppsp: C2RustUnnamed = 3072;
pub const _sch_isgraph: C2RustUnnamed = 172;
pub const _sch_isidnum: C2RustUnnamed = 516;
pub const _sch_isalnum: C2RustUnnamed = 140;
pub const _sch_isalpha: C2RustUnnamed = 136;
pub const _sch_isnvsp: C2RustUnnamed = 2048;
pub const _sch_isvsp: C2RustUnnamed = 1024;
pub const _sch_isidst: C2RustUnnamed = 512;
pub const _sch_isxdigit: C2RustUnnamed = 256;
pub const _sch_isupper: C2RustUnnamed = 128;
pub const _sch_ispunct: C2RustUnnamed = 32;
pub const _sch_isprint: C2RustUnnamed = 16;
pub const _sch_islower: C2RustUnnamed = 8;
pub const _sch_isdigit: C2RustUnnamed = 4;
pub const _sch_iscntrl: C2RustUnnamed = 2;
pub const _sch_isblank: C2RustUnnamed = 1;
#[inline]
unsafe extern "C" fn startswith(
    mut str: *const libc::c_char,
    mut prefix: *const libc::c_char,
) -> bool {
    return strncmp(str, prefix, strlen(prefix)) == 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn input_file_give_next_buffer(
    mut where_0: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut return_value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut size: size_t = 0;
    if f_in.is_null() {
        return 0 as *mut libc::c_char;
    }
    if preprocess != 0 {
        size = do_scrub_chars(
            Some(
                input_file_get
                    as unsafe extern "C" fn(*mut libc::c_char, size_t) -> size_t,
            ),
            where_0,
            (32 as libc::c_int * 1024 as libc::c_int) as size_t,
        );
    } else {
        size = input_file_get(
            where_0,
            (32 as libc::c_int * 1024 as libc::c_int) as size_t,
        );
    }
    if size != 0 {
        return_value = where_0.offset(size as isize);
    } else {
        if fclose(f_in) != 0 {
            as_warn(
                dcgettext(
                    0 as *const libc::c_char,
                    b"can't close %s: %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                file_name,
                xstrerror(*__errno_location()),
            );
        }
        f_in = 0 as *mut FILE;
        return_value = 0 as *mut libc::c_char;
    }
    return return_value;
}
#[no_mangle]
pub unsafe extern "C" fn input_file_push() -> *mut libc::c_char {
    let mut saved: *mut saved_file = 0 as *mut saved_file;
    saved = xmalloc(::core::mem::size_of::<saved_file>() as libc::c_ulong)
        as *mut saved_file;
    (*saved).f_in = f_in;
    (*saved).file_name = file_name;
    (*saved).preprocess = preprocess;
    if preprocess != 0 {
        (*saved).app_save = app_push();
    }
    input_file_begin();
    return saved as *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn input_file_buffer_size() -> size_t {
    return (32 as libc::c_int * 1024 as libc::c_int) as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn input_file_begin() {
    f_in = 0 as *mut FILE;
}
#[no_mangle]
pub unsafe extern "C" fn input_file_close() {
    if !f_in.is_null() {
        fclose(f_in);
    }
    f_in = 0 as *mut FILE;
}
#[no_mangle]
pub unsafe extern "C" fn input_file_end() {}
#[no_mangle]
pub unsafe extern "C" fn input_file_open(
    mut filename: *const libc::c_char,
    mut pre: libc::c_int,
) {
    let mut c: libc::c_int = 0;
    let mut buf: [libc::c_char; 80] = [0; 80];
    preprocess = pre;
    if !filename.is_null() {} else {
        as_abort(
            b"input-file.c\0" as *const u8 as *const libc::c_char,
            125 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 40],
                &[libc::c_char; 40],
            >(b"void input_file_open(const char *, int)\0"))
                .as_ptr(),
        );
    };
    if *filename.offset(0 as libc::c_int as isize) != 0 {
        f_in = fopen(filename, b"r\0" as *const u8 as *const libc::c_char);
        file_name = filename;
    } else {
        f_in = stdin;
        file_name = dcgettext(
            0 as *const libc::c_char,
            b"{standard input}\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        );
    }
    if f_in.is_null() {
        as_bad(
            dcgettext(
                0 as *const libc::c_char,
                b"can't open %s for reading: %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            file_name,
            xstrerror(*__errno_location()),
        );
        return;
    }
    c = getc(f_in);
    if ferror(f_in) != 0 {
        as_bad(
            dcgettext(
                0 as *const libc::c_char,
                b"can't read from %s: %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            file_name,
            xstrerror(*__errno_location()),
        );
        fclose(f_in);
        f_in = 0 as *mut FILE;
        return;
    }
    if feof(f_in) != 0 {
        fclose(f_in);
        f_in = 0 as *mut FILE;
        return;
    }
    if c != -(1 as libc::c_int) {} else {
        as_abort(
            b"input-file.c\0" as *const u8 as *const libc::c_char,
            165 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 40],
                &[libc::c_char; 40],
            >(b"void input_file_open(const char *, int)\0"))
                .as_ptr(),
        );
    };
    if c == '#' as i32 {
        c = getc(f_in);
        if c == 'N' as i32 {
            if !(fgets(
                buf.as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_char; 80]>() as libc::c_ulong
                    as libc::c_int,
                f_in,
            ))
                .is_null()
                && startswith(
                    buf.as_mut_ptr(),
                    b"O_APP\0" as *const u8 as *const libc::c_char,
                ) as libc::c_int != 0
                && _sch_istable[(buf[5 as libc::c_int as usize] as libc::c_int
                    & 0xff as libc::c_int) as usize] as libc::c_int
                    & _sch_isspace as libc::c_int as libc::c_ushort as libc::c_int != 0
            {
                preprocess = 0 as libc::c_int;
            }
            if (strchr(buf.as_mut_ptr(), '\n' as i32)).is_null() {
                ungetc('#' as i32, f_in);
            } else {
                ungetc('\n' as i32, f_in);
            }
        } else if c == 'A' as i32 {
            if !(fgets(
                buf.as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_char; 80]>() as libc::c_ulong
                    as libc::c_int,
                f_in,
            ))
                .is_null()
                && startswith(
                    buf.as_mut_ptr(),
                    b"PP\0" as *const u8 as *const libc::c_char,
                ) as libc::c_int != 0
                && _sch_istable[(buf[2 as libc::c_int as usize] as libc::c_int
                    & 0xff as libc::c_int) as usize] as libc::c_int
                    & _sch_isspace as libc::c_int as libc::c_ushort as libc::c_int != 0
            {
                preprocess = 1 as libc::c_int;
            }
            if (strchr(buf.as_mut_ptr(), '\n' as i32)).is_null() {
                ungetc('#' as i32, f_in);
            } else {
                ungetc('\n' as i32, f_in);
            }
        } else if c == '\n' as i32 {
            ungetc('\n' as i32, f_in);
        } else {
            ungetc('#' as i32, f_in);
        }
    } else {
        ungetc(c, f_in);
    };
}
#[no_mangle]
pub unsafe extern "C" fn input_file_pop(mut arg: *mut libc::c_char) {
    let mut saved: *mut saved_file = arg as *mut saved_file;
    input_file_end();
    f_in = (*saved).f_in;
    file_name = (*saved).file_name;
    preprocess = (*saved).preprocess;
    if preprocess != 0 {
        app_pop((*saved).app_save);
    }
    free(arg as *mut libc::c_void);
}
#[no_mangle]
pub static mut preprocess: libc::c_int = 0 as libc::c_int;
static mut f_in: *mut FILE = 0 as *const FILE as *mut FILE;
static mut file_name: *const libc::c_char = 0 as *const libc::c_char;
unsafe extern "C" fn input_file_get(
    mut buf: *mut libc::c_char,
    mut buflen: size_t,
) -> size_t {
    let mut size: size_t = 0;
    if feof(f_in) != 0 {
        return 0 as libc::c_int as size_t;
    }
    size = fread(
        buf as *mut libc::c_void,
        ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
        buflen,
        f_in,
    );
    if ferror(f_in) != 0 {
        as_bad(
            dcgettext(
                0 as *const libc::c_char,
                b"can't read from %s: %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            file_name,
            xstrerror(*__errno_location()),
        );
    }
    return size;
}
