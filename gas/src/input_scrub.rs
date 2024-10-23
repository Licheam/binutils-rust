extern "C" {
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
    fn free(_: *mut libc::c_void);
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    fn xrealloc(_: *mut libc::c_void, _: size_t) -> *mut libc::c_void;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    static mut flag_no_comments: libc::c_uchar;
    static mut listing: libc::c_int;
    static mut max_macro_nest: libc::c_int;
    fn as_fatal(format: *const libc::c_char, _: ...) -> !;
    fn as_warn(format: *const libc::c_char, _: ...);
    fn as_abort(_: *const libc::c_char, _: libc::c_int, _: *const libc::c_char) -> !;
    fn listing_newline(ps: *mut libc::c_char);
    fn cond_finish_check(_: libc::c_int);
    fn do_scrub_begin(_: libc::c_int);
    fn filename_cmp(s1: *const libc::c_char, s2: *const libc::c_char) -> libc::c_int;
    fn input_file_give_next_buffer(where_0: *mut libc::c_char) -> *mut libc::c_char;
    fn input_file_push() -> *mut libc::c_char;
    fn input_file_buffer_size() -> size_t;
    fn input_file_begin();
    fn input_file_close();
    fn input_file_end();
    fn input_file_open(filename: *const libc::c_char, pre: libc::c_int);
    fn input_file_pop(arg: *mut libc::c_char);
    fn sb_build(_: *mut sb, _: size_t);
    fn sb_kill(_: *mut sb);
    fn sb_scrub_and_add_sb(_: *mut sb, _: *mut sb);
    fn sb_add_char(_: *mut sb, _: size_t);
    fn sb_terminate(_: *mut sb) -> *mut libc::c_char;
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct input_save {
    pub buffer_start: *mut libc::c_char,
    pub partial_where: *mut libc::c_char,
    pub partial_size: size_t,
    pub save_source: [libc::c_char; 1],
    pub buffer_length: size_t,
    pub physical_input_file: *const libc::c_char,
    pub logical_input_file: *const libc::c_char,
    pub physical_input_line: libc::c_uint,
    pub logical_input_line: libc::c_int,
    pub sb_index: size_t,
    pub from_sb: sb,
    pub from_sb_is_expansion: libc::c_int,
    pub next_saved_file: *mut input_save,
    pub input_file_save: *mut libc::c_char,
    pub saved_position: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sb {
    pub ptr: *mut libc::c_char,
    pub len: size_t,
    pub max: size_t,
}
#[no_mangle]
pub unsafe extern "C" fn input_scrub_include_file(
    mut filename: *const libc::c_char,
    mut position: *mut libc::c_char,
) -> *mut libc::c_char {
    next_saved_file = input_scrub_push(position);
    return input_scrub_new_file(filename);
}
#[no_mangle]
pub unsafe extern "C" fn input_scrub_new_file(
    mut filename: *const libc::c_char,
) -> *mut libc::c_char {
    input_file_open(filename, (flag_no_comments == 0) as libc::c_int);
    physical_input_file = if *filename.offset(0 as libc::c_int as isize) as libc::c_int
        != 0
    {
        filename
    } else {
        dcgettext(
            0 as *const libc::c_char,
            b"{standard input}\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ) as *const libc::c_char
    };
    physical_input_line = 0 as libc::c_int as libc::c_uint;
    partial_size = 0 as libc::c_int as size_t;
    return buffer_start.offset(1 as libc::c_int as isize);
}
#[no_mangle]
pub unsafe extern "C" fn input_scrub_next_buffer(
    mut bufp: *mut *mut libc::c_char,
) -> *mut libc::c_char {
    let mut limit: *mut libc::c_char = 0 as *mut libc::c_char;
    if sb_index != -(1 as libc::c_int) as size_t {
        if sb_index >= from_sb.len {
            sb_kill(&mut from_sb);
            if from_sb_is_expansion != 0 {
                cond_finish_check(macro_nest);
            }
            macro_nest -= 1;
            macro_nest;
            partial_where = 0 as *mut libc::c_char;
            partial_size = 0 as libc::c_int as size_t;
            if !next_saved_file.is_null() {
                *bufp = input_scrub_pop(next_saved_file);
            }
            return partial_where;
        }
        partial_where = (from_sb.ptr).offset(from_sb.len as isize);
        partial_size = 0 as libc::c_int as size_t;
        *bufp = (from_sb.ptr).offset(sb_index as isize);
        sb_index = from_sb.len;
        return partial_where;
    }
    if partial_size != 0 {
        memmove(
            buffer_start.offset(1 as libc::c_int as isize) as *mut libc::c_void,
            partial_where as *const libc::c_void,
            partial_size,
        );
        memcpy(
            buffer_start.offset(1 as libc::c_int as isize) as *mut libc::c_void,
            save_source.as_mut_ptr() as *const libc::c_void,
            1 as libc::c_int as libc::c_ulong,
        );
    }
    let mut current_block_41: u64;
    loop {
        let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut start: *mut libc::c_char = buffer_start
            .offset(1 as libc::c_int as isize)
            .offset(partial_size as isize);
        *bufp = buffer_start.offset(1 as libc::c_int as isize);
        limit = input_file_give_next_buffer(start);
        if limit.is_null() {
            if partial_size == 0 {
                break;
            }
            as_warn(
                dcgettext(
                    0 as *const libc::c_char,
                    b"end of file not at end of a line; newline inserted\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            p = buffer_start
                .offset(1 as libc::c_int as isize)
                .offset(partial_size as isize);
            let fresh0 = p;
            p = p.offset(1);
            *fresh0 = '\n' as i32 as libc::c_char;
            limit = p;
        } else {
            *limit = '\0' as i32 as libc::c_char;
            p = limit.offset(-(1 as libc::c_int as isize));
            loop {
                if !(*p as libc::c_int != '\n' as i32 || 0 as libc::c_int != 0) {
                    current_block_41 = 14818589718467733107;
                    break;
                }
                if p < start {
                    current_block_41 = 8384097119818790273;
                    break;
                }
                p = p.offset(-1);
                p;
            }
            match current_block_41 {
                14818589718467733107 => {
                    p = p.offset(1);
                    p;
                }
                _ => {
                    partial_size = limit
                        .offset_from(buffer_start.offset(1 as libc::c_int as isize))
                        as libc::c_long as size_t;
                    if buffer_length.wrapping_sub(input_file_buffer_size())
                        < partial_size
                    {
                        buffer_length = (buffer_length as libc::c_ulong)
                            .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t
                            as size_t;
                        buffer_start = xrealloc(
                            buffer_start as *mut libc::c_void,
                            (::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
                                .wrapping_mul(
                                    buffer_length
                                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                        .wrapping_add(1 as libc::c_int as libc::c_ulong),
                                ),
                        ) as *mut libc::c_char;
                    }
                    continue;
                }
            }
        }
        partial_where = p;
        partial_size = limit.offset_from(p) as libc::c_long as size_t;
        memcpy(
            save_source.as_mut_ptr() as *mut libc::c_void,
            partial_where as *const libc::c_void,
            1 as libc::c_int as libc::c_ulong,
        );
        memcpy(
            partial_where as *mut libc::c_void,
            b"\0\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            1 as libc::c_int as libc::c_ulong,
        );
        return partial_where;
    }
    if listing != 0 {
        listing_newline(0 as *mut libc::c_char);
    }
    partial_where = 0 as *mut libc::c_char;
    if !next_saved_file.is_null() {
        *bufp = input_scrub_pop(next_saved_file);
    }
    return partial_where;
}
#[no_mangle]
pub unsafe extern "C" fn input_scrub_end() {
    if !buffer_start.is_null() {
        free(buffer_start as *mut libc::c_void);
        buffer_start = 0 as *mut libc::c_char;
        input_file_end();
    }
}
#[no_mangle]
pub unsafe extern "C" fn seen_at_least_1_file() -> libc::c_int {
    return (physical_input_file != 0 as *mut libc::c_void as *const libc::c_char)
        as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn as_where(mut linep: *mut libc::c_uint) -> *const libc::c_char {
    if !logical_input_file.is_null()
        && (linep.is_null() || logical_input_line >= 0 as libc::c_int)
    {
        if !linep.is_null() {
            *linep = logical_input_line as libc::c_uint;
        }
        return logical_input_file;
    }
    return as_where_physical(linep);
}
#[no_mangle]
pub unsafe extern "C" fn as_where_physical(
    mut linep: *mut libc::c_uint,
) -> *const libc::c_char {
    if !physical_input_file.is_null() {
        if !linep.is_null() {
            *linep = physical_input_line;
        }
        return physical_input_file;
    }
    if !linep.is_null() {
        *linep = 0 as libc::c_int as libc::c_uint;
    }
    return 0 as *const libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn bump_line_counters() {
    if sb_index == -(1 as libc::c_int) as size_t {
        physical_input_line = physical_input_line.wrapping_add(1);
        physical_input_line;
        if logical_input_line >= 0 as libc::c_int {
            logical_input_line += 1;
            logical_input_line;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn input_scrub_begin() {
    physical_input_file = 0 as *const libc::c_char;
    next_saved_file = 0 as *mut input_save;
    input_scrub_reinit();
    do_scrub_begin(0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn input_scrub_close() {
    input_file_close();
    physical_input_line = 0 as libc::c_int as libc::c_uint;
    logical_input_line = -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn new_logical_line(
    mut fname: *const libc::c_char,
    mut line_number: libc::c_int,
) -> libc::c_int {
    return new_logical_line_flags(fname, line_number, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn new_logical_line_flags(
    mut fname: *const libc::c_char,
    mut line_number: libc::c_int,
    mut flags: libc::c_int,
) -> libc::c_int {
    match flags {
        1 => {
            if line_number != -(1 as libc::c_int) {
                as_abort(
                    b"input-scrub.c\0" as *const u8 as *const libc::c_char,
                    453 as libc::c_int,
                    (*::core::mem::transmute::<
                        &[u8; 51],
                        &[libc::c_char; 51],
                    >(b"int new_logical_line_flags(const char *, int, int)\0"))
                        .as_ptr(),
                );
            }
        }
        0 | 2 | 4 => {}
        _ => {
            as_abort(
                b"input-scrub.c\0" as *const u8 as *const libc::c_char,
                460 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 51],
                    &[libc::c_char; 51],
                >(b"int new_logical_line_flags(const char *, int, int)\0"))
                    .as_ptr(),
            );
        }
    }
    if line_number >= 0 as libc::c_int {
        logical_input_line = line_number;
    } else if line_number == -(1 as libc::c_int) && !fname.is_null() && *fname == 0
        && flags & (1 as libc::c_int) << 2 as libc::c_int != 0
    {
        logical_input_file = physical_input_file;
        logical_input_line = physical_input_line as libc::c_int;
        fname = 0 as *const libc::c_char;
    }
    if !fname.is_null()
        && (logical_input_file.is_null() || filename_cmp(logical_input_file, fname) != 0)
    {
        logical_input_file = fname;
        return 1 as libc::c_int;
    } else {
        return 0 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn input_scrub_include_sb(
    mut from: *mut sb,
    mut position: *mut libc::c_char,
    mut is_expansion: libc::c_int,
) {
    let mut newline: libc::c_int = 0;
    if macro_nest > max_macro_nest {
        as_fatal(
            dcgettext(
                0 as *const libc::c_char,
                b"macros nested too deeply\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    macro_nest += 1;
    macro_nest;
    next_saved_file = input_scrub_push(position);
    newline = ((*from).len >= 1 as libc::c_int as libc::c_ulong
        && *((*from).ptr).offset(0 as libc::c_int as isize) as libc::c_int
            != '\n' as i32) as libc::c_int;
    sb_build(&mut from_sb, ((*from).len).wrapping_add(newline as libc::c_ulong));
    from_sb_is_expansion = is_expansion;
    if newline != 0 {
        sb_add_char(&mut from_sb, '\n' as i32 as size_t);
    }
    sb_scrub_and_add_sb(&mut from_sb, from);
    sb_terminate(&mut from_sb);
    sb_index = 1 as libc::c_int as size_t;
    logical_input_line = (*next_saved_file).logical_input_line;
    logical_input_file = (*next_saved_file).logical_input_file;
}
static mut buffer_start: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
static mut partial_where: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
static mut partial_size: size_t = 0;
static mut save_source: [libc::c_char; 1] = [0; 1];
static mut buffer_length: size_t = 0;
static mut sb_index: size_t = -(1 as libc::c_int) as size_t;
static mut from_sb: sb = sb {
    ptr: 0 as *const libc::c_char as *mut libc::c_char,
    len: 0,
    max: 0,
};
static mut from_sb_is_expansion: libc::c_int = 1 as libc::c_int;
#[no_mangle]
pub static mut macro_nest: libc::c_int = 0;
static mut physical_input_file: *const libc::c_char = 0 as *const libc::c_char;
static mut logical_input_file: *const libc::c_char = 0 as *const libc::c_char;
static mut physical_input_line: libc::c_uint = 0;
static mut logical_input_line: libc::c_int = 0;
unsafe extern "C" fn input_scrub_push(
    mut saved_position: *mut libc::c_char,
) -> *mut input_save {
    let mut saved: *mut input_save = 0 as *mut input_save;
    saved = xmalloc(::core::mem::size_of::<input_save>() as libc::c_ulong)
        as *mut input_save;
    (*saved).saved_position = saved_position;
    (*saved).buffer_start = buffer_start;
    (*saved).partial_where = partial_where;
    (*saved).partial_size = partial_size;
    (*saved).buffer_length = buffer_length;
    (*saved).physical_input_file = physical_input_file;
    (*saved).logical_input_file = logical_input_file;
    (*saved).physical_input_line = physical_input_line;
    (*saved).logical_input_line = logical_input_line;
    (*saved).sb_index = sb_index;
    (*saved).from_sb = from_sb;
    (*saved).from_sb_is_expansion = from_sb_is_expansion;
    memcpy(
        ((*saved).save_source).as_mut_ptr() as *mut libc::c_void,
        save_source.as_mut_ptr() as *const libc::c_void,
        ::core::mem::size_of::<[libc::c_char; 1]>() as libc::c_ulong,
    );
    (*saved).next_saved_file = next_saved_file;
    (*saved).input_file_save = input_file_push();
    sb_index = -(1 as libc::c_int) as size_t;
    input_scrub_reinit();
    return saved;
}
unsafe extern "C" fn input_scrub_pop(mut saved: *mut input_save) -> *mut libc::c_char {
    let mut saved_position: *mut libc::c_char = 0 as *mut libc::c_char;
    input_scrub_end();
    input_file_pop((*saved).input_file_save);
    saved_position = (*saved).saved_position;
    buffer_start = (*saved).buffer_start;
    buffer_length = (*saved).buffer_length;
    physical_input_file = (*saved).physical_input_file;
    logical_input_file = (*saved).logical_input_file;
    physical_input_line = (*saved).physical_input_line;
    logical_input_line = (*saved).logical_input_line;
    sb_index = (*saved).sb_index;
    from_sb = (*saved).from_sb;
    from_sb_is_expansion = (*saved).from_sb_is_expansion;
    partial_where = (*saved).partial_where;
    partial_size = (*saved).partial_size;
    next_saved_file = (*saved).next_saved_file;
    memcpy(
        save_source.as_mut_ptr() as *mut libc::c_void,
        ((*saved).save_source).as_mut_ptr() as *const libc::c_void,
        ::core::mem::size_of::<[libc::c_char; 1]>() as libc::c_ulong,
    );
    free(saved as *mut libc::c_void);
    return saved_position;
}
static mut next_saved_file: *mut input_save = 0 as *const input_save as *mut input_save;
unsafe extern "C" fn input_scrub_reinit() {
    input_file_begin();
    logical_input_line = -(1 as libc::c_int);
    logical_input_file = 0 as *const libc::c_char;
    buffer_length = (input_file_buffer_size())
        .wrapping_mul(2 as libc::c_int as libc::c_ulong);
    buffer_start = xmalloc(
        (::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
            .wrapping_mul(
                ((1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int)
                    as libc::c_ulong)
                    .wrapping_add(buffer_length),
            ),
    ) as *mut libc::c_char;
    memcpy(
        buffer_start as *mut libc::c_void,
        b"\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        1 as libc::c_int as libc::c_ulong,
    );
}
