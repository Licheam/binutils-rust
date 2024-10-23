extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn memmove(
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
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn strtoul(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_ulong;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn lseek(__fd: libc::c_int, __offset: __off_t, __whence: libc::c_int) -> __off_t;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
}
pub type size_t = libc::c_ulong;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
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
pub type off_t = __off_t;
pub type ssize_t = __ssize_t;
pub type uint64_t = __uint64_t;
pub type ld_plugin_status = libc::c_uint;
pub const LDPS_ERR: ld_plugin_status = 3;
pub const LDPS_BAD_HANDLE: ld_plugin_status = 2;
pub const LDPS_NO_SYMS: ld_plugin_status = 1;
pub const LDPS_OK: ld_plugin_status = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ld_plugin_input_file {
    pub name: *const libc::c_char,
    pub fd: libc::c_int,
    pub offset: off_t,
    pub filesize: off_t,
    pub handle: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ld_plugin_symbol {
    pub name: *mut libc::c_char,
    pub version: *mut libc::c_char,
    pub def: libc::c_char,
    pub symbol_type: libc::c_char,
    pub section_kind: libc::c_char,
    pub unused: libc::c_char,
    pub visibility: libc::c_int,
    pub size: uint64_t,
    pub comdat_key: *mut libc::c_char,
    pub resolution: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ld_plugin_section {
    pub handle: *const libc::c_void,
    pub shndx: libc::c_uint,
}
pub type ld_plugin_claim_file_handler = Option::<
    unsafe extern "C" fn(
        *const ld_plugin_input_file,
        *mut libc::c_int,
    ) -> ld_plugin_status,
>;
pub type ld_plugin_all_symbols_read_handler = Option::<
    unsafe extern "C" fn() -> ld_plugin_status,
>;
pub type ld_plugin_cleanup_handler = Option::<
    unsafe extern "C" fn() -> ld_plugin_status,
>;
pub type ld_plugin_register_claim_file = Option::<
    unsafe extern "C" fn(ld_plugin_claim_file_handler) -> ld_plugin_status,
>;
pub type ld_plugin_register_all_symbols_read = Option::<
    unsafe extern "C" fn(ld_plugin_all_symbols_read_handler) -> ld_plugin_status,
>;
pub type ld_plugin_register_cleanup = Option::<
    unsafe extern "C" fn(ld_plugin_cleanup_handler) -> ld_plugin_status,
>;
pub type ld_plugin_add_symbols = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        libc::c_int,
        *const ld_plugin_symbol,
    ) -> ld_plugin_status,
>;
pub type ld_plugin_get_input_file = Option::<
    unsafe extern "C" fn(
        *const libc::c_void,
        *mut ld_plugin_input_file,
    ) -> ld_plugin_status,
>;
pub type ld_plugin_get_view = Option::<
    unsafe extern "C" fn(
        *const libc::c_void,
        *mut *const libc::c_void,
    ) -> ld_plugin_status,
>;
pub type ld_plugin_release_input_file = Option::<
    unsafe extern "C" fn(*const libc::c_void) -> ld_plugin_status,
>;
pub type ld_plugin_get_symbols = Option::<
    unsafe extern "C" fn(
        *const libc::c_void,
        libc::c_int,
        *mut ld_plugin_symbol,
    ) -> ld_plugin_status,
>;
pub type ld_plugin_add_input_file = Option::<
    unsafe extern "C" fn(*const libc::c_char) -> ld_plugin_status,
>;
pub type ld_plugin_add_input_library = Option::<
    unsafe extern "C" fn(*const libc::c_char) -> ld_plugin_status,
>;
pub type ld_plugin_set_extra_library_path = Option::<
    unsafe extern "C" fn(*const libc::c_char) -> ld_plugin_status,
>;
pub type ld_plugin_message = Option::<
    unsafe extern "C" fn(libc::c_int, *const libc::c_char, ...) -> ld_plugin_status,
>;
pub type ld_plugin_get_input_section_count = Option::<
    unsafe extern "C" fn(*const libc::c_void, *mut libc::c_uint) -> ld_plugin_status,
>;
pub type ld_plugin_get_input_section_type = Option::<
    unsafe extern "C" fn(ld_plugin_section, *mut libc::c_uint) -> ld_plugin_status,
>;
pub type ld_plugin_get_input_section_name = Option::<
    unsafe extern "C" fn(ld_plugin_section, *mut *mut libc::c_char) -> ld_plugin_status,
>;
pub type ld_plugin_get_input_section_contents = Option::<
    unsafe extern "C" fn(
        ld_plugin_section,
        *mut *const libc::c_uchar,
        *mut size_t,
    ) -> ld_plugin_status,
>;
pub type ld_plugin_update_section_order = Option::<
    unsafe extern "C" fn(*const ld_plugin_section, libc::c_uint) -> ld_plugin_status,
>;
pub type ld_plugin_allow_section_ordering = Option::<
    unsafe extern "C" fn() -> ld_plugin_status,
>;
pub type ld_plugin_allow_unique_segment_for_sections = Option::<
    unsafe extern "C" fn() -> ld_plugin_status,
>;
pub type ld_plugin_unique_segment_for_sections = Option::<
    unsafe extern "C" fn(
        *const libc::c_char,
        uint64_t,
        uint64_t,
        *const ld_plugin_section,
        libc::c_uint,
    ) -> ld_plugin_status,
>;
pub type ld_plugin_get_input_section_alignment = Option::<
    unsafe extern "C" fn(ld_plugin_section, *mut libc::c_uint) -> ld_plugin_status,
>;
pub type ld_plugin_get_input_section_size = Option::<
    unsafe extern "C" fn(ld_plugin_section, *mut uint64_t) -> ld_plugin_status,
>;
pub type ld_plugin_new_input_handler = Option::<
    unsafe extern "C" fn(*const ld_plugin_input_file) -> ld_plugin_status,
>;
pub type ld_plugin_register_new_input = Option::<
    unsafe extern "C" fn(ld_plugin_new_input_handler) -> ld_plugin_status,
>;
pub type ld_plugin_get_wrap_symbols = Option::<
    unsafe extern "C" fn(
        *mut uint64_t,
        *mut *mut *const libc::c_char,
    ) -> ld_plugin_status,
>;
pub type ld_plugin_level = libc::c_uint;
pub const LDPL_FATAL: ld_plugin_level = 3;
pub const LDPL_ERROR: ld_plugin_level = 2;
pub const LDPL_WARNING: ld_plugin_level = 1;
pub const LDPL_INFO: ld_plugin_level = 0;
pub type ld_plugin_tag = libc::c_uint;
pub const LDPT_ADD_SYMBOLS_V2: ld_plugin_tag = 33;
pub const LDPT_GET_WRAP_SYMBOLS: ld_plugin_tag = 32;
pub const LDPT_REGISTER_NEW_INPUT_HOOK: ld_plugin_tag = 31;
pub const LDPT_GET_INPUT_SECTION_SIZE: ld_plugin_tag = 30;
pub const LDPT_GET_INPUT_SECTION_ALIGNMENT: ld_plugin_tag = 29;
pub const LDPT_GET_SYMBOLS_V3: ld_plugin_tag = 28;
pub const LDPT_UNIQUE_SEGMENT_FOR_SECTIONS: ld_plugin_tag = 27;
pub const LDPT_ALLOW_UNIQUE_SEGMENT_FOR_SECTIONS: ld_plugin_tag = 26;
pub const LDPT_GET_SYMBOLS_V2: ld_plugin_tag = 25;
pub const LDPT_ALLOW_SECTION_ORDERING: ld_plugin_tag = 24;
pub const LDPT_UPDATE_SECTION_ORDER: ld_plugin_tag = 23;
pub const LDPT_GET_INPUT_SECTION_CONTENTS: ld_plugin_tag = 22;
pub const LDPT_GET_INPUT_SECTION_NAME: ld_plugin_tag = 21;
pub const LDPT_GET_INPUT_SECTION_TYPE: ld_plugin_tag = 20;
pub const LDPT_GET_INPUT_SECTION_COUNT: ld_plugin_tag = 19;
pub const LDPT_GET_VIEW: ld_plugin_tag = 18;
pub const LDPT_GNU_LD_VERSION: ld_plugin_tag = 17;
pub const LDPT_SET_EXTRA_LIBRARY_PATH: ld_plugin_tag = 16;
pub const LDPT_OUTPUT_NAME: ld_plugin_tag = 15;
pub const LDPT_ADD_INPUT_LIBRARY: ld_plugin_tag = 14;
pub const LDPT_RELEASE_INPUT_FILE: ld_plugin_tag = 13;
pub const LDPT_GET_INPUT_FILE: ld_plugin_tag = 12;
pub const LDPT_MESSAGE: ld_plugin_tag = 11;
pub const LDPT_ADD_INPUT_FILE: ld_plugin_tag = 10;
pub const LDPT_GET_SYMBOLS: ld_plugin_tag = 9;
pub const LDPT_ADD_SYMBOLS: ld_plugin_tag = 8;
pub const LDPT_REGISTER_CLEANUP_HOOK: ld_plugin_tag = 7;
pub const LDPT_REGISTER_ALL_SYMBOLS_READ_HOOK: ld_plugin_tag = 6;
pub const LDPT_REGISTER_CLAIM_FILE_HOOK: ld_plugin_tag = 5;
pub const LDPT_OPTION: ld_plugin_tag = 4;
pub const LDPT_LINKER_OUTPUT: ld_plugin_tag = 3;
pub const LDPT_GOLD_VERSION: ld_plugin_tag = 2;
pub const LDPT_API_VERSION: ld_plugin_tag = 1;
pub const LDPT_NULL: ld_plugin_tag = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ld_plugin_tv {
    pub tv_tag: ld_plugin_tag,
    pub tv_u: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub tv_val: libc::c_int,
    pub tv_string: *const libc::c_char,
    pub tv_register_claim_file: ld_plugin_register_claim_file,
    pub tv_register_all_symbols_read: ld_plugin_register_all_symbols_read,
    pub tv_register_cleanup: ld_plugin_register_cleanup,
    pub tv_add_symbols: ld_plugin_add_symbols,
    pub tv_get_symbols: ld_plugin_get_symbols,
    pub tv_add_input_file: ld_plugin_add_input_file,
    pub tv_message: ld_plugin_message,
    pub tv_get_input_file: ld_plugin_get_input_file,
    pub tv_get_view: ld_plugin_get_view,
    pub tv_release_input_file: ld_plugin_release_input_file,
    pub tv_add_input_library: ld_plugin_add_input_library,
    pub tv_set_extra_library_path: ld_plugin_set_extra_library_path,
    pub tv_get_input_section_count: ld_plugin_get_input_section_count,
    pub tv_get_input_section_type: ld_plugin_get_input_section_type,
    pub tv_get_input_section_name: ld_plugin_get_input_section_name,
    pub tv_get_input_section_contents: ld_plugin_get_input_section_contents,
    pub tv_update_section_order: ld_plugin_update_section_order,
    pub tv_allow_section_ordering: ld_plugin_allow_section_ordering,
    pub tv_allow_unique_segment_for_sections: ld_plugin_allow_unique_segment_for_sections,
    pub tv_unique_segment_for_sections: ld_plugin_unique_segment_for_sections,
    pub tv_get_input_section_alignment: ld_plugin_get_input_section_alignment,
    pub tv_get_input_section_size: ld_plugin_get_input_section_size,
    pub tv_register_new_input: ld_plugin_register_new_input,
    pub tv_get_wrap_symbols: ld_plugin_get_wrap_symbols,
}
pub type C2RustUnnamed_0 = libc::c_uint;
pub const _ISalnum: C2RustUnnamed_0 = 8;
pub const _ISpunct: C2RustUnnamed_0 = 4;
pub const _IScntrl: C2RustUnnamed_0 = 2;
pub const _ISblank: C2RustUnnamed_0 = 1;
pub const _ISgraph: C2RustUnnamed_0 = 32768;
pub const _ISprint: C2RustUnnamed_0 = 16384;
pub const _ISspace: C2RustUnnamed_0 = 8192;
pub const _ISxdigit: C2RustUnnamed_0 = 4096;
pub const _ISdigit: C2RustUnnamed_0 = 2048;
pub const _ISalpha: C2RustUnnamed_0 = 1024;
pub const _ISlower: C2RustUnnamed_0 = 512;
pub const _ISupper: C2RustUnnamed_0 = 256;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linerec {
    pub next: *mut linerec,
    pub line: [libc::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct arhdr {
    pub ar_name: [libc::c_char; 16],
    pub ar_date: [libc::c_char; 12],
    pub ar_uid: [libc::c_char; 6],
    pub ar_gid: [libc::c_char; 6],
    pub ar_mode: [libc::c_char; 8],
    pub ar_size: [libc::c_char; 10],
    pub ar_fmag: [libc::c_char; 2],
}
#[no_mangle]
pub unsafe extern "C" fn onload(mut tv: *mut ld_plugin_tv) -> ld_plugin_status {
    let mut rv: ld_plugin_status = LDPS_OK;
    if tv.is_null() {
        return LDPS_ERR;
    }
    if (*tv.offset(0 as libc::c_int as isize)).tv_tag as libc::c_uint
        == LDPT_MESSAGE as libc::c_int as libc::c_uint
    {
        tv_message = (*tv.offset(0 as libc::c_int as isize)).tv_u.tv_message;
    }
    loop {
        rv = parse_tv_tag(tv);
        if rv as libc::c_uint != LDPS_OK as libc::c_int as libc::c_uint {
            return rv;
        }
        let fresh0 = tv;
        tv = tv.offset(1);
        if !((*fresh0).tv_tag as libc::c_uint
            != LDPT_NULL as libc::c_int as libc::c_uint)
        {
            break;
        }
    }
    if tv_register_claim_file.is_some() && tv_register_all_symbols_read.is_some()
        && tv_register_cleanup.is_some()
    {
        (Some(tv_register_claim_file.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            Some(
                onclaim_file
                    as unsafe extern "C" fn(
                        *const ld_plugin_input_file,
                        *mut libc::c_int,
                    ) -> ld_plugin_status,
            ),
        );
        (Some(tv_register_all_symbols_read.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(Some(onall_symbols_read as unsafe extern "C" fn() -> ld_plugin_status));
        (Some(tv_register_cleanup.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(Some(oncleanup as unsafe extern "C" fn() -> ld_plugin_status));
    }
    fflush(0 as *mut FILE);
    return LDPS_OK;
}
static mut tv_register_claim_file: ld_plugin_register_claim_file = None;
static mut tv_register_all_symbols_read: ld_plugin_register_all_symbols_read = None;
static mut tv_register_cleanup: ld_plugin_register_cleanup = None;
static mut tv_message: ld_plugin_message = None;
static mut tv_add_input_library: ld_plugin_add_input_library = None;
static mut tv_set_extra_library_path: ld_plugin_set_extra_library_path = None;
unsafe extern "C" fn parse_tv_tag(mut tv: *mut ld_plugin_tv) -> ld_plugin_status {
    match (*tv).tv_tag as libc::c_uint {
        5 => {
            tv_register_claim_file = (*tv).tv_u.tv_register_claim_file;
        }
        6 => {
            tv_register_all_symbols_read = (*tv).tv_u.tv_register_all_symbols_read;
        }
        7 => {
            tv_register_cleanup = (*tv).tv_u.tv_register_cleanup;
        }
        11 => {
            tv_message = (*tv).tv_u.tv_message;
        }
        14 => {
            tv_add_input_library = (*tv).tv_u.tv_add_input_library;
        }
        16 => {
            tv_set_extra_library_path = (*tv).tv_u.tv_set_extra_library_path;
        }
        _ => {}
    }
    return LDPS_OK;
}
static mut line_head: *mut linerec = 0 as *const linerec as *mut linerec;
static mut line_tail: *mut *mut linerec = unsafe {
    &line_head as *const *mut linerec as *mut *mut linerec
};
unsafe extern "C" fn get_libdeps(mut fd: libc::c_int) -> ld_plugin_status {
    let mut ah: arhdr = arhdr {
        ar_name: [0; 16],
        ar_date: [0; 12],
        ar_uid: [0; 6],
        ar_gid: [0; 6],
        ar_mode: [0; 8],
        ar_size: [0; 10],
        ar_fmag: [0; 2],
    };
    let mut len: libc::c_int = 0;
    let mut mlen: libc::c_ulong = 0;
    let mut lr: *mut linerec = 0 as *mut linerec;
    let mut rc: ld_plugin_status = LDPS_NO_SYMS;
    lseek(fd, 8 as libc::c_int as __off_t, 0 as libc::c_int);
    loop {
        len = read(
            fd,
            &mut ah as *mut arhdr as *mut libc::c_void,
            ::core::mem::size_of::<arhdr>() as libc::c_ulong,
        ) as libc::c_int;
        if len as libc::c_ulong != ::core::mem::size_of::<arhdr>() as libc::c_ulong {
            break;
        }
        mlen = strtoul(
            (ah.ar_size).as_mut_ptr(),
            0 as *mut *mut libc::c_char,
            10 as libc::c_int,
        );
        if mlen == 0
            || strncmp(
                (ah.ar_name).as_mut_ptr(),
                b"__.LIBDEP/ \0" as *const u8 as *const libc::c_char,
                (::core::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            ) != 0
        {
            lseek(fd, mlen as __off_t, 1 as libc::c_int);
        } else {
            lr = malloc(
                (::core::mem::size_of::<linerec>() as libc::c_ulong).wrapping_add(mlen),
            ) as *mut linerec;
            if lr.is_null() {
                return LDPS_ERR;
            }
            (*lr).next = 0 as *mut linerec;
            len = read(fd, ((*lr).line).as_mut_ptr() as *mut libc::c_void, mlen)
                as libc::c_int;
            *((*lr).line)
                .as_mut_ptr()
                .offset(
                    mlen.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                ) = '\0' as i32 as libc::c_char;
            *line_tail = lr;
            line_tail = &mut (*lr).next;
            rc = LDPS_OK;
            break;
        }
    }
    return rc;
}
unsafe extern "C" fn str2vec(mut in_0: *mut libc::c_char) -> *mut *mut libc::c_char {
    let mut res: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut first: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sq: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dq: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    end = in_0.offset(strlen(in_0) as isize);
    s = in_0;
    while *(*__ctype_b_loc()).offset(*s as libc::c_uchar as libc::c_int as isize)
        as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        s = s.offset(1);
        s;
    }
    first = s;
    i = 1 as libc::c_int;
    loop {
        s = strchr(s, ' ' as i32);
        if s.is_null() {
            break;
        }
        s = s.offset(1);
        s;
        i += 1;
        i;
    }
    res = malloc(
        ((i + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong),
    ) as *mut *mut libc::c_char;
    if res.is_null() {
        return res;
    }
    i = 0 as libc::c_int;
    sq = 0 as *mut libc::c_char;
    dq = 0 as *mut libc::c_char;
    let ref mut fresh1 = *res.offset(0 as libc::c_int as isize);
    *fresh1 = first;
    let mut current_block_48: u64;
    s = first;
    while *s != 0 {
        if *s as libc::c_int == '\\' as i32 {
            memmove(
                s as *mut libc::c_void,
                s.offset(1 as libc::c_int as isize) as *const libc::c_void,
                (end.offset_from(s) as libc::c_long - 1 as libc::c_int as libc::c_long)
                    as libc::c_ulong,
            );
            end = end.offset(-1);
            end;
        }
        if *(*__ctype_b_loc()).offset(*s as libc::c_uchar as libc::c_int as isize)
            as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            != 0
        {
            if !sq.is_null() || !dq.is_null() {
                current_block_48 = 3512920355445576850;
            } else {
                let fresh2 = s;
                s = s.offset(1);
                *fresh2 = '\0' as i32 as libc::c_char;
                while *(*__ctype_b_loc())
                    .offset(*s as libc::c_uchar as libc::c_int as isize) as libc::c_int
                    & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
                {
                    s = s.offset(1);
                    s;
                }
                if *s != 0 {
                    i += 1;
                    let ref mut fresh3 = *res.offset(i as isize);
                    *fresh3 = s;
                }
                current_block_48 = 13472856163611868459;
            }
        } else {
            current_block_48 = 13472856163611868459;
        }
        match current_block_48 {
            13472856163611868459 => {
                if *s as libc::c_int == '\'' as i32 && dq.is_null() {
                    if !sq.is_null() {
                        memmove(
                            sq as *mut libc::c_void,
                            sq.offset(1 as libc::c_int as isize) as *const libc::c_void,
                            (s.offset_from(sq) as libc::c_long
                                - 1 as libc::c_int as libc::c_long) as libc::c_ulong,
                        );
                        memmove(
                            s.offset(-(2 as libc::c_int as isize)) as *mut libc::c_void,
                            s.offset(1 as libc::c_int as isize) as *const libc::c_void,
                            (end.offset_from(s) as libc::c_long
                                - 1 as libc::c_int as libc::c_long) as libc::c_ulong,
                        );
                        end = end.offset(-(2 as libc::c_int as isize));
                        s = s.offset(-1);
                        s;
                        sq = 0 as *mut libc::c_char;
                    } else {
                        sq = s;
                    }
                }
                if *s as libc::c_int == '"' as i32 && sq.is_null() {
                    if !dq.is_null() {
                        memmove(
                            dq as *mut libc::c_void,
                            dq.offset(1 as libc::c_int as isize) as *const libc::c_void,
                            (s.offset_from(dq) as libc::c_long
                                - 1 as libc::c_int as libc::c_long) as libc::c_ulong,
                        );
                        memmove(
                            s.offset(-(2 as libc::c_int as isize)) as *mut libc::c_void,
                            s.offset(1 as libc::c_int as isize) as *const libc::c_void,
                            (end.offset_from(s) as libc::c_long
                                - 1 as libc::c_int as libc::c_long) as libc::c_ulong,
                        );
                        end = end.offset(-(2 as libc::c_int as isize));
                        s = s.offset(-1);
                        s;
                        dq = 0 as *mut libc::c_char;
                    } else {
                        dq = s;
                    }
                }
            }
            _ => {}
        }
        s = s.offset(1);
        s;
    }
    i += 1;
    let ref mut fresh4 = *res.offset(i as isize);
    *fresh4 = 0 as *mut libc::c_char;
    return res;
}
static mut prevfile: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
unsafe extern "C" fn onclaim_file(
    mut file: *const ld_plugin_input_file,
    mut claimed: *mut libc::c_int,
) -> ld_plugin_status {
    let mut rv: ld_plugin_status = LDPS_OK;
    *claimed = 0 as libc::c_int;
    if !prevfile.is_null() && strcmp((*file).name, prevfile) == 0 {
        return LDPS_OK;
    }
    if (*file).offset == 0 {
        return LDPS_OK;
    }
    if !prevfile.is_null() {
        free(prevfile as *mut libc::c_void);
    }
    prevfile = strdup((*file).name);
    if prevfile.is_null() {
        return LDPS_ERR;
    }
    rv = get_libdeps((*file).fd);
    if rv as libc::c_uint == LDPS_ERR as libc::c_int as libc::c_uint {
        return rv;
    }
    if rv as libc::c_uint == LDPS_OK as libc::c_int as libc::c_uint {
        let mut lr: *mut linerec = line_tail as *mut linerec;
        if tv_message.is_some() {
            (Some(tv_message.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                LDPL_INFO as libc::c_int,
                b"got deps for library %s: %s\0" as *const u8 as *const libc::c_char,
                (*file).name,
                ((*lr).line).as_mut_ptr(),
            );
        }
        fflush(0 as *mut FILE);
    }
    return LDPS_OK;
}
unsafe extern "C" fn onall_symbols_read() -> ld_plugin_status {
    let mut lr: *mut linerec = 0 as *mut linerec;
    let mut vec: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut rv: ld_plugin_status = LDPS_OK;
    loop {
        lr = line_head;
        if lr.is_null() {
            break;
        }
        line_head = (*lr).next;
        vec = str2vec(((*lr).line).as_mut_ptr());
        if !vec.is_null() {
            let mut i: libc::c_int = 0;
            i = 0 as libc::c_int;
            while !(*vec.offset(i as isize)).is_null() {
                if *(*vec.offset(i as isize)).offset(0 as libc::c_int as isize)
                    as libc::c_int != '-' as i32
                {
                    if tv_message.is_some() {
                        (Some(tv_message.expect("non-null function pointer")))
                            .expect(
                                "non-null function pointer",
                            )(
                            LDPL_WARNING as libc::c_int,
                            b"ignoring libdep argument %s\0" as *const u8
                                as *const libc::c_char,
                            *vec.offset(i as isize),
                        );
                    }
                    fflush(0 as *mut FILE);
                } else {
                    if *(*vec.offset(i as isize)).offset(1 as libc::c_int as isize)
                        as libc::c_int == 'l' as i32
                    {
                        rv = tv_add_input_library
                            .expect(
                                "non-null function pointer",
                            )(
                            (*vec.offset(i as isize)).offset(2 as libc::c_int as isize),
                        );
                    } else if *(*vec.offset(i as isize))
                        .offset(1 as libc::c_int as isize) as libc::c_int == 'L' as i32
                    {
                        rv = tv_set_extra_library_path
                            .expect(
                                "non-null function pointer",
                            )(
                            (*vec.offset(i as isize)).offset(2 as libc::c_int as isize),
                        );
                    } else {
                        if tv_message.is_some() {
                            (Some(tv_message.expect("non-null function pointer")))
                                .expect(
                                    "non-null function pointer",
                                )(
                                LDPL_WARNING as libc::c_int,
                                b"ignoring libdep argument %s\0" as *const u8
                                    as *const libc::c_char,
                                *vec.offset(i as isize),
                            );
                        }
                        fflush(0 as *mut FILE);
                    }
                    if rv as libc::c_uint != LDPS_OK as libc::c_int as libc::c_uint {
                        break;
                    }
                }
                i += 1;
                i;
            }
            free(vec as *mut libc::c_void);
        }
        free(lr as *mut libc::c_void);
    }
    line_tail = 0 as *mut *mut linerec;
    return rv;
}
unsafe extern "C" fn oncleanup() -> ld_plugin_status {
    if !prevfile.is_null() {
        free(prevfile as *mut libc::c_void);
        prevfile = 0 as *mut libc::c_char;
    }
    if !line_head.is_null() {
        let mut lr: *mut linerec = 0 as *mut linerec;
        loop {
            lr = line_head;
            if lr.is_null() {
                break;
            }
            line_head = (*lr).next;
            free(lr as *mut libc::c_void);
        }
        line_tail = 0 as *mut *mut linerec;
    }
    return LDPS_OK;
}
