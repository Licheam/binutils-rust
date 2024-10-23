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
extern crate bfd_sys;
extern crate libctf_sys;
extern crate libiberty_sys;
extern crate zlib_sys;
extern crate libc;
pub mod src {
pub mod bucomm;
pub mod debug;
pub mod dwarf;
pub mod elfcomm;
pub mod filemode;
pub mod prdbg;
pub mod rdcoff;
pub mod rddbg;
pub mod stabs;
pub mod version;
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
    pub type xcoff_tdata;
    pub type pe_tdata;
    pub type coff_tdata;
    pub type artdata;
    pub type aout_data_struct;
    pub type bfd_iovec;
    pub type _bfd_window_internal;
    pub type ecoff_debug_swap;
    pub type internal_reloc;
    pub type ctf_dict;
    pub type ctf_archive_internal;
    pub type ctf_dump_state;
    pub type ctf_next;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn vsnprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ::core::ffi::VaList,
    ) -> libc::c_int;
    fn asprintf(
        __ptr: *mut *mut libc::c_char,
        __fmt: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
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
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn abort() -> !;
    fn exit(_: libc::c_int) -> !;
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
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
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strcspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    fn getpagesize() -> libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
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
    fn bfd_sprintf_vma(_: *mut bfd, _: *mut libc::c_char, _: bfd_vma);
    fn bfd_fprintf_vma(_: *mut bfd, _: *mut libc::c_void, _: bfd_vma);
    fn bfd_get_stab_name(_: libc::c_int) -> *const libc::c_char;
    fn bfd_init() -> libc::c_uint;
    fn bfd_openr(filename: *const libc::c_char, target: *const libc::c_char) -> *mut bfd;
    fn bfd_close(abfd: *mut bfd) -> bool;
    fn bfd_close_all_done(_: *mut bfd) -> bool;
    fn bfd_get_mtime(abfd: *mut bfd) -> libc::c_long;
    fn bfd_get_file_size(abfd: *mut bfd) -> ufile_ptr;
    static mut _bfd_std_section: [asection; 4];
    fn bfd_get_section_by_name(
        abfd: *mut bfd,
        name: *const libc::c_char,
    ) -> *mut asection;
    fn bfd_map_over_sections(
        abfd: *mut bfd,
        func: Option::<
            unsafe extern "C" fn(*mut bfd, *mut asection, *mut libc::c_void) -> (),
        >,
        obj: *mut libc::c_void,
    );
    fn bfd_malloc_and_get_section(
        abfd: *mut bfd,
        section: *mut asection,
        buf: *mut *mut bfd_byte,
    ) -> bool;
    fn bfd_scan_arch(string: *const libc::c_char) -> *const bfd_arch_info_type;
    fn bfd_get_arch(abfd: *const bfd) -> bfd_architecture;
    fn bfd_get_mach(abfd: *const bfd) -> libc::c_ulong;
    fn bfd_arch_bits_per_address(abfd: *const bfd) -> libc::c_uint;
    fn bfd_printable_arch_mach(
        arch: bfd_architecture,
        machine_0: libc::c_ulong,
    ) -> *const libc::c_char;
    fn bfd_octets_per_byte(abfd: *const bfd, sec: *const asection) -> libc::c_uint;
    fn bfd_is_local_label(abfd: *mut bfd, sym: *mut asymbol) -> bool;
    fn bfd_get_error() -> bfd_error_type;
    fn bfd_set_error(error_tag: bfd_error_type);
    fn bfd_errmsg(error_tag: bfd_error_type) -> *const libc::c_char;
    fn bfd_set_error_program_name(_: *const libc::c_char);
    fn bfd_get_reloc_upper_bound(abfd: *mut bfd, sect: *mut asection) -> libc::c_long;
    fn bfd_canonicalize_reloc(
        abfd: *mut bfd,
        sec: *mut asection,
        loc_0: *mut *mut arelent,
        syms_0: *mut *mut asymbol,
    ) -> libc::c_long;
    fn bfd_get_arch_size(abfd: *mut bfd) -> libc::c_int;
    fn bfd_demangle(
        _: *mut bfd,
        _: *const libc::c_char,
        _: libc::c_int,
    ) -> *mut libc::c_char;
    fn bfd_openr_next_archived_file(archive: *mut bfd, previous: *mut bfd) -> *mut bfd;
    fn bfd_check_format(abfd: *mut bfd, format: bfd_format) -> bool;
    fn bfd_check_format_matches(
        abfd: *mut bfd,
        format: bfd_format,
        matching: *mut *mut *mut libc::c_char,
    ) -> bool;
    fn bfd_simple_get_relocated_section_contents(
        abfd: *mut bfd,
        sec: *mut asection,
        outbuf: *mut bfd_byte,
        symbol_table: *mut *mut asymbol,
    ) -> *mut bfd_byte;
    fn bfd_get_full_section_contents(
        abfd: *mut bfd,
        section: *mut asection,
        ptr: *mut *mut bfd_byte,
    ) -> bool;
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
    fn list_supported_architectures(_: *const libc::c_char, _: *mut FILE);
    fn display_info() -> libc::c_int;
    fn print_arelt_descr(_: *mut FILE, _: *mut bfd, _: bool, _: bool);
    fn parse_vma(_: *const libc::c_char, _: *const libc::c_char) -> bfd_vma;
    fn get_file_size(_: *const libc::c_char) -> off_t;
    static mut program_name: *mut libc::c_char;
    fn print_version(_: *const libc::c_char);
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    fn xrealloc(_: *mut libc::c_void, _: size_t) -> *mut libc::c_void;
    fn warn(_: *const libc::c_char, _: ...);
    static mut byte_get: Option::<
        unsafe extern "C" fn(*const libc::c_uchar, libc::c_uint) -> elf_vma,
    >;
    fn byte_get_little_endian(_: *const libc::c_uchar, _: libc::c_uint) -> elf_vma;
    fn byte_get_big_endian(_: *const libc::c_uchar, _: libc::c_uint) -> elf_vma;
    static mut do_follow_links: libc::c_int;
    static mut dwarf_cutoff_level: libc::c_int;
    static mut dwarf_start_die: libc::c_ulong;
    static mut dwarf_check: libc::c_int;
    fn init_dwarf_regnames_by_bfd_arch_and_mach(
        arch: bfd_architecture,
        mach: libc::c_ulong,
    );
    static mut do_wide: libc::c_int;
    fn load_separate_debug_files(_: *mut libc::c_void, _: *const libc::c_char) -> bool;
    fn free_debug_memory();
    fn dwarf_select_sections_by_names(_: *const libc::c_char);
    fn dwarf_select_sections_by_letters(_: *const libc::c_char);
    fn dwarf_select_sections_all();
    static mut eh_addr_size: libc::c_uint;
    static mut first_separate_info: *mut separate_info;
    static mut debug_displays: [dwarf_section_display; 0];
    fn ctf_bfdopen_ctfsect(
        _: *mut bfd,
        _: *const ctf_sect_t,
        _: *mut libc::c_int,
    ) -> *mut ctf_archive_t;
    fn ctf_close(_: *mut ctf_archive_t);
    fn ctf_dict_open(
        _: *const ctf_archive_t,
        _: *const libc::c_char,
        _: *mut libc::c_int,
    ) -> *mut ctf_dict_t;
    fn ctf_dict_close(_: *mut ctf_dict_t);
    fn ctf_import(_: *mut ctf_dict_t, _: *mut ctf_dict_t) -> libc::c_int;
    fn ctf_errno(_: *mut ctf_dict_t) -> libc::c_int;
    fn ctf_errmsg(_: libc::c_int) -> *const libc::c_char;
    fn ctf_archive_iter(
        _: *const ctf_archive_t,
        _: Option::<ctf_archive_member_f>,
        _: *mut libc::c_void,
    ) -> libc::c_int;
    fn ctf_dump(
        _: *mut ctf_dict_t,
        state: *mut *mut ctf_dump_state_t,
        sect: ctf_sect_names_t,
        _: Option::<ctf_dump_decorate_f>,
        arg: *mut libc::c_void,
    ) -> *mut libc::c_char;
    fn ctf_errwarning_next(
        _: *mut ctf_dict_t,
        _: *mut *mut ctf_next_t,
        is_warning: *mut libc::c_int,
        errp: *mut libc::c_int,
    ) -> *mut libc::c_char;
    fn getopt_long(
        argc: libc::c_int,
        argv: *const *mut libc::c_char,
        shortopts: *const libc::c_char,
        longopts: *const option,
        longind: *mut libc::c_int,
    ) -> libc::c_int;
    static _sch_istable: [libc::c_ushort; 256];
    fn disassembler(
        arc: bfd_architecture,
        big: bool,
        mach: libc::c_ulong,
        abfd: *mut bfd,
    ) -> disassembler_ftype;
    fn disassemble_init_for_target(_: *mut disassemble_info);
    fn disassemble_free_target(_: *mut disassemble_info);
    fn disassembler_usage(_: *mut FILE);
    fn remove_whitespace_and_extra_commas(_: *mut libc::c_char) -> *mut libc::c_char;
    fn init_disassemble_info(
        dinfo: *mut disassemble_info,
        stream: *mut libc::c_void,
        fprintf_func: fprintf_ftype,
    );
    fn expandargv(_: *mut libc::c_int, _: *mut *mut *mut libc::c_char);
    fn lbasename(_: *const libc::c_char) -> *const libc::c_char;
    fn concat(_: *const libc::c_char, _: ...) -> *mut libc::c_char;
    fn xmalloc_set_program_name(_: *const libc::c_char);
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn cplus_demangle_set_style(style: demangling_styles) -> demangling_styles;
    fn cplus_demangle_name_to_style(name: *const libc::c_char) -> demangling_styles;
    fn iterative_hash(_: *const libc::c_void, _: size_t, _: hashval_t) -> hashval_t;
    fn filename_cmp(s1: *const libc::c_char, s2: *const libc::c_char) -> libc::c_int;
    fn read_debugging_info(
        _: *mut bfd,
        _: *mut *mut asymbol,
        _: libc::c_long,
        _: bool,
    ) -> *mut libc::c_void;
    fn print_debugging_info(
        _: *mut FILE,
        _: *mut libc::c_void,
        _: *mut bfd,
        _: *mut *mut asymbol,
        _: Option::<
            unsafe extern "C" fn(
                *mut bfd,
                *const libc::c_char,
                libc::c_int,
            ) -> *mut libc::c_char,
        >,
        _: bool,
    ) -> bool;
    fn mmap(
        __addr: *mut libc::c_void,
        __len: size_t,
        __prot: libc::c_int,
        __flags: libc::c_int,
        __fd: libc::c_int,
        __offset: __off_t,
    ) -> *mut libc::c_void;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type size_t = libc::c_ulong;
pub type va_list = __builtin_va_list;
pub type __uint8_t = libc::c_uchar;
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
pub type __ssize_t = libc::c_long;
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
pub type ssize_t = __ssize_t;
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
pub type uint8_t = __uint8_t;
pub type bfd_int64_t = libc::c_long;
pub type bfd_uint64_t = libc::c_ulong;
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
    pub link: C2RustUnnamed_19,
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
pub union C2RustUnnamed_19 {
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
    pub u: C2RustUnnamed_20,
    pub namidx: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_20 {
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
    pub d_un: C2RustUnnamed_21,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_21 {
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
    pub tc_data: C2RustUnnamed_22,
    pub version: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_22 {
    pub hppa_arg_reloc: libc::c_uint,
    pub mips_extr: *mut libc::c_void,
    pub any: *mut libc::c_void,
}
pub type hashval_t = libc::c_uint;
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
pub type elf_vma = libc::c_ulong;
pub type dwarf_vma = libc::c_ulong;
pub type dwarf_size_type = libc::c_ulong;
pub type dwarf_section_display_enum = libc::c_uint;
pub const max: dwarf_section_display_enum = 44;
pub const separate_debug_str: dwarf_section_display_enum = 43;
pub const debug_sup: dwarf_section_display_enum = 42;
pub const gnu_debugaltlink: dwarf_section_display_enum = 41;
pub const gnu_debuglink: dwarf_section_display_enum = 40;
pub const dwp_tu_index: dwarf_section_display_enum = 39;
pub const dwp_cu_index: dwarf_section_display_enum = 38;
pub const debug_addr: dwarf_section_display_enum = 37;
pub const str_index_dwo: dwarf_section_display_enum = 36;
pub const str_index: dwarf_section_display_enum = 35;
pub const str_dwo: dwarf_section_display_enum = 34;
pub const macinfo_dwo: dwarf_section_display_enum = 33;
pub const macro_dwo: dwarf_section_display_enum = 32;
pub const loc_dwo: dwarf_section_display_enum = 31;
pub const line_dwo: dwarf_section_display_enum = 30;
pub const types_dwo: dwarf_section_display_enum = 29;
pub const abbrev_dwo: dwarf_section_display_enum = 28;
pub const info_dwo: dwarf_section_display_enum = 27;
pub const trace_aranges: dwarf_section_display_enum = 26;
pub const trace_abbrev: dwarf_section_display_enum = 25;
pub const trace_info: dwarf_section_display_enum = 24;
pub const debug_names: dwarf_section_display_enum = 23;
pub const gdb_index: dwarf_section_display_enum = 22;
pub const weaknames: dwarf_section_display_enum = 21;
pub const types: dwarf_section_display_enum = 20;
pub const static_vars: dwarf_section_display_enum = 19;
pub const static_func: dwarf_section_display_enum = 18;
pub const rnglists: dwarf_section_display_enum = 17;
pub const ranges: dwarf_section_display_enum = 16;
pub const gnu_pubtypes: dwarf_section_display_enum = 15;
pub const pubtypes: dwarf_section_display_enum = 14;
pub const loclists: dwarf_section_display_enum = 13;
pub const loc: dwarf_section_display_enum = 12;
pub const line_str: dwarf_section_display_enum = 11;
pub const str: dwarf_section_display_enum = 10;
pub const macro_0: dwarf_section_display_enum = 9;
pub const macinfo: dwarf_section_display_enum = 8;
pub const eh_frame: dwarf_section_display_enum = 7;
pub const gnu_pubnames: dwarf_section_display_enum = 6;
pub const pubnames: dwarf_section_display_enum = 5;
pub const line: dwarf_section_display_enum = 4;
pub const info: dwarf_section_display_enum = 3;
pub const frame: dwarf_section_display_enum = 2;
pub const aranges: dwarf_section_display_enum = 1;
pub const abbrev: dwarf_section_display_enum = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dwarf_section {
    pub uncompressed_name: *const libc::c_char,
    pub compressed_name: *const libc::c_char,
    pub name: *const libc::c_char,
    pub filename: *const libc::c_char,
    pub start: *mut libc::c_uchar,
    pub address: dwarf_vma,
    pub size: dwarf_size_type,
    pub abbrev_sec: dwarf_section_display_enum,
    pub reloc_info: *mut libc::c_void,
    pub num_relocs: libc::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dwarf_section_display {
    pub section: dwarf_section,
    pub display: Option::<
        unsafe extern "C" fn(*mut dwarf_section, *mut libc::c_void) -> libc::c_int,
    >,
    pub enabled: *mut libc::c_int,
    pub relocate: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct separate_info {
    pub handle: *mut libc::c_void,
    pub filename: *const libc::c_char,
    pub next: *mut separate_info,
}
pub const _sch_iscntrl: C2RustUnnamed_24 = 2;
pub type ctf_dict_t = ctf_dict;
pub type ctf_archive_t = ctf_archive_internal;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ctf_sect {
    pub cts_name: *const libc::c_char,
    pub cts_data: *const libc::c_void,
    pub cts_size: size_t,
    pub cts_entsize: size_t,
}
pub type ctf_sect_t = ctf_sect;
pub type ctf_sect_names = libc::c_uint;
pub const CTF_SECT_STR: ctf_sect_names = 6;
pub const CTF_SECT_TYPE: ctf_sect_names = 5;
pub const CTF_SECT_VAR: ctf_sect_names = 4;
pub const CTF_SECT_FUNCIDX: ctf_sect_names = 3;
pub const CTF_SECT_FUNC: ctf_sect_names = 3;
pub const CTF_SECT_OBJTIDX: ctf_sect_names = 2;
pub const CTF_SECT_OBJT: ctf_sect_names = 2;
pub const CTF_SECT_LABEL: ctf_sect_names = 1;
pub const CTF_SECT_HEADER: ctf_sect_names = 0;
pub type ctf_sect_names_t = ctf_sect_names;
pub type C2RustUnnamed_23 = libc::c_uint;
pub const ECTF_NONAME: C2RustUnnamed_23 = 1058;
pub const ECTF_INCOMPLETE: C2RustUnnamed_23 = 1057;
pub const ECTF_NEEDSBFD: C2RustUnnamed_23 = 1056;
pub const ECTF_FLAGS: C2RustUnnamed_23 = 1055;
pub const ECTF_NEXT_WRONGFP: C2RustUnnamed_23 = 1054;
pub const ECTF_NEXT_WRONGFUN: C2RustUnnamed_23 = 1053;
pub const ECTF_NEXT_END: C2RustUnnamed_23 = 1052;
pub const ECTF_NONREPRESENTABLE: C2RustUnnamed_23 = 1051;
pub const ECTF_INTERNAL: C2RustUnnamed_23 = 1050;
pub const ECTF_NOTYET: C2RustUnnamed_23 = 1049;
pub const ECTF_DUMPSECTCHANGED: C2RustUnnamed_23 = 1048;
pub const ECTF_DUMPSECTUNKNOWN: C2RustUnnamed_23 = 1047;
pub const ECTF_SLICEOVERFLOW: C2RustUnnamed_23 = 1046;
pub const ECTF_ARNNAME: C2RustUnnamed_23 = 1045;
pub const ECTF_ARCREATE: C2RustUnnamed_23 = 1044;
pub const ECTF_COMPRESS: C2RustUnnamed_23 = 1043;
pub const ECTF_OVERROLLBACK: C2RustUnnamed_23 = 1042;
pub const ECTF_CONFLICT: C2RustUnnamed_23 = 1041;
pub const ECTF_DUPLICATE: C2RustUnnamed_23 = 1040;
pub const ECTF_FULL: C2RustUnnamed_23 = 1039;
pub const ECTF_DTFULL: C2RustUnnamed_23 = 1038;
pub const ECTF_RDONLY: C2RustUnnamed_23 = 1037;
pub const ECTF_NOMEMBNAM: C2RustUnnamed_23 = 1036;
pub const ECTF_NOENUMNAM: C2RustUnnamed_23 = 1035;
pub const ECTF_NOTSUP: C2RustUnnamed_23 = 1034;
pub const ECTF_NOLABELDATA: C2RustUnnamed_23 = 1033;
pub const ECTF_NOLABEL: C2RustUnnamed_23 = 1032;
pub const ECTF_NOTYPEDAT: C2RustUnnamed_23 = 1031;
pub const ECTF_NOTDATA: C2RustUnnamed_23 = 1030;
pub const ECTF_NOFUNCDAT: C2RustUnnamed_23 = 1029;
pub const ECTF_NOTFUNC: C2RustUnnamed_23 = 1028;
pub const ECTF_SYNTAX: C2RustUnnamed_23 = 1027;
pub const ECTF_NOTYPE: C2RustUnnamed_23 = 1026;
pub const ECTF_NAMELEN: C2RustUnnamed_23 = 1025;
pub const ECTF_NOTREF: C2RustUnnamed_23 = 1024;
pub const ECTF_NOTARRAY: C2RustUnnamed_23 = 1023;
pub const ECTF_NOTINTFP: C2RustUnnamed_23 = 1022;
pub const ECTF_NOTSUE: C2RustUnnamed_23 = 1021;
pub const ECTF_NOTENUM: C2RustUnnamed_23 = 1020;
pub const ECTF_NOTSOU: C2RustUnnamed_23 = 1019;
pub const ECTF_BADID: C2RustUnnamed_23 = 1018;
pub const ECTF_BADNAME: C2RustUnnamed_23 = 1017;
pub const ECTF_STRTAB: C2RustUnnamed_23 = 1016;
pub const ECTF_DECOMPRESS: C2RustUnnamed_23 = 1015;
pub const ECTF_ZALLOC: C2RustUnnamed_23 = 1014;
pub const ECTF_LINKADDEDLATE: C2RustUnnamed_23 = 1013;
pub const ECTF_DMODEL: C2RustUnnamed_23 = 1012;
pub const ECTF_NOPARENT: C2RustUnnamed_23 = 1011;
pub const ECTF_NOSYMTAB: C2RustUnnamed_23 = 1010;
pub const ECTF_NOCTFBUF: C2RustUnnamed_23 = 1009;
pub const ECTF_NOCTFDATA: C2RustUnnamed_23 = 1008;
pub const ECTF_CORRUPT: C2RustUnnamed_23 = 1007;
pub const ECTF_STRBAD: C2RustUnnamed_23 = 1006;
pub const ECTF_SYMBAD: C2RustUnnamed_23 = 1005;
pub const ECTF_SYMTAB: C2RustUnnamed_23 = 1004;
pub const ECTF_BFD_AMBIGUOUS: C2RustUnnamed_23 = 1003;
pub const ECTF_CTFVERS: C2RustUnnamed_23 = 1002;
pub const ECTF_BFDERR: C2RustUnnamed_23 = 1001;
pub const ECTF_FMT: C2RustUnnamed_23 = 1000;
pub type ctf_archive_member_f = unsafe extern "C" fn(
    *mut ctf_dict_t,
    *const libc::c_char,
    *mut libc::c_void,
) -> libc::c_int;
pub type ctf_dump_decorate_f = unsafe extern "C" fn(
    ctf_sect_names_t,
    *mut libc::c_char,
    *mut libc::c_void,
) -> *mut libc::c_char;
pub type ctf_dump_state_t = ctf_dump_state;
pub type ctf_next_t = ctf_next;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
pub type C2RustUnnamed_24 = libc::c_uint;
pub const _sch_isbasic: C2RustUnnamed_24 = 3088;
pub const _sch_iscppsp: C2RustUnnamed_24 = 3072;
pub const _sch_isgraph: C2RustUnnamed_24 = 172;
pub const _sch_isidnum: C2RustUnnamed_24 = 516;
pub const _sch_isalnum: C2RustUnnamed_24 = 140;
pub const _sch_isalpha: C2RustUnnamed_24 = 136;
pub const _sch_isnvsp: C2RustUnnamed_24 = 2048;
pub const _sch_isvsp: C2RustUnnamed_24 = 1024;
pub const _sch_isidst: C2RustUnnamed_24 = 512;
pub const _sch_isxdigit: C2RustUnnamed_24 = 256;
pub const _sch_isupper: C2RustUnnamed_24 = 128;
pub const _sch_isspace: C2RustUnnamed_24 = 64;
pub const _sch_ispunct: C2RustUnnamed_24 = 32;
pub const _sch_isprint: C2RustUnnamed_24 = 16;
pub const _sch_islower: C2RustUnnamed_24 = 8;
pub const _sch_isdigit: C2RustUnnamed_24 = 4;
pub const _sch_isblank: C2RustUnnamed_24 = 1;
pub type fprintf_ftype = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char, ...) -> libc::c_int,
>;
pub type dis_insn_type = libc::c_uint;
pub const dis_dref2: dis_insn_type = 7;
pub const dis_dref: dis_insn_type = 6;
pub const dis_condjsr: dis_insn_type = 5;
pub const dis_jsr: dis_insn_type = 4;
pub const dis_condbranch: dis_insn_type = 3;
pub const dis_branch: dis_insn_type = 2;
pub const dis_nonbranch: dis_insn_type = 1;
pub const dis_noninsn: dis_insn_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct disassemble_info {
    pub fprintf_func: fprintf_ftype,
    pub stream: *mut libc::c_void,
    pub application_data: *mut libc::c_void,
    pub flavour: bfd_flavour,
    pub arch: bfd_architecture,
    pub mach: libc::c_ulong,
    pub endian: bfd_endian,
    pub endian_code: bfd_endian,
    pub section: *mut asection,
    pub symbols: *mut *mut asymbol,
    pub num_symbols: libc::c_int,
    pub symtab: *mut *mut asymbol,
    pub symtab_pos: libc::c_int,
    pub symtab_size: libc::c_int,
    pub flags: libc::c_ulong,
    pub dynrelbuf: *mut *mut arelent,
    pub dynrelcount: libc::c_long,
    pub private_data: *mut libc::c_void,
    pub read_memory_func: Option::<
        unsafe extern "C" fn(
            bfd_vma,
            *mut bfd_byte,
            libc::c_uint,
            *mut disassemble_info,
        ) -> libc::c_int,
    >,
    pub memory_error_func: Option::<
        unsafe extern "C" fn(libc::c_int, bfd_vma, *mut disassemble_info) -> (),
    >,
    pub print_address_func: Option::<
        unsafe extern "C" fn(bfd_vma, *mut disassemble_info) -> (),
    >,
    pub symbol_at_address_func: Option::<
        unsafe extern "C" fn(bfd_vma, *mut disassemble_info) -> *mut asymbol,
    >,
    pub symbol_is_valid: Option::<
        unsafe extern "C" fn(*mut asymbol, *mut disassemble_info) -> bool,
    >,
    pub buffer: *mut bfd_byte,
    pub buffer_vma: bfd_vma,
    pub buffer_length: size_t,
    pub bytes_per_line: libc::c_int,
    pub bytes_per_chunk: libc::c_int,
    pub display_endian: bfd_endian,
    pub octets_per_byte: libc::c_uint,
    pub skip_zeroes: libc::c_uint,
    pub skip_zeroes_at_end: libc::c_uint,
    pub disassembler_needs_relocs: bool,
    pub insn_info_valid: libc::c_char,
    pub branch_delay_insns: libc::c_char,
    pub data_size: libc::c_char,
    pub insn_type: dis_insn_type,
    pub target: bfd_vma,
    pub target2: bfd_vma,
    pub disassembler_options: *const libc::c_char,
    pub stop_vma: bfd_vma,
    pub stop_offset: bfd_vma,
}
pub type disassembler_ftype = Option::<
    unsafe extern "C" fn(bfd_vma, *mut disassemble_info) -> libc::c_int,
>;
pub type demangling_styles = libc::c_int;
pub const rust_demangling: demangling_styles = 131072;
pub const dlang_demangling: demangling_styles = 65536;
pub const gnat_demangling: demangling_styles = 32768;
pub const java_demangling: demangling_styles = 4;
pub const gnu_v3_demangling: demangling_styles = 16384;
pub const auto_demangling: demangling_styles = 256;
pub const unknown_demangling: demangling_styles = 0;
pub const no_demangling: demangling_styles = -1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct objdump_private_option {
    pub name: *const libc::c_char,
    pub selected: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct objdump_private_desc {
    pub help: Option::<unsafe extern "C" fn(*mut FILE) -> ()>,
    pub filter: Option::<unsafe extern "C" fn(*mut bfd) -> libc::c_int>,
    pub dump: Option::<unsafe extern "C" fn(*mut bfd) -> ()>,
    pub options: *mut objdump_private_option,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct only {
    pub name: *const libc::c_char,
    pub seen: bool,
    pub next: *mut only,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct objdump_disasm_info {
    pub abfd: *mut bfd,
    pub require_sec: bool,
    pub disassemble_fn: disassembler_ftype,
    pub reloc: *mut arelent,
    pub symbol: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct jump_info {
    pub next: *mut jump_info,
    pub prev: *mut jump_info,
    pub start: C2RustUnnamed_25,
    pub end: bfd_vma,
    pub level: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_25 {
    pub addresses: *mut bfd_vma,
    pub count: size_t,
    pub max_count: size_t,
}
pub type option_values = libc::c_uint;
pub const OPTION_VISUALIZE_JUMPS: option_values = 167;
pub const OPTION_CTF_PARENT: option_values = 166;
pub const OPTION_CTF: option_values = 165;
pub const OPTION_SOURCE_COMMENT: option_values = 164;
pub const OPTION_INLINES: option_values = 163;
pub const OPTION_NO_RECURSE_LIMIT: option_values = 162;
pub const OPTION_RECURSE_LIMIT: option_values = 161;
pub const OPTION_DWARF_START: option_values = 160;
pub const OPTION_DWARF_CHECK: option_values = 159;
pub const OPTION_DWARF_DEPTH: option_values = 158;
pub const OPTION_ADJUST_VMA: option_values = 157;
pub const OPTION_INSN_WIDTH: option_values = 156;
pub const OPTION_PREFIX_STRIP: option_values = 155;
pub const OPTION_PREFIX: option_values = 154;
pub const OPTION_DWARF: option_values = 153;
pub const OPTION_STOP_ADDRESS: option_values = 152;
pub const OPTION_START_ADDRESS: option_values = 151;
pub const OPTION_ENDIAN: option_values = 150;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct print_file_list {
    pub next: *mut print_file_list,
    pub filename: *const libc::c_char,
    pub modname: *const libc::c_char,
    pub map: *const libc::c_char,
    pub mapsize: size_t,
    pub linemap: *mut *const libc::c_char,
    pub maxline: libc::c_uint,
    pub last_line: libc::c_uint,
    pub max_printed: libc::c_uint,
    pub first: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SFILE {
    pub buffer: *mut libc::c_char,
    pub pos: size_t,
    pub alloc: size_t,
}
pub type loop_control = libc::c_uint;
pub const next_sym: loop_control = 2;
pub const function_sym: loop_control = 1;
pub const stop_offset_reached: loop_control = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stab_section_names {
    pub section_name: *const libc::c_char,
    pub string_section_name: *const libc::c_char,
    pub string_offset: libc::c_uint,
}
#[inline]
unsafe extern "C" fn putchar(mut __c: libc::c_int) -> libc::c_int {
    return putc(__c, stdout);
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
    mut str_0: *const libc::c_char,
    mut prefix_0: *const libc::c_char,
) -> bool {
    return strncmp(str_0, prefix_0, strlen(prefix_0)) == 0 as libc::c_int;
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
unsafe extern "C" fn bfd_section_alignment(mut sec: *const asection) -> libc::c_uint {
    return (*sec).alignment_power;
}
#[inline]
unsafe extern "C" fn bfd_is_com_section(mut sec: *const asection) -> bool {
    return (*sec).flags & 0x1000 as libc::c_int as libc::c_uint
        != 0 as libc::c_int as libc::c_uint;
}
#[inline]
unsafe extern "C" fn bfd_is_und_section(mut sec: *const asection) -> bool {
    return sec
        == &mut *_bfd_std_section.as_mut_ptr().offset(1 as libc::c_int as isize)
            as *mut asection as *const asection;
}
#[inline]
unsafe extern "C" fn bfd_is_abs_section(mut sec: *const asection) -> bool {
    return sec
        == &mut *_bfd_std_section.as_mut_ptr().offset(2 as libc::c_int as isize)
            as *mut asection as *const asection;
}
#[inline]
unsafe extern "C" fn bfd_get_filename(mut abfd: *const bfd) -> *const libc::c_char {
    return (*abfd).filename;
}
#[inline]
unsafe extern "C" fn bfd_get_file_flags(mut abfd: *const bfd) -> flagword {
    return (*abfd).flags;
}
#[inline]
unsafe extern "C" fn bfd_asymbol_section(mut sy: *const asymbol) -> *mut asection {
    return (*sy).section;
}
#[inline]
unsafe extern "C" fn bfd_asymbol_value(mut sy: *const asymbol) -> bfd_vma {
    return ((*(*sy).section).vma).wrapping_add((*sy).value);
}
#[inline]
unsafe extern "C" fn bfd_asymbol_name(mut sy: *const asymbol) -> *const libc::c_char {
    return (*sy).name;
}
#[inline]
unsafe extern "C" fn bfd_asymbol_bfd(mut sy: *const asymbol) -> *mut bfd {
    return (*sy).the_bfd;
}
#[inline]
unsafe extern "C" fn bfd_get_flavour(mut abfd: *const bfd) -> bfd_flavour {
    return (*(*abfd).xvec).flavour;
}
#[inline]
unsafe extern "C" fn bfd_big_endian(mut abfd: *const bfd) -> bool {
    return (*(*abfd).xvec).byteorder as libc::c_uint
        == BFD_ENDIAN_BIG as libc::c_int as libc::c_uint;
}
#[inline]
unsafe extern "C" fn bfd_little_endian(mut abfd: *const bfd) -> bool {
    return (*(*abfd).xvec).byteorder as libc::c_uint
        == BFD_ENDIAN_LITTLE as libc::c_int as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn load_debug_section(
    mut debug: dwarf_section_display_enum,
    mut file: *mut libc::c_void,
) -> bool {
    let mut section: *mut dwarf_section = &mut (*debug_displays
        .as_mut_ptr()
        .offset(debug as isize))
        .section;
    let mut abfd: *mut bfd = file as *mut bfd;
    let mut sec: *mut asection = 0 as *mut asection;
    if !((*section).start).is_null() {
        if strcmp((*section).filename, bfd_get_filename(abfd)) == 0 as libc::c_int {
            return 1 as libc::c_int != 0;
        }
    }
    sec = bfd_get_section_by_name(abfd, (*section).uncompressed_name);
    if !sec.is_null() {
        (*section).name = (*section).uncompressed_name;
    } else {
        sec = bfd_get_section_by_name(abfd, (*section).compressed_name);
        if !sec.is_null() {
            (*section).name = (*section).compressed_name;
        }
    }
    if sec.is_null() {
        return 0 as libc::c_int != 0;
    }
    return load_specific_debug_section(debug, sec, file);
}
#[no_mangle]
pub unsafe extern "C" fn free_debug_section(mut debug: dwarf_section_display_enum) {
    let mut section: *mut dwarf_section = &mut (*debug_displays
        .as_mut_ptr()
        .offset(debug as isize))
        .section;
    free((*section).start as *mut libc::c_char as *mut libc::c_void);
    (*section).start = 0 as *mut libc::c_uchar;
    (*section).address = 0 as libc::c_int as dwarf_vma;
    (*section).size = 0 as libc::c_int as dwarf_size_type;
}
#[no_mangle]
pub unsafe extern "C" fn close_debug_file(mut file: *mut libc::c_void) {
    let mut abfd: *mut bfd = file as *mut bfd;
    bfd_close(abfd);
}
#[no_mangle]
pub unsafe extern "C" fn open_debug_file(
    mut pathname: *const libc::c_char,
) -> *mut libc::c_void {
    let mut data: *mut bfd = 0 as *mut bfd;
    data = bfd_openr(pathname, 0 as *const libc::c_char);
    if data.is_null() {
        return 0 as *mut libc::c_void;
    }
    if !bfd_check_format(data, bfd_object) {
        return 0 as *mut libc::c_void;
    }
    return data as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn reloc_at(
    mut dsec: *mut dwarf_section,
    mut offset: dwarf_vma,
) -> bool {
    let mut relocs: *mut *mut arelent = 0 as *mut *mut arelent;
    let mut rp: *mut arelent = 0 as *mut arelent;
    if dsec.is_null() || ((*dsec).reloc_info).is_null() {
        return 0 as libc::c_int != 0;
    }
    relocs = (*dsec).reloc_info as *mut *mut arelent;
    loop {
        rp = *relocs;
        if rp.is_null() {
            break;
        }
        if (*rp).address == offset {
            return 1 as libc::c_int != 0;
        }
        relocs = relocs.offset(1);
        relocs;
    }
    return 0 as libc::c_int != 0;
}
static mut exit_status: libc::c_int = 0 as libc::c_int;
static mut default_target: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
static mut show_version: libc::c_int = 0 as libc::c_int;
static mut dump_section_contents: libc::c_int = 0;
static mut dump_section_headers: libc::c_int = 0;
static mut dump_file_header: bool = false;
static mut dump_symtab: libc::c_int = 0;
static mut dump_dynamic_symtab: libc::c_int = 0;
static mut dump_reloc_info: libc::c_int = 0;
static mut dump_dynamic_reloc_info: libc::c_int = 0;
static mut dump_ar_hdrs: libc::c_int = 0;
static mut dump_private_headers: libc::c_int = 0;
static mut dump_private_options: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
static mut no_addresses: libc::c_int = 0;
static mut prefix_addresses: libc::c_int = 0;
static mut with_line_numbers: libc::c_int = 0;
static mut with_source_code: bool = false;
static mut show_raw_insn: libc::c_int = 0;
static mut dump_dwarf_section_info: libc::c_int = 0;
static mut dump_stab_section_info: libc::c_int = 0;
static mut dump_ctf_section_info: libc::c_int = 0;
static mut dump_ctf_section_name: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
static mut dump_ctf_parent_name: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
static mut do_demangle: libc::c_int = 0;
static mut disassemble: bool = false;
static mut disassemble_all: bool = false;
static mut disassemble_zeroes: libc::c_int = 0;
static mut formats_info: bool = false;
static mut wide_output: libc::c_int = 0;
static mut insn_width: libc::c_int = 0;
static mut start_address: bfd_vma = -(1 as libc::c_int) as bfd_vma;
static mut stop_address: bfd_vma = -(1 as libc::c_int) as bfd_vma;
static mut dump_debugging: libc::c_int = 0;
static mut dump_debugging_tags: libc::c_int = 0;
static mut suppress_bfd_header: libc::c_int = 0;
static mut dump_special_syms: libc::c_int = 0 as libc::c_int;
static mut adjust_section_vma: bfd_vma = 0 as libc::c_int as bfd_vma;
static mut file_start_context: libc::c_int = 0 as libc::c_int;
static mut display_file_offsets: bool = false;
static mut prefix: *const libc::c_char = 0 as *const libc::c_char;
static mut prefix_strip: libc::c_int = 0;
static mut prefix_length: size_t = 0;
static mut unwind_inlines: bool = false;
static mut disasm_sym: *const libc::c_char = 0 as *const libc::c_char;
static mut source_comment: *const libc::c_char = 0 as *const libc::c_char;
static mut visualize_jumps: bool = 0 as libc::c_int != 0;
static mut color_output: bool = 0 as libc::c_int != 0;
static mut extended_color_output: bool = 0 as libc::c_int != 0;
static mut process_links: libc::c_int = 0 as libc::c_int;
static mut demangle_flags: libc::c_int = (1 as libc::c_int) << 1 as libc::c_int
    | (1 as libc::c_int) << 0 as libc::c_int;
static mut only_list: *mut only = 0 as *const only as *mut only;
static mut include_paths: *mut *const libc::c_char = 0 as *const *const libc::c_char
    as *mut *const libc::c_char;
static mut include_path_count: libc::c_int = 0;
static mut machine: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut disassembler_options: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
static mut endian: bfd_endian = BFD_ENDIAN_UNKNOWN;
static mut syms: *mut *mut asymbol = 0 as *const *mut asymbol as *mut *mut asymbol;
static mut symcount: libc::c_long = 0 as libc::c_int as libc::c_long;
static mut sorted_syms: *mut *mut asymbol = 0 as *const *mut asymbol
    as *mut *mut asymbol;
static mut sorted_symcount: libc::c_long = 0 as libc::c_int as libc::c_long;
static mut dynsyms: *mut *mut asymbol = 0 as *const *mut asymbol as *mut *mut asymbol;
static mut synthsyms: *mut asymbol = 0 as *const asymbol as *mut asymbol;
static mut synthcount: libc::c_long = 0 as libc::c_int as libc::c_long;
static mut dynsymcount: libc::c_long = 0 as libc::c_int as libc::c_long;
static mut stabs: *mut bfd_byte = 0 as *const bfd_byte as *mut bfd_byte;
static mut stab_size: bfd_size_type = 0;
static mut strtab: *mut bfd_byte = 0 as *const bfd_byte as *mut bfd_byte;
static mut stabstr_size: bfd_size_type = 0;
static mut objdump_private_vectors: [*const objdump_private_desc; 1] = [
    0 as *const objdump_private_desc,
];
static mut detected_jumps: *mut jump_info = 0 as *const jump_info as *mut jump_info;
unsafe extern "C" fn usage(mut stream: *mut FILE, mut status: libc::c_int) -> ! {
    fprintf(
        stream,
        dcgettext(
            0 as *const libc::c_char,
            b"Usage: %s <option(s)> <file(s)>\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        program_name,
    );
    fprintf(
        stream,
        dcgettext(
            0 as *const libc::c_char,
            b" Display information from object <file(s)>.\n\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    fprintf(
        stream,
        dcgettext(
            0 as *const libc::c_char,
            b" At least one of the following switches must be given:\n\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    fprintf(
        stream,
        dcgettext(
            0 as *const libc::c_char,
            b"  -a, --archive-headers    Display archive header information\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    fprintf(
        stream,
        dcgettext(
            0 as *const libc::c_char,
            b"  -f, --file-headers       Display the contents of the overall file header\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    fprintf(
        stream,
        dcgettext(
            0 as *const libc::c_char,
            b"  -p, --private-headers    Display object format specific file header contents\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    fprintf(
        stream,
        dcgettext(
            0 as *const libc::c_char,
            b"  -P, --private=OPT,OPT... Display object format specific contents\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    fprintf(
        stream,
        dcgettext(
            0 as *const libc::c_char,
            b"  -h, --[section-]headers  Display the contents of the section headers\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    fprintf(
        stream,
        dcgettext(
            0 as *const libc::c_char,
            b"  -x, --all-headers        Display the contents of all headers\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    fprintf(
        stream,
        dcgettext(
            0 as *const libc::c_char,
            b"  -d, --disassemble        Display assembler contents of executable sections\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    fprintf(
        stream,
        dcgettext(
            0 as *const libc::c_char,
            b"  -D, --disassemble-all    Display assembler contents of all sections\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    fprintf(
        stream,
        dcgettext(
            0 as *const libc::c_char,
            b"      --disassemble=<sym>  Display assembler contents from <sym>\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    fprintf(
        stream,
        dcgettext(
            0 as *const libc::c_char,
            b"  -S, --source             Intermix source code with disassembly\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    fprintf(
        stream,
        dcgettext(
            0 as *const libc::c_char,
            b"      --source-comment[=<txt>] Prefix lines of source code with <txt>\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    fprintf(
        stream,
        dcgettext(
            0 as *const libc::c_char,
            b"  -s, --full-contents      Display the full contents of all sections requested\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    fprintf(
        stream,
        dcgettext(
            0 as *const libc::c_char,
            b"  -g, --debugging          Display debug information in object file\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    fprintf(
        stream,
        dcgettext(
            0 as *const libc::c_char,
            b"  -e, --debugging-tags     Display debug information using ctags style\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    fprintf(
        stream,
        dcgettext(
            0 as *const libc::c_char,
            b"  -G, --stabs              Display (in raw form) any STABS info in the file\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    fprintf(
        stream,
        dcgettext(
            0 as *const libc::c_char,
            b"  -W, --dwarf[a/=abbrev, A/=addr, r/=aranges, c/=cu_index, L/=decodedline,\n              f/=frames, F/=frames-interp, g/=gdb_index, i/=info, o/=loc,\n              m/=macro, p/=pubnames, t/=pubtypes, R/=Ranges, l/=rawline,\n              s/=str, O/=str-offsets, u/=trace_abbrev, T/=trace_aranges,\n              U/=trace_info]\n                           Display the contents of DWARF debug sections\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    fprintf(
        stream,
        dcgettext(
            0 as *const libc::c_char,
            b"  -Wk,--dwarf=links        Display the contents of sections that link to\n                            separate debuginfo files\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    fprintf(
        stream,
        dcgettext(
            0 as *const libc::c_char,
            b"  -WK,--dwarf=follow-links\n                           Follow links to separate debug info files (default)\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    fprintf(
        stream,
        dcgettext(
            0 as *const libc::c_char,
            b"  -WN,--dwarf=no-follow-links\n                           Do not follow links to separate debug info files\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    fprintf(
        stream,
        dcgettext(
            0 as *const libc::c_char,
            b"  -L, --process-links      Display the contents of non-debug sections in\n                            separate debuginfo files.  (Implies -WK)\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    fprintf(
        stream,
        dcgettext(
            0 as *const libc::c_char,
            b"      --ctf=SECTION        Display CTF info from SECTION\n\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    fprintf(
        stream,
        dcgettext(
            0 as *const libc::c_char,
            b"  -t, --syms               Display the contents of the symbol table(s)\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    fprintf(
        stream,
        dcgettext(
            0 as *const libc::c_char,
            b"  -T, --dynamic-syms       Display the contents of the dynamic symbol table\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    fprintf(
        stream,
        dcgettext(
            0 as *const libc::c_char,
            b"  -r, --reloc              Display the relocation entries in the file\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    fprintf(
        stream,
        dcgettext(
            0 as *const libc::c_char,
            b"  -R, --dynamic-reloc      Display the dynamic relocation entries in the file\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    fprintf(
        stream,
        dcgettext(
            0 as *const libc::c_char,
            b"  @<file>                  Read options from <file>\n\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    fprintf(
        stream,
        dcgettext(
            0 as *const libc::c_char,
            b"  -v, --version            Display this program's version number\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    fprintf(
        stream,
        dcgettext(
            0 as *const libc::c_char,
            b"  -i, --info               List object formats and architectures supported\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    fprintf(
        stream,
        dcgettext(
            0 as *const libc::c_char,
            b"  -H, --help               Display this information\n\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    if status != 2 as libc::c_int {
        let mut desc: *const *const objdump_private_desc = 0
            as *const *const objdump_private_desc;
        fprintf(
            stream,
            dcgettext(
                0 as *const libc::c_char,
                b"\n The following switches are optional:\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        fprintf(
            stream,
            dcgettext(
                0 as *const libc::c_char,
                b"  -b, --target=BFDNAME           Specify the target object format as BFDNAME\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        fprintf(
            stream,
            dcgettext(
                0 as *const libc::c_char,
                b"  -m, --architecture=MACHINE     Specify the target architecture as MACHINE\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        fprintf(
            stream,
            dcgettext(
                0 as *const libc::c_char,
                b"  -j, --section=NAME             Only display information for section NAME\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        fprintf(
            stream,
            dcgettext(
                0 as *const libc::c_char,
                b"  -M, --disassembler-options=OPT Pass text OPT on to the disassembler\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        fprintf(
            stream,
            dcgettext(
                0 as *const libc::c_char,
                b"  -EB --endian=big               Assume big endian format when disassembling\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        fprintf(
            stream,
            dcgettext(
                0 as *const libc::c_char,
                b"  -EL --endian=little            Assume little endian format when disassembling\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        fprintf(
            stream,
            dcgettext(
                0 as *const libc::c_char,
                b"      --file-start-context       Include context from start of file (with -S)\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        fprintf(
            stream,
            dcgettext(
                0 as *const libc::c_char,
                b"  -I, --include=DIR              Add DIR to search list for source files\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        fprintf(
            stream,
            dcgettext(
                0 as *const libc::c_char,
                b"  -l, --line-numbers             Include line numbers and filenames in output\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        fprintf(
            stream,
            dcgettext(
                0 as *const libc::c_char,
                b"  -F, --file-offsets             Include file offsets when displaying information\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        fprintf(
            stream,
            dcgettext(
                0 as *const libc::c_char,
                b"  -C, --demangle[=STYLE]         Decode mangled/processed symbol names\n                                  The STYLE, if specified, can be `auto', `gnu',\n                                  `lucid', `arm', `hp', `edg', `gnu-v3', `java'\n                                  or `gnat'\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        fprintf(
            stream,
            dcgettext(
                0 as *const libc::c_char,
                b"      --recurse-limit            Enable a limit on recursion whilst demangling\n                                  (default)\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        fprintf(
            stream,
            dcgettext(
                0 as *const libc::c_char,
                b"      --no-recurse-limit         Disable a limit on recursion whilst demangling\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        fprintf(
            stream,
            dcgettext(
                0 as *const libc::c_char,
                b"  -w, --wide                     Format output for more than 80 columns\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        fprintf(
            stream,
            dcgettext(
                0 as *const libc::c_char,
                b"  -z, --disassemble-zeroes       Do not skip blocks of zeroes when disassembling\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        fprintf(
            stream,
            dcgettext(
                0 as *const libc::c_char,
                b"      --start-address=ADDR       Only process data whose address is >= ADDR\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        fprintf(
            stream,
            dcgettext(
                0 as *const libc::c_char,
                b"      --stop-address=ADDR        Only process data whose address is < ADDR\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        fprintf(
            stream,
            dcgettext(
                0 as *const libc::c_char,
                b"      --no-addresses             Do not print address alongside disassembly\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        fprintf(
            stream,
            dcgettext(
                0 as *const libc::c_char,
                b"      --prefix-addresses         Print complete address alongside disassembly\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        fprintf(
            stream,
            dcgettext(
                0 as *const libc::c_char,
                b"      --[no-]show-raw-insn       Display hex alongside symbolic disassembly\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        fprintf(
            stream,
            dcgettext(
                0 as *const libc::c_char,
                b"      --insn-width=WIDTH         Display WIDTH bytes on a single line for -d\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        fprintf(
            stream,
            dcgettext(
                0 as *const libc::c_char,
                b"      --adjust-vma=OFFSET        Add OFFSET to all displayed section addresses\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        fprintf(
            stream,
            dcgettext(
                0 as *const libc::c_char,
                b"      --special-syms             Include special symbols in symbol dumps\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        fprintf(
            stream,
            dcgettext(
                0 as *const libc::c_char,
                b"      --inlines                  Print all inlines for source line (with -l)\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        fprintf(
            stream,
            dcgettext(
                0 as *const libc::c_char,
                b"      --prefix=PREFIX            Add PREFIX to absolute paths for -S\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        fprintf(
            stream,
            dcgettext(
                0 as *const libc::c_char,
                b"      --prefix-strip=LEVEL       Strip initial directory names for -S\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        fprintf(
            stream,
            dcgettext(
                0 as *const libc::c_char,
                b"      --dwarf-depth=N            Do not display DIEs at depth N or greater\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        fprintf(
            stream,
            dcgettext(
                0 as *const libc::c_char,
                b"      --dwarf-start=N            Display DIEs starting at offset N\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        fprintf(
            stream,
            dcgettext(
                0 as *const libc::c_char,
                b"      --dwarf-check              Make additional dwarf consistency checks.\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        fprintf(
            stream,
            dcgettext(
                0 as *const libc::c_char,
                b"      --ctf-parent=SECTION       Use SECTION as the CTF parent\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        fprintf(
            stream,
            dcgettext(
                0 as *const libc::c_char,
                b"      --visualize-jumps          Visualize jumps by drawing ASCII art lines\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        fprintf(
            stream,
            dcgettext(
                0 as *const libc::c_char,
                b"      --visualize-jumps=color    Use colors in the ASCII art\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        fprintf(
            stream,
            dcgettext(
                0 as *const libc::c_char,
                b"      --visualize-jumps=extended-color\n                                 Use extended 8-bit color codes\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        fprintf(
            stream,
            dcgettext(
                0 as *const libc::c_char,
                b"      --visualize-jumps=off      Disable jump visualization\n\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        list_supported_targets(program_name, stream);
        list_supported_architectures(program_name, stream);
        disassembler_usage(stream);
        if !(objdump_private_vectors[0 as libc::c_int as usize]).is_null() {
            fprintf(
                stream,
                dcgettext(
                    0 as *const libc::c_char,
                    b"\nOptions supported for -P/--private switch:\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            desc = objdump_private_vectors.as_ptr();
            while !(*desc).is_null() {
                ((**desc).help).expect("non-null function pointer")(stream);
                desc = desc.offset(1);
                desc;
            }
        }
    }
    if (*::core::mem::transmute::<
        &[u8; 39],
        &[libc::c_char; 39],
    >(b"<https://www.sourceware.org/bugzilla/>\0"))[0 as libc::c_int as usize]
        as libc::c_int != 0 && status == 0 as libc::c_int
    {
        fprintf(
            stream,
            dcgettext(
                0 as *const libc::c_char,
                b"Report bugs to %s.\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            b"<https://www.sourceware.org/bugzilla/>\0" as *const u8
                as *const libc::c_char,
        );
    }
    exit(status);
}
static mut long_options: [option; 59] = unsafe {
    [
        {
            let mut init = option {
                name: b"adjust-vma\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: OPTION_ADJUST_VMA as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"all-headers\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'x' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"private-headers\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'p' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"private\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'P' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"architecture\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'm' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"archive-headers\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'a' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"debugging\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'g' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"debugging-tags\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'e' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"demangle\0" as *const u8 as *const libc::c_char,
                has_arg: 2 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'C' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"disassemble\0" as *const u8 as *const libc::c_char,
                has_arg: 2 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'd' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"disassemble-all\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'D' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"disassembler-options\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'M' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"disassemble-zeroes\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'z' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"dynamic-reloc\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'R' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"dynamic-syms\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'T' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"endian\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: OPTION_ENDIAN as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"file-headers\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'f' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"file-offsets\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'F' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"file-start-context\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: &file_start_context as *const libc::c_int as *mut libc::c_int,
                val: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"full-contents\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 's' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"headers\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'h' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"help\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'H' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"info\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'i' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"line-numbers\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'l' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"no-show-raw-insn\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: &show_raw_insn as *const libc::c_int as *mut libc::c_int,
                val: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = option {
                name: b"no-addresses\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: &no_addresses as *const libc::c_int as *mut libc::c_int,
                val: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"process-links\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: &process_links as *const libc::c_int as *mut libc::c_int,
                val: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"prefix-addresses\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: &prefix_addresses as *const libc::c_int as *mut libc::c_int,
                val: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"recurse-limit\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: OPTION_RECURSE_LIMIT as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"recursion-limit\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: OPTION_RECURSE_LIMIT as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"no-recurse-limit\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: OPTION_NO_RECURSE_LIMIT as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"no-recursion-limit\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: OPTION_NO_RECURSE_LIMIT as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"reloc\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'r' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"section\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'j' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"section-headers\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'h' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"show-raw-insn\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: &show_raw_insn as *const libc::c_int as *mut libc::c_int,
                val: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"source\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'S' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"source-comment\0" as *const u8 as *const libc::c_char,
                has_arg: 2 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: OPTION_SOURCE_COMMENT as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"special-syms\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: &dump_special_syms as *const libc::c_int as *mut libc::c_int,
                val: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"include\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'I' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"dwarf\0" as *const u8 as *const libc::c_char,
                has_arg: 2 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: OPTION_DWARF as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"ctf\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: OPTION_CTF as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"ctf-parent\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: OPTION_CTF_PARENT as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"stabs\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'G' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"start-address\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: OPTION_START_ADDRESS as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"stop-address\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: OPTION_STOP_ADDRESS as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"syms\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 't' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"target\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'b' as i32,
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
                name: b"wide\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'w' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"prefix\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: OPTION_PREFIX as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"prefix-strip\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: OPTION_PREFIX_STRIP as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"insn-width\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: OPTION_INSN_WIDTH as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"dwarf-depth\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: OPTION_DWARF_DEPTH as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"dwarf-start\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: OPTION_DWARF_START as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"dwarf-check\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: OPTION_DWARF_CHECK as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"inlines\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: OPTION_INLINES as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"visualize-jumps\0" as *const u8 as *const libc::c_char,
                has_arg: 2 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: OPTION_VISUALIZE_JUMPS as libc::c_int,
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
    ]
};
unsafe extern "C" fn nonfatal(mut msg: *const libc::c_char) {
    bfd_nonfatal(msg);
    exit_status = 1 as libc::c_int;
}
unsafe extern "C" fn sanitize_string(
    mut in_0: *const libc::c_char,
) -> *const libc::c_char {
    static mut buffer: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
    static mut buffer_len: size_t = 0 as libc::c_int as size_t;
    let mut original: *const libc::c_char = in_0;
    let mut out: *mut libc::c_char = 0 as *mut libc::c_char;
    if in_0.is_null() {
        return b"\0" as *const u8 as *const libc::c_char;
    }
    loop {
        let fresh0 = in_0;
        in_0 = in_0.offset(1);
        let mut c: libc::c_char = *fresh0;
        if c as libc::c_int == 0 as libc::c_int {
            return original;
        }
        if _sch_istable[(c as libc::c_int & 0xff as libc::c_int) as usize] as libc::c_int
            & _sch_iscntrl as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            break;
        }
    }
    in_0 = original;
    if buffer_len < (strlen(in_0)).wrapping_mul(2 as libc::c_int as libc::c_ulong) {
        free(buffer as *mut libc::c_void);
        buffer_len = (strlen(in_0)).wrapping_mul(2 as libc::c_int as libc::c_ulong);
        buffer = xmalloc(buffer_len.wrapping_add(1 as libc::c_int as libc::c_ulong))
            as *mut libc::c_char;
    }
    out = buffer;
    loop {
        let fresh1 = in_0;
        in_0 = in_0.offset(1);
        let mut c_0: libc::c_char = *fresh1;
        if c_0 as libc::c_int == 0 as libc::c_int {
            break;
        }
        if _sch_istable[(c_0 as libc::c_int & 0xff as libc::c_int) as usize]
            as libc::c_int & _sch_iscntrl as libc::c_int as libc::c_ushort as libc::c_int
            == 0
        {
            let fresh2 = out;
            out = out.offset(1);
            *fresh2 = c_0;
        } else {
            let fresh3 = out;
            out = out.offset(1);
            *fresh3 = '^' as i32 as libc::c_char;
            let fresh4 = out;
            out = out.offset(1);
            *fresh4 = (c_0 as libc::c_int + 0x40 as libc::c_int) as libc::c_char;
        }
    }
    *out = 0 as libc::c_int as libc::c_char;
    return buffer;
}
unsafe extern "C" fn process_section_p(mut section: *mut asection) -> bool {
    let mut only: *mut only = 0 as *mut only;
    if only_list.is_null() {
        return 1 as libc::c_int != 0;
    }
    only = only_list;
    while !only.is_null() {
        if strcmp((*only).name, (*section).name) == 0 as libc::c_int {
            (*only).seen = 1 as libc::c_int != 0;
            return 1 as libc::c_int != 0;
        }
        only = (*only).next;
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn add_only(mut name: *mut libc::c_char) {
    let mut only: *mut only = 0 as *mut only;
    only = only_list;
    while !only.is_null() {
        if strcmp((*only).name, name) == 0 as libc::c_int {
            return;
        }
        only = (*only).next;
    }
    only = xmalloc(::core::mem::size_of::<only>() as libc::c_ulong) as *mut only;
    (*only).name = name;
    (*only).seen = 0 as libc::c_int != 0;
    (*only).next = only_list;
    only_list = only;
}
unsafe extern "C" fn free_only_list() {
    let mut at_least_one_seen: bool = 0 as libc::c_int != 0;
    let mut only: *mut only = 0 as *mut only;
    let mut next: *mut only = 0 as *mut only;
    if only_list.is_null() {
        return;
    }
    only = only_list;
    while !only.is_null() {
        if (*only).seen {
            at_least_one_seen = 1 as libc::c_int != 0;
            break;
        } else {
            only = (*only).next;
        }
    }
    only = only_list;
    while !only.is_null() {
        if !at_least_one_seen {
            non_fatal(
                dcgettext(
                    0 as *const libc::c_char,
                    b"section '%s' mentioned in a -j option, but not found in any input file\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*only).name,
            );
            exit_status = 1 as libc::c_int;
        }
        next = (*only).next;
        free(only as *mut libc::c_void);
        only = next;
    }
}
unsafe extern "C" fn dump_section_header(
    mut abfd: *mut bfd,
    mut section: *mut asection,
    mut data: *mut libc::c_void,
) {
    let mut comma: *mut libc::c_char = b"\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    let mut opb: libc::c_uint = bfd_octets_per_byte(abfd, section);
    let mut longest_section_name: libc::c_int = *(data as *mut libc::c_int);
    if (*section).flags & 0x100000 as libc::c_int as libc::c_uint != 0 {
        return;
    }
    if !process_section_p(section) {
        return;
    }
    printf(
        b"%3d %-*s %08lx  \0" as *const u8 as *const libc::c_char,
        (*section).index,
        longest_section_name,
        sanitize_string(bfd_section_name(section)),
        (bfd_section_size(section)).wrapping_div(opb as libc::c_ulong),
    );
    bfd_fprintf_vma(abfd, stdout as *mut libc::c_void, bfd_section_vma(section));
    printf(b"  \0" as *const u8 as *const libc::c_char);
    bfd_fprintf_vma(abfd, stdout as *mut libc::c_void, (*section).lma);
    printf(
        b"  %08lx  2**%u\0" as *const u8 as *const libc::c_char,
        (*section).filepos as libc::c_ulong,
        bfd_section_alignment(section),
    );
    if wide_output == 0 {
        printf(b"\n                \0" as *const u8 as *const libc::c_char);
    }
    printf(b"  \0" as *const u8 as *const libc::c_char);
    if (*section).flags & 0x100 as libc::c_int as libc::c_uint != 0 {
        printf(
            b"%s%s\0" as *const u8 as *const libc::c_char,
            comma,
            b"CONTENTS\0" as *const u8 as *const libc::c_char,
        );
        comma = b", \0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if (*section).flags & 0x1 as libc::c_int as libc::c_uint != 0 {
        printf(
            b"%s%s\0" as *const u8 as *const libc::c_char,
            comma,
            b"ALLOC\0" as *const u8 as *const libc::c_char,
        );
        comma = b", \0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if (*section).flags & 0x80 as libc::c_int as libc::c_uint != 0 {
        printf(
            b"%s%s\0" as *const u8 as *const libc::c_char,
            comma,
            b"CONSTRUCTOR\0" as *const u8 as *const libc::c_char,
        );
        comma = b", \0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if (*section).flags & 0x2 as libc::c_int as libc::c_uint != 0 {
        printf(
            b"%s%s\0" as *const u8 as *const libc::c_char,
            comma,
            b"LOAD\0" as *const u8 as *const libc::c_char,
        );
        comma = b", \0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if (*section).flags & 0x4 as libc::c_int as libc::c_uint != 0 {
        printf(
            b"%s%s\0" as *const u8 as *const libc::c_char,
            comma,
            b"RELOC\0" as *const u8 as *const libc::c_char,
        );
        comma = b", \0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if (*section).flags & 0x8 as libc::c_int as libc::c_uint != 0 {
        printf(
            b"%s%s\0" as *const u8 as *const libc::c_char,
            comma,
            b"READONLY\0" as *const u8 as *const libc::c_char,
        );
        comma = b", \0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if (*section).flags & 0x10 as libc::c_int as libc::c_uint != 0 {
        printf(
            b"%s%s\0" as *const u8 as *const libc::c_char,
            comma,
            b"CODE\0" as *const u8 as *const libc::c_char,
        );
        comma = b", \0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if (*section).flags & 0x20 as libc::c_int as libc::c_uint != 0 {
        printf(
            b"%s%s\0" as *const u8 as *const libc::c_char,
            comma,
            b"DATA\0" as *const u8 as *const libc::c_char,
        );
        comma = b", \0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if (*section).flags & 0x40 as libc::c_int as libc::c_uint != 0 {
        printf(
            b"%s%s\0" as *const u8 as *const libc::c_char,
            comma,
            b"ROM\0" as *const u8 as *const libc::c_char,
        );
        comma = b", \0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if (*section).flags & 0x2000 as libc::c_int as libc::c_uint != 0 {
        printf(
            b"%s%s\0" as *const u8 as *const libc::c_char,
            comma,
            b"DEBUGGING\0" as *const u8 as *const libc::c_char,
        );
        comma = b", \0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if (*section).flags & 0x200 as libc::c_int as libc::c_uint != 0 {
        printf(
            b"%s%s\0" as *const u8 as *const libc::c_char,
            comma,
            b"NEVER_LOAD\0" as *const u8 as *const libc::c_char,
        );
        comma = b", \0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if (*section).flags & 0x8000 as libc::c_int as libc::c_uint != 0 {
        printf(
            b"%s%s\0" as *const u8 as *const libc::c_char,
            comma,
            b"EXCLUDE\0" as *const u8 as *const libc::c_char,
        );
        comma = b", \0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if (*section).flags & 0x10000 as libc::c_int as libc::c_uint != 0 {
        printf(
            b"%s%s\0" as *const u8 as *const libc::c_char,
            comma,
            b"SORT_ENTRIES\0" as *const u8 as *const libc::c_char,
        );
        comma = b", \0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if bfd_get_arch(abfd) as libc::c_uint
        == bfd_arch_tic54x as libc::c_int as libc::c_uint
    {
        if (*section).flags & 0x10000000 as libc::c_int as libc::c_uint != 0 {
            printf(
                b"%s%s\0" as *const u8 as *const libc::c_char,
                comma,
                b"BLOCK\0" as *const u8 as *const libc::c_char,
            );
            comma = b", \0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        if (*section).flags & 0x20000000 as libc::c_int as libc::c_uint != 0 {
            printf(
                b"%s%s\0" as *const u8 as *const libc::c_char,
                comma,
                b"CLINK\0" as *const u8 as *const libc::c_char,
            );
            comma = b", \0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
    }
    if (*section).flags & 0x400000 as libc::c_int as libc::c_uint != 0 {
        printf(
            b"%s%s\0" as *const u8 as *const libc::c_char,
            comma,
            b"SMALL_DATA\0" as *const u8 as *const libc::c_char,
        );
        comma = b", \0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if bfd_get_flavour(abfd) as libc::c_uint
        == bfd_target_coff_flavour as libc::c_int as libc::c_uint
    {
        if (*section).flags & 0x8000000 as libc::c_int as libc::c_uint != 0 {
            printf(
                b"%s%s\0" as *const u8 as *const libc::c_char,
                comma,
                b"SHARED\0" as *const u8 as *const libc::c_char,
            );
            comma = b", \0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        if (*section).flags & 0x40000000 as libc::c_int as libc::c_uint != 0 {
            printf(
                b"%s%s\0" as *const u8 as *const libc::c_char,
                comma,
                b"NOREAD\0" as *const u8 as *const libc::c_char,
            );
            comma = b", \0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
    } else if bfd_get_flavour(abfd) as libc::c_uint
        == bfd_target_elf_flavour as libc::c_int as libc::c_uint
    {
        if (*section).flags & 0x40000000 as libc::c_int as libc::c_uint != 0 {
            printf(
                b"%s%s\0" as *const u8 as *const libc::c_char,
                comma,
                b"OCTETS\0" as *const u8 as *const libc::c_char,
            );
            comma = b", \0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        if (*section).flags & 0x80000000 as libc::c_uint != 0 {
            printf(
                b"%s%s\0" as *const u8 as *const libc::c_char,
                comma,
                b"PURECODE\0" as *const u8 as *const libc::c_char,
            );
            comma = b", \0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
    }
    if (*section).flags & 0x400 as libc::c_int as libc::c_uint != 0 {
        printf(
            b"%s%s\0" as *const u8 as *const libc::c_char,
            comma,
            b"THREAD_LOCAL\0" as *const u8 as *const libc::c_char,
        );
        comma = b", \0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if (*section).flags & 0x2000000 as libc::c_int as libc::c_uint != 0 {
        printf(
            b"%s%s\0" as *const u8 as *const libc::c_char,
            comma,
            b"GROUP\0" as *const u8 as *const libc::c_char,
        );
        comma = b", \0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if bfd_get_arch(abfd) as libc::c_uint == bfd_arch_mep as libc::c_int as libc::c_uint
    {
        if (*section).flags & 0x20000000 as libc::c_int as libc::c_uint != 0 {
            printf(
                b"%s%s\0" as *const u8 as *const libc::c_char,
                comma,
                b"VLIW\0" as *const u8 as *const libc::c_char,
            );
            comma = b", \0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
    }
    if (*section).flags & 0x20000 as libc::c_int as libc::c_uint
        != 0 as libc::c_int as libc::c_uint
    {
        let mut ls: *const libc::c_char = 0 as *const libc::c_char;
        let mut comdat: *mut coff_comdat_info = 0 as *mut coff_comdat_info;
        match (*section).flags & 0xc0000 as libc::c_int as libc::c_uint {
            0 => {
                ls = b"LINK_ONCE_DISCARD\0" as *const u8 as *const libc::c_char;
            }
            262144 => {
                ls = b"LINK_ONCE_ONE_ONLY\0" as *const u8 as *const libc::c_char;
            }
            524288 => {
                ls = b"LINK_ONCE_SAME_SIZE\0" as *const u8 as *const libc::c_char;
            }
            786432 => {
                ls = b"LINK_ONCE_SAME_CONTENTS\0" as *const u8 as *const libc::c_char;
            }
            _ => {
                abort();
            }
        }
        printf(b"%s%s\0" as *const u8 as *const libc::c_char, comma, ls);
        comdat = if bfd_get_flavour(abfd) as libc::c_uint
            == bfd_target_coff_flavour as libc::c_int as libc::c_uint
            && !((*section).used_by_bfd as *mut coff_section_tdata).is_null()
        {
            (*((*section).used_by_bfd as *mut coff_section_tdata)).comdat
        } else {
            0 as *mut coff_comdat_info
        };
        if !comdat.is_null() {
            printf(
                b" (COMDAT %s %ld)\0" as *const u8 as *const libc::c_char,
                (*comdat).name,
                (*comdat).symbol,
            );
        }
        comma = b", \0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn find_longest_section_name(
    mut _abfd: *mut bfd,
    mut section: *mut asection,
    mut data: *mut libc::c_void,
) {
    let mut longest_so_far: *mut libc::c_int = data as *mut libc::c_int;
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut len: libc::c_int = 0;
    if (*section).flags & 0x100000 as libc::c_int as libc::c_uint != 0 {
        return;
    }
    if !process_section_p(section) {
        return;
    }
    name = bfd_section_name(section);
    len = strlen(name) as libc::c_int;
    if len > *longest_so_far {
        *longest_so_far = len;
    }
}
unsafe extern "C" fn dump_headers(mut abfd: *mut bfd) {
    let mut max_section_name_length: libc::c_int = 13 as libc::c_int;
    let mut bfd_vma_width: libc::c_int = 0;
    if bfd_get_arch_size(abfd) == 32 as libc::c_int {
        bfd_vma_width = 10 as libc::c_int;
    } else {
        bfd_vma_width = 18 as libc::c_int;
    }
    printf(
        dcgettext(
            0 as *const libc::c_char,
            b"Sections:\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    if wide_output != 0 {
        bfd_map_over_sections(
            abfd,
            Some(
                find_longest_section_name
                    as unsafe extern "C" fn(
                        *mut bfd,
                        *mut asection,
                        *mut libc::c_void,
                    ) -> (),
            ),
            &mut max_section_name_length as *mut libc::c_int as *mut libc::c_void,
        );
    }
    printf(
        dcgettext(
            0 as *const libc::c_char,
            b"Idx %-*s Size      %-*s%-*sFile off  Algn\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
        max_section_name_length,
        b"Name\0" as *const u8 as *const libc::c_char,
        bfd_vma_width,
        b"VMA\0" as *const u8 as *const libc::c_char,
        bfd_vma_width,
        b"LMA\0" as *const u8 as *const libc::c_char,
    );
    if wide_output != 0 {
        printf(
            dcgettext(
                0 as *const libc::c_char,
                b"  Flags\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    bfd_map_over_sections(
        abfd,
        Some(
            dump_section_header
                as unsafe extern "C" fn(*mut bfd, *mut asection, *mut libc::c_void) -> (),
        ),
        &mut max_section_name_length as *mut libc::c_int as *mut libc::c_void,
    );
}
unsafe extern "C" fn slurp_symtab(mut abfd: *mut bfd) -> *mut *mut asymbol {
    let mut sy: *mut *mut asymbol = 0 as *mut *mut asymbol;
    let mut storage: libc::c_long = 0;
    if bfd_get_file_flags(abfd) & 0x10 as libc::c_int as libc::c_uint == 0 {
        symcount = 0 as libc::c_int as libc::c_long;
        return 0 as *mut *mut asymbol;
    }
    storage = (Some(
        ((*(*abfd).xvec)._bfd_get_symtab_upper_bound).expect("non-null function pointer"),
    ))
        .expect("non-null function pointer")(abfd);
    if storage < 0 as libc::c_int as libc::c_long {
        non_fatal(
            dcgettext(
                0 as *const libc::c_char,
                b"failed to read symbol table from: %s\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            bfd_get_filename(abfd),
        );
        bfd_fatal(
            dcgettext(
                0 as *const libc::c_char,
                b"error message was\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    if storage != 0 {
        let mut filesize: off_t = bfd_get_file_size(abfd) as off_t;
        if filesize > 0 as libc::c_int as libc::c_long && filesize < storage
            && bfd_get_flavour(abfd) as libc::c_uint
                != bfd_target_mmo_flavour as libc::c_int as libc::c_uint
        {
            bfd_nonfatal_message(
                bfd_get_filename(abfd),
                abfd,
                0 as *const asection,
                dcgettext(
                    0 as *const libc::c_char,
                    b"error: symbol table size (%#lx) is larger than filesize (%#lx)\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                storage,
                filesize,
            );
            exit_status = 1 as libc::c_int;
            symcount = 0 as libc::c_int as libc::c_long;
            return 0 as *mut *mut asymbol;
        }
        sy = xmalloc(storage as size_t) as *mut *mut asymbol;
    }
    symcount = (Some(
        ((*(*abfd).xvec)._bfd_canonicalize_symtab).expect("non-null function pointer"),
    ))
        .expect("non-null function pointer")(abfd, sy);
    if symcount < 0 as libc::c_int as libc::c_long {
        bfd_fatal(bfd_get_filename(abfd));
    }
    return sy;
}
unsafe extern "C" fn slurp_dynamic_symtab(mut abfd: *mut bfd) -> *mut *mut asymbol {
    let mut sy: *mut *mut asymbol = 0 as *mut *mut asymbol;
    let mut storage: libc::c_long = 0;
    storage = (Some(
        ((*(*abfd).xvec)._bfd_get_dynamic_symtab_upper_bound)
            .expect("non-null function pointer"),
    ))
        .expect("non-null function pointer")(abfd);
    if storage < 0 as libc::c_int as libc::c_long {
        if bfd_get_file_flags(abfd) & 0x40 as libc::c_int as libc::c_uint == 0 {
            non_fatal(
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: not a dynamic object\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                bfd_get_filename(abfd),
            );
            exit_status = 1 as libc::c_int;
            dynsymcount = 0 as libc::c_int as libc::c_long;
            return 0 as *mut *mut asymbol;
        }
        bfd_fatal(bfd_get_filename(abfd));
    }
    if storage != 0 {
        sy = xmalloc(storage as size_t) as *mut *mut asymbol;
    }
    dynsymcount = (Some(
        ((*(*abfd).xvec)._bfd_canonicalize_dynamic_symtab)
            .expect("non-null function pointer"),
    ))
        .expect("non-null function pointer")(abfd, sy);
    if dynsymcount < 0 as libc::c_int as libc::c_long {
        bfd_fatal(bfd_get_filename(abfd));
    }
    return sy;
}
unsafe extern "C" fn is_significant_symbol_name(mut name: *const libc::c_char) -> bool {
    return startswith(name, b".plt\0" as *const u8 as *const libc::c_char) as libc::c_int
        != 0
        || startswith(name, b".got\0" as *const u8 as *const libc::c_char) as libc::c_int
            != 0;
}
unsafe extern "C" fn remove_useless_symbols(
    mut symbols: *mut *mut asymbol,
    mut count: libc::c_long,
) -> libc::c_long {
    let mut in_ptr: *mut *mut asymbol = symbols;
    let mut out_ptr: *mut *mut asymbol = symbols;
    loop {
        count -= 1;
        if !(count >= 0 as libc::c_int as libc::c_long) {
            break;
        }
        let fresh5 = in_ptr;
        in_ptr = in_ptr.offset(1);
        let mut sym: *mut asymbol = *fresh5;
        if ((*sym).name).is_null()
            || *((*sym).name).offset(0 as libc::c_int as isize) as libc::c_int
                == '\0' as i32
        {
            continue;
        }
        if (*sym).flags
            & ((1 as libc::c_int) << 2 as libc::c_int
                | (1 as libc::c_int) << 8 as libc::c_int) as libc::c_uint != 0
            && !is_significant_symbol_name((*sym).name)
        {
            continue;
        }
        if bfd_is_und_section((*sym).section) as libc::c_int != 0
            || bfd_is_com_section((*sym).section) as libc::c_int != 0
        {
            continue;
        }
        let fresh6 = out_ptr;
        out_ptr = out_ptr.offset(1);
        *fresh6 = sym;
    }
    return out_ptr.offset_from(symbols) as libc::c_long;
}
static mut compare_section: *const asection = 0 as *const asection;
unsafe extern "C" fn compare_symbols(
    mut ap: *const libc::c_void,
    mut bp: *const libc::c_void,
) -> libc::c_int {
    let mut a: *const asymbol = *(ap as *mut *const asymbol);
    let mut b: *const asymbol = *(bp as *mut *const asymbol);
    let mut an: *const libc::c_char = 0 as *const libc::c_char;
    let mut bn: *const libc::c_char = 0 as *const libc::c_char;
    let mut anl: size_t = 0;
    let mut bnl: size_t = 0;
    let mut as_0: bool = false;
    let mut af: bool = false;
    let mut bs: bool = false;
    let mut bf: bool = false;
    let mut aflags: flagword = 0;
    let mut bflags: flagword = 0;
    if bfd_asymbol_value(a) > bfd_asymbol_value(b) {
        return 1 as libc::c_int
    } else if bfd_asymbol_value(a) < bfd_asymbol_value(b) {
        return -(1 as libc::c_int)
    }
    as_0 = strcmp((*compare_section).name, (*(*a).section).name) == 0 as libc::c_int;
    bs = strcmp((*compare_section).name, (*(*b).section).name) == 0 as libc::c_int;
    if as_0 as libc::c_int != 0 && !bs {
        return -(1 as libc::c_int);
    }
    if !as_0 && bs as libc::c_int != 0 {
        return 1 as libc::c_int;
    }
    an = bfd_asymbol_name(a);
    bn = bfd_asymbol_name(b);
    anl = strlen(an);
    bnl = strlen(bn);
    af = !(strstr(an, b"gnu_compiled\0" as *const u8 as *const libc::c_char)).is_null()
        || !(strstr(an, b"gcc2_compiled\0" as *const u8 as *const libc::c_char))
            .is_null();
    bf = !(strstr(bn, b"gnu_compiled\0" as *const u8 as *const libc::c_char)).is_null()
        || !(strstr(bn, b"gcc2_compiled\0" as *const u8 as *const libc::c_char))
            .is_null();
    if af as libc::c_int != 0 && !bf {
        return 1 as libc::c_int;
    }
    if !af && bf as libc::c_int != 0 {
        return -(1 as libc::c_int);
    }
    af = (*a).flags & ((1 as libc::c_int) << 14 as libc::c_int) as libc::c_uint
        != 0 as libc::c_int as libc::c_uint
        || anl > 2 as libc::c_int as libc::c_ulong
            && *an.offset(anl.wrapping_sub(2 as libc::c_int as libc::c_ulong) as isize)
                as libc::c_int == '.' as i32
            && (*an.offset(anl.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                as libc::c_int == 'o' as i32
                || *an
                    .offset(anl.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                    as libc::c_int == 'a' as i32);
    bf = (*b).flags & ((1 as libc::c_int) << 14 as libc::c_int) as libc::c_uint
        != 0 as libc::c_int as libc::c_uint
        || bnl > 2 as libc::c_int as libc::c_ulong
            && *bn.offset(bnl.wrapping_sub(2 as libc::c_int as libc::c_ulong) as isize)
                as libc::c_int == '.' as i32
            && (*bn.offset(bnl.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                as libc::c_int == 'o' as i32
                || *bn
                    .offset(bnl.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                    as libc::c_int == 'a' as i32);
    if af as libc::c_int != 0 && !bf {
        return 1 as libc::c_int;
    }
    if !af && bf as libc::c_int != 0 {
        return -(1 as libc::c_int);
    }
    aflags = (*a).flags;
    bflags = (*b).flags;
    if aflags & ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uint
        != bflags & ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uint
    {
        if aflags & ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uint
            != 0 as libc::c_int as libc::c_uint
        {
            return 1 as libc::c_int
        } else {
            return -(1 as libc::c_int)
        }
    }
    if aflags & ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_uint
        != bflags & ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_uint
    {
        if aflags & ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_uint
            != 0 as libc::c_int as libc::c_uint
        {
            return 1 as libc::c_int
        } else {
            return -(1 as libc::c_int)
        }
    }
    if aflags & ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_uint
        != bflags & ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_uint
    {
        if aflags & ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_uint
            != 0 as libc::c_int as libc::c_uint
        {
            return -(1 as libc::c_int)
        } else {
            return 1 as libc::c_int
        }
    }
    if aflags & ((1 as libc::c_int) << 16 as libc::c_int) as libc::c_uint
        != bflags & ((1 as libc::c_int) << 16 as libc::c_int) as libc::c_uint
    {
        if aflags & ((1 as libc::c_int) << 16 as libc::c_int) as libc::c_uint
            != 0 as libc::c_int as libc::c_uint
        {
            return -(1 as libc::c_int)
        } else {
            return 1 as libc::c_int
        }
    }
    if aflags & ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint
        != bflags & ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint
    {
        if aflags & ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint
            != 0 as libc::c_int as libc::c_uint
        {
            return 1 as libc::c_int
        } else {
            return -(1 as libc::c_int)
        }
    }
    if aflags & ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uint
        != bflags & ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uint
    {
        if aflags & ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uint
            != 0 as libc::c_int as libc::c_uint
        {
            return -(1 as libc::c_int)
        } else {
            return 1 as libc::c_int
        }
    }
    if bfd_get_flavour(bfd_asymbol_bfd(a)) as libc::c_uint
        == bfd_target_elf_flavour as libc::c_int as libc::c_uint
        && bfd_get_flavour(bfd_asymbol_bfd(b)) as libc::c_uint
            == bfd_target_elf_flavour as libc::c_int as libc::c_uint
    {
        let mut asz: bfd_vma = 0;
        let mut bsz: bfd_vma = 0;
        asz = 0 as libc::c_int as bfd_vma;
        if (*a).flags
            & ((1 as libc::c_int) << 8 as libc::c_int
                | (1 as libc::c_int) << 21 as libc::c_int) as libc::c_uint
            == 0 as libc::c_int as libc::c_uint
        {
            asz = (*(a as *mut elf_symbol_type)).internal_elf_sym.st_size;
        }
        bsz = 0 as libc::c_int as bfd_vma;
        if (*b).flags
            & ((1 as libc::c_int) << 8 as libc::c_int
                | (1 as libc::c_int) << 21 as libc::c_int) as libc::c_uint
            == 0 as libc::c_int as libc::c_uint
        {
            bsz = (*(b as *mut elf_symbol_type)).internal_elf_sym.st_size;
        }
        if asz != bsz {
            return if asz > bsz { -(1 as libc::c_int) } else { 1 as libc::c_int };
        }
    }
    if *an.offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32
        && *bn.offset(0 as libc::c_int as isize) as libc::c_int != '.' as i32
    {
        return 1 as libc::c_int;
    }
    if *an.offset(0 as libc::c_int as isize) as libc::c_int != '.' as i32
        && *bn.offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32
    {
        return -(1 as libc::c_int);
    }
    return strcmp(an, bn);
}
unsafe extern "C" fn compare_relocs(
    mut ap: *const libc::c_void,
    mut bp: *const libc::c_void,
) -> libc::c_int {
    let mut a: *const arelent = *(ap as *mut *const arelent);
    let mut b: *const arelent = *(bp as *mut *const arelent);
    if (*a).address > (*b).address {
        return 1 as libc::c_int
    } else if (*a).address < (*b).address {
        return -(1 as libc::c_int)
    }
    if a > b {
        return 1 as libc::c_int
    } else if a < b {
        return -(1 as libc::c_int)
    } else {
        return 0 as libc::c_int
    };
}
unsafe extern "C" fn objdump_print_value(
    mut vma: bfd_vma,
    mut inf: *mut disassemble_info,
    mut skip_zeroes: bool,
) {
    let mut buf: [libc::c_char; 30] = [0; 30];
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut aux: *mut objdump_disasm_info = 0 as *mut objdump_disasm_info;
    aux = (*inf).application_data as *mut objdump_disasm_info;
    bfd_sprintf_vma((*aux).abfd, buf.as_mut_ptr(), vma);
    if !skip_zeroes {
        p = buf.as_mut_ptr();
    } else {
        p = buf.as_mut_ptr();
        while *p as libc::c_int == '0' as i32 {
            p = p.offset(1);
            p;
        }
        if *p as libc::c_int == '\0' as i32 {
            p = p.offset(-1);
            p;
        }
    }
    (Some(((*inf).fprintf_func).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )((*inf).stream, b"%s\0" as *const u8 as *const libc::c_char, p);
}
unsafe extern "C" fn objdump_print_symname(
    mut abfd: *mut bfd,
    mut inf: *mut disassemble_info,
    mut sym: *mut asymbol,
) {
    let mut alloc: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut version_string: *const libc::c_char = 0 as *const libc::c_char;
    let mut hidden: bool = 0 as libc::c_int != 0;
    alloc = 0 as *mut libc::c_char;
    name = bfd_asymbol_name(sym);
    if do_demangle != 0
        && *name.offset(0 as libc::c_int as isize) as libc::c_int != '\0' as i32
    {
        alloc = bfd_demangle(abfd, name, demangle_flags);
        if !alloc.is_null() {
            name = alloc;
        }
    }
    if (*sym).flags
        & ((1 as libc::c_int) << 8 as libc::c_int
            | (1 as libc::c_int) << 21 as libc::c_int) as libc::c_uint
        == 0 as libc::c_int as libc::c_uint
    {
        version_string = (Some(
            ((*(*abfd).xvec)._bfd_get_symbol_version_string)
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(abfd, sym, 1 as libc::c_int != 0, &mut hidden);
    }
    if bfd_is_und_section(bfd_asymbol_section(sym)) {
        hidden = 1 as libc::c_int != 0;
    }
    name = sanitize_string(name);
    if !inf.is_null() {
        (Some(((*inf).fprintf_func).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )((*inf).stream, b"%s\0" as *const u8 as *const libc::c_char, name);
        if !version_string.is_null() && *version_string as libc::c_int != '\0' as i32 {
            (Some(((*inf).fprintf_func).expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                (*inf).stream,
                if hidden as libc::c_int != 0 {
                    b"@%s\0" as *const u8 as *const libc::c_char
                } else {
                    b"@@%s\0" as *const u8 as *const libc::c_char
                },
                version_string,
            );
        }
    } else {
        printf(b"%s\0" as *const u8 as *const libc::c_char, name);
        if !version_string.is_null() && *version_string as libc::c_int != '\0' as i32 {
            printf(
                if hidden as libc::c_int != 0 {
                    b"@%s\0" as *const u8 as *const libc::c_char
                } else {
                    b"@@%s\0" as *const u8 as *const libc::c_char
                },
                version_string,
            );
        }
    }
    if !alloc.is_null() {
        free(alloc as *mut libc::c_void);
    }
}
#[inline]
unsafe extern "C" fn sym_ok(
    mut want_section: bool,
    mut _abfd: *mut bfd,
    mut place: libc::c_long,
    mut sec: *mut asection,
    mut inf: *mut disassemble_info,
) -> bool {
    if want_section {
        if (*(**sorted_syms.offset(place as isize)).section).owner == (*sec).owner
            && (**sorted_syms.offset(place as isize)).section != sec
        {
            return 0 as libc::c_int != 0;
        }
        if strcmp(
            bfd_section_name((**sorted_syms.offset(place as isize)).section),
            bfd_section_name(sec),
        ) != 0 as libc::c_int
        {
            return 0 as libc::c_int != 0;
        }
    }
    return ((*inf).symbol_is_valid)
        .expect("non-null function pointer")(*sorted_syms.offset(place as isize), inf);
}
unsafe extern "C" fn find_symbol_for_address(
    mut vma: bfd_vma,
    mut inf: *mut disassemble_info,
    mut place: *mut libc::c_long,
) -> *mut asymbol {
    let mut min: libc::c_long = 0 as libc::c_int as libc::c_long;
    let mut max_count: libc::c_long = sorted_symcount;
    let mut thisplace: libc::c_long = 0;
    let mut aux: *mut objdump_disasm_info = 0 as *mut objdump_disasm_info;
    let mut abfd: *mut bfd = 0 as *mut bfd;
    let mut sec: *mut asection = 0 as *mut asection;
    let mut opb: libc::c_uint = 0;
    let mut want_section: bool = false;
    let mut rel_count: libc::c_long = 0;
    if sorted_symcount < 1 as libc::c_int as libc::c_long {
        return 0 as *mut asymbol;
    }
    aux = (*inf).application_data as *mut objdump_disasm_info;
    abfd = (*aux).abfd;
    sec = (*inf).section;
    opb = (*inf).octets_per_byte;
    while (min + 1 as libc::c_int as libc::c_long) < max_count {
        let mut sym: *mut asymbol = 0 as *mut asymbol;
        thisplace = (max_count + min) / 2 as libc::c_int as libc::c_long;
        sym = *sorted_syms.offset(thisplace as isize);
        if bfd_asymbol_value(sym) > vma {
            max_count = thisplace;
        } else if bfd_asymbol_value(sym) < vma {
            min = thisplace;
        } else {
            min = thisplace;
            break;
        }
    }
    thisplace = min;
    while thisplace > 0 as libc::c_int as libc::c_long
        && bfd_asymbol_value(*sorted_syms.offset(thisplace as isize))
            == bfd_asymbol_value(
                *sorted_syms
                    .offset((thisplace - 1 as libc::c_int as libc::c_long) as isize),
            )
    {
        thisplace -= 1;
        thisplace;
    }
    min = thisplace;
    while min < max_count
        && bfd_asymbol_value(*sorted_syms.offset(min as isize))
            == bfd_asymbol_value(*sorted_syms.offset(thisplace as isize))
    {
        if sym_ok(1 as libc::c_int != 0, abfd, min, sec, inf) {
            thisplace = min;
            if !place.is_null() {
                *place = thisplace;
            }
            return *sorted_syms.offset(thisplace as isize);
        }
        min += 1;
        min;
    }
    want_section = (*aux).require_sec as libc::c_int != 0
        || (*abfd).flags & 0x1 as libc::c_int as libc::c_uint
            != 0 as libc::c_int as libc::c_uint && vma >= bfd_section_vma(sec)
            && vma
                < (bfd_section_vma(sec))
                    .wrapping_add(
                        (bfd_section_size(sec)).wrapping_div(opb as libc::c_ulong),
                    );
    if !sym_ok(want_section, abfd, thisplace, sec, inf) {
        let mut i: libc::c_long = 0;
        let mut newplace: libc::c_long = sorted_symcount;
        i = min - 1 as libc::c_int as libc::c_long;
        while i >= 0 as libc::c_int as libc::c_long {
            if sym_ok(want_section, abfd, i, sec, inf) {
                if newplace == sorted_symcount {
                    newplace = i;
                }
                if bfd_asymbol_value(*sorted_syms.offset(i as isize))
                    != bfd_asymbol_value(*sorted_syms.offset(newplace as isize))
                {
                    break;
                }
                newplace = i;
            }
            i -= 1;
            i;
        }
        if newplace != sorted_symcount {
            thisplace = newplace;
        } else {
            i = thisplace + 1 as libc::c_int as libc::c_long;
            while i < sorted_symcount {
                if sym_ok(want_section, abfd, i, sec, inf) {
                    thisplace = i;
                    break;
                } else {
                    i += 1;
                    i;
                }
            }
        }
        if !sym_ok(want_section, abfd, thisplace, sec, inf) {
            return 0 as *mut asymbol;
        }
    }
    rel_count = (*inf).dynrelcount;
    if !want_section && (**sorted_syms.offset(thisplace as isize)).value != vma
        && rel_count > 0 as libc::c_int as libc::c_long && !((*inf).dynrelbuf).is_null()
        && (**((*inf).dynrelbuf).offset(0 as libc::c_int as isize)).address <= vma
        && (**((*inf).dynrelbuf)
            .offset((rel_count - 1 as libc::c_int as libc::c_long) as isize))
            .address >= vma
        && (**sorted_syms.offset(thisplace as isize)).flags
            & ((1 as libc::c_int) << 21 as libc::c_int) as libc::c_uint
            == 0 as libc::c_int as libc::c_uint
    {
        let mut rel_low: *mut *mut arelent = 0 as *mut *mut arelent;
        let mut rel_high: *mut *mut arelent = 0 as *mut *mut arelent;
        rel_low = (*inf).dynrelbuf;
        rel_high = rel_low
            .offset(rel_count as isize)
            .offset(-(1 as libc::c_int as isize));
        while rel_low <= rel_high {
            let mut rel_mid: *mut *mut arelent = &mut *rel_low
                .offset(
                    (rel_high.offset_from(rel_low) as libc::c_long
                        / 2 as libc::c_int as libc::c_long) as isize,
                ) as *mut *mut arelent;
            let mut rel: *mut arelent = *rel_mid;
            if (*rel).address == vma {
                let mut rel_vma: *mut *mut arelent = rel_mid;
                rel_mid = rel_mid.offset(-1);
                rel_mid;
                while rel_mid >= rel_low
                    && (**rel_mid.offset(0 as libc::c_int as isize)).address == vma
                {
                    rel_vma = rel_mid;
                    rel_mid = rel_mid.offset(-1);
                    rel_mid;
                }
                while rel_vma <= rel_high
                    && (**rel_vma.offset(0 as libc::c_int as isize)).address == vma
                {
                    rel = *rel_vma;
                    if !((*rel).sym_ptr_ptr).is_null()
                        && !bfd_is_abs_section((**(*rel).sym_ptr_ptr).section)
                    {
                        if !place.is_null() {
                            *place = thisplace;
                        }
                        return *(*rel).sym_ptr_ptr;
                    }
                    rel_vma = rel_vma.offset(1);
                    rel_vma;
                }
                break;
            } else if vma < (*rel).address {
                rel_high = rel_mid;
            } else {
                if !(vma >= (**rel_mid.offset(1 as libc::c_int as isize)).address) {
                    break;
                }
                rel_low = rel_mid.offset(1 as libc::c_int as isize);
            }
        }
    }
    if !place.is_null() {
        *place = thisplace;
    }
    return *sorted_syms.offset(thisplace as isize);
}
unsafe extern "C" fn objdump_print_addr_with_sym(
    mut abfd: *mut bfd,
    mut sec: *mut asection,
    mut sym: *mut asymbol,
    mut vma: bfd_vma,
    mut inf: *mut disassemble_info,
    mut skip_zeroes: bool,
) {
    if no_addresses == 0 {
        objdump_print_value(vma, inf, skip_zeroes);
        (Some(((*inf).fprintf_func).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )((*inf).stream, b" \0" as *const u8 as *const libc::c_char);
    }
    if sym.is_null() {
        let mut secaddr: bfd_vma = 0;
        (Some(((*inf).fprintf_func).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            (*inf).stream,
            b"<%s\0" as *const u8 as *const libc::c_char,
            sanitize_string(bfd_section_name(sec)),
        );
        secaddr = bfd_section_vma(sec);
        if vma < secaddr {
            (Some(((*inf).fprintf_func).expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )((*inf).stream, b"-0x\0" as *const u8 as *const libc::c_char);
            objdump_print_value(secaddr.wrapping_sub(vma), inf, 1 as libc::c_int != 0);
        } else if vma > secaddr {
            (Some(((*inf).fprintf_func).expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )((*inf).stream, b"+0x\0" as *const u8 as *const libc::c_char);
            objdump_print_value(vma.wrapping_sub(secaddr), inf, 1 as libc::c_int != 0);
        }
        (Some(((*inf).fprintf_func).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )((*inf).stream, b">\0" as *const u8 as *const libc::c_char);
    } else {
        (Some(((*inf).fprintf_func).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )((*inf).stream, b"<\0" as *const u8 as *const libc::c_char);
        objdump_print_symname(abfd, inf, sym);
        if !(bfd_asymbol_value(sym) == vma) {
            if !(bfd_get_file_flags(abfd)
                & (0x2 as libc::c_int | 0x40 as libc::c_int) as libc::c_uint != 0
                && bfd_is_und_section((*sym).section) as libc::c_int != 0)
            {
                if bfd_asymbol_value(sym) > vma {
                    (Some(((*inf).fprintf_func).expect("non-null function pointer")))
                        .expect(
                            "non-null function pointer",
                        )((*inf).stream, b"-0x\0" as *const u8 as *const libc::c_char);
                    objdump_print_value(
                        (bfd_asymbol_value(sym)).wrapping_sub(vma),
                        inf,
                        1 as libc::c_int != 0,
                    );
                } else if vma > bfd_asymbol_value(sym) {
                    (Some(((*inf).fprintf_func).expect("non-null function pointer")))
                        .expect(
                            "non-null function pointer",
                        )((*inf).stream, b"+0x\0" as *const u8 as *const libc::c_char);
                    objdump_print_value(
                        vma.wrapping_sub(bfd_asymbol_value(sym)),
                        inf,
                        1 as libc::c_int != 0,
                    );
                }
            }
        }
        (Some(((*inf).fprintf_func).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )((*inf).stream, b">\0" as *const u8 as *const libc::c_char);
    }
    if display_file_offsets {
        ((*inf).fprintf_func)
            .expect(
                "non-null function pointer",
            )(
            (*inf).stream,
            dcgettext(
                0 as *const libc::c_char,
                b" (File Offset: 0x%lx)\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            ((*sec).filepos as libc::c_ulong).wrapping_add(vma.wrapping_sub((*sec).vma))
                as libc::c_long,
        );
    }
}
unsafe extern "C" fn objdump_print_addr(
    mut vma: bfd_vma,
    mut inf: *mut disassemble_info,
    mut skip_zeroes: bool,
) {
    let mut aux: *mut objdump_disasm_info = 0 as *mut objdump_disasm_info;
    let mut sym: *mut asymbol = 0 as *mut asymbol;
    let mut skip_find: bool = 0 as libc::c_int != 0;
    aux = (*inf).application_data as *mut objdump_disasm_info;
    if sorted_symcount < 1 as libc::c_int as libc::c_long {
        if no_addresses == 0 {
            (Some(((*inf).fprintf_func).expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )((*inf).stream, b"0x\0" as *const u8 as *const libc::c_char);
            objdump_print_value(vma, inf, skip_zeroes);
        }
        if display_file_offsets {
            ((*inf).fprintf_func)
                .expect(
                    "non-null function pointer",
                )(
                (*inf).stream,
                dcgettext(
                    0 as *const libc::c_char,
                    b" (File Offset: 0x%lx)\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                ((*(*inf).section).filepos as libc::c_ulong)
                    .wrapping_add(vma.wrapping_sub((*(*inf).section).vma))
                    as libc::c_long,
            );
        }
        return;
    }
    if !((*aux).reloc).is_null() && !((*(*aux).reloc).sym_ptr_ptr).is_null()
        && !(*(*(*aux).reloc).sym_ptr_ptr).is_null()
    {
        sym = *(*(*aux).reloc).sym_ptr_ptr;
        vma = (vma as libc::c_ulong).wrapping_add(bfd_asymbol_value(sym)) as bfd_vma
            as bfd_vma;
        if bfd_is_und_section(bfd_asymbol_section(sym)) {
            skip_find = 1 as libc::c_int != 0;
        }
    }
    if !skip_find {
        sym = find_symbol_for_address(vma, inf, 0 as *mut libc::c_long);
    }
    objdump_print_addr_with_sym((*aux).abfd, (*inf).section, sym, vma, inf, skip_zeroes);
}
unsafe extern "C" fn objdump_print_address(
    mut vma: bfd_vma,
    mut inf: *mut disassemble_info,
) {
    objdump_print_addr(vma, inf, prefix_addresses == 0);
}
unsafe extern "C" fn objdump_symbol_at_address(
    mut vma: bfd_vma,
    mut inf: *mut disassemble_info,
) -> *mut asymbol {
    let mut sym: *mut asymbol = 0 as *mut asymbol;
    sym = find_symbol_for_address(vma, inf, 0 as *mut libc::c_long);
    if !sym.is_null() && bfd_asymbol_value(sym) == vma {
        return sym;
    }
    return 0 as *mut asymbol;
}
static mut prev_functionname: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
static mut prev_line: libc::c_uint = 0;
static mut prev_discriminator: libc::c_uint = 0;
static mut print_files: *mut print_file_list = 0 as *const print_file_list
    as *mut print_file_list;
unsafe extern "C" fn slurp_file(
    mut fn_0: *const libc::c_char,
    mut size: *mut size_t,
    mut fst: *mut stat,
) -> *const libc::c_char {
    let mut ps: libc::c_int = getpagesize();
    let mut msize: size_t = 0;
    let mut map: *const libc::c_char = 0 as *const libc::c_char;
    let mut fd: libc::c_int = open(fn_0, 0 as libc::c_int | 0 as libc::c_int);
    if fd < 0 as libc::c_int {
        return 0 as *const libc::c_char;
    }
    if fstat(fd, fst) < 0 as libc::c_int {
        close(fd);
        return 0 as *const libc::c_char;
    }
    *size = (*fst).st_size as size_t;
    msize = (*size)
        .wrapping_add(ps as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        & !(ps - 1 as libc::c_int) as libc::c_ulong;
    map = mmap(
        0 as *mut libc::c_void,
        msize,
        0x1 as libc::c_int,
        0x1 as libc::c_int,
        fd,
        0 as libc::c_int as __off_t,
    ) as *const libc::c_char;
    if map != -(1 as libc::c_long) as *mut libc::c_char as *const libc::c_char {
        close(fd);
        return map;
    }
    map = malloc(*size) as *const libc::c_char;
    if map.is_null()
        || read(fd, map as *mut libc::c_char as *mut libc::c_void, *size) as size_t
            != *size
    {
        free(map as *mut libc::c_void);
        map = 0 as *const libc::c_char;
    }
    close(fd);
    return map;
}
unsafe extern "C" fn index_file(
    mut map: *const libc::c_char,
    mut size: size_t,
    mut maxline: *mut libc::c_uint,
) -> *mut *const libc::c_char {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut lstart: *const libc::c_char = 0 as *const libc::c_char;
    let mut end: *const libc::c_char = 0 as *const libc::c_char;
    let mut chars_per_line: libc::c_int = 45 as libc::c_int;
    let mut lineno: libc::c_uint = 0;
    let mut linemap: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
    let mut line_map_size: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    lineno = 0 as libc::c_int as libc::c_uint;
    lstart = map;
    end = map.offset(size as isize);
    let mut current_block_21: u64;
    p = map;
    while p < end {
        if *p as libc::c_int == '\n' as i32 {
            if p.offset(1 as libc::c_int as isize) < end
                && *p.offset(1 as libc::c_int as isize) as libc::c_int == '\r' as i32
            {
                p = p.offset(1);
                p;
            }
            current_block_21 = 1841672684692190573;
        } else if *p as libc::c_int == '\r' as i32 {
            if p.offset(1 as libc::c_int as isize) < end
                && *p.offset(1 as libc::c_int as isize) as libc::c_int == '\n' as i32
            {
                p = p.offset(1);
                p;
            }
            current_block_21 = 1841672684692190573;
        } else {
            current_block_21 = 715039052867723359;
        }
        match current_block_21 {
            1841672684692190573 => {
                if linemap.is_null()
                    || line_map_size
                        < lineno.wrapping_add(1 as libc::c_int as libc::c_uint)
                            as libc::c_ulong
                {
                    let mut newsize: libc::c_ulong = 0;
                    chars_per_line -= 5 as libc::c_int;
                    if chars_per_line <= 1 as libc::c_int {
                        chars_per_line = 1 as libc::c_int;
                    }
                    line_map_size = size
                        .wrapping_div(chars_per_line as libc::c_ulong)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong);
                    if line_map_size
                        < lineno.wrapping_add(1 as libc::c_int as libc::c_uint)
                            as libc::c_ulong
                    {
                        line_map_size = lineno
                            .wrapping_add(1 as libc::c_int as libc::c_uint)
                            as libc::c_ulong;
                    }
                    newsize = line_map_size
                        .wrapping_mul(
                            ::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                        );
                    linemap = xrealloc(linemap as *mut libc::c_void, newsize)
                        as *mut *const libc::c_char;
                }
                let fresh7 = lineno;
                lineno = lineno.wrapping_add(1);
                let ref mut fresh8 = *linemap.offset(fresh7 as isize);
                *fresh8 = lstart;
                lstart = p.offset(1 as libc::c_int as isize);
            }
            _ => {}
        }
        p = p.offset(1);
        p;
    }
    *maxline = lineno;
    return linemap;
}
unsafe extern "C" fn try_print_file_open(
    mut origname: *const libc::c_char,
    mut modname: *const libc::c_char,
    mut fst: *mut stat,
) -> *mut print_file_list {
    let mut p: *mut print_file_list = 0 as *mut print_file_list;
    p = xmalloc(::core::mem::size_of::<print_file_list>() as libc::c_ulong)
        as *mut print_file_list;
    (*p).map = slurp_file(modname, &mut (*p).mapsize, fst);
    if ((*p).map).is_null() {
        free(p as *mut libc::c_void);
        return 0 as *mut print_file_list;
    }
    (*p).linemap = index_file((*p).map, (*p).mapsize, &mut (*p).maxline);
    (*p).last_line = 0 as libc::c_int as libc::c_uint;
    (*p).max_printed = 0 as libc::c_int as libc::c_uint;
    (*p).filename = origname;
    (*p).modname = modname;
    (*p).next = print_files;
    (*p).first = 1 as libc::c_int;
    print_files = p;
    return p;
}
unsafe extern "C" fn update_source_path(
    mut filename: *const libc::c_char,
    mut abfd: *mut bfd,
) -> *mut print_file_list {
    let mut p: *mut print_file_list = 0 as *mut print_file_list;
    let mut fname: *const libc::c_char = 0 as *const libc::c_char;
    let mut fst: stat = stat {
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
    let mut i: libc::c_int = 0;
    p = try_print_file_open(filename, filename, &mut fst);
    if p.is_null() {
        if include_path_count == 0 as libc::c_int {
            return 0 as *mut print_file_list;
        }
        fname = lbasename(filename);
        i = 0 as libc::c_int;
        while i < include_path_count {
            let mut modname: *mut libc::c_char = concat(
                *include_paths.offset(i as isize),
                b"/\0" as *const u8 as *const libc::c_char,
                fname,
                0 as *const libc::c_char,
            );
            p = try_print_file_open(filename, modname, &mut fst);
            if !p.is_null() {
                break;
            }
            free(modname as *mut libc::c_void);
            i += 1;
            i;
        }
    }
    if !p.is_null() {
        let mut mtime: libc::c_long = bfd_get_mtime(abfd);
        if fst.st_mtim.tv_sec > mtime {
            warn(
                dcgettext(
                    0 as *const libc::c_char,
                    b"source file %s is more recent than object file\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                filename,
            );
        }
    }
    return p;
}
unsafe extern "C" fn print_line(mut p: *mut print_file_list, mut linenum: libc::c_uint) {
    let mut l: *const libc::c_char = 0 as *const libc::c_char;
    let mut len: size_t = 0;
    linenum = linenum.wrapping_sub(1);
    linenum;
    if linenum >= (*p).maxline {
        return;
    }
    l = *((*p).linemap).offset(linenum as isize);
    if !source_comment.is_null() && strlen(l) > 0 as libc::c_int as libc::c_ulong {
        printf(b"%s\0" as *const u8 as *const libc::c_char, source_comment);
    }
    len = strcspn(l, b"\n\r\0" as *const u8 as *const libc::c_char);
    if len == 0 as libc::c_int as libc::c_ulong
        || fwrite(
            l as *const libc::c_void,
            len,
            1 as libc::c_int as libc::c_ulong,
            stdout,
        ) == 1 as libc::c_int as libc::c_ulong
    {
        putchar('\n' as i32);
    }
}
unsafe extern "C" fn dump_lines(
    mut p: *mut print_file_list,
    mut start: libc::c_uint,
    mut end: libc::c_uint,
) {
    if ((*p).map).is_null() {
        return;
    }
    while start <= end {
        print_line(p, start);
        start = start.wrapping_add(1);
        start;
    }
}
unsafe extern "C" fn show_line(
    mut abfd: *mut bfd,
    mut section: *mut asection,
    mut addr_offset: bfd_vma,
) {
    let mut filename: *const libc::c_char = 0 as *const libc::c_char;
    let mut functionname: *const libc::c_char = 0 as *const libc::c_char;
    let mut linenumber: libc::c_uint = 0;
    let mut discriminator: libc::c_uint = 0;
    let mut reloc: bool = false;
    let mut path: *mut libc::c_char = 0 as *mut libc::c_char;
    if with_line_numbers == 0 && !with_source_code {
        return;
    }
    if !(Some(
        ((*(*abfd).xvec)._bfd_find_nearest_line).expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )(
        abfd,
        syms,
        section,
        addr_offset,
        &mut filename,
        &mut functionname,
        &mut linenumber,
        &mut discriminator,
    )
    {
        return;
    }
    if !filename.is_null() && *filename as libc::c_int == '\0' as i32 {
        filename = 0 as *const libc::c_char;
    }
    if !functionname.is_null() && *functionname as libc::c_int == '\0' as i32 {
        functionname = 0 as *const libc::c_char;
    }
    if !filename.is_null()
        && (*filename.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32
            || *filename.offset(0 as libc::c_int as isize) as libc::c_int == '\\' as i32
                && 0 as libc::c_int != 0
            || *filename.offset(0 as libc::c_int as isize) as libc::c_int != 0
                && *filename.offset(1 as libc::c_int as isize) as libc::c_int
                    == ':' as i32 && 0 as libc::c_int != 0) && !prefix.is_null()
    {
        let mut path_up: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut fname: *const libc::c_char = filename;
        path = xmalloc(
            prefix_length
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_add(strlen(filename)),
        ) as *mut libc::c_char;
        if prefix_length != 0 {
            memcpy(
                path as *mut libc::c_void,
                prefix as *const libc::c_void,
                prefix_length,
            );
        }
        path_up = path.offset(prefix_length as isize);
        if prefix_strip > 0 as libc::c_int {
            let mut level: libc::c_int = 0 as libc::c_int;
            let mut s: *const libc::c_char = 0 as *const libc::c_char;
            s = fname.offset(1 as libc::c_int as isize);
            while *s as libc::c_int != '\0' as i32 && level < prefix_strip {
                if *s as libc::c_int == '/' as i32
                    || *s as libc::c_int == '\\' as i32 && 0 as libc::c_int != 0
                {
                    fname = s;
                    level += 1;
                    level;
                }
                s = s.offset(1);
                s;
            }
        }
        strcpy(path_up, fname);
        filename = path;
        reloc = 1 as libc::c_int != 0;
    } else {
        reloc = 0 as libc::c_int != 0;
    }
    if with_line_numbers != 0 {
        if !functionname.is_null()
            && (prev_functionname.is_null()
                || strcmp(functionname, prev_functionname) != 0 as libc::c_int)
        {
            let mut demangle_alloc: *mut libc::c_char = 0 as *mut libc::c_char;
            if do_demangle != 0
                && *functionname.offset(0 as libc::c_int as isize) as libc::c_int
                    != '\0' as i32
            {
                demangle_alloc = bfd_demangle(abfd, functionname, demangle_flags);
            }
            if !demangle_alloc.is_null() {
                printf(
                    b"%s:\n\0" as *const u8 as *const libc::c_char,
                    sanitize_string(demangle_alloc),
                );
            } else {
                printf(
                    b"%s():\n\0" as *const u8 as *const libc::c_char,
                    sanitize_string(functionname),
                );
            }
            prev_line = -(1 as libc::c_int) as libc::c_uint;
            free(demangle_alloc as *mut libc::c_void);
        }
        if linenumber > 0 as libc::c_int as libc::c_uint
            && (linenumber != prev_line || discriminator != prev_discriminator)
        {
            if discriminator > 0 as libc::c_int as libc::c_uint {
                printf(
                    b"%s:%u (discriminator %u)\n\0" as *const u8 as *const libc::c_char,
                    if filename.is_null() {
                        b"???\0" as *const u8 as *const libc::c_char
                    } else {
                        sanitize_string(filename)
                    },
                    linenumber,
                    discriminator,
                );
            } else {
                printf(
                    b"%s:%u\n\0" as *const u8 as *const libc::c_char,
                    if filename.is_null() {
                        b"???\0" as *const u8 as *const libc::c_char
                    } else {
                        sanitize_string(filename)
                    },
                    linenumber,
                );
            }
        }
        if unwind_inlines {
            let mut filename2: *const libc::c_char = 0 as *const libc::c_char;
            let mut functionname2: *const libc::c_char = 0 as *const libc::c_char;
            let mut line2: libc::c_uint = 0;
            while (Some(
                ((*(*abfd).xvec)._bfd_find_inliner_info)
                    .expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(abfd, &mut filename2, &mut functionname2, &mut line2)
            {
                printf(
                    b"inlined by %s:%u\0" as *const u8 as *const libc::c_char,
                    sanitize_string(filename2),
                    line2,
                );
                printf(
                    b" (%s)\n\0" as *const u8 as *const libc::c_char,
                    sanitize_string(functionname2),
                );
            }
        }
    }
    if with_source_code as libc::c_int != 0 && !filename.is_null()
        && linenumber > 0 as libc::c_int as libc::c_uint
    {
        let mut pp: *mut *mut print_file_list = 0 as *mut *mut print_file_list;
        let mut p: *mut print_file_list = 0 as *mut print_file_list;
        let mut l: libc::c_uint = 0;
        pp = &mut print_files;
        while !(*pp).is_null() {
            if filename_cmp((**pp).filename, filename) == 0 as libc::c_int {
                break;
            }
            pp = &mut (**pp).next;
        }
        p = *pp;
        if p.is_null() {
            if reloc {
                filename = xstrdup(filename);
            }
            p = update_source_path(filename, abfd);
        }
        if !p.is_null() && linenumber != (*p).last_line {
            if file_start_context != 0 && (*p).first != 0 {
                l = 1 as libc::c_int as libc::c_uint;
            } else {
                l = linenumber.wrapping_sub(5 as libc::c_int as libc::c_uint);
                if l >= linenumber {
                    l = 1 as libc::c_int as libc::c_uint;
                }
                if (*p).max_printed >= l {
                    if (*p).max_printed < linenumber {
                        l = ((*p).max_printed)
                            .wrapping_add(1 as libc::c_int as libc::c_uint);
                    } else {
                        l = linenumber;
                    }
                }
            }
            dump_lines(p, l, linenumber);
            if (*p).max_printed < linenumber {
                (*p).max_printed = linenumber;
            }
            (*p).last_line = linenumber;
            (*p).first = 0 as libc::c_int;
        }
    }
    if !functionname.is_null()
        && (prev_functionname.is_null()
            || strcmp(functionname, prev_functionname) != 0 as libc::c_int)
    {
        if !prev_functionname.is_null() {
            free(prev_functionname as *mut libc::c_void);
        }
        prev_functionname = xmalloc(
            (strlen(functionname)).wrapping_add(1 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
        strcpy(prev_functionname, functionname);
    }
    if linenumber > 0 as libc::c_int as libc::c_uint && linenumber != prev_line {
        prev_line = linenumber;
    }
    if discriminator != prev_discriminator {
        prev_discriminator = discriminator;
    }
    if !path.is_null() {
        free(path as *mut libc::c_void);
    }
}
unsafe extern "C" fn objdump_sprintf(
    mut f: *mut SFILE,
    mut format: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut n: size_t = 0;
    let mut args_0: ::core::ffi::VaListImpl;
    loop {
        let mut space: size_t = ((*f).alloc).wrapping_sub((*f).pos);
        args_0 = args.clone();
        n = vsnprintf(
            ((*f).buffer).offset((*f).pos as isize),
            space,
            format,
            args_0.as_va_list(),
        ) as size_t;
        if space > n {
            break;
        }
        (*f)
            .alloc = ((*f).alloc)
            .wrapping_add(n)
            .wrapping_mul(2 as libc::c_int as libc::c_ulong);
        (*f)
            .buffer = xrealloc((*f).buffer as *mut libc::c_void, (*f).alloc)
            as *mut libc::c_char;
    }
    (*f).pos = ((*f).pos as libc::c_ulong).wrapping_add(n) as size_t as size_t;
    return n as libc::c_int;
}
unsafe extern "C" fn jump_info_new(
    mut start: bfd_vma,
    mut end: bfd_vma,
    mut level: libc::c_int,
) -> *mut jump_info {
    let mut result: *mut jump_info = xmalloc(
        ::core::mem::size_of::<jump_info>() as libc::c_ulong,
    ) as *mut jump_info;
    (*result).next = 0 as *mut jump_info;
    (*result).prev = 0 as *mut jump_info;
    (*result)
        .start
        .addresses = xmalloc(
        (::core::mem::size_of::<*mut bfd_vma>() as libc::c_ulong)
            .wrapping_mul(2 as libc::c_int as libc::c_ulong),
    ) as *mut bfd_vma;
    *((*result).start.addresses).offset(0 as libc::c_int as isize) = start;
    (*result).start.count = 1 as libc::c_int as size_t;
    (*result).start.max_count = 2 as libc::c_int as size_t;
    (*result).end = end;
    (*result).level = level;
    return result;
}
unsafe extern "C" fn jump_info_free(mut ji: *mut jump_info) -> *mut jump_info {
    let mut result: *mut jump_info = 0 as *mut jump_info;
    if !ji.is_null() {
        result = (*ji).next;
        if !((*ji).start.addresses).is_null() {
            free((*ji).start.addresses as *mut libc::c_void);
        }
        free(ji as *mut libc::c_void);
    }
    return result;
}
unsafe extern "C" fn jump_info_min_address(mut ji: *const jump_info) -> bfd_vma {
    let mut min_address: bfd_vma = (*ji).end;
    let mut i: size_t = 0;
    i = (*ji).start.count;
    loop {
        let fresh9 = i;
        i = i.wrapping_sub(1);
        if !(fresh9 > 0 as libc::c_int as libc::c_ulong) {
            break;
        }
        if *((*ji).start.addresses).offset(i as isize) < min_address {
            min_address = *((*ji).start.addresses).offset(i as isize);
        }
    }
    return min_address;
}
unsafe extern "C" fn jump_info_max_address(mut ji: *const jump_info) -> bfd_vma {
    let mut max_address: bfd_vma = (*ji).end;
    let mut i: size_t = 0;
    i = (*ji).start.count;
    loop {
        let fresh10 = i;
        i = i.wrapping_sub(1);
        if !(fresh10 > 0 as libc::c_int as libc::c_ulong) {
            break;
        }
        if *((*ji).start.addresses).offset(i as isize) > max_address {
            max_address = *((*ji).start.addresses).offset(i as isize);
        }
    }
    return max_address;
}
unsafe extern "C" fn jump_info_end_address(mut ji: *const jump_info) -> bfd_vma {
    return (*ji).end;
}
unsafe extern "C" fn jump_info_is_start_address(
    mut ji: *const jump_info,
    mut address: bfd_vma,
) -> bool {
    let mut result: bool = 0 as libc::c_int != 0;
    let mut i: size_t = 0;
    i = (*ji).start.count;
    loop {
        let fresh11 = i;
        i = i.wrapping_sub(1);
        if !(fresh11 > 0 as libc::c_int as libc::c_ulong) {
            break;
        }
        if !(address == *((*ji).start.addresses).offset(i as isize)) {
            continue;
        }
        result = 1 as libc::c_int != 0;
        break;
    }
    return result;
}
unsafe extern "C" fn jump_info_is_end_address(
    mut ji: *const jump_info,
    mut address: bfd_vma,
) -> bool {
    return address == (*ji).end;
}
unsafe extern "C" fn jump_info_size(mut ji: *const jump_info) -> bfd_vma {
    return (jump_info_max_address(ji)).wrapping_sub(jump_info_min_address(ji));
}
unsafe extern "C" fn jump_info_unlink(
    mut node: *mut jump_info,
    mut base: *mut *mut jump_info,
) {
    if !((*node).next).is_null() {
        (*(*node).next).prev = (*node).prev;
    }
    if !((*node).prev).is_null() {
        (*(*node).prev).next = (*node).next;
    } else {
        *base = (*node).next;
    }
    (*node).next = 0 as *mut jump_info;
    (*node).prev = 0 as *mut jump_info;
}
unsafe extern "C" fn jump_info_insert(
    mut node: *mut jump_info,
    mut target: *mut jump_info,
    mut base: *mut *mut jump_info,
) {
    (*node).next = target;
    (*node).prev = (*target).prev;
    (*target).prev = node;
    if !((*node).prev).is_null() {
        (*(*node).prev).next = node;
    } else {
        *base = node;
    };
}
unsafe extern "C" fn jump_info_add_front(
    mut node: *mut jump_info,
    mut base: *mut *mut jump_info,
) {
    (*node).next = *base;
    if !((*node).next).is_null() {
        (*(*node).next).prev = node;
    }
    (*node).prev = 0 as *mut jump_info;
    *base = node;
}
unsafe extern "C" fn jump_info_move_linked(
    mut node: *mut jump_info,
    mut target: *mut jump_info,
    mut base: *mut *mut jump_info,
) {
    jump_info_unlink(node, base);
    jump_info_insert(node, target, base);
}
unsafe extern "C" fn jump_info_intersect(
    mut a: *const jump_info,
    mut b: *const jump_info,
) -> bool {
    return jump_info_max_address(a) >= jump_info_min_address(b)
        && jump_info_min_address(a) <= jump_info_max_address(b);
}
unsafe extern "C" fn jump_info_merge(mut base: *mut *mut jump_info) {
    let mut a: *mut jump_info = 0 as *mut jump_info;
    a = *base;
    while !a.is_null() {
        let mut b: *mut jump_info = 0 as *mut jump_info;
        b = (*a).next;
        while !b.is_null() {
            if (*a).end == (*b).end {
                let mut needed_size: size_t = ((*a).start.count)
                    .wrapping_add((*b).start.count);
                let mut i: size_t = 0;
                if needed_size > (*a).start.max_count {
                    (*a)
                        .start
                        .max_count = ((*a).start.max_count as libc::c_ulong)
                        .wrapping_add((*b).start.max_count) as size_t as size_t;
                    (*a)
                        .start
                        .addresses = xrealloc(
                        (*a).start.addresses as *mut libc::c_void,
                        ((*a).start.max_count)
                            .wrapping_mul(
                                ::core::mem::size_of::<*mut bfd_vma>() as libc::c_ulong,
                            ),
                    ) as *mut bfd_vma;
                }
                i = 0 as libc::c_int as size_t;
                while i < (*b).start.count {
                    let fresh12 = (*a).start.count;
                    (*a).start.count = ((*a).start.count).wrapping_add(1);
                    *((*a).start.addresses)
                        .offset(
                            fresh12 as isize,
                        ) = *((*b).start.addresses).offset(i as isize);
                    i = i.wrapping_add(1);
                    i;
                }
                let mut tmp: *mut jump_info = (*b).prev;
                jump_info_unlink(b, base);
                jump_info_free(b);
                b = tmp;
            }
            b = (*b).next;
        }
        a = (*a).next;
    }
}
unsafe extern "C" fn jump_info_sort(mut base: *mut *mut jump_info) {
    let mut current_element: *mut jump_info = *base;
    while !current_element.is_null() {
        let mut best_match: *mut jump_info = current_element;
        let mut runner: *mut jump_info = (*current_element).next;
        let mut best_size: bfd_vma = jump_info_size(best_match);
        while !runner.is_null() {
            let mut runner_size: bfd_vma = jump_info_size(runner);
            if runner_size < best_size
                || runner_size == best_size
                    && jump_info_min_address(runner) < jump_info_min_address(best_match)
            {
                best_match = runner;
                best_size = runner_size;
            }
            runner = (*runner).next;
        }
        if best_match == current_element {
            current_element = (*current_element).next;
        } else {
            jump_info_move_linked(best_match, current_element, base);
        }
    }
}
unsafe extern "C" fn jump_info_visualize_address(
    mut address: bfd_vma,
    mut max_level: libc::c_int,
    mut line_buffer: *mut libc::c_char,
    mut color_buffer: *mut uint8_t,
) {
    let mut ji: *mut jump_info = detected_jumps;
    let mut len: size_t = ((max_level + 1 as libc::c_int) * 3 as libc::c_int) as size_t;
    memset(line_buffer as *mut libc::c_void, ' ' as i32, len);
    memset(color_buffer as *mut libc::c_void, 0 as libc::c_int, len);
    while !ji.is_null() {
        if jump_info_max_address(ji) < address {
            let mut tmp: *mut jump_info = ji;
            ji = (*ji).next;
            jump_info_unlink(tmp, &mut detected_jumps);
            jump_info_free(tmp);
        } else {
            if jump_info_min_address(ji) <= address {
                let mut hash_address: bfd_vma = jump_info_end_address(ji);
                let mut color: uint8_t = iterative_hash(
                    &mut hash_address as *mut bfd_vma as *const libc::c_void,
                    ::core::mem::size_of::<bfd_vma>() as libc::c_ulong,
                    0 as libc::c_int as hashval_t,
                ) as uint8_t;
                let mut offset: libc::c_int = (max_level - (*ji).level)
                    * 3 as libc::c_int;
                if jump_info_is_start_address(ji, address) {
                    let mut i: size_t = (offset + 1 as libc::c_int) as size_t;
                    while i < len.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
                        if *line_buffer.offset(i as isize) as libc::c_int == ' ' as i32 {
                            *line_buffer.offset(i as isize) = '-' as i32 as libc::c_char;
                            *color_buffer.offset(i as isize) = color;
                        }
                        i = i.wrapping_add(1);
                        i;
                    }
                    if *line_buffer.offset(i as isize) as libc::c_int == ' ' as i32 {
                        *line_buffer.offset(i as isize) = '-' as i32 as libc::c_char;
                        *color_buffer.offset(i as isize) = color;
                    } else if *line_buffer.offset(i as isize) as libc::c_int
                        == '>' as i32
                    {
                        *line_buffer.offset(i as isize) = 'X' as i32 as libc::c_char;
                        *color_buffer.offset(i as isize) = color;
                    }
                    if *line_buffer.offset(offset as isize) as libc::c_int == ' ' as i32
                    {
                        if address <= (*ji).end {
                            *line_buffer
                                .offset(
                                    offset as isize,
                                ) = (if jump_info_min_address(ji) == address {
                                '/' as i32
                            } else {
                                '+' as i32
                            }) as libc::c_char;
                        } else {
                            *line_buffer
                                .offset(
                                    offset as isize,
                                ) = (if jump_info_max_address(ji) == address {
                                '\\' as i32
                            } else {
                                '+' as i32
                            }) as libc::c_char;
                        }
                        *color_buffer.offset(offset as isize) = color;
                    }
                } else if jump_info_is_end_address(ji, address) {
                    let mut i_0: size_t = (offset + 1 as libc::c_int) as size_t;
                    while i_0 < len.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
                        if *line_buffer.offset(i_0 as isize) as libc::c_int == ' ' as i32
                        {
                            *line_buffer
                                .offset(i_0 as isize) = '-' as i32 as libc::c_char;
                            *color_buffer.offset(i_0 as isize) = color;
                        }
                        i_0 = i_0.wrapping_add(1);
                        i_0;
                    }
                    if *line_buffer.offset(i_0 as isize) as libc::c_int == ' ' as i32 {
                        *line_buffer.offset(i_0 as isize) = '>' as i32 as libc::c_char;
                        *color_buffer.offset(i_0 as isize) = color;
                    } else if *line_buffer.offset(i_0 as isize) as libc::c_int
                        == '-' as i32
                    {
                        *line_buffer.offset(i_0 as isize) = 'X' as i32 as libc::c_char;
                        *color_buffer.offset(i_0 as isize) = color;
                    }
                    if *line_buffer.offset(offset as isize) as libc::c_int == ' ' as i32
                    {
                        if jump_info_min_address(ji) < address {
                            *line_buffer
                                .offset(
                                    offset as isize,
                                ) = (if jump_info_max_address(ji) > address {
                                '>' as i32
                            } else {
                                '\\' as i32
                            }) as libc::c_char;
                        } else {
                            *line_buffer
                                .offset(offset as isize) = '/' as i32 as libc::c_char;
                        }
                        *color_buffer.offset(offset as isize) = color;
                    }
                } else if *line_buffer.offset(offset as isize) as libc::c_int
                    == ' ' as i32
                {
                    *line_buffer.offset(offset as isize) = '|' as i32 as libc::c_char;
                    *color_buffer.offset(offset as isize) = color;
                }
            }
            ji = (*ji).next;
        }
    }
}
unsafe extern "C" fn disassemble_jumps(
    mut inf: *mut disassemble_info,
    mut disassemble_fn: disassembler_ftype,
    mut start_offset: bfd_vma,
    mut stop_offset: bfd_vma,
    mut rel_offset: bfd_vma,
    mut relppp: *mut *mut *mut arelent,
    mut relppend: *mut *mut arelent,
) -> *mut jump_info {
    let mut aux: *mut objdump_disasm_info = 0 as *mut objdump_disasm_info;
    let mut jumps: *mut jump_info = 0 as *mut jump_info;
    let mut section: *mut asection = 0 as *mut asection;
    let mut addr_offset: bfd_vma = 0;
    let mut opb: libc::c_uint = (*inf).octets_per_byte;
    let mut octets: libc::c_int = opb as libc::c_int;
    let mut sfile: SFILE = SFILE {
        buffer: 0 as *mut libc::c_char,
        pos: 0,
        alloc: 0,
    };
    aux = (*inf).application_data as *mut objdump_disasm_info;
    section = (*inf).section;
    sfile.alloc = 120 as libc::c_int as size_t;
    sfile.buffer = xmalloc(sfile.alloc) as *mut libc::c_char;
    sfile.pos = 0 as libc::c_int as size_t;
    (*inf).insn_info_valid = 0 as libc::c_int as libc::c_char;
    (*inf)
        .fprintf_func = ::core::mem::transmute::<
        Option::<
            unsafe extern "C" fn(*mut SFILE, *const libc::c_char, ...) -> libc::c_int,
        >,
        fprintf_ftype,
    >(
        Some(
            objdump_sprintf
                as unsafe extern "C" fn(
                    *mut SFILE,
                    *const libc::c_char,
                    ...
                ) -> libc::c_int,
        ),
    );
    (*inf).stream = &mut sfile as *mut SFILE as *mut libc::c_void;
    addr_offset = start_offset;
    while addr_offset < stop_offset {
        let mut previous_octets: libc::c_int = 0;
        previous_octets = octets;
        octets = 0 as libc::c_int;
        sfile.pos = 0 as libc::c_int as size_t;
        (*inf).bytes_per_line = 0 as libc::c_int;
        (*inf).bytes_per_chunk = 0 as libc::c_int;
        (*inf)
            .flags = ((if disassemble_all as libc::c_int != 0 {
            (1 as libc::c_uint) << 30 as libc::c_int
        } else {
            0 as libc::c_int as libc::c_uint
        })
            | (if wide_output != 0 {
                (1 as libc::c_uint) << 28 as libc::c_int
            } else {
                0 as libc::c_int as libc::c_uint
            })) as libc::c_ulong;
        if !machine.is_null() {
            (*inf).flags |= ((1 as libc::c_uint) << 29 as libc::c_int) as libc::c_ulong;
        }
        if (*inf).disassembler_needs_relocs as libc::c_int != 0
            && bfd_get_file_flags((*aux).abfd) & 0x2 as libc::c_int as libc::c_uint
                == 0 as libc::c_int as libc::c_uint
            && bfd_get_file_flags((*aux).abfd) & 0x40 as libc::c_int as libc::c_uint
                == 0 as libc::c_int as libc::c_uint && *relppp < relppend
        {
            let mut distance_to_rel: bfd_signed_vma = 0;
            distance_to_rel = ((***relppp).address)
                .wrapping_sub(rel_offset.wrapping_add(addr_offset)) as bfd_signed_vma;
            if distance_to_rel == 0 as libc::c_int as libc::c_long
                || distance_to_rel > 0 as libc::c_int as libc::c_long
                    && distance_to_rel
                        < (previous_octets as libc::c_uint).wrapping_div(opb)
                            as bfd_signed_vma
            {
                (*inf).flags
                    |= ((1 as libc::c_uint) << 31 as libc::c_int) as libc::c_ulong;
            }
        }
        if !disassemble_all
            && (*section).flags
                & (0x10 as libc::c_int | 0x100 as libc::c_int) as libc::c_uint
                == (0x10 as libc::c_int | 0x100 as libc::c_int) as libc::c_uint
        {
            (*inf).stop_vma = ((*section).vma).wrapping_add(stop_offset);
        }
        (*inf).stop_offset = stop_offset;
        (*inf).insn_info_valid = 0 as libc::c_int as libc::c_char;
        octets = (Some(disassemble_fn.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(((*section).vma).wrapping_add(addr_offset), inf);
        if (*inf).insn_info_valid as libc::c_int != 0
            && ((*inf).insn_type as libc::c_uint
                == dis_branch as libc::c_int as libc::c_uint
                || (*inf).insn_type as libc::c_uint
                    == dis_condbranch as libc::c_int as libc::c_uint
                || (*inf).insn_type as libc::c_uint
                    == dis_jsr as libc::c_int as libc::c_uint
                || (*inf).insn_type as libc::c_uint
                    == dis_condjsr as libc::c_int as libc::c_uint)
            && (*inf).target >= ((*section).vma).wrapping_add(start_offset)
            && (*inf).target < ((*section).vma).wrapping_add(stop_offset)
        {
            let mut ji: *mut jump_info = jump_info_new(
                ((*section).vma).wrapping_add(addr_offset),
                (*inf).target,
                -(1 as libc::c_int),
            );
            jump_info_add_front(ji, &mut jumps);
        }
        (*inf).stop_vma = 0 as libc::c_int as bfd_vma;
        addr_offset = (addr_offset as libc::c_ulong)
            .wrapping_add((octets as libc::c_uint).wrapping_div(opb) as libc::c_ulong)
            as bfd_vma as bfd_vma;
    }
    (*inf)
        .fprintf_func = ::core::mem::transmute::<
        Option::<
            unsafe extern "C" fn(*mut FILE, *const libc::c_char, ...) -> libc::c_int,
        >,
        fprintf_ftype,
    >(
        Some(
            fprintf
                as unsafe extern "C" fn(
                    *mut FILE,
                    *const libc::c_char,
                    ...
                ) -> libc::c_int,
        ),
    );
    (*inf).stream = stdout as *mut libc::c_void;
    free(sfile.buffer as *mut libc::c_void);
    jump_info_merge(&mut jumps);
    jump_info_sort(&mut jumps);
    let mut last_jump: *mut jump_info = jumps;
    let mut max_level: libc::c_int = -(1 as libc::c_int);
    while !last_jump.is_null() {
        let mut base: *mut jump_info = last_jump;
        max_level += 1;
        (*base).level = max_level;
        let mut exchange_item: *mut jump_info = (*last_jump).next;
        let mut it: *mut jump_info = exchange_item;
        while !it.is_null() {
            let mut ok: bool = 1 as libc::c_int != 0;
            let mut it_collision: *mut jump_info = 0 as *mut jump_info;
            it_collision = base;
            while it_collision != exchange_item {
                if jump_info_intersect(it_collision, it) {
                    ok = 0 as libc::c_int != 0;
                    break;
                } else {
                    it_collision = (*it_collision).next;
                }
            }
            if ok {
                if it != exchange_item {
                    let mut save: *mut jump_info = (*it).prev;
                    jump_info_move_linked(it, exchange_item, &mut jumps);
                    last_jump = it;
                    it = save;
                } else {
                    last_jump = exchange_item;
                    exchange_item = (*exchange_item).next;
                }
                (*last_jump).level = max_level;
            }
            it = (*it).next;
        }
        last_jump = exchange_item;
    }
    return jumps;
}
unsafe extern "C" fn null_print(
    mut _stream: *const libc::c_void,
    mut _format: *const libc::c_char,
    mut _args: ...
) -> libc::c_int {
    return 1 as libc::c_int;
}
unsafe extern "C" fn print_jump_visualisation(
    mut addr: bfd_vma,
    mut max_level: libc::c_int,
    mut line_buffer: *mut libc::c_char,
    mut color_buffer: *mut uint8_t,
) {
    if line_buffer.is_null() {
        return;
    }
    jump_info_visualize_address(addr, max_level, line_buffer, color_buffer);
    let mut line_buffer_size: size_t = strlen(line_buffer);
    let mut last_color: libc::c_char = 0 as libc::c_int as libc::c_char;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i <= line_buffer_size {
        if color_output {
            let mut color: uint8_t = (if i < line_buffer_size {
                *color_buffer.offset(i as isize) as libc::c_int
            } else {
                0 as libc::c_int
            }) as uint8_t;
            if color as libc::c_int != last_color as libc::c_int {
                if color != 0 {
                    if extended_color_output {
                        printf(
                            b"\x1B[38;5;%dm\0" as *const u8 as *const libc::c_char,
                            124 as libc::c_int
                                + color as libc::c_int % 108 as libc::c_int,
                        );
                    } else {
                        printf(
                            b"\x1B[%dm\0" as *const u8 as *const libc::c_char,
                            31 as libc::c_int + color as libc::c_int % 7 as libc::c_int,
                        );
                    }
                } else {
                    printf(b"\x1B[0m\0" as *const u8 as *const libc::c_char);
                }
                last_color = color as libc::c_char;
            }
        }
        putchar(
            if i < line_buffer_size {
                *line_buffer.offset(i as isize) as libc::c_int
            } else {
                ' ' as i32
            },
        );
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn disassemble_bytes(
    mut inf: *mut disassemble_info,
    mut disassemble_fn: disassembler_ftype,
    mut insns: bool,
    mut data: *mut bfd_byte,
    mut start_offset: bfd_vma,
    mut stop_offset: bfd_vma,
    mut rel_offset: bfd_vma,
    mut relppp: *mut *mut *mut arelent,
    mut relppend: *mut *mut arelent,
) {
    let mut aux: *mut objdump_disasm_info = 0 as *mut objdump_disasm_info;
    let mut section: *mut asection = 0 as *mut asection;
    let mut octets_per_line: libc::c_uint = 0;
    let mut skip_addr_chars: libc::c_uint = 0;
    let mut addr_offset: bfd_vma = 0;
    let mut opb: libc::c_uint = (*inf).octets_per_byte;
    let mut skip_zeroes: libc::c_uint = (*inf).skip_zeroes;
    let mut skip_zeroes_at_end: libc::c_uint = (*inf).skip_zeroes_at_end;
    let mut octets: size_t = 0;
    let mut sfile: SFILE = SFILE {
        buffer: 0 as *mut libc::c_char,
        pos: 0,
        alloc: 0,
    };
    aux = (*inf).application_data as *mut objdump_disasm_info;
    section = (*inf).section;
    sfile.alloc = 120 as libc::c_int as size_t;
    sfile.buffer = xmalloc(sfile.alloc) as *mut libc::c_char;
    sfile.pos = 0 as libc::c_int as size_t;
    if insn_width != 0 {
        octets_per_line = insn_width as libc::c_uint;
    } else if insns {
        octets_per_line = 4 as libc::c_int as libc::c_uint;
    } else {
        octets_per_line = 16 as libc::c_int as libc::c_uint;
    }
    skip_addr_chars = 0 as libc::c_int as libc::c_uint;
    if no_addresses == 0 && prefix_addresses == 0 {
        let mut buf: [libc::c_char; 30] = [0; 30];
        bfd_sprintf_vma(
            (*aux).abfd,
            buf.as_mut_ptr(),
            ((*section).vma)
                .wrapping_add(((*section).size).wrapping_div(opb as libc::c_ulong)),
        );
        while buf[skip_addr_chars as usize] as libc::c_int == '0' as i32 {
            skip_addr_chars = skip_addr_chars.wrapping_add(1);
            skip_addr_chars;
        }
        if buf[skip_addr_chars as usize] as libc::c_int == '\0' as i32
            && (*section).vma != 0 as libc::c_int as libc::c_ulong
        {
            skip_addr_chars = 0 as libc::c_int as libc::c_uint;
        }
        if skip_addr_chars != 0 as libc::c_int as libc::c_uint {
            skip_addr_chars = skip_addr_chars
                .wrapping_sub(1 as libc::c_int as libc::c_uint)
                & -(4 as libc::c_int) as libc::c_uint;
        }
    }
    (*inf).insn_info_valid = 0 as libc::c_int as libc::c_char;
    let mut color_buffer: *mut uint8_t = 0 as *mut uint8_t;
    let mut line_buffer: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut max_level: libc::c_int = -(1 as libc::c_int);
    if !detected_jumps.is_null() {
        let mut ji: *mut jump_info = 0 as *mut jump_info;
        ji = detected_jumps;
        while !ji.is_null() {
            if (*ji).level > max_level {
                max_level = (*ji).level;
            }
            ji = (*ji).next;
        }
        let mut len: size_t = ((max_level + 1 as libc::c_int) * 3 as libc::c_int
            + 1 as libc::c_int) as size_t;
        line_buffer = xmalloc(len) as *mut libc::c_char;
        *line_buffer
            .offset(
                len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
            ) = 0 as libc::c_int as libc::c_char;
        color_buffer = xmalloc(len) as *mut uint8_t;
        *color_buffer
            .offset(
                len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
            ) = 0 as libc::c_int as uint8_t;
    }
    addr_offset = start_offset;
    while addr_offset < stop_offset {
        let mut need_nl: bool = 0 as libc::c_int != 0;
        octets = 0 as libc::c_int as size_t;
        (*aux).reloc = 0 as *mut arelent;
        if disassemble_zeroes == 0 {
            while addr_offset.wrapping_mul(opb as libc::c_ulong).wrapping_add(octets)
                < stop_offset.wrapping_mul(opb as libc::c_ulong)
            {
                if *data
                    .offset(
                        addr_offset
                            .wrapping_mul(opb as libc::c_ulong)
                            .wrapping_add(octets) as isize,
                    ) as libc::c_int != 0 as libc::c_int
                {
                    break;
                }
                octets = octets.wrapping_add(1);
                octets;
            }
        }
        if disassemble_zeroes == 0
            && ((*inf).insn_info_valid as libc::c_int == 0 as libc::c_int
                || (*inf).branch_delay_insns as libc::c_int == 0 as libc::c_int)
            && (octets >= skip_zeroes as libc::c_ulong
                || addr_offset.wrapping_mul(opb as libc::c_ulong).wrapping_add(octets)
                    == stop_offset.wrapping_mul(opb as libc::c_ulong)
                    && octets < skip_zeroes_at_end as libc::c_ulong)
        {
            if addr_offset.wrapping_mul(opb as libc::c_ulong).wrapping_add(octets)
                != stop_offset.wrapping_mul(opb as libc::c_ulong)
            {
                octets &= !(3 as libc::c_int) as libc::c_ulong;
            }
            if display_file_offsets as libc::c_int != 0
                && addr_offset.wrapping_add(octets.wrapping_div(opb as libc::c_ulong))
                    < stop_offset
            {
                printf(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"\t... (skipping %lu zeroes, resuming at file offset: 0x%lx)\n\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    octets.wrapping_div(opb as libc::c_ulong),
                    ((*section).filepos as libc::c_ulong)
                        .wrapping_add(addr_offset)
                        .wrapping_add(octets.wrapping_div(opb as libc::c_ulong)),
                );
            } else {
                printf(b"\t...\n\0" as *const u8 as *const libc::c_char);
            }
        } else {
            let mut buf_0: [libc::c_char; 50] = [0; 50];
            let mut bpc: libc::c_uint = 0 as libc::c_int as libc::c_uint;
            let mut pb: libc::c_uint = 0 as libc::c_int as libc::c_uint;
            if with_line_numbers != 0 || with_source_code as libc::c_int != 0 {
                show_line((*aux).abfd, section, addr_offset);
            }
            if no_addresses != 0 {
                printf(b"\t\0" as *const u8 as *const libc::c_char);
            } else if prefix_addresses == 0 {
                let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
                bfd_sprintf_vma(
                    (*aux).abfd,
                    buf_0.as_mut_ptr(),
                    ((*section).vma).wrapping_add(addr_offset),
                );
                s = buf_0.as_mut_ptr().offset(skip_addr_chars as isize);
                while *s as libc::c_int == '0' as i32 {
                    *s = ' ' as i32 as libc::c_char;
                    s = s.offset(1);
                    s;
                }
                if *s as libc::c_int == '\0' as i32 {
                    s = s.offset(-1);
                    *s = '0' as i32 as libc::c_char;
                }
                printf(
                    b"%s:\t\0" as *const u8 as *const libc::c_char,
                    buf_0.as_mut_ptr().offset(skip_addr_chars as isize),
                );
            } else {
                (*aux).require_sec = 1 as libc::c_int != 0;
                objdump_print_address(((*section).vma).wrapping_add(addr_offset), inf);
                (*aux).require_sec = 0 as libc::c_int != 0;
                putchar(' ' as i32);
            }
            print_jump_visualisation(
                ((*section).vma).wrapping_add(addr_offset),
                max_level,
                line_buffer,
                color_buffer,
            );
            if insns {
                let mut insn_size: libc::c_int = 0;
                sfile.pos = 0 as libc::c_int as size_t;
                (*inf)
                    .fprintf_func = ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut SFILE,
                            *const libc::c_char,
                            ...
                        ) -> libc::c_int,
                    >,
                    fprintf_ftype,
                >(
                    Some(
                        objdump_sprintf
                            as unsafe extern "C" fn(
                                *mut SFILE,
                                *const libc::c_char,
                                ...
                            ) -> libc::c_int,
                    ),
                );
                (*inf).stream = &mut sfile as *mut SFILE as *mut libc::c_void;
                (*inf).bytes_per_line = 0 as libc::c_int;
                (*inf).bytes_per_chunk = 0 as libc::c_int;
                (*inf)
                    .flags = ((if disassemble_all as libc::c_int != 0 {
                    (1 as libc::c_uint) << 30 as libc::c_int
                } else {
                    0 as libc::c_int as libc::c_uint
                })
                    | (if wide_output != 0 {
                        (1 as libc::c_uint) << 28 as libc::c_int
                    } else {
                        0 as libc::c_int as libc::c_uint
                    })) as libc::c_ulong;
                if !machine.is_null() {
                    (*inf).flags
                        |= ((1 as libc::c_uint) << 29 as libc::c_int) as libc::c_ulong;
                }
                if (*inf).disassembler_needs_relocs as libc::c_int != 0
                    && bfd_get_file_flags((*aux).abfd)
                        & 0x2 as libc::c_int as libc::c_uint
                        == 0 as libc::c_int as libc::c_uint
                    && bfd_get_file_flags((*aux).abfd)
                        & 0x40 as libc::c_int as libc::c_uint
                        == 0 as libc::c_int as libc::c_uint && *relppp < relppend
                {
                    let mut distance_to_rel: bfd_signed_vma = 0;
                    let mut max_reloc_offset: libc::c_int = (*(*(*aux).abfd).arch_info)
                        .max_reloc_offset_into_insn;
                    distance_to_rel = ((***relppp).address)
                        .wrapping_sub(rel_offset)
                        .wrapping_sub(addr_offset) as bfd_signed_vma;
                    insn_size = 0 as libc::c_int;
                    if distance_to_rel > 0 as libc::c_int as libc::c_long
                        && (max_reloc_offset < 0 as libc::c_int
                            || distance_to_rel <= max_reloc_offset as libc::c_long)
                    {
                        if insn_width != 0 {
                            insn_size = insn_width;
                        } else {
                            (*inf)
                                .fprintf_func = ::core::mem::transmute::<
                                Option::<
                                    unsafe extern "C" fn(
                                        *const libc::c_void,
                                        *const libc::c_char,
                                        ...
                                    ) -> libc::c_int,
                                >,
                                fprintf_ftype,
                            >(
                                Some(
                                    null_print
                                        as unsafe extern "C" fn(
                                            *const libc::c_void,
                                            *const libc::c_char,
                                            ...
                                        ) -> libc::c_int,
                                ),
                            );
                            insn_size = disassemble_fn
                                .expect(
                                    "non-null function pointer",
                                )(((*section).vma).wrapping_add(addr_offset), inf);
                            (*inf)
                                .fprintf_func = ::core::mem::transmute::<
                                Option::<
                                    unsafe extern "C" fn(
                                        *mut SFILE,
                                        *const libc::c_char,
                                        ...
                                    ) -> libc::c_int,
                                >,
                                fprintf_ftype,
                            >(
                                Some(
                                    objdump_sprintf
                                        as unsafe extern "C" fn(
                                            *mut SFILE,
                                            *const libc::c_char,
                                            ...
                                        ) -> libc::c_int,
                                ),
                            );
                        }
                    }
                    if distance_to_rel == 0 as libc::c_int as libc::c_long
                        || distance_to_rel > 0 as libc::c_int as libc::c_long
                            && distance_to_rel
                                < (insn_size / opb as libc::c_int) as libc::c_long
                    {
                        (*inf).flags
                            |= ((1 as libc::c_uint) << 31 as libc::c_int)
                                as libc::c_ulong;
                        (*aux).reloc = **relppp;
                    }
                }
                if !disassemble_all
                    && (*section).flags
                        & (0x10 as libc::c_int | 0x100 as libc::c_int) as libc::c_uint
                        == (0x10 as libc::c_int | 0x100 as libc::c_int) as libc::c_uint
                {
                    (*inf).stop_vma = ((*section).vma).wrapping_add(stop_offset);
                }
                (*inf).stop_offset = stop_offset;
                insn_size = (Some(disassemble_fn.expect("non-null function pointer")))
                    .expect(
                        "non-null function pointer",
                    )(((*section).vma).wrapping_add(addr_offset), inf);
                octets = insn_size as size_t;
                (*inf).stop_vma = 0 as libc::c_int as bfd_vma;
                (*inf)
                    .fprintf_func = ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut FILE,
                            *const libc::c_char,
                            ...
                        ) -> libc::c_int,
                    >,
                    fprintf_ftype,
                >(
                    Some(
                        fprintf
                            as unsafe extern "C" fn(
                                *mut FILE,
                                *const libc::c_char,
                                ...
                            ) -> libc::c_int,
                    ),
                );
                (*inf).stream = stdout as *mut libc::c_void;
                if insn_width == 0 as libc::c_int
                    && (*inf).bytes_per_line != 0 as libc::c_int
                {
                    octets_per_line = (*inf).bytes_per_line as libc::c_uint;
                }
                if insn_size < opb as libc::c_int {
                    if sfile.pos != 0 {
                        printf(
                            b"%s\n\0" as *const u8 as *const libc::c_char,
                            sfile.buffer,
                        );
                    }
                    if insn_size >= 0 as libc::c_int {
                        non_fatal(
                            dcgettext(
                                0 as *const libc::c_char,
                                b"disassemble_fn returned length %d\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            insn_size,
                        );
                        exit_status = 1 as libc::c_int;
                    }
                    break;
                }
            } else {
                let mut j: bfd_vma = 0;
                octets = octets_per_line as size_t;
                if addr_offset.wrapping_add(octets.wrapping_div(opb as libc::c_ulong))
                    > stop_offset
                {
                    octets = stop_offset
                        .wrapping_sub(addr_offset)
                        .wrapping_mul(opb as libc::c_ulong);
                }
                j = addr_offset.wrapping_mul(opb as libc::c_ulong);
                while j
                    < addr_offset.wrapping_mul(opb as libc::c_ulong).wrapping_add(octets)
                {
                    if _sch_istable[(*data.offset(j as isize) as libc::c_int
                        & 0xff as libc::c_int) as usize] as libc::c_int
                        & _sch_isprint as libc::c_int as libc::c_ushort as libc::c_int
                        != 0
                    {
                        buf_0[j
                            .wrapping_sub(addr_offset.wrapping_mul(opb as libc::c_ulong))
                            as usize] = *data.offset(j as isize) as libc::c_char;
                    } else {
                        buf_0[j
                            .wrapping_sub(addr_offset.wrapping_mul(opb as libc::c_ulong))
                            as usize] = '.' as i32 as libc::c_char;
                    }
                    j = j.wrapping_add(1);
                    j;
                }
                buf_0[j.wrapping_sub(addr_offset.wrapping_mul(opb as libc::c_ulong))
                    as usize] = '\0' as i32 as libc::c_char;
            }
            if if prefix_addresses != 0 {
                (show_raw_insn > 0 as libc::c_int) as libc::c_int
            } else {
                (show_raw_insn >= 0 as libc::c_int) as libc::c_int
            } != 0
            {
                let mut j_0: bfd_vma = 0;
                pb = octets as libc::c_uint;
                if pb > octets_per_line && prefix_addresses == 0 && wide_output == 0 {
                    pb = octets_per_line;
                }
                if (*inf).bytes_per_chunk != 0 {
                    bpc = (*inf).bytes_per_chunk as libc::c_uint;
                } else {
                    bpc = 1 as libc::c_int as libc::c_uint;
                }
                j_0 = addr_offset.wrapping_mul(opb as libc::c_ulong);
                while j_0
                    < addr_offset
                        .wrapping_mul(opb as libc::c_ulong)
                        .wrapping_add(pb as libc::c_ulong)
                {
                    if j_0.wrapping_add(bpc as libc::c_ulong)
                        <= stop_offset.wrapping_mul(opb as libc::c_ulong)
                    {
                        let mut k: libc::c_uint = 0;
                        if (*inf).display_endian as libc::c_uint
                            == BFD_ENDIAN_LITTLE as libc::c_int as libc::c_uint
                        {
                            k = bpc;
                            loop {
                                let fresh13 = k;
                                k = k.wrapping_sub(1);
                                if !(fresh13 != 0 as libc::c_int as libc::c_uint) {
                                    break;
                                }
                                printf(
                                    b"%02x\0" as *const u8 as *const libc::c_char,
                                    *data.offset(j_0.wrapping_add(k as libc::c_ulong) as isize)
                                        as libc::c_uint,
                                );
                            }
                        } else {
                            k = 0 as libc::c_int as libc::c_uint;
                            while k < bpc {
                                printf(
                                    b"%02x\0" as *const u8 as *const libc::c_char,
                                    *data.offset(j_0.wrapping_add(k as libc::c_ulong) as isize)
                                        as libc::c_uint,
                                );
                                k = k.wrapping_add(1);
                                k;
                            }
                        }
                    }
                    putchar(' ' as i32);
                    j_0 = (j_0 as libc::c_ulong).wrapping_add(bpc as libc::c_ulong)
                        as bfd_vma as bfd_vma;
                }
                while pb < octets_per_line {
                    let mut k_0: libc::c_uint = 0;
                    k_0 = 0 as libc::c_int as libc::c_uint;
                    while k_0 < bpc {
                        printf(b"  \0" as *const u8 as *const libc::c_char);
                        k_0 = k_0.wrapping_add(1);
                        k_0;
                    }
                    putchar(' ' as i32);
                    pb = pb.wrapping_add(bpc);
                }
                if insns {
                    putchar('\t' as i32);
                } else {
                    printf(b"    \0" as *const u8 as *const libc::c_char);
                }
            }
            if !insns {
                printf(b"%s\0" as *const u8 as *const libc::c_char, buf_0.as_mut_ptr());
            } else if sfile.pos != 0 {
                printf(b"%s\0" as *const u8 as *const libc::c_char, sfile.buffer);
            }
            if if prefix_addresses != 0 {
                (show_raw_insn > 0 as libc::c_int) as libc::c_int
            } else {
                (show_raw_insn >= 0 as libc::c_int) as libc::c_int
            } != 0
            {
                while (pb as libc::c_ulong) < octets {
                    let mut j_1: bfd_vma = 0;
                    let mut s_0: *mut libc::c_char = 0 as *mut libc::c_char;
                    putchar('\n' as i32);
                    j_1 = addr_offset
                        .wrapping_mul(opb as libc::c_ulong)
                        .wrapping_add(pb as libc::c_ulong);
                    if no_addresses != 0 {
                        printf(b"\t\0" as *const u8 as *const libc::c_char);
                    } else {
                        bfd_sprintf_vma(
                            (*aux).abfd,
                            buf_0.as_mut_ptr(),
                            ((*section).vma)
                                .wrapping_add(j_1.wrapping_div(opb as libc::c_ulong)),
                        );
                        s_0 = buf_0.as_mut_ptr().offset(skip_addr_chars as isize);
                        while *s_0 as libc::c_int == '0' as i32 {
                            *s_0 = ' ' as i32 as libc::c_char;
                            s_0 = s_0.offset(1);
                            s_0;
                        }
                        if *s_0 as libc::c_int == '\0' as i32 {
                            s_0 = s_0.offset(-1);
                            *s_0 = '0' as i32 as libc::c_char;
                        }
                        printf(
                            b"%s:\t\0" as *const u8 as *const libc::c_char,
                            buf_0.as_mut_ptr().offset(skip_addr_chars as isize),
                        );
                    }
                    print_jump_visualisation(
                        ((*section).vma)
                            .wrapping_add(j_1.wrapping_div(opb as libc::c_ulong)),
                        max_level,
                        line_buffer,
                        color_buffer,
                    );
                    pb = pb.wrapping_add(octets_per_line);
                    if pb as libc::c_ulong > octets {
                        pb = octets as libc::c_uint;
                    }
                    while j_1
                        < addr_offset
                            .wrapping_mul(opb as libc::c_ulong)
                            .wrapping_add(pb as libc::c_ulong)
                    {
                        if j_1.wrapping_add(bpc as libc::c_ulong)
                            <= stop_offset.wrapping_mul(opb as libc::c_ulong)
                        {
                            let mut k_1: libc::c_uint = 0;
                            if (*inf).display_endian as libc::c_uint
                                == BFD_ENDIAN_LITTLE as libc::c_int as libc::c_uint
                            {
                                k_1 = bpc;
                                loop {
                                    let fresh14 = k_1;
                                    k_1 = k_1.wrapping_sub(1);
                                    if !(fresh14 != 0 as libc::c_int as libc::c_uint) {
                                        break;
                                    }
                                    printf(
                                        b"%02x\0" as *const u8 as *const libc::c_char,
                                        *data
                                            .offset(j_1.wrapping_add(k_1 as libc::c_ulong) as isize)
                                            as libc::c_uint,
                                    );
                                }
                            } else {
                                k_1 = 0 as libc::c_int as libc::c_uint;
                                while k_1 < bpc {
                                    printf(
                                        b"%02x\0" as *const u8 as *const libc::c_char,
                                        *data
                                            .offset(j_1.wrapping_add(k_1 as libc::c_ulong) as isize)
                                            as libc::c_uint,
                                    );
                                    k_1 = k_1.wrapping_add(1);
                                    k_1;
                                }
                            }
                        }
                        putchar(' ' as i32);
                        j_1 = (j_1 as libc::c_ulong).wrapping_add(bpc as libc::c_ulong)
                            as bfd_vma as bfd_vma;
                    }
                }
            }
            if wide_output == 0 {
                putchar('\n' as i32);
            } else {
                need_nl = 1 as libc::c_int != 0;
            }
        }
        while *relppp < relppend
            && (***relppp).address
                < rel_offset
                    .wrapping_add(addr_offset)
                    .wrapping_add(octets.wrapping_div(opb as libc::c_ulong))
        {
            if dump_reloc_info != 0 || dump_dynamic_reloc_info != 0 {
                let mut q: *mut arelent = 0 as *mut arelent;
                q = **relppp;
                if wide_output != 0 {
                    putchar('\t' as i32);
                } else {
                    printf(b"\t\t\t\0" as *const u8 as *const libc::c_char);
                }
                if no_addresses == 0 {
                    objdump_print_value(
                        ((*section).vma)
                            .wrapping_sub(rel_offset)
                            .wrapping_add((*q).address),
                        inf,
                        1 as libc::c_int != 0,
                    );
                    printf(b": \0" as *const u8 as *const libc::c_char);
                }
                if ((*q).howto).is_null() {
                    printf(b"*unknown*\t\0" as *const u8 as *const libc::c_char);
                } else if !((*(*q).howto).name).is_null() {
                    printf(
                        b"%s\t\0" as *const u8 as *const libc::c_char,
                        (*(*q).howto).name,
                    );
                } else {
                    printf(
                        b"%d\t\0" as *const u8 as *const libc::c_char,
                        (*(*q).howto).type_0,
                    );
                }
                if ((*q).sym_ptr_ptr).is_null() || (*(*q).sym_ptr_ptr).is_null() {
                    printf(b"*unknown*\0" as *const u8 as *const libc::c_char);
                } else {
                    let mut sym_name: *const libc::c_char = 0 as *const libc::c_char;
                    sym_name = bfd_asymbol_name(*(*q).sym_ptr_ptr);
                    if !sym_name.is_null() && *sym_name as libc::c_int != '\0' as i32 {
                        objdump_print_symname((*aux).abfd, inf, *(*q).sym_ptr_ptr);
                    } else {
                        let mut sym_sec: *mut asection = 0 as *mut asection;
                        sym_sec = bfd_asymbol_section(*(*q).sym_ptr_ptr);
                        sym_name = bfd_section_name(sym_sec);
                        if sym_name.is_null() || *sym_name as libc::c_int == '\0' as i32
                        {
                            sym_name = b"*unknown*\0" as *const u8
                                as *const libc::c_char;
                        }
                        printf(
                            b"%s\0" as *const u8 as *const libc::c_char,
                            sanitize_string(sym_name),
                        );
                    }
                }
                if (*q).addend != 0 {
                    let mut addend: bfd_vma = (*q).addend;
                    if (addend as bfd_signed_vma) < 0 as libc::c_int as libc::c_long {
                        printf(b"-0x\0" as *const u8 as *const libc::c_char);
                        addend = addend.wrapping_neg();
                    } else {
                        printf(b"+0x\0" as *const u8 as *const libc::c_char);
                    }
                    objdump_print_value(addend, inf, 1 as libc::c_int != 0);
                }
                printf(b"\n\0" as *const u8 as *const libc::c_char);
                need_nl = 0 as libc::c_int != 0;
            }
            *relppp = (*relppp).offset(1);
            let _ = *relppp;
        }
        if need_nl {
            printf(b"\n\0" as *const u8 as *const libc::c_char);
        }
        addr_offset = (addr_offset as libc::c_ulong)
            .wrapping_add(octets.wrapping_div(opb as libc::c_ulong)) as bfd_vma
            as bfd_vma;
    }
    free(sfile.buffer as *mut libc::c_void);
    free(line_buffer as *mut libc::c_void);
    free(color_buffer as *mut libc::c_void);
}
unsafe extern "C" fn disassemble_section(
    mut abfd: *mut bfd,
    mut section: *mut asection,
    mut inf: *mut libc::c_void,
) {
    let mut bed: *const elf_backend_data = 0 as *const elf_backend_data;
    let mut sign_adjust: bfd_vma = 0 as libc::c_int as bfd_vma;
    let mut pinfo: *mut disassemble_info = inf as *mut disassemble_info;
    let mut paux: *mut objdump_disasm_info = 0 as *mut objdump_disasm_info;
    let mut opb: libc::c_uint = (*pinfo).octets_per_byte;
    let mut data: *mut bfd_byte = 0 as *mut bfd_byte;
    let mut datasize: bfd_size_type = 0 as libc::c_int as bfd_size_type;
    let mut rel_pp: *mut *mut arelent = 0 as *mut *mut arelent;
    let mut rel_ppstart: *mut *mut arelent = 0 as *mut *mut arelent;
    let mut rel_ppend: *mut *mut arelent = 0 as *mut *mut arelent;
    let mut stop_offset: bfd_vma = 0;
    let mut sym: *mut asymbol = 0 as *mut asymbol;
    let mut place: libc::c_long = 0 as libc::c_int as libc::c_long;
    let mut rel_count: libc::c_long = 0;
    let mut rel_offset: bfd_vma = 0;
    let mut addr_offset: libc::c_ulong = 0;
    let mut do_print: bool = false;
    let mut loop_until: loop_control = stop_offset_reached;
    if !disassemble_all && only_list.is_null()
        && (*section).flags
            & (0x10 as libc::c_int | 0x100 as libc::c_int) as libc::c_uint
            != (0x10 as libc::c_int | 0x100 as libc::c_int) as libc::c_uint
    {
        return;
    }
    if !process_section_p(section) {
        return;
    }
    datasize = bfd_section_size(section);
    if datasize == 0 as libc::c_int as libc::c_ulong {
        return;
    }
    if start_address == -(1 as libc::c_int) as bfd_vma || start_address < (*section).vma
    {
        addr_offset = 0 as libc::c_int as libc::c_ulong;
    } else {
        addr_offset = start_address.wrapping_sub((*section).vma);
    }
    if stop_address == -(1 as libc::c_int) as bfd_vma {
        stop_offset = datasize.wrapping_div(opb as libc::c_ulong);
    } else {
        if stop_address < (*section).vma {
            stop_offset = 0 as libc::c_int as bfd_vma;
        } else {
            stop_offset = stop_address.wrapping_sub((*section).vma);
        }
        if stop_offset > datasize.wrapping_div(opb as libc::c_ulong) {
            stop_offset = datasize.wrapping_div(opb as libc::c_ulong);
        }
    }
    if addr_offset >= stop_offset {
        return;
    }
    paux = (*pinfo).application_data as *mut objdump_disasm_info;
    if !((*pinfo).dynrelbuf).is_null() && dump_dynamic_reloc_info != 0 {
        rel_pp = (*pinfo).dynrelbuf;
        rel_count = (*pinfo).dynrelcount;
        rel_offset = (*section).vma;
    } else {
        rel_count = 0 as libc::c_int as libc::c_long;
        rel_pp = 0 as *mut *mut arelent;
        rel_offset = 0 as libc::c_int as bfd_vma;
        if (*section).flags & 0x4 as libc::c_int as libc::c_uint
            != 0 as libc::c_int as libc::c_uint
            && (dump_reloc_info != 0
                || (*pinfo).disassembler_needs_relocs as libc::c_int != 0)
        {
            let mut relsize: libc::c_long = 0;
            relsize = bfd_get_reloc_upper_bound(abfd, section);
            if relsize < 0 as libc::c_int as libc::c_long {
                bfd_fatal(bfd_get_filename(abfd));
            }
            if relsize > 0 as libc::c_int as libc::c_long {
                rel_pp = xmalloc(relsize as size_t) as *mut *mut arelent;
                rel_ppstart = rel_pp;
                rel_count = bfd_canonicalize_reloc(abfd, section, rel_pp, syms);
                if rel_count < 0 as libc::c_int as libc::c_long {
                    bfd_fatal(bfd_get_filename(abfd));
                }
                qsort(
                    rel_pp as *mut libc::c_void,
                    rel_count as size_t,
                    ::core::mem::size_of::<*mut arelent>() as libc::c_ulong,
                    Some(
                        compare_relocs
                            as unsafe extern "C" fn(
                                *const libc::c_void,
                                *const libc::c_void,
                            ) -> libc::c_int,
                    ),
                );
            }
        }
    }
    rel_ppend = rel_pp.offset(rel_count as isize);
    if !bfd_malloc_and_get_section(abfd, section, &mut data) {
        non_fatal(
            dcgettext(
                0 as *const libc::c_char,
                b"Reading section %s failed because: %s\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            (*section).name,
            bfd_errmsg(bfd_get_error()),
        );
        return;
    }
    (*pinfo).buffer = data;
    (*pinfo).buffer_vma = (*section).vma;
    (*pinfo).buffer_length = datasize;
    (*pinfo).section = section;
    compare_section = section;
    if sorted_symcount > 1 as libc::c_int as libc::c_long {
        qsort(
            sorted_syms as *mut libc::c_void,
            sorted_symcount as size_t,
            ::core::mem::size_of::<*mut asymbol>() as libc::c_ulong,
            Some(
                compare_symbols
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
        );
    }
    while rel_pp < rel_ppend && (**rel_pp).address < rel_offset.wrapping_add(addr_offset)
    {
        rel_pp = rel_pp.offset(1);
        rel_pp;
    }
    printf(
        dcgettext(
            0 as *const libc::c_char,
            b"\nDisassembly of section %s:\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        sanitize_string((*section).name),
    );
    (*paux).require_sec = 1 as libc::c_int != 0;
    sym = find_symbol_for_address(
        ((*section).vma).wrapping_add(addr_offset),
        inf as *mut disassemble_info,
        &mut place,
    );
    (*paux).require_sec = 0 as libc::c_int != 0;
    if bfd_get_flavour(abfd) as libc::c_uint
        == bfd_target_elf_flavour as libc::c_int as libc::c_uint
        && {
            bed = (*(*abfd).xvec).backend_data as *const elf_backend_data;
            !bed.is_null()
        } && (*bed).sign_extend_vma() as libc::c_int != 0
    {
        sign_adjust = (1 as libc::c_int as bfd_vma)
            << (*(*bed).s).arch_size as libc::c_int - 1 as libc::c_int;
    }
    do_print = ((*paux).symbol).is_null();
    loop_until = stop_offset_reached;
    while addr_offset < stop_offset {
        let mut addr: bfd_vma = 0;
        let mut nextsym: *mut asymbol = 0 as *mut asymbol;
        let mut nextstop_offset: bfd_vma = 0;
        let mut insns: bool = false;
        addr = ((*section).vma).wrapping_add(addr_offset);
        addr = (addr
            & (sign_adjust << 1 as libc::c_int)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) ^ sign_adjust)
            .wrapping_sub(sign_adjust);
        if !sym.is_null() && bfd_asymbol_value(sym) <= addr {
            let mut x: libc::c_int = 0;
            x = place as libc::c_int;
            while (x as libc::c_long) < sorted_symcount
                && bfd_asymbol_value(*sorted_syms.offset(x as isize)) <= addr
            {
                x += 1;
                x;
            }
            (*pinfo).symbols = sorted_syms.offset(place as isize);
            (*pinfo).num_symbols = (x as libc::c_long - place) as libc::c_int;
            (*pinfo).symtab_pos = place as libc::c_int;
        } else {
            (*pinfo).symbols = 0 as *mut *mut asymbol;
            (*pinfo).num_symbols = 0 as libc::c_int;
            (*pinfo).symtab_pos = -(1 as libc::c_int);
        }
        if !sym.is_null() && !((*paux).symbol).is_null() {
            if do_print {
                match loop_until as libc::c_uint {
                    1 => {
                        if (*sym).flags
                            & ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_uint
                            != 0
                        {
                            do_print = 0 as libc::c_int != 0;
                        }
                    }
                    2 => {
                        if !bfd_is_local_label(abfd, sym) {
                            do_print = 0 as libc::c_int != 0;
                        }
                    }
                    0 | _ => {}
                }
            } else {
                let mut name: *const libc::c_char = bfd_asymbol_name(sym);
                let mut alloc: *mut libc::c_char = 0 as *mut libc::c_char;
                if do_demangle != 0
                    && *name.offset(0 as libc::c_int as isize) as libc::c_int
                        != '\0' as i32
                {
                    alloc = bfd_demangle(abfd, name, demangle_flags);
                    if !alloc.is_null() {
                        name = alloc;
                    }
                }
                if strcmp(name, (*paux).symbol) == 0 as libc::c_int {
                    do_print = 1 as libc::c_int != 0;
                    if (*sym).flags
                        & ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_uint != 0
                    {
                        if bfd_get_flavour(abfd) as libc::c_uint
                            == bfd_target_elf_flavour as libc::c_int as libc::c_uint
                            && (*(sym as *mut elf_symbol_type)).internal_elf_sym.st_size
                                > 0 as libc::c_int as libc::c_ulong
                        {
                            stop_offset = addr_offset
                                .wrapping_add(
                                    (*(sym as *mut elf_symbol_type)).internal_elf_sym.st_size,
                                );
                            loop_until = stop_offset_reached;
                        } else {
                            loop_until = function_sym;
                        }
                    } else {
                        loop_until = next_sym;
                    }
                }
                free(alloc as *mut libc::c_void);
            }
        }
        if prefix_addresses == 0 && do_print as libc::c_int != 0 {
            ((*pinfo).fprintf_func)
                .expect(
                    "non-null function pointer",
                )((*pinfo).stream, b"\n\0" as *const u8 as *const libc::c_char);
            objdump_print_addr_with_sym(
                abfd,
                section,
                sym,
                addr,
                pinfo,
                0 as libc::c_int != 0,
            );
            ((*pinfo).fprintf_func)
                .expect(
                    "non-null function pointer",
                )((*pinfo).stream, b":\n\0" as *const u8 as *const libc::c_char);
        }
        if !sym.is_null() && bfd_asymbol_value(sym) > addr {
            nextsym = sym;
        } else if sym.is_null() {
            nextsym = 0 as *mut asymbol;
        } else {
            while place < sorted_symcount
                && !(strcmp(
                    bfd_section_name((**sorted_syms.offset(place as isize)).section),
                    bfd_section_name(section),
                ) == 0 as libc::c_int
                    && bfd_asymbol_value(*sorted_syms.offset(place as isize))
                        > bfd_asymbol_value(sym)
                    && ((*pinfo).symbol_is_valid)
                        .expect(
                            "non-null function pointer",
                        )(*sorted_syms.offset(place as isize), pinfo) as libc::c_int
                        != 0)
            {
                place += 1;
                place;
            }
            if place >= sorted_symcount {
                nextsym = 0 as *mut asymbol;
            } else {
                nextsym = *sorted_syms.offset(place as isize);
            }
        }
        if !sym.is_null() && bfd_asymbol_value(sym) > addr {
            nextstop_offset = (bfd_asymbol_value(sym)).wrapping_sub((*section).vma);
        } else if nextsym.is_null() {
            nextstop_offset = stop_offset;
        } else {
            nextstop_offset = (bfd_asymbol_value(nextsym)).wrapping_sub((*section).vma);
        }
        if nextstop_offset > stop_offset || nextstop_offset <= addr_offset {
            nextstop_offset = stop_offset;
        }
        if disassemble_all as libc::c_int != 0 || sym.is_null()
            || (*sym).section != section || bfd_asymbol_value(sym) > addr
            || (*sym).flags & ((1 as libc::c_int) << 16 as libc::c_int) as libc::c_uint
                == 0 as libc::c_int as libc::c_uint
                && (strstr(
                    bfd_asymbol_name(sym),
                    b"gnu_compiled\0" as *const u8 as *const libc::c_char,
                ))
                    .is_null()
                && (strstr(
                    bfd_asymbol_name(sym),
                    b"gcc2_compiled\0" as *const u8 as *const libc::c_char,
                ))
                    .is_null()
            || (*sym).flags & ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_uint
                != 0 as libc::c_int as libc::c_uint
        {
            insns = 1 as libc::c_int != 0;
        } else {
            insns = 0 as libc::c_int != 0;
        }
        if do_print {
            if visualize_jumps as libc::c_int != 0 && !abfd.is_null() && !sym.is_null()
                && !((*sym).name).is_null()
            {
                let mut di: disassemble_info = disassemble_info {
                    fprintf_func: None,
                    stream: 0 as *mut libc::c_void,
                    application_data: 0 as *mut libc::c_void,
                    flavour: bfd_target_unknown_flavour,
                    arch: bfd_arch_unknown,
                    mach: 0,
                    endian: BFD_ENDIAN_BIG,
                    endian_code: BFD_ENDIAN_BIG,
                    section: 0 as *mut asection,
                    symbols: 0 as *mut *mut asymbol,
                    num_symbols: 0,
                    symtab: 0 as *mut *mut asymbol,
                    symtab_pos: 0,
                    symtab_size: 0,
                    flags: 0,
                    dynrelbuf: 0 as *mut *mut arelent,
                    dynrelcount: 0,
                    private_data: 0 as *mut libc::c_void,
                    read_memory_func: None,
                    memory_error_func: None,
                    print_address_func: None,
                    symbol_at_address_func: None,
                    symbol_is_valid: None,
                    buffer: 0 as *mut bfd_byte,
                    buffer_vma: 0,
                    buffer_length: 0,
                    bytes_per_line: 0,
                    bytes_per_chunk: 0,
                    display_endian: BFD_ENDIAN_BIG,
                    octets_per_byte: 0,
                    skip_zeroes: 0,
                    skip_zeroes_at_end: 0,
                    disassembler_needs_relocs: false,
                    insn_info_valid: 0,
                    branch_delay_insns: 0,
                    data_size: 0,
                    insn_type: dis_noninsn,
                    target: 0,
                    target2: 0,
                    disassembler_options: 0 as *const libc::c_char,
                    stop_vma: 0,
                    stop_offset: 0,
                };
                let mut sf: SFILE = SFILE {
                    buffer: 0 as *mut libc::c_char,
                    pos: 0,
                    alloc: 0,
                };
                sf
                    .alloc = (strlen((*sym).name))
                    .wrapping_add(40 as libc::c_int as libc::c_ulong);
                sf.buffer = xmalloc(sf.alloc) as *mut libc::c_char;
                sf.pos = 0 as libc::c_int as size_t;
                di
                    .fprintf_func = ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut SFILE,
                            *const libc::c_char,
                            ...
                        ) -> libc::c_int,
                    >,
                    fprintf_ftype,
                >(
                    Some(
                        objdump_sprintf
                            as unsafe extern "C" fn(
                                *mut SFILE,
                                *const libc::c_char,
                                ...
                            ) -> libc::c_int,
                    ),
                );
                di.stream = &mut sf as *mut SFILE as *mut libc::c_void;
                objdump_print_symname(abfd, &mut di, sym);
                detected_jumps = disassemble_jumps(
                    pinfo,
                    (*paux).disassemble_fn,
                    addr_offset,
                    nextstop_offset,
                    rel_offset,
                    &mut rel_pp,
                    rel_ppend,
                );
                free(sf.buffer as *mut libc::c_void);
            }
            disassemble_bytes(
                pinfo,
                (*paux).disassemble_fn,
                insns,
                data,
                addr_offset,
                nextstop_offset,
                rel_offset,
                &mut rel_pp,
                rel_ppend,
            );
            while !detected_jumps.is_null() {
                detected_jumps = jump_info_free(detected_jumps);
            }
        }
        addr_offset = nextstop_offset;
        sym = nextsym;
    }
    free(data as *mut libc::c_void);
    if !rel_ppstart.is_null() {
        free(rel_ppstart as *mut libc::c_void);
    }
}
unsafe extern "C" fn disassemble_data(mut abfd: *mut bfd) {
    let mut disasm_info: disassemble_info = disassemble_info {
        fprintf_func: None,
        stream: 0 as *mut libc::c_void,
        application_data: 0 as *mut libc::c_void,
        flavour: bfd_target_unknown_flavour,
        arch: bfd_arch_unknown,
        mach: 0,
        endian: BFD_ENDIAN_BIG,
        endian_code: BFD_ENDIAN_BIG,
        section: 0 as *mut asection,
        symbols: 0 as *mut *mut asymbol,
        num_symbols: 0,
        symtab: 0 as *mut *mut asymbol,
        symtab_pos: 0,
        symtab_size: 0,
        flags: 0,
        dynrelbuf: 0 as *mut *mut arelent,
        dynrelcount: 0,
        private_data: 0 as *mut libc::c_void,
        read_memory_func: None,
        memory_error_func: None,
        print_address_func: None,
        symbol_at_address_func: None,
        symbol_is_valid: None,
        buffer: 0 as *mut bfd_byte,
        buffer_vma: 0,
        buffer_length: 0,
        bytes_per_line: 0,
        bytes_per_chunk: 0,
        display_endian: BFD_ENDIAN_BIG,
        octets_per_byte: 0,
        skip_zeroes: 0,
        skip_zeroes_at_end: 0,
        disassembler_needs_relocs: false,
        insn_info_valid: 0,
        branch_delay_insns: 0,
        data_size: 0,
        insn_type: dis_noninsn,
        target: 0,
        target2: 0,
        disassembler_options: 0 as *const libc::c_char,
        stop_vma: 0,
        stop_offset: 0,
    };
    let mut aux: objdump_disasm_info = objdump_disasm_info {
        abfd: 0 as *mut bfd,
        require_sec: false,
        disassemble_fn: None,
        reloc: 0 as *mut arelent,
        symbol: 0 as *const libc::c_char,
    };
    let mut i: libc::c_long = 0;
    print_files = 0 as *mut print_file_list;
    prev_functionname = 0 as *mut libc::c_char;
    prev_line = -(1 as libc::c_int) as libc::c_uint;
    prev_discriminator = 0 as libc::c_int as libc::c_uint;
    sorted_symcount = if symcount != 0 { symcount } else { dynsymcount };
    sorted_syms = xmalloc(
        ((sorted_symcount + synthcount) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut asymbol>() as libc::c_ulong),
    ) as *mut *mut asymbol;
    if sorted_symcount != 0 as libc::c_int as libc::c_long {
        memcpy(
            sorted_syms as *mut libc::c_void,
            (if symcount != 0 { syms } else { dynsyms }) as *const libc::c_void,
            (sorted_symcount as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<*mut asymbol>() as libc::c_ulong),
        );
        sorted_symcount = remove_useless_symbols(sorted_syms, sorted_symcount);
    }
    i = 0 as libc::c_int as libc::c_long;
    while i < synthcount {
        let ref mut fresh15 = *sorted_syms.offset(sorted_symcount as isize);
        *fresh15 = synthsyms.offset(i as isize);
        sorted_symcount += 1;
        sorted_symcount;
        i += 1;
        i;
    }
    init_disassemble_info(
        &mut disasm_info,
        stdout as *mut libc::c_void,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut FILE, *const libc::c_char, ...) -> libc::c_int,
            >,
            fprintf_ftype,
        >(
            Some(
                fprintf
                    as unsafe extern "C" fn(
                        *mut FILE,
                        *const libc::c_char,
                        ...
                    ) -> libc::c_int,
            ),
        ),
    );
    disasm_info
        .application_data = &mut aux as *mut objdump_disasm_info as *mut libc::c_void;
    aux.abfd = abfd;
    aux.require_sec = 0 as libc::c_int != 0;
    disasm_info.dynrelbuf = 0 as *mut *mut arelent;
    disasm_info.dynrelcount = 0 as libc::c_int as libc::c_long;
    aux.reloc = 0 as *mut arelent;
    aux.symbol = disasm_sym;
    disasm_info
        .print_address_func = Some(
        objdump_print_address
            as unsafe extern "C" fn(bfd_vma, *mut disassemble_info) -> (),
    );
    disasm_info
        .symbol_at_address_func = Some(
        objdump_symbol_at_address
            as unsafe extern "C" fn(bfd_vma, *mut disassemble_info) -> *mut asymbol,
    );
    if !machine.is_null() {
        let mut inf: *const bfd_arch_info_type = bfd_scan_arch(machine);
        if inf.is_null() {
            fatal(
                dcgettext(
                    0 as *const libc::c_char,
                    b"can't use supplied machine %s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                machine,
            );
        }
        (*abfd).arch_info = inf;
    }
    if endian as libc::c_uint != BFD_ENDIAN_UNKNOWN as libc::c_int as libc::c_uint {
        let mut xvec: *mut bfd_target = 0 as *mut bfd_target;
        xvec = xmalloc(::core::mem::size_of::<bfd_target>() as libc::c_ulong)
            as *mut bfd_target;
        memcpy(
            xvec as *mut libc::c_void,
            (*abfd).xvec as *const libc::c_void,
            ::core::mem::size_of::<bfd_target>() as libc::c_ulong,
        );
        (*xvec).byteorder = endian;
        (*abfd).xvec = xvec;
    }
    aux
        .disassemble_fn = disassembler(
        bfd_get_arch(abfd),
        bfd_big_endian(abfd),
        bfd_get_mach(abfd),
        abfd,
    );
    if (aux.disassemble_fn).is_none() {
        non_fatal(
            dcgettext(
                0 as *const libc::c_char,
                b"can't disassemble for architecture %s\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            bfd_printable_arch_mach(
                bfd_get_arch(abfd),
                0 as libc::c_int as libc::c_ulong,
            ),
        );
        exit_status = 1 as libc::c_int;
        return;
    }
    disasm_info.flavour = bfd_get_flavour(abfd);
    disasm_info.arch = bfd_get_arch(abfd);
    disasm_info.mach = bfd_get_mach(abfd);
    disasm_info.disassembler_options = disassembler_options;
    disasm_info.octets_per_byte = bfd_octets_per_byte(abfd, 0 as *const asection);
    disasm_info.skip_zeroes = 8 as libc::c_int as libc::c_uint;
    disasm_info.skip_zeroes_at_end = 3 as libc::c_int as libc::c_uint;
    disasm_info.disassembler_needs_relocs = 0 as libc::c_int != 0;
    if bfd_big_endian(abfd) {
        disasm_info.endian = BFD_ENDIAN_BIG;
        disasm_info.display_endian = disasm_info.endian;
    } else if bfd_little_endian(abfd) {
        disasm_info.endian = BFD_ENDIAN_LITTLE;
        disasm_info.display_endian = disasm_info.endian;
    } else {
        disasm_info.endian = BFD_ENDIAN_UNKNOWN;
    }
    disasm_info.endian_code = disasm_info.endian;
    disassemble_init_for_target(&mut disasm_info);
    let mut relsize: libc::c_long = (Some(
        ((*(*abfd).xvec)._bfd_get_dynamic_reloc_upper_bound)
            .expect("non-null function pointer"),
    ))
        .expect("non-null function pointer")(abfd);
    if relsize < 0 as libc::c_int as libc::c_long && dump_dynamic_reloc_info != 0 {
        bfd_fatal(bfd_get_filename(abfd));
    }
    if relsize > 0 as libc::c_int as libc::c_long {
        disasm_info.dynrelbuf = xmalloc(relsize as size_t) as *mut *mut arelent;
        disasm_info
            .dynrelcount = (Some(
            ((*(*abfd).xvec)._bfd_canonicalize_dynamic_reloc)
                .expect("non-null function pointer"),
        ))
            .expect("non-null function pointer")(abfd, disasm_info.dynrelbuf, dynsyms);
        if disasm_info.dynrelcount < 0 as libc::c_int as libc::c_long {
            bfd_fatal(bfd_get_filename(abfd));
        }
        qsort(
            disasm_info.dynrelbuf as *mut libc::c_void,
            disasm_info.dynrelcount as size_t,
            ::core::mem::size_of::<*mut arelent>() as libc::c_ulong,
            Some(
                compare_relocs
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
        );
    }
    disasm_info.symtab = sorted_syms;
    disasm_info.symtab_size = sorted_symcount as libc::c_int;
    bfd_map_over_sections(
        abfd,
        Some(
            disassemble_section
                as unsafe extern "C" fn(*mut bfd, *mut asection, *mut libc::c_void) -> (),
        ),
        &mut disasm_info as *mut disassemble_info as *mut libc::c_void,
    );
    free(disasm_info.dynrelbuf as *mut libc::c_void);
    disasm_info.dynrelbuf = 0 as *mut *mut arelent;
    free(sorted_syms as *mut libc::c_void);
    disassemble_free_target(&mut disasm_info);
}
unsafe extern "C" fn load_specific_debug_section(
    mut debug: dwarf_section_display_enum,
    mut sec: *mut asection,
    mut file: *mut libc::c_void,
) -> bool {
    let mut section: *mut dwarf_section = &mut (*debug_displays
        .as_mut_ptr()
        .offset(debug as isize))
        .section;
    let mut abfd: *mut bfd = file as *mut bfd;
    let mut contents: *mut bfd_byte = 0 as *mut bfd_byte;
    let mut amt: bfd_size_type = 0;
    let mut alloced: size_t = 0;
    let mut ret: bool = false;
    if !((*section).start).is_null() {
        if strcmp((*section).filename, bfd_get_filename(abfd)) == 0 as libc::c_int {
            return 1 as libc::c_int != 0;
        }
        free((*section).start as *mut libc::c_void);
    }
    (*section).filename = bfd_get_filename(abfd);
    (*section).reloc_info = 0 as *mut libc::c_void;
    (*section).num_relocs = 0 as libc::c_int as libc::c_ulong;
    (*section).address = bfd_section_vma(sec);
    (*section).size = bfd_section_size(sec);
    amt = ((*section).size).wrapping_add(1 as libc::c_int as libc::c_ulong);
    alloced = amt;
    if alloced != amt || alloced == 0 as libc::c_int as libc::c_ulong {
        (*section).start = 0 as *mut libc::c_uchar;
        free_debug_section(debug);
        printf(
            dcgettext(
                0 as *const libc::c_char,
                b"\nSection '%s' has an invalid size: %#llx.\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            sanitize_string((*section).name),
            (*section).size as libc::c_ulonglong,
        );
        return 0 as libc::c_int != 0;
    }
    contents = xmalloc(alloced) as *mut bfd_byte;
    (*section).start = contents;
    *((*section).start)
        .offset((*section).size as isize) = 0 as libc::c_int as libc::c_uchar;
    if (*abfd).flags & (0x2 as libc::c_int | 0x40 as libc::c_int) as libc::c_uint
        == 0 as libc::c_int as libc::c_uint
        && (*debug_displays.as_mut_ptr().offset(debug as isize)).relocate as libc::c_int
            != 0
    {
        ret = !(bfd_simple_get_relocated_section_contents(
            abfd,
            sec,
            (*section).start,
            syms,
        ))
            .is_null();
        if ret {
            let mut reloc_size: libc::c_long = bfd_get_reloc_upper_bound(abfd, sec);
            if reloc_size > 0 as libc::c_int as libc::c_long {
                let mut reloc_count: libc::c_ulong = 0;
                let mut relocs: *mut *mut arelent = 0 as *mut *mut arelent;
                relocs = xmalloc(reloc_size as size_t) as *mut *mut arelent;
                reloc_count = bfd_canonicalize_reloc(
                    abfd,
                    sec,
                    relocs,
                    0 as *mut *mut asymbol,
                ) as libc::c_ulong;
                if reloc_count == 0 as libc::c_int as libc::c_ulong {
                    free(relocs as *mut libc::c_void);
                } else {
                    (*section).reloc_info = relocs as *mut libc::c_void;
                    (*section).num_relocs = reloc_count;
                }
            }
        }
    } else {
        ret = bfd_get_full_section_contents(abfd, sec, &mut contents);
    }
    if !ret {
        free_debug_section(debug);
        printf(
            dcgettext(
                0 as *const libc::c_char,
                b"\nCan't get contents for section '%s'.\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            sanitize_string((*section).name),
        );
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn dump_dwarf_section(
    mut abfd: *mut bfd,
    mut section: *mut asection,
    mut _arg: *mut libc::c_void,
) {
    let mut name: *const libc::c_char = bfd_section_name(section);
    let mut match_0: *const libc::c_char = 0 as *const libc::c_char;
    let mut i: libc::c_int = 0;
    if startswith(name, b".gnu.linkonce.wi.\0" as *const u8 as *const libc::c_char) {
        match_0 = b".debug_info\0" as *const u8 as *const libc::c_char;
    } else {
        match_0 = name;
    }
    i = 0 as libc::c_int;
    while i < max as libc::c_int {
        if (strcmp(
            (*debug_displays.as_mut_ptr().offset(i as isize)).section.uncompressed_name,
            match_0,
        ) == 0 as libc::c_int
            || strcmp(
                (*debug_displays.as_mut_ptr().offset(i as isize))
                    .section
                    .compressed_name,
                match_0,
            ) == 0 as libc::c_int)
            && !((*debug_displays.as_mut_ptr().offset(i as isize)).enabled).is_null()
            && *(*debug_displays.as_mut_ptr().offset(i as isize)).enabled != 0
        {
            let mut sec: *mut dwarf_section = &mut (*debug_displays
                .as_mut_ptr()
                .offset(i as isize))
                .section;
            if strcmp((*sec).uncompressed_name, match_0) == 0 as libc::c_int {
                (*sec).name = (*sec).uncompressed_name;
            } else {
                (*sec).name = (*sec).compressed_name;
            }
            if load_specific_debug_section(
                i as dwarf_section_display_enum,
                section,
                abfd as *mut libc::c_void,
            ) {
                ((*debug_displays.as_mut_ptr().offset(i as isize)).display)
                    .expect("non-null function pointer")(sec, abfd as *mut libc::c_void);
                if i != info as libc::c_int && i != abbrev as libc::c_int {
                    free_debug_section(i as dwarf_section_display_enum);
                }
            }
            break;
        } else {
            i += 1;
            i;
        }
    }
}
unsafe extern "C" fn dump_dwarf(mut abfd: *mut bfd) {
    if byte_get.is_none() {
        warn(
            dcgettext(
                0 as *const libc::c_char,
                b"File %s does not contain any dwarf debug information\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            bfd_get_filename(abfd),
        );
        return;
    }
    match bfd_get_arch(abfd) as libc::c_uint {
        28 => {
            eh_addr_size = 4 as libc::c_int as libc::c_uint;
        }
        _ => {
            eh_addr_size = (bfd_arch_bits_per_address(abfd))
                .wrapping_div(8 as libc::c_int as libc::c_uint);
        }
    }
    init_dwarf_regnames_by_bfd_arch_and_mach(bfd_get_arch(abfd), bfd_get_mach(abfd));
    bfd_map_over_sections(
        abfd,
        Some(
            dump_dwarf_section
                as unsafe extern "C" fn(*mut bfd, *mut asection, *mut libc::c_void) -> (),
        ),
        0 as *mut libc::c_void,
    );
}
unsafe extern "C" fn read_section_stabs(
    mut abfd: *mut bfd,
    mut sect_name: *const libc::c_char,
    mut size_ptr: *mut bfd_size_type,
    mut entsize_ptr: *mut bfd_size_type,
) -> *mut bfd_byte {
    let mut stabsect: *mut asection = 0 as *mut asection;
    let mut contents: *mut bfd_byte = 0 as *mut bfd_byte;
    stabsect = bfd_get_section_by_name(abfd, sect_name);
    if stabsect.is_null() {
        printf(
            dcgettext(
                0 as *const libc::c_char,
                b"No %s section present\n\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            sanitize_string(sect_name),
        );
        return 0 as *mut bfd_byte;
    }
    if !bfd_malloc_and_get_section(abfd, stabsect, &mut contents) {
        non_fatal(
            dcgettext(
                0 as *const libc::c_char,
                b"reading %s section of %s failed: %s\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            sect_name,
            bfd_get_filename(abfd),
            bfd_errmsg(bfd_get_error()),
        );
        exit_status = 1 as libc::c_int;
        free(contents as *mut libc::c_void);
        return 0 as *mut bfd_byte;
    }
    *size_ptr = bfd_section_size(stabsect);
    if !entsize_ptr.is_null() {
        *entsize_ptr = (*stabsect).entsize as bfd_size_type;
    }
    return contents;
}
unsafe extern "C" fn print_section_stabs(
    mut abfd: *mut bfd,
    mut stabsect_name: *const libc::c_char,
    mut string_offset_ptr: *mut libc::c_uint,
) {
    let mut i: libc::c_int = 0;
    let mut file_string_table_offset: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut next_file_string_table_offset: libc::c_uint = *string_offset_ptr;
    let mut stabp: *mut bfd_byte = 0 as *mut bfd_byte;
    let mut stabs_end: *mut bfd_byte = 0 as *mut bfd_byte;
    stabp = stabs;
    stabs_end = stabp.offset(stab_size as isize);
    printf(
        dcgettext(
            0 as *const libc::c_char,
            b"Contents of %s section:\n\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        sanitize_string(stabsect_name),
    );
    printf(
        b"Symnum n_type n_othr n_desc n_value  n_strx String\n\0" as *const u8
            as *const libc::c_char,
    );
    i = -(1 as libc::c_int);
    while stabp <= stabs_end.offset(-(12 as libc::c_int as isize)) {
        let mut name: *const libc::c_char = 0 as *const libc::c_char;
        let mut strx: libc::c_ulong = 0;
        let mut type_0: libc::c_uchar = 0;
        let mut other: libc::c_uchar = 0;
        let mut desc: libc::c_ushort = 0;
        let mut value: bfd_vma = 0;
        strx = (Some(((*(*abfd).xvec).bfd_h_getx32).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(stabp.offset(0 as libc::c_int as isize) as *const libc::c_void);
        type_0 = (*(stabp.offset(4 as libc::c_int as isize) as *const libc::c_uchar)
            as bfd_vma & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
        other = (*(stabp.offset(5 as libc::c_int as isize) as *const libc::c_uchar)
            as bfd_vma & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
        desc = (Some(((*(*abfd).xvec).bfd_h_getx16).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(stabp.offset(6 as libc::c_int as isize) as *const libc::c_void)
            as libc::c_ushort;
        value = (Some(
            ((*(*abfd).xvec).bfd_h_getx32).expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(stabp.offset(8 as libc::c_int as isize) as *const libc::c_void);
        printf(b"\n%-6d \0" as *const u8 as *const libc::c_char, i);
        name = bfd_get_stab_name(type_0 as libc::c_int);
        if !name.is_null() {
            printf(b"%-6s\0" as *const u8 as *const libc::c_char, sanitize_string(name));
        } else if type_0 as libc::c_int == 0 as libc::c_int {
            printf(b"HdrSym\0" as *const u8 as *const libc::c_char);
        } else {
            printf(b"%-6d\0" as *const u8 as *const libc::c_char, type_0 as libc::c_int);
        }
        printf(
            b" %-6d %-6d \0" as *const u8 as *const libc::c_char,
            other as libc::c_int,
            desc as libc::c_int,
        );
        bfd_fprintf_vma(abfd, stdout as *mut libc::c_void, value);
        printf(b" %-6lu\0" as *const u8 as *const libc::c_char, strx);
        if type_0 as libc::c_int == 0 as libc::c_int {
            file_string_table_offset = next_file_string_table_offset;
            next_file_string_table_offset = (next_file_string_table_offset
                as libc::c_ulong)
                .wrapping_add(value) as libc::c_uint as libc::c_uint;
        } else {
            let mut amt: bfd_size_type = strx
                .wrapping_add(file_string_table_offset as libc::c_ulong);
            if amt < stabstr_size {
                printf(
                    b" %.*s\0" as *const u8 as *const libc::c_char,
                    stabstr_size.wrapping_sub(amt) as libc::c_int,
                    strtab.offset(amt as isize),
                );
            } else {
                printf(b" *\0" as *const u8 as *const libc::c_char);
            }
        }
        stabp = stabp.offset(12 as libc::c_int as isize);
        i += 1;
        i;
    }
    printf(b"\n\n\0" as *const u8 as *const libc::c_char);
    *string_offset_ptr = next_file_string_table_offset;
}
unsafe extern "C" fn find_stabs_section(
    mut abfd: *mut bfd,
    mut section: *mut asection,
    mut names: *mut libc::c_void,
) {
    let mut len: libc::c_int = 0;
    let mut sought: *mut stab_section_names = names as *mut stab_section_names;
    len = strlen((*sought).section_name) as libc::c_int;
    if strncmp((*sought).section_name, (*section).name, len as libc::c_ulong)
        == 0 as libc::c_int
        && (*((*section).name).offset(len as isize) as libc::c_int == 0 as libc::c_int
            || *((*section).name).offset(len as isize) as libc::c_int == '.' as i32
                && _sch_istable[(*((*section).name)
                    .offset((len + 1 as libc::c_int) as isize) as libc::c_int
                    & 0xff as libc::c_int) as usize] as libc::c_int
                    & _sch_isdigit as libc::c_int as libc::c_ushort as libc::c_int != 0)
    {
        if strtab.is_null() {
            strtab = read_section_stabs(
                abfd,
                (*sought).string_section_name,
                &mut stabstr_size,
                0 as *mut bfd_size_type,
            );
        }
        if !strtab.is_null() {
            stabs = read_section_stabs(
                abfd,
                (*section).name,
                &mut stab_size,
                0 as *mut bfd_size_type,
            );
            if !stabs.is_null() {
                print_section_stabs(abfd, (*section).name, &mut (*sought).string_offset);
            }
        }
    }
}
unsafe extern "C" fn dump_stabs_section(
    mut abfd: *mut bfd,
    mut stabsect_name: *mut libc::c_char,
    mut strsect_name: *mut libc::c_char,
) {
    let mut s: stab_section_names = stab_section_names {
        section_name: 0 as *const libc::c_char,
        string_section_name: 0 as *const libc::c_char,
        string_offset: 0,
    };
    s.section_name = stabsect_name;
    s.string_section_name = strsect_name;
    s.string_offset = 0 as libc::c_int as libc::c_uint;
    bfd_map_over_sections(
        abfd,
        Some(
            find_stabs_section
                as unsafe extern "C" fn(*mut bfd, *mut asection, *mut libc::c_void) -> (),
        ),
        &mut s as *mut stab_section_names as *mut libc::c_void,
    );
    free(strtab as *mut libc::c_void);
    strtab = 0 as *mut bfd_byte;
}
unsafe extern "C" fn dump_stabs(mut abfd: *mut bfd) {
    dump_stabs_section(
        abfd,
        b".stab\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b".stabstr\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dump_stabs_section(
        abfd,
        b".stab.excl\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b".stab.exclstr\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dump_stabs_section(
        abfd,
        b".stab.index\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b".stab.indexstr\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dump_stabs_section(
        abfd,
        b"LC_SYMTAB.stabs\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"LC_SYMTAB.stabstr\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dump_stabs_section(
        abfd,
        b"$GDB_SYMBOLS$\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"$GDB_STRINGS$\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
unsafe extern "C" fn dump_bfd_header(mut abfd: *mut bfd) {
    let mut comma: *mut libc::c_char = b"\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    printf(
        dcgettext(
            0 as *const libc::c_char,
            b"architecture: %s, \0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        bfd_printable_arch_mach(bfd_get_arch(abfd), bfd_get_mach(abfd)),
    );
    printf(
        dcgettext(
            0 as *const libc::c_char,
            b"flags 0x%08x:\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        (*abfd).flags
            & !(0x800 as libc::c_int | 0x4000 as libc::c_int | 0x8000 as libc::c_int
                | 0x1000 as libc::c_int | 0x10000 as libc::c_int | 0x400 as libc::c_int
                | 0x2000 as libc::c_int | 0x20000 as libc::c_int | 0x40000 as libc::c_int
                | 0x80000 as libc::c_int) as libc::c_uint,
    );
    if (*abfd).flags & 0x1 as libc::c_int as libc::c_uint != 0 {
        printf(
            b"%s%s\0" as *const u8 as *const libc::c_char,
            comma,
            b"HAS_RELOC\0" as *const u8 as *const libc::c_char,
        );
        comma = b", \0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if (*abfd).flags & 0x2 as libc::c_int as libc::c_uint != 0 {
        printf(
            b"%s%s\0" as *const u8 as *const libc::c_char,
            comma,
            b"EXEC_P\0" as *const u8 as *const libc::c_char,
        );
        comma = b", \0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if (*abfd).flags & 0x4 as libc::c_int as libc::c_uint != 0 {
        printf(
            b"%s%s\0" as *const u8 as *const libc::c_char,
            comma,
            b"HAS_LINENO\0" as *const u8 as *const libc::c_char,
        );
        comma = b", \0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if (*abfd).flags & 0x8 as libc::c_int as libc::c_uint != 0 {
        printf(
            b"%s%s\0" as *const u8 as *const libc::c_char,
            comma,
            b"HAS_DEBUG\0" as *const u8 as *const libc::c_char,
        );
        comma = b", \0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if (*abfd).flags & 0x10 as libc::c_int as libc::c_uint != 0 {
        printf(
            b"%s%s\0" as *const u8 as *const libc::c_char,
            comma,
            b"HAS_SYMS\0" as *const u8 as *const libc::c_char,
        );
        comma = b", \0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if (*abfd).flags & 0x20 as libc::c_int as libc::c_uint != 0 {
        printf(
            b"%s%s\0" as *const u8 as *const libc::c_char,
            comma,
            b"HAS_LOCALS\0" as *const u8 as *const libc::c_char,
        );
        comma = b", \0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if (*abfd).flags & 0x40 as libc::c_int as libc::c_uint != 0 {
        printf(
            b"%s%s\0" as *const u8 as *const libc::c_char,
            comma,
            b"DYNAMIC\0" as *const u8 as *const libc::c_char,
        );
        comma = b", \0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if (*abfd).flags & 0x80 as libc::c_int as libc::c_uint != 0 {
        printf(
            b"%s%s\0" as *const u8 as *const libc::c_char,
            comma,
            b"WP_TEXT\0" as *const u8 as *const libc::c_char,
        );
        comma = b", \0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if (*abfd).flags & 0x100 as libc::c_int as libc::c_uint != 0 {
        printf(
            b"%s%s\0" as *const u8 as *const libc::c_char,
            comma,
            b"D_PAGED\0" as *const u8 as *const libc::c_char,
        );
        comma = b", \0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if (*abfd).flags & 0x200 as libc::c_int as libc::c_uint != 0 {
        printf(
            b"%s%s\0" as *const u8 as *const libc::c_char,
            comma,
            b"BFD_IS_RELAXABLE\0" as *const u8 as *const libc::c_char,
        );
        comma = b", \0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    printf(
        dcgettext(
            0 as *const libc::c_char,
            b"\nstart address 0x\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    bfd_fprintf_vma(abfd, stdout as *mut libc::c_void, (*abfd).start_address);
    printf(b"\n\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn dump_ctf_indent_lines(
    mut _sect: ctf_sect_names_t,
    mut s: *mut libc::c_char,
    mut arg: *mut libc::c_void,
) -> *mut libc::c_char {
    let mut blanks: *const libc::c_char = arg as *const libc::c_char;
    let mut new_s: *mut libc::c_char = 0 as *mut libc::c_char;
    if asprintf(
        &mut new_s as *mut *mut libc::c_char,
        b"%s%s\0" as *const u8 as *const libc::c_char,
        blanks,
        s,
    ) < 0 as libc::c_int
    {
        return s;
    }
    return new_s;
}
unsafe extern "C" fn make_ctfsect(
    mut name: *const libc::c_char,
    mut data: *mut bfd_byte,
    mut size: bfd_size_type,
) -> ctf_sect_t {
    let mut ctfsect: ctf_sect_t = ctf_sect_t {
        cts_name: 0 as *const libc::c_char,
        cts_data: 0 as *const libc::c_void,
        cts_size: 0,
        cts_entsize: 0,
    };
    ctfsect.cts_name = name;
    ctfsect.cts_entsize = 1 as libc::c_int as size_t;
    ctfsect.cts_size = size;
    ctfsect.cts_data = data as *const libc::c_void;
    return ctfsect;
}
unsafe extern "C" fn dump_ctf_errs(mut fp: *mut ctf_dict_t) {
    let mut it: *mut ctf_next_t = 0 as *mut ctf_next_t;
    let mut errtext: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut is_warning: libc::c_int = 0;
    let mut err: libc::c_int = 0;
    loop {
        errtext = ctf_errwarning_next(fp, &mut it, &mut is_warning, &mut err);
        if errtext.is_null() {
            break;
        }
        non_fatal(
            dcgettext(
                0 as *const libc::c_char,
                b"%s: %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            if is_warning != 0 {
                dcgettext(
                    0 as *const libc::c_char,
                    b"warning\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                )
            } else {
                dcgettext(
                    0 as *const libc::c_char,
                    b"error\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                )
            },
            errtext,
        );
        free(errtext as *mut libc::c_void);
    }
    if err != ECTF_NEXT_END as libc::c_int {
        non_fatal(
            dcgettext(
                0 as *const libc::c_char,
                b"CTF error: cannot get CTF errors: `%s'\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            ctf_errmsg(err),
        );
    }
}
unsafe extern "C" fn dump_ctf_archive_member(
    mut ctf: *mut ctf_dict_t,
    mut name: *const libc::c_char,
    mut arg: *mut libc::c_void,
) -> libc::c_int {
    let mut parent: *mut ctf_dict_t = arg as *mut ctf_dict_t;
    let mut things: [*const libc::c_char; 8] = [
        b"Header\0" as *const u8 as *const libc::c_char,
        b"Labels\0" as *const u8 as *const libc::c_char,
        b"Data objects\0" as *const u8 as *const libc::c_char,
        b"Function objects\0" as *const u8 as *const libc::c_char,
        b"Variables\0" as *const u8 as *const libc::c_char,
        b"Types\0" as *const u8 as *const libc::c_char,
        b"Strings\0" as *const u8 as *const libc::c_char,
        b"\0" as *const u8 as *const libc::c_char,
    ];
    let mut thing: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
    let mut i: size_t = 0;
    if strcmp(name, b".ctf\0" as *const u8 as *const libc::c_char) != 0 as libc::c_int {
        printf(
            dcgettext(
                0 as *const libc::c_char,
                b"\nCTF archive member: %s:\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            sanitize_string(name),
        );
        ctf_import(ctf, parent);
    }
    i = 0 as libc::c_int as size_t;
    thing = things.as_mut_ptr();
    while **thing.offset(0 as libc::c_int as isize) != 0 {
        let mut s: *mut ctf_dump_state_t = 0 as *mut ctf_dump_state_t;
        let mut item: *mut libc::c_char = 0 as *mut libc::c_char;
        printf(b"\n  %s:\n\0" as *const u8 as *const libc::c_char, *thing);
        loop {
            item = ctf_dump(
                ctf,
                &mut s,
                i as ctf_sect_names_t,
                Some(
                    dump_ctf_indent_lines
                        as unsafe extern "C" fn(
                            ctf_sect_names_t,
                            *mut libc::c_char,
                            *mut libc::c_void,
                        ) -> *mut libc::c_char,
                ),
                b"    \0" as *const u8 as *const libc::c_char as *mut libc::c_void,
            );
            if item.is_null() {
                break;
            }
            printf(b"%s\n\0" as *const u8 as *const libc::c_char, item);
            free(item as *mut libc::c_void);
        }
        if ctf_errno(ctf) != 0 {
            non_fatal(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Iteration failed: %s, %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                *thing,
                ctf_errmsg(ctf_errno(ctf)),
            );
            break;
        } else {
            thing = thing.offset(1);
            thing;
            i = i.wrapping_add(1);
            i;
        }
    }
    dump_ctf_errs(ctf);
    return 0 as libc::c_int;
}
unsafe extern "C" fn dump_ctf(
    mut abfd: *mut bfd,
    mut sect_name: *const libc::c_char,
    mut parent_name: *const libc::c_char,
) {
    let mut ctfa: *mut ctf_archive_t = 0 as *mut ctf_archive_t;
    let mut parenta: *mut ctf_archive_t = 0 as *mut ctf_archive_t;
    let mut lookparent: *mut ctf_archive_t = 0 as *mut ctf_archive_t;
    let mut ctfdata: *mut bfd_byte = 0 as *mut bfd_byte;
    let mut parentdata: *mut bfd_byte = 0 as *mut bfd_byte;
    let mut ctfsize: bfd_size_type = 0;
    let mut parentsize: bfd_size_type = 0;
    let mut ctfsect: ctf_sect_t = ctf_sect_t {
        cts_name: 0 as *const libc::c_char,
        cts_data: 0 as *const libc::c_void,
        cts_size: 0,
        cts_entsize: 0,
    };
    let mut parent: *mut ctf_dict_t = 0 as *mut ctf_dict_t;
    let mut err: libc::c_int = 0;
    ctfdata = read_section_stabs(abfd, sect_name, &mut ctfsize, 0 as *mut bfd_size_type);
    if ctfdata.is_null() {
        bfd_fatal(bfd_get_filename(abfd));
    }
    if !parent_name.is_null()
        && {
            parentdata = read_section_stabs(
                abfd,
                parent_name,
                &mut parentsize,
                0 as *mut bfd_size_type,
            );
            parentdata.is_null()
        }
    {
        bfd_fatal(bfd_get_filename(abfd));
    }
    ctfsect = make_ctfsect(sect_name, ctfdata, ctfsize);
    ctfa = ctf_bfdopen_ctfsect(abfd, &mut ctfsect, &mut err);
    if ctfa.is_null() {
        dump_ctf_errs(0 as *mut ctf_dict_t);
        non_fatal(
            dcgettext(
                0 as *const libc::c_char,
                b"CTF open failure: %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            ctf_errmsg(err),
        );
        bfd_fatal(bfd_get_filename(abfd));
    }
    if !parentdata.is_null() {
        ctfsect = make_ctfsect(parent_name, parentdata, parentsize);
        parenta = ctf_bfdopen_ctfsect(abfd, &mut ctfsect, &mut err);
        if parenta.is_null() {
            dump_ctf_errs(0 as *mut ctf_dict_t);
            non_fatal(
                dcgettext(
                    0 as *const libc::c_char,
                    b"CTF open failure: %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                ctf_errmsg(err),
            );
            bfd_fatal(bfd_get_filename(abfd));
        }
        lookparent = parenta;
    } else {
        lookparent = ctfa;
    }
    parent = ctf_dict_open(lookparent, 0 as *const libc::c_char, &mut err);
    if parent.is_null() {
        dump_ctf_errs(0 as *mut ctf_dict_t);
        non_fatal(
            dcgettext(
                0 as *const libc::c_char,
                b"CTF open failure: %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            ctf_errmsg(err),
        );
        bfd_fatal(bfd_get_filename(abfd));
    }
    printf(
        dcgettext(
            0 as *const libc::c_char,
            b"Contents of CTF section %s:\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        sanitize_string(sect_name),
    );
    err = ctf_archive_iter(
        ctfa,
        Some(
            dump_ctf_archive_member
                as unsafe extern "C" fn(
                    *mut ctf_dict_t,
                    *const libc::c_char,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
        parent as *mut libc::c_void,
    );
    if err != 0 as libc::c_int {
        dump_ctf_errs(0 as *mut ctf_dict_t);
        non_fatal(
            dcgettext(
                0 as *const libc::c_char,
                b"CTF archive member open failure: %s\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            ctf_errmsg(err),
        );
        bfd_fatal(bfd_get_filename(abfd));
    }
    ctf_dict_close(parent);
    ctf_close(ctfa);
    ctf_close(parenta);
    free(parentdata as *mut libc::c_void);
    free(ctfdata as *mut libc::c_void);
}
unsafe extern "C" fn dump_bfd_private_header(mut abfd: *mut bfd) {
    if !(Some(
        ((*(*abfd).xvec)._bfd_print_private_bfd_data).expect("non-null function pointer"),
    ))
        .expect("non-null function pointer")(abfd, stdout as *mut libc::c_void)
    {
        non_fatal(
            dcgettext(
                0 as *const libc::c_char,
                b"warning: private headers incomplete: %s\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            bfd_errmsg(bfd_get_error()),
        );
    }
}
unsafe extern "C" fn dump_target_specific(mut abfd: *mut bfd) {
    let mut desc: *const *const objdump_private_desc = 0
        as *const *const objdump_private_desc;
    let mut opt: *mut objdump_private_option = 0 as *mut objdump_private_option;
    let mut e: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut b: *mut libc::c_char = 0 as *mut libc::c_char;
    desc = objdump_private_vectors.as_ptr();
    while !(*desc).is_null() {
        if ((**desc).filter).expect("non-null function pointer")(abfd) != 0 {
            break;
        }
        desc = desc.offset(1);
        desc;
    }
    if (*desc).is_null() {
        non_fatal(
            dcgettext(
                0 as *const libc::c_char,
                b"option -P/--private not supported by this file\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return;
    }
    opt = (**desc).options;
    while !((*opt).name).is_null() {
        (*opt).selected = 0 as libc::c_int as libc::c_uint;
        opt = opt.offset(1);
        opt;
    }
    b = dump_private_options;
    loop {
        e = strchr(b, ',' as i32);
        if !e.is_null() {
            *e = 0 as libc::c_int as libc::c_char;
        }
        opt = (**desc).options;
        while !((*opt).name).is_null() {
            if strcmp((*opt).name, b) == 0 as libc::c_int {
                (*opt).selected = 1 as libc::c_int as libc::c_uint;
                break;
            } else {
                opt = opt.offset(1);
                opt;
            }
        }
        if ((*opt).name).is_null() {
            non_fatal(
                dcgettext(
                    0 as *const libc::c_char,
                    b"target specific dump '%s' not supported\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                b,
            );
        }
        if !e.is_null() {
            *e = ',' as i32 as libc::c_char;
            b = e.offset(1 as libc::c_int as isize);
        }
        if e.is_null() {
            break;
        }
    }
    ((**desc).dump).expect("non-null function pointer")(abfd);
}
unsafe extern "C" fn dump_section(
    mut abfd: *mut bfd,
    mut section: *mut asection,
    mut _dummy: *mut libc::c_void,
) {
    let mut data: *mut bfd_byte = 0 as *mut bfd_byte;
    let mut datasize: bfd_size_type = 0;
    let mut addr_offset: bfd_vma = 0;
    let mut start_offset: bfd_vma = 0;
    let mut stop_offset: bfd_vma = 0;
    let mut opb: libc::c_uint = bfd_octets_per_byte(abfd, section);
    let onaline: libc::c_int = 16 as libc::c_int;
    let mut buf: [libc::c_char; 64] = [0; 64];
    let mut count: libc::c_int = 0;
    let mut width: libc::c_int = 0;
    if !process_section_p(section) {
        return;
    }
    if (*section).flags & 0x100 as libc::c_int as libc::c_uint
        == 0 as libc::c_int as libc::c_uint
    {
        return;
    }
    datasize = bfd_section_size(section);
    if datasize == 0 as libc::c_int as libc::c_ulong {
        return;
    }
    if start_address == -(1 as libc::c_int) as bfd_vma || start_address < (*section).vma
    {
        start_offset = 0 as libc::c_int as bfd_vma;
    } else {
        start_offset = start_address.wrapping_sub((*section).vma);
    }
    if stop_address == -(1 as libc::c_int) as bfd_vma {
        stop_offset = datasize.wrapping_div(opb as libc::c_ulong);
    } else {
        if stop_address < (*section).vma {
            stop_offset = 0 as libc::c_int as bfd_vma;
        } else {
            stop_offset = stop_address.wrapping_sub((*section).vma);
        }
        if stop_offset > datasize.wrapping_div(opb as libc::c_ulong) {
            stop_offset = datasize.wrapping_div(opb as libc::c_ulong);
        }
    }
    if start_offset >= stop_offset {
        return;
    }
    printf(
        dcgettext(
            0 as *const libc::c_char,
            b"Contents of section %s:\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        sanitize_string((*section).name),
    );
    if display_file_offsets {
        printf(
            dcgettext(
                0 as *const libc::c_char,
                b"  (Starting at file offset: 0x%lx)\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            ((*section).filepos as libc::c_ulong).wrapping_add(start_offset),
        );
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    if !bfd_get_full_section_contents(abfd, section, &mut data) {
        non_fatal(
            dcgettext(
                0 as *const libc::c_char,
                b"Reading section %s failed because: %s\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            (*section).name,
            bfd_errmsg(bfd_get_error()),
        );
        return;
    }
    width = 4 as libc::c_int;
    bfd_sprintf_vma(abfd, buf.as_mut_ptr(), start_offset.wrapping_add((*section).vma));
    if strlen(buf.as_mut_ptr())
        >= ::core::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong
    {
        abort();
    }
    count = 0 as libc::c_int;
    while buf[count as usize] as libc::c_int == '0' as i32
        && buf[(count + 1 as libc::c_int) as usize] as libc::c_int != '\0' as i32
    {
        count += 1;
        count;
    }
    count = (strlen(buf.as_mut_ptr())).wrapping_sub(count as libc::c_ulong)
        as libc::c_int;
    if count > width {
        width = count;
    }
    bfd_sprintf_vma(
        abfd,
        buf.as_mut_ptr(),
        stop_offset
            .wrapping_add((*section).vma)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    if strlen(buf.as_mut_ptr())
        >= ::core::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong
    {
        abort();
    }
    count = 0 as libc::c_int;
    while buf[count as usize] as libc::c_int == '0' as i32
        && buf[(count + 1 as libc::c_int) as usize] as libc::c_int != '\0' as i32
    {
        count += 1;
        count;
    }
    count = (strlen(buf.as_mut_ptr())).wrapping_sub(count as libc::c_ulong)
        as libc::c_int;
    if count > width {
        width = count;
    }
    addr_offset = start_offset;
    while addr_offset < stop_offset {
        let mut j: bfd_size_type = 0;
        bfd_sprintf_vma(
            abfd,
            buf.as_mut_ptr(),
            addr_offset.wrapping_add((*section).vma),
        );
        count = strlen(buf.as_mut_ptr()) as libc::c_int;
        if count as size_t
            >= ::core::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong
        {
            abort();
        }
        putchar(' ' as i32);
        while count < width {
            putchar('0' as i32);
            count += 1;
            count;
        }
        fputs(buf.as_mut_ptr().offset(count as isize).offset(-(width as isize)), stdout);
        putchar(' ' as i32);
        j = addr_offset.wrapping_mul(opb as libc::c_ulong);
        while j
            < addr_offset
                .wrapping_mul(opb as libc::c_ulong)
                .wrapping_add(onaline as libc::c_ulong)
        {
            if j < stop_offset.wrapping_mul(opb as libc::c_ulong) {
                printf(
                    b"%02x\0" as *const u8 as *const libc::c_char,
                    *data.offset(j as isize) as libc::c_uint,
                );
            } else {
                printf(b"  \0" as *const u8 as *const libc::c_char);
            }
            if j & 3 as libc::c_int as libc::c_ulong == 3 as libc::c_int as libc::c_ulong
            {
                printf(b" \0" as *const u8 as *const libc::c_char);
            }
            j = j.wrapping_add(1);
            j;
        }
        printf(b" \0" as *const u8 as *const libc::c_char);
        j = addr_offset.wrapping_mul(opb as libc::c_ulong);
        while j
            < addr_offset
                .wrapping_mul(opb as libc::c_ulong)
                .wrapping_add(onaline as libc::c_ulong)
        {
            if j >= stop_offset.wrapping_mul(opb as libc::c_ulong) {
                printf(b" \0" as *const u8 as *const libc::c_char);
            } else {
                printf(
                    b"%c\0" as *const u8 as *const libc::c_char,
                    if _sch_istable[(*data.offset(j as isize) as libc::c_int
                        & 0xff as libc::c_int) as usize] as libc::c_int
                        & _sch_isprint as libc::c_int as libc::c_ushort as libc::c_int
                        != 0
                    {
                        *data.offset(j as isize) as libc::c_int
                    } else {
                        '.' as i32
                    },
                );
            }
            j = j.wrapping_add(1);
            j;
        }
        putchar('\n' as i32);
        addr_offset = (addr_offset as libc::c_ulong)
            .wrapping_add((onaline as libc::c_uint).wrapping_div(opb) as libc::c_ulong)
            as bfd_vma as bfd_vma;
    }
    free(data as *mut libc::c_void);
}
unsafe extern "C" fn dump_data(mut abfd: *mut bfd) {
    bfd_map_over_sections(
        abfd,
        Some(
            dump_section
                as unsafe extern "C" fn(*mut bfd, *mut asection, *mut libc::c_void) -> (),
        ),
        0 as *mut libc::c_void,
    );
}
unsafe extern "C" fn dump_symbols(mut _abfd: *mut bfd, mut dynamic: bool) {
    let mut current: *mut *mut asymbol = 0 as *mut *mut asymbol;
    let mut max_count: libc::c_long = 0;
    let mut count: libc::c_long = 0;
    if dynamic {
        current = dynsyms;
        max_count = dynsymcount;
        printf(b"DYNAMIC SYMBOL TABLE:\n\0" as *const u8 as *const libc::c_char);
    } else {
        current = syms;
        max_count = symcount;
        printf(b"SYMBOL TABLE:\n\0" as *const u8 as *const libc::c_char);
    }
    if max_count == 0 as libc::c_int as libc::c_long {
        printf(
            dcgettext(
                0 as *const libc::c_char,
                b"no symbols\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    count = 0 as libc::c_int as libc::c_long;
    while count < max_count {
        let mut cur_bfd: *mut bfd = 0 as *mut bfd;
        if (*current).is_null() {
            printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"no information for symbol number %ld\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                count,
            );
        } else {
            cur_bfd = bfd_asymbol_bfd(*current);
            if cur_bfd.is_null() {
                printf(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"could not determine the type of symbol number %ld\n\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    count,
                );
            } else if process_section_p((**current).section) as libc::c_int != 0
                && (dump_special_syms != 0
                    || !(Some(
                        ((*(*cur_bfd).xvec)._bfd_is_target_special_symbol)
                            .expect("non-null function pointer"),
                    ))
                        .expect("non-null function pointer")(cur_bfd, *current))
            {
                let mut name: *const libc::c_char = (**current).name;
                if do_demangle != 0 && !name.is_null()
                    && *name as libc::c_int != '\0' as i32
                {
                    let mut alloc: *mut libc::c_char = 0 as *mut libc::c_char;
                    alloc = bfd_demangle(cur_bfd, name, demangle_flags);
                    if !alloc.is_null() {
                        (**current).name = alloc;
                    }
                    (Some(
                        ((*(*cur_bfd).xvec)._bfd_print_symbol)
                            .expect("non-null function pointer"),
                    ))
                        .expect(
                            "non-null function pointer",
                        )(
                        cur_bfd,
                        stdout as *mut libc::c_void,
                        *current,
                        bfd_print_symbol_all,
                    );
                    if !alloc.is_null() {
                        (**current).name = name;
                        free(alloc as *mut libc::c_void);
                    }
                } else {
                    (Some(
                        ((*(*cur_bfd).xvec)._bfd_print_symbol)
                            .expect("non-null function pointer"),
                    ))
                        .expect(
                            "non-null function pointer",
                        )(
                        cur_bfd,
                        stdout as *mut libc::c_void,
                        *current,
                        bfd_print_symbol_all,
                    );
                }
                printf(b"\n\0" as *const u8 as *const libc::c_char);
            }
        }
        current = current.offset(1);
        current;
        count += 1;
        count;
    }
    printf(b"\n\n\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn dump_reloc_set(
    mut abfd: *mut bfd,
    mut sec: *mut asection,
    mut relpp: *mut *mut arelent,
    mut relcount: libc::c_long,
) {
    let mut p: *mut *mut arelent = 0 as *mut *mut arelent;
    let mut last_filename: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut last_functionname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut last_line: libc::c_uint = 0;
    let mut last_discriminator: libc::c_uint = 0;
    static mut width: libc::c_int = 0;
    if width == 0 as libc::c_int {
        let mut buf: [libc::c_char; 30] = [0; 30];
        bfd_sprintf_vma(abfd, buf.as_mut_ptr(), -(1 as libc::c_int) as bfd_vma);
        width = (strlen(buf.as_mut_ptr()))
            .wrapping_sub(7 as libc::c_int as libc::c_ulong) as libc::c_int;
    }
    printf(
        b"OFFSET %*s TYPE %*s VALUE \n\0" as *const u8 as *const libc::c_char,
        width,
        b"\0" as *const u8 as *const libc::c_char,
        12 as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char,
    );
    last_filename = 0 as *mut libc::c_char;
    last_functionname = 0 as *mut libc::c_char;
    last_line = 0 as libc::c_int as libc::c_uint;
    last_discriminator = 0 as libc::c_int as libc::c_uint;
    p = relpp;
    while relcount != 0 && !(*p).is_null() {
        let mut q: *mut arelent = *p;
        let mut filename: *const libc::c_char = 0 as *const libc::c_char;
        let mut functionname: *const libc::c_char = 0 as *const libc::c_char;
        let mut linenumber: libc::c_uint = 0;
        let mut discriminator: libc::c_uint = 0;
        let mut sym_name: *const libc::c_char = 0 as *const libc::c_char;
        let mut section_name: *const libc::c_char = 0 as *const libc::c_char;
        let mut addend2: bfd_vma = 0 as libc::c_int as bfd_vma;
        if !(start_address != -(1 as libc::c_int) as bfd_vma
            && (*q).address < start_address)
        {
            if !(stop_address != -(1 as libc::c_int) as bfd_vma
                && (*q).address > stop_address)
            {
                if with_line_numbers != 0 && !sec.is_null()
                    && (Some(
                        ((*(*abfd).xvec)._bfd_find_nearest_line)
                            .expect("non-null function pointer"),
                    ))
                        .expect(
                            "non-null function pointer",
                        )(
                        abfd,
                        syms,
                        sec,
                        (*q).address,
                        &mut filename,
                        &mut functionname,
                        &mut linenumber,
                        &mut discriminator,
                    ) as libc::c_int != 0
                {
                    if !functionname.is_null()
                        && (last_functionname.is_null()
                            || strcmp(functionname, last_functionname)
                                != 0 as libc::c_int)
                    {
                        printf(
                            b"%s():\n\0" as *const u8 as *const libc::c_char,
                            sanitize_string(functionname),
                        );
                        if !last_functionname.is_null() {
                            free(last_functionname as *mut libc::c_void);
                        }
                        last_functionname = xstrdup(functionname);
                    }
                    if linenumber > 0 as libc::c_int as libc::c_uint
                        && (linenumber != last_line
                            || !filename.is_null() && !last_filename.is_null()
                                && filename_cmp(filename, last_filename) != 0 as libc::c_int
                            || discriminator != last_discriminator)
                    {
                        if discriminator > 0 as libc::c_int as libc::c_uint {
                            printf(
                                b"%s:%u\n\0" as *const u8 as *const libc::c_char,
                                if filename.is_null() {
                                    b"???\0" as *const u8 as *const libc::c_char
                                } else {
                                    sanitize_string(filename)
                                },
                                linenumber,
                            );
                        } else {
                            printf(
                                b"%s:%u (discriminator %u)\n\0" as *const u8
                                    as *const libc::c_char,
                                if filename.is_null() {
                                    b"???\0" as *const u8 as *const libc::c_char
                                } else {
                                    sanitize_string(filename)
                                },
                                linenumber,
                                discriminator,
                            );
                        }
                        last_line = linenumber;
                        last_discriminator = discriminator;
                        if !last_filename.is_null() {
                            free(last_filename as *mut libc::c_void);
                        }
                        if filename.is_null() {
                            last_filename = 0 as *mut libc::c_char;
                        } else {
                            last_filename = xstrdup(filename);
                        }
                    }
                }
                if !((*q).sym_ptr_ptr).is_null() && !(*(*q).sym_ptr_ptr).is_null() {
                    sym_name = (**(*q).sym_ptr_ptr).name;
                    section_name = (*(**(*q).sym_ptr_ptr).section).name;
                } else {
                    sym_name = 0 as *const libc::c_char;
                    section_name = 0 as *const libc::c_char;
                }
                bfd_fprintf_vma(abfd, stdout as *mut libc::c_void, (*q).address);
                if ((*q).howto).is_null() {
                    printf(b" *unknown*         \0" as *const u8 as *const libc::c_char);
                } else if !((*(*q).howto).name).is_null() {
                    let mut name: *const libc::c_char = (*(*q).howto).name;
                    if (*(*abfd).xvec).flavour as libc::c_uint
                        == bfd_target_elf_flavour as libc::c_int as libc::c_uint
                        && (*((*(*abfd).tdata.elf_obj_data).elf_header).as_mut_ptr())
                            .e_machine as libc::c_int == 43 as libc::c_int
                        && relcount > 1 as libc::c_int as libc::c_long
                        && strcmp(
                            (*(*q).howto).name,
                            b"R_SPARC_LO10\0" as *const u8 as *const libc::c_char,
                        ) == 0
                    {
                        let mut q2: *mut arelent = *p.offset(1 as libc::c_int as isize);
                        if !q2.is_null() && !((*q2).howto).is_null()
                            && (*q).address == (*q2).address
                            && strcmp(
                                (*(*q2).howto).name,
                                b"R_SPARC_13\0" as *const u8 as *const libc::c_char,
                            ) == 0
                        {
                            name = b"R_SPARC_OLO10\0" as *const u8
                                as *const libc::c_char;
                            addend2 = (*q2).addend;
                            p = p.offset(1);
                            p;
                        }
                    }
                    printf(b" %-16s  \0" as *const u8 as *const libc::c_char, name);
                } else {
                    printf(
                        b" %-16d  \0" as *const u8 as *const libc::c_char,
                        (*(*q).howto).type_0,
                    );
                }
                if !sym_name.is_null() {
                    objdump_print_symname(
                        abfd,
                        0 as *mut disassemble_info,
                        *(*q).sym_ptr_ptr,
                    );
                } else {
                    if section_name.is_null() {
                        section_name = b"*unknown*\0" as *const u8
                            as *const libc::c_char;
                    }
                    printf(
                        b"[%s]\0" as *const u8 as *const libc::c_char,
                        sanitize_string(section_name),
                    );
                }
                if (*q).addend != 0 {
                    let mut addend: bfd_signed_vma = (*q).addend as bfd_signed_vma;
                    if addend < 0 as libc::c_int as libc::c_long {
                        printf(b"-0x\0" as *const u8 as *const libc::c_char);
                        addend = -addend;
                    } else {
                        printf(b"+0x\0" as *const u8 as *const libc::c_char);
                    }
                    bfd_fprintf_vma(
                        abfd,
                        stdout as *mut libc::c_void,
                        addend as bfd_vma,
                    );
                }
                if addend2 != 0 {
                    printf(b"+0x\0" as *const u8 as *const libc::c_char);
                    bfd_fprintf_vma(abfd, stdout as *mut libc::c_void, addend2);
                }
                printf(b"\n\0" as *const u8 as *const libc::c_char);
            }
        }
        p = p.offset(1);
        p;
        relcount -= 1;
        relcount;
    }
    if !last_filename.is_null() {
        free(last_filename as *mut libc::c_void);
    }
    if !last_functionname.is_null() {
        free(last_functionname as *mut libc::c_void);
    }
}
unsafe extern "C" fn dump_relocs_in_section(
    mut abfd: *mut bfd,
    mut section: *mut asection,
    mut _dummy: *mut libc::c_void,
) {
    let mut relpp: *mut *mut arelent = 0 as *mut *mut arelent;
    let mut relcount: libc::c_long = 0;
    let mut relsize: libc::c_long = 0;
    if bfd_is_abs_section(section) as libc::c_int != 0
        || bfd_is_und_section(section) as libc::c_int != 0
        || bfd_is_com_section(section) as libc::c_int != 0 || !process_section_p(section)
        || (*section).flags & 0x4 as libc::c_int as libc::c_uint
            == 0 as libc::c_int as libc::c_uint
    {
        return;
    }
    printf(
        b"RELOCATION RECORDS FOR [%s]:\0" as *const u8 as *const libc::c_char,
        sanitize_string((*section).name),
    );
    relsize = bfd_get_reloc_upper_bound(abfd, section);
    if relsize == 0 as libc::c_int as libc::c_long {
        printf(b" (none)\n\n\0" as *const u8 as *const libc::c_char);
        return;
    }
    if relsize < 0 as libc::c_int as libc::c_long {
        relcount = relsize;
    } else {
        relpp = xmalloc(relsize as size_t) as *mut *mut arelent;
        relcount = bfd_canonicalize_reloc(abfd, section, relpp, syms);
    }
    if relcount < 0 as libc::c_int as libc::c_long {
        printf(b"\n\0" as *const u8 as *const libc::c_char);
        non_fatal(
            dcgettext(
                0 as *const libc::c_char,
                b"failed to read relocs in: %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            sanitize_string(bfd_get_filename(abfd)),
        );
        bfd_fatal(
            dcgettext(
                0 as *const libc::c_char,
                b"error message was\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    } else if relcount == 0 as libc::c_int as libc::c_long {
        printf(b" (none)\n\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\n\0" as *const u8 as *const libc::c_char);
        dump_reloc_set(abfd, section, relpp, relcount);
        printf(b"\n\n\0" as *const u8 as *const libc::c_char);
    }
    free(relpp as *mut libc::c_void);
}
unsafe extern "C" fn dump_relocs(mut abfd: *mut bfd) {
    bfd_map_over_sections(
        abfd,
        Some(
            dump_relocs_in_section
                as unsafe extern "C" fn(*mut bfd, *mut asection, *mut libc::c_void) -> (),
        ),
        0 as *mut libc::c_void,
    );
}
unsafe extern "C" fn dump_dynamic_relocs(mut abfd: *mut bfd) {
    let mut relsize: libc::c_long = 0;
    let mut relpp: *mut *mut arelent = 0 as *mut *mut arelent;
    let mut relcount: libc::c_long = 0;
    relsize = (Some(
        ((*(*abfd).xvec)._bfd_get_dynamic_reloc_upper_bound)
            .expect("non-null function pointer"),
    ))
        .expect("non-null function pointer")(abfd);
    if relsize < 0 as libc::c_int as libc::c_long {
        bfd_fatal(bfd_get_filename(abfd));
    }
    printf(b"DYNAMIC RELOCATION RECORDS\0" as *const u8 as *const libc::c_char);
    if relsize == 0 as libc::c_int as libc::c_long {
        printf(b" (none)\n\n\0" as *const u8 as *const libc::c_char);
    } else {
        relpp = xmalloc(relsize as size_t) as *mut *mut arelent;
        relcount = (Some(
            ((*(*abfd).xvec)._bfd_canonicalize_dynamic_reloc)
                .expect("non-null function pointer"),
        ))
            .expect("non-null function pointer")(abfd, relpp, dynsyms);
        if relcount < 0 as libc::c_int as libc::c_long {
            bfd_fatal(bfd_get_filename(abfd));
        } else if relcount == 0 as libc::c_int as libc::c_long {
            printf(b" (none)\n\n\0" as *const u8 as *const libc::c_char);
        } else {
            printf(b"\n\0" as *const u8 as *const libc::c_char);
            dump_reloc_set(abfd, 0 as *mut asection, relpp, relcount);
            printf(b"\n\n\0" as *const u8 as *const libc::c_char);
        }
        free(relpp as *mut libc::c_void);
    };
}
unsafe extern "C" fn add_include_path(mut path: *const libc::c_char) {
    if *path.offset(0 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int {
        return;
    }
    include_path_count += 1;
    include_path_count;
    include_paths = xrealloc(
        include_paths as *mut libc::c_void,
        (include_path_count as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*const libc::c_char>() as libc::c_ulong),
    ) as *mut *const libc::c_char;
    let ref mut fresh16 = *include_paths
        .offset((include_path_count - 1 as libc::c_int) as isize);
    *fresh16 = path;
}
unsafe extern "C" fn adjust_addresses(
    mut _abfd: *mut bfd,
    mut section: *mut asection,
    mut arg: *mut libc::c_void,
) {
    if (*section).flags & 0x2000 as libc::c_int as libc::c_uint
        == 0 as libc::c_int as libc::c_uint
    {
        let mut has_reloc_p: *mut bool = arg as *mut bool;
        (*section)
            .vma = ((*section).vma as libc::c_ulong).wrapping_add(adjust_section_vma)
            as bfd_vma as bfd_vma;
        if *has_reloc_p {
            (*section)
                .lma = ((*section).lma as libc::c_ulong).wrapping_add(adjust_section_vma)
                as bfd_vma as bfd_vma;
        }
    }
}
unsafe extern "C" fn sign_extend_address(
    mut _abfd: *mut bfd,
    mut vma: bfd_vma,
    mut arch_size: libc::c_uint,
) -> bfd_vma {
    let mut mask: bfd_vma = 0;
    mask = (1 as libc::c_int as bfd_vma)
        << arch_size.wrapping_sub(1 as libc::c_int as libc::c_uint);
    return (vma
        & (mask << 1 as libc::c_int).wrapping_sub(1 as libc::c_int as libc::c_ulong)
        ^ mask)
        .wrapping_sub(mask);
}
unsafe extern "C" fn dump_bfd(mut abfd: *mut bfd, mut is_mainfile: bool) {
    let mut bed: *const elf_backend_data = 0 as *const elf_backend_data;
    if bfd_big_endian(abfd) {
        byte_get = Some(
            byte_get_big_endian
                as unsafe extern "C" fn(*const libc::c_uchar, libc::c_uint) -> elf_vma,
        );
    } else if bfd_little_endian(abfd) {
        byte_get = Some(
            byte_get_little_endian
                as unsafe extern "C" fn(*const libc::c_uchar, libc::c_uint) -> elf_vma,
        );
    } else {
        byte_get = None;
    }
    if is_mainfile {
        load_separate_debug_files(abfd as *mut libc::c_void, bfd_get_filename(abfd));
        if do_follow_links != 0 {
            let mut i: *mut separate_info = 0 as *mut separate_info;
            i = first_separate_info;
            while !i.is_null() {
                dump_bfd((*i).handle as *mut bfd, 0 as libc::c_int != 0);
                i = (*i).next;
            }
        }
    }
    if bfd_get_flavour(abfd) as libc::c_uint
        == bfd_target_elf_flavour as libc::c_int as libc::c_uint
        && {
            bed = (*(*abfd).xvec).backend_data as *const elf_backend_data;
            !bed.is_null()
        } && (*bed).sign_extend_vma() as libc::c_int != 0
    {
        start_address = sign_extend_address(
            abfd,
            start_address,
            (*(*bed).s).arch_size as libc::c_uint,
        );
        stop_address = sign_extend_address(
            abfd,
            stop_address,
            (*(*bed).s).arch_size as libc::c_uint,
        );
    }
    if adjust_section_vma != 0 as libc::c_int as libc::c_ulong {
        let mut has_reloc: bool = (*abfd).flags & 0x1 as libc::c_int as libc::c_uint
            != 0;
        bfd_map_over_sections(
            abfd,
            Some(
                adjust_addresses
                    as unsafe extern "C" fn(
                        *mut bfd,
                        *mut asection,
                        *mut libc::c_void,
                    ) -> (),
            ),
            &mut has_reloc as *mut bool as *mut libc::c_void,
        );
    }
    if !is_mainfile && process_links == 0 {
        return;
    }
    if dump_debugging_tags == 0 && suppress_bfd_header == 0 {
        printf(
            dcgettext(
                0 as *const libc::c_char,
                b"\n%s:     file format %s\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            sanitize_string(bfd_get_filename(abfd)),
            (*(*abfd).xvec).name,
        );
    }
    if dump_ar_hdrs != 0 {
        print_arelt_descr(stdout, abfd, 1 as libc::c_int != 0, 0 as libc::c_int != 0);
    }
    if dump_file_header {
        dump_bfd_header(abfd);
    }
    if dump_private_headers != 0 {
        dump_bfd_private_header(abfd);
    }
    if !dump_private_options.is_null() {
        dump_target_specific(abfd);
    }
    if dump_debugging_tags == 0 && suppress_bfd_header == 0 {
        putchar('\n' as i32);
    }
    if dump_symtab != 0 || dump_reloc_info != 0 || disassemble as libc::c_int != 0
        || dump_debugging != 0 || dump_dwarf_section_info != 0
    {
        syms = slurp_symtab(abfd);
        if do_follow_links != 0 && is_mainfile as libc::c_int != 0 {
            let mut i_0: *mut separate_info = 0 as *mut separate_info;
            i_0 = first_separate_info;
            while !i_0.is_null() {
                let mut extra_syms: *mut *mut asymbol = 0 as *mut *mut asymbol;
                let mut old_symcount: libc::c_long = symcount;
                extra_syms = slurp_symtab((*i_0).handle as *mut bfd);
                if !extra_syms.is_null() {
                    if old_symcount == 0 as libc::c_int as libc::c_long {
                        syms = extra_syms;
                    } else {
                        syms = xrealloc(
                            syms as *mut libc::c_void,
                            ((symcount + old_symcount + 1 as libc::c_int as libc::c_long)
                                as libc::c_ulong)
                                .wrapping_mul(
                                    ::core::mem::size_of::<*mut asymbol>() as libc::c_ulong,
                                ),
                        ) as *mut *mut asymbol;
                        memcpy(
                            syms.offset(old_symcount as isize) as *mut libc::c_void,
                            extra_syms as *const libc::c_void,
                            ((symcount + 1 as libc::c_int as libc::c_long)
                                as libc::c_ulong)
                                .wrapping_mul(
                                    ::core::mem::size_of::<*mut asymbol>() as libc::c_ulong,
                                ),
                        );
                    }
                }
                symcount += old_symcount;
                i_0 = (*i_0).next;
            }
        }
    }
    if dump_section_headers != 0 {
        dump_headers(abfd);
    }
    if dump_dynamic_symtab != 0 || dump_dynamic_reloc_info != 0
        || disassemble as libc::c_int != 0
            && (Some(
                ((*(*abfd).xvec)._bfd_get_dynamic_symtab_upper_bound)
                    .expect("non-null function pointer"),
            ))
                .expect("non-null function pointer")(abfd)
                > 0 as libc::c_int as libc::c_long
    {
        dynsyms = slurp_dynamic_symtab(abfd);
    }
    if disassemble {
        synthcount = (Some(
            ((*(*abfd).xvec)._bfd_get_synthetic_symtab)
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(abfd, symcount, syms, dynsymcount, dynsyms, &mut synthsyms);
        if synthcount < 0 as libc::c_int as libc::c_long {
            synthcount = 0 as libc::c_int as libc::c_long;
        }
    }
    if dump_symtab != 0 {
        dump_symbols(abfd, 0 as libc::c_int != 0);
    }
    if dump_dynamic_symtab != 0 {
        dump_symbols(abfd, 1 as libc::c_int != 0);
    }
    if dump_dwarf_section_info != 0 {
        dump_dwarf(abfd);
    }
    if dump_ctf_section_info != 0 {
        dump_ctf(abfd, dump_ctf_section_name, dump_ctf_parent_name);
    }
    if dump_stab_section_info != 0 {
        dump_stabs(abfd);
    }
    if dump_reloc_info != 0 && !disassemble {
        dump_relocs(abfd);
    }
    if dump_dynamic_reloc_info != 0 && !disassemble {
        dump_dynamic_relocs(abfd);
    }
    if dump_section_contents != 0 {
        dump_data(abfd);
    }
    if disassemble {
        disassemble_data(abfd);
    }
    if dump_debugging != 0 {
        let mut dhandle: *mut libc::c_void = 0 as *mut libc::c_void;
        dhandle = read_debugging_info(abfd, syms, symcount, 1 as libc::c_int != 0);
        if !dhandle.is_null() {
            if !print_debugging_info(
                stdout,
                dhandle,
                abfd,
                syms,
                Some(
                    bfd_demangle
                        as unsafe extern "C" fn(
                            *mut bfd,
                            *const libc::c_char,
                            libc::c_int,
                        ) -> *mut libc::c_char,
                ),
                dump_debugging_tags != 0 as libc::c_int,
            ) {
                non_fatal(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s: printing debugging information failed\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    bfd_get_filename(abfd),
                );
                exit_status = 1 as libc::c_int;
            }
            free(dhandle);
        } else if dump_dwarf_section_info == 0 {
            dwarf_select_sections_all();
            dump_dwarf(abfd);
        }
    }
    if !syms.is_null() {
        free(syms as *mut libc::c_void);
        syms = 0 as *mut *mut asymbol;
    }
    if !dynsyms.is_null() {
        free(dynsyms as *mut libc::c_void);
        dynsyms = 0 as *mut *mut asymbol;
    }
    if !synthsyms.is_null() {
        free(synthsyms as *mut libc::c_void);
        synthsyms = 0 as *mut asymbol;
    }
    symcount = 0 as libc::c_int as libc::c_long;
    dynsymcount = 0 as libc::c_int as libc::c_long;
    synthcount = 0 as libc::c_int as libc::c_long;
    if is_mainfile {
        free_debug_memory();
    }
}
unsafe extern "C" fn display_object_bfd(mut abfd: *mut bfd) {
    let mut matching: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    if bfd_check_format_matches(abfd, bfd_object, &mut matching) {
        dump_bfd(abfd, 1 as libc::c_int != 0);
        return;
    }
    if bfd_get_error() as libc::c_uint
        == bfd_error_file_ambiguously_recognized as libc::c_int as libc::c_uint
    {
        nonfatal(bfd_get_filename(abfd));
        list_matching_formats(matching);
        free(matching as *mut libc::c_void);
        return;
    }
    if bfd_get_error() as libc::c_uint
        != bfd_error_file_not_recognized as libc::c_int as libc::c_uint
    {
        nonfatal(bfd_get_filename(abfd));
        return;
    }
    if bfd_check_format_matches(abfd, bfd_core, &mut matching) {
        dump_bfd(abfd, 1 as libc::c_int != 0);
        return;
    }
    nonfatal(bfd_get_filename(abfd));
    if bfd_get_error() as libc::c_uint
        == bfd_error_file_ambiguously_recognized as libc::c_int as libc::c_uint
    {
        list_matching_formats(matching);
        free(matching as *mut libc::c_void);
    }
}
unsafe extern "C" fn display_any_bfd(mut file: *mut bfd, mut level: libc::c_int) {
    if dump_section_contents == 0 {
        (*file).flags |= 0x8000 as libc::c_int as libc::c_uint;
    }
    if bfd_check_format(file, bfd_archive) {
        let mut arfile: *mut bfd = 0 as *mut bfd;
        let mut last_arfile: *mut bfd = 0 as *mut bfd;
        if level == 0 as libc::c_int {
            printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"In archive %s:\n\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                sanitize_string(bfd_get_filename(file)),
            );
        } else if level > 100 as libc::c_int {
            fatal(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Archive nesting is too deep\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        } else {
            printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"In nested archive %s:\n\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                sanitize_string(bfd_get_filename(file)),
            );
        }
        loop {
            bfd_set_error(bfd_error_no_error);
            arfile = bfd_openr_next_archived_file(file, arfile);
            if arfile.is_null() {
                if bfd_get_error() as libc::c_uint
                    != bfd_error_no_more_archived_files as libc::c_int as libc::c_uint
                {
                    nonfatal(bfd_get_filename(file));
                }
                break;
            } else {
                display_any_bfd(arfile, level + 1 as libc::c_int);
                if !last_arfile.is_null() {
                    bfd_close(last_arfile);
                    if arfile == last_arfile {
                        last_arfile = 0 as *mut bfd;
                        break;
                    }
                }
                last_arfile = arfile;
            }
        }
        if !last_arfile.is_null() {
            bfd_close(last_arfile);
        }
    } else {
        display_object_bfd(file);
    };
}
unsafe extern "C" fn display_file(
    mut filename: *mut libc::c_char,
    mut target: *mut libc::c_char,
    mut last_file: bool,
) {
    let mut file: *mut bfd = 0 as *mut bfd;
    if get_file_size(filename) < 1 as libc::c_int as libc::c_long {
        exit_status = 1 as libc::c_int;
        return;
    }
    file = bfd_openr(filename, target);
    if file.is_null() {
        nonfatal(filename);
        return;
    }
    display_any_bfd(file, 0 as libc::c_int);
    if !last_file {
        bfd_close(file);
    } else {
        bfd_close_all_done(file);
    };
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut c: libc::c_int = 0;
    let mut target: *mut libc::c_char = default_target;
    let mut seenflag: bool = 0 as libc::c_int != 0;
    setlocale(5 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
    setlocale(0 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
    bindtextdomain(
        b"binutils\0" as *const u8 as *const libc::c_char,
        b"/usr/local/share/locale\0" as *const u8 as *const libc::c_char,
    );
    textdomain(b"binutils\0" as *const u8 as *const libc::c_char);
    program_name = *argv;
    xmalloc_set_program_name(program_name);
    bfd_set_error_program_name(program_name);
    expandargv(&mut argc, &mut argv);
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
    loop {
        c = getopt_long(
            argc,
            argv,
            b"pP:ib:m:M:VvCdDlfFaHhrRtTxsSI:j:wE:zgeGW::\0" as *const u8
                as *const libc::c_char,
            long_options.as_mut_ptr(),
            0 as *mut libc::c_int,
        );
        if !(c != -(1 as libc::c_int)) {
            break;
        }
        match c {
            0 => {}
            109 => {
                machine = optarg;
            }
            77 => {
                let mut options: *mut libc::c_char = 0 as *mut libc::c_char;
                if !disassembler_options.is_null() {
                    options = concat(
                        disassembler_options,
                        b",\0" as *const u8 as *const libc::c_char,
                        optarg,
                        0 as *mut libc::c_void as *const libc::c_char,
                    );
                } else {
                    options = optarg;
                }
                disassembler_options = remove_whitespace_and_extra_commas(options);
            }
            106 => {
                add_only(optarg);
            }
            70 => {
                display_file_offsets = 1 as libc::c_int != 0;
            }
            108 => {
                with_line_numbers = 1 as libc::c_int;
            }
            98 => {
                target = optarg;
            }
            67 => {
                do_demangle = 1 as libc::c_int;
                if !optarg.is_null() {
                    let mut style: demangling_styles = unknown_demangling;
                    style = cplus_demangle_name_to_style(optarg);
                    if style as libc::c_int == unknown_demangling as libc::c_int {
                        fatal(
                            dcgettext(
                                0 as *const libc::c_char,
                                b"unknown demangling style `%s'\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            optarg,
                        );
                    }
                    cplus_demangle_set_style(style);
                }
            }
            161 => {
                demangle_flags &= !((1 as libc::c_int) << 18 as libc::c_int);
            }
            162 => {
                demangle_flags |= (1 as libc::c_int) << 18 as libc::c_int;
            }
            119 => {
                wide_output = 1 as libc::c_int;
                do_wide = wide_output;
            }
            157 => {
                adjust_section_vma = parse_vma(
                    optarg,
                    b"--adjust-vma\0" as *const u8 as *const libc::c_char,
                );
            }
            151 => {
                start_address = parse_vma(
                    optarg,
                    b"--start-address\0" as *const u8 as *const libc::c_char,
                );
                if stop_address != -(1 as libc::c_int) as bfd_vma
                    && stop_address <= start_address
                {
                    fatal(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"error: the start address should be before the end address\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                }
            }
            152 => {
                stop_address = parse_vma(
                    optarg,
                    b"--stop-address\0" as *const u8 as *const libc::c_char,
                );
                if start_address != -(1 as libc::c_int) as bfd_vma
                    && stop_address <= start_address
                {
                    fatal(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"error: the stop address should be after the start address\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                }
            }
            154 => {
                prefix = optarg;
                prefix_length = strlen(prefix);
                while *prefix
                    .offset(
                        prefix_length.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            as isize,
                    ) as libc::c_int == '/' as i32
                    || *prefix
                        .offset(
                            prefix_length.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                as isize,
                        ) as libc::c_int == '\\' as i32 && 0 as libc::c_int != 0
                {
                    prefix_length = prefix_length.wrapping_sub(1);
                    prefix_length;
                }
            }
            155 => {
                prefix_strip = atoi(optarg);
                if prefix_strip < 0 as libc::c_int {
                    fatal(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"error: prefix strip must be non-negative\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                }
            }
            156 => {
                insn_width = strtoul(
                    optarg,
                    0 as *mut *mut libc::c_char,
                    0 as libc::c_int,
                ) as libc::c_int;
                if insn_width <= 0 as libc::c_int {
                    fatal(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"error: instruction width must be positive\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                }
            }
            163 => {
                unwind_inlines = 1 as libc::c_int != 0;
            }
            167 => {
                visualize_jumps = 1 as libc::c_int != 0;
                color_output = 0 as libc::c_int != 0;
                extended_color_output = 0 as libc::c_int != 0;
                if !optarg.is_null() {
                    if strcmp(optarg, b"color\0" as *const u8 as *const libc::c_char)
                        == 0 as libc::c_int
                    {
                        color_output = 1 as libc::c_int != 0;
                    } else if strcmp(
                        optarg,
                        b"extended-color\0" as *const u8 as *const libc::c_char,
                    ) == 0 as libc::c_int
                    {
                        color_output = 1 as libc::c_int != 0;
                        extended_color_output = 1 as libc::c_int != 0;
                    } else if strcmp(
                        optarg,
                        b"off\0" as *const u8 as *const libc::c_char,
                    ) == 0 as libc::c_int
                    {
                        visualize_jumps = 0 as libc::c_int != 0;
                    } else {
                        nonfatal(
                            dcgettext(
                                0 as *const libc::c_char,
                                b"unrecognized argument to --visualize-option\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                    }
                }
            }
            69 => {
                if strcmp(optarg, b"B\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    endian = BFD_ENDIAN_BIG;
                } else if strcmp(optarg, b"L\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    endian = BFD_ENDIAN_LITTLE;
                } else {
                    nonfatal(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"unrecognized -E option\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    usage(stderr, 1 as libc::c_int);
                }
            }
            150 => {
                if strncmp(
                    optarg,
                    b"big\0" as *const u8 as *const libc::c_char,
                    strlen(optarg),
                ) == 0 as libc::c_int
                {
                    endian = BFD_ENDIAN_BIG;
                } else if strncmp(
                    optarg,
                    b"little\0" as *const u8 as *const libc::c_char,
                    strlen(optarg),
                ) == 0 as libc::c_int
                {
                    endian = BFD_ENDIAN_LITTLE;
                } else {
                    non_fatal(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"unrecognized --endian type `%s'\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        optarg,
                    );
                    exit_status = 1 as libc::c_int;
                    usage(stderr, 1 as libc::c_int);
                }
            }
            102 => {
                dump_file_header = 1 as libc::c_int != 0;
                seenflag = 1 as libc::c_int != 0;
            }
            105 => {
                formats_info = 1 as libc::c_int != 0;
                seenflag = 1 as libc::c_int != 0;
            }
            73 => {
                add_include_path(optarg);
            }
            112 => {
                dump_private_headers = 1 as libc::c_int;
                seenflag = 1 as libc::c_int != 0;
            }
            80 => {
                dump_private_options = optarg;
                seenflag = 1 as libc::c_int != 0;
            }
            120 => {
                dump_private_headers = 1 as libc::c_int;
                dump_symtab = 1 as libc::c_int;
                dump_reloc_info = 1 as libc::c_int;
                dump_file_header = 1 as libc::c_int != 0;
                dump_ar_hdrs = 1 as libc::c_int;
                dump_section_headers = 1 as libc::c_int;
                seenflag = 1 as libc::c_int != 0;
            }
            116 => {
                dump_symtab = 1 as libc::c_int;
                seenflag = 1 as libc::c_int != 0;
            }
            84 => {
                dump_dynamic_symtab = 1 as libc::c_int;
                seenflag = 1 as libc::c_int != 0;
            }
            100 => {
                disassemble = 1 as libc::c_int != 0;
                seenflag = 1 as libc::c_int != 0;
                disasm_sym = optarg;
            }
            122 => {
                disassemble_zeroes = 1 as libc::c_int;
            }
            68 => {
                disassemble = 1 as libc::c_int != 0;
                disassemble_all = 1 as libc::c_int != 0;
                seenflag = 1 as libc::c_int != 0;
            }
            83 => {
                disassemble = 1 as libc::c_int != 0;
                with_source_code = 1 as libc::c_int != 0;
                seenflag = 1 as libc::c_int != 0;
            }
            164 => {
                disassemble = 1 as libc::c_int != 0;
                with_source_code = 1 as libc::c_int != 0;
                seenflag = 1 as libc::c_int != 0;
                if !optarg.is_null() {
                    source_comment = xstrdup(sanitize_string(optarg));
                } else {
                    source_comment = xstrdup(
                        b"# \0" as *const u8 as *const libc::c_char,
                    );
                }
            }
            103 => {
                dump_debugging = 1 as libc::c_int;
                seenflag = 1 as libc::c_int != 0;
            }
            101 => {
                dump_debugging = 1 as libc::c_int;
                dump_debugging_tags = 1 as libc::c_int;
                do_demangle = 1 as libc::c_int;
                seenflag = 1 as libc::c_int != 0;
            }
            76 => {
                process_links = 1 as libc::c_int;
                do_follow_links = 1 as libc::c_int;
            }
            87 => {
                dump_dwarf_section_info = 1 as libc::c_int;
                seenflag = 1 as libc::c_int != 0;
                if !optarg.is_null() {
                    dwarf_select_sections_by_letters(optarg);
                } else {
                    dwarf_select_sections_all();
                }
            }
            153 => {
                dump_dwarf_section_info = 1 as libc::c_int;
                seenflag = 1 as libc::c_int != 0;
                if !optarg.is_null() {
                    dwarf_select_sections_by_names(optarg);
                } else {
                    dwarf_select_sections_all();
                }
            }
            158 => {
                let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
                dwarf_cutoff_level = strtoul(optarg, &mut cp, 0 as libc::c_int)
                    as libc::c_int;
            }
            160 => {
                let mut cp_0: *mut libc::c_char = 0 as *mut libc::c_char;
                dwarf_start_die = strtoul(optarg, &mut cp_0, 0 as libc::c_int);
                suppress_bfd_header = 1 as libc::c_int;
            }
            159 => {
                dwarf_check = 1 as libc::c_int;
            }
            165 => {
                dump_ctf_section_info = 1 as libc::c_int;
                dump_ctf_section_name = xstrdup(optarg);
                seenflag = 1 as libc::c_int != 0;
            }
            166 => {
                dump_ctf_parent_name = xstrdup(optarg);
            }
            71 => {
                dump_stab_section_info = 1 as libc::c_int;
                seenflag = 1 as libc::c_int != 0;
            }
            115 => {
                dump_section_contents = 1 as libc::c_int;
                seenflag = 1 as libc::c_int != 0;
            }
            114 => {
                dump_reloc_info = 1 as libc::c_int;
                seenflag = 1 as libc::c_int != 0;
            }
            82 => {
                dump_dynamic_reloc_info = 1 as libc::c_int;
                seenflag = 1 as libc::c_int != 0;
            }
            97 => {
                dump_ar_hdrs = 1 as libc::c_int;
                seenflag = 1 as libc::c_int != 0;
            }
            104 => {
                dump_section_headers = 1 as libc::c_int;
                seenflag = 1 as libc::c_int != 0;
            }
            118 | 86 => {
                show_version = 1 as libc::c_int;
                seenflag = 1 as libc::c_int != 0;
            }
            72 => {
                usage(stdout, 0 as libc::c_int);
            }
            _ => {
                usage(stderr, 1 as libc::c_int);
            }
        }
    }
    if show_version != 0 {
        print_version(b"objdump\0" as *const u8 as *const libc::c_char);
    }
    if !seenflag {
        usage(stderr, 2 as libc::c_int);
    }
    if formats_info {
        exit_status = display_info();
    } else if optind == argc {
        display_file(
            b"a.out\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            target,
            1 as libc::c_int != 0,
        );
    } else {
        while optind < argc {
            display_file(
                *argv.offset(optind as isize),
                target,
                optind == argc - 1 as libc::c_int,
            );
            optind += 1;
            optind;
        }
    }
    free_only_list();
    free(dump_ctf_section_name as *mut libc::c_void);
    free(dump_ctf_parent_name as *mut libc::c_void);
    free(source_comment as *mut libc::c_void);
    return exit_status;
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
