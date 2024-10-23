use ::libc;
extern "C" {
    static _sch_istable: [libc::c_ushort; 256];
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn free(_: *mut libc::c_void);
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn rust_demangle(
        mangled: *const libc::c_char,
        options: libc::c_int,
    ) -> *mut libc::c_char;
    fn cplus_demangle_v3(
        mangled: *const libc::c_char,
        options: libc::c_int,
    ) -> *mut libc::c_char;
    fn java_demangle_v3(mangled: *const libc::c_char) -> *mut libc::c_char;
    fn dlang_demangle(
        mangled: *const libc::c_char,
        options: libc::c_int,
    ) -> *mut libc::c_char;
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
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
pub type demangling_styles = libc::c_int;
pub const rust_demangling: demangling_styles = 131072;
pub const dlang_demangling: demangling_styles = 65536;
pub const gnat_demangling: demangling_styles = 32768;
pub const java_demangling: demangling_styles = 4;
pub const gnu_v3_demangling: demangling_styles = 16384;
pub const auto_demangling: demangling_styles = 256;
pub const unknown_demangling: demangling_styles = 0;
pub const no_demangling: demangling_styles = -1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct demangler_engine {
    pub demangling_style_name: *const libc::c_char,
    pub demangling_style: demangling_styles,
    pub demangling_style_doc: *const libc::c_char,
}
#[no_mangle]
pub static mut current_demangling_style: demangling_styles = auto_demangling;
#[no_mangle]
pub static mut libiberty_demanglers: [demangler_engine; 8] = [
    {
        let mut init = demangler_engine {
            demangling_style_name: b"none\0" as *const u8 as *const libc::c_char,
            demangling_style: no_demangling,
            demangling_style_doc: b"Demangling disabled\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = demangler_engine {
            demangling_style_name: b"auto\0" as *const u8 as *const libc::c_char,
            demangling_style: auto_demangling,
            demangling_style_doc: b"Automatic selection based on executable\0"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = demangler_engine {
            demangling_style_name: b"gnu-v3\0" as *const u8 as *const libc::c_char,
            demangling_style: gnu_v3_demangling,
            demangling_style_doc: b"GNU (g++) V3 (Itanium C++ ABI) style demangling\0"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = demangler_engine {
            demangling_style_name: b"java\0" as *const u8 as *const libc::c_char,
            demangling_style: java_demangling,
            demangling_style_doc: b"Java style demangling\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = demangler_engine {
            demangling_style_name: b"gnat\0" as *const u8 as *const libc::c_char,
            demangling_style: gnat_demangling,
            demangling_style_doc: b"GNAT style demangling\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = demangler_engine {
            demangling_style_name: b"dlang\0" as *const u8 as *const libc::c_char,
            demangling_style: dlang_demangling,
            demangling_style_doc: b"DLANG style demangling\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = demangler_engine {
            demangling_style_name: b"rust\0" as *const u8 as *const libc::c_char,
            demangling_style: rust_demangling,
            demangling_style_doc: b"Rust style demangling\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = demangler_engine {
            demangling_style_name: 0 as *const libc::c_char,
            demangling_style: unknown_demangling,
            demangling_style_doc: 0 as *const libc::c_char,
        };
        init
    },
];
#[no_mangle]
pub unsafe extern "C" fn cplus_demangle_set_style(
    mut style: demangling_styles,
) -> demangling_styles {
    let mut demangler: *const demangler_engine = libiberty_demanglers.as_ptr();
    while (*demangler).demangling_style as libc::c_int
        != unknown_demangling as libc::c_int
    {
        if style as libc::c_int == (*demangler).demangling_style as libc::c_int {
            current_demangling_style = style;
            return current_demangling_style;
        }
        demangler = demangler.offset(1);
        demangler;
    }
    return unknown_demangling;
}
#[no_mangle]
pub unsafe extern "C" fn cplus_demangle_name_to_style(
    mut name: *const libc::c_char,
) -> demangling_styles {
    let mut demangler: *const demangler_engine = libiberty_demanglers.as_ptr();
    while (*demangler).demangling_style as libc::c_int
        != unknown_demangling as libc::c_int
    {
        if strcmp(name, (*demangler).demangling_style_name) == 0 as libc::c_int {
            return (*demangler).demangling_style;
        }
        demangler = demangler.offset(1);
        demangler;
    }
    return unknown_demangling;
}
#[no_mangle]
pub unsafe extern "C" fn cplus_demangle(
    mut mangled: *const libc::c_char,
    mut options: libc::c_int,
) -> *mut libc::c_char {
    let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
    if current_demangling_style as libc::c_int == no_demangling as libc::c_int {
        return xstrdup(mangled);
    }
    if options
        & ((1 as libc::c_int) << 8 as libc::c_int
            | (1 as libc::c_int) << 14 as libc::c_int
            | (1 as libc::c_int) << 2 as libc::c_int
            | (1 as libc::c_int) << 15 as libc::c_int
            | (1 as libc::c_int) << 16 as libc::c_int
            | (1 as libc::c_int) << 17 as libc::c_int) == 0 as libc::c_int
    {
        options
            |= current_demangling_style as libc::c_int
                & ((1 as libc::c_int) << 8 as libc::c_int
                    | (1 as libc::c_int) << 14 as libc::c_int
                    | (1 as libc::c_int) << 2 as libc::c_int
                    | (1 as libc::c_int) << 15 as libc::c_int
                    | (1 as libc::c_int) << 16 as libc::c_int
                    | (1 as libc::c_int) << 17 as libc::c_int);
    }
    if options & (1 as libc::c_int) << 17 as libc::c_int != 0
        || options & (1 as libc::c_int) << 8 as libc::c_int != 0
    {
        ret = rust_demangle(mangled, options);
        if !ret.is_null() || options & (1 as libc::c_int) << 17 as libc::c_int != 0 {
            return ret;
        }
    }
    if options & (1 as libc::c_int) << 14 as libc::c_int != 0
        || options & (1 as libc::c_int) << 8 as libc::c_int != 0
    {
        ret = cplus_demangle_v3(mangled, options);
        if !ret.is_null() || options & (1 as libc::c_int) << 14 as libc::c_int != 0 {
            return ret;
        }
    }
    if options & (1 as libc::c_int) << 2 as libc::c_int != 0 {
        ret = java_demangle_v3(mangled);
        if !ret.is_null() {
            return ret;
        }
    }
    if options & (1 as libc::c_int) << 15 as libc::c_int != 0 {
        return ada_demangle(mangled, options);
    }
    if options & (1 as libc::c_int) << 16 as libc::c_int != 0 {
        ret = dlang_demangle(mangled, options);
        if !ret.is_null() {
            return ret;
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn ada_demangle(
    mut mangled: *const libc::c_char,
    mut option: libc::c_int,
) -> *mut libc::c_char {
    let mut current_block: u64;
    let mut len0: libc::c_int = 0;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut d: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut demangled: *mut libc::c_char = 0 as *mut libc::c_char;
    if strncmp(
        mangled,
        b"_ada_\0" as *const u8 as *const libc::c_char,
        5 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int
    {
        mangled = mangled.offset(5 as libc::c_int as isize);
    }
    if !(_sch_istable[(*mangled.offset(0 as libc::c_int as isize) as libc::c_int
        & 0xff as libc::c_int) as usize] as libc::c_int
        & _sch_islower as libc::c_int as libc::c_ushort as libc::c_int == 0)
    {
        len0 = (strlen(mangled))
            .wrapping_add(7 as libc::c_int as libc::c_ulong)
            .wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int;
        demangled = xmalloc(
            (::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
                .wrapping_mul(len0 as libc::c_ulong),
        ) as *mut libc::c_char;
        d = demangled;
        p = mangled;
        loop {
            if _sch_istable[(*p as libc::c_int & 0xff as libc::c_int) as usize]
                as libc::c_int
                & _sch_islower as libc::c_int as libc::c_ushort as libc::c_int != 0
            {
                loop {
                    let fresh0 = p;
                    p = p.offset(1);
                    let fresh1 = d;
                    d = d.offset(1);
                    *fresh1 = *fresh0;
                    if !(_sch_istable[(*p as libc::c_int & 0xff as libc::c_int) as usize]
                        as libc::c_int
                        & _sch_islower as libc::c_int as libc::c_ushort as libc::c_int
                        != 0
                        || _sch_istable[(*p as libc::c_int & 0xff as libc::c_int)
                            as usize] as libc::c_int
                            & _sch_isdigit as libc::c_int as libc::c_ushort
                                as libc::c_int != 0
                        || *p.offset(0 as libc::c_int as isize) as libc::c_int
                            == '_' as i32
                            && (_sch_istable[(*p.offset(1 as libc::c_int as isize)
                                as libc::c_int & 0xff as libc::c_int) as usize]
                                as libc::c_int
                                & _sch_islower as libc::c_int as libc::c_ushort
                                    as libc::c_int != 0
                                || _sch_istable[(*p.offset(1 as libc::c_int as isize)
                                    as libc::c_int & 0xff as libc::c_int) as usize]
                                    as libc::c_int
                                    & _sch_isdigit as libc::c_int as libc::c_ushort
                                        as libc::c_int != 0))
                    {
                        break;
                    }
                }
            } else if *p.offset(0 as libc::c_int as isize) as libc::c_int == 'O' as i32 {
                static mut operators: [[*const libc::c_char; 2]; 20] = [
                    [
                        b"Oabs\0" as *const u8 as *const libc::c_char,
                        b"abs\0" as *const u8 as *const libc::c_char,
                    ],
                    [
                        b"Oand\0" as *const u8 as *const libc::c_char,
                        b"and\0" as *const u8 as *const libc::c_char,
                    ],
                    [
                        b"Omod\0" as *const u8 as *const libc::c_char,
                        b"mod\0" as *const u8 as *const libc::c_char,
                    ],
                    [
                        b"Onot\0" as *const u8 as *const libc::c_char,
                        b"not\0" as *const u8 as *const libc::c_char,
                    ],
                    [
                        b"Oor\0" as *const u8 as *const libc::c_char,
                        b"or\0" as *const u8 as *const libc::c_char,
                    ],
                    [
                        b"Orem\0" as *const u8 as *const libc::c_char,
                        b"rem\0" as *const u8 as *const libc::c_char,
                    ],
                    [
                        b"Oxor\0" as *const u8 as *const libc::c_char,
                        b"xor\0" as *const u8 as *const libc::c_char,
                    ],
                    [
                        b"Oeq\0" as *const u8 as *const libc::c_char,
                        b"=\0" as *const u8 as *const libc::c_char,
                    ],
                    [
                        b"One\0" as *const u8 as *const libc::c_char,
                        b"/=\0" as *const u8 as *const libc::c_char,
                    ],
                    [
                        b"Olt\0" as *const u8 as *const libc::c_char,
                        b"<\0" as *const u8 as *const libc::c_char,
                    ],
                    [
                        b"Ole\0" as *const u8 as *const libc::c_char,
                        b"<=\0" as *const u8 as *const libc::c_char,
                    ],
                    [
                        b"Ogt\0" as *const u8 as *const libc::c_char,
                        b">\0" as *const u8 as *const libc::c_char,
                    ],
                    [
                        b"Oge\0" as *const u8 as *const libc::c_char,
                        b">=\0" as *const u8 as *const libc::c_char,
                    ],
                    [
                        b"Oadd\0" as *const u8 as *const libc::c_char,
                        b"+\0" as *const u8 as *const libc::c_char,
                    ],
                    [
                        b"Osubtract\0" as *const u8 as *const libc::c_char,
                        b"-\0" as *const u8 as *const libc::c_char,
                    ],
                    [
                        b"Oconcat\0" as *const u8 as *const libc::c_char,
                        b"&\0" as *const u8 as *const libc::c_char,
                    ],
                    [
                        b"Omultiply\0" as *const u8 as *const libc::c_char,
                        b"*\0" as *const u8 as *const libc::c_char,
                    ],
                    [
                        b"Odivide\0" as *const u8 as *const libc::c_char,
                        b"/\0" as *const u8 as *const libc::c_char,
                    ],
                    [
                        b"Oexpon\0" as *const u8 as *const libc::c_char,
                        b"**\0" as *const u8 as *const libc::c_char,
                    ],
                    [0 as *const libc::c_char, 0 as *const libc::c_char],
                ];
                let mut k: libc::c_int = 0;
                k = 0 as libc::c_int;
                while !(operators[k as usize][0 as libc::c_int as usize]).is_null() {
                    let mut slen: size_t = strlen(
                        operators[k as usize][0 as libc::c_int as usize],
                    );
                    if strncmp(p, operators[k as usize][0 as libc::c_int as usize], slen)
                        == 0 as libc::c_int
                    {
                        p = p.offset(slen as isize);
                        slen = strlen(operators[k as usize][1 as libc::c_int as usize]);
                        let fresh2 = d;
                        d = d.offset(1);
                        *fresh2 = '"' as i32 as libc::c_char;
                        memcpy(
                            d as *mut libc::c_void,
                            operators[k as usize][1 as libc::c_int as usize]
                                as *const libc::c_void,
                            slen,
                        );
                        d = d.offset(slen as isize);
                        let fresh3 = d;
                        d = d.offset(1);
                        *fresh3 = '"' as i32 as libc::c_char;
                        break;
                    } else {
                        k += 1;
                        k;
                    }
                }
                if (operators[k as usize][0 as libc::c_int as usize]).is_null() {
                    current_block = 11743042597625389825;
                    break;
                }
            } else {
                current_block = 11743042597625389825;
                break;
            }
            if *p.offset(0 as libc::c_int as isize) as libc::c_int == 'T' as i32
                && *p.offset(1 as libc::c_int as isize) as libc::c_int == 'K' as i32
            {
                if *p.offset(2 as libc::c_int as isize) as libc::c_int == 'B' as i32
                    && *p.offset(3 as libc::c_int as isize) as libc::c_int
                        == 0 as libc::c_int
                {
                    current_block = 2798392256336243897;
                    break;
                } else {
                    if !(*p.offset(2 as libc::c_int as isize) as libc::c_int
                        == '_' as i32
                        && *p.offset(3 as libc::c_int as isize) as libc::c_int
                            == '_' as i32)
                    {
                        current_block = 11743042597625389825;
                        break;
                    }
                    p = p.offset(4 as libc::c_int as isize);
                    let fresh4 = d;
                    d = d.offset(1);
                    *fresh4 = '.' as i32 as libc::c_char;
                }
            } else if *p.offset(0 as libc::c_int as isize) as libc::c_int == 'E' as i32
                && *p.offset(1 as libc::c_int as isize) as libc::c_int
                    == 0 as libc::c_int
            {
                current_block = 11743042597625389825;
                break;
            } else {
                if (*p.offset(0 as libc::c_int as isize) as libc::c_int == 'P' as i32
                    || *p.offset(0 as libc::c_int as isize) as libc::c_int == 'N' as i32)
                    && *p.offset(1 as libc::c_int as isize) as libc::c_int
                        == 0 as libc::c_int
                {
                    current_block = 2798392256336243897;
                    break;
                }
                if (*p as libc::c_int == 'N' as i32 || *p as libc::c_int == 'S' as i32)
                    && *p.offset(1 as libc::c_int as isize) as libc::c_int
                        == 0 as libc::c_int
                {
                    current_block = 11743042597625389825;
                    break;
                }
                if *p.offset(0 as libc::c_int as isize) as libc::c_int == 'X' as i32 {
                    p = p.offset(1);
                    p;
                    while *p.offset(0 as libc::c_int as isize) as libc::c_int
                        == 'n' as i32
                        || *p.offset(0 as libc::c_int as isize) as libc::c_int
                            == 'b' as i32
                    {
                        p = p.offset(1);
                        p;
                    }
                }
                if *p.offset(0 as libc::c_int as isize) as libc::c_int == 'S' as i32
                    && *p.offset(1 as libc::c_int as isize) as libc::c_int
                        != 0 as libc::c_int
                    && (*p.offset(2 as libc::c_int as isize) as libc::c_int == '_' as i32
                        || *p.offset(2 as libc::c_int as isize) as libc::c_int
                            == 0 as libc::c_int)
                {
                    let mut name: *const libc::c_char = 0 as *const libc::c_char;
                    match *p.offset(1 as libc::c_int as isize) as libc::c_int {
                        82 => {
                            name = b"'Read\0" as *const u8 as *const libc::c_char;
                        }
                        87 => {
                            name = b"'Write\0" as *const u8 as *const libc::c_char;
                        }
                        73 => {
                            name = b"'Input\0" as *const u8 as *const libc::c_char;
                        }
                        79 => {
                            name = b"'Output\0" as *const u8 as *const libc::c_char;
                        }
                        _ => {
                            current_block = 11743042597625389825;
                            break;
                        }
                    }
                    p = p.offset(2 as libc::c_int as isize);
                    strcpy(d, name);
                    d = d.offset(strlen(name) as isize);
                } else if *p.offset(0 as libc::c_int as isize) as libc::c_int
                    == 'D' as i32
                {
                    let mut name_0: *const libc::c_char = 0 as *const libc::c_char;
                    match *p.offset(1 as libc::c_int as isize) as libc::c_int {
                        70 => {
                            name_0 = b".Finalize\0" as *const u8 as *const libc::c_char;
                        }
                        65 => {
                            name_0 = b".Adjust\0" as *const u8 as *const libc::c_char;
                        }
                        _ => {
                            current_block = 11743042597625389825;
                            break;
                        }
                    }
                    strcpy(d, name_0);
                    d = d.offset(strlen(name_0) as isize);
                    current_block = 2798392256336243897;
                    break;
                }
                if *p.offset(0 as libc::c_int as isize) as libc::c_int == '_' as i32 {
                    if *p.offset(1 as libc::c_int as isize) as libc::c_int == '_' as i32
                    {
                        p = p.offset(2 as libc::c_int as isize);
                        if _sch_istable[(*p as libc::c_int & 0xff as libc::c_int)
                            as usize] as libc::c_int
                            & _sch_isdigit as libc::c_int as libc::c_ushort
                                as libc::c_int != 0
                        {
                            loop {
                                p = p.offset(1);
                                p;
                                if !(_sch_istable[(*p as libc::c_int & 0xff as libc::c_int)
                                    as usize] as libc::c_int
                                    & _sch_isdigit as libc::c_int as libc::c_ushort
                                        as libc::c_int != 0
                                    || *p.offset(0 as libc::c_int as isize) as libc::c_int
                                        == '_' as i32
                                        && _sch_istable[(*p.offset(1 as libc::c_int as isize)
                                            as libc::c_int & 0xff as libc::c_int) as usize]
                                            as libc::c_int
                                            & _sch_isdigit as libc::c_int as libc::c_ushort
                                                as libc::c_int != 0)
                                {
                                    break;
                                }
                            }
                            if *p as libc::c_int == 'X' as i32 {
                                p = p.offset(1);
                                p;
                                while *p.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'n' as i32
                                    || *p.offset(0 as libc::c_int as isize) as libc::c_int
                                        == 'b' as i32
                                {
                                    p = p.offset(1);
                                    p;
                                }
                            }
                        } else if *p.offset(0 as libc::c_int as isize) as libc::c_int
                            == '_' as i32
                            && *p.offset(1 as libc::c_int as isize) as libc::c_int
                                != '_' as i32
                        {
                            static mut special: [[*const libc::c_char; 2]; 6] = [
                                [
                                    b"_elabb\0" as *const u8 as *const libc::c_char,
                                    b"'Elab_Body\0" as *const u8 as *const libc::c_char,
                                ],
                                [
                                    b"_elabs\0" as *const u8 as *const libc::c_char,
                                    b"'Elab_Spec\0" as *const u8 as *const libc::c_char,
                                ],
                                [
                                    b"_size\0" as *const u8 as *const libc::c_char,
                                    b"'Size\0" as *const u8 as *const libc::c_char,
                                ],
                                [
                                    b"_alignment\0" as *const u8 as *const libc::c_char,
                                    b"'Alignment\0" as *const u8 as *const libc::c_char,
                                ],
                                [
                                    b"_assign\0" as *const u8 as *const libc::c_char,
                                    b".\":=\"\0" as *const u8 as *const libc::c_char,
                                ],
                                [0 as *const libc::c_char, 0 as *const libc::c_char],
                            ];
                            let mut k_0: libc::c_int = 0;
                            k_0 = 0 as libc::c_int;
                            while !(special[k_0 as usize][0 as libc::c_int as usize])
                                .is_null()
                            {
                                let mut slen_0: size_t = strlen(
                                    special[k_0 as usize][0 as libc::c_int as usize],
                                );
                                if strncmp(
                                    p,
                                    special[k_0 as usize][0 as libc::c_int as usize],
                                    slen_0,
                                ) == 0 as libc::c_int
                                {
                                    p = p.offset(slen_0 as isize);
                                    slen_0 = strlen(
                                        special[k_0 as usize][1 as libc::c_int as usize],
                                    );
                                    memcpy(
                                        d as *mut libc::c_void,
                                        special[k_0 as usize][1 as libc::c_int as usize]
                                            as *const libc::c_void,
                                        slen_0,
                                    );
                                    d = d.offset(slen_0 as isize);
                                    break;
                                } else {
                                    k_0 += 1;
                                    k_0;
                                }
                            }
                            if !(special[k_0 as usize][0 as libc::c_int as usize])
                                .is_null()
                            {
                                current_block = 2798392256336243897;
                                break;
                            } else {
                                current_block = 11743042597625389825;
                                break;
                            }
                        } else {
                            let fresh5 = d;
                            d = d.offset(1);
                            *fresh5 = '.' as i32 as libc::c_char;
                            continue;
                        }
                    } else {
                        if !(*p.offset(1 as libc::c_int as isize) as libc::c_int
                            == 'B' as i32
                            || *p.offset(1 as libc::c_int as isize) as libc::c_int
                                == 'E' as i32)
                        {
                            current_block = 11743042597625389825;
                            break;
                        }
                        p = p.offset(2 as libc::c_int as isize);
                        while _sch_istable[(*p as libc::c_int & 0xff as libc::c_int)
                            as usize] as libc::c_int
                            & _sch_isdigit as libc::c_int as libc::c_ushort
                                as libc::c_int != 0
                        {
                            p = p.offset(1);
                            p;
                        }
                        if *p.offset(0 as libc::c_int as isize) as libc::c_int
                            == 's' as i32
                            && *p.offset(1 as libc::c_int as isize) as libc::c_int
                                == 0 as libc::c_int
                        {
                            current_block = 2798392256336243897;
                            break;
                        } else {
                            current_block = 11743042597625389825;
                            break;
                        }
                    }
                }
                if *p.offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32
                    && _sch_istable[(*p.offset(1 as libc::c_int as isize) as libc::c_int
                        & 0xff as libc::c_int) as usize] as libc::c_int
                        & _sch_isdigit as libc::c_int as libc::c_ushort as libc::c_int
                        != 0
                {
                    p = p.offset(2 as libc::c_int as isize);
                    while _sch_istable[(*p as libc::c_int & 0xff as libc::c_int)
                        as usize] as libc::c_int
                        & _sch_isdigit as libc::c_int as libc::c_ushort as libc::c_int
                        != 0
                    {
                        p = p.offset(1);
                        p;
                    }
                }
                if !(*p as libc::c_int == 0 as libc::c_int) {
                    current_block = 11743042597625389825;
                    break;
                }
                current_block = 2798392256336243897;
                break;
            }
        }
        match current_block {
            11743042597625389825 => {}
            _ => {
                *d = 0 as libc::c_int as libc::c_char;
                return demangled;
            }
        }
    }
    free(demangled as *mut libc::c_void);
    len0 = strlen(mangled) as libc::c_int;
    demangled = xmalloc(
        (::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
            .wrapping_mul((len0 + 3 as libc::c_int) as libc::c_ulong),
    ) as *mut libc::c_char;
    if *mangled.offset(0 as libc::c_int as isize) as libc::c_int == '<' as i32 {
        strcpy(demangled, mangled);
    } else {
        sprintf(demangled, b"<%s>\0" as *const u8 as *const libc::c_char, mangled);
    }
    return demangled;
}
