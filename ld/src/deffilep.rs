extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fgetc(__stream: *mut FILE) -> libc::c_int;
    fn ungetc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
    fn strtoul(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_ulong;
    fn strtoull(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_ulonglong;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
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
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn lbasename(_: *const libc::c_char) -> *const libc::c_char;
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    fn xrealloc(_: *mut libc::c_void, _: size_t) -> *mut libc::c_void;
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
    static _sch_istable: [libc::c_ushort; 256];
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn einfo(_: *const libc::c_char, _: ...);
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
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
pub type bfd_vma = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct def_file_section {
    pub name: *mut libc::c_char,
    pub class: *mut libc::c_char,
    pub flag_read: libc::c_char,
    pub flag_write: libc::c_char,
    pub flag_execute: libc::c_char,
    pub flag_shared: libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct def_file_export {
    pub name: *mut libc::c_char,
    pub internal_name: *mut libc::c_char,
    pub its_name: *mut libc::c_char,
    pub ordinal: libc::c_int,
    pub hint: libc::c_int,
    pub flag_private: libc::c_char,
    pub flag_constant: libc::c_char,
    pub flag_noname: libc::c_char,
    pub flag_data: libc::c_char,
    pub flag_forward: libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct def_file_module {
    pub next: *mut def_file_module,
    pub user_data: *mut libc::c_void,
    pub name: [libc::c_char; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct def_file_import {
    pub internal_name: *mut libc::c_char,
    pub module: *mut def_file_module,
    pub name: *mut libc::c_char,
    pub its_name: *mut libc::c_char,
    pub ordinal: libc::c_int,
    pub data: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct def_file_aligncomm {
    pub next: *mut def_file_aligncomm,
    pub symbol_name: *mut libc::c_char,
    pub alignment: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct def_file {
    pub name: *mut libc::c_char,
    pub is_dll: libc::c_int,
    pub base_address: bfd_vma,
    pub description: *mut libc::c_char,
    pub stack_reserve: libc::c_int,
    pub stack_commit: libc::c_int,
    pub heap_reserve: libc::c_int,
    pub heap_commit: libc::c_int,
    pub num_section_defs: libc::c_int,
    pub section_defs: *mut def_file_section,
    pub num_exports: libc::c_int,
    pub exports: *mut def_file_export,
    pub modules: *mut def_file_module,
    pub num_imports: libc::c_int,
    pub imports: *mut def_file_import,
    pub version_major: libc::c_int,
    pub version_minor: libc::c_int,
    pub aligncomms: *mut def_file_aligncomm,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct def_pool_str {
    pub next: *mut def_pool_str,
    pub data: [libc::c_char; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct directive {
    pub next: *mut directive,
    pub name: *mut libc::c_char,
    pub len: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub param: *mut libc::c_char,
    pub token: libc::c_int,
}
pub type yy_state_t = yytype_uint8;
pub type yytype_uint8 = libc::c_uchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub union YYSTYPE {
    pub id: *mut libc::c_char,
    pub id_const: *const libc::c_char,
    pub number: libc::c_int,
    pub vma: bfd_vma,
    pub digits: *mut libc::c_char,
}
pub type yytype_int8 = libc::c_schar;
pub type yy_state_fast_t = libc::c_int;
pub type yytype_int16 = libc::c_short;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub name: *mut libc::c_char,
    pub token: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union yyalloc {
    pub yyss_alloc: yy_state_t,
    pub yyvs_alloc: YYSTYPE,
}
#[no_mangle]
pub unsafe extern "C" fn def_file_empty() -> *mut def_file {
    let mut rv: *mut def_file = xmalloc(
        ::core::mem::size_of::<def_file>() as libc::c_ulong,
    ) as *mut def_file;
    memset(
        rv as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<def_file>() as libc::c_ulong,
    );
    (*rv).is_dll = -(1 as libc::c_int);
    (*rv).base_address = -(1 as libc::c_int) as bfd_vma;
    (*rv).stack_commit = -(1 as libc::c_int);
    (*rv).stack_reserve = (*rv).stack_commit;
    (*rv).heap_commit = -(1 as libc::c_int);
    (*rv).heap_reserve = (*rv).heap_commit;
    (*rv).version_minor = -(1 as libc::c_int);
    (*rv).version_major = (*rv).version_minor;
    return rv;
}
#[no_mangle]
pub unsafe extern "C" fn def_file_parse(
    mut filename: *const libc::c_char,
    mut add_to: *mut def_file,
) -> *mut def_file {
    let mut d: *mut directive = 0 as *mut directive;
    the_file = fopen(filename, b"r\0" as *const u8 as *const libc::c_char);
    def_filename = filename;
    linenumber = 1 as libc::c_int;
    if the_file.is_null() {
        perror(filename);
        return 0 as *mut def_file;
    }
    if !add_to.is_null() {
        def = add_to;
    } else {
        def = def_file_empty();
    }
    saw_newline = 1 as libc::c_int;
    if def_parse() != 0 {
        def_file_free(def);
        fclose(the_file);
        def_pool_free();
        return 0 as *mut def_file;
    }
    fclose(the_file);
    loop {
        d = directives;
        if d.is_null() {
            break;
        }
        def_file_add_directive(def, (*d).name, (*d).len);
        directives = (*d).next;
        free((*d).name as *mut libc::c_void);
        free(d as *mut libc::c_void);
    }
    def_pool_free();
    return def;
}
#[no_mangle]
pub unsafe extern "C" fn def_file_free(mut fdef: *mut def_file) {
    let mut i: libc::c_int = 0;
    if fdef.is_null() {
        return;
    }
    free((*fdef).name as *mut libc::c_void);
    free((*fdef).description as *mut libc::c_void);
    if !((*fdef).section_defs).is_null() {
        i = 0 as libc::c_int;
        while i < (*fdef).num_section_defs {
            free((*((*fdef).section_defs).offset(i as isize)).name as *mut libc::c_void);
            free(
                (*((*fdef).section_defs).offset(i as isize)).class as *mut libc::c_void,
            );
            i += 1;
            i;
        }
        free((*fdef).section_defs as *mut libc::c_void);
    }
    if !((*fdef).exports).is_null() {
        i = 0 as libc::c_int;
        while i < (*fdef).num_exports {
            if (*((*fdef).exports).offset(i as isize)).internal_name
                != (*((*fdef).exports).offset(i as isize)).name
            {
                free(
                    (*((*fdef).exports).offset(i as isize)).internal_name
                        as *mut libc::c_void,
                );
            }
            free((*((*fdef).exports).offset(i as isize)).name as *mut libc::c_void);
            free((*((*fdef).exports).offset(i as isize)).its_name as *mut libc::c_void);
            i += 1;
            i;
        }
        free((*fdef).exports as *mut libc::c_void);
    }
    if !((*fdef).imports).is_null() {
        i = 0 as libc::c_int;
        while i < (*fdef).num_imports {
            if (*((*fdef).imports).offset(i as isize)).internal_name
                != (*((*fdef).imports).offset(i as isize)).name
            {
                free(
                    (*((*fdef).imports).offset(i as isize)).internal_name
                        as *mut libc::c_void,
                );
            }
            free((*((*fdef).imports).offset(i as isize)).name as *mut libc::c_void);
            free((*((*fdef).imports).offset(i as isize)).its_name as *mut libc::c_void);
            i += 1;
            i;
        }
        free((*fdef).imports as *mut libc::c_void);
    }
    while !((*fdef).modules).is_null() {
        let mut m: *mut def_file_module = (*fdef).modules;
        (*fdef).modules = (*(*fdef).modules).next;
        free(m as *mut libc::c_void);
    }
    while !((*fdef).aligncomms).is_null() {
        let mut c: *mut def_file_aligncomm = (*fdef).aligncomms;
        (*fdef).aligncomms = (*(*fdef).aligncomms).next;
        free((*c).symbol_name as *mut libc::c_void);
        free(c as *mut libc::c_void);
    }
    free(fdef as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn def_file_add_export(
    mut fdef: *mut def_file,
    mut external_name: *const libc::c_char,
    mut internal_name: *const libc::c_char,
    mut ordinal: libc::c_int,
    mut its_name: *const libc::c_char,
    mut is_dup: *mut libc::c_int,
) -> *mut def_file_export {
    let mut e: *mut def_file_export = 0 as *mut def_file_export;
    let mut pos: libc::c_int = 0;
    let mut max_exports: libc::c_int = (*fdef).num_exports
        + (32 as libc::c_int - 1 as libc::c_int)
        & !(32 as libc::c_int - 1 as libc::c_int);
    if !internal_name.is_null() && external_name.is_null() {
        external_name = internal_name;
    }
    if !external_name.is_null() && internal_name.is_null() {
        internal_name = external_name;
    }
    *is_dup = 0 as libc::c_int;
    pos = find_export_in_list(
        (*fdef).exports,
        (*fdef).num_exports,
        external_name,
        internal_name,
        its_name,
        ordinal,
        is_dup,
    );
    if *is_dup != 0 as libc::c_int {
        return ((*fdef).exports).offset(pos as isize);
    }
    if (*fdef).num_exports >= max_exports {
        max_exports = (*fdef).num_exports + 1 as libc::c_int
            + (32 as libc::c_int - 1 as libc::c_int)
            & !(32 as libc::c_int - 1 as libc::c_int);
        if !((*fdef).exports).is_null() {
            (*fdef)
                .exports = xrealloc(
                (*fdef).exports as *mut libc::c_void,
                (max_exports as libc::c_ulong)
                    .wrapping_mul(
                        ::core::mem::size_of::<def_file_export>() as libc::c_ulong,
                    ),
            ) as *mut def_file_export;
        } else {
            (*fdef)
                .exports = xmalloc(
                (max_exports as libc::c_ulong)
                    .wrapping_mul(
                        ::core::mem::size_of::<def_file_export>() as libc::c_ulong,
                    ),
            ) as *mut def_file_export;
        }
    }
    e = ((*fdef).exports).offset(pos as isize);
    if pos != (*fdef).num_exports {
        memmove(
            &mut *e.offset(1 as libc::c_int as isize) as *mut def_file_export
                as *mut libc::c_void,
            e as *const libc::c_void,
            (::core::mem::size_of::<def_file_export>() as libc::c_ulong)
                .wrapping_mul(((*fdef).num_exports - pos) as libc::c_ulong),
        );
    }
    memset(
        e as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<def_file_export>() as libc::c_ulong,
    );
    (*e).name = xstrdup(external_name);
    (*e).internal_name = xstrdup(internal_name);
    (*e)
        .its_name = if !its_name.is_null() {
        xstrdup(its_name)
    } else {
        0 as *mut libc::c_char
    };
    (*e).ordinal = ordinal;
    (*fdef).num_exports += 1;
    (*fdef).num_exports;
    return e;
}
#[no_mangle]
pub unsafe extern "C" fn def_file_add_import(
    mut fdef: *mut def_file,
    mut name: *const libc::c_char,
    mut module: *const libc::c_char,
    mut ordinal: libc::c_int,
    mut internal_name: *const libc::c_char,
    mut its_name: *const libc::c_char,
    mut is_dup: *mut libc::c_int,
) -> *mut def_file_import {
    let mut i: *mut def_file_import = 0 as *mut def_file_import;
    let mut pos: libc::c_int = 0;
    let mut max_imports: libc::c_int = (*fdef).num_imports
        + (16 as libc::c_int - 1 as libc::c_int)
        & !(16 as libc::c_int - 1 as libc::c_int);
    *is_dup = 0 as libc::c_int;
    pos = find_import_in_list(
        (*fdef).imports,
        (*fdef).num_imports,
        name,
        if internal_name.is_null() { name } else { internal_name },
        module,
        ordinal,
        is_dup,
    );
    if *is_dup != 0 as libc::c_int {
        return ((*fdef).imports).offset(pos as isize);
    }
    if (*fdef).num_imports >= max_imports {
        max_imports = (*fdef).num_imports + 1 as libc::c_int
            + (16 as libc::c_int - 1 as libc::c_int)
            & !(16 as libc::c_int - 1 as libc::c_int);
        if !((*fdef).imports).is_null() {
            (*fdef)
                .imports = xrealloc(
                (*fdef).imports as *mut libc::c_void,
                (max_imports as libc::c_ulong)
                    .wrapping_mul(
                        ::core::mem::size_of::<def_file_import>() as libc::c_ulong,
                    ),
            ) as *mut def_file_import;
        } else {
            (*fdef)
                .imports = xmalloc(
                (max_imports as libc::c_ulong)
                    .wrapping_mul(
                        ::core::mem::size_of::<def_file_import>() as libc::c_ulong,
                    ),
            ) as *mut def_file_import;
        }
    }
    i = ((*fdef).imports).offset(pos as isize);
    if pos != (*fdef).num_imports {
        memmove(
            i.offset(1 as libc::c_int as isize) as *mut libc::c_void,
            i as *const libc::c_void,
            (::core::mem::size_of::<def_file_import>() as libc::c_ulong)
                .wrapping_mul(((*fdef).num_imports - pos) as libc::c_ulong),
        );
    }
    fill_in_import(
        i,
        name,
        def_stash_module(fdef, module),
        ordinal,
        internal_name,
        its_name,
    );
    (*fdef).num_imports += 1;
    (*fdef).num_imports;
    return i;
}
#[no_mangle]
pub unsafe extern "C" fn def_file_add_import_from(
    mut fdef: *mut def_file,
    mut num_imports: libc::c_int,
    mut name: *const libc::c_char,
    mut module: *const libc::c_char,
    mut ordinal: libc::c_int,
    mut internal_name: *const libc::c_char,
    mut _its_name: *const libc::c_char,
) -> libc::c_int {
    let mut i: *mut def_file_import = 0 as *mut def_file_import;
    let mut is_dup: libc::c_int = 0;
    let mut pos: libc::c_int = 0;
    let mut max_imports: libc::c_int = (*fdef).num_imports
        + (16 as libc::c_int - 1 as libc::c_int)
        & !(16 as libc::c_int - 1 as libc::c_int);
    is_dup = 0 as libc::c_int;
    pos = find_import_in_list(
        (*fdef).imports,
        (*fdef).num_imports,
        name,
        if !internal_name.is_null() { internal_name } else { name },
        module,
        ordinal,
        &mut is_dup,
    );
    if is_dup != 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if !((*fdef).imports).is_null() && pos != (*fdef).num_imports {
        i = ((*fdef).imports).offset(pos as isize);
        if !((*i).module).is_null()
            && strcmp(((*(*i).module).name).as_mut_ptr(), module) == 0 as libc::c_int
        {
            return -(1 as libc::c_int);
        }
    }
    if (*fdef).num_imports + num_imports - 1 as libc::c_int >= max_imports {
        max_imports = (*fdef).num_imports + num_imports
            + (16 as libc::c_int - 1 as libc::c_int)
            & !(16 as libc::c_int - 1 as libc::c_int);
        if !((*fdef).imports).is_null() {
            (*fdef)
                .imports = xrealloc(
                (*fdef).imports as *mut libc::c_void,
                (max_imports as libc::c_ulong)
                    .wrapping_mul(
                        ::core::mem::size_of::<def_file_import>() as libc::c_ulong,
                    ),
            ) as *mut def_file_import;
        } else {
            (*fdef)
                .imports = xmalloc(
                (max_imports as libc::c_ulong)
                    .wrapping_mul(
                        ::core::mem::size_of::<def_file_import>() as libc::c_ulong,
                    ),
            ) as *mut def_file_import;
        }
    }
    i = ((*fdef).imports).offset(pos as isize);
    if pos != (*fdef).num_imports {
        memmove(
            i.offset(num_imports as isize) as *mut libc::c_void,
            i as *const libc::c_void,
            (::core::mem::size_of::<def_file_import>() as libc::c_ulong)
                .wrapping_mul(((*fdef).num_imports - pos) as libc::c_ulong),
        );
    }
    return pos;
}
#[no_mangle]
pub unsafe extern "C" fn def_file_add_import_at(
    mut fdef: *mut def_file,
    mut pos: libc::c_int,
    mut name: *const libc::c_char,
    mut module: *const libc::c_char,
    mut ordinal: libc::c_int,
    mut internal_name: *const libc::c_char,
    mut its_name: *const libc::c_char,
) -> *mut def_file_import {
    let mut i: *mut def_file_import = ((*fdef).imports).offset(pos as isize);
    fill_in_import(
        i,
        name,
        def_stash_module(fdef, module),
        ordinal,
        internal_name,
        its_name,
    );
    (*fdef).num_imports += 1;
    (*fdef).num_imports;
    return i;
}
#[no_mangle]
pub unsafe extern "C" fn def_file_add_directive(
    mut my_def: *mut def_file,
    mut param: *const libc::c_char,
    mut len: libc::c_int,
) {
    let mut save_def: *mut def_file = def;
    let mut pend: *const libc::c_char = param.offset(len as isize);
    let mut tend: *mut libc::c_char = param as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    def = my_def;
    while param < pend {
        while param < pend
            && (_sch_istable[(*param as libc::c_int & 0xff as libc::c_int) as usize]
                as libc::c_int
                & _sch_isspace as libc::c_int as libc::c_ushort as libc::c_int != 0
                || *param as libc::c_int == '\n' as i32
                || *param as libc::c_int == 0 as libc::c_int)
        {
            param = param.offset(1);
            param;
        }
        if param == pend {
            break;
        }
        tend = param.offset(1 as libc::c_int as isize) as *mut libc::c_char;
        while tend < pend as *mut libc::c_char
            && !(_sch_istable[(*tend.offset(-(1 as libc::c_int) as isize) as libc::c_int
                & 0xff as libc::c_int) as usize] as libc::c_int
                & _sch_isspace as libc::c_int as libc::c_ushort as libc::c_int != 0
                && *tend as libc::c_int == '-' as i32)
            && *tend as libc::c_int != '\n' as i32
            && *tend as libc::c_int != 0 as libc::c_int
        {
            tend = tend.offset(1);
            tend;
        }
        i = 0 as libc::c_int;
        while !(diropts[i as usize].param).is_null() {
            len = strlen(diropts[i as usize].param) as libc::c_int;
            if tend.offset_from(param) as libc::c_long >= len as libc::c_long
                && strncmp(param, diropts[i as usize].param, len as libc::c_ulong)
                    == 0 as libc::c_int
                && (*param.offset(len as isize) as libc::c_int == ':' as i32
                    || *param.offset(len as isize) as libc::c_int == ' ' as i32)
            {
                lex_parse_string_end = tend;
                lex_parse_string = param
                    .offset(len as isize)
                    .offset(1 as libc::c_int as isize);
                lex_forced_token = diropts[i as usize].token;
                saw_newline = 0 as libc::c_int;
                if !(def_parse() != 0) {
                    break;
                }
            }
            i += 1;
            i;
        }
        if (diropts[i as usize].param).is_null() {
            if tend < pend as *mut libc::c_char {
                let mut saved: libc::c_char = 0;
                saved = *tend;
                *tend = 0 as libc::c_int as libc::c_char;
                einfo(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Warning: .drectve `%s' unrecognized\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    param,
                );
                *tend = saved;
            } else {
                einfo(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Warning: corrupt .drectve at end of def file\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
            }
        }
        lex_parse_string = 0 as *const libc::c_char;
        param = tend;
    }
    def = save_def;
    def_pool_free();
}
#[no_mangle]
pub unsafe extern "C" fn def_get_module(
    mut fdef: *mut def_file,
    mut name: *const libc::c_char,
) -> *mut def_file_module {
    let mut s: *mut def_file_module = 0 as *mut def_file_module;
    s = (*fdef).modules;
    while !s.is_null() {
        if strcmp(((*s).name).as_mut_ptr(), name) == 0 as libc::c_int {
            return s;
        }
        s = (*s).next;
    }
    return 0 as *mut def_file_module;
}
static mut pool_strs: *mut def_pool_str = 0 as *const def_pool_str as *mut def_pool_str;
unsafe extern "C" fn def_pool_alloc(mut sz: size_t) -> *mut libc::c_char {
    let mut e: *mut def_pool_str = 0 as *mut def_pool_str;
    e = xmalloc(
        (::core::mem::size_of::<def_pool_str>() as libc::c_ulong).wrapping_add(sz),
    ) as *mut def_pool_str;
    (*e).next = pool_strs;
    pool_strs = e;
    return ((*e).data).as_mut_ptr();
}
unsafe extern "C" fn def_pool_strdup(mut str: *const libc::c_char) -> *mut libc::c_char {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0;
    if str.is_null() {
        return 0 as *mut libc::c_char;
    }
    len = (strlen(str)).wrapping_add(1 as libc::c_int as libc::c_ulong);
    s = def_pool_alloc(len);
    memcpy(s as *mut libc::c_void, str as *const libc::c_void, len);
    return s;
}
unsafe extern "C" fn def_pool_free() {
    let mut p: *mut def_pool_str = 0 as *mut def_pool_str;
    loop {
        p = pool_strs;
        if p.is_null() {
            break;
        }
        pool_strs = (*p).next;
        free(p as *mut libc::c_void);
    };
}
unsafe extern "C" fn def_description(mut text: *const libc::c_char) {
    let mut len: libc::c_int = (if !((*def).description).is_null() {
        strlen((*def).description)
    } else {
        0 as libc::c_int as libc::c_ulong
    }) as libc::c_int;
    len = (len as libc::c_ulong)
        .wrapping_add((strlen(text)).wrapping_add(1 as libc::c_int as libc::c_ulong))
        as libc::c_int as libc::c_int;
    if !((*def).description).is_null() {
        (*def)
            .description = xrealloc(
            (*def).description as *mut libc::c_void,
            len as size_t,
        ) as *mut libc::c_char;
        strcat((*def).description, text);
    } else {
        (*def).description = xmalloc(len as size_t) as *mut libc::c_char;
        strcpy((*def).description, text);
    };
}
unsafe extern "C" fn def_exports(
    mut external_name: *const libc::c_char,
    mut internal_name: *const libc::c_char,
    mut ordinal: libc::c_int,
    mut flags: libc::c_int,
    mut its_name: *const libc::c_char,
) {
    let mut dfe: *mut def_file_export = 0 as *mut def_file_export;
    let mut is_dup: libc::c_int = 0 as libc::c_int;
    if internal_name.is_null() && !external_name.is_null() {
        internal_name = external_name;
    }
    dfe = def_file_add_export(
        def,
        external_name,
        internal_name,
        ordinal,
        its_name,
        &mut is_dup,
    );
    if is_dup != 0 {
        return;
    }
    if flags & 1 as libc::c_int != 0 {
        (*dfe).flag_noname = 1 as libc::c_int as libc::c_char;
    }
    if flags & 2 as libc::c_int != 0 {
        (*dfe).flag_constant = 1 as libc::c_int as libc::c_char;
    }
    if flags & 4 as libc::c_int != 0 {
        (*dfe).flag_data = 1 as libc::c_int as libc::c_char;
    }
    if flags & 8 as libc::c_int != 0 {
        (*dfe).flag_private = 1 as libc::c_int as libc::c_char;
    }
}
unsafe extern "C" fn def_heapsize(mut reserve: libc::c_int, mut commit: libc::c_int) {
    (*def).heap_reserve = reserve;
    (*def).heap_commit = commit;
}
unsafe extern "C" fn def_import(
    mut internal_name: *const libc::c_char,
    mut module: *const libc::c_char,
    mut dllext: *const libc::c_char,
    mut name: *const libc::c_char,
    mut ordinal: libc::c_int,
    mut its_name: *const libc::c_char,
) {
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ext: *const libc::c_char = if !dllext.is_null() {
        dllext
    } else {
        b"dll\0" as *const u8 as *const libc::c_char
    };
    let mut is_dup: libc::c_int = 0 as libc::c_int;
    buf = xmalloc(
        (strlen(module))
            .wrapping_add(strlen(ext))
            .wrapping_add(2 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    sprintf(buf, b"%s.%s\0" as *const u8 as *const libc::c_char, module, ext);
    module = buf;
    def_file_add_import(
        def,
        name,
        module,
        ordinal,
        internal_name,
        its_name,
        &mut is_dup,
    );
    free(buf as *mut libc::c_void);
}
unsafe extern "C" fn def_image_name(
    mut name: *const libc::c_char,
    mut base: bfd_vma,
    mut is_dll: libc::c_int,
) {
    if *name != 0 {
        let mut image_name: *const libc::c_char = lbasename(name);
        if image_name != name {
            einfo(
                b"%s:%d: Warning: path components stripped from %s, '%s'\n\0"
                    as *const u8 as *const libc::c_char,
                def_filename,
                linenumber,
                if is_dll != 0 {
                    b"LIBRARY\0" as *const u8 as *const libc::c_char
                } else {
                    b"NAME\0" as *const u8 as *const libc::c_char
                },
                name,
            );
        }
        free((*def).name as *mut libc::c_void);
        if (strchr(image_name, '.' as i32)).is_null() {
            let mut suffix: *const libc::c_char = if is_dll != 0 {
                b".dll\0" as *const u8 as *const libc::c_char
            } else {
                b".exe\0" as *const u8 as *const libc::c_char
            };
            (*def)
                .name = xmalloc(
                (strlen(image_name))
                    .wrapping_add(strlen(suffix))
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            ) as *mut libc::c_char;
            sprintf(
                (*def).name,
                b"%s%s\0" as *const u8 as *const libc::c_char,
                image_name,
                suffix,
            );
        } else {
            (*def).name = xstrdup(image_name);
        }
    }
    (*def).base_address = base;
    (*def).is_dll = is_dll;
}
unsafe extern "C" fn def_section(mut name: *const libc::c_char, mut attr: libc::c_int) {
    let mut s: *mut def_file_section = 0 as *mut def_file_section;
    let mut max_sections: libc::c_int = (*def).num_section_defs
        + (4 as libc::c_int - 1 as libc::c_int) & !(4 as libc::c_int - 1 as libc::c_int);
    if (*def).num_section_defs >= max_sections {
        max_sections = (*def).num_section_defs + 1 as libc::c_int
            + (4 as libc::c_int - 1 as libc::c_int)
            & !(4 as libc::c_int - 1 as libc::c_int);
        if !((*def).section_defs).is_null() {
            (*def)
                .section_defs = xrealloc(
                (*def).section_defs as *mut libc::c_void,
                (max_sections as libc::c_ulong)
                    .wrapping_mul(
                        ::core::mem::size_of::<def_file_import>() as libc::c_ulong,
                    ),
            ) as *mut def_file_section;
        } else {
            (*def)
                .section_defs = xmalloc(
                (max_sections as libc::c_ulong)
                    .wrapping_mul(
                        ::core::mem::size_of::<def_file_import>() as libc::c_ulong,
                    ),
            ) as *mut def_file_section;
        }
    }
    s = ((*def).section_defs).offset((*def).num_section_defs as isize);
    memset(
        s as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<def_file_section>() as libc::c_ulong,
    );
    (*s).name = xstrdup(name);
    if attr & 1 as libc::c_int != 0 {
        (*s).flag_read = 1 as libc::c_int as libc::c_char;
    }
    if attr & 2 as libc::c_int != 0 {
        (*s).flag_write = 1 as libc::c_int as libc::c_char;
    }
    if attr & 4 as libc::c_int != 0 {
        (*s).flag_execute = 1 as libc::c_int as libc::c_char;
    }
    if attr & 8 as libc::c_int != 0 {
        (*s).flag_shared = 1 as libc::c_int as libc::c_char;
    }
    (*def).num_section_defs += 1;
    (*def).num_section_defs;
}
unsafe extern "C" fn def_section_alt(
    mut name: *const libc::c_char,
    mut attr: *const libc::c_char,
) {
    let mut aval: libc::c_int = 0 as libc::c_int;
    while *attr != 0 {
        match *attr as libc::c_int {
            82 | 114 => {
                aval |= 1 as libc::c_int;
            }
            87 | 119 => {
                aval |= 2 as libc::c_int;
            }
            88 | 120 => {
                aval |= 4 as libc::c_int;
            }
            83 | 115 => {
                aval |= 8 as libc::c_int;
            }
            _ => {}
        }
        attr = attr.offset(1);
        attr;
    }
    def_section(name, aval);
}
unsafe extern "C" fn def_stacksize(mut reserve: libc::c_int, mut commit: libc::c_int) {
    (*def).stack_reserve = reserve;
    (*def).stack_commit = commit;
}
unsafe extern "C" fn def_version(mut major: libc::c_int, mut minor: libc::c_int) {
    (*def).version_major = major;
    (*def).version_minor = minor;
}
unsafe extern "C" fn def_directive(mut str: *mut libc::c_char) {
    let mut d: *mut directive = xmalloc(
        ::core::mem::size_of::<directive>() as libc::c_ulong,
    ) as *mut directive;
    (*d).next = directives;
    directives = d;
    (*d).name = xstrdup(str);
    (*d).len = strlen(str) as libc::c_int;
}
unsafe extern "C" fn def_aligncomm(mut str: *mut libc::c_char, mut align: libc::c_int) {
    let mut c: *mut def_file_aligncomm = 0 as *mut def_file_aligncomm;
    let mut p: *mut def_file_aligncomm = 0 as *mut def_file_aligncomm;
    p = 0 as *mut def_file_aligncomm;
    c = (*def).aligncomms;
    while !c.is_null() {
        let mut e: libc::c_int = strcmp((*c).symbol_name, str);
        if e == 0 {
            e = (*c).alignment as libc::c_int - align;
            if e == 0 {
                return;
            }
        }
        if e > 0 as libc::c_int {
            break;
        }
        p = c;
        c = (*p).next;
    }
    c = xmalloc(::core::mem::size_of::<def_file_aligncomm>() as libc::c_ulong)
        as *mut def_file_aligncomm;
    (*c).symbol_name = xstrdup(str);
    (*c).alignment = align as libc::c_uint;
    if p.is_null() {
        (*c).next = (*def).aligncomms;
        (*def).aligncomms = c;
    } else {
        (*c).next = (*p).next;
        (*p).next = c;
    };
}
unsafe extern "C" fn def_parse() -> libc::c_int {
    let mut current_block: u64;
    let mut def_state: yy_state_fast_t = 0;
    let mut yyerrstatus: libc::c_int = 0;
    let mut yyssa: [yy_state_t; 200] = [0; 200];
    let mut yyss: *mut yy_state_t = 0 as *mut yy_state_t;
    let mut yyssp: *mut yy_state_t = 0 as *mut yy_state_t;
    let mut yyvsa: [YYSTYPE; 200] = [YYSTYPE {
        id: 0 as *mut libc::c_char,
    }; 200];
    let mut yyvs: *mut YYSTYPE = 0 as *mut YYSTYPE;
    let mut yyvsp: *mut YYSTYPE = 0 as *mut YYSTYPE;
    let mut yystacksize: libc::c_long = 0;
    let mut yyn: libc::c_int = 0;
    let mut yyresult: libc::c_int = 0;
    let mut yytoken: libc::c_int = 0 as libc::c_int;
    let mut def_val: YYSTYPE = YYSTYPE {
        id: 0 as *mut libc::c_char,
    };
    let mut def_yylen: libc::c_int = 0 as libc::c_int;
    yyss = yyssa.as_mut_ptr();
    yyssp = yyss;
    yyvs = yyvsa.as_mut_ptr();
    yyvsp = yyvs;
    yystacksize = 200 as libc::c_int as libc::c_long;
    def_state = 0 as libc::c_int;
    yyerrstatus = 0 as libc::c_int;
    def_nerrs = 0 as libc::c_int;
    def_char = -(2 as libc::c_int);
    's_64: loop {
        (0 as libc::c_int != 0
            && (0 as libc::c_int <= def_state && def_state < 146 as libc::c_int))
            as libc::c_int;
        *yyssp = def_state as yy_state_t;
        if yyss.offset(yystacksize as isize).offset(-(1 as libc::c_int as isize))
            <= yyssp
        {
            let mut yysize: libc::c_long = yyssp.offset_from(yyss) as libc::c_long
                + 1 as libc::c_int as libc::c_long;
            if 10000 as libc::c_int as libc::c_long <= yystacksize {
                current_block = 9922335338917765748;
                break;
            }
            yystacksize *= 2 as libc::c_int as libc::c_long;
            if (10000 as libc::c_int as libc::c_long) < yystacksize {
                yystacksize = 10000 as libc::c_int as libc::c_long;
            }
            let mut yyss1: *mut yy_state_t = yyss;
            let mut yyptr: *mut yyalloc = malloc(
                (yystacksize
                    * (::core::mem::size_of::<yy_state_t>() as libc::c_ulong
                        as libc::c_long
                        + ::core::mem::size_of::<YYSTYPE>() as libc::c_ulong
                            as libc::c_long)
                    + (::core::mem::size_of::<yyalloc>() as libc::c_ulong as libc::c_long
                        - 1 as libc::c_int as libc::c_long)) as libc::c_ulong,
            ) as *mut yyalloc;
            if yyptr.is_null() {
                current_block = 9922335338917765748;
                break;
            }
            let mut yynewbytes: libc::c_long = 0;
            libc::memcpy(
                &mut (*yyptr).yyss_alloc as *mut yy_state_t as *mut libc::c_void,
                yyss as *const libc::c_void,
                (yysize as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<yy_state_t>() as libc::c_ulong)
                    as libc::size_t,
            );
            yyss = &mut (*yyptr).yyss_alloc;
            yynewbytes = yystacksize
                * ::core::mem::size_of::<yy_state_t>() as libc::c_ulong as libc::c_long
                + (::core::mem::size_of::<yyalloc>() as libc::c_ulong as libc::c_long
                    - 1 as libc::c_int as libc::c_long);
            yyptr = yyptr
                .offset(
                    (yynewbytes
                        / ::core::mem::size_of::<yyalloc>() as libc::c_ulong
                            as libc::c_long) as isize,
                );
            let mut yynewbytes_0: libc::c_long = 0;
            libc::memcpy(
                &mut (*yyptr).yyvs_alloc as *mut YYSTYPE as *mut libc::c_void,
                yyvs as *const libc::c_void,
                (yysize as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<YYSTYPE>() as libc::c_ulong)
                    as libc::size_t,
            );
            yyvs = &mut (*yyptr).yyvs_alloc;
            yynewbytes_0 = yystacksize
                * ::core::mem::size_of::<YYSTYPE>() as libc::c_ulong as libc::c_long
                + (::core::mem::size_of::<yyalloc>() as libc::c_ulong as libc::c_long
                    - 1 as libc::c_int as libc::c_long);
            yyptr = yyptr
                .offset(
                    (yynewbytes_0
                        / ::core::mem::size_of::<yyalloc>() as libc::c_ulong
                            as libc::c_long) as isize,
                );
            if yyss1 != yyssa.as_mut_ptr() {
                free(yyss1 as *mut libc::c_void);
            }
            yyssp = yyss.offset(yysize as isize).offset(-(1 as libc::c_int as isize));
            yyvsp = yyvs.offset(yysize as isize).offset(-(1 as libc::c_int as isize));
            if yyss.offset(yystacksize as isize).offset(-(1 as libc::c_int as isize))
                <= yyssp
            {
                current_block = 15904800055075958447;
                break;
            }
        }
        if def_state == 69 as libc::c_int {
            yyresult = 0 as libc::c_int;
            current_block = 8678559895144391263;
            break;
        } else {
            yyn = def_pact[def_state as usize] as libc::c_int;
            if yyn == -(82 as libc::c_int) {
                current_block = 12554570050757128726;
            } else {
                if def_char == -(2 as libc::c_int) {
                    def_char = def_lex();
                }
                if def_char <= 0 as libc::c_int {
                    yytoken = 0 as libc::c_int;
                    def_char = yytoken;
                } else {
                    yytoken = if 0 as libc::c_int <= def_char
                        && def_char <= 285 as libc::c_int
                    {
                        yytranslate[def_char as usize] as libc::c_int
                    } else {
                        2 as libc::c_int
                    };
                }
                yyn += yytoken;
                if yyn < 0 as libc::c_int || (149 as libc::c_int) < yyn
                    || def_yycheck[yyn as usize] as libc::c_int != yytoken
                {
                    current_block = 12554570050757128726;
                } else {
                    yyn = def_yytable[yyn as usize] as libc::c_int;
                    if yyn <= 0 as libc::c_int {
                        yyn = -yyn;
                        current_block = 10442193796068875558;
                    } else {
                        if yyerrstatus != 0 {
                            yyerrstatus -= 1;
                            yyerrstatus;
                        }
                        def_state = yyn;
                        yyvsp = yyvsp.offset(1);
                        *yyvsp = def_lval;
                        def_char = -(2 as libc::c_int);
                        current_block = 9034886713717424956;
                    }
                }
            }
            match current_block {
                12554570050757128726 => {
                    yyn = yydefact[def_state as usize] as libc::c_int;
                    if yyn == 0 as libc::c_int {
                        yytoken = if def_char == -(2 as libc::c_int) {
                            -(2 as libc::c_int)
                        } else if 0 as libc::c_int <= def_char
                            && def_char <= 285 as libc::c_int
                        {
                            yytranslate[def_char as usize] as libc::c_int
                        } else {
                            2 as libc::c_int
                        };
                        if yyerrstatus == 0 {
                            def_nerrs += 1;
                            def_nerrs;
                            def_error(
                                b"syntax error\0" as *const u8 as *const libc::c_char,
                            );
                        }
                        if yyerrstatus == 3 as libc::c_int {
                            if def_char <= 0 as libc::c_int {
                                if def_char == 0 as libc::c_int {
                                    current_block = 15904800055075958447;
                                    break;
                                }
                            } else {
                                yydestruct(
                                    b"Error: discarding\0" as *const u8 as *const libc::c_char,
                                    yytoken,
                                    &mut def_lval,
                                );
                                def_char = -(2 as libc::c_int);
                            }
                        }
                        yyerrstatus = 3 as libc::c_int;
                        loop {
                            yyn = def_pact[def_state as usize] as libc::c_int;
                            if !(yyn == -(82 as libc::c_int)) {
                                yyn += 1 as libc::c_int;
                                if 0 as libc::c_int <= yyn && yyn <= 149 as libc::c_int
                                    && def_yycheck[yyn as usize] as libc::c_int
                                        == 1 as libc::c_int
                                {
                                    yyn = def_yytable[yyn as usize] as libc::c_int;
                                    if (0 as libc::c_int) < yyn {
                                        break;
                                    }
                                }
                            }
                            if yyssp == yyss {
                                current_block = 15904800055075958447;
                                break 's_64;
                            }
                            yydestruct(
                                b"Error: popping\0" as *const u8 as *const libc::c_char,
                                yystos[def_state as usize] as libc::c_int,
                                yyvsp,
                            );
                            yyvsp = yyvsp.offset(-(1 as libc::c_int as isize));
                            yyssp = yyssp.offset(-(1 as libc::c_int as isize));
                            def_state = *yyssp as yy_state_fast_t;
                        }
                        yyvsp = yyvsp.offset(1);
                        *yyvsp = def_lval;
                        def_state = yyn;
                        current_block = 9034886713717424956;
                    } else {
                        current_block = 10442193796068875558;
                    }
                }
                _ => {}
            }
            match current_block {
                10442193796068875558 => {
                    def_yylen = def_r2[yyn as usize] as libc::c_int;
                    def_val = *yyvsp.offset((1 as libc::c_int - def_yylen) as isize);
                    match yyn {
                        4 => {
                            def_image_name(
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).id,
                                (*yyvsp.offset(0 as libc::c_int as isize)).vma,
                                0 as libc::c_int,
                            );
                        }
                        5 => {
                            def_image_name(
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).id,
                                (*yyvsp.offset(0 as libc::c_int as isize)).vma,
                                1 as libc::c_int,
                            );
                        }
                        6 => {
                            def_description(
                                (*yyvsp.offset(0 as libc::c_int as isize)).id,
                            );
                        }
                        7 => {
                            def_stacksize(
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).number,
                                (*yyvsp.offset(0 as libc::c_int as isize)).number,
                            );
                        }
                        8 => {
                            def_heapsize(
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).number,
                                (*yyvsp.offset(0 as libc::c_int as isize)).number,
                            );
                        }
                        9 => {
                            def_section(
                                b"CODE\0" as *const u8 as *const libc::c_char,
                                (*yyvsp.offset(0 as libc::c_int as isize)).number,
                            );
                        }
                        10 => {
                            def_section(
                                b"DATA\0" as *const u8 as *const libc::c_char,
                                (*yyvsp.offset(0 as libc::c_int as isize)).number,
                            );
                        }
                        14 => {
                            def_version(
                                (*yyvsp.offset(0 as libc::c_int as isize)).number,
                                0 as libc::c_int,
                            );
                        }
                        15 => {
                            def_version(
                                (*yyvsp.offset(-(2 as libc::c_int) as isize)).number,
                                (*yyvsp.offset(0 as libc::c_int as isize)).number,
                            );
                        }
                        16 => {
                            def_directive((*yyvsp.offset(0 as libc::c_int as isize)).id);
                        }
                        17 => {
                            def_aligncomm(
                                (*yyvsp.offset(-(2 as libc::c_int) as isize)).id,
                                (*yyvsp.offset(0 as libc::c_int as isize)).number,
                            );
                        }
                        21 => {
                            def_exports(
                                (*yyvsp.offset(-(6 as libc::c_int) as isize)).id,
                                (*yyvsp.offset(-(5 as libc::c_int) as isize)).id,
                                (*yyvsp.offset(-(4 as libc::c_int) as isize)).number,
                                (*yyvsp.offset(-(2 as libc::c_int) as isize)).number,
                                (*yyvsp.offset(0 as libc::c_int as isize)).id,
                            );
                        }
                        22 => {
                            def_val
                                .number = (*yyvsp.offset(-(2 as libc::c_int) as isize))
                                .number | (*yyvsp.offset(0 as libc::c_int as isize)).number;
                        }
                        23 => {
                            def_val.number = 0 as libc::c_int;
                        }
                        24 => {
                            def_val.number = 1 as libc::c_int;
                        }
                        25 => {
                            def_val.number = 1 as libc::c_int;
                        }
                        26 => {
                            def_val.number = 2 as libc::c_int;
                        }
                        27 => {
                            def_val.number = 2 as libc::c_int;
                        }
                        28 => {
                            def_val.number = 4 as libc::c_int;
                        }
                        29 => {
                            def_val.number = 4 as libc::c_int;
                        }
                        30 => {
                            def_val.number = 8 as libc::c_int;
                        }
                        31 => {
                            def_val.number = 8 as libc::c_int;
                        }
                        34 => {
                            def_import(
                                (*yyvsp.offset(-(7 as libc::c_int) as isize)).id,
                                (*yyvsp.offset(-(5 as libc::c_int) as isize)).id,
                                (*yyvsp.offset(-(3 as libc::c_int) as isize)).id,
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).id,
                                -(1 as libc::c_int),
                                (*yyvsp.offset(0 as libc::c_int as isize)).id,
                            );
                        }
                        35 => {
                            def_import(
                                (*yyvsp.offset(-(7 as libc::c_int) as isize)).id,
                                (*yyvsp.offset(-(5 as libc::c_int) as isize)).id,
                                (*yyvsp.offset(-(3 as libc::c_int) as isize)).id,
                                0 as *const libc::c_char,
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).number,
                                (*yyvsp.offset(0 as libc::c_int as isize)).id,
                            );
                        }
                        36 => {
                            def_import(
                                (*yyvsp.offset(-(5 as libc::c_int) as isize)).id,
                                (*yyvsp.offset(-(3 as libc::c_int) as isize)).id,
                                0 as *const libc::c_char,
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).id,
                                -(1 as libc::c_int),
                                (*yyvsp.offset(0 as libc::c_int as isize)).id,
                            );
                        }
                        37 => {
                            def_import(
                                (*yyvsp.offset(-(5 as libc::c_int) as isize)).id,
                                (*yyvsp.offset(-(3 as libc::c_int) as isize)).id,
                                0 as *const libc::c_char,
                                0 as *const libc::c_char,
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).number,
                                (*yyvsp.offset(0 as libc::c_int as isize)).id,
                            );
                        }
                        38 => {
                            def_import(
                                0 as *const libc::c_char,
                                (*yyvsp.offset(-(5 as libc::c_int) as isize)).id,
                                (*yyvsp.offset(-(3 as libc::c_int) as isize)).id,
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).id,
                                -(1 as libc::c_int),
                                (*yyvsp.offset(0 as libc::c_int as isize)).id,
                            );
                        }
                        39 => {
                            def_import(
                                0 as *const libc::c_char,
                                (*yyvsp.offset(-(3 as libc::c_int) as isize)).id,
                                0 as *const libc::c_char,
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).id,
                                -(1 as libc::c_int),
                                (*yyvsp.offset(0 as libc::c_int as isize)).id,
                            );
                        }
                        42 => {
                            def_section(
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).id,
                                (*yyvsp.offset(0 as libc::c_int as isize)).number,
                            );
                        }
                        43 => {
                            def_section_alt(
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).id,
                                (*yyvsp.offset(0 as libc::c_int as isize)).id,
                            );
                        }
                        44 => {
                            def_val
                                .number = (*yyvsp.offset(-(2 as libc::c_int) as isize))
                                .number | (*yyvsp.offset(0 as libc::c_int as isize)).number;
                        }
                        45 => {
                            def_val
                                .number = (*yyvsp.offset(0 as libc::c_int as isize)).number;
                        }
                        48 => {
                            def_val
                                .number = (*yyvsp.offset(0 as libc::c_int as isize)).number;
                        }
                        49 => {
                            def_val.number = -(1 as libc::c_int);
                        }
                        50 => {
                            def_val.number = 1 as libc::c_int;
                        }
                        51 => {
                            def_val.number = 2 as libc::c_int;
                        }
                        52 => {
                            def_val.number = 4 as libc::c_int;
                        }
                        53 => {
                            def_val.number = 8 as libc::c_int;
                        }
                        54 => {
                            def_val
                                .id_const = b"BASE\0" as *const u8 as *const libc::c_char;
                        }
                        55 => {
                            def_val
                                .id_const = b"CODE\0" as *const u8 as *const libc::c_char;
                        }
                        56 => {
                            def_val
                                .id_const = b"CONSTANT\0" as *const u8
                                as *const libc::c_char;
                        }
                        57 => {
                            def_val
                                .id_const = b"constant\0" as *const u8
                                as *const libc::c_char;
                        }
                        58 => {
                            def_val
                                .id_const = b"DATA\0" as *const u8 as *const libc::c_char;
                        }
                        59 => {
                            def_val
                                .id_const = b"data\0" as *const u8 as *const libc::c_char;
                        }
                        60 => {
                            def_val
                                .id_const = b"DESCRIPTION\0" as *const u8
                                as *const libc::c_char;
                        }
                        61 => {
                            def_val
                                .id_const = b"DIRECTIVE\0" as *const u8
                                as *const libc::c_char;
                        }
                        62 => {
                            def_val
                                .id_const = b"EXECUTE\0" as *const u8
                                as *const libc::c_char;
                        }
                        63 => {
                            def_val
                                .id_const = b"EXPORTS\0" as *const u8
                                as *const libc::c_char;
                        }
                        64 => {
                            def_val
                                .id_const = b"HEAPSIZE\0" as *const u8
                                as *const libc::c_char;
                        }
                        65 => {
                            def_val
                                .id_const = b"IMPORTS\0" as *const u8
                                as *const libc::c_char;
                        }
                        66 => {
                            def_val
                                .id_const = b"NAME\0" as *const u8 as *const libc::c_char;
                        }
                        67 => {
                            def_val
                                .id_const = b"NONAME\0" as *const u8 as *const libc::c_char;
                        }
                        68 => {
                            def_val
                                .id_const = b"noname\0" as *const u8 as *const libc::c_char;
                        }
                        69 => {
                            def_val
                                .id_const = b"PRIVATE\0" as *const u8
                                as *const libc::c_char;
                        }
                        70 => {
                            def_val
                                .id_const = b"private\0" as *const u8
                                as *const libc::c_char;
                        }
                        71 => {
                            def_val
                                .id_const = b"READ\0" as *const u8 as *const libc::c_char;
                        }
                        72 => {
                            def_val
                                .id_const = b"SHARED\0" as *const u8 as *const libc::c_char;
                        }
                        73 => {
                            def_val
                                .id_const = b"STACKSIZE\0" as *const u8
                                as *const libc::c_char;
                        }
                        74 => {
                            def_val
                                .id_const = b"VERSION\0" as *const u8
                                as *const libc::c_char;
                        }
                        75 => {
                            def_val
                                .id_const = b"WRITE\0" as *const u8 as *const libc::c_char;
                        }
                        76 => {
                            def_val.id = (*yyvsp.offset(0 as libc::c_int as isize)).id;
                        }
                        77 => {
                            let mut name: *mut libc::c_char = xmalloc(
                                (strlen(
                                    (*yyvsp.offset(0 as libc::c_int as isize)).id_const,
                                ))
                                    .wrapping_add(2 as libc::c_int as libc::c_ulong),
                            ) as *mut libc::c_char;
                            sprintf(
                                name,
                                b".%s\0" as *const u8 as *const libc::c_char,
                                (*yyvsp.offset(0 as libc::c_int as isize)).id_const,
                            );
                            def_val.id = name;
                        }
                        78 => {
                            let mut name_0: *mut libc::c_char = def_pool_alloc(
                                (strlen((*yyvsp.offset(0 as libc::c_int as isize)).id))
                                    .wrapping_add(2 as libc::c_int as libc::c_ulong),
                            );
                            sprintf(
                                name_0,
                                b".%s\0" as *const u8 as *const libc::c_char,
                                (*yyvsp.offset(0 as libc::c_int as isize)).id,
                            );
                            def_val.id = name_0;
                        }
                        79 => {
                            let mut name_1: *mut libc::c_char = def_pool_alloc(
                                (strlen(
                                    (*yyvsp.offset(-(2 as libc::c_int) as isize)).id_const,
                                ))
                                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(
                                        strlen((*yyvsp.offset(0 as libc::c_int as isize)).id),
                                    )
                                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
                            );
                            sprintf(
                                name_1,
                                b"%s.%s\0" as *const u8 as *const libc::c_char,
                                (*yyvsp.offset(-(2 as libc::c_int) as isize)).id_const,
                                (*yyvsp.offset(0 as libc::c_int as isize)).id,
                            );
                            def_val.id = name_1;
                        }
                        80 => {
                            let mut name_2: *mut libc::c_char = def_pool_alloc(
                                (strlen((*yyvsp.offset(-(2 as libc::c_int) as isize)).id))
                                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(
                                        strlen((*yyvsp.offset(0 as libc::c_int as isize)).id),
                                    )
                                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
                            );
                            sprintf(
                                name_2,
                                b"%s.%s\0" as *const u8 as *const libc::c_char,
                                (*yyvsp.offset(-(2 as libc::c_int) as isize)).id,
                                (*yyvsp.offset(0 as libc::c_int as isize)).id,
                            );
                            def_val.id = name_2;
                        }
                        81 => {
                            def_val.id = (*yyvsp.offset(0 as libc::c_int as isize)).id;
                        }
                        82 => {
                            def_val
                                .id = b"\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char;
                        }
                        83 => {
                            def_val.id = (*yyvsp.offset(0 as libc::c_int as isize)).id;
                        }
                        84 => {
                            def_val.id = 0 as *mut libc::c_char;
                        }
                        85 => {
                            def_val
                                .number = (*yyvsp.offset(0 as libc::c_int as isize)).number;
                        }
                        86 => {
                            def_val.number = -(1 as libc::c_int);
                        }
                        87 => {
                            def_val.id = (*yyvsp.offset(0 as libc::c_int as isize)).id;
                        }
                        88 => {
                            def_val.id = 0 as *mut libc::c_char;
                        }
                        89 => {
                            def_val.vma = (*yyvsp.offset(0 as libc::c_int as isize)).vma;
                        }
                        90 => {
                            def_val.vma = -(1 as libc::c_int) as bfd_vma;
                        }
                        91 => {
                            def_val.id = (*yyvsp.offset(0 as libc::c_int as isize)).id;
                        }
                        92 => {
                            let mut id: *mut libc::c_char = def_pool_alloc(
                                (strlen((*yyvsp.offset(0 as libc::c_int as isize)).id))
                                    .wrapping_add(2 as libc::c_int as libc::c_ulong),
                            );
                            sprintf(
                                id,
                                b".%s\0" as *const u8 as *const libc::c_char,
                                (*yyvsp.offset(0 as libc::c_int as isize)).id,
                            );
                            def_val.id = id;
                        }
                        93 => {
                            let mut id_0: *mut libc::c_char = def_pool_alloc(
                                (strlen((*yyvsp.offset(-(3 as libc::c_int) as isize)).id))
                                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(
                                        strlen((*yyvsp.offset(-(1 as libc::c_int) as isize)).digits),
                                    )
                                    .wrapping_add(
                                        strlen((*yyvsp.offset(0 as libc::c_int as isize)).id),
                                    )
                                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
                            );
                            sprintf(
                                id_0,
                                b"%s.%s%s\0" as *const u8 as *const libc::c_char,
                                (*yyvsp.offset(-(3 as libc::c_int) as isize)).id,
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).digits,
                                (*yyvsp.offset(0 as libc::c_int as isize)).id,
                            );
                            def_val.id = id_0;
                        }
                        94 => {
                            def_val
                                .digits = (*yyvsp.offset(0 as libc::c_int as isize)).digits;
                        }
                        95 => {
                            def_val
                                .digits = b"\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char;
                        }
                        96 => {
                            def_val.id = (*yyvsp.offset(0 as libc::c_int as isize)).id;
                        }
                        97 => {
                            def_val
                                .id = b"\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char;
                        }
                        98 => {
                            def_val
                                .number = strtoul(
                                (*yyvsp.offset(0 as libc::c_int as isize)).digits,
                                0 as *mut *mut libc::c_char,
                                0 as libc::c_int,
                            ) as libc::c_int;
                        }
                        99 => {
                            def_val
                                .vma = strtoull(
                                (*yyvsp.offset(0 as libc::c_int as isize)).digits,
                                0 as *mut *mut libc::c_char,
                                0 as libc::c_int,
                            ) as bfd_vma;
                        }
                        _ => {}
                    }
                    yyvsp = yyvsp.offset(-(def_yylen as isize));
                    yyssp = yyssp.offset(-(def_yylen as isize));
                    def_yylen = 0 as libc::c_int;
                    yyvsp = yyvsp.offset(1);
                    *yyvsp = def_val;
                    let def_yylhs: libc::c_int = def_r1[yyn as usize] as libc::c_int
                        - 35 as libc::c_int;
                    let yyi: libc::c_int = yypgoto[def_yylhs as usize] as libc::c_int
                        + *yyssp as libc::c_int;
                    def_state = if 0 as libc::c_int <= yyi && yyi <= 149 as libc::c_int
                        && def_yycheck[yyi as usize] as libc::c_int
                            == *yyssp as libc::c_int
                    {
                        def_yytable[yyi as usize] as libc::c_int
                    } else {
                        yydefgoto[def_yylhs as usize] as libc::c_int
                    };
                }
                _ => {}
            }
            yyssp = yyssp.offset(1);
            yyssp;
        }
    }
    match current_block {
        9922335338917765748 => {
            def_error(b"memory exhausted\0" as *const u8 as *const libc::c_char);
            yyresult = 2 as libc::c_int;
        }
        15904800055075958447 => {
            yyresult = 1 as libc::c_int;
        }
        _ => {}
    }
    if def_char != -(2 as libc::c_int) {
        yytoken = if 0 as libc::c_int <= def_char && def_char <= 285 as libc::c_int {
            yytranslate[def_char as usize] as libc::c_int
        } else {
            2 as libc::c_int
        };
        yydestruct(
            b"Cleanup: discarding lookahead\0" as *const u8 as *const libc::c_char,
            yytoken,
            &mut def_lval,
        );
    }
    yyvsp = yyvsp.offset(-(def_yylen as isize));
    yyssp = yyssp.offset(-(def_yylen as isize));
    while yyssp != yyss {
        yydestruct(
            b"Cleanup: popping\0" as *const u8 as *const libc::c_char,
            yystos[*yyssp as usize] as libc::c_int,
            yyvsp,
        );
        yyvsp = yyvsp.offset(-(1 as libc::c_int as isize));
        yyssp = yyssp.offset(-(1 as libc::c_int as isize));
    }
    if yyss != yyssa.as_mut_ptr() {
        free(yyss as *mut libc::c_void);
    }
    return yyresult;
}
unsafe extern "C" fn def_error(mut err: *const libc::c_char) -> libc::c_int {
    einfo(
        b"%P: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
        if !def_filename.is_null() {
            def_filename
        } else {
            b"<unknown-file>\0" as *const u8 as *const libc::c_char
        },
        linenumber,
        err,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn def_lex() -> libc::c_int {
    let mut c: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut q: libc::c_int = 0;
    if lex_forced_token != 0 {
        i = lex_forced_token;
        lex_forced_token = 0 as libc::c_int;
        return i;
    }
    c = def_getc();
    while c != -(1 as libc::c_int) && (c == ' ' as i32 || c == '\t' as i32)
        && saw_newline != 0
    {
        c = def_getc();
    }
    if c == -(1 as libc::c_int) {
        return 0 as libc::c_int;
    }
    if saw_newline != 0 && c == ';' as i32 {
        loop {
            c = def_getc();
            if !(c != -(1 as libc::c_int) && c != '\n' as i32) {
                break;
            }
        }
        if c == '\n' as i32 {
            return def_lex();
        }
        return 0 as libc::c_int;
    }
    saw_newline = 0 as libc::c_int;
    if _sch_istable[(c & 0xff as libc::c_int) as usize] as libc::c_int
        & _sch_isdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        bufptr = 0 as libc::c_int;
        while c != -(1 as libc::c_int)
            && (_sch_istable[(c & 0xff as libc::c_int) as usize] as libc::c_int
                & _sch_isxdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
                || c == 'x' as i32)
        {
            put_buf(c as libc::c_char);
            c = def_getc();
        }
        if c != -(1 as libc::c_int) {
            def_ungetc(c);
        }
        def_lval.digits = def_pool_strdup(buffer);
        return 285 as libc::c_int;
    }
    if _sch_istable[(c & 0xff as libc::c_int) as usize] as libc::c_int
        & _sch_isalpha as libc::c_int as libc::c_ushort as libc::c_int != 0
        || !(strchr(b"$:-_?@\0" as *const u8 as *const libc::c_char, c)).is_null()
    {
        bufptr = 0 as libc::c_int;
        q = c;
        put_buf(c as libc::c_char);
        c = def_getc();
        if q == '@' as i32 {
            if _sch_istable[(c & 0xff as libc::c_int) as usize] as libc::c_int
                & _sch_isblank as libc::c_int as libc::c_ushort as libc::c_int != 0
            {
                return q
            } else if _sch_istable[(c & 0xff as libc::c_int) as usize] as libc::c_int
                & _sch_isdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
            {
                def_ungetc(c);
                return q;
            }
        }
        while c != -(1 as libc::c_int)
            && (_sch_istable[(c & 0xff as libc::c_int) as usize] as libc::c_int
                & _sch_isalnum as libc::c_int as libc::c_ushort as libc::c_int != 0
                || !(strchr(b"$:-_?/@<>\0" as *const u8 as *const libc::c_char, c))
                    .is_null())
        {
            put_buf(c as libc::c_char);
            c = def_getc();
        }
        if c != -(1 as libc::c_int) {
            def_ungetc(c);
        }
        if _sch_istable[(q & 0xff as libc::c_int) as usize] as libc::c_int
            & _sch_isalpha as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            i = 0 as libc::c_int;
            while !(tokens[i as usize].name).is_null() {
                if strcmp(tokens[i as usize].name, buffer) == 0 as libc::c_int {
                    return tokens[i as usize].token;
                }
                i += 1;
                i;
            }
        }
        def_lval.id = def_pool_strdup(buffer);
        return 284 as libc::c_int;
    }
    if c == '\'' as i32 || c == '"' as i32 {
        q = c;
        c = def_getc();
        bufptr = 0 as libc::c_int;
        while c != -(1 as libc::c_int) && c != q {
            put_buf(c as libc::c_char);
            c = def_getc();
        }
        def_lval.id = def_pool_strdup(buffer);
        return 284 as libc::c_int;
    }
    if c == '=' as i32 {
        c = def_getc();
        if c == '=' as i32 {
            return 283 as libc::c_int;
        }
        def_ungetc(c);
        return '=' as i32;
    }
    if c == '.' as i32 || c == ',' as i32 {
        return c;
    }
    if c == '\n' as i32 {
        linenumber += 1;
        linenumber;
        saw_newline = 1 as libc::c_int;
    }
    return def_lex();
}
static mut lex_forced_token: libc::c_int = 0 as libc::c_int;
static mut lex_parse_string: *const libc::c_char = 0 as *const libc::c_char;
static mut lex_parse_string_end: *const libc::c_char = 0 as *const libc::c_char;
#[no_mangle]
pub static mut def_lval: YYSTYPE = YYSTYPE {
    id: 0 as *mut libc::c_char,
};
static mut the_file: *mut FILE = 0 as *const FILE as *mut FILE;
static mut def_filename: *const libc::c_char = 0 as *const libc::c_char;
static mut linenumber: libc::c_int = 0;
static mut def: *mut def_file = 0 as *const def_file as *mut def_file;
static mut saw_newline: libc::c_int = 0;
static mut directives: *mut directive = 0 as *const directive as *mut directive;
unsafe extern "C" fn are_names_equal(
    mut s1: *const libc::c_char,
    mut s2: *const libc::c_char,
) -> libc::c_int {
    if s1.is_null() && s2.is_null() {
        return 0 as libc::c_int;
    }
    if s1.is_null() || s2.is_null() {
        return if s1.is_null() { -(1 as libc::c_int) } else { 1 as libc::c_int };
    }
    return strcmp(s1, s2);
}
unsafe extern "C" fn cmp_export_elem(
    mut e: *const def_file_export,
    mut ex_name: *const libc::c_char,
    mut in_name: *const libc::c_char,
    mut its_name: *const libc::c_char,
    mut ord: libc::c_int,
) -> libc::c_int {
    let mut r: libc::c_int = 0;
    r = are_names_equal(ex_name, (*e).name);
    if r != 0 as libc::c_int {
        return r;
    }
    r = are_names_equal(in_name, (*e).internal_name);
    if r != 0 as libc::c_int {
        return r;
    }
    r = are_names_equal(its_name, (*e).its_name);
    if r != 0 as libc::c_int {
        return r;
    }
    return ord - (*e).ordinal;
}
unsafe extern "C" fn find_export_in_list(
    mut b: *mut def_file_export,
    mut max: libc::c_int,
    mut ex_name: *const libc::c_char,
    mut in_name: *const libc::c_char,
    mut its_name: *const libc::c_char,
    mut ord: libc::c_int,
    mut is_ident: *mut libc::c_int,
) -> libc::c_int {
    let mut e: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut p: libc::c_int = 0;
    *is_ident = 0 as libc::c_int;
    if max == 0 {
        return 0 as libc::c_int;
    }
    e = cmp_export_elem(b, ex_name, in_name, its_name, ord);
    if e <= 0 as libc::c_int {
        if e == 0 {
            *is_ident = 1 as libc::c_int;
        }
        return 0 as libc::c_int;
    }
    if max == 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    e = cmp_export_elem(
        b.offset((max - 1 as libc::c_int) as isize),
        ex_name,
        in_name,
        its_name,
        ord,
    );
    if e > 0 as libc::c_int {
        return max
    } else if e == 0 || max == 2 as libc::c_int {
        if e == 0 {
            *is_ident = 1 as libc::c_int;
        }
        return max - 1 as libc::c_int;
    }
    l = 0 as libc::c_int;
    r = max - 1 as libc::c_int;
    while l < r {
        p = (l + r) / 2 as libc::c_int;
        e = cmp_export_elem(b.offset(p as isize), ex_name, in_name, its_name, ord);
        if e == 0 {
            *is_ident = 1 as libc::c_int;
            return p;
        } else if e < 0 as libc::c_int {
            r = p - 1 as libc::c_int;
        } else if e > 0 as libc::c_int {
            l = p + 1 as libc::c_int;
        }
    }
    e = cmp_export_elem(b.offset(l as isize), ex_name, in_name, its_name, ord);
    if e > 0 as libc::c_int {
        l += 1;
        l;
    } else if e == 0 {
        *is_ident = 1 as libc::c_int;
    }
    return l;
}
static mut yytranslate: [yytype_int8; 286] = [
    0 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    32 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    31 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    33 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    34 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    9 as libc::c_int as yytype_int8,
    10 as libc::c_int as yytype_int8,
    11 as libc::c_int as yytype_int8,
    12 as libc::c_int as yytype_int8,
    13 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    15 as libc::c_int as yytype_int8,
    16 as libc::c_int as yytype_int8,
    17 as libc::c_int as yytype_int8,
    18 as libc::c_int as yytype_int8,
    19 as libc::c_int as yytype_int8,
    20 as libc::c_int as yytype_int8,
    21 as libc::c_int as yytype_int8,
    22 as libc::c_int as yytype_int8,
    23 as libc::c_int as yytype_int8,
    24 as libc::c_int as yytype_int8,
    25 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    27 as libc::c_int as yytype_int8,
    28 as libc::c_int as yytype_int8,
    29 as libc::c_int as yytype_int8,
    30 as libc::c_int as yytype_int8,
];
unsafe extern "C" fn def_stash_module(
    mut fdef: *mut def_file,
    mut name: *const libc::c_char,
) -> *mut def_file_module {
    let mut s: *mut def_file_module = 0 as *mut def_file_module;
    s = def_get_module(fdef, name);
    if !s.is_null() {
        return s;
    }
    s = xmalloc(
        (::core::mem::size_of::<def_file_module>() as libc::c_ulong)
            .wrapping_add(strlen(name)),
    ) as *mut def_file_module;
    (*s).next = (*fdef).modules;
    (*fdef).modules = s;
    (*s).user_data = 0 as *mut libc::c_void;
    strcpy(((*s).name).as_mut_ptr(), name);
    return s;
}
static mut def_pact: [yytype_int8; 146] = [
    122 as libc::c_int as yytype_int8,
    11 as libc::c_int as yytype_int8,
    11 as libc::c_int as yytype_int8,
    -(25 as libc::c_int) as yytype_int8,
    9 as libc::c_int as yytype_int8,
    9 as libc::c_int as yytype_int8,
    53 as libc::c_int as yytype_int8,
    53 as libc::c_int as yytype_int8,
    -(17 as libc::c_int) as yytype_int8,
    11 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    9 as libc::c_int as yytype_int8,
    -(18 as libc::c_int) as yytype_int8,
    20 as libc::c_int as yytype_int8,
    95 as libc::c_int as yytype_int8,
    -(82 as libc::c_int) as yytype_int8,
    -(82 as libc::c_int) as yytype_int8,
    -(82 as libc::c_int) as yytype_int8,
    -(82 as libc::c_int) as yytype_int8,
    -(82 as libc::c_int) as yytype_int8,
    -(82 as libc::c_int) as yytype_int8,
    -(82 as libc::c_int) as yytype_int8,
    -(82 as libc::c_int) as yytype_int8,
    -(82 as libc::c_int) as yytype_int8,
    -(82 as libc::c_int) as yytype_int8,
    -(82 as libc::c_int) as yytype_int8,
    -(82 as libc::c_int) as yytype_int8,
    -(82 as libc::c_int) as yytype_int8,
    -(82 as libc::c_int) as yytype_int8,
    -(82 as libc::c_int) as yytype_int8,
    -(82 as libc::c_int) as yytype_int8,
    -(82 as libc::c_int) as yytype_int8,
    -(82 as libc::c_int) as yytype_int8,
    -(82 as libc::c_int) as yytype_int8,
    -(82 as libc::c_int) as yytype_int8,
    -(82 as libc::c_int) as yytype_int8,
    -(82 as libc::c_int) as yytype_int8,
    -(82 as libc::c_int) as yytype_int8,
    29 as libc::c_int as yytype_int8,
    11 as libc::c_int as yytype_int8,
    47 as libc::c_int as yytype_int8,
    -(82 as libc::c_int) as yytype_int8,
    67 as libc::c_int as yytype_int8,
    67 as libc::c_int as yytype_int8,
    -(82 as libc::c_int) as yytype_int8,
    -(82 as libc::c_int) as yytype_int8,
    54 as libc::c_int as yytype_int8,
    54 as libc::c_int as yytype_int8,
    -(82 as libc::c_int) as yytype_int8,
    -(82 as libc::c_int) as yytype_int8,
    -(82 as libc::c_int) as yytype_int8,
    -(82 as libc::c_int) as yytype_int8,
    48 as libc::c_int as yytype_int8,
    -(82 as libc::c_int) as yytype_int8,
    48 as libc::c_int as yytype_int8,
    -(14 as libc::c_int) as yytype_int8,
    -(17 as libc::c_int) as yytype_int8,
    -(82 as libc::c_int) as yytype_int8,
    11 as libc::c_int as yytype_int8,
    -(82 as libc::c_int) as yytype_int8,
    58 as libc::c_int as yytype_int8,
    50 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    -(82 as libc::c_int) as yytype_int8,
    61 as libc::c_int as yytype_int8,
    -(82 as libc::c_int) as yytype_int8,
    64 as libc::c_int as yytype_int8,
    33 as libc::c_int as yytype_int8,
    -(82 as libc::c_int) as yytype_int8,
    -(82 as libc::c_int) as yytype_int8,
    -(82 as libc::c_int) as yytype_int8,
    11 as libc::c_int as yytype_int8,
    47 as libc::c_int as yytype_int8,
    -(82 as libc::c_int) as yytype_int8,
    11 as libc::c_int as yytype_int8,
    63 as libc::c_int as yytype_int8,
    -(82 as libc::c_int) as yytype_int8,
    -(82 as libc::c_int) as yytype_int8,
    9 as libc::c_int as yytype_int8,
    -(82 as libc::c_int) as yytype_int8,
    -(82 as libc::c_int) as yytype_int8,
    -(82 as libc::c_int) as yytype_int8,
    53 as libc::c_int as yytype_int8,
    -(82 as libc::c_int) as yytype_int8,
    48 as libc::c_int as yytype_int8,
    -(82 as libc::c_int) as yytype_int8,
    -(82 as libc::c_int) as yytype_int8,
    11 as libc::c_int as yytype_int8,
    60 as libc::c_int as yytype_int8,
    76 as libc::c_int as yytype_int8,
    81 as libc::c_int as yytype_int8,
    -(82 as libc::c_int) as yytype_int8,
    9 as libc::c_int as yytype_int8,
    -(82 as libc::c_int) as yytype_int8,
    83 as libc::c_int as yytype_int8,
    9 as libc::c_int as yytype_int8,
    -(82 as libc::c_int) as yytype_int8,
    -(82 as libc::c_int) as yytype_int8,
    84 as libc::c_int as yytype_int8,
    -(82 as libc::c_int) as yytype_int8,
    -(82 as libc::c_int) as yytype_int8,
    -(82 as libc::c_int) as yytype_int8,
    9 as libc::c_int as yytype_int8,
    79 as libc::c_int as yytype_int8,
    -(26 as libc::c_int) as yytype_int8,
    85 as libc::c_int as yytype_int8,
    -(82 as libc::c_int) as yytype_int8,
    -(82 as libc::c_int) as yytype_int8,
    88 as libc::c_int as yytype_int8,
    -(82 as libc::c_int) as yytype_int8,
    -(82 as libc::c_int) as yytype_int8,
    -(82 as libc::c_int) as yytype_int8,
    -(82 as libc::c_int) as yytype_int8,
    36 as libc::c_int as yytype_int8,
    89 as libc::c_int as yytype_int8,
    90 as libc::c_int as yytype_int8,
    -(82 as libc::c_int) as yytype_int8,
    55 as libc::c_int as yytype_int8,
    -(82 as libc::c_int) as yytype_int8,
    -(82 as libc::c_int) as yytype_int8,
    -(82 as libc::c_int) as yytype_int8,
    -(82 as libc::c_int) as yytype_int8,
    -(82 as libc::c_int) as yytype_int8,
    -(82 as libc::c_int) as yytype_int8,
    -(82 as libc::c_int) as yytype_int8,
    -(82 as libc::c_int) as yytype_int8,
    -(82 as libc::c_int) as yytype_int8,
    -(82 as libc::c_int) as yytype_int8,
    79 as libc::c_int as yytype_int8,
    79 as libc::c_int as yytype_int8,
    -(82 as libc::c_int) as yytype_int8,
    92 as libc::c_int as yytype_int8,
    13 as libc::c_int as yytype_int8,
    92 as libc::c_int as yytype_int8,
    92 as libc::c_int as yytype_int8,
    36 as libc::c_int as yytype_int8,
    -(82 as libc::c_int) as yytype_int8,
    59 as libc::c_int as yytype_int8,
    -(82 as libc::c_int) as yytype_int8,
    -(82 as libc::c_int) as yytype_int8,
    -(82 as libc::c_int) as yytype_int8,
    -(82 as libc::c_int) as yytype_int8,
    92 as libc::c_int as yytype_int8,
    92 as libc::c_int as yytype_int8,
    -(82 as libc::c_int) as yytype_int8,
    -(82 as libc::c_int) as yytype_int8,
];
unsafe extern "C" fn cmp_import_elem(
    mut e: *const def_file_import,
    mut ex_name: *const libc::c_char,
    mut in_name: *const libc::c_char,
    mut module: *const libc::c_char,
    mut ord: libc::c_int,
) -> libc::c_int {
    let mut r: libc::c_int = 0;
    r = are_names_equal(
        module,
        if !((*e).module).is_null() {
            ((*(*e).module).name).as_mut_ptr()
        } else {
            0 as *mut libc::c_char
        },
    );
    if r != 0 {
        return r;
    }
    r = are_names_equal(ex_name, (*e).name);
    if r != 0 as libc::c_int {
        return r;
    }
    r = are_names_equal(in_name, (*e).internal_name);
    if r != 0 as libc::c_int {
        return r;
    }
    if ord != (*e).ordinal {
        return if ord < (*e).ordinal { -(1 as libc::c_int) } else { 1 as libc::c_int };
    }
    return 0 as libc::c_int;
}
static mut yydefact: [yytype_int8; 146] = [
    0 as libc::c_int as yytype_int8,
    82 as libc::c_int as yytype_int8,
    82 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    18 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    66 as libc::c_int as yytype_int8,
    60 as libc::c_int as yytype_int8,
    73 as libc::c_int as yytype_int8,
    64 as libc::c_int as yytype_int8,
    55 as libc::c_int as yytype_int8,
    58 as libc::c_int as yytype_int8,
    59 as libc::c_int as yytype_int8,
    63 as libc::c_int as yytype_int8,
    65 as libc::c_int as yytype_int8,
    74 as libc::c_int as yytype_int8,
    54 as libc::c_int as yytype_int8,
    56 as libc::c_int as yytype_int8,
    57 as libc::c_int as yytype_int8,
    69 as libc::c_int as yytype_int8,
    70 as libc::c_int as yytype_int8,
    71 as libc::c_int as yytype_int8,
    75 as libc::c_int as yytype_int8,
    62 as libc::c_int as yytype_int8,
    72 as libc::c_int as yytype_int8,
    67 as libc::c_int as yytype_int8,
    68 as libc::c_int as yytype_int8,
    61 as libc::c_int as yytype_int8,
    76 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    81 as libc::c_int as yytype_int8,
    90 as libc::c_int as yytype_int8,
    90 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    98 as libc::c_int as yytype_int8,
    49 as libc::c_int as yytype_int8,
    49 as libc::c_int as yytype_int8,
    50 as libc::c_int as yytype_int8,
    51 as libc::c_int as yytype_int8,
    52 as libc::c_int as yytype_int8,
    53 as libc::c_int as yytype_int8,
    9 as libc::c_int as yytype_int8,
    45 as libc::c_int as yytype_int8,
    10 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    11 as libc::c_int as yytype_int8,
    41 as libc::c_int as yytype_int8,
    12 as libc::c_int as yytype_int8,
    19 as libc::c_int as yytype_int8,
    88 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    13 as libc::c_int as yytype_int8,
    33 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    91 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    16 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    77 as libc::c_int as yytype_int8,
    78 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    46 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    43 as libc::c_int as yytype_int8,
    42 as libc::c_int as yytype_int8,
    40 as libc::c_int as yytype_int8,
    20 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    86 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    32 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    92 as libc::c_int as yytype_int8,
    95 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    80 as libc::c_int as yytype_int8,
    79 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    48 as libc::c_int as yytype_int8,
    44 as libc::c_int as yytype_int8,
    87 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    47 as libc::c_int as yytype_int8,
    84 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    15 as libc::c_int as yytype_int8,
    94 as libc::c_int as yytype_int8,
    97 as libc::c_int as yytype_int8,
    17 as libc::c_int as yytype_int8,
    99 as libc::c_int as yytype_int8,
    89 as libc::c_int as yytype_int8,
    85 as libc::c_int as yytype_int8,
    23 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    39 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    96 as libc::c_int as yytype_int8,
    93 as libc::c_int as yytype_int8,
    28 as libc::c_int as yytype_int8,
    29 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    27 as libc::c_int as yytype_int8,
    30 as libc::c_int as yytype_int8,
    31 as libc::c_int as yytype_int8,
    24 as libc::c_int as yytype_int8,
    25 as libc::c_int as yytype_int8,
    47 as libc::c_int as yytype_int8,
    47 as libc::c_int as yytype_int8,
    83 as libc::c_int as yytype_int8,
    84 as libc::c_int as yytype_int8,
    84 as libc::c_int as yytype_int8,
    84 as libc::c_int as yytype_int8,
    84 as libc::c_int as yytype_int8,
    23 as libc::c_int as yytype_int8,
    38 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    36 as libc::c_int as yytype_int8,
    37 as libc::c_int as yytype_int8,
    21 as libc::c_int as yytype_int8,
    22 as libc::c_int as yytype_int8,
    84 as libc::c_int as yytype_int8,
    84 as libc::c_int as yytype_int8,
    34 as libc::c_int as yytype_int8,
    35 as libc::c_int as yytype_int8,
];
unsafe extern "C" fn find_import_in_list(
    mut b: *mut def_file_import,
    mut max: libc::c_int,
    mut ex_name: *const libc::c_char,
    mut in_name: *const libc::c_char,
    mut module: *const libc::c_char,
    mut ord: libc::c_int,
    mut is_ident: *mut libc::c_int,
) -> libc::c_int {
    let mut e: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut p: libc::c_int = 0;
    *is_ident = 0 as libc::c_int;
    if max == 0 {
        return 0 as libc::c_int;
    }
    e = cmp_import_elem(b, ex_name, in_name, module, ord);
    if e <= 0 as libc::c_int {
        if e == 0 {
            *is_ident = 1 as libc::c_int;
        }
        return 0 as libc::c_int;
    }
    if max == 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    e = cmp_import_elem(
        b.offset((max - 1 as libc::c_int) as isize),
        ex_name,
        in_name,
        module,
        ord,
    );
    if e > 0 as libc::c_int {
        return max
    } else if e == 0 || max == 2 as libc::c_int {
        if e == 0 {
            *is_ident = 1 as libc::c_int;
        }
        return max - 1 as libc::c_int;
    }
    l = 0 as libc::c_int;
    r = max - 1 as libc::c_int;
    while l < r {
        p = (l + r) / 2 as libc::c_int;
        e = cmp_import_elem(b.offset(p as isize), ex_name, in_name, module, ord);
        if e == 0 {
            *is_ident = 1 as libc::c_int;
            return p;
        } else if e < 0 as libc::c_int {
            r = p - 1 as libc::c_int;
        } else if e > 0 as libc::c_int {
            l = p + 1 as libc::c_int;
        }
    }
    e = cmp_import_elem(b.offset(l as isize), ex_name, in_name, module, ord);
    if e > 0 as libc::c_int {
        l += 1;
        l;
    } else if e == 0 {
        *is_ident = 1 as libc::c_int;
    }
    return l;
}
static mut yypgoto: [yytype_int16; 27] = [
    -(82 as libc::c_int) as yytype_int16,
    -(82 as libc::c_int) as yytype_int16,
    107 as libc::c_int as yytype_int16,
    -(82 as libc::c_int) as yytype_int16,
    65 as libc::c_int as yytype_int16,
    -(11 as libc::c_int) as yytype_int16,
    -(82 as libc::c_int) as yytype_int16,
    -(82 as libc::c_int) as yytype_int16,
    75 as libc::c_int as yytype_int16,
    -(82 as libc::c_int) as yytype_int16,
    82 as libc::c_int as yytype_int16,
    -(4 as libc::c_int) as yytype_int16,
    -(81 as libc::c_int) as yytype_int16,
    93 as libc::c_int as yytype_int16,
    57 as libc::c_int as yytype_int16,
    102 as libc::c_int as yytype_int16,
    -(8 as libc::c_int) as yytype_int16,
    141 as libc::c_int as yytype_int16,
    -(75 as libc::c_int) as yytype_int16,
    -(82 as libc::c_int) as yytype_int16,
    -(82 as libc::c_int) as yytype_int16,
    101 as libc::c_int as yytype_int16,
    -(82 as libc::c_int) as yytype_int16,
    -(82 as libc::c_int) as yytype_int16,
    -(82 as libc::c_int) as yytype_int16,
    -(5 as libc::c_int) as yytype_int16,
    -(82 as libc::c_int) as yytype_int16,
];
static mut yydefgoto: [yytype_int16; 27] = [
    -(1 as libc::c_int) as yytype_int16,
    14 as libc::c_int as yytype_int16,
    15 as libc::c_int as yytype_int16,
    58 as libc::c_int as yytype_int16,
    59 as libc::c_int as yytype_int16,
    128 as libc::c_int as yytype_int16,
    129 as libc::c_int as yytype_int16,
    62 as libc::c_int as yytype_int16,
    63 as libc::c_int as yytype_int16,
    56 as libc::c_int as yytype_int16,
    57 as libc::c_int as yytype_int16,
    52 as libc::c_int as yytype_int16,
    82 as libc::c_int as yytype_int16,
    79 as libc::c_int as yytype_int16,
    53 as libc::c_int as yytype_int16,
    40 as libc::c_int as yytype_int16,
    41 as libc::c_int as yytype_int16,
    42 as libc::c_int as yytype_int16,
    116 as libc::c_int as yytype_int16,
    103 as libc::c_int as yytype_int16,
    88 as libc::c_int as yytype_int16,
    76 as libc::c_int as yytype_int16,
    67 as libc::c_int as yytype_int16,
    108 as libc::c_int as yytype_int16,
    119 as libc::c_int as yytype_int16,
    46 as libc::c_int as yytype_int16,
    111 as libc::c_int as yytype_int16,
];
static mut def_yytable: [yytype_int16; 150] = [
    47 as libc::c_int as yytype_int16,
    60 as libc::c_int as yytype_int16,
    114 as libc::c_int as yytype_int16,
    54 as libc::c_int as yytype_int16,
    44 as libc::c_int as yytype_int16,
    115 as libc::c_int as yytype_int16,
    64 as libc::c_int as yytype_int16,
    48 as libc::c_int as yytype_int16,
    49 as libc::c_int as yytype_int16,
    50 as libc::c_int as yytype_int16,
    51 as libc::c_int as yytype_int16,
    65 as libc::c_int as yytype_int16,
    55 as libc::c_int as yytype_int16,
    66 as libc::c_int as yytype_int16,
    16 as libc::c_int as yytype_int16,
    83 as libc::c_int as yytype_int16,
    17 as libc::c_int as yytype_int16,
    18 as libc::c_int as yytype_int16,
    19 as libc::c_int as yytype_int16,
    20 as libc::c_int as yytype_int16,
    21 as libc::c_int as yytype_int16,
    22 as libc::c_int as yytype_int16,
    113 as libc::c_int as yytype_int16,
    23 as libc::c_int as yytype_int16,
    24 as libc::c_int as yytype_int16,
    25 as libc::c_int as yytype_int16,
    26 as libc::c_int as yytype_int16,
    27 as libc::c_int as yytype_int16,
    28 as libc::c_int as yytype_int16,
    29 as libc::c_int as yytype_int16,
    30 as libc::c_int as yytype_int16,
    73 as libc::c_int as yytype_int16,
    31 as libc::c_int as yytype_int16,
    32 as libc::c_int as yytype_int16,
    33 as libc::c_int as yytype_int16,
    34 as libc::c_int as yytype_int16,
    35 as libc::c_int as yytype_int16,
    36 as libc::c_int as yytype_int16,
    37 as libc::c_int as yytype_int16,
    45 as libc::c_int as yytype_int16,
    38 as libc::c_int as yytype_int16,
    114 as libc::c_int as yytype_int16,
    39 as libc::c_int as yytype_int16,
    61 as libc::c_int as yytype_int16,
    137 as libc::c_int as yytype_int16,
    120 as libc::c_int as yytype_int16,
    121 as libc::c_int as yytype_int16,
    134 as libc::c_int as yytype_int16,
    135 as libc::c_int as yytype_int16,
    68 as libc::c_int as yytype_int16,
    60 as libc::c_int as yytype_int16,
    84 as libc::c_int as yytype_int16,
    122 as libc::c_int as yytype_int16,
    123 as libc::c_int as yytype_int16,
    124 as libc::c_int as yytype_int16,
    125 as libc::c_int as yytype_int16,
    136 as libc::c_int as yytype_int16,
    138 as libc::c_int as yytype_int16,
    139 as libc::c_int as yytype_int16,
    140 as libc::c_int as yytype_int16,
    71 as libc::c_int as yytype_int16,
    126 as libc::c_int as yytype_int16,
    127 as libc::c_int as yytype_int16,
    96 as libc::c_int as yytype_int16,
    94 as libc::c_int as yytype_int16,
    95 as libc::c_int as yytype_int16,
    97 as libc::c_int as yytype_int16,
    144 as libc::c_int as yytype_int16,
    145 as libc::c_int as yytype_int16,
    -(47 as libc::c_int) as yytype_int16,
    -(47 as libc::c_int) as yytype_int16,
    -(47 as libc::c_int) as yytype_int16,
    -(47 as libc::c_int) as yytype_int16,
    99 as libc::c_int as yytype_int16,
    48 as libc::c_int as yytype_int16,
    49 as libc::c_int as yytype_int16,
    50 as libc::c_int as yytype_int16,
    51 as libc::c_int as yytype_int16,
    74 as libc::c_int as yytype_int16,
    101 as libc::c_int as yytype_int16,
    81 as libc::c_int as yytype_int16,
    89 as libc::c_int as yytype_int16,
    75 as libc::c_int as yytype_int16,
    90 as libc::c_int as yytype_int16,
    132 as libc::c_int as yytype_int16,
    45 as libc::c_int as yytype_int16,
    78 as libc::c_int as yytype_int16,
    106 as libc::c_int as yytype_int16,
    142 as libc::c_int as yytype_int16,
    45 as libc::c_int as yytype_int16,
    109 as libc::c_int as yytype_int16,
    87 as libc::c_int as yytype_int16,
    92 as libc::c_int as yytype_int16,
    93 as libc::c_int as yytype_int16,
    102 as libc::c_int as yytype_int16,
    69 as libc::c_int as yytype_int16,
    98 as libc::c_int as yytype_int16,
    112 as libc::c_int as yytype_int16,
    1 as libc::c_int as yytype_int16,
    2 as libc::c_int as yytype_int16,
    3 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    5 as libc::c_int as yytype_int16,
    6 as libc::c_int as yytype_int16,
    7 as libc::c_int as yytype_int16,
    104 as libc::c_int as yytype_int16,
    8 as libc::c_int as yytype_int16,
    9 as libc::c_int as yytype_int16,
    10 as libc::c_int as yytype_int16,
    11 as libc::c_int as yytype_int16,
    105 as libc::c_int as yytype_int16,
    81 as libc::c_int as yytype_int16,
    133 as libc::c_int as yytype_int16,
    107 as libc::c_int as yytype_int16,
    110 as libc::c_int as yytype_int16,
    12 as libc::c_int as yytype_int16,
    117 as libc::c_int as yytype_int16,
    118 as libc::c_int as yytype_int16,
    130 as libc::c_int as yytype_int16,
    131 as libc::c_int as yytype_int16,
    114 as libc::c_int as yytype_int16,
    70 as libc::c_int as yytype_int16,
    13 as libc::c_int as yytype_int16,
    86 as libc::c_int as yytype_int16,
    141 as libc::c_int as yytype_int16,
    1 as libc::c_int as yytype_int16,
    2 as libc::c_int as yytype_int16,
    3 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    5 as libc::c_int as yytype_int16,
    6 as libc::c_int as yytype_int16,
    7 as libc::c_int as yytype_int16,
    143 as libc::c_int as yytype_int16,
    8 as libc::c_int as yytype_int16,
    9 as libc::c_int as yytype_int16,
    10 as libc::c_int as yytype_int16,
    11 as libc::c_int as yytype_int16,
    91 as libc::c_int as yytype_int16,
    85 as libc::c_int as yytype_int16,
    100 as libc::c_int as yytype_int16,
    80 as libc::c_int as yytype_int16,
    72 as libc::c_int as yytype_int16,
    12 as libc::c_int as yytype_int16,
    43 as libc::c_int as yytype_int16,
    77 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    13 as libc::c_int as yytype_int16,
];
unsafe extern "C" fn fill_in_import(
    mut i: *mut def_file_import,
    mut name: *const libc::c_char,
    mut module: *mut def_file_module,
    mut ordinal: libc::c_int,
    mut internal_name: *const libc::c_char,
    mut its_name: *const libc::c_char,
) {
    memset(
        i as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<def_file_import>() as libc::c_ulong,
    );
    if !name.is_null() {
        (*i).name = xstrdup(name);
    }
    (*i).module = module;
    (*i).ordinal = ordinal;
    if !internal_name.is_null() {
        (*i).internal_name = xstrdup(internal_name);
    } else {
        (*i).internal_name = (*i).name;
    }
    (*i)
        .its_name = if !its_name.is_null() {
        xstrdup(its_name)
    } else {
        0 as *mut libc::c_char
    };
}
static mut def_yycheck: [yytype_int16; 150] = [
    5 as libc::c_int as yytype_int16,
    9 as libc::c_int as yytype_int16,
    28 as libc::c_int as yytype_int16,
    7 as libc::c_int as yytype_int16,
    29 as libc::c_int as yytype_int16,
    31 as libc::c_int as yytype_int16,
    11 as libc::c_int as yytype_int16,
    21 as libc::c_int as yytype_int16,
    22 as libc::c_int as yytype_int16,
    23 as libc::c_int as yytype_int16,
    24 as libc::c_int as yytype_int16,
    29 as libc::c_int as yytype_int16,
    29 as libc::c_int as yytype_int16,
    31 as libc::c_int as yytype_int16,
    3 as libc::c_int as yytype_int16,
    29 as libc::c_int as yytype_int16,
    5 as libc::c_int as yytype_int16,
    6 as libc::c_int as yytype_int16,
    7 as libc::c_int as yytype_int16,
    8 as libc::c_int as yytype_int16,
    9 as libc::c_int as yytype_int16,
    10 as libc::c_int as yytype_int16,
    103 as libc::c_int as yytype_int16,
    12 as libc::c_int as yytype_int16,
    13 as libc::c_int as yytype_int16,
    14 as libc::c_int as yytype_int16,
    15 as libc::c_int as yytype_int16,
    16 as libc::c_int as yytype_int16,
    17 as libc::c_int as yytype_int16,
    18 as libc::c_int as yytype_int16,
    19 as libc::c_int as yytype_int16,
    39 as libc::c_int as yytype_int16,
    21 as libc::c_int as yytype_int16,
    22 as libc::c_int as yytype_int16,
    23 as libc::c_int as yytype_int16,
    24 as libc::c_int as yytype_int16,
    25 as libc::c_int as yytype_int16,
    26 as libc::c_int as yytype_int16,
    27 as libc::c_int as yytype_int16,
    30 as libc::c_int as yytype_int16,
    29 as libc::c_int as yytype_int16,
    28 as libc::c_int as yytype_int16,
    31 as libc::c_int as yytype_int16,
    29 as libc::c_int as yytype_int16,
    31 as libc::c_int as yytype_int16,
    9 as libc::c_int as yytype_int16,
    10 as libc::c_int as yytype_int16,
    128 as libc::c_int as yytype_int16,
    129 as libc::c_int as yytype_int16,
    29 as libc::c_int as yytype_int16,
    58 as libc::c_int as yytype_int16,
    55 as libc::c_int as yytype_int16,
    16 as libc::c_int as yytype_int16,
    17 as libc::c_int as yytype_int16,
    18 as libc::c_int as yytype_int16,
    19 as libc::c_int as yytype_int16,
    131 as libc::c_int as yytype_int16,
    132 as libc::c_int as yytype_int16,
    133 as libc::c_int as yytype_int16,
    134 as libc::c_int as yytype_int16,
    31 as libc::c_int as yytype_int16,
    25 as libc::c_int as yytype_int16,
    26 as libc::c_int as yytype_int16,
    71 as libc::c_int as yytype_int16,
    31 as libc::c_int as yytype_int16,
    32 as libc::c_int as yytype_int16,
    74 as libc::c_int as yytype_int16,
    142 as libc::c_int as yytype_int16,
    143 as libc::c_int as yytype_int16,
    21 as libc::c_int as yytype_int16,
    22 as libc::c_int as yytype_int16,
    23 as libc::c_int as yytype_int16,
    24 as libc::c_int as yytype_int16,
    78 as libc::c_int as yytype_int16,
    21 as libc::c_int as yytype_int16,
    22 as libc::c_int as yytype_int16,
    23 as libc::c_int as yytype_int16,
    24 as libc::c_int as yytype_int16,
    31 as libc::c_int as yytype_int16,
    87 as libc::c_int as yytype_int16,
    32 as libc::c_int as yytype_int16,
    31 as libc::c_int as yytype_int16,
    15 as libc::c_int as yytype_int16,
    33 as libc::c_int as yytype_int16,
    29 as libc::c_int as yytype_int16,
    30 as libc::c_int as yytype_int16,
    32 as libc::c_int as yytype_int16,
    92 as libc::c_int as yytype_int16,
    29 as libc::c_int as yytype_int16,
    30 as libc::c_int as yytype_int16,
    95 as libc::c_int as yytype_int16,
    33 as libc::c_int as yytype_int16,
    31 as libc::c_int as yytype_int16,
    29 as libc::c_int as yytype_int16,
    34 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    33 as libc::c_int as yytype_int16,
    102 as libc::c_int as yytype_int16,
    3 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    5 as libc::c_int as yytype_int16,
    6 as libc::c_int as yytype_int16,
    7 as libc::c_int as yytype_int16,
    8 as libc::c_int as yytype_int16,
    9 as libc::c_int as yytype_int16,
    29 as libc::c_int as yytype_int16,
    11 as libc::c_int as yytype_int16,
    12 as libc::c_int as yytype_int16,
    13 as libc::c_int as yytype_int16,
    14 as libc::c_int as yytype_int16,
    29 as libc::c_int as yytype_int16,
    32 as libc::c_int as yytype_int16,
    117 as libc::c_int as yytype_int16,
    30 as libc::c_int as yytype_int16,
    30 as libc::c_int as yytype_int16,
    20 as libc::c_int as yytype_int16,
    31 as libc::c_int as yytype_int16,
    29 as libc::c_int as yytype_int16,
    29 as libc::c_int as yytype_int16,
    29 as libc::c_int as yytype_int16,
    28 as libc::c_int as yytype_int16,
    14 as libc::c_int as yytype_int16,
    27 as libc::c_int as yytype_int16,
    58 as libc::c_int as yytype_int16,
    135 as libc::c_int as yytype_int16,
    3 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    5 as libc::c_int as yytype_int16,
    6 as libc::c_int as yytype_int16,
    7 as libc::c_int as yytype_int16,
    8 as libc::c_int as yytype_int16,
    9 as libc::c_int as yytype_int16,
    137 as libc::c_int as yytype_int16,
    11 as libc::c_int as yytype_int16,
    12 as libc::c_int as yytype_int16,
    13 as libc::c_int as yytype_int16,
    14 as libc::c_int as yytype_int16,
    62 as libc::c_int as yytype_int16,
    56 as libc::c_int as yytype_int16,
    82 as libc::c_int as yytype_int16,
    47 as libc::c_int as yytype_int16,
    39 as libc::c_int as yytype_int16,
    20 as libc::c_int as yytype_int16,
    2 as libc::c_int as yytype_int16,
    43 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    27 as libc::c_int as yytype_int16,
];
static mut yystos: [yytype_int8; 146] = [
    0 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    9 as libc::c_int as yytype_int8,
    11 as libc::c_int as yytype_int8,
    12 as libc::c_int as yytype_int8,
    13 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    20 as libc::c_int as yytype_int8,
    27 as libc::c_int as yytype_int8,
    36 as libc::c_int as yytype_int8,
    37 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    9 as libc::c_int as yytype_int8,
    10 as libc::c_int as yytype_int8,
    12 as libc::c_int as yytype_int8,
    13 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    15 as libc::c_int as yytype_int8,
    16 as libc::c_int as yytype_int8,
    17 as libc::c_int as yytype_int8,
    18 as libc::c_int as yytype_int8,
    19 as libc::c_int as yytype_int8,
    21 as libc::c_int as yytype_int8,
    22 as libc::c_int as yytype_int8,
    23 as libc::c_int as yytype_int8,
    24 as libc::c_int as yytype_int8,
    25 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    27 as libc::c_int as yytype_int8,
    29 as libc::c_int as yytype_int8,
    31 as libc::c_int as yytype_int8,
    50 as libc::c_int as yytype_int8,
    51 as libc::c_int as yytype_int8,
    52 as libc::c_int as yytype_int8,
    52 as libc::c_int as yytype_int8,
    29 as libc::c_int as yytype_int8,
    30 as libc::c_int as yytype_int8,
    60 as libc::c_int as yytype_int8,
    60 as libc::c_int as yytype_int8,
    21 as libc::c_int as yytype_int8,
    22 as libc::c_int as yytype_int8,
    23 as libc::c_int as yytype_int8,
    24 as libc::c_int as yytype_int8,
    46 as libc::c_int as yytype_int8,
    49 as libc::c_int as yytype_int8,
    46 as libc::c_int as yytype_int8,
    29 as libc::c_int as yytype_int8,
    44 as libc::c_int as yytype_int8,
    45 as libc::c_int as yytype_int8,
    38 as libc::c_int as yytype_int8,
    39 as libc::c_int as yytype_int8,
    51 as libc::c_int as yytype_int8,
    29 as libc::c_int as yytype_int8,
    42 as libc::c_int as yytype_int8,
    43 as libc::c_int as yytype_int8,
    60 as libc::c_int as yytype_int8,
    29 as libc::c_int as yytype_int8,
    31 as libc::c_int as yytype_int8,
    57 as libc::c_int as yytype_int8,
    29 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    37 as libc::c_int as yytype_int8,
    31 as libc::c_int as yytype_int8,
    50 as libc::c_int as yytype_int8,
    51 as libc::c_int as yytype_int8,
    31 as libc::c_int as yytype_int8,
    15 as libc::c_int as yytype_int8,
    56 as libc::c_int as yytype_int8,
    56 as libc::c_int as yytype_int8,
    32 as libc::c_int as yytype_int8,
    48 as libc::c_int as yytype_int8,
    48 as libc::c_int as yytype_int8,
    32 as libc::c_int as yytype_int8,
    47 as libc::c_int as yytype_int8,
    29 as libc::c_int as yytype_int8,
    46 as libc::c_int as yytype_int8,
    45 as libc::c_int as yytype_int8,
    39 as libc::c_int as yytype_int8,
    33 as libc::c_int as yytype_int8,
    55 as libc::c_int as yytype_int8,
    31 as libc::c_int as yytype_int8,
    33 as libc::c_int as yytype_int8,
    43 as libc::c_int as yytype_int8,
    31 as libc::c_int as yytype_int8,
    29 as libc::c_int as yytype_int8,
    31 as libc::c_int as yytype_int8,
    32 as libc::c_int as yytype_int8,
    51 as libc::c_int as yytype_int8,
    51 as libc::c_int as yytype_int8,
    33 as libc::c_int as yytype_int8,
    60 as libc::c_int as yytype_int8,
    49 as libc::c_int as yytype_int8,
    51 as libc::c_int as yytype_int8,
    34 as libc::c_int as yytype_int8,
    54 as libc::c_int as yytype_int8,
    29 as libc::c_int as yytype_int8,
    29 as libc::c_int as yytype_int8,
    60 as libc::c_int as yytype_int8,
    30 as libc::c_int as yytype_int8,
    58 as libc::c_int as yytype_int8,
    60 as libc::c_int as yytype_int8,
    30 as libc::c_int as yytype_int8,
    61 as libc::c_int as yytype_int8,
    60 as libc::c_int as yytype_int8,
    47 as libc::c_int as yytype_int8,
    28 as libc::c_int as yytype_int8,
    31 as libc::c_int as yytype_int8,
    53 as libc::c_int as yytype_int8,
    31 as libc::c_int as yytype_int8,
    29 as libc::c_int as yytype_int8,
    59 as libc::c_int as yytype_int8,
    9 as libc::c_int as yytype_int8,
    10 as libc::c_int as yytype_int8,
    16 as libc::c_int as yytype_int8,
    17 as libc::c_int as yytype_int8,
    18 as libc::c_int as yytype_int8,
    19 as libc::c_int as yytype_int8,
    25 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    40 as libc::c_int as yytype_int8,
    41 as libc::c_int as yytype_int8,
    29 as libc::c_int as yytype_int8,
    29 as libc::c_int as yytype_int8,
    29 as libc::c_int as yytype_int8,
    60 as libc::c_int as yytype_int8,
    47 as libc::c_int as yytype_int8,
    47 as libc::c_int as yytype_int8,
    53 as libc::c_int as yytype_int8,
    31 as libc::c_int as yytype_int8,
    53 as libc::c_int as yytype_int8,
    53 as libc::c_int as yytype_int8,
    53 as libc::c_int as yytype_int8,
    40 as libc::c_int as yytype_int8,
    29 as libc::c_int as yytype_int8,
    60 as libc::c_int as yytype_int8,
    53 as libc::c_int as yytype_int8,
    53 as libc::c_int as yytype_int8,
];
static mut def_r1: [yytype_int8; 100] = [
    0 as libc::c_int as yytype_int8,
    35 as libc::c_int as yytype_int8,
    36 as libc::c_int as yytype_int8,
    36 as libc::c_int as yytype_int8,
    37 as libc::c_int as yytype_int8,
    37 as libc::c_int as yytype_int8,
    37 as libc::c_int as yytype_int8,
    37 as libc::c_int as yytype_int8,
    37 as libc::c_int as yytype_int8,
    37 as libc::c_int as yytype_int8,
    37 as libc::c_int as yytype_int8,
    37 as libc::c_int as yytype_int8,
    37 as libc::c_int as yytype_int8,
    37 as libc::c_int as yytype_int8,
    37 as libc::c_int as yytype_int8,
    37 as libc::c_int as yytype_int8,
    37 as libc::c_int as yytype_int8,
    37 as libc::c_int as yytype_int8,
    38 as libc::c_int as yytype_int8,
    38 as libc::c_int as yytype_int8,
    38 as libc::c_int as yytype_int8,
    39 as libc::c_int as yytype_int8,
    40 as libc::c_int as yytype_int8,
    40 as libc::c_int as yytype_int8,
    41 as libc::c_int as yytype_int8,
    41 as libc::c_int as yytype_int8,
    41 as libc::c_int as yytype_int8,
    41 as libc::c_int as yytype_int8,
    41 as libc::c_int as yytype_int8,
    41 as libc::c_int as yytype_int8,
    41 as libc::c_int as yytype_int8,
    41 as libc::c_int as yytype_int8,
    42 as libc::c_int as yytype_int8,
    42 as libc::c_int as yytype_int8,
    43 as libc::c_int as yytype_int8,
    43 as libc::c_int as yytype_int8,
    43 as libc::c_int as yytype_int8,
    43 as libc::c_int as yytype_int8,
    43 as libc::c_int as yytype_int8,
    43 as libc::c_int as yytype_int8,
    44 as libc::c_int as yytype_int8,
    44 as libc::c_int as yytype_int8,
    45 as libc::c_int as yytype_int8,
    45 as libc::c_int as yytype_int8,
    46 as libc::c_int as yytype_int8,
    46 as libc::c_int as yytype_int8,
    47 as libc::c_int as yytype_int8,
    47 as libc::c_int as yytype_int8,
    48 as libc::c_int as yytype_int8,
    48 as libc::c_int as yytype_int8,
    49 as libc::c_int as yytype_int8,
    49 as libc::c_int as yytype_int8,
    49 as libc::c_int as yytype_int8,
    49 as libc::c_int as yytype_int8,
    50 as libc::c_int as yytype_int8,
    50 as libc::c_int as yytype_int8,
    50 as libc::c_int as yytype_int8,
    50 as libc::c_int as yytype_int8,
    50 as libc::c_int as yytype_int8,
    50 as libc::c_int as yytype_int8,
    50 as libc::c_int as yytype_int8,
    50 as libc::c_int as yytype_int8,
    50 as libc::c_int as yytype_int8,
    50 as libc::c_int as yytype_int8,
    50 as libc::c_int as yytype_int8,
    50 as libc::c_int as yytype_int8,
    50 as libc::c_int as yytype_int8,
    50 as libc::c_int as yytype_int8,
    50 as libc::c_int as yytype_int8,
    50 as libc::c_int as yytype_int8,
    50 as libc::c_int as yytype_int8,
    50 as libc::c_int as yytype_int8,
    50 as libc::c_int as yytype_int8,
    50 as libc::c_int as yytype_int8,
    50 as libc::c_int as yytype_int8,
    50 as libc::c_int as yytype_int8,
    51 as libc::c_int as yytype_int8,
    51 as libc::c_int as yytype_int8,
    51 as libc::c_int as yytype_int8,
    51 as libc::c_int as yytype_int8,
    51 as libc::c_int as yytype_int8,
    52 as libc::c_int as yytype_int8,
    52 as libc::c_int as yytype_int8,
    53 as libc::c_int as yytype_int8,
    53 as libc::c_int as yytype_int8,
    54 as libc::c_int as yytype_int8,
    54 as libc::c_int as yytype_int8,
    55 as libc::c_int as yytype_int8,
    55 as libc::c_int as yytype_int8,
    56 as libc::c_int as yytype_int8,
    56 as libc::c_int as yytype_int8,
    57 as libc::c_int as yytype_int8,
    57 as libc::c_int as yytype_int8,
    57 as libc::c_int as yytype_int8,
    58 as libc::c_int as yytype_int8,
    58 as libc::c_int as yytype_int8,
    59 as libc::c_int as yytype_int8,
    59 as libc::c_int as yytype_int8,
    60 as libc::c_int as yytype_int8,
    61 as libc::c_int as yytype_int8,
];
static mut def_r2: [yytype_int8; 100] = [
    0 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
];
#[no_mangle]
pub static mut diropts: [C2RustUnnamed_0; 6] = [
    {
        let mut init = C2RustUnnamed_0 {
            param: b"-heap\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            token: 262 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            param: b"-stack\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            token: 261 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            param: b"-attr\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            token: 266 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            param: b"-export\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            token: 267 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            param: b"-aligncomm\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            token: 275 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            param: 0 as *const libc::c_char as *mut libc::c_char,
            token: 0 as libc::c_int,
        };
        init
    },
];
static mut buffer: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut buflen: libc::c_int = 0 as libc::c_int;
static mut bufptr: libc::c_int = 0 as libc::c_int;
unsafe extern "C" fn put_buf(mut c: libc::c_char) {
    if bufptr == buflen {
        buflen += 50 as libc::c_int;
        if !buffer.is_null() {
            buffer = xrealloc(
                buffer as *mut libc::c_void,
                (buflen + 1 as libc::c_int) as size_t,
            ) as *mut libc::c_char;
        } else {
            buffer = xmalloc((buflen + 1 as libc::c_int) as size_t) as *mut libc::c_char;
        }
    }
    let fresh0 = bufptr;
    bufptr = bufptr + 1;
    *buffer.offset(fresh0 as isize) = c;
    *buffer.offset(bufptr as isize) = 0 as libc::c_int as libc::c_char;
}
static mut tokens: [C2RustUnnamed_1; 26] = [
    {
        let mut init = C2RustUnnamed_1 {
            name: b"BASE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            token: 270 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            name: b"CODE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            token: 263 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            name: b"CONSTANT\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            token: 271 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            name: b"constant\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            token: 272 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            name: b"DATA\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            token: 264 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            name: b"data\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            token: 265 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            name: b"DESCRIPTION\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            token: 260 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            name: b"DIRECTIVE\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            token: 282 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            name: b"EXECUTE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            token: 278 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            name: b"EXPORTS\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            token: 267 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            name: b"HEAPSIZE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            token: 262 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            name: b"IMPORTS\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            token: 268 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            name: b"LIBRARY\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            token: 259 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            name: b"NAME\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            token: 258 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            name: b"NONAME\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            token: 280 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            name: b"noname\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            token: 281 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            name: b"PRIVATE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            token: 273 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            name: b"private\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            token: 274 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            name: b"READ\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            token: 276 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            name: b"SECTIONS\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            token: 266 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            name: b"SEGMENTS\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            token: 266 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            name: b"SHARED\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            token: 279 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            name: b"STACKSIZE\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            token: 261 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            name: b"VERSION\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            token: 269 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            name: b"WRITE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            token: 277 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            name: 0 as *const libc::c_char as *mut libc::c_char,
            token: 0 as libc::c_int,
        };
        init
    },
];
unsafe extern "C" fn yydestruct(
    mut yymsg: *const libc::c_char,
    mut _yytype: libc::c_int,
    mut _yyvaluep: *mut YYSTYPE,
) {
    if yymsg.is_null() {
        yymsg = b"Deleting\0" as *const u8 as *const libc::c_char;
    }
}
#[no_mangle]
pub static mut def_char: libc::c_int = 0;
#[no_mangle]
pub static mut def_nerrs: libc::c_int = 0;
unsafe extern "C" fn def_getc() -> libc::c_int {
    let mut rv: libc::c_int = 0;
    if !lex_parse_string.is_null() {
        if lex_parse_string >= lex_parse_string_end {
            rv = -(1 as libc::c_int);
        } else {
            let fresh1 = lex_parse_string;
            lex_parse_string = lex_parse_string.offset(1);
            rv = *fresh1 as libc::c_int;
        }
    } else {
        rv = fgetc(the_file);
    }
    if rv == '\n' as i32 {
        saw_newline = 1 as libc::c_int;
    }
    return rv;
}
unsafe extern "C" fn def_ungetc(mut c: libc::c_int) -> libc::c_int {
    if !lex_parse_string.is_null() {
        lex_parse_string = lex_parse_string.offset(-1);
        lex_parse_string;
        return c;
    } else {
        return ungetc(c, the_file)
    };
}
