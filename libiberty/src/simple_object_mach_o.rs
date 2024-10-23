use ::libc;
extern "C" {
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
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
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
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
pub struct mach_o_header_64 {
    pub magic: [libc::c_uchar; 4],
    pub cputype: [libc::c_uchar; 4],
    pub cpusubtype: [libc::c_uchar; 4],
    pub filetype: [libc::c_uchar; 4],
    pub ncmds: [libc::c_uchar; 4],
    pub sizeofcmds: [libc::c_uchar; 4],
    pub flags: [libc::c_uchar; 4],
    pub reserved: [libc::c_uchar; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct simple_object_mach_o_attributes {
    pub magic: libc::c_uint,
    pub is_big_endian: libc::c_int,
    pub cputype: libc::c_uint,
    pub cpusubtype: libc::c_uint,
    pub flags: libc::c_uint,
    pub reserved: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mach_o_section_64 {
    pub sectname: [libc::c_uchar; 16],
    pub segname: [libc::c_uchar; 16],
    pub addr: [libc::c_uchar; 8],
    pub size: [libc::c_uchar; 8],
    pub offset: [libc::c_uchar; 4],
    pub align: [libc::c_uchar; 4],
    pub reloff: [libc::c_uchar; 4],
    pub nreloc: [libc::c_uchar; 4],
    pub flags: [libc::c_uchar; 4],
    pub reserved1: [libc::c_uchar; 4],
    pub reserved2: [libc::c_uchar; 4],
    pub reserved3: [libc::c_uchar; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mach_o_segment_command_64 {
    pub cmd: [libc::c_uchar; 4],
    pub cmdsize: [libc::c_uchar; 4],
    pub segname: [libc::c_uchar; 16],
    pub vmaddr: [libc::c_uchar; 8],
    pub vmsize: [libc::c_uchar; 8],
    pub fileoff: [libc::c_uchar; 8],
    pub filesize: [libc::c_uchar; 8],
    pub maxprot: [libc::c_uchar; 4],
    pub initprot: [libc::c_uchar; 4],
    pub nsects: [libc::c_uchar; 4],
    pub flags: [libc::c_uchar; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mach_o_section_32 {
    pub sectname: [libc::c_uchar; 16],
    pub segname: [libc::c_uchar; 16],
    pub addr: [libc::c_uchar; 4],
    pub size: [libc::c_uchar; 4],
    pub offset: [libc::c_uchar; 4],
    pub align: [libc::c_uchar; 4],
    pub reloff: [libc::c_uchar; 4],
    pub nreloc: [libc::c_uchar; 4],
    pub flags: [libc::c_uchar; 4],
    pub reserved1: [libc::c_uchar; 4],
    pub reserved2: [libc::c_uchar; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mach_o_segment_command_32 {
    pub cmd: [libc::c_uchar; 4],
    pub cmdsize: [libc::c_uchar; 4],
    pub segname: [libc::c_uchar; 16],
    pub vmaddr: [libc::c_uchar; 4],
    pub vmsize: [libc::c_uchar; 4],
    pub fileoff: [libc::c_uchar; 4],
    pub filesize: [libc::c_uchar; 4],
    pub maxprot: [libc::c_uchar; 4],
    pub initprot: [libc::c_uchar; 4],
    pub nsects: [libc::c_uchar; 4],
    pub flags: [libc::c_uchar; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mach_o_header_32 {
    pub magic: [libc::c_uchar; 4],
    pub cputype: [libc::c_uchar; 4],
    pub cpusubtype: [libc::c_uchar; 4],
    pub filetype: [libc::c_uchar; 4],
    pub ncmds: [libc::c_uchar; 4],
    pub sizeofcmds: [libc::c_uchar; 4],
    pub flags: [libc::c_uchar; 4],
}
pub type ulong_type = uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct simple_object_mach_o_read {
    pub segment_name: *mut libc::c_char,
    pub magic: libc::c_uint,
    pub is_big_endian: libc::c_int,
    pub cputype: libc::c_uint,
    pub cpusubtype: libc::c_uint,
    pub ncmds: libc::c_uint,
    pub flags: libc::c_uint,
    pub reserved: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mach_o_load_command {
    pub cmd: [libc::c_uchar; 4],
    pub cmdsize: [libc::c_uchar; 4],
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
unsafe extern "C" fn simple_object_fetch_little_64(
    mut buf: *const libc::c_uchar,
) -> ulong_type {
    return (*buf.offset(7 as libc::c_int as isize) as ulong_type) << 56 as libc::c_int
        | (*buf.offset(6 as libc::c_int as isize) as ulong_type) << 48 as libc::c_int
        | (*buf.offset(5 as libc::c_int as isize) as ulong_type) << 40 as libc::c_int
        | (*buf.offset(4 as libc::c_int as isize) as ulong_type) << 32 as libc::c_int
        | (*buf.offset(3 as libc::c_int as isize) as ulong_type) << 24 as libc::c_int
        | (*buf.offset(2 as libc::c_int as isize) as ulong_type) << 16 as libc::c_int
        | (*buf.offset(1 as libc::c_int as isize) as ulong_type) << 8 as libc::c_int
        | *buf.offset(0 as libc::c_int as isize) as ulong_type;
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
#[inline]
unsafe extern "C" fn simple_object_set_little_64(
    mut buf: *mut libc::c_uchar,
    mut val: ulong_type,
) {
    *buf
        .offset(
            7 as libc::c_int as isize,
        ) = (val >> 56 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as libc::c_uchar;
    *buf
        .offset(
            6 as libc::c_int as isize,
        ) = (val >> 48 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as libc::c_uchar;
    *buf
        .offset(
            5 as libc::c_int as isize,
        ) = (val >> 40 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as libc::c_uchar;
    *buf
        .offset(
            4 as libc::c_int as isize,
        ) = (val >> 32 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as libc::c_uchar;
    *buf
        .offset(
            3 as libc::c_int as isize,
        ) = (val >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as libc::c_uchar;
    *buf
        .offset(
            2 as libc::c_int as isize,
        ) = (val >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as libc::c_uchar;
    *buf
        .offset(
            1 as libc::c_int as isize,
        ) = (val >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as libc::c_uchar;
    *buf
        .offset(
            0 as libc::c_int as isize,
        ) = (val & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
}
unsafe extern "C" fn simple_object_mach_o_match(
    mut header: *mut libc::c_uchar,
    mut descriptor: libc::c_int,
    mut offset: off_t,
    mut segment_name: *const libc::c_char,
    mut errmsg: *mut *const libc::c_char,
    mut err: *mut libc::c_int,
) -> *mut libc::c_void {
    let mut magic: libc::c_uint = 0;
    let mut is_big_endian: libc::c_int = 0;
    let mut fetch_32: Option::<
        unsafe extern "C" fn(*const libc::c_uchar) -> libc::c_uint,
    > = None;
    let mut filetype: libc::c_uint = 0;
    let mut omr: *mut simple_object_mach_o_read = 0 as *mut simple_object_mach_o_read;
    let mut buf: [libc::c_uchar; 32] = [0; 32];
    let mut b: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    magic = simple_object_fetch_big_32(header as *const libc::c_uchar);
    if magic == 0xfeedface as libc::c_uint || magic == 0xfeedfacf as libc::c_uint {
        is_big_endian = 1 as libc::c_int;
    } else {
        magic = simple_object_fetch_little_32(header as *const libc::c_uchar);
        if magic == 0xfeedface as libc::c_uint || magic == 0xfeedfacf as libc::c_uint {
            is_big_endian = 0 as libc::c_int;
        } else {
            *errmsg = 0 as *const libc::c_char;
            *err = 0 as libc::c_int;
            return 0 as *mut libc::c_void;
        }
    }
    if segment_name.is_null() {
        *errmsg = b"Mach-O file found but no segment name specified\0" as *const u8
            as *const libc::c_char;
        *err = 0 as libc::c_int;
        return 0 as *mut libc::c_void;
    }
    if strlen(segment_name) > 16 as libc::c_int as libc::c_ulong {
        *errmsg = b"Mach-O segment name too long\0" as *const u8 as *const libc::c_char;
        *err = 0 as libc::c_int;
        return 0 as *mut libc::c_void;
    }
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
        buf.as_mut_ptr(),
        if magic == 0xfeedface as libc::c_uint {
            ::core::mem::size_of::<mach_o_header_32>() as libc::c_ulong
        } else {
            ::core::mem::size_of::<mach_o_header_64>() as libc::c_ulong
        },
        errmsg,
        err,
    ) == 0
    {
        return 0 as *mut libc::c_void;
    }
    b = &mut *buf.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut libc::c_uchar;
    filetype = (Some(fetch_32.expect("non-null function pointer")))
        .expect("non-null function pointer")(b.offset(12 as libc::c_ulong as isize));
    if filetype != 0x1 as libc::c_int as libc::c_uint {
        *errmsg = b"Mach-O file is not object file\0" as *const u8
            as *const libc::c_char;
        *err = 0 as libc::c_int;
        return 0 as *mut libc::c_void;
    }
    omr = xmalloc(::core::mem::size_of::<simple_object_mach_o_read>() as libc::c_ulong)
        as *mut simple_object_mach_o_read;
    (*omr).segment_name = xstrdup(segment_name);
    (*omr).magic = magic;
    (*omr).is_big_endian = is_big_endian;
    (*omr)
        .cputype = (Some(fetch_32.expect("non-null function pointer")))
        .expect("non-null function pointer")(b.offset(4 as libc::c_ulong as isize));
    (*omr)
        .cpusubtype = (Some(fetch_32.expect("non-null function pointer")))
        .expect("non-null function pointer")(b.offset(8 as libc::c_ulong as isize));
    (*omr)
        .ncmds = (Some(fetch_32.expect("non-null function pointer")))
        .expect("non-null function pointer")(b.offset(16 as libc::c_ulong as isize));
    (*omr)
        .flags = (Some(fetch_32.expect("non-null function pointer")))
        .expect("non-null function pointer")(b.offset(24 as libc::c_ulong as isize));
    if magic == 0xfeedface as libc::c_uint {
        (*omr).reserved = 0 as libc::c_int as libc::c_uint;
    } else {
        (*omr)
            .reserved = (Some(fetch_32.expect("non-null function pointer")))
            .expect("non-null function pointer")(b.offset(28 as libc::c_ulong as isize));
    }
    return omr as *mut libc::c_void;
}
unsafe extern "C" fn simple_object_mach_o_section_info(
    mut is_big_endian: libc::c_int,
    mut is_32: libc::c_int,
    mut sechdr: *const libc::c_uchar,
    mut offset: *mut off_t,
    mut size: *mut size_t,
) {
    let mut fetch_32: Option::<
        unsafe extern "C" fn(*const libc::c_uchar) -> libc::c_uint,
    > = None;
    let mut fetch_64: Option::<
        unsafe extern "C" fn(*const libc::c_uchar) -> ulong_type,
    > = None;
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
    fetch_64 = None;
    fetch_64 = if is_big_endian != 0 {
        Some(
            simple_object_fetch_big_64
                as unsafe extern "C" fn(*const libc::c_uchar) -> ulong_type,
        )
    } else {
        Some(
            simple_object_fetch_little_64
                as unsafe extern "C" fn(*const libc::c_uchar) -> ulong_type,
        )
    };
    if is_32 != 0 {
        *offset = fetch_32
            .expect(
                "non-null function pointer",
            )(sechdr.offset(40 as libc::c_ulong as isize)) as off_t;
        *size = fetch_32
            .expect(
                "non-null function pointer",
            )(sechdr.offset(36 as libc::c_ulong as isize)) as size_t;
    } else {
        *offset = fetch_32
            .expect(
                "non-null function pointer",
            )(sechdr.offset(48 as libc::c_ulong as isize)) as off_t;
        *size = fetch_64
            .expect(
                "non-null function pointer",
            )(sechdr.offset(40 as libc::c_ulong as isize));
    };
}
unsafe extern "C" fn simple_object_mach_o_segment(
    mut sobj: *mut simple_object_read,
    mut offset: off_t,
    mut segbuf: *const libc::c_uchar,
    mut pfn: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const libc::c_char,
            off_t,
            off_t,
        ) -> libc::c_int,
    >,
    mut data: *mut libc::c_void,
    mut errmsg: *mut *const libc::c_char,
    mut err: *mut libc::c_int,
) -> libc::c_int {
    let mut omr: *mut simple_object_mach_o_read = (*sobj).data
        as *mut simple_object_mach_o_read;
    let mut fetch_32: Option::<
        unsafe extern "C" fn(*const libc::c_uchar) -> libc::c_uint,
    > = None;
    let mut is_32: libc::c_int = 0;
    let mut seghdrsize: size_t = 0;
    let mut sechdrsize: size_t = 0;
    let mut segname_offset: size_t = 0;
    let mut sectname_offset: size_t = 0;
    let mut nsects: libc::c_uint = 0;
    let mut secdata: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut i: libc::c_uint = 0;
    let mut gnu_sections_found: libc::c_uint = 0;
    let mut strtab_index: libc::c_uint = 0;
    let mut index_index: libc::c_uint = 0;
    let mut nametab_index: libc::c_uint = 0;
    let mut sections_index: libc::c_uint = 0;
    let mut strtab: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut nametab: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut index: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut strtab_size: size_t = 0;
    let mut nametab_size: size_t = 0;
    let mut index_size: size_t = 0;
    let mut n_wrapped_sects: libc::c_uint = 0;
    let mut wrapper_sect_size: size_t = 0;
    let mut wrapper_sect_offset: off_t = 0 as libc::c_int as off_t;
    fetch_32 = if (*omr).is_big_endian != 0 {
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
    is_32 = ((*omr).magic == 0xfeedface as libc::c_uint) as libc::c_int;
    if is_32 != 0 {
        seghdrsize = ::core::mem::size_of::<mach_o_segment_command_32>()
            as libc::c_ulong;
        sechdrsize = ::core::mem::size_of::<mach_o_section_32>() as libc::c_ulong;
        segname_offset = 16 as libc::c_ulong;
        sectname_offset = 0 as libc::c_ulong;
        nsects = (Some(fetch_32.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(segbuf.offset(48 as libc::c_ulong as isize));
    } else {
        seghdrsize = ::core::mem::size_of::<mach_o_segment_command_64>()
            as libc::c_ulong;
        sechdrsize = ::core::mem::size_of::<mach_o_section_64>() as libc::c_ulong;
        segname_offset = 16 as libc::c_ulong;
        sectname_offset = 0 as libc::c_ulong;
        nsects = (Some(fetch_32.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(segbuf.offset(64 as libc::c_ulong as isize));
    }
    secdata = xmalloc(
        (::core::mem::size_of::<libc::c_uchar>() as libc::c_ulong)
            .wrapping_mul((nsects as libc::c_ulong).wrapping_mul(sechdrsize)),
    ) as *mut libc::c_uchar;
    if simple_object_internal_read(
        (*sobj).descriptor,
        (offset as libc::c_ulong).wrapping_add(seghdrsize) as off_t,
        secdata,
        (nsects as libc::c_ulong).wrapping_mul(sechdrsize),
        errmsg,
        err,
    ) == 0
    {
        free(secdata as *mut libc::c_void);
        return 0 as libc::c_int;
    }
    gnu_sections_found = 0 as libc::c_int as libc::c_uint;
    index_index = nsects;
    sections_index = nsects;
    strtab_index = nsects;
    nametab_index = nsects;
    i = 0 as libc::c_int as libc::c_uint;
    while i < nsects {
        let mut nameoff: size_t = 0;
        nameoff = (i as libc::c_ulong)
            .wrapping_mul(sechdrsize)
            .wrapping_add(segname_offset);
        if !(strcmp(
            (secdata as *mut libc::c_char).offset(nameoff as isize),
            (*omr).segment_name,
        ) != 0 as libc::c_int)
        {
            nameoff = (i as libc::c_ulong)
                .wrapping_mul(sechdrsize)
                .wrapping_add(sectname_offset);
            if strcmp(
                (secdata as *mut libc::c_char).offset(nameoff as isize),
                b"__wrapper_names\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                nametab_index = i;
                gnu_sections_found |= 0x4 as libc::c_int as libc::c_uint;
            } else if strcmp(
                (secdata as *mut libc::c_char).offset(nameoff as isize),
                b"__wrapper_index\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                index_index = i;
                gnu_sections_found |= 0x2 as libc::c_int as libc::c_uint;
            } else if strcmp(
                (secdata as *mut libc::c_char).offset(nameoff as isize),
                b"__wrapper_sects\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                sections_index = i;
                gnu_sections_found |= 0x1 as libc::c_int as libc::c_uint;
            } else if strcmp(
                (secdata as *mut libc::c_char).offset(nameoff as isize),
                b"__section_names\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                strtab_index = i;
                gnu_sections_found |= 0x8 as libc::c_int as libc::c_uint;
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    if gnu_sections_found
        & (0x1 as libc::c_int | 0x2 as libc::c_int | 0x4 as libc::c_int) as libc::c_uint
        != 0 as libc::c_int as libc::c_uint
    {
        let mut nametab_offset: off_t = 0;
        let mut index_offset: off_t = 0;
        if gnu_sections_found
            & (0x1 as libc::c_int | 0x2 as libc::c_int | 0x4 as libc::c_int)
                as libc::c_uint
            != (0x1 as libc::c_int | 0x2 as libc::c_int | 0x4 as libc::c_int)
                as libc::c_uint
        {
            *errmsg = b"GNU Mach-o section wrapper: required section missing\0"
                as *const u8 as *const libc::c_char;
            *err = 0 as libc::c_int;
            free(secdata as *mut libc::c_void);
            return 0 as libc::c_int;
        }
        simple_object_mach_o_section_info(
            (*omr).is_big_endian,
            is_32,
            secdata
                .offset(
                    (nametab_index as libc::c_ulong).wrapping_mul(sechdrsize) as isize,
                ),
            &mut nametab_offset,
            &mut nametab_size,
        );
        nametab = xmalloc(
            (::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
                .wrapping_mul(nametab_size),
        ) as *mut libc::c_char;
        if simple_object_internal_read(
            (*sobj).descriptor,
            (*sobj).offset + nametab_offset,
            nametab as *mut libc::c_uchar,
            nametab_size,
            errmsg,
            err,
        ) == 0
        {
            free(nametab as *mut libc::c_void);
            free(secdata as *mut libc::c_void);
            return 0 as libc::c_int;
        }
        simple_object_mach_o_section_info(
            (*omr).is_big_endian,
            is_32,
            secdata
                .offset(
                    (index_index as libc::c_ulong).wrapping_mul(sechdrsize) as isize,
                ),
            &mut index_offset,
            &mut index_size,
        );
        index = xmalloc(
            (::core::mem::size_of::<libc::c_uchar>() as libc::c_ulong)
                .wrapping_mul(index_size),
        ) as *mut libc::c_uchar;
        if simple_object_internal_read(
            (*sobj).descriptor,
            (*sobj).offset + index_offset,
            index,
            index_size,
            errmsg,
            err,
        ) == 0
        {
            free(index as *mut libc::c_void);
            free(nametab as *mut libc::c_void);
            free(secdata as *mut libc::c_void);
            return 0 as libc::c_int;
        }
        n_wrapped_sects = index_size.wrapping_div(16 as libc::c_int as libc::c_ulong)
            as libc::c_uint;
        simple_object_mach_o_section_info(
            (*omr).is_big_endian,
            is_32,
            secdata
                .offset(
                    (sections_index as libc::c_ulong).wrapping_mul(sechdrsize) as isize,
                ),
            &mut wrapper_sect_offset,
            &mut wrapper_sect_size,
        );
    } else {
        index = 0 as *mut libc::c_uchar;
        index_size = 0 as libc::c_int as size_t;
        nametab = 0 as *mut libc::c_char;
        nametab_size = 0 as libc::c_int as size_t;
        n_wrapped_sects = 0 as libc::c_int as libc::c_uint;
    }
    if gnu_sections_found & 0x8 as libc::c_int as libc::c_uint
        != 0 as libc::c_int as libc::c_uint
    {
        let mut strtab_offset: off_t = 0;
        simple_object_mach_o_section_info(
            (*omr).is_big_endian,
            is_32,
            secdata
                .offset(
                    (strtab_index as libc::c_ulong).wrapping_mul(sechdrsize) as isize,
                ),
            &mut strtab_offset,
            &mut strtab_size,
        );
        strtab = xmalloc(
            (::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
                .wrapping_mul(strtab_size),
        ) as *mut libc::c_char;
        if simple_object_internal_read(
            (*sobj).descriptor,
            (*sobj).offset + strtab_offset,
            strtab as *mut libc::c_uchar,
            strtab_size,
            errmsg,
            err,
        ) == 0
        {
            free(strtab as *mut libc::c_void);
            free(index as *mut libc::c_void);
            free(nametab as *mut libc::c_void);
            free(secdata as *mut libc::c_void);
            return 0 as libc::c_int;
        }
    } else {
        strtab = 0 as *mut libc::c_char;
        strtab_size = 0 as libc::c_int as size_t;
        strtab_index = nsects;
    }
    let mut current_block_147: u64;
    i = 0 as libc::c_int as libc::c_uint;
    while i < nsects {
        let mut sechdr: *const libc::c_uchar = 0 as *const libc::c_uchar;
        let mut namebuf: [libc::c_char; 34] = [0; 34];
        let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut secoffset: off_t = 0;
        let mut secsize: size_t = 0;
        let mut l: libc::c_int = 0;
        sechdr = secdata.offset((i as libc::c_ulong).wrapping_mul(sechdrsize) as isize);
        if !(gnu_sections_found & 0x8 as libc::c_int as libc::c_uint
            != 0 as libc::c_int as libc::c_uint && i == strtab_index)
        {
            if !(strcmp(
                (sechdr as *mut libc::c_char).offset(segname_offset as isize),
                (*omr).segment_name,
            ) != 0 as libc::c_int)
            {
                if gnu_sections_found
                    & (0x1 as libc::c_int | 0x2 as libc::c_int | 0x4 as libc::c_int)
                        as libc::c_uint != 0 as libc::c_int as libc::c_uint
                {
                    if i == nametab_index || i == index_index {
                        current_block_147 = 11796148217846552555;
                    } else if i == sections_index {
                        let mut j: libc::c_uint = 0;
                        j = 0 as libc::c_int as libc::c_uint;
                        while j < n_wrapped_sects {
                            let mut subsect_offset: libc::c_uint = 0;
                            let mut subsect_length: libc::c_uint = 0;
                            let mut name_offset: libc::c_uint = 0;
                            subsect_offset = (Some(
                                fetch_32.expect("non-null function pointer"),
                            ))
                                .expect(
                                    "non-null function pointer",
                                )(
                                index
                                    .offset(
                                        (16 as libc::c_int as libc::c_uint).wrapping_mul(j) as isize,
                                    ),
                            );
                            subsect_length = (Some(
                                fetch_32.expect("non-null function pointer"),
                            ))
                                .expect(
                                    "non-null function pointer",
                                )(
                                index
                                    .offset(
                                        (16 as libc::c_int as libc::c_uint).wrapping_mul(j) as isize,
                                    )
                                    .offset(4 as libc::c_int as isize),
                            );
                            name_offset = (Some(
                                fetch_32.expect("non-null function pointer"),
                            ))
                                .expect(
                                    "non-null function pointer",
                                )(
                                index
                                    .offset(
                                        (16 as libc::c_int as libc::c_uint).wrapping_mul(j) as isize,
                                    )
                                    .offset(8 as libc::c_int as isize),
                            );
                            secoffset = wrapper_sect_offset
                                + subsect_offset as libc::c_long;
                            secsize = subsect_length as size_t;
                            name = nametab.offset(name_offset as isize);
                            if (Some(pfn.expect("non-null function pointer")))
                                .expect(
                                    "non-null function pointer",
                                )(data, name, secoffset, secsize as off_t) == 0
                            {
                                *errmsg = 0 as *const libc::c_char;
                                *err = 0 as libc::c_int;
                                free(index as *mut libc::c_void);
                                free(nametab as *mut libc::c_void);
                                free(strtab as *mut libc::c_void);
                                free(secdata as *mut libc::c_void);
                                return 0 as libc::c_int;
                            }
                            j = j.wrapping_add(1);
                            j;
                        }
                        current_block_147 = 11796148217846552555;
                    } else {
                        current_block_147 = 4488496028633655612;
                    }
                } else {
                    current_block_147 = 4488496028633655612;
                }
                match current_block_147 {
                    11796148217846552555 => {}
                    _ => {
                        if gnu_sections_found & 0x8 as libc::c_int as libc::c_uint
                            != 0 as libc::c_int as libc::c_uint
                        {
                            memcpy(
                                namebuf.as_mut_ptr() as *mut libc::c_void,
                                sechdr.offset(sectname_offset as isize)
                                    as *const libc::c_void,
                                16 as libc::c_int as libc::c_ulong,
                            );
                            namebuf[16 as libc::c_int
                                as usize] = '\0' as i32 as libc::c_char;
                            name = &mut *namebuf
                                .as_mut_ptr()
                                .offset(0 as libc::c_int as isize) as *mut libc::c_char;
                            if !strtab.is_null()
                                && *name.offset(0 as libc::c_int as isize) as libc::c_int
                                    == '_' as i32
                                && *name.offset(1 as libc::c_int as isize) as libc::c_int
                                    == '_' as i32
                            {
                                let mut stringoffset: libc::c_ulong = 0;
                                if sscanf(
                                    name.offset(2 as libc::c_int as isize),
                                    b"%08lX\0" as *const u8 as *const libc::c_char,
                                    &mut stringoffset as *mut libc::c_ulong,
                                ) == 1 as libc::c_int
                                {
                                    if stringoffset >= strtab_size {
                                        *errmsg = b"section name offset out of range\0" as *const u8
                                            as *const libc::c_char;
                                        *err = 0 as libc::c_int;
                                        free(index as *mut libc::c_void);
                                        free(nametab as *mut libc::c_void);
                                        free(strtab as *mut libc::c_void);
                                        free(secdata as *mut libc::c_void);
                                        return 0 as libc::c_int;
                                    }
                                    name = strtab.offset(stringoffset as isize);
                                }
                            }
                        } else {
                            name = &mut *namebuf
                                .as_mut_ptr()
                                .offset(0 as libc::c_int as isize) as *mut libc::c_char;
                            memcpy(
                                namebuf.as_mut_ptr() as *mut libc::c_void,
                                (sechdr as *mut libc::c_char)
                                    .offset(segname_offset as isize) as *const libc::c_void,
                                16 as libc::c_int as libc::c_ulong,
                            );
                            namebuf[16 as libc::c_int
                                as usize] = '\0' as i32 as libc::c_char;
                            l = strlen(namebuf.as_mut_ptr()) as libc::c_int;
                            namebuf[l as usize] = ',' as i32 as libc::c_char;
                            memcpy(
                                namebuf
                                    .as_mut_ptr()
                                    .offset(l as isize)
                                    .offset(1 as libc::c_int as isize) as *mut libc::c_void,
                                (sechdr as *mut libc::c_char)
                                    .offset(sectname_offset as isize) as *const libc::c_void,
                                16 as libc::c_int as libc::c_ulong,
                            );
                            namebuf[(l + 1 as libc::c_int + 16 as libc::c_int)
                                as usize] = '\0' as i32 as libc::c_char;
                        }
                        simple_object_mach_o_section_info(
                            (*omr).is_big_endian,
                            is_32,
                            sechdr,
                            &mut secoffset,
                            &mut secsize,
                        );
                        if (Some(pfn.expect("non-null function pointer")))
                            .expect(
                                "non-null function pointer",
                            )(data, name, secoffset, secsize as off_t) == 0
                        {
                            *errmsg = 0 as *const libc::c_char;
                            *err = 0 as libc::c_int;
                            free(index as *mut libc::c_void);
                            free(nametab as *mut libc::c_void);
                            free(strtab as *mut libc::c_void);
                            free(secdata as *mut libc::c_void);
                            return 0 as libc::c_int;
                        }
                    }
                }
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    free(index as *mut libc::c_void);
    free(nametab as *mut libc::c_void);
    free(strtab as *mut libc::c_void);
    free(secdata as *mut libc::c_void);
    return 1 as libc::c_int;
}
unsafe extern "C" fn simple_object_mach_o_find_sections(
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
    let mut omr: *mut simple_object_mach_o_read = (*sobj).data
        as *mut simple_object_mach_o_read;
    let mut offset: off_t = 0;
    let mut seghdrsize: size_t = 0;
    let mut fetch_32: Option::<
        unsafe extern "C" fn(*const libc::c_uchar) -> libc::c_uint,
    > = None;
    let mut errmsg: *const libc::c_char = 0 as *const libc::c_char;
    let mut i: libc::c_uint = 0;
    if (*omr).magic == 0xfeedface as libc::c_uint {
        offset = ::core::mem::size_of::<mach_o_header_32>() as libc::c_ulong as off_t;
        seghdrsize = ::core::mem::size_of::<mach_o_segment_command_32>()
            as libc::c_ulong;
    } else {
        offset = ::core::mem::size_of::<mach_o_header_64>() as libc::c_ulong as off_t;
        seghdrsize = ::core::mem::size_of::<mach_o_segment_command_64>()
            as libc::c_ulong;
    }
    fetch_32 = if (*omr).is_big_endian != 0 {
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
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*omr).ncmds {
        let mut loadbuf: [libc::c_uchar; 8] = [0; 8];
        let mut cmd: libc::c_uint = 0;
        let mut cmdsize: libc::c_uint = 0;
        if simple_object_internal_read(
            (*sobj).descriptor,
            (*sobj).offset + offset,
            loadbuf.as_mut_ptr(),
            ::core::mem::size_of::<mach_o_load_command>() as libc::c_ulong,
            &mut errmsg,
            err,
        ) == 0
        {
            return errmsg;
        }
        cmd = (Some(fetch_32.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(loadbuf.as_mut_ptr().offset(0 as libc::c_ulong as isize));
        cmdsize = (Some(fetch_32.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(loadbuf.as_mut_ptr().offset(4 as libc::c_ulong as isize));
        if cmd == 0x1 as libc::c_int as libc::c_uint
            || cmd == 0x19 as libc::c_int as libc::c_uint
        {
            let mut segbuf: [libc::c_uchar; 72] = [0; 72];
            let mut r: libc::c_int = 0;
            if simple_object_internal_read(
                (*sobj).descriptor,
                (*sobj).offset + offset,
                segbuf.as_mut_ptr(),
                seghdrsize,
                &mut errmsg,
                err,
            ) == 0
            {
                return errmsg;
            }
            r = simple_object_mach_o_segment(
                sobj,
                offset,
                segbuf.as_mut_ptr(),
                pfn,
                data,
                &mut errmsg,
                err,
            );
            if r == 0 {
                return errmsg;
            }
        }
        offset += cmdsize as libc::c_long;
        i = i.wrapping_add(1);
        i;
    }
    return 0 as *const libc::c_char;
}
unsafe extern "C" fn simple_object_mach_o_fetch_attributes(
    mut sobj: *mut simple_object_read,
    mut errmsg: *mut *const libc::c_char,
    mut err: *mut libc::c_int,
) -> *mut libc::c_void {
    let mut omr: *mut simple_object_mach_o_read = (*sobj).data
        as *mut simple_object_mach_o_read;
    let mut ret: *mut simple_object_mach_o_attributes = 0
        as *mut simple_object_mach_o_attributes;
    ret = xmalloc(
        ::core::mem::size_of::<simple_object_mach_o_attributes>() as libc::c_ulong,
    ) as *mut simple_object_mach_o_attributes;
    (*ret).magic = (*omr).magic;
    (*ret).is_big_endian = (*omr).is_big_endian;
    (*ret).cputype = (*omr).cputype;
    (*ret).cpusubtype = (*omr).cpusubtype;
    (*ret).flags = (*omr).flags;
    (*ret).reserved = (*omr).reserved;
    return ret as *mut libc::c_void;
}
unsafe extern "C" fn simple_object_mach_o_release_read(mut data: *mut libc::c_void) {
    let mut omr: *mut simple_object_mach_o_read = data as *mut simple_object_mach_o_read;
    free((*omr).segment_name as *mut libc::c_void);
    free(omr as *mut libc::c_void);
}
unsafe extern "C" fn simple_object_mach_o_attributes_merge(
    mut todata: *mut libc::c_void,
    mut fromdata: *mut libc::c_void,
    mut err: *mut libc::c_int,
) -> *const libc::c_char {
    let mut to: *mut simple_object_mach_o_attributes = todata
        as *mut simple_object_mach_o_attributes;
    let mut from: *mut simple_object_mach_o_attributes = fromdata
        as *mut simple_object_mach_o_attributes;
    if (*to).magic != (*from).magic || (*to).is_big_endian != (*from).is_big_endian
        || (*to).cputype != (*from).cputype
    {
        *err = 0 as libc::c_int;
        return b"Mach-O object format mismatch\0" as *const u8 as *const libc::c_char;
    }
    return 0 as *const libc::c_char;
}
unsafe extern "C" fn simple_object_mach_o_release_attributes(
    mut data: *mut libc::c_void,
) {
    free(data);
}
unsafe extern "C" fn simple_object_mach_o_start_write(
    mut attributes_data: *mut libc::c_void,
    mut errmsg: *mut *const libc::c_char,
    mut err: *mut libc::c_int,
) -> *mut libc::c_void {
    let mut attrs: *mut simple_object_mach_o_attributes = attributes_data
        as *mut simple_object_mach_o_attributes;
    let mut ret: *mut simple_object_mach_o_attributes = 0
        as *mut simple_object_mach_o_attributes;
    ret = xmalloc(
        ::core::mem::size_of::<simple_object_mach_o_attributes>() as libc::c_ulong,
    ) as *mut simple_object_mach_o_attributes;
    *ret = *attrs;
    return ret as *mut libc::c_void;
}
unsafe extern "C" fn simple_object_mach_o_write_header(
    mut sobj: *mut simple_object_write,
    mut descriptor: libc::c_int,
    mut nsects: size_t,
    mut errmsg: *mut *const libc::c_char,
    mut err: *mut libc::c_int,
) -> libc::c_int {
    let mut attrs: *mut simple_object_mach_o_attributes = (*sobj).data
        as *mut simple_object_mach_o_attributes;
    let mut set_32: Option::<
        unsafe extern "C" fn(*mut libc::c_uchar, libc::c_uint) -> (),
    > = None;
    let mut hdrbuf: [libc::c_uchar; 32] = [0; 32];
    let mut hdr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut wrsize: size_t = 0;
    set_32 = if (*attrs).is_big_endian != 0 {
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
        ::core::mem::size_of::<[libc::c_uchar; 32]>() as libc::c_ulong,
    );
    hdr = &mut *hdrbuf.as_mut_ptr().offset(0 as libc::c_int as isize)
        as *mut libc::c_uchar;
    set_32
        .expect(
            "non-null function pointer",
        )(hdr.offset(0 as libc::c_ulong as isize), (*attrs).magic);
    set_32
        .expect(
            "non-null function pointer",
        )(hdr.offset(4 as libc::c_ulong as isize), (*attrs).cputype);
    set_32
        .expect(
            "non-null function pointer",
        )(hdr.offset(8 as libc::c_ulong as isize), (*attrs).cpusubtype);
    set_32
        .expect(
            "non-null function pointer",
        )(hdr.offset(12 as libc::c_ulong as isize), 0x1 as libc::c_int as libc::c_uint);
    set_32
        .expect(
            "non-null function pointer",
        )(hdr.offset(16 as libc::c_ulong as isize), 1 as libc::c_int as libc::c_uint);
    set_32
        .expect(
            "non-null function pointer",
        )(hdr.offset(24 as libc::c_ulong as isize), (*attrs).flags);
    if (*attrs).magic == 0xfeedface as libc::c_uint {
        wrsize = ::core::mem::size_of::<mach_o_header_32>() as libc::c_ulong;
        set_32
            .expect(
                "non-null function pointer",
            )(
            hdr.offset(20 as libc::c_ulong as isize),
            (::core::mem::size_of::<mach_o_segment_command_32>() as libc::c_ulong)
                .wrapping_add(
                    nsects
                        .wrapping_mul(
                            ::core::mem::size_of::<mach_o_section_32>() as libc::c_ulong,
                        ),
                ) as libc::c_uint,
        );
    } else {
        set_32
            .expect(
                "non-null function pointer",
            )(
            hdr.offset(20 as libc::c_ulong as isize),
            (::core::mem::size_of::<mach_o_segment_command_64>() as libc::c_ulong)
                .wrapping_add(
                    nsects
                        .wrapping_mul(
                            ::core::mem::size_of::<mach_o_section_64>() as libc::c_ulong,
                        ),
                ) as libc::c_uint,
        );
        set_32
            .expect(
                "non-null function pointer",
            )(hdr.offset(28 as libc::c_ulong as isize), (*attrs).reserved);
        wrsize = ::core::mem::size_of::<mach_o_header_64>() as libc::c_ulong;
    }
    return simple_object_internal_write(
        descriptor,
        0 as libc::c_int as off_t,
        hdrbuf.as_mut_ptr(),
        wrsize,
        errmsg,
        err,
    );
}
unsafe extern "C" fn simple_object_mach_o_write_section_header(
    mut sobj: *mut simple_object_write,
    mut descriptor: libc::c_int,
    mut sechdr_offset: size_t,
    mut name: *const libc::c_char,
    mut segn: *const libc::c_char,
    mut secaddr: size_t,
    mut secsize: size_t,
    mut offset: size_t,
    mut align: libc::c_uint,
    mut errmsg: *mut *const libc::c_char,
    mut err: *mut libc::c_int,
) -> libc::c_int {
    let mut attrs: *mut simple_object_mach_o_attributes = (*sobj).data
        as *mut simple_object_mach_o_attributes;
    let mut set_32: Option::<
        unsafe extern "C" fn(*mut libc::c_uchar, libc::c_uint) -> (),
    > = None;
    let mut hdrbuf: [libc::c_uchar; 80] = [0; 80];
    let mut hdr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut sechdrsize: size_t = 0;
    set_32 = if (*attrs).is_big_endian != 0 {
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
        ::core::mem::size_of::<[libc::c_uchar; 80]>() as libc::c_ulong,
    );
    hdr = &mut *hdrbuf.as_mut_ptr().offset(0 as libc::c_int as isize)
        as *mut libc::c_uchar;
    if (*attrs).magic == 0xfeedface as libc::c_uint {
        strncpy(
            (hdr as *mut libc::c_char).offset(0 as libc::c_ulong as isize),
            name,
            16 as libc::c_int as libc::c_ulong,
        );
        strncpy(
            (hdr as *mut libc::c_char).offset(16 as libc::c_ulong as isize),
            segn,
            16 as libc::c_int as libc::c_ulong,
        );
        set_32
            .expect(
                "non-null function pointer",
            )(hdr.offset(32 as libc::c_ulong as isize), secaddr as libc::c_uint);
        set_32
            .expect(
                "non-null function pointer",
            )(hdr.offset(36 as libc::c_ulong as isize), secsize as libc::c_uint);
        set_32
            .expect(
                "non-null function pointer",
            )(hdr.offset(40 as libc::c_ulong as isize), offset as libc::c_uint);
        set_32
            .expect(
                "non-null function pointer",
            )(hdr.offset(44 as libc::c_ulong as isize), align);
        set_32
            .expect(
                "non-null function pointer",
            )(
            hdr.offset(56 as libc::c_ulong as isize),
            0x2000000 as libc::c_int as libc::c_uint,
        );
        sechdrsize = ::core::mem::size_of::<mach_o_section_32>() as libc::c_ulong;
    } else {
        let mut set_64: Option::<
            unsafe extern "C" fn(*mut libc::c_uchar, ulong_type) -> (),
        > = None;
        set_64 = if (*attrs).is_big_endian != 0 {
            Some(
                simple_object_set_big_64
                    as unsafe extern "C" fn(*mut libc::c_uchar, ulong_type) -> (),
            )
        } else {
            Some(
                simple_object_set_little_64
                    as unsafe extern "C" fn(*mut libc::c_uchar, ulong_type) -> (),
            )
        };
        strncpy(
            (hdr as *mut libc::c_char).offset(0 as libc::c_ulong as isize),
            name,
            16 as libc::c_int as libc::c_ulong,
        );
        strncpy(
            (hdr as *mut libc::c_char).offset(16 as libc::c_ulong as isize),
            segn,
            16 as libc::c_int as libc::c_ulong,
        );
        set_64
            .expect(
                "non-null function pointer",
            )(hdr.offset(32 as libc::c_ulong as isize), secaddr);
        set_64
            .expect(
                "non-null function pointer",
            )(hdr.offset(40 as libc::c_ulong as isize), secsize);
        set_32
            .expect(
                "non-null function pointer",
            )(hdr.offset(48 as libc::c_ulong as isize), offset as libc::c_uint);
        set_32
            .expect(
                "non-null function pointer",
            )(hdr.offset(52 as libc::c_ulong as isize), align);
        set_32
            .expect(
                "non-null function pointer",
            )(
            hdr.offset(64 as libc::c_ulong as isize),
            0x2000000 as libc::c_int as libc::c_uint,
        );
        sechdrsize = ::core::mem::size_of::<mach_o_section_64>() as libc::c_ulong;
    }
    return simple_object_internal_write(
        descriptor,
        sechdr_offset as off_t,
        hdr,
        sechdrsize,
        errmsg,
        err,
    );
}
unsafe extern "C" fn simple_object_mach_o_write_segment(
    mut sobj: *mut simple_object_write,
    mut descriptor: libc::c_int,
    mut nsects: *mut size_t,
    mut errmsg: *mut *const libc::c_char,
    mut err: *mut libc::c_int,
) -> libc::c_int {
    let mut attrs: *mut simple_object_mach_o_attributes = (*sobj).data
        as *mut simple_object_mach_o_attributes;
    let mut set_32: Option::<
        unsafe extern "C" fn(*mut libc::c_uchar, libc::c_uint) -> (),
    > = None;
    let mut hdrsize: size_t = 0;
    let mut seghdrsize: size_t = 0;
    let mut sechdrsize: size_t = 0;
    let mut cmdsize: size_t = 0;
    let mut offset: size_t = 0;
    let mut sechdr_offset: size_t = 0;
    let mut secaddr: size_t = 0;
    let mut name_offset: libc::c_uint = 0;
    let mut section: *mut simple_object_write_section = 0
        as *mut simple_object_write_section;
    let mut hdrbuf: [libc::c_uchar; 72] = [0; 72];
    let mut hdr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut nsects_in: size_t = 0;
    let mut index: *mut libc::c_uint = 0 as *mut libc::c_uint;
    let mut snames: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sect: libc::c_uint = 0;
    set_32 = if (*attrs).is_big_endian != 0 {
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
    if (*attrs).magic == 0xfeedface as libc::c_uint {
        hdrsize = ::core::mem::size_of::<mach_o_header_32>() as libc::c_ulong;
        seghdrsize = ::core::mem::size_of::<mach_o_segment_command_32>()
            as libc::c_ulong;
        sechdrsize = ::core::mem::size_of::<mach_o_section_32>() as libc::c_ulong;
    } else {
        hdrsize = ::core::mem::size_of::<mach_o_header_64>() as libc::c_ulong;
        seghdrsize = ::core::mem::size_of::<mach_o_segment_command_64>()
            as libc::c_ulong;
        sechdrsize = ::core::mem::size_of::<mach_o_section_64>() as libc::c_ulong;
    }
    name_offset = 0 as libc::c_int as libc::c_uint;
    nsects_in = 0 as libc::c_int as size_t;
    *nsects = nsects_in;
    section = (*sobj).sections;
    while !section.is_null() {
        nsects_in = nsects_in.wrapping_add(1);
        nsects_in;
        section = (*section).next;
    }
    if !((*sobj).segment_name).is_null() {
        *nsects = 3 as libc::c_int as size_t;
        index = xmalloc(
            (::core::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(nsects_in.wrapping_mul(4 as libc::c_int as libc::c_ulong)),
        ) as *mut libc::c_uint;
        section = (*sobj).sections;
        sect = 0 as libc::c_int as libc::c_uint;
        while !section.is_null() {
            *index
                .offset(
                    sect
                        .wrapping_mul(4 as libc::c_int as libc::c_uint)
                        .wrapping_add(2 as libc::c_int as libc::c_uint) as isize,
                ) = name_offset;
            *index
                .offset(
                    sect
                        .wrapping_mul(4 as libc::c_int as libc::c_uint)
                        .wrapping_add(3 as libc::c_int as libc::c_uint) as isize,
                ) = (strlen((*section).name))
                .wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_uint;
            name_offset = (name_offset as libc::c_ulong)
                .wrapping_add(
                    (strlen((*section).name))
                        .wrapping_add(1 as libc::c_int as libc::c_ulong),
                ) as libc::c_uint as libc::c_uint;
            section = (*section).next;
            sect = sect.wrapping_add(1);
            sect;
        }
        snames = xmalloc(
            (::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
                .wrapping_mul(name_offset as libc::c_ulong),
        ) as *mut libc::c_char;
    } else {
        *nsects = nsects_in;
        index = 0 as *mut libc::c_uint;
        snames = 0 as *mut libc::c_char;
    }
    sechdr_offset = hdrsize.wrapping_add(seghdrsize);
    cmdsize = seghdrsize.wrapping_add((*nsects).wrapping_mul(sechdrsize));
    offset = hdrsize.wrapping_add(cmdsize);
    secaddr = 0 as libc::c_int as size_t;
    section = (*sobj).sections;
    sect = 0 as libc::c_int as libc::c_uint;
    while !section.is_null() {
        let mut mask: size_t = 0;
        let mut new_offset: size_t = 0;
        let mut secsize: size_t = 0;
        let mut buffer: *mut simple_object_write_section_buffer = 0
            as *mut simple_object_write_section_buffer;
        mask = ((1 as libc::c_uint) << (*section).align)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t;
        new_offset = offset.wrapping_add(mask);
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
                errmsg,
                err,
            ) == 0
            {
                return 0 as libc::c_int;
            }
            offset = (offset as libc::c_ulong).wrapping_add(write) as size_t as size_t;
        }
        secsize = 0 as libc::c_int as size_t;
        buffer = (*section).buffers;
        while !buffer.is_null() {
            if simple_object_internal_write(
                descriptor,
                offset.wrapping_add(secsize) as off_t,
                (*buffer).buffer as *const libc::c_uchar,
                (*buffer).size,
                errmsg,
                err,
            ) == 0
            {
                return 0 as libc::c_int;
            }
            secsize = (secsize as libc::c_ulong).wrapping_add((*buffer).size) as size_t
                as size_t;
            buffer = (*buffer).next;
        }
        if !((*sobj).segment_name).is_null() {
            *index
                .offset(
                    sect
                        .wrapping_mul(4 as libc::c_int as libc::c_uint)
                        .wrapping_add(0 as libc::c_int as libc::c_uint) as isize,
                ) = offset as libc::c_uint;
            *index
                .offset(
                    sect
                        .wrapping_mul(4 as libc::c_int as libc::c_uint)
                        .wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                ) = secsize as libc::c_uint;
            memcpy(
                snames
                    .offset(
                        *index
                            .offset(
                                sect
                                    .wrapping_mul(4 as libc::c_int as libc::c_uint)
                                    .wrapping_add(2 as libc::c_int as libc::c_uint) as isize,
                            ) as isize,
                    ) as *mut libc::c_void,
                (*section).name as *const libc::c_void,
                *index
                    .offset(
                        sect
                            .wrapping_mul(4 as libc::c_int as libc::c_uint)
                            .wrapping_add(3 as libc::c_int as libc::c_uint) as isize,
                    ) as libc::c_ulong,
            );
        } else {
            let mut namebuf: [libc::c_char; 17] = [0; 17];
            let mut segnbuf: [libc::c_char; 17] = [0; 17];
            let mut comma: *mut libc::c_char = 0 as *mut libc::c_char;
            memset(
                namebuf.as_mut_ptr() as *mut libc::c_void,
                0 as libc::c_int,
                ::core::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong,
            );
            memset(
                segnbuf.as_mut_ptr() as *mut libc::c_void,
                0 as libc::c_int,
                ::core::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong,
            );
            comma = strchr((*section).name, ',' as i32);
            if !comma.is_null() {
                let mut len: libc::c_int = comma.offset_from((*section).name)
                    as libc::c_long as libc::c_int;
                len = if len > 16 as libc::c_int { 16 as libc::c_int } else { len };
                strncpy(namebuf.as_mut_ptr(), (*section).name, len as libc::c_ulong);
                strncpy(
                    segnbuf.as_mut_ptr(),
                    comma.offset(1 as libc::c_int as isize),
                    16 as libc::c_int as libc::c_ulong,
                );
            } else {
                strncpy(
                    namebuf.as_mut_ptr(),
                    (*section).name,
                    16 as libc::c_int as libc::c_ulong,
                );
            }
            if simple_object_mach_o_write_section_header(
                sobj,
                descriptor,
                sechdr_offset,
                namebuf.as_mut_ptr(),
                segnbuf.as_mut_ptr(),
                secaddr,
                secsize,
                offset,
                (*section).align,
                errmsg,
                err,
            ) == 0
            {
                return 0 as libc::c_int;
            }
            sechdr_offset = (sechdr_offset as libc::c_ulong).wrapping_add(sechdrsize)
                as size_t as size_t;
        }
        offset = (offset as libc::c_ulong).wrapping_add(secsize) as size_t as size_t;
        secaddr = (secaddr as libc::c_ulong).wrapping_add(secsize) as size_t as size_t;
        section = (*section).next;
        sect = sect.wrapping_add(1);
        sect;
    }
    if !((*sobj).segment_name).is_null() {
        let mut secsize_0: size_t = 0;
        let mut i: libc::c_uint = 0;
        secsize_0 = offset
            .wrapping_sub(*index.offset(0 as libc::c_int as isize) as libc::c_ulong);
        if simple_object_mach_o_write_section_header(
            sobj,
            descriptor,
            sechdr_offset,
            b"__wrapper_sects\0" as *const u8 as *const libc::c_char,
            (*sobj).segment_name,
            0 as libc::c_int as size_t,
            secsize_0,
            *index.offset(0 as libc::c_int as isize) as size_t,
            (*(*sobj).sections).align,
            errmsg,
            err,
        ) == 0
        {
            return 0 as libc::c_int;
        }
        i = 1 as libc::c_int as libc::c_uint;
        while (i as libc::c_ulong) < nsects_in {
            let ref mut fresh0 = *index
                .offset((4 as libc::c_int as libc::c_uint).wrapping_mul(i) as isize);
            *fresh0 = (*fresh0).wrapping_sub(*index.offset(0 as libc::c_int as isize));
            i = i.wrapping_add(1);
            i;
        }
        *index.offset(0 as libc::c_int as isize) = 0 as libc::c_int as libc::c_uint;
        sechdr_offset = (sechdr_offset as libc::c_ulong).wrapping_add(sechdrsize)
            as size_t as size_t;
        if simple_object_mach_o_write_section_header(
            sobj,
            descriptor,
            sechdr_offset,
            b"__wrapper_names\0" as *const u8 as *const libc::c_char,
            (*sobj).segment_name,
            0 as libc::c_int as size_t,
            name_offset as size_t,
            offset,
            0 as libc::c_int as libc::c_uint,
            errmsg,
            err,
        ) == 0
        {
            return 0 as libc::c_int;
        }
        if simple_object_internal_write(
            descriptor,
            offset as off_t,
            snames as *const libc::c_uchar,
            name_offset as size_t,
            errmsg,
            err,
        ) == 0
        {
            return 0 as libc::c_int;
        }
        sechdr_offset = (sechdr_offset as libc::c_ulong).wrapping_add(sechdrsize)
            as size_t as size_t;
        secaddr = (secaddr as libc::c_ulong).wrapping_add(name_offset as libc::c_ulong)
            as size_t as size_t;
        offset = (offset as libc::c_ulong).wrapping_add(name_offset as libc::c_ulong)
            as size_t as size_t;
        offset = (offset as libc::c_ulong)
            .wrapping_add(3 as libc::c_int as libc::c_ulong) as size_t as size_t;
        offset &= !(0x3 as libc::c_int) as libc::c_ulong;
        if simple_object_mach_o_write_section_header(
            sobj,
            descriptor,
            sechdr_offset,
            b"__wrapper_index\0" as *const u8 as *const libc::c_char,
            (*sobj).segment_name,
            0 as libc::c_int as size_t,
            nsects_in.wrapping_mul(16 as libc::c_int as libc::c_ulong),
            offset,
            2 as libc::c_int as libc::c_uint,
            errmsg,
            err,
        ) == 0
        {
            return 0 as libc::c_int;
        }
        if simple_object_internal_write(
            descriptor,
            offset as off_t,
            index as *const libc::c_uchar,
            nsects_in.wrapping_mul(16 as libc::c_int as libc::c_ulong),
            errmsg,
            err,
        ) == 0
        {
            return 0 as libc::c_int;
        }
        free(index as *mut libc::c_void);
        free(snames as *mut libc::c_void);
    }
    memset(
        hdrbuf.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<[libc::c_uchar; 72]>() as libc::c_ulong,
    );
    hdr = &mut *hdrbuf.as_mut_ptr().offset(0 as libc::c_int as isize)
        as *mut libc::c_uchar;
    if (*attrs).magic == 0xfeedface as libc::c_uint {
        set_32
            .expect(
                "non-null function pointer",
            )(
            hdr.offset(0 as libc::c_ulong as isize),
            0x1 as libc::c_int as libc::c_uint,
        );
        set_32
            .expect(
                "non-null function pointer",
            )(hdr.offset(4 as libc::c_ulong as isize), cmdsize as libc::c_uint);
        set_32
            .expect(
                "non-null function pointer",
            )(
            hdr.offset(32 as libc::c_ulong as isize),
            hdrsize.wrapping_add(cmdsize) as libc::c_uint,
        );
        set_32
            .expect(
                "non-null function pointer",
            )(
            hdr.offset(36 as libc::c_ulong as isize),
            offset.wrapping_sub(hdrsize.wrapping_add(cmdsize)) as libc::c_uint,
        );
        set_32
            .expect(
                "non-null function pointer",
            )(hdr.offset(48 as libc::c_ulong as isize), *nsects as libc::c_uint);
    } else {
        let mut set_64: Option::<
            unsafe extern "C" fn(*mut libc::c_uchar, ulong_type) -> (),
        > = None;
        set_64 = if (*attrs).is_big_endian != 0 {
            Some(
                simple_object_set_big_64
                    as unsafe extern "C" fn(*mut libc::c_uchar, ulong_type) -> (),
            )
        } else {
            Some(
                simple_object_set_little_64
                    as unsafe extern "C" fn(*mut libc::c_uchar, ulong_type) -> (),
            )
        };
        set_32
            .expect(
                "non-null function pointer",
            )(
            hdr.offset(0 as libc::c_ulong as isize),
            0x1 as libc::c_int as libc::c_uint,
        );
        set_32
            .expect(
                "non-null function pointer",
            )(hdr.offset(4 as libc::c_ulong as isize), cmdsize as libc::c_uint);
        set_64
            .expect(
                "non-null function pointer",
            )(hdr.offset(40 as libc::c_ulong as isize), hdrsize.wrapping_add(cmdsize));
        set_64
            .expect(
                "non-null function pointer",
            )(
            hdr.offset(48 as libc::c_ulong as isize),
            offset.wrapping_sub(hdrsize.wrapping_add(cmdsize)),
        );
        set_32
            .expect(
                "non-null function pointer",
            )(hdr.offset(64 as libc::c_ulong as isize), *nsects as libc::c_uint);
    }
    return simple_object_internal_write(
        descriptor,
        hdrsize as off_t,
        hdr,
        seghdrsize,
        errmsg,
        err,
    );
}
unsafe extern "C" fn simple_object_mach_o_write_to_file(
    mut sobj: *mut simple_object_write,
    mut descriptor: libc::c_int,
    mut err: *mut libc::c_int,
) -> *const libc::c_char {
    let mut nsects: size_t = 0 as libc::c_int as size_t;
    let mut errmsg: *const libc::c_char = 0 as *const libc::c_char;
    if simple_object_mach_o_write_segment(
        sobj,
        descriptor,
        &mut nsects,
        &mut errmsg,
        err,
    ) == 0
    {
        return errmsg;
    }
    if simple_object_mach_o_write_header(sobj, descriptor, nsects, &mut errmsg, err) == 0
    {
        return errmsg;
    }
    return 0 as *const libc::c_char;
}
unsafe extern "C" fn simple_object_mach_o_release_write(mut data: *mut libc::c_void) {
    free(data);
}
#[no_mangle]
pub static mut simple_object_mach_o_functions: simple_object_functions = unsafe {
    {
        let mut init = simple_object_functions {
            match_0: Some(
                simple_object_mach_o_match
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
                simple_object_mach_o_find_sections
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
                simple_object_mach_o_fetch_attributes
                    as unsafe extern "C" fn(
                        *mut simple_object_read,
                        *mut *const libc::c_char,
                        *mut libc::c_int,
                    ) -> *mut libc::c_void,
            ),
            release_read: Some(
                simple_object_mach_o_release_read
                    as unsafe extern "C" fn(*mut libc::c_void) -> (),
            ),
            attributes_merge: Some(
                simple_object_mach_o_attributes_merge
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut libc::c_void,
                        *mut libc::c_int,
                    ) -> *const libc::c_char,
            ),
            release_attributes: Some(
                simple_object_mach_o_release_attributes
                    as unsafe extern "C" fn(*mut libc::c_void) -> (),
            ),
            start_write: Some(
                simple_object_mach_o_start_write
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut *const libc::c_char,
                        *mut libc::c_int,
                    ) -> *mut libc::c_void,
            ),
            write_to_file: Some(
                simple_object_mach_o_write_to_file
                    as unsafe extern "C" fn(
                        *mut simple_object_write,
                        libc::c_int,
                        *mut libc::c_int,
                    ) -> *const libc::c_char,
            ),
            release_write: Some(
                simple_object_mach_o_release_write
                    as unsafe extern "C" fn(*mut libc::c_void) -> (),
            ),
            copy_lto_debug_sections: None,
        };
        init
    }
};
