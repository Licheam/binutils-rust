use ::libc;
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn free(_: *mut libc::c_void);
    fn abort() -> !;
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    fn xrealloc(_: *mut libc::c_void, _: size_t) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dyn_string {
    pub allocated: libc::c_int,
    pub length: libc::c_int,
    pub s: *mut libc::c_char,
}
pub type dyn_string_t = *mut dyn_string;
#[no_mangle]
pub unsafe extern "C" fn dyn_string_init(
    mut ds_struct_ptr: *mut dyn_string,
    mut space: libc::c_int,
) -> libc::c_int {
    if space == 0 as libc::c_int {
        space = 1 as libc::c_int;
    }
    (*ds_struct_ptr)
        .s = xmalloc(
        (::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
            .wrapping_mul(space as libc::c_ulong),
    ) as *mut libc::c_char;
    (*ds_struct_ptr).allocated = space;
    (*ds_struct_ptr).length = 0 as libc::c_int;
    *((*ds_struct_ptr).s)
        .offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dyn_string_new(mut space: libc::c_int) -> dyn_string_t {
    let mut result: dyn_string_t = 0 as *mut dyn_string;
    result = xmalloc(::core::mem::size_of::<dyn_string>() as libc::c_ulong)
        as *mut dyn_string;
    dyn_string_init(result, space);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn dyn_string_delete(mut ds: dyn_string_t) {
    free((*ds).s as *mut libc::c_void);
    free(ds as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn dyn_string_release(mut ds: dyn_string_t) -> *mut libc::c_char {
    let mut result: *mut libc::c_char = (*ds).s;
    (*ds).s = 0 as *mut libc::c_char;
    free(ds as *mut libc::c_void);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn dyn_string_resize(
    mut ds: dyn_string_t,
    mut space: libc::c_int,
) -> dyn_string_t {
    let mut new_allocated: libc::c_int = (*ds).allocated;
    space += 1;
    space;
    while space > new_allocated {
        new_allocated *= 2 as libc::c_int;
    }
    if new_allocated != (*ds).allocated {
        (*ds).allocated = new_allocated;
        (*ds)
            .s = xrealloc(
            (*ds).s as *mut libc::c_void,
            (::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
                .wrapping_mul((*ds).allocated as libc::c_ulong),
        ) as *mut libc::c_char;
    }
    return ds;
}
#[no_mangle]
pub unsafe extern "C" fn dyn_string_clear(mut ds: dyn_string_t) {
    *((*ds).s).offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    (*ds).length = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dyn_string_copy(
    mut dest: dyn_string_t,
    mut src: dyn_string_t,
) -> libc::c_int {
    if dest == src {
        abort();
    }
    if (dyn_string_resize(dest, (*src).length)).is_null() {
        return 0 as libc::c_int;
    }
    strcpy((*dest).s, (*src).s);
    (*dest).length = (*src).length;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dyn_string_copy_cstr(
    mut dest: dyn_string_t,
    mut src: *const libc::c_char,
) -> libc::c_int {
    let mut length: libc::c_int = strlen(src) as libc::c_int;
    if (dyn_string_resize(dest, length)).is_null() {
        return 0 as libc::c_int;
    }
    strcpy((*dest).s, src);
    (*dest).length = length;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dyn_string_prepend(
    mut dest: dyn_string_t,
    mut src: dyn_string_t,
) -> libc::c_int {
    return dyn_string_insert(dest, 0 as libc::c_int, src);
}
#[no_mangle]
pub unsafe extern "C" fn dyn_string_prepend_cstr(
    mut dest: dyn_string_t,
    mut src: *const libc::c_char,
) -> libc::c_int {
    return dyn_string_insert_cstr(dest, 0 as libc::c_int, src);
}
#[no_mangle]
pub unsafe extern "C" fn dyn_string_insert(
    mut dest: dyn_string_t,
    mut pos: libc::c_int,
    mut src: dyn_string_t,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    if src == dest {
        abort();
    }
    if (dyn_string_resize(dest, (*dest).length + (*src).length)).is_null() {
        return 0 as libc::c_int;
    }
    i = (*dest).length;
    while i >= pos {
        *((*dest).s)
            .offset((i + (*src).length) as isize) = *((*dest).s).offset(i as isize);
        i -= 1;
        i;
    }
    strncpy(((*dest).s).offset(pos as isize), (*src).s, (*src).length as libc::c_ulong);
    (*dest).length += (*src).length;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dyn_string_insert_cstr(
    mut dest: dyn_string_t,
    mut pos: libc::c_int,
    mut src: *const libc::c_char,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut length: libc::c_int = strlen(src) as libc::c_int;
    if (dyn_string_resize(dest, (*dest).length + length)).is_null() {
        return 0 as libc::c_int;
    }
    i = (*dest).length;
    while i >= pos {
        *((*dest).s).offset((i + length) as isize) = *((*dest).s).offset(i as isize);
        i -= 1;
        i;
    }
    memcpy(
        ((*dest).s).offset(pos as isize) as *mut libc::c_void,
        src as *const libc::c_void,
        length as libc::c_ulong,
    );
    (*dest).length += length;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dyn_string_insert_char(
    mut dest: dyn_string_t,
    mut pos: libc::c_int,
    mut c: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    if (dyn_string_resize(dest, (*dest).length + 1 as libc::c_int)).is_null() {
        return 0 as libc::c_int;
    }
    i = (*dest).length;
    while i >= pos {
        *((*dest).s)
            .offset((i + 1 as libc::c_int) as isize) = *((*dest).s).offset(i as isize);
        i -= 1;
        i;
    }
    *((*dest).s).offset(pos as isize) = c as libc::c_char;
    (*dest).length += 1;
    (*dest).length;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dyn_string_append(
    mut dest: dyn_string_t,
    mut s: dyn_string_t,
) -> libc::c_int {
    if (dyn_string_resize(dest, (*dest).length + (*s).length)).is_null() {
        return 0 as libc::c_int;
    }
    strcpy(((*dest).s).offset((*dest).length as isize), (*s).s);
    (*dest).length += (*s).length;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dyn_string_append_cstr(
    mut dest: dyn_string_t,
    mut s: *const libc::c_char,
) -> libc::c_int {
    let mut len: libc::c_int = strlen(s) as libc::c_int;
    if (dyn_string_resize(dest, (*dest).length + len)).is_null() {
        return 0 as libc::c_int;
    }
    strcpy(((*dest).s).offset((*dest).length as isize), s);
    (*dest).length += len;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dyn_string_append_char(
    mut dest: dyn_string_t,
    mut c: libc::c_int,
) -> libc::c_int {
    if (dyn_string_resize(dest, (*dest).length + 1 as libc::c_int)).is_null() {
        return 0 as libc::c_int;
    }
    *((*dest).s).offset((*dest).length as isize) = c as libc::c_char;
    *((*dest).s)
        .offset(
            ((*dest).length + 1 as libc::c_int) as isize,
        ) = '\0' as i32 as libc::c_char;
    (*dest).length += 1;
    (*dest).length;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dyn_string_substring(
    mut dest: dyn_string_t,
    mut src: dyn_string_t,
    mut start: libc::c_int,
    mut end: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut length: libc::c_int = end - start;
    if start > end || start > (*src).length || end > (*src).length {
        abort();
    }
    if (dyn_string_resize(dest, length)).is_null() {
        return 0 as libc::c_int;
    }
    i = length;
    loop {
        i -= 1;
        if !(i >= 0 as libc::c_int) {
            break;
        }
        *((*dest).s).offset(i as isize) = *((*src).s).offset((start + i) as isize);
    }
    *((*dest).s).offset(length as isize) = '\0' as i32 as libc::c_char;
    (*dest).length = length;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dyn_string_eq(
    mut ds1: dyn_string_t,
    mut ds2: dyn_string_t,
) -> libc::c_int {
    if (*ds1).length != (*ds2).length {
        return 0 as libc::c_int
    } else {
        return (strcmp((*ds1).s, (*ds2).s) == 0) as libc::c_int
    };
}
