use ::libc;
extern "C" {
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strtoul(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_ulong;
    fn abs(_: libc::c_int) -> libc::c_int;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type va_list = __builtin_va_list;
#[no_mangle]
pub unsafe extern "C" fn libiberty_vprintf_buffer_size(
    mut format: *const libc::c_char,
    mut args: ::core::ffi::VaList,
) -> libc::c_int {
    let mut p: *const libc::c_char = format;
    let mut total_width: libc::c_int = (strlen(format))
        .wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int;
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    while *p as libc::c_int != '\0' as i32 {
        let fresh0 = p;
        p = p.offset(1);
        if *fresh0 as libc::c_int == '%' as i32 {
            while !(strchr(
                b"-+ #0\0" as *const u8 as *const libc::c_char,
                *p as libc::c_int,
            ))
                .is_null()
            {
                p = p.offset(1);
                p;
            }
            if *p as libc::c_int == '*' as i32 {
                p = p.offset(1);
                p;
                total_width += abs(ap.arg::<libc::c_int>());
            } else {
                total_width = (total_width as libc::c_ulong)
                    .wrapping_add(
                        strtoul(
                            p,
                            &mut p as *mut *const libc::c_char as *mut *mut libc::c_char,
                            10 as libc::c_int,
                        ),
                    ) as libc::c_int as libc::c_int;
            }
            if *p as libc::c_int == '.' as i32 {
                p = p.offset(1);
                p;
                if *p as libc::c_int == '*' as i32 {
                    p = p.offset(1);
                    p;
                    total_width += abs(ap.arg::<libc::c_int>());
                } else {
                    total_width = (total_width as libc::c_ulong)
                        .wrapping_add(
                            strtoul(
                                p,
                                &mut p as *mut *const libc::c_char
                                    as *mut *mut libc::c_char,
                                10 as libc::c_int,
                            ),
                        ) as libc::c_int as libc::c_int;
                }
            }
            while !(strchr(
                b"hlL\0" as *const u8 as *const libc::c_char,
                *p as libc::c_int,
            ))
                .is_null()
            {
                p = p.offset(1);
                p;
            }
            total_width += 30 as libc::c_int;
            match *p as libc::c_int {
                100 | 105 | 111 | 117 | 120 | 88 | 99 => {
                    ap.arg::<libc::c_int>();
                }
                102 | 101 | 69 | 103 | 71 => {
                    ap.arg::<libc::c_double>();
                    total_width += 307 as libc::c_int;
                }
                115 => {
                    total_width = (total_width as libc::c_ulong)
                        .wrapping_add(strlen(ap.arg::<*mut libc::c_char>()))
                        as libc::c_int as libc::c_int;
                }
                112 | 110 => {
                    ap.arg::<*mut libc::c_char>();
                }
                _ => {}
            }
            p = p.offset(1);
            p;
        }
    }
    return total_width;
}
