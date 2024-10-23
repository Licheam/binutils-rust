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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct external_filehdr {
    pub f_magic: [libc::c_uchar; 2],
    pub f_nscns: [libc::c_uchar; 2],
    pub f_timdat: [libc::c_uchar; 4],
    pub f_symptr: [libc::c_uchar; 4],
    pub f_nsyms: [libc::c_uchar; 4],
    pub f_opthdr: [libc::c_uchar; 2],
    pub f_flags: [libc::c_uchar; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct simple_object_coff_attributes {
    pub magic: libc::c_ushort,
    pub is_big_endian: libc::c_uchar,
    pub flags: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub sym: external_syment,
    pub aux: external_auxent,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union external_auxent {
    pub x_file: C2RustUnnamed_1,
    pub x_scn: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub x_scnlen: [libc::c_uchar; 4],
    pub x_nreloc: [libc::c_uchar; 2],
    pub x_nlinno: [libc::c_uchar; 2],
    pub x_checksum: [libc::c_uchar; 4],
    pub x_associated: [libc::c_uchar; 2],
    pub x_comdat: [libc::c_uchar; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub x_fname: [libc::c_char; 18],
    pub x_n: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub x_zeroes: [libc::c_uchar; 4],
    pub x_offset: [libc::c_uchar; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct external_syment {
    pub e: C2RustUnnamed_3,
    pub e_value: [libc::c_uchar; 4],
    pub e_scnum: [libc::c_uchar; 2],
    pub e_type: [libc::c_uchar; 2],
    pub e_sclass: [libc::c_uchar; 1],
    pub e_numaux: [libc::c_uchar; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub e_name: [libc::c_uchar; 8],
    pub e: C2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub e_zeroes: [libc::c_uchar; 4],
    pub e_offset: [libc::c_uchar; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct external_scnhdr {
    pub s_name: [libc::c_uchar; 8],
    pub s_paddr: [libc::c_uchar; 4],
    pub s_vaddr: [libc::c_uchar; 4],
    pub s_size: [libc::c_uchar; 4],
    pub s_scnptr: [libc::c_uchar; 4],
    pub s_relptr: [libc::c_uchar; 4],
    pub s_lnnoptr: [libc::c_uchar; 4],
    pub s_nreloc: [libc::c_uchar; 2],
    pub s_nlnno: [libc::c_uchar; 2],
    pub s_flags: [libc::c_uchar; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct simple_object_coff_read {
    pub magic: libc::c_ushort,
    pub is_big_endian: libc::c_uchar,
    pub nscns: libc::c_ushort,
    pub symptr: off_t,
    pub nsyms: libc::c_uint,
    pub flags: libc::c_ushort,
    pub scnhdr_offset: off_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct coff_magic_struct {
    pub magic: libc::c_ushort,
    pub is_big_endian: libc::c_uchar,
    pub non_object_flags: libc::c_ushort,
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
unsafe extern "C" fn simple_object_fetch_little_16(
    mut buf: *const libc::c_uchar,
) -> libc::c_ushort {
    return ((*buf.offset(1 as libc::c_int as isize) as libc::c_ushort as libc::c_int)
        << 8 as libc::c_int
        | *buf.offset(0 as libc::c_int as isize) as libc::c_ushort as libc::c_int)
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
unsafe extern "C" fn simple_object_fetch_little_32(
    mut buf: *const libc::c_uchar,
) -> libc::c_uint {
    return (*buf.offset(3 as libc::c_int as isize) as libc::c_uint) << 24 as libc::c_int
        | (*buf.offset(2 as libc::c_int as isize) as libc::c_uint) << 16 as libc::c_int
        | (*buf.offset(1 as libc::c_int as isize) as libc::c_uint) << 8 as libc::c_int
        | *buf.offset(0 as libc::c_int as isize) as libc::c_uint;
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
unsafe extern "C" fn simple_object_set_little_16(
    mut buf: *mut libc::c_uchar,
    mut val: libc::c_ushort,
) {
    *buf
        .offset(
            1 as libc::c_int as isize,
        ) = (val as libc::c_int >> 8 as libc::c_int & 0xff as libc::c_int)
        as libc::c_uchar;
    *buf
        .offset(
            0 as libc::c_int as isize,
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
unsafe extern "C" fn simple_object_set_little_32(
    mut buf: *mut libc::c_uchar,
    mut val: libc::c_uint,
) {
    *buf
        .offset(
            3 as libc::c_int as isize,
        ) = (val >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as libc::c_uchar;
    *buf
        .offset(
            2 as libc::c_int as isize,
        ) = (val >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as libc::c_uchar;
    *buf
        .offset(
            1 as libc::c_int as isize,
        ) = (val >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as libc::c_uchar;
    *buf
        .offset(
            0 as libc::c_int as isize,
        ) = (val & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
}
static mut coff_magic: [coff_magic_struct; 2] = [
    {
        let mut init = coff_magic_struct {
            magic: 0x14c as libc::c_int as libc::c_ushort,
            is_big_endian: 0 as libc::c_int as libc::c_uchar,
            non_object_flags: (0x2 as libc::c_int | 0x1000 as libc::c_int
                | 0x2000 as libc::c_int) as libc::c_ushort,
        };
        init
    },
    {
        let mut init = coff_magic_struct {
            magic: 0x8664 as libc::c_int as libc::c_ushort,
            is_big_endian: 0 as libc::c_int as libc::c_uchar,
            non_object_flags: (0x2 as libc::c_int | 0x1000 as libc::c_int
                | 0x2000 as libc::c_int) as libc::c_ushort,
        };
        init
    },
];
unsafe extern "C" fn simple_object_coff_match(
    mut header: *mut libc::c_uchar,
    mut descriptor: libc::c_int,
    mut offset: off_t,
    mut segment_name: *const libc::c_char,
    mut errmsg: *mut *const libc::c_char,
    mut err: *mut libc::c_int,
) -> *mut libc::c_void {
    let mut c: size_t = 0;
    let mut magic_big: libc::c_ushort = 0;
    let mut magic_little: libc::c_ushort = 0;
    let mut magic: libc::c_ushort = 0;
    let mut i: size_t = 0;
    let mut is_big_endian: libc::c_int = 0;
    let mut fetch_16: Option::<
        unsafe extern "C" fn(*const libc::c_uchar) -> libc::c_ushort,
    > = None;
    let mut fetch_32: Option::<
        unsafe extern "C" fn(*const libc::c_uchar) -> libc::c_uint,
    > = None;
    let mut hdrbuf: [libc::c_uchar; 20] = [0; 20];
    let mut flags: libc::c_ushort = 0;
    let mut ocr: *mut simple_object_coff_read = 0 as *mut simple_object_coff_read;
    c = (::core::mem::size_of::<[coff_magic_struct; 2]>() as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<coff_magic_struct>() as libc::c_ulong);
    magic_big = simple_object_fetch_big_16(header as *const libc::c_uchar);
    magic_little = simple_object_fetch_little_16(header as *const libc::c_uchar);
    i = 0 as libc::c_int as size_t;
    while i < c {
        if if coff_magic[i as usize].is_big_endian as libc::c_int != 0 {
            (coff_magic[i as usize].magic as libc::c_int == magic_big as libc::c_int)
                as libc::c_int
        } else {
            (coff_magic[i as usize].magic as libc::c_int == magic_little as libc::c_int)
                as libc::c_int
        } != 0
        {
            break;
        }
        i = i.wrapping_add(1);
        i;
    }
    if i >= c {
        *errmsg = 0 as *const libc::c_char;
        *err = 0 as libc::c_int;
        return 0 as *mut libc::c_void;
    }
    is_big_endian = coff_magic[i as usize].is_big_endian as libc::c_int;
    magic = (if is_big_endian != 0 {
        magic_big as libc::c_int
    } else {
        magic_little as libc::c_int
    }) as libc::c_ushort;
    fetch_16 = if is_big_endian != 0 {
        Some(
            simple_object_fetch_big_16
                as unsafe extern "C" fn(*const libc::c_uchar) -> libc::c_ushort,
        )
    } else {
        Some(
            simple_object_fetch_little_16
                as unsafe extern "C" fn(*const libc::c_uchar) -> libc::c_ushort,
        )
    };
    fetch_32 = if is_big_endian != 0 {
        Some(
            simple_object_fetch_big_32
                as unsafe extern "C" fn(*const libc::c_uchar) -> libc::c_uint,
        )
    } else {
        Some(
            simple_object_fetch_little_32
                as unsafe extern "C" fn(*const libc::c_uchar) -> libc::c_uint,
        )
    };
    if simple_object_internal_read(
        descriptor,
        offset,
        hdrbuf.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_uchar; 20]>() as libc::c_ulong,
        errmsg,
        err,
    ) == 0
    {
        return 0 as *mut libc::c_void;
    }
    flags = fetch_16
        .expect(
            "non-null function pointer",
        )(hdrbuf.as_mut_ptr().offset(18 as libc::c_ulong as isize));
    if flags as libc::c_int & coff_magic[i as usize].non_object_flags as libc::c_int
        != 0 as libc::c_int
    {
        *errmsg = b"not relocatable object file\0" as *const u8 as *const libc::c_char;
        *err = 0 as libc::c_int;
        return 0 as *mut libc::c_void;
    }
    ocr = xmalloc(::core::mem::size_of::<simple_object_coff_read>() as libc::c_ulong)
        as *mut simple_object_coff_read;
    (*ocr).magic = magic;
    (*ocr).is_big_endian = is_big_endian as libc::c_uchar;
    (*ocr)
        .nscns = fetch_16
        .expect(
            "non-null function pointer",
        )(hdrbuf.as_mut_ptr().offset(2 as libc::c_ulong as isize));
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
    (*ocr).flags = flags;
    (*ocr)
        .scnhdr_offset = (::core::mem::size_of::<external_filehdr>() as libc::c_ulong)
        .wrapping_add(
            fetch_16
                .expect(
                    "non-null function pointer",
                )(hdrbuf.as_mut_ptr().offset(16 as libc::c_ulong as isize))
                as libc::c_ulong,
        ) as off_t;
    return ocr as *mut libc::c_void;
}
unsafe extern "C" fn simple_object_coff_read_strtab(
    mut sobj: *mut simple_object_read,
    mut strtab_size: *mut size_t,
    mut errmsg: *mut *const libc::c_char,
    mut err: *mut libc::c_int,
) -> *mut libc::c_char {
    let mut ocr: *mut simple_object_coff_read = (*sobj).data
        as *mut simple_object_coff_read;
    let mut strtab_offset: off_t = 0;
    let mut strsizebuf: [libc::c_uchar; 4] = [0; 4];
    let mut strsize: size_t = 0;
    let mut strtab: *mut libc::c_char = 0 as *mut libc::c_char;
    strtab_offset = (((*sobj).offset + (*ocr).symptr) as libc::c_ulong)
        .wrapping_add(
            ((*ocr).nsyms as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<external_syment>() as libc::c_ulong),
        ) as off_t;
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
    strsize = (if (*ocr).is_big_endian as libc::c_int != 0 {
        simple_object_fetch_big_32(strsizebuf.as_mut_ptr())
    } else {
        simple_object_fetch_little_32(strsizebuf.as_mut_ptr())
    }) as size_t;
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
unsafe extern "C" fn simple_object_coff_find_sections(
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
    let mut ocr: *mut simple_object_coff_read = (*sobj).data
        as *mut simple_object_coff_read;
    let mut scnhdr_size: size_t = 0;
    let mut scnbuf: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut errmsg: *const libc::c_char = 0 as *const libc::c_char;
    let mut fetch_32: Option::<
        unsafe extern "C" fn(*const libc::c_uchar) -> libc::c_uint,
    > = None;
    let mut nscns: libc::c_uint = 0;
    let mut strtab: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut strtab_size: size_t = 0;
    let mut i: libc::c_uint = 0;
    scnhdr_size = ::core::mem::size_of::<external_scnhdr>() as libc::c_ulong;
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
    fetch_32 = if (*ocr).is_big_endian as libc::c_int != 0 {
        Some(
            simple_object_fetch_big_32
                as unsafe extern "C" fn(*const libc::c_uchar) -> libc::c_uint,
        )
    } else {
        Some(
            simple_object_fetch_little_32
                as unsafe extern "C" fn(*const libc::c_uchar) -> libc::c_uint,
        )
    };
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
        let mut size: libc::c_uint = 0;
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
                    strtab = simple_object_coff_read_strtab(
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
        scnptr = fetch_32
            .expect(
                "non-null function pointer",
            )(scnhdr.offset(20 as libc::c_ulong as isize)) as off_t;
        size = fetch_32
            .expect(
                "non-null function pointer",
            )(scnhdr.offset(16 as libc::c_ulong as isize));
        if (Some(pfn.expect("non-null function pointer")))
            .expect("non-null function pointer")(data, name, scnptr, size as off_t) == 0
        {
            break;
        }
        i = i.wrapping_add(1);
        i;
    }
    if !strtab.is_null() {
        free(strtab as *mut libc::c_void);
    }
    free(scnbuf as *mut libc::c_void);
    return 0 as *const libc::c_char;
}
unsafe extern "C" fn simple_object_coff_fetch_attributes(
    mut sobj: *mut simple_object_read,
    mut errmsg: *mut *const libc::c_char,
    mut err: *mut libc::c_int,
) -> *mut libc::c_void {
    let mut ocr: *mut simple_object_coff_read = (*sobj).data
        as *mut simple_object_coff_read;
    let mut ret: *mut simple_object_coff_attributes = 0
        as *mut simple_object_coff_attributes;
    ret = xmalloc(
        ::core::mem::size_of::<simple_object_coff_attributes>() as libc::c_ulong,
    ) as *mut simple_object_coff_attributes;
    (*ret).magic = (*ocr).magic;
    (*ret).is_big_endian = (*ocr).is_big_endian;
    (*ret).flags = (*ocr).flags;
    return ret as *mut libc::c_void;
}
unsafe extern "C" fn simple_object_coff_release_read(mut data: *mut libc::c_void) {
    free(data);
}
unsafe extern "C" fn simple_object_coff_attributes_merge(
    mut todata: *mut libc::c_void,
    mut fromdata: *mut libc::c_void,
    mut err: *mut libc::c_int,
) -> *const libc::c_char {
    let mut to: *mut simple_object_coff_attributes = todata
        as *mut simple_object_coff_attributes;
    let mut from: *mut simple_object_coff_attributes = fromdata
        as *mut simple_object_coff_attributes;
    if (*to).magic as libc::c_int != (*from).magic as libc::c_int
        || (*to).is_big_endian as libc::c_int != (*from).is_big_endian as libc::c_int
    {
        *err = 0 as libc::c_int;
        return b"COFF object format mismatch\0" as *const u8 as *const libc::c_char;
    }
    return 0 as *const libc::c_char;
}
unsafe extern "C" fn simple_object_coff_release_attributes(mut data: *mut libc::c_void) {
    free(data);
}
unsafe extern "C" fn simple_object_coff_start_write(
    mut attributes_data: *mut libc::c_void,
    mut errmsg: *mut *const libc::c_char,
    mut err: *mut libc::c_int,
) -> *mut libc::c_void {
    let mut attrs: *mut simple_object_coff_attributes = attributes_data
        as *mut simple_object_coff_attributes;
    let mut ret: *mut simple_object_coff_attributes = 0
        as *mut simple_object_coff_attributes;
    ret = xmalloc(
        ::core::mem::size_of::<simple_object_coff_attributes>() as libc::c_ulong,
    ) as *mut simple_object_coff_attributes;
    *ret = *attrs;
    return ret as *mut libc::c_void;
}
unsafe extern "C" fn simple_object_coff_write_filehdr(
    mut sobj: *mut simple_object_write,
    mut descriptor: libc::c_int,
    mut nscns: libc::c_uint,
    mut symtab_offset: size_t,
    mut nsyms: libc::c_uint,
    mut errmsg: *mut *const libc::c_char,
    mut err: *mut libc::c_int,
) -> libc::c_int {
    let mut attrs: *mut simple_object_coff_attributes = (*sobj).data
        as *mut simple_object_coff_attributes;
    let mut hdrbuf: [libc::c_uchar; 20] = [0; 20];
    let mut hdr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut set_16: Option::<
        unsafe extern "C" fn(*mut libc::c_uchar, libc::c_ushort) -> (),
    > = None;
    let mut set_32: Option::<
        unsafe extern "C" fn(*mut libc::c_uchar, libc::c_uint) -> (),
    > = None;
    hdr = &mut *hdrbuf.as_mut_ptr().offset(0 as libc::c_int as isize)
        as *mut libc::c_uchar;
    set_16 = if (*attrs).is_big_endian as libc::c_int != 0 {
        Some(
            simple_object_set_big_16
                as unsafe extern "C" fn(*mut libc::c_uchar, libc::c_ushort) -> (),
        )
    } else {
        Some(
            simple_object_set_little_16
                as unsafe extern "C" fn(*mut libc::c_uchar, libc::c_ushort) -> (),
        )
    };
    set_32 = if (*attrs).is_big_endian as libc::c_int != 0 {
        Some(
            simple_object_set_big_32
                as unsafe extern "C" fn(*mut libc::c_uchar, libc::c_uint) -> (),
        )
    } else {
        Some(
            simple_object_set_little_32
                as unsafe extern "C" fn(*mut libc::c_uchar, libc::c_uint) -> (),
        )
    };
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
    set_32
        .expect(
            "non-null function pointer",
        )(hdr.offset(8 as libc::c_ulong as isize), symtab_offset as libc::c_uint);
    set_32
        .expect(
            "non-null function pointer",
        )(hdr.offset(12 as libc::c_ulong as isize), nsyms);
    set_16
        .expect(
            "non-null function pointer",
        )(hdr.offset(18 as libc::c_ulong as isize), (*attrs).flags);
    return simple_object_internal_write(
        descriptor,
        0 as libc::c_int as off_t,
        hdrbuf.as_mut_ptr(),
        ::core::mem::size_of::<external_filehdr>() as libc::c_ulong,
        errmsg,
        err,
    );
}
unsafe extern "C" fn simple_object_coff_write_scnhdr(
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
    let mut attrs: *mut simple_object_coff_attributes = (*sobj).data
        as *mut simple_object_coff_attributes;
    let mut set_32: Option::<
        unsafe extern "C" fn(*mut libc::c_uchar, libc::c_uint) -> (),
    > = None;
    let mut hdrbuf: [libc::c_uchar; 40] = [0; 40];
    let mut hdr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut namelen: size_t = 0;
    let mut flags: libc::c_uint = 0;
    set_32 = if (*attrs).is_big_endian as libc::c_int != 0 {
        Some(
            simple_object_set_big_32
                as unsafe extern "C" fn(*mut libc::c_uchar, libc::c_uint) -> (),
        )
    } else {
        Some(
            simple_object_set_little_32
                as unsafe extern "C" fn(*mut libc::c_uchar, libc::c_uint) -> (),
        )
    };
    memset(
        hdrbuf.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<[libc::c_uchar; 40]>() as libc::c_ulong,
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
    set_32
        .expect(
            "non-null function pointer",
        )(hdr.offset(16 as libc::c_ulong as isize), scnsize as libc::c_uint);
    set_32
        .expect(
            "non-null function pointer",
        )(hdr.offset(20 as libc::c_ulong as isize), offset as libc::c_uint);
    flags = ((1 as libc::c_int) << 6 as libc::c_int
        | (1 as libc::c_int) << 25 as libc::c_int
        | (1 as libc::c_int) << 28 as libc::c_int
        | (1 as libc::c_int) << 30 as libc::c_int) as libc::c_uint;
    if align > 13 as libc::c_int as libc::c_uint {
        align = 13 as libc::c_int as libc::c_uint;
    }
    flags |= align.wrapping_add(1 as libc::c_int as libc::c_uint) << 20 as libc::c_int;
    set_32
        .expect(
            "non-null function pointer",
        )(hdr.offset(36 as libc::c_ulong as isize), flags);
    return simple_object_internal_write(
        descriptor,
        scnhdr_offset,
        hdrbuf.as_mut_ptr(),
        ::core::mem::size_of::<external_scnhdr>() as libc::c_ulong,
        errmsg,
        err,
    );
}
unsafe extern "C" fn simple_object_coff_write_to_file(
    mut sobj: *mut simple_object_write,
    mut descriptor: libc::c_int,
    mut err: *mut libc::c_int,
) -> *const libc::c_char {
    let mut attrs: *mut simple_object_coff_attributes = (*sobj).data
        as *mut simple_object_coff_attributes;
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
    let mut syms: [C2RustUnnamed; 2] = [C2RustUnnamed {
        sym: external_syment {
            e: C2RustUnnamed_3 { e_name: [0; 8] },
            e_value: [0; 4],
            e_scnum: [0; 2],
            e_type: [0; 2],
            e_sclass: [0; 1],
            e_numaux: [0; 1],
        },
    }; 2];
    let mut set_16: Option::<
        unsafe extern "C" fn(*mut libc::c_uchar, libc::c_ushort) -> (),
    > = None;
    let mut set_32: Option::<
        unsafe extern "C" fn(*mut libc::c_uchar, libc::c_uint) -> (),
    > = None;
    set_16 = if (*attrs).is_big_endian as libc::c_int != 0 {
        Some(
            simple_object_set_big_16
                as unsafe extern "C" fn(*mut libc::c_uchar, libc::c_ushort) -> (),
        )
    } else {
        Some(
            simple_object_set_little_16
                as unsafe extern "C" fn(*mut libc::c_uchar, libc::c_ushort) -> (),
        )
    };
    set_32 = if (*attrs).is_big_endian as libc::c_int != 0 {
        Some(
            simple_object_set_big_32
                as unsafe extern "C" fn(*mut libc::c_uchar, libc::c_uint) -> (),
        )
    } else {
        Some(
            simple_object_set_little_32
                as unsafe extern "C" fn(*mut libc::c_uchar, libc::c_uint) -> (),
        )
    };
    nscns = 0 as libc::c_int as libc::c_uint;
    section = (*sobj).sections;
    while !section.is_null() {
        nscns = nscns.wrapping_add(1);
        nscns;
        section = (*section).next;
    }
    scnhdr_offset = ::core::mem::size_of::<external_filehdr>() as libc::c_ulong as off_t;
    offset = (scnhdr_offset as libc::c_ulong)
        .wrapping_add(
            (nscns as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<external_scnhdr>() as libc::c_ulong),
        );
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
        if simple_object_coff_write_scnhdr(
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
        scnhdr_offset = (scnhdr_offset as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<external_scnhdr>() as libc::c_ulong)
            as off_t as off_t;
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
            (nsyms as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<external_syment>() as libc::c_ulong),
        ) as size_t as size_t;
    memset(
        &mut *syms.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut C2RustUnnamed
            as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<[C2RustUnnamed; 2]>() as libc::c_ulong,
    );
    strcpy(
        &mut *((*syms.as_mut_ptr().offset(0 as libc::c_int as isize)).sym.e.e_name)
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize) as *mut libc::c_uchar
            as *mut libc::c_char,
        b".file\0" as *const u8 as *const libc::c_char,
    );
    set_16
        .expect(
            "non-null function pointer",
        )(
        &mut *((*syms.as_mut_ptr().offset(0 as libc::c_int as isize)).sym.e_scnum)
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize),
        -(2 as libc::c_int) as libc::c_ushort,
    );
    set_16
        .expect(
            "non-null function pointer",
        )(
        &mut *((*syms.as_mut_ptr().offset(0 as libc::c_int as isize)).sym.e_type)
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize),
        ((0 as libc::c_int) << 4 as libc::c_int | 0 as libc::c_int) as libc::c_ushort,
    );
    syms[0 as libc::c_int as usize]
        .sym
        .e_sclass[0 as libc::c_int as usize] = 103 as libc::c_int as libc::c_uchar;
    *(syms[0 as libc::c_int as usize].sym.e_numaux)
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_uchar;
    sflen = strlen(source_filename);
    if sflen <= 18 as libc::c_int as libc::c_ulong {
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
                .x_n
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
        &mut *syms.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut C2RustUnnamed
            as *const libc::c_uchar,
        ::core::mem::size_of::<[C2RustUnnamed; 2]>() as libc::c_ulong,
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
        .wrapping_add(::core::mem::size_of::<[C2RustUnnamed; 2]>() as libc::c_ulong)
        as off_t;
    memset(
        &mut *syms.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut C2RustUnnamed
            as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<[C2RustUnnamed; 2]>() as libc::c_ulong,
    );
    set_16
        .expect(
            "non-null function pointer",
        )(
        &mut *((*syms.as_mut_ptr().offset(0 as libc::c_int as isize)).sym.e_type)
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize),
        ((0 as libc::c_int) << 4 as libc::c_int | 0 as libc::c_int) as libc::c_ushort,
    );
    syms[0 as libc::c_int as usize]
        .sym
        .e_sclass[0 as libc::c_int as usize] = 3 as libc::c_int as libc::c_uchar;
    *(syms[0 as libc::c_int as usize].sym.e_numaux)
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
            &mut *((*syms.as_mut_ptr().offset(0 as libc::c_int as isize)).sym.e_scnum)
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
                    .e
                    .e
                    .e_zeroes)
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
                    .e
                    .e
                    .e_offset)
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
                    .e
                    .e_name)
                    .as_mut_ptr()
                    .offset(0 as libc::c_int as isize) as *mut libc::c_uchar
                    as *mut libc::c_void,
                (*section).name as *const libc::c_void,
                strlen((*section).name),
            );
            memset(
                &mut *((*syms.as_mut_ptr().offset(0 as libc::c_int as isize))
                    .sym
                    .e
                    .e_name)
                    .as_mut_ptr()
                    .offset(
                        (strlen
                            as unsafe extern "C" fn(
                                *const libc::c_char,
                            ) -> libc::c_ulong)((*section).name) as isize,
                    ) as *mut libc::c_uchar as *mut libc::c_void,
                0 as libc::c_int,
                (8 as libc::c_int as libc::c_ulong).wrapping_sub(strlen((*section).name)),
            );
        }
        if simple_object_internal_write(
            descriptor,
            secsym_offset,
            &mut *syms.as_mut_ptr().offset(0 as libc::c_int as isize)
                as *mut C2RustUnnamed as *const libc::c_uchar,
            ::core::mem::size_of::<[C2RustUnnamed; 2]>() as libc::c_ulong,
            &mut errmsg,
            err,
        ) == 0
        {
            return errmsg;
        }
        secsym_offset = (secsym_offset as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[C2RustUnnamed; 2]>() as libc::c_ulong)
            as off_t as off_t;
        section = (*section).next;
    }
    if simple_object_coff_write_filehdr(
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
unsafe extern "C" fn simple_object_coff_release_write(mut data: *mut libc::c_void) {
    free(data);
}
#[no_mangle]
pub static mut simple_object_coff_functions: simple_object_functions = unsafe {
    {
        let mut init = simple_object_functions {
            match_0: Some(
                simple_object_coff_match
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
                simple_object_coff_find_sections
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
                simple_object_coff_fetch_attributes
                    as unsafe extern "C" fn(
                        *mut simple_object_read,
                        *mut *const libc::c_char,
                        *mut libc::c_int,
                    ) -> *mut libc::c_void,
            ),
            release_read: Some(
                simple_object_coff_release_read
                    as unsafe extern "C" fn(*mut libc::c_void) -> (),
            ),
            attributes_merge: Some(
                simple_object_coff_attributes_merge
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut libc::c_void,
                        *mut libc::c_int,
                    ) -> *const libc::c_char,
            ),
            release_attributes: Some(
                simple_object_coff_release_attributes
                    as unsafe extern "C" fn(*mut libc::c_void) -> (),
            ),
            start_write: Some(
                simple_object_coff_start_write
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut *const libc::c_char,
                        *mut libc::c_int,
                    ) -> *mut libc::c_void,
            ),
            write_to_file: Some(
                simple_object_coff_write_to_file
                    as unsafe extern "C" fn(
                        *mut simple_object_write,
                        libc::c_int,
                        *mut libc::c_int,
                    ) -> *const libc::c_char,
            ),
            release_write: Some(
                simple_object_coff_release_write
                    as unsafe extern "C" fn(*mut libc::c_void) -> (),
            ),
            copy_lto_debug_sections: None,
        };
        init
    }
};
