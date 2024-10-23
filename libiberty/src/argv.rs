use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fseek(
        __stream: *mut FILE,
        __off: libc::c_long,
        __whence: libc::c_int,
    ) -> libc::c_int;
    fn ftell(__stream: *mut FILE) -> libc::c_long;
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    fn xexit(status: libc::c_int) -> !;
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    fn xrealloc(_: *mut libc::c_void, _: size_t) -> *mut libc::c_void;
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
    static _sch_istable: [libc::c_ushort; 256];
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn free(_: *mut libc::c_void);
    fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
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
pub const _sch_isspace: C2RustUnnamed = 64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
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
#[no_mangle]
pub unsafe extern "C" fn dupargv(
    mut argv: *const *mut libc::c_char,
) -> *mut *mut libc::c_char {
    let mut argc: libc::c_int = 0;
    let mut copy: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    if argv.is_null() {
        return 0 as *mut *mut libc::c_char;
    }
    argc = 0 as libc::c_int;
    while !(*argv.offset(argc as isize)).is_null() {
        argc += 1;
        argc;
    }
    copy = xmalloc(
        ((argc + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong),
    ) as *mut *mut libc::c_char;
    argc = 0 as libc::c_int;
    while !(*argv.offset(argc as isize)).is_null() {
        let ref mut fresh0 = *copy.offset(argc as isize);
        *fresh0 = xstrdup(*argv.offset(argc as isize));
        argc += 1;
        argc;
    }
    let ref mut fresh1 = *copy.offset(argc as isize);
    *fresh1 = 0 as *mut libc::c_char;
    return copy;
}
#[no_mangle]
pub unsafe extern "C" fn freeargv(mut vector: *mut *mut libc::c_char) {
    let mut scan: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    if !vector.is_null() {
        scan = vector;
        while !(*scan).is_null() {
            free(*scan as *mut libc::c_void);
            scan = scan.offset(1);
            scan;
        }
        free(vector as *mut libc::c_void);
    }
}
unsafe extern "C" fn consume_whitespace(mut input: *mut *const libc::c_char) {
    while _sch_istable[(**input as libc::c_int & 0xff as libc::c_int) as usize]
        as libc::c_int & _sch_isspace as libc::c_int as libc::c_ushort as libc::c_int
        != 0
    {
        *input = (*input).offset(1);
        *input;
    }
}
unsafe extern "C" fn only_whitespace(mut input: *const libc::c_char) -> libc::c_int {
    while *input as libc::c_int != '\0' as i32
        && _sch_istable[(*input as libc::c_int & 0xff as libc::c_int) as usize]
            as libc::c_int & _sch_isspace as libc::c_int as libc::c_ushort as libc::c_int
            != 0
    {
        input = input.offset(1);
        input;
    }
    return (*input as libc::c_int == '\0' as i32) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn buildargv(
    mut input: *const libc::c_char,
) -> *mut *mut libc::c_char {
    let mut arg: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut copybuf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut squote: libc::c_int = 0 as libc::c_int;
    let mut dquote: libc::c_int = 0 as libc::c_int;
    let mut bsquote: libc::c_int = 0 as libc::c_int;
    let mut argc: libc::c_int = 0 as libc::c_int;
    let mut maxargc: libc::c_int = 0 as libc::c_int;
    let mut argv: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut nargv: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    if !input.is_null() {
        copybuf = xmalloc(
            (strlen(input)).wrapping_add(1 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
        loop {
            consume_whitespace(&mut input);
            if maxargc == 0 as libc::c_int || argc >= maxargc - 1 as libc::c_int {
                if argv.is_null() {
                    maxargc = 8 as libc::c_int;
                    nargv = xmalloc(
                        (maxargc as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                            ),
                    ) as *mut *mut libc::c_char;
                } else {
                    maxargc *= 2 as libc::c_int;
                    nargv = xrealloc(
                        argv as *mut libc::c_void,
                        (maxargc as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                            ),
                    ) as *mut *mut libc::c_char;
                }
                argv = nargv;
                let ref mut fresh2 = *argv.offset(argc as isize);
                *fresh2 = 0 as *mut libc::c_char;
            }
            arg = copybuf;
            while *input as libc::c_int != '\0' as i32 {
                if _sch_istable[(*input as libc::c_int & 0xff as libc::c_int) as usize]
                    as libc::c_int
                    & _sch_isspace as libc::c_int as libc::c_ushort as libc::c_int != 0
                    && squote == 0 && dquote == 0 && bsquote == 0
                {
                    break;
                }
                if bsquote != 0 {
                    bsquote = 0 as libc::c_int;
                    let fresh3 = arg;
                    arg = arg.offset(1);
                    *fresh3 = *input;
                } else if *input as libc::c_int == '\\' as i32 {
                    bsquote = 1 as libc::c_int;
                } else if squote != 0 {
                    if *input as libc::c_int == '\'' as i32 {
                        squote = 0 as libc::c_int;
                    } else {
                        let fresh4 = arg;
                        arg = arg.offset(1);
                        *fresh4 = *input;
                    }
                } else if dquote != 0 {
                    if *input as libc::c_int == '"' as i32 {
                        dquote = 0 as libc::c_int;
                    } else {
                        let fresh5 = arg;
                        arg = arg.offset(1);
                        *fresh5 = *input;
                    }
                } else if *input as libc::c_int == '\'' as i32 {
                    squote = 1 as libc::c_int;
                } else if *input as libc::c_int == '"' as i32 {
                    dquote = 1 as libc::c_int;
                } else {
                    let fresh6 = arg;
                    arg = arg.offset(1);
                    *fresh6 = *input;
                }
                input = input.offset(1);
                input;
            }
            *arg = '\0' as i32 as libc::c_char;
            let ref mut fresh7 = *argv.offset(argc as isize);
            *fresh7 = xstrdup(copybuf);
            argc += 1;
            argc;
            let ref mut fresh8 = *argv.offset(argc as isize);
            *fresh8 = 0 as *mut libc::c_char;
            consume_whitespace(&mut input);
            if !(*input as libc::c_int != '\0' as i32) {
                break;
            }
        }
        free(copybuf as *mut libc::c_void);
    }
    return argv;
}
#[no_mangle]
pub unsafe extern "C" fn writeargv(
    mut argv: *const *mut libc::c_char,
    mut f: *mut FILE,
) -> libc::c_int {
    let mut status: libc::c_int = 0 as libc::c_int;
    if f.is_null() {
        return 1 as libc::c_int;
    }
    's_10: while !(*argv).is_null() {
        let mut arg: *const libc::c_char = *argv;
        while *arg as libc::c_int != '\0' as i32 {
            let mut c: libc::c_char = *arg;
            if _sch_istable[(c as libc::c_int & 0xff as libc::c_int) as usize]
                as libc::c_int
                & _sch_isspace as libc::c_int as libc::c_ushort as libc::c_int != 0
                || c as libc::c_int == '\\' as i32 || c as libc::c_int == '\'' as i32
                || c as libc::c_int == '"' as i32
            {
                if -(1 as libc::c_int) == fputc('\\' as i32, f) {
                    status = 1 as libc::c_int;
                    break 's_10;
                }
            }
            if -(1 as libc::c_int) == fputc(c as libc::c_int, f) {
                status = 1 as libc::c_int;
                break 's_10;
            } else {
                arg = arg.offset(1);
                arg;
            }
        }
        if arg == *argv as *const libc::c_char {
            if -(1 as libc::c_int)
                == fputs(b"\"\"\0" as *const u8 as *const libc::c_char, f)
            {
                status = 1 as libc::c_int;
                break;
            }
        }
        if -(1 as libc::c_int) == fputc('\n' as i32, f) {
            status = 1 as libc::c_int;
            break;
        } else {
            argv = argv.offset(1);
            argv;
        }
    }
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn expandargv(
    mut argcp: *mut libc::c_int,
    mut argvp: *mut *mut *mut libc::c_char,
) {
    let mut i: libc::c_int = 0 as libc::c_int;
    let original_argv: *mut *mut libc::c_char = *argvp;
    let mut iteration_limit: libc::c_uint = 2000 as libc::c_int as libc::c_uint;
    loop {
        i += 1;
        if !(i < *argcp) {
            break;
        }
        let mut filename: *const libc::c_char = 0 as *const libc::c_char;
        let mut f: *mut FILE = 0 as *mut FILE;
        let mut pos: libc::c_long = 0;
        let mut len: size_t = 0;
        let mut buffer: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut file_argv: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
        let mut file_argc: size_t = 0;
        let mut sb: stat = stat {
            st_dev: 0,
            st_ino: 0,
            st_nlink: 0,
            st_mode: 0,
            st_uid: 0,
            st_gid: 0,
            __pad0: 0,
            st_rdev: 0,
            st_size: 0,
            st_blksize: 0,
            st_blocks: 0,
            st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
            st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
            st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
            __glibc_reserved: [0; 3],
        };
        filename = *(*argvp).offset(i as isize);
        if *filename.offset(0 as libc::c_int as isize) as libc::c_int != '@' as i32 {
            continue;
        }
        iteration_limit = iteration_limit.wrapping_sub(1);
        if iteration_limit == 0 as libc::c_int as libc::c_uint {
            fprintf(
                stderr,
                b"%s: error: too many @-files encountered\n\0" as *const u8
                    as *const libc::c_char,
                *(*argvp).offset(0 as libc::c_int as isize),
            );
            xexit(1 as libc::c_int);
        }
        if stat(filename.offset(1 as libc::c_int as isize), &mut sb) < 0 as libc::c_int {
            continue;
        }
        if sb.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o40000 as libc::c_int as libc::c_uint
        {
            fprintf(
                stderr,
                b"%s: error: @-file refers to a directory\n\0" as *const u8
                    as *const libc::c_char,
                *(*argvp).offset(0 as libc::c_int as isize),
            );
            xexit(1 as libc::c_int);
        }
        filename = filename.offset(1);
        f = fopen(filename, b"r\0" as *const u8 as *const libc::c_char);
        if f.is_null() {
            continue;
        }
        if !(fseek(f, 0 as libc::c_long, 2 as libc::c_int) == -(1 as libc::c_int)) {
            pos = ftell(f);
            if !(pos == -(1 as libc::c_int) as libc::c_long) {
                if !(fseek(f, 0 as libc::c_long, 0 as libc::c_int)
                    == -(1 as libc::c_int))
                {
                    buffer = xmalloc(
                        (pos as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                            )
                            .wrapping_add(1 as libc::c_int as libc::c_ulong),
                    ) as *mut libc::c_char;
                    len = fread(
                        buffer as *mut libc::c_void,
                        ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                        pos as libc::c_ulong,
                        f,
                    );
                    if len != pos as size_t && ferror(f) != 0 {
                        free(buffer as *mut libc::c_void);
                    } else {
                        *buffer.offset(len as isize) = '\0' as i32 as libc::c_char;
                        if only_whitespace(buffer) != 0 {
                            file_argv = xmalloc(
                                ::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                            ) as *mut *mut libc::c_char;
                            let ref mut fresh9 = *file_argv
                                .offset(0 as libc::c_int as isize);
                            *fresh9 = 0 as *mut libc::c_char;
                        } else {
                            file_argv = buildargv(buffer);
                        }
                        if *argvp == original_argv {
                            *argvp = dupargv(*argvp);
                        }
                        file_argc = 0 as libc::c_int as size_t;
                        while !(*file_argv.offset(file_argc as isize)).is_null() {
                            file_argc = file_argc.wrapping_add(1);
                            file_argc;
                        }
                        free(*(*argvp).offset(i as isize) as *mut libc::c_void);
                        *argvp = xrealloc(
                            *argvp as *mut libc::c_void,
                            (*argcp as libc::c_ulong)
                                .wrapping_add(file_argc)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(
                                    ::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                                ),
                        ) as *mut *mut libc::c_char;
                        memmove(
                            (*argvp).offset(i as isize).offset(file_argc as isize)
                                as *mut libc::c_void,
                            (*argvp).offset(i as isize).offset(1 as libc::c_int as isize)
                                as *const libc::c_void,
                            ((*argcp - i) as libc::c_ulong)
                                .wrapping_mul(
                                    ::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                                ),
                        );
                        memcpy(
                            (*argvp).offset(i as isize) as *mut libc::c_void,
                            file_argv as *const libc::c_void,
                            file_argc
                                .wrapping_mul(
                                    ::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                                ),
                        );
                        *argcp = (*argcp as libc::c_ulong)
                            .wrapping_add(
                                file_argc.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                            ) as libc::c_int as libc::c_int;
                        free(file_argv as *mut libc::c_void);
                        free(buffer as *mut libc::c_void);
                        i -= 1;
                        i;
                    }
                }
            }
        }
        fclose(f);
    };
}
#[no_mangle]
pub unsafe extern "C" fn countargv(mut argv: *const *mut libc::c_char) -> libc::c_int {
    let mut argc: libc::c_int = 0;
    if argv.is_null() {
        return 0 as libc::c_int;
    }
    argc = 0 as libc::c_int;
    while !(*argv.offset(argc as isize)).is_null() {
        argc += 1;
        argc;
    }
    return argc;
}
