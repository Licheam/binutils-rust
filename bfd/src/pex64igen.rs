use ::libc;
use ::c2rust_bitfields;
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
    pub type elf_obj_tdata;
    pub type tekhex_data_struct;
    pub type ihex_data_struct;
    pub type verilog_data_struct;
    pub type srec_data_struct;
    pub type ecoff_tdata;
    pub type elf_internal_sym;
    pub type elf_strtab_hash;
    pub type aout_data_struct;
    pub type bfd_strtab_hash;
    fn free(_: *mut libc::c_void);
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
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
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn time(__timer: *mut time_t) -> time_t;
    fn ctime(__timer: *const time_t) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn bfd_fprintf_vma(_: *mut bfd, _: *mut libc::c_void, _: bfd_vma);
    fn bfd_bread(_: *mut libc::c_void, _: bfd_size_type, _: *mut bfd) -> bfd_size_type;
    fn bfd_bwrite(
        _: *const libc::c_void,
        _: bfd_size_type,
        _: *mut bfd,
    ) -> bfd_size_type;
    fn bfd_seek(_: *mut bfd, _: file_ptr, _: libc::c_int) -> libc::c_int;
    fn bfd_getb32(_: *const libc::c_void) -> bfd_vma;
    fn bfd_getl32(_: *const libc::c_void) -> bfd_vma;
    fn bfd_getb16(_: *const libc::c_void) -> bfd_vma;
    fn bfd_getl16(_: *const libc::c_void) -> bfd_vma;
    fn bfd_putb32(_: bfd_vma, _: *mut libc::c_void);
    fn bfd_putl32(_: bfd_vma, _: *mut libc::c_void);
    fn bfd_putb16(_: bfd_vma, _: *mut libc::c_void);
    fn bfd_putl16(_: bfd_vma, _: *mut libc::c_void);
    fn bfd_alloc(abfd: *mut bfd, wanted: bfd_size_type) -> *mut libc::c_void;
    fn bfd_zalloc(abfd: *mut bfd, wanted: bfd_size_type) -> *mut libc::c_void;
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
    fn bfd_sections_find_if(
        abfd: *mut bfd,
        operation: Option::<
            unsafe extern "C" fn(*mut bfd, *mut asection, *mut libc::c_void) -> bool,
        >,
        obj: *mut libc::c_void,
    ) -> *mut asection;
    fn bfd_set_section_contents(
        abfd: *mut bfd,
        section: *mut asection,
        data: *const libc::c_void,
        offset: file_ptr,
        count: bfd_size_type,
    ) -> bool;
    fn bfd_get_section_contents(
        abfd: *mut bfd,
        section: *mut asection,
        location: *mut libc::c_void,
        offset: file_ptr,
        count: bfd_size_type,
    ) -> bool;
    fn bfd_malloc_and_get_section(
        abfd: *mut bfd,
        section: *mut asection,
        buf: *mut *mut bfd_byte,
    ) -> bool;
    fn bfd_set_error(error_tag: bfd_error_type);
    fn _bfd_error_handler(fmt: *const libc::c_char, _: ...);
    fn bfd_malloc(_: bfd_size_type) -> *mut libc::c_void;
    fn bfd_assert(_: *const libc::c_char, _: libc::c_int);
    fn bfd_realloc(MEM: *mut libc::c_void, SIZE: bfd_size_type) -> *mut libc::c_void;
    fn towlower(__wc: wint_t) -> wint_t;
    fn _bfd_coff_internal_syment_name(
        _: *mut bfd,
        _: *const internal_syment,
        _: *mut libc::c_char,
    ) -> *const libc::c_char;
    fn bfd_link_hash_lookup(
        _: *mut bfd_link_hash_table,
        _: *const libc::c_char,
        create: bool,
        copy: bool,
        follow: bool,
    ) -> *mut bfd_link_hash_entry;
    fn coff_get_symbol_info(_: *mut bfd, _: *mut asymbol, ret: *mut symbol_info);
}
pub type size_t = libc::c_ulong;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
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
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
pub type ptrdiff_t = libc::c_long;
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
pub type hashval_t = libc::c_uint;
pub type htab_hash = Option::<unsafe extern "C" fn(*const libc::c_void) -> hashval_t>;
pub type htab_eq = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
pub type htab_del = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
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
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
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
    pub link: C2RustUnnamed_32,
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
pub type bfd_vma = libc::c_ulong;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bfd_sym_chain {
    pub next: *mut bfd_sym_chain,
    pub name: *const libc::c_char,
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
    pub x_sym: C2RustUnnamed_23,
    pub x_file: C2RustUnnamed_21,
    pub x_scn: C2RustUnnamed_20,
    pub x_tv: C2RustUnnamed_19,
    pub x_csect: C2RustUnnamed_14,
    pub x_sect: C2RustUnnamed_13,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_13 {
    pub x_scnlen: libc::c_long,
    pub x_nreloc: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_14 {
    pub x_scnlen: C2RustUnnamed_15,
    pub x_parmhash: libc::c_long,
    pub x_snhash: libc::c_ushort,
    pub x_smtyp: libc::c_uchar,
    pub x_smclas: libc::c_uchar,
    pub x_stab: libc::c_long,
    pub x_snstab: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_15 {
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
    pub u: C2RustUnnamed_16,
    pub is_sym: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_16 {
    pub auxent: internal_auxent,
    pub syment: internal_syment,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct internal_syment {
    pub _n: C2RustUnnamed_17,
    pub n_value: bfd_vma,
    pub n_scnum: libc::c_int,
    pub n_flags: libc::c_ushort,
    pub n_type: libc::c_ushort,
    pub n_sclass: libc::c_uchar,
    pub n_numaux: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_17 {
    pub _n_name: [libc::c_char; 8],
    pub _n_n: C2RustUnnamed_18,
    pub _n_nptr: [*mut libc::c_char; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_18 {
    pub _n_zeroes: bfd_hostptr_t,
    pub _n_offset: bfd_hostptr_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_19 {
    pub x_tvfill: libc::c_long,
    pub x_tvlen: libc::c_ushort,
    pub x_tvran: [libc::c_ushort; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_20 {
    pub x_scnlen: libc::c_long,
    pub x_nreloc: libc::c_ushort,
    pub x_nlinno: libc::c_ushort,
    pub x_checksum: libc::c_ulong,
    pub x_associated: libc::c_ushort,
    pub x_comdat: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_21 {
    pub x_fname: [libc::c_char; 20],
    pub x_n: C2RustUnnamed_22,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_22 {
    pub x_zeroes: libc::c_long,
    pub x_offset: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_23 {
    pub x_tagndx: C2RustUnnamed_30,
    pub x_misc: C2RustUnnamed_28,
    pub x_fcnary: C2RustUnnamed_24,
    pub x_tvndx: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_24 {
    pub x_fcn: C2RustUnnamed_26,
    pub x_ary: C2RustUnnamed_25,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_25 {
    pub x_dimen: [libc::c_ushort; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_26 {
    pub x_lnnoptr: bfd_signed_vma,
    pub x_endndx: C2RustUnnamed_27,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_27 {
    pub l: libc::c_long,
    pub p: *mut coff_ptr_struct,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_28 {
    pub x_lnsz: C2RustUnnamed_29,
    pub x_fsize: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_29 {
    pub x_lnno: libc::c_ushort,
    pub x_size: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_30 {
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
pub type asymbol = bfd_symbol;
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
    pub build_id: C2RustUnnamed_31,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_31 {
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
pub struct artdata {
    pub first_file_filepos: file_ptr,
    pub cache: htab_t,
    pub archive_head: *mut bfd,
    pub symdefs: *mut carsym,
    pub symdef_count: symindex,
    pub extended_names: *mut libc::c_char,
    pub extended_names_size: bfd_size_type,
    pub armap_timestamp: libc::c_long,
    pub armap_datepos: file_ptr,
    pub tdata: *mut libc::c_void,
}
pub type symindex = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct carsym {
    pub name: *const libc::c_char,
    pub file_offset: file_ptr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_32 {
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
pub struct bfd_iovec {
    pub bread: Option::<
        unsafe extern "C" fn(*mut bfd, *mut libc::c_void, file_ptr) -> file_ptr,
    >,
    pub bwrite: Option::<
        unsafe extern "C" fn(*mut bfd, *const libc::c_void, file_ptr) -> file_ptr,
    >,
    pub btell: Option::<unsafe extern "C" fn(*mut bfd) -> file_ptr>,
    pub bseek: Option::<
        unsafe extern "C" fn(*mut bfd, file_ptr, libc::c_int) -> libc::c_int,
    >,
    pub bclose: Option::<unsafe extern "C" fn(*mut bfd) -> libc::c_int>,
    pub bflush: Option::<unsafe extern "C" fn(*mut bfd) -> libc::c_int>,
    pub bstat: Option::<unsafe extern "C" fn(*mut bfd, *mut stat) -> libc::c_int>,
    pub bmmap: Option::<
        unsafe extern "C" fn(
            *mut bfd,
            *mut libc::c_void,
            bfd_size_type,
            libc::c_int,
            libc::c_int,
            file_ptr,
            *mut *mut libc::c_void,
            *mut bfd_size_type,
        ) -> *mut libc::c_void,
    >,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct orl {
    pub name: *mut *mut libc::c_char,
    pub u: C2RustUnnamed_33,
    pub namidx: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_33 {
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
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct _bfd_window_internal {
    pub next: *mut _bfd_window_internal,
    pub data: *mut libc::c_void,
    pub size: bfd_size_type,
    #[bitfield(name = "refcount", ty = "libc::c_int", bits = "0..=30")]
    #[bitfield(name = "mapped", ty = "libc::c_uint", bits = "31..=31")]
    pub refcount_mapped: [u8; 4],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 4],
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stab_info {
    pub strings: *mut bfd_strtab_hash,
    pub includes: bfd_hash_table,
    pub stabstr: *mut bfd_section,
}
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
pub struct internal_extra_pe_filehdr {
    pub e_magic: libc::c_ushort,
    pub e_cblp: libc::c_ushort,
    pub e_cp: libc::c_ushort,
    pub e_crlc: libc::c_ushort,
    pub e_cparhdr: libc::c_ushort,
    pub e_minalloc: libc::c_ushort,
    pub e_maxalloc: libc::c_ushort,
    pub e_ss: libc::c_ushort,
    pub e_sp: libc::c_ushort,
    pub e_csum: libc::c_ushort,
    pub e_ip: libc::c_ushort,
    pub e_cs: libc::c_ushort,
    pub e_lfarlc: libc::c_ushort,
    pub e_ovno: libc::c_ushort,
    pub e_res: [libc::c_ushort; 4],
    pub e_oemid: libc::c_ushort,
    pub e_oeminfo: libc::c_ushort,
    pub e_res2: [libc::c_ushort; 10],
    pub e_lfanew: bfd_vma,
    pub dos_message: [libc::c_uint; 16],
    pub nt_signature: bfd_vma,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct internal_filehdr {
    pub pe: internal_extra_pe_filehdr,
    pub f_magic: libc::c_ushort,
    pub f_nscns: libc::c_uint,
    pub f_timdat: libc::c_long,
    pub f_symptr: bfd_vma,
    pub f_nsyms: libc::c_long,
    pub f_opthdr: libc::c_ushort,
    pub f_flags: libc::c_ushort,
    pub f_target_id: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct internal_IMAGE_DEBUG_DIRECTORY {
    pub Characteristics: libc::c_ulong,
    pub TimeDateStamp: libc::c_ulong,
    pub MajorVersion: libc::c_ushort,
    pub MinorVersion: libc::c_ushort,
    pub Type: libc::c_ulong,
    pub SizeOfData: libc::c_ulong,
    pub AddressOfRawData: libc::c_ulong,
    pub PointerToRawData: libc::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _CODEVIEW_INFO {
    pub CVSignature: libc::c_ulong,
    pub Signature: [libc::c_char; 16],
    pub SignatureLength: libc::c_uint,
    pub Age: libc::c_ulong,
}
pub type CODEVIEW_INFO = _CODEVIEW_INFO;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct internal_aouthdr {
    pub magic: libc::c_short,
    pub vstamp: libc::c_short,
    pub tsize: bfd_vma,
    pub dsize: bfd_vma,
    pub bsize: bfd_vma,
    pub entry: bfd_vma,
    pub text_start: bfd_vma,
    pub data_start: bfd_vma,
    pub o_toc: bfd_vma,
    pub o_snentry: libc::c_short,
    pub o_sntext: libc::c_short,
    pub o_sndata: libc::c_short,
    pub o_sntoc: libc::c_short,
    pub o_snloader: libc::c_short,
    pub o_snbss: libc::c_short,
    pub o_algntext: libc::c_short,
    pub o_algndata: libc::c_short,
    pub o_modtype: libc::c_short,
    pub o_cputype: libc::c_short,
    pub o_maxstack: bfd_vma,
    pub o_maxdata: bfd_vma,
    pub o_flags: libc::c_char,
    pub o_sntdata: libc::c_short,
    pub o_sntbss: libc::c_short,
    pub o_x64flags: libc::c_short,
    pub bss_start: bfd_vma,
    pub gp_value: bfd_vma,
    pub gprmask: libc::c_ulong,
    pub cprmask: [libc::c_ulong; 4],
    pub fprmask: libc::c_ulong,
    pub o_inlib: libc::c_long,
    pub o_sri: libc::c_long,
    pub vid: [libc::c_long; 2],
    pub pe: internal_extra_pe_aouthdr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct internal_scnhdr {
    pub s_name: [libc::c_char; 8],
    pub s_paddr: bfd_vma,
    pub s_vaddr: bfd_vma,
    pub s_size: bfd_vma,
    pub s_scnptr: bfd_vma,
    pub s_relptr: bfd_vma,
    pub s_lnnoptr: bfd_vma,
    pub s_nreloc: libc::c_ulong,
    pub s_nlnno: libc::c_ulong,
    pub s_flags: libc::c_ulong,
    pub s_page: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct internal_lineno {
    pub l_addr: C2RustUnnamed_34,
    pub l_lnno: libc::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_34 {
    pub l_symndx: bfd_signed_vma,
    pub l_paddr: bfd_signed_vma,
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
pub type wint_t = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct external_filehdr {
    pub f_magic: [libc::c_char; 2],
    pub f_nscns: [libc::c_char; 2],
    pub f_timdat: [libc::c_char; 4],
    pub f_symptr: [libc::c_char; 4],
    pub f_nsyms: [libc::c_char; 4],
    pub f_opthdr: [libc::c_char; 2],
    pub f_flags: [libc::c_char; 2],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct external_aouthdr {
    pub magic: [libc::c_char; 2],
    pub vstamp: [libc::c_char; 2],
    pub tsize: [libc::c_char; 4],
    pub dsize: [libc::c_char; 4],
    pub bsize: [libc::c_char; 4],
    pub entry: [libc::c_char; 4],
    pub text_start: [libc::c_char; 4],
    pub data_start: [libc::c_char; 4],
}
pub type AOUTHDR = external_aouthdr;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct external_aouthdr64 {
    pub magic: [libc::c_char; 2],
    pub vstamp: [libc::c_char; 2],
    pub tsize: [libc::c_char; 4],
    pub dsize: [libc::c_char; 4],
    pub bsize: [libc::c_char; 4],
    pub entry: [libc::c_char; 4],
    pub text_start: [libc::c_char; 4],
}
pub type AOUTHDR64 = external_aouthdr64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct external_scnhdr {
    pub s_name: [libc::c_char; 8],
    pub s_paddr: [libc::c_char; 4],
    pub s_vaddr: [libc::c_char; 4],
    pub s_size: [libc::c_char; 4],
    pub s_scnptr: [libc::c_char; 4],
    pub s_relptr: [libc::c_char; 4],
    pub s_lnnoptr: [libc::c_char; 4],
    pub s_nreloc: [libc::c_char; 2],
    pub s_nlnno: [libc::c_char; 2],
    pub s_flags: [libc::c_char; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct external_lineno {
    pub l_addr: C2RustUnnamed_35,
    pub l_lnno: [libc::c_char; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_35 {
    pub l_symndx: [libc::c_char; 4],
    pub l_paddr: [libc::c_char; 4],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct external_syment {
    pub e: C2RustUnnamed_36,
    pub e_value: [libc::c_char; 4],
    pub e_scnum: [libc::c_char; 2],
    pub e_type: [libc::c_char; 2],
    pub e_sclass: [libc::c_char; 1],
    pub e_numaux: [libc::c_char; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_36 {
    pub e_name: [libc::c_char; 8],
    pub e: C2RustUnnamed_37,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_37 {
    pub e_zeroes: [libc::c_char; 4],
    pub e_offset: [libc::c_char; 4],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub union external_auxent {
    pub x_sym: C2RustUnnamed_42,
    pub x_file: C2RustUnnamed_40,
    pub x_scn: C2RustUnnamed_39,
    pub x_tv: C2RustUnnamed_38,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_38 {
    pub x_tvfill: [libc::c_char; 4],
    pub x_tvlen: [libc::c_char; 2],
    pub x_tvran: [[libc::c_char; 2]; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_39 {
    pub x_scnlen: [libc::c_char; 4],
    pub x_nreloc: [libc::c_char; 2],
    pub x_nlinno: [libc::c_char; 2],
    pub x_checksum: [libc::c_char; 4],
    pub x_associated: [libc::c_char; 2],
    pub x_comdat: [libc::c_char; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_40 {
    pub x_fname: [libc::c_char; 14],
    pub x_n: C2RustUnnamed_41,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_41 {
    pub x_zeroes: [libc::c_char; 4],
    pub x_offset: [libc::c_char; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_42 {
    pub x_tagndx: [libc::c_char; 4],
    pub x_misc: C2RustUnnamed_46,
    pub x_fcnary: C2RustUnnamed_43,
    pub x_tvndx: [libc::c_char; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_43 {
    pub x_fcn: C2RustUnnamed_45,
    pub x_ary: C2RustUnnamed_44,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_44 {
    pub x_dimen: [[libc::c_char; 2]; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_45 {
    pub x_lnnoptr: [libc::c_char; 4],
    pub x_endndx: [libc::c_char; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_46 {
    pub x_lnsz: C2RustUnnamed_47,
    pub x_fsize: [libc::c_char; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_47 {
    pub x_lnno: [libc::c_char; 2],
    pub x_size: [libc::c_char; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct external_PEI_filehdr {
    pub e_magic: [libc::c_char; 2],
    pub e_cblp: [libc::c_char; 2],
    pub e_cp: [libc::c_char; 2],
    pub e_crlc: [libc::c_char; 2],
    pub e_cparhdr: [libc::c_char; 2],
    pub e_minalloc: [libc::c_char; 2],
    pub e_maxalloc: [libc::c_char; 2],
    pub e_ss: [libc::c_char; 2],
    pub e_sp: [libc::c_char; 2],
    pub e_csum: [libc::c_char; 2],
    pub e_ip: [libc::c_char; 2],
    pub e_cs: [libc::c_char; 2],
    pub e_lfarlc: [libc::c_char; 2],
    pub e_ovno: [libc::c_char; 2],
    pub e_res: [[libc::c_char; 2]; 4],
    pub e_oemid: [libc::c_char; 2],
    pub e_oeminfo: [libc::c_char; 2],
    pub e_res2: [[libc::c_char; 2]; 10],
    pub e_lfanew: [libc::c_char; 4],
    pub dos_message: [[libc::c_char; 4]; 16],
    pub nt_signature: [libc::c_char; 4],
    pub f_magic: [libc::c_char; 2],
    pub f_nscns: [libc::c_char; 2],
    pub f_timdat: [libc::c_char; 4],
    pub f_symptr: [libc::c_char; 4],
    pub f_nsyms: [libc::c_char; 4],
    pub f_opthdr: [libc::c_char; 2],
    pub f_flags: [libc::c_char; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PEPAOUTHDR {
    pub standard: AOUTHDR64,
    pub ImageBase: [libc::c_char; 8],
    pub SectionAlignment: [libc::c_char; 4],
    pub FileAlignment: [libc::c_char; 4],
    pub MajorOperatingSystemVersion: [libc::c_char; 2],
    pub MinorOperatingSystemVersion: [libc::c_char; 2],
    pub MajorImageVersion: [libc::c_char; 2],
    pub MinorImageVersion: [libc::c_char; 2],
    pub MajorSubsystemVersion: [libc::c_char; 2],
    pub MinorSubsystemVersion: [libc::c_char; 2],
    pub Reserved1: [libc::c_char; 4],
    pub SizeOfImage: [libc::c_char; 4],
    pub SizeOfHeaders: [libc::c_char; 4],
    pub CheckSum: [libc::c_char; 4],
    pub Subsystem: [libc::c_char; 2],
    pub DllCharacteristics: [libc::c_char; 2],
    pub SizeOfStackReserve: [libc::c_char; 8],
    pub SizeOfStackCommit: [libc::c_char; 8],
    pub SizeOfHeapReserve: [libc::c_char; 8],
    pub SizeOfHeapCommit: [libc::c_char; 8],
    pub LoaderFlags: [libc::c_char; 4],
    pub NumberOfRvaAndSizes: [libc::c_char; 4],
    pub DataDirectory: [[[libc::c_char; 4]; 2]; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct external_IMAGE_DEBUG_DIRECTORY {
    pub Characteristics: [libc::c_char; 4],
    pub TimeDateStamp: [libc::c_char; 4],
    pub MajorVersion: [libc::c_char; 2],
    pub MinorVersion: [libc::c_char; 2],
    pub Type: [libc::c_char; 4],
    pub SizeOfData: [libc::c_char; 4],
    pub AddressOfRawData: [libc::c_char; 4],
    pub PointerToRawData: [libc::c_char; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _CV_INFO_PDB70 {
    pub CvSignature: [libc::c_char; 4],
    pub Signature: [libc::c_char; 16],
    pub Age: [libc::c_char; 4],
    pub PdbFileName: [libc::c_char; 0],
}
pub type CV_INFO_PDB70 = _CV_INFO_PDB70;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _CV_INFO_PDB20 {
    pub CvHeader: [libc::c_char; 4],
    pub Offset: [libc::c_char; 4],
    pub Signature: [libc::c_char; 4],
    pub Age: [libc::c_char; 4],
    pub PdbFileName: [libc::c_char; 0],
}
pub type CV_INFO_PDB20 = _CV_INFO_PDB20;
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
pub struct pei_section_tdata {
    pub virt_size: bfd_size_type,
    pub pe_flags: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct coff_link_hash_table {
    pub root: bfd_link_hash_table,
    pub stab_info: stab_info,
}
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
pub struct pe_required_section_flags {
    pub section_name: [libc::c_char; 8],
    pub must_have: libc::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rsrc_regions {
    pub section_start: *mut bfd_byte,
    pub section_end: *mut bfd_byte,
    pub strings_start: *mut bfd_byte,
    pub resource_start: *mut bfd_byte,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EDT_type {
    pub export_flags: libc::c_long,
    pub time_stamp: libc::c_long,
    pub major_ver: libc::c_short,
    pub minor_ver: libc::c_short,
    pub name: bfd_vma,
    pub base: libc::c_long,
    pub num_functions: libc::c_ulong,
    pub num_names: libc::c_ulong,
    pub eat_addr: bfd_vma,
    pub npt_addr: bfd_vma,
    pub ot_addr: bfd_vma,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rsrc_directory {
    pub characteristics: libc::c_uint,
    pub time: libc::c_uint,
    pub major: libc::c_uint,
    pub minor: libc::c_uint,
    pub names: rsrc_dir_chain,
    pub ids: rsrc_dir_chain,
    pub entry: *mut rsrc_entry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rsrc_entry {
    pub is_name: bool,
    pub name_id: C2RustUnnamed_49,
    pub is_dir: bool,
    pub value: C2RustUnnamed_48,
    pub next_entry: *mut rsrc_entry,
    pub parent: *mut rsrc_directory,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_48 {
    pub directory: *mut rsrc_directory,
    pub leaf: *mut rsrc_leaf,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rsrc_leaf {
    pub size: libc::c_uint,
    pub codepage: libc::c_uint,
    pub data: *mut bfd_byte,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_49 {
    pub id: libc::c_uint,
    pub name: rsrc_string,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rsrc_string {
    pub len: libc::c_uint,
    pub string: *mut bfd_byte,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rsrc_dir_chain {
    pub num_entries: libc::c_uint,
    pub first_entry: *mut rsrc_entry,
    pub last_entry: *mut rsrc_entry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rsrc_write_data {
    pub abfd: *mut bfd,
    pub datastart: *mut bfd_byte,
    pub next_table: *mut bfd_byte,
    pub next_leaf: *mut bfd_byte,
    pub next_string: *mut bfd_byte,
    pub next_data: *mut bfd_byte,
    pub rva_bias: bfd_vma,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sym_cache {
    pub symcount: libc::c_int,
    pub syms: *mut *mut asymbol,
}
#[inline]
unsafe extern "C" fn startswith(
    mut str: *const libc::c_char,
    mut prefix: *const libc::c_char,
) -> bool {
    return strncmp(str, prefix, strlen(prefix)) == 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn bfd_is_abs_section(mut sec: *const asection) -> bool {
    return sec
        == &mut *_bfd_std_section.as_mut_ptr().offset(2 as libc::c_int as isize)
            as *mut asection as *const asection;
}
#[inline]
unsafe extern "C" fn discarded_section(mut sec: *const asection) -> bool {
    return !bfd_is_abs_section(sec)
        && bfd_is_abs_section((*sec).output_section) as libc::c_int != 0
        && (*sec).sec_info_type() as libc::c_int != 2 as libc::c_int
        && (*sec).sec_info_type() as libc::c_int != 4 as libc::c_int;
}
#[inline]
unsafe extern "C" fn bfd_get_file_flags(mut abfd: *const bfd) -> flagword {
    return (*abfd).flags;
}
#[inline]
unsafe extern "C" fn bfd_get_flavour(mut abfd: *const bfd) -> bfd_flavour {
    return (*(*abfd).xvec).flavour;
}
#[inline]
unsafe extern "C" fn bfd_get_symbol_leading_char(mut abfd: *const bfd) -> libc::c_char {
    return (*(*abfd).xvec).symbol_leading_char;
}
#[no_mangle]
pub unsafe extern "C" fn _bfd_pex64i_swap_sym_in(
    mut abfd: *mut bfd,
    mut ext1: *mut libc::c_void,
    mut in1: *mut libc::c_void,
) {
    let mut ext: *mut external_syment = ext1 as *mut external_syment;
    let mut in_0: *mut internal_syment = in1 as *mut internal_syment;
    if (*ext).e.e_name[0 as libc::c_int as usize] as libc::c_int == 0 as libc::c_int {
        (*in_0)._n._n_n._n_zeroes = 0 as libc::c_int as bfd_hostptr_t;
        (*in_0)
            ._n
            ._n_n
            ._n_offset = (Some(
            ((*(*abfd).xvec).bfd_h_getx32).expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(((*ext).e.e.e_offset).as_mut_ptr() as *const libc::c_void);
    } else {
        memcpy(
            ((*in_0)._n._n_name).as_mut_ptr() as *mut libc::c_void,
            ((*ext).e.e_name).as_mut_ptr() as *const libc::c_void,
            8 as libc::c_int as libc::c_ulong,
        );
    }
    (*in_0)
        .n_value = (Some(
        ((*(*abfd).xvec).bfd_h_getx32).expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )(((*ext).e_value).as_mut_ptr() as *const libc::c_void);
    (*in_0)
        .n_scnum = (Some(
        ((*(*abfd).xvec).bfd_h_getx16).expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )(((*ext).e_scnum).as_mut_ptr() as *const libc::c_void) as libc::c_short
        as libc::c_int;
    if ::core::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong
        == 2 as libc::c_int as libc::c_ulong
    {
        (*in_0)
            .n_type = (Some(
            ((*(*abfd).xvec).bfd_h_getx16).expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(((*ext).e_type).as_mut_ptr() as *const libc::c_void) as libc::c_ushort;
    } else {
        (*in_0)
            .n_type = (Some(
            ((*(*abfd).xvec).bfd_h_getx32).expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(((*ext).e_type).as_mut_ptr() as *const libc::c_void) as libc::c_ushort;
    }
    (*in_0)
        .n_sclass = (*(((*ext).e_sclass).as_mut_ptr() as *const libc::c_uchar) as bfd_vma
        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
    (*in_0)
        .n_numaux = (*(((*ext).e_numaux).as_mut_ptr() as *const libc::c_uchar) as bfd_vma
        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
    if (*in_0).n_sclass as libc::c_int == 104 as libc::c_int {
        let mut namebuf: [libc::c_char; 9] = [0; 9];
        let mut name: *const libc::c_char = 0 as *const libc::c_char;
        (*in_0).n_value = 0 as libc::c_int as bfd_vma;
        if (*in_0).n_scnum == 0 as libc::c_int {
            let mut sec: *mut asection = 0 as *mut asection;
            name = _bfd_coff_internal_syment_name(abfd, in_0, namebuf.as_mut_ptr());
            if name.is_null() {
                _bfd_error_handler(
                    dcgettext(
                        b"bfd\0" as *const u8 as *const libc::c_char,
                        b"%pB: unable to find name for empty section\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    abfd,
                );
                bfd_set_error(bfd_error_invalid_target);
                return;
            }
            sec = bfd_get_section_by_name(abfd, name);
            if !sec.is_null() {
                (*in_0).n_scnum = (*sec).target_index;
            }
        }
        if (*in_0).n_scnum == 0 as libc::c_int {
            let mut unused_section_number: libc::c_int = 0 as libc::c_int;
            let mut sec_0: *mut asection = 0 as *mut asection;
            let mut flags: flagword = 0;
            let mut name_len: size_t = 0;
            let mut sec_name: *mut libc::c_char = 0 as *mut libc::c_char;
            sec_0 = (*abfd).sections;
            while !sec_0.is_null() {
                if unused_section_number <= (*sec_0).target_index {
                    unused_section_number = (*sec_0).target_index + 1 as libc::c_int;
                }
                sec_0 = (*sec_0).next;
            }
            name_len = (strlen(name)).wrapping_add(1 as libc::c_int as libc::c_ulong);
            sec_name = bfd_alloc(abfd, name_len) as *mut libc::c_char;
            if sec_name.is_null() {
                _bfd_error_handler(
                    dcgettext(
                        b"bfd\0" as *const u8 as *const libc::c_char,
                        b"%pB: out of memory creating name for empty section\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    abfd,
                );
                return;
            }
            memcpy(sec_name as *mut libc::c_void, name as *const libc::c_void, name_len);
            flags = (0x100 as libc::c_int | 0x1 as libc::c_int | 0x20 as libc::c_int
                | 0x2 as libc::c_int) as flagword;
            sec_0 = bfd_make_section_anyway_with_flags(abfd, sec_name, flags);
            if sec_0.is_null() {
                _bfd_error_handler(
                    dcgettext(
                        b"bfd\0" as *const u8 as *const libc::c_char,
                        b"%pB: unable to create fake empty section\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    abfd,
                );
                return;
            }
            (*sec_0).vma = 0 as libc::c_int as bfd_vma;
            (*sec_0).lma = 0 as libc::c_int as bfd_vma;
            (*sec_0).size = 0 as libc::c_int as bfd_size_type;
            (*sec_0).filepos = 0 as libc::c_int as file_ptr;
            (*sec_0).rel_filepos = 0 as libc::c_int as file_ptr;
            (*sec_0).reloc_count = 0 as libc::c_int as libc::c_uint;
            (*sec_0).line_filepos = 0 as libc::c_int as file_ptr;
            (*sec_0).lineno_count = 0 as libc::c_int as libc::c_uint;
            (*sec_0).userdata = 0 as *mut libc::c_void;
            (*sec_0).next = 0 as *mut bfd_section;
            (*sec_0).alignment_power = 2 as libc::c_int as libc::c_uint;
            (*sec_0).target_index = unused_section_number;
            (*in_0).n_scnum = unused_section_number;
        }
        (*in_0).n_sclass = 3 as libc::c_int as libc::c_uchar;
    }
}
unsafe extern "C" fn abs_finder(
    mut _abfd: *mut bfd,
    mut sec: *mut asection,
    mut data: *mut libc::c_void,
) -> bool {
    let mut abs_val: bfd_vma = *(data as *mut bfd_vma);
    return (*sec).vma <= abs_val
        && ((*sec).vma as libc::c_ulonglong)
            .wrapping_add((1 as libc::c_ulonglong) << 32 as libc::c_int)
            > abs_val as libc::c_ulonglong;
}
#[no_mangle]
pub unsafe extern "C" fn _bfd_pex64i_swap_sym_out(
    mut abfd: *mut bfd,
    mut inp: *mut libc::c_void,
    mut extp: *mut libc::c_void,
) -> libc::c_uint {
    let mut in_0: *mut internal_syment = inp as *mut internal_syment;
    let mut ext: *mut external_syment = extp as *mut external_syment;
    if (*in_0)._n._n_name[0 as libc::c_int as usize] as libc::c_int == 0 as libc::c_int {
        (Some(((*(*abfd).xvec).bfd_h_putx32).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            0 as libc::c_int as bfd_vma,
            ((*ext).e.e.e_zeroes).as_mut_ptr() as *mut libc::c_void,
        );
        (Some(((*(*abfd).xvec).bfd_h_putx32).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            (*in_0)._n._n_n._n_offset,
            ((*ext).e.e.e_offset).as_mut_ptr() as *mut libc::c_void,
        );
    } else {
        memcpy(
            ((*ext).e.e_name).as_mut_ptr() as *mut libc::c_void,
            ((*in_0)._n._n_name).as_mut_ptr() as *const libc::c_void,
            8 as libc::c_int as libc::c_ulong,
        );
    }
    if ::core::mem::size_of::<bfd_vma>() as libc::c_ulong
        > 4 as libc::c_int as libc::c_ulong
        && (*in_0).n_value as libc::c_ulonglong
            > ((1 as libc::c_ulonglong)
                << (if ::core::mem::size_of::<bfd_vma>() as libc::c_ulong
                    > 4 as libc::c_int as libc::c_ulong
                {
                    32 as libc::c_int
                } else {
                    31 as libc::c_int
                }))
                .wrapping_sub(1 as libc::c_int as libc::c_ulonglong)
        && (*in_0).n_scnum == -(1 as libc::c_int)
    {
        let mut sec: *mut asection = 0 as *mut asection;
        sec = bfd_sections_find_if(
            abfd,
            Some(
                abs_finder
                    as unsafe extern "C" fn(
                        *mut bfd,
                        *mut asection,
                        *mut libc::c_void,
                    ) -> bool,
            ),
            &mut (*in_0).n_value as *mut bfd_vma as *mut libc::c_void,
        );
        if !sec.is_null() {
            (*in_0)
                .n_value = ((*in_0).n_value as libc::c_ulong).wrapping_sub((*sec).vma)
                as bfd_vma as bfd_vma;
            (*in_0).n_scnum = (*sec).target_index;
        }
    }
    (Some(((*(*abfd).xvec).bfd_h_putx32).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )((*in_0).n_value, ((*ext).e_value).as_mut_ptr() as *mut libc::c_void);
    (Some(((*(*abfd).xvec).bfd_h_putx16).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (*in_0).n_scnum as bfd_vma,
        ((*ext).e_scnum).as_mut_ptr() as *mut libc::c_void,
    );
    if ::core::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong
        == 2 as libc::c_int as libc::c_ulong
    {
        (Some(((*(*abfd).xvec).bfd_h_putx16).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            (*in_0).n_type as bfd_vma,
            ((*ext).e_type).as_mut_ptr() as *mut libc::c_void,
        );
    } else {
        (Some(((*(*abfd).xvec).bfd_h_putx32).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            (*in_0).n_type as bfd_vma,
            ((*ext).e_type).as_mut_ptr() as *mut libc::c_void,
        );
    }
    *(((*ext).e_sclass).as_mut_ptr()
        as *mut libc::c_uchar) = ((*in_0).n_sclass as libc::c_int & 0xff as libc::c_int)
        as libc::c_uchar;
    *(((*ext).e_numaux).as_mut_ptr()
        as *mut libc::c_uchar) = ((*in_0).n_numaux as libc::c_int & 0xff as libc::c_int)
        as libc::c_uchar;
    return 18 as libc::c_int as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn _bfd_pex64i_swap_aux_in(
    mut abfd: *mut bfd,
    mut ext1: *mut libc::c_void,
    mut type_0: libc::c_int,
    mut in_class: libc::c_int,
    mut _indx: libc::c_int,
    mut _numaux: libc::c_int,
    mut in1: *mut libc::c_void,
) {
    let mut ext: *mut external_auxent = ext1 as *mut external_auxent;
    let mut in_0: *mut internal_auxent = in1 as *mut internal_auxent;
    memset(
        in_0 as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<internal_auxent>() as libc::c_ulong,
    );
    match in_class {
        103 => {
            if (*ext).x_file.x_fname[0 as libc::c_int as usize] as libc::c_int
                == 0 as libc::c_int
            {
                (*in_0).x_file.x_n.x_zeroes = 0 as libc::c_int as libc::c_long;
                (*in_0)
                    .x_file
                    .x_n
                    .x_offset = (Some(
                    ((*(*abfd).xvec).bfd_h_getx32).expect("non-null function pointer"),
                ))
                    .expect(
                        "non-null function pointer",
                    )(((*ext).x_file.x_n.x_offset).as_mut_ptr() as *const libc::c_void)
                    as libc::c_long;
            } else {
                memcpy(
                    ((*in_0).x_file.x_fname).as_mut_ptr() as *mut libc::c_void,
                    ((*ext).x_file.x_fname).as_mut_ptr() as *const libc::c_void,
                    18 as libc::c_int as libc::c_ulong,
                );
            }
            return;
        }
        3 | 113 | 106 => {
            if type_0 == 0 as libc::c_int {
                (*in_0)
                    .x_scn
                    .x_scnlen = (Some(
                    ((*(*abfd).xvec).bfd_h_getx32).expect("non-null function pointer"),
                ))
                    .expect(
                        "non-null function pointer",
                    )(((*ext).x_scn.x_scnlen).as_mut_ptr() as *const libc::c_void)
                    as libc::c_long;
                (*in_0)
                    .x_scn
                    .x_nreloc = (Some(
                    ((*(*abfd).xvec).bfd_h_getx16).expect("non-null function pointer"),
                ))
                    .expect(
                        "non-null function pointer",
                    )(((*ext).x_scn.x_nreloc).as_mut_ptr() as *const libc::c_void)
                    as libc::c_ushort;
                (*in_0)
                    .x_scn
                    .x_nlinno = (Some(
                    ((*(*abfd).xvec).bfd_h_getx16).expect("non-null function pointer"),
                ))
                    .expect(
                        "non-null function pointer",
                    )(((*ext).x_scn.x_nlinno).as_mut_ptr() as *const libc::c_void)
                    as libc::c_ushort;
                (*in_0)
                    .x_scn
                    .x_checksum = (Some(
                    ((*(*abfd).xvec).bfd_h_getx32).expect("non-null function pointer"),
                ))
                    .expect(
                        "non-null function pointer",
                    )(((*ext).x_scn.x_checksum).as_mut_ptr() as *const libc::c_void);
                (*in_0)
                    .x_scn
                    .x_associated = (Some(
                    ((*(*abfd).xvec).bfd_h_getx16).expect("non-null function pointer"),
                ))
                    .expect(
                        "non-null function pointer",
                    )(((*ext).x_scn.x_associated).as_mut_ptr() as *const libc::c_void)
                    as libc::c_ushort;
                (*in_0)
                    .x_scn
                    .x_comdat = (*(((*ext).x_scn.x_comdat).as_mut_ptr()
                    as *const libc::c_uchar) as bfd_vma
                    & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                return;
            }
        }
        _ => {}
    }
    (*in_0)
        .x_sym
        .x_tagndx
        .l = (Some(((*(*abfd).xvec).bfd_h_getx32).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(((*ext).x_sym.x_tagndx).as_mut_ptr() as *const libc::c_void) as libc::c_long;
    (*in_0)
        .x_sym
        .x_tvndx = (Some(
        ((*(*abfd).xvec).bfd_h_getx16).expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )(((*ext).x_sym.x_tvndx).as_mut_ptr() as *const libc::c_void) as libc::c_ushort;
    if in_class == 100 as libc::c_int || in_class == 101 as libc::c_int
        || type_0 as libc::c_ulong & 0x30 as libc::c_int as libc::c_ulong
            == (2 as libc::c_int as libc::c_ulong) << 4 as libc::c_int
        || (in_class == 10 as libc::c_int || in_class == 12 as libc::c_int
            || in_class == 15 as libc::c_int)
    {
        (*in_0)
            .x_sym
            .x_fcnary
            .x_fcn
            .x_lnnoptr = (Some(
            ((*(*abfd).xvec).bfd_h_getx32).expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            ((*ext).x_sym.x_fcnary.x_fcn.x_lnnoptr).as_mut_ptr() as *const libc::c_void,
        ) as bfd_signed_vma;
        (*in_0)
            .x_sym
            .x_fcnary
            .x_fcn
            .x_endndx
            .l = (Some(
            ((*(*abfd).xvec).bfd_h_getx32).expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(((*ext).x_sym.x_fcnary.x_fcn.x_endndx).as_mut_ptr() as *const libc::c_void)
            as libc::c_long;
    } else {
        (*in_0)
            .x_sym
            .x_fcnary
            .x_ary
            .x_dimen[0 as libc::c_int
            as usize] = (Some(
            ((*(*abfd).xvec).bfd_h_getx16).expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            ((*ext).x_sym.x_fcnary.x_ary.x_dimen[0 as libc::c_int as usize]).as_mut_ptr()
                as *const libc::c_void,
        ) as libc::c_ushort;
        (*in_0)
            .x_sym
            .x_fcnary
            .x_ary
            .x_dimen[1 as libc::c_int
            as usize] = (Some(
            ((*(*abfd).xvec).bfd_h_getx16).expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            ((*ext).x_sym.x_fcnary.x_ary.x_dimen[1 as libc::c_int as usize]).as_mut_ptr()
                as *const libc::c_void,
        ) as libc::c_ushort;
        (*in_0)
            .x_sym
            .x_fcnary
            .x_ary
            .x_dimen[2 as libc::c_int
            as usize] = (Some(
            ((*(*abfd).xvec).bfd_h_getx16).expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            ((*ext).x_sym.x_fcnary.x_ary.x_dimen[2 as libc::c_int as usize]).as_mut_ptr()
                as *const libc::c_void,
        ) as libc::c_ushort;
        (*in_0)
            .x_sym
            .x_fcnary
            .x_ary
            .x_dimen[3 as libc::c_int
            as usize] = (Some(
            ((*(*abfd).xvec).bfd_h_getx16).expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            ((*ext).x_sym.x_fcnary.x_ary.x_dimen[3 as libc::c_int as usize]).as_mut_ptr()
                as *const libc::c_void,
        ) as libc::c_ushort;
    }
    if type_0 as libc::c_ulong & 0x30 as libc::c_int as libc::c_ulong
        == (2 as libc::c_int as libc::c_ulong) << 4 as libc::c_int
    {
        (*in_0)
            .x_sym
            .x_misc
            .x_fsize = (Some(
            ((*(*abfd).xvec).bfd_h_getx32).expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(((*ext).x_sym.x_misc.x_fsize).as_mut_ptr() as *const libc::c_void)
            as libc::c_long;
    } else {
        (*in_0)
            .x_sym
            .x_misc
            .x_lnsz
            .x_lnno = (Some(
            ((*(*abfd).xvec).bfd_h_getx16).expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(((*ext).x_sym.x_misc.x_lnsz.x_lnno).as_mut_ptr() as *const libc::c_void)
            as libc::c_ushort;
        (*in_0)
            .x_sym
            .x_misc
            .x_lnsz
            .x_size = (Some(
            ((*(*abfd).xvec).bfd_h_getx16).expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(((*ext).x_sym.x_misc.x_lnsz.x_size).as_mut_ptr() as *const libc::c_void)
            as libc::c_ushort;
    };
}
#[no_mangle]
pub unsafe extern "C" fn _bfd_pex64i_swap_aux_out(
    mut abfd: *mut bfd,
    mut inp: *mut libc::c_void,
    mut type_0: libc::c_int,
    mut in_class: libc::c_int,
    mut _indx: libc::c_int,
    mut _numaux: libc::c_int,
    mut extp: *mut libc::c_void,
) -> libc::c_uint {
    let mut in_0: *mut internal_auxent = inp as *mut internal_auxent;
    let mut ext: *mut external_auxent = extp as *mut external_auxent;
    memset(
        ext as *mut libc::c_void,
        0 as libc::c_int,
        18 as libc::c_int as libc::c_ulong,
    );
    match in_class {
        103 => {
            if (*in_0).x_file.x_fname[0 as libc::c_int as usize] as libc::c_int
                == 0 as libc::c_int
            {
                (Some(
                    ((*(*abfd).xvec).bfd_h_putx32).expect("non-null function pointer"),
                ))
                    .expect(
                        "non-null function pointer",
                    )(
                    0 as libc::c_int as bfd_vma,
                    ((*ext).x_file.x_n.x_zeroes).as_mut_ptr() as *mut libc::c_void,
                );
                (Some(
                    ((*(*abfd).xvec).bfd_h_putx32).expect("non-null function pointer"),
                ))
                    .expect(
                        "non-null function pointer",
                    )(
                    (*in_0).x_file.x_n.x_offset as bfd_vma,
                    ((*ext).x_file.x_n.x_offset).as_mut_ptr() as *mut libc::c_void,
                );
            } else {
                memcpy(
                    ((*ext).x_file.x_fname).as_mut_ptr() as *mut libc::c_void,
                    ((*in_0).x_file.x_fname).as_mut_ptr() as *const libc::c_void,
                    ::core::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong,
                );
            }
            return 18 as libc::c_int as libc::c_uint;
        }
        3 | 113 | 106 => {
            if type_0 == 0 as libc::c_int {
                (Some(
                    ((*(*abfd).xvec).bfd_h_putx32).expect("non-null function pointer"),
                ))
                    .expect(
                        "non-null function pointer",
                    )(
                    (*in_0).x_scn.x_scnlen as bfd_vma,
                    ((*ext).x_scn.x_scnlen).as_mut_ptr() as *mut libc::c_void,
                );
                (Some(
                    ((*(*abfd).xvec).bfd_h_putx16).expect("non-null function pointer"),
                ))
                    .expect(
                        "non-null function pointer",
                    )(
                    (*in_0).x_scn.x_nreloc as bfd_vma,
                    ((*ext).x_scn.x_nreloc).as_mut_ptr() as *mut libc::c_void,
                );
                (Some(
                    ((*(*abfd).xvec).bfd_h_putx16).expect("non-null function pointer"),
                ))
                    .expect(
                        "non-null function pointer",
                    )(
                    (*in_0).x_scn.x_nlinno as bfd_vma,
                    ((*ext).x_scn.x_nlinno).as_mut_ptr() as *mut libc::c_void,
                );
                (Some(
                    ((*(*abfd).xvec).bfd_h_putx32).expect("non-null function pointer"),
                ))
                    .expect(
                        "non-null function pointer",
                    )(
                    (*in_0).x_scn.x_checksum,
                    ((*ext).x_scn.x_checksum).as_mut_ptr() as *mut libc::c_void,
                );
                (Some(
                    ((*(*abfd).xvec).bfd_h_putx16).expect("non-null function pointer"),
                ))
                    .expect(
                        "non-null function pointer",
                    )(
                    (*in_0).x_scn.x_associated as bfd_vma,
                    ((*ext).x_scn.x_associated).as_mut_ptr() as *mut libc::c_void,
                );
                *(((*ext).x_scn.x_comdat).as_mut_ptr()
                    as *mut libc::c_uchar) = ((*in_0).x_scn.x_comdat as libc::c_int
                    & 0xff as libc::c_int) as libc::c_uchar;
                return 18 as libc::c_int as libc::c_uint;
            }
        }
        _ => {}
    }
    (Some(((*(*abfd).xvec).bfd_h_putx32).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (*in_0).x_sym.x_tagndx.l as bfd_vma,
        ((*ext).x_sym.x_tagndx).as_mut_ptr() as *mut libc::c_void,
    );
    (Some(((*(*abfd).xvec).bfd_h_putx16).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (*in_0).x_sym.x_tvndx as bfd_vma,
        ((*ext).x_sym.x_tvndx).as_mut_ptr() as *mut libc::c_void,
    );
    if in_class == 100 as libc::c_int || in_class == 101 as libc::c_int
        || type_0 as libc::c_ulong & 0x30 as libc::c_int as libc::c_ulong
            == (2 as libc::c_int as libc::c_ulong) << 4 as libc::c_int
        || (in_class == 10 as libc::c_int || in_class == 12 as libc::c_int
            || in_class == 15 as libc::c_int)
    {
        (Some(((*(*abfd).xvec).bfd_h_putx32).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            (*in_0).x_sym.x_fcnary.x_fcn.x_lnnoptr as bfd_vma,
            ((*ext).x_sym.x_fcnary.x_fcn.x_lnnoptr).as_mut_ptr() as *mut libc::c_void,
        );
        (Some(((*(*abfd).xvec).bfd_h_putx32).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            (*in_0).x_sym.x_fcnary.x_fcn.x_endndx.l as bfd_vma,
            ((*ext).x_sym.x_fcnary.x_fcn.x_endndx).as_mut_ptr() as *mut libc::c_void,
        );
    } else {
        (Some(((*(*abfd).xvec).bfd_h_putx16).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            (*in_0).x_sym.x_fcnary.x_ary.x_dimen[0 as libc::c_int as usize] as bfd_vma,
            ((*ext).x_sym.x_fcnary.x_ary.x_dimen[0 as libc::c_int as usize]).as_mut_ptr()
                as *mut libc::c_void,
        );
        (Some(((*(*abfd).xvec).bfd_h_putx16).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            (*in_0).x_sym.x_fcnary.x_ary.x_dimen[1 as libc::c_int as usize] as bfd_vma,
            ((*ext).x_sym.x_fcnary.x_ary.x_dimen[1 as libc::c_int as usize]).as_mut_ptr()
                as *mut libc::c_void,
        );
        (Some(((*(*abfd).xvec).bfd_h_putx16).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            (*in_0).x_sym.x_fcnary.x_ary.x_dimen[2 as libc::c_int as usize] as bfd_vma,
            ((*ext).x_sym.x_fcnary.x_ary.x_dimen[2 as libc::c_int as usize]).as_mut_ptr()
                as *mut libc::c_void,
        );
        (Some(((*(*abfd).xvec).bfd_h_putx16).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            (*in_0).x_sym.x_fcnary.x_ary.x_dimen[3 as libc::c_int as usize] as bfd_vma,
            ((*ext).x_sym.x_fcnary.x_ary.x_dimen[3 as libc::c_int as usize]).as_mut_ptr()
                as *mut libc::c_void,
        );
    }
    if type_0 as libc::c_ulong & 0x30 as libc::c_int as libc::c_ulong
        == (2 as libc::c_int as libc::c_ulong) << 4 as libc::c_int
    {
        (Some(((*(*abfd).xvec).bfd_h_putx32).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            (*in_0).x_sym.x_misc.x_fsize as bfd_vma,
            ((*ext).x_sym.x_misc.x_fsize).as_mut_ptr() as *mut libc::c_void,
        );
    } else {
        (Some(((*(*abfd).xvec).bfd_h_putx16).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            (*in_0).x_sym.x_misc.x_lnsz.x_lnno as bfd_vma,
            ((*ext).x_sym.x_misc.x_lnsz.x_lnno).as_mut_ptr() as *mut libc::c_void,
        );
        (Some(((*(*abfd).xvec).bfd_h_putx16).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            (*in_0).x_sym.x_misc.x_lnsz.x_size as bfd_vma,
            ((*ext).x_sym.x_misc.x_lnsz.x_size).as_mut_ptr() as *mut libc::c_void,
        );
    }
    return 18 as libc::c_int as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn _bfd_pex64i_swap_lineno_in(
    mut abfd: *mut bfd,
    mut ext1: *mut libc::c_void,
    mut in1: *mut libc::c_void,
) {
    let mut ext: *mut external_lineno = ext1 as *mut external_lineno;
    let mut in_0: *mut internal_lineno = in1 as *mut internal_lineno;
    (*in_0)
        .l_addr
        .l_symndx = (Some(
        ((*(*abfd).xvec).bfd_h_getx32).expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )(((*ext).l_addr.l_symndx).as_mut_ptr() as *const libc::c_void)
        as bfd_signed_vma;
    (*in_0)
        .l_lnno = (Some(
        ((*(*abfd).xvec).bfd_h_getx16).expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )(((*ext).l_lnno).as_mut_ptr() as *const libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn _bfd_pex64i_swap_lineno_out(
    mut abfd: *mut bfd,
    mut inp: *mut libc::c_void,
    mut outp: *mut libc::c_void,
) -> libc::c_uint {
    let mut in_0: *mut internal_lineno = inp as *mut internal_lineno;
    let mut ext: *mut external_lineno = outp as *mut external_lineno;
    (Some(((*(*abfd).xvec).bfd_h_putx32).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (*in_0).l_addr.l_symndx as bfd_vma,
        ((*ext).l_addr.l_symndx).as_mut_ptr() as *mut libc::c_void,
    );
    (Some(((*(*abfd).xvec).bfd_h_putx16).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )((*in_0).l_lnno, ((*ext).l_lnno).as_mut_ptr() as *mut libc::c_void);
    return (4 as libc::c_int + 2 as libc::c_int) as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn _bfd_pex64i_swap_aouthdr_in(
    mut abfd: *mut bfd,
    mut aouthdr_ext1: *mut libc::c_void,
    mut aouthdr_int1: *mut libc::c_void,
) {
    let mut src: *mut PEPAOUTHDR = aouthdr_ext1 as *mut PEPAOUTHDR;
    let mut aouthdr_ext: *mut AOUTHDR = aouthdr_ext1 as *mut AOUTHDR;
    let mut aouthdr_int: *mut internal_aouthdr = aouthdr_int1 as *mut internal_aouthdr;
    let mut a: *mut internal_extra_pe_aouthdr = &mut (*aouthdr_int).pe;
    (*aouthdr_int)
        .magic = (Some(
        ((*(*abfd).xvec).bfd_h_getx16).expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )(((*aouthdr_ext).magic).as_mut_ptr() as *const libc::c_void) as libc::c_short;
    (*aouthdr_int)
        .vstamp = (Some(
        ((*(*abfd).xvec).bfd_h_getx16).expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )(((*aouthdr_ext).vstamp).as_mut_ptr() as *const libc::c_void) as libc::c_short;
    (*aouthdr_int)
        .tsize = (Some(
        ((*(*abfd).xvec).bfd_h_getx32).expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )(((*aouthdr_ext).tsize).as_mut_ptr() as *const libc::c_void);
    (*aouthdr_int)
        .dsize = (Some(
        ((*(*abfd).xvec).bfd_h_getx32).expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )(((*aouthdr_ext).dsize).as_mut_ptr() as *const libc::c_void);
    (*aouthdr_int)
        .bsize = (Some(
        ((*(*abfd).xvec).bfd_h_getx32).expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )(((*aouthdr_ext).bsize).as_mut_ptr() as *const libc::c_void);
    (*aouthdr_int)
        .entry = (Some(
        ((*(*abfd).xvec).bfd_h_getx32).expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )(((*aouthdr_ext).entry).as_mut_ptr() as *const libc::c_void);
    (*aouthdr_int)
        .text_start = (Some(
        ((*(*abfd).xvec).bfd_h_getx32).expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )(((*aouthdr_ext).text_start).as_mut_ptr() as *const libc::c_void);
    (*a).Magic = (*aouthdr_int).magic;
    (*a)
        .MajorLinkerVersion = (*(((*aouthdr_ext).vstamp).as_mut_ptr()
        as *const libc::c_uchar) as bfd_vma & 0xff as libc::c_int as libc::c_ulong)
        as libc::c_char;
    (*a)
        .MinorLinkerVersion = (*(((*aouthdr_ext).vstamp)
        .as_mut_ptr()
        .offset(1 as libc::c_int as isize) as *const libc::c_uchar) as bfd_vma
        & 0xff as libc::c_int as libc::c_ulong) as libc::c_char;
    (*a).SizeOfCode = (*aouthdr_int).tsize;
    (*a).SizeOfInitializedData = (*aouthdr_int).dsize;
    (*a).SizeOfUninitializedData = (*aouthdr_int).bsize;
    (*a).AddressOfEntryPoint = (*aouthdr_int).entry;
    (*a).BaseOfCode = (*aouthdr_int).text_start;
    (*a)
        .ImageBase = (Some(
        ((*(*abfd).xvec).bfd_h_getx64).expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )(((*src).ImageBase).as_mut_ptr() as *const libc::c_void);
    (*a)
        .SectionAlignment = (Some(
        ((*(*abfd).xvec).bfd_h_getx32).expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )(((*src).SectionAlignment).as_mut_ptr() as *const libc::c_void) as uint32_t;
    (*a)
        .FileAlignment = (Some(
        ((*(*abfd).xvec).bfd_h_getx32).expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )(((*src).FileAlignment).as_mut_ptr() as *const libc::c_void) as uint32_t;
    (*a)
        .MajorOperatingSystemVersion = (Some(
        ((*(*abfd).xvec).bfd_h_getx16).expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )(((*src).MajorOperatingSystemVersion).as_mut_ptr() as *const libc::c_void)
        as libc::c_short;
    (*a)
        .MinorOperatingSystemVersion = (Some(
        ((*(*abfd).xvec).bfd_h_getx16).expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )(((*src).MinorOperatingSystemVersion).as_mut_ptr() as *const libc::c_void)
        as libc::c_short;
    (*a)
        .MajorImageVersion = (Some(
        ((*(*abfd).xvec).bfd_h_getx16).expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )(((*src).MajorImageVersion).as_mut_ptr() as *const libc::c_void)
        as libc::c_short;
    (*a)
        .MinorImageVersion = (Some(
        ((*(*abfd).xvec).bfd_h_getx16).expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )(((*src).MinorImageVersion).as_mut_ptr() as *const libc::c_void)
        as libc::c_short;
    (*a)
        .MajorSubsystemVersion = (Some(
        ((*(*abfd).xvec).bfd_h_getx16).expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )(((*src).MajorSubsystemVersion).as_mut_ptr() as *const libc::c_void)
        as libc::c_short;
    (*a)
        .MinorSubsystemVersion = (Some(
        ((*(*abfd).xvec).bfd_h_getx16).expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )(((*src).MinorSubsystemVersion).as_mut_ptr() as *const libc::c_void)
        as libc::c_short;
    (*a)
        .Reserved1 = (Some(
        ((*(*abfd).xvec).bfd_h_getx32).expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )(((*src).Reserved1).as_mut_ptr() as *const libc::c_void) as uint32_t;
    (*a)
        .SizeOfImage = (Some(
        ((*(*abfd).xvec).bfd_h_getx32).expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )(((*src).SizeOfImage).as_mut_ptr() as *const libc::c_void) as uint32_t;
    (*a)
        .SizeOfHeaders = (Some(
        ((*(*abfd).xvec).bfd_h_getx32).expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )(((*src).SizeOfHeaders).as_mut_ptr() as *const libc::c_void) as uint32_t;
    (*a)
        .CheckSum = (Some(
        ((*(*abfd).xvec).bfd_h_getx32).expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )(((*src).CheckSum).as_mut_ptr() as *const libc::c_void) as uint32_t;
    (*a)
        .Subsystem = (Some(
        ((*(*abfd).xvec).bfd_h_getx16).expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )(((*src).Subsystem).as_mut_ptr() as *const libc::c_void) as libc::c_short;
    (*a)
        .DllCharacteristics = (Some(
        ((*(*abfd).xvec).bfd_h_getx16).expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )(((*src).DllCharacteristics).as_mut_ptr() as *const libc::c_void)
        as libc::c_ushort;
    (*a)
        .SizeOfStackReserve = (Some(
        ((*(*abfd).xvec).bfd_h_getx64).expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )(((*src).SizeOfStackReserve).as_mut_ptr() as *const libc::c_void);
    (*a)
        .SizeOfStackCommit = (Some(
        ((*(*abfd).xvec).bfd_h_getx64).expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )(((*src).SizeOfStackCommit).as_mut_ptr() as *const libc::c_void);
    (*a)
        .SizeOfHeapReserve = (Some(
        ((*(*abfd).xvec).bfd_h_getx64).expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )(((*src).SizeOfHeapReserve).as_mut_ptr() as *const libc::c_void);
    (*a)
        .SizeOfHeapCommit = (Some(
        ((*(*abfd).xvec).bfd_h_getx64).expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )(((*src).SizeOfHeapCommit).as_mut_ptr() as *const libc::c_void);
    (*a)
        .LoaderFlags = (Some(
        ((*(*abfd).xvec).bfd_h_getx32).expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )(((*src).LoaderFlags).as_mut_ptr() as *const libc::c_void) as uint32_t;
    (*a)
        .NumberOfRvaAndSizes = (Some(
        ((*(*abfd).xvec).bfd_h_getx32).expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )(((*src).NumberOfRvaAndSizes).as_mut_ptr() as *const libc::c_void) as uint32_t;
    let mut idx: libc::c_uint = 0;
    if (*a).NumberOfRvaAndSizes > 16 as libc::c_int as libc::c_uint {
        _bfd_error_handler(
            dcgettext(
                b"bfd\0" as *const u8 as *const libc::c_char,
                b"%pB: aout header specifies an invalid number of data-directory entries: %u\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            abfd,
            (*a).NumberOfRvaAndSizes,
        );
        bfd_set_error(bfd_error_bad_value);
        (*a).NumberOfRvaAndSizes = 0 as libc::c_int as uint32_t;
    }
    idx = 0 as libc::c_int as libc::c_uint;
    while idx < (*a).NumberOfRvaAndSizes {
        let mut size: libc::c_int = (Some(
            ((*(*abfd).xvec).bfd_h_getx32).expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            ((*src).DataDirectory[idx as usize][1 as libc::c_int as usize]).as_mut_ptr()
                as *const libc::c_void,
        ) as libc::c_int;
        (*a).DataDirectory[idx as usize].Size = size as libc::c_long;
        if size != 0 {
            (*a)
                .DataDirectory[idx as usize]
                .VirtualAddress = (Some(
                ((*(*abfd).xvec).bfd_h_getx32).expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(
                ((*src).DataDirectory[idx as usize][0 as libc::c_int as usize])
                    .as_mut_ptr() as *const libc::c_void,
            );
        } else {
            (*a)
                .DataDirectory[idx as usize]
                .VirtualAddress = 0 as libc::c_int as bfd_vma;
        }
        idx = idx.wrapping_add(1);
        idx;
    }
    while idx < 16 as libc::c_int as libc::c_uint {
        (*a).DataDirectory[idx as usize].Size = 0 as libc::c_int as libc::c_long;
        (*a).DataDirectory[idx as usize].VirtualAddress = 0 as libc::c_int as bfd_vma;
        idx = idx.wrapping_add(1);
        idx;
    }
    if (*aouthdr_int).entry != 0 {
        (*aouthdr_int)
            .entry = ((*aouthdr_int).entry as libc::c_ulong).wrapping_add((*a).ImageBase)
            as bfd_vma as bfd_vma;
    }
    if (*aouthdr_int).tsize != 0 {
        (*aouthdr_int)
            .text_start = ((*aouthdr_int).text_start as libc::c_ulong)
            .wrapping_add((*a).ImageBase) as bfd_vma as bfd_vma;
    }
}
unsafe extern "C" fn add_data_entry(
    mut abfd: *mut bfd,
    mut aout: *mut internal_extra_pe_aouthdr,
    mut idx: libc::c_int,
    mut name: *mut libc::c_char,
    mut base: bfd_vma,
) {
    let mut sec: *mut asection = bfd_get_section_by_name(abfd, name);
    if !sec.is_null() && !((*sec).used_by_bfd as *mut coff_section_tdata).is_null()
        && !((*((*sec).used_by_bfd as *mut coff_section_tdata)).tdata
            as *mut pei_section_tdata)
            .is_null()
    {
        let mut size: libc::c_int = (*((*((*sec).used_by_bfd as *mut coff_section_tdata))
            .tdata as *mut pei_section_tdata))
            .virt_size as libc::c_int;
        (*aout).DataDirectory[idx as usize].Size = size as libc::c_long;
        if size != 0 {
            (*aout)
                .DataDirectory[idx as usize]
                .VirtualAddress = ((*sec).vma).wrapping_sub(base)
                & 0xffffffff as libc::c_uint as libc::c_ulong;
            (*sec).flags |= 0x20 as libc::c_int as libc::c_uint;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn _bfd_pex64i_swap_aouthdr_out(
    mut abfd: *mut bfd,
    mut in_0: *mut libc::c_void,
    mut out: *mut libc::c_void,
) -> libc::c_uint {
    let mut aouthdr_in: *mut internal_aouthdr = in_0 as *mut internal_aouthdr;
    let mut pe: *mut pe_data_type = (*abfd).tdata.pe_obj_data;
    let mut extra: *mut internal_extra_pe_aouthdr = &mut (*pe).pe_opthdr;
    let mut aouthdr_out: *mut PEPAOUTHDR = out as *mut PEPAOUTHDR;
    let mut sa: bfd_vma = 0;
    let mut fa: bfd_vma = 0;
    let mut ib: bfd_vma = 0;
    let mut idata2: IMAGE_DATA_DIRECTORY = IMAGE_DATA_DIRECTORY {
        VirtualAddress: 0,
        Size: 0,
    };
    let mut idata5: IMAGE_DATA_DIRECTORY = IMAGE_DATA_DIRECTORY {
        VirtualAddress: 0,
        Size: 0,
    };
    let mut tls: IMAGE_DATA_DIRECTORY = IMAGE_DATA_DIRECTORY {
        VirtualAddress: 0,
        Size: 0,
    };
    sa = (*extra).SectionAlignment as bfd_vma;
    fa = (*extra).FileAlignment as bfd_vma;
    ib = (*extra).ImageBase;
    idata2 = (*pe).pe_opthdr.DataDirectory[1 as libc::c_int as usize];
    idata5 = (*pe).pe_opthdr.DataDirectory[12 as libc::c_int as usize];
    tls = (*pe).pe_opthdr.DataDirectory[9 as libc::c_int as usize];
    if (*aouthdr_in).tsize != 0 {
        (*aouthdr_in)
            .text_start = ((*aouthdr_in).text_start as libc::c_ulong).wrapping_sub(ib)
            as bfd_vma as bfd_vma;
    }
    if (*aouthdr_in).dsize != 0 {
        (*aouthdr_in)
            .data_start = ((*aouthdr_in).data_start as libc::c_ulong).wrapping_sub(ib)
            as bfd_vma as bfd_vma;
    }
    if (*aouthdr_in).entry != 0 {
        (*aouthdr_in)
            .entry = ((*aouthdr_in).entry as libc::c_ulong).wrapping_sub(ib) as bfd_vma
            as bfd_vma;
    }
    (*aouthdr_in)
        .bsize = ((*aouthdr_in).bsize)
        .wrapping_add(fa)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong) & fa.wrapping_neg();
    (*extra).NumberOfRvaAndSizes = 16 as libc::c_int as uint32_t;
    add_data_entry(
        abfd,
        extra,
        0 as libc::c_int,
        b".edata\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ib,
    );
    add_data_entry(
        abfd,
        extra,
        2 as libc::c_int,
        b".rsrc\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ib,
    );
    add_data_entry(
        abfd,
        extra,
        3 as libc::c_int,
        b".pdata\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ib,
    );
    (*extra).DataDirectory[1 as libc::c_int as usize] = idata2;
    (*extra).DataDirectory[12 as libc::c_int as usize] = idata5;
    (*extra).DataDirectory[9 as libc::c_int as usize] = tls;
    if (*extra).DataDirectory[1 as libc::c_int as usize].VirtualAddress
        == 0 as libc::c_int as libc::c_ulong
    {
        add_data_entry(
            abfd,
            extra,
            1 as libc::c_int,
            b".idata\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ib,
        );
    }
    if (*pe).has_reloc_section != 0 {
        add_data_entry(
            abfd,
            extra,
            5 as libc::c_int,
            b".reloc\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ib,
        );
    }
    let mut sec: *mut asection = 0 as *mut asection;
    let mut hsize: bfd_vma = 0 as libc::c_int as bfd_vma;
    let mut dsize: bfd_vma = 0 as libc::c_int as bfd_vma;
    let mut isize: bfd_vma = 0 as libc::c_int as bfd_vma;
    let mut tsize: bfd_vma = 0 as libc::c_int as bfd_vma;
    sec = (*abfd).sections;
    while !sec.is_null() {
        let mut rounded: libc::c_int = (((*sec).size)
            .wrapping_add(fa)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong) & fa.wrapping_neg())
            as libc::c_int;
        if !(rounded == 0 as libc::c_int) {
            if hsize == 0 as libc::c_int as libc::c_ulong {
                hsize = (*sec).filepos as bfd_vma;
            }
            if (*sec).flags & 0x20 as libc::c_int as libc::c_uint != 0 {
                dsize = (dsize as libc::c_ulong).wrapping_add(rounded as libc::c_ulong)
                    as bfd_vma as bfd_vma;
            }
            if (*sec).flags & 0x10 as libc::c_int as libc::c_uint != 0 {
                tsize = (tsize as libc::c_ulong).wrapping_add(rounded as libc::c_ulong)
                    as bfd_vma as bfd_vma;
            }
            if !((*sec).used_by_bfd as *mut coff_section_tdata).is_null()
                && !((*((*sec).used_by_bfd as *mut coff_section_tdata)).tdata
                    as *mut pei_section_tdata)
                    .is_null()
            {
                isize = ((*sec).vma)
                    .wrapping_sub((*extra).ImageBase)
                    .wrapping_add(
                        (((*((*((*sec).used_by_bfd as *mut coff_section_tdata)).tdata
                            as *mut pei_section_tdata))
                            .virt_size)
                            .wrapping_add(fa)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            & fa.wrapping_neg())
                            .wrapping_add(sa)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            & sa.wrapping_neg(),
                    );
            }
        }
        sec = (*sec).next;
    }
    (*aouthdr_in).dsize = dsize;
    (*aouthdr_in).tsize = tsize;
    (*extra).SizeOfHeaders = hsize as uint32_t;
    (*extra).SizeOfImage = isize as uint32_t;
    (Some(((*(*abfd).xvec).bfd_h_putx16).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (*aouthdr_in).magic as bfd_vma,
        ((*aouthdr_out).standard.magic).as_mut_ptr() as *mut libc::c_void,
    );
    (Some(((*(*abfd).xvec).bfd_h_putx16).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        ((237000000 as libc::c_int / 1000000 as libc::c_int) as libc::c_short
            as libc::c_int / 100 as libc::c_int
            + (237000000 as libc::c_int / 1000000 as libc::c_int) as libc::c_short
                as libc::c_int % 100 as libc::c_int * 256 as libc::c_int) as bfd_vma,
        ((*aouthdr_out).standard.vstamp).as_mut_ptr() as *mut libc::c_void,
    );
    (Some(((*(*abfd).xvec).bfd_h_putx32).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (*aouthdr_in).tsize,
        ((*aouthdr_out).standard.tsize).as_mut_ptr() as *mut libc::c_void,
    );
    (Some(((*(*abfd).xvec).bfd_h_putx32).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (*aouthdr_in).dsize,
        ((*aouthdr_out).standard.dsize).as_mut_ptr() as *mut libc::c_void,
    );
    (Some(((*(*abfd).xvec).bfd_h_putx32).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (*aouthdr_in).bsize,
        ((*aouthdr_out).standard.bsize).as_mut_ptr() as *mut libc::c_void,
    );
    (Some(((*(*abfd).xvec).bfd_h_putx32).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (*aouthdr_in).entry,
        ((*aouthdr_out).standard.entry).as_mut_ptr() as *mut libc::c_void,
    );
    (Some(((*(*abfd).xvec).bfd_h_putx32).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (*aouthdr_in).text_start,
        ((*aouthdr_out).standard.text_start).as_mut_ptr() as *mut libc::c_void,
    );
    (Some(((*(*abfd).xvec).bfd_h_putx64).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (*extra).ImageBase,
        ((*aouthdr_out).ImageBase).as_mut_ptr() as *mut libc::c_void,
    );
    (Some(((*(*abfd).xvec).bfd_h_putx32).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (*extra).SectionAlignment as bfd_vma,
        ((*aouthdr_out).SectionAlignment).as_mut_ptr() as *mut libc::c_void,
    );
    (Some(((*(*abfd).xvec).bfd_h_putx32).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (*extra).FileAlignment as bfd_vma,
        ((*aouthdr_out).FileAlignment).as_mut_ptr() as *mut libc::c_void,
    );
    (Some(((*(*abfd).xvec).bfd_h_putx16).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (*extra).MajorOperatingSystemVersion as bfd_vma,
        ((*aouthdr_out).MajorOperatingSystemVersion).as_mut_ptr() as *mut libc::c_void,
    );
    (Some(((*(*abfd).xvec).bfd_h_putx16).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (*extra).MinorOperatingSystemVersion as bfd_vma,
        ((*aouthdr_out).MinorOperatingSystemVersion).as_mut_ptr() as *mut libc::c_void,
    );
    (Some(((*(*abfd).xvec).bfd_h_putx16).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (*extra).MajorImageVersion as bfd_vma,
        ((*aouthdr_out).MajorImageVersion).as_mut_ptr() as *mut libc::c_void,
    );
    (Some(((*(*abfd).xvec).bfd_h_putx16).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (*extra).MinorImageVersion as bfd_vma,
        ((*aouthdr_out).MinorImageVersion).as_mut_ptr() as *mut libc::c_void,
    );
    (Some(((*(*abfd).xvec).bfd_h_putx16).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (*extra).MajorSubsystemVersion as bfd_vma,
        ((*aouthdr_out).MajorSubsystemVersion).as_mut_ptr() as *mut libc::c_void,
    );
    (Some(((*(*abfd).xvec).bfd_h_putx16).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (*extra).MinorSubsystemVersion as bfd_vma,
        ((*aouthdr_out).MinorSubsystemVersion).as_mut_ptr() as *mut libc::c_void,
    );
    (Some(((*(*abfd).xvec).bfd_h_putx32).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (*extra).Reserved1 as bfd_vma,
        ((*aouthdr_out).Reserved1).as_mut_ptr() as *mut libc::c_void,
    );
    (Some(((*(*abfd).xvec).bfd_h_putx32).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (*extra).SizeOfImage as bfd_vma,
        ((*aouthdr_out).SizeOfImage).as_mut_ptr() as *mut libc::c_void,
    );
    (Some(((*(*abfd).xvec).bfd_h_putx32).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (*extra).SizeOfHeaders as bfd_vma,
        ((*aouthdr_out).SizeOfHeaders).as_mut_ptr() as *mut libc::c_void,
    );
    (Some(((*(*abfd).xvec).bfd_h_putx32).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (*extra).CheckSum as bfd_vma,
        ((*aouthdr_out).CheckSum).as_mut_ptr() as *mut libc::c_void,
    );
    (Some(((*(*abfd).xvec).bfd_h_putx16).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (*extra).Subsystem as bfd_vma,
        ((*aouthdr_out).Subsystem).as_mut_ptr() as *mut libc::c_void,
    );
    (Some(((*(*abfd).xvec).bfd_h_putx16).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (*extra).DllCharacteristics as bfd_vma,
        ((*aouthdr_out).DllCharacteristics).as_mut_ptr() as *mut libc::c_void,
    );
    (Some(((*(*abfd).xvec).bfd_h_putx64).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (*extra).SizeOfStackReserve,
        ((*aouthdr_out).SizeOfStackReserve).as_mut_ptr() as *mut libc::c_void,
    );
    (Some(((*(*abfd).xvec).bfd_h_putx64).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (*extra).SizeOfStackCommit,
        ((*aouthdr_out).SizeOfStackCommit).as_mut_ptr() as *mut libc::c_void,
    );
    (Some(((*(*abfd).xvec).bfd_h_putx64).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (*extra).SizeOfHeapReserve,
        ((*aouthdr_out).SizeOfHeapReserve).as_mut_ptr() as *mut libc::c_void,
    );
    (Some(((*(*abfd).xvec).bfd_h_putx64).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (*extra).SizeOfHeapCommit,
        ((*aouthdr_out).SizeOfHeapCommit).as_mut_ptr() as *mut libc::c_void,
    );
    (Some(((*(*abfd).xvec).bfd_h_putx32).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (*extra).LoaderFlags as bfd_vma,
        ((*aouthdr_out).LoaderFlags).as_mut_ptr() as *mut libc::c_void,
    );
    (Some(((*(*abfd).xvec).bfd_h_putx32).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (*extra).NumberOfRvaAndSizes as bfd_vma,
        ((*aouthdr_out).NumberOfRvaAndSizes).as_mut_ptr() as *mut libc::c_void,
    );
    let mut idx: libc::c_int = 0;
    idx = 0 as libc::c_int;
    while idx < 16 as libc::c_int {
        (Some(((*(*abfd).xvec).bfd_h_putx32).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            (*extra).DataDirectory[idx as usize].VirtualAddress,
            ((*aouthdr_out).DataDirectory[idx as usize][0 as libc::c_int as usize])
                .as_mut_ptr() as *mut libc::c_void,
        );
        (Some(((*(*abfd).xvec).bfd_h_putx32).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            (*extra).DataDirectory[idx as usize].Size as bfd_vma,
            ((*aouthdr_out).DataDirectory[idx as usize][1 as libc::c_int as usize])
                .as_mut_ptr() as *mut libc::c_void,
        );
        idx += 1;
        idx;
    }
    return (24 as libc::c_int + 196 as libc::c_int + 5 as libc::c_int * 4 as libc::c_int)
        as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn _bfd_pex64i_only_swap_filehdr_out(
    mut abfd: *mut bfd,
    mut in_0: *mut libc::c_void,
    mut out: *mut libc::c_void,
) -> libc::c_uint {
    let mut idx: libc::c_int = 0;
    let mut filehdr_in: *mut internal_filehdr = in_0 as *mut internal_filehdr;
    let mut filehdr_out: *mut external_PEI_filehdr = out as *mut external_PEI_filehdr;
    if (*(*abfd).tdata.pe_obj_data).has_reloc_section != 0
        || (*(*abfd).tdata.pe_obj_data).dont_strip_reloc != 0
    {
        (*filehdr_in)
            .f_flags = ((*filehdr_in).f_flags as libc::c_int & !(0x1 as libc::c_int))
            as libc::c_ushort;
    }
    if (*(*abfd).tdata.pe_obj_data).dll != 0 {
        (*filehdr_in)
            .f_flags = ((*filehdr_in).f_flags as libc::c_int | 0x2000 as libc::c_int)
            as libc::c_ushort;
    }
    (*filehdr_in).pe.e_magic = 0x5a4d as libc::c_int as libc::c_ushort;
    (*filehdr_in).pe.e_cblp = 0x90 as libc::c_int as libc::c_ushort;
    (*filehdr_in).pe.e_cp = 0x3 as libc::c_int as libc::c_ushort;
    (*filehdr_in).pe.e_crlc = 0 as libc::c_int as libc::c_ushort;
    (*filehdr_in).pe.e_cparhdr = 0x4 as libc::c_int as libc::c_ushort;
    (*filehdr_in).pe.e_minalloc = 0 as libc::c_int as libc::c_ushort;
    (*filehdr_in).pe.e_maxalloc = 0xffff as libc::c_int as libc::c_ushort;
    (*filehdr_in).pe.e_ss = 0 as libc::c_int as libc::c_ushort;
    (*filehdr_in).pe.e_sp = 0xb8 as libc::c_int as libc::c_ushort;
    (*filehdr_in).pe.e_csum = 0 as libc::c_int as libc::c_ushort;
    (*filehdr_in).pe.e_ip = 0 as libc::c_int as libc::c_ushort;
    (*filehdr_in).pe.e_cs = 0 as libc::c_int as libc::c_ushort;
    (*filehdr_in).pe.e_lfarlc = 0x40 as libc::c_int as libc::c_ushort;
    (*filehdr_in).pe.e_ovno = 0 as libc::c_int as libc::c_ushort;
    idx = 0 as libc::c_int;
    while idx < 4 as libc::c_int {
        (*filehdr_in).pe.e_res[idx as usize] = 0 as libc::c_int as libc::c_ushort;
        idx += 1;
        idx;
    }
    (*filehdr_in).pe.e_oemid = 0 as libc::c_int as libc::c_ushort;
    (*filehdr_in).pe.e_oeminfo = 0 as libc::c_int as libc::c_ushort;
    idx = 0 as libc::c_int;
    while idx < 10 as libc::c_int {
        (*filehdr_in).pe.e_res2[idx as usize] = 0 as libc::c_int as libc::c_ushort;
        idx += 1;
        idx;
    }
    (*filehdr_in).pe.e_lfanew = 0x80 as libc::c_int as bfd_vma;
    memcpy(
        ((*filehdr_in).pe.dos_message).as_mut_ptr() as *mut libc::c_void,
        ((*(*abfd).tdata.pe_obj_data).dos_message).as_mut_ptr() as *const libc::c_void,
        ::core::mem::size_of::<[libc::c_uint; 16]>() as libc::c_ulong,
    );
    (*filehdr_in).pe.nt_signature = 0x4550 as libc::c_int as bfd_vma;
    (Some(((*(*abfd).xvec).bfd_h_putx16).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (*filehdr_in).f_magic as bfd_vma,
        ((*filehdr_out).f_magic).as_mut_ptr() as *mut libc::c_void,
    );
    (Some(((*(*abfd).xvec).bfd_h_putx16).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (*filehdr_in).f_nscns as bfd_vma,
        ((*filehdr_out).f_nscns).as_mut_ptr() as *mut libc::c_void,
    );
    if (*(*abfd).tdata.pe_obj_data).timestamp == -(1 as libc::c_int) {
        (Some(((*(*abfd).xvec).bfd_h_putx32).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            time(0 as *mut time_t) as bfd_vma,
            ((*filehdr_out).f_timdat).as_mut_ptr() as *mut libc::c_void,
        );
    } else {
        (Some(((*(*abfd).xvec).bfd_h_putx32).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            (*(*abfd).tdata.pe_obj_data).timestamp as bfd_vma,
            ((*filehdr_out).f_timdat).as_mut_ptr() as *mut libc::c_void,
        );
    }
    (Some(((*(*abfd).xvec).bfd_h_putx32).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (*filehdr_in).f_symptr,
        ((*filehdr_out).f_symptr).as_mut_ptr() as *mut libc::c_void,
    );
    (Some(((*(*abfd).xvec).bfd_h_putx32).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (*filehdr_in).f_nsyms as bfd_vma,
        ((*filehdr_out).f_nsyms).as_mut_ptr() as *mut libc::c_void,
    );
    (Some(((*(*abfd).xvec).bfd_h_putx16).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (*filehdr_in).f_opthdr as bfd_vma,
        ((*filehdr_out).f_opthdr).as_mut_ptr() as *mut libc::c_void,
    );
    (Some(((*(*abfd).xvec).bfd_h_putx16).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (*filehdr_in).f_flags as bfd_vma,
        ((*filehdr_out).f_flags).as_mut_ptr() as *mut libc::c_void,
    );
    (Some(((*(*abfd).xvec).bfd_h_putx16).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (*filehdr_in).pe.e_magic as bfd_vma,
        ((*filehdr_out).e_magic).as_mut_ptr() as *mut libc::c_void,
    );
    (Some(((*(*abfd).xvec).bfd_h_putx16).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (*filehdr_in).pe.e_cblp as bfd_vma,
        ((*filehdr_out).e_cblp).as_mut_ptr() as *mut libc::c_void,
    );
    (Some(((*(*abfd).xvec).bfd_h_putx16).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (*filehdr_in).pe.e_cp as bfd_vma,
        ((*filehdr_out).e_cp).as_mut_ptr() as *mut libc::c_void,
    );
    (Some(((*(*abfd).xvec).bfd_h_putx16).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (*filehdr_in).pe.e_crlc as bfd_vma,
        ((*filehdr_out).e_crlc).as_mut_ptr() as *mut libc::c_void,
    );
    (Some(((*(*abfd).xvec).bfd_h_putx16).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (*filehdr_in).pe.e_cparhdr as bfd_vma,
        ((*filehdr_out).e_cparhdr).as_mut_ptr() as *mut libc::c_void,
    );
    (Some(((*(*abfd).xvec).bfd_h_putx16).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (*filehdr_in).pe.e_minalloc as bfd_vma,
        ((*filehdr_out).e_minalloc).as_mut_ptr() as *mut libc::c_void,
    );
    (Some(((*(*abfd).xvec).bfd_h_putx16).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (*filehdr_in).pe.e_maxalloc as bfd_vma,
        ((*filehdr_out).e_maxalloc).as_mut_ptr() as *mut libc::c_void,
    );
    (Some(((*(*abfd).xvec).bfd_h_putx16).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (*filehdr_in).pe.e_ss as bfd_vma,
        ((*filehdr_out).e_ss).as_mut_ptr() as *mut libc::c_void,
    );
    (Some(((*(*abfd).xvec).bfd_h_putx16).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (*filehdr_in).pe.e_sp as bfd_vma,
        ((*filehdr_out).e_sp).as_mut_ptr() as *mut libc::c_void,
    );
    (Some(((*(*abfd).xvec).bfd_h_putx16).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (*filehdr_in).pe.e_csum as bfd_vma,
        ((*filehdr_out).e_csum).as_mut_ptr() as *mut libc::c_void,
    );
    (Some(((*(*abfd).xvec).bfd_h_putx16).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (*filehdr_in).pe.e_ip as bfd_vma,
        ((*filehdr_out).e_ip).as_mut_ptr() as *mut libc::c_void,
    );
    (Some(((*(*abfd).xvec).bfd_h_putx16).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (*filehdr_in).pe.e_cs as bfd_vma,
        ((*filehdr_out).e_cs).as_mut_ptr() as *mut libc::c_void,
    );
    (Some(((*(*abfd).xvec).bfd_h_putx16).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (*filehdr_in).pe.e_lfarlc as bfd_vma,
        ((*filehdr_out).e_lfarlc).as_mut_ptr() as *mut libc::c_void,
    );
    (Some(((*(*abfd).xvec).bfd_h_putx16).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (*filehdr_in).pe.e_ovno as bfd_vma,
        ((*filehdr_out).e_ovno).as_mut_ptr() as *mut libc::c_void,
    );
    idx = 0 as libc::c_int;
    while idx < 4 as libc::c_int {
        (Some(((*(*abfd).xvec).bfd_h_putx16).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            (*filehdr_in).pe.e_res[idx as usize] as bfd_vma,
            ((*filehdr_out).e_res[idx as usize]).as_mut_ptr() as *mut libc::c_void,
        );
        idx += 1;
        idx;
    }
    (Some(((*(*abfd).xvec).bfd_h_putx16).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (*filehdr_in).pe.e_oemid as bfd_vma,
        ((*filehdr_out).e_oemid).as_mut_ptr() as *mut libc::c_void,
    );
    (Some(((*(*abfd).xvec).bfd_h_putx16).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (*filehdr_in).pe.e_oeminfo as bfd_vma,
        ((*filehdr_out).e_oeminfo).as_mut_ptr() as *mut libc::c_void,
    );
    idx = 0 as libc::c_int;
    while idx < 10 as libc::c_int {
        (Some(((*(*abfd).xvec).bfd_h_putx16).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            (*filehdr_in).pe.e_res2[idx as usize] as bfd_vma,
            ((*filehdr_out).e_res2[idx as usize]).as_mut_ptr() as *mut libc::c_void,
        );
        idx += 1;
        idx;
    }
    (Some(((*(*abfd).xvec).bfd_h_putx32).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (*filehdr_in).pe.e_lfanew,
        ((*filehdr_out).e_lfanew).as_mut_ptr() as *mut libc::c_void,
    );
    idx = 0 as libc::c_int;
    while idx < 16 as libc::c_int {
        (Some(((*(*abfd).xvec).bfd_h_putx32).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            (*filehdr_in).pe.dos_message[idx as usize] as bfd_vma,
            ((*filehdr_out).dos_message[idx as usize]).as_mut_ptr() as *mut libc::c_void,
        );
        idx += 1;
        idx;
    }
    (Some(((*(*abfd).xvec).bfd_h_putx32).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (*filehdr_in).pe.nt_signature,
        ((*filehdr_out).nt_signature).as_mut_ptr() as *mut libc::c_void,
    );
    return 20 as libc::c_int as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn _bfd_pex64_only_swap_filehdr_out(
    mut abfd: *mut bfd,
    mut in_0: *mut libc::c_void,
    mut out: *mut libc::c_void,
) -> libc::c_uint {
    let mut filehdr_in: *mut internal_filehdr = in_0 as *mut internal_filehdr;
    let mut filehdr_out: *mut external_filehdr = out as *mut external_filehdr;
    (Some(((*(*abfd).xvec).bfd_h_putx16).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (*filehdr_in).f_magic as bfd_vma,
        ((*filehdr_out).f_magic).as_mut_ptr() as *mut libc::c_void,
    );
    (Some(((*(*abfd).xvec).bfd_h_putx16).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (*filehdr_in).f_nscns as bfd_vma,
        ((*filehdr_out).f_nscns).as_mut_ptr() as *mut libc::c_void,
    );
    (Some(((*(*abfd).xvec).bfd_h_putx32).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (*filehdr_in).f_timdat as bfd_vma,
        ((*filehdr_out).f_timdat).as_mut_ptr() as *mut libc::c_void,
    );
    (Some(((*(*abfd).xvec).bfd_h_putx32).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (*filehdr_in).f_symptr,
        ((*filehdr_out).f_symptr).as_mut_ptr() as *mut libc::c_void,
    );
    (Some(((*(*abfd).xvec).bfd_h_putx32).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (*filehdr_in).f_nsyms as bfd_vma,
        ((*filehdr_out).f_nsyms).as_mut_ptr() as *mut libc::c_void,
    );
    (Some(((*(*abfd).xvec).bfd_h_putx16).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (*filehdr_in).f_opthdr as bfd_vma,
        ((*filehdr_out).f_opthdr).as_mut_ptr() as *mut libc::c_void,
    );
    (Some(((*(*abfd).xvec).bfd_h_putx16).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (*filehdr_in).f_flags as bfd_vma,
        ((*filehdr_out).f_flags).as_mut_ptr() as *mut libc::c_void,
    );
    return 20 as libc::c_int as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn _bfd_pex64i_swap_scnhdr_out(
    mut abfd: *mut bfd,
    mut in_0: *mut libc::c_void,
    mut out: *mut libc::c_void,
) -> libc::c_uint {
    let mut scnhdr_int: *mut internal_scnhdr = in_0 as *mut internal_scnhdr;
    let mut scnhdr_ext: *mut external_scnhdr = out as *mut external_scnhdr;
    let mut ret: libc::c_uint = 40 as libc::c_int as libc::c_uint;
    let mut ps: bfd_vma = 0;
    let mut ss: bfd_vma = 0;
    memcpy(
        ((*scnhdr_ext).s_name).as_mut_ptr() as *mut libc::c_void,
        ((*scnhdr_int).s_name).as_mut_ptr() as *const libc::c_void,
        ::core::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong,
    );
    ss = ((*scnhdr_int).s_vaddr)
        .wrapping_sub((*(*abfd).tdata.pe_obj_data).pe_opthdr.ImageBase);
    if (*scnhdr_int).s_vaddr < (*(*abfd).tdata.pe_obj_data).pe_opthdr.ImageBase {
        _bfd_error_handler(
            dcgettext(
                b"bfd\0" as *const u8 as *const libc::c_char,
                b"%pB:%.8s: section below image base\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            abfd,
            ((*scnhdr_int).s_name).as_mut_ptr(),
        );
    } else if ss != ss & 0xffffffff as libc::c_uint as libc::c_ulong {
        _bfd_error_handler(
            dcgettext(
                b"bfd\0" as *const u8 as *const libc::c_char,
                b"%pB:%.8s: RVA truncated\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            abfd,
            ((*scnhdr_int).s_name).as_mut_ptr(),
        );
    }
    (Some(((*(*abfd).xvec).bfd_h_putx32).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        ss & 0xffffffff as libc::c_uint as libc::c_ulong,
        ((*scnhdr_ext).s_vaddr).as_mut_ptr() as *mut libc::c_void,
    );
    if (*scnhdr_int).s_flags & 0x80 as libc::c_int as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        if startswith(
            (*(*abfd).xvec).name,
            b"pei-\0" as *const u8 as *const libc::c_char,
        ) {
            ps = (*scnhdr_int).s_size;
            ss = 0 as libc::c_int as bfd_vma;
        } else {
            ps = 0 as libc::c_int as bfd_vma;
            ss = (*scnhdr_int).s_size;
        }
    } else {
        if startswith(
            (*(*abfd).xvec).name,
            b"pei-\0" as *const u8 as *const libc::c_char,
        ) {
            ps = (*scnhdr_int).s_paddr;
        } else {
            ps = 0 as libc::c_int as bfd_vma;
        }
        ss = (*scnhdr_int).s_size;
    }
    (Some(((*(*abfd).xvec).bfd_h_putx32).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(ss, ((*scnhdr_ext).s_size).as_mut_ptr() as *mut libc::c_void);
    (Some(((*(*abfd).xvec).bfd_h_putx32).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(ps, ((*scnhdr_ext).s_paddr).as_mut_ptr() as *mut libc::c_void);
    (Some(((*(*abfd).xvec).bfd_h_putx32).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (*scnhdr_int).s_scnptr,
        ((*scnhdr_ext).s_scnptr).as_mut_ptr() as *mut libc::c_void,
    );
    (Some(((*(*abfd).xvec).bfd_h_putx32).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (*scnhdr_int).s_relptr,
        ((*scnhdr_ext).s_relptr).as_mut_ptr() as *mut libc::c_void,
    );
    (Some(((*(*abfd).xvec).bfd_h_putx32).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (*scnhdr_int).s_lnnoptr,
        ((*scnhdr_ext).s_lnnoptr).as_mut_ptr() as *mut libc::c_void,
    );
    let mut known_sections: [pe_required_section_flags; 12] = [
        {
            let mut init = pe_required_section_flags {
                section_name: *::core::mem::transmute::<
                    &[u8; 8],
                    &mut [libc::c_char; 8],
                >(b".arch\0\0\0"),
                must_have: (0x40000000 as libc::c_int | 0x40 as libc::c_int
                    | 0x2000000 as libc::c_int
                    | (3 as libc::c_int + 1 as libc::c_int) << 20 as libc::c_int)
                    as libc::c_ulong,
            };
            init
        },
        {
            let mut init = pe_required_section_flags {
                section_name: *::core::mem::transmute::<
                    &[u8; 8],
                    &mut [libc::c_char; 8],
                >(b".bss\0\0\0\0"),
                must_have: ((0x40000000 as libc::c_int | 0x80 as libc::c_int)
                    as libc::c_uint | 0x80000000 as libc::c_uint) as libc::c_ulong,
            };
            init
        },
        {
            let mut init = pe_required_section_flags {
                section_name: *::core::mem::transmute::<
                    &[u8; 8],
                    &mut [libc::c_char; 8],
                >(b".data\0\0\0"),
                must_have: ((0x40000000 as libc::c_int | 0x40 as libc::c_int)
                    as libc::c_uint | 0x80000000 as libc::c_uint) as libc::c_ulong,
            };
            init
        },
        {
            let mut init = pe_required_section_flags {
                section_name: *::core::mem::transmute::<
                    &[u8; 8],
                    &mut [libc::c_char; 8],
                >(b".edata\0\0"),
                must_have: (0x40000000 as libc::c_int | 0x40 as libc::c_int)
                    as libc::c_ulong,
            };
            init
        },
        {
            let mut init = pe_required_section_flags {
                section_name: *::core::mem::transmute::<
                    &[u8; 8],
                    &mut [libc::c_char; 8],
                >(b".idata\0\0"),
                must_have: ((0x40000000 as libc::c_int | 0x40 as libc::c_int)
                    as libc::c_uint | 0x80000000 as libc::c_uint) as libc::c_ulong,
            };
            init
        },
        {
            let mut init = pe_required_section_flags {
                section_name: *::core::mem::transmute::<
                    &[u8; 8],
                    &mut [libc::c_char; 8],
                >(b".pdata\0\0"),
                must_have: (0x40000000 as libc::c_int | 0x40 as libc::c_int)
                    as libc::c_ulong,
            };
            init
        },
        {
            let mut init = pe_required_section_flags {
                section_name: *::core::mem::transmute::<
                    &[u8; 8],
                    &mut [libc::c_char; 8],
                >(b".rdata\0\0"),
                must_have: (0x40000000 as libc::c_int | 0x40 as libc::c_int)
                    as libc::c_ulong,
            };
            init
        },
        {
            let mut init = pe_required_section_flags {
                section_name: *::core::mem::transmute::<
                    &[u8; 8],
                    &mut [libc::c_char; 8],
                >(b".reloc\0\0"),
                must_have: (0x40000000 as libc::c_int | 0x40 as libc::c_int
                    | 0x2000000 as libc::c_int) as libc::c_ulong,
            };
            init
        },
        {
            let mut init = pe_required_section_flags {
                section_name: *::core::mem::transmute::<
                    &[u8; 8],
                    &mut [libc::c_char; 8],
                >(b".rsrc\0\0\0"),
                must_have: ((0x40000000 as libc::c_int | 0x40 as libc::c_int)
                    as libc::c_uint | 0x80000000 as libc::c_uint) as libc::c_ulong,
            };
            init
        },
        {
            let mut init = pe_required_section_flags {
                section_name: *::core::mem::transmute::<
                    &[u8; 8],
                    &mut [libc::c_char; 8],
                >(b".text\0\0\0"),
                must_have: (0x40000000 as libc::c_int | 0x20 as libc::c_int
                    | 0x20000000 as libc::c_int) as libc::c_ulong,
            };
            init
        },
        {
            let mut init = pe_required_section_flags {
                section_name: *::core::mem::transmute::<
                    &[u8; 8],
                    &mut [libc::c_char; 8],
                >(b".tls\0\0\0\0"),
                must_have: ((0x40000000 as libc::c_int | 0x40 as libc::c_int)
                    as libc::c_uint | 0x80000000 as libc::c_uint) as libc::c_ulong,
            };
            init
        },
        {
            let mut init = pe_required_section_flags {
                section_name: *::core::mem::transmute::<
                    &[u8; 8],
                    &mut [libc::c_char; 8],
                >(b".xdata\0\0"),
                must_have: (0x40000000 as libc::c_int | 0x40 as libc::c_int)
                    as libc::c_ulong,
            };
            init
        },
    ];
    let mut p: *mut pe_required_section_flags = 0 as *mut pe_required_section_flags;
    p = known_sections.as_mut_ptr();
    while p
        < known_sections
            .as_mut_ptr()
            .offset(
                (::core::mem::size_of::<[pe_required_section_flags; 12]>()
                    as libc::c_ulong)
                    .wrapping_div(
                        ::core::mem::size_of::<pe_required_section_flags>()
                            as libc::c_ulong,
                    ) as isize,
            )
    {
        if memcmp(
            ((*scnhdr_int).s_name).as_mut_ptr() as *const libc::c_void,
            ((*p).section_name).as_mut_ptr() as *const libc::c_void,
            8 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            if memcmp(
                ((*scnhdr_int).s_name).as_mut_ptr() as *const libc::c_void,
                b".text\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                ::core::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong,
            ) != 0 || bfd_get_file_flags(abfd) & 0x80 as libc::c_int as libc::c_uint != 0
            {
                (*scnhdr_int).s_flags &= !(0x80000000 as libc::c_uint) as libc::c_ulong;
            }
            (*scnhdr_int).s_flags |= (*p).must_have;
            break;
        } else {
            p = p.offset(1);
            p;
        }
    }
    (Some(((*(*abfd).xvec).bfd_h_putx32).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (*scnhdr_int).s_flags,
        ((*scnhdr_ext).s_flags).as_mut_ptr() as *mut libc::c_void,
    );
    if !((*(*abfd).tdata.coff_obj_data).link_info).is_null()
        && !((*(*(*abfd).tdata.coff_obj_data).link_info).type_0() as libc::c_int
            == type_relocatable as libc::c_int)
        && !((*(*(*abfd).tdata.coff_obj_data).link_info).type_0() as libc::c_int
            == type_dll as libc::c_int
            || (*(*(*abfd).tdata.coff_obj_data).link_info).type_0() as libc::c_int
                == type_pie as libc::c_int)
        && memcmp(
            ((*scnhdr_int).s_name).as_mut_ptr() as *const libc::c_void,
            b".text\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            ::core::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong,
        ) == 0 as libc::c_int
    {
        (Some(((*(*abfd).xvec).bfd_h_putx16).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            (*scnhdr_int).s_nlnno & 0xffff as libc::c_int as libc::c_ulong,
            ((*scnhdr_ext).s_nlnno).as_mut_ptr() as *mut libc::c_void,
        );
        (Some(((*(*abfd).xvec).bfd_h_putx16).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            (*scnhdr_int).s_nlnno >> 16 as libc::c_int,
            ((*scnhdr_ext).s_nreloc).as_mut_ptr() as *mut libc::c_void,
        );
    } else {
        if (*scnhdr_int).s_nlnno <= 0xffff as libc::c_int as libc::c_ulong {
            (Some(((*(*abfd).xvec).bfd_h_putx16).expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                (*scnhdr_int).s_nlnno,
                ((*scnhdr_ext).s_nlnno).as_mut_ptr() as *mut libc::c_void,
            );
        } else {
            _bfd_error_handler(
                dcgettext(
                    b"bfd\0" as *const u8 as *const libc::c_char,
                    b"%pB: line number overflow: 0x%lx > 0xffff\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                abfd,
                (*scnhdr_int).s_nlnno,
            );
            bfd_set_error(bfd_error_file_truncated);
            (Some(((*(*abfd).xvec).bfd_h_putx16).expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                0xffff as libc::c_int as bfd_vma,
                ((*scnhdr_ext).s_nlnno).as_mut_ptr() as *mut libc::c_void,
            );
            ret = 0 as libc::c_int as libc::c_uint;
        }
        if (*scnhdr_int).s_nreloc < 0xffff as libc::c_int as libc::c_ulong {
            (Some(((*(*abfd).xvec).bfd_h_putx16).expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                (*scnhdr_int).s_nreloc,
                ((*scnhdr_ext).s_nreloc).as_mut_ptr() as *mut libc::c_void,
            );
        } else {
            (Some(((*(*abfd).xvec).bfd_h_putx16).expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                0xffff as libc::c_int as bfd_vma,
                ((*scnhdr_ext).s_nreloc).as_mut_ptr() as *mut libc::c_void,
            );
            (*scnhdr_int).s_flags |= 0x1000000 as libc::c_int as libc::c_ulong;
            (Some(((*(*abfd).xvec).bfd_h_putx32).expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                (*scnhdr_int).s_flags,
                ((*scnhdr_ext).s_flags).as_mut_ptr() as *mut libc::c_void,
            );
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn _bfd_pex64i_swap_debugdir_in(
    mut abfd: *mut bfd,
    mut ext1: *mut libc::c_void,
    mut in1: *mut libc::c_void,
) {
    let mut ext: *mut external_IMAGE_DEBUG_DIRECTORY = ext1
        as *mut external_IMAGE_DEBUG_DIRECTORY;
    let mut in_0: *mut internal_IMAGE_DEBUG_DIRECTORY = in1
        as *mut internal_IMAGE_DEBUG_DIRECTORY;
    (*in_0)
        .Characteristics = (Some(
        ((*(*abfd).xvec).bfd_h_getx32).expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )(((*ext).Characteristics).as_mut_ptr() as *const libc::c_void);
    (*in_0)
        .TimeDateStamp = (Some(
        ((*(*abfd).xvec).bfd_h_getx32).expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )(((*ext).TimeDateStamp).as_mut_ptr() as *const libc::c_void);
    (*in_0)
        .MajorVersion = (Some(
        ((*(*abfd).xvec).bfd_h_getx16).expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )(((*ext).MajorVersion).as_mut_ptr() as *const libc::c_void) as libc::c_ushort;
    (*in_0)
        .MinorVersion = (Some(
        ((*(*abfd).xvec).bfd_h_getx16).expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )(((*ext).MinorVersion).as_mut_ptr() as *const libc::c_void) as libc::c_ushort;
    (*in_0)
        .Type = (Some(
        ((*(*abfd).xvec).bfd_h_getx32).expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )(((*ext).Type).as_mut_ptr() as *const libc::c_void);
    (*in_0)
        .SizeOfData = (Some(
        ((*(*abfd).xvec).bfd_h_getx32).expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )(((*ext).SizeOfData).as_mut_ptr() as *const libc::c_void);
    (*in_0)
        .AddressOfRawData = (Some(
        ((*(*abfd).xvec).bfd_h_getx32).expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )(((*ext).AddressOfRawData).as_mut_ptr() as *const libc::c_void);
    (*in_0)
        .PointerToRawData = (Some(
        ((*(*abfd).xvec).bfd_h_getx32).expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )(((*ext).PointerToRawData).as_mut_ptr() as *const libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn _bfd_pex64i_swap_debugdir_out(
    mut abfd: *mut bfd,
    mut inp: *mut libc::c_void,
    mut extp: *mut libc::c_void,
) -> libc::c_uint {
    let mut ext: *mut external_IMAGE_DEBUG_DIRECTORY = extp
        as *mut external_IMAGE_DEBUG_DIRECTORY;
    let mut in_0: *mut internal_IMAGE_DEBUG_DIRECTORY = inp
        as *mut internal_IMAGE_DEBUG_DIRECTORY;
    (Some(((*(*abfd).xvec).bfd_h_putx32).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (*in_0).Characteristics,
        ((*ext).Characteristics).as_mut_ptr() as *mut libc::c_void,
    );
    (Some(((*(*abfd).xvec).bfd_h_putx32).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (*in_0).TimeDateStamp,
        ((*ext).TimeDateStamp).as_mut_ptr() as *mut libc::c_void,
    );
    (Some(((*(*abfd).xvec).bfd_h_putx16).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (*in_0).MajorVersion as bfd_vma,
        ((*ext).MajorVersion).as_mut_ptr() as *mut libc::c_void,
    );
    (Some(((*(*abfd).xvec).bfd_h_putx16).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (*in_0).MinorVersion as bfd_vma,
        ((*ext).MinorVersion).as_mut_ptr() as *mut libc::c_void,
    );
    (Some(((*(*abfd).xvec).bfd_h_putx32).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )((*in_0).Type, ((*ext).Type).as_mut_ptr() as *mut libc::c_void);
    (Some(((*(*abfd).xvec).bfd_h_putx32).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )((*in_0).SizeOfData, ((*ext).SizeOfData).as_mut_ptr() as *mut libc::c_void);
    (Some(((*(*abfd).xvec).bfd_h_putx32).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (*in_0).AddressOfRawData,
        ((*ext).AddressOfRawData).as_mut_ptr() as *mut libc::c_void,
    );
    (Some(((*(*abfd).xvec).bfd_h_putx32).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (*in_0).PointerToRawData,
        ((*ext).PointerToRawData).as_mut_ptr() as *mut libc::c_void,
    );
    return ::core::mem::size_of::<external_IMAGE_DEBUG_DIRECTORY>() as libc::c_ulong
        as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn _bfd_pex64i_slurp_codeview_record(
    mut abfd: *mut bfd,
    mut where_0: file_ptr,
    mut length: libc::c_ulong,
    mut cvinfo: *mut CODEVIEW_INFO,
) -> *mut CODEVIEW_INFO {
    let mut buffer: [libc::c_char; 257] = [0; 257];
    let mut nread: bfd_size_type = 0;
    if bfd_seek(abfd, where_0, 0 as libc::c_int) != 0 as libc::c_int {
        return 0 as *mut CODEVIEW_INFO;
    }
    if length <= ::core::mem::size_of::<CV_INFO_PDB70>() as libc::c_ulong
        && length <= ::core::mem::size_of::<CV_INFO_PDB20>() as libc::c_ulong
    {
        return 0 as *mut CODEVIEW_INFO;
    }
    if length > 256 as libc::c_int as libc::c_ulong {
        length = 256 as libc::c_int as libc::c_ulong;
    }
    nread = bfd_bread(buffer.as_mut_ptr() as *mut libc::c_void, length, abfd);
    if length != nread {
        return 0 as *mut CODEVIEW_INFO;
    }
    memset(
        buffer.as_mut_ptr().offset(nread as isize) as *mut libc::c_void,
        0 as libc::c_int,
        (::core::mem::size_of::<[libc::c_char; 257]>() as libc::c_ulong)
            .wrapping_sub(nread),
    );
    (*cvinfo)
        .CVSignature = (Some(
        ((*(*abfd).xvec).bfd_h_getx32).expect("non-null function pointer"),
    ))
        .expect("non-null function pointer")(buffer.as_mut_ptr() as *const libc::c_void);
    (*cvinfo).Age = 0 as libc::c_int as libc::c_ulong;
    if (*cvinfo).CVSignature == 0x53445352 as libc::c_int as libc::c_ulong
        && length > ::core::mem::size_of::<CV_INFO_PDB70>() as libc::c_ulong
    {
        let mut cvinfo70: *mut CV_INFO_PDB70 = buffer.as_mut_ptr() as *mut CV_INFO_PDB70;
        (*cvinfo)
            .Age = (Some(
            ((*(*abfd).xvec).bfd_h_getx32).expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(((*cvinfo70).Age).as_mut_ptr() as *const libc::c_void);
        bfd_putb32(
            bfd_getl32(((*cvinfo70).Signature).as_mut_ptr() as *const libc::c_void),
            ((*cvinfo).Signature).as_mut_ptr() as *mut libc::c_void,
        );
        bfd_putb16(
            bfd_getl16(
                &mut *((*cvinfo70).Signature)
                    .as_mut_ptr()
                    .offset(4 as libc::c_int as isize) as *mut libc::c_char
                    as *const libc::c_void,
            ),
            &mut *((*cvinfo).Signature).as_mut_ptr().offset(4 as libc::c_int as isize)
                as *mut libc::c_char as *mut libc::c_void,
        );
        bfd_putb16(
            bfd_getl16(
                &mut *((*cvinfo70).Signature)
                    .as_mut_ptr()
                    .offset(6 as libc::c_int as isize) as *mut libc::c_char
                    as *const libc::c_void,
            ),
            &mut *((*cvinfo).Signature).as_mut_ptr().offset(6 as libc::c_int as isize)
                as *mut libc::c_char as *mut libc::c_void,
        );
        memcpy(
            &mut *((*cvinfo).Signature).as_mut_ptr().offset(8 as libc::c_int as isize)
                as *mut libc::c_char as *mut libc::c_void,
            &mut *((*cvinfo70).Signature).as_mut_ptr().offset(8 as libc::c_int as isize)
                as *mut libc::c_char as *const libc::c_void,
            8 as libc::c_int as libc::c_ulong,
        );
        (*cvinfo).SignatureLength = 16 as libc::c_int as libc::c_uint;
        return cvinfo;
    } else if (*cvinfo).CVSignature == 0x3031424e as libc::c_int as libc::c_ulong
        && length > ::core::mem::size_of::<CV_INFO_PDB20>() as libc::c_ulong
    {
        let mut cvinfo20: *mut CV_INFO_PDB20 = buffer.as_mut_ptr() as *mut CV_INFO_PDB20;
        (*cvinfo)
            .Age = (Some(
            ((*(*abfd).xvec).bfd_h_getx32).expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(((*cvinfo20).Age).as_mut_ptr() as *const libc::c_void);
        memcpy(
            ((*cvinfo).Signature).as_mut_ptr() as *mut libc::c_void,
            ((*cvinfo20).Signature).as_mut_ptr() as *const libc::c_void,
            4 as libc::c_int as libc::c_ulong,
        );
        (*cvinfo).SignatureLength = 4 as libc::c_int as libc::c_uint;
        return cvinfo;
    }
    return 0 as *mut CODEVIEW_INFO;
}
#[no_mangle]
pub unsafe extern "C" fn _bfd_pex64i_write_codeview_record(
    mut abfd: *mut bfd,
    mut where_0: file_ptr,
    mut cvinfo: *mut CODEVIEW_INFO,
) -> libc::c_uint {
    let size: bfd_size_type = (::core::mem::size_of::<CV_INFO_PDB70>() as libc::c_ulong)
        .wrapping_add(1 as libc::c_int as libc::c_ulong);
    let mut written: bfd_size_type = 0;
    let mut cvinfo70: *mut CV_INFO_PDB70 = 0 as *mut CV_INFO_PDB70;
    let mut buffer: *mut libc::c_char = 0 as *mut libc::c_char;
    if bfd_seek(abfd, where_0, 0 as libc::c_int) != 0 as libc::c_int {
        return 0 as libc::c_int as libc::c_uint;
    }
    buffer = bfd_malloc(size) as *mut libc::c_char;
    if buffer.is_null() {
        return 0 as libc::c_int as libc::c_uint;
    }
    cvinfo70 = buffer as *mut CV_INFO_PDB70;
    (Some(((*(*abfd).xvec).bfd_h_putx32).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        0x53445352 as libc::c_int as bfd_vma,
        ((*cvinfo70).CvSignature).as_mut_ptr() as *mut libc::c_void,
    );
    bfd_putl32(
        bfd_getb32(((*cvinfo).Signature).as_mut_ptr() as *const libc::c_void),
        ((*cvinfo70).Signature).as_mut_ptr() as *mut libc::c_void,
    );
    bfd_putl16(
        bfd_getb16(
            &mut *((*cvinfo).Signature).as_mut_ptr().offset(4 as libc::c_int as isize)
                as *mut libc::c_char as *const libc::c_void,
        ),
        &mut *((*cvinfo70).Signature).as_mut_ptr().offset(4 as libc::c_int as isize)
            as *mut libc::c_char as *mut libc::c_void,
    );
    bfd_putl16(
        bfd_getb16(
            &mut *((*cvinfo).Signature).as_mut_ptr().offset(6 as libc::c_int as isize)
                as *mut libc::c_char as *const libc::c_void,
        ),
        &mut *((*cvinfo70).Signature).as_mut_ptr().offset(6 as libc::c_int as isize)
            as *mut libc::c_char as *mut libc::c_void,
    );
    memcpy(
        &mut *((*cvinfo70).Signature).as_mut_ptr().offset(8 as libc::c_int as isize)
            as *mut libc::c_char as *mut libc::c_void,
        &mut *((*cvinfo).Signature).as_mut_ptr().offset(8 as libc::c_int as isize)
            as *mut libc::c_char as *const libc::c_void,
        8 as libc::c_int as libc::c_ulong,
    );
    (Some(((*(*abfd).xvec).bfd_h_putx32).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )((*cvinfo).Age, ((*cvinfo70).Age).as_mut_ptr() as *mut libc::c_void);
    *((*cvinfo70).PdbFileName)
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    written = bfd_bwrite(buffer as *const libc::c_void, size, abfd);
    free(buffer as *mut libc::c_void);
    return (if written == size { size } else { 0 as libc::c_int as libc::c_ulong })
        as libc::c_uint;
}
static mut dir_names: [*mut libc::c_char; 16] = [
    b"Export Directory [.edata (or where ever we found it)]\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"Import Directory [parts of .idata]\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"Resource Directory [.rsrc]\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"Exception Directory [.pdata]\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"Security Directory\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Base Relocation Directory [.reloc]\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"Debug Directory\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Description Directory\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Special Directory\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Thread Storage Directory [.tls]\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"Load Configuration Directory\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"Bound Import Directory\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Import Address Table Directory\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"Delay Import Directory\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"CLR Runtime Header\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Reserved\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
];
unsafe extern "C" fn pe_print_idata(
    mut abfd: *mut bfd,
    mut vfile: *mut libc::c_void,
) -> bool {
    let mut file: *mut FILE = vfile as *mut FILE;
    let mut data: *mut bfd_byte = 0 as *mut bfd_byte;
    let mut section: *mut asection = 0 as *mut asection;
    let mut adj: bfd_signed_vma = 0;
    let mut datasize: bfd_size_type = 0 as libc::c_int as bfd_size_type;
    let mut dataoff: bfd_size_type = 0;
    let mut i: bfd_size_type = 0;
    let mut onaline: libc::c_int = 20 as libc::c_int;
    let mut pe: *mut pe_data_type = (*abfd).tdata.pe_obj_data;
    let mut extra: *mut internal_extra_pe_aouthdr = &mut (*pe).pe_opthdr;
    let mut addr: bfd_vma = 0;
    addr = (*extra).DataDirectory[1 as libc::c_int as usize].VirtualAddress;
    if addr == 0 as libc::c_int as libc::c_ulong
        && (*extra).DataDirectory[1 as libc::c_int as usize].Size
            == 0 as libc::c_int as libc::c_long
    {
        section = bfd_get_section_by_name(
            abfd,
            b".idata\0" as *const u8 as *const libc::c_char,
        );
        if section.is_null() {
            return 1 as libc::c_int != 0;
        }
        addr = (*section).vma;
        datasize = (*section).size;
        if datasize == 0 as libc::c_int as libc::c_ulong {
            return 1 as libc::c_int != 0;
        }
    } else {
        addr = (addr as libc::c_ulong).wrapping_add((*extra).ImageBase) as bfd_vma
            as bfd_vma;
        section = (*abfd).sections;
        while !section.is_null() {
            datasize = (*section).size;
            if addr >= (*section).vma && addr < ((*section).vma).wrapping_add(datasize) {
                break;
            }
            section = (*section).next;
        }
        if section.is_null() {
            fprintf(
                file,
                dcgettext(
                    b"bfd\0" as *const u8 as *const libc::c_char,
                    b"\nThere is an import table, but the section containing it could not be found\n\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            return 1 as libc::c_int != 0;
        } else if (*section).flags & 0x100 as libc::c_int as libc::c_uint == 0 {
            fprintf(
                file,
                dcgettext(
                    b"bfd\0" as *const u8 as *const libc::c_char,
                    b"\nThere is an import table in %s, but that section has no contents\n\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*section).name,
            );
            return 1 as libc::c_int != 0;
        }
    }
    fprintf(
        file,
        dcgettext(
            b"bfd\0" as *const u8 as *const libc::c_char,
            b"\nThere is an import table in %s at 0x%lx\n\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
        (*section).name,
        addr,
    );
    dataoff = addr.wrapping_sub((*section).vma);
    fprintf(
        file,
        dcgettext(
            b"bfd\0" as *const u8 as *const libc::c_char,
            b"\nThe Import Tables (interpreted %s section contents)\n\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
        (*section).name,
    );
    fprintf(
        file,
        dcgettext(
            b"bfd\0" as *const u8 as *const libc::c_char,
            b" vma:            Hint    Time      Forward  DLL       First\n                 Table   Stamp     Chain    Name      Thunk\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    if !bfd_malloc_and_get_section(abfd, section, &mut data) {
        free(data as *mut libc::c_void);
        return 0 as libc::c_int != 0;
    }
    adj = ((*section).vma).wrapping_sub((*extra).ImageBase) as bfd_signed_vma;
    let mut current_block_75: u64;
    i = dataoff;
    while i.wrapping_add(onaline as libc::c_ulong) <= datasize {
        let mut hint_addr: bfd_vma = 0;
        let mut time_stamp: bfd_vma = 0;
        let mut forward_chain: bfd_vma = 0;
        let mut dll_name: bfd_vma = 0;
        let mut first_thunk: bfd_vma = 0;
        let mut idx: libc::c_int = 0 as libc::c_int;
        let mut j: bfd_size_type = 0;
        let mut dll: *mut libc::c_char = 0 as *mut libc::c_char;
        fprintf(
            file,
            b" %08lx\t\0" as *const u8 as *const libc::c_char,
            i.wrapping_add(adj as libc::c_ulong),
        );
        hint_addr = (Some(
            ((*(*abfd).xvec).bfd_getx32).expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(data.offset(i as isize) as *const libc::c_void);
        time_stamp = (Some(
            ((*(*abfd).xvec).bfd_getx32).expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            data.offset(i as isize).offset(4 as libc::c_int as isize)
                as *const libc::c_void,
        );
        forward_chain = (Some(
            ((*(*abfd).xvec).bfd_getx32).expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            data.offset(i as isize).offset(8 as libc::c_int as isize)
                as *const libc::c_void,
        );
        dll_name = (Some(
            ((*(*abfd).xvec).bfd_getx32).expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            data.offset(i as isize).offset(12 as libc::c_int as isize)
                as *const libc::c_void,
        );
        first_thunk = (Some(
            ((*(*abfd).xvec).bfd_getx32).expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            data.offset(i as isize).offset(16 as libc::c_int as isize)
                as *const libc::c_void,
        );
        fprintf(
            file,
            b"%08lx %08lx %08lx %08lx %08lx\n\0" as *const u8 as *const libc::c_char,
            hint_addr,
            time_stamp,
            forward_chain,
            dll_name,
            first_thunk,
        );
        if hint_addr == 0 as libc::c_int as libc::c_ulong
            && first_thunk == 0 as libc::c_int as libc::c_ulong
        {
            break;
        }
        if dll_name.wrapping_sub(adj as libc::c_ulong) >= (*section).size {
            break;
        }
        dll = (data as *mut libc::c_char)
            .offset(dll_name as isize)
            .offset(-(adj as isize));
        let mut maxlen: bfd_size_type = ((data.offset(datasize as isize)
            as *mut libc::c_char)
            .offset_from(dll) as libc::c_long - 1 as libc::c_int as libc::c_long)
            as bfd_size_type;
        fprintf(
            file,
            dcgettext(
                b"bfd\0" as *const u8 as *const libc::c_char,
                b"\n\tDLL Name: %.*s\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            maxlen as libc::c_int,
            dll,
        );
        if hint_addr == 0 as libc::c_int as libc::c_ulong {
            hint_addr = first_thunk;
        }
        if hint_addr != 0 as libc::c_int as libc::c_ulong
            && hint_addr.wrapping_sub(adj as libc::c_ulong) < datasize
        {
            let mut ft_data: *mut bfd_byte = 0 as *mut bfd_byte;
            let mut ft_section: *mut asection = 0 as *mut asection;
            let mut ft_addr: bfd_vma = 0;
            let mut ft_datasize: bfd_size_type = 0;
            let mut ft_idx: libc::c_int = 0;
            let mut ft_allocated: libc::c_int = 0;
            fprintf(
                file,
                dcgettext(
                    b"bfd\0" as *const u8 as *const libc::c_char,
                    b"\tvma:  Hint/Ord Member-Name Bound-To\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            idx = hint_addr.wrapping_sub(adj as libc::c_ulong) as libc::c_int;
            ft_addr = first_thunk.wrapping_add((*extra).ImageBase);
            ft_idx = first_thunk.wrapping_sub(adj as libc::c_ulong) as libc::c_int;
            ft_data = data.offset(ft_idx as isize);
            ft_datasize = datasize.wrapping_sub(ft_idx as libc::c_ulong);
            ft_allocated = 0 as libc::c_int;
            if first_thunk != hint_addr {
                ft_section = (*abfd).sections;
                while !ft_section.is_null() {
                    if ft_addr >= (*ft_section).vma
                        && ft_addr < ((*ft_section).vma).wrapping_add((*ft_section).size)
                    {
                        break;
                    }
                    ft_section = (*ft_section).next;
                }
                if ft_section.is_null() {
                    fprintf(
                        file,
                        dcgettext(
                            b"bfd\0" as *const u8 as *const libc::c_char,
                            b"\nThere is a first thunk, but the section containing it could not be found\n\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    current_block_75 = 16924917904204750491;
                } else if ft_section != section {
                    ft_idx = first_thunk
                        .wrapping_sub(
                            ((*ft_section).vma).wrapping_sub((*extra).ImageBase),
                        ) as libc::c_int;
                    ft_datasize = ((*ft_section).size)
                        .wrapping_sub(ft_idx as libc::c_ulong);
                    ft_data = bfd_malloc(ft_datasize) as *mut bfd_byte;
                    if ft_data.is_null() {
                        current_block_75 = 16924917904204750491;
                    } else if !bfd_get_section_contents(
                        abfd,
                        ft_section,
                        ft_data as *mut libc::c_void,
                        ft_idx as bfd_vma as file_ptr,
                        ft_datasize,
                    ) {
                        free(ft_data as *mut libc::c_void);
                        current_block_75 = 16924917904204750491;
                    } else {
                        ft_allocated = 1 as libc::c_int;
                        current_block_75 = 5597585068398118923;
                    }
                } else {
                    current_block_75 = 5597585068398118923;
                }
            } else {
                current_block_75 = 5597585068398118923;
            }
            match current_block_75 {
                16924917904204750491 => {}
                _ => {
                    j = 0 as libc::c_int as bfd_size_type;
                    while (idx as libc::c_ulong)
                        .wrapping_add(j)
                        .wrapping_add(8 as libc::c_int as libc::c_ulong) <= datasize
                    {
                        let mut amt: bfd_size_type = 0;
                        let mut member: libc::c_ulong = (Some(
                            ((*(*abfd).xvec).bfd_getx32)
                                .expect("non-null function pointer"),
                        ))
                            .expect(
                                "non-null function pointer",
                            )(
                            data.offset(idx as isize).offset(j as isize)
                                as *const libc::c_void,
                        );
                        let mut member_high: libc::c_ulong = (Some(
                            ((*(*abfd).xvec).bfd_getx32)
                                .expect("non-null function pointer"),
                        ))
                            .expect(
                                "non-null function pointer",
                            )(
                            data
                                .offset(idx as isize)
                                .offset(j as isize)
                                .offset(4 as libc::c_int as isize) as *const libc::c_void,
                        );
                        if member == 0 && member_high == 0 {
                            break;
                        }
                        amt = member.wrapping_sub(adj as libc::c_ulong);
                        if member_high & 0x80000000 as libc::c_uint as libc::c_ulong != 0
                        {
                            fprintf(
                                file,
                                b"\t%lx%08lx\t %4lx%08lx  <none>\0" as *const u8
                                    as *const libc::c_char,
                                member_high,
                                member,
                                member_high & 0x7fffffff as libc::c_int as libc::c_ulong,
                                member,
                            );
                        } else if amt >= datasize
                            || amt.wrapping_add(2 as libc::c_int as libc::c_ulong)
                                >= datasize
                        {
                            fprintf(
                                file,
                                dcgettext(
                                    b"bfd\0" as *const u8 as *const libc::c_char,
                                    b"\t<corrupt: 0x%04lx>\0" as *const u8
                                        as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                                member,
                            );
                        } else {
                            let mut ordinal: libc::c_int = 0;
                            let mut member_name: *mut libc::c_char = 0
                                as *mut libc::c_char;
                            ordinal = (Some(
                                ((*(*abfd).xvec).bfd_getx16)
                                    .expect("non-null function pointer"),
                            ))
                                .expect(
                                    "non-null function pointer",
                                )(data.offset(amt as isize) as *const libc::c_void)
                                as libc::c_int;
                            member_name = (data as *mut libc::c_char)
                                .offset(amt as isize)
                                .offset(2 as libc::c_int as isize);
                            fprintf(
                                file,
                                b"\t%04lx\t %4d  %.*s\0" as *const u8
                                    as *const libc::c_char,
                                member,
                                ordinal,
                                datasize
                                    .wrapping_sub(
                                        amt.wrapping_add(2 as libc::c_int as libc::c_ulong),
                                    ) as libc::c_int,
                                member_name,
                            );
                        }
                        if time_stamp != 0 as libc::c_int as libc::c_ulong
                            && first_thunk != 0 as libc::c_int as libc::c_ulong
                            && first_thunk != hint_addr
                            && j.wrapping_add(4 as libc::c_int as libc::c_ulong)
                                <= ft_datasize
                        {
                            fprintf(
                                file,
                                b"\t%04lx\0" as *const u8 as *const libc::c_char,
                                (Some(
                                    ((*(*abfd).xvec).bfd_getx32)
                                        .expect("non-null function pointer"),
                                ))
                                    .expect(
                                        "non-null function pointer",
                                    )(ft_data.offset(j as isize) as *const libc::c_void),
                            );
                        }
                        fprintf(file, b"\n\0" as *const u8 as *const libc::c_char);
                        j = (j as libc::c_ulong)
                            .wrapping_add(8 as libc::c_int as libc::c_ulong)
                            as bfd_size_type as bfd_size_type;
                    }
                    if ft_allocated != 0 {
                        free(ft_data as *mut libc::c_void);
                    }
                    current_block_75 = 8464383504555462953;
                }
            }
        } else {
            current_block_75 = 8464383504555462953;
        }
        match current_block_75 {
            8464383504555462953 => {
                fprintf(file, b"\n\0" as *const u8 as *const libc::c_char);
            }
            _ => {}
        }
        i = (i as libc::c_ulong).wrapping_add(onaline as libc::c_ulong) as bfd_size_type
            as bfd_size_type;
    }
    free(data as *mut libc::c_void);
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn pe_print_edata(
    mut abfd: *mut bfd,
    mut vfile: *mut libc::c_void,
) -> bool {
    let mut file: *mut FILE = vfile as *mut FILE;
    let mut data: *mut bfd_byte = 0 as *mut bfd_byte;
    let mut section: *mut asection = 0 as *mut asection;
    let mut datasize: bfd_size_type = 0 as libc::c_int as bfd_size_type;
    let mut dataoff: bfd_size_type = 0;
    let mut i: bfd_size_type = 0;
    let mut adj: bfd_vma = 0;
    let mut edt: EDT_type = EDT_type {
        export_flags: 0,
        time_stamp: 0,
        major_ver: 0,
        minor_ver: 0,
        name: 0,
        base: 0,
        num_functions: 0,
        num_names: 0,
        eat_addr: 0,
        npt_addr: 0,
        ot_addr: 0,
    };
    let mut pe: *mut pe_data_type = (*abfd).tdata.pe_obj_data;
    let mut extra: *mut internal_extra_pe_aouthdr = &mut (*pe).pe_opthdr;
    let mut addr: bfd_vma = 0;
    addr = (*extra).DataDirectory[0 as libc::c_int as usize].VirtualAddress;
    if addr == 0 as libc::c_int as libc::c_ulong
        && (*extra).DataDirectory[0 as libc::c_int as usize].Size
            == 0 as libc::c_int as libc::c_long
    {
        section = bfd_get_section_by_name(
            abfd,
            b".edata\0" as *const u8 as *const libc::c_char,
        );
        if section.is_null() {
            return 1 as libc::c_int != 0;
        }
        addr = (*section).vma;
        dataoff = 0 as libc::c_int as bfd_size_type;
        datasize = (*section).size;
        if datasize == 0 as libc::c_int as libc::c_ulong {
            return 1 as libc::c_int != 0;
        }
    } else {
        addr = (addr as libc::c_ulong).wrapping_add((*extra).ImageBase) as bfd_vma
            as bfd_vma;
        section = (*abfd).sections;
        while !section.is_null() {
            if addr >= (*section).vma
                && addr < ((*section).vma).wrapping_add((*section).size)
            {
                break;
            }
            section = (*section).next;
        }
        if section.is_null() {
            fprintf(
                file,
                dcgettext(
                    b"bfd\0" as *const u8 as *const libc::c_char,
                    b"\nThere is an export table, but the section containing it could not be found\n\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            return 1 as libc::c_int != 0;
        } else if (*section).flags & 0x100 as libc::c_int as libc::c_uint == 0 {
            fprintf(
                file,
                dcgettext(
                    b"bfd\0" as *const u8 as *const libc::c_char,
                    b"\nThere is an export table in %s, but that section has no contents\n\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*section).name,
            );
            return 1 as libc::c_int != 0;
        }
        dataoff = addr.wrapping_sub((*section).vma);
        datasize = (*extra).DataDirectory[0 as libc::c_int as usize].Size
            as bfd_size_type;
        if dataoff > (*section).size
            || datasize > ((*section).size).wrapping_sub(dataoff)
        {
            fprintf(
                file,
                dcgettext(
                    b"bfd\0" as *const u8 as *const libc::c_char,
                    b"\nThere is an export table in %s, but it does not fit into that section\n\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*section).name,
            );
            return 1 as libc::c_int != 0;
        }
    }
    if datasize < 40 as libc::c_int as libc::c_ulong {
        fprintf(
            file,
            dcgettext(
                b"bfd\0" as *const u8 as *const libc::c_char,
                b"\nThere is an export table in %s, but it is too small (%d)\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            (*section).name,
            datasize as libc::c_int,
        );
        return 1 as libc::c_int != 0;
    }
    fprintf(
        file,
        dcgettext(
            b"bfd\0" as *const u8 as *const libc::c_char,
            b"\nThere is an export table in %s at 0x%lx\n\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
        (*section).name,
        addr,
    );
    data = bfd_malloc(datasize) as *mut bfd_byte;
    if data.is_null() {
        return 0 as libc::c_int != 0;
    }
    if !bfd_get_section_contents(
        abfd,
        section,
        data as *mut libc::c_void,
        dataoff as file_ptr,
        datasize,
    ) {
        return 0 as libc::c_int != 0;
    }
    edt
        .export_flags = (Some(
        ((*(*abfd).xvec).bfd_getx32).expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )(data.offset(0 as libc::c_int as isize) as *const libc::c_void) as libc::c_long;
    edt
        .time_stamp = (Some(
        ((*(*abfd).xvec).bfd_getx32).expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )(data.offset(4 as libc::c_int as isize) as *const libc::c_void) as libc::c_long;
    edt
        .major_ver = (Some(
        ((*(*abfd).xvec).bfd_getx16).expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )(data.offset(8 as libc::c_int as isize) as *const libc::c_void)
        as libc::c_short;
    edt
        .minor_ver = (Some(
        ((*(*abfd).xvec).bfd_getx16).expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )(data.offset(10 as libc::c_int as isize) as *const libc::c_void)
        as libc::c_short;
    edt
        .name = (Some(((*(*abfd).xvec).bfd_getx32).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(data.offset(12 as libc::c_int as isize) as *const libc::c_void);
    edt
        .base = (Some(((*(*abfd).xvec).bfd_getx32).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(data.offset(16 as libc::c_int as isize) as *const libc::c_void)
        as libc::c_long;
    edt
        .num_functions = (Some(
        ((*(*abfd).xvec).bfd_getx32).expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )(data.offset(20 as libc::c_int as isize) as *const libc::c_void);
    edt
        .num_names = (Some(
        ((*(*abfd).xvec).bfd_getx32).expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )(data.offset(24 as libc::c_int as isize) as *const libc::c_void);
    edt
        .eat_addr = (Some(
        ((*(*abfd).xvec).bfd_getx32).expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )(data.offset(28 as libc::c_int as isize) as *const libc::c_void);
    edt
        .npt_addr = (Some(
        ((*(*abfd).xvec).bfd_getx32).expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )(data.offset(32 as libc::c_int as isize) as *const libc::c_void);
    edt
        .ot_addr = (Some(
        ((*(*abfd).xvec).bfd_getx32).expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )(data.offset(36 as libc::c_int as isize) as *const libc::c_void);
    adj = ((*section).vma).wrapping_sub((*extra).ImageBase).wrapping_add(dataoff);
    fprintf(
        file,
        dcgettext(
            b"bfd\0" as *const u8 as *const libc::c_char,
            b"\nThe Export Tables (interpreted %s section contents)\n\n\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
        (*section).name,
    );
    fprintf(
        file,
        dcgettext(
            b"bfd\0" as *const u8 as *const libc::c_char,
            b"Export Flags \t\t\t%lx\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        edt.export_flags as libc::c_ulong,
    );
    fprintf(
        file,
        dcgettext(
            b"bfd\0" as *const u8 as *const libc::c_char,
            b"Time/Date stamp \t\t%lx\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        edt.time_stamp as libc::c_ulong,
    );
    fprintf(
        file,
        dcgettext(
            b"bfd\0" as *const u8 as *const libc::c_char,
            b"Major/Minor \t\t\t%d/%d\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        edt.major_ver as libc::c_int,
        edt.minor_ver as libc::c_int,
    );
    fprintf(
        file,
        dcgettext(
            b"bfd\0" as *const u8 as *const libc::c_char,
            b"Name \t\t\t\t\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    bfd_fprintf_vma(abfd, file as *mut libc::c_void, edt.name);
    if edt.name >= adj && edt.name < adj.wrapping_add(datasize) {
        fprintf(
            file,
            b" %.*s\n\0" as *const u8 as *const libc::c_char,
            datasize.wrapping_sub((edt.name).wrapping_sub(adj)) as libc::c_int,
            data.offset(edt.name as isize).offset(-(adj as isize)),
        );
    } else {
        fprintf(
            file,
            b"(outside .edata section)\n\0" as *const u8 as *const libc::c_char,
        );
    }
    fprintf(
        file,
        dcgettext(
            b"bfd\0" as *const u8 as *const libc::c_char,
            b"Ordinal Base \t\t\t%ld\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        edt.base,
    );
    fprintf(
        file,
        dcgettext(
            b"bfd\0" as *const u8 as *const libc::c_char,
            b"Number in:\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    fprintf(
        file,
        dcgettext(
            b"bfd\0" as *const u8 as *const libc::c_char,
            b"\tExport Address Table \t\t%08lx\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        edt.num_functions,
    );
    fprintf(
        file,
        dcgettext(
            b"bfd\0" as *const u8 as *const libc::c_char,
            b"\t[Name Pointer/Ordinal] Table\t%08lx\n\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
        edt.num_names,
    );
    fprintf(
        file,
        dcgettext(
            b"bfd\0" as *const u8 as *const libc::c_char,
            b"Table Addresses\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    fprintf(
        file,
        dcgettext(
            b"bfd\0" as *const u8 as *const libc::c_char,
            b"\tExport Address Table \t\t\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    bfd_fprintf_vma(abfd, file as *mut libc::c_void, edt.eat_addr);
    fprintf(file, b"\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        file,
        dcgettext(
            b"bfd\0" as *const u8 as *const libc::c_char,
            b"\tName Pointer Table \t\t\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    bfd_fprintf_vma(abfd, file as *mut libc::c_void, edt.npt_addr);
    fprintf(file, b"\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        file,
        dcgettext(
            b"bfd\0" as *const u8 as *const libc::c_char,
            b"\tOrdinal Table \t\t\t\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    bfd_fprintf_vma(abfd, file as *mut libc::c_void, edt.ot_addr);
    fprintf(file, b"\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        file,
        dcgettext(
            b"bfd\0" as *const u8 as *const libc::c_char,
            b"\nExport Address Table -- Ordinal Base %ld\n\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
        edt.base,
    );
    if (edt.eat_addr).wrapping_sub(adj) >= datasize
        || (edt.num_functions)
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(4 as libc::c_int as libc::c_ulong) < edt.num_functions
        || (edt.eat_addr)
            .wrapping_sub(adj)
            .wrapping_add(
                (edt.num_functions)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(4 as libc::c_int as libc::c_ulong),
            ) > datasize
    {
        fprintf(
            file,
            dcgettext(
                b"bfd\0" as *const u8 as *const libc::c_char,
                b"\tInvalid Export Address Table rva (0x%lx) or entry count (0x%lx)\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            edt.eat_addr as libc::c_long,
            edt.num_functions as libc::c_long,
        );
    } else {
        i = 0 as libc::c_int as bfd_size_type;
        while i < edt.num_functions {
            let mut eat_member: bfd_vma = (Some(
                ((*(*abfd).xvec).bfd_getx32).expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(
                data
                    .offset(edt.eat_addr as isize)
                    .offset(i.wrapping_mul(4 as libc::c_int as libc::c_ulong) as isize)
                    .offset(-(adj as isize)) as *const libc::c_void,
            );
            if !(eat_member == 0 as libc::c_int as libc::c_ulong) {
                if eat_member.wrapping_sub(adj) <= datasize {
                    fprintf(
                        file,
                        b"\t[%4ld] +base[%4ld] %04lx %s -- %.*s\n\0" as *const u8
                            as *const libc::c_char,
                        i as libc::c_long,
                        i.wrapping_add(edt.base as libc::c_ulong) as libc::c_long,
                        eat_member,
                        dcgettext(
                            b"bfd\0" as *const u8 as *const libc::c_char,
                            b"Forwarder RVA\0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        datasize.wrapping_sub(eat_member.wrapping_sub(adj))
                            as libc::c_int,
                        data.offset(eat_member as isize).offset(-(adj as isize)),
                    );
                } else {
                    fprintf(
                        file,
                        b"\t[%4ld] +base[%4ld] %04lx %s\n\0" as *const u8
                            as *const libc::c_char,
                        i as libc::c_long,
                        i.wrapping_add(edt.base as libc::c_ulong) as libc::c_long,
                        eat_member,
                        dcgettext(
                            b"bfd\0" as *const u8 as *const libc::c_char,
                            b"Export RVA\0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                }
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    fprintf(
        file,
        dcgettext(
            b"bfd\0" as *const u8 as *const libc::c_char,
            b"\n[Ordinal/Name Pointer] Table\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    if (edt.npt_addr)
        .wrapping_add((edt.num_names).wrapping_mul(4 as libc::c_int as libc::c_ulong))
        .wrapping_sub(adj) >= datasize
        || (edt.num_names).wrapping_mul(4 as libc::c_int as libc::c_ulong)
            < edt.num_names
        || data.offset(edt.npt_addr as isize).offset(-(adj as isize)) < data
    {
        fprintf(
            file,
            dcgettext(
                b"bfd\0" as *const u8 as *const libc::c_char,
                b"\tInvalid Name Pointer Table rva (0x%lx) or entry count (0x%lx)\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            edt.npt_addr as libc::c_long,
            edt.num_names as libc::c_long,
        );
    } else if (edt.ot_addr)
        .wrapping_add((edt.num_names).wrapping_mul(2 as libc::c_int as libc::c_ulong))
        .wrapping_sub(adj) >= datasize
        || data.offset(edt.ot_addr as isize).offset(-(adj as isize)) < data
    {
        fprintf(
            file,
            dcgettext(
                b"bfd\0" as *const u8 as *const libc::c_char,
                b"\tInvalid Ordinal Table rva (0x%lx) or entry count (0x%lx)\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            edt.ot_addr as libc::c_long,
            edt.num_names as libc::c_long,
        );
    } else {
        i = 0 as libc::c_int as bfd_size_type;
        while i < edt.num_names {
            let mut name_ptr: bfd_vma = 0;
            let mut ord: bfd_vma = 0;
            ord = (Some(
                ((*(*abfd).xvec).bfd_getx16).expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(
                data
                    .offset(edt.ot_addr as isize)
                    .offset(i.wrapping_mul(2 as libc::c_int as libc::c_ulong) as isize)
                    .offset(-(adj as isize)) as *const libc::c_void,
            );
            name_ptr = (Some(
                ((*(*abfd).xvec).bfd_getx32).expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(
                data
                    .offset(edt.npt_addr as isize)
                    .offset(i.wrapping_mul(4 as libc::c_int as libc::c_ulong) as isize)
                    .offset(-(adj as isize)) as *const libc::c_void,
            );
            if name_ptr.wrapping_sub(adj) >= datasize {
                fprintf(
                    file,
                    dcgettext(
                        b"bfd\0" as *const u8 as *const libc::c_char,
                        b"\t[%4ld] <corrupt offset: %lx>\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    ord as libc::c_long,
                    name_ptr as libc::c_long,
                );
            } else {
                let mut name: *mut libc::c_char = (data as *mut libc::c_char)
                    .offset(name_ptr as isize)
                    .offset(-(adj as isize));
                fprintf(
                    file,
                    b"\t[%4ld] %.*s\n\0" as *const u8 as *const libc::c_char,
                    ord as libc::c_long,
                    (data.offset(datasize as isize) as *mut libc::c_char)
                        .offset_from(name) as libc::c_long as libc::c_int,
                    name,
                );
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    free(data as *mut libc::c_void);
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn pe_print_pdata(
    mut abfd: *mut bfd,
    mut vfile: *mut libc::c_void,
) -> bool {
    let mut file: *mut FILE = vfile as *mut FILE;
    let mut data: *mut bfd_byte = 0 as *mut bfd_byte;
    let mut section: *mut asection = bfd_get_section_by_name(
        abfd,
        b".pdata\0" as *const u8 as *const libc::c_char,
    );
    let mut datasize: bfd_size_type = 0 as libc::c_int as bfd_size_type;
    let mut i: bfd_size_type = 0;
    let mut start: bfd_size_type = 0;
    let mut stop: bfd_size_type = 0;
    let mut onaline: libc::c_int = 5 as libc::c_int * 4 as libc::c_int;
    if section.is_null() || ((*section).used_by_bfd as *mut coff_section_tdata).is_null()
        || ((*((*section).used_by_bfd as *mut coff_section_tdata)).tdata
            as *mut pei_section_tdata)
            .is_null()
    {
        return 1 as libc::c_int != 0;
    }
    stop = (*((*((*section).used_by_bfd as *mut coff_section_tdata)).tdata
        as *mut pei_section_tdata))
        .virt_size;
    if stop.wrapping_rem(onaline as libc::c_ulong) != 0 as libc::c_int as libc::c_ulong {
        fprintf(
            file,
            dcgettext(
                b"bfd\0" as *const u8 as *const libc::c_char,
                b"warning, .pdata section size (%ld) is not a multiple of %d\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stop as libc::c_long,
            onaline,
        );
    }
    fprintf(
        file,
        dcgettext(
            b"bfd\0" as *const u8 as *const libc::c_char,
            b"\nThe Function Table (interpreted .pdata section contents)\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    fprintf(
        file,
        dcgettext(
            b"bfd\0" as *const u8 as *const libc::c_char,
            b" vma:\t\tBegin    End      EH       EH       PrologEnd  Exception\n     \t\tAddress  Address  Handler  Data     Address    Mask\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    datasize = (*section).size;
    if datasize == 0 as libc::c_int as libc::c_ulong {
        return 1 as libc::c_int != 0;
    }
    if datasize < stop {
        fprintf(
            file,
            dcgettext(
                b"bfd\0" as *const u8 as *const libc::c_char,
                b"Virtual size of .pdata section (%ld) larger than real size (%ld)\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stop as libc::c_long,
            datasize as libc::c_long,
        );
        return 0 as libc::c_int != 0;
    }
    if !bfd_malloc_and_get_section(abfd, section, &mut data) {
        free(data as *mut libc::c_void);
        return 0 as libc::c_int != 0;
    }
    start = 0 as libc::c_int as bfd_size_type;
    i = start;
    while i < stop {
        let mut begin_addr: bfd_vma = 0;
        let mut end_addr: bfd_vma = 0;
        let mut eh_handler: bfd_vma = 0;
        let mut eh_data: bfd_vma = 0;
        let mut prolog_end_addr: bfd_vma = 0;
        let mut em_data: libc::c_int = 0;
        if i.wrapping_add((5 as libc::c_int * 4 as libc::c_int) as libc::c_ulong) > stop
        {
            break;
        }
        begin_addr = (Some(
            ((*(*abfd).xvec).bfd_getx32).expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(data.offset(i as isize) as *const libc::c_void);
        end_addr = (Some(
            ((*(*abfd).xvec).bfd_getx32).expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            data.offset(i as isize).offset(4 as libc::c_int as isize)
                as *const libc::c_void,
        );
        eh_handler = (Some(
            ((*(*abfd).xvec).bfd_getx32).expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            data.offset(i as isize).offset(8 as libc::c_int as isize)
                as *const libc::c_void,
        );
        eh_data = (Some(
            ((*(*abfd).xvec).bfd_getx32).expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            data.offset(i as isize).offset(12 as libc::c_int as isize)
                as *const libc::c_void,
        );
        prolog_end_addr = (Some(
            ((*(*abfd).xvec).bfd_getx32).expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            data.offset(i as isize).offset(16 as libc::c_int as isize)
                as *const libc::c_void,
        );
        if begin_addr == 0 as libc::c_int as libc::c_ulong
            && end_addr == 0 as libc::c_int as libc::c_ulong
            && eh_handler == 0 as libc::c_int as libc::c_ulong
            && eh_data == 0 as libc::c_int as libc::c_ulong
            && prolog_end_addr == 0 as libc::c_int as libc::c_ulong
        {
            break;
        }
        em_data = ((eh_handler & 0x1 as libc::c_int as libc::c_ulong) << 2 as libc::c_int
            | prolog_end_addr & 0x3 as libc::c_int as libc::c_ulong) as libc::c_int;
        eh_handler &= !(0x3 as libc::c_int as bfd_vma);
        prolog_end_addr &= !(0x3 as libc::c_int as bfd_vma);
        fputc(' ' as i32, file);
        bfd_fprintf_vma(abfd, file as *mut libc::c_void, i.wrapping_add((*section).vma));
        fputc('\t' as i32, file);
        bfd_fprintf_vma(abfd, file as *mut libc::c_void, begin_addr);
        fputc(' ' as i32, file);
        bfd_fprintf_vma(abfd, file as *mut libc::c_void, end_addr);
        fputc(' ' as i32, file);
        bfd_fprintf_vma(abfd, file as *mut libc::c_void, eh_handler);
        fputc(' ' as i32, file);
        bfd_fprintf_vma(abfd, file as *mut libc::c_void, eh_data);
        fputc(' ' as i32, file);
        bfd_fprintf_vma(abfd, file as *mut libc::c_void, prolog_end_addr);
        fprintf(file, b"   %x\0" as *const u8 as *const libc::c_char, em_data);
        fprintf(file, b"\n\0" as *const u8 as *const libc::c_char);
        i = (i as libc::c_ulong).wrapping_add(onaline as libc::c_ulong) as bfd_size_type
            as bfd_size_type;
    }
    free(data as *mut libc::c_void);
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn slurp_symtab(
    mut abfd: *mut bfd,
    mut psc: *mut sym_cache,
) -> *mut *mut asymbol {
    let mut sy: *mut *mut asymbol = 0 as *mut *mut asymbol;
    let mut storage: libc::c_long = 0;
    if bfd_get_file_flags(abfd) & 0x10 as libc::c_int as libc::c_uint == 0 {
        (*psc).symcount = 0 as libc::c_int;
        return 0 as *mut *mut asymbol;
    }
    storage = (Some(
        ((*(*abfd).xvec)._bfd_get_symtab_upper_bound).expect("non-null function pointer"),
    ))
        .expect("non-null function pointer")(abfd);
    if storage < 0 as libc::c_int as libc::c_long {
        return 0 as *mut *mut asymbol;
    }
    if storage != 0 {
        sy = bfd_malloc(storage as bfd_size_type) as *mut *mut asymbol;
        if sy.is_null() {
            return 0 as *mut *mut asymbol;
        }
    }
    (*psc)
        .symcount = (Some(
        ((*(*abfd).xvec)._bfd_canonicalize_symtab).expect("non-null function pointer"),
    ))
        .expect("non-null function pointer")(abfd, sy) as libc::c_int;
    if (*psc).symcount < 0 as libc::c_int {
        return 0 as *mut *mut asymbol;
    }
    return sy;
}
unsafe extern "C" fn my_symbol_for_address(
    mut abfd: *mut bfd,
    mut func: bfd_vma,
    mut psc: *mut sym_cache,
) -> *const libc::c_char {
    let mut i: libc::c_int = 0;
    if ((*psc).syms).is_null() {
        (*psc).syms = slurp_symtab(abfd, psc);
    }
    i = 0 as libc::c_int;
    while i < (*psc).symcount {
        if ((*(**((*psc).syms).offset(i as isize)).section).vma)
            .wrapping_add((**((*psc).syms).offset(i as isize)).value) == func
        {
            return (**((*psc).syms).offset(i as isize)).name;
        }
        i += 1;
        i;
    }
    return 0 as *const libc::c_char;
}
unsafe extern "C" fn cleanup_syms(mut psc: *mut sym_cache) {
    (*psc).symcount = 0 as libc::c_int;
    free((*psc).syms as *mut libc::c_void);
    (*psc).syms = 0 as *mut *mut asymbol;
}
#[no_mangle]
pub unsafe extern "C" fn _bfd_pex64_print_ce_compressed_pdata(
    mut abfd: *mut bfd,
    mut vfile: *mut libc::c_void,
) -> bool {
    let mut file: *mut FILE = vfile as *mut FILE;
    let mut data: *mut bfd_byte = 0 as *mut bfd_byte;
    let mut section: *mut asection = bfd_get_section_by_name(
        abfd,
        b".pdata\0" as *const u8 as *const libc::c_char,
    );
    let mut datasize: bfd_size_type = 0 as libc::c_int as bfd_size_type;
    let mut i: bfd_size_type = 0;
    let mut start: bfd_size_type = 0;
    let mut stop: bfd_size_type = 0;
    let mut onaline: libc::c_int = 2 as libc::c_int * 4 as libc::c_int;
    let mut cache: sym_cache = {
        let mut init = sym_cache {
            symcount: 0 as libc::c_int,
            syms: 0 as *mut *mut asymbol,
        };
        init
    };
    if section.is_null() || ((*section).used_by_bfd as *mut coff_section_tdata).is_null()
        || ((*((*section).used_by_bfd as *mut coff_section_tdata)).tdata
            as *mut pei_section_tdata)
            .is_null()
    {
        return 1 as libc::c_int != 0;
    }
    stop = (*((*((*section).used_by_bfd as *mut coff_section_tdata)).tdata
        as *mut pei_section_tdata))
        .virt_size;
    if stop.wrapping_rem(onaline as libc::c_ulong) != 0 as libc::c_int as libc::c_ulong {
        fprintf(
            file,
            dcgettext(
                b"bfd\0" as *const u8 as *const libc::c_char,
                b"warning, .pdata section size (%ld) is not a multiple of %d\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stop as libc::c_long,
            onaline,
        );
    }
    fprintf(
        file,
        dcgettext(
            b"bfd\0" as *const u8 as *const libc::c_char,
            b"\nThe Function Table (interpreted .pdata section contents)\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    fprintf(
        file,
        dcgettext(
            b"bfd\0" as *const u8 as *const libc::c_char,
            b" vma:\t\tBegin    Prolog   Function Flags    Exception EH\n     \t\tAddress  Length   Length   32b exc  Handler   Data\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    datasize = (*section).size;
    if datasize == 0 as libc::c_int as libc::c_ulong {
        return 1 as libc::c_int != 0;
    }
    if !bfd_malloc_and_get_section(abfd, section, &mut data) {
        free(data as *mut libc::c_void);
        return 0 as libc::c_int != 0;
    }
    start = 0 as libc::c_int as bfd_size_type;
    i = start;
    while i < stop {
        let mut begin_addr: bfd_vma = 0;
        let mut other_data: bfd_vma = 0;
        let mut prolog_length: bfd_vma = 0;
        let mut function_length: bfd_vma = 0;
        let mut flag32bit: libc::c_int = 0;
        let mut exception_flag: libc::c_int = 0;
        let mut tsection: *mut asection = 0 as *mut asection;
        if i.wrapping_add((2 as libc::c_int * 4 as libc::c_int) as libc::c_ulong) > stop
        {
            break;
        }
        begin_addr = (Some(
            ((*(*abfd).xvec).bfd_getx32).expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(data.offset(i as isize) as *const libc::c_void);
        other_data = (Some(
            ((*(*abfd).xvec).bfd_getx32).expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            data.offset(i as isize).offset(4 as libc::c_int as isize)
                as *const libc::c_void,
        );
        if begin_addr == 0 as libc::c_int as libc::c_ulong
            && other_data == 0 as libc::c_int as libc::c_ulong
        {
            break;
        }
        prolog_length = other_data & 0xff as libc::c_int as libc::c_ulong;
        function_length = (other_data & 0x3fffff00 as libc::c_int as libc::c_ulong)
            >> 8 as libc::c_int;
        flag32bit = ((other_data & 0x40000000 as libc::c_int as libc::c_ulong)
            >> 30 as libc::c_int) as libc::c_int;
        exception_flag = ((other_data & 0x80000000 as libc::c_uint as libc::c_ulong)
            >> 31 as libc::c_int) as libc::c_int;
        fputc(' ' as i32, file);
        bfd_fprintf_vma(abfd, file as *mut libc::c_void, i.wrapping_add((*section).vma));
        fputc('\t' as i32, file);
        bfd_fprintf_vma(abfd, file as *mut libc::c_void, begin_addr);
        fputc(' ' as i32, file);
        bfd_fprintf_vma(abfd, file as *mut libc::c_void, prolog_length);
        fputc(' ' as i32, file);
        bfd_fprintf_vma(abfd, file as *mut libc::c_void, function_length);
        fputc(' ' as i32, file);
        fprintf(
            file,
            b"%2d  %2d   \0" as *const u8 as *const libc::c_char,
            flag32bit,
            exception_flag,
        );
        tsection = bfd_get_section_by_name(
            abfd,
            b".text\0" as *const u8 as *const libc::c_char,
        );
        if !tsection.is_null()
            && !((*tsection).used_by_bfd as *mut coff_section_tdata).is_null()
            && !((*((*tsection).used_by_bfd as *mut coff_section_tdata)).tdata
                as *mut pei_section_tdata)
                .is_null()
        {
            let mut eh_off: bfd_vma = begin_addr
                .wrapping_sub(8 as libc::c_int as libc::c_ulong)
                .wrapping_sub((*tsection).vma);
            let mut tdata: *mut bfd_byte = 0 as *mut bfd_byte;
            tdata = bfd_malloc(8 as libc::c_int as bfd_size_type) as *mut bfd_byte;
            if !tdata.is_null() {
                if bfd_get_section_contents(
                    abfd,
                    tsection,
                    tdata as *mut libc::c_void,
                    eh_off as file_ptr,
                    8 as libc::c_int as bfd_size_type,
                ) {
                    let mut eh: bfd_vma = 0;
                    let mut eh_data: bfd_vma = 0;
                    eh = (Some(
                        ((*(*abfd).xvec).bfd_getx32).expect("non-null function pointer"),
                    ))
                        .expect(
                            "non-null function pointer",
                        )(tdata as *const libc::c_void);
                    eh_data = (Some(
                        ((*(*abfd).xvec).bfd_getx32).expect("non-null function pointer"),
                    ))
                        .expect(
                            "non-null function pointer",
                        )(
                        tdata.offset(4 as libc::c_int as isize) as *const libc::c_void,
                    );
                    fprintf(
                        file,
                        b"%08x  \0" as *const u8 as *const libc::c_char,
                        eh as libc::c_uint,
                    );
                    fprintf(
                        file,
                        b"%08x\0" as *const u8 as *const libc::c_char,
                        eh_data as libc::c_uint,
                    );
                    if eh != 0 as libc::c_int as libc::c_ulong {
                        let mut s: *const libc::c_char = my_symbol_for_address(
                            abfd,
                            eh,
                            &mut cache,
                        );
                        if !s.is_null() {
                            fprintf(
                                file,
                                b" (%s) \0" as *const u8 as *const libc::c_char,
                                s,
                            );
                        }
                    }
                }
                free(tdata as *mut libc::c_void);
            }
        }
        fprintf(file, b"\n\0" as *const u8 as *const libc::c_char);
        i = (i as libc::c_ulong).wrapping_add(onaline as libc::c_ulong) as bfd_size_type
            as bfd_size_type;
    }
    free(data as *mut libc::c_void);
    cleanup_syms(&mut cache);
    return 1 as libc::c_int != 0;
}
static mut tbl: [*const libc::c_char; 13] = [
    b"ABSOLUTE\0" as *const u8 as *const libc::c_char,
    b"HIGH\0" as *const u8 as *const libc::c_char,
    b"LOW\0" as *const u8 as *const libc::c_char,
    b"HIGHLOW\0" as *const u8 as *const libc::c_char,
    b"HIGHADJ\0" as *const u8 as *const libc::c_char,
    b"MIPS_JMPADDR\0" as *const u8 as *const libc::c_char,
    b"SECTION\0" as *const u8 as *const libc::c_char,
    b"REL32\0" as *const u8 as *const libc::c_char,
    b"RESERVED1\0" as *const u8 as *const libc::c_char,
    b"MIPS_JMPADDR16\0" as *const u8 as *const libc::c_char,
    b"DIR64\0" as *const u8 as *const libc::c_char,
    b"HIGH3ADJ\0" as *const u8 as *const libc::c_char,
    b"UNKNOWN\0" as *const u8 as *const libc::c_char,
];
unsafe extern "C" fn pe_print_reloc(
    mut abfd: *mut bfd,
    mut vfile: *mut libc::c_void,
) -> bool {
    let mut file: *mut FILE = vfile as *mut FILE;
    let mut data: *mut bfd_byte = 0 as *mut bfd_byte;
    let mut section: *mut asection = bfd_get_section_by_name(
        abfd,
        b".reloc\0" as *const u8 as *const libc::c_char,
    );
    let mut p: *mut bfd_byte = 0 as *mut bfd_byte;
    let mut end: *mut bfd_byte = 0 as *mut bfd_byte;
    if section.is_null() || (*section).size == 0 as libc::c_int as libc::c_ulong
        || (*section).flags & 0x100 as libc::c_int as libc::c_uint == 0
    {
        return 1 as libc::c_int != 0;
    }
    fprintf(
        file,
        dcgettext(
            b"bfd\0" as *const u8 as *const libc::c_char,
            b"\n\nPE File Base Relocations (interpreted .reloc section contents)\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    if !bfd_malloc_and_get_section(abfd, section, &mut data) {
        free(data as *mut libc::c_void);
        return 0 as libc::c_int != 0;
    }
    p = data;
    end = data.offset((*section).size as isize);
    while p.offset(8 as libc::c_int as isize) <= end {
        let mut j: libc::c_int = 0;
        let mut virtual_address: bfd_vma = 0;
        let mut number: libc::c_ulong = 0;
        let mut size: libc::c_ulong = 0;
        let mut chunk_end: *mut bfd_byte = 0 as *mut bfd_byte;
        virtual_address = (Some(
            ((*(*abfd).xvec).bfd_getx32).expect("non-null function pointer"),
        ))
            .expect("non-null function pointer")(p as *const libc::c_void);
        size = (Some(((*(*abfd).xvec).bfd_getx32).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(p.offset(4 as libc::c_int as isize) as *const libc::c_void);
        p = p.offset(8 as libc::c_int as isize);
        number = size
            .wrapping_sub(8 as libc::c_int as libc::c_ulong)
            .wrapping_div(2 as libc::c_int as libc::c_ulong);
        if size == 0 as libc::c_int as libc::c_ulong {
            break;
        }
        fprintf(
            file,
            dcgettext(
                b"bfd\0" as *const u8 as *const libc::c_char,
                b"\nVirtual Address: %08lx Chunk size %ld (0x%lx) Number of fixups %ld\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            virtual_address,
            size,
            size,
            number,
        );
        chunk_end = p.offset(-(8 as libc::c_int as isize)).offset(size as isize);
        if chunk_end > end {
            chunk_end = end;
        }
        j = 0 as libc::c_int;
        while p.offset(2 as libc::c_int as isize) <= chunk_end {
            let mut e: libc::c_ushort = (Some(
                ((*(*abfd).xvec).bfd_getx16).expect("non-null function pointer"),
            ))
                .expect("non-null function pointer")(p as *const libc::c_void)
                as libc::c_ushort;
            let mut t: libc::c_uint = ((e as libc::c_int & 0xf000 as libc::c_int)
                >> 12 as libc::c_int) as libc::c_uint;
            let mut off: libc::c_int = e as libc::c_int & 0xfff as libc::c_int;
            if t as libc::c_ulong
                >= (::core::mem::size_of::<[*const libc::c_char; 13]>() as libc::c_ulong)
                    .wrapping_div(
                        ::core::mem::size_of::<*const libc::c_char>() as libc::c_ulong,
                    )
            {
                t = (::core::mem::size_of::<[*const libc::c_char; 13]>()
                    as libc::c_ulong)
                    .wrapping_div(
                        ::core::mem::size_of::<*const libc::c_char>() as libc::c_ulong,
                    )
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_uint;
            }
            fprintf(
                file,
                dcgettext(
                    b"bfd\0" as *const u8 as *const libc::c_char,
                    b"\treloc %4d offset %4x [%4lx] %s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                j,
                off,
                (off as libc::c_ulong).wrapping_add(virtual_address),
                tbl[t as usize],
            );
            p = p.offset(2 as libc::c_int as isize);
            j += 1;
            j;
            if t == 4 as libc::c_int as libc::c_uint
                && p.offset(2 as libc::c_int as isize) <= chunk_end
            {
                fprintf(
                    file,
                    b" (%4x)\0" as *const u8 as *const libc::c_char,
                    (Some(
                        ((*(*abfd).xvec).bfd_getx16).expect("non-null function pointer"),
                    ))
                        .expect("non-null function pointer")(p as *const libc::c_void)
                        as libc::c_uint,
                );
                p = p.offset(2 as libc::c_int as isize);
                j += 1;
                j;
            }
            fprintf(file, b"\n\0" as *const u8 as *const libc::c_char);
        }
    }
    free(data as *mut libc::c_void);
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn rsrc_print_resource_entries(
    mut file: *mut FILE,
    mut abfd: *mut bfd,
    mut indent: libc::c_uint,
    mut is_name: bool,
    mut data: *mut bfd_byte,
    mut regions: *mut rsrc_regions,
    mut rva_bias: bfd_vma,
) -> *mut bfd_byte {
    let mut entry: libc::c_ulong = 0;
    let mut addr: libc::c_ulong = 0;
    let mut size: libc::c_ulong = 0;
    let mut leaf: *mut bfd_byte = 0 as *mut bfd_byte;
    if data.offset(8 as libc::c_int as isize) >= (*regions).section_end {
        return ((*regions).section_end).offset(1 as libc::c_int as isize);
    }
    fprintf(
        file,
        dcgettext(
            b"bfd\0" as *const u8 as *const libc::c_char,
            b"%03x %*.s Entry: \0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        data.offset_from((*regions).section_start) as libc::c_long as libc::c_int,
        indent,
        b" \0" as *const u8 as *const libc::c_char,
    );
    entry = (Some(((*(*abfd).xvec).bfd_getx32).expect("non-null function pointer")))
        .expect("non-null function pointer")(data as *const libc::c_void);
    if is_name {
        let mut name: *mut bfd_byte = 0 as *mut bfd_byte;
        if entry & 0x80000000 as libc::c_uint as libc::c_ulong != 0 {
            name = ((*regions).section_start)
                .offset((entry & 0x7fffffff as libc::c_int as libc::c_ulong) as isize);
        } else {
            name = ((*regions).section_start)
                .offset(entry as isize)
                .offset(-(rva_bias as isize));
        }
        if name.offset(2 as libc::c_int as isize) < (*regions).section_end
            && name > (*regions).section_start
        {
            let mut len: libc::c_uint = 0;
            if ((*regions).strings_start).is_null() {
                (*regions).strings_start = name;
            }
            len = (Some(
                ((*(*abfd).xvec).bfd_getx16).expect("non-null function pointer"),
            ))
                .expect("non-null function pointer")(name as *const libc::c_void)
                as libc::c_uint;
            fprintf(
                file,
                dcgettext(
                    b"bfd\0" as *const u8 as *const libc::c_char,
                    b"name: [val: %08lx len %d]: \0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                entry,
                len,
            );
            if name
                .offset(2 as libc::c_int as isize)
                .offset(len.wrapping_mul(2 as libc::c_int as libc::c_uint) as isize)
                < (*regions).section_end
            {
                loop {
                    let fresh0 = len;
                    len = len.wrapping_sub(1);
                    if !(fresh0 != 0) {
                        break;
                    }
                    let mut c: libc::c_char = 0;
                    name = name.offset(2 as libc::c_int as isize);
                    c = *name as libc::c_char;
                    if c as libc::c_int > 0 as libc::c_int
                        && (c as libc::c_int) < 32 as libc::c_int
                    {
                        fprintf(
                            file,
                            b"^%c\0" as *const u8 as *const libc::c_char,
                            c as libc::c_int + 64 as libc::c_int,
                        );
                    } else {
                        fprintf(
                            file,
                            b"%.1s\0" as *const u8 as *const libc::c_char,
                            name,
                        );
                    }
                }
            } else {
                fprintf(
                    file,
                    dcgettext(
                        b"bfd\0" as *const u8 as *const libc::c_char,
                        b"<corrupt string length: %#x>\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    len,
                );
                return ((*regions).section_end).offset(1 as libc::c_int as isize);
            }
        } else {
            fprintf(
                file,
                dcgettext(
                    b"bfd\0" as *const u8 as *const libc::c_char,
                    b"<corrupt string offset: %#lx>\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                entry,
            );
            return ((*regions).section_end).offset(1 as libc::c_int as isize);
        }
    } else {
        fprintf(
            file,
            dcgettext(
                b"bfd\0" as *const u8 as *const libc::c_char,
                b"ID: %#08lx\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            entry,
        );
    }
    entry = (Some(((*(*abfd).xvec).bfd_getx32).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(data.offset(4 as libc::c_int as isize) as *const libc::c_void) as libc::c_long
        as libc::c_ulong;
    fprintf(
        file,
        dcgettext(
            b"bfd\0" as *const u8 as *const libc::c_char,
            b", Value: %#08lx\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        entry,
    );
    if entry & 0x80000000 as libc::c_uint as libc::c_ulong != 0 {
        data = ((*regions).section_start)
            .offset((entry & 0x7fffffff as libc::c_int as libc::c_ulong) as isize);
        if data <= (*regions).section_start || data > (*regions).section_end {
            return ((*regions).section_end).offset(1 as libc::c_int as isize);
        }
        return rsrc_print_resource_directory(
            file,
            abfd,
            indent.wrapping_add(1 as libc::c_int as libc::c_uint),
            data,
            regions,
            rva_bias,
        );
    }
    leaf = ((*regions).section_start).offset(entry as isize);
    if leaf.offset(16 as libc::c_int as isize) >= (*regions).section_end
        || leaf < (*regions).section_start
    {
        return ((*regions).section_end).offset(1 as libc::c_int as isize);
    }
    addr = (Some(((*(*abfd).xvec).bfd_getx32).expect("non-null function pointer")))
        .expect("non-null function pointer")(leaf as *const libc::c_void) as libc::c_long
        as libc::c_ulong;
    size = (Some(((*(*abfd).xvec).bfd_getx32).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(leaf.offset(4 as libc::c_int as isize) as *const libc::c_void) as libc::c_long
        as libc::c_ulong;
    fprintf(
        file,
        dcgettext(
            b"bfd\0" as *const u8 as *const libc::c_char,
            b"%03x %*.s  Leaf: Addr: %#08lx, Size: %#08lx, Codepage: %d\n\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
        entry as libc::c_int,
        indent,
        b" \0" as *const u8 as *const libc::c_char,
        addr,
        size,
        (Some(((*(*abfd).xvec).bfd_getx32).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(leaf.offset(8 as libc::c_int as isize) as *const libc::c_void)
            as libc::c_int,
    );
    if (Some(((*(*abfd).xvec).bfd_getx32).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(leaf.offset(12 as libc::c_int as isize) as *const libc::c_void)
        != 0 as libc::c_int as libc::c_ulong
        || ((*regions).section_start)
            .offset(addr.wrapping_sub(rva_bias) as isize)
            .offset(size as isize) > (*regions).section_end
    {
        return ((*regions).section_end).offset(1 as libc::c_int as isize);
    }
    if ((*regions).resource_start).is_null() {
        (*regions)
            .resource_start = ((*regions).section_start)
            .offset(addr.wrapping_sub(rva_bias) as isize);
    }
    return ((*regions).section_start)
        .offset(addr.wrapping_sub(rva_bias) as isize)
        .offset(size as isize);
}
unsafe extern "C" fn rsrc_print_resource_directory(
    mut file: *mut FILE,
    mut abfd: *mut bfd,
    mut indent: libc::c_uint,
    mut data: *mut bfd_byte,
    mut regions: *mut rsrc_regions,
    mut rva_bias: bfd_vma,
) -> *mut bfd_byte {
    let mut num_names: libc::c_uint = 0;
    let mut num_ids: libc::c_uint = 0;
    let mut highest_data: *mut bfd_byte = data;
    if data.offset(16 as libc::c_int as isize) >= (*regions).section_end {
        return ((*regions).section_end).offset(1 as libc::c_int as isize);
    }
    fprintf(
        file,
        b"%03x %*.s \0" as *const u8 as *const libc::c_char,
        data.offset_from((*regions).section_start) as libc::c_long as libc::c_int,
        indent,
        b" \0" as *const u8 as *const libc::c_char,
    );
    match indent {
        0 => {
            fprintf(file, b"Type\0" as *const u8 as *const libc::c_char);
        }
        2 => {
            fprintf(file, b"Name\0" as *const u8 as *const libc::c_char);
        }
        4 => {
            fprintf(file, b"Language\0" as *const u8 as *const libc::c_char);
        }
        _ => {
            fprintf(
                file,
                dcgettext(
                    b"bfd\0" as *const u8 as *const libc::c_char,
                    b"<unknown directory type: %d>\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                indent,
            );
            return ((*regions).section_end).offset(1 as libc::c_int as isize);
        }
    }
    num_names = (Some(((*(*abfd).xvec).bfd_getx16).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(data.offset(12 as libc::c_int as isize) as *const libc::c_void) as libc::c_int
        as libc::c_uint;
    num_ids = (Some(((*(*abfd).xvec).bfd_getx16).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(data.offset(14 as libc::c_int as isize) as *const libc::c_void) as libc::c_int
        as libc::c_uint;
    fprintf(
        file,
        dcgettext(
            b"bfd\0" as *const u8 as *const libc::c_char,
            b" Table: Char: %d, Time: %08lx, Ver: %d/%d, Num Names: %d, IDs: %d\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        (Some(((*(*abfd).xvec).bfd_getx32).expect("non-null function pointer")))
            .expect("non-null function pointer")(data as *const libc::c_void)
            as libc::c_int,
        (Some(((*(*abfd).xvec).bfd_getx32).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(data.offset(4 as libc::c_int as isize) as *const libc::c_void)
            as libc::c_long,
        (Some(((*(*abfd).xvec).bfd_getx16).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(data.offset(8 as libc::c_int as isize) as *const libc::c_void)
            as libc::c_int,
        (Some(((*(*abfd).xvec).bfd_getx16).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(data.offset(10 as libc::c_int as isize) as *const libc::c_void)
            as libc::c_int,
        num_names,
        num_ids,
    );
    data = data.offset(16 as libc::c_int as isize);
    loop {
        let fresh1 = num_names;
        num_names = num_names.wrapping_sub(1);
        if !(fresh1 != 0) {
            break;
        }
        let mut entry_end: *mut bfd_byte = 0 as *mut bfd_byte;
        entry_end = rsrc_print_resource_entries(
            file,
            abfd,
            indent.wrapping_add(1 as libc::c_int as libc::c_uint),
            1 as libc::c_int != 0,
            data,
            regions,
            rva_bias,
        );
        data = data.offset(8 as libc::c_int as isize);
        highest_data = if highest_data > entry_end { highest_data } else { entry_end };
        if entry_end >= (*regions).section_end {
            return entry_end;
        }
    }
    loop {
        let fresh2 = num_ids;
        num_ids = num_ids.wrapping_sub(1);
        if !(fresh2 != 0) {
            break;
        }
        let mut entry_end_0: *mut bfd_byte = 0 as *mut bfd_byte;
        entry_end_0 = rsrc_print_resource_entries(
            file,
            abfd,
            indent.wrapping_add(1 as libc::c_int as libc::c_uint),
            0 as libc::c_int != 0,
            data,
            regions,
            rva_bias,
        );
        data = data.offset(8 as libc::c_int as isize);
        highest_data = if highest_data > entry_end_0 {
            highest_data
        } else {
            entry_end_0
        };
        if entry_end_0 >= (*regions).section_end {
            return entry_end_0;
        }
    }
    return if highest_data > data { highest_data } else { data };
}
unsafe extern "C" fn rsrc_print_section(
    mut abfd: *mut bfd,
    mut vfile: *mut libc::c_void,
) -> bool {
    let mut rva_bias: bfd_vma = 0;
    let mut pe: *mut pe_data_type = 0 as *mut pe_data_type;
    let mut file: *mut FILE = vfile as *mut FILE;
    let mut datasize: bfd_size_type = 0;
    let mut section: *mut asection = 0 as *mut asection;
    let mut data: *mut bfd_byte = 0 as *mut bfd_byte;
    let mut regions: rsrc_regions = rsrc_regions {
        section_start: 0 as *mut bfd_byte,
        section_end: 0 as *mut bfd_byte,
        strings_start: 0 as *mut bfd_byte,
        resource_start: 0 as *mut bfd_byte,
    };
    pe = (*abfd).tdata.pe_obj_data;
    if pe.is_null() {
        return 1 as libc::c_int != 0;
    }
    section = bfd_get_section_by_name(
        abfd,
        b".rsrc\0" as *const u8 as *const libc::c_char,
    );
    if section.is_null() {
        return 1 as libc::c_int != 0;
    }
    if (*section).flags & 0x100 as libc::c_int as libc::c_uint == 0 {
        return 1 as libc::c_int != 0;
    }
    datasize = (*section).size;
    if datasize == 0 as libc::c_int as libc::c_ulong {
        return 1 as libc::c_int != 0;
    }
    rva_bias = ((*section).vma).wrapping_sub((*pe).pe_opthdr.ImageBase);
    if !bfd_malloc_and_get_section(abfd, section, &mut data) {
        free(data as *mut libc::c_void);
        return 0 as libc::c_int != 0;
    }
    regions.section_start = data;
    regions.section_end = data.offset(datasize as isize);
    regions.strings_start = 0 as *mut bfd_byte;
    regions.resource_start = 0 as *mut bfd_byte;
    fflush(file);
    fprintf(
        file,
        b"\nThe .rsrc Resource Directory section:\n\0" as *const u8
            as *const libc::c_char,
    );
    while data < regions.section_end {
        let mut p: *mut bfd_byte = data;
        data = rsrc_print_resource_directory(
            file,
            abfd,
            0 as libc::c_int as libc::c_uint,
            data,
            &mut regions,
            rva_bias,
        );
        if data == (regions.section_end).offset(1 as libc::c_int as isize) {
            fprintf(
                file,
                dcgettext(
                    b"bfd\0" as *const u8 as *const libc::c_char,
                    b"Corrupt .rsrc section detected!\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        } else {
            let mut align: libc::c_int = ((1 as libc::c_int)
                << (*section).alignment_power) - 1 as libc::c_int;
            data = (data.offset(align as isize) as ptrdiff_t & !align as libc::c_long)
                as *mut bfd_byte;
            rva_bias = (rva_bias as libc::c_ulong)
                .wrapping_add(data.offset_from(p) as libc::c_long as libc::c_ulong)
                as bfd_vma as bfd_vma;
            if data == (regions.section_end).offset(-(4 as libc::c_int as isize)) {
                data = regions.section_end;
            } else if data < regions.section_end {
                loop {
                    data = data.offset(1);
                    if !(data < regions.section_end) {
                        break;
                    }
                    if *data as libc::c_int != 0 as libc::c_int {
                        break;
                    }
                }
                if data < regions.section_end {
                    fprintf(
                        file,
                        dcgettext(
                            b"bfd\0" as *const u8 as *const libc::c_char,
                            b"\nWARNING: Extra data in .rsrc section - it will be ignored by Windows:\n\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                }
            }
        }
    }
    if !(regions.strings_start).is_null() {
        fprintf(
            file,
            dcgettext(
                b"bfd\0" as *const u8 as *const libc::c_char,
                b" String table starts at offset: %#03x\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            (regions.strings_start).offset_from(regions.section_start) as libc::c_long
                as libc::c_int,
        );
    }
    if !(regions.resource_start).is_null() {
        fprintf(
            file,
            dcgettext(
                b"bfd\0" as *const u8 as *const libc::c_char,
                b" Resources start at offset: %#03x\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            (regions.resource_start).offset_from(regions.section_start) as libc::c_long
                as libc::c_int,
        );
    }
    free(regions.section_start as *mut libc::c_void);
    return 1 as libc::c_int != 0;
}
static mut debug_type_names: [*mut libc::c_char; 17] = [
    b"Unknown\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"COFF\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"CodeView\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"FPO\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Misc\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Exception\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Fixup\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"OMAP-to-SRC\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"OMAP-from-SRC\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Borland\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Reserved\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"CLSID\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Feature\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"CoffGrp\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"ILTCG\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"MPX\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Repro\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
];
unsafe extern "C" fn pe_print_debugdata(
    mut abfd: *mut bfd,
    mut vfile: *mut libc::c_void,
) -> bool {
    let mut file: *mut FILE = vfile as *mut FILE;
    let mut pe: *mut pe_data_type = (*abfd).tdata.pe_obj_data;
    let mut extra: *mut internal_extra_pe_aouthdr = &mut (*pe).pe_opthdr;
    let mut section: *mut asection = 0 as *mut asection;
    let mut data: *mut bfd_byte = 0 as *mut bfd_byte;
    let mut dataoff: bfd_size_type = 0;
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_uint = 0;
    let mut addr: bfd_vma = (*extra)
        .DataDirectory[6 as libc::c_int as usize]
        .VirtualAddress;
    let mut size: bfd_size_type = (*extra).DataDirectory[6 as libc::c_int as usize].Size
        as bfd_size_type;
    if size == 0 as libc::c_int as libc::c_ulong {
        return 1 as libc::c_int != 0;
    }
    addr = (addr as libc::c_ulong).wrapping_add((*extra).ImageBase) as bfd_vma
        as bfd_vma;
    section = (*abfd).sections;
    while !section.is_null() {
        if addr >= (*section).vma
            && addr < ((*section).vma).wrapping_add((*section).size)
        {
            break;
        }
        section = (*section).next;
    }
    if section.is_null() {
        fprintf(
            file,
            dcgettext(
                b"bfd\0" as *const u8 as *const libc::c_char,
                b"\nThere is a debug directory, but the section containing it could not be found\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return 1 as libc::c_int != 0;
    } else if (*section).flags & 0x100 as libc::c_int as libc::c_uint == 0 {
        fprintf(
            file,
            dcgettext(
                b"bfd\0" as *const u8 as *const libc::c_char,
                b"\nThere is a debug directory in %s, but that section has no contents\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            (*section).name,
        );
        return 1 as libc::c_int != 0;
    } else if (*section).size < size {
        fprintf(
            file,
            dcgettext(
                b"bfd\0" as *const u8 as *const libc::c_char,
                b"\nError: section %s contains the debug data starting address but it is too small\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            (*section).name,
        );
        return 0 as libc::c_int != 0;
    }
    fprintf(
        file,
        dcgettext(
            b"bfd\0" as *const u8 as *const libc::c_char,
            b"\nThere is a debug directory in %s at 0x%lx\n\n\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
        (*section).name,
        addr,
    );
    dataoff = addr.wrapping_sub((*section).vma);
    if size > ((*section).size).wrapping_sub(dataoff) {
        fprintf(
            file,
            dcgettext(
                b"bfd\0" as *const u8 as *const libc::c_char,
                b"The debug data size field in the data directory is too big for the section\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return 0 as libc::c_int != 0;
    }
    fprintf(
        file,
        dcgettext(
            b"bfd\0" as *const u8 as *const libc::c_char,
            b"Type                Size     Rva      Offset\n\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    if !bfd_malloc_and_get_section(abfd, section, &mut data) {
        free(data as *mut libc::c_void);
        return 0 as libc::c_int != 0;
    }
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong)
        < size
            .wrapping_div(
                ::core::mem::size_of::<external_IMAGE_DEBUG_DIRECTORY>() as libc::c_ulong,
            )
    {
        let mut type_name: *const libc::c_char = 0 as *const libc::c_char;
        let mut ext: *mut external_IMAGE_DEBUG_DIRECTORY = &mut *(data
            .offset(dataoff as isize) as *mut external_IMAGE_DEBUG_DIRECTORY)
            .offset(i as isize) as *mut external_IMAGE_DEBUG_DIRECTORY;
        let mut idd: internal_IMAGE_DEBUG_DIRECTORY = internal_IMAGE_DEBUG_DIRECTORY {
            Characteristics: 0,
            TimeDateStamp: 0,
            MajorVersion: 0,
            MinorVersion: 0,
            Type: 0,
            SizeOfData: 0,
            AddressOfRawData: 0,
            PointerToRawData: 0,
        };
        _bfd_pex64i_swap_debugdir_in(
            abfd,
            ext as *mut libc::c_void,
            &mut idd as *mut internal_IMAGE_DEBUG_DIRECTORY as *mut libc::c_void,
        );
        if idd.Type >= 17 as libc::c_int as libc::c_ulong {
            type_name = debug_type_names[0 as libc::c_int as usize];
        } else {
            type_name = debug_type_names[idd.Type as usize];
        }
        fprintf(
            file,
            b" %2ld  %14s %08lx %08lx %08lx\n\0" as *const u8 as *const libc::c_char,
            idd.Type,
            type_name,
            idd.SizeOfData,
            idd.AddressOfRawData,
            idd.PointerToRawData,
        );
        if idd.Type == 2 as libc::c_int as libc::c_ulong {
            let mut signature: [libc::c_char; 33] = [0; 33];
            let mut buffer: [libc::c_char; 257] = [0; 257];
            let mut cvinfo: *mut CODEVIEW_INFO = buffer.as_mut_ptr()
                as *mut CODEVIEW_INFO;
            if !(_bfd_pex64i_slurp_codeview_record(
                abfd,
                idd.PointerToRawData as file_ptr,
                idd.SizeOfData,
                cvinfo,
            ))
                .is_null()
            {
                j = 0 as libc::c_int as libc::c_uint;
                while j < (*cvinfo).SignatureLength {
                    sprintf(
                        &mut *signature
                            .as_mut_ptr()
                            .offset(
                                j.wrapping_mul(2 as libc::c_int as libc::c_uint) as isize,
                            ) as *mut libc::c_char,
                        b"%02x\0" as *const u8 as *const libc::c_char,
                        (*cvinfo).Signature[j as usize] as libc::c_int
                            & 0xff as libc::c_int,
                    );
                    j = j.wrapping_add(1);
                    j;
                }
                fprintf(
                    file,
                    dcgettext(
                        b"bfd\0" as *const u8 as *const libc::c_char,
                        b"(format %c%c%c%c signature %s age %ld)\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    buffer[0 as libc::c_int as usize] as libc::c_int,
                    buffer[1 as libc::c_int as usize] as libc::c_int,
                    buffer[2 as libc::c_int as usize] as libc::c_int,
                    buffer[3 as libc::c_int as usize] as libc::c_int,
                    signature.as_mut_ptr(),
                    (*cvinfo).Age,
                );
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    free(data as *mut libc::c_void);
    if size
        .wrapping_rem(
            ::core::mem::size_of::<external_IMAGE_DEBUG_DIRECTORY>() as libc::c_ulong,
        ) != 0 as libc::c_int as libc::c_ulong
    {
        fprintf(
            file,
            dcgettext(
                b"bfd\0" as *const u8 as *const libc::c_char,
                b"The debug directory size is not a multiple of the debug directory entry size\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn pe_is_repro(mut abfd: *mut bfd) -> bool {
    let mut pe: *mut pe_data_type = (*abfd).tdata.pe_obj_data;
    let mut extra: *mut internal_extra_pe_aouthdr = &mut (*pe).pe_opthdr;
    let mut section: *mut asection = 0 as *mut asection;
    let mut data: *mut bfd_byte = 0 as *mut bfd_byte;
    let mut dataoff: bfd_size_type = 0;
    let mut i: libc::c_uint = 0;
    let mut res: bool = 0 as libc::c_int != 0;
    let mut addr: bfd_vma = (*extra)
        .DataDirectory[6 as libc::c_int as usize]
        .VirtualAddress;
    let mut size: bfd_size_type = (*extra).DataDirectory[6 as libc::c_int as usize].Size
        as bfd_size_type;
    if size == 0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int != 0;
    }
    addr = (addr as libc::c_ulong).wrapping_add((*extra).ImageBase) as bfd_vma
        as bfd_vma;
    section = (*abfd).sections;
    while !section.is_null() {
        if addr >= (*section).vma
            && addr < ((*section).vma).wrapping_add((*section).size)
        {
            break;
        }
        section = (*section).next;
    }
    if section.is_null() || (*section).flags & 0x100 as libc::c_int as libc::c_uint == 0
        || (*section).size < size
    {
        return 0 as libc::c_int != 0;
    }
    dataoff = addr.wrapping_sub((*section).vma);
    if size > ((*section).size).wrapping_sub(dataoff) {
        return 0 as libc::c_int != 0;
    }
    if !bfd_malloc_and_get_section(abfd, section, &mut data) {
        free(data as *mut libc::c_void);
        return 0 as libc::c_int != 0;
    }
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong)
        < size
            .wrapping_div(
                ::core::mem::size_of::<external_IMAGE_DEBUG_DIRECTORY>() as libc::c_ulong,
            )
    {
        let mut ext: *mut external_IMAGE_DEBUG_DIRECTORY = &mut *(data
            .offset(dataoff as isize) as *mut external_IMAGE_DEBUG_DIRECTORY)
            .offset(i as isize) as *mut external_IMAGE_DEBUG_DIRECTORY;
        let mut idd: internal_IMAGE_DEBUG_DIRECTORY = internal_IMAGE_DEBUG_DIRECTORY {
            Characteristics: 0,
            TimeDateStamp: 0,
            MajorVersion: 0,
            MinorVersion: 0,
            Type: 0,
            SizeOfData: 0,
            AddressOfRawData: 0,
            PointerToRawData: 0,
        };
        _bfd_pex64i_swap_debugdir_in(
            abfd,
            ext as *mut libc::c_void,
            &mut idd as *mut internal_IMAGE_DEBUG_DIRECTORY as *mut libc::c_void,
        );
        if idd.Type == 16 as libc::c_int as libc::c_ulong {
            res = 1 as libc::c_int != 0;
            break;
        } else {
            i = i.wrapping_add(1);
            i;
        }
    }
    free(data as *mut libc::c_void);
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn _bfd_pex64_print_private_bfd_data_common(
    mut abfd: *mut bfd,
    mut vfile: *mut libc::c_void,
) -> bool {
    let mut file: *mut FILE = vfile as *mut FILE;
    let mut j: libc::c_int = 0;
    let mut pe: *mut pe_data_type = (*abfd).tdata.pe_obj_data;
    let mut i: *mut internal_extra_pe_aouthdr = &mut (*pe).pe_opthdr;
    let mut subsystem_name: *const libc::c_char = 0 as *const libc::c_char;
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    fprintf(
        file,
        dcgettext(
            b"bfd\0" as *const u8 as *const libc::c_char,
            b"\nCharacteristics 0x%x\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        (*pe).real_flags,
    );
    if (*pe).real_flags & 0x1 as libc::c_int as libc::c_uint != 0 {
        fprintf(
            file,
            b"\t%s\n\0" as *const u8 as *const libc::c_char,
            b"relocations stripped\0" as *const u8 as *const libc::c_char,
        );
    }
    if (*pe).real_flags & 0x2 as libc::c_int as libc::c_uint != 0 {
        fprintf(
            file,
            b"\t%s\n\0" as *const u8 as *const libc::c_char,
            b"executable\0" as *const u8 as *const libc::c_char,
        );
    }
    if (*pe).real_flags & 0x4 as libc::c_int as libc::c_uint != 0 {
        fprintf(
            file,
            b"\t%s\n\0" as *const u8 as *const libc::c_char,
            b"line numbers stripped\0" as *const u8 as *const libc::c_char,
        );
    }
    if (*pe).real_flags & 0x8 as libc::c_int as libc::c_uint != 0 {
        fprintf(
            file,
            b"\t%s\n\0" as *const u8 as *const libc::c_char,
            b"symbols stripped\0" as *const u8 as *const libc::c_char,
        );
    }
    if (*pe).real_flags & 0x20 as libc::c_int as libc::c_uint != 0 {
        fprintf(
            file,
            b"\t%s\n\0" as *const u8 as *const libc::c_char,
            b"large address aware\0" as *const u8 as *const libc::c_char,
        );
    }
    if (*pe).real_flags & 0x80 as libc::c_int as libc::c_uint != 0 {
        fprintf(
            file,
            b"\t%s\n\0" as *const u8 as *const libc::c_char,
            b"little endian\0" as *const u8 as *const libc::c_char,
        );
    }
    if (*pe).real_flags & 0x100 as libc::c_int as libc::c_uint != 0 {
        fprintf(
            file,
            b"\t%s\n\0" as *const u8 as *const libc::c_char,
            b"32 bit words\0" as *const u8 as *const libc::c_char,
        );
    }
    if (*pe).real_flags & 0x200 as libc::c_int as libc::c_uint != 0 {
        fprintf(
            file,
            b"\t%s\n\0" as *const u8 as *const libc::c_char,
            b"debugging information removed\0" as *const u8 as *const libc::c_char,
        );
    }
    if (*pe).real_flags & 0x400 as libc::c_int as libc::c_uint != 0 {
        fprintf(
            file,
            b"\t%s\n\0" as *const u8 as *const libc::c_char,
            b"copy to swap file if on removable media\0" as *const u8
                as *const libc::c_char,
        );
    }
    if (*pe).real_flags & 0x800 as libc::c_int as libc::c_uint != 0 {
        fprintf(
            file,
            b"\t%s\n\0" as *const u8 as *const libc::c_char,
            b"copy to swap file if on network media\0" as *const u8
                as *const libc::c_char,
        );
    }
    if (*pe).real_flags & 0x1000 as libc::c_int as libc::c_uint != 0 {
        fprintf(
            file,
            b"\t%s\n\0" as *const u8 as *const libc::c_char,
            b"system file\0" as *const u8 as *const libc::c_char,
        );
    }
    if (*pe).real_flags & 0x2000 as libc::c_int as libc::c_uint != 0 {
        fprintf(
            file,
            b"\t%s\n\0" as *const u8 as *const libc::c_char,
            b"DLL\0" as *const u8 as *const libc::c_char,
        );
    }
    if (*pe).real_flags & 0x4000 as libc::c_int as libc::c_uint != 0 {
        fprintf(
            file,
            b"\t%s\n\0" as *const u8 as *const libc::c_char,
            b"run only on uniprocessor machine\0" as *const u8 as *const libc::c_char,
        );
    }
    if (*pe).real_flags & 0x8000 as libc::c_int as libc::c_uint != 0 {
        fprintf(
            file,
            b"\t%s\n\0" as *const u8 as *const libc::c_char,
            b"big endian\0" as *const u8 as *const libc::c_char,
        );
    }
    if pe_is_repro(abfd) {
        fprintf(
            file,
            b"\nTime/Date\t\t%08lx\0" as *const u8 as *const libc::c_char,
            (*pe).coff.timestamp,
        );
        fprintf(
            file,
            b"\t(This is a reproducible build file hash, not a timestamp)\n\0"
                as *const u8 as *const libc::c_char,
        );
    } else {
        let mut t: time_t = (*pe).coff.timestamp;
        fprintf(
            file,
            b"\nTime/Date\t\t%s\0" as *const u8 as *const libc::c_char,
            ctime(&mut t),
        );
    }
    match (*i).Magic as libc::c_int {
        267 => {
            name = b"PE32\0" as *const u8 as *const libc::c_char;
        }
        523 => {
            name = b"PE32+\0" as *const u8 as *const libc::c_char;
        }
        263 => {
            name = b"ROM\0" as *const u8 as *const libc::c_char;
        }
        _ => {
            name = 0 as *const libc::c_char;
        }
    }
    fprintf(
        file,
        b"Magic\t\t\t%04x\0" as *const u8 as *const libc::c_char,
        (*i).Magic as libc::c_int,
    );
    if !name.is_null() {
        fprintf(file, b"\t(%s)\0" as *const u8 as *const libc::c_char, name);
    }
    fprintf(
        file,
        b"\nMajorLinkerVersion\t%d\n\0" as *const u8 as *const libc::c_char,
        (*i).MajorLinkerVersion as libc::c_int,
    );
    fprintf(
        file,
        b"MinorLinkerVersion\t%d\n\0" as *const u8 as *const libc::c_char,
        (*i).MinorLinkerVersion as libc::c_int,
    );
    fprintf(file, b"SizeOfCode\t\t\0" as *const u8 as *const libc::c_char);
    bfd_fprintf_vma(abfd, file as *mut libc::c_void, (*i).SizeOfCode);
    fprintf(file, b"\nSizeOfInitializedData\t\0" as *const u8 as *const libc::c_char);
    bfd_fprintf_vma(abfd, file as *mut libc::c_void, (*i).SizeOfInitializedData);
    fprintf(file, b"\nSizeOfUninitializedData\t\0" as *const u8 as *const libc::c_char);
    bfd_fprintf_vma(abfd, file as *mut libc::c_void, (*i).SizeOfUninitializedData);
    fprintf(file, b"\nAddressOfEntryPoint\t\0" as *const u8 as *const libc::c_char);
    bfd_fprintf_vma(abfd, file as *mut libc::c_void, (*i).AddressOfEntryPoint);
    fprintf(file, b"\nBaseOfCode\t\t\0" as *const u8 as *const libc::c_char);
    bfd_fprintf_vma(abfd, file as *mut libc::c_void, (*i).BaseOfCode);
    fprintf(file, b"\nImageBase\t\t\0" as *const u8 as *const libc::c_char);
    bfd_fprintf_vma(abfd, file as *mut libc::c_void, (*i).ImageBase);
    fprintf(
        file,
        b"\nSectionAlignment\t%08x\n\0" as *const u8 as *const libc::c_char,
        (*i).SectionAlignment,
    );
    fprintf(
        file,
        b"FileAlignment\t\t%08x\n\0" as *const u8 as *const libc::c_char,
        (*i).FileAlignment,
    );
    fprintf(
        file,
        b"MajorOSystemVersion\t%d\n\0" as *const u8 as *const libc::c_char,
        (*i).MajorOperatingSystemVersion as libc::c_int,
    );
    fprintf(
        file,
        b"MinorOSystemVersion\t%d\n\0" as *const u8 as *const libc::c_char,
        (*i).MinorOperatingSystemVersion as libc::c_int,
    );
    fprintf(
        file,
        b"MajorImageVersion\t%d\n\0" as *const u8 as *const libc::c_char,
        (*i).MajorImageVersion as libc::c_int,
    );
    fprintf(
        file,
        b"MinorImageVersion\t%d\n\0" as *const u8 as *const libc::c_char,
        (*i).MinorImageVersion as libc::c_int,
    );
    fprintf(
        file,
        b"MajorSubsystemVersion\t%d\n\0" as *const u8 as *const libc::c_char,
        (*i).MajorSubsystemVersion as libc::c_int,
    );
    fprintf(
        file,
        b"MinorSubsystemVersion\t%d\n\0" as *const u8 as *const libc::c_char,
        (*i).MinorSubsystemVersion as libc::c_int,
    );
    fprintf(
        file,
        b"Win32Version\t\t%08x\n\0" as *const u8 as *const libc::c_char,
        (*i).Reserved1,
    );
    fprintf(
        file,
        b"SizeOfImage\t\t%08x\n\0" as *const u8 as *const libc::c_char,
        (*i).SizeOfImage,
    );
    fprintf(
        file,
        b"SizeOfHeaders\t\t%08x\n\0" as *const u8 as *const libc::c_char,
        (*i).SizeOfHeaders,
    );
    fprintf(
        file,
        b"CheckSum\t\t%08x\n\0" as *const u8 as *const libc::c_char,
        (*i).CheckSum,
    );
    match (*i).Subsystem as libc::c_int {
        0 => {
            subsystem_name = b"unspecified\0" as *const u8 as *const libc::c_char;
        }
        1 => {
            subsystem_name = b"NT native\0" as *const u8 as *const libc::c_char;
        }
        2 => {
            subsystem_name = b"Windows GUI\0" as *const u8 as *const libc::c_char;
        }
        3 => {
            subsystem_name = b"Windows CUI\0" as *const u8 as *const libc::c_char;
        }
        7 => {
            subsystem_name = b"POSIX CUI\0" as *const u8 as *const libc::c_char;
        }
        9 => {
            subsystem_name = b"Wince CUI\0" as *const u8 as *const libc::c_char;
        }
        10 => {
            subsystem_name = b"EFI application\0" as *const u8 as *const libc::c_char;
        }
        11 => {
            subsystem_name = b"EFI boot service driver\0" as *const u8
                as *const libc::c_char;
        }
        12 => {
            subsystem_name = b"EFI runtime driver\0" as *const u8 as *const libc::c_char;
        }
        13 => {
            subsystem_name = b"SAL runtime driver\0" as *const u8 as *const libc::c_char;
        }
        14 => {
            subsystem_name = b"XBOX\0" as *const u8 as *const libc::c_char;
        }
        _ => {
            subsystem_name = 0 as *const libc::c_char;
        }
    }
    fprintf(
        file,
        b"Subsystem\t\t%08x\0" as *const u8 as *const libc::c_char,
        (*i).Subsystem as libc::c_int,
    );
    if !subsystem_name.is_null() {
        fprintf(file, b"\t(%s)\0" as *const u8 as *const libc::c_char, subsystem_name);
    }
    fprintf(
        file,
        b"\nDllCharacteristics\t%08x\n\0" as *const u8 as *const libc::c_char,
        (*i).DllCharacteristics as libc::c_int,
    );
    if (*i).DllCharacteristics != 0 {
        let mut dllch: libc::c_ushort = (*i).DllCharacteristics;
        let mut indent: *const libc::c_char = b"\t\t\t\t\t\0" as *const u8
            as *const libc::c_char;
        if dllch as libc::c_int & 0x20 as libc::c_int != 0 {
            fprintf(
                file,
                b"%sHIGH_ENTROPY_VA\n\0" as *const u8 as *const libc::c_char,
                indent,
            );
        }
        if dllch as libc::c_int & 0x40 as libc::c_int != 0 {
            fprintf(
                file,
                b"%sDYNAMIC_BASE\n\0" as *const u8 as *const libc::c_char,
                indent,
            );
        }
        if dllch as libc::c_int & 0x80 as libc::c_int != 0 {
            fprintf(
                file,
                b"%sFORCE_INTEGRITY\n\0" as *const u8 as *const libc::c_char,
                indent,
            );
        }
        if dllch as libc::c_int & 0x100 as libc::c_int != 0 {
            fprintf(
                file,
                b"%sNX_COMPAT\n\0" as *const u8 as *const libc::c_char,
                indent,
            );
        }
        if dllch as libc::c_int & 0x200 as libc::c_int != 0 {
            fprintf(
                file,
                b"%sNO_ISOLATION\n\0" as *const u8 as *const libc::c_char,
                indent,
            );
        }
        if dllch as libc::c_int & 0x400 as libc::c_int != 0 {
            fprintf(file, b"%sNO_SEH\n\0" as *const u8 as *const libc::c_char, indent);
        }
        if dllch as libc::c_int & 0x800 as libc::c_int != 0 {
            fprintf(file, b"%sNO_BIND\n\0" as *const u8 as *const libc::c_char, indent);
        }
        if dllch as libc::c_int & 0x1000 as libc::c_int != 0 {
            fprintf(
                file,
                b"%sAPPCONTAINER\n\0" as *const u8 as *const libc::c_char,
                indent,
            );
        }
        if dllch as libc::c_int & 0x2000 as libc::c_int != 0 {
            fprintf(
                file,
                b"%sWDM_DRIVER\n\0" as *const u8 as *const libc::c_char,
                indent,
            );
        }
        if dllch as libc::c_int & 0x4000 as libc::c_int != 0 {
            fprintf(file, b"%sGUARD_CF\n\0" as *const u8 as *const libc::c_char, indent);
        }
        if dllch as libc::c_int & 0x8000 as libc::c_int != 0 {
            fprintf(
                file,
                b"%sTERMINAL_SERVICE_AWARE\n\0" as *const u8 as *const libc::c_char,
                indent,
            );
        }
    }
    fprintf(file, b"SizeOfStackReserve\t\0" as *const u8 as *const libc::c_char);
    bfd_fprintf_vma(abfd, file as *mut libc::c_void, (*i).SizeOfStackReserve);
    fprintf(file, b"\nSizeOfStackCommit\t\0" as *const u8 as *const libc::c_char);
    bfd_fprintf_vma(abfd, file as *mut libc::c_void, (*i).SizeOfStackCommit);
    fprintf(file, b"\nSizeOfHeapReserve\t\0" as *const u8 as *const libc::c_char);
    bfd_fprintf_vma(abfd, file as *mut libc::c_void, (*i).SizeOfHeapReserve);
    fprintf(file, b"\nSizeOfHeapCommit\t\0" as *const u8 as *const libc::c_char);
    bfd_fprintf_vma(abfd, file as *mut libc::c_void, (*i).SizeOfHeapCommit);
    fprintf(
        file,
        b"\nLoaderFlags\t\t%08lx\n\0" as *const u8 as *const libc::c_char,
        (*i).LoaderFlags as libc::c_ulong,
    );
    fprintf(
        file,
        b"NumberOfRvaAndSizes\t%08lx\n\0" as *const u8 as *const libc::c_char,
        (*i).NumberOfRvaAndSizes as libc::c_ulong,
    );
    fprintf(file, b"\nThe Data Directory\n\0" as *const u8 as *const libc::c_char);
    j = 0 as libc::c_int;
    while j < 16 as libc::c_int {
        fprintf(file, b"Entry %1x \0" as *const u8 as *const libc::c_char, j);
        bfd_fprintf_vma(
            abfd,
            file as *mut libc::c_void,
            (*i).DataDirectory[j as usize].VirtualAddress,
        );
        fprintf(
            file,
            b" %08lx \0" as *const u8 as *const libc::c_char,
            (*i).DataDirectory[j as usize].Size as libc::c_ulong,
        );
        fprintf(
            file,
            b"%s\n\0" as *const u8 as *const libc::c_char,
            dir_names[j as usize],
        );
        j += 1;
        j;
    }
    pe_print_idata(abfd, vfile);
    pe_print_edata(abfd, vfile);
    if ((*((*(*abfd).xvec).backend_data as *mut bfd_coff_backend_data))
        ._bfd_coff_print_pdata)
        .is_some()
    {
        ((*((*(*abfd).xvec).backend_data as *mut bfd_coff_backend_data))
            ._bfd_coff_print_pdata)
            .expect("non-null function pointer")(abfd, vfile);
    } else {
        pe_print_pdata(abfd, vfile);
    }
    pe_print_reloc(abfd, vfile);
    pe_print_debugdata(abfd, file as *mut libc::c_void);
    rsrc_print_section(abfd, vfile);
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn is_vma_in_section(
    mut _abfd: *mut bfd,
    mut sect: *mut asection,
    mut obj: *mut libc::c_void,
) -> bool {
    let mut addr: bfd_vma = *(obj as *mut bfd_vma);
    return addr >= (*sect).vma && addr < ((*sect).vma).wrapping_add((*sect).size);
}
unsafe extern "C" fn find_section_by_vma(
    mut abfd: *mut bfd,
    mut addr: bfd_vma,
) -> *mut asection {
    return bfd_sections_find_if(
        abfd,
        Some(
            is_vma_in_section
                as unsafe extern "C" fn(
                    *mut bfd,
                    *mut asection,
                    *mut libc::c_void,
                ) -> bool,
        ),
        &mut addr as *mut bfd_vma as *mut libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn _bfd_pex64_bfd_copy_private_bfd_data_common(
    mut ibfd: *mut bfd,
    mut obfd: *mut bfd,
) -> bool {
    let mut ipe: *mut pe_data_type = 0 as *mut pe_data_type;
    let mut ope: *mut pe_data_type = 0 as *mut pe_data_type;
    if (*(*ibfd).xvec).flavour as libc::c_uint
        != bfd_target_coff_flavour as libc::c_int as libc::c_uint
        || (*(*obfd).xvec).flavour as libc::c_uint
            != bfd_target_coff_flavour as libc::c_int as libc::c_uint
    {
        return 1 as libc::c_int != 0;
    }
    ipe = (*ibfd).tdata.pe_obj_data;
    ope = (*obfd).tdata.pe_obj_data;
    (*ope).dll = (*ipe).dll;
    if (*obfd).xvec != (*ibfd).xvec {
        (*ope).pe_opthdr.Subsystem = 0 as libc::c_int as libc::c_short;
    }
    if (*(*obfd).tdata.pe_obj_data).has_reloc_section == 0 {
        (*(*obfd).tdata.pe_obj_data)
            .pe_opthdr
            .DataDirectory[5 as libc::c_int as usize]
            .VirtualAddress = 0 as libc::c_int as bfd_vma;
        (*(*obfd).tdata.pe_obj_data)
            .pe_opthdr
            .DataDirectory[5 as libc::c_int as usize]
            .Size = 0 as libc::c_int as libc::c_long;
    }
    if (*(*ibfd).tdata.pe_obj_data).has_reloc_section == 0
        && (*(*ibfd).tdata.pe_obj_data).real_flags & 0x1 as libc::c_int as libc::c_uint
            == 0
    {
        (*(*obfd).tdata.pe_obj_data).dont_strip_reloc = 1 as libc::c_int;
    }
    memcpy(
        ((*ope).dos_message).as_mut_ptr() as *mut libc::c_void,
        ((*ipe).dos_message).as_mut_ptr() as *const libc::c_void,
        ::core::mem::size_of::<[libc::c_int; 16]>() as libc::c_ulong,
    );
    if (*ope).pe_opthdr.DataDirectory[6 as libc::c_int as usize].Size
        != 0 as libc::c_int as libc::c_long
    {
        let mut addr: bfd_vma = ((*ope)
            .pe_opthdr
            .DataDirectory[6 as libc::c_int as usize]
            .VirtualAddress)
            .wrapping_add((*ope).pe_opthdr.ImageBase);
        let mut last: bfd_vma = addr
            .wrapping_add(
                (*ope).pe_opthdr.DataDirectory[6 as libc::c_int as usize].Size
                    as libc::c_ulong,
            )
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        let mut section: *mut asection = find_section_by_vma(obfd, last);
        let mut data: *mut bfd_byte = 0 as *mut bfd_byte;
        if !section.is_null() && addr < (*section).vma {
            _bfd_error_handler(
                dcgettext(
                    b"bfd\0" as *const u8 as *const libc::c_char,
                    b"%pB: Data Directory (%lx bytes at %lx) extends across section boundary at %lx\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                obfd,
                (*ope).pe_opthdr.DataDirectory[6 as libc::c_int as usize].Size,
                addr,
                (*section).vma,
            );
            return 0 as libc::c_int != 0;
        }
        if !section.is_null()
            && bfd_malloc_and_get_section(obfd, section, &mut data) as libc::c_int != 0
        {
            let mut i: libc::c_uint = 0;
            let mut dd: *mut external_IMAGE_DEBUG_DIRECTORY = data
                .offset(addr.wrapping_sub((*section).vma) as isize)
                as *mut external_IMAGE_DEBUG_DIRECTORY;
            i = 0 as libc::c_int as libc::c_uint;
            while (i as libc::c_ulong)
                < ((*ope).pe_opthdr.DataDirectory[6 as libc::c_int as usize].Size
                    as libc::c_ulong)
                    .wrapping_div(
                        ::core::mem::size_of::<external_IMAGE_DEBUG_DIRECTORY>()
                            as libc::c_ulong,
                    )
            {
                let mut ddsection: *mut asection = 0 as *mut asection;
                let mut edd: *mut external_IMAGE_DEBUG_DIRECTORY = &mut *dd
                    .offset(i as isize) as *mut external_IMAGE_DEBUG_DIRECTORY;
                let mut idd: internal_IMAGE_DEBUG_DIRECTORY = internal_IMAGE_DEBUG_DIRECTORY {
                    Characteristics: 0,
                    TimeDateStamp: 0,
                    MajorVersion: 0,
                    MinorVersion: 0,
                    Type: 0,
                    SizeOfData: 0,
                    AddressOfRawData: 0,
                    PointerToRawData: 0,
                };
                _bfd_pex64i_swap_debugdir_in(
                    obfd,
                    edd as *mut libc::c_void,
                    &mut idd as *mut internal_IMAGE_DEBUG_DIRECTORY as *mut libc::c_void,
                );
                if !(idd.AddressOfRawData == 0 as libc::c_int as libc::c_ulong) {
                    ddsection = find_section_by_vma(
                        obfd,
                        (idd.AddressOfRawData).wrapping_add((*ope).pe_opthdr.ImageBase),
                    );
                    if !ddsection.is_null() {
                        idd
                            .PointerToRawData = ((*ddsection).filepos as libc::c_ulong)
                            .wrapping_add(
                                (idd.AddressOfRawData)
                                    .wrapping_add((*ope).pe_opthdr.ImageBase),
                            )
                            .wrapping_sub((*ddsection).vma);
                        _bfd_pex64i_swap_debugdir_out(
                            obfd,
                            &mut idd as *mut internal_IMAGE_DEBUG_DIRECTORY
                                as *mut libc::c_void,
                            edd as *mut libc::c_void,
                        );
                    }
                }
                i = i.wrapping_add(1);
                i;
            }
            if !bfd_set_section_contents(
                obfd,
                section,
                data as *const libc::c_void,
                0 as libc::c_int as file_ptr,
                (*section).size,
            ) {
                _bfd_error_handler(
                    dcgettext(
                        b"bfd\0" as *const u8 as *const libc::c_char,
                        b"failed to update file offsets in debug directory\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                free(data as *mut libc::c_void);
                return 0 as libc::c_int != 0;
            }
            free(data as *mut libc::c_void);
        } else if !section.is_null() {
            _bfd_error_handler(
                dcgettext(
                    b"bfd\0" as *const u8 as *const libc::c_char,
                    b"%pB: failed to read debug data section\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                obfd,
            );
            return 0 as libc::c_int != 0;
        }
    }
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn _bfd_pex64_bfd_copy_private_section_data(
    mut ibfd: *mut bfd,
    mut isec: *mut asection,
    mut obfd: *mut bfd,
    mut osec: *mut asection,
) -> bool {
    if bfd_get_flavour(ibfd) as libc::c_uint
        != bfd_target_coff_flavour as libc::c_int as libc::c_uint
        || bfd_get_flavour(obfd) as libc::c_uint
            != bfd_target_coff_flavour as libc::c_int as libc::c_uint
    {
        return 1 as libc::c_int != 0;
    }
    if !((*isec).used_by_bfd as *mut coff_section_tdata).is_null()
        && !((*((*isec).used_by_bfd as *mut coff_section_tdata)).tdata
            as *mut pei_section_tdata)
            .is_null()
    {
        if ((*osec).used_by_bfd as *mut coff_section_tdata).is_null() {
            let mut amt: size_t = ::core::mem::size_of::<coff_section_tdata>()
                as libc::c_ulong;
            (*osec).used_by_bfd = bfd_zalloc(obfd, amt);
            if ((*osec).used_by_bfd).is_null() {
                return 0 as libc::c_int != 0;
            }
        }
        if ((*((*osec).used_by_bfd as *mut coff_section_tdata)).tdata
            as *mut pei_section_tdata)
            .is_null()
        {
            let mut amt_0: size_t = ::core::mem::size_of::<pei_section_tdata>()
                as libc::c_ulong;
            let ref mut fresh3 = (*((*osec).used_by_bfd as *mut coff_section_tdata))
                .tdata;
            *fresh3 = bfd_zalloc(obfd, amt_0);
            if ((*((*osec).used_by_bfd as *mut coff_section_tdata)).tdata).is_null() {
                return 0 as libc::c_int != 0;
            }
        }
        (*((*((*osec).used_by_bfd as *mut coff_section_tdata)).tdata
            as *mut pei_section_tdata))
            .virt_size = (*((*((*isec).used_by_bfd as *mut coff_section_tdata)).tdata
            as *mut pei_section_tdata))
            .virt_size;
        (*((*((*osec).used_by_bfd as *mut coff_section_tdata)).tdata
            as *mut pei_section_tdata))
            .pe_flags = (*((*((*isec).used_by_bfd as *mut coff_section_tdata)).tdata
            as *mut pei_section_tdata))
            .pe_flags;
    }
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn _bfd_pex64_get_symbol_info(
    mut abfd: *mut bfd,
    mut symbol: *mut asymbol,
    mut ret: *mut symbol_info,
) {
    coff_get_symbol_info(abfd, symbol, ret);
}
unsafe extern "C" fn sort_x64_pdata(
    mut l: *const libc::c_void,
    mut r: *const libc::c_void,
) -> libc::c_int {
    let mut lp: *const libc::c_char = l as *const libc::c_char;
    let mut rp: *const libc::c_char = r as *const libc::c_char;
    let mut vl: bfd_vma = 0;
    let mut vr: bfd_vma = 0;
    vl = bfd_getl32(lp as *const libc::c_void);
    vr = bfd_getl32(rp as *const libc::c_void);
    if vl != vr {
        return if vl < vr { -(1 as libc::c_int) } else { 1 as libc::c_int };
    }
    return 0 as libc::c_int;
}
static mut sizeof_leaves: libc::c_uint = 0;
static mut sizeof_strings: libc::c_uint = 0;
static mut sizeof_tables_and_entries: libc::c_uint = 0;
unsafe extern "C" fn rsrc_count_entries(
    mut abfd: *mut bfd,
    mut is_name: bool,
    mut datastart: *mut bfd_byte,
    mut data: *mut bfd_byte,
    mut dataend: *mut bfd_byte,
    mut rva_bias: bfd_vma,
) -> *mut bfd_byte {
    let mut entry: libc::c_ulong = 0;
    let mut addr: libc::c_ulong = 0;
    let mut size: libc::c_ulong = 0;
    if data.offset(8 as libc::c_int as isize) >= dataend {
        return dataend.offset(1 as libc::c_int as isize);
    }
    if is_name {
        let mut name: *mut bfd_byte = 0 as *mut bfd_byte;
        entry = (Some(((*(*abfd).xvec).bfd_getx32).expect("non-null function pointer")))
            .expect("non-null function pointer")(data as *const libc::c_void)
            as libc::c_long as libc::c_ulong;
        if entry & 0x80000000 as libc::c_uint as libc::c_ulong != 0 {
            name = datastart
                .offset((entry & 0x7fffffff as libc::c_int as libc::c_ulong) as isize);
        } else {
            name = datastart.offset(entry as isize).offset(-(rva_bias as isize));
        }
        if name.offset(2 as libc::c_int as isize) >= dataend || name < datastart {
            return dataend.offset(1 as libc::c_int as isize);
        }
        let mut len: libc::c_uint = (Some(
            ((*(*abfd).xvec).bfd_getx16).expect("non-null function pointer"),
        ))
            .expect("non-null function pointer")(name as *const libc::c_void)
            as libc::c_uint;
        if len == 0 as libc::c_int as libc::c_uint
            || len > 256 as libc::c_int as libc::c_uint
        {
            return dataend.offset(1 as libc::c_int as isize);
        }
    }
    entry = (Some(((*(*abfd).xvec).bfd_getx32).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(data.offset(4 as libc::c_int as isize) as *const libc::c_void) as libc::c_long
        as libc::c_ulong;
    if entry & 0x80000000 as libc::c_uint as libc::c_ulong != 0 {
        data = datastart
            .offset((entry & 0x7fffffff as libc::c_int as libc::c_ulong) as isize);
        if data <= datastart || data >= dataend {
            return dataend.offset(1 as libc::c_int as isize);
        }
        return rsrc_count_directory(abfd, datastart, data, dataend, rva_bias);
    }
    if datastart.offset(entry as isize).offset(16 as libc::c_int as isize) >= dataend {
        return dataend.offset(1 as libc::c_int as isize);
    }
    addr = (Some(((*(*abfd).xvec).bfd_getx32).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(datastart.offset(entry as isize) as *const libc::c_void) as libc::c_long
        as libc::c_ulong;
    size = (Some(((*(*abfd).xvec).bfd_getx32).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        datastart.offset(entry as isize).offset(4 as libc::c_int as isize)
            as *const libc::c_void,
    ) as libc::c_long as libc::c_ulong;
    return datastart
        .offset(addr as isize)
        .offset(-(rva_bias as isize))
        .offset(size as isize);
}
unsafe extern "C" fn rsrc_count_directory(
    mut abfd: *mut bfd,
    mut datastart: *mut bfd_byte,
    mut data: *mut bfd_byte,
    mut dataend: *mut bfd_byte,
    mut rva_bias: bfd_vma,
) -> *mut bfd_byte {
    let mut num_entries: libc::c_uint = 0;
    let mut num_ids: libc::c_uint = 0;
    let mut highest_data: *mut bfd_byte = data;
    if data.offset(16 as libc::c_int as isize) >= dataend {
        return dataend.offset(1 as libc::c_int as isize);
    }
    num_entries = (Some(
        ((*(*abfd).xvec).bfd_getx16).expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )(data.offset(12 as libc::c_int as isize) as *const libc::c_void) as libc::c_int
        as libc::c_uint;
    num_ids = (Some(((*(*abfd).xvec).bfd_getx16).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(data.offset(14 as libc::c_int as isize) as *const libc::c_void) as libc::c_int
        as libc::c_uint;
    num_entries = num_entries.wrapping_add(num_ids);
    data = data.offset(16 as libc::c_int as isize);
    loop {
        let fresh4 = num_entries;
        num_entries = num_entries.wrapping_sub(1);
        if !(fresh4 != 0) {
            break;
        }
        let mut entry_end: *mut bfd_byte = 0 as *mut bfd_byte;
        entry_end = rsrc_count_entries(
            abfd,
            num_entries >= num_ids,
            datastart,
            data,
            dataend,
            rva_bias,
        );
        data = data.offset(8 as libc::c_int as isize);
        highest_data = if highest_data > entry_end { highest_data } else { entry_end };
        if entry_end >= dataend {
            break;
        }
    }
    return if highest_data > data { highest_data } else { data };
}
unsafe extern "C" fn rsrc_parse_entry(
    mut abfd: *mut bfd,
    mut is_name: bool,
    mut entry: *mut rsrc_entry,
    mut datastart: *mut bfd_byte,
    mut data: *mut bfd_byte,
    mut dataend: *mut bfd_byte,
    mut rva_bias: bfd_vma,
    mut parent: *mut rsrc_directory,
) -> *mut bfd_byte {
    let mut val: libc::c_ulong = 0;
    let mut addr: libc::c_ulong = 0;
    let mut size: libc::c_ulong = 0;
    val = (Some(((*(*abfd).xvec).bfd_getx32).expect("non-null function pointer")))
        .expect("non-null function pointer")(data as *const libc::c_void);
    (*entry).parent = parent;
    (*entry).is_name = is_name;
    if is_name {
        let mut address: *mut bfd_byte = 0 as *mut bfd_byte;
        if val & 0x80000000 as libc::c_uint as libc::c_ulong != 0 {
            val = val & 0x7fffffff as libc::c_int as libc::c_ulong;
            address = datastart.offset(val as isize);
        } else {
            address = datastart.offset(val as isize).offset(-(rva_bias as isize));
        }
        if address.offset(3 as libc::c_int as isize) > dataend {
            return dataend;
        }
        (*entry)
            .name_id
            .name
            .len = (Some(
            ((*(*abfd).xvec).bfd_getx16).expect("non-null function pointer"),
        ))
            .expect("non-null function pointer")(address as *const libc::c_void)
            as libc::c_uint;
        (*entry).name_id.name.string = address.offset(2 as libc::c_int as isize);
    } else {
        (*entry).name_id.id = val as libc::c_uint;
    }
    val = (Some(((*(*abfd).xvec).bfd_getx32).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(data.offset(4 as libc::c_int as isize) as *const libc::c_void);
    if val & 0x80000000 as libc::c_uint as libc::c_ulong != 0 {
        (*entry).is_dir = 1 as libc::c_int != 0;
        (*entry)
            .value
            .directory = bfd_malloc(
            ::core::mem::size_of::<rsrc_directory>() as libc::c_ulong,
        ) as *mut rsrc_directory;
        if ((*entry).value.directory).is_null() {
            return dataend;
        }
        return rsrc_parse_directory(
            abfd,
            (*entry).value.directory,
            datastart,
            datastart
                .offset((val & 0x7fffffff as libc::c_int as libc::c_ulong) as isize),
            dataend,
            rva_bias,
            entry,
        );
    }
    (*entry).is_dir = 0 as libc::c_int != 0;
    (*entry)
        .value
        .leaf = bfd_malloc(::core::mem::size_of::<rsrc_leaf>() as libc::c_ulong)
        as *mut rsrc_leaf;
    if ((*entry).value.leaf).is_null() {
        return dataend;
    }
    data = datastart.offset(val as isize);
    if data < datastart || data >= dataend {
        return dataend;
    }
    addr = (Some(((*(*abfd).xvec).bfd_getx32).expect("non-null function pointer")))
        .expect("non-null function pointer")(data as *const libc::c_void);
    (*(*entry).value.leaf)
        .size = (Some(((*(*abfd).xvec).bfd_getx32).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(data.offset(4 as libc::c_int as isize) as *const libc::c_void) as libc::c_uint;
    size = (*(*entry).value.leaf).size as libc::c_ulong;
    (*(*entry).value.leaf)
        .codepage = (Some(
        ((*(*abfd).xvec).bfd_getx32).expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )(data.offset(8 as libc::c_int as isize) as *const libc::c_void) as libc::c_uint;
    (*(*entry).value.leaf).data = bfd_malloc(size) as *mut bfd_byte;
    if ((*(*entry).value.leaf).data).is_null() {
        return dataend;
    }
    memcpy(
        (*(*entry).value.leaf).data as *mut libc::c_void,
        datastart.offset(addr as isize).offset(-(rva_bias as isize))
            as *const libc::c_void,
        size,
    );
    return datastart.offset(addr.wrapping_sub(rva_bias) as isize).offset(size as isize);
}
unsafe extern "C" fn rsrc_parse_entries(
    mut abfd: *mut bfd,
    mut chain: *mut rsrc_dir_chain,
    mut is_name: bool,
    mut highest_data: *mut bfd_byte,
    mut datastart: *mut bfd_byte,
    mut data: *mut bfd_byte,
    mut dataend: *mut bfd_byte,
    mut rva_bias: bfd_vma,
    mut parent: *mut rsrc_directory,
) -> *mut bfd_byte {
    let mut i: libc::c_uint = 0;
    let mut entry: *mut rsrc_entry = 0 as *mut rsrc_entry;
    if (*chain).num_entries == 0 as libc::c_int as libc::c_uint {
        (*chain).last_entry = 0 as *mut rsrc_entry;
        (*chain).first_entry = (*chain).last_entry;
        return highest_data;
    }
    entry = bfd_malloc(::core::mem::size_of::<rsrc_entry>() as libc::c_ulong)
        as *mut rsrc_entry;
    if entry.is_null() {
        return dataend;
    }
    (*chain).first_entry = entry;
    i = (*chain).num_entries;
    loop {
        let fresh5 = i;
        i = i.wrapping_sub(1);
        if !(fresh5 != 0) {
            break;
        }
        let mut entry_end: *mut bfd_byte = 0 as *mut bfd_byte;
        entry_end = rsrc_parse_entry(
            abfd,
            is_name,
            entry,
            datastart,
            data,
            dataend,
            rva_bias,
            parent,
        );
        data = data.offset(8 as libc::c_int as isize);
        highest_data = if entry_end > highest_data { entry_end } else { highest_data };
        if entry_end > dataend {
            return dataend;
        }
        if i != 0 {
            (*entry)
                .next_entry = bfd_malloc(
                ::core::mem::size_of::<rsrc_entry>() as libc::c_ulong,
            ) as *mut rsrc_entry;
            entry = (*entry).next_entry;
            if entry.is_null() {
                return dataend;
            }
        } else {
            (*entry).next_entry = 0 as *mut rsrc_entry;
        }
    }
    (*chain).last_entry = entry;
    return highest_data;
}
unsafe extern "C" fn rsrc_parse_directory(
    mut abfd: *mut bfd,
    mut table: *mut rsrc_directory,
    mut datastart: *mut bfd_byte,
    mut data: *mut bfd_byte,
    mut dataend: *mut bfd_byte,
    mut rva_bias: bfd_vma,
    mut entry: *mut rsrc_entry,
) -> *mut bfd_byte {
    let mut highest_data: *mut bfd_byte = data;
    if table.is_null() {
        return dataend;
    }
    (*table)
        .characteristics = (Some(
        ((*(*abfd).xvec).bfd_getx32).expect("non-null function pointer"),
    ))
        .expect("non-null function pointer")(data as *const libc::c_void)
        as libc::c_uint;
    (*table)
        .time = (Some(((*(*abfd).xvec).bfd_getx32).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(data.offset(4 as libc::c_int as isize) as *const libc::c_void) as libc::c_uint;
    (*table)
        .major = (Some(((*(*abfd).xvec).bfd_getx16).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(data.offset(8 as libc::c_int as isize) as *const libc::c_void) as libc::c_uint;
    (*table)
        .minor = (Some(((*(*abfd).xvec).bfd_getx16).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(data.offset(10 as libc::c_int as isize) as *const libc::c_void)
        as libc::c_uint;
    (*table)
        .names
        .num_entries = (Some(
        ((*(*abfd).xvec).bfd_getx16).expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )(data.offset(12 as libc::c_int as isize) as *const libc::c_void)
        as libc::c_uint;
    (*table)
        .ids
        .num_entries = (Some(
        ((*(*abfd).xvec).bfd_getx16).expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )(data.offset(14 as libc::c_int as isize) as *const libc::c_void)
        as libc::c_uint;
    (*table).entry = entry;
    data = data.offset(16 as libc::c_int as isize);
    highest_data = rsrc_parse_entries(
        abfd,
        &mut (*table).names,
        1 as libc::c_int != 0,
        data,
        datastart,
        data,
        dataend,
        rva_bias,
        table,
    );
    data = data
        .offset(
            ((*table).names.num_entries).wrapping_mul(8 as libc::c_int as libc::c_uint)
                as isize,
        );
    highest_data = rsrc_parse_entries(
        abfd,
        &mut (*table).ids,
        0 as libc::c_int != 0,
        highest_data,
        datastart,
        data,
        dataend,
        rva_bias,
        table,
    );
    data = data
        .offset(
            ((*table).ids.num_entries).wrapping_mul(8 as libc::c_int as libc::c_uint)
                as isize,
        );
    return if highest_data > data { highest_data } else { data };
}
unsafe extern "C" fn rsrc_write_string(
    mut data: *mut rsrc_write_data,
    mut string: *mut rsrc_string,
) {
    (Some(((*(*(*data).abfd).xvec).bfd_putx16).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )((*string).len as bfd_vma, (*data).next_string as *mut libc::c_void);
    memcpy(
        ((*data).next_string).offset(2 as libc::c_int as isize) as *mut libc::c_void,
        (*string).string as *const libc::c_void,
        ((*string).len).wrapping_mul(2 as libc::c_int as libc::c_uint) as libc::c_ulong,
    );
    (*data)
        .next_string = ((*data).next_string)
        .offset(
            ((*string).len)
                .wrapping_add(1 as libc::c_int as libc::c_uint)
                .wrapping_mul(2 as libc::c_int as libc::c_uint) as isize,
        );
}
#[inline]
unsafe extern "C" fn rsrc_compute_rva(
    mut data: *mut rsrc_write_data,
    mut addr: *mut bfd_byte,
) -> libc::c_uint {
    return (addr.offset_from((*data).datastart) as libc::c_long as libc::c_ulong)
        .wrapping_add((*data).rva_bias) as libc::c_uint;
}
unsafe extern "C" fn rsrc_write_leaf(
    mut data: *mut rsrc_write_data,
    mut leaf: *mut rsrc_leaf,
) {
    (Some(((*(*(*data).abfd).xvec).bfd_putx32).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        rsrc_compute_rva(data, (*data).next_data) as bfd_vma,
        (*data).next_leaf as *mut libc::c_void,
    );
    (Some(((*(*(*data).abfd).xvec).bfd_putx32).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (*leaf).size as bfd_vma,
        ((*data).next_leaf).offset(4 as libc::c_int as isize) as *mut libc::c_void,
    );
    (Some(((*(*(*data).abfd).xvec).bfd_putx32).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (*leaf).codepage as bfd_vma,
        ((*data).next_leaf).offset(8 as libc::c_int as isize) as *mut libc::c_void,
    );
    (Some(((*(*(*data).abfd).xvec).bfd_putx32).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        0 as libc::c_int as bfd_vma,
        ((*data).next_leaf).offset(12 as libc::c_int as isize) as *mut libc::c_void,
    );
    (*data).next_leaf = ((*data).next_leaf).offset(16 as libc::c_int as isize);
    memcpy(
        (*data).next_data as *mut libc::c_void,
        (*leaf).data as *const libc::c_void,
        (*leaf).size as libc::c_ulong,
    );
    (*data)
        .next_data = ((*data).next_data)
        .offset(
            (((*leaf).size).wrapping_add(7 as libc::c_int as libc::c_uint)
                & !(7 as libc::c_int) as libc::c_uint) as isize,
        );
}
unsafe extern "C" fn rsrc_write_entry(
    mut data: *mut rsrc_write_data,
    mut where_0: *mut bfd_byte,
    mut entry: *mut rsrc_entry,
) {
    if (*entry).is_name {
        (Some(((*(*(*data).abfd).xvec).bfd_putx32).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            (((*data).next_string).offset_from((*data).datastart) as libc::c_long
                | 0x80000000 as libc::c_uint as libc::c_long) as bfd_vma,
            where_0 as *mut libc::c_void,
        );
        rsrc_write_string(data, &mut (*entry).name_id.name);
    } else {
        (Some(((*(*(*data).abfd).xvec).bfd_putx32).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )((*entry).name_id.id as bfd_vma, where_0 as *mut libc::c_void);
    }
    if (*entry).is_dir {
        (Some(((*(*(*data).abfd).xvec).bfd_putx32).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            (((*data).next_table).offset_from((*data).datastart) as libc::c_long
                | 0x80000000 as libc::c_uint as libc::c_long) as bfd_vma,
            where_0.offset(4 as libc::c_int as isize) as *mut libc::c_void,
        );
        rsrc_write_directory(data, (*entry).value.directory);
    } else {
        (Some(((*(*(*data).abfd).xvec).bfd_putx32).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            ((*data).next_leaf).offset_from((*data).datastart) as libc::c_long
                as bfd_vma,
            where_0.offset(4 as libc::c_int as isize) as *mut libc::c_void,
        );
        rsrc_write_leaf(data, (*entry).value.leaf);
    };
}
unsafe extern "C" fn rsrc_compute_region_sizes(mut dir: *mut rsrc_directory) {
    let mut entry: *mut rsrc_entry = 0 as *mut rsrc_entry;
    if dir.is_null() {
        return;
    }
    sizeof_tables_and_entries = sizeof_tables_and_entries
        .wrapping_add(16 as libc::c_int as libc::c_uint);
    entry = (*dir).names.first_entry;
    while !entry.is_null() {
        sizeof_tables_and_entries = sizeof_tables_and_entries
            .wrapping_add(8 as libc::c_int as libc::c_uint);
        sizeof_strings = sizeof_strings
            .wrapping_add(
                ((*entry).name_id.name.len)
                    .wrapping_add(1 as libc::c_int as libc::c_uint)
                    .wrapping_mul(2 as libc::c_int as libc::c_uint),
            );
        if (*entry).is_dir {
            rsrc_compute_region_sizes((*entry).value.directory);
        } else {
            sizeof_leaves = sizeof_leaves
                .wrapping_add(16 as libc::c_int as libc::c_uint);
        }
        entry = (*entry).next_entry;
    }
    entry = (*dir).ids.first_entry;
    while !entry.is_null() {
        sizeof_tables_and_entries = sizeof_tables_and_entries
            .wrapping_add(8 as libc::c_int as libc::c_uint);
        if (*entry).is_dir {
            rsrc_compute_region_sizes((*entry).value.directory);
        } else {
            sizeof_leaves = sizeof_leaves
                .wrapping_add(16 as libc::c_int as libc::c_uint);
        }
        entry = (*entry).next_entry;
    }
}
unsafe extern "C" fn rsrc_write_directory(
    mut data: *mut rsrc_write_data,
    mut dir: *mut rsrc_directory,
) {
    let mut entry: *mut rsrc_entry = 0 as *mut rsrc_entry;
    let mut i: libc::c_uint = 0;
    let mut next_entry: *mut bfd_byte = 0 as *mut bfd_byte;
    let mut nt: *mut bfd_byte = 0 as *mut bfd_byte;
    (Some(((*(*(*data).abfd).xvec).bfd_putx32).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )((*dir).characteristics as bfd_vma, (*data).next_table as *mut libc::c_void);
    (Some(((*(*(*data).abfd).xvec).bfd_putx32).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        0 as libc::c_int as bfd_vma,
        ((*data).next_table).offset(4 as libc::c_int as isize) as *mut libc::c_void,
    );
    (Some(((*(*(*data).abfd).xvec).bfd_putx16).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (*dir).major as bfd_vma,
        ((*data).next_table).offset(8 as libc::c_int as isize) as *mut libc::c_void,
    );
    (Some(((*(*(*data).abfd).xvec).bfd_putx16).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (*dir).minor as bfd_vma,
        ((*data).next_table).offset(10 as libc::c_int as isize) as *mut libc::c_void,
    );
    (Some(((*(*(*data).abfd).xvec).bfd_putx16).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (*dir).names.num_entries as bfd_vma,
        ((*data).next_table).offset(12 as libc::c_int as isize) as *mut libc::c_void,
    );
    (Some(((*(*(*data).abfd).xvec).bfd_putx16).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (*dir).ids.num_entries as bfd_vma,
        ((*data).next_table).offset(14 as libc::c_int as isize) as *mut libc::c_void,
    );
    next_entry = ((*data).next_table).offset(16 as libc::c_int as isize);
    (*data)
        .next_table = next_entry
        .offset(
            ((*dir).names.num_entries).wrapping_mul(8 as libc::c_int as libc::c_uint)
                as isize,
        )
        .offset(
            ((*dir).ids.num_entries).wrapping_mul(8 as libc::c_int as libc::c_uint)
                as isize,
        );
    nt = (*data).next_table;
    i = (*dir).names.num_entries;
    entry = (*dir).names.first_entry;
    while i > 0 as libc::c_int as libc::c_uint && !entry.is_null() {
        if !(*entry).is_name {
            bfd_assert(
                b"peXXigen.c\0" as *const u8 as *const libc::c_char,
                3558 as libc::c_int,
            );
        }
        rsrc_write_entry(data, next_entry, entry);
        next_entry = next_entry.offset(8 as libc::c_int as isize);
        i = i.wrapping_sub(1);
        i;
        entry = (*entry).next_entry;
    }
    if !(i == 0 as libc::c_int as libc::c_uint) {
        bfd_assert(
            b"peXXigen.c\0" as *const u8 as *const libc::c_char,
            3562 as libc::c_int,
        );
    }
    if !entry.is_null() {
        bfd_assert(
            b"peXXigen.c\0" as *const u8 as *const libc::c_char,
            3563 as libc::c_int,
        );
    }
    i = (*dir).ids.num_entries;
    entry = (*dir).ids.first_entry;
    while i > 0 as libc::c_int as libc::c_uint && !entry.is_null() {
        if (*entry).is_name {
            bfd_assert(
                b"peXXigen.c\0" as *const u8 as *const libc::c_char,
                3569 as libc::c_int,
            );
        }
        rsrc_write_entry(data, next_entry, entry);
        next_entry = next_entry.offset(8 as libc::c_int as isize);
        i = i.wrapping_sub(1);
        i;
        entry = (*entry).next_entry;
    }
    if !(i == 0 as libc::c_int as libc::c_uint) {
        bfd_assert(
            b"peXXigen.c\0" as *const u8 as *const libc::c_char,
            3573 as libc::c_int,
        );
    }
    if !entry.is_null() {
        bfd_assert(
            b"peXXigen.c\0" as *const u8 as *const libc::c_char,
            3574 as libc::c_int,
        );
    }
    if !(nt == next_entry) {
        bfd_assert(
            b"peXXigen.c\0" as *const u8 as *const libc::c_char,
            3575 as libc::c_int,
        );
    }
}
unsafe extern "C" fn u16_mbtouc(
    mut puc: *mut wint_t,
    mut s: *const libc::c_ushort,
    mut n: libc::c_uint,
) -> libc::c_uint {
    let mut c: libc::c_ushort = *s;
    if (c as libc::c_int) < 0xd800 as libc::c_int
        || c as libc::c_int >= 0xe000 as libc::c_int
    {
        *puc = c as wint_t;
        return 1 as libc::c_int as libc::c_uint;
    }
    if (c as libc::c_int) < 0xdc00 as libc::c_int {
        if n >= 2 as libc::c_int as libc::c_uint {
            if *s.offset(1 as libc::c_int as isize) as libc::c_int
                >= 0xdc00 as libc::c_int
                && (*s.offset(1 as libc::c_int as isize) as libc::c_int)
                    < 0xe000 as libc::c_int
            {
                *puc = (0x10000 as libc::c_int
                    + ((c as libc::c_int - 0xd800 as libc::c_int) << 10 as libc::c_int)
                    + (*s.offset(1 as libc::c_int as isize) as libc::c_int
                        - 0xdc00 as libc::c_int)) as wint_t;
                return 2 as libc::c_int as libc::c_uint;
            }
        } else {
            *puc = 0xfffd as libc::c_int as wint_t;
            return n;
        }
    }
    *puc = 0xfffd as libc::c_int as wint_t;
    return 1 as libc::c_int as libc::c_uint;
}
unsafe extern "C" fn rsrc_cmp(
    mut is_name: bool,
    mut a: *mut rsrc_entry,
    mut b: *mut rsrc_entry,
) -> libc::c_int {
    let mut res: libc::c_int = 0;
    let mut astring: *mut bfd_byte = 0 as *mut bfd_byte;
    let mut alen: libc::c_uint = 0;
    let mut bstring: *mut bfd_byte = 0 as *mut bfd_byte;
    let mut blen: libc::c_uint = 0;
    if !is_name {
        return ((*a).name_id.id).wrapping_sub((*b).name_id.id) as libc::c_int;
    }
    astring = (*a).name_id.name.string;
    alen = (*a).name_id.name.len;
    bstring = (*b).name_id.name.string;
    blen = (*b).name_id.name.len;
    let mut i: libc::c_uint = 0;
    res = 0 as libc::c_int;
    i = if alen < blen { alen } else { blen };
    loop {
        let fresh6 = i;
        i = i.wrapping_sub(1);
        if !(fresh6 != 0) {
            break;
        }
        let mut awc: wint_t = 0;
        let mut bwc: wint_t = 0;
        let mut Alen: libc::c_uint = u16_mbtouc(
            &mut awc,
            astring as *const libc::c_ushort,
            2 as libc::c_int as libc::c_uint,
        );
        let mut Blen: libc::c_uint = u16_mbtouc(
            &mut bwc,
            bstring as *const libc::c_ushort,
            2 as libc::c_int as libc::c_uint,
        );
        if Alen != Blen {
            return Alen.wrapping_sub(Blen) as libc::c_int;
        }
        awc = towlower(awc);
        bwc = towlower(bwc);
        res = awc.wrapping_sub(bwc) as libc::c_int;
        if res != 0 {
            break;
        }
        astring = astring.offset(2 as libc::c_int as isize);
        bstring = bstring.offset(2 as libc::c_int as isize);
    }
    if res == 0 as libc::c_int {
        res = alen.wrapping_sub(blen) as libc::c_int;
    }
    return res;
}
unsafe extern "C" fn rsrc_print_name(
    mut buffer: *mut libc::c_char,
    mut string: rsrc_string,
) {
    let mut i: libc::c_uint = 0;
    let mut name: *mut bfd_byte = string.string;
    i = string.len;
    loop {
        let fresh7 = i;
        i = i.wrapping_sub(1);
        if !(fresh7 != 0) {
            break;
        }
        sprintf(
            buffer.offset(strlen(buffer) as isize),
            b"%.1s\0" as *const u8 as *const libc::c_char,
            name,
        );
        name = name.offset(2 as libc::c_int as isize);
    };
}
unsafe extern "C" fn rsrc_resource_name(
    mut entry: *mut rsrc_entry,
    mut dir: *mut rsrc_directory,
    mut buffer: *mut libc::c_char,
) -> *const libc::c_char {
    let mut is_string: bool = 0 as libc::c_int != 0;
    *buffer.offset(0 as libc::c_int as isize) = 0 as libc::c_int as libc::c_char;
    if !dir.is_null() && !((*dir).entry).is_null() && !((*(*dir).entry).parent).is_null()
        && !((*(*(*dir).entry).parent).entry).is_null()
    {
        strcpy(buffer, b"type: \0" as *const u8 as *const libc::c_char);
        if (*(*(*(*dir).entry).parent).entry).is_name {
            rsrc_print_name(
                buffer.offset(strlen(buffer) as isize),
                (*(*(*(*dir).entry).parent).entry).name_id.name,
            );
        } else {
            let mut id: libc::c_uint = (*(*(*(*dir).entry).parent).entry).name_id.id;
            sprintf(
                buffer.offset(strlen(buffer) as isize),
                b"%x\0" as *const u8 as *const libc::c_char,
                id,
            );
            match id {
                1 => {
                    strcat(buffer, b" (CURSOR)\0" as *const u8 as *const libc::c_char);
                }
                2 => {
                    strcat(buffer, b" (BITMAP)\0" as *const u8 as *const libc::c_char);
                }
                3 => {
                    strcat(buffer, b" (ICON)\0" as *const u8 as *const libc::c_char);
                }
                4 => {
                    strcat(buffer, b" (MENU)\0" as *const u8 as *const libc::c_char);
                }
                5 => {
                    strcat(buffer, b" (DIALOG)\0" as *const u8 as *const libc::c_char);
                }
                6 => {
                    strcat(buffer, b" (STRING)\0" as *const u8 as *const libc::c_char);
                    is_string = 1 as libc::c_int != 0;
                }
                7 => {
                    strcat(buffer, b" (FONTDIR)\0" as *const u8 as *const libc::c_char);
                }
                8 => {
                    strcat(buffer, b" (FONT)\0" as *const u8 as *const libc::c_char);
                }
                9 => {
                    strcat(
                        buffer,
                        b" (ACCELERATOR)\0" as *const u8 as *const libc::c_char,
                    );
                }
                10 => {
                    strcat(buffer, b" (RCDATA)\0" as *const u8 as *const libc::c_char);
                }
                11 => {
                    strcat(
                        buffer,
                        b" (MESSAGETABLE)\0" as *const u8 as *const libc::c_char,
                    );
                }
                12 => {
                    strcat(
                        buffer,
                        b" (GROUP_CURSOR)\0" as *const u8 as *const libc::c_char,
                    );
                }
                14 => {
                    strcat(
                        buffer,
                        b" (GROUP_ICON)\0" as *const u8 as *const libc::c_char,
                    );
                }
                16 => {
                    strcat(buffer, b" (VERSION)\0" as *const u8 as *const libc::c_char);
                }
                17 => {
                    strcat(
                        buffer,
                        b" (DLGINCLUDE)\0" as *const u8 as *const libc::c_char,
                    );
                }
                19 => {
                    strcat(buffer, b" (PLUGPLAY)\0" as *const u8 as *const libc::c_char);
                }
                20 => {
                    strcat(buffer, b" (VXD)\0" as *const u8 as *const libc::c_char);
                }
                21 => {
                    strcat(
                        buffer,
                        b" (ANICURSOR)\0" as *const u8 as *const libc::c_char,
                    );
                }
                22 => {
                    strcat(buffer, b" (ANIICON)\0" as *const u8 as *const libc::c_char);
                }
                23 => {
                    strcat(buffer, b" (HTML)\0" as *const u8 as *const libc::c_char);
                }
                24 => {
                    strcat(buffer, b" (MANIFEST)\0" as *const u8 as *const libc::c_char);
                }
                240 => {
                    strcat(buffer, b" (DLGINIT)\0" as *const u8 as *const libc::c_char);
                }
                241 => {
                    strcat(buffer, b" (TOOLBAR)\0" as *const u8 as *const libc::c_char);
                }
                _ => {}
            }
        }
    }
    if !dir.is_null() && !((*dir).entry).is_null() {
        strcat(buffer, b" name: \0" as *const u8 as *const libc::c_char);
        if (*(*dir).entry).is_name {
            rsrc_print_name(
                buffer.offset(strlen(buffer) as isize),
                (*(*dir).entry).name_id.name,
            );
        } else {
            let mut id_0: libc::c_uint = (*(*dir).entry).name_id.id;
            sprintf(
                buffer.offset(strlen(buffer) as isize),
                b"%x\0" as *const u8 as *const libc::c_char,
                id_0,
            );
            if is_string {
                sprintf(
                    buffer.offset(strlen(buffer) as isize),
                    b" (resource id range: %d - %d)\0" as *const u8
                        as *const libc::c_char,
                    id_0.wrapping_sub(1 as libc::c_int as libc::c_uint)
                        << 4 as libc::c_int,
                    (id_0 << 4 as libc::c_int)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint),
                );
            }
        }
    }
    if !entry.is_null() {
        strcat(buffer, b" lang: \0" as *const u8 as *const libc::c_char);
        if (*entry).is_name {
            rsrc_print_name(
                buffer.offset(strlen(buffer) as isize),
                (*entry).name_id.name,
            );
        } else {
            sprintf(
                buffer.offset(strlen(buffer) as isize),
                b"%x\0" as *const u8 as *const libc::c_char,
                (*entry).name_id.id,
            );
        }
    }
    return buffer;
}
unsafe extern "C" fn rsrc_merge_string_entries(
    mut a: *mut rsrc_entry,
    mut b: *mut rsrc_entry,
) -> bool {
    let mut copy_needed: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut i: libc::c_uint = 0;
    let mut astring: *mut bfd_byte = 0 as *mut bfd_byte;
    let mut bstring: *mut bfd_byte = 0 as *mut bfd_byte;
    let mut new_data: *mut bfd_byte = 0 as *mut bfd_byte;
    let mut nstring: *mut bfd_byte = 0 as *mut bfd_byte;
    if (*a).is_dir {
        bfd_assert(
            b"peXXigen.c\0" as *const u8 as *const libc::c_char,
            3798 as libc::c_int,
        );
    }
    astring = (*(*a).value.leaf).data;
    if (*b).is_dir {
        bfd_assert(
            b"peXXigen.c\0" as *const u8 as *const libc::c_char,
            3801 as libc::c_int,
        );
    }
    bstring = (*(*b).value.leaf).data;
    i = 0 as libc::c_int as libc::c_uint;
    while i < 16 as libc::c_int as libc::c_uint {
        let mut alen: libc::c_uint = (*astring.offset(0 as libc::c_int as isize)
            as libc::c_int
            + ((*astring.offset(1 as libc::c_int as isize) as libc::c_int)
                << 8 as libc::c_int)) as libc::c_uint;
        let mut blen: libc::c_uint = (*bstring.offset(0 as libc::c_int as isize)
            as libc::c_int
            + ((*bstring.offset(1 as libc::c_int as isize) as libc::c_int)
                << 8 as libc::c_int)) as libc::c_uint;
        if alen == 0 as libc::c_int as libc::c_uint {
            copy_needed = copy_needed
                .wrapping_add(blen.wrapping_mul(2 as libc::c_int as libc::c_uint));
        } else if !(blen == 0 as libc::c_int as libc::c_uint) {
            if alen != blen {
                break;
            }
            if memcmp(
                astring.offset(2 as libc::c_int as isize) as *const libc::c_void,
                bstring.offset(2 as libc::c_int as isize) as *const libc::c_void,
                alen.wrapping_mul(2 as libc::c_int as libc::c_uint) as libc::c_ulong,
            ) != 0 as libc::c_int
            {
                break;
            }
        }
        astring = astring
            .offset(
                alen
                    .wrapping_add(1 as libc::c_int as libc::c_uint)
                    .wrapping_mul(2 as libc::c_int as libc::c_uint) as isize,
            );
        bstring = bstring
            .offset(
                blen
                    .wrapping_add(1 as libc::c_int as libc::c_uint)
                    .wrapping_mul(2 as libc::c_int as libc::c_uint) as isize,
            );
        i = i.wrapping_add(1);
        i;
    }
    if i != 16 as libc::c_int as libc::c_uint {
        if !((*a).parent).is_null() && !((*(*a).parent).entry).is_null()
            && !(*(*(*a).parent).entry).is_name
        {
            _bfd_error_handler(
                dcgettext(
                    b"bfd\0" as *const u8 as *const libc::c_char,
                    b".rsrc merge failure: duplicate string resource: %d\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (((*(*(*a).parent).entry).name_id.id)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) << 4 as libc::c_int)
                    .wrapping_add(i),
            );
        }
        return 0 as libc::c_int != 0;
    }
    if copy_needed == 0 as libc::c_int as libc::c_uint {
        return 1 as libc::c_int != 0;
    }
    new_data = bfd_malloc(
        ((*(*a).value.leaf).size).wrapping_add(copy_needed) as bfd_size_type,
    ) as *mut bfd_byte;
    if new_data.is_null() {
        return 0 as libc::c_int != 0;
    }
    nstring = new_data;
    astring = (*(*a).value.leaf).data;
    bstring = (*(*b).value.leaf).data;
    i = 0 as libc::c_int as libc::c_uint;
    while i < 16 as libc::c_int as libc::c_uint {
        let mut alen_0: libc::c_uint = (*astring.offset(0 as libc::c_int as isize)
            as libc::c_int
            + ((*astring.offset(1 as libc::c_int as isize) as libc::c_int)
                << 8 as libc::c_int)) as libc::c_uint;
        let mut blen_0: libc::c_uint = (*bstring.offset(0 as libc::c_int as isize)
            as libc::c_int
            + ((*bstring.offset(1 as libc::c_int as isize) as libc::c_int)
                << 8 as libc::c_int)) as libc::c_uint;
        if alen_0 != 0 as libc::c_int as libc::c_uint {
            memcpy(
                nstring as *mut libc::c_void,
                astring as *const libc::c_void,
                alen_0
                    .wrapping_add(1 as libc::c_int as libc::c_uint)
                    .wrapping_mul(2 as libc::c_int as libc::c_uint) as libc::c_ulong,
            );
            nstring = nstring
                .offset(
                    alen_0
                        .wrapping_add(1 as libc::c_int as libc::c_uint)
                        .wrapping_mul(2 as libc::c_int as libc::c_uint) as isize,
                );
        } else if blen_0 != 0 as libc::c_int as libc::c_uint {
            memcpy(
                nstring as *mut libc::c_void,
                bstring as *const libc::c_void,
                blen_0
                    .wrapping_add(1 as libc::c_int as libc::c_uint)
                    .wrapping_mul(2 as libc::c_int as libc::c_uint) as libc::c_ulong,
            );
            nstring = nstring
                .offset(
                    blen_0
                        .wrapping_add(1 as libc::c_int as libc::c_uint)
                        .wrapping_mul(2 as libc::c_int as libc::c_uint) as isize,
                );
        } else {
            let fresh8 = nstring;
            nstring = nstring.offset(1);
            *fresh8 = 0 as libc::c_int as bfd_byte;
            let fresh9 = nstring;
            nstring = nstring.offset(1);
            *fresh9 = 0 as libc::c_int as bfd_byte;
        }
        astring = astring
            .offset(
                alen_0
                    .wrapping_add(1 as libc::c_int as libc::c_uint)
                    .wrapping_mul(2 as libc::c_int as libc::c_uint) as isize,
            );
        bstring = bstring
            .offset(
                blen_0
                    .wrapping_add(1 as libc::c_int as libc::c_uint)
                    .wrapping_mul(2 as libc::c_int as libc::c_uint) as isize,
            );
        i = i.wrapping_add(1);
        i;
    }
    if !(nstring.offset_from(new_data) as libc::c_long
        == ((*(*a).value.leaf).size).wrapping_add(copy_needed) as libc::c_int
            as libc::c_long)
    {
        bfd_assert(
            b"peXXigen.c\0" as *const u8 as *const libc::c_char,
            3878 as libc::c_int,
        );
    }
    free((*(*a).value.leaf).data as *mut libc::c_void);
    (*(*a).value.leaf).data = new_data;
    (*(*a).value.leaf).size = ((*(*a).value.leaf).size).wrapping_add(copy_needed);
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn rsrc_sort_entries(
    mut chain: *mut rsrc_dir_chain,
    mut is_name: bool,
    mut dir: *mut rsrc_directory,
) {
    let mut entry: *mut rsrc_entry = 0 as *mut rsrc_entry;
    let mut next: *mut rsrc_entry = 0 as *mut rsrc_entry;
    let mut points_to_entry: *mut *mut rsrc_entry = 0 as *mut *mut rsrc_entry;
    let mut swapped: bool = false;
    if (*chain).num_entries < 2 as libc::c_int as libc::c_uint {
        return;
    }
    loop {
        swapped = 0 as libc::c_int != 0;
        points_to_entry = &mut (*chain).first_entry;
        entry = *points_to_entry;
        next = (*entry).next_entry;
        loop {
            let mut cmp: libc::c_int = rsrc_cmp(is_name, entry, next);
            if cmp > 0 as libc::c_int {
                (*entry).next_entry = (*next).next_entry;
                (*next).next_entry = entry;
                *points_to_entry = next;
                points_to_entry = &mut (*next).next_entry;
                next = (*entry).next_entry;
                swapped = 1 as libc::c_int != 0;
            } else if cmp == 0 as libc::c_int {
                if (*entry).is_dir as libc::c_int != 0
                    && (*next).is_dir as libc::c_int != 0
                {
                    if !(*entry).is_name
                        && (*entry).name_id.id == 1 as libc::c_int as libc::c_uint
                        && !dir.is_null() && !((*dir).entry).is_null()
                        && !(*(*dir).entry).is_name
                        && (*(*dir).entry).name_id.id
                            == 0x18 as libc::c_int as libc::c_uint
                    {
                        if !((*(*next).value.directory).names.num_entries
                            == 0 as libc::c_int as libc::c_uint
                            && (*(*next).value.directory).ids.num_entries
                                == 1 as libc::c_int as libc::c_uint
                            && !(*(*(*next).value.directory).ids.first_entry).is_name
                            && (*(*(*next).value.directory).ids.first_entry).name_id.id
                                == 0 as libc::c_int as libc::c_uint)
                        {
                            if (*(*entry).value.directory).names.num_entries
                                == 0 as libc::c_int as libc::c_uint
                                && (*(*entry).value.directory).ids.num_entries
                                    == 1 as libc::c_int as libc::c_uint
                                && !(*(*(*entry).value.directory).ids.first_entry).is_name
                                && (*(*(*entry).value.directory).ids.first_entry).name_id.id
                                    == 0 as libc::c_int as libc::c_uint
                            {
                                (*entry).next_entry = (*next).next_entry;
                                (*next).next_entry = entry;
                                *points_to_entry = next;
                                points_to_entry = &mut (*next).next_entry;
                                next = (*entry).next_entry;
                                swapped = 1 as libc::c_int != 0;
                            } else {
                                _bfd_error_handler(
                                    dcgettext(
                                        b"bfd\0" as *const u8 as *const libc::c_char,
                                        b".rsrc merge failure: multiple non-default manifests\0"
                                            as *const u8 as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                                bfd_set_error(bfd_error_file_truncated);
                                return;
                            }
                        }
                        (*entry).next_entry = (*next).next_entry;
                        (*chain).num_entries = ((*chain).num_entries).wrapping_sub(1);
                        (*chain).num_entries;
                        if (*chain).num_entries < 2 as libc::c_int as libc::c_uint {
                            return;
                        }
                        next = (*next).next_entry;
                    } else {
                        rsrc_merge(entry, next);
                    }
                } else if (*entry).is_dir as libc::c_int != (*next).is_dir as libc::c_int
                {
                    _bfd_error_handler(
                        dcgettext(
                            b"bfd\0" as *const u8 as *const libc::c_char,
                            b".rsrc merge failure: a directory matches a leaf\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    bfd_set_error(bfd_error_file_truncated);
                    return;
                } else if !(!(*entry).is_name
                    && (*entry).name_id.id == 0 as libc::c_int as libc::c_uint
                    && !dir.is_null() && !((*dir).entry).is_null()
                    && !(*(*dir).entry).is_name
                    && (*(*dir).entry).name_id.id == 1 as libc::c_int as libc::c_uint
                    && !((*(*dir).entry).parent).is_null()
                    && !((*(*(*dir).entry).parent).entry).is_null()
                    && !(*(*(*(*dir).entry).parent).entry).is_name
                    && (*(*(*(*dir).entry).parent).entry).name_id.id
                        == 0x18 as libc::c_int as libc::c_uint)
                {
                    if !dir.is_null() && !((*dir).entry).is_null()
                        && !((*(*dir).entry).parent).is_null()
                        && !((*(*(*dir).entry).parent).entry).is_null()
                        && !(*(*(*(*dir).entry).parent).entry).is_name
                        && (*(*(*(*dir).entry).parent).entry).name_id.id
                            == 0x6 as libc::c_int as libc::c_uint
                    {
                        if !rsrc_merge_string_entries(entry, next) {
                            bfd_set_error(bfd_error_file_truncated);
                            return;
                        }
                    } else {
                        if dir.is_null() || ((*dir).entry).is_null()
                            || ((*(*dir).entry).parent).is_null()
                            || ((*(*(*dir).entry).parent).entry).is_null()
                        {
                            _bfd_error_handler(
                                dcgettext(
                                    b"bfd\0" as *const u8 as *const libc::c_char,
                                    b".rsrc merge failure: duplicate leaf\0" as *const u8
                                        as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                            );
                        } else {
                            let mut buff: [libc::c_char; 256] = [0; 256];
                            _bfd_error_handler(
                                dcgettext(
                                    b"bfd\0" as *const u8 as *const libc::c_char,
                                    b".rsrc merge failure: duplicate leaf: %s\0" as *const u8
                                        as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                                rsrc_resource_name(entry, dir, buff.as_mut_ptr()),
                            );
                        }
                        bfd_set_error(bfd_error_file_truncated);
                        return;
                    }
                }
                (*entry).next_entry = (*next).next_entry;
                (*chain).num_entries = ((*chain).num_entries).wrapping_sub(1);
                (*chain).num_entries;
                if (*chain).num_entries < 2 as libc::c_int as libc::c_uint {
                    return;
                }
                next = (*next).next_entry;
            } else {
                points_to_entry = &mut (*entry).next_entry;
                entry = next;
                next = (*next).next_entry;
            }
            if next.is_null() {
                break;
            }
        }
        (*chain).last_entry = entry;
        if !swapped {
            break;
        }
    };
}
unsafe extern "C" fn rsrc_attach_chain(
    mut achain: *mut rsrc_dir_chain,
    mut bchain: *mut rsrc_dir_chain,
) {
    if (*bchain).num_entries == 0 as libc::c_int as libc::c_uint {
        return;
    }
    (*achain).num_entries = ((*achain).num_entries).wrapping_add((*bchain).num_entries);
    if ((*achain).first_entry).is_null() {
        (*achain).first_entry = (*bchain).first_entry;
        (*achain).last_entry = (*bchain).last_entry;
    } else {
        (*(*achain).last_entry).next_entry = (*bchain).first_entry;
        (*achain).last_entry = (*bchain).last_entry;
    }
    (*bchain).num_entries = 0 as libc::c_int as libc::c_uint;
    (*bchain).last_entry = 0 as *mut rsrc_entry;
    (*bchain).first_entry = (*bchain).last_entry;
}
unsafe extern "C" fn rsrc_merge(mut a: *mut rsrc_entry, mut b: *mut rsrc_entry) {
    let mut adir: *mut rsrc_directory = 0 as *mut rsrc_directory;
    let mut bdir: *mut rsrc_directory = 0 as *mut rsrc_directory;
    if !(*a).is_dir {
        bfd_assert(
            b"peXXigen.c\0" as *const u8 as *const libc::c_char,
            4092 as libc::c_int,
        );
    }
    if !(*b).is_dir {
        bfd_assert(
            b"peXXigen.c\0" as *const u8 as *const libc::c_char,
            4093 as libc::c_int,
        );
    }
    adir = (*a).value.directory;
    bdir = (*b).value.directory;
    if (*adir).characteristics != (*bdir).characteristics {
        _bfd_error_handler(
            dcgettext(
                b"bfd\0" as *const u8 as *const libc::c_char,
                b".rsrc merge failure: dirs with differing characteristics\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        bfd_set_error(bfd_error_file_truncated);
        return;
    }
    if (*adir).major != (*bdir).major || (*adir).minor != (*bdir).minor {
        _bfd_error_handler(
            dcgettext(
                b"bfd\0" as *const u8 as *const libc::c_char,
                b".rsrc merge failure: differing directory versions\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        bfd_set_error(bfd_error_file_truncated);
        return;
    }
    rsrc_attach_chain(&mut (*adir).names, &mut (*bdir).names);
    rsrc_attach_chain(&mut (*adir).ids, &mut (*bdir).ids);
    rsrc_sort_entries(&mut (*adir).names, 1 as libc::c_int != 0, adir);
    rsrc_sort_entries(&mut (*adir).ids, 0 as libc::c_int != 0, adir);
}
unsafe extern "C" fn rsrc_process_section(
    mut abfd: *mut bfd,
    mut pfinfo: *mut coff_final_link_info,
) {
    let mut current_block: u64;
    let mut new_table: rsrc_directory = rsrc_directory {
        characteristics: 0,
        time: 0,
        major: 0,
        minor: 0,
        names: rsrc_dir_chain {
            num_entries: 0,
            first_entry: 0 as *mut rsrc_entry,
            last_entry: 0 as *mut rsrc_entry,
        },
        ids: rsrc_dir_chain {
            num_entries: 0,
            first_entry: 0 as *mut rsrc_entry,
            last_entry: 0 as *mut rsrc_entry,
        },
        entry: 0 as *mut rsrc_entry,
    };
    let mut size: bfd_size_type = 0;
    let mut sec: *mut asection = 0 as *mut asection;
    let mut pe: *mut pe_data_type = 0 as *mut pe_data_type;
    let mut rva_bias: bfd_vma = 0;
    let mut data: *mut bfd_byte = 0 as *mut bfd_byte;
    let mut datastart: *mut bfd_byte = 0 as *mut bfd_byte;
    let mut dataend: *mut bfd_byte = 0 as *mut bfd_byte;
    let mut new_data: *mut bfd_byte = 0 as *mut bfd_byte;
    let mut num_resource_sets: libc::c_uint = 0;
    let mut type_tables: *mut rsrc_directory = 0 as *mut rsrc_directory;
    let mut write_data: rsrc_write_data = rsrc_write_data {
        abfd: 0 as *mut bfd,
        datastart: 0 as *mut bfd_byte,
        next_table: 0 as *mut bfd_byte,
        next_leaf: 0 as *mut bfd_byte,
        next_string: 0 as *mut bfd_byte,
        next_data: 0 as *mut bfd_byte,
        rva_bias: 0,
    };
    let mut indx: libc::c_uint = 0;
    let mut input: *mut bfd = 0 as *mut bfd;
    let mut num_input_rsrc: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut max_num_input_rsrc: libc::c_uint = 4 as libc::c_int as libc::c_uint;
    let mut rsrc_sizes: *mut ptrdiff_t = 0 as *mut ptrdiff_t;
    new_table.names.num_entries = 0 as libc::c_int as libc::c_uint;
    new_table.ids.num_entries = 0 as libc::c_int as libc::c_uint;
    sec = bfd_get_section_by_name(abfd, b".rsrc\0" as *const u8 as *const libc::c_char);
    if sec.is_null()
        || {
            size = (*sec).rawsize;
            size == 0 as libc::c_int as libc::c_ulong
        }
    {
        return;
    }
    pe = (*abfd).tdata.pe_obj_data;
    if pe.is_null() {
        return;
    }
    rva_bias = ((*sec).vma).wrapping_sub((*pe).pe_opthdr.ImageBase);
    data = bfd_malloc(size) as *mut bfd_byte;
    if data.is_null() {
        return;
    }
    datastart = data;
    if bfd_get_section_contents(
        abfd,
        sec,
        data as *mut libc::c_void,
        0 as libc::c_int as file_ptr,
        size,
    ) {
        rsrc_sizes = bfd_malloc(
            (max_num_input_rsrc as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<ptrdiff_t>() as libc::c_ulong),
        ) as *mut ptrdiff_t;
        if !rsrc_sizes.is_null() {
            input = (*(*pfinfo).info).input_bfds;
            loop {
                if input.is_null() {
                    current_block = 5689316957504528238;
                    break;
                }
                let mut rsrc_sec: *mut asection = bfd_get_section_by_name(
                    input,
                    b".rsrc\0" as *const u8 as *const libc::c_char,
                );
                if !rsrc_sec.is_null() && !discarded_section(rsrc_sec) {
                    if num_input_rsrc == max_num_input_rsrc {
                        max_num_input_rsrc = max_num_input_rsrc
                            .wrapping_add(10 as libc::c_int as libc::c_uint);
                        rsrc_sizes = bfd_realloc(
                            rsrc_sizes as *mut libc::c_void,
                            (max_num_input_rsrc as libc::c_ulong)
                                .wrapping_mul(
                                    ::core::mem::size_of::<ptrdiff_t>() as libc::c_ulong,
                                ),
                        ) as *mut ptrdiff_t;
                        if rsrc_sizes.is_null() {
                            current_block = 2358326333830668383;
                            break;
                        }
                    }
                    if !((*rsrc_sec).size > 0 as libc::c_int as libc::c_ulong) {
                        bfd_assert(
                            b"peXXigen.c\0" as *const u8 as *const libc::c_char,
                            4202 as libc::c_int,
                        );
                    }
                    let fresh10 = num_input_rsrc;
                    num_input_rsrc = num_input_rsrc.wrapping_add(1);
                    *rsrc_sizes.offset(fresh10 as isize) = (*rsrc_sec).size as ptrdiff_t;
                }
                input = (*input).link.next;
            }
            match current_block {
                2358326333830668383 => {}
                _ => {
                    if !(num_input_rsrc < 2 as libc::c_int as libc::c_uint) {
                        dataend = data.offset(size as isize);
                        num_resource_sets = 0 as libc::c_int as libc::c_uint;
                        loop {
                            if !(data < dataend) {
                                current_block = 18435049525520518667;
                                break;
                            }
                            let mut p: *mut bfd_byte = data;
                            data = rsrc_count_directory(
                                abfd,
                                data,
                                data,
                                dataend,
                                rva_bias,
                            );
                            if data > dataend {
                                _bfd_error_handler(
                                    dcgettext(
                                        b"bfd\0" as *const u8 as *const libc::c_char,
                                        b"%pB: .rsrc merge failure: corrupt .rsrc section\0"
                                            as *const u8 as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                    abfd,
                                );
                                bfd_set_error(bfd_error_file_truncated);
                                current_block = 2358326333830668383;
                                break;
                            } else if data.offset_from(p) as libc::c_long
                                > *rsrc_sizes.offset(num_resource_sets as isize)
                            {
                                _bfd_error_handler(
                                    dcgettext(
                                        b"bfd\0" as *const u8 as *const libc::c_char,
                                        b"%pB: .rsrc merge failure: unexpected .rsrc size\0"
                                            as *const u8 as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                    abfd,
                                );
                                bfd_set_error(bfd_error_file_truncated);
                                current_block = 2358326333830668383;
                                break;
                            } else {
                                data = p
                                    .offset(
                                        *rsrc_sizes.offset(num_resource_sets as isize) as isize,
                                    );
                                rva_bias = (rva_bias as libc::c_ulong)
                                    .wrapping_add(
                                        data.offset_from(p) as libc::c_long as libc::c_ulong,
                                    ) as bfd_vma as bfd_vma;
                                num_resource_sets = num_resource_sets.wrapping_add(1);
                                num_resource_sets;
                            }
                        }
                        match current_block {
                            2358326333830668383 => {}
                            _ => {
                                if !(num_resource_sets == num_input_rsrc) {
                                    bfd_assert(
                                        b"peXXigen.c\0" as *const u8 as *const libc::c_char,
                                        4244 as libc::c_int,
                                    );
                                }
                                data = datastart;
                                rva_bias = ((*sec).vma)
                                    .wrapping_sub((*pe).pe_opthdr.ImageBase);
                                type_tables = bfd_malloc(
                                    (num_resource_sets as libc::c_ulong)
                                        .wrapping_mul(
                                            ::core::mem::size_of::<rsrc_directory>() as libc::c_ulong,
                                        ),
                                ) as *mut rsrc_directory;
                                if !type_tables.is_null() {
                                    indx = 0 as libc::c_int as libc::c_uint;
                                    while data < dataend {
                                        let mut p_0: *mut bfd_byte = data;
                                        rsrc_parse_directory(
                                            abfd,
                                            type_tables.offset(indx as isize),
                                            data,
                                            data,
                                            dataend,
                                            rva_bias,
                                            0 as *mut rsrc_entry,
                                        );
                                        data = p_0
                                            .offset(*rsrc_sizes.offset(indx as isize) as isize);
                                        rva_bias = (rva_bias as libc::c_ulong)
                                            .wrapping_add(
                                                data.offset_from(p_0) as libc::c_long as libc::c_ulong,
                                            ) as bfd_vma as bfd_vma;
                                        indx = indx.wrapping_add(1);
                                        indx;
                                    }
                                    if !(indx == num_resource_sets) {
                                        bfd_assert(
                                            b"peXXigen.c\0" as *const u8 as *const libc::c_char,
                                            4265 as libc::c_int,
                                        );
                                    }
                                    new_table
                                        .characteristics = (*type_tables
                                        .offset(0 as libc::c_int as isize))
                                        .characteristics;
                                    new_table
                                        .time = (*type_tables.offset(0 as libc::c_int as isize))
                                        .time;
                                    new_table
                                        .major = (*type_tables.offset(0 as libc::c_int as isize))
                                        .major;
                                    new_table
                                        .minor = (*type_tables.offset(0 as libc::c_int as isize))
                                        .minor;
                                    new_table.names.first_entry = 0 as *mut rsrc_entry;
                                    new_table.names.last_entry = 0 as *mut rsrc_entry;
                                    indx = 0 as libc::c_int as libc::c_uint;
                                    while indx < num_resource_sets {
                                        rsrc_attach_chain(
                                            &mut new_table.names,
                                            &mut (*type_tables.offset(indx as isize)).names,
                                        );
                                        indx = indx.wrapping_add(1);
                                        indx;
                                    }
                                    rsrc_sort_entries(
                                        &mut new_table.names,
                                        1 as libc::c_int != 0,
                                        &mut new_table,
                                    );
                                    new_table.ids.first_entry = 0 as *mut rsrc_entry;
                                    new_table.ids.last_entry = 0 as *mut rsrc_entry;
                                    indx = 0 as libc::c_int as libc::c_uint;
                                    while indx < num_resource_sets {
                                        rsrc_attach_chain(
                                            &mut new_table.ids,
                                            &mut (*type_tables.offset(indx as isize)).ids,
                                        );
                                        indx = indx.wrapping_add(1);
                                        indx;
                                    }
                                    rsrc_sort_entries(
                                        &mut new_table.ids,
                                        0 as libc::c_int != 0,
                                        &mut new_table,
                                    );
                                    sizeof_tables_and_entries = 0 as libc::c_int
                                        as libc::c_uint;
                                    sizeof_strings = sizeof_tables_and_entries;
                                    sizeof_leaves = sizeof_strings;
                                    rsrc_compute_region_sizes(&mut new_table);
                                    sizeof_strings = sizeof_strings
                                        .wrapping_add(7 as libc::c_int as libc::c_uint)
                                        & !(7 as libc::c_int) as libc::c_uint;
                                    new_data = bfd_zalloc(abfd, size) as *mut bfd_byte;
                                    if !new_data.is_null() {
                                        write_data.abfd = abfd;
                                        write_data.datastart = new_data;
                                        write_data.next_table = new_data;
                                        write_data
                                            .next_leaf = new_data
                                            .offset(sizeof_tables_and_entries as isize);
                                        write_data
                                            .next_string = (write_data.next_leaf)
                                            .offset(sizeof_leaves as isize);
                                        write_data
                                            .next_data = (write_data.next_string)
                                            .offset(sizeof_strings as isize);
                                        write_data
                                            .rva_bias = ((*sec).vma)
                                            .wrapping_sub((*pe).pe_opthdr.ImageBase);
                                        rsrc_write_directory(&mut write_data, &mut new_table);
                                        bfd_set_section_contents(
                                            (*pfinfo).output_bfd,
                                            sec,
                                            new_data as *const libc::c_void,
                                            0 as libc::c_int as file_ptr,
                                            size,
                                        );
                                        (*sec).rawsize = size;
                                        (*sec).size = (*sec).rawsize;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    free(datastart as *mut libc::c_void);
    free(rsrc_sizes as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn _bfd_pex64i_final_link_postscript(
    mut abfd: *mut bfd,
    mut pfinfo: *mut coff_final_link_info,
) -> bool {
    let mut h1: *mut coff_link_hash_entry = 0 as *mut coff_link_hash_entry;
    let mut info: *mut bfd_link_info = (*pfinfo).info;
    let mut result: bool = 1 as libc::c_int != 0;
    h1 = bfd_link_hash_lookup(
        &mut (*((*info).hash as *mut coff_link_hash_table)).root,
        b".idata$2\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int != 0,
        0 as libc::c_int != 0,
        1 as libc::c_int != 0,
    ) as *mut coff_link_hash_entry;
    if !h1.is_null() {
        if (((*h1).root).type_0() as libc::c_int == bfd_link_hash_defined as libc::c_int
            || ((*h1).root).type_0() as libc::c_int
                == bfd_link_hash_defweak as libc::c_int)
            && !((*h1).root.u.def.section).is_null()
            && !((*(*h1).root.u.def.section).output_section).is_null()
        {
            (*(*abfd).tdata.pe_obj_data)
                .pe_opthdr
                .DataDirectory[1 as libc::c_int as usize]
                .VirtualAddress = ((*h1).root.u.def.value)
                .wrapping_add((*(*(*h1).root.u.def.section).output_section).vma)
                .wrapping_add((*(*h1).root.u.def.section).output_offset);
        } else {
            _bfd_error_handler(
                dcgettext(
                    b"bfd\0" as *const u8 as *const libc::c_char,
                    b"%pB: unable to fill in DataDictionary[1] because .idata$2 is missing\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                abfd,
            );
            result = 0 as libc::c_int != 0;
        }
        h1 = bfd_link_hash_lookup(
            &mut (*((*info).hash as *mut coff_link_hash_table)).root,
            b".idata$4\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int != 0,
            0 as libc::c_int != 0,
            1 as libc::c_int != 0,
        ) as *mut coff_link_hash_entry;
        if !h1.is_null()
            && (((*h1).root).type_0() as libc::c_int
                == bfd_link_hash_defined as libc::c_int
                || ((*h1).root).type_0() as libc::c_int
                    == bfd_link_hash_defweak as libc::c_int)
            && !((*h1).root.u.def.section).is_null()
            && !((*(*h1).root.u.def.section).output_section).is_null()
        {
            (*(*abfd).tdata.pe_obj_data)
                .pe_opthdr
                .DataDirectory[1 as libc::c_int as usize]
                .Size = ((*h1).root.u.def.value)
                .wrapping_add((*(*(*h1).root.u.def.section).output_section).vma)
                .wrapping_add((*(*h1).root.u.def.section).output_offset)
                .wrapping_sub(
                    (*(*abfd).tdata.pe_obj_data)
                        .pe_opthdr
                        .DataDirectory[1 as libc::c_int as usize]
                        .VirtualAddress,
                ) as libc::c_long;
        } else {
            _bfd_error_handler(
                dcgettext(
                    b"bfd\0" as *const u8 as *const libc::c_char,
                    b"%pB: unable to fill in DataDictionary[1] because .idata$4 is missing\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                abfd,
            );
            result = 0 as libc::c_int != 0;
        }
        h1 = bfd_link_hash_lookup(
            &mut (*((*info).hash as *mut coff_link_hash_table)).root,
            b".idata$5\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int != 0,
            0 as libc::c_int != 0,
            1 as libc::c_int != 0,
        ) as *mut coff_link_hash_entry;
        if !h1.is_null()
            && (((*h1).root).type_0() as libc::c_int
                == bfd_link_hash_defined as libc::c_int
                || ((*h1).root).type_0() as libc::c_int
                    == bfd_link_hash_defweak as libc::c_int)
            && !((*h1).root.u.def.section).is_null()
            && !((*(*h1).root.u.def.section).output_section).is_null()
        {
            (*(*abfd).tdata.pe_obj_data)
                .pe_opthdr
                .DataDirectory[12 as libc::c_int as usize]
                .VirtualAddress = ((*h1).root.u.def.value)
                .wrapping_add((*(*(*h1).root.u.def.section).output_section).vma)
                .wrapping_add((*(*h1).root.u.def.section).output_offset);
        } else {
            _bfd_error_handler(
                dcgettext(
                    b"bfd\0" as *const u8 as *const libc::c_char,
                    b"%pB: unable to fill in DataDictionary[12] because .idata$5 is missing\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                abfd,
            );
            result = 0 as libc::c_int != 0;
        }
        h1 = bfd_link_hash_lookup(
            &mut (*((*info).hash as *mut coff_link_hash_table)).root,
            b".idata$6\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int != 0,
            0 as libc::c_int != 0,
            1 as libc::c_int != 0,
        ) as *mut coff_link_hash_entry;
        if !h1.is_null()
            && (((*h1).root).type_0() as libc::c_int
                == bfd_link_hash_defined as libc::c_int
                || ((*h1).root).type_0() as libc::c_int
                    == bfd_link_hash_defweak as libc::c_int)
            && !((*h1).root.u.def.section).is_null()
            && !((*(*h1).root.u.def.section).output_section).is_null()
        {
            (*(*abfd).tdata.pe_obj_data)
                .pe_opthdr
                .DataDirectory[12 as libc::c_int as usize]
                .Size = ((*h1).root.u.def.value)
                .wrapping_add((*(*(*h1).root.u.def.section).output_section).vma)
                .wrapping_add((*(*h1).root.u.def.section).output_offset)
                .wrapping_sub(
                    (*(*abfd).tdata.pe_obj_data)
                        .pe_opthdr
                        .DataDirectory[12 as libc::c_int as usize]
                        .VirtualAddress,
                ) as libc::c_long;
        } else {
            _bfd_error_handler(
                dcgettext(
                    b"bfd\0" as *const u8 as *const libc::c_char,
                    b"%pB: unable to fill in DataDictionary[PE_IMPORT_ADDRESS_TABLE (12)] because .idata$6 is missing\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                abfd,
            );
            result = 0 as libc::c_int != 0;
        }
    } else {
        h1 = bfd_link_hash_lookup(
            &mut (*((*info).hash as *mut coff_link_hash_table)).root,
            b"__IAT_start__\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int != 0,
            0 as libc::c_int != 0,
            1 as libc::c_int != 0,
        ) as *mut coff_link_hash_entry;
        if !h1.is_null()
            && (((*h1).root).type_0() as libc::c_int
                == bfd_link_hash_defined as libc::c_int
                || ((*h1).root).type_0() as libc::c_int
                    == bfd_link_hash_defweak as libc::c_int)
            && !((*h1).root.u.def.section).is_null()
            && !((*(*h1).root.u.def.section).output_section).is_null()
        {
            let mut iat_va: bfd_vma = 0;
            iat_va = ((*h1).root.u.def.value)
                .wrapping_add((*(*(*h1).root.u.def.section).output_section).vma)
                .wrapping_add((*(*h1).root.u.def.section).output_offset);
            h1 = bfd_link_hash_lookup(
                &mut (*((*info).hash as *mut coff_link_hash_table)).root,
                b"__IAT_end__\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int != 0,
                0 as libc::c_int != 0,
                1 as libc::c_int != 0,
            ) as *mut coff_link_hash_entry;
            if !h1.is_null()
                && (((*h1).root).type_0() as libc::c_int
                    == bfd_link_hash_defined as libc::c_int
                    || ((*h1).root).type_0() as libc::c_int
                        == bfd_link_hash_defweak as libc::c_int)
                && !((*h1).root.u.def.section).is_null()
                && !((*(*h1).root.u.def.section).output_section).is_null()
            {
                (*(*abfd).tdata.pe_obj_data)
                    .pe_opthdr
                    .DataDirectory[12 as libc::c_int as usize]
                    .Size = ((*h1).root.u.def.value)
                    .wrapping_add((*(*(*h1).root.u.def.section).output_section).vma)
                    .wrapping_add((*(*h1).root.u.def.section).output_offset)
                    .wrapping_sub(iat_va) as libc::c_long;
                if (*(*abfd).tdata.pe_obj_data)
                    .pe_opthdr
                    .DataDirectory[12 as libc::c_int as usize]
                    .Size != 0 as libc::c_int as libc::c_long
                {
                    (*(*abfd).tdata.pe_obj_data)
                        .pe_opthdr
                        .DataDirectory[12 as libc::c_int as usize]
                        .VirtualAddress = iat_va
                        .wrapping_sub((*(*abfd).tdata.pe_obj_data).pe_opthdr.ImageBase);
                }
            } else {
                _bfd_error_handler(
                    dcgettext(
                        b"bfd\0" as *const u8 as *const libc::c_char,
                        b"%pB: unable to fill in DataDictionary[PE_IMPORT_ADDRESS_TABLE(12)] because .idata$6 is missing\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    abfd,
                );
                result = 0 as libc::c_int != 0;
            }
        }
    }
    h1 = bfd_link_hash_lookup(
        &mut (*((*info).hash as *mut coff_link_hash_table)).root,
        if bfd_get_symbol_leading_char(abfd) as libc::c_int != 0 as libc::c_int {
            b"__tls_used\0" as *const u8 as *const libc::c_char
        } else {
            b"_tls_used\0" as *const u8 as *const libc::c_char
        },
        0 as libc::c_int != 0,
        0 as libc::c_int != 0,
        1 as libc::c_int != 0,
    ) as *mut coff_link_hash_entry;
    if !h1.is_null() {
        if (((*h1).root).type_0() as libc::c_int == bfd_link_hash_defined as libc::c_int
            || ((*h1).root).type_0() as libc::c_int
                == bfd_link_hash_defweak as libc::c_int)
            && !((*h1).root.u.def.section).is_null()
            && !((*(*h1).root.u.def.section).output_section).is_null()
        {
            (*(*abfd).tdata.pe_obj_data)
                .pe_opthdr
                .DataDirectory[9 as libc::c_int as usize]
                .VirtualAddress = ((*h1).root.u.def.value)
                .wrapping_add((*(*(*h1).root.u.def.section).output_section).vma)
                .wrapping_add((*(*h1).root.u.def.section).output_offset)
                .wrapping_sub((*(*abfd).tdata.pe_obj_data).pe_opthdr.ImageBase);
        } else {
            _bfd_error_handler(
                dcgettext(
                    b"bfd\0" as *const u8 as *const libc::c_char,
                    b"%pB: unable to fill in DataDictionary[9] because __tls_used is missing\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                abfd,
            );
            result = 0 as libc::c_int != 0;
        }
        (*(*abfd).tdata.pe_obj_data)
            .pe_opthdr
            .DataDirectory[9 as libc::c_int as usize]
            .Size = 0x28 as libc::c_int as libc::c_long;
    }
    let mut sec: *mut asection = bfd_get_section_by_name(
        abfd,
        b".pdata\0" as *const u8 as *const libc::c_char,
    );
    if !sec.is_null() {
        let mut x: bfd_size_type = (*sec).rawsize;
        let mut tmp_data: *mut bfd_byte = 0 as *mut bfd_byte;
        if x != 0 {
            tmp_data = bfd_malloc(x) as *mut bfd_byte;
        }
        if !tmp_data.is_null() {
            if bfd_get_section_contents(
                abfd,
                sec,
                tmp_data as *mut libc::c_void,
                0 as libc::c_int as file_ptr,
                x,
            ) {
                qsort(
                    tmp_data as *mut libc::c_void,
                    x.wrapping_div(12 as libc::c_int as libc::c_ulong),
                    12 as libc::c_int as size_t,
                    Some(
                        sort_x64_pdata
                            as unsafe extern "C" fn(
                                *const libc::c_void,
                                *const libc::c_void,
                            ) -> libc::c_int,
                    ),
                );
                bfd_set_section_contents(
                    (*pfinfo).output_bfd,
                    sec,
                    tmp_data as *const libc::c_void,
                    0 as libc::c_int as file_ptr,
                    x,
                );
            }
            free(tmp_data as *mut libc::c_void);
        } else {
            result = 0 as libc::c_int != 0;
        }
    }
    rsrc_process_section(abfd, pfinfo);
    return result;
}
