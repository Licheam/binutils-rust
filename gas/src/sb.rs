extern "C" {
    fn free(_: *mut libc::c_void);
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    fn xrealloc(_: *mut libc::c_void, _: size_t) -> *mut libc::c_void;
    fn as_fatal(format: *const libc::c_char, _: ...) -> !;
    fn do_scrub_chars(
        get: Option::<unsafe extern "C" fn(*mut libc::c_char, size_t) -> size_t>,
        _: *mut libc::c_char,
        _: size_t,
    ) -> size_t;
}
pub type size_t = libc::c_ulong;
pub type __ssize_t = libc::c_long;
pub type ssize_t = __ssize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sb {
    pub ptr: *mut libc::c_char,
    pub len: size_t,
    pub max: size_t,
}
#[no_mangle]
pub unsafe extern "C" fn sb_new(mut ptr: *mut sb) {
    sb_build(
        ptr,
        (64 as libc::c_int as libc::c_ulong)
            .wrapping_sub(
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<size_t>() as libc::c_ulong),
            )
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
}
#[no_mangle]
pub unsafe extern "C" fn sb_build(mut ptr: *mut sb, mut size: size_t) {
    (*ptr)
        .ptr = xmalloc(
        (::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
            .wrapping_mul(size.wrapping_add(1 as libc::c_int as libc::c_ulong)),
    ) as *mut libc::c_char;
    (*ptr).max = size;
    (*ptr).len = 0 as libc::c_int as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn sb_kill(mut ptr: *mut sb) {
    free((*ptr).ptr as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn sb_add_sb(mut ptr: *mut sb, mut s: *mut sb) {
    sb_check(ptr, (*s).len);
    memcpy(
        ((*ptr).ptr).offset((*ptr).len as isize) as *mut libc::c_void,
        (*s).ptr as *const libc::c_void,
        (*s).len,
    );
    (*ptr)
        .len = ((*ptr).len as libc::c_ulong).wrapping_add((*s).len) as size_t as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn sb_scrub_and_add_sb(mut ptr: *mut sb, mut s: *mut sb) {
    sb_to_scrub = s;
    scrub_position = (*s).ptr;
    sb_check(ptr, (*s).len);
    (*ptr)
        .len = ((*ptr).len as libc::c_ulong)
        .wrapping_add(
            do_scrub_chars(
                Some(
                    scrub_from_sb
                        as unsafe extern "C" fn(*mut libc::c_char, size_t) -> size_t,
                ),
                ((*ptr).ptr).offset((*ptr).len as isize),
                (*s).len,
            ),
        ) as size_t as size_t;
    sb_to_scrub = 0 as *mut sb;
    scrub_position = 0 as *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn sb_reset(mut ptr: *mut sb) {
    (*ptr).len = 0 as libc::c_int as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn sb_add_char(mut ptr: *mut sb, mut c: size_t) {
    sb_check(ptr, 1 as libc::c_int as size_t);
    let fresh0 = (*ptr).len;
    (*ptr).len = ((*ptr).len).wrapping_add(1);
    *((*ptr).ptr).offset(fresh0 as isize) = c as libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn sb_add_string(mut ptr: *mut sb, mut s: *const libc::c_char) {
    let mut len: size_t = strlen(s);
    sb_check(ptr, len);
    memcpy(
        ((*ptr).ptr).offset((*ptr).len as isize) as *mut libc::c_void,
        s as *const libc::c_void,
        len,
    );
    (*ptr).len = ((*ptr).len as libc::c_ulong).wrapping_add(len) as size_t as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn sb_add_buffer(
    mut ptr: *mut sb,
    mut s: *const libc::c_char,
    mut len: size_t,
) {
    sb_check(ptr, len);
    memcpy(
        ((*ptr).ptr).offset((*ptr).len as isize) as *mut libc::c_void,
        s as *const libc::c_void,
        len,
    );
    (*ptr).len = ((*ptr).len as libc::c_ulong).wrapping_add(len) as size_t as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn sb_terminate(mut in_0: *mut sb) -> *mut libc::c_char {
    *((*in_0).ptr).offset((*in_0).len as isize) = 0 as libc::c_int as libc::c_char;
    return (*in_0).ptr;
}
#[no_mangle]
pub unsafe extern "C" fn sb_skip_white(mut idx: size_t, mut ptr: *mut sb) -> size_t {
    while idx < (*ptr).len
        && (*((*ptr).ptr).offset(idx as isize) as libc::c_int == ' ' as i32
            || *((*ptr).ptr).offset(idx as isize) as libc::c_int == '\t' as i32)
    {
        idx = idx.wrapping_add(1);
        idx;
    }
    return idx;
}
#[no_mangle]
pub unsafe extern "C" fn sb_skip_comma(mut idx: size_t, mut ptr: *mut sb) -> size_t {
    while idx < (*ptr).len
        && (*((*ptr).ptr).offset(idx as isize) as libc::c_int == ' ' as i32
            || *((*ptr).ptr).offset(idx as isize) as libc::c_int == '\t' as i32)
    {
        idx = idx.wrapping_add(1);
        idx;
    }
    if idx < (*ptr).len
        && *((*ptr).ptr).offset(idx as isize) as libc::c_int == ',' as i32
    {
        idx = idx.wrapping_add(1);
        idx;
    }
    while idx < (*ptr).len
        && (*((*ptr).ptr).offset(idx as isize) as libc::c_int == ' ' as i32
            || *((*ptr).ptr).offset(idx as isize) as libc::c_int == '\t' as i32)
    {
        idx = idx.wrapping_add(1);
        idx;
    }
    return idx;
}
unsafe extern "C" fn sb_check(mut ptr: *mut sb, mut len: size_t) {
    let mut want: size_t = ((*ptr).len).wrapping_add(len);
    if want > (*ptr).max {
        let mut max: size_t = 0;
        want = (want as libc::c_ulong)
            .wrapping_add(
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<size_t>() as libc::c_ulong)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            ) as size_t as size_t;
        if (want as ssize_t) < 0 as libc::c_int as libc::c_long {
            as_fatal(b"string buffer overflow\0" as *const u8 as *const libc::c_char);
        }
        max = (1 as libc::c_int as size_t)
            << (8 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<size_t>() as libc::c_ulong)
                .wrapping_sub(
                    (if ::core::mem::size_of::<size_t>() as libc::c_ulong
                        <= ::core::mem::size_of::<libc::c_long>() as libc::c_ulong
                    {
                        (want as libc::c_long as libc::c_ulong).leading_zeros() as i32
                    } else {
                        (want as libc::c_longlong as libc::c_ulonglong).leading_zeros()
                            as i32
                    }) as libc::c_ulong,
                );
        max = (max as libc::c_ulong)
            .wrapping_sub(
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<size_t>() as libc::c_ulong)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            ) as size_t as size_t;
        (*ptr).max = max;
        (*ptr)
            .ptr = xrealloc(
            (*ptr).ptr as *mut libc::c_void,
            (::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
                .wrapping_mul(max.wrapping_add(1 as libc::c_int as libc::c_ulong)),
        ) as *mut libc::c_char;
    }
}
static mut sb_to_scrub: *mut sb = 0 as *const sb as *mut sb;
static mut scrub_position: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
unsafe extern "C" fn scrub_from_sb(
    mut buf: *mut libc::c_char,
    mut buflen: size_t,
) -> size_t {
    let mut copy: size_t = 0;
    copy = ((*sb_to_scrub).len)
        .wrapping_sub(
            scrub_position.offset_from((*sb_to_scrub).ptr) as libc::c_long
                as libc::c_ulong,
        );
    if copy > buflen {
        copy = buflen;
    }
    memcpy(buf as *mut libc::c_void, scrub_position as *const libc::c_void, copy);
    scrub_position = scrub_position.offset(copy as isize);
    return copy;
}
