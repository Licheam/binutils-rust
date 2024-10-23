use ::libc;
extern "C" {
    static _sch_istable: [libc::c_ushort; 256];
}
pub const _sch_isalpha: C2RustUnnamed = 136;
pub type C2RustUnnamed = libc::c_uint;
pub const _sch_isbasic: C2RustUnnamed = 3088;
pub const _sch_iscppsp: C2RustUnnamed = 3072;
pub const _sch_isgraph: C2RustUnnamed = 172;
pub const _sch_isidnum: C2RustUnnamed = 516;
pub const _sch_isalnum: C2RustUnnamed = 140;
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
#[no_mangle]
pub unsafe extern "C" fn unix_lbasename(
    mut name: *const libc::c_char,
) -> *const libc::c_char {
    let mut base: *const libc::c_char = 0 as *const libc::c_char;
    base = name;
    while *name != 0 {
        if *name as libc::c_int == '/' as i32
            || *name as libc::c_int == '\\' as i32 && 0 as libc::c_int != 0
        {
            base = name.offset(1 as libc::c_int as isize);
        }
        name = name.offset(1);
        name;
    }
    return base;
}
#[no_mangle]
pub unsafe extern "C" fn dos_lbasename(
    mut name: *const libc::c_char,
) -> *const libc::c_char {
    let mut base: *const libc::c_char = 0 as *const libc::c_char;
    if _sch_istable[(*name.offset(0 as libc::c_int as isize) as libc::c_int
        & 0xff as libc::c_int) as usize] as libc::c_int
        & _sch_isalpha as libc::c_int as libc::c_ushort as libc::c_int != 0
        && *name.offset(1 as libc::c_int as isize) as libc::c_int == ':' as i32
    {
        name = name.offset(2 as libc::c_int as isize);
    }
    base = name;
    while *name != 0 {
        if *name as libc::c_int == '/' as i32
            || *name as libc::c_int == '\\' as i32 && 1 as libc::c_int != 0
        {
            base = name.offset(1 as libc::c_int as isize);
        }
        name = name.offset(1);
        name;
    }
    return base;
}
#[no_mangle]
pub unsafe extern "C" fn lbasename(
    mut name: *const libc::c_char,
) -> *const libc::c_char {
    return unix_lbasename(name);
}
