use ::libc;
extern "C" {
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    fn xcalloc(_: size_t, _: size_t) -> *mut libc::c_void;
    fn simple_object_write_create_section(
        simple_object: *mut simple_object_write,
        name: *const libc::c_char,
        align: libc::c_uint,
        errmsg: *mut *const libc::c_char,
        err: *mut libc::c_int,
    ) -> *mut simple_object_write_section;
    fn simple_object_write_add_data(
        simple_object: *mut simple_object_write,
        section: *mut simple_object_write_section,
        buffer: *const libc::c_void,
        size: size_t,
        copy: libc::c_int,
        err: *mut libc::c_int,
    ) -> *const libc::c_char;
    fn free(_: *mut libc::c_void);
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
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
pub type ulong_type = uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct elf_type_functions {
    pub fetch_Elf_Half: Option::<
        unsafe extern "C" fn(*const libc::c_uchar) -> libc::c_ushort,
    >,
    pub fetch_Elf_Word: Option::<
        unsafe extern "C" fn(*const libc::c_uchar) -> libc::c_uint,
    >,
    pub fetch_Elf_Addr: Option::<
        unsafe extern "C" fn(*const libc::c_uchar) -> ulong_type,
    >,
    pub set_Elf_Half: Option::<
        unsafe extern "C" fn(*mut libc::c_uchar, libc::c_ushort) -> (),
    >,
    pub set_Elf_Word: Option::<
        unsafe extern "C" fn(*mut libc::c_uchar, libc::c_uint) -> (),
    >,
    pub set_Elf_Addr: Option::<
        unsafe extern "C" fn(*mut libc::c_uchar, ulong_type) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct simple_object_elf_read {
    pub type_functions: *const elf_type_functions,
    pub ei_data: libc::c_uchar,
    pub ei_class: libc::c_uchar,
    pub ei_osabi: libc::c_uchar,
    pub machine: libc::c_ushort,
    pub flags: libc::c_uint,
    pub shoff: ulong_type,
    pub shnum: libc::c_uint,
    pub shstrndx: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Elf64_External_Sym {
    pub st_name: [libc::c_uchar; 4],
    pub st_info: libc::c_uchar,
    pub st_other: libc::c_uchar,
    pub st_shndx: [libc::c_uchar; 2],
    pub st_value: [libc::c_uchar; 8],
    pub st_size: [libc::c_uchar; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Elf32_External_Sym {
    pub st_name: [libc::c_uchar; 4],
    pub st_value: [libc::c_uchar; 4],
    pub st_size: [libc::c_uchar; 4],
    pub st_info: libc::c_uchar,
    pub st_other: libc::c_uchar,
    pub st_shndx: [libc::c_uchar; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct simple_object_elf_write {
    pub attrs: simple_object_elf_attributes,
    pub shdrs: *mut libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct simple_object_elf_attributes {
    pub type_functions: *const elf_type_functions,
    pub ei_data: libc::c_uchar,
    pub ei_class: libc::c_uchar,
    pub ei_osabi: libc::c_uchar,
    pub machine: libc::c_ushort,
    pub flags: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Elf64_External_Shdr {
    pub sh_name: [libc::c_uchar; 4],
    pub sh_type: [libc::c_uchar; 4],
    pub sh_flags: [libc::c_uchar; 8],
    pub sh_addr: [libc::c_uchar; 8],
    pub sh_offset: [libc::c_uchar; 8],
    pub sh_size: [libc::c_uchar; 8],
    pub sh_link: [libc::c_uchar; 4],
    pub sh_info: [libc::c_uchar; 4],
    pub sh_addralign: [libc::c_uchar; 8],
    pub sh_entsize: [libc::c_uchar; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Elf32_External_Shdr {
    pub sh_name: [libc::c_uchar; 4],
    pub sh_type: [libc::c_uchar; 4],
    pub sh_flags: [libc::c_uchar; 4],
    pub sh_addr: [libc::c_uchar; 4],
    pub sh_offset: [libc::c_uchar; 4],
    pub sh_size: [libc::c_uchar; 4],
    pub sh_link: [libc::c_uchar; 4],
    pub sh_info: [libc::c_uchar; 4],
    pub sh_addralign: [libc::c_uchar; 4],
    pub sh_entsize: [libc::c_uchar; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Elf64_External_Ehdr {
    pub e_ident: [libc::c_uchar; 16],
    pub e_type: [libc::c_uchar; 2],
    pub e_machine: [libc::c_uchar; 2],
    pub e_version: [libc::c_uchar; 4],
    pub e_entry: [libc::c_uchar; 8],
    pub e_phoff: [libc::c_uchar; 8],
    pub e_shoff: [libc::c_uchar; 8],
    pub e_flags: [libc::c_uchar; 4],
    pub e_ehsize: [libc::c_uchar; 2],
    pub e_phentsize: [libc::c_uchar; 2],
    pub e_phnum: [libc::c_uchar; 2],
    pub e_shentsize: [libc::c_uchar; 2],
    pub e_shnum: [libc::c_uchar; 2],
    pub e_shstrndx: [libc::c_uchar; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Elf32_External_Ehdr {
    pub e_ident: [libc::c_uchar; 16],
    pub e_type: [libc::c_uchar; 2],
    pub e_machine: [libc::c_uchar; 2],
    pub e_version: [libc::c_uchar; 4],
    pub e_entry: [libc::c_uchar; 4],
    pub e_phoff: [libc::c_uchar; 4],
    pub e_shoff: [libc::c_uchar; 4],
    pub e_flags: [libc::c_uchar; 4],
    pub e_ehsize: [libc::c_uchar; 2],
    pub e_phentsize: [libc::c_uchar; 2],
    pub e_phnum: [libc::c_uchar; 2],
    pub e_shentsize: [libc::c_uchar; 2],
    pub e_shnum: [libc::c_uchar; 2],
    pub e_shstrndx: [libc::c_uchar; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Elf64_External_Phdr {
    pub p_type: [libc::c_uchar; 4],
    pub p_flags: [libc::c_uchar; 4],
    pub p_offset: [libc::c_uchar; 8],
    pub p_vaddr: [libc::c_uchar; 8],
    pub p_paddr: [libc::c_uchar; 8],
    pub p_filesz: [libc::c_uchar; 8],
    pub p_memsz: [libc::c_uchar; 8],
    pub p_align: [libc::c_uchar; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Elf32_External_Phdr {
    pub p_type: [libc::c_uchar; 4],
    pub p_offset: [libc::c_uchar; 4],
    pub p_vaddr: [libc::c_uchar; 4],
    pub p_paddr: [libc::c_uchar; 4],
    pub p_filesz: [libc::c_uchar; 4],
    pub p_memsz: [libc::c_uchar; 4],
    pub p_flags: [libc::c_uchar; 4],
    pub p_align: [libc::c_uchar; 4],
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
unsafe extern "C" fn simple_object_fetch_big_32_ulong(
    mut buf: *const libc::c_uchar,
) -> ulong_type {
    return simple_object_fetch_big_32(buf) as ulong_type;
}
#[inline]
unsafe extern "C" fn simple_object_fetch_little_32_ulong(
    mut buf: *const libc::c_uchar,
) -> ulong_type {
    return simple_object_fetch_little_32(buf) as ulong_type;
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
#[inline]
unsafe extern "C" fn simple_object_set_big_32_ulong(
    mut buf: *mut libc::c_uchar,
    mut val: ulong_type,
) {
    simple_object_set_big_32(buf, val as libc::c_uint);
}
#[inline]
unsafe extern "C" fn simple_object_set_little_32_ulong(
    mut buf: *mut libc::c_uchar,
    mut val: ulong_type,
) {
    simple_object_set_little_32(buf, val as libc::c_uint);
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
static mut elf_big_32_functions: elf_type_functions = unsafe {
    {
        let mut init = elf_type_functions {
            fetch_Elf_Half: Some(
                simple_object_fetch_big_16
                    as unsafe extern "C" fn(*const libc::c_uchar) -> libc::c_ushort,
            ),
            fetch_Elf_Word: Some(
                simple_object_fetch_big_32
                    as unsafe extern "C" fn(*const libc::c_uchar) -> libc::c_uint,
            ),
            fetch_Elf_Addr: Some(
                simple_object_fetch_big_32_ulong
                    as unsafe extern "C" fn(*const libc::c_uchar) -> ulong_type,
            ),
            set_Elf_Half: Some(
                simple_object_set_big_16
                    as unsafe extern "C" fn(*mut libc::c_uchar, libc::c_ushort) -> (),
            ),
            set_Elf_Word: Some(
                simple_object_set_big_32
                    as unsafe extern "C" fn(*mut libc::c_uchar, libc::c_uint) -> (),
            ),
            set_Elf_Addr: Some(
                simple_object_set_big_32_ulong
                    as unsafe extern "C" fn(*mut libc::c_uchar, ulong_type) -> (),
            ),
        };
        init
    }
};
static mut elf_little_32_functions: elf_type_functions = unsafe {
    {
        let mut init = elf_type_functions {
            fetch_Elf_Half: Some(
                simple_object_fetch_little_16
                    as unsafe extern "C" fn(*const libc::c_uchar) -> libc::c_ushort,
            ),
            fetch_Elf_Word: Some(
                simple_object_fetch_little_32
                    as unsafe extern "C" fn(*const libc::c_uchar) -> libc::c_uint,
            ),
            fetch_Elf_Addr: Some(
                simple_object_fetch_little_32_ulong
                    as unsafe extern "C" fn(*const libc::c_uchar) -> ulong_type,
            ),
            set_Elf_Half: Some(
                simple_object_set_little_16
                    as unsafe extern "C" fn(*mut libc::c_uchar, libc::c_ushort) -> (),
            ),
            set_Elf_Word: Some(
                simple_object_set_little_32
                    as unsafe extern "C" fn(*mut libc::c_uchar, libc::c_uint) -> (),
            ),
            set_Elf_Addr: Some(
                simple_object_set_little_32_ulong
                    as unsafe extern "C" fn(*mut libc::c_uchar, ulong_type) -> (),
            ),
        };
        init
    }
};
static mut elf_big_64_functions: elf_type_functions = unsafe {
    {
        let mut init = elf_type_functions {
            fetch_Elf_Half: Some(
                simple_object_fetch_big_16
                    as unsafe extern "C" fn(*const libc::c_uchar) -> libc::c_ushort,
            ),
            fetch_Elf_Word: Some(
                simple_object_fetch_big_32
                    as unsafe extern "C" fn(*const libc::c_uchar) -> libc::c_uint,
            ),
            fetch_Elf_Addr: Some(
                simple_object_fetch_big_64
                    as unsafe extern "C" fn(*const libc::c_uchar) -> ulong_type,
            ),
            set_Elf_Half: Some(
                simple_object_set_big_16
                    as unsafe extern "C" fn(*mut libc::c_uchar, libc::c_ushort) -> (),
            ),
            set_Elf_Word: Some(
                simple_object_set_big_32
                    as unsafe extern "C" fn(*mut libc::c_uchar, libc::c_uint) -> (),
            ),
            set_Elf_Addr: Some(
                simple_object_set_big_64
                    as unsafe extern "C" fn(*mut libc::c_uchar, ulong_type) -> (),
            ),
        };
        init
    }
};
static mut elf_little_64_functions: elf_type_functions = unsafe {
    {
        let mut init = elf_type_functions {
            fetch_Elf_Half: Some(
                simple_object_fetch_little_16
                    as unsafe extern "C" fn(*const libc::c_uchar) -> libc::c_ushort,
            ),
            fetch_Elf_Word: Some(
                simple_object_fetch_little_32
                    as unsafe extern "C" fn(*const libc::c_uchar) -> libc::c_uint,
            ),
            fetch_Elf_Addr: Some(
                simple_object_fetch_little_64
                    as unsafe extern "C" fn(*const libc::c_uchar) -> ulong_type,
            ),
            set_Elf_Half: Some(
                simple_object_set_little_16
                    as unsafe extern "C" fn(*mut libc::c_uchar, libc::c_ushort) -> (),
            ),
            set_Elf_Word: Some(
                simple_object_set_little_32
                    as unsafe extern "C" fn(*mut libc::c_uchar, libc::c_uint) -> (),
            ),
            set_Elf_Addr: Some(
                simple_object_set_little_64
                    as unsafe extern "C" fn(*mut libc::c_uchar, ulong_type) -> (),
            ),
        };
        init
    }
};
unsafe extern "C" fn simple_object_elf_match(
    mut header: *mut libc::c_uchar,
    mut descriptor: libc::c_int,
    mut offset: off_t,
    mut segment_name: *const libc::c_char,
    mut errmsg: *mut *const libc::c_char,
    mut err: *mut libc::c_int,
) -> *mut libc::c_void {
    let mut ei_data: libc::c_uchar = 0;
    let mut ei_class: libc::c_uchar = 0;
    let mut type_functions: *const elf_type_functions = 0 as *const elf_type_functions;
    let mut ehdr: [libc::c_uchar; 64] = [0; 64];
    let mut eor: *mut simple_object_elf_read = 0 as *mut simple_object_elf_read;
    if *header.offset(0 as libc::c_int as isize) as libc::c_int != 0x7f as libc::c_int
        || *header.offset(1 as libc::c_int as isize) as libc::c_int != 'E' as i32
        || *header.offset(2 as libc::c_int as isize) as libc::c_int != 'L' as i32
        || *header.offset(3 as libc::c_int as isize) as libc::c_int != 'F' as i32
        || *header.offset(6 as libc::c_int as isize) as libc::c_int != 1 as libc::c_int
    {
        *errmsg = 0 as *const libc::c_char;
        *err = 0 as libc::c_int;
        return 0 as *mut libc::c_void;
    }
    ei_data = *header.offset(5 as libc::c_int as isize);
    if ei_data as libc::c_int != 1 as libc::c_int
        && ei_data as libc::c_int != 2 as libc::c_int
    {
        *errmsg = b"unknown ELF endianness\0" as *const u8 as *const libc::c_char;
        *err = 0 as libc::c_int;
        return 0 as *mut libc::c_void;
    }
    ei_class = *header.offset(4 as libc::c_int as isize);
    match ei_class as libc::c_int {
        1 => {
            type_functions = if ei_data as libc::c_int == 1 as libc::c_int {
                &elf_little_32_functions
            } else {
                &elf_big_32_functions
            };
        }
        2 => {
            type_functions = if ei_data as libc::c_int == 1 as libc::c_int {
                &elf_little_64_functions
            } else {
                &elf_big_64_functions
            };
        }
        _ => {
            *errmsg = b"unrecognized ELF size\0" as *const u8 as *const libc::c_char;
            *err = 0 as libc::c_int;
            return 0 as *mut libc::c_void;
        }
    }
    if simple_object_internal_read(
        descriptor,
        offset,
        ehdr.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_uchar; 64]>() as libc::c_ulong,
        errmsg,
        err,
    ) == 0
    {
        return 0 as *mut libc::c_void;
    }
    eor = xmalloc(::core::mem::size_of::<simple_object_elf_read>() as libc::c_ulong)
        as *mut simple_object_elf_read;
    (*eor).type_functions = type_functions;
    (*eor).ei_data = ei_data;
    (*eor).ei_class = ei_class;
    (*eor).ei_osabi = *header.offset(7 as libc::c_int as isize);
    (*eor)
        .machine = (if ei_class as libc::c_int == 1 as libc::c_int {
        ((*type_functions).fetch_Elf_Half)
            .expect(
                "non-null function pointer",
            )(ehdr.as_mut_ptr().offset(18 as libc::c_ulong as isize)) as libc::c_int
    } else {
        ((*type_functions).fetch_Elf_Half)
            .expect(
                "non-null function pointer",
            )(ehdr.as_mut_ptr().offset(18 as libc::c_ulong as isize)) as libc::c_int
    }) as libc::c_ushort;
    (*eor)
        .flags = if ei_class as libc::c_int == 1 as libc::c_int {
        ((*type_functions).fetch_Elf_Word)
            .expect(
                "non-null function pointer",
            )(ehdr.as_mut_ptr().offset(36 as libc::c_ulong as isize))
    } else {
        ((*type_functions).fetch_Elf_Word)
            .expect(
                "non-null function pointer",
            )(ehdr.as_mut_ptr().offset(48 as libc::c_ulong as isize))
    };
    (*eor)
        .shoff = if ei_class as libc::c_int == 1 as libc::c_int {
        ((*type_functions).fetch_Elf_Addr)
            .expect(
                "non-null function pointer",
            )(ehdr.as_mut_ptr().offset(32 as libc::c_ulong as isize))
    } else {
        ((*type_functions).fetch_Elf_Addr)
            .expect(
                "non-null function pointer",
            )(ehdr.as_mut_ptr().offset(40 as libc::c_ulong as isize))
    };
    (*eor)
        .shnum = (if ei_class as libc::c_int == 1 as libc::c_int {
        ((*type_functions).fetch_Elf_Half)
            .expect(
                "non-null function pointer",
            )(ehdr.as_mut_ptr().offset(48 as libc::c_ulong as isize)) as libc::c_int
    } else {
        ((*type_functions).fetch_Elf_Half)
            .expect(
                "non-null function pointer",
            )(ehdr.as_mut_ptr().offset(60 as libc::c_ulong as isize)) as libc::c_int
    }) as libc::c_uint;
    (*eor)
        .shstrndx = (if ei_class as libc::c_int == 1 as libc::c_int {
        ((*type_functions).fetch_Elf_Half)
            .expect(
                "non-null function pointer",
            )(ehdr.as_mut_ptr().offset(50 as libc::c_ulong as isize)) as libc::c_int
    } else {
        ((*type_functions).fetch_Elf_Half)
            .expect(
                "non-null function pointer",
            )(ehdr.as_mut_ptr().offset(62 as libc::c_ulong as isize)) as libc::c_int
    }) as libc::c_uint;
    if ((*eor).shnum == 0 as libc::c_int as libc::c_uint
        || (*eor).shstrndx == 0xffff as libc::c_int as libc::c_uint)
        && (*eor).shoff != 0 as libc::c_int as libc::c_ulong
    {
        let mut shdr: [libc::c_uchar; 64] = [0; 64];
        if simple_object_internal_read(
            descriptor,
            (offset as libc::c_ulong).wrapping_add((*eor).shoff) as off_t,
            shdr.as_mut_ptr(),
            if ei_class as libc::c_int == 1 as libc::c_int {
                ::core::mem::size_of::<Elf32_External_Shdr>() as libc::c_ulong
            } else {
                ::core::mem::size_of::<Elf64_External_Shdr>() as libc::c_ulong
            },
            errmsg,
            err,
        ) == 0
        {
            free(eor as *mut libc::c_void);
            return 0 as *mut libc::c_void;
        }
        if (*eor).shnum == 0 as libc::c_int as libc::c_uint {
            (*eor)
                .shnum = (if ei_class as libc::c_int == 1 as libc::c_int {
                ((*type_functions).fetch_Elf_Addr)
                    .expect(
                        "non-null function pointer",
                    )(shdr.as_mut_ptr().offset(20 as libc::c_ulong as isize))
            } else {
                ((*type_functions).fetch_Elf_Addr)
                    .expect(
                        "non-null function pointer",
                    )(shdr.as_mut_ptr().offset(32 as libc::c_ulong as isize))
            }) as libc::c_uint;
        }
        if (*eor).shstrndx == 0xffff as libc::c_int as libc::c_uint {
            (*eor)
                .shstrndx = if ei_class as libc::c_int == 1 as libc::c_int {
                ((*type_functions).fetch_Elf_Word)
                    .expect(
                        "non-null function pointer",
                    )(shdr.as_mut_ptr().offset(24 as libc::c_ulong as isize))
            } else {
                ((*type_functions).fetch_Elf_Word)
                    .expect(
                        "non-null function pointer",
                    )(shdr.as_mut_ptr().offset(40 as libc::c_ulong as isize))
            };
            if (*eor).shstrndx >= (*eor).shnum
                && (*eor).shstrndx
                    >= (0xff00 as libc::c_int + 0x100 as libc::c_int) as libc::c_uint
            {
                (*eor)
                    .shstrndx = ((*eor).shstrndx)
                    .wrapping_sub(0x100 as libc::c_int as libc::c_uint);
            }
        }
    }
    if (*eor).shstrndx >= (*eor).shnum {
        *errmsg = b"invalid ELF shstrndx >= shnum\0" as *const u8 as *const libc::c_char;
        *err = 0 as libc::c_int;
        free(eor as *mut libc::c_void);
        return 0 as *mut libc::c_void;
    }
    if (*eor).shstrndx == 0 as libc::c_int as libc::c_uint {
        *errmsg = b"invalid ELF shstrndx == 0\0" as *const u8 as *const libc::c_char;
        *err = 0 as libc::c_int;
        free(eor as *mut libc::c_void);
        return 0 as *mut libc::c_void;
    }
    return eor as *mut libc::c_void;
}
unsafe extern "C" fn simple_object_elf_find_sections(
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
    let mut eor: *mut simple_object_elf_read = (*sobj).data
        as *mut simple_object_elf_read;
    let mut type_functions: *const elf_type_functions = (*eor).type_functions;
    let mut ei_class: libc::c_uchar = (*eor).ei_class;
    let mut shdr_size: size_t = 0;
    let mut shnum: libc::c_uint = 0;
    let mut shdrs: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut errmsg: *const libc::c_char = 0 as *const libc::c_char;
    let mut shstrhdr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut name_size: size_t = 0;
    let mut shstroff: off_t = 0;
    let mut names: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut i: libc::c_uint = 0;
    shdr_size = if ei_class as libc::c_int == 1 as libc::c_int {
        ::core::mem::size_of::<Elf32_External_Shdr>() as libc::c_ulong
    } else {
        ::core::mem::size_of::<Elf64_External_Shdr>() as libc::c_ulong
    };
    shnum = (*eor).shnum;
    shdrs = xmalloc(
        (::core::mem::size_of::<libc::c_uchar>() as libc::c_ulong)
            .wrapping_mul(
                shdr_size
                    .wrapping_mul(
                        shnum.wrapping_sub(1 as libc::c_int as libc::c_uint)
                            as libc::c_ulong,
                    ),
            ),
    ) as *mut libc::c_uchar;
    if simple_object_internal_read(
        (*sobj).descriptor,
        ((*sobj).offset as libc::c_ulong)
            .wrapping_add((*eor).shoff)
            .wrapping_add(shdr_size) as off_t,
        shdrs,
        shdr_size
            .wrapping_mul(
                shnum.wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_ulong,
            ),
        &mut errmsg,
        err,
    ) == 0
    {
        free(shdrs as *mut libc::c_void);
        return errmsg;
    }
    shstrhdr = shdrs
        .offset(
            (((*eor).shstrndx).wrapping_sub(1 as libc::c_int as libc::c_uint)
                as libc::c_ulong)
                .wrapping_mul(shdr_size) as isize,
        );
    name_size = if ei_class as libc::c_int == 1 as libc::c_int {
        ((*type_functions).fetch_Elf_Addr)
            .expect(
                "non-null function pointer",
            )(shstrhdr.offset(20 as libc::c_ulong as isize))
    } else {
        ((*type_functions).fetch_Elf_Addr)
            .expect(
                "non-null function pointer",
            )(shstrhdr.offset(32 as libc::c_ulong as isize))
    };
    shstroff = (if ei_class as libc::c_int == 1 as libc::c_int {
        ((*type_functions).fetch_Elf_Addr)
            .expect(
                "non-null function pointer",
            )(shstrhdr.offset(16 as libc::c_ulong as isize))
    } else {
        ((*type_functions).fetch_Elf_Addr)
            .expect(
                "non-null function pointer",
            )(shstrhdr.offset(24 as libc::c_ulong as isize))
    }) as off_t;
    names = xmalloc(
        (::core::mem::size_of::<libc::c_uchar>() as libc::c_ulong)
            .wrapping_mul(name_size),
    ) as *mut libc::c_uchar;
    if simple_object_internal_read(
        (*sobj).descriptor,
        (*sobj).offset + shstroff,
        names,
        name_size,
        &mut errmsg,
        err,
    ) == 0
    {
        free(names as *mut libc::c_void);
        free(shdrs as *mut libc::c_void);
        return errmsg;
    }
    i = 1 as libc::c_int as libc::c_uint;
    while i < shnum {
        let mut shdr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        let mut sh_name: libc::c_uint = 0;
        let mut name: *const libc::c_char = 0 as *const libc::c_char;
        let mut offset: off_t = 0;
        let mut length: off_t = 0;
        shdr = shdrs
            .offset(
                (i.wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_ulong)
                    .wrapping_mul(shdr_size) as isize,
            );
        sh_name = if ei_class as libc::c_int == 1 as libc::c_int {
            ((*type_functions).fetch_Elf_Word)
                .expect(
                    "non-null function pointer",
                )(shdr.offset(0 as libc::c_ulong as isize))
        } else {
            ((*type_functions).fetch_Elf_Word)
                .expect(
                    "non-null function pointer",
                )(shdr.offset(0 as libc::c_ulong as isize))
        };
        if sh_name as libc::c_ulong >= name_size {
            *err = 0 as libc::c_int;
            free(names as *mut libc::c_void);
            free(shdrs as *mut libc::c_void);
            return b"ELF section name out of range\0" as *const u8
                as *const libc::c_char;
        }
        name = (names as *const libc::c_char).offset(sh_name as isize);
        offset = (if ei_class as libc::c_int == 1 as libc::c_int {
            ((*type_functions).fetch_Elf_Addr)
                .expect(
                    "non-null function pointer",
                )(shdr.offset(16 as libc::c_ulong as isize))
        } else {
            ((*type_functions).fetch_Elf_Addr)
                .expect(
                    "non-null function pointer",
                )(shdr.offset(24 as libc::c_ulong as isize))
        }) as off_t;
        length = (if ei_class as libc::c_int == 1 as libc::c_int {
            ((*type_functions).fetch_Elf_Addr)
                .expect(
                    "non-null function pointer",
                )(shdr.offset(20 as libc::c_ulong as isize))
        } else {
            ((*type_functions).fetch_Elf_Addr)
                .expect(
                    "non-null function pointer",
                )(shdr.offset(32 as libc::c_ulong as isize))
        }) as off_t;
        if (Some(pfn.expect("non-null function pointer")))
            .expect("non-null function pointer")(data, name, offset, length) == 0
        {
            break;
        }
        i = i.wrapping_add(1);
        i;
    }
    free(names as *mut libc::c_void);
    free(shdrs as *mut libc::c_void);
    return 0 as *const libc::c_char;
}
unsafe extern "C" fn simple_object_elf_fetch_attributes(
    mut sobj: *mut simple_object_read,
    mut errmsg: *mut *const libc::c_char,
    mut err: *mut libc::c_int,
) -> *mut libc::c_void {
    let mut eor: *mut simple_object_elf_read = (*sobj).data
        as *mut simple_object_elf_read;
    let mut ret: *mut simple_object_elf_attributes = 0
        as *mut simple_object_elf_attributes;
    ret = xmalloc(
        ::core::mem::size_of::<simple_object_elf_attributes>() as libc::c_ulong,
    ) as *mut simple_object_elf_attributes;
    (*ret).type_functions = (*eor).type_functions;
    (*ret).ei_data = (*eor).ei_data;
    (*ret).ei_class = (*eor).ei_class;
    (*ret).ei_osabi = (*eor).ei_osabi;
    (*ret).machine = (*eor).machine;
    (*ret).flags = (*eor).flags;
    return ret as *mut libc::c_void;
}
unsafe extern "C" fn simple_object_elf_release_read(mut data: *mut libc::c_void) {
    free(data);
}
unsafe extern "C" fn simple_object_elf_attributes_merge(
    mut todata: *mut libc::c_void,
    mut fromdata: *mut libc::c_void,
    mut err: *mut libc::c_int,
) -> *const libc::c_char {
    let mut to: *mut simple_object_elf_attributes = todata
        as *mut simple_object_elf_attributes;
    let mut from: *mut simple_object_elf_attributes = fromdata
        as *mut simple_object_elf_attributes;
    if (*to).ei_data as libc::c_int != (*from).ei_data as libc::c_int
        || (*to).ei_class as libc::c_int != (*from).ei_class as libc::c_int
    {
        *err = 0 as libc::c_int;
        return b"ELF object format mismatch\0" as *const u8 as *const libc::c_char;
    }
    if (*to).machine as libc::c_int != (*from).machine as libc::c_int {
        let mut ok: libc::c_int = 0;
        ok = 0 as libc::c_int;
        match (*to).machine as libc::c_int {
            2 => {
                if (*from).machine as libc::c_int == 18 as libc::c_int {
                    (*to).machine = (*from).machine;
                    ok = 1 as libc::c_int;
                }
            }
            18 => {
                if (*from).machine as libc::c_int == 2 as libc::c_int {
                    ok = 1 as libc::c_int;
                }
            }
            _ => {}
        }
        if ok == 0 {
            *err = 0 as libc::c_int;
            return b"ELF machine number mismatch\0" as *const u8 as *const libc::c_char;
        }
    }
    return 0 as *const libc::c_char;
}
unsafe extern "C" fn simple_object_elf_release_attributes(mut data: *mut libc::c_void) {
    free(data);
}
unsafe extern "C" fn simple_object_elf_start_write(
    mut attributes_data: *mut libc::c_void,
    mut errmsg: *mut *const libc::c_char,
    mut err: *mut libc::c_int,
) -> *mut libc::c_void {
    let mut attrs: *mut simple_object_elf_attributes = attributes_data
        as *mut simple_object_elf_attributes;
    let mut ret: *mut simple_object_elf_write = 0 as *mut simple_object_elf_write;
    ret = xmalloc(::core::mem::size_of::<simple_object_elf_write>() as libc::c_ulong)
        as *mut simple_object_elf_write;
    (*ret).attrs = *attrs;
    (*ret).shdrs = 0 as *mut libc::c_uchar;
    return ret as *mut libc::c_void;
}
unsafe extern "C" fn simple_object_elf_write_ehdr(
    mut sobj: *mut simple_object_write,
    mut descriptor: libc::c_int,
    mut errmsg: *mut *const libc::c_char,
    mut err: *mut libc::c_int,
) -> libc::c_int {
    let mut attrs: *mut simple_object_elf_attributes = (*sobj).data
        as *mut simple_object_elf_attributes;
    let mut fns: *const elf_type_functions = 0 as *const elf_type_functions;
    let mut cl: libc::c_uchar = 0;
    let mut ehdr_size: size_t = 0;
    let mut buf: [libc::c_uchar; 64] = [0; 64];
    let mut section: *mut simple_object_write_section = 0
        as *mut simple_object_write_section;
    let mut shnum: libc::c_uint = 0;
    let mut shstrndx: libc::c_uint = 0;
    fns = (*attrs).type_functions;
    cl = (*attrs).ei_class;
    shnum = 0 as libc::c_int as libc::c_uint;
    section = (*sobj).sections;
    while !section.is_null() {
        shnum = shnum.wrapping_add(1);
        shnum;
        section = (*section).next;
    }
    if shnum > 0 as libc::c_int as libc::c_uint {
        shnum = shnum.wrapping_add(2 as libc::c_int as libc::c_uint);
    }
    ehdr_size = if cl as libc::c_int == 1 as libc::c_int {
        ::core::mem::size_of::<Elf32_External_Ehdr>() as libc::c_ulong
    } else {
        ::core::mem::size_of::<Elf64_External_Ehdr>() as libc::c_ulong
    };
    memset(
        buf.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<Elf64_External_Ehdr>() as libc::c_ulong,
    );
    buf[0 as libc::c_int as usize] = 0x7f as libc::c_int as libc::c_uchar;
    buf[1 as libc::c_int as usize] = 'E' as i32 as libc::c_uchar;
    buf[2 as libc::c_int as usize] = 'L' as i32 as libc::c_uchar;
    buf[3 as libc::c_int as usize] = 'F' as i32 as libc::c_uchar;
    buf[4 as libc::c_int as usize] = cl;
    buf[5 as libc::c_int as usize] = (*attrs).ei_data;
    buf[6 as libc::c_int as usize] = 1 as libc::c_int as libc::c_uchar;
    buf[7 as libc::c_int as usize] = (*attrs).ei_osabi;
    if cl as libc::c_int == 1 as libc::c_int {
        ((*fns).set_Elf_Half)
            .expect(
                "non-null function pointer",
            )(
            buf.as_mut_ptr().offset(16 as libc::c_ulong as isize),
            1 as libc::c_int as libc::c_ushort,
        );
    } else {
        ((*fns).set_Elf_Half)
            .expect(
                "non-null function pointer",
            )(
            buf.as_mut_ptr().offset(16 as libc::c_ulong as isize),
            1 as libc::c_int as libc::c_ushort,
        );
    };
    if cl as libc::c_int == 1 as libc::c_int {
        ((*fns).set_Elf_Half)
            .expect(
                "non-null function pointer",
            )(buf.as_mut_ptr().offset(18 as libc::c_ulong as isize), (*attrs).machine);
    } else {
        ((*fns).set_Elf_Half)
            .expect(
                "non-null function pointer",
            )(buf.as_mut_ptr().offset(18 as libc::c_ulong as isize), (*attrs).machine);
    };
    if cl as libc::c_int == 1 as libc::c_int {
        ((*fns).set_Elf_Word)
            .expect(
                "non-null function pointer",
            )(
            buf.as_mut_ptr().offset(20 as libc::c_ulong as isize),
            1 as libc::c_int as libc::c_uint,
        );
    } else {
        ((*fns).set_Elf_Word)
            .expect(
                "non-null function pointer",
            )(
            buf.as_mut_ptr().offset(20 as libc::c_ulong as isize),
            1 as libc::c_int as libc::c_uint,
        );
    };
    if cl as libc::c_int == 1 as libc::c_int {
        ((*fns).set_Elf_Addr)
            .expect(
                "non-null function pointer",
            )(buf.as_mut_ptr().offset(32 as libc::c_ulong as isize), ehdr_size);
    } else {
        ((*fns).set_Elf_Addr)
            .expect(
                "non-null function pointer",
            )(buf.as_mut_ptr().offset(40 as libc::c_ulong as isize), ehdr_size);
    };
    if cl as libc::c_int == 1 as libc::c_int {
        ((*fns).set_Elf_Word)
            .expect(
                "non-null function pointer",
            )(buf.as_mut_ptr().offset(36 as libc::c_ulong as isize), (*attrs).flags);
    } else {
        ((*fns).set_Elf_Word)
            .expect(
                "non-null function pointer",
            )(buf.as_mut_ptr().offset(48 as libc::c_ulong as isize), (*attrs).flags);
    };
    if cl as libc::c_int == 1 as libc::c_int {
        ((*fns).set_Elf_Half)
            .expect(
                "non-null function pointer",
            )(
            buf.as_mut_ptr().offset(40 as libc::c_ulong as isize),
            ehdr_size as libc::c_ushort,
        );
    } else {
        ((*fns).set_Elf_Half)
            .expect(
                "non-null function pointer",
            )(
            buf.as_mut_ptr().offset(52 as libc::c_ulong as isize),
            ehdr_size as libc::c_ushort,
        );
    };
    if cl as libc::c_int == 1 as libc::c_int {
        ((*fns).set_Elf_Half)
            .expect(
                "non-null function pointer",
            )(
            buf.as_mut_ptr().offset(42 as libc::c_ulong as isize),
            (if cl as libc::c_int == 1 as libc::c_int {
                ::core::mem::size_of::<Elf32_External_Phdr>() as libc::c_ulong
            } else {
                ::core::mem::size_of::<Elf64_External_Phdr>() as libc::c_ulong
            }) as libc::c_ushort,
        );
    } else {
        ((*fns).set_Elf_Half)
            .expect(
                "non-null function pointer",
            )(
            buf.as_mut_ptr().offset(54 as libc::c_ulong as isize),
            (if cl as libc::c_int == 1 as libc::c_int {
                ::core::mem::size_of::<Elf32_External_Phdr>() as libc::c_ulong
            } else {
                ::core::mem::size_of::<Elf64_External_Phdr>() as libc::c_ulong
            }) as libc::c_ushort,
        );
    };
    if cl as libc::c_int == 1 as libc::c_int {
        ((*fns).set_Elf_Half)
            .expect(
                "non-null function pointer",
            )(
            buf.as_mut_ptr().offset(46 as libc::c_ulong as isize),
            (if cl as libc::c_int == 1 as libc::c_int {
                ::core::mem::size_of::<Elf32_External_Shdr>() as libc::c_ulong
            } else {
                ::core::mem::size_of::<Elf64_External_Shdr>() as libc::c_ulong
            }) as libc::c_ushort,
        );
    } else {
        ((*fns).set_Elf_Half)
            .expect(
                "non-null function pointer",
            )(
            buf.as_mut_ptr().offset(58 as libc::c_ulong as isize),
            (if cl as libc::c_int == 1 as libc::c_int {
                ::core::mem::size_of::<Elf32_External_Shdr>() as libc::c_ulong
            } else {
                ::core::mem::size_of::<Elf64_External_Shdr>() as libc::c_ulong
            }) as libc::c_ushort,
        );
    };
    if cl as libc::c_int == 1 as libc::c_int {
        ((*fns).set_Elf_Half)
            .expect(
                "non-null function pointer",
            )(
            buf.as_mut_ptr().offset(48 as libc::c_ulong as isize),
            (if shnum >= 0xff00 as libc::c_int as libc::c_uint {
                0 as libc::c_int as libc::c_uint
            } else {
                shnum
            }) as libc::c_ushort,
        );
    } else {
        ((*fns).set_Elf_Half)
            .expect(
                "non-null function pointer",
            )(
            buf.as_mut_ptr().offset(60 as libc::c_ulong as isize),
            (if shnum >= 0xff00 as libc::c_int as libc::c_uint {
                0 as libc::c_int as libc::c_uint
            } else {
                shnum
            }) as libc::c_ushort,
        );
    };
    if shnum == 0 as libc::c_int as libc::c_uint {
        shstrndx = 0 as libc::c_int as libc::c_uint;
    } else {
        shstrndx = shnum.wrapping_sub(1 as libc::c_int as libc::c_uint);
        if shstrndx >= 0xff00 as libc::c_int as libc::c_uint {
            shstrndx = 0xffff as libc::c_int as libc::c_uint;
        }
    }
    if cl as libc::c_int == 1 as libc::c_int {
        ((*fns).set_Elf_Half)
            .expect(
                "non-null function pointer",
            )(
            buf.as_mut_ptr().offset(50 as libc::c_ulong as isize),
            shstrndx as libc::c_ushort,
        );
    } else {
        ((*fns).set_Elf_Half)
            .expect(
                "non-null function pointer",
            )(
            buf.as_mut_ptr().offset(62 as libc::c_ulong as isize),
            shstrndx as libc::c_ushort,
        );
    };
    return simple_object_internal_write(
        descriptor,
        0 as libc::c_int as off_t,
        buf.as_mut_ptr(),
        ehdr_size,
        errmsg,
        err,
    );
}
unsafe extern "C" fn simple_object_elf_write_shdr(
    mut sobj: *mut simple_object_write,
    mut descriptor: libc::c_int,
    mut offset: off_t,
    mut sh_name: libc::c_uint,
    mut sh_type: libc::c_uint,
    mut sh_flags: libc::c_uint,
    mut sh_addr: off_t,
    mut sh_offset: libc::c_uint,
    mut sh_size: libc::c_uint,
    mut sh_link: libc::c_uint,
    mut sh_info: libc::c_uint,
    mut sh_addralign: size_t,
    mut sh_entsize: size_t,
    mut errmsg: *mut *const libc::c_char,
    mut err: *mut libc::c_int,
) -> libc::c_int {
    let mut attrs: *mut simple_object_elf_attributes = (*sobj).data
        as *mut simple_object_elf_attributes;
    let mut fns: *const elf_type_functions = 0 as *const elf_type_functions;
    let mut cl: libc::c_uchar = 0;
    let mut shdr_size: size_t = 0;
    let mut buf: [libc::c_uchar; 64] = [0; 64];
    fns = (*attrs).type_functions;
    cl = (*attrs).ei_class;
    shdr_size = if cl as libc::c_int == 1 as libc::c_int {
        ::core::mem::size_of::<Elf32_External_Shdr>() as libc::c_ulong
    } else {
        ::core::mem::size_of::<Elf64_External_Shdr>() as libc::c_ulong
    };
    memset(
        buf.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<Elf64_External_Shdr>() as libc::c_ulong,
    );
    if cl as libc::c_int == 1 as libc::c_int {
        ((*fns).set_Elf_Word)
            .expect(
                "non-null function pointer",
            )(buf.as_mut_ptr().offset(0 as libc::c_ulong as isize), sh_name);
    } else {
        ((*fns).set_Elf_Word)
            .expect(
                "non-null function pointer",
            )(buf.as_mut_ptr().offset(0 as libc::c_ulong as isize), sh_name);
    };
    if cl as libc::c_int == 1 as libc::c_int {
        ((*fns).set_Elf_Word)
            .expect(
                "non-null function pointer",
            )(buf.as_mut_ptr().offset(4 as libc::c_ulong as isize), sh_type);
    } else {
        ((*fns).set_Elf_Word)
            .expect(
                "non-null function pointer",
            )(buf.as_mut_ptr().offset(4 as libc::c_ulong as isize), sh_type);
    };
    if cl as libc::c_int == 1 as libc::c_int {
        ((*fns).set_Elf_Addr)
            .expect(
                "non-null function pointer",
            )(
            buf.as_mut_ptr().offset(8 as libc::c_ulong as isize),
            sh_flags as ulong_type,
        );
    } else {
        ((*fns).set_Elf_Addr)
            .expect(
                "non-null function pointer",
            )(
            buf.as_mut_ptr().offset(8 as libc::c_ulong as isize),
            sh_flags as ulong_type,
        );
    };
    if cl as libc::c_int == 1 as libc::c_int {
        ((*fns).set_Elf_Addr)
            .expect(
                "non-null function pointer",
            )(
            buf.as_mut_ptr().offset(12 as libc::c_ulong as isize),
            sh_addr as ulong_type,
        );
    } else {
        ((*fns).set_Elf_Addr)
            .expect(
                "non-null function pointer",
            )(
            buf.as_mut_ptr().offset(16 as libc::c_ulong as isize),
            sh_addr as ulong_type,
        );
    };
    if cl as libc::c_int == 1 as libc::c_int {
        ((*fns).set_Elf_Addr)
            .expect(
                "non-null function pointer",
            )(
            buf.as_mut_ptr().offset(16 as libc::c_ulong as isize),
            sh_offset as ulong_type,
        );
    } else {
        ((*fns).set_Elf_Addr)
            .expect(
                "non-null function pointer",
            )(
            buf.as_mut_ptr().offset(24 as libc::c_ulong as isize),
            sh_offset as ulong_type,
        );
    };
    if cl as libc::c_int == 1 as libc::c_int {
        ((*fns).set_Elf_Addr)
            .expect(
                "non-null function pointer",
            )(
            buf.as_mut_ptr().offset(20 as libc::c_ulong as isize),
            sh_size as ulong_type,
        );
    } else {
        ((*fns).set_Elf_Addr)
            .expect(
                "non-null function pointer",
            )(
            buf.as_mut_ptr().offset(32 as libc::c_ulong as isize),
            sh_size as ulong_type,
        );
    };
    if cl as libc::c_int == 1 as libc::c_int {
        ((*fns).set_Elf_Word)
            .expect(
                "non-null function pointer",
            )(buf.as_mut_ptr().offset(24 as libc::c_ulong as isize), sh_link);
    } else {
        ((*fns).set_Elf_Word)
            .expect(
                "non-null function pointer",
            )(buf.as_mut_ptr().offset(40 as libc::c_ulong as isize), sh_link);
    };
    if cl as libc::c_int == 1 as libc::c_int {
        ((*fns).set_Elf_Word)
            .expect(
                "non-null function pointer",
            )(buf.as_mut_ptr().offset(28 as libc::c_ulong as isize), sh_info);
    } else {
        ((*fns).set_Elf_Word)
            .expect(
                "non-null function pointer",
            )(buf.as_mut_ptr().offset(44 as libc::c_ulong as isize), sh_info);
    };
    if cl as libc::c_int == 1 as libc::c_int {
        ((*fns).set_Elf_Addr)
            .expect(
                "non-null function pointer",
            )(buf.as_mut_ptr().offset(32 as libc::c_ulong as isize), sh_addralign);
    } else {
        ((*fns).set_Elf_Addr)
            .expect(
                "non-null function pointer",
            )(buf.as_mut_ptr().offset(48 as libc::c_ulong as isize), sh_addralign);
    };
    if cl as libc::c_int == 1 as libc::c_int {
        ((*fns).set_Elf_Addr)
            .expect(
                "non-null function pointer",
            )(buf.as_mut_ptr().offset(36 as libc::c_ulong as isize), sh_entsize);
    } else {
        ((*fns).set_Elf_Addr)
            .expect(
                "non-null function pointer",
            )(buf.as_mut_ptr().offset(56 as libc::c_ulong as isize), sh_entsize);
    };
    return simple_object_internal_write(
        descriptor,
        offset,
        buf.as_mut_ptr(),
        shdr_size,
        errmsg,
        err,
    );
}
unsafe extern "C" fn simple_object_elf_write_to_file(
    mut sobj: *mut simple_object_write,
    mut descriptor: libc::c_int,
    mut err: *mut libc::c_int,
) -> *const libc::c_char {
    let mut eow: *mut simple_object_elf_write = (*sobj).data
        as *mut simple_object_elf_write;
    let mut attrs: *mut simple_object_elf_attributes = &mut (*eow).attrs;
    let mut cl: libc::c_uchar = 0;
    let mut ehdr_size: size_t = 0;
    let mut shdr_size: size_t = 0;
    let mut errmsg: *const libc::c_char = 0 as *const libc::c_char;
    let mut section: *mut simple_object_write_section = 0
        as *mut simple_object_write_section;
    let mut shnum: libc::c_uint = 0;
    let mut shdr_offset: size_t = 0;
    let mut sh_offset: size_t = 0;
    let mut first_sh_size: libc::c_uint = 0;
    let mut first_sh_link: libc::c_uint = 0;
    let mut sh_name: size_t = 0;
    let mut zero: libc::c_uchar = 0;
    let mut secnum: libc::c_uint = 0;
    if simple_object_elf_write_ehdr(sobj, descriptor, &mut errmsg, err) == 0 {
        return errmsg;
    }
    cl = (*attrs).ei_class;
    if cl as libc::c_int == 1 as libc::c_int {
        ehdr_size = ::core::mem::size_of::<Elf32_External_Ehdr>() as libc::c_ulong;
        shdr_size = ::core::mem::size_of::<Elf32_External_Shdr>() as libc::c_ulong;
    } else {
        ehdr_size = ::core::mem::size_of::<Elf64_External_Ehdr>() as libc::c_ulong;
        shdr_size = ::core::mem::size_of::<Elf64_External_Shdr>() as libc::c_ulong;
    }
    shnum = 0 as libc::c_int as libc::c_uint;
    section = (*sobj).sections;
    while !section.is_null() {
        shnum = shnum.wrapping_add(1);
        shnum;
        section = (*section).next;
    }
    if shnum == 0 as libc::c_int as libc::c_uint {
        return 0 as *const libc::c_char;
    }
    shnum = shnum.wrapping_add(2 as libc::c_int as libc::c_uint);
    shdr_offset = ehdr_size;
    sh_offset = shdr_offset
        .wrapping_add((shnum as libc::c_ulong).wrapping_mul(shdr_size));
    if shnum < 0xff00 as libc::c_int as libc::c_uint {
        first_sh_size = 0 as libc::c_int as libc::c_uint;
    } else {
        first_sh_size = shnum;
    }
    if shnum.wrapping_sub(1 as libc::c_int as libc::c_uint)
        < 0xff00 as libc::c_int as libc::c_uint
    {
        first_sh_link = 0 as libc::c_int as libc::c_uint;
    } else {
        first_sh_link = shnum.wrapping_sub(1 as libc::c_int as libc::c_uint);
    }
    if simple_object_elf_write_shdr(
        sobj,
        descriptor,
        shdr_offset as off_t,
        0 as libc::c_int as libc::c_uint,
        0 as libc::c_int as libc::c_uint,
        0 as libc::c_int as libc::c_uint,
        0 as libc::c_int as off_t,
        0 as libc::c_int as libc::c_uint,
        first_sh_size,
        first_sh_link,
        0 as libc::c_int as libc::c_uint,
        0 as libc::c_int as size_t,
        0 as libc::c_int as size_t,
        &mut errmsg,
        err,
    ) == 0
    {
        return errmsg;
    }
    shdr_offset = (shdr_offset as libc::c_ulong).wrapping_add(shdr_size) as size_t
        as size_t;
    sh_name = 1 as libc::c_int as size_t;
    secnum = 0 as libc::c_int as libc::c_uint;
    section = (*sobj).sections;
    while !section.is_null() {
        let mut mask: size_t = 0;
        let mut new_sh_offset: size_t = 0;
        let mut sh_size: size_t = 0;
        let mut buffer: *mut simple_object_write_section_buffer = 0
            as *mut simple_object_write_section_buffer;
        let mut sh_type: libc::c_uint = 1 as libc::c_int as libc::c_uint;
        let mut sh_flags: libc::c_uint = 0 as libc::c_int as libc::c_uint;
        let mut sh_addr: off_t = 0 as libc::c_int as off_t;
        let mut sh_link: libc::c_uint = 0 as libc::c_int as libc::c_uint;
        let mut sh_info: libc::c_uint = 0 as libc::c_int as libc::c_uint;
        let mut sh_addralign: size_t = ((1 as libc::c_uint) << (*section).align)
            as size_t;
        let mut sh_entsize: size_t = 0 as libc::c_int as size_t;
        if !((*eow).shdrs).is_null() {
            sh_type = if (*attrs).ei_class as libc::c_int == 1 as libc::c_int {
                ((*(*attrs).type_functions).fetch_Elf_Word)
                    .expect(
                        "non-null function pointer",
                    )(
                    ((*eow).shdrs)
                        .offset(
                            (secnum as libc::c_ulong).wrapping_mul(shdr_size) as isize,
                        )
                        .offset(4 as libc::c_ulong as isize),
                )
            } else {
                ((*(*attrs).type_functions).fetch_Elf_Word)
                    .expect(
                        "non-null function pointer",
                    )(
                    ((*eow).shdrs)
                        .offset(
                            (secnum as libc::c_ulong).wrapping_mul(shdr_size) as isize,
                        )
                        .offset(4 as libc::c_ulong as isize),
                )
            };
            sh_flags = (if (*attrs).ei_class as libc::c_int == 1 as libc::c_int {
                ((*(*attrs).type_functions).fetch_Elf_Addr)
                    .expect(
                        "non-null function pointer",
                    )(
                    ((*eow).shdrs)
                        .offset(
                            (secnum as libc::c_ulong).wrapping_mul(shdr_size) as isize,
                        )
                        .offset(8 as libc::c_ulong as isize),
                )
            } else {
                ((*(*attrs).type_functions).fetch_Elf_Addr)
                    .expect(
                        "non-null function pointer",
                    )(
                    ((*eow).shdrs)
                        .offset(
                            (secnum as libc::c_ulong).wrapping_mul(shdr_size) as isize,
                        )
                        .offset(8 as libc::c_ulong as isize),
                )
            }) as libc::c_uint;
            sh_addr = (if (*attrs).ei_class as libc::c_int == 1 as libc::c_int {
                ((*(*attrs).type_functions).fetch_Elf_Addr)
                    .expect(
                        "non-null function pointer",
                    )(
                    ((*eow).shdrs)
                        .offset(
                            (secnum as libc::c_ulong).wrapping_mul(shdr_size) as isize,
                        )
                        .offset(12 as libc::c_ulong as isize),
                )
            } else {
                ((*(*attrs).type_functions).fetch_Elf_Addr)
                    .expect(
                        "non-null function pointer",
                    )(
                    ((*eow).shdrs)
                        .offset(
                            (secnum as libc::c_ulong).wrapping_mul(shdr_size) as isize,
                        )
                        .offset(16 as libc::c_ulong as isize),
                )
            }) as off_t;
            sh_link = if (*attrs).ei_class as libc::c_int == 1 as libc::c_int {
                ((*(*attrs).type_functions).fetch_Elf_Word)
                    .expect(
                        "non-null function pointer",
                    )(
                    ((*eow).shdrs)
                        .offset(
                            (secnum as libc::c_ulong).wrapping_mul(shdr_size) as isize,
                        )
                        .offset(24 as libc::c_ulong as isize),
                )
            } else {
                ((*(*attrs).type_functions).fetch_Elf_Word)
                    .expect(
                        "non-null function pointer",
                    )(
                    ((*eow).shdrs)
                        .offset(
                            (secnum as libc::c_ulong).wrapping_mul(shdr_size) as isize,
                        )
                        .offset(40 as libc::c_ulong as isize),
                )
            };
            sh_info = if (*attrs).ei_class as libc::c_int == 1 as libc::c_int {
                ((*(*attrs).type_functions).fetch_Elf_Word)
                    .expect(
                        "non-null function pointer",
                    )(
                    ((*eow).shdrs)
                        .offset(
                            (secnum as libc::c_ulong).wrapping_mul(shdr_size) as isize,
                        )
                        .offset(28 as libc::c_ulong as isize),
                )
            } else {
                ((*(*attrs).type_functions).fetch_Elf_Word)
                    .expect(
                        "non-null function pointer",
                    )(
                    ((*eow).shdrs)
                        .offset(
                            (secnum as libc::c_ulong).wrapping_mul(shdr_size) as isize,
                        )
                        .offset(44 as libc::c_ulong as isize),
                )
            };
            sh_addralign = if (*attrs).ei_class as libc::c_int == 1 as libc::c_int {
                ((*(*attrs).type_functions).fetch_Elf_Addr)
                    .expect(
                        "non-null function pointer",
                    )(
                    ((*eow).shdrs)
                        .offset(
                            (secnum as libc::c_ulong).wrapping_mul(shdr_size) as isize,
                        )
                        .offset(32 as libc::c_ulong as isize),
                )
            } else {
                ((*(*attrs).type_functions).fetch_Elf_Addr)
                    .expect(
                        "non-null function pointer",
                    )(
                    ((*eow).shdrs)
                        .offset(
                            (secnum as libc::c_ulong).wrapping_mul(shdr_size) as isize,
                        )
                        .offset(48 as libc::c_ulong as isize),
                )
            };
            sh_entsize = if (*attrs).ei_class as libc::c_int == 1 as libc::c_int {
                ((*(*attrs).type_functions).fetch_Elf_Addr)
                    .expect(
                        "non-null function pointer",
                    )(
                    ((*eow).shdrs)
                        .offset(
                            (secnum as libc::c_ulong).wrapping_mul(shdr_size) as isize,
                        )
                        .offset(36 as libc::c_ulong as isize),
                )
            } else {
                ((*(*attrs).type_functions).fetch_Elf_Addr)
                    .expect(
                        "non-null function pointer",
                    )(
                    ((*eow).shdrs)
                        .offset(
                            (secnum as libc::c_ulong).wrapping_mul(shdr_size) as isize,
                        )
                        .offset(56 as libc::c_ulong as isize),
                )
            };
            secnum = secnum.wrapping_add(1);
            secnum;
        }
        mask = sh_addralign.wrapping_sub(1 as libc::c_int as libc::c_ulong);
        new_sh_offset = sh_offset.wrapping_add(mask);
        new_sh_offset &= !mask;
        while new_sh_offset > sh_offset {
            let mut zeroes: [libc::c_uchar; 16] = [0; 16];
            let mut write: size_t = 0;
            memset(
                zeroes.as_mut_ptr() as *mut libc::c_void,
                0 as libc::c_int,
                ::core::mem::size_of::<[libc::c_uchar; 16]>() as libc::c_ulong,
            );
            write = new_sh_offset.wrapping_sub(sh_offset);
            if write > ::core::mem::size_of::<[libc::c_uchar; 16]>() as libc::c_ulong {
                write = ::core::mem::size_of::<[libc::c_uchar; 16]>() as libc::c_ulong;
            }
            if simple_object_internal_write(
                descriptor,
                sh_offset as off_t,
                zeroes.as_mut_ptr(),
                write,
                &mut errmsg,
                err,
            ) == 0
            {
                return errmsg;
            }
            sh_offset = (sh_offset as libc::c_ulong).wrapping_add(write) as size_t
                as size_t;
        }
        sh_size = 0 as libc::c_int as size_t;
        buffer = (*section).buffers;
        while !buffer.is_null() {
            if simple_object_internal_write(
                descriptor,
                sh_offset.wrapping_add(sh_size) as off_t,
                (*buffer).buffer as *const libc::c_uchar,
                (*buffer).size,
                &mut errmsg,
                err,
            ) == 0
            {
                return errmsg;
            }
            sh_size = (sh_size as libc::c_ulong).wrapping_add((*buffer).size) as size_t
                as size_t;
            buffer = (*buffer).next;
        }
        if simple_object_elf_write_shdr(
            sobj,
            descriptor,
            shdr_offset as off_t,
            sh_name as libc::c_uint,
            sh_type,
            sh_flags,
            sh_addr,
            sh_offset as libc::c_uint,
            sh_size as libc::c_uint,
            sh_link,
            sh_info,
            sh_addralign,
            sh_entsize,
            &mut errmsg,
            err,
        ) == 0
        {
            return errmsg;
        }
        shdr_offset = (shdr_offset as libc::c_ulong).wrapping_add(shdr_size) as size_t
            as size_t;
        sh_name = (sh_name as libc::c_ulong)
            .wrapping_add(
                (strlen((*section).name)).wrapping_add(1 as libc::c_int as libc::c_ulong),
            ) as size_t as size_t;
        sh_offset = (sh_offset as libc::c_ulong).wrapping_add(sh_size) as size_t
            as size_t;
        section = (*section).next;
    }
    if simple_object_elf_write_shdr(
        sobj,
        descriptor,
        shdr_offset as off_t,
        sh_name as libc::c_uint,
        3 as libc::c_int as libc::c_uint,
        0 as libc::c_int as libc::c_uint,
        0 as libc::c_int as off_t,
        sh_offset as libc::c_uint,
        sh_name
            .wrapping_add(strlen(b".shstrtab\0" as *const u8 as *const libc::c_char))
            .wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_uint,
        0 as libc::c_int as libc::c_uint,
        0 as libc::c_int as libc::c_uint,
        1 as libc::c_int as size_t,
        0 as libc::c_int as size_t,
        &mut errmsg,
        err,
    ) == 0
    {
        return errmsg;
    }
    zero = 0 as libc::c_int as libc::c_uchar;
    if simple_object_internal_write(
        descriptor,
        sh_offset as off_t,
        &mut zero,
        1 as libc::c_int as size_t,
        &mut errmsg,
        err,
    ) == 0
    {
        return errmsg;
    }
    sh_offset = sh_offset.wrapping_add(1);
    sh_offset;
    section = (*sobj).sections;
    while !section.is_null() {
        let mut len: size_t = 0;
        len = (strlen((*section).name)).wrapping_add(1 as libc::c_int as libc::c_ulong);
        if simple_object_internal_write(
            descriptor,
            sh_offset as off_t,
            (*section).name as *const libc::c_uchar,
            len,
            &mut errmsg,
            err,
        ) == 0
        {
            return errmsg;
        }
        sh_offset = (sh_offset as libc::c_ulong).wrapping_add(len) as size_t as size_t;
        section = (*section).next;
    }
    if simple_object_internal_write(
        descriptor,
        sh_offset as off_t,
        b".shstrtab\0" as *const u8 as *const libc::c_char as *const libc::c_uchar,
        (strlen(b".shstrtab\0" as *const u8 as *const libc::c_char))
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
        &mut errmsg,
        err,
    ) == 0
    {
        return errmsg;
    }
    return 0 as *const libc::c_char;
}
unsafe extern "C" fn simple_object_elf_release_write(mut data: *mut libc::c_void) {
    let mut eow: *mut simple_object_elf_write = data as *mut simple_object_elf_write;
    if !((*eow).shdrs).is_null() {
        free((*eow).shdrs as *mut libc::c_void);
    }
    free(data);
}
unsafe extern "C" fn simple_object_elf_copy_lto_debug_sections(
    mut sobj: *mut simple_object_read,
    mut dobj: *mut simple_object_write,
    mut pfn: Option::<unsafe extern "C" fn(*const libc::c_char) -> *mut libc::c_char>,
    mut err: *mut libc::c_int,
) -> *const libc::c_char {
    let mut eor: *mut simple_object_elf_read = (*sobj).data
        as *mut simple_object_elf_read;
    let mut type_functions: *const elf_type_functions = (*eor).type_functions;
    let mut eow: *mut simple_object_elf_write = (*dobj).data
        as *mut simple_object_elf_write;
    let mut ei_class: libc::c_uchar = (*eor).ei_class;
    let mut shdr_size: size_t = 0;
    let mut shnum: libc::c_uint = 0;
    let mut shdrs: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut errmsg: *const libc::c_char = 0 as *const libc::c_char;
    let mut shstrhdr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut name_size: size_t = 0;
    let mut shstroff: off_t = 0;
    let mut names: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut i: libc::c_uint = 0;
    let mut changed: libc::c_int = 0;
    let mut pfnret: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut pfnname: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
    let mut new_i: libc::c_uint = 0;
    let mut sh_map: *mut libc::c_uint = 0 as *mut libc::c_uint;
    let mut first_shndx: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut symtab_indices_shndx: *mut libc::c_uint = 0 as *mut libc::c_uint;
    shdr_size = if ei_class as libc::c_int == 1 as libc::c_int {
        ::core::mem::size_of::<Elf32_External_Shdr>() as libc::c_ulong
    } else {
        ::core::mem::size_of::<Elf64_External_Shdr>() as libc::c_ulong
    };
    shnum = (*eor).shnum;
    shdrs = xmalloc(
        (::core::mem::size_of::<libc::c_uchar>() as libc::c_ulong)
            .wrapping_mul(
                shdr_size
                    .wrapping_mul(
                        shnum.wrapping_sub(1 as libc::c_int as libc::c_uint)
                            as libc::c_ulong,
                    ),
            ),
    ) as *mut libc::c_uchar;
    if simple_object_internal_read(
        (*sobj).descriptor,
        ((*sobj).offset as libc::c_ulong)
            .wrapping_add((*eor).shoff)
            .wrapping_add(shdr_size) as off_t,
        shdrs,
        shdr_size
            .wrapping_mul(
                shnum.wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_ulong,
            ),
        &mut errmsg,
        err,
    ) == 0
    {
        free(shdrs as *mut libc::c_void);
        return errmsg;
    }
    shstrhdr = shdrs
        .offset(
            (((*eor).shstrndx).wrapping_sub(1 as libc::c_int as libc::c_uint)
                as libc::c_ulong)
                .wrapping_mul(shdr_size) as isize,
        );
    name_size = if ei_class as libc::c_int == 1 as libc::c_int {
        ((*type_functions).fetch_Elf_Addr)
            .expect(
                "non-null function pointer",
            )(shstrhdr.offset(20 as libc::c_ulong as isize))
    } else {
        ((*type_functions).fetch_Elf_Addr)
            .expect(
                "non-null function pointer",
            )(shstrhdr.offset(32 as libc::c_ulong as isize))
    };
    shstroff = (if ei_class as libc::c_int == 1 as libc::c_int {
        ((*type_functions).fetch_Elf_Addr)
            .expect(
                "non-null function pointer",
            )(shstrhdr.offset(16 as libc::c_ulong as isize))
    } else {
        ((*type_functions).fetch_Elf_Addr)
            .expect(
                "non-null function pointer",
            )(shstrhdr.offset(24 as libc::c_ulong as isize))
    }) as off_t;
    names = xmalloc(
        (::core::mem::size_of::<libc::c_uchar>() as libc::c_ulong)
            .wrapping_mul(name_size),
    ) as *mut libc::c_uchar;
    if simple_object_internal_read(
        (*sobj).descriptor,
        (*sobj).offset + shstroff,
        names,
        name_size,
        &mut errmsg,
        err,
    ) == 0
    {
        free(names as *mut libc::c_void);
        free(shdrs as *mut libc::c_void);
        return errmsg;
    }
    pfnret = xmalloc(
        (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(shnum as libc::c_ulong),
    ) as *mut libc::c_int;
    pfnname = xmalloc(
        (::core::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
            .wrapping_mul(shnum as libc::c_ulong),
    ) as *mut *const libc::c_char;
    symtab_indices_shndx = xcalloc(
        shnum.wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        ::core::mem::size_of::<libc::c_uint>() as libc::c_ulong,
    ) as *mut libc::c_uint;
    i = 1 as libc::c_int as libc::c_uint;
    while i < shnum {
        let mut shdr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        let mut sh_name: libc::c_uint = 0;
        let mut sh_type: libc::c_uint = 0;
        let mut name: *const libc::c_char = 0 as *const libc::c_char;
        let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
        shdr = shdrs
            .offset(
                (i.wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_ulong)
                    .wrapping_mul(shdr_size) as isize,
            );
        sh_name = if ei_class as libc::c_int == 1 as libc::c_int {
            ((*type_functions).fetch_Elf_Word)
                .expect(
                    "non-null function pointer",
                )(shdr.offset(0 as libc::c_ulong as isize))
        } else {
            ((*type_functions).fetch_Elf_Word)
                .expect(
                    "non-null function pointer",
                )(shdr.offset(0 as libc::c_ulong as isize))
        };
        if sh_name as libc::c_ulong >= name_size {
            *err = 0 as libc::c_int;
            free(names as *mut libc::c_void);
            free(shdrs as *mut libc::c_void);
            return b"ELF section name out of range\0" as *const u8
                as *const libc::c_char;
        }
        name = (names as *const libc::c_char).offset(sh_name as isize);
        ret = (Some(pfn.expect("non-null function pointer")))
            .expect("non-null function pointer")(name);
        *pfnret
            .offset(
                i.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
            ) = if ret.is_null() { -(1 as libc::c_int) } else { 0 as libc::c_int };
        let ref mut fresh0 = *pfnname
            .offset(i.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize);
        *fresh0 = if ret.is_null() { name } else { ret as *const libc::c_char };
        if first_shndx == 0 as libc::c_int as libc::c_uint
            && *pfnret.offset(i.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize)
                == 0 as libc::c_int
        {
            first_shndx = i;
        }
        sh_type = if ei_class as libc::c_int == 1 as libc::c_int {
            ((*type_functions).fetch_Elf_Word)
                .expect(
                    "non-null function pointer",
                )(shdr.offset(4 as libc::c_ulong as isize))
        } else {
            ((*type_functions).fetch_Elf_Word)
                .expect(
                    "non-null function pointer",
                )(shdr.offset(4 as libc::c_ulong as isize))
        };
        if sh_type == 18 as libc::c_int as libc::c_uint {
            let mut sh_link: libc::c_uint = 0;
            sh_link = if ei_class as libc::c_int == 1 as libc::c_int {
                ((*type_functions).fetch_Elf_Word)
                    .expect(
                        "non-null function pointer",
                    )(shdr.offset(24 as libc::c_ulong as isize))
            } else {
                ((*type_functions).fetch_Elf_Word)
                    .expect(
                        "non-null function pointer",
                    )(shdr.offset(40 as libc::c_ulong as isize))
            };
            *symtab_indices_shndx
                .offset(
                    sh_link.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
                ) = i.wrapping_sub(1 as libc::c_int as libc::c_uint);
            *pfnret
                .offset(
                    i.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
                ) = -(1 as libc::c_int);
        }
        i = i.wrapping_add(1);
        i;
    }
    loop {
        changed = 0 as libc::c_int;
        i = 1 as libc::c_int as libc::c_uint;
        while i < shnum {
            let mut shdr_0: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
            let mut sh_type_0: libc::c_uint = 0;
            let mut sh_info: libc::c_uint = 0;
            let mut sh_link_0: libc::c_uint = 0;
            let mut offset: off_t = 0;
            let mut length: off_t = 0;
            shdr_0 = shdrs
                .offset(
                    (i.wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_ulong)
                        .wrapping_mul(shdr_size) as isize,
                );
            sh_type_0 = if ei_class as libc::c_int == 1 as libc::c_int {
                ((*type_functions).fetch_Elf_Word)
                    .expect(
                        "non-null function pointer",
                    )(shdr_0.offset(4 as libc::c_ulong as isize))
            } else {
                ((*type_functions).fetch_Elf_Word)
                    .expect(
                        "non-null function pointer",
                    )(shdr_0.offset(4 as libc::c_ulong as isize))
            };
            sh_info = if ei_class as libc::c_int == 1 as libc::c_int {
                ((*type_functions).fetch_Elf_Word)
                    .expect(
                        "non-null function pointer",
                    )(shdr_0.offset(28 as libc::c_ulong as isize))
            } else {
                ((*type_functions).fetch_Elf_Word)
                    .expect(
                        "non-null function pointer",
                    )(shdr_0.offset(44 as libc::c_ulong as isize))
            };
            sh_link_0 = if ei_class as libc::c_int == 1 as libc::c_int {
                ((*type_functions).fetch_Elf_Word)
                    .expect(
                        "non-null function pointer",
                    )(shdr_0.offset(24 as libc::c_ulong as isize))
            } else {
                ((*type_functions).fetch_Elf_Word)
                    .expect(
                        "non-null function pointer",
                    )(shdr_0.offset(40 as libc::c_ulong as isize))
            };
            if sh_type_0 == 17 as libc::c_int as libc::c_uint {
                let mut entsize: libc::c_uint = (if ei_class as libc::c_int
                    == 1 as libc::c_int
                {
                    ((*type_functions).fetch_Elf_Addr)
                        .expect(
                            "non-null function pointer",
                        )(shdr_0.offset(36 as libc::c_ulong as isize))
                } else {
                    ((*type_functions).fetch_Elf_Addr)
                        .expect(
                            "non-null function pointer",
                        )(shdr_0.offset(56 as libc::c_ulong as isize))
                }) as libc::c_uint;
                let mut ent: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                let mut buf: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                let mut keep: libc::c_int = 0 as libc::c_int;
                offset = (if ei_class as libc::c_int == 1 as libc::c_int {
                    ((*type_functions).fetch_Elf_Addr)
                        .expect(
                            "non-null function pointer",
                        )(shdr_0.offset(16 as libc::c_ulong as isize))
                } else {
                    ((*type_functions).fetch_Elf_Addr)
                        .expect(
                            "non-null function pointer",
                        )(shdr_0.offset(24 as libc::c_ulong as isize))
                }) as off_t;
                length = (if ei_class as libc::c_int == 1 as libc::c_int {
                    ((*type_functions).fetch_Elf_Addr)
                        .expect(
                            "non-null function pointer",
                        )(shdr_0.offset(20 as libc::c_ulong as isize))
                } else {
                    ((*type_functions).fetch_Elf_Addr)
                        .expect(
                            "non-null function pointer",
                        )(shdr_0.offset(32 as libc::c_ulong as isize))
                }) as off_t;
                buf = xmalloc(
                    (::core::mem::size_of::<libc::c_uchar>() as libc::c_ulong)
                        .wrapping_mul(length as libc::c_ulong),
                ) as *mut libc::c_uchar;
                if simple_object_internal_read(
                    (*sobj).descriptor,
                    (*sobj).offset + offset,
                    buf,
                    length as size_t,
                    &mut errmsg,
                    err,
                ) == 0
                {
                    free(buf as *mut libc::c_void);
                    free(names as *mut libc::c_void);
                    free(shdrs as *mut libc::c_void);
                    return errmsg;
                }
                ent = buf.offset(entsize as isize);
                while ent < buf.offset(length as isize) {
                    let mut sec: libc::c_uint = ((*type_functions).fetch_Elf_Word)
                        .expect("non-null function pointer")(ent);
                    if *pfnret
                        .offset(
                            sec.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
                        ) == 0 as libc::c_int
                    {
                        keep = 1 as libc::c_int;
                    }
                    ent = ent.offset(entsize as isize);
                }
                if keep != 0 {
                    changed
                        |= (*pfnret
                            .offset(
                                sh_link_0.wrapping_sub(1 as libc::c_int as libc::c_uint)
                                    as isize,
                            ) == -(1 as libc::c_int)
                            || *pfnret
                                .offset(
                                    i.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
                                ) == -(1 as libc::c_int)) as libc::c_int;
                    *pfnret
                        .offset(
                            sh_link_0.wrapping_sub(1 as libc::c_int as libc::c_uint)
                                as isize,
                        ) = 0 as libc::c_int;
                    *pfnret
                        .offset(
                            i.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
                        ) = 0 as libc::c_int;
                }
            }
            if sh_type_0 == 4 as libc::c_int as libc::c_uint
                || sh_type_0 == 9 as libc::c_int as libc::c_uint
            {
                if *pfnret
                    .offset(
                        sh_info.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
                    ) == 0 as libc::c_int
                {
                    changed
                        |= (*pfnret
                            .offset(
                                sh_link_0.wrapping_sub(1 as libc::c_int as libc::c_uint)
                                    as isize,
                            ) == -(1 as libc::c_int)
                            || *pfnret
                                .offset(
                                    i.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
                                ) == -(1 as libc::c_int)) as libc::c_int;
                    *pfnret
                        .offset(
                            sh_link_0.wrapping_sub(1 as libc::c_int as libc::c_uint)
                                as isize,
                        ) = 0 as libc::c_int;
                    *pfnret
                        .offset(
                            i.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
                        ) = 0 as libc::c_int;
                }
            }
            if sh_type_0 == 2 as libc::c_int as libc::c_uint {
                if *pfnret
                    .offset(i.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize)
                    == 0 as libc::c_int
                {
                    changed
                        |= (*pfnret
                            .offset(
                                sh_link_0.wrapping_sub(1 as libc::c_int as libc::c_uint)
                                    as isize,
                            ) == -(1 as libc::c_int)) as libc::c_int;
                    *pfnret
                        .offset(
                            sh_link_0.wrapping_sub(1 as libc::c_int as libc::c_uint)
                                as isize,
                        ) = 0 as libc::c_int;
                }
            }
            i = i.wrapping_add(1);
            i;
        }
        if !(changed != 0) {
            break;
        }
    }
    sh_map = xmalloc(
        (::core::mem::size_of::<libc::c_uint>() as libc::c_ulong)
            .wrapping_mul(shnum as libc::c_ulong),
    ) as *mut libc::c_uint;
    *sh_map.offset(0 as libc::c_int as isize) = 0 as libc::c_int as libc::c_uint;
    new_i = 1 as libc::c_int as libc::c_uint;
    i = 1 as libc::c_int as libc::c_uint;
    while i < shnum {
        if *pfnret.offset(i.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize)
            == -(1 as libc::c_int)
        {
            *sh_map.offset(i as isize) = 0 as libc::c_int as libc::c_uint;
        } else {
            let fresh1 = new_i;
            new_i = new_i.wrapping_add(1);
            *sh_map.offset(i as isize) = fresh1;
        }
        i = i.wrapping_add(1);
        i;
    }
    if new_i.wrapping_sub(1 as libc::c_int as libc::c_uint)
        >= 0xff00 as libc::c_int as libc::c_uint
    {
        *err = 95 as libc::c_int;
        return b"Too many copied sections\0" as *const u8 as *const libc::c_char;
    }
    (*eow)
        .shdrs = xmalloc(
        (::core::mem::size_of::<libc::c_uchar>() as libc::c_ulong)
            .wrapping_mul(
                shdr_size
                    .wrapping_mul(
                        new_i.wrapping_sub(1 as libc::c_int as libc::c_uint)
                            as libc::c_ulong,
                    ),
            ),
    ) as *mut libc::c_uchar;
    new_i = 0 as libc::c_int as libc::c_uint;
    i = 1 as libc::c_int as libc::c_uint;
    while i < shnum {
        let mut shdr_1: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        let mut sh_name_0: libc::c_uint = 0;
        let mut sh_type_1: libc::c_uint = 0;
        let mut name_0: *const libc::c_char = 0 as *const libc::c_char;
        let mut offset_0: off_t = 0;
        let mut length_0: off_t = 0;
        let mut dest: *mut simple_object_write_section = 0
            as *mut simple_object_write_section;
        let mut flags: off_t = 0;
        let mut buf_0: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        if !(*pfnret.offset(i.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize)
            != 0)
        {
            new_i = new_i.wrapping_add(1);
            new_i;
            shdr_1 = shdrs
                .offset(
                    (i.wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_ulong)
                        .wrapping_mul(shdr_size) as isize,
                );
            sh_name_0 = if ei_class as libc::c_int == 1 as libc::c_int {
                ((*type_functions).fetch_Elf_Word)
                    .expect(
                        "non-null function pointer",
                    )(shdr_1.offset(0 as libc::c_ulong as isize))
            } else {
                ((*type_functions).fetch_Elf_Word)
                    .expect(
                        "non-null function pointer",
                    )(shdr_1.offset(0 as libc::c_ulong as isize))
            };
            if sh_name_0 as libc::c_ulong >= name_size {
                *err = 0 as libc::c_int;
                free(names as *mut libc::c_void);
                free(shdrs as *mut libc::c_void);
                free(symtab_indices_shndx as *mut libc::c_void);
                return b"ELF section name out of range\0" as *const u8
                    as *const libc::c_char;
            }
            name_0 = *pfnname
                .offset(i.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize);
            offset_0 = (if ei_class as libc::c_int == 1 as libc::c_int {
                ((*type_functions).fetch_Elf_Addr)
                    .expect(
                        "non-null function pointer",
                    )(shdr_1.offset(16 as libc::c_ulong as isize))
            } else {
                ((*type_functions).fetch_Elf_Addr)
                    .expect(
                        "non-null function pointer",
                    )(shdr_1.offset(24 as libc::c_ulong as isize))
            }) as off_t;
            length_0 = (if ei_class as libc::c_int == 1 as libc::c_int {
                ((*type_functions).fetch_Elf_Addr)
                    .expect(
                        "non-null function pointer",
                    )(shdr_1.offset(20 as libc::c_ulong as isize))
            } else {
                ((*type_functions).fetch_Elf_Addr)
                    .expect(
                        "non-null function pointer",
                    )(shdr_1.offset(32 as libc::c_ulong as isize))
            }) as off_t;
            sh_type_1 = if ei_class as libc::c_int == 1 as libc::c_int {
                ((*type_functions).fetch_Elf_Word)
                    .expect(
                        "non-null function pointer",
                    )(shdr_1.offset(4 as libc::c_ulong as isize))
            } else {
                ((*type_functions).fetch_Elf_Word)
                    .expect(
                        "non-null function pointer",
                    )(shdr_1.offset(4 as libc::c_ulong as isize))
            };
            dest = simple_object_write_create_section(
                dobj,
                *pfnname
                    .offset(i.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize),
                0 as libc::c_int as libc::c_uint,
                &mut errmsg,
                err,
            );
            if dest.is_null() {
                free(names as *mut libc::c_void);
                free(shdrs as *mut libc::c_void);
                free(symtab_indices_shndx as *mut libc::c_void);
                return errmsg;
            }
            memcpy(
                ((*eow).shdrs)
                    .offset(
                        (new_i.wrapping_sub(1 as libc::c_int as libc::c_uint)
                            as libc::c_ulong)
                            .wrapping_mul(shdr_size) as isize,
                    ) as *mut libc::c_void,
                shdr_1 as *const libc::c_void,
                shdr_size,
            );
            shdr_1 = ((*eow).shdrs)
                .offset(
                    (new_i.wrapping_sub(1 as libc::c_int as libc::c_uint)
                        as libc::c_ulong)
                        .wrapping_mul(shdr_size) as isize,
                );
            buf_0 = xmalloc(
                (::core::mem::size_of::<libc::c_uchar>() as libc::c_ulong)
                    .wrapping_mul(length_0 as libc::c_ulong),
            ) as *mut libc::c_uchar;
            if simple_object_internal_read(
                (*sobj).descriptor,
                (*sobj).offset + offset_0,
                buf_0,
                length_0 as size_t,
                &mut errmsg,
                err,
            ) == 0
            {
                free(buf_0 as *mut libc::c_void);
                free(names as *mut libc::c_void);
                free(shdrs as *mut libc::c_void);
                free(symtab_indices_shndx as *mut libc::c_void);
                return errmsg;
            }
            if sh_type_1 == 2 as libc::c_int as libc::c_uint {
                let mut entsize_0: libc::c_uint = (if ei_class as libc::c_int
                    == 1 as libc::c_int
                {
                    ((*type_functions).fetch_Elf_Addr)
                        .expect(
                            "non-null function pointer",
                        )(shdr_1.offset(36 as libc::c_ulong as isize))
                } else {
                    ((*type_functions).fetch_Elf_Addr)
                        .expect(
                            "non-null function pointer",
                        )(shdr_1.offset(56 as libc::c_ulong as isize))
                }) as libc::c_uint;
                let mut prevailing_name_idx: size_t = 0 as libc::c_int as size_t;
                let mut ent_0: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                let mut shndx_table: *mut libc::c_uint = 0 as *mut libc::c_uint;
                if *symtab_indices_shndx
                    .offset(i.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize)
                    != 0 as libc::c_int as libc::c_uint
                {
                    let mut sidxhdr: *mut libc::c_uchar = shdrs
                        .offset(
                            (*symtab_indices_shndx
                                .offset(
                                    i.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
                                ) as libc::c_ulong)
                                .wrapping_mul(shdr_size) as isize,
                        );
                    let mut sidxoff: off_t = (if ei_class as libc::c_int
                        == 1 as libc::c_int
                    {
                        ((*type_functions).fetch_Elf_Addr)
                            .expect(
                                "non-null function pointer",
                            )(sidxhdr.offset(16 as libc::c_ulong as isize))
                    } else {
                        ((*type_functions).fetch_Elf_Addr)
                            .expect(
                                "non-null function pointer",
                            )(sidxhdr.offset(24 as libc::c_ulong as isize))
                    }) as off_t;
                    let mut sidxsz: size_t = if ei_class as libc::c_int
                        == 1 as libc::c_int
                    {
                        ((*type_functions).fetch_Elf_Addr)
                            .expect(
                                "non-null function pointer",
                            )(sidxhdr.offset(20 as libc::c_ulong as isize))
                    } else {
                        ((*type_functions).fetch_Elf_Addr)
                            .expect(
                                "non-null function pointer",
                            )(sidxhdr.offset(32 as libc::c_ulong as isize))
                    };
                    let mut shndx_type: libc::c_uint = if ei_class as libc::c_int
                        == 1 as libc::c_int
                    {
                        ((*type_functions).fetch_Elf_Word)
                            .expect(
                                "non-null function pointer",
                            )(sidxhdr.offset(4 as libc::c_ulong as isize))
                    } else {
                        ((*type_functions).fetch_Elf_Word)
                            .expect(
                                "non-null function pointer",
                            )(sidxhdr.offset(4 as libc::c_ulong as isize))
                    };
                    if shndx_type != 18 as libc::c_int as libc::c_uint {
                        return b"Wrong section type of a SYMTAB SECTION INDICES section\0"
                            as *const u8 as *const libc::c_char;
                    }
                    shndx_table = xmalloc(
                        (::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
                            .wrapping_mul(sidxsz),
                    ) as *mut libc::c_char as *mut libc::c_uint;
                    simple_object_internal_read(
                        (*sobj).descriptor,
                        (*sobj).offset + sidxoff,
                        shndx_table as *mut libc::c_uchar,
                        sidxsz,
                        &mut errmsg,
                        err,
                    );
                }
                ent_0 = buf_0;
                while ent_0 < buf_0.offset(length_0 as isize) {
                    let mut st_shndx: libc::c_uint = (if ei_class as libc::c_int
                        == 1 as libc::c_int
                    {
                        ((*type_functions).fetch_Elf_Half)
                            .expect(
                                "non-null function pointer",
                            )(ent_0.offset(14 as libc::c_ulong as isize)) as libc::c_int
                    } else {
                        ((*type_functions).fetch_Elf_Half)
                            .expect(
                                "non-null function pointer",
                            )(ent_0.offset(6 as libc::c_ulong as isize)) as libc::c_int
                    }) as libc::c_uint;
                    let mut st_info: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                    let mut st_other: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                    if ei_class as libc::c_int == 1 as libc::c_int {
                        st_info = &mut (*(ent_0 as *mut Elf32_External_Sym)).st_info;
                        st_other = &mut (*(ent_0 as *mut Elf32_External_Sym)).st_other;
                    } else {
                        st_info = &mut (*(ent_0 as *mut Elf64_External_Sym)).st_info;
                        st_other = &mut (*(ent_0 as *mut Elf64_External_Sym)).st_other;
                    }
                    if st_shndx == 0xffff as libc::c_int as libc::c_uint {
                        st_shndx = ((*type_functions).fetch_Elf_Word)
                            .expect(
                                "non-null function pointer",
                            )(
                            shndx_table
                                .offset(
                                    (ent_0.offset_from(buf_0) as libc::c_long
                                        / entsize_0 as libc::c_long) as isize,
                                ) as *mut libc::c_uchar,
                        );
                    }
                    if st_shndx != 0xfff2 as libc::c_int as libc::c_uint
                        && !(st_shndx != 0 as libc::c_int as libc::c_uint
                            && st_shndx < shnum
                            && *pfnret
                                .offset(
                                    st_shndx.wrapping_sub(1 as libc::c_int as libc::c_uint)
                                        as isize,
                                ) == -(1 as libc::c_int))
                        && *st_info as libc::c_int >> 4 as libc::c_int
                            == 2 as libc::c_int
                        && *st_other as libc::c_int == 2 as libc::c_int
                    {
                        prevailing_name_idx = (if ei_class as libc::c_int
                            == 1 as libc::c_int
                        {
                            ((*type_functions).fetch_Elf_Word)
                                .expect(
                                    "non-null function pointer",
                                )(ent_0.offset(0 as libc::c_ulong as isize))
                        } else {
                            ((*type_functions).fetch_Elf_Word)
                                .expect(
                                    "non-null function pointer",
                                )(ent_0.offset(0 as libc::c_ulong as isize))
                        }) as size_t;
                        break;
                    } else {
                        ent_0 = ent_0.offset(entsize_0 as isize);
                    }
                }
                ent_0 = buf_0;
                while ent_0 < buf_0.offset(length_0 as isize) {
                    let mut st_shndx_0: libc::c_uint = (if ei_class as libc::c_int
                        == 1 as libc::c_int
                    {
                        ((*type_functions).fetch_Elf_Half)
                            .expect(
                                "non-null function pointer",
                            )(ent_0.offset(14 as libc::c_ulong as isize)) as libc::c_int
                    } else {
                        ((*type_functions).fetch_Elf_Half)
                            .expect(
                                "non-null function pointer",
                            )(ent_0.offset(6 as libc::c_ulong as isize)) as libc::c_int
                    }) as libc::c_uint;
                    let mut raw_st_shndx: libc::c_uint = st_shndx_0;
                    let mut st_info_0: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                    let mut st_other_0: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                    let mut discard: libc::c_int = 0 as libc::c_int;
                    if ei_class as libc::c_int == 1 as libc::c_int {
                        st_info_0 = &mut (*(ent_0 as *mut Elf32_External_Sym)).st_info;
                        st_other_0 = &mut (*(ent_0 as *mut Elf32_External_Sym)).st_other;
                    } else {
                        st_info_0 = &mut (*(ent_0 as *mut Elf64_External_Sym)).st_info;
                        st_other_0 = &mut (*(ent_0 as *mut Elf64_External_Sym)).st_other;
                    }
                    if st_shndx_0 == 0xffff as libc::c_int as libc::c_uint {
                        st_shndx_0 = ((*type_functions).fetch_Elf_Word)
                            .expect(
                                "non-null function pointer",
                            )(
                            shndx_table
                                .offset(
                                    (ent_0.offset_from(buf_0) as libc::c_long
                                        / entsize_0 as libc::c_long) as isize,
                                ) as *mut libc::c_uchar,
                        );
                    }
                    if st_shndx_0 == 0xfff2 as libc::c_int as libc::c_uint {
                        discard = 1 as libc::c_int;
                    } else if st_shndx_0 != 0 as libc::c_int as libc::c_uint
                        && st_shndx_0 < shnum
                        && *pfnret
                            .offset(
                                st_shndx_0.wrapping_sub(1 as libc::c_int as libc::c_uint)
                                    as isize,
                            ) == -(1 as libc::c_int)
                    {
                        discard = 1 as libc::c_int;
                    } else if st_shndx_0 == 0 as libc::c_int as libc::c_uint
                        && *st_info_0 as libc::c_int >> 4 as libc::c_int
                            == 1 as libc::c_int
                    {
                        discard = 1 as libc::c_int;
                    }
                    if discard != 0 {
                        let mut bind: libc::c_int = *st_info_0 as libc::c_int
                            >> 4 as libc::c_int;
                        let mut other: libc::c_int = 0 as libc::c_int;
                        if bind == 0 as libc::c_int {
                            if ei_class as libc::c_int == 1 as libc::c_int {
                                ((*type_functions).set_Elf_Word)
                                    .expect(
                                        "non-null function pointer",
                                    )(
                                    ent_0.offset(0 as libc::c_ulong as isize),
                                    0 as libc::c_int as libc::c_uint,
                                );
                            } else {
                                ((*type_functions).set_Elf_Word)
                                    .expect(
                                        "non-null function pointer",
                                    )(
                                    ent_0.offset(0 as libc::c_ulong as isize),
                                    0 as libc::c_int as libc::c_uint,
                                );
                            };
                            if ei_class as libc::c_int == 1 as libc::c_int {
                                ((*type_functions).set_Elf_Half)
                                    .expect(
                                        "non-null function pointer",
                                    )(
                                    ent_0.offset(14 as libc::c_ulong as isize),
                                    *sh_map.offset(first_shndx as isize) as libc::c_ushort,
                                );
                            } else {
                                ((*type_functions).set_Elf_Half)
                                    .expect(
                                        "non-null function pointer",
                                    )(
                                    ent_0.offset(6 as libc::c_ulong as isize),
                                    *sh_map.offset(first_shndx as isize) as libc::c_ushort,
                                );
                            };
                        } else {
                            bind = 2 as libc::c_int;
                            other = 2 as libc::c_int;
                            if ei_class as libc::c_int == 1 as libc::c_int {
                                ((*type_functions).set_Elf_Word)
                                    .expect(
                                        "non-null function pointer",
                                    )(
                                    ent_0.offset(0 as libc::c_ulong as isize),
                                    prevailing_name_idx as libc::c_uint,
                                );
                            } else {
                                ((*type_functions).set_Elf_Word)
                                    .expect(
                                        "non-null function pointer",
                                    )(
                                    ent_0.offset(0 as libc::c_ulong as isize),
                                    prevailing_name_idx as libc::c_uint,
                                );
                            };
                            if ei_class as libc::c_int == 1 as libc::c_int {
                                ((*type_functions).set_Elf_Half)
                                    .expect(
                                        "non-null function pointer",
                                    )(
                                    ent_0.offset(14 as libc::c_ulong as isize),
                                    0 as libc::c_int as libc::c_ushort,
                                );
                            } else {
                                ((*type_functions).set_Elf_Half)
                                    .expect(
                                        "non-null function pointer",
                                    )(
                                    ent_0.offset(6 as libc::c_ulong as isize),
                                    0 as libc::c_int as libc::c_ushort,
                                );
                            };
                        }
                        *st_other_0 = other as libc::c_uchar;
                        *st_info_0 = ((bind << 4 as libc::c_int)
                            + (0 as libc::c_int & 0xf as libc::c_int)) as libc::c_uchar;
                        if ei_class as libc::c_int == 1 as libc::c_int {
                            ((*type_functions).set_Elf_Addr)
                                .expect(
                                    "non-null function pointer",
                                )(
                                ent_0.offset(4 as libc::c_ulong as isize),
                                0 as libc::c_int as ulong_type,
                            );
                        } else {
                            ((*type_functions).set_Elf_Addr)
                                .expect(
                                    "non-null function pointer",
                                )(
                                ent_0.offset(8 as libc::c_ulong as isize),
                                0 as libc::c_int as ulong_type,
                            );
                        };
                        if ei_class as libc::c_int == 1 as libc::c_int {
                            ((*type_functions).set_Elf_Word)
                                .expect(
                                    "non-null function pointer",
                                )(
                                ent_0.offset(8 as libc::c_ulong as isize),
                                0 as libc::c_int as libc::c_uint,
                            );
                        } else {
                            ((*type_functions).set_Elf_Word)
                                .expect(
                                    "non-null function pointer",
                                )(
                                ent_0.offset(16 as libc::c_ulong as isize),
                                0 as libc::c_int as libc::c_uint,
                            );
                        };
                    } else if raw_st_shndx < 0xff00 as libc::c_int as libc::c_uint
                        || raw_st_shndx == 0xffff as libc::c_int as libc::c_uint
                    {
                        if ei_class as libc::c_int == 1 as libc::c_int {
                            ((*type_functions).set_Elf_Half)
                                .expect(
                                    "non-null function pointer",
                                )(
                                ent_0.offset(14 as libc::c_ulong as isize),
                                *sh_map.offset(st_shndx_0 as isize) as libc::c_ushort,
                            );
                        } else {
                            ((*type_functions).set_Elf_Half)
                                .expect(
                                    "non-null function pointer",
                                )(
                                ent_0.offset(6 as libc::c_ulong as isize),
                                *sh_map.offset(st_shndx_0 as isize) as libc::c_ushort,
                            );
                        };
                    }
                    ent_0 = ent_0.offset(entsize_0 as isize);
                }
                free(shndx_table as *mut libc::c_void);
            } else if sh_type_1 == 17 as libc::c_int as libc::c_uint {
                let mut ent_1: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                let mut dst: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                ent_1 = buf_0.offset(4 as libc::c_int as isize);
                dst = ent_1;
                while ent_1 < buf_0.offset(length_0 as isize) {
                    let mut shndx: libc::c_uint = ((*type_functions).fetch_Elf_Word)
                        .expect("non-null function pointer")(ent_1);
                    if !(*pfnret
                        .offset(
                            shndx.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
                        ) == -(1 as libc::c_int))
                    {
                        ((*type_functions).set_Elf_Word)
                            .expect(
                                "non-null function pointer",
                            )(dst, *sh_map.offset(shndx as isize));
                        dst = dst.offset(4 as libc::c_int as isize);
                    }
                    ent_1 = ent_1.offset(4 as libc::c_int as isize);
                }
                length_0 = dst.offset_from(buf_0) as libc::c_long;
            }
            errmsg = simple_object_write_add_data(
                dobj,
                dest,
                buf_0 as *const libc::c_void,
                length_0 as size_t,
                1 as libc::c_int,
                err,
            );
            free(buf_0 as *mut libc::c_void);
            if !errmsg.is_null() {
                free(names as *mut libc::c_void);
                free(shdrs as *mut libc::c_void);
                free(symtab_indices_shndx as *mut libc::c_void);
                return errmsg;
            }
            flags = (if ei_class as libc::c_int == 1 as libc::c_int {
                ((*type_functions).fetch_Elf_Addr)
                    .expect(
                        "non-null function pointer",
                    )(shdr_1.offset(8 as libc::c_ulong as isize))
            } else {
                ((*type_functions).fetch_Elf_Addr)
                    .expect(
                        "non-null function pointer",
                    )(shdr_1.offset(8 as libc::c_ulong as isize))
            }) as off_t;
            let mut sh_info_0: libc::c_uint = 0;
            let mut sh_link_1: libc::c_uint = 0;
            if flags & 0x40 as libc::c_int as libc::c_long != 0
                || sh_type_1 == 9 as libc::c_int as libc::c_uint
                || sh_type_1 == 4 as libc::c_int as libc::c_uint
            {
                sh_info_0 = if ei_class as libc::c_int == 1 as libc::c_int {
                    ((*type_functions).fetch_Elf_Word)
                        .expect(
                            "non-null function pointer",
                        )(shdr_1.offset(28 as libc::c_ulong as isize))
                } else {
                    ((*type_functions).fetch_Elf_Word)
                        .expect(
                            "non-null function pointer",
                        )(shdr_1.offset(44 as libc::c_ulong as isize))
                };
                if sh_info_0 < 0xff00 as libc::c_int as libc::c_uint
                    || sh_info_0 > 0xffff as libc::c_int as libc::c_uint
                {
                    sh_info_0 = *sh_map.offset(sh_info_0 as isize);
                }
                if ei_class as libc::c_int == 1 as libc::c_int {
                    ((*type_functions).set_Elf_Word)
                        .expect(
                            "non-null function pointer",
                        )(shdr_1.offset(28 as libc::c_ulong as isize), sh_info_0);
                } else {
                    ((*type_functions).set_Elf_Word)
                        .expect(
                            "non-null function pointer",
                        )(shdr_1.offset(44 as libc::c_ulong as isize), sh_info_0);
                };
            }
            sh_link_1 = if ei_class as libc::c_int == 1 as libc::c_int {
                ((*type_functions).fetch_Elf_Word)
                    .expect(
                        "non-null function pointer",
                    )(shdr_1.offset(24 as libc::c_ulong as isize))
            } else {
                ((*type_functions).fetch_Elf_Word)
                    .expect(
                        "non-null function pointer",
                    )(shdr_1.offset(40 as libc::c_ulong as isize))
            };
            if sh_link_1 < 0xff00 as libc::c_int as libc::c_uint
                || sh_link_1 > 0xffff as libc::c_int as libc::c_uint
            {
                sh_link_1 = *sh_map.offset(sh_link_1 as isize);
            }
            if ei_class as libc::c_int == 1 as libc::c_int {
                ((*type_functions).set_Elf_Word)
                    .expect(
                        "non-null function pointer",
                    )(shdr_1.offset(24 as libc::c_ulong as isize), sh_link_1);
            } else {
                ((*type_functions).set_Elf_Word)
                    .expect(
                        "non-null function pointer",
                    )(shdr_1.offset(40 as libc::c_ulong as isize), sh_link_1);
            };
            if strcmp(name_0, b".note.GNU-stack\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                flags &= !(0x4 as libc::c_int) as libc::c_long;
            }
            flags &= !(0x80000000 as libc::c_uint) as libc::c_long;
            if ei_class as libc::c_int == 1 as libc::c_int {
                ((*type_functions).set_Elf_Addr)
                    .expect(
                        "non-null function pointer",
                    )(shdr_1.offset(8 as libc::c_ulong as isize), flags as ulong_type);
            } else {
                ((*type_functions).set_Elf_Addr)
                    .expect(
                        "non-null function pointer",
                    )(shdr_1.offset(8 as libc::c_ulong as isize), flags as ulong_type);
            };
        }
        i = i.wrapping_add(1);
        i;
    }
    free(names as *mut libc::c_void);
    free(shdrs as *mut libc::c_void);
    free(pfnret as *mut libc::c_void);
    free(pfnname as *mut libc::c_void);
    free(symtab_indices_shndx as *mut libc::c_void);
    free(sh_map as *mut libc::c_void);
    return 0 as *const libc::c_char;
}
#[no_mangle]
pub static mut simple_object_elf_functions: simple_object_functions = unsafe {
    {
        let mut init = simple_object_functions {
            match_0: Some(
                simple_object_elf_match
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
                simple_object_elf_find_sections
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
                simple_object_elf_fetch_attributes
                    as unsafe extern "C" fn(
                        *mut simple_object_read,
                        *mut *const libc::c_char,
                        *mut libc::c_int,
                    ) -> *mut libc::c_void,
            ),
            release_read: Some(
                simple_object_elf_release_read
                    as unsafe extern "C" fn(*mut libc::c_void) -> (),
            ),
            attributes_merge: Some(
                simple_object_elf_attributes_merge
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut libc::c_void,
                        *mut libc::c_int,
                    ) -> *const libc::c_char,
            ),
            release_attributes: Some(
                simple_object_elf_release_attributes
                    as unsafe extern "C" fn(*mut libc::c_void) -> (),
            ),
            start_write: Some(
                simple_object_elf_start_write
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut *const libc::c_char,
                        *mut libc::c_int,
                    ) -> *mut libc::c_void,
            ),
            write_to_file: Some(
                simple_object_elf_write_to_file
                    as unsafe extern "C" fn(
                        *mut simple_object_write,
                        libc::c_int,
                        *mut libc::c_int,
                    ) -> *const libc::c_char,
            ),
            release_write: Some(
                simple_object_elf_release_write
                    as unsafe extern "C" fn(*mut libc::c_void) -> (),
            ),
            copy_lto_debug_sections: Some(
                simple_object_elf_copy_lto_debug_sections
                    as unsafe extern "C" fn(
                        *mut simple_object_read,
                        *mut simple_object_write,
                        Option::<
                            unsafe extern "C" fn(
                                *const libc::c_char,
                            ) -> *mut libc::c_char,
                        >,
                        *mut libc::c_int,
                    ) -> *const libc::c_char,
            ),
        };
        init
    }
};
