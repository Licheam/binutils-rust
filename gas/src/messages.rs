extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn unlink_if_ordinary(_: *const libc::c_char) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn vfprintf(
        _: *mut FILE,
        _: *const libc::c_char,
        _: ::core::ffi::VaList,
    ) -> libc::c_int;
    fn vsnprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ::core::ffi::VaList,
    ) -> libc::c_int;
    fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn strsignal(__sig: libc::c_int) -> *mut libc::c_char;
    fn xexit(status: libc::c_int) -> !;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    static mut flag_no_warnings: libc::c_int;
    static mut out_file_name: *const libc::c_char;
    fn listing_error(message: *const libc::c_char);
    fn as_where(_: *mut libc::c_uint) -> *const libc::c_char;
    fn listing_warning(message: *const libc::c_char);
    fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type size_t = libc::c_ulong;
pub type va_list = __builtin_va_list;
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
pub type bfd_vma = libc::c_ulong;
pub type bfd_signed_vma = libc::c_long;
pub type addressT = bfd_vma;
pub type offsetT = bfd_signed_vma;
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
#[no_mangle]
pub unsafe extern "C" fn as_bad(mut format: *const libc::c_char, mut args: ...) {
    let mut args_0: ::core::ffi::VaListImpl;
    let mut buffer: [libc::c_char; 2000] = [0; 2000];
    args_0 = args.clone();
    vsnprintf(
        buffer.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 2000]>() as libc::c_ulong,
        format,
        args_0.as_va_list(),
    );
    as_bad_internal(
        0 as *mut libc::c_void as *mut libc::c_char,
        0 as libc::c_int as libc::c_uint,
        buffer.as_mut_ptr(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn as_fatal(mut format: *const libc::c_char, mut args: ...) -> ! {
    let mut args_0: ::core::ffi::VaListImpl;
    as_show_where();
    args_0 = args.clone();
    fprintf(
        stderr,
        dcgettext(
            0 as *const libc::c_char,
            b"Fatal error: \0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    vfprintf(stderr, format, args_0.as_va_list());
    putc('\n' as i32, stderr);
    if !out_file_name.is_null() {
        unlink_if_ordinary(out_file_name);
    }
    xexit(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn as_tsktsk(mut format: *const libc::c_char, mut args: ...) {
    let mut args_0: ::core::ffi::VaListImpl;
    as_show_where();
    args_0 = args.clone();
    vfprintf(stderr, format, args_0.as_va_list());
    putc('\n' as i32, stderr);
}
#[no_mangle]
pub unsafe extern "C" fn as_warn(mut format: *const libc::c_char, mut args: ...) {
    let mut args_0: ::core::ffi::VaListImpl;
    let mut buffer: [libc::c_char; 2000] = [0; 2000];
    if flag_no_warnings == 0 {
        args_0 = args.clone();
        vsnprintf(
            buffer.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 2000]>() as libc::c_ulong,
            format,
            args_0.as_va_list(),
        );
        as_warn_internal(
            0 as *mut libc::c_void as *mut libc::c_char,
            0 as libc::c_int as libc::c_uint,
            buffer.as_mut_ptr(),
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn as_bad_where(
    mut file: *const libc::c_char,
    mut line: libc::c_uint,
    mut format: *const libc::c_char,
    mut args: ...
) {
    let mut args_0: ::core::ffi::VaListImpl;
    let mut buffer: [libc::c_char; 2000] = [0; 2000];
    args_0 = args.clone();
    vsnprintf(
        buffer.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 2000]>() as libc::c_ulong,
        format,
        args_0.as_va_list(),
    );
    as_bad_internal(file, line, buffer.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn as_warn_where(
    mut file: *const libc::c_char,
    mut line: libc::c_uint,
    mut format: *const libc::c_char,
    mut args: ...
) {
    let mut args_0: ::core::ffi::VaListImpl;
    let mut buffer: [libc::c_char; 2000] = [0; 2000];
    if flag_no_warnings == 0 {
        args_0 = args.clone();
        vsnprintf(
            buffer.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 2000]>() as libc::c_ulong,
            format,
            args_0.as_va_list(),
        );
        as_warn_internal(file, line, buffer.as_mut_ptr());
    }
}
#[no_mangle]
pub unsafe extern "C" fn as_abort(
    mut file: *const libc::c_char,
    mut line: libc::c_int,
    mut fn_0: *const libc::c_char,
) -> ! {
    as_show_where();
    if file.is_null() {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"Internal error (%s).\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            if !fn_0.is_null() {
                fn_0
            } else {
                b"unknown\0" as *const u8 as *const libc::c_char
            },
        );
    } else if !fn_0.is_null() {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"Internal error in %s at %s:%d.\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            fn_0,
            file,
            line,
        );
    } else {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"Internal error at %s:%d.\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            file,
            line,
        );
    }
    fprintf(
        stderr,
        dcgettext(
            0 as *const libc::c_char,
            b"Please report this bug.\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    xexit(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn signal_init() {
    signal(
        11 as libc::c_int,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(libc::c_int) -> !>,
            __sighandler_t,
        >(Some(signal_crash as unsafe extern "C" fn(libc::c_int) -> !)),
    );
    signal(
        4 as libc::c_int,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(libc::c_int) -> !>,
            __sighandler_t,
        >(Some(signal_crash as unsafe extern "C" fn(libc::c_int) -> !)),
    );
    signal(
        7 as libc::c_int,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(libc::c_int) -> !>,
            __sighandler_t,
        >(Some(signal_crash as unsafe extern "C" fn(libc::c_int) -> !)),
    );
    signal(
        6 as libc::c_int,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(libc::c_int) -> !>,
            __sighandler_t,
        >(Some(signal_crash as unsafe extern "C" fn(libc::c_int) -> !)),
    );
    signal(
        8 as libc::c_int,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(libc::c_int) -> !>,
            __sighandler_t,
        >(Some(signal_crash as unsafe extern "C" fn(libc::c_int) -> !)),
    );
}
#[no_mangle]
pub unsafe extern "C" fn had_errors() -> libc::c_int {
    return error_count;
}
#[no_mangle]
pub unsafe extern "C" fn had_warnings() -> libc::c_int {
    return warning_count;
}
#[no_mangle]
pub unsafe extern "C" fn as_warn_value_out_of_range(
    mut prefix: *const libc::c_char,
    mut value: offsetT,
    mut min: offsetT,
    mut max: offsetT,
    mut file: *const libc::c_char,
    mut line: libc::c_uint,
) {
    as_internal_value_out_of_range(
        prefix,
        value,
        min,
        max,
        file,
        line,
        0 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn as_bad_value_out_of_range(
    mut prefix: *const libc::c_char,
    mut value: offsetT,
    mut min: offsetT,
    mut max: offsetT,
    mut file: *const libc::c_char,
    mut line: libc::c_uint,
) {
    as_internal_value_out_of_range(
        prefix,
        value,
        min,
        max,
        file,
        line,
        1 as libc::c_int,
    );
}
unsafe extern "C" fn identify(mut file: *const libc::c_char) {
    static mut identified: libc::c_int = 0;
    if identified != 0 {
        return;
    }
    identified += 1;
    identified;
    if file.is_null() {
        let mut x: libc::c_uint = 0;
        file = as_where(&mut x);
    }
    if !file.is_null() {
        fprintf(stderr, b"%s: \0" as *const u8 as *const libc::c_char, file);
    }
    fprintf(
        stderr,
        dcgettext(
            0 as *const libc::c_char,
            b"Assembler messages:\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
}
unsafe extern "C" fn as_show_where() {
    let mut file: *const libc::c_char = 0 as *const libc::c_char;
    let mut line: libc::c_uint = 0;
    file = as_where(&mut line);
    identify(file);
    if !file.is_null() {
        if line != 0 as libc::c_int as libc::c_uint {
            fprintf(
                stderr,
                b"%s:%u: \0" as *const u8 as *const libc::c_char,
                file,
                line,
            );
        } else {
            fprintf(stderr, b"%s: \0" as *const u8 as *const libc::c_char, file);
        }
    }
}
unsafe extern "C" fn as_warn_internal(
    mut file: *const libc::c_char,
    mut line: libc::c_uint,
    mut buffer: *mut libc::c_char,
) {
    warning_count += 1;
    warning_count;
    if file.is_null() {
        file = as_where(&mut line);
    }
    identify(file);
    if !file.is_null() {
        if line != 0 as libc::c_int as libc::c_uint {
            fprintf(
                stderr,
                b"%s:%u: %s%s\n\0" as *const u8 as *const libc::c_char,
                file,
                line,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Warning: \0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                buffer,
            );
        } else {
            fprintf(
                stderr,
                b"%s: %s%s\n\0" as *const u8 as *const libc::c_char,
                file,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Warning: \0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                buffer,
            );
        }
    } else {
        fprintf(
            stderr,
            b"%s%s\n\0" as *const u8 as *const libc::c_char,
            dcgettext(
                0 as *const libc::c_char,
                b"Warning: \0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            buffer,
        );
    }
    listing_warning(buffer);
}
unsafe extern "C" fn as_bad_internal(
    mut file: *const libc::c_char,
    mut line: libc::c_uint,
    mut buffer: *mut libc::c_char,
) {
    error_count += 1;
    error_count;
    if file.is_null() {
        file = as_where(&mut line);
    }
    identify(file);
    if !file.is_null() {
        if line != 0 as libc::c_int as libc::c_uint {
            fprintf(
                stderr,
                b"%s:%u: %s%s\n\0" as *const u8 as *const libc::c_char,
                file,
                line,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Error: \0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                buffer,
            );
        } else {
            fprintf(
                stderr,
                b"%s: %s%s\n\0" as *const u8 as *const libc::c_char,
                file,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Error: \0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                buffer,
            );
        }
    } else {
        fprintf(
            stderr,
            b"%s%s\n\0" as *const u8 as *const libc::c_char,
            dcgettext(
                0 as *const libc::c_char,
                b"Error: \0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            buffer,
        );
    }
    listing_error(buffer);
}
unsafe extern "C" fn signal_crash(mut signo: libc::c_int) -> ! {
    signal(signo, None);
    as_abort(0 as *const libc::c_char, 0 as libc::c_int, strsignal(signo));
}
static mut warning_count: libc::c_int = 0;
static mut error_count: libc::c_int = 0;
unsafe extern "C" fn as_internal_value_out_of_range(
    mut prefix: *const libc::c_char,
    mut val: offsetT,
    mut min: offsetT,
    mut max: offsetT,
    mut file: *const libc::c_char,
    mut line: libc::c_uint,
    mut bad: libc::c_int,
) {
    let mut err: *const libc::c_char = 0 as *const libc::c_char;
    if prefix.is_null() {
        prefix = b"\0" as *const u8 as *const libc::c_char;
    }
    if val >= min && val <= max {
        let mut right: addressT = (max & -max) as addressT;
        if max <= 1 as libc::c_int as libc::c_long {
            as_abort(
                b"messages.c\0" as *const u8 as *const libc::c_char,
                381 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 110],
                    &[libc::c_char; 110],
                >(
                    b"void as_internal_value_out_of_range(const char *, offsetT, offsetT, offsetT, const char *, unsigned int, int)\0",
                ))
                    .as_ptr(),
            );
        }
        err = dcgettext(
            0 as *const libc::c_char,
            b"%s out of domain (%ld is not a multiple of %ld)\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        );
        if bad != 0 {
            as_bad_where(file, line, err, prefix, val, right);
        } else {
            as_warn_where(file, line, err, prefix, val, right);
        }
        return;
    }
    if val < 1024 as libc::c_int as libc::c_long
        && min < 1024 as libc::c_int as libc::c_long
        && max < 1024 as libc::c_int as libc::c_long
        && val > -(1024 as libc::c_int) as libc::c_long
        && min > -(1024 as libc::c_int) as libc::c_long
        && max > -(1024 as libc::c_int) as libc::c_long
    {
        err = dcgettext(
            0 as *const libc::c_char,
            b"%s out of range (%ld is not between %ld and %ld)\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        );
        if bad != 0 {
            as_bad_where(file, line, err, prefix, val, min, max);
        } else {
            as_warn_where(file, line, err, prefix, val, min, max);
        }
    } else {
        let mut val_buf: [libc::c_char; 26] = [0; 26];
        let mut min_buf: [libc::c_char; 26] = [0; 26];
        let mut max_buf: [libc::c_char; 26] = [0; 26];
        if ::core::mem::size_of::<offsetT>() as libc::c_ulong
            > ::core::mem::size_of::<bfd_vma>() as libc::c_ulong
        {
            as_abort(
                b"messages.c\0" as *const u8 as *const libc::c_char,
                416 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 110],
                    &[libc::c_char; 110],
                >(
                    b"void as_internal_value_out_of_range(const char *, offsetT, offsetT, offsetT, const char *, unsigned int, int)\0",
                ))
                    .as_ptr(),
            );
        }
        sprintf(
            val_buf.as_mut_ptr(),
            b"%016lx\0" as *const u8 as *const libc::c_char,
            val as bfd_vma,
        );
        sprintf(
            min_buf.as_mut_ptr(),
            b"%016lx\0" as *const u8 as *const libc::c_char,
            min as bfd_vma,
        );
        sprintf(
            max_buf.as_mut_ptr(),
            b"%016lx\0" as *const u8 as *const libc::c_char,
            max as bfd_vma,
        );
        err = dcgettext(
            0 as *const libc::c_char,
            b"%s out of range (0x%s is not between 0x%s and 0x%s)\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        );
        if bad != 0 {
            as_bad_where(
                file,
                line,
                err,
                prefix,
                val_buf.as_mut_ptr(),
                min_buf.as_mut_ptr(),
                max_buf.as_mut_ptr(),
            );
        } else {
            as_warn_where(
                file,
                line,
                err,
                prefix,
                val_buf.as_mut_ptr(),
                min_buf.as_mut_ptr(),
                max_buf.as_mut_ptr(),
            );
        }
    };
}
