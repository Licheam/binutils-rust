extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn exit(_: libc::c_int) -> !;
}
#[no_mangle]
pub unsafe extern "C" fn print_version(mut name: *const libc::c_char) {
    printf(
        b"GNU %s %s\n\0" as *const u8 as *const libc::c_char,
        name,
        b"(GNU Binutils) 2.37\0" as *const u8 as *const libc::c_char,
    );
    printf(
        dcgettext(
            0 as *const libc::c_char,
            b"Copyright (C) 2021 Free Software Foundation, Inc.\n\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    printf(
        dcgettext(
            0 as *const libc::c_char,
            b"This program is free software; you may redistribute it under the terms of\nthe GNU General Public License version 3 or (at your option) any later version.\nThis program has absolutely no warranty.\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    exit(0 as libc::c_int);
}
