#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(c_variadic)]
#![feature(extern_types)]
#![feature(label_break_value)]


extern crate libc;
pub mod src {
pub mod elfcomm;
pub mod version;
} // mod src
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fseek(
        __stream: *mut FILE,
        __off: libc::c_long,
        __whence: libc::c_int,
    ) -> libc::c_int;
    fn ftell(__stream: *mut FILE) -> libc::c_long;
    fn rewind(__stream: *mut FILE);
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
    fn strtoul(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_ulong;
    fn free(_: *mut libc::c_void);
    fn abort() -> !;
    fn exit(_: libc::c_int) -> !;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    fn setlocale(
        __category: libc::c_int,
        __locale: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn textdomain(__domainname: *const libc::c_char) -> *mut libc::c_char;
    fn bindtextdomain(
        __domainname: *const libc::c_char,
        __dirname: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn expandargv(_: *mut libc::c_int, _: *mut *mut *mut libc::c_char);
    fn concat(_: *const libc::c_char, _: ...) -> *mut libc::c_char;
    fn reconcat(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: ...
    ) -> *mut libc::c_char;
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    fn error(_: *const libc::c_char, _: ...);
    static mut byte_put: Option::<
        unsafe extern "C" fn(*mut libc::c_uchar, elf_vma, libc::c_uint) -> (),
    >;
    fn byte_put_little_endian(_: *mut libc::c_uchar, _: elf_vma, _: libc::c_uint);
    fn byte_put_big_endian(_: *mut libc::c_uchar, _: elf_vma, _: libc::c_uint);
    static mut byte_get: Option::<
        unsafe extern "C" fn(*const libc::c_uchar, libc::c_uint) -> elf_vma,
    >;
    fn byte_get_big_endian(_: *const libc::c_uchar, _: libc::c_uint) -> elf_vma;
    fn adjust_relative_path(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn release_archive(_: *mut archive_info);
    fn make_qualified_name(
        _: *mut archive_info,
        _: *mut archive_info,
        _: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn byte_get_little_endian(_: *const libc::c_uchar, _: libc::c_uint) -> elf_vma;
    fn setup_archive(
        _: *mut archive_info,
        _: *const libc::c_char,
        _: *mut FILE,
        _: off_t,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_int;
    fn get_archive_member_name(
        _: *mut archive_info,
        _: *mut archive_info,
    ) -> *mut libc::c_char;
    fn print_version(_: *const libc::c_char);
    fn getopt_long(
        argc: libc::c_int,
        argv: *const *mut libc::c_char,
        shortopts: *const libc::c_char,
        longopts: *const option,
        longind: *mut libc::c_int,
    ) -> libc::c_int;
    fn mmap(
        __addr: *mut libc::c_void,
        __len: size_t,
        __prot: libc::c_int,
        __flags: libc::c_int,
        __fd: libc::c_int,
        __offset: __off_t,
    ) -> *mut libc::c_void;
    fn munmap(__addr: *mut libc::c_void, __len: size_t) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
pub type bfd_size_type = libc::c_ulong;
pub type bfd_vma = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ar_hdr {
    pub ar_name: [libc::c_char; 16],
    pub ar_date: [libc::c_char; 12],
    pub ar_uid: [libc::c_char; 6],
    pub ar_gid: [libc::c_char; 6],
    pub ar_mode: [libc::c_char; 8],
    pub ar_size: [libc::c_char; 10],
    pub ar_fmag: [libc::c_char; 2],
}
pub type elf_vma = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct archive_info {
    pub file_name: *mut libc::c_char,
    pub file: *mut FILE,
    pub index_num: elf_vma,
    pub index_array: *mut elf_vma,
    pub sym_table: *mut libc::c_char,
    pub sym_size: libc::c_ulong,
    pub longnames: *mut libc::c_char,
    pub longnames_size: libc::c_ulong,
    pub nested_member_origin: libc::c_ulong,
    pub next_arhdr_offset: libc::c_ulong,
    pub is_thin_archive: libc::c_int,
    pub uses_64bit_indices: libc::c_int,
    pub arhdr: ar_hdr,
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
pub struct Elf_External_Note {
    pub namesz: [libc::c_uchar; 4],
    pub descsz: [libc::c_uchar; 4],
    pub type_0: [libc::c_uchar; 4],
    pub name: [libc::c_char; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct elf_internal_ehdr {
    pub e_ident: [libc::c_uchar; 16],
    pub e_entry: bfd_vma,
    pub e_phoff: bfd_size_type,
    pub e_shoff: bfd_size_type,
    pub e_version: libc::c_ulong,
    pub e_flags: libc::c_ulong,
    pub e_type: libc::c_ushort,
    pub e_machine: libc::c_ushort,
    pub e_ehsize: libc::c_uint,
    pub e_phentsize: libc::c_uint,
    pub e_phnum: libc::c_uint,
    pub e_shentsize: libc::c_uint,
    pub e_shnum: libc::c_uint,
    pub e_shstrndx: libc::c_uint,
}
pub type Elf_Internal_Ehdr = elf_internal_ehdr;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct elf_internal_phdr {
    pub p_type: libc::c_ulong,
    pub p_flags: libc::c_ulong,
    pub p_offset: bfd_vma,
    pub p_vaddr: bfd_vma,
    pub p_paddr: bfd_vma,
    pub p_filesz: bfd_vma,
    pub p_memsz: bfd_vma,
    pub p_align: bfd_vma,
}
pub type Elf_Internal_Phdr = elf_internal_phdr;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct elf_internal_note {
    pub namesz: libc::c_ulong,
    pub descsz: libc::c_ulong,
    pub type_0: libc::c_ulong,
    pub namedata: *mut libc::c_char,
    pub descdata: *mut libc::c_char,
    pub descpos: bfd_vma,
}
pub type Elf_Internal_Note = elf_internal_note;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
pub type elfclass = libc::c_int;
pub const ELF_CLASS_BOTH: elfclass = 3;
pub const ELF_CLASS_64: elfclass = 2;
pub const ELF_CLASS_32: elfclass = 1;
pub const ELF_CLASS_NONE: elfclass = 0;
pub const ELF_CLASS_UNKNOWN: elfclass = -1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub osabi: libc::c_int,
    pub name: *const libc::c_char,
}
pub type command_line_switch = libc::c_uint;
pub const OPTION_DISABLE_X86_FEATURE: command_line_switch = 157;
pub const OPTION_ENABLE_X86_FEATURE: command_line_switch = 156;
pub const OPTION_OUTPUT_OSABI: command_line_switch = 155;
pub const OPTION_INPUT_OSABI: command_line_switch = 154;
pub const OPTION_OUTPUT_TYPE: command_line_switch = 153;
pub const OPTION_INPUT_TYPE: command_line_switch = 152;
pub const OPTION_OUTPUT_MACH: command_line_switch = 151;
pub const OPTION_INPUT_MACH: command_line_switch = 150;
#[no_mangle]
pub static mut program_name: *mut libc::c_char = b"elfedit\0" as *const u8
    as *const libc::c_char as *mut libc::c_char;
static mut archive_file_offset: libc::c_long = 0;
static mut archive_file_size: libc::c_ulong = 0;
static mut elf_header: Elf_Internal_Ehdr = Elf_Internal_Ehdr {
    e_ident: [0; 16],
    e_entry: 0,
    e_phoff: 0,
    e_shoff: 0,
    e_version: 0,
    e_flags: 0,
    e_type: 0,
    e_machine: 0,
    e_ehsize: 0,
    e_phentsize: 0,
    e_phnum: 0,
    e_shentsize: 0,
    e_shnum: 0,
    e_shstrndx: 0,
};
static mut ehdr32: Elf32_External_Ehdr = Elf32_External_Ehdr {
    e_ident: [0; 16],
    e_type: [0; 2],
    e_machine: [0; 2],
    e_version: [0; 4],
    e_entry: [0; 4],
    e_phoff: [0; 4],
    e_shoff: [0; 4],
    e_flags: [0; 4],
    e_ehsize: [0; 2],
    e_phentsize: [0; 2],
    e_phnum: [0; 2],
    e_shentsize: [0; 2],
    e_shnum: [0; 2],
    e_shstrndx: [0; 2],
};
static mut ehdr64: Elf64_External_Ehdr = Elf64_External_Ehdr {
    e_ident: [0; 16],
    e_type: [0; 2],
    e_machine: [0; 2],
    e_version: [0; 4],
    e_entry: [0; 8],
    e_phoff: [0; 8],
    e_shoff: [0; 8],
    e_flags: [0; 4],
    e_ehsize: [0; 2],
    e_phentsize: [0; 2],
    e_phnum: [0; 2],
    e_shentsize: [0; 2],
    e_shnum: [0; 2],
    e_shstrndx: [0; 2],
};
static mut input_elf_machine: libc::c_int = -(1 as libc::c_int);
static mut output_elf_machine: libc::c_int = -(1 as libc::c_int);
static mut input_elf_type: libc::c_int = -(1 as libc::c_int);
static mut output_elf_type: libc::c_int = -(1 as libc::c_int);
static mut input_elf_osabi: libc::c_int = -(1 as libc::c_int);
static mut output_elf_osabi: libc::c_int = -(1 as libc::c_int);
static mut input_elf_class: elfclass = ELF_CLASS_UNKNOWN;
static mut output_elf_class: elfclass = ELF_CLASS_BOTH;
static mut enable_x86_features: libc::c_uint = 0;
static mut disable_x86_features: libc::c_uint = 0;
unsafe extern "C" fn update_gnu_property(
    mut file_name: *const libc::c_char,
    mut file: *mut FILE,
) -> libc::c_int {
    let mut map: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut phdrs: *mut Elf_Internal_Phdr = 0 as *mut Elf_Internal_Phdr;
    let mut st_buf: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    let mut i: libc::c_uint = 0;
    let mut ret: libc::c_int = 0;
    if enable_x86_features == 0 && disable_x86_features == 0 {
        return 0 as libc::c_int;
    }
    if elf_header.e_machine as libc::c_int != 3 as libc::c_int
        && elf_header.e_machine as libc::c_int != 62 as libc::c_int
    {
        error(
            dcgettext(
                0 as *const libc::c_char,
                b"%s: Not an i386 nor x86-64 ELF file\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            file_name,
        );
        return 0 as libc::c_int;
    }
    if fstat(fileno(file), &mut st_buf) < 0 as libc::c_int {
        error(
            dcgettext(
                0 as *const libc::c_char,
                b"%s: stat () failed\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            file_name,
        );
        return 1 as libc::c_int;
    }
    map = mmap(
        0 as *mut libc::c_void,
        st_buf.st_size as size_t,
        0x1 as libc::c_int | 0x2 as libc::c_int,
        0x1 as libc::c_int,
        fileno(file),
        0 as libc::c_int as __off_t,
    ) as *mut libc::c_char;
    if map == -(1 as libc::c_int) as *mut libc::c_void as *mut libc::c_char {
        error(
            dcgettext(
                0 as *const libc::c_char,
                b"%s: mmap () failed\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            file_name,
        );
        return 0 as libc::c_int;
    }
    phdrs = xmalloc(
        (elf_header.e_phnum as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<Elf_Internal_Phdr>() as libc::c_ulong),
    ) as *mut Elf_Internal_Phdr;
    if elf_header.e_ident[4 as libc::c_int as usize] as libc::c_int == 1 as libc::c_int {
        let mut phdrs32: *mut Elf32_External_Phdr = map
            .offset(elf_header.e_phoff as isize) as *mut Elf32_External_Phdr;
        i = 0 as libc::c_int as libc::c_uint;
        while i < elf_header.e_phnum {
            (*phdrs.offset(i as isize))
                .p_type = byte_get
                .expect(
                    "non-null function pointer",
                )(
                ((*phdrs32.offset(i as isize)).p_type).as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_uchar; 4]>() as libc::c_ulong
                    as libc::c_uint,
            );
            (*phdrs.offset(i as isize))
                .p_offset = byte_get
                .expect(
                    "non-null function pointer",
                )(
                ((*phdrs32.offset(i as isize)).p_offset).as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_uchar; 4]>() as libc::c_ulong
                    as libc::c_uint,
            );
            (*phdrs.offset(i as isize))
                .p_vaddr = byte_get
                .expect(
                    "non-null function pointer",
                )(
                ((*phdrs32.offset(i as isize)).p_vaddr).as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_uchar; 4]>() as libc::c_ulong
                    as libc::c_uint,
            );
            (*phdrs.offset(i as isize))
                .p_paddr = byte_get
                .expect(
                    "non-null function pointer",
                )(
                ((*phdrs32.offset(i as isize)).p_paddr).as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_uchar; 4]>() as libc::c_ulong
                    as libc::c_uint,
            );
            (*phdrs.offset(i as isize))
                .p_filesz = byte_get
                .expect(
                    "non-null function pointer",
                )(
                ((*phdrs32.offset(i as isize)).p_filesz).as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_uchar; 4]>() as libc::c_ulong
                    as libc::c_uint,
            );
            (*phdrs.offset(i as isize))
                .p_memsz = byte_get
                .expect(
                    "non-null function pointer",
                )(
                ((*phdrs32.offset(i as isize)).p_memsz).as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_uchar; 4]>() as libc::c_ulong
                    as libc::c_uint,
            );
            (*phdrs.offset(i as isize))
                .p_flags = byte_get
                .expect(
                    "non-null function pointer",
                )(
                ((*phdrs32.offset(i as isize)).p_flags).as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_uchar; 4]>() as libc::c_ulong
                    as libc::c_uint,
            );
            (*phdrs.offset(i as isize))
                .p_align = byte_get
                .expect(
                    "non-null function pointer",
                )(
                ((*phdrs32.offset(i as isize)).p_align).as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_uchar; 4]>() as libc::c_ulong
                    as libc::c_uint,
            );
            i = i.wrapping_add(1);
            i;
        }
    } else {
        let mut phdrs64: *mut Elf64_External_Phdr = map
            .offset(elf_header.e_phoff as isize) as *mut Elf64_External_Phdr;
        i = 0 as libc::c_int as libc::c_uint;
        while i < elf_header.e_phnum {
            (*phdrs.offset(i as isize))
                .p_type = byte_get
                .expect(
                    "non-null function pointer",
                )(
                ((*phdrs64.offset(i as isize)).p_type).as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_uchar; 4]>() as libc::c_ulong
                    as libc::c_uint,
            );
            (*phdrs.offset(i as isize))
                .p_offset = byte_get
                .expect(
                    "non-null function pointer",
                )(
                ((*phdrs64.offset(i as isize)).p_offset).as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_uchar; 8]>() as libc::c_ulong
                    as libc::c_uint,
            );
            (*phdrs.offset(i as isize))
                .p_vaddr = byte_get
                .expect(
                    "non-null function pointer",
                )(
                ((*phdrs64.offset(i as isize)).p_vaddr).as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_uchar; 8]>() as libc::c_ulong
                    as libc::c_uint,
            );
            (*phdrs.offset(i as isize))
                .p_paddr = byte_get
                .expect(
                    "non-null function pointer",
                )(
                ((*phdrs64.offset(i as isize)).p_paddr).as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_uchar; 8]>() as libc::c_ulong
                    as libc::c_uint,
            );
            (*phdrs.offset(i as isize))
                .p_filesz = byte_get
                .expect(
                    "non-null function pointer",
                )(
                ((*phdrs64.offset(i as isize)).p_filesz).as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_uchar; 8]>() as libc::c_ulong
                    as libc::c_uint,
            );
            (*phdrs.offset(i as isize))
                .p_memsz = byte_get
                .expect(
                    "non-null function pointer",
                )(
                ((*phdrs64.offset(i as isize)).p_memsz).as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_uchar; 8]>() as libc::c_ulong
                    as libc::c_uint,
            );
            (*phdrs.offset(i as isize))
                .p_flags = byte_get
                .expect(
                    "non-null function pointer",
                )(
                ((*phdrs64.offset(i as isize)).p_flags).as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_uchar; 4]>() as libc::c_ulong
                    as libc::c_uint,
            );
            (*phdrs.offset(i as isize))
                .p_align = byte_get
                .expect(
                    "non-null function pointer",
                )(
                ((*phdrs64.offset(i as isize)).p_align).as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_uchar; 8]>() as libc::c_ulong
                    as libc::c_uint,
            );
            i = i.wrapping_add(1);
            i;
        }
    }
    ret = 0 as libc::c_int;
    i = 0 as libc::c_int as libc::c_uint;
    's_163: while i < elf_header.e_phnum {
        if (*phdrs.offset(i as isize)).p_type == 4 as libc::c_int as libc::c_ulong {
            let mut offset: size_t = (*phdrs.offset(i as isize)).p_offset;
            let mut size: size_t = (*phdrs.offset(i as isize)).p_filesz;
            let mut align: size_t = (*phdrs.offset(i as isize)).p_align;
            let mut buf: *mut libc::c_char = map.offset(offset as isize);
            let mut p: *mut libc::c_char = buf;
            while p < buf.offset(size as isize) {
                let mut xnp: *mut Elf_External_Note = p as *mut Elf_External_Note;
                let mut in_0: Elf_Internal_Note = Elf_Internal_Note {
                    namesz: 0,
                    descsz: 0,
                    type_0: 0,
                    namedata: 0 as *mut libc::c_char,
                    descdata: 0 as *mut libc::c_char,
                    descpos: 0,
                };
                if 12 as libc::c_ulong
                    > (buf.offset_from(p) as libc::c_long as libc::c_ulong)
                        .wrapping_add(size)
                {
                    ret = 1 as libc::c_int;
                    break 's_163;
                } else {
                    in_0
                        .type_0 = byte_get
                        .expect(
                            "non-null function pointer",
                        )(
                        ((*xnp).type_0).as_mut_ptr(),
                        ::core::mem::size_of::<[libc::c_uchar; 4]>() as libc::c_ulong
                            as libc::c_uint,
                    );
                    in_0
                        .namesz = byte_get
                        .expect(
                            "non-null function pointer",
                        )(
                        ((*xnp).namesz).as_mut_ptr(),
                        ::core::mem::size_of::<[libc::c_uchar; 4]>() as libc::c_ulong
                            as libc::c_uint,
                    );
                    in_0.namedata = ((*xnp).name).as_mut_ptr();
                    if in_0.namesz
                        > (buf.offset_from(in_0.namedata) as libc::c_long
                            as libc::c_ulong)
                            .wrapping_add(size)
                    {
                        ret = 1 as libc::c_int;
                        break 's_163;
                    } else {
                        in_0
                            .descsz = byte_get
                            .expect(
                                "non-null function pointer",
                            )(
                            ((*xnp).descsz).as_mut_ptr(),
                            ::core::mem::size_of::<[libc::c_uchar; 4]>() as libc::c_ulong
                                as libc::c_uint,
                        );
                        in_0
                            .descdata = p
                            .offset(
                                ((12 as libc::c_ulong)
                                    .wrapping_add(in_0.namesz)
                                    .wrapping_add(
                                        align.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                    ) & !align.wrapping_sub(1 as libc::c_int as libc::c_ulong))
                                    as isize,
                            );
                        in_0
                            .descpos = offset
                            .wrapping_add(
                                (in_0.descdata).offset_from(buf) as libc::c_long
                                    as libc::c_ulong,
                            );
                        if in_0.descsz != 0 as libc::c_int as libc::c_ulong
                            && (in_0.descdata >= buf.offset(size as isize)
                                || in_0.descsz
                                    > (buf.offset_from(in_0.descdata) as libc::c_long
                                        as libc::c_ulong)
                                        .wrapping_add(size))
                        {
                            ret = 1 as libc::c_int;
                            break 's_163;
                        } else {
                            if in_0.namesz
                                == ::core::mem::size_of::<[libc::c_char; 4]>()
                                    as libc::c_ulong
                                && strcmp(
                                    in_0.namedata,
                                    b"GNU\0" as *const u8 as *const libc::c_char,
                                ) == 0 as libc::c_int
                                && in_0.type_0 == 5 as libc::c_int as libc::c_ulong
                            {
                                let mut ptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                                let mut ptr_end: *mut libc::c_uchar = 0
                                    as *mut libc::c_uchar;
                                if in_0.descsz < 8 as libc::c_int as libc::c_ulong
                                    || (in_0.descsz).wrapping_rem(align)
                                        != 0 as libc::c_int as libc::c_ulong
                                {
                                    ret = 1 as libc::c_int;
                                    break 's_163;
                                } else {
                                    ptr = in_0.descdata as *mut libc::c_uchar;
                                    ptr_end = ptr.offset(in_0.descsz as isize);
                                    loop {
                                        let mut type_0: libc::c_uint = byte_get
                                            .expect(
                                                "non-null function pointer",
                                            )(ptr, 4 as libc::c_int as libc::c_uint) as libc::c_uint;
                                        let mut datasz: libc::c_uint = byte_get
                                            .expect(
                                                "non-null function pointer",
                                            )(
                                            ptr.offset(4 as libc::c_int as isize),
                                            4 as libc::c_int as libc::c_uint,
                                        ) as libc::c_uint;
                                        let mut bitmask: libc::c_uint = 0;
                                        let mut old_bitmask: libc::c_uint = 0;
                                        ptr = ptr.offset(8 as libc::c_int as isize);
                                        if ptr.offset(datasz as isize) > ptr_end {
                                            ret = 1 as libc::c_int;
                                            break 's_163;
                                        } else if type_0
                                            == (0xc0000002 as libc::c_uint)
                                                .wrapping_add(0 as libc::c_int as libc::c_uint)
                                        {
                                            if datasz != 4 as libc::c_int as libc::c_uint {
                                                ret = 1 as libc::c_int;
                                                break 's_163;
                                            } else {
                                                old_bitmask = byte_get
                                                    .expect(
                                                        "non-null function pointer",
                                                    )(ptr, 4 as libc::c_int as libc::c_uint) as libc::c_uint;
                                                bitmask = old_bitmask;
                                                if enable_x86_features != 0 {
                                                    bitmask |= enable_x86_features;
                                                }
                                                if disable_x86_features != 0 {
                                                    bitmask &= !disable_x86_features;
                                                }
                                                if old_bitmask != bitmask {
                                                    byte_put
                                                        .expect(
                                                            "non-null function pointer",
                                                        )(
                                                        ptr,
                                                        bitmask as elf_vma,
                                                        4 as libc::c_int as libc::c_uint,
                                                    );
                                                }
                                                break 's_163;
                                            }
                                        } else {
                                            ptr = ptr
                                                .offset(
                                                    ((datasz as bfd_vma)
                                                        .wrapping_add(
                                                            align.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                                        ) & !align.wrapping_sub(1 as libc::c_int as libc::c_ulong))
                                                        as isize,
                                                );
                                            if !(ptr_end.offset_from(ptr) as libc::c_long
                                                >= 8 as libc::c_int as libc::c_long)
                                            {
                                                break;
                                            }
                                        }
                                    }
                                }
                            }
                            p = p
                                .offset(
                                    (((12 as libc::c_ulong)
                                        .wrapping_add(in_0.namesz)
                                        .wrapping_add(
                                            align.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                        ) & !align.wrapping_sub(1 as libc::c_int as libc::c_ulong))
                                        .wrapping_add(in_0.descsz)
                                        .wrapping_add(
                                            align.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                        ) & !align.wrapping_sub(1 as libc::c_int as libc::c_ulong))
                                        as isize,
                                );
                        }
                    }
                }
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    if ret != 0 as libc::c_int {
        error(
            dcgettext(
                0 as *const libc::c_char,
                b"%s: Invalid PT_NOTE segment\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            file_name,
        );
    }
    free(phdrs as *mut libc::c_void);
    munmap(map as *mut libc::c_void, st_buf.st_size as size_t);
    return ret;
}
unsafe extern "C" fn elf_x86_feature(
    mut feature: *const libc::c_char,
    mut enable: libc::c_int,
) -> libc::c_int {
    let mut x86_feature: libc::c_uint = 0;
    if strcasecmp(feature, b"ibt\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        x86_feature = (1 as libc::c_uint) << 0 as libc::c_int;
    } else if strcasecmp(feature, b"shstk\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        x86_feature = (1 as libc::c_uint) << 1 as libc::c_int;
    } else if strcasecmp(feature, b"lam_u48\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        x86_feature = (1 as libc::c_uint) << 2 as libc::c_int;
    } else if strcasecmp(feature, b"lam_u57\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        x86_feature = (1 as libc::c_uint) << 3 as libc::c_int;
    } else {
        error(
            dcgettext(
                0 as *const libc::c_char,
                b"Unknown x86 feature: %s\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            feature,
        );
        return -(1 as libc::c_int);
    }
    if enable != 0 {
        enable_x86_features |= x86_feature;
        disable_x86_features &= !x86_feature;
    } else {
        disable_x86_features |= x86_feature;
        enable_x86_features &= !x86_feature;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn elf_class(mut mach: libc::c_int) -> elfclass {
    match mach {
        3 | 6 => return ELF_CLASS_32,
        180 | 181 => return ELF_CLASS_64,
        62 | 0 => return ELF_CLASS_BOTH,
        _ => return ELF_CLASS_BOTH,
    };
}
unsafe extern "C" fn update_elf_header(
    mut file_name: *const libc::c_char,
    mut file: *mut FILE,
) -> libc::c_int {
    let mut class: libc::c_int = 0;
    let mut machine: libc::c_int = 0;
    let mut type_0: libc::c_int = 0;
    let mut status: libc::c_int = 0;
    let mut osabi: libc::c_int = 0;
    if elf_header.e_ident[6 as libc::c_int as usize] as libc::c_int != 1 as libc::c_int {
        error(
            dcgettext(
                0 as *const libc::c_char,
                b"%s: Unsupported EI_VERSION: %d is not %d\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            file_name,
            elf_header.e_ident[6 as libc::c_int as usize] as libc::c_int,
            1 as libc::c_int,
        );
        return 0 as libc::c_int;
    }
    if output_elf_machine == elf_header.e_machine as libc::c_int {
        return 1 as libc::c_int;
    }
    class = elf_header.e_ident[4 as libc::c_int as usize] as libc::c_int;
    machine = elf_header.e_machine as libc::c_int;
    if input_elf_class as libc::c_int == ELF_CLASS_UNKNOWN as libc::c_int {
        input_elf_class = elf_class(machine);
    }
    if input_elf_class as libc::c_int != ELF_CLASS_BOTH as libc::c_int
        && input_elf_class as libc::c_int != class
    {
        error(
            dcgettext(
                0 as *const libc::c_char,
                b"%s: Unmatched input EI_CLASS: %d is not %d\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            file_name,
            class,
            input_elf_class as libc::c_int,
        );
        return 0 as libc::c_int;
    }
    if output_elf_class as libc::c_int != ELF_CLASS_BOTH as libc::c_int
        && output_elf_class as libc::c_int != class
    {
        error(
            dcgettext(
                0 as *const libc::c_char,
                b"%s: Unmatched output EI_CLASS: %d is not %d\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            file_name,
            class,
            output_elf_class as libc::c_int,
        );
        return 0 as libc::c_int;
    }
    if input_elf_machine != -(1 as libc::c_int) && machine != input_elf_machine {
        error(
            dcgettext(
                0 as *const libc::c_char,
                b"%s: Unmatched e_machine: %d is not %d\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            file_name,
            machine,
            input_elf_machine,
        );
        return 0 as libc::c_int;
    }
    type_0 = elf_header.e_type as libc::c_int;
    if input_elf_type != -(1 as libc::c_int) && type_0 != input_elf_type {
        error(
            dcgettext(
                0 as *const libc::c_char,
                b"%s: Unmatched e_type: %d is not %d\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            file_name,
            type_0,
            input_elf_type,
        );
        return 0 as libc::c_int;
    }
    osabi = elf_header.e_ident[7 as libc::c_int as usize] as libc::c_int;
    if input_elf_osabi != -(1 as libc::c_int) && osabi != input_elf_osabi {
        error(
            dcgettext(
                0 as *const libc::c_char,
                b"%s: Unmatched EI_OSABI: %d is not %d\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            file_name,
            osabi,
            input_elf_osabi,
        );
        return 0 as libc::c_int;
    }
    match class {
        1 => {
            if output_elf_machine != -(1 as libc::c_int) {
                byte_put
                    .expect(
                        "non-null function pointer",
                    )(
                    (ehdr32.e_machine).as_mut_ptr(),
                    output_elf_machine as elf_vma,
                    ::core::mem::size_of::<[libc::c_uchar; 2]>() as libc::c_ulong
                        as libc::c_uint,
                );
            }
            if output_elf_type != -(1 as libc::c_int) {
                byte_put
                    .expect(
                        "non-null function pointer",
                    )(
                    (ehdr32.e_type).as_mut_ptr(),
                    output_elf_type as elf_vma,
                    ::core::mem::size_of::<[libc::c_uchar; 2]>() as libc::c_ulong
                        as libc::c_uint,
                );
            }
            if output_elf_osabi != -(1 as libc::c_int) {
                ehdr32
                    .e_ident[7 as libc::c_int
                    as usize] = output_elf_osabi as libc::c_uchar;
            }
            status = (fwrite(
                &mut ehdr32 as *mut Elf32_External_Ehdr as *const libc::c_void,
                ::core::mem::size_of::<Elf32_External_Ehdr>() as libc::c_ulong,
                1 as libc::c_int as libc::c_ulong,
                file,
            ) == 1 as libc::c_int as libc::c_ulong) as libc::c_int;
        }
        2 => {
            if output_elf_machine != -(1 as libc::c_int) {
                byte_put
                    .expect(
                        "non-null function pointer",
                    )(
                    (ehdr64.e_machine).as_mut_ptr(),
                    output_elf_machine as elf_vma,
                    ::core::mem::size_of::<[libc::c_uchar; 2]>() as libc::c_ulong
                        as libc::c_uint,
                );
            }
            if output_elf_type != -(1 as libc::c_int) {
                byte_put
                    .expect(
                        "non-null function pointer",
                    )(
                    (ehdr64.e_type).as_mut_ptr(),
                    output_elf_type as elf_vma,
                    ::core::mem::size_of::<[libc::c_uchar; 2]>() as libc::c_ulong
                        as libc::c_uint,
                );
            }
            if output_elf_osabi != -(1 as libc::c_int) {
                ehdr64
                    .e_ident[7 as libc::c_int
                    as usize] = output_elf_osabi as libc::c_uchar;
            }
            status = (fwrite(
                &mut ehdr64 as *mut Elf64_External_Ehdr as *const libc::c_void,
                ::core::mem::size_of::<Elf64_External_Ehdr>() as libc::c_ulong,
                1 as libc::c_int as libc::c_ulong,
                file,
            ) == 1 as libc::c_int as libc::c_ulong) as libc::c_int;
        }
        _ => {
            abort();
        }
    }
    if status != 1 as libc::c_int {
        error(
            dcgettext(
                0 as *const libc::c_char,
                b"%s: Failed to update ELF header: %s\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            file_name,
            strerror(*__errno_location()),
        );
    }
    return status;
}
unsafe extern "C" fn get_file_header(mut file: *mut FILE) -> libc::c_int {
    if fread(
        (elf_header.e_ident).as_mut_ptr() as *mut libc::c_void,
        16 as libc::c_int as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        file,
    ) != 1 as libc::c_int as libc::c_ulong
    {
        return 0 as libc::c_int;
    }
    if elf_header.e_ident[0 as libc::c_int as usize] as libc::c_int
        != 0x7f as libc::c_int
        || elf_header.e_ident[1 as libc::c_int as usize] as libc::c_int != 'E' as i32
        || elf_header.e_ident[2 as libc::c_int as usize] as libc::c_int != 'L' as i32
        || elf_header.e_ident[3 as libc::c_int as usize] as libc::c_int != 'F' as i32
    {
        return 0 as libc::c_int;
    }
    let mut current_block_7: u64;
    match elf_header.e_ident[5 as libc::c_int as usize] as libc::c_int {
        0 => {
            current_block_7 = 5843925138985246205;
        }
        2 => {
            byte_get = Some(
                byte_get_big_endian
                    as unsafe extern "C" fn(
                        *const libc::c_uchar,
                        libc::c_uint,
                    ) -> elf_vma,
            );
            byte_put = Some(
                byte_put_big_endian
                    as unsafe extern "C" fn(
                        *mut libc::c_uchar,
                        elf_vma,
                        libc::c_uint,
                    ) -> (),
            );
            current_block_7 = 14523784380283086299;
        }
        1 | _ => {
            current_block_7 = 5843925138985246205;
        }
    }
    match current_block_7 {
        5843925138985246205 => {
            byte_get = Some(
                byte_get_little_endian
                    as unsafe extern "C" fn(
                        *const libc::c_uchar,
                        libc::c_uint,
                    ) -> elf_vma,
            );
            byte_put = Some(
                byte_put_little_endian
                    as unsafe extern "C" fn(
                        *mut libc::c_uchar,
                        elf_vma,
                        libc::c_uint,
                    ) -> (),
            );
        }
        _ => {}
    }
    match elf_header.e_ident[4 as libc::c_int as usize] as libc::c_int {
        1 => {
            if fread(
                (ehdr32.e_type).as_mut_ptr() as *mut libc::c_void,
                (::core::mem::size_of::<Elf32_External_Ehdr>() as libc::c_ulong)
                    .wrapping_sub(16 as libc::c_int as libc::c_ulong),
                1 as libc::c_int as libc::c_ulong,
                file,
            ) != 1 as libc::c_int as libc::c_ulong
            {
                return 0 as libc::c_int;
            }
            elf_header
                .e_type = byte_get
                .expect(
                    "non-null function pointer",
                )(
                (ehdr32.e_type).as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_uchar; 2]>() as libc::c_ulong
                    as libc::c_uint,
            ) as libc::c_ushort;
            elf_header
                .e_machine = byte_get
                .expect(
                    "non-null function pointer",
                )(
                (ehdr32.e_machine).as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_uchar; 2]>() as libc::c_ulong
                    as libc::c_uint,
            ) as libc::c_ushort;
            elf_header
                .e_version = byte_get
                .expect(
                    "non-null function pointer",
                )(
                (ehdr32.e_version).as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_uchar; 4]>() as libc::c_ulong
                    as libc::c_uint,
            );
            elf_header
                .e_entry = byte_get
                .expect(
                    "non-null function pointer",
                )(
                (ehdr32.e_entry).as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_uchar; 4]>() as libc::c_ulong
                    as libc::c_uint,
            );
            elf_header
                .e_phoff = byte_get
                .expect(
                    "non-null function pointer",
                )(
                (ehdr32.e_phoff).as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_uchar; 4]>() as libc::c_ulong
                    as libc::c_uint,
            );
            elf_header
                .e_shoff = byte_get
                .expect(
                    "non-null function pointer",
                )(
                (ehdr32.e_shoff).as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_uchar; 4]>() as libc::c_ulong
                    as libc::c_uint,
            );
            elf_header
                .e_flags = byte_get
                .expect(
                    "non-null function pointer",
                )(
                (ehdr32.e_flags).as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_uchar; 4]>() as libc::c_ulong
                    as libc::c_uint,
            );
            elf_header
                .e_ehsize = byte_get
                .expect(
                    "non-null function pointer",
                )(
                (ehdr32.e_ehsize).as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_uchar; 2]>() as libc::c_ulong
                    as libc::c_uint,
            ) as libc::c_uint;
            elf_header
                .e_phentsize = byte_get
                .expect(
                    "non-null function pointer",
                )(
                (ehdr32.e_phentsize).as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_uchar; 2]>() as libc::c_ulong
                    as libc::c_uint,
            ) as libc::c_uint;
            elf_header
                .e_phnum = byte_get
                .expect(
                    "non-null function pointer",
                )(
                (ehdr32.e_phnum).as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_uchar; 2]>() as libc::c_ulong
                    as libc::c_uint,
            ) as libc::c_uint;
            elf_header
                .e_shentsize = byte_get
                .expect(
                    "non-null function pointer",
                )(
                (ehdr32.e_shentsize).as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_uchar; 2]>() as libc::c_ulong
                    as libc::c_uint,
            ) as libc::c_uint;
            elf_header
                .e_shnum = byte_get
                .expect(
                    "non-null function pointer",
                )(
                (ehdr32.e_shnum).as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_uchar; 2]>() as libc::c_ulong
                    as libc::c_uint,
            ) as libc::c_uint;
            elf_header
                .e_shstrndx = byte_get
                .expect(
                    "non-null function pointer",
                )(
                (ehdr32.e_shstrndx).as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_uchar; 2]>() as libc::c_ulong
                    as libc::c_uint,
            ) as libc::c_uint;
            memcpy(
                &mut ehdr32 as *mut Elf32_External_Ehdr as *mut libc::c_void,
                &mut elf_header as *mut Elf_Internal_Ehdr as *const libc::c_void,
                16 as libc::c_int as libc::c_ulong,
            );
        }
        2 => {
            if (::core::mem::size_of::<bfd_vma>() as libc::c_ulong)
                < 8 as libc::c_int as libc::c_ulong
            {
                error(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"This executable has been built without support for a\n64 bit data type and so it cannot process 64 bit ELF files.\n\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                return 0 as libc::c_int;
            }
            if fread(
                (ehdr64.e_type).as_mut_ptr() as *mut libc::c_void,
                (::core::mem::size_of::<Elf64_External_Ehdr>() as libc::c_ulong)
                    .wrapping_sub(16 as libc::c_int as libc::c_ulong),
                1 as libc::c_int as libc::c_ulong,
                file,
            ) != 1 as libc::c_int as libc::c_ulong
            {
                return 0 as libc::c_int;
            }
            elf_header
                .e_type = byte_get
                .expect(
                    "non-null function pointer",
                )(
                (ehdr64.e_type).as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_uchar; 2]>() as libc::c_ulong
                    as libc::c_uint,
            ) as libc::c_ushort;
            elf_header
                .e_machine = byte_get
                .expect(
                    "non-null function pointer",
                )(
                (ehdr64.e_machine).as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_uchar; 2]>() as libc::c_ulong
                    as libc::c_uint,
            ) as libc::c_ushort;
            elf_header
                .e_version = byte_get
                .expect(
                    "non-null function pointer",
                )(
                (ehdr64.e_version).as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_uchar; 4]>() as libc::c_ulong
                    as libc::c_uint,
            );
            elf_header
                .e_entry = byte_get
                .expect(
                    "non-null function pointer",
                )(
                (ehdr64.e_entry).as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_uchar; 8]>() as libc::c_ulong
                    as libc::c_uint,
            );
            elf_header
                .e_phoff = byte_get
                .expect(
                    "non-null function pointer",
                )(
                (ehdr64.e_phoff).as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_uchar; 8]>() as libc::c_ulong
                    as libc::c_uint,
            );
            elf_header
                .e_shoff = byte_get
                .expect(
                    "non-null function pointer",
                )(
                (ehdr64.e_shoff).as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_uchar; 8]>() as libc::c_ulong
                    as libc::c_uint,
            );
            elf_header
                .e_flags = byte_get
                .expect(
                    "non-null function pointer",
                )(
                (ehdr64.e_flags).as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_uchar; 4]>() as libc::c_ulong
                    as libc::c_uint,
            );
            elf_header
                .e_ehsize = byte_get
                .expect(
                    "non-null function pointer",
                )(
                (ehdr64.e_ehsize).as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_uchar; 2]>() as libc::c_ulong
                    as libc::c_uint,
            ) as libc::c_uint;
            elf_header
                .e_phentsize = byte_get
                .expect(
                    "non-null function pointer",
                )(
                (ehdr64.e_phentsize).as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_uchar; 2]>() as libc::c_ulong
                    as libc::c_uint,
            ) as libc::c_uint;
            elf_header
                .e_phnum = byte_get
                .expect(
                    "non-null function pointer",
                )(
                (ehdr64.e_phnum).as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_uchar; 2]>() as libc::c_ulong
                    as libc::c_uint,
            ) as libc::c_uint;
            elf_header
                .e_shentsize = byte_get
                .expect(
                    "non-null function pointer",
                )(
                (ehdr64.e_shentsize).as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_uchar; 2]>() as libc::c_ulong
                    as libc::c_uint,
            ) as libc::c_uint;
            elf_header
                .e_shnum = byte_get
                .expect(
                    "non-null function pointer",
                )(
                (ehdr64.e_shnum).as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_uchar; 2]>() as libc::c_ulong
                    as libc::c_uint,
            ) as libc::c_uint;
            elf_header
                .e_shstrndx = byte_get
                .expect(
                    "non-null function pointer",
                )(
                (ehdr64.e_shstrndx).as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_uchar; 2]>() as libc::c_ulong
                    as libc::c_uint,
            ) as libc::c_uint;
            memcpy(
                &mut ehdr64 as *mut Elf64_External_Ehdr as *mut libc::c_void,
                &mut elf_header as *mut Elf_Internal_Ehdr as *const libc::c_void,
                16 as libc::c_int as libc::c_ulong,
            );
        }
        _ => return 0 as libc::c_int,
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn process_object(
    mut file_name: *const libc::c_char,
    mut file: *mut FILE,
) -> libc::c_int {
    let mut offset: libc::c_long = ftell(file);
    if get_file_header(file) == 0 {
        error(
            dcgettext(
                0 as *const libc::c_char,
                b"%s: Failed to read ELF header\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            file_name,
        );
        return 1 as libc::c_int;
    }
    if fseek(file, offset, 0 as libc::c_int) != 0 as libc::c_int {
        error(
            dcgettext(
                0 as *const libc::c_char,
                b"%s: Failed to seek to ELF header\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            file_name,
        );
    }
    if update_elf_header(file_name, file) == 0 {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn process_archive(
    mut file_name: *const libc::c_char,
    mut file: *mut FILE,
    mut is_thin_archive: bool,
) -> libc::c_int {
    let mut arch: archive_info = archive_info {
        file_name: 0 as *mut libc::c_char,
        file: 0 as *mut FILE,
        index_num: 0,
        index_array: 0 as *mut elf_vma,
        sym_table: 0 as *mut libc::c_char,
        sym_size: 0,
        longnames: 0 as *mut libc::c_char,
        longnames_size: 0,
        nested_member_origin: 0,
        next_arhdr_offset: 0,
        is_thin_archive: 0,
        uses_64bit_indices: 0,
        arhdr: ar_hdr {
            ar_name: [0; 16],
            ar_date: [0; 12],
            ar_uid: [0; 6],
            ar_gid: [0; 6],
            ar_mode: [0; 8],
            ar_size: [0; 10],
            ar_fmag: [0; 2],
        },
    };
    let mut nested_arch: archive_info = archive_info {
        file_name: 0 as *mut libc::c_char,
        file: 0 as *mut FILE,
        index_num: 0,
        index_array: 0 as *mut elf_vma,
        sym_table: 0 as *mut libc::c_char,
        sym_size: 0,
        longnames: 0 as *mut libc::c_char,
        longnames_size: 0,
        nested_member_origin: 0,
        next_arhdr_offset: 0,
        is_thin_archive: 0,
        uses_64bit_indices: 0,
        arhdr: ar_hdr {
            ar_name: [0; 16],
            ar_date: [0; 12],
            ar_uid: [0; 6],
            ar_gid: [0; 6],
            ar_mode: [0; 8],
            ar_size: [0; 10],
            ar_fmag: [0; 2],
        },
    };
    let mut got: size_t = 0;
    let mut ret: libc::c_int = 0;
    let mut statbuf: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    arch.file_name = 0 as *mut libc::c_char;
    arch.file = 0 as *mut FILE;
    arch.index_array = 0 as *mut elf_vma;
    arch.sym_table = 0 as *mut libc::c_char;
    arch.longnames = 0 as *mut libc::c_char;
    nested_arch.file_name = 0 as *mut libc::c_char;
    nested_arch.file = 0 as *mut FILE;
    nested_arch.index_array = 0 as *mut elf_vma;
    nested_arch.sym_table = 0 as *mut libc::c_char;
    nested_arch.longnames = 0 as *mut libc::c_char;
    if fstat(fileno(file), &mut statbuf) < 0 as libc::c_int
        || setup_archive(
            &mut arch,
            file_name,
            file,
            statbuf.st_size,
            is_thin_archive as libc::c_int,
            0 as libc::c_int,
        ) != 0 as libc::c_int
    {
        ret = 1 as libc::c_int;
    } else {
        ret = 0 as libc::c_int;
        loop {
            let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut namelen: size_t = 0;
            let mut qualified_name: *mut libc::c_char = 0 as *mut libc::c_char;
            if fseek(file, arch.next_arhdr_offset as libc::c_long, 0 as libc::c_int)
                != 0 as libc::c_int
            {
                error(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s: failed to seek to next archive header\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    file_name,
                );
                return 1 as libc::c_int;
            }
            got = fread(
                &mut arch.arhdr as *mut ar_hdr as *mut libc::c_void,
                1 as libc::c_int as libc::c_ulong,
                ::core::mem::size_of::<ar_hdr>() as libc::c_ulong,
                file,
            );
            if got != ::core::mem::size_of::<ar_hdr>() as libc::c_ulong {
                if got == 0 as libc::c_int as libc::c_ulong {
                    break;
                }
                error(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s: failed to read archive header\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    file_name,
                );
                ret = 1 as libc::c_int;
                break;
            } else if memcmp(
                (arch.arhdr.ar_fmag).as_mut_ptr() as *const libc::c_void,
                b"`\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                2 as libc::c_int as libc::c_ulong,
            ) != 0 as libc::c_int
            {
                error(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s: did not find a valid archive header\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    arch.file_name,
                );
                ret = 1 as libc::c_int;
                break;
            } else {
                arch
                    .next_arhdr_offset = (arch.next_arhdr_offset)
                    .wrapping_add(::core::mem::size_of::<ar_hdr>() as libc::c_ulong);
                archive_file_size = strtoul(
                    (arch.arhdr.ar_size).as_mut_ptr(),
                    0 as *mut *mut libc::c_char,
                    10 as libc::c_int,
                );
                if archive_file_size & 0o1 as libc::c_int as libc::c_ulong != 0 {
                    archive_file_size = archive_file_size.wrapping_add(1);
                    archive_file_size;
                }
                name = get_archive_member_name(&mut arch, &mut nested_arch);
                if name.is_null() {
                    error(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"%s: bad archive file name\n\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        file_name,
                    );
                    ret = 1 as libc::c_int;
                    break;
                } else {
                    namelen = strlen(name);
                    qualified_name = make_qualified_name(
                        &mut arch,
                        &mut nested_arch,
                        name,
                    );
                    if qualified_name.is_null() {
                        error(
                            dcgettext(
                                0 as *const libc::c_char,
                                b"%s: bad archive file name\n\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            file_name,
                        );
                        free(name as *mut libc::c_void);
                        ret = 1 as libc::c_int;
                        break;
                    } else {
                        if is_thin_archive as libc::c_int != 0
                            && arch.nested_member_origin
                                == 0 as libc::c_int as libc::c_ulong
                        {
                            let mut member_file: *mut FILE = 0 as *mut FILE;
                            let mut member_file_name: *mut libc::c_char = adjust_relative_path(
                                file_name,
                                name,
                                namelen,
                            );
                            free(name as *mut libc::c_void);
                            if member_file_name.is_null() {
                                free(qualified_name as *mut libc::c_void);
                                ret = 1 as libc::c_int;
                                break;
                            } else {
                                member_file = fopen(
                                    member_file_name,
                                    b"r+b\0" as *const u8 as *const libc::c_char,
                                );
                                if member_file.is_null() {
                                    error(
                                        dcgettext(
                                            0 as *const libc::c_char,
                                            b"Input file '%s' is not readable\n\0" as *const u8
                                                as *const libc::c_char,
                                            5 as libc::c_int,
                                        ),
                                        member_file_name,
                                    );
                                    free(member_file_name as *mut libc::c_void);
                                    free(qualified_name as *mut libc::c_void);
                                    ret = 1 as libc::c_int;
                                    break;
                                } else {
                                    archive_file_offset = arch.nested_member_origin
                                        as libc::c_long;
                                    ret |= process_object(qualified_name, member_file);
                                    fclose(member_file);
                                    free(member_file_name as *mut libc::c_void);
                                }
                            }
                        } else if is_thin_archive {
                            free(name as *mut libc::c_void);
                            archive_file_offset = (arch.nested_member_origin)
                                .wrapping_add(
                                    ::core::mem::size_of::<ar_hdr>() as libc::c_ulong,
                                ) as libc::c_long;
                            if fseek(
                                nested_arch.file,
                                archive_file_offset,
                                0 as libc::c_int,
                            ) != 0 as libc::c_int
                            {
                                error(
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"%s: failed to seek to archive member\n\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                    nested_arch.file_name,
                                );
                                free(qualified_name as *mut libc::c_void);
                                ret = 1 as libc::c_int;
                                break;
                            } else {
                                ret |= process_object(qualified_name, nested_arch.file);
                            }
                        } else {
                            free(name as *mut libc::c_void);
                            archive_file_offset = arch.next_arhdr_offset as libc::c_long;
                            arch
                                .next_arhdr_offset = (arch.next_arhdr_offset)
                                .wrapping_add(archive_file_size);
                            ret |= process_object(qualified_name, file);
                        }
                        free(qualified_name as *mut libc::c_void);
                    }
                }
            }
        }
    }
    if !(nested_arch.file).is_null() {
        fclose(nested_arch.file);
    }
    release_archive(&mut nested_arch);
    release_archive(&mut arch);
    return ret;
}
unsafe extern "C" fn check_file(
    mut file_name: *const libc::c_char,
    mut statbuf_p: *mut stat,
) -> libc::c_int {
    let mut statbuf: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    if statbuf_p.is_null() {
        statbuf_p = &mut statbuf;
    }
    if stat(file_name, statbuf_p) < 0 as libc::c_int {
        if *__errno_location() == 2 as libc::c_int {
            error(
                dcgettext(
                    0 as *const libc::c_char,
                    b"'%s': No such file\n\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                file_name,
            );
        } else {
            error(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Could not locate '%s'.  System error message: %s\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                file_name,
                strerror(*__errno_location()),
            );
        }
        return 1 as libc::c_int;
    }
    if !((*statbuf_p).st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o100000 as libc::c_int as libc::c_uint)
    {
        error(
            dcgettext(
                0 as *const libc::c_char,
                b"'%s' is not an ordinary file\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            file_name,
        );
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn process_file(mut file_name: *const libc::c_char) -> libc::c_int {
    let mut file: *mut FILE = 0 as *mut FILE;
    let mut armag: [libc::c_char; 8] = [0; 8];
    let mut ret: libc::c_int = 0;
    if check_file(file_name, 0 as *mut stat) != 0 {
        return 1 as libc::c_int;
    }
    file = fopen(file_name, b"r+b\0" as *const u8 as *const libc::c_char);
    if file.is_null() {
        error(
            dcgettext(
                0 as *const libc::c_char,
                b"Input file '%s' is not readable\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            file_name,
        );
        return 1 as libc::c_int;
    }
    if fread(
        armag.as_mut_ptr() as *mut libc::c_void,
        8 as libc::c_int as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        file,
    ) != 1 as libc::c_int as libc::c_ulong
    {
        error(
            dcgettext(
                0 as *const libc::c_char,
                b"%s: Failed to read file's magic number\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            file_name,
        );
        fclose(file);
        return 1 as libc::c_int;
    }
    if memcmp(
        armag.as_mut_ptr() as *const libc::c_void,
        b"!<arch>\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        8 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int
    {
        ret = process_archive(file_name, file, 0 as libc::c_int != 0);
    } else if memcmp(
        armag.as_mut_ptr() as *const libc::c_void,
        b"!<thin>\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        8 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int
    {
        ret = process_archive(file_name, file, 1 as libc::c_int != 0);
    } else {
        rewind(file);
        archive_file_offset = 0 as libc::c_int as libc::c_long;
        archive_file_size = archive_file_offset as libc::c_ulong;
        ret = process_object(file_name, file);
        if ret == 0
            && (elf_header.e_type as libc::c_int == 2 as libc::c_int
                || elf_header.e_type as libc::c_int == 3 as libc::c_int)
        {
            ret = update_gnu_property(file_name, file);
        }
    }
    fclose(file);
    return ret;
}
static mut osabis: [C2RustUnnamed; 16] = [
    {
        let mut init = C2RustUnnamed {
            osabi: 0 as libc::c_int,
            name: b"none\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            osabi: 1 as libc::c_int,
            name: b"HPUX\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            osabi: 2 as libc::c_int,
            name: b"NetBSD\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            osabi: 3 as libc::c_int,
            name: b"GNU\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            osabi: 3 as libc::c_int,
            name: b"Linux\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            osabi: 6 as libc::c_int,
            name: b"Solaris\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            osabi: 7 as libc::c_int,
            name: b"AIX\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            osabi: 8 as libc::c_int,
            name: b"Irix\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            osabi: 9 as libc::c_int,
            name: b"FreeBSD\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            osabi: 10 as libc::c_int,
            name: b"TRU64\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            osabi: 11 as libc::c_int,
            name: b"Modesto\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            osabi: 12 as libc::c_int,
            name: b"OpenBSD\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            osabi: 13 as libc::c_int,
            name: b"OpenVMS\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            osabi: 14 as libc::c_int,
            name: b"NSK\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            osabi: 15 as libc::c_int,
            name: b"AROS\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            osabi: 16 as libc::c_int,
            name: b"FenixOS\0" as *const u8 as *const libc::c_char,
        };
        init
    },
];
unsafe extern "C" fn elf_osabi(mut osabi: *const libc::c_char) -> libc::c_int {
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong)
        < (::core::mem::size_of::<[C2RustUnnamed; 16]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<C2RustUnnamed>() as libc::c_ulong)
    {
        if strcasecmp(osabi, osabis[i as usize].name) == 0 as libc::c_int {
            return osabis[i as usize].osabi;
        }
        i = i.wrapping_add(1);
        i;
    }
    error(
        dcgettext(
            0 as *const libc::c_char,
            b"Unknown OSABI: %s\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        osabi,
    );
    return -(1 as libc::c_int);
}
unsafe extern "C" fn elf_machine(mut mach: *const libc::c_char) -> libc::c_int {
    if strcasecmp(mach, b"i386\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        return 3 as libc::c_int;
    }
    if strcasecmp(mach, b"iamcu\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        return 6 as libc::c_int;
    }
    if strcasecmp(mach, b"l1om\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        return 180 as libc::c_int;
    }
    if strcasecmp(mach, b"k1om\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        return 181 as libc::c_int;
    }
    if strcasecmp(mach, b"x86_64\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        return 62 as libc::c_int;
    }
    if strcasecmp(mach, b"x86-64\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        return 62 as libc::c_int;
    }
    if strcasecmp(mach, b"none\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    error(
        dcgettext(
            0 as *const libc::c_char,
            b"Unknown machine type: %s\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        mach,
    );
    return -(1 as libc::c_int);
}
unsafe extern "C" fn elf_type(mut type_0: *const libc::c_char) -> libc::c_int {
    if strcasecmp(type_0, b"rel\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    if strcasecmp(type_0, b"exec\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        return 2 as libc::c_int;
    }
    if strcasecmp(type_0, b"dyn\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        return 3 as libc::c_int;
    }
    if strcasecmp(type_0, b"none\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    error(
        dcgettext(
            0 as *const libc::c_char,
            b"Unknown type: %s\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        type_0,
    );
    return -(1 as libc::c_int);
}
static mut options: [option; 11] = [
    {
        let mut init = option {
            name: b"input-mach\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPTION_INPUT_MACH as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"output-mach\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPTION_OUTPUT_MACH as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"input-type\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPTION_INPUT_TYPE as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"output-type\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPTION_OUTPUT_TYPE as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"input-osabi\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPTION_INPUT_OSABI as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"output-osabi\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPTION_OUTPUT_OSABI as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"enable-x86-feature\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPTION_ENABLE_X86_FEATURE as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"disable-x86-feature\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPTION_DISABLE_X86_FEATURE as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"version\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'v' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"help\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'h' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: 0 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
];
unsafe extern "C" fn usage(mut stream: *mut FILE, mut exit_status: libc::c_int) -> ! {
    let mut i: libc::c_uint = 0;
    let mut osabi: *mut libc::c_char = concat(
        osabis[0 as libc::c_int as usize].name,
        0 as *mut libc::c_void,
    );
    i = 1 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong)
        < (::core::mem::size_of::<[C2RustUnnamed; 16]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<C2RustUnnamed>() as libc::c_ulong)
    {
        osabi = reconcat(
            osabi,
            osabi,
            b"|\0" as *const u8 as *const libc::c_char,
            osabis[i as usize].name,
            0 as *mut libc::c_void,
        );
        i = i.wrapping_add(1);
        i;
    }
    fprintf(
        stream,
        dcgettext(
            0 as *const libc::c_char,
            b"Usage: %s <option(s)> elffile(s)\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        program_name,
    );
    fprintf(
        stream,
        dcgettext(
            0 as *const libc::c_char,
            b" Update the ELF header of ELF files\n\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    fprintf(
        stream,
        dcgettext(
            0 as *const libc::c_char,
            b" The options are:\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    fprintf(
        stream,
        dcgettext(
            0 as *const libc::c_char,
            b"  --input-mach [none|i386|iamcu|l1om|k1om|x86_64]\n                              Set input machine type\n  --output-mach [none|i386|iamcu|l1om|k1om|x86_64]\n                              Set output machine type\n  --input-type [none|rel|exec|dyn]\n                              Set input file type\n  --output-type [none|rel|exec|dyn]\n                              Set output file type\n  --input-osabi [%s]\n                              Set input OSABI\n  --output-osabi [%s]\n                              Set output OSABI\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        osabi,
        osabi,
    );
    fprintf(
        stream,
        dcgettext(
            0 as *const libc::c_char,
            b"  --enable-x86-feature [ibt|shstk|lam_u48|lam_u57]\n                              Enable x86 feature\n  --disable-x86-feature [ibt|shstk|lam_u48|lam_u57]\n                              Disable x86 feature\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    fprintf(
        stream,
        dcgettext(
            0 as *const libc::c_char,
            b"  -h --help                   Display this information\n  -v --version                Display the version number of %s\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        program_name,
    );
    if (*::core::mem::transmute::<
        &[u8; 39],
        &[libc::c_char; 39],
    >(b"<https://www.sourceware.org/bugzilla/>\0"))[0 as libc::c_int as usize]
        as libc::c_int != 0 && exit_status == 0 as libc::c_int
    {
        fprintf(
            stream,
            dcgettext(
                0 as *const libc::c_char,
                b"Report bugs to %s\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            b"<https://www.sourceware.org/bugzilla/>\0" as *const u8
                as *const libc::c_char,
        );
    }
    free(osabi as *mut libc::c_void);
    exit(exit_status);
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut c: libc::c_int = 0;
    let mut status: libc::c_int = 0;
    setlocale(5 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
    setlocale(0 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
    bindtextdomain(
        b"binutils\0" as *const u8 as *const libc::c_char,
        b"/usr/local/share/locale\0" as *const u8 as *const libc::c_char,
    );
    textdomain(b"binutils\0" as *const u8 as *const libc::c_char);
    expandargv(&mut argc, &mut argv);
    loop {
        c = getopt_long(
            argc,
            argv,
            b"hv\0" as *const u8 as *const libc::c_char,
            options.as_mut_ptr(),
            0 as *mut libc::c_int,
        );
        if !(c != -(1 as libc::c_int)) {
            break;
        }
        match c {
            150 => {
                input_elf_machine = elf_machine(optarg);
                if input_elf_machine < 0 as libc::c_int {
                    return 1 as libc::c_int;
                }
                input_elf_class = elf_class(input_elf_machine);
                if input_elf_class as libc::c_int == ELF_CLASS_UNKNOWN as libc::c_int {
                    return 1 as libc::c_int;
                }
            }
            151 => {
                output_elf_machine = elf_machine(optarg);
                if output_elf_machine < 0 as libc::c_int {
                    return 1 as libc::c_int;
                }
                output_elf_class = elf_class(output_elf_machine);
                if output_elf_class as libc::c_int == ELF_CLASS_UNKNOWN as libc::c_int {
                    return 1 as libc::c_int;
                }
            }
            152 => {
                input_elf_type = elf_type(optarg);
                if input_elf_type < 0 as libc::c_int {
                    return 1 as libc::c_int;
                }
            }
            153 => {
                output_elf_type = elf_type(optarg);
                if output_elf_type < 0 as libc::c_int {
                    return 1 as libc::c_int;
                }
            }
            154 => {
                input_elf_osabi = elf_osabi(optarg);
                if input_elf_osabi < 0 as libc::c_int {
                    return 1 as libc::c_int;
                }
            }
            155 => {
                output_elf_osabi = elf_osabi(optarg);
                if output_elf_osabi < 0 as libc::c_int {
                    return 1 as libc::c_int;
                }
            }
            156 => {
                if elf_x86_feature(optarg, 1 as libc::c_int) < 0 as libc::c_int {
                    return 1 as libc::c_int;
                }
            }
            157 => {
                if elf_x86_feature(optarg, 0 as libc::c_int) < 0 as libc::c_int {
                    return 1 as libc::c_int;
                }
            }
            104 => {
                usage(stdout, 0 as libc::c_int);
            }
            118 => {
                print_version(program_name);
            }
            _ => {
                usage(stderr, 1 as libc::c_int);
            }
        }
    }
    if optind == argc
        || output_elf_machine == -(1 as libc::c_int) && enable_x86_features == 0
            && disable_x86_features == 0 && output_elf_type == -(1 as libc::c_int)
            && output_elf_osabi == -(1 as libc::c_int)
    {
        usage(stderr, 1 as libc::c_int);
    }
    status = 0 as libc::c_int;
    while optind < argc {
        let fresh0 = optind;
        optind = optind + 1;
        status |= process_file(*argv.offset(fresh0 as isize));
    }
    return status;
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}
