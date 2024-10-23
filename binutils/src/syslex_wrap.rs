extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn getc(__stream: *mut FILE) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __errno_location() -> *mut libc::c_int;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn isatty(__fd: libc::c_int) -> libc::c_int;
    static mut yylval: YYSTYPE;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn clearerr(__stream: *mut FILE);
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    fn fileno(__stream: *mut FILE) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
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
pub type int16_t = __int16_t;
pub type uint8_t = __uint8_t;
pub type flex_uint8_t = uint8_t;
pub type flex_int16_t = int16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yy_buffer_state {
    pub yy_input_file: *mut FILE,
    pub yy_ch_buf: *mut libc::c_char,
    pub yy_buf_pos: *mut libc::c_char,
    pub yy_buf_size: libc::c_int,
    pub yy_n_chars: libc::c_int,
    pub yy_is_our_buffer: libc::c_int,
    pub yy_is_interactive: libc::c_int,
    pub yy_at_bol: libc::c_int,
    pub yy_bs_lineno: libc::c_int,
    pub yy_bs_column: libc::c_int,
    pub yy_fill_buffer: libc::c_int,
    pub yy_buffer_status: libc::c_int,
}
pub type YY_BUFFER_STATE = *mut yy_buffer_state;
pub type yy_size_t = size_t;
pub type YY_CHAR = flex_uint8_t;
pub type yy_state_type = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub union YYSTYPE {
    pub i: libc::c_int,
    pub s: *mut libc::c_char,
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
#[no_mangle]
pub static mut yyleng: libc::c_int = 0;
#[no_mangle]
pub static mut yyin: *mut FILE = 0 as *const FILE as *mut FILE;
#[no_mangle]
pub static mut yyout: *mut FILE = 0 as *const FILE as *mut FILE;
static mut yy_buffer_stack_top: size_t = 0 as libc::c_int as size_t;
static mut yy_buffer_stack_max: size_t = 0 as libc::c_int as size_t;
static mut yy_buffer_stack: *mut YY_BUFFER_STATE = 0 as *const YY_BUFFER_STATE
    as *mut YY_BUFFER_STATE;
static mut yy_hold_char: libc::c_char = 0;
static mut yy_n_chars: libc::c_int = 0;
static mut yy_c_buf_p: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut yy_init: libc::c_int = 0 as libc::c_int;
static mut yy_start: libc::c_int = 0 as libc::c_int;
static mut yy_did_buffer_switch_on_eof: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn yyrestart(mut input_file: *mut FILE) {
    if if !yy_buffer_stack.is_null() {
        *yy_buffer_stack.offset(yy_buffer_stack_top as isize)
    } else {
        0 as YY_BUFFER_STATE
    }
        .is_null()
    {
        yyensure_buffer_stack();
        let ref mut fresh0 = *yy_buffer_stack.offset(yy_buffer_stack_top as isize);
        *fresh0 = yy_create_buffer(yyin, 16384 as libc::c_int);
    }
    yy_init_buffer(
        if !yy_buffer_stack.is_null() {
            *yy_buffer_stack.offset(yy_buffer_stack_top as isize)
        } else {
            0 as YY_BUFFER_STATE
        },
        input_file,
    );
    yy_load_buffer_state();
}
unsafe extern "C" fn yy_load_buffer_state() {
    yy_n_chars = (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_n_chars;
    yy_c_buf_p = (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_buf_pos;
    yytext = yy_c_buf_p;
    yyin = (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_input_file;
    yy_hold_char = *yy_c_buf_p;
}
#[no_mangle]
pub static mut yytext: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
unsafe extern "C" fn yy_init_buffer(mut b: YY_BUFFER_STATE, mut file: *mut FILE) {
    let mut oerrno: libc::c_int = *__errno_location();
    yy_flush_buffer(b);
    (*b).yy_input_file = file;
    (*b).yy_fill_buffer = 1 as libc::c_int;
    if b
        != (if !yy_buffer_stack.is_null() {
            *yy_buffer_stack.offset(yy_buffer_stack_top as isize)
        } else {
            0 as YY_BUFFER_STATE
        })
    {
        (*b).yy_bs_lineno = 1 as libc::c_int;
        (*b).yy_bs_column = 0 as libc::c_int;
    }
    (*b)
        .yy_is_interactive = if !file.is_null() {
        (isatty(fileno(file)) > 0 as libc::c_int) as libc::c_int
    } else {
        0 as libc::c_int
    };
    *__errno_location() = oerrno;
}
#[no_mangle]
pub unsafe extern "C" fn yy_flush_buffer(mut b: YY_BUFFER_STATE) {
    if b.is_null() {
        return;
    }
    (*b).yy_n_chars = 0 as libc::c_int;
    *((*b).yy_ch_buf)
        .offset(0 as libc::c_int as isize) = 0 as libc::c_int as libc::c_char;
    *((*b).yy_ch_buf)
        .offset(1 as libc::c_int as isize) = 0 as libc::c_int as libc::c_char;
    (*b)
        .yy_buf_pos = &mut *((*b).yy_ch_buf).offset(0 as libc::c_int as isize)
        as *mut libc::c_char;
    (*b).yy_at_bol = 1 as libc::c_int;
    (*b).yy_buffer_status = 0 as libc::c_int;
    if b
        == (if !yy_buffer_stack.is_null() {
            *yy_buffer_stack.offset(yy_buffer_stack_top as isize)
        } else {
            0 as YY_BUFFER_STATE
        })
    {
        yy_load_buffer_state();
    }
}
#[no_mangle]
pub unsafe extern "C" fn yy_create_buffer(
    mut file: *mut FILE,
    mut size: libc::c_int,
) -> YY_BUFFER_STATE {
    let mut b: YY_BUFFER_STATE = 0 as *mut yy_buffer_state;
    b = yyalloc(::core::mem::size_of::<yy_buffer_state>() as libc::c_ulong)
        as YY_BUFFER_STATE;
    if b.is_null() {
        yy_fatal_error(
            b"out of dynamic memory in yy_create_buffer()\0" as *const u8
                as *const libc::c_char,
        );
    }
    (*b).yy_buf_size = size;
    (*b)
        .yy_ch_buf = yyalloc(((*b).yy_buf_size + 2 as libc::c_int) as yy_size_t)
        as *mut libc::c_char;
    if ((*b).yy_ch_buf).is_null() {
        yy_fatal_error(
            b"out of dynamic memory in yy_create_buffer()\0" as *const u8
                as *const libc::c_char,
        );
    }
    (*b).yy_is_our_buffer = 1 as libc::c_int;
    yy_init_buffer(b, file);
    return b;
}
unsafe extern "C" fn yy_fatal_error(mut msg: *const libc::c_char) -> ! {
    fprintf(stderr, b"%s\n\0" as *const u8 as *const libc::c_char, msg);
    exit(2 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn yyalloc(mut size: yy_size_t) -> *mut libc::c_void {
    return malloc(size);
}
unsafe extern "C" fn yyensure_buffer_stack() {
    let mut num_to_alloc: yy_size_t = 0;
    if yy_buffer_stack.is_null() {
        num_to_alloc = 1 as libc::c_int as yy_size_t;
        yy_buffer_stack = yyalloc(
            num_to_alloc
                .wrapping_mul(
                    ::core::mem::size_of::<*mut yy_buffer_state>() as libc::c_ulong,
                ),
        ) as *mut *mut yy_buffer_state;
        if yy_buffer_stack.is_null() {
            yy_fatal_error(
                b"out of dynamic memory in yyensure_buffer_stack()\0" as *const u8
                    as *const libc::c_char,
            );
        }
        memset(
            yy_buffer_stack as *mut libc::c_void,
            0 as libc::c_int,
            num_to_alloc
                .wrapping_mul(
                    ::core::mem::size_of::<*mut yy_buffer_state>() as libc::c_ulong,
                ),
        );
        yy_buffer_stack_max = num_to_alloc;
        yy_buffer_stack_top = 0 as libc::c_int as size_t;
        return;
    }
    if yy_buffer_stack_top
        >= yy_buffer_stack_max.wrapping_sub(1 as libc::c_int as libc::c_ulong)
    {
        let mut grow_size: yy_size_t = 8 as libc::c_int as yy_size_t;
        num_to_alloc = yy_buffer_stack_max.wrapping_add(grow_size);
        yy_buffer_stack = yyrealloc(
            yy_buffer_stack as *mut libc::c_void,
            num_to_alloc
                .wrapping_mul(
                    ::core::mem::size_of::<*mut yy_buffer_state>() as libc::c_ulong,
                ),
        ) as *mut *mut yy_buffer_state;
        if yy_buffer_stack.is_null() {
            yy_fatal_error(
                b"out of dynamic memory in yyensure_buffer_stack()\0" as *const u8
                    as *const libc::c_char,
            );
        }
        memset(
            yy_buffer_stack.offset(yy_buffer_stack_max as isize) as *mut libc::c_void,
            0 as libc::c_int,
            grow_size
                .wrapping_mul(
                    ::core::mem::size_of::<*mut yy_buffer_state>() as libc::c_ulong,
                ),
        );
        yy_buffer_stack_max = num_to_alloc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn yyrealloc(
    mut ptr: *mut libc::c_void,
    mut size: yy_size_t,
) -> *mut libc::c_void {
    return realloc(ptr, size);
}
#[no_mangle]
pub unsafe extern "C" fn yy_switch_to_buffer(mut new_buffer: YY_BUFFER_STATE) {
    yyensure_buffer_stack();
    if (if !yy_buffer_stack.is_null() {
        *yy_buffer_stack.offset(yy_buffer_stack_top as isize)
    } else {
        0 as YY_BUFFER_STATE
    }) == new_buffer
    {
        return;
    }
    if !if !yy_buffer_stack.is_null() {
        *yy_buffer_stack.offset(yy_buffer_stack_top as isize)
    } else {
        0 as YY_BUFFER_STATE
    }
        .is_null()
    {
        *yy_c_buf_p = yy_hold_char;
        let ref mut fresh1 = (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
            .yy_buf_pos;
        *fresh1 = yy_c_buf_p;
        (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_n_chars = yy_n_chars;
    }
    let ref mut fresh2 = *yy_buffer_stack.offset(yy_buffer_stack_top as isize);
    *fresh2 = new_buffer;
    yy_load_buffer_state();
    yy_did_buffer_switch_on_eof = 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn yy_delete_buffer(mut b: YY_BUFFER_STATE) {
    if b.is_null() {
        return;
    }
    if b
        == (if !yy_buffer_stack.is_null() {
            *yy_buffer_stack.offset(yy_buffer_stack_top as isize)
        } else {
            0 as YY_BUFFER_STATE
        })
    {
        let ref mut fresh3 = *yy_buffer_stack.offset(yy_buffer_stack_top as isize);
        *fresh3 = 0 as YY_BUFFER_STATE;
    }
    if (*b).yy_is_our_buffer != 0 {
        yyfree((*b).yy_ch_buf as *mut libc::c_void);
    }
    yyfree(b as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn yyfree(mut ptr: *mut libc::c_void) {
    free(ptr as *mut libc::c_char as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn yypush_buffer_state(mut new_buffer: YY_BUFFER_STATE) {
    if new_buffer.is_null() {
        return;
    }
    yyensure_buffer_stack();
    if !if !yy_buffer_stack.is_null() {
        *yy_buffer_stack.offset(yy_buffer_stack_top as isize)
    } else {
        0 as YY_BUFFER_STATE
    }
        .is_null()
    {
        *yy_c_buf_p = yy_hold_char;
        let ref mut fresh4 = (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
            .yy_buf_pos;
        *fresh4 = yy_c_buf_p;
        (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_n_chars = yy_n_chars;
    }
    if !if !yy_buffer_stack.is_null() {
        *yy_buffer_stack.offset(yy_buffer_stack_top as isize)
    } else {
        0 as YY_BUFFER_STATE
    }
        .is_null()
    {
        yy_buffer_stack_top = yy_buffer_stack_top.wrapping_add(1);
        yy_buffer_stack_top;
    }
    let ref mut fresh5 = *yy_buffer_stack.offset(yy_buffer_stack_top as isize);
    *fresh5 = new_buffer;
    yy_load_buffer_state();
    yy_did_buffer_switch_on_eof = 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn yypop_buffer_state() {
    if if !yy_buffer_stack.is_null() {
        *yy_buffer_stack.offset(yy_buffer_stack_top as isize)
    } else {
        0 as YY_BUFFER_STATE
    }
        .is_null()
    {
        return;
    }
    yy_delete_buffer(
        if !yy_buffer_stack.is_null() {
            *yy_buffer_stack.offset(yy_buffer_stack_top as isize)
        } else {
            0 as YY_BUFFER_STATE
        },
    );
    let ref mut fresh6 = *yy_buffer_stack.offset(yy_buffer_stack_top as isize);
    *fresh6 = 0 as YY_BUFFER_STATE;
    if yy_buffer_stack_top > 0 as libc::c_int as libc::c_ulong {
        yy_buffer_stack_top = yy_buffer_stack_top.wrapping_sub(1);
        yy_buffer_stack_top;
    }
    if !if !yy_buffer_stack.is_null() {
        *yy_buffer_stack.offset(yy_buffer_stack_top as isize)
    } else {
        0 as YY_BUFFER_STATE
    }
        .is_null()
    {
        yy_load_buffer_state();
        yy_did_buffer_switch_on_eof = 1 as libc::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn yy_scan_buffer(
    mut base: *mut libc::c_char,
    mut size: yy_size_t,
) -> YY_BUFFER_STATE {
    let mut b: YY_BUFFER_STATE = 0 as *mut yy_buffer_state;
    if size < 2 as libc::c_int as libc::c_ulong
        || *base.offset(size.wrapping_sub(2 as libc::c_int as libc::c_ulong) as isize)
            as libc::c_int != 0 as libc::c_int
        || *base.offset(size.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
            as libc::c_int != 0 as libc::c_int
    {
        return 0 as YY_BUFFER_STATE;
    }
    b = yyalloc(::core::mem::size_of::<yy_buffer_state>() as libc::c_ulong)
        as YY_BUFFER_STATE;
    if b.is_null() {
        yy_fatal_error(
            b"out of dynamic memory in yy_scan_buffer()\0" as *const u8
                as *const libc::c_char,
        );
    }
    (*b)
        .yy_buf_size = size.wrapping_sub(2 as libc::c_int as libc::c_ulong)
        as libc::c_int;
    (*b).yy_ch_buf = base;
    (*b).yy_buf_pos = (*b).yy_ch_buf;
    (*b).yy_is_our_buffer = 0 as libc::c_int;
    (*b).yy_input_file = 0 as *mut FILE;
    (*b).yy_n_chars = (*b).yy_buf_size;
    (*b).yy_is_interactive = 0 as libc::c_int;
    (*b).yy_at_bol = 1 as libc::c_int;
    (*b).yy_fill_buffer = 0 as libc::c_int;
    (*b).yy_buffer_status = 0 as libc::c_int;
    yy_switch_to_buffer(b);
    return b;
}
#[no_mangle]
pub unsafe extern "C" fn yy_scan_string(
    mut yystr: *const libc::c_char,
) -> YY_BUFFER_STATE {
    return yy_scan_bytes(yystr, strlen(yystr) as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn yy_scan_bytes(
    mut yybytes: *const libc::c_char,
    mut _yybytes_len: libc::c_int,
) -> YY_BUFFER_STATE {
    let mut b: YY_BUFFER_STATE = 0 as *mut yy_buffer_state;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n: yy_size_t = 0;
    let mut i: libc::c_int = 0;
    n = (_yybytes_len + 2 as libc::c_int) as yy_size_t;
    buf = yyalloc(n) as *mut libc::c_char;
    if buf.is_null() {
        yy_fatal_error(
            b"out of dynamic memory in yy_scan_bytes()\0" as *const u8
                as *const libc::c_char,
        );
    }
    i = 0 as libc::c_int;
    while i < _yybytes_len {
        *buf.offset(i as isize) = *yybytes.offset(i as isize);
        i += 1;
        i;
    }
    let ref mut fresh7 = *buf.offset((_yybytes_len + 1 as libc::c_int) as isize);
    *fresh7 = 0 as libc::c_int as libc::c_char;
    *buf.offset(_yybytes_len as isize) = *fresh7;
    b = yy_scan_buffer(buf, n);
    if b.is_null() {
        yy_fatal_error(
            b"bad buffer in yy_scan_bytes()\0" as *const u8 as *const libc::c_char,
        );
    }
    (*b).yy_is_our_buffer = 1 as libc::c_int;
    return b;
}
#[no_mangle]
pub static mut yylineno: libc::c_int = 1 as libc::c_int;
unsafe extern "C" fn yy_get_previous_state() -> yy_state_type {
    let mut yy_current_state: yy_state_type = 0;
    let mut yy_cp: *mut libc::c_char = 0 as *mut libc::c_char;
    yy_current_state = yy_start;
    yy_cp = yytext.offset(0 as libc::c_int as isize);
    while yy_cp < yy_c_buf_p {
        let mut yy_c: YY_CHAR = (if *yy_cp as libc::c_int != 0 {
            yy_ec[*yy_cp as YY_CHAR as usize] as libc::c_int
        } else {
            1 as libc::c_int
        }) as YY_CHAR;
        if yy_accept[yy_current_state as usize] != 0 {
            yy_last_accepting_state = yy_current_state;
            yy_last_accepting_cpos = yy_cp;
        }
        while yy_chk[(yy_base[yy_current_state as usize] as libc::c_int
            + yy_c as libc::c_int) as usize] as libc::c_int != yy_current_state
        {
            yy_current_state = yy_def[yy_current_state as usize] as libc::c_int;
            if yy_current_state >= 81 as libc::c_int {
                yy_c = yy_meta[yy_c as usize];
            }
        }
        yy_current_state = yy_nxt[(yy_base[yy_current_state as usize] as libc::c_int
            + yy_c as libc::c_int) as usize] as yy_state_type;
        yy_cp = yy_cp.offset(1);
        yy_cp;
    }
    return yy_current_state;
}
static mut yy_ec: [YY_CHAR; 256] = [
    0 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    2 as libc::c_int as YY_CHAR,
    3 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    4 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    5 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    6 as libc::c_int as YY_CHAR,
    7 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    8 as libc::c_int as YY_CHAR,
    9 as libc::c_int as YY_CHAR,
    9 as libc::c_int as YY_CHAR,
    9 as libc::c_int as YY_CHAR,
    9 as libc::c_int as YY_CHAR,
    9 as libc::c_int as YY_CHAR,
    9 as libc::c_int as YY_CHAR,
    9 as libc::c_int as YY_CHAR,
    9 as libc::c_int as YY_CHAR,
    9 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    10 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    11 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    12 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    13 as libc::c_int as YY_CHAR,
    14 as libc::c_int as YY_CHAR,
    15 as libc::c_int as YY_CHAR,
    16 as libc::c_int as YY_CHAR,
    17 as libc::c_int as YY_CHAR,
    18 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    20 as libc::c_int as YY_CHAR,
    21 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    22 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    23 as libc::c_int as YY_CHAR,
    24 as libc::c_int as YY_CHAR,
    25 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    26 as libc::c_int as YY_CHAR,
    27 as libc::c_int as YY_CHAR,
    28 as libc::c_int as YY_CHAR,
    29 as libc::c_int as YY_CHAR,
    30 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    31 as libc::c_int as YY_CHAR,
    32 as libc::c_int as YY_CHAR,
    33 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
];
static mut yy_base: [flex_int16_t; 84] = [
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    100 as libc::c_int as flex_int16_t,
    101 as libc::c_int as flex_int16_t,
    101 as libc::c_int as flex_int16_t,
    101 as libc::c_int as flex_int16_t,
    101 as libc::c_int as flex_int16_t,
    94 as libc::c_int as flex_int16_t,
    101 as libc::c_int as flex_int16_t,
    101 as libc::c_int as flex_int16_t,
    26 as libc::c_int as flex_int16_t,
    28 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    101 as libc::c_int as flex_int16_t,
    101 as libc::c_int as flex_int16_t,
    82 as libc::c_int as flex_int16_t,
    26 as libc::c_int as flex_int16_t,
    18 as libc::c_int as flex_int16_t,
    74 as libc::c_int as flex_int16_t,
    79 as libc::c_int as flex_int16_t,
    78 as libc::c_int as flex_int16_t,
    81 as libc::c_int as flex_int16_t,
    88 as libc::c_int as flex_int16_t,
    101 as libc::c_int as flex_int16_t,
    32 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    76 as libc::c_int as flex_int16_t,
    65 as libc::c_int as flex_int16_t,
    62 as libc::c_int as flex_int16_t,
    61 as libc::c_int as flex_int16_t,
    75 as libc::c_int as flex_int16_t,
    20 as libc::c_int as flex_int16_t,
    59 as libc::c_int as flex_int16_t,
    61 as libc::c_int as flex_int16_t,
    66 as libc::c_int as flex_int16_t,
    58 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    57 as libc::c_int as flex_int16_t,
    56 as libc::c_int as flex_int16_t,
    54 as libc::c_int as flex_int16_t,
    63 as libc::c_int as flex_int16_t,
    53 as libc::c_int as flex_int16_t,
    62 as libc::c_int as flex_int16_t,
    54 as libc::c_int as flex_int16_t,
    101 as libc::c_int as flex_int16_t,
    59 as libc::c_int as flex_int16_t,
    48 as libc::c_int as flex_int16_t,
    53 as libc::c_int as flex_int16_t,
    46 as libc::c_int as flex_int16_t,
    59 as libc::c_int as flex_int16_t,
    101 as libc::c_int as flex_int16_t,
    44 as libc::c_int as flex_int16_t,
    43 as libc::c_int as flex_int16_t,
    101 as libc::c_int as flex_int16_t,
    41 as libc::c_int as flex_int16_t,
    55 as libc::c_int as flex_int16_t,
    46 as libc::c_int as flex_int16_t,
    53 as libc::c_int as flex_int16_t,
    44 as libc::c_int as flex_int16_t,
    31 as libc::c_int as flex_int16_t,
    101 as libc::c_int as flex_int16_t,
    101 as libc::c_int as flex_int16_t,
    39 as libc::c_int as flex_int16_t,
    27 as libc::c_int as flex_int16_t,
    21 as libc::c_int as flex_int16_t,
    39 as libc::c_int as flex_int16_t,
    19 as libc::c_int as flex_int16_t,
    101 as libc::c_int as flex_int16_t,
    35 as libc::c_int as flex_int16_t,
    101 as libc::c_int as flex_int16_t,
    33 as libc::c_int as flex_int16_t,
    26 as libc::c_int as flex_int16_t,
    29 as libc::c_int as flex_int16_t,
    101 as libc::c_int as flex_int16_t,
    101 as libc::c_int as flex_int16_t,
    28 as libc::c_int as flex_int16_t,
    101 as libc::c_int as flex_int16_t,
    101 as libc::c_int as flex_int16_t,
    101 as libc::c_int as flex_int16_t,
    58 as libc::c_int as flex_int16_t,
    61 as libc::c_int as flex_int16_t,
    41 as libc::c_int as flex_int16_t,
];
static mut yy_nxt: [flex_int16_t; 135] = [
    0 as libc::c_int as flex_int16_t,
    4 as libc::c_int as flex_int16_t,
    5 as libc::c_int as flex_int16_t,
    6 as libc::c_int as flex_int16_t,
    7 as libc::c_int as flex_int16_t,
    8 as libc::c_int as flex_int16_t,
    9 as libc::c_int as flex_int16_t,
    10 as libc::c_int as flex_int16_t,
    11 as libc::c_int as flex_int16_t,
    12 as libc::c_int as flex_int16_t,
    13 as libc::c_int as flex_int16_t,
    14 as libc::c_int as flex_int16_t,
    15 as libc::c_int as flex_int16_t,
    16 as libc::c_int as flex_int16_t,
    17 as libc::c_int as flex_int16_t,
    18 as libc::c_int as flex_int16_t,
    4 as libc::c_int as flex_int16_t,
    4 as libc::c_int as flex_int16_t,
    4 as libc::c_int as flex_int16_t,
    4 as libc::c_int as flex_int16_t,
    4 as libc::c_int as flex_int16_t,
    19 as libc::c_int as flex_int16_t,
    4 as libc::c_int as flex_int16_t,
    4 as libc::c_int as flex_int16_t,
    4 as libc::c_int as flex_int16_t,
    4 as libc::c_int as flex_int16_t,
    20 as libc::c_int as flex_int16_t,
    21 as libc::c_int as flex_int16_t,
    4 as libc::c_int as flex_int16_t,
    4 as libc::c_int as flex_int16_t,
    22 as libc::c_int as flex_int16_t,
    4 as libc::c_int as flex_int16_t,
    4 as libc::c_int as flex_int16_t,
    4 as libc::c_int as flex_int16_t,
    25 as libc::c_int as flex_int16_t,
    25 as libc::c_int as flex_int16_t,
    25 as libc::c_int as flex_int16_t,
    25 as libc::c_int as flex_int16_t,
    32 as libc::c_int as flex_int16_t,
    29 as libc::c_int as flex_int16_t,
    25 as libc::c_int as flex_int16_t,
    25 as libc::c_int as flex_int16_t,
    33 as libc::c_int as flex_int16_t,
    44 as libc::c_int as flex_int16_t,
    38 as libc::c_int as flex_int16_t,
    79 as libc::c_int as flex_int16_t,
    78 as libc::c_int as flex_int16_t,
    30 as libc::c_int as flex_int16_t,
    77 as libc::c_int as flex_int16_t,
    45 as libc::c_int as flex_int16_t,
    76 as libc::c_int as flex_int16_t,
    75 as libc::c_int as flex_int16_t,
    74 as libc::c_int as flex_int16_t,
    73 as libc::c_int as flex_int16_t,
    72 as libc::c_int as flex_int16_t,
    71 as libc::c_int as flex_int16_t,
    70 as libc::c_int as flex_int16_t,
    26 as libc::c_int as flex_int16_t,
    31 as libc::c_int as flex_int16_t,
    23 as libc::c_int as flex_int16_t,
    23 as libc::c_int as flex_int16_t,
    23 as libc::c_int as flex_int16_t,
    27 as libc::c_int as flex_int16_t,
    69 as libc::c_int as flex_int16_t,
    27 as libc::c_int as flex_int16_t,
    68 as libc::c_int as flex_int16_t,
    67 as libc::c_int as flex_int16_t,
    66 as libc::c_int as flex_int16_t,
    65 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    63 as libc::c_int as flex_int16_t,
    62 as libc::c_int as flex_int16_t,
    61 as libc::c_int as flex_int16_t,
    60 as libc::c_int as flex_int16_t,
    59 as libc::c_int as flex_int16_t,
    58 as libc::c_int as flex_int16_t,
    57 as libc::c_int as flex_int16_t,
    56 as libc::c_int as flex_int16_t,
    55 as libc::c_int as flex_int16_t,
    54 as libc::c_int as flex_int16_t,
    53 as libc::c_int as flex_int16_t,
    52 as libc::c_int as flex_int16_t,
    51 as libc::c_int as flex_int16_t,
    50 as libc::c_int as flex_int16_t,
    49 as libc::c_int as flex_int16_t,
    48 as libc::c_int as flex_int16_t,
    47 as libc::c_int as flex_int16_t,
    46 as libc::c_int as flex_int16_t,
    43 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    41 as libc::c_int as flex_int16_t,
    40 as libc::c_int as flex_int16_t,
    39 as libc::c_int as flex_int16_t,
    24 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    36 as libc::c_int as flex_int16_t,
    35 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    28 as libc::c_int as flex_int16_t,
    24 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    3 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
];
static mut yy_meta: [YY_CHAR; 34] = [
    0 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    2 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    3 as libc::c_int as YY_CHAR,
    3 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    3 as libc::c_int as YY_CHAR,
    3 as libc::c_int as YY_CHAR,
    3 as libc::c_int as YY_CHAR,
    3 as libc::c_int as YY_CHAR,
    3 as libc::c_int as YY_CHAR,
    3 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
];
static mut yy_def: [flex_int16_t; 84] = [
    0 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    81 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    82 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    81 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    83 as libc::c_int as flex_int16_t,
    82 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    83 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
];
static mut yy_chk: [flex_int16_t; 135] = [
    0 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    11 as libc::c_int as flex_int16_t,
    11 as libc::c_int as flex_int16_t,
    12 as libc::c_int as flex_int16_t,
    12 as libc::c_int as flex_int16_t,
    18 as libc::c_int as flex_int16_t,
    17 as libc::c_int as flex_int16_t,
    25 as libc::c_int as flex_int16_t,
    25 as libc::c_int as flex_int16_t,
    18 as libc::c_int as flex_int16_t,
    33 as libc::c_int as flex_int16_t,
    83 as libc::c_int as flex_int16_t,
    77 as libc::c_int as flex_int16_t,
    74 as libc::c_int as flex_int16_t,
    17 as libc::c_int as flex_int16_t,
    73 as libc::c_int as flex_int16_t,
    33 as libc::c_int as flex_int16_t,
    72 as libc::c_int as flex_int16_t,
    70 as libc::c_int as flex_int16_t,
    68 as libc::c_int as flex_int16_t,
    67 as libc::c_int as flex_int16_t,
    66 as libc::c_int as flex_int16_t,
    65 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    11 as libc::c_int as flex_int16_t,
    17 as libc::c_int as flex_int16_t,
    81 as libc::c_int as flex_int16_t,
    81 as libc::c_int as flex_int16_t,
    81 as libc::c_int as flex_int16_t,
    82 as libc::c_int as flex_int16_t,
    61 as libc::c_int as flex_int16_t,
    82 as libc::c_int as flex_int16_t,
    60 as libc::c_int as flex_int16_t,
    59 as libc::c_int as flex_int16_t,
    58 as libc::c_int as flex_int16_t,
    57 as libc::c_int as flex_int16_t,
    56 as libc::c_int as flex_int16_t,
    54 as libc::c_int as flex_int16_t,
    53 as libc::c_int as flex_int16_t,
    51 as libc::c_int as flex_int16_t,
    50 as libc::c_int as flex_int16_t,
    49 as libc::c_int as flex_int16_t,
    48 as libc::c_int as flex_int16_t,
    47 as libc::c_int as flex_int16_t,
    45 as libc::c_int as flex_int16_t,
    44 as libc::c_int as flex_int16_t,
    43 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    41 as libc::c_int as flex_int16_t,
    40 as libc::c_int as flex_int16_t,
    39 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    36 as libc::c_int as flex_int16_t,
    35 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    32 as libc::c_int as flex_int16_t,
    31 as libc::c_int as flex_int16_t,
    30 as libc::c_int as flex_int16_t,
    29 as libc::c_int as flex_int16_t,
    28 as libc::c_int as flex_int16_t,
    23 as libc::c_int as flex_int16_t,
    22 as libc::c_int as flex_int16_t,
    21 as libc::c_int as flex_int16_t,
    20 as libc::c_int as flex_int16_t,
    19 as libc::c_int as flex_int16_t,
    16 as libc::c_int as flex_int16_t,
    8 as libc::c_int as flex_int16_t,
    3 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
];
static mut yy_last_accepting_cpos: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
static mut yy_last_accepting_state: yy_state_type = 0;
static mut yy_accept: [flex_int16_t; 81] = [
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    26 as libc::c_int as flex_int16_t,
    25 as libc::c_int as flex_int16_t,
    7 as libc::c_int as flex_int16_t,
    8 as libc::c_int as flex_int16_t,
    5 as libc::c_int as flex_int16_t,
    25 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    11 as libc::c_int as flex_int16_t,
    11 as libc::c_int as flex_int16_t,
    6 as libc::c_int as flex_int16_t,
    3 as libc::c_int as flex_int16_t,
    4 as libc::c_int as flex_int16_t,
    25 as libc::c_int as flex_int16_t,
    25 as libc::c_int as flex_int16_t,
    25 as libc::c_int as flex_int16_t,
    25 as libc::c_int as flex_int16_t,
    25 as libc::c_int as flex_int16_t,
    25 as libc::c_int as flex_int16_t,
    25 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    9 as libc::c_int as flex_int16_t,
    11 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    6 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    10 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    13 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    16 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    12 as libc::c_int as flex_int16_t,
    15 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    23 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    14 as libc::c_int as flex_int16_t,
    18 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    17 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    24 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    20 as libc::c_int as flex_int16_t,
    22 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    21 as libc::c_int as flex_int16_t,
    19 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
];
unsafe extern "C" fn yy_try_NUL_trans(
    mut yy_current_state: yy_state_type,
) -> yy_state_type {
    let mut yy_is_jam: libc::c_int = 0;
    let mut yy_cp: *mut libc::c_char = yy_c_buf_p;
    let mut yy_c: YY_CHAR = 1 as libc::c_int as YY_CHAR;
    if yy_accept[yy_current_state as usize] != 0 {
        yy_last_accepting_state = yy_current_state;
        yy_last_accepting_cpos = yy_cp;
    }
    while yy_chk[(yy_base[yy_current_state as usize] as libc::c_int
        + yy_c as libc::c_int) as usize] as libc::c_int != yy_current_state
    {
        yy_current_state = yy_def[yy_current_state as usize] as libc::c_int;
        if yy_current_state >= 81 as libc::c_int {
            yy_c = yy_meta[yy_c as usize];
        }
    }
    yy_current_state = yy_nxt[(yy_base[yy_current_state as usize] as libc::c_int
        + yy_c as libc::c_int) as usize] as yy_state_type;
    yy_is_jam = (yy_current_state == 80 as libc::c_int) as libc::c_int;
    return if yy_is_jam != 0 { 0 as libc::c_int } else { yy_current_state };
}
unsafe extern "C" fn yy_get_next_buffer() -> libc::c_int {
    let mut dest: *mut libc::c_char = (**yy_buffer_stack
        .offset(yy_buffer_stack_top as isize))
        .yy_ch_buf;
    let mut source: *mut libc::c_char = yytext;
    let mut number_to_move: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut ret_val: libc::c_int = 0;
    if yy_c_buf_p
        > &mut *((**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_ch_buf)
            .offset((yy_n_chars + 1 as libc::c_int) as isize) as *mut libc::c_char
    {
        yy_fatal_error(
            b"fatal flex scanner internal error--end of buffer missed\0" as *const u8
                as *const libc::c_char,
        );
    }
    if (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_fill_buffer
        == 0 as libc::c_int
    {
        if yy_c_buf_p.offset_from(yytext) as libc::c_long
            - 0 as libc::c_int as libc::c_long == 1 as libc::c_int as libc::c_long
        {
            return 1 as libc::c_int
        } else {
            return 2 as libc::c_int
        }
    }
    number_to_move = (yy_c_buf_p.offset_from(yytext) as libc::c_long
        - 1 as libc::c_int as libc::c_long) as libc::c_int;
    i = 0 as libc::c_int;
    while i < number_to_move {
        let fresh8 = source;
        source = source.offset(1);
        let fresh9 = dest;
        dest = dest.offset(1);
        *fresh9 = *fresh8;
        i += 1;
        i;
    }
    if (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_buffer_status
        == 2 as libc::c_int
    {
        yy_n_chars = 0 as libc::c_int;
        (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_n_chars = yy_n_chars;
    } else {
        let mut num_to_read: libc::c_int = (**yy_buffer_stack
            .offset(yy_buffer_stack_top as isize))
            .yy_buf_size - number_to_move - 1 as libc::c_int;
        while num_to_read <= 0 as libc::c_int {
            let mut b: YY_BUFFER_STATE = *yy_buffer_stack
                .offset(yy_buffer_stack_top as isize);
            let mut yy_c_buf_p_offset: libc::c_int = yy_c_buf_p
                .offset_from((*b).yy_ch_buf) as libc::c_long as libc::c_int;
            if (*b).yy_is_our_buffer != 0 {
                let mut new_size: libc::c_int = (*b).yy_buf_size * 2 as libc::c_int;
                if new_size <= 0 as libc::c_int {
                    (*b).yy_buf_size += (*b).yy_buf_size / 8 as libc::c_int;
                } else {
                    (*b).yy_buf_size *= 2 as libc::c_int;
                }
                (*b)
                    .yy_ch_buf = yyrealloc(
                    (*b).yy_ch_buf as *mut libc::c_void,
                    ((*b).yy_buf_size + 2 as libc::c_int) as yy_size_t,
                ) as *mut libc::c_char;
            } else {
                (*b).yy_ch_buf = 0 as *mut libc::c_char;
            }
            if ((*b).yy_ch_buf).is_null() {
                yy_fatal_error(
                    b"fatal error - scanner input buffer overflow\0" as *const u8
                        as *const libc::c_char,
                );
            }
            yy_c_buf_p = &mut *((*b).yy_ch_buf).offset(yy_c_buf_p_offset as isize)
                as *mut libc::c_char;
            num_to_read = (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                .yy_buf_size - number_to_move - 1 as libc::c_int;
        }
        if num_to_read > 8192 as libc::c_int {
            num_to_read = 8192 as libc::c_int;
        }
        if (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_is_interactive
            != 0
        {
            let mut c: libc::c_int = '*' as i32;
            let mut n: libc::c_int = 0;
            n = 0 as libc::c_int;
            while n < num_to_read
                && {
                    c = getc(yyin);
                    c != -(1 as libc::c_int)
                } && c != '\n' as i32
            {
                *(&mut *((**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                    .yy_ch_buf)
                    .offset(number_to_move as isize) as *mut libc::c_char)
                    .offset(n as isize) = c as libc::c_char;
                n += 1;
                n;
            }
            if c == '\n' as i32 {
                let fresh10 = n;
                n = n + 1;
                *(&mut *((**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                    .yy_ch_buf)
                    .offset(number_to_move as isize) as *mut libc::c_char)
                    .offset(fresh10 as isize) = c as libc::c_char;
            }
            if c == -(1 as libc::c_int) && ferror(yyin) != 0 {
                yy_fatal_error(
                    b"input in flex scanner failed\0" as *const u8 as *const libc::c_char,
                );
            }
            yy_n_chars = n;
        } else {
            *__errno_location() = 0 as libc::c_int;
            loop {
                yy_n_chars = fread(
                    &mut *((**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                        .yy_ch_buf)
                        .offset(number_to_move as isize) as *mut libc::c_char
                        as *mut libc::c_void,
                    1 as libc::c_int as libc::c_ulong,
                    num_to_read as yy_size_t,
                    yyin,
                ) as libc::c_int;
                if !(yy_n_chars == 0 as libc::c_int && ferror(yyin) != 0) {
                    break;
                }
                if *__errno_location() != 4 as libc::c_int {
                    yy_fatal_error(
                        b"input in flex scanner failed\0" as *const u8
                            as *const libc::c_char,
                    );
                } else {
                    *__errno_location() = 0 as libc::c_int;
                    clearerr(yyin);
                }
            }
        }
        (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_n_chars = yy_n_chars;
    }
    if yy_n_chars == 0 as libc::c_int {
        if number_to_move == 0 as libc::c_int {
            ret_val = 1 as libc::c_int;
            yyrestart(yyin);
        } else {
            ret_val = 2 as libc::c_int;
            (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                .yy_buffer_status = 2 as libc::c_int;
        }
    } else {
        ret_val = 0 as libc::c_int;
    }
    if yy_n_chars + number_to_move
        > (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_buf_size
    {
        let mut new_size_0: libc::c_int = yy_n_chars + number_to_move
            + (yy_n_chars >> 1 as libc::c_int);
        let ref mut fresh11 = (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
            .yy_ch_buf;
        *fresh11 = yyrealloc(
            (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_ch_buf
                as *mut libc::c_void,
            new_size_0 as yy_size_t,
        ) as *mut libc::c_char;
        if ((**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_ch_buf).is_null()
        {
            yy_fatal_error(
                b"out of dynamic memory in yy_get_next_buffer()\0" as *const u8
                    as *const libc::c_char,
            );
        }
        (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
            .yy_buf_size = new_size_0 - 2 as libc::c_int;
    }
    yy_n_chars += number_to_move;
    *((**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_ch_buf)
        .offset(yy_n_chars as isize) = 0 as libc::c_int as libc::c_char;
    *((**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_ch_buf)
        .offset(
            (yy_n_chars + 1 as libc::c_int) as isize,
        ) = 0 as libc::c_int as libc::c_char;
    yytext = &mut *((**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_ch_buf)
        .offset(0 as libc::c_int as isize) as *mut libc::c_char;
    return ret_val;
}
#[no_mangle]
pub static mut yy_flex_debug: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn yylex() -> libc::c_int {
    let mut yy_amount_of_matched_text: libc::c_int = 0;
    let mut yy_next_state: yy_state_type = 0;
    let mut current_block: u64;
    let mut yy_current_state: yy_state_type = 0;
    let mut yy_cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut yy_bp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut yy_act: libc::c_int = 0;
    if yy_init == 0 {
        yy_init = 1 as libc::c_int;
        if yy_start == 0 {
            yy_start = 1 as libc::c_int;
        }
        if yyin.is_null() {
            yyin = stdin;
        }
        if yyout.is_null() {
            yyout = stdout;
        }
        if if !yy_buffer_stack.is_null() {
            *yy_buffer_stack.offset(yy_buffer_stack_top as isize)
        } else {
            0 as YY_BUFFER_STATE
        }
            .is_null()
        {
            yyensure_buffer_stack();
            let ref mut fresh12 = *yy_buffer_stack.offset(yy_buffer_stack_top as isize);
            *fresh12 = yy_create_buffer(yyin, 16384 as libc::c_int);
        }
        yy_load_buffer_state();
    }
    loop {
        yy_cp = yy_c_buf_p;
        *yy_cp = yy_hold_char;
        yy_bp = yy_cp;
        yy_current_state = yy_start;
        '_yy_match: loop {
            loop {
                let mut yy_c: YY_CHAR = yy_ec[*yy_cp as YY_CHAR as usize];
                if yy_accept[yy_current_state as usize] != 0 {
                    yy_last_accepting_state = yy_current_state;
                    yy_last_accepting_cpos = yy_cp;
                }
                while yy_chk[(yy_base[yy_current_state as usize] as libc::c_int
                    + yy_c as libc::c_int) as usize] as libc::c_int != yy_current_state
                {
                    yy_current_state = yy_def[yy_current_state as usize] as libc::c_int;
                    if yy_current_state >= 81 as libc::c_int {
                        yy_c = yy_meta[yy_c as usize];
                    }
                }
                yy_current_state = yy_nxt[(yy_base[yy_current_state as usize]
                    as libc::c_int + yy_c as libc::c_int) as usize] as yy_state_type;
                yy_cp = yy_cp.offset(1);
                yy_cp;
                if !(yy_base[yy_current_state as usize] as libc::c_int
                    != 101 as libc::c_int)
                {
                    break;
                }
            }
            '_yy_find_action: loop {
                yy_act = yy_accept[yy_current_state as usize] as libc::c_int;
                if yy_act == 0 as libc::c_int {
                    yy_cp = yy_last_accepting_cpos;
                    yy_current_state = yy_last_accepting_state;
                    yy_act = yy_accept[yy_current_state as usize] as libc::c_int;
                }
                yytext = yy_bp;
                yyleng = yy_cp.offset_from(yy_bp) as libc::c_long as libc::c_int;
                yy_hold_char = *yy_cp;
                *yy_cp = '\0' as i32 as libc::c_char;
                yy_c_buf_p = yy_cp;
                loop {
                    match yy_act {
                        0 => {
                            *yy_cp = yy_hold_char;
                            yy_cp = yy_last_accepting_cpos;
                            yy_current_state = yy_last_accepting_state;
                            continue '_yy_find_action;
                        }
                        1 => return '(' as i32,
                        2 => return ')' as i32,
                        3 => return '[' as i32,
                        4 => return ']' as i32,
                        5 | 6 | 7 | 8 => {
                            break '_yy_match;
                        }
                        9 => {
                            yylval
                                .s = malloc((yyleng - 1 as libc::c_int) as libc::c_ulong)
                                as *mut libc::c_char;
                            memcpy(
                                yylval.s as *mut libc::c_void,
                                yytext.offset(1 as libc::c_int as isize)
                                    as *const libc::c_void,
                                (yyleng - 2 as libc::c_int) as libc::c_ulong,
                            );
                            *(yylval.s)
                                .offset(
                                    (yyleng - 2 as libc::c_int) as isize,
                                ) = '\0' as i32 as libc::c_char;
                            return 261 as libc::c_int;
                        }
                        10 => {
                            yylval
                                .i = strtol(
                                yytext,
                                0 as *mut *mut libc::c_char,
                                16 as libc::c_int,
                            ) as libc::c_int;
                            return 262 as libc::c_int;
                        }
                        11 => {
                            yylval.i = atoi(yytext);
                            return 262 as libc::c_int;
                        }
                        12 => {
                            yylval.i = 1 as libc::c_int;
                            return 263 as libc::c_int;
                        }
                        13 => {
                            yylval.i = 1 as libc::c_int;
                            return 263 as libc::c_int;
                        }
                        14 => {
                            yylval.i = 8 as libc::c_int;
                            return 263 as libc::c_int;
                        }
                        15 => {
                            yylval.i = 8 as libc::c_int;
                            return 263 as libc::c_int;
                        }
                        16 => {
                            yylval
                                .s = b"INT\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char;
                            return 260 as libc::c_int;
                        }
                        17 => {
                            yylval
                                .s = b"BARRAY\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char;
                            return 260 as libc::c_int;
                        }
                        18 => {
                            yylval
                                .s = b"CHARS\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char;
                            return 260 as libc::c_int;
                        }
                        19 => {
                            yylval.i = 0 as libc::c_int;
                            return 262 as libc::c_int;
                        }
                        20 => {
                            yylval.i = -(4 as libc::c_int);
                            return 262 as libc::c_int;
                        }
                        21 => {
                            yylval.i = -(2 as libc::c_int);
                            return 262 as libc::c_int;
                        }
                        22 => {
                            yylval.i = -(1 as libc::c_int);
                            return 262 as libc::c_int;
                        }
                        23 => return 258 as libc::c_int,
                        24 => return 259 as libc::c_int,
                        25 => {
                            fwrite(
                                yytext as *const libc::c_void,
                                yyleng as size_t,
                                1 as libc::c_int as libc::c_ulong,
                                yyout,
                            ) != 0;
                            break '_yy_match;
                        }
                        27 => return 0 as libc::c_int,
                        26 => {
                            yy_amount_of_matched_text = yy_cp.offset_from(yytext)
                                as libc::c_long as libc::c_int - 1 as libc::c_int;
                            *yy_cp = yy_hold_char;
                            if (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                .yy_buffer_status == 0 as libc::c_int
                            {
                                yy_n_chars = (**yy_buffer_stack
                                    .offset(yy_buffer_stack_top as isize))
                                    .yy_n_chars;
                                let ref mut fresh13 = (**yy_buffer_stack
                                    .offset(yy_buffer_stack_top as isize))
                                    .yy_input_file;
                                *fresh13 = yyin;
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_buffer_status = 1 as libc::c_int;
                            }
                            if yy_c_buf_p
                                <= &mut *((**yy_buffer_stack
                                    .offset(yy_buffer_stack_top as isize))
                                    .yy_ch_buf)
                                    .offset(yy_n_chars as isize) as *mut libc::c_char
                            {
                                yy_next_state = 0;
                                yy_c_buf_p = yytext
                                    .offset(yy_amount_of_matched_text as isize);
                                yy_current_state = yy_get_previous_state();
                                yy_next_state = yy_try_NUL_trans(yy_current_state);
                                yy_bp = yytext.offset(0 as libc::c_int as isize);
                                if yy_next_state != 0 {
                                    current_block = 3024367268842933116;
                                    break;
                                } else {
                                    current_block = 16593409533420678784;
                                    break;
                                }
                            } else {
                                match yy_get_next_buffer() {
                                    1 => {
                                        yy_did_buffer_switch_on_eof = 0 as libc::c_int;
                                        yy_c_buf_p = yytext.offset(0 as libc::c_int as isize);
                                        yy_act = 26 as libc::c_int
                                            + (yy_start - 1 as libc::c_int) / 2 as libc::c_int
                                            + 1 as libc::c_int;
                                    }
                                    0 => {
                                        yy_c_buf_p = yytext
                                            .offset(yy_amount_of_matched_text as isize);
                                        yy_current_state = yy_get_previous_state();
                                        yy_cp = yy_c_buf_p;
                                        yy_bp = yytext.offset(0 as libc::c_int as isize);
                                        break '_yy_find_action;
                                    }
                                    2 => {
                                        yy_c_buf_p = &mut *((**yy_buffer_stack
                                            .offset(yy_buffer_stack_top as isize))
                                            .yy_ch_buf)
                                            .offset(yy_n_chars as isize) as *mut libc::c_char;
                                        yy_current_state = yy_get_previous_state();
                                        yy_cp = yy_c_buf_p;
                                        yy_bp = yytext.offset(0 as libc::c_int as isize);
                                        continue '_yy_find_action;
                                    }
                                    _ => {
                                        break '_yy_match;
                                    }
                                }
                            }
                        }
                        _ => {
                            yy_fatal_error(
                                b"fatal flex scanner internal error--no action found\0"
                                    as *const u8 as *const libc::c_char,
                            );
                        }
                    }
                }
                match current_block {
                    16593409533420678784 => {
                        yy_cp = yy_c_buf_p;
                    }
                    _ => {
                        yy_c_buf_p = yy_c_buf_p.offset(1);
                        yy_cp = yy_c_buf_p;
                        yy_current_state = yy_next_state;
                        break;
                    }
                }
            }
        }
    };
}
unsafe extern "C" fn yy_init_globals() -> libc::c_int {
    yy_buffer_stack = 0 as *mut YY_BUFFER_STATE;
    yy_buffer_stack_top = 0 as libc::c_int as size_t;
    yy_buffer_stack_max = 0 as libc::c_int as size_t;
    yy_c_buf_p = 0 as *mut libc::c_char;
    yy_init = 0 as libc::c_int;
    yy_start = 0 as libc::c_int;
    yyin = 0 as *mut FILE;
    yyout = 0 as *mut FILE;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn yylex_destroy() -> libc::c_int {
    while !if !yy_buffer_stack.is_null() {
        *yy_buffer_stack.offset(yy_buffer_stack_top as isize)
    } else {
        0 as YY_BUFFER_STATE
    }
        .is_null()
    {
        yy_delete_buffer(
            if !yy_buffer_stack.is_null() {
                *yy_buffer_stack.offset(yy_buffer_stack_top as isize)
            } else {
                0 as YY_BUFFER_STATE
            },
        );
        let ref mut fresh14 = *yy_buffer_stack.offset(yy_buffer_stack_top as isize);
        *fresh14 = 0 as YY_BUFFER_STATE;
        yypop_buffer_state();
    }
    yyfree(yy_buffer_stack as *mut libc::c_void);
    yy_buffer_stack = 0 as *mut YY_BUFFER_STATE;
    yy_init_globals();
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn yyget_debug() -> libc::c_int {
    return yy_flex_debug;
}
#[no_mangle]
pub unsafe extern "C" fn yyset_debug(mut _bdebug: libc::c_int) {
    yy_flex_debug = _bdebug;
}
#[no_mangle]
pub unsafe extern "C" fn yyget_in() -> *mut FILE {
    return yyin;
}
#[no_mangle]
pub unsafe extern "C" fn yyset_in(mut _in_str: *mut FILE) {
    yyin = _in_str;
}
#[no_mangle]
pub unsafe extern "C" fn yyget_out() -> *mut FILE {
    return yyout;
}
#[no_mangle]
pub unsafe extern "C" fn yyset_out(mut _out_str: *mut FILE) {
    yyout = _out_str;
}
#[no_mangle]
pub unsafe extern "C" fn yyget_leng() -> libc::c_int {
    return yyleng;
}
#[no_mangle]
pub unsafe extern "C" fn yyget_text() -> *mut libc::c_char {
    return yytext;
}
#[no_mangle]
pub unsafe extern "C" fn yyget_lineno() -> libc::c_int {
    return yylineno;
}
#[no_mangle]
pub unsafe extern "C" fn yyset_lineno(mut _line_number: libc::c_int) {
    yylineno = _line_number;
}
