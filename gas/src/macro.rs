extern "C" {
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncasecmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn free(_: *mut libc::c_void);
    fn xcalloc(_: size_t, _: size_t) -> *mut libc::c_void;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn as_bad(format: *const libc::c_char, _: ...);
    fn as_warn(format: *const libc::c_char, _: ...);
    fn as_bad_where(
        file: *const libc::c_char,
        line: libc::c_uint,
        format: *const libc::c_char,
        _: ...
    );
    fn as_warn_where(
        file: *const libc::c_char,
        line: libc::c_uint,
        format: *const libc::c_char,
        _: ...
    );
    fn htab_create_alloc(
        _: size_t,
        _: htab_hash,
        _: htab_eq,
        _: htab_del,
        _: htab_alloc,
        _: htab_free,
    ) -> htab_t;
    fn htab_delete(_: htab_t);
    fn htab_find(_: htab_t, _: *const libc::c_void) -> *mut libc::c_void;
    fn htab_find_slot(
        _: htab_t,
        _: *const libc::c_void,
        _: insert_option,
    ) -> *mut *mut libc::c_void;
    fn htab_clear_slot(_: htab_t, _: *mut *mut libc::c_void);
    fn htab_remove_elt(_: htab_t, _: *const libc::c_void);
    fn htab_hash_string(_: *const libc::c_void) -> hashval_t;
    fn htab_insert(
        _: htab_t,
        _: *mut libc::c_void,
        _: libc::c_int,
    ) -> *mut *mut libc::c_void;
    static mut lex_type: [libc::c_char; 0];
    fn s_app_line(_: libc::c_int);
    fn temp_ilp(_: *mut libc::c_char);
    fn restore_ilp();
    static _sch_istable: [libc::c_ushort; 256];
    static _sch_tolower: [libc::c_uchar; 256];
    fn sb_new(_: *mut sb);
    fn sb_kill(_: *mut sb);
    fn sb_add_sb(_: *mut sb, _: *mut sb);
    fn sb_reset(_: *mut sb);
    fn sb_add_char(_: *mut sb, _: size_t);
    fn sb_add_string(_: *mut sb, _: *const libc::c_char);
    fn sb_add_buffer(_: *mut sb, _: *const libc::c_char, _: size_t);
    fn sb_terminate(_: *mut sb) -> *mut libc::c_char;
    fn sb_skip_white(_: size_t, _: *mut sb) -> size_t;
    fn sb_skip_comma(_: size_t, _: *mut sb) -> size_t;
}
pub type size_t = libc::c_ulong;
pub type bfd_signed_vma = libc::c_long;
pub type offsetT = bfd_signed_vma;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct htab {
    pub hash_f: htab_hash,
    pub eq_f: htab_eq,
    pub del_f: htab_del,
    pub entries: *mut *mut libc::c_void,
    pub size: size_t,
    pub n_elements: size_t,
    pub n_deleted: size_t,
    pub searches: libc::c_uint,
    pub collisions: libc::c_uint,
    pub alloc_f: htab_alloc,
    pub free_f: htab_free,
    pub alloc_arg: *mut libc::c_void,
    pub alloc_with_arg_f: htab_alloc_with_arg,
    pub free_with_arg_f: htab_free_with_arg,
    pub size_prime_index: libc::c_uint,
}
pub type htab_free_with_arg = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
>;
pub type htab_alloc_with_arg = Option::<
    unsafe extern "C" fn(*mut libc::c_void, size_t, size_t) -> *mut libc::c_void,
>;
pub type htab_free = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type htab_alloc = Option::<
    unsafe extern "C" fn(size_t, size_t) -> *mut libc::c_void,
