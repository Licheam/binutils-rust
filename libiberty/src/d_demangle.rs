use ::libc;
extern "C" {
    static _sch_istable: [libc::c_ushort; 256];
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn free(_: *mut libc::c_void);
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    fn xrealloc(_: *mut libc::c_void, _: size_t) -> *mut libc::c_void;
}
pub type C2RustUnnamed = libc::c_uint;
pub const _sch_isbasic: C2RustUnnamed = 3088;
pub const _sch_iscppsp: C2RustUnnamed = 3072;
pub const _sch_isgraph: C2RustUnnamed = 172;
pub const _sch_isidnum: C2RustUnnamed = 516;
pub const _sch_isalnum: C2RustUnnamed = 140;
pub const _sch_isalpha: C2RustUnnamed = 136;
pub const _sch_isnvsp: C2RustUnnamed = 2048;
pub const _sch_isvsp: C2RustUnnamed = 1024;
pub const _sch_isidst: C2RustUnnamed = 512;
pub const _sch_isxdigit: C2RustUnnamed = 256;
pub const _sch_isupper: C2RustUnnamed = 128;
pub const _sch_isspace: C2RustUnnamed = 64;
pub const _sch_ispunct: C2RustUnnamed = 32;
pub const _sch_isprint: C2RustUnnamed = 16;
pub const _sch_islower: C2RustUnnamed = 8;
pub const _sch_isdigit: C2RustUnnamed = 4;
pub const _sch_iscntrl: C2RustUnnamed = 2;
pub const _sch_isblank: C2RustUnnamed = 1;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct string {
    pub b: *mut libc::c_char,
    pub p: *mut libc::c_char,
    pub e: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dlang_info {
    pub s: *const libc::c_char,
    pub last_backref: libc::c_int,
}
unsafe extern "C" fn string_need(mut s: *mut string, mut n: size_t) {
    let mut tem: size_t = 0;
    if ((*s).b).is_null() {
        if n < 32 as libc::c_int as libc::c_ulong {
            n = 32 as libc::c_int as size_t;
        }
        (*s)
            .b = xmalloc(
            (::core::mem::size_of::<libc::c_char>() as libc::c_ulong).wrapping_mul(n),
        ) as *mut libc::c_char;
        (*s).p = (*s).b;
        (*s).e = ((*s).b).offset(n as isize);
    } else if (((*s).e).offset_from((*s).p) as libc::c_long as size_t) < n {
        tem = ((*s).p).offset_from((*s).b) as libc::c_long as size_t;
        n = (n as libc::c_ulong).wrapping_add(tem) as size_t as size_t;
        n = (n as libc::c_ulong).wrapping_mul(2 as libc::c_int as libc::c_ulong)
            as size_t as size_t;
        (*s)
            .b = xrealloc(
            (*s).b as *mut libc::c_void,
            (::core::mem::size_of::<libc::c_char>() as libc::c_ulong).wrapping_mul(n),
        ) as *mut libc::c_char;
        (*s).p = ((*s).b).offset(tem as isize);
        (*s).e = ((*s).b).offset(n as isize);
    }
}
unsafe extern "C" fn string_delete(mut s: *mut string) {
    if !((*s).b).is_null() {
        free((*s).b as *mut libc::c_void);
        (*s).p = 0 as *mut libc::c_char;
        (*s).e = (*s).p;
        (*s).b = (*s).e;
    }
}
unsafe extern "C" fn string_init(mut s: *mut string) {
    (*s).e = 0 as *mut libc::c_char;
    (*s).p = (*s).e;
    (*s).b = (*s).p;
}
unsafe extern "C" fn string_length(mut s: *mut string) -> libc::c_int {
    if (*s).p == (*s).b {
        return 0 as libc::c_int;
    }
    return ((*s).p).offset_from((*s).b) as libc::c_long as libc::c_int;
}
unsafe extern "C" fn string_setlength(mut s: *mut string, mut n: libc::c_int) {
    if n - string_length(s) < 0 as libc::c_int {
        (*s).p = ((*s).b).offset(n as isize);
    }
}
unsafe extern "C" fn string_append(mut p: *mut string, mut s: *const libc::c_char) {
    let mut n: size_t = strlen(s);
    string_need(p, n);
    memcpy((*p).p as *mut libc::c_void, s as *const libc::c_void, n);
    (*p).p = ((*p).p).offset(n as isize);
}
unsafe extern "C" fn string_appendn(
    mut p: *mut string,
    mut s: *const libc::c_char,
    mut n: size_t,
) {
    if n != 0 as libc::c_int as libc::c_ulong {
        string_need(p, n);
        memcpy((*p).p as *mut libc::c_void, s as *const libc::c_void, n);
        (*p).p = ((*p).p).offset(n as isize);
    }
}
unsafe extern "C" fn string_prependn(
    mut p: *mut string,
    mut s: *const libc::c_char,
    mut n: size_t,
) {
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    if n != 0 as libc::c_int as libc::c_ulong {
        string_need(p, n);
        q = ((*p).p).offset(-(1 as libc::c_int as isize));
        while q >= (*p).b {
            *q.offset(n as isize) = *q.offset(0 as libc::c_int as isize);
            q = q.offset(-1);
            q;
        }
        memcpy((*p).b as *mut libc::c_void, s as *const libc::c_void, n);
        (*p).p = ((*p).p).offset(n as isize);
    }
}
unsafe extern "C" fn string_prepend(mut p: *mut string, mut s: *const libc::c_char) {
    if !s.is_null() && *s as libc::c_int != '\0' as i32 {
        string_prependn(p, s, strlen(s));
    }
}
unsafe extern "C" fn dlang_number(
    mut mangled: *const libc::c_char,
    mut ret: *mut libc::c_ulong,
) -> *const libc::c_char {
    if mangled.is_null()
        || _sch_istable[(*mangled as libc::c_int & 0xff as libc::c_int) as usize]
            as libc::c_int & _sch_isdigit as libc::c_int as libc::c_ushort as libc::c_int
            == 0
    {
        return 0 as *const libc::c_char;
    }
    let mut val: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    while _sch_istable[(*mangled as libc::c_int & 0xff as libc::c_int) as usize]
        as libc::c_int & _sch_isdigit as libc::c_int as libc::c_ushort as libc::c_int
        != 0
    {
        let mut digit: libc::c_ulong = (*mangled.offset(0 as libc::c_int as isize)
            as libc::c_int - '0' as i32) as libc::c_ulong;
        if val
            > ((2147483647 as libc::c_int as libc::c_uint)
                .wrapping_mul(2 as libc::c_uint)
                .wrapping_add(1 as libc::c_uint) as libc::c_ulong)
                .wrapping_sub(digit)
                .wrapping_div(10 as libc::c_int as libc::c_ulong)
        {
            return 0 as *const libc::c_char;
        }
        val = val.wrapping_mul(10 as libc::c_int as libc::c_ulong).wrapping_add(digit);
        mangled = mangled.offset(1);
        mangled;
    }
    if *mangled as libc::c_int == '\0' as i32 {
        return 0 as *const libc::c_char;
    }
    *ret = val;
    return mangled;
}
unsafe extern "C" fn dlang_hexdigit(
    mut mangled: *const libc::c_char,
    mut ret: *mut libc::c_char,
) -> *const libc::c_char {
    let mut c: libc::c_char = 0;
    if mangled.is_null()
        || _sch_istable[(*mangled.offset(0 as libc::c_int as isize) as libc::c_int
            & 0xff as libc::c_int) as usize] as libc::c_int
            & _sch_isxdigit as libc::c_int as libc::c_ushort as libc::c_int == 0
        || _sch_istable[(*mangled.offset(1 as libc::c_int as isize) as libc::c_int
            & 0xff as libc::c_int) as usize] as libc::c_int
            & _sch_isxdigit as libc::c_int as libc::c_ushort as libc::c_int == 0
    {
        return 0 as *const libc::c_char;
    }
    c = *mangled.offset(0 as libc::c_int as isize);
    if _sch_istable[(c as libc::c_int & 0xff as libc::c_int) as usize] as libc::c_int
        & _sch_isdigit as libc::c_int as libc::c_ushort as libc::c_int == 0
    {
        *ret = (c as libc::c_int
            - (if _sch_istable[(c as libc::c_int & 0xff as libc::c_int) as usize]
                as libc::c_int
                & _sch_isupper as libc::c_int as libc::c_ushort as libc::c_int != 0
            {
                'A' as i32
            } else {
                'a' as i32
            }) + 10 as libc::c_int) as libc::c_char;
    } else {
        *ret = (c as libc::c_int - '0' as i32) as libc::c_char;
    }
    c = *mangled.offset(1 as libc::c_int as isize);
    if _sch_istable[(c as libc::c_int & 0xff as libc::c_int) as usize] as libc::c_int
        & _sch_isdigit as libc::c_int as libc::c_ushort as libc::c_int == 0
    {
        *ret = ((*ret as libc::c_int) << 4 as libc::c_int
            | c as libc::c_int
                - (if _sch_istable[(c as libc::c_int & 0xff as libc::c_int) as usize]
                    as libc::c_int
                    & _sch_isupper as libc::c_int as libc::c_ushort as libc::c_int != 0
                {
                    'A' as i32
                } else {
                    'a' as i32
                }) + 10 as libc::c_int) as libc::c_char;
    } else {
        *ret = ((*ret as libc::c_int) << 4 as libc::c_int
            | c as libc::c_int - '0' as i32) as libc::c_char;
    }
    mangled = mangled.offset(2 as libc::c_int as isize);
    return mangled;
}
unsafe extern "C" fn dlang_call_convention_p(
    mut mangled: *const libc::c_char,
) -> libc::c_int {
    match *mangled as libc::c_int {
        70 | 85 | 86 | 87 | 82 | 89 => return 1 as libc::c_int,
        _ => return 0 as libc::c_int,
    };
}
unsafe extern "C" fn dlang_decode_backref(
    mut mangled: *const libc::c_char,
    mut ret: *mut libc::c_long,
) -> *const libc::c_char {
    if mangled.is_null()
        || _sch_istable[(*mangled as libc::c_int & 0xff as libc::c_int) as usize]
            as libc::c_int & _sch_isalpha as libc::c_int as libc::c_ushort as libc::c_int
            == 0
    {
        return 0 as *const libc::c_char;
    }
    let mut val: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    while _sch_istable[(*mangled as libc::c_int & 0xff as libc::c_int) as usize]
        as libc::c_int & _sch_isalpha as libc::c_int as libc::c_ushort as libc::c_int
        != 0
    {
        if val
            > (9223372036854775807 as libc::c_long as libc::c_ulong)
                .wrapping_mul(2 as libc::c_ulong)
                .wrapping_add(1 as libc::c_ulong)
                .wrapping_sub(25 as libc::c_int as libc::c_ulong)
                .wrapping_div(26 as libc::c_int as libc::c_ulong)
        {
            break;
        }
        val = val.wrapping_mul(26 as libc::c_int as libc::c_ulong);
        if *mangled.offset(0 as libc::c_int as isize) as libc::c_int >= 'a' as i32
            && *mangled.offset(0 as libc::c_int as isize) as libc::c_int <= 'z' as i32
        {
            val = val
                .wrapping_add(
                    (*mangled.offset(0 as libc::c_int as isize) as libc::c_int
                        - 'a' as i32) as libc::c_ulong,
                );
            if val as libc::c_long <= 0 as libc::c_int as libc::c_long {
                break;
            }
            *ret = val as libc::c_long;
            return mangled.offset(1 as libc::c_int as isize);
        } else {
            val = val
                .wrapping_add(
                    (*mangled.offset(0 as libc::c_int as isize) as libc::c_int
                        - 'A' as i32) as libc::c_ulong,
                );
            mangled = mangled.offset(1);
            mangled;
        }
    }
    return 0 as *const libc::c_char;
}
unsafe extern "C" fn dlang_backref(
    mut mangled: *const libc::c_char,
    mut ret: *mut *const libc::c_char,
    mut info: *mut dlang_info,
) -> *const libc::c_char {
    *ret = 0 as *const libc::c_char;
    if mangled.is_null() || *mangled as libc::c_int != 'Q' as i32 {
        return 0 as *const libc::c_char;
    }
    let mut qpos: *const libc::c_char = mangled;
    let mut refpos: libc::c_long = 0;
    mangled = mangled.offset(1);
    mangled;
    mangled = dlang_decode_backref(mangled, &mut refpos);
    if mangled.is_null() {
        return 0 as *const libc::c_char;
    }
    if refpos > qpos.offset_from((*info).s) as libc::c_long {
        return 0 as *const libc::c_char;
    }
    *ret = qpos.offset(-(refpos as isize));
    return mangled;
}
unsafe extern "C" fn dlang_symbol_backref(
    mut decl: *mut string,
    mut mangled: *const libc::c_char,
    mut info: *mut dlang_info,
) -> *const libc::c_char {
    let mut backref: *const libc::c_char = 0 as *const libc::c_char;
    let mut len: libc::c_ulong = 0;
    mangled = dlang_backref(mangled, &mut backref, info);
    backref = dlang_number(backref, &mut len);
    if backref.is_null() {
        return 0 as *const libc::c_char;
    }
    backref = dlang_lname(decl, backref, len);
    if backref.is_null() {
        return 0 as *const libc::c_char;
    }
    return mangled;
}
unsafe extern "C" fn dlang_type_backref(
    mut decl: *mut string,
    mut mangled: *const libc::c_char,
    mut info: *mut dlang_info,
    mut is_function: libc::c_int,
) -> *const libc::c_char {
    let mut backref: *const libc::c_char = 0 as *const libc::c_char;
    if mangled.offset_from((*info).s) as libc::c_long
        >= (*info).last_backref as libc::c_long
    {
        return 0 as *const libc::c_char;
    }
    let mut save_refpos: libc::c_int = (*info).last_backref;
    (*info).last_backref = mangled.offset_from((*info).s) as libc::c_long as libc::c_int;
    mangled = dlang_backref(mangled, &mut backref, info);
    if is_function != 0 {
        backref = dlang_function_type(decl, backref, info);
    } else {
        backref = dlang_type(decl, backref, info);
    }
    (*info).last_backref = save_refpos;
    if backref.is_null() {
        return 0 as *const libc::c_char;
    }
    return mangled;
}
unsafe extern "C" fn dlang_symbol_name_p(
    mut mangled: *const libc::c_char,
    mut info: *mut dlang_info,
) -> libc::c_int {
    let mut ret: libc::c_long = 0;
    let mut qref: *const libc::c_char = mangled;
    if _sch_istable[(*mangled as libc::c_int & 0xff as libc::c_int) as usize]
        as libc::c_int & _sch_isdigit as libc::c_int as libc::c_ushort as libc::c_int
        != 0
    {
        return 1 as libc::c_int;
    }
    if *mangled.offset(0 as libc::c_int as isize) as libc::c_int == '_' as i32
        && *mangled.offset(1 as libc::c_int as isize) as libc::c_int == '_' as i32
        && (*mangled.offset(2 as libc::c_int as isize) as libc::c_int == 'T' as i32
            || *mangled.offset(2 as libc::c_int as isize) as libc::c_int == 'U' as i32)
    {
        return 1 as libc::c_int;
    }
    if *mangled as libc::c_int != 'Q' as i32 {
        return 0 as libc::c_int;
    }
    mangled = dlang_decode_backref(mangled.offset(1 as libc::c_int as isize), &mut ret);
    if mangled.is_null() || ret > qref.offset_from((*info).s) as libc::c_long {
        return 0 as libc::c_int;
    }
    return _sch_istable[(*qref.offset(-ret as isize) as libc::c_int
        & 0xff as libc::c_int) as usize] as libc::c_int
        & _sch_isdigit as libc::c_int as libc::c_ushort as libc::c_int;
}
unsafe extern "C" fn dlang_call_convention(
    mut decl: *mut string,
    mut mangled: *const libc::c_char,
) -> *const libc::c_char {
    if mangled.is_null() || *mangled as libc::c_int == '\0' as i32 {
        return 0 as *const libc::c_char;
    }
    match *mangled as libc::c_int {
        70 => {
            mangled = mangled.offset(1);
            mangled;
        }
        85 => {
            mangled = mangled.offset(1);
            mangled;
            string_append(decl, b"extern(C) \0" as *const u8 as *const libc::c_char);
        }
        87 => {
            mangled = mangled.offset(1);
            mangled;
            string_append(
                decl,
                b"extern(Windows) \0" as *const u8 as *const libc::c_char,
            );
        }
        86 => {
            mangled = mangled.offset(1);
            mangled;
            string_append(
                decl,
                b"extern(Pascal) \0" as *const u8 as *const libc::c_char,
            );
        }
        82 => {
            mangled = mangled.offset(1);
            mangled;
            string_append(decl, b"extern(C++) \0" as *const u8 as *const libc::c_char);
        }
        89 => {
            mangled = mangled.offset(1);
            mangled;
            string_append(
                decl,
                b"extern(Objective-C) \0" as *const u8 as *const libc::c_char,
            );
        }
        _ => return 0 as *const libc::c_char,
    }
    return mangled;
}
unsafe extern "C" fn dlang_type_modifiers(
    mut decl: *mut string,
    mut mangled: *const libc::c_char,
) -> *const libc::c_char {
    if mangled.is_null() || *mangled as libc::c_int == '\0' as i32 {
        return 0 as *const libc::c_char;
    }
    match *mangled as libc::c_int {
        120 => {
            mangled = mangled.offset(1);
            mangled;
            string_append(decl, b" const\0" as *const u8 as *const libc::c_char);
            return mangled;
        }
        121 => {
            mangled = mangled.offset(1);
            mangled;
            string_append(decl, b" immutable\0" as *const u8 as *const libc::c_char);
            return mangled;
        }
        79 => {
            mangled = mangled.offset(1);
            mangled;
            string_append(decl, b" shared\0" as *const u8 as *const libc::c_char);
            return dlang_type_modifiers(decl, mangled);
        }
        78 => {
            mangled = mangled.offset(1);
            mangled;
            if *mangled as libc::c_int == 'g' as i32 {
                mangled = mangled.offset(1);
                mangled;
                string_append(decl, b" inout\0" as *const u8 as *const libc::c_char);
                return dlang_type_modifiers(decl, mangled);
            } else {
                return 0 as *const libc::c_char
            }
        }
        _ => return mangled,
    };
}
unsafe extern "C" fn dlang_attributes(
    mut decl: *mut string,
    mut mangled: *const libc::c_char,
) -> *const libc::c_char {
    if mangled.is_null() || *mangled as libc::c_int == '\0' as i32 {
        return 0 as *const libc::c_char;
    }
    while *mangled as libc::c_int == 'N' as i32 {
        mangled = mangled.offset(1);
        mangled;
        match *mangled as libc::c_int {
            97 => {
                mangled = mangled.offset(1);
                mangled;
                string_append(decl, b"pure \0" as *const u8 as *const libc::c_char);
            }
            98 => {
                mangled = mangled.offset(1);
                mangled;
                string_append(decl, b"nothrow \0" as *const u8 as *const libc::c_char);
            }
            99 => {
                mangled = mangled.offset(1);
                mangled;
                string_append(decl, b"ref \0" as *const u8 as *const libc::c_char);
            }
            100 => {
                mangled = mangled.offset(1);
                mangled;
                string_append(decl, b"@property \0" as *const u8 as *const libc::c_char);
            }
            101 => {
                mangled = mangled.offset(1);
                mangled;
                string_append(decl, b"@trusted \0" as *const u8 as *const libc::c_char);
            }
            102 => {
                mangled = mangled.offset(1);
                mangled;
                string_append(decl, b"@safe \0" as *const u8 as *const libc::c_char);
            }
            103 | 104 | 107 => {
                mangled = mangled.offset(-1);
                mangled;
                break;
            }
            105 => {
                mangled = mangled.offset(1);
                mangled;
                string_append(decl, b"@nogc \0" as *const u8 as *const libc::c_char);
            }
            106 => {
                mangled = mangled.offset(1);
                mangled;
                string_append(decl, b"return \0" as *const u8 as *const libc::c_char);
            }
            108 => {
                mangled = mangled.offset(1);
                mangled;
                string_append(decl, b"scope \0" as *const u8 as *const libc::c_char);
            }
            109 => {
                mangled = mangled.offset(1);
                mangled;
                string_append(decl, b"@live \0" as *const u8 as *const libc::c_char);
            }
            _ => return 0 as *const libc::c_char,
        }
    }
    return mangled;
}
unsafe extern "C" fn dlang_function_type_noreturn(
    mut args: *mut string,
    mut call: *mut string,
    mut attr: *mut string,
    mut mangled: *const libc::c_char,
    mut info: *mut dlang_info,
) -> *const libc::c_char {
    let mut dump: string = string {
        b: 0 as *mut libc::c_char,
        p: 0 as *mut libc::c_char,
        e: 0 as *mut libc::c_char,
    };
    string_init(&mut dump);
    mangled = dlang_call_convention(
        if !call.is_null() { call } else { &mut dump },
        mangled,
    );
    mangled = dlang_attributes(if !attr.is_null() { attr } else { &mut dump }, mangled);
    if !args.is_null() {
        string_append(args, b"(\0" as *const u8 as *const libc::c_char);
    }
    mangled = dlang_function_args(
        if !args.is_null() { args } else { &mut dump },
        mangled,
        info,
    );
    if !args.is_null() {
        string_append(args, b")\0" as *const u8 as *const libc::c_char);
    }
    string_delete(&mut dump);
    return mangled;
}
unsafe extern "C" fn dlang_function_type(
    mut decl: *mut string,
    mut mangled: *const libc::c_char,
    mut info: *mut dlang_info,
) -> *const libc::c_char {
    let mut attr: string = string {
        b: 0 as *mut libc::c_char,
        p: 0 as *mut libc::c_char,
        e: 0 as *mut libc::c_char,
    };
    let mut args: string = string {
        b: 0 as *mut libc::c_char,
        p: 0 as *mut libc::c_char,
        e: 0 as *mut libc::c_char,
    };
    let mut type_0: string = string {
        b: 0 as *mut libc::c_char,
        p: 0 as *mut libc::c_char,
        e: 0 as *mut libc::c_char,
    };
    if mangled.is_null() || *mangled as libc::c_int == '\0' as i32 {
        return 0 as *const libc::c_char;
    }
    string_init(&mut attr);
    string_init(&mut args);
    string_init(&mut type_0);
    mangled = dlang_function_type_noreturn(&mut args, decl, &mut attr, mangled, info);
    mangled = dlang_type(&mut type_0, mangled, info);
    string_appendn(decl, type_0.b, string_length(&mut type_0) as size_t);
    string_appendn(decl, args.b, string_length(&mut args) as size_t);
    string_append(decl, b" \0" as *const u8 as *const libc::c_char);
    string_appendn(decl, attr.b, string_length(&mut attr) as size_t);
    string_delete(&mut attr);
    string_delete(&mut args);
    string_delete(&mut type_0);
    return mangled;
}
unsafe extern "C" fn dlang_function_args(
    mut decl: *mut string,
    mut mangled: *const libc::c_char,
    mut info: *mut dlang_info,
) -> *const libc::c_char {
    let mut n: size_t = 0 as libc::c_int as size_t;
    while !mangled.is_null() && *mangled as libc::c_int != '\0' as i32 {
        match *mangled as libc::c_int {
            88 => {
                mangled = mangled.offset(1);
                mangled;
                string_append(decl, b"...\0" as *const u8 as *const libc::c_char);
                return mangled;
            }
            89 => {
                mangled = mangled.offset(1);
                mangled;
                if n != 0 as libc::c_int as libc::c_ulong {
                    string_append(decl, b", \0" as *const u8 as *const libc::c_char);
                }
                string_append(decl, b"...\0" as *const u8 as *const libc::c_char);
                return mangled;
            }
            90 => {
                mangled = mangled.offset(1);
                mangled;
                return mangled;
            }
            _ => {}
        }
        let fresh0 = n;
        n = n.wrapping_add(1);
        if fresh0 != 0 {
            string_append(decl, b", \0" as *const u8 as *const libc::c_char);
        }
        if *mangled as libc::c_int == 'M' as i32 {
            mangled = mangled.offset(1);
            mangled;
            string_append(decl, b"scope \0" as *const u8 as *const libc::c_char);
        }
        if *mangled.offset(0 as libc::c_int as isize) as libc::c_int == 'N' as i32
            && *mangled.offset(1 as libc::c_int as isize) as libc::c_int == 'k' as i32
        {
            mangled = mangled.offset(2 as libc::c_int as isize);
            string_append(decl, b"return \0" as *const u8 as *const libc::c_char);
        }
        match *mangled as libc::c_int {
            73 => {
                mangled = mangled.offset(1);
                mangled;
                string_append(decl, b"in \0" as *const u8 as *const libc::c_char);
                if *mangled as libc::c_int == 'K' as i32 {
                    mangled = mangled.offset(1);
                    mangled;
                    string_append(decl, b"ref \0" as *const u8 as *const libc::c_char);
                }
            }
            74 => {
                mangled = mangled.offset(1);
                mangled;
                string_append(decl, b"out \0" as *const u8 as *const libc::c_char);
            }
            75 => {
                mangled = mangled.offset(1);
                mangled;
                string_append(decl, b"ref \0" as *const u8 as *const libc::c_char);
            }
            76 => {
                mangled = mangled.offset(1);
                mangled;
                string_append(decl, b"lazy \0" as *const u8 as *const libc::c_char);
            }
            _ => {}
        }
        mangled = dlang_type(decl, mangled, info);
    }
    return mangled;
}
unsafe extern "C" fn dlang_type(
    mut decl: *mut string,
    mut mangled: *const libc::c_char,
    mut info: *mut dlang_info,
) -> *const libc::c_char {
    if mangled.is_null() || *mangled as libc::c_int == '\0' as i32 {
        return 0 as *const libc::c_char;
    }
    let mut current_block_164: u64;
    match *mangled as libc::c_int {
        79 => {
            mangled = mangled.offset(1);
            mangled;
            string_append(decl, b"shared(\0" as *const u8 as *const libc::c_char);
            mangled = dlang_type(decl, mangled, info);
            string_append(decl, b")\0" as *const u8 as *const libc::c_char);
            return mangled;
        }
        120 => {
            mangled = mangled.offset(1);
            mangled;
            string_append(decl, b"const(\0" as *const u8 as *const libc::c_char);
            mangled = dlang_type(decl, mangled, info);
            string_append(decl, b")\0" as *const u8 as *const libc::c_char);
            return mangled;
        }
        121 => {
            mangled = mangled.offset(1);
            mangled;
            string_append(decl, b"immutable(\0" as *const u8 as *const libc::c_char);
            mangled = dlang_type(decl, mangled, info);
            string_append(decl, b")\0" as *const u8 as *const libc::c_char);
            return mangled;
        }
        78 => {
            mangled = mangled.offset(1);
            mangled;
            if *mangled as libc::c_int == 'g' as i32 {
                mangled = mangled.offset(1);
                mangled;
                string_append(decl, b"inout(\0" as *const u8 as *const libc::c_char);
                mangled = dlang_type(decl, mangled, info);
                string_append(decl, b")\0" as *const u8 as *const libc::c_char);
                return mangled;
            } else if *mangled as libc::c_int == 'h' as i32 {
                mangled = mangled.offset(1);
                mangled;
                string_append(decl, b"__vector(\0" as *const u8 as *const libc::c_char);
                mangled = dlang_type(decl, mangled, info);
                string_append(decl, b")\0" as *const u8 as *const libc::c_char);
                return mangled;
            } else {
                return 0 as *const libc::c_char
            }
        }
        65 => {
            mangled = mangled.offset(1);
            mangled;
            mangled = dlang_type(decl, mangled, info);
            string_append(decl, b"[]\0" as *const u8 as *const libc::c_char);
            return mangled;
        }
        71 => {
            let mut numptr: *const libc::c_char = 0 as *const libc::c_char;
            let mut num: size_t = 0 as libc::c_int as size_t;
            mangled = mangled.offset(1);
            mangled;
            numptr = mangled;
            while _sch_istable[(*mangled as libc::c_int & 0xff as libc::c_int) as usize]
                as libc::c_int
                & _sch_isdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
            {
                num = num.wrapping_add(1);
                num;
                mangled = mangled.offset(1);
                mangled;
            }
            mangled = dlang_type(decl, mangled, info);
            string_append(decl, b"[\0" as *const u8 as *const libc::c_char);
            string_appendn(decl, numptr, num);
            string_append(decl, b"]\0" as *const u8 as *const libc::c_char);
            return mangled;
        }
        72 => {
            let mut type_0: string = string {
                b: 0 as *mut libc::c_char,
                p: 0 as *mut libc::c_char,
                e: 0 as *mut libc::c_char,
            };
            let mut sztype: size_t = 0;
            mangled = mangled.offset(1);
            mangled;
            string_init(&mut type_0);
            mangled = dlang_type(&mut type_0, mangled, info);
            sztype = string_length(&mut type_0) as size_t;
            mangled = dlang_type(decl, mangled, info);
            string_append(decl, b"[\0" as *const u8 as *const libc::c_char);
            string_appendn(decl, type_0.b, sztype);
            string_append(decl, b"]\0" as *const u8 as *const libc::c_char);
            string_delete(&mut type_0);
            return mangled;
        }
        80 => {
            mangled = mangled.offset(1);
            mangled;
            if dlang_call_convention_p(mangled) == 0 {
                mangled = dlang_type(decl, mangled, info);
                string_append(decl, b"*\0" as *const u8 as *const libc::c_char);
                return mangled;
            }
            current_block_164 = 7282908690880421722;
        }
        70 => {
            current_block_164 = 7282908690880421722;
        }
        85 => {
            current_block_164 = 9935778755722992700;
        }
        87 => {
            current_block_164 = 17391114015930030077;
        }
        86 => {
            current_block_164 = 7540500656593025517;
        }
        82 | 89 => {
            current_block_164 = 7339781465918505082;
        }
        67 => {
            current_block_164 = 6638091054638106328;
        }
        83 => {
            current_block_164 = 6638091054638106328;
        }
        69 | 84 => {
            current_block_164 = 3003170618667347221;
        }
        68 => {
            let mut mods: string = string {
                b: 0 as *mut libc::c_char,
                p: 0 as *mut libc::c_char,
                e: 0 as *mut libc::c_char,
            };
            let mut szmods: size_t = 0;
            mangled = mangled.offset(1);
            mangled;
            string_init(&mut mods);
            mangled = dlang_type_modifiers(&mut mods, mangled);
            szmods = string_length(&mut mods) as size_t;
            if *mangled as libc::c_int == 'Q' as i32 {
                mangled = dlang_type_backref(decl, mangled, info, 1 as libc::c_int);
            } else {
                mangled = dlang_function_type(decl, mangled, info);
            }
            string_append(decl, b"delegate\0" as *const u8 as *const libc::c_char);
            string_appendn(decl, mods.b, szmods);
            string_delete(&mut mods);
            return mangled;
        }
        66 => {
            mangled = mangled.offset(1);
            mangled;
            return dlang_parse_tuple(decl, mangled, info);
        }
        110 => {
            mangled = mangled.offset(1);
            mangled;
            string_append(decl, b"none\0" as *const u8 as *const libc::c_char);
            return mangled;
        }
        118 => {
            mangled = mangled.offset(1);
            mangled;
            string_append(decl, b"void\0" as *const u8 as *const libc::c_char);
            return mangled;
        }
        103 => {
            mangled = mangled.offset(1);
            mangled;
            string_append(decl, b"byte\0" as *const u8 as *const libc::c_char);
            return mangled;
        }
        104 => {
            mangled = mangled.offset(1);
            mangled;
            string_append(decl, b"ubyte\0" as *const u8 as *const libc::c_char);
            return mangled;
        }
        115 => {
            mangled = mangled.offset(1);
            mangled;
            string_append(decl, b"short\0" as *const u8 as *const libc::c_char);
            return mangled;
        }
        116 => {
            mangled = mangled.offset(1);
            mangled;
            string_append(decl, b"ushort\0" as *const u8 as *const libc::c_char);
            return mangled;
        }
        105 => {
            mangled = mangled.offset(1);
            mangled;
            string_append(decl, b"int\0" as *const u8 as *const libc::c_char);
            return mangled;
        }
        107 => {
            mangled = mangled.offset(1);
            mangled;
            string_append(decl, b"uint\0" as *const u8 as *const libc::c_char);
            return mangled;
        }
        108 => {
            mangled = mangled.offset(1);
            mangled;
            string_append(decl, b"long\0" as *const u8 as *const libc::c_char);
            return mangled;
        }
        109 => {
            mangled = mangled.offset(1);
            mangled;
            string_append(decl, b"ulong\0" as *const u8 as *const libc::c_char);
            return mangled;
        }
        102 => {
            mangled = mangled.offset(1);
            mangled;
            string_append(decl, b"float\0" as *const u8 as *const libc::c_char);
            return mangled;
        }
        100 => {
            mangled = mangled.offset(1);
            mangled;
            string_append(decl, b"double\0" as *const u8 as *const libc::c_char);
            return mangled;
        }
        101 => {
            mangled = mangled.offset(1);
            mangled;
            string_append(decl, b"real\0" as *const u8 as *const libc::c_char);
            return mangled;
        }
        111 => {
            mangled = mangled.offset(1);
            mangled;
            string_append(decl, b"ifloat\0" as *const u8 as *const libc::c_char);
            return mangled;
        }
        112 => {
            mangled = mangled.offset(1);
            mangled;
            string_append(decl, b"idouble\0" as *const u8 as *const libc::c_char);
            return mangled;
        }
        106 => {
            mangled = mangled.offset(1);
            mangled;
            string_append(decl, b"ireal\0" as *const u8 as *const libc::c_char);
            return mangled;
        }
        113 => {
            mangled = mangled.offset(1);
            mangled;
            string_append(decl, b"cfloat\0" as *const u8 as *const libc::c_char);
            return mangled;
        }
        114 => {
            mangled = mangled.offset(1);
            mangled;
            string_append(decl, b"cdouble\0" as *const u8 as *const libc::c_char);
            return mangled;
        }
        99 => {
            mangled = mangled.offset(1);
            mangled;
            string_append(decl, b"creal\0" as *const u8 as *const libc::c_char);
            return mangled;
        }
        98 => {
            mangled = mangled.offset(1);
            mangled;
            string_append(decl, b"bool\0" as *const u8 as *const libc::c_char);
            return mangled;
        }
        97 => {
            mangled = mangled.offset(1);
            mangled;
            string_append(decl, b"char\0" as *const u8 as *const libc::c_char);
            return mangled;
        }
        117 => {
            mangled = mangled.offset(1);
            mangled;
            string_append(decl, b"wchar\0" as *const u8 as *const libc::c_char);
            return mangled;
        }
        119 => {
            mangled = mangled.offset(1);
            mangled;
            string_append(decl, b"dchar\0" as *const u8 as *const libc::c_char);
            return mangled;
        }
        122 => {
            mangled = mangled.offset(1);
            mangled;
            match *mangled as libc::c_int {
                105 => {
                    mangled = mangled.offset(1);
                    mangled;
                    string_append(decl, b"cent\0" as *const u8 as *const libc::c_char);
                    return mangled;
                }
                107 => {
                    mangled = mangled.offset(1);
                    mangled;
                    string_append(decl, b"ucent\0" as *const u8 as *const libc::c_char);
                    return mangled;
                }
                _ => {}
            }
            return 0 as *const libc::c_char;
        }
        81 => return dlang_type_backref(decl, mangled, info, 0 as libc::c_int),
        _ => return 0 as *const libc::c_char,
    }
    match current_block_164 {
        7282908690880421722 => {
            current_block_164 = 9935778755722992700;
        }
        6638091054638106328 => {
            current_block_164 = 3003170618667347221;
        }
        _ => {}
    }
    match current_block_164 {
        3003170618667347221 => {
            mangled = mangled.offset(1);
            mangled;
            return dlang_parse_qualified(decl, mangled, info, 0 as libc::c_int);
        }
        9935778755722992700 => {
            current_block_164 = 17391114015930030077;
        }
        _ => {}
    }
    match current_block_164 {
        17391114015930030077 => {
            current_block_164 = 7540500656593025517;
        }
        _ => {}
    }
    match current_block_164 {
        7540500656593025517 => {}
        _ => {}
    }
    mangled = dlang_function_type(decl, mangled, info);
    string_append(decl, b"function\0" as *const u8 as *const libc::c_char);
    return mangled;
}
unsafe extern "C" fn dlang_identifier(
    mut decl: *mut string,
    mut mangled: *const libc::c_char,
    mut info: *mut dlang_info,
) -> *const libc::c_char {
    let mut len: libc::c_ulong = 0;
    if mangled.is_null() || *mangled as libc::c_int == '\0' as i32 {
        return 0 as *const libc::c_char;
    }
    if *mangled as libc::c_int == 'Q' as i32 {
        return dlang_symbol_backref(decl, mangled, info);
    }
    if *mangled.offset(0 as libc::c_int as isize) as libc::c_int == '_' as i32
        && *mangled.offset(1 as libc::c_int as isize) as libc::c_int == '_' as i32
        && (*mangled.offset(2 as libc::c_int as isize) as libc::c_int == 'T' as i32
            || *mangled.offset(2 as libc::c_int as isize) as libc::c_int == 'U' as i32)
    {
        return dlang_parse_template(
            decl,
            mangled,
            info,
            (1 as libc::c_ulong).wrapping_neg(),
        );
    }
    let mut endptr: *const libc::c_char = dlang_number(mangled, &mut len);
    if endptr.is_null() || len == 0 as libc::c_int as libc::c_ulong {
        return 0 as *const libc::c_char;
    }
    if strlen(endptr) < len {
        return 0 as *const libc::c_char;
    }
    mangled = endptr;
    if len >= 5 as libc::c_int as libc::c_ulong
        && *mangled.offset(0 as libc::c_int as isize) as libc::c_int == '_' as i32
        && *mangled.offset(1 as libc::c_int as isize) as libc::c_int == '_' as i32
        && (*mangled.offset(2 as libc::c_int as isize) as libc::c_int == 'T' as i32
            || *mangled.offset(2 as libc::c_int as isize) as libc::c_int == 'U' as i32)
    {
        return dlang_parse_template(decl, mangled, info, len);
    }
    return dlang_lname(decl, mangled, len);
}
unsafe extern "C" fn dlang_lname(
    mut decl: *mut string,
    mut mangled: *const libc::c_char,
    mut len: libc::c_ulong,
) -> *const libc::c_char {
    match len {
        6 => {
            if strncmp(mangled, b"__ctor\0" as *const u8 as *const libc::c_char, len)
                == 0 as libc::c_int
            {
                string_append(decl, b"this\0" as *const u8 as *const libc::c_char);
                mangled = mangled.offset(len as isize);
                return mangled;
            } else if strncmp(
                mangled,
                b"__dtor\0" as *const u8 as *const libc::c_char,
                len,
            ) == 0 as libc::c_int
            {
                string_append(decl, b"~this\0" as *const u8 as *const libc::c_char);
                mangled = mangled.offset(len as isize);
                return mangled;
            } else if strncmp(
                mangled,
                b"__initZ\0" as *const u8 as *const libc::c_char,
                len.wrapping_add(1 as libc::c_int as libc::c_ulong),
            ) == 0 as libc::c_int
            {
                string_prepend(
                    decl,
                    b"initializer for \0" as *const u8 as *const libc::c_char,
                );
                string_setlength(decl, string_length(decl) - 1 as libc::c_int);
                mangled = mangled.offset(len as isize);
                return mangled;
            } else if strncmp(
                mangled,
                b"__vtblZ\0" as *const u8 as *const libc::c_char,
                len.wrapping_add(1 as libc::c_int as libc::c_ulong),
            ) == 0 as libc::c_int
            {
                string_prepend(
                    decl,
                    b"vtable for \0" as *const u8 as *const libc::c_char,
                );
                string_setlength(decl, string_length(decl) - 1 as libc::c_int);
                mangled = mangled.offset(len as isize);
                return mangled;
            }
        }
        7 => {
            if strncmp(
                mangled,
                b"__ClassZ\0" as *const u8 as *const libc::c_char,
                len.wrapping_add(1 as libc::c_int as libc::c_ulong),
            ) == 0 as libc::c_int
            {
                string_prepend(
                    decl,
                    b"ClassInfo for \0" as *const u8 as *const libc::c_char,
                );
                string_setlength(decl, string_length(decl) - 1 as libc::c_int);
                mangled = mangled.offset(len as isize);
                return mangled;
            }
        }
        10 => {
            if strncmp(
                mangled,
                b"__postblitMFZ\0" as *const u8 as *const libc::c_char,
                len.wrapping_add(3 as libc::c_int as libc::c_ulong),
            ) == 0 as libc::c_int
            {
                string_append(decl, b"this(this)\0" as *const u8 as *const libc::c_char);
                mangled = mangled
                    .offset(
                        len.wrapping_add(3 as libc::c_int as libc::c_ulong) as isize,
                    );
                return mangled;
            }
        }
        11 => {
            if strncmp(
                mangled,
                b"__InterfaceZ\0" as *const u8 as *const libc::c_char,
                len.wrapping_add(1 as libc::c_int as libc::c_ulong),
            ) == 0 as libc::c_int
            {
                string_prepend(
                    decl,
                    b"Interface for \0" as *const u8 as *const libc::c_char,
                );
                string_setlength(decl, string_length(decl) - 1 as libc::c_int);
                mangled = mangled.offset(len as isize);
                return mangled;
            }
        }
        12 => {
            if strncmp(
                mangled,
                b"__ModuleInfoZ\0" as *const u8 as *const libc::c_char,
                len.wrapping_add(1 as libc::c_int as libc::c_ulong),
            ) == 0 as libc::c_int
            {
                string_prepend(
                    decl,
                    b"ModuleInfo for \0" as *const u8 as *const libc::c_char,
                );
                string_setlength(decl, string_length(decl) - 1 as libc::c_int);
                mangled = mangled.offset(len as isize);
                return mangled;
            }
        }
        _ => {}
    }
    string_appendn(decl, mangled, len);
    mangled = mangled.offset(len as isize);
    return mangled;
}
unsafe extern "C" fn dlang_parse_integer(
    mut decl: *mut string,
    mut mangled: *const libc::c_char,
    mut type_0: libc::c_char,
) -> *const libc::c_char {
    if type_0 as libc::c_int == 'a' as i32 || type_0 as libc::c_int == 'u' as i32
        || type_0 as libc::c_int == 'w' as i32
    {
        let mut value: [libc::c_char; 20] = [0; 20];
        let mut pos: libc::c_int = ::core::mem::size_of::<[libc::c_char; 20]>()
            as libc::c_ulong as libc::c_int;
        let mut width: libc::c_int = 0 as libc::c_int;
        let mut val: libc::c_ulong = 0;
        mangled = dlang_number(mangled, &mut val);
        if mangled.is_null() {
            return 0 as *const libc::c_char;
        }
        string_append(decl, b"'\0" as *const u8 as *const libc::c_char);
        if type_0 as libc::c_int == 'a' as i32
            && val >= 0x20 as libc::c_int as libc::c_ulong
            && val < 0x7f as libc::c_int as libc::c_ulong
        {
            let mut c: libc::c_char = val as libc::c_char;
            string_appendn(decl, &mut c, 1 as libc::c_int as size_t);
        } else {
            match type_0 as libc::c_int {
                97 => {
                    string_append(decl, b"\\x\0" as *const u8 as *const libc::c_char);
                    width = 2 as libc::c_int;
                }
                117 => {
                    string_append(decl, b"\\u\0" as *const u8 as *const libc::c_char);
                    width = 4 as libc::c_int;
                }
                119 => {
                    string_append(decl, b"\\U\0" as *const u8 as *const libc::c_char);
                    width = 8 as libc::c_int;
                }
                _ => {}
            }
            while val > 0 as libc::c_int as libc::c_ulong {
                let mut digit: libc::c_int = val
                    .wrapping_rem(16 as libc::c_int as libc::c_ulong) as libc::c_int;
                if digit < 10 as libc::c_int {
                    pos -= 1;
                    value[pos as usize] = (digit + '0' as i32) as libc::c_char;
                } else {
                    pos -= 1;
                    value[pos
                        as usize] = (digit - 10 as libc::c_int + 'a' as i32)
                        as libc::c_char;
                }
                val = val.wrapping_div(16 as libc::c_int as libc::c_ulong);
                width -= 1;
                width;
            }
            while width > 0 as libc::c_int {
                pos -= 1;
                value[pos as usize] = '0' as i32 as libc::c_char;
                width -= 1;
                width;
            }
            string_appendn(
                decl,
                &mut *value.as_mut_ptr().offset(pos as isize),
                (::core::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong)
                    .wrapping_sub(pos as libc::c_ulong),
            );
        }
        string_append(decl, b"'\0" as *const u8 as *const libc::c_char);
    } else if type_0 as libc::c_int == 'b' as i32 {
        let mut val_0: libc::c_ulong = 0;
        mangled = dlang_number(mangled, &mut val_0);
        if mangled.is_null() {
            return 0 as *const libc::c_char;
        }
        string_append(
            decl,
            if val_0 != 0 {
                b"true\0" as *const u8 as *const libc::c_char
            } else {
                b"false\0" as *const u8 as *const libc::c_char
            },
        );
    } else {
        let mut numptr: *const libc::c_char = mangled;
        let mut num: size_t = 0 as libc::c_int as size_t;
        if _sch_istable[(*mangled as libc::c_int & 0xff as libc::c_int) as usize]
            as libc::c_int & _sch_isdigit as libc::c_int as libc::c_ushort as libc::c_int
            == 0
        {
            return 0 as *const libc::c_char;
        }
        while _sch_istable[(*mangled as libc::c_int & 0xff as libc::c_int) as usize]
            as libc::c_int & _sch_isdigit as libc::c_int as libc::c_ushort as libc::c_int
            != 0
        {
            num = num.wrapping_add(1);
            num;
            mangled = mangled.offset(1);
            mangled;
        }
        string_appendn(decl, numptr, num);
        let mut current_block_41: u64;
        match type_0 as libc::c_int {
            116 => {
                current_block_41 = 15804937335748271953;
            }
            104 | 107 => {
                current_block_41 = 15804937335748271953;
            }
            108 => {
                string_append(decl, b"L\0" as *const u8 as *const libc::c_char);
                current_block_41 = 5330834795799507926;
            }
            109 => {
                string_append(decl, b"uL\0" as *const u8 as *const libc::c_char);
                current_block_41 = 5330834795799507926;
            }
            _ => {
                current_block_41 = 5330834795799507926;
            }
        }
        match current_block_41 {
            15804937335748271953 => {
                string_append(decl, b"u\0" as *const u8 as *const libc::c_char);
            }
            _ => {}
        }
    }
    return mangled;
}
unsafe extern "C" fn dlang_parse_real(
    mut decl: *mut string,
    mut mangled: *const libc::c_char,
) -> *const libc::c_char {
    if strncmp(
        mangled,
        b"NAN\0" as *const u8 as *const libc::c_char,
        3 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int
    {
        string_append(decl, b"NaN\0" as *const u8 as *const libc::c_char);
        mangled = mangled.offset(3 as libc::c_int as isize);
        return mangled;
    } else if strncmp(
        mangled,
        b"INF\0" as *const u8 as *const libc::c_char,
        3 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int
    {
        string_append(decl, b"Inf\0" as *const u8 as *const libc::c_char);
        mangled = mangled.offset(3 as libc::c_int as isize);
        return mangled;
    } else if strncmp(
        mangled,
        b"NINF\0" as *const u8 as *const libc::c_char,
        4 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int
    {
        string_append(decl, b"-Inf\0" as *const u8 as *const libc::c_char);
        mangled = mangled.offset(4 as libc::c_int as isize);
        return mangled;
    }
    if *mangled as libc::c_int == 'N' as i32 {
        string_append(decl, b"-\0" as *const u8 as *const libc::c_char);
        mangled = mangled.offset(1);
        mangled;
    }
    if _sch_istable[(*mangled as libc::c_int & 0xff as libc::c_int) as usize]
        as libc::c_int & _sch_isxdigit as libc::c_int as libc::c_ushort as libc::c_int
        == 0
    {
        return 0 as *const libc::c_char;
    }
    string_append(decl, b"0x\0" as *const u8 as *const libc::c_char);
    string_appendn(decl, mangled, 1 as libc::c_int as size_t);
    string_append(decl, b".\0" as *const u8 as *const libc::c_char);
    mangled = mangled.offset(1);
    mangled;
    while _sch_istable[(*mangled as libc::c_int & 0xff as libc::c_int) as usize]
        as libc::c_int & _sch_isxdigit as libc::c_int as libc::c_ushort as libc::c_int
        != 0
    {
        string_appendn(decl, mangled, 1 as libc::c_int as size_t);
        mangled = mangled.offset(1);
        mangled;
    }
    if *mangled as libc::c_int != 'P' as i32 {
        return 0 as *const libc::c_char;
    }
    string_append(decl, b"p\0" as *const u8 as *const libc::c_char);
    mangled = mangled.offset(1);
    mangled;
    if *mangled as libc::c_int == 'N' as i32 {
        string_append(decl, b"-\0" as *const u8 as *const libc::c_char);
        mangled = mangled.offset(1);
        mangled;
    }
    while _sch_istable[(*mangled as libc::c_int & 0xff as libc::c_int) as usize]
        as libc::c_int & _sch_isdigit as libc::c_int as libc::c_ushort as libc::c_int
        != 0
    {
        string_appendn(decl, mangled, 1 as libc::c_int as size_t);
        mangled = mangled.offset(1);
        mangled;
    }
    return mangled;
}
unsafe extern "C" fn dlang_parse_string(
    mut decl: *mut string,
    mut mangled: *const libc::c_char,
) -> *const libc::c_char {
    let mut type_0: libc::c_char = *mangled;
    let mut len: libc::c_ulong = 0;
    mangled = mangled.offset(1);
    mangled;
    mangled = dlang_number(mangled, &mut len);
    if mangled.is_null() || *mangled as libc::c_int != '_' as i32 {
        return 0 as *const libc::c_char;
    }
    mangled = mangled.offset(1);
    mangled;
    string_append(decl, b"\"\0" as *const u8 as *const libc::c_char);
    loop {
        let fresh1 = len;
        len = len.wrapping_sub(1);
        if !(fresh1 != 0) {
            break;
        }
        let mut val: libc::c_char = 0;
        let mut endptr: *const libc::c_char = dlang_hexdigit(mangled, &mut val);
        if endptr.is_null() {
            return 0 as *const libc::c_char;
        }
        match val as libc::c_int {
            32 => {
                string_append(decl, b" \0" as *const u8 as *const libc::c_char);
            }
            9 => {
                string_append(decl, b"\\t\0" as *const u8 as *const libc::c_char);
            }
            10 => {
                string_append(decl, b"\\n\0" as *const u8 as *const libc::c_char);
            }
            13 => {
                string_append(decl, b"\\r\0" as *const u8 as *const libc::c_char);
            }
            12 => {
                string_append(decl, b"\\f\0" as *const u8 as *const libc::c_char);
            }
            11 => {
                string_append(decl, b"\\v\0" as *const u8 as *const libc::c_char);
            }
            _ => {
                if _sch_istable[(val as libc::c_int & 0xff as libc::c_int) as usize]
                    as libc::c_int
                    & _sch_isprint as libc::c_int as libc::c_ushort as libc::c_int != 0
                {
                    string_appendn(decl, &mut val, 1 as libc::c_int as size_t);
                } else {
                    string_append(decl, b"\\x\0" as *const u8 as *const libc::c_char);
                    string_appendn(decl, mangled, 2 as libc::c_int as size_t);
                }
            }
        }
        mangled = endptr;
    }
    string_append(decl, b"\"\0" as *const u8 as *const libc::c_char);
    if type_0 as libc::c_int != 'a' as i32 {
        string_appendn(decl, &mut type_0, 1 as libc::c_int as size_t);
    }
    return mangled;
}
unsafe extern "C" fn dlang_parse_arrayliteral(
    mut decl: *mut string,
    mut mangled: *const libc::c_char,
) -> *const libc::c_char {
    let mut elements: libc::c_ulong = 0;
    mangled = dlang_number(mangled, &mut elements);
    if mangled.is_null() {
        return 0 as *const libc::c_char;
    }
    string_append(decl, b"[\0" as *const u8 as *const libc::c_char);
    loop {
        let fresh2 = elements;
        elements = elements.wrapping_sub(1);
        if !(fresh2 != 0) {
            break;
        }
        mangled = dlang_value(
            decl,
            mangled,
            0 as *const libc::c_char,
            '\0' as i32 as libc::c_char,
        );
        if mangled.is_null() {
            return 0 as *const libc::c_char;
        }
        if elements != 0 as libc::c_int as libc::c_ulong {
            string_append(decl, b", \0" as *const u8 as *const libc::c_char);
        }
    }
    string_append(decl, b"]\0" as *const u8 as *const libc::c_char);
    return mangled;
}
unsafe extern "C" fn dlang_parse_assocarray(
    mut decl: *mut string,
    mut mangled: *const libc::c_char,
) -> *const libc::c_char {
    let mut elements: libc::c_ulong = 0;
    mangled = dlang_number(mangled, &mut elements);
    if mangled.is_null() {
        return 0 as *const libc::c_char;
    }
    string_append(decl, b"[\0" as *const u8 as *const libc::c_char);
    loop {
        let fresh3 = elements;
        elements = elements.wrapping_sub(1);
        if !(fresh3 != 0) {
            break;
        }
        mangled = dlang_value(
            decl,
            mangled,
            0 as *const libc::c_char,
            '\0' as i32 as libc::c_char,
        );
        if mangled.is_null() {
            return 0 as *const libc::c_char;
        }
        string_append(decl, b":\0" as *const u8 as *const libc::c_char);
        mangled = dlang_value(
            decl,
            mangled,
            0 as *const libc::c_char,
            '\0' as i32 as libc::c_char,
        );
        if mangled.is_null() {
            return 0 as *const libc::c_char;
        }
        if elements != 0 as libc::c_int as libc::c_ulong {
            string_append(decl, b", \0" as *const u8 as *const libc::c_char);
        }
    }
    string_append(decl, b"]\0" as *const u8 as *const libc::c_char);
    return mangled;
}
unsafe extern "C" fn dlang_parse_structlit(
    mut decl: *mut string,
    mut mangled: *const libc::c_char,
    mut name: *const libc::c_char,
) -> *const libc::c_char {
    let mut args: libc::c_ulong = 0;
    mangled = dlang_number(mangled, &mut args);
    if mangled.is_null() {
        return 0 as *const libc::c_char;
    }
    if !name.is_null() {
        string_append(decl, name);
    }
    string_append(decl, b"(\0" as *const u8 as *const libc::c_char);
    loop {
        let fresh4 = args;
        args = args.wrapping_sub(1);
        if !(fresh4 != 0) {
            break;
        }
        mangled = dlang_value(
            decl,
            mangled,
            0 as *const libc::c_char,
            '\0' as i32 as libc::c_char,
        );
        if mangled.is_null() {
            return 0 as *const libc::c_char;
        }
        if args != 0 as libc::c_int as libc::c_ulong {
            string_append(decl, b", \0" as *const u8 as *const libc::c_char);
        }
    }
    string_append(decl, b")\0" as *const u8 as *const libc::c_char);
    return mangled;
}
unsafe extern "C" fn dlang_value(
    mut decl: *mut string,
    mut mangled: *const libc::c_char,
    mut name: *const libc::c_char,
    mut type_0: libc::c_char,
) -> *const libc::c_char {
    if mangled.is_null() || *mangled as libc::c_int == '\0' as i32 {
        return 0 as *const libc::c_char;
    }
    let mut current_block_26: u64;
    match *mangled as libc::c_int {
        110 => {
            mangled = mangled.offset(1);
            mangled;
            string_append(decl, b"null\0" as *const u8 as *const libc::c_char);
            current_block_26 = 4775909272756257391;
        }
        78 => {
            mangled = mangled.offset(1);
            mangled;
            string_append(decl, b"-\0" as *const u8 as *const libc::c_char);
            mangled = dlang_parse_integer(decl, mangled, type_0);
            current_block_26 = 4775909272756257391;
        }
        105 => {
            mangled = mangled.offset(1);
            mangled;
            current_block_26 = 10239502229319570081;
        }
        48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
            current_block_26 = 10239502229319570081;
        }
        101 => {
            mangled = mangled.offset(1);
            mangled;
            mangled = dlang_parse_real(decl, mangled);
            current_block_26 = 4775909272756257391;
        }
        99 => {
            mangled = mangled.offset(1);
            mangled;
            mangled = dlang_parse_real(decl, mangled);
            string_append(decl, b"+\0" as *const u8 as *const libc::c_char);
            if mangled.is_null() || *mangled as libc::c_int != 'c' as i32 {
                return 0 as *const libc::c_char;
            }
            mangled = mangled.offset(1);
            mangled;
            mangled = dlang_parse_real(decl, mangled);
            string_append(decl, b"i\0" as *const u8 as *const libc::c_char);
            current_block_26 = 4775909272756257391;
        }
        97 => {
            current_block_26 = 14805713993715184369;
        }
        119 | 100 => {
            current_block_26 = 14805713993715184369;
        }
        65 => {
            mangled = mangled.offset(1);
            mangled;
            if type_0 as libc::c_int == 'H' as i32 {
                mangled = dlang_parse_assocarray(decl, mangled);
            } else {
                mangled = dlang_parse_arrayliteral(decl, mangled);
            }
            current_block_26 = 4775909272756257391;
        }
        83 => {
            mangled = mangled.offset(1);
            mangled;
            mangled = dlang_parse_structlit(decl, mangled, name);
            current_block_26 = 4775909272756257391;
        }
        _ => return 0 as *const libc::c_char,
    }
    match current_block_26 {
        10239502229319570081 => {
            mangled = dlang_parse_integer(decl, mangled, type_0);
        }
        14805713993715184369 => {
            mangled = dlang_parse_string(decl, mangled);
        }
        _ => {}
    }
    return mangled;
}
unsafe extern "C" fn dlang_parse_mangle(
    mut decl: *mut string,
    mut mangled: *const libc::c_char,
    mut info: *mut dlang_info,
) -> *const libc::c_char {
    mangled = mangled.offset(2 as libc::c_int as isize);
    mangled = dlang_parse_qualified(decl, mangled, info, 1 as libc::c_int);
    if !mangled.is_null() {
        if *mangled as libc::c_int == 'Z' as i32 {
            mangled = mangled.offset(1);
            mangled;
        } else {
            let mut type_0: string = string {
                b: 0 as *mut libc::c_char,
                p: 0 as *mut libc::c_char,
                e: 0 as *mut libc::c_char,
            };
            string_init(&mut type_0);
            mangled = dlang_type(&mut type_0, mangled, info);
            string_delete(&mut type_0);
        }
    }
    return mangled;
}
unsafe extern "C" fn dlang_parse_qualified(
    mut decl: *mut string,
    mut mangled: *const libc::c_char,
    mut info: *mut dlang_info,
    mut suffix_modifiers: libc::c_int,
) -> *const libc::c_char {
    let mut n: size_t = 0 as libc::c_int as size_t;
    loop {
        let fresh5 = n;
        n = n.wrapping_add(1);
        if fresh5 != 0 {
            string_append(decl, b".\0" as *const u8 as *const libc::c_char);
        }
        while *mangled as libc::c_int == '0' as i32 {
            mangled = mangled.offset(1);
            mangled;
        }
        mangled = dlang_identifier(decl, mangled, info);
        if !mangled.is_null()
            && (*mangled as libc::c_int == 'M' as i32
                || dlang_call_convention_p(mangled) != 0)
        {
            let mut mods: string = string {
                b: 0 as *mut libc::c_char,
                p: 0 as *mut libc::c_char,
                e: 0 as *mut libc::c_char,
            };
            let mut start: *const libc::c_char = mangled;
            let mut saved: libc::c_int = string_length(decl);
            string_init(&mut mods);
            if *mangled as libc::c_int == 'M' as i32 {
                mangled = mangled.offset(1);
                mangled;
                mangled = dlang_type_modifiers(&mut mods, mangled);
                string_setlength(decl, saved);
            }
            mangled = dlang_function_type_noreturn(
                decl,
                0 as *mut string,
                0 as *mut string,
                mangled,
                info,
            );
            if suffix_modifiers != 0 {
                string_appendn(decl, mods.b, string_length(&mut mods) as size_t);
            }
            if mangled.is_null() || *mangled as libc::c_int == '\0' as i32 {
                mangled = start;
                string_setlength(decl, saved);
            }
            string_delete(&mut mods);
        }
        if !(!mangled.is_null() && dlang_symbol_name_p(mangled, info) != 0) {
            break;
        }
    }
    return mangled;
}
unsafe extern "C" fn dlang_parse_tuple(
    mut decl: *mut string,
    mut mangled: *const libc::c_char,
    mut info: *mut dlang_info,
) -> *const libc::c_char {
    let mut elements: libc::c_ulong = 0;
    mangled = dlang_number(mangled, &mut elements);
    if mangled.is_null() {
        return 0 as *const libc::c_char;
    }
    string_append(decl, b"Tuple!(\0" as *const u8 as *const libc::c_char);
    loop {
        let fresh6 = elements;
        elements = elements.wrapping_sub(1);
        if !(fresh6 != 0) {
            break;
        }
        mangled = dlang_type(decl, mangled, info);
        if mangled.is_null() {
            return 0 as *const libc::c_char;
        }
        if elements != 0 as libc::c_int as libc::c_ulong {
            string_append(decl, b", \0" as *const u8 as *const libc::c_char);
        }
    }
    string_append(decl, b")\0" as *const u8 as *const libc::c_char);
    return mangled;
}
unsafe extern "C" fn dlang_template_symbol_param(
    mut decl: *mut string,
    mut mangled: *const libc::c_char,
    mut info: *mut dlang_info,
) -> *const libc::c_char {
    if strncmp(
        mangled,
        b"_D\0" as *const u8 as *const libc::c_char,
        2 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int
        && dlang_symbol_name_p(mangled.offset(2 as libc::c_int as isize), info) != 0
    {
        return dlang_parse_mangle(decl, mangled, info);
    }
    if *mangled as libc::c_int == 'Q' as i32 {
        return dlang_parse_qualified(decl, mangled, info, 0 as libc::c_int);
    }
    let mut len: libc::c_ulong = 0;
    let mut endptr: *const libc::c_char = dlang_number(mangled, &mut len);
    if endptr.is_null() || len == 0 as libc::c_int as libc::c_ulong {
        return 0 as *const libc::c_char;
    }
    let mut psize: libc::c_long = len as libc::c_long;
    let mut pend: *const libc::c_char = 0 as *const libc::c_char;
    let mut saved: libc::c_int = string_length(decl);
    pend = endptr;
    while !endptr.is_null() {
        mangled = pend;
        if psize == 0 as libc::c_int as libc::c_long {
            psize = len as libc::c_long;
            pend = endptr;
            endptr = 0 as *const libc::c_char;
        }
        if dlang_symbol_name_p(mangled, info) != 0 {
            mangled = dlang_parse_qualified(decl, mangled, info, 0 as libc::c_int);
        } else if strncmp(
            mangled,
            b"_D\0" as *const u8 as *const libc::c_char,
            2 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
            && dlang_symbol_name_p(mangled.offset(2 as libc::c_int as isize), info) != 0
        {
            mangled = dlang_parse_mangle(decl, mangled, info);
        }
        if !mangled.is_null()
            && (endptr.is_null() || mangled.offset_from(pend) as libc::c_long == psize)
        {
            return mangled;
        }
        psize /= 10 as libc::c_int as libc::c_long;
        string_setlength(decl, saved);
        pend = pend.offset(-1);
        pend;
    }
    return 0 as *const libc::c_char;
}
unsafe extern "C" fn dlang_template_args(
    mut decl: *mut string,
    mut mangled: *const libc::c_char,
    mut info: *mut dlang_info,
) -> *const libc::c_char {
    let mut n: size_t = 0 as libc::c_int as size_t;
    while !mangled.is_null() && *mangled as libc::c_int != '\0' as i32 {
        match *mangled as libc::c_int {
            90 => {
                mangled = mangled.offset(1);
                mangled;
                return mangled;
            }
            _ => {}
        }
        let fresh7 = n;
        n = n.wrapping_add(1);
        if fresh7 != 0 {
            string_append(decl, b", \0" as *const u8 as *const libc::c_char);
        }
        if *mangled as libc::c_int == 'H' as i32 {
            mangled = mangled.offset(1);
            mangled;
        }
        match *mangled as libc::c_int {
            83 => {
                mangled = mangled.offset(1);
                mangled;
                mangled = dlang_template_symbol_param(decl, mangled, info);
            }
            84 => {
                mangled = mangled.offset(1);
                mangled;
                mangled = dlang_type(decl, mangled, info);
            }
            86 => {
                let mut name: string = string {
                    b: 0 as *mut libc::c_char,
                    p: 0 as *mut libc::c_char,
                    e: 0 as *mut libc::c_char,
                };
                let mut type_0: libc::c_char = 0;
                mangled = mangled.offset(1);
                mangled;
                type_0 = *mangled;
                if type_0 as libc::c_int == 'Q' as i32 {
                    let mut backref: *const libc::c_char = 0 as *const libc::c_char;
                    if (dlang_backref(mangled, &mut backref, info)).is_null() {
                        return 0 as *const libc::c_char;
                    }
                    type_0 = *backref;
                }
                string_init(&mut name);
                mangled = dlang_type(&mut name, mangled, info);
                string_need(&mut name, 1 as libc::c_int as size_t);
                *name.p = '\0' as i32 as libc::c_char;
                mangled = dlang_value(decl, mangled, name.b, type_0);
                string_delete(&mut name);
            }
            88 => {
                let mut len: libc::c_ulong = 0;
                let mut endptr: *const libc::c_char = 0 as *const libc::c_char;
                mangled = mangled.offset(1);
                mangled;
                endptr = dlang_number(mangled, &mut len);
                if endptr.is_null() || strlen(endptr) < len {
                    return 0 as *const libc::c_char;
                }
                string_appendn(decl, endptr, len);
                mangled = endptr.offset(len as isize);
            }
            _ => return 0 as *const libc::c_char,
        }
    }
    return mangled;
}
unsafe extern "C" fn dlang_parse_template(
    mut decl: *mut string,
    mut mangled: *const libc::c_char,
    mut info: *mut dlang_info,
    mut len: libc::c_ulong,
) -> *const libc::c_char {
    let mut start: *const libc::c_char = mangled;
    let mut args: string = string {
        b: 0 as *mut libc::c_char,
        p: 0 as *mut libc::c_char,
        e: 0 as *mut libc::c_char,
    };
    if dlang_symbol_name_p(mangled.offset(3 as libc::c_int as isize), info) == 0
        || *mangled.offset(3 as libc::c_int as isize) as libc::c_int == '0' as i32
    {
        return 0 as *const libc::c_char;
    }
    mangled = mangled.offset(3 as libc::c_int as isize);
    mangled = dlang_identifier(decl, mangled, info);
    string_init(&mut args);
    mangled = dlang_template_args(&mut args, mangled, info);
    string_append(decl, b"!(\0" as *const u8 as *const libc::c_char);
    string_appendn(decl, args.b, string_length(&mut args) as size_t);
    string_append(decl, b")\0" as *const u8 as *const libc::c_char);
    string_delete(&mut args);
    if len != (1 as libc::c_ulong).wrapping_neg() && !mangled.is_null()
        && mangled.offset_from(start) as libc::c_long as libc::c_ulong != len
    {
        return 0 as *const libc::c_char;
    }
    return mangled;
}
unsafe extern "C" fn dlang_demangle_init_info(
    mut mangled: *const libc::c_char,
    mut last_backref: libc::c_int,
    mut info: *mut dlang_info,
) {
    (*info).s = mangled;
    (*info).last_backref = last_backref;
}
#[no_mangle]
pub unsafe extern "C" fn dlang_demangle(
    mut mangled: *const libc::c_char,
    mut option: libc::c_int,
) -> *mut libc::c_char {
    let mut decl: string = string {
        b: 0 as *mut libc::c_char,
        p: 0 as *mut libc::c_char,
        e: 0 as *mut libc::c_char,
    };
    let mut demangled: *mut libc::c_char = 0 as *mut libc::c_char;
    if mangled.is_null() || *mangled as libc::c_int == '\0' as i32 {
        return 0 as *mut libc::c_char;
    }
    if strncmp(
        mangled,
        b"_D\0" as *const u8 as *const libc::c_char,
        2 as libc::c_int as libc::c_ulong,
    ) != 0 as libc::c_int
    {
        return 0 as *mut libc::c_char;
    }
    string_init(&mut decl);
    if strcmp(mangled, b"_Dmain\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        string_append(&mut decl, b"D main\0" as *const u8 as *const libc::c_char);
    } else {
        let mut info: dlang_info = dlang_info {
            s: 0 as *const libc::c_char,
            last_backref: 0,
        };
        dlang_demangle_init_info(mangled, strlen(mangled) as libc::c_int, &mut info);
        mangled = dlang_parse_mangle(&mut decl, mangled, &mut info);
        if mangled.is_null() || *mangled as libc::c_int != '\0' as i32 {
            string_delete(&mut decl);
        }
    }
    if string_length(&mut decl) > 0 as libc::c_int {
        string_need(&mut decl, 1 as libc::c_int as size_t);
        *decl.p = '\0' as i32 as libc::c_char;
        demangled = decl.b;
    }
    return demangled;
}
