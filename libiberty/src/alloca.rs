use ::libc;
extern "C" {
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn abort() -> !;
}
pub type size_t = libc::c_ulong;
pub type header = hdr;
#[derive(Copy, Clone)]
#[repr(C)]
pub union hdr {
    pub align: [libc::c_char; 8],
    pub h: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub next: *mut hdr,
    pub deep: *mut libc::c_char,
}
#[no_mangle]
pub static mut libiberty_optr: *const libc::c_char = 0 as *const libc::c_char;
#[no_mangle]
pub static mut libiberty_nptr: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut libiberty_len: libc::c_ulong = 0;
static mut last_alloca_header: *mut header = 0 as *const header as *mut header;
#[no_mangle]
pub unsafe extern "C" fn C_alloca(mut size: size_t) -> *mut libc::c_void {
    let mut probe: libc::c_char = 0;
    let mut depth: *mut libc::c_char = &mut probe;
    let mut hp: *mut header = 0 as *mut header;
    hp = last_alloca_header;
    while !hp.is_null() {
        if !(1 as libc::c_int > 0 as libc::c_int && (*hp).h.deep > depth
            || (1 as libc::c_int) < 0 as libc::c_int && (*hp).h.deep < depth)
        {
            break;
        }
        let mut np: *mut header = (*hp).h.next;
        free(hp as *mut libc::c_void);
        hp = np;
    }
    last_alloca_header = hp;
    if size == 0 as libc::c_int as libc::c_ulong {
        return 0 as *mut libc::c_void;
    }
    let mut new_storage: *mut libc::c_void = xmalloc(
        (::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
            .wrapping_mul(
                (::core::mem::size_of::<header>() as libc::c_ulong).wrapping_add(size),
            ),
    ) as *mut libc::c_char as *mut libc::c_void;
    if new_storage.is_null() {
        abort();
    }
    let ref mut fresh0 = (*(new_storage as *mut header)).h.next;
    *fresh0 = last_alloca_header;
    let ref mut fresh1 = (*(new_storage as *mut header)).h.deep;
    *fresh1 = depth;
    last_alloca_header = new_storage as *mut header;
    return (new_storage as *mut libc::c_char)
        .offset(::core::mem::size_of::<header>() as libc::c_ulong as isize)
        as *mut libc::c_void;
}
