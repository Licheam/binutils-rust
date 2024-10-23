use ::libc;
extern "C" {
    fn sha1_finish_ctx(
        ctx: *mut sha1_ctx,
        resbuf: *mut libc::c_void,
    ) -> *mut libc::c_void;
}
pub type __uint32_t = libc::c_uint;
pub type size_t = libc::c_ulong;
pub type uint32_t = __uint32_t;
pub type sha1_uint32 = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sha1_ctx {
    pub A: sha1_uint32,
    pub B: sha1_uint32,
    pub C: sha1_uint32,
    pub D: sha1_uint32,
    pub E: sha1_uint32,
    pub total: [sha1_uint32; 2],
    pub buflen: sha1_uint32,
    pub buffer: [sha1_uint32; 32],
}
pub type ctf_sha1_t = sha1_ctx;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub align: uint32_t,
    pub digest: [libc::c_uchar; 21],
}
static mut hex: [libc::c_char; 17] = unsafe {
    *::core::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"0123456789abcdef\0")
};
#[no_mangle]
pub unsafe extern "C" fn ctf_sha1_fini(
    mut sha1: *mut ctf_sha1_t,
    mut buf: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut i: size_t = 0;
    let mut align: C2RustUnnamed = C2RustUnnamed { align: 0 };
    sha1_finish_ctx(sha1, (align.digest).as_mut_ptr() as *mut libc::c_void);
    if buf.is_null() {
        return 0 as *mut libc::c_char;
    }
    *buf
        .offset(
            (41 as libc::c_int - 1 as libc::c_int) as isize,
        ) = '\0' as i32 as libc::c_char;
    i = 0 as libc::c_int as size_t;
    while i
        < ((41 as libc::c_int - 1 as libc::c_int) / 2 as libc::c_int) as libc::c_ulong
    {
        *buf
            .offset(
                (2 as libc::c_int as libc::c_ulong).wrapping_mul(i) as isize,
            ) = hex[(align.digest[i as usize] as libc::c_int >> 4 as libc::c_int)
            as usize];
        *buf
            .offset(
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(i)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
            ) = hex[(align.digest[i as usize] as libc::c_int & 0xf as libc::c_int)
            as usize];
        i = i.wrapping_add(1);
        i;
    }
    return buf;
}
