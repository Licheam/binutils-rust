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

#[macro_use]
extern crate c2rust_bitfields;
extern crate libc;
pub mod src {
pub mod bucomm;
pub mod debug;
pub mod filemode;
pub mod rdcoff;
pub mod rddbg;
pub mod rename;
pub mod stabs;
pub mod version;
pub mod wrstabs;
} // mod src
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type bfd_sym_data_struct;
    pub type bfd_pef_xlib_data_struct;
    pub type bfd_pef_data_struct;
    pub type plugin_data_struct;
    pub type mach_o_fat_data_struct;
    pub type mach_o_data_struct;
    pub type netbsd_core_struct;
    pub type versados_data_struct;
    pub type cisco_core_struct;
    pub type osf_core_struct;
    pub type lynx_core_struct;
    pub type sgi_core_struct;
    pub type hppabsd_core_struct;
    pub type hpux_core_struct;
    pub type som_data_struct;
    pub type trad_core_struct;
    pub type sco5_core_struct;
    pub type sun_core_struct;
    pub type mmo_data_struct;
    pub type elf_strtab_hash;
    pub type dwarf1_debug;
    pub type got_entry;
    pub type plt_entry;
    pub type tekhex_data_struct;
    pub type ihex_data_struct;
    pub type verilog_data_struct;
    pub type srec_data_struct;
    pub type ecoff_tdata;
    pub type artdata;
    pub type aout_data_struct;
    pub type bfd_iovec;
    pub type _bfd_window_internal;
    pub type bfd_strtab_hash;
    pub type ecoff_debug_swap;
    pub type cie;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn strtoul(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_ulong;
    fn free(_: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn getc(__stream: *mut FILE) -> libc::c_int;
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
    fn feof(__stream: *mut FILE) -> libc::c_int;
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    fn chmod(__file: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    static mut stderr: *mut FILE;
    static mut stdout: *mut FILE;
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
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn dup(__fd: libc::c_int) -> libc::c_int;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn rmdir(__path: *const libc::c_char) -> libc::c_int;
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
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncasecmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn bfd_bread(_: *mut libc::c_void, _: bfd_size_type, _: *mut bfd) -> bfd_size_type;
    fn bfd_bwrite(
        _: *const libc::c_void,
        _: bfd_size_type,
        _: *mut bfd,
    ) -> bfd_size_type;
    fn bfd_seek(_: *mut bfd, _: file_ptr, _: libc::c_int) -> libc::c_int;
    fn bfd_stat(_: *mut bfd, _: *mut stat) -> libc::c_int;
    fn bfd_init() -> libc::c_uint;
    fn bfd_openr(filename: *const libc::c_char, target: *const libc::c_char) -> *mut bfd;
    fn bfd_fdopenw(
        filename: *const libc::c_char,
        target: *const libc::c_char,
        fd: libc::c_int,
    ) -> *mut bfd;
    fn bfd_openw(filename: *const libc::c_char, target: *const libc::c_char) -> *mut bfd;
    fn bfd_close(abfd: *mut bfd) -> bool;
    fn bfd_close_all_done(_: *mut bfd) -> bool;
    fn bfd_create_gnu_debuglink_section(
        abfd: *mut bfd,
        filename: *const libc::c_char,
    ) -> *mut bfd_section;
    fn bfd_fill_in_gnu_debuglink_section(
        abfd: *mut bfd,
        sect: *mut bfd_section,
        filename: *const libc::c_char,
    ) -> bool;
    static mut _bfd_std_section: [asection; 4];
    fn bfd_get_section_by_name(
        abfd: *mut bfd,
        name: *const libc::c_char,
    ) -> *mut asection;
    fn bfd_make_section_anyway_with_flags(
        abfd: *mut bfd,
        name: *const libc::c_char,
        flags: flagword,
    ) -> *mut asection;
    fn bfd_make_section_with_flags(
        _: *mut bfd,
        name: *const libc::c_char,
        flags: flagword,
    ) -> *mut asection;
    fn bfd_map_over_sections(
        abfd: *mut bfd,
        func: Option::<
            unsafe extern "C" fn(*mut bfd, *mut asection, *mut libc::c_void) -> (),
        >,
        obj: *mut libc::c_void,
    );
    fn bfd_set_section_size(sec: *mut asection, val: bfd_size_type) -> bool;
    fn bfd_set_section_contents(
        abfd: *mut bfd,
        section: *mut asection,
        data: *const libc::c_void,
        offset: file_ptr,
        count: bfd_size_type,
    ) -> bool;
    fn bfd_malloc_and_get_section(
        abfd: *mut bfd,
        section: *mut asection,
        buf: *mut *mut bfd_byte,
    ) -> bool;
    fn bfd_scan_arch(string: *const libc::c_char) -> *const bfd_arch_info_type;
    fn bfd_get_arch(abfd: *const bfd) -> bfd_architecture;
    fn bfd_get_mach(abfd: *const bfd) -> libc::c_ulong;
    fn bfd_printable_arch_mach(
        arch: bfd_architecture,
        machine: libc::c_ulong,
    ) -> *const libc::c_char;
    fn bfd_octets_per_byte(abfd: *const bfd, sec: *const asection) -> libc::c_uint;
    fn bfd_is_local_label(abfd: *mut bfd, sym: *mut asymbol) -> bool;
    fn bfd_set_symtab(
        abfd: *mut bfd,
        location: *mut *mut asymbol,
        count: libc::c_uint,
    ) -> bool;
    fn bfd_decode_symclass(symbol: *mut asymbol) -> libc::c_int;
    fn bfd_get_error() -> bfd_error_type;
    fn bfd_set_error(error_tag: bfd_error_type);
    fn bfd_set_error_program_name(_: *const libc::c_char);
    fn bfd_get_reloc_upper_bound(abfd: *mut bfd, sect: *mut asection) -> libc::c_long;
    fn bfd_canonicalize_reloc(
        abfd: *mut bfd,
        sec: *mut asection,
        loc: *mut *mut arelent,
        syms: *mut *mut asymbol,
    ) -> libc::c_long;
    fn bfd_set_file_flags(abfd: *mut bfd, flags: flagword) -> bool;
    fn bfd_set_start_address(abfd: *mut bfd, vma: bfd_vma) -> bool;
    fn bfd_alt_mach_code(abfd: *mut bfd, alternative: libc::c_int) -> bool;
    fn bfd_convert_section_size(
        ibfd: *mut bfd,
        isec: *mut asection,
        obfd: *mut bfd,
        size: bfd_size_type,
    ) -> bfd_size_type;
    fn bfd_convert_section_contents(
        ibfd: *mut bfd,
        isec: *mut asection,
        obfd: *mut bfd,
        ptr: *mut *mut bfd_byte,
        ptr_size: *mut bfd_size_type,
    ) -> bool;
    fn bfd_openr_next_archived_file(archive: *mut bfd, previous: *mut bfd) -> *mut bfd;
    fn bfd_check_format(abfd: *mut bfd, format: bfd_format) -> bool;
    fn bfd_check_format_matches(
        abfd: *mut bfd,
        format: bfd_format,
        matching: *mut *mut *mut libc::c_char,
    ) -> bool;
    fn bfd_set_format(abfd: *mut bfd, format: bfd_format) -> bool;
    fn bfd_get_full_section_contents(
        abfd: *mut bfd,
        section: *mut asection,
        ptr: *mut *mut bfd_byte,
    ) -> bool;
    fn getopt_long(
        argc: libc::c_int,
        argv: *const *mut libc::c_char,
        shortopts: *const libc::c_char,
        longopts: *const option,
        longind: *mut libc::c_int,
    ) -> libc::c_int;
    fn expandargv(_: *mut libc::c_int, _: *mut *mut *mut libc::c_char);
    fn concat(_: *const libc::c_char, _: ...) -> *mut libc::c_char;
    fn unlink_if_ordinary(_: *const libc::c_char) -> libc::c_int;
    fn xmalloc_set_program_name(_: *const libc::c_char);
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    fn xrealloc(_: *mut libc::c_void, _: size_t) -> *mut libc::c_void;
    fn xcalloc(_: size_t, _: size_t) -> *mut libc::c_void;
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn xstrndup(_: *const libc::c_char, _: size_t) -> *mut libc::c_char;
    fn bfd_get_archive_filename(_: *const bfd) -> *const libc::c_char;
    fn bfd_nonfatal(_: *const libc::c_char);
    fn bfd_nonfatal_message(
        _: *const libc::c_char,
        _: *const bfd,
        _: *const asection,
        _: *const libc::c_char,
        _: ...
    );
    fn bfd_fatal(_: *const libc::c_char) -> !;
    fn fatal(_: *const libc::c_char, _: ...) -> !;
    fn non_fatal(_: *const libc::c_char, _: ...);
    fn set_default_bfd_target();
    fn list_matching_formats(_: *mut *mut libc::c_char);
    fn list_supported_targets(_: *const libc::c_char, _: *mut FILE);
    fn display_info() -> libc::c_int;
    fn make_tempname(_: *const libc::c_char, _: *mut libc::c_int) -> *mut libc::c_char;
    fn make_tempdir(_: *const libc::c_char) -> *mut libc::c_char;
    fn parse_vma(_: *const libc::c_char, _: *const libc::c_char) -> bfd_vma;
    fn get_file_size(_: *const libc::c_char) -> off_t;
    fn is_valid_archive_path(_: *const libc::c_char) -> bool;
    static mut program_name: *mut libc::c_char;
    fn print_version(_: *const libc::c_char);
    fn set_times(_: *const libc::c_char, _: *const stat);
    fn smart_rename(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_int,
        _: *mut stat,
        _: bool,
    ) -> libc::c_int;
    fn read_debugging_info(
        _: *mut bfd,
        _: *mut *mut asymbol,
        _: libc::c_long,
        _: bool,
    ) -> *mut libc::c_void;
    fn write_stabs_in_sections_debugging_info(
        _: *mut bfd,
        _: *mut libc::c_void,
        _: *mut *mut bfd_byte,
        _: *mut bfd_size_type,
        _: *mut *mut bfd_byte,
        _: *mut bfd_size_type,
    ) -> bool;
    fn htab_create_alloc(
        _: size_t,
        _: htab_hash,
        _: htab_eq,
        _: htab_del,
        _: htab_alloc,
        _: htab_free,
    ) -> htab_t;
    fn htab_find(_: htab_t, _: *const libc::c_void) -> *mut libc::c_void;
    fn htab_find_slot(
        _: htab_t,
        _: *const libc::c_void,
        _: insert_option,
    ) -> *mut *mut libc::c_void;
    fn htab_traverse(_: htab_t, _: htab_trav, _: *mut libc::c_void);
    fn htab_elements(_: htab_t) -> size_t;
    fn htab_hash_string(_: *const libc::c_void) -> hashval_t;
    fn filename_cmp(s1: *const libc::c_char, s2: *const libc::c_char) -> libc::c_int;
    fn htab_eq_string(_: *const libc::c_void, _: *const libc::c_void) -> libc::c_int;
    fn fnmatch(
        __pattern: *const libc::c_char,
        __string: *const libc::c_char,
        __flags: libc::c_int,
    ) -> libc::c_int;
    static mut is_strip: libc::c_int;
    static mut _bfd_srec_len: libc::c_uint;
    static mut _bfd_srec_forceS3: bool;
    static mut VerilogDataWidth: libc::c_uint;
}
pub type size_t = libc::c_ulong;
pub type __uint32_t = libc::c_uint;
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
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
pub type uint32_t = __uint32_t;
pub type bfd_int64_t = libc::c_long;
pub type bfd_uint64_t = libc::c_ulong;
pub type bfd_hostptr_t = libc::c_ulong;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct bfd {
    pub filename: *const libc::c_char,
    pub xvec: *const bfd_target,
    pub iostream: *mut libc::c_void,
    pub iovec: *const bfd_iovec,
    pub lru_prev: *mut bfd,
    pub lru_next: *mut bfd,
    pub where_0: ufile_ptr,
    pub mtime: libc::c_long,
    pub id: libc::c_uint,
    pub flags: flagword,
    #[bitfield(name = "format", ty = "bfd_format", bits = "0..=2")]
    #[bitfield(name = "direction", ty = "bfd_direction", bits = "3..=4")]
    #[bitfield(name = "cacheable", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "target_defaulted", ty = "libc::c_uint", bits = "6..=6")]
    #[bitfield(name = "opened_once", ty = "libc::c_uint", bits = "7..=7")]
    #[bitfield(name = "mtime_set", ty = "libc::c_uint", bits = "8..=8")]
    #[bitfield(name = "no_export", ty = "libc::c_uint", bits = "9..=9")]
    #[bitfield(name = "output_has_begun", ty = "libc::c_uint", bits = "10..=10")]
    #[bitfield(name = "has_armap", ty = "libc::c_uint", bits = "11..=11")]
    #[bitfield(name = "is_thin_archive", ty = "libc::c_uint", bits = "12..=12")]
    #[bitfield(name = "no_element_cache", ty = "libc::c_uint", bits = "13..=13")]
    #[bitfield(name = "selective_search", ty = "libc::c_uint", bits = "14..=14")]
    #[bitfield(name = "is_linker_output", ty = "libc::c_uint", bits = "15..=15")]
    #[bitfield(name = "is_linker_input", ty = "libc::c_uint", bits = "16..=16")]
    #[bitfield(name = "plugin_format", ty = "bfd_plugin_format", bits = "17..=18")]
    #[bitfield(name = "lto_output", ty = "libc::c_uint", bits = "19..=19")]
    #[bitfield(name = "lto_slim_object", ty = "libc::c_uint", bits = "20..=20")]
    #[bitfield(name = "read_only", ty = "libc::c_uint", bits = "21..=21")]
    pub format_direction_cacheable_target_defaulted_opened_once_mtime_set_no_export_output_has_begun_has_armap_is_thin_archive_no_element_cache_selective_search_is_linker_output_is_linker_input_plugin_format_lto_output_lto_slim_object_read_only: [u8; 3],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 5],
    pub plugin_dummy_bfd: *mut bfd,
    pub origin: ufile_ptr,
    pub proxy_origin: ufile_ptr,
    pub section_htab: bfd_hash_table,
    pub sections: *mut bfd_section,
    pub section_last: *mut bfd_section,
    pub section_count: libc::c_uint,
    pub archive_plugin_fd: libc::c_int,
    pub archive_plugin_fd_open_count: libc::c_uint,
    pub archive_pass: libc::c_int,
    pub start_address: bfd_vma,
    pub outsymbols: *mut *mut bfd_symbol,
    pub symcount: libc::c_uint,
    pub dynsymcount: libc::c_uint,
    pub arch_info: *const bfd_arch_info,
    pub size: ufile_ptr,
    pub arelt_data: *mut libc::c_void,
    pub my_archive: *mut bfd,
    pub archive_next: *mut bfd,
    pub archive_head: *mut bfd,
    pub nested_archives: *mut bfd,
    pub link: C2RustUnnamed_38,
    pub tdata: C2RustUnnamed,
    pub usrdata: *mut libc::c_void,
    pub memory: *mut libc::c_void,
    pub build_id: *const bfd_build_id,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bfd_build_id {
    pub size: bfd_size_type,
    pub data: [bfd_byte; 1],
}
pub type bfd_byte = libc::c_uchar;
pub type bfd_size_type = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub aout_data: *mut aout_data_struct,
    pub aout_ar_data: *mut artdata,
    pub coff_obj_data: *mut coff_tdata,
    pub pe_obj_data: *mut pe_tdata,
    pub xcoff_obj_data: *mut xcoff_tdata,
    pub ecoff_obj_data: *mut ecoff_tdata,
    pub srec_data: *mut srec_data_struct,
    pub verilog_data: *mut verilog_data_struct,
    pub ihex_data: *mut ihex_data_struct,
    pub tekhex_data: *mut tekhex_data_struct,
    pub elf_obj_data: *mut elf_obj_tdata,
    pub mmo_data: *mut mmo_data_struct,
    pub sun_core_data: *mut sun_core_struct,
    pub sco5_core_data: *mut sco5_core_struct,
    pub trad_core_data: *mut trad_core_struct,
    pub som_data: *mut som_data_struct,
    pub hpux_core_data: *mut hpux_core_struct,
    pub hppabsd_core_data: *mut hppabsd_core_struct,
    pub sgi_core_data: *mut sgi_core_struct,
    pub lynx_core_data: *mut lynx_core_struct,
    pub osf_core_data: *mut osf_core_struct,
    pub cisco_core_data: *mut cisco_core_struct,
    pub versados_data: *mut versados_data_struct,
    pub netbsd_core_data: *mut netbsd_core_struct,
    pub mach_o_data: *mut mach_o_data_struct,
    pub mach_o_fat_data: *mut mach_o_fat_data_struct,
    pub plugin_data: *mut plugin_data_struct,
    pub pef_data: *mut bfd_pef_data_struct,
    pub pef_xlib_data: *mut bfd_pef_xlib_data_struct,
    pub sym_data: *mut bfd_sym_data_struct,
    pub any: *mut libc::c_void,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct elf_obj_tdata {
    pub elf_header: [Elf_Internal_Ehdr; 1],
    pub elf_sect_ptr: *mut *mut Elf_Internal_Shdr,
    pub phdr: *mut Elf_Internal_Phdr,
    pub symtab_hdr: Elf_Internal_Shdr,
    pub shstrtab_hdr: Elf_Internal_Shdr,
    pub strtab_hdr: Elf_Internal_Shdr,
    pub dynsymtab_hdr: Elf_Internal_Shdr,
    pub dynstrtab_hdr: Elf_Internal_Shdr,
    pub dynversym_hdr: Elf_Internal_Shdr,
    pub dynverref_hdr: Elf_Internal_Shdr,
    pub dynverdef_hdr: Elf_Internal_Shdr,
    pub symtab_shndx_list: *mut elf_section_list,
    pub gp: bfd_vma,
    pub gp_size: libc::c_uint,
    pub num_elf_sections: libc::c_uint,
    pub being_created: *mut libc::c_uchar,
    pub sym_hashes: *mut *mut elf_link_hash_entry,
    pub local_got: C2RustUnnamed_15,
    pub dt_name: *const libc::c_char,
    pub dt_audit: *const libc::c_char,
    pub line_info: *mut libc::c_void,
    pub dwarf1_find_line_info: *mut dwarf1_debug,
    pub dwarf2_find_line_info: *mut libc::c_void,
    pub elf_find_function_cache: *mut libc::c_void,
    pub cverdefs: libc::c_uint,
    pub cverrefs: libc::c_uint,
    pub verdef: *mut Elf_Internal_Verdef,
    pub verref: *mut Elf_Internal_Verneed,
    pub eh_frame_section: *mut asection,
    pub symbuf: *mut libc::c_void,
    pub properties: *mut elf_property_list,
    pub known_obj_attributes: [[obj_attribute; 71]; 2],
    pub other_obj_attributes: [*mut obj_attribute_list; 2],
    pub sdt_note_head: *mut sdt_note,
    pub group_sect_ptr: *mut *mut Elf_Internal_Shdr,
    pub num_group: libc::c_uint,
    pub group_search_offset: libc::c_uint,
    pub symtab_section: libc::c_uint,
    pub dynsymtab_section: libc::c_uint,
    pub dynversym_section: libc::c_uint,
    pub dynverdef_section: libc::c_uint,
    pub dynverref_section: libc::c_uint,
    #[bitfield(name = "object_id", ty = "elf_target_id", bits = "0..=5")]
    #[bitfield(name = "dyn_lib_class", ty = "dynamic_lib_link_class", bits = "6..=9")]
    #[bitfield(name = "has_gnu_osabi", ty = "elf_gnu_osabi", bits = "10..=13")]
    #[bitfield(name = "has_no_copy_on_protected", ty = "libc::c_uint", bits = "14..=14")]
    #[bitfield(name = "bad_symtab", ty = "libc::c_uint", bits = "15..=15")]
    #[bitfield(name = "is_pie", ty = "libc::c_uint", bits = "16..=16")]
    pub object_id_dyn_lib_class_has_gnu_osabi_has_no_copy_on_protected_bad_symtab_is_pie: [u8; 3],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
    pub core: *mut core_elf_obj_tdata,
    pub o: *mut output_elf_obj_tdata,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct output_elf_obj_tdata {
    pub seg_map: *mut elf_segment_map,
    pub strtab_ptr: *mut elf_strtab_hash,
    pub section_syms: *mut *mut asymbol,
    pub eh_frame_hdr: *mut asection,
    pub build_id: C2RustUnnamed_13,
    pub program_header_size: bfd_size_type,
    pub next_file_pos: file_ptr,
    pub link_info: *mut bfd_link_info,
    pub num_section_syms: libc::c_int,
    pub shstrtab_section: libc::c_uint,
    pub strtab_section: libc::c_uint,
    pub stack_flags: libc::c_uint,
    pub flags_init: bool,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct bfd_link_info {
    #[bitfield(name = "type_0", ty = "output_type", bits = "0..=1")]
    #[bitfield(name = "symbolic", ty = "libc::c_uint", bits = "2..=2")]
    #[bitfield(name = "nocopyreloc", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "export_dynamic", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "create_default_symver", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "gc_sections", ty = "libc::c_uint", bits = "6..=6")]
    #[bitfield(name = "gc_keep_exported", ty = "libc::c_uint", bits = "7..=7")]
    #[bitfield(name = "notice_all", ty = "libc::c_uint", bits = "8..=8")]
    #[bitfield(name = "lto_plugin_active", ty = "libc::c_uint", bits = "9..=9")]
    #[bitfield(name = "lto_all_symbols_read", ty = "libc::c_uint", bits = "10..=10")]
    #[bitfield(name = "strip_discarded", ty = "libc::c_uint", bits = "11..=11")]
    #[bitfield(name = "dynamic_data", ty = "libc::c_uint", bits = "12..=12")]
    #[bitfield(name = "resolve_section_groups", ty = "libc::c_uint", bits = "13..=13")]
    #[bitfield(name = "big_endian", ty = "libc::c_uint", bits = "14..=14")]
    #[bitfield(name = "strip", ty = "bfd_link_strip", bits = "15..=16")]
    #[bitfield(name = "discard", ty = "bfd_link_discard", bits = "17..=18")]
    #[bitfield(name = "elf_stt_common", ty = "bfd_link_elf_stt_common", bits = "19..=20")]
    #[bitfield(name = "common_skip_ar_symbols", ty = "bfd_link_common_skip_ar_symbols", bits = "21..=22")]
    #[bitfield(name = "unresolved_syms_in_objects", ty = "report_method", bits = "23..=24")]
    #[bitfield(name = "unresolved_syms_in_shared_libs", ty = "report_method", bits = "25..=26")]
    #[bitfield(name = "warn_unresolved_syms", ty = "libc::c_uint", bits = "27..=27")]
    #[bitfield(name = "static_link", ty = "libc::c_uint", bits = "28..=28")]
    #[bitfield(name = "keep_memory", ty = "libc::c_uint", bits = "29..=29")]
    #[bitfield(name = "emitrelocations", ty = "libc::c_uint", bits = "30..=30")]
    #[bitfield(name = "relro", ty = "libc::c_uint", bits = "31..=31")]
    #[bitfield(name = "separate_code", ty = "libc::c_uint", bits = "32..=32")]
    #[bitfield(name = "eh_frame_hdr_type", ty = "libc::c_uint", bits = "33..=34")]
    #[bitfield(name = "textrel_check", ty = "textrel_check_method", bits = "35..=36")]
    #[bitfield(name = "emit_hash", ty = "libc::c_uint", bits = "37..=37")]
    #[bitfield(name = "emit_gnu_hash", ty = "libc::c_uint", bits = "38..=38")]
    #[bitfield(name = "reduce_memory_overheads", ty = "libc::c_uint", bits = "39..=39")]
    #[bitfield(name = "traditional_format", ty = "libc::c_uint", bits = "40..=40")]
    #[bitfield(name = "combreloc", ty = "libc::c_uint", bits = "41..=41")]
    #[bitfield(name = "default_imported_symver", ty = "libc::c_uint", bits = "42..=42")]
    #[bitfield(name = "new_dtags", ty = "libc::c_uint", bits = "43..=43")]
    #[bitfield(name = "no_ld_generated_unwind_info", ty = "libc::c_uint", bits = "44..=44")]
    #[bitfield(name = "task_link", ty = "libc::c_uint", bits = "45..=45")]
    #[bitfield(name = "allow_multiple_definition", ty = "libc::c_uint", bits = "46..=46")]
    #[bitfield(name = "prohibit_multiple_definition_absolute", ty = "libc::c_uint", bits = "47..=47")]
    #[bitfield(name = "warn_multiple_definition", ty = "libc::c_uint", bits = "48..=48")]
    #[bitfield(name = "allow_undefined_version", ty = "libc::c_uint", bits = "49..=49")]
    #[bitfield(name = "dynamic", ty = "libc::c_uint", bits = "50..=50")]
    #[bitfield(name = "execstack", ty = "libc::c_uint", bits = "51..=51")]
    #[bitfield(name = "noexecstack", ty = "libc::c_uint", bits = "52..=52")]
    #[bitfield(name = "optimize", ty = "libc::c_uint", bits = "53..=53")]
    #[bitfield(name = "print_gc_sections", ty = "libc::c_uint", bits = "54..=54")]
    #[bitfield(name = "warn_alternate_em", ty = "libc::c_uint", bits = "55..=55")]
    #[bitfield(name = "user_phdrs", ty = "libc::c_uint", bits = "56..=56")]
    #[bitfield(name = "load_phdrs", ty = "libc::c_uint", bits = "57..=57")]
    #[bitfield(name = "check_relocs_after_open_input", ty = "libc::c_uint", bits = "58..=58")]
    #[bitfield(name = "nointerp", ty = "libc::c_uint", bits = "59..=59")]
    #[bitfield(name = "inhibit_common_definition", ty = "libc::c_uint", bits = "60..=60")]
    #[bitfield(name = "has_map_file", ty = "libc::c_uint", bits = "61..=61")]
    #[bitfield(name = "non_contiguous_regions", ty = "libc::c_uint", bits = "62..=62")]
    #[bitfield(name = "non_contiguous_regions_warnings", ty = "libc::c_uint", bits = "63..=63")]
    #[bitfield(name = "unique_symbol", ty = "libc::c_uint", bits = "64..=64")]
    pub type_0_symbolic_nocopyreloc_export_dynamic_create_default_symver_gc_sections_gc_keep_exported_notice_all_lto_plugin_active_lto_all_symbols_read_strip_discarded_dynamic_data_resolve_section_groups_big_endian_strip_discard_elf_stt_common_common_skip_ar_symbols_unresolved_syms_in_objects_unresolved_syms_in_shared_libs_warn_unresolved_syms_static_link_keep_memory_emitrelocations_relro_separate_code_eh_frame_hdr_type_textrel_check_emit_hash_emit_gnu_hash_reduce_memory_overheads_traditional_format_combreloc_default_imported_symver_new_dtags_no_ld_generated_unwind_info_task_link_allow_multiple_definition_prohibit_multiple_definition_absolute_warn_multiple_definition_allow_undefined_version_dynamic_execstack_noexecstack_optimize_print_gc_sections_warn_alternate_em_user_phdrs_load_phdrs_check_relocs_after_open_input_nointerp_inhibit_common_definition_has_map_file_non_contiguous_regions_non_contiguous_regions_warnings_unique_symbol: [u8; 9],
    pub wrap_char: libc::c_char,
    pub path_separator: libc::c_char,
    pub compress_debug: compressed_debug_section_type,
    pub stacksize: bfd_signed_vma,
    pub disable_target_specific_optimizations: libc::c_int,
    pub callbacks: *const bfd_link_callbacks,
    pub hash: *mut bfd_link_hash_table,
    pub keep_hash: *mut bfd_hash_table,
    pub notice_hash: *mut bfd_hash_table,
    pub wrap_hash: *mut bfd_hash_table,
    pub ignore_hash: *mut bfd_hash_table,
    pub output_bfd: *mut bfd,
    pub out_implib_bfd: *mut bfd,
    pub input_bfds: *mut bfd,
    pub input_bfds_tail: *mut *mut bfd,
    pub create_object_symbols_section: *mut asection,
    pub gc_sym_list: *mut bfd_sym_chain,
    pub base_file: *mut libc::c_void,
    pub init_function: *const libc::c_char,
    pub fini_function: *const libc::c_char,
    pub relax_pass: libc::c_int,
    pub relax_trip: libc::c_int,
    pub extern_protected_data: libc::c_int,
    pub dynamic_undefined_weak: libc::c_int,
    pub pei386_auto_import: libc::c_int,
    pub pei386_runtime_pseudo_reloc: libc::c_int,
    pub spare_dynamic_tags: libc::c_uint,
    pub flags: bfd_vma,
    pub flags_1: bfd_vma,
    pub gnu_flags_1: bfd_vma,
    pub start_stop_gc: libc::c_int,
    pub start_stop_visibility: libc::c_uint,
    pub maxpagesize: bfd_vma,
    pub commonpagesize: bfd_vma,
    pub relro_start: bfd_vma,
    pub relro_end: bfd_vma,
    pub dynamic_list: *mut bfd_elf_dynamic_list,
    pub version_info: *mut bfd_elf_version_tree,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bfd_elf_version_tree {
    pub next: *mut bfd_elf_version_tree,
    pub name: *const libc::c_char,
    pub vernum: libc::c_uint,
    pub globals: bfd_elf_version_expr_head,
    pub locals: bfd_elf_version_expr_head,
    pub deps: *mut bfd_elf_version_deps,
    pub name_indx: libc::c_uint,
    pub used: libc::c_int,
    pub match_0: Option::<
        unsafe extern "C" fn(
            *mut bfd_elf_version_expr_head,
            *mut bfd_elf_version_expr,
            *const libc::c_char,
        ) -> *mut bfd_elf_version_expr,
    >,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct bfd_elf_version_expr {
    pub next: *mut bfd_elf_version_expr,
    pub pattern: *const libc::c_char,
    #[bitfield(name = "literal", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "symver", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "script", ty = "libc::c_uint", bits = "2..=2")]
    #[bitfield(name = "mask", ty = "libc::c_uint", bits = "3..=5")]
    pub literal_symver_script_mask: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bfd_elf_version_expr_head {
    pub list: *mut bfd_elf_version_expr,
    pub htab: *mut libc::c_void,
    pub remaining: *mut bfd_elf_version_expr,
    pub mask: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bfd_elf_version_deps {
    pub next: *mut bfd_elf_version_deps,
    pub version_needed: *mut bfd_elf_version_tree,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bfd_elf_dynamic_list {
    pub head: bfd_elf_version_expr_head,
    pub match_0: Option::<
        unsafe extern "C" fn(
            *mut bfd_elf_version_expr_head,
            *mut bfd_elf_version_expr,
            *const libc::c_char,
        ) -> *mut bfd_elf_version_expr,
    >,
}
pub type bfd_vma = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bfd_sym_chain {
    pub next: *mut bfd_sym_chain,
    pub name: *const libc::c_char,
}
pub type asection = bfd_section;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct bfd_section {
    pub name: *const libc::c_char,
    pub id: libc::c_uint,
    pub section_id: libc::c_uint,
    pub index: libc::c_uint,
    pub next: *mut bfd_section,
    pub prev: *mut bfd_section,
    pub flags: flagword,
    #[bitfield(name = "user_set_vma", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "linker_mark", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "linker_has_input", ty = "libc::c_uint", bits = "2..=2")]
    #[bitfield(name = "gc_mark", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "compress_status", ty = "libc::c_uint", bits = "4..=5")]
    #[bitfield(name = "segment_mark", ty = "libc::c_uint", bits = "6..=6")]
    #[bitfield(name = "sec_info_type", ty = "libc::c_uint", bits = "7..=9")]
    #[bitfield(name = "use_rela_p", ty = "libc::c_uint", bits = "10..=10")]
    #[bitfield(name = "sec_flg0", ty = "libc::c_uint", bits = "11..=11")]
    #[bitfield(name = "sec_flg1", ty = "libc::c_uint", bits = "12..=12")]
    #[bitfield(name = "sec_flg2", ty = "libc::c_uint", bits = "13..=13")]
    #[bitfield(name = "sec_flg3", ty = "libc::c_uint", bits = "14..=14")]
    #[bitfield(name = "sec_flg4", ty = "libc::c_uint", bits = "15..=15")]
    #[bitfield(name = "sec_flg5", ty = "libc::c_uint", bits = "16..=16")]
    pub user_set_vma_linker_mark_linker_has_input_gc_mark_compress_status_segment_mark_sec_info_type_use_rela_p_sec_flg0_sec_flg1_sec_flg2_sec_flg3_sec_flg4_sec_flg5: [u8; 3],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
    pub vma: bfd_vma,
    pub lma: bfd_vma,
    pub size: bfd_size_type,
    pub rawsize: bfd_size_type,
    pub compressed_size: bfd_size_type,
    pub relax: *mut relax_table,
    pub relax_count: libc::c_int,
    pub output_offset: bfd_vma,
    pub output_section: *mut bfd_section,
    pub alignment_power: libc::c_uint,
    pub relocation: *mut reloc_cache_entry,
    pub orelocation: *mut *mut reloc_cache_entry,
    pub reloc_count: libc::c_uint,
    pub filepos: file_ptr,
    pub rel_filepos: file_ptr,
    pub line_filepos: file_ptr,
    pub userdata: *mut libc::c_void,
    pub contents: *mut libc::c_uchar,
    pub lineno: *mut alent,
    pub lineno_count: libc::c_uint,
    pub entsize: libc::c_uint,
    pub kept_section: *mut bfd_section,
    pub moving_line_filepos: file_ptr,
    pub target_index: libc::c_int,
    pub used_by_bfd: *mut libc::c_void,
    pub constructor_chain: *mut relent_chain,
    pub owner: *mut bfd,
    pub symbol: *mut bfd_symbol,
    pub symbol_ptr_ptr: *mut *mut bfd_symbol,
    pub map_head: C2RustUnnamed_0,
    pub map_tail: C2RustUnnamed_0,
    pub already_assigned: *mut bfd_section,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub link_order: *mut bfd_link_order,
    pub s: *mut bfd_section,
    pub linked_to_symbol_name: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bfd_link_order {
    pub next: *mut bfd_link_order,
    pub type_0: bfd_link_order_type,
    pub offset: bfd_vma,
    pub size: bfd_size_type,
    pub u: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub indirect: C2RustUnnamed_5,
    pub data: C2RustUnnamed_4,
    pub reloc: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub p: *mut bfd_link_order_reloc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bfd_link_order_reloc {
    pub reloc: bfd_reloc_code_real_type,
    pub u: C2RustUnnamed_3,
    pub addend: bfd_vma,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub section: *mut asection,
    pub name: *const libc::c_char,
}
pub type bfd_reloc_code_real_type = bfd_reloc_code_real;
pub type bfd_reloc_code_real = libc::c_uint;
pub const BFD_RELOC_UNUSED: bfd_reloc_code_real = 2383;
pub const BFD_RELOC_S12Z_OPR: bfd_reloc_code_real = 2382;
pub const BFD_RELOC_CKCORE_PCREL_BLOOP_IMM12BY4: bfd_reloc_code_real = 2381;
pub const BFD_RELOC_CKCORE_PCREL_BLOOP_IMM4BY4: bfd_reloc_code_real = 2380;
pub const BFD_RELOC_CKCORE_IRELATIVE: bfd_reloc_code_real = 2379;
pub const BFD_RELOC_CKCORE_CALLGRAPH: bfd_reloc_code_real = 2378;
pub const BFD_RELOC_CKCORE_NOJSRI: bfd_reloc_code_real = 2377;
pub const BFD_RELOC_CKCORE_PCREL_FLRW_IMM8BY4: bfd_reloc_code_real = 2376;
pub const BFD_RELOC_CKCORE_TLS_TPOFF32: bfd_reloc_code_real = 2375;
pub const BFD_RELOC_CKCORE_TLS_DTPOFF32: bfd_reloc_code_real = 2374;
pub const BFD_RELOC_CKCORE_TLS_DTPMOD32: bfd_reloc_code_real = 2373;
pub const BFD_RELOC_CKCORE_TLS_LDO32: bfd_reloc_code_real = 2372;
pub const BFD_RELOC_CKCORE_TLS_LDM32: bfd_reloc_code_real = 2371;
pub const BFD_RELOC_CKCORE_TLS_GD32: bfd_reloc_code_real = 2370;
pub const BFD_RELOC_CKCORE_TLS_IE32: bfd_reloc_code_real = 2369;
pub const BFD_RELOC_CKCORE_TLS_LE32: bfd_reloc_code_real = 2368;
pub const BFD_RELOC_CKCORE_PCREL_IMM7BY4: bfd_reloc_code_real = 2367;
pub const BFD_RELOC_CKCORE_PLT_IMM18BY4: bfd_reloc_code_real = 2366;
pub const BFD_RELOC_CKCORE_GOT_IMM18BY4: bfd_reloc_code_real = 2365;
pub const BFD_RELOC_CKCORE_GOTOFF_IMM18: bfd_reloc_code_real = 2364;
pub const BFD_RELOC_CKCORE_DOFFSET_IMM18BY4: bfd_reloc_code_real = 2363;
pub const BFD_RELOC_CKCORE_DOFFSET_IMM18BY2: bfd_reloc_code_real = 2362;
pub const BFD_RELOC_CKCORE_DOFFSET_IMM18: bfd_reloc_code_real = 2361;
pub const BFD_RELOC_CKCORE_PCREL_IMM18BY2: bfd_reloc_code_real = 2360;
pub const BFD_RELOC_CKCORE_DOFFSET_LO16: bfd_reloc_code_real = 2359;
pub const BFD_RELOC_CKCORE_TOFFSET_LO16: bfd_reloc_code_real = 2358;
pub const BFD_RELOC_CKCORE_PCREL_JSR_IMM26BY2: bfd_reloc_code_real = 2357;
pub const BFD_RELOC_CKCORE_ADDRPLT_LO16: bfd_reloc_code_real = 2356;
pub const BFD_RELOC_CKCORE_ADDRPLT_HI16: bfd_reloc_code_real = 2355;
pub const BFD_RELOC_CKCORE_ADDRGOT_LO16: bfd_reloc_code_real = 2354;
pub const BFD_RELOC_CKCORE_ADDRGOT_HI16: bfd_reloc_code_real = 2353;
pub const BFD_RELOC_CKCORE_PLT_LO16: bfd_reloc_code_real = 2352;
pub const BFD_RELOC_CKCORE_PLT_HI16: bfd_reloc_code_real = 2351;
pub const BFD_RELOC_CKCORE_PLT12: bfd_reloc_code_real = 2350;
pub const BFD_RELOC_CKCORE_GOT_LO16: bfd_reloc_code_real = 2349;
pub const BFD_RELOC_CKCORE_GOT_HI16: bfd_reloc_code_real = 2348;
pub const BFD_RELOC_CKCORE_GOT12: bfd_reloc_code_real = 2347;
pub const BFD_RELOC_CKCORE_GOTOFF_LO16: bfd_reloc_code_real = 2346;
pub const BFD_RELOC_CKCORE_GOTOFF_HI16: bfd_reloc_code_real = 2345;
pub const BFD_RELOC_CKCORE_GOTPC_LO16: bfd_reloc_code_real = 2344;
pub const BFD_RELOC_CKCORE_GOTPC_HI16: bfd_reloc_code_real = 2343;
pub const BFD_RELOC_CKCORE_ADDR_LO16: bfd_reloc_code_real = 2342;
pub const BFD_RELOC_CKCORE_ADDR_HI16: bfd_reloc_code_real = 2341;
pub const BFD_RELOC_CKCORE_PCREL_IMM10BY4: bfd_reloc_code_real = 2340;
pub const BFD_RELOC_CKCORE_PCREL_IMM10BY2: bfd_reloc_code_real = 2339;
pub const BFD_RELOC_CKCORE_PCREL_IMM16BY4: bfd_reloc_code_real = 2338;
pub const BFD_RELOC_CKCORE_PCREL_IMM16BY2: bfd_reloc_code_real = 2337;
pub const BFD_RELOC_CKCORE_PCREL_IMM26BY2: bfd_reloc_code_real = 2336;
pub const BFD_RELOC_CKCORE_ADDRPLT: bfd_reloc_code_real = 2335;
pub const BFD_RELOC_CKCORE_ADDRGOT: bfd_reloc_code_real = 2334;
pub const BFD_RELOC_CKCORE_PLT32: bfd_reloc_code_real = 2333;
pub const BFD_RELOC_CKCORE_GOT32: bfd_reloc_code_real = 2332;
pub const BFD_RELOC_CKCORE_GOTPC: bfd_reloc_code_real = 2331;
pub const BFD_RELOC_CKCORE_GOTOFF: bfd_reloc_code_real = 2330;
pub const BFD_RELOC_CKCORE_JUMP_SLOT: bfd_reloc_code_real = 2329;
pub const BFD_RELOC_CKCORE_GLOB_DAT: bfd_reloc_code_real = 2328;
pub const BFD_RELOC_CKCORE_COPY: bfd_reloc_code_real = 2327;
pub const BFD_RELOC_CKCORE_RELATIVE: bfd_reloc_code_real = 2326;
pub const BFD_RELOC_CKCORE_GNU_VTENTRY: bfd_reloc_code_real = 2325;
pub const BFD_RELOC_CKCORE_GNU_VTINHERIT: bfd_reloc_code_real = 2324;
pub const BFD_RELOC_CKCORE_PCREL_JSR_IMM11BY2: bfd_reloc_code_real = 2323;
pub const BFD_RELOC_CKCORE_PCREL32: bfd_reloc_code_real = 2322;
pub const BFD_RELOC_CKCORE_PCREL_IMM4BY2: bfd_reloc_code_real = 2321;
pub const BFD_RELOC_CKCORE_PCREL_IMM11BY2: bfd_reloc_code_real = 2320;
pub const BFD_RELOC_CKCORE_PCREL_IMM8BY4: bfd_reloc_code_real = 2319;
pub const BFD_RELOC_CKCORE_ADDR32: bfd_reloc_code_real = 2318;
pub const BFD_RELOC_CKCORE_NONE: bfd_reloc_code_real = 2317;
pub const BFD_RELOC_WASM32_PLT_SIG: bfd_reloc_code_real = 2316;
pub const BFD_RELOC_WASM32_INDEX: bfd_reloc_code_real = 2315;
pub const BFD_RELOC_WASM32_CODE_POINTER: bfd_reloc_code_real = 2314;
pub const BFD_RELOC_WASM32_COPY: bfd_reloc_code_real = 2313;
pub const BFD_RELOC_WASM32_ABS32_CODE: bfd_reloc_code_real = 2312;
pub const BFD_RELOC_WASM32_PLT_INDEX: bfd_reloc_code_real = 2311;
pub const BFD_RELOC_WASM32_LEB128_PLT: bfd_reloc_code_real = 2310;
pub const BFD_RELOC_WASM32_LEB128_GOT_CODE: bfd_reloc_code_real = 2309;
pub const BFD_RELOC_WASM32_LEB128_GOT: bfd_reloc_code_real = 2308;
pub const BFD_RELOC_WASM32_LEB128: bfd_reloc_code_real = 2307;
pub const BFD_RELOC_VISIUM_IM16_PCREL: bfd_reloc_code_real = 2306;
pub const BFD_RELOC_VISIUM_LO16_PCREL: bfd_reloc_code_real = 2305;
pub const BFD_RELOC_VISIUM_HI16_PCREL: bfd_reloc_code_real = 2304;
pub const BFD_RELOC_VISIUM_REL16: bfd_reloc_code_real = 2303;
pub const BFD_RELOC_VISIUM_IM16: bfd_reloc_code_real = 2302;
pub const BFD_RELOC_VISIUM_LO16: bfd_reloc_code_real = 2301;
pub const BFD_RELOC_VISIUM_HI16: bfd_reloc_code_real = 2300;
pub const BFD_RELOC_EPIPHANY_IMM8: bfd_reloc_code_real = 2299;
pub const BFD_RELOC_EPIPHANY_IMM11: bfd_reloc_code_real = 2298;
pub const BFD_RELOC_EPIPHANY_SIMM11: bfd_reloc_code_real = 2297;
pub const BFD_RELOC_EPIPHANY_LOW: bfd_reloc_code_real = 2296;
pub const BFD_RELOC_EPIPHANY_HIGH: bfd_reloc_code_real = 2295;
pub const BFD_RELOC_EPIPHANY_SIMM24: bfd_reloc_code_real = 2294;
pub const BFD_RELOC_EPIPHANY_SIMM8: bfd_reloc_code_real = 2293;
pub const BFD_RELOC_BPF_DISP32: bfd_reloc_code_real = 2292;
pub const BFD_RELOC_BPF_DISP16: bfd_reloc_code_real = 2291;
pub const BFD_RELOC_BPF_16: bfd_reloc_code_real = 2290;
pub const BFD_RELOC_BPF_32: bfd_reloc_code_real = 2289;
pub const BFD_RELOC_BPF_64: bfd_reloc_code_real = 2288;
pub const BFD_RELOC_TILEGX_IMM8_Y1_TLS_ADD: bfd_reloc_code_real = 2287;
pub const BFD_RELOC_TILEGX_IMM8_Y0_TLS_ADD: bfd_reloc_code_real = 2286;
pub const BFD_RELOC_TILEGX_IMM8_X1_TLS_ADD: bfd_reloc_code_real = 2285;
pub const BFD_RELOC_TILEGX_IMM8_X0_TLS_ADD: bfd_reloc_code_real = 2284;
pub const BFD_RELOC_TILEGX_TLS_IE_LOAD: bfd_reloc_code_real = 2283;
pub const BFD_RELOC_TILEGX_IMM8_Y1_TLS_GD_ADD: bfd_reloc_code_real = 2282;
pub const BFD_RELOC_TILEGX_IMM8_Y0_TLS_GD_ADD: bfd_reloc_code_real = 2281;
pub const BFD_RELOC_TILEGX_IMM8_X1_TLS_GD_ADD: bfd_reloc_code_real = 2280;
pub const BFD_RELOC_TILEGX_IMM8_X0_TLS_GD_ADD: bfd_reloc_code_real = 2279;
pub const BFD_RELOC_TILEGX_TLS_GD_CALL: bfd_reloc_code_real = 2278;
pub const BFD_RELOC_TILEGX_TLS_TPOFF32: bfd_reloc_code_real = 2277;
pub const BFD_RELOC_TILEGX_TLS_DTPOFF32: bfd_reloc_code_real = 2276;
pub const BFD_RELOC_TILEGX_TLS_DTPMOD32: bfd_reloc_code_real = 2275;
pub const BFD_RELOC_TILEGX_TLS_TPOFF64: bfd_reloc_code_real = 2274;
pub const BFD_RELOC_TILEGX_TLS_DTPOFF64: bfd_reloc_code_real = 2273;
pub const BFD_RELOC_TILEGX_TLS_DTPMOD64: bfd_reloc_code_real = 2272;
pub const BFD_RELOC_TILEGX_IMM16_X1_HW1_LAST_TLS_IE: bfd_reloc_code_real = 2271;
pub const BFD_RELOC_TILEGX_IMM16_X0_HW1_LAST_TLS_IE: bfd_reloc_code_real = 2270;
pub const BFD_RELOC_TILEGX_IMM16_X1_HW0_LAST_TLS_IE: bfd_reloc_code_real = 2269;
pub const BFD_RELOC_TILEGX_IMM16_X0_HW0_LAST_TLS_IE: bfd_reloc_code_real = 2268;
pub const BFD_RELOC_TILEGX_IMM16_X1_HW2_LAST_PLT_PCREL: bfd_reloc_code_real = 2267;
pub const BFD_RELOC_TILEGX_IMM16_X0_HW2_LAST_PLT_PCREL: bfd_reloc_code_real = 2266;
pub const BFD_RELOC_TILEGX_IMM16_X1_HW1_LAST_PLT_PCREL: bfd_reloc_code_real = 2265;
pub const BFD_RELOC_TILEGX_IMM16_X0_HW1_LAST_PLT_PCREL: bfd_reloc_code_real = 2264;
pub const BFD_RELOC_TILEGX_IMM16_X1_HW0_LAST_PLT_PCREL: bfd_reloc_code_real = 2263;
pub const BFD_RELOC_TILEGX_IMM16_X0_HW0_LAST_PLT_PCREL: bfd_reloc_code_real = 2262;
pub const BFD_RELOC_TILEGX_IMM16_X1_HW0_TLS_IE: bfd_reloc_code_real = 2261;
pub const BFD_RELOC_TILEGX_IMM16_X0_HW0_TLS_IE: bfd_reloc_code_real = 2260;
pub const BFD_RELOC_TILEGX_IMM16_X1_HW1_LAST_TLS_GD: bfd_reloc_code_real = 2259;
pub const BFD_RELOC_TILEGX_IMM16_X0_HW1_LAST_TLS_GD: bfd_reloc_code_real = 2258;
pub const BFD_RELOC_TILEGX_IMM16_X1_HW0_LAST_TLS_GD: bfd_reloc_code_real = 2257;
pub const BFD_RELOC_TILEGX_IMM16_X0_HW0_LAST_TLS_GD: bfd_reloc_code_real = 2256;
pub const BFD_RELOC_TILEGX_IMM16_X1_HW1_LAST_TLS_LE: bfd_reloc_code_real = 2255;
pub const BFD_RELOC_TILEGX_IMM16_X0_HW1_LAST_TLS_LE: bfd_reloc_code_real = 2254;
pub const BFD_RELOC_TILEGX_IMM16_X1_HW0_LAST_TLS_LE: bfd_reloc_code_real = 2253;
pub const BFD_RELOC_TILEGX_IMM16_X0_HW0_LAST_TLS_LE: bfd_reloc_code_real = 2252;
pub const BFD_RELOC_TILEGX_IMM16_X1_HW0_TLS_LE: bfd_reloc_code_real = 2251;
pub const BFD_RELOC_TILEGX_IMM16_X0_HW0_TLS_LE: bfd_reloc_code_real = 2250;
pub const BFD_RELOC_TILEGX_IMM16_X1_HW0_TLS_GD: bfd_reloc_code_real = 2249;
pub const BFD_RELOC_TILEGX_IMM16_X0_HW0_TLS_GD: bfd_reloc_code_real = 2248;
pub const BFD_RELOC_TILEGX_IMM16_X1_HW3_PLT_PCREL: bfd_reloc_code_real = 2247;
pub const BFD_RELOC_TILEGX_IMM16_X0_HW3_PLT_PCREL: bfd_reloc_code_real = 2246;
pub const BFD_RELOC_TILEGX_IMM16_X1_HW1_LAST_GOT: bfd_reloc_code_real = 2245;
pub const BFD_RELOC_TILEGX_IMM16_X0_HW1_LAST_GOT: bfd_reloc_code_real = 2244;
pub const BFD_RELOC_TILEGX_IMM16_X1_HW0_LAST_GOT: bfd_reloc_code_real = 2243;
pub const BFD_RELOC_TILEGX_IMM16_X0_HW0_LAST_GOT: bfd_reloc_code_real = 2242;
pub const BFD_RELOC_TILEGX_IMM16_X1_HW2_PLT_PCREL: bfd_reloc_code_real = 2241;
pub const BFD_RELOC_TILEGX_IMM16_X0_HW2_PLT_PCREL: bfd_reloc_code_real = 2240;
pub const BFD_RELOC_TILEGX_IMM16_X1_HW1_PLT_PCREL: bfd_reloc_code_real = 2239;
pub const BFD_RELOC_TILEGX_IMM16_X0_HW1_PLT_PCREL: bfd_reloc_code_real = 2238;
pub const BFD_RELOC_TILEGX_IMM16_X1_HW0_PLT_PCREL: bfd_reloc_code_real = 2237;
pub const BFD_RELOC_TILEGX_IMM16_X0_HW0_PLT_PCREL: bfd_reloc_code_real = 2236;
pub const BFD_RELOC_TILEGX_IMM16_X1_HW0_GOT: bfd_reloc_code_real = 2235;
pub const BFD_RELOC_TILEGX_IMM16_X0_HW0_GOT: bfd_reloc_code_real = 2234;
pub const BFD_RELOC_TILEGX_IMM16_X1_HW2_LAST_PCREL: bfd_reloc_code_real = 2233;
pub const BFD_RELOC_TILEGX_IMM16_X0_HW2_LAST_PCREL: bfd_reloc_code_real = 2232;
pub const BFD_RELOC_TILEGX_IMM16_X1_HW1_LAST_PCREL: bfd_reloc_code_real = 2231;
pub const BFD_RELOC_TILEGX_IMM16_X0_HW1_LAST_PCREL: bfd_reloc_code_real = 2230;
pub const BFD_RELOC_TILEGX_IMM16_X1_HW0_LAST_PCREL: bfd_reloc_code_real = 2229;
pub const BFD_RELOC_TILEGX_IMM16_X0_HW0_LAST_PCREL: bfd_reloc_code_real = 2228;
pub const BFD_RELOC_TILEGX_IMM16_X1_HW3_PCREL: bfd_reloc_code_real = 2227;
pub const BFD_RELOC_TILEGX_IMM16_X0_HW3_PCREL: bfd_reloc_code_real = 2226;
pub const BFD_RELOC_TILEGX_IMM16_X1_HW2_PCREL: bfd_reloc_code_real = 2225;
pub const BFD_RELOC_TILEGX_IMM16_X0_HW2_PCREL: bfd_reloc_code_real = 2224;
pub const BFD_RELOC_TILEGX_IMM16_X1_HW1_PCREL: bfd_reloc_code_real = 2223;
pub const BFD_RELOC_TILEGX_IMM16_X0_HW1_PCREL: bfd_reloc_code_real = 2222;
pub const BFD_RELOC_TILEGX_IMM16_X1_HW0_PCREL: bfd_reloc_code_real = 2221;
pub const BFD_RELOC_TILEGX_IMM16_X0_HW0_PCREL: bfd_reloc_code_real = 2220;
pub const BFD_RELOC_TILEGX_IMM16_X1_HW2_LAST: bfd_reloc_code_real = 2219;
pub const BFD_RELOC_TILEGX_IMM16_X0_HW2_LAST: bfd_reloc_code_real = 2218;
pub const BFD_RELOC_TILEGX_IMM16_X1_HW1_LAST: bfd_reloc_code_real = 2217;
pub const BFD_RELOC_TILEGX_IMM16_X0_HW1_LAST: bfd_reloc_code_real = 2216;
pub const BFD_RELOC_TILEGX_IMM16_X1_HW0_LAST: bfd_reloc_code_real = 2215;
pub const BFD_RELOC_TILEGX_IMM16_X0_HW0_LAST: bfd_reloc_code_real = 2214;
pub const BFD_RELOC_TILEGX_IMM16_X1_HW3: bfd_reloc_code_real = 2213;
pub const BFD_RELOC_TILEGX_IMM16_X0_HW3: bfd_reloc_code_real = 2212;
pub const BFD_RELOC_TILEGX_IMM16_X1_HW2: bfd_reloc_code_real = 2211;
pub const BFD_RELOC_TILEGX_IMM16_X0_HW2: bfd_reloc_code_real = 2210;
pub const BFD_RELOC_TILEGX_IMM16_X1_HW1: bfd_reloc_code_real = 2209;
pub const BFD_RELOC_TILEGX_IMM16_X0_HW1: bfd_reloc_code_real = 2208;
pub const BFD_RELOC_TILEGX_IMM16_X1_HW0: bfd_reloc_code_real = 2207;
pub const BFD_RELOC_TILEGX_IMM16_X0_HW0: bfd_reloc_code_real = 2206;
pub const BFD_RELOC_TILEGX_SHAMT_Y1: bfd_reloc_code_real = 2205;
pub const BFD_RELOC_TILEGX_SHAMT_Y0: bfd_reloc_code_real = 2204;
pub const BFD_RELOC_TILEGX_SHAMT_X1: bfd_reloc_code_real = 2203;
pub const BFD_RELOC_TILEGX_SHAMT_X0: bfd_reloc_code_real = 2202;
pub const BFD_RELOC_TILEGX_MMEND_X0: bfd_reloc_code_real = 2201;
pub const BFD_RELOC_TILEGX_MMSTART_X0: bfd_reloc_code_real = 2200;
pub const BFD_RELOC_TILEGX_MF_IMM14_X1: bfd_reloc_code_real = 2199;
pub const BFD_RELOC_TILEGX_MT_IMM14_X1: bfd_reloc_code_real = 2198;
pub const BFD_RELOC_TILEGX_DEST_IMM8_X1: bfd_reloc_code_real = 2197;
pub const BFD_RELOC_TILEGX_IMM8_Y1: bfd_reloc_code_real = 2196;
pub const BFD_RELOC_TILEGX_IMM8_X1: bfd_reloc_code_real = 2195;
pub const BFD_RELOC_TILEGX_IMM8_Y0: bfd_reloc_code_real = 2194;
pub const BFD_RELOC_TILEGX_IMM8_X0: bfd_reloc_code_real = 2193;
pub const BFD_RELOC_TILEGX_JUMPOFF_X1_PLT: bfd_reloc_code_real = 2192;
pub const BFD_RELOC_TILEGX_JUMPOFF_X1: bfd_reloc_code_real = 2191;
pub const BFD_RELOC_TILEGX_BROFF_X1: bfd_reloc_code_real = 2190;
pub const BFD_RELOC_TILEGX_RELATIVE: bfd_reloc_code_real = 2189;
pub const BFD_RELOC_TILEGX_JMP_SLOT: bfd_reloc_code_real = 2188;
pub const BFD_RELOC_TILEGX_GLOB_DAT: bfd_reloc_code_real = 2187;
pub const BFD_RELOC_TILEGX_COPY: bfd_reloc_code_real = 2186;
pub const BFD_RELOC_TILEGX_HW2_LAST: bfd_reloc_code_real = 2185;
pub const BFD_RELOC_TILEGX_HW1_LAST: bfd_reloc_code_real = 2184;
pub const BFD_RELOC_TILEGX_HW0_LAST: bfd_reloc_code_real = 2183;
pub const BFD_RELOC_TILEGX_HW3: bfd_reloc_code_real = 2182;
pub const BFD_RELOC_TILEGX_HW2: bfd_reloc_code_real = 2181;
pub const BFD_RELOC_TILEGX_HW1: bfd_reloc_code_real = 2180;
pub const BFD_RELOC_TILEGX_HW0: bfd_reloc_code_real = 2179;
pub const BFD_RELOC_TILEPRO_IMM16_X1_TLS_LE_HA: bfd_reloc_code_real = 2178;
pub const BFD_RELOC_TILEPRO_IMM16_X0_TLS_LE_HA: bfd_reloc_code_real = 2177;
pub const BFD_RELOC_TILEPRO_IMM16_X1_TLS_LE_HI: bfd_reloc_code_real = 2176;
pub const BFD_RELOC_TILEPRO_IMM16_X0_TLS_LE_HI: bfd_reloc_code_real = 2175;
pub const BFD_RELOC_TILEPRO_IMM16_X1_TLS_LE_LO: bfd_reloc_code_real = 2174;
pub const BFD_RELOC_TILEPRO_IMM16_X0_TLS_LE_LO: bfd_reloc_code_real = 2173;
pub const BFD_RELOC_TILEPRO_IMM16_X1_TLS_LE: bfd_reloc_code_real = 2172;
pub const BFD_RELOC_TILEPRO_IMM16_X0_TLS_LE: bfd_reloc_code_real = 2171;
pub const BFD_RELOC_TILEPRO_TLS_TPOFF32: bfd_reloc_code_real = 2170;
pub const BFD_RELOC_TILEPRO_TLS_DTPOFF32: bfd_reloc_code_real = 2169;
pub const BFD_RELOC_TILEPRO_TLS_DTPMOD32: bfd_reloc_code_real = 2168;
pub const BFD_RELOC_TILEPRO_IMM16_X1_TLS_IE_HA: bfd_reloc_code_real = 2167;
pub const BFD_RELOC_TILEPRO_IMM16_X0_TLS_IE_HA: bfd_reloc_code_real = 2166;
pub const BFD_RELOC_TILEPRO_IMM16_X1_TLS_IE_HI: bfd_reloc_code_real = 2165;
pub const BFD_RELOC_TILEPRO_IMM16_X0_TLS_IE_HI: bfd_reloc_code_real = 2164;
pub const BFD_RELOC_TILEPRO_IMM16_X1_TLS_IE_LO: bfd_reloc_code_real = 2163;
pub const BFD_RELOC_TILEPRO_IMM16_X0_TLS_IE_LO: bfd_reloc_code_real = 2162;
pub const BFD_RELOC_TILEPRO_IMM16_X1_TLS_IE: bfd_reloc_code_real = 2161;
pub const BFD_RELOC_TILEPRO_IMM16_X0_TLS_IE: bfd_reloc_code_real = 2160;
pub const BFD_RELOC_TILEPRO_IMM16_X1_TLS_GD_HA: bfd_reloc_code_real = 2159;
pub const BFD_RELOC_TILEPRO_IMM16_X0_TLS_GD_HA: bfd_reloc_code_real = 2158;
pub const BFD_RELOC_TILEPRO_IMM16_X1_TLS_GD_HI: bfd_reloc_code_real = 2157;
pub const BFD_RELOC_TILEPRO_IMM16_X0_TLS_GD_HI: bfd_reloc_code_real = 2156;
pub const BFD_RELOC_TILEPRO_IMM16_X1_TLS_GD_LO: bfd_reloc_code_real = 2155;
pub const BFD_RELOC_TILEPRO_IMM16_X0_TLS_GD_LO: bfd_reloc_code_real = 2154;
pub const BFD_RELOC_TILEPRO_IMM16_X1_TLS_GD: bfd_reloc_code_real = 2153;
pub const BFD_RELOC_TILEPRO_IMM16_X0_TLS_GD: bfd_reloc_code_real = 2152;
pub const BFD_RELOC_TILEPRO_TLS_IE_LOAD: bfd_reloc_code_real = 2151;
pub const BFD_RELOC_TILEPRO_IMM8_Y1_TLS_GD_ADD: bfd_reloc_code_real = 2150;
pub const BFD_RELOC_TILEPRO_IMM8_Y0_TLS_GD_ADD: bfd_reloc_code_real = 2149;
pub const BFD_RELOC_TILEPRO_IMM8_X1_TLS_GD_ADD: bfd_reloc_code_real = 2148;
pub const BFD_RELOC_TILEPRO_IMM8_X0_TLS_GD_ADD: bfd_reloc_code_real = 2147;
pub const BFD_RELOC_TILEPRO_TLS_GD_CALL: bfd_reloc_code_real = 2146;
pub const BFD_RELOC_TILEPRO_SHAMT_Y1: bfd_reloc_code_real = 2145;
pub const BFD_RELOC_TILEPRO_SHAMT_Y0: bfd_reloc_code_real = 2144;
pub const BFD_RELOC_TILEPRO_SHAMT_X1: bfd_reloc_code_real = 2143;
pub const BFD_RELOC_TILEPRO_SHAMT_X0: bfd_reloc_code_real = 2142;
pub const BFD_RELOC_TILEPRO_MMEND_X1: bfd_reloc_code_real = 2141;
pub const BFD_RELOC_TILEPRO_MMSTART_X1: bfd_reloc_code_real = 2140;
pub const BFD_RELOC_TILEPRO_MMEND_X0: bfd_reloc_code_real = 2139;
pub const BFD_RELOC_TILEPRO_MMSTART_X0: bfd_reloc_code_real = 2138;
pub const BFD_RELOC_TILEPRO_IMM16_X1_GOT_HA: bfd_reloc_code_real = 2137;
pub const BFD_RELOC_TILEPRO_IMM16_X0_GOT_HA: bfd_reloc_code_real = 2136;
pub const BFD_RELOC_TILEPRO_IMM16_X1_GOT_HI: bfd_reloc_code_real = 2135;
pub const BFD_RELOC_TILEPRO_IMM16_X0_GOT_HI: bfd_reloc_code_real = 2134;
pub const BFD_RELOC_TILEPRO_IMM16_X1_GOT_LO: bfd_reloc_code_real = 2133;
pub const BFD_RELOC_TILEPRO_IMM16_X0_GOT_LO: bfd_reloc_code_real = 2132;
pub const BFD_RELOC_TILEPRO_IMM16_X1_GOT: bfd_reloc_code_real = 2131;
pub const BFD_RELOC_TILEPRO_IMM16_X0_GOT: bfd_reloc_code_real = 2130;
pub const BFD_RELOC_TILEPRO_IMM16_X1_HA_PCREL: bfd_reloc_code_real = 2129;
pub const BFD_RELOC_TILEPRO_IMM16_X0_HA_PCREL: bfd_reloc_code_real = 2128;
pub const BFD_RELOC_TILEPRO_IMM16_X1_HI_PCREL: bfd_reloc_code_real = 2127;
pub const BFD_RELOC_TILEPRO_IMM16_X0_HI_PCREL: bfd_reloc_code_real = 2126;
pub const BFD_RELOC_TILEPRO_IMM16_X1_LO_PCREL: bfd_reloc_code_real = 2125;
pub const BFD_RELOC_TILEPRO_IMM16_X0_LO_PCREL: bfd_reloc_code_real = 2124;
pub const BFD_RELOC_TILEPRO_IMM16_X1_PCREL: bfd_reloc_code_real = 2123;
pub const BFD_RELOC_TILEPRO_IMM16_X0_PCREL: bfd_reloc_code_real = 2122;
pub const BFD_RELOC_TILEPRO_IMM16_X1_HA: bfd_reloc_code_real = 2121;
pub const BFD_RELOC_TILEPRO_IMM16_X0_HA: bfd_reloc_code_real = 2120;
pub const BFD_RELOC_TILEPRO_IMM16_X1_HI: bfd_reloc_code_real = 2119;
pub const BFD_RELOC_TILEPRO_IMM16_X0_HI: bfd_reloc_code_real = 2118;
pub const BFD_RELOC_TILEPRO_IMM16_X1_LO: bfd_reloc_code_real = 2117;
pub const BFD_RELOC_TILEPRO_IMM16_X0_LO: bfd_reloc_code_real = 2116;
pub const BFD_RELOC_TILEPRO_IMM16_X1: bfd_reloc_code_real = 2115;
pub const BFD_RELOC_TILEPRO_IMM16_X0: bfd_reloc_code_real = 2114;
pub const BFD_RELOC_TILEPRO_MF_IMM15_X1: bfd_reloc_code_real = 2113;
pub const BFD_RELOC_TILEPRO_MT_IMM15_X1: bfd_reloc_code_real = 2112;
pub const BFD_RELOC_TILEPRO_DEST_IMM8_X1: bfd_reloc_code_real = 2111;
pub const BFD_RELOC_TILEPRO_IMM8_Y1: bfd_reloc_code_real = 2110;
pub const BFD_RELOC_TILEPRO_IMM8_X1: bfd_reloc_code_real = 2109;
pub const BFD_RELOC_TILEPRO_IMM8_Y0: bfd_reloc_code_real = 2108;
pub const BFD_RELOC_TILEPRO_IMM8_X0: bfd_reloc_code_real = 2107;
pub const BFD_RELOC_TILEPRO_JOFFLONG_X1_PLT: bfd_reloc_code_real = 2106;
pub const BFD_RELOC_TILEPRO_JOFFLONG_X1: bfd_reloc_code_real = 2105;
pub const BFD_RELOC_TILEPRO_BROFF_X1: bfd_reloc_code_real = 2104;
pub const BFD_RELOC_TILEPRO_RELATIVE: bfd_reloc_code_real = 2103;
pub const BFD_RELOC_TILEPRO_JMP_SLOT: bfd_reloc_code_real = 2102;
pub const BFD_RELOC_TILEPRO_GLOB_DAT: bfd_reloc_code_real = 2101;
pub const BFD_RELOC_TILEPRO_COPY: bfd_reloc_code_real = 2100;
pub const BFD_RELOC_AARCH64_TLSDESC_LD_LO12_NC: bfd_reloc_code_real = 2099;
pub const BFD_RELOC_AARCH64_TLSIE_LD_GOTTPREL_LO12_NC: bfd_reloc_code_real = 2098;
pub const BFD_RELOC_AARCH64_LD_GOT_LO12_NC: bfd_reloc_code_real = 2097;
pub const BFD_RELOC_AARCH64_TLSLE_LDST_TPREL_LO12_NC: bfd_reloc_code_real = 2096;
pub const BFD_RELOC_AARCH64_TLSLE_LDST_TPREL_LO12: bfd_reloc_code_real = 2095;
pub const BFD_RELOC_AARCH64_TLSLD_LDST_DTPREL_LO12_NC: bfd_reloc_code_real = 2094;
pub const BFD_RELOC_AARCH64_TLSLD_LDST_DTPREL_LO12: bfd_reloc_code_real = 2093;
pub const BFD_RELOC_AARCH64_LDST_LO12: bfd_reloc_code_real = 2092;
pub const BFD_RELOC_AARCH64_GAS_INTERNAL_FIXUP: bfd_reloc_code_real = 2091;
pub const BFD_RELOC_AARCH64_RELOC_END: bfd_reloc_code_real = 2090;
pub const BFD_RELOC_AARCH64_IRELATIVE: bfd_reloc_code_real = 2089;
pub const BFD_RELOC_AARCH64_TLSDESC: bfd_reloc_code_real = 2088;
pub const BFD_RELOC_AARCH64_TLS_TPREL: bfd_reloc_code_real = 2087;
pub const BFD_RELOC_AARCH64_TLS_DTPREL: bfd_reloc_code_real = 2086;
pub const BFD_RELOC_AARCH64_TLS_DTPMOD: bfd_reloc_code_real = 2085;
pub const BFD_RELOC_AARCH64_RELATIVE: bfd_reloc_code_real = 2084;
pub const BFD_RELOC_AARCH64_JUMP_SLOT: bfd_reloc_code_real = 2083;
pub const BFD_RELOC_AARCH64_GLOB_DAT: bfd_reloc_code_real = 2082;
pub const BFD_RELOC_AARCH64_COPY: bfd_reloc_code_real = 2081;
pub const BFD_RELOC_AARCH64_TLSDESC_CALL: bfd_reloc_code_real = 2080;
pub const BFD_RELOC_AARCH64_TLSDESC_ADD: bfd_reloc_code_real = 2079;
pub const BFD_RELOC_AARCH64_TLSDESC_LDR: bfd_reloc_code_real = 2078;
pub const BFD_RELOC_AARCH64_TLSDESC_OFF_G0_NC: bfd_reloc_code_real = 2077;
pub const BFD_RELOC_AARCH64_TLSDESC_OFF_G1: bfd_reloc_code_real = 2076;
pub const BFD_RELOC_AARCH64_TLSDESC_ADD_LO12: bfd_reloc_code_real = 2075;
pub const BFD_RELOC_AARCH64_TLSDESC_LD32_LO12_NC: bfd_reloc_code_real = 2074;
pub const BFD_RELOC_AARCH64_TLSDESC_LD64_LO12: bfd_reloc_code_real = 2073;
pub const BFD_RELOC_AARCH64_TLSDESC_ADR_PAGE21: bfd_reloc_code_real = 2072;
pub const BFD_RELOC_AARCH64_TLSDESC_ADR_PREL21: bfd_reloc_code_real = 2071;
pub const BFD_RELOC_AARCH64_TLSDESC_LD_PREL19: bfd_reloc_code_real = 2070;
pub const BFD_RELOC_AARCH64_TLSLE_LDST8_TPREL_LO12_NC: bfd_reloc_code_real = 2069;
pub const BFD_RELOC_AARCH64_TLSLE_LDST8_TPREL_LO12: bfd_reloc_code_real = 2068;
pub const BFD_RELOC_AARCH64_TLSLE_LDST64_TPREL_LO12_NC: bfd_reloc_code_real = 2067;
pub const BFD_RELOC_AARCH64_TLSLE_LDST64_TPREL_LO12: bfd_reloc_code_real = 2066;
pub const BFD_RELOC_AARCH64_TLSLE_LDST32_TPREL_LO12_NC: bfd_reloc_code_real = 2065;
pub const BFD_RELOC_AARCH64_TLSLE_LDST32_TPREL_LO12: bfd_reloc_code_real = 2064;
pub const BFD_RELOC_AARCH64_TLSLE_LDST16_TPREL_LO12_NC: bfd_reloc_code_real = 2063;
pub const BFD_RELOC_AARCH64_TLSLE_LDST16_TPREL_LO12: bfd_reloc_code_real = 2062;
pub const BFD_RELOC_AARCH64_TLSLE_ADD_TPREL_LO12_NC: bfd_reloc_code_real = 2061;
pub const BFD_RELOC_AARCH64_TLSLE_ADD_TPREL_LO12: bfd_reloc_code_real = 2060;
pub const BFD_RELOC_AARCH64_TLSLE_ADD_TPREL_HI12: bfd_reloc_code_real = 2059;
pub const BFD_RELOC_AARCH64_TLSLE_MOVW_TPREL_G0_NC: bfd_reloc_code_real = 2058;
pub const BFD_RELOC_AARCH64_TLSLE_MOVW_TPREL_G0: bfd_reloc_code_real = 2057;
pub const BFD_RELOC_AARCH64_TLSLE_MOVW_TPREL_G1_NC: bfd_reloc_code_real = 2056;
pub const BFD_RELOC_AARCH64_TLSLE_MOVW_TPREL_G1: bfd_reloc_code_real = 2055;
pub const BFD_RELOC_AARCH64_TLSLE_MOVW_TPREL_G2: bfd_reloc_code_real = 2054;
pub const BFD_RELOC_AARCH64_TLSLD_MOVW_DTPREL_G2: bfd_reloc_code_real = 2053;
pub const BFD_RELOC_AARCH64_TLSLD_MOVW_DTPREL_G1_NC: bfd_reloc_code_real = 2052;
pub const BFD_RELOC_AARCH64_TLSLD_MOVW_DTPREL_G1: bfd_reloc_code_real = 2051;
pub const BFD_RELOC_AARCH64_TLSLD_MOVW_DTPREL_G0_NC: bfd_reloc_code_real = 2050;
pub const BFD_RELOC_AARCH64_TLSLD_MOVW_DTPREL_G0: bfd_reloc_code_real = 2049;
pub const BFD_RELOC_AARCH64_TLSLD_LDST8_DTPREL_LO12_NC: bfd_reloc_code_real = 2048;
pub const BFD_RELOC_AARCH64_TLSLD_LDST8_DTPREL_LO12: bfd_reloc_code_real = 2047;
pub const BFD_RELOC_AARCH64_TLSLD_LDST64_DTPREL_LO12_NC: bfd_reloc_code_real = 2046;
pub const BFD_RELOC_AARCH64_TLSLD_LDST64_DTPREL_LO12: bfd_reloc_code_real = 2045;
pub const BFD_RELOC_AARCH64_TLSLD_LDST32_DTPREL_LO12_NC: bfd_reloc_code_real = 2044;
pub const BFD_RELOC_AARCH64_TLSLD_LDST32_DTPREL_LO12: bfd_reloc_code_real = 2043;
pub const BFD_RELOC_AARCH64_TLSLD_LDST16_DTPREL_LO12_NC: bfd_reloc_code_real = 2042;
pub const BFD_RELOC_AARCH64_TLSLD_LDST16_DTPREL_LO12: bfd_reloc_code_real = 2041;
pub const BFD_RELOC_AARCH64_TLSLD_ADR_PREL21: bfd_reloc_code_real = 2040;
pub const BFD_RELOC_AARCH64_TLSLD_ADR_PAGE21: bfd_reloc_code_real = 2039;
pub const BFD_RELOC_AARCH64_TLSLD_ADD_LO12_NC: bfd_reloc_code_real = 2038;
pub const BFD_RELOC_AARCH64_TLSLD_ADD_DTPREL_LO12_NC: bfd_reloc_code_real = 2037;
pub const BFD_RELOC_AARCH64_TLSLD_ADD_DTPREL_LO12: bfd_reloc_code_real = 2036;
pub const BFD_RELOC_AARCH64_TLSLD_ADD_DTPREL_HI12: bfd_reloc_code_real = 2035;
pub const BFD_RELOC_AARCH64_TLSIE_MOVW_GOTTPREL_G1: bfd_reloc_code_real = 2034;
pub const BFD_RELOC_AARCH64_TLSIE_MOVW_GOTTPREL_G0_NC: bfd_reloc_code_real = 2033;
pub const BFD_RELOC_AARCH64_TLSIE_LD_GOTTPREL_PREL19: bfd_reloc_code_real = 2032;
pub const BFD_RELOC_AARCH64_TLSIE_LD32_GOTTPREL_LO12_NC: bfd_reloc_code_real = 2031;
pub const BFD_RELOC_AARCH64_TLSIE_LD64_GOTTPREL_LO12_NC: bfd_reloc_code_real = 2030;
pub const BFD_RELOC_AARCH64_TLSIE_ADR_GOTTPREL_PAGE21: bfd_reloc_code_real = 2029;
pub const BFD_RELOC_AARCH64_TLSGD_MOVW_G1: bfd_reloc_code_real = 2028;
pub const BFD_RELOC_AARCH64_TLSGD_MOVW_G0_NC: bfd_reloc_code_real = 2027;
pub const BFD_RELOC_AARCH64_TLSGD_ADD_LO12_NC: bfd_reloc_code_real = 2026;
pub const BFD_RELOC_AARCH64_TLSGD_ADR_PREL21: bfd_reloc_code_real = 2025;
pub const BFD_RELOC_AARCH64_TLSGD_ADR_PAGE21: bfd_reloc_code_real = 2024;
pub const BFD_RELOC_AARCH64_LD64_GOTPAGE_LO15: bfd_reloc_code_real = 2023;
pub const BFD_RELOC_AARCH64_LD32_GOTPAGE_LO14: bfd_reloc_code_real = 2022;
pub const BFD_RELOC_AARCH64_LD64_GOTOFF_LO15: bfd_reloc_code_real = 2021;
pub const BFD_RELOC_AARCH64_MOVW_GOTOFF_G1: bfd_reloc_code_real = 2020;
pub const BFD_RELOC_AARCH64_MOVW_GOTOFF_G0_NC: bfd_reloc_code_real = 2019;
pub const BFD_RELOC_AARCH64_LD32_GOT_LO12_NC: bfd_reloc_code_real = 2018;
pub const BFD_RELOC_AARCH64_LD64_GOT_LO12_NC: bfd_reloc_code_real = 2017;
pub const BFD_RELOC_AARCH64_ADR_GOT_PAGE: bfd_reloc_code_real = 2016;
pub const BFD_RELOC_AARCH64_GOT_LD_PREL19: bfd_reloc_code_real = 2015;
pub const BFD_RELOC_AARCH64_LDST128_LO12: bfd_reloc_code_real = 2014;
pub const BFD_RELOC_AARCH64_LDST64_LO12: bfd_reloc_code_real = 2013;
pub const BFD_RELOC_AARCH64_LDST32_LO12: bfd_reloc_code_real = 2012;
pub const BFD_RELOC_AARCH64_LDST16_LO12: bfd_reloc_code_real = 2011;
pub const BFD_RELOC_AARCH64_CALL26: bfd_reloc_code_real = 2010;
pub const BFD_RELOC_AARCH64_JUMP26: bfd_reloc_code_real = 2009;
pub const BFD_RELOC_AARCH64_BRANCH19: bfd_reloc_code_real = 2008;
pub const BFD_RELOC_AARCH64_TSTBR14: bfd_reloc_code_real = 2007;
pub const BFD_RELOC_AARCH64_LDST8_LO12: bfd_reloc_code_real = 2006;
pub const BFD_RELOC_AARCH64_ADD_LO12: bfd_reloc_code_real = 2005;
pub const BFD_RELOC_AARCH64_ADR_HI21_NC_PCREL: bfd_reloc_code_real = 2004;
pub const BFD_RELOC_AARCH64_ADR_HI21_PCREL: bfd_reloc_code_real = 2003;
pub const BFD_RELOC_AARCH64_ADR_LO21_PCREL: bfd_reloc_code_real = 2002;
pub const BFD_RELOC_AARCH64_LD_LO19_PCREL: bfd_reloc_code_real = 2001;
pub const BFD_RELOC_AARCH64_MOVW_PREL_G3: bfd_reloc_code_real = 2000;
pub const BFD_RELOC_AARCH64_MOVW_PREL_G2_NC: bfd_reloc_code_real = 1999;
pub const BFD_RELOC_AARCH64_MOVW_PREL_G2: bfd_reloc_code_real = 1998;
pub const BFD_RELOC_AARCH64_MOVW_PREL_G1_NC: bfd_reloc_code_real = 1997;
pub const BFD_RELOC_AARCH64_MOVW_PREL_G1: bfd_reloc_code_real = 1996;
pub const BFD_RELOC_AARCH64_MOVW_PREL_G0_NC: bfd_reloc_code_real = 1995;
pub const BFD_RELOC_AARCH64_MOVW_PREL_G0: bfd_reloc_code_real = 1994;
pub const BFD_RELOC_AARCH64_MOVW_G2_S: bfd_reloc_code_real = 1993;
pub const BFD_RELOC_AARCH64_MOVW_G1_S: bfd_reloc_code_real = 1992;
pub const BFD_RELOC_AARCH64_MOVW_G0_S: bfd_reloc_code_real = 1991;
pub const BFD_RELOC_AARCH64_MOVW_G3: bfd_reloc_code_real = 1990;
pub const BFD_RELOC_AARCH64_MOVW_G2_NC: bfd_reloc_code_real = 1989;
pub const BFD_RELOC_AARCH64_MOVW_G2: bfd_reloc_code_real = 1988;
pub const BFD_RELOC_AARCH64_MOVW_G1_NC: bfd_reloc_code_real = 1987;
pub const BFD_RELOC_AARCH64_MOVW_G1: bfd_reloc_code_real = 1986;
pub const BFD_RELOC_AARCH64_MOVW_G0_NC: bfd_reloc_code_real = 1985;
pub const BFD_RELOC_AARCH64_MOVW_G0: bfd_reloc_code_real = 1984;
pub const BFD_RELOC_AARCH64_16_PCREL: bfd_reloc_code_real = 1983;
pub const BFD_RELOC_AARCH64_32_PCREL: bfd_reloc_code_real = 1982;
pub const BFD_RELOC_AARCH64_64_PCREL: bfd_reloc_code_real = 1981;
pub const BFD_RELOC_AARCH64_16: bfd_reloc_code_real = 1980;
pub const BFD_RELOC_AARCH64_32: bfd_reloc_code_real = 1979;
pub const BFD_RELOC_AARCH64_64: bfd_reloc_code_real = 1978;
pub const BFD_RELOC_AARCH64_NONE: bfd_reloc_code_real = 1977;
pub const BFD_RELOC_AARCH64_NULL: bfd_reloc_code_real = 1976;
pub const BFD_RELOC_AARCH64_RELOC_START: bfd_reloc_code_real = 1975;
pub const BFD_RELOC_MICROBLAZE_64_TEXTREL: bfd_reloc_code_real = 1974;
pub const BFD_RELOC_MICROBLAZE_64_TEXTPCREL: bfd_reloc_code_real = 1973;
pub const BFD_RELOC_MICROBLAZE_64_TLSTPREL: bfd_reloc_code_real = 1972;
pub const BFD_RELOC_MICROBLAZE_64_TLSGOTTPREL: bfd_reloc_code_real = 1971;
pub const BFD_RELOC_MICROBLAZE_64_TLSDTPREL: bfd_reloc_code_real = 1970;
pub const BFD_RELOC_MICROBLAZE_32_TLSDTPREL: bfd_reloc_code_real = 1969;
pub const BFD_RELOC_MICROBLAZE_32_TLSDTPMOD: bfd_reloc_code_real = 1968;
pub const BFD_RELOC_MICROBLAZE_64_TLSLD: bfd_reloc_code_real = 1967;
pub const BFD_RELOC_MICROBLAZE_64_TLSGD: bfd_reloc_code_real = 1966;
pub const BFD_RELOC_MICROBLAZE_64_TLS: bfd_reloc_code_real = 1965;
pub const BFD_RELOC_MICROBLAZE_COPY: bfd_reloc_code_real = 1964;
pub const BFD_RELOC_MICROBLAZE_32_GOTOFF: bfd_reloc_code_real = 1963;
pub const BFD_RELOC_MICROBLAZE_64_GOTOFF: bfd_reloc_code_real = 1962;
pub const BFD_RELOC_MICROBLAZE_64_PLT: bfd_reloc_code_real = 1961;
pub const BFD_RELOC_MICROBLAZE_64_GOT: bfd_reloc_code_real = 1960;
pub const BFD_RELOC_MICROBLAZE_64_GOTPC: bfd_reloc_code_real = 1959;
pub const BFD_RELOC_MICROBLAZE_64_NONE: bfd_reloc_code_real = 1958;
pub const BFD_RELOC_MICROBLAZE_32_SYM_OP_SYM: bfd_reloc_code_real = 1957;
pub const BFD_RELOC_MICROBLAZE_32_RWSDA: bfd_reloc_code_real = 1956;
pub const BFD_RELOC_MICROBLAZE_32_ROSDA: bfd_reloc_code_real = 1955;
pub const BFD_RELOC_MICROBLAZE_32_LO_PCREL: bfd_reloc_code_real = 1954;
pub const BFD_RELOC_MICROBLAZE_32_LO: bfd_reloc_code_real = 1953;
pub const BFD_RELOC_MACH_O_ARM64_POINTER_TO_GOT: bfd_reloc_code_real = 1952;
pub const BFD_RELOC_MACH_O_ARM64_GOT_LOAD_PAGEOFF12: bfd_reloc_code_real = 1951;
pub const BFD_RELOC_MACH_O_ARM64_GOT_LOAD_PAGE21: bfd_reloc_code_real = 1950;
pub const BFD_RELOC_MACH_O_ARM64_ADDEND: bfd_reloc_code_real = 1949;
pub const BFD_RELOC_MACH_O_X86_64_TLV: bfd_reloc_code_real = 1948;
pub const BFD_RELOC_MACH_O_X86_64_PCREL32_4: bfd_reloc_code_real = 1947;
pub const BFD_RELOC_MACH_O_X86_64_PCREL32_2: bfd_reloc_code_real = 1946;
pub const BFD_RELOC_MACH_O_X86_64_PCREL32_1: bfd_reloc_code_real = 1945;
pub const BFD_RELOC_MACH_O_X86_64_GOT_LOAD: bfd_reloc_code_real = 1944;
pub const BFD_RELOC_MACH_O_X86_64_GOT: bfd_reloc_code_real = 1943;
pub const BFD_RELOC_MACH_O_X86_64_BRANCH8: bfd_reloc_code_real = 1942;
pub const BFD_RELOC_MACH_O_X86_64_BRANCH32: bfd_reloc_code_real = 1941;
pub const BFD_RELOC_MACH_O_SUBTRACTOR64: bfd_reloc_code_real = 1940;
pub const BFD_RELOC_MACH_O_SUBTRACTOR32: bfd_reloc_code_real = 1939;
pub const BFD_RELOC_MACH_O_PAIR: bfd_reloc_code_real = 1938;
pub const BFD_RELOC_MACH_O_LOCAL_SECTDIFF: bfd_reloc_code_real = 1937;
pub const BFD_RELOC_MACH_O_SECTDIFF: bfd_reloc_code_real = 1936;
pub const BFD_RELOC_LM32_RELATIVE: bfd_reloc_code_real = 1935;
pub const BFD_RELOC_LM32_JMP_SLOT: bfd_reloc_code_real = 1934;
pub const BFD_RELOC_LM32_GLOB_DAT: bfd_reloc_code_real = 1933;
pub const BFD_RELOC_LM32_COPY: bfd_reloc_code_real = 1932;
pub const BFD_RELOC_LM32_GOTOFF_LO16: bfd_reloc_code_real = 1931;
pub const BFD_RELOC_LM32_GOTOFF_HI16: bfd_reloc_code_real = 1930;
pub const BFD_RELOC_LM32_16_GOT: bfd_reloc_code_real = 1929;
pub const BFD_RELOC_LM32_BRANCH: bfd_reloc_code_real = 1928;
pub const BFD_RELOC_LM32_CALL: bfd_reloc_code_real = 1927;
pub const BFD_RELOC_Z8K_IMM4L: bfd_reloc_code_real = 1926;
pub const BFD_RELOC_Z8K_CALLR: bfd_reloc_code_real = 1925;
pub const BFD_RELOC_Z8K_DISP7: bfd_reloc_code_real = 1924;
pub const BFD_RELOC_Z80_16_BE: bfd_reloc_code_real = 1923;
pub const BFD_RELOC_Z80_WORD1: bfd_reloc_code_real = 1922;
pub const BFD_RELOC_Z80_WORD0: bfd_reloc_code_real = 1921;
pub const BFD_RELOC_Z80_BYTE3: bfd_reloc_code_real = 1920;
pub const BFD_RELOC_Z80_BYTE2: bfd_reloc_code_real = 1919;
pub const BFD_RELOC_Z80_BYTE1: bfd_reloc_code_real = 1918;
pub const BFD_RELOC_Z80_BYTE0: bfd_reloc_code_real = 1917;
pub const BFD_RELOC_Z80_DISP8: bfd_reloc_code_real = 1916;
pub const BFD_RELOC_XTENSA_NDIFF32: bfd_reloc_code_real = 1915;
pub const BFD_RELOC_XTENSA_NDIFF16: bfd_reloc_code_real = 1914;
pub const BFD_RELOC_XTENSA_NDIFF8: bfd_reloc_code_real = 1913;
pub const BFD_RELOC_XTENSA_PDIFF32: bfd_reloc_code_real = 1912;
pub const BFD_RELOC_XTENSA_PDIFF16: bfd_reloc_code_real = 1911;
pub const BFD_RELOC_XTENSA_PDIFF8: bfd_reloc_code_real = 1910;
pub const BFD_RELOC_XTENSA_TLS_CALL: bfd_reloc_code_real = 1909;
pub const BFD_RELOC_XTENSA_TLS_ARG: bfd_reloc_code_real = 1908;
pub const BFD_RELOC_XTENSA_TLS_FUNC: bfd_reloc_code_real = 1907;
pub const BFD_RELOC_XTENSA_TLS_TPOFF: bfd_reloc_code_real = 1906;
pub const BFD_RELOC_XTENSA_TLS_DTPOFF: bfd_reloc_code_real = 1905;
pub const BFD_RELOC_XTENSA_TLSDESC_ARG: bfd_reloc_code_real = 1904;
pub const BFD_RELOC_XTENSA_TLSDESC_FN: bfd_reloc_code_real = 1903;
pub const BFD_RELOC_XTENSA_ASM_SIMPLIFY: bfd_reloc_code_real = 1902;
pub const BFD_RELOC_XTENSA_ASM_EXPAND: bfd_reloc_code_real = 1901;
pub const BFD_RELOC_XTENSA_OP2: bfd_reloc_code_real = 1900;
pub const BFD_RELOC_XTENSA_OP1: bfd_reloc_code_real = 1899;
pub const BFD_RELOC_XTENSA_OP0: bfd_reloc_code_real = 1898;
pub const BFD_RELOC_XTENSA_SLOT14_ALT: bfd_reloc_code_real = 1897;
pub const BFD_RELOC_XTENSA_SLOT13_ALT: bfd_reloc_code_real = 1896;
pub const BFD_RELOC_XTENSA_SLOT12_ALT: bfd_reloc_code_real = 1895;
pub const BFD_RELOC_XTENSA_SLOT11_ALT: bfd_reloc_code_real = 1894;
pub const BFD_RELOC_XTENSA_SLOT10_ALT: bfd_reloc_code_real = 1893;
pub const BFD_RELOC_XTENSA_SLOT9_ALT: bfd_reloc_code_real = 1892;
pub const BFD_RELOC_XTENSA_SLOT8_ALT: bfd_reloc_code_real = 1891;
pub const BFD_RELOC_XTENSA_SLOT7_ALT: bfd_reloc_code_real = 1890;
pub const BFD_RELOC_XTENSA_SLOT6_ALT: bfd_reloc_code_real = 1889;
pub const BFD_RELOC_XTENSA_SLOT5_ALT: bfd_reloc_code_real = 1888;
pub const BFD_RELOC_XTENSA_SLOT4_ALT: bfd_reloc_code_real = 1887;
pub const BFD_RELOC_XTENSA_SLOT3_ALT: bfd_reloc_code_real = 1886;
pub const BFD_RELOC_XTENSA_SLOT2_ALT: bfd_reloc_code_real = 1885;
pub const BFD_RELOC_XTENSA_SLOT1_ALT: bfd_reloc_code_real = 1884;
pub const BFD_RELOC_XTENSA_SLOT0_ALT: bfd_reloc_code_real = 1883;
pub const BFD_RELOC_XTENSA_SLOT14_OP: bfd_reloc_code_real = 1882;
pub const BFD_RELOC_XTENSA_SLOT13_OP: bfd_reloc_code_real = 1881;
pub const BFD_RELOC_XTENSA_SLOT12_OP: bfd_reloc_code_real = 1880;
pub const BFD_RELOC_XTENSA_SLOT11_OP: bfd_reloc_code_real = 1879;
pub const BFD_RELOC_XTENSA_SLOT10_OP: bfd_reloc_code_real = 1878;
pub const BFD_RELOC_XTENSA_SLOT9_OP: bfd_reloc_code_real = 1877;
pub const BFD_RELOC_XTENSA_SLOT8_OP: bfd_reloc_code_real = 1876;
pub const BFD_RELOC_XTENSA_SLOT7_OP: bfd_reloc_code_real = 1875;
pub const BFD_RELOC_XTENSA_SLOT6_OP: bfd_reloc_code_real = 1874;
pub const BFD_RELOC_XTENSA_SLOT5_OP: bfd_reloc_code_real = 1873;
pub const BFD_RELOC_XTENSA_SLOT4_OP: bfd_reloc_code_real = 1872;
pub const BFD_RELOC_XTENSA_SLOT3_OP: bfd_reloc_code_real = 1871;
pub const BFD_RELOC_XTENSA_SLOT2_OP: bfd_reloc_code_real = 1870;
pub const BFD_RELOC_XTENSA_SLOT1_OP: bfd_reloc_code_real = 1869;
pub const BFD_RELOC_XTENSA_SLOT0_OP: bfd_reloc_code_real = 1868;
pub const BFD_RELOC_XTENSA_DIFF32: bfd_reloc_code_real = 1867;
pub const BFD_RELOC_XTENSA_DIFF16: bfd_reloc_code_real = 1866;
pub const BFD_RELOC_XTENSA_DIFF8: bfd_reloc_code_real = 1865;
pub const BFD_RELOC_XTENSA_PLT: bfd_reloc_code_real = 1864;
pub const BFD_RELOC_XTENSA_RELATIVE: bfd_reloc_code_real = 1863;
pub const BFD_RELOC_XTENSA_JMP_SLOT: bfd_reloc_code_real = 1862;
pub const BFD_RELOC_XTENSA_GLOB_DAT: bfd_reloc_code_real = 1861;
pub const BFD_RELOC_XTENSA_RTLD: bfd_reloc_code_real = 1860;
pub const BFD_RELOC_IQ2000_UHI16: bfd_reloc_code_real = 1859;
pub const BFD_RELOC_IQ2000_OFFSET_21: bfd_reloc_code_real = 1858;
pub const BFD_RELOC_IQ2000_OFFSET_16: bfd_reloc_code_real = 1857;
pub const BFD_RELOC_PRU_GNU_DIFF32_PMEM: bfd_reloc_code_real = 1856;
pub const BFD_RELOC_PRU_GNU_DIFF16_PMEM: bfd_reloc_code_real = 1855;
pub const BFD_RELOC_PRU_GNU_DIFF32: bfd_reloc_code_real = 1854;
pub const BFD_RELOC_PRU_GNU_DIFF16: bfd_reloc_code_real = 1853;
pub const BFD_RELOC_PRU_GNU_DIFF8: bfd_reloc_code_real = 1852;
pub const BFD_RELOC_PRU_16_PMEM: bfd_reloc_code_real = 1851;
pub const BFD_RELOC_PRU_32_PMEM: bfd_reloc_code_real = 1850;
pub const BFD_RELOC_PRU_U8_PCREL: bfd_reloc_code_real = 1849;
pub const BFD_RELOC_PRU_S10_PCREL: bfd_reloc_code_real = 1848;
pub const BFD_RELOC_PRU_LDI32: bfd_reloc_code_real = 1847;
pub const BFD_RELOC_PRU_U16_PMEMIMM: bfd_reloc_code_real = 1846;
pub const BFD_RELOC_PRU_U16: bfd_reloc_code_real = 1845;
pub const BFD_RELOC_NIOS2_R2_T1X1I6_2: bfd_reloc_code_real = 1844;
pub const BFD_RELOC_NIOS2_R2_T1X1I6: bfd_reloc_code_real = 1843;
pub const BFD_RELOC_NIOS2_R2_L5I4X1: bfd_reloc_code_real = 1842;
pub const BFD_RELOC_NIOS2_R2_F1I5_2: bfd_reloc_code_real = 1841;
pub const BFD_RELOC_NIOS2_R2_X2L5: bfd_reloc_code_real = 1840;
pub const BFD_RELOC_NIOS2_R2_X1I7_2: bfd_reloc_code_real = 1839;
pub const BFD_RELOC_NIOS2_R2_T2I4_2: bfd_reloc_code_real = 1838;
pub const BFD_RELOC_NIOS2_R2_T2I4_1: bfd_reloc_code_real = 1837;
pub const BFD_RELOC_NIOS2_R2_T2I4: bfd_reloc_code_real = 1836;
pub const BFD_RELOC_NIOS2_R2_T1I7_2: bfd_reloc_code_real = 1835;
pub const BFD_RELOC_NIOS2_R2_T1I7_1_PCREL: bfd_reloc_code_real = 1834;
pub const BFD_RELOC_NIOS2_R2_I10_1_PCREL: bfd_reloc_code_real = 1833;
pub const BFD_RELOC_NIOS2_R2_S12: bfd_reloc_code_real = 1832;
pub const BFD_RELOC_NIOS2_CALL_HA: bfd_reloc_code_real = 1831;
pub const BFD_RELOC_NIOS2_CALL_LO: bfd_reloc_code_real = 1830;
pub const BFD_RELOC_NIOS2_GOT_HA: bfd_reloc_code_real = 1829;
pub const BFD_RELOC_NIOS2_GOT_LO: bfd_reloc_code_real = 1828;
pub const BFD_RELOC_NIOS2_CALL26_NOAT: bfd_reloc_code_real = 1827;
pub const BFD_RELOC_NIOS2_GOTOFF: bfd_reloc_code_real = 1826;
pub const BFD_RELOC_NIOS2_RELATIVE: bfd_reloc_code_real = 1825;
pub const BFD_RELOC_NIOS2_JUMP_SLOT: bfd_reloc_code_real = 1824;
pub const BFD_RELOC_NIOS2_GLOB_DAT: bfd_reloc_code_real = 1823;
pub const BFD_RELOC_NIOS2_COPY: bfd_reloc_code_real = 1822;
pub const BFD_RELOC_NIOS2_TLS_TPREL: bfd_reloc_code_real = 1821;
pub const BFD_RELOC_NIOS2_TLS_DTPREL: bfd_reloc_code_real = 1820;
pub const BFD_RELOC_NIOS2_TLS_DTPMOD: bfd_reloc_code_real = 1819;
pub const BFD_RELOC_NIOS2_TLS_LE16: bfd_reloc_code_real = 1818;
pub const BFD_RELOC_NIOS2_TLS_IE16: bfd_reloc_code_real = 1817;
pub const BFD_RELOC_NIOS2_TLS_LDO16: bfd_reloc_code_real = 1816;
pub const BFD_RELOC_NIOS2_TLS_LDM16: bfd_reloc_code_real = 1815;
pub const BFD_RELOC_NIOS2_TLS_GD16: bfd_reloc_code_real = 1814;
pub const BFD_RELOC_NIOS2_PCREL_HA: bfd_reloc_code_real = 1813;
pub const BFD_RELOC_NIOS2_PCREL_LO: bfd_reloc_code_real = 1812;
pub const BFD_RELOC_NIOS2_GOTOFF_HA: bfd_reloc_code_real = 1811;
pub const BFD_RELOC_NIOS2_GOTOFF_LO: bfd_reloc_code_real = 1810;
pub const BFD_RELOC_NIOS2_CALL16: bfd_reloc_code_real = 1809;
pub const BFD_RELOC_NIOS2_GOT16: bfd_reloc_code_real = 1808;
pub const BFD_RELOC_NIOS2_ALIGN: bfd_reloc_code_real = 1807;
pub const BFD_RELOC_NIOS2_CALLR: bfd_reloc_code_real = 1806;
pub const BFD_RELOC_NIOS2_CJMP: bfd_reloc_code_real = 1805;
pub const BFD_RELOC_NIOS2_UJMP: bfd_reloc_code_real = 1804;
pub const BFD_RELOC_NIOS2_GPREL: bfd_reloc_code_real = 1803;
pub const BFD_RELOC_NIOS2_HIADJ16: bfd_reloc_code_real = 1802;
pub const BFD_RELOC_NIOS2_LO16: bfd_reloc_code_real = 1801;
pub const BFD_RELOC_NIOS2_HI16: bfd_reloc_code_real = 1800;
pub const BFD_RELOC_NIOS2_IMM8: bfd_reloc_code_real = 1799;
pub const BFD_RELOC_NIOS2_IMM6: bfd_reloc_code_real = 1798;
pub const BFD_RELOC_NIOS2_CACHE_OPX: bfd_reloc_code_real = 1797;
pub const BFD_RELOC_NIOS2_IMM5: bfd_reloc_code_real = 1796;
pub const BFD_RELOC_NIOS2_CALL26: bfd_reloc_code_real = 1795;
pub const BFD_RELOC_NIOS2_U16: bfd_reloc_code_real = 1794;
pub const BFD_RELOC_NIOS2_S16: bfd_reloc_code_real = 1793;
pub const BFD_RELOC_MSP430_SUB_ULEB128: bfd_reloc_code_real = 1792;
pub const BFD_RELOC_MSP430_SET_ULEB128: bfd_reloc_code_real = 1791;
pub const BFD_RELOC_MSP430_SYM_DIFF: bfd_reloc_code_real = 1790;
pub const BFD_RELOC_MSP430_PREL31: bfd_reloc_code_real = 1789;
pub const BFD_RELOC_MSP430_ABS_HI16: bfd_reloc_code_real = 1788;
pub const BFD_RELOC_MSP430X_ABS16: bfd_reloc_code_real = 1787;
pub const BFD_RELOC_MSP430X_PCR20_CALL: bfd_reloc_code_real = 1786;
pub const BFD_RELOC_MSP430X_PCR16: bfd_reloc_code_real = 1785;
pub const BFD_RELOC_MSP430X_ABS20_ADR_DST: bfd_reloc_code_real = 1784;
pub const BFD_RELOC_MSP430X_ABS20_ADR_SRC: bfd_reloc_code_real = 1783;
pub const BFD_RELOC_MSP430X_ABS20_EXT_ODST: bfd_reloc_code_real = 1782;
pub const BFD_RELOC_MSP430X_ABS20_EXT_DST: bfd_reloc_code_real = 1781;
pub const BFD_RELOC_MSP430X_ABS20_EXT_SRC: bfd_reloc_code_real = 1780;
pub const BFD_RELOC_MSP430X_PCR20_EXT_ODST: bfd_reloc_code_real = 1779;
pub const BFD_RELOC_MSP430X_PCR20_EXT_DST: bfd_reloc_code_real = 1778;
pub const BFD_RELOC_MSP430X_PCR20_EXT_SRC: bfd_reloc_code_real = 1777;
pub const BFD_RELOC_MSP430_ABS8: bfd_reloc_code_real = 1776;
pub const BFD_RELOC_MSP430_RL_PCREL: bfd_reloc_code_real = 1775;
pub const BFD_RELOC_MSP430_2X_PCREL: bfd_reloc_code_real = 1774;
pub const BFD_RELOC_MSP430_16_BYTE: bfd_reloc_code_real = 1773;
pub const BFD_RELOC_MSP430_16_PCREL_BYTE: bfd_reloc_code_real = 1772;
pub const BFD_RELOC_MSP430_16: bfd_reloc_code_real = 1771;
pub const BFD_RELOC_MSP430_16_PCREL: bfd_reloc_code_real = 1770;
pub const BFD_RELOC_MSP430_10_PCREL: bfd_reloc_code_real = 1769;
pub const BFD_RELOC_MT_PCINSN8: bfd_reloc_code_real = 1768;
pub const BFD_RELOC_MT_GNU_VTENTRY: bfd_reloc_code_real = 1767;
pub const BFD_RELOC_MT_GNU_VTINHERIT: bfd_reloc_code_real = 1766;
pub const BFD_RELOC_MT_LO16: bfd_reloc_code_real = 1765;
pub const BFD_RELOC_MT_HI16: bfd_reloc_code_real = 1764;
pub const BFD_RELOC_MT_PC16: bfd_reloc_code_real = 1763;
pub const BFD_RELOC_VAX_RELATIVE: bfd_reloc_code_real = 1762;
pub const BFD_RELOC_VAX_JMP_SLOT: bfd_reloc_code_real = 1761;
pub const BFD_RELOC_VAX_GLOB_DAT: bfd_reloc_code_real = 1760;
pub const BFD_RELOC_XC16X_SOF: bfd_reloc_code_real = 1759;
pub const BFD_RELOC_XC16X_SEG: bfd_reloc_code_real = 1758;
pub const BFD_RELOC_XC16X_POF: bfd_reloc_code_real = 1757;
pub const BFD_RELOC_XC16X_PAG: bfd_reloc_code_real = 1756;
pub const BFD_RELOC_RELC: bfd_reloc_code_real = 1755;
pub const BFD_RELOC_XSTORMY16_FPTR16: bfd_reloc_code_real = 1754;
pub const BFD_RELOC_XSTORMY16_24: bfd_reloc_code_real = 1753;
pub const BFD_RELOC_XSTORMY16_12: bfd_reloc_code_real = 1752;
pub const BFD_RELOC_XSTORMY16_REL_12: bfd_reloc_code_real = 1751;
pub const BFD_RELOC_H8_DISP32A16: bfd_reloc_code_real = 1750;
pub const BFD_RELOC_H8_DIR32A16: bfd_reloc_code_real = 1749;
pub const BFD_RELOC_H8_DIR24R8: bfd_reloc_code_real = 1748;
pub const BFD_RELOC_H8_DIR24A8: bfd_reloc_code_real = 1747;
pub const BFD_RELOC_H8_DIR16R8: bfd_reloc_code_real = 1746;
pub const BFD_RELOC_H8_DIR16A8: bfd_reloc_code_real = 1745;
pub const BFD_RELOC_OR1K_TLS_DTPMOD: bfd_reloc_code_real = 1744;
pub const BFD_RELOC_OR1K_TLS_DTPOFF: bfd_reloc_code_real = 1743;
pub const BFD_RELOC_OR1K_TLS_TPOFF: bfd_reloc_code_real = 1742;
pub const BFD_RELOC_OR1K_TLS_LE_SLO16: bfd_reloc_code_real = 1741;
pub const BFD_RELOC_OR1K_TLS_LE_LO16: bfd_reloc_code_real = 1740;
pub const BFD_RELOC_OR1K_TLS_LE_AHI16: bfd_reloc_code_real = 1739;
pub const BFD_RELOC_OR1K_TLS_LE_HI16: bfd_reloc_code_real = 1738;
pub const BFD_RELOC_OR1K_TLS_IE_LO13: bfd_reloc_code_real = 1737;
pub const BFD_RELOC_OR1K_TLS_IE_PG21: bfd_reloc_code_real = 1736;
pub const BFD_RELOC_OR1K_TLS_IE_LO16: bfd_reloc_code_real = 1735;
pub const BFD_RELOC_OR1K_TLS_IE_AHI16: bfd_reloc_code_real = 1734;
pub const BFD_RELOC_OR1K_TLS_IE_HI16: bfd_reloc_code_real = 1733;
pub const BFD_RELOC_OR1K_TLS_LDO_LO16: bfd_reloc_code_real = 1732;
pub const BFD_RELOC_OR1K_TLS_LDO_HI16: bfd_reloc_code_real = 1731;
pub const BFD_RELOC_OR1K_TLS_LDM_LO13: bfd_reloc_code_real = 1730;
pub const BFD_RELOC_OR1K_TLS_LDM_PG21: bfd_reloc_code_real = 1729;
pub const BFD_RELOC_OR1K_TLS_LDM_LO16: bfd_reloc_code_real = 1728;
pub const BFD_RELOC_OR1K_TLS_LDM_HI16: bfd_reloc_code_real = 1727;
pub const BFD_RELOC_OR1K_TLS_GD_LO13: bfd_reloc_code_real = 1726;
pub const BFD_RELOC_OR1K_TLS_GD_PG21: bfd_reloc_code_real = 1725;
pub const BFD_RELOC_OR1K_TLS_GD_LO16: bfd_reloc_code_real = 1724;
pub const BFD_RELOC_OR1K_TLS_GD_HI16: bfd_reloc_code_real = 1723;
pub const BFD_RELOC_OR1K_RELATIVE: bfd_reloc_code_real = 1722;
pub const BFD_RELOC_OR1K_JMP_SLOT: bfd_reloc_code_real = 1721;
pub const BFD_RELOC_OR1K_GLOB_DAT: bfd_reloc_code_real = 1720;
pub const BFD_RELOC_OR1K_COPY: bfd_reloc_code_real = 1719;
pub const BFD_RELOC_OR1K_GOTOFF_SLO16: bfd_reloc_code_real = 1718;
pub const BFD_RELOC_OR1K_PLTA26: bfd_reloc_code_real = 1717;
pub const BFD_RELOC_OR1K_PLT26: bfd_reloc_code_real = 1716;
pub const BFD_RELOC_OR1K_GOT_LO13: bfd_reloc_code_real = 1715;
pub const BFD_RELOC_OR1K_GOT_PG21: bfd_reloc_code_real = 1714;
pub const BFD_RELOC_OR1K_GOT16: bfd_reloc_code_real = 1713;
pub const BFD_RELOC_OR1K_GOT_AHI16: bfd_reloc_code_real = 1712;
pub const BFD_RELOC_OR1K_GOTPC_LO16: bfd_reloc_code_real = 1711;
pub const BFD_RELOC_OR1K_GOTPC_HI16: bfd_reloc_code_real = 1710;
pub const BFD_RELOC_OR1K_SLO13: bfd_reloc_code_real = 1709;
pub const BFD_RELOC_OR1K_LO13: bfd_reloc_code_real = 1708;
pub const BFD_RELOC_OR1K_PCREL_PG21: bfd_reloc_code_real = 1707;
pub const BFD_RELOC_OR1K_SLO16: bfd_reloc_code_real = 1706;
pub const BFD_RELOC_OR1K_REL_26: bfd_reloc_code_real = 1705;
pub const BFD_RELOC_CRIS_32_IE: bfd_reloc_code_real = 1704;
pub const BFD_RELOC_CRIS_DTPMOD: bfd_reloc_code_real = 1703;
pub const BFD_RELOC_CRIS_16_TPREL: bfd_reloc_code_real = 1702;
pub const BFD_RELOC_CRIS_32_TPREL: bfd_reloc_code_real = 1701;
pub const BFD_RELOC_CRIS_16_GOT_TPREL: bfd_reloc_code_real = 1700;
pub const BFD_RELOC_CRIS_32_GOT_TPREL: bfd_reloc_code_real = 1699;
pub const BFD_RELOC_CRIS_16_DTPREL: bfd_reloc_code_real = 1698;
pub const BFD_RELOC_CRIS_32_DTPREL: bfd_reloc_code_real = 1697;
pub const BFD_RELOC_CRIS_DTP: bfd_reloc_code_real = 1696;
pub const BFD_RELOC_CRIS_32_GD: bfd_reloc_code_real = 1695;
pub const BFD_RELOC_CRIS_16_GOT_GD: bfd_reloc_code_real = 1694;
pub const BFD_RELOC_CRIS_32_GOT_GD: bfd_reloc_code_real = 1693;
pub const BFD_RELOC_CRIS_32_PLT_PCREL: bfd_reloc_code_real = 1692;
pub const BFD_RELOC_CRIS_32_PLT_GOTREL: bfd_reloc_code_real = 1691;
pub const BFD_RELOC_CRIS_32_GOTREL: bfd_reloc_code_real = 1690;
pub const BFD_RELOC_CRIS_16_GOTPLT: bfd_reloc_code_real = 1689;
pub const BFD_RELOC_CRIS_32_GOTPLT: bfd_reloc_code_real = 1688;
pub const BFD_RELOC_CRIS_16_GOT: bfd_reloc_code_real = 1687;
pub const BFD_RELOC_CRIS_32_GOT: bfd_reloc_code_real = 1686;
pub const BFD_RELOC_CRIS_RELATIVE: bfd_reloc_code_real = 1685;
pub const BFD_RELOC_CRIS_JUMP_SLOT: bfd_reloc_code_real = 1684;
pub const BFD_RELOC_CRIS_GLOB_DAT: bfd_reloc_code_real = 1683;
pub const BFD_RELOC_CRIS_COPY: bfd_reloc_code_real = 1682;
pub const BFD_RELOC_CRIS_UNSIGNED_4: bfd_reloc_code_real = 1681;
pub const BFD_RELOC_CRIS_LAPCQ_OFFSET: bfd_reloc_code_real = 1680;
pub const BFD_RELOC_CRIS_UNSIGNED_16: bfd_reloc_code_real = 1679;
pub const BFD_RELOC_CRIS_SIGNED_16: bfd_reloc_code_real = 1678;
pub const BFD_RELOC_CRIS_UNSIGNED_8: bfd_reloc_code_real = 1677;
pub const BFD_RELOC_CRIS_SIGNED_8: bfd_reloc_code_real = 1676;
pub const BFD_RELOC_CRIS_UNSIGNED_6: bfd_reloc_code_real = 1675;
pub const BFD_RELOC_CRIS_SIGNED_6: bfd_reloc_code_real = 1674;
pub const BFD_RELOC_CRIS_UNSIGNED_5: bfd_reloc_code_real = 1673;
pub const BFD_RELOC_CRIS_BDISP8: bfd_reloc_code_real = 1672;
pub const BFD_RELOC_CRX_SWITCH32: bfd_reloc_code_real = 1671;
pub const BFD_RELOC_CRX_SWITCH16: bfd_reloc_code_real = 1670;
pub const BFD_RELOC_CRX_SWITCH8: bfd_reloc_code_real = 1669;
pub const BFD_RELOC_CRX_IMM32: bfd_reloc_code_real = 1668;
pub const BFD_RELOC_CRX_IMM16: bfd_reloc_code_real = 1667;
pub const BFD_RELOC_CRX_NUM32: bfd_reloc_code_real = 1666;
pub const BFD_RELOC_CRX_NUM16: bfd_reloc_code_real = 1665;
pub const BFD_RELOC_CRX_NUM8: bfd_reloc_code_real = 1664;
pub const BFD_RELOC_CRX_ABS32: bfd_reloc_code_real = 1663;
pub const BFD_RELOC_CRX_ABS16: bfd_reloc_code_real = 1662;
pub const BFD_RELOC_CRX_REGREL32: bfd_reloc_code_real = 1661;
pub const BFD_RELOC_CRX_REGREL28: bfd_reloc_code_real = 1660;
pub const BFD_RELOC_CRX_REGREL22: bfd_reloc_code_real = 1659;
pub const BFD_RELOC_CRX_REGREL12: bfd_reloc_code_real = 1658;
pub const BFD_RELOC_CRX_REL32: bfd_reloc_code_real = 1657;
pub const BFD_RELOC_CRX_REL24: bfd_reloc_code_real = 1656;
pub const BFD_RELOC_CRX_REL16: bfd_reloc_code_real = 1655;
pub const BFD_RELOC_CRX_REL8_CMP: bfd_reloc_code_real = 1654;
pub const BFD_RELOC_CRX_REL8: bfd_reloc_code_real = 1653;
pub const BFD_RELOC_CRX_REL4: bfd_reloc_code_real = 1652;
pub const BFD_RELOC_CR16_GLOB_DAT: bfd_reloc_code_real = 1651;
pub const BFD_RELOC_CR16_GOTC_REGREL20: bfd_reloc_code_real = 1650;
pub const BFD_RELOC_CR16_GOT_REGREL20: bfd_reloc_code_real = 1649;
pub const BFD_RELOC_CR16_SWITCH32: bfd_reloc_code_real = 1648;
pub const BFD_RELOC_CR16_SWITCH16: bfd_reloc_code_real = 1647;
pub const BFD_RELOC_CR16_SWITCH8: bfd_reloc_code_real = 1646;
pub const BFD_RELOC_CR16_DISP24a: bfd_reloc_code_real = 1645;
pub const BFD_RELOC_CR16_DISP24: bfd_reloc_code_real = 1644;
pub const BFD_RELOC_CR16_DISP20: bfd_reloc_code_real = 1643;
pub const BFD_RELOC_CR16_DISP16: bfd_reloc_code_real = 1642;
pub const BFD_RELOC_CR16_DISP8: bfd_reloc_code_real = 1641;
pub const BFD_RELOC_CR16_DISP4: bfd_reloc_code_real = 1640;
pub const BFD_RELOC_CR16_IMM32a: bfd_reloc_code_real = 1639;
pub const BFD_RELOC_CR16_IMM32: bfd_reloc_code_real = 1638;
pub const BFD_RELOC_CR16_IMM24: bfd_reloc_code_real = 1637;
pub const BFD_RELOC_CR16_IMM20: bfd_reloc_code_real = 1636;
pub const BFD_RELOC_CR16_IMM16: bfd_reloc_code_real = 1635;
pub const BFD_RELOC_CR16_IMM8: bfd_reloc_code_real = 1634;
pub const BFD_RELOC_CR16_IMM4: bfd_reloc_code_real = 1633;
pub const BFD_RELOC_CR16_ABS24: bfd_reloc_code_real = 1632;
pub const BFD_RELOC_CR16_ABS20: bfd_reloc_code_real = 1631;
pub const BFD_RELOC_CR16_REGREL20a: bfd_reloc_code_real = 1630;
pub const BFD_RELOC_CR16_REGREL20: bfd_reloc_code_real = 1629;
pub const BFD_RELOC_CR16_REGREL16: bfd_reloc_code_real = 1628;
pub const BFD_RELOC_CR16_REGREL14a: bfd_reloc_code_real = 1627;
pub const BFD_RELOC_CR16_REGREL14: bfd_reloc_code_real = 1626;
pub const BFD_RELOC_CR16_REGREL4a: bfd_reloc_code_real = 1625;
pub const BFD_RELOC_CR16_REGREL4: bfd_reloc_code_real = 1624;
pub const BFD_RELOC_CR16_REGREL0: bfd_reloc_code_real = 1623;
pub const BFD_RELOC_CR16_NUM32a: bfd_reloc_code_real = 1622;
pub const BFD_RELOC_CR16_NUM32: bfd_reloc_code_real = 1621;
pub const BFD_RELOC_CR16_NUM16: bfd_reloc_code_real = 1620;
pub const BFD_RELOC_CR16_NUM8: bfd_reloc_code_real = 1619;
pub const BFD_RELOC_S12Z_15_PCREL: bfd_reloc_code_real = 1618;
pub const BFD_RELOC_M68HC12_HI8XG: bfd_reloc_code_real = 1617;
pub const BFD_RELOC_M68HC12_LO8XG: bfd_reloc_code_real = 1616;
pub const BFD_RELOC_M68HC12_10_PCREL: bfd_reloc_code_real = 1615;
pub const BFD_RELOC_M68HC12_9_PCREL: bfd_reloc_code_real = 1614;
pub const BFD_RELOC_M68HC12_16B: bfd_reloc_code_real = 1613;
pub const BFD_RELOC_M68HC12_9B: bfd_reloc_code_real = 1612;
pub const BFD_RELOC_XGATE_IMM5: bfd_reloc_code_real = 1611;
pub const BFD_RELOC_XGATE_IMM4: bfd_reloc_code_real = 1610;
pub const BFD_RELOC_XGATE_IMM3: bfd_reloc_code_real = 1609;
pub const BFD_RELOC_XGATE_IMM8_HI: bfd_reloc_code_real = 1608;
pub const BFD_RELOC_XGATE_IMM8_LO: bfd_reloc_code_real = 1607;
pub const BFD_RELOC_XGATE_PCREL_10: bfd_reloc_code_real = 1606;
pub const BFD_RELOC_XGATE_PCREL_9: bfd_reloc_code_real = 1605;
pub const BFD_RELOC_XGATE_24: bfd_reloc_code_real = 1604;
pub const BFD_RELOC_XGATE_GPAGE: bfd_reloc_code_real = 1603;
pub const BFD_RELOC_XGATE_LO16: bfd_reloc_code_real = 1602;
pub const BFD_RELOC_XGATE_RL_GROUP: bfd_reloc_code_real = 1601;
pub const BFD_RELOC_XGATE_RL_JUMP: bfd_reloc_code_real = 1600;
pub const BFD_RELOC_M68HC12_5B: bfd_reloc_code_real = 1599;
pub const BFD_RELOC_M68HC11_24: bfd_reloc_code_real = 1598;
pub const BFD_RELOC_M68HC11_PAGE: bfd_reloc_code_real = 1597;
pub const BFD_RELOC_M68HC11_LO16: bfd_reloc_code_real = 1596;
pub const BFD_RELOC_M68HC11_RL_GROUP: bfd_reloc_code_real = 1595;
pub const BFD_RELOC_M68HC11_RL_JUMP: bfd_reloc_code_real = 1594;
pub const BFD_RELOC_M68HC11_3B: bfd_reloc_code_real = 1593;
pub const BFD_RELOC_M68HC11_LO8: bfd_reloc_code_real = 1592;
pub const BFD_RELOC_M68HC11_HI8: bfd_reloc_code_real = 1591;
pub const BFD_RELOC_IA64_LTOFF_DTPREL22: bfd_reloc_code_real = 1590;
pub const BFD_RELOC_IA64_DTPREL64LSB: bfd_reloc_code_real = 1589;
pub const BFD_RELOC_IA64_DTPREL64MSB: bfd_reloc_code_real = 1588;
pub const BFD_RELOC_IA64_DTPREL32LSB: bfd_reloc_code_real = 1587;
pub const BFD_RELOC_IA64_DTPREL32MSB: bfd_reloc_code_real = 1586;
pub const BFD_RELOC_IA64_DTPREL64I: bfd_reloc_code_real = 1585;
pub const BFD_RELOC_IA64_DTPREL22: bfd_reloc_code_real = 1584;
pub const BFD_RELOC_IA64_DTPREL14: bfd_reloc_code_real = 1583;
pub const BFD_RELOC_IA64_LTOFF_DTPMOD22: bfd_reloc_code_real = 1582;
pub const BFD_RELOC_IA64_DTPMOD64LSB: bfd_reloc_code_real = 1581;
pub const BFD_RELOC_IA64_DTPMOD64MSB: bfd_reloc_code_real = 1580;
pub const BFD_RELOC_IA64_LTOFF_TPREL22: bfd_reloc_code_real = 1579;
pub const BFD_RELOC_IA64_TPREL64LSB: bfd_reloc_code_real = 1578;
pub const BFD_RELOC_IA64_TPREL64MSB: bfd_reloc_code_real = 1577;
pub const BFD_RELOC_IA64_TPREL64I: bfd_reloc_code_real = 1576;
pub const BFD_RELOC_IA64_TPREL22: bfd_reloc_code_real = 1575;
pub const BFD_RELOC_IA64_TPREL14: bfd_reloc_code_real = 1574;
pub const BFD_RELOC_IA64_LDXMOV: bfd_reloc_code_real = 1573;
pub const BFD_RELOC_IA64_LTOFF22X: bfd_reloc_code_real = 1572;
pub const BFD_RELOC_IA64_COPY: bfd_reloc_code_real = 1571;
pub const BFD_RELOC_IA64_IPLTLSB: bfd_reloc_code_real = 1570;
pub const BFD_RELOC_IA64_IPLTMSB: bfd_reloc_code_real = 1569;
pub const BFD_RELOC_IA64_LTV64LSB: bfd_reloc_code_real = 1568;
pub const BFD_RELOC_IA64_LTV64MSB: bfd_reloc_code_real = 1567;
pub const BFD_RELOC_IA64_LTV32LSB: bfd_reloc_code_real = 1566;
pub const BFD_RELOC_IA64_LTV32MSB: bfd_reloc_code_real = 1565;
pub const BFD_RELOC_IA64_REL64LSB: bfd_reloc_code_real = 1564;
pub const BFD_RELOC_IA64_REL64MSB: bfd_reloc_code_real = 1563;
pub const BFD_RELOC_IA64_REL32LSB: bfd_reloc_code_real = 1562;
pub const BFD_RELOC_IA64_REL32MSB: bfd_reloc_code_real = 1561;
pub const BFD_RELOC_IA64_SECREL64LSB: bfd_reloc_code_real = 1560;
pub const BFD_RELOC_IA64_SECREL64MSB: bfd_reloc_code_real = 1559;
pub const BFD_RELOC_IA64_SECREL32LSB: bfd_reloc_code_real = 1558;
pub const BFD_RELOC_IA64_SECREL32MSB: bfd_reloc_code_real = 1557;
pub const BFD_RELOC_IA64_SEGREL64LSB: bfd_reloc_code_real = 1556;
pub const BFD_RELOC_IA64_SEGREL64MSB: bfd_reloc_code_real = 1555;
pub const BFD_RELOC_IA64_SEGREL32LSB: bfd_reloc_code_real = 1554;
pub const BFD_RELOC_IA64_SEGREL32MSB: bfd_reloc_code_real = 1553;
pub const BFD_RELOC_IA64_LTOFF_FPTR64LSB: bfd_reloc_code_real = 1552;
pub const BFD_RELOC_IA64_LTOFF_FPTR64MSB: bfd_reloc_code_real = 1551;
pub const BFD_RELOC_IA64_LTOFF_FPTR32LSB: bfd_reloc_code_real = 1550;
pub const BFD_RELOC_IA64_LTOFF_FPTR32MSB: bfd_reloc_code_real = 1549;
pub const BFD_RELOC_IA64_LTOFF_FPTR64I: bfd_reloc_code_real = 1548;
pub const BFD_RELOC_IA64_LTOFF_FPTR22: bfd_reloc_code_real = 1547;
pub const BFD_RELOC_IA64_PCREL64LSB: bfd_reloc_code_real = 1546;
pub const BFD_RELOC_IA64_PCREL64MSB: bfd_reloc_code_real = 1545;
pub const BFD_RELOC_IA64_PCREL32LSB: bfd_reloc_code_real = 1544;
pub const BFD_RELOC_IA64_PCREL32MSB: bfd_reloc_code_real = 1543;
pub const BFD_RELOC_IA64_PCREL64I: bfd_reloc_code_real = 1542;
pub const BFD_RELOC_IA64_PCREL60B: bfd_reloc_code_real = 1541;
pub const BFD_RELOC_IA64_PCREL22: bfd_reloc_code_real = 1540;
pub const BFD_RELOC_IA64_PCREL21F: bfd_reloc_code_real = 1539;
pub const BFD_RELOC_IA64_PCREL21M: bfd_reloc_code_real = 1538;
pub const BFD_RELOC_IA64_PCREL21BI: bfd_reloc_code_real = 1537;
pub const BFD_RELOC_IA64_PCREL21B: bfd_reloc_code_real = 1536;
pub const BFD_RELOC_IA64_FPTR64LSB: bfd_reloc_code_real = 1535;
pub const BFD_RELOC_IA64_FPTR64MSB: bfd_reloc_code_real = 1534;
pub const BFD_RELOC_IA64_FPTR32LSB: bfd_reloc_code_real = 1533;
pub const BFD_RELOC_IA64_FPTR32MSB: bfd_reloc_code_real = 1532;
pub const BFD_RELOC_IA64_FPTR64I: bfd_reloc_code_real = 1531;
pub const BFD_RELOC_IA64_PLTOFF64LSB: bfd_reloc_code_real = 1530;
pub const BFD_RELOC_IA64_PLTOFF64MSB: bfd_reloc_code_real = 1529;
pub const BFD_RELOC_IA64_PLTOFF64I: bfd_reloc_code_real = 1528;
pub const BFD_RELOC_IA64_PLTOFF22: bfd_reloc_code_real = 1527;
pub const BFD_RELOC_IA64_LTOFF64I: bfd_reloc_code_real = 1526;
pub const BFD_RELOC_IA64_LTOFF22: bfd_reloc_code_real = 1525;
pub const BFD_RELOC_IA64_GPREL64LSB: bfd_reloc_code_real = 1524;
pub const BFD_RELOC_IA64_GPREL64MSB: bfd_reloc_code_real = 1523;
pub const BFD_RELOC_IA64_GPREL32LSB: bfd_reloc_code_real = 1522;
pub const BFD_RELOC_IA64_GPREL32MSB: bfd_reloc_code_real = 1521;
pub const BFD_RELOC_IA64_GPREL64I: bfd_reloc_code_real = 1520;
pub const BFD_RELOC_IA64_GPREL22: bfd_reloc_code_real = 1519;
pub const BFD_RELOC_IA64_DIR64LSB: bfd_reloc_code_real = 1518;
pub const BFD_RELOC_IA64_DIR64MSB: bfd_reloc_code_real = 1517;
pub const BFD_RELOC_IA64_DIR32LSB: bfd_reloc_code_real = 1516;
pub const BFD_RELOC_IA64_DIR32MSB: bfd_reloc_code_real = 1515;
pub const BFD_RELOC_IA64_IMM64: bfd_reloc_code_real = 1514;
pub const BFD_RELOC_IA64_IMM22: bfd_reloc_code_real = 1513;
pub const BFD_RELOC_IA64_IMM14: bfd_reloc_code_real = 1512;
pub const BFD_RELOC_VTABLE_ENTRY: bfd_reloc_code_real = 1511;
pub const BFD_RELOC_VTABLE_INHERIT: bfd_reloc_code_real = 1510;
pub const BFD_RELOC_VPE4KMATH_INSN: bfd_reloc_code_real = 1509;
pub const BFD_RELOC_VPE4KMATH_DATA: bfd_reloc_code_real = 1508;
pub const BFD_RELOC_IP2K_FR_OFFSET: bfd_reloc_code_real = 1507;
pub const BFD_RELOC_IP2K_TEXT: bfd_reloc_code_real = 1506;
pub const BFD_RELOC_IP2K_PC_SKIP: bfd_reloc_code_real = 1505;
pub const BFD_RELOC_IP2K_HI8INSN: bfd_reloc_code_real = 1504;
pub const BFD_RELOC_IP2K_LO8INSN: bfd_reloc_code_real = 1503;
pub const BFD_RELOC_IP2K_EX8DATA: bfd_reloc_code_real = 1502;
pub const BFD_RELOC_IP2K_HI8DATA: bfd_reloc_code_real = 1501;
pub const BFD_RELOC_IP2K_LO8DATA: bfd_reloc_code_real = 1500;
pub const BFD_RELOC_IP2K_PAGE3: bfd_reloc_code_real = 1499;
pub const BFD_RELOC_IP2K_ADDR16CJP: bfd_reloc_code_real = 1498;
pub const BFD_RELOC_IP2K_BANK: bfd_reloc_code_real = 1497;
pub const BFD_RELOC_IP2K_FR9: bfd_reloc_code_real = 1496;
pub const BFD_RELOC_SCORE_DUMMY_HI16: bfd_reloc_code_real = 1495;
pub const BFD_RELOC_SCORE_CALL15: bfd_reloc_code_real = 1494;
pub const BFD_RELOC_SCORE_GOT_LO16: bfd_reloc_code_real = 1493;
pub const BFD_RELOC_SCORE_GOT15: bfd_reloc_code_real = 1492;
pub const BFD_RELOC_SCORE_BCMP: bfd_reloc_code_real = 1491;
pub const BFD_RELOC_SCORE16_BRANCH: bfd_reloc_code_real = 1490;
pub const BFD_RELOC_SCORE16_JMP: bfd_reloc_code_real = 1489;
pub const BFD_RELOC_SCORE_IMM32: bfd_reloc_code_real = 1488;
pub const BFD_RELOC_SCORE_IMM30: bfd_reloc_code_real = 1487;
pub const BFD_RELOC_SCORE_BRANCH: bfd_reloc_code_real = 1486;
pub const BFD_RELOC_SCORE_JMP: bfd_reloc_code_real = 1485;
pub const BFD_RELOC_SCORE_DUMMY2: bfd_reloc_code_real = 1484;
pub const BFD_RELOC_SCORE_GPREL15: bfd_reloc_code_real = 1483;
pub const BFD_RELOC_390_IRELATIVE: bfd_reloc_code_real = 1482;
pub const BFD_RELOC_390_TLS_GOTIE20: bfd_reloc_code_real = 1481;
pub const BFD_RELOC_390_GOTPLT20: bfd_reloc_code_real = 1480;
pub const BFD_RELOC_390_GOT20: bfd_reloc_code_real = 1479;
pub const BFD_RELOC_390_20: bfd_reloc_code_real = 1478;
pub const BFD_RELOC_390_TLS_TPOFF: bfd_reloc_code_real = 1477;
pub const BFD_RELOC_390_TLS_DTPOFF: bfd_reloc_code_real = 1476;
pub const BFD_RELOC_390_TLS_DTPMOD: bfd_reloc_code_real = 1475;
pub const BFD_RELOC_390_TLS_LDO64: bfd_reloc_code_real = 1474;
pub const BFD_RELOC_390_TLS_LDO32: bfd_reloc_code_real = 1473;
pub const BFD_RELOC_390_TLS_LE64: bfd_reloc_code_real = 1472;
pub const BFD_RELOC_390_TLS_LE32: bfd_reloc_code_real = 1471;
pub const BFD_RELOC_390_TLS_IEENT: bfd_reloc_code_real = 1470;
pub const BFD_RELOC_390_TLS_IE64: bfd_reloc_code_real = 1469;
pub const BFD_RELOC_390_TLS_IE32: bfd_reloc_code_real = 1468;
pub const BFD_RELOC_390_TLS_LDM64: bfd_reloc_code_real = 1467;
pub const BFD_RELOC_390_TLS_LDM32: bfd_reloc_code_real = 1466;
pub const BFD_RELOC_390_TLS_GOTIE64: bfd_reloc_code_real = 1465;
pub const BFD_RELOC_390_TLS_GOTIE32: bfd_reloc_code_real = 1464;
pub const BFD_RELOC_390_TLS_GOTIE12: bfd_reloc_code_real = 1463;
pub const BFD_RELOC_390_TLS_GD64: bfd_reloc_code_real = 1462;
pub const BFD_RELOC_390_TLS_GD32: bfd_reloc_code_real = 1461;
pub const BFD_RELOC_390_TLS_LDCALL: bfd_reloc_code_real = 1460;
pub const BFD_RELOC_390_TLS_GDCALL: bfd_reloc_code_real = 1459;
pub const BFD_RELOC_390_TLS_LOAD: bfd_reloc_code_real = 1458;
pub const BFD_RELOC_390_PLTOFF64: bfd_reloc_code_real = 1457;
pub const BFD_RELOC_390_PLTOFF32: bfd_reloc_code_real = 1456;
pub const BFD_RELOC_390_PLTOFF16: bfd_reloc_code_real = 1455;
pub const BFD_RELOC_390_GOTPLTENT: bfd_reloc_code_real = 1454;
pub const BFD_RELOC_390_GOTPLT64: bfd_reloc_code_real = 1453;
pub const BFD_RELOC_390_GOTPLT32: bfd_reloc_code_real = 1452;
pub const BFD_RELOC_390_GOTPLT16: bfd_reloc_code_real = 1451;
pub const BFD_RELOC_390_GOTPLT12: bfd_reloc_code_real = 1450;
pub const BFD_RELOC_390_GOTOFF64: bfd_reloc_code_real = 1449;
pub const BFD_RELOC_390_GOTENT: bfd_reloc_code_real = 1448;
pub const BFD_RELOC_390_PLT64: bfd_reloc_code_real = 1447;
pub const BFD_RELOC_390_GOT64: bfd_reloc_code_real = 1446;
pub const BFD_RELOC_390_GOTPCDBL: bfd_reloc_code_real = 1445;
pub const BFD_RELOC_390_PLT32DBL: bfd_reloc_code_real = 1444;
pub const BFD_RELOC_390_PC32DBL: bfd_reloc_code_real = 1443;
pub const BFD_RELOC_390_PLT24DBL: bfd_reloc_code_real = 1442;
pub const BFD_RELOC_390_PC24DBL: bfd_reloc_code_real = 1441;
pub const BFD_RELOC_390_PLT16DBL: bfd_reloc_code_real = 1440;
pub const BFD_RELOC_390_PC16DBL: bfd_reloc_code_real = 1439;
pub const BFD_RELOC_390_PLT12DBL: bfd_reloc_code_real = 1438;
pub const BFD_RELOC_390_PC12DBL: bfd_reloc_code_real = 1437;
pub const BFD_RELOC_390_GOT16: bfd_reloc_code_real = 1436;
pub const BFD_RELOC_390_GOTPC: bfd_reloc_code_real = 1435;
pub const BFD_RELOC_390_RELATIVE: bfd_reloc_code_real = 1434;
pub const BFD_RELOC_390_JMP_SLOT: bfd_reloc_code_real = 1433;
pub const BFD_RELOC_390_GLOB_DAT: bfd_reloc_code_real = 1432;
pub const BFD_RELOC_390_COPY: bfd_reloc_code_real = 1431;
pub const BFD_RELOC_390_PLT32: bfd_reloc_code_real = 1430;
pub const BFD_RELOC_390_GOT12: bfd_reloc_code_real = 1429;
pub const BFD_RELOC_390_12: bfd_reloc_code_real = 1428;
pub const BFD_RELOC_RX_RELAX: bfd_reloc_code_real = 1427;
pub const BFD_RELOC_RX_ABS16UL: bfd_reloc_code_real = 1426;
pub const BFD_RELOC_RX_ABS16UW: bfd_reloc_code_real = 1425;
pub const BFD_RELOC_RX_ABS16U: bfd_reloc_code_real = 1424;
pub const BFD_RELOC_RX_ABS32_REV: bfd_reloc_code_real = 1423;
pub const BFD_RELOC_RX_ABS32: bfd_reloc_code_real = 1422;
pub const BFD_RELOC_RX_ABS16_REV: bfd_reloc_code_real = 1421;
pub const BFD_RELOC_RX_ABS16: bfd_reloc_code_real = 1420;
pub const BFD_RELOC_RX_ABS8: bfd_reloc_code_real = 1419;
pub const BFD_RELOC_RX_OP_NEG: bfd_reloc_code_real = 1418;
pub const BFD_RELOC_RX_OP_SUBTRACT: bfd_reloc_code_real = 1417;
pub const BFD_RELOC_RX_SYM: bfd_reloc_code_real = 1416;
pub const BFD_RELOC_RX_GPRELL: bfd_reloc_code_real = 1415;
pub const BFD_RELOC_RX_GPRELW: bfd_reloc_code_real = 1414;
pub const BFD_RELOC_RX_GPRELB: bfd_reloc_code_real = 1413;
pub const BFD_RELOC_RX_DIFF: bfd_reloc_code_real = 1412;
pub const BFD_RELOC_RX_DIR3U_PCREL: bfd_reloc_code_real = 1411;
pub const BFD_RELOC_RX_24U: bfd_reloc_code_real = 1410;
pub const BFD_RELOC_RX_16U: bfd_reloc_code_real = 1409;
pub const BFD_RELOC_RX_8U: bfd_reloc_code_real = 1408;
pub const BFD_RELOC_RX_32_OP: bfd_reloc_code_real = 1407;
pub const BFD_RELOC_RX_24_OP: bfd_reloc_code_real = 1406;
pub const BFD_RELOC_RX_16_OP: bfd_reloc_code_real = 1405;
pub const BFD_RELOC_RX_NEG32: bfd_reloc_code_real = 1404;
pub const BFD_RELOC_RX_NEG24: bfd_reloc_code_real = 1403;
pub const BFD_RELOC_RX_NEG16: bfd_reloc_code_real = 1402;
pub const BFD_RELOC_RX_NEG8: bfd_reloc_code_real = 1401;
pub const BFD_RELOC_RL78_SADDR: bfd_reloc_code_real = 1400;
pub const BFD_RELOC_RL78_CODE: bfd_reloc_code_real = 1399;
pub const BFD_RELOC_RL78_LO16: bfd_reloc_code_real = 1398;
pub const BFD_RELOC_RL78_HI8: bfd_reloc_code_real = 1397;
pub const BFD_RELOC_RL78_HI16: bfd_reloc_code_real = 1396;
pub const BFD_RELOC_RL78_RELAX: bfd_reloc_code_real = 1395;
pub const BFD_RELOC_RL78_ABS16UL: bfd_reloc_code_real = 1394;
pub const BFD_RELOC_RL78_ABS16UW: bfd_reloc_code_real = 1393;
pub const BFD_RELOC_RL78_ABS16U: bfd_reloc_code_real = 1392;
pub const BFD_RELOC_RL78_ABS32_REV: bfd_reloc_code_real = 1391;
pub const BFD_RELOC_RL78_ABS32: bfd_reloc_code_real = 1390;
pub const BFD_RELOC_RL78_ABS16_REV: bfd_reloc_code_real = 1389;
pub const BFD_RELOC_RL78_ABS16: bfd_reloc_code_real = 1388;
pub const BFD_RELOC_RL78_ABS8: bfd_reloc_code_real = 1387;
pub const BFD_RELOC_RL78_OP_SHRA: bfd_reloc_code_real = 1386;
pub const BFD_RELOC_RL78_OP_AND: bfd_reloc_code_real = 1385;
pub const BFD_RELOC_RL78_OP_NEG: bfd_reloc_code_real = 1384;
pub const BFD_RELOC_RL78_OP_SUBTRACT: bfd_reloc_code_real = 1383;
pub const BFD_RELOC_RL78_SYM: bfd_reloc_code_real = 1382;
pub const BFD_RELOC_RL78_GPRELL: bfd_reloc_code_real = 1381;
pub const BFD_RELOC_RL78_GPRELW: bfd_reloc_code_real = 1380;
pub const BFD_RELOC_RL78_GPRELB: bfd_reloc_code_real = 1379;
pub const BFD_RELOC_RL78_DIFF: bfd_reloc_code_real = 1378;
pub const BFD_RELOC_RL78_DIR3U_PCREL: bfd_reloc_code_real = 1377;
pub const BFD_RELOC_RL78_24U: bfd_reloc_code_real = 1376;
pub const BFD_RELOC_RL78_16U: bfd_reloc_code_real = 1375;
pub const BFD_RELOC_RL78_8U: bfd_reloc_code_real = 1374;
pub const BFD_RELOC_RL78_32_OP: bfd_reloc_code_real = 1373;
pub const BFD_RELOC_RL78_24_OP: bfd_reloc_code_real = 1372;
pub const BFD_RELOC_RL78_16_OP: bfd_reloc_code_real = 1371;
pub const BFD_RELOC_RL78_NEG32: bfd_reloc_code_real = 1370;
pub const BFD_RELOC_RL78_NEG24: bfd_reloc_code_real = 1369;
pub const BFD_RELOC_RL78_NEG16: bfd_reloc_code_real = 1368;
pub const BFD_RELOC_RL78_NEG8: bfd_reloc_code_real = 1367;
pub const BFD_RELOC_RISCV_32_PCREL: bfd_reloc_code_real = 1366;
pub const BFD_RELOC_RISCV_SET32: bfd_reloc_code_real = 1365;
pub const BFD_RELOC_RISCV_SET16: bfd_reloc_code_real = 1364;
pub const BFD_RELOC_RISCV_SET8: bfd_reloc_code_real = 1363;
pub const BFD_RELOC_RISCV_SET6: bfd_reloc_code_real = 1362;
pub const BFD_RELOC_RISCV_SUB6: bfd_reloc_code_real = 1361;
pub const BFD_RELOC_RISCV_CFA: bfd_reloc_code_real = 1360;
pub const BFD_RELOC_RISCV_RELAX: bfd_reloc_code_real = 1359;
pub const BFD_RELOC_RISCV_TPREL_S: bfd_reloc_code_real = 1358;
pub const BFD_RELOC_RISCV_TPREL_I: bfd_reloc_code_real = 1357;
pub const BFD_RELOC_RISCV_GPREL_S: bfd_reloc_code_real = 1356;
pub const BFD_RELOC_RISCV_GPREL_I: bfd_reloc_code_real = 1355;
pub const BFD_RELOC_RISCV_RVC_LUI: bfd_reloc_code_real = 1354;
pub const BFD_RELOC_RISCV_RVC_JUMP: bfd_reloc_code_real = 1353;
pub const BFD_RELOC_RISCV_RVC_BRANCH: bfd_reloc_code_real = 1352;
pub const BFD_RELOC_RISCV_ALIGN: bfd_reloc_code_real = 1351;
pub const BFD_RELOC_RISCV_TLS_TPREL64: bfd_reloc_code_real = 1350;
pub const BFD_RELOC_RISCV_TLS_TPREL32: bfd_reloc_code_real = 1349;
pub const BFD_RELOC_RISCV_TLS_DTPREL64: bfd_reloc_code_real = 1348;
pub const BFD_RELOC_RISCV_TLS_DTPMOD64: bfd_reloc_code_real = 1347;
pub const BFD_RELOC_RISCV_TLS_DTPREL32: bfd_reloc_code_real = 1346;
pub const BFD_RELOC_RISCV_TLS_DTPMOD32: bfd_reloc_code_real = 1345;
pub const BFD_RELOC_RISCV_JMP: bfd_reloc_code_real = 1344;
pub const BFD_RELOC_RISCV_TLS_GD_HI20: bfd_reloc_code_real = 1343;
pub const BFD_RELOC_RISCV_TLS_GOT_HI20: bfd_reloc_code_real = 1342;
pub const BFD_RELOC_RISCV_GOT_HI20: bfd_reloc_code_real = 1341;
pub const BFD_RELOC_RISCV_SUB64: bfd_reloc_code_real = 1340;
pub const BFD_RELOC_RISCV_SUB32: bfd_reloc_code_real = 1339;
pub const BFD_RELOC_RISCV_SUB16: bfd_reloc_code_real = 1338;
pub const BFD_RELOC_RISCV_SUB8: bfd_reloc_code_real = 1337;
pub const BFD_RELOC_RISCV_ADD64: bfd_reloc_code_real = 1336;
pub const BFD_RELOC_RISCV_ADD32: bfd_reloc_code_real = 1335;
pub const BFD_RELOC_RISCV_ADD16: bfd_reloc_code_real = 1334;
pub const BFD_RELOC_RISCV_ADD8: bfd_reloc_code_real = 1333;
pub const BFD_RELOC_RISCV_CALL_PLT: bfd_reloc_code_real = 1332;
pub const BFD_RELOC_RISCV_CALL: bfd_reloc_code_real = 1331;
pub const BFD_RELOC_RISCV_TPREL_ADD: bfd_reloc_code_real = 1330;
pub const BFD_RELOC_RISCV_TPREL_LO12_S: bfd_reloc_code_real = 1329;
pub const BFD_RELOC_RISCV_TPREL_LO12_I: bfd_reloc_code_real = 1328;
pub const BFD_RELOC_RISCV_TPREL_HI20: bfd_reloc_code_real = 1327;
pub const BFD_RELOC_RISCV_GPREL12_S: bfd_reloc_code_real = 1326;
pub const BFD_RELOC_RISCV_GPREL12_I: bfd_reloc_code_real = 1325;
pub const BFD_RELOC_RISCV_LO12_S: bfd_reloc_code_real = 1324;
pub const BFD_RELOC_RISCV_LO12_I: bfd_reloc_code_real = 1323;
pub const BFD_RELOC_RISCV_PCREL_LO12_S: bfd_reloc_code_real = 1322;
pub const BFD_RELOC_RISCV_PCREL_LO12_I: bfd_reloc_code_real = 1321;
pub const BFD_RELOC_RISCV_PCREL_HI20: bfd_reloc_code_real = 1320;
pub const BFD_RELOC_RISCV_HI20: bfd_reloc_code_real = 1319;
pub const BFD_RELOC_AVR_PORT5: bfd_reloc_code_real = 1318;
pub const BFD_RELOC_AVR_PORT6: bfd_reloc_code_real = 1317;
pub const BFD_RELOC_AVR_LDS_STS_16: bfd_reloc_code_real = 1316;
pub const BFD_RELOC_AVR_DIFF32: bfd_reloc_code_real = 1315;
pub const BFD_RELOC_AVR_DIFF16: bfd_reloc_code_real = 1314;
pub const BFD_RELOC_AVR_DIFF8: bfd_reloc_code_real = 1313;
pub const BFD_RELOC_AVR_8_HLO: bfd_reloc_code_real = 1312;
pub const BFD_RELOC_AVR_8_HI: bfd_reloc_code_real = 1311;
pub const BFD_RELOC_AVR_8_LO: bfd_reloc_code_real = 1310;
pub const BFD_RELOC_AVR_6_ADIW: bfd_reloc_code_real = 1309;
pub const BFD_RELOC_AVR_6: bfd_reloc_code_real = 1308;
pub const BFD_RELOC_AVR_LDI: bfd_reloc_code_real = 1307;
pub const BFD_RELOC_AVR_CALL: bfd_reloc_code_real = 1306;
pub const BFD_RELOC_AVR_HH8_LDI_PM_NEG: bfd_reloc_code_real = 1305;
pub const BFD_RELOC_AVR_HI8_LDI_PM_NEG: bfd_reloc_code_real = 1304;
pub const BFD_RELOC_AVR_LO8_LDI_PM_NEG: bfd_reloc_code_real = 1303;
pub const BFD_RELOC_AVR_HH8_LDI_PM: bfd_reloc_code_real = 1302;
pub const BFD_RELOC_AVR_HI8_LDI_GS: bfd_reloc_code_real = 1301;
pub const BFD_RELOC_AVR_HI8_LDI_PM: bfd_reloc_code_real = 1300;
pub const BFD_RELOC_AVR_LO8_LDI_GS: bfd_reloc_code_real = 1299;
pub const BFD_RELOC_AVR_LO8_LDI_PM: bfd_reloc_code_real = 1298;
pub const BFD_RELOC_AVR_MS8_LDI_NEG: bfd_reloc_code_real = 1297;
pub const BFD_RELOC_AVR_HH8_LDI_NEG: bfd_reloc_code_real = 1296;
pub const BFD_RELOC_AVR_HI8_LDI_NEG: bfd_reloc_code_real = 1295;
pub const BFD_RELOC_AVR_LO8_LDI_NEG: bfd_reloc_code_real = 1294;
pub const BFD_RELOC_AVR_MS8_LDI: bfd_reloc_code_real = 1293;
pub const BFD_RELOC_AVR_HH8_LDI: bfd_reloc_code_real = 1292;
pub const BFD_RELOC_AVR_HI8_LDI: bfd_reloc_code_real = 1291;
pub const BFD_RELOC_AVR_LO8_LDI: bfd_reloc_code_real = 1290;
pub const BFD_RELOC_AVR_16_PM: bfd_reloc_code_real = 1289;
pub const BFD_RELOC_AVR_13_PCREL: bfd_reloc_code_real = 1288;
pub const BFD_RELOC_AVR_7_PCREL: bfd_reloc_code_real = 1287;
pub const BFD_RELOC_MMIX_LOCAL: bfd_reloc_code_real = 1286;
pub const BFD_RELOC_MMIX_BASE_PLUS_OFFSET: bfd_reloc_code_real = 1285;
pub const BFD_RELOC_MMIX_REG: bfd_reloc_code_real = 1284;
pub const BFD_RELOC_MMIX_REG_OR_BYTE: bfd_reloc_code_real = 1283;
pub const BFD_RELOC_MMIX_ADDR27: bfd_reloc_code_real = 1282;
pub const BFD_RELOC_MMIX_ADDR19: bfd_reloc_code_real = 1281;
pub const BFD_RELOC_MMIX_JMP_3: bfd_reloc_code_real = 1280;
pub const BFD_RELOC_MMIX_JMP_2: bfd_reloc_code_real = 1279;
pub const BFD_RELOC_MMIX_JMP_1: bfd_reloc_code_real = 1278;
pub const BFD_RELOC_MMIX_JMP: bfd_reloc_code_real = 1277;
pub const BFD_RELOC_MMIX_PUSHJ_STUBBABLE: bfd_reloc_code_real = 1276;
pub const BFD_RELOC_MMIX_PUSHJ_3: bfd_reloc_code_real = 1275;
pub const BFD_RELOC_MMIX_PUSHJ_2: bfd_reloc_code_real = 1274;
pub const BFD_RELOC_MMIX_PUSHJ_1: bfd_reloc_code_real = 1273;
pub const BFD_RELOC_MMIX_PUSHJ: bfd_reloc_code_real = 1272;
pub const BFD_RELOC_MMIX_CBRANCH_3: bfd_reloc_code_real = 1271;
pub const BFD_RELOC_MMIX_CBRANCH_2: bfd_reloc_code_real = 1270;
pub const BFD_RELOC_MMIX_CBRANCH_1: bfd_reloc_code_real = 1269;
pub const BFD_RELOC_MMIX_CBRANCH_J: bfd_reloc_code_real = 1268;
pub const BFD_RELOC_MMIX_CBRANCH: bfd_reloc_code_real = 1267;
pub const BFD_RELOC_MMIX_GETA_3: bfd_reloc_code_real = 1266;
pub const BFD_RELOC_MMIX_GETA_2: bfd_reloc_code_real = 1265;
pub const BFD_RELOC_MMIX_GETA_1: bfd_reloc_code_real = 1264;
pub const BFD_RELOC_MMIX_GETA: bfd_reloc_code_real = 1263;
pub const BFD_RELOC_METAG_TLS_LE_LO16: bfd_reloc_code_real = 1262;
pub const BFD_RELOC_METAG_TLS_LE_HI16: bfd_reloc_code_real = 1261;
pub const BFD_RELOC_METAG_TLS_LE: bfd_reloc_code_real = 1260;
pub const BFD_RELOC_METAG_TLS_DTPOFF: bfd_reloc_code_real = 1259;
pub const BFD_RELOC_METAG_TLS_DTPMOD: bfd_reloc_code_real = 1258;
pub const BFD_RELOC_METAG_TLS_TPOFF: bfd_reloc_code_real = 1257;
pub const BFD_RELOC_METAG_TLS_IENONPIC_LO16: bfd_reloc_code_real = 1256;
pub const BFD_RELOC_METAG_TLS_IENONPIC_HI16: bfd_reloc_code_real = 1255;
pub const BFD_RELOC_METAG_TLS_IENONPIC: bfd_reloc_code_real = 1254;
pub const BFD_RELOC_METAG_TLS_IE: bfd_reloc_code_real = 1253;
pub const BFD_RELOC_METAG_TLS_LDO: bfd_reloc_code_real = 1252;
pub const BFD_RELOC_METAG_TLS_LDO_LO16: bfd_reloc_code_real = 1251;
pub const BFD_RELOC_METAG_TLS_LDO_HI16: bfd_reloc_code_real = 1250;
pub const BFD_RELOC_METAG_TLS_LDM: bfd_reloc_code_real = 1249;
pub const BFD_RELOC_METAG_TLS_GD: bfd_reloc_code_real = 1248;
pub const BFD_RELOC_METAG_GLOB_DAT: bfd_reloc_code_real = 1247;
pub const BFD_RELOC_METAG_RELATIVE: bfd_reloc_code_real = 1246;
pub const BFD_RELOC_METAG_JMP_SLOT: bfd_reloc_code_real = 1245;
pub const BFD_RELOC_METAG_COPY: bfd_reloc_code_real = 1244;
pub const BFD_RELOC_METAG_PLT: bfd_reloc_code_real = 1243;
pub const BFD_RELOC_METAG_GOTOFF: bfd_reloc_code_real = 1242;
pub const BFD_RELOC_METAG_RELBRANCH_PLT: bfd_reloc_code_real = 1241;
pub const BFD_RELOC_METAG_LO16_PLT: bfd_reloc_code_real = 1240;
pub const BFD_RELOC_METAG_HI16_PLT: bfd_reloc_code_real = 1239;
pub const BFD_RELOC_METAG_LO16_GOTPC: bfd_reloc_code_real = 1238;
pub const BFD_RELOC_METAG_HI16_GOTPC: bfd_reloc_code_real = 1237;
pub const BFD_RELOC_METAG_GETSET_GOT: bfd_reloc_code_real = 1236;
pub const BFD_RELOC_METAG_GETSET_GOTOFF: bfd_reloc_code_real = 1235;
pub const BFD_RELOC_METAG_LO16_GOTOFF: bfd_reloc_code_real = 1234;
pub const BFD_RELOC_METAG_HI16_GOTOFF: bfd_reloc_code_real = 1233;
pub const BFD_RELOC_METAG_REL16: bfd_reloc_code_real = 1232;
pub const BFD_RELOC_METAG_REL8: bfd_reloc_code_real = 1231;
pub const BFD_RELOC_METAG_LOOG: bfd_reloc_code_real = 1230;
pub const BFD_RELOC_METAG_HIOG: bfd_reloc_code_real = 1229;
pub const BFD_RELOC_METAG_GETSETOFF: bfd_reloc_code_real = 1228;
pub const BFD_RELOC_METAG_RELBRANCH: bfd_reloc_code_real = 1227;
pub const BFD_RELOC_METAG_LOADDR16: bfd_reloc_code_real = 1226;
pub const BFD_RELOC_METAG_HIADDR16: bfd_reloc_code_real = 1225;
pub const BFD_RELOC_MEP_GNU_VTENTRY: bfd_reloc_code_real = 1224;
pub const BFD_RELOC_MEP_GNU_VTINHERIT: bfd_reloc_code_real = 1223;
pub const BFD_RELOC_MEP_ADDR24A4: bfd_reloc_code_real = 1222;
pub const BFD_RELOC_MEP_UIMM24: bfd_reloc_code_real = 1221;
pub const BFD_RELOC_MEP_TPREL7A4: bfd_reloc_code_real = 1220;
pub const BFD_RELOC_MEP_TPREL7A2: bfd_reloc_code_real = 1219;
pub const BFD_RELOC_MEP_TPREL7: bfd_reloc_code_real = 1218;
pub const BFD_RELOC_MEP_TPREL: bfd_reloc_code_real = 1217;
pub const BFD_RELOC_MEP_GPREL: bfd_reloc_code_real = 1216;
pub const BFD_RELOC_MEP_HI16S: bfd_reloc_code_real = 1215;
pub const BFD_RELOC_MEP_HI16U: bfd_reloc_code_real = 1214;
pub const BFD_RELOC_MEP_LOW16: bfd_reloc_code_real = 1213;
pub const BFD_RELOC_MEP_PCABS24A2: bfd_reloc_code_real = 1212;
pub const BFD_RELOC_MEP_PCREL24A2: bfd_reloc_code_real = 1211;
pub const BFD_RELOC_MEP_PCREL17A2: bfd_reloc_code_real = 1210;
pub const BFD_RELOC_MEP_PCREL12A2: bfd_reloc_code_real = 1209;
pub const BFD_RELOC_MEP_PCREL8A2: bfd_reloc_code_real = 1208;
pub const BFD_RELOC_MEP_32: bfd_reloc_code_real = 1207;
pub const BFD_RELOC_MEP_16: bfd_reloc_code_real = 1206;
pub const BFD_RELOC_MEP_8: bfd_reloc_code_real = 1205;
pub const BFD_RELOC_MCORE_RVA: bfd_reloc_code_real = 1204;
pub const BFD_RELOC_MCORE_PCREL_JSR_IMM11BY2: bfd_reloc_code_real = 1203;
pub const BFD_RELOC_MCORE_PCREL_32: bfd_reloc_code_real = 1202;
pub const BFD_RELOC_MCORE_PCREL_IMM4BY2: bfd_reloc_code_real = 1201;
pub const BFD_RELOC_MCORE_PCREL_IMM11BY2: bfd_reloc_code_real = 1200;
pub const BFD_RELOC_MCORE_PCREL_IMM8BY4: bfd_reloc_code_real = 1199;
pub const BFD_RELOC_FR30_12_PCREL: bfd_reloc_code_real = 1198;
pub const BFD_RELOC_FR30_9_PCREL: bfd_reloc_code_real = 1197;
pub const BFD_RELOC_FR30_10_IN_8: bfd_reloc_code_real = 1196;
pub const BFD_RELOC_FR30_9_IN_8: bfd_reloc_code_real = 1195;
pub const BFD_RELOC_FR30_8_IN_8: bfd_reloc_code_real = 1194;
pub const BFD_RELOC_FR30_6_IN_4: bfd_reloc_code_real = 1193;
pub const BFD_RELOC_FR30_20: bfd_reloc_code_real = 1192;
pub const BFD_RELOC_FR30_48: bfd_reloc_code_real = 1191;
pub const BFD_RELOC_C6000_NOCMP: bfd_reloc_code_real = 1190;
pub const BFD_RELOC_C6000_FPHEAD: bfd_reloc_code_real = 1189;
pub const BFD_RELOC_C6000_ALIGN: bfd_reloc_code_real = 1188;
pub const BFD_RELOC_C6000_PCR_L16: bfd_reloc_code_real = 1187;
pub const BFD_RELOC_C6000_PCR_H16: bfd_reloc_code_real = 1186;
pub const BFD_RELOC_C6000_EHTYPE: bfd_reloc_code_real = 1185;
pub const BFD_RELOC_C6000_JUMP_SLOT: bfd_reloc_code_real = 1184;
pub const BFD_RELOC_C6000_COPY: bfd_reloc_code_real = 1183;
pub const BFD_RELOC_C6000_PREL31: bfd_reloc_code_real = 1182;
pub const BFD_RELOC_C6000_DSBT_INDEX: bfd_reloc_code_real = 1181;
pub const BFD_RELOC_C6000_SBR_GOT_H16_W: bfd_reloc_code_real = 1180;
pub const BFD_RELOC_C6000_SBR_GOT_L16_W: bfd_reloc_code_real = 1179;
pub const BFD_RELOC_C6000_SBR_GOT_U15_W: bfd_reloc_code_real = 1178;
pub const BFD_RELOC_C6000_SBR_H16_W: bfd_reloc_code_real = 1177;
pub const BFD_RELOC_C6000_SBR_H16_H: bfd_reloc_code_real = 1176;
pub const BFD_RELOC_C6000_SBR_H16_B: bfd_reloc_code_real = 1175;
pub const BFD_RELOC_C6000_SBR_L16_W: bfd_reloc_code_real = 1174;
pub const BFD_RELOC_C6000_SBR_L16_H: bfd_reloc_code_real = 1173;
pub const BFD_RELOC_C6000_SBR_L16_B: bfd_reloc_code_real = 1172;
pub const BFD_RELOC_C6000_SBR_S16: bfd_reloc_code_real = 1171;
pub const BFD_RELOC_C6000_SBR_U15_W: bfd_reloc_code_real = 1170;
pub const BFD_RELOC_C6000_SBR_U15_H: bfd_reloc_code_real = 1169;
pub const BFD_RELOC_C6000_SBR_U15_B: bfd_reloc_code_real = 1168;
pub const BFD_RELOC_C6000_ABS_H16: bfd_reloc_code_real = 1167;
pub const BFD_RELOC_C6000_ABS_L16: bfd_reloc_code_real = 1166;
pub const BFD_RELOC_C6000_ABS_S16: bfd_reloc_code_real = 1165;
pub const BFD_RELOC_C6000_PCR_S7: bfd_reloc_code_real = 1164;
pub const BFD_RELOC_C6000_PCR_S10: bfd_reloc_code_real = 1163;
pub const BFD_RELOC_C6000_PCR_S12: bfd_reloc_code_real = 1162;
pub const BFD_RELOC_C6000_PCR_S21: bfd_reloc_code_real = 1161;
pub const BFD_RELOC_TIC54X_MS7_OF_23: bfd_reloc_code_real = 1160;
pub const BFD_RELOC_TIC54X_16_OF_23: bfd_reloc_code_real = 1159;
pub const BFD_RELOC_TIC54X_23: bfd_reloc_code_real = 1158;
pub const BFD_RELOC_TIC54X_PARTMS9: bfd_reloc_code_real = 1157;
pub const BFD_RELOC_TIC54X_PARTLS7: bfd_reloc_code_real = 1156;
pub const BFD_RELOC_TIC30_LDP: bfd_reloc_code_real = 1155;
pub const BFD_RELOC_V850_DATA: bfd_reloc_code_real = 1154;
pub const BFD_RELOC_V850_CODE: bfd_reloc_code_real = 1153;
pub const BFD_RELOC_V850_32_GOTOFF: bfd_reloc_code_real = 1152;
pub const BFD_RELOC_V850_16_GOTOFF: bfd_reloc_code_real = 1151;
pub const BFD_RELOC_V850_RELATIVE: bfd_reloc_code_real = 1150;
pub const BFD_RELOC_V850_JMP_SLOT: bfd_reloc_code_real = 1149;
pub const BFD_RELOC_V850_GLOB_DAT: bfd_reloc_code_real = 1148;
pub const BFD_RELOC_V850_COPY: bfd_reloc_code_real = 1147;
pub const BFD_RELOC_V850_32_PLT_PCREL: bfd_reloc_code_real = 1146;
pub const BFD_RELOC_V850_22_PLT_PCREL: bfd_reloc_code_real = 1145;
pub const BFD_RELOC_V850_32_GOT: bfd_reloc_code_real = 1144;
pub const BFD_RELOC_V850_16_GOT: bfd_reloc_code_real = 1143;
pub const BFD_RELOC_V850_32_GOTPCREL: bfd_reloc_code_real = 1142;
pub const BFD_RELOC_V850_CALLT_15_16_OFFSET: bfd_reloc_code_real = 1141;
pub const BFD_RELOC_V850_LO16_S1: bfd_reloc_code_real = 1140;
pub const BFD_RELOC_V850_16_S1: bfd_reloc_code_real = 1139;
pub const BFD_RELOC_V850_16_SPLIT_OFFSET: bfd_reloc_code_real = 1138;
pub const BFD_RELOC_V850_32_ABS: bfd_reloc_code_real = 1137;
pub const BFD_RELOC_V850_32_PCREL: bfd_reloc_code_real = 1136;
pub const BFD_RELOC_V850_23: bfd_reloc_code_real = 1135;
pub const BFD_RELOC_V850_17_PCREL: bfd_reloc_code_real = 1134;
pub const BFD_RELOC_V850_16_PCREL: bfd_reloc_code_real = 1133;
pub const BFD_RELOC_V850_LO16_SPLIT_OFFSET: bfd_reloc_code_real = 1132;
pub const BFD_RELOC_V850_ALIGN: bfd_reloc_code_real = 1131;
pub const BFD_RELOC_V850_LONGJUMP: bfd_reloc_code_real = 1130;
pub const BFD_RELOC_V850_LONGCALL: bfd_reloc_code_real = 1129;
pub const BFD_RELOC_V850_CALLT_16_16_OFFSET: bfd_reloc_code_real = 1128;
pub const BFD_RELOC_V850_CALLT_6_7_OFFSET: bfd_reloc_code_real = 1127;
pub const BFD_RELOC_V850_ZDA_16_16_SPLIT_OFFSET: bfd_reloc_code_real = 1126;
pub const BFD_RELOC_V850_SDA_16_16_SPLIT_OFFSET: bfd_reloc_code_real = 1125;
pub const BFD_RELOC_V850_TDA_4_4_OFFSET: bfd_reloc_code_real = 1124;
pub const BFD_RELOC_V850_TDA_4_5_OFFSET: bfd_reloc_code_real = 1123;
pub const BFD_RELOC_V850_TDA_16_16_OFFSET: bfd_reloc_code_real = 1122;
pub const BFD_RELOC_V850_TDA_7_7_OFFSET: bfd_reloc_code_real = 1121;
pub const BFD_RELOC_V850_TDA_7_8_OFFSET: bfd_reloc_code_real = 1120;
pub const BFD_RELOC_V850_TDA_6_8_OFFSET: bfd_reloc_code_real = 1119;
pub const BFD_RELOC_V850_ZDA_15_16_OFFSET: bfd_reloc_code_real = 1118;
pub const BFD_RELOC_V850_ZDA_16_16_OFFSET: bfd_reloc_code_real = 1117;
pub const BFD_RELOC_V850_SDA_15_16_OFFSET: bfd_reloc_code_real = 1116;
pub const BFD_RELOC_V850_SDA_16_16_OFFSET: bfd_reloc_code_real = 1115;
pub const BFD_RELOC_V850_22_PCREL: bfd_reloc_code_real = 1114;
pub const BFD_RELOC_V850_9_PCREL: bfd_reloc_code_real = 1113;
pub const BFD_RELOC_NDS32_LSI: bfd_reloc_code_real = 1112;
pub const BFD_RELOC_NDS32_GROUP: bfd_reloc_code_real = 1111;
pub const BFD_RELOC_NDS32_REMOVE: bfd_reloc_code_real = 1110;
pub const BFD_RELOC_NDS32_TLS_DESC_MEM: bfd_reloc_code_real = 1109;
pub const BFD_RELOC_NDS32_TLS_DESC_CALL: bfd_reloc_code_real = 1108;
pub const BFD_RELOC_NDS32_TLS_DESC_FUNC: bfd_reloc_code_real = 1107;
pub const BFD_RELOC_NDS32_TLS_DESC_ADD: bfd_reloc_code_real = 1106;
pub const BFD_RELOC_NDS32_TLS_DESC_SDA17S2: bfd_reloc_code_real = 1105;
pub const BFD_RELOC_NDS32_TLS_DESC_20: bfd_reloc_code_real = 1104;
pub const BFD_RELOC_NDS32_TLS_DESC_LO12: bfd_reloc_code_real = 1103;
pub const BFD_RELOC_NDS32_TLS_DESC_HI20: bfd_reloc_code_real = 1102;
pub const BFD_RELOC_NDS32_TLS_DESC: bfd_reloc_code_real = 1101;
pub const BFD_RELOC_NDS32_TLS_IEGP_LW: bfd_reloc_code_real = 1100;
pub const BFD_RELOC_NDS32_TLS_IEGP_LO12S2: bfd_reloc_code_real = 1099;
pub const BFD_RELOC_NDS32_TLS_IEGP_LO12: bfd_reloc_code_real = 1098;
pub const BFD_RELOC_NDS32_TLS_IEGP_HI20: bfd_reloc_code_real = 1097;
pub const BFD_RELOC_NDS32_TLS_IE_LO12S2: bfd_reloc_code_real = 1096;
pub const BFD_RELOC_NDS32_TLS_IE_LO12: bfd_reloc_code_real = 1095;
pub const BFD_RELOC_NDS32_TLS_IE_HI20: bfd_reloc_code_real = 1094;
pub const BFD_RELOC_NDS32_TLS_LE_LS: bfd_reloc_code_real = 1093;
pub const BFD_RELOC_NDS32_TLS_LE_ADD: bfd_reloc_code_real = 1092;
pub const BFD_RELOC_NDS32_TLS_LE_15S2: bfd_reloc_code_real = 1091;
pub const BFD_RELOC_NDS32_TLS_LE_15S1: bfd_reloc_code_real = 1090;
pub const BFD_RELOC_NDS32_TLS_LE_15S0: bfd_reloc_code_real = 1089;
pub const BFD_RELOC_NDS32_TLS_LE_20: bfd_reloc_code_real = 1088;
pub const BFD_RELOC_NDS32_TLS_LE_LO12: bfd_reloc_code_real = 1087;
pub const BFD_RELOC_NDS32_TLS_LE_HI20: bfd_reloc_code_real = 1086;
pub const BFD_RELOC_NDS32_GOTTPOFF: bfd_reloc_code_real = 1085;
pub const BFD_RELOC_NDS32_TPOFF: bfd_reloc_code_real = 1084;
pub const BFD_RELOC_NDS32_10IFCU_PCREL: bfd_reloc_code_real = 1083;
pub const BFD_RELOC_NDS32_17IFC_PCREL: bfd_reloc_code_real = 1082;
pub const BFD_RELOC_NDS32_TRAN: bfd_reloc_code_real = 1081;
pub const BFD_RELOC_NDS32_DATA: bfd_reloc_code_real = 1080;
pub const BFD_RELOC_NDS32_25_ABS: bfd_reloc_code_real = 1079;
pub const BFD_RELOC_NDS32_EMPTY: bfd_reloc_code_real = 1078;
pub const BFD_RELOC_NDS32_DIFF_ULEB128: bfd_reloc_code_real = 1077;
pub const BFD_RELOC_NDS32_DIFF32: bfd_reloc_code_real = 1076;
pub const BFD_RELOC_NDS32_DIFF16: bfd_reloc_code_real = 1075;
pub const BFD_RELOC_NDS32_DIFF8: bfd_reloc_code_real = 1074;
pub const BFD_RELOC_NDS32_SUBTRAHEND: bfd_reloc_code_real = 1073;
pub const BFD_RELOC_NDS32_MINUEND: bfd_reloc_code_real = 1072;
pub const BFD_RELOC_NDS32_RELAX_REGION_END: bfd_reloc_code_real = 1071;
pub const BFD_RELOC_NDS32_RELAX_REGION_BEGIN: bfd_reloc_code_real = 1070;
pub const BFD_RELOC_NDS32_PLTBLOCK: bfd_reloc_code_real = 1069;
pub const BFD_RELOC_NDS32_PTR_RESOLVED: bfd_reloc_code_real = 1068;
pub const BFD_RELOC_NDS32_PTR_COUNT: bfd_reloc_code_real = 1067;
pub const BFD_RELOC_NDS32_PTR: bfd_reloc_code_real = 1066;
pub const BFD_RELOC_NDS32_MULCALL_SUFF: bfd_reloc_code_real = 1065;
pub const BFD_RELOC_NDS32_PLT_GOT_SUFF: bfd_reloc_code_real = 1064;
pub const BFD_RELOC_NDS32_GOTOFF_SUFF: bfd_reloc_code_real = 1063;
pub const BFD_RELOC_NDS32_GOT_SUFF: bfd_reloc_code_real = 1062;
pub const BFD_RELOC_NDS32_RELAX_ENTRY: bfd_reloc_code_real = 1061;
pub const BFD_RELOC_NDS32_SDA_FP7U2_RELA: bfd_reloc_code_real = 1060;
pub const BFD_RELOC_NDS32_10_UPCREL: bfd_reloc_code_real = 1059;
pub const BFD_RELOC_NDS32_5: bfd_reloc_code_real = 1058;
pub const BFD_RELOC_NDS32_GOT17S2: bfd_reloc_code_real = 1057;
pub const BFD_RELOC_NDS32_GOT15S2: bfd_reloc_code_real = 1056;
pub const BFD_RELOC_NDS32_GOTOFF_LO19: bfd_reloc_code_real = 1055;
pub const BFD_RELOC_NDS32_GOTOFF_LO15: bfd_reloc_code_real = 1054;
pub const BFD_RELOC_NDS32_GOT_LO19: bfd_reloc_code_real = 1053;
pub const BFD_RELOC_NDS32_GOT_LO15: bfd_reloc_code_real = 1052;
pub const BFD_RELOC_NDS32_PLT_GOTREL_LO19: bfd_reloc_code_real = 1051;
pub const BFD_RELOC_NDS32_PLT_GOTREL_LO15: bfd_reloc_code_real = 1050;
pub const BFD_RELOC_NDS32_PLT_GOTREL_LO20: bfd_reloc_code_real = 1049;
pub const BFD_RELOC_NDS32_UPDATE_TA: bfd_reloc_code_real = 1048;
pub const BFD_RELOC_NDS32_DWARF2_LEB: bfd_reloc_code_real = 1047;
pub const BFD_RELOC_NDS32_DWARF2_OP2: bfd_reloc_code_real = 1046;
pub const BFD_RELOC_NDS32_DWARF2_OP1: bfd_reloc_code_real = 1045;
pub const BFD_RELOC_NDS32_LO12S2_SP: bfd_reloc_code_real = 1044;
pub const BFD_RELOC_NDS32_LO12S2_DP: bfd_reloc_code_real = 1043;
pub const BFD_RELOC_NDS32_SDA12S2_SP: bfd_reloc_code_real = 1042;
pub const BFD_RELOC_NDS32_SDA12S2_DP: bfd_reloc_code_real = 1041;
pub const BFD_RELOC_NDS32_PLT_GOTREL_LO12: bfd_reloc_code_real = 1040;
pub const BFD_RELOC_NDS32_PLT_GOTREL_HI20: bfd_reloc_code_real = 1039;
pub const BFD_RELOC_NDS32_PLTREL_LO12: bfd_reloc_code_real = 1038;
pub const BFD_RELOC_NDS32_PLTREL_HI20: bfd_reloc_code_real = 1037;
pub const BFD_RELOC_NDS32_LONGJUMP7: bfd_reloc_code_real = 1036;
pub const BFD_RELOC_NDS32_LONGJUMP6: bfd_reloc_code_real = 1035;
pub const BFD_RELOC_NDS32_LONGJUMP5: bfd_reloc_code_real = 1034;
pub const BFD_RELOC_NDS32_LONGJUMP4: bfd_reloc_code_real = 1033;
pub const BFD_RELOC_NDS32_LONGCALL6: bfd_reloc_code_real = 1032;
pub const BFD_RELOC_NDS32_LONGCALL5: bfd_reloc_code_real = 1031;
pub const BFD_RELOC_NDS32_LONGCALL4: bfd_reloc_code_real = 1030;
pub const BFD_RELOC_NDS32_25_FIXED: bfd_reloc_code_real = 1029;
pub const BFD_RELOC_NDS32_17_FIXED: bfd_reloc_code_real = 1028;
pub const BFD_RELOC_NDS32_15_FIXED: bfd_reloc_code_real = 1027;
pub const BFD_RELOC_NDS32_9_FIXED: bfd_reloc_code_real = 1026;
pub const BFD_RELOC_NDS32_LOADSTORE: bfd_reloc_code_real = 1025;
pub const BFD_RELOC_NDS32_LONGJUMP3: bfd_reloc_code_real = 1024;
pub const BFD_RELOC_NDS32_LONGJUMP2: bfd_reloc_code_real = 1023;
pub const BFD_RELOC_NDS32_LONGJUMP1: bfd_reloc_code_real = 1022;
pub const BFD_RELOC_NDS32_LONGCALL3: bfd_reloc_code_real = 1021;
pub const BFD_RELOC_NDS32_LONGCALL2: bfd_reloc_code_real = 1020;
pub const BFD_RELOC_NDS32_LONGCALL1: bfd_reloc_code_real = 1019;
pub const BFD_RELOC_NDS32_LABEL: bfd_reloc_code_real = 1018;
pub const BFD_RELOC_NDS32_INSN16: bfd_reloc_code_real = 1017;
pub const BFD_RELOC_NDS32_GOTPC_LO12: bfd_reloc_code_real = 1016;
pub const BFD_RELOC_NDS32_GOTPC_HI20: bfd_reloc_code_real = 1015;
pub const BFD_RELOC_NDS32_GOT_LO12: bfd_reloc_code_real = 1014;
pub const BFD_RELOC_NDS32_GOT_HI20: bfd_reloc_code_real = 1013;
pub const BFD_RELOC_NDS32_GOTPC20: bfd_reloc_code_real = 1012;
pub const BFD_RELOC_NDS32_GOTOFF_LO12: bfd_reloc_code_real = 1011;
pub const BFD_RELOC_NDS32_GOTOFF_HI20: bfd_reloc_code_real = 1010;
pub const BFD_RELOC_NDS32_GOTOFF: bfd_reloc_code_real = 1009;
pub const BFD_RELOC_NDS32_RELATIVE: bfd_reloc_code_real = 1008;
pub const BFD_RELOC_NDS32_JMP_SLOT: bfd_reloc_code_real = 1007;
pub const BFD_RELOC_NDS32_GLOB_DAT: bfd_reloc_code_real = 1006;
pub const BFD_RELOC_NDS32_COPY: bfd_reloc_code_real = 1005;
pub const BFD_RELOC_NDS32_25_PLTREL: bfd_reloc_code_real = 1004;
pub const BFD_RELOC_NDS32_9_PLTREL: bfd_reloc_code_real = 1003;
pub const BFD_RELOC_NDS32_GOT20: bfd_reloc_code_real = 1002;
pub const BFD_RELOC_NDS32_SDA19S0: bfd_reloc_code_real = 1001;
pub const BFD_RELOC_NDS32_SDA18S1: bfd_reloc_code_real = 1000;
pub const BFD_RELOC_NDS32_SDA17S2: bfd_reloc_code_real = 999;
pub const BFD_RELOC_NDS32_SDA16S3: bfd_reloc_code_real = 998;
pub const BFD_RELOC_NDS32_SDA15S0: bfd_reloc_code_real = 997;
pub const BFD_RELOC_NDS32_SDA15S1: bfd_reloc_code_real = 996;
pub const BFD_RELOC_NDS32_SDA15S2: bfd_reloc_code_real = 995;
pub const BFD_RELOC_NDS32_SDA15S3: bfd_reloc_code_real = 994;
pub const BFD_RELOC_NDS32_LO12S0_ORI: bfd_reloc_code_real = 993;
pub const BFD_RELOC_NDS32_LO12S0: bfd_reloc_code_real = 992;
pub const BFD_RELOC_NDS32_LO12S1: bfd_reloc_code_real = 991;
pub const BFD_RELOC_NDS32_LO12S2: bfd_reloc_code_real = 990;
pub const BFD_RELOC_NDS32_LO12S3: bfd_reloc_code_real = 989;
pub const BFD_RELOC_NDS32_HI20: bfd_reloc_code_real = 988;
pub const BFD_RELOC_NDS32_25_PCREL: bfd_reloc_code_real = 987;
pub const BFD_RELOC_NDS32_17_PCREL: bfd_reloc_code_real = 986;
pub const BFD_RELOC_NDS32_15_PCREL: bfd_reloc_code_real = 985;
pub const BFD_RELOC_NDS32_WORD_9_PCREL: bfd_reloc_code_real = 984;
pub const BFD_RELOC_NDS32_9_PCREL: bfd_reloc_code_real = 983;
pub const BFD_RELOC_NDS32_20: bfd_reloc_code_real = 982;
pub const BFD_RELOC_M32R_GOTPC_LO: bfd_reloc_code_real = 981;
pub const BFD_RELOC_M32R_GOTPC_HI_SLO: bfd_reloc_code_real = 980;
pub const BFD_RELOC_M32R_GOTPC_HI_ULO: bfd_reloc_code_real = 979;
pub const BFD_RELOC_M32R_GOT16_LO: bfd_reloc_code_real = 978;
pub const BFD_RELOC_M32R_GOT16_HI_SLO: bfd_reloc_code_real = 977;
pub const BFD_RELOC_M32R_GOT16_HI_ULO: bfd_reloc_code_real = 976;
pub const BFD_RELOC_M32R_GOTPC24: bfd_reloc_code_real = 975;
pub const BFD_RELOC_M32R_GOTOFF_LO: bfd_reloc_code_real = 974;
pub const BFD_RELOC_M32R_GOTOFF_HI_SLO: bfd_reloc_code_real = 973;
pub const BFD_RELOC_M32R_GOTOFF_HI_ULO: bfd_reloc_code_real = 972;
pub const BFD_RELOC_M32R_GOTOFF: bfd_reloc_code_real = 971;
pub const BFD_RELOC_M32R_RELATIVE: bfd_reloc_code_real = 970;
pub const BFD_RELOC_M32R_JMP_SLOT: bfd_reloc_code_real = 969;
pub const BFD_RELOC_M32R_GLOB_DAT: bfd_reloc_code_real = 968;
pub const BFD_RELOC_M32R_COPY: bfd_reloc_code_real = 967;
pub const BFD_RELOC_M32R_26_PLTREL: bfd_reloc_code_real = 966;
pub const BFD_RELOC_M32R_GOT24: bfd_reloc_code_real = 965;
pub const BFD_RELOC_M32R_SDA16: bfd_reloc_code_real = 964;
pub const BFD_RELOC_M32R_LO16: bfd_reloc_code_real = 963;
pub const BFD_RELOC_M32R_HI16_SLO: bfd_reloc_code_real = 962;
pub const BFD_RELOC_M32R_HI16_ULO: bfd_reloc_code_real = 961;
pub const BFD_RELOC_M32R_26_PCREL: bfd_reloc_code_real = 960;
pub const BFD_RELOC_M32R_18_PCREL: bfd_reloc_code_real = 959;
pub const BFD_RELOC_M32R_10_PCREL: bfd_reloc_code_real = 958;
pub const BFD_RELOC_M32R_24: bfd_reloc_code_real = 957;
pub const BFD_RELOC_M32C_RL_2ADDR: bfd_reloc_code_real = 956;
pub const BFD_RELOC_M32C_RL_1ADDR: bfd_reloc_code_real = 955;
pub const BFD_RELOC_M32C_RL_JUMP: bfd_reloc_code_real = 954;
pub const BFD_RELOC_M32C_HI8: bfd_reloc_code_real = 953;
pub const BFD_RELOC_DLX_JMP26: bfd_reloc_code_real = 952;
pub const BFD_RELOC_DLX_LO16: bfd_reloc_code_real = 951;
pub const BFD_RELOC_DLX_HI16_S: bfd_reloc_code_real = 950;
pub const BFD_RELOC_D30V_32_PCREL: bfd_reloc_code_real = 949;
pub const BFD_RELOC_D30V_32: bfd_reloc_code_real = 948;
pub const BFD_RELOC_D30V_21_PCREL_R: bfd_reloc_code_real = 947;
pub const BFD_RELOC_D30V_21_PCREL: bfd_reloc_code_real = 946;
pub const BFD_RELOC_D30V_21: bfd_reloc_code_real = 945;
pub const BFD_RELOC_D30V_15_PCREL_R: bfd_reloc_code_real = 944;
pub const BFD_RELOC_D30V_15_PCREL: bfd_reloc_code_real = 943;
pub const BFD_RELOC_D30V_15: bfd_reloc_code_real = 942;
pub const BFD_RELOC_D30V_9_PCREL_R: bfd_reloc_code_real = 941;
pub const BFD_RELOC_D30V_9_PCREL: bfd_reloc_code_real = 940;
pub const BFD_RELOC_D30V_6: bfd_reloc_code_real = 939;
pub const BFD_RELOC_D10V_18_PCREL: bfd_reloc_code_real = 938;
pub const BFD_RELOC_D10V_18: bfd_reloc_code_real = 937;
pub const BFD_RELOC_D10V_10_PCREL_L: bfd_reloc_code_real = 936;
pub const BFD_RELOC_D10V_10_PCREL_R: bfd_reloc_code_real = 935;
pub const BFD_ARELOC_BFIN_ADDR: bfd_reloc_code_real = 934;
pub const BFD_ARELOC_BFIN_HWPAGE: bfd_reloc_code_real = 933;
pub const BFD_ARELOC_BFIN_PAGE: bfd_reloc_code_real = 932;
pub const BFD_ARELOC_BFIN_COMP: bfd_reloc_code_real = 931;
pub const BFD_ARELOC_BFIN_NEG: bfd_reloc_code_real = 930;
pub const BFD_ARELOC_BFIN_LEN: bfd_reloc_code_real = 929;
pub const BFD_ARELOC_BFIN_LOR: bfd_reloc_code_real = 928;
pub const BFD_ARELOC_BFIN_LAND: bfd_reloc_code_real = 927;
pub const BFD_ARELOC_BFIN_XOR: bfd_reloc_code_real = 926;
pub const BFD_ARELOC_BFIN_OR: bfd_reloc_code_real = 925;
pub const BFD_ARELOC_BFIN_AND: bfd_reloc_code_real = 924;
pub const BFD_ARELOC_BFIN_RSHIFT: bfd_reloc_code_real = 923;
pub const BFD_ARELOC_BFIN_LSHIFT: bfd_reloc_code_real = 922;
pub const BFD_ARELOC_BFIN_MOD: bfd_reloc_code_real = 921;
pub const BFD_ARELOC_BFIN_DIV: bfd_reloc_code_real = 920;
pub const BFD_ARELOC_BFIN_MULT: bfd_reloc_code_real = 919;
pub const BFD_ARELOC_BFIN_SUB: bfd_reloc_code_real = 918;
pub const BFD_ARELOC_BFIN_ADD: bfd_reloc_code_real = 917;
pub const BFD_ARELOC_BFIN_CONST: bfd_reloc_code_real = 916;
pub const BFD_ARELOC_BFIN_PUSH: bfd_reloc_code_real = 915;
pub const BFD_RELOC_BFIN_PLTPC: bfd_reloc_code_real = 914;
pub const BFD_RELOC_BFIN_GOT: bfd_reloc_code_real = 913;
pub const BFD_RELOC_BFIN_GOTOFFLO: bfd_reloc_code_real = 912;
pub const BFD_RELOC_BFIN_GOTOFFHI: bfd_reloc_code_real = 911;
pub const BFD_RELOC_BFIN_GOTOFF17M4: bfd_reloc_code_real = 910;
pub const BFD_RELOC_BFIN_FUNCDESC_GOTOFFLO: bfd_reloc_code_real = 909;
pub const BFD_RELOC_BFIN_FUNCDESC_GOTOFFHI: bfd_reloc_code_real = 908;
pub const BFD_RELOC_BFIN_FUNCDESC_GOTOFF17M4: bfd_reloc_code_real = 907;
pub const BFD_RELOC_BFIN_FUNCDESC_VALUE: bfd_reloc_code_real = 906;
pub const BFD_RELOC_BFIN_FUNCDESC_GOTLO: bfd_reloc_code_real = 905;
pub const BFD_RELOC_BFIN_FUNCDESC_GOTHI: bfd_reloc_code_real = 904;
pub const BFD_RELOC_BFIN_FUNCDESC_GOT17M4: bfd_reloc_code_real = 903;
pub const BFD_RELOC_BFIN_FUNCDESC: bfd_reloc_code_real = 902;
pub const BFD_RELOC_BFIN_GOTLO: bfd_reloc_code_real = 901;
pub const BFD_RELOC_BFIN_GOTHI: bfd_reloc_code_real = 900;
pub const BFD_RELOC_BFIN_GOT17M4: bfd_reloc_code_real = 899;
pub const BFD_RELOC_BFIN_24_PCREL_JUMP_L: bfd_reloc_code_real = 898;
pub const BFD_RELOC_BFIN_24_PCREL_CALL_X: bfd_reloc_code_real = 897;
pub const BFD_RELOC_BFIN_12_PCREL_JUMP_S: bfd_reloc_code_real = 896;
pub const BFD_RELOC_BFIN_12_PCREL_JUMP: bfd_reloc_code_real = 895;
pub const BFD_RELOC_BFIN_11_PCREL: bfd_reloc_code_real = 894;
pub const BFD_RELOC_BFIN_10_PCREL: bfd_reloc_code_real = 893;
pub const BFD_RELOC_BFIN_16_LOW: bfd_reloc_code_real = 892;
pub const BFD_RELOC_BFIN_5_PCREL: bfd_reloc_code_real = 891;
pub const BFD_RELOC_BFIN_4_PCREL: bfd_reloc_code_real = 890;
pub const BFD_RELOC_BFIN_16_HIGH: bfd_reloc_code_real = 889;
pub const BFD_RELOC_BFIN_16_IMM: bfd_reloc_code_real = 888;
pub const BFD_RELOC_ARC_JLI_SECTOFF: bfd_reloc_code_real = 887;
pub const BFD_RELOC_ARC_NPS_CMEM16: bfd_reloc_code_real = 886;
pub const BFD_RELOC_ARC_S21H_PCREL_PLT: bfd_reloc_code_real = 885;
pub const BFD_RELOC_ARC_S25W_PCREL_PLT: bfd_reloc_code_real = 884;
pub const BFD_RELOC_ARC_TLS_LE_32: bfd_reloc_code_real = 883;
pub const BFD_RELOC_ARC_TLS_LE_S9: bfd_reloc_code_real = 882;
pub const BFD_RELOC_ARC_TLS_DTPOFF_S9: bfd_reloc_code_real = 881;
pub const BFD_RELOC_ARC_TLS_DTPOFF: bfd_reloc_code_real = 880;
pub const BFD_RELOC_ARC_TLS_IE_GOT: bfd_reloc_code_real = 879;
pub const BFD_RELOC_ARC_TLS_GD_CALL: bfd_reloc_code_real = 878;
pub const BFD_RELOC_ARC_TLS_GD_LD: bfd_reloc_code_real = 877;
pub const BFD_RELOC_ARC_TLS_GD_GOT: bfd_reloc_code_real = 876;
pub const BFD_RELOC_ARC_TLS_TPOFF: bfd_reloc_code_real = 875;
pub const BFD_RELOC_ARC_TLS_DTPMOD: bfd_reloc_code_real = 874;
pub const BFD_RELOC_ARC_S25H_PCREL_PLT: bfd_reloc_code_real = 873;
pub const BFD_RELOC_ARC_S21W_PCREL_PLT: bfd_reloc_code_real = 872;
pub const BFD_RELOC_ARC_GOTPC: bfd_reloc_code_real = 871;
pub const BFD_RELOC_ARC_GOTOFF: bfd_reloc_code_real = 870;
pub const BFD_RELOC_ARC_RELATIVE: bfd_reloc_code_real = 869;
pub const BFD_RELOC_ARC_JMP_SLOT: bfd_reloc_code_real = 868;
pub const BFD_RELOC_ARC_GLOB_DAT: bfd_reloc_code_real = 867;
pub const BFD_RELOC_ARC_COPY: bfd_reloc_code_real = 866;
pub const BFD_RELOC_ARC_PLT32: bfd_reloc_code_real = 865;
pub const BFD_RELOC_ARC_GOTPC32: bfd_reloc_code_real = 864;
pub const BFD_RELOC_ARC_GOT32: bfd_reloc_code_real = 863;
pub const BFD_RELOC_ARC_PC32: bfd_reloc_code_real = 862;
pub const BFD_RELOC_ARC_32_PCREL: bfd_reloc_code_real = 861;
pub const BFD_RELOC_ARC_SDA16_ST2: bfd_reloc_code_real = 860;
pub const BFD_RELOC_ARC_SDA_12: bfd_reloc_code_real = 859;
pub const BFD_RELOC_ARC_SECTOFF_2: bfd_reloc_code_real = 858;
pub const BFD_RELOC_ARC_SECTOFF_1: bfd_reloc_code_real = 857;
pub const BFD_RELOC_ARC_SECTOFF_ME_2: bfd_reloc_code_real = 856;
pub const BFD_RELOC_ARC_SECTOFF_ME_1: bfd_reloc_code_real = 855;
pub const BFD_RELOC_AC_SECTOFF_S9_2: bfd_reloc_code_real = 854;
pub const BFD_RELOC_AC_SECTOFF_S9_1: bfd_reloc_code_real = 853;
pub const BFD_RELOC_AC_SECTOFF_S9: bfd_reloc_code_real = 852;
pub const BFD_RELOC_AC_SECTOFF_U8_2: bfd_reloc_code_real = 851;
pub const BFD_RELOC_AC_SECTOFF_U8_1: bfd_reloc_code_real = 850;
pub const BFD_RELOC_AC_SECTOFF_U8: bfd_reloc_code_real = 849;
pub const BFD_RELOC_ARC_W_ME: bfd_reloc_code_real = 848;
pub const BFD_RELOC_ARC_SDA32_ME: bfd_reloc_code_real = 847;
pub const BFD_RELOC_ARC_SECTOFF_ME: bfd_reloc_code_real = 846;
pub const BFD_RELOC_ARC_N32_ME: bfd_reloc_code_real = 845;
pub const BFD_RELOC_ARC_32_ME_S: bfd_reloc_code_real = 844;
pub const BFD_RELOC_ARC_32_ME: bfd_reloc_code_real = 843;
pub const BFD_RELOC_ARC_W: bfd_reloc_code_real = 842;
pub const BFD_RELOC_ARC_S13_PCREL: bfd_reloc_code_real = 841;
pub const BFD_RELOC_ARC_SDA16_LD2: bfd_reloc_code_real = 840;
pub const BFD_RELOC_ARC_SDA16_LD1: bfd_reloc_code_real = 839;
pub const BFD_RELOC_ARC_SDA16_LD: bfd_reloc_code_real = 838;
pub const BFD_RELOC_ARC_SDA_LDST2: bfd_reloc_code_real = 837;
pub const BFD_RELOC_ARC_SDA_LDST1: bfd_reloc_code_real = 836;
pub const BFD_RELOC_ARC_SDA_LDST: bfd_reloc_code_real = 835;
pub const BFD_RELOC_ARC_SDA32: bfd_reloc_code_real = 834;
pub const BFD_RELOC_ARC_S25W_PCREL: bfd_reloc_code_real = 833;
pub const BFD_RELOC_ARC_S25H_PCREL: bfd_reloc_code_real = 832;
pub const BFD_RELOC_ARC_S21W_PCREL: bfd_reloc_code_real = 831;
pub const BFD_RELOC_ARC_S21H_PCREL: bfd_reloc_code_real = 830;
pub const BFD_RELOC_ARC_SECTOFF: bfd_reloc_code_real = 829;
pub const BFD_RELOC_ARC_SDA: bfd_reloc_code_real = 828;
pub const BFD_RELOC_ARC_N32: bfd_reloc_code_real = 827;
pub const BFD_RELOC_ARC_N24: bfd_reloc_code_real = 826;
pub const BFD_RELOC_ARC_N16: bfd_reloc_code_real = 825;
pub const BFD_RELOC_ARC_N8: bfd_reloc_code_real = 824;
pub const BFD_RELOC_ARC_32: bfd_reloc_code_real = 823;
pub const BFD_RELOC_ARC_24: bfd_reloc_code_real = 822;
pub const BFD_RELOC_ARC_16: bfd_reloc_code_real = 821;
pub const BFD_RELOC_ARC_8: bfd_reloc_code_real = 820;
pub const BFD_RELOC_ARC_NONE: bfd_reloc_code_real = 819;
pub const BFD_RELOC_SH_FUNCDESC: bfd_reloc_code_real = 818;
pub const BFD_RELOC_SH_GOTOFFFUNCDESC20: bfd_reloc_code_real = 817;
pub const BFD_RELOC_SH_GOTOFFFUNCDESC: bfd_reloc_code_real = 816;
pub const BFD_RELOC_SH_GOTFUNCDESC20: bfd_reloc_code_real = 815;
pub const BFD_RELOC_SH_GOTFUNCDESC: bfd_reloc_code_real = 814;
pub const BFD_RELOC_SH_GOTOFF20: bfd_reloc_code_real = 813;
pub const BFD_RELOC_SH_GOT20: bfd_reloc_code_real = 812;
pub const BFD_RELOC_SH_TLS_TPOFF32: bfd_reloc_code_real = 811;
pub const BFD_RELOC_SH_TLS_DTPOFF32: bfd_reloc_code_real = 810;
pub const BFD_RELOC_SH_TLS_DTPMOD32: bfd_reloc_code_real = 809;
pub const BFD_RELOC_SH_TLS_LE_32: bfd_reloc_code_real = 808;
pub const BFD_RELOC_SH_TLS_IE_32: bfd_reloc_code_real = 807;
pub const BFD_RELOC_SH_TLS_LDO_32: bfd_reloc_code_real = 806;
pub const BFD_RELOC_SH_TLS_LD_32: bfd_reloc_code_real = 805;
pub const BFD_RELOC_SH_TLS_GD_32: bfd_reloc_code_real = 804;
pub const BFD_RELOC_SH_PT_16: bfd_reloc_code_real = 803;
pub const BFD_RELOC_SH_IMM_HI16_PCREL: bfd_reloc_code_real = 802;
pub const BFD_RELOC_SH_IMM_HI16: bfd_reloc_code_real = 801;
pub const BFD_RELOC_SH_IMM_MEDHI16_PCREL: bfd_reloc_code_real = 800;
pub const BFD_RELOC_SH_IMM_MEDHI16: bfd_reloc_code_real = 799;
pub const BFD_RELOC_SH_IMM_MEDLOW16_PCREL: bfd_reloc_code_real = 798;
pub const BFD_RELOC_SH_IMM_MEDLOW16: bfd_reloc_code_real = 797;
pub const BFD_RELOC_SH_IMM_LOW16_PCREL: bfd_reloc_code_real = 796;
pub const BFD_RELOC_SH_IMM_LOW16: bfd_reloc_code_real = 795;
pub const BFD_RELOC_SH_IMMU16: bfd_reloc_code_real = 794;
pub const BFD_RELOC_SH_IMMS16: bfd_reloc_code_real = 793;
pub const BFD_RELOC_SH_IMMS10BY8: bfd_reloc_code_real = 792;
pub const BFD_RELOC_SH_IMMS10BY4: bfd_reloc_code_real = 791;
pub const BFD_RELOC_SH_IMMS10BY2: bfd_reloc_code_real = 790;
pub const BFD_RELOC_SH_IMMS10: bfd_reloc_code_real = 789;
pub const BFD_RELOC_SH_IMMU6: bfd_reloc_code_real = 788;
pub const BFD_RELOC_SH_IMMS6BY32: bfd_reloc_code_real = 787;
pub const BFD_RELOC_SH_IMMS6: bfd_reloc_code_real = 786;
pub const BFD_RELOC_SH_IMMU5: bfd_reloc_code_real = 785;
pub const BFD_RELOC_SH_SHMEDIA_CODE: bfd_reloc_code_real = 784;
pub const BFD_RELOC_SH_GOTPLT32: bfd_reloc_code_real = 783;
pub const BFD_RELOC_SH_GOTPLT10BY8: bfd_reloc_code_real = 782;
pub const BFD_RELOC_SH_GOTPLT10BY4: bfd_reloc_code_real = 781;
pub const BFD_RELOC_SH_GOT10BY8: bfd_reloc_code_real = 780;
pub const BFD_RELOC_SH_GOT10BY4: bfd_reloc_code_real = 779;
pub const BFD_RELOC_SH_RELATIVE64: bfd_reloc_code_real = 778;
pub const BFD_RELOC_SH_JMP_SLOT64: bfd_reloc_code_real = 777;
pub const BFD_RELOC_SH_GLOB_DAT64: bfd_reloc_code_real = 776;
pub const BFD_RELOC_SH_COPY64: bfd_reloc_code_real = 775;
pub const BFD_RELOC_SH_GOTPC_HI16: bfd_reloc_code_real = 774;
pub const BFD_RELOC_SH_GOTPC_MEDHI16: bfd_reloc_code_real = 773;
pub const BFD_RELOC_SH_GOTPC_MEDLOW16: bfd_reloc_code_real = 772;
pub const BFD_RELOC_SH_GOTPC_LOW16: bfd_reloc_code_real = 771;
pub const BFD_RELOC_SH_GOTOFF_HI16: bfd_reloc_code_real = 770;
pub const BFD_RELOC_SH_GOTOFF_MEDHI16: bfd_reloc_code_real = 769;
pub const BFD_RELOC_SH_GOTOFF_MEDLOW16: bfd_reloc_code_real = 768;
pub const BFD_RELOC_SH_GOTOFF_LOW16: bfd_reloc_code_real = 767;
pub const BFD_RELOC_SH_PLT_HI16: bfd_reloc_code_real = 766;
pub const BFD_RELOC_SH_PLT_MEDHI16: bfd_reloc_code_real = 765;
pub const BFD_RELOC_SH_PLT_MEDLOW16: bfd_reloc_code_real = 764;
pub const BFD_RELOC_SH_PLT_LOW16: bfd_reloc_code_real = 763;
pub const BFD_RELOC_SH_GOTPLT_HI16: bfd_reloc_code_real = 762;
pub const BFD_RELOC_SH_GOTPLT_MEDHI16: bfd_reloc_code_real = 761;
pub const BFD_RELOC_SH_GOTPLT_MEDLOW16: bfd_reloc_code_real = 760;
pub const BFD_RELOC_SH_GOTPLT_LOW16: bfd_reloc_code_real = 759;
pub const BFD_RELOC_SH_GOT_HI16: bfd_reloc_code_real = 758;
pub const BFD_RELOC_SH_GOT_MEDHI16: bfd_reloc_code_real = 757;
pub const BFD_RELOC_SH_GOT_MEDLOW16: bfd_reloc_code_real = 756;
pub const BFD_RELOC_SH_GOT_LOW16: bfd_reloc_code_real = 755;
pub const BFD_RELOC_SH_GOTPC: bfd_reloc_code_real = 754;
pub const BFD_RELOC_SH_RELATIVE: bfd_reloc_code_real = 753;
pub const BFD_RELOC_SH_JMP_SLOT: bfd_reloc_code_real = 752;
pub const BFD_RELOC_SH_GLOB_DAT: bfd_reloc_code_real = 751;
pub const BFD_RELOC_SH_COPY: bfd_reloc_code_real = 750;
pub const BFD_RELOC_SH_LOOP_END: bfd_reloc_code_real = 749;
pub const BFD_RELOC_SH_LOOP_START: bfd_reloc_code_real = 748;
pub const BFD_RELOC_SH_LABEL: bfd_reloc_code_real = 747;
pub const BFD_RELOC_SH_DATA: bfd_reloc_code_real = 746;
pub const BFD_RELOC_SH_CODE: bfd_reloc_code_real = 745;
pub const BFD_RELOC_SH_ALIGN: bfd_reloc_code_real = 744;
pub const BFD_RELOC_SH_COUNT: bfd_reloc_code_real = 743;
pub const BFD_RELOC_SH_USES: bfd_reloc_code_real = 742;
pub const BFD_RELOC_SH_SWITCH32: bfd_reloc_code_real = 741;
pub const BFD_RELOC_SH_SWITCH16: bfd_reloc_code_real = 740;
pub const BFD_RELOC_SH_PCRELIMM8BY4: bfd_reloc_code_real = 739;
pub const BFD_RELOC_SH_PCRELIMM8BY2: bfd_reloc_code_real = 738;
pub const BFD_RELOC_SH_IMM8BY4: bfd_reloc_code_real = 737;
pub const BFD_RELOC_SH_IMM8BY2: bfd_reloc_code_real = 736;
pub const BFD_RELOC_SH_IMM8: bfd_reloc_code_real = 735;
pub const BFD_RELOC_SH_IMM4BY4: bfd_reloc_code_real = 734;
pub const BFD_RELOC_SH_IMM4BY2: bfd_reloc_code_real = 733;
pub const BFD_RELOC_SH_IMM4: bfd_reloc_code_real = 732;
pub const BFD_RELOC_SH_DISP20BY8: bfd_reloc_code_real = 731;
pub const BFD_RELOC_SH_DISP20: bfd_reloc_code_real = 730;
pub const BFD_RELOC_SH_DISP12BY8: bfd_reloc_code_real = 729;
pub const BFD_RELOC_SH_DISP12BY4: bfd_reloc_code_real = 728;
pub const BFD_RELOC_SH_DISP12BY2: bfd_reloc_code_real = 727;
pub const BFD_RELOC_SH_DISP12: bfd_reloc_code_real = 726;
pub const BFD_RELOC_SH_IMM3U: bfd_reloc_code_real = 725;
pub const BFD_RELOC_SH_IMM3: bfd_reloc_code_real = 724;
pub const BFD_RELOC_SH_PCDISP12BY2: bfd_reloc_code_real = 723;
pub const BFD_RELOC_SH_PCDISP8BY2: bfd_reloc_code_real = 722;
pub const BFD_RELOC_ARM_THUMB_SHIFT: bfd_reloc_code_real = 721;
pub const BFD_RELOC_ARM_THUMB_IMM: bfd_reloc_code_real = 720;
pub const BFD_RELOC_ARM_THUMB_ADD: bfd_reloc_code_real = 719;
pub const BFD_RELOC_ARM_HWLITERAL: bfd_reloc_code_real = 718;
pub const BFD_RELOC_ARM_T32_OFFSET_IMM: bfd_reloc_code_real = 717;
pub const BFD_RELOC_ARM_T32_OFFSET_U8: bfd_reloc_code_real = 716;
pub const BFD_RELOC_ARM_OFFSET_IMM8: bfd_reloc_code_real = 715;
pub const BFD_RELOC_ARM_IN_POOL: bfd_reloc_code_real = 714;
pub const BFD_RELOC_ARM_LITERAL: bfd_reloc_code_real = 713;
pub const BFD_RELOC_ARM_LDR_IMM: bfd_reloc_code_real = 712;
pub const BFD_RELOC_ARM_ADR_IMM: bfd_reloc_code_real = 711;
pub const BFD_RELOC_ARM_T32_VLDR_VSTR_OFF_IMM: bfd_reloc_code_real = 710;
pub const BFD_RELOC_ARM_T32_CP_OFF_IMM_S2: bfd_reloc_code_real = 709;
pub const BFD_RELOC_ARM_T32_CP_OFF_IMM: bfd_reloc_code_real = 708;
pub const BFD_RELOC_ARM_CP_OFF_IMM_S2: bfd_reloc_code_real = 707;
pub const BFD_RELOC_ARM_CP_OFF_IMM: bfd_reloc_code_real = 706;
pub const BFD_RELOC_ARM_MULTI: bfd_reloc_code_real = 705;
pub const BFD_RELOC_ARM_SWI: bfd_reloc_code_real = 704;
pub const BFD_RELOC_ARM_HVC: bfd_reloc_code_real = 703;
pub const BFD_RELOC_ARM_SMC: bfd_reloc_code_real = 702;
pub const BFD_RELOC_ARM_SHIFT_IMM: bfd_reloc_code_real = 701;
pub const BFD_RELOC_ARM_T32_ADD_PC12: bfd_reloc_code_real = 700;
pub const BFD_RELOC_ARM_T32_IMM12: bfd_reloc_code_real = 699;
pub const BFD_RELOC_ARM_T32_ADD_IMM: bfd_reloc_code_real = 698;
pub const BFD_RELOC_ARM_T32_IMMEDIATE: bfd_reloc_code_real = 697;
pub const BFD_RELOC_ARM_ADRL_IMMEDIATE: bfd_reloc_code_real = 696;
pub const BFD_RELOC_ARM_IMMEDIATE: bfd_reloc_code_real = 695;
pub const BFD_RELOC_ARM_THUMB_ALU_ABS_G3_NC: bfd_reloc_code_real = 694;
pub const BFD_RELOC_ARM_THUMB_ALU_ABS_G2_NC: bfd_reloc_code_real = 693;
pub const BFD_RELOC_ARM_THUMB_ALU_ABS_G1_NC: bfd_reloc_code_real = 692;
pub const BFD_RELOC_ARM_THUMB_ALU_ABS_G0_NC: bfd_reloc_code_real = 691;
pub const BFD_RELOC_ARM_IRELATIVE: bfd_reloc_code_real = 690;
pub const BFD_RELOC_ARM_V4BX: bfd_reloc_code_real = 689;
pub const BFD_RELOC_ARM_LDC_SB_G2: bfd_reloc_code_real = 688;
pub const BFD_RELOC_ARM_LDC_SB_G1: bfd_reloc_code_real = 687;
pub const BFD_RELOC_ARM_LDC_SB_G0: bfd_reloc_code_real = 686;
pub const BFD_RELOC_ARM_LDRS_SB_G2: bfd_reloc_code_real = 685;
pub const BFD_RELOC_ARM_LDRS_SB_G1: bfd_reloc_code_real = 684;
pub const BFD_RELOC_ARM_LDRS_SB_G0: bfd_reloc_code_real = 683;
pub const BFD_RELOC_ARM_LDR_SB_G2: bfd_reloc_code_real = 682;
pub const BFD_RELOC_ARM_LDR_SB_G1: bfd_reloc_code_real = 681;
pub const BFD_RELOC_ARM_LDR_SB_G0: bfd_reloc_code_real = 680;
pub const BFD_RELOC_ARM_ALU_SB_G2: bfd_reloc_code_real = 679;
pub const BFD_RELOC_ARM_ALU_SB_G1: bfd_reloc_code_real = 678;
pub const BFD_RELOC_ARM_ALU_SB_G1_NC: bfd_reloc_code_real = 677;
pub const BFD_RELOC_ARM_ALU_SB_G0: bfd_reloc_code_real = 676;
pub const BFD_RELOC_ARM_ALU_SB_G0_NC: bfd_reloc_code_real = 675;
pub const BFD_RELOC_ARM_LDC_PC_G2: bfd_reloc_code_real = 674;
pub const BFD_RELOC_ARM_LDC_PC_G1: bfd_reloc_code_real = 673;
pub const BFD_RELOC_ARM_LDC_PC_G0: bfd_reloc_code_real = 672;
pub const BFD_RELOC_ARM_LDRS_PC_G2: bfd_reloc_code_real = 671;
pub const BFD_RELOC_ARM_LDRS_PC_G1: bfd_reloc_code_real = 670;
pub const BFD_RELOC_ARM_LDRS_PC_G0: bfd_reloc_code_real = 669;
pub const BFD_RELOC_ARM_LDR_PC_G2: bfd_reloc_code_real = 668;
pub const BFD_RELOC_ARM_LDR_PC_G1: bfd_reloc_code_real = 667;
pub const BFD_RELOC_ARM_LDR_PC_G0: bfd_reloc_code_real = 666;
pub const BFD_RELOC_ARM_ALU_PC_G2: bfd_reloc_code_real = 665;
pub const BFD_RELOC_ARM_ALU_PC_G1: bfd_reloc_code_real = 664;
pub const BFD_RELOC_ARM_ALU_PC_G1_NC: bfd_reloc_code_real = 663;
pub const BFD_RELOC_ARM_ALU_PC_G0: bfd_reloc_code_real = 662;
pub const BFD_RELOC_ARM_ALU_PC_G0_NC: bfd_reloc_code_real = 661;
pub const BFD_RELOC_ARM_TLS_DESC: bfd_reloc_code_real = 660;
pub const BFD_RELOC_ARM_THM_TLS_DESCSEQ: bfd_reloc_code_real = 659;
pub const BFD_RELOC_ARM_TLS_DESCSEQ: bfd_reloc_code_real = 658;
pub const BFD_RELOC_ARM_THM_TLS_CALL: bfd_reloc_code_real = 657;
pub const BFD_RELOC_ARM_TLS_CALL: bfd_reloc_code_real = 656;
pub const BFD_RELOC_ARM_TLS_GOTDESC: bfd_reloc_code_real = 655;
pub const BFD_RELOC_ARM_TLS_LE32: bfd_reloc_code_real = 654;
pub const BFD_RELOC_ARM_TLS_IE32: bfd_reloc_code_real = 653;
pub const BFD_RELOC_ARM_TLS_TPOFF32: bfd_reloc_code_real = 652;
pub const BFD_RELOC_ARM_TLS_DTPMOD32: bfd_reloc_code_real = 651;
pub const BFD_RELOC_ARM_TLS_DTPOFF32: bfd_reloc_code_real = 650;
pub const BFD_RELOC_ARM_TLS_LDM32: bfd_reloc_code_real = 649;
pub const BFD_RELOC_ARM_TLS_LDO32: bfd_reloc_code_real = 648;
pub const BFD_RELOC_ARM_TLS_GD32: bfd_reloc_code_real = 647;
pub const BFD_RELOC_ARM_GOT_PREL: bfd_reloc_code_real = 646;
pub const BFD_RELOC_ARM_GOTPC: bfd_reloc_code_real = 645;
pub const BFD_RELOC_ARM_GOTOFF: bfd_reloc_code_real = 644;
pub const BFD_RELOC_ARM_RELATIVE: bfd_reloc_code_real = 643;
pub const BFD_RELOC_ARM_PLT32: bfd_reloc_code_real = 642;
pub const BFD_RELOC_ARM_GOT32: bfd_reloc_code_real = 641;
pub const BFD_RELOC_ARM_GLOB_DAT: bfd_reloc_code_real = 640;
pub const BFD_RELOC_ARM_JUMP_SLOT: bfd_reloc_code_real = 639;
pub const BFD_RELOC_ARM_TLS_IE32_FDPIC: bfd_reloc_code_real = 638;
pub const BFD_RELOC_ARM_TLS_LDM32_FDPIC: bfd_reloc_code_real = 637;
pub const BFD_RELOC_ARM_TLS_GD32_FDPIC: bfd_reloc_code_real = 636;
pub const BFD_RELOC_ARM_FUNCDESC_VALUE: bfd_reloc_code_real = 635;
pub const BFD_RELOC_ARM_FUNCDESC: bfd_reloc_code_real = 634;
pub const BFD_RELOC_ARM_GOTOFFFUNCDESC: bfd_reloc_code_real = 633;
pub const BFD_RELOC_ARM_GOTFUNCDESC: bfd_reloc_code_real = 632;
pub const BFD_RELOC_ARM_THUMB_MOVT_PCREL: bfd_reloc_code_real = 631;
pub const BFD_RELOC_ARM_THUMB_MOVW_PCREL: bfd_reloc_code_real = 630;
pub const BFD_RELOC_ARM_THUMB_MOVT: bfd_reloc_code_real = 629;
pub const BFD_RELOC_ARM_THUMB_MOVW: bfd_reloc_code_real = 628;
pub const BFD_RELOC_ARM_MOVT_PCREL: bfd_reloc_code_real = 627;
pub const BFD_RELOC_ARM_MOVW_PCREL: bfd_reloc_code_real = 626;
pub const BFD_RELOC_ARM_MOVT: bfd_reloc_code_real = 625;
pub const BFD_RELOC_ARM_MOVW: bfd_reloc_code_real = 624;
pub const BFD_RELOC_ARM_PREL31: bfd_reloc_code_real = 623;
pub const BFD_RELOC_ARM_TARGET2: bfd_reloc_code_real = 622;
pub const BFD_RELOC_ARM_SBREL32: bfd_reloc_code_real = 621;
pub const BFD_RELOC_ARM_ROSEGREL32: bfd_reloc_code_real = 620;
pub const BFD_RELOC_ARM_TARGET1: bfd_reloc_code_real = 619;
pub const BFD_RELOC_ARM_THUMB_OFFSET: bfd_reloc_code_real = 618;
pub const BFD_RELOC_ARM_OFFSET_IMM: bfd_reloc_code_real = 617;
pub const BFD_RELOC_THUMB_PCREL_BRANCH25: bfd_reloc_code_real = 616;
pub const BFD_RELOC_THUMB_PCREL_BRANCH23: bfd_reloc_code_real = 615;
pub const BFD_RELOC_THUMB_PCREL_BRANCH20: bfd_reloc_code_real = 614;
pub const BFD_RELOC_THUMB_PCREL_BRANCH12: bfd_reloc_code_real = 613;
pub const BFD_RELOC_THUMB_PCREL_BRANCH9: bfd_reloc_code_real = 612;
pub const BFD_RELOC_THUMB_PCREL_BRANCH7: bfd_reloc_code_real = 611;
pub const BFD_RELOC_ARM_THUMB_LOOP12: bfd_reloc_code_real = 610;
pub const BFD_RELOC_ARM_THUMB_BF19: bfd_reloc_code_real = 609;
pub const BFD_RELOC_ARM_THUMB_BF13: bfd_reloc_code_real = 608;
pub const BFD_RELOC_ARM_THUMB_BF17: bfd_reloc_code_real = 607;
pub const BFD_RELOC_THUMB_PCREL_BFCSEL: bfd_reloc_code_real = 606;
pub const BFD_RELOC_THUMB_PCREL_BRANCH5: bfd_reloc_code_real = 605;
pub const BFD_RELOC_ARM_PCREL_JUMP: bfd_reloc_code_real = 604;
pub const BFD_RELOC_ARM_PCREL_CALL: bfd_reloc_code_real = 603;
pub const BFD_RELOC_THUMB_PCREL_BLX: bfd_reloc_code_real = 602;
pub const BFD_RELOC_ARM_PCREL_BLX: bfd_reloc_code_real = 601;
pub const BFD_RELOC_ARM_PCREL_BRANCH: bfd_reloc_code_real = 600;
pub const BFD_RELOC_CTOR: bfd_reloc_code_real = 599;
pub const BFD_RELOC_I370_D12: bfd_reloc_code_real = 598;
pub const BFD_RELOC_PPC64_TLS_PCREL: bfd_reloc_code_real = 597;
pub const BFD_RELOC_PPC64_GOT_DTPREL_PCREL34: bfd_reloc_code_real = 596;
pub const BFD_RELOC_PPC64_GOT_TPREL_PCREL34: bfd_reloc_code_real = 595;
pub const BFD_RELOC_PPC64_GOT_TLSLD_PCREL34: bfd_reloc_code_real = 594;
pub const BFD_RELOC_PPC64_GOT_TLSGD_PCREL34: bfd_reloc_code_real = 593;
pub const BFD_RELOC_PPC64_DTPREL34: bfd_reloc_code_real = 592;
pub const BFD_RELOC_PPC64_TPREL34: bfd_reloc_code_real = 591;
pub const BFD_RELOC_PPC64_DTPREL16_HIGHESTA: bfd_reloc_code_real = 590;
pub const BFD_RELOC_PPC64_DTPREL16_HIGHEST: bfd_reloc_code_real = 589;
pub const BFD_RELOC_PPC64_DTPREL16_HIGHERA: bfd_reloc_code_real = 588;
pub const BFD_RELOC_PPC64_DTPREL16_HIGHER: bfd_reloc_code_real = 587;
pub const BFD_RELOC_PPC64_DTPREL16_HIGHA: bfd_reloc_code_real = 586;
pub const BFD_RELOC_PPC64_DTPREL16_HIGH: bfd_reloc_code_real = 585;
pub const BFD_RELOC_PPC64_DTPREL16_LO_DS: bfd_reloc_code_real = 584;
pub const BFD_RELOC_PPC64_DTPREL16_DS: bfd_reloc_code_real = 583;
pub const BFD_RELOC_PPC64_TPREL16_HIGHESTA: bfd_reloc_code_real = 582;
pub const BFD_RELOC_PPC64_TPREL16_HIGHEST: bfd_reloc_code_real = 581;
pub const BFD_RELOC_PPC64_TPREL16_HIGHERA: bfd_reloc_code_real = 580;
pub const BFD_RELOC_PPC64_TPREL16_HIGHER: bfd_reloc_code_real = 579;
pub const BFD_RELOC_PPC64_TPREL16_HIGHA: bfd_reloc_code_real = 578;
pub const BFD_RELOC_PPC64_TPREL16_HIGH: bfd_reloc_code_real = 577;
pub const BFD_RELOC_PPC64_TPREL16_LO_DS: bfd_reloc_code_real = 576;
pub const BFD_RELOC_PPC64_TPREL16_DS: bfd_reloc_code_real = 575;
pub const BFD_RELOC_PPC64_TLSML: bfd_reloc_code_real = 574;
pub const BFD_RELOC_PPC64_TLSM: bfd_reloc_code_real = 573;
pub const BFD_RELOC_PPC64_TLSIE: bfd_reloc_code_real = 572;
pub const BFD_RELOC_PPC64_TLSLE: bfd_reloc_code_real = 571;
pub const BFD_RELOC_PPC64_TLSLD: bfd_reloc_code_real = 570;
pub const BFD_RELOC_PPC64_TLSGD: bfd_reloc_code_real = 569;
pub const BFD_RELOC_PPC_GOT_DTPREL16_HA: bfd_reloc_code_real = 568;
pub const BFD_RELOC_PPC_GOT_DTPREL16_HI: bfd_reloc_code_real = 567;
pub const BFD_RELOC_PPC_GOT_DTPREL16_LO: bfd_reloc_code_real = 566;
pub const BFD_RELOC_PPC_GOT_DTPREL16: bfd_reloc_code_real = 565;
pub const BFD_RELOC_PPC_GOT_TPREL16_HA: bfd_reloc_code_real = 564;
pub const BFD_RELOC_PPC_GOT_TPREL16_HI: bfd_reloc_code_real = 563;
pub const BFD_RELOC_PPC_GOT_TPREL16_LO: bfd_reloc_code_real = 562;
pub const BFD_RELOC_PPC_GOT_TPREL16: bfd_reloc_code_real = 561;
pub const BFD_RELOC_PPC_GOT_TLSLD16_HA: bfd_reloc_code_real = 560;
pub const BFD_RELOC_PPC_GOT_TLSLD16_HI: bfd_reloc_code_real = 559;
pub const BFD_RELOC_PPC_GOT_TLSLD16_LO: bfd_reloc_code_real = 558;
pub const BFD_RELOC_PPC_GOT_TLSLD16: bfd_reloc_code_real = 557;
pub const BFD_RELOC_PPC_GOT_TLSGD16_HA: bfd_reloc_code_real = 556;
pub const BFD_RELOC_PPC_GOT_TLSGD16_HI: bfd_reloc_code_real = 555;
pub const BFD_RELOC_PPC_GOT_TLSGD16_LO: bfd_reloc_code_real = 554;
pub const BFD_RELOC_PPC_GOT_TLSGD16: bfd_reloc_code_real = 553;
pub const BFD_RELOC_PPC_DTPREL: bfd_reloc_code_real = 552;
pub const BFD_RELOC_PPC_DTPREL16_HA: bfd_reloc_code_real = 551;
pub const BFD_RELOC_PPC_DTPREL16_HI: bfd_reloc_code_real = 550;
pub const BFD_RELOC_PPC_DTPREL16_LO: bfd_reloc_code_real = 549;
pub const BFD_RELOC_PPC_DTPREL16: bfd_reloc_code_real = 548;
pub const BFD_RELOC_PPC_TPREL: bfd_reloc_code_real = 547;
pub const BFD_RELOC_PPC_TPREL16_HA: bfd_reloc_code_real = 546;
pub const BFD_RELOC_PPC_TPREL16_HI: bfd_reloc_code_real = 545;
pub const BFD_RELOC_PPC_TPREL16_LO: bfd_reloc_code_real = 544;
pub const BFD_RELOC_PPC_TPREL16: bfd_reloc_code_real = 543;
pub const BFD_RELOC_PPC_DTPMOD: bfd_reloc_code_real = 542;
pub const BFD_RELOC_PPC_TLSML: bfd_reloc_code_real = 541;
pub const BFD_RELOC_PPC_TLSM: bfd_reloc_code_real = 540;
pub const BFD_RELOC_PPC_TLSIE: bfd_reloc_code_real = 539;
pub const BFD_RELOC_PPC_TLSLE: bfd_reloc_code_real = 538;
pub const BFD_RELOC_PPC_TLSLD: bfd_reloc_code_real = 537;
pub const BFD_RELOC_PPC_TLSGD: bfd_reloc_code_real = 536;
pub const BFD_RELOC_PPC_TLS: bfd_reloc_code_real = 535;
pub const BFD_RELOC_PPC64_PCREL28: bfd_reloc_code_real = 534;
pub const BFD_RELOC_PPC64_D28: bfd_reloc_code_real = 533;
pub const BFD_RELOC_PPC64_REL16_HIGHESTA34: bfd_reloc_code_real = 532;
pub const BFD_RELOC_PPC64_REL16_HIGHEST34: bfd_reloc_code_real = 531;
pub const BFD_RELOC_PPC64_REL16_HIGHERA34: bfd_reloc_code_real = 530;
pub const BFD_RELOC_PPC64_REL16_HIGHER34: bfd_reloc_code_real = 529;
pub const BFD_RELOC_PPC64_ADDR16_HIGHESTA34: bfd_reloc_code_real = 528;
pub const BFD_RELOC_PPC64_ADDR16_HIGHEST34: bfd_reloc_code_real = 527;
pub const BFD_RELOC_PPC64_ADDR16_HIGHERA34: bfd_reloc_code_real = 526;
pub const BFD_RELOC_PPC64_ADDR16_HIGHER34: bfd_reloc_code_real = 525;
pub const BFD_RELOC_PPC64_PLT_PCREL34: bfd_reloc_code_real = 524;
pub const BFD_RELOC_PPC64_GOT_PCREL34: bfd_reloc_code_real = 523;
pub const BFD_RELOC_PPC64_PCREL34: bfd_reloc_code_real = 522;
pub const BFD_RELOC_PPC64_D34_HA30: bfd_reloc_code_real = 521;
pub const BFD_RELOC_PPC64_D34_HI30: bfd_reloc_code_real = 520;
pub const BFD_RELOC_PPC64_D34_LO: bfd_reloc_code_real = 519;
pub const BFD_RELOC_PPC64_D34: bfd_reloc_code_real = 518;
pub const BFD_RELOC_PPC64_REL24_NOTOC: bfd_reloc_code_real = 517;
pub const BFD_RELOC_PPC64_ENTRY: bfd_reloc_code_real = 516;
pub const BFD_RELOC_PPC64_ADDR64_LOCAL: bfd_reloc_code_real = 515;
pub const BFD_RELOC_PPC64_REL16_HIGHESTA: bfd_reloc_code_real = 514;
pub const BFD_RELOC_PPC64_REL16_HIGHEST: bfd_reloc_code_real = 513;
pub const BFD_RELOC_PPC64_REL16_HIGHERA: bfd_reloc_code_real = 512;
pub const BFD_RELOC_PPC64_REL16_HIGHER: bfd_reloc_code_real = 511;
pub const BFD_RELOC_PPC64_REL16_HIGHA: bfd_reloc_code_real = 510;
pub const BFD_RELOC_PPC64_REL16_HIGH: bfd_reloc_code_real = 509;
pub const BFD_RELOC_PPC64_ADDR16_HIGHA: bfd_reloc_code_real = 508;
pub const BFD_RELOC_PPC64_ADDR16_HIGH: bfd_reloc_code_real = 507;
pub const BFD_RELOC_PPC64_PLTGOT16_LO_DS: bfd_reloc_code_real = 506;
pub const BFD_RELOC_PPC64_PLTGOT16_DS: bfd_reloc_code_real = 505;
pub const BFD_RELOC_PPC64_TOC16_LO_DS: bfd_reloc_code_real = 504;
pub const BFD_RELOC_PPC64_TOC16_DS: bfd_reloc_code_real = 503;
pub const BFD_RELOC_PPC64_SECTOFF_LO_DS: bfd_reloc_code_real = 502;
pub const BFD_RELOC_PPC64_SECTOFF_DS: bfd_reloc_code_real = 501;
pub const BFD_RELOC_PPC64_PLT16_LO_DS: bfd_reloc_code_real = 500;
pub const BFD_RELOC_PPC64_GOT16_LO_DS: bfd_reloc_code_real = 499;
pub const BFD_RELOC_PPC64_GOT16_DS: bfd_reloc_code_real = 498;
pub const BFD_RELOC_PPC64_ADDR16_LO_DS: bfd_reloc_code_real = 497;
pub const BFD_RELOC_PPC64_ADDR16_DS: bfd_reloc_code_real = 496;
pub const BFD_RELOC_PPC64_PLTGOT16_HA: bfd_reloc_code_real = 495;
pub const BFD_RELOC_PPC64_PLTGOT16_HI: bfd_reloc_code_real = 494;
pub const BFD_RELOC_PPC64_PLTGOT16_LO: bfd_reloc_code_real = 493;
pub const BFD_RELOC_PPC64_PLTGOT16: bfd_reloc_code_real = 492;
pub const BFD_RELOC_PPC64_TOC: bfd_reloc_code_real = 491;
pub const BFD_RELOC_PPC64_TOC16_HA: bfd_reloc_code_real = 490;
pub const BFD_RELOC_PPC64_TOC16_HI: bfd_reloc_code_real = 489;
pub const BFD_RELOC_PPC64_TOC16_LO: bfd_reloc_code_real = 488;
pub const BFD_RELOC_PPC64_HIGHEST_S: bfd_reloc_code_real = 487;
pub const BFD_RELOC_PPC64_HIGHEST: bfd_reloc_code_real = 486;
pub const BFD_RELOC_PPC64_HIGHER_S: bfd_reloc_code_real = 485;
pub const BFD_RELOC_PPC64_HIGHER: bfd_reloc_code_real = 484;
pub const BFD_RELOC_PPC_NEG: bfd_reloc_code_real = 483;
pub const BFD_RELOC_PPC_REL16DX_HA: bfd_reloc_code_real = 482;
pub const BFD_RELOC_PPC_16DX_HA: bfd_reloc_code_real = 481;
pub const BFD_RELOC_PPC_VLE_SDAREL_HA16D: bfd_reloc_code_real = 480;
pub const BFD_RELOC_PPC_VLE_SDAREL_HA16A: bfd_reloc_code_real = 479;
pub const BFD_RELOC_PPC_VLE_SDAREL_HI16D: bfd_reloc_code_real = 478;
pub const BFD_RELOC_PPC_VLE_SDAREL_HI16A: bfd_reloc_code_real = 477;
pub const BFD_RELOC_PPC_VLE_SDAREL_LO16D: bfd_reloc_code_real = 476;
pub const BFD_RELOC_PPC_VLE_SDAREL_LO16A: bfd_reloc_code_real = 475;
pub const BFD_RELOC_PPC_VLE_SDA21_LO: bfd_reloc_code_real = 474;
pub const BFD_RELOC_PPC_VLE_SDA21: bfd_reloc_code_real = 473;
pub const BFD_RELOC_PPC_VLE_HA16D: bfd_reloc_code_real = 472;
pub const BFD_RELOC_PPC_VLE_HA16A: bfd_reloc_code_real = 471;
pub const BFD_RELOC_PPC_VLE_HI16D: bfd_reloc_code_real = 470;
pub const BFD_RELOC_PPC_VLE_HI16A: bfd_reloc_code_real = 469;
pub const BFD_RELOC_PPC_VLE_LO16D: bfd_reloc_code_real = 468;
pub const BFD_RELOC_PPC_VLE_LO16A: bfd_reloc_code_real = 467;
pub const BFD_RELOC_PPC_VLE_REL24: bfd_reloc_code_real = 466;
pub const BFD_RELOC_PPC_VLE_REL15: bfd_reloc_code_real = 465;
pub const BFD_RELOC_PPC_VLE_REL8: bfd_reloc_code_real = 464;
pub const BFD_RELOC_PPC_EMB_RELSDA: bfd_reloc_code_real = 463;
pub const BFD_RELOC_PPC_EMB_BIT_FLD: bfd_reloc_code_real = 462;
pub const BFD_RELOC_PPC_EMB_RELST_HA: bfd_reloc_code_real = 461;
pub const BFD_RELOC_PPC_EMB_RELST_HI: bfd_reloc_code_real = 460;
pub const BFD_RELOC_PPC_EMB_RELST_LO: bfd_reloc_code_real = 459;
pub const BFD_RELOC_PPC_EMB_RELSEC16: bfd_reloc_code_real = 458;
pub const BFD_RELOC_PPC_EMB_MRKREF: bfd_reloc_code_real = 457;
pub const BFD_RELOC_PPC_EMB_SDA21: bfd_reloc_code_real = 456;
pub const BFD_RELOC_PPC_EMB_SDA2REL: bfd_reloc_code_real = 455;
pub const BFD_RELOC_PPC_EMB_SDA2I16: bfd_reloc_code_real = 454;
pub const BFD_RELOC_PPC_EMB_SDAI16: bfd_reloc_code_real = 453;
pub const BFD_RELOC_PPC_EMB_NADDR16_HA: bfd_reloc_code_real = 452;
pub const BFD_RELOC_PPC_EMB_NADDR16_HI: bfd_reloc_code_real = 451;
pub const BFD_RELOC_PPC_EMB_NADDR16_LO: bfd_reloc_code_real = 450;
pub const BFD_RELOC_PPC_EMB_NADDR16: bfd_reloc_code_real = 449;
pub const BFD_RELOC_PPC_EMB_NADDR32: bfd_reloc_code_real = 448;
pub const BFD_RELOC_PPC_LOCAL24PC: bfd_reloc_code_real = 447;
pub const BFD_RELOC_PPC_RELATIVE: bfd_reloc_code_real = 446;
pub const BFD_RELOC_PPC_JMP_SLOT: bfd_reloc_code_real = 445;
pub const BFD_RELOC_PPC_GLOB_DAT: bfd_reloc_code_real = 444;
pub const BFD_RELOC_PPC_COPY: bfd_reloc_code_real = 443;
pub const BFD_RELOC_PPC_BA16_BRNTAKEN: bfd_reloc_code_real = 442;
pub const BFD_RELOC_PPC_BA16_BRTAKEN: bfd_reloc_code_real = 441;
pub const BFD_RELOC_PPC_BA16: bfd_reloc_code_real = 440;
pub const BFD_RELOC_PPC_B16_BRNTAKEN: bfd_reloc_code_real = 439;
pub const BFD_RELOC_PPC_B16_BRTAKEN: bfd_reloc_code_real = 438;
pub const BFD_RELOC_PPC_B16: bfd_reloc_code_real = 437;
pub const BFD_RELOC_PPC_TOC16_HI: bfd_reloc_code_real = 436;
pub const BFD_RELOC_PPC_TOC16_LO: bfd_reloc_code_real = 435;
pub const BFD_RELOC_PPC_TOC16: bfd_reloc_code_real = 434;
pub const BFD_RELOC_PPC_BA26: bfd_reloc_code_real = 433;
pub const BFD_RELOC_PPC_B26: bfd_reloc_code_real = 432;
pub const BFD_RELOC_PJ_CODE_REL32: bfd_reloc_code_real = 431;
pub const BFD_RELOC_PJ_CODE_REL16: bfd_reloc_code_real = 430;
pub const BFD_RELOC_PJ_CODE_DIR32: bfd_reloc_code_real = 429;
pub const BFD_RELOC_PJ_CODE_DIR16: bfd_reloc_code_real = 428;
pub const BFD_RELOC_PJ_CODE_LO16: bfd_reloc_code_real = 427;
pub const BFD_RELOC_PJ_CODE_HI16: bfd_reloc_code_real = 426;
pub const BFD_RELOC_PDP11_DISP_6_PCREL: bfd_reloc_code_real = 425;
pub const BFD_RELOC_PDP11_DISP_8_PCREL: bfd_reloc_code_real = 424;
pub const BFD_RELOC_NS32K_DISP_32_PCREL: bfd_reloc_code_real = 423;
pub const BFD_RELOC_NS32K_DISP_16_PCREL: bfd_reloc_code_real = 422;
pub const BFD_RELOC_NS32K_DISP_8_PCREL: bfd_reloc_code_real = 421;
pub const BFD_RELOC_NS32K_DISP_32: bfd_reloc_code_real = 420;
pub const BFD_RELOC_NS32K_DISP_16: bfd_reloc_code_real = 419;
pub const BFD_RELOC_NS32K_DISP_8: bfd_reloc_code_real = 418;
pub const BFD_RELOC_NS32K_IMM_32_PCREL: bfd_reloc_code_real = 417;
pub const BFD_RELOC_NS32K_IMM_16_PCREL: bfd_reloc_code_real = 416;
pub const BFD_RELOC_NS32K_IMM_8_PCREL: bfd_reloc_code_real = 415;
pub const BFD_RELOC_NS32K_IMM_32: bfd_reloc_code_real = 414;
pub const BFD_RELOC_NS32K_IMM_16: bfd_reloc_code_real = 413;
pub const BFD_RELOC_NS32K_IMM_8: bfd_reloc_code_real = 412;
pub const BFD_RELOC_X86_64_REX_GOTPCRELX: bfd_reloc_code_real = 411;
pub const BFD_RELOC_X86_64_GOTPCRELX: bfd_reloc_code_real = 410;
pub const BFD_RELOC_X86_64_PLT32_BND: bfd_reloc_code_real = 409;
pub const BFD_RELOC_X86_64_PC32_BND: bfd_reloc_code_real = 408;
pub const BFD_RELOC_X86_64_IRELATIVE: bfd_reloc_code_real = 407;
pub const BFD_RELOC_X86_64_TLSDESC: bfd_reloc_code_real = 406;
pub const BFD_RELOC_X86_64_TLSDESC_CALL: bfd_reloc_code_real = 405;
pub const BFD_RELOC_X86_64_GOTPC32_TLSDESC: bfd_reloc_code_real = 404;
pub const BFD_RELOC_X86_64_PLTOFF64: bfd_reloc_code_real = 403;
pub const BFD_RELOC_X86_64_GOTPLT64: bfd_reloc_code_real = 402;
pub const BFD_RELOC_X86_64_GOTPC64: bfd_reloc_code_real = 401;
pub const BFD_RELOC_X86_64_GOTPCREL64: bfd_reloc_code_real = 400;
pub const BFD_RELOC_X86_64_GOT64: bfd_reloc_code_real = 399;
pub const BFD_RELOC_X86_64_GOTPC32: bfd_reloc_code_real = 398;
pub const BFD_RELOC_X86_64_GOTOFF64: bfd_reloc_code_real = 397;
pub const BFD_RELOC_X86_64_TPOFF32: bfd_reloc_code_real = 396;
pub const BFD_RELOC_X86_64_GOTTPOFF: bfd_reloc_code_real = 395;
pub const BFD_RELOC_X86_64_DTPOFF32: bfd_reloc_code_real = 394;
pub const BFD_RELOC_X86_64_TLSLD: bfd_reloc_code_real = 393;
pub const BFD_RELOC_X86_64_TLSGD: bfd_reloc_code_real = 392;
pub const BFD_RELOC_X86_64_TPOFF64: bfd_reloc_code_real = 391;
pub const BFD_RELOC_X86_64_DTPOFF64: bfd_reloc_code_real = 390;
pub const BFD_RELOC_X86_64_DTPMOD64: bfd_reloc_code_real = 389;
pub const BFD_RELOC_X86_64_32S: bfd_reloc_code_real = 388;
pub const BFD_RELOC_X86_64_GOTPCREL: bfd_reloc_code_real = 387;
pub const BFD_RELOC_X86_64_RELATIVE: bfd_reloc_code_real = 386;
pub const BFD_RELOC_X86_64_JUMP_SLOT: bfd_reloc_code_real = 385;
pub const BFD_RELOC_X86_64_GLOB_DAT: bfd_reloc_code_real = 384;
pub const BFD_RELOC_X86_64_COPY: bfd_reloc_code_real = 383;
pub const BFD_RELOC_X86_64_PLT32: bfd_reloc_code_real = 382;
pub const BFD_RELOC_X86_64_GOT32: bfd_reloc_code_real = 381;
pub const BFD_RELOC_386_GOT32X: bfd_reloc_code_real = 380;
pub const BFD_RELOC_386_IRELATIVE: bfd_reloc_code_real = 379;
pub const BFD_RELOC_386_TLS_DESC: bfd_reloc_code_real = 378;
pub const BFD_RELOC_386_TLS_DESC_CALL: bfd_reloc_code_real = 377;
pub const BFD_RELOC_386_TLS_GOTDESC: bfd_reloc_code_real = 376;
pub const BFD_RELOC_386_TLS_TPOFF32: bfd_reloc_code_real = 375;
pub const BFD_RELOC_386_TLS_DTPOFF32: bfd_reloc_code_real = 374;
pub const BFD_RELOC_386_TLS_DTPMOD32: bfd_reloc_code_real = 373;
pub const BFD_RELOC_386_TLS_LE_32: bfd_reloc_code_real = 372;
pub const BFD_RELOC_386_TLS_IE_32: bfd_reloc_code_real = 371;
pub const BFD_RELOC_386_TLS_LDO_32: bfd_reloc_code_real = 370;
pub const BFD_RELOC_386_TLS_LDM: bfd_reloc_code_real = 369;
pub const BFD_RELOC_386_TLS_GD: bfd_reloc_code_real = 368;
pub const BFD_RELOC_386_TLS_LE: bfd_reloc_code_real = 367;
pub const BFD_RELOC_386_TLS_GOTIE: bfd_reloc_code_real = 366;
pub const BFD_RELOC_386_TLS_IE: bfd_reloc_code_real = 365;
pub const BFD_RELOC_386_TLS_TPOFF: bfd_reloc_code_real = 364;
pub const BFD_RELOC_386_GOTPC: bfd_reloc_code_real = 363;
pub const BFD_RELOC_386_GOTOFF: bfd_reloc_code_real = 362;
pub const BFD_RELOC_386_RELATIVE: bfd_reloc_code_real = 361;
pub const BFD_RELOC_386_JUMP_SLOT: bfd_reloc_code_real = 360;
pub const BFD_RELOC_386_GLOB_DAT: bfd_reloc_code_real = 359;
pub const BFD_RELOC_386_COPY: bfd_reloc_code_real = 358;
pub const BFD_RELOC_386_PLT32: bfd_reloc_code_real = 357;
pub const BFD_RELOC_386_GOT32: bfd_reloc_code_real = 356;
pub const BFD_RELOC_MN10300_16_PCREL: bfd_reloc_code_real = 355;
pub const BFD_RELOC_MN10300_32_PCREL: bfd_reloc_code_real = 354;
pub const BFD_RELOC_MN10300_TLS_TPOFF: bfd_reloc_code_real = 353;
pub const BFD_RELOC_MN10300_TLS_DTPOFF: bfd_reloc_code_real = 352;
pub const BFD_RELOC_MN10300_TLS_DTPMOD: bfd_reloc_code_real = 351;
pub const BFD_RELOC_MN10300_TLS_LE: bfd_reloc_code_real = 350;
pub const BFD_RELOC_MN10300_TLS_IE: bfd_reloc_code_real = 349;
pub const BFD_RELOC_MN10300_TLS_GOTIE: bfd_reloc_code_real = 348;
pub const BFD_RELOC_MN10300_TLS_LDO: bfd_reloc_code_real = 347;
pub const BFD_RELOC_MN10300_TLS_LD: bfd_reloc_code_real = 346;
pub const BFD_RELOC_MN10300_TLS_GD: bfd_reloc_code_real = 345;
pub const BFD_RELOC_MN10300_ALIGN: bfd_reloc_code_real = 344;
pub const BFD_RELOC_MN10300_SYM_DIFF: bfd_reloc_code_real = 343;
pub const BFD_RELOC_MN10300_RELATIVE: bfd_reloc_code_real = 342;
pub const BFD_RELOC_MN10300_JMP_SLOT: bfd_reloc_code_real = 341;
pub const BFD_RELOC_MN10300_GLOB_DAT: bfd_reloc_code_real = 340;
pub const BFD_RELOC_MN10300_COPY: bfd_reloc_code_real = 339;
pub const BFD_RELOC_MN10300_GOT16: bfd_reloc_code_real = 338;
pub const BFD_RELOC_MN10300_GOT24: bfd_reloc_code_real = 337;
pub const BFD_RELOC_MN10300_GOT32: bfd_reloc_code_real = 336;
pub const BFD_RELOC_MN10300_GOTOFF24: bfd_reloc_code_real = 335;
pub const BFD_RELOC_FRV_TLSMOFF: bfd_reloc_code_real = 334;
pub const BFD_RELOC_FRV_TLSOFF_RELAX: bfd_reloc_code_real = 333;
pub const BFD_RELOC_FRV_GETTLSOFF_RELAX: bfd_reloc_code_real = 332;
pub const BFD_RELOC_FRV_TLSDESC_RELAX: bfd_reloc_code_real = 331;
pub const BFD_RELOC_FRV_TLSOFF: bfd_reloc_code_real = 330;
pub const BFD_RELOC_FRV_GOTTLSOFFLO: bfd_reloc_code_real = 329;
pub const BFD_RELOC_FRV_GOTTLSOFFHI: bfd_reloc_code_real = 328;
pub const BFD_RELOC_FRV_GOTTLSOFF12: bfd_reloc_code_real = 327;
pub const BFD_RELOC_FRV_TLSMOFFLO: bfd_reloc_code_real = 326;
pub const BFD_RELOC_FRV_TLSMOFFHI: bfd_reloc_code_real = 325;
pub const BFD_RELOC_FRV_TLSMOFF12: bfd_reloc_code_real = 324;
pub const BFD_RELOC_FRV_GOTTLSDESCLO: bfd_reloc_code_real = 323;
pub const BFD_RELOC_FRV_GOTTLSDESCHI: bfd_reloc_code_real = 322;
pub const BFD_RELOC_FRV_GOTTLSDESC12: bfd_reloc_code_real = 321;
pub const BFD_RELOC_FRV_TLSDESC_VALUE: bfd_reloc_code_real = 320;
pub const BFD_RELOC_FRV_GETTLSOFF: bfd_reloc_code_real = 319;
pub const BFD_RELOC_FRV_GOTOFFLO: bfd_reloc_code_real = 318;
pub const BFD_RELOC_FRV_GOTOFFHI: bfd_reloc_code_real = 317;
pub const BFD_RELOC_FRV_GOTOFF12: bfd_reloc_code_real = 316;
pub const BFD_RELOC_FRV_FUNCDESC_GOTOFFLO: bfd_reloc_code_real = 315;
pub const BFD_RELOC_FRV_FUNCDESC_GOTOFFHI: bfd_reloc_code_real = 314;
pub const BFD_RELOC_FRV_FUNCDESC_GOTOFF12: bfd_reloc_code_real = 313;
pub const BFD_RELOC_FRV_FUNCDESC_VALUE: bfd_reloc_code_real = 312;
pub const BFD_RELOC_FRV_FUNCDESC_GOTLO: bfd_reloc_code_real = 311;
pub const BFD_RELOC_FRV_FUNCDESC_GOTHI: bfd_reloc_code_real = 310;
pub const BFD_RELOC_FRV_FUNCDESC_GOT12: bfd_reloc_code_real = 309;
pub const BFD_RELOC_FRV_FUNCDESC: bfd_reloc_code_real = 308;
pub const BFD_RELOC_FRV_GOTLO: bfd_reloc_code_real = 307;
pub const BFD_RELOC_FRV_GOTHI: bfd_reloc_code_real = 306;
pub const BFD_RELOC_FRV_GOT12: bfd_reloc_code_real = 305;
pub const BFD_RELOC_FRV_GPRELLO: bfd_reloc_code_real = 304;
pub const BFD_RELOC_FRV_GPRELHI: bfd_reloc_code_real = 303;
pub const BFD_RELOC_FRV_GPREL32: bfd_reloc_code_real = 302;
pub const BFD_RELOC_FRV_GPRELU12: bfd_reloc_code_real = 301;
pub const BFD_RELOC_FRV_GPREL12: bfd_reloc_code_real = 300;
pub const BFD_RELOC_FRV_HI16: bfd_reloc_code_real = 299;
pub const BFD_RELOC_FRV_LO16: bfd_reloc_code_real = 298;
pub const BFD_RELOC_FRV_LABEL24: bfd_reloc_code_real = 297;
pub const BFD_RELOC_FRV_LABEL16: bfd_reloc_code_real = 296;
pub const BFD_RELOC_FT32_DIFF32: bfd_reloc_code_real = 295;
pub const BFD_RELOC_FT32_15: bfd_reloc_code_real = 294;
pub const BFD_RELOC_FT32_SC1: bfd_reloc_code_real = 293;
pub const BFD_RELOC_FT32_SC0: bfd_reloc_code_real = 292;
pub const BFD_RELOC_FT32_RELAX: bfd_reloc_code_real = 291;
pub const BFD_RELOC_FT32_18: bfd_reloc_code_real = 290;
pub const BFD_RELOC_FT32_17: bfd_reloc_code_real = 289;
pub const BFD_RELOC_FT32_20: bfd_reloc_code_real = 288;
pub const BFD_RELOC_FT32_10: bfd_reloc_code_real = 287;
pub const BFD_RELOC_MOXIE_10_PCREL: bfd_reloc_code_real = 286;
pub const BFD_RELOC_MIPS_JUMP_SLOT: bfd_reloc_code_real = 285;
pub const BFD_RELOC_MIPS_COPY: bfd_reloc_code_real = 284;
pub const BFD_RELOC_MIPS_EH: bfd_reloc_code_real = 283;
pub const BFD_RELOC_MICROMIPS_TLS_TPREL_LO16: bfd_reloc_code_real = 282;
pub const BFD_RELOC_MIPS_TLS_TPREL_LO16: bfd_reloc_code_real = 281;
pub const BFD_RELOC_MICROMIPS_TLS_TPREL_HI16: bfd_reloc_code_real = 280;
pub const BFD_RELOC_MIPS_TLS_TPREL_HI16: bfd_reloc_code_real = 279;
pub const BFD_RELOC_MIPS_TLS_TPREL64: bfd_reloc_code_real = 278;
pub const BFD_RELOC_MIPS_TLS_TPREL32: bfd_reloc_code_real = 277;
pub const BFD_RELOC_MICROMIPS_TLS_GOTTPREL: bfd_reloc_code_real = 276;
pub const BFD_RELOC_MIPS_TLS_GOTTPREL: bfd_reloc_code_real = 275;
pub const BFD_RELOC_MICROMIPS_TLS_DTPREL_LO16: bfd_reloc_code_real = 274;
pub const BFD_RELOC_MIPS_TLS_DTPREL_LO16: bfd_reloc_code_real = 273;
pub const BFD_RELOC_MICROMIPS_TLS_DTPREL_HI16: bfd_reloc_code_real = 272;
pub const BFD_RELOC_MIPS_TLS_DTPREL_HI16: bfd_reloc_code_real = 271;
pub const BFD_RELOC_MICROMIPS_TLS_LDM: bfd_reloc_code_real = 270;
pub const BFD_RELOC_MIPS_TLS_LDM: bfd_reloc_code_real = 269;
pub const BFD_RELOC_MICROMIPS_TLS_GD: bfd_reloc_code_real = 268;
pub const BFD_RELOC_MIPS_TLS_GD: bfd_reloc_code_real = 267;
pub const BFD_RELOC_MIPS_TLS_DTPREL64: bfd_reloc_code_real = 266;
pub const BFD_RELOC_MIPS_TLS_DTPMOD64: bfd_reloc_code_real = 265;
pub const BFD_RELOC_MIPS_TLS_DTPREL32: bfd_reloc_code_real = 264;
pub const BFD_RELOC_MIPS_TLS_DTPMOD32: bfd_reloc_code_real = 263;
pub const BFD_RELOC_MICROMIPS_JALR: bfd_reloc_code_real = 262;
pub const BFD_RELOC_MIPS_JALR: bfd_reloc_code_real = 261;
pub const BFD_RELOC_MIPS_RELGOT: bfd_reloc_code_real = 260;
pub const BFD_RELOC_MIPS_REL16: bfd_reloc_code_real = 259;
pub const BFD_RELOC_MICROMIPS_SCN_DISP: bfd_reloc_code_real = 258;
pub const BFD_RELOC_MIPS_SCN_DISP: bfd_reloc_code_real = 257;
pub const BFD_RELOC_MICROMIPS_HIGHER: bfd_reloc_code_real = 256;
pub const BFD_RELOC_MIPS_HIGHER: bfd_reloc_code_real = 255;
pub const BFD_RELOC_MICROMIPS_HIGHEST: bfd_reloc_code_real = 254;
pub const BFD_RELOC_MIPS_HIGHEST: bfd_reloc_code_real = 253;
pub const BFD_RELOC_MIPS_DELETE: bfd_reloc_code_real = 252;
pub const BFD_RELOC_MIPS_INSERT_B: bfd_reloc_code_real = 251;
pub const BFD_RELOC_MIPS_INSERT_A: bfd_reloc_code_real = 250;
pub const BFD_RELOC_MIPS_SHIFT6: bfd_reloc_code_real = 249;
pub const BFD_RELOC_MIPS_SHIFT5: bfd_reloc_code_real = 248;
pub const BFD_RELOC_MICROMIPS_GOT_DISP: bfd_reloc_code_real = 247;
pub const BFD_RELOC_MIPS_GOT_DISP: bfd_reloc_code_real = 246;
pub const BFD_RELOC_MICROMIPS_GOT_OFST: bfd_reloc_code_real = 245;
pub const BFD_RELOC_MIPS_GOT_OFST: bfd_reloc_code_real = 244;
pub const BFD_RELOC_MICROMIPS_GOT_PAGE: bfd_reloc_code_real = 243;
pub const BFD_RELOC_MIPS_GOT_PAGE: bfd_reloc_code_real = 242;
pub const BFD_RELOC_MICROMIPS_SUB: bfd_reloc_code_real = 241;
pub const BFD_RELOC_MIPS_SUB: bfd_reloc_code_real = 240;
pub const BFD_RELOC_MICROMIPS_CALL_LO16: bfd_reloc_code_real = 239;
pub const BFD_RELOC_MIPS_CALL_LO16: bfd_reloc_code_real = 238;
pub const BFD_RELOC_MICROMIPS_CALL_HI16: bfd_reloc_code_real = 237;
pub const BFD_RELOC_MIPS_CALL_HI16: bfd_reloc_code_real = 236;
pub const BFD_RELOC_MICROMIPS_GOT_LO16: bfd_reloc_code_real = 235;
pub const BFD_RELOC_MIPS_GOT_LO16: bfd_reloc_code_real = 234;
pub const BFD_RELOC_MICROMIPS_GOT_HI16: bfd_reloc_code_real = 233;
pub const BFD_RELOC_MIPS_GOT_HI16: bfd_reloc_code_real = 232;
pub const BFD_RELOC_MICROMIPS_CALL16: bfd_reloc_code_real = 231;
pub const BFD_RELOC_MIPS_CALL16: bfd_reloc_code_real = 230;
pub const BFD_RELOC_MICROMIPS_GOT16: bfd_reloc_code_real = 229;
pub const BFD_RELOC_MIPS_GOT16: bfd_reloc_code_real = 228;
pub const BFD_RELOC_MICROMIPS_LO16: bfd_reloc_code_real = 227;
pub const BFD_RELOC_MICROMIPS_HI16_S: bfd_reloc_code_real = 226;
pub const BFD_RELOC_MICROMIPS_HI16: bfd_reloc_code_real = 225;
pub const BFD_RELOC_MICROMIPS_GPREL16: bfd_reloc_code_real = 224;
pub const BFD_RELOC_MIPS_19_PCREL_S2: bfd_reloc_code_real = 223;
pub const BFD_RELOC_MIPS_18_PCREL_S3: bfd_reloc_code_real = 222;
pub const BFD_RELOC_MIPS_26_PCREL_S2: bfd_reloc_code_real = 221;
pub const BFD_RELOC_MIPS_21_PCREL_S2: bfd_reloc_code_real = 220;
pub const BFD_RELOC_MIPS16_16_PCREL_S1: bfd_reloc_code_real = 219;
pub const BFD_RELOC_MICROMIPS_16_PCREL_S1: bfd_reloc_code_real = 218;
pub const BFD_RELOC_MICROMIPS_10_PCREL_S1: bfd_reloc_code_real = 217;
pub const BFD_RELOC_MICROMIPS_7_PCREL_S1: bfd_reloc_code_real = 216;
pub const BFD_RELOC_MICROMIPS_LITERAL: bfd_reloc_code_real = 215;
pub const BFD_RELOC_MIPS_LITERAL: bfd_reloc_code_real = 214;
pub const BFD_RELOC_MIPS16_TLS_TPREL_LO16: bfd_reloc_code_real = 213;
pub const BFD_RELOC_MIPS16_TLS_TPREL_HI16: bfd_reloc_code_real = 212;
pub const BFD_RELOC_MIPS16_TLS_GOTTPREL: bfd_reloc_code_real = 211;
pub const BFD_RELOC_MIPS16_TLS_DTPREL_LO16: bfd_reloc_code_real = 210;
pub const BFD_RELOC_MIPS16_TLS_DTPREL_HI16: bfd_reloc_code_real = 209;
pub const BFD_RELOC_MIPS16_TLS_LDM: bfd_reloc_code_real = 208;
pub const BFD_RELOC_MIPS16_TLS_GD: bfd_reloc_code_real = 207;
pub const BFD_RELOC_MIPS16_LO16: bfd_reloc_code_real = 206;
pub const BFD_RELOC_MIPS16_HI16_S: bfd_reloc_code_real = 205;
pub const BFD_RELOC_MIPS16_HI16: bfd_reloc_code_real = 204;
pub const BFD_RELOC_MIPS16_CALL16: bfd_reloc_code_real = 203;
pub const BFD_RELOC_MIPS16_GOT16: bfd_reloc_code_real = 202;
pub const BFD_RELOC_LO16_PCREL: bfd_reloc_code_real = 201;
pub const BFD_RELOC_HI16_S_PCREL: bfd_reloc_code_real = 200;
pub const BFD_RELOC_HI16_PCREL: bfd_reloc_code_real = 199;
pub const BFD_RELOC_LO16: bfd_reloc_code_real = 198;
pub const BFD_RELOC_HI16_S: bfd_reloc_code_real = 197;
pub const BFD_RELOC_HI16: bfd_reloc_code_real = 196;
pub const BFD_RELOC_MIPS16_GPREL: bfd_reloc_code_real = 195;
pub const BFD_RELOC_MIPS16_JMP: bfd_reloc_code_real = 194;
pub const BFD_RELOC_MICROMIPS_JMP: bfd_reloc_code_real = 193;
pub const BFD_RELOC_MIPS_JMP: bfd_reloc_code_real = 192;
pub const BFD_RELOC_ALPHA_TPREL16: bfd_reloc_code_real = 191;
pub const BFD_RELOC_ALPHA_TPREL_LO16: bfd_reloc_code_real = 190;
pub const BFD_RELOC_ALPHA_TPREL_HI16: bfd_reloc_code_real = 189;
pub const BFD_RELOC_ALPHA_TPREL64: bfd_reloc_code_real = 188;
pub const BFD_RELOC_ALPHA_GOTTPREL16: bfd_reloc_code_real = 187;
pub const BFD_RELOC_ALPHA_DTPREL16: bfd_reloc_code_real = 186;
pub const BFD_RELOC_ALPHA_DTPREL_LO16: bfd_reloc_code_real = 185;
pub const BFD_RELOC_ALPHA_DTPREL_HI16: bfd_reloc_code_real = 184;
pub const BFD_RELOC_ALPHA_DTPREL64: bfd_reloc_code_real = 183;
pub const BFD_RELOC_ALPHA_GOTDTPREL16: bfd_reloc_code_real = 182;
pub const BFD_RELOC_ALPHA_DTPMOD64: bfd_reloc_code_real = 181;
pub const BFD_RELOC_ALPHA_TLSLDM: bfd_reloc_code_real = 180;
pub const BFD_RELOC_ALPHA_TLSGD: bfd_reloc_code_real = 179;
pub const BFD_RELOC_ALPHA_BOH: bfd_reloc_code_real = 178;
pub const BFD_RELOC_ALPHA_LDA: bfd_reloc_code_real = 177;
pub const BFD_RELOC_ALPHA_BSR: bfd_reloc_code_real = 176;
pub const BFD_RELOC_ALPHA_NOP: bfd_reloc_code_real = 175;
pub const BFD_RELOC_ALPHA_BRSGP: bfd_reloc_code_real = 174;
pub const BFD_RELOC_ALPHA_GPREL_LO16: bfd_reloc_code_real = 173;
pub const BFD_RELOC_ALPHA_GPREL_HI16: bfd_reloc_code_real = 172;
pub const BFD_RELOC_ALPHA_CODEADDR: bfd_reloc_code_real = 171;
pub const BFD_RELOC_ALPHA_LINKAGE: bfd_reloc_code_real = 170;
pub const BFD_RELOC_ALPHA_HINT: bfd_reloc_code_real = 169;
pub const BFD_RELOC_ALPHA_LITUSE: bfd_reloc_code_real = 168;
pub const BFD_RELOC_ALPHA_ELF_LITERAL: bfd_reloc_code_real = 167;
pub const BFD_RELOC_ALPHA_LITERAL: bfd_reloc_code_real = 166;
pub const BFD_RELOC_ALPHA_GPDISP: bfd_reloc_code_real = 165;
pub const BFD_RELOC_ALPHA_GPDISP_LO16: bfd_reloc_code_real = 164;
pub const BFD_RELOC_ALPHA_GPDISP_HI16: bfd_reloc_code_real = 163;
pub const BFD_RELOC_SPU_ADD_PIC: bfd_reloc_code_real = 162;
pub const BFD_RELOC_SPU_PPU64: bfd_reloc_code_real = 161;
pub const BFD_RELOC_SPU_PPU32: bfd_reloc_code_real = 160;
pub const BFD_RELOC_SPU_HI16: bfd_reloc_code_real = 159;
pub const BFD_RELOC_SPU_LO16: bfd_reloc_code_real = 158;
pub const BFD_RELOC_SPU_PCREL16: bfd_reloc_code_real = 157;
pub const BFD_RELOC_SPU_PCREL9b: bfd_reloc_code_real = 156;
pub const BFD_RELOC_SPU_PCREL9a: bfd_reloc_code_real = 155;
pub const BFD_RELOC_SPU_IMM18: bfd_reloc_code_real = 154;
pub const BFD_RELOC_SPU_IMM16W: bfd_reloc_code_real = 153;
pub const BFD_RELOC_SPU_IMM16: bfd_reloc_code_real = 152;
pub const BFD_RELOC_SPU_IMM10W: bfd_reloc_code_real = 151;
pub const BFD_RELOC_SPU_IMM10: bfd_reloc_code_real = 150;
pub const BFD_RELOC_SPU_IMM8: bfd_reloc_code_real = 149;
pub const BFD_RELOC_SPU_IMM7: bfd_reloc_code_real = 148;
pub const BFD_RELOC_SPARC_TLS_TPOFF64: bfd_reloc_code_real = 147;
pub const BFD_RELOC_SPARC_TLS_TPOFF32: bfd_reloc_code_real = 146;
pub const BFD_RELOC_SPARC_TLS_DTPOFF64: bfd_reloc_code_real = 145;
pub const BFD_RELOC_SPARC_TLS_DTPOFF32: bfd_reloc_code_real = 144;
pub const BFD_RELOC_SPARC_TLS_DTPMOD64: bfd_reloc_code_real = 143;
pub const BFD_RELOC_SPARC_TLS_DTPMOD32: bfd_reloc_code_real = 142;
pub const BFD_RELOC_SPARC_TLS_LE_LOX10: bfd_reloc_code_real = 141;
pub const BFD_RELOC_SPARC_TLS_LE_HIX22: bfd_reloc_code_real = 140;
pub const BFD_RELOC_SPARC_TLS_IE_ADD: bfd_reloc_code_real = 139;
pub const BFD_RELOC_SPARC_TLS_IE_LDX: bfd_reloc_code_real = 138;
pub const BFD_RELOC_SPARC_TLS_IE_LD: bfd_reloc_code_real = 137;
pub const BFD_RELOC_SPARC_TLS_IE_LO10: bfd_reloc_code_real = 136;
pub const BFD_RELOC_SPARC_TLS_IE_HI22: bfd_reloc_code_real = 135;
pub const BFD_RELOC_SPARC_TLS_LDO_ADD: bfd_reloc_code_real = 134;
pub const BFD_RELOC_SPARC_TLS_LDO_LOX10: bfd_reloc_code_real = 133;
pub const BFD_RELOC_SPARC_TLS_LDO_HIX22: bfd_reloc_code_real = 132;
pub const BFD_RELOC_SPARC_TLS_LDM_CALL: bfd_reloc_code_real = 131;
pub const BFD_RELOC_SPARC_TLS_LDM_ADD: bfd_reloc_code_real = 130;
pub const BFD_RELOC_SPARC_TLS_LDM_LO10: bfd_reloc_code_real = 129;
pub const BFD_RELOC_SPARC_TLS_LDM_HI22: bfd_reloc_code_real = 128;
pub const BFD_RELOC_SPARC_TLS_GD_CALL: bfd_reloc_code_real = 127;
pub const BFD_RELOC_SPARC_TLS_GD_ADD: bfd_reloc_code_real = 126;
pub const BFD_RELOC_SPARC_TLS_GD_LO10: bfd_reloc_code_real = 125;
pub const BFD_RELOC_SPARC_TLS_GD_HI22: bfd_reloc_code_real = 124;
pub const BFD_RELOC_SPARC_REV32: bfd_reloc_code_real = 123;
pub const BFD_RELOC_SPARC_WDISP10: bfd_reloc_code_real = 122;
pub const BFD_RELOC_SPARC_SIZE64: bfd_reloc_code_real = 121;
pub const BFD_RELOC_SPARC_SIZE32: bfd_reloc_code_real = 120;
pub const BFD_RELOC_SPARC_H34: bfd_reloc_code_real = 119;
pub const BFD_RELOC_SPARC_REGISTER: bfd_reloc_code_real = 118;
pub const BFD_RELOC_SPARC_L44: bfd_reloc_code_real = 117;
pub const BFD_RELOC_SPARC_M44: bfd_reloc_code_real = 116;
pub const BFD_RELOC_SPARC_H44: bfd_reloc_code_real = 115;
pub const BFD_RELOC_SPARC_LOX10: bfd_reloc_code_real = 114;
pub const BFD_RELOC_SPARC_HIX22: bfd_reloc_code_real = 113;
pub const BFD_RELOC_SPARC_PLT64: bfd_reloc_code_real = 112;
pub const BFD_RELOC_SPARC_PLT32: bfd_reloc_code_real = 111;
pub const BFD_RELOC_SPARC_5: bfd_reloc_code_real = 110;
pub const BFD_RELOC_SPARC_6: bfd_reloc_code_real = 109;
pub const BFD_RELOC_SPARC_7: bfd_reloc_code_real = 108;
pub const BFD_RELOC_SPARC_WDISP19: bfd_reloc_code_real = 107;
pub const BFD_RELOC_SPARC_WDISP16: bfd_reloc_code_real = 106;
pub const BFD_RELOC_SPARC_PC_LM22: bfd_reloc_code_real = 105;
pub const BFD_RELOC_SPARC_PC_HM10: bfd_reloc_code_real = 104;
pub const BFD_RELOC_SPARC_PC_HH22: bfd_reloc_code_real = 103;
pub const BFD_RELOC_SPARC_LM22: bfd_reloc_code_real = 102;
pub const BFD_RELOC_SPARC_HM10: bfd_reloc_code_real = 101;
pub const BFD_RELOC_SPARC_HH22: bfd_reloc_code_real = 100;
pub const BFD_RELOC_SPARC_OLO10: bfd_reloc_code_real = 99;
pub const BFD_RELOC_SPARC_11: bfd_reloc_code_real = 98;
pub const BFD_RELOC_SPARC_10: bfd_reloc_code_real = 97;
pub const BFD_RELOC_SPARC_BASE22: bfd_reloc_code_real = 96;
pub const BFD_RELOC_SPARC_BASE13: bfd_reloc_code_real = 95;
pub const BFD_RELOC_SPARC_IRELATIVE: bfd_reloc_code_real = 94;
pub const BFD_RELOC_SPARC_JMP_IREL: bfd_reloc_code_real = 93;
pub const BFD_RELOC_SPARC_GOTDATA_OP: bfd_reloc_code_real = 92;
pub const BFD_RELOC_SPARC_GOTDATA_OP_LOX10: bfd_reloc_code_real = 91;
pub const BFD_RELOC_SPARC_GOTDATA_OP_HIX22: bfd_reloc_code_real = 90;
pub const BFD_RELOC_SPARC_GOTDATA_LOX10: bfd_reloc_code_real = 89;
pub const BFD_RELOC_SPARC_GOTDATA_HIX22: bfd_reloc_code_real = 88;
pub const BFD_RELOC_SPARC_UA64: bfd_reloc_code_real = 87;
pub const BFD_RELOC_SPARC_UA32: bfd_reloc_code_real = 86;
pub const BFD_RELOC_SPARC_UA16: bfd_reloc_code_real = 85;
pub const BFD_RELOC_SPARC_RELATIVE: bfd_reloc_code_real = 84;
pub const BFD_RELOC_SPARC_JMP_SLOT: bfd_reloc_code_real = 83;
pub const BFD_RELOC_SPARC_GLOB_DAT: bfd_reloc_code_real = 82;
pub const BFD_RELOC_SPARC_COPY: bfd_reloc_code_real = 81;
pub const BFD_RELOC_SPARC_WPLT30: bfd_reloc_code_real = 80;
pub const BFD_RELOC_SPARC_PC22: bfd_reloc_code_real = 79;
pub const BFD_RELOC_SPARC_PC10: bfd_reloc_code_real = 78;
pub const BFD_RELOC_SPARC_GOT22: bfd_reloc_code_real = 77;
pub const BFD_RELOC_SPARC_GOT13: bfd_reloc_code_real = 76;
pub const BFD_RELOC_SPARC_GOT10: bfd_reloc_code_real = 75;
pub const BFD_RELOC_SPARC13: bfd_reloc_code_real = 74;
pub const BFD_RELOC_SPARC22: bfd_reloc_code_real = 73;
pub const BFD_RELOC_SPARC_WDISP22: bfd_reloc_code_real = 72;
pub const BFD_RELOC_NONE: bfd_reloc_code_real = 71;
pub const BFD_RELOC_GPREL32: bfd_reloc_code_real = 70;
pub const BFD_RELOC_GPREL16: bfd_reloc_code_real = 69;
pub const BFD_RELOC_LO10: bfd_reloc_code_real = 68;
pub const BFD_RELOC_HI22: bfd_reloc_code_real = 67;
pub const BFD_RELOC_23_PCREL_S2: bfd_reloc_code_real = 66;
pub const BFD_RELOC_16_PCREL_S2: bfd_reloc_code_real = 65;
pub const BFD_RELOC_32_PCREL_S2: bfd_reloc_code_real = 64;
pub const BFD_RELOC_8_FFnn: bfd_reloc_code_real = 63;
pub const BFD_RELOC_RVA: bfd_reloc_code_real = 62;
pub const BFD_RELOC_8_BASEREL: bfd_reloc_code_real = 61;
pub const BFD_RELOC_HI16_S_BASEREL: bfd_reloc_code_real = 60;
pub const BFD_RELOC_HI16_BASEREL: bfd_reloc_code_real = 59;
pub const BFD_RELOC_LO16_BASEREL: bfd_reloc_code_real = 58;
pub const BFD_RELOC_16_BASEREL: bfd_reloc_code_real = 57;
pub const BFD_RELOC_32_BASEREL: bfd_reloc_code_real = 56;
pub const BFD_RELOC_68K_TLS_LE8: bfd_reloc_code_real = 55;
pub const BFD_RELOC_68K_TLS_LE16: bfd_reloc_code_real = 54;
pub const BFD_RELOC_68K_TLS_LE32: bfd_reloc_code_real = 53;
pub const BFD_RELOC_68K_TLS_IE8: bfd_reloc_code_real = 52;
pub const BFD_RELOC_68K_TLS_IE16: bfd_reloc_code_real = 51;
pub const BFD_RELOC_68K_TLS_IE32: bfd_reloc_code_real = 50;
pub const BFD_RELOC_68K_TLS_LDO8: bfd_reloc_code_real = 49;
pub const BFD_RELOC_68K_TLS_LDO16: bfd_reloc_code_real = 48;
pub const BFD_RELOC_68K_TLS_LDO32: bfd_reloc_code_real = 47;
pub const BFD_RELOC_68K_TLS_LDM8: bfd_reloc_code_real = 46;
pub const BFD_RELOC_68K_TLS_LDM16: bfd_reloc_code_real = 45;
pub const BFD_RELOC_68K_TLS_LDM32: bfd_reloc_code_real = 44;
pub const BFD_RELOC_68K_TLS_GD8: bfd_reloc_code_real = 43;
pub const BFD_RELOC_68K_TLS_GD16: bfd_reloc_code_real = 42;
pub const BFD_RELOC_68K_TLS_GD32: bfd_reloc_code_real = 41;
pub const BFD_RELOC_68K_RELATIVE: bfd_reloc_code_real = 40;
pub const BFD_RELOC_68K_JMP_SLOT: bfd_reloc_code_real = 39;
pub const BFD_RELOC_68K_GLOB_DAT: bfd_reloc_code_real = 38;
pub const BFD_RELOC_SIZE64: bfd_reloc_code_real = 37;
pub const BFD_RELOC_SIZE32: bfd_reloc_code_real = 36;
pub const BFD_RELOC_8_PLTOFF: bfd_reloc_code_real = 35;
pub const BFD_RELOC_HI16_S_PLTOFF: bfd_reloc_code_real = 34;
pub const BFD_RELOC_HI16_PLTOFF: bfd_reloc_code_real = 33;
pub const BFD_RELOC_LO16_PLTOFF: bfd_reloc_code_real = 32;
pub const BFD_RELOC_16_PLTOFF: bfd_reloc_code_real = 31;
pub const BFD_RELOC_32_PLTOFF: bfd_reloc_code_real = 30;
pub const BFD_RELOC_64_PLTOFF: bfd_reloc_code_real = 29;
pub const BFD_RELOC_8_PLT_PCREL: bfd_reloc_code_real = 28;
pub const BFD_RELOC_16_PLT_PCREL: bfd_reloc_code_real = 27;
pub const BFD_RELOC_24_PLT_PCREL: bfd_reloc_code_real = 26;
pub const BFD_RELOC_32_PLT_PCREL: bfd_reloc_code_real = 25;
pub const BFD_RELOC_64_PLT_PCREL: bfd_reloc_code_real = 24;
pub const BFD_RELOC_8_GOTOFF: bfd_reloc_code_real = 23;
pub const BFD_RELOC_HI16_S_GOTOFF: bfd_reloc_code_real = 22;
pub const BFD_RELOC_HI16_GOTOFF: bfd_reloc_code_real = 21;
pub const BFD_RELOC_LO16_GOTOFF: bfd_reloc_code_real = 20;
pub const BFD_RELOC_16_GOTOFF: bfd_reloc_code_real = 19;
pub const BFD_RELOC_32_GOTOFF: bfd_reloc_code_real = 18;
pub const BFD_RELOC_8_GOT_PCREL: bfd_reloc_code_real = 17;
pub const BFD_RELOC_16_GOT_PCREL: bfd_reloc_code_real = 16;
pub const BFD_RELOC_32_GOT_PCREL: bfd_reloc_code_real = 15;
pub const BFD_RELOC_32_SECREL: bfd_reloc_code_real = 14;
pub const BFD_RELOC_8_PCREL: bfd_reloc_code_real = 13;
pub const BFD_RELOC_12_PCREL: bfd_reloc_code_real = 12;
pub const BFD_RELOC_16_PCREL: bfd_reloc_code_real = 11;
pub const BFD_RELOC_24_PCREL: bfd_reloc_code_real = 10;
pub const BFD_RELOC_32_PCREL: bfd_reloc_code_real = 9;
pub const BFD_RELOC_64_PCREL: bfd_reloc_code_real = 8;
pub const BFD_RELOC_8: bfd_reloc_code_real = 7;
pub const BFD_RELOC_14: bfd_reloc_code_real = 6;
pub const BFD_RELOC_16: bfd_reloc_code_real = 5;
pub const BFD_RELOC_24: bfd_reloc_code_real = 4;
pub const BFD_RELOC_26: bfd_reloc_code_real = 3;
pub const BFD_RELOC_32: bfd_reloc_code_real = 2;
pub const BFD_RELOC_64: bfd_reloc_code_real = 1;
pub const _dummy_first_bfd_reloc_code_real: bfd_reloc_code_real = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub size: libc::c_uint,
    pub contents: *mut bfd_byte,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub section: *mut asection,
}
pub type bfd_link_order_type = libc::c_uint;
pub const bfd_symbol_reloc_link_order: bfd_link_order_type = 4;
pub const bfd_section_reloc_link_order: bfd_link_order_type = 3;
pub const bfd_data_link_order: bfd_link_order_type = 2;
pub const bfd_indirect_link_order: bfd_link_order_type = 1;
pub const bfd_undefined_link_order: bfd_link_order_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bfd_symbol {
    pub the_bfd: *mut bfd,
    pub name: *const libc::c_char,
    pub value: symvalue,
    pub flags: flagword,
    pub section: *mut bfd_section,
    pub udata: C2RustUnnamed_6,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
    pub p: *mut libc::c_void,
    pub i: bfd_vma,
}
pub type flagword = libc::c_uint;
pub type symvalue = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct relent_chain {
    pub relent: arelent,
    pub next: *mut relent_chain,
}
pub type arelent = reloc_cache_entry;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct reloc_cache_entry {
    pub sym_ptr_ptr: *mut *mut bfd_symbol,
    pub address: bfd_size_type,
    pub addend: bfd_vma,
    pub howto: *const reloc_howto_type,
}
pub type reloc_howto_type = reloc_howto_struct;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct reloc_howto_struct {
    pub type_0: libc::c_uint,
    #[bitfield(name = "size", ty = "libc::c_uint", bits = "0..=2")]
    #[bitfield(name = "bitsize", ty = "libc::c_uint", bits = "3..=9")]
    #[bitfield(name = "rightshift", ty = "libc::c_uint", bits = "10..=15")]
    #[bitfield(name = "bitpos", ty = "libc::c_uint", bits = "16..=21")]
    #[bitfield(name = "complain_on_overflow", ty = "complain_overflow", bits = "22..=23")]
    #[bitfield(name = "negate", ty = "libc::c_uint", bits = "24..=24")]
    #[bitfield(name = "pc_relative", ty = "libc::c_uint", bits = "25..=25")]
    #[bitfield(name = "partial_inplace", ty = "libc::c_uint", bits = "26..=26")]
    #[bitfield(name = "pcrel_offset", ty = "libc::c_uint", bits = "27..=27")]
    pub size_bitsize_rightshift_bitpos_complain_on_overflow_negate_pc_relative_partial_inplace_pcrel_offset: [u8; 4],
    pub src_mask: bfd_vma,
    pub dst_mask: bfd_vma,
    pub special_function: Option::<
        unsafe extern "C" fn(
            *mut bfd,
            *mut arelent,
            *mut bfd_symbol,
            *mut libc::c_void,
            *mut asection,
            *mut bfd,
            *mut *mut libc::c_char,
        ) -> bfd_reloc_status_type,
    >,
    pub name: *const libc::c_char,
}
pub type bfd_reloc_status_type = bfd_reloc_status;
pub type bfd_reloc_status = libc::c_uint;
pub const bfd_reloc_dangerous: bfd_reloc_status = 9;
pub const bfd_reloc_undefined: bfd_reloc_status = 8;
pub const bfd_reloc_other: bfd_reloc_status = 7;
pub const bfd_reloc_notsupported: bfd_reloc_status = 6;
pub const bfd_reloc_continue: bfd_reloc_status = 5;
pub const bfd_reloc_outofrange: bfd_reloc_status = 4;
pub const bfd_reloc_overflow: bfd_reloc_status = 3;
pub const bfd_reloc_ok: bfd_reloc_status = 2;
pub type complain_overflow = libc::c_uint;
pub const complain_overflow_unsigned: complain_overflow = 3;
pub const complain_overflow_signed: complain_overflow = 2;
pub const complain_overflow_bitfield: complain_overflow = 1;
pub const complain_overflow_dont: complain_overflow = 0;
pub type file_ptr = libc::c_long;
pub type alent = lineno_cache_entry;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lineno_cache_entry {
    pub line_number: libc::c_uint,
    pub u: C2RustUnnamed_7,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
    pub sym: *mut bfd_symbol,
    pub offset: bfd_vma,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct relax_table {
    pub addr: bfd_vma,
    pub size: libc::c_int,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct bfd_hash_table {
    pub table: *mut *mut bfd_hash_entry,
    pub newfunc: Option::<
        unsafe extern "C" fn(
            *mut bfd_hash_entry,
            *mut bfd_hash_table,
            *const libc::c_char,
        ) -> *mut bfd_hash_entry,
    >,
    pub memory: *mut libc::c_void,
    pub size: libc::c_uint,
    pub count: libc::c_uint,
    pub entsize: libc::c_uint,
    #[bitfield(name = "frozen", ty = "libc::c_uint", bits = "0..=0")]
    pub frozen: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bfd_hash_entry {
    pub next: *mut bfd_hash_entry,
    pub string: *const libc::c_char,
    pub hash: libc::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bfd_link_hash_table {
    pub table: bfd_hash_table,
    pub undefs: *mut bfd_link_hash_entry,
    pub undefs_tail: *mut bfd_link_hash_entry,
    pub hash_table_free: Option::<unsafe extern "C" fn(*mut bfd) -> ()>,
    pub type_0: bfd_link_hash_table_type,
}
pub type bfd_link_hash_table_type = libc::c_uint;
pub const bfd_link_elf_hash_table: bfd_link_hash_table_type = 1;
pub const bfd_link_generic_hash_table: bfd_link_hash_table_type = 0;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct bfd_link_hash_entry {
    pub root: bfd_hash_entry,
    #[bitfield(name = "type_0", ty = "bfd_link_hash_type", bits = "0..=7")]
    #[bitfield(name = "non_ir_ref_regular", ty = "libc::c_uint", bits = "8..=8")]
    #[bitfield(name = "non_ir_ref_dynamic", ty = "libc::c_uint", bits = "9..=9")]
    #[bitfield(name = "linker_def", ty = "libc::c_uint", bits = "10..=10")]
    #[bitfield(name = "ldscript_def", ty = "libc::c_uint", bits = "11..=11")]
    #[bitfield(name = "rel_from_abs", ty = "libc::c_uint", bits = "12..=12")]
    pub type_0_non_ir_ref_regular_non_ir_ref_dynamic_linker_def_ldscript_def_rel_from_abs: [u8; 2],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 6],
    pub u: C2RustUnnamed_8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_8 {
    pub undef: C2RustUnnamed_12,
    pub def: C2RustUnnamed_11,
    pub i: C2RustUnnamed_10,
    pub c: C2RustUnnamed_9,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub next: *mut bfd_link_hash_entry,
    pub p: *mut bfd_link_hash_common_entry,
    pub size: bfd_size_type,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bfd_link_hash_common_entry {
    pub alignment_power: libc::c_uint,
    pub section: *mut asection,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub next: *mut bfd_link_hash_entry,
    pub link: *mut bfd_link_hash_entry,
    pub warning: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub next: *mut bfd_link_hash_entry,
    pub section: *mut asection,
    pub value: bfd_vma,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_12 {
    pub next: *mut bfd_link_hash_entry,
    pub abfd: *mut bfd,
}
pub type bfd_link_hash_type = libc::c_uint;
pub const bfd_link_hash_warning: bfd_link_hash_type = 7;
pub const bfd_link_hash_indirect: bfd_link_hash_type = 6;
pub const bfd_link_hash_common: bfd_link_hash_type = 5;
pub const bfd_link_hash_defweak: bfd_link_hash_type = 4;
pub const bfd_link_hash_defined: bfd_link_hash_type = 3;
pub const bfd_link_hash_undefweak: bfd_link_hash_type = 2;
pub const bfd_link_hash_undefined: bfd_link_hash_type = 1;
pub const bfd_link_hash_new: bfd_link_hash_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bfd_link_callbacks {
    pub add_archive_element: Option::<
        unsafe extern "C" fn(
            *mut bfd_link_info,
            *mut bfd,
            *const libc::c_char,
            *mut *mut bfd,
        ) -> bool,
    >,
    pub multiple_definition: Option::<
        unsafe extern "C" fn(
            *mut bfd_link_info,
            *mut bfd_link_hash_entry,
            *mut bfd,
            *mut asection,
            bfd_vma,
        ) -> (),
    >,
    pub multiple_common: Option::<
        unsafe extern "C" fn(
            *mut bfd_link_info,
            *mut bfd_link_hash_entry,
            *mut bfd,
            bfd_link_hash_type,
            bfd_vma,
        ) -> (),
    >,
    pub add_to_set: Option::<
        unsafe extern "C" fn(
            *mut bfd_link_info,
            *mut bfd_link_hash_entry,
            bfd_reloc_code_real_type,
            *mut bfd,
            *mut asection,
            bfd_vma,
        ) -> (),
    >,
    pub constructor: Option::<
        unsafe extern "C" fn(
            *mut bfd_link_info,
            bool,
            *const libc::c_char,
            *mut bfd,
            *mut asection,
            bfd_vma,
        ) -> (),
    >,
    pub warning: Option::<
        unsafe extern "C" fn(
            *mut bfd_link_info,
            *const libc::c_char,
            *const libc::c_char,
            *mut bfd,
            *mut asection,
            bfd_vma,
        ) -> (),
    >,
    pub undefined_symbol: Option::<
        unsafe extern "C" fn(
            *mut bfd_link_info,
            *const libc::c_char,
            *mut bfd,
            *mut asection,
            bfd_vma,
            bool,
        ) -> (),
    >,
    pub reloc_overflow: Option::<
        unsafe extern "C" fn(
            *mut bfd_link_info,
            *mut bfd_link_hash_entry,
            *const libc::c_char,
            *const libc::c_char,
            bfd_vma,
            *mut bfd,
            *mut asection,
            bfd_vma,
        ) -> (),
    >,
    pub reloc_dangerous: Option::<
        unsafe extern "C" fn(
            *mut bfd_link_info,
            *const libc::c_char,
            *mut bfd,
            *mut asection,
            bfd_vma,
        ) -> (),
    >,
    pub unattached_reloc: Option::<
        unsafe extern "C" fn(
            *mut bfd_link_info,
            *const libc::c_char,
            *mut bfd,
            *mut asection,
            bfd_vma,
        ) -> (),
    >,
    pub notice: Option::<
        unsafe extern "C" fn(
            *mut bfd_link_info,
            *mut bfd_link_hash_entry,
            *mut bfd_link_hash_entry,
            *mut bfd,
            *mut asection,
            bfd_vma,
            flagword,
        ) -> bool,
    >,
    pub einfo: Option::<unsafe extern "C" fn(*const libc::c_char, ...) -> ()>,
    pub info: Option::<unsafe extern "C" fn(*const libc::c_char, ...) -> ()>,
    pub minfo: Option::<unsafe extern "C" fn(*const libc::c_char, ...) -> ()>,
    pub override_segment_assignment: Option::<
        unsafe extern "C" fn(
            *mut bfd_link_info,
            *mut bfd,
            *mut asection,
            *mut asection,
            bool,
        ) -> bool,
    >,
    pub examine_strtab: Option::<unsafe extern "C" fn(*mut elf_strtab_hash) -> ()>,
    pub ctf_new_symbol: Option::<
        unsafe extern "C" fn(libc::c_int, *mut elf_internal_sym) -> (),
    >,
    pub ctf_new_dynsym: Option::<
        unsafe extern "C" fn(libc::c_int, *mut elf_internal_sym) -> (),
    >,
    pub emit_ctf: Option::<unsafe extern "C" fn() -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct elf_internal_sym {
    pub st_value: bfd_vma,
    pub st_size: bfd_vma,
    pub st_name: libc::c_ulong,
    pub st_info: libc::c_uchar,
    pub st_other: libc::c_uchar,
    pub st_target_internal: libc::c_uchar,
    pub st_shndx: libc::c_uint,
}
pub type bfd_signed_vma = libc::c_long;
pub type compressed_debug_section_type = libc::c_uint;
pub const COMPRESS_DEBUG_GABI_ZLIB: compressed_debug_section_type = 5;
pub const COMPRESS_DEBUG_GNU_ZLIB: compressed_debug_section_type = 3;
pub const COMPRESS_DEBUG: compressed_debug_section_type = 1;
pub const COMPRESS_DEBUG_NONE: compressed_debug_section_type = 0;
pub type textrel_check_method = libc::c_uint;
pub const textrel_check_error: textrel_check_method = 2;
pub const textrel_check_warning: textrel_check_method = 1;
pub const textrel_check_none: textrel_check_method = 0;
pub type report_method = libc::c_uint;
pub const RM_DIAGNOSE: report_method = 2;
pub const RM_IGNORE: report_method = 1;
pub const RM_NOT_YET_SET: report_method = 0;
pub type bfd_link_common_skip_ar_symbols = libc::c_uint;
pub const bfd_link_common_skip_all: bfd_link_common_skip_ar_symbols = 3;
pub const bfd_link_common_skip_data: bfd_link_common_skip_ar_symbols = 2;
pub const bfd_link_common_skip_text: bfd_link_common_skip_ar_symbols = 1;
pub const bfd_link_common_skip_none: bfd_link_common_skip_ar_symbols = 0;
pub type bfd_link_elf_stt_common = libc::c_uint;
pub const no_elf_stt_common: bfd_link_elf_stt_common = 2;
pub const elf_stt_common: bfd_link_elf_stt_common = 1;
pub const unchanged: bfd_link_elf_stt_common = 0;
pub type bfd_link_discard = libc::c_uint;
pub const discard_all: bfd_link_discard = 3;
pub const discard_l: bfd_link_discard = 2;
pub const discard_none: bfd_link_discard = 1;
pub const discard_sec_merge: bfd_link_discard = 0;
pub type bfd_link_strip = libc::c_uint;
pub const strip_all: bfd_link_strip = 3;
pub const strip_some: bfd_link_strip = 2;
pub const strip_debugger: bfd_link_strip = 1;
pub const strip_none: bfd_link_strip = 0;
pub type output_type = libc::c_uint;
pub const type_dll: output_type = 3;
pub const type_relocatable: output_type = 2;
pub const type_pie: output_type = 1;
pub const type_pde: output_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_13 {
    pub after_write_object_contents: Option::<unsafe extern "C" fn(*mut bfd) -> bool>,
    pub style: *const libc::c_char,
    pub sec: *mut asection,
}
pub type asymbol = bfd_symbol;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct elf_segment_map {
    pub next: *mut elf_segment_map,
    pub p_type: libc::c_ulong,
    pub p_flags: libc::c_ulong,
    pub p_paddr: bfd_vma,
    pub p_vaddr_offset: bfd_vma,
    pub p_align: bfd_vma,
    pub p_size: bfd_vma,
    #[bitfield(name = "p_flags_valid", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "p_paddr_valid", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "p_align_valid", ty = "libc::c_uint", bits = "2..=2")]
    #[bitfield(name = "p_size_valid", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "includes_filehdr", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "includes_phdrs", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "no_sort_lma", ty = "libc::c_uint", bits = "6..=6")]
    pub p_flags_valid_p_paddr_valid_p_align_valid_p_size_valid_includes_filehdr_includes_phdrs_no_sort_lma: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub idx: libc::c_uint,
    pub count: libc::c_uint,
    pub sections: [*mut asection; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct core_elf_obj_tdata {
    pub signal: libc::c_int,
    pub pid: libc::c_int,
    pub lwpid: libc::c_int,
    pub program: *mut libc::c_char,
    pub command: *mut libc::c_char,
}
pub type elf_gnu_osabi = libc::c_uint;
pub const elf_gnu_osabi_retain: elf_gnu_osabi = 8;
pub const elf_gnu_osabi_unique: elf_gnu_osabi = 4;
pub const elf_gnu_osabi_ifunc: elf_gnu_osabi = 2;
pub const elf_gnu_osabi_mbind: elf_gnu_osabi = 1;
pub type dynamic_lib_link_class = libc::c_uint;
pub const DYN_NO_NEEDED: dynamic_lib_link_class = 8;
pub const DYN_NO_ADD_NEEDED: dynamic_lib_link_class = 4;
pub const DYN_DT_NEEDED: dynamic_lib_link_class = 2;
pub const DYN_AS_NEEDED: dynamic_lib_link_class = 1;
pub const DYN_NORMAL: dynamic_lib_link_class = 0;
pub type elf_target_id = libc::c_uint;
pub const GENERIC_ELF_DATA: elf_target_id = 38;
pub const RISCV_ELF_DATA: elf_target_id = 37;
pub const TILEPRO_ELF_DATA: elf_target_id = 36;
pub const TILEGX_ELF_DATA: elf_target_id = 35;
pub const XTENSA_ELF_DATA: elf_target_id = 34;
pub const X86_64_ELF_DATA: elf_target_id = 33;
pub const TIC6X_ELF_DATA: elf_target_id = 32;
pub const SPU_ELF_DATA: elf_target_id = 31;
pub const SPARC_ELF_DATA: elf_target_id = 30;
pub const SH_ELF_DATA: elf_target_id = 29;
pub const S390_ELF_DATA: elf_target_id = 28;
pub const PRU_ELF_DATA: elf_target_id = 27;
pub const PPC64_ELF_DATA: elf_target_id = 26;
pub const PPC32_ELF_DATA: elf_target_id = 25;
pub const OR1K_ELF_DATA: elf_target_id = 24;
pub const NIOS2_ELF_DATA: elf_target_id = 23;
pub const NDS32_ELF_DATA: elf_target_id = 22;
pub const MN10300_ELF_DATA: elf_target_id = 21;
pub const MIPS_ELF_DATA: elf_target_id = 20;
pub const MICROBLAZE_ELF_DATA: elf_target_id = 19;
pub const METAG_ELF_DATA: elf_target_id = 18;
pub const M68K_ELF_DATA: elf_target_id = 17;
pub const M68HC11_ELF_DATA: elf_target_id = 16;
pub const M32R_ELF_DATA: elf_target_id = 15;
pub const LM32_ELF_DATA: elf_target_id = 14;
pub const IA64_ELF_DATA: elf_target_id = 13;
pub const I386_ELF_DATA: elf_target_id = 12;
pub const HPPA64_ELF_DATA: elf_target_id = 11;
pub const HPPA32_ELF_DATA: elf_target_id = 10;
pub const FRV_ELF_DATA: elf_target_id = 9;
pub const CSKY_ELF_DATA: elf_target_id = 8;
pub const CRIS_ELF_DATA: elf_target_id = 7;
pub const BFIN_ELF_DATA: elf_target_id = 6;
pub const AVR_ELF_DATA: elf_target_id = 5;
pub const ARM_ELF_DATA: elf_target_id = 4;
pub const ARC_ELF_DATA: elf_target_id = 3;
pub const ALPHA_ELF_DATA: elf_target_id = 2;
pub const AARCH64_ELF_DATA: elf_target_id = 1;
pub type Elf_Internal_Shdr = elf_internal_shdr;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct elf_internal_shdr {
    pub sh_name: libc::c_uint,
    pub sh_type: libc::c_uint,
    pub sh_flags: bfd_vma,
    pub sh_addr: bfd_vma,
    pub sh_offset: file_ptr,
    pub sh_size: bfd_size_type,
    pub sh_link: libc::c_uint,
    pub sh_info: libc::c_uint,
    pub sh_addralign: bfd_vma,
    pub sh_entsize: bfd_size_type,
    pub bfd_section: *mut asection,
    pub contents: *mut libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sdt_note {
    pub next: *mut sdt_note,
    pub size: bfd_size_type,
    pub data: [bfd_byte; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct obj_attribute_list {
    pub next: *mut obj_attribute_list,
    pub tag: libc::c_uint,
    pub attr: obj_attribute,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct obj_attribute {
    pub type_0: libc::c_int,
    pub i: libc::c_uint,
    pub s: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct elf_property_list {
    pub next: *mut elf_property_list,
    pub property: elf_property,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct elf_property {
    pub pr_type: libc::c_uint,
    pub pr_datasz: libc::c_uint,
    pub u: C2RustUnnamed_14,
    pub pr_kind: elf_property_kind,
}
pub type elf_property_kind = libc::c_uint;
pub const property_number: elf_property_kind = 4;
pub const property_remove: elf_property_kind = 3;
pub const property_corrupt: elf_property_kind = 2;
pub const property_ignored: elf_property_kind = 1;
pub const property_unknown: elf_property_kind = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_14 {
    pub number: bfd_vma,
}
pub type Elf_Internal_Verneed = elf_internal_verneed;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct elf_internal_verneed {
    pub vn_version: libc::c_ushort,
    pub vn_cnt: libc::c_ushort,
    pub vn_file: libc::c_ulong,
    pub vn_aux: libc::c_ulong,
    pub vn_next: libc::c_ulong,
    pub vn_bfd: *mut bfd,
    pub vn_filename: *const libc::c_char,
    pub vn_auxptr: *mut elf_internal_vernaux,
    pub vn_nextref: *mut elf_internal_verneed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct elf_internal_vernaux {
    pub vna_hash: libc::c_ulong,
    pub vna_flags: libc::c_ushort,
    pub vna_other: libc::c_ushort,
    pub vna_name: libc::c_ulong,
    pub vna_next: libc::c_ulong,
    pub vna_nodename: *const libc::c_char,
    pub vna_nextptr: *mut elf_internal_vernaux,
}
pub type Elf_Internal_Verdef = elf_internal_verdef;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct elf_internal_verdef {
    pub vd_version: libc::c_ushort,
    pub vd_flags: libc::c_ushort,
    pub vd_ndx: libc::c_ushort,
    pub vd_cnt: libc::c_ushort,
    pub vd_hash: libc::c_ulong,
    pub vd_aux: libc::c_ulong,
    pub vd_next: libc::c_ulong,
    pub vd_bfd: *mut bfd,
    pub vd_nodename: *const libc::c_char,
    pub vd_nextdef: *mut elf_internal_verdef,
    pub vd_auxptr: *mut elf_internal_verdaux,
    pub vd_exp_refno: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct elf_internal_verdaux {
    pub vda_name: libc::c_ulong,
    pub vda_next: libc::c_ulong,
    pub vda_nodename: *const libc::c_char,
    pub vda_nextptr: *mut elf_internal_verdaux,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_15 {
    pub refcounts: *mut bfd_signed_vma,
    pub offsets: *mut bfd_vma,
    pub ents: *mut *mut got_entry,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct elf_link_hash_entry {
    pub root: bfd_link_hash_entry,
    pub indx: libc::c_long,
    pub dynindx: libc::c_long,
    pub got: gotplt_union,
    pub plt: gotplt_union,
    pub size: bfd_size_type,
    pub dyn_relocs: *mut elf_dyn_relocs,
    #[bitfield(name = "type_0", ty = "libc::c_uint", bits = "0..=7")]
    #[bitfield(name = "other", ty = "libc::c_uint", bits = "8..=15")]
    #[bitfield(name = "target_internal", ty = "libc::c_uint", bits = "16..=23")]
    #[bitfield(name = "ref_regular", ty = "libc::c_uint", bits = "24..=24")]
    #[bitfield(name = "def_regular", ty = "libc::c_uint", bits = "25..=25")]
    #[bitfield(name = "ref_dynamic", ty = "libc::c_uint", bits = "26..=26")]
    #[bitfield(name = "def_dynamic", ty = "libc::c_uint", bits = "27..=27")]
    #[bitfield(name = "ref_regular_nonweak", ty = "libc::c_uint", bits = "28..=28")]
    #[bitfield(name = "ref_ir_nonweak", ty = "libc::c_uint", bits = "29..=29")]
    #[bitfield(name = "dynamic_adjusted", ty = "libc::c_uint", bits = "30..=30")]
    #[bitfield(name = "needs_copy", ty = "libc::c_uint", bits = "31..=31")]
    #[bitfield(name = "needs_plt", ty = "libc::c_uint", bits = "32..=32")]
    #[bitfield(name = "non_elf", ty = "libc::c_uint", bits = "33..=33")]
    #[bitfield(name = "versioned", ty = "elf_symbol_version", bits = "34..=35")]
    #[bitfield(name = "forced_local", ty = "libc::c_uint", bits = "36..=36")]
    #[bitfield(name = "dynamic", ty = "libc::c_uint", bits = "37..=37")]
    #[bitfield(name = "mark", ty = "libc::c_uint", bits = "38..=38")]
    #[bitfield(name = "non_got_ref", ty = "libc::c_uint", bits = "39..=39")]
    #[bitfield(name = "dynamic_def", ty = "libc::c_uint", bits = "40..=40")]
    #[bitfield(name = "ref_dynamic_nonweak", ty = "libc::c_uint", bits = "41..=41")]
    #[bitfield(name = "pointer_equality_needed", ty = "libc::c_uint", bits = "42..=42")]
    #[bitfield(name = "unique_global", ty = "libc::c_uint", bits = "43..=43")]
    #[bitfield(name = "protected_def", ty = "libc::c_uint", bits = "44..=44")]
    #[bitfield(name = "start_stop", ty = "libc::c_uint", bits = "45..=45")]
    #[bitfield(name = "is_weakalias", ty = "libc::c_uint", bits = "46..=46")]
    pub type_0_other_target_internal_ref_regular_def_regular_ref_dynamic_def_dynamic_ref_regular_nonweak_ref_ir_nonweak_dynamic_adjusted_needs_copy_needs_plt_non_elf_versioned_forced_local_dynamic_mark_non_got_ref_dynamic_def_ref_dynamic_nonweak_pointer_equality_needed_unique_global_protected_def_start_stop_is_weakalias: [u8; 6],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 2],
    pub dynstr_index: libc::c_ulong,
    pub u: C2RustUnnamed_18,
    pub verinfo: C2RustUnnamed_17,
    pub u2: C2RustUnnamed_16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_16 {
    pub start_stop_section: *mut asection,
    pub vtable: *mut elf_link_virtual_table_entry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct elf_link_virtual_table_entry {
    pub size: size_t,
    pub used: *mut bool,
    pub parent: *mut elf_link_hash_entry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_17 {
    pub verdef: *mut Elf_Internal_Verdef,
    pub vertree: *mut bfd_elf_version_tree,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_18 {
    pub alias: *mut elf_link_hash_entry,
    pub elf_hash_value: libc::c_ulong,
}
pub type elf_symbol_version = libc::c_uint;
pub const versioned_hidden: elf_symbol_version = 3;
pub const versioned: elf_symbol_version = 2;
pub const unversioned: elf_symbol_version = 1;
pub const unknown: elf_symbol_version = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct elf_dyn_relocs {
    pub next: *mut elf_dyn_relocs,
    pub sec: *mut asection,
    pub count: bfd_size_type,
    pub pc_count: bfd_size_type,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union gotplt_union {
    pub refcount: bfd_signed_vma,
    pub offset: bfd_vma,
    pub glist: *mut got_entry,
    pub plist: *mut plt_entry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct elf_section_list {
    pub hdr: Elf_Internal_Shdr,
    pub ndx: libc::c_uint,
    pub next: *mut elf_section_list,
}
pub type Elf_Internal_Phdr = elf_internal_phdr;
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
pub type Elf_Internal_Ehdr = elf_internal_ehdr;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcoff_tdata {
    pub coff: coff_data_type,
    pub xcoff64: bool,
    pub full_aouthdr: bool,
    pub toc: bfd_vma,
    pub sntoc: libc::c_int,
    pub snentry: libc::c_int,
    pub text_align_power: libc::c_int,
    pub data_align_power: libc::c_int,
    pub modtype: libc::c_short,
    pub cputype: libc::c_short,
    pub maxdata: bfd_vma,
    pub maxstack: bfd_vma,
    pub csects: *mut *mut asection,
    pub debug_indices: *mut libc::c_long,
    pub lineno_counts: *mut libc::c_uint,
    pub import_file_id: libc::c_uint,
}
pub type coff_data_type = coff_tdata;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct coff_tdata {
    pub symbols: *mut coff_symbol_struct,
    pub conversion_table: *mut libc::c_uint,
    pub conv_table_size: libc::c_int,
    pub sym_filepos: file_ptr,
    pub raw_syments: *mut coff_ptr_struct,
    pub raw_syment_count: libc::c_ulong,
    pub relocbase: libc::c_ulong,
    pub local_n_btmask: libc::c_uint,
    pub local_n_btshft: libc::c_uint,
    pub local_n_tmask: libc::c_uint,
    pub local_n_tshift: libc::c_uint,
    pub local_symesz: libc::c_uint,
    pub local_auxesz: libc::c_uint,
    pub local_linesz: libc::c_uint,
    pub external_syms: *mut libc::c_void,
    pub keep_syms: bool,
    pub strings: *mut libc::c_char,
    pub strings_len: bfd_size_type,
    pub keep_strings: bool,
    pub strings_written: bool,
    pub pe: libc::c_int,
    pub sym_hashes: *mut *mut coff_link_hash_entry,
    pub local_toc_sym_map: *mut libc::c_int,
    pub link_info: *mut bfd_link_info,
    pub line_info: *mut libc::c_void,
    pub dwarf2_find_line_info: *mut libc::c_void,
    pub timestamp: libc::c_long,
    pub flags: flagword,
    pub go32: bool,
    pub stub: *mut libc::c_char,
    pub stub_size: bfd_size_type,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct coff_link_hash_entry {
    pub root: bfd_link_hash_entry,
    pub indx: libc::c_long,
    pub type_0: libc::c_ushort,
    pub symbol_class: libc::c_uchar,
    pub numaux: libc::c_char,
    pub auxbfd: *mut bfd,
    pub aux: *mut internal_auxent,
    pub coff_link_hash_flags: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union internal_auxent {
    pub x_sym: C2RustUnnamed_29,
    pub x_file: C2RustUnnamed_27,
    pub x_scn: C2RustUnnamed_26,
    pub x_tv: C2RustUnnamed_25,
    pub x_csect: C2RustUnnamed_20,
    pub x_sect: C2RustUnnamed_19,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_19 {
    pub x_scnlen: libc::c_long,
    pub x_nreloc: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_20 {
    pub x_scnlen: C2RustUnnamed_21,
    pub x_parmhash: libc::c_long,
    pub x_snhash: libc::c_ushort,
    pub x_smtyp: libc::c_uchar,
    pub x_smclas: libc::c_uchar,
    pub x_stab: libc::c_long,
    pub x_snstab: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_21 {
    pub l: bfd_signed_vma,
    pub p: *mut coff_ptr_struct,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct coff_ptr_struct {
    pub offset: libc::c_uint,
    #[bitfield(name = "fix_value", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "fix_tag", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "fix_end", ty = "libc::c_uint", bits = "2..=2")]
    #[bitfield(name = "fix_scnlen", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "fix_line", ty = "libc::c_uint", bits = "4..=4")]
    pub fix_value_fix_tag_fix_end_fix_scnlen_fix_line: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub u: C2RustUnnamed_22,
    pub is_sym: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_22 {
    pub auxent: internal_auxent,
    pub syment: internal_syment,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct internal_syment {
    pub _n: C2RustUnnamed_23,
    pub n_value: bfd_vma,
    pub n_scnum: libc::c_int,
    pub n_flags: libc::c_ushort,
    pub n_type: libc::c_ushort,
    pub n_sclass: libc::c_uchar,
    pub n_numaux: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_23 {
    pub _n_name: [libc::c_char; 8],
    pub _n_n: C2RustUnnamed_24,
    pub _n_nptr: [*mut libc::c_char; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_24 {
    pub _n_zeroes: bfd_hostptr_t,
    pub _n_offset: bfd_hostptr_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_25 {
    pub x_tvfill: libc::c_long,
    pub x_tvlen: libc::c_ushort,
    pub x_tvran: [libc::c_ushort; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_26 {
    pub x_scnlen: libc::c_long,
    pub x_nreloc: libc::c_ushort,
    pub x_nlinno: libc::c_ushort,
    pub x_checksum: libc::c_ulong,
    pub x_associated: libc::c_ushort,
    pub x_comdat: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_27 {
    pub x_fname: [libc::c_char; 20],
    pub x_n: C2RustUnnamed_28,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_28 {
    pub x_zeroes: libc::c_long,
    pub x_offset: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_29 {
    pub x_tagndx: C2RustUnnamed_36,
    pub x_misc: C2RustUnnamed_34,
    pub x_fcnary: C2RustUnnamed_30,
    pub x_tvndx: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_30 {
    pub x_fcn: C2RustUnnamed_32,
    pub x_ary: C2RustUnnamed_31,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_31 {
    pub x_dimen: [libc::c_ushort; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_32 {
    pub x_lnnoptr: bfd_signed_vma,
    pub x_endndx: C2RustUnnamed_33,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_33 {
    pub l: libc::c_long,
    pub p: *mut coff_ptr_struct,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_34 {
    pub x_lnsz: C2RustUnnamed_35,
    pub x_fsize: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_35 {
    pub x_lnno: libc::c_ushort,
    pub x_size: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_36 {
    pub l: libc::c_long,
    pub p: *mut coff_ptr_struct,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct coff_symbol_struct {
    pub symbol: asymbol,
    pub native: *mut combined_entry_type,
    pub lineno: *mut lineno_cache_entry,
    pub done_lineno: bool,
}
pub type combined_entry_type = coff_ptr_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pe_tdata {
    pub coff: coff_data_type,
    pub pe_opthdr: internal_extra_pe_aouthdr,
    pub dll: libc::c_int,
    pub has_reloc_section: libc::c_int,
    pub dont_strip_reloc: libc::c_int,
    pub dos_message: [libc::c_int; 16],
    pub timestamp: libc::c_int,
    pub in_reloc_p: Option::<
        unsafe extern "C" fn(*mut bfd, *const reloc_howto_type) -> bool,
    >,
    pub real_flags: flagword,
    pub build_id: C2RustUnnamed_37,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_37 {
    pub after_write_object_contents: Option::<unsafe extern "C" fn(*mut bfd) -> bool>,
    pub style: *const libc::c_char,
    pub sec: *mut asection,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct internal_extra_pe_aouthdr {
    pub Magic: libc::c_short,
    pub MajorLinkerVersion: libc::c_char,
    pub MinorLinkerVersion: libc::c_char,
    pub SizeOfCode: bfd_vma,
    pub SizeOfInitializedData: bfd_vma,
    pub SizeOfUninitializedData: bfd_vma,
    pub AddressOfEntryPoint: bfd_vma,
    pub BaseOfCode: bfd_vma,
    pub BaseOfData: bfd_vma,
    pub ImageBase: bfd_vma,
    pub SectionAlignment: uint32_t,
    pub FileAlignment: uint32_t,
    pub MajorOperatingSystemVersion: libc::c_short,
    pub MinorOperatingSystemVersion: libc::c_short,
    pub MajorImageVersion: libc::c_short,
    pub MinorImageVersion: libc::c_short,
    pub MajorSubsystemVersion: libc::c_short,
    pub MinorSubsystemVersion: libc::c_short,
    pub Reserved1: uint32_t,
    pub SizeOfImage: uint32_t,
    pub SizeOfHeaders: uint32_t,
    pub CheckSum: uint32_t,
    pub Subsystem: libc::c_short,
    pub DllCharacteristics: libc::c_ushort,
    pub SizeOfStackReserve: bfd_vma,
    pub SizeOfStackCommit: bfd_vma,
    pub SizeOfHeapReserve: bfd_vma,
    pub SizeOfHeapCommit: bfd_vma,
    pub LoaderFlags: uint32_t,
    pub NumberOfRvaAndSizes: uint32_t,
    pub DataDirectory: [IMAGE_DATA_DIRECTORY; 16],
}
pub type IMAGE_DATA_DIRECTORY = _IMAGE_DATA_DIRECTORY;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IMAGE_DATA_DIRECTORY {
    pub VirtualAddress: bfd_vma,
    pub Size: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_38 {
    pub next: *mut bfd,
    pub hash: *mut bfd_link_hash_table,
}
pub type ufile_ptr = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bfd_arch_info {
    pub bits_per_word: libc::c_int,
    pub bits_per_address: libc::c_int,
    pub bits_per_byte: libc::c_int,
    pub arch: bfd_architecture,
    pub mach: libc::c_ulong,
    pub arch_name: *const libc::c_char,
    pub printable_name: *const libc::c_char,
    pub section_align_power: libc::c_uint,
    pub the_default: bool,
    pub compatible: Option::<
        unsafe extern "C" fn(
            *const bfd_arch_info,
            *const bfd_arch_info,
        ) -> *const bfd_arch_info,
    >,
    pub scan: Option::<
        unsafe extern "C" fn(*const bfd_arch_info, *const libc::c_char) -> bool,
    >,
    pub fill: Option::<
        unsafe extern "C" fn(bfd_size_type, bool, bool) -> *mut libc::c_void,
    >,
    pub next: *const bfd_arch_info,
    pub max_reloc_offset_into_insn: libc::c_int,
}
pub type bfd_architecture = libc::c_uint;
pub const bfd_arch_last: bfd_architecture = 88;
pub const bfd_arch_csky: bfd_architecture = 87;
pub const bfd_arch_nfp: bfd_architecture = 86;
pub const bfd_arch_pru: bfd_architecture = 85;
pub const bfd_arch_wasm32: bfd_architecture = 84;
pub const bfd_arch_visium: bfd_architecture = 83;
pub const bfd_arch_nios2: bfd_architecture = 82;
pub const bfd_arch_aarch64: bfd_architecture = 81;
pub const bfd_arch_tilegx: bfd_architecture = 80;
pub const bfd_arch_tilepro: bfd_architecture = 79;
pub const bfd_arch_microblaze: bfd_architecture = 78;
pub const bfd_arch_lm32: bfd_architecture = 77;
pub const bfd_arch_z80: bfd_architecture = 76;
pub const bfd_arch_xtensa: bfd_architecture = 75;
pub const bfd_arch_xgate: bfd_architecture = 74;
pub const bfd_arch_xc16x: bfd_architecture = 73;
pub const bfd_arch_msp430: bfd_architecture = 72;
pub const bfd_arch_xstormy16: bfd_architecture = 71;
pub const bfd_arch_mmix: bfd_architecture = 70;
pub const bfd_arch_score: bfd_architecture = 69;
pub const bfd_arch_s390: bfd_architecture = 68;
pub const bfd_arch_rx: bfd_architecture = 67;
pub const bfd_arch_rl78: bfd_architecture = 66;
pub const bfd_arch_riscv: bfd_architecture = 65;
pub const bfd_arch_cris: bfd_architecture = 64;
pub const bfd_arch_crx: bfd_architecture = 63;
pub const bfd_arch_cr16: bfd_architecture = 62;
pub const bfd_arch_bfin: bfd_architecture = 61;
pub const bfd_arch_avr: bfd_architecture = 60;
pub const bfd_arch_pj: bfd_architecture = 59;
pub const bfd_arch_mt: bfd_architecture = 58;
pub const bfd_arch_epiphany: bfd_architecture = 57;
pub const bfd_arch_bpf: bfd_architecture = 56;
pub const bfd_arch_iq2000: bfd_architecture = 55;
pub const bfd_arch_ip2k: bfd_architecture = 54;
pub const bfd_arch_ia64: bfd_architecture = 53;
pub const bfd_arch_metag: bfd_architecture = 52;
pub const bfd_arch_mep: bfd_architecture = 51;
pub const bfd_arch_mcore: bfd_architecture = 50;
pub const bfd_arch_ft32: bfd_architecture = 49;
pub const bfd_arch_moxie: bfd_architecture = 48;
pub const bfd_arch_frv: bfd_architecture = 47;
pub const bfd_arch_fr30: bfd_architecture = 46;
pub const bfd_arch_mn10300: bfd_architecture = 45;
pub const bfd_arch_mn10200: bfd_architecture = 44;
pub const bfd_arch_m32r: bfd_architecture = 43;
pub const bfd_arch_m32c: bfd_architecture = 42;
pub const bfd_arch_arc: bfd_architecture = 41;
pub const bfd_arch_v850_rh850: bfd_architecture = 40;
pub const bfd_arch_v850: bfd_architecture = 39;
pub const bfd_arch_tic6x: bfd_architecture = 38;
pub const bfd_arch_tic54x: bfd_architecture = 37;
pub const bfd_arch_tic4x: bfd_architecture = 36;
pub const bfd_arch_tic30: bfd_architecture = 35;
pub const bfd_arch_ns32k: bfd_architecture = 34;
pub const bfd_arch_nds32: bfd_architecture = 33;
pub const bfd_arch_arm: bfd_architecture = 32;
pub const bfd_arch_alpha: bfd_architecture = 31;
pub const bfd_arch_sh: bfd_architecture = 30;
pub const bfd_arch_z8k: bfd_architecture = 29;
pub const bfd_arch_s12z: bfd_architecture = 28;
pub const bfd_arch_m9s12xg: bfd_architecture = 27;
pub const bfd_arch_m9s12x: bfd_architecture = 26;
pub const bfd_arch_m68hc12: bfd_architecture = 25;
pub const bfd_arch_m68hc11: bfd_architecture = 24;
pub const bfd_arch_dlx: bfd_architecture = 23;
pub const bfd_arch_d30v: bfd_architecture = 22;
pub const bfd_arch_d10v: bfd_architecture = 21;
pub const bfd_arch_hppa: bfd_architecture = 20;
pub const bfd_arch_rs6000: bfd_architecture = 19;
pub const bfd_arch_powerpc: bfd_architecture = 18;
pub const bfd_arch_pdp11: bfd_architecture = 17;
pub const bfd_arch_h8300: bfd_architecture = 16;
pub const bfd_arch_pyramid: bfd_architecture = 15;
pub const bfd_arch_m98k: bfd_architecture = 14;
pub const bfd_arch_convex: bfd_architecture = 13;
pub const bfd_arch_romp: bfd_architecture = 12;
pub const bfd_arch_iamcu: bfd_architecture = 11;
pub const bfd_arch_k1om: bfd_architecture = 10;
pub const bfd_arch_l1om: bfd_architecture = 9;
pub const bfd_arch_i386: bfd_architecture = 8;
pub const bfd_arch_mips: bfd_architecture = 7;
pub const bfd_arch_spu: bfd_architecture = 6;
pub const bfd_arch_sparc: bfd_architecture = 5;
pub const bfd_arch_or1k: bfd_architecture = 4;
pub const bfd_arch_vax: bfd_architecture = 3;
pub const bfd_arch_m68k: bfd_architecture = 2;
pub const bfd_arch_obscure: bfd_architecture = 1;
pub const bfd_arch_unknown: bfd_architecture = 0;
pub type bfd_plugin_format = libc::c_uint;
pub const bfd_plugin_no: bfd_plugin_format = 2;
pub const bfd_plugin_yes: bfd_plugin_format = 1;
pub const bfd_plugin_unknown: bfd_plugin_format = 0;
pub type bfd_direction = libc::c_uint;
pub const both_direction: bfd_direction = 3;
pub const write_direction: bfd_direction = 2;
pub const read_direction: bfd_direction = 1;
pub const no_direction: bfd_direction = 0;
pub type bfd_format = libc::c_uint;
pub const bfd_type_end: bfd_format = 4;
pub const bfd_core: bfd_format = 3;
pub const bfd_archive: bfd_format = 2;
pub const bfd_object: bfd_format = 1;
pub const bfd_unknown: bfd_format = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bfd_target {
    pub name: *const libc::c_char,
    pub flavour: bfd_flavour,
    pub byteorder: bfd_endian,
    pub header_byteorder: bfd_endian,
    pub object_flags: flagword,
    pub section_flags: flagword,
    pub symbol_leading_char: libc::c_char,
    pub ar_pad_char: libc::c_char,
    pub ar_max_namelen: libc::c_uchar,
    pub match_priority: libc::c_uchar,
    pub keep_unused_section_symbols: bool,
    pub bfd_getx64: Option::<unsafe extern "C" fn(*const libc::c_void) -> bfd_uint64_t>,
    pub bfd_getx_signed_64: Option::<
        unsafe extern "C" fn(*const libc::c_void) -> bfd_int64_t,
    >,
    pub bfd_putx64: Option::<
        unsafe extern "C" fn(bfd_uint64_t, *mut libc::c_void) -> (),
    >,
    pub bfd_getx32: Option::<unsafe extern "C" fn(*const libc::c_void) -> bfd_vma>,
    pub bfd_getx_signed_32: Option::<
        unsafe extern "C" fn(*const libc::c_void) -> bfd_signed_vma,
    >,
    pub bfd_putx32: Option::<unsafe extern "C" fn(bfd_vma, *mut libc::c_void) -> ()>,
    pub bfd_getx16: Option::<unsafe extern "C" fn(*const libc::c_void) -> bfd_vma>,
    pub bfd_getx_signed_16: Option::<
        unsafe extern "C" fn(*const libc::c_void) -> bfd_signed_vma,
    >,
    pub bfd_putx16: Option::<unsafe extern "C" fn(bfd_vma, *mut libc::c_void) -> ()>,
    pub bfd_h_getx64: Option::<
        unsafe extern "C" fn(*const libc::c_void) -> bfd_uint64_t,
    >,
    pub bfd_h_getx_signed_64: Option::<
        unsafe extern "C" fn(*const libc::c_void) -> bfd_int64_t,
    >,
    pub bfd_h_putx64: Option::<
        unsafe extern "C" fn(bfd_uint64_t, *mut libc::c_void) -> (),
    >,
    pub bfd_h_getx32: Option::<unsafe extern "C" fn(*const libc::c_void) -> bfd_vma>,
    pub bfd_h_getx_signed_32: Option::<
        unsafe extern "C" fn(*const libc::c_void) -> bfd_signed_vma,
    >,
    pub bfd_h_putx32: Option::<unsafe extern "C" fn(bfd_vma, *mut libc::c_void) -> ()>,
    pub bfd_h_getx16: Option::<unsafe extern "C" fn(*const libc::c_void) -> bfd_vma>,
    pub bfd_h_getx_signed_16: Option::<
        unsafe extern "C" fn(*const libc::c_void) -> bfd_signed_vma,
    >,
    pub bfd_h_putx16: Option::<unsafe extern "C" fn(bfd_vma, *mut libc::c_void) -> ()>,
    pub _bfd_check_format: [Option::<unsafe extern "C" fn(*mut bfd) -> bfd_cleanup>; 4],
    pub _bfd_set_format: [Option::<unsafe extern "C" fn(*mut bfd) -> bool>; 4],
    pub _bfd_write_contents: [Option::<unsafe extern "C" fn(*mut bfd) -> bool>; 4],
    pub _close_and_cleanup: Option::<unsafe extern "C" fn(*mut bfd) -> bool>,
    pub _bfd_free_cached_info: Option::<unsafe extern "C" fn(*mut bfd) -> bool>,
    pub _new_section_hook: Option::<unsafe extern "C" fn(*mut bfd, sec_ptr) -> bool>,
    pub _bfd_get_section_contents: Option::<
        unsafe extern "C" fn(
            *mut bfd,
            sec_ptr,
            *mut libc::c_void,
            file_ptr,
            bfd_size_type,
        ) -> bool,
    >,
    pub _bfd_get_section_contents_in_window: Option::<
        unsafe extern "C" fn(
            *mut bfd,
            sec_ptr,
            *mut bfd_window,
            file_ptr,
            bfd_size_type,
        ) -> bool,
    >,
    pub _bfd_copy_private_bfd_data: Option::<
        unsafe extern "C" fn(*mut bfd, *mut bfd) -> bool,
    >,
    pub _bfd_merge_private_bfd_data: Option::<
        unsafe extern "C" fn(*mut bfd, *mut bfd_link_info) -> bool,
    >,
    pub _bfd_init_private_section_data: Option::<
        unsafe extern "C" fn(
            *mut bfd,
            sec_ptr,
            *mut bfd,
            sec_ptr,
            *mut bfd_link_info,
        ) -> bool,
    >,
    pub _bfd_copy_private_section_data: Option::<
        unsafe extern "C" fn(*mut bfd, sec_ptr, *mut bfd, sec_ptr) -> bool,
    >,
    pub _bfd_copy_private_symbol_data: Option::<
        unsafe extern "C" fn(*mut bfd, *mut asymbol, *mut bfd, *mut asymbol) -> bool,
    >,
    pub _bfd_copy_private_header_data: Option::<
        unsafe extern "C" fn(*mut bfd, *mut bfd) -> bool,
    >,
    pub _bfd_set_private_flags: Option::<
        unsafe extern "C" fn(*mut bfd, flagword) -> bool,
    >,
    pub _bfd_print_private_bfd_data: Option::<
        unsafe extern "C" fn(*mut bfd, *mut libc::c_void) -> bool,
    >,
    pub _core_file_failing_command: Option::<
        unsafe extern "C" fn(*mut bfd) -> *mut libc::c_char,
    >,
    pub _core_file_failing_signal: Option::<
        unsafe extern "C" fn(*mut bfd) -> libc::c_int,
    >,
    pub _core_file_matches_executable_p: Option::<
        unsafe extern "C" fn(*mut bfd, *mut bfd) -> bool,
    >,
    pub _core_file_pid: Option::<unsafe extern "C" fn(*mut bfd) -> libc::c_int>,
    pub _bfd_slurp_armap: Option::<unsafe extern "C" fn(*mut bfd) -> bool>,
    pub _bfd_slurp_extended_name_table: Option::<unsafe extern "C" fn(*mut bfd) -> bool>,
    pub _bfd_construct_extended_name_table: Option::<
        unsafe extern "C" fn(
            *mut bfd,
            *mut *mut libc::c_char,
            *mut bfd_size_type,
            *mut *const libc::c_char,
        ) -> bool,
    >,
    pub _bfd_truncate_arname: Option::<
        unsafe extern "C" fn(*mut bfd, *const libc::c_char, *mut libc::c_char) -> (),
    >,
    pub write_armap: Option::<
        unsafe extern "C" fn(
            *mut bfd,
            libc::c_uint,
            *mut orl,
            libc::c_uint,
            libc::c_int,
        ) -> bool,
    >,
    pub _bfd_read_ar_hdr_fn: Option::<
        unsafe extern "C" fn(*mut bfd) -> *mut libc::c_void,
    >,
    pub _bfd_write_ar_hdr_fn: Option::<unsafe extern "C" fn(*mut bfd, *mut bfd) -> bool>,
    pub openr_next_archived_file: Option::<
        unsafe extern "C" fn(*mut bfd, *mut bfd) -> *mut bfd,
    >,
    pub _bfd_get_elt_at_index: Option::<
        unsafe extern "C" fn(*mut bfd, symindex) -> *mut bfd,
    >,
    pub _bfd_stat_arch_elt: Option::<
        unsafe extern "C" fn(*mut bfd, *mut stat) -> libc::c_int,
    >,
    pub _bfd_update_armap_timestamp: Option::<unsafe extern "C" fn(*mut bfd) -> bool>,
    pub _bfd_get_symtab_upper_bound: Option::<
        unsafe extern "C" fn(*mut bfd) -> libc::c_long,
    >,
    pub _bfd_canonicalize_symtab: Option::<
        unsafe extern "C" fn(*mut bfd, *mut *mut bfd_symbol) -> libc::c_long,
    >,
    pub _bfd_make_empty_symbol: Option::<
        unsafe extern "C" fn(*mut bfd) -> *mut bfd_symbol,
    >,
    pub _bfd_print_symbol: Option::<
        unsafe extern "C" fn(
            *mut bfd,
            *mut libc::c_void,
            *mut bfd_symbol,
            bfd_print_symbol_type,
        ) -> (),
    >,
    pub _bfd_get_symbol_info: Option::<
        unsafe extern "C" fn(*mut bfd, *mut bfd_symbol, *mut symbol_info) -> (),
    >,
    pub _bfd_get_symbol_version_string: Option::<
        unsafe extern "C" fn(
            *mut bfd,
            *mut bfd_symbol,
            bool,
            *mut bool,
        ) -> *const libc::c_char,
    >,
    pub _bfd_is_local_label_name: Option::<
        unsafe extern "C" fn(*mut bfd, *const libc::c_char) -> bool,
    >,
    pub _bfd_is_target_special_symbol: Option::<
        unsafe extern "C" fn(*mut bfd, *mut asymbol) -> bool,
    >,
    pub _get_lineno: Option::<
        unsafe extern "C" fn(*mut bfd, *mut bfd_symbol) -> *mut alent,
    >,
    pub _bfd_find_nearest_line: Option::<
        unsafe extern "C" fn(
            *mut bfd,
            *mut *mut bfd_symbol,
            *mut bfd_section,
            bfd_vma,
            *mut *const libc::c_char,
            *mut *const libc::c_char,
            *mut libc::c_uint,
            *mut libc::c_uint,
        ) -> bool,
    >,
    pub _bfd_find_line: Option::<
        unsafe extern "C" fn(
            *mut bfd,
            *mut *mut bfd_symbol,
            *mut bfd_symbol,
            *mut *const libc::c_char,
            *mut libc::c_uint,
        ) -> bool,
    >,
    pub _bfd_find_inliner_info: Option::<
        unsafe extern "C" fn(
            *mut bfd,
            *mut *const libc::c_char,
            *mut *const libc::c_char,
            *mut libc::c_uint,
        ) -> bool,
    >,
    pub _bfd_make_debug_symbol: Option::<
        unsafe extern "C" fn(*mut bfd, *mut libc::c_void, libc::c_ulong) -> *mut asymbol,
    >,
    pub _read_minisymbols: Option::<
        unsafe extern "C" fn(
            *mut bfd,
            bool,
            *mut *mut libc::c_void,
            *mut libc::c_uint,
        ) -> libc::c_long,
    >,
    pub _minisymbol_to_symbol: Option::<
        unsafe extern "C" fn(
            *mut bfd,
            bool,
            *const libc::c_void,
            *mut asymbol,
        ) -> *mut asymbol,
    >,
    pub _get_reloc_upper_bound: Option::<
        unsafe extern "C" fn(*mut bfd, sec_ptr) -> libc::c_long,
    >,
    pub _bfd_canonicalize_reloc: Option::<
        unsafe extern "C" fn(
            *mut bfd,
            sec_ptr,
            *mut *mut arelent,
            *mut *mut bfd_symbol,
        ) -> libc::c_long,
    >,
    pub _bfd_set_reloc: Option::<
        unsafe extern "C" fn(*mut bfd, sec_ptr, *mut *mut arelent, libc::c_uint) -> (),
    >,
    pub reloc_type_lookup: Option::<
        unsafe extern "C" fn(
            *mut bfd,
            bfd_reloc_code_real_type,
        ) -> *const reloc_howto_type,
    >,
    pub reloc_name_lookup: Option::<
        unsafe extern "C" fn(*mut bfd, *const libc::c_char) -> *const reloc_howto_type,
    >,
    pub _bfd_set_arch_mach: Option::<
        unsafe extern "C" fn(*mut bfd, bfd_architecture, libc::c_ulong) -> bool,
    >,
    pub _bfd_set_section_contents: Option::<
        unsafe extern "C" fn(
            *mut bfd,
            sec_ptr,
            *const libc::c_void,
            file_ptr,
            bfd_size_type,
        ) -> bool,
    >,
    pub _bfd_sizeof_headers: Option::<
        unsafe extern "C" fn(*mut bfd, *mut bfd_link_info) -> libc::c_int,
    >,
    pub _bfd_get_relocated_section_contents: Option::<
        unsafe extern "C" fn(
            *mut bfd,
            *mut bfd_link_info,
            *mut bfd_link_order,
            *mut bfd_byte,
            bool,
            *mut *mut bfd_symbol,
        ) -> *mut bfd_byte,
    >,
    pub _bfd_relax_section: Option::<
        unsafe extern "C" fn(
            *mut bfd,
            *mut bfd_section,
            *mut bfd_link_info,
            *mut bool,
        ) -> bool,
    >,
    pub _bfd_link_hash_table_create: Option::<
        unsafe extern "C" fn(*mut bfd) -> *mut bfd_link_hash_table,
    >,
    pub _bfd_link_add_symbols: Option::<
        unsafe extern "C" fn(*mut bfd, *mut bfd_link_info) -> bool,
    >,
    pub _bfd_link_just_syms: Option::<
        unsafe extern "C" fn(*mut asection, *mut bfd_link_info) -> (),
    >,
    pub _bfd_copy_link_hash_symbol_type: Option::<
        unsafe extern "C" fn(
            *mut bfd,
            *mut bfd_link_hash_entry,
            *mut bfd_link_hash_entry,
        ) -> (),
    >,
    pub _bfd_final_link: Option::<
        unsafe extern "C" fn(*mut bfd, *mut bfd_link_info) -> bool,
    >,
    pub _bfd_link_split_section: Option::<
        unsafe extern "C" fn(*mut bfd, *mut bfd_section) -> bool,
    >,
    pub _bfd_link_check_relocs: Option::<
        unsafe extern "C" fn(*mut bfd, *mut bfd_link_info) -> bool,
    >,
    pub _bfd_gc_sections: Option::<
        unsafe extern "C" fn(*mut bfd, *mut bfd_link_info) -> bool,
    >,
    pub _bfd_lookup_section_flags: Option::<
        unsafe extern "C" fn(*mut bfd_link_info, *mut flag_info, *mut asection) -> bool,
    >,
    pub _bfd_merge_sections: Option::<
        unsafe extern "C" fn(*mut bfd, *mut bfd_link_info) -> bool,
    >,
    pub _bfd_is_group_section: Option::<
        unsafe extern "C" fn(*mut bfd, *const bfd_section) -> bool,
    >,
    pub _bfd_group_name: Option::<
        unsafe extern "C" fn(*mut bfd, *const bfd_section) -> *const libc::c_char,
    >,
    pub _bfd_discard_group: Option::<
        unsafe extern "C" fn(*mut bfd, *mut bfd_section) -> bool,
    >,
    pub _section_already_linked: Option::<
        unsafe extern "C" fn(*mut bfd, *mut asection, *mut bfd_link_info) -> bool,
    >,
    pub _bfd_define_common_symbol: Option::<
        unsafe extern "C" fn(
            *mut bfd,
            *mut bfd_link_info,
            *mut bfd_link_hash_entry,
        ) -> bool,
    >,
    pub _bfd_link_hide_symbol: Option::<
        unsafe extern "C" fn(
            *mut bfd,
            *mut bfd_link_info,
            *mut bfd_link_hash_entry,
        ) -> (),
    >,
    pub _bfd_define_start_stop: Option::<
        unsafe extern "C" fn(
            *mut bfd_link_info,
            *const libc::c_char,
            *mut asection,
        ) -> *mut bfd_link_hash_entry,
    >,
    pub _bfd_get_dynamic_symtab_upper_bound: Option::<
        unsafe extern "C" fn(*mut bfd) -> libc::c_long,
    >,
    pub _bfd_canonicalize_dynamic_symtab: Option::<
        unsafe extern "C" fn(*mut bfd, *mut *mut bfd_symbol) -> libc::c_long,
    >,
    pub _bfd_get_synthetic_symtab: Option::<
        unsafe extern "C" fn(
            *mut bfd,
            libc::c_long,
            *mut *mut bfd_symbol,
            libc::c_long,
            *mut *mut bfd_symbol,
            *mut *mut bfd_symbol,
        ) -> libc::c_long,
    >,
    pub _bfd_get_dynamic_reloc_upper_bound: Option::<
        unsafe extern "C" fn(*mut bfd) -> libc::c_long,
    >,
    pub _bfd_canonicalize_dynamic_reloc: Option::<
        unsafe extern "C" fn(
            *mut bfd,
            *mut *mut arelent,
            *mut *mut bfd_symbol,
        ) -> libc::c_long,
    >,
    pub alternative_target: *const bfd_target,
    pub backend_data: *const libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct flag_info {
    pub only_with_flags: flagword,
    pub not_with_flags: flagword,
    pub flag_list: *mut flag_info_list,
    pub flags_initialized: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct flag_info_list {
    pub with: flag_type,
    pub name: *const libc::c_char,
    pub valid: bool,
    pub next: *mut flag_info_list,
}
pub type flag_type = libc::c_uint;
pub const without_flags: flag_type = 1;
pub const with_flags: flag_type = 0;
pub type sec_ptr = *mut bfd_section;
pub type symbol_info = _symbol_info;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _symbol_info {
    pub value: symvalue,
    pub type_0: libc::c_char,
    pub name: *const libc::c_char,
    pub stab_type: libc::c_uchar,
    pub stab_other: libc::c_char,
    pub stab_desc: libc::c_short,
    pub stab_name: *const libc::c_char,
}
pub type bfd_print_symbol_type = bfd_print_symbol;
pub type bfd_print_symbol = libc::c_uint;
pub const bfd_print_symbol_all: bfd_print_symbol = 2;
pub const bfd_print_symbol_more: bfd_print_symbol = 1;
pub const bfd_print_symbol_name: bfd_print_symbol = 0;
pub type symindex = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct orl {
    pub name: *mut *mut libc::c_char,
    pub u: C2RustUnnamed_39,
    pub namidx: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_39 {
    pub pos: file_ptr,
    pub abfd: *mut bfd,
}
pub type bfd_window = _bfd_window;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _bfd_window {
    pub data: *mut libc::c_void,
    pub size: bfd_size_type,
    pub i: *mut _bfd_window_internal,
}
pub type bfd_cleanup = Option::<unsafe extern "C" fn(*mut bfd) -> ()>;
pub type bfd_endian = libc::c_uint;
pub const BFD_ENDIAN_UNKNOWN: bfd_endian = 2;
pub const BFD_ENDIAN_LITTLE: bfd_endian = 1;
pub const BFD_ENDIAN_BIG: bfd_endian = 0;
pub type bfd_flavour = libc::c_uint;
pub const bfd_target_sym_flavour: bfd_flavour = 20;
pub const bfd_target_pef_xlib_flavour: bfd_flavour = 19;
pub const bfd_target_pef_flavour: bfd_flavour = 18;
pub const bfd_target_mach_o_flavour: bfd_flavour = 17;
pub const bfd_target_mmo_flavour: bfd_flavour = 16;
pub const bfd_target_evax_flavour: bfd_flavour = 15;
pub const bfd_target_ovax_flavour: bfd_flavour = 14;
pub const bfd_target_msdos_flavour: bfd_flavour = 13;
pub const bfd_target_versados_flavour: bfd_flavour = 12;
pub const bfd_target_os9k_flavour: bfd_flavour = 11;
pub const bfd_target_som_flavour: bfd_flavour = 10;
pub const bfd_target_ihex_flavour: bfd_flavour = 9;
pub const bfd_target_verilog_flavour: bfd_flavour = 8;
pub const bfd_target_srec_flavour: bfd_flavour = 7;
pub const bfd_target_tekhex_flavour: bfd_flavour = 6;
pub const bfd_target_elf_flavour: bfd_flavour = 5;
pub const bfd_target_xcoff_flavour: bfd_flavour = 4;
pub const bfd_target_ecoff_flavour: bfd_flavour = 3;
pub const bfd_target_coff_flavour: bfd_flavour = 2;
pub const bfd_target_aout_flavour: bfd_flavour = 1;
pub const bfd_target_unknown_flavour: bfd_flavour = 0;
pub type bfd_arch_info_type = bfd_arch_info;
pub type bfd_error = libc::c_uint;
pub const bfd_error_invalid_error_code: bfd_error = 22;
pub const bfd_error_on_input: bfd_error = 21;
pub const bfd_error_sorry: bfd_error = 20;
pub const bfd_error_file_too_big: bfd_error = 19;
pub const bfd_error_file_truncated: bfd_error = 18;
pub const bfd_error_bad_value: bfd_error = 17;
pub const bfd_error_no_debug_section: bfd_error = 16;
pub const bfd_error_nonrepresentable_section: bfd_error = 15;
pub const bfd_error_no_contents: bfd_error = 14;
pub const bfd_error_file_ambiguously_recognized: bfd_error = 13;
pub const bfd_error_file_not_recognized: bfd_error = 12;
pub const bfd_error_missing_dso: bfd_error = 11;
pub const bfd_error_malformed_archive: bfd_error = 10;
pub const bfd_error_no_more_archived_files: bfd_error = 9;
pub const bfd_error_no_armap: bfd_error = 8;
pub const bfd_error_no_symbols: bfd_error = 7;
pub const bfd_error_no_memory: bfd_error = 6;
pub const bfd_error_invalid_operation: bfd_error = 5;
pub const bfd_error_wrong_object_format: bfd_error = 4;
pub const bfd_error_wrong_format: bfd_error = 3;
pub const bfd_error_invalid_target: bfd_error = 2;
pub const bfd_error_system_call: bfd_error = 1;
pub const bfd_error_no_error: bfd_error = 0;
pub type bfd_error_type = bfd_error;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
pub type hashval_t = libc::c_uint;
pub type htab_hash = Option::<unsafe extern "C" fn(*const libc::c_void) -> hashval_t>;
pub type htab_eq = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
pub type htab_del = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type htab_trav = Option::<
    unsafe extern "C" fn(*mut *mut libc::c_void, *mut libc::c_void) -> libc::c_int,
>;
pub type htab_alloc = Option::<
    unsafe extern "C" fn(size_t, size_t) -> *mut libc::c_void,
>;
pub type htab_free = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type htab_alloc_with_arg = Option::<
    unsafe extern "C" fn(*mut libc::c_void, size_t, size_t) -> *mut libc::c_void,
>;
pub type htab_free_with_arg = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
>;
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
pub type htab_t = *mut htab;
pub type insert_option = libc::c_uint;
pub const INSERT: insert_option = 1;
pub const NO_INSERT: insert_option = 0;
pub type Elf_Internal_Sym = elf_internal_sym;
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
pub struct elf_internal_rela {
    pub r_offset: bfd_vma,
    pub r_info: bfd_vma,
    pub r_addend: bfd_vma,
}
pub type Elf_Internal_Rela = elf_internal_rela;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct elf_internal_dyn {
    pub d_tag: bfd_vma,
    pub d_un: C2RustUnnamed_40,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_40 {
    pub d_val: bfd_vma,
    pub d_ptr: bfd_vma,
}
pub type Elf_Internal_Dyn = elf_internal_dyn;
pub type notice_asneeded_action = libc::c_uint;
pub const notice_needed: notice_asneeded_action = 2;
pub const notice_not_needed: notice_asneeded_action = 1;
pub const notice_as_needed: notice_asneeded_action = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct elf_symbol_type {
    pub symbol: asymbol,
    pub internal_elf_sym: Elf_Internal_Sym,
    pub tc_data: C2RustUnnamed_41,
    pub version: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_41 {
    pub hppa_arg_reloc: libc::c_uint,
    pub mips_extr: *mut libc::c_void,
    pub any: *mut libc::c_void,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct eh_cie_fde {
    pub u: C2RustUnnamed_42,
    pub reloc_index: libc::c_uint,
    pub size: libc::c_uint,
    pub offset: libc::c_uint,
    pub new_offset: libc::c_uint,
    #[bitfield(name = "fde_encoding", ty = "libc::c_uint", bits = "0..=7")]
    #[bitfield(name = "lsda_encoding", ty = "libc::c_uint", bits = "8..=15")]
    #[bitfield(name = "lsda_offset", ty = "libc::c_uint", bits = "16..=23")]
    #[bitfield(name = "cie", ty = "libc::c_uint", bits = "24..=24")]
    #[bitfield(name = "removed", ty = "libc::c_uint", bits = "25..=25")]
    #[bitfield(name = "add_augmentation_size", ty = "libc::c_uint", bits = "26..=26")]
    #[bitfield(name = "make_relative", ty = "libc::c_uint", bits = "27..=27")]
    #[bitfield(name = "pad1", ty = "libc::c_uint", bits = "28..=31")]
    pub fde_encoding_lsda_encoding_lsda_offset_cie_removed_add_augmentation_size_make_relative_pad1: [u8; 4],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 4],
    pub set_loc: *mut libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_42 {
    pub fde: C2RustUnnamed_45,
    pub cie: C2RustUnnamed_43,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2RustUnnamed_43 {
    pub u: C2RustUnnamed_44,
    #[bitfield(name = "personality_offset", ty = "libc::c_uint", bits = "0..=7")]
    #[bitfield(name = "aug_str_len", ty = "libc::c_uint", bits = "8..=10")]
    #[bitfield(name = "aug_data_len", ty = "libc::c_uint", bits = "11..=15")]
    #[bitfield(name = "gc_mark", ty = "libc::c_uint", bits = "16..=16")]
    #[bitfield(name = "make_lsda_relative", ty = "libc::c_uint", bits = "17..=17")]
    #[bitfield(name = "make_per_encoding_relative", ty = "libc::c_uint", bits = "18..=18")]
    #[bitfield(name = "per_encoding_relative", ty = "libc::c_uint", bits = "19..=19")]
    #[bitfield(name = "per_encoding_aligned8", ty = "libc::c_uint", bits = "20..=20")]
    #[bitfield(name = "add_fde_encoding", ty = "libc::c_uint", bits = "21..=21")]
    #[bitfield(name = "merged", ty = "libc::c_uint", bits = "22..=22")]
    #[bitfield(name = "pad1", ty = "libc::c_uint", bits = "23..=31")]
    pub personality_offset_aug_str_len_aug_data_len_gc_mark_make_lsda_relative_make_per_encoding_relative_per_encoding_relative_per_encoding_aligned8_add_fde_encoding_merged_pad1: [u8; 4],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_44 {
    pub full_cie: *mut cie,
    pub merged_with: *mut eh_cie_fde,
    pub sec: *mut asection,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_45 {
    pub cie_inf: *mut eh_cie_fde,
    pub next_for_section: *mut eh_cie_fde,
}
pub type elf_target_os = libc::c_uint;
pub const is_nacl: elf_target_os = 3;
pub const is_vxworks: elf_target_os = 2;
pub const is_solaris: elf_target_os = 1;
pub const is_normal: elf_target_os = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct elf_size_info {
    pub sizeof_ehdr: libc::c_uchar,
    pub sizeof_phdr: libc::c_uchar,
    pub sizeof_shdr: libc::c_uchar,
    pub sizeof_rel: libc::c_uchar,
    pub sizeof_rela: libc::c_uchar,
    pub sizeof_sym: libc::c_uchar,
    pub sizeof_dyn: libc::c_uchar,
    pub sizeof_note: libc::c_uchar,
    pub sizeof_hash_entry: libc::c_uchar,
    pub int_rels_per_ext_rel: libc::c_uchar,
    pub arch_size: libc::c_uchar,
    pub log_file_align: libc::c_uchar,
    pub elfclass: libc::c_uchar,
    pub ev_current: libc::c_uchar,
    pub write_out_phdrs: Option::<
        unsafe extern "C" fn(
            *mut bfd,
            *const Elf_Internal_Phdr,
            libc::c_uint,
        ) -> libc::c_int,
    >,
    pub write_shdrs_and_ehdr: Option::<unsafe extern "C" fn(*mut bfd) -> bool>,
    pub checksum_contents: Option::<
        unsafe extern "C" fn(
            *mut bfd,
            Option::<
                unsafe extern "C" fn(
                    *const libc::c_void,
                    size_t,
                    *mut libc::c_void,
                ) -> (),
            >,
            *mut libc::c_void,
        ) -> bool,
    >,
    pub write_relocs: Option::<
        unsafe extern "C" fn(*mut bfd, *mut asection, *mut libc::c_void) -> (),
    >,
    pub swap_symbol_in: Option::<
        unsafe extern "C" fn(
            *mut bfd,
            *const libc::c_void,
            *const libc::c_void,
            *mut Elf_Internal_Sym,
        ) -> bool,
    >,
    pub swap_symbol_out: Option::<
        unsafe extern "C" fn(
            *mut bfd,
            *const Elf_Internal_Sym,
            *mut libc::c_void,
            *mut libc::c_void,
        ) -> (),
    >,
    pub slurp_reloc_table: Option::<
        unsafe extern "C" fn(*mut bfd, *mut asection, *mut *mut asymbol, bool) -> bool,
    >,
    pub slurp_symbol_table: Option::<
        unsafe extern "C" fn(*mut bfd, *mut *mut asymbol, bool) -> libc::c_long,
    >,
    pub swap_dyn_in: Option::<
        unsafe extern "C" fn(*mut bfd, *const libc::c_void, *mut Elf_Internal_Dyn) -> (),
    >,
    pub swap_dyn_out: Option::<
        unsafe extern "C" fn(*mut bfd, *const Elf_Internal_Dyn, *mut libc::c_void) -> (),
    >,
    pub swap_reloc_in: Option::<
        unsafe extern "C" fn(*mut bfd, *const bfd_byte, *mut Elf_Internal_Rela) -> (),
    >,
    pub swap_reloc_out: Option::<
        unsafe extern "C" fn(*mut bfd, *const Elf_Internal_Rela, *mut bfd_byte) -> (),
    >,
    pub swap_reloca_in: Option::<
        unsafe extern "C" fn(*mut bfd, *const bfd_byte, *mut Elf_Internal_Rela) -> (),
    >,
    pub swap_reloca_out: Option::<
        unsafe extern "C" fn(*mut bfd, *const Elf_Internal_Rela, *mut bfd_byte) -> (),
    >,
}
pub type elf_reloc_type_class = libc::c_uint;
pub const reloc_class_plt: elf_reloc_type_class = 4;
pub const reloc_class_ifunc: elf_reloc_type_class = 3;
pub const reloc_class_copy: elf_reloc_type_class = 2;
pub const reloc_class_relative: elf_reloc_type_class = 1;
pub const reloc_class_normal: elf_reloc_type_class = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct elf_reloc_cookie {
    pub rels: *mut Elf_Internal_Rela,
    pub rel: *mut Elf_Internal_Rela,
    pub relend: *mut Elf_Internal_Rela,
    pub locsyms: *mut Elf_Internal_Sym,
    pub abfd: *mut bfd,
    pub locsymcount: size_t,
    pub extsymoff: size_t,
    pub sym_hashes: *mut *mut elf_link_hash_entry,
    pub r_sym_shift: libc::c_int,
    pub bad_symtab: bool,
}
pub type irix_compat_t = libc::c_uint;
pub const ict_irix6: irix_compat_t = 2;
pub const ict_irix5: irix_compat_t = 1;
pub const ict_none: irix_compat_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bfd_elf_special_section {
    pub prefix: *const libc::c_char,
    pub prefix_length: libc::c_uint,
    pub suffix_length: libc::c_int,
    pub type_0: libc::c_uint,
    pub attr: bfd_vma,
}
pub type elf_gc_mark_hook_fn = Option::<
    unsafe extern "C" fn(
        *mut asection,
        *mut bfd_link_info,
        *mut Elf_Internal_Rela,
        *mut elf_link_hash_entry,
        *mut Elf_Internal_Sym,
    ) -> *mut asection,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bfd_elf_section_reloc_data {
    pub hdr: *mut Elf_Internal_Shdr,
    pub count: libc::c_uint,
    pub idx: libc::c_int,
    pub hashes: *mut *mut elf_link_hash_entry,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct elf_backend_data {
    pub arch: bfd_architecture,
    pub target_id: elf_target_id,
    pub target_os: elf_target_os,
    pub elf_machine_code: libc::c_int,
    pub elf_osabi: libc::c_int,
    pub maxpagesize: bfd_vma,
    pub minpagesize: bfd_vma,
    pub commonpagesize: bfd_vma,
    pub relropagesize: bfd_vma,
    pub dynamic_sec_flags: flagword,
    pub arch_data: *const libc::c_void,
    pub elf_info_to_howto: Option::<
        unsafe extern "C" fn(*mut bfd, *mut arelent, *mut Elf_Internal_Rela) -> bool,
    >,
    pub elf_info_to_howto_rel: Option::<
        unsafe extern "C" fn(*mut bfd, *mut arelent, *mut Elf_Internal_Rela) -> bool,
    >,
    pub elf_backend_sym_is_global: Option::<
        unsafe extern "C" fn(*mut bfd, *mut asymbol) -> bool,
    >,
    pub elf_backend_object_p: Option::<unsafe extern "C" fn(*mut bfd) -> bool>,
    pub elf_backend_symbol_processing: Option::<
        unsafe extern "C" fn(*mut bfd, *mut asymbol) -> (),
    >,
    pub elf_backend_symbol_table_processing: Option::<
        unsafe extern "C" fn(*mut bfd, *mut elf_symbol_type, libc::c_uint) -> bool,
    >,
    pub elf_backend_get_symbol_type: Option::<
        unsafe extern "C" fn(*mut Elf_Internal_Sym, libc::c_int) -> libc::c_int,
    >,
    pub elf_backend_archive_symbol_lookup: Option::<
        unsafe extern "C" fn(
            *mut bfd,
            *mut bfd_link_info,
            *const libc::c_char,
        ) -> *mut bfd_link_hash_entry,
    >,
    pub elf_backend_name_local_section_symbols: Option::<
        unsafe extern "C" fn(*mut bfd) -> bool,
    >,
    pub elf_backend_section_processing: Option::<
        unsafe extern "C" fn(*mut bfd, *mut Elf_Internal_Shdr) -> bool,
    >,
    pub elf_backend_section_from_shdr: Option::<
        unsafe extern "C" fn(
            *mut bfd,
            *mut Elf_Internal_Shdr,
            *const libc::c_char,
            libc::c_int,
        ) -> bool,
    >,
    pub elf_backend_section_flags: Option::<
        unsafe extern "C" fn(*const Elf_Internal_Shdr) -> bool,
    >,
    pub get_sec_type_attr: Option::<
        unsafe extern "C" fn(*mut bfd, *mut asection) -> *const bfd_elf_special_section,
    >,
    pub elf_backend_section_from_phdr: Option::<
        unsafe extern "C" fn(
            *mut bfd,
            *mut Elf_Internal_Phdr,
            libc::c_int,
            *const libc::c_char,
        ) -> bool,
    >,
    pub elf_backend_fake_sections: Option::<
        unsafe extern "C" fn(*mut bfd, *mut Elf_Internal_Shdr, *mut asection) -> bool,
    >,
    pub elf_backend_section_from_bfd_section: Option::<
        unsafe extern "C" fn(*mut bfd, *mut asection, *mut libc::c_int) -> bool,
    >,
    pub elf_add_symbol_hook: Option::<
        unsafe extern "C" fn(
            *mut bfd,
            *mut bfd_link_info,
            *mut Elf_Internal_Sym,
            *mut *const libc::c_char,
            *mut flagword,
            *mut *mut asection,
            *mut bfd_vma,
        ) -> bool,
    >,
    pub elf_backend_link_output_symbol_hook: Option::<
        unsafe extern "C" fn(
            *mut bfd_link_info,
            *const libc::c_char,
            *mut Elf_Internal_Sym,
            *mut asection,
            *mut elf_link_hash_entry,
        ) -> libc::c_int,
    >,
    pub elf_backend_create_dynamic_sections: Option::<
        unsafe extern "C" fn(*mut bfd, *mut bfd_link_info) -> bool,
    >,
    pub elf_backend_omit_section_dynsym: Option::<
        unsafe extern "C" fn(*mut bfd, *mut bfd_link_info, *mut asection) -> bool,
    >,
    pub relocs_compatible: Option::<
        unsafe extern "C" fn(*const bfd_target, *const bfd_target) -> bool,
    >,
    pub check_relocs: Option::<
        unsafe extern "C" fn(
            *mut bfd,
            *mut bfd_link_info,
            *mut asection,
            *const Elf_Internal_Rela,
        ) -> bool,
    >,
    pub check_directives: Option::<
        unsafe extern "C" fn(*mut bfd, *mut bfd_link_info) -> bool,
    >,
    pub notice_as_needed: Option::<
        unsafe extern "C" fn(
            *mut bfd,
            *mut bfd_link_info,
            notice_asneeded_action,
        ) -> bool,
    >,
    pub elf_backend_adjust_dynamic_symbol: Option::<
        unsafe extern "C" fn(*mut bfd_link_info, *mut elf_link_hash_entry) -> bool,
    >,
    pub elf_backend_always_size_sections: Option::<
        unsafe extern "C" fn(*mut bfd, *mut bfd_link_info) -> bool,
    >,
    pub elf_backend_size_dynamic_sections: Option::<
        unsafe extern "C" fn(*mut bfd, *mut bfd_link_info) -> bool,
    >,
    pub elf_backend_strip_zero_sized_dynamic_sections: Option::<
        unsafe extern "C" fn(*mut bfd_link_info) -> bool,
    >,
    pub elf_backend_init_index_section: Option::<
        unsafe extern "C" fn(*mut bfd, *mut bfd_link_info) -> (),
    >,
    pub elf_backend_relocate_section: Option::<
        unsafe extern "C" fn(
            *mut bfd,
            *mut bfd_link_info,
            *mut bfd,
            *mut asection,
            *mut bfd_byte,
            *mut Elf_Internal_Rela,
            *mut Elf_Internal_Sym,
            *mut *mut asection,
        ) -> libc::c_int,
    >,
    pub elf_backend_finish_dynamic_symbol: Option::<
        unsafe extern "C" fn(
            *mut bfd,
            *mut bfd_link_info,
            *mut elf_link_hash_entry,
            *mut Elf_Internal_Sym,
        ) -> bool,
    >,
    pub elf_backend_finish_dynamic_sections: Option::<
        unsafe extern "C" fn(*mut bfd, *mut bfd_link_info) -> bool,
    >,
    pub elf_backend_begin_write_processing: Option::<
        unsafe extern "C" fn(*mut bfd, *mut bfd_link_info) -> (),
    >,
    pub elf_backend_final_write_processing: Option::<
        unsafe extern "C" fn(*mut bfd) -> bool,
    >,
    pub elf_backend_additional_program_headers: Option::<
        unsafe extern "C" fn(*mut bfd, *mut bfd_link_info) -> libc::c_int,
    >,
    pub elf_backend_modify_segment_map: Option::<
        unsafe extern "C" fn(*mut bfd, *mut bfd_link_info) -> bool,
    >,
    pub elf_backend_modify_headers: Option::<
        unsafe extern "C" fn(*mut bfd, *mut bfd_link_info) -> bool,
    >,
    pub elf_backend_allow_non_load_phdr: Option::<
        unsafe extern "C" fn(*mut bfd, *const Elf_Internal_Phdr, libc::c_uint) -> bool,
    >,
    pub gc_keep: Option::<unsafe extern "C" fn(*mut bfd_link_info) -> ()>,
    pub gc_mark_dynamic_ref: Option::<
        unsafe extern "C" fn(*mut elf_link_hash_entry, *mut libc::c_void) -> bool,
    >,
    pub gc_mark_hook: elf_gc_mark_hook_fn,
    pub gc_mark_extra_sections: Option::<
        unsafe extern "C" fn(*mut bfd_link_info, elf_gc_mark_hook_fn) -> bool,
    >,
    pub elf_backend_init_file_header: Option::<
        unsafe extern "C" fn(*mut bfd, *mut bfd_link_info) -> bool,
    >,
    pub elf_backend_print_symbol_all: Option::<
        unsafe extern "C" fn(
            *mut bfd,
            *mut libc::c_void,
            *mut asymbol,
        ) -> *const libc::c_char,
    >,
    pub elf_backend_output_arch_local_syms: Option::<
        unsafe extern "C" fn(
            *mut bfd,
            *mut bfd_link_info,
            *mut libc::c_void,
            Option::<
                unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const libc::c_char,
                    *mut Elf_Internal_Sym,
                    *mut asection,
                    *mut elf_link_hash_entry,
                ) -> libc::c_int,
            >,
        ) -> bool,
    >,
    pub elf_backend_output_arch_syms: Option::<
        unsafe extern "C" fn(
            *mut bfd,
            *mut bfd_link_info,
            *mut libc::c_void,
            Option::<
                unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const libc::c_char,
                    *mut Elf_Internal_Sym,
                    *mut asection,
                    *mut elf_link_hash_entry,
                ) -> libc::c_int,
            >,
        ) -> bool,
    >,
    pub elf_backend_filter_implib_symbols: Option::<
        unsafe extern "C" fn(
            *mut bfd,
            *mut bfd_link_info,
            *mut *mut asymbol,
            libc::c_long,
        ) -> libc::c_uint,
    >,
    pub elf_backend_copy_indirect_symbol: Option::<
        unsafe extern "C" fn(
            *mut bfd_link_info,
            *mut elf_link_hash_entry,
            *mut elf_link_hash_entry,
        ) -> (),
    >,
    pub elf_backend_hide_symbol: Option::<
        unsafe extern "C" fn(*mut bfd_link_info, *mut elf_link_hash_entry, bool) -> (),
    >,
    pub elf_backend_fixup_symbol: Option::<
        unsafe extern "C" fn(*mut bfd_link_info, *mut elf_link_hash_entry) -> bool,
    >,
    pub elf_backend_merge_symbol_attribute: Option::<
        unsafe extern "C" fn(*mut elf_link_hash_entry, libc::c_uint, bool, bool) -> (),
    >,
    pub elf_backend_get_target_dtag: Option::<
        unsafe extern "C" fn(bfd_vma) -> *mut libc::c_char,
    >,
    pub elf_backend_ignore_undef_symbol: Option::<
        unsafe extern "C" fn(*mut elf_link_hash_entry) -> bool,
    >,
    pub elf_backend_emit_relocs: Option::<
        unsafe extern "C" fn(
            *mut bfd,
            *mut asection,
            *mut Elf_Internal_Shdr,
            *mut Elf_Internal_Rela,
            *mut *mut elf_link_hash_entry,
        ) -> bool,
    >,
    pub elf_backend_update_relocs: Option::<
        unsafe extern "C" fn(*mut asection, *mut bfd_elf_section_reloc_data) -> (),
    >,
    pub elf_backend_count_relocs: Option::<
        unsafe extern "C" fn(*mut bfd_link_info, *mut asection) -> libc::c_uint,
    >,
    pub elf_backend_count_additional_relocs: Option::<
        unsafe extern "C" fn(*mut asection) -> libc::c_uint,
    >,
    pub sort_relocs_p: Option::<unsafe extern "C" fn(*mut asection) -> bool>,
    pub elf_backend_grok_prstatus: Option::<
        unsafe extern "C" fn(*mut bfd, *mut Elf_Internal_Note) -> bool,
    >,
    pub elf_backend_grok_psinfo: Option::<
        unsafe extern "C" fn(*mut bfd, *mut Elf_Internal_Note) -> bool,
    >,
    pub elf_backend_grok_freebsd_prstatus: Option::<
        unsafe extern "C" fn(*mut bfd, *mut Elf_Internal_Note) -> bool,
    >,
    pub elf_backend_write_core_note: Option::<
        unsafe extern "C" fn(
            *mut bfd,
            *mut libc::c_char,
            *mut libc::c_int,
            libc::c_int,
            ...
        ) -> *mut libc::c_char,
    >,
    pub elf_backend_lookup_section_flags_hook: Option::<
        unsafe extern "C" fn(*mut libc::c_char) -> flagword,
    >,
    pub elf_backend_reloc_type_class: Option::<
        unsafe extern "C" fn(
            *const bfd_link_info,
            *const asection,
            *const Elf_Internal_Rela,
        ) -> elf_reloc_type_class,
    >,
    pub elf_backend_discard_info: Option::<
        unsafe extern "C" fn(*mut bfd, *mut elf_reloc_cookie, *mut bfd_link_info) -> bool,
    >,
    pub elf_backend_ignore_discarded_relocs: Option::<
        unsafe extern "C" fn(*mut asection) -> bool,
    >,
    pub action_discarded: Option::<unsafe extern "C" fn(*mut asection) -> libc::c_uint>,
    pub elf_backend_eh_frame_address_size: Option::<
        unsafe extern "C" fn(*mut bfd, *const asection) -> libc::c_uint,
    >,
    pub elf_backend_can_make_relative_eh_frame: Option::<
        unsafe extern "C" fn(*mut bfd, *mut bfd_link_info, *mut asection) -> bool,
    >,
    pub elf_backend_can_make_lsda_relative_eh_frame: Option::<
        unsafe extern "C" fn(*mut bfd, *mut bfd_link_info, *mut asection) -> bool,
    >,
    pub elf_backend_encode_eh_address: Option::<
        unsafe extern "C" fn(
            *mut bfd,
            *mut bfd_link_info,
            *mut asection,
            bfd_vma,
            *mut asection,
            bfd_vma,
            *mut bfd_vma,
        ) -> bfd_byte,
    >,
    pub elf_backend_write_section: Option::<
        unsafe extern "C" fn(
            *mut bfd,
            *mut bfd_link_info,
            *mut asection,
            *mut bfd_byte,
        ) -> bool,
    >,
    pub elf_backend_elfsym_local_is_section: Option::<
        unsafe extern "C" fn(*mut bfd) -> bool,
    >,
    pub elf_backend_mips_irix_compat: Option::<
        unsafe extern "C" fn(*mut bfd) -> irix_compat_t,
    >,
    pub elf_backend_mips_rtype_to_howto: Option::<
        unsafe extern "C" fn(*mut bfd, libc::c_uint, bool) -> *const reloc_howto_type,
    >,
    pub elf_backend_ecoff_debug_swap: *const ecoff_debug_swap,
    pub elf_backend_bfd_from_remote_memory: Option::<
        unsafe extern "C" fn(
            *mut bfd,
            bfd_vma,
            bfd_size_type,
            *mut bfd_vma,
            Option::<
                unsafe extern "C" fn(
                    bfd_vma,
                    *mut bfd_byte,
                    bfd_size_type,
                ) -> libc::c_int,
            >,
        ) -> *mut bfd,
    >,
    pub elf_backend_core_find_build_id: Option::<
        unsafe extern "C" fn(*mut bfd, bfd_vma) -> bool,
    >,
    pub plt_sym_val: Option::<
        unsafe extern "C" fn(bfd_vma, *const asection, *const arelent) -> bfd_vma,
    >,
    pub common_definition: Option::<unsafe extern "C" fn(*mut Elf_Internal_Sym) -> bool>,
    pub common_section_index: Option::<
        unsafe extern "C" fn(*mut asection) -> libc::c_uint,
    >,
    pub common_section: Option::<unsafe extern "C" fn(*mut asection) -> *mut asection>,
    pub merge_symbol: Option::<
        unsafe extern "C" fn(
            *mut elf_link_hash_entry,
            *const Elf_Internal_Sym,
            *mut *mut asection,
            bool,
            bool,
            *mut bfd,
            *const asection,
        ) -> bool,
    >,
    pub elf_hash_symbol: Option::<
        unsafe extern "C" fn(*mut elf_link_hash_entry) -> bool,
    >,
    pub record_xhash_symbol: Option::<
        unsafe extern "C" fn(*mut elf_link_hash_entry, bfd_vma) -> (),
    >,
    pub is_function_type: Option::<unsafe extern "C" fn(libc::c_uint) -> bool>,
    pub maybe_function_sym: Option::<
        unsafe extern "C" fn(
            *const asymbol,
            *mut asection,
            *mut bfd_vma,
        ) -> bfd_size_type,
    >,
    pub get_reloc_section: Option::<
        unsafe extern "C" fn(*mut bfd, *const libc::c_char) -> *mut asection,
    >,
    pub elf_backend_copy_special_section_fields: Option::<
        unsafe extern "C" fn(
            *const bfd,
            *mut bfd,
            *const Elf_Internal_Shdr,
            *mut Elf_Internal_Shdr,
        ) -> bool,
    >,
    pub link_order_error_handler: Option::<
        unsafe extern "C" fn(*const libc::c_char, ...) -> (),
    >,
    pub relplt_name: *const libc::c_char,
    pub elf_machine_alt1: libc::c_int,
    pub elf_machine_alt2: libc::c_int,
    pub s: *const elf_size_info,
    pub special_sections: *const bfd_elf_special_section,
    pub got_header_size: bfd_vma,
    pub got_elt_size: Option::<
        unsafe extern "C" fn(
            *mut bfd,
            *mut bfd_link_info,
            *mut elf_link_hash_entry,
            *mut bfd,
            libc::c_ulong,
        ) -> bfd_vma,
    >,
    pub obj_attrs_vendor: *const libc::c_char,
    pub obj_attrs_section: *const libc::c_char,
    pub obj_attrs_arg_type: Option::<unsafe extern "C" fn(libc::c_int) -> libc::c_int>,
    pub obj_attrs_section_type: libc::c_uint,
    pub obj_attrs_order: Option::<unsafe extern "C" fn(libc::c_int) -> libc::c_int>,
    pub obj_attrs_handle_unknown: Option::<
        unsafe extern "C" fn(*mut bfd, libc::c_int) -> bool,
    >,
    pub parse_gnu_properties: Option::<
        unsafe extern "C" fn(
            *mut bfd,
            libc::c_uint,
            *mut bfd_byte,
            libc::c_uint,
        ) -> elf_property_kind,
    >,
    pub merge_gnu_properties: Option::<
        unsafe extern "C" fn(
            *mut bfd_link_info,
            *mut bfd,
            *mut bfd,
            *mut elf_property,
            *mut elf_property,
        ) -> bool,
    >,
    pub setup_gnu_properties: Option::<
        unsafe extern "C" fn(*mut bfd_link_info) -> *mut bfd,
    >,
    pub fixup_gnu_properties: Option::<
        unsafe extern "C" fn(*mut bfd_link_info, *mut *mut elf_property_list) -> (),
    >,
    pub compact_eh_encoding: Option::<
        unsafe extern "C" fn(*mut bfd_link_info) -> libc::c_int,
    >,
    pub cant_unwind_opcode: Option::<
        unsafe extern "C" fn(*mut bfd_link_info) -> libc::c_int,
    >,
    pub symbol_section_index: Option::<
        unsafe extern "C" fn(*mut bfd, *mut elf_symbol_type) -> libc::c_uint,
    >,
    pub init_secondary_reloc_section: Option::<
        unsafe extern "C" fn(
            *mut bfd,
            *mut Elf_Internal_Shdr,
            *const libc::c_char,
            libc::c_uint,
        ) -> bool,
    >,
    pub slurp_secondary_relocs: Option::<
        unsafe extern "C" fn(*mut bfd, *mut asection, *mut *mut asymbol, bool) -> bool,
    >,
    pub write_secondary_relocs: Option::<
        unsafe extern "C" fn(*mut bfd, *mut asection) -> bool,
    >,
    pub static_tls_alignment: libc::c_uint,
    pub stack_align: libc::c_uint,
    pub elf_strtab_flags: libc::c_ulong,
    #[bitfield(name = "collect", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "type_change_ok", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "may_use_rel_p", ty = "libc::c_uint", bits = "2..=2")]
    #[bitfield(name = "may_use_rela_p", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "default_use_rela_p", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "rela_plts_and_copies_p", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "rela_normal", ty = "libc::c_uint", bits = "6..=6")]
    #[bitfield(name = "dtrel_excludes_plt", ty = "libc::c_uint", bits = "7..=7")]
    #[bitfield(name = "sign_extend_vma", ty = "libc::c_uint", bits = "8..=8")]
    #[bitfield(name = "want_got_plt", ty = "libc::c_uint", bits = "9..=9")]
    #[bitfield(name = "plt_readonly", ty = "libc::c_uint", bits = "10..=10")]
    #[bitfield(name = "want_plt_sym", ty = "libc::c_uint", bits = "11..=11")]
    #[bitfield(name = "plt_not_loaded", ty = "libc::c_uint", bits = "12..=12")]
    #[bitfield(name = "plt_alignment", ty = "libc::c_uint", bits = "13..=16")]
    #[bitfield(name = "can_gc_sections", ty = "libc::c_uint", bits = "17..=17")]
    #[bitfield(name = "can_refcount", ty = "libc::c_uint", bits = "18..=18")]
    #[bitfield(name = "want_got_sym", ty = "libc::c_uint", bits = "19..=19")]
    #[bitfield(name = "want_dynbss", ty = "libc::c_uint", bits = "20..=20")]
    #[bitfield(name = "want_dynrelro", ty = "libc::c_uint", bits = "21..=21")]
    #[bitfield(name = "want_p_paddr_set_to_zero", ty = "libc::c_uint", bits = "22..=22")]
    #[bitfield(name = "no_page_alias", ty = "libc::c_uint", bits = "23..=23")]
    #[bitfield(name = "default_execstack", ty = "libc::c_uint", bits = "24..=24")]
    #[bitfield(name = "caches_rawsize", ty = "libc::c_uint", bits = "25..=25")]
    #[bitfield(name = "extern_protected_data", ty = "libc::c_uint", bits = "26..=26")]
    #[bitfield(name = "always_renumber_dynsyms", ty = "libc::c_uint", bits = "27..=27")]
    #[bitfield(name = "linux_prpsinfo32_ugid16", ty = "libc::c_uint", bits = "28..=28")]
    #[bitfield(name = "linux_prpsinfo64_ugid16", ty = "libc::c_uint", bits = "29..=29")]
    pub collect_type_change_ok_may_use_rel_p_may_use_rela_p_default_use_rela_p_rela_plts_and_copies_p_rela_normal_dtrel_excludes_plt_sign_extend_vma_want_got_plt_plt_readonly_want_plt_sym_plt_not_loaded_plt_alignment_can_gc_sections_can_refcount_want_got_sym_want_dynbss_want_dynrelro_want_p_paddr_set_to_zero_no_page_alias_default_execstack_caches_rawsize_extern_protected_data_always_renumber_dynsyms_linux_prpsinfo32_ugid16_linux_prpsinfo64_ugid16: [u8; 4],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bfd_elf_section_data {
    pub this_hdr: Elf_Internal_Shdr,
    pub section_flag_info: *mut flag_info,
    pub rel: bfd_elf_section_reloc_data,
    pub rela: bfd_elf_section_reloc_data,
    pub this_idx: libc::c_int,
    pub dynindx: libc::c_int,
    pub linked_to: *mut asection,
    pub relocs: *mut Elf_Internal_Rela,
    pub local_dynrel: *mut libc::c_void,
    pub sreloc: *mut asection,
    pub group: C2RustUnnamed_46,
    pub sec_group: *mut asection,
    pub next_in_group: *mut asection,
    pub fde_list: *mut eh_cie_fde,
    pub eh_frame_entry: *mut asection,
    pub has_secondary_relocs: bool,
    pub sec_info: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_46 {
    pub name: *const libc::c_char,
    pub id: *mut bfd_symbol,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct internal_reloc {
    pub r_vaddr: bfd_vma,
    pub r_symndx: libc::c_long,
    pub r_type: libc::c_ushort,
    pub r_size: libc::c_uchar,
    pub r_extern: libc::c_uchar,
    pub r_offset: libc::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct coff_comdat_info {
    pub name: *const libc::c_char,
    pub symbol: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct coff_section_tdata {
    pub relocs: *mut internal_reloc,
    pub keep_relocs: bool,
    pub contents: *mut bfd_byte,
    pub keep_contents: bool,
    pub saved_bias: bool,
    pub bias: bfd_signed_vma,
    pub offset: bfd_vma,
    pub i: libc::c_uint,
    pub function: *const libc::c_char,
    pub comdat: *mut coff_comdat_info,
    pub line_base: libc::c_int,
    pub stab_info: *mut libc::c_void,
    pub tdata: *mut libc::c_void,
}
pub type pe_data_type = pe_tdata;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct coff_debug_merge_hash_table {
    pub root: bfd_hash_table,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct coff_link_section_info {
    pub relocs: *mut internal_reloc,
    pub rel_hashes: *mut *mut coff_link_hash_entry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct coff_final_link_info {
    pub info: *mut bfd_link_info,
    pub output_bfd: *mut bfd,
    pub failed: bool,
    pub global_to_static: bool,
    pub strtab: *mut bfd_strtab_hash,
    pub section_info: *mut coff_link_section_info,
    pub last_file_index: libc::c_long,
    pub last_file: internal_syment,
    pub last_bf_index: libc::c_long,
    pub last_bf: internal_auxent,
    pub debug_merge: coff_debug_merge_hash_table,
    pub internal_syms: *mut internal_syment,
    pub sec_ptrs: *mut *mut asection,
    pub sym_indices: *mut libc::c_long,
    pub outsyms: *mut bfd_byte,
    pub linenos: *mut bfd_byte,
    pub contents: *mut bfd_byte,
    pub external_relocs: *mut bfd_byte,
    pub internal_relocs: *mut internal_reloc,
}
pub type coff_symbol_classification = libc::c_uint;
pub const COFF_SYMBOL_PE_SECTION: coff_symbol_classification = 4;
pub const COFF_SYMBOL_LOCAL: coff_symbol_classification = 3;
pub const COFF_SYMBOL_UNDEFINED: coff_symbol_classification = 2;
pub const COFF_SYMBOL_COMMON: coff_symbol_classification = 1;
pub const COFF_SYMBOL_GLOBAL: coff_symbol_classification = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bfd_coff_backend_data {
    pub _bfd_coff_swap_aux_in: Option::<
        unsafe extern "C" fn(
            *mut bfd,
            *mut libc::c_void,
            libc::c_int,
            libc::c_int,
            libc::c_int,
            libc::c_int,
            *mut libc::c_void,
        ) -> (),
    >,
    pub _bfd_coff_swap_sym_in: Option::<
        unsafe extern "C" fn(*mut bfd, *mut libc::c_void, *mut libc::c_void) -> (),
    >,
    pub _bfd_coff_swap_lineno_in: Option::<
        unsafe extern "C" fn(*mut bfd, *mut libc::c_void, *mut libc::c_void) -> (),
    >,
    pub _bfd_coff_swap_aux_out: Option::<
        unsafe extern "C" fn(
            *mut bfd,
            *mut libc::c_void,
            libc::c_int,
            libc::c_int,
            libc::c_int,
            libc::c_int,
            *mut libc::c_void,
        ) -> libc::c_uint,
    >,
    pub _bfd_coff_swap_sym_out: Option::<
        unsafe extern "C" fn(
            *mut bfd,
            *mut libc::c_void,
            *mut libc::c_void,
        ) -> libc::c_uint,
    >,
    pub _bfd_coff_swap_lineno_out: Option::<
        unsafe extern "C" fn(
            *mut bfd,
            *mut libc::c_void,
            *mut libc::c_void,
        ) -> libc::c_uint,
    >,
    pub _bfd_coff_swap_reloc_out: Option::<
        unsafe extern "C" fn(
            *mut bfd,
            *mut libc::c_void,
            *mut libc::c_void,
        ) -> libc::c_uint,
    >,
    pub _bfd_coff_swap_filehdr_out: Option::<
        unsafe extern "C" fn(
            *mut bfd,
            *mut libc::c_void,
            *mut libc::c_void,
        ) -> libc::c_uint,
    >,
    pub _bfd_coff_swap_aouthdr_out: Option::<
        unsafe extern "C" fn(
            *mut bfd,
            *mut libc::c_void,
            *mut libc::c_void,
        ) -> libc::c_uint,
    >,
    pub _bfd_coff_swap_scnhdr_out: Option::<
        unsafe extern "C" fn(
            *mut bfd,
            *mut libc::c_void,
            *mut libc::c_void,
        ) -> libc::c_uint,
    >,
    pub _bfd_filhsz: libc::c_uint,
    pub _bfd_aoutsz: libc::c_uint,
    pub _bfd_scnhsz: libc::c_uint,
    pub _bfd_symesz: libc::c_uint,
    pub _bfd_auxesz: libc::c_uint,
    pub _bfd_relsz: libc::c_uint,
    pub _bfd_linesz: libc::c_uint,
    pub _bfd_filnmlen: libc::c_uint,
    pub _bfd_coff_long_filenames: bool,
    pub _bfd_coff_long_section_names: bool,
    pub _bfd_coff_set_long_section_names: Option::<
        unsafe extern "C" fn(*mut bfd, libc::c_int) -> bool,
    >,
    pub _bfd_coff_default_section_alignment_power: libc::c_uint,
    pub _bfd_coff_force_symnames_in_strings: bool,
    pub _bfd_coff_debug_string_prefix_length: libc::c_uint,
    pub _bfd_coff_max_nscns: libc::c_uint,
    pub _bfd_coff_swap_filehdr_in: Option::<
        unsafe extern "C" fn(*mut bfd, *mut libc::c_void, *mut libc::c_void) -> (),
    >,
    pub _bfd_coff_swap_aouthdr_in: Option::<
        unsafe extern "C" fn(*mut bfd, *mut libc::c_void, *mut libc::c_void) -> (),
    >,
    pub _bfd_coff_swap_scnhdr_in: Option::<
        unsafe extern "C" fn(*mut bfd, *mut libc::c_void, *mut libc::c_void) -> (),
    >,
    pub _bfd_coff_swap_reloc_in: Option::<
        unsafe extern "C" fn(*mut bfd, *mut libc::c_void, *mut libc::c_void) -> (),
    >,
    pub _bfd_coff_bad_format_hook: Option::<
        unsafe extern "C" fn(*mut bfd, *mut libc::c_void) -> bool,
    >,
    pub _bfd_coff_set_arch_mach_hook: Option::<
        unsafe extern "C" fn(*mut bfd, *mut libc::c_void) -> bool,
    >,
    pub _bfd_coff_mkobject_hook: Option::<
        unsafe extern "C" fn(
            *mut bfd,
            *mut libc::c_void,
            *mut libc::c_void,
        ) -> *mut libc::c_void,
    >,
    pub _bfd_styp_to_sec_flags_hook: Option::<
        unsafe extern "C" fn(
            *mut bfd,
            *mut libc::c_void,
            *const libc::c_char,
            *mut asection,
            *mut flagword,
        ) -> bool,
    >,
    pub _bfd_set_alignment_hook: Option::<
        unsafe extern "C" fn(*mut bfd, *mut asection, *mut libc::c_void) -> (),
    >,
    pub _bfd_coff_slurp_symbol_table: Option::<unsafe extern "C" fn(*mut bfd) -> bool>,
    pub _bfd_coff_symname_in_debug: Option::<
        unsafe extern "C" fn(*mut bfd, *mut internal_syment) -> bool,
    >,
    pub _bfd_coff_pointerize_aux_hook: Option::<
        unsafe extern "C" fn(
            *mut bfd,
            *mut combined_entry_type,
            *mut combined_entry_type,
            libc::c_uint,
            *mut combined_entry_type,
        ) -> bool,
    >,
    pub _bfd_coff_print_aux: Option::<
        unsafe extern "C" fn(
            *mut bfd,
            *mut FILE,
            *mut combined_entry_type,
            *mut combined_entry_type,
            *mut combined_entry_type,
            libc::c_uint,
        ) -> bool,
    >,
    pub _bfd_coff_reloc16_extra_cases: Option::<
        unsafe extern "C" fn(
            *mut bfd,
            *mut bfd_link_info,
            *mut bfd_link_order,
            *mut arelent,
            *mut bfd_byte,
            *mut libc::c_uint,
            *mut libc::c_uint,
        ) -> (),
    >,
    pub _bfd_coff_reloc16_estimate: Option::<
        unsafe extern "C" fn(
            *mut bfd,
            *mut asection,
            *mut arelent,
            libc::c_uint,
            *mut bfd_link_info,
        ) -> libc::c_int,
    >,
    pub _bfd_coff_classify_symbol: Option::<
        unsafe extern "C" fn(
            *mut bfd,
            *mut internal_syment,
        ) -> coff_symbol_classification,
    >,
    pub _bfd_coff_compute_section_file_positions: Option::<
        unsafe extern "C" fn(*mut bfd) -> bool,
    >,
    pub _bfd_coff_start_final_link: Option::<
        unsafe extern "C" fn(*mut bfd, *mut bfd_link_info) -> bool,
    >,
    pub _bfd_coff_relocate_section: Option::<
        unsafe extern "C" fn(
            *mut bfd,
            *mut bfd_link_info,
            *mut bfd,
            *mut asection,
            *mut bfd_byte,
            *mut internal_reloc,
            *mut internal_syment,
            *mut *mut asection,
        ) -> bool,
    >,
    pub _bfd_coff_rtype_to_howto: Option::<
        unsafe extern "C" fn(
            *mut bfd,
            *mut asection,
            *mut internal_reloc,
            *mut coff_link_hash_entry,
            *mut internal_syment,
            *mut bfd_vma,
        ) -> *const reloc_howto_type,
    >,
    pub _bfd_coff_adjust_symndx: Option::<
        unsafe extern "C" fn(
            *mut bfd,
            *mut bfd_link_info,
            *mut bfd,
            *mut asection,
            *mut internal_reloc,
            *mut bool,
        ) -> bool,
    >,
    pub _bfd_coff_link_add_one_symbol: Option::<
        unsafe extern "C" fn(
            *mut bfd_link_info,
            *mut bfd,
            *const libc::c_char,
            flagword,
            *mut asection,
            bfd_vma,
            *const libc::c_char,
            bool,
            bool,
            *mut *mut bfd_link_hash_entry,
        ) -> bool,
    >,
    pub _bfd_coff_link_output_has_begun: Option::<
        unsafe extern "C" fn(*mut bfd, *mut coff_final_link_info) -> bool,
    >,
    pub _bfd_coff_final_link_postscript: Option::<
        unsafe extern "C" fn(*mut bfd, *mut coff_final_link_info) -> bool,
    >,
    pub _bfd_coff_print_pdata: Option::<
        unsafe extern "C" fn(*mut bfd, *mut libc::c_void) -> bool,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct is_specified_symbol_predicate_data {
    pub name: *const libc::c_char,
    pub found: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct redefine_node {
    pub source: *mut libc::c_char,
    pub target: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct addsym_node {
    pub next: *mut addsym_node,
    pub symdef: *mut libc::c_char,
    pub symval: libc::c_long,
    pub flags: flagword,
    pub section: *mut libc::c_char,
    pub othersym: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct section_rename {
    pub old_name: *const libc::c_char,
    pub new_name: *const libc::c_char,
    pub flags: flagword,
    pub next: *mut section_rename,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct merged_note_section {
    pub sec: *mut asection,
    pub contents: *mut bfd_byte,
    pub size: bfd_size_type,
    pub next: *mut merged_note_section,
}
pub type strip_action = libc::c_uint;
pub const STRIP_ALL: strip_action = 7;
pub const STRIP_NONDWO: strip_action = 6;
pub const STRIP_DWO: strip_action = 5;
pub const STRIP_NONDEBUG: strip_action = 4;
pub const STRIP_UNNEEDED: strip_action = 3;
pub const STRIP_DEBUG: strip_action = 2;
pub const STRIP_NONE: strip_action = 1;
pub const STRIP_UNDEF: strip_action = 0;
pub type locals_action = libc::c_uint;
pub const LOCALS_ALL: locals_action = 2;
pub const LOCALS_START_L: locals_action = 1;
pub const LOCALS_UNDEF: locals_action = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct section_list {
    pub next: *mut section_list,
    pub pattern: *const libc::c_char,
    pub used: bool,
    pub context: libc::c_uint,
    pub vma_val: bfd_vma,
    pub lma_val: bfd_vma,
    pub flags: flagword,
    pub alignment: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct section_add {
    pub next: *mut section_add,
    pub name: *const libc::c_char,
    pub filename: *const libc::c_char,
    pub size: size_t,
    pub contents: *mut bfd_byte,
    pub section: *mut asection,
}
pub type C2RustUnnamed_47 = libc::c_uint;
pub const decompress: C2RustUnnamed_47 = 16;
pub const compress_gabi_zlib: C2RustUnnamed_47 = 9;
pub const compress_gnu_zlib: C2RustUnnamed_47 = 5;
pub const compress_zlib: C2RustUnnamed_47 = 3;
pub const compress: C2RustUnnamed_47 = 1;
pub const nothing: C2RustUnnamed_47 = 0;
pub type long_section_name_handling = libc::c_uint;
pub const KEEP: long_section_name_handling = 2;
pub const ENABLE: long_section_name_handling = 1;
pub const DISABLE: long_section_name_handling = 0;
pub type command_line_switch = libc::c_uint;
pub const OPTION_WRITABLE_TEXT: command_line_switch = 218;
pub const OPTION_WEAKEN_SYMBOLS: command_line_switch = 217;
pub const OPTION_WEAKEN: command_line_switch = 216;
pub const OPTION_VERILOG_DATA_WIDTH: command_line_switch = 215;
pub const OPTION_UPDATE_SECTION: command_line_switch = 214;
pub const OPTION_SUBSYSTEM: command_line_switch = 213;
pub const OPTION_STRIP_UNNEEDED_SYMBOLS: command_line_switch = 212;
pub const OPTION_STRIP_UNNEEDED_SYMBOL: command_line_switch = 211;
pub const OPTION_STRIP_UNNEEDED: command_line_switch = 210;
pub const OPTION_STRIP_SYMBOLS: command_line_switch = 209;
pub const OPTION_STRIP_DWO: command_line_switch = 208;
pub const OPTION_STACK: command_line_switch = 207;
pub const OPTION_SREC_LEN: command_line_switch = 206;
pub const OPTION_SREC_FORCES3: command_line_switch = 205;
pub const OPTION_SET_START: command_line_switch = 204;
pub const OPTION_SET_SECTION_ALIGNMENT: command_line_switch = 203;
pub const OPTION_SET_SECTION_FLAGS: command_line_switch = 202;
pub const OPTION_PE_SECTION_ALIGNMENT: command_line_switch = 201;
pub const OPTION_REVERSE_BYTES: command_line_switch = 200;
pub const OPTION_RENAME_SECTION: command_line_switch = 199;
pub const OPTION_REMOVE_RELOCS: command_line_switch = 198;
pub const OPTION_REMOVE_LEADING_CHAR: command_line_switch = 197;
pub const OPTION_REDEFINE_SYMS: command_line_switch = 196;
pub const OPTION_REDEFINE_SYM: command_line_switch = 195;
pub const OPTION_READONLY_TEXT: command_line_switch = 194;
pub const OPTION_PURE: command_line_switch = 193;
pub const OPTION_PREFIX_SYMBOLS: command_line_switch = 192;
pub const OPTION_PREFIX_SECTIONS: command_line_switch = 191;
pub const OPTION_PREFIX_ALLOC_SECTIONS: command_line_switch = 190;
pub const OPTION_PAD_TO: command_line_switch = 189;
pub const OPTION_ONLY_KEEP_DEBUG: command_line_switch = 188;
pub const OPTION_NO_CHANGE_WARNINGS: command_line_switch = 187;
pub const OPTION_NO_MERGE_NOTES: command_line_switch = 186;
pub const OPTION_MERGE_NOTES: command_line_switch = 185;
pub const OPTION_LONG_SECTION_NAMES: command_line_switch = 184;
pub const OPTION_LOCALIZE_SYMBOLS: command_line_switch = 183;
pub const OPTION_LOCALIZE_HIDDEN: command_line_switch = 182;
pub const OPTION_KEEP_SECTION_SYMBOLS: command_line_switch = 181;
pub const OPTION_KEEP_SYMBOLS: command_line_switch = 180;
pub const OPTION_KEEP_SECTION: command_line_switch = 179;
pub const OPTION_KEEP_FILE_SYMBOLS: command_line_switch = 178;
pub const OPTION_KEEPGLOBAL_SYMBOLS: command_line_switch = 177;
pub const OPTION_INTERLEAVE_WIDTH: command_line_switch = 176;
pub const OPTION_IMPURE: command_line_switch = 175;
pub const OPTION_IMAGE_BASE: command_line_switch = 174;
pub const OPTION_HEAP: command_line_switch = 173;
pub const OPTION_GLOBALIZE_SYMBOLS: command_line_switch = 172;
pub const OPTION_GLOBALIZE_SYMBOL: command_line_switch = 171;
pub const OPTION_GAP_FILL: command_line_switch = 170;
pub const OPTION_FORMATS_INFO: command_line_switch = 169;
pub const OPTION_FILE_ALIGNMENT: command_line_switch = 168;
pub const OPTION_EXTRACT_SYMBOL: command_line_switch = 167;
pub const OPTION_EXTRACT_DWO: command_line_switch = 166;
pub const OPTION_ELF_STT_COMMON: command_line_switch = 165;
pub const OPTION_DUMP_SECTION: command_line_switch = 164;
pub const OPTION_DECOMPRESS_DEBUG_SECTIONS: command_line_switch = 163;
pub const OPTION_DEBUGGING: command_line_switch = 162;
pub const OPTION_COMPRESS_DEBUG_SECTIONS: command_line_switch = 161;
pub const OPTION_CHANGE_WARNINGS: command_line_switch = 160;
pub const OPTION_CHANGE_START: command_line_switch = 159;
pub const OPTION_CHANGE_SECTION_VMA: command_line_switch = 158;
pub const OPTION_CHANGE_SECTION_LMA: command_line_switch = 157;
pub const OPTION_CHANGE_SECTION_ADDRESS: command_line_switch = 156;
pub const OPTION_CHANGE_LEADING_CHAR: command_line_switch = 155;
pub const OPTION_CHANGE_ADDRESSES: command_line_switch = 154;
pub const OPTION_ALT_MACH_CODE: command_line_switch = 153;
pub const OPTION_ADD_SYMBOL: command_line_switch = 152;
pub const OPTION_ADD_GNU_DEBUGLINK: command_line_switch = 151;
pub const OPTION_ADD_SECTION: command_line_switch = 150;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct objcopy_internal_note {
    pub note: Elf_Internal_Note,
    pub padded_namesz: libc::c_ulong,
    pub start: bfd_vma,
    pub end: bfd_vma,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct name_list {
    pub next: *mut name_list,
    pub name: *const libc::c_char,
    pub obfd: *mut bfd,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_48 {
    pub name: *const libc::c_char,
    pub set_def: libc::c_char,
    pub value: libc::c_short,
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
#[inline]
unsafe extern "C" fn startswith(
    mut str: *const libc::c_char,
    mut prefix: *const libc::c_char,
) -> bool {
    return strncmp(str, prefix, strlen(prefix)) == 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn bfd_section_name(mut sec: *const asection) -> *const libc::c_char {
    return (*sec).name;
}
#[inline]
unsafe extern "C" fn bfd_section_size(mut sec: *const asection) -> bfd_size_type {
    return (*sec).size;
}
#[inline]
unsafe extern "C" fn bfd_section_vma(mut sec: *const asection) -> bfd_vma {
    return (*sec).vma;
}
#[inline]
unsafe extern "C" fn bfd_section_lma(mut sec: *const asection) -> bfd_vma {
    return (*sec).lma;
}
#[inline]
unsafe extern "C" fn bfd_section_alignment(mut sec: *const asection) -> libc::c_uint {
    return (*sec).alignment_power;
}
#[inline]
unsafe extern "C" fn bfd_section_flags(mut sec: *const asection) -> flagword {
    return (*sec).flags;
}
#[inline]
unsafe extern "C" fn bfd_is_com_section(mut sec: *const asection) -> bool {
    return (*sec).flags & 0x1000 as libc::c_int as libc::c_uint
        != 0 as libc::c_int as libc::c_uint;
}
#[inline]
unsafe extern "C" fn bfd_set_section_vma(
    mut sec: *mut asection,
    mut val: bfd_vma,
) -> bool {
    (*sec).lma = val;
    (*sec).vma = (*sec).lma;
    (*sec).set_user_set_vma(1 as libc::c_int as libc::c_uint);
    return 1 as libc::c_int != 0;
}
#[inline]
unsafe extern "C" fn bfd_set_section_alignment(
    mut sec: *mut asection,
    mut val: libc::c_uint,
) -> bool {
    (*sec).alignment_power = val;
    return 1 as libc::c_int != 0;
}
#[inline]
unsafe extern "C" fn bfd_is_und_section(mut sec: *const asection) -> bool {
    return sec
        == &mut *_bfd_std_section.as_mut_ptr().offset(1 as libc::c_int as isize)
            as *mut asection as *const asection;
}
#[inline]
unsafe extern "C" fn bfd_get_filename(mut abfd: *const bfd) -> *const libc::c_char {
    return (*abfd).filename;
}
#[inline]
unsafe extern "C" fn bfd_get_format(mut abfd: *const bfd) -> bfd_format {
    return (*abfd).format();
}
#[inline]
unsafe extern "C" fn bfd_get_file_flags(mut abfd: *const bfd) -> flagword {
    return (*abfd).flags;
}
#[inline]
unsafe extern "C" fn bfd_get_start_address(mut abfd: *const bfd) -> bfd_vma {
    return (*abfd).start_address;
}
#[inline]
unsafe extern "C" fn bfd_count_sections(mut abfd: *const bfd) -> libc::c_uint {
    return (*abfd).section_count;
}
#[inline]
unsafe extern "C" fn bfd_asymbol_section(mut sy: *const asymbol) -> *mut asection {
    return (*sy).section;
}
#[inline]
unsafe extern "C" fn bfd_asymbol_name(mut sy: *const asymbol) -> *const libc::c_char {
    return (*sy).name;
}
#[inline]
unsafe extern "C" fn bfd_set_asymbol_name(
    mut sy: *mut asymbol,
    mut name: *const libc::c_char,
) {
    (*sy).name = name;
}
#[inline]
unsafe extern "C" fn bfd_get_target(mut abfd: *const bfd) -> *const libc::c_char {
    return (*(*abfd).xvec).name;
}
#[inline]
unsafe extern "C" fn bfd_get_flavour(mut abfd: *const bfd) -> bfd_flavour {
    return (*(*abfd).xvec).flavour;
}
#[inline]
unsafe extern "C" fn bfd_applicable_file_flags(mut abfd: *const bfd) -> flagword {
    return (*(*abfd).xvec).object_flags;
}
#[inline]
unsafe extern "C" fn bfd_applicable_section_flags(mut abfd: *const bfd) -> flagword {
    return (*(*abfd).xvec).section_flags;
}
#[inline]
unsafe extern "C" fn bfd_get_symbol_leading_char(mut abfd: *const bfd) -> libc::c_char {
    return (*(*abfd).xvec).symbol_leading_char;
}
#[inline]
unsafe extern "C" fn bfd_keep_unused_section_symbols(mut abfd: *const bfd) -> bool {
    return (*(*abfd).xvec).keep_unused_section_symbols;
}
static mut pe_file_alignment: bfd_vma = -(1 as libc::c_int) as bfd_vma;
static mut pe_heap_commit: bfd_vma = -(1 as libc::c_int) as bfd_vma;
static mut pe_heap_reserve: bfd_vma = -(1 as libc::c_int) as bfd_vma;
static mut pe_image_base: bfd_vma = -(1 as libc::c_int) as bfd_vma;
static mut pe_section_alignment: bfd_vma = -(1 as libc::c_int) as bfd_vma;
static mut pe_stack_commit: bfd_vma = -(1 as libc::c_int) as bfd_vma;
static mut pe_stack_reserve: bfd_vma = -(1 as libc::c_int) as bfd_vma;
static mut pe_subsystem: libc::c_short = -(1 as libc::c_int) as libc::c_short;
static mut pe_major_subsystem_version: libc::c_short = -(1 as libc::c_int)
    as libc::c_short;
static mut pe_minor_subsystem_version: libc::c_short = -(1 as libc::c_int)
    as libc::c_short;
static mut section_rename_list: *mut section_rename = 0 as *const section_rename
    as *mut section_rename;
static mut isympp: *mut *mut asymbol = 0 as *const *mut asymbol as *mut *mut asymbol;
static mut osympp: *mut *mut asymbol = 0 as *const *mut asymbol as *mut *mut asymbol;
static mut copy_byte: libc::c_int = -(1 as libc::c_int);
static mut interleave: libc::c_int = 0 as libc::c_int;
static mut copy_width: libc::c_int = 1 as libc::c_int;
static mut keep_section_symbols: bool = 0 as libc::c_int != 0;
static mut verbose: bool = false;
static mut preserve_dates: bool = false;
static mut deterministic: libc::c_int = -(1 as libc::c_int);
static mut status: libc::c_int = 0 as libc::c_int;
static mut merge_notes: bool = 0 as libc::c_int != 0;
static mut strip_symbols: strip_action = STRIP_UNDEF;
static mut discard_locals: locals_action = LOCALS_UNDEF;
static mut change_sections: *mut section_list = 0 as *const section_list
    as *mut section_list;
static mut sections_removed: bool = false;
static mut sections_copied: bool = false;
static mut change_start: bfd_vma = 0 as libc::c_int as bfd_vma;
static mut set_start_set: bool = 0 as libc::c_int != 0;
static mut set_start: bfd_vma = 0;
static mut change_section_address: bfd_vma = 0 as libc::c_int as bfd_vma;
static mut gap_fill_set: bool = 0 as libc::c_int != 0;
static mut gap_fill: bfd_byte = 0 as libc::c_int as bfd_byte;
static mut pad_to_set: bool = 0 as libc::c_int != 0;
static mut pad_to: bfd_vma = 0;
static mut use_alt_mach_code: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
static mut bfd_flags_to_set: flagword = 0;
static mut bfd_flags_to_clear: flagword = 0;
static mut add_sections: *mut section_add = 0 as *const section_add as *mut section_add;
static mut update_sections: *mut section_add = 0 as *const section_add
    as *mut section_add;
static mut dump_sections: *mut section_add = 0 as *const section_add as *mut section_add;
static mut gnu_debuglink_filename: *const libc::c_char = 0 as *const libc::c_char;
static mut convert_debugging: bool = 0 as libc::c_int != 0;
static mut do_debug_sections: C2RustUnnamed_47 = nothing;
static mut do_elf_stt_common: bfd_link_elf_stt_common = unchanged;
static mut change_leading_char: bool = 0 as libc::c_int != 0;
static mut remove_leading_char: bool = 0 as libc::c_int != 0;
static mut wildcard: bool = 0 as libc::c_int != 0;
static mut localize_hidden: bool = 0 as libc::c_int != 0;
static mut strip_specific_htab: htab_t = 0 as *const htab as htab_t;
static mut strip_unneeded_htab: htab_t = 0 as *const htab as htab_t;
static mut keep_specific_htab: htab_t = 0 as *const htab as htab_t;
static mut localize_specific_htab: htab_t = 0 as *const htab as htab_t;
static mut globalize_specific_htab: htab_t = 0 as *const htab as htab_t;
static mut keepglobal_specific_htab: htab_t = 0 as *const htab as htab_t;
static mut weaken_specific_htab: htab_t = 0 as *const htab as htab_t;
static mut redefine_specific_htab: htab_t = 0 as *const htab as htab_t;
static mut redefine_specific_reverse_htab: htab_t = 0 as *const htab as htab_t;
static mut add_sym_list: *mut addsym_node = 0 as *const addsym_node as *mut addsym_node;
static mut add_sym_tail: *mut *mut addsym_node = unsafe {
    &add_sym_list as *const *mut addsym_node as *mut *mut addsym_node
};
static mut add_symbols: libc::c_int = 0 as libc::c_int;
static mut strip_specific_buffer: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
static mut strip_unneeded_buffer: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
static mut keep_specific_buffer: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
static mut localize_specific_buffer: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
static mut globalize_specific_buffer: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
static mut keepglobal_specific_buffer: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
static mut weaken_specific_buffer: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
static mut weaken: bool = 0 as libc::c_int != 0;
static mut keep_file_symbols: bool = 0 as libc::c_int != 0;
static mut prefix_symbols_string: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
static mut prefix_sections_string: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
static mut prefix_alloc_sections_string: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
static mut extract_symbol: bool = 0 as libc::c_int != 0;
static mut reverse_bytes: libc::c_int = 0 as libc::c_int;
static mut long_section_names: long_section_name_handling = KEEP;
static mut strip_options: [option; 32] = [
    {
        let mut init = option {
            name: b"disable-deterministic-archives\0" as *const u8
                as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'U' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"discard-all\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'x' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"discard-locals\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'X' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"enable-deterministic-archives\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'D' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"format\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'F' as i32,
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
            name: b"info\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPTION_FORMATS_INFO as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"input-format\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'I' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"input-target\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'I' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"keep-section-symbols\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPTION_KEEP_SECTION_SYMBOLS as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"keep-file-symbols\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPTION_KEEP_FILE_SYMBOLS as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"keep-section\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPTION_KEEP_SECTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"keep-symbol\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'K' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"merge-notes\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'M' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"no-merge-notes\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPTION_NO_MERGE_NOTES as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"only-keep-debug\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPTION_ONLY_KEEP_DEBUG as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"output-file\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'o' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"output-format\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'O' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"output-target\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'O' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"preserve-dates\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'p' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"remove-section\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'R' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"remove-relocations\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPTION_REMOVE_RELOCS as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"strip-all\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 's' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"strip-debug\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'S' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"strip-dwo\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPTION_STRIP_DWO as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"strip-symbol\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'N' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"strip-unneeded\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPTION_STRIP_UNNEEDED as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"target\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'F' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"verbose\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'v' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"version\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'V' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"wildcard\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'w' as i32,
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
static mut copy_options: [option; 102] = [
    {
        let mut init = option {
            name: b"add-gnu-debuglink\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPTION_ADD_GNU_DEBUGLINK as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"add-section\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPTION_ADD_SECTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"add-symbol\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPTION_ADD_SYMBOL as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"adjust-section-vma\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPTION_CHANGE_SECTION_ADDRESS as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"adjust-start\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPTION_CHANGE_START as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"adjust-vma\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPTION_CHANGE_ADDRESSES as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"adjust-warnings\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPTION_CHANGE_WARNINGS as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"alt-machine-code\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPTION_ALT_MACH_CODE as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"binary-architecture\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'B' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"byte\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'b' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"change-addresses\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPTION_CHANGE_ADDRESSES as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"change-leading-char\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPTION_CHANGE_LEADING_CHAR as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"change-section-address\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPTION_CHANGE_SECTION_ADDRESS as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"change-section-lma\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPTION_CHANGE_SECTION_LMA as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"change-section-vma\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPTION_CHANGE_SECTION_VMA as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"change-start\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPTION_CHANGE_START as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"change-warnings\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPTION_CHANGE_WARNINGS as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"compress-debug-sections\0" as *const u8 as *const libc::c_char,
            has_arg: 2 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPTION_COMPRESS_DEBUG_SECTIONS as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"debugging\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPTION_DEBUGGING as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"decompress-debug-sections\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPTION_DECOMPRESS_DEBUG_SECTIONS as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"disable-deterministic-archives\0" as *const u8
                as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'U' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"discard-all\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'x' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"discard-locals\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'X' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"dump-section\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPTION_DUMP_SECTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"elf-stt-common\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPTION_ELF_STT_COMMON as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"enable-deterministic-archives\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'D' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"extract-dwo\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPTION_EXTRACT_DWO as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"extract-symbol\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPTION_EXTRACT_SYMBOL as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"file-alignment\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPTION_FILE_ALIGNMENT as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"format\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'F' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"gap-fill\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPTION_GAP_FILL as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"globalize-symbol\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPTION_GLOBALIZE_SYMBOL as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"globalize-symbols\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPTION_GLOBALIZE_SYMBOLS as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"heap\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPTION_HEAP as libc::c_int,
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
            name: b"image-base\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPTION_IMAGE_BASE as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"impure\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPTION_IMPURE as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"info\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPTION_FORMATS_INFO as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"input-format\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'I' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"input-target\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'I' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"interleave\0" as *const u8 as *const libc::c_char,
            has_arg: 2 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'i' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"interleave-width\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPTION_INTERLEAVE_WIDTH as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"keep-file-symbols\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPTION_KEEP_FILE_SYMBOLS as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"keep-global-symbol\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'G' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"keep-global-symbols\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPTION_KEEPGLOBAL_SYMBOLS as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"keep-section\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPTION_KEEP_SECTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"keep-symbol\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'K' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"keep-symbols\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPTION_KEEP_SYMBOLS as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"keep-section-symbols\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPTION_KEEP_SECTION_SYMBOLS as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"localize-hidden\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPTION_LOCALIZE_HIDDEN as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"localize-symbol\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'L' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"localize-symbols\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPTION_LOCALIZE_SYMBOLS as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"long-section-names\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPTION_LONG_SECTION_NAMES as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"merge-notes\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'M' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"no-merge-notes\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPTION_NO_MERGE_NOTES as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"no-adjust-warnings\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPTION_NO_CHANGE_WARNINGS as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"no-change-warnings\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPTION_NO_CHANGE_WARNINGS as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"only-keep-debug\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPTION_ONLY_KEEP_DEBUG as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"only-section\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'j' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"output-format\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'O' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"output-target\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'O' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"pad-to\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPTION_PAD_TO as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"prefix-alloc-sections\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPTION_PREFIX_ALLOC_SECTIONS as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"prefix-sections\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPTION_PREFIX_SECTIONS as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"prefix-symbols\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPTION_PREFIX_SYMBOLS as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"preserve-dates\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'p' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"pure\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPTION_PURE as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"readonly-text\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPTION_READONLY_TEXT as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"redefine-sym\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPTION_REDEFINE_SYM as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"redefine-syms\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPTION_REDEFINE_SYMS as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"remove-leading-char\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPTION_REMOVE_LEADING_CHAR as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"remove-section\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'R' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"remove-relocations\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPTION_REMOVE_RELOCS as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"rename-section\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPTION_RENAME_SECTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"reverse-bytes\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPTION_REVERSE_BYTES as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"section-alignment\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPTION_PE_SECTION_ALIGNMENT as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"set-section-flags\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPTION_SET_SECTION_FLAGS as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"set-section-alignment\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPTION_SET_SECTION_ALIGNMENT as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"set-start\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPTION_SET_START as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"srec-forceS3\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPTION_SREC_FORCES3 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"srec-len\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPTION_SREC_LEN as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"stack\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPTION_STACK as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"strip-all\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'S' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"strip-debug\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'g' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"strip-dwo\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPTION_STRIP_DWO as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"strip-symbol\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'N' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"strip-symbols\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPTION_STRIP_SYMBOLS as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"strip-unneeded\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPTION_STRIP_UNNEEDED as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"strip-unneeded-symbol\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPTION_STRIP_UNNEEDED_SYMBOL as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"strip-unneeded-symbols\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPTION_STRIP_UNNEEDED_SYMBOLS as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"subsystem\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPTION_SUBSYSTEM as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"target\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'F' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"update-section\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPTION_UPDATE_SECTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"verbose\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'v' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"verilog-data-width\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPTION_VERILOG_DATA_WIDTH as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"version\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'V' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"weaken\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPTION_WEAKEN as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"weaken-symbol\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'W' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"weaken-symbols\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPTION_WEAKEN_SYMBOLS as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"wildcard\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'w' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"writable-text\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPTION_WRITABLE_TEXT as libc::c_int,
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
unsafe extern "C" fn setup_section(
    mut ibfd: *mut bfd,
    mut isection: sec_ptr,
    mut obfdarg: *mut libc::c_void,
) {
    let mut obfd: *mut bfd = obfdarg as *mut bfd;
    let mut p: *mut section_list = 0 as *mut section_list;
    let mut osection: sec_ptr = 0 as *mut bfd_section;
    let mut size: bfd_size_type = 0;
    let mut vma: bfd_vma = 0;
    let mut lma: bfd_vma = 0;
    let mut flags: flagword = 0;
    let mut err: *const libc::c_char = 0 as *const libc::c_char;
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut new_name: *const libc::c_char = 0 as *const libc::c_char;
    let mut prefix: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut make_nobits: bool = false;
    let mut alignment: libc::c_uint = 0;
    if is_strip_section(ibfd, isection) {
        return;
    }
    name = bfd_section_name(isection as *const asection);
    flags = bfd_section_flags(isection as *const asection);
    if bfd_get_flavour(ibfd) as libc::c_uint != bfd_get_flavour(obfd) as libc::c_uint {
        flags &= bfd_applicable_section_flags(ibfd);
        flags &= bfd_applicable_section_flags(obfd);
    }
    new_name = find_section_rename(name, &mut flags);
    if new_name != name {
        name = new_name;
        flags = check_new_section_flags(flags, obfd, name);
    }
    if !prefix_alloc_sections_string.is_null()
        && bfd_section_flags(isection as *const asection)
            & 0x1 as libc::c_int as libc::c_uint != 0 as libc::c_int as libc::c_uint
    {
        prefix = prefix_alloc_sections_string;
    } else if !prefix_sections_string.is_null() {
        prefix = prefix_sections_string;
    }
    if !prefix.is_null() {
        let mut n: *mut libc::c_char = 0 as *mut libc::c_char;
        n = xmalloc(
            (strlen(prefix))
                .wrapping_add(strlen(name))
                .wrapping_add(1 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
        strcpy(n, prefix);
        strcat(n, name);
        name = n;
    }
    make_nobits = 0 as libc::c_int != 0;
    p = find_section_list(
        bfd_section_name(isection as *const asection),
        0 as libc::c_int != 0,
        ((1 as libc::c_int) << 7 as libc::c_int) as libc::c_uint,
    );
    if !p.is_null() {
        flags = (*p).flags
            | flags & (0x100 as libc::c_int | 0x4 as libc::c_int) as libc::c_uint;
        flags = check_new_section_flags(
            flags,
            obfd,
            bfd_section_name(isection as *const asection),
        );
    } else if strip_symbols as libc::c_uint
        == STRIP_NONDEBUG as libc::c_int as libc::c_uint
        && flags & (0x1 as libc::c_int | 0x2000000 as libc::c_int) as libc::c_uint
            != 0 as libc::c_int as libc::c_uint
        && !is_nondebug_keep_contents_section(ibfd, isection)
    {
        flags
            &= !(0x100 as libc::c_int | 0x2 as libc::c_int | 0x2000000 as libc::c_int)
                as libc::c_uint;
        if bfd_get_flavour(obfd) as libc::c_uint
            == bfd_target_elf_flavour as libc::c_int as libc::c_uint
        {
            make_nobits = 1 as libc::c_int != 0;
            (*isection).flags
                &= !(0x100 as libc::c_int | 0x2 as libc::c_int
                    | 0x2000000 as libc::c_int) as libc::c_uint;
        }
    }
    osection = bfd_make_section_anyway_with_flags(obfd, name, flags);
    if osection.is_null() {
        err = dcgettext(
            0 as *const libc::c_char,
            b"failed to create output section\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        );
    } else {
        if make_nobits {
            (*((*osection).used_by_bfd as *mut bfd_elf_section_data))
                .this_hdr
                .sh_type = 8 as libc::c_int as libc::c_uint;
        }
        size = bfd_section_size(isection as *const asection);
        size = bfd_convert_section_size(ibfd, isection, obfd, size);
        if copy_byte >= 0 as libc::c_int {
            size = size
                .wrapping_add(interleave as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                .wrapping_div(interleave as libc::c_ulong)
                .wrapping_mul(copy_width as libc::c_ulong);
        } else if extract_symbol {
            size = 0 as libc::c_int as bfd_size_type;
        }
        if !bfd_set_section_size(osection, size) {
            err = dcgettext(
                0 as *const libc::c_char,
                b"failed to set size\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            );
        } else {
            vma = bfd_section_vma(isection as *const asection);
            p = find_section_list(
                bfd_section_name(isection as *const asection),
                0 as libc::c_int != 0,
                ((1 as libc::c_int) << 4 as libc::c_int
                    | (1 as libc::c_int) << 3 as libc::c_int) as libc::c_uint,
            );
            if !p.is_null() {
                if (*p).context
                    & ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_uint != 0
                {
                    vma = (*p).vma_val;
                } else {
                    vma = (vma as libc::c_ulong).wrapping_add((*p).vma_val) as bfd_vma
                        as bfd_vma;
                }
            } else {
                vma = (vma as libc::c_ulong).wrapping_add(change_section_address)
                    as bfd_vma as bfd_vma;
            }
            if !bfd_set_section_vma(osection, vma) {
                err = dcgettext(
                    0 as *const libc::c_char,
                    b"failed to set vma\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                );
            } else {
                lma = (*isection).lma;
                p = find_section_list(
                    bfd_section_name(isection as *const asection),
                    0 as libc::c_int != 0,
                    ((1 as libc::c_int) << 6 as libc::c_int
                        | (1 as libc::c_int) << 5 as libc::c_int) as libc::c_uint,
                );
                if !p.is_null() {
                    if (*p).context
                        & ((1 as libc::c_int) << 6 as libc::c_int) as libc::c_uint != 0
                    {
                        lma = (lma as libc::c_ulong).wrapping_add((*p).lma_val)
                            as bfd_vma as bfd_vma;
                    } else {
                        lma = (*p).lma_val;
                    }
                } else {
                    lma = (lma as libc::c_ulong).wrapping_add(change_section_address)
                        as bfd_vma as bfd_vma;
                }
                (*osection).lma = lma;
                p = find_section_list(
                    bfd_section_name(isection as *const asection),
                    0 as libc::c_int != 0,
                    ((1 as libc::c_int) << 9 as libc::c_int) as libc::c_uint,
                );
                if !p.is_null() {
                    alignment = (*p).alignment;
                } else {
                    alignment = bfd_section_alignment(isection as *const asection);
                }
                if !bfd_set_section_alignment(osection, alignment) {
                    err = dcgettext(
                        0 as *const libc::c_char,
                        b"failed to set alignment\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    );
                } else {
                    (*osection).entsize = (*isection).entsize;
                    (*osection).set_compress_status((*isection).compress_status());
                    (*isection).output_section = osection;
                    (*isection).output_offset = 0 as libc::c_int as bfd_vma;
                    if (*isection).flags & 0x2000000 as libc::c_int as libc::c_uint
                        != 0 as libc::c_int as libc::c_uint
                    {
                        let mut gsym: *mut asymbol = group_signature(isection);
                        if !gsym.is_null() {
                            (*gsym).flags
                                |= ((1 as libc::c_int) << 5 as libc::c_int) as libc::c_uint;
                            if bfd_get_flavour(ibfd) as libc::c_uint
                                == bfd_target_elf_flavour as libc::c_int as libc::c_uint
                            {
                                let ref mut fresh0 = (*((*isection).used_by_bfd
                                    as *mut bfd_elf_section_data))
                                    .group
                                    .id;
                                *fresh0 = gsym;
                            }
                        }
                    }
                    if !(Some(
                        ((*(*obfd).xvec)._bfd_copy_private_section_data)
                            .expect("non-null function pointer"),
                    ))
                        .expect(
                            "non-null function pointer",
                        )(ibfd, isection, obfd, osection)
                    {
                        err = dcgettext(
                            0 as *const libc::c_char,
                            b"failed to copy private data\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        );
                    } else {
                        return
                    }
                }
            }
        }
    }
    status = 1 as libc::c_int;
    bfd_nonfatal_message(
        0 as *const libc::c_char,
        obfd,
        osection as *const asection,
        err,
    );
}
unsafe extern "C" fn setup_bfd_headers(mut ibfd: *mut bfd, mut obfd: *mut bfd) {
    if !(Some(
        ((*(*obfd).xvec)._bfd_copy_private_header_data)
            .expect("non-null function pointer"),
    ))
        .expect("non-null function pointer")(ibfd, obfd)
    {
        status = 1 as libc::c_int;
        bfd_nonfatal_message(
            0 as *const libc::c_char,
            ibfd,
            0 as *const asection,
            dcgettext(
                0 as *const libc::c_char,
                b"error in private header data\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return;
    }
}
unsafe extern "C" fn copy_relocations_in_section(
    mut ibfd: *mut bfd,
    mut isection: sec_ptr,
    mut obfdarg: *mut libc::c_void,
) {
    let mut obfd: *mut bfd = obfdarg as *mut bfd;
    let mut relsize: libc::c_long = 0;
    let mut relpp: *mut *mut arelent = 0 as *mut *mut arelent;
    let mut relcount: libc::c_long = 0;
    let mut osection: sec_ptr = 0 as *mut bfd_section;
    if skip_section(ibfd, isection, 0 as libc::c_int != 0) {
        return;
    }
    osection = (*isection).output_section;
    if bfd_get_format(obfd) as libc::c_uint == bfd_core as libc::c_int as libc::c_uint
        || strip_symbols as libc::c_uint == STRIP_NONDWO as libc::c_int as libc::c_uint
        || discard_relocations(ibfd, isection) as libc::c_int != 0
    {
        relsize = 0 as libc::c_int as libc::c_long;
    } else {
        relsize = bfd_get_reloc_upper_bound(ibfd, isection);
        if relsize < 0 as libc::c_int as libc::c_long {
            if relsize == -(1 as libc::c_int) as libc::c_long
                && bfd_get_error() as libc::c_uint
                    == bfd_error_invalid_operation as libc::c_int as libc::c_uint
            {
                relsize = 0 as libc::c_int as libc::c_long;
            } else {
                status = 1 as libc::c_int;
                bfd_nonfatal_message(
                    0 as *const libc::c_char,
                    ibfd,
                    isection as *const asection,
                    0 as *const libc::c_char,
                );
                return;
            }
        }
    }
    if relsize == 0 as libc::c_int as libc::c_long {
        (Some(((*(*obfd).xvec)._bfd_set_reloc).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(obfd, osection, 0 as *mut *mut arelent, 0 as libc::c_int as libc::c_uint);
        (*osection).flags &= !(0x4 as libc::c_int) as libc::c_uint;
    } else {
        if !((*isection).orelocation).is_null() {
            relcount = (*isection).reloc_count as libc::c_long;
            relpp = (*isection).orelocation;
        } else {
            relpp = xmalloc(relsize as size_t) as *mut *mut arelent;
            relcount = bfd_canonicalize_reloc(ibfd, isection, relpp, isympp);
            if relcount < 0 as libc::c_int as libc::c_long {
                status = 1 as libc::c_int;
                bfd_nonfatal_message(
                    0 as *const libc::c_char,
                    ibfd,
                    isection as *const asection,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"relocation count is negative\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                free(relpp as *mut libc::c_void);
                return;
            }
        }
        if strip_symbols as libc::c_uint == STRIP_ALL as libc::c_int as libc::c_uint {
            let mut temp_relpp: *mut *mut arelent = 0 as *mut *mut arelent;
            let mut temp_relcount: libc::c_long = 0 as libc::c_int as libc::c_long;
            let mut i: libc::c_long = 0;
            temp_relpp = xmalloc(relsize as size_t) as *mut *mut arelent;
            i = 0 as libc::c_int as libc::c_long;
            while i < relcount {
                if !((**relpp.offset(i as isize)).sym_ptr_ptr).is_null()
                    && !(*(**relpp.offset(i as isize)).sym_ptr_ptr).is_null()
                {
                    if is_specified_symbol(
                        bfd_asymbol_name(*(**relpp.offset(i as isize)).sym_ptr_ptr),
                        keep_specific_htab,
                    ) {
                        let fresh1 = temp_relcount;
                        temp_relcount = temp_relcount + 1;
                        let ref mut fresh2 = *temp_relpp.offset(fresh1 as isize);
                        *fresh2 = *relpp.offset(i as isize);
                    }
                }
                i += 1;
                i;
            }
            relcount = temp_relcount;
            if relpp != (*isection).orelocation {
                free(relpp as *mut libc::c_void);
            }
            relpp = temp_relpp;
        }
        (Some(((*(*obfd).xvec)._bfd_set_reloc).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            obfd,
            osection,
            if relcount == 0 as libc::c_int as libc::c_long {
                0 as *mut *mut arelent
            } else {
                relpp
            },
            relcount as libc::c_uint,
        );
        if relcount == 0 as libc::c_int as libc::c_long {
            (*osection).flags &= !(0x4 as libc::c_int) as libc::c_uint;
            if relpp != (*isection).orelocation {
                free(relpp as *mut libc::c_void);
            }
        }
    };
}
unsafe extern "C" fn copy_section(
    mut ibfd: *mut bfd,
    mut isection: sec_ptr,
    mut obfdarg: *mut libc::c_void,
) {
    let mut obfd: *mut bfd = obfdarg as *mut bfd;
    let mut p: *mut section_list = 0 as *mut section_list;
    let mut osection: sec_ptr = 0 as *mut bfd_section;
    let mut size: bfd_size_type = 0;
    if skip_section(ibfd, isection, 1 as libc::c_int != 0) {
        return;
    }
    osection = (*isection).output_section;
    size = bfd_section_size(isection as *const asection);
    if bfd_section_flags(isection as *const asection)
        & 0x100 as libc::c_int as libc::c_uint != 0
        && bfd_section_flags(osection as *const asection)
            & 0x100 as libc::c_int as libc::c_uint != 0
    {
        let mut memhunk: *mut bfd_byte = 0 as *mut bfd_byte;
        if !bfd_get_full_section_contents(ibfd, isection, &mut memhunk)
            || !bfd_convert_section_contents(
                ibfd,
                isection,
                obfd,
                &mut memhunk,
                &mut size,
            )
        {
            status = 1 as libc::c_int;
            bfd_nonfatal_message(
                0 as *const libc::c_char,
                ibfd,
                isection as *const asection,
                0 as *const libc::c_char,
            );
            free(memhunk as *mut libc::c_void);
            return;
        }
        if reverse_bytes != 0 {
            if size.wrapping_rem(reverse_bytes as libc::c_ulong)
                == 0 as libc::c_int as libc::c_ulong
            {
                let mut i: libc::c_ulong = 0;
                let mut j: libc::c_ulong = 0;
                let mut b: bfd_byte = 0;
                i = 0 as libc::c_int as libc::c_ulong;
                while i < size {
                    j = 0 as libc::c_int as libc::c_ulong;
                    while j < (reverse_bytes / 2 as libc::c_int) as libc::c_ulong {
                        let mut m: *mut bfd_byte = memhunk;
                        b = *m.offset(i.wrapping_add(j) as isize);
                        *m
                            .offset(
                                i.wrapping_add(j) as isize,
                            ) = *m
                            .offset(
                                i
                                    .wrapping_add(reverse_bytes as libc::c_ulong)
                                    .wrapping_sub(
                                        j.wrapping_add(1 as libc::c_int as libc::c_ulong),
                                    ) as isize,
                            );
                        *m
                            .offset(
                                i
                                    .wrapping_add(reverse_bytes as libc::c_ulong)
                                    .wrapping_sub(
                                        j.wrapping_add(1 as libc::c_int as libc::c_ulong),
                                    ) as isize,
                            ) = b;
                        j = j.wrapping_add(1);
                        j;
                    }
                    i = i.wrapping_add(reverse_bytes as libc::c_ulong);
                }
            } else {
                fatal(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"cannot reverse bytes: length of section %s must be evenly divisible by %d\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    bfd_section_name(isection as *const asection),
                    reverse_bytes,
                );
            }
        }
        if copy_byte >= 0 as libc::c_int {
            let mut from: *mut libc::c_char = (memhunk as *mut libc::c_char)
                .offset(copy_byte as isize);
            let mut to: *mut libc::c_char = memhunk as *mut libc::c_char;
            let mut end: *mut libc::c_char = (memhunk as *mut libc::c_char)
                .offset(size as isize);
            let mut i_0: libc::c_int = 0;
            let mut extra: libc::c_int = ((*isection).lma)
                .wrapping_rem(interleave as libc::c_ulong) as libc::c_int;
            from = from.offset(-(extra as isize));
            if copy_byte < extra {
                from = from.offset(interleave as isize);
            }
            while from < end {
                i_0 = 0 as libc::c_int;
                while i_0 < copy_width {
                    if &mut *from.offset(i_0 as isize) as *mut libc::c_char >= end {
                        break;
                    }
                    let fresh3 = to;
                    to = to.offset(1);
                    *fresh3 = *from.offset(i_0 as isize);
                    i_0 += 1;
                    i_0;
                }
                from = from.offset(interleave as isize);
            }
            size = size
                .wrapping_add(interleave as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                .wrapping_sub(copy_byte as libc::c_ulong)
                .wrapping_div(interleave as libc::c_ulong)
                .wrapping_mul(copy_width as libc::c_ulong);
            (*osection)
                .lma = ((*osection).lma as libc::c_ulong)
                .wrapping_div(interleave as libc::c_ulong) as bfd_vma as bfd_vma;
            if copy_byte < extra {
                (*osection).lma = ((*osection).lma).wrapping_add(1);
                (*osection).lma;
            }
        }
        if !bfd_set_section_contents(
            obfd,
            osection,
            memhunk as *const libc::c_void,
            0 as libc::c_int as file_ptr,
            size,
        ) {
            status = 1 as libc::c_int;
            bfd_nonfatal_message(
                0 as *const libc::c_char,
                obfd,
                osection as *const asection,
                0 as *const libc::c_char,
            );
            free(memhunk as *mut libc::c_void);
            return;
        }
        free(memhunk as *mut libc::c_void);
    } else {
        p = find_section_list(
            bfd_section_name(isection as *const asection),
            0 as libc::c_int != 0,
            ((1 as libc::c_int) << 7 as libc::c_int) as libc::c_uint,
        );
        if !p.is_null()
            && (*p).flags & 0x100 as libc::c_int as libc::c_uint
                != 0 as libc::c_int as libc::c_uint
        {
            let mut memhunk_0: *mut libc::c_void = xmalloc(size);
            memset(memhunk_0, 0 as libc::c_int, size);
            if !bfd_set_section_contents(
                obfd,
                osection,
                memhunk_0,
                0 as libc::c_int as file_ptr,
                size,
            ) {
                status = 1 as libc::c_int;
                bfd_nonfatal_message(
                    0 as *const libc::c_char,
                    obfd,
                    osection as *const asection,
                    0 as *const libc::c_char,
                );
                free(memhunk_0);
                return;
            }
            free(memhunk_0);
        }
    };
}
unsafe extern "C" fn get_sections(
    mut obfd: *mut bfd,
    mut osection: *mut asection,
    mut secppparg: *mut libc::c_void,
) {
    let mut secppp: *mut *mut *mut asection = secppparg as *mut *mut *mut asection;
    **secppp = osection;
    *secppp = (*secppp).offset(1);
    *secppp;
}
unsafe extern "C" fn compare_section_lma(
    mut arg1: *const libc::c_void,
    mut arg2: *const libc::c_void,
) -> libc::c_int {
    let mut sec1: *const asection = *(arg1 as *mut *const asection);
    let mut sec2: *const asection = *(arg2 as *mut *const asection);
    let mut flags1: flagword = 0;
    let mut flags2: flagword = 0;
    flags1 = (*sec1).flags;
    flags2 = (*sec2).flags;
    if flags1 & 0x100 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint
        || flags1 & 0x2 as libc::c_int as libc::c_uint
            == 0 as libc::c_int as libc::c_uint
    {
        if flags2 & 0x100 as libc::c_int as libc::c_uint
            != 0 as libc::c_int as libc::c_uint
            && flags2 & 0x2 as libc::c_int as libc::c_uint
                != 0 as libc::c_int as libc::c_uint
        {
            return -(1 as libc::c_int);
        }
    } else if flags2 & 0x100 as libc::c_int as libc::c_uint
        == 0 as libc::c_int as libc::c_uint
        || flags2 & 0x2 as libc::c_int as libc::c_uint
            == 0 as libc::c_int as libc::c_uint
    {
        return 1 as libc::c_int
    }
    if (*sec1).lma > (*sec2).lma {
        return 1 as libc::c_int;
    }
    if (*sec1).lma < (*sec2).lma {
        return -(1 as libc::c_int);
    }
    if bfd_section_size(sec1) > bfd_section_size(sec2) {
        return 1 as libc::c_int;
    }
    if bfd_section_size(sec1) < bfd_section_size(sec2) {
        return -(1 as libc::c_int);
    }
    if (*sec1).id > (*sec2).id {
        return 1 as libc::c_int;
    }
    if (*sec1).id < (*sec2).id {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn mark_symbols_used_in_relocations(
    mut ibfd: *mut bfd,
    mut isection: sec_ptr,
    mut symbolsarg: *mut libc::c_void,
) {
    let mut symbols: *mut *mut asymbol = symbolsarg as *mut *mut asymbol;
    let mut relsize: libc::c_long = 0;
    let mut relpp: *mut *mut arelent = 0 as *mut *mut arelent;
    let mut relcount: libc::c_long = 0;
    let mut i: libc::c_long = 0;
    if ((*isection).output_section).is_null() {
        return;
    }
    relsize = bfd_get_reloc_upper_bound(ibfd, isection);
    if relsize < 0 as libc::c_int as libc::c_long {
        if relsize == -(1 as libc::c_int) as libc::c_long
            && bfd_get_error() as libc::c_uint
                == bfd_error_invalid_operation as libc::c_int as libc::c_uint
        {
            return;
        }
        bfd_fatal(bfd_get_filename(ibfd));
    }
    if relsize == 0 as libc::c_int as libc::c_long {
        return;
    }
    relpp = xmalloc(relsize as size_t) as *mut *mut arelent;
    relcount = bfd_canonicalize_reloc(ibfd, isection, relpp, symbols);
    if relcount < 0 as libc::c_int as libc::c_long {
        bfd_fatal(bfd_get_filename(ibfd));
    }
    i = 0 as libc::c_int as libc::c_long;
    while i < relcount {
        if !((**relpp.offset(i as isize)).sym_ptr_ptr).is_null()
            && !(*(**relpp.offset(i as isize)).sym_ptr_ptr).is_null()
            && *(**relpp.offset(i as isize)).sym_ptr_ptr
                != _bfd_std_section[0 as libc::c_int as usize].symbol
            && *(**relpp.offset(i as isize)).sym_ptr_ptr
                != _bfd_std_section[2 as libc::c_int as usize].symbol
            && *(**relpp.offset(i as isize)).sym_ptr_ptr
                != _bfd_std_section[1 as libc::c_int as usize].symbol
        {
            let ref mut fresh4 = (**(**relpp.offset(i as isize)).sym_ptr_ptr).flags;
            *fresh4 |= ((1 as libc::c_int) << 5 as libc::c_int) as libc::c_uint;
        }
        i += 1;
        i;
    }
    free(relpp as *mut libc::c_void);
}
unsafe extern "C" fn write_debugging_info(
    mut obfd: *mut bfd,
    mut dhandle: *mut libc::c_void,
    mut symcountp: *mut libc::c_long,
    mut symppp: *mut *mut *mut asymbol,
) -> bool {
    if bfd_get_flavour(obfd) as libc::c_uint
        == bfd_target_coff_flavour as libc::c_int as libc::c_uint
        || bfd_get_flavour(obfd) as libc::c_uint
            == bfd_target_elf_flavour as libc::c_int as libc::c_uint
    {
        let mut syms: *mut bfd_byte = 0 as *mut bfd_byte;
        let mut strings: *mut bfd_byte = 0 as *mut bfd_byte;
        let mut symsize: bfd_size_type = 0;
        let mut stringsize: bfd_size_type = 0;
        let mut stabsec: *mut asection = 0 as *mut asection;
        let mut stabstrsec: *mut asection = 0 as *mut asection;
        let mut flags: flagword = 0;
        if !write_stabs_in_sections_debugging_info(
            obfd,
            dhandle,
            &mut syms,
            &mut symsize,
            &mut strings,
            &mut stringsize,
        ) {
            return 0 as libc::c_int != 0;
        }
        flags = (0x100 as libc::c_int | 0x8 as libc::c_int | 0x2000 as libc::c_int)
            as flagword;
        stabsec = bfd_make_section_with_flags(
            obfd,
            b".stab\0" as *const u8 as *const libc::c_char,
            flags,
        );
        stabstrsec = bfd_make_section_with_flags(
            obfd,
            b".stabstr\0" as *const u8 as *const libc::c_char,
            flags,
        );
        if stabsec.is_null() || stabstrsec.is_null()
            || !bfd_set_section_size(stabsec, symsize)
            || !bfd_set_section_size(stabstrsec, stringsize)
            || !bfd_set_section_alignment(stabsec, 2 as libc::c_int as libc::c_uint)
            || !bfd_set_section_alignment(stabstrsec, 0 as libc::c_int as libc::c_uint)
        {
            bfd_nonfatal_message(
                0 as *const libc::c_char,
                obfd,
                0 as *const asection,
                dcgettext(
                    0 as *const libc::c_char,
                    b"can't create debugging section\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            free(strings as *mut libc::c_void);
            return 0 as libc::c_int != 0;
        }
        if !bfd_set_section_contents(
            obfd,
            stabsec,
            syms as *const libc::c_void,
            0 as libc::c_int as file_ptr,
            symsize,
        )
            || !bfd_set_section_contents(
                obfd,
                stabstrsec,
                strings as *const libc::c_void,
                0 as libc::c_int as file_ptr,
                stringsize,
            )
        {
            bfd_nonfatal_message(
                0 as *const libc::c_char,
                obfd,
                0 as *const asection,
                dcgettext(
                    0 as *const libc::c_char,
                    b"can't set debugging section contents\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            free(strings as *mut libc::c_void);
            return 0 as libc::c_int != 0;
        }
        return 1 as libc::c_int != 0;
    }
    bfd_nonfatal_message(
        0 as *const libc::c_char,
        obfd,
        0 as *const asection,
        dcgettext(
            0 as *const libc::c_char,
            b"don't know how to write debugging information for %s\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
        bfd_get_target(obfd),
    );
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn lookup_sym_redefinition(
    mut source: *const libc::c_char,
) -> *const libc::c_char {
    let mut key_node: redefine_node = {
        let mut init = redefine_node {
            source: source as *mut libc::c_char,
            target: 0 as *mut libc::c_char,
        };
        init
    };
    let mut redef_node: *mut redefine_node = htab_find(
        redefine_specific_htab,
        &mut key_node as *mut redefine_node as *const libc::c_void,
    ) as *mut redefine_node;
    return if redef_node.is_null() {
        source
    } else {
        (*redef_node).target as *const libc::c_char
    };
}
unsafe extern "C" fn find_section_rename(
    mut old_name: *const libc::c_char,
    mut returned_flags: *mut flagword,
) -> *const libc::c_char {
    let mut srename: *const section_rename = 0 as *const section_rename;
    srename = section_rename_list;
    while !srename.is_null() {
        if strcmp((*srename).old_name, old_name) == 0 as libc::c_int {
            if !returned_flags.is_null()
                && (*srename).flags != -(1 as libc::c_int) as flagword
            {
                *returned_flags = (*srename).flags;
            }
            return (*srename).new_name;
        }
        srename = (*srename).next;
    }
    return old_name;
}
unsafe extern "C" fn copy_usage(
    mut stream: *mut FILE,
    mut exit_status: libc::c_int,
) -> ! {
    fprintf(
        stream,
        dcgettext(
            0 as *const libc::c_char,
            b"Usage: %s [option(s)] in-file [out-file]\n\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
        program_name,
    );
    fprintf(
        stream,
        dcgettext(
            0 as *const libc::c_char,
            b" Copies a binary file, possibly transforming it in the process\n\0"
                as *const u8 as *const libc::c_char,
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
            b"  -I --input-target <bfdname>      Assume input file is in format <bfdname>\n  -O --output-target <bfdname>     Create an output file in format <bfdname>\n  -B --binary-architecture <arch>  Set output arch, when input is arch-less\n  -F --target <bfdname>            Set both input and output format to <bfdname>\n     --debugging                   Convert debugging information, if possible\n  -p --preserve-dates              Copy modified/access timestamps to the output\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    fprintf(
        stream,
        dcgettext(
            0 as *const libc::c_char,
            b"  -D --enable-deterministic-archives\n                                   Produce deterministic output when stripping archives\n  -U --disable-deterministic-archives\n                                   Disable -D behavior (default)\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    fprintf(
        stream,
        dcgettext(
            0 as *const libc::c_char,
            b"  -j --only-section <name>         Only copy section <name> into the output\n     --add-gnu-debuglink=<file>    Add section .gnu_debuglink linking to <file>\n  -R --remove-section <name>       Remove section <name> from the output\n     --remove-relocations <name>   Remove relocations from section <name>\n  -S --strip-all                   Remove all symbol and relocation information\n  -g --strip-debug                 Remove all debugging symbols & sections\n     --strip-dwo                   Remove all DWO sections\n     --strip-unneeded              Remove all symbols not needed by relocations\n  -N --strip-symbol <name>         Do not copy symbol <name>\n     --strip-unneeded-symbol <name>\n                                   Do not copy symbol <name> unless needed by\n                                     relocations\n     --only-keep-debug             Strip everything but the debug information\n     --extract-dwo                 Copy only DWO sections\n     --extract-symbol              Remove section contents but keep symbols\n     --keep-section <name>         Do not strip section <name>\n  -K --keep-symbol <name>          Do not strip symbol <name>\n     --keep-section-symbols        Do not strip section symbols\n     --keep-file-symbols           Do not strip file symbol(s)\n     --localize-hidden             Turn all ELF hidden symbols into locals\n  -L --localize-symbol <name>      Force symbol <name> to be marked as a local\n     --globalize-symbol <name>     Force symbol <name> to be marked as a global\n  -G --keep-global-symbol <name>   Localize all symbols except <name>\n  -W --weaken-symbol <name>        Force symbol <name> to be marked as a weak\n     --weaken                      Force all global symbols to be marked as weak\n  -w --wildcard                    Permit wildcard in symbol comparison\n  -x --discard-all                 Remove all non-global symbols\n  -X --discard-locals              Remove any compiler-generated symbols\n  -i --interleave[=<number>]       Only copy N out of every <number> bytes\n     --interleave-width <number>   Set N for --interleave\n  -b --byte <num>                  Select byte <num> in every interleaved block\n     --gap-fill <val>              Fill gaps between sections with <val>\n     --pad-to <addr>               Pad the last section up to address <addr>\n     --set-start <addr>            Set the start address to <addr>\n    {--change-start|--adjust-start} <incr>\n                                   Add <incr> to the start address\n    {--change-addresses|--adjust-vma} <incr>\n                                   Add <incr> to LMA, VMA and start addresses\n    {--change-section-address|--adjust-section-vma} <name>{=|+|-}<val>\n                                   Change LMA and VMA of section <name> by <val>\n     --change-section-lma <name>{=|+|-}<val>\n                                   Change the LMA of section <name> by <val>\n     --change-section-vma <name>{=|+|-}<val>\n                                   Change the VMA of section <name> by <val>\n    {--[no-]change-warnings|--[no-]adjust-warnings}\n                                   Warn if a named section does not exist\n     --set-section-flags <name>=<flags>\n                                   Set section <name>'s properties to <flags>\n     --set-section-alignment <name>=<align>\n                                   Set section <name>'s alignment to <align> bytes\n     --add-section <name>=<file>   Add section <name> found in <file> to output\n     --update-section <name>=<file>\n                                   Update contents of section <name> with\n                                   contents found in <file>\n     --dump-section <name>=<file>  Dump the contents of section <name> into <file>\n     --rename-section <old>=<new>[,<flags>] Rename section <old> to <new>\n     --long-section-names {enable|disable|keep}\n                                   Handle long section names in Coff objects.\n     --change-leading-char         Force output format's leading character style\n     --remove-leading-char         Remove leading character from global symbols\n     --reverse-bytes=<num>         Reverse <num> bytes at a time, in output sections with content\n     --redefine-sym <old>=<new>    Redefine symbol name <old> to <new>\n     --redefine-syms <file>        --redefine-sym for all symbol pairs \n                                     listed in <file>\n     --srec-len <number>           Restrict the length of generated Srecords\n     --srec-forceS3                Restrict the type of generated Srecords to S3\n     --strip-symbols <file>        -N for all symbols listed in <file>\n     --strip-unneeded-symbols <file>\n                                   --strip-unneeded-symbol for all symbols listed\n                                     in <file>\n     --keep-symbols <file>         -K for all symbols listed in <file>\n     --localize-symbols <file>     -L for all symbols listed in <file>\n     --globalize-symbols <file>    --globalize-symbol for all in <file>\n     --keep-global-symbols <file>  -G for all symbols listed in <file>\n     --weaken-symbols <file>       -W for all symbols listed in <file>\n     --add-symbol <name>=[<section>:]<value>[,<flags>]  Add a symbol\n     --alt-machine-code <index>    Use the target's <index>'th alternative machine\n     --writable-text               Mark the output text as writable\n     --readonly-text               Make the output text write protected\n     --pure                        Mark the output file as demand paged\n     --impure                      Mark the output file as impure\n     --prefix-symbols <prefix>     Add <prefix> to start of every symbol name\n     --prefix-sections <prefix>    Add <prefix> to start of every section name\n     --prefix-alloc-sections <prefix>\n                                   Add <prefix> to start of every allocatable\n                                     section name\n     --file-alignment <num>        Set PE file alignment to <num>\n     --heap <reserve>[,<commit>]   Set PE reserve/commit heap to <reserve>/\n                                   <commit>\n     --image-base <address>        Set PE image base to <address>\n     --section-alignment <num>     Set PE section alignment to <num>\n     --stack <reserve>[,<commit>]  Set PE reserve/commit stack to <reserve>/\n                                   <commit>\n     --subsystem <name>[:<version>]\n                                   Set PE subsystem to <name> [& <version>]\n     --compress-debug-sections[={none|zlib|zlib-gnu|zlib-gabi}]\n                                   Compress DWARF debug sections using zlib\n     --decompress-debug-sections   Decompress DWARF debug sections using zlib\n     --elf-stt-common=[yes|no]     Generate ELF common symbols with STT_COMMON\n                                     type\n     --verilog-data-width <number> Specifies data width, in bytes, for verilog output\n  -M  --merge-notes                Remove redundant entries in note sections\n      --no-merge-notes             Do not attempt to remove redundant notes (default)\n  -v --verbose                     List all object files modified\n  @<file>                          Read options from <file>\n  -V --version                     Display this program's version number\n  -h --help                        Display this output\n     --info                        List object formats & architectures supported\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    list_supported_targets(program_name, stream);
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
    exit(exit_status);
}
unsafe extern "C" fn strip_usage(
    mut stream: *mut FILE,
    mut exit_status: libc::c_int,
) -> ! {
    fprintf(
        stream,
        dcgettext(
            0 as *const libc::c_char,
            b"Usage: %s <option(s)> in-file(s)\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        program_name,
    );
    fprintf(
        stream,
        dcgettext(
            0 as *const libc::c_char,
            b" Removes symbols and sections from files\n\0" as *const u8
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
            b"  -I --input-target=<bfdname>      Assume input file is in format <bfdname>\n  -O --output-target=<bfdname>     Create an output file in format <bfdname>\n  -F --target=<bfdname>            Set both input and output format to <bfdname>\n  -p --preserve-dates              Copy modified/access timestamps to the output\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    fprintf(
        stream,
        dcgettext(
            0 as *const libc::c_char,
            b"  -D --enable-deterministic-archives\n                                   Produce deterministic output when stripping archives\n  -U --disable-deterministic-archives\n                                   Disable -D behavior (default)\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    fprintf(
        stream,
        dcgettext(
            0 as *const libc::c_char,
            b"  -R --remove-section=<name>       Also remove section <name> from the output\n     --remove-relocations <name>   Remove relocations from section <name>\n  -s --strip-all                   Remove all symbol and relocation information\n  -g -S -d --strip-debug           Remove all debugging symbols & sections\n     --strip-dwo                   Remove all DWO sections\n     --strip-unneeded              Remove all symbols not needed by relocations\n     --only-keep-debug             Strip everything but the debug information\n  -M  --merge-notes                Remove redundant entries in note sections (default)\n      --no-merge-notes             Do not attempt to remove redundant notes\n  -N --strip-symbol=<name>         Do not copy symbol <name>\n     --keep-section=<name>         Do not strip section <name>\n  -K --keep-symbol=<name>          Do not strip symbol <name>\n     --keep-section-symbols        Do not strip section symbols\n     --keep-file-symbols           Do not strip file symbol(s)\n  -w --wildcard                    Permit wildcard in symbol comparison\n  -x --discard-all                 Remove all non-global symbols\n  -X --discard-locals              Remove any compiler-generated symbols\n  -v --verbose                     List all object files modified\n  -V --version                     Display this program's version number\n  -h --help                        Display this output\n     --info                        List object formats & architectures supported\n  -o <file>                        Place stripped output into <file>\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    list_supported_targets(program_name, stream);
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
    exit(exit_status);
}
unsafe extern "C" fn parse_flags(mut s: *const libc::c_char) -> flagword {
    let mut ret: flagword = 0;
    let mut snext: *const libc::c_char = 0 as *const libc::c_char;
    let mut len: libc::c_int = 0;
    ret = 0 as libc::c_int as flagword;
    loop {
        snext = strchr(s, ',' as i32);
        if snext.is_null() {
            len = strlen(s) as libc::c_int;
        } else {
            len = snext.offset_from(s) as libc::c_long as libc::c_int;
            snext = snext.offset(1);
            snext;
        }
        if strncasecmp(
            b"alloc\0" as *const u8 as *const libc::c_char,
            s,
            len as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            ret |= 0x1 as libc::c_int as libc::c_uint;
        } else if strncasecmp(
            b"load\0" as *const u8 as *const libc::c_char,
            s,
            len as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            ret |= 0x2 as libc::c_int as libc::c_uint;
        } else if strncasecmp(
            b"noload\0" as *const u8 as *const libc::c_char,
            s,
            len as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            ret |= 0x200 as libc::c_int as libc::c_uint;
        } else if strncasecmp(
            b"readonly\0" as *const u8 as *const libc::c_char,
            s,
            len as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            ret |= 0x8 as libc::c_int as libc::c_uint;
        } else if strncasecmp(
            b"debug\0" as *const u8 as *const libc::c_char,
            s,
            len as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            ret |= 0x2000 as libc::c_int as libc::c_uint;
        } else if strncasecmp(
            b"code\0" as *const u8 as *const libc::c_char,
            s,
            len as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            ret |= 0x10 as libc::c_int as libc::c_uint;
        } else if strncasecmp(
            b"data\0" as *const u8 as *const libc::c_char,
            s,
            len as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            ret |= 0x20 as libc::c_int as libc::c_uint;
        } else if strncasecmp(
            b"rom\0" as *const u8 as *const libc::c_char,
            s,
            len as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            ret |= 0x40 as libc::c_int as libc::c_uint;
        } else if strncasecmp(
            b"exclude\0" as *const u8 as *const libc::c_char,
            s,
            len as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            ret |= 0x8000 as libc::c_int as libc::c_uint;
        } else if strncasecmp(
            b"share\0" as *const u8 as *const libc::c_char,
            s,
            len as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            ret |= 0x8000000 as libc::c_int as libc::c_uint;
        } else if strncasecmp(
            b"contents\0" as *const u8 as *const libc::c_char,
            s,
            len as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            ret |= 0x100 as libc::c_int as libc::c_uint;
        } else if strncasecmp(
            b"merge\0" as *const u8 as *const libc::c_char,
            s,
            len as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            ret |= 0x800000 as libc::c_int as libc::c_uint;
        } else if strncasecmp(
            b"strings\0" as *const u8 as *const libc::c_char,
            s,
            len as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            ret |= 0x1000000 as libc::c_int as libc::c_uint;
        } else {
            let mut copy: *mut libc::c_char = 0 as *mut libc::c_char;
            copy = xmalloc((len + 1 as libc::c_int) as size_t) as *mut libc::c_char;
            strncpy(copy, s, len as libc::c_ulong);
            *copy.offset(len as isize) = '\0' as i32 as libc::c_char;
            non_fatal(
                dcgettext(
                    0 as *const libc::c_char,
                    b"unrecognized section flag `%s'\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                copy,
            );
            fatal(
                dcgettext(
                    0 as *const libc::c_char,
                    b"supported flags: %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                b"alloc, load, noload, readonly, debug, code, data, rom, exclude, share, contents, merge, strings\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        s = snext;
        if s.is_null() {
            break;
        }
    }
    return ret;
}
unsafe extern "C" fn parse_symflags(
    mut s: *const libc::c_char,
    mut other: *mut *const libc::c_char,
) -> flagword {
    let mut ret: flagword = 0;
    let mut snext: *const libc::c_char = 0 as *const libc::c_char;
    let mut len: size_t = 0;
    ret = 0 as libc::c_int as flagword;
    loop {
        snext = strchr(s, ',' as i32);
        if snext.is_null() {
            len = strlen(s);
        } else {
            len = snext.offset_from(s) as libc::c_long as size_t;
            snext = snext.offset(1);
            snext;
        }
        if len
            == (::core::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            && strncasecmp(b"local\0" as *const u8 as *const libc::c_char, s, len)
                == 0 as libc::c_int
        {
            ret |= ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint;
        } else if len
            == (::core::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            && strncasecmp(b"global\0" as *const u8 as *const libc::c_char, s, len)
                == 0 as libc::c_int
        {
            ret |= ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uint;
        } else if len
            == (::core::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            && strncasecmp(b"export\0" as *const u8 as *const libc::c_char, s, len)
                == 0 as libc::c_int
        {
            ret |= ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uint;
        } else if len
            == (::core::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            && strncasecmp(b"debug\0" as *const u8 as *const libc::c_char, s, len)
                == 0 as libc::c_int
        {
            ret |= ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uint;
        } else if len
            == (::core::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            && strncasecmp(b"function\0" as *const u8 as *const libc::c_char, s, len)
                == 0 as libc::c_int
        {
            ret |= ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_uint;
        } else if len
            == (::core::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            && strncasecmp(b"weak\0" as *const u8 as *const libc::c_char, s, len)
                == 0 as libc::c_int
        {
            ret |= ((1 as libc::c_int) << 7 as libc::c_int) as libc::c_uint;
        } else if len
            == (::core::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            && strncasecmp(b"section\0" as *const u8 as *const libc::c_char, s, len)
                == 0 as libc::c_int
        {
            ret |= ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_uint;
        } else if len
            == (::core::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            && strncasecmp(b"constructor\0" as *const u8 as *const libc::c_char, s, len)
                == 0 as libc::c_int
        {
            ret |= ((1 as libc::c_int) << 11 as libc::c_int) as libc::c_uint;
        } else if len
            == (::core::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            && strncasecmp(b"warning\0" as *const u8 as *const libc::c_char, s, len)
                == 0 as libc::c_int
        {
            ret |= ((1 as libc::c_int) << 12 as libc::c_int) as libc::c_uint;
        } else if len
            == (::core::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            && strncasecmp(b"indirect\0" as *const u8 as *const libc::c_char, s, len)
                == 0 as libc::c_int
        {
            ret |= ((1 as libc::c_int) << 13 as libc::c_int) as libc::c_uint;
        } else if len
            == (::core::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            && strncasecmp(b"file\0" as *const u8 as *const libc::c_char, s, len)
                == 0 as libc::c_int
        {
            ret |= ((1 as libc::c_int) << 14 as libc::c_int) as libc::c_uint;
        } else if len
            == (::core::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            && strncasecmp(b"object\0" as *const u8 as *const libc::c_char, s, len)
                == 0 as libc::c_int
        {
            ret |= ((1 as libc::c_int) << 16 as libc::c_int) as libc::c_uint;
        } else if len
            == (::core::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            && strncasecmp(b"synthetic\0" as *const u8 as *const libc::c_char, s, len)
                == 0 as libc::c_int
        {
            ret |= ((1 as libc::c_int) << 21 as libc::c_int) as libc::c_uint;
        } else if len
            == (::core::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            && strncasecmp(
                b"indirect-function\0" as *const u8 as *const libc::c_char,
                s,
                len,
            ) == 0 as libc::c_int
        {
            ret
                |= ((1 as libc::c_int) << 22 as libc::c_int
                    | (1 as libc::c_int) << 3 as libc::c_int) as libc::c_uint;
        } else if len
            == (::core::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            && strncasecmp(
                b"unique-object\0" as *const u8 as *const libc::c_char,
                s,
                len,
            ) == 0 as libc::c_int
        {
            ret
                |= ((1 as libc::c_int) << 23 as libc::c_int
                    | (1 as libc::c_int) << 16 as libc::c_int) as libc::c_uint;
        } else if len >= ::core::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong
            && strncasecmp(
                b"before=\0" as *const u8 as *const libc::c_char,
                s,
                (::core::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            ) == 0 as libc::c_int
        {
            *other = xstrndup(
                s
                    .offset(
                        ::core::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong
                            as isize,
                    )
                    .offset(-(1 as libc::c_int as isize)),
                len
                    .wrapping_sub(
                        ::core::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong,
                    )
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            );
        } else {
            let mut copy: *mut libc::c_char = 0 as *mut libc::c_char;
            copy = xmalloc(len.wrapping_add(1 as libc::c_int as libc::c_ulong))
                as *mut libc::c_char;
            strncpy(copy, s, len);
            *copy.offset(len as isize) = '\0' as i32 as libc::c_char;
            non_fatal(
                dcgettext(
                    0 as *const libc::c_char,
                    b"unrecognized symbol flag `%s'\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                copy,
            );
            fatal(
                dcgettext(
                    0 as *const libc::c_char,
                    b"supported flags: %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                b"local, global, export, debug, function, weak, section, constructor, warning, indirect, file, object, synthetic, indirect-function, unique-object, before=<othersym>\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        s = snext;
        if s.is_null() {
            break;
        }
    }
    return ret;
}
unsafe extern "C" fn find_section_list(
    mut name: *const libc::c_char,
    mut add: bool,
    mut context: libc::c_uint,
) -> *mut section_list {
    let mut p: *mut section_list = 0 as *mut section_list;
    let mut match_0: *mut section_list = 0 as *mut section_list;
    p = change_sections;
    while !p.is_null() {
        if add {
            if strcmp((*p).pattern, name) == 0 as libc::c_int {
                if (*p).context
                    & ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint != 0
                    && context & ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uint
                        != 0
                    || context & ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint
                        != 0
                        && (*p).context
                            & ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uint
                            != 0
                {
                    fatal(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"error: %s both copied and removed\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        name,
                    );
                }
                if (*p).context
                    & ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_uint != 0
                    && context & ((1 as libc::c_int) << 4 as libc::c_int) as libc::c_uint
                        != 0
                    || context & ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_uint
                        != 0
                        && context
                            & ((1 as libc::c_int) << 4 as libc::c_int) as libc::c_uint
                            != 0
                {
                    fatal(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"error: %s both sets and alters VMA\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        name,
                    );
                }
                if (*p).context
                    & ((1 as libc::c_int) << 5 as libc::c_int) as libc::c_uint != 0
                    && context & ((1 as libc::c_int) << 6 as libc::c_int) as libc::c_uint
                        != 0
                    || context & ((1 as libc::c_int) << 5 as libc::c_int) as libc::c_uint
                        != 0
                        && context
                            & ((1 as libc::c_int) << 6 as libc::c_int) as libc::c_uint
                            != 0
                {
                    fatal(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"error: %s both sets and alters LMA\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        name,
                    );
                }
                (*p).context |= context;
                return p;
            }
        } else if (*p).context & context != 0 {
            if *((*p).pattern).offset(0 as libc::c_int as isize) as libc::c_int
                == '!' as i32
            {
                if fnmatch(
                    ((*p).pattern).offset(1 as libc::c_int as isize),
                    name,
                    0 as libc::c_int,
                ) == 0 as libc::c_int
                {
                    (*p).used = 1 as libc::c_int != 0;
                    return 0 as *mut section_list;
                }
            } else if fnmatch((*p).pattern, name, 0 as libc::c_int) == 0 as libc::c_int {
                if match_0.is_null() {
                    match_0 = p;
                }
            }
        }
        p = (*p).next;
    }
    if !add {
        if !match_0.is_null() {
            (*match_0).used = 1 as libc::c_int != 0;
        }
        return match_0;
    }
    p = xmalloc(::core::mem::size_of::<section_list>() as libc::c_ulong)
        as *mut section_list;
    (*p).pattern = name;
    (*p).used = 0 as libc::c_int != 0;
    (*p).context = context;
    (*p).vma_val = 0 as libc::c_int as bfd_vma;
    (*p).lma_val = 0 as libc::c_int as bfd_vma;
    (*p).flags = 0 as libc::c_int as flagword;
    (*p).alignment = 0 as libc::c_int as libc::c_uint;
    (*p).next = change_sections;
    change_sections = p;
    return p;
}
unsafe extern "C" fn eq_string_redefnode(
    mut s1: *const libc::c_void,
    mut s2: *const libc::c_void,
) -> libc::c_int {
    let mut node1: *mut redefine_node = s1 as *mut redefine_node;
    let mut node2: *mut redefine_node = s2 as *mut redefine_node;
    return (strcmp(
        (*node1).source as *const libc::c_char,
        (*node2).source as *const libc::c_char,
    ) == 0) as libc::c_int;
}
unsafe extern "C" fn htab_hash_redefnode(mut p: *const libc::c_void) -> hashval_t {
    let mut redefnode: *mut redefine_node = p as *mut redefine_node;
    return htab_hash_string((*redefnode).source as *const libc::c_void);
}
unsafe extern "C" fn create_symbol2redef_htab() -> htab_t {
    return htab_create_alloc(
        16 as libc::c_int as size_t,
        Some(
            htab_hash_redefnode as unsafe extern "C" fn(*const libc::c_void) -> hashval_t,
        ),
        Some(
            eq_string_redefnode
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
        None,
        Some(xcalloc as unsafe extern "C" fn(size_t, size_t) -> *mut libc::c_void),
        Some(free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
    );
}
unsafe extern "C" fn create_symbol_htab() -> htab_t {
    return htab_create_alloc(
        16 as libc::c_int as size_t,
        Some(htab_hash_string as unsafe extern "C" fn(*const libc::c_void) -> hashval_t),
        Some(
            htab_eq_string
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
        None,
        Some(xcalloc as unsafe extern "C" fn(size_t, size_t) -> *mut libc::c_void),
        Some(free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
    );
}
unsafe extern "C" fn create_symbol_htabs() {
    strip_specific_htab = create_symbol_htab();
    strip_unneeded_htab = create_symbol_htab();
    keep_specific_htab = create_symbol_htab();
    localize_specific_htab = create_symbol_htab();
    globalize_specific_htab = create_symbol_htab();
    keepglobal_specific_htab = create_symbol_htab();
    weaken_specific_htab = create_symbol_htab();
    redefine_specific_htab = create_symbol2redef_htab();
    redefine_specific_reverse_htab = create_symbol_htab();
}
unsafe extern "C" fn add_specific_symbol(
    mut name: *const libc::c_char,
    mut htab: htab_t,
) {
    let ref mut fresh5 = *htab_find_slot(htab, name as *const libc::c_void, INSERT);
    *fresh5 = name as *mut libc::c_char as *mut libc::c_void;
}
unsafe extern "C" fn add_specific_symbol_node(
    mut node: *const libc::c_void,
    mut htab: htab_t,
) {
    let ref mut fresh6 = *htab_find_slot(htab, node, INSERT);
    *fresh6 = node as *mut libc::c_void;
}
unsafe extern "C" fn add_specific_symbols(
    mut filename: *const libc::c_char,
    mut htab: htab_t,
    mut buffer_p: *mut *mut libc::c_char,
) {
    let mut size: off_t = 0;
    let mut f: *mut FILE = 0 as *mut FILE;
    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buffer: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut line_count: libc::c_uint = 0;
    size = get_file_size(filename);
    if size == 0 as libc::c_int as libc::c_long {
        status = 1 as libc::c_int;
        return;
    }
    buffer = xmalloc((size + 2 as libc::c_int as libc::c_long) as size_t)
        as *mut libc::c_char;
    f = fopen(filename, b"r\0" as *const u8 as *const libc::c_char);
    if f.is_null() {
        fatal(
            dcgettext(
                0 as *const libc::c_char,
                b"cannot open '%s': %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            filename,
            strerror(*__errno_location()),
        );
    }
    if fread(
        buffer as *mut libc::c_void,
        1 as libc::c_int as libc::c_ulong,
        size as libc::c_ulong,
        f,
    ) == 0 as libc::c_int as libc::c_ulong || ferror(f) != 0
    {
        fatal(
            dcgettext(
                0 as *const libc::c_char,
                b"%s: fread failed\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            filename,
        );
    }
    fclose(f);
    *buffer.offset(size as isize) = '\n' as i32 as libc::c_char;
    *buffer
        .offset(
            (size + 1 as libc::c_int as libc::c_long) as isize,
        ) = '\0' as i32 as libc::c_char;
    line_count = 1 as libc::c_int as libc::c_uint;
    line = buffer;
    while *line as libc::c_int != '\0' as i32 {
        let mut eol: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut name_end: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut finished: libc::c_int = 0 as libc::c_int;
        eol = line;
        loop {
            match *eol as libc::c_int {
                10 => {
                    *eol = '\0' as i32 as libc::c_char;
                    if *eol.offset(1 as libc::c_int as isize) as libc::c_int
                        == '\r' as i32
                    {
                        eol = eol.offset(1);
                        eol;
                    }
                    finished = 1 as libc::c_int;
                }
                13 => {
                    *eol = '\0' as i32 as libc::c_char;
                    if *eol.offset(1 as libc::c_int as isize) as libc::c_int
                        == '\n' as i32
                    {
                        eol = eol.offset(1);
                        eol;
                    }
                    finished = 1 as libc::c_int;
                }
                0 => {
                    finished = 1 as libc::c_int;
                }
                35 => {
                    *eol = '\0' as i32 as libc::c_char;
                }
                _ => {}
            }
            if finished != 0 {
                break;
            }
            eol = eol.offset(1);
            eol;
        }
        name = line;
        while *name as libc::c_int == ' ' as i32 || *name as libc::c_int == '\t' as i32 {
            name = name.offset(1);
            name;
        }
        name_end = name;
        while !(*name_end as libc::c_int == ' ' as i32
            || *name_end as libc::c_int == '\t' as i32)
            && !(*name_end as libc::c_int == '\n' as i32
                || *name_end as libc::c_int == '\r' as i32
                || *name_end as libc::c_int == '\0' as i32)
        {
            name_end = name_end.offset(1);
            name_end;
        }
        if !(*name_end as libc::c_int == '\n' as i32
            || *name_end as libc::c_int == '\r' as i32
            || *name_end as libc::c_int == '\0' as i32)
        {
            let mut extra: *mut libc::c_char = 0 as *mut libc::c_char;
            extra = name_end.offset(1 as libc::c_int as isize);
            while *extra as libc::c_int == ' ' as i32
                || *extra as libc::c_int == '\t' as i32
            {
                extra = extra.offset(1);
                extra;
            }
            if !(*extra as libc::c_int == '\n' as i32
                || *extra as libc::c_int == '\r' as i32
                || *extra as libc::c_int == '\0' as i32)
            {
                non_fatal(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s:%d: Ignoring rubbish found on this line\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    filename,
                    line_count,
                );
            }
        }
        *name_end = '\0' as i32 as libc::c_char;
        if name_end > name {
            add_specific_symbol(name, htab);
        }
        line = eol;
        line_count = line_count.wrapping_add(1);
        line_count;
        line = line.offset(1);
        line;
    }
    *buffer_p = buffer;
}
unsafe extern "C" fn is_specified_symbol_predicate(
    mut slot: *mut *mut libc::c_void,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut d: *mut is_specified_symbol_predicate_data = data
        as *mut is_specified_symbol_predicate_data;
    let mut slot_name: *const libc::c_char = *slot as *mut libc::c_char;
    if *slot_name as libc::c_int != '!' as i32 {
        if fnmatch(slot_name, (*d).name, 0 as libc::c_int) == 0 {
            (*d).found = 1 as libc::c_int != 0;
            return 1 as libc::c_int;
        }
    } else if fnmatch(
        slot_name.offset(1 as libc::c_int as isize),
        (*d).name,
        0 as libc::c_int,
    ) == 0
    {
        (*d).found = 0 as libc::c_int != 0;
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn is_specified_symbol(
    mut name: *const libc::c_char,
    mut htab: htab_t,
) -> bool {
    if wildcard {
        let mut data: is_specified_symbol_predicate_data = is_specified_symbol_predicate_data {
            name: 0 as *const libc::c_char,
            found: false,
        };
        data.name = name;
        data.found = 0 as libc::c_int != 0;
        htab_traverse(
            htab,
            Some(
                is_specified_symbol_predicate
                    as unsafe extern "C" fn(
                        *mut *mut libc::c_void,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            &mut data as *mut is_specified_symbol_predicate_data as *mut libc::c_void,
        );
        return data.found;
    }
    return !(htab_find(htab, name as *const libc::c_void)).is_null();
}
unsafe extern "C" fn group_signature(mut group: *mut asection) -> *mut asymbol {
    let mut abfd: *mut bfd = (*group).owner;
    let mut ghdr: *mut Elf_Internal_Shdr = 0 as *mut Elf_Internal_Shdr;
    if isympp.is_null() {
        return 0 as *mut asymbol;
    }
    if bfd_get_flavour(abfd) as libc::c_uint
        != bfd_target_elf_flavour as libc::c_int as libc::c_uint
    {
        return 0 as *mut asymbol;
    }
    ghdr = &mut (*((*group).used_by_bfd as *mut bfd_elf_section_data)).this_hdr;
    if (*ghdr).sh_link == (*(*abfd).tdata.elf_obj_data).symtab_section {
        let mut bed: *const elf_backend_data = (*(*abfd).xvec).backend_data
            as *const elf_backend_data;
        let mut symhdr: *mut Elf_Internal_Shdr = &mut (*(*abfd).tdata.elf_obj_data)
            .symtab_hdr;
        if (*ghdr).sh_info > 0 as libc::c_int as libc::c_uint
            && ((*ghdr).sh_info as libc::c_ulong)
                < ((*symhdr).sh_size)
                    .wrapping_div((*(*bed).s).sizeof_sym as libc::c_ulong)
        {
            return *isympp
                .offset(
                    ((*ghdr).sh_info).wrapping_sub(1 as libc::c_int as libc::c_uint)
                        as isize,
                );
        }
    }
    return 0 as *mut asymbol;
}
unsafe extern "C" fn is_dwo_section(mut abfd: *mut bfd, mut sec: *mut asection) -> bool {
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut len: libc::c_int = 0;
    if sec.is_null()
        || {
            name = bfd_section_name(sec);
            name.is_null()
        }
    {
        return 0 as libc::c_int != 0;
    }
    len = strlen(name) as libc::c_int;
    if len < 5 as libc::c_int {
        return 0 as libc::c_int != 0;
    }
    return startswith(
        name.offset(len as isize).offset(-(4 as libc::c_int as isize)),
        b".dwo\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn is_update_section(
    mut abfd: *mut bfd,
    mut sec: *mut asection,
) -> bool {
    if !update_sections.is_null() {
        let mut pupdate: *mut section_add = 0 as *mut section_add;
        pupdate = update_sections;
        while !pupdate.is_null() {
            if strcmp((*sec).name, (*pupdate).name) == 0 as libc::c_int {
                return 1 as libc::c_int != 0;
            }
            pupdate = (*pupdate).next;
        }
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn is_mergeable_note_section(
    mut abfd: *mut bfd,
    mut sec: *mut asection,
) -> bool {
    if merge_notes as libc::c_int != 0
        && bfd_get_flavour(abfd) as libc::c_uint
            == bfd_target_elf_flavour as libc::c_int as libc::c_uint
        && (*((*sec).used_by_bfd as *mut bfd_elf_section_data)).this_hdr.sh_type
            == 7 as libc::c_int as libc::c_uint
        && startswith(
            (*sec).name,
            b".gnu.build.attributes\0" as *const u8 as *const libc::c_char,
        ) as libc::c_int != 0
    {
        return 1 as libc::c_int != 0;
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn is_strip_section_1(
    mut abfd: *mut bfd,
    mut sec: *mut asection,
) -> bool {
    if !(find_section_list(
        bfd_section_name(sec),
        0 as libc::c_int != 0,
        ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uint,
    ))
        .is_null()
    {
        return 0 as libc::c_int != 0;
    }
    if sections_removed as libc::c_int != 0 || sections_copied as libc::c_int != 0 {
        let mut p: *mut section_list = 0 as *mut section_list;
        let mut q: *mut section_list = 0 as *mut section_list;
        p = find_section_list(
            bfd_section_name(sec),
            0 as libc::c_int != 0,
            ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint,
        );
        q = find_section_list(
            bfd_section_name(sec),
            0 as libc::c_int != 0,
            ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uint,
        );
        if !p.is_null() && !q.is_null() {
            fatal(
                dcgettext(
                    0 as *const libc::c_char,
                    b"error: section %s matches both remove and copy options\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                bfd_section_name(sec),
            );
        }
        if !p.is_null() && is_update_section(abfd, sec) as libc::c_int != 0 {
            fatal(
                dcgettext(
                    0 as *const libc::c_char,
                    b"error: section %s matches both update and remove options\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                bfd_section_name(sec),
            );
        }
        if !p.is_null() {
            return 1 as libc::c_int != 0;
        }
        if sections_copied as libc::c_int != 0 && q.is_null() {
            return 1 as libc::c_int != 0;
        }
    }
    if bfd_section_flags(sec) & 0x2000 as libc::c_int as libc::c_uint
        != 0 as libc::c_int as libc::c_uint
    {
        if strip_symbols as libc::c_uint == STRIP_DEBUG as libc::c_int as libc::c_uint
            || strip_symbols as libc::c_uint
                == STRIP_UNNEEDED as libc::c_int as libc::c_uint
            || strip_symbols as libc::c_uint == STRIP_ALL as libc::c_int as libc::c_uint
            || discard_locals as libc::c_uint
                == LOCALS_ALL as libc::c_int as libc::c_uint
            || convert_debugging as libc::c_int != 0
        {
            if strcmp(
                bfd_section_name(sec),
                b".reloc\0" as *const u8 as *const libc::c_char,
            ) != 0 as libc::c_int
            {
                return 1 as libc::c_int != 0;
            }
        }
        if strip_symbols as libc::c_uint == STRIP_DWO as libc::c_int as libc::c_uint {
            return is_dwo_section(abfd, sec);
        }
        if strip_symbols as libc::c_uint == STRIP_NONDEBUG as libc::c_int as libc::c_uint
        {
            return 0 as libc::c_int != 0;
        }
    }
    if strip_symbols as libc::c_uint == STRIP_NONDWO as libc::c_int as libc::c_uint {
        return !is_dwo_section(abfd, sec);
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn is_strip_section(
    mut abfd: *mut bfd,
    mut sec: *mut asection,
) -> bool {
    if is_strip_section_1(abfd, sec) {
        return 1 as libc::c_int != 0;
    }
    if bfd_section_flags(sec) & 0x2000000 as libc::c_int as libc::c_uint
        != 0 as libc::c_int as libc::c_uint
    {
        let mut gsym: *mut asymbol = 0 as *mut asymbol;
        let mut gname: *const libc::c_char = 0 as *const libc::c_char;
        let mut elt: *mut asection = 0 as *mut asection;
        let mut first: *mut asection = 0 as *mut asection;
        gsym = group_signature(sec);
        if gsym.is_null() {
            return 1 as libc::c_int != 0;
        }
        gname = (*gsym).name;
        if strip_symbols as libc::c_uint == STRIP_ALL as libc::c_int as libc::c_uint
            && !is_specified_symbol(gname, keep_specific_htab)
            || is_specified_symbol(gname, strip_specific_htab) as libc::c_int != 0
        {
            return 1 as libc::c_int != 0;
        }
        elt = (*((*sec).used_by_bfd as *mut bfd_elf_section_data)).next_in_group;
        first = elt;
        while !elt.is_null() {
            if !is_strip_section_1(abfd, elt) {
                return 0 as libc::c_int != 0;
            }
            elt = (*((*elt).used_by_bfd as *mut bfd_elf_section_data)).next_in_group;
            if elt == first {
                break;
            }
        }
        return 1 as libc::c_int != 0;
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn is_nondebug_keep_contents_section(
    mut ibfd: *mut bfd,
    mut isection: *mut asection,
) -> bool {
    if bfd_get_flavour(ibfd) as libc::c_uint
        == bfd_target_elf_flavour as libc::c_int as libc::c_uint
    {
        return (*((*isection).used_by_bfd as *mut bfd_elf_section_data)).this_hdr.sh_type
            == 7 as libc::c_int as libc::c_uint;
    }
    if bfd_get_flavour(ibfd) as libc::c_uint
        == bfd_target_coff_flavour as libc::c_int as libc::c_uint
    {
        return strcmp(
            bfd_section_name(isection),
            b".buildid\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int;
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn is_hidden_symbol(mut sym: *mut asymbol) -> bool {
    let mut elf_sym: *mut elf_symbol_type = 0 as *mut elf_symbol_type;
    elf_sym = if (*sym).flags & ((1 as libc::c_int) << 21 as libc::c_int) as libc::c_uint
        == 0 as libc::c_int as libc::c_uint && !((*sym).the_bfd).is_null()
        && (*(*(*sym).the_bfd).xvec).flavour as libc::c_uint
            == bfd_target_elf_flavour as libc::c_int as libc::c_uint
        && !((*(*sym).the_bfd).tdata.elf_obj_data).is_null()
    {
        sym as *mut elf_symbol_type
    } else {
        0 as *mut elf_symbol_type
    };
    if !elf_sym.is_null() {
        match (*elf_sym).internal_elf_sym.st_other as libc::c_int & 0x3 as libc::c_int {
            2 | 1 => return 1 as libc::c_int != 0,
            _ => {}
        }
    }
    return 0 as libc::c_int != 0;
}
static mut empty_name: *const libc::c_char = b"\0" as *const u8 as *const libc::c_char;
unsafe extern "C" fn need_sym_before(
    mut node: *mut *mut addsym_node,
    mut sym: *const libc::c_char,
) -> bool {
    let mut count: libc::c_int = 0;
    let mut ptr: *mut addsym_node = add_sym_list;
    count = 0 as libc::c_int;
    while count < add_symbols {
        if ((*ptr).othersym).is_null() {
            break;
        }
        if !((*ptr).othersym == empty_name) {
            if strcmp((*ptr).othersym, sym) == 0 as libc::c_int {
                free((*ptr).othersym as *mut libc::c_char as *mut libc::c_void);
                (*ptr).othersym = empty_name;
                *node = ptr;
                return 1 as libc::c_int != 0;
            }
            ptr = (*ptr).next;
        }
        count += 1;
        count;
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn create_new_symbol(
    mut ptr: *mut addsym_node,
    mut obfd: *mut bfd,
) -> *mut asymbol {
    let mut sym: *mut asymbol = (Some(
        ((*(*obfd).xvec)._bfd_make_empty_symbol).expect("non-null function pointer"),
    ))
        .expect("non-null function pointer")(obfd);
    bfd_set_asymbol_name(sym, (*ptr).symdef);
    (*sym).value = (*ptr).symval as symvalue;
    (*sym).flags = (*ptr).flags;
    if !((*ptr).section).is_null() {
        let mut sec: *mut asection = bfd_get_section_by_name(obfd, (*ptr).section);
        if sec.is_null() {
            fatal(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Section %s not found\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*ptr).section,
            );
        }
        (*sym).section = sec;
    } else {
        (*sym)
            .section = &mut *_bfd_std_section
            .as_mut_ptr()
            .offset(2 as libc::c_int as isize) as *mut asection;
    }
    return sym;
}
unsafe extern "C" fn filter_symbols(
    mut abfd: *mut bfd,
    mut obfd: *mut bfd,
    mut osyms: *mut *mut asymbol,
    mut isyms: *mut *mut asymbol,
    mut symcount: libc::c_long,
) -> libc::c_uint {
    let mut from: *mut *mut asymbol = isyms;
    let mut to: *mut *mut asymbol = osyms;
    let mut src_count: libc::c_long = 0 as libc::c_int as libc::c_long;
    let mut dst_count: libc::c_long = 0 as libc::c_int as libc::c_long;
    let mut relocatable: libc::c_int = ((*abfd).flags
        & (0x2 as libc::c_int | 0x40 as libc::c_int) as libc::c_uint
        == 0 as libc::c_int as libc::c_uint) as libc::c_int;
    while src_count < symcount {
        let mut sym: *mut asymbol = *from.offset(src_count as isize);
        let mut flags: flagword = (*sym).flags;
        let mut name: *mut libc::c_char = bfd_asymbol_name(sym) as *mut libc::c_char;
        let mut keep: bool = false;
        let mut used_in_reloc: bool = 0 as libc::c_int != 0;
        let mut undefined: bool = false;
        let mut rem_leading_char: bool = false;
        let mut add_leading_char: bool = false;
        undefined = bfd_is_und_section(bfd_asymbol_section(sym));
        if !add_sym_list.is_null() {
            let mut ptr: *mut addsym_node = 0 as *mut addsym_node;
            if need_sym_before(&mut ptr, name) {
                let fresh7 = dst_count;
                dst_count = dst_count + 1;
                let ref mut fresh8 = *to.offset(fresh7 as isize);
                *fresh8 = create_new_symbol(ptr, obfd);
            }
        }
        if htab_elements(redefine_specific_htab) != 0 || !section_rename_list.is_null() {
            let mut new_name: *mut libc::c_char = 0 as *mut libc::c_char;
            new_name = lookup_sym_redefinition(name) as *mut libc::c_char;
            if new_name == name
                && flags & ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_uint
                    != 0 as libc::c_int as libc::c_uint
            {
                new_name = find_section_rename(name, 0 as *mut flagword)
                    as *mut libc::c_char;
            }
            bfd_set_asymbol_name(sym, new_name);
            name = new_name;
        }
        rem_leading_char = *name.offset(0 as libc::c_int as isize) as libc::c_int
            != '\0' as i32
            && *name.offset(0 as libc::c_int as isize) as libc::c_int
                == bfd_get_symbol_leading_char(abfd) as libc::c_int
            && (change_leading_char as libc::c_int != 0
                || remove_leading_char as libc::c_int != 0
                    && (flags
                        & ((1 as libc::c_int) << 1 as libc::c_int
                            | (1 as libc::c_int) << 7 as libc::c_int) as libc::c_uint
                        != 0 as libc::c_int as libc::c_uint
                        || undefined as libc::c_int != 0
                        || bfd_is_com_section(bfd_asymbol_section(sym)) as libc::c_int
                            != 0));
        add_leading_char = change_leading_char as libc::c_int != 0
            && bfd_get_symbol_leading_char(obfd) as libc::c_int != '\0' as i32
            && (bfd_get_symbol_leading_char(abfd) as libc::c_int == '\0' as i32
                || *name.offset(0 as libc::c_int as isize) as libc::c_int
                    == bfd_get_symbol_leading_char(abfd) as libc::c_int);
        if rem_leading_char as libc::c_int != 0 && add_leading_char as libc::c_int != 0
            && prefix_symbols_string.is_null()
        {
            *name.offset(0 as libc::c_int as isize) = bfd_get_symbol_leading_char(obfd);
            bfd_set_asymbol_name(sym, name);
            rem_leading_char = 0 as libc::c_int != 0;
            add_leading_char = 0 as libc::c_int != 0;
        }
        if rem_leading_char {
            name = name.offset(1);
            bfd_set_asymbol_name(sym, name);
        }
        if add_leading_char as libc::c_int != 0 || !prefix_symbols_string.is_null() {
            let mut n: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut ptr_0: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut len: size_t = (strlen(name))
                .wrapping_add(1 as libc::c_int as libc::c_ulong);
            if add_leading_char {
                len = len.wrapping_add(1);
                len;
            }
            if !prefix_symbols_string.is_null() {
                len = (len as libc::c_ulong).wrapping_add(strlen(prefix_symbols_string))
                    as size_t as size_t;
            }
            n = xmalloc(len) as *mut libc::c_char;
            ptr_0 = n;
            if add_leading_char {
                let fresh9 = ptr_0;
                ptr_0 = ptr_0.offset(1);
                *fresh9 = bfd_get_symbol_leading_char(obfd);
            }
            if !prefix_symbols_string.is_null() {
                strcpy(ptr_0, prefix_symbols_string);
                ptr_0 = ptr_0.offset(strlen(prefix_symbols_string) as isize);
            }
            strcpy(ptr_0, name);
            bfd_set_asymbol_name(sym, n);
            name = n;
        }
        if strip_symbols as libc::c_uint == STRIP_ALL as libc::c_int as libc::c_uint {
            keep = 0 as libc::c_int != 0;
        } else if flags & ((1 as libc::c_int) << 5 as libc::c_int) as libc::c_uint
            != 0 as libc::c_int as libc::c_uint
            || flags & ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_uint
                != 0 as libc::c_int as libc::c_uint
                && (**(*bfd_asymbol_section(sym)).symbol_ptr_ptr).flags
                    & ((1 as libc::c_int) << 5 as libc::c_int) as libc::c_uint
                    != 0 as libc::c_int as libc::c_uint
        {
            keep = 1 as libc::c_int != 0;
            used_in_reloc = 1 as libc::c_int != 0;
        } else if relocatable != 0
            && (flags
                & ((1 as libc::c_int) << 1 as libc::c_int
                    | (1 as libc::c_int) << 7 as libc::c_int) as libc::c_uint
                != 0 as libc::c_int as libc::c_uint
                || bfd_is_com_section(bfd_asymbol_section(sym)) as libc::c_int != 0)
        {
            keep = 1 as libc::c_int != 0;
        } else if bfd_decode_symclass(sym) == 'I' as i32 {
            keep = 1 as libc::c_int != 0;
        } else if flags & ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uint
            != 0 as libc::c_int as libc::c_uint
            || flags & ((1 as libc::c_int) << 7 as libc::c_int) as libc::c_uint
                != 0 as libc::c_int as libc::c_uint || undefined as libc::c_int != 0
            || bfd_is_com_section(bfd_asymbol_section(sym)) as libc::c_int != 0
        {
            keep = strip_symbols as libc::c_uint
                != STRIP_UNNEEDED as libc::c_int as libc::c_uint;
        } else if flags & ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uint
            != 0 as libc::c_int as libc::c_uint
        {
            keep = strip_symbols as libc::c_uint
                != STRIP_DEBUG as libc::c_int as libc::c_uint
                && strip_symbols as libc::c_uint
                    != STRIP_UNNEEDED as libc::c_int as libc::c_uint
                && !convert_debugging;
        } else if !if bfd_get_flavour(abfd) as libc::c_uint
            == bfd_target_coff_flavour as libc::c_int as libc::c_uint
            && !((*bfd_asymbol_section(sym)).used_by_bfd as *mut coff_section_tdata)
                .is_null()
        {
            (*((*bfd_asymbol_section(sym)).used_by_bfd as *mut coff_section_tdata))
                .comdat
        } else {
            0 as *mut coff_comdat_info
        }
            .is_null()
        {
            keep = 1 as libc::c_int != 0;
        } else {
            keep = strip_symbols as libc::c_uint
                != STRIP_UNNEEDED as libc::c_int as libc::c_uint
                && (discard_locals as libc::c_uint
                    != LOCALS_ALL as libc::c_int as libc::c_uint
                    && (discard_locals as libc::c_uint
                        != LOCALS_START_L as libc::c_int as libc::c_uint
                        || !bfd_is_local_label(abfd, sym)));
        }
        if keep as libc::c_int != 0
            && is_specified_symbol(name, strip_specific_htab) as libc::c_int != 0
        {
            if used_in_reloc {
                non_fatal(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"not stripping symbol `%s' because it is named in a relocation\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    name,
                );
                status = 1 as libc::c_int;
            } else {
                keep = 0 as libc::c_int != 0;
            }
        }
        if keep as libc::c_int != 0
            && flags & ((1 as libc::c_int) << 5 as libc::c_int) as libc::c_uint == 0
            && is_specified_symbol(name, strip_unneeded_htab) as libc::c_int != 0
        {
            keep = 0 as libc::c_int != 0;
        }
        if !keep
            && (keep_file_symbols as libc::c_int != 0
                && flags & ((1 as libc::c_int) << 14 as libc::c_int) as libc::c_uint != 0
                || is_specified_symbol(name, keep_specific_htab) as libc::c_int != 0)
        {
            keep = 1 as libc::c_int != 0;
        }
        if keep as libc::c_int != 0
            && is_strip_section(abfd, bfd_asymbol_section(sym)) as libc::c_int != 0
        {
            keep = 0 as libc::c_int != 0;
        }
        if keep {
            if (flags & ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uint
                != 0 as libc::c_int as libc::c_uint || undefined as libc::c_int != 0)
                && (weaken as libc::c_int != 0
                    || is_specified_symbol(name, weaken_specific_htab) as libc::c_int
                        != 0)
            {
                (*sym).flags
                    &= !((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uint;
                (*sym).flags |= ((1 as libc::c_int) << 7 as libc::c_int) as libc::c_uint;
            }
            if !undefined
                && flags
                    & ((1 as libc::c_int) << 1 as libc::c_int
                        | (1 as libc::c_int) << 7 as libc::c_int) as libc::c_uint != 0
                && (is_specified_symbol(name, localize_specific_htab) as libc::c_int != 0
                    || htab_elements(keepglobal_specific_htab)
                        != 0 as libc::c_int as libc::c_ulong
                        && !is_specified_symbol(name, keepglobal_specific_htab)
                    || localize_hidden as libc::c_int != 0
                        && is_hidden_symbol(sym) as libc::c_int != 0)
            {
                (*sym).flags
                    &= !((1 as libc::c_int) << 1 as libc::c_int
                        | (1 as libc::c_int) << 7 as libc::c_int) as libc::c_uint;
                (*sym).flags |= ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint;
            }
            if !undefined
                && flags & ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint != 0
                && is_specified_symbol(name, globalize_specific_htab) as libc::c_int != 0
            {
                (*sym).flags
                    &= !((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint;
                (*sym).flags |= ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uint;
            }
            let fresh10 = dst_count;
            dst_count = dst_count + 1;
            let ref mut fresh11 = *to.offset(fresh10 as isize);
            *fresh11 = sym;
        }
        src_count += 1;
        src_count;
    }
    if !add_sym_list.is_null() {
        let mut ptr_1: *mut addsym_node = add_sym_list;
        src_count = 0 as libc::c_int as libc::c_long;
        while src_count < add_symbols as libc::c_long {
            if !((*ptr_1).othersym).is_null() {
                if (*ptr_1).othersym != empty_name {
                    fatal(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"'before=%s' not found\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        (*ptr_1).othersym,
                    );
                }
            } else {
                let fresh12 = dst_count;
                dst_count = dst_count + 1;
                let ref mut fresh13 = *to.offset(fresh12 as isize);
                *fresh13 = create_new_symbol(ptr_1, obfd);
            }
            ptr_1 = (*ptr_1).next;
            src_count += 1;
            src_count;
        }
    }
    let ref mut fresh14 = *to.offset(dst_count as isize);
    *fresh14 = 0 as *mut asymbol;
    return dst_count as libc::c_uint;
}
unsafe extern "C" fn add_redefine_and_check(
    mut cause: *const libc::c_char,
    mut source: *const libc::c_char,
    mut target: *const libc::c_char,
) {
    let mut new_node: *mut redefine_node = xmalloc(
        ::core::mem::size_of::<redefine_node>() as libc::c_ulong,
    ) as *mut redefine_node;
    (*new_node).source = strdup(source);
    (*new_node).target = strdup(target);
    if !(htab_find(redefine_specific_htab, new_node as *const libc::c_void)).is_null() {
        fatal(
            dcgettext(
                0 as *const libc::c_char,
                b"%s: Multiple redefinition of symbol \"%s\"\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            cause,
            source,
        );
    }
    if !(htab_find(redefine_specific_reverse_htab, target as *const libc::c_void))
        .is_null()
    {
        fatal(
            dcgettext(
                0 as *const libc::c_char,
                b"%s: Symbol \"%s\" is target of more than one redefinition\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            cause,
            target,
        );
    }
    add_specific_symbol_node(new_node as *const libc::c_void, redefine_specific_htab);
    add_specific_symbol((*new_node).target, redefine_specific_reverse_htab);
}
unsafe extern "C" fn add_redefine_syms_file(mut filename: *const libc::c_char) {
    let mut current_block: u64;
    let mut file: *mut FILE = 0 as *mut FILE;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut bufsize: size_t = 0;
    let mut len: size_t = 0;
    let mut outsym_off: size_t = 0;
    let mut c: libc::c_int = 0;
    let mut lineno: libc::c_int = 0;
    file = fopen(filename, b"r\0" as *const u8 as *const libc::c_char);
    if file.is_null() {
        fatal(
            dcgettext(
                0 as *const libc::c_char,
                b"couldn't open symbol redefinition file %s (error: %s)\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            filename,
            strerror(*__errno_location()),
        );
    }
    bufsize = 100 as libc::c_int as size_t;
    buf = xmalloc(bufsize.wrapping_add(1 as libc::c_int as libc::c_ulong))
        as *mut libc::c_char;
    lineno = 1 as libc::c_int;
    c = getc(file);
    len = 0 as libc::c_int as size_t;
    outsym_off = 0 as libc::c_int as size_t;
    while c != -(1 as libc::c_int) {
        loop {
            if !(!(c == ' ' as i32 || c == '\t' as i32)
                && !(c == '\n' as i32 || c == '\r' as i32 || c == '\0' as i32)
                && c != -(1 as libc::c_int))
            {
                current_block = 9606288038608642794;
                break;
            }
            if c == '#' as i32 {
                current_block = 13220371211661906836;
                break;
            }
            let fresh15 = len;
            len = len.wrapping_add(1);
            *buf.offset(fresh15 as isize) = c as libc::c_char;
            if len >= bufsize {
                bufsize = (bufsize as libc::c_ulong)
                    .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
                buf = xrealloc(
                    buf as *mut libc::c_void,
                    bufsize.wrapping_add(1 as libc::c_int as libc::c_ulong),
                ) as *mut libc::c_char;
            }
            c = getc(file);
        }
        match current_block {
            9606288038608642794 => {
                let fresh16 = len;
                len = len.wrapping_add(1);
                *buf.offset(fresh16 as isize) = '\0' as i32 as libc::c_char;
                if c == -(1 as libc::c_int) {
                    break;
                }
                while c == ' ' as i32 || c == '\t' as i32 {
                    c = getc(file);
                }
                if c == '#' as i32
                    || (c == '\n' as i32 || c == '\r' as i32 || c == '\0' as i32)
                {
                    current_block = 13220371211661906836;
                } else {
                    if c == -(1 as libc::c_int) {
                        break;
                    }
                    outsym_off = len;
                    loop {
                        if !(!(c == ' ' as i32 || c == '\t' as i32)
                            && !(c == '\n' as i32 || c == '\r' as i32
                                || c == '\0' as i32) && c != -(1 as libc::c_int))
                        {
                            current_block = 14648156034262866959;
                            break;
                        }
                        if c == '#' as i32 {
                            current_block = 13220371211661906836;
                            break;
                        }
                        let fresh17 = len;
                        len = len.wrapping_add(1);
                        *buf.offset(fresh17 as isize) = c as libc::c_char;
                        if len >= bufsize {
                            bufsize = (bufsize as libc::c_ulong)
                                .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t
                                as size_t;
                            buf = xrealloc(
                                buf as *mut libc::c_void,
                                bufsize.wrapping_add(1 as libc::c_int as libc::c_ulong),
                            ) as *mut libc::c_char;
                        }
                        c = getc(file);
                    }
                    match current_block {
                        13220371211661906836 => {}
                        _ => {
                            let fresh18 = len;
                            len = len.wrapping_add(1);
                            *buf.offset(fresh18 as isize) = '\0' as i32 as libc::c_char;
                            if c == -(1 as libc::c_int) {
                                break;
                            }
                            while !(c == '\n' as i32 || c == '\r' as i32
                                || c == '\0' as i32) && c != -(1 as libc::c_int)
                                && (c == ' ' as i32 || c == '\t' as i32)
                            {
                                c = getc(file);
                            }
                            if c == '#' as i32 {
                                current_block = 13220371211661906836;
                            } else if c == '\r' as i32
                                && {
                                    c = getc(file);
                                    c == '\n' as i32
                                } || c == '\n' as i32 || c == -(1 as libc::c_int)
                            {
                                current_block = 8735804898826114203;
                            } else {
                                fatal(
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"%s:%d: garbage found at end of line\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                    filename,
                                    lineno,
                                );
                            }
                        }
                    }
                }
            }
            _ => {}
        }
        match current_block {
            13220371211661906836 => {
                if len != 0 as libc::c_int as libc::c_ulong
                    && (outsym_off == 0 as libc::c_int as libc::c_ulong
                        || outsym_off == len)
                {
                    fatal(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"%s:%d: missing new symbol name\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        filename,
                        lineno,
                    );
                }
                let fresh19 = len;
                len = len.wrapping_add(1);
                *buf.offset(fresh19 as isize) = '\0' as i32 as libc::c_char;
                while c != '\n' as i32 && c != -(1 as libc::c_int) {
                    c = getc(file);
                }
            }
            _ => {}
        }
        if *buf.offset(0 as libc::c_int as isize) as libc::c_int != '\0' as i32 {
            add_redefine_and_check(
                filename,
                &mut *buf.offset(0 as libc::c_int as isize),
                &mut *buf.offset(outsym_off as isize),
            );
        }
        lineno += 1;
        lineno;
        len = 0 as libc::c_int as size_t;
        outsym_off = 0 as libc::c_int as size_t;
        if c == -(1 as libc::c_int) {
            break;
        }
        c = getc(file);
    }
    if len != 0 as libc::c_int as libc::c_ulong {
        fatal(
            dcgettext(
                0 as *const libc::c_char,
                b"%s:%d: premature end of file\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            filename,
            lineno,
        );
    }
    free(buf as *mut libc::c_void);
    fclose(file);
}
unsafe extern "C" fn copy_unknown_object(
    mut ibfd: *mut bfd,
    mut obfd: *mut bfd,
) -> bool {
    let mut cbuf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tocopy: libc::c_int = 0;
    let mut ncopied: libc::c_long = 0;
    let mut size: libc::c_long = 0;
    let mut buf: stat = stat {
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
    if (Some(
        ((*(*(if !((*ibfd).my_archive).is_null() { (*ibfd).my_archive } else { ibfd }))
            .xvec)
            ._bfd_stat_arch_elt)
            .expect("non-null function pointer"),
    ))
        .expect("non-null function pointer")(ibfd, &mut buf) != 0 as libc::c_int
    {
        bfd_nonfatal_message(
            0 as *const libc::c_char,
            ibfd,
            0 as *const asection,
            0 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    size = buf.st_size;
    if size < 0 as libc::c_int as libc::c_long {
        non_fatal(
            dcgettext(
                0 as *const libc::c_char,
                b"stat returns negative size for `%s'\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            bfd_get_archive_filename(ibfd),
        );
        return 0 as libc::c_int != 0;
    }
    if bfd_seek(ibfd, 0 as libc::c_int as file_ptr, 0 as libc::c_int) != 0 as libc::c_int
    {
        bfd_nonfatal(bfd_get_archive_filename(ibfd));
        return 0 as libc::c_int != 0;
    }
    if verbose {
        printf(
            dcgettext(
                0 as *const libc::c_char,
                b"copy from `%s' [unknown] to `%s' [unknown]\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            bfd_get_archive_filename(ibfd),
            bfd_get_filename(obfd),
        );
    }
    cbuf = xmalloc(8192 as libc::c_int as size_t) as *mut libc::c_char;
    ncopied = 0 as libc::c_int as libc::c_long;
    while ncopied < size {
        tocopy = (size - ncopied) as libc::c_int;
        if tocopy > 8192 as libc::c_int {
            tocopy = 8192 as libc::c_int;
        }
        if bfd_bread(cbuf as *mut libc::c_void, tocopy as bfd_size_type, ibfd)
            != tocopy as bfd_size_type
        {
            bfd_nonfatal_message(
                0 as *const libc::c_char,
                ibfd,
                0 as *const asection,
                0 as *const libc::c_char,
            );
            free(cbuf as *mut libc::c_void);
            return 0 as libc::c_int != 0;
        }
        if bfd_bwrite(cbuf as *const libc::c_void, tocopy as bfd_size_type, obfd)
            != tocopy as bfd_size_type
        {
            bfd_nonfatal_message(
                0 as *const libc::c_char,
                obfd,
                0 as *const asection,
                0 as *const libc::c_char,
            );
            free(cbuf as *mut libc::c_void);
            return 0 as libc::c_int != 0;
        }
        ncopied += tocopy as libc::c_long;
    }
    chmod(bfd_get_filename(obfd), buf.st_mode | 0o400 as libc::c_int as libc::c_uint);
    free(cbuf as *mut libc::c_void);
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn overlaps_or_adjoins(
    mut pnote1: *mut objcopy_internal_note,
    mut pnote2: *mut objcopy_internal_note,
) -> bool {
    if (*pnote1).end < (*pnote2).start {
        return (if ((*pnote1).end)
            .wrapping_add(16 as libc::c_int as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong) >= (*pnote1).end
        {
            ((*pnote1).end)
                .wrapping_add((16 as libc::c_int - 1 as libc::c_int) as libc::c_ulong)
                & !((16 as libc::c_int - 1 as libc::c_int) as bfd_vma)
        } else {
            !(0 as libc::c_int as bfd_vma)
        }) < (*pnote2).start;
    }
    if (*pnote2).end < (*pnote2).start {
        return (if ((*pnote2).end)
            .wrapping_add(16 as libc::c_int as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong) >= (*pnote2).end
        {
            ((*pnote2).end)
                .wrapping_add((16 as libc::c_int - 1 as libc::c_int) as libc::c_ulong)
                & !((16 as libc::c_int - 1 as libc::c_int) as bfd_vma)
        } else {
            !(0 as libc::c_int as bfd_vma)
        }) < (*pnote1).start;
    }
    if (*pnote1).end < (*pnote2).end {
        return 1 as libc::c_int != 0;
    }
    if (*pnote2).end < (*pnote1).end {
        return 1 as libc::c_int != 0;
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn contained_by(
    mut needle: *mut objcopy_internal_note,
    mut haystack: *mut objcopy_internal_note,
) -> bool {
    return (*needle).start >= (*haystack).start && (*needle).end <= (*haystack).end;
}
unsafe extern "C" fn is_open_note(mut pnote: *mut objcopy_internal_note) -> bool {
    return (*pnote).note.type_0 == 0x100 as libc::c_int as libc::c_ulong;
}
unsafe extern "C" fn is_func_note(mut pnote: *mut objcopy_internal_note) -> bool {
    return (*pnote).note.type_0 == 0x101 as libc::c_int as libc::c_ulong;
}
unsafe extern "C" fn is_deleted_note(mut pnote: *mut objcopy_internal_note) -> bool {
    return (*pnote).note.type_0 == 0 as libc::c_int as libc::c_ulong;
}
unsafe extern "C" fn is_version_note(mut pnote: *mut objcopy_internal_note) -> bool {
    return (*pnote).note.namesz > 4 as libc::c_int as libc::c_ulong
        && *((*pnote).note.namedata).offset(0 as libc::c_int as isize) as libc::c_int
            == 'G' as i32
        && *((*pnote).note.namedata).offset(1 as libc::c_int as isize) as libc::c_int
            == 'A' as i32
        && *((*pnote).note.namedata).offset(2 as libc::c_int as isize) as libc::c_int
            == '$' as i32
        && *((*pnote).note.namedata).offset(3 as libc::c_int as isize) as libc::c_int
            == 1 as libc::c_int;
}
unsafe extern "C" fn is_64bit(mut abfd: *mut bfd) -> bool {
    if bfd_get_flavour(abfd) as libc::c_uint
        != bfd_target_elf_flavour as libc::c_int as libc::c_uint
    {
        return 0 as libc::c_int != 0;
    }
    return (*((*(*abfd).tdata.elf_obj_data).elf_header).as_mut_ptr())
        .e_ident[4 as libc::c_int as usize] as libc::c_int == 2 as libc::c_int;
}
unsafe extern "C" fn compare_gnu_build_notes(
    mut data1: *const libc::c_void,
    mut data2: *const libc::c_void,
) -> libc::c_int {
    let mut pnote1: *mut objcopy_internal_note = data1 as *mut objcopy_internal_note;
    let mut pnote2: *mut objcopy_internal_note = data2 as *mut objcopy_internal_note;
    let mut cmp: libc::c_int = memcmp(
        ((*pnote1).note.namedata).offset(3 as libc::c_int as isize)
            as *const libc::c_void,
        ((*pnote2).note.namedata).offset(3 as libc::c_int as isize)
            as *const libc::c_void,
        if (*pnote1).note.namesz < (*pnote2).note.namesz {
            ((*pnote1).note.namesz).wrapping_sub(3 as libc::c_int as libc::c_ulong)
        } else {
            ((*pnote2).note.namesz).wrapping_sub(3 as libc::c_int as libc::c_ulong)
        },
    );
    if cmp != 0 {
        return cmp;
    }
    if (*pnote1).end < (*pnote2).start {
        return -(1 as libc::c_int);
    }
    if (*pnote1).start > (*pnote2).end {
        return 1 as libc::c_int;
    }
    if (*pnote1).start < (*pnote2).start {
        return -(1 as libc::c_int);
    }
    if (*pnote1).end > (*pnote2).end {
        return 1 as libc::c_int;
    }
    if (*pnote1).end < (*pnote2).end {
        return -(1 as libc::c_int);
    }
    if is_open_note(pnote1) as libc::c_int != 0 && !is_open_note(pnote2) {
        return -(1 as libc::c_int);
    }
    if !is_open_note(pnote1) && is_open_note(pnote2) as libc::c_int != 0 {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn sort_gnu_build_notes(
    mut data1: *const libc::c_void,
    mut data2: *const libc::c_void,
) -> libc::c_int {
    let mut pnote1: *mut objcopy_internal_note = data1 as *mut objcopy_internal_note;
    let mut pnote2: *mut objcopy_internal_note = data2 as *mut objcopy_internal_note;
    if (*pnote1).note.type_0 != (*pnote2).note.type_0 {
        if is_deleted_note(pnote1) {
            return 1 as libc::c_int;
        }
        if is_open_note(pnote1) {
            return -(1 as libc::c_int);
        }
        if is_deleted_note(pnote2) {
            return -(1 as libc::c_int);
        }
        return 1 as libc::c_int;
    }
    if (*pnote1).start < (*pnote2).start {
        return -(1 as libc::c_int);
    }
    if (*pnote1).start > (*pnote2).start {
        return 1 as libc::c_int;
    }
    if (*pnote1).end > (*pnote2).end {
        return -(1 as libc::c_int);
    }
    if (*pnote1).end < (*pnote2).end {
        return 1 as libc::c_int;
    }
    if (*pnote1).note.namesz > 4 as libc::c_int as libc::c_ulong
        && (*pnote2).note.namesz > 4 as libc::c_int as libc::c_ulong
        && *((*pnote1).note.namedata).offset(3 as libc::c_int as isize) as libc::c_int
            != *((*pnote2).note.namedata).offset(3 as libc::c_int as isize)
                as libc::c_int
    {
        return *((*pnote1).note.namedata).offset(3 as libc::c_int as isize)
            as libc::c_int
            - *((*pnote2).note.namedata).offset(3 as libc::c_int as isize)
                as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn merge_gnu_build_notes(
    mut abfd: *mut bfd,
    mut sec: *mut asection,
    mut size: bfd_size_type,
    mut contents: *mut bfd_byte,
) -> bfd_size_type {
    let mut new_contents: *mut bfd_byte = 0 as *mut bfd_byte;
    let mut old: *mut bfd_byte = 0 as *mut bfd_byte;
    let mut new: *mut bfd_byte = 0 as *mut bfd_byte;
    let mut new_size: bfd_size_type = 0;
    let mut prev_start: bfd_vma = 0;
    let mut prev_end: bfd_vma = 0;
    let mut current_block: u64;
    let mut pnotes_end: *mut objcopy_internal_note = 0 as *mut objcopy_internal_note;
    let mut pnotes: *mut objcopy_internal_note = 0 as *mut objcopy_internal_note;
    let mut pnote: *mut objcopy_internal_note = 0 as *mut objcopy_internal_note;
    let mut remain: bfd_size_type = size;
    let mut version_1_seen: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut version_2_seen: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut version_3_seen: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut err: *const libc::c_char = 0 as *const libc::c_char;
    let mut in_0: *mut bfd_byte = contents;
    let mut previous_func_start: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut previous_open_start: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut previous_func_end: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut previous_open_end: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut relsize: libc::c_long = 0;
    relsize = bfd_get_reloc_upper_bound(abfd, sec);
    if relsize > 0 as libc::c_int as libc::c_long {
        let mut relpp: *mut *mut arelent = 0 as *mut *mut arelent;
        let mut relcount: libc::c_long = 0;
        relpp = xmalloc(relsize as size_t) as *mut *mut arelent;
        relcount = bfd_canonicalize_reloc(abfd, sec, relpp, isympp);
        free(relpp as *mut libc::c_void);
        if relcount != 0 as libc::c_int as libc::c_long {
            if is_strip == 0 {
                non_fatal(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s[%s]: Cannot merge - there are relocations against this section\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    bfd_get_filename(abfd),
                    bfd_section_name(sec),
                );
            }
            current_block = 4462513327734392438;
        } else {
            current_block = 11050875288958768710;
        }
    } else {
        current_block = 11050875288958768710;
    }
    match current_block {
        11050875288958768710 => {
            pnotes = xcalloc(
                size.wrapping_div(12 as libc::c_int as libc::c_ulong),
                ::core::mem::size_of::<objcopy_internal_note>() as libc::c_ulong,
            ) as *mut objcopy_internal_note;
            pnote = pnotes;
            loop {
                if !(remain >= 12 as libc::c_int as libc::c_ulong) {
                    current_block = 16203797167131938757;
                    break;
                }
                let mut start: bfd_vma = 0;
                let mut end: bfd_vma = 0;
                (*pnote)
                    .note
                    .namesz = (Some(
                    ((*(*abfd).xvec).bfd_getx32).expect("non-null function pointer"),
                ))
                    .expect("non-null function pointer")(in_0 as *const libc::c_void);
                (*pnote)
                    .note
                    .descsz = (Some(
                    ((*(*abfd).xvec).bfd_getx32).expect("non-null function pointer"),
                ))
                    .expect(
                        "non-null function pointer",
                    )(in_0.offset(4 as libc::c_int as isize) as *const libc::c_void);
                (*pnote)
                    .note
                    .type_0 = (Some(
                    ((*(*abfd).xvec).bfd_getx32).expect("non-null function pointer"),
                ))
                    .expect(
                        "non-null function pointer",
                    )(in_0.offset(8 as libc::c_int as isize) as *const libc::c_void);
                (*pnote)
                    .padded_namesz = ((*pnote).note.namesz)
                    .wrapping_add(3 as libc::c_int as libc::c_ulong)
                    & !(3 as libc::c_int) as libc::c_ulong;
                if ((*pnote).note.descsz).wrapping_add(3 as libc::c_int as libc::c_ulong)
                    & !(3 as libc::c_int) as libc::c_ulong != (*pnote).note.descsz
                {
                    err = dcgettext(
                        0 as *const libc::c_char,
                        b"corrupt GNU build attribute note: description size not a factor of 4\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    );
                    current_block = 4462513327734392438;
                    break;
                } else if (*pnote).note.type_0 != 0x100 as libc::c_int as libc::c_ulong
                    && (*pnote).note.type_0 != 0x101 as libc::c_int as libc::c_ulong
                {
                    err = dcgettext(
                        0 as *const libc::c_char,
                        b"corrupt GNU build attribute note: wrong note type\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    );
                    current_block = 4462513327734392438;
                    break;
                } else if ((*pnote).padded_namesz)
                    .wrapping_add((*pnote).note.descsz)
                    .wrapping_add(12 as libc::c_int as libc::c_ulong) > remain
                {
                    err = dcgettext(
                        0 as *const libc::c_char,
                        b"corrupt GNU build attribute note: note too big\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    );
                    current_block = 4462513327734392438;
                    break;
                } else if (*pnote).note.namesz < 2 as libc::c_int as libc::c_ulong {
                    err = dcgettext(
                        0 as *const libc::c_char,
                        b"corrupt GNU build attribute note: name too small\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    );
                    current_block = 4462513327734392438;
                    break;
                } else {
                    (*pnote)
                        .note
                        .namedata = in_0.offset(12 as libc::c_int as isize)
                        as *mut libc::c_char;
                    (*pnote)
                        .note
                        .descdata = in_0
                        .offset(12 as libc::c_int as isize)
                        .offset((*pnote).padded_namesz as isize) as *mut libc::c_char;
                    remain = (remain as libc::c_ulong)
                        .wrapping_sub(
                            (12 as libc::c_int as libc::c_ulong)
                                .wrapping_add((*pnote).padded_namesz)
                                .wrapping_add((*pnote).note.descsz),
                        ) as bfd_size_type as bfd_size_type;
                    in_0 = in_0
                        .offset(
                            (12 as libc::c_int as libc::c_ulong)
                                .wrapping_add((*pnote).padded_namesz)
                                .wrapping_add((*pnote).note.descsz) as isize,
                        );
                    if (*pnote).note.namesz > 2 as libc::c_int as libc::c_ulong
                        && *((*pnote).note.namedata).offset(0 as libc::c_int as isize)
                            as libc::c_int == '$' as i32
                        && *((*pnote).note.namedata).offset(1 as libc::c_int as isize)
                            as libc::c_int == 1 as libc::c_int
                        && *((*pnote).note.namedata).offset(2 as libc::c_int as isize)
                            as libc::c_int == '1' as i32
                    {
                        version_1_seen = version_1_seen.wrapping_add(1);
                        version_1_seen;
                    } else if is_version_note(pnote) {
                        if *((*pnote).note.namedata).offset(4 as libc::c_int as isize)
                            as libc::c_int == '2' as i32
                        {
                            version_2_seen = version_2_seen.wrapping_add(1);
                            version_2_seen;
                        } else if *((*pnote).note.namedata)
                            .offset(4 as libc::c_int as isize) as libc::c_int
                            == '3' as i32
                        {
                            version_3_seen = version_3_seen.wrapping_add(1);
                            version_3_seen;
                        } else {
                            err = dcgettext(
                                0 as *const libc::c_char,
                                b"corrupt GNU build attribute note: unsupported version\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            );
                            current_block = 4462513327734392438;
                            break;
                        }
                    }
                    match (*pnote).note.descsz {
                        0 => {
                            end = 0 as libc::c_int as bfd_vma;
                            start = end;
                        }
                        4 => {
                            start = (Some(
                                ((*(*abfd).xvec).bfd_getx32)
                                    .expect("non-null function pointer"),
                            ))
                                .expect(
                                    "non-null function pointer",
                                )((*pnote).note.descdata as *const libc::c_void);
                            end = -(1 as libc::c_int) as bfd_vma;
                        }
                        8 => {
                            start = (Some(
                                ((*(*abfd).xvec).bfd_getx32)
                                    .expect("non-null function pointer"),
                            ))
                                .expect(
                                    "non-null function pointer",
                                )((*pnote).note.descdata as *const libc::c_void);
                            end = (Some(
                                ((*(*abfd).xvec).bfd_getx32)
                                    .expect("non-null function pointer"),
                            ))
                                .expect(
                                    "non-null function pointer",
                                )(
                                ((*pnote).note.descdata).offset(4 as libc::c_int as isize)
                                    as *const libc::c_void,
                            );
                        }
                        16 => {
                            start = (Some(
                                ((*(*abfd).xvec).bfd_getx64)
                                    .expect("non-null function pointer"),
                            ))
                                .expect(
                                    "non-null function pointer",
                                )((*pnote).note.descdata as *const libc::c_void);
                            end = (Some(
                                ((*(*abfd).xvec).bfd_getx64)
                                    .expect("non-null function pointer"),
                            ))
                                .expect(
                                    "non-null function pointer",
                                )(
                                ((*pnote).note.descdata).offset(8 as libc::c_int as isize)
                                    as *const libc::c_void,
                            );
                        }
                        _ => {
                            err = dcgettext(
                                0 as *const libc::c_char,
                                b"corrupt GNU build attribute note: bad description size\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            );
                            current_block = 4462513327734392438;
                            break;
                        }
                    }
                    if start > end {
                        start = end;
                    }
                    if is_open_note(pnote) {
                        if start != 0 {
                            previous_open_start = start;
                        }
                        (*pnote).start = previous_open_start;
                        if end != 0 {
                            previous_open_end = end;
                        }
                        (*pnote).end = previous_open_end;
                    } else {
                        if start != 0 {
                            previous_func_start = start;
                        }
                        (*pnote).start = previous_func_start;
                        if end != 0 {
                            previous_func_end = end;
                        }
                        (*pnote).end = previous_func_end;
                    }
                    if *((*pnote).note.namedata)
                        .offset(
                            ((*pnote).note.namesz)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                        ) as libc::c_int != 0 as libc::c_int
                    {
                        err = dcgettext(
                            0 as *const libc::c_char,
                            b"corrupt GNU build attribute note: name not NUL terminated\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        );
                        current_block = 4462513327734392438;
                        break;
                    } else {
                        pnote = pnote.offset(1);
                        pnote;
                    }
                }
            }
            match current_block {
                4462513327734392438 => {}
                _ => {
                    pnotes_end = pnote;
                    if remain != 0 as libc::c_int as libc::c_ulong {
                        err = dcgettext(
                            0 as *const libc::c_char,
                            b"corrupt GNU build attribute notes: excess data at end\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        );
                    } else {
                        if version_1_seen == 0 as libc::c_int as libc::c_uint
                            && version_2_seen == 0 as libc::c_int as libc::c_uint
                            && version_3_seen == 0 as libc::c_int as libc::c_uint
                        {
                            non_fatal(
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"%s[%s]: Warning: version note missing - assuming version 3\0"
                                        as *const u8 as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                                bfd_get_filename(abfd),
                                bfd_section_name(sec),
                            );
                            version_3_seen = 2 as libc::c_int as libc::c_uint;
                        }
                        if version_1_seen > 0 as libc::c_int as libc::c_uint
                            && version_2_seen > 0 as libc::c_int as libc::c_uint
                            || version_1_seen > 0 as libc::c_int as libc::c_uint
                                && version_3_seen > 0 as libc::c_int as libc::c_uint
                            || version_2_seen > 0 as libc::c_int as libc::c_uint
                                && version_3_seen > 0 as libc::c_int as libc::c_uint
                        {
                            err = dcgettext(
                                0 as *const libc::c_char,
                                b"bad GNU build attribute notes: multiple different versions\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            );
                        } else if !(version_3_seen == 0 as libc::c_int as libc::c_uint) {
                            qsort(
                                pnotes as *mut libc::c_void,
                                pnotes_end.offset_from(pnotes) as libc::c_long as size_t,
                                ::core::mem::size_of::<objcopy_internal_note>()
                                    as libc::c_ulong,
                                Some(
                                    compare_gnu_build_notes
                                        as unsafe extern "C" fn(
                                            *const libc::c_void,
                                            *const libc::c_void,
                                        ) -> libc::c_int,
                                ),
                            );
                            pnote = pnotes;
                            while pnote < pnotes_end {
                                if !is_deleted_note(pnote) {
                                    if (*pnote).start == (*pnote).end {
                                        (*pnote).note.type_0 = 0 as libc::c_int as libc::c_ulong;
                                    } else {
                                        let mut iter: libc::c_int = 0;
                                        let mut back: *mut objcopy_internal_note = 0
                                            as *mut objcopy_internal_note;
                                        iter = 0 as libc::c_int;
                                        back = pnote.offset(-(1 as libc::c_int as isize));
                                        while back >= pnotes {
                                            if !is_deleted_note(back) {
                                                if (*back).note.namesz != (*pnote).note.namesz
                                                    || memcmp(
                                                        (*back).note.namedata as *const libc::c_void,
                                                        (*pnote).note.namedata as *const libc::c_void,
                                                        (*pnote).note.namesz,
                                                    ) != 0 as libc::c_int
                                                {
                                                    break;
                                                }
                                                if (*back).start == (*pnote).start
                                                    && (*back).end == (*pnote).end
                                                {
                                                    (*pnote).note.type_0 = 0 as libc::c_int as libc::c_ulong;
                                                    break;
                                                } else if contained_by(pnote, back) {
                                                    (*pnote).note.type_0 = 0 as libc::c_int as libc::c_ulong;
                                                    break;
                                                } else if overlaps_or_adjoins(back, pnote) as libc::c_int
                                                    != 0
                                                    && is_func_note(back) as libc::c_int
                                                        == is_func_note(pnote) as libc::c_int
                                                {
                                                    (*back)
                                                        .end = if (*back).end > (*pnote).end {
                                                        (*back).end
                                                    } else {
                                                        (*pnote).end
                                                    };
                                                    (*back)
                                                        .start = if (*back).start < (*pnote).start {
                                                        (*back).start
                                                    } else {
                                                        (*pnote).start
                                                    };
                                                    (*pnote).note.type_0 = 0 as libc::c_int as libc::c_ulong;
                                                    break;
                                                } else {
                                                    let fresh20 = iter;
                                                    iter = iter + 1;
                                                    if fresh20 > 16 as libc::c_int {
                                                        break;
                                                    }
                                                }
                                            }
                                            back = back.offset(-1);
                                            back;
                                        }
                                    }
                                }
                                pnote = pnote.offset(1);
                                pnote;
                            }
                            qsort(
                                pnotes as *mut libc::c_void,
                                pnotes_end.offset_from(pnotes) as libc::c_long as size_t,
                                ::core::mem::size_of::<objcopy_internal_note>()
                                    as libc::c_ulong,
                                Some(
                                    sort_gnu_build_notes
                                        as unsafe extern "C" fn(
                                            *const libc::c_void,
                                            *const libc::c_void,
                                        ) -> libc::c_int,
                                ),
                            );
                            new_contents = 0 as *mut bfd_byte;
                            old = 0 as *mut bfd_byte;
                            new = 0 as *mut bfd_byte;
                            new_size = 0;
                            prev_start = 0 as libc::c_int as bfd_vma;
                            prev_end = 0 as libc::c_int as bfd_vma;
                            new_contents = xmalloc(
                                size.wrapping_mul(2 as libc::c_int as libc::c_ulong),
                            ) as *mut bfd_byte;
                            new = new_contents;
                            pnote = pnotes;
                            old = contents;
                            while pnote < pnotes_end {
                                let mut note_size: bfd_size_type = (12 as libc::c_int
                                    as libc::c_ulong)
                                    .wrapping_add((*pnote).padded_namesz)
                                    .wrapping_add((*pnote).note.descsz);
                                if !is_deleted_note(pnote) {
                                    if (*pnote).start == prev_start && (*pnote).end == prev_end
                                    {
                                        (Some(
                                            ((*(*abfd).xvec).bfd_putx32)
                                                .expect("non-null function pointer"),
                                        ))
                                            .expect(
                                                "non-null function pointer",
                                            )((*pnote).note.namesz, new as *mut libc::c_void);
                                        (Some(
                                            ((*(*abfd).xvec).bfd_putx32)
                                                .expect("non-null function pointer"),
                                        ))
                                            .expect(
                                                "non-null function pointer",
                                            )(
                                            0 as libc::c_int as bfd_vma,
                                            new.offset(4 as libc::c_int as isize) as *mut libc::c_void,
                                        );
                                        (Some(
                                            ((*(*abfd).xvec).bfd_putx32)
                                                .expect("non-null function pointer"),
                                        ))
                                            .expect(
                                                "non-null function pointer",
                                            )(
                                            (*pnote).note.type_0,
                                            new.offset(8 as libc::c_int as isize) as *mut libc::c_void,
                                        );
                                        new = new.offset(12 as libc::c_int as isize);
                                        memcpy(
                                            new as *mut libc::c_void,
                                            (*pnote).note.namedata as *const libc::c_void,
                                            (*pnote).note.namesz,
                                        );
                                        if (*pnote).note.namesz < (*pnote).padded_namesz {
                                            memset(
                                                new.offset((*pnote).note.namesz as isize)
                                                    as *mut libc::c_void,
                                                0 as libc::c_int,
                                                ((*pnote).padded_namesz).wrapping_sub((*pnote).note.namesz),
                                            );
                                        }
                                        new = new.offset((*pnote).padded_namesz as isize);
                                    } else {
                                        (Some(
                                            ((*(*abfd).xvec).bfd_putx32)
                                                .expect("non-null function pointer"),
                                        ))
                                            .expect(
                                                "non-null function pointer",
                                            )((*pnote).note.namesz, new as *mut libc::c_void);
                                        (Some(
                                            ((*(*abfd).xvec).bfd_putx32)
                                                .expect("non-null function pointer"),
                                        ))
                                            .expect(
                                                "non-null function pointer",
                                            )(
                                            (if is_64bit(abfd) as libc::c_int != 0 {
                                                16 as libc::c_int
                                            } else {
                                                8 as libc::c_int
                                            }) as bfd_vma,
                                            new.offset(4 as libc::c_int as isize) as *mut libc::c_void,
                                        );
                                        (Some(
                                            ((*(*abfd).xvec).bfd_putx32)
                                                .expect("non-null function pointer"),
                                        ))
                                            .expect(
                                                "non-null function pointer",
                                            )(
                                            (*pnote).note.type_0,
                                            new.offset(8 as libc::c_int as isize) as *mut libc::c_void,
                                        );
                                        new = new.offset(12 as libc::c_int as isize);
                                        memcpy(
                                            new as *mut libc::c_void,
                                            (*pnote).note.namedata as *const libc::c_void,
                                            (*pnote).note.namesz,
                                        );
                                        if (*pnote).note.namesz < (*pnote).padded_namesz {
                                            memset(
                                                new.offset((*pnote).note.namesz as isize)
                                                    as *mut libc::c_void,
                                                0 as libc::c_int,
                                                ((*pnote).padded_namesz).wrapping_sub((*pnote).note.namesz),
                                            );
                                        }
                                        new = new.offset((*pnote).padded_namesz as isize);
                                        if is_64bit(abfd) {
                                            (Some(
                                                ((*(*abfd).xvec).bfd_putx64)
                                                    .expect("non-null function pointer"),
                                            ))
                                                .expect(
                                                    "non-null function pointer",
                                                )((*pnote).start, new as *mut libc::c_void);
                                            (Some(
                                                ((*(*abfd).xvec).bfd_putx64)
                                                    .expect("non-null function pointer"),
                                            ))
                                                .expect(
                                                    "non-null function pointer",
                                                )(
                                                (*pnote).end,
                                                new.offset(8 as libc::c_int as isize) as *mut libc::c_void,
                                            );
                                            new = new.offset(16 as libc::c_int as isize);
                                        } else {
                                            (Some(
                                                ((*(*abfd).xvec).bfd_putx32)
                                                    .expect("non-null function pointer"),
                                            ))
                                                .expect(
                                                    "non-null function pointer",
                                                )((*pnote).start, new as *mut libc::c_void);
                                            (Some(
                                                ((*(*abfd).xvec).bfd_putx32)
                                                    .expect("non-null function pointer"),
                                            ))
                                                .expect(
                                                    "non-null function pointer",
                                                )(
                                                (*pnote).end,
                                                new.offset(4 as libc::c_int as isize) as *mut libc::c_void,
                                            );
                                            new = new.offset(8 as libc::c_int as isize);
                                        }
                                        prev_start = (*pnote).start;
                                        prev_end = (*pnote).end;
                                    }
                                }
                                old = old.offset(note_size as isize);
                                pnote = pnote.offset(1);
                                pnote;
                            }
                            new_size = new.offset_from(new_contents) as libc::c_long
                                as bfd_size_type;
                            if new_size < size {
                                memcpy(
                                    contents as *mut libc::c_void,
                                    new_contents as *const libc::c_void,
                                    new_size,
                                );
                                size = new_size;
                            }
                            free(new_contents as *mut libc::c_void);
                        }
                    }
                }
            }
        }
        _ => {}
    }
    if !err.is_null() {
        bfd_set_error(bfd_error_bad_value);
        bfd_nonfatal_message(0 as *const libc::c_char, abfd, sec, err);
        status = 1 as libc::c_int;
    }
    free(pnotes as *mut libc::c_void);
    return size;
}
unsafe extern "C" fn check_new_section_flags(
    mut flags: flagword,
    mut abfd: *mut bfd,
    mut secname: *const libc::c_char,
) -> flagword {
    if flags & 0x8000000 as libc::c_int as libc::c_uint != 0
        && bfd_get_flavour(abfd) as libc::c_uint
            != bfd_target_coff_flavour as libc::c_int as libc::c_uint
    {
        non_fatal(
            dcgettext(
                0 as *const libc::c_char,
                b"%s[%s]: Note - dropping 'share' flag as output format is not COFF\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            bfd_get_filename(abfd),
            secname,
        );
        flags &= !(0x8000000 as libc::c_int) as libc::c_uint;
    }
    return flags;
}
unsafe extern "C" fn copy_object(
    mut ibfd: *mut bfd,
    mut obfd: *mut bfd,
    mut input_arch: *const bfd_arch_info_type,
) -> bool {
    let mut start: bfd_vma = 0;
    let mut symcount: libc::c_long = 0;
    let mut osections: *mut *mut asection = 0 as *mut *mut asection;
    let mut osec: *mut asection = 0 as *mut asection;
    let mut gnu_debuglink_section: *mut asection = 0 as *mut asection;
    let mut gaps: *mut bfd_size_type = 0 as *mut bfd_size_type;
    let mut max_gap: bfd_size_type = 0 as libc::c_int as bfd_size_type;
    let mut symsize: libc::c_long = 0;
    let mut dhandle: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut iarch: bfd_architecture = bfd_arch_unknown;
    let mut imach: libc::c_uint = 0;
    let mut num_sec: libc::c_uint = 0;
    let mut i: libc::c_uint = 0;
    if (*(*ibfd).xvec).byteorder as libc::c_uint
        != (*(*obfd).xvec).byteorder as libc::c_uint
        && (*(*ibfd).xvec).byteorder as libc::c_uint
            != BFD_ENDIAN_UNKNOWN as libc::c_int as libc::c_uint
        && (*(*obfd).xvec).byteorder as libc::c_uint
            != BFD_ENDIAN_UNKNOWN as libc::c_int as libc::c_uint
    {
        non_fatal(
            dcgettext(
                0 as *const libc::c_char,
                b"unable to change endianness of '%s'\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            bfd_get_archive_filename(ibfd),
        );
        return 0 as libc::c_int != 0;
    }
    if (*ibfd).read_only() != 0 {
        non_fatal(
            dcgettext(
                0 as *const libc::c_char,
                b"unable to modify '%s' due to errors\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            bfd_get_archive_filename(ibfd),
        );
        return 0 as libc::c_int != 0;
    }
    if !bfd_set_format(obfd, bfd_get_format(ibfd)) {
        bfd_nonfatal_message(
            0 as *const libc::c_char,
            obfd,
            0 as *const asection,
            0 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    if ((*ibfd).sections).is_null() {
        non_fatal(
            dcgettext(
                0 as *const libc::c_char,
                b"error: the input file '%s' has no sections\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            bfd_get_archive_filename(ibfd),
        );
        return 0 as libc::c_int != 0;
    }
    if bfd_get_flavour(ibfd) as libc::c_uint
        != bfd_target_elf_flavour as libc::c_int as libc::c_uint
    {
        if do_debug_sections as libc::c_uint & compress as libc::c_int as libc::c_uint
            != 0 as libc::c_int as libc::c_uint
            && do_debug_sections as libc::c_uint
                != compress as libc::c_int as libc::c_uint
        {
            non_fatal(
                dcgettext(
                    0 as *const libc::c_char,
                    b"--compress-debug-sections=[zlib|zlib-gnu|zlib-gabi] is unsupported on `%s'\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                bfd_get_archive_filename(ibfd),
            );
            return 0 as libc::c_int != 0;
        }
        if do_elf_stt_common as u64 != 0 {
            non_fatal(
                dcgettext(
                    0 as *const libc::c_char,
                    b"--elf-stt-common=[yes|no] is unsupported on `%s'\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                bfd_get_archive_filename(ibfd),
            );
            return 0 as libc::c_int != 0;
        }
    }
    if verbose {
        printf(
            dcgettext(
                0 as *const libc::c_char,
                b"copy from `%s' [%s] to `%s' [%s]\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            bfd_get_archive_filename(ibfd),
            bfd_get_target(ibfd),
            bfd_get_filename(obfd),
            bfd_get_target(obfd),
        );
    }
    if extract_symbol {
        start = 0 as libc::c_int as bfd_vma;
    } else {
        if set_start_set {
            start = set_start;
        } else {
            start = bfd_get_start_address(ibfd);
        }
        start = (start as libc::c_ulong).wrapping_add(change_start) as bfd_vma
            as bfd_vma;
    }
    if bfd_get_format(obfd) as libc::c_uint != bfd_core as libc::c_int as libc::c_uint {
        let mut flags: flagword = 0;
        flags = bfd_get_file_flags(ibfd);
        flags |= bfd_flags_to_set;
        flags &= !bfd_flags_to_clear;
        flags &= bfd_applicable_file_flags(obfd);
        if strip_symbols as libc::c_uint == STRIP_ALL as libc::c_int as libc::c_uint {
            flags &= !(0x1 as libc::c_int) as libc::c_uint;
        }
        if !bfd_set_start_address(obfd, start) || !bfd_set_file_flags(obfd, flags) {
            bfd_nonfatal_message(
                0 as *const libc::c_char,
                ibfd,
                0 as *const asection,
                0 as *const libc::c_char,
            );
            return 0 as libc::c_int != 0;
        }
    }
    iarch = bfd_get_arch(ibfd);
    imach = bfd_get_mach(ibfd) as libc::c_uint;
    if !input_arch.is_null() {
        if iarch as libc::c_uint == bfd_arch_unknown as libc::c_int as libc::c_uint {
            iarch = (*input_arch).arch;
            imach = (*input_arch).mach as libc::c_uint;
        } else {
            non_fatal(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Input file `%s' ignores binary architecture parameter.\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                bfd_get_archive_filename(ibfd),
            );
        }
    }
    if iarch as libc::c_uint == bfd_arch_unknown as libc::c_int as libc::c_uint
        && bfd_get_flavour(ibfd) as libc::c_uint
            != bfd_target_elf_flavour as libc::c_int as libc::c_uint
        && bfd_get_flavour(obfd) as libc::c_uint
            == bfd_target_elf_flavour as libc::c_int as libc::c_uint
    {
        let mut bed: *const elf_backend_data = (*(*obfd).xvec).backend_data
            as *const elf_backend_data;
        iarch = (*bed).arch;
        imach = 0 as libc::c_int as libc::c_uint;
    }
    if !(Some(((*(*obfd).xvec)._bfd_set_arch_mach).expect("non-null function pointer")))
        .expect("non-null function pointer")(obfd, iarch, imach as libc::c_ulong)
        && ((*ibfd).target_defaulted() as libc::c_int != 0
            || bfd_get_arch(ibfd) as libc::c_uint != bfd_get_arch(obfd) as libc::c_uint)
    {
        if bfd_get_arch(ibfd) as libc::c_uint
            == bfd_arch_unknown as libc::c_int as libc::c_uint
        {
            non_fatal(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Unable to recognise the format of the input file `%s'\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                bfd_get_archive_filename(ibfd),
            );
        } else {
            non_fatal(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Output file cannot represent architecture `%s'\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                bfd_printable_arch_mach(bfd_get_arch(ibfd), bfd_get_mach(ibfd)),
            );
        }
        return 0 as libc::c_int != 0;
    }
    if !bfd_set_format(obfd, bfd_get_format(ibfd)) {
        bfd_nonfatal_message(
            0 as *const libc::c_char,
            ibfd,
            0 as *const asection,
            0 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    if bfd_get_flavour(obfd) as libc::c_uint
        == bfd_target_coff_flavour as libc::c_int as libc::c_uint
        && startswith(
            (*(*obfd).xvec).name,
            b"pei-\0" as *const u8 as *const libc::c_char,
        ) as libc::c_int != 0
    {
        let mut pe: *mut pe_data_type = (*obfd).tdata.pe_obj_data;
        if bfd_get_flavour(ibfd) as libc::c_uint
            == bfd_target_coff_flavour as libc::c_int as libc::c_uint
            && startswith(
                (*(*ibfd).xvec).name,
                b"pei-\0" as *const u8 as *const libc::c_char,
            ) as libc::c_int != 0
        {
            (*pe).pe_opthdr = (*(*ibfd).tdata.pe_obj_data).pe_opthdr;
        }
        if pe_file_alignment != -(1 as libc::c_int) as bfd_vma {
            (*pe).pe_opthdr.FileAlignment = pe_file_alignment as uint32_t;
        } else {
            pe_file_alignment = 0x200 as libc::c_int as bfd_vma;
        }
        if pe_heap_commit != -(1 as libc::c_int) as bfd_vma {
            (*pe).pe_opthdr.SizeOfHeapCommit = pe_heap_commit;
        }
        if pe_heap_reserve != -(1 as libc::c_int) as bfd_vma {
            (*pe).pe_opthdr.SizeOfHeapCommit = pe_heap_reserve;
        }
        if pe_image_base != -(1 as libc::c_int) as bfd_vma {
            (*pe).pe_opthdr.ImageBase = pe_image_base;
        }
        if pe_section_alignment != -(1 as libc::c_int) as bfd_vma {
            (*pe).pe_opthdr.SectionAlignment = pe_section_alignment as uint32_t;
        } else {
            pe_section_alignment = 0x1000 as libc::c_int as bfd_vma;
        }
        if pe_stack_commit != -(1 as libc::c_int) as bfd_vma {
            (*pe).pe_opthdr.SizeOfStackCommit = pe_stack_commit;
        }
        if pe_stack_reserve != -(1 as libc::c_int) as bfd_vma {
            (*pe).pe_opthdr.SizeOfStackCommit = pe_stack_reserve;
        }
        if pe_subsystem as libc::c_int != -(1 as libc::c_int) {
            (*pe).pe_opthdr.Subsystem = pe_subsystem;
        }
        if pe_major_subsystem_version as libc::c_int != -(1 as libc::c_int) {
            (*pe).pe_opthdr.MajorSubsystemVersion = pe_major_subsystem_version;
        }
        if pe_minor_subsystem_version as libc::c_int != -(1 as libc::c_int) {
            (*pe).pe_opthdr.MinorSubsystemVersion = pe_minor_subsystem_version;
        }
        if pe_file_alignment > pe_section_alignment {
            let mut file_alignment: [libc::c_char; 20] = [0; 20];
            let mut section_alignment: [libc::c_char; 20] = [0; 20];
            sprintf(
                file_alignment.as_mut_ptr(),
                b"%016lx\0" as *const u8 as *const libc::c_char,
                pe_file_alignment,
            );
            sprintf(
                section_alignment.as_mut_ptr(),
                b"%016lx\0" as *const u8 as *const libc::c_char,
                pe_section_alignment,
            );
            non_fatal(
                dcgettext(
                    0 as *const libc::c_char,
                    b"warning: file alignment (0x%s) > section alignment (0x%s)\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                file_alignment.as_mut_ptr(),
                section_alignment.as_mut_ptr(),
            );
        }
        if preserve_dates as libc::c_int != 0
            && bfd_get_flavour(ibfd) as libc::c_uint
                == bfd_target_coff_flavour as libc::c_int as libc::c_uint
            && startswith(
                (*(*ibfd).xvec).name,
                b"pei-\0" as *const u8 as *const libc::c_char,
            ) as libc::c_int != 0
        {
            (*pe).timestamp = (*(*ibfd).tdata.pe_obj_data).coff.timestamp as libc::c_int;
        }
    }
    free(isympp as *mut libc::c_void);
    if osympp != isympp {
        free(osympp as *mut libc::c_void);
    }
    isympp = 0 as *mut *mut asymbol;
    osympp = 0 as *mut *mut asymbol;
    symsize = (Some(
        ((*(*ibfd).xvec)._bfd_get_symtab_upper_bound).expect("non-null function pointer"),
    ))
        .expect("non-null function pointer")(ibfd);
    if symsize < 0 as libc::c_int as libc::c_long {
        bfd_nonfatal_message(
            0 as *const libc::c_char,
            ibfd,
            0 as *const asection,
            0 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    isympp = xmalloc(symsize as size_t) as *mut *mut asymbol;
    osympp = isympp;
    symcount = (Some(
        ((*(*ibfd).xvec)._bfd_canonicalize_symtab).expect("non-null function pointer"),
    ))
        .expect("non-null function pointer")(ibfd, isympp);
    if symcount < 0 as libc::c_int as libc::c_long {
        bfd_nonfatal_message(
            0 as *const libc::c_char,
            ibfd,
            0 as *const asection,
            0 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    if symcount == 0 as libc::c_int as libc::c_long {
        free(isympp as *mut libc::c_void);
        isympp = 0 as *mut *mut asymbol;
        osympp = isympp;
    }
    bfd_map_over_sections(
        ibfd,
        Some(
            setup_section
                as unsafe extern "C" fn(*mut bfd, *mut asection, *mut libc::c_void) -> (),
        ),
        obfd as *mut libc::c_void,
    );
    if !extract_symbol {
        setup_bfd_headers(ibfd, obfd);
    }
    if !add_sections.is_null() {
        let mut padd: *mut section_add = 0 as *mut section_add;
        let mut pset: *mut section_list = 0 as *mut section_list;
        padd = add_sections;
        while !padd.is_null() {
            let mut flags_0: flagword = 0;
            pset = find_section_list(
                (*padd).name,
                0 as libc::c_int != 0,
                ((1 as libc::c_int) << 7 as libc::c_int) as libc::c_uint,
            );
            if !pset.is_null() {
                flags_0 = (*pset).flags | 0x100 as libc::c_int as libc::c_uint;
                flags_0 = check_new_section_flags(flags_0, obfd, (*padd).name);
            } else {
                flags_0 = (0x100 as libc::c_int | 0x8 as libc::c_int
                    | 0x20 as libc::c_int) as flagword;
            }
            if !(bfd_get_section_by_name(obfd, (*padd).name)).is_null() {
                bfd_nonfatal_message(
                    0 as *const libc::c_char,
                    obfd,
                    0 as *const asection,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"can't add section '%s'\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    (*padd).name,
                );
                return 0 as libc::c_int != 0;
            } else {
                (*padd)
                    .section = bfd_make_section_with_flags(
                    obfd,
                    (*padd).name,
                    flags_0 | 0x100000 as libc::c_int as libc::c_uint,
                );
                if ((*padd).section).is_null() {
                    bfd_nonfatal_message(
                        0 as *const libc::c_char,
                        obfd,
                        0 as *const asection,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"can't create section `%s'\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        (*padd).name,
                    );
                    return 0 as libc::c_int != 0;
                }
            }
            if !bfd_set_section_size((*padd).section, (*padd).size) {
                bfd_nonfatal_message(
                    0 as *const libc::c_char,
                    obfd,
                    (*padd).section,
                    0 as *const libc::c_char,
                );
                return 0 as libc::c_int != 0;
            }
            pset = find_section_list(
                (*padd).name,
                0 as libc::c_int != 0,
                ((1 as libc::c_int) << 3 as libc::c_int
                    | (1 as libc::c_int) << 4 as libc::c_int) as libc::c_uint,
            );
            if !pset.is_null() && !bfd_set_section_vma((*padd).section, (*pset).vma_val)
            {
                bfd_nonfatal_message(
                    0 as *const libc::c_char,
                    obfd,
                    (*padd).section,
                    0 as *const libc::c_char,
                );
                return 0 as libc::c_int != 0;
            }
            pset = find_section_list(
                (*padd).name,
                0 as libc::c_int != 0,
                ((1 as libc::c_int) << 5 as libc::c_int
                    | (1 as libc::c_int) << 6 as libc::c_int) as libc::c_uint,
            );
            if !pset.is_null() {
                (*(*padd).section).lma = (*pset).lma_val;
                if !bfd_set_section_alignment(
                    (*padd).section,
                    bfd_section_alignment((*padd).section),
                ) {
                    bfd_nonfatal_message(
                        0 as *const libc::c_char,
                        obfd,
                        (*padd).section,
                        0 as *const libc::c_char,
                    );
                    return 0 as libc::c_int != 0;
                }
            }
            padd = (*padd).next;
        }
    }
    if !update_sections.is_null() {
        let mut pupdate: *mut section_add = 0 as *mut section_add;
        pupdate = update_sections;
        while !pupdate.is_null() {
            (*pupdate).section = bfd_get_section_by_name(ibfd, (*pupdate).name);
            if ((*pupdate).section).is_null() {
                non_fatal(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"error: %s not found, can't be updated\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    (*pupdate).name,
                );
                return 0 as libc::c_int != 0;
            }
            osec = (*(*pupdate).section).output_section;
            if !bfd_set_section_size(osec, (*pupdate).size) {
                bfd_nonfatal_message(
                    0 as *const libc::c_char,
                    obfd,
                    osec,
                    0 as *const libc::c_char,
                );
                return 0 as libc::c_int != 0;
            }
            pupdate = (*pupdate).next;
        }
    }
    let mut merged_note_sections: *mut merged_note_section = 0
        as *mut merged_note_section;
    if merge_notes {
        osec = (*ibfd).sections;
        while !osec.is_null() {
            if is_mergeable_note_section(ibfd, osec) {
                if !((*osec).output_section).is_null() {
                    let mut size: bfd_size_type = bfd_section_size(osec);
                    if size == 0 as libc::c_int as libc::c_ulong {
                        bfd_nonfatal_message(
                            0 as *const libc::c_char,
                            ibfd,
                            osec,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"warning: note section is empty\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                    } else {
                        let mut merged: *mut merged_note_section = xmalloc(
                            ::core::mem::size_of::<merged_note_section>()
                                as libc::c_ulong,
                        ) as *mut merged_note_section;
                        (*merged).contents = 0 as *mut bfd_byte;
                        if !bfd_get_full_section_contents(
                            ibfd,
                            osec,
                            &mut (*merged).contents,
                        ) {
                            bfd_nonfatal_message(
                                0 as *const libc::c_char,
                                ibfd,
                                osec,
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"warning: could not load note section\0" as *const u8
                                        as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                            );
                            free(merged as *mut libc::c_void);
                        } else {
                            (*merged)
                                .size = merge_gnu_build_notes(
                                ibfd,
                                osec,
                                size,
                                (*merged).contents,
                            );
                            if size != (*merged).size
                                && !bfd_set_section_size(
                                    (*osec).output_section,
                                    (*merged).size,
                                )
                            {
                                bfd_nonfatal_message(
                                    0 as *const libc::c_char,
                                    obfd,
                                    osec,
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"warning: failed to set merged notes size\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                                free((*merged).contents as *mut libc::c_void);
                                free(merged as *mut libc::c_void);
                            } else {
                                (*merged).sec = osec;
                                (*merged).next = merged_note_sections;
                                merged_note_sections = merged;
                            }
                        }
                    }
                }
            }
            osec = (*osec).next;
        }
    }
    if !dump_sections.is_null() {
        let mut pdump: *mut section_add = 0 as *mut section_add;
        pdump = dump_sections;
        while !pdump.is_null() {
            let mut f: *mut FILE = 0 as *mut FILE;
            let mut contents: *mut bfd_byte = 0 as *mut bfd_byte;
            osec = bfd_get_section_by_name(ibfd, (*pdump).name);
            if osec.is_null() {
                bfd_nonfatal_message(
                    0 as *const libc::c_char,
                    ibfd,
                    0 as *const asection,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"can't dump section '%s' - it does not exist\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    (*pdump).name,
                );
            } else if bfd_section_flags(osec) & 0x100 as libc::c_int as libc::c_uint
                == 0 as libc::c_int as libc::c_uint
            {
                bfd_nonfatal_message(
                    0 as *const libc::c_char,
                    ibfd,
                    osec,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"can't dump section - it has no contents\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
            } else {
                let mut size_0: bfd_size_type = bfd_section_size(osec);
                f = fopen((*pdump).filename, b"w\0" as *const u8 as *const libc::c_char);
                if f.is_null() {
                    bfd_nonfatal_message(
                        (*pdump).filename,
                        0 as *const bfd,
                        0 as *const asection,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"could not open section dump file\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                } else {
                    if bfd_malloc_and_get_section(ibfd, osec, &mut contents) {
                        if size_0 != 0 as libc::c_int as libc::c_ulong
                            && fwrite(
                                contents as *const libc::c_void,
                                1 as libc::c_int as libc::c_ulong,
                                size_0,
                                f,
                            ) != size_0
                        {
                            non_fatal(
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"error writing section contents to %s (error: %s)\0"
                                        as *const u8 as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                                (*pdump).filename,
                                strerror(*__errno_location()),
                            );
                            free(contents as *mut libc::c_void);
                            fclose(f);
                            return 0 as libc::c_int != 0;
                        }
                    } else {
                        bfd_nonfatal_message(
                            0 as *const libc::c_char,
                            ibfd,
                            osec,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"could not retrieve section contents\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                    }
                    fclose(f);
                    free(contents as *mut libc::c_void);
                }
            }
            pdump = (*pdump).next;
        }
    }
    if !gnu_debuglink_filename.is_null() {
        if !(bfd_get_section_by_name(
            obfd,
            b".gnu_debuglink\0" as *const u8 as *const libc::c_char,
        ))
            .is_null()
        {
            non_fatal(
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: debuglink section already exists\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                bfd_get_filename(obfd),
            );
            gnu_debuglink_filename = 0 as *const libc::c_char;
        } else {
            gnu_debuglink_section = bfd_create_gnu_debuglink_section(
                obfd,
                gnu_debuglink_filename,
            );
            if gnu_debuglink_section.is_null() {
                bfd_nonfatal_message(
                    0 as *const libc::c_char,
                    obfd,
                    0 as *const asection,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"cannot create debug link section `%s'\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    gnu_debuglink_filename,
                );
                return 0 as libc::c_int != 0;
            }
            if bfd_get_flavour(obfd) as libc::c_uint
                == bfd_target_coff_flavour as libc::c_int as libc::c_uint
            {
                let mut debuglink_vma: bfd_vma = 0;
                let mut highest_section: *mut asection = 0 as *mut asection;
                osec = (*obfd).sections;
                highest_section = 0 as *mut asection;
                while !osec.is_null() {
                    if (*osec).vma > 0 as libc::c_int as libc::c_ulong
                        && (highest_section.is_null()
                            || (*osec).vma > (*highest_section).vma)
                    {
                        highest_section = osec;
                    }
                    osec = (*osec).next;
                }
                if !highest_section.is_null() {
                    debuglink_vma = if ((*highest_section).vma)
                        .wrapping_add((*highest_section).size)
                        .wrapping_add(0x1000 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        >= ((*highest_section).vma).wrapping_add((*highest_section).size)
                    {
                        ((*highest_section).vma)
                            .wrapping_add((*highest_section).size)
                            .wrapping_add(
                                (0x1000 as libc::c_int - 1 as libc::c_int) as libc::c_ulong,
                            ) & !((0x1000 as libc::c_int - 1 as libc::c_int) as bfd_vma)
                    } else {
                        !(0 as libc::c_int as bfd_vma)
                    };
                } else {
                    debuglink_vma = 0x1000 as libc::c_int as bfd_vma;
                }
                bfd_set_section_vma(gnu_debuglink_section, debuglink_vma);
            }
        }
    }
    num_sec = bfd_count_sections(obfd);
    if num_sec != 0 as libc::c_int as libc::c_uint
        && (gap_fill_set as libc::c_int != 0 || pad_to_set as libc::c_int != 0)
    {
        let mut set: *mut *mut asection = 0 as *mut *mut asection;
        osections = xmalloc(
            (num_sec as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<*mut asection>() as libc::c_ulong),
        ) as *mut *mut asection;
        set = osections;
        bfd_map_over_sections(
            obfd,
            Some(
                get_sections
                    as unsafe extern "C" fn(
                        *mut bfd,
                        *mut asection,
                        *mut libc::c_void,
                    ) -> (),
            ),
            &mut set as *mut *mut *mut asection as *mut libc::c_void,
        );
        qsort(
            osections as *mut libc::c_void,
            num_sec as size_t,
            ::core::mem::size_of::<*mut asection>() as libc::c_ulong,
            Some(
                compare_section_lma
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
        );
        gaps = xmalloc(
            (num_sec as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<bfd_size_type>() as libc::c_ulong),
        ) as *mut bfd_size_type;
        memset(
            gaps as *mut libc::c_void,
            0 as libc::c_int,
            (num_sec as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<bfd_size_type>() as libc::c_ulong),
        );
        if gap_fill_set {
            i = 0 as libc::c_int as libc::c_uint;
            while i < num_sec.wrapping_sub(1 as libc::c_int as libc::c_uint) {
                let mut flags_1: flagword = 0;
                let mut size_1: bfd_size_type = 0;
                let mut gap_start: bfd_vma = 0;
                let mut gap_stop: bfd_vma = 0;
                let mut opb1: libc::c_uint = bfd_octets_per_byte(
                    obfd,
                    *osections.offset(i as isize),
                );
                let mut opb2: libc::c_uint = bfd_octets_per_byte(
                    obfd,
                    *osections
                        .offset(
                            i.wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                        ),
                );
                flags_1 = bfd_section_flags(*osections.offset(i as isize));
                if !(flags_1 & 0x100 as libc::c_int as libc::c_uint
                    == 0 as libc::c_int as libc::c_uint
                    || flags_1 & 0x2 as libc::c_int as libc::c_uint
                        == 0 as libc::c_int as libc::c_uint)
                {
                    size_1 = bfd_section_size(*osections.offset(i as isize));
                    gap_start = (bfd_section_lma(*osections.offset(i as isize)))
                        .wrapping_mul(opb1 as libc::c_ulong)
                        .wrapping_add(size_1);
                    gap_stop = (bfd_section_lma(
                        *osections
                            .offset(
                                i.wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                            ),
                    ))
                        .wrapping_mul(opb2 as libc::c_ulong);
                    if gap_start < gap_stop {
                        if !bfd_set_section_size(
                            *osections.offset(i as isize),
                            size_1.wrapping_add(gap_stop.wrapping_sub(gap_start)),
                        ) {
                            bfd_nonfatal_message(
                                0 as *const libc::c_char,
                                obfd,
                                *osections.offset(i as isize),
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"Can't fill gap after section\0" as *const u8
                                        as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                            );
                            status = 1 as libc::c_int;
                            break;
                        } else {
                            *gaps.offset(i as isize) = gap_stop.wrapping_sub(gap_start);
                            if max_gap < gap_stop.wrapping_sub(gap_start) {
                                max_gap = gap_stop.wrapping_sub(gap_start);
                            }
                        }
                    }
                }
                i = i.wrapping_add(1);
                i;
            }
        }
        if pad_to_set {
            let mut lma: bfd_vma = 0;
            let mut size_2: bfd_size_type = 0;
            let mut opb: libc::c_uint = bfd_octets_per_byte(
                obfd,
                *osections
                    .offset(
                        num_sec.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
                    ),
            );
            let mut _pad_to: bfd_vma = pad_to.wrapping_mul(opb as libc::c_ulong);
            lma = (bfd_section_lma(
                *osections
                    .offset(
                        num_sec.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
                    ),
            ))
                .wrapping_mul(opb as libc::c_ulong);
            size_2 = bfd_section_size(
                *osections
                    .offset(
                        num_sec.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
                    ),
            );
            if lma.wrapping_add(size_2) < _pad_to {
                if !bfd_set_section_size(
                    *osections
                        .offset(
                            num_sec.wrapping_sub(1 as libc::c_int as libc::c_uint)
                                as isize,
                        ),
                    _pad_to.wrapping_sub(lma),
                ) {
                    bfd_nonfatal_message(
                        0 as *const libc::c_char,
                        obfd,
                        *osections
                            .offset(
                                num_sec.wrapping_sub(1 as libc::c_int as libc::c_uint)
                                    as isize,
                            ),
                        dcgettext(
                            0 as *const libc::c_char,
                            b"can't add padding\0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    status = 1 as libc::c_int;
                } else {
                    *gaps
                        .offset(
                            num_sec.wrapping_sub(1 as libc::c_int as libc::c_uint)
                                as isize,
                        ) = _pad_to.wrapping_sub(lma.wrapping_add(size_2));
                    if max_gap < _pad_to.wrapping_sub(lma.wrapping_add(size_2)) {
                        max_gap = _pad_to.wrapping_sub(lma.wrapping_add(size_2));
                    }
                }
            }
        }
    }
    dhandle = 0 as *mut libc::c_void;
    if convert_debugging {
        dhandle = read_debugging_info(ibfd, isympp, symcount, 0 as libc::c_int != 0);
    }
    if (*obfd).flags & (0x2 as libc::c_int | 0x40 as libc::c_int) as libc::c_uint
        != 0 as libc::c_int as libc::c_uint
        && (*obfd).flags & 0x1 as libc::c_int as libc::c_uint
            == 0 as libc::c_int as libc::c_uint
    {
        if bfd_keep_unused_section_symbols(obfd) as libc::c_int != 0
            || keep_section_symbols as libc::c_int != 0
        {
            let mut asect: *mut asection = 0 as *mut asection;
            asect = (*obfd).sections;
            while !asect.is_null() {
                if !((*asect).symbol).is_null() {
                    (*(*asect).symbol).flags
                        |= ((1 as libc::c_int) << 24 as libc::c_int) as libc::c_uint;
                }
                asect = (*asect).next;
            }
        } else {
            let mut s: libc::c_long = 0;
            s = 0 as libc::c_int as libc::c_long;
            while s < symcount {
                if (**isympp.offset(s as isize)).flags
                    & ((1 as libc::c_int) << 24 as libc::c_int) as libc::c_uint != 0
                {
                    let ref mut fresh21 = (**isympp.offset(s as isize)).flags;
                    *fresh21
                        &= !((1 as libc::c_int) << 24 as libc::c_int) as libc::c_uint;
                }
                s += 1;
                s;
            }
        }
    }
    if strip_symbols as libc::c_uint == STRIP_DEBUG as libc::c_int as libc::c_uint
        || strip_symbols as libc::c_uint == STRIP_ALL as libc::c_int as libc::c_uint
        || strip_symbols as libc::c_uint == STRIP_UNNEEDED as libc::c_int as libc::c_uint
        || strip_symbols as libc::c_uint == STRIP_NONDEBUG as libc::c_int as libc::c_uint
        || strip_symbols as libc::c_uint == STRIP_DWO as libc::c_int as libc::c_uint
        || strip_symbols as libc::c_uint == STRIP_NONDWO as libc::c_int as libc::c_uint
        || discard_locals as libc::c_uint != LOCALS_UNDEF as libc::c_int as libc::c_uint
        || localize_hidden as libc::c_int != 0
        || htab_elements(strip_specific_htab) != 0 as libc::c_int as libc::c_ulong
        || htab_elements(keep_specific_htab) != 0 as libc::c_int as libc::c_ulong
        || htab_elements(localize_specific_htab) != 0 as libc::c_int as libc::c_ulong
        || htab_elements(globalize_specific_htab) != 0 as libc::c_int as libc::c_ulong
        || htab_elements(keepglobal_specific_htab) != 0 as libc::c_int as libc::c_ulong
        || htab_elements(weaken_specific_htab) != 0 as libc::c_int as libc::c_ulong
        || htab_elements(redefine_specific_htab) != 0 as libc::c_int as libc::c_ulong
        || !prefix_symbols_string.is_null() || sections_removed as libc::c_int != 0
        || sections_copied as libc::c_int != 0 || convert_debugging as libc::c_int != 0
        || change_leading_char as libc::c_int != 0
        || remove_leading_char as libc::c_int != 0 || !section_rename_list.is_null()
        || weaken as libc::c_int != 0 || add_symbols != 0
    {
        if strip_symbols as libc::c_uint != STRIP_ALL as libc::c_int as libc::c_uint {
            bfd_set_error(bfd_error_no_error);
            bfd_map_over_sections(
                ibfd,
                Some(
                    mark_symbols_used_in_relocations
                        as unsafe extern "C" fn(
                            *mut bfd,
                            *mut asection,
                            *mut libc::c_void,
                        ) -> (),
                ),
                isympp as *mut libc::c_void,
            );
            if bfd_get_error() as libc::c_uint
                != bfd_error_no_error as libc::c_int as libc::c_uint
            {
                status = 1 as libc::c_int;
                return 0 as libc::c_int != 0;
            }
        }
        osympp = xmalloc(
            ((symcount + add_symbols as libc::c_long + 1 as libc::c_int as libc::c_long)
                as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<*mut asymbol>() as libc::c_ulong),
        ) as *mut *mut asymbol;
        symcount = filter_symbols(ibfd, obfd, osympp, isympp, symcount) as libc::c_long;
    }
    if convert_debugging as libc::c_int != 0 && !dhandle.is_null() {
        let mut res: bool = false;
        res = write_debugging_info(obfd, dhandle, &mut symcount, &mut osympp);
        free(dhandle);
        dhandle = 0 as *mut libc::c_void;
        if !res {
            status = 1 as libc::c_int;
            return 0 as libc::c_int != 0;
        }
    }
    bfd_set_symtab(obfd, osympp, symcount as libc::c_uint);
    bfd_map_over_sections(
        ibfd,
        Some(
            copy_relocations_in_section
                as unsafe extern "C" fn(*mut bfd, *mut asection, *mut libc::c_void) -> (),
        ),
        obfd as *mut libc::c_void,
    );
    bfd_map_over_sections(
        ibfd,
        Some(
            copy_section
                as unsafe extern "C" fn(*mut bfd, *mut asection, *mut libc::c_void) -> (),
        ),
        obfd as *mut libc::c_void,
    );
    if !add_sections.is_null() {
        let mut padd_0: *mut section_add = 0 as *mut section_add;
        padd_0 = add_sections;
        while !padd_0.is_null() {
            if !bfd_set_section_contents(
                obfd,
                (*padd_0).section,
                (*padd_0).contents as *const libc::c_void,
                0 as libc::c_int as file_ptr,
                (*padd_0).size,
            ) {
                bfd_nonfatal_message(
                    0 as *const libc::c_char,
                    obfd,
                    (*padd_0).section,
                    0 as *const libc::c_char,
                );
                return 0 as libc::c_int != 0;
            }
            padd_0 = (*padd_0).next;
        }
    }
    if !update_sections.is_null() {
        let mut pupdate_0: *mut section_add = 0 as *mut section_add;
        pupdate_0 = update_sections;
        while !pupdate_0.is_null() {
            osec = (*(*pupdate_0).section).output_section;
            if !bfd_set_section_contents(
                obfd,
                osec,
                (*pupdate_0).contents as *const libc::c_void,
                0 as libc::c_int as file_ptr,
                (*pupdate_0).size,
            ) {
                bfd_nonfatal_message(
                    0 as *const libc::c_char,
                    obfd,
                    osec,
                    0 as *const libc::c_char,
                );
                return 0 as libc::c_int != 0;
            }
            pupdate_0 = (*pupdate_0).next;
        }
    }
    if !merged_note_sections.is_null() {
        let mut merged_0: *mut merged_note_section = 0 as *mut merged_note_section;
        let mut current_block_342: u64;
        osec = (*obfd).sections;
        while !osec.is_null() {
            if is_mergeable_note_section(obfd, osec) {
                if merged_0.is_null() {
                    merged_0 = merged_note_sections;
                }
                if (*(*merged_0).sec).output_section != osec {
                    merged_0 = merged_note_sections;
                    while !merged_0.is_null() {
                        if (*(*merged_0).sec).output_section == osec {
                            break;
                        }
                        merged_0 = (*merged_0).next;
                    }
                    if merged_0.is_null() {
                        bfd_nonfatal_message(
                            0 as *const libc::c_char,
                            obfd,
                            osec,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"error: failed to locate merged notes\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                        current_block_342 = 11911614146017124710;
                    } else {
                        current_block_342 = 3316925224002568308;
                    }
                } else {
                    current_block_342 = 3316925224002568308;
                }
                match current_block_342 {
                    11911614146017124710 => {}
                    _ => {
                        if ((*merged_0).contents).is_null() {
                            bfd_nonfatal_message(
                                0 as *const libc::c_char,
                                obfd,
                                osec,
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"error: failed to merge notes\0" as *const u8
                                        as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                            );
                        } else {
                            if !bfd_set_section_contents(
                                obfd,
                                osec,
                                (*merged_0).contents as *const libc::c_void,
                                0 as libc::c_int as file_ptr,
                                (*merged_0).size,
                            ) {
                                bfd_nonfatal_message(
                                    0 as *const libc::c_char,
                                    obfd,
                                    osec,
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"error: failed to copy merged notes into output\0"
                                            as *const u8 as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                                return 0 as libc::c_int != 0;
                            }
                            merged_0 = (*merged_0).next;
                        }
                    }
                }
            }
            osec = (*osec).next;
        }
        let mut next: *mut merged_note_section = 0 as *mut merged_note_section;
        merged_0 = merged_note_sections;
        while !merged_0.is_null() {
            next = (*merged_0).next;
            free((*merged_0).contents as *mut libc::c_void);
            free(merged_0 as *mut libc::c_void);
            merged_0 = next;
        }
    } else if merge_notes as libc::c_int != 0 && is_strip == 0 {
        non_fatal(
            dcgettext(
                0 as *const libc::c_char,
                b"%s: Could not find any mergeable note sections\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            bfd_get_filename(ibfd),
        );
    }
    if !gnu_debuglink_filename.is_null() {
        if !bfd_fill_in_gnu_debuglink_section(
            obfd,
            gnu_debuglink_section,
            gnu_debuglink_filename,
        ) {
            bfd_nonfatal_message(
                0 as *const libc::c_char,
                obfd,
                0 as *const asection,
                dcgettext(
                    0 as *const libc::c_char,
                    b"cannot fill debug link section `%s'\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                gnu_debuglink_filename,
            );
            return 0 as libc::c_int != 0;
        }
    }
    if !gaps.is_null() {
        let mut buf: *mut bfd_byte = 0 as *mut bfd_byte;
        if max_gap > 8192 as libc::c_int as libc::c_ulong {
            max_gap = 8192 as libc::c_int as bfd_size_type;
        }
        buf = xmalloc(max_gap) as *mut bfd_byte;
        memset(buf as *mut libc::c_void, gap_fill as libc::c_int, max_gap);
        i = 0 as libc::c_int as libc::c_uint;
        while i < num_sec {
            if *gaps.offset(i as isize) != 0 as libc::c_int as libc::c_ulong {
                let mut left: bfd_size_type = 0;
                let mut off: file_ptr = 0;
                left = *gaps.offset(i as isize);
                off = (bfd_section_size(*osections.offset(i as isize)))
                    .wrapping_sub(left) as file_ptr;
                while left > 0 as libc::c_int as libc::c_ulong {
                    let mut now: bfd_size_type = 0;
                    if left > 8192 as libc::c_int as libc::c_ulong {
                        now = 8192 as libc::c_int as bfd_size_type;
                    } else {
                        now = left;
                    }
                    if !bfd_set_section_contents(
                        obfd,
                        *osections.offset(i as isize),
                        buf as *const libc::c_void,
                        off,
                        now,
                    ) {
                        bfd_nonfatal_message(
                            0 as *const libc::c_char,
                            obfd,
                            *osections.offset(i as isize),
                            0 as *const libc::c_char,
                        );
                        free(buf as *mut libc::c_void);
                        return 0 as libc::c_int != 0;
                    }
                    left = (left as libc::c_ulong).wrapping_sub(now) as bfd_size_type
                        as bfd_size_type;
                    off = (off as libc::c_ulong).wrapping_add(now) as file_ptr
                        as file_ptr;
                }
            }
            i = i.wrapping_add(1);
            i;
        }
        free(buf as *mut libc::c_void);
        free(gaps as *mut libc::c_void);
        gaps = 0 as *mut bfd_size_type;
    }
    if !(Some(
        ((*(*obfd).xvec)._bfd_copy_private_bfd_data).expect("non-null function pointer"),
    ))
        .expect("non-null function pointer")(ibfd, obfd)
    {
        bfd_nonfatal_message(
            0 as *const libc::c_char,
            obfd,
            0 as *const asection,
            dcgettext(
                0 as *const libc::c_char,
                b"error copying private BFD data\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return 0 as libc::c_int != 0;
    }
    if use_alt_mach_code != 0 as libc::c_int as libc::c_ulong {
        if !bfd_alt_mach_code(obfd, use_alt_mach_code as libc::c_int) {
            non_fatal(
                dcgettext(
                    0 as *const libc::c_char,
                    b"this target does not support %lu alternative machine codes\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                use_alt_mach_code,
            );
            if bfd_get_flavour(obfd) as libc::c_uint
                == bfd_target_elf_flavour as libc::c_int as libc::c_uint
            {
                non_fatal(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"treating that number as an absolute e_machine value instead\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                (*((*(*obfd).tdata.elf_obj_data).elf_header).as_mut_ptr())
                    .e_machine = use_alt_mach_code as libc::c_ushort;
            } else {
                non_fatal(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"ignoring the alternative value\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
            }
        }
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn copy_archive(
    mut ibfd: *mut bfd,
    mut obfd: *mut bfd,
    mut output_target: *const libc::c_char,
    mut force_output_target: bool,
    mut input_arch: *const bfd_arch_info_type,
) {
    let mut current_block: u64;
    let mut list: *mut name_list = 0 as *mut name_list;
    let mut l: *mut name_list = 0 as *mut name_list;
    let mut ptr: *mut *mut bfd = &mut (*obfd).archive_head;
    let mut this_element: *mut bfd = 0 as *mut bfd;
    let mut dir: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut filename: *const libc::c_char = 0 as *const libc::c_char;
    if (*ibfd).is_thin_archive() != 0 {
        status = 1 as libc::c_int;
        bfd_set_error(bfd_error_invalid_operation);
        bfd_nonfatal_message(
            0 as *const libc::c_char,
            ibfd,
            0 as *const asection,
            dcgettext(
                0 as *const libc::c_char,
                b"sorry: copying thin archives is not currently supported\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return;
    }
    dir = make_tempdir(bfd_get_filename(obfd));
    if dir.is_null() {
        fatal(
            dcgettext(
                0 as *const libc::c_char,
                b"cannot create tempdir for archive copying (error: %s)\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            strerror(*__errno_location()),
        );
    }
    if strip_symbols as libc::c_uint == STRIP_ALL as libc::c_int as libc::c_uint {
        (*obfd).set_has_armap(0 as libc::c_int as libc::c_uint);
    } else {
        (*obfd).set_has_armap((*ibfd).has_armap());
    }
    (*obfd).set_is_thin_archive((*ibfd).is_thin_archive());
    if deterministic != 0 {
        (*obfd).flags |= 0x2000 as libc::c_int as libc::c_uint;
    }
    list = 0 as *mut name_list;
    this_element = bfd_openr_next_archived_file(ibfd, 0 as *mut bfd);
    if !bfd_set_format(obfd, bfd_get_format(ibfd)) {
        status = 1 as libc::c_int;
        bfd_nonfatal_message(
            0 as *const libc::c_char,
            obfd,
            0 as *const asection,
            0 as *const libc::c_char,
        );
    } else {
        loop {
            if !(status == 0 && !this_element.is_null()) {
                current_block = 4216521074440650966;
                break;
            }
            let mut output_name: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut output_bfd: *mut bfd = 0 as *mut bfd;
            let mut last_element: *mut bfd = 0 as *mut bfd;
            let mut buf: stat = stat {
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
            let mut stat_status: libc::c_int = 0 as libc::c_int;
            let mut del: bool = 1 as libc::c_int != 0;
            let mut ok_object: bool = false;
            if !is_valid_archive_path(bfd_get_filename(this_element)) {
                non_fatal(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"illegal pathname found in archive member: %s\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    bfd_get_filename(this_element),
                );
                status = 1 as libc::c_int;
                current_block = 14298695868788299571;
                break;
            } else {
                output_name = concat(
                    dir,
                    b"/\0" as *const u8 as *const libc::c_char,
                    bfd_get_filename(this_element),
                    0 as *mut libc::c_char,
                );
                if stat(output_name, &mut buf) >= 0 as libc::c_int {
                    let mut tmpdir: *mut libc::c_char = make_tempdir(output_name);
                    free(output_name as *mut libc::c_void);
                    if tmpdir.is_null() {
                        non_fatal(
                            dcgettext(
                                0 as *const libc::c_char,
                                b"cannot create tempdir for archive copying (error: %s)\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            strerror(*__errno_location()),
                        );
                        status = 1 as libc::c_int;
                        current_block = 14298695868788299571;
                        break;
                    } else {
                        l = xmalloc(::core::mem::size_of::<name_list>() as libc::c_ulong)
                            as *mut name_list;
                        (*l).name = tmpdir;
                        (*l).next = list;
                        (*l).obfd = 0 as *mut bfd;
                        list = l;
                        output_name = concat(
                            tmpdir,
                            b"/\0" as *const u8 as *const libc::c_char,
                            bfd_get_filename(this_element),
                            0 as *mut libc::c_char,
                        );
                    }
                }
                if preserve_dates {
                    stat_status = (Some(
                        ((*(*if !((*this_element).my_archive).is_null() {
                            (*this_element).my_archive
                        } else {
                            this_element
                        })
                            .xvec)
                            ._bfd_stat_arch_elt)
                            .expect("non-null function pointer"),
                    ))
                        .expect("non-null function pointer")(this_element, &mut buf);
                    if stat_status != 0 as libc::c_int {
                        non_fatal(
                            dcgettext(
                                0 as *const libc::c_char,
                                b"internal stat error on %s\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            bfd_get_filename(this_element),
                        );
                    }
                }
                l = xmalloc(::core::mem::size_of::<name_list>() as libc::c_ulong)
                    as *mut name_list;
                (*l).name = output_name;
                (*l).next = list;
                (*l).obfd = 0 as *mut bfd;
                list = l;
                ok_object = bfd_check_format(this_element, bfd_object);
                if !ok_object {
                    bfd_nonfatal_message(
                        0 as *const libc::c_char,
                        this_element,
                        0 as *const asection,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Unable to recognise the format of file\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                }
                if force_output_target as libc::c_int != 0 || !ok_object {
                    output_bfd = bfd_openw(output_name, output_target);
                } else {
                    output_bfd = bfd_openw(output_name, bfd_get_target(this_element));
                }
                if output_bfd.is_null() {
                    bfd_nonfatal_message(
                        output_name,
                        0 as *const bfd,
                        0 as *const asection,
                        0 as *const libc::c_char,
                    );
                    status = 1 as libc::c_int;
                    current_block = 14298695868788299571;
                    break;
                } else {
                    if ok_object {
                        del = !copy_object(this_element, output_bfd, input_arch);
                        if del as libc::c_int != 0
                            && bfd_get_arch(this_element) as libc::c_uint
                                == bfd_arch_unknown as libc::c_int as libc::c_uint
                        {
                            ok_object = 0 as libc::c_int != 0;
                        } else if !bfd_close(output_bfd) {
                            bfd_nonfatal_message(
                                output_name,
                                0 as *const bfd,
                                0 as *const asection,
                                0 as *const libc::c_char,
                            );
                            status = 1 as libc::c_int;
                        }
                    }
                    if !ok_object {
                        del = !copy_unknown_object(this_element, output_bfd);
                        if !bfd_close_all_done(output_bfd) {
                            bfd_nonfatal_message(
                                output_name,
                                0 as *const bfd,
                                0 as *const asection,
                                0 as *const libc::c_char,
                            );
                            status = 1 as libc::c_int;
                        }
                    }
                    if del {
                        unlink(output_name);
                        status = 1 as libc::c_int;
                    } else {
                        if preserve_dates as libc::c_int != 0
                            && stat_status == 0 as libc::c_int
                        {
                            set_times(output_name, &mut buf);
                        }
                        output_bfd = bfd_openr(output_name, output_target);
                        (*l).obfd = output_bfd;
                        *ptr = output_bfd;
                        ptr = &mut (*output_bfd).archive_next;
                        last_element = this_element;
                        this_element = bfd_openr_next_archived_file(ibfd, last_element);
                        bfd_close(last_element);
                    }
                }
            }
        }
        match current_block {
            14298695868788299571 => {}
            _ => {
                *ptr = 0 as *mut bfd;
                filename = bfd_get_filename(obfd);
                if !bfd_close(obfd) {
                    status = 1 as libc::c_int;
                    bfd_nonfatal_message(
                        filename,
                        0 as *const bfd,
                        0 as *const asection,
                        0 as *const libc::c_char,
                    );
                }
                filename = bfd_get_filename(ibfd);
                if !bfd_close(ibfd) {
                    status = 1 as libc::c_int;
                    bfd_nonfatal_message(
                        filename,
                        0 as *const bfd,
                        0 as *const asection,
                        0 as *const libc::c_char,
                    );
                }
            }
        }
    }
    let mut next: *mut name_list = 0 as *mut name_list;
    l = list;
    while !l.is_null() {
        if ((*l).obfd).is_null() {
            rmdir((*l).name);
        } else {
            bfd_close((*l).obfd);
            unlink((*l).name);
        }
        next = (*l).next;
        free(l as *mut libc::c_void);
        l = next;
    }
    rmdir(dir);
}
unsafe extern "C" fn set_long_section_mode(
    mut output_bfd: *mut bfd,
    mut input_bfd: *mut bfd,
    mut style: long_section_name_handling,
) {
    if bfd_get_flavour(output_bfd) as libc::c_uint
        == bfd_target_coff_flavour as libc::c_int as libc::c_uint
    {
        if style as libc::c_uint == KEEP as libc::c_int as libc::c_uint
            && bfd_get_flavour(input_bfd) as libc::c_uint
                == bfd_target_coff_flavour as libc::c_int as libc::c_uint
        {
            style = (if (*((*(*input_bfd).xvec).backend_data
                as *mut bfd_coff_backend_data))
                ._bfd_coff_long_section_names as libc::c_int != 0
            {
                ENABLE as libc::c_int
            } else {
                DISABLE as libc::c_int
            }) as long_section_name_handling;
        }
        ((*((*(*output_bfd).xvec).backend_data as *mut bfd_coff_backend_data))
            ._bfd_coff_set_long_section_names)
            .expect(
                "non-null function pointer",
            )(
            output_bfd,
            (style as libc::c_uint != DISABLE as libc::c_int as libc::c_uint)
                as libc::c_int,
        );
    }
}
unsafe extern "C" fn copy_file(
    mut input_filename: *const libc::c_char,
    mut output_filename: *const libc::c_char,
    mut ofd: libc::c_int,
    mut in_stat: *mut stat,
    mut input_target: *const libc::c_char,
    mut output_target: *const libc::c_char,
    mut input_arch: *const bfd_arch_info_type,
) {
    let mut ibfd: *mut bfd = 0 as *mut bfd;
    let mut obj_matching: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut core_matching: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut size: off_t = get_file_size(input_filename);
    if size < 1 as libc::c_int as libc::c_long {
        if size == 0 as libc::c_int as libc::c_long {
            non_fatal(
                dcgettext(
                    0 as *const libc::c_char,
                    b"error: the input file '%s' is empty\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                input_filename,
            );
        }
        status = 1 as libc::c_int;
        return;
    }
    ibfd = bfd_openr(input_filename, input_target);
    if ibfd.is_null() || bfd_stat(ibfd, in_stat) != 0 as libc::c_int {
        bfd_nonfatal_message(
            input_filename,
            0 as *const bfd,
            0 as *const asection,
            0 as *const libc::c_char,
        );
        status = 1 as libc::c_int;
        return;
    }
    match do_debug_sections as libc::c_uint {
        1 | 3 | 5 | 9 => {
            (*ibfd).flags |= 0x4000 as libc::c_int as libc::c_uint;
            if do_debug_sections as libc::c_uint
                != compress_gnu_zlib as libc::c_int as libc::c_uint
            {
                (*ibfd).flags |= 0x20000 as libc::c_int as libc::c_uint;
            }
        }
        16 => {
            (*ibfd).flags |= 0x8000 as libc::c_int as libc::c_uint;
        }
        _ => {}
    }
    match do_elf_stt_common as libc::c_uint {
        1 => {
            (*ibfd).flags
                |= (0x40000 as libc::c_int | 0x80000 as libc::c_int) as libc::c_uint;
        }
        2 => {
            (*ibfd).flags |= 0x40000 as libc::c_int as libc::c_uint;
        }
        _ => {}
    }
    if bfd_check_format(ibfd, bfd_archive) {
        let mut force_output_target: bool = false;
        let mut obfd: *mut bfd = 0 as *mut bfd;
        if output_target.is_null() {
            output_target = bfd_get_target(ibfd);
            force_output_target = 0 as libc::c_int != 0;
        } else {
            force_output_target = 1 as libc::c_int != 0;
        }
        if ofd >= 0 as libc::c_int {
            obfd = bfd_fdopenw(output_filename, output_target, ofd);
        } else {
            obfd = bfd_openw(output_filename, output_target);
        }
        if obfd.is_null() {
            close(ofd);
            bfd_nonfatal_message(
                output_filename,
                0 as *const bfd,
                0 as *const asection,
                0 as *const libc::c_char,
            );
            status = 1 as libc::c_int;
            return;
        }
        if !gnu_debuglink_filename.is_null() {
            non_fatal(
                dcgettext(
                    0 as *const libc::c_char,
                    b"--add-gnu-debuglink ignored for archive %s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                bfd_get_filename(ibfd),
            );
            gnu_debuglink_filename = 0 as *const libc::c_char;
        }
        set_long_section_mode(obfd, ibfd, long_section_names);
        copy_archive(ibfd, obfd, output_target, force_output_target, input_arch);
    } else {
        let mut obfd_0: *mut bfd = 0 as *mut bfd;
        let mut current_block_79: u64;
        if bfd_check_format_matches(ibfd, bfd_object, &mut obj_matching) {
            obfd_0 = 0 as *mut bfd;
            current_block_79 = 1666500880240381721;
        } else {
            let mut obj_error: bfd_error_type = bfd_get_error();
            let mut core_error: bfd_error_type = bfd_error_no_error;
            if bfd_check_format_matches(ibfd, bfd_core, &mut core_matching) {
                if obj_error as libc::c_uint
                    == bfd_error_file_ambiguously_recognized as libc::c_int
                        as libc::c_uint
                {
                    free(obj_matching as *mut libc::c_void);
                }
                current_block_79 = 1666500880240381721;
            } else {
                core_error = bfd_get_error();
                if obj_error as libc::c_uint != core_error as libc::c_uint {
                    bfd_set_error(obj_error);
                }
                bfd_nonfatal_message(
                    input_filename,
                    0 as *const bfd,
                    0 as *const asection,
                    0 as *const libc::c_char,
                );
                if obj_error as libc::c_uint
                    == bfd_error_file_ambiguously_recognized as libc::c_int
                        as libc::c_uint
                {
                    list_matching_formats(obj_matching);
                    free(obj_matching as *mut libc::c_void);
                }
                if core_error as libc::c_uint
                    == bfd_error_file_ambiguously_recognized as libc::c_int
                        as libc::c_uint
                {
                    list_matching_formats(core_matching);
                    free(core_matching as *mut libc::c_void);
                }
                status = 1 as libc::c_int;
                current_block_79 = 13484060386966298149;
            }
        }
        match current_block_79 {
            1666500880240381721 => {
                if output_target.is_null() {
                    output_target = bfd_get_target(ibfd);
                }
                if ofd >= 0 as libc::c_int {
                    obfd_0 = bfd_fdopenw(output_filename, output_target, ofd);
                } else {
                    obfd_0 = bfd_openw(output_filename, output_target);
                }
                if obfd_0.is_null() {
                    close(ofd);
                    bfd_nonfatal_message(
                        output_filename,
                        0 as *const bfd,
                        0 as *const asection,
                        0 as *const libc::c_char,
                    );
                    status = 1 as libc::c_int;
                    return;
                }
                set_long_section_mode(obfd_0, ibfd, long_section_names);
                if !copy_object(ibfd, obfd_0, input_arch) {
                    status = 1 as libc::c_int;
                }
                if if status != 0 {
                    bfd_close_all_done(obfd_0) as libc::c_int
                } else {
                    bfd_close(obfd_0) as libc::c_int
                } == 0
                {
                    status = 1 as libc::c_int;
                    bfd_nonfatal_message(
                        output_filename,
                        0 as *const bfd,
                        0 as *const asection,
                        0 as *const libc::c_char,
                    );
                    return;
                }
                if !bfd_close(ibfd) {
                    status = 1 as libc::c_int;
                    bfd_nonfatal_message(
                        input_filename,
                        0 as *const bfd,
                        0 as *const asection,
                        0 as *const libc::c_char,
                    );
                    return;
                }
            }
            _ => {}
        }
    };
}
unsafe extern "C" fn add_section_rename(
    mut old_name: *const libc::c_char,
    mut new_name: *const libc::c_char,
    mut flags: flagword,
) {
    let mut srename: *mut section_rename = 0 as *mut section_rename;
    srename = section_rename_list;
    while !srename.is_null() {
        if strcmp((*srename).old_name, old_name) == 0 as libc::c_int {
            if strcmp((*srename).new_name, new_name) == 0 as libc::c_int
                && (*srename).flags == flags
            {
                return;
            }
            fatal(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Multiple renames of section %s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                old_name,
            );
        }
        srename = (*srename).next;
    }
    srename = xmalloc(::core::mem::size_of::<section_rename>() as libc::c_ulong)
        as *mut section_rename;
    (*srename).old_name = old_name;
    (*srename).new_name = new_name;
    (*srename).flags = flags;
    (*srename).next = section_rename_list;
    section_rename_list = srename;
}
unsafe extern "C" fn skip_section(
    mut ibfd: *mut bfd,
    mut isection: sec_ptr,
    mut skip_copy: bool,
) -> bool {
    let mut osection: sec_ptr = 0 as *mut bfd_section;
    let mut size: bfd_size_type = 0;
    let mut flags: flagword = 0;
    if status != 0 as libc::c_int {
        return 1 as libc::c_int != 0;
    }
    if extract_symbol {
        return 1 as libc::c_int != 0;
    }
    if is_strip_section(ibfd, isection) {
        return 1 as libc::c_int != 0;
    }
    if is_update_section(ibfd, isection) {
        return 1 as libc::c_int != 0;
    }
    if skip_copy as libc::c_int != 0
        && is_mergeable_note_section(ibfd, isection) as libc::c_int != 0
    {
        return 1 as libc::c_int != 0;
    }
    flags = bfd_section_flags(isection as *const asection);
    if flags & 0x2000000 as libc::c_int as libc::c_uint
        != 0 as libc::c_int as libc::c_uint
    {
        return 1 as libc::c_int != 0;
    }
    osection = (*isection).output_section;
    size = bfd_section_size(isection as *const asection);
    if size == 0 as libc::c_int as libc::c_ulong || osection.is_null() {
        return 1 as libc::c_int != 0;
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn handle_remove_relocations_option(
    mut section_pattern: *const libc::c_char,
) {
    find_section_list(
        section_pattern,
        1 as libc::c_int != 0,
        ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_uint,
    );
}
unsafe extern "C" fn discard_relocations(
    mut ibfd: *mut bfd,
    mut isection: *mut asection,
) -> bool {
    return !(find_section_list(
        bfd_section_name(isection),
        0 as libc::c_int != 0,
        ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_uint,
    ))
        .is_null();
}
unsafe extern "C" fn handle_remove_section_option(
    mut section_pattern: *const libc::c_char,
) {
    find_section_list(
        section_pattern,
        1 as libc::c_int != 0,
        ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint,
    );
    if startswith(section_pattern, b".rel\0" as *const u8 as *const libc::c_char) {
        section_pattern = section_pattern.offset(4 as libc::c_int as isize);
        if *section_pattern as libc::c_int == 'a' as i32 {
            section_pattern = section_pattern.offset(1);
            section_pattern;
        }
        if *section_pattern != 0 {
            handle_remove_relocations_option(section_pattern);
        }
    }
    sections_removed = 1 as libc::c_int != 0;
}
unsafe extern "C" fn default_deterministic() {
    if deterministic < 0 as libc::c_int {
        deterministic = 0 as libc::c_int;
    }
}
unsafe extern "C" fn strip_main(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut input_target: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut output_target: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut show_version: bool = 0 as libc::c_int != 0;
    let mut formats_info: bool = 0 as libc::c_int != 0;
    let mut c: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut output_file: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut merge_notes_set: bool = 0 as libc::c_int != 0;
    loop {
        c = getopt_long(
            argc,
            argv as *const *mut libc::c_char,
            b"I:O:F:K:MN:R:o:sSpdgxXHhVvwDU\0" as *const u8 as *const libc::c_char,
            strip_options.as_mut_ptr(),
            0 as *mut libc::c_int,
        );
        if !(c != -(1 as libc::c_int)) {
            break;
        }
        match c {
            73 => {
                input_target = optarg;
            }
            79 => {
                output_target = optarg;
            }
            70 => {
                output_target = optarg;
                input_target = output_target;
            }
            82 => {
                handle_remove_section_option(optarg);
            }
            179 => {
                find_section_list(
                    optarg,
                    1 as libc::c_int != 0,
                    ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uint,
                );
            }
            198 => {
                handle_remove_relocations_option(optarg);
            }
            115 => {
                strip_symbols = STRIP_ALL;
            }
            83 | 103 | 100 => {
                strip_symbols = STRIP_DEBUG;
            }
            208 => {
                strip_symbols = STRIP_DWO;
            }
            210 => {
                strip_symbols = STRIP_UNNEEDED;
            }
            75 => {
                add_specific_symbol(optarg, keep_specific_htab);
            }
            77 => {
                merge_notes = 1 as libc::c_int != 0;
                merge_notes_set = 1 as libc::c_int != 0;
            }
            186 => {
                merge_notes = 0 as libc::c_int != 0;
                merge_notes_set = 1 as libc::c_int != 0;
            }
            78 => {
                add_specific_symbol(optarg, strip_specific_htab);
            }
            111 => {
                output_file = optarg;
            }
            112 => {
                preserve_dates = 1 as libc::c_int != 0;
            }
            68 => {
                deterministic = 1 as libc::c_int;
            }
            85 => {
                deterministic = 0 as libc::c_int;
            }
            120 => {
                discard_locals = LOCALS_ALL;
            }
            88 => {
                discard_locals = LOCALS_START_L;
            }
            118 => {
                verbose = 1 as libc::c_int != 0;
            }
            86 => {
                show_version = 1 as libc::c_int != 0;
            }
            169 => {
                formats_info = 1 as libc::c_int != 0;
            }
            188 => {
                strip_symbols = STRIP_NONDEBUG;
            }
            178 => {
                keep_file_symbols = 1 as libc::c_int != 0;
            }
            181 => {
                keep_section_symbols = 1 as libc::c_int != 0;
            }
            0 => {}
            119 => {
                wildcard = 1 as libc::c_int != 0;
            }
            72 | 104 => {
                strip_usage(stdout, 0 as libc::c_int);
            }
            _ => {
                strip_usage(stderr, 1 as libc::c_int);
            }
        }
    }
    if !merge_notes_set
        && (strip_symbols as libc::c_uint == STRIP_UNDEF as libc::c_int as libc::c_uint
            || strip_symbols as libc::c_uint == STRIP_ALL as libc::c_int as libc::c_uint
            || strip_symbols as libc::c_uint
                == STRIP_UNNEEDED as libc::c_int as libc::c_uint
            || strip_symbols as libc::c_uint
                == STRIP_NONDEBUG as libc::c_int as libc::c_uint
            || strip_symbols as libc::c_uint
                == STRIP_NONDWO as libc::c_int as libc::c_uint)
    {
        merge_notes = 1 as libc::c_int != 0;
    }
    if formats_info {
        display_info();
        return 0 as libc::c_int;
    }
    if show_version {
        print_version(b"strip\0" as *const u8 as *const libc::c_char);
    }
    default_deterministic();
    if strip_symbols as libc::c_uint == STRIP_UNDEF as libc::c_int as libc::c_uint
        && discard_locals as libc::c_uint == LOCALS_UNDEF as libc::c_int as libc::c_uint
        && htab_elements(strip_specific_htab) == 0 as libc::c_int as libc::c_ulong
    {
        strip_symbols = STRIP_ALL;
    }
    if output_target.is_null() {
        output_target = input_target;
    }
    i = optind;
    if i == argc || !output_file.is_null() && (i + 1 as libc::c_int) < argc {
        strip_usage(stderr, 1 as libc::c_int);
    }
    while i < argc {
        let mut hold_status: libc::c_int = status;
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
        let mut tmpname: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut tmpfd: libc::c_int = -(1 as libc::c_int);
        let mut copyfd: libc::c_int = -(1 as libc::c_int);
        if get_file_size(*argv.offset(i as isize)) < 1 as libc::c_int as libc::c_long {
            status = 1 as libc::c_int;
        } else {
            if output_file.is_null()
                || filename_cmp(*argv.offset(i as isize), output_file)
                    == 0 as libc::c_int
            {
                tmpname = make_tempname(*argv.offset(i as isize), &mut tmpfd);
                if tmpfd >= 0 as libc::c_int {
                    copyfd = dup(tmpfd);
                }
            } else {
                tmpname = output_file;
            }
            if tmpname.is_null() {
                bfd_nonfatal_message(
                    *argv.offset(i as isize),
                    0 as *const bfd,
                    0 as *const asection,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"could not create temporary file to hold stripped copy\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                status = 1 as libc::c_int;
            } else {
                status = 0 as libc::c_int;
                copy_file(
                    *argv.offset(i as isize),
                    tmpname,
                    tmpfd,
                    &mut statbuf,
                    input_target,
                    output_target,
                    0 as *const bfd_arch_info_type,
                );
                if status == 0 as libc::c_int {
                    let mut oname: *const libc::c_char = if !output_file.is_null() {
                        output_file
                    } else {
                        *argv.offset(i as isize)
                    };
                    status = (smart_rename(
                        tmpname,
                        oname,
                        copyfd,
                        &mut statbuf,
                        preserve_dates,
                    ) != 0 as libc::c_int) as libc::c_int;
                    if status == 0 as libc::c_int {
                        status = hold_status;
                    }
                } else {
                    if copyfd >= 0 as libc::c_int {
                        close(copyfd);
                    }
                    unlink_if_ordinary(tmpname);
                }
                if output_file != tmpname {
                    free(tmpname as *mut libc::c_void);
                }
            }
        }
        i += 1;
        i;
    }
    return status;
}
unsafe extern "C" fn set_pe_subsystem(mut s: *const libc::c_char) {
    let mut version: *const libc::c_char = 0 as *const libc::c_char;
    let mut subsystem: *const libc::c_char = 0 as *const libc::c_char;
    let mut i: size_t = 0;
    static mut v: [C2RustUnnamed_48; 10] = [
        {
            let mut init = C2RustUnnamed_48 {
                name: b"native\0" as *const u8 as *const libc::c_char,
                set_def: 0 as libc::c_int as libc::c_char,
                value: 1 as libc::c_int as libc::c_short,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_48 {
                name: b"windows\0" as *const u8 as *const libc::c_char,
                set_def: 0 as libc::c_int as libc::c_char,
                value: 2 as libc::c_int as libc::c_short,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_48 {
                name: b"console\0" as *const u8 as *const libc::c_char,
                set_def: 0 as libc::c_int as libc::c_char,
                value: 3 as libc::c_int as libc::c_short,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_48 {
                name: b"posix\0" as *const u8 as *const libc::c_char,
                set_def: 0 as libc::c_int as libc::c_char,
                value: 7 as libc::c_int as libc::c_short,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_48 {
                name: b"wince\0" as *const u8 as *const libc::c_char,
                set_def: 0 as libc::c_int as libc::c_char,
                value: 9 as libc::c_int as libc::c_short,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_48 {
                name: b"efi-app\0" as *const u8 as *const libc::c_char,
                set_def: 1 as libc::c_int as libc::c_char,
                value: 10 as libc::c_int as libc::c_short,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_48 {
                name: b"efi-bsd\0" as *const u8 as *const libc::c_char,
                set_def: 1 as libc::c_int as libc::c_char,
                value: 11 as libc::c_int as libc::c_short,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_48 {
                name: b"efi-rtd\0" as *const u8 as *const libc::c_char,
                set_def: 1 as libc::c_int as libc::c_char,
                value: 12 as libc::c_int as libc::c_short,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_48 {
                name: b"sal-rtd\0" as *const u8 as *const libc::c_char,
                set_def: 1 as libc::c_int as libc::c_char,
                value: 13 as libc::c_int as libc::c_short,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_48 {
                name: b"xbox\0" as *const u8 as *const libc::c_char,
                set_def: 0 as libc::c_int as libc::c_char,
                value: 14 as libc::c_int as libc::c_short,
            };
            init
        },
    ];
    let mut value: libc::c_short = 0;
    let mut copy: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut set_def: libc::c_int = -(1 as libc::c_int);
    version = strchr(s, ':' as i32);
    if version.is_null() {
        subsystem = s;
    } else {
        let mut len: libc::c_int = version.offset_from(s) as libc::c_long as libc::c_int;
        copy = xstrdup(s);
        subsystem = copy;
        *copy.offset(len as isize) = '\0' as i32 as libc::c_char;
        version = copy.offset(1 as libc::c_int as isize).offset(len as isize);
        pe_major_subsystem_version = strtoul(version, &mut copy, 0 as libc::c_int)
            as libc::c_short;
        if *copy as libc::c_int == '.' as i32 {
            pe_minor_subsystem_version = strtoul(
                copy.offset(1 as libc::c_int as isize),
                &mut copy,
                0 as libc::c_int,
            ) as libc::c_short;
        }
        if *copy as libc::c_int != '\0' as i32 {
            non_fatal(
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: bad version in PE subsystem\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                s,
            );
        }
    }
    value = strtol(subsystem, &mut copy, 0 as libc::c_int) as libc::c_short;
    if *copy as libc::c_int == '\0' as i32 {
        i = 0 as libc::c_int as size_t;
        while i
            < (::core::mem::size_of::<[C2RustUnnamed_48; 10]>() as libc::c_ulong)
                .wrapping_div(
                    ::core::mem::size_of::<C2RustUnnamed_48>() as libc::c_ulong,
                )
        {
            if v[i as usize].value as libc::c_int == value as libc::c_int {
                pe_subsystem = value;
                set_def = v[i as usize].set_def as libc::c_int;
                break;
            } else {
                i = i.wrapping_add(1);
                i;
            }
        }
    } else {
        i = 0 as libc::c_int as size_t;
        while i
            < (::core::mem::size_of::<[C2RustUnnamed_48; 10]>() as libc::c_ulong)
                .wrapping_div(
                    ::core::mem::size_of::<C2RustUnnamed_48>() as libc::c_ulong,
                )
        {
            if strcmp(subsystem, v[i as usize].name) == 0 as libc::c_int {
                pe_subsystem = v[i as usize].value;
                set_def = v[i as usize].set_def as libc::c_int;
                break;
            } else {
                i = i.wrapping_add(1);
                i;
            }
        }
    }
    match set_def {
        -1 => {
            fatal(
                dcgettext(
                    0 as *const libc::c_char,
                    b"unknown PE subsystem: %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                s,
            );
        }
        0 => {}
        _ => {
            if pe_file_alignment == -(1 as libc::c_int) as bfd_vma {
                pe_file_alignment = 0x200 as libc::c_int as bfd_vma;
            }
            if pe_section_alignment == -(1 as libc::c_int) as bfd_vma {
                pe_section_alignment = 0x1000 as libc::c_int as bfd_vma;
            }
        }
    }
    if s != subsystem {
        free(subsystem as *mut libc::c_char as *mut libc::c_void);
    }
}
unsafe extern "C" fn convert_efi_target(mut efi: *mut libc::c_char) {
    *efi.offset(0 as libc::c_int as isize) = 'p' as i32 as libc::c_char;
    *efi.offset(1 as libc::c_int as isize) = 'e' as i32 as libc::c_char;
    *efi.offset(2 as libc::c_int as isize) = 'i' as i32 as libc::c_char;
    if strcmp(
        efi.offset(4 as libc::c_int as isize),
        b"ia32\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        *efi.offset(5 as libc::c_int as isize) = '3' as i32 as libc::c_char;
        *efi.offset(6 as libc::c_int as isize) = '8' as i32 as libc::c_char;
        *efi.offset(7 as libc::c_int as isize) = '6' as i32 as libc::c_char;
    } else if strcmp(
        efi.offset(4 as libc::c_int as isize),
        b"x86_64\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        *efi.offset(7 as libc::c_int as isize) = '-' as i32 as libc::c_char;
    }
}
unsafe extern "C" fn init_section_add(
    mut arg: *const libc::c_char,
    mut next: *mut section_add,
    mut option: *const libc::c_char,
) -> *mut section_add {
    let mut pa: *mut section_add = 0 as *mut section_add;
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    s = strchr(arg, '=' as i32);
    if s.is_null() {
        fatal(
            dcgettext(
                0 as *const libc::c_char,
                b"bad format for %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            option,
        );
    }
    pa = xmalloc(::core::mem::size_of::<section_add>() as libc::c_ulong)
        as *mut section_add;
    (*pa).name = xstrndup(arg, s.offset_from(arg) as libc::c_long as size_t);
    (*pa).filename = s.offset(1 as libc::c_int as isize);
    (*pa).next = next;
    (*pa).contents = 0 as *mut bfd_byte;
    (*pa).size = 0 as libc::c_int as size_t;
    return pa;
}
unsafe extern "C" fn section_add_load_file(mut pa: *mut section_add) {
    let mut off: size_t = 0;
    let mut alloc: size_t = 0;
    let mut f: *mut FILE = 0 as *mut FILE;
    f = fopen((*pa).filename, b"r\0" as *const u8 as *const libc::c_char);
    if f.is_null() {
        fatal(
            dcgettext(
                0 as *const libc::c_char,
                b"cannot open: %s: %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            (*pa).filename,
            strerror(*__errno_location()),
        );
    }
    off = 0 as libc::c_int as size_t;
    alloc = 4096 as libc::c_int as size_t;
    (*pa).contents = xmalloc(alloc) as *mut bfd_byte;
    while feof(f) == 0 {
        let mut got: off_t = 0;
        if off == alloc {
            alloc <<= 1 as libc::c_int;
            (*pa)
                .contents = xrealloc((*pa).contents as *mut libc::c_void, alloc)
                as *mut bfd_byte;
        }
        got = fread(
            ((*pa).contents).offset(off as isize) as *mut libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            alloc.wrapping_sub(off),
            f,
        ) as off_t;
        if ferror(f) != 0 {
            fatal(
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: fread failed\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*pa).filename,
            );
        }
        off = (off as libc::c_ulong).wrapping_add(got as libc::c_ulong) as size_t
            as size_t;
    }
    (*pa).size = off;
    fclose(f);
}
unsafe extern "C" fn copy_main(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut input_filename: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut output_filename: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmpname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut input_target: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut output_target: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut show_version: bool = 0 as libc::c_int != 0;
    let mut change_warn: bool = 1 as libc::c_int != 0;
    let mut formats_info: bool = 0 as libc::c_int != 0;
    let mut use_globalize: bool = 0 as libc::c_int != 0;
    let mut use_keep_global: bool = 0 as libc::c_int != 0;
    let mut c: libc::c_int = 0;
    let mut tmpfd: libc::c_int = -(1 as libc::c_int);
    let mut copyfd: libc::c_int = 0;
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
    let mut input_arch: *const bfd_arch_info_type = 0 as *const bfd_arch_info_type;
    loop {
        c = getopt_long(
            argc,
            argv as *const *mut libc::c_char,
            b"b:B:i:I:j:K:MN:s:O:d:F:L:G:R:SpgxXHhVvW:wDU\0" as *const u8
                as *const libc::c_char,
            copy_options.as_mut_ptr(),
            0 as *mut libc::c_int,
        );
        if !(c != -(1 as libc::c_int)) {
            break;
        }
        match c {
            98 => {
                copy_byte = atoi(optarg);
                if copy_byte < 0 as libc::c_int {
                    fatal(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"byte number must be non-negative\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                }
            }
            66 => {
                input_arch = bfd_scan_arch(optarg);
                if input_arch.is_null() {
                    fatal(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"architecture %s unknown\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        optarg,
                    );
                }
            }
            105 => {
                if !optarg.is_null() {
                    interleave = atoi(optarg);
                    if interleave < 1 as libc::c_int {
                        fatal(
                            dcgettext(
                                0 as *const libc::c_char,
                                b"interleave must be positive\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                    }
                } else {
                    interleave = 4 as libc::c_int;
                }
            }
            176 => {
                copy_width = atoi(optarg);
                if copy_width < 1 as libc::c_int {
                    fatal(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"interleave width must be positive\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                }
            }
            73 | 115 => {
                input_target = optarg;
            }
            79 | 100 => {
                output_target = optarg;
            }
            70 => {
                output_target = optarg;
                input_target = output_target;
            }
            106 => {
                find_section_list(
                    optarg,
                    1 as libc::c_int != 0,
                    ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uint,
                );
                sections_copied = 1 as libc::c_int != 0;
            }
            82 => {
                handle_remove_section_option(optarg);
            }
            179 => {
                find_section_list(
                    optarg,
                    1 as libc::c_int != 0,
                    ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uint,
                );
            }
            198 => {
                handle_remove_relocations_option(optarg);
            }
            83 => {
                strip_symbols = STRIP_ALL;
            }
            103 => {
                strip_symbols = STRIP_DEBUG;
            }
            208 => {
                strip_symbols = STRIP_DWO;
            }
            210 => {
                strip_symbols = STRIP_UNNEEDED;
            }
            188 => {
                strip_symbols = STRIP_NONDEBUG;
            }
            178 => {
                keep_file_symbols = 1 as libc::c_int != 0;
            }
            151 => {
                long_section_names = ENABLE;
                gnu_debuglink_filename = optarg;
            }
            75 => {
                add_specific_symbol(optarg, keep_specific_htab);
            }
            77 => {
                merge_notes = 1 as libc::c_int != 0;
            }
            186 => {
                merge_notes = 0 as libc::c_int != 0;
            }
            78 => {
                add_specific_symbol(optarg, strip_specific_htab);
            }
            211 => {
                add_specific_symbol(optarg, strip_unneeded_htab);
            }
            76 => {
                add_specific_symbol(optarg, localize_specific_htab);
            }
            171 => {
                use_globalize = 1 as libc::c_int != 0;
                add_specific_symbol(optarg, globalize_specific_htab);
            }
            71 => {
                use_keep_global = 1 as libc::c_int != 0;
                add_specific_symbol(optarg, keepglobal_specific_htab);
            }
            87 => {
                add_specific_symbol(optarg, weaken_specific_htab);
            }
            112 => {
                preserve_dates = 1 as libc::c_int != 0;
            }
            68 => {
                deterministic = 1 as libc::c_int;
            }
            85 => {
                deterministic = 0 as libc::c_int;
            }
            119 => {
                wildcard = 1 as libc::c_int != 0;
            }
            120 => {
                discard_locals = LOCALS_ALL;
            }
            88 => {
                discard_locals = LOCALS_START_L;
            }
            118 => {
                verbose = 1 as libc::c_int != 0;
            }
            86 => {
                show_version = 1 as libc::c_int != 0;
            }
            169 => {
                formats_info = 1 as libc::c_int != 0;
            }
            216 => {
                weaken = 1 as libc::c_int != 0;
            }
            150 => {
                add_sections = init_section_add(
                    optarg,
                    add_sections,
                    b"--add-section\0" as *const u8 as *const libc::c_char,
                );
                section_add_load_file(add_sections);
            }
            214 => {
                update_sections = init_section_add(
                    optarg,
                    update_sections,
                    b"--update-section\0" as *const u8 as *const libc::c_char,
                );
                section_add_load_file(update_sections);
            }
            164 => {
                dump_sections = init_section_add(
                    optarg,
                    dump_sections,
                    b"--dump-section\0" as *const u8 as *const libc::c_char,
                );
            }
            152 => {
                let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut newsym: *mut addsym_node = xmalloc(
                    ::core::mem::size_of::<addsym_node>() as libc::c_ulong,
                ) as *mut addsym_node;
                (*newsym).next = 0 as *mut addsym_node;
                s = strchr(optarg, '=' as i32);
                if s.is_null() {
                    fatal(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"bad format for %s\0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        b"--add-symbol\0" as *const u8 as *const libc::c_char,
                    );
                }
                t = strchr(s.offset(1 as libc::c_int as isize), ':' as i32);
                (*newsym)
                    .symdef = xstrndup(
                    optarg,
                    s.offset_from(optarg) as libc::c_long as size_t,
                );
                if !t.is_null() {
                    (*newsym)
                        .section = xstrndup(
                        s.offset(1 as libc::c_int as isize),
                        t.offset_from(s.offset(1 as libc::c_int as isize))
                            as libc::c_long as size_t,
                    );
                    (*newsym)
                        .symval = strtol(
                        t.offset(1 as libc::c_int as isize),
                        0 as *mut *mut libc::c_char,
                        0 as libc::c_int,
                    );
                } else {
                    (*newsym).section = 0 as *mut libc::c_char;
                    (*newsym)
                        .symval = strtol(
                        s.offset(1 as libc::c_int as isize),
                        0 as *mut *mut libc::c_char,
                        0 as libc::c_int,
                    );
                    t = s;
                }
                t = strchr(t.offset(1 as libc::c_int as isize), ',' as i32);
                (*newsym).othersym = 0 as *const libc::c_char;
                if !t.is_null() {
                    (*newsym)
                        .flags = parse_symflags(
                        t.offset(1 as libc::c_int as isize),
                        &mut (*newsym).othersym,
                    );
                } else {
                    (*newsym)
                        .flags = ((1 as libc::c_int) << 1 as libc::c_int) as flagword;
                }
                if !((*newsym).othersym).is_null() {
                    (*newsym).next = add_sym_list;
                    if add_sym_list.is_null() {
                        add_sym_tail = &mut (*newsym).next;
                    }
                    add_sym_list = newsym;
                } else {
                    *add_sym_tail = newsym;
                    add_sym_tail = &mut (*newsym).next;
                }
                add_symbols += 1;
                add_symbols;
            }
            159 => {
                change_start = parse_vma(
                    optarg,
                    b"--change-start\0" as *const u8 as *const libc::c_char,
                );
            }
            156 | 157 | 158 => {
                let mut p: *mut section_list = 0 as *mut section_list;
                let mut context: libc::c_uint = 0 as libc::c_int as libc::c_uint;
                let mut s_0: *const libc::c_char = 0 as *const libc::c_char;
                let mut len: libc::c_int = 0;
                let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut option: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut val: bfd_vma = 0;
                match c {
                    156 => {
                        option = b"--change-section-address\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char;
                        context = ((1 as libc::c_int) << 6 as libc::c_int
                            | (1 as libc::c_int) << 4 as libc::c_int) as libc::c_uint;
                    }
                    157 => {
                        option = b"--change-section-lma\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char;
                        context = ((1 as libc::c_int) << 6 as libc::c_int)
                            as libc::c_uint;
                    }
                    158 => {
                        option = b"--change-section-vma\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char;
                        context = ((1 as libc::c_int) << 4 as libc::c_int)
                            as libc::c_uint;
                    }
                    _ => {}
                }
                s_0 = strchr(optarg, '=' as i32);
                if s_0.is_null() {
                    s_0 = strchr(optarg, '+' as i32);
                    if s_0.is_null() {
                        s_0 = strchr(optarg, '-' as i32);
                        if s_0.is_null() {
                            fatal(
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"bad format for %s\0" as *const u8 as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                                option,
                            );
                        }
                    }
                } else {
                    match c {
                        156 => {
                            context = ((1 as libc::c_int) << 5 as libc::c_int
                                | (1 as libc::c_int) << 3 as libc::c_int) as libc::c_uint;
                        }
                        157 => {
                            context = ((1 as libc::c_int) << 5 as libc::c_int)
                                as libc::c_uint;
                        }
                        158 => {
                            context = ((1 as libc::c_int) << 3 as libc::c_int)
                                as libc::c_uint;
                        }
                        _ => {}
                    }
                }
                len = s_0.offset_from(optarg) as libc::c_long as libc::c_int;
                name = xmalloc((len + 1 as libc::c_int) as size_t) as *mut libc::c_char;
                strncpy(name, optarg, len as libc::c_ulong);
                *name.offset(len as isize) = '\0' as i32 as libc::c_char;
                p = find_section_list(name, 1 as libc::c_int != 0, context);
                val = parse_vma(s_0.offset(1 as libc::c_int as isize), option);
                if *s_0 as libc::c_int == '-' as i32 {
                    val = val.wrapping_neg();
                }
                let mut current_block_119: u64;
                match c {
                    156 => {
                        (*p).vma_val = val;
                        current_block_119 = 4174189276627025355;
                    }
                    157 => {
                        current_block_119 = 4174189276627025355;
                    }
                    158 => {
                        (*p).vma_val = val;
                        current_block_119 = 8507773468922410051;
                    }
                    _ => {
                        current_block_119 = 8507773468922410051;
                    }
                }
                match current_block_119 {
                    4174189276627025355 => {
                        (*p).lma_val = val;
                    }
                    _ => {}
                }
            }
            154 => {
                change_section_address = parse_vma(
                    optarg,
                    b"--change-addresses\0" as *const u8 as *const libc::c_char,
                );
                change_start = change_section_address;
            }
            160 => {
                change_warn = 1 as libc::c_int != 0;
            }
            155 => {
                change_leading_char = 1 as libc::c_int != 0;
            }
            161 => {
                if !optarg.is_null() {
                    if strcasecmp(optarg, b"none\0" as *const u8 as *const libc::c_char)
                        == 0 as libc::c_int
                    {
                        do_debug_sections = decompress;
                    } else if strcasecmp(
                        optarg,
                        b"zlib\0" as *const u8 as *const libc::c_char,
                    ) == 0 as libc::c_int
                    {
                        do_debug_sections = compress_zlib;
                    } else if strcasecmp(
                        optarg,
                        b"zlib-gnu\0" as *const u8 as *const libc::c_char,
                    ) == 0 as libc::c_int
                    {
                        do_debug_sections = compress_gnu_zlib;
                    } else if strcasecmp(
                        optarg,
                        b"zlib-gabi\0" as *const u8 as *const libc::c_char,
                    ) == 0 as libc::c_int
                    {
                        do_debug_sections = compress_gabi_zlib;
                    } else {
                        fatal(
                            dcgettext(
                                0 as *const libc::c_char,
                                b"unrecognized --compress-debug-sections type `%s'\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            optarg,
                        );
                    }
                } else {
                    do_debug_sections = compress;
                }
            }
            162 => {
                convert_debugging = 1 as libc::c_int != 0;
            }
            163 => {
                do_debug_sections = decompress;
            }
            165 => {
                if strcasecmp(optarg, b"yes\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    do_elf_stt_common = elf_stt_common;
                } else if strcasecmp(optarg, b"no\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    do_elf_stt_common = no_elf_stt_common;
                } else {
                    fatal(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"unrecognized --elf-stt-common= option `%s'\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        optarg,
                    );
                }
            }
            170 => {
                let mut gap_fill_vma: bfd_vma = 0;
                gap_fill_vma = parse_vma(
                    optarg,
                    b"--gap-fill\0" as *const u8 as *const libc::c_char,
                );
                gap_fill = gap_fill_vma as bfd_byte;
                if gap_fill as bfd_vma != gap_fill_vma {
                    let mut buff: [libc::c_char; 20] = [0; 20];
                    sprintf(
                        buff.as_mut_ptr(),
                        b"%016lx\0" as *const u8 as *const libc::c_char,
                        gap_fill_vma,
                    );
                    non_fatal(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Warning: truncating gap-fill from 0x%s to 0x%x\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        buff.as_mut_ptr(),
                        gap_fill as libc::c_int,
                    );
                }
                gap_fill_set = 1 as libc::c_int != 0;
            }
            187 => {
                change_warn = 0 as libc::c_int != 0;
            }
            189 => {
                pad_to = parse_vma(
                    optarg,
                    b"--pad-to\0" as *const u8 as *const libc::c_char,
                );
                pad_to_set = 1 as libc::c_int != 0;
            }
            197 => {
                remove_leading_char = 1 as libc::c_int != 0;
            }
            195 => {
                let mut len_0: libc::c_int = 0;
                let mut s_1: *const libc::c_char = 0 as *const libc::c_char;
                let mut nextarg: *const libc::c_char = 0 as *const libc::c_char;
                let mut source: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut target: *mut libc::c_char = 0 as *mut libc::c_char;
                s_1 = strchr(optarg, '=' as i32);
                if s_1.is_null() {
                    fatal(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"bad format for %s\0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        b"--redefine-sym\0" as *const u8 as *const libc::c_char,
                    );
                }
                len_0 = s_1.offset_from(optarg) as libc::c_long as libc::c_int;
                source = xmalloc((len_0 + 1 as libc::c_int) as size_t)
                    as *mut libc::c_char;
                strncpy(source, optarg, len_0 as libc::c_ulong);
                *source.offset(len_0 as isize) = '\0' as i32 as libc::c_char;
                nextarg = s_1.offset(1 as libc::c_int as isize);
                len_0 = strlen(nextarg) as libc::c_int;
                target = xmalloc((len_0 + 1 as libc::c_int) as size_t)
                    as *mut libc::c_char;
                strcpy(target, nextarg);
                add_redefine_and_check(
                    b"--redefine-sym\0" as *const u8 as *const libc::c_char,
                    source,
                    target,
                );
                free(source as *mut libc::c_void);
                free(target as *mut libc::c_void);
            }
            196 => {
                add_redefine_syms_file(optarg);
            }
            202 => {
                let mut p_0: *mut section_list = 0 as *mut section_list;
                let mut s_2: *const libc::c_char = 0 as *const libc::c_char;
                let mut len_1: libc::c_int = 0;
                let mut name_0: *mut libc::c_char = 0 as *mut libc::c_char;
                s_2 = strchr(optarg, '=' as i32);
                if s_2.is_null() {
                    fatal(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"bad format for %s\0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        b"--set-section-flags\0" as *const u8 as *const libc::c_char,
                    );
                }
                len_1 = s_2.offset_from(optarg) as libc::c_long as libc::c_int;
                name_0 = xmalloc((len_1 + 1 as libc::c_int) as size_t)
                    as *mut libc::c_char;
                strncpy(name_0, optarg, len_1 as libc::c_ulong);
                *name_0.offset(len_1 as isize) = '\0' as i32 as libc::c_char;
                p_0 = find_section_list(
                    name_0,
                    1 as libc::c_int != 0,
                    ((1 as libc::c_int) << 7 as libc::c_int) as libc::c_uint,
                );
                (*p_0).flags = parse_flags(s_2.offset(1 as libc::c_int as isize));
            }
            203 => {
                let mut p_1: *mut section_list = 0 as *mut section_list;
                let mut s_3: *const libc::c_char = 0 as *const libc::c_char;
                let mut len_2: libc::c_int = 0;
                let mut name_1: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut palign: libc::c_int = 0;
                let mut align: libc::c_int = 0;
                s_3 = strchr(optarg, '=' as i32);
                if s_3.is_null() {
                    fatal(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"bad format for --set-section-alignment: argument needed\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                }
                align = atoi(s_3.offset(1 as libc::c_int as isize));
                if align <= 0 as libc::c_int {
                    fatal(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"bad format for --set-section-alignment: numeric argument needed\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                }
                palign = 0 as libc::c_int;
                while align & 1 as libc::c_int == 0 as libc::c_int {
                    align >>= 1 as libc::c_int;
                    palign += 1;
                    palign;
                }
                if align != 1 as libc::c_int {
                    fatal(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"bad format for --set-section-alignment: alignment is not a power of two\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                }
                len_2 = s_3.offset_from(optarg) as libc::c_long as libc::c_int;
                name_1 = xmalloc((len_2 + 1 as libc::c_int) as size_t)
                    as *mut libc::c_char;
                strncpy(name_1, optarg, len_2 as libc::c_ulong);
                *name_1.offset(len_2 as isize) = '\0' as i32 as libc::c_char;
                p_1 = find_section_list(
                    name_1,
                    1 as libc::c_int != 0,
                    ((1 as libc::c_int) << 9 as libc::c_int) as libc::c_uint,
                );
                if !p_1.is_null() {
                    (*p_1).alignment = palign as libc::c_uint;
                }
            }
            199 => {
                let mut flags: flagword = 0;
                let mut eq: *const libc::c_char = 0 as *const libc::c_char;
                let mut fl: *const libc::c_char = 0 as *const libc::c_char;
                let mut old_name: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut new_name: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut len_3: libc::c_uint = 0;
                eq = strchr(optarg, '=' as i32);
                if eq.is_null() {
                    fatal(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"bad format for %s\0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        b"--rename-section\0" as *const u8 as *const libc::c_char,
                    );
                }
                len_3 = eq.offset_from(optarg) as libc::c_long as libc::c_uint;
                if len_3 == 0 as libc::c_int as libc::c_uint {
                    fatal(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"bad format for %s\0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        b"--rename-section\0" as *const u8 as *const libc::c_char,
                    );
                }
                old_name = xmalloc(
                    len_3.wrapping_add(1 as libc::c_int as libc::c_uint) as size_t,
                ) as *mut libc::c_char;
                strncpy(old_name, optarg, len_3 as libc::c_ulong);
                *old_name.offset(len_3 as isize) = 0 as libc::c_int as libc::c_char;
                eq = eq.offset(1);
                eq;
                fl = strchr(eq, ',' as i32);
                if !fl.is_null() {
                    flags = parse_flags(fl.offset(1 as libc::c_int as isize));
                    len_3 = fl.offset_from(eq) as libc::c_long as libc::c_uint;
                } else {
                    flags = -(1 as libc::c_int) as flagword;
                    len_3 = strlen(eq) as libc::c_uint;
                }
                if len_3 == 0 as libc::c_int as libc::c_uint {
                    fatal(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"bad format for %s\0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        b"--rename-section\0" as *const u8 as *const libc::c_char,
                    );
                }
                new_name = xmalloc(
                    len_3.wrapping_add(1 as libc::c_int as libc::c_uint) as size_t,
                ) as *mut libc::c_char;
                strncpy(new_name, eq, len_3 as libc::c_ulong);
                *new_name.offset(len_3 as isize) = 0 as libc::c_int as libc::c_char;
                add_section_rename(old_name, new_name, flags);
            }
            204 => {
                set_start = parse_vma(
                    optarg,
                    b"--set-start\0" as *const u8 as *const libc::c_char,
                );
                set_start_set = 1 as libc::c_int != 0;
            }
            206 => {
                _bfd_srec_len = parse_vma(
                    optarg,
                    b"--srec-len\0" as *const u8 as *const libc::c_char,
                ) as libc::c_uint;
            }
            205 => {
                _bfd_srec_forceS3 = 1 as libc::c_int != 0;
            }
            209 => {
                add_specific_symbols(
                    optarg,
                    strip_specific_htab,
                    &mut strip_specific_buffer,
                );
            }
            212 => {
                add_specific_symbols(
                    optarg,
                    strip_unneeded_htab,
                    &mut strip_unneeded_buffer,
                );
            }
            180 => {
                add_specific_symbols(
                    optarg,
                    keep_specific_htab,
                    &mut keep_specific_buffer,
                );
            }
            181 => {
                keep_section_symbols = 1 as libc::c_int != 0;
            }
            182 => {
                localize_hidden = 1 as libc::c_int != 0;
            }
            183 => {
                add_specific_symbols(
                    optarg,
                    localize_specific_htab,
                    &mut localize_specific_buffer,
                );
            }
            184 => {
                if strcmp(b"enable\0" as *const u8 as *const libc::c_char, optarg) == 0 {
                    long_section_names = ENABLE;
                } else if strcmp(
                    b"disable\0" as *const u8 as *const libc::c_char,
                    optarg,
                ) == 0
                {
                    long_section_names = DISABLE;
                } else if strcmp(b"keep\0" as *const u8 as *const libc::c_char, optarg)
                    == 0
                {
                    long_section_names = KEEP;
                } else {
                    fatal(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"unknown long section names option '%s'\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        optarg,
                    );
                }
            }
            172 => {
                use_globalize = 1 as libc::c_int != 0;
                add_specific_symbols(
                    optarg,
                    globalize_specific_htab,
                    &mut globalize_specific_buffer,
                );
            }
            177 => {
                use_keep_global = 1 as libc::c_int != 0;
                add_specific_symbols(
                    optarg,
                    keepglobal_specific_htab,
                    &mut keepglobal_specific_buffer,
                );
            }
            217 => {
                add_specific_symbols(
                    optarg,
                    weaken_specific_htab,
                    &mut weaken_specific_buffer,
                );
            }
            153 => {
                use_alt_mach_code = strtoul(
                    optarg,
                    0 as *mut *mut libc::c_char,
                    0 as libc::c_int,
                );
                if use_alt_mach_code == 0 as libc::c_int as libc::c_ulong {
                    fatal(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"unable to parse alternative machine code\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                }
            }
            192 => {
                prefix_symbols_string = optarg;
            }
            191 => {
                prefix_sections_string = optarg;
            }
            190 => {
                prefix_alloc_sections_string = optarg;
            }
            194 => {
                bfd_flags_to_set |= 0x80 as libc::c_int as libc::c_uint;
                bfd_flags_to_clear &= !(0x80 as libc::c_int) as libc::c_uint;
            }
            218 => {
                bfd_flags_to_clear |= 0x80 as libc::c_int as libc::c_uint;
                bfd_flags_to_set &= !(0x80 as libc::c_int) as libc::c_uint;
            }
            193 => {
                bfd_flags_to_set |= 0x100 as libc::c_int as libc::c_uint;
                bfd_flags_to_clear &= !(0x100 as libc::c_int) as libc::c_uint;
            }
            175 => {
                bfd_flags_to_clear |= 0x100 as libc::c_int as libc::c_uint;
                bfd_flags_to_set &= !(0x100 as libc::c_int) as libc::c_uint;
            }
            166 => {
                strip_symbols = STRIP_NONDWO;
            }
            167 => {
                extract_symbol = 1 as libc::c_int != 0;
            }
            200 => {
                let mut prev: libc::c_int = reverse_bytes;
                reverse_bytes = atoi(optarg);
                if reverse_bytes <= 0 as libc::c_int
                    || reverse_bytes % 2 as libc::c_int != 0 as libc::c_int
                {
                    fatal(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"number of bytes to reverse must be positive and even\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                }
                if prev != 0 && prev != reverse_bytes {
                    non_fatal(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Warning: ignoring previous --reverse-bytes value of %d\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        prev,
                    );
                }
            }
            168 => {
                pe_file_alignment = parse_vma(
                    optarg,
                    b"--file-alignment\0" as *const u8 as *const libc::c_char,
                );
            }
            173 => {
                let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
                pe_heap_reserve = strtoul(optarg, &mut end, 0 as libc::c_int);
                if end == optarg
                    || *end as libc::c_int != '.' as i32
                        && *end as libc::c_int != '\0' as i32
                {
                    non_fatal(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"%s: invalid reserve value for --heap\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        optarg,
                    );
                } else if *end as libc::c_int != '\0' as i32 {
                    pe_heap_commit = strtoul(
                        end.offset(1 as libc::c_int as isize),
                        &mut end,
                        0 as libc::c_int,
                    );
                    if *end as libc::c_int != '\0' as i32 {
                        non_fatal(
                            dcgettext(
                                0 as *const libc::c_char,
                                b"%s: invalid commit value for --heap\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            optarg,
                        );
                    }
                }
            }
            174 => {
                pe_image_base = parse_vma(
                    optarg,
                    b"--image-base\0" as *const u8 as *const libc::c_char,
                );
            }
            201 => {
                pe_section_alignment = parse_vma(
                    optarg,
                    b"--section-alignment\0" as *const u8 as *const libc::c_char,
                );
            }
            213 => {
                set_pe_subsystem(optarg);
            }
            207 => {
                let mut end_0: *mut libc::c_char = 0 as *mut libc::c_char;
                pe_stack_reserve = strtoul(optarg, &mut end_0, 0 as libc::c_int);
                if end_0 == optarg
                    || *end_0 as libc::c_int != '.' as i32
                        && *end_0 as libc::c_int != '\0' as i32
                {
                    non_fatal(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"%s: invalid reserve value for --stack\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        optarg,
                    );
                } else if *end_0 as libc::c_int != '\0' as i32 {
                    pe_stack_commit = strtoul(
                        end_0.offset(1 as libc::c_int as isize),
                        &mut end_0,
                        0 as libc::c_int,
                    );
                    if *end_0 as libc::c_int != '\0' as i32 {
                        non_fatal(
                            dcgettext(
                                0 as *const libc::c_char,
                                b"%s: invalid commit value for --stack\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            optarg,
                        );
                    }
                }
            }
            215 => {
                VerilogDataWidth = parse_vma(
                    optarg,
                    b"--verilog-data-width\0" as *const u8 as *const libc::c_char,
                ) as libc::c_uint;
                if VerilogDataWidth < 1 as libc::c_int as libc::c_uint {
                    fatal(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"verilog data width must be at least 1 byte\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                }
            }
            0 => {}
            72 | 104 => {
                copy_usage(stdout, 0 as libc::c_int);
            }
            _ => {
                copy_usage(stderr, 1 as libc::c_int);
            }
        }
    }
    if use_globalize as libc::c_int != 0 && use_keep_global as libc::c_int != 0 {
        fatal(
            dcgettext(
                0 as *const libc::c_char,
                b"--globalize-symbol(s) is incompatible with -G/--keep-global-symbol(s)\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    if formats_info {
        display_info();
        return 0 as libc::c_int;
    }
    if show_version {
        print_version(b"objcopy\0" as *const u8 as *const libc::c_char);
    }
    if interleave != 0 && copy_byte == -(1 as libc::c_int) {
        fatal(
            dcgettext(
                0 as *const libc::c_char,
                b"interleave start byte must be set with --byte\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    if copy_byte >= interleave {
        fatal(
            dcgettext(
                0 as *const libc::c_char,
                b"byte number must be less than interleave\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    if copy_width > interleave - copy_byte {
        fatal(
            dcgettext(
                0 as *const libc::c_char,
                b"interleave width must be less than or equal to interleave - byte`\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    if optind == argc || (optind + 2 as libc::c_int) < argc {
        copy_usage(stderr, 1 as libc::c_int);
    }
    input_filename = *argv.offset(optind as isize);
    if (optind + 1 as libc::c_int) < argc {
        output_filename = *argv.offset((optind + 1 as libc::c_int) as isize);
    }
    default_deterministic();
    if strip_symbols as libc::c_uint == STRIP_UNDEF as libc::c_int as libc::c_uint
        && discard_locals as libc::c_uint == LOCALS_UNDEF as libc::c_int as libc::c_uint
    {
        strip_symbols = STRIP_NONE;
    }
    if output_target.is_null() {
        output_target = input_target;
    }
    if !input_target.is_null()
        && startswith(input_target, b"efi-\0" as *const u8 as *const libc::c_char)
            as libc::c_int != 0
    {
        let mut efi: *mut libc::c_char = 0 as *mut libc::c_char;
        efi = xstrdup(output_target.offset(4 as libc::c_int as isize));
        if startswith(efi, b"bsdrv-\0" as *const u8 as *const libc::c_char)
            as libc::c_int != 0
            || startswith(efi, b"rtdrv-\0" as *const u8 as *const libc::c_char)
                as libc::c_int != 0
        {
            efi = efi.offset(2 as libc::c_int as isize);
        } else if !startswith(efi, b"app-\0" as *const u8 as *const libc::c_char) {
            fatal(
                dcgettext(
                    0 as *const libc::c_char,
                    b"unknown input EFI target: %s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                input_target,
            );
        }
        input_target = efi;
        convert_efi_target(efi);
    }
    if !output_target.is_null()
        && startswith(output_target, b"efi-\0" as *const u8 as *const libc::c_char)
            as libc::c_int != 0
    {
        let mut efi_0: *mut libc::c_char = 0 as *mut libc::c_char;
        efi_0 = xstrdup(output_target.offset(4 as libc::c_int as isize));
        if startswith(efi_0, b"app-\0" as *const u8 as *const libc::c_char) {
            if pe_subsystem as libc::c_int == -(1 as libc::c_int) {
                pe_subsystem = 10 as libc::c_int as libc::c_short;
            }
        } else if startswith(efi_0, b"bsdrv-\0" as *const u8 as *const libc::c_char) {
            if pe_subsystem as libc::c_int == -(1 as libc::c_int) {
                pe_subsystem = 11 as libc::c_int as libc::c_short;
            }
            efi_0 = efi_0.offset(2 as libc::c_int as isize);
        } else if startswith(efi_0, b"rtdrv-\0" as *const u8 as *const libc::c_char) {
            if pe_subsystem as libc::c_int == -(1 as libc::c_int) {
                pe_subsystem = 12 as libc::c_int as libc::c_short;
            }
            efi_0 = efi_0.offset(2 as libc::c_int as isize);
        } else {
            fatal(
                dcgettext(
                    0 as *const libc::c_char,
                    b"unknown output EFI target: %s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                output_target,
            );
        }
        if pe_file_alignment == -(1 as libc::c_int) as bfd_vma {
            pe_file_alignment = 0x200 as libc::c_int as bfd_vma;
        }
        if pe_section_alignment == -(1 as libc::c_int) as bfd_vma {
            pe_section_alignment = 0x1000 as libc::c_int as bfd_vma;
        }
        output_target = efi_0;
        convert_efi_target(efi_0);
    }
    copyfd = -(1 as libc::c_int);
    if output_filename.is_null()
        || filename_cmp(input_filename, output_filename) == 0 as libc::c_int
    {
        tmpname = make_tempname(input_filename, &mut tmpfd);
        if tmpfd >= 0 as libc::c_int {
            copyfd = dup(tmpfd);
        }
    } else {
        tmpname = output_filename;
    }
    if tmpname.is_null() {
        fatal(
            dcgettext(
                0 as *const libc::c_char,
                b"warning: could not create temporary file whilst copying '%s', (error: %s)\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            input_filename,
            strerror(*__errno_location()),
        );
    }
    copy_file(
        input_filename,
        tmpname,
        tmpfd,
        &mut statbuf,
        input_target,
        output_target,
        input_arch,
    );
    if status == 0 as libc::c_int {
        let mut oname: *const libc::c_char = if !output_filename.is_null() {
            output_filename
        } else {
            input_filename
        };
        status = (smart_rename(tmpname, oname, copyfd, &mut statbuf, preserve_dates)
            != 0 as libc::c_int) as libc::c_int;
    } else {
        if copyfd >= 0 as libc::c_int {
            close(copyfd);
        }
        unlink_if_ordinary(tmpname);
    }
    if tmpname != output_filename {
        free(tmpname as *mut libc::c_void);
    }
    if change_warn {
        let mut p_2: *mut section_list = 0 as *mut section_list;
        p_2 = change_sections;
        while !p_2.is_null() {
            if !(*p_2).used {
                if (*p_2).context
                    & ((1 as libc::c_int) << 3 as libc::c_int
                        | (1 as libc::c_int) << 4 as libc::c_int) as libc::c_uint != 0
                {
                    let mut buff_0: [libc::c_char; 20] = [0; 20];
                    sprintf(
                        buff_0.as_mut_ptr(),
                        b"%016lx\0" as *const u8 as *const libc::c_char,
                        (*p_2).vma_val,
                    );
                    non_fatal(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"%s %s%c0x%s never used\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        b"--change-section-vma\0" as *const u8 as *const libc::c_char,
                        (*p_2).pattern,
                        if (*p_2).context
                            & ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_uint
                            != 0
                        {
                            '=' as i32
                        } else {
                            '+' as i32
                        },
                        buff_0.as_mut_ptr(),
                    );
                }
                if (*p_2).context
                    & ((1 as libc::c_int) << 5 as libc::c_int
                        | (1 as libc::c_int) << 6 as libc::c_int) as libc::c_uint != 0
                {
                    let mut buff_1: [libc::c_char; 20] = [0; 20];
                    sprintf(
                        buff_1.as_mut_ptr(),
                        b"%016lx\0" as *const u8 as *const libc::c_char,
                        (*p_2).lma_val,
                    );
                    non_fatal(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"%s %s%c0x%s never used\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        b"--change-section-lma\0" as *const u8 as *const libc::c_char,
                        (*p_2).pattern,
                        if (*p_2).context
                            & ((1 as libc::c_int) << 5 as libc::c_int) as libc::c_uint
                            != 0
                        {
                            '=' as i32
                        } else {
                            '+' as i32
                        },
                        buff_1.as_mut_ptr(),
                    );
                }
            }
            p_2 = (*p_2).next;
        }
    }
    free(strip_specific_buffer as *mut libc::c_void);
    free(strip_unneeded_buffer as *mut libc::c_void);
    free(keep_specific_buffer as *mut libc::c_void);
    free(localize_specific_buffer as *mut libc::c_void);
    free(globalize_specific_buffer as *mut libc::c_void);
    free(keepglobal_specific_buffer as *mut libc::c_void);
    free(weaken_specific_buffer as *mut libc::c_void);
    return 0 as libc::c_int;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    setlocale(5 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
    setlocale(0 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
    bindtextdomain(
        b"binutils\0" as *const u8 as *const libc::c_char,
        b"/usr/local/share/locale\0" as *const u8 as *const libc::c_char,
    );
    textdomain(b"binutils\0" as *const u8 as *const libc::c_char);
    program_name = *argv.offset(0 as libc::c_int as isize);
    xmalloc_set_program_name(program_name);
    expandargv(&mut argc, &mut argv);
    strip_symbols = STRIP_UNDEF;
    discard_locals = LOCALS_UNDEF;
    if bfd_init() as libc::c_ulong
        != ::core::mem::size_of::<bfd_section>() as libc::c_ulong
    {
        fatal(
            dcgettext(
                0 as *const libc::c_char,
                b"fatal error: libbfd ABI mismatch\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    set_default_bfd_target();
    if is_strip < 0 as libc::c_int {
        let mut i: libc::c_int = strlen(program_name) as libc::c_int;
        is_strip = (i >= 5 as libc::c_int
            && filename_cmp(
                program_name.offset(i as isize).offset(-(5 as libc::c_int as isize)),
                b"strip\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int) as libc::c_int;
    }
    create_symbol_htabs();
    if !argv.is_null() {
        bfd_set_error_program_name(*argv.offset(0 as libc::c_int as isize));
    }
    if is_strip != 0 {
        strip_main(argc, argv);
    } else {
        copy_main(argc, argv);
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
