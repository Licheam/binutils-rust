use ::libc;
extern "C" {
    fn abort() -> !;
    fn mktemp(__template: *mut libc::c_char) -> *mut libc::c_char;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn choose_tmpdir() -> *const libc::c_char;
    fn xmalloc(_: size_t) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
#[no_mangle]
pub unsafe extern "C" fn choose_temp_base() -> *mut libc::c_char {
    let mut base: *const libc::c_char = choose_tmpdir();
    let mut temp_filename: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: libc::c_int = 0;
    len = strlen(base) as libc::c_int;
    temp_filename = xmalloc(
        (::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
            .wrapping_mul(
                (len as libc::c_ulong)
                    .wrapping_add(
                        (::core::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    )
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            ),
    ) as *mut libc::c_char;
    strcpy(temp_filename, base);
    strcpy(
        temp_filename.offset(len as isize),
        b"ccXXXXXX\0" as *const u8 as *const libc::c_char,
    );
    if (mktemp(temp_filename)).is_null() {
        abort();
    }
    return temp_filename;
}
