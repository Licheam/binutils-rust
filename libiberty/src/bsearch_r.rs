use ::libc;
pub type size_t = libc::c_ulong;
#[no_mangle]
pub unsafe extern "C" fn bsearch_r(
    mut key: *const libc::c_void,
    mut base0: *const libc::c_void,
    mut nmemb: size_t,
    mut size: size_t,
    mut compar: Option::<
        unsafe extern "C" fn(
            *const libc::c_void,
            *const libc::c_void,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    mut arg: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut base: *const libc::c_char = base0 as *const libc::c_char;
    let mut lim: libc::c_int = 0;
    let mut cmp: libc::c_int = 0;
    let mut p: *const libc::c_void = 0 as *const libc::c_void;
    lim = nmemb as libc::c_int;
    while lim != 0 as libc::c_int {
        p = base
            .offset(
                ((lim >> 1 as libc::c_int) as libc::c_ulong).wrapping_mul(size) as isize,
            ) as *const libc::c_void;
        cmp = (Some(compar.expect("non-null function pointer")))
            .expect("non-null function pointer")(key, p, arg);
        if cmp == 0 as libc::c_int {
            return p as *mut libc::c_void;
        }
        if cmp > 0 as libc::c_int {
            base = (p as *const libc::c_char).offset(size as isize);
            lim -= 1;
            lim;
        }
        lim >>= 1 as libc::c_int;
    }
    return 0 as *mut libc::c_void;
}
