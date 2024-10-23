use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    static mut _xexit_cleanup: Option::<unsafe extern "C" fn() -> ()>;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xatexit {
    pub next: *mut xatexit,
    pub ind: libc::c_int,
    pub fns: [Option::<unsafe extern "C" fn() -> ()>; 32],
}
static mut xatexit_first: xatexit = xatexit {
    next: 0 as *const xatexit as *mut xatexit,
    ind: 0,
    fns: [None; 32],
};
static mut xatexit_head: *mut xatexit = unsafe {
    &xatexit_first as *const xatexit as *mut xatexit
};
#[no_mangle]
pub unsafe extern "C" fn xatexit(
    mut fn_0: Option::<unsafe extern "C" fn() -> ()>,
) -> libc::c_int {
    let mut p: *mut xatexit = 0 as *mut xatexit;
    if _xexit_cleanup.is_none() {
        _xexit_cleanup = Some(xatexit_cleanup as unsafe extern "C" fn() -> ());
    }
    p = xatexit_head;
    if (*p).ind >= 32 as libc::c_int {
        p = malloc(::core::mem::size_of::<xatexit>() as libc::c_ulong) as *mut xatexit;
        if p.is_null() {
            return -(1 as libc::c_int);
        }
        (*p).ind = 0 as libc::c_int;
        (*p).next = xatexit_head;
        xatexit_head = p;
    }
    let fresh0 = (*p).ind;
    (*p).ind = (*p).ind + 1;
    (*p).fns[fresh0 as usize] = fn_0;
    return 0 as libc::c_int;
}
unsafe extern "C" fn xatexit_cleanup() {
    let mut p: *mut xatexit = 0 as *mut xatexit;
    let mut n: libc::c_int = 0;
    p = xatexit_head;
    while !p.is_null() {
        n = (*p).ind;
        loop {
            n -= 1;
            if !(n >= 0 as libc::c_int) {
                break;
            }
            (Some(
                (*((*p).fns).as_mut_ptr().offset(n as isize))
                    .expect("non-null function pointer"),
            ))
                .expect("non-null function pointer")();
        }
        p = (*p).next;
    }
}
