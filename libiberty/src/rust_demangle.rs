use ::libc;
extern "C" {
    static _sch_istable: [libc::c_ushort; 256];
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
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
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type size_t = libc::c_ulong;
pub type uint = libc::c_uint;
pub type demangle_callbackref = Option::<
    unsafe extern "C" fn(*const libc::c_char, size_t, *mut libc::c_void) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rust_demangler {
    pub sym: *const libc::c_char,
    pub sym_len: size_t,
    pub callback_opaque: *mut libc::c_void,
    pub callback: demangle_callbackref,
    pub next: size_t,
    pub errored: libc::c_int,
    pub skipping_printing: libc::c_int,
    pub verbose: libc::c_int,
    pub version: libc::c_int,
    pub recursion: uint,
    pub bound_lifetime_depth: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rust_mangled_ident {
    pub ascii: *const libc::c_char,
    pub ascii_len: size_t,
    pub punycode: *const libc::c_char,
    pub punycode_len: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct str_buf {
    pub ptr: *mut libc::c_char,
    pub len: size_t,
    pub cap: size_t,
    pub errored: libc::c_int,
}
unsafe extern "C" fn peek(mut rdm: *const rust_demangler) -> libc::c_char {
    if (*rdm).next < (*rdm).sym_len {
        return *((*rdm).sym).offset((*rdm).next as isize);
    }
    return 0 as libc::c_int as libc::c_char;
}
unsafe extern "C" fn eat(
    mut rdm: *mut rust_demangler,
    mut c: libc::c_char,
) -> libc::c_int {
    if peek(rdm) as libc::c_int == c as libc::c_int {
        (*rdm).next = ((*rdm).next).wrapping_add(1);
        (*rdm).next;
        return 1 as libc::c_int;
    } else {
        return 0 as libc::c_int
    };
}
unsafe extern "C" fn next(mut rdm: *mut rust_demangler) -> libc::c_char {
    let mut c: libc::c_char = peek(rdm);
    if c == 0 {
        (*rdm).errored = 1 as libc::c_int;
    } else {
        (*rdm).next = ((*rdm).next).wrapping_add(1);
        (*rdm).next;
    }
    return c;
}
unsafe extern "C" fn parse_integer_62(mut rdm: *mut rust_demangler) -> uint64_t {
    let mut c: libc::c_char = 0;
    let mut x: uint64_t = 0;
    if eat(rdm, '_' as i32 as libc::c_char) != 0 {
        return 0 as libc::c_int as uint64_t;
    }
    x = 0 as libc::c_int as uint64_t;
    while eat(rdm, '_' as i32 as libc::c_char) == 0 {
        c = next(rdm);
        x = (x as libc::c_ulong).wrapping_mul(62 as libc::c_int as libc::c_ulong)
            as uint64_t as uint64_t;
        if _sch_istable[(c as libc::c_int & 0xff as libc::c_int) as usize] as libc::c_int
            & _sch_isdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            x = (x as libc::c_ulong)
                .wrapping_add((c as libc::c_int - '0' as i32) as libc::c_ulong)
                as uint64_t as uint64_t;
        } else if _sch_istable[(c as libc::c_int & 0xff as libc::c_int) as usize]
            as libc::c_int & _sch_islower as libc::c_int as libc::c_ushort as libc::c_int
            != 0
        {
            x = (x as libc::c_ulong)
                .wrapping_add(
                    (10 as libc::c_int + (c as libc::c_int - 'a' as i32))
                        as libc::c_ulong,
                ) as uint64_t as uint64_t;
        } else if _sch_istable[(c as libc::c_int & 0xff as libc::c_int) as usize]
            as libc::c_int & _sch_isupper as libc::c_int as libc::c_ushort as libc::c_int
            != 0
        {
            x = (x as libc::c_ulong)
                .wrapping_add(
                    (10 as libc::c_int + 26 as libc::c_int
                        + (c as libc::c_int - 'A' as i32)) as libc::c_ulong,
                ) as uint64_t as uint64_t;
        } else {
            (*rdm).errored = 1 as libc::c_int;
            return 0 as libc::c_int as uint64_t;
        }
    }
    return x.wrapping_add(1 as libc::c_int as libc::c_ulong);
}
unsafe extern "C" fn parse_opt_integer_62(
    mut rdm: *mut rust_demangler,
    mut tag: libc::c_char,
) -> uint64_t {
    if eat(rdm, tag) == 0 {
        return 0 as libc::c_int as uint64_t;
    }
    return (1 as libc::c_int as libc::c_ulong).wrapping_add(parse_integer_62(rdm));
}
unsafe extern "C" fn parse_disambiguator(mut rdm: *mut rust_demangler) -> uint64_t {
    return parse_opt_integer_62(rdm, 's' as i32 as libc::c_char);
}
unsafe extern "C" fn parse_hex_nibbles(
    mut rdm: *mut rust_demangler,
    mut value: *mut uint64_t,
) -> size_t {
    let mut c: libc::c_char = 0;
    let mut hex_len: size_t = 0;
    hex_len = 0 as libc::c_int as size_t;
    *value = 0 as libc::c_int as uint64_t;
    while eat(rdm, '_' as i32 as libc::c_char) == 0 {
        *value <<= 4 as libc::c_int;
        c = next(rdm);
        if _sch_istable[(c as libc::c_int & 0xff as libc::c_int) as usize] as libc::c_int
            & _sch_isdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            *value |= (c as libc::c_int - '0' as i32) as libc::c_ulong;
        } else if c as libc::c_int >= 'a' as i32 && c as libc::c_int <= 'f' as i32 {
            *value
                |= (10 as libc::c_int + (c as libc::c_int - 'a' as i32))
                    as libc::c_ulong;
        } else {
            (*rdm).errored = 1 as libc::c_int;
            return 0 as libc::c_int as size_t;
        }
        hex_len = hex_len.wrapping_add(1);
        hex_len;
    }
    return hex_len;
}
unsafe extern "C" fn parse_ident(mut rdm: *mut rust_demangler) -> rust_mangled_ident {
    let mut c: libc::c_char = 0;
    let mut start: size_t = 0;
    let mut len: size_t = 0;
    let mut is_punycode: libc::c_int = 0 as libc::c_int;
    let mut ident: rust_mangled_ident = rust_mangled_ident {
        ascii: 0 as *const libc::c_char,
        ascii_len: 0,
        punycode: 0 as *const libc::c_char,
        punycode_len: 0,
    };
    ident.ascii = 0 as *const libc::c_char;
    ident.ascii_len = 0 as libc::c_int as size_t;
    ident.punycode = 0 as *const libc::c_char;
    ident.punycode_len = 0 as libc::c_int as size_t;
    if (*rdm).version != -(1 as libc::c_int) {
        is_punycode = eat(rdm, 'u' as i32 as libc::c_char);
    }
    c = next(rdm);
    if _sch_istable[(c as libc::c_int & 0xff as libc::c_int) as usize] as libc::c_int
        & _sch_isdigit as libc::c_int as libc::c_ushort as libc::c_int == 0
    {
        (*rdm).errored = 1 as libc::c_int;
        return ident;
    }
    len = (c as libc::c_int - '0' as i32) as size_t;
    if c as libc::c_int != '0' as i32 {
        while _sch_istable[(peek(rdm) as libc::c_int & 0xff as libc::c_int) as usize]
            as libc::c_int & _sch_isdigit as libc::c_int as libc::c_ushort as libc::c_int
            != 0
        {
            len = len
                .wrapping_mul(10 as libc::c_int as libc::c_ulong)
                .wrapping_add((next(rdm) as libc::c_int - '0' as i32) as libc::c_ulong);
        }
    }
    if (*rdm).version != -(1 as libc::c_int) {
        eat(rdm, '_' as i32 as libc::c_char);
    }
    start = (*rdm).next;
    (*rdm).next = ((*rdm).next as libc::c_ulong).wrapping_add(len) as size_t as size_t;
    if start > (*rdm).next || (*rdm).next > (*rdm).sym_len {
        (*rdm).errored = 1 as libc::c_int;
        return ident;
    }
    ident.ascii = ((*rdm).sym).offset(start as isize);
    ident.ascii_len = len;
    if is_punycode != 0 {
        ident.punycode_len = 0 as libc::c_int as size_t;
        while ident.ascii_len > 0 as libc::c_int as libc::c_ulong {
            ident.ascii_len = (ident.ascii_len).wrapping_sub(1);
            ident.ascii_len;
            if *(ident.ascii).offset(ident.ascii_len as isize) as libc::c_int
                == '_' as i32
            {
                break;
            }
            ident.punycode_len = (ident.punycode_len).wrapping_add(1);
            ident.punycode_len;
        }
        if ident.punycode_len == 0 {
            (*rdm).errored = 1 as libc::c_int;
            return ident;
        }
        ident
            .punycode = (ident.ascii)
            .offset(len.wrapping_sub(ident.punycode_len) as isize);
    }
    if ident.ascii_len == 0 as libc::c_int as libc::c_ulong {
        ident.ascii = 0 as *const libc::c_char;
    }
    return ident;
}
unsafe extern "C" fn print_str(
    mut rdm: *mut rust_demangler,
    mut data: *const libc::c_char,
    mut len: size_t,
) {
    if (*rdm).errored == 0 && (*rdm).skipping_printing == 0 {
        ((*rdm).callback)
            .expect("non-null function pointer")(data, len, (*rdm).callback_opaque);
    }
}
unsafe extern "C" fn print_uint64(mut rdm: *mut rust_demangler, mut x: uint64_t) {
    let mut s: [libc::c_char; 21] = [0; 21];
    snprintf(
        s.as_mut_ptr(),
        21 as libc::c_int as libc::c_ulong,
        b"%lu\0" as *const u8 as *const libc::c_char,
        x,
    );
    print_str(rdm, s.as_mut_ptr(), strlen(s.as_mut_ptr()));
}
unsafe extern "C" fn print_uint64_hex(mut rdm: *mut rust_demangler, mut x: uint64_t) {
    let mut s: [libc::c_char; 17] = [0; 17];
    snprintf(
        s.as_mut_ptr(),
        17 as libc::c_int as libc::c_ulong,
        b"%lx\0" as *const u8 as *const libc::c_char,
        x,
    );
    print_str(rdm, s.as_mut_ptr(), strlen(s.as_mut_ptr()));
}
unsafe extern "C" fn decode_lower_hex_nibble(mut nibble: libc::c_char) -> libc::c_int {
    if '0' as i32 <= nibble as libc::c_int && nibble as libc::c_int <= '9' as i32 {
        return nibble as libc::c_int - '0' as i32;
    }
    if 'a' as i32 <= nibble as libc::c_int && nibble as libc::c_int <= 'f' as i32 {
        return 0xa as libc::c_int + (nibble as libc::c_int - 'a' as i32);
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn decode_legacy_escape(
    mut e: *const libc::c_char,
    mut len: size_t,
    mut out_len: *mut size_t,
) -> libc::c_char {
    let mut c: libc::c_char = 0 as libc::c_int as libc::c_char;
    let mut escape_len: size_t = 0 as libc::c_int as size_t;
    let mut lo_nibble: libc::c_int = -(1 as libc::c_int);
    let mut hi_nibble: libc::c_int = -(1 as libc::c_int);
    if len < 3 as libc::c_int as libc::c_ulong
        || *e.offset(0 as libc::c_int as isize) as libc::c_int != '$' as i32
    {
        return 0 as libc::c_int as libc::c_char;
    }
    e = e.offset(1);
    e;
    len = len.wrapping_sub(1);
    len;
    if *e.offset(0 as libc::c_int as isize) as libc::c_int == 'C' as i32 {
        escape_len = 1 as libc::c_int as size_t;
        c = ',' as i32 as libc::c_char;
    } else if len > 2 as libc::c_int as libc::c_ulong {
        escape_len = 2 as libc::c_int as size_t;
        if *e.offset(0 as libc::c_int as isize) as libc::c_int == 'S' as i32
            && *e.offset(1 as libc::c_int as isize) as libc::c_int == 'P' as i32
        {
            c = '@' as i32 as libc::c_char;
        } else if *e.offset(0 as libc::c_int as isize) as libc::c_int == 'B' as i32
            && *e.offset(1 as libc::c_int as isize) as libc::c_int == 'P' as i32
        {
            c = '*' as i32 as libc::c_char;
        } else if *e.offset(0 as libc::c_int as isize) as libc::c_int == 'R' as i32
            && *e.offset(1 as libc::c_int as isize) as libc::c_int == 'F' as i32
        {
            c = '&' as i32 as libc::c_char;
        } else if *e.offset(0 as libc::c_int as isize) as libc::c_int == 'L' as i32
            && *e.offset(1 as libc::c_int as isize) as libc::c_int == 'T' as i32
        {
            c = '<' as i32 as libc::c_char;
        } else if *e.offset(0 as libc::c_int as isize) as libc::c_int == 'G' as i32
            && *e.offset(1 as libc::c_int as isize) as libc::c_int == 'T' as i32
        {
            c = '>' as i32 as libc::c_char;
        } else if *e.offset(0 as libc::c_int as isize) as libc::c_int == 'L' as i32
            && *e.offset(1 as libc::c_int as isize) as libc::c_int == 'P' as i32
        {
            c = '(' as i32 as libc::c_char;
        } else if *e.offset(0 as libc::c_int as isize) as libc::c_int == 'R' as i32
            && *e.offset(1 as libc::c_int as isize) as libc::c_int == 'P' as i32
        {
            c = ')' as i32 as libc::c_char;
        } else if *e.offset(0 as libc::c_int as isize) as libc::c_int == 'u' as i32
            && len > 3 as libc::c_int as libc::c_ulong
        {
            escape_len = 3 as libc::c_int as size_t;
            hi_nibble = decode_lower_hex_nibble(*e.offset(1 as libc::c_int as isize));
            if hi_nibble < 0 as libc::c_int {
                return 0 as libc::c_int as libc::c_char;
            }
            lo_nibble = decode_lower_hex_nibble(*e.offset(2 as libc::c_int as isize));
            if lo_nibble < 0 as libc::c_int {
                return 0 as libc::c_int as libc::c_char;
            }
            if hi_nibble > 7 as libc::c_int {
                return 0 as libc::c_int as libc::c_char;
            }
            c = (hi_nibble << 4 as libc::c_int | lo_nibble) as libc::c_char;
            if (c as libc::c_int) < 0x20 as libc::c_int {
                return 0 as libc::c_int as libc::c_char;
            }
        }
    }
    if c == 0 || len <= escape_len
        || *e.offset(escape_len as isize) as libc::c_int != '$' as i32
    {
        return 0 as libc::c_int as libc::c_char;
    }
    *out_len = (2 as libc::c_int as libc::c_ulong).wrapping_add(escape_len);
    return c;
}
unsafe extern "C" fn print_ident(
    mut rdm: *mut rust_demangler,
    mut ident: rust_mangled_ident,
) {
    let mut current_block: u64;
    let mut unescaped: libc::c_char = 0;
    let mut out: *mut uint8_t = 0 as *mut uint8_t;
    let mut p: *mut uint8_t = 0 as *mut uint8_t;
    let mut d: uint8_t = 0;
    let mut len: size_t = 0;
    let mut cap: size_t = 0;
    let mut punycode_pos: size_t = 0;
    let mut j: size_t = 0;
    let mut c: uint32_t = 0;
    let mut base: size_t = 0;
    let mut t_min: size_t = 0;
    let mut t_max: size_t = 0;
    let mut skew: size_t = 0;
    let mut damp: size_t = 0;
    let mut bias: size_t = 0;
    let mut i: size_t = 0;
    let mut delta: size_t = 0;
    let mut w: size_t = 0;
    let mut k: size_t = 0;
    let mut t: size_t = 0;
    if (*rdm).errored != 0 || (*rdm).skipping_printing != 0 {
        return;
    }
    if (*rdm).version == -(1 as libc::c_int) {
        if ident.ascii_len >= 2 as libc::c_int as libc::c_ulong
            && *(ident.ascii).offset(0 as libc::c_int as isize) as libc::c_int
                == '_' as i32
            && *(ident.ascii).offset(1 as libc::c_int as isize) as libc::c_int
                == '$' as i32
        {
            ident.ascii = (ident.ascii).offset(1);
            ident.ascii;
            ident.ascii_len = (ident.ascii_len).wrapping_sub(1);
            ident.ascii_len;
        }
        while ident.ascii_len > 0 as libc::c_int as libc::c_ulong {
            if *(ident.ascii).offset(0 as libc::c_int as isize) as libc::c_int
                == '$' as i32
            {
                unescaped = decode_legacy_escape(ident.ascii, ident.ascii_len, &mut len);
                if unescaped != 0 {
                    print_str(rdm, &mut unescaped, 1 as libc::c_int as size_t);
                } else {
                    print_str(rdm, ident.ascii, ident.ascii_len);
                    return;
                }
            } else if *(ident.ascii).offset(0 as libc::c_int as isize) as libc::c_int
                == '.' as i32
            {
                if ident.ascii_len >= 2 as libc::c_int as libc::c_ulong
                    && *(ident.ascii).offset(1 as libc::c_int as isize) as libc::c_int
                        == '.' as i32
                {
                    print_str(
                        rdm,
                        b"::\0" as *const u8 as *const libc::c_char,
                        strlen(b"::\0" as *const u8 as *const libc::c_char),
                    );
                    len = 2 as libc::c_int as size_t;
                } else {
                    print_str(
                        rdm,
                        b".\0" as *const u8 as *const libc::c_char,
                        strlen(b".\0" as *const u8 as *const libc::c_char),
                    );
                    len = 1 as libc::c_int as size_t;
                }
            } else {
                len = 0 as libc::c_int as size_t;
                while len < ident.ascii_len {
                    if *(ident.ascii).offset(len as isize) as libc::c_int == '$' as i32
                        || *(ident.ascii).offset(len as isize) as libc::c_int
                            == '.' as i32
                    {
                        break;
                    }
                    len = len.wrapping_add(1);
                    len;
                }
                print_str(rdm, ident.ascii, len);
            }
            ident.ascii = (ident.ascii).offset(len as isize);
            ident
                .ascii_len = (ident.ascii_len as libc::c_ulong).wrapping_sub(len)
                as size_t as size_t;
        }
        return;
    }
    if (ident.punycode).is_null() {
        print_str(rdm, ident.ascii, ident.ascii_len);
        return;
    }
    len = 0 as libc::c_int as size_t;
    cap = 4 as libc::c_int as size_t;
    while cap < ident.ascii_len {
        cap = (cap as libc::c_ulong).wrapping_mul(2 as libc::c_int as libc::c_ulong)
            as size_t as size_t;
        if cap
            .wrapping_mul(4 as libc::c_int as libc::c_ulong)
            .wrapping_div(4 as libc::c_int as libc::c_ulong) != cap
        {
            (*rdm).errored = 1 as libc::c_int;
            return;
        }
    }
    out = malloc(cap.wrapping_mul(4 as libc::c_int as libc::c_ulong)) as *mut uint8_t;
    if out.is_null() {
        (*rdm).errored = 1 as libc::c_int;
        return;
    }
    len = 0 as libc::c_int as size_t;
    while len < ident.ascii_len {
        p = out.offset((4 as libc::c_int as libc::c_ulong).wrapping_mul(len) as isize);
        *p.offset(0 as libc::c_int as isize) = 0 as libc::c_int as uint8_t;
        *p.offset(1 as libc::c_int as isize) = 0 as libc::c_int as uint8_t;
        *p.offset(2 as libc::c_int as isize) = 0 as libc::c_int as uint8_t;
        *p
            .offset(
                3 as libc::c_int as isize,
            ) = *(ident.ascii).offset(len as isize) as uint8_t;
        len = len.wrapping_add(1);
        len;
    }
    base = 36 as libc::c_int as size_t;
    t_min = 1 as libc::c_int as size_t;
    t_max = 26 as libc::c_int as size_t;
    skew = 38 as libc::c_int as size_t;
    damp = 700 as libc::c_int as size_t;
    bias = 72 as libc::c_int as size_t;
    i = 0 as libc::c_int as size_t;
    c = 0x80 as libc::c_int as uint32_t;
    punycode_pos = 0 as libc::c_int as size_t;
    's_268: loop {
        if !(punycode_pos < ident.punycode_len) {
            current_block = 496303045384785551;
            break;
        }
        delta = 0 as libc::c_int as size_t;
        w = 1 as libc::c_int as size_t;
        k = 0 as libc::c_int as size_t;
        loop {
            k = (k as libc::c_ulong).wrapping_add(base) as size_t as size_t;
            t = if k < bias {
                0 as libc::c_int as libc::c_ulong
            } else {
                k.wrapping_sub(bias)
            };
            if t < t_min {
                t = t_min;
            }
            if t > t_max {
                t = t_max;
            }
            if punycode_pos >= ident.punycode_len {
                current_block = 6953103552215617040;
                break 's_268;
            }
            let fresh0 = punycode_pos;
            punycode_pos = punycode_pos.wrapping_add(1);
            d = *(ident.punycode).offset(fresh0 as isize) as uint8_t;
            if _sch_istable[(d as libc::c_int & 0xff as libc::c_int) as usize]
                as libc::c_int
                & _sch_islower as libc::c_int as libc::c_ushort as libc::c_int != 0
            {
                d = (d as libc::c_int - 'a' as i32) as uint8_t;
            } else if _sch_istable[(d as libc::c_int & 0xff as libc::c_int) as usize]
                as libc::c_int
                & _sch_isdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
            {
                d = (26 as libc::c_int + (d as libc::c_int - '0' as i32)) as uint8_t;
            } else {
                (*rdm).errored = 1 as libc::c_int;
                current_block = 6953103552215617040;
                break 's_268;
            }
            delta = (delta as libc::c_ulong)
                .wrapping_add((d as libc::c_ulong).wrapping_mul(w)) as size_t as size_t;
            w = (w as libc::c_ulong).wrapping_mul(base.wrapping_sub(t)) as size_t
                as size_t;
            if !(d as libc::c_ulong >= t) {
                break;
            }
        }
        len = len.wrapping_add(1);
        len;
        i = (i as libc::c_ulong).wrapping_add(delta) as size_t as size_t;
        c = (c as libc::c_ulong).wrapping_add(i.wrapping_div(len)) as uint32_t
            as uint32_t;
        i = (i as libc::c_ulong).wrapping_rem(len) as size_t as size_t;
        if cap < len {
            cap = (cap as libc::c_ulong).wrapping_mul(2 as libc::c_int as libc::c_ulong)
                as size_t as size_t;
            if cap
                .wrapping_mul(4 as libc::c_int as libc::c_ulong)
                .wrapping_div(4 as libc::c_int as libc::c_ulong) != cap || cap < len
            {
                (*rdm).errored = 1 as libc::c_int;
                current_block = 6953103552215617040;
                break;
            }
        }
        p = realloc(
            out as *mut libc::c_void,
            cap.wrapping_mul(4 as libc::c_int as libc::c_ulong),
        ) as *mut uint8_t;
        if p.is_null() {
            (*rdm).errored = 1 as libc::c_int;
            current_block = 6953103552215617040;
            break;
        } else {
            out = p;
            p = out.offset(i.wrapping_mul(4 as libc::c_int as libc::c_ulong) as isize);
            memmove(
                p.offset(4 as libc::c_int as isize) as *mut libc::c_void,
                p as *const libc::c_void,
                len
                    .wrapping_sub(i)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(4 as libc::c_int as libc::c_ulong),
            );
            *p
                .offset(
                    0 as libc::c_int as isize,
                ) = (if c >= 0x10000 as libc::c_int as libc::c_uint {
                0xf0 as libc::c_int as libc::c_uint | c >> 18 as libc::c_int
            } else {
                0 as libc::c_int as libc::c_uint
            }) as uint8_t;
            *p
                .offset(
                    1 as libc::c_int as isize,
                ) = (if c >= 0x800 as libc::c_int as libc::c_uint {
                (if c < 0x10000 as libc::c_int as libc::c_uint {
                    0xe0 as libc::c_int
                } else {
                    0x80 as libc::c_int
                }) as libc::c_uint
                    | c >> 12 as libc::c_int & 0x3f as libc::c_int as libc::c_uint
            } else {
                0 as libc::c_int as libc::c_uint
            }) as uint8_t;
            *p
                .offset(
                    2 as libc::c_int as isize,
                ) = ((if c < 0x800 as libc::c_int as libc::c_uint {
                0xc0 as libc::c_int
            } else {
                0x80 as libc::c_int
            }) as libc::c_uint
                | c >> 6 as libc::c_int & 0x3f as libc::c_int as libc::c_uint)
                as uint8_t;
            *p
                .offset(
                    3 as libc::c_int as isize,
                ) = (0x80 as libc::c_int as libc::c_uint
                | c & 0x3f as libc::c_int as libc::c_uint) as uint8_t;
            if punycode_pos == ident.punycode_len {
                current_block = 496303045384785551;
                break;
            }
            i = i.wrapping_add(1);
            i;
            delta = (delta as libc::c_ulong).wrapping_div(damp) as size_t as size_t;
            damp = 2 as libc::c_int as size_t;
            delta = (delta as libc::c_ulong).wrapping_add(delta.wrapping_div(len))
                as size_t as size_t;
            k = 0 as libc::c_int as size_t;
            while delta
                > base
                    .wrapping_sub(t_min)
                    .wrapping_mul(t_max)
                    .wrapping_div(2 as libc::c_int as libc::c_ulong)
            {
                delta = (delta as libc::c_ulong).wrapping_div(base.wrapping_sub(t_min))
                    as size_t as size_t;
                k = (k as libc::c_ulong).wrapping_add(base) as size_t as size_t;
            }
            bias = k
                .wrapping_add(
                    base
                        .wrapping_sub(t_min)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(delta)
                        .wrapping_div(delta.wrapping_add(skew)),
                );
        }
    }
    match current_block {
        496303045384785551 => {
            i = 0 as libc::c_int as size_t;
            j = 0 as libc::c_int as size_t;
            while i < len.wrapping_mul(4 as libc::c_int as libc::c_ulong) {
                if *out.offset(i as isize) as libc::c_int != 0 as libc::c_int {
                    let fresh1 = j;
                    j = j.wrapping_add(1);
                    *out.offset(fresh1 as isize) = *out.offset(i as isize);
                }
                i = i.wrapping_add(1);
                i;
            }
            print_str(rdm, out as *const libc::c_char, j);
        }
        _ => {}
    }
    free(out as *mut libc::c_void);
}
unsafe extern "C" fn print_lifetime_from_index(
    mut rdm: *mut rust_demangler,
    mut lt: uint64_t,
) {
    let mut c: libc::c_char = 0;
    let mut depth: uint64_t = 0;
    print_str(
        rdm,
        b"'\0" as *const u8 as *const libc::c_char,
        strlen(b"'\0" as *const u8 as *const libc::c_char),
    );
    if lt == 0 as libc::c_int as libc::c_ulong {
        print_str(
            rdm,
            b"_\0" as *const u8 as *const libc::c_char,
            strlen(b"_\0" as *const u8 as *const libc::c_char),
        );
        return;
    }
    depth = ((*rdm).bound_lifetime_depth).wrapping_sub(lt);
    if depth < 26 as libc::c_int as libc::c_ulong {
        c = ('a' as i32 as libc::c_ulong).wrapping_add(depth) as libc::c_char;
        print_str(rdm, &mut c, 1 as libc::c_int as size_t);
    } else {
        print_str(
            rdm,
            b"_\0" as *const u8 as *const libc::c_char,
            strlen(b"_\0" as *const u8 as *const libc::c_char),
        );
        print_uint64(rdm, depth);
    };
}
unsafe extern "C" fn demangle_binder(mut rdm: *mut rust_demangler) {
    let mut i: uint64_t = 0;
    let mut bound_lifetimes: uint64_t = 0;
    if (*rdm).errored != 0 {
        return;
    }
    bound_lifetimes = parse_opt_integer_62(rdm, 'G' as i32 as libc::c_char);
    if bound_lifetimes > 0 as libc::c_int as libc::c_ulong {
        print_str(
            rdm,
            b"for<\0" as *const u8 as *const libc::c_char,
            strlen(b"for<\0" as *const u8 as *const libc::c_char),
        );
        i = 0 as libc::c_int as uint64_t;
        while i < bound_lifetimes {
            if i > 0 as libc::c_int as libc::c_ulong {
                print_str(
                    rdm,
                    b", \0" as *const u8 as *const libc::c_char,
                    strlen(b", \0" as *const u8 as *const libc::c_char),
                );
            }
            (*rdm).bound_lifetime_depth = ((*rdm).bound_lifetime_depth).wrapping_add(1);
            (*rdm).bound_lifetime_depth;
            print_lifetime_from_index(rdm, 1 as libc::c_int as uint64_t);
            i = i.wrapping_add(1);
            i;
        }
        print_str(
            rdm,
            b"> \0" as *const u8 as *const libc::c_char,
            strlen(b"> \0" as *const u8 as *const libc::c_char),
        );
    }
}
unsafe extern "C" fn demangle_path(
    mut rdm: *mut rust_demangler,
    mut in_value: libc::c_int,
) {
    let mut current_block: u64;
    let mut tag: libc::c_char = 0;
    let mut ns: libc::c_char = 0;
    let mut was_skipping_printing: libc::c_int = 0;
    let mut i: size_t = 0;
    let mut backref: size_t = 0;
    let mut old_next: size_t = 0;
    let mut dis: uint64_t = 0;
    let mut name: rust_mangled_ident = rust_mangled_ident {
        ascii: 0 as *const libc::c_char,
        ascii_len: 0,
        punycode: 0 as *const libc::c_char,
        punycode_len: 0,
    };
    if (*rdm).errored != 0 {
        return;
    }
    if (*rdm).recursion != -(1 as libc::c_int) as uint {
        (*rdm).recursion = ((*rdm).recursion).wrapping_add(1);
        (*rdm).recursion;
        if (*rdm).recursion > 1024 as libc::c_int as libc::c_uint {
            current_block = 3304099842109091603;
        } else {
            current_block = 15619007995458559411;
        }
    } else {
        current_block = 15619007995458559411;
    }
    match current_block {
        15619007995458559411 => {
            tag = next(rdm);
            match tag as libc::c_int {
                67 => {
                    current_block = 16681027592547421651;
                    match current_block {
                        17369132196136995081 => {
                            demangle_path(rdm, in_value);
                            if in_value != 0 {
                                print_str(
                                    rdm,
                                    b"::\0" as *const u8 as *const libc::c_char,
                                    strlen(b"::\0" as *const u8 as *const libc::c_char),
                                );
                            }
                            print_str(
                                rdm,
                                b"<\0" as *const u8 as *const libc::c_char,
                                strlen(b"<\0" as *const u8 as *const libc::c_char),
                            );
                            i = 0 as libc::c_int as size_t;
                            while (*rdm).errored == 0
                                && eat(rdm, 'E' as i32 as libc::c_char) == 0
                            {
                                if i > 0 as libc::c_int as libc::c_ulong {
                                    print_str(
                                        rdm,
                                        b", \0" as *const u8 as *const libc::c_char,
                                        strlen(b", \0" as *const u8 as *const libc::c_char),
                                    );
                                }
                                demangle_generic_arg(rdm);
                                i = i.wrapping_add(1);
                                i;
                            }
                            print_str(
                                rdm,
                                b">\0" as *const u8 as *const libc::c_char,
                                strlen(b">\0" as *const u8 as *const libc::c_char),
                            );
                            current_block = 15919730065710799250;
                        }
                        195292842195342531 => {
                            parse_disambiguator(rdm);
                            was_skipping_printing = (*rdm).skipping_printing;
                            (*rdm).skipping_printing = 1 as libc::c_int;
                            demangle_path(rdm, in_value);
                            (*rdm).skipping_printing = was_skipping_printing;
                            current_block = 405477181314099796;
                        }
                        11045451106458149833 => {
                            ns = next(rdm);
                            if _sch_istable[(ns as libc::c_int & 0xff as libc::c_int)
                                as usize] as libc::c_int
                                & _sch_islower as libc::c_int as libc::c_ushort
                                    as libc::c_int == 0
                                && _sch_istable[(ns as libc::c_int & 0xff as libc::c_int)
                                    as usize] as libc::c_int
                                    & _sch_isupper as libc::c_int as libc::c_ushort
                                        as libc::c_int == 0
                            {
                                current_block = 3304099842109091603;
                            } else {
                                demangle_path(rdm, in_value);
                                dis = parse_disambiguator(rdm);
                                name = parse_ident(rdm);
                                if _sch_istable[(ns as libc::c_int & 0xff as libc::c_int)
                                    as usize] as libc::c_int
                                    & _sch_isupper as libc::c_int as libc::c_ushort
                                        as libc::c_int != 0
                                {
                                    print_str(
                                        rdm,
                                        b"::{\0" as *const u8 as *const libc::c_char,
                                        strlen(b"::{\0" as *const u8 as *const libc::c_char),
                                    );
                                    match ns as libc::c_int {
                                        67 => {
                                            print_str(
                                                rdm,
                                                b"closure\0" as *const u8 as *const libc::c_char,
                                                strlen(b"closure\0" as *const u8 as *const libc::c_char),
                                            );
                                        }
                                        83 => {
                                            print_str(
                                                rdm,
                                                b"shim\0" as *const u8 as *const libc::c_char,
                                                strlen(b"shim\0" as *const u8 as *const libc::c_char),
                                            );
                                        }
                                        _ => {
                                            print_str(rdm, &mut ns, 1 as libc::c_int as size_t);
                                        }
                                    }
                                    if !(name.ascii).is_null() || !(name.punycode).is_null() {
                                        print_str(
                                            rdm,
                                            b":\0" as *const u8 as *const libc::c_char,
                                            strlen(b":\0" as *const u8 as *const libc::c_char),
                                        );
                                        print_ident(rdm, name);
                                    }
                                    print_str(
                                        rdm,
                                        b"#\0" as *const u8 as *const libc::c_char,
                                        strlen(b"#\0" as *const u8 as *const libc::c_char),
                                    );
                                    print_uint64(rdm, dis);
                                    print_str(
                                        rdm,
                                        b"}\0" as *const u8 as *const libc::c_char,
                                        strlen(b"}\0" as *const u8 as *const libc::c_char),
                                    );
                                } else if !(name.ascii).is_null()
                                    || !(name.punycode).is_null()
                                {
                                    print_str(
                                        rdm,
                                        b"::\0" as *const u8 as *const libc::c_char,
                                        strlen(b"::\0" as *const u8 as *const libc::c_char),
                                    );
                                    print_ident(rdm, name);
                                }
                                current_block = 15919730065710799250;
                            }
                        }
                        16681027592547421651 => {
                            dis = parse_disambiguator(rdm);
                            name = parse_ident(rdm);
                            print_ident(rdm, name);
                            if (*rdm).verbose != 0 {
                                print_str(
                                    rdm,
                                    b"[\0" as *const u8 as *const libc::c_char,
                                    strlen(b"[\0" as *const u8 as *const libc::c_char),
                                );
                                print_uint64_hex(rdm, dis);
                                print_str(
                                    rdm,
                                    b"]\0" as *const u8 as *const libc::c_char,
                                    strlen(b"]\0" as *const u8 as *const libc::c_char),
                                );
                            }
                            current_block = 15919730065710799250;
                        }
                        517681984938920942 => {
                            backref = parse_integer_62(rdm);
                            if (*rdm).skipping_printing == 0 {
                                old_next = (*rdm).next;
                                (*rdm).next = backref;
                                demangle_path(rdm, in_value);
                                (*rdm).next = old_next;
                            }
                            current_block = 15919730065710799250;
                        }
                        _ => {}
                    }
                    match current_block {
                        15919730065710799250 => {}
                        3304099842109091603 => {}
                        _ => {
                            print_str(
                                rdm,
                                b"<\0" as *const u8 as *const libc::c_char,
                                strlen(b"<\0" as *const u8 as *const libc::c_char),
                            );
                            demangle_type(rdm);
                            if tag as libc::c_int != 'M' as i32 {
                                print_str(
                                    rdm,
                                    b" as \0" as *const u8 as *const libc::c_char,
                                    strlen(b" as \0" as *const u8 as *const libc::c_char),
                                );
                                demangle_path(rdm, 0 as libc::c_int);
                            }
                            print_str(
                                rdm,
                                b">\0" as *const u8 as *const libc::c_char,
                                strlen(b">\0" as *const u8 as *const libc::c_char),
                            );
                            current_block = 15919730065710799250;
                        }
                    }
                }
                78 => {
                    current_block = 11045451106458149833;
                    match current_block {
                        17369132196136995081 => {
                            demangle_path(rdm, in_value);
                            if in_value != 0 {
                                print_str(
                                    rdm,
                                    b"::\0" as *const u8 as *const libc::c_char,
                                    strlen(b"::\0" as *const u8 as *const libc::c_char),
                                );
                            }
                            print_str(
                                rdm,
                                b"<\0" as *const u8 as *const libc::c_char,
                                strlen(b"<\0" as *const u8 as *const libc::c_char),
                            );
                            i = 0 as libc::c_int as size_t;
                            while (*rdm).errored == 0
                                && eat(rdm, 'E' as i32 as libc::c_char) == 0
                            {
                                if i > 0 as libc::c_int as libc::c_ulong {
                                    print_str(
                                        rdm,
                                        b", \0" as *const u8 as *const libc::c_char,
                                        strlen(b", \0" as *const u8 as *const libc::c_char),
                                    );
                                }
                                demangle_generic_arg(rdm);
                                i = i.wrapping_add(1);
                                i;
                            }
                            print_str(
                                rdm,
                                b">\0" as *const u8 as *const libc::c_char,
                                strlen(b">\0" as *const u8 as *const libc::c_char),
                            );
                            current_block = 15919730065710799250;
                        }
                        195292842195342531 => {
                            parse_disambiguator(rdm);
                            was_skipping_printing = (*rdm).skipping_printing;
                            (*rdm).skipping_printing = 1 as libc::c_int;
                            demangle_path(rdm, in_value);
                            (*rdm).skipping_printing = was_skipping_printing;
                            current_block = 405477181314099796;
                        }
                        11045451106458149833 => {
                            ns = next(rdm);
                            if _sch_istable[(ns as libc::c_int & 0xff as libc::c_int)
                                as usize] as libc::c_int
                                & _sch_islower as libc::c_int as libc::c_ushort
                                    as libc::c_int == 0
                                && _sch_istable[(ns as libc::c_int & 0xff as libc::c_int)
                                    as usize] as libc::c_int
                                    & _sch_isupper as libc::c_int as libc::c_ushort
                                        as libc::c_int == 0
                            {
                                current_block = 3304099842109091603;
                            } else {
                                demangle_path(rdm, in_value);
                                dis = parse_disambiguator(rdm);
                                name = parse_ident(rdm);
                                if _sch_istable[(ns as libc::c_int & 0xff as libc::c_int)
                                    as usize] as libc::c_int
                                    & _sch_isupper as libc::c_int as libc::c_ushort
                                        as libc::c_int != 0
                                {
                                    print_str(
                                        rdm,
                                        b"::{\0" as *const u8 as *const libc::c_char,
                                        strlen(b"::{\0" as *const u8 as *const libc::c_char),
                                    );
                                    match ns as libc::c_int {
                                        67 => {
                                            print_str(
                                                rdm,
                                                b"closure\0" as *const u8 as *const libc::c_char,
                                                strlen(b"closure\0" as *const u8 as *const libc::c_char),
                                            );
                                        }
                                        83 => {
                                            print_str(
                                                rdm,
                                                b"shim\0" as *const u8 as *const libc::c_char,
                                                strlen(b"shim\0" as *const u8 as *const libc::c_char),
                                            );
                                        }
                                        _ => {
                                            print_str(rdm, &mut ns, 1 as libc::c_int as size_t);
                                        }
                                    }
                                    if !(name.ascii).is_null() || !(name.punycode).is_null() {
                                        print_str(
                                            rdm,
                                            b":\0" as *const u8 as *const libc::c_char,
                                            strlen(b":\0" as *const u8 as *const libc::c_char),
                                        );
                                        print_ident(rdm, name);
                                    }
                                    print_str(
                                        rdm,
                                        b"#\0" as *const u8 as *const libc::c_char,
                                        strlen(b"#\0" as *const u8 as *const libc::c_char),
                                    );
                                    print_uint64(rdm, dis);
                                    print_str(
                                        rdm,
                                        b"}\0" as *const u8 as *const libc::c_char,
                                        strlen(b"}\0" as *const u8 as *const libc::c_char),
                                    );
                                } else if !(name.ascii).is_null()
                                    || !(name.punycode).is_null()
                                {
                                    print_str(
                                        rdm,
                                        b"::\0" as *const u8 as *const libc::c_char,
                                        strlen(b"::\0" as *const u8 as *const libc::c_char),
                                    );
                                    print_ident(rdm, name);
                                }
                                current_block = 15919730065710799250;
                            }
                        }
                        16681027592547421651 => {
                            dis = parse_disambiguator(rdm);
                            name = parse_ident(rdm);
                            print_ident(rdm, name);
                            if (*rdm).verbose != 0 {
                                print_str(
                                    rdm,
                                    b"[\0" as *const u8 as *const libc::c_char,
                                    strlen(b"[\0" as *const u8 as *const libc::c_char),
                                );
                                print_uint64_hex(rdm, dis);
                                print_str(
                                    rdm,
                                    b"]\0" as *const u8 as *const libc::c_char,
                                    strlen(b"]\0" as *const u8 as *const libc::c_char),
                                );
                            }
                            current_block = 15919730065710799250;
                        }
                        517681984938920942 => {
                            backref = parse_integer_62(rdm);
                            if (*rdm).skipping_printing == 0 {
                                old_next = (*rdm).next;
                                (*rdm).next = backref;
                                demangle_path(rdm, in_value);
                                (*rdm).next = old_next;
                            }
                            current_block = 15919730065710799250;
                        }
                        _ => {}
                    }
                    match current_block {
                        15919730065710799250 => {}
                        3304099842109091603 => {}
                        _ => {
                            print_str(
                                rdm,
                                b"<\0" as *const u8 as *const libc::c_char,
                                strlen(b"<\0" as *const u8 as *const libc::c_char),
                            );
                            demangle_type(rdm);
                            if tag as libc::c_int != 'M' as i32 {
                                print_str(
                                    rdm,
                                    b" as \0" as *const u8 as *const libc::c_char,
                                    strlen(b" as \0" as *const u8 as *const libc::c_char),
                                );
                                demangle_path(rdm, 0 as libc::c_int);
                            }
                            print_str(
                                rdm,
                                b">\0" as *const u8 as *const libc::c_char,
                                strlen(b">\0" as *const u8 as *const libc::c_char),
                            );
                            current_block = 15919730065710799250;
                        }
                    }
                }
                77 | 88 => {
                    current_block = 195292842195342531;
                    match current_block {
                        17369132196136995081 => {
                            demangle_path(rdm, in_value);
                            if in_value != 0 {
                                print_str(
                                    rdm,
                                    b"::\0" as *const u8 as *const libc::c_char,
                                    strlen(b"::\0" as *const u8 as *const libc::c_char),
                                );
                            }
                            print_str(
                                rdm,
                                b"<\0" as *const u8 as *const libc::c_char,
                                strlen(b"<\0" as *const u8 as *const libc::c_char),
                            );
                            i = 0 as libc::c_int as size_t;
                            while (*rdm).errored == 0
                                && eat(rdm, 'E' as i32 as libc::c_char) == 0
                            {
                                if i > 0 as libc::c_int as libc::c_ulong {
                                    print_str(
                                        rdm,
                                        b", \0" as *const u8 as *const libc::c_char,
                                        strlen(b", \0" as *const u8 as *const libc::c_char),
                                    );
                                }
                                demangle_generic_arg(rdm);
                                i = i.wrapping_add(1);
                                i;
                            }
                            print_str(
                                rdm,
                                b">\0" as *const u8 as *const libc::c_char,
                                strlen(b">\0" as *const u8 as *const libc::c_char),
                            );
                            current_block = 15919730065710799250;
                        }
                        195292842195342531 => {
                            parse_disambiguator(rdm);
                            was_skipping_printing = (*rdm).skipping_printing;
                            (*rdm).skipping_printing = 1 as libc::c_int;
                            demangle_path(rdm, in_value);
                            (*rdm).skipping_printing = was_skipping_printing;
                            current_block = 405477181314099796;
                        }
                        11045451106458149833 => {
                            ns = next(rdm);
                            if _sch_istable[(ns as libc::c_int & 0xff as libc::c_int)
                                as usize] as libc::c_int
                                & _sch_islower as libc::c_int as libc::c_ushort
                                    as libc::c_int == 0
                                && _sch_istable[(ns as libc::c_int & 0xff as libc::c_int)
                                    as usize] as libc::c_int
                                    & _sch_isupper as libc::c_int as libc::c_ushort
                                        as libc::c_int == 0
                            {
                                current_block = 3304099842109091603;
                            } else {
                                demangle_path(rdm, in_value);
                                dis = parse_disambiguator(rdm);
                                name = parse_ident(rdm);
                                if _sch_istable[(ns as libc::c_int & 0xff as libc::c_int)
                                    as usize] as libc::c_int
                                    & _sch_isupper as libc::c_int as libc::c_ushort
                                        as libc::c_int != 0
                                {
                                    print_str(
                                        rdm,
                                        b"::{\0" as *const u8 as *const libc::c_char,
                                        strlen(b"::{\0" as *const u8 as *const libc::c_char),
                                    );
                                    match ns as libc::c_int {
                                        67 => {
                                            print_str(
                                                rdm,
                                                b"closure\0" as *const u8 as *const libc::c_char,
                                                strlen(b"closure\0" as *const u8 as *const libc::c_char),
                                            );
                                        }
                                        83 => {
                                            print_str(
                                                rdm,
                                                b"shim\0" as *const u8 as *const libc::c_char,
                                                strlen(b"shim\0" as *const u8 as *const libc::c_char),
                                            );
                                        }
                                        _ => {
                                            print_str(rdm, &mut ns, 1 as libc::c_int as size_t);
                                        }
                                    }
                                    if !(name.ascii).is_null() || !(name.punycode).is_null() {
                                        print_str(
                                            rdm,
                                            b":\0" as *const u8 as *const libc::c_char,
                                            strlen(b":\0" as *const u8 as *const libc::c_char),
                                        );
                                        print_ident(rdm, name);
                                    }
                                    print_str(
                                        rdm,
                                        b"#\0" as *const u8 as *const libc::c_char,
                                        strlen(b"#\0" as *const u8 as *const libc::c_char),
                                    );
                                    print_uint64(rdm, dis);
                                    print_str(
                                        rdm,
                                        b"}\0" as *const u8 as *const libc::c_char,
                                        strlen(b"}\0" as *const u8 as *const libc::c_char),
                                    );
                                } else if !(name.ascii).is_null()
                                    || !(name.punycode).is_null()
                                {
                                    print_str(
                                        rdm,
                                        b"::\0" as *const u8 as *const libc::c_char,
                                        strlen(b"::\0" as *const u8 as *const libc::c_char),
                                    );
                                    print_ident(rdm, name);
                                }
                                current_block = 15919730065710799250;
                            }
                        }
                        16681027592547421651 => {
                            dis = parse_disambiguator(rdm);
                            name = parse_ident(rdm);
                            print_ident(rdm, name);
                            if (*rdm).verbose != 0 {
                                print_str(
                                    rdm,
                                    b"[\0" as *const u8 as *const libc::c_char,
                                    strlen(b"[\0" as *const u8 as *const libc::c_char),
                                );
                                print_uint64_hex(rdm, dis);
                                print_str(
                                    rdm,
                                    b"]\0" as *const u8 as *const libc::c_char,
                                    strlen(b"]\0" as *const u8 as *const libc::c_char),
                                );
                            }
                            current_block = 15919730065710799250;
                        }
                        517681984938920942 => {
                            backref = parse_integer_62(rdm);
                            if (*rdm).skipping_printing == 0 {
                                old_next = (*rdm).next;
                                (*rdm).next = backref;
                                demangle_path(rdm, in_value);
                                (*rdm).next = old_next;
                            }
                            current_block = 15919730065710799250;
                        }
                        _ => {}
                    }
                    match current_block {
                        15919730065710799250 => {}
                        3304099842109091603 => {}
                        _ => {
                            print_str(
                                rdm,
                                b"<\0" as *const u8 as *const libc::c_char,
                                strlen(b"<\0" as *const u8 as *const libc::c_char),
                            );
                            demangle_type(rdm);
                            if tag as libc::c_int != 'M' as i32 {
                                print_str(
                                    rdm,
                                    b" as \0" as *const u8 as *const libc::c_char,
                                    strlen(b" as \0" as *const u8 as *const libc::c_char),
                                );
                                demangle_path(rdm, 0 as libc::c_int);
                            }
                            print_str(
                                rdm,
                                b">\0" as *const u8 as *const libc::c_char,
                                strlen(b">\0" as *const u8 as *const libc::c_char),
                            );
                            current_block = 15919730065710799250;
                        }
                    }
                }
                89 => {
                    current_block = 405477181314099796;
                    match current_block {
                        17369132196136995081 => {
                            demangle_path(rdm, in_value);
                            if in_value != 0 {
                                print_str(
                                    rdm,
                                    b"::\0" as *const u8 as *const libc::c_char,
                                    strlen(b"::\0" as *const u8 as *const libc::c_char),
                                );
                            }
                            print_str(
                                rdm,
                                b"<\0" as *const u8 as *const libc::c_char,
                                strlen(b"<\0" as *const u8 as *const libc::c_char),
                            );
                            i = 0 as libc::c_int as size_t;
                            while (*rdm).errored == 0
                                && eat(rdm, 'E' as i32 as libc::c_char) == 0
                            {
                                if i > 0 as libc::c_int as libc::c_ulong {
                                    print_str(
                                        rdm,
                                        b", \0" as *const u8 as *const libc::c_char,
                                        strlen(b", \0" as *const u8 as *const libc::c_char),
                                    );
                                }
                                demangle_generic_arg(rdm);
                                i = i.wrapping_add(1);
                                i;
                            }
                            print_str(
                                rdm,
                                b">\0" as *const u8 as *const libc::c_char,
                                strlen(b">\0" as *const u8 as *const libc::c_char),
                            );
                            current_block = 15919730065710799250;
                        }
                        195292842195342531 => {
                            parse_disambiguator(rdm);
                            was_skipping_printing = (*rdm).skipping_printing;
                            (*rdm).skipping_printing = 1 as libc::c_int;
                            demangle_path(rdm, in_value);
                            (*rdm).skipping_printing = was_skipping_printing;
                            current_block = 405477181314099796;
                        }
                        11045451106458149833 => {
                            ns = next(rdm);
                            if _sch_istable[(ns as libc::c_int & 0xff as libc::c_int)
                                as usize] as libc::c_int
                                & _sch_islower as libc::c_int as libc::c_ushort
                                    as libc::c_int == 0
                                && _sch_istable[(ns as libc::c_int & 0xff as libc::c_int)
                                    as usize] as libc::c_int
                                    & _sch_isupper as libc::c_int as libc::c_ushort
                                        as libc::c_int == 0
                            {
                                current_block = 3304099842109091603;
                            } else {
                                demangle_path(rdm, in_value);
                                dis = parse_disambiguator(rdm);
                                name = parse_ident(rdm);
                                if _sch_istable[(ns as libc::c_int & 0xff as libc::c_int)
                                    as usize] as libc::c_int
                                    & _sch_isupper as libc::c_int as libc::c_ushort
                                        as libc::c_int != 0
                                {
                                    print_str(
                                        rdm,
                                        b"::{\0" as *const u8 as *const libc::c_char,
                                        strlen(b"::{\0" as *const u8 as *const libc::c_char),
                                    );
                                    match ns as libc::c_int {
                                        67 => {
                                            print_str(
                                                rdm,
                                                b"closure\0" as *const u8 as *const libc::c_char,
                                                strlen(b"closure\0" as *const u8 as *const libc::c_char),
                                            );
                                        }
                                        83 => {
                                            print_str(
                                                rdm,
                                                b"shim\0" as *const u8 as *const libc::c_char,
                                                strlen(b"shim\0" as *const u8 as *const libc::c_char),
                                            );
                                        }
                                        _ => {
                                            print_str(rdm, &mut ns, 1 as libc::c_int as size_t);
                                        }
                                    }
                                    if !(name.ascii).is_null() || !(name.punycode).is_null() {
                                        print_str(
                                            rdm,
                                            b":\0" as *const u8 as *const libc::c_char,
                                            strlen(b":\0" as *const u8 as *const libc::c_char),
                                        );
                                        print_ident(rdm, name);
                                    }
                                    print_str(
                                        rdm,
                                        b"#\0" as *const u8 as *const libc::c_char,
                                        strlen(b"#\0" as *const u8 as *const libc::c_char),
                                    );
                                    print_uint64(rdm, dis);
                                    print_str(
                                        rdm,
                                        b"}\0" as *const u8 as *const libc::c_char,
                                        strlen(b"}\0" as *const u8 as *const libc::c_char),
                                    );
                                } else if !(name.ascii).is_null()
                                    || !(name.punycode).is_null()
                                {
                                    print_str(
                                        rdm,
                                        b"::\0" as *const u8 as *const libc::c_char,
                                        strlen(b"::\0" as *const u8 as *const libc::c_char),
                                    );
                                    print_ident(rdm, name);
                                }
                                current_block = 15919730065710799250;
                            }
                        }
                        16681027592547421651 => {
                            dis = parse_disambiguator(rdm);
                            name = parse_ident(rdm);
                            print_ident(rdm, name);
                            if (*rdm).verbose != 0 {
                                print_str(
                                    rdm,
                                    b"[\0" as *const u8 as *const libc::c_char,
                                    strlen(b"[\0" as *const u8 as *const libc::c_char),
                                );
                                print_uint64_hex(rdm, dis);
                                print_str(
                                    rdm,
                                    b"]\0" as *const u8 as *const libc::c_char,
                                    strlen(b"]\0" as *const u8 as *const libc::c_char),
                                );
                            }
                            current_block = 15919730065710799250;
                        }
                        517681984938920942 => {
                            backref = parse_integer_62(rdm);
                            if (*rdm).skipping_printing == 0 {
                                old_next = (*rdm).next;
                                (*rdm).next = backref;
                                demangle_path(rdm, in_value);
                                (*rdm).next = old_next;
                            }
                            current_block = 15919730065710799250;
                        }
                        _ => {}
                    }
                    match current_block {
                        15919730065710799250 => {}
                        3304099842109091603 => {}
                        _ => {
                            print_str(
                                rdm,
                                b"<\0" as *const u8 as *const libc::c_char,
                                strlen(b"<\0" as *const u8 as *const libc::c_char),
                            );
                            demangle_type(rdm);
                            if tag as libc::c_int != 'M' as i32 {
                                print_str(
                                    rdm,
                                    b" as \0" as *const u8 as *const libc::c_char,
                                    strlen(b" as \0" as *const u8 as *const libc::c_char),
                                );
                                demangle_path(rdm, 0 as libc::c_int);
                            }
                            print_str(
                                rdm,
                                b">\0" as *const u8 as *const libc::c_char,
                                strlen(b">\0" as *const u8 as *const libc::c_char),
                            );
                            current_block = 15919730065710799250;
                        }
                    }
                }
                73 => {
                    current_block = 17369132196136995081;
                    match current_block {
                        17369132196136995081 => {
                            demangle_path(rdm, in_value);
                            if in_value != 0 {
                                print_str(
                                    rdm,
                                    b"::\0" as *const u8 as *const libc::c_char,
                                    strlen(b"::\0" as *const u8 as *const libc::c_char),
                                );
                            }
                            print_str(
                                rdm,
                                b"<\0" as *const u8 as *const libc::c_char,
                                strlen(b"<\0" as *const u8 as *const libc::c_char),
                            );
                            i = 0 as libc::c_int as size_t;
                            while (*rdm).errored == 0
                                && eat(rdm, 'E' as i32 as libc::c_char) == 0
                            {
                                if i > 0 as libc::c_int as libc::c_ulong {
                                    print_str(
                                        rdm,
                                        b", \0" as *const u8 as *const libc::c_char,
                                        strlen(b", \0" as *const u8 as *const libc::c_char),
                                    );
                                }
                                demangle_generic_arg(rdm);
                                i = i.wrapping_add(1);
                                i;
                            }
                            print_str(
                                rdm,
                                b">\0" as *const u8 as *const libc::c_char,
                                strlen(b">\0" as *const u8 as *const libc::c_char),
                            );
                            current_block = 15919730065710799250;
                        }
                        195292842195342531 => {
                            parse_disambiguator(rdm);
                            was_skipping_printing = (*rdm).skipping_printing;
                            (*rdm).skipping_printing = 1 as libc::c_int;
                            demangle_path(rdm, in_value);
                            (*rdm).skipping_printing = was_skipping_printing;
                            current_block = 405477181314099796;
                        }
                        11045451106458149833 => {
                            ns = next(rdm);
                            if _sch_istable[(ns as libc::c_int & 0xff as libc::c_int)
                                as usize] as libc::c_int
                                & _sch_islower as libc::c_int as libc::c_ushort
                                    as libc::c_int == 0
                                && _sch_istable[(ns as libc::c_int & 0xff as libc::c_int)
                                    as usize] as libc::c_int
                                    & _sch_isupper as libc::c_int as libc::c_ushort
                                        as libc::c_int == 0
                            {
                                current_block = 3304099842109091603;
                            } else {
                                demangle_path(rdm, in_value);
                                dis = parse_disambiguator(rdm);
                                name = parse_ident(rdm);
                                if _sch_istable[(ns as libc::c_int & 0xff as libc::c_int)
                                    as usize] as libc::c_int
                                    & _sch_isupper as libc::c_int as libc::c_ushort
                                        as libc::c_int != 0
                                {
                                    print_str(
                                        rdm,
                                        b"::{\0" as *const u8 as *const libc::c_char,
                                        strlen(b"::{\0" as *const u8 as *const libc::c_char),
                                    );
                                    match ns as libc::c_int {
                                        67 => {
                                            print_str(
                                                rdm,
                                                b"closure\0" as *const u8 as *const libc::c_char,
                                                strlen(b"closure\0" as *const u8 as *const libc::c_char),
                                            );
                                        }
                                        83 => {
                                            print_str(
                                                rdm,
                                                b"shim\0" as *const u8 as *const libc::c_char,
                                                strlen(b"shim\0" as *const u8 as *const libc::c_char),
                                            );
                                        }
                                        _ => {
                                            print_str(rdm, &mut ns, 1 as libc::c_int as size_t);
                                        }
                                    }
                                    if !(name.ascii).is_null() || !(name.punycode).is_null() {
                                        print_str(
                                            rdm,
                                            b":\0" as *const u8 as *const libc::c_char,
                                            strlen(b":\0" as *const u8 as *const libc::c_char),
                                        );
                                        print_ident(rdm, name);
                                    }
                                    print_str(
                                        rdm,
                                        b"#\0" as *const u8 as *const libc::c_char,
                                        strlen(b"#\0" as *const u8 as *const libc::c_char),
                                    );
                                    print_uint64(rdm, dis);
                                    print_str(
                                        rdm,
                                        b"}\0" as *const u8 as *const libc::c_char,
                                        strlen(b"}\0" as *const u8 as *const libc::c_char),
                                    );
                                } else if !(name.ascii).is_null()
                                    || !(name.punycode).is_null()
                                {
                                    print_str(
                                        rdm,
                                        b"::\0" as *const u8 as *const libc::c_char,
                                        strlen(b"::\0" as *const u8 as *const libc::c_char),
                                    );
                                    print_ident(rdm, name);
                                }
                                current_block = 15919730065710799250;
                            }
                        }
                        16681027592547421651 => {
                            dis = parse_disambiguator(rdm);
                            name = parse_ident(rdm);
                            print_ident(rdm, name);
                            if (*rdm).verbose != 0 {
                                print_str(
                                    rdm,
                                    b"[\0" as *const u8 as *const libc::c_char,
                                    strlen(b"[\0" as *const u8 as *const libc::c_char),
                                );
                                print_uint64_hex(rdm, dis);
                                print_str(
                                    rdm,
                                    b"]\0" as *const u8 as *const libc::c_char,
                                    strlen(b"]\0" as *const u8 as *const libc::c_char),
                                );
                            }
                            current_block = 15919730065710799250;
                        }
                        517681984938920942 => {
                            backref = parse_integer_62(rdm);
                            if (*rdm).skipping_printing == 0 {
                                old_next = (*rdm).next;
                                (*rdm).next = backref;
                                demangle_path(rdm, in_value);
                                (*rdm).next = old_next;
                            }
                            current_block = 15919730065710799250;
                        }
                        _ => {}
                    }
                    match current_block {
                        15919730065710799250 => {}
                        3304099842109091603 => {}
                        _ => {
                            print_str(
                                rdm,
                                b"<\0" as *const u8 as *const libc::c_char,
                                strlen(b"<\0" as *const u8 as *const libc::c_char),
                            );
                            demangle_type(rdm);
                            if tag as libc::c_int != 'M' as i32 {
                                print_str(
                                    rdm,
                                    b" as \0" as *const u8 as *const libc::c_char,
                                    strlen(b" as \0" as *const u8 as *const libc::c_char),
                                );
                                demangle_path(rdm, 0 as libc::c_int);
                            }
                            print_str(
                                rdm,
                                b">\0" as *const u8 as *const libc::c_char,
                                strlen(b">\0" as *const u8 as *const libc::c_char),
                            );
                            current_block = 15919730065710799250;
                        }
                    }
                }
                66 => {
                    current_block = 517681984938920942;
                    match current_block {
                        17369132196136995081 => {
                            demangle_path(rdm, in_value);
                            if in_value != 0 {
                                print_str(
                                    rdm,
                                    b"::\0" as *const u8 as *const libc::c_char,
                                    strlen(b"::\0" as *const u8 as *const libc::c_char),
                                );
                            }
                            print_str(
                                rdm,
                                b"<\0" as *const u8 as *const libc::c_char,
                                strlen(b"<\0" as *const u8 as *const libc::c_char),
                            );
                            i = 0 as libc::c_int as size_t;
                            while (*rdm).errored == 0
                                && eat(rdm, 'E' as i32 as libc::c_char) == 0
                            {
                                if i > 0 as libc::c_int as libc::c_ulong {
                                    print_str(
                                        rdm,
                                        b", \0" as *const u8 as *const libc::c_char,
                                        strlen(b", \0" as *const u8 as *const libc::c_char),
                                    );
                                }
                                demangle_generic_arg(rdm);
                                i = i.wrapping_add(1);
                                i;
                            }
                            print_str(
                                rdm,
                                b">\0" as *const u8 as *const libc::c_char,
                                strlen(b">\0" as *const u8 as *const libc::c_char),
                            );
                            current_block = 15919730065710799250;
                        }
                        195292842195342531 => {
                            parse_disambiguator(rdm);
                            was_skipping_printing = (*rdm).skipping_printing;
                            (*rdm).skipping_printing = 1 as libc::c_int;
                            demangle_path(rdm, in_value);
                            (*rdm).skipping_printing = was_skipping_printing;
                            current_block = 405477181314099796;
                        }
                        11045451106458149833 => {
                            ns = next(rdm);
                            if _sch_istable[(ns as libc::c_int & 0xff as libc::c_int)
                                as usize] as libc::c_int
                                & _sch_islower as libc::c_int as libc::c_ushort
                                    as libc::c_int == 0
                                && _sch_istable[(ns as libc::c_int & 0xff as libc::c_int)
                                    as usize] as libc::c_int
                                    & _sch_isupper as libc::c_int as libc::c_ushort
                                        as libc::c_int == 0
                            {
                                current_block = 3304099842109091603;
                            } else {
                                demangle_path(rdm, in_value);
                                dis = parse_disambiguator(rdm);
                                name = parse_ident(rdm);
                                if _sch_istable[(ns as libc::c_int & 0xff as libc::c_int)
                                    as usize] as libc::c_int
                                    & _sch_isupper as libc::c_int as libc::c_ushort
                                        as libc::c_int != 0
                                {
                                    print_str(
                                        rdm,
                                        b"::{\0" as *const u8 as *const libc::c_char,
                                        strlen(b"::{\0" as *const u8 as *const libc::c_char),
                                    );
                                    match ns as libc::c_int {
                                        67 => {
                                            print_str(
                                                rdm,
                                                b"closure\0" as *const u8 as *const libc::c_char,
                                                strlen(b"closure\0" as *const u8 as *const libc::c_char),
                                            );
                                        }
                                        83 => {
                                            print_str(
                                                rdm,
                                                b"shim\0" as *const u8 as *const libc::c_char,
                                                strlen(b"shim\0" as *const u8 as *const libc::c_char),
                                            );
                                        }
                                        _ => {
                                            print_str(rdm, &mut ns, 1 as libc::c_int as size_t);
                                        }
                                    }
                                    if !(name.ascii).is_null() || !(name.punycode).is_null() {
                                        print_str(
                                            rdm,
                                            b":\0" as *const u8 as *const libc::c_char,
                                            strlen(b":\0" as *const u8 as *const libc::c_char),
                                        );
                                        print_ident(rdm, name);
                                    }
                                    print_str(
                                        rdm,
                                        b"#\0" as *const u8 as *const libc::c_char,
                                        strlen(b"#\0" as *const u8 as *const libc::c_char),
                                    );
                                    print_uint64(rdm, dis);
                                    print_str(
                                        rdm,
                                        b"}\0" as *const u8 as *const libc::c_char,
                                        strlen(b"}\0" as *const u8 as *const libc::c_char),
                                    );
                                } else if !(name.ascii).is_null()
                                    || !(name.punycode).is_null()
                                {
                                    print_str(
                                        rdm,
                                        b"::\0" as *const u8 as *const libc::c_char,
                                        strlen(b"::\0" as *const u8 as *const libc::c_char),
                                    );
                                    print_ident(rdm, name);
                                }
                                current_block = 15919730065710799250;
                            }
                        }
                        16681027592547421651 => {
                            dis = parse_disambiguator(rdm);
                            name = parse_ident(rdm);
                            print_ident(rdm, name);
                            if (*rdm).verbose != 0 {
                                print_str(
                                    rdm,
                                    b"[\0" as *const u8 as *const libc::c_char,
                                    strlen(b"[\0" as *const u8 as *const libc::c_char),
                                );
                                print_uint64_hex(rdm, dis);
                                print_str(
                                    rdm,
                                    b"]\0" as *const u8 as *const libc::c_char,
                                    strlen(b"]\0" as *const u8 as *const libc::c_char),
                                );
                            }
                            current_block = 15919730065710799250;
                        }
                        517681984938920942 => {
                            backref = parse_integer_62(rdm);
                            if (*rdm).skipping_printing == 0 {
                                old_next = (*rdm).next;
                                (*rdm).next = backref;
                                demangle_path(rdm, in_value);
                                (*rdm).next = old_next;
                            }
                            current_block = 15919730065710799250;
                        }
                        _ => {}
                    }
                    match current_block {
                        15919730065710799250 => {}
                        3304099842109091603 => {}
                        _ => {
                            print_str(
                                rdm,
                                b"<\0" as *const u8 as *const libc::c_char,
                                strlen(b"<\0" as *const u8 as *const libc::c_char),
                            );
                            demangle_type(rdm);
                            if tag as libc::c_int != 'M' as i32 {
                                print_str(
                                    rdm,
                                    b" as \0" as *const u8 as *const libc::c_char,
                                    strlen(b" as \0" as *const u8 as *const libc::c_char),
                                );
                                demangle_path(rdm, 0 as libc::c_int);
                            }
                            print_str(
                                rdm,
                                b">\0" as *const u8 as *const libc::c_char,
                                strlen(b">\0" as *const u8 as *const libc::c_char),
                            );
                            current_block = 15919730065710799250;
                        }
                    }
                }
                _ => {
                    current_block = 3304099842109091603;
                }
            }
        }
        _ => {}
    }
    match current_block {
        3304099842109091603 => {
            (*rdm).errored = 1 as libc::c_int;
        }
        _ => {}
    }
    if (*rdm).recursion != -(1 as libc::c_int) as uint {
        (*rdm).recursion = ((*rdm).recursion).wrapping_sub(1);
        (*rdm).recursion;
    }
}
unsafe extern "C" fn demangle_generic_arg(mut rdm: *mut rust_demangler) {
    let mut lt: uint64_t = 0;
    if eat(rdm, 'L' as i32 as libc::c_char) != 0 {
        lt = parse_integer_62(rdm);
        print_lifetime_from_index(rdm, lt);
    } else if eat(rdm, 'K' as i32 as libc::c_char) != 0 {
        demangle_const(rdm);
    } else {
        demangle_type(rdm);
    };
}
unsafe extern "C" fn basic_type(mut tag: libc::c_char) -> *const libc::c_char {
    match tag as libc::c_int {
        98 => return b"bool\0" as *const u8 as *const libc::c_char,
        99 => return b"char\0" as *const u8 as *const libc::c_char,
        101 => return b"str\0" as *const u8 as *const libc::c_char,
        117 => return b"()\0" as *const u8 as *const libc::c_char,
        97 => return b"i8\0" as *const u8 as *const libc::c_char,
        115 => return b"i16\0" as *const u8 as *const libc::c_char,
        108 => return b"i32\0" as *const u8 as *const libc::c_char,
        120 => return b"i64\0" as *const u8 as *const libc::c_char,
        110 => return b"i128\0" as *const u8 as *const libc::c_char,
        105 => return b"isize\0" as *const u8 as *const libc::c_char,
        104 => return b"u8\0" as *const u8 as *const libc::c_char,
        116 => return b"u16\0" as *const u8 as *const libc::c_char,
        109 => return b"u32\0" as *const u8 as *const libc::c_char,
        121 => return b"u64\0" as *const u8 as *const libc::c_char,
        111 => return b"u128\0" as *const u8 as *const libc::c_char,
        106 => return b"usize\0" as *const u8 as *const libc::c_char,
        102 => return b"f32\0" as *const u8 as *const libc::c_char,
        100 => return b"f64\0" as *const u8 as *const libc::c_char,
        122 => return b"!\0" as *const u8 as *const libc::c_char,
        112 => return b"_\0" as *const u8 as *const libc::c_char,
        118 => return b"...\0" as *const u8 as *const libc::c_char,
        _ => return 0 as *const libc::c_char,
    };
}
unsafe extern "C" fn demangle_type(mut rdm: *mut rust_demangler) {
    let mut tag: libc::c_char = 0;
    let mut i: size_t = 0;
    let mut old_next: size_t = 0;
    let mut backref: size_t = 0;
    let mut lt: uint64_t = 0;
    let mut old_bound_lifetime_depth: uint64_t = 0;
    let mut basic: *const libc::c_char = 0 as *const libc::c_char;
    let mut abi: rust_mangled_ident = rust_mangled_ident {
        ascii: 0 as *const libc::c_char,
        ascii_len: 0,
        punycode: 0 as *const libc::c_char,
        punycode_len: 0,
    };
    if (*rdm).errored != 0 {
        return;
    }
    tag = next(rdm);
    basic = basic_type(tag);
    if !basic.is_null() {
        print_str(rdm, basic, strlen(basic));
        return;
    }
    let mut current_block_104: u64;
    match tag as libc::c_int {
        82 | 81 => {
            print_str(
                rdm,
                b"&\0" as *const u8 as *const libc::c_char,
                strlen(b"&\0" as *const u8 as *const libc::c_char),
            );
            if eat(rdm, 'L' as i32 as libc::c_char) != 0 {
                lt = parse_integer_62(rdm);
                if lt != 0 {
                    print_lifetime_from_index(rdm, lt);
                    print_str(
                        rdm,
                        b" \0" as *const u8 as *const libc::c_char,
                        strlen(b" \0" as *const u8 as *const libc::c_char),
                    );
                }
            }
            if tag as libc::c_int != 'R' as i32 {
                print_str(
                    rdm,
                    b"mut \0" as *const u8 as *const libc::c_char,
                    strlen(b"mut \0" as *const u8 as *const libc::c_char),
                );
            }
            demangle_type(rdm);
        }
        80 | 79 => {
            print_str(
                rdm,
                b"*\0" as *const u8 as *const libc::c_char,
                strlen(b"*\0" as *const u8 as *const libc::c_char),
            );
            if tag as libc::c_int != 'P' as i32 {
                print_str(
                    rdm,
                    b"mut \0" as *const u8 as *const libc::c_char,
                    strlen(b"mut \0" as *const u8 as *const libc::c_char),
                );
            } else {
                print_str(
                    rdm,
                    b"const \0" as *const u8 as *const libc::c_char,
                    strlen(b"const \0" as *const u8 as *const libc::c_char),
                );
            }
            demangle_type(rdm);
        }
        65 | 83 => {
            print_str(
                rdm,
                b"[\0" as *const u8 as *const libc::c_char,
                strlen(b"[\0" as *const u8 as *const libc::c_char),
            );
            demangle_type(rdm);
            if tag as libc::c_int == 'A' as i32 {
                print_str(
                    rdm,
                    b"; \0" as *const u8 as *const libc::c_char,
                    strlen(b"; \0" as *const u8 as *const libc::c_char),
                );
                demangle_const(rdm);
            }
            print_str(
                rdm,
                b"]\0" as *const u8 as *const libc::c_char,
                strlen(b"]\0" as *const u8 as *const libc::c_char),
            );
        }
        84 => {
            print_str(
                rdm,
                b"(\0" as *const u8 as *const libc::c_char,
                strlen(b"(\0" as *const u8 as *const libc::c_char),
            );
            i = 0 as libc::c_int as size_t;
            while (*rdm).errored == 0 && eat(rdm, 'E' as i32 as libc::c_char) == 0 {
                if i > 0 as libc::c_int as libc::c_ulong {
                    print_str(
                        rdm,
                        b", \0" as *const u8 as *const libc::c_char,
                        strlen(b", \0" as *const u8 as *const libc::c_char),
                    );
                }
                demangle_type(rdm);
                i = i.wrapping_add(1);
                i;
            }
            if i == 1 as libc::c_int as libc::c_ulong {
                print_str(
                    rdm,
                    b",\0" as *const u8 as *const libc::c_char,
                    strlen(b",\0" as *const u8 as *const libc::c_char),
                );
            }
            print_str(
                rdm,
                b")\0" as *const u8 as *const libc::c_char,
                strlen(b")\0" as *const u8 as *const libc::c_char),
            );
        }
        70 => {
            old_bound_lifetime_depth = (*rdm).bound_lifetime_depth;
            demangle_binder(rdm);
            if eat(rdm, 'U' as i32 as libc::c_char) != 0 {
                print_str(
                    rdm,
                    b"unsafe \0" as *const u8 as *const libc::c_char,
                    strlen(b"unsafe \0" as *const u8 as *const libc::c_char),
                );
            }
            if eat(rdm, 'K' as i32 as libc::c_char) != 0 {
                if eat(rdm, 'C' as i32 as libc::c_char) != 0 {
                    abi.ascii = b"C\0" as *const u8 as *const libc::c_char;
                    abi.ascii_len = 1 as libc::c_int as size_t;
                    current_block_104 = 1345366029464561491;
                } else {
                    abi = parse_ident(rdm);
                    if (abi.ascii).is_null() || !(abi.punycode).is_null() {
                        (*rdm).errored = 1 as libc::c_int;
                        current_block_104 = 10977967421569260512;
                    } else {
                        current_block_104 = 1345366029464561491;
                    }
                }
                match current_block_104 {
                    10977967421569260512 => {}
                    _ => {
                        print_str(
                            rdm,
                            b"extern \"\0" as *const u8 as *const libc::c_char,
                            strlen(b"extern \"\0" as *const u8 as *const libc::c_char),
                        );
                        i = 0 as libc::c_int as size_t;
                        while i < abi.ascii_len {
                            if *(abi.ascii).offset(i as isize) as libc::c_int
                                == '_' as i32
                            {
                                print_str(rdm, abi.ascii, i);
                                print_str(
                                    rdm,
                                    b"-\0" as *const u8 as *const libc::c_char,
                                    strlen(b"-\0" as *const u8 as *const libc::c_char),
                                );
                                abi
                                    .ascii = (abi.ascii)
                                    .offset(
                                        i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                                    );
                                abi
                                    .ascii_len = (abi.ascii_len as libc::c_ulong)
                                    .wrapping_sub(
                                        i.wrapping_add(1 as libc::c_int as libc::c_ulong),
                                    ) as size_t as size_t;
                                i = 0 as libc::c_int as size_t;
                            }
                            i = i.wrapping_add(1);
                            i;
                        }
                        print_str(rdm, abi.ascii, abi.ascii_len);
                        print_str(
                            rdm,
                            b"\" \0" as *const u8 as *const libc::c_char,
                            strlen(b"\" \0" as *const u8 as *const libc::c_char),
                        );
                        current_block_104 = 13325891313334703151;
                    }
                }
            } else {
                current_block_104 = 13325891313334703151;
            }
            match current_block_104 {
                13325891313334703151 => {
                    print_str(
                        rdm,
                        b"fn(\0" as *const u8 as *const libc::c_char,
                        strlen(b"fn(\0" as *const u8 as *const libc::c_char),
                    );
                    i = 0 as libc::c_int as size_t;
                    while (*rdm).errored == 0
                        && eat(rdm, 'E' as i32 as libc::c_char) == 0
                    {
                        if i > 0 as libc::c_int as libc::c_ulong {
                            print_str(
                                rdm,
                                b", \0" as *const u8 as *const libc::c_char,
                                strlen(b", \0" as *const u8 as *const libc::c_char),
                            );
                        }
                        demangle_type(rdm);
                        i = i.wrapping_add(1);
                        i;
                    }
                    print_str(
                        rdm,
                        b")\0" as *const u8 as *const libc::c_char,
                        strlen(b")\0" as *const u8 as *const libc::c_char),
                    );
                    if !(eat(rdm, 'u' as i32 as libc::c_char) != 0) {
                        print_str(
                            rdm,
                            b" -> \0" as *const u8 as *const libc::c_char,
                            strlen(b" -> \0" as *const u8 as *const libc::c_char),
                        );
                        demangle_type(rdm);
                    }
                }
                _ => {}
            }
            (*rdm).bound_lifetime_depth = old_bound_lifetime_depth;
        }
        68 => {
            print_str(
                rdm,
                b"dyn \0" as *const u8 as *const libc::c_char,
                strlen(b"dyn \0" as *const u8 as *const libc::c_char),
            );
            old_bound_lifetime_depth = (*rdm).bound_lifetime_depth;
            demangle_binder(rdm);
            i = 0 as libc::c_int as size_t;
            while (*rdm).errored == 0 && eat(rdm, 'E' as i32 as libc::c_char) == 0 {
                if i > 0 as libc::c_int as libc::c_ulong {
                    print_str(
                        rdm,
                        b" + \0" as *const u8 as *const libc::c_char,
                        strlen(b" + \0" as *const u8 as *const libc::c_char),
                    );
                }
                demangle_dyn_trait(rdm);
                i = i.wrapping_add(1);
                i;
            }
            (*rdm).bound_lifetime_depth = old_bound_lifetime_depth;
            if eat(rdm, 'L' as i32 as libc::c_char) == 0 {
                (*rdm).errored = 1 as libc::c_int;
                return;
            }
            lt = parse_integer_62(rdm);
            if lt != 0 {
                print_str(
                    rdm,
                    b" + \0" as *const u8 as *const libc::c_char,
                    strlen(b" + \0" as *const u8 as *const libc::c_char),
                );
                print_lifetime_from_index(rdm, lt);
            }
        }
        66 => {
            backref = parse_integer_62(rdm);
            if (*rdm).skipping_printing == 0 {
                old_next = (*rdm).next;
                (*rdm).next = backref;
                demangle_type(rdm);
                (*rdm).next = old_next;
            }
        }
        _ => {
            (*rdm).next = ((*rdm).next).wrapping_sub(1);
            (*rdm).next;
            demangle_path(rdm, 0 as libc::c_int);
        }
    };
}
unsafe extern "C" fn demangle_path_maybe_open_generics(
    mut rdm: *mut rust_demangler,
) -> libc::c_int {
    let mut open: libc::c_int = 0;
    let mut i: size_t = 0;
    let mut old_next: size_t = 0;
    let mut backref: size_t = 0;
    open = 0 as libc::c_int;
    if (*rdm).errored != 0 {
        return open;
    }
    if eat(rdm, 'B' as i32 as libc::c_char) != 0 {
        backref = parse_integer_62(rdm);
        if (*rdm).skipping_printing == 0 {
            old_next = (*rdm).next;
            (*rdm).next = backref;
            open = demangle_path_maybe_open_generics(rdm);
            (*rdm).next = old_next;
        }
    } else if eat(rdm, 'I' as i32 as libc::c_char) != 0 {
        demangle_path(rdm, 0 as libc::c_int);
        print_str(
            rdm,
            b"<\0" as *const u8 as *const libc::c_char,
            strlen(b"<\0" as *const u8 as *const libc::c_char),
        );
        open = 1 as libc::c_int;
        i = 0 as libc::c_int as size_t;
        while (*rdm).errored == 0 && eat(rdm, 'E' as i32 as libc::c_char) == 0 {
            if i > 0 as libc::c_int as libc::c_ulong {
                print_str(
                    rdm,
                    b", \0" as *const u8 as *const libc::c_char,
                    strlen(b", \0" as *const u8 as *const libc::c_char),
                );
            }
            demangle_generic_arg(rdm);
            i = i.wrapping_add(1);
            i;
        }
    } else {
        demangle_path(rdm, 0 as libc::c_int);
    }
    return open;
}
unsafe extern "C" fn demangle_dyn_trait(mut rdm: *mut rust_demangler) {
    let mut open: libc::c_int = 0;
    let mut name: rust_mangled_ident = rust_mangled_ident {
        ascii: 0 as *const libc::c_char,
        ascii_len: 0,
        punycode: 0 as *const libc::c_char,
        punycode_len: 0,
    };
    if (*rdm).errored != 0 {
        return;
    }
    open = demangle_path_maybe_open_generics(rdm);
    while eat(rdm, 'p' as i32 as libc::c_char) != 0 {
        if open == 0 {
            print_str(
                rdm,
                b"<\0" as *const u8 as *const libc::c_char,
                strlen(b"<\0" as *const u8 as *const libc::c_char),
            );
        } else {
            print_str(
                rdm,
                b", \0" as *const u8 as *const libc::c_char,
                strlen(b", \0" as *const u8 as *const libc::c_char),
            );
        }
        open = 1 as libc::c_int;
        name = parse_ident(rdm);
        print_ident(rdm, name);
        print_str(
            rdm,
            b" = \0" as *const u8 as *const libc::c_char,
            strlen(b" = \0" as *const u8 as *const libc::c_char),
        );
        demangle_type(rdm);
    }
    if open != 0 {
        print_str(
            rdm,
            b">\0" as *const u8 as *const libc::c_char,
            strlen(b">\0" as *const u8 as *const libc::c_char),
        );
    }
}
unsafe extern "C" fn demangle_const(mut rdm: *mut rust_demangler) {
    let mut ty_tag: libc::c_char = 0;
    let mut old_next: size_t = 0;
    let mut backref: size_t = 0;
    if (*rdm).errored != 0 {
        return;
    }
    if eat(rdm, 'B' as i32 as libc::c_char) != 0 {
        backref = parse_integer_62(rdm);
        if (*rdm).skipping_printing == 0 {
            old_next = (*rdm).next;
            (*rdm).next = backref;
            demangle_const(rdm);
            (*rdm).next = old_next;
        }
        return;
    }
    ty_tag = next(rdm);
    match ty_tag as libc::c_int {
        112 => {
            print_str(
                rdm,
                b"_\0" as *const u8 as *const libc::c_char,
                strlen(b"_\0" as *const u8 as *const libc::c_char),
            );
            return;
        }
        104 | 116 | 109 | 121 | 111 | 106 => {
            demangle_const_uint(rdm);
        }
        97 | 115 | 108 | 120 | 110 | 105 => {
            demangle_const_int(rdm);
        }
        98 => {
            demangle_const_bool(rdm);
        }
        99 => {
            demangle_const_char(rdm);
        }
        _ => {
            (*rdm).errored = 1 as libc::c_int;
            return;
        }
    }
    if (*rdm).errored != 0 {
        return;
    }
    if (*rdm).verbose != 0 {
        print_str(
            rdm,
            b": \0" as *const u8 as *const libc::c_char,
            strlen(b": \0" as *const u8 as *const libc::c_char),
        );
        print_str(rdm, basic_type(ty_tag), strlen(basic_type(ty_tag)));
    }
}
unsafe extern "C" fn demangle_const_uint(mut rdm: *mut rust_demangler) {
    let mut hex_len: size_t = 0;
    let mut value: uint64_t = 0;
    if (*rdm).errored != 0 {
        return;
    }
    hex_len = parse_hex_nibbles(rdm, &mut value);
    if hex_len > 16 as libc::c_int as libc::c_ulong {
        print_str(
            rdm,
            b"0x\0" as *const u8 as *const libc::c_char,
            strlen(b"0x\0" as *const u8 as *const libc::c_char),
        );
        print_str(
            rdm,
            ((*rdm).sym).offset(((*rdm).next).wrapping_sub(hex_len) as isize),
            hex_len,
        );
    } else if hex_len > 0 as libc::c_int as libc::c_ulong {
        print_uint64(rdm, value);
    } else {
        (*rdm).errored = 1 as libc::c_int;
    };
}
unsafe extern "C" fn demangle_const_int(mut rdm: *mut rust_demangler) {
    if eat(rdm, 'n' as i32 as libc::c_char) != 0 {
        print_str(
            rdm,
            b"-\0" as *const u8 as *const libc::c_char,
            strlen(b"-\0" as *const u8 as *const libc::c_char),
        );
    }
    demangle_const_uint(rdm);
}
unsafe extern "C" fn demangle_const_bool(mut rdm: *mut rust_demangler) {
    let mut value: uint64_t = 0;
    if parse_hex_nibbles(rdm, &mut value) != 1 as libc::c_int as libc::c_ulong {
        (*rdm).errored = 1 as libc::c_int;
        return;
    }
    if value == 0 as libc::c_int as libc::c_ulong {
        print_str(
            rdm,
            b"false\0" as *const u8 as *const libc::c_char,
            strlen(b"false\0" as *const u8 as *const libc::c_char),
        );
    } else if value == 1 as libc::c_int as libc::c_ulong {
        print_str(
            rdm,
            b"true\0" as *const u8 as *const libc::c_char,
            strlen(b"true\0" as *const u8 as *const libc::c_char),
        );
    } else {
        (*rdm).errored = 1 as libc::c_int;
    };
}
unsafe extern "C" fn demangle_const_char(mut rdm: *mut rust_demangler) {
    let mut hex_len: size_t = 0;
    let mut value: uint64_t = 0;
    hex_len = parse_hex_nibbles(rdm, &mut value);
    if hex_len == 0 as libc::c_int as libc::c_ulong
        || hex_len > 8 as libc::c_int as libc::c_ulong
    {
        (*rdm).errored = 1 as libc::c_int;
        return;
    }
    print_str(
        rdm,
        b"'\0" as *const u8 as *const libc::c_char,
        strlen(b"'\0" as *const u8 as *const libc::c_char),
    );
    if value == '\t' as i32 as libc::c_ulong {
        print_str(
            rdm,
            b"\\t\0" as *const u8 as *const libc::c_char,
            strlen(b"\\t\0" as *const u8 as *const libc::c_char),
        );
    } else if value == '\r' as i32 as libc::c_ulong {
        print_str(
            rdm,
            b"\\r\0" as *const u8 as *const libc::c_char,
            strlen(b"\\r\0" as *const u8 as *const libc::c_char),
        );
    } else if value == '\n' as i32 as libc::c_ulong {
        print_str(
            rdm,
            b"\\n\0" as *const u8 as *const libc::c_char,
            strlen(b"\\n\0" as *const u8 as *const libc::c_char),
        );
    } else if value > ' ' as i32 as libc::c_ulong && value < '~' as i32 as libc::c_ulong
    {
        let mut c: libc::c_char = value as libc::c_char;
        print_str(rdm, &mut c, 1 as libc::c_int as size_t);
    } else {
        print_str(
            rdm,
            b"\\u{\0" as *const u8 as *const libc::c_char,
            strlen(b"\\u{\0" as *const u8 as *const libc::c_char),
        );
        print_uint64_hex(rdm, value);
        print_str(
            rdm,
            b"}\0" as *const u8 as *const libc::c_char,
            strlen(b"}\0" as *const u8 as *const libc::c_char),
        );
    }
    print_str(
        rdm,
        b"'\0" as *const u8 as *const libc::c_char,
        strlen(b"'\0" as *const u8 as *const libc::c_char),
    );
}
unsafe extern "C" fn is_legacy_prefixed_hash(
    mut ident: rust_mangled_ident,
) -> libc::c_int {
    let mut seen: uint16_t = 0;
    let mut nibble: libc::c_int = 0;
    let mut i: size_t = 0;
    let mut count: size_t = 0;
    if ident.ascii_len != 17 as libc::c_int as libc::c_ulong
        || *(ident.ascii).offset(0 as libc::c_int as isize) as libc::c_int != 'h' as i32
    {
        return 0 as libc::c_int;
    }
    seen = 0 as libc::c_int as uint16_t;
    i = 0 as libc::c_int as size_t;
    while i < 16 as libc::c_int as libc::c_ulong {
        nibble = decode_lower_hex_nibble(
            *(ident.ascii)
                .offset((1 as libc::c_int as libc::c_ulong).wrapping_add(i) as isize),
        );
        if nibble < 0 as libc::c_int {
            return 0 as libc::c_int;
        }
        seen = (seen as libc::c_int
            | (1 as libc::c_int as uint16_t as libc::c_int) << nibble) as uint16_t;
        i = i.wrapping_add(1);
        i;
    }
    count = 0 as libc::c_int as size_t;
    while seen != 0 {
        if seen as libc::c_int & 1 as libc::c_int != 0 {
            count = count.wrapping_add(1);
            count;
        }
        seen = (seen as libc::c_int >> 1 as libc::c_int) as uint16_t;
    }
    return (count >= 5 as libc::c_int as libc::c_ulong) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rust_demangle_callback(
    mut mangled: *const libc::c_char,
    mut options: libc::c_int,
    mut callback: demangle_callbackref,
    mut opaque: *mut libc::c_void,
) -> libc::c_int {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut rdm: rust_demangler = rust_demangler {
        sym: 0 as *const libc::c_char,
        sym_len: 0,
        callback_opaque: 0 as *mut libc::c_void,
        callback: None,
        next: 0,
        errored: 0,
        skipping_printing: 0,
        verbose: 0,
        version: 0,
        recursion: 0,
        bound_lifetime_depth: 0,
    };
    let mut ident: rust_mangled_ident = rust_mangled_ident {
        ascii: 0 as *const libc::c_char,
        ascii_len: 0,
        punycode: 0 as *const libc::c_char,
        punycode_len: 0,
    };
    rdm.sym = mangled;
    rdm.sym_len = 0 as libc::c_int as size_t;
    rdm.callback_opaque = opaque;
    rdm.callback = callback;
    rdm.next = 0 as libc::c_int as size_t;
    rdm.errored = 0 as libc::c_int;
    rdm.skipping_printing = 0 as libc::c_int;
    rdm
        .verbose = (options & (1 as libc::c_int) << 3 as libc::c_int != 0 as libc::c_int)
        as libc::c_int;
    rdm.version = 0 as libc::c_int;
    rdm
        .recursion = if options & (1 as libc::c_int) << 18 as libc::c_int != 0 {
        -(1 as libc::c_int) as uint
    } else {
        0 as libc::c_int as libc::c_uint
    };
    rdm.bound_lifetime_depth = 0 as libc::c_int as uint64_t;
    if *(rdm.sym).offset(0 as libc::c_int as isize) as libc::c_int == '_' as i32
        && *(rdm.sym).offset(1 as libc::c_int as isize) as libc::c_int == 'R' as i32
    {
        rdm.sym = (rdm.sym).offset(2 as libc::c_int as isize);
    } else if *(rdm.sym).offset(0 as libc::c_int as isize) as libc::c_int == '_' as i32
        && *(rdm.sym).offset(1 as libc::c_int as isize) as libc::c_int == 'Z' as i32
        && *(rdm.sym).offset(2 as libc::c_int as isize) as libc::c_int == 'N' as i32
    {
        rdm.sym = (rdm.sym).offset(3 as libc::c_int as isize);
        rdm.version = -(1 as libc::c_int);
    } else {
        return 0 as libc::c_int
    }
    if rdm.version != -(1 as libc::c_int)
        && _sch_istable[(*(rdm.sym).offset(0 as libc::c_int as isize) as libc::c_int
            & 0xff as libc::c_int) as usize] as libc::c_int
            & _sch_isupper as libc::c_int as libc::c_ushort as libc::c_int == 0
    {
        return 0 as libc::c_int;
    }
    p = rdm.sym;
    while *p != 0 {
        rdm.sym_len = (rdm.sym_len).wrapping_add(1);
        rdm.sym_len;
        if !(*p as libc::c_int == '_' as i32
            || _sch_istable[(*p as libc::c_int & 0xff as libc::c_int) as usize]
                as libc::c_int
                & _sch_isalnum as libc::c_int as libc::c_ushort as libc::c_int != 0)
        {
            if !(rdm.version == -(1 as libc::c_int)
                && (*p as libc::c_int == '$' as i32 || *p as libc::c_int == '.' as i32
                    || *p as libc::c_int == ':' as i32))
            {
                return 0 as libc::c_int;
            }
        }
        p = p.offset(1);
        p;
    }
    if rdm.version == -(1 as libc::c_int) {
        if !(rdm.sym_len > 0 as libc::c_int as libc::c_ulong
            && *(rdm.sym)
                .offset(
                    (rdm.sym_len).wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        as isize,
                ) as libc::c_int == 'E' as i32)
        {
            return 0 as libc::c_int;
        }
        rdm.sym_len = (rdm.sym_len).wrapping_sub(1);
        rdm.sym_len;
        if !(rdm.sym_len > 19 as libc::c_int as libc::c_ulong
            && memcmp(
                &*(rdm.sym)
                    .offset(
                        (rdm.sym_len).wrapping_sub(19 as libc::c_int as libc::c_ulong)
                            as isize,
                    ) as *const libc::c_char as *const libc::c_void,
                b"17h\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                3 as libc::c_int as libc::c_ulong,
            ) == 0)
        {
            return 0 as libc::c_int;
        }
        loop {
            ident = parse_ident(&mut rdm);
            if rdm.errored != 0 || (ident.ascii).is_null() {
                return 0 as libc::c_int;
            }
            if !(rdm.next < rdm.sym_len) {
                break;
            }
        }
        if is_legacy_prefixed_hash(ident) == 0 {
            return 0 as libc::c_int;
        }
        rdm.next = 0 as libc::c_int as size_t;
        if rdm.verbose == 0 && rdm.sym_len > 19 as libc::c_int as libc::c_ulong {
            rdm
                .sym_len = (rdm.sym_len as libc::c_ulong)
                .wrapping_sub(19 as libc::c_int as libc::c_ulong) as size_t as size_t;
        }
        loop {
            if rdm.next > 0 as libc::c_int as libc::c_ulong {
                print_str(
                    &mut rdm,
                    b"::\0" as *const u8 as *const libc::c_char,
                    2 as libc::c_int as size_t,
                );
            }
            ident = parse_ident(&mut rdm);
            print_ident(&mut rdm, ident);
            if !(rdm.next < rdm.sym_len) {
                break;
            }
        }
    } else {
        demangle_path(&mut rdm, 1 as libc::c_int);
        if rdm.errored == 0 && rdm.next < rdm.sym_len {
            rdm.skipping_printing = 1 as libc::c_int;
            demangle_path(&mut rdm, 0 as libc::c_int);
        }
        rdm.errored |= (rdm.next != rdm.sym_len) as libc::c_int;
    }
    return (rdm.errored == 0) as libc::c_int;
}
unsafe extern "C" fn str_buf_reserve(mut buf: *mut str_buf, mut extra: size_t) {
    let mut available: size_t = 0;
    let mut min_new_cap: size_t = 0;
    let mut new_cap: size_t = 0;
    let mut new_ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    if (*buf).errored != 0 {
        return;
    }
    available = ((*buf).cap).wrapping_sub((*buf).len);
    if extra <= available {
        return;
    }
    min_new_cap = ((*buf).cap).wrapping_add(extra.wrapping_sub(available));
    if min_new_cap < (*buf).cap {
        (*buf).errored = 1 as libc::c_int;
        return;
    }
    new_cap = (*buf).cap;
    if new_cap == 0 as libc::c_int as libc::c_ulong {
        new_cap = 4 as libc::c_int as size_t;
    }
    while new_cap < min_new_cap {
        new_cap = (new_cap as libc::c_ulong)
            .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
        if new_cap < (*buf).cap {
            (*buf).errored = 1 as libc::c_int;
            return;
        }
    }
    new_ptr = realloc((*buf).ptr as *mut libc::c_void, new_cap) as *mut libc::c_char;
    if new_ptr.is_null() {
        free((*buf).ptr as *mut libc::c_void);
        (*buf).ptr = 0 as *mut libc::c_char;
        (*buf).len = 0 as libc::c_int as size_t;
        (*buf).cap = 0 as libc::c_int as size_t;
        (*buf).errored = 1 as libc::c_int;
    } else {
        (*buf).ptr = new_ptr;
        (*buf).cap = new_cap;
    };
}
unsafe extern "C" fn str_buf_append(
    mut buf: *mut str_buf,
    mut data: *const libc::c_char,
    mut len: size_t,
) {
    str_buf_reserve(buf, len);
    if (*buf).errored != 0 {
        return;
    }
    memcpy(
        ((*buf).ptr).offset((*buf).len as isize) as *mut libc::c_void,
        data as *const libc::c_void,
        len,
    );
    (*buf).len = ((*buf).len as libc::c_ulong).wrapping_add(len) as size_t as size_t;
}
unsafe extern "C" fn str_buf_demangle_callback(
    mut data: *const libc::c_char,
    mut len: size_t,
    mut opaque: *mut libc::c_void,
) {
    str_buf_append(opaque as *mut str_buf, data, len);
}
#[no_mangle]
pub unsafe extern "C" fn rust_demangle(
    mut mangled: *const libc::c_char,
    mut options: libc::c_int,
) -> *mut libc::c_char {
    let mut out: str_buf = str_buf {
        ptr: 0 as *mut libc::c_char,
        len: 0,
        cap: 0,
        errored: 0,
    };
    let mut success: libc::c_int = 0;
    out.ptr = 0 as *mut libc::c_char;
    out.len = 0 as libc::c_int as size_t;
    out.cap = 0 as libc::c_int as size_t;
    out.errored = 0 as libc::c_int;
    success = rust_demangle_callback(
        mangled,
        options,
        Some(
            str_buf_demangle_callback
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    size_t,
                    *mut libc::c_void,
                ) -> (),
        ),
        &mut out as *mut str_buf as *mut libc::c_void,
    );
    if success == 0 {
        free(out.ptr as *mut libc::c_void);
        return 0 as *mut libc::c_char;
    }
    str_buf_append(
        &mut out,
        b"\0\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int as size_t,
    );
    return out.ptr;
}
