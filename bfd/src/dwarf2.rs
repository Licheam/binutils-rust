use ::libc;
use ::c2rust_bitfields;
extern "C" {
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
    pub type aout_data_struct;
    pub type ecoff_debug_swap;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
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
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn htab_create_alloc(
        _: size_t,
        _: htab_hash,
        _: htab_eq,
        _: htab_del,
        _: htab_alloc,
        _: htab_free,
    ) -> htab_t;
    fn htab_delete(_: htab_t);
    fn htab_find(_: htab_t, _: *const libc::c_void) -> *mut libc::c_void;
    fn htab_find_slot(
        _: htab_t,
        _: *const libc::c_void,
        _: insert_option,
    ) -> *mut *mut libc::c_void;
    static mut htab_hash_pointer: htab_hash;
    fn htab_hash_string(_: *const libc::c_void) -> hashval_t;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn bfd_hash_table_init(
        _: *mut bfd_hash_table,
        _: Option::<
            unsafe extern "C" fn(
                *mut bfd_hash_entry,
                *mut bfd_hash_table,
                *const libc::c_char,
            ) -> *mut bfd_hash_entry,
        >,
        _: libc::c_uint,
    ) -> bool;
    fn bfd_hash_table_free(_: *mut bfd_hash_table);
    fn bfd_hash_lookup(
        _: *mut bfd_hash_table,
        _: *const libc::c_char,
        create: bool,
        copy: bool,
    ) -> *mut bfd_hash_entry;
    fn bfd_hash_newfunc(
        _: *mut bfd_hash_entry,
        _: *mut bfd_hash_table,
        _: *const libc::c_char,
    ) -> *mut bfd_hash_entry;
    fn bfd_hash_allocate(_: *mut bfd_hash_table, _: libc::c_uint) -> *mut libc::c_void;
    fn bfd_openr(filename: *const libc::c_char, target: *const libc::c_char) -> *mut bfd;
    fn bfd_close(abfd: *mut bfd) -> bool;
    fn bfd_alloc(abfd: *mut bfd, wanted: bfd_size_type) -> *mut libc::c_void;
    fn bfd_zalloc(abfd: *mut bfd, wanted: bfd_size_type) -> *mut libc::c_void;
    fn bfd_follow_gnu_debuglink(
        abfd: *mut bfd,
        dir: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn bfd_follow_gnu_debugaltlink(
        abfd: *mut bfd,
        dir: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn bfd_follow_build_id_debuglink(
        abfd: *mut bfd,
        dir: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn bfd_get_file_size(abfd: *mut bfd) -> ufile_ptr;
    fn bfd_get_section_by_name(
        abfd: *mut bfd,
        name: *const libc::c_char,
    ) -> *mut asection;
    fn bfd_get_section_contents(
        abfd: *mut bfd,
        section: *mut asection,
        location: *mut libc::c_void,
        offset: file_ptr,
        count: bfd_size_type,
    ) -> bool;
    fn bfd_set_error(error_tag: bfd_error_type);
    fn _bfd_error_handler(fmt: *const libc::c_char, _: ...);
    fn bfd_simple_get_relocated_section_contents(
        abfd: *mut bfd,
        sec: *mut asection,
        outbuf: *mut bfd_byte,
        symbol_table: *mut *mut asymbol,
    ) -> *mut bfd_byte;
    fn bfd_check_format(abfd: *mut bfd, format: bfd_format) -> bool;
    fn xcalloc(_: size_t, _: size_t) -> *mut libc::c_void;
    fn bfd_malloc(_: bfd_size_type) -> *mut libc::c_void;
    fn bfd_release(_: *mut bfd, _: *mut libc::c_void);
    fn bfd_assert(_: *const libc::c_char, _: libc::c_int);
    fn _bfd_abort(_: *const libc::c_char, _: libc::c_int, _: *const libc::c_char) -> !;
    fn _bfd_safe_read_leb128(
        _: *mut bfd,
        _: *mut *mut bfd_byte,
        _: bool,
        _: *const bfd_byte,
    ) -> bfd_vma;
    fn bfd_realloc(MEM: *mut libc::c_void, SIZE: bfd_size_type) -> *mut libc::c_void;
    fn bfd_generic_link_read_symbols(_: *mut bfd) -> bool;
}
pub type size_t = libc::c_ulong;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type int64_t = __int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
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
pub type insert_option = libc::c_uint;
pub const INSERT: insert_option = 1;
pub const NO_INSERT: insert_option = 0;
pub type uint64_t = __uint64_t;
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
pub struct dwarf_debug_section {
    pub uncompressed_name: *const libc::c_char,
    pub compressed_name: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dwarf2_debug {
    pub debug_sections: *const dwarf_debug_section,
    pub f: dwarf2_debug_file,
    pub alt: dwarf2_debug_file,
    pub orig_bfd: *mut bfd,
    pub inliner_chain: *mut funcinfo,
    pub sec_vma: *mut bfd_vma,
    pub sec_vma_count: libc::c_uint,
    pub adjusted_section_count: libc::c_int,
    pub adjusted_sections: *mut adjusted_section,
    pub info_hash_count: libc::c_int,
    pub funcinfo_hash_table: *mut info_hash_table,
    pub varinfo_hash_table: *mut info_hash_table,
    pub hash_units_head: *mut comp_unit,
    pub info_hash_status: libc::c_int,
    pub close_on_cleanup: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct comp_unit {
    pub next_unit: *mut comp_unit,
    pub prev_unit: *mut comp_unit,
    pub abfd: *mut bfd,
    pub arange: arange,
    pub name: *mut libc::c_char,
    pub abbrevs: *mut *mut abbrev_info,
    pub lang: libc::c_int,
    pub error: libc::c_int,
    pub comp_dir: *mut libc::c_char,
    pub stmtlist: libc::c_int,
    pub info_ptr_unit: *mut bfd_byte,
    pub line_offset: libc::c_ulong,
    pub first_child_die_ptr: *mut bfd_byte,
    pub end_ptr: *mut bfd_byte,
    pub line_table: *mut line_info_table,
    pub function_table: *mut funcinfo,
    pub lookup_funcinfo_table: *mut lookup_funcinfo,
    pub number_of_functions: bfd_size_type,
    pub variable_table: *mut varinfo,
    pub stash: *mut dwarf2_debug,
    pub file: *mut dwarf2_debug_file,
    pub version: libc::c_int,
    pub addr_size: libc::c_uchar,
    pub offset_size: libc::c_uchar,
    pub base_address: bfd_vma,
    pub cached: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dwarf2_debug_file {
    pub bfd_ptr: *mut bfd,
    pub syms: *mut *mut asymbol,
    pub info_ptr: *mut bfd_byte,
    pub dwarf_info_buffer: *mut bfd_byte,
    pub dwarf_info_size: bfd_size_type,
    pub dwarf_abbrev_buffer: *mut bfd_byte,
    pub dwarf_abbrev_size: bfd_size_type,
    pub dwarf_line_buffer: *mut bfd_byte,
    pub dwarf_line_size: bfd_size_type,
    pub dwarf_str_buffer: *mut bfd_byte,
    pub dwarf_str_size: bfd_size_type,
    pub dwarf_line_str_buffer: *mut bfd_byte,
    pub dwarf_line_str_size: bfd_size_type,
    pub dwarf_ranges_buffer: *mut bfd_byte,
    pub dwarf_ranges_size: bfd_size_type,
    pub dwarf_rnglists_buffer: *mut bfd_byte,
    pub dwarf_rnglists_size: bfd_size_type,
    pub all_comp_units: *mut comp_unit,
    pub last_comp_unit: *mut comp_unit,
    pub line_table: *mut line_info_table,
    pub abbrev_offsets: htab_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct line_info_table {
    pub abfd: *mut bfd,
    pub num_files: libc::c_uint,
    pub num_dirs: libc::c_uint,
    pub num_sequences: libc::c_uint,
    pub comp_dir: *mut libc::c_char,
    pub dirs: *mut *mut libc::c_char,
    pub files: *mut fileinfo,
    pub sequences: *mut line_sequence,
    pub lcl_head: *mut line_info,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct line_info {
    pub prev_line: *mut line_info,
    pub address: bfd_vma,
    pub filename: *mut libc::c_char,
    pub line: libc::c_uint,
    pub column: libc::c_uint,
    pub discriminator: libc::c_uint,
    pub op_index: libc::c_uchar,
    pub end_sequence: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct line_sequence {
    pub low_pc: bfd_vma,
    pub prev_sequence: *mut line_sequence,
    pub last_line: *mut line_info,
    pub line_info_lookup: *mut *mut line_info,
    pub num_lines: bfd_size_type,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fileinfo {
    pub name: *mut libc::c_char,
    pub dir: libc::c_uint,
    pub time: libc::c_uint,
    pub size: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct varinfo {
    pub prev_var: *mut varinfo,
    pub unit_offset: bfd_uint64_t,
    pub file: *mut libc::c_char,
    pub line: libc::c_int,
    pub tag: libc::c_int,
    pub name: *mut libc::c_char,
    pub addr: bfd_vma,
    pub sec: *mut asection,
    pub stack: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lookup_funcinfo {
    pub funcinfo: *mut funcinfo,
    pub low_addr: bfd_vma,
    pub high_addr: bfd_vma,
    pub idx: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct funcinfo {
    pub prev_func: *mut funcinfo,
    pub caller_func: *mut funcinfo,
    pub caller_file: *mut libc::c_char,
    pub file: *mut libc::c_char,
    pub caller_line: libc::c_int,
    pub line: libc::c_int,
    pub tag: libc::c_int,
    pub is_linkage: bool,
    pub name: *const libc::c_char,
    pub arange: arange,
    pub sec: *mut asection,
    pub unit_offset: bfd_uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct arange {
    pub next: *mut arange,
    pub low: bfd_vma,
    pub high: bfd_vma,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct abbrev_info {
    pub number: libc::c_uint,
    pub tag: dwarf_tag,
    pub has_children: bool,
    pub num_attrs: libc::c_uint,
    pub attrs: *mut attr_abbrev,
    pub next: *mut abbrev_info,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct attr_abbrev {
    pub name: dwarf_attribute,
    pub form: dwarf_form,
    pub implicit_const: bfd_vma,
}
pub type dwarf_form = libc::c_uint;
pub const DW_FORM_GNU_strp_alt: dwarf_form = 7969;
pub const DW_FORM_GNU_ref_alt: dwarf_form = 7968;
pub const DW_FORM_GNU_str_index: dwarf_form = 7938;
pub const DW_FORM_GNU_addr_index: dwarf_form = 7937;
pub const DW_FORM_addrx4: dwarf_form = 44;
pub const DW_FORM_addrx3: dwarf_form = 43;
pub const DW_FORM_addrx2: dwarf_form = 42;
pub const DW_FORM_addrx1: dwarf_form = 41;
pub const DW_FORM_strx4: dwarf_form = 40;
pub const DW_FORM_strx3: dwarf_form = 39;
pub const DW_FORM_strx2: dwarf_form = 38;
pub const DW_FORM_strx1: dwarf_form = 37;
pub const DW_FORM_ref_sup8: dwarf_form = 36;
pub const DW_FORM_rnglistx: dwarf_form = 35;
pub const DW_FORM_loclistx: dwarf_form = 34;
pub const DW_FORM_implicit_const: dwarf_form = 33;
pub const DW_FORM_line_strp: dwarf_form = 31;
pub const DW_FORM_data16: dwarf_form = 30;
pub const DW_FORM_strp_sup: dwarf_form = 29;
pub const DW_FORM_ref_sup4: dwarf_form = 28;
pub const DW_FORM_addrx: dwarf_form = 27;
pub const DW_FORM_strx: dwarf_form = 26;
pub const DW_FORM_ref_sig8: dwarf_form = 32;
pub const DW_FORM_flag_present: dwarf_form = 25;
pub const DW_FORM_exprloc: dwarf_form = 24;
pub const DW_FORM_sec_offset: dwarf_form = 23;
pub const DW_FORM_indirect: dwarf_form = 22;
pub const DW_FORM_ref_udata: dwarf_form = 21;
pub const DW_FORM_ref8: dwarf_form = 20;
pub const DW_FORM_ref4: dwarf_form = 19;
pub const DW_FORM_ref2: dwarf_form = 18;
pub const DW_FORM_ref1: dwarf_form = 17;
pub const DW_FORM_ref_addr: dwarf_form = 16;
pub const DW_FORM_udata: dwarf_form = 15;
pub const DW_FORM_strp: dwarf_form = 14;
pub const DW_FORM_sdata: dwarf_form = 13;
pub const DW_FORM_flag: dwarf_form = 12;
pub const DW_FORM_data1: dwarf_form = 11;
pub const DW_FORM_block1: dwarf_form = 10;
pub const DW_FORM_block: dwarf_form = 9;
pub const DW_FORM_string: dwarf_form = 8;
pub const DW_FORM_data8: dwarf_form = 7;
pub const DW_FORM_data4: dwarf_form = 6;
pub const DW_FORM_data2: dwarf_form = 5;
pub const DW_FORM_block4: dwarf_form = 4;
pub const DW_FORM_block2: dwarf_form = 3;
pub const DW_FORM_addr: dwarf_form = 1;
pub type dwarf_attribute = libc::c_uint;
pub const DW_AT_APPLE_property: dwarf_attribute = 16365;
pub const DW_AT_APPLE_objc_complete_type: dwarf_attribute = 16364;
pub const DW_AT_APPLE_property_attribute: dwarf_attribute = 16363;
pub const DW_AT_APPLE_property_setter: dwarf_attribute = 16362;
pub const DW_AT_APPLE_property_getter: dwarf_attribute = 16361;
pub const DW_AT_APPLE_property_name: dwarf_attribute = 16360;
pub const DW_AT_APPLE_omit_frame_ptr: dwarf_attribute = 16359;
pub const DW_AT_APPLE_runtime_class: dwarf_attribute = 16358;
pub const DW_AT_APPLE_major_runtime_vers: dwarf_attribute = 16357;
pub const DW_AT_APPLE_block: dwarf_attribute = 16356;
pub const DW_AT_APPLE_isa: dwarf_attribute = 16355;
pub const DW_AT_APPLE_flags: dwarf_attribute = 16354;
pub const DW_AT_APPLE_optimized: dwarf_attribute = 16353;
pub const DW_AT_PGI_lstride: dwarf_attribute = 14850;
pub const DW_AT_PGI_soffset: dwarf_attribute = 14849;
pub const DW_AT_PGI_lbase: dwarf_attribute = 14848;
pub const DW_AT_upc_threads_scaled: dwarf_attribute = 12816;
pub const DW_AT_GNU_bias: dwarf_attribute = 8965;
pub const DW_AT_GNU_denominator: dwarf_attribute = 8964;
pub const DW_AT_GNU_numerator: dwarf_attribute = 8963;
pub const DW_AT_GNAT_descriptive_type: dwarf_attribute = 8962;
pub const DW_AT_use_GNAT_descriptive_type: dwarf_attribute = 8961;
pub const DW_AT_VMS_rtnbeg_pd_address: dwarf_attribute = 8705;
pub const DW_AT_GNU_entry_view: dwarf_attribute = 8504;
pub const DW_AT_GNU_locviews: dwarf_attribute = 8503;
pub const DW_AT_GNU_discriminator: dwarf_attribute = 8502;
pub const DW_AT_GNU_pubtypes: dwarf_attribute = 8501;
pub const DW_AT_GNU_pubnames: dwarf_attribute = 8500;
pub const DW_AT_GNU_addr_base: dwarf_attribute = 8499;
pub const DW_AT_GNU_ranges_base: dwarf_attribute = 8498;
pub const DW_AT_GNU_dwo_id: dwarf_attribute = 8497;
pub const DW_AT_GNU_dwo_name: dwarf_attribute = 8496;
pub const DW_AT_GNU_deleted: dwarf_attribute = 8474;
pub const DW_AT_GNU_macros: dwarf_attribute = 8473;
pub const DW_AT_GNU_all_source_call_sites: dwarf_attribute = 8472;
pub const DW_AT_GNU_all_call_sites: dwarf_attribute = 8471;
pub const DW_AT_GNU_all_tail_call_sites: dwarf_attribute = 8470;
pub const DW_AT_GNU_tail_call: dwarf_attribute = 8469;
pub const DW_AT_GNU_call_site_target_clobbered: dwarf_attribute = 8468;
pub const DW_AT_GNU_call_site_target: dwarf_attribute = 8467;
pub const DW_AT_GNU_call_site_data_value: dwarf_attribute = 8466;
pub const DW_AT_GNU_call_site_value: dwarf_attribute = 8465;
pub const DW_AT_GNU_template_name: dwarf_attribute = 8464;
pub const DW_AT_GNU_odr_signature: dwarf_attribute = 8463;
pub const DW_AT_GNU_shared_locks_required: dwarf_attribute = 8462;
pub const DW_AT_GNU_exclusive_locks_required: dwarf_attribute = 8461;
pub const DW_AT_GNU_locks_excluded: dwarf_attribute = 8460;
pub const DW_AT_GNU_pt_guarded: dwarf_attribute = 8459;
pub const DW_AT_GNU_guarded: dwarf_attribute = 8458;
pub const DW_AT_GNU_pt_guarded_by: dwarf_attribute = 8457;
pub const DW_AT_GNU_guarded_by: dwarf_attribute = 8456;
pub const DW_AT_GNU_vector: dwarf_attribute = 8455;
pub const DW_AT_body_end: dwarf_attribute = 8454;
pub const DW_AT_body_begin: dwarf_attribute = 8453;
pub const DW_AT_src_coords: dwarf_attribute = 8452;
pub const DW_AT_mac_info: dwarf_attribute = 8451;
pub const DW_AT_src_info: dwarf_attribute = 8450;
pub const DW_AT_sf_names: dwarf_attribute = 8449;
pub const DW_AT_HP_is_result_param: dwarf_attribute = 8233;
pub const DW_AT_HP_default_location: dwarf_attribute = 8227;
pub const DW_AT_HP_definition_points: dwarf_attribute = 8226;
pub const DW_AT_HP_widened_byte_size: dwarf_attribute = 8225;
pub const DW_AT_HP_unit_size: dwarf_attribute = 8224;
pub const DW_AT_HP_unit_name: dwarf_attribute = 8223;
pub const DW_AT_HP_prof_flags: dwarf_attribute = 8219;
pub const DW_AT_HP_linkage_name: dwarf_attribute = 8218;
pub const DW_AT_HP_all_variables_modifiable: dwarf_attribute = 8217;
pub const DW_AT_HP_cold_region_high_pc: dwarf_attribute = 8216;
pub const DW_AT_HP_cold_region_low_pc: dwarf_attribute = 8215;
pub const DW_AT_HP_opt_flags: dwarf_attribute = 8214;
pub const DW_AT_HP_prof_version_id: dwarf_attribute = 8213;
pub const DW_AT_HP_opt_level: dwarf_attribute = 8212;
pub const DW_AT_HP_pass_by_reference: dwarf_attribute = 8211;
pub const DW_AT_HP_raw_data_ptr: dwarf_attribute = 8210;
pub const DW_AT_HP_proc_per_section: dwarf_attribute = 8209;
pub const DW_AT_HP_actuals_stmt_list: dwarf_attribute = 8208;
pub const DW_AT_HP_epilogue: dwarf_attribute = 8200;
pub const DW_AT_HP_prologue: dwarf_attribute = 8197;
pub const DW_AT_HP_unmodifiable: dwarf_attribute = 8193;
pub const DW_AT_HP_block_index: dwarf_attribute = 8192;
pub const DW_AT_MIPS_has_inlines: dwarf_attribute = 8203;
pub const DW_AT_MIPS_clone_origin: dwarf_attribute = 8202;
pub const DW_AT_MIPS_abstract_name: dwarf_attribute = 8201;
pub const DW_AT_MIPS_stride: dwarf_attribute = 8200;
pub const DW_AT_MIPS_linkage_name: dwarf_attribute = 8199;
pub const DW_AT_MIPS_software_pipeline_depth: dwarf_attribute = 8198;
pub const DW_AT_MIPS_loop_unroll_factor: dwarf_attribute = 8197;
pub const DW_AT_MIPS_epilog_begin: dwarf_attribute = 8196;
pub const DW_AT_MIPS_tail_loop_begin: dwarf_attribute = 8195;
pub const DW_AT_MIPS_loop_begin: dwarf_attribute = 8194;
pub const DW_AT_MIPS_fde: dwarf_attribute = 8193;
pub const DW_AT_hi_user: dwarf_attribute = 16383;
pub const DW_AT_lo_user: dwarf_attribute = 8192;
pub const DW_AT_loclists_base: dwarf_attribute = 140;
pub const DW_AT_defaulted: dwarf_attribute = 139;
pub const DW_AT_deleted: dwarf_attribute = 138;
pub const DW_AT_export_symbols: dwarf_attribute = 137;
pub const DW_AT_alignment: dwarf_attribute = 136;
pub const DW_AT_noreturn: dwarf_attribute = 135;
pub const DW_AT_call_data_value: dwarf_attribute = 134;
pub const DW_AT_call_data_location: dwarf_attribute = 133;
pub const DW_AT_call_target_clobbered: dwarf_attribute = 132;
pub const DW_AT_call_target: dwarf_attribute = 131;
pub const DW_AT_call_tail_call: dwarf_attribute = 130;
pub const DW_AT_call_pc: dwarf_attribute = 129;
pub const DW_AT_call_parameter: dwarf_attribute = 128;
pub const DW_AT_call_origin: dwarf_attribute = 127;
pub const DW_AT_call_value: dwarf_attribute = 126;
pub const DW_AT_call_return_pc: dwarf_attribute = 125;
pub const DW_AT_call_all_tail_calls: dwarf_attribute = 124;
pub const DW_AT_call_all_source_calls: dwarf_attribute = 123;
pub const DW_AT_call_all_calls: dwarf_attribute = 122;
pub const DW_AT_macros: dwarf_attribute = 121;
pub const DW_AT_rvalue_reference: dwarf_attribute = 120;
pub const DW_AT_reference: dwarf_attribute = 119;
pub const DW_AT_dwo_name: dwarf_attribute = 118;
pub const DW_AT_rnglists_base: dwarf_attribute = 116;
pub const DW_AT_addr_base: dwarf_attribute = 115;
pub const DW_AT_str_offsets_base: dwarf_attribute = 114;
pub const DW_AT_rank: dwarf_attribute = 113;
pub const DW_AT_string_length_byte_size: dwarf_attribute = 112;
pub const DW_AT_string_length_bit_size: dwarf_attribute = 111;
pub const DW_AT_linkage_name: dwarf_attribute = 110;
pub const DW_AT_enum_class: dwarf_attribute = 109;
pub const DW_AT_const_expr: dwarf_attribute = 108;
pub const DW_AT_data_bit_offset: dwarf_attribute = 107;
pub const DW_AT_main_subprogram: dwarf_attribute = 106;
pub const DW_AT_signature: dwarf_attribute = 105;
pub const DW_AT_recursive: dwarf_attribute = 104;
pub const DW_AT_pure: dwarf_attribute = 103;
pub const DW_AT_elemental: dwarf_attribute = 102;
pub const DW_AT_endianity: dwarf_attribute = 101;
pub const DW_AT_object_pointer: dwarf_attribute = 100;
pub const DW_AT_explicit: dwarf_attribute = 99;
pub const DW_AT_threads_scaled: dwarf_attribute = 98;
pub const DW_AT_mutable: dwarf_attribute = 97;
pub const DW_AT_picture_string: dwarf_attribute = 96;
pub const DW_AT_digit_count: dwarf_attribute = 95;
pub const DW_AT_decimal_sign: dwarf_attribute = 94;
pub const DW_AT_small: dwarf_attribute = 93;
pub const DW_AT_decimal_scale: dwarf_attribute = 92;
pub const DW_AT_binary_scale: dwarf_attribute = 91;
pub const DW_AT_description: dwarf_attribute = 90;
pub const DW_AT_call_line: dwarf_attribute = 89;
pub const DW_AT_call_file: dwarf_attribute = 88;
pub const DW_AT_call_column: dwarf_attribute = 87;
pub const DW_AT_trampoline: dwarf_attribute = 86;
pub const DW_AT_ranges: dwarf_attribute = 85;
pub const DW_AT_extension: dwarf_attribute = 84;
pub const DW_AT_use_UTF8: dwarf_attribute = 83;
pub const DW_AT_entry_pc: dwarf_attribute = 82;
pub const DW_AT_byte_stride: dwarf_attribute = 81;
pub const DW_AT_data_location: dwarf_attribute = 80;
pub const DW_AT_associated: dwarf_attribute = 79;
pub const DW_AT_allocated: dwarf_attribute = 78;
pub const DW_AT_vtable_elem_location: dwarf_attribute = 77;
pub const DW_AT_virtuality: dwarf_attribute = 76;
pub const DW_AT_variable_parameter: dwarf_attribute = 75;
pub const DW_AT_use_location: dwarf_attribute = 74;
pub const DW_AT_type: dwarf_attribute = 73;
pub const DW_AT_static_link: dwarf_attribute = 72;
pub const DW_AT_specification: dwarf_attribute = 71;
pub const DW_AT_segment: dwarf_attribute = 70;
pub const DW_AT_priority: dwarf_attribute = 69;
pub const DW_AT_namelist_items: dwarf_attribute = 68;
pub const DW_AT_macro_info: dwarf_attribute = 67;
pub const DW_AT_identifier_case: dwarf_attribute = 66;
pub const DW_AT_friend: dwarf_attribute = 65;
pub const DW_AT_frame_base: dwarf_attribute = 64;
pub const DW_AT_external: dwarf_attribute = 63;
pub const DW_AT_encoding: dwarf_attribute = 62;
pub const DW_AT_discr_list: dwarf_attribute = 61;
pub const DW_AT_declaration: dwarf_attribute = 60;
pub const DW_AT_decl_line: dwarf_attribute = 59;
pub const DW_AT_decl_file: dwarf_attribute = 58;
pub const DW_AT_decl_column: dwarf_attribute = 57;
pub const DW_AT_data_member_location: dwarf_attribute = 56;
pub const DW_AT_count: dwarf_attribute = 55;
pub const DW_AT_calling_convention: dwarf_attribute = 54;
pub const DW_AT_base_types: dwarf_attribute = 53;
pub const DW_AT_artificial: dwarf_attribute = 52;
pub const DW_AT_address_class: dwarf_attribute = 51;
pub const DW_AT_accessibility: dwarf_attribute = 50;
pub const DW_AT_abstract_origin: dwarf_attribute = 49;
pub const DW_AT_upper_bound: dwarf_attribute = 47;
pub const DW_AT_bit_stride: dwarf_attribute = 46;
pub const DW_AT_start_scope: dwarf_attribute = 44;
pub const DW_AT_return_addr: dwarf_attribute = 42;
pub const DW_AT_prototyped: dwarf_attribute = 39;
pub const DW_AT_producer: dwarf_attribute = 37;
pub const DW_AT_lower_bound: dwarf_attribute = 34;
pub const DW_AT_is_optional: dwarf_attribute = 33;
pub const DW_AT_inline: dwarf_attribute = 32;
pub const DW_AT_default_value: dwarf_attribute = 30;
pub const DW_AT_containing_type: dwarf_attribute = 29;
pub const DW_AT_const_value: dwarf_attribute = 28;
pub const DW_AT_comp_dir: dwarf_attribute = 27;
pub const DW_AT_common_reference: dwarf_attribute = 26;
pub const DW_AT_string_length: dwarf_attribute = 25;
pub const DW_AT_import: dwarf_attribute = 24;
pub const DW_AT_visibility: dwarf_attribute = 23;
pub const DW_AT_discr_value: dwarf_attribute = 22;
pub const DW_AT_discr: dwarf_attribute = 21;
pub const DW_AT_member: dwarf_attribute = 20;
pub const DW_AT_language: dwarf_attribute = 19;
pub const DW_AT_high_pc: dwarf_attribute = 18;
pub const DW_AT_low_pc: dwarf_attribute = 17;
pub const DW_AT_stmt_list: dwarf_attribute = 16;
pub const DW_AT_element_list: dwarf_attribute = 15;
pub const DW_AT_bit_size: dwarf_attribute = 13;
pub const DW_AT_bit_offset: dwarf_attribute = 12;
pub const DW_AT_byte_size: dwarf_attribute = 11;
pub const DW_AT_subscr_data: dwarf_attribute = 10;
pub const DW_AT_ordering: dwarf_attribute = 9;
pub const DW_AT_name: dwarf_attribute = 3;
pub const DW_AT_location: dwarf_attribute = 2;
pub const DW_AT_sibling: dwarf_attribute = 1;
pub type dwarf_tag = libc::c_uint;
pub const DW_TAG_PGI_interface_block: dwarf_tag = 40992;
pub const DW_TAG_PGI_kanji_type: dwarf_tag = 40960;
pub const DW_TAG_upc_relaxed_type: dwarf_tag = 34663;
pub const DW_TAG_upc_strict_type: dwarf_tag = 34662;
pub const DW_TAG_upc_shared_type: dwarf_tag = 34661;
pub const DW_TAG_GNU_call_site_parameter: dwarf_tag = 16650;
pub const DW_TAG_GNU_call_site: dwarf_tag = 16649;
pub const DW_TAG_GNU_formal_parameter_pack: dwarf_tag = 16648;
pub const DW_TAG_GNU_template_parameter_pack: dwarf_tag = 16647;
pub const DW_TAG_GNU_template_template_param: dwarf_tag = 16646;
pub const DW_TAG_GNU_EINCL: dwarf_tag = 16645;
pub const DW_TAG_GNU_BINCL: dwarf_tag = 16644;
pub const DW_TAG_class_template: dwarf_tag = 16643;
pub const DW_TAG_function_template: dwarf_tag = 16642;
pub const DW_TAG_format_label: dwarf_tag = 16641;
pub const DW_TAG_HP_Bliss_field_set: dwarf_tag = 16530;
pub const DW_TAG_HP_Bliss_field: dwarf_tag = 16529;
pub const DW_TAG_HP_array_descriptor: dwarf_tag = 16528;
pub const DW_TAG_MIPS_loop: dwarf_tag = 16513;
pub const DW_TAG_hi_user: dwarf_tag = 65535;
pub const DW_TAG_lo_user: dwarf_tag = 16512;
pub const DW_TAG_immutable_type: dwarf_tag = 75;
pub const DW_TAG_skeleton_unit: dwarf_tag = 74;
pub const DW_TAG_call_site_parameter: dwarf_tag = 73;
pub const DW_TAG_call_site: dwarf_tag = 72;
pub const DW_TAG_atomic_type: dwarf_tag = 71;
pub const DW_TAG_dynamic_type: dwarf_tag = 70;
pub const DW_TAG_generic_subrange: dwarf_tag = 69;
pub const DW_TAG_coarray_type: dwarf_tag = 68;
pub const DW_TAG_template_alias: dwarf_tag = 67;
pub const DW_TAG_rvalue_reference_type: dwarf_tag = 66;
pub const DW_TAG_type_unit: dwarf_tag = 65;
pub const DW_TAG_shared_type: dwarf_tag = 64;
pub const DW_TAG_condition: dwarf_tag = 63;
pub const DW_TAG_imported_unit: dwarf_tag = 61;
pub const DW_TAG_partial_unit: dwarf_tag = 60;
pub const DW_TAG_unspecified_type: dwarf_tag = 59;
pub const DW_TAG_imported_module: dwarf_tag = 58;
pub const DW_TAG_namespace: dwarf_tag = 57;
pub const DW_TAG_interface_type: dwarf_tag = 56;
pub const DW_TAG_restrict_type: dwarf_tag = 55;
pub const DW_TAG_dwarf_procedure: dwarf_tag = 54;
pub const DW_TAG_volatile_type: dwarf_tag = 53;
pub const DW_TAG_variable: dwarf_tag = 52;
pub const DW_TAG_variant_part: dwarf_tag = 51;
pub const DW_TAG_try_block: dwarf_tag = 50;
pub const DW_TAG_thrown_type: dwarf_tag = 49;
pub const DW_TAG_template_value_param: dwarf_tag = 48;
pub const DW_TAG_template_type_param: dwarf_tag = 47;
pub const DW_TAG_subprogram: dwarf_tag = 46;
pub const DW_TAG_packed_type: dwarf_tag = 45;
pub const DW_TAG_namelist_item: dwarf_tag = 44;
pub const DW_TAG_namelist: dwarf_tag = 43;
pub const DW_TAG_friend: dwarf_tag = 42;
pub const DW_TAG_file_type: dwarf_tag = 41;
pub const DW_TAG_enumerator: dwarf_tag = 40;
pub const DW_TAG_constant: dwarf_tag = 39;
pub const DW_TAG_const_type: dwarf_tag = 38;
pub const DW_TAG_catch_block: dwarf_tag = 37;
pub const DW_TAG_base_type: dwarf_tag = 36;
pub const DW_TAG_access_declaration: dwarf_tag = 35;
pub const DW_TAG_with_stmt: dwarf_tag = 34;
pub const DW_TAG_subrange_type: dwarf_tag = 33;
pub const DW_TAG_set_type: dwarf_tag = 32;
pub const DW_TAG_ptr_to_member_type: dwarf_tag = 31;
pub const DW_TAG_module: dwarf_tag = 30;
pub const DW_TAG_inlined_subroutine: dwarf_tag = 29;
pub const DW_TAG_inheritance: dwarf_tag = 28;
pub const DW_TAG_common_inclusion: dwarf_tag = 27;
pub const DW_TAG_common_block: dwarf_tag = 26;
pub const DW_TAG_variant: dwarf_tag = 25;
pub const DW_TAG_unspecified_parameters: dwarf_tag = 24;
pub const DW_TAG_union_type: dwarf_tag = 23;
pub const DW_TAG_typedef: dwarf_tag = 22;
pub const DW_TAG_subroutine_type: dwarf_tag = 21;
pub const DW_TAG_structure_type: dwarf_tag = 19;
pub const DW_TAG_string_type: dwarf_tag = 18;
pub const DW_TAG_compile_unit: dwarf_tag = 17;
pub const DW_TAG_reference_type: dwarf_tag = 16;
pub const DW_TAG_pointer_type: dwarf_tag = 15;
pub const DW_TAG_member: dwarf_tag = 13;
pub const DW_TAG_lexical_block: dwarf_tag = 11;
pub const DW_TAG_label: dwarf_tag = 10;
pub const DW_TAG_imported_declaration: dwarf_tag = 8;
pub const DW_TAG_formal_parameter: dwarf_tag = 5;
pub const DW_TAG_enumeration_type: dwarf_tag = 4;
pub const DW_TAG_entry_point: dwarf_tag = 3;
pub const DW_TAG_class_type: dwarf_tag = 2;
pub const DW_TAG_array_type: dwarf_tag = 1;
pub const DW_TAG_padding: dwarf_tag = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct info_hash_table {
    pub base: bfd_hash_table,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct adjusted_section {
    pub section: *mut asection,
    pub adj_vma: bfd_vma,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct elf_find_function_cache {
    pub last_section: *mut asection,
    pub func: *mut asymbol,
    pub filename: *const libc::c_char,
    pub func_size: bfd_size_type,
}
pub type C2RustUnnamed_21 = libc::c_uint;
pub const file_after_symbol_seen: C2RustUnnamed_21 = 2;
pub const symbol_seen: C2RustUnnamed_21 = 1;
pub const nothing_seen: C2RustUnnamed_21 = 0;
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
pub type Elf_Internal_Sym = elf_internal_sym;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bfd_elf_special_section {
    pub prefix: *const libc::c_char,
    pub prefix_length: libc::c_uint,
    pub suffix_length: libc::c_int,
    pub type_0: libc::c_uint,
    pub attr: bfd_vma,
}
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
pub type Elf_Internal_Rela = elf_internal_rela;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct elf_internal_rela {
    pub r_offset: bfd_vma,
    pub r_info: bfd_vma,
    pub r_addend: bfd_vma,
}
pub type Elf_Internal_Dyn = elf_internal_dyn;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct elf_internal_dyn {
    pub d_tag: bfd_vma,
    pub d_un: C2RustUnnamed_23,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_23 {
    pub d_val: bfd_vma,
    pub d_ptr: bfd_vma,
}
pub type irix_compat_t = libc::c_uint;
pub const ict_irix6: irix_compat_t = 2;
pub const ict_irix5: irix_compat_t = 1;
pub const ict_none: irix_compat_t = 0;
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
pub type elf_reloc_type_class = libc::c_uint;
pub const reloc_class_plt: elf_reloc_type_class = 4;
pub const reloc_class_ifunc: elf_reloc_type_class = 3;
pub const reloc_class_copy: elf_reloc_type_class = 2;
pub const reloc_class_relative: elf_reloc_type_class = 1;
pub const reloc_class_normal: elf_reloc_type_class = 0;
pub type Elf_Internal_Note = elf_internal_note;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bfd_elf_section_reloc_data {
    pub hdr: *mut Elf_Internal_Shdr,
    pub count: libc::c_uint,
    pub idx: libc::c_int,
    pub hashes: *mut *mut elf_link_hash_entry,
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
pub type notice_asneeded_action = libc::c_uint;
pub const notice_needed: notice_asneeded_action = 2;
pub const notice_not_needed: notice_asneeded_action = 1;
pub const notice_as_needed: notice_asneeded_action = 0;
pub type elf_target_os = libc::c_uint;
pub const is_nacl: elf_target_os = 3;
pub const is_vxworks: elf_target_os = 2;
pub const is_solaris: elf_target_os = 1;
pub const is_normal: elf_target_os = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nest_funcinfo {
    pub func: *mut funcinfo,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dwarf_block {
    pub size: libc::c_uint,
    pub data: *mut bfd_byte,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_24 {
    pub str_0: *mut libc::c_char,
    pub blk: *mut dwarf_block,
    pub val: bfd_uint64_t,
    pub sval: bfd_int64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct attribute {
    pub name: dwarf_attribute,
    pub form: dwarf_form,
    pub u: C2RustUnnamed_24,
}
pub const DW_OP_addr: dwarf_location_atom = 3;
pub const DW_RLE_startx_length: dwarf_range_list_entry = 3;
pub const DW_RLE_startx_endx: dwarf_range_list_entry = 2;
pub const DW_RLE_base_addressx: dwarf_range_list_entry = 1;
pub const DW_RLE_start_end: dwarf_range_list_entry = 6;
pub const DW_RLE_offset_pair: dwarf_range_list_entry = 4;
pub const DW_RLE_start_length: dwarf_range_list_entry = 7;
pub const DW_RLE_base_address: dwarf_range_list_entry = 5;
pub const DW_RLE_end_of_list: dwarf_range_list_entry = 0;
pub type dwarf_range_list_entry = libc::c_uint;
pub const debug_rnglists: dwarf_debug_section_enum = 12;
pub const debug_ranges: dwarf_debug_section_enum = 11;
pub const DW_LANG_C11: dwarf_source_language = 29;
pub const DW_LANG_UPC: dwarf_source_language = 18;
pub const DW_LANG_PLI: dwarf_source_language = 15;
pub const DW_LANG_Ada95: dwarf_source_language = 13;
pub const DW_LANG_C99: dwarf_source_language = 12;
pub const DW_LANG_Pascal83: dwarf_source_language = 9;
pub const DW_LANG_Fortran77: dwarf_source_language = 7;
pub const DW_LANG_Cobol85: dwarf_source_language = 6;
pub const DW_LANG_Cobol74: dwarf_source_language = 5;
pub const DW_LANG_Ada83: dwarf_source_language = 3;
pub const DW_LANG_C: dwarf_source_language = 2;
pub const DW_LANG_C89: dwarf_source_language = 1;
pub const debug_str_alt: dwarf_debug_section_enum = 16;
pub const debug_line_str: dwarf_debug_section_enum = 17;
pub const debug_str: dwarf_debug_section_enum = 15;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct abbrev_offset_entry {
    pub offset: size_t,
    pub abbrevs: *mut *mut abbrev_info,
}
pub const debug_abbrev: dwarf_debug_section_enum = 0;
pub const DW_UT_type: dwarf_unit_type = 2;
pub type dwarf_unit_type = libc::c_uint;
pub const DW_UT_hi_user: dwarf_unit_type = 255;
pub const DW_UT_lo_user: dwarf_unit_type = 128;
pub const DW_UT_split_type: dwarf_unit_type = 6;
pub const DW_UT_split_compile: dwarf_unit_type = 5;
pub const DW_UT_skeleton: dwarf_unit_type = 4;
pub const DW_UT_partial: dwarf_unit_type = 3;
pub const DW_UT_compile: dwarf_unit_type = 1;
pub const debug_info_alt: dwarf_debug_section_enum = 4;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct line_head {
    pub total_length: bfd_vma,
    pub version: libc::c_ushort,
    pub prologue_length: bfd_vma,
    pub minimum_instruction_length: libc::c_uchar,
    pub maximum_ops_per_insn: libc::c_uchar,
    pub default_is_stmt: libc::c_uchar,
    pub line_base: libc::c_int,
    pub line_range: libc::c_uchar,
    pub opcode_base: libc::c_uchar,
    pub standard_opcode_lengths: *mut libc::c_uchar,
}
pub const DW_LNS_fixed_advance_pc: dwarf_line_number_ops = 9;
pub const DW_LNS_const_add_pc: dwarf_line_number_ops = 8;
pub const DW_LNS_set_basic_block: dwarf_line_number_ops = 7;
pub const DW_LNS_negate_stmt: dwarf_line_number_ops = 6;
pub const DW_LNS_set_column: dwarf_line_number_ops = 5;
pub const DW_LNS_set_file: dwarf_line_number_ops = 4;
pub const DW_LNS_advance_line: dwarf_line_number_ops = 3;
pub const DW_LNS_advance_pc: dwarf_line_number_ops = 2;
pub const DW_LNS_copy: dwarf_line_number_ops = 1;
pub const DW_LNE_HP_source_file_correlation: dwarf_line_number_x_ops = 128;
pub const DW_LNE_set_discriminator: dwarf_line_number_x_ops = 4;
pub const DW_LNE_define_file: dwarf_line_number_x_ops = 3;
pub const DW_LNE_set_address: dwarf_line_number_x_ops = 2;
pub const DW_LNE_end_sequence: dwarf_line_number_x_ops = 1;
pub const DW_LNS_extended_op: dwarf_line_number_ops = 0;
pub const DW_LNCT_MD5: dwarf_line_number_content_type = 5;
pub const DW_LNCT_size: dwarf_line_number_content_type = 4;
pub const DW_LNCT_timestamp: dwarf_line_number_content_type = 3;
pub const DW_LNCT_directory_index: dwarf_line_number_content_type = 2;
pub const DW_LNCT_path: dwarf_line_number_content_type = 1;
pub const debug_line: dwarf_debug_section_enum = 5;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct info_list_node {
    pub next: *mut info_list_node,
    pub info: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct info_hash_entry {
    pub root: bfd_hash_entry,
    pub head: *mut info_list_node,
}
pub const debug_info: dwarf_debug_section_enum = 3;
pub type dwarf_location_atom = libc::c_uint;
pub const DW_OP_AARCH64_operation: dwarf_location_atom = 234;
pub const DW_OP_PGI_omp_thread_num: dwarf_location_atom = 248;
pub const DW_OP_HP_tls: dwarf_location_atom = 230;
pub const DW_OP_HP_unmod_range: dwarf_location_atom = 229;
pub const DW_OP_HP_mod_range: dwarf_location_atom = 228;
pub const DW_OP_HP_fltconst8: dwarf_location_atom = 227;
pub const DW_OP_HP_fltconst4: dwarf_location_atom = 226;
pub const DW_OP_HP_is_value: dwarf_location_atom = 225;
pub const DW_OP_HP_unknown: dwarf_location_atom = 224;
pub const DW_OP_GNU_variable_value: dwarf_location_atom = 253;
pub const DW_OP_GNU_const_index: dwarf_location_atom = 252;
pub const DW_OP_GNU_addr_index: dwarf_location_atom = 251;
pub const DW_OP_GNU_parameter_ref: dwarf_location_atom = 250;
pub const DW_OP_GNU_reinterpret: dwarf_location_atom = 249;
pub const DW_OP_GNU_convert: dwarf_location_atom = 247;
pub const DW_OP_GNU_deref_type: dwarf_location_atom = 246;
pub const DW_OP_GNU_regval_type: dwarf_location_atom = 245;
pub const DW_OP_GNU_const_type: dwarf_location_atom = 244;
pub const DW_OP_GNU_entry_value: dwarf_location_atom = 243;
pub const DW_OP_GNU_implicit_pointer: dwarf_location_atom = 242;
pub const DW_OP_GNU_encoded_addr: dwarf_location_atom = 241;
pub const DW_OP_GNU_uninit: dwarf_location_atom = 240;
pub const DW_OP_GNU_push_tls_address: dwarf_location_atom = 224;
pub const DW_OP_hi_user: dwarf_location_atom = 255;
pub const DW_OP_lo_user: dwarf_location_atom = 224;
pub const DW_OP_reinterpret: dwarf_location_atom = 169;
pub const DW_OP_convert: dwarf_location_atom = 168;
pub const DW_OP_xderef_type: dwarf_location_atom = 167;
pub const DW_OP_deref_type: dwarf_location_atom = 166;
pub const DW_OP_regval_type: dwarf_location_atom = 165;
pub const DW_OP_const_type: dwarf_location_atom = 164;
pub const DW_OP_entry_value: dwarf_location_atom = 163;
pub const DW_OP_constx: dwarf_location_atom = 162;
pub const DW_OP_addrx: dwarf_location_atom = 161;
pub const DW_OP_implicit_pointer: dwarf_location_atom = 160;
pub const DW_OP_stack_value: dwarf_location_atom = 159;
pub const DW_OP_implicit_value: dwarf_location_atom = 158;
pub const DW_OP_bit_piece: dwarf_location_atom = 157;
pub const DW_OP_call_frame_cfa: dwarf_location_atom = 156;
pub const DW_OP_form_tls_address: dwarf_location_atom = 155;
pub const DW_OP_call_ref: dwarf_location_atom = 154;
pub const DW_OP_call4: dwarf_location_atom = 153;
pub const DW_OP_call2: dwarf_location_atom = 152;
pub const DW_OP_push_object_address: dwarf_location_atom = 151;
pub const DW_OP_nop: dwarf_location_atom = 150;
pub const DW_OP_xderef_size: dwarf_location_atom = 149;
pub const DW_OP_deref_size: dwarf_location_atom = 148;
pub const DW_OP_piece: dwarf_location_atom = 147;
pub const DW_OP_bregx: dwarf_location_atom = 146;
pub const DW_OP_fbreg: dwarf_location_atom = 145;
pub const DW_OP_regx: dwarf_location_atom = 144;
pub const DW_OP_breg31: dwarf_location_atom = 143;
pub const DW_OP_breg30: dwarf_location_atom = 142;
pub const DW_OP_breg29: dwarf_location_atom = 141;
pub const DW_OP_breg28: dwarf_location_atom = 140;
pub const DW_OP_breg27: dwarf_location_atom = 139;
pub const DW_OP_breg26: dwarf_location_atom = 138;
pub const DW_OP_breg25: dwarf_location_atom = 137;
pub const DW_OP_breg24: dwarf_location_atom = 136;
pub const DW_OP_breg23: dwarf_location_atom = 135;
pub const DW_OP_breg22: dwarf_location_atom = 134;
pub const DW_OP_breg21: dwarf_location_atom = 133;
pub const DW_OP_breg20: dwarf_location_atom = 132;
pub const DW_OP_breg19: dwarf_location_atom = 131;
pub const DW_OP_breg18: dwarf_location_atom = 130;
pub const DW_OP_breg17: dwarf_location_atom = 129;
pub const DW_OP_breg16: dwarf_location_atom = 128;
pub const DW_OP_breg15: dwarf_location_atom = 127;
pub const DW_OP_breg14: dwarf_location_atom = 126;
pub const DW_OP_breg13: dwarf_location_atom = 125;
pub const DW_OP_breg12: dwarf_location_atom = 124;
pub const DW_OP_breg11: dwarf_location_atom = 123;
pub const DW_OP_breg10: dwarf_location_atom = 122;
pub const DW_OP_breg9: dwarf_location_atom = 121;
pub const DW_OP_breg8: dwarf_location_atom = 120;
pub const DW_OP_breg7: dwarf_location_atom = 119;
pub const DW_OP_breg6: dwarf_location_atom = 118;
pub const DW_OP_breg5: dwarf_location_atom = 117;
pub const DW_OP_breg4: dwarf_location_atom = 116;
pub const DW_OP_breg3: dwarf_location_atom = 115;
pub const DW_OP_breg2: dwarf_location_atom = 114;
pub const DW_OP_breg1: dwarf_location_atom = 113;
pub const DW_OP_breg0: dwarf_location_atom = 112;
pub const DW_OP_reg31: dwarf_location_atom = 111;
pub const DW_OP_reg30: dwarf_location_atom = 110;
pub const DW_OP_reg29: dwarf_location_atom = 109;
pub const DW_OP_reg28: dwarf_location_atom = 108;
pub const DW_OP_reg27: dwarf_location_atom = 107;
pub const DW_OP_reg26: dwarf_location_atom = 106;
pub const DW_OP_reg25: dwarf_location_atom = 105;
pub const DW_OP_reg24: dwarf_location_atom = 104;
pub const DW_OP_reg23: dwarf_location_atom = 103;
pub const DW_OP_reg22: dwarf_location_atom = 102;
pub const DW_OP_reg21: dwarf_location_atom = 101;
pub const DW_OP_reg20: dwarf_location_atom = 100;
pub const DW_OP_reg19: dwarf_location_atom = 99;
pub const DW_OP_reg18: dwarf_location_atom = 98;
pub const DW_OP_reg17: dwarf_location_atom = 97;
pub const DW_OP_reg16: dwarf_location_atom = 96;
pub const DW_OP_reg15: dwarf_location_atom = 95;
pub const DW_OP_reg14: dwarf_location_atom = 94;
pub const DW_OP_reg13: dwarf_location_atom = 93;
pub const DW_OP_reg12: dwarf_location_atom = 92;
pub const DW_OP_reg11: dwarf_location_atom = 91;
pub const DW_OP_reg10: dwarf_location_atom = 90;
pub const DW_OP_reg9: dwarf_location_atom = 89;
pub const DW_OP_reg8: dwarf_location_atom = 88;
pub const DW_OP_reg7: dwarf_location_atom = 87;
pub const DW_OP_reg6: dwarf_location_atom = 86;
pub const DW_OP_reg5: dwarf_location_atom = 85;
pub const DW_OP_reg4: dwarf_location_atom = 84;
pub const DW_OP_reg3: dwarf_location_atom = 83;
pub const DW_OP_reg2: dwarf_location_atom = 82;
pub const DW_OP_reg1: dwarf_location_atom = 81;
pub const DW_OP_reg0: dwarf_location_atom = 80;
pub const DW_OP_lit31: dwarf_location_atom = 79;
pub const DW_OP_lit30: dwarf_location_atom = 78;
pub const DW_OP_lit29: dwarf_location_atom = 77;
pub const DW_OP_lit28: dwarf_location_atom = 76;
pub const DW_OP_lit27: dwarf_location_atom = 75;
pub const DW_OP_lit26: dwarf_location_atom = 74;
pub const DW_OP_lit25: dwarf_location_atom = 73;
pub const DW_OP_lit24: dwarf_location_atom = 72;
pub const DW_OP_lit23: dwarf_location_atom = 71;
pub const DW_OP_lit22: dwarf_location_atom = 70;
pub const DW_OP_lit21: dwarf_location_atom = 69;
pub const DW_OP_lit20: dwarf_location_atom = 68;
pub const DW_OP_lit19: dwarf_location_atom = 67;
pub const DW_OP_lit18: dwarf_location_atom = 66;
pub const DW_OP_lit17: dwarf_location_atom = 65;
pub const DW_OP_lit16: dwarf_location_atom = 64;
pub const DW_OP_lit15: dwarf_location_atom = 63;
pub const DW_OP_lit14: dwarf_location_atom = 62;
pub const DW_OP_lit13: dwarf_location_atom = 61;
pub const DW_OP_lit12: dwarf_location_atom = 60;
pub const DW_OP_lit11: dwarf_location_atom = 59;
pub const DW_OP_lit10: dwarf_location_atom = 58;
pub const DW_OP_lit9: dwarf_location_atom = 57;
pub const DW_OP_lit8: dwarf_location_atom = 56;
pub const DW_OP_lit7: dwarf_location_atom = 55;
pub const DW_OP_lit6: dwarf_location_atom = 54;
pub const DW_OP_lit5: dwarf_location_atom = 53;
pub const DW_OP_lit4: dwarf_location_atom = 52;
pub const DW_OP_lit3: dwarf_location_atom = 51;
pub const DW_OP_lit2: dwarf_location_atom = 50;
pub const DW_OP_lit1: dwarf_location_atom = 49;
pub const DW_OP_lit0: dwarf_location_atom = 48;
pub const DW_OP_skip: dwarf_location_atom = 47;
pub const DW_OP_ne: dwarf_location_atom = 46;
pub const DW_OP_lt: dwarf_location_atom = 45;
pub const DW_OP_le: dwarf_location_atom = 44;
pub const DW_OP_gt: dwarf_location_atom = 43;
pub const DW_OP_ge: dwarf_location_atom = 42;
pub const DW_OP_eq: dwarf_location_atom = 41;
pub const DW_OP_bra: dwarf_location_atom = 40;
pub const DW_OP_xor: dwarf_location_atom = 39;
pub const DW_OP_shra: dwarf_location_atom = 38;
pub const DW_OP_shr: dwarf_location_atom = 37;
pub const DW_OP_shl: dwarf_location_atom = 36;
pub const DW_OP_plus_uconst: dwarf_location_atom = 35;
pub const DW_OP_plus: dwarf_location_atom = 34;
pub const DW_OP_or: dwarf_location_atom = 33;
pub const DW_OP_not: dwarf_location_atom = 32;
pub const DW_OP_neg: dwarf_location_atom = 31;
pub const DW_OP_mul: dwarf_location_atom = 30;
pub const DW_OP_mod: dwarf_location_atom = 29;
pub const DW_OP_minus: dwarf_location_atom = 28;
pub const DW_OP_div: dwarf_location_atom = 27;
pub const DW_OP_and: dwarf_location_atom = 26;
pub const DW_OP_abs: dwarf_location_atom = 25;
pub const DW_OP_xderef: dwarf_location_atom = 24;
pub const DW_OP_rot: dwarf_location_atom = 23;
pub const DW_OP_swap: dwarf_location_atom = 22;
pub const DW_OP_pick: dwarf_location_atom = 21;
pub const DW_OP_over: dwarf_location_atom = 20;
pub const DW_OP_drop: dwarf_location_atom = 19;
pub const DW_OP_dup: dwarf_location_atom = 18;
pub const DW_OP_consts: dwarf_location_atom = 17;
pub const DW_OP_constu: dwarf_location_atom = 16;
pub const DW_OP_const8s: dwarf_location_atom = 15;
pub const DW_OP_const8u: dwarf_location_atom = 14;
pub const DW_OP_const4s: dwarf_location_atom = 13;
pub const DW_OP_const4u: dwarf_location_atom = 12;
pub const DW_OP_const2s: dwarf_location_atom = 11;
pub const DW_OP_const2u: dwarf_location_atom = 10;
pub const DW_OP_const1s: dwarf_location_atom = 9;
pub const DW_OP_const1u: dwarf_location_atom = 8;
pub const DW_OP_deref: dwarf_location_atom = 6;
pub type dwarf_line_number_ops = libc::c_uint;
pub const DW_LNS_set_isa: dwarf_line_number_ops = 12;
pub const DW_LNS_set_epilogue_begin: dwarf_line_number_ops = 11;
pub const DW_LNS_set_prologue_end: dwarf_line_number_ops = 10;
pub type dwarf_line_number_x_ops = libc::c_uint;
pub const DW_LNE_hi_user: dwarf_line_number_x_ops = 255;
pub const DW_LNE_lo_user: dwarf_line_number_x_ops = 128;
pub const DW_LNE_HP_define_proc: dwarf_line_number_x_ops = 32;
pub const DW_LNE_HP_negate_front_end_logical: dwarf_line_number_x_ops = 25;
pub const DW_LNE_HP_negate_function_exit: dwarf_line_number_x_ops = 24;
pub const DW_LNE_HP_negate_post_semantics: dwarf_line_number_x_ops = 23;
pub const DW_LNE_HP_set_sequence: dwarf_line_number_x_ops = 22;
pub const DW_LNE_HP_set_routine_name: dwarf_line_number_x_ops = 21;
pub const DW_LNE_HP_set_file_line_column: dwarf_line_number_x_ops = 20;
pub const DW_LNE_HP_pop_context: dwarf_line_number_x_ops = 19;
pub const DW_LNE_HP_push_context: dwarf_line_number_x_ops = 18;
pub const DW_LNE_HP_negate_is_UV_update: dwarf_line_number_x_ops = 17;
pub type dwarf_line_number_content_type = libc::c_uint;
pub const DW_LNCT_hi_user: dwarf_line_number_content_type = 16383;
pub const DW_LNCT_lo_user: dwarf_line_number_content_type = 8192;
pub type dwarf_source_language = libc::c_uint;
pub const DW_LANG_Rust_old: dwarf_source_language = 36864;
pub const DW_LANG_HP_Assembler: dwarf_source_language = 32775;
pub const DW_LANG_HP_IMacro: dwarf_source_language = 32774;
pub const DW_LANG_HP_Pascal91: dwarf_source_language = 32773;
pub const DW_LANG_HP_Basic91: dwarf_source_language = 32772;
pub const DW_LANG_HP_Bliss: dwarf_source_language = 32771;
pub const DW_LANG_Upc: dwarf_source_language = 34661;
pub const DW_LANG_Mips_Assembler: dwarf_source_language = 32769;
pub const DW_LANG_hi_user: dwarf_source_language = 65535;
pub const DW_LANG_lo_user: dwarf_source_language = 32768;
pub const DW_LANG_RenderScript: dwarf_source_language = 36;
pub const DW_LANG_Fortran08: dwarf_source_language = 35;
pub const DW_LANG_Fortran03: dwarf_source_language = 34;
pub const DW_LANG_C_plus_plus_14: dwarf_source_language = 33;
pub const DW_LANG_Dylan: dwarf_source_language = 32;
pub const DW_LANG_Julia: dwarf_source_language = 31;
pub const DW_LANG_Swift: dwarf_source_language = 30;
pub const DW_LANG_Rust: dwarf_source_language = 28;
pub const DW_LANG_OCaml: dwarf_source_language = 27;
pub const DW_LANG_C_plus_plus_11: dwarf_source_language = 26;
pub const DW_LANG_C_plus_plus_03: dwarf_source_language = 25;
pub const DW_LANG_Haskell: dwarf_source_language = 24;
pub const DW_LANG_Modula3: dwarf_source_language = 23;
pub const DW_LANG_Go: dwarf_source_language = 22;
pub const DW_LANG_OpenCL: dwarf_source_language = 21;
pub const DW_LANG_Python: dwarf_source_language = 20;
pub const DW_LANG_D: dwarf_source_language = 19;
pub const DW_LANG_ObjC_plus_plus: dwarf_source_language = 17;
pub const DW_LANG_ObjC: dwarf_source_language = 16;
pub const DW_LANG_Fortran95: dwarf_source_language = 14;
pub const DW_LANG_Java: dwarf_source_language = 11;
pub const DW_LANG_Modula2: dwarf_source_language = 10;
pub const DW_LANG_Fortran90: dwarf_source_language = 8;
pub const DW_LANG_C_plus_plus: dwarf_source_language = 4;
pub type dwarf_debug_section_enum = libc::c_uint;
pub const debug_max: dwarf_debug_section_enum = 25;
pub const debug_weaknames: dwarf_debug_section_enum = 24;
pub const debug_varnames: dwarf_debug_section_enum = 23;
pub const debug_typenames: dwarf_debug_section_enum = 22;
pub const debug_funcnames: dwarf_debug_section_enum = 21;
pub const debug_srcinfo: dwarf_debug_section_enum = 20;
pub const debug_sfnames: dwarf_debug_section_enum = 19;
pub const debug_types: dwarf_debug_section_enum = 18;
pub const debug_static_vars: dwarf_debug_section_enum = 14;
pub const debug_static_func: dwarf_debug_section_enum = 13;
pub const debug_pubtypes: dwarf_debug_section_enum = 10;
pub const debug_pubnames: dwarf_debug_section_enum = 9;
pub const debug_macro: dwarf_debug_section_enum = 8;
pub const debug_macinfo: dwarf_debug_section_enum = 7;
pub const debug_loc: dwarf_debug_section_enum = 6;
pub const debug_frame: dwarf_debug_section_enum = 2;
pub const debug_aranges: dwarf_debug_section_enum = 1;
#[inline]
unsafe extern "C" fn startswith(
    mut str: *const libc::c_char,
    mut prefix: *const libc::c_char,
) -> bool {
    return strncmp(str, prefix, strlen(prefix)) == 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn bfd_get_outsymbols(mut abfd: *const bfd) -> *mut *mut bfd_symbol {
    return (*abfd).outsymbols;
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
unsafe extern "C" fn bfd_get_section_limit_octets(
    mut abfd: *const bfd,
    mut sec: *const asection,
) -> bfd_size_type {
    if (*abfd).direction() as libc::c_int != write_direction as libc::c_int
        && (*sec).rawsize != 0 as libc::c_int as libc::c_ulong
    {
        return (*sec).rawsize;
    }
    return (*sec).size;
}
#[inline]
unsafe extern "C" fn bfd_get_flavour(mut abfd: *const bfd) -> bfd_flavour {
    return (*(*abfd).xvec).flavour;
}
#[inline]
unsafe extern "C" fn bfd_little_endian(mut abfd: *const bfd) -> bool {
    return (*(*abfd).xvec).byteorder as libc::c_uint
        == BFD_ENDIAN_LITTLE as libc::c_int as libc::c_uint;
}
#[no_mangle]
pub static mut dwarf_debug_sections: [dwarf_debug_section; 26] = [
    {
        let mut init = dwarf_debug_section {
            uncompressed_name: b".debug_abbrev\0" as *const u8 as *const libc::c_char,
            compressed_name: b".zdebug_abbrev\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = dwarf_debug_section {
            uncompressed_name: b".debug_aranges\0" as *const u8 as *const libc::c_char,
            compressed_name: b".zdebug_aranges\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = dwarf_debug_section {
            uncompressed_name: b".debug_frame\0" as *const u8 as *const libc::c_char,
            compressed_name: b".zdebug_frame\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = dwarf_debug_section {
            uncompressed_name: b".debug_info\0" as *const u8 as *const libc::c_char,
            compressed_name: b".zdebug_info\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = dwarf_debug_section {
            uncompressed_name: b".debug_info\0" as *const u8 as *const libc::c_char,
            compressed_name: b".zdebug_info\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = dwarf_debug_section {
            uncompressed_name: b".debug_line\0" as *const u8 as *const libc::c_char,
            compressed_name: b".zdebug_line\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = dwarf_debug_section {
            uncompressed_name: b".debug_loc\0" as *const u8 as *const libc::c_char,
            compressed_name: b".zdebug_loc\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = dwarf_debug_section {
            uncompressed_name: b".debug_macinfo\0" as *const u8 as *const libc::c_char,
            compressed_name: b".zdebug_macinfo\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = dwarf_debug_section {
            uncompressed_name: b".debug_macro\0" as *const u8 as *const libc::c_char,
            compressed_name: b".zdebug_macro\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = dwarf_debug_section {
            uncompressed_name: b".debug_pubnames\0" as *const u8 as *const libc::c_char,
            compressed_name: b".zdebug_pubnames\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = dwarf_debug_section {
            uncompressed_name: b".debug_pubtypes\0" as *const u8 as *const libc::c_char,
            compressed_name: b".zdebug_pubtypes\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = dwarf_debug_section {
            uncompressed_name: b".debug_ranges\0" as *const u8 as *const libc::c_char,
            compressed_name: b".zdebug_ranges\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = dwarf_debug_section {
            uncompressed_name: b".debug_rnglists\0" as *const u8 as *const libc::c_char,
            compressed_name: b".zdebug_rnglist\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = dwarf_debug_section {
            uncompressed_name: b".debug_static_func\0" as *const u8
                as *const libc::c_char,
            compressed_name: b".zdebug_static_func\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = dwarf_debug_section {
            uncompressed_name: b".debug_static_vars\0" as *const u8
                as *const libc::c_char,
            compressed_name: b".zdebug_static_vars\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = dwarf_debug_section {
            uncompressed_name: b".debug_str\0" as *const u8 as *const libc::c_char,
            compressed_name: b".zdebug_str\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = dwarf_debug_section {
            uncompressed_name: b".debug_str\0" as *const u8 as *const libc::c_char,
            compressed_name: b".zdebug_str\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = dwarf_debug_section {
            uncompressed_name: b".debug_line_str\0" as *const u8 as *const libc::c_char,
            compressed_name: b".zdebug_line_str\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = dwarf_debug_section {
            uncompressed_name: b".debug_types\0" as *const u8 as *const libc::c_char,
            compressed_name: b".zdebug_types\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = dwarf_debug_section {
            uncompressed_name: b".debug_sfnames\0" as *const u8 as *const libc::c_char,
            compressed_name: b".zdebug_sfnames\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = dwarf_debug_section {
            uncompressed_name: b".debug_srcinfo\0" as *const u8 as *const libc::c_char,
            compressed_name: b".zebug_srcinfo\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = dwarf_debug_section {
            uncompressed_name: b".debug_funcnames\0" as *const u8 as *const libc::c_char,
            compressed_name: b".zdebug_funcnames\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = dwarf_debug_section {
            uncompressed_name: b".debug_typenames\0" as *const u8 as *const libc::c_char,
            compressed_name: b".zdebug_typenames\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = dwarf_debug_section {
            uncompressed_name: b".debug_varnames\0" as *const u8 as *const libc::c_char,
            compressed_name: b".zdebug_varnames\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = dwarf_debug_section {
            uncompressed_name: b".debug_weaknames\0" as *const u8 as *const libc::c_char,
            compressed_name: b".zdebug_weaknames\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = dwarf_debug_section {
            uncompressed_name: 0 as *const libc::c_char,
            compressed_name: 0 as *const libc::c_char,
        };
        init
    },
];
unsafe extern "C" fn info_hash_table_newfunc(
    mut entry: *mut bfd_hash_entry,
    mut table: *mut bfd_hash_table,
    mut string: *const libc::c_char,
) -> *mut bfd_hash_entry {
    let mut ret: *mut info_hash_entry = entry as *mut info_hash_entry;
    if ret.is_null() {
        ret = bfd_hash_allocate(
            table,
            ::core::mem::size_of::<info_hash_entry>() as libc::c_ulong as libc::c_uint,
        ) as *mut info_hash_entry;
        if ret.is_null() {
            return 0 as *mut bfd_hash_entry;
        }
    }
    ret = bfd_hash_newfunc(ret as *mut bfd_hash_entry, table, string)
        as *mut info_hash_entry;
    if !ret.is_null() {
        (*ret).head = 0 as *mut info_list_node;
    }
    return ret as *mut bfd_hash_entry;
}
unsafe extern "C" fn create_info_hash_table(mut abfd: *mut bfd) -> *mut info_hash_table {
    let mut hash_table: *mut info_hash_table = 0 as *mut info_hash_table;
    hash_table = bfd_alloc(
        abfd,
        ::core::mem::size_of::<info_hash_table>() as libc::c_ulong,
    ) as *mut info_hash_table;
    if hash_table.is_null() {
        return hash_table;
    }
    if !bfd_hash_table_init(
        &mut (*hash_table).base,
        Some(
            info_hash_table_newfunc
                as unsafe extern "C" fn(
                    *mut bfd_hash_entry,
                    *mut bfd_hash_table,
                    *const libc::c_char,
                ) -> *mut bfd_hash_entry,
        ),
        ::core::mem::size_of::<info_hash_entry>() as libc::c_ulong as libc::c_uint,
    ) {
        bfd_release(abfd, hash_table as *mut libc::c_void);
        return 0 as *mut info_hash_table;
    }
    return hash_table;
}
unsafe extern "C" fn insert_info_hash_table(
    mut hash_table: *mut info_hash_table,
    mut key: *const libc::c_char,
    mut info: *mut libc::c_void,
    mut copy_p: bool,
) -> bool {
    let mut entry: *mut info_hash_entry = 0 as *mut info_hash_entry;
    let mut node: *mut info_list_node = 0 as *mut info_list_node;
    entry = bfd_hash_lookup(&mut (*hash_table).base, key, 1 as libc::c_int != 0, copy_p)
        as *mut info_hash_entry;
    if entry.is_null() {
        return 0 as libc::c_int != 0;
    }
    node = bfd_hash_allocate(
        &mut (*hash_table).base,
        ::core::mem::size_of::<info_list_node>() as libc::c_ulong as libc::c_uint,
    ) as *mut info_list_node;
    if node.is_null() {
        return 0 as libc::c_int != 0;
    }
    (*node).info = info;
    (*node).next = (*entry).head;
    (*entry).head = node;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn lookup_info_hash_table(
    mut hash_table: *mut info_hash_table,
    mut key: *const libc::c_char,
) -> *mut info_list_node {
    let mut entry: *mut info_hash_entry = 0 as *mut info_hash_entry;
    entry = bfd_hash_lookup(
        &mut (*hash_table).base,
        key,
        0 as libc::c_int != 0,
        0 as libc::c_int != 0,
    ) as *mut info_hash_entry;
    return if !entry.is_null() { (*entry).head } else { 0 as *mut info_list_node };
}
unsafe extern "C" fn read_section(
    mut abfd: *mut bfd,
    mut sec: *const dwarf_debug_section,
    mut syms: *mut *mut asymbol,
    mut offset: bfd_uint64_t,
    mut section_buffer: *mut *mut bfd_byte,
    mut section_size: *mut bfd_size_type,
) -> bool {
    let mut section_name: *const libc::c_char = (*sec).uncompressed_name;
    let mut contents: *mut bfd_byte = *section_buffer;
    if contents.is_null() {
        let mut amt: bfd_size_type = 0;
        let mut msec: *mut asection = 0 as *mut asection;
        let mut filesize: ufile_ptr = 0;
        msec = bfd_get_section_by_name(abfd, section_name);
        if msec.is_null() {
            section_name = (*sec).compressed_name;
            msec = bfd_get_section_by_name(abfd, section_name);
        }
        if msec.is_null() {
            _bfd_error_handler(
                dcgettext(
                    b"bfd\0" as *const u8 as *const libc::c_char,
                    b"DWARF error: can't find %s section.\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*sec).uncompressed_name,
            );
            bfd_set_error(bfd_error_bad_value);
            return 0 as libc::c_int != 0;
        }
        amt = bfd_get_section_limit_octets(abfd, msec);
        filesize = bfd_get_file_size(abfd);
        if amt >= filesize {
            _bfd_error_handler(
                dcgettext(
                    b"bfd\0" as *const u8 as *const libc::c_char,
                    b"DWARF error: section %s is larger than its filesize! (0x%lx vs 0x%lx)\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                section_name,
                amt as libc::c_long,
                filesize as libc::c_long,
            );
            bfd_set_error(bfd_error_bad_value);
            return 0 as libc::c_int != 0;
        }
        *section_size = amt;
        amt = (amt as libc::c_ulong).wrapping_add(1 as libc::c_int as libc::c_ulong)
            as bfd_size_type as bfd_size_type;
        if amt == 0 as libc::c_int as libc::c_ulong {
            bfd_set_error(bfd_error_no_memory);
            return 0 as libc::c_int != 0;
        }
        contents = bfd_malloc(amt) as *mut bfd_byte;
        if contents.is_null() {
            return 0 as libc::c_int != 0;
        }
        if if !syms.is_null() {
            (bfd_simple_get_relocated_section_contents(abfd, msec, contents, syms))
                .is_null() as libc::c_int
        } else {
            !bfd_get_section_contents(
                abfd,
                msec,
                contents as *mut libc::c_void,
                0 as libc::c_int as file_ptr,
                *section_size,
            ) as libc::c_int
        } != 0
        {
            free(contents as *mut libc::c_void);
            return 0 as libc::c_int != 0;
        }
        *contents.offset(*section_size as isize) = 0 as libc::c_int as bfd_byte;
        *section_buffer = contents;
    }
    if offset != 0 as libc::c_int as libc::c_ulong && offset >= *section_size {
        _bfd_error_handler(
            dcgettext(
                b"bfd\0" as *const u8 as *const libc::c_char,
                b"DWARF error: offset (%lu) greater than or equal to %s size (%lu)\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            offset,
            section_name,
            *section_size,
        );
        bfd_set_error(bfd_error_bad_value);
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
#[inline]
unsafe extern "C" fn read_n_bytes(
    mut abfd: *mut bfd,
    mut ptr: *mut *mut bfd_byte,
    mut end: *mut bfd_byte,
    mut n: libc::c_int,
) -> uint64_t {
    let mut buf: *mut bfd_byte = *ptr;
    if (end.offset_from(buf) as libc::c_long) < n as libc::c_long {
        *ptr = end;
        return 0 as libc::c_int as uint64_t;
    }
    *ptr = buf.offset(n as isize);
    return if n * 8 as libc::c_int == 8 as libc::c_int {
        *(buf as *const libc::c_uchar) as bfd_vma & 0xff as libc::c_int as libc::c_ulong
    } else if n * 8 as libc::c_int == 16 as libc::c_int {
        (Some(((*(*abfd).xvec).bfd_getx16).expect("non-null function pointer")))
            .expect("non-null function pointer")(buf as *const libc::c_void)
    } else if n * 8 as libc::c_int == 32 as libc::c_int {
        (Some(((*(*abfd).xvec).bfd_getx32).expect("non-null function pointer")))
            .expect("non-null function pointer")(buf as *const libc::c_void)
    } else if n * 8 as libc::c_int == 64 as libc::c_int {
        (Some(((*(*abfd).xvec).bfd_getx64).expect("non-null function pointer")))
            .expect("non-null function pointer")(buf as *const libc::c_void)
    } else {
        _bfd_abort(
            b"./dwarf2.c\0" as *const u8 as *const libc::c_char,
            621 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 59],
                &[libc::c_char; 59],
            >(b"uint64_t read_n_bytes(bfd *, bfd_byte **, bfd_byte *, int)\0"))
                .as_ptr(),
        );
        -(1 as libc::c_int) as bfd_vma
    };
}
unsafe extern "C" fn read_1_byte(
    mut abfd: *mut bfd,
    mut ptr: *mut *mut bfd_byte,
    mut end: *mut bfd_byte,
) -> libc::c_uint {
    return read_n_bytes(abfd, ptr, end, 1 as libc::c_int) as libc::c_uint;
}
unsafe extern "C" fn read_1_signed_byte(
    mut abfd: *mut bfd,
    mut ptr: *mut *mut bfd_byte,
    mut end: *mut bfd_byte,
) -> libc::c_int {
    let mut buf: *mut bfd_byte = *ptr;
    if (end.offset_from(buf) as libc::c_long) < 1 as libc::c_int as libc::c_long {
        *ptr = end;
        return 0 as libc::c_int;
    }
    *ptr = buf.offset(1 as libc::c_int as isize);
    return ((*(buf as *const libc::c_uchar) as bfd_signed_vma
        & 0xff as libc::c_int as libc::c_long ^ 0x80 as libc::c_int as libc::c_long)
        - 0x80 as libc::c_int as libc::c_long) as libc::c_int;
}
unsafe extern "C" fn read_2_bytes(
    mut abfd: *mut bfd,
    mut ptr: *mut *mut bfd_byte,
    mut end: *mut bfd_byte,
) -> libc::c_uint {
    return read_n_bytes(abfd, ptr, end, 2 as libc::c_int) as libc::c_uint;
}
unsafe extern "C" fn read_3_bytes(
    mut abfd: *mut bfd,
    mut ptr: *mut *mut bfd_byte,
    mut end: *mut bfd_byte,
) -> libc::c_uint {
    let mut val: libc::c_uint = read_1_byte(abfd, ptr, end);
    val <<= 8 as libc::c_int;
    val |= read_1_byte(abfd, ptr, end);
    val <<= 8 as libc::c_int;
    val |= read_1_byte(abfd, ptr, end);
    if bfd_little_endian(abfd) {
        val = val >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint
            | val & 0xff00 as libc::c_int as libc::c_uint
            | (val & 0xff as libc::c_int as libc::c_uint) << 16 as libc::c_int;
    }
    return val;
}
unsafe extern "C" fn read_4_bytes(
    mut abfd: *mut bfd,
    mut ptr: *mut *mut bfd_byte,
    mut end: *mut bfd_byte,
) -> libc::c_uint {
    return read_n_bytes(abfd, ptr, end, 4 as libc::c_int) as libc::c_uint;
}
unsafe extern "C" fn read_8_bytes(
    mut abfd: *mut bfd,
    mut ptr: *mut *mut bfd_byte,
    mut end: *mut bfd_byte,
) -> uint64_t {
    return read_n_bytes(abfd, ptr, end, 8 as libc::c_int);
}
unsafe extern "C" fn read_blk(
    mut abfd: *mut bfd,
    mut ptr: *mut *mut bfd_byte,
    mut end: *mut bfd_byte,
    mut size: size_t,
) -> *mut dwarf_block {
    let mut buf: *mut bfd_byte = *ptr;
    let mut block: *mut dwarf_block = 0 as *mut dwarf_block;
    block = bfd_alloc(abfd, ::core::mem::size_of::<dwarf_block>() as libc::c_ulong)
        as *mut dwarf_block;
    if block.is_null() {
        return 0 as *mut dwarf_block;
    }
    if size > end.offset_from(buf) as libc::c_long as size_t {
        *ptr = end;
        (*block).data = 0 as *mut bfd_byte;
        (*block).size = 0 as libc::c_int as libc::c_uint;
    } else {
        *ptr = buf.offset(size as isize);
        (*block).data = buf;
        (*block).size = size as libc::c_uint;
    }
    return block;
}
unsafe extern "C" fn read_string(
    mut ptr: *mut *mut bfd_byte,
    mut buf_end: *mut bfd_byte,
) -> *mut libc::c_char {
    let mut buf: *mut bfd_byte = *ptr;
    let mut str: *mut bfd_byte = buf;
    while buf < buf_end {
        let fresh0 = buf;
        buf = buf.offset(1);
        if !(*fresh0 as libc::c_int == 0 as libc::c_int) {
            continue;
        }
        if str == buf.offset(-(1 as libc::c_int as isize)) {
            break;
        }
        *ptr = buf;
        return str as *mut libc::c_char;
    }
    *ptr = buf;
    return 0 as *mut libc::c_char;
}
unsafe extern "C" fn read_indirect_string(
    mut unit: *mut comp_unit,
    mut ptr: *mut *mut bfd_byte,
    mut buf_end: *mut bfd_byte,
) -> *mut libc::c_char {
    let mut offset: bfd_uint64_t = 0;
    let mut stash: *mut dwarf2_debug = (*unit).stash;
    let mut file: *mut dwarf2_debug_file = (*unit).file;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    if (*unit).offset_size as libc::c_ulong
        > buf_end.offset_from(*ptr) as libc::c_long as size_t
    {
        *ptr = buf_end;
        return 0 as *mut libc::c_char;
    }
    if (*unit).offset_size as libc::c_int == 4 as libc::c_int {
        offset = read_4_bytes((*unit).abfd, ptr, buf_end) as bfd_uint64_t;
    } else {
        offset = read_8_bytes((*unit).abfd, ptr, buf_end);
    }
    if !read_section(
        (*unit).abfd,
        &*((*stash).debug_sections).offset(debug_str as libc::c_int as isize),
        (*file).syms,
        offset,
        &mut (*file).dwarf_str_buffer,
        &mut (*file).dwarf_str_size,
    ) {
        return 0 as *mut libc::c_char;
    }
    str = ((*file).dwarf_str_buffer as *mut libc::c_char).offset(offset as isize);
    if *str as libc::c_int == '\0' as i32 {
        return 0 as *mut libc::c_char;
    }
    return str;
}
unsafe extern "C" fn read_indirect_line_string(
    mut unit: *mut comp_unit,
    mut ptr: *mut *mut bfd_byte,
    mut buf_end: *mut bfd_byte,
) -> *mut libc::c_char {
    let mut offset: bfd_uint64_t = 0;
    let mut stash: *mut dwarf2_debug = (*unit).stash;
    let mut file: *mut dwarf2_debug_file = (*unit).file;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    if (*unit).offset_size as libc::c_ulong
        > buf_end.offset_from(*ptr) as libc::c_long as size_t
    {
        *ptr = buf_end;
        return 0 as *mut libc::c_char;
    }
    if (*unit).offset_size as libc::c_int == 4 as libc::c_int {
        offset = read_4_bytes((*unit).abfd, ptr, buf_end) as bfd_uint64_t;
    } else {
        offset = read_8_bytes((*unit).abfd, ptr, buf_end);
    }
    if !read_section(
        (*unit).abfd,
        &*((*stash).debug_sections).offset(debug_line_str as libc::c_int as isize),
        (*file).syms,
        offset,
        &mut (*file).dwarf_line_str_buffer,
        &mut (*file).dwarf_line_str_size,
    ) {
        return 0 as *mut libc::c_char;
    }
    str = ((*file).dwarf_line_str_buffer as *mut libc::c_char).offset(offset as isize);
    if *str as libc::c_int == '\0' as i32 {
        return 0 as *mut libc::c_char;
    }
    return str;
}
unsafe extern "C" fn read_alt_indirect_string(
    mut unit: *mut comp_unit,
    mut ptr: *mut *mut bfd_byte,
    mut buf_end: *mut bfd_byte,
) -> *mut libc::c_char {
    let mut offset: bfd_uint64_t = 0;
    let mut stash: *mut dwarf2_debug = (*unit).stash;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    if (*unit).offset_size as libc::c_ulong
        > buf_end.offset_from(*ptr) as libc::c_long as size_t
    {
        *ptr = buf_end;
        return 0 as *mut libc::c_char;
    }
    if (*unit).offset_size as libc::c_int == 4 as libc::c_int {
        offset = read_4_bytes((*unit).abfd, ptr, buf_end) as bfd_uint64_t;
    } else {
        offset = read_8_bytes((*unit).abfd, ptr, buf_end);
    }
    if ((*stash).alt.bfd_ptr).is_null() {
        let mut debug_bfd: *mut bfd = 0 as *mut bfd;
        let mut debug_filename: *mut libc::c_char = bfd_follow_gnu_debugaltlink(
            (*unit).abfd,
            b"/usr/local/lib/debug\0" as *const u8 as *const libc::c_char,
        );
        if debug_filename.is_null() {
            return 0 as *mut libc::c_char;
        }
        debug_bfd = bfd_openr(debug_filename, 0 as *const libc::c_char);
        free(debug_filename as *mut libc::c_void);
        if debug_bfd.is_null() {
            return 0 as *mut libc::c_char;
        }
        if !bfd_check_format(debug_bfd, bfd_object) {
            bfd_close(debug_bfd);
            return 0 as *mut libc::c_char;
        }
        (*stash).alt.bfd_ptr = debug_bfd;
    }
    if !read_section(
        (*(*unit).stash).alt.bfd_ptr,
        ((*stash).debug_sections).offset(debug_str_alt as libc::c_int as isize),
        (*stash).alt.syms,
        offset,
        &mut (*stash).alt.dwarf_str_buffer,
        &mut (*stash).alt.dwarf_str_size,
    ) {
        return 0 as *mut libc::c_char;
    }
    str = ((*stash).alt.dwarf_str_buffer as *mut libc::c_char).offset(offset as isize);
    if *str as libc::c_int == '\0' as i32 {
        return 0 as *mut libc::c_char;
    }
    return str;
}
unsafe extern "C" fn read_alt_indirect_ref(
    mut unit: *mut comp_unit,
    mut offset: bfd_uint64_t,
) -> *mut bfd_byte {
    let mut stash: *mut dwarf2_debug = (*unit).stash;
    if ((*stash).alt.bfd_ptr).is_null() {
        let mut debug_bfd: *mut bfd = 0 as *mut bfd;
        let mut debug_filename: *mut libc::c_char = bfd_follow_gnu_debugaltlink(
            (*unit).abfd,
            b"/usr/local/lib/debug\0" as *const u8 as *const libc::c_char,
        );
        if debug_filename.is_null() {
            return 0 as *mut bfd_byte;
        }
        debug_bfd = bfd_openr(debug_filename, 0 as *const libc::c_char);
        free(debug_filename as *mut libc::c_void);
        if debug_bfd.is_null() {
            return 0 as *mut bfd_byte;
        }
        if !bfd_check_format(debug_bfd, bfd_object) {
            bfd_close(debug_bfd);
            return 0 as *mut bfd_byte;
        }
        (*stash).alt.bfd_ptr = debug_bfd;
    }
    if !read_section(
        (*(*unit).stash).alt.bfd_ptr,
        ((*stash).debug_sections).offset(debug_info_alt as libc::c_int as isize),
        (*stash).alt.syms,
        offset,
        &mut (*stash).alt.dwarf_info_buffer,
        &mut (*stash).alt.dwarf_info_size,
    ) {
        return 0 as *mut bfd_byte;
    }
    return ((*stash).alt.dwarf_info_buffer).offset(offset as isize);
}
unsafe extern "C" fn read_address(
    mut unit: *mut comp_unit,
    mut ptr: *mut *mut bfd_byte,
    mut buf_end: *mut bfd_byte,
) -> bfd_uint64_t {
    let mut buf: *mut bfd_byte = *ptr;
    let mut signed_vma: libc::c_int = 0 as libc::c_int;
    if bfd_get_flavour((*unit).abfd) as libc::c_uint
        == bfd_target_elf_flavour as libc::c_int as libc::c_uint
    {
        signed_vma = (*((*(*(*unit).abfd).xvec).backend_data as *const elf_backend_data))
            .sign_extend_vma() as libc::c_int;
    }
    if (*unit).addr_size as libc::c_ulong
        > buf_end.offset_from(buf) as libc::c_long as size_t || buf > buf_end
    {
        *ptr = buf_end;
        return 0 as libc::c_int as bfd_uint64_t;
    }
    *ptr = buf.offset((*unit).addr_size as libc::c_int as isize);
    if signed_vma != 0 {
        match (*unit).addr_size as libc::c_int {
            8 => {
                return (Some(
                    ((*(*(*unit).abfd).xvec).bfd_getx_signed_64)
                        .expect("non-null function pointer"),
                ))
                    .expect("non-null function pointer")(buf as *const libc::c_void)
                    as bfd_uint64_t;
            }
            4 => {
                return (Some(
                    ((*(*(*unit).abfd).xvec).bfd_getx_signed_32)
                        .expect("non-null function pointer"),
                ))
                    .expect("non-null function pointer")(buf as *const libc::c_void)
                    as bfd_uint64_t;
            }
            2 => {
                return (Some(
                    ((*(*(*unit).abfd).xvec).bfd_getx_signed_16)
                        .expect("non-null function pointer"),
                ))
                    .expect("non-null function pointer")(buf as *const libc::c_void)
                    as bfd_uint64_t;
            }
            _ => {
                _bfd_abort(
                    b"./dwarf2.c\0" as *const u8 as *const libc::c_char,
                    931 as libc::c_int,
                    (*::core::mem::transmute::<
                        &[u8; 71],
                        &[libc::c_char; 71],
                    >(
                        b"bfd_uint64_t read_address(struct comp_unit *, bfd_byte **, bfd_byte *)\0",
                    ))
                        .as_ptr(),
                );
            }
        }
    } else {
        match (*unit).addr_size as libc::c_int {
            8 => {
                return (Some(
                    ((*(*(*unit).abfd).xvec).bfd_getx64)
                        .expect("non-null function pointer"),
                ))
                    .expect("non-null function pointer")(buf as *const libc::c_void);
            }
            4 => {
                return (Some(
                    ((*(*(*unit).abfd).xvec).bfd_getx32)
                        .expect("non-null function pointer"),
                ))
                    .expect("non-null function pointer")(buf as *const libc::c_void);
            }
            2 => {
                return (Some(
                    ((*(*(*unit).abfd).xvec).bfd_getx16)
                        .expect("non-null function pointer"),
                ))
                    .expect("non-null function pointer")(buf as *const libc::c_void);
            }
            _ => {
                _bfd_abort(
                    b"./dwarf2.c\0" as *const u8 as *const libc::c_char,
                    945 as libc::c_int,
                    (*::core::mem::transmute::<
                        &[u8; 71],
                        &[libc::c_char; 71],
                    >(
                        b"bfd_uint64_t read_address(struct comp_unit *, bfd_byte **, bfd_byte *)\0",
                    ))
                        .as_ptr(),
                );
            }
        }
    };
}
unsafe extern "C" fn lookup_abbrev(
    mut number: libc::c_uint,
    mut abbrevs: *mut *mut abbrev_info,
) -> *mut abbrev_info {
    let mut hash_number: libc::c_uint = 0;
    let mut abbrev: *mut abbrev_info = 0 as *mut abbrev_info;
    hash_number = number.wrapping_rem(121 as libc::c_int as libc::c_uint);
    abbrev = *abbrevs.offset(hash_number as isize);
    while !abbrev.is_null() {
        if (*abbrev).number == number {
            return abbrev
        } else {
            abbrev = (*abbrev).next;
        }
    }
    return 0 as *mut abbrev_info;
}
unsafe extern "C" fn hash_abbrev(mut p: *const libc::c_void) -> hashval_t {
    let mut ent: *const abbrev_offset_entry = p as *const abbrev_offset_entry;
    return htab_hash_pointer
        .expect("non-null function pointer")((*ent).offset as *mut libc::c_void);
}
unsafe extern "C" fn eq_abbrev(
    mut pa: *const libc::c_void,
    mut pb: *const libc::c_void,
) -> libc::c_int {
    let mut a: *const abbrev_offset_entry = pa as *const abbrev_offset_entry;
    let mut b: *const abbrev_offset_entry = pb as *const abbrev_offset_entry;
    return ((*a).offset == (*b).offset) as libc::c_int;
}
unsafe extern "C" fn del_abbrev(mut p: *mut libc::c_void) {
    let mut ent: *mut abbrev_offset_entry = p as *mut abbrev_offset_entry;
    let mut abbrevs: *mut *mut abbrev_info = (*ent).abbrevs;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < 121 as libc::c_int as libc::c_ulong {
        let mut abbrev: *mut abbrev_info = *abbrevs.offset(i as isize);
        while !abbrev.is_null() {
            free((*abbrev).attrs as *mut libc::c_void);
            abbrev = (*abbrev).next;
        }
        i = i.wrapping_add(1);
        i;
    }
    free(ent as *mut libc::c_void);
}
unsafe extern "C" fn read_abbrevs(
    mut abfd: *mut bfd,
    mut offset: bfd_uint64_t,
    mut stash: *mut dwarf2_debug,
    mut file: *mut dwarf2_debug_file,
) -> *mut *mut abbrev_info {
    let mut current_block: u64;
    let mut abbrevs: *mut *mut abbrev_info = 0 as *mut *mut abbrev_info;
    let mut abbrev_ptr: *mut bfd_byte = 0 as *mut bfd_byte;
    let mut abbrev_end: *mut bfd_byte = 0 as *mut bfd_byte;
    let mut cur_abbrev: *mut abbrev_info = 0 as *mut abbrev_info;
    let mut abbrev_number: libc::c_uint = 0;
    let mut abbrev_name: libc::c_uint = 0;
    let mut abbrev_form: libc::c_uint = 0;
    let mut hash_number: libc::c_uint = 0;
    let mut amt: size_t = 0;
    let mut slot: *mut *mut libc::c_void = 0 as *mut *mut libc::c_void;
    let mut ent: abbrev_offset_entry = {
        let mut init = abbrev_offset_entry {
            offset: offset,
            abbrevs: 0 as *mut *mut abbrev_info,
        };
        init
    };
    if ent.offset != offset {
        return 0 as *mut *mut abbrev_info;
    }
    slot = htab_find_slot(
        (*file).abbrev_offsets,
        &mut ent as *mut abbrev_offset_entry as *const libc::c_void,
        INSERT,
    );
    if slot.is_null() {
        return 0 as *mut *mut abbrev_info;
    }
    if !(*slot).is_null() {
        return (*(*slot as *mut abbrev_offset_entry)).abbrevs;
    }
    if !read_section(
        abfd,
        &*((*stash).debug_sections).offset(debug_abbrev as libc::c_int as isize),
        (*file).syms,
        offset,
        &mut (*file).dwarf_abbrev_buffer,
        &mut (*file).dwarf_abbrev_size,
    ) {
        return 0 as *mut *mut abbrev_info;
    }
    amt = (::core::mem::size_of::<*mut abbrev_info>() as libc::c_ulong)
        .wrapping_mul(121 as libc::c_int as libc::c_ulong);
    abbrevs = bfd_zalloc(abfd, amt) as *mut *mut abbrev_info;
    if abbrevs.is_null() {
        return 0 as *mut *mut abbrev_info;
    }
    abbrev_ptr = ((*file).dwarf_abbrev_buffer).offset(offset as isize);
    abbrev_end = ((*file).dwarf_abbrev_buffer)
        .offset((*file).dwarf_abbrev_size as isize);
    abbrev_number = _bfd_safe_read_leb128(
        abfd,
        &mut abbrev_ptr,
        0 as libc::c_int != 0,
        abbrev_end,
    ) as libc::c_uint;
    's_72: loop {
        if !(abbrev_number != 0) {
            current_block = 14832935472441733737;
            break;
        }
        amt = ::core::mem::size_of::<abbrev_info>() as libc::c_ulong;
        cur_abbrev = bfd_zalloc(abfd, amt) as *mut abbrev_info;
        if cur_abbrev.is_null() {
            current_block = 10977827244041522626;
            break;
        }
        (*cur_abbrev).number = abbrev_number;
        (*cur_abbrev)
            .tag = _bfd_safe_read_leb128(
            abfd,
            &mut abbrev_ptr,
            0 as libc::c_int != 0,
            abbrev_end,
        ) as dwarf_tag;
        (*cur_abbrev).has_children = read_1_byte(abfd, &mut abbrev_ptr, abbrev_end) != 0;
        loop {
            let mut implicit_const: bfd_vma = -(1 as libc::c_int) as bfd_vma;
            abbrev_name = _bfd_safe_read_leb128(
                abfd,
                &mut abbrev_ptr,
                0 as libc::c_int != 0,
                abbrev_end,
            ) as libc::c_uint;
            abbrev_form = _bfd_safe_read_leb128(
                abfd,
                &mut abbrev_ptr,
                0 as libc::c_int != 0,
                abbrev_end,
            ) as libc::c_uint;
            if abbrev_form == DW_FORM_implicit_const as libc::c_int as libc::c_uint {
                implicit_const = _bfd_safe_read_leb128(
                    abfd,
                    &mut abbrev_ptr,
                    1 as libc::c_int != 0,
                    abbrev_end,
                );
            }
            if abbrev_name == 0 as libc::c_int as libc::c_uint {
                break;
            }
            if ((*cur_abbrev).num_attrs).wrapping_rem(4 as libc::c_int as libc::c_uint)
                == 0 as libc::c_int as libc::c_uint
            {
                let mut tmp: *mut attr_abbrev = 0 as *mut attr_abbrev;
                amt = ((*cur_abbrev).num_attrs)
                    .wrapping_add(4 as libc::c_int as libc::c_uint) as size_t;
                amt = (amt as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<attr_abbrev>() as libc::c_ulong)
                    as size_t as size_t;
                tmp = bfd_realloc((*cur_abbrev).attrs as *mut libc::c_void, amt)
                    as *mut attr_abbrev;
                if tmp.is_null() {
                    current_block = 10977827244041522626;
                    break 's_72;
                }
                (*cur_abbrev).attrs = tmp;
            }
            (*((*cur_abbrev).attrs).offset((*cur_abbrev).num_attrs as isize))
                .name = abbrev_name as dwarf_attribute;
            (*((*cur_abbrev).attrs).offset((*cur_abbrev).num_attrs as isize))
                .form = abbrev_form as dwarf_form;
            (*((*cur_abbrev).attrs).offset((*cur_abbrev).num_attrs as isize))
                .implicit_const = implicit_const;
            (*cur_abbrev).num_attrs = ((*cur_abbrev).num_attrs).wrapping_add(1);
            (*cur_abbrev).num_attrs;
        }
        hash_number = abbrev_number.wrapping_rem(121 as libc::c_int as libc::c_uint);
        (*cur_abbrev).next = *abbrevs.offset(hash_number as isize);
        let ref mut fresh1 = *abbrevs.offset(hash_number as isize);
        *fresh1 = cur_abbrev;
        if abbrev_ptr.offset_from((*file).dwarf_abbrev_buffer) as libc::c_long as size_t
            >= (*file).dwarf_abbrev_size
        {
            current_block = 14832935472441733737;
            break;
        }
        abbrev_number = _bfd_safe_read_leb128(
            abfd,
            &mut abbrev_ptr,
            0 as libc::c_int != 0,
            abbrev_end,
        ) as libc::c_uint;
        if !(lookup_abbrev(abbrev_number, abbrevs)).is_null() {
            current_block = 14832935472441733737;
            break;
        }
    }
    match current_block {
        14832935472441733737 => {
            *slot = bfd_malloc(
                ::core::mem::size_of::<abbrev_offset_entry>() as libc::c_ulong,
            );
            if !(*slot).is_null() {
                ent.abbrevs = abbrevs;
                memcpy(
                    *slot,
                    &mut ent as *mut abbrev_offset_entry as *const libc::c_void,
                    ::core::mem::size_of::<abbrev_offset_entry>() as libc::c_ulong,
                );
                return abbrevs;
            }
        }
        _ => {}
    }
    if !abbrevs.is_null() {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < 121 as libc::c_int as libc::c_ulong {
            let mut abbrev: *mut abbrev_info = *abbrevs.offset(i as isize);
            while !abbrev.is_null() {
                free((*abbrev).attrs as *mut libc::c_void);
                abbrev = (*abbrev).next;
            }
            i = i.wrapping_add(1);
            i;
        }
        free(abbrevs as *mut libc::c_void);
    }
    return 0 as *mut *mut abbrev_info;
}
#[inline]
unsafe extern "C" fn is_str_attr(mut form: dwarf_form) -> bool {
    return form as libc::c_uint == DW_FORM_string as libc::c_int as libc::c_uint
        || form as libc::c_uint == DW_FORM_strp as libc::c_int as libc::c_uint
        || form as libc::c_uint == DW_FORM_strx as libc::c_int as libc::c_uint
        || form as libc::c_uint == DW_FORM_strx1 as libc::c_int as libc::c_uint
        || form as libc::c_uint == DW_FORM_strx2 as libc::c_int as libc::c_uint
        || form as libc::c_uint == DW_FORM_strx3 as libc::c_int as libc::c_uint
        || form as libc::c_uint == DW_FORM_strx4 as libc::c_int as libc::c_uint
        || form as libc::c_uint == DW_FORM_line_strp as libc::c_int as libc::c_uint
        || form as libc::c_uint == DW_FORM_GNU_strp_alt as libc::c_int as libc::c_uint;
}
unsafe extern "C" fn read_indexed_string(
    mut idx: bfd_uint64_t,
    mut unit: *mut comp_unit,
) -> *const libc::c_char {
    return b"<indexed strings not yet supported>\0" as *const u8 as *const libc::c_char;
}
unsafe extern "C" fn read_attribute_value(
    mut attr: *mut attribute,
    mut form: libc::c_uint,
    mut implicit_const: bfd_vma,
    mut unit: *mut comp_unit,
    mut info_ptr: *mut bfd_byte,
    mut info_ptr_end: *mut bfd_byte,
) -> *mut bfd_byte {
    let mut abfd: *mut bfd = (*unit).abfd;
    let mut amt: size_t = 0;
    if info_ptr >= info_ptr_end
        && form != DW_FORM_flag_present as libc::c_int as libc::c_uint
    {
        _bfd_error_handler(
            dcgettext(
                b"bfd\0" as *const u8 as *const libc::c_char,
                b"DWARF error: info pointer extends beyond end of attributes\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        bfd_set_error(bfd_error_bad_value);
        return info_ptr;
    }
    (*attr).form = form as dwarf_form;
    let mut current_block_62: u64;
    match form {
        25 => {
            (*attr).u.val = 1 as libc::c_int as bfd_uint64_t;
            current_block_62 = 18038362259723567392;
        }
        16 => {
            if (*unit).version >= 3 as libc::c_int {
                if (*unit).offset_size as libc::c_int == 4 as libc::c_int {
                    (*attr)
                        .u
                        .val = read_4_bytes((*unit).abfd, &mut info_ptr, info_ptr_end)
                        as bfd_uint64_t;
                } else {
                    (*attr)
                        .u
                        .val = read_8_bytes((*unit).abfd, &mut info_ptr, info_ptr_end);
                }
                current_block_62 = 18038362259723567392;
            } else {
                current_block_62 = 10106175619532914583;
            }
        }
        1 => {
            current_block_62 = 10106175619532914583;
        }
        7968 | 23 => {
            if (*unit).offset_size as libc::c_int == 4 as libc::c_int {
                (*attr)
                    .u
                    .val = read_4_bytes((*unit).abfd, &mut info_ptr, info_ptr_end)
                    as bfd_uint64_t;
            } else {
                (*attr).u.val = read_8_bytes((*unit).abfd, &mut info_ptr, info_ptr_end);
            }
            current_block_62 = 18038362259723567392;
        }
        3 => {
            amt = read_2_bytes(abfd, &mut info_ptr, info_ptr_end) as size_t;
            (*attr).u.blk = read_blk(abfd, &mut info_ptr, info_ptr_end, amt);
            if ((*attr).u.blk).is_null() {
                return 0 as *mut bfd_byte;
            }
            current_block_62 = 18038362259723567392;
        }
        4 => {
            amt = read_4_bytes(abfd, &mut info_ptr, info_ptr_end) as size_t;
            (*attr).u.blk = read_blk(abfd, &mut info_ptr, info_ptr_end, amt);
            if ((*attr).u.blk).is_null() {
                return 0 as *mut bfd_byte;
            }
            current_block_62 = 18038362259723567392;
        }
        17 | 12 | 11 | 41 => {
            (*attr)
                .u
                .val = read_1_byte(abfd, &mut info_ptr, info_ptr_end) as bfd_uint64_t;
            current_block_62 = 18038362259723567392;
        }
        5 | 18 => {
            (*attr)
                .u
                .val = read_2_bytes(abfd, &mut info_ptr, info_ptr_end) as bfd_uint64_t;
            current_block_62 = 18038362259723567392;
        }
        43 => {
            (*attr)
                .u
                .val = read_3_bytes(abfd, &mut info_ptr, info_ptr_end) as bfd_uint64_t;
            current_block_62 = 18038362259723567392;
        }
        19 | 6 | 44 => {
            (*attr)
                .u
                .val = read_4_bytes(abfd, &mut info_ptr, info_ptr_end) as bfd_uint64_t;
            current_block_62 = 18038362259723567392;
        }
        7 | 20 | 32 => {
            (*attr).u.val = read_8_bytes(abfd, &mut info_ptr, info_ptr_end);
            current_block_62 = 18038362259723567392;
        }
        8 => {
            (*attr).u.str_0 = read_string(&mut info_ptr, info_ptr_end);
            current_block_62 = 18038362259723567392;
        }
        14 => {
            (*attr).u.str_0 = read_indirect_string(unit, &mut info_ptr, info_ptr_end);
            current_block_62 = 18038362259723567392;
        }
        31 => {
            (*attr)
                .u
                .str_0 = read_indirect_line_string(unit, &mut info_ptr, info_ptr_end);
            current_block_62 = 18038362259723567392;
        }
        7969 => {
            (*attr)
                .u
                .str_0 = read_alt_indirect_string(unit, &mut info_ptr, info_ptr_end);
            current_block_62 = 18038362259723567392;
        }
        37 => {
            (*attr)
                .u
                .val = read_1_byte(abfd, &mut info_ptr, info_ptr_end) as bfd_uint64_t;
            (*attr)
                .u
                .str_0 = read_indexed_string((*attr).u.val, unit) as *mut libc::c_char;
            current_block_62 = 18038362259723567392;
        }
        38 => {
            (*attr)
                .u
                .val = read_2_bytes(abfd, &mut info_ptr, info_ptr_end) as bfd_uint64_t;
            (*attr)
                .u
                .str_0 = read_indexed_string((*attr).u.val, unit) as *mut libc::c_char;
            current_block_62 = 18038362259723567392;
        }
        39 => {
            (*attr)
                .u
                .val = read_3_bytes(abfd, &mut info_ptr, info_ptr_end) as bfd_uint64_t;
            (*attr)
                .u
                .str_0 = read_indexed_string((*attr).u.val, unit) as *mut libc::c_char;
            current_block_62 = 18038362259723567392;
        }
        40 => {
            (*attr)
                .u
                .val = read_4_bytes(abfd, &mut info_ptr, info_ptr_end) as bfd_uint64_t;
            (*attr)
                .u
                .str_0 = read_indexed_string((*attr).u.val, unit) as *mut libc::c_char;
            current_block_62 = 18038362259723567392;
        }
        26 => {
            (*attr)
                .u
                .val = _bfd_safe_read_leb128(
                abfd,
                &mut info_ptr,
                0 as libc::c_int != 0,
                info_ptr_end,
            );
            (*attr)
                .u
                .str_0 = read_indexed_string((*attr).u.val, unit) as *mut libc::c_char;
            current_block_62 = 18038362259723567392;
        }
        24 | 9 => {
            amt = _bfd_safe_read_leb128(
                abfd,
                &mut info_ptr,
                0 as libc::c_int != 0,
                info_ptr_end,
            );
            (*attr).u.blk = read_blk(abfd, &mut info_ptr, info_ptr_end, amt);
            if ((*attr).u.blk).is_null() {
                return 0 as *mut bfd_byte;
            }
            current_block_62 = 18038362259723567392;
        }
        10 => {
            amt = read_1_byte(abfd, &mut info_ptr, info_ptr_end) as size_t;
            (*attr).u.blk = read_blk(abfd, &mut info_ptr, info_ptr_end, amt);
            if ((*attr).u.blk).is_null() {
                return 0 as *mut bfd_byte;
            }
            current_block_62 = 18038362259723567392;
        }
        13 => {
            (*attr)
                .u
                .sval = _bfd_safe_read_leb128(
                abfd,
                &mut info_ptr,
                1 as libc::c_int != 0,
                info_ptr_end,
            ) as bfd_int64_t;
            current_block_62 = 18038362259723567392;
        }
        21 | 15 | 27 => {
            (*attr)
                .u
                .val = _bfd_safe_read_leb128(
                abfd,
                &mut info_ptr,
                0 as libc::c_int != 0,
                info_ptr_end,
            );
            current_block_62 = 18038362259723567392;
        }
        22 => {
            form = _bfd_safe_read_leb128(
                abfd,
                &mut info_ptr,
                0 as libc::c_int != 0,
                info_ptr_end,
            ) as libc::c_uint;
            if form == DW_FORM_implicit_const as libc::c_int as libc::c_uint {
                implicit_const = _bfd_safe_read_leb128(
                    abfd,
                    &mut info_ptr,
                    1 as libc::c_int != 0,
                    info_ptr_end,
                );
            }
            info_ptr = read_attribute_value(
                attr,
                form,
                implicit_const,
                unit,
                info_ptr,
                info_ptr_end,
            );
            current_block_62 = 18038362259723567392;
        }
        33 => {
            (*attr).form = DW_FORM_sdata;
            (*attr).u.sval = implicit_const as bfd_int64_t;
            current_block_62 = 18038362259723567392;
        }
        30 => {
            (*attr)
                .u
                .blk = read_blk(
                abfd,
                &mut info_ptr,
                info_ptr_end,
                16 as libc::c_int as size_t,
            );
            if ((*attr).u.blk).is_null() {
                return 0 as *mut bfd_byte;
            }
            current_block_62 = 18038362259723567392;
        }
        _ => {
            _bfd_error_handler(
                dcgettext(
                    b"bfd\0" as *const u8 as *const libc::c_char,
                    b"DWARF error: invalid or unhandled FORM value: %#x\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                form,
            );
            bfd_set_error(bfd_error_bad_value);
            return 0 as *mut bfd_byte;
        }
    }
    match current_block_62 {
        10106175619532914583 => {
            (*attr).u.val = read_address(unit, &mut info_ptr, info_ptr_end);
        }
        _ => {}
    }
    return info_ptr;
}
unsafe extern "C" fn read_attribute(
    mut attr: *mut attribute,
    mut abbrev: *mut attr_abbrev,
    mut unit: *mut comp_unit,
    mut info_ptr: *mut bfd_byte,
    mut info_ptr_end: *mut bfd_byte,
) -> *mut bfd_byte {
    (*attr).name = (*abbrev).name;
    info_ptr = read_attribute_value(
        attr,
        (*abbrev).form as libc::c_uint,
        (*abbrev).implicit_const,
        unit,
        info_ptr,
        info_ptr_end,
    );
    return info_ptr;
}
unsafe extern "C" fn non_mangled(mut lang: libc::c_int) -> bool {
    match lang {
        1 | 2 | 3 | 5 | 6 | 7 | 9 | 12 | 13 | 15 | 18 | 29 => {
            return 1 as libc::c_int != 0;
        }
        _ => return 0 as libc::c_int != 0,
    };
}
#[inline]
unsafe extern "C" fn new_line_sorts_after(
    mut new_line: *mut line_info,
    mut line: *mut line_info,
) -> bool {
    return (*new_line).address > (*line).address
        || (*new_line).address == (*line).address
            && (*new_line).op_index as libc::c_int > (*line).op_index as libc::c_int;
}
unsafe extern "C" fn add_line_info(
    mut table: *mut line_info_table,
    mut address: bfd_vma,
    mut op_index: libc::c_uchar,
    mut filename: *mut libc::c_char,
    mut line: libc::c_uint,
    mut column: libc::c_uint,
    mut discriminator: libc::c_uint,
    mut end_sequence: libc::c_int,
) -> bool {
    let mut amt: size_t = ::core::mem::size_of::<line_info>() as libc::c_ulong;
    let mut seq: *mut line_sequence = (*table).sequences;
    let mut info: *mut line_info = bfd_alloc((*table).abfd, amt) as *mut line_info;
    if info.is_null() {
        return 0 as libc::c_int != 0;
    }
    (*info).prev_line = 0 as *mut line_info;
    (*info).address = address;
    (*info).op_index = op_index;
    (*info).line = line;
    (*info).column = column;
    (*info).discriminator = discriminator;
    (*info).end_sequence = end_sequence as libc::c_uchar;
    if !filename.is_null()
        && *filename.offset(0 as libc::c_int as isize) as libc::c_int != 0
    {
        (*info)
            .filename = bfd_alloc(
            (*table).abfd,
            (strlen(filename)).wrapping_add(1 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
        if ((*info).filename).is_null() {
            return 0 as libc::c_int != 0;
        }
        strcpy((*info).filename, filename);
    } else {
        (*info).filename = 0 as *mut libc::c_char;
    }
    if !seq.is_null() && (*(*seq).last_line).address == address
        && (*(*seq).last_line).op_index as libc::c_int == op_index as libc::c_int
        && (*(*seq).last_line).end_sequence as libc::c_int == end_sequence
    {
        if (*table).lcl_head == (*seq).last_line {
            (*table).lcl_head = info;
        }
        (*info).prev_line = (*(*seq).last_line).prev_line;
        (*seq).last_line = info;
    } else if seq.is_null() || (*(*seq).last_line).end_sequence as libc::c_int != 0 {
        amt = ::core::mem::size_of::<line_sequence>() as libc::c_ulong;
        seq = bfd_malloc(amt) as *mut line_sequence;
        if seq.is_null() {
            return 0 as libc::c_int != 0;
        }
        (*seq).low_pc = address;
        (*seq).prev_sequence = (*table).sequences;
        (*seq).last_line = info;
        (*table).lcl_head = info;
        (*table).sequences = seq;
        (*table).num_sequences = ((*table).num_sequences).wrapping_add(1);
        (*table).num_sequences;
    } else if (*info).end_sequence as libc::c_int != 0
        || new_line_sorts_after(info, (*seq).last_line) as libc::c_int != 0
    {
        (*info).prev_line = (*seq).last_line;
        (*seq).last_line = info;
        if ((*table).lcl_head).is_null() {
            (*table).lcl_head = info;
        }
    } else if !new_line_sorts_after(info, (*table).lcl_head)
        && (((*(*table).lcl_head).prev_line).is_null()
            || new_line_sorts_after(info, (*(*table).lcl_head).prev_line) as libc::c_int
                != 0)
    {
        (*info).prev_line = (*(*table).lcl_head).prev_line;
        (*(*table).lcl_head).prev_line = info;
    } else {
        let mut li2: *mut line_info = (*seq).last_line;
        let mut li1: *mut line_info = (*li2).prev_line;
        while !li1.is_null() {
            if !new_line_sorts_after(info, li2)
                && new_line_sorts_after(info, li1) as libc::c_int != 0
            {
                break;
            }
            li2 = li1;
            li1 = (*li1).prev_line;
        }
        (*table).lcl_head = li2;
        (*info).prev_line = (*(*table).lcl_head).prev_line;
        (*(*table).lcl_head).prev_line = info;
        if address < (*seq).low_pc {
            (*seq).low_pc = address;
        }
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn concat_filename(
    mut table: *mut line_info_table,
    mut file: libc::c_uint,
) -> *mut libc::c_char {
    let mut filename: *mut libc::c_char = 0 as *mut libc::c_char;
    if table.is_null()
        || file.wrapping_sub(1 as libc::c_int as libc::c_uint) >= (*table).num_files
    {
        if file != 0 {
            _bfd_error_handler(
                dcgettext(
                    b"bfd\0" as *const u8 as *const libc::c_char,
                    b"DWARF error: mangled line number section (bad file number)\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
        return strdup(b"<unknown>\0" as *const u8 as *const libc::c_char);
    }
    filename = (*((*table).files)
        .offset(file.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize))
        .name;
    if filename.is_null() {
        return strdup(b"<unknown>\0" as *const u8 as *const libc::c_char);
    }
    if !(*filename.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32
        || *filename.offset(0 as libc::c_int as isize) as libc::c_int == '\\' as i32
            && 0 as libc::c_int != 0
        || *filename.offset(0 as libc::c_int as isize) as libc::c_int != 0
            && *filename.offset(1 as libc::c_int as isize) as libc::c_int == ':' as i32
            && 0 as libc::c_int != 0)
    {
        let mut dir_name: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut subdir_name: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut len: size_t = 0;
        if (*((*table).files)
            .offset(file.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize))
            .dir != 0
            && (*((*table).files)
                .offset(file.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize))
                .dir <= (*table).num_dirs && !((*table).dirs).is_null()
        {
            subdir_name = *((*table).dirs)
                .offset(
                    ((*((*table).files)
                        .offset(
                            file.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
                        ))
                        .dir)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
                );
        }
        if subdir_name.is_null()
            || !(*subdir_name.offset(0 as libc::c_int as isize) as libc::c_int
                == '/' as i32
                || *subdir_name.offset(0 as libc::c_int as isize) as libc::c_int
                    == '\\' as i32 && 0 as libc::c_int != 0
                || *subdir_name.offset(0 as libc::c_int as isize) as libc::c_int != 0
                    && *subdir_name.offset(1 as libc::c_int as isize) as libc::c_int
                        == ':' as i32 && 0 as libc::c_int != 0)
        {
            dir_name = (*table).comp_dir;
        }
        if dir_name.is_null() {
            dir_name = subdir_name;
            subdir_name = 0 as *mut libc::c_char;
        }
        if dir_name.is_null() {
            return strdup(filename);
        }
        len = (strlen(dir_name))
            .wrapping_add(strlen(filename))
            .wrapping_add(2 as libc::c_int as libc::c_ulong);
        if !subdir_name.is_null() {
            len = (len as libc::c_ulong)
                .wrapping_add(
                    (strlen(subdir_name)).wrapping_add(1 as libc::c_int as libc::c_ulong),
                ) as size_t as size_t;
            name = bfd_malloc(len) as *mut libc::c_char;
            if !name.is_null() {
                sprintf(
                    name,
                    b"%s/%s/%s\0" as *const u8 as *const libc::c_char,
                    dir_name,
                    subdir_name,
                    filename,
                );
            }
        } else {
            name = bfd_malloc(len) as *mut libc::c_char;
            if !name.is_null() {
                sprintf(
                    name,
                    b"%s/%s\0" as *const u8 as *const libc::c_char,
                    dir_name,
                    filename,
                );
            }
        }
        return name;
    }
    return strdup(filename);
}
unsafe extern "C" fn arange_add(
    mut unit: *const comp_unit,
    mut first_arange: *mut arange,
    mut low_pc: bfd_vma,
    mut high_pc: bfd_vma,
) -> bool {
    let mut arange: *mut arange = 0 as *mut arange;
    if low_pc == high_pc {
        return 1 as libc::c_int != 0;
    }
    if (*first_arange).high == 0 as libc::c_int as libc::c_ulong {
        (*first_arange).low = low_pc;
        (*first_arange).high = high_pc;
        return 1 as libc::c_int != 0;
    }
    arange = first_arange;
    loop {
        if low_pc == (*arange).high {
            (*arange).high = high_pc;
            return 1 as libc::c_int != 0;
        }
        if high_pc == (*arange).low {
            (*arange).low = low_pc;
            return 1 as libc::c_int != 0;
        }
        arange = (*arange).next;
        if arange.is_null() {
            break;
        }
    }
    arange = bfd_alloc((*unit).abfd, ::core::mem::size_of::<arange>() as libc::c_ulong)
        as *mut arange;
    if arange.is_null() {
        return 0 as libc::c_int != 0;
    }
    (*arange).low = low_pc;
    (*arange).high = high_pc;
    (*arange).next = (*first_arange).next;
    (*first_arange).next = arange;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn compare_sequences(
    mut a: *const libc::c_void,
    mut b: *const libc::c_void,
) -> libc::c_int {
    let mut seq1: *const line_sequence = a as *const line_sequence;
    let mut seq2: *const line_sequence = b as *const line_sequence;
    if (*seq1).low_pc < (*seq2).low_pc {
        return -(1 as libc::c_int);
    }
    if (*seq1).low_pc > (*seq2).low_pc {
        return 1 as libc::c_int;
    }
    if (*(*seq1).last_line).address < (*(*seq2).last_line).address {
        return 1 as libc::c_int;
    }
    if (*(*seq1).last_line).address > (*(*seq2).last_line).address {
        return -(1 as libc::c_int);
    }
    if ((*(*seq1).last_line).op_index as libc::c_int)
        < (*(*seq2).last_line).op_index as libc::c_int
    {
        return 1 as libc::c_int;
    }
    if (*(*seq1).last_line).op_index as libc::c_int
        > (*(*seq2).last_line).op_index as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    if (*seq1).num_lines < (*seq2).num_lines {
        return -(1 as libc::c_int);
    }
    if (*seq1).num_lines > (*seq2).num_lines {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn build_line_info_table(
    mut table: *mut line_info_table,
    mut seq: *mut line_sequence,
) -> bool {
    let mut amt: size_t = 0;
    let mut line_info_lookup: *mut *mut line_info = 0 as *mut *mut line_info;
    let mut each_line: *mut line_info = 0 as *mut line_info;
    let mut num_lines: libc::c_uint = 0;
    let mut line_index: libc::c_uint = 0;
    if !((*seq).line_info_lookup).is_null() {
        return 1 as libc::c_int != 0;
    }
    num_lines = 0 as libc::c_int as libc::c_uint;
    each_line = (*seq).last_line;
    while !each_line.is_null() {
        num_lines = num_lines.wrapping_add(1);
        num_lines;
        each_line = (*each_line).prev_line;
    }
    (*seq).num_lines = num_lines as bfd_size_type;
    if num_lines == 0 as libc::c_int as libc::c_uint {
        return 1 as libc::c_int != 0;
    }
    amt = (::core::mem::size_of::<*mut line_info>() as libc::c_ulong)
        .wrapping_mul(num_lines as libc::c_ulong);
    line_info_lookup = bfd_alloc((*table).abfd, amt) as *mut *mut line_info;
    (*seq).line_info_lookup = line_info_lookup;
    if line_info_lookup.is_null() {
        return 0 as libc::c_int != 0;
    }
    line_index = num_lines;
    each_line = (*seq).last_line;
    while !each_line.is_null() {
        line_index = line_index.wrapping_sub(1);
        let ref mut fresh2 = *line_info_lookup.offset(line_index as isize);
        *fresh2 = each_line;
        each_line = (*each_line).prev_line;
    }
    if !(line_index == 0 as libc::c_int as libc::c_uint) {
        bfd_assert(
            b"./dwarf2.c\0" as *const u8 as *const libc::c_char,
            1841 as libc::c_int,
        );
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn sort_line_sequences(mut table: *mut line_info_table) -> bool {
    let mut amt: size_t = 0;
    let mut sequences: *mut line_sequence = 0 as *mut line_sequence;
    let mut seq: *mut line_sequence = 0 as *mut line_sequence;
    let mut n: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut num_sequences: libc::c_uint = (*table).num_sequences;
    let mut last_high_pc: bfd_vma = 0;
    if num_sequences == 0 as libc::c_int as libc::c_uint {
        return 1 as libc::c_int != 0;
    }
    amt = (::core::mem::size_of::<line_sequence>() as libc::c_ulong)
        .wrapping_mul(num_sequences as libc::c_ulong);
    sequences = bfd_alloc((*table).abfd, amt) as *mut line_sequence;
    if sequences.is_null() {
        return 0 as libc::c_int != 0;
    }
    seq = (*table).sequences;
    n = 0 as libc::c_int as libc::c_uint;
    while n < num_sequences {
        let mut last_seq: *mut line_sequence = seq;
        if seq.is_null() {
            bfd_assert(
                b"./dwarf2.c\0" as *const u8 as *const libc::c_char,
                1872 as libc::c_int,
            );
        }
        (*sequences.offset(n as isize)).low_pc = (*seq).low_pc;
        let ref mut fresh3 = (*sequences.offset(n as isize)).prev_sequence;
        *fresh3 = 0 as *mut line_sequence;
        let ref mut fresh4 = (*sequences.offset(n as isize)).last_line;
        *fresh4 = (*seq).last_line;
        let ref mut fresh5 = (*sequences.offset(n as isize)).line_info_lookup;
        *fresh5 = 0 as *mut *mut line_info;
        (*sequences.offset(n as isize)).num_lines = n as bfd_size_type;
        seq = (*seq).prev_sequence;
        free(last_seq as *mut libc::c_void);
        n = n.wrapping_add(1);
        n;
    }
    if !seq.is_null() {
        bfd_assert(
            b"./dwarf2.c\0" as *const u8 as *const libc::c_char,
            1881 as libc::c_int,
        );
    }
    qsort(
        sequences as *mut libc::c_void,
        n as size_t,
        ::core::mem::size_of::<line_sequence>() as libc::c_ulong,
        Some(
            compare_sequences
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    num_sequences = 1 as libc::c_int as libc::c_uint;
    last_high_pc = (*(*sequences.offset(0 as libc::c_int as isize)).last_line).address;
    let mut current_block_35: u64;
    n = 1 as libc::c_int as libc::c_uint;
    while n < (*table).num_sequences {
        if (*sequences.offset(n as isize)).low_pc < last_high_pc {
            if (*(*sequences.offset(n as isize)).last_line).address <= last_high_pc {
                current_block_35 = 10043043949733653460;
            } else {
                (*sequences.offset(n as isize)).low_pc = last_high_pc;
                current_block_35 = 14648156034262866959;
            }
        } else {
            current_block_35 = 14648156034262866959;
        }
        match current_block_35 {
            14648156034262866959 => {
                last_high_pc = (*(*sequences.offset(n as isize)).last_line).address;
                if n > num_sequences {
                    (*sequences.offset(num_sequences as isize))
                        .low_pc = (*sequences.offset(n as isize)).low_pc;
                    let ref mut fresh6 = (*sequences.offset(num_sequences as isize))
                        .last_line;
                    *fresh6 = (*sequences.offset(n as isize)).last_line;
                }
                num_sequences = num_sequences.wrapping_add(1);
                num_sequences;
            }
            _ => {}
        }
        n = n.wrapping_add(1);
        n;
    }
    (*table).sequences = sequences;
    (*table).num_sequences = num_sequences;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn line_info_add_include_dir(
    mut table: *mut line_info_table,
    mut cur_dir: *mut libc::c_char,
) -> bool {
    if ((*table).num_dirs).wrapping_rem(5 as libc::c_int as libc::c_uint)
        == 0 as libc::c_int as libc::c_uint
    {
        let mut tmp: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
        let mut amt: size_t = 0;
        amt = ((*table).num_dirs).wrapping_add(5 as libc::c_int as libc::c_uint)
            as size_t;
        amt = (amt as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
            as size_t as size_t;
        tmp = bfd_realloc((*table).dirs as *mut libc::c_void, amt)
            as *mut *mut libc::c_char;
        if tmp.is_null() {
            return 0 as libc::c_int != 0;
        }
        (*table).dirs = tmp;
    }
    let fresh7 = (*table).num_dirs;
    (*table).num_dirs = ((*table).num_dirs).wrapping_add(1);
    let ref mut fresh8 = *((*table).dirs).offset(fresh7 as isize);
    *fresh8 = cur_dir;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn line_info_add_include_dir_stub(
    mut table: *mut line_info_table,
    mut cur_dir: *mut libc::c_char,
    mut dir: libc::c_uint,
    mut xtime: libc::c_uint,
    mut size: libc::c_uint,
) -> bool {
    return line_info_add_include_dir(table, cur_dir);
}
unsafe extern "C" fn line_info_add_file_name(
    mut table: *mut line_info_table,
    mut cur_file: *mut libc::c_char,
    mut dir: libc::c_uint,
    mut xtime: libc::c_uint,
    mut size: libc::c_uint,
) -> bool {
    if ((*table).num_files).wrapping_rem(5 as libc::c_int as libc::c_uint)
        == 0 as libc::c_int as libc::c_uint
    {
        let mut tmp: *mut fileinfo = 0 as *mut fileinfo;
        let mut amt: size_t = 0;
        amt = ((*table).num_files).wrapping_add(5 as libc::c_int as libc::c_uint)
            as size_t;
        amt = (amt as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<fileinfo>() as libc::c_ulong) as size_t
            as size_t;
        tmp = bfd_realloc((*table).files as *mut libc::c_void, amt) as *mut fileinfo;
        if tmp.is_null() {
            return 0 as libc::c_int != 0;
        }
        (*table).files = tmp;
    }
    let ref mut fresh9 = (*((*table).files).offset((*table).num_files as isize)).name;
    *fresh9 = cur_file;
    (*((*table).files).offset((*table).num_files as isize)).dir = dir;
    (*((*table).files).offset((*table).num_files as isize)).time = xtime;
    (*((*table).files).offset((*table).num_files as isize)).size = size;
    (*table).num_files = ((*table).num_files).wrapping_add(1);
    (*table).num_files;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn read_formatted_entries(
    mut unit: *mut comp_unit,
    mut bufp: *mut *mut bfd_byte,
    mut buf_end: *mut bfd_byte,
    mut table: *mut line_info_table,
    mut callback: Option::<
        unsafe extern "C" fn(
            *mut line_info_table,
            *mut libc::c_char,
            libc::c_uint,
            libc::c_uint,
            libc::c_uint,
        ) -> bool,
    >,
) -> bool {
    let mut abfd: *mut bfd = (*unit).abfd;
    let mut format_count: bfd_byte = 0;
    let mut formati: bfd_byte = 0;
    let mut data_count: bfd_vma = 0;
    let mut datai: bfd_vma = 0;
    let mut buf: *mut bfd_byte = *bufp;
    let mut format_header_data: *mut bfd_byte = 0 as *mut bfd_byte;
    format_count = read_1_byte(abfd, &mut buf, buf_end) as bfd_byte;
    format_header_data = buf;
    formati = 0 as libc::c_int as bfd_byte;
    while (formati as libc::c_int) < format_count as libc::c_int {
        _bfd_safe_read_leb128(abfd, &mut buf, 0 as libc::c_int != 0, buf_end);
        _bfd_safe_read_leb128(abfd, &mut buf, 0 as libc::c_int != 0, buf_end);
        formati = formati.wrapping_add(1);
        formati;
    }
    data_count = _bfd_safe_read_leb128(abfd, &mut buf, 0 as libc::c_int != 0, buf_end);
    if format_count as libc::c_int == 0 as libc::c_int
        && data_count != 0 as libc::c_int as libc::c_ulong
    {
        _bfd_error_handler(
            dcgettext(
                b"bfd\0" as *const u8 as *const libc::c_char,
                b"DWARF error: zero format count\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        bfd_set_error(bfd_error_bad_value);
        return 0 as libc::c_int != 0;
    }
    if data_count > buf_end.offset_from(buf) as libc::c_long as bfd_vma {
        _bfd_error_handler(
            dcgettext(
                b"bfd\0" as *const u8 as *const libc::c_char,
                b"DWARF error: data count (%lx) larger than buffer size\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            data_count,
        );
        bfd_set_error(bfd_error_bad_value);
        return 0 as libc::c_int != 0;
    }
    datai = 0 as libc::c_int as bfd_vma;
    while datai < data_count {
        let mut format: *mut bfd_byte = format_header_data;
        let mut fe: fileinfo = fileinfo {
            name: 0 as *mut libc::c_char,
            dir: 0,
            time: 0,
            size: 0,
        };
        memset(
            &mut fe as *mut fileinfo as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<fileinfo>() as libc::c_ulong,
        );
        formati = 0 as libc::c_int as bfd_byte;
        while (formati as libc::c_int) < format_count as libc::c_int {
            let mut content_type: bfd_vma = 0;
            let mut form: bfd_vma = 0;
            let mut string_trash: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut stringp: *mut *mut libc::c_char = &mut string_trash;
            let mut uint_trash: libc::c_uint = 0;
            let mut uintp: *mut libc::c_uint = &mut uint_trash;
            let mut attr: attribute = attribute {
                name: 0 as dwarf_attribute,
                form: 0 as dwarf_form,
                u: C2RustUnnamed_24 {
                    str_0: 0 as *mut libc::c_char,
                },
            };
            content_type = _bfd_safe_read_leb128(
                abfd,
                &mut format,
                0 as libc::c_int != 0,
                buf_end,
            );
            match content_type {
                1 => {
                    stringp = &mut fe.name;
                }
                2 => {
                    uintp = &mut fe.dir;
                }
                3 => {
                    uintp = &mut fe.time;
                }
                4 => {
                    uintp = &mut fe.size;
                }
                5 => {}
                _ => {
                    _bfd_error_handler(
                        dcgettext(
                            b"bfd\0" as *const u8 as *const libc::c_char,
                            b"DWARF error: unknown format content type %lu\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        content_type,
                    );
                    bfd_set_error(bfd_error_bad_value);
                    return 0 as libc::c_int != 0;
                }
            }
            form = _bfd_safe_read_leb128(
                abfd,
                &mut format,
                0 as libc::c_int != 0,
                buf_end,
            );
            buf = read_attribute_value(
                &mut attr,
                form as libc::c_uint,
                0 as libc::c_int as bfd_vma,
                unit,
                buf,
                buf_end,
            );
            if buf.is_null() {
                return 0 as libc::c_int != 0;
            }
            match form {
                8 | 31 => {
                    *stringp = attr.u.str_0;
                }
                11 | 5 | 6 | 7 | 15 => {
                    *uintp = attr.u.val as libc::c_uint;
                }
                30 | _ => {}
            }
            formati = formati.wrapping_add(1);
            formati;
        }
        if datai != 0 as libc::c_int as libc::c_ulong {
            if !callback
                .expect(
                    "non-null function pointer",
                )(table, fe.name, fe.dir, fe.time, fe.size)
            {
                return 0 as libc::c_int != 0;
            }
        }
        datai = datai.wrapping_add(1);
        datai;
    }
    *bufp = buf;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn decode_line_info(mut unit: *mut comp_unit) -> *mut line_info_table {
    let mut current_block: u64;
    let mut abfd: *mut bfd = (*unit).abfd;
    let mut stash: *mut dwarf2_debug = (*unit).stash;
    let mut file: *mut dwarf2_debug_file = (*unit).file;
    let mut table: *mut line_info_table = 0 as *mut line_info_table;
    let mut line_ptr: *mut bfd_byte = 0 as *mut bfd_byte;
    let mut line_end: *mut bfd_byte = 0 as *mut bfd_byte;
    let mut lh: line_head = line_head {
        total_length: 0,
        version: 0,
        prologue_length: 0,
        minimum_instruction_length: 0,
        maximum_ops_per_insn: 0,
        default_is_stmt: 0,
        line_base: 0,
        line_range: 0,
        opcode_base: 0,
        standard_opcode_lengths: 0 as *mut libc::c_uchar,
    };
    let mut i: libc::c_uint = 0;
    let mut offset_size: libc::c_uint = 0;
    let mut cur_file: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cur_dir: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut op_code: libc::c_uchar = 0;
    let mut extended_op: libc::c_uchar = 0;
    let mut adj_opcode: libc::c_uchar = 0;
    let mut exop_len: libc::c_uint = 0;
    let mut amt: size_t = 0;
    if (*unit).line_offset == 0 as libc::c_int as libc::c_ulong
        && !((*file).line_table).is_null()
    {
        return (*file).line_table;
    }
    if !read_section(
        abfd,
        &*((*stash).debug_sections).offset(debug_line as libc::c_int as isize),
        (*file).syms,
        (*unit).line_offset,
        &mut (*file).dwarf_line_buffer,
        &mut (*file).dwarf_line_size,
    ) {
        return 0 as *mut line_info_table;
    }
    if (*file).dwarf_line_size < 16 as libc::c_int as libc::c_ulong {
        _bfd_error_handler(
            dcgettext(
                b"bfd\0" as *const u8 as *const libc::c_char,
                b"DWARF error: line info section is too small (%ld)\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            (*file).dwarf_line_size as int64_t,
        );
        bfd_set_error(bfd_error_bad_value);
        return 0 as *mut line_info_table;
    }
    line_ptr = ((*file).dwarf_line_buffer).offset((*unit).line_offset as isize);
    line_end = ((*file).dwarf_line_buffer).offset((*file).dwarf_line_size as isize);
    lh.total_length = read_4_bytes(abfd, &mut line_ptr, line_end) as bfd_vma;
    offset_size = 4 as libc::c_int as libc::c_uint;
    if lh.total_length == 0xffffffff as libc::c_uint as libc::c_ulong {
        lh.total_length = read_8_bytes(abfd, &mut line_ptr, line_end);
        offset_size = 8 as libc::c_int as libc::c_uint;
    } else if lh.total_length == 0 as libc::c_int as libc::c_ulong
        && (*unit).addr_size as libc::c_int == 8 as libc::c_int
    {
        lh.total_length = read_4_bytes(abfd, &mut line_ptr, line_end) as bfd_vma;
        offset_size = 8 as libc::c_int as libc::c_uint;
    }
    if lh.total_length > line_end.offset_from(line_ptr) as libc::c_long as size_t {
        _bfd_error_handler(
            dcgettext(
                b"bfd\0" as *const u8 as *const libc::c_char,
                b"DWARF error: line info data is bigger (%#lx) than the space remaining in the section (%#lx)\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            lh.total_length,
            line_end.offset_from(line_ptr) as libc::c_long as libc::c_ulong,
        );
        bfd_set_error(bfd_error_bad_value);
        return 0 as *mut line_info_table;
    }
    line_end = line_ptr.offset(lh.total_length as isize);
    lh.version = read_2_bytes(abfd, &mut line_ptr, line_end) as libc::c_ushort;
    if (lh.version as libc::c_int) < 2 as libc::c_int
        || lh.version as libc::c_int > 5 as libc::c_int
    {
        _bfd_error_handler(
            dcgettext(
                b"bfd\0" as *const u8 as *const libc::c_char,
                b"DWARF error: unhandled .debug_line version %d\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            lh.version as libc::c_int,
        );
        bfd_set_error(bfd_error_bad_value);
        return 0 as *mut line_info_table;
    }
    if line_ptr
        .offset(offset_size as isize)
        .offset(
            (if lh.version as libc::c_int >= 5 as libc::c_int {
                8 as libc::c_int
            } else {
                (if lh.version as libc::c_int >= 4 as libc::c_int {
                    6 as libc::c_int
                } else {
                    5 as libc::c_int
                })
            }) as isize,
        ) >= line_end
    {
        _bfd_error_handler(
            dcgettext(
                b"bfd\0" as *const u8 as *const libc::c_char,
                b"DWARF error: ran out of room reading prologue\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        bfd_set_error(bfd_error_bad_value);
        return 0 as *mut line_info_table;
    }
    if lh.version as libc::c_int >= 5 as libc::c_int {
        let mut segment_selector_size: libc::c_uint = 0;
        read_1_byte(abfd, &mut line_ptr, line_end);
        segment_selector_size = read_1_byte(abfd, &mut line_ptr, line_end);
        if segment_selector_size != 0 as libc::c_int as libc::c_uint {
            _bfd_error_handler(
                dcgettext(
                    b"bfd\0" as *const u8 as *const libc::c_char,
                    b"DWARF error: line info unsupported segment selector size %u\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                segment_selector_size,
            );
            bfd_set_error(bfd_error_bad_value);
            return 0 as *mut line_info_table;
        }
    }
    if offset_size == 4 as libc::c_int as libc::c_uint {
        lh.prologue_length = read_4_bytes(abfd, &mut line_ptr, line_end) as bfd_vma;
    } else {
        lh.prologue_length = read_8_bytes(abfd, &mut line_ptr, line_end);
    }
    lh
        .minimum_instruction_length = read_1_byte(abfd, &mut line_ptr, line_end)
        as libc::c_uchar;
    if lh.version as libc::c_int >= 4 as libc::c_int {
        lh
            .maximum_ops_per_insn = read_1_byte(abfd, &mut line_ptr, line_end)
            as libc::c_uchar;
    } else {
        lh.maximum_ops_per_insn = 1 as libc::c_int as libc::c_uchar;
    }
    if lh.maximum_ops_per_insn as libc::c_int == 0 as libc::c_int {
        _bfd_error_handler(
            dcgettext(
                b"bfd\0" as *const u8 as *const libc::c_char,
                b"DWARF error: invalid maximum operations per instruction\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        bfd_set_error(bfd_error_bad_value);
        return 0 as *mut line_info_table;
    }
    lh.default_is_stmt = read_1_byte(abfd, &mut line_ptr, line_end) as libc::c_uchar;
    lh.line_base = read_1_signed_byte(abfd, &mut line_ptr, line_end);
    lh.line_range = read_1_byte(abfd, &mut line_ptr, line_end) as libc::c_uchar;
    lh.opcode_base = read_1_byte(abfd, &mut line_ptr, line_end) as libc::c_uchar;
    if line_ptr.offset((lh.opcode_base as libc::c_int - 1 as libc::c_int) as isize)
        >= line_end
    {
        _bfd_error_handler(
            dcgettext(
                b"bfd\0" as *const u8 as *const libc::c_char,
                b"DWARF error: ran out of room reading opcodes\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        bfd_set_error(bfd_error_bad_value);
        return 0 as *mut line_info_table;
    }
    amt = (lh.opcode_base as libc::c_ulong)
        .wrapping_mul(::core::mem::size_of::<libc::c_uchar>() as libc::c_ulong);
    lh.standard_opcode_lengths = bfd_alloc(abfd, amt) as *mut libc::c_uchar;
    *(lh.standard_opcode_lengths)
        .offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_uchar;
    i = 1 as libc::c_int as libc::c_uint;
    while i < lh.opcode_base as libc::c_uint {
        *(lh.standard_opcode_lengths)
            .offset(
                i as isize,
            ) = read_1_byte(abfd, &mut line_ptr, line_end) as libc::c_uchar;
        i = i.wrapping_add(1);
        i;
    }
    amt = ::core::mem::size_of::<line_info_table>() as libc::c_ulong;
    table = bfd_alloc(abfd, amt) as *mut line_info_table;
    if table.is_null() {
        return 0 as *mut line_info_table;
    }
    (*table).abfd = abfd;
    (*table).comp_dir = (*unit).comp_dir;
    (*table).num_files = 0 as libc::c_int as libc::c_uint;
    (*table).files = 0 as *mut fileinfo;
    (*table).num_dirs = 0 as libc::c_int as libc::c_uint;
    (*table).dirs = 0 as *mut *mut libc::c_char;
    (*table).num_sequences = 0 as libc::c_int as libc::c_uint;
    (*table).sequences = 0 as *mut line_sequence;
    (*table).lcl_head = 0 as *mut line_info;
    if lh.version as libc::c_int >= 5 as libc::c_int {
        if !read_formatted_entries(
            unit,
            &mut line_ptr,
            line_end,
            table,
            Some(
                line_info_add_include_dir_stub
                    as unsafe extern "C" fn(
                        *mut line_info_table,
                        *mut libc::c_char,
                        libc::c_uint,
                        libc::c_uint,
                        libc::c_uint,
                    ) -> bool,
            ),
        ) {
            current_block = 1334384470518822623;
        } else if !read_formatted_entries(
            unit,
            &mut line_ptr,
            line_end,
            table,
            Some(
                line_info_add_file_name
                    as unsafe extern "C" fn(
                        *mut line_info_table,
                        *mut libc::c_char,
                        libc::c_uint,
                        libc::c_uint,
                        libc::c_uint,
                    ) -> bool,
            ),
        ) {
            current_block = 1334384470518822623;
        } else {
            current_block = 2168227384378665163;
        }
    } else {
        loop {
            cur_dir = read_string(&mut line_ptr, line_end);
            if cur_dir.is_null() {
                current_block = 2606304779496145856;
                break;
            }
            if !line_info_add_include_dir(table, cur_dir) {
                current_block = 1334384470518822623;
                break;
            }
        }
        match current_block {
            1334384470518822623 => {}
            _ => {
                loop {
                    cur_file = read_string(&mut line_ptr, line_end);
                    if cur_file.is_null() {
                        current_block = 2168227384378665163;
                        break;
                    }
                    let mut dir: libc::c_uint = 0;
                    let mut xtime: libc::c_uint = 0;
                    let mut size: libc::c_uint = 0;
                    dir = _bfd_safe_read_leb128(
                        abfd,
                        &mut line_ptr,
                        0 as libc::c_int != 0,
                        line_end,
                    ) as libc::c_uint;
                    xtime = _bfd_safe_read_leb128(
                        abfd,
                        &mut line_ptr,
                        0 as libc::c_int != 0,
                        line_end,
                    ) as libc::c_uint;
                    size = _bfd_safe_read_leb128(
                        abfd,
                        &mut line_ptr,
                        0 as libc::c_int != 0,
                        line_end,
                    ) as libc::c_uint;
                    if !line_info_add_file_name(table, cur_file, dir, xtime, size) {
                        current_block = 1334384470518822623;
                        break;
                    }
                }
            }
        }
    }
    '_fail: loop {
        match current_block {
            1334384470518822623 => {
                while !((*table).sequences).is_null() {
                    let mut seq: *mut line_sequence = (*table).sequences;
                    (*table).sequences = (*(*table).sequences).prev_sequence;
                    free(seq as *mut libc::c_void);
                }
                break;
            }
            _ => {
                if line_ptr < line_end {
                    let mut address: bfd_vma = 0 as libc::c_int as bfd_vma;
                    let mut op_index: libc::c_uchar = 0 as libc::c_int as libc::c_uchar;
                    let mut filename: *mut libc::c_char = if (*table).num_files != 0 {
                        concat_filename(table, 1 as libc::c_int as libc::c_uint)
                    } else {
                        0 as *mut libc::c_char
                    };
                    let mut line: libc::c_uint = 1 as libc::c_int as libc::c_uint;
                    let mut column: libc::c_uint = 0 as libc::c_int as libc::c_uint;
                    let mut discriminator: libc::c_uint = 0 as libc::c_int
                        as libc::c_uint;
                    let mut is_stmt: libc::c_int = lh.default_is_stmt as libc::c_int;
                    let mut end_sequence: libc::c_int = 0 as libc::c_int;
                    let mut dir_0: libc::c_uint = 0;
                    let mut xtime_0: libc::c_uint = 0;
                    let mut size_0: libc::c_uint = 0;
                    let mut low_pc: bfd_vma = -(1 as libc::c_int) as bfd_vma;
                    let mut high_pc: bfd_vma = 0 as libc::c_int as bfd_vma;
                    while end_sequence == 0 && line_ptr < line_end {
                        op_code = read_1_byte(abfd, &mut line_ptr, line_end)
                            as libc::c_uchar;
                        if op_code as libc::c_int >= lh.opcode_base as libc::c_int {
                            adj_opcode = (op_code as libc::c_int
                                - lh.opcode_base as libc::c_int) as libc::c_uchar;
                            if !(lh.line_range as libc::c_int == 0 as libc::c_int) {
                                if lh.maximum_ops_per_insn as libc::c_int
                                    == 1 as libc::c_int
                                {
                                    address = (address as libc::c_ulong)
                                        .wrapping_add(
                                            (adj_opcode as libc::c_int / lh.line_range as libc::c_int
                                                * lh.minimum_instruction_length as libc::c_int)
                                                as libc::c_ulong,
                                        ) as bfd_vma as bfd_vma;
                                } else {
                                    address = (address as libc::c_ulong)
                                        .wrapping_add(
                                            ((op_index as libc::c_int
                                                + adj_opcode as libc::c_int / lh.line_range as libc::c_int)
                                                / lh.maximum_ops_per_insn as libc::c_int
                                                * lh.minimum_instruction_length as libc::c_int)
                                                as libc::c_ulong,
                                        ) as bfd_vma as bfd_vma;
                                    op_index = ((op_index as libc::c_int
                                        + adj_opcode as libc::c_int / lh.line_range as libc::c_int)
                                        % lh.maximum_ops_per_insn as libc::c_int) as libc::c_uchar;
                                }
                                line = line
                                    .wrapping_add(
                                        (lh.line_base
                                            + adj_opcode as libc::c_int % lh.line_range as libc::c_int)
                                            as libc::c_uint,
                                    );
                                if add_line_info(
                                    table,
                                    address,
                                    op_index,
                                    filename,
                                    line,
                                    column,
                                    discriminator,
                                    0 as libc::c_int,
                                ) {
                                    discriminator = 0 as libc::c_int as libc::c_uint;
                                    if address < low_pc {
                                        low_pc = address;
                                    }
                                    if address > high_pc {
                                        high_pc = address;
                                    }
                                    continue;
                                }
                            }
                        } else {
                            match op_code as libc::c_int {
                                0 => {
                                    exop_len = _bfd_safe_read_leb128(
                                        abfd,
                                        &mut line_ptr,
                                        0 as libc::c_int != 0,
                                        line_end,
                                    ) as libc::c_uint;
                                    extended_op = read_1_byte(abfd, &mut line_ptr, line_end)
                                        as libc::c_uchar;
                                    match extended_op as libc::c_int {
                                        1 => {
                                            end_sequence = 1 as libc::c_int;
                                            if add_line_info(
                                                table,
                                                address,
                                                op_index,
                                                filename,
                                                line,
                                                column,
                                                discriminator,
                                                end_sequence,
                                            ) {
                                                discriminator = 0 as libc::c_int as libc::c_uint;
                                                if address < low_pc {
                                                    low_pc = address;
                                                }
                                                if address > high_pc {
                                                    high_pc = address;
                                                }
                                                if arange_add(unit, &mut (*unit).arange, low_pc, high_pc) {
                                                    continue;
                                                }
                                            }
                                        }
                                        2 => {
                                            address = read_address(unit, &mut line_ptr, line_end);
                                            op_index = 0 as libc::c_int as libc::c_uchar;
                                            continue;
                                        }
                                        3 => {
                                            cur_file = read_string(&mut line_ptr, line_end);
                                            dir_0 = _bfd_safe_read_leb128(
                                                abfd,
                                                &mut line_ptr,
                                                0 as libc::c_int != 0,
                                                line_end,
                                            ) as libc::c_uint;
                                            xtime_0 = _bfd_safe_read_leb128(
                                                abfd,
                                                &mut line_ptr,
                                                0 as libc::c_int != 0,
                                                line_end,
                                            ) as libc::c_uint;
                                            size_0 = _bfd_safe_read_leb128(
                                                abfd,
                                                &mut line_ptr,
                                                0 as libc::c_int != 0,
                                                line_end,
                                            ) as libc::c_uint;
                                            if line_info_add_file_name(
                                                table,
                                                cur_file,
                                                dir_0,
                                                xtime_0,
                                                size_0,
                                            ) {
                                                continue;
                                            }
                                        }
                                        4 => {
                                            discriminator = _bfd_safe_read_leb128(
                                                abfd,
                                                &mut line_ptr,
                                                0 as libc::c_int != 0,
                                                line_end,
                                            ) as libc::c_uint;
                                            continue;
                                        }
                                        128 => {
                                            line_ptr = line_ptr
                                                .offset(
                                                    exop_len.wrapping_sub(1 as libc::c_int as libc::c_uint)
                                                        as isize,
                                                );
                                            continue;
                                        }
                                        _ => {
                                            _bfd_error_handler(
                                                dcgettext(
                                                    b"bfd\0" as *const u8 as *const libc::c_char,
                                                    b"DWARF error: mangled line number section\0" as *const u8
                                                        as *const libc::c_char,
                                                    5 as libc::c_int,
                                                ),
                                            );
                                            bfd_set_error(bfd_error_bad_value);
                                        }
                                    }
                                }
                                1 => {
                                    if add_line_info(
                                        table,
                                        address,
                                        op_index,
                                        filename,
                                        line,
                                        column,
                                        discriminator,
                                        0 as libc::c_int,
                                    ) {
                                        discriminator = 0 as libc::c_int as libc::c_uint;
                                        if address < low_pc {
                                            low_pc = address;
                                        }
                                        if address > high_pc {
                                            high_pc = address;
                                        }
                                        continue;
                                    }
                                }
                                2 => {
                                    if lh.maximum_ops_per_insn as libc::c_int
                                        == 1 as libc::c_int
                                    {
                                        address = (address as libc::c_ulong)
                                            .wrapping_add(
                                                (lh.minimum_instruction_length as libc::c_ulong)
                                                    .wrapping_mul(
                                                        _bfd_safe_read_leb128(
                                                            abfd,
                                                            &mut line_ptr,
                                                            0 as libc::c_int != 0,
                                                            line_end,
                                                        ),
                                                    ),
                                            ) as bfd_vma as bfd_vma;
                                    } else {
                                        let mut adjust: bfd_vma = _bfd_safe_read_leb128(
                                            abfd,
                                            &mut line_ptr,
                                            0 as libc::c_int != 0,
                                            line_end,
                                        );
                                        address = (op_index as libc::c_ulong)
                                            .wrapping_add(adjust)
                                            .wrapping_div(lh.maximum_ops_per_insn as libc::c_ulong)
                                            .wrapping_mul(
                                                lh.minimum_instruction_length as libc::c_ulong,
                                            );
                                        op_index = (op_index as libc::c_ulong)
                                            .wrapping_add(adjust)
                                            .wrapping_rem(lh.maximum_ops_per_insn as libc::c_ulong)
                                            as libc::c_uchar;
                                    }
                                    continue;
                                }
                                3 => {
                                    line = (line as libc::c_ulong)
                                        .wrapping_add(
                                            _bfd_safe_read_leb128(
                                                abfd,
                                                &mut line_ptr,
                                                1 as libc::c_int != 0,
                                                line_end,
                                            ),
                                        ) as libc::c_uint as libc::c_uint;
                                    continue;
                                }
                                4 => {
                                    let mut filenum: libc::c_uint = 0;
                                    filenum = _bfd_safe_read_leb128(
                                        abfd,
                                        &mut line_ptr,
                                        0 as libc::c_int != 0,
                                        line_end,
                                    ) as libc::c_uint;
                                    free(filename as *mut libc::c_void);
                                    filename = concat_filename(table, filenum);
                                    continue;
                                }
                                5 => {
                                    column = _bfd_safe_read_leb128(
                                        abfd,
                                        &mut line_ptr,
                                        0 as libc::c_int != 0,
                                        line_end,
                                    ) as libc::c_uint;
                                    continue;
                                }
                                6 => {
                                    is_stmt = (is_stmt == 0) as libc::c_int;
                                    continue;
                                }
                                7 => {
                                    continue;
                                }
                                8 => {
                                    if !(lh.line_range as libc::c_int == 0 as libc::c_int) {
                                        if lh.maximum_ops_per_insn as libc::c_int
                                            == 1 as libc::c_int
                                        {
                                            address = (address as libc::c_ulong)
                                                .wrapping_add(
                                                    (lh.minimum_instruction_length as libc::c_int
                                                        * ((255 as libc::c_int - lh.opcode_base as libc::c_int)
                                                            / lh.line_range as libc::c_int)) as libc::c_ulong,
                                                ) as bfd_vma as bfd_vma;
                                        } else {
                                            let mut adjust_0: bfd_vma = ((255 as libc::c_int
                                                - lh.opcode_base as libc::c_int)
                                                / lh.line_range as libc::c_int) as bfd_vma;
                                            address = (address as libc::c_ulong)
                                                .wrapping_add(
                                                    (lh.minimum_instruction_length as libc::c_ulong)
                                                        .wrapping_mul(
                                                            (op_index as libc::c_ulong)
                                                                .wrapping_add(adjust_0)
                                                                .wrapping_div(lh.maximum_ops_per_insn as libc::c_ulong),
                                                        ),
                                                ) as bfd_vma as bfd_vma;
                                            op_index = (op_index as libc::c_ulong)
                                                .wrapping_add(adjust_0)
                                                .wrapping_rem(lh.maximum_ops_per_insn as libc::c_ulong)
                                                as libc::c_uchar;
                                        }
                                        continue;
                                    }
                                }
                                9 => {
                                    address = (address as libc::c_ulong)
                                        .wrapping_add(
                                            read_2_bytes(abfd, &mut line_ptr, line_end) as libc::c_ulong,
                                        ) as bfd_vma as bfd_vma;
                                    op_index = 0 as libc::c_int as libc::c_uchar;
                                    continue;
                                }
                                _ => {
                                    i = 0 as libc::c_int as libc::c_uint;
                                    while i
                                        < *(lh.standard_opcode_lengths).offset(op_code as isize)
                                            as libc::c_uint
                                    {
                                        _bfd_safe_read_leb128(
                                            abfd,
                                            &mut line_ptr,
                                            0 as libc::c_int != 0,
                                            line_end,
                                        );
                                        i = i.wrapping_add(1);
                                        i;
                                    }
                                    continue;
                                }
                            }
                        }
                        free(filename as *mut libc::c_void);
                        current_block = 1334384470518822623;
                        continue '_fail;
                    }
                    free(filename as *mut libc::c_void);
                    current_block = 2168227384378665163;
                } else {
                    if (*unit).line_offset == 0 as libc::c_int as libc::c_ulong {
                        (*file).line_table = table;
                    }
                    if sort_line_sequences(table) {
                        return table;
                    }
                    current_block = 1334384470518822623;
                }
            }
        }
    }
    free((*table).files as *mut libc::c_void);
    free((*table).dirs as *mut libc::c_void);
    return 0 as *mut line_info_table;
}
unsafe extern "C" fn lookup_address_in_line_info_table(
    mut table: *mut line_info_table,
    mut addr: bfd_vma,
    mut filename_ptr: *mut *const libc::c_char,
    mut linenumber_ptr: *mut libc::c_uint,
    mut discriminator_ptr: *mut libc::c_uint,
) -> bfd_vma {
    let mut seq: *mut line_sequence = 0 as *mut line_sequence;
    let mut info: *mut line_info = 0 as *mut line_info;
    let mut low: libc::c_int = 0;
    let mut high: libc::c_int = 0;
    let mut mid: libc::c_int = 0;
    low = 0 as libc::c_int;
    high = (*table).num_sequences as libc::c_int;
    while low < high {
        mid = (low + high) / 2 as libc::c_int;
        seq = &mut *((*table).sequences).offset(mid as isize) as *mut line_sequence;
        if addr < (*seq).low_pc {
            high = mid;
        } else {
            if !(addr >= (*(*seq).last_line).address) {
                break;
            }
            low = mid + 1 as libc::c_int;
        }
    }
    if !(seq.is_null() || addr < (*seq).low_pc || addr >= (*(*seq).last_line).address) {
        if build_line_info_table(table, seq) {
            low = 0 as libc::c_int;
            high = (*seq).num_lines as libc::c_int;
            info = 0 as *mut line_info;
            while low < high {
                mid = (low + high) / 2 as libc::c_int;
                info = *((*seq).line_info_lookup).offset(mid as isize);
                if addr < (*info).address {
                    high = mid;
                } else {
                    if !(addr
                        >= (**((*seq).line_info_lookup)
                            .offset((mid + 1 as libc::c_int) as isize))
                            .address)
                    {
                        break;
                    }
                    low = mid + 1 as libc::c_int;
                }
            }
            if !info.is_null() && addr >= (*info).address
                && addr
                    < (**((*seq).line_info_lookup)
                        .offset((mid + 1 as libc::c_int) as isize))
                        .address
                && !((*info).end_sequence as libc::c_int != 0
                    || info == (*seq).last_line)
            {
                *filename_ptr = (*info).filename;
                *linenumber_ptr = (*info).line;
                if !discriminator_ptr.is_null() {
                    *discriminator_ptr = (*info).discriminator;
                }
                return ((*(*seq).last_line).address).wrapping_sub((*seq).low_pc);
            }
        }
    }
    *filename_ptr = 0 as *const libc::c_char;
    return 0 as libc::c_int as bfd_vma;
}
unsafe extern "C" fn read_debug_ranges(mut unit: *mut comp_unit) -> bool {
    let mut stash: *mut dwarf2_debug = (*unit).stash;
    let mut file: *mut dwarf2_debug_file = (*unit).file;
    return read_section(
        (*unit).abfd,
        &*((*stash).debug_sections).offset(debug_ranges as libc::c_int as isize),
        (*file).syms,
        0 as libc::c_int as bfd_uint64_t,
        &mut (*file).dwarf_ranges_buffer,
        &mut (*file).dwarf_ranges_size,
    );
}
unsafe extern "C" fn read_debug_rnglists(mut unit: *mut comp_unit) -> bool {
    let mut stash: *mut dwarf2_debug = (*unit).stash;
    let mut file: *mut dwarf2_debug_file = (*unit).file;
    return read_section(
        (*unit).abfd,
        &*((*stash).debug_sections).offset(debug_rnglists as libc::c_int as isize),
        (*file).syms,
        0 as libc::c_int as bfd_uint64_t,
        &mut (*file).dwarf_rnglists_buffer,
        &mut (*file).dwarf_rnglists_size,
    );
}
unsafe extern "C" fn compare_lookup_funcinfos(
    mut a: *const libc::c_void,
    mut b: *const libc::c_void,
) -> libc::c_int {
    let mut lookup1: *const lookup_funcinfo = a as *const lookup_funcinfo;
    let mut lookup2: *const lookup_funcinfo = b as *const lookup_funcinfo;
    if (*lookup1).low_addr < (*lookup2).low_addr {
        return -(1 as libc::c_int);
    }
    if (*lookup1).low_addr > (*lookup2).low_addr {
        return 1 as libc::c_int;
    }
    if (*lookup1).high_addr < (*lookup2).high_addr {
        return -(1 as libc::c_int);
    }
    if (*lookup1).high_addr > (*lookup2).high_addr {
        return 1 as libc::c_int;
    }
    if (*lookup1).idx < (*lookup2).idx {
        return -(1 as libc::c_int);
    }
    if (*lookup1).idx > (*lookup2).idx {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn build_lookup_funcinfo_table(mut unit: *mut comp_unit) -> bool {
    let mut lookup_funcinfo_table: *mut lookup_funcinfo = (*unit).lookup_funcinfo_table;
    let mut number_of_functions: libc::c_uint = (*unit).number_of_functions
        as libc::c_uint;
    let mut each: *mut funcinfo = 0 as *mut funcinfo;
    let mut entry: *mut lookup_funcinfo = 0 as *mut lookup_funcinfo;
    let mut func_index: size_t = 0;
    let mut range: *mut arange = 0 as *mut arange;
    let mut low_addr: bfd_vma = 0;
    let mut high_addr: bfd_vma = 0;
    if !lookup_funcinfo_table.is_null()
        || number_of_functions == 0 as libc::c_int as libc::c_uint
    {
        return 1 as libc::c_int != 0;
    }
    lookup_funcinfo_table = bfd_malloc(
        (number_of_functions as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<lookup_funcinfo>() as libc::c_ulong),
    ) as *mut lookup_funcinfo;
    if lookup_funcinfo_table.is_null() {
        return 0 as libc::c_int != 0;
    }
    func_index = number_of_functions as size_t;
    each = (*unit).function_table;
    while !each.is_null() {
        func_index = func_index.wrapping_sub(1);
        entry = &mut *lookup_funcinfo_table.offset(func_index as isize)
            as *mut lookup_funcinfo;
        (*entry).funcinfo = each;
        (*entry).idx = func_index as libc::c_uint;
        low_addr = (*(*entry).funcinfo).arange.low;
        high_addr = (*(*entry).funcinfo).arange.high;
        range = (*(*entry).funcinfo).arange.next;
        while !range.is_null() {
            if (*range).low < low_addr {
                low_addr = (*range).low;
            }
            if (*range).high > high_addr {
                high_addr = (*range).high;
            }
            range = (*range).next;
        }
        (*entry).low_addr = low_addr;
        (*entry).high_addr = high_addr;
        each = (*each).prev_func;
    }
    if !(func_index == 0 as libc::c_int as libc::c_ulong) {
        bfd_assert(
            b"./dwarf2.c\0" as *const u8 as *const libc::c_char,
            2664 as libc::c_int,
        );
    }
    qsort(
        lookup_funcinfo_table as *mut libc::c_void,
        number_of_functions as size_t,
        ::core::mem::size_of::<lookup_funcinfo>() as libc::c_ulong,
        Some(
            compare_lookup_funcinfos
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    high_addr = (*lookup_funcinfo_table.offset(0 as libc::c_int as isize)).high_addr;
    func_index = 1 as libc::c_int as size_t;
    while func_index < number_of_functions as libc::c_ulong {
        entry = &mut *lookup_funcinfo_table.offset(func_index as isize)
            as *mut lookup_funcinfo;
        if (*entry).high_addr > high_addr {
            high_addr = (*entry).high_addr;
        } else {
            (*entry).high_addr = high_addr;
        }
        func_index = func_index.wrapping_add(1);
        func_index;
    }
    (*unit).lookup_funcinfo_table = lookup_funcinfo_table;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn lookup_address_in_function_table(
    mut unit: *mut comp_unit,
    mut addr: bfd_vma,
    mut function_ptr: *mut *mut funcinfo,
) -> bool {
    let mut number_of_functions: libc::c_uint = (*unit).number_of_functions
        as libc::c_uint;
    let mut lookup_funcinfo: *mut lookup_funcinfo = 0 as *mut lookup_funcinfo;
    let mut funcinfo: *mut funcinfo = 0 as *mut funcinfo;
    let mut best_fit: *mut funcinfo = 0 as *mut funcinfo;
    let mut best_fit_len: bfd_vma = 0 as libc::c_int as bfd_vma;
    let mut low: bfd_size_type = 0;
    let mut high: bfd_size_type = 0;
    let mut mid: bfd_size_type = 0;
    let mut first: bfd_size_type = 0;
    let mut arange: *mut arange = 0 as *mut arange;
    if number_of_functions == 0 as libc::c_int as libc::c_uint {
        return 0 as libc::c_int != 0;
    }
    if !build_lookup_funcinfo_table(unit) {
        return 0 as libc::c_int != 0;
    }
    if (*((*unit).lookup_funcinfo_table)
        .offset(
            number_of_functions.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
        ))
        .high_addr < addr
    {
        return 0 as libc::c_int != 0;
    }
    low = 0 as libc::c_int as bfd_size_type;
    high = number_of_functions as bfd_size_type;
    first = high;
    while low < high {
        mid = low.wrapping_add(high).wrapping_div(2 as libc::c_int as libc::c_ulong);
        lookup_funcinfo = &mut *((*unit).lookup_funcinfo_table).offset(mid as isize)
            as *mut lookup_funcinfo;
        if addr < (*lookup_funcinfo).low_addr {
            high = mid;
        } else if addr >= (*lookup_funcinfo).high_addr {
            low = mid.wrapping_add(1 as libc::c_int as libc::c_ulong);
        } else {
            first = mid;
            high = first;
        }
    }
    while first < number_of_functions as libc::c_ulong {
        if addr < (*((*unit).lookup_funcinfo_table).offset(first as isize)).low_addr {
            break;
        }
        funcinfo = (*((*unit).lookup_funcinfo_table).offset(first as isize)).funcinfo;
        arange = &mut (*funcinfo).arange;
        while !arange.is_null() {
            if !(addr < (*arange).low || addr >= (*arange).high) {
                if best_fit.is_null()
                    || ((*arange).high).wrapping_sub((*arange).low) < best_fit_len
                    || ((*arange).high).wrapping_sub((*arange).low) == best_fit_len
                        && funcinfo > best_fit
                {
                    best_fit = funcinfo;
                    best_fit_len = ((*arange).high).wrapping_sub((*arange).low);
                }
            }
            arange = (*arange).next;
        }
        first = first.wrapping_add(1);
        first;
    }
    if best_fit.is_null() {
        return 0 as libc::c_int != 0;
    }
    *function_ptr = best_fit;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn lookup_symbol_in_function_table(
    mut unit: *mut comp_unit,
    mut sym: *mut asymbol,
    mut addr: bfd_vma,
    mut filename_ptr: *mut *const libc::c_char,
    mut linenumber_ptr: *mut libc::c_uint,
) -> bool {
    let mut each_func: *mut funcinfo = 0 as *mut funcinfo;
    let mut best_fit: *mut funcinfo = 0 as *mut funcinfo;
    let mut best_fit_len: bfd_vma = 0 as libc::c_int as bfd_vma;
    let mut arange: *mut arange = 0 as *mut arange;
    let mut name: *const libc::c_char = bfd_asymbol_name(sym);
    let mut sec: *mut asection = bfd_asymbol_section(sym);
    each_func = (*unit).function_table;
    while !each_func.is_null() {
        arange = &mut (*each_func).arange;
        while !arange.is_null() {
            if (((*each_func).sec).is_null() || (*each_func).sec == sec)
                && addr >= (*arange).low && addr < (*arange).high
                && !((*each_func).name).is_null()
                && strcmp(name, (*each_func).name) == 0 as libc::c_int
                && (best_fit.is_null()
                    || ((*arange).high).wrapping_sub((*arange).low) < best_fit_len)
            {
                best_fit = each_func;
                best_fit_len = ((*arange).high).wrapping_sub((*arange).low);
            }
            arange = (*arange).next;
        }
        each_func = (*each_func).prev_func;
    }
    if !best_fit.is_null() {
        (*best_fit).sec = sec;
        *filename_ptr = (*best_fit).file;
        *linenumber_ptr = (*best_fit).line as libc::c_uint;
        return 1 as libc::c_int != 0;
    } else {
        return 0 as libc::c_int != 0
    };
}
unsafe extern "C" fn lookup_symbol_in_variable_table(
    mut unit: *mut comp_unit,
    mut sym: *mut asymbol,
    mut addr: bfd_vma,
    mut filename_ptr: *mut *const libc::c_char,
    mut linenumber_ptr: *mut libc::c_uint,
) -> bool {
    let mut name: *const libc::c_char = bfd_asymbol_name(sym);
    let mut sec: *mut asection = bfd_asymbol_section(sym);
    let mut each: *mut varinfo = 0 as *mut varinfo;
    each = (*unit).variable_table;
    while !each.is_null() {
        if !(*each).stack && !((*each).file).is_null() && !((*each).name).is_null()
            && (*each).addr == addr && (((*each).sec).is_null() || (*each).sec == sec)
            && strcmp(name, (*each).name) == 0 as libc::c_int
        {
            break;
        }
        each = (*each).prev_var;
    }
    if !each.is_null() {
        (*each).sec = sec;
        *filename_ptr = (*each).file;
        *linenumber_ptr = (*each).line as libc::c_uint;
        return 1 as libc::c_int != 0;
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn find_abstract_instance(
    mut unit: *mut comp_unit,
    mut attr_ptr: *mut attribute,
    mut recur_count: libc::c_uint,
    mut pname: *mut *const libc::c_char,
    mut is_linkage: *mut bool,
    mut filename_ptr: *mut *mut libc::c_char,
    mut linenumber_ptr: *mut libc::c_int,
) -> bool {
    let mut abfd: *mut bfd = (*unit).abfd;
    let mut info_ptr: *mut bfd_byte = 0 as *mut bfd_byte;
    let mut info_ptr_end: *mut bfd_byte = 0 as *mut bfd_byte;
    let mut abbrev_number: libc::c_uint = 0;
    let mut i: libc::c_uint = 0;
    let mut abbrev: *mut abbrev_info = 0 as *mut abbrev_info;
    let mut die_ref: bfd_uint64_t = (*attr_ptr).u.val;
    let mut attr: attribute = attribute {
        name: 0 as dwarf_attribute,
        form: 0 as dwarf_form,
        u: C2RustUnnamed_24 {
            str_0: 0 as *mut libc::c_char,
        },
    };
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    if recur_count == 100 as libc::c_int as libc::c_uint {
        _bfd_error_handler(
            dcgettext(
                b"bfd\0" as *const u8 as *const libc::c_char,
                b"DWARF error: abstract instance recursion detected\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        bfd_set_error(bfd_error_bad_value);
        return 0 as libc::c_int != 0;
    }
    if (*attr_ptr).form as libc::c_uint
        == DW_FORM_ref_addr as libc::c_int as libc::c_uint
    {
        let mut total: size_t = 0;
        info_ptr = (*(*unit).file).dwarf_info_buffer;
        info_ptr_end = info_ptr.offset((*(*unit).file).dwarf_info_size as isize);
        total = info_ptr_end.offset_from(info_ptr) as libc::c_long as size_t;
        if die_ref == 0 {
            return 1 as libc::c_int != 0
        } else if die_ref >= total {
            _bfd_error_handler(
                dcgettext(
                    b"bfd\0" as *const u8 as *const libc::c_char,
                    b"DWARF error: invalid abstract instance DIE ref\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            bfd_set_error(bfd_error_bad_value);
            return 0 as libc::c_int != 0;
        }
        info_ptr = info_ptr.offset(die_ref as isize);
    } else if (*attr_ptr).form as libc::c_uint
        == DW_FORM_GNU_ref_alt as libc::c_int as libc::c_uint
    {
        let mut first_time: bool = ((*(*unit).stash).alt.dwarf_info_buffer).is_null();
        info_ptr = read_alt_indirect_ref(unit, die_ref);
        if first_time {
            (*(*unit).stash).alt.info_ptr = (*(*unit).stash).alt.dwarf_info_buffer;
        }
        if info_ptr.is_null() {
            _bfd_error_handler(
                dcgettext(
                    b"bfd\0" as *const u8 as *const libc::c_char,
                    b"DWARF error: unable to read alt ref %lu\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                die_ref,
            );
            bfd_set_error(bfd_error_bad_value);
            return 0 as libc::c_int != 0;
        }
        info_ptr_end = ((*(*unit).stash).alt.dwarf_info_buffer)
            .offset((*(*unit).stash).alt.dwarf_info_size as isize);
        if !((*(*unit).stash).alt.all_comp_units).is_null() {
            unit = (*(*unit).stash).alt.all_comp_units;
        }
    }
    if (*attr_ptr).form as libc::c_uint
        == DW_FORM_ref_addr as libc::c_int as libc::c_uint
        || (*attr_ptr).form as libc::c_uint
            == DW_FORM_GNU_ref_alt as libc::c_int as libc::c_uint
    {
        if info_ptr >= (*unit).info_ptr_unit && info_ptr < (*unit).end_ptr {
            info_ptr_end = (*unit).end_ptr;
        } else {
            let mut u: *mut comp_unit = 0 as *mut comp_unit;
            u = (*unit).prev_unit;
            while !u.is_null() {
                if info_ptr >= (*u).info_ptr_unit && info_ptr < (*u).end_ptr {
                    break;
                }
                u = (*u).prev_unit;
            }
            if u.is_null() {
                u = (*unit).next_unit;
                while !u.is_null() {
                    if info_ptr >= (*u).info_ptr_unit && info_ptr < (*u).end_ptr {
                        break;
                    }
                    u = (*u).next_unit;
                }
            }
            if (*attr_ptr).form as libc::c_uint
                == DW_FORM_ref_addr as libc::c_int as libc::c_uint
            {
                while u.is_null() {
                    u = stash_comp_unit((*unit).stash, &mut (*(*unit).stash).f);
                    if u.is_null() {
                        break;
                    }
                    if info_ptr >= (*u).info_ptr_unit && info_ptr < (*u).end_ptr {
                        break;
                    }
                    u = 0 as *mut comp_unit;
                }
            }
            if (*attr_ptr).form as libc::c_uint
                == DW_FORM_GNU_ref_alt as libc::c_int as libc::c_uint
            {
                while u.is_null() {
                    u = stash_comp_unit((*unit).stash, &mut (*(*unit).stash).alt);
                    if u.is_null() {
                        break;
                    }
                    if info_ptr >= (*u).info_ptr_unit && info_ptr < (*u).end_ptr {
                        break;
                    }
                    u = 0 as *mut comp_unit;
                }
            }
            if u.is_null() {
                _bfd_error_handler(
                    dcgettext(
                        b"bfd\0" as *const u8 as *const libc::c_char,
                        b"DWARF error: unable to locate abstract instance DIE ref %lu\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    die_ref,
                );
                bfd_set_error(bfd_error_bad_value);
                return 0 as libc::c_int != 0;
            }
            unit = u;
            info_ptr_end = (*unit).end_ptr;
        }
    } else {
        let mut total_0: size_t = 0;
        info_ptr = (*unit).info_ptr_unit;
        info_ptr_end = (*unit).end_ptr;
        total_0 = info_ptr_end.offset_from(info_ptr) as libc::c_long as size_t;
        if die_ref == 0 || die_ref >= total_0 {
            _bfd_error_handler(
                dcgettext(
                    b"bfd\0" as *const u8 as *const libc::c_char,
                    b"DWARF error: invalid abstract instance DIE ref\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            bfd_set_error(bfd_error_bad_value);
            return 0 as libc::c_int != 0;
        }
        info_ptr = info_ptr.offset(die_ref as isize);
    }
    abbrev_number = _bfd_safe_read_leb128(
        abfd,
        &mut info_ptr,
        0 as libc::c_int != 0,
        info_ptr_end,
    ) as libc::c_uint;
    if abbrev_number != 0 {
        abbrev = lookup_abbrev(abbrev_number, (*unit).abbrevs);
        if abbrev.is_null() {
            _bfd_error_handler(
                dcgettext(
                    b"bfd\0" as *const u8 as *const libc::c_char,
                    b"DWARF error: could not find abbrev number %u\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                abbrev_number,
            );
            bfd_set_error(bfd_error_bad_value);
            return 0 as libc::c_int != 0;
        } else {
            i = 0 as libc::c_int as libc::c_uint;
            while i < (*abbrev).num_attrs {
                info_ptr = read_attribute(
                    &mut attr,
                    &mut *((*abbrev).attrs).offset(i as isize),
                    unit,
                    info_ptr,
                    info_ptr_end,
                );
                if info_ptr.is_null() {
                    break;
                }
                match attr.name as libc::c_uint {
                    3 => {
                        if name.is_null() && is_str_attr(attr.form) as libc::c_int != 0 {
                            name = attr.u.str_0;
                            if non_mangled((*unit).lang) {
                                *is_linkage = 1 as libc::c_int != 0;
                            }
                        }
                    }
                    71 => {
                        if !find_abstract_instance(
                            unit,
                            &mut attr,
                            recur_count.wrapping_add(1 as libc::c_int as libc::c_uint),
                            &mut name,
                            is_linkage,
                            filename_ptr,
                            linenumber_ptr,
                        ) {
                            return 0 as libc::c_int != 0;
                        }
                    }
                    110 | 8199 => {
                        if is_str_attr(attr.form) {
                            name = attr.u.str_0;
                            *is_linkage = 1 as libc::c_int != 0;
                        }
                    }
                    58 => {
                        if !comp_unit_maybe_decode_line_info(unit) {
                            return 0 as libc::c_int != 0;
                        }
                        *filename_ptr = concat_filename(
                            (*unit).line_table,
                            attr.u.val as libc::c_uint,
                        );
                    }
                    59 => {
                        *linenumber_ptr = attr.u.val as libc::c_int;
                    }
                    _ => {}
                }
                i = i.wrapping_add(1);
                i;
            }
        }
    }
    *pname = name;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn read_ranges(
    mut unit: *mut comp_unit,
    mut arange: *mut arange,
    mut offset: bfd_uint64_t,
) -> bool {
    let mut ranges_ptr: *mut bfd_byte = 0 as *mut bfd_byte;
    let mut ranges_end: *mut bfd_byte = 0 as *mut bfd_byte;
    let mut base_address: bfd_vma = (*unit).base_address;
    if ((*(*unit).file).dwarf_ranges_buffer).is_null() {
        if !read_debug_ranges(unit) {
            return 0 as libc::c_int != 0;
        }
    }
    ranges_ptr = ((*(*unit).file).dwarf_ranges_buffer).offset(offset as isize);
    if ranges_ptr < (*(*unit).file).dwarf_ranges_buffer {
        return 0 as libc::c_int != 0;
    }
    ranges_end = ((*(*unit).file).dwarf_ranges_buffer)
        .offset((*(*unit).file).dwarf_ranges_size as isize);
    if ranges_ptr >= ranges_end {
        return 0 as libc::c_int != 0;
    }
    loop {
        let mut low_pc: bfd_vma = 0;
        let mut high_pc: bfd_vma = 0;
        if (2 as libc::c_uint).wrapping_mul((*unit).addr_size as libc::c_uint)
            as libc::c_ulong
            > ranges_end.offset_from(ranges_ptr) as libc::c_long as size_t
        {
            return 0 as libc::c_int != 0;
        }
        low_pc = read_address(unit, &mut ranges_ptr, ranges_end);
        high_pc = read_address(unit, &mut ranges_ptr, ranges_end);
        if low_pc == 0 as libc::c_int as libc::c_ulong
            && high_pc == 0 as libc::c_int as libc::c_ulong
        {
            break;
        }
        if low_pc == (1 as libc::c_ulong).wrapping_neg()
            && high_pc != (1 as libc::c_ulong).wrapping_neg()
        {
            base_address = high_pc;
        } else if !arange_add(
            unit,
            arange,
            base_address.wrapping_add(low_pc),
            base_address.wrapping_add(high_pc),
        ) {
            return 0 as libc::c_int != 0
        }
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn read_rnglists(
    mut unit: *mut comp_unit,
    mut arange: *mut arange,
    mut offset: bfd_uint64_t,
) -> bool {
    let mut rngs_ptr: *mut bfd_byte = 0 as *mut bfd_byte;
    let mut rngs_end: *mut bfd_byte = 0 as *mut bfd_byte;
    let mut base_address: bfd_vma = (*unit).base_address;
    let mut low_pc: bfd_vma = 0;
    let mut high_pc: bfd_vma = 0;
    let mut abfd: *mut bfd = (*unit).abfd;
    if ((*(*unit).file).dwarf_rnglists_buffer).is_null() {
        if !read_debug_rnglists(unit) {
            return 0 as libc::c_int != 0;
        }
    }
    rngs_ptr = ((*(*unit).file).dwarf_rnglists_buffer).offset(offset as isize);
    if rngs_ptr < (*(*unit).file).dwarf_rnglists_buffer {
        return 0 as libc::c_int != 0;
    }
    rngs_end = (*(*unit).file).dwarf_rnglists_buffer;
    rngs_end = rngs_end.offset((*(*unit).file).dwarf_rnglists_size as isize);
    loop {
        let mut rlet: dwarf_range_list_entry = DW_RLE_end_of_list;
        if rngs_ptr >= rngs_end {
            return 0 as libc::c_int != 0;
        }
        rlet = read_1_byte(abfd, &mut rngs_ptr, rngs_end) as dwarf_range_list_entry;
        match rlet as libc::c_uint {
            0 => return 1 as libc::c_int != 0,
            5 => {
                if (*unit).addr_size as libc::c_ulong
                    > rngs_end.offset_from(rngs_ptr) as libc::c_long as size_t
                {
                    return 0 as libc::c_int != 0;
                }
                base_address = read_address(unit, &mut rngs_ptr, rngs_end);
                continue;
            }
            7 => {
                if (*unit).addr_size as libc::c_ulong
                    > rngs_end.offset_from(rngs_ptr) as libc::c_long as size_t
                {
                    return 0 as libc::c_int != 0;
                }
                low_pc = read_address(unit, &mut rngs_ptr, rngs_end);
                high_pc = low_pc;
                high_pc = (high_pc as libc::c_ulong)
                    .wrapping_add(
                        _bfd_safe_read_leb128(
                            abfd,
                            &mut rngs_ptr,
                            0 as libc::c_int != 0,
                            rngs_end,
                        ),
                    ) as bfd_vma as bfd_vma;
            }
            4 => {
                low_pc = base_address;
                low_pc = (low_pc as libc::c_ulong)
                    .wrapping_add(
                        _bfd_safe_read_leb128(
                            abfd,
                            &mut rngs_ptr,
                            0 as libc::c_int != 0,
                            rngs_end,
                        ),
                    ) as bfd_vma as bfd_vma;
                high_pc = base_address;
                high_pc = (high_pc as libc::c_ulong)
                    .wrapping_add(
                        _bfd_safe_read_leb128(
                            abfd,
                            &mut rngs_ptr,
                            0 as libc::c_int != 0,
                            rngs_end,
                        ),
                    ) as bfd_vma as bfd_vma;
            }
            6 => {
                if (2 as libc::c_uint).wrapping_mul((*unit).addr_size as libc::c_uint)
                    as libc::c_ulong
                    > rngs_end.offset_from(rngs_ptr) as libc::c_long as size_t
                {
                    return 0 as libc::c_int != 0;
                }
                low_pc = read_address(unit, &mut rngs_ptr, rngs_end);
                high_pc = read_address(unit, &mut rngs_ptr, rngs_end);
            }
            1 | 2 | 3 | _ => return 0 as libc::c_int != 0,
        }
        if !arange_add(unit, arange, low_pc, high_pc) {
            return 0 as libc::c_int != 0;
        }
    };
}
unsafe extern "C" fn read_rangelist(
    mut unit: *mut comp_unit,
    mut arange: *mut arange,
    mut offset: bfd_uint64_t,
) -> bool {
    if (*unit).version <= 4 as libc::c_int {
        return read_ranges(unit, arange, offset)
    } else {
        return read_rnglists(unit, arange, offset)
    };
}
unsafe extern "C" fn lookup_func_by_offset(
    mut offset: bfd_uint64_t,
    mut table: *mut funcinfo,
) -> *mut funcinfo {
    while !table.is_null() {
        if (*table).unit_offset == offset {
            return table;
        }
        table = (*table).prev_func;
    }
    return 0 as *mut funcinfo;
}
unsafe extern "C" fn lookup_var_by_offset(
    mut offset: bfd_uint64_t,
    mut table: *mut varinfo,
) -> *mut varinfo {
    while !table.is_null() {
        if (*table).unit_offset == offset {
            return table;
        }
        table = (*table).prev_var;
    }
    return 0 as *mut varinfo;
}
static mut previous_failed_abbrev: libc::c_uint = 0;
unsafe extern "C" fn scan_unit_for_symbols(mut unit: *mut comp_unit) -> bool {
    let mut current_block: u64;
    let mut abfd: *mut bfd = (*unit).abfd;
    let mut info_ptr: *mut bfd_byte = (*unit).first_child_die_ptr;
    let mut info_ptr_end: *mut bfd_byte = (*unit).end_ptr;
    let mut nesting_level: libc::c_int = 0 as libc::c_int;
    let mut nested_funcs: *mut nest_funcinfo = 0 as *mut nest_funcinfo;
    let mut nested_funcs_size: libc::c_int = 0;
    nested_funcs_size = 32 as libc::c_int;
    nested_funcs = bfd_malloc(
        (nested_funcs_size as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<nest_funcinfo>() as libc::c_ulong),
    ) as *mut nest_funcinfo;
    if nested_funcs.is_null() {
        return 0 as libc::c_int != 0;
    }
    let ref mut fresh10 = (*nested_funcs.offset(nesting_level as isize)).func;
    *fresh10 = 0 as *mut funcinfo;
    's_29: loop {
        if !(nesting_level >= 0 as libc::c_int) {
            current_block = 8545136480011357681;
            break;
        }
        let mut abbrev_number: libc::c_uint = 0;
        let mut i: libc::c_uint = 0;
        let mut abbrev: *mut abbrev_info = 0 as *mut abbrev_info;
        let mut func: *mut funcinfo = 0 as *mut funcinfo;
        let mut var: *mut varinfo = 0 as *mut varinfo;
        let mut current_offset: bfd_uint64_t = 0;
        if info_ptr >= info_ptr_end {
            current_block = 15191033149860947047;
            break;
        }
        current_offset = info_ptr.offset_from((*unit).info_ptr_unit) as libc::c_long
            as bfd_uint64_t;
        abbrev_number = _bfd_safe_read_leb128(
            abfd,
            &mut info_ptr,
            0 as libc::c_int != 0,
            info_ptr_end,
        ) as libc::c_uint;
        if abbrev_number == 0 as libc::c_int as libc::c_uint {
            nesting_level -= 1;
            nesting_level;
        } else {
            abbrev = lookup_abbrev(abbrev_number, (*unit).abbrevs);
            if abbrev.is_null() {
                if abbrev_number != previous_failed_abbrev {
                    _bfd_error_handler(
                        dcgettext(
                            b"bfd\0" as *const u8 as *const libc::c_char,
                            b"DWARF error: could not find abbrev number %u\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        abbrev_number,
                    );
                    previous_failed_abbrev = abbrev_number;
                }
                bfd_set_error(bfd_error_bad_value);
                current_block = 15191033149860947047;
                break;
            } else {
                if (*abbrev).tag as libc::c_uint
                    == DW_TAG_subprogram as libc::c_int as libc::c_uint
                    || (*abbrev).tag as libc::c_uint
                        == DW_TAG_entry_point as libc::c_int as libc::c_uint
                    || (*abbrev).tag as libc::c_uint
                        == DW_TAG_inlined_subroutine as libc::c_int as libc::c_uint
                {
                    let mut amt: size_t = ::core::mem::size_of::<funcinfo>()
                        as libc::c_ulong;
                    var = 0 as *mut varinfo;
                    func = bfd_zalloc(abfd, amt) as *mut funcinfo;
                    if func.is_null() {
                        current_block = 15191033149860947047;
                        break;
                    }
                    (*func).tag = (*abbrev).tag as libc::c_int;
                    (*func).prev_func = (*unit).function_table;
                    (*func).unit_offset = current_offset;
                    (*unit).function_table = func;
                    (*unit)
                        .number_of_functions = ((*unit).number_of_functions)
                        .wrapping_add(1);
                    (*unit).number_of_functions;
                    if (*unit).cached {
                        bfd_assert(
                            b"./dwarf2.c\0" as *const u8 as *const libc::c_char,
                            3328 as libc::c_int,
                        );
                    }
                    if (*func).tag == DW_TAG_inlined_subroutine as libc::c_int {
                        i = nesting_level as libc::c_uint;
                        loop {
                            let fresh11 = i;
                            i = i.wrapping_sub(1);
                            if !(fresh11 != 0 as libc::c_int as libc::c_uint) {
                                break;
                            }
                            if ((*nested_funcs.offset(i as isize)).func).is_null() {
                                continue;
                            }
                            (*func)
                                .caller_func = (*nested_funcs.offset(i as isize)).func;
                            break;
                        }
                    }
                    let ref mut fresh12 = (*nested_funcs.offset(nesting_level as isize))
                        .func;
                    *fresh12 = func;
                } else {
                    func = 0 as *mut funcinfo;
                    if (*abbrev).tag as libc::c_uint
                        == DW_TAG_variable as libc::c_int as libc::c_uint
                        || (*abbrev).tag as libc::c_uint
                            == DW_TAG_member as libc::c_int as libc::c_uint
                    {
                        let mut amt_0: size_t = ::core::mem::size_of::<varinfo>()
                            as libc::c_ulong;
                        var = bfd_zalloc(abfd, amt_0) as *mut varinfo;
                        if var.is_null() {
                            current_block = 15191033149860947047;
                            break;
                        }
                        (*var).tag = (*abbrev).tag as libc::c_int;
                        (*var).stack = 1 as libc::c_int != 0;
                        (*var).prev_var = (*unit).variable_table;
                        (*unit).variable_table = var;
                        (*var).unit_offset = current_offset;
                    } else {
                        var = 0 as *mut varinfo;
                    }
                    let ref mut fresh13 = (*nested_funcs.offset(nesting_level as isize))
                        .func;
                    *fresh13 = 0 as *mut funcinfo;
                }
                i = 0 as libc::c_int as libc::c_uint;
                while i < (*abbrev).num_attrs {
                    let mut attr: attribute = attribute {
                        name: 0 as dwarf_attribute,
                        form: 0 as dwarf_form,
                        u: C2RustUnnamed_24 {
                            str_0: 0 as *mut libc::c_char,
                        },
                    };
                    info_ptr = read_attribute(
                        &mut attr,
                        &mut *((*abbrev).attrs).offset(i as isize),
                        unit,
                        info_ptr,
                        info_ptr_end,
                    );
                    if info_ptr.is_null() {
                        current_block = 15191033149860947047;
                        break 's_29;
                    }
                    i = i.wrapping_add(1);
                    i;
                }
                if !(*abbrev).has_children {
                    continue;
                }
                nesting_level += 1;
                nesting_level;
                if nesting_level >= nested_funcs_size {
                    let mut tmp: *mut nest_funcinfo = 0 as *mut nest_funcinfo;
                    nested_funcs_size *= 2 as libc::c_int;
                    tmp = bfd_realloc(
                        nested_funcs as *mut libc::c_void,
                        (nested_funcs_size as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<nest_funcinfo>() as libc::c_ulong,
                            ),
                    ) as *mut nest_funcinfo;
                    if tmp.is_null() {
                        current_block = 15191033149860947047;
                        break;
                    }
                    nested_funcs = tmp;
                }
                let ref mut fresh14 = (*nested_funcs.offset(nesting_level as isize))
                    .func;
                *fresh14 = 0 as *mut funcinfo;
            }
        }
    }
    match current_block {
        8545136480011357681 => {
            info_ptr = (*unit).first_child_die_ptr;
            nesting_level = 0 as libc::c_int;
            's_291: loop {
                if !(nesting_level >= 0 as libc::c_int) {
                    current_block = 1830138855519935310;
                    break;
                }
                let mut abbrev_number_0: libc::c_uint = 0;
                let mut i_0: libc::c_uint = 0;
                let mut abbrev_0: *mut abbrev_info = 0 as *mut abbrev_info;
                let mut attr_0: attribute = attribute {
                    name: 0 as dwarf_attribute,
                    form: 0 as dwarf_form,
                    u: C2RustUnnamed_24 {
                        str_0: 0 as *mut libc::c_char,
                    },
                };
                let mut func_0: *mut funcinfo = 0 as *mut funcinfo;
                let mut var_0: *mut varinfo = 0 as *mut varinfo;
                let mut low_pc: bfd_vma = 0 as libc::c_int as bfd_vma;
                let mut high_pc: bfd_vma = 0 as libc::c_int as bfd_vma;
                let mut high_pc_relative: bool = 0 as libc::c_int != 0;
                let mut current_offset_0: bfd_uint64_t = 0;
                if info_ptr >= info_ptr_end {
                    current_block = 15191033149860947047;
                    break;
                }
                current_offset_0 = info_ptr.offset_from((*unit).info_ptr_unit)
                    as libc::c_long as bfd_uint64_t;
                abbrev_number_0 = _bfd_safe_read_leb128(
                    abfd,
                    &mut info_ptr,
                    0 as libc::c_int != 0,
                    info_ptr_end,
                ) as libc::c_uint;
                if abbrev_number_0 == 0 {
                    nesting_level -= 1;
                    nesting_level;
                } else {
                    abbrev_0 = lookup_abbrev(abbrev_number_0, (*unit).abbrevs);
                    if abbrev_0.is_null() {
                        bfd_assert(
                            b"./dwarf2.c\0" as *const u8 as *const libc::c_char,
                            3426 as libc::c_int,
                        );
                    }
                    func_0 = 0 as *mut funcinfo;
                    var_0 = 0 as *mut varinfo;
                    if (*abbrev_0).tag as libc::c_uint
                        == DW_TAG_subprogram as libc::c_int as libc::c_uint
                        || (*abbrev_0).tag as libc::c_uint
                            == DW_TAG_entry_point as libc::c_int as libc::c_uint
                        || (*abbrev_0).tag as libc::c_uint
                            == DW_TAG_inlined_subroutine as libc::c_int as libc::c_uint
                    {
                        func_0 = lookup_func_by_offset(
                            current_offset_0,
                            (*unit).function_table,
                        );
                        if func_0.is_null() {
                            current_block = 15191033149860947047;
                            break;
                        }
                    } else if (*abbrev_0).tag as libc::c_uint
                        == DW_TAG_variable as libc::c_int as libc::c_uint
                        || (*abbrev_0).tag as libc::c_uint
                            == DW_TAG_member as libc::c_int as libc::c_uint
                    {
                        var_0 = lookup_var_by_offset(
                            current_offset_0,
                            (*unit).variable_table,
                        );
                        if var_0.is_null() {
                            current_block = 15191033149860947047;
                            break;
                        }
                    }
                    i_0 = 0 as libc::c_int as libc::c_uint;
                    while i_0 < (*abbrev_0).num_attrs {
                        info_ptr = read_attribute(
                            &mut attr_0,
                            &mut *((*abbrev_0).attrs).offset(i_0 as isize),
                            unit,
                            info_ptr,
                            info_ptr_end,
                        );
                        if info_ptr.is_null() {
                            current_block = 15191033149860947047;
                            break 's_291;
                        }
                        if !func_0.is_null() {
                            match attr_0.name as libc::c_uint {
                                88 => {
                                    current_block = 18070827520340661055;
                                    match current_block {
                                        7711255570521815756 => {
                                            (*func_0).line = attr_0.u.val as libc::c_int;
                                        }
                                        18070827520340661055 => {
                                            (*func_0)
                                                .caller_file = concat_filename(
                                                (*unit).line_table,
                                                attr_0.u.val as libc::c_uint,
                                            );
                                        }
                                        8202393673878014038 => {
                                            (*func_0).caller_line = attr_0.u.val as libc::c_int;
                                        }
                                        16126536645098213004 => {
                                            if !find_abstract_instance(
                                                unit,
                                                &mut attr_0,
                                                0 as libc::c_int as libc::c_uint,
                                                &mut (*func_0).name,
                                                &mut (*func_0).is_linkage,
                                                &mut (*func_0).file,
                                                &mut (*func_0).line,
                                            ) {
                                                current_block = 15191033149860947047;
                                                break 's_291;
                                            }
                                        }
                                        8067717456239466500 => {
                                            if ((*func_0).name).is_null()
                                                && is_str_attr(attr_0.form) as libc::c_int != 0
                                            {
                                                (*func_0).name = attr_0.u.str_0;
                                                if non_mangled((*unit).lang) {
                                                    (*func_0).is_linkage = 1 as libc::c_int != 0;
                                                }
                                            }
                                        }
                                        7537626883245681487 => {
                                            if is_str_attr(attr_0.form) {
                                                (*func_0).name = attr_0.u.str_0;
                                                (*func_0).is_linkage = 1 as libc::c_int != 0;
                                            }
                                        }
                                        9520546616418542419 => {
                                            low_pc = attr_0.u.val;
                                        }
                                        17352930546238167574 => {
                                            if !read_rangelist(
                                                unit,
                                                &mut (*func_0).arange,
                                                attr_0.u.val,
                                            ) {
                                                current_block = 15191033149860947047;
                                                break 's_291;
                                            }
                                        }
                                        14860613234516215618 => {
                                            (*func_0)
                                                .file = concat_filename(
                                                (*unit).line_table,
                                                attr_0.u.val as libc::c_uint,
                                            );
                                        }
                                        _ => {
                                            high_pc = attr_0.u.val;
                                            high_pc_relative = attr_0.form as libc::c_uint
                                                != DW_FORM_addr as libc::c_int as libc::c_uint;
                                        }
                                    }
                                }
                                89 => {
                                    current_block = 8202393673878014038;
                                    match current_block {
                                        7711255570521815756 => {
                                            (*func_0).line = attr_0.u.val as libc::c_int;
                                        }
                                        18070827520340661055 => {
                                            (*func_0)
                                                .caller_file = concat_filename(
                                                (*unit).line_table,
                                                attr_0.u.val as libc::c_uint,
                                            );
                                        }
                                        8202393673878014038 => {
                                            (*func_0).caller_line = attr_0.u.val as libc::c_int;
                                        }
                                        16126536645098213004 => {
                                            if !find_abstract_instance(
                                                unit,
                                                &mut attr_0,
                                                0 as libc::c_int as libc::c_uint,
                                                &mut (*func_0).name,
                                                &mut (*func_0).is_linkage,
                                                &mut (*func_0).file,
                                                &mut (*func_0).line,
                                            ) {
                                                current_block = 15191033149860947047;
                                                break 's_291;
                                            }
                                        }
                                        8067717456239466500 => {
                                            if ((*func_0).name).is_null()
                                                && is_str_attr(attr_0.form) as libc::c_int != 0
                                            {
                                                (*func_0).name = attr_0.u.str_0;
                                                if non_mangled((*unit).lang) {
                                                    (*func_0).is_linkage = 1 as libc::c_int != 0;
                                                }
                                            }
                                        }
                                        7537626883245681487 => {
                                            if is_str_attr(attr_0.form) {
                                                (*func_0).name = attr_0.u.str_0;
                                                (*func_0).is_linkage = 1 as libc::c_int != 0;
                                            }
                                        }
                                        9520546616418542419 => {
                                            low_pc = attr_0.u.val;
                                        }
                                        17352930546238167574 => {
                                            if !read_rangelist(
                                                unit,
                                                &mut (*func_0).arange,
                                                attr_0.u.val,
                                            ) {
                                                current_block = 15191033149860947047;
                                                break 's_291;
                                            }
                                        }
                                        14860613234516215618 => {
                                            (*func_0)
                                                .file = concat_filename(
                                                (*unit).line_table,
                                                attr_0.u.val as libc::c_uint,
                                            );
                                        }
                                        _ => {
                                            high_pc = attr_0.u.val;
                                            high_pc_relative = attr_0.form as libc::c_uint
                                                != DW_FORM_addr as libc::c_int as libc::c_uint;
                                        }
                                    }
                                }
                                49 | 71 => {
                                    current_block = 16126536645098213004;
                                    match current_block {
                                        7711255570521815756 => {
                                            (*func_0).line = attr_0.u.val as libc::c_int;
                                        }
                                        18070827520340661055 => {
                                            (*func_0)
                                                .caller_file = concat_filename(
                                                (*unit).line_table,
                                                attr_0.u.val as libc::c_uint,
                                            );
                                        }
                                        8202393673878014038 => {
                                            (*func_0).caller_line = attr_0.u.val as libc::c_int;
                                        }
                                        16126536645098213004 => {
                                            if !find_abstract_instance(
                                                unit,
                                                &mut attr_0,
                                                0 as libc::c_int as libc::c_uint,
                                                &mut (*func_0).name,
                                                &mut (*func_0).is_linkage,
                                                &mut (*func_0).file,
                                                &mut (*func_0).line,
                                            ) {
                                                current_block = 15191033149860947047;
                                                break 's_291;
                                            }
                                        }
                                        8067717456239466500 => {
                                            if ((*func_0).name).is_null()
                                                && is_str_attr(attr_0.form) as libc::c_int != 0
                                            {
                                                (*func_0).name = attr_0.u.str_0;
                                                if non_mangled((*unit).lang) {
                                                    (*func_0).is_linkage = 1 as libc::c_int != 0;
                                                }
                                            }
                                        }
                                        7537626883245681487 => {
                                            if is_str_attr(attr_0.form) {
                                                (*func_0).name = attr_0.u.str_0;
                                                (*func_0).is_linkage = 1 as libc::c_int != 0;
                                            }
                                        }
                                        9520546616418542419 => {
                                            low_pc = attr_0.u.val;
                                        }
                                        17352930546238167574 => {
                                            if !read_rangelist(
                                                unit,
                                                &mut (*func_0).arange,
                                                attr_0.u.val,
                                            ) {
                                                current_block = 15191033149860947047;
                                                break 's_291;
                                            }
                                        }
                                        14860613234516215618 => {
                                            (*func_0)
                                                .file = concat_filename(
                                                (*unit).line_table,
                                                attr_0.u.val as libc::c_uint,
                                            );
                                        }
                                        _ => {
                                            high_pc = attr_0.u.val;
                                            high_pc_relative = attr_0.form as libc::c_uint
                                                != DW_FORM_addr as libc::c_int as libc::c_uint;
                                        }
                                    }
                                }
                                3 => {
                                    current_block = 8067717456239466500;
                                    match current_block {
                                        7711255570521815756 => {
                                            (*func_0).line = attr_0.u.val as libc::c_int;
                                        }
                                        18070827520340661055 => {
                                            (*func_0)
                                                .caller_file = concat_filename(
                                                (*unit).line_table,
                                                attr_0.u.val as libc::c_uint,
                                            );
                                        }
                                        8202393673878014038 => {
                                            (*func_0).caller_line = attr_0.u.val as libc::c_int;
                                        }
                                        16126536645098213004 => {
                                            if !find_abstract_instance(
                                                unit,
                                                &mut attr_0,
                                                0 as libc::c_int as libc::c_uint,
                                                &mut (*func_0).name,
                                                &mut (*func_0).is_linkage,
                                                &mut (*func_0).file,
                                                &mut (*func_0).line,
                                            ) {
                                                current_block = 15191033149860947047;
                                                break 's_291;
                                            }
                                        }
                                        8067717456239466500 => {
                                            if ((*func_0).name).is_null()
                                                && is_str_attr(attr_0.form) as libc::c_int != 0
                                            {
                                                (*func_0).name = attr_0.u.str_0;
                                                if non_mangled((*unit).lang) {
                                                    (*func_0).is_linkage = 1 as libc::c_int != 0;
                                                }
                                            }
                                        }
                                        7537626883245681487 => {
                                            if is_str_attr(attr_0.form) {
                                                (*func_0).name = attr_0.u.str_0;
                                                (*func_0).is_linkage = 1 as libc::c_int != 0;
                                            }
                                        }
                                        9520546616418542419 => {
                                            low_pc = attr_0.u.val;
                                        }
                                        17352930546238167574 => {
                                            if !read_rangelist(
                                                unit,
                                                &mut (*func_0).arange,
                                                attr_0.u.val,
                                            ) {
                                                current_block = 15191033149860947047;
                                                break 's_291;
                                            }
                                        }
                                        14860613234516215618 => {
                                            (*func_0)
                                                .file = concat_filename(
                                                (*unit).line_table,
                                                attr_0.u.val as libc::c_uint,
                                            );
                                        }
                                        _ => {
                                            high_pc = attr_0.u.val;
                                            high_pc_relative = attr_0.form as libc::c_uint
                                                != DW_FORM_addr as libc::c_int as libc::c_uint;
                                        }
                                    }
                                }
                                110 | 8199 => {
                                    current_block = 7537626883245681487;
                                    match current_block {
                                        7711255570521815756 => {
                                            (*func_0).line = attr_0.u.val as libc::c_int;
                                        }
                                        18070827520340661055 => {
                                            (*func_0)
                                                .caller_file = concat_filename(
                                                (*unit).line_table,
                                                attr_0.u.val as libc::c_uint,
                                            );
                                        }
                                        8202393673878014038 => {
                                            (*func_0).caller_line = attr_0.u.val as libc::c_int;
                                        }
                                        16126536645098213004 => {
                                            if !find_abstract_instance(
                                                unit,
                                                &mut attr_0,
                                                0 as libc::c_int as libc::c_uint,
                                                &mut (*func_0).name,
                                                &mut (*func_0).is_linkage,
                                                &mut (*func_0).file,
                                                &mut (*func_0).line,
                                            ) {
                                                current_block = 15191033149860947047;
                                                break 's_291;
                                            }
                                        }
                                        8067717456239466500 => {
                                            if ((*func_0).name).is_null()
                                                && is_str_attr(attr_0.form) as libc::c_int != 0
                                            {
                                                (*func_0).name = attr_0.u.str_0;
                                                if non_mangled((*unit).lang) {
                                                    (*func_0).is_linkage = 1 as libc::c_int != 0;
                                                }
                                            }
                                        }
                                        7537626883245681487 => {
                                            if is_str_attr(attr_0.form) {
                                                (*func_0).name = attr_0.u.str_0;
                                                (*func_0).is_linkage = 1 as libc::c_int != 0;
                                            }
                                        }
                                        9520546616418542419 => {
                                            low_pc = attr_0.u.val;
                                        }
                                        17352930546238167574 => {
                                            if !read_rangelist(
                                                unit,
                                                &mut (*func_0).arange,
                                                attr_0.u.val,
                                            ) {
                                                current_block = 15191033149860947047;
                                                break 's_291;
                                            }
                                        }
                                        14860613234516215618 => {
                                            (*func_0)
                                                .file = concat_filename(
                                                (*unit).line_table,
                                                attr_0.u.val as libc::c_uint,
                                            );
                                        }
                                        _ => {
                                            high_pc = attr_0.u.val;
                                            high_pc_relative = attr_0.form as libc::c_uint
                                                != DW_FORM_addr as libc::c_int as libc::c_uint;
                                        }
                                    }
                                }
                                17 => {
                                    current_block = 9520546616418542419;
                                    match current_block {
                                        7711255570521815756 => {
                                            (*func_0).line = attr_0.u.val as libc::c_int;
                                        }
                                        18070827520340661055 => {
                                            (*func_0)
                                                .caller_file = concat_filename(
                                                (*unit).line_table,
                                                attr_0.u.val as libc::c_uint,
                                            );
                                        }
                                        8202393673878014038 => {
                                            (*func_0).caller_line = attr_0.u.val as libc::c_int;
                                        }
                                        16126536645098213004 => {
                                            if !find_abstract_instance(
                                                unit,
                                                &mut attr_0,
                                                0 as libc::c_int as libc::c_uint,
                                                &mut (*func_0).name,
                                                &mut (*func_0).is_linkage,
                                                &mut (*func_0).file,
                                                &mut (*func_0).line,
                                            ) {
                                                current_block = 15191033149860947047;
                                                break 's_291;
                                            }
                                        }
                                        8067717456239466500 => {
                                            if ((*func_0).name).is_null()
                                                && is_str_attr(attr_0.form) as libc::c_int != 0
                                            {
                                                (*func_0).name = attr_0.u.str_0;
                                                if non_mangled((*unit).lang) {
                                                    (*func_0).is_linkage = 1 as libc::c_int != 0;
                                                }
                                            }
                                        }
                                        7537626883245681487 => {
                                            if is_str_attr(attr_0.form) {
                                                (*func_0).name = attr_0.u.str_0;
                                                (*func_0).is_linkage = 1 as libc::c_int != 0;
                                            }
                                        }
                                        9520546616418542419 => {
                                            low_pc = attr_0.u.val;
                                        }
                                        17352930546238167574 => {
                                            if !read_rangelist(
                                                unit,
                                                &mut (*func_0).arange,
                                                attr_0.u.val,
                                            ) {
                                                current_block = 15191033149860947047;
                                                break 's_291;
                                            }
                                        }
                                        14860613234516215618 => {
                                            (*func_0)
                                                .file = concat_filename(
                                                (*unit).line_table,
                                                attr_0.u.val as libc::c_uint,
                                            );
                                        }
                                        _ => {
                                            high_pc = attr_0.u.val;
                                            high_pc_relative = attr_0.form as libc::c_uint
                                                != DW_FORM_addr as libc::c_int as libc::c_uint;
                                        }
                                    }
                                }
                                18 => {
                                    current_block = 1222295074918101514;
                                    match current_block {
                                        7711255570521815756 => {
                                            (*func_0).line = attr_0.u.val as libc::c_int;
                                        }
                                        18070827520340661055 => {
                                            (*func_0)
                                                .caller_file = concat_filename(
                                                (*unit).line_table,
                                                attr_0.u.val as libc::c_uint,
                                            );
                                        }
                                        8202393673878014038 => {
                                            (*func_0).caller_line = attr_0.u.val as libc::c_int;
                                        }
                                        16126536645098213004 => {
                                            if !find_abstract_instance(
                                                unit,
                                                &mut attr_0,
                                                0 as libc::c_int as libc::c_uint,
                                                &mut (*func_0).name,
                                                &mut (*func_0).is_linkage,
                                                &mut (*func_0).file,
                                                &mut (*func_0).line,
                                            ) {
                                                current_block = 15191033149860947047;
                                                break 's_291;
                                            }
                                        }
                                        8067717456239466500 => {
                                            if ((*func_0).name).is_null()
                                                && is_str_attr(attr_0.form) as libc::c_int != 0
                                            {
                                                (*func_0).name = attr_0.u.str_0;
                                                if non_mangled((*unit).lang) {
                                                    (*func_0).is_linkage = 1 as libc::c_int != 0;
                                                }
                                            }
                                        }
                                        7537626883245681487 => {
                                            if is_str_attr(attr_0.form) {
                                                (*func_0).name = attr_0.u.str_0;
                                                (*func_0).is_linkage = 1 as libc::c_int != 0;
                                            }
                                        }
                                        9520546616418542419 => {
                                            low_pc = attr_0.u.val;
                                        }
                                        17352930546238167574 => {
                                            if !read_rangelist(
                                                unit,
                                                &mut (*func_0).arange,
                                                attr_0.u.val,
                                            ) {
                                                current_block = 15191033149860947047;
                                                break 's_291;
                                            }
                                        }
                                        14860613234516215618 => {
                                            (*func_0)
                                                .file = concat_filename(
                                                (*unit).line_table,
                                                attr_0.u.val as libc::c_uint,
                                            );
                                        }
                                        _ => {
                                            high_pc = attr_0.u.val;
                                            high_pc_relative = attr_0.form as libc::c_uint
                                                != DW_FORM_addr as libc::c_int as libc::c_uint;
                                        }
                                    }
                                }
                                85 => {
                                    current_block = 17352930546238167574;
                                    match current_block {
                                        7711255570521815756 => {
                                            (*func_0).line = attr_0.u.val as libc::c_int;
                                        }
                                        18070827520340661055 => {
                                            (*func_0)
                                                .caller_file = concat_filename(
                                                (*unit).line_table,
                                                attr_0.u.val as libc::c_uint,
                                            );
                                        }
                                        8202393673878014038 => {
                                            (*func_0).caller_line = attr_0.u.val as libc::c_int;
                                        }
                                        16126536645098213004 => {
                                            if !find_abstract_instance(
                                                unit,
                                                &mut attr_0,
                                                0 as libc::c_int as libc::c_uint,
                                                &mut (*func_0).name,
                                                &mut (*func_0).is_linkage,
                                                &mut (*func_0).file,
                                                &mut (*func_0).line,
                                            ) {
                                                current_block = 15191033149860947047;
                                                break 's_291;
                                            }
                                        }
                                        8067717456239466500 => {
                                            if ((*func_0).name).is_null()
                                                && is_str_attr(attr_0.form) as libc::c_int != 0
                                            {
                                                (*func_0).name = attr_0.u.str_0;
                                                if non_mangled((*unit).lang) {
                                                    (*func_0).is_linkage = 1 as libc::c_int != 0;
                                                }
                                            }
                                        }
                                        7537626883245681487 => {
                                            if is_str_attr(attr_0.form) {
                                                (*func_0).name = attr_0.u.str_0;
                                                (*func_0).is_linkage = 1 as libc::c_int != 0;
                                            }
                                        }
                                        9520546616418542419 => {
                                            low_pc = attr_0.u.val;
                                        }
                                        17352930546238167574 => {
                                            if !read_rangelist(
                                                unit,
                                                &mut (*func_0).arange,
                                                attr_0.u.val,
                                            ) {
                                                current_block = 15191033149860947047;
                                                break 's_291;
                                            }
                                        }
                                        14860613234516215618 => {
                                            (*func_0)
                                                .file = concat_filename(
                                                (*unit).line_table,
                                                attr_0.u.val as libc::c_uint,
                                            );
                                        }
                                        _ => {
                                            high_pc = attr_0.u.val;
                                            high_pc_relative = attr_0.form as libc::c_uint
                                                != DW_FORM_addr as libc::c_int as libc::c_uint;
                                        }
                                    }
                                }
                                58 => {
                                    current_block = 14860613234516215618;
                                    match current_block {
                                        7711255570521815756 => {
                                            (*func_0).line = attr_0.u.val as libc::c_int;
                                        }
                                        18070827520340661055 => {
                                            (*func_0)
                                                .caller_file = concat_filename(
                                                (*unit).line_table,
                                                attr_0.u.val as libc::c_uint,
                                            );
                                        }
                                        8202393673878014038 => {
                                            (*func_0).caller_line = attr_0.u.val as libc::c_int;
                                        }
                                        16126536645098213004 => {
                                            if !find_abstract_instance(
                                                unit,
                                                &mut attr_0,
                                                0 as libc::c_int as libc::c_uint,
                                                &mut (*func_0).name,
                                                &mut (*func_0).is_linkage,
                                                &mut (*func_0).file,
                                                &mut (*func_0).line,
                                            ) {
                                                current_block = 15191033149860947047;
                                                break 's_291;
                                            }
                                        }
                                        8067717456239466500 => {
                                            if ((*func_0).name).is_null()
                                                && is_str_attr(attr_0.form) as libc::c_int != 0
                                            {
                                                (*func_0).name = attr_0.u.str_0;
                                                if non_mangled((*unit).lang) {
                                                    (*func_0).is_linkage = 1 as libc::c_int != 0;
                                                }
                                            }
                                        }
                                        7537626883245681487 => {
                                            if is_str_attr(attr_0.form) {
                                                (*func_0).name = attr_0.u.str_0;
                                                (*func_0).is_linkage = 1 as libc::c_int != 0;
                                            }
                                        }
                                        9520546616418542419 => {
                                            low_pc = attr_0.u.val;
                                        }
                                        17352930546238167574 => {
                                            if !read_rangelist(
                                                unit,
                                                &mut (*func_0).arange,
                                                attr_0.u.val,
                                            ) {
                                                current_block = 15191033149860947047;
                                                break 's_291;
                                            }
                                        }
                                        14860613234516215618 => {
                                            (*func_0)
                                                .file = concat_filename(
                                                (*unit).line_table,
                                                attr_0.u.val as libc::c_uint,
                                            );
                                        }
                                        _ => {
                                            high_pc = attr_0.u.val;
                                            high_pc_relative = attr_0.form as libc::c_uint
                                                != DW_FORM_addr as libc::c_int as libc::c_uint;
                                        }
                                    }
                                }
                                59 => {
                                    current_block = 7711255570521815756;
                                    match current_block {
                                        7711255570521815756 => {
                                            (*func_0).line = attr_0.u.val as libc::c_int;
                                        }
                                        18070827520340661055 => {
                                            (*func_0)
                                                .caller_file = concat_filename(
                                                (*unit).line_table,
                                                attr_0.u.val as libc::c_uint,
                                            );
                                        }
                                        8202393673878014038 => {
                                            (*func_0).caller_line = attr_0.u.val as libc::c_int;
                                        }
                                        16126536645098213004 => {
                                            if !find_abstract_instance(
                                                unit,
                                                &mut attr_0,
                                                0 as libc::c_int as libc::c_uint,
                                                &mut (*func_0).name,
                                                &mut (*func_0).is_linkage,
                                                &mut (*func_0).file,
                                                &mut (*func_0).line,
                                            ) {
                                                current_block = 15191033149860947047;
                                                break 's_291;
                                            }
                                        }
                                        8067717456239466500 => {
                                            if ((*func_0).name).is_null()
                                                && is_str_attr(attr_0.form) as libc::c_int != 0
                                            {
                                                (*func_0).name = attr_0.u.str_0;
                                                if non_mangled((*unit).lang) {
                                                    (*func_0).is_linkage = 1 as libc::c_int != 0;
                                                }
                                            }
                                        }
                                        7537626883245681487 => {
                                            if is_str_attr(attr_0.form) {
                                                (*func_0).name = attr_0.u.str_0;
                                                (*func_0).is_linkage = 1 as libc::c_int != 0;
                                            }
                                        }
                                        9520546616418542419 => {
                                            low_pc = attr_0.u.val;
                                        }
                                        17352930546238167574 => {
                                            if !read_rangelist(
                                                unit,
                                                &mut (*func_0).arange,
                                                attr_0.u.val,
                                            ) {
                                                current_block = 15191033149860947047;
                                                break 's_291;
                                            }
                                        }
                                        14860613234516215618 => {
                                            (*func_0)
                                                .file = concat_filename(
                                                (*unit).line_table,
                                                attr_0.u.val as libc::c_uint,
                                            );
                                        }
                                        _ => {
                                            high_pc = attr_0.u.val;
                                            high_pc_relative = attr_0.form as libc::c_uint
                                                != DW_FORM_addr as libc::c_int as libc::c_uint;
                                        }
                                    }
                                }
                                _ => {}
                            }
                        } else if !var_0.is_null() {
                            match attr_0.name as libc::c_uint {
                                71 => {
                                    if attr_0.u.val != 0 {
                                        let mut spec_var: *mut varinfo = 0 as *mut varinfo;
                                        spec_var = lookup_var_by_offset(
                                            attr_0.u.val,
                                            (*unit).variable_table,
                                        );
                                        if spec_var.is_null() {
                                            _bfd_error_handler(
                                                dcgettext(
                                                    b"bfd\0" as *const u8 as *const libc::c_char,
                                                    b"DWARF error: could not find variable specification at offset 0x%lx\0"
                                                        as *const u8 as *const libc::c_char,
                                                    5 as libc::c_int,
                                                ),
                                                attr_0.u.val,
                                            );
                                        } else {
                                            if ((*var_0).name).is_null() {
                                                (*var_0).name = (*spec_var).name;
                                            }
                                            if ((*var_0).file).is_null()
                                                && !((*spec_var).file).is_null()
                                            {
                                                (*var_0).file = strdup((*spec_var).file);
                                            }
                                            if (*var_0).line == 0 as libc::c_int {
                                                (*var_0).line = (*spec_var).line;
                                            }
                                            if ((*var_0).sec).is_null() {
                                                (*var_0).sec = (*spec_var).sec;
                                            }
                                        }
                                    }
                                }
                                3 => {
                                    if is_str_attr(attr_0.form) {
                                        (*var_0).name = attr_0.u.str_0;
                                    }
                                }
                                58 => {
                                    (*var_0)
                                        .file = concat_filename(
                                        (*unit).line_table,
                                        attr_0.u.val as libc::c_uint,
                                    );
                                }
                                59 => {
                                    (*var_0).line = attr_0.u.val as libc::c_int;
                                }
                                63 => {
                                    if attr_0.u.val != 0 as libc::c_int as libc::c_ulong {
                                        (*var_0).stack = 0 as libc::c_int != 0;
                                    }
                                }
                                2 => {
                                    match attr_0.form as libc::c_uint {
                                        9 | 10 | 3 | 4 | 24 => {
                                            if !((*attr_0.u.blk).data).is_null()
                                                && *(*attr_0.u.blk).data as libc::c_int
                                                    == DW_OP_addr as libc::c_int
                                            {
                                                (*var_0).stack = 0 as libc::c_int != 0;
                                                if (*attr_0.u.blk).size
                                                    == ((*unit).addr_size as libc::c_uint)
                                                        .wrapping_add(1 as libc::c_uint)
                                                {
                                                    (*var_0)
                                                        .addr = if (*unit).addr_size as libc::c_int
                                                        * 8 as libc::c_int == 8 as libc::c_int
                                                    {
                                                        *(((*attr_0.u.blk).data).offset(1 as libc::c_int as isize)
                                                            as *const libc::c_uchar) as bfd_vma
                                                            & 0xff as libc::c_int as libc::c_ulong
                                                    } else if (*unit).addr_size as libc::c_int
                                                        * 8 as libc::c_int == 16 as libc::c_int
                                                    {
                                                        (Some(
                                                            ((*(*(*unit).abfd).xvec).bfd_getx16)
                                                                .expect("non-null function pointer"),
                                                        ))
                                                            .expect(
                                                                "non-null function pointer",
                                                            )(
                                                            ((*attr_0.u.blk).data).offset(1 as libc::c_int as isize)
                                                                as *const libc::c_void,
                                                        )
                                                    } else if (*unit).addr_size as libc::c_int
                                                        * 8 as libc::c_int == 32 as libc::c_int
                                                    {
                                                        (Some(
                                                            ((*(*(*unit).abfd).xvec).bfd_getx32)
                                                                .expect("non-null function pointer"),
                                                        ))
                                                            .expect(
                                                                "non-null function pointer",
                                                            )(
                                                            ((*attr_0.u.blk).data).offset(1 as libc::c_int as isize)
                                                                as *const libc::c_void,
                                                        )
                                                    } else if (*unit).addr_size as libc::c_int
                                                        * 8 as libc::c_int == 64 as libc::c_int
                                                    {
                                                        (Some(
                                                            ((*(*(*unit).abfd).xvec).bfd_getx64)
                                                                .expect("non-null function pointer"),
                                                        ))
                                                            .expect(
                                                                "non-null function pointer",
                                                            )(
                                                            ((*attr_0.u.blk).data).offset(1 as libc::c_int as isize)
                                                                as *const libc::c_void,
                                                        )
                                                    } else {
                                                        _bfd_abort(
                                                            b"./dwarf2.c\0" as *const u8 as *const libc::c_char,
                                                            3597 as libc::c_int,
                                                            (*::core::mem::transmute::<
                                                                &[u8; 48],
                                                                &[libc::c_char; 48],
                                                            >(b"_Bool scan_unit_for_symbols(struct comp_unit *)\0"))
                                                                .as_ptr(),
                                                        );
                                                        -(1 as libc::c_int) as bfd_vma
                                                    };
                                                }
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                                _ => {}
                            }
                        }
                        i_0 = i_0.wrapping_add(1);
                        i_0;
                    }
                    if (*abbrev_0).has_children {
                        nesting_level += 1;
                        nesting_level;
                    }
                    if high_pc_relative {
                        high_pc = (high_pc as libc::c_ulong).wrapping_add(low_pc)
                            as bfd_vma as bfd_vma;
                    }
                    if !(!func_0.is_null()
                        && high_pc != 0 as libc::c_int as libc::c_ulong)
                    {
                        continue;
                    }
                    if !arange_add(unit, &mut (*func_0).arange, low_pc, high_pc) {
                        current_block = 15191033149860947047;
                        break;
                    }
                }
            }
            match current_block {
                15191033149860947047 => {}
                _ => {
                    free(nested_funcs as *mut libc::c_void);
                    return 1 as libc::c_int != 0;
                }
            }
        }
        _ => {}
    }
    free(nested_funcs as *mut libc::c_void);
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn parse_comp_unit(
    mut stash: *mut dwarf2_debug,
    mut file: *mut dwarf2_debug_file,
    mut info_ptr: *mut bfd_byte,
    mut unit_length: bfd_vma,
    mut info_ptr_unit: *mut bfd_byte,
    mut offset_size: libc::c_uint,
) -> *mut comp_unit {
    let mut unit: *mut comp_unit = 0 as *mut comp_unit;
    let mut version: libc::c_uint = 0;
    let mut abbrev_offset: bfd_uint64_t = 0 as libc::c_int as bfd_uint64_t;
    let mut addr_size: libc::c_uint = -(1 as libc::c_int) as libc::c_uint;
    let mut abbrevs: *mut *mut abbrev_info = 0 as *mut *mut abbrev_info;
    let mut abbrev_number: libc::c_uint = 0;
    let mut i: libc::c_uint = 0;
    let mut abbrev: *mut abbrev_info = 0 as *mut abbrev_info;
    let mut attr: attribute = attribute {
        name: 0 as dwarf_attribute,
        form: 0 as dwarf_form,
        u: C2RustUnnamed_24 {
            str_0: 0 as *mut libc::c_char,
        },
    };
    let mut end_ptr: *mut bfd_byte = info_ptr.offset(unit_length as isize);
    let mut amt: size_t = 0;
    let mut low_pc: bfd_vma = 0 as libc::c_int as bfd_vma;
    let mut high_pc: bfd_vma = 0 as libc::c_int as bfd_vma;
    let mut abfd: *mut bfd = (*file).bfd_ptr;
    let mut high_pc_relative: bool = 0 as libc::c_int != 0;
    let mut unit_type: dwarf_unit_type = 0 as dwarf_unit_type;
    version = read_2_bytes(abfd, &mut info_ptr, end_ptr);
    if version < 2 as libc::c_int as libc::c_uint
        || version > 5 as libc::c_int as libc::c_uint
    {
        if version != 0 {
            _bfd_error_handler(
                dcgettext(
                    b"bfd\0" as *const u8 as *const libc::c_char,
                    b"DWARF error: found dwarf version '%u', this reader only handles version 2, 3, 4 and 5 information\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                version,
            );
            bfd_set_error(bfd_error_bad_value);
        }
        return 0 as *mut comp_unit;
    }
    if version < 5 as libc::c_int as libc::c_uint {
        unit_type = DW_UT_compile;
    } else {
        unit_type = read_1_byte(abfd, &mut info_ptr, end_ptr) as dwarf_unit_type;
        addr_size = read_1_byte(abfd, &mut info_ptr, end_ptr);
    }
    if !(offset_size == 4 as libc::c_int as libc::c_uint
        || offset_size == 8 as libc::c_int as libc::c_uint)
    {
        bfd_assert(
            b"./dwarf2.c\0" as *const u8 as *const libc::c_char,
            3692 as libc::c_int,
        );
    }
    if offset_size == 4 as libc::c_int as libc::c_uint {
        abbrev_offset = read_4_bytes(abfd, &mut info_ptr, end_ptr) as bfd_uint64_t;
    } else {
        abbrev_offset = read_8_bytes(abfd, &mut info_ptr, end_ptr);
    }
    if version < 5 as libc::c_int as libc::c_uint {
        addr_size = read_1_byte(abfd, &mut info_ptr, end_ptr);
    }
    if unit_type as libc::c_uint == DW_UT_type as libc::c_int as libc::c_uint {
        info_ptr = info_ptr.offset(8 as libc::c_int as isize);
        info_ptr = info_ptr.offset(offset_size as isize);
    }
    if addr_size as libc::c_ulong > ::core::mem::size_of::<bfd_vma>() as libc::c_ulong {
        _bfd_error_handler(
            dcgettext(
                b"bfd\0" as *const u8 as *const libc::c_char,
                b"DWARF error: found address size '%u', this reader can not handle sizes greater than '%u'\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            addr_size,
            ::core::mem::size_of::<bfd_vma>() as libc::c_ulong as libc::c_uint,
        );
        bfd_set_error(bfd_error_bad_value);
        return 0 as *mut comp_unit;
    }
    if addr_size != 2 as libc::c_int as libc::c_uint
        && addr_size != 4 as libc::c_int as libc::c_uint
        && addr_size != 8 as libc::c_int as libc::c_uint
    {
        _bfd_error_handler(
            b"DWARF error: found address size '%u', this reader can only handle address sizes '2', '4' and '8'\0"
                as *const u8 as *const libc::c_char,
            addr_size,
        );
        bfd_set_error(bfd_error_bad_value);
        return 0 as *mut comp_unit;
    }
    abbrevs = read_abbrevs(abfd, abbrev_offset, stash, file);
    if abbrevs.is_null() {
        return 0 as *mut comp_unit;
    }
    abbrev_number = _bfd_safe_read_leb128(
        abfd,
        &mut info_ptr,
        0 as libc::c_int != 0,
        end_ptr,
    ) as libc::c_uint;
    if abbrev_number == 0 {
        return 0 as *mut comp_unit;
    }
    abbrev = lookup_abbrev(abbrev_number, abbrevs);
    if abbrev.is_null() {
        _bfd_error_handler(
            dcgettext(
                b"bfd\0" as *const u8 as *const libc::c_char,
                b"DWARF error: could not find abbrev number %u\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            abbrev_number,
        );
        bfd_set_error(bfd_error_bad_value);
        return 0 as *mut comp_unit;
    }
    amt = ::core::mem::size_of::<comp_unit>() as libc::c_ulong;
    unit = bfd_zalloc(abfd, amt) as *mut comp_unit;
    if unit.is_null() {
        return 0 as *mut comp_unit;
    }
    (*unit).abfd = abfd;
    (*unit).version = version as libc::c_int;
    (*unit).addr_size = addr_size as libc::c_uchar;
    (*unit).offset_size = offset_size as libc::c_uchar;
    (*unit).abbrevs = abbrevs;
    (*unit).end_ptr = end_ptr;
    (*unit).stash = stash;
    (*unit).file = file;
    (*unit).info_ptr_unit = info_ptr_unit;
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*abbrev).num_attrs {
        info_ptr = read_attribute(
            &mut attr,
            &mut *((*abbrev).attrs).offset(i as isize),
            unit,
            info_ptr,
            end_ptr,
        );
        if info_ptr.is_null() {
            return 0 as *mut comp_unit;
        }
        match attr.name as libc::c_uint {
            16 => {
                (*unit).stmtlist = 1 as libc::c_int;
                (*unit).line_offset = attr.u.val;
            }
            3 => {
                if is_str_attr(attr.form) {
                    (*unit).name = attr.u.str_0;
                }
            }
            17 => {
                low_pc = attr.u.val;
                if (*abbrev).tag as libc::c_uint
                    == DW_TAG_compile_unit as libc::c_int as libc::c_uint
                {
                    (*unit).base_address = low_pc;
                }
            }
            18 => {
                high_pc = attr.u.val;
                high_pc_relative = attr.form as libc::c_uint
                    != DW_FORM_addr as libc::c_int as libc::c_uint;
            }
            85 => {
                if !read_rangelist(unit, &mut (*unit).arange, attr.u.val) {
                    return 0 as *mut comp_unit;
                }
            }
            27 => {
                let mut comp_dir: *mut libc::c_char = attr.u.str_0;
                if !is_str_attr(attr.form) {
                    _bfd_error_handler(
                        dcgettext(
                            b"bfd\0" as *const u8 as *const libc::c_char,
                            b"DWARF error: DW_AT_comp_dir attribute encountered with a non-string form\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    comp_dir = 0 as *mut libc::c_char;
                }
                if !comp_dir.is_null() {
                    let mut cp: *mut libc::c_char = strchr(comp_dir, ':' as i32);
                    if !cp.is_null() && cp != comp_dir
                        && *cp.offset(-(1 as libc::c_int) as isize) as libc::c_int
                            == '.' as i32
                        && *cp.offset(1 as libc::c_int as isize) as libc::c_int
                            == '/' as i32
                    {
                        comp_dir = cp.offset(1 as libc::c_int as isize);
                    }
                }
                (*unit).comp_dir = comp_dir;
            }
            19 => {
                (*unit).lang = attr.u.val as libc::c_int;
            }
            _ => {}
        }
        i = i.wrapping_add(1);
        i;
    }
    if high_pc_relative {
        high_pc = (high_pc as libc::c_ulong).wrapping_add(low_pc) as bfd_vma as bfd_vma;
    }
    if high_pc != 0 as libc::c_int as libc::c_ulong {
        if !arange_add(unit, &mut (*unit).arange, low_pc, high_pc) {
            return 0 as *mut comp_unit;
        }
    }
    (*unit).first_child_die_ptr = info_ptr;
    return unit;
}
unsafe extern "C" fn comp_unit_contains_address(
    mut unit: *mut comp_unit,
    mut addr: bfd_vma,
) -> bool {
    let mut arange: *mut arange = 0 as *mut arange;
    if (*unit).error != 0 {
        return 0 as libc::c_int != 0;
    }
    arange = &mut (*unit).arange;
    loop {
        if addr >= (*arange).low && addr < (*arange).high {
            return 1 as libc::c_int != 0;
        }
        arange = (*arange).next;
        if arange.is_null() {
            break;
        }
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn comp_unit_find_nearest_line(
    mut unit: *mut comp_unit,
    mut addr: bfd_vma,
    mut filename_ptr: *mut *const libc::c_char,
    mut function_ptr: *mut *mut funcinfo,
    mut linenumber_ptr: *mut libc::c_uint,
    mut discriminator_ptr: *mut libc::c_uint,
) -> bfd_vma {
    let mut func_p: bool = false;
    if !comp_unit_maybe_decode_line_info(unit) {
        return 0 as libc::c_int as bfd_vma;
    }
    *function_ptr = 0 as *mut funcinfo;
    func_p = lookup_address_in_function_table(unit, addr, function_ptr);
    if func_p as libc::c_int != 0
        && (**function_ptr).tag == DW_TAG_inlined_subroutine as libc::c_int
    {
        (*(*unit).stash).inliner_chain = *function_ptr;
    }
    return lookup_address_in_line_info_table(
        (*unit).line_table,
        addr,
        filename_ptr,
        linenumber_ptr,
        discriminator_ptr,
    );
}
unsafe extern "C" fn comp_unit_maybe_decode_line_info(mut unit: *mut comp_unit) -> bool {
    if (*unit).error != 0 {
        return 0 as libc::c_int != 0;
    }
    if ((*unit).line_table).is_null() {
        if (*unit).stmtlist == 0 {
            (*unit).error = 1 as libc::c_int;
            return 0 as libc::c_int != 0;
        }
        (*unit).line_table = decode_line_info(unit);
        if ((*unit).line_table).is_null() {
            (*unit).error = 1 as libc::c_int;
            return 0 as libc::c_int != 0;
        }
        if (*unit).first_child_die_ptr < (*unit).end_ptr && !scan_unit_for_symbols(unit)
        {
            (*unit).error = 1 as libc::c_int;
            return 0 as libc::c_int != 0;
        }
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn comp_unit_find_line(
    mut unit: *mut comp_unit,
    mut sym: *mut asymbol,
    mut addr: bfd_vma,
    mut filename_ptr: *mut *const libc::c_char,
    mut linenumber_ptr: *mut libc::c_uint,
) -> bool {
    if !comp_unit_maybe_decode_line_info(unit) {
        return 0 as libc::c_int != 0;
    }
    if (*sym).flags & ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_uint != 0 {
        return lookup_symbol_in_function_table(
            unit,
            sym,
            addr,
            filename_ptr,
            linenumber_ptr,
        );
    }
    return lookup_symbol_in_variable_table(
        unit,
        sym,
        addr,
        filename_ptr,
        linenumber_ptr,
    );
}
unsafe extern "C" fn reverse_funcinfo_list(mut head: *mut funcinfo) -> *mut funcinfo {
    let mut rhead: *mut funcinfo = 0 as *mut funcinfo;
    let mut temp: *mut funcinfo = 0 as *mut funcinfo;
    rhead = 0 as *mut funcinfo;
    while !head.is_null() {
        temp = (*head).prev_func;
        (*head).prev_func = rhead;
        rhead = head;
        head = temp;
    }
    return rhead;
}
unsafe extern "C" fn reverse_varinfo_list(mut head: *mut varinfo) -> *mut varinfo {
    let mut rhead: *mut varinfo = 0 as *mut varinfo;
    let mut temp: *mut varinfo = 0 as *mut varinfo;
    rhead = 0 as *mut varinfo;
    while !head.is_null() {
        temp = (*head).prev_var;
        (*head).prev_var = rhead;
        rhead = head;
        head = temp;
    }
    return rhead;
}
unsafe extern "C" fn comp_unit_hash_info(
    mut stash: *mut dwarf2_debug,
    mut unit: *mut comp_unit,
    mut funcinfo_hash_table: *mut info_hash_table,
    mut varinfo_hash_table: *mut info_hash_table,
) -> bool {
    let mut each_func: *mut funcinfo = 0 as *mut funcinfo;
    let mut each_var: *mut varinfo = 0 as *mut varinfo;
    let mut okay: bool = 1 as libc::c_int != 0;
    if !((*stash).info_hash_status != 2 as libc::c_int) {
        bfd_assert(
            b"./dwarf2.c\0" as *const u8 as *const libc::c_char,
            4021 as libc::c_int,
        );
    }
    if !comp_unit_maybe_decode_line_info(unit) {
        return 0 as libc::c_int != 0;
    }
    if (*unit).cached {
        bfd_assert(
            b"./dwarf2.c\0" as *const u8 as *const libc::c_char,
            4026 as libc::c_int,
        );
    }
    (*unit).function_table = reverse_funcinfo_list((*unit).function_table);
    each_func = (*unit).function_table;
    while !each_func.is_null() && okay as libc::c_int != 0 {
        if !((*each_func).name).is_null() {
            okay = insert_info_hash_table(
                funcinfo_hash_table,
                (*each_func).name,
                each_func as *mut libc::c_void,
                0 as libc::c_int != 0,
            );
        }
        each_func = (*each_func).prev_func;
    }
    (*unit).function_table = reverse_funcinfo_list((*unit).function_table);
    if !okay {
        return 0 as libc::c_int != 0;
    }
    (*unit).variable_table = reverse_varinfo_list((*unit).variable_table);
    each_var = (*unit).variable_table;
    while !each_var.is_null() && okay as libc::c_int != 0 {
        if !(*each_var).stack && !((*each_var).file).is_null()
            && !((*each_var).name).is_null()
        {
            okay = insert_info_hash_table(
                varinfo_hash_table,
                (*each_var).name,
                each_var as *mut libc::c_void,
                0 as libc::c_int != 0,
            );
        }
        each_var = (*each_var).prev_var;
    }
    (*unit).variable_table = reverse_varinfo_list((*unit).variable_table);
    (*unit).cached = 1 as libc::c_int != 0;
    return okay;
}
unsafe extern "C" fn find_debug_info(
    mut abfd: *mut bfd,
    mut debug_sections: *const dwarf_debug_section,
    mut after_sec: *mut asection,
) -> *mut asection {
    let mut msec: *mut asection = 0 as *mut asection;
    let mut look: *const libc::c_char = 0 as *const libc::c_char;
    if after_sec.is_null() {
        look = (*debug_sections.offset(debug_info as libc::c_int as isize))
            .uncompressed_name;
        msec = bfd_get_section_by_name(abfd, look);
        if !msec.is_null() {
            return msec;
        }
        look = (*debug_sections.offset(debug_info as libc::c_int as isize))
            .compressed_name;
        msec = bfd_get_section_by_name(abfd, look);
        if !msec.is_null() {
            return msec;
        }
        msec = (*abfd).sections;
        while !msec.is_null() {
            if startswith(
                (*msec).name,
                b".gnu.linkonce.wi.\0" as *const u8 as *const libc::c_char,
            ) {
                return msec;
            }
            msec = (*msec).next;
        }
        return 0 as *mut asection;
    }
    msec = (*after_sec).next;
    while !msec.is_null() {
        look = (*debug_sections.offset(debug_info as libc::c_int as isize))
            .uncompressed_name;
        if strcmp((*msec).name, look) == 0 as libc::c_int {
            return msec;
        }
        look = (*debug_sections.offset(debug_info as libc::c_int as isize))
            .compressed_name;
        if !look.is_null() && strcmp((*msec).name, look) == 0 as libc::c_int {
            return msec;
        }
        if startswith(
            (*msec).name,
            b".gnu.linkonce.wi.\0" as *const u8 as *const libc::c_char,
        ) {
            return msec;
        }
        msec = (*msec).next;
    }
    return 0 as *mut asection;
}
unsafe extern "C" fn set_debug_vma(mut orig_bfd: *mut bfd, mut debug_bfd: *mut bfd) {
    let mut s: *mut asection = 0 as *mut asection;
    let mut d: *mut asection = 0 as *mut asection;
    s = (*orig_bfd).sections;
    d = (*debug_bfd).sections;
    while !s.is_null() && !d.is_null() {
        if (*d).flags & 0x2000 as libc::c_int as libc::c_uint
            != 0 as libc::c_int as libc::c_uint
        {
            break;
        }
        if strcmp((*s).name, (*d).name) == 0 as libc::c_int {
            (*d).output_section = (*s).output_section;
            (*d).output_offset = (*s).output_offset;
            (*d).vma = (*s).vma;
        }
        s = (*s).next;
        d = (*d).next;
    }
}
unsafe extern "C" fn _bfd_dwarf2_stash_syms(
    mut stash: *mut dwarf2_debug,
    mut abfd: *mut bfd,
    mut sec: *mut *mut asection,
    mut syms: *mut *mut *mut asymbol,
) {
    if (*stash).f.bfd_ptr != abfd {
        let mut s: *mut asection = 0 as *mut asection;
        let mut d: *mut asection = 0 as *mut asection;
        if (*sec).is_null() {
            *syms = (*stash).f.syms;
            return;
        }
        s = (*abfd).sections;
        d = (*(*stash).f.bfd_ptr).sections;
        while !s.is_null() && !d.is_null() {
            if (*d).flags & 0x2000 as libc::c_int as libc::c_uint
                != 0 as libc::c_int as libc::c_uint
            {
                break;
            }
            if s == *sec && strcmp((*s).name, (*d).name) == 0 as libc::c_int {
                *sec = d;
                *syms = (*stash).f.syms;
                break;
            } else {
                s = (*s).next;
                d = (*d).next;
            }
        }
    }
}
unsafe extern "C" fn unset_sections(mut stash: *mut dwarf2_debug) {
    let mut i: libc::c_int = 0;
    let mut p: *mut adjusted_section = 0 as *mut adjusted_section;
    i = (*stash).adjusted_section_count;
    p = (*stash).adjusted_sections;
    while i > 0 as libc::c_int {
        (*(*p).section).vma = 0 as libc::c_int as bfd_vma;
        i -= 1;
        i;
        p = p.offset(1);
        p;
    }
}
unsafe extern "C" fn place_sections(
    mut orig_bfd: *mut bfd,
    mut stash: *mut dwarf2_debug,
) -> bool {
    let mut abfd: *mut bfd = 0 as *mut bfd;
    let mut p: *mut adjusted_section = 0 as *mut adjusted_section;
    let mut i: libc::c_int = 0;
    let mut debug_info_name: *const libc::c_char = 0 as *const libc::c_char;
    if (*stash).adjusted_section_count != 0 as libc::c_int {
        i = (*stash).adjusted_section_count;
        p = (*stash).adjusted_sections;
        while i > 0 as libc::c_int {
            (*(*p).section).vma = (*p).adj_vma;
            i -= 1;
            i;
            p = p.offset(1);
            p;
        }
        return 1 as libc::c_int != 0;
    }
    debug_info_name = (*((*stash).debug_sections)
        .offset(debug_info as libc::c_int as isize))
        .uncompressed_name;
    i = 0 as libc::c_int;
    abfd = orig_bfd;
    loop {
        let mut sect: *mut asection = 0 as *mut asection;
        sect = (*abfd).sections;
        while !sect.is_null() {
            let mut is_debug_info: libc::c_int = 0;
            if !(!((*sect).output_section).is_null() && (*sect).output_section != sect
                && (*sect).flags & 0x2000 as libc::c_int as libc::c_uint
                    == 0 as libc::c_int as libc::c_uint
                || (*sect).vma != 0 as libc::c_int as libc::c_ulong)
            {
                is_debug_info = (strcmp((*sect).name, debug_info_name)
                    == 0 as libc::c_int
                    || startswith(
                        (*sect).name,
                        b".gnu.linkonce.wi.\0" as *const u8 as *const libc::c_char,
                    ) as libc::c_int != 0) as libc::c_int;
                if !(!((*sect).flags & 0x1 as libc::c_int as libc::c_uint
                    != 0 as libc::c_int as libc::c_uint && abfd == orig_bfd)
                    && is_debug_info == 0)
                {
                    i += 1;
                    i;
                }
            }
            sect = (*sect).next;
        }
        if abfd == (*stash).f.bfd_ptr {
            break;
        }
        abfd = (*stash).f.bfd_ptr;
    }
    if i <= 1 as libc::c_int {
        (*stash).adjusted_section_count = -(1 as libc::c_int);
    } else {
        let mut last_vma: bfd_vma = 0 as libc::c_int as bfd_vma;
        let mut last_dwarf: bfd_vma = 0 as libc::c_int as bfd_vma;
        let mut amt: size_t = (i as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<adjusted_section>() as libc::c_ulong);
        p = bfd_malloc(amt) as *mut adjusted_section;
        if p.is_null() {
            return 0 as libc::c_int != 0;
        }
        (*stash).adjusted_sections = p;
        (*stash).adjusted_section_count = i;
        abfd = orig_bfd;
        loop {
            let mut sect_0: *mut asection = 0 as *mut asection;
            sect_0 = (*abfd).sections;
            while !sect_0.is_null() {
                let mut sz: bfd_size_type = 0;
                let mut is_debug_info_0: libc::c_int = 0;
                if !(!((*sect_0).output_section).is_null()
                    && (*sect_0).output_section != sect_0
                    && (*sect_0).flags & 0x2000 as libc::c_int as libc::c_uint
                        == 0 as libc::c_int as libc::c_uint
                    || (*sect_0).vma != 0 as libc::c_int as libc::c_ulong)
                {
                    is_debug_info_0 = (strcmp((*sect_0).name, debug_info_name)
                        == 0 as libc::c_int
                        || startswith(
                            (*sect_0).name,
                            b".gnu.linkonce.wi.\0" as *const u8 as *const libc::c_char,
                        ) as libc::c_int != 0) as libc::c_int;
                    if !(!((*sect_0).flags & 0x1 as libc::c_int as libc::c_uint
                        != 0 as libc::c_int as libc::c_uint && abfd == orig_bfd)
                        && is_debug_info_0 == 0)
                    {
                        sz = if (*sect_0).rawsize != 0 {
                            (*sect_0).rawsize
                        } else {
                            (*sect_0).size
                        };
                        if is_debug_info_0 != 0 {
                            if !((*sect_0).alignment_power
                                == 0 as libc::c_int as libc::c_uint)
                            {
                                bfd_assert(
                                    b"./dwarf2.c\0" as *const u8 as *const libc::c_char,
                                    4299 as libc::c_int,
                                );
                            }
                            (*sect_0).vma = last_dwarf;
                            last_dwarf = (last_dwarf as libc::c_ulong).wrapping_add(sz)
                                as bfd_vma as bfd_vma;
                        } else {
                            last_vma = last_vma
                                .wrapping_add(
                                    !((1 as libc::c_int as bfd_vma)
                                        << (*sect_0).alignment_power)
                                        .wrapping_neg(),
                                )
                                & ((1 as libc::c_int as bfd_vma)
                                    << (*sect_0).alignment_power)
                                    .wrapping_neg();
                            (*sect_0).vma = last_vma;
                            last_vma = (last_vma as libc::c_ulong).wrapping_add(sz)
                                as bfd_vma as bfd_vma;
                        }
                        (*p).section = sect_0;
                        (*p).adj_vma = (*sect_0).vma;
                        p = p.offset(1);
                        p;
                    }
                }
                sect_0 = (*sect_0).next;
            }
            if abfd == (*stash).f.bfd_ptr {
                break;
            }
            abfd = (*stash).f.bfd_ptr;
        }
    }
    if orig_bfd != (*stash).f.bfd_ptr {
        set_debug_vma(orig_bfd, (*stash).f.bfd_ptr);
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn info_hash_lookup_funcinfo(
    mut hash_table: *mut info_hash_table,
    mut sym: *mut asymbol,
    mut addr: bfd_vma,
    mut filename_ptr: *mut *const libc::c_char,
    mut linenumber_ptr: *mut libc::c_uint,
) -> bool {
    let mut each_func: *mut funcinfo = 0 as *mut funcinfo;
    let mut best_fit: *mut funcinfo = 0 as *mut funcinfo;
    let mut best_fit_len: bfd_vma = 0 as libc::c_int as bfd_vma;
    let mut node: *mut info_list_node = 0 as *mut info_list_node;
    let mut arange: *mut arange = 0 as *mut arange;
    let mut name: *const libc::c_char = bfd_asymbol_name(sym);
    let mut sec: *mut asection = bfd_asymbol_section(sym);
    node = lookup_info_hash_table(hash_table, name);
    while !node.is_null() {
        each_func = (*node).info as *mut funcinfo;
        arange = &mut (*each_func).arange;
        while !arange.is_null() {
            if (((*each_func).sec).is_null() || (*each_func).sec == sec)
                && addr >= (*arange).low && addr < (*arange).high
                && (best_fit.is_null()
                    || ((*arange).high).wrapping_sub((*arange).low) < best_fit_len)
            {
                best_fit = each_func;
                best_fit_len = ((*arange).high).wrapping_sub((*arange).low);
            }
            arange = (*arange).next;
        }
        node = (*node).next;
    }
    if !best_fit.is_null() {
        (*best_fit).sec = sec;
        *filename_ptr = (*best_fit).file;
        *linenumber_ptr = (*best_fit).line as libc::c_uint;
        return 1 as libc::c_int != 0;
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn info_hash_lookup_varinfo(
    mut hash_table: *mut info_hash_table,
    mut sym: *mut asymbol,
    mut addr: bfd_vma,
    mut filename_ptr: *mut *const libc::c_char,
    mut linenumber_ptr: *mut libc::c_uint,
) -> bool {
    let mut name: *const libc::c_char = bfd_asymbol_name(sym);
    let mut sec: *mut asection = bfd_asymbol_section(sym);
    let mut each: *mut varinfo = 0 as *mut varinfo;
    let mut node: *mut info_list_node = 0 as *mut info_list_node;
    node = lookup_info_hash_table(hash_table, name);
    while !node.is_null() {
        each = (*node).info as *mut varinfo;
        if (*each).addr == addr && (((*each).sec).is_null() || (*each).sec == sec) {
            (*each).sec = sec;
            *filename_ptr = (*each).file;
            *linenumber_ptr = (*each).line as libc::c_uint;
            return 1 as libc::c_int != 0;
        }
        node = (*node).next;
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn stash_maybe_update_info_hash_tables(
    mut stash: *mut dwarf2_debug,
) -> bool {
    let mut each: *mut comp_unit = 0 as *mut comp_unit;
    if (*stash).f.all_comp_units == (*stash).hash_units_head {
        return 1 as libc::c_int != 0;
    }
    if !((*stash).hash_units_head).is_null() {
        each = (*(*stash).hash_units_head).prev_unit;
    } else {
        each = (*stash).f.last_comp_unit;
    }
    while !each.is_null() {
        if !comp_unit_hash_info(
            stash,
            each,
            (*stash).funcinfo_hash_table,
            (*stash).varinfo_hash_table,
        ) {
            (*stash).info_hash_status = 2 as libc::c_int;
            return 0 as libc::c_int != 0;
        }
        each = (*each).prev_unit;
    }
    (*stash).hash_units_head = (*stash).f.all_comp_units;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn stash_maybe_enable_info_hash_tables(
    mut abfd: *mut bfd,
    mut stash: *mut dwarf2_debug,
) {
    if !((*stash).info_hash_status == 0 as libc::c_int) {
        bfd_assert(
            b"./dwarf2.c\0" as *const u8 as *const libc::c_char,
            4513 as libc::c_int,
        );
    }
    let fresh15 = (*stash).info_hash_count;
    (*stash).info_hash_count = (*stash).info_hash_count + 1;
    if fresh15 < 100 as libc::c_int {
        return;
    }
    (*stash).funcinfo_hash_table = create_info_hash_table(abfd);
    (*stash).varinfo_hash_table = create_info_hash_table(abfd);
    if ((*stash).funcinfo_hash_table).is_null()
        || ((*stash).varinfo_hash_table).is_null()
    {
        (*stash).info_hash_status = 2 as libc::c_int;
        return;
    }
    if stash_maybe_update_info_hash_tables(stash) {
        (*stash).info_hash_status = 1 as libc::c_int;
    }
}
unsafe extern "C" fn stash_find_line_fast(
    mut stash: *mut dwarf2_debug,
    mut sym: *mut asymbol,
    mut addr: bfd_vma,
    mut filename_ptr: *mut *const libc::c_char,
    mut linenumber_ptr: *mut libc::c_uint,
) -> bool {
    if !((*stash).info_hash_status == 1 as libc::c_int) {
        bfd_assert(
            b"./dwarf2.c\0" as *const u8 as *const libc::c_char,
            4549 as libc::c_int,
        );
    }
    if (*sym).flags & ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_uint != 0 {
        return info_hash_lookup_funcinfo(
            (*stash).funcinfo_hash_table,
            sym,
            addr,
            filename_ptr,
            linenumber_ptr,
        );
    }
    return info_hash_lookup_varinfo(
        (*stash).varinfo_hash_table,
        sym,
        addr,
        filename_ptr,
        linenumber_ptr,
    );
}
unsafe extern "C" fn save_section_vma(
    mut abfd: *const bfd,
    mut stash: *mut dwarf2_debug,
) -> bool {
    let mut s: *mut asection = 0 as *mut asection;
    let mut i: libc::c_uint = 0;
    if (*abfd).section_count == 0 as libc::c_int as libc::c_uint {
        return 1 as libc::c_int != 0;
    }
    (*stash)
        .sec_vma = bfd_malloc(
        (::core::mem::size_of::<bfd_vma>() as libc::c_ulong)
            .wrapping_mul((*abfd).section_count as libc::c_ulong),
    ) as *mut bfd_vma;
    if ((*stash).sec_vma).is_null() {
        return 0 as libc::c_int != 0;
    }
    (*stash).sec_vma_count = (*abfd).section_count;
    i = 0 as libc::c_int as libc::c_uint;
    s = (*abfd).sections;
    while !s.is_null() && i < (*abfd).section_count {
        if !((*s).output_section).is_null() {
            *((*stash).sec_vma)
                .offset(
                    i as isize,
                ) = ((*(*s).output_section).vma).wrapping_add((*s).output_offset);
        } else {
            *((*stash).sec_vma).offset(i as isize) = (*s).vma;
        }
        i = i.wrapping_add(1);
        i;
        s = (*s).next;
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn section_vma_same(
    mut abfd: *const bfd,
    mut stash: *const dwarf2_debug,
) -> bool {
    let mut s: *mut asection = 0 as *mut asection;
    let mut i: libc::c_uint = 0;
    if (*abfd).section_count != (*stash).sec_vma_count {
        return 0 as libc::c_int != 0;
    }
    i = 0 as libc::c_int as libc::c_uint;
    s = (*abfd).sections;
    while !s.is_null() && i < (*abfd).section_count {
        let mut vma: bfd_vma = 0;
        if !((*s).output_section).is_null() {
            vma = ((*(*s).output_section).vma).wrapping_add((*s).output_offset);
        } else {
            vma = (*s).vma;
        }
        if vma != *((*stash).sec_vma).offset(i as isize) {
            return 0 as libc::c_int != 0;
        }
        i = i.wrapping_add(1);
        i;
        s = (*s).next;
    }
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn _bfd_dwarf2_slurp_debug_info(
    mut abfd: *mut bfd,
    mut debug_bfd: *mut bfd,
    mut debug_sections: *const dwarf_debug_section,
    mut symbols: *mut *mut asymbol,
    mut pinfo: *mut *mut libc::c_void,
    mut do_place: bool,
) -> bool {
    let mut amt: size_t = ::core::mem::size_of::<dwarf2_debug>() as libc::c_ulong;
    let mut total_size: bfd_size_type = 0;
    let mut msec: *mut asection = 0 as *mut asection;
    let mut stash: *mut dwarf2_debug = *pinfo as *mut dwarf2_debug;
    if !stash.is_null() {
        if (*stash).orig_bfd == abfd && section_vma_same(abfd, stash) as libc::c_int != 0
        {
            if !((*stash).f.bfd_ptr).is_null() {
                if do_place as libc::c_int != 0 && !place_sections(abfd, stash) {
                    return 0 as libc::c_int != 0;
                }
                return 1 as libc::c_int != 0;
            }
            return 0 as libc::c_int != 0;
        }
        _bfd_dwarf2_cleanup_debug_info(abfd, pinfo);
        memset(stash as *mut libc::c_void, 0 as libc::c_int, amt);
    } else {
        stash = bfd_zalloc(abfd, amt) as *mut dwarf2_debug;
        if stash.is_null() {
            return 0 as libc::c_int != 0;
        }
    }
    (*stash).orig_bfd = abfd;
    (*stash).debug_sections = debug_sections;
    (*stash).f.syms = symbols;
    if !save_section_vma(abfd, stash) {
        return 0 as libc::c_int != 0;
    }
    (*stash)
        .f
        .abbrev_offsets = htab_create_alloc(
        10 as libc::c_int as size_t,
        Some(hash_abbrev as unsafe extern "C" fn(*const libc::c_void) -> hashval_t),
        Some(
            eq_abbrev
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
        Some(del_abbrev as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        Some(
            calloc
                as unsafe extern "C" fn(
                    libc::c_ulong,
                    libc::c_ulong,
                ) -> *mut libc::c_void,
        ),
        Some(free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
    );
    if ((*stash).f.abbrev_offsets).is_null() {
        return 0 as libc::c_int != 0;
    }
    (*stash)
        .alt
        .abbrev_offsets = htab_create_alloc(
        10 as libc::c_int as size_t,
        Some(hash_abbrev as unsafe extern "C" fn(*const libc::c_void) -> hashval_t),
        Some(
            eq_abbrev
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
        Some(del_abbrev as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        Some(
            calloc
                as unsafe extern "C" fn(
                    libc::c_ulong,
                    libc::c_ulong,
                ) -> *mut libc::c_void,
        ),
        Some(free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
    );
    if ((*stash).alt.abbrev_offsets).is_null() {
        return 0 as libc::c_int != 0;
    }
    *pinfo = stash as *mut libc::c_void;
    if debug_bfd.is_null() {
        debug_bfd = abfd;
    }
    msec = find_debug_info(debug_bfd, debug_sections, 0 as *mut asection);
    if msec.is_null() && abfd == debug_bfd {
        let mut debug_filename: *mut libc::c_char = 0 as *mut libc::c_char;
        debug_filename = bfd_follow_build_id_debuglink(
            abfd,
            b"/usr/local/lib/debug\0" as *const u8 as *const libc::c_char,
        );
        if debug_filename.is_null() {
            debug_filename = bfd_follow_gnu_debuglink(
                abfd,
                b"/usr/local/lib/debug\0" as *const u8 as *const libc::c_char,
            );
        }
        if debug_filename.is_null() {
            return 0 as libc::c_int != 0;
        }
        debug_bfd = bfd_openr(debug_filename, 0 as *const libc::c_char);
        free(debug_filename as *mut libc::c_void);
        if debug_bfd.is_null() {
            return 0 as libc::c_int != 0;
        }
        (*debug_bfd).flags |= 0x8000 as libc::c_int as libc::c_uint;
        if !bfd_check_format(debug_bfd, bfd_object)
            || {
                msec = find_debug_info(debug_bfd, debug_sections, 0 as *mut asection);
                msec.is_null()
            } || !bfd_generic_link_read_symbols(debug_bfd)
        {
            bfd_close(debug_bfd);
            return 0 as libc::c_int != 0;
        }
        symbols = bfd_get_outsymbols(debug_bfd);
        (*stash).f.syms = symbols;
        (*stash).close_on_cleanup = 1 as libc::c_int != 0;
    }
    (*stash).f.bfd_ptr = debug_bfd;
    if do_place as libc::c_int != 0 && !place_sections(abfd, stash) {
        return 0 as libc::c_int != 0;
    }
    if (find_debug_info(debug_bfd, debug_sections, msec)).is_null() {
        total_size = (*msec).size;
        if !read_section(
            debug_bfd,
            &*((*stash).debug_sections).offset(debug_info as libc::c_int as isize),
            symbols,
            0 as libc::c_int as bfd_uint64_t,
            &mut (*stash).f.dwarf_info_buffer,
            &mut total_size,
        ) {
            return 0 as libc::c_int != 0;
        }
    } else {
        total_size = 0 as libc::c_int as bfd_size_type;
        while !msec.is_null() {
            if total_size.wrapping_add((*msec).size) < total_size
                || total_size.wrapping_add((*msec).size) < (*msec).size
            {
                bfd_set_error(bfd_error_no_memory);
                return 0 as libc::c_int != 0;
            }
            total_size = (total_size as libc::c_ulong).wrapping_add((*msec).size)
                as bfd_size_type as bfd_size_type;
            msec = find_debug_info(debug_bfd, debug_sections, msec);
        }
        (*stash).f.dwarf_info_buffer = bfd_malloc(total_size) as *mut bfd_byte;
        if ((*stash).f.dwarf_info_buffer).is_null() {
            return 0 as libc::c_int != 0;
        }
        total_size = 0 as libc::c_int as bfd_size_type;
        msec = find_debug_info(debug_bfd, debug_sections, 0 as *mut asection);
        while !msec.is_null() {
            let mut size: bfd_size_type = 0;
            size = (*msec).size;
            if !(size == 0 as libc::c_int as libc::c_ulong) {
                if (bfd_simple_get_relocated_section_contents(
                    debug_bfd,
                    msec,
                    ((*stash).f.dwarf_info_buffer).offset(total_size as isize),
                    symbols,
                ))
                    .is_null()
                {
                    return 0 as libc::c_int != 0;
                }
                total_size = (total_size as libc::c_ulong).wrapping_add(size)
                    as bfd_size_type as bfd_size_type;
            }
            msec = find_debug_info(debug_bfd, debug_sections, msec);
        }
    }
    (*stash).f.info_ptr = (*stash).f.dwarf_info_buffer;
    (*stash).f.dwarf_info_size = total_size;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn stash_comp_unit(
    mut stash: *mut dwarf2_debug,
    mut file: *mut dwarf2_debug_file,
) -> *mut comp_unit {
    let mut length: bfd_size_type = 0;
    let mut offset_size: libc::c_uint = 0;
    let mut info_ptr_unit: *mut bfd_byte = (*file).info_ptr;
    let mut info_ptr_end: *mut bfd_byte = ((*file).dwarf_info_buffer)
        .offset((*file).dwarf_info_size as isize);
    if (*file).info_ptr >= info_ptr_end {
        return 0 as *mut comp_unit;
    }
    length = read_4_bytes((*file).bfd_ptr, &mut (*file).info_ptr, info_ptr_end)
        as bfd_size_type;
    if length == 0xffffffff as libc::c_uint as libc::c_ulong {
        offset_size = 8 as libc::c_int as libc::c_uint;
        length = read_8_bytes((*file).bfd_ptr, &mut (*file).info_ptr, info_ptr_end);
    } else if length == 0 as libc::c_int as libc::c_ulong {
        offset_size = 8 as libc::c_int as libc::c_uint;
        length = read_4_bytes((*file).bfd_ptr, &mut (*file).info_ptr, info_ptr_end)
            as bfd_size_type;
    } else {
        offset_size = 4 as libc::c_int as libc::c_uint;
    }
    if length != 0 as libc::c_int as libc::c_ulong
        && length <= info_ptr_end.offset_from((*file).info_ptr) as libc::c_long as size_t
    {
        let mut each: *mut comp_unit = parse_comp_unit(
            stash,
            file,
            (*file).info_ptr,
            length,
            info_ptr_unit,
            offset_size,
        );
        if !each.is_null() {
            if !((*file).all_comp_units).is_null() {
                (*(*file).all_comp_units).prev_unit = each;
            } else {
                (*file).last_comp_unit = each;
            }
            (*each).next_unit = (*file).all_comp_units;
            (*file).all_comp_units = each;
            (*file).info_ptr = ((*file).info_ptr).offset(length as isize);
            return each;
        }
    }
    (*file).info_ptr = info_ptr_end;
    return 0 as *mut comp_unit;
}
unsafe extern "C" fn hash_asymbol(mut sym: *const libc::c_void) -> hashval_t {
    let mut asym: *const asymbol = sym as *const asymbol;
    return htab_hash_string((*asym).name as *const libc::c_void);
}
unsafe extern "C" fn eq_asymbol(
    mut a: *const libc::c_void,
    mut b: *const libc::c_void,
) -> libc::c_int {
    let mut sa: *const asymbol = a as *const asymbol;
    let mut sb: *const asymbol = b as *const asymbol;
    return (strcmp((*sa).name, (*sb).name) == 0 as libc::c_int) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _bfd_dwarf2_find_symbol_bias(
    mut symbols: *mut *mut asymbol,
    mut pinfo: *mut *mut libc::c_void,
) -> bfd_signed_vma {
    let mut stash: *mut dwarf2_debug = 0 as *mut dwarf2_debug;
    let mut unit: *mut comp_unit = 0 as *mut comp_unit;
    let mut sym_hash: htab_t = 0 as *mut htab;
    let mut result: bfd_signed_vma = 0 as libc::c_int as bfd_signed_vma;
    let mut psym: *mut *mut asymbol = 0 as *mut *mut asymbol;
    stash = *pinfo as *mut dwarf2_debug;
    if stash.is_null() || symbols.is_null() {
        return 0 as libc::c_int as bfd_signed_vma;
    }
    sym_hash = htab_create_alloc(
        10 as libc::c_int as size_t,
        Some(hash_asymbol as unsafe extern "C" fn(*const libc::c_void) -> hashval_t),
        Some(
            eq_asymbol
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
        None,
        Some(xcalloc as unsafe extern "C" fn(size_t, size_t) -> *mut libc::c_void),
        Some(free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
    );
    psym = symbols;
    while !(*psym).is_null() {
        let mut sym: *mut asymbol = *psym;
        if (*sym).flags & ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_uint != 0
            && !((*sym).section).is_null()
        {
            let mut slot: *mut *mut libc::c_void = htab_find_slot(
                sym_hash,
                sym as *const libc::c_void,
                INSERT,
            );
            *slot = sym as *mut libc::c_void;
        }
        psym = psym.offset(1);
        psym;
    }
    unit = (*stash).f.all_comp_units;
    's_56: while !unit.is_null() {
        let mut func: *mut funcinfo = 0 as *mut funcinfo;
        comp_unit_maybe_decode_line_info(unit);
        func = (*unit).function_table;
        while !func.is_null() {
            if !((*func).name).is_null() && (*func).arange.low != 0 {
                let mut search: asymbol = asymbol {
                    the_bfd: 0 as *mut bfd,
                    name: 0 as *const libc::c_char,
                    value: 0,
                    flags: 0,
                    section: 0 as *mut bfd_section,
                    udata: C2RustUnnamed_6 {
                        p: 0 as *mut libc::c_void,
                    },
                };
                let mut sym_0: *mut asymbol = 0 as *mut asymbol;
                search.name = (*func).name;
                sym_0 = htab_find(
                    sym_hash,
                    &mut search as *mut asymbol as *const libc::c_void,
                ) as *mut asymbol;
                if !sym_0.is_null() {
                    result = (*func).arange.low as bfd_signed_vma
                        - ((*sym_0).value).wrapping_add((*(*sym_0).section).vma)
                            as bfd_signed_vma;
                    break 's_56;
                }
            }
            func = (*func).prev_func;
        }
        unit = (*unit).next_unit;
    }
    htab_delete(sym_hash);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn _bfd_dwarf2_find_nearest_line(
    mut abfd: *mut bfd,
    mut symbols: *mut *mut asymbol,
    mut symbol: *mut asymbol,
    mut section: *mut asection,
    mut offset: bfd_vma,
    mut filename_ptr: *mut *const libc::c_char,
    mut functionname_ptr: *mut *const libc::c_char,
    mut linenumber_ptr: *mut libc::c_uint,
    mut discriminator_ptr: *mut libc::c_uint,
    mut debug_sections: *const dwarf_debug_section,
    mut pinfo: *mut *mut libc::c_void,
) -> libc::c_int {
    let mut current_block: u64;
    let mut stash: *mut dwarf2_debug = 0 as *mut dwarf2_debug;
    let mut addr: bfd_vma = 0;
    let mut each: *mut comp_unit = 0 as *mut comp_unit;
    let mut function: *mut funcinfo = 0 as *mut funcinfo;
    let mut found: libc::c_int = 0 as libc::c_int;
    let mut do_line: bool = false;
    *filename_ptr = 0 as *const libc::c_char;
    if !functionname_ptr.is_null() {
        *functionname_ptr = 0 as *const libc::c_char;
    }
    *linenumber_ptr = 0 as libc::c_int as libc::c_uint;
    if !discriminator_ptr.is_null() {
        *discriminator_ptr = 0 as libc::c_int as libc::c_uint;
    }
    if !_bfd_dwarf2_slurp_debug_info(
        abfd,
        0 as *mut bfd,
        debug_sections,
        symbols,
        pinfo,
        (*abfd).flags & (0x2 as libc::c_int | 0x40 as libc::c_int) as libc::c_uint
            == 0 as libc::c_int as libc::c_uint,
    ) {
        return 0 as libc::c_int;
    }
    stash = *pinfo as *mut dwarf2_debug;
    do_line = !symbol.is_null();
    if do_line {
        if !(section.is_null() && offset == 0 as libc::c_int as libc::c_ulong
            && functionname_ptr.is_null())
        {
            bfd_assert(
                b"./dwarf2.c\0" as *const u8 as *const libc::c_char,
                4998 as libc::c_int,
            );
        }
        section = bfd_asymbol_section(symbol);
        addr = (*symbol).value;
    } else {
        if !(!section.is_null() && !functionname_ptr.is_null()) {
            bfd_assert(
                b"./dwarf2.c\0" as *const u8 as *const libc::c_char,
                5004 as libc::c_int,
            );
        }
        addr = offset;
        if !symbols.is_null()
            && (*section).flags & 0x10 as libc::c_int as libc::c_uint
                == 0 as libc::c_int as libc::c_uint
        {
            let mut tmp: *mut *mut asymbol = 0 as *mut *mut asymbol;
            tmp = symbols;
            while !(*tmp).is_null() {
                if (**tmp).the_bfd == abfd && (**tmp).section == section
                    && (**tmp).value == offset
                    && (**tmp).flags
                        & ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_uint
                        == 0 as libc::c_int as libc::c_uint
                {
                    symbol = *tmp;
                    do_line = 1 as libc::c_int != 0;
                    if (*symbol).flags
                        & ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uint
                        != 0 as libc::c_int as libc::c_uint
                    {
                        break;
                    }
                }
                tmp = tmp.offset(1);
                tmp;
            }
        }
    }
    if !((*section).output_section).is_null() {
        addr = (addr as libc::c_ulong)
            .wrapping_add(
                ((*(*section).output_section).vma).wrapping_add((*section).output_offset),
            ) as bfd_vma as bfd_vma;
    } else {
        addr = (addr as libc::c_ulong).wrapping_add((*section).vma) as bfd_vma
            as bfd_vma;
    }
    if ((*stash).f.info_ptr).is_null() {
        return 0 as libc::c_int;
    }
    (*stash).inliner_chain = 0 as *mut funcinfo;
    if do_line {
        if (*stash).info_hash_status == 0 as libc::c_int {
            stash_maybe_enable_info_hash_tables(abfd, stash);
        }
        if (*stash).info_hash_status == 1 as libc::c_int {
            stash_maybe_update_info_hash_tables(stash);
        }
        if (*stash).info_hash_status == 1 as libc::c_int {
            found = stash_find_line_fast(
                stash,
                symbol,
                addr,
                filename_ptr,
                linenumber_ptr,
            ) as libc::c_int;
            if found != 0 {
                current_block = 17514171739089497340;
            } else {
                current_block = 5372832139739605200;
            }
        } else {
            each = (*stash).f.all_comp_units;
            loop {
                if each.is_null() {
                    current_block = 5372832139739605200;
                    break;
                }
                if (*symbol).flags
                    & ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_uint
                    == 0 as libc::c_int as libc::c_uint
                    || (*each).arange.high == 0 as libc::c_int as libc::c_ulong
                    || comp_unit_contains_address(each, addr) as libc::c_int != 0
                {
                    found = comp_unit_find_line(
                        each,
                        symbol,
                        addr,
                        filename_ptr,
                        linenumber_ptr,
                    ) as libc::c_int;
                    if found != 0 {
                        current_block = 17514171739089497340;
                        break;
                    }
                }
                each = (*each).next_unit;
            }
        }
    } else {
        let mut min_range: bfd_vma = -(1 as libc::c_int) as bfd_vma;
        let mut local_filename: *const libc::c_char = 0 as *const libc::c_char;
        let mut local_function: *mut funcinfo = 0 as *mut funcinfo;
        let mut local_linenumber: libc::c_uint = 0 as libc::c_int as libc::c_uint;
        let mut local_discriminator: libc::c_uint = 0 as libc::c_int as libc::c_uint;
        each = (*stash).f.all_comp_units;
        while !each.is_null() {
            let mut range: bfd_vma = -(1 as libc::c_int) as bfd_vma;
            found = (((*each).arange.high == 0 as libc::c_int as libc::c_ulong
                || comp_unit_contains_address(each, addr) as libc::c_int != 0)
                && {
                    range = comp_unit_find_nearest_line(
                        each,
                        addr,
                        &mut local_filename,
                        &mut local_function,
                        &mut local_linenumber,
                        &mut local_discriminator,
                    );
                    range != 0 as libc::c_int as libc::c_ulong
                }) as libc::c_int;
            if found != 0 {
                if range <= min_range {
                    if !filename_ptr.is_null() && !local_filename.is_null() {
                        *filename_ptr = local_filename;
                    }
                    if !local_function.is_null() {
                        function = local_function;
                    }
                    if !discriminator_ptr.is_null() && local_discriminator != 0 {
                        *discriminator_ptr = local_discriminator;
                    }
                    if local_linenumber != 0 {
                        *linenumber_ptr = local_linenumber;
                    }
                    min_range = range;
                }
            }
            each = (*each).next_unit;
        }
        if *linenumber_ptr != 0 {
            found = 1 as libc::c_int;
            current_block = 17514171739089497340;
        } else {
            current_block = 5372832139739605200;
        }
    }
    match current_block {
        5372832139739605200 => {
            loop {
                each = stash_comp_unit(stash, &mut (*stash).f);
                if each.is_null() {
                    break;
                }
                if do_line {
                    found = (((*symbol).flags
                        & ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_uint
                        == 0 as libc::c_int as libc::c_uint
                        || (*each).arange.high == 0 as libc::c_int as libc::c_ulong
                        || comp_unit_contains_address(each, addr) as libc::c_int != 0)
                        && comp_unit_find_line(
                            each,
                            symbol,
                            addr,
                            filename_ptr,
                            linenumber_ptr,
                        ) as libc::c_int != 0) as libc::c_int;
                } else {
                    found = (((*each).arange.high == 0 as libc::c_int as libc::c_ulong
                        || comp_unit_contains_address(each, addr) as libc::c_int != 0)
                        && comp_unit_find_nearest_line(
                            each,
                            addr,
                            filename_ptr,
                            &mut function,
                            linenumber_ptr,
                            discriminator_ptr,
                        ) != 0 as libc::c_int as libc::c_ulong) as libc::c_int;
                }
                if found != 0 {
                    break;
                }
            }
        }
        _ => {}
    }
    if !functionname_ptr.is_null() && !function.is_null()
        && (*function).is_linkage as libc::c_int != 0
    {
        *functionname_ptr = (*function).name;
    } else if !functionname_ptr.is_null()
        && ((*functionname_ptr).is_null()
            || !function.is_null() && !(*function).is_linkage)
    {
        let mut fun: *mut asymbol = 0 as *mut asymbol;
        let mut syms: *mut *mut asymbol = symbols;
        let mut sec: *mut asection = section;
        _bfd_dwarf2_stash_syms(stash, abfd, &mut sec, &mut syms);
        fun = _bfd_elf_find_function(
            abfd,
            syms,
            sec,
            offset,
            if !(*filename_ptr).is_null() {
                0 as *mut *const libc::c_char
            } else {
                filename_ptr
            },
            functionname_ptr,
        );
        if found == 0 && !fun.is_null() {
            found = 2 as libc::c_int;
        }
        if !function.is_null() && !(*function).is_linkage {
            let mut sec_vma: bfd_vma = 0;
            sec_vma = (*section).vma;
            if !((*section).output_section).is_null() {
                sec_vma = ((*(*section).output_section).vma)
                    .wrapping_add((*section).output_offset);
            }
            if !fun.is_null()
                && ((*fun).value).wrapping_add(sec_vma) == (*function).arange.low
            {
                (*function).name = *functionname_ptr;
            }
            (*function).is_linkage = 1 as libc::c_int != 0;
        }
    }
    if (*abfd).flags & (0x2 as libc::c_int | 0x40 as libc::c_int) as libc::c_uint
        == 0 as libc::c_int as libc::c_uint
    {
        unset_sections(stash);
    }
    return found;
}
#[no_mangle]
pub unsafe extern "C" fn _bfd_dwarf2_find_inliner_info(
    mut abfd: *mut bfd,
    mut filename_ptr: *mut *const libc::c_char,
    mut functionname_ptr: *mut *const libc::c_char,
    mut linenumber_ptr: *mut libc::c_uint,
    mut pinfo: *mut *mut libc::c_void,
) -> bool {
    let mut stash: *mut dwarf2_debug = 0 as *mut dwarf2_debug;
    stash = *pinfo as *mut dwarf2_debug;
    if !stash.is_null() {
        let mut func: *mut funcinfo = (*stash).inliner_chain;
        if !func.is_null() && !((*func).caller_func).is_null() {
            *filename_ptr = (*func).caller_file;
            *functionname_ptr = (*(*func).caller_func).name;
            *linenumber_ptr = (*func).caller_line as libc::c_uint;
            (*stash).inliner_chain = (*func).caller_func;
            return 1 as libc::c_int != 0;
        }
    }
    return 0 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn _bfd_dwarf2_cleanup_debug_info(
    mut abfd: *mut bfd,
    mut pinfo: *mut *mut libc::c_void,
) {
    let mut stash: *mut dwarf2_debug = *pinfo as *mut dwarf2_debug;
    let mut each: *mut comp_unit = 0 as *mut comp_unit;
    let mut file: *mut dwarf2_debug_file = 0 as *mut dwarf2_debug_file;
    if abfd.is_null() || stash.is_null() {
        return;
    }
    if !((*stash).varinfo_hash_table).is_null() {
        bfd_hash_table_free(&mut (*(*stash).varinfo_hash_table).base);
    }
    if !((*stash).funcinfo_hash_table).is_null() {
        bfd_hash_table_free(&mut (*(*stash).funcinfo_hash_table).base);
    }
    file = &mut (*stash).f;
    loop {
        each = (*file).all_comp_units;
        while !each.is_null() {
            let mut function_table: *mut funcinfo = (*each).function_table;
            let mut variable_table: *mut varinfo = (*each).variable_table;
            if !((*each).line_table).is_null()
                && (*each).line_table != (*file).line_table
            {
                free((*(*each).line_table).files as *mut libc::c_void);
                free((*(*each).line_table).dirs as *mut libc::c_void);
            }
            free((*each).lookup_funcinfo_table as *mut libc::c_void);
            (*each).lookup_funcinfo_table = 0 as *mut lookup_funcinfo;
            while !function_table.is_null() {
                free((*function_table).file as *mut libc::c_void);
                (*function_table).file = 0 as *mut libc::c_char;
                free((*function_table).caller_file as *mut libc::c_void);
                (*function_table).caller_file = 0 as *mut libc::c_char;
                function_table = (*function_table).prev_func;
            }
            while !variable_table.is_null() {
                free((*variable_table).file as *mut libc::c_void);
                (*variable_table).file = 0 as *mut libc::c_char;
                variable_table = (*variable_table).prev_var;
            }
            each = (*each).next_unit;
        }
        if !((*file).line_table).is_null() {
            free((*(*file).line_table).files as *mut libc::c_void);
            free((*(*file).line_table).dirs as *mut libc::c_void);
        }
        htab_delete((*file).abbrev_offsets);
        free((*file).dwarf_line_str_buffer as *mut libc::c_void);
        free((*file).dwarf_str_buffer as *mut libc::c_void);
        free((*file).dwarf_ranges_buffer as *mut libc::c_void);
        free((*file).dwarf_line_buffer as *mut libc::c_void);
        free((*file).dwarf_abbrev_buffer as *mut libc::c_void);
        free((*file).dwarf_info_buffer as *mut libc::c_void);
        if file == &mut (*stash).alt as *mut dwarf2_debug_file {
            break;
        }
        file = &mut (*stash).alt;
    }
    free((*stash).sec_vma as *mut libc::c_void);
    free((*stash).adjusted_sections as *mut libc::c_void);
    if (*stash).close_on_cleanup {
        bfd_close((*stash).f.bfd_ptr);
    }
    if !((*stash).alt.bfd_ptr).is_null() {
        bfd_close((*stash).alt.bfd_ptr);
    }
}
#[no_mangle]
pub unsafe extern "C" fn _bfd_elf_find_function(
    mut abfd: *mut bfd,
    mut symbols: *mut *mut asymbol,
    mut section: *mut asection,
    mut offset: bfd_vma,
    mut filename_ptr: *mut *const libc::c_char,
    mut functionname_ptr: *mut *const libc::c_char,
) -> *mut asymbol {
    let mut cache: *mut elf_find_function_cache = 0 as *mut elf_find_function_cache;
    if symbols.is_null() {
        return 0 as *mut asymbol;
    }
    if bfd_get_flavour(abfd) as libc::c_uint
        != bfd_target_elf_flavour as libc::c_int as libc::c_uint
    {
        return 0 as *mut asymbol;
    }
    cache = (*(*abfd).tdata.elf_obj_data).elf_find_function_cache
        as *mut elf_find_function_cache;
    if cache.is_null() {
        cache = bfd_zalloc(
            abfd,
            ::core::mem::size_of::<elf_find_function_cache>() as libc::c_ulong,
        ) as *mut elf_find_function_cache;
        (*(*abfd).tdata.elf_obj_data)
            .elf_find_function_cache = cache as *mut libc::c_void;
        if cache.is_null() {
            return 0 as *mut asymbol;
        }
    }
    if (*cache).last_section != section || ((*cache).func).is_null()
        || offset < (*(*cache).func).value
        || offset >= ((*(*cache).func).value).wrapping_add((*cache).func_size)
    {
        let mut file: *mut asymbol = 0 as *mut asymbol;
        let mut low_func: bfd_vma = 0;
        let mut p: *mut *mut asymbol = 0 as *mut *mut asymbol;
        let mut state: C2RustUnnamed_21 = nothing_seen;
        let mut bed: *const elf_backend_data = (*(*abfd).xvec).backend_data
            as *const elf_backend_data;
        file = 0 as *mut asymbol;
        low_func = 0 as libc::c_int as bfd_vma;
        state = nothing_seen;
        (*cache).filename = 0 as *const libc::c_char;
        (*cache).func = 0 as *mut asymbol;
        (*cache).func_size = 0 as libc::c_int as bfd_size_type;
        (*cache).last_section = section;
        p = symbols;
        while !(*p).is_null() {
            let mut sym: *mut asymbol = *p;
            let mut code_off: bfd_vma = 0;
            let mut size: bfd_size_type = 0;
            if (*sym).flags & ((1 as libc::c_int) << 14 as libc::c_int) as libc::c_uint
                != 0 as libc::c_int as libc::c_uint
            {
                file = sym;
                if state as libc::c_uint == symbol_seen as libc::c_int as libc::c_uint {
                    state = file_after_symbol_seen;
                }
            } else {
                size = ((*bed).maybe_function_sym)
                    .expect("non-null function pointer")(sym, section, &mut code_off);
                if size != 0 as libc::c_int as libc::c_ulong && code_off <= offset
                    && (code_off > low_func
                        || code_off == low_func && size > (*cache).func_size)
                {
                    (*cache).func = sym;
                    (*cache).func_size = size;
                    (*cache).filename = 0 as *const libc::c_char;
                    low_func = code_off;
                    if !file.is_null()
                        && ((*sym).flags
                            & ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint
                            != 0 as libc::c_int as libc::c_uint
                            || state as libc::c_uint
                                != file_after_symbol_seen as libc::c_int as libc::c_uint)
                    {
                        (*cache).filename = bfd_asymbol_name(file);
                    }
                }
                if state as libc::c_uint == nothing_seen as libc::c_int as libc::c_uint {
                    state = symbol_seen;
                }
            }
            p = p.offset(1);
            p;
        }
    }
    if ((*cache).func).is_null() {
        return 0 as *mut asymbol;
    }
    if !filename_ptr.is_null() {
        *filename_ptr = (*cache).filename;
    }
    if !functionname_ptr.is_null() {
        *functionname_ptr = bfd_asymbol_name((*cache).func);
    }
    return (*cache).func;
}
unsafe extern "C" fn run_static_initializers() {
    previous_failed_abbrev = (1 as libc::c_uint).wrapping_neg();
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
