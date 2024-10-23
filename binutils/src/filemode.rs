#[no_mangle]
pub unsafe extern "C" fn mode_string(
    mut mode: libc::c_ulong,
    mut str: *mut libc::c_char,
) {
    *str.offset(0 as libc::c_int as isize) = ftypelet(mode);
    *str
        .offset(
            1 as libc::c_int as isize,
        ) = (if mode & 0o400 as libc::c_int as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        'r' as i32
    } else {
        '-' as i32
    }) as libc::c_char;
    *str
        .offset(
            2 as libc::c_int as isize,
        ) = (if mode & 0o200 as libc::c_int as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        'w' as i32
    } else {
        '-' as i32
    }) as libc::c_char;
    *str
        .offset(
            3 as libc::c_int as isize,
        ) = (if mode & 0o100 as libc::c_int as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        'x' as i32
    } else {
        '-' as i32
    }) as libc::c_char;
    *str
        .offset(
            4 as libc::c_int as isize,
        ) = (if mode & (0o400 as libc::c_int >> 3 as libc::c_int) as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        'r' as i32
    } else {
        '-' as i32
    }) as libc::c_char;
    *str
        .offset(
            5 as libc::c_int as isize,
        ) = (if mode & (0o200 as libc::c_int >> 3 as libc::c_int) as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        'w' as i32
    } else {
        '-' as i32
    }) as libc::c_char;
    *str
        .offset(
            6 as libc::c_int as isize,
        ) = (if mode & (0o100 as libc::c_int >> 3 as libc::c_int) as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        'x' as i32
    } else {
        '-' as i32
    }) as libc::c_char;
    *str
        .offset(
            7 as libc::c_int as isize,
        ) = (if mode
        & (0o400 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int) as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        'r' as i32
    } else {
        '-' as i32
    }) as libc::c_char;
    *str
        .offset(
            8 as libc::c_int as isize,
        ) = (if mode
        & (0o200 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int) as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        'w' as i32
    } else {
        '-' as i32
    }) as libc::c_char;
    *str
        .offset(
            9 as libc::c_int as isize,
        ) = (if mode
        & (0o100 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int) as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        'x' as i32
    } else {
        '-' as i32
    }) as libc::c_char;
    setst(mode, str);
}
unsafe extern "C" fn ftypelet(mut bits: libc::c_ulong) -> libc::c_char {
    if bits & 0o170000 as libc::c_int as libc::c_ulong
        == 0o40000 as libc::c_int as libc::c_ulong
    {
        return 'd' as i32 as libc::c_char;
    }
    if bits & 0o170000 as libc::c_int as libc::c_ulong
        == 0o120000 as libc::c_int as libc::c_ulong
    {
        return 'l' as i32 as libc::c_char;
    }
    if bits & 0o170000 as libc::c_int as libc::c_ulong
        == 0o60000 as libc::c_int as libc::c_ulong
    {
        return 'b' as i32 as libc::c_char;
    }
    if bits & 0o170000 as libc::c_int as libc::c_ulong
        == 0o20000 as libc::c_int as libc::c_ulong
    {
        return 'c' as i32 as libc::c_char;
    }
    if bits & 0o170000 as libc::c_int as libc::c_ulong
        == 0o140000 as libc::c_int as libc::c_ulong
    {
        return 's' as i32 as libc::c_char;
    }
    if bits & 0o170000 as libc::c_int as libc::c_ulong
        == 0o10000 as libc::c_int as libc::c_ulong
    {
        return 'p' as i32 as libc::c_char;
    }
    return '-' as i32 as libc::c_char;
}
unsafe extern "C" fn setst(mut bits: libc::c_ulong, mut chars: *mut libc::c_char) {
    if bits & 0o4000 as libc::c_int as libc::c_ulong != 0 {
        if *chars.offset(3 as libc::c_int as isize) as libc::c_int != 'x' as i32 {
            *chars.offset(3 as libc::c_int as isize) = 'S' as i32 as libc::c_char;
        } else {
            *chars.offset(3 as libc::c_int as isize) = 's' as i32 as libc::c_char;
        }
    }
    if bits & 0o2000 as libc::c_int as libc::c_ulong != 0 {
        if *chars.offset(6 as libc::c_int as isize) as libc::c_int != 'x' as i32 {
            *chars.offset(6 as libc::c_int as isize) = 'S' as i32 as libc::c_char;
        } else {
            *chars.offset(6 as libc::c_int as isize) = 's' as i32 as libc::c_char;
        }
    }
    if bits & 0o1000 as libc::c_int as libc::c_ulong != 0 {
        if *chars.offset(9 as libc::c_int as isize) as libc::c_int != 'x' as i32 {
            *chars.offset(9 as libc::c_int as isize) = 'T' as i32 as libc::c_char;
        } else {
            *chars.offset(9 as libc::c_int as isize) = 't' as i32 as libc::c_char;
        }
    }
}