>;
pub type htab_del = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type htab_eq = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
pub type htab_hash = Option::<unsafe extern "C" fn(*const libc::c_void) -> hashval_t>;
pub type hashval_t = libc::c_uint;
pub type htab_t = *mut htab;
pub type insert_option = libc::c_uint;
pub const INSERT: insert_option = 1;
pub const NO_INSERT: insert_option = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sb {
    pub ptr: *mut libc::c_char,
    pub len: size_t,
    pub max: size_t,
}
pub type formal_type = libc::c_uint;
pub const FORMAL_VARARG: formal_type = 2;
pub const FORMAL_REQUIRED: formal_type = 1;
pub const FORMAL_OPTIONAL: formal_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct formal_struct {
    pub next: *mut formal_struct,
    pub name: sb,
    pub def: sb,
    pub actual: sb,
    pub index: libc::c_int,
    pub type_0: formal_type,
}
pub type formal_entry = formal_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct macro_struct {
    pub sub: sb,
    pub formal_count: libc::c_int,
    pub formals: *mut formal_entry,
    pub formal_hash: *mut htab,
    pub name: *const libc::c_char,
    pub file: *const libc::c_char,
    pub line: libc::c_uint,
}
pub type macro_entry = macro_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct macro_hash_entry {
    pub name: *const libc::c_char,
    pub macro_0: *mut macro_entry,
}
pub type macro_hash_entry_t = macro_hash_entry;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct formal_hash_entry {
    pub name: *const libc::c_char,
    pub formal: *mut formal_entry,
}
pub type formal_hash_entry_t = formal_hash_entry;
#[inline]
unsafe extern "C" fn xmemdup0(
    mut in_0: *const libc::c_char,
    mut len: size_t,
) -> *mut libc::c_char {
    let mut out: *mut libc::c_char = xmalloc(
        len.wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    *out.offset(len as isize) = 0 as libc::c_int as libc::c_char;
    return memcpy(out as *mut libc::c_void, in_0 as *const libc::c_void, len)
        as *mut libc::c_char;
}
#[no_mangle]
pub static mut macro_defined: libc::c_int = 0;
#[no_mangle]
pub static mut macro_hash: *mut htab = 0 as *const htab as *mut htab;
#[inline]
unsafe extern "C" fn hash_macro_entry(mut e: *const libc::c_void) -> hashval_t {
    let mut entry: *const macro_hash_entry_t = e as *const macro_hash_entry_t;
    return htab_hash_string((*entry).name as *const libc::c_void);
}
#[inline]
unsafe extern "C" fn eq_macro_entry(
    mut a: *const libc::c_void,
    mut b: *const libc::c_void,
) -> libc::c_int {
    let mut ea: *const macro_hash_entry_t = a as *const macro_hash_entry_t;
    let mut eb: *const macro_hash_entry_t = b as *const macro_hash_entry_t;
    return (strcmp((*ea).name, (*eb).name) == 0 as libc::c_int) as libc::c_int;
}
#[inline]
unsafe extern "C" fn macro_entry_alloc(
    mut name: *const libc::c_char,
    mut macro_0: *mut macro_entry,
) -> *mut macro_hash_entry_t {
    let mut entry: *mut macro_hash_entry_t = xmalloc(
        ::core::mem::size_of::<macro_hash_entry_t>() as libc::c_ulong,
    ) as *mut macro_hash_entry_t;
    (*entry).name = name;
    (*entry).macro_0 = macro_0;
    return entry;
}
#[inline]
unsafe extern "C" fn macro_entry_find(
    mut table: htab_t,
    mut name: *const libc::c_char,
) -> *mut macro_entry {
    let mut needle: macro_hash_entry_t = {
        let mut init = macro_hash_entry {
            name: name,
            macro_0: 0 as *mut macro_entry,
        };
        init
    };
    let mut entry: *mut macro_hash_entry_t = htab_find(
        table,
        &mut needle as *mut macro_hash_entry_t as *const libc::c_void,
    ) as *mut macro_hash_entry_t;
    return if !entry.is_null() { (*entry).macro_0 } else { 0 as *mut macro_entry };
}
#[inline]
unsafe extern "C" fn hash_formal_entry(mut e: *const libc::c_void) -> hashval_t {
    let mut entry: *const formal_hash_entry_t = e as *const formal_hash_entry_t;
    return htab_hash_string((*entry).name as *const libc::c_void);
}
#[inline]
unsafe extern "C" fn eq_formal_entry(
    mut a: *const libc::c_void,
    mut b: *const libc::c_void,
) -> libc::c_int {
    let mut ea: *const formal_hash_entry_t = a as *const formal_hash_entry_t;
    let mut eb: *const formal_hash_entry_t = b as *const formal_hash_entry_t;
    return (strcmp((*ea).name, (*eb).name) == 0 as libc::c_int) as libc::c_int;
}
#[inline]
unsafe extern "C" fn formal_entry_alloc(
    mut name: *const libc::c_char,
    mut formal: *mut formal_entry,
) -> *mut formal_hash_entry_t {
    let mut entry: *mut formal_hash_entry_t = xmalloc(
        ::core::mem::size_of::<formal_hash_entry_t>() as libc::c_ulong,
    ) as *mut formal_hash_entry_t;
    (*entry).name = name;
    (*entry).formal = formal;
    return entry;
}
#[inline]
unsafe extern "C" fn formal_entry_find(
    mut table: htab_t,
    mut name: *const libc::c_char,
) -> *mut formal_entry {
    let mut needle: formal_hash_entry_t = {
        let mut init = formal_hash_entry {
            name: name,
            formal: 0 as *mut formal_entry,
        };
        init
    };
    let mut entry: *mut formal_hash_entry_t = htab_find(
        table,
        &mut needle as *mut formal_hash_entry_t as *const libc::c_void,
    ) as *mut formal_hash_entry_t;
    return if !entry.is_null() { (*entry).formal } else { 0 as *mut formal_entry };
}
#[no_mangle]
pub unsafe extern "C" fn buffer_and_nest(
    mut from: *const libc::c_char,
    mut to: *const libc::c_char,
    mut ptr: *mut sb,
    mut get_line: Option::<unsafe extern "C" fn(*mut sb) -> size_t>,
) -> libc::c_int {
    let mut from_len: size_t = 0;
    let mut to_len: size_t = strlen(to);
    let mut depth: libc::c_int = 1 as libc::c_int;
    let mut line_start: size_t = (*ptr).len;
    let mut more: size_t = get_line.expect("non-null function pointer")(ptr);
    if to_len == 4 as libc::c_int as libc::c_ulong
        && strcasecmp(to, b"ENDR\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
    {
        from = 0 as *const libc::c_char;
        from_len = 0 as libc::c_int as size_t;
    } else {
        from_len = strlen(from);
    }
    while more != 0 {
        let mut i: size_t = line_start;
        let mut had_colon: bool = 0 as libc::c_int != 0;
        if 0 as libc::c_int == 0 {
            while i < (*ptr).len
                && (*((*ptr).ptr).offset(i as isize) as libc::c_int == ' ' as i32
                    || *((*ptr).ptr).offset(i as isize) as libc::c_int == '\t' as i32)
            {
                i = i.wrapping_add(1);
                i;
            }
        }
        while !(i >= (*ptr).len
            || *lex_type
                .as_mut_ptr()
                .offset(*((*ptr).ptr).offset(i as isize) as libc::c_uchar as isize)
                as libc::c_int & 2 as libc::c_int == 0)
        {
            i = i.wrapping_add(1);
            i;
            while i < (*ptr).len
                && *lex_type
                    .as_mut_ptr()
                    .offset(*((*ptr).ptr).offset(i as isize) as libc::c_uchar as isize)
                    as libc::c_int & 1 as libc::c_int != 0
            {
                i = i.wrapping_add(1);
                i;
            }
            if i < (*ptr).len
                && *lex_type
                    .as_mut_ptr()
                    .offset(*((*ptr).ptr).offset(i as isize) as libc::c_uchar as isize)
                    as libc::c_int & 4 as libc::c_int != 0
            {
                i = i.wrapping_add(1);
                i;
            }
            while i < (*ptr).len
                && (*((*ptr).ptr).offset(i as isize) as libc::c_int == ' ' as i32
                    || *((*ptr).ptr).offset(i as isize) as libc::c_int == '\t' as i32)
            {
                i = i.wrapping_add(1);
                i;
            }
            if i >= (*ptr).len
                || *((*ptr).ptr).offset(i as isize) as libc::c_int != ':' as i32
            {
                if 0 as libc::c_int != 0 && !had_colon {
                    break;
                }
                i = line_start;
                break;
            } else {
                i = i.wrapping_add(1);
                i;
                line_start = i;
                had_colon = 1 as libc::c_int != 0;
            }
        }
        while i < (*ptr).len
            && (*((*ptr).ptr).offset(i as isize) as libc::c_int == ' ' as i32
                || *((*ptr).ptr).offset(i as isize) as libc::c_int == '\t' as i32)
        {
            i = i.wrapping_add(1);
            i;
        }
        if i < (*ptr).len
            && (*((*ptr).ptr).offset(i as isize) as libc::c_int == '.' as i32
                || 0 as libc::c_int != 0 || macro_mri != 0)
        {
            if 0 as libc::c_int == 0
                && *((*ptr).ptr).offset(i as isize) as libc::c_int == '.' as i32
            {
                i = i.wrapping_add(1);
                i;
            }
            if from.is_null()
                && {
                    from_len = 4 as libc::c_int as size_t;
                    strncasecmp(
                        ((*ptr).ptr).offset(i as isize),
                        b"IRPC\0" as *const u8 as *const libc::c_char,
                        from_len,
                    ) != 0 as libc::c_int
                }
                && {
                    from_len = 3 as libc::c_int as size_t;
                    strncasecmp(
                        ((*ptr).ptr).offset(i as isize),
                        b"IRP\0" as *const u8 as *const libc::c_char,
                        from_len,
                    ) != 0 as libc::c_int
                }
                && {
                    from_len = 5 as libc::c_int as size_t;
                    strncasecmp(
                        ((*ptr).ptr).offset(i as isize),
                        b"IREPC\0" as *const u8 as *const libc::c_char,
                        from_len,
                    ) != 0 as libc::c_int
                }
                && {
                    from_len = 4 as libc::c_int as size_t;
                    strncasecmp(
                        ((*ptr).ptr).offset(i as isize),
                        b"IREP\0" as *const u8 as *const libc::c_char,
                        from_len,
                    ) != 0 as libc::c_int
                }
                && {
                    from_len = 4 as libc::c_int as size_t;
                    strncasecmp(
                        ((*ptr).ptr).offset(i as isize),
                        b"REPT\0" as *const u8 as *const libc::c_char,
                        from_len,
                    ) != 0 as libc::c_int
                }
                && {
                    from_len = 3 as libc::c_int as size_t;
                    strncasecmp(
                        ((*ptr).ptr).offset(i as isize),
                        b"REP\0" as *const u8 as *const libc::c_char,
                        from_len,
                    ) != 0 as libc::c_int
                }
            {
                from_len = 0 as libc::c_int as size_t;
            }
            if (if !from.is_null() {
                (strncasecmp(((*ptr).ptr).offset(i as isize), from, from_len)
                    == 0 as libc::c_int) as libc::c_int
            } else {
                (from_len > 0 as libc::c_int as libc::c_ulong) as libc::c_int
            }) != 0
                && ((*ptr).len == i.wrapping_add(from_len)
                    || !(*lex_type
                        .as_mut_ptr()
                        .offset(
                            *((*ptr).ptr).offset(i.wrapping_add(from_len) as isize)
                                as libc::c_uchar as isize,
                        ) as libc::c_int & 1 as libc::c_int != 0
                        || *lex_type
                            .as_mut_ptr()
                            .offset(
                                *((*ptr).ptr).offset(i.wrapping_add(from_len) as isize)
                                    as libc::c_uchar as isize,
                            ) as libc::c_int & 4 as libc::c_int != 0))
            {
                depth += 1;
                depth;
            }
            if strncasecmp(((*ptr).ptr).offset(i as isize), to, to_len)
                == 0 as libc::c_int
                && ((*ptr).len == i.wrapping_add(to_len)
                    || !(*lex_type
                        .as_mut_ptr()
                        .offset(
                            *((*ptr).ptr).offset(i.wrapping_add(to_len) as isize)
                                as libc::c_uchar as isize,
                        ) as libc::c_int & 1 as libc::c_int != 0
                        || *lex_type
                            .as_mut_ptr()
                            .offset(
                                *((*ptr).ptr).offset(i.wrapping_add(to_len) as isize)
                                    as libc::c_uchar as isize,
                            ) as libc::c_int & 4 as libc::c_int != 0))
            {
                depth -= 1;
                depth;
                if depth == 0 as libc::c_int {
                    (*ptr).len = line_start;
                    break;
                }
            }
            if strncasecmp(
                ((*ptr).ptr).offset(i as isize),
                b"linefile\0" as *const u8 as *const libc::c_char,
                8 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int
            {
                let mut saved_eol_char: libc::c_char = *((*ptr).ptr)
                    .offset((*ptr).len as isize);
                *((*ptr).ptr).offset((*ptr).len as isize) = '\0' as i32 as libc::c_char;
                temp_ilp(
                    ((*ptr).ptr).offset(i as isize).offset(8 as libc::c_int as isize),
                );
                s_app_line(0 as libc::c_int);
                restore_ilp();
                *((*ptr).ptr).offset((*ptr).len as isize) = saved_eol_char;
                (*ptr).len = line_start;
            }
        }
        sb_add_char(ptr, more);
        line_start = (*ptr).len;
        more = get_line.expect("non-null function pointer")(ptr);
    }
    return (depth == 0 as libc::c_int) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn macro_init(
    mut alternate: libc::c_int,
    mut mri: libc::c_int,
    mut strip_at: libc::c_int,
    mut exp: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            size_t,
            *mut sb,
            *mut offsetT,
        ) -> size_t,
    >,
) {
    macro_hash = htab_create_alloc(
        16 as libc::c_int as size_t,
        Some(hash_macro_entry as unsafe extern "C" fn(*const libc::c_void) -> hashval_t),
        Some(
            eq_macro_entry
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
        None,
        Some(xcalloc as unsafe extern "C" fn(size_t, size_t) -> *mut libc::c_void),
        Some(free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
    );
    macro_defined = 0 as libc::c_int;
    macro_alternate = alternate;
    macro_mri = mri;
    macro_strip_at = strip_at;
    macro_expr = exp;
}
#[no_mangle]
pub unsafe extern "C" fn macro_set_alternate(mut alternate: libc::c_int) {
    macro_alternate = alternate;
}
#[no_mangle]
pub unsafe extern "C" fn macro_mri_mode(mut mri: libc::c_int) {
    macro_mri = mri;
}
#[no_mangle]
pub unsafe extern "C" fn define_macro(
    mut idx: size_t,
    mut in_0: *mut sb,
    mut label: *mut sb,
    mut get_line: Option::<unsafe extern "C" fn(*mut sb) -> size_t>,
    mut file: *const libc::c_char,
    mut line: libc::c_uint,
    mut namep: *mut *const libc::c_char,
) -> *const libc::c_char {
    let mut macro_0: *mut macro_entry = 0 as *mut macro_entry;
    let mut name: sb = sb {
        ptr: 0 as *mut libc::c_char,
        len: 0,
        max: 0,
    };
    let mut error: *const libc::c_char = 0 as *const libc::c_char;
    macro_0 = xmalloc(::core::mem::size_of::<macro_entry>() as libc::c_ulong)
        as *mut macro_entry;
    sb_new(&mut (*macro_0).sub);
    sb_new(&mut name);
    (*macro_0).file = file;
    (*macro_0).line = line;
    (*macro_0).formal_count = 0 as libc::c_int;
    (*macro_0).formals = 0 as *mut formal_entry;
    (*macro_0)
        .formal_hash = htab_create_alloc(
        7 as libc::c_int as size_t,
        Some(
            hash_formal_entry as unsafe extern "C" fn(*const libc::c_void) -> hashval_t,
        ),
        Some(
            eq_formal_entry
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
        None,
        Some(xcalloc as unsafe extern "C" fn(size_t, size_t) -> *mut libc::c_void),
        Some(free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
    );
    idx = sb_skip_white(idx, in_0);
    if buffer_and_nest(
        b"MACRO\0" as *const u8 as *const libc::c_char,
        b"ENDM\0" as *const u8 as *const libc::c_char,
        &mut (*macro_0).sub,
        get_line,
    ) == 0
    {
        error = dcgettext(
            0 as *const libc::c_char,
            b"unexpected end of file in macro `%s' definition\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        );
    }
    if !label.is_null() && (*label).len != 0 as libc::c_int as libc::c_ulong {
        sb_add_sb(&mut name, label);
        (*macro_0).name = sb_terminate(&mut name);
        if idx < (*in_0).len
            && *((*in_0).ptr).offset(idx as isize) as libc::c_int == '(' as i32
        {
            idx = do_formals(
                macro_0,
                idx.wrapping_add(1 as libc::c_int as libc::c_ulong),
                in_0,
            );
            if idx < (*in_0).len
                && *((*in_0).ptr).offset(idx as isize) as libc::c_int == ')' as i32
            {
                idx = sb_skip_white(
                    idx.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    in_0,
                );
            } else if error.is_null() {
                error = dcgettext(
                    0 as *const libc::c_char,
                    b"missing `)' after formals in macro definition `%s'\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                );
            }
        } else {
            idx = do_formals(macro_0, idx, in_0);
        }
    } else {
        let mut cidx: size_t = 0;
        idx = get_token(idx, in_0, &mut name);
        (*macro_0).name = sb_terminate(&mut name);
        if name.len == 0 as libc::c_int as libc::c_ulong {
            error = dcgettext(
                0 as *const libc::c_char,
                b"Missing macro name\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            );
        }
        cidx = sb_skip_white(idx, in_0);
        idx = sb_skip_comma(cidx, in_0);
        if idx == cidx || idx < (*in_0).len {
            idx = do_formals(macro_0, idx, in_0);
        } else {
            idx = cidx;
        }
    }
    if error.is_null() && idx < (*in_0).len {
        error = dcgettext(
            0 as *const libc::c_char,
            b"Bad parameter list for macro `%s'\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        );
    }
    idx = 0 as libc::c_int as size_t;
    while idx < name.len {
        *(name.ptr)
            .offset(
                idx as isize,
            ) = _sch_tolower[(*(name.ptr).offset(idx as isize) as libc::c_int
            & 0xff as libc::c_int) as usize] as libc::c_char;
        idx = idx.wrapping_add(1);
        idx;
    }
    if error.is_null() {
        let mut elt: *mut macro_hash_entry_t = macro_entry_alloc(
            (*macro_0).name,
            macro_0,
        );
        if !(htab_insert(macro_hash, elt as *mut libc::c_void, 0 as libc::c_int))
            .is_null()
        {
            free(elt as *mut libc::c_void);
            error = dcgettext(
                0 as *const libc::c_char,
                b"Macro `%s' was already defined\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            );
        }
    }
    if !namep.is_null() {
        *namep = (*macro_0).name;
    }
    if error.is_null() {
        macro_defined = 1 as libc::c_int;
    } else {
        free_macro(macro_0);
    }
    return error;
}
#[no_mangle]
pub unsafe extern "C" fn check_macro(
    mut line: *const libc::c_char,
    mut expand: *mut sb,
    mut error: *mut *const libc::c_char,
    mut info: *mut *mut macro_entry,
) -> libc::c_int {
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut copy: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cls: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut macro_0: *mut macro_entry = 0 as *mut macro_entry;
    let mut line_sb: sb = sb {
        ptr: 0 as *mut libc::c_char,
        len: 0,
        max: 0,
    };
    if *lex_type.as_mut_ptr().offset(*line as libc::c_uchar as isize) as libc::c_int
        & 2 as libc::c_int == 0 && (macro_mri == 0 || *line as libc::c_int != '.' as i32)
    {
        return 0 as libc::c_int;
    }
    s = line.offset(1 as libc::c_int as isize);
    while *lex_type.as_mut_ptr().offset(*s as libc::c_uchar as isize) as libc::c_int
        & 1 as libc::c_int != 0
    {
        s = s.offset(1);
        s;
    }
    if *lex_type.as_mut_ptr().offset(*s as libc::c_uchar as isize) as libc::c_int
        & 4 as libc::c_int != 0
    {
        s = s.offset(1);
        s;
    }
    copy = xmemdup0(line, s.offset_from(line) as libc::c_long as size_t);
    cls = copy;
    while *cls as libc::c_int != '\0' as i32 {
        *cls = _sch_tolower[(*cls as libc::c_int & 0xff as libc::c_int) as usize]
            as libc::c_char;
        cls = cls.offset(1);
        cls;
    }
    macro_0 = macro_entry_find(macro_hash, copy);
    free(copy as *mut libc::c_void);
    if macro_0.is_null() {
        return 0 as libc::c_int;
    }
    sb_new(&mut line_sb);
    while *s as libc::c_int != '\0' as i32 && *s as libc::c_int != '\n' as i32
        && *s as libc::c_int != '\r' as i32
    {
        let fresh0 = s;
        s = s.offset(1);
        sb_add_char(&mut line_sb, *fresh0 as size_t);
    }
    sb_new(expand);
    *error = macro_expand(0 as libc::c_int as size_t, &mut line_sb, macro_0, expand);
    sb_kill(&mut line_sb);
    if !info.is_null() {
        *info = macro_0;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn delete_macro(mut name: *const libc::c_char) {
    let mut copy: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: size_t = 0;
    let mut len: size_t = 0;
    let mut slot: *mut *mut libc::c_void = 0 as *mut *mut libc::c_void;
    let mut needle: macro_hash_entry_t = macro_hash_entry_t {
        name: 0 as *const libc::c_char,
        macro_0: 0 as *mut macro_entry,
    };
    len = strlen(name);
    copy = xmalloc(
        (::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
            .wrapping_mul(len.wrapping_add(1 as libc::c_int as libc::c_ulong)),
    ) as *mut libc::c_char;
    i = 0 as libc::c_int as size_t;
    while i < len {
        *copy
            .offset(
                i as isize,
            ) = _sch_tolower[(*name.offset(i as isize) as libc::c_int
            & 0xff as libc::c_int) as usize] as libc::c_char;
        i = i.wrapping_add(1);
        i;
    }
    *copy.offset(i as isize) = '\0' as i32 as libc::c_char;
    needle.name = copy;
    needle.macro_0 = 0 as *mut macro_entry;
    slot = htab_find_slot(
        macro_hash,
        &mut needle as *mut macro_hash_entry_t as *const libc::c_void,
        NO_INSERT,
    );
    if !slot.is_null() {
        free_macro((*(*slot as *mut macro_hash_entry_t)).macro_0);
        htab_clear_slot(macro_hash, slot);
    } else {
        as_warn(
            dcgettext(
                0 as *const libc::c_char,
                b"Attempt to purge non-existing macro `%s'\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            copy,
        );
    }
    free(copy as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn expand_irp(
    mut irpc: libc::c_int,
    mut idx: size_t,
    mut in_0: *mut sb,
    mut out: *mut sb,
    mut get_line: Option::<unsafe extern "C" fn(*mut sb) -> size_t>,
) -> *const libc::c_char {
    let mut sub: sb = sb {
        ptr: 0 as *mut libc::c_char,
        len: 0,
        max: 0,
    };
    let mut f: formal_entry = formal_entry {
        next: 0 as *mut formal_struct,
        name: sb {
            ptr: 0 as *mut libc::c_char,
            len: 0,
            max: 0,
        },
        def: sb {
            ptr: 0 as *mut libc::c_char,
            len: 0,
            max: 0,
        },
        actual: sb {
            ptr: 0 as *mut libc::c_char,
            len: 0,
            max: 0,
        },
        index: 0,
        type_0: FORMAL_OPTIONAL,
    };
    let mut h: *mut htab = 0 as *mut htab;
    let mut err: *const libc::c_char = 0 as *const libc::c_char;
    idx = sb_skip_white(idx, in_0);
    sb_new(&mut sub);
    if buffer_and_nest(
        0 as *const libc::c_char,
        b"ENDR\0" as *const u8 as *const libc::c_char,
        &mut sub,
        get_line,
    ) == 0
    {
        return dcgettext(
            0 as *const libc::c_char,
            b"unexpected end of file in irp or irpc\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        );
    }
    sb_new(&mut f.name);
    sb_new(&mut f.def);
    sb_new(&mut f.actual);
    idx = get_token(idx, in_0, &mut f.name);
    if f.name.len == 0 as libc::c_int as libc::c_ulong {
        return dcgettext(
            0 as *const libc::c_char,
            b"missing model parameter\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        );
    }
    h = htab_create_alloc(
        16 as libc::c_int as size_t,
        Some(
            hash_formal_entry as unsafe extern "C" fn(*const libc::c_void) -> hashval_t,
        ),
        Some(
            eq_formal_entry
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
        None,
        Some(xcalloc as unsafe extern "C" fn(size_t, size_t) -> *mut libc::c_void),
        Some(free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
    );
    htab_insert(
        h,
        formal_entry_alloc(sb_terminate(&mut f.name), &mut f) as *mut libc::c_void,
        0 as libc::c_int,
    );
    f.index = 1 as libc::c_int;
    f.next = 0 as *mut formal_struct;
    f.type_0 = FORMAL_OPTIONAL;
    sb_reset(out);
    idx = sb_skip_comma(idx, in_0);
    if idx >= (*in_0).len {
        err = macro_expand_body(&mut sub, out, &mut f, h, 0 as *const macro_entry);
    } else {
        let mut in_quotes: bool = 0 as libc::c_int != 0;
        if irpc != 0 && *((*in_0).ptr).offset(idx as isize) as libc::c_int == '"' as i32
        {
            in_quotes = 1 as libc::c_int != 0;
            idx = idx.wrapping_add(1);
            idx;
        }
        while idx < (*in_0).len {
            if irpc == 0 {
                idx = get_any_string(idx, in_0, &mut f.actual);
            } else {
                if *((*in_0).ptr).offset(idx as isize) as libc::c_int == '"' as i32 {
                    let mut nxt: size_t = 0;
                    if irpc != 0 {
                        in_quotes = !in_quotes;
                    }
                    nxt = sb_skip_white(
                        idx.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        in_0,
                    );
                    if nxt >= (*in_0).len {
                        idx = nxt;
                        break;
                    }
                }
                sb_reset(&mut f.actual);
                sb_add_char(
                    &mut f.actual,
                    *((*in_0).ptr).offset(idx as isize) as size_t,
                );
                idx = idx.wrapping_add(1);
                idx;
            }
            err = macro_expand_body(&mut sub, out, &mut f, h, 0 as *const macro_entry);
            if !err.is_null() {
                break;
            }
            if irpc == 0 {
                idx = sb_skip_comma(idx, in_0);
            } else if !in_quotes {
                idx = sb_skip_white(idx, in_0);
            }
        }
    }
    htab_delete(h);
    sb_kill(&mut f.actual);
    sb_kill(&mut f.def);
    sb_kill(&mut f.name);
    sb_kill(&mut sub);
    return err;
}
static mut macro_alternate: libc::c_int = 0;
static mut macro_mri: libc::c_int = 0;
static mut macro_strip_at: libc::c_int = 0;
static mut macro_expr: Option::<
    unsafe extern "C" fn(*const libc::c_char, size_t, *mut sb, *mut offsetT) -> size_t,
> = None;
static mut macro_number: libc::c_int = 0;
unsafe extern "C" fn get_token(
    mut idx: size_t,
    mut in_0: *mut sb,
    mut name: *mut sb,
) -> size_t {
    if idx < (*in_0).len
        && *lex_type
            .as_mut_ptr()
            .offset(*((*in_0).ptr).offset(idx as isize) as libc::c_uchar as isize)
            as libc::c_int & 2 as libc::c_int != 0
    {
        let fresh1 = idx;
        idx = idx.wrapping_add(1);
        sb_add_char(name, *((*in_0).ptr).offset(fresh1 as isize) as size_t);
        while idx < (*in_0).len
            && *lex_type
                .as_mut_ptr()
                .offset(*((*in_0).ptr).offset(idx as isize) as libc::c_uchar as isize)
                as libc::c_int & 1 as libc::c_int != 0
        {
            let fresh2 = idx;
            idx = idx.wrapping_add(1);
            sb_add_char(name, *((*in_0).ptr).offset(fresh2 as isize) as size_t);
        }
        if idx < (*in_0).len
            && *lex_type
                .as_mut_ptr()
                .offset(*((*in_0).ptr).offset(idx as isize) as libc::c_uchar as isize)
                as libc::c_int & 4 as libc::c_int != 0
        {
            let fresh3 = idx;
            idx = idx.wrapping_add(1);
            sb_add_char(name, *((*in_0).ptr).offset(fresh3 as isize) as size_t);
        }
    }
    if macro_alternate != 0 && idx < (*in_0).len
        && *((*in_0).ptr).offset(idx as isize) as libc::c_int == '&' as i32
    {
        idx = idx.wrapping_add(1);
        idx;
    }
    return idx;
}
unsafe extern "C" fn getstring(
    mut idx: size_t,
    mut in_0: *mut sb,
    mut acc: *mut sb,
) -> size_t {
    while idx < (*in_0).len
        && (*((*in_0).ptr).offset(idx as isize) as libc::c_int == '"' as i32
            || *((*in_0).ptr).offset(idx as isize) as libc::c_int == '<' as i32
                && (macro_alternate != 0 || macro_mri != 0)
            || *((*in_0).ptr).offset(idx as isize) as libc::c_int == '\'' as i32
                && macro_alternate != 0)
    {
        if *((*in_0).ptr).offset(idx as isize) as libc::c_int == '<' as i32 {
            let mut nest: libc::c_int = 0 as libc::c_int;
            idx = idx.wrapping_add(1);
            idx;
            while idx < (*in_0).len
                && (*((*in_0).ptr).offset(idx as isize) as libc::c_int != '>' as i32
                    || nest != 0)
            {
                if *((*in_0).ptr).offset(idx as isize) as libc::c_int == '!' as i32 {
                    idx = idx.wrapping_add(1);
                    idx;
                    let fresh4 = idx;
                    idx = idx.wrapping_add(1);
                    sb_add_char(acc, *((*in_0).ptr).offset(fresh4 as isize) as size_t);
                } else {
                    if *((*in_0).ptr).offset(idx as isize) as libc::c_int == '>' as i32 {
                        nest -= 1;
                        nest;
                    }
                    if *((*in_0).ptr).offset(idx as isize) as libc::c_int == '<' as i32 {
                        nest += 1;
                        nest;
                    }
                    let fresh5 = idx;
                    idx = idx.wrapping_add(1);
                    sb_add_char(acc, *((*in_0).ptr).offset(fresh5 as isize) as size_t);
                }
            }
            idx = idx.wrapping_add(1);
            idx;
        } else if *((*in_0).ptr).offset(idx as isize) as libc::c_int == '"' as i32
            || *((*in_0).ptr).offset(idx as isize) as libc::c_int == '\'' as i32
        {
            let mut tchar: libc::c_char = *((*in_0).ptr).offset(idx as isize);
            let mut escaped: libc::c_int = 0 as libc::c_int;
            idx = idx.wrapping_add(1);
            idx;
            while idx < (*in_0).len {
                if *((*in_0).ptr)
                    .offset(idx.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                    as libc::c_int == '\\' as i32
                {
                    escaped ^= 1 as libc::c_int;
                } else {
                    escaped = 0 as libc::c_int;
                }
                if macro_alternate != 0
                    && *((*in_0).ptr).offset(idx as isize) as libc::c_int == '!' as i32
                {
                    idx = idx.wrapping_add(1);
                    idx;
                    sb_add_char(acc, *((*in_0).ptr).offset(idx as isize) as size_t);
                    idx = idx.wrapping_add(1);
                    idx;
                } else if escaped != 0
                    && *((*in_0).ptr).offset(idx as isize) as libc::c_int
                        == tchar as libc::c_int
                {
                    sb_add_char(acc, tchar as size_t);
                    idx = idx.wrapping_add(1);
                    idx;
                } else {
                    if *((*in_0).ptr).offset(idx as isize) as libc::c_int
                        == tchar as libc::c_int
                    {
                        idx = idx.wrapping_add(1);
                        idx;
                        if idx >= (*in_0).len
                            || *((*in_0).ptr).offset(idx as isize) as libc::c_int
                                != tchar as libc::c_int
                        {
                            break;
                        }
                    }
                    sb_add_char(acc, *((*in_0).ptr).offset(idx as isize) as size_t);
                    idx = idx.wrapping_add(1);
                    idx;
                }
            }
        }
    }
    return idx;
}
unsafe extern "C" fn get_any_string(
    mut idx: size_t,
    mut in_0: *mut sb,
    mut out: *mut sb,
) -> size_t {
    sb_reset(out);
    idx = sb_skip_white(idx, in_0);
    if idx < (*in_0).len {
        if (*in_0).len > idx.wrapping_add(2 as libc::c_int as libc::c_ulong)
            && *((*in_0).ptr)
                .offset(idx.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
                as libc::c_int == '\'' as i32
            && (*((*in_0).ptr).offset(idx as isize) as libc::c_int == 'b' as i32
                || *((*in_0).ptr).offset(idx as isize) as libc::c_int == 'B' as i32
                || *((*in_0).ptr).offset(idx as isize) as libc::c_int == 'q' as i32
                || *((*in_0).ptr).offset(idx as isize) as libc::c_int == 'Q' as i32
                || *((*in_0).ptr).offset(idx as isize) as libc::c_int == 'h' as i32
                || *((*in_0).ptr).offset(idx as isize) as libc::c_int == 'H' as i32
                || *((*in_0).ptr).offset(idx as isize) as libc::c_int == 'd' as i32
                || *((*in_0).ptr).offset(idx as isize) as libc::c_int == 'D' as i32)
        {
            while idx < (*in_0).len
                && !(*((*in_0).ptr).offset(idx as isize) as libc::c_int == ' ' as i32
                    || *((*in_0).ptr).offset(idx as isize) as libc::c_int == '\t' as i32
                    || *((*in_0).ptr).offset(idx as isize) as libc::c_int == ',' as i32
                    || *((*in_0).ptr).offset(idx as isize) as libc::c_int == '"' as i32
                    || *((*in_0).ptr).offset(idx as isize) as libc::c_int == ';' as i32
                    || *((*in_0).ptr).offset(idx as isize) as libc::c_int == ')' as i32
                    || *((*in_0).ptr).offset(idx as isize) as libc::c_int == '(' as i32
                    || (macro_alternate != 0 || macro_mri != 0)
                        && (*((*in_0).ptr).offset(idx as isize) as libc::c_int
                            == '<' as i32
                            || *((*in_0).ptr).offset(idx as isize) as libc::c_int
                                == '>' as i32))
            {
                let fresh6 = idx;
                idx = idx.wrapping_add(1);
                sb_add_char(out, *((*in_0).ptr).offset(fresh6 as isize) as size_t);
            }
        } else if *((*in_0).ptr).offset(idx as isize) as libc::c_int == '%' as i32
            && macro_alternate != 0
        {
            let mut val: offsetT = 0;
            let mut buf: [libc::c_char; 64] = [0; 64];
            idx = (Some(macro_expr.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const libc::c_char,
                    b"% operator needs absolute expression\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                idx.wrapping_add(1 as libc::c_int as libc::c_ulong),
                in_0,
                &mut val,
            );
            sprintf(buf.as_mut_ptr(), b"%ld\0" as *const u8 as *const libc::c_char, val);
            sb_add_string(out, buf.as_mut_ptr());
        } else if *((*in_0).ptr).offset(idx as isize) as libc::c_int == '"' as i32
            || *((*in_0).ptr).offset(idx as isize) as libc::c_int == '<' as i32
                && (macro_alternate != 0 || macro_mri != 0)
            || macro_alternate != 0
                && *((*in_0).ptr).offset(idx as isize) as libc::c_int == '\'' as i32
        {
            if macro_alternate != 0 && macro_strip_at == 0
                && *((*in_0).ptr).offset(idx as isize) as libc::c_int != '<' as i32
            {
                sb_add_char(out, '"' as i32 as size_t);
                idx = getstring(idx, in_0, out);
                sb_add_char(out, '"' as i32 as size_t);
            } else {
                idx = getstring(idx, in_0, out);
            }
        } else {
            let mut br_buf: *mut libc::c_char = xmalloc(
                (::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
                    .wrapping_mul(1 as libc::c_int as libc::c_ulong),
            ) as *mut libc::c_char;
            let mut in_br: *mut libc::c_char = br_buf;
            *in_br = '\0' as i32 as libc::c_char;
            while idx < (*in_0).len
                && (*in_br as libc::c_int != 0
                    || *((*in_0).ptr).offset(idx as isize) as libc::c_int != ' ' as i32
                        && *((*in_0).ptr).offset(idx as isize) as libc::c_int
                            != '\t' as i32)
                && *((*in_0).ptr).offset(idx as isize) as libc::c_int != ',' as i32
                && (*((*in_0).ptr).offset(idx as isize) as libc::c_int != '<' as i32
                    || macro_alternate == 0 && macro_mri == 0)
            {
                let mut tchar: libc::c_char = *((*in_0).ptr).offset(idx as isize);
                match tchar as libc::c_int {
                    34 | 39 => {
                        let fresh7 = idx;
                        idx = idx.wrapping_add(1);
                        sb_add_char(
                            out,
                            *((*in_0).ptr).offset(fresh7 as isize) as size_t,
                        );
                        while idx < (*in_0).len
                            && *((*in_0).ptr).offset(idx as isize) as libc::c_int
                                != tchar as libc::c_int
                        {
                            let fresh8 = idx;
                            idx = idx.wrapping_add(1);
                            sb_add_char(
                                out,
                                *((*in_0).ptr).offset(fresh8 as isize) as size_t,
                            );
                        }
                        if idx == (*in_0).len {
                            free(br_buf as *mut libc::c_void);
                            return idx;
                        }
                    }
                    40 | 91 => {
                        if in_br > br_buf {
                            in_br = in_br.offset(-1);
                            in_br;
                        } else {
                            br_buf = xmalloc(
                                (::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
                                    .wrapping_mul(
                                        (strlen(in_br))
                                            .wrapping_add(2 as libc::c_int as libc::c_ulong),
                                    ),
                            ) as *mut libc::c_char;
                            strcpy(br_buf.offset(1 as libc::c_int as isize), in_br);
                            free(in_br as *mut libc::c_void);
                            in_br = br_buf;
                        }
                        *in_br = tchar;
                    }
                    41 => {
                        if *in_br as libc::c_int == '(' as i32 {
                            in_br = in_br.offset(1);
                            in_br;
                        }
                    }
                    93 => {
                        if *in_br as libc::c_int == '[' as i32 {
                            in_br = in_br.offset(1);
                            in_br;
                        }
                    }
                    _ => {}
                }
                sb_add_char(out, tchar as size_t);
                idx = idx.wrapping_add(1);
                idx;
            }
            free(br_buf as *mut libc::c_void);
        }
    }
    return idx;
}
unsafe extern "C" fn new_formal() -> *mut formal_entry {
    let mut formal: *mut formal_entry = 0 as *mut formal_entry;
    formal = xmalloc(::core::mem::size_of::<formal_entry>() as libc::c_ulong)
        as *mut formal_entry;
    sb_new(&mut (*formal).name);
    sb_new(&mut (*formal).def);
    sb_new(&mut (*formal).actual);
    (*formal).next = 0 as *mut formal_struct;
    (*formal).type_0 = FORMAL_OPTIONAL;
    return formal;
}
unsafe extern "C" fn del_formal(mut formal: *mut formal_entry) {
    sb_kill(&mut (*formal).actual);
    sb_kill(&mut (*formal).def);
    sb_kill(&mut (*formal).name);
    free(formal as *mut libc::c_void);
}
unsafe extern "C" fn do_formals(
    mut macro_0: *mut macro_entry,
    mut idx: size_t,
    mut in_0: *mut sb,
) -> size_t {
    let mut p: *mut *mut formal_entry = &mut (*macro_0).formals;
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    idx = sb_skip_white(idx, in_0);
    while idx < (*in_0).len {
        let mut formal: *mut formal_entry = new_formal();
        let mut cidx: size_t = 0;
        let mut elt: *mut formal_hash_entry_t = 0 as *mut formal_hash_entry_t;
        idx = get_token(idx, in_0, &mut (*formal).name);
        if (*formal).name.len == 0 as libc::c_int as libc::c_ulong {
            if (*macro_0).formal_count != 0 {
                idx = idx.wrapping_sub(1);
                idx;
            }
            del_formal(formal);
            break;
        } else {
            idx = sb_skip_white(idx, in_0);
            name = sb_terminate(&mut (*formal).name);
            if macro_mri == 0 && idx < (*in_0).len
                && *((*in_0).ptr).offset(idx as isize) as libc::c_int == ':' as i32
                && (*lex_type.as_mut_ptr().offset(':' as i32 as libc::c_uchar as isize)
                    as libc::c_int & 2 as libc::c_int == 0
                    || idx.wrapping_add(1 as libc::c_int as libc::c_ulong) >= (*in_0).len
                    || *lex_type
                        .as_mut_ptr()
                        .offset(
                            *((*in_0).ptr)
                                .offset(
                                    idx.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                                ) as libc::c_uchar as isize,
                        ) as libc::c_int & 1 as libc::c_int == 0)
            {
                let mut qual: sb = sb {
                    ptr: 0 as *mut libc::c_char,
                    len: 0,
                    max: 0,
                };
                sb_new(&mut qual);
                idx = get_token(
                    sb_skip_white(
                        idx.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        in_0,
                    ),
                    in_0,
                    &mut qual,
                );
                sb_terminate(&mut qual);
                if qual.len == 0 as libc::c_int as libc::c_ulong {
                    as_bad_where(
                        (*macro_0).file,
                        (*macro_0).line,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Missing parameter qualifier for `%s' in macro `%s'\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        name,
                        (*macro_0).name,
                    );
                } else if strcmp(qual.ptr, b"req\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    (*formal).type_0 = FORMAL_REQUIRED;
                } else if strcmp(
                    qual.ptr,
                    b"vararg\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    (*formal).type_0 = FORMAL_VARARG;
                } else {
                    as_bad_where(
                        (*macro_0).file,
                        (*macro_0).line,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"`%s' is not a valid parameter qualifier for `%s' in macro `%s'\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        qual.ptr,
                        name,
                        (*macro_0).name,
                    );
                }
                sb_kill(&mut qual);
                idx = sb_skip_white(idx, in_0);
            }
            if idx < (*in_0).len
                && *((*in_0).ptr).offset(idx as isize) as libc::c_int == '=' as i32
            {
                idx = get_any_string(
                    idx.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    in_0,
                    &mut (*formal).def,
                );
                idx = sb_skip_white(idx, in_0);
                if (*formal).type_0 as libc::c_uint
                    == FORMAL_REQUIRED as libc::c_int as libc::c_uint
                {
                    sb_reset(&mut (*formal).def);
                    as_warn_where(
                        (*macro_0).file,
                        (*macro_0).line,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Pointless default value for required parameter `%s' in macro `%s'\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        name,
                        (*macro_0).name,
                    );
                }
            }
            elt = formal_entry_alloc(name, formal);
            if !(htab_insert(
                (*macro_0).formal_hash,
                elt as *mut libc::c_void,
                0 as libc::c_int,
            ))
                .is_null()
            {
                free(elt as *mut libc::c_void);
                as_bad_where(
                    (*macro_0).file,
                    (*macro_0).line,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"A parameter named `%s' already exists for macro `%s'\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    name,
                    (*macro_0).name,
                );
            }
            let fresh9 = (*macro_0).formal_count;
            (*macro_0).formal_count = (*macro_0).formal_count + 1;
            (*formal).index = fresh9;
            *p = formal;
            p = &mut (*formal).next;
            if (*formal).type_0 as libc::c_uint
                == FORMAL_VARARG as libc::c_int as libc::c_uint
            {
                break;
            }
            cidx = idx;
            idx = sb_skip_comma(idx, in_0);
            if !(idx != cidx && idx >= (*in_0).len) {
                continue;
            }
            idx = cidx;
            break;
        }
    }
    if macro_mri != 0 {
        let mut formal_0: *mut formal_entry = new_formal();
        let mut elt_0: *mut formal_hash_entry_t = 0 as *mut formal_hash_entry_t;
        if macro_strip_at != 0 {
            name = b"$NARG\0" as *const u8 as *const libc::c_char;
        } else {
            name = b"NARG\0" as *const u8 as *const libc::c_char;
        }
        sb_add_string(&mut (*formal_0).name, name);
        elt_0 = formal_entry_alloc(name, formal_0);
        if !(htab_insert(
            (*macro_0).formal_hash,
            elt_0 as *mut libc::c_void,
            0 as libc::c_int,
        ))
            .is_null()
        {
            free(elt_0 as *mut libc::c_void);
            as_bad_where(
                (*macro_0).file,
                (*macro_0).line,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Reserved word `%s' used as parameter in macro `%s'\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                name,
                (*macro_0).name,
            );
        }
        (*formal_0).index = -(2 as libc::c_int);
        *p = formal_0;
    }
    return idx;
}
unsafe extern "C" fn free_macro(mut macro_0: *mut macro_entry) {
    let mut formal: *mut formal_entry = 0 as *mut formal_entry;
    formal = (*macro_0).formals;
    while !formal.is_null() {
        let mut f: *mut formal_entry = 0 as *mut formal_entry;
        f = formal;
        formal = (*formal).next;
        del_formal(f);
    }
    htab_delete((*macro_0).formal_hash);
    sb_kill(&mut (*macro_0).sub);
    free(macro_0 as *mut libc::c_void);
}
unsafe extern "C" fn get_apost_token(
    mut idx: size_t,
    mut in_0: *mut sb,
    mut name: *mut sb,
    mut kind: libc::c_int,
) -> size_t {
    idx = get_token(idx, in_0, name);
    if idx < (*in_0).len && *((*in_0).ptr).offset(idx as isize) as libc::c_int == kind
        && (macro_mri == 0 || macro_strip_at != 0)
        && (macro_strip_at == 0 || kind == '@' as i32)
    {
        idx = idx.wrapping_add(1);
        idx;
    }
    return idx;
}
unsafe extern "C" fn sub_actual(
    mut start: size_t,
    mut in_0: *mut sb,
    mut t: *mut sb,
    mut formal_hash: *mut htab,
    mut kind: libc::c_int,
    mut out: *mut sb,
    mut copyifnotthere: libc::c_int,
) -> size_t {
    let mut src: size_t = 0;
    let mut ptr: *mut formal_entry = 0 as *mut formal_entry;
    src = get_apost_token(start, in_0, t, kind);
    if macro_strip_at != 0 && kind == '@' as i32
        && (src == start
            || *((*in_0).ptr)
                .offset(src.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                as libc::c_int != '@' as i32)
    {
        ptr = 0 as *mut formal_entry;
    } else {
        ptr = formal_entry_find(formal_hash, sb_terminate(t));
    }
    if !ptr.is_null() {
        if (*ptr).actual.len != 0 {
            sb_add_sb(out, &mut (*ptr).actual);
        } else {
            sb_add_sb(out, &mut (*ptr).def);
        }
    } else if kind == '&' as i32 {
        sb_add_char(out, '&' as i32 as size_t);
        sb_add_sb(out, t);
        if src != start
            && *((*in_0).ptr)
                .offset(src.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                as libc::c_int == '&' as i32
        {
            sb_add_char(out, '&' as i32 as size_t);
        }
    } else if copyifnotthere != 0 {
        sb_add_sb(out, t);
    } else {
        sb_add_char(out, '\\' as i32 as size_t);
        sb_add_sb(out, t);
    }
    return src;
}
unsafe extern "C" fn macro_expand_body(
    mut in_0: *mut sb,
    mut out: *mut sb,
    mut formals: *mut formal_entry,
    mut formal_hash: *mut htab,
    mut macro_0: *const macro_entry,
) -> *const libc::c_char {
    let mut t: sb = sb {
        ptr: 0 as *mut libc::c_char,
        len: 0,
        max: 0,
    };
    let mut src: size_t = 0 as libc::c_int as size_t;
    let mut inquote: libc::c_int = 0 as libc::c_int;
    let mut macro_line: libc::c_int = 0 as libc::c_int;
    let mut loclist: *mut formal_entry = 0 as *mut formal_entry;
    let mut err: *const libc::c_char = 0 as *const libc::c_char;
    sb_new(&mut t);
    while src < (*in_0).len && err.is_null() {
        if *((*in_0).ptr).offset(src as isize) as libc::c_int == '&' as i32 {
            sb_reset(&mut t);
            if macro_mri != 0 {
                if src.wrapping_add(1 as libc::c_int as libc::c_ulong) < (*in_0).len
                    && *((*in_0).ptr)
                        .offset(
                            src.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        ) as libc::c_int == '&' as i32
                {
                    src = sub_actual(
                        src.wrapping_add(2 as libc::c_int as libc::c_ulong),
                        in_0,
                        &mut t,
                        formal_hash,
                        '\'' as i32,
                        out,
                        1 as libc::c_int,
                    );
                } else {
                    let fresh10 = src;
                    src = src.wrapping_add(1);
                    sb_add_char(out, *((*in_0).ptr).offset(fresh10 as isize) as size_t);
                }
            } else {
                src = sub_actual(
                    src.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    in_0,
                    &mut t,
                    formal_hash,
                    '&' as i32,
                    out,
                    0 as libc::c_int,
                );
            }
        } else if *((*in_0).ptr).offset(src as isize) as libc::c_int == '\\' as i32 {
            src = src.wrapping_add(1);
            src;
            if src < (*in_0).len
                && *((*in_0).ptr).offset(src as isize) as libc::c_int == '(' as i32
            {
                src = src.wrapping_add(1);
                src;
                while src < (*in_0).len
                    && *((*in_0).ptr).offset(src as isize) as libc::c_int != ')' as i32
                {
                    let fresh11 = src;
                    src = src.wrapping_add(1);
                    sb_add_char(out, *((*in_0).ptr).offset(fresh11 as isize) as size_t);
                }
                if src < (*in_0).len {
                    src = src.wrapping_add(1);
                    src;
                } else if macro_0.is_null() {
                    err = dcgettext(
                        0 as *const libc::c_char,
                        b"missing `)'\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    );
                } else {
                    as_bad_where(
                        (*macro_0).file,
                        ((*macro_0).line).wrapping_add(macro_line as libc::c_uint),
                        dcgettext(
                            0 as *const libc::c_char,
                            b"missing `)'\0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                }
            } else if src < (*in_0).len
                && *((*in_0).ptr).offset(src as isize) as libc::c_int == '@' as i32
            {
                let mut buffer: [libc::c_char; 12] = [0; 12];
                src = src.wrapping_add(1);
                src;
                sprintf(
                    buffer.as_mut_ptr(),
                    b"%d\0" as *const u8 as *const libc::c_char,
                    macro_number,
                );
                sb_add_string(out, buffer.as_mut_ptr());
            } else if src < (*in_0).len
                && *((*in_0).ptr).offset(src as isize) as libc::c_int == '&' as i32
            {
                sb_add_char(out, '\\' as i32 as size_t);
                sb_add_char(out, '&' as i32 as size_t);
                src = src.wrapping_add(1);
                src;
            } else if macro_mri != 0 && src < (*in_0).len
                && _sch_istable[(*((*in_0).ptr).offset(src as isize) as libc::c_int
                    & 0xff as libc::c_int) as usize] as libc::c_int
                    & _sch_isalnum as libc::c_int as libc::c_ushort as libc::c_int != 0
            {
                let mut ind: libc::c_int = 0;
                let mut f: *mut formal_entry = 0 as *mut formal_entry;
                if _sch_istable[(*((*in_0).ptr).offset(src as isize) as libc::c_int
                    & 0xff as libc::c_int) as usize] as libc::c_int
                    & _sch_isdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
                {
                    ind = *((*in_0).ptr).offset(src as isize) as libc::c_int
                        - '0' as i32;
                } else if _sch_istable[(*((*in_0).ptr).offset(src as isize)
                    as libc::c_int & 0xff as libc::c_int) as usize] as libc::c_int
                    & _sch_isupper as libc::c_int as libc::c_ushort as libc::c_int != 0
                {
                    ind = *((*in_0).ptr).offset(src as isize) as libc::c_int - 'A' as i32
                        + 10 as libc::c_int;
                } else {
                    ind = *((*in_0).ptr).offset(src as isize) as libc::c_int - 'a' as i32
                        + 10 as libc::c_int;
                }
                src = src.wrapping_add(1);
                src;
                f = formals;
                while !f.is_null() {
                    if (*f).index == ind - 1 as libc::c_int {
                        if (*f).actual.len != 0 as libc::c_int as libc::c_ulong {
                            sb_add_sb(out, &mut (*f).actual);
                        } else {
                            sb_add_sb(out, &mut (*f).def);
                        }
                        break;
                    } else {
                        f = (*f).next;
                    }
                }
            } else {
                sb_reset(&mut t);
                src = sub_actual(
                    src,
                    in_0,
                    &mut t,
                    formal_hash,
                    '\'' as i32,
                    out,
                    0 as libc::c_int,
                );
            }
        } else if (macro_alternate != 0 || macro_mri != 0)
            && *lex_type
                .as_mut_ptr()
                .offset(*((*in_0).ptr).offset(src as isize) as libc::c_uchar as isize)
                as libc::c_int & 2 as libc::c_int != 0
            && (inquote == 0 || macro_strip_at == 0
                || src > 0 as libc::c_int as libc::c_ulong
                    && *((*in_0).ptr)
                        .offset(
                            src.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                        ) as libc::c_int == '@' as i32)
        {
            if macro_0.is_null()
                || src.wrapping_add(5 as libc::c_int as libc::c_ulong) >= (*in_0).len
                || strncasecmp(
                    ((*in_0).ptr).offset(src as isize),
                    b"LOCAL\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int as libc::c_ulong,
                ) != 0 as libc::c_int
                || !(*((*in_0).ptr)
                    .offset(src.wrapping_add(5 as libc::c_int as libc::c_ulong) as isize)
                    as libc::c_int == ' ' as i32
                    || *((*in_0).ptr)
                        .offset(
                            src.wrapping_add(5 as libc::c_int as libc::c_ulong) as isize,
                        ) as libc::c_int == '\t' as i32) || inquote != 0
            {
                sb_reset(&mut t);
                src = sub_actual(
                    src,
                    in_0,
                    &mut t,
                    formal_hash,
                    if macro_strip_at != 0 && inquote != 0 {
                        '@' as i32
                    } else {
                        '\'' as i32
                    },
                    out,
                    1 as libc::c_int,
                );
            } else {
                src = sb_skip_white(
                    src.wrapping_add(5 as libc::c_int as libc::c_ulong),
                    in_0,
                );
                while *((*in_0).ptr).offset(src as isize) as libc::c_int != '\n' as i32 {
                    let mut name: *const libc::c_char = 0 as *const libc::c_char;
                    let mut f_0: *mut formal_entry = new_formal();
                    let mut elt: *mut formal_hash_entry_t = 0
                        as *mut formal_hash_entry_t;
                    src = get_token(src, in_0, &mut (*f_0).name);
                    name = sb_terminate(&mut (*f_0).name);
                    elt = formal_entry_alloc(name, f_0);
                    if !(htab_insert(
                        formal_hash,
                        elt as *mut libc::c_void,
                        0 as libc::c_int,
                    ))
                        .is_null()
                    {
                        free(elt as *mut libc::c_void);
                        as_bad_where(
                            (*macro_0).file,
                            ((*macro_0).line).wrapping_add(macro_line as libc::c_uint),
                            dcgettext(
                                0 as *const libc::c_char,
                                b"`%s' was already used as parameter (or another local) name\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            name,
                        );
                        del_formal(f_0);
                    } else {
                        static mut loccnt: libc::c_int = 0;
                        let mut buf: [libc::c_char; 20] = [0; 20];
                        (*f_0).index = -(3 as libc::c_int);
                        (*f_0).next = loclist;
                        loclist = f_0;
                        loccnt += 1;
                        sprintf(
                            buf.as_mut_ptr(),
                            if 1 as libc::c_int != 0 {
                                b".LL%04x\0" as *const u8 as *const libc::c_char
                            } else {
                                b"LL%04x\0" as *const u8 as *const libc::c_char
                            },
                            loccnt,
                        );
                        sb_add_string(&mut (*f_0).actual, buf.as_mut_ptr());
                    }
                    src = sb_skip_comma(src, in_0);
                }
            }
        } else if *((*in_0).ptr).offset(src as isize) as libc::c_int == '"' as i32
            || macro_mri != 0
                && *((*in_0).ptr).offset(src as isize) as libc::c_int == '\'' as i32
        {
            inquote = (inquote == 0) as libc::c_int;
            let fresh12 = src;
            src = src.wrapping_add(1);
            sb_add_char(out, *((*in_0).ptr).offset(fresh12 as isize) as size_t);
        } else if *((*in_0).ptr).offset(src as isize) as libc::c_int == '@' as i32
            && macro_strip_at != 0
        {
            src = src.wrapping_add(1);
            src;
            if src < (*in_0).len
                && *((*in_0).ptr).offset(src as isize) as libc::c_int == '@' as i32
            {
                sb_add_char(out, '@' as i32 as size_t);
                src = src.wrapping_add(1);
                src;
            }
        } else if macro_mri != 0
            && *((*in_0).ptr).offset(src as isize) as libc::c_int == '=' as i32
            && src.wrapping_add(1 as libc::c_int as libc::c_ulong) < (*in_0).len
            && *((*in_0).ptr)
                .offset(src.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
                as libc::c_int == '=' as i32
        {
            let mut ptr: *mut formal_entry = 0 as *mut formal_entry;
            sb_reset(&mut t);
            src = get_token(
                src.wrapping_add(2 as libc::c_int as libc::c_ulong),
                in_0,
                &mut t,
            );
            ptr = formal_entry_find(formal_hash, sb_terminate(&mut t));
            if ptr.is_null() {
                sb_add_char(out, '=' as i32 as size_t);
                sb_add_char(out, '=' as i32 as size_t);
                sb_add_sb(out, &mut t);
            } else if (*ptr).actual.len != 0 {
                sb_add_string(out, b"-1\0" as *const u8 as *const libc::c_char);
            } else {
                sb_add_char(out, '0' as i32 as size_t);
            }
        } else {
            if *((*in_0).ptr).offset(src as isize) as libc::c_int == '\n' as i32 {
                macro_line += 1;
                macro_line;
            }
            let fresh13 = src;
            src = src.wrapping_add(1);
            sb_add_char(out, *((*in_0).ptr).offset(fresh13 as isize) as size_t);
        }
    }
    sb_kill(&mut t);
    while !loclist.is_null() {
        let mut f_1: *mut formal_entry = 0 as *mut formal_entry;
        let mut name_0: *const libc::c_char = 0 as *const libc::c_char;
        f_1 = (*loclist).next;
        name_0 = sb_terminate(&mut (*loclist).name);
        let mut needle: formal_hash_entry_t = {
            let mut init = formal_hash_entry {
                name: name_0,
                formal: 0 as *mut formal_entry,
            };
            init
        };
        htab_remove_elt(
            formal_hash,
            &mut needle as *mut formal_hash_entry_t as *const libc::c_void,
        );
        del_formal(loclist);
        loclist = f_1;
    }
    return err;
}
unsafe extern "C" fn macro_expand(
    mut idx: size_t,
    mut in_0: *mut sb,
    mut m: *mut macro_entry,
    mut out: *mut sb,
) -> *const libc::c_char {
    let mut t: sb = sb {
        ptr: 0 as *mut libc::c_char,
        len: 0,
        max: 0,
    };
    let mut ptr: *mut formal_entry = 0 as *mut formal_entry;
    let mut f: *mut formal_entry = 0 as *mut formal_entry;
    let mut is_keyword: libc::c_int = 0 as libc::c_int;
    let mut narg: libc::c_int = 0 as libc::c_int;
    let mut err: *const libc::c_char = 0 as *const libc::c_char;
    sb_new(&mut t);
    f = (*m).formals;
    while !f.is_null() {
        sb_reset(&mut (*f).actual);
        f = (*f).next;
    }
    f = (*m).formals;
    while !f.is_null() && (*f).index < 0 as libc::c_int {
        f = (*f).next;
    }
    if macro_mri != 0 {
        if idx < (*in_0).len
            && *((*in_0).ptr).offset(idx as isize) as libc::c_int == '.' as i32
        {
            idx = idx.wrapping_add(1);
            idx;
            if idx < (*in_0).len
                && *((*in_0).ptr).offset(idx as isize) as libc::c_int != ' ' as i32
                && *((*in_0).ptr).offset(idx as isize) as libc::c_int != '\t' as i32
            {
                let mut n: *mut formal_entry = new_formal();
                (*n).index = -(1 as libc::c_int);
                (*n).next = (*m).formals;
                (*m).formals = n;
                idx = get_any_string(idx, in_0, &mut (*n).actual);
            }
        }
    }
    idx = sb_skip_white(idx, in_0);
    while idx < (*in_0).len {
        let mut scan: size_t = 0;
        scan = idx;
        while scan < (*in_0).len
            && !(*((*in_0).ptr).offset(scan as isize) as libc::c_int == ' ' as i32
                || *((*in_0).ptr).offset(scan as isize) as libc::c_int == '\t' as i32
                || *((*in_0).ptr).offset(scan as isize) as libc::c_int == ',' as i32
                || *((*in_0).ptr).offset(scan as isize) as libc::c_int == '"' as i32
                || *((*in_0).ptr).offset(scan as isize) as libc::c_int == ';' as i32
                || *((*in_0).ptr).offset(scan as isize) as libc::c_int == ')' as i32
                || *((*in_0).ptr).offset(scan as isize) as libc::c_int == '(' as i32
                || (macro_alternate != 0 || macro_mri != 0)
                    && (*((*in_0).ptr).offset(scan as isize) as libc::c_int == '<' as i32
                        || *((*in_0).ptr).offset(scan as isize) as libc::c_int
                            == '>' as i32))
            && !(macro_mri != 0
                && *((*in_0).ptr).offset(scan as isize) as libc::c_int == '\'' as i32)
            && (macro_alternate == 0
                && *((*in_0).ptr).offset(scan as isize) as libc::c_int != '=' as i32)
        {
            scan = scan.wrapping_add(1);
            scan;
        }
        if scan < (*in_0).len && macro_alternate == 0
            && *((*in_0).ptr).offset(scan as isize) as libc::c_int == '=' as i32
        {
            is_keyword = 1 as libc::c_int;
            sb_reset(&mut t);
            idx = get_token(idx, in_0, &mut t);
            if *((*in_0).ptr).offset(idx as isize) as libc::c_int != '=' as i32 {
                err = dcgettext(
                    0 as *const libc::c_char,
                    b"confusion in formal parameters\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                );
                break;
            } else {
                ptr = formal_entry_find((*m).formal_hash, sb_terminate(&mut t));
                if ptr.is_null() {
                    as_bad(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Parameter named `%s' does not exist for macro `%s'\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        t.ptr,
                        (*m).name,
                    );
                    sb_reset(&mut t);
                    idx = get_any_string(
                        idx.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        in_0,
                        &mut t,
                    );
                } else {
                    if (*ptr).actual.len != 0 {
                        as_warn(
                            dcgettext(
                                0 as *const libc::c_char,
                                b"Value for parameter `%s' of macro `%s' was already specified\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            (*ptr).name.ptr,
                            (*m).name,
                        );
                        sb_reset(&mut (*ptr).actual);
                    }
                    idx = get_any_string(
                        idx.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        in_0,
                        &mut (*ptr).actual,
                    );
                    if (*ptr).actual.len > 0 as libc::c_int as libc::c_ulong {
                        narg += 1;
                        narg;
                    }
                }
            }
        } else if is_keyword != 0 {
            err = dcgettext(
                0 as *const libc::c_char,
                b"can't mix positional and keyword arguments\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            );
            break;
        } else {
            if f.is_null() {
                let mut pf: *mut *mut formal_entry = 0 as *mut *mut formal_entry;
                let mut c: libc::c_int = 0;
                if macro_mri == 0 {
                    err = dcgettext(
                        0 as *const libc::c_char,
                        b"too many positional arguments\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    );
                    break;
                } else {
                    f = new_formal();
                    c = -(1 as libc::c_int);
                    pf = &mut (*m).formals;
                    while !(*pf).is_null() {
                        if (**pf).index >= c {
                            c = (**pf).index + 1 as libc::c_int;
                        }
                        pf = &mut (**pf).next;
                    }
                    if c == -(1 as libc::c_int) {
                        c = 0 as libc::c_int;
                    }
                    *pf = f;
                    (*f).index = c;
                }
            }
            if (*f).type_0 as libc::c_uint
                != FORMAL_VARARG as libc::c_int as libc::c_uint
            {
                idx = get_any_string(idx, in_0, &mut (*f).actual);
            } else {
                sb_add_buffer(
                    &mut (*f).actual,
                    ((*in_0).ptr).offset(idx as isize),
                    ((*in_0).len).wrapping_sub(idx),
                );
                idx = (*in_0).len;
            }
            if (*f).actual.len > 0 as libc::c_int as libc::c_ulong {
                narg += 1;
                narg;
            }
            loop {
                f = (*f).next;
                if !(!f.is_null() && (*f).index < 0 as libc::c_int) {
                    break;
                }
            }
        }
        if macro_mri == 0 {
            idx = sb_skip_comma(idx, in_0);
        } else {
            if *((*in_0).ptr).offset(idx as isize) as libc::c_int == ',' as i32 {
                idx = idx.wrapping_add(1);
                idx;
            }
            if *((*in_0).ptr).offset(idx as isize) as libc::c_int == ' ' as i32
                || *((*in_0).ptr).offset(idx as isize) as libc::c_int == '\t' as i32
            {
                break;
            }
        }
    }
    if err.is_null() {
        ptr = (*m).formals;
        while !ptr.is_null() {
            if (*ptr).type_0 as libc::c_uint
                == FORMAL_REQUIRED as libc::c_int as libc::c_uint
                && (*ptr).actual.len == 0 as libc::c_int as libc::c_ulong
            {
                as_bad(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Missing value for required parameter `%s' of macro `%s'\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    (*ptr).name.ptr,
                    (*m).name,
                );
            }
            ptr = (*ptr).next;
        }
        if macro_mri != 0 {
            let mut buffer: [libc::c_char; 20] = [0; 20];
            sb_reset(&mut t);
            sb_add_string(
                &mut t,
                if macro_strip_at != 0 {
                    b"$NARG\0" as *const u8 as *const libc::c_char
                } else {
                    b"NARG\0" as *const u8 as *const libc::c_char
                },
            );
            ptr = formal_entry_find((*m).formal_hash, sb_terminate(&mut t));
            sprintf(
                buffer.as_mut_ptr(),
                b"%d\0" as *const u8 as *const libc::c_char,
                narg,
            );
            sb_add_string(&mut (*ptr).actual, buffer.as_mut_ptr());
        }
        err = macro_expand_body(&mut (*m).sub, out, (*m).formals, (*m).formal_hash, m);
    }
    if macro_mri != 0 {
        let mut pf_0: *mut *mut formal_entry = 0 as *mut *mut formal_entry;
        pf_0 = &mut (*m).formals;
        while !(*pf_0).is_null() {
            if (**pf_0).name.len != 0 as libc::c_int as libc::c_ulong {
                pf_0 = &mut (**pf_0).next;
            } else {
                f = (**pf_0).next;
                del_formal(*pf_0);
                *pf_0 = f;
            }
        }
    }
    sb_kill(&mut t);
    if err.is_null() {
        macro_number += 1;
        macro_number;
    }
    return err;
}
