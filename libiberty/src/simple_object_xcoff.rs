use ::libc;
extern "C" {
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn free(_: *mut libc::c_void);
    fn memcpy(
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
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn simple_object_internal_read(
        descriptor: libc::c_int,
        offset: off_t,
        buffer: *mut libc::c_uchar,
        size: size_t,
        errmsg: *mut *const libc::c_char,
        err: *mut libc::c_int,
    ) -> libc::c_int;
    fn simple_object_internal_write(
        descriptor: libc::c_int,
        offset: off_t,
        buffer: *const libc::c_uchar,
        size: size_t,
        errmsg: *mut *const libc::c_char,
        err: *mut libc::c_int,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type off_t = __off_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct simple_object_read_struct {
    pub descriptor: libc::c_int,
    pub offset: off_t,
    pub functions: *const simple_object_functions,
    pub data: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct simple_object_functions {
    pub match_0: Option::<
        unsafe extern "C" fn(
            *mut libc::c_uchar,
            libc::c_int,
            off_t,
            *const libc::c_char,
            *mut *const libc::c_char,
            *mut libc::c_int,
        ) -> *mut libc::c_void,
    >,
    pub find_sections: Option::<
        unsafe extern "C" fn(
            *mut simple_object_read,
            Option::<
                unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const libc::c_char,
                    off_t,
                    off_t,
                ) -> libc::c_int,
            >,
            *mut libc::c_void,
            *mut libc::c_int,
        ) -> *const libc::c_char,
    >,
    pub fetch_attributes: Option::<
        unsafe extern "C" fn(
            *mut simple_object_read,
            *mut *const libc::c_char,
            *mut libc::c_int,
        ) -> *mut libc::c_void,
    >,
    pub release_read: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub attributes_merge: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut libc::c_void,
            *mut libc::c_int,
        ) -> *const libc::c_char,
    >,
    pub release_attributes: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub start_write: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut *const libc::c_char,
            *mut libc::c_int,
        ) -> *mut libc::c_void,
    >,
    pub write_to_file: Option::<
        unsafe extern "C" fn(
            *mut simple_object_write,
            libc::c_int,
            *mut libc::c_int,
        ) -> *const libc::c_char,
    >,
    pub release_write: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub copy_lto_debug_sections: Option::<
        unsafe extern "C" fn(
            *mut simple_object_read,
            *mut simple_object_write,
            Option::<unsafe extern "C" fn(*const libc::c_char) -> *mut libc::c_char>,
            *mut libc::c_int,
        ) -> *const libc::c_char,
    >,
}
pub type simple_object_write = simple_object_write_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct simple_object_write_struct {
    pub functions: *const simple_object_functions,
    pub segment_name: *mut libc::c_char,
    pub sections: *mut simple_object_write_section,
    pub last_section: *mut simple_object_write_section,
    pub data: *mut libc::c_void,
}
pub type simple_object_write_section = simple_object_write_section_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct simple_object_write_section_struct {
    pub next: *mut simple_object_write_section,
    pub name: *mut libc::c_char,
    pub align: libc::c_uint,
    pub buffers: *mut simple_object_write_section_buffer,
    pub last_buffer: *mut simple_object_write_section_buffer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct simple_object_write_section_buffer {
    pub next: *mut simple_object_write_section_buffer,
    pub size: size_t,
    pub buffer: *const libc::c_void,
    pub free_buffer: *mut libc::c_void,
}
pub type simple_object_read = simple_object_read_struct;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct external_filehdr {
    pub f_magic: [libc::c_uchar; 2],
    pub f_nscns: [libc::c_uchar; 2],
    pub f_timdat: [libc::c_uchar; 4],
    pub u: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub xcoff32: C2RustUnnamed_1,
    pub xcoff64: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub f_symptr: [libc::c_uchar; 8],
    pub f_opthdr: [libc::c_uchar; 2],
    pub f_flags: [libc::c_uchar; 2],
    pub f_nsyms: [libc::c_uchar; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub f_symptr: [libc::c_uchar; 4],
    pub f_nsyms: [libc::c_uchar; 4],
    pub f_opthdr: [libc::c_uchar; 2],
    pub f_flags: [libc::c_uchar; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct simple_object_xcoff_attributes {
    pub magic: libc::c_ushort,
    pub flags: libc::c_ushort,
}
pub type ulong_type = uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub sym: external_syment,
    pub aux: external_auxent,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union external_auxent {
    pub x_file: C2RustUnnamed_10,
    pub x_scn: C2RustUnnamed_9,
    pub u: C2RustUnnamed_4,
    pub x_sect: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub x_scnlen: [libc::c_uchar; 4],
    pub pad1: [libc::c_uchar; 4],
    pub x_nreloc: [libc::c_uchar; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub xcoff32: C2RustUnnamed_7,
    pub xcoff64: C2RustUnnamed_5,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub x_csect: C2RustUnnamed_6,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub x_scnlen_lo: [libc::c_uchar; 4],
    pub x_parmhash: [libc::c_uchar; 4],
    pub x_snhash: [libc::c_uchar; 2],
    pub x_smtyp: libc::c_uchar,
    pub x_smclas: libc::c_uchar,
    pub x_scnlen_hi: [libc::c_uchar; 4],
    pub pad: libc::c_uchar,
    pub x_auxtype: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub x_csect: C2RustUnnamed_8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub x_scnlen: [libc::c_uchar; 4],
    pub x_parmhash: [libc::c_uchar; 4],
    pub x_snhash: [libc::c_uchar; 2],
    pub x_smtyp: libc::c_uchar,
    pub x_smclas: libc::c_uchar,
    pub x_stab: libc::c_uchar,
    pub x_snstab: [libc::c_uchar; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub x_scnlen: [libc::c_uchar; 4],
    pub x_nreloc: [libc::c_uchar; 2],
    pub x_nlinno: [libc::c_uchar; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_10 {
    pub x_fname: [libc::c_char; 14],
    pub _x: C2RustUnnamed_11,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub x_zeroes: [libc::c_uchar; 4],
    pub x_offset: [libc::c_uchar; 4],
    pub x_pad: [libc::c_uchar; 6],
    pub x_ftype: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct external_syment {
    pub u: C2RustUnnamed_12,
    pub n_scnum: [libc::c_uchar; 2],
    pub n_type: [libc::c_uchar; 2],
    pub n_sclass: [libc::c_uchar; 1],
    pub n_numaux: [libc::c_uchar; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_12 {
    pub xcoff32: C2RustUnnamed_14,
    pub xcoff64: C2RustUnnamed_13,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_13 {
    pub n_value: [libc::c_uchar; 8],
    pub n_offset: [libc::c_uchar; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_14 {
    pub n: C2RustUnnamed_15,
    pub n_value: [libc::c_uchar; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_15 {
    pub n_name: [libc::c_char; 8],
    pub n: C2RustUnnamed_16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_16 {
    pub n_zeroes: [libc::c_uchar; 4],
    pub n_offset: [libc::c_uchar; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct simple_object_xcoff_read {
    pub magic: libc::c_ushort,
    pub nscns: libc::c_ushort,
    pub symptr: off_t,
    pub nsyms: libc::c_uint,
    pub flags: libc::c_ushort,
    pub scnhdr_offset: off_t,
}
#[inline]
unsafe extern "C" fn simple_object_fetch_big_16(
    mut buf: *const libc::c_uchar,
) -> libc::c_ushort {
    return ((*buf.offset(0 as libc::c_int as isize) as libc::c_ushort as libc::c_int)
        << 8 as libc::c_int
        | *buf.offset(1 as libc::c_int as isize) as libc::c_ushort as libc::c_int)
        as libc::c_ushort;
}
#[inline]
unsafe extern "C" fn simple_object_fetch_big_32(
    mut buf: *const libc::c_uchar,
) -> libc::c_uint {
    return (*buf.offset(0 as libc::c_int as isize) as libc::c_uint) << 24 as libc::c_int
        | (*buf.offset(1 as libc::c_int as isize) as libc::c_uint) << 16 as libc::c_int
        | (*buf.offset(2 as libc::c_int as isize) as libc::c_uint) << 8 as libc::c_int
        | *buf.offset(3 as libc::c_int as isize) as libc::c_uint;
}
#[inline]
unsafe extern "C" fn simple_object_fetch_big_64(
    mut buf: *const libc::c_uchar,
) -> ulong_type {
    return (*buf.offset(0 as libc::c_int as isize) as ulong_type) << 56 as libc::c_int
        | (*buf.offset(1 as libc::c_int as isize) as ulong_type) << 48 as libc::c_int
        | (*buf.offset(2 as libc::c_int as isize) as ulong_type) << 40 as libc::c_int
        | (*buf.offset(3 as libc::c_int as isize) as ulong_type) << 32 as libc::c_int
        | (*buf.offset(4 as libc::c_int as isize) as ulong_type) << 24 as libc::c_int
        | (*buf.offset(5 as libc::c_int as isize) as ulong_type) << 16 as libc::c_int
        | (*buf.offset(6 as libc::c_int as isize) as ulong_type) << 8 as libc::c_int
        | *buf.offset(7 as libc::c_int as isize) as ulong_type;
}
#[inline]
unsafe extern "C" fn simple_object_set_big_16(
    mut buf: *mut libc::c_uchar,
    mut val: libc::c_ushort,
) {
    *buf
        .offset(
            0 as libc::c_int as isize,
        ) = (val as libc::c_int >> 8 as libc::c_int & 0xff as libc::c_int)
        as libc::c_uchar;
    *buf
        .offset(
            1 as libc::c_int as isize,
        ) = (val as libc::c_int & 0xff as libc::c_int) as libc::c_uchar;
}
#[inline]
unsafe extern "C" fn simple_object_set_big_32(
    mut buf: *mut libc::c_uchar,
    mut val: libc::c_uint,
) {
    *buf
        .offset(
            0 as libc::c_int as isize,
        ) = (val >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as libc::c_uchar;
    *buf
        .offset(
            1 as libc::c_int as isize,
        ) = (val >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as libc::c_uchar;
    *buf
        .offset(
            2 as libc::c_int as isize,
        ) = (val >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as libc::c_uchar;
    *buf
        .offset(
            3 as libc::c_int as isize,
        ) = (val & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
}
#[inline]
unsafe extern "C" fn simple_object_set_big_64(
    mut buf: *mut libc::c_uchar,
    mut val: ulong_type,
) {
    *buf
        .offset(
            0 as libc::c_int as isize,
        ) = (val >> 56 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as libc::c_uchar;
    *buf
        .offset(
            1 as libc::c_int as isize,
        ) = (val >> 48 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as libc::c_uchar;
    *buf
        .offset(
            2 as libc::c_int as isize,
        ) = (val >> 40 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as libc::c_uchar;
    *buf
        .offset(
            3 as libc::c_int as isize,
        ) = (val >> 32 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as libc::c_uchar;
    *buf
        .offset(
            4 as libc::c_int as isize,
        ) = (val >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as libc::c_uchar;
    *buf
        .offset(
            5 as libc::c_int as isize,
        ) = (val >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as libc::c_uchar;
    *buf
        .offset(
            6 as libc::c_int as isize,
        ) = (val >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as libc::c_uchar;
    *buf
        .offset(
            7 as libc::c_int as isize,
        ) = (val & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
}
unsafe extern "C" fn simple_object_xcoff_match(
    mut header: *mut libc::c_uchar,
    mut descriptor: libc::c_int,
    mut offset: off_t,
    mut segment_name: *const libc::c_char,
    mut errmsg: *mut *const libc::c_char,
    mut err: *mut libc::c_int,
) -> *mut libc::c_void {
    let mut magic: libc::c_ushort = 0;
    let mut fetch_16: Option::<
        unsafe extern "C" fn(*const libc::c_uchar) -> libc::c_ushort,
    > = None;
    let mut fetch_32: Option::<
        unsafe extern "C" fn(*const libc::c_uchar) -> libc::c_uint,
    > = None;
    let mut fetch_64: Option::<
        unsafe extern "C" fn(*const libc::c_uchar) -> ulong_type,
    > = None;
    let mut hdrbuf: [libc::c_uchar; 24] = [0; 24];
    let mut ocr: *mut simple_object_xcoff_read = 0 as *mut simple_object_xcoff_read;
    let mut u64: libc::c_int = 0;
    magic = simple_object_fetch_big_16(header as *const libc::c_uchar);
    if magic as libc::c_int != 0o737 as libc::c_int
        && magic as libc::c_int != 0o767 as libc::c_int
    {
        *errmsg = 0 as *const libc::c_char;
        *err = 0 as libc::c_int;
        return 0 as *mut libc::c_void;
    }
    fetch_16 = Some(
        simple_object_fetch_big_16
            as unsafe extern "C" fn(*const libc::c_uchar) -> libc::c_ushort,
    );
    fetch_32 = Some(
        simple_object_fetch_big_32
            as unsafe extern "C" fn(*const libc::c_uchar) -> libc::c_uint,
    );
    fetch_64 = Some(
        simple_object_fetch_big_64
            as unsafe extern "C" fn(*const libc::c_uchar) -> ulong_type,
    );
    if simple_object_internal_read(
        descriptor,
        offset,
        hdrbuf.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_uchar; 24]>() as libc::c_ulong,
        errmsg,
        err,
    ) == 0
    {
        return 0 as *mut libc::c_void;
    }
    u64 = (magic as libc::c_int == 0o767 as libc::c_int) as libc::c_int;
    ocr = xmalloc(::core::mem::size_of::<simple_object_xcoff_read>() as libc::c_ulong)
        as *mut simple_object_xcoff_read;
    (*ocr).magic = magic;
    (*ocr)
        .nscns = fetch_16
        .expect(
            "non-null function pointer",
        )(hdrbuf.as_mut_ptr().offset(2 as libc::c_ulong as isize));
    if u64 != 0 {
        (*ocr)
            .symptr = fetch_64
            .expect(
                "non-null function pointer",
            )(hdrbuf.as_mut_ptr().offset(8 as libc::c_ulong as isize)) as off_t;
        (*ocr)
            .nsyms = fetch_32
            .expect(
                "non-null function pointer",
            )(hdrbuf.as_mut_ptr().offset(20 as libc::c_ulong as isize));
        (*ocr)
            .scnhdr_offset = (::core::mem::size_of::<external_filehdr>()
            as libc::c_ulong)
            .wrapping_add(
                fetch_16
                    .expect(
                        "non-null function pointer",
                    )(hdrbuf.as_mut_ptr().offset(16 as libc::c_ulong as isize))
                    as libc::c_ulong,
            ) as off_t;
    } else {
        (*ocr)
            .symptr = fetch_32
            .expect(
                "non-null function pointer",
            )(hdrbuf.as_mut_ptr().offset(8 as libc::c_ulong as isize)) as off_t;
        (*ocr)
            .nsyms = fetch_32
            .expect(
                "non-null function pointer",
            )(hdrbuf.as_mut_ptr().offset(12 as libc::c_ulong as isize));
        (*ocr)
            .scnhdr_offset = (::core::mem::size_of::<external_filehdr>()
            as libc::c_ulong)
            .wrapping_sub(4 as libc::c_int as libc::c_ulong)
            .wrapping_add(
                fetch_16
                    .expect(
                        "non-null function pointer",
                    )(hdrbuf.as_mut_ptr().offset(16 as libc::c_ulong as isize))
                    as libc::c_ulong,
            ) as off_t;
    }
    return ocr as *mut libc::c_void;
}
unsafe extern "C" fn simple_object_xcoff_read_strtab(
    mut sobj: *mut simple_object_read,
    mut strtab_size: *mut size_t,
    mut errmsg: *mut *const libc::c_char,
    mut err: *mut libc::c_int,
) -> *mut libc::c_char {
    let mut ocr: *mut simple_object_xcoff_read = (*sobj).data
        as *mut simple_object_xcoff_read;
    let mut strtab_offset: off_t = 0;
    let mut strsizebuf: [libc::c_uchar; 4] = [0; 4];
    let mut strsize: size_t = 0;
    let mut strtab: *mut libc::c_char = 0 as *mut libc::c_char;
    strtab_offset = (*sobj).offset + (*ocr).symptr
        + ((*ocr).nsyms).wrapping_mul(18 as libc::c_int as libc::c_uint) as libc::c_long;
    if simple_object_internal_read(
        (*sobj).descriptor,
        strtab_offset,
        strsizebuf.as_mut_ptr(),
        4 as libc::c_int as size_t,
        errmsg,
        err,
    ) == 0
    {
        return 0 as *mut libc::c_char;
    }
    strsize = simple_object_fetch_big_32(strsizebuf.as_mut_ptr()) as size_t;
    strtab = xmalloc(
        (::core::mem::size_of::<libc::c_char>() as libc::c_ulong).wrapping_mul(strsize),
    ) as *mut libc::c_char;
    if simple_object_internal_read(
        (*sobj).descriptor,
        strtab_offset,
        strtab as *mut libc::c_uchar,
        strsize,
        errmsg,
        err,
    ) == 0
    {
        free(strtab as *mut libc::c_void);
        return 0 as *mut libc::c_char;
    }
    *strtab_size = strsize;
    return strtab;
}
unsafe extern "C" fn simple_object_xcoff_find_sections(
    mut sobj: *mut simple_object_read,
    mut pfn: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const libc::c_char,
            off_t,
            off_t,
        ) -> libc::c_int,
    >,
    mut data: *mut libc::c_void,
    mut err: *mut libc::c_int,
) -> *const libc::c_char {
    let mut ocr: *mut simple_object_xcoff_read = (*sobj).data
        as *mut simple_object_xcoff_read;
    let mut u64: libc::c_int = ((*ocr).magic as libc::c_int == 0o767 as libc::c_int)
        as libc::c_int;
    let mut scnhdr_size: size_t = 0;
    let mut scnbuf: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut errmsg: *const libc::c_char = 0 as *const libc::c_char;
    let mut fetch_16: Option::<
        unsafe extern "C" fn(*const libc::c_uchar) -> libc::c_ushort,
    > = None;
    let mut fetch_32: Option::<
        unsafe extern "C" fn(*const libc::c_uchar) -> libc::c_uint,
    > = None;
    let mut fetch_64: Option::<
        unsafe extern "C" fn(*const libc::c_uchar) -> ulong_type,
    > = None;
    let mut nscns: libc::c_uint = 0;
    let mut strtab: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut strtab_size: size_t = 0;
    let mut symtab: *mut external_syment = 0 as *mut external_syment;
    let mut i: libc::c_uint = 0;
    scnhdr_size = (if u64 != 0 { 68 as libc::c_int } else { 40 as libc::c_int })
        as size_t;
    scnbuf = xmalloc(
        (::core::mem::size_of::<libc::c_uchar>() as libc::c_ulong)
            .wrapping_mul(scnhdr_size.wrapping_mul((*ocr).nscns as libc::c_ulong)),
    ) as *mut libc::c_uchar;
    if simple_object_internal_read(
        (*sobj).descriptor,
        (*sobj).offset + (*ocr).scnhdr_offset,
        scnbuf,
        scnhdr_size.wrapping_mul((*ocr).nscns as libc::c_ulong),
        &mut errmsg,
        err,
    ) == 0
    {
        free(scnbuf as *mut libc::c_void);
        return errmsg;
    }
    fetch_16 = Some(
        simple_object_fetch_big_16
            as unsafe extern "C" fn(*const libc::c_uchar) -> libc::c_ushort,
    );
    fetch_32 = Some(
        simple_object_fetch_big_32
            as unsafe extern "C" fn(*const libc::c_uchar) -> libc::c_uint,
    );
    fetch_64 = Some(
        simple_object_fetch_big_64
            as unsafe extern "C" fn(*const libc::c_uchar) -> ulong_type,
    );
    nscns = (*ocr).nscns as libc::c_uint;
    strtab = 0 as *mut libc::c_char;
    strtab_size = 0 as libc::c_int as size_t;
    i = 0 as libc::c_int as libc::c_uint;
    while i < nscns {
        let mut scnhdr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        let mut scnname: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        let mut namebuf: [libc::c_char; 9] = [0; 9];
        let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut scnptr: off_t = 0;
        let mut size: off_t = 0;
        scnhdr = scnbuf.offset((i as libc::c_ulong).wrapping_mul(scnhdr_size) as isize);
        scnname = scnhdr.offset(0 as libc::c_ulong as isize);
        memcpy(
            namebuf.as_mut_ptr() as *mut libc::c_void,
            scnname as *const libc::c_void,
            8 as libc::c_int as libc::c_ulong,
        );
        namebuf[8 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
        name = &mut *namebuf.as_mut_ptr().offset(0 as libc::c_int as isize)
            as *mut libc::c_char;
        if namebuf[0 as libc::c_int as usize] as libc::c_int == '/' as i32 {
            let mut strindex: size_t = 0;
            let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
            strindex = strtol(
                namebuf.as_mut_ptr().offset(1 as libc::c_int as isize),
                &mut end,
                10 as libc::c_int,
            ) as size_t;
            if *end as libc::c_int == '\0' as i32 {
                if strtab.is_null() {
                    strtab = simple_object_xcoff_read_strtab(
                        sobj,
                        &mut strtab_size,
                        &mut errmsg,
                        err,
                    );
                    if strtab.is_null() {
                        free(scnbuf as *mut libc::c_void);
                        return errmsg;
                    }
                }
                if strindex < 4 as libc::c_int as libc::c_ulong
                    || strindex >= strtab_size
                {
                    free(strtab as *mut libc::c_void);
                    free(scnbuf as *mut libc::c_void);
                    *err = 0 as libc::c_int;
                    return b"section string index out of range\0" as *const u8
                        as *const libc::c_char;
                }
                name = strtab.offset(strindex as isize);
            }
        }
        if u64 != 0 {
            scnptr = fetch_64
                .expect(
                    "non-null function pointer",
                )(scnhdr.offset(32 as libc::c_ulong as isize)) as off_t;
            size = fetch_64
                .expect(
                    "non-null function pointer",
                )(scnhdr.offset(24 as libc::c_ulong as isize)) as off_t;
        } else {
            scnptr = fetch_32
                .expect(
                    "non-null function pointer",
                )(scnhdr.offset(20 as libc::c_ulong as isize)) as off_t;
            size = fetch_32
                .expect(
                    "non-null function pointer",
                )(scnhdr.offset(16 as libc::c_ulong as isize)) as off_t;
        }
        if (Some(pfn.expect("non-null function pointer")))
            .expect("non-null function pointer")(data, name, scnptr, size) == 0
        {
            break;
        }
        i = i.wrapping_add(1);
        i;
    }
    if (*ocr).nsyms > 0 as libc::c_int as libc::c_uint {
        let mut sym: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        let mut n_name: *const libc::c_char = 0 as *const libc::c_char;
        let mut size_0: off_t = 0;
        let mut n_value: off_t = 0;
        let mut n_numaux: libc::c_uint = 0;
        let mut n_offset: libc::c_uint = 0;
        let mut n_zeroes: libc::c_uint = 0;
        let mut n_scnum: libc::c_short = 0;
        symtab = xmalloc(
            (::core::mem::size_of::<external_syment>() as libc::c_ulong)
                .wrapping_mul(
                    ((*ocr).nsyms).wrapping_mul(18 as libc::c_int as libc::c_uint)
                        as libc::c_ulong,
                ),
        ) as *mut external_syment;
        if simple_object_internal_read(
            (*sobj).descriptor,
            (*sobj).offset + (*ocr).symptr,
            symtab as *mut libc::c_uchar,
            ((*ocr).nsyms).wrapping_mul(18 as libc::c_int as libc::c_uint) as size_t,
            &mut errmsg,
            err,
        ) == 0
        {
            free(symtab as *mut libc::c_void);
            free(scnbuf as *mut libc::c_void);
            return 0 as *const libc::c_char;
        }
        let mut current_block_90: u64;
        i = 0 as libc::c_int as libc::c_uint;
        while i < (*ocr).nsyms {
            sym = &mut *symtab.offset(i as isize) as *mut external_syment
                as *mut libc::c_uchar;
            n_numaux = *((*symtab.offset(i as isize)).n_numaux)
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize) as libc::c_uint;
            if !((*symtab.offset(i as isize)).n_sclass[0 as libc::c_int as usize]
                as libc::c_int != 2 as libc::c_int
                && (*symtab.offset(i as isize)).n_sclass[0 as libc::c_int as usize]
                    as libc::c_int != 107 as libc::c_int)
            {
                if !(n_numaux < 1 as libc::c_int as libc::c_uint
                    || i.wrapping_add(n_numaux) >= (*ocr).nsyms)
                {
                    n_scnum = fetch_16
                        .expect(
                            "non-null function pointer",
                        )(sym.offset(12 as libc::c_ulong as isize)) as libc::c_short;
                    if !((n_scnum as libc::c_int) < 1 as libc::c_int
                        || n_scnum as libc::c_uint > nscns)
                    {
                        if u64 != 0 {
                            n_value = fetch_64
                                .expect(
                                    "non-null function pointer",
                                )(sym.offset(0 as libc::c_ulong as isize)) as off_t;
                            n_offset = fetch_32
                                .expect(
                                    "non-null function pointer",
                                )(sym.offset(8 as libc::c_ulong as isize));
                            current_block_90 = 17836213544692497527;
                        } else {
                            n_zeroes = fetch_32
                                .expect(
                                    "non-null function pointer",
                                )(sym.offset(0 as libc::c_ulong as isize));
                            if n_zeroes != 0 as libc::c_int as libc::c_uint {
                                current_block_90 = 2989495919056355252;
                            } else {
                                n_value = fetch_32
                                    .expect(
                                        "non-null function pointer",
                                    )(sym.offset(8 as libc::c_ulong as isize)) as off_t;
                                n_offset = fetch_32
                                    .expect(
                                        "non-null function pointer",
                                    )(sym.offset(4 as libc::c_ulong as isize));
                                current_block_90 = 17836213544692497527;
                            }
                        }
                        match current_block_90 {
                            2989495919056355252 => {}
                            _ => {
                                if strtab.is_null() {
                                    strtab = simple_object_xcoff_read_strtab(
                                        sobj,
                                        &mut strtab_size,
                                        &mut errmsg,
                                        err,
                                    );
                                    if strtab.is_null() {
                                        free(symtab as *mut libc::c_void);
                                        free(scnbuf as *mut libc::c_void);
                                        return errmsg;
                                    }
                                }
                                if n_offset as libc::c_ulong >= strtab_size {
                                    free(strtab as *mut libc::c_void);
                                    free(symtab as *mut libc::c_void);
                                    free(scnbuf as *mut libc::c_void);
                                    *err = 0 as libc::c_int;
                                    return b"symbol string index out of range\0" as *const u8
                                        as *const libc::c_char;
                                }
                                n_name = strtab.offset(n_offset as isize);
                                if strcmp(
                                    n_name,
                                    b".go_export\0" as *const u8 as *const libc::c_char,
                                ) == 0
                                {
                                    let mut auxent: *mut external_auxent = 0
                                        as *mut external_auxent;
                                    let mut aux: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                                    let mut scnhdr_0: *mut libc::c_uchar = 0
                                        as *mut libc::c_uchar;
                                    let mut scnptr_0: off_t = 0;
                                    let mut x_scnlen: off_t = 0;
                                    auxent = &mut *symtab
                                        .offset(i.wrapping_add(n_numaux) as isize)
                                        as *mut external_syment as *mut external_auxent;
                                    aux = auxent as *mut libc::c_uchar;
                                    if u64 != 0 {
                                        let mut x_scnlen64: ulong_type = 0;
                                        if (*auxent).u.xcoff64.x_csect.x_smtyp as libc::c_int
                                            & 0x7 as libc::c_int != 1 as libc::c_int
                                            || (*auxent).u.xcoff64.x_csect.x_smclas as libc::c_int
                                                != 7 as libc::c_int
                                        {
                                            current_block_90 = 2989495919056355252;
                                        } else {
                                            x_scnlen64 = fetch_32
                                                .expect(
                                                    "non-null function pointer",
                                                )(aux.offset(12 as libc::c_ulong as isize)) as ulong_type;
                                            x_scnlen = (x_scnlen64 << 32 as libc::c_int
                                                | fetch_32
                                                    .expect(
                                                        "non-null function pointer",
                                                    )(aux.offset(0 as libc::c_ulong as isize)) as libc::c_ulong)
                                                as off_t;
                                            current_block_90 = 993425571616822999;
                                        }
                                    } else if (*auxent).u.xcoff32.x_csect.x_smtyp as libc::c_int
                                        & 0x7 as libc::c_int != 1 as libc::c_int
                                        || (*auxent).u.xcoff32.x_csect.x_smclas as libc::c_int
                                            != 7 as libc::c_int
                                    {
                                        current_block_90 = 2989495919056355252;
                                    } else {
                                        x_scnlen = fetch_32
                                            .expect(
                                                "non-null function pointer",
                                            )(aux.offset(0 as libc::c_ulong as isize)) as off_t;
                                        current_block_90 = 993425571616822999;
                                    }
                                    match current_block_90 {
                                        2989495919056355252 => {}
                                        _ => {
                                            scnhdr_0 = scnbuf
                                                .offset(
                                                    ((n_scnum as libc::c_int - 1 as libc::c_int)
                                                        as libc::c_ulong)
                                                        .wrapping_mul(scnhdr_size) as isize,
                                                );
                                            if u64 != 0 {
                                                scnptr_0 = fetch_64
                                                    .expect(
                                                        "non-null function pointer",
                                                    )(scnhdr_0.offset(32 as libc::c_ulong as isize)) as off_t;
                                                size_0 = fetch_64
                                                    .expect(
                                                        "non-null function pointer",
                                                    )(scnhdr_0.offset(24 as libc::c_ulong as isize)) as off_t;
                                            } else {
                                                scnptr_0 = fetch_32
                                                    .expect(
                                                        "non-null function pointer",
                                                    )(scnhdr_0.offset(20 as libc::c_ulong as isize)) as off_t;
                                                size_0 = fetch_32
                                                    .expect(
                                                        "non-null function pointer",
                                                    )(scnhdr_0.offset(16 as libc::c_ulong as isize)) as off_t;
                                            }
                                            if n_value + x_scnlen > size_0 {
                                                break;
                                            }
                                            (Some(pfn.expect("non-null function pointer")))
                                                .expect(
                                                    "non-null function pointer",
                                                )(
                                                data,
                                                b".go_export\0" as *const u8 as *const libc::c_char,
                                                scnptr_0 + n_value,
                                                x_scnlen,
                                            );
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            i = i.wrapping_add(n_numaux.wrapping_add(1 as libc::c_int as libc::c_uint));
        }
    }
    if !symtab.is_null() {
        free(symtab as *mut libc::c_void);
    }
    if !strtab.is_null() {
        free(strtab as *mut libc::c_void);
    }
    free(scnbuf as *mut libc::c_void);
    return 0 as *const libc::c_char;
}
unsafe extern "C" fn simple_object_xcoff_fetch_attributes(
    mut sobj: *mut simple_object_read,
    mut errmsg: *mut *const libc::c_char,
    mut err: *mut libc::c_int,
) -> *mut libc::c_void {
    let mut ocr: *mut simple_object_xcoff_read = (*sobj).data
        as *mut simple_object_xcoff_read;
    let mut ret: *mut simple_object_xcoff_attributes = 0
        as *mut simple_object_xcoff_attributes;
    ret = xmalloc(
        ::core::mem::size_of::<simple_object_xcoff_attributes>() as libc::c_ulong,
    ) as *mut simple_object_xcoff_attributes;
    (*ret).magic = (*ocr).magic;
    (*ret).flags = (*ocr).flags;
    return ret as *mut libc::c_void;
}
unsafe extern "C" fn simple_object_xcoff_release_read(mut data: *mut libc::c_void) {
    free(data);
}
unsafe extern "C" fn simple_object_xcoff_attributes_merge(
    mut todata: *mut libc::c_void,
    mut fromdata: *mut libc::c_void,
    mut err: *mut libc::c_int,
) -> *const libc::c_char {
    let mut to: *mut simple_object_xcoff_attributes = todata
        as *mut simple_object_xcoff_attributes;
    let mut from: *mut simple_object_xcoff_attributes = fromdata
        as *mut simple_object_xcoff_attributes;
    if (*to).magic as libc::c_int != (*from).magic as libc::c_int {
        *err = 0 as libc::c_int;
        return b"XCOFF object format mismatch\0" as *const u8 as *const libc::c_char;
    }
    return 0 as *const libc::c_char;
}
unsafe extern "C" fn simple_object_xcoff_release_attributes(
    mut data: *mut libc::c_void,
) {
    free(data);
}
unsafe extern "C" fn simple_object_xcoff_start_write(
    mut attributes_data: *mut libc::c_void,
    mut errmsg: *mut *const libc::c_char,
    mut err: *mut libc::c_int,
) -> *mut libc::c_void {
    let mut attrs: *mut simple_object_xcoff_attributes = attributes_data
        as *mut simple_object_xcoff_attributes;
    let mut ret: *mut simple_object_xcoff_attributes = 0
        as *mut simple_object_xcoff_attributes;
    ret = xmalloc(
        ::core::mem::size_of::<simple_object_xcoff_attributes>() as libc::c_ulong,
    ) as *mut simple_object_xcoff_attributes;
    *ret = *attrs;
    return ret as *mut libc::c_void;
}
unsafe extern "C" fn simple_object_xcoff_write_filehdr(
    mut sobj: *mut simple_object_write,
    mut descriptor: libc::c_int,
    mut nscns: libc::c_uint,
    mut symtab_offset: size_t,
    mut nsyms: libc::c_uint,
    mut errmsg: *mut *const libc::c_char,
    mut err: *mut libc::c_int,
) -> libc::c_int {
    let mut attrs: *mut simple_object_xcoff_attributes = (*sobj).data
        as *mut simple_object_xcoff_attributes;
    let mut u64: libc::c_int = ((*attrs).magic as libc::c_int == 0o767 as libc::c_int)
        as libc::c_int;
    let mut hdrbuf: [libc::c_uchar; 24] = [0; 24];
    let mut hdr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut set_16: Option::<
        unsafe extern "C" fn(*mut libc::c_uchar, libc::c_ushort) -> (),
    > = None;
    let mut set_32: Option::<
        unsafe extern "C" fn(*mut libc::c_uchar, libc::c_uint) -> (),
    > = None;
    let mut set_64: Option::<
        unsafe extern "C" fn(*mut libc::c_uchar, ulong_type) -> (),
    > = None;
    hdr = &mut *hdrbuf.as_mut_ptr().offset(0 as libc::c_int as isize)
        as *mut libc::c_uchar;
    set_16 = Some(
        simple_object_set_big_16
            as unsafe extern "C" fn(*mut libc::c_uchar, libc::c_ushort) -> (),
    );
    set_32 = Some(
        simple_object_set_big_32
            as unsafe extern "C" fn(*mut libc::c_uchar, libc::c_uint) -> (),
    );
    set_64 = Some(
        simple_object_set_big_64
            as unsafe extern "C" fn(*mut libc::c_uchar, ulong_type) -> (),
    );
    memset(
        hdr as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<external_filehdr>() as libc::c_ulong,
    );
    set_16
        .expect(
            "non-null function pointer",
        )(hdr.offset(0 as libc::c_ulong as isize), (*attrs).magic);
    set_16
        .expect(
            "non-null function pointer",
        )(hdr.offset(2 as libc::c_ulong as isize), nscns as libc::c_ushort);
    if u64 != 0 {
        set_64
            .expect(
                "non-null function pointer",
            )(hdr.offset(8 as libc::c_ulong as isize), symtab_offset);
        set_32
            .expect(
                "non-null function pointer",
            )(hdr.offset(20 as libc::c_ulong as isize), nsyms);
        set_16
            .expect(
                "non-null function pointer",
            )(hdr.offset(18 as libc::c_ulong as isize), (*attrs).flags);
    } else {
        set_32
            .expect(
                "non-null function pointer",
            )(hdr.offset(8 as libc::c_ulong as isize), symtab_offset as libc::c_uint);
        set_32
            .expect(
                "non-null function pointer",
            )(hdr.offset(20 as libc::c_ulong as isize), nsyms);
        set_16
            .expect(
                "non-null function pointer",
            )(hdr.offset(18 as libc::c_ulong as isize), (*attrs).flags);
    }
    return simple_object_internal_write(
        descriptor,
        0 as libc::c_int as off_t,
        hdrbuf.as_mut_ptr(),
        ::core::mem::size_of::<external_filehdr>() as libc::c_ulong,
        errmsg,
        err,
    );
}
unsafe extern "C" fn simple_object_xcoff_write_scnhdr(
    mut sobj: *mut simple_object_write,
    mut descriptor: libc::c_int,
    mut name: *const libc::c_char,
    mut name_offset: *mut size_t,
    mut scnhdr_offset: off_t,
    mut scnsize: size_t,
    mut offset: off_t,
    mut align: libc::c_uint,
    mut errmsg: *mut *const libc::c_char,
    mut err: *mut libc::c_int,
) -> libc::c_int {
    let mut ocr: *mut simple_object_xcoff_read = (*sobj).data
        as *mut simple_object_xcoff_read;
    let mut u64: libc::c_int = ((*ocr).magic as libc::c_int == 0o767 as libc::c_int)
        as libc::c_int;
    let mut set_32: Option::<
        unsafe extern "C" fn(*mut libc::c_uchar, libc::c_uint) -> (),
    > = None;
    let mut set_64: Option::<
        unsafe extern "C" fn(*mut libc::c_uchar, libc::c_uint) -> (),
    > = None;
    let mut hdrbuf: [libc::c_uchar; 68] = [0; 68];
    let mut hdr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut namelen: size_t = 0;
    let mut flags: libc::c_uint = 0;
    set_32 = Some(
        simple_object_set_big_32
            as unsafe extern "C" fn(*mut libc::c_uchar, libc::c_uint) -> (),
    );
    set_64 = Some(
        simple_object_set_big_32
            as unsafe extern "C" fn(*mut libc::c_uchar, libc::c_uint) -> (),
    );
    memset(
        hdrbuf.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<[libc::c_uchar; 68]>() as libc::c_ulong,
    );
    hdr = &mut *hdrbuf.as_mut_ptr().offset(0 as libc::c_int as isize)
        as *mut libc::c_uchar;
    namelen = strlen(name);
    if namelen <= 8 as libc::c_int as libc::c_ulong {
        strncpy(
            (hdr as *mut libc::c_char).offset(0 as libc::c_ulong as isize),
            name,
            8 as libc::c_int as libc::c_ulong,
        );
    } else {
        snprintf(
            (hdr as *mut libc::c_char).offset(0 as libc::c_ulong as isize),
            8 as libc::c_int as libc::c_ulong,
            b"/%lu\0" as *const u8 as *const libc::c_char,
            *name_offset,
        );
        *name_offset = (*name_offset as libc::c_ulong)
            .wrapping_add(namelen.wrapping_add(1 as libc::c_int as libc::c_ulong))
            as size_t as size_t;
    }
    if u64 != 0 {
        set_64
            .expect(
                "non-null function pointer",
            )(hdr.offset(24 as libc::c_ulong as isize), scnsize as libc::c_uint);
        set_64
            .expect(
                "non-null function pointer",
            )(hdr.offset(32 as libc::c_ulong as isize), offset as libc::c_uint);
    } else {
        set_32
            .expect(
                "non-null function pointer",
            )(hdr.offset(16 as libc::c_ulong as isize), scnsize as libc::c_uint);
        set_32
            .expect(
                "non-null function pointer",
            )(hdr.offset(20 as libc::c_ulong as isize), offset as libc::c_uint);
    }
    flags = 0x40 as libc::c_int as libc::c_uint;
    if align > 13 as libc::c_int as libc::c_uint {
        align = 13 as libc::c_int as libc::c_uint;
    }
    if u64 != 0 {
        set_32
            .expect(
                "non-null function pointer",
            )(hdr.offset(64 as libc::c_ulong as isize), flags);
    } else {
        set_32
            .expect(
                "non-null function pointer",
            )(hdr.offset(36 as libc::c_ulong as isize), flags);
    }
    return simple_object_internal_write(
        descriptor,
        scnhdr_offset,
        hdrbuf.as_mut_ptr(),
        (if u64 != 0 { 68 as libc::c_int } else { 40 as libc::c_int }) as size_t,
        errmsg,
        err,
    );
}
unsafe extern "C" fn simple_object_xcoff_write_to_file(
    mut sobj: *mut simple_object_write,
    mut descriptor: libc::c_int,
    mut err: *mut libc::c_int,
) -> *const libc::c_char {
    let mut ocr: *mut simple_object_xcoff_read = (*sobj).data
        as *mut simple_object_xcoff_read;
    let mut u64: libc::c_int = ((*ocr).magic as libc::c_int == 0o767 as libc::c_int)
        as libc::c_int;
    let mut nscns: libc::c_uint = 0;
    let mut secnum: libc::c_uint = 0;
    let mut section: *mut simple_object_write_section = 0
        as *mut simple_object_write_section;
    let mut scnhdr_offset: off_t = 0;
    let mut symtab_offset: size_t = 0;
    let mut secsym_offset: off_t = 0;
    let mut nsyms: libc::c_uint = 0;
    let mut offset: size_t = 0;
    let mut name_offset: size_t = 0;
    let mut errmsg: *const libc::c_char = 0 as *const libc::c_char;
    let mut strsizebuf: [libc::c_uchar; 4] = [0; 4];
    let mut source_filename: *const libc::c_char = b"fake\0" as *const u8
        as *const libc::c_char;
    let mut sflen: size_t = 0;
    let mut syms: [C2RustUnnamed_2; 2] = [C2RustUnnamed_2 {
        sym: external_syment {
            u: C2RustUnnamed_12 {
                xcoff32: C2RustUnnamed_14 {
                    n: C2RustUnnamed_15 { n_name: [0; 8] },
                    n_value: [0; 4],
                },
            },
            n_scnum: [0; 2],
            n_type: [0; 2],
            n_sclass: [0; 1],
            n_numaux: [0; 1],
        },
    }; 2];
    let mut set_16: Option::<
        unsafe extern "C" fn(*mut libc::c_uchar, libc::c_ushort) -> (),
    > = None;
    let mut set_32: Option::<
        unsafe extern "C" fn(*mut libc::c_uchar, libc::c_uint) -> (),
    > = None;
    set_16 = Some(
        simple_object_set_big_16
            as unsafe extern "C" fn(*mut libc::c_uchar, libc::c_ushort) -> (),
    );
    set_32 = Some(
        simple_object_set_big_32
            as unsafe extern "C" fn(*mut libc::c_uchar, libc::c_uint) -> (),
    );
    nscns = 0 as libc::c_int as libc::c_uint;
    section = (*sobj).sections;
    while !section.is_null() {
        nscns = nscns.wrapping_add(1);
        nscns;
        section = (*section).next;
    }
    scnhdr_offset = (::core::mem::size_of::<external_filehdr>() as libc::c_ulong)
        .wrapping_sub(
            (if u64 != 0 { 4 as libc::c_int } else { 0 as libc::c_int }) as libc::c_ulong,
        ) as off_t;
    offset = (scnhdr_offset
        + nscns
            .wrapping_mul(
                (if u64 != 0 { 68 as libc::c_int } else { 40 as libc::c_int })
                    as libc::c_uint,
            ) as libc::c_long) as size_t;
    name_offset = 4 as libc::c_int as size_t;
    section = (*sobj).sections;
    while !section.is_null() {
        let mut mask: size_t = 0;
        let mut new_offset: size_t = 0;
        let mut scnsize: size_t = 0;
        let mut buffer: *mut simple_object_write_section_buffer = 0
            as *mut simple_object_write_section_buffer;
        mask = ((1 as libc::c_uint) << (*section).align)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t;
        new_offset = offset & mask;
        new_offset &= !mask;
        while new_offset > offset {
            let mut zeroes: [libc::c_uchar; 16] = [0; 16];
            let mut write: size_t = 0;
            memset(
                zeroes.as_mut_ptr() as *mut libc::c_void,
                0 as libc::c_int,
                ::core::mem::size_of::<[libc::c_uchar; 16]>() as libc::c_ulong,
            );
            write = new_offset.wrapping_sub(offset);
            if write > ::core::mem::size_of::<[libc::c_uchar; 16]>() as libc::c_ulong {
                write = ::core::mem::size_of::<[libc::c_uchar; 16]>() as libc::c_ulong;
            }
            if simple_object_internal_write(
                descriptor,
                offset as off_t,
                zeroes.as_mut_ptr(),
                write,
                &mut errmsg,
                err,
            ) == 0
            {
                return errmsg;
            }
        }
        scnsize = 0 as libc::c_int as size_t;
        buffer = (*section).buffers;
        while !buffer.is_null() {
            if simple_object_internal_write(
                descriptor,
                offset.wrapping_add(scnsize) as off_t,
                (*buffer).buffer as *const libc::c_uchar,
                (*buffer).size,
                &mut errmsg,
                err,
            ) == 0
            {
                return errmsg;
            }
            scnsize = (scnsize as libc::c_ulong).wrapping_add((*buffer).size) as size_t
                as size_t;
            buffer = (*buffer).next;
        }
        if simple_object_xcoff_write_scnhdr(
            sobj,
            descriptor,
            (*section).name,
            &mut name_offset,
            scnhdr_offset,
            scnsize,
            offset as off_t,
            (*section).align,
            &mut errmsg,
            err,
        ) == 0
        {
            return errmsg;
        }
        scnhdr_offset
            += (if u64 != 0 { 68 as libc::c_int } else { 40 as libc::c_int })
                as libc::c_long;
        offset = (offset as libc::c_ulong).wrapping_add(scnsize) as size_t as size_t;
        section = (*section).next;
    }
    offset = (offset as libc::c_ulong)
        .wrapping_add(offset & 1 as libc::c_int as libc::c_ulong) as size_t as size_t;
    nsyms = (2 as libc::c_int as libc::c_uint)
        .wrapping_mul(nscns.wrapping_add(1 as libc::c_int as libc::c_uint));
    symtab_offset = offset;
    offset = (offset as libc::c_ulong)
        .wrapping_add(
            nsyms.wrapping_mul(18 as libc::c_int as libc::c_uint) as libc::c_ulong,
        ) as size_t as size_t;
    memset(
        &mut *syms.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut C2RustUnnamed_2
            as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<[C2RustUnnamed_2; 2]>() as libc::c_ulong,
    );
    if u64 == 0 {
        strcpy(
            &mut *((*syms.as_mut_ptr().offset(0 as libc::c_int as isize))
                .sym
                .u
                .xcoff32
                .n
                .n_name)
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize) as *mut libc::c_char,
            b".file\0" as *const u8 as *const libc::c_char,
        );
    }
    set_16
        .expect(
            "non-null function pointer",
        )(
        &mut *((*syms.as_mut_ptr().offset(0 as libc::c_int as isize)).sym.n_scnum)
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize),
        -(2 as libc::c_int) as libc::c_ushort,
    );
    set_16
        .expect(
            "non-null function pointer",
        )(
        &mut *((*syms.as_mut_ptr().offset(0 as libc::c_int as isize)).sym.n_type)
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize),
        ((0 as libc::c_int) << 4 as libc::c_int | 0 as libc::c_int) as libc::c_ushort,
    );
    syms[0 as libc::c_int as usize]
        .sym
        .n_sclass[0 as libc::c_int as usize] = 103 as libc::c_int as libc::c_uchar;
    *(syms[0 as libc::c_int as usize].sym.n_numaux)
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_uchar;
    sflen = strlen(source_filename);
    if sflen <= 14 as libc::c_int as libc::c_ulong {
        memcpy(
            &mut *((*syms.as_mut_ptr().offset(1 as libc::c_int as isize))
                .aux
                .x_file
                .x_fname)
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize) as *mut libc::c_char
                as *mut libc::c_void,
            source_filename as *const libc::c_void,
            sflen,
        );
    } else {
        set_32
            .expect(
                "non-null function pointer",
            )(
            &mut *((*syms.as_mut_ptr().offset(1 as libc::c_int as isize))
                .aux
                .x_file
                ._x
                .x_offset)
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize),
            name_offset as libc::c_uint,
        );
        if simple_object_internal_write(
            descriptor,
            offset.wrapping_add(name_offset) as off_t,
            source_filename as *const libc::c_uchar,
            sflen.wrapping_add(1 as libc::c_int as libc::c_ulong),
            &mut errmsg,
            err,
        ) == 0
        {
            return errmsg;
        }
        name_offset = (name_offset as libc::c_ulong)
            .wrapping_add(
                (strlen(source_filename)).wrapping_add(1 as libc::c_int as libc::c_ulong),
            ) as size_t as size_t;
    }
    if simple_object_internal_write(
        descriptor,
        symtab_offset as off_t,
        &mut *syms.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut C2RustUnnamed_2
            as *const libc::c_uchar,
        ::core::mem::size_of::<[C2RustUnnamed_2; 2]>() as libc::c_ulong,
        &mut errmsg,
        err,
    ) == 0
    {
        return errmsg;
    }
    set_32
        .expect(
            "non-null function pointer",
        )(strsizebuf.as_mut_ptr(), name_offset as libc::c_uint);
    if simple_object_internal_write(
        descriptor,
        offset as off_t,
        strsizebuf.as_mut_ptr(),
        4 as libc::c_int as size_t,
        &mut errmsg,
        err,
    ) == 0
    {
        return errmsg;
    }
    name_offset = 4 as libc::c_int as size_t;
    secsym_offset = symtab_offset
        .wrapping_add(::core::mem::size_of::<[C2RustUnnamed_2; 2]>() as libc::c_ulong)
        as off_t;
    memset(
        &mut *syms.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut C2RustUnnamed_2
            as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<[C2RustUnnamed_2; 2]>() as libc::c_ulong,
    );
    set_16
        .expect(
            "non-null function pointer",
        )(
        &mut *((*syms.as_mut_ptr().offset(0 as libc::c_int as isize)).sym.n_type)
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize),
        ((0 as libc::c_int) << 4 as libc::c_int | 0 as libc::c_int) as libc::c_ushort,
    );
    syms[0 as libc::c_int as usize]
        .sym
        .n_sclass[0 as libc::c_int as usize] = 3 as libc::c_int as libc::c_uchar;
    *(syms[0 as libc::c_int as usize].sym.n_numaux)
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_uchar;
    secnum = 1 as libc::c_int as libc::c_uint;
    section = (*sobj).sections;
    while !section.is_null() {
        let mut namelen: size_t = 0;
        let mut scnsize_0: size_t = 0;
        let mut buffer_0: *mut simple_object_write_section_buffer = 0
            as *mut simple_object_write_section_buffer;
        namelen = strlen((*section).name);
        let fresh0 = secnum;
        secnum = secnum.wrapping_add(1);
        set_16
            .expect(
                "non-null function pointer",
            )(
            &mut *((*syms.as_mut_ptr().offset(0 as libc::c_int as isize)).sym.n_scnum)
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize),
            fresh0 as libc::c_ushort,
        );
        scnsize_0 = 0 as libc::c_int as size_t;
        buffer_0 = (*section).buffers;
        while !buffer_0.is_null() {
            scnsize_0 = (scnsize_0 as libc::c_ulong).wrapping_add((*buffer_0).size)
                as size_t as size_t;
            buffer_0 = (*buffer_0).next;
        }
        set_32
            .expect(
                "non-null function pointer",
            )(
            &mut *((*syms.as_mut_ptr().offset(1 as libc::c_int as isize))
                .aux
                .x_scn
                .x_scnlen)
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize),
            scnsize_0 as libc::c_uint,
        );
        if namelen > 8 as libc::c_int as libc::c_ulong {
            set_32
                .expect(
                    "non-null function pointer",
                )(
                &mut *((*syms.as_mut_ptr().offset(0 as libc::c_int as isize))
                    .sym
                    .u
                    .xcoff32
                    .n
                    .n
                    .n_zeroes)
                    .as_mut_ptr()
                    .offset(0 as libc::c_int as isize),
                0 as libc::c_int as libc::c_uint,
            );
            set_32
                .expect(
                    "non-null function pointer",
                )(
                &mut *((*syms.as_mut_ptr().offset(0 as libc::c_int as isize))
                    .sym
                    .u
                    .xcoff32
                    .n
                    .n
                    .n_offset)
                    .as_mut_ptr()
                    .offset(0 as libc::c_int as isize),
                name_offset as libc::c_uint,
            );
            if simple_object_internal_write(
                descriptor,
                offset.wrapping_add(name_offset) as off_t,
                (*section).name as *const libc::c_uchar,
                namelen.wrapping_add(1 as libc::c_int as libc::c_ulong),
                &mut errmsg,
                err,
            ) == 0
            {
                return errmsg;
            }
            name_offset = (name_offset as libc::c_ulong)
                .wrapping_add(namelen.wrapping_add(1 as libc::c_int as libc::c_ulong))
                as size_t as size_t;
        } else {
            memcpy(
                &mut *((*syms.as_mut_ptr().offset(0 as libc::c_int as isize))
                    .sym
                    .u
                    .xcoff32
                    .n
                    .n_name)
                    .as_mut_ptr()
                    .offset(0 as libc::c_int as isize) as *mut libc::c_char
                    as *mut libc::c_void,
                (*section).name as *const libc::c_void,
                strlen((*section).name),
            );
            memset(
                &mut *((*syms.as_mut_ptr().offset(0 as libc::c_int as isize))
                    .sym
                    .u
                    .xcoff32
                    .n
                    .n_name)
                    .as_mut_ptr()
                    .offset(
                        (strlen
                            as unsafe extern "C" fn(
                                *const libc::c_char,
                            ) -> libc::c_ulong)((*section).name) as isize,
                    ) as *mut libc::c_char as *mut libc::c_void,
                0 as libc::c_int,
                (8 as libc::c_int as libc::c_ulong).wrapping_sub(strlen((*section).name)),
            );
        }
        if simple_object_internal_write(
            descriptor,
            secsym_offset,
            &mut *syms.as_mut_ptr().offset(0 as libc::c_int as isize)
                as *mut C2RustUnnamed_2 as *const libc::c_uchar,
            ::core::mem::size_of::<[C2RustUnnamed_2; 2]>() as libc::c_ulong,
            &mut errmsg,
            err,
        ) == 0
        {
            return errmsg;
        }
        secsym_offset = (secsym_offset as libc::c_ulong)
            .wrapping_add(
                ::core::mem::size_of::<[C2RustUnnamed_2; 2]>() as libc::c_ulong,
            ) as off_t as off_t;
        section = (*section).next;
    }
    if simple_object_xcoff_write_filehdr(
        sobj,
        descriptor,
        nscns,
        symtab_offset,
        nsyms,
        &mut errmsg,
        err,
    ) == 0
    {
        return errmsg;
    }
    return 0 as *const libc::c_char;
}
unsafe extern "C" fn simple_object_xcoff_release_write(mut data: *mut libc::c_void) {
    free(data);
}
#[no_mangle]
pub static mut simple_object_xcoff_functions: simple_object_functions = unsafe {
    {
        let mut init = simple_object_functions {
            match_0: Some(
                simple_object_xcoff_match
                    as unsafe extern "C" fn(
                        *mut libc::c_uchar,
                        libc::c_int,
                        off_t,
                        *const libc::c_char,
                        *mut *const libc::c_char,
                        *mut libc::c_int,
                    ) -> *mut libc::c_void,
            ),
            find_sections: Some(
                simple_object_xcoff_find_sections
                    as unsafe extern "C" fn(
                        *mut simple_object_read,
                        Option::<
                            unsafe extern "C" fn(
                                *mut libc::c_void,
                                *const libc::c_char,
                                off_t,
                                off_t,
                            ) -> libc::c_int,
                        >,
                        *mut libc::c_void,
                        *mut libc::c_int,
                    ) -> *const libc::c_char,
            ),
            fetch_attributes: Some(
                simple_object_xcoff_fetch_attributes
                    as unsafe extern "C" fn(
                        *mut simple_object_read,
                        *mut *const libc::c_char,
                        *mut libc::c_int,
                    ) -> *mut libc::c_void,
            ),
            release_read: Some(
                simple_object_xcoff_release_read
                    as unsafe extern "C" fn(*mut libc::c_void) -> (),
            ),
            attributes_merge: Some(
                simple_object_xcoff_attributes_merge
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut libc::c_void,
                        *mut libc::c_int,
                    ) -> *const libc::c_char,
            ),
            release_attributes: Some(
                simple_object_xcoff_release_attributes
                    as unsafe extern "C" fn(*mut libc::c_void) -> (),
            ),
            start_write: Some(
                simple_object_xcoff_start_write
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut *const libc::c_char,
                        *mut libc::c_int,
                    ) -> *mut libc::c_void,
            ),
            write_to_file: Some(
                simple_object_xcoff_write_to_file
                    as unsafe extern "C" fn(
                        *mut simple_object_write,
                        libc::c_int,
                        *mut libc::c_int,
                    ) -> *const libc::c_char,
            ),
            release_write: Some(
                simple_object_xcoff_release_write
                    as unsafe extern "C" fn(*mut libc::c_void) -> (),
            ),
            copy_lto_debug_sections: None,
        };
        init
    }
};
