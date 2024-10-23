use ::libc;
extern "C" {
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn free(_: *mut libc::c_void);
    static _sch_tolower: [libc::c_uchar; 256];
    fn lrealpath(_: *const libc::c_char) -> *mut libc::c_char;
}
pub type size_t = libc::c_ulong;
pub type hashval_t = libc::c_uint;
#[no_mangle]
pub unsafe extern "C" fn filename_cmp(
    mut s1: *const libc::c_char,
    mut s2: *const libc::c_char,
) -> libc::c_int {
    return strcmp(s1, s2);
}
#[no_mangle]
pub unsafe extern "C" fn filename_ncmp(
    mut s1: *const libc::c_char,
    mut s2: *const libc::c_char,
    mut n: size_t,
) -> libc::c_int {
    return strncmp(s1, s2, n);
}
#[no_mangle]
pub unsafe extern "C" fn filename_hash(mut s: *const libc::c_void) -> hashval_t {
    let mut str: *const libc::c_uchar = s as *const libc::c_uchar;
    let mut r: hashval_t = 0 as libc::c_int as hashval_t;
    let mut c: libc::c_uchar = 0;
    loop {
        let fresh0 = str;
        str = str.offset(1);
        c = *fresh0;
        if !(c as libc::c_int != 0 as libc::c_int) {
            break;
        }
        if c as libc::c_int == '\\' as i32 {
            c = '/' as i32 as libc::c_uchar;
        }
        c = _sch_tolower[(c as libc::c_int & 0xff as libc::c_int) as usize];
        r = r
            .wrapping_mul(67 as libc::c_int as libc::c_uint)
            .wrapping_add(c as libc::c_uint)
            .wrapping_sub(113 as libc::c_int as libc::c_uint);
    }
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn filename_eq(
    mut s1: *const libc::c_void,
    mut s2: *const libc::c_void,
) -> libc::c_int {
    return (filename_cmp(s1 as *const libc::c_char, s2 as *const libc::c_char)
        == 0 as libc::c_int) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn canonical_filename_eq(
    mut a: *const libc::c_char,
    mut b: *const libc::c_char,
) -> libc::c_int {
    let mut ca: *mut libc::c_char = lrealpath(a);
    let mut cb: *mut libc::c_char = lrealpath(b);
    let mut res: libc::c_int = filename_eq(
        ca as *const libc::c_void,
        cb as *const libc::c_void,
    );
    free(ca as *mut libc::c_void);
    free(cb as *mut libc::c_void);
    return res;
}
