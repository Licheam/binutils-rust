extern "C" {
    fn free(_: *mut libc::c_void);
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn as_fatal(format: *const libc::c_char, _: ...) -> !;
    fn as_warn(format: *const libc::c_char, _: ...);
    fn as_abort(_: *const libc::c_char, _: libc::c_int, _: *const libc::c_char) -> !;
    static mut i386_comment_chars: *const libc::c_char;
    static line_separator_chars: [libc::c_char; 0];
    static line_comment_chars: [libc::c_char; 0];
    static extra_symbol_chars: [libc::c_char; 0];
}
pub type size_t = libc::c_ulong;
pub type ptrdiff_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct app_save {
    pub state: libc::c_int,
    pub old_state: libc::c_int,
    pub out_string: *const libc::c_char,
    pub out_buf: [libc::c_char; 20],
    pub add_newlines: libc::c_int,
    pub saved_input: *mut libc::c_char,
    pub saved_input_len: size_t,
    pub mri_state: *const libc::c_char,
    pub mri_last_ch: libc::c_char,
    pub last_char: libc::c_char,
}
#[no_mangle]
pub unsafe extern "C" fn app_push() -> *mut libc::c_char {
    let mut saved: *mut app_save = 0 as *mut app_save;
    saved = xmalloc(::core::mem::size_of::<app_save>() as libc::c_ulong)
        as *mut app_save;
    (*saved).state = state;
    (*saved).old_state = old_state;
    (*saved).out_string = out_string;
    memcpy(
        ((*saved).out_buf).as_mut_ptr() as *mut libc::c_void,
        out_buf.as_mut_ptr() as *const libc::c_void,
        ::core::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong,
    );
    (*saved).add_newlines = add_newlines;
    if saved_input.is_null() {
        (*saved).saved_input = 0 as *mut libc::c_char;
    } else {
        (*saved)
            .saved_input = xmalloc(
            (::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
                .wrapping_mul(saved_input_len),
        ) as *mut libc::c_char;
        memcpy(
            (*saved).saved_input as *mut libc::c_void,
            saved_input as *const libc::c_void,
            saved_input_len,
        );
        (*saved).saved_input_len = saved_input_len;
    }
    (*saved).mri_state = mri_state;
    (*saved).mri_last_ch = mri_last_ch;
    (*saved).last_char = last_char;
    state = 0 as libc::c_int;
    saved_input = 0 as *mut libc::c_char;
    add_newlines = 0 as libc::c_int;
    return saved as *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn do_scrub_chars(
    mut get: Option::<unsafe extern "C" fn(*mut libc::c_char, size_t) -> size_t>,
    mut tostart: *mut libc::c_char,
    mut tolen: size_t,
) -> size_t {
    let mut current_block: u64;
    let mut to: *mut libc::c_char = tostart;
    let mut toend: *mut libc::c_char = tostart.offset(tolen as isize);
    let mut from: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fromend: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fromlen: size_t = 0;
    let mut ch: libc::c_int = 0;
    let mut ch2: libc::c_int = 0 as libc::c_int;
    static mut quotechar: libc::c_char = 0;
    if !saved_input.is_null() {
        from = saved_input;
        fromend = from.offset(saved_input_len as isize);
    } else {
        fromlen = (Some(get.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            input_buffer.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 32768]>() as libc::c_ulong,
        );
        if fromlen == 0 as libc::c_int as libc::c_ulong {
            return 0 as libc::c_int as size_t;
        }
        from = input_buffer.as_mut_ptr();
        fromend = from.offset(fromlen as isize);
    }
    's_54: loop {
        match state {
            -1 => {
                let fresh0 = out_string;
                out_string = out_string.offset(1);
                ch = *fresh0 as libc::c_int;
                if *out_string as libc::c_int == '\0' as i32 {
                    state = old_state;
                    old_state = 3 as libc::c_int;
                }
                let fresh1 = to;
                to = to.offset(1);
                *fresh1 = ch as libc::c_char;
                if to >= toend {
                    current_block = 14626083211295674653;
                    break;
                }
            }
            -2 => {
                loop {
                    ch = if from < fromend {
                        let fresh2 = from;
                        from = from.offset(1);
                        *(fresh2 as *mut libc::c_uchar) as libc::c_int
                    } else {
                        saved_input = 0 as *mut libc::c_char;
                        fromlen = (Some(get.expect("non-null function pointer")))
                            .expect(
                                "non-null function pointer",
                            )(
                            input_buffer.as_mut_ptr(),
                            ::core::mem::size_of::<[libc::c_char; 32768]>()
                                as libc::c_ulong,
                        );
                        from = input_buffer.as_mut_ptr();
                        fromend = from.offset(fromlen as isize);
                        if fromlen == 0 as libc::c_int as libc::c_ulong {
                            -(1 as libc::c_int)
                        } else {
                            let fresh3 = from;
                            from = from.offset(1);
                            *(fresh3 as *mut libc::c_uchar) as libc::c_int
                        }
                    };
                    if ch == -(1 as libc::c_int) {
                        as_warn(
                            dcgettext(
                                0 as *const libc::c_char,
                                b"end of file in comment\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                        current_block = 10102895335899226404;
                        break 's_54;
                    } else {
                        if ch == '\n' as i32 {
                            let fresh4 = to;
                            to = to.offset(1);
                            *fresh4 = '\n' as i32 as libc::c_char;
                            if to >= toend {
                                current_block = 14626083211295674653;
                                break 's_54;
                            }
                        }
                        if ch != '*' as i32 {
                            continue;
                        }
                        loop {
                            ch = if from < fromend {
                                let fresh5 = from;
                                from = from.offset(1);
                                *(fresh5 as *mut libc::c_uchar) as libc::c_int
                            } else {
                                saved_input = 0 as *mut libc::c_char;
                                fromlen = (Some(get.expect("non-null function pointer")))
                                    .expect(
                                        "non-null function pointer",
                                    )(
                                    input_buffer.as_mut_ptr(),
                                    ::core::mem::size_of::<[libc::c_char; 32768]>()
                                        as libc::c_ulong,
                                );
                                from = input_buffer.as_mut_ptr();
                                fromend = from.offset(fromlen as isize);
                                if fromlen == 0 as libc::c_int as libc::c_ulong {
                                    -(1 as libc::c_int)
                                } else {
                                    let fresh6 = from;
                                    from = from.offset(1);
                                    *(fresh6 as *mut libc::c_uchar) as libc::c_int
                                }
                            };
                            if !(ch == '*' as i32) {
                                break;
                            }
                        }
                        if ch == -(1 as libc::c_int) {
                            as_warn(
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"end of file in comment\0" as *const u8
                                        as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                            );
                            current_block = 10102895335899226404;
                            break 's_54;
                        } else {
                            if ch == '/' as i32 {
                                break;
                            }
                            from = from.offset(-1);
                            *from = ch as libc::c_char;
                        }
                    }
                }
                state = old_state;
                from = from.offset(-1);
                *from = ' ' as i32 as libc::c_char;
            }
            4 => {
                ch = if from < fromend {
                    let fresh7 = from;
                    from = from.offset(1);
                    *(fresh7 as *mut libc::c_uchar) as libc::c_int
                } else {
                    saved_input = 0 as *mut libc::c_char;
                    fromlen = (Some(get.expect("non-null function pointer")))
                        .expect(
                            "non-null function pointer",
                        )(
                        input_buffer.as_mut_ptr(),
                        ::core::mem::size_of::<[libc::c_char; 32768]>() as libc::c_ulong,
                    );
                    from = input_buffer.as_mut_ptr();
                    fromend = from.offset(fromlen as isize);
                    if fromlen == 0 as libc::c_int as libc::c_ulong {
                        -(1 as libc::c_int)
                    } else {
                        let fresh8 = from;
                        from = from.offset(1);
                        *(fresh8 as *mut libc::c_uchar) as libc::c_int
                    }
                };
                if ch == -(1 as libc::c_int) {
                    current_block = 10102895335899226404;
                    break;
                }
                if ch >= '0' as i32 && ch <= '9' as i32 {
                    let fresh9 = to;
                    to = to.offset(1);
                    *fresh9 = ch as libc::c_char;
                    if to >= toend {
                        current_block = 14626083211295674653;
                        break;
                    }
                } else {
                    while ch != -(1 as libc::c_int)
                        && lex[ch as usize] as libc::c_int == 2 as libc::c_int
                    {
                        ch = if from < fromend {
                            let fresh10 = from;
                            from = from.offset(1);
                            *(fresh10 as *mut libc::c_uchar) as libc::c_int
                        } else {
                            saved_input = 0 as *mut libc::c_char;
                            fromlen = (Some(get.expect("non-null function pointer")))
                                .expect(
                                    "non-null function pointer",
                                )(
                                input_buffer.as_mut_ptr(),
                                ::core::mem::size_of::<[libc::c_char; 32768]>()
                                    as libc::c_ulong,
                            );
                            from = input_buffer.as_mut_ptr();
                            fromend = from.offset(fromlen as isize);
                            if fromlen == 0 as libc::c_int as libc::c_ulong {
                                -(1 as libc::c_int)
                            } else {
                                let fresh11 = from;
                                from = from.offset(1);
                                *(fresh11 as *mut libc::c_uchar) as libc::c_int
                            }
                        };
                    }
                    if ch == '"' as i32 {
                        quotechar = ch as libc::c_char;
                        state = 5 as libc::c_int;
                        old_state = 3 as libc::c_int;
                        let fresh12 = to;
                        to = to.offset(1);
                        *fresh12 = ch as libc::c_char;
                        if to >= toend {
                            current_block = 14626083211295674653;
                            break;
                        }
                    } else {
                        while ch != -(1 as libc::c_int) && ch != '\n' as i32 {
                            ch = if from < fromend {
                                let fresh13 = from;
                                from = from.offset(1);
                                *(fresh13 as *mut libc::c_uchar) as libc::c_int
                            } else {
                                saved_input = 0 as *mut libc::c_char;
                                fromlen = (Some(get.expect("non-null function pointer")))
                                    .expect(
                                        "non-null function pointer",
                                    )(
                                    input_buffer.as_mut_ptr(),
                                    ::core::mem::size_of::<[libc::c_char; 32768]>()
                                        as libc::c_ulong,
                                );
                                from = input_buffer.as_mut_ptr();
                                fromend = from.offset(fromlen as isize);
                                if fromlen == 0 as libc::c_int as libc::c_ulong {
                                    -(1 as libc::c_int)
                                } else {
                                    let fresh14 = from;
                                    from = from.offset(1);
                                    *(fresh14 as *mut libc::c_uchar) as libc::c_int
                                }
                            };
                        }
                        state = 0 as libc::c_int;
                        let fresh15 = to;
                        to = to.offset(1);
                        *fresh15 = ch as libc::c_char;
                        if to >= toend {
                            current_block = 14626083211295674653;
                            break;
                        }
                    }
                }
            }
            5 => {
                let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut len: ptrdiff_t = 0;
                s = from;
                while s < fromend {
                    ch = *s as libc::c_int;
                    if ch == '\\' as i32 || ch == quotechar as libc::c_int
                        || ch == '\n' as i32
                    {
                        break;
                    }
                    s = s.offset(1);
                    s;
                }
                len = s.offset_from(from) as libc::c_long;
                if len > toend.offset_from(to) as libc::c_long {
                    len = toend.offset_from(to) as libc::c_long;
                }
                if len > 0 as libc::c_int as libc::c_long {
                    memcpy(
                        to as *mut libc::c_void,
                        from as *const libc::c_void,
                        len as libc::c_ulong,
                    );
                    to = to.offset(len as isize);
                    from = from.offset(len as isize);
                    if to >= toend {
                        current_block = 14626083211295674653;
                        break;
                    }
                }
                ch = if from < fromend {
                    let fresh16 = from;
                    from = from.offset(1);
                    *(fresh16 as *mut libc::c_uchar) as libc::c_int
                } else {
                    saved_input = 0 as *mut libc::c_char;
                    fromlen = (Some(get.expect("non-null function pointer")))
                        .expect(
                            "non-null function pointer",
                        )(
                        input_buffer.as_mut_ptr(),
                        ::core::mem::size_of::<[libc::c_char; 32768]>() as libc::c_ulong,
                    );
                    from = input_buffer.as_mut_ptr();
                    fromend = from.offset(fromlen as isize);
                    if fromlen == 0 as libc::c_int as libc::c_ulong {
                        -(1 as libc::c_int)
                    } else {
                        let fresh17 = from;
                        from = from.offset(1);
                        *(fresh17 as *mut libc::c_uchar) as libc::c_int
                    }
                };
                if ch == -(1 as libc::c_int) {
                    static mut one_char_buf: [libc::c_char; 1] = [0; 1];
                    as_warn(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"end of file in string; '%c' inserted\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        quotechar as libc::c_int,
                    );
                    state = old_state;
                    fromend = one_char_buf
                        .as_mut_ptr()
                        .offset(1 as libc::c_int as isize);
                    from = fromend;
                    fromlen = 1 as libc::c_int as size_t;
                    from = from.offset(-1);
                    *from = '\n' as i32 as libc::c_char;
                    let fresh18 = to;
                    to = to.offset(1);
                    *fresh18 = quotechar;
                    if to >= toend {
                        current_block = 14626083211295674653;
                        break;
                    }
                } else if ch == quotechar as libc::c_int {
                    state = old_state;
                    let fresh19 = to;
                    to = to.offset(1);
                    *fresh19 = ch as libc::c_char;
                    if to >= toend {
                        current_block = 14626083211295674653;
                        break;
                    }
                } else if 1 as libc::c_int != 0 && ch == '\\' as i32 {
                    state = 6 as libc::c_int;
                    let fresh20 = to;
                    to = to.offset(1);
                    *fresh20 = ch as libc::c_char;
                    if to >= toend {
                        current_block = 14626083211295674653;
                        break;
                    }
                } else if 0 as libc::c_int != 0 && ch == '\n' as i32 {
                    state = old_state;
                    from = from.offset(-1);
                    *from = ch as libc::c_char;
                    let fresh21 = to;
                    to = to.offset(1);
                    *fresh21 = '\'' as i32 as libc::c_char;
                    if to >= toend {
                        current_block = 14626083211295674653;
                        break;
                    }
                } else {
                    let fresh22 = to;
                    to = to.offset(1);
                    *fresh22 = ch as libc::c_char;
                    if to >= toend {
                        current_block = 14626083211295674653;
                        break;
                    }
                }
            }
            6 => {
                state = 5 as libc::c_int;
                ch = if from < fromend {
                    let fresh23 = from;
                    from = from.offset(1);
                    *(fresh23 as *mut libc::c_uchar) as libc::c_int
                } else {
                    saved_input = 0 as *mut libc::c_char;
                    fromlen = (Some(get.expect("non-null function pointer")))
                        .expect(
                            "non-null function pointer",
                        )(
                        input_buffer.as_mut_ptr(),
                        ::core::mem::size_of::<[libc::c_char; 32768]>() as libc::c_ulong,
                    );
                    from = input_buffer.as_mut_ptr();
                    fromend = from.offset(fromlen as isize);
                    if fromlen == 0 as libc::c_int as libc::c_ulong {
                        -(1 as libc::c_int)
                    } else {
                        let fresh24 = from;
                        from = from.offset(1);
                        *(fresh24 as *mut libc::c_uchar) as libc::c_int
                    }
                };
                match ch {
                    10 => {
                        from = from.offset(-1);
                        *from = 'n' as i32 as libc::c_char;
                        add_newlines += 1;
                        add_newlines;
                        let fresh25 = to;
                        to = to.offset(1);
                        *fresh25 = '\\' as i32 as libc::c_char;
                        if to >= toend {
                            current_block = 14626083211295674653;
                            break;
                        }
                    }
                    -1 => {
                        as_warn(
                            dcgettext(
                                0 as *const libc::c_char,
                                b"end of file in string; '%c' inserted\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            quotechar as libc::c_int,
                        );
                        let fresh26 = to;
                        to = to.offset(1);
                        *fresh26 = quotechar;
                        if to >= toend {
                            current_block = 14626083211295674653;
                            break;
                        }
                    }
                    34 | 92 | 98 | 102 | 110 | 114 | 116 | 118 | 120 | 88 | 48 | 49 | 50
                    | 51 | 52 | 53 | 54 | 55 | _ => {
                        let fresh27 = to;
                        to = to.offset(1);
                        *fresh27 = ch as libc::c_char;
                        if to >= toend {
                            current_block = 14626083211295674653;
                            break;
                        }
                    }
                }
            }
            _ => {
                ch = if from < fromend {
                    let fresh28 = from;
                    from = from.offset(1);
                    *(fresh28 as *mut libc::c_uchar) as libc::c_int
                } else {
                    saved_input = 0 as *mut libc::c_char;
                    fromlen = (Some(get.expect("non-null function pointer")))
                        .expect(
                            "non-null function pointer",
                        )(
                        input_buffer.as_mut_ptr(),
                        ::core::mem::size_of::<[libc::c_char; 32768]>() as libc::c_ulong,
                    );
                    from = input_buffer.as_mut_ptr();
                    fromend = from.offset(fromlen as isize);
                    if fromlen == 0 as libc::c_int as libc::c_ulong {
                        -(1 as libc::c_int)
                    } else {
                        let fresh29 = from;
                        from = from.offset(1);
                        *(fresh29 as *mut libc::c_uchar) as libc::c_int
                    }
                };
                loop {
                    if ch == -(1 as libc::c_int) {
                        if !(state != 0 as libc::c_int) {
                            current_block = 10102895335899226404;
                            break 's_54;
                        }
                        as_warn(
                            dcgettext(
                                0 as *const libc::c_char,
                                b"end of file not at end of a line; newline inserted\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                        state = 0 as libc::c_int;
                        let fresh30 = to;
                        to = to.offset(1);
                        *fresh30 = '\n' as i32 as libc::c_char;
                        if to >= toend {
                            current_block = 14626083211295674653;
                            break 's_54;
                        } else {
                            current_block = 10102895335899226404;
                            break 's_54;
                        }
                    } else {
                        match lex[ch as usize] as libc::c_int {
                            2 => {
                                loop {
                                    ch = if from < fromend {
                                        let fresh31 = from;
                                        from = from.offset(1);
                                        *(fresh31 as *mut libc::c_uchar) as libc::c_int
                                    } else {
                                        saved_input = 0 as *mut libc::c_char;
                                        fromlen = (Some(get.expect("non-null function pointer")))
                                            .expect(
                                                "non-null function pointer",
                                            )(
                                            input_buffer.as_mut_ptr(),
                                            ::core::mem::size_of::<[libc::c_char; 32768]>()
                                                as libc::c_ulong,
                                        );
                                        from = input_buffer.as_mut_ptr();
                                        fromend = from.offset(fromlen as isize);
                                        if fromlen == 0 as libc::c_int as libc::c_ulong {
                                            -(1 as libc::c_int)
                                        } else {
                                            let fresh32 = from;
                                            from = from.offset(1);
                                            *(fresh32 as *mut libc::c_uchar) as libc::c_int
                                        }
                                    };
                                    if !(ch != -(1 as libc::c_int)
                                        && lex[ch as usize] as libc::c_int == 2 as libc::c_int)
                                    {
                                        break;
                                    }
                                }
                                if ch == -(1 as libc::c_int) {
                                    current_block = 10102895335899226404;
                                    break 's_54;
                                }
                                if state == 0 as libc::c_int {
                                    state = 1 as libc::c_int;
                                    from = from.offset(-1);
                                    *from = ch as libc::c_char;
                                    let fresh33 = to;
                                    to = to.offset(1);
                                    *fresh33 = ' ' as i32 as libc::c_char;
                                    if to >= toend {
                                        current_block = 14626083211295674653;
                                        break 's_54;
                                    } else {
                                        continue 's_54;
                                    }
                                } else {
                                    if lex[ch as usize] as libc::c_int == 4 as libc::c_int
                                        || lex[ch as usize] as libc::c_int == 3 as libc::c_int
                                        || lex[ch as usize] as libc::c_int == 14 as libc::c_int
                                    {
                                        continue;
                                    }
                                    if (state == 2 as libc::c_int || state == 11 as libc::c_int)
                                        && lex[ch as usize] as libc::c_int == 9 as libc::c_int
                                        && 0 as libc::c_int == 0
                                    {
                                        state = 1 as libc::c_int;
                                        let fresh35 = to;
                                        to = to.offset(1);
                                        *fresh35 = ch as libc::c_char;
                                        if to >= toend {
                                            current_block = 14626083211295674653;
                                            break 's_54;
                                        } else {
                                            continue 's_54;
                                        }
                                    } else {
                                        match state {
                                            1 => {}
                                            2 => {
                                                state = 3 as libc::c_int;
                                                if to.offset(1 as libc::c_int as isize) < toend {
                                                    let fresh36 = to;
                                                    to = to.offset(1);
                                                    *fresh36 = ' ' as i32 as libc::c_char;
                                                    if to >= toend {
                                                        current_block = 14626083211295674653;
                                                        break 's_54;
                                                    }
                                                } else {
                                                    from = from.offset(-1);
                                                    *from = ch as libc::c_char;
                                                    let fresh37 = to;
                                                    to = to.offset(1);
                                                    *fresh37 = ' ' as i32 as libc::c_char;
                                                    if to >= toend {
                                                        current_block = 14626083211295674653;
                                                        break 's_54;
                                                    } else {
                                                        continue 's_54;
                                                    }
                                                }
                                            }
                                            3 => {}
                                            9 | 10 => {
                                                state = 10 as libc::c_int;
                                            }
                                            11 => {
                                                if 0 as libc::c_int != 0 || 0 as libc::c_int != 0 {
                                                    state = 1 as libc::c_int;
                                                } else {
                                                    state = 3 as libc::c_int;
                                                }
                                                from = from.offset(-1);
                                                *from = ch as libc::c_char;
                                                let fresh40 = to;
                                                to = to.offset(1);
                                                *fresh40 = ' ' as i32 as libc::c_char;
                                                if to >= toend {
                                                    current_block = 14626083211295674653;
                                                    break 's_54;
                                                } else {
                                                    continue 's_54;
                                                }
                                            }
                                            _ => {
                                                as_fatal(
                                                    dcgettext(
                                                        0 as *const libc::c_char,
                                                        b"Case value %ld unexpected at line %d of file \"%s\"\n\0"
                                                            as *const u8 as *const libc::c_char,
                                                        5 as libc::c_int,
                                                    ),
                                                    state as libc::c_long,
                                                    963 as libc::c_int,
                                                    b"app.c\0" as *const u8 as *const libc::c_char,
                                                );
                                            }
                                        }
                                    }
                                }
                            }
                            6 => {
                                ch2 = if from < fromend {
                                    let fresh41 = from;
                                    from = from.offset(1);
                                    *(fresh41 as *mut libc::c_uchar) as libc::c_int
                                } else {
                                    saved_input = 0 as *mut libc::c_char;
                                    fromlen = (Some(get.expect("non-null function pointer")))
                                        .expect(
                                            "non-null function pointer",
                                        )(
                                        input_buffer.as_mut_ptr(),
                                        ::core::mem::size_of::<[libc::c_char; 32768]>()
                                            as libc::c_ulong,
                                    );
                                    from = input_buffer.as_mut_ptr();
                                    fromend = from.offset(fromlen as isize);
                                    if fromlen == 0 as libc::c_int as libc::c_ulong {
                                        -(1 as libc::c_int)
                                    } else {
                                        let fresh42 = from;
                                        from = from.offset(1);
                                        *(fresh42 as *mut libc::c_uchar) as libc::c_int
                                    }
                                };
                                if ch2 == '*' as i32 {
                                    loop {
                                        loop {
                                            ch2 = if from < fromend {
                                                let fresh43 = from;
                                                from = from.offset(1);
                                                *(fresh43 as *mut libc::c_uchar) as libc::c_int
                                            } else {
                                                saved_input = 0 as *mut libc::c_char;
                                                fromlen = (Some(get.expect("non-null function pointer")))
                                                    .expect(
                                                        "non-null function pointer",
                                                    )(
                                                    input_buffer.as_mut_ptr(),
                                                    ::core::mem::size_of::<[libc::c_char; 32768]>()
                                                        as libc::c_ulong,
                                                );
                                                from = input_buffer.as_mut_ptr();
                                                fromend = from.offset(fromlen as isize);
                                                if fromlen == 0 as libc::c_int as libc::c_ulong {
                                                    -(1 as libc::c_int)
                                                } else {
                                                    let fresh44 = from;
                                                    from = from.offset(1);
                                                    *(fresh44 as *mut libc::c_uchar) as libc::c_int
                                                }
                                            };
                                            if ch2 != -(1 as libc::c_int)
                                                && lex[ch2 as usize] as libc::c_int == 10 as libc::c_int
                                            {
                                                add_newlines += 1;
                                                add_newlines;
                                            }
                                            if !(ch2 != -(1 as libc::c_int) && ch2 != '*' as i32) {
                                                break;
                                            }
                                        }
                                        while ch2 == '*' as i32 {
                                            ch2 = if from < fromend {
                                                let fresh45 = from;
                                                from = from.offset(1);
                                                *(fresh45 as *mut libc::c_uchar) as libc::c_int
                                            } else {
                                                saved_input = 0 as *mut libc::c_char;
                                                fromlen = (Some(get.expect("non-null function pointer")))
                                                    .expect(
                                                        "non-null function pointer",
                                                    )(
                                                    input_buffer.as_mut_ptr(),
                                                    ::core::mem::size_of::<[libc::c_char; 32768]>()
                                                        as libc::c_ulong,
                                                );
                                                from = input_buffer.as_mut_ptr();
                                                fromend = from.offset(fromlen as isize);
                                                if fromlen == 0 as libc::c_int as libc::c_ulong {
                                                    -(1 as libc::c_int)
                                                } else {
                                                    let fresh46 = from;
                                                    from = from.offset(1);
                                                    *(fresh46 as *mut libc::c_uchar) as libc::c_int
                                                }
                                            };
                                        }
                                        if ch2 == -(1 as libc::c_int) || ch2 == '/' as i32 {
                                            break;
                                        }
                                        from = from.offset(-1);
                                        *from = ch2 as libc::c_char;
                                    }
                                    if ch2 == -(1 as libc::c_int) {
                                        as_warn(
                                            dcgettext(
                                                0 as *const libc::c_char,
                                                b"end of file in multiline comment\0" as *const u8
                                                    as *const libc::c_char,
                                                5 as libc::c_int,
                                            ),
                                        );
                                    }
                                    ch = ' ' as i32;
                                } else {
                                    if ch2 != -(1 as libc::c_int) {
                                        from = from.offset(-1);
                                        *from = ch2 as libc::c_char;
                                    }
                                    if state == 9 as libc::c_int || state == 10 as libc::c_int {
                                        state = 3 as libc::c_int;
                                    }
                                    let fresh47 = to;
                                    to = to.offset(1);
                                    *fresh47 = ch as libc::c_char;
                                    if to >= toend {
                                        current_block = 14626083211295674653;
                                        break 's_54;
                                    } else {
                                        continue 's_54;
                                    }
                                }
                            }
                            8 => {
                                quotechar = ch as libc::c_char;
                                if state == 10 as libc::c_int {
                                    current_block = 11510301253690344784;
                                    break;
                                } else {
                                    current_block = 9728093949049737828;
                                    break;
                                }
                            }
                            11 => {
                                if state == 10 as libc::c_int {
                                    current_block = 4746208608892471902;
                                    break;
                                } else {
                                    current_block = 16286683003977321678;
                                    break;
                                }
                            }
                            9 => {
                                if state == 9 as libc::c_int || state == 10 as libc::c_int {
                                    state = 3 as libc::c_int;
                                } else if state != 3 as libc::c_int {
                                    state = 1 as libc::c_int;
                                }
                                let fresh62 = to;
                                to = to.offset(1);
                                *fresh62 = ch as libc::c_char;
                                if to >= toend {
                                    current_block = 14626083211295674653;
                                    break 's_54;
                                } else {
                                    continue 's_54;
                                }
                            }
                            10 => {
                                if add_newlines != 0 {
                                    add_newlines -= 1;
                                    add_newlines;
                                    from = from.offset(-1);
                                    *from = ch as libc::c_char;
                                }
                                current_block = 12724295801459943709;
                                break;
                            }
                            3 => {
                                current_block = 12724295801459943709;
                                break;
                            }
                            14 => {
                                state = 1 as libc::c_int;
                                let fresh64 = to;
                                to = to.offset(1);
                                *fresh64 = ch as libc::c_char;
                                if to >= toend {
                                    current_block = 14626083211295674653;
                                    break 's_54;
                                } else {
                                    continue 's_54;
                                }
                            }
                            5 => {
                                if ch == '/' as i32 {
                                    current_block = 6955753519599442332;
                                    break;
                                } else {
                                    current_block = 3066043723669372660;
                                    break;
                                }
                            }
                            4 => {
                                current_block = 15212124324706515022;
                                break;
                            }
                            1 => {
                                if state == 10 as libc::c_int {
                                    current_block = 14898553815918780345;
                                    break;
                                } else {
                                    current_block = 4161230424924847443;
                                    break;
                                }
                            }
                            _ => {
                                current_block = 1203695113984090064;
                                break;
                            }
                        }
                    }
                }
                match current_block {
                    4161230424924847443 => {
                        if state == 3 as libc::c_int {
                            state = 9 as libc::c_int;
                        }
                        if to.offset(1 as libc::c_int as isize) < toend
                            && mri_state.is_null()
                        {
                            let mut s_0: *mut libc::c_char = 0 as *mut libc::c_char;
                            let mut len_0: ptrdiff_t = 0;
                            s_0 = from;
                            while s_0 < fromend {
                                let mut type_0: libc::c_int = 0;
                                ch2 = *(s_0 as *mut libc::c_uchar) as libc::c_int;
                                type_0 = lex[ch2 as usize] as libc::c_int;
                                if type_0 != 0 as libc::c_int && type_0 != 1 as libc::c_int
                                {
                                    break;
                                }
                                s_0 = s_0.offset(1);
                                s_0;
                            }
                            if s_0 > from {
                                s_0 = s_0.offset(-1);
                                s_0;
                            }
                            len_0 = s_0.offset_from(from) as libc::c_long;
                            if len_0
                                > toend.offset_from(to) as libc::c_long
                                    - 1 as libc::c_int as libc::c_long
                            {
                                len_0 = toend.offset_from(to) as libc::c_long
                                    - 1 as libc::c_int as libc::c_long;
                            }
                            if len_0 > 0 as libc::c_int as libc::c_long {
                                let fresh79 = to;
                                to = to.offset(1);
                                *fresh79 = ch as libc::c_char;
                                if to >= toend {
                                    current_block = 14626083211295674653;
                                    break;
                                }
                                memcpy(
                                    to as *mut libc::c_void,
                                    from as *const libc::c_void,
                                    len_0 as libc::c_ulong,
                                );
                                to = to.offset(len_0 as isize);
                                from = from.offset(len_0 as isize);
                                if to >= toend {
                                    current_block = 14626083211295674653;
                                    break;
                                }
                                ch = if from < fromend {
                                    let fresh80 = from;
                                    from = from.offset(1);
                                    *(fresh80 as *mut libc::c_uchar) as libc::c_int
                                } else {
                                    saved_input = 0 as *mut libc::c_char;
                                    fromlen = (Some(get.expect("non-null function pointer")))
                                        .expect(
                                            "non-null function pointer",
                                        )(
                                        input_buffer.as_mut_ptr(),
                                        ::core::mem::size_of::<[libc::c_char; 32768]>()
                                            as libc::c_ulong,
                                    );
                                    from = input_buffer.as_mut_ptr();
                                    fromend = from.offset(fromlen as isize);
                                    if fromlen == 0 as libc::c_int as libc::c_ulong {
                                        -(1 as libc::c_int)
                                    } else {
                                        let fresh81 = from;
                                        from = from.offset(1);
                                        *(fresh81 as *mut libc::c_uchar) as libc::c_int
                                    }
                                };
                            }
                        }
                        current_block = 1203695113984090064;
                    }
                    6955753519599442332 => {
                        ch2 = if from < fromend {
                            let fresh65 = from;
                            from = from.offset(1);
                            *(fresh65 as *mut libc::c_uchar) as libc::c_int
                        } else {
                            saved_input = 0 as *mut libc::c_char;
                            fromlen = (Some(get.expect("non-null function pointer")))
                                .expect(
                                    "non-null function pointer",
                                )(
                                input_buffer.as_mut_ptr(),
                                ::core::mem::size_of::<[libc::c_char; 32768]>()
                                    as libc::c_ulong,
                            );
                            from = input_buffer.as_mut_ptr();
                            fromend = from.offset(fromlen as isize);
                            if fromlen == 0 as libc::c_int as libc::c_ulong {
                                -(1 as libc::c_int)
                            } else {
                                let fresh66 = from;
                                from = from.offset(1);
                                *(fresh66 as *mut libc::c_uchar) as libc::c_int
                            }
                        };
                        if ch2 == '*' as i32 {
                            old_state = 3 as libc::c_int;
                            state = -(2 as libc::c_int);
                            continue;
                        } else if ch2 != -(1 as libc::c_int) {
                            from = from.offset(-1);
                            *from = ch2 as libc::c_char;
                        }
                        current_block = 3066043723669372660;
                    }
                    12724295801459943709 => {
                        state = 0 as libc::c_int;
                        let fresh63 = to;
                        to = to.offset(1);
                        *fresh63 = ch as libc::c_char;
                        if to >= toend {
                            current_block = 14626083211295674653;
                            break;
                        } else {
                            continue;
                        }
                    }
                    16286683003977321678 => {
                        ch = if from < fromend {
                            let fresh53 = from;
                            from = from.offset(1);
                            *(fresh53 as *mut libc::c_uchar) as libc::c_int
                        } else {
                            saved_input = 0 as *mut libc::c_char;
                            fromlen = (Some(get.expect("non-null function pointer")))
                                .expect(
                                    "non-null function pointer",
                                )(
                                input_buffer.as_mut_ptr(),
                                ::core::mem::size_of::<[libc::c_char; 32768]>()
                                    as libc::c_ulong,
                            );
                            from = input_buffer.as_mut_ptr();
                            fromend = from.offset(fromlen as isize);
                            if fromlen == 0 as libc::c_int as libc::c_ulong {
                                -(1 as libc::c_int)
                            } else {
                                let fresh54 = from;
                                from = from.offset(1);
                                *(fresh54 as *mut libc::c_uchar) as libc::c_int
                            }
                        };
                        if ch == -(1 as libc::c_int) {
                            as_warn(
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"end of file after a one-character quote; \\0 inserted\0"
                                        as *const u8 as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                            );
                            ch = 0 as libc::c_int;
                        }
                        if ch == '\\' as i32 {
                            ch = if from < fromend {
                                let fresh55 = from;
                                from = from.offset(1);
                                *(fresh55 as *mut libc::c_uchar) as libc::c_int
                            } else {
                                saved_input = 0 as *mut libc::c_char;
                                fromlen = (Some(get.expect("non-null function pointer")))
                                    .expect(
                                        "non-null function pointer",
                                    )(
                                    input_buffer.as_mut_ptr(),
                                    ::core::mem::size_of::<[libc::c_char; 32768]>()
                                        as libc::c_ulong,
                                );
                                from = input_buffer.as_mut_ptr();
                                fromend = from.offset(fromlen as isize);
                                if fromlen == 0 as libc::c_int as libc::c_ulong {
                                    -(1 as libc::c_int)
                                } else {
                                    let fresh56 = from;
                                    from = from.offset(1);
                                    *(fresh56 as *mut libc::c_uchar) as libc::c_int
                                }
                            };
                            if ch == -(1 as libc::c_int) {
                                as_warn(
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"end of file in escape character\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                                ch = '\\' as i32;
                            } else {
                                ch = process_escape(ch);
                            }
                        }
                        sprintf(
                            out_buf.as_mut_ptr(),
                            b"%d\0" as *const u8 as *const libc::c_char,
                            ch as libc::c_uchar as libc::c_int,
                        );
                        ch = if from < fromend {
                            let fresh57 = from;
                            from = from.offset(1);
                            *(fresh57 as *mut libc::c_uchar) as libc::c_int
                        } else {
                            saved_input = 0 as *mut libc::c_char;
                            fromlen = (Some(get.expect("non-null function pointer")))
                                .expect(
                                    "non-null function pointer",
                                )(
                                input_buffer.as_mut_ptr(),
                                ::core::mem::size_of::<[libc::c_char; 32768]>()
                                    as libc::c_ulong,
                            );
                            from = input_buffer.as_mut_ptr();
                            fromend = from.offset(fromlen as isize);
                            if fromlen == 0 as libc::c_int as libc::c_ulong {
                                -(1 as libc::c_int)
                            } else {
                                let fresh58 = from;
                                from = from.offset(1);
                                *(fresh58 as *mut libc::c_uchar) as libc::c_int
                            }
                        };
                        if ch != '\'' as i32 {
                            if ch != -(1 as libc::c_int) {
                                from = from.offset(-1);
                                *from = ch as libc::c_char;
                            }
                        }
                        if strlen(out_buf.as_mut_ptr())
                            == 1 as libc::c_int as libc::c_ulong
                        {
                            let fresh59 = to;
                            to = to.offset(1);
                            *fresh59 = out_buf[0 as libc::c_int as usize];
                            if to >= toend {
                                current_block = 14626083211295674653;
                                break;
                            } else {
                                continue;
                            }
                        } else {
                            if state == 9 as libc::c_int {
                                old_state = 3 as libc::c_int;
                            } else {
                                old_state = state;
                            }
                            state = -(1 as libc::c_int);
                            out_string = out_buf.as_mut_ptr();
                            let fresh60 = out_string;
                            out_string = out_string.offset(1);
                            let fresh61 = to;
                            to = to.offset(1);
                            *fresh61 = *fresh60;
                            if to >= toend {
                                current_block = 14626083211295674653;
                                break;
                            } else {
                                continue;
                            }
                        }
                    }
                    4746208608892471902 => {
                        from = from.offset(-1);
                        *from = ch as libc::c_char;
                        state = 3 as libc::c_int;
                        let fresh52 = to;
                        to = to.offset(1);
                        *fresh52 = ' ' as i32 as libc::c_char;
                        if to >= toend {
                            current_block = 14626083211295674653;
                            break;
                        } else {
                            continue;
                        }
                    }
                    11510301253690344784 => {
                        from = from.offset(-1);
                        *from = ch as libc::c_char;
                        state = 3 as libc::c_int;
                        let fresh48 = to;
                        to = to.offset(1);
                        *fresh48 = ' ' as i32 as libc::c_char;
                        if to >= toend {
                            current_block = 14626083211295674653;
                            break;
                        }
                        ch = if from < fromend {
                            let fresh49 = from;
                            from = from.offset(1);
                            *(fresh49 as *mut libc::c_uchar) as libc::c_int
                        } else {
                            saved_input = 0 as *mut libc::c_char;
                            fromlen = (Some(get.expect("non-null function pointer")))
                                .expect(
                                    "non-null function pointer",
                                )(
                                input_buffer.as_mut_ptr(),
                                ::core::mem::size_of::<[libc::c_char; 32768]>()
                                    as libc::c_ulong,
                            );
                            from = input_buffer.as_mut_ptr();
                            fromend = from.offset(fromlen as isize);
                            if fromlen == 0 as libc::c_int as libc::c_ulong {
                                -(1 as libc::c_int)
                            } else {
                                let fresh50 = from;
                                from = from.offset(1);
                                *(fresh50 as *mut libc::c_uchar) as libc::c_int
                            }
                        };
                        old_state = 3 as libc::c_int;
                        current_block = 1995330570110937187;
                    }
                    14898553815918780345 => {
                        from = from.offset(-1);
                        *from = ch as libc::c_char;
                        state = 3 as libc::c_int;
                        let fresh78 = to;
                        to = to.offset(1);
                        *fresh78 = ' ' as i32 as libc::c_char;
                        if to >= toend {
                            current_block = 14626083211295674653;
                            break;
                        } else {
                            continue;
                        }
                    }
                    9728093949049737828 => {
                        if state == 9 as libc::c_int {
                            old_state = 3 as libc::c_int;
                        } else {
                            old_state = state;
                        }
                        current_block = 1995330570110937187;
                    }
                    _ => {}
                }
                match current_block {
                    3066043723669372660 => {
                        if state == 0 as libc::c_int || state == 1 as libc::c_int {
                            let mut startch: libc::c_int = 0;
                            startch = ch;
                            loop {
                                ch = if from < fromend {
                                    let fresh67 = from;
                                    from = from.offset(1);
                                    *(fresh67 as *mut libc::c_uchar) as libc::c_int
                                } else {
                                    saved_input = 0 as *mut libc::c_char;
                                    fromlen = (Some(get.expect("non-null function pointer")))
                                        .expect(
                                            "non-null function pointer",
                                        )(
                                        input_buffer.as_mut_ptr(),
                                        ::core::mem::size_of::<[libc::c_char; 32768]>()
                                            as libc::c_ulong,
                                    );
                                    from = input_buffer.as_mut_ptr();
                                    fromend = from.offset(fromlen as isize);
                                    if fromlen == 0 as libc::c_int as libc::c_ulong {
                                        -(1 as libc::c_int)
                                    } else {
                                        let fresh68 = from;
                                        from = from.offset(1);
                                        *(fresh68 as *mut libc::c_uchar) as libc::c_int
                                    }
                                };
                                if !(ch != -(1 as libc::c_int)
                                    && lex[ch as usize] as libc::c_int == 2 as libc::c_int)
                                {
                                    break;
                                }
                            }
                            if ch == -(1 as libc::c_int) {
                                as_warn(
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"end of file in comment; newline inserted\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                                let fresh69 = to;
                                to = to.offset(1);
                                *fresh69 = '\n' as i32 as libc::c_char;
                                if to >= toend {
                                    current_block = 14626083211295674653;
                                    break;
                                } else {
                                    continue;
                                }
                            } else if ch < '0' as i32 || ch > '9' as i32
                                || state != 0 as libc::c_int || startch != '#' as i32
                            {
                                while ch != -(1 as libc::c_int)
                                    && !(lex[ch as usize] as libc::c_int == 10 as libc::c_int)
                                {
                                    ch = if from < fromend {
                                        let fresh70 = from;
                                        from = from.offset(1);
                                        *(fresh70 as *mut libc::c_uchar) as libc::c_int
                                    } else {
                                        saved_input = 0 as *mut libc::c_char;
                                        fromlen = (Some(get.expect("non-null function pointer")))
                                            .expect(
                                                "non-null function pointer",
                                            )(
                                            input_buffer.as_mut_ptr(),
                                            ::core::mem::size_of::<[libc::c_char; 32768]>()
                                                as libc::c_ulong,
                                        );
                                        from = input_buffer.as_mut_ptr();
                                        fromend = from.offset(fromlen as isize);
                                        if fromlen == 0 as libc::c_int as libc::c_ulong {
                                            -(1 as libc::c_int)
                                        } else {
                                            let fresh71 = from;
                                            from = from.offset(1);
                                            *(fresh71 as *mut libc::c_uchar) as libc::c_int
                                        }
                                    };
                                }
                                if ch == -(1 as libc::c_int) {
                                    as_warn(
                                        dcgettext(
                                            0 as *const libc::c_char,
                                            b"end of file in comment; newline inserted\0" as *const u8
                                                as *const libc::c_char,
                                            5 as libc::c_int,
                                        ),
                                    );
                                    let fresh72 = to;
                                    to = to.offset(1);
                                    *fresh72 = '\n' as i32 as libc::c_char;
                                    if to >= toend {
                                        current_block = 14626083211295674653;
                                        break;
                                    }
                                } else {
                                    from = from.offset(-1);
                                    *from = ch as libc::c_char;
                                }
                                state = 0 as libc::c_int;
                                continue;
                            } else {
                                from = from.offset(-1);
                                *from = ch as libc::c_char;
                                old_state = 4 as libc::c_int;
                                state = -(1 as libc::c_int);
                                out_string = b"\t.linefile \0" as *const u8
                                    as *const libc::c_char;
                                let fresh73 = out_string;
                                out_string = out_string.offset(1);
                                let fresh74 = to;
                                to = to.offset(1);
                                *fresh74 = *fresh73;
                                if to >= toend {
                                    current_block = 14626083211295674653;
                                    break;
                                } else {
                                    continue;
                                }
                            }
                        } else if (strchr(i386_comment_chars, ch)).is_null()
                            && (0 as libc::c_int == 0
                                || ch != '!' as i32 && ch != '*' as i32)
                        {
                            current_block = 1203695113984090064;
                        } else if 0 as libc::c_int != 0
                            && (ch == '!' as i32 || ch == '*' as i32 || ch == '#' as i32)
                            && state != 1 as libc::c_int && state != 10 as libc::c_int
                        {
                            current_block = 1203695113984090064;
                        } else {
                            current_block = 15212124324706515022;
                        }
                    }
                    1995330570110937187 => {
                        state = 5 as libc::c_int;
                        let fresh51 = to;
                        to = to.offset(1);
                        *fresh51 = ch as libc::c_char;
                        if to >= toend {
                            current_block = 14626083211295674653;
                            break;
                        } else {
                            continue;
                        }
                    }
                    _ => {}
                }
                match current_block {
                    15212124324706515022 => {
                        if !((if to > tostart {
                            *to.offset(-(1 as libc::c_int) as isize) as libc::c_int
                        } else {
                            last_char as libc::c_int
                        }) == '\\' as i32)
                        {
                            loop {
                                ch = if from < fromend {
                                    let fresh75 = from;
                                    from = from.offset(1);
                                    *(fresh75 as *mut libc::c_uchar) as libc::c_int
                                } else {
                                    saved_input = 0 as *mut libc::c_char;
                                    fromlen = (Some(get.expect("non-null function pointer")))
                                        .expect(
                                            "non-null function pointer",
                                        )(
                                        input_buffer.as_mut_ptr(),
                                        ::core::mem::size_of::<[libc::c_char; 32768]>()
                                            as libc::c_ulong,
                                    );
                                    from = input_buffer.as_mut_ptr();
                                    fromend = from.offset(fromlen as isize);
                                    if fromlen == 0 as libc::c_int as libc::c_ulong {
                                        -(1 as libc::c_int)
                                    } else {
                                        let fresh76 = from;
                                        from = from.offset(1);
                                        *(fresh76 as *mut libc::c_uchar) as libc::c_int
                                    }
                                };
                                if !(ch != -(1 as libc::c_int)
                                    && !(lex[ch as usize] as libc::c_int == 10 as libc::c_int))
                                {
                                    break;
                                }
                            }
                            if ch == -(1 as libc::c_int) {
                                as_warn(
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"end of file in comment; newline inserted\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                            }
                            state = 0 as libc::c_int;
                            let fresh77 = to;
                            to = to.offset(1);
                            *fresh77 = '\n' as i32 as libc::c_char;
                            if to >= toend {
                                current_block = 14626083211295674653;
                                break;
                            } else {
                                continue;
                            }
                        }
                    }
                    _ => {}
                }
                if state == 0 as libc::c_int {
                    state = 11 as libc::c_int;
                } else if state == 1 as libc::c_int {
                    state = 2 as libc::c_int;
                } else if state == 9 as libc::c_int {
                    if !(lex[ch as usize] as libc::c_int == 1 as libc::c_int) {
                        state = 3 as libc::c_int;
                    }
                } else if state == 10 as libc::c_int {
                    if ch == '\\' as i32 {
                        if to.offset(1 as libc::c_int as isize) >= toend {
                            from = from.offset(-1);
                            *from = ch as libc::c_char;
                            current_block = 14626083211295674653;
                            break;
                        } else {
                            let fresh82 = to;
                            to = to.offset(1);
                            *fresh82 = ' ' as i32 as libc::c_char;
                        }
                    }
                    state = 3 as libc::c_int;
                }
                let fresh83 = to;
                to = to.offset(1);
                *fresh83 = ch as libc::c_char;
                if to >= toend {
                    current_block = 14626083211295674653;
                    break;
                }
            }
        }
    }
    match current_block {
        14626083211295674653 => {
            if fromend > from {
                saved_input = from;
                saved_input_len = fromend.offset_from(from) as libc::c_long as size_t;
            } else {
                saved_input = 0 as *mut libc::c_char;
            }
            if to > tostart {
                last_char = *to.offset(-(1 as libc::c_int) as isize);
            }
            return to.offset_from(tostart) as libc::c_long as size_t;
        }
        _ => {
            if to > tostart {
                last_char = *to.offset(-(1 as libc::c_int) as isize);
            }
            return to.offset_from(tostart) as libc::c_long as size_t;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn app_pop(mut arg: *mut libc::c_char) {
    let mut saved: *mut app_save = arg as *mut app_save;
    state = (*saved).state;
    old_state = (*saved).old_state;
    out_string = (*saved).out_string;
    memcpy(
        out_buf.as_mut_ptr() as *mut libc::c_void,
        ((*saved).out_buf).as_mut_ptr() as *const libc::c_void,
        ::core::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong,
    );
    add_newlines = (*saved).add_newlines;
    if ((*saved).saved_input).is_null() {
        saved_input = 0 as *mut libc::c_char;
    } else {
        if (*saved).saved_input_len
            <= ::core::mem::size_of::<[libc::c_char; 32768]>() as libc::c_ulong
        {} else {
            as_abort(
                b"app.c\0" as *const u8 as *const libc::c_char,
                302 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 21],
                    &[libc::c_char; 21],
                >(b"void app_pop(char *)\0"))
                    .as_ptr(),
            );
        };
        memcpy(
            input_buffer.as_mut_ptr() as *mut libc::c_void,
            (*saved).saved_input as *const libc::c_void,
            (*saved).saved_input_len,
        );
        saved_input = input_buffer.as_mut_ptr();
        saved_input_len = (*saved).saved_input_len;
        free((*saved).saved_input as *mut libc::c_void);
    }
    mri_state = (*saved).mri_state;
    mri_last_ch = (*saved).mri_last_ch;
    last_char = (*saved).last_char;
    free(arg as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn do_scrub_begin(mut _m68k_mri: libc::c_int) {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut c: libc::c_int = 0;
    lex[' ' as i32 as usize] = 2 as libc::c_int as libc::c_char;
    lex['\t' as i32 as usize] = 2 as libc::c_int as libc::c_char;
    lex['\r' as i32 as usize] = 2 as libc::c_int as libc::c_char;
    lex['\n' as i32 as usize] = 10 as libc::c_int as libc::c_char;
    lex[':' as i32 as usize] = 9 as libc::c_int as libc::c_char;
    lex['"' as i32 as usize] = 8 as libc::c_int as libc::c_char;
    lex['\'' as i32 as usize] = 11 as libc::c_int as libc::c_char;
    p = symbol_chars.as_ptr();
    while *p != 0 {
        lex[*p as libc::c_uchar as usize] = 1 as libc::c_int as libc::c_char;
        p = p.offset(1);
        p;
    }
    c = 128 as libc::c_int;
    while c < 256 as libc::c_int {
        lex[c as usize] = 1 as libc::c_int as libc::c_char;
        c += 1;
        c;
    }
    p = extra_symbol_chars.as_ptr();
    while *p != 0 {
        lex[*p as libc::c_uchar as usize] = 1 as libc::c_int as libc::c_char;
        p = p.offset(1);
        p;
    }
    p = i386_comment_chars;
    while *p != 0 {
        lex[*p as libc::c_uchar as usize] = 4 as libc::c_int as libc::c_char;
        p = p.offset(1);
        p;
    }
    p = line_comment_chars.as_ptr();
    while *p != 0 {
        lex[*p as libc::c_uchar as usize] = 5 as libc::c_int as libc::c_char;
        p = p.offset(1);
        p;
    }
    p = line_separator_chars.as_ptr();
    while *p != 0 {
        lex[*p as libc::c_uchar as usize] = 3 as libc::c_int as libc::c_char;
        p = p.offset(1);
        p;
    }
    if lex['/' as i32 as usize] as libc::c_int == 0 as libc::c_int {
        lex['/' as i32 as usize] = 6 as libc::c_int as libc::c_char;
    }
}
static mut last_char: libc::c_char = 0;
static mut lex: [libc::c_char; 256] = [0; 256];
static mut symbol_chars: [libc::c_char; 66] = unsafe {
    *::core::mem::transmute::<
        &[u8; 66],
        &[libc::c_char; 66],
    >(b"$._ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789\0")
};
unsafe extern "C" fn process_escape(mut ch: libc::c_int) -> libc::c_int {
    match ch {
        98 => return '\u{8}' as i32,
        102 => return '\u{c}' as i32,
        110 => return '\n' as i32,
        114 => return '\r' as i32,
        116 => return '\t' as i32,
        39 => return '\'' as i32,
        34 => return '"' as i32,
        _ => return ch,
    };
}
static mut state: libc::c_int = 0;
static mut old_state: libc::c_int = 0;
static mut out_string: *const libc::c_char = 0 as *const libc::c_char;
static mut out_buf: [libc::c_char; 20] = [0; 20];
static mut add_newlines: libc::c_int = 0;
static mut saved_input: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
static mut saved_input_len: size_t = 0;
static mut input_buffer: [libc::c_char; 32768] = [0; 32768];
static mut mri_state: *const libc::c_char = 0 as *const libc::c_char;
static mut mri_last_ch: libc::c_char = 0;
