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
    pub type ctf_dict;
    pub type ctf_archive_internal;
    fn memcpy(
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
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    static mut optarg: *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strtoul(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_ulong;
    fn free(_: *mut libc::c_void);
    fn xrealloc(_: *mut libc::c_void, _: size_t) -> *mut libc::c_void;
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
    static mut config: ld_config_type;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    static mut link_info: bfd_link_info;
    fn einfo(_: *const libc::c_char, _: ...);
    static mut input_flags: lang_input_statement_flags;
    static mut lang_has_input_file: bool;
    fn add_excluded_libs(_: *const libc::c_char);
    fn ldfile_set_output_arch(_: *const libc::c_char, _: bfd_architecture);
    fn ldemul_default_target(
        _: libc::c_int,
        _: *mut *mut libc::c_char,
    ) -> *mut libc::c_char;
    fn after_check_relocs_default();
    fn finish_default();
    fn syslib_default(_: *mut libc::c_char);
    fn hll_default(_: *mut libc::c_char);
    fn bfd_elf_discard_info(_: *mut bfd, _: *mut bfd_link_info) -> libc::c_int;
    static mut ldelf_emit_note_gnu_build_id: *const libc::c_char;
    fn ldelf_after_parse();
    fn ldelf_load_symbols(_: *mut lang_input_statement_type) -> bool;
    fn ldelf_after_open(
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: *const libc::c_char,
    );
    fn ldelf_append_to_separated_string(_: *mut *mut libc::c_char, _: *mut libc::c_char);
    fn ldelf_before_allocation(
        _: *mut libc::c_char,
        _: *mut libc::c_char,
        _: *const libc::c_char,
    );
    fn ldelf_open_dynamic_archive(
        _: *const libc::c_char,
        _: *mut search_dirs_type,
        _: *mut lang_input_statement_type,
    ) -> bool;
    fn ldelf_place_orphan(
        _: *mut asection,
        _: *const libc::c_char,
        _: libc::c_int,
    ) -> *mut lang_output_section_statement_type;
    fn ldelf_before_place_orphans();
    fn ldelf_set_output_arch();
    fn ldelf_map_segments(_: bool);
    fn ldelf_emit_ctf_early() -> libc::c_int;
    fn ldelf_acquire_strings_for_ctf(
        ctf_output: *mut ctf_dict,
        strtab: *mut elf_strtab_hash,
    );
    fn ldelf_new_dynsym_for_ctf(
        ctf_output: *mut ctf_dict,
        symidx: libc::c_int,
        sym: *mut elf_internal_sym,
    );
    fn _bfd_elf_linker_x86_set_options(
        _: *mut bfd_link_info,
        _: *mut elf_linker_x86_params,
    );
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
pub type ctf_archive_t = ctf_archive_internal;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct name_list {
    pub name: *const libc::c_char,
    pub next: *mut name_list,
}
pub type sort_order = libc::c_uint;
pub const sort_descending: sort_order = 2;
pub const sort_ascending: sort_order = 1;
pub const sort_none: sort_order = 0;
pub type sort_type = libc::c_uint;
pub const by_init_priority: sort_type = 6;
pub const by_none: sort_type = 5;
pub const by_alignment_name: sort_type = 4;
pub const by_name_alignment: sort_type = 3;
pub const by_alignment: sort_type = 2;
pub const by_name: sort_type = 1;
pub const none: sort_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wildcard_spec {
    pub name: *const libc::c_char,
    pub exclude_name_list: *mut name_list,
    pub sorted: sort_type,
    pub section_flag_list: *mut flag_info,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wildcard_list {
    pub next: *mut wildcard_list,
    pub spec: wildcard_spec,
}
pub type orphan_handling_enum = libc::c_uint;
pub const orphan_handling_error: orphan_handling_enum = 3;
pub const orphan_handling_warn: orphan_handling_enum = 2;
pub const orphan_handling_discard: orphan_handling_enum = 1;
pub const orphan_handling_place: orphan_handling_enum = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ld_config_type {
    pub magic_demand_paged: bool,
    pub make_executable: bool,
    pub has_shared: bool,
    pub build_constructors: bool,
    pub warn_constructors: bool,
    pub warn_common: bool,
    pub warn_once: bool,
    pub orphan_handling: orphan_handling_enum,
    pub warn_multiple_gp: bool,
    pub warn_section_align: bool,
    pub fatal_warnings: bool,
    pub sort_common: sort_order,
    pub text_read_only: bool,
    pub stats: bool,
    pub unique_orphan_sections: bool,
    pub only_cmd_line_lib_dirs: bool,
    pub sane_expr: bool,
    pub separate_code: bool,
    pub rpath_separator: libc::c_char,
    pub map_filename: *mut libc::c_char,
    pub map_file: *mut FILE,
    pub dependency_file: *mut libc::c_char,
    pub split_by_reloc: libc::c_uint,
    pub split_by_file: bfd_size_type,
    pub hash_table_size: libc::c_ulong,
    pub print_map_discarded: bool,
    pub ctf_variables: bool,
    pub ctf_share_duplicated: bool,
}
pub type node_tree_enum = libc::c_uint;
pub const etree_rel: node_tree_enum = 9;
pub const etree_assert: node_tree_enum = 8;
pub const etree_value: node_tree_enum = 7;
pub const etree_provided: node_tree_enum = 6;
pub const etree_provide: node_tree_enum = 5;
pub const etree_assign: node_tree_enum = 4;
pub const etree_name: node_tree_enum = 3;
pub const etree_unary: node_tree_enum = 2;
pub const etree_trinary: node_tree_enum = 1;
pub const etree_binary: node_tree_enum = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct node_type {
    pub node_code: libc::c_int,
    pub lineno: libc::c_uint,
    pub filename: *const libc::c_char,
    pub node_class: node_tree_enum,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union etree_union {
    pub type_0: node_type,
    pub binary: C2RustUnnamed_28,
    pub trinary: C2RustUnnamed_27,
    pub assign: C2RustUnnamed_26,
    pub unary: C2RustUnnamed_25,
    pub name: C2RustUnnamed_24,
    pub value: C2RustUnnamed_23,
    pub rel: C2RustUnnamed_22,
    pub assert_s: C2RustUnnamed_21,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_21 {
    pub type_0: node_type,
    pub child: *mut etree_union,
    pub message: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_22 {
    pub type_0: node_type,
    pub section: *mut asection,
    pub value: bfd_vma,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_23 {
    pub type_0: node_type,
    pub value: bfd_vma,
    pub str_0: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_24 {
    pub type_0: node_type,
    pub name: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_25 {
    pub type_0: node_type,
    pub child: *mut etree_union,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_26 {
    pub type_0: node_type,
    pub dst: *const libc::c_char,
    pub src: *mut etree_union,
    pub hidden: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_27 {
    pub type_0: node_type,
    pub cond: *mut etree_union,
    pub lhs: *mut etree_union,
    pub rhs: *mut etree_union,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_28 {
    pub type_0: node_type,
    pub lhs: *mut etree_union,
    pub rhs: *mut etree_union,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union lang_statement_union {
    pub header: lang_statement_header_type,
    pub address_statement: lang_address_statement_type,
    pub assignment_statement: lang_assignment_statement_type,
    pub data_statement: lang_data_statement_type,
    pub fill_statement: lang_fill_statement_type,
    pub group_statement: lang_group_statement_type,
    pub input_section: lang_input_section_type,
    pub input_statement: lang_input_statement_type,
    pub insert_statement: lang_insert_statement_type,
    pub output_section_statement: lang_output_section_statement_type,
    pub output_statement: lang_output_statement_type,
    pub padding_statement: lang_padding_statement_type,
    pub reloc_statement: lang_reloc_statement_type,
    pub target_statement: lang_target_statement_type,
    pub wild_statement: lang_wild_statement_type,
}
pub type lang_wild_statement_type = lang_wild_statement_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lang_wild_statement_struct {
    pub header: lang_statement_header_type,
    pub filename: *const libc::c_char,
    pub filenames_sorted: bool,
    pub section_list: *mut wildcard_list,
    pub keep_sections: bool,
    pub children: lang_statement_list_type,
    pub exclude_name_list: *mut name_list,
    pub walk_wild_section_handler: walk_wild_section_handler_t,
    pub handler_data: [*mut wildcard_list; 4],
    pub tree: *mut lang_section_bst_type,
    pub section_flag_list: *mut flag_info,
}
pub type lang_section_bst_type = lang_section_bst;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lang_section_bst {
    pub section: *mut asection,
    pub pattern: *mut libc::c_void,
    pub left: *mut lang_section_bst,
    pub right: *mut lang_section_bst,
}
pub type walk_wild_section_handler_t = Option::<
    unsafe extern "C" fn(
        *mut lang_wild_statement_type,
        *mut lang_input_statement_type,
        callback_t,
        *mut libc::c_void,
    ) -> (),
>;
pub type callback_t = Option::<
    unsafe extern "C" fn(
        *mut lang_wild_statement_type,
        *mut wildcard_list,
        *mut asection,
        *mut lang_input_statement_type,
        *mut libc::c_void,
    ) -> (),
>;
pub type lang_input_statement_type = lang_input_statement_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lang_input_statement_struct {
    pub header: lang_statement_header_type,
    pub filename: *const libc::c_char,
    pub local_sym_name: *const libc::c_char,
    pub extra_search_path: *const libc::c_char,
    pub the_bfd: *mut bfd,
    pub the_ctf: *mut ctf_archive_t,
    pub section_flag_list: *mut flag_info,
    pub next: *mut lang_input_statement_struct,
    pub next_real_file: *mut lang_input_statement_struct,
    pub target: *const libc::c_char,
    pub flags: lang_input_statement_flags,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct lang_input_statement_flags {
    #[bitfield(name = "maybe_archive", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "full_name_provided", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "search_dirs", ty = "libc::c_uint", bits = "2..=2")]
    #[bitfield(name = "sysrooted", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "just_syms", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "dynamic", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "add_DT_NEEDED_for_dynamic", ty = "libc::c_uint", bits = "6..=6")]
    #[bitfield(name = "add_DT_NEEDED_for_regular", ty = "libc::c_uint", bits = "7..=7")]
    #[bitfield(name = "whole_archive", ty = "libc::c_uint", bits = "8..=8")]
    #[bitfield(name = "loaded", ty = "libc::c_uint", bits = "9..=9")]
    #[bitfield(name = "real", ty = "libc::c_uint", bits = "10..=10")]
    #[bitfield(name = "missing_file", ty = "libc::c_uint", bits = "11..=11")]
    #[bitfield(name = "reload", ty = "libc::c_uint", bits = "12..=12")]
    #[bitfield(name = "claimed", ty = "libc::c_uint", bits = "13..=13")]
    #[bitfield(name = "claim_archive", ty = "libc::c_uint", bits = "14..=14")]
    #[bitfield(name = "lto_output", ty = "libc::c_uint", bits = "15..=15")]
    pub maybe_archive_full_name_provided_search_dirs_sysrooted_just_syms_dynamic_add_DT_NEEDED_for_dynamic_add_DT_NEEDED_for_regular_whole_archive_loaded_real_missing_file_reload_claimed_claim_archive_lto_output: [u8; 2],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 6],
    pub pushed: *mut lang_input_statement_flags,
}
pub type lang_statement_header_type = lang_statement_header_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lang_statement_header_struct {
    pub next: *mut lang_statement_union,
    pub type_0: statement_enum,
}
pub type statement_enum = libc::c_uint;
pub const lang_object_symbols_statement_enum: statement_enum = 15;
pub const lang_constructors_statement_enum: statement_enum = 14;
pub const lang_wild_statement_enum: statement_enum = 13;
pub const lang_target_statement_enum: statement_enum = 12;
pub const lang_reloc_statement_enum: statement_enum = 11;
pub const lang_padding_statement_enum: statement_enum = 10;
pub const lang_output_statement_enum: statement_enum = 9;
pub const lang_output_section_statement_enum: statement_enum = 8;
pub const lang_insert_statement_enum: statement_enum = 7;
pub const lang_input_statement_enum: statement_enum = 6;
pub const lang_input_section_enum: statement_enum = 5;
pub const lang_group_statement_enum: statement_enum = 4;
pub const lang_fill_statement_enum: statement_enum = 3;
pub const lang_data_statement_enum: statement_enum = 2;
pub const lang_assignment_statement_enum: statement_enum = 1;
pub const lang_address_statement_enum: statement_enum = 0;
pub type lang_statement_list_type = statement_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct statement_list {
    pub head: *mut lang_statement_union,
    pub tail: *mut *mut lang_statement_union,
}
pub type lang_target_statement_type = lang_target_statement_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lang_target_statement_struct {
    pub header: lang_statement_header_type,
    pub target: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lang_reloc_statement_type {
    pub header: lang_statement_header_type,
    pub reloc: bfd_reloc_code_real_type,
    pub howto: *const reloc_howto_type,
    pub section: *mut asection,
    pub name: *const libc::c_char,
    pub addend_exp: *mut etree_union,
    pub addend_value: bfd_vma,
    pub output_section: *mut asection,
    pub output_offset: bfd_vma,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lang_padding_statement_type {
    pub header: lang_statement_header_type,
    pub output_offset: bfd_vma,
    pub size: bfd_size_type,
    pub output_section: *mut asection,
    pub fill: *mut fill_type,
}
pub type fill_type = _fill_type;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _fill_type {
    pub size: size_t,
    pub data: [libc::c_uchar; 1],
}
pub type lang_output_statement_type = lang_output_statement_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lang_output_statement_struct {
    pub header: lang_statement_header_type,
    pub name: *const libc::c_char,
}
pub type lang_output_section_statement_type = lang_output_section_statement_struct;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct lang_output_section_statement_struct {
    pub header: lang_statement_header_type,
    pub children: lang_statement_list_type,
    pub next: *mut lang_output_section_statement_struct,
    pub prev: *mut lang_output_section_statement_struct,
    pub name: *const libc::c_char,
    pub bfd_section: *mut asection,
    pub region: *mut lang_memory_region_type,
    pub lma_region: *mut lang_memory_region_type,
    pub fill: *mut fill_type,
    pub addr_tree: *mut etree_union,
    pub load_base: *mut etree_union,
    pub section_alignment: *mut etree_union,
    pub subsection_alignment: *mut etree_union,
    pub update_dot_tree: *mut etree_union,
    pub phdrs: *mut lang_output_section_phdr_list,
    pub data: *mut libc::c_void,
    pub block_value: libc::c_uint,
    pub constraint: libc::c_int,
    pub flags: flagword,
    pub sectype: section_type,
    #[bitfield(name = "processed_vma", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "processed_lma", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "all_input_readonly", ty = "libc::c_uint", bits = "2..=2")]
    #[bitfield(name = "ignored", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "update_dot", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "after_end", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "align_lma_with_input", ty = "libc::c_uint", bits = "6..=6")]
    #[bitfield(name = "dup_output", ty = "libc::c_uint", bits = "7..=7")]
    pub processed_vma_processed_lma_all_input_readonly_ignored_update_dot_after_end_align_lma_with_input_dup_output: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
pub type section_type = libc::c_uint;
pub const noalloc_section: section_type = 4;
pub const noload_section: section_type = 3;
pub const overlay_section: section_type = 2;
pub const first_overlay_section: section_type = 1;
pub const normal_section: section_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lang_output_section_phdr_list {
    pub next: *mut lang_output_section_phdr_list,
    pub name: *const libc::c_char,
    pub used: bool,
}
pub type lang_memory_region_type = memory_region_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct memory_region_struct {
    pub name_list: lang_memory_region_name,
    pub next: *mut memory_region_struct,
    pub origin_exp: *mut etree_union,
    pub origin: bfd_vma,
    pub length: bfd_size_type,
    pub length_exp: *mut etree_union,
    pub current: bfd_vma,
    pub last_os: *mut lang_statement_union,
    pub flags: flagword,
    pub not_flags: flagword,
    pub had_full_message: bool,
}
pub type lang_memory_region_name = memory_region_name_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct memory_region_name_struct {
    pub name: *const libc::c_char,
    pub next: *mut memory_region_name_struct,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lang_insert_statement_type {
    pub header: lang_statement_header_type,
    pub where_0: *const libc::c_char,
    pub is_before: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lang_input_section_type {
    pub header: lang_statement_header_type,
    pub section: *mut asection,
    pub pattern: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lang_group_statement_type {
    pub header: lang_statement_header_type,
    pub children: lang_statement_list_type,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lang_fill_statement_type {
    pub header: lang_statement_header_type,
    pub fill: *mut fill_type,
    pub size: libc::c_int,
    pub output_section: *mut asection,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lang_data_statement_type {
    pub header: lang_statement_header_type,
    pub type_0: libc::c_uint,
    pub exp: *mut etree_union,
    pub value: bfd_vma,
    pub output_section: *mut asection,
    pub output_offset: bfd_vma,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lang_assignment_statement_type {
    pub header: lang_statement_header_type,
    pub exp: *mut etree_union,
}
pub type lang_address_statement_type = lang_address_statement_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lang_address_statement_struct {
    pub header: lang_statement_header_type,
    pub section_name: *const libc::c_char,
    pub address: *mut etree_union,
    pub segment: *const segment_type,
}
pub type segment_type = segment_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct segment_struct {
    pub next: *mut segment_struct,
    pub name: *const libc::c_char,
    pub value: bfd_vma,
    pub used: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct search_dirs {
    pub next: *mut search_dirs,
    pub name: *const libc::c_char,
    pub cmdline: bool,
}
pub type search_dirs_type = search_dirs;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ld_emulation_xfer_struct {
    pub before_parse: Option::<unsafe extern "C" fn() -> ()>,
    pub syslib: Option::<unsafe extern "C" fn(*mut libc::c_char) -> ()>,
    pub hll: Option::<unsafe extern "C" fn(*mut libc::c_char) -> ()>,
    pub after_parse: Option::<unsafe extern "C" fn() -> ()>,
    pub after_open: Option::<unsafe extern "C" fn() -> ()>,
    pub after_check_relocs: Option::<unsafe extern "C" fn() -> ()>,
    pub before_place_orphans: Option::<unsafe extern "C" fn() -> ()>,
    pub after_allocation: Option::<unsafe extern "C" fn() -> ()>,
    pub set_output_arch: Option::<unsafe extern "C" fn() -> ()>,
    pub choose_target: Option::<
        unsafe extern "C" fn(libc::c_int, *mut *mut libc::c_char) -> *mut libc::c_char,
    >,
    pub before_allocation: Option::<unsafe extern "C" fn() -> ()>,
    pub get_script: Option::<
        unsafe extern "C" fn(*mut libc::c_int) -> *mut libc::c_char,
    >,
    pub emulation_name: *mut libc::c_char,
    pub target_name: *mut libc::c_char,
    pub finish: Option::<unsafe extern "C" fn() -> ()>,
    pub create_output_section_statements: Option::<unsafe extern "C" fn() -> ()>,
    pub open_dynamic_archive: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            *mut search_dirs,
            *mut lang_input_statement_struct,
        ) -> bool,
    >,
    pub place_orphan: Option::<
        unsafe extern "C" fn(
            *mut asection,
            *const libc::c_char,
            libc::c_int,
        ) -> *mut lang_output_section_statement_type,
    >,
    pub set_symbols: Option::<unsafe extern "C" fn() -> ()>,
    pub parse_args: Option::<
        unsafe extern "C" fn(libc::c_int, *mut *mut libc::c_char) -> bool,
    >,
    pub add_options: Option::<
        unsafe extern "C" fn(
            libc::c_int,
            *mut *mut libc::c_char,
            libc::c_int,
            *mut *mut option,
            libc::c_int,
            *mut *mut option,
        ) -> (),
    >,
    pub handle_option: Option::<unsafe extern "C" fn(libc::c_int) -> bool>,
    pub unrecognized_file: Option::<
        unsafe extern "C" fn(*mut lang_input_statement_struct) -> bool,
    >,
    pub list_options: Option::<unsafe extern "C" fn(*mut FILE) -> ()>,
    pub recognized_file: Option::<
        unsafe extern "C" fn(*mut lang_input_statement_struct) -> bool,
    >,
    pub find_potential_libraries: Option::<
        unsafe extern "C" fn(
            *mut libc::c_char,
            *mut lang_input_statement_struct,
        ) -> libc::c_int,
    >,
    pub new_vers_pattern: Option::<
        unsafe extern "C" fn(*mut bfd_elf_version_expr) -> *mut bfd_elf_version_expr,
    >,
    pub extra_map_file_text: Option::<
        unsafe extern "C" fn(*mut bfd, *mut bfd_link_info, *mut FILE) -> (),
    >,
    pub emit_ctf_early: Option::<unsafe extern "C" fn() -> libc::c_int>,
    pub acquire_strings_for_ctf: Option::<
        unsafe extern "C" fn(*mut ctf_dict, *mut elf_strtab_hash) -> (),
    >,
    pub new_dynsym_for_ctf: Option::<
        unsafe extern "C" fn(*mut ctf_dict, libc::c_int, *mut elf_internal_sym) -> (),
    >,
    pub print_symbol: Option::<
        unsafe extern "C" fn(*mut bfd_link_hash_entry, *mut libc::c_void) -> bool,
    >,
}
pub type option_values = libc::c_uint;
pub const OPTION_CTF_SHARE_TYPES: option_values = 281;
pub const OPTION_NO_CTF_VARIABLES: option_values = 280;
pub const OPTION_CTF_VARIABLES: option_values = 279;
pub const OPTION_DEPENDENCY_FILE: option_values = 278;
pub const OPTION_NON_CONTIGUOUS_REGIONS_WARNINGS: option_values = 277;
pub const OPTION_NON_CONTIGUOUS_REGIONS: option_values = 276;
pub const OPTION_NO_PRINT_MAP_DISCARDED: option_values = 275;
pub const OPTION_PRINT_MAP_DISCARDED: option_values = 274;
pub const OPTION_FORCE_GROUP_ALLOCATION: option_values = 273;
pub const OPTION_ORPHAN_HANDLING: option_values = 272;
pub const OPTION_REQUIRE_DEFINED_SYMBOL: option_values = 271;
pub const OPTION_PRINT_MEMORY_USAGE: option_values = 270;
pub const OPTION_DISABLE_MULTIPLE_DEFS_ABS: option_values = 269;
pub const OPTION_POP_STATE: option_values = 268;
pub const OPTION_PUSH_STATE: option_values = 267;
pub const OPTION_IGNORE_UNRESOLVED_SYMBOL: option_values = 266;
pub const OPTION_PRINT_SYSROOT: option_values = 265;
pub const OPTION_PRINT_OUTPUT_FORMAT: option_values = 264;
pub const OPTION_DEFAULT_SCRIPT: option_values = 263;
pub const OPTION_PLUGIN_OPT: option_values = 262;
pub const OPTION_PLUGIN: option_values = 261;
pub const OPTION_REDUCE_MEMORY_OVERHEADS: option_values = 260;
pub const OPTION_WARN_ALTERNATE_EM: option_values = 259;
pub const OPTION_WARN_TEXTREL: option_values = 258;
pub const OPTION_ERROR_UNRESOLVED_SYMBOLS: option_values = 257;
pub const OPTION_WARN_UNRESOLVED_SYMBOLS: option_values = 256;
pub const OPTION_UNRESOLVED_SYMBOLS: option_values = 255;
pub const OPTION_NO_PIE: option_values = 254;
pub const OPTION_PIE: option_values = 253;
pub const OPTION_NO_ACCEPT_UNKNOWN_INPUT_ARCH: option_values = 252;
pub const OPTION_ACCEPT_UNKNOWN_INPUT_ARCH: option_values = 251;
pub const OPTION_NO_STRIP_DISCARDED: option_values = 250;
pub const OPTION_STRIP_DISCARDED: option_values = 249;
pub const OPTION_NO_OMAGIC: option_values = 248;
pub const OPTION_NOSTDLIB: option_values = 247;
pub const OPTION_NO_DEFINE_COMMON: option_values = 246;
pub const OPTION_SPARE_DYNAMIC_TAGS: option_values = 245;
pub const OPTION_DISCARD_NONE: option_values = 244;
pub const OPTION_DEFAULT_IMPORTED_SYMVER: option_values = 243;
pub const OPTION_DEFAULT_SYMVER: option_values = 242;
pub const OPTION_NO_UNDEFINED_VERSION: option_values = 241;
pub const OPTION_ERROR_HANDLING_SCRIPT: option_values = 240;
pub const OPTION_ALLOW_MULTIPLE_DEFINITION: option_values = 239;
pub const OPTION_NO_ALLOW_SHLIB_UNDEFINED: option_values = 238;
pub const OPTION_ALLOW_SHLIB_UNDEFINED: option_values = 237;
pub const OPTION_TARGET_HELP: option_values = 236;
pub const OPTION_UNIQUE: option_values = 235;
pub const OPTION_SECTION_START: option_values = 234;
pub const OPTION_FINI: option_values = 233;
pub const OPTION_INIT: option_values = 232;
pub const OPTION_NO_UNDEFINED: option_values = 231;
pub const OPTION_NO_CHECK_SECTIONS: option_values = 230;
pub const OPTION_CHECK_SECTIONS: option_values = 229;
pub const OPTION_HASH_SIZE: option_values = 228;
pub const OPTION_GC_KEEP_EXPORTED: option_values = 227;
pub const OPTION_NO_PRINT_GC_SECTIONS: option_values = 226;
pub const OPTION_PRINT_GC_SECTIONS: option_values = 225;
pub const OPTION_NO_GC_SECTIONS: option_values = 224;
pub const OPTION_GC_SECTIONS: option_values = 223;
pub const OPTION_FORCE_EXE_SUFFIX: option_values = 222;
pub const OPTION_WRAP: option_values = 221;
pub const OPTION_NO_ADD_DT_NEEDED_FOR_REGULAR: option_values = 220;
pub const OPTION_ADD_DT_NEEDED_FOR_REGULAR: option_values = 219;
pub const OPTION_NO_ADD_DT_NEEDED_FOR_DYNAMIC: option_values = 218;
pub const OPTION_ADD_DT_NEEDED_FOR_DYNAMIC: option_values = 217;
pub const OPTION_WHOLE_ARCHIVE: option_values = 216;
pub const OPTION_SPLIT_BY_FILE: option_values = 215;
pub const OPTION_SPLIT_BY_RELOC: option_values = 214;
pub const OPTION_WARN_SECTION_ALIGN: option_values = 213;
pub const OPTION_WARN_ONCE: option_values = 212;
pub const OPTION_WARN_MULTIPLE_GP: option_values = 211;
pub const OPTION_NO_WARN_FATAL: option_values = 210;
pub const OPTION_WARN_FATAL: option_values = 209;
pub const OPTION_WARN_CONSTRUCTORS: option_values = 208;
pub const OPTION_WARN_COMMON: option_values = 207;
pub const OPTION_EXPORT_DYNAMIC_SYMBOL_LIST: option_values = 206;
pub const OPTION_EXPORT_DYNAMIC_SYMBOL: option_values = 205;
pub const OPTION_DYNAMIC_LIST_DATA: option_values = 204;
pub const OPTION_DYNAMIC_LIST_CPP_TYPEINFO: option_values = 203;
pub const OPTION_DYNAMIC_LIST_CPP_NEW: option_values = 202;
pub const OPTION_DYNAMIC_LIST: option_values = 201;
pub const OPTION_VERSION_EXPORTS_SECTION: option_values = 200;
pub const OPTION_VERSION_SCRIPT: option_values = 199;
pub const OPTION_VERSION: option_values = 198;
pub const OPTION_VERBOSE: option_values = 197;
pub const OPTION_UR: option_values = 196;
pub const OPTION_TRADITIONAL_FORMAT: option_values = 195;
pub const OPTION_TLDATA_SEGMENT: option_values = 194;
pub const OPTION_TRODATA_SEGMENT: option_values = 193;
pub const OPTION_TTEXT_SEGMENT: option_values = 192;
pub const OPTION_TTEXT: option_values = 191;
pub const OPTION_TDATA: option_values = 190;
pub const OPTION_TBSS: option_values = 189;
pub const OPTION_TASK_LINK: option_values = 188;
pub const OPTION_SYMBOLIC_FUNCTIONS: option_values = 187;
pub const OPTION_SYMBOLIC: option_values = 186;
pub const OPTION_STATS: option_values = 185;
pub const OPTION_SORT_SECTION: option_values = 184;
pub const OPTION_SORT_COMMON: option_values = 183;
pub const OPTION_SONAME: option_values = 182;
pub const OPTION_SHARED: option_values = 181;
pub const OPTION_RPATH_LINK: option_values = 180;
pub const OPTION_RPATH: option_values = 179;
pub const OPTION_RETAIN_SYMBOLS_FILE: option_values = 178;
pub const OPTION_NO_SYMBOLIC: option_values = 177;
pub const OPTION_NO_RELAX: option_values = 176;
pub const OPTION_RELAX: option_values = 175;
pub const OPTION_OFORMAT: option_values = 174;
pub const OPTION_NO_WHOLE_ARCHIVE: option_values = 173;
pub const OPTION_NON_SHARED: option_values = 172;
pub const OPTION_NOINHIBIT_EXEC: option_values = 171;
pub const OPTION_NO_WARN_SEARCH_MISMATCH: option_values = 170;
pub const OPTION_NO_WARN_MISMATCH: option_values = 169;
pub const OPTION_NO_KEEP_MEMORY: option_values = 168;
pub const OPTION_NO_DEMANGLE: option_values = 167;
pub const OPTION_MAP: option_values = 166;
pub const OPTION_IGNORE: option_values = 165;
pub const OPTION_HELP: option_values = 164;
pub const OPTION_NO_EXPORT_DYNAMIC: option_values = 163;
pub const OPTION_EXPORT_DYNAMIC: option_values = 162;
pub const OPTION_EMBEDDED_RELOCS: option_values = 161;
pub const OPTION_EL: option_values = 160;
pub const OPTION_EB: option_values = 159;
pub const OPTION_OUT_IMPLIB: option_values = 158;
pub const OPTION_SYSROOT: option_values = 157;
pub const OPTION_NO_DYNAMIC_LINKER: option_values = 156;
pub const OPTION_DYNAMIC_LINKER: option_values = 155;
pub const OPTION_DEMANGLE: option_values = 154;
pub const OPTION_DEFSYM: option_values = 153;
pub const OPTION_CREF: option_values = 152;
pub const OPTION_CALL_SHARED: option_values = 151;
pub const OPTION_ASSERT: option_values = 150;
pub type elf_x86_prop_report = libc::c_uint;
pub const prop_report_shstk: elf_x86_prop_report = 8;
pub const prop_report_ibt: elf_x86_prop_report = 4;
pub const prop_report_error: elf_x86_prop_report = 2;
pub const prop_report_warning: elf_x86_prop_report = 1;
pub const prop_report_none: elf_x86_prop_report = 0;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct elf_linker_x86_params {
    #[bitfield(name = "bndplt", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "ibtplt", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "ibt", ty = "libc::c_uint", bits = "2..=2")]
    #[bitfield(name = "shstk", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "lam_u48", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "lam_u57", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "no_reloc_overflow_check", ty = "libc::c_uint", bits = "6..=6")]
    #[bitfield(name = "call_nop_as_suffix", ty = "libc::c_uint", bits = "7..=7")]
    #[bitfield(name = "static_before_all_inputs", ty = "libc::c_uint", bits = "8..=8")]
    #[bitfield(name = "has_dynamic_linker", ty = "libc::c_uint", bits = "9..=9")]
    #[bitfield(name = "report_relative_reloc", ty = "libc::c_uint", bits = "10..=10")]
    pub bndplt_ibtplt_ibt_shstk_lam_u48_lam_u57_no_reloc_overflow_check_call_nop_as_suffix_static_before_all_inputs_has_dynamic_linker_report_relative_reloc: [u8; 2],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 2],
    pub isa_level: libc::c_uint,
    pub cet_report: elf_x86_prop_report,
    pub lam_u48_report: elf_x86_prop_report,
    pub lam_u57_report: elf_x86_prop_report,
    pub call_nop_byte: libc::c_char,
}
pub type elf_options = libc::c_uint;
pub const OPTION_COMPRESS_DEBUG: elf_options = 409;
pub const OPTION_AUDIT: elf_options = 408;
pub const OPTION_BUILD_ID: elf_options = 407;
pub const OPTION_HASH_STYLE: elf_options = 406;
pub const OPTION_EXCLUDE_LIBS: elf_options = 405;
pub const OPTION_NO_EH_FRAME_HDR: elf_options = 404;
pub const OPTION_EH_FRAME_HDR: elf_options = 403;
pub const OPTION_GROUP: elf_options = 402;
pub const OPTION_ENABLE_NEW_DTAGS: elf_options = 401;
pub const OPTION_DISABLE_NEW_DTAGS: elf_options = 400;
#[inline]
unsafe extern "C" fn startswith(
    mut str: *const libc::c_char,
    mut prefix: *const libc::c_char,
) -> bool {
    return strncmp(str, prefix, strlen(prefix)) == 0 as libc::c_int;
}
unsafe extern "C" fn gldelf_x86_64_before_parse() {
    ldfile_set_output_arch(
        b"i386:x86-64\0" as *const u8 as *const libc::c_char,
        bfd_arch_i386,
    );
    input_flags.set_dynamic(1 as libc::c_int as libc::c_uint);
    config.has_shared = 1 as libc::c_int != 0;
    config.separate_code = 0 as libc::c_int != 0;
    link_info.set_check_relocs_after_open_input(1 as libc::c_int as libc::c_uint);
    link_info.set_relro(1 as libc::c_int as libc::c_uint);
    link_info.set_separate_code(1 as libc::c_int as libc::c_uint);
}
unsafe extern "C" fn gldelf_x86_64_after_open() {
    ldelf_after_open(
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
        64 as libc::c_int,
        b"/usr/local\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn gldelf_x86_64_before_allocation() {
    ldelf_before_allocation(audit, depaudit, 0 as *const libc::c_char);
}
unsafe extern "C" fn gldelf_x86_64_after_allocation() {
    let mut need_layout: libc::c_int = bfd_elf_discard_info(
        link_info.output_bfd,
        &mut link_info,
    );
    if need_layout < 0 as libc::c_int {
        einfo(
            dcgettext(
                0 as *const libc::c_char,
                b"%X%P: .eh_frame/.stab edit: %E\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    } else {
        ldelf_map_segments(need_layout != 0);
    };
}
static mut params: elf_linker_x86_params = elf_linker_x86_params {
    bndplt_ibtplt_ibt_shstk_lam_u48_lam_u57_no_reloc_overflow_check_call_nop_as_suffix_static_before_all_inputs_has_dynamic_linker_report_relative_reloc: [0; 2],
    c2rust_padding: [0; 2],
    isa_level: 0,
    cet_report: prop_report_none,
    lam_u48_report: prop_report_none,
    lam_u57_report: prop_report_none,
    call_nop_byte: 0,
};
unsafe extern "C" fn elf_x86_create_output_section_statements() {
    _bfd_elf_linker_x86_set_options(&mut link_info, &mut params);
}
unsafe extern "C" fn elf_x86_before_parse() {
    params.call_nop_byte = 0x67 as libc::c_int as libc::c_char;
    gldelf_x86_64_before_parse();
}
static mut audit: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut depaudit: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
unsafe extern "C" fn gldelf_x86_64_get_script(
    mut isfile: *mut libc::c_int,
) -> *mut libc::c_char {
    *isfile = 0 as libc::c_int;
    if link_info.type_0() as libc::c_int == type_relocatable as libc::c_int
        && config.build_constructors as libc::c_int != 0
    {
        return b"/* Script for -Ur */\n/* Copyright (C) 2014-2021 Free Software Foundation, Inc.\n   Copying and distribution of this script, with or without modification,\n   are permitted in any medium without royalty provided the copyright\n   notice and this notice are preserved.  */\nOUTPUT_FORMAT(\"elf64-x86-64\", \"elf64-x86-64\",\n\t      \"elf64-x86-64\")\nOUTPUT_ARCH(i386:x86-64)\n /* For some reason, the Solaris linker makes bad executables\n  if gld -r is used and the intermediate file has sections starting\n  at non-zero addresses.  Could be a Solaris ld bug, could be a GNU ld\n  bug.  But for now assigning the zero vmas works.  */\nSECTIONS\n{\n  .interp       0 : { *(.interp) }\n  .note.gnu.build-id 0: { *(.note.gnu.build-id) }\n  .hash         0 : { *(.hash) }\n  .gnu.hash     0 : { *(.gnu.hash) }\n  .dynsym       0 : { *(.dynsym) }\n  .dynstr       0 : { *(.dynstr) }\n  .gnu.version  0 : { *(.gnu.version) }\n  .gnu.version_d 0: { *(.gnu.version_d) }\n  .gnu.version_r 0: { *(.gnu.version_r) }\n  .rela.init    0 : { *(.rela.init) }\n  .rela.text    0 : { *(.rela.text) }\n  .rela.fini    0 : { *(.rela.fini) }\n  .rela.rodata  0 : { *(.rela.rodata) }\n  .rela.data.rel.ro 0 : { *(.rela.data.rel.ro) }\n  .rela.data    0 : { *(.rela.data) }\n  .rela.tdata\t0 : { *(.rela.tdata) }\n  .rela.tbss\t0 : { *(.rela.tbss) }\n  .rela.ctors   0 : { *(.rela.ctors) }\n  .rela.dtors   0 : { *(.rela.dtors) }\n  .rela.got     0 : { *(.rela.got) }\n  .rela.bss     0 : { *(.rela.bss) }\n  .rela.ldata   0 : { *(.rela.ldata) }\n  .rela.lbss    0 : { *(.rela.lbss) }\n  .rela.lrodata 0 : { *(.rela.lrodata) }\n  .rela.ifunc   0 : { *(.rela.ifunc) }\n  .rela.plt     0 :\n    {\n      *(.rela.plt)\n    }\n  .init         0 :\n  {\n    KEEP (*(SORT_NONE(.init)))\n  }\n  .plt          0 : { *(.plt) }\n.plt.got      0 : { *(.plt.got) }\n.plt.sec      0 : { *(.plt.sec) }\n  .text         0 :\n  {\n    *(.text .stub)\n    /* .gnu.warning sections are handled specially by elf.em.  */\n    *(.gnu.warning)\n  }\n  .fini         0 :\n  {\n    KEEP (*(SORT_NONE(.fini)))\n  }\n  .rodata       0 : { *(.rodata) }\n  .rodata1      0 : { *(.rodata1) }\n  .eh_frame_hdr 0 : { *(.eh_frame_hdr) }\n  .eh_frame     0 : ONLY_IF_RO { KEEP (*(.eh_frame)) }\n  .gcc_except_table 0 : ONLY_IF_RO { *(.gcc_except_table) }\n  .gnu_extab 0 : ONLY_IF_RO { *(.gnu_extab*) }\n  /* These sections are generated by the Sun/Oracle C++ compiler.  */\n  .exception_ranges 0 : ONLY_IF_RO { *(.exception_ranges) }\n  /* Exception handling  */\n  .eh_frame     0 : ONLY_IF_RW { KEEP (*(.eh_frame)) }\n  .gnu_extab    0 : ONLY_IF_RW { *(.gnu_extab) }\n  .gcc_except_table 0 : ONLY_IF_RW { *(.gcc_except_table) }\n  .exception_ranges 0 : ONLY_IF_RW { *(.exception_ranges) }\n  /* Thread Local Storage sections  */\n  .tdata\t0 :\n   {\n     *(.tdata)\n   }\n  .tbss\t\t0 : { *(.tbss) }\n  .jcr          0 : { KEEP (*(.jcr)) }\n  .dynamic      0 : { *(.dynamic) }\n  .got          0 : { *(.got) }\n  .got.plt      0 : { *(.got.plt) }\n  .data         0 :\n  {\n    *(.data)\n    SORT(CONSTRUCTORS)\n  }\n  .data1        0 : { *(.data1) }\n  .bss          0 :\n  {\n   *(.bss)\n  }\n  .lbss 0 :\n  {\n    *(.lbss)\n  }\n  .lrodata 0  :\n  {\n    *(.lrodata)\n  }\n  .ldata 0  :\n  {\n    *(.ldata)\n  }\n  /* Stabs debugging sections.  */\n  .stab          0 : { *(.stab) }\n  .stabstr       0 : { *(.stabstr) }\n  .stab.excl     0 : { *(.stab.excl) }\n  .stab.exclstr  0 : { *(.stab.exclstr) }\n  .stab.index    0 : { *(.stab.index) }\n  .stab.indexstr 0 : { *(.stab.indexstr) }\n  .comment       0 : { *(.comment) }\n  .gnu.build.attributes : { *(.gnu.build.attributes) }\n  /* DWARF debug sections.\n     Symbols in the DWARF debugging sections are relative to the beginning\n     of the section so we begin them at 0.  */\n  /* DWARF 1.  */\n  .debug          0 : { *(.debug) }\n  .line           0 : { *(.line) }\n  /* GNU DWARF 1 extensions.  */\n  .debug_srcinfo  0 : { *(.debug_srcinfo) }\n  .debug_sfnames  0 : { *(.debug_sfnames) }\n  /* DWARF 1.1 and DWARF 2.  */\n  .debug_aranges  0 : { *(.debug_aranges) }\n  .debug_pubnames 0 : { *(.debug_pubnames) }\n  /* DWARF 2.  */\n  .debug_info     0 : { *(.debug_info) }\n  .debug_abbrev   0 : { *(.debug_abbrev) }\n  .debug_line     0 : { *(.debug_line) }\n  .debug_frame    0 : { *(.debug_frame) }\n  .debug_str      0 : { *(.debug_str) }\n  .debug_loc      0 : { *(.debug_loc) }\n  .debug_macinfo  0 : { *(.debug_macinfo) }\n  /* SGI/MIPS DWARF 2 extensions.  */\n  .debug_weaknames 0 : { *(.debug_weaknames) }\n  .debug_funcnames 0 : { *(.debug_funcnames) }\n  .debug_typenames 0 : { *(.debug_typenames) }\n  .debug_varnames  0 : { *(.debug_varnames) }\n  /* DWARF 3.  */\n  .debug_pubtypes 0 : { *(.debug_pubtypes) }\n  .debug_ranges   0 : { *(.debug_ranges) }\n  /* DWARF 5.  */\n  .debug_addr     0 : { *(.debug_addr) }\n  .debug_line_str 0 : { *(.debug_line_str) }\n  .debug_loclists 0 : { *(.debug_loclists) }\n  .debug_macro    0 : { *(.debug_macro) }\n  .debug_names    0 : { *(.debug_names) }\n  .debug_rnglists 0 : { *(.debug_rnglists) }\n  .debug_str_offsets 0 : { *(.debug_str_offsets) }\n  .debug_sup      0 : { *(.debug_sup) }\n  .gnu.attributes 0 : { KEEP (*(.gnu.attributes)) }\n}\n\n\0"
            as *const u8 as *const libc::c_char as *mut libc::c_char
    } else if link_info.type_0() as libc::c_int == type_relocatable as libc::c_int {
        return b"/* Script for -r */\n/* Copyright (C) 2014-2021 Free Software Foundation, Inc.\n   Copying and distribution of this script, with or without modification,\n   are permitted in any medium without royalty provided the copyright\n   notice and this notice are preserved.  */\nOUTPUT_FORMAT(\"elf64-x86-64\", \"elf64-x86-64\",\n\t      \"elf64-x86-64\")\nOUTPUT_ARCH(i386:x86-64)\n /* For some reason, the Solaris linker makes bad executables\n  if gld -r is used and the intermediate file has sections starting\n  at non-zero addresses.  Could be a Solaris ld bug, could be a GNU ld\n  bug.  But for now assigning the zero vmas works.  */\nSECTIONS\n{\n  .interp       0 : { *(.interp) }\n  .note.gnu.build-id 0: { *(.note.gnu.build-id) }\n  .hash         0 : { *(.hash) }\n  .gnu.hash     0 : { *(.gnu.hash) }\n  .dynsym       0 : { *(.dynsym) }\n  .dynstr       0 : { *(.dynstr) }\n  .gnu.version  0 : { *(.gnu.version) }\n  .gnu.version_d 0: { *(.gnu.version_d) }\n  .gnu.version_r 0: { *(.gnu.version_r) }\n  .rela.init    0 : { *(.rela.init) }\n  .rela.text    0 : { *(.rela.text) }\n  .rela.fini    0 : { *(.rela.fini) }\n  .rela.rodata  0 : { *(.rela.rodata) }\n  .rela.data.rel.ro 0 : { *(.rela.data.rel.ro) }\n  .rela.data    0 : { *(.rela.data) }\n  .rela.tdata\t0 : { *(.rela.tdata) }\n  .rela.tbss\t0 : { *(.rela.tbss) }\n  .rela.ctors   0 : { *(.rela.ctors) }\n  .rela.dtors   0 : { *(.rela.dtors) }\n  .rela.got     0 : { *(.rela.got) }\n  .rela.bss     0 : { *(.rela.bss) }\n  .rela.ldata   0 : { *(.rela.ldata) }\n  .rela.lbss    0 : { *(.rela.lbss) }\n  .rela.lrodata 0 : { *(.rela.lrodata) }\n  .rela.ifunc   0 : { *(.rela.ifunc) }\n  .rela.plt     0 :\n    {\n      *(.rela.plt)\n    }\n  .init         0 :\n  {\n    KEEP (*(SORT_NONE(.init)))\n  }\n  .plt          0 : { *(.plt) }\n.plt.got      0 : { *(.plt.got) }\n.plt.sec      0 : { *(.plt.sec) }\n  .text         0 :\n  {\n    *(.text .stub)\n    /* .gnu.warning sections are handled specially by elf.em.  */\n    *(.gnu.warning)\n  }\n  .fini         0 :\n  {\n    KEEP (*(SORT_NONE(.fini)))\n  }\n  .rodata       0 : { *(.rodata) }\n  .rodata1      0 : { *(.rodata1) }\n  .eh_frame_hdr 0 : { *(.eh_frame_hdr) }\n  .eh_frame     0 : ONLY_IF_RO { KEEP (*(.eh_frame)) }\n  .gcc_except_table 0 : ONLY_IF_RO { *(.gcc_except_table) }\n  .gnu_extab 0 : ONLY_IF_RO { *(.gnu_extab*) }\n  /* These sections are generated by the Sun/Oracle C++ compiler.  */\n  .exception_ranges 0 : ONLY_IF_RO { *(.exception_ranges) }\n  /* Exception handling  */\n  .eh_frame     0 : ONLY_IF_RW { KEEP (*(.eh_frame)) }\n  .gnu_extab    0 : ONLY_IF_RW { *(.gnu_extab) }\n  .gcc_except_table 0 : ONLY_IF_RW { *(.gcc_except_table) }\n  .exception_ranges 0 : ONLY_IF_RW { *(.exception_ranges) }\n  /* Thread Local Storage sections  */\n  .tdata\t0 :\n   {\n     *(.tdata)\n   }\n  .tbss\t\t0 : { *(.tbss) }\n  .jcr          0 : { KEEP (*(.jcr)) }\n  .dynamic      0 : { *(.dynamic) }\n  .got          0 : { *(.got) }\n  .got.plt      0 : { *(.got.plt) }\n  .data         0 :\n  {\n    *(.data)\n  }\n  .data1        0 : { *(.data1) }\n  .bss          0 :\n  {\n   *(.bss)\n  }\n  .lbss 0 :\n  {\n    *(.lbss)\n  }\n  .lrodata 0  :\n  {\n    *(.lrodata)\n  }\n  .ldata 0  :\n  {\n    *(.ldata)\n  }\n  /* Stabs debugging sections.  */\n  .stab          0 : { *(.stab) }\n  .stabstr       0 : { *(.stabstr) }\n  .stab.excl     0 : { *(.stab.excl) }\n  .stab.exclstr  0 : { *(.stab.exclstr) }\n  .stab.index    0 : { *(.stab.index) }\n  .stab.indexstr 0 : { *(.stab.indexstr) }\n  .comment       0 : { *(.comment) }\n  .gnu.build.attributes : { *(.gnu.build.attributes) }\n  /* DWARF debug sections.\n     Symbols in the DWARF debugging sections are relative to the beginning\n     of the section so we begin them at 0.  */\n  /* DWARF 1.  */\n  .debug          0 : { *(.debug) }\n  .line           0 : { *(.line) }\n  /* GNU DWARF 1 extensions.  */\n  .debug_srcinfo  0 : { *(.debug_srcinfo) }\n  .debug_sfnames  0 : { *(.debug_sfnames) }\n  /* DWARF 1.1 and DWARF 2.  */\n  .debug_aranges  0 : { *(.debug_aranges) }\n  .debug_pubnames 0 : { *(.debug_pubnames) }\n  /* DWARF 2.  */\n  .debug_info     0 : { *(.debug_info) }\n  .debug_abbrev   0 : { *(.debug_abbrev) }\n  .debug_line     0 : { *(.debug_line) }\n  .debug_frame    0 : { *(.debug_frame) }\n  .debug_str      0 : { *(.debug_str) }\n  .debug_loc      0 : { *(.debug_loc) }\n  .debug_macinfo  0 : { *(.debug_macinfo) }\n  /* SGI/MIPS DWARF 2 extensions.  */\n  .debug_weaknames 0 : { *(.debug_weaknames) }\n  .debug_funcnames 0 : { *(.debug_funcnames) }\n  .debug_typenames 0 : { *(.debug_typenames) }\n  .debug_varnames  0 : { *(.debug_varnames) }\n  /* DWARF 3.  */\n  .debug_pubtypes 0 : { *(.debug_pubtypes) }\n  .debug_ranges   0 : { *(.debug_ranges) }\n  /* DWARF 5.  */\n  .debug_addr     0 : { *(.debug_addr) }\n  .debug_line_str 0 : { *(.debug_line_str) }\n  .debug_loclists 0 : { *(.debug_loclists) }\n  .debug_macro    0 : { *(.debug_macro) }\n  .debug_names    0 : { *(.debug_names) }\n  .debug_rnglists 0 : { *(.debug_rnglists) }\n  .debug_str_offsets 0 : { *(.debug_str_offsets) }\n  .debug_sup      0 : { *(.debug_sup) }\n  .gnu.attributes 0 : { KEEP (*(.gnu.attributes)) }\n}\n\n\0"
            as *const u8 as *const libc::c_char as *mut libc::c_char
    } else if !config.text_read_only {
        return b"/* Script for -N */\n/* Copyright (C) 2014-2021 Free Software Foundation, Inc.\n   Copying and distribution of this script, with or without modification,\n   are permitted in any medium without royalty provided the copyright\n   notice and this notice are preserved.  */\nOUTPUT_FORMAT(\"elf64-x86-64\", \"elf64-x86-64\",\n\t      \"elf64-x86-64\")\nOUTPUT_ARCH(i386:x86-64)\nENTRY(_start)\nSEARCH_DIR(\"=/usr/local/x86_64-pc-linux-gnu/lib64\"); SEARCH_DIR(\"=/usr/local/lib64\"); SEARCH_DIR(\"=/lib64\"); SEARCH_DIR(\"=/usr/lib64\"); SEARCH_DIR(\"=/usr/local/x86_64-pc-linux-gnu/lib\"); SEARCH_DIR(\"=/usr/local/lib\"); SEARCH_DIR(\"=/lib\"); SEARCH_DIR(\"=/usr/lib\");\nSECTIONS\n{\n  /* Read-only sections, merged into text segment: */\n  PROVIDE (__executable_start = SEGMENT_START(\"text-segment\", 0x400000)); . = SEGMENT_START(\"text-segment\", 0x400000) + SIZEOF_HEADERS;\n  .interp         : { *(.interp) }\n  .note.gnu.build-id  : { *(.note.gnu.build-id) }\n  .hash           : { *(.hash) }\n  .gnu.hash       : { *(.gnu.hash) }\n  .dynsym         : { *(.dynsym) }\n  .dynstr         : { *(.dynstr) }\n  .gnu.version    : { *(.gnu.version) }\n  .gnu.version_d  : { *(.gnu.version_d) }\n  .gnu.version_r  : { *(.gnu.version_r) }\n  .rela.init      : { *(.rela.init) }\n  .rela.text      : { *(.rela.text .rela.text.* .rela.gnu.linkonce.t.*) }\n  .rela.fini      : { *(.rela.fini) }\n  .rela.rodata    : { *(.rela.rodata .rela.rodata.* .rela.gnu.linkonce.r.*) }\n  .rela.data.rel.ro   : { *(.rela.data.rel.ro .rela.data.rel.ro.* .rela.gnu.linkonce.d.rel.ro.*) }\n  .rela.data      : { *(.rela.data .rela.data.* .rela.gnu.linkonce.d.*) }\n  .rela.tdata\t  : { *(.rela.tdata .rela.tdata.* .rela.gnu.linkonce.td.*) }\n  .rela.tbss\t  : { *(.rela.tbss .rela.tbss.* .rela.gnu.linkonce.tb.*) }\n  .rela.ctors     : { *(.rela.ctors) }\n  .rela.dtors     : { *(.rela.dtors) }\n  .rela.got       : { *(.rela.got) }\n  .rela.bss       : { *(.rela.bss .rela.bss.* .rela.gnu.linkonce.b.*) }\n  .rela.ldata     : { *(.rela.ldata .rela.ldata.* .rela.gnu.linkonce.l.*) }\n  .rela.lbss      : { *(.rela.lbss .rela.lbss.* .rela.gnu.linkonce.lb.*) }\n  .rela.lrodata   : { *(.rela.lrodata .rela.lrodata.* .rela.gnu.linkonce.lr.*) }\n  .rela.ifunc     : { *(.rela.ifunc) }\n  .rela.plt       :\n    {\n      *(.rela.plt)\n      PROVIDE_HIDDEN (__rela_iplt_start = .);\n      *(.rela.iplt)\n      PROVIDE_HIDDEN (__rela_iplt_end = .);\n    }\n  .init           :\n  {\n    KEEP (*(SORT_NONE(.init)))\n  }\n  .plt            : { *(.plt) *(.iplt) }\n.plt.got        : { *(.plt.got) }\n.plt.sec        : { *(.plt.sec) }\n  .text           :\n  {\n    *(.text.unlikely .text.*_unlikely .text.unlikely.*)\n    *(.text.exit .text.exit.*)\n    *(.text.startup .text.startup.*)\n    *(.text.hot .text.hot.*)\n    *(SORT(.text.sorted.*))\n    *(.text .stub .text.* .gnu.linkonce.t.*)\n    /* .gnu.warning sections are handled specially by elf.em.  */\n    *(.gnu.warning)\n  }\n  .fini           :\n  {\n    KEEP (*(SORT_NONE(.fini)))\n  }\n  PROVIDE (__etext = .);\n  PROVIDE (_etext = .);\n  PROVIDE (etext = .);\n  .rodata         : { *(.rodata .rodata.* .gnu.linkonce.r.*) }\n  .rodata1        : { *(.rodata1) }\n  .eh_frame_hdr   : { *(.eh_frame_hdr) *(.eh_frame_entry .eh_frame_entry.*) }\n  .eh_frame       : ONLY_IF_RO { KEEP (*(.eh_frame)) *(.eh_frame.*) }\n  .gcc_except_table   : ONLY_IF_RO { *(.gcc_except_table .gcc_except_table.*) }\n  .gnu_extab   : ONLY_IF_RO { *(.gnu_extab*) }\n  /* These sections are generated by the Sun/Oracle C++ compiler.  */\n  .exception_ranges   : ONLY_IF_RO { *(.exception_ranges*) }\n  /* Adjust the address for the data segment.  We want to adjust up to\n     the same address within the page on the next page up.  */\n  . = .;\n  /* Exception handling  */\n  .eh_frame       : ONLY_IF_RW { KEEP (*(.eh_frame)) *(.eh_frame.*) }\n  .gnu_extab      : ONLY_IF_RW { *(.gnu_extab) }\n  .gcc_except_table   : ONLY_IF_RW { *(.gcc_except_table .gcc_except_table.*) }\n  .exception_ranges   : ONLY_IF_RW { *(.exception_ranges*) }\n  /* Thread Local Storage sections  */\n  .tdata\t  :\n   {\n     PROVIDE_HIDDEN (__tdata_start = .);\n     *(.tdata .tdata.* .gnu.linkonce.td.*)\n   }\n  .tbss\t\t  : { *(.tbss .tbss.* .gnu.linkonce.tb.*) *(.tcommon) }\n  .preinit_array    :\n  {\n    PROVIDE_HIDDEN (__preinit_array_start = .);\n    KEEP (*(.preinit_array))\n    PROVIDE_HIDDEN (__preinit_array_end = .);\n  }\n  .init_array    :\n  {\n    PROVIDE_HIDDEN (__init_array_start = .);\n    KEEP (*(SORT_BY_INIT_PRIORITY(.init_array.*) SORT_BY_INIT_PRIORITY(.ctors.*)))\n    KEEP (*(.init_array EXCLUDE_FILE (*crtbegin.o *crtbegin?.o *crtend.o *crtend?.o ) .ctors))\n    PROVIDE_HIDDEN (__init_array_end = .);\n  }\n  .fini_array    :\n  {\n    PROVIDE_HIDDEN (__fini_array_start = .);\n    KEEP (*(SORT_BY_INIT_PRIORITY(.fini_array.*) SORT_BY_INIT_PRIORITY(.dtors.*)))\n    KEEP (*(.fini_array EXCLUDE_FILE (*crtbegin.o *crtbegin?.o *crtend.o *crtend?.o ) .dtors))\n    PROVIDE_HIDDEN (__fini_array_end = .);\n  }\n  .ctors          :\n  {\n    /* gcc uses crtbegin.o to find the start of\n       the constructors, so we make sure it is\n       first.  Because this is a wildcard, it\n       doesn't matter if the user does not\n       actually link against crtbegin.o; the\n       linker won't look for a file to match a\n       wildcard.  The wildcard also means that it\n       doesn't matter which directory crtbegin.o\n       is in.  */\n    KEEP (*crtbegin.o(.ctors))\n    KEEP (*crtbegin?.o(.ctors))\n    /* We don't want to include the .ctor section from\n       the crtend.o file until after the sorted ctors.\n       The .ctor section from the crtend file contains the\n       end of ctors marker and it must be last */\n    KEEP (*(EXCLUDE_FILE (*crtend.o *crtend?.o ) .ctors))\n    KEEP (*(SORT(.ctors.*)))\n    KEEP (*(.ctors))\n  }\n  .dtors          :\n  {\n    KEEP (*crtbegin.o(.dtors))\n    KEEP (*crtbegin?.o(.dtors))\n    KEEP (*(EXCLUDE_FILE (*crtend.o *crtend?.o ) .dtors))\n    KEEP (*(SORT(.dtors.*)))\n    KEEP (*(.dtors))\n  }\n  .jcr            : { KEEP (*(.jcr)) }\n  .data.rel.ro : { *(.data.rel.ro.local* .gnu.linkonce.d.rel.ro.local.*) *(.data.rel.ro .data.rel.ro.* .gnu.linkonce.d.rel.ro.*) }\n  .dynamic        : { *(.dynamic) }\n  .got            : { *(.got) *(.igot) }\n  .got.plt        : { *(.got.plt) *(.igot.plt) }\n  .data           :\n  {\n    *(.data .data.* .gnu.linkonce.d.*)\n    SORT(CONSTRUCTORS)\n  }\n  .data1          : { *(.data1) }\n  _edata = .; PROVIDE (edata = .);\n  . = .;\n  __bss_start = .;\n  .bss            :\n  {\n   *(.dynbss)\n   *(.bss .bss.* .gnu.linkonce.b.*)\n   *(COMMON)\n   /* Align here to ensure that the .bss section occupies space up to\n      _end.  Align after .bss to ensure correct alignment even if the\n      .bss section disappears because there are no input sections.\n      FIXME: Why do we need it? When there is no .bss section, we do not\n      pad the .data section.  */\n   . = ALIGN(. != 0 ? 64 / 8 : 1);\n  }\n  .lbss   :\n  {\n    *(.dynlbss)\n    *(.lbss .lbss.* .gnu.linkonce.lb.*)\n    *(LARGE_COMMON)\n  }\n  . = ALIGN(64 / 8);\n  . = SEGMENT_START(\"ldata-segment\", .);\n  .lrodata   ALIGN(CONSTANT (MAXPAGESIZE)) + (. & (CONSTANT (MAXPAGESIZE) - 1)) :\n  {\n    *(.lrodata .lrodata.* .gnu.linkonce.lr.*)\n  }\n  .ldata   ALIGN(CONSTANT (MAXPAGESIZE)) + (. & (CONSTANT (MAXPAGESIZE) - 1)) :\n  {\n    *(.ldata .ldata.* .gnu.linkonce.l.*)\n    . = ALIGN(. != 0 ? 64 / 8 : 1);\n  }\n  . = ALIGN(64 / 8);\n  _end = .; PROVIDE (end = .);\n  /* Stabs debugging sections.  */\n  .stab          0 : { *(.stab) }\n  .stabstr       0 : { *(.stabstr) }\n  .stab.excl     0 : { *(.stab.excl) }\n  .stab.exclstr  0 : { *(.stab.exclstr) }\n  .stab.index    0 : { *(.stab.index) }\n  .stab.indexstr 0 : { *(.stab.indexstr) }\n  .comment       0 : { *(.comment) }\n  .gnu.build.attributes : { *(.gnu.build.attributes .gnu.build.attributes.*) }\n  /* DWARF debug sections.\n     Symbols in the DWARF debugging sections are relative to the beginning\n     of the section so we begin them at 0.  */\n  /* DWARF 1.  */\n  .debug          0 : { *(.debug) }\n  .line           0 : { *(.line) }\n  /* GNU DWARF 1 extensions.  */\n  .debug_srcinfo  0 : { *(.debug_srcinfo) }\n  .debug_sfnames  0 : { *(.debug_sfnames) }\n  /* DWARF 1.1 and DWARF 2.  */\n  .debug_aranges  0 : { *(.debug_aranges) }\n  .debug_pubnames 0 : { *(.debug_pubnames) }\n  /* DWARF 2.  */\n  .debug_info     0 : { *(.debug_info .gnu.linkonce.wi.*) }\n  .debug_abbrev   0 : { *(.debug_abbrev) }\n  .debug_line     0 : { *(.debug_line .debug_line.* .debug_line_end) }\n  .debug_frame    0 : { *(.debug_frame) }\n  .debug_str      0 : { *(.debug_str) }\n  .debug_loc      0 : { *(.debug_loc) }\n  .debug_macinfo  0 : { *(.debug_macinfo) }\n  /* SGI/MIPS DWARF 2 extensions.  */\n  .debug_weaknames 0 : { *(.debug_weaknames) }\n  .debug_funcnames 0 : { *(.debug_funcnames) }\n  .debug_typenames 0 : { *(.debug_typenames) }\n  .debug_varnames  0 : { *(.debug_varnames) }\n  /* DWARF 3.  */\n  .debug_pubtypes 0 : { *(.debug_pubtypes) }\n  .debug_ranges   0 : { *(.debug_ranges) }\n  /* DWARF 5.  */\n  .debug_addr     0 : { *(.debug_addr) }\n  .debug_line_str 0 : { *(.debug_line_str) }\n  .debug_loclists 0 : { *(.debug_loclists) }\n  .debug_macro    0 : { *(.debug_macro) }\n  .debug_names    0 : { *(.debug_names) }\n  .debug_rnglists 0 : { *(.debug_rnglists) }\n  .debug_str_offsets 0 : { *(.debug_str_offsets) }\n  .debug_sup      0 : { *(.debug_sup) }\n  .gnu.attributes 0 : { KEEP (*(.gnu.attributes)) }\n  /DISCARD/ : { *(.note.GNU-stack) *(.gnu_debuglink) *(.gnu.lto_*) }\n}\n\n\0"
            as *const u8 as *const libc::c_char as *mut libc::c_char
    } else if !config.magic_demand_paged {
        return b"/* Script for -n */\n/* Copyright (C) 2014-2021 Free Software Foundation, Inc.\n   Copying and distribution of this script, with or without modification,\n   are permitted in any medium without royalty provided the copyright\n   notice and this notice are preserved.  */\nOUTPUT_FORMAT(\"elf64-x86-64\", \"elf64-x86-64\",\n\t      \"elf64-x86-64\")\nOUTPUT_ARCH(i386:x86-64)\nENTRY(_start)\nSEARCH_DIR(\"=/usr/local/x86_64-pc-linux-gnu/lib64\"); SEARCH_DIR(\"=/usr/local/lib64\"); SEARCH_DIR(\"=/lib64\"); SEARCH_DIR(\"=/usr/lib64\"); SEARCH_DIR(\"=/usr/local/x86_64-pc-linux-gnu/lib\"); SEARCH_DIR(\"=/usr/local/lib\"); SEARCH_DIR(\"=/lib\"); SEARCH_DIR(\"=/usr/lib\");\nSECTIONS\n{\n  /* Read-only sections, merged into text segment: */\n  PROVIDE (__executable_start = SEGMENT_START(\"text-segment\", 0x400000)); . = SEGMENT_START(\"text-segment\", 0x400000) + SIZEOF_HEADERS;\n  .interp         : { *(.interp) }\n  .note.gnu.build-id  : { *(.note.gnu.build-id) }\n  .hash           : { *(.hash) }\n  .gnu.hash       : { *(.gnu.hash) }\n  .dynsym         : { *(.dynsym) }\n  .dynstr         : { *(.dynstr) }\n  .gnu.version    : { *(.gnu.version) }\n  .gnu.version_d  : { *(.gnu.version_d) }\n  .gnu.version_r  : { *(.gnu.version_r) }\n  .rela.init      : { *(.rela.init) }\n  .rela.text      : { *(.rela.text .rela.text.* .rela.gnu.linkonce.t.*) }\n  .rela.fini      : { *(.rela.fini) }\n  .rela.rodata    : { *(.rela.rodata .rela.rodata.* .rela.gnu.linkonce.r.*) }\n  .rela.data.rel.ro   : { *(.rela.data.rel.ro .rela.data.rel.ro.* .rela.gnu.linkonce.d.rel.ro.*) }\n  .rela.data      : { *(.rela.data .rela.data.* .rela.gnu.linkonce.d.*) }\n  .rela.tdata\t  : { *(.rela.tdata .rela.tdata.* .rela.gnu.linkonce.td.*) }\n  .rela.tbss\t  : { *(.rela.tbss .rela.tbss.* .rela.gnu.linkonce.tb.*) }\n  .rela.ctors     : { *(.rela.ctors) }\n  .rela.dtors     : { *(.rela.dtors) }\n  .rela.got       : { *(.rela.got) }\n  .rela.bss       : { *(.rela.bss .rela.bss.* .rela.gnu.linkonce.b.*) }\n  .rela.ldata     : { *(.rela.ldata .rela.ldata.* .rela.gnu.linkonce.l.*) }\n  .rela.lbss      : { *(.rela.lbss .rela.lbss.* .rela.gnu.linkonce.lb.*) }\n  .rela.lrodata   : { *(.rela.lrodata .rela.lrodata.* .rela.gnu.linkonce.lr.*) }\n  .rela.ifunc     : { *(.rela.ifunc) }\n  .rela.plt       :\n    {\n      *(.rela.plt)\n      PROVIDE_HIDDEN (__rela_iplt_start = .);\n      *(.rela.iplt)\n      PROVIDE_HIDDEN (__rela_iplt_end = .);\n    }\n  .init           :\n  {\n    KEEP (*(SORT_NONE(.init)))\n  }\n  .plt            : { *(.plt) *(.iplt) }\n.plt.got        : { *(.plt.got) }\n.plt.sec        : { *(.plt.sec) }\n  .text           :\n  {\n    *(.text.unlikely .text.*_unlikely .text.unlikely.*)\n    *(.text.exit .text.exit.*)\n    *(.text.startup .text.startup.*)\n    *(.text.hot .text.hot.*)\n    *(SORT(.text.sorted.*))\n    *(.text .stub .text.* .gnu.linkonce.t.*)\n    /* .gnu.warning sections are handled specially by elf.em.  */\n    *(.gnu.warning)\n  }\n  .fini           :\n  {\n    KEEP (*(SORT_NONE(.fini)))\n  }\n  PROVIDE (__etext = .);\n  PROVIDE (_etext = .);\n  PROVIDE (etext = .);\n  .rodata         : { *(.rodata .rodata.* .gnu.linkonce.r.*) }\n  .rodata1        : { *(.rodata1) }\n  .eh_frame_hdr   : { *(.eh_frame_hdr) *(.eh_frame_entry .eh_frame_entry.*) }\n  .eh_frame       : ONLY_IF_RO { KEEP (*(.eh_frame)) *(.eh_frame.*) }\n  .gcc_except_table   : ONLY_IF_RO { *(.gcc_except_table .gcc_except_table.*) }\n  .gnu_extab   : ONLY_IF_RO { *(.gnu_extab*) }\n  /* These sections are generated by the Sun/Oracle C++ compiler.  */\n  .exception_ranges   : ONLY_IF_RO { *(.exception_ranges*) }\n  /* Adjust the address for the data segment.  We want to adjust up to\n     the same address within the page on the next page up.  */\n  . = DATA_SEGMENT_ALIGN (CONSTANT (MAXPAGESIZE), CONSTANT (COMMONPAGESIZE));\n  /* Exception handling  */\n  .eh_frame       : ONLY_IF_RW { KEEP (*(.eh_frame)) *(.eh_frame.*) }\n  .gnu_extab      : ONLY_IF_RW { *(.gnu_extab) }\n  .gcc_except_table   : ONLY_IF_RW { *(.gcc_except_table .gcc_except_table.*) }\n  .exception_ranges   : ONLY_IF_RW { *(.exception_ranges*) }\n  /* Thread Local Storage sections  */\n  .tdata\t  :\n   {\n     PROVIDE_HIDDEN (__tdata_start = .);\n     *(.tdata .tdata.* .gnu.linkonce.td.*)\n   }\n  .tbss\t\t  : { *(.tbss .tbss.* .gnu.linkonce.tb.*) *(.tcommon) }\n  .preinit_array    :\n  {\n    PROVIDE_HIDDEN (__preinit_array_start = .);\n    KEEP (*(.preinit_array))\n    PROVIDE_HIDDEN (__preinit_array_end = .);\n  }\n  .init_array    :\n  {\n    PROVIDE_HIDDEN (__init_array_start = .);\n    KEEP (*(SORT_BY_INIT_PRIORITY(.init_array.*) SORT_BY_INIT_PRIORITY(.ctors.*)))\n    KEEP (*(.init_array EXCLUDE_FILE (*crtbegin.o *crtbegin?.o *crtend.o *crtend?.o ) .ctors))\n    PROVIDE_HIDDEN (__init_array_end = .);\n  }\n  .fini_array    :\n  {\n    PROVIDE_HIDDEN (__fini_array_start = .);\n    KEEP (*(SORT_BY_INIT_PRIORITY(.fini_array.*) SORT_BY_INIT_PRIORITY(.dtors.*)))\n    KEEP (*(.fini_array EXCLUDE_FILE (*crtbegin.o *crtbegin?.o *crtend.o *crtend?.o ) .dtors))\n    PROVIDE_HIDDEN (__fini_array_end = .);\n  }\n  .ctors          :\n  {\n    /* gcc uses crtbegin.o to find the start of\n       the constructors, so we make sure it is\n       first.  Because this is a wildcard, it\n       doesn't matter if the user does not\n       actually link against crtbegin.o; the\n       linker won't look for a file to match a\n       wildcard.  The wildcard also means that it\n       doesn't matter which directory crtbegin.o\n       is in.  */\n    KEEP (*crtbegin.o(.ctors))\n    KEEP (*crtbegin?.o(.ctors))\n    /* We don't want to include the .ctor section from\n       the crtend.o file until after the sorted ctors.\n       The .ctor section from the crtend file contains the\n       end of ctors marker and it must be last */\n    KEEP (*(EXCLUDE_FILE (*crtend.o *crtend?.o ) .ctors))\n    KEEP (*(SORT(.ctors.*)))\n    KEEP (*(.ctors))\n  }\n  .dtors          :\n  {\n    KEEP (*crtbegin.o(.dtors))\n    KEEP (*crtbegin?.o(.dtors))\n    KEEP (*(EXCLUDE_FILE (*crtend.o *crtend?.o ) .dtors))\n    KEEP (*(SORT(.dtors.*)))\n    KEEP (*(.dtors))\n  }\n  .jcr            : { KEEP (*(.jcr)) }\n  .data.rel.ro : { *(.data.rel.ro.local* .gnu.linkonce.d.rel.ro.local.*) *(.data.rel.ro .data.rel.ro.* .gnu.linkonce.d.rel.ro.*) }\n  .dynamic        : { *(.dynamic) }\n  .got            : { *(.got) *(.igot) }\n  . = DATA_SEGMENT_RELRO_END (SIZEOF (.got.plt) >= 24 ? 24 : 0, .);\n  .got.plt        : { *(.got.plt) *(.igot.plt) }\n  .data           :\n  {\n    *(.data .data.* .gnu.linkonce.d.*)\n    SORT(CONSTRUCTORS)\n  }\n  .data1          : { *(.data1) }\n  _edata = .; PROVIDE (edata = .);\n  . = .;\n  __bss_start = .;\n  .bss            :\n  {\n   *(.dynbss)\n   *(.bss .bss.* .gnu.linkonce.b.*)\n   *(COMMON)\n   /* Align here to ensure that the .bss section occupies space up to\n      _end.  Align after .bss to ensure correct alignment even if the\n      .bss section disappears because there are no input sections.\n      FIXME: Why do we need it? When there is no .bss section, we do not\n      pad the .data section.  */\n   . = ALIGN(. != 0 ? 64 / 8 : 1);\n  }\n  .lbss   :\n  {\n    *(.dynlbss)\n    *(.lbss .lbss.* .gnu.linkonce.lb.*)\n    *(LARGE_COMMON)\n  }\n  . = ALIGN(64 / 8);\n  . = SEGMENT_START(\"ldata-segment\", .);\n  .lrodata   ALIGN(CONSTANT (MAXPAGESIZE)) + (. & (CONSTANT (MAXPAGESIZE) - 1)) :\n  {\n    *(.lrodata .lrodata.* .gnu.linkonce.lr.*)\n  }\n  .ldata   ALIGN(CONSTANT (MAXPAGESIZE)) + (. & (CONSTANT (MAXPAGESIZE) - 1)) :\n  {\n    *(.ldata .ldata.* .gnu.linkonce.l.*)\n    . = ALIGN(. != 0 ? 64 / 8 : 1);\n  }\n  . = ALIGN(64 / 8);\n  _end = .; PROVIDE (end = .);\n  . = DATA_SEGMENT_END (.);\n  /* Stabs debugging sections.  */\n  .stab          0 : { *(.stab) }\n  .stabstr       0 : { *(.stabstr) }\n  .stab.excl     0 : { *(.stab.excl) }\n  .stab.exclstr  0 : { *(.stab.exclstr) }\n  .stab.index    0 : { *(.stab.index) }\n  .stab.indexstr 0 : { *(.stab.indexstr) }\n  .comment       0 : { *(.comment) }\n  .gnu.build.attributes : { *(.gnu.build.attributes .gnu.build.attributes.*) }\n  /* DWARF debug sections.\n     Symbols in the DWARF debugging sections are relative to the beginning\n     of the section so we begin them at 0.  */\n  /* DWARF 1.  */\n  .debug          0 : { *(.debug) }\n  .line           0 : { *(.line) }\n  /* GNU DWARF 1 extensions.  */\n  .debug_srcinfo  0 : { *(.debug_srcinfo) }\n  .debug_sfnames  0 : { *(.debug_sfnames) }\n  /* DWARF 1.1 and DWARF 2.  */\n  .debug_aranges  0 : { *(.debug_aranges) }\n  .debug_pubnames 0 : { *(.debug_pubnames) }\n  /* DWARF 2.  */\n  .debug_info     0 : { *(.debug_info .gnu.linkonce.wi.*) }\n  .debug_abbrev   0 : { *(.debug_abbrev) }\n  .debug_line     0 : { *(.debug_line .debug_line.* .debug_line_end) }\n  .debug_frame    0 : { *(.debug_frame) }\n  .debug_str      0 : { *(.debug_str) }\n  .debug_loc      0 : { *(.debug_loc) }\n  .debug_macinfo  0 : { *(.debug_macinfo) }\n  /* SGI/MIPS DWARF 2 extensions.  */\n  .debug_weaknames 0 : { *(.debug_weaknames) }\n  .debug_funcnames 0 : { *(.debug_funcnames) }\n  .debug_typenames 0 : { *(.debug_typenames) }\n  .debug_varnames  0 : { *(.debug_varnames) }\n  /* DWARF 3.  */\n  .debug_pubtypes 0 : { *(.debug_pubtypes) }\n  .debug_ranges   0 : { *(.debug_ranges) }\n  /* DWARF 5.  */\n  .debug_addr     0 : { *(.debug_addr) }\n  .debug_line_str 0 : { *(.debug_line_str) }\n  .debug_loclists 0 : { *(.debug_loclists) }\n  .debug_macro    0 : { *(.debug_macro) }\n  .debug_names    0 : { *(.debug_names) }\n  .debug_rnglists 0 : { *(.debug_rnglists) }\n  .debug_str_offsets 0 : { *(.debug_str_offsets) }\n  .debug_sup      0 : { *(.debug_sup) }\n  .gnu.attributes 0 : { KEEP (*(.gnu.attributes)) }\n  /DISCARD/ : { *(.note.GNU-stack) *(.gnu_debuglink) *(.gnu.lto_*) }\n}\n\n\0"
            as *const u8 as *const libc::c_char as *mut libc::c_char
    } else if link_info.type_0() as libc::c_int == type_pie as libc::c_int
        && link_info.combreloc() as libc::c_int != 0
        && link_info.separate_code() as libc::c_int != 0
        && link_info.flags & ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_ulong
            != 0
    {
        return b"/* Script for -pie -z combreloc -z separate-code -z relro -z now */\n/* Copyright (C) 2014-2021 Free Software Foundation, Inc.\n   Copying and distribution of this script, with or without modification,\n   are permitted in any medium without royalty provided the copyright\n   notice and this notice are preserved.  */\nOUTPUT_FORMAT(\"elf64-x86-64\", \"elf64-x86-64\",\n\t      \"elf64-x86-64\")\nOUTPUT_ARCH(i386:x86-64)\nENTRY(_start)\nSEARCH_DIR(\"=/usr/local/x86_64-pc-linux-gnu/lib64\"); SEARCH_DIR(\"=/usr/local/lib64\"); SEARCH_DIR(\"=/lib64\"); SEARCH_DIR(\"=/usr/lib64\"); SEARCH_DIR(\"=/usr/local/x86_64-pc-linux-gnu/lib\"); SEARCH_DIR(\"=/usr/local/lib\"); SEARCH_DIR(\"=/lib\"); SEARCH_DIR(\"=/usr/lib\");\nSECTIONS\n{\n  PROVIDE (__executable_start = SEGMENT_START(\"text-segment\", 0)); . = SEGMENT_START(\"text-segment\", 0) + SIZEOF_HEADERS;\n  .interp         : { *(.interp) }\n  .note.gnu.build-id  : { *(.note.gnu.build-id) }\n  .hash           : { *(.hash) }\n  .gnu.hash       : { *(.gnu.hash) }\n  .dynsym         : { *(.dynsym) }\n  .dynstr         : { *(.dynstr) }\n  .gnu.version    : { *(.gnu.version) }\n  .gnu.version_d  : { *(.gnu.version_d) }\n  .gnu.version_r  : { *(.gnu.version_r) }\n  .rela.dyn       :\n    {\n      *(.rela.init)\n      *(.rela.text .rela.text.* .rela.gnu.linkonce.t.*)\n      *(.rela.fini)\n      *(.rela.rodata .rela.rodata.* .rela.gnu.linkonce.r.*)\n      *(.rela.data .rela.data.* .rela.gnu.linkonce.d.*)\n      *(.rela.tdata .rela.tdata.* .rela.gnu.linkonce.td.*)\n      *(.rela.tbss .rela.tbss.* .rela.gnu.linkonce.tb.*)\n      *(.rela.ctors)\n      *(.rela.dtors)\n      *(.rela.got)\n      *(.rela.bss .rela.bss.* .rela.gnu.linkonce.b.*)\n      *(.rela.ldata .rela.ldata.* .rela.gnu.linkonce.l.*)\n      *(.rela.lbss .rela.lbss.* .rela.gnu.linkonce.lb.*)\n      *(.rela.lrodata .rela.lrodata.* .rela.gnu.linkonce.lr.*)\n      *(.rela.ifunc)\n    }\n  .rela.plt       :\n    {\n      *(.rela.plt)\n      *(.rela.iplt)\n    }\n  . = ALIGN(CONSTANT (MAXPAGESIZE));\n  .init           :\n  {\n    KEEP (*(SORT_NONE(.init)))\n  }\n  .plt            : { *(.plt) *(.iplt) }\n.plt.got        : { *(.plt.got) }\n.plt.sec        : { *(.plt.sec) }\n  .text           :\n  {\n    *(.text.unlikely .text.*_unlikely .text.unlikely.*)\n    *(.text.exit .text.exit.*)\n    *(.text.startup .text.startup.*)\n    *(.text.hot .text.hot.*)\n    *(SORT(.text.sorted.*))\n    *(.text .stub .text.* .gnu.linkonce.t.*)\n    /* .gnu.warning sections are handled specially by elf.em.  */\n    *(.gnu.warning)\n  }\n  .fini           :\n  {\n    KEEP (*(SORT_NONE(.fini)))\n  }\n  PROVIDE (__etext = .);\n  PROVIDE (_etext = .);\n  PROVIDE (etext = .);\n  . = ALIGN(CONSTANT (MAXPAGESIZE));\n  /* Adjust the address for the rodata segment.  We want to adjust up to\n     the same address within the page on the next page up.  */\n  . = SEGMENT_START(\"rodata-segment\", ALIGN(CONSTANT (MAXPAGESIZE)) + (. & (CONSTANT (MAXPAGESIZE) - 1)));\n  .rodata         : { *(.rodata .rodata.* .gnu.linkonce.r.*) }\n  .rodata1        : { *(.rodata1) }\n  .eh_frame_hdr   : { *(.eh_frame_hdr) *(.eh_frame_entry .eh_frame_entry.*) }\n  .eh_frame       : ONLY_IF_RO { KEEP (*(.eh_frame)) *(.eh_frame.*) }\n  .gcc_except_table   : ONLY_IF_RO { *(.gcc_except_table .gcc_except_table.*) }\n  .gnu_extab   : ONLY_IF_RO { *(.gnu_extab*) }\n  /* These sections are generated by the Sun/Oracle C++ compiler.  */\n  .exception_ranges   : ONLY_IF_RO { *(.exception_ranges*) }\n  /* Adjust the address for the data segment.  We want to adjust up to\n     the same address within the page on the next page up.  */\n  . = DATA_SEGMENT_ALIGN (CONSTANT (MAXPAGESIZE), CONSTANT (COMMONPAGESIZE));\n  /* Exception handling  */\n  .eh_frame       : ONLY_IF_RW { KEEP (*(.eh_frame)) *(.eh_frame.*) }\n  .gnu_extab      : ONLY_IF_RW { *(.gnu_extab) }\n  .gcc_except_table   : ONLY_IF_RW { *(.gcc_except_table .gcc_except_table.*) }\n  .exception_ranges   : ONLY_IF_RW { *(.exception_ranges*) }\n  /* Thread Local Storage sections  */\n  .tdata\t  :\n   {\n     PROVIDE_HIDDEN (__tdata_start = .);\n     *(.tdata .tdata.* .gnu.linkonce.td.*)\n   }\n  .tbss\t\t  : { *(.tbss .tbss.* .gnu.linkonce.tb.*) *(.tcommon) }\n  .preinit_array    :\n  {\n    PROVIDE_HIDDEN (__preinit_array_start = .);\n    KEEP (*(.preinit_array))\n    PROVIDE_HIDDEN (__preinit_array_end = .);\n  }\n  .init_array    :\n  {\n    PROVIDE_HIDDEN (__init_array_start = .);\n    KEEP (*(SORT_BY_INIT_PRIORITY(.init_array.*) SORT_BY_INIT_PRIORITY(.ctors.*)))\n    KEEP (*(.init_array EXCLUDE_FILE (*crtbegin.o *crtbegin?.o *crtend.o *crtend?.o ) .ctors))\n    PROVIDE_HIDDEN (__init_array_end = .);\n  }\n  .fini_array    :\n  {\n    PROVIDE_HIDDEN (__fini_array_start = .);\n    KEEP (*(SORT_BY_INIT_PRIORITY(.fini_array.*) SORT_BY_INIT_PRIORITY(.dtors.*)))\n    KEEP (*(.fini_array EXCLUDE_FILE (*crtbegin.o *crtbegin?.o *crtend.o *crtend?.o ) .dtors))\n    PROVIDE_HIDDEN (__fini_array_end = .);\n  }\n  .ctors          :\n  {\n    /* gcc uses crtbegin.o to find the start of\n       the constructors, so we make sure it is\n       first.  Because this is a wildcard, it\n       doesn't matter if the user does not\n       actually link against crtbegin.o; the\n       linker won't look for a file to match a\n       wildcard.  The wildcard also means that it\n       doesn't matter which directory crtbegin.o\n       is in.  */\n    KEEP (*crtbegin.o(.ctors))\n    KEEP (*crtbegin?.o(.ctors))\n    /* We don't want to include the .ctor section from\n       the crtend.o file until after the sorted ctors.\n       The .ctor section from the crtend file contains the\n       end of ctors marker and it must be last */\n    KEEP (*(EXCLUDE_FILE (*crtend.o *crtend?.o ) .ctors))\n    KEEP (*(SORT(.ctors.*)))\n    KEEP (*(.ctors))\n  }\n  .dtors          :\n  {\n    KEEP (*crtbegin.o(.dtors))\n    KEEP (*crtbegin?.o(.dtors))\n    KEEP (*(EXCLUDE_FILE (*crtend.o *crtend?.o ) .dtors))\n    KEEP (*(SORT(.dtors.*)))\n    KEEP (*(.dtors))\n  }\n  .jcr            : { KEEP (*(.jcr)) }\n  .data.rel.ro : { *(.data.rel.ro.local* .gnu.linkonce.d.rel.ro.local.*) *(.data.rel.ro .data.rel.ro.* .gnu.linkonce.d.rel.ro.*) }\n  .dynamic        : { *(.dynamic) }\n  .got            : { *(.got.plt) *(.igot.plt) *(.got) *(.igot) }\n  . = DATA_SEGMENT_RELRO_END (0, .);\n  .data           :\n  {\n    *(.data .data.* .gnu.linkonce.d.*)\n    SORT(CONSTRUCTORS)\n  }\n  .data1          : { *(.data1) }\n  _edata = .; PROVIDE (edata = .);\n  . = .;\n  __bss_start = .;\n  .bss            :\n  {\n   *(.dynbss)\n   *(.bss .bss.* .gnu.linkonce.b.*)\n   *(COMMON)\n   /* Align here to ensure that the .bss section occupies space up to\n      _end.  Align after .bss to ensure correct alignment even if the\n      .bss section disappears because there are no input sections.\n      FIXME: Why do we need it? When there is no .bss section, we do not\n      pad the .data section.  */\n   . = ALIGN(. != 0 ? 64 / 8 : 1);\n  }\n  .lbss   :\n  {\n    *(.dynlbss)\n    *(.lbss .lbss.* .gnu.linkonce.lb.*)\n    *(LARGE_COMMON)\n  }\n  . = ALIGN(64 / 8);\n  . = SEGMENT_START(\"ldata-segment\", .);\n  .lrodata   ALIGN(CONSTANT (MAXPAGESIZE)) + (. & (CONSTANT (MAXPAGESIZE) - 1)) :\n  {\n    *(.lrodata .lrodata.* .gnu.linkonce.lr.*)\n  }\n  .ldata   ALIGN(CONSTANT (MAXPAGESIZE)) + (. & (CONSTANT (MAXPAGESIZE) - 1)) :\n  {\n    *(.ldata .ldata.* .gnu.linkonce.l.*)\n    . = ALIGN(. != 0 ? 64 / 8 : 1);\n  }\n  . = ALIGN(64 / 8);\n  _end = .; PROVIDE (end = .);\n  . = DATA_SEGMENT_END (.);\n  /* Stabs debugging sections.  */\n  .stab          0 : { *(.stab) }\n  .stabstr       0 : { *(.stabstr) }\n  .stab.excl     0 : { *(.stab.excl) }\n  .stab.exclstr  0 : { *(.stab.exclstr) }\n  .stab.index    0 : { *(.stab.index) }\n  .stab.indexstr 0 : { *(.stab.indexstr) }\n  .comment       0 : { *(.comment) }\n  .gnu.build.attributes : { *(.gnu.build.attributes .gnu.build.attributes.*) }\n  /* DWARF debug sections.\n     Symbols in the DWARF debugging sections are relative to the beginning\n     of the section so we begin them at 0.  */\n  /* DWARF 1.  */\n  .debug          0 : { *(.debug) }\n  .line           0 : { *(.line) }\n  /* GNU DWARF 1 extensions.  */\n  .debug_srcinfo  0 : { *(.debug_srcinfo) }\n  .debug_sfnames  0 : { *(.debug_sfnames) }\n  /* DWARF 1.1 and DWARF 2.  */\n  .debug_aranges  0 : { *(.debug_aranges) }\n  .debug_pubnames 0 : { *(.debug_pubnames) }\n  /* DWARF 2.  */\n  .debug_info     0 : { *(.debug_info .gnu.linkonce.wi.*) }\n  .debug_abbrev   0 : { *(.debug_abbrev) }\n  .debug_line     0 : { *(.debug_line .debug_line.* .debug_line_end) }\n  .debug_frame    0 : { *(.debug_frame) }\n  .debug_str      0 : { *(.debug_str) }\n  .debug_loc      0 : { *(.debug_loc) }\n  .debug_macinfo  0 : { *(.debug_macinfo) }\n  /* SGI/MIPS DWARF 2 extensions.  */\n  .debug_weaknames 0 : { *(.debug_weaknames) }\n  .debug_funcnames 0 : { *(.debug_funcnames) }\n  .debug_typenames 0 : { *(.debug_typenames) }\n  .debug_varnames  0 : { *(.debug_varnames) }\n  /* DWARF 3.  */\n  .debug_pubtypes 0 : { *(.debug_pubtypes) }\n  .debug_ranges   0 : { *(.debug_ranges) }\n  /* DWARF 5.  */\n  .debug_addr     0 : { *(.debug_addr) }\n  .debug_line_str 0 : { *(.debug_line_str) }\n  .debug_loclists 0 : { *(.debug_loclists) }\n  .debug_macro    0 : { *(.debug_macro) }\n  .debug_names    0 : { *(.debug_names) }\n  .debug_rnglists 0 : { *(.debug_rnglists) }\n  .debug_str_offsets 0 : { *(.debug_str_offsets) }\n  .debug_sup      0 : { *(.debug_sup) }\n  .gnu.attributes 0 : { KEEP (*(.gnu.attributes)) }\n  /DISCARD/ : { *(.note.GNU-stack) *(.gnu_debuglink) *(.gnu.lto_*) }\n}\n\n\0"
            as *const u8 as *const libc::c_char as *mut libc::c_char
    } else if link_info.type_0() as libc::c_int == type_pie as libc::c_int
        && link_info.combreloc() as libc::c_int != 0
        && link_info.relro() as libc::c_int != 0
        && link_info.flags & ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_ulong
            != 0
    {
        return b"/* Script for -pie -z combreloc -z relro -z now */\n/* Copyright (C) 2014-2021 Free Software Foundation, Inc.\n   Copying and distribution of this script, with or without modification,\n   are permitted in any medium without royalty provided the copyright\n   notice and this notice are preserved.  */\nOUTPUT_FORMAT(\"elf64-x86-64\", \"elf64-x86-64\",\n\t      \"elf64-x86-64\")\nOUTPUT_ARCH(i386:x86-64)\nENTRY(_start)\nSEARCH_DIR(\"=/usr/local/x86_64-pc-linux-gnu/lib64\"); SEARCH_DIR(\"=/usr/local/lib64\"); SEARCH_DIR(\"=/lib64\"); SEARCH_DIR(\"=/usr/lib64\"); SEARCH_DIR(\"=/usr/local/x86_64-pc-linux-gnu/lib\"); SEARCH_DIR(\"=/usr/local/lib\"); SEARCH_DIR(\"=/lib\"); SEARCH_DIR(\"=/usr/lib\");\nSECTIONS\n{\n  /* Read-only sections, merged into text segment: */\n  PROVIDE (__executable_start = SEGMENT_START(\"text-segment\", 0)); . = SEGMENT_START(\"text-segment\", 0) + SIZEOF_HEADERS;\n  .interp         : { *(.interp) }\n  .note.gnu.build-id  : { *(.note.gnu.build-id) }\n  .hash           : { *(.hash) }\n  .gnu.hash       : { *(.gnu.hash) }\n  .dynsym         : { *(.dynsym) }\n  .dynstr         : { *(.dynstr) }\n  .gnu.version    : { *(.gnu.version) }\n  .gnu.version_d  : { *(.gnu.version_d) }\n  .gnu.version_r  : { *(.gnu.version_r) }\n  .rela.dyn       :\n    {\n      *(.rela.init)\n      *(.rela.text .rela.text.* .rela.gnu.linkonce.t.*)\n      *(.rela.fini)\n      *(.rela.rodata .rela.rodata.* .rela.gnu.linkonce.r.*)\n      *(.rela.data .rela.data.* .rela.gnu.linkonce.d.*)\n      *(.rela.tdata .rela.tdata.* .rela.gnu.linkonce.td.*)\n      *(.rela.tbss .rela.tbss.* .rela.gnu.linkonce.tb.*)\n      *(.rela.ctors)\n      *(.rela.dtors)\n      *(.rela.got)\n      *(.rela.bss .rela.bss.* .rela.gnu.linkonce.b.*)\n      *(.rela.ldata .rela.ldata.* .rela.gnu.linkonce.l.*)\n      *(.rela.lbss .rela.lbss.* .rela.gnu.linkonce.lb.*)\n      *(.rela.lrodata .rela.lrodata.* .rela.gnu.linkonce.lr.*)\n      *(.rela.ifunc)\n    }\n  .rela.plt       :\n    {\n      *(.rela.plt)\n      *(.rela.iplt)\n    }\n  .init           :\n  {\n    KEEP (*(SORT_NONE(.init)))\n  }\n  .plt            : { *(.plt) *(.iplt) }\n.plt.got        : { *(.plt.got) }\n.plt.sec        : { *(.plt.sec) }\n  .text           :\n  {\n    *(.text.unlikely .text.*_unlikely .text.unlikely.*)\n    *(.text.exit .text.exit.*)\n    *(.text.startup .text.startup.*)\n    *(.text.hot .text.hot.*)\n    *(SORT(.text.sorted.*))\n    *(.text .stub .text.* .gnu.linkonce.t.*)\n    /* .gnu.warning sections are handled specially by elf.em.  */\n    *(.gnu.warning)\n  }\n  .fini           :\n  {\n    KEEP (*(SORT_NONE(.fini)))\n  }\n  PROVIDE (__etext = .);\n  PROVIDE (_etext = .);\n  PROVIDE (etext = .);\n  .rodata         : { *(.rodata .rodata.* .gnu.linkonce.r.*) }\n  .rodata1        : { *(.rodata1) }\n  .eh_frame_hdr   : { *(.eh_frame_hdr) *(.eh_frame_entry .eh_frame_entry.*) }\n  .eh_frame       : ONLY_IF_RO { KEEP (*(.eh_frame)) *(.eh_frame.*) }\n  .gcc_except_table   : ONLY_IF_RO { *(.gcc_except_table .gcc_except_table.*) }\n  .gnu_extab   : ONLY_IF_RO { *(.gnu_extab*) }\n  /* These sections are generated by the Sun/Oracle C++ compiler.  */\n  .exception_ranges   : ONLY_IF_RO { *(.exception_ranges*) }\n  /* Adjust the address for the data segment.  We want to adjust up to\n     the same address within the page on the next page up.  */\n  . = DATA_SEGMENT_ALIGN (CONSTANT (MAXPAGESIZE), CONSTANT (COMMONPAGESIZE));\n  /* Exception handling  */\n  .eh_frame       : ONLY_IF_RW { KEEP (*(.eh_frame)) *(.eh_frame.*) }\n  .gnu_extab      : ONLY_IF_RW { *(.gnu_extab) }\n  .gcc_except_table   : ONLY_IF_RW { *(.gcc_except_table .gcc_except_table.*) }\n  .exception_ranges   : ONLY_IF_RW { *(.exception_ranges*) }\n  /* Thread Local Storage sections  */\n  .tdata\t  :\n   {\n     PROVIDE_HIDDEN (__tdata_start = .);\n     *(.tdata .tdata.* .gnu.linkonce.td.*)\n   }\n  .tbss\t\t  : { *(.tbss .tbss.* .gnu.linkonce.tb.*) *(.tcommon) }\n  .preinit_array    :\n  {\n    PROVIDE_HIDDEN (__preinit_array_start = .);\n    KEEP (*(.preinit_array))\n    PROVIDE_HIDDEN (__preinit_array_end = .);\n  }\n  .init_array    :\n  {\n    PROVIDE_HIDDEN (__init_array_start = .);\n    KEEP (*(SORT_BY_INIT_PRIORITY(.init_array.*) SORT_BY_INIT_PRIORITY(.ctors.*)))\n    KEEP (*(.init_array EXCLUDE_FILE (*crtbegin.o *crtbegin?.o *crtend.o *crtend?.o ) .ctors))\n    PROVIDE_HIDDEN (__init_array_end = .);\n  }\n  .fini_array    :\n  {\n    PROVIDE_HIDDEN (__fini_array_start = .);\n    KEEP (*(SORT_BY_INIT_PRIORITY(.fini_array.*) SORT_BY_INIT_PRIORITY(.dtors.*)))\n    KEEP (*(.fini_array EXCLUDE_FILE (*crtbegin.o *crtbegin?.o *crtend.o *crtend?.o ) .dtors))\n    PROVIDE_HIDDEN (__fini_array_end = .);\n  }\n  .ctors          :\n  {\n    /* gcc uses crtbegin.o to find the start of\n       the constructors, so we make sure it is\n       first.  Because this is a wildcard, it\n       doesn't matter if the user does not\n       actually link against crtbegin.o; the\n       linker won't look for a file to match a\n       wildcard.  The wildcard also means that it\n       doesn't matter which directory crtbegin.o\n       is in.  */\n    KEEP (*crtbegin.o(.ctors))\n    KEEP (*crtbegin?.o(.ctors))\n    /* We don't want to include the .ctor section from\n       the crtend.o file until after the sorted ctors.\n       The .ctor section from the crtend file contains the\n       end of ctors marker and it must be last */\n    KEEP (*(EXCLUDE_FILE (*crtend.o *crtend?.o ) .ctors))\n    KEEP (*(SORT(.ctors.*)))\n    KEEP (*(.ctors))\n  }\n  .dtors          :\n  {\n    KEEP (*crtbegin.o(.dtors))\n    KEEP (*crtbegin?.o(.dtors))\n    KEEP (*(EXCLUDE_FILE (*crtend.o *crtend?.o ) .dtors))\n    KEEP (*(SORT(.dtors.*)))\n    KEEP (*(.dtors))\n  }\n  .jcr            : { KEEP (*(.jcr)) }\n  .data.rel.ro : { *(.data.rel.ro.local* .gnu.linkonce.d.rel.ro.local.*) *(.data.rel.ro .data.rel.ro.* .gnu.linkonce.d.rel.ro.*) }\n  .dynamic        : { *(.dynamic) }\n  .got            : { *(.got.plt) *(.igot.plt) *(.got) *(.igot) }\n  . = DATA_SEGMENT_RELRO_END (0, .);\n  .data           :\n  {\n    *(.data .data.* .gnu.linkonce.d.*)\n    SORT(CONSTRUCTORS)\n  }\n  .data1          : { *(.data1) }\n  _edata = .; PROVIDE (edata = .);\n  . = .;\n  __bss_start = .;\n  .bss            :\n  {\n   *(.dynbss)\n   *(.bss .bss.* .gnu.linkonce.b.*)\n   *(COMMON)\n   /* Align here to ensure that the .bss section occupies space up to\n      _end.  Align after .bss to ensure correct alignment even if the\n      .bss section disappears because there are no input sections.\n      FIXME: Why do we need it? When there is no .bss section, we do not\n      pad the .data section.  */\n   . = ALIGN(. != 0 ? 64 / 8 : 1);\n  }\n  .lbss   :\n  {\n    *(.dynlbss)\n    *(.lbss .lbss.* .gnu.linkonce.lb.*)\n    *(LARGE_COMMON)\n  }\n  . = ALIGN(64 / 8);\n  . = SEGMENT_START(\"ldata-segment\", .);\n  .lrodata   ALIGN(CONSTANT (MAXPAGESIZE)) + (. & (CONSTANT (MAXPAGESIZE) - 1)) :\n  {\n    *(.lrodata .lrodata.* .gnu.linkonce.lr.*)\n  }\n  .ldata   ALIGN(CONSTANT (MAXPAGESIZE)) + (. & (CONSTANT (MAXPAGESIZE) - 1)) :\n  {\n    *(.ldata .ldata.* .gnu.linkonce.l.*)\n    . = ALIGN(. != 0 ? 64 / 8 : 1);\n  }\n  . = ALIGN(64 / 8);\n  _end = .; PROVIDE (end = .);\n  . = DATA_SEGMENT_END (.);\n  /* Stabs debugging sections.  */\n  .stab          0 : { *(.stab) }\n  .stabstr       0 : { *(.stabstr) }\n  .stab.excl     0 : { *(.stab.excl) }\n  .stab.exclstr  0 : { *(.stab.exclstr) }\n  .stab.index    0 : { *(.stab.index) }\n  .stab.indexstr 0 : { *(.stab.indexstr) }\n  .comment       0 : { *(.comment) }\n  .gnu.build.attributes : { *(.gnu.build.attributes .gnu.build.attributes.*) }\n  /* DWARF debug sections.\n     Symbols in the DWARF debugging sections are relative to the beginning\n     of the section so we begin them at 0.  */\n  /* DWARF 1.  */\n  .debug          0 : { *(.debug) }\n  .line           0 : { *(.line) }\n  /* GNU DWARF 1 extensions.  */\n  .debug_srcinfo  0 : { *(.debug_srcinfo) }\n  .debug_sfnames  0 : { *(.debug_sfnames) }\n  /* DWARF 1.1 and DWARF 2.  */\n  .debug_aranges  0 : { *(.debug_aranges) }\n  .debug_pubnames 0 : { *(.debug_pubnames) }\n  /* DWARF 2.  */\n  .debug_info     0 : { *(.debug_info .gnu.linkonce.wi.*) }\n  .debug_abbrev   0 : { *(.debug_abbrev) }\n  .debug_line     0 : { *(.debug_line .debug_line.* .debug_line_end) }\n  .debug_frame    0 : { *(.debug_frame) }\n  .debug_str      0 : { *(.debug_str) }\n  .debug_loc      0 : { *(.debug_loc) }\n  .debug_macinfo  0 : { *(.debug_macinfo) }\n  /* SGI/MIPS DWARF 2 extensions.  */\n  .debug_weaknames 0 : { *(.debug_weaknames) }\n  .debug_funcnames 0 : { *(.debug_funcnames) }\n  .debug_typenames 0 : { *(.debug_typenames) }\n  .debug_varnames  0 : { *(.debug_varnames) }\n  /* DWARF 3.  */\n  .debug_pubtypes 0 : { *(.debug_pubtypes) }\n  .debug_ranges   0 : { *(.debug_ranges) }\n  /* DWARF 5.  */\n  .debug_addr     0 : { *(.debug_addr) }\n  .debug_line_str 0 : { *(.debug_line_str) }\n  .debug_loclists 0 : { *(.debug_loclists) }\n  .debug_macro    0 : { *(.debug_macro) }\n  .debug_names    0 : { *(.debug_names) }\n  .debug_rnglists 0 : { *(.debug_rnglists) }\n  .debug_str_offsets 0 : { *(.debug_str_offsets) }\n  .debug_sup      0 : { *(.debug_sup) }\n  .gnu.attributes 0 : { KEEP (*(.gnu.attributes)) }\n  /DISCARD/ : { *(.note.GNU-stack) *(.gnu_debuglink) *(.gnu.lto_*) }\n}\n\n\0"
            as *const u8 as *const libc::c_char as *mut libc::c_char
    } else if link_info.type_0() as libc::c_int == type_pie as libc::c_int
        && link_info.separate_code() as libc::c_int != 0
        && link_info.combreloc() as libc::c_int != 0
    {
        return b"/* Script for -pie -z combreloc -z separate-code */\n/* Copyright (C) 2014-2021 Free Software Foundation, Inc.\n   Copying and distribution of this script, with or without modification,\n   are permitted in any medium without royalty provided the copyright\n   notice and this notice are preserved.  */\nOUTPUT_FORMAT(\"elf64-x86-64\", \"elf64-x86-64\",\n\t      \"elf64-x86-64\")\nOUTPUT_ARCH(i386:x86-64)\nENTRY(_start)\nSEARCH_DIR(\"=/usr/local/x86_64-pc-linux-gnu/lib64\"); SEARCH_DIR(\"=/usr/local/lib64\"); SEARCH_DIR(\"=/lib64\"); SEARCH_DIR(\"=/usr/lib64\"); SEARCH_DIR(\"=/usr/local/x86_64-pc-linux-gnu/lib\"); SEARCH_DIR(\"=/usr/local/lib\"); SEARCH_DIR(\"=/lib\"); SEARCH_DIR(\"=/usr/lib\");\nSECTIONS\n{\n  PROVIDE (__executable_start = SEGMENT_START(\"text-segment\", 0)); . = SEGMENT_START(\"text-segment\", 0) + SIZEOF_HEADERS;\n  .interp         : { *(.interp) }\n  .note.gnu.build-id  : { *(.note.gnu.build-id) }\n  .hash           : { *(.hash) }\n  .gnu.hash       : { *(.gnu.hash) }\n  .dynsym         : { *(.dynsym) }\n  .dynstr         : { *(.dynstr) }\n  .gnu.version    : { *(.gnu.version) }\n  .gnu.version_d  : { *(.gnu.version_d) }\n  .gnu.version_r  : { *(.gnu.version_r) }\n  .rela.dyn       :\n    {\n      *(.rela.init)\n      *(.rela.text .rela.text.* .rela.gnu.linkonce.t.*)\n      *(.rela.fini)\n      *(.rela.rodata .rela.rodata.* .rela.gnu.linkonce.r.*)\n      *(.rela.data .rela.data.* .rela.gnu.linkonce.d.*)\n      *(.rela.tdata .rela.tdata.* .rela.gnu.linkonce.td.*)\n      *(.rela.tbss .rela.tbss.* .rela.gnu.linkonce.tb.*)\n      *(.rela.ctors)\n      *(.rela.dtors)\n      *(.rela.got)\n      *(.rela.bss .rela.bss.* .rela.gnu.linkonce.b.*)\n      *(.rela.ldata .rela.ldata.* .rela.gnu.linkonce.l.*)\n      *(.rela.lbss .rela.lbss.* .rela.gnu.linkonce.lb.*)\n      *(.rela.lrodata .rela.lrodata.* .rela.gnu.linkonce.lr.*)\n      *(.rela.ifunc)\n    }\n  .rela.plt       :\n    {\n      *(.rela.plt)\n      *(.rela.iplt)\n    }\n  . = ALIGN(CONSTANT (MAXPAGESIZE));\n  .init           :\n  {\n    KEEP (*(SORT_NONE(.init)))\n  }\n  .plt            : { *(.plt) *(.iplt) }\n.plt.got        : { *(.plt.got) }\n.plt.sec        : { *(.plt.sec) }\n  .text           :\n  {\n    *(.text.unlikely .text.*_unlikely .text.unlikely.*)\n    *(.text.exit .text.exit.*)\n    *(.text.startup .text.startup.*)\n    *(.text.hot .text.hot.*)\n    *(SORT(.text.sorted.*))\n    *(.text .stub .text.* .gnu.linkonce.t.*)\n    /* .gnu.warning sections are handled specially by elf.em.  */\n    *(.gnu.warning)\n  }\n  .fini           :\n  {\n    KEEP (*(SORT_NONE(.fini)))\n  }\n  PROVIDE (__etext = .);\n  PROVIDE (_etext = .);\n  PROVIDE (etext = .);\n  . = ALIGN(CONSTANT (MAXPAGESIZE));\n  /* Adjust the address for the rodata segment.  We want to adjust up to\n     the same address within the page on the next page up.  */\n  . = SEGMENT_START(\"rodata-segment\", ALIGN(CONSTANT (MAXPAGESIZE)) + (. & (CONSTANT (MAXPAGESIZE) - 1)));\n  .rodata         : { *(.rodata .rodata.* .gnu.linkonce.r.*) }\n  .rodata1        : { *(.rodata1) }\n  .eh_frame_hdr   : { *(.eh_frame_hdr) *(.eh_frame_entry .eh_frame_entry.*) }\n  .eh_frame       : ONLY_IF_RO { KEEP (*(.eh_frame)) *(.eh_frame.*) }\n  .gcc_except_table   : ONLY_IF_RO { *(.gcc_except_table .gcc_except_table.*) }\n  .gnu_extab   : ONLY_IF_RO { *(.gnu_extab*) }\n  /* These sections are generated by the Sun/Oracle C++ compiler.  */\n  .exception_ranges   : ONLY_IF_RO { *(.exception_ranges*) }\n  /* Adjust the address for the data segment.  We want to adjust up to\n     the same address within the page on the next page up.  */\n  . = DATA_SEGMENT_ALIGN (CONSTANT (MAXPAGESIZE), CONSTANT (COMMONPAGESIZE));\n  /* Exception handling  */\n  .eh_frame       : ONLY_IF_RW { KEEP (*(.eh_frame)) *(.eh_frame.*) }\n  .gnu_extab      : ONLY_IF_RW { *(.gnu_extab) }\n  .gcc_except_table   : ONLY_IF_RW { *(.gcc_except_table .gcc_except_table.*) }\n  .exception_ranges   : ONLY_IF_RW { *(.exception_ranges*) }\n  /* Thread Local Storage sections  */\n  .tdata\t  :\n   {\n     PROVIDE_HIDDEN (__tdata_start = .);\n     *(.tdata .tdata.* .gnu.linkonce.td.*)\n   }\n  .tbss\t\t  : { *(.tbss .tbss.* .gnu.linkonce.tb.*) *(.tcommon) }\n  .preinit_array    :\n  {\n    PROVIDE_HIDDEN (__preinit_array_start = .);\n    KEEP (*(.preinit_array))\n    PROVIDE_HIDDEN (__preinit_array_end = .);\n  }\n  .init_array    :\n  {\n    PROVIDE_HIDDEN (__init_array_start = .);\n    KEEP (*(SORT_BY_INIT_PRIORITY(.init_array.*) SORT_BY_INIT_PRIORITY(.ctors.*)))\n    KEEP (*(.init_array EXCLUDE_FILE (*crtbegin.o *crtbegin?.o *crtend.o *crtend?.o ) .ctors))\n    PROVIDE_HIDDEN (__init_array_end = .);\n  }\n  .fini_array    :\n  {\n    PROVIDE_HIDDEN (__fini_array_start = .);\n    KEEP (*(SORT_BY_INIT_PRIORITY(.fini_array.*) SORT_BY_INIT_PRIORITY(.dtors.*)))\n    KEEP (*(.fini_array EXCLUDE_FILE (*crtbegin.o *crtbegin?.o *crtend.o *crtend?.o ) .dtors))\n    PROVIDE_HIDDEN (__fini_array_end = .);\n  }\n  .ctors          :\n  {\n    /* gcc uses crtbegin.o to find the start of\n       the constructors, so we make sure it is\n       first.  Because this is a wildcard, it\n       doesn't matter if the user does not\n       actually link against crtbegin.o; the\n       linker won't look for a file to match a\n       wildcard.  The wildcard also means that it\n       doesn't matter which directory crtbegin.o\n       is in.  */\n    KEEP (*crtbegin.o(.ctors))\n    KEEP (*crtbegin?.o(.ctors))\n    /* We don't want to include the .ctor section from\n       the crtend.o file until after the sorted ctors.\n       The .ctor section from the crtend file contains the\n       end of ctors marker and it must be last */\n    KEEP (*(EXCLUDE_FILE (*crtend.o *crtend?.o ) .ctors))\n    KEEP (*(SORT(.ctors.*)))\n    KEEP (*(.ctors))\n  }\n  .dtors          :\n  {\n    KEEP (*crtbegin.o(.dtors))\n    KEEP (*crtbegin?.o(.dtors))\n    KEEP (*(EXCLUDE_FILE (*crtend.o *crtend?.o ) .dtors))\n    KEEP (*(SORT(.dtors.*)))\n    KEEP (*(.dtors))\n  }\n  .jcr            : { KEEP (*(.jcr)) }\n  .data.rel.ro : { *(.data.rel.ro.local* .gnu.linkonce.d.rel.ro.local.*) *(.data.rel.ro .data.rel.ro.* .gnu.linkonce.d.rel.ro.*) }\n  .dynamic        : { *(.dynamic) }\n  .got            : { *(.got) *(.igot) }\n  . = DATA_SEGMENT_RELRO_END (SIZEOF (.got.plt) >= 24 ? 24 : 0, .);\n  .got.plt        : { *(.got.plt) *(.igot.plt) }\n  .data           :\n  {\n    *(.data .data.* .gnu.linkonce.d.*)\n    SORT(CONSTRUCTORS)\n  }\n  .data1          : { *(.data1) }\n  _edata = .; PROVIDE (edata = .);\n  . = .;\n  __bss_start = .;\n  .bss            :\n  {\n   *(.dynbss)\n   *(.bss .bss.* .gnu.linkonce.b.*)\n   *(COMMON)\n   /* Align here to ensure that the .bss section occupies space up to\n      _end.  Align after .bss to ensure correct alignment even if the\n      .bss section disappears because there are no input sections.\n      FIXME: Why do we need it? When there is no .bss section, we do not\n      pad the .data section.  */\n   . = ALIGN(. != 0 ? 64 / 8 : 1);\n  }\n  .lbss   :\n  {\n    *(.dynlbss)\n    *(.lbss .lbss.* .gnu.linkonce.lb.*)\n    *(LARGE_COMMON)\n  }\n  . = ALIGN(64 / 8);\n  . = SEGMENT_START(\"ldata-segment\", .);\n  .lrodata   ALIGN(CONSTANT (MAXPAGESIZE)) + (. & (CONSTANT (MAXPAGESIZE) - 1)) :\n  {\n    *(.lrodata .lrodata.* .gnu.linkonce.lr.*)\n  }\n  .ldata   ALIGN(CONSTANT (MAXPAGESIZE)) + (. & (CONSTANT (MAXPAGESIZE) - 1)) :\n  {\n    *(.ldata .ldata.* .gnu.linkonce.l.*)\n    . = ALIGN(. != 0 ? 64 / 8 : 1);\n  }\n  . = ALIGN(64 / 8);\n  _end = .; PROVIDE (end = .);\n  . = DATA_SEGMENT_END (.);\n  /* Stabs debugging sections.  */\n  .stab          0 : { *(.stab) }\n  .stabstr       0 : { *(.stabstr) }\n  .stab.excl     0 : { *(.stab.excl) }\n  .stab.exclstr  0 : { *(.stab.exclstr) }\n  .stab.index    0 : { *(.stab.index) }\n  .stab.indexstr 0 : { *(.stab.indexstr) }\n  .comment       0 : { *(.comment) }\n  .gnu.build.attributes : { *(.gnu.build.attributes .gnu.build.attributes.*) }\n  /* DWARF debug sections.\n     Symbols in the DWARF debugging sections are relative to the beginning\n     of the section so we begin them at 0.  */\n  /* DWARF 1.  */\n  .debug          0 : { *(.debug) }\n  .line           0 : { *(.line) }\n  /* GNU DWARF 1 extensions.  */\n  .debug_srcinfo  0 : { *(.debug_srcinfo) }\n  .debug_sfnames  0 : { *(.debug_sfnames) }\n  /* DWARF 1.1 and DWARF 2.  */\n  .debug_aranges  0 : { *(.debug_aranges) }\n  .debug_pubnames 0 : { *(.debug_pubnames) }\n  /* DWARF 2.  */\n  .debug_info     0 : { *(.debug_info .gnu.linkonce.wi.*) }\n  .debug_abbrev   0 : { *(.debug_abbrev) }\n  .debug_line     0 : { *(.debug_line .debug_line.* .debug_line_end) }\n  .debug_frame    0 : { *(.debug_frame) }\n  .debug_str      0 : { *(.debug_str) }\n  .debug_loc      0 : { *(.debug_loc) }\n  .debug_macinfo  0 : { *(.debug_macinfo) }\n  /* SGI/MIPS DWARF 2 extensions.  */\n  .debug_weaknames 0 : { *(.debug_weaknames) }\n  .debug_funcnames 0 : { *(.debug_funcnames) }\n  .debug_typenames 0 : { *(.debug_typenames) }\n  .debug_varnames  0 : { *(.debug_varnames) }\n  /* DWARF 3.  */\n  .debug_pubtypes 0 : { *(.debug_pubtypes) }\n  .debug_ranges   0 : { *(.debug_ranges) }\n  /* DWARF 5.  */\n  .debug_addr     0 : { *(.debug_addr) }\n  .debug_line_str 0 : { *(.debug_line_str) }\n  .debug_loclists 0 : { *(.debug_loclists) }\n  .debug_macro    0 : { *(.debug_macro) }\n  .debug_names    0 : { *(.debug_names) }\n  .debug_rnglists 0 : { *(.debug_rnglists) }\n  .debug_str_offsets 0 : { *(.debug_str_offsets) }\n  .debug_sup      0 : { *(.debug_sup) }\n  .gnu.attributes 0 : { KEEP (*(.gnu.attributes)) }\n  /DISCARD/ : { *(.note.GNU-stack) *(.gnu_debuglink) *(.gnu.lto_*) }\n}\n\n\0"
            as *const u8 as *const libc::c_char as *mut libc::c_char
    } else if link_info.type_0() as libc::c_int == type_pie as libc::c_int
        && link_info.combreloc() as libc::c_int != 0
    {
        return b"/* Script for -pie -z combreloc */\n/* Copyright (C) 2014-2021 Free Software Foundation, Inc.\n   Copying and distribution of this script, with or without modification,\n   are permitted in any medium without royalty provided the copyright\n   notice and this notice are preserved.  */\nOUTPUT_FORMAT(\"elf64-x86-64\", \"elf64-x86-64\",\n\t      \"elf64-x86-64\")\nOUTPUT_ARCH(i386:x86-64)\nENTRY(_start)\nSEARCH_DIR(\"=/usr/local/x86_64-pc-linux-gnu/lib64\"); SEARCH_DIR(\"=/usr/local/lib64\"); SEARCH_DIR(\"=/lib64\"); SEARCH_DIR(\"=/usr/lib64\"); SEARCH_DIR(\"=/usr/local/x86_64-pc-linux-gnu/lib\"); SEARCH_DIR(\"=/usr/local/lib\"); SEARCH_DIR(\"=/lib\"); SEARCH_DIR(\"=/usr/lib\");\nSECTIONS\n{\n  /* Read-only sections, merged into text segment: */\n  PROVIDE (__executable_start = SEGMENT_START(\"text-segment\", 0)); . = SEGMENT_START(\"text-segment\", 0) + SIZEOF_HEADERS;\n  .interp         : { *(.interp) }\n  .note.gnu.build-id  : { *(.note.gnu.build-id) }\n  .hash           : { *(.hash) }\n  .gnu.hash       : { *(.gnu.hash) }\n  .dynsym         : { *(.dynsym) }\n  .dynstr         : { *(.dynstr) }\n  .gnu.version    : { *(.gnu.version) }\n  .gnu.version_d  : { *(.gnu.version_d) }\n  .gnu.version_r  : { *(.gnu.version_r) }\n  .rela.dyn       :\n    {\n      *(.rela.init)\n      *(.rela.text .rela.text.* .rela.gnu.linkonce.t.*)\n      *(.rela.fini)\n      *(.rela.rodata .rela.rodata.* .rela.gnu.linkonce.r.*)\n      *(.rela.data .rela.data.* .rela.gnu.linkonce.d.*)\n      *(.rela.tdata .rela.tdata.* .rela.gnu.linkonce.td.*)\n      *(.rela.tbss .rela.tbss.* .rela.gnu.linkonce.tb.*)\n      *(.rela.ctors)\n      *(.rela.dtors)\n      *(.rela.got)\n      *(.rela.bss .rela.bss.* .rela.gnu.linkonce.b.*)\n      *(.rela.ldata .rela.ldata.* .rela.gnu.linkonce.l.*)\n      *(.rela.lbss .rela.lbss.* .rela.gnu.linkonce.lb.*)\n      *(.rela.lrodata .rela.lrodata.* .rela.gnu.linkonce.lr.*)\n      *(.rela.ifunc)\n    }\n  .rela.plt       :\n    {\n      *(.rela.plt)\n      *(.rela.iplt)\n    }\n  .init           :\n  {\n    KEEP (*(SORT_NONE(.init)))\n  }\n  .plt            : { *(.plt) *(.iplt) }\n.plt.got        : { *(.plt.got) }\n.plt.sec        : { *(.plt.sec) }\n  .text           :\n  {\n    *(.text.unlikely .text.*_unlikely .text.unlikely.*)\n    *(.text.exit .text.exit.*)\n    *(.text.startup .text.startup.*)\n    *(.text.hot .text.hot.*)\n    *(SORT(.text.sorted.*))\n    *(.text .stub .text.* .gnu.linkonce.t.*)\n    /* .gnu.warning sections are handled specially by elf.em.  */\n    *(.gnu.warning)\n  }\n  .fini           :\n  {\n    KEEP (*(SORT_NONE(.fini)))\n  }\n  PROVIDE (__etext = .);\n  PROVIDE (_etext = .);\n  PROVIDE (etext = .);\n  .rodata         : { *(.rodata .rodata.* .gnu.linkonce.r.*) }\n  .rodata1        : { *(.rodata1) }\n  .eh_frame_hdr   : { *(.eh_frame_hdr) *(.eh_frame_entry .eh_frame_entry.*) }\n  .eh_frame       : ONLY_IF_RO { KEEP (*(.eh_frame)) *(.eh_frame.*) }\n  .gcc_except_table   : ONLY_IF_RO { *(.gcc_except_table .gcc_except_table.*) }\n  .gnu_extab   : ONLY_IF_RO { *(.gnu_extab*) }\n  /* These sections are generated by the Sun/Oracle C++ compiler.  */\n  .exception_ranges   : ONLY_IF_RO { *(.exception_ranges*) }\n  /* Adjust the address for the data segment.  We want to adjust up to\n     the same address within the page on the next page up.  */\n  . = DATA_SEGMENT_ALIGN (CONSTANT (MAXPAGESIZE), CONSTANT (COMMONPAGESIZE));\n  /* Exception handling  */\n  .eh_frame       : ONLY_IF_RW { KEEP (*(.eh_frame)) *(.eh_frame.*) }\n  .gnu_extab      : ONLY_IF_RW { *(.gnu_extab) }\n  .gcc_except_table   : ONLY_IF_RW { *(.gcc_except_table .gcc_except_table.*) }\n  .exception_ranges   : ONLY_IF_RW { *(.exception_ranges*) }\n  /* Thread Local Storage sections  */\n  .tdata\t  :\n   {\n     PROVIDE_HIDDEN (__tdata_start = .);\n     *(.tdata .tdata.* .gnu.linkonce.td.*)\n   }\n  .tbss\t\t  : { *(.tbss .tbss.* .gnu.linkonce.tb.*) *(.tcommon) }\n  .preinit_array    :\n  {\n    PROVIDE_HIDDEN (__preinit_array_start = .);\n    KEEP (*(.preinit_array))\n    PROVIDE_HIDDEN (__preinit_array_end = .);\n  }\n  .init_array    :\n  {\n    PROVIDE_HIDDEN (__init_array_start = .);\n    KEEP (*(SORT_BY_INIT_PRIORITY(.init_array.*) SORT_BY_INIT_PRIORITY(.ctors.*)))\n    KEEP (*(.init_array EXCLUDE_FILE (*crtbegin.o *crtbegin?.o *crtend.o *crtend?.o ) .ctors))\n    PROVIDE_HIDDEN (__init_array_end = .);\n  }\n  .fini_array    :\n  {\n    PROVIDE_HIDDEN (__fini_array_start = .);\n    KEEP (*(SORT_BY_INIT_PRIORITY(.fini_array.*) SORT_BY_INIT_PRIORITY(.dtors.*)))\n    KEEP (*(.fini_array EXCLUDE_FILE (*crtbegin.o *crtbegin?.o *crtend.o *crtend?.o ) .dtors))\n    PROVIDE_HIDDEN (__fini_array_end = .);\n  }\n  .ctors          :\n  {\n    /* gcc uses crtbegin.o to find the start of\n       the constructors, so we make sure it is\n       first.  Because this is a wildcard, it\n       doesn't matter if the user does not\n       actually link against crtbegin.o; the\n       linker won't look for a file to match a\n       wildcard.  The wildcard also means that it\n       doesn't matter which directory crtbegin.o\n       is in.  */\n    KEEP (*crtbegin.o(.ctors))\n    KEEP (*crtbegin?.o(.ctors))\n    /* We don't want to include the .ctor section from\n       the crtend.o file until after the sorted ctors.\n       The .ctor section from the crtend file contains the\n       end of ctors marker and it must be last */\n    KEEP (*(EXCLUDE_FILE (*crtend.o *crtend?.o ) .ctors))\n    KEEP (*(SORT(.ctors.*)))\n    KEEP (*(.ctors))\n  }\n  .dtors          :\n  {\n    KEEP (*crtbegin.o(.dtors))\n    KEEP (*crtbegin?.o(.dtors))\n    KEEP (*(EXCLUDE_FILE (*crtend.o *crtend?.o ) .dtors))\n    KEEP (*(SORT(.dtors.*)))\n    KEEP (*(.dtors))\n  }\n  .jcr            : { KEEP (*(.jcr)) }\n  .data.rel.ro : { *(.data.rel.ro.local* .gnu.linkonce.d.rel.ro.local.*) *(.data.rel.ro .data.rel.ro.* .gnu.linkonce.d.rel.ro.*) }\n  .dynamic        : { *(.dynamic) }\n  .got            : { *(.got) *(.igot) }\n  . = DATA_SEGMENT_RELRO_END (SIZEOF (.got.plt) >= 24 ? 24 : 0, .);\n  .got.plt        : { *(.got.plt) *(.igot.plt) }\n  .data           :\n  {\n    *(.data .data.* .gnu.linkonce.d.*)\n    SORT(CONSTRUCTORS)\n  }\n  .data1          : { *(.data1) }\n  _edata = .; PROVIDE (edata = .);\n  . = .;\n  __bss_start = .;\n  .bss            :\n  {\n   *(.dynbss)\n   *(.bss .bss.* .gnu.linkonce.b.*)\n   *(COMMON)\n   /* Align here to ensure that the .bss section occupies space up to\n      _end.  Align after .bss to ensure correct alignment even if the\n      .bss section disappears because there are no input sections.\n      FIXME: Why do we need it? When there is no .bss section, we do not\n      pad the .data section.  */\n   . = ALIGN(. != 0 ? 64 / 8 : 1);\n  }\n  .lbss   :\n  {\n    *(.dynlbss)\n    *(.lbss .lbss.* .gnu.linkonce.lb.*)\n    *(LARGE_COMMON)\n  }\n  . = ALIGN(64 / 8);\n  . = SEGMENT_START(\"ldata-segment\", .);\n  .lrodata   ALIGN(CONSTANT (MAXPAGESIZE)) + (. & (CONSTANT (MAXPAGESIZE) - 1)) :\n  {\n    *(.lrodata .lrodata.* .gnu.linkonce.lr.*)\n  }\n  .ldata   ALIGN(CONSTANT (MAXPAGESIZE)) + (. & (CONSTANT (MAXPAGESIZE) - 1)) :\n  {\n    *(.ldata .ldata.* .gnu.linkonce.l.*)\n    . = ALIGN(. != 0 ? 64 / 8 : 1);\n  }\n  . = ALIGN(64 / 8);\n  _end = .; PROVIDE (end = .);\n  . = DATA_SEGMENT_END (.);\n  /* Stabs debugging sections.  */\n  .stab          0 : { *(.stab) }\n  .stabstr       0 : { *(.stabstr) }\n  .stab.excl     0 : { *(.stab.excl) }\n  .stab.exclstr  0 : { *(.stab.exclstr) }\n  .stab.index    0 : { *(.stab.index) }\n  .stab.indexstr 0 : { *(.stab.indexstr) }\n  .comment       0 : { *(.comment) }\n  .gnu.build.attributes : { *(.gnu.build.attributes .gnu.build.attributes.*) }\n  /* DWARF debug sections.\n     Symbols in the DWARF debugging sections are relative to the beginning\n     of the section so we begin them at 0.  */\n  /* DWARF 1.  */\n  .debug          0 : { *(.debug) }\n  .line           0 : { *(.line) }\n  /* GNU DWARF 1 extensions.  */\n  .debug_srcinfo  0 : { *(.debug_srcinfo) }\n  .debug_sfnames  0 : { *(.debug_sfnames) }\n  /* DWARF 1.1 and DWARF 2.  */\n  .debug_aranges  0 : { *(.debug_aranges) }\n  .debug_pubnames 0 : { *(.debug_pubnames) }\n  /* DWARF 2.  */\n  .debug_info     0 : { *(.debug_info .gnu.linkonce.wi.*) }\n  .debug_abbrev   0 : { *(.debug_abbrev) }\n  .debug_line     0 : { *(.debug_line .debug_line.* .debug_line_end) }\n  .debug_frame    0 : { *(.debug_frame) }\n  .debug_str      0 : { *(.debug_str) }\n  .debug_loc      0 : { *(.debug_loc) }\n  .debug_macinfo  0 : { *(.debug_macinfo) }\n  /* SGI/MIPS DWARF 2 extensions.  */\n  .debug_weaknames 0 : { *(.debug_weaknames) }\n  .debug_funcnames 0 : { *(.debug_funcnames) }\n  .debug_typenames 0 : { *(.debug_typenames) }\n  .debug_varnames  0 : { *(.debug_varnames) }\n  /* DWARF 3.  */\n  .debug_pubtypes 0 : { *(.debug_pubtypes) }\n  .debug_ranges   0 : { *(.debug_ranges) }\n  /* DWARF 5.  */\n  .debug_addr     0 : { *(.debug_addr) }\n  .debug_line_str 0 : { *(.debug_line_str) }\n  .debug_loclists 0 : { *(.debug_loclists) }\n  .debug_macro    0 : { *(.debug_macro) }\n  .debug_names    0 : { *(.debug_names) }\n  .debug_rnglists 0 : { *(.debug_rnglists) }\n  .debug_str_offsets 0 : { *(.debug_str_offsets) }\n  .debug_sup      0 : { *(.debug_sup) }\n  .gnu.attributes 0 : { KEEP (*(.gnu.attributes)) }\n  /DISCARD/ : { *(.note.GNU-stack) *(.gnu_debuglink) *(.gnu.lto_*) }\n}\n\n\0"
            as *const u8 as *const libc::c_char as *mut libc::c_char
    } else if link_info.type_0() as libc::c_int == type_pie as libc::c_int
        && link_info.separate_code() as libc::c_int != 0
    {
        return b"/* Script for -pie -z separate-code */\n/* Copyright (C) 2014-2021 Free Software Foundation, Inc.\n   Copying and distribution of this script, with or without modification,\n   are permitted in any medium without royalty provided the copyright\n   notice and this notice are preserved.  */\nOUTPUT_FORMAT(\"elf64-x86-64\", \"elf64-x86-64\",\n\t      \"elf64-x86-64\")\nOUTPUT_ARCH(i386:x86-64)\nENTRY(_start)\nSEARCH_DIR(\"=/usr/local/x86_64-pc-linux-gnu/lib64\"); SEARCH_DIR(\"=/usr/local/lib64\"); SEARCH_DIR(\"=/lib64\"); SEARCH_DIR(\"=/usr/lib64\"); SEARCH_DIR(\"=/usr/local/x86_64-pc-linux-gnu/lib\"); SEARCH_DIR(\"=/usr/local/lib\"); SEARCH_DIR(\"=/lib\"); SEARCH_DIR(\"=/usr/lib\");\nSECTIONS\n{\n  PROVIDE (__executable_start = SEGMENT_START(\"text-segment\", 0)); . = SEGMENT_START(\"text-segment\", 0) + SIZEOF_HEADERS;\n  .interp         : { *(.interp) }\n  .note.gnu.build-id  : { *(.note.gnu.build-id) }\n  .hash           : { *(.hash) }\n  .gnu.hash       : { *(.gnu.hash) }\n  .dynsym         : { *(.dynsym) }\n  .dynstr         : { *(.dynstr) }\n  .gnu.version    : { *(.gnu.version) }\n  .gnu.version_d  : { *(.gnu.version_d) }\n  .gnu.version_r  : { *(.gnu.version_r) }\n  .rela.init      : { *(.rela.init) }\n  .rela.text      : { *(.rela.text .rela.text.* .rela.gnu.linkonce.t.*) }\n  .rela.fini      : { *(.rela.fini) }\n  .rela.rodata    : { *(.rela.rodata .rela.rodata.* .rela.gnu.linkonce.r.*) }\n  .rela.data.rel.ro   : { *(.rela.data.rel.ro .rela.data.rel.ro.* .rela.gnu.linkonce.d.rel.ro.*) }\n  .rela.data      : { *(.rela.data .rela.data.* .rela.gnu.linkonce.d.*) }\n  .rela.tdata\t  : { *(.rela.tdata .rela.tdata.* .rela.gnu.linkonce.td.*) }\n  .rela.tbss\t  : { *(.rela.tbss .rela.tbss.* .rela.gnu.linkonce.tb.*) }\n  .rela.ctors     : { *(.rela.ctors) }\n  .rela.dtors     : { *(.rela.dtors) }\n  .rela.got       : { *(.rela.got) }\n  .rela.bss       : { *(.rela.bss .rela.bss.* .rela.gnu.linkonce.b.*) }\n  .rela.ldata     : { *(.rela.ldata .rela.ldata.* .rela.gnu.linkonce.l.*) }\n  .rela.lbss      : { *(.rela.lbss .rela.lbss.* .rela.gnu.linkonce.lb.*) }\n  .rela.lrodata   : { *(.rela.lrodata .rela.lrodata.* .rela.gnu.linkonce.lr.*) }\n  .rela.ifunc     : { *(.rela.ifunc) }\n  .rela.plt       :\n    {\n      *(.rela.plt)\n      *(.rela.iplt)\n    }\n  . = ALIGN(CONSTANT (MAXPAGESIZE));\n  .init           :\n  {\n    KEEP (*(SORT_NONE(.init)))\n  }\n  .plt            : { *(.plt) *(.iplt) }\n.plt.got        : { *(.plt.got) }\n.plt.sec        : { *(.plt.sec) }\n  .text           :\n  {\n    *(.text.unlikely .text.*_unlikely .text.unlikely.*)\n    *(.text.exit .text.exit.*)\n    *(.text.startup .text.startup.*)\n    *(.text.hot .text.hot.*)\n    *(SORT(.text.sorted.*))\n    *(.text .stub .text.* .gnu.linkonce.t.*)\n    /* .gnu.warning sections are handled specially by elf.em.  */\n    *(.gnu.warning)\n  }\n  .fini           :\n  {\n    KEEP (*(SORT_NONE(.fini)))\n  }\n  PROVIDE (__etext = .);\n  PROVIDE (_etext = .);\n  PROVIDE (etext = .);\n  . = ALIGN(CONSTANT (MAXPAGESIZE));\n  /* Adjust the address for the rodata segment.  We want to adjust up to\n     the same address within the page on the next page up.  */\n  . = SEGMENT_START(\"rodata-segment\", ALIGN(CONSTANT (MAXPAGESIZE)) + (. & (CONSTANT (MAXPAGESIZE) - 1)));\n  .rodata         : { *(.rodata .rodata.* .gnu.linkonce.r.*) }\n  .rodata1        : { *(.rodata1) }\n  .eh_frame_hdr   : { *(.eh_frame_hdr) *(.eh_frame_entry .eh_frame_entry.*) }\n  .eh_frame       : ONLY_IF_RO { KEEP (*(.eh_frame)) *(.eh_frame.*) }\n  .gcc_except_table   : ONLY_IF_RO { *(.gcc_except_table .gcc_except_table.*) }\n  .gnu_extab   : ONLY_IF_RO { *(.gnu_extab*) }\n  /* These sections are generated by the Sun/Oracle C++ compiler.  */\n  .exception_ranges   : ONLY_IF_RO { *(.exception_ranges*) }\n  /* Adjust the address for the data segment.  We want to adjust up to\n     the same address within the page on the next page up.  */\n  . = DATA_SEGMENT_ALIGN (CONSTANT (MAXPAGESIZE), CONSTANT (COMMONPAGESIZE));\n  /* Exception handling  */\n  .eh_frame       : ONLY_IF_RW { KEEP (*(.eh_frame)) *(.eh_frame.*) }\n  .gnu_extab      : ONLY_IF_RW { *(.gnu_extab) }\n  .gcc_except_table   : ONLY_IF_RW { *(.gcc_except_table .gcc_except_table.*) }\n  .exception_ranges   : ONLY_IF_RW { *(.exception_ranges*) }\n  /* Thread Local Storage sections  */\n  .tdata\t  :\n   {\n     PROVIDE_HIDDEN (__tdata_start = .);\n     *(.tdata .tdata.* .gnu.linkonce.td.*)\n   }\n  .tbss\t\t  : { *(.tbss .tbss.* .gnu.linkonce.tb.*) *(.tcommon) }\n  .preinit_array    :\n  {\n    PROVIDE_HIDDEN (__preinit_array_start = .);\n    KEEP (*(.preinit_array))\n    PROVIDE_HIDDEN (__preinit_array_end = .);\n  }\n  .init_array    :\n  {\n    PROVIDE_HIDDEN (__init_array_start = .);\n    KEEP (*(SORT_BY_INIT_PRIORITY(.init_array.*) SORT_BY_INIT_PRIORITY(.ctors.*)))\n    KEEP (*(.init_array EXCLUDE_FILE (*crtbegin.o *crtbegin?.o *crtend.o *crtend?.o ) .ctors))\n    PROVIDE_HIDDEN (__init_array_end = .);\n  }\n  .fini_array    :\n  {\n    PROVIDE_HIDDEN (__fini_array_start = .);\n    KEEP (*(SORT_BY_INIT_PRIORITY(.fini_array.*) SORT_BY_INIT_PRIORITY(.dtors.*)))\n    KEEP (*(.fini_array EXCLUDE_FILE (*crtbegin.o *crtbegin?.o *crtend.o *crtend?.o ) .dtors))\n    PROVIDE_HIDDEN (__fini_array_end = .);\n  }\n  .ctors          :\n  {\n    /* gcc uses crtbegin.o to find the start of\n       the constructors, so we make sure it is\n       first.  Because this is a wildcard, it\n       doesn't matter if the user does not\n       actually link against crtbegin.o; the\n       linker won't look for a file to match a\n       wildcard.  The wildcard also means that it\n       doesn't matter which directory crtbegin.o\n       is in.  */\n    KEEP (*crtbegin.o(.ctors))\n    KEEP (*crtbegin?.o(.ctors))\n    /* We don't want to include the .ctor section from\n       the crtend.o file until after the sorted ctors.\n       The .ctor section from the crtend file contains the\n       end of ctors marker and it must be last */\n    KEEP (*(EXCLUDE_FILE (*crtend.o *crtend?.o ) .ctors))\n    KEEP (*(SORT(.ctors.*)))\n    KEEP (*(.ctors))\n  }\n  .dtors          :\n  {\n    KEEP (*crtbegin.o(.dtors))\n    KEEP (*crtbegin?.o(.dtors))\n    KEEP (*(EXCLUDE_FILE (*crtend.o *crtend?.o ) .dtors))\n    KEEP (*(SORT(.dtors.*)))\n    KEEP (*(.dtors))\n  }\n  .jcr            : { KEEP (*(.jcr)) }\n  .data.rel.ro : { *(.data.rel.ro.local* .gnu.linkonce.d.rel.ro.local.*) *(.data.rel.ro .data.rel.ro.* .gnu.linkonce.d.rel.ro.*) }\n  .dynamic        : { *(.dynamic) }\n  .got            : { *(.got) *(.igot) }\n  . = DATA_SEGMENT_RELRO_END (SIZEOF (.got.plt) >= 24 ? 24 : 0, .);\n  .got.plt        : { *(.got.plt) *(.igot.plt) }\n  .data           :\n  {\n    *(.data .data.* .gnu.linkonce.d.*)\n    SORT(CONSTRUCTORS)\n  }\n  .data1          : { *(.data1) }\n  _edata = .; PROVIDE (edata = .);\n  . = .;\n  __bss_start = .;\n  .bss            :\n  {\n   *(.dynbss)\n   *(.bss .bss.* .gnu.linkonce.b.*)\n   *(COMMON)\n   /* Align here to ensure that the .bss section occupies space up to\n      _end.  Align after .bss to ensure correct alignment even if the\n      .bss section disappears because there are no input sections.\n      FIXME: Why do we need it? When there is no .bss section, we do not\n      pad the .data section.  */\n   . = ALIGN(. != 0 ? 64 / 8 : 1);\n  }\n  .lbss   :\n  {\n    *(.dynlbss)\n    *(.lbss .lbss.* .gnu.linkonce.lb.*)\n    *(LARGE_COMMON)\n  }\n  . = ALIGN(64 / 8);\n  . = SEGMENT_START(\"ldata-segment\", .);\n  .lrodata   ALIGN(CONSTANT (MAXPAGESIZE)) + (. & (CONSTANT (MAXPAGESIZE) - 1)) :\n  {\n    *(.lrodata .lrodata.* .gnu.linkonce.lr.*)\n  }\n  .ldata   ALIGN(CONSTANT (MAXPAGESIZE)) + (. & (CONSTANT (MAXPAGESIZE) - 1)) :\n  {\n    *(.ldata .ldata.* .gnu.linkonce.l.*)\n    . = ALIGN(. != 0 ? 64 / 8 : 1);\n  }\n  . = ALIGN(64 / 8);\n  _end = .; PROVIDE (end = .);\n  . = DATA_SEGMENT_END (.);\n  /* Stabs debugging sections.  */\n  .stab          0 : { *(.stab) }\n  .stabstr       0 : { *(.stabstr) }\n  .stab.excl     0 : { *(.stab.excl) }\n  .stab.exclstr  0 : { *(.stab.exclstr) }\n  .stab.index    0 : { *(.stab.index) }\n  .stab.indexstr 0 : { *(.stab.indexstr) }\n  .comment       0 : { *(.comment) }\n  .gnu.build.attributes : { *(.gnu.build.attributes .gnu.build.attributes.*) }\n  /* DWARF debug sections.\n     Symbols in the DWARF debugging sections are relative to the beginning\n     of the section so we begin them at 0.  */\n  /* DWARF 1.  */\n  .debug          0 : { *(.debug) }\n  .line           0 : { *(.line) }\n  /* GNU DWARF 1 extensions.  */\n  .debug_srcinfo  0 : { *(.debug_srcinfo) }\n  .debug_sfnames  0 : { *(.debug_sfnames) }\n  /* DWARF 1.1 and DWARF 2.  */\n  .debug_aranges  0 : { *(.debug_aranges) }\n  .debug_pubnames 0 : { *(.debug_pubnames) }\n  /* DWARF 2.  */\n  .debug_info     0 : { *(.debug_info .gnu.linkonce.wi.*) }\n  .debug_abbrev   0 : { *(.debug_abbrev) }\n  .debug_line     0 : { *(.debug_line .debug_line.* .debug_line_end) }\n  .debug_frame    0 : { *(.debug_frame) }\n  .debug_str      0 : { *(.debug_str) }\n  .debug_loc      0 : { *(.debug_loc) }\n  .debug_macinfo  0 : { *(.debug_macinfo) }\n  /* SGI/MIPS DWARF 2 extensions.  */\n  .debug_weaknames 0 : { *(.debug_weaknames) }\n  .debug_funcnames 0 : { *(.debug_funcnames) }\n  .debug_typenames 0 : { *(.debug_typenames) }\n  .debug_varnames  0 : { *(.debug_varnames) }\n  /* DWARF 3.  */\n  .debug_pubtypes 0 : { *(.debug_pubtypes) }\n  .debug_ranges   0 : { *(.debug_ranges) }\n  /* DWARF 5.  */\n  .debug_addr     0 : { *(.debug_addr) }\n  .debug_line_str 0 : { *(.debug_line_str) }\n  .debug_loclists 0 : { *(.debug_loclists) }\n  .debug_macro    0 : { *(.debug_macro) }\n  .debug_names    0 : { *(.debug_names) }\n  .debug_rnglists 0 : { *(.debug_rnglists) }\n  .debug_str_offsets 0 : { *(.debug_str_offsets) }\n  .debug_sup      0 : { *(.debug_sup) }\n  .gnu.attributes 0 : { KEEP (*(.gnu.attributes)) }\n  /DISCARD/ : { *(.note.GNU-stack) *(.gnu_debuglink) *(.gnu.lto_*) }\n}\n\n\0"
            as *const u8 as *const libc::c_char as *mut libc::c_char
    } else if link_info.type_0() as libc::c_int == type_pie as libc::c_int {
        return b"/* Script for -pie */\n/* Copyright (C) 2014-2021 Free Software Foundation, Inc.\n   Copying and distribution of this script, with or without modification,\n   are permitted in any medium without royalty provided the copyright\n   notice and this notice are preserved.  */\nOUTPUT_FORMAT(\"elf64-x86-64\", \"elf64-x86-64\",\n\t      \"elf64-x86-64\")\nOUTPUT_ARCH(i386:x86-64)\nENTRY(_start)\nSEARCH_DIR(\"=/usr/local/x86_64-pc-linux-gnu/lib64\"); SEARCH_DIR(\"=/usr/local/lib64\"); SEARCH_DIR(\"=/lib64\"); SEARCH_DIR(\"=/usr/lib64\"); SEARCH_DIR(\"=/usr/local/x86_64-pc-linux-gnu/lib\"); SEARCH_DIR(\"=/usr/local/lib\"); SEARCH_DIR(\"=/lib\"); SEARCH_DIR(\"=/usr/lib\");\nSECTIONS\n{\n  /* Read-only sections, merged into text segment: */\n  PROVIDE (__executable_start = SEGMENT_START(\"text-segment\", 0)); . = SEGMENT_START(\"text-segment\", 0) + SIZEOF_HEADERS;\n  .interp         : { *(.interp) }\n  .note.gnu.build-id  : { *(.note.gnu.build-id) }\n  .hash           : { *(.hash) }\n  .gnu.hash       : { *(.gnu.hash) }\n  .dynsym         : { *(.dynsym) }\n  .dynstr         : { *(.dynstr) }\n  .gnu.version    : { *(.gnu.version) }\n  .gnu.version_d  : { *(.gnu.version_d) }\n  .gnu.version_r  : { *(.gnu.version_r) }\n  .rela.init      : { *(.rela.init) }\n  .rela.text      : { *(.rela.text .rela.text.* .rela.gnu.linkonce.t.*) }\n  .rela.fini      : { *(.rela.fini) }\n  .rela.rodata    : { *(.rela.rodata .rela.rodata.* .rela.gnu.linkonce.r.*) }\n  .rela.data.rel.ro   : { *(.rela.data.rel.ro .rela.data.rel.ro.* .rela.gnu.linkonce.d.rel.ro.*) }\n  .rela.data      : { *(.rela.data .rela.data.* .rela.gnu.linkonce.d.*) }\n  .rela.tdata\t  : { *(.rela.tdata .rela.tdata.* .rela.gnu.linkonce.td.*) }\n  .rela.tbss\t  : { *(.rela.tbss .rela.tbss.* .rela.gnu.linkonce.tb.*) }\n  .rela.ctors     : { *(.rela.ctors) }\n  .rela.dtors     : { *(.rela.dtors) }\n  .rela.got       : { *(.rela.got) }\n  .rela.bss       : { *(.rela.bss .rela.bss.* .rela.gnu.linkonce.b.*) }\n  .rela.ldata     : { *(.rela.ldata .rela.ldata.* .rela.gnu.linkonce.l.*) }\n  .rela.lbss      : { *(.rela.lbss .rela.lbss.* .rela.gnu.linkonce.lb.*) }\n  .rela.lrodata   : { *(.rela.lrodata .rela.lrodata.* .rela.gnu.linkonce.lr.*) }\n  .rela.ifunc     : { *(.rela.ifunc) }\n  .rela.plt       :\n    {\n      *(.rela.plt)\n      *(.rela.iplt)\n    }\n  .init           :\n  {\n    KEEP (*(SORT_NONE(.init)))\n  }\n  .plt            : { *(.plt) *(.iplt) }\n.plt.got        : { *(.plt.got) }\n.plt.sec        : { *(.plt.sec) }\n  .text           :\n  {\n    *(.text.unlikely .text.*_unlikely .text.unlikely.*)\n    *(.text.exit .text.exit.*)\n    *(.text.startup .text.startup.*)\n    *(.text.hot .text.hot.*)\n    *(SORT(.text.sorted.*))\n    *(.text .stub .text.* .gnu.linkonce.t.*)\n    /* .gnu.warning sections are handled specially by elf.em.  */\n    *(.gnu.warning)\n  }\n  .fini           :\n  {\n    KEEP (*(SORT_NONE(.fini)))\n  }\n  PROVIDE (__etext = .);\n  PROVIDE (_etext = .);\n  PROVIDE (etext = .);\n  .rodata         : { *(.rodata .rodata.* .gnu.linkonce.r.*) }\n  .rodata1        : { *(.rodata1) }\n  .eh_frame_hdr   : { *(.eh_frame_hdr) *(.eh_frame_entry .eh_frame_entry.*) }\n  .eh_frame       : ONLY_IF_RO { KEEP (*(.eh_frame)) *(.eh_frame.*) }\n  .gcc_except_table   : ONLY_IF_RO { *(.gcc_except_table .gcc_except_table.*) }\n  .gnu_extab   : ONLY_IF_RO { *(.gnu_extab*) }\n  /* These sections are generated by the Sun/Oracle C++ compiler.  */\n  .exception_ranges   : ONLY_IF_RO { *(.exception_ranges*) }\n  /* Adjust the address for the data segment.  We want to adjust up to\n     the same address within the page on the next page up.  */\n  . = DATA_SEGMENT_ALIGN (CONSTANT (MAXPAGESIZE), CONSTANT (COMMONPAGESIZE));\n  /* Exception handling  */\n  .eh_frame       : ONLY_IF_RW { KEEP (*(.eh_frame)) *(.eh_frame.*) }\n  .gnu_extab      : ONLY_IF_RW { *(.gnu_extab) }\n  .gcc_except_table   : ONLY_IF_RW { *(.gcc_except_table .gcc_except_table.*) }\n  .exception_ranges   : ONLY_IF_RW { *(.exception_ranges*) }\n  /* Thread Local Storage sections  */\n  .tdata\t  :\n   {\n     PROVIDE_HIDDEN (__tdata_start = .);\n     *(.tdata .tdata.* .gnu.linkonce.td.*)\n   }\n  .tbss\t\t  : { *(.tbss .tbss.* .gnu.linkonce.tb.*) *(.tcommon) }\n  .preinit_array    :\n  {\n    PROVIDE_HIDDEN (__preinit_array_start = .);\n    KEEP (*(.preinit_array))\n    PROVIDE_HIDDEN (__preinit_array_end = .);\n  }\n  .init_array    :\n  {\n    PROVIDE_HIDDEN (__init_array_start = .);\n    KEEP (*(SORT_BY_INIT_PRIORITY(.init_array.*) SORT_BY_INIT_PRIORITY(.ctors.*)))\n    KEEP (*(.init_array EXCLUDE_FILE (*crtbegin.o *crtbegin?.o *crtend.o *crtend?.o ) .ctors))\n    PROVIDE_HIDDEN (__init_array_end = .);\n  }\n  .fini_array    :\n  {\n    PROVIDE_HIDDEN (__fini_array_start = .);\n    KEEP (*(SORT_BY_INIT_PRIORITY(.fini_array.*) SORT_BY_INIT_PRIORITY(.dtors.*)))\n    KEEP (*(.fini_array EXCLUDE_FILE (*crtbegin.o *crtbegin?.o *crtend.o *crtend?.o ) .dtors))\n    PROVIDE_HIDDEN (__fini_array_end = .);\n  }\n  .ctors          :\n  {\n    /* gcc uses crtbegin.o to find the start of\n       the constructors, so we make sure it is\n       first.  Because this is a wildcard, it\n       doesn't matter if the user does not\n       actually link against crtbegin.o; the\n       linker won't look for a file to match a\n       wildcard.  The wildcard also means that it\n       doesn't matter which directory crtbegin.o\n       is in.  */\n    KEEP (*crtbegin.o(.ctors))\n    KEEP (*crtbegin?.o(.ctors))\n    /* We don't want to include the .ctor section from\n       the crtend.o file until after the sorted ctors.\n       The .ctor section from the crtend file contains the\n       end of ctors marker and it must be last */\n    KEEP (*(EXCLUDE_FILE (*crtend.o *crtend?.o ) .ctors))\n    KEEP (*(SORT(.ctors.*)))\n    KEEP (*(.ctors))\n  }\n  .dtors          :\n  {\n    KEEP (*crtbegin.o(.dtors))\n    KEEP (*crtbegin?.o(.dtors))\n    KEEP (*(EXCLUDE_FILE (*crtend.o *crtend?.o ) .dtors))\n    KEEP (*(SORT(.dtors.*)))\n    KEEP (*(.dtors))\n  }\n  .jcr            : { KEEP (*(.jcr)) }\n  .data.rel.ro : { *(.data.rel.ro.local* .gnu.linkonce.d.rel.ro.local.*) *(.data.rel.ro .data.rel.ro.* .gnu.linkonce.d.rel.ro.*) }\n  .dynamic        : { *(.dynamic) }\n  .got            : { *(.got) *(.igot) }\n  . = DATA_SEGMENT_RELRO_END (SIZEOF (.got.plt) >= 24 ? 24 : 0, .);\n  .got.plt        : { *(.got.plt) *(.igot.plt) }\n  .data           :\n  {\n    *(.data .data.* .gnu.linkonce.d.*)\n    SORT(CONSTRUCTORS)\n  }\n  .data1          : { *(.data1) }\n  _edata = .; PROVIDE (edata = .);\n  . = .;\n  __bss_start = .;\n  .bss            :\n  {\n   *(.dynbss)\n   *(.bss .bss.* .gnu.linkonce.b.*)\n   *(COMMON)\n   /* Align here to ensure that the .bss section occupies space up to\n      _end.  Align after .bss to ensure correct alignment even if the\n      .bss section disappears because there are no input sections.\n      FIXME: Why do we need it? When there is no .bss section, we do not\n      pad the .data section.  */\n   . = ALIGN(. != 0 ? 64 / 8 : 1);\n  }\n  .lbss   :\n  {\n    *(.dynlbss)\n    *(.lbss .lbss.* .gnu.linkonce.lb.*)\n    *(LARGE_COMMON)\n  }\n  . = ALIGN(64 / 8);\n  . = SEGMENT_START(\"ldata-segment\", .);\n  .lrodata   ALIGN(CONSTANT (MAXPAGESIZE)) + (. & (CONSTANT (MAXPAGESIZE) - 1)) :\n  {\n    *(.lrodata .lrodata.* .gnu.linkonce.lr.*)\n  }\n  .ldata   ALIGN(CONSTANT (MAXPAGESIZE)) + (. & (CONSTANT (MAXPAGESIZE) - 1)) :\n  {\n    *(.ldata .ldata.* .gnu.linkonce.l.*)\n    . = ALIGN(. != 0 ? 64 / 8 : 1);\n  }\n  . = ALIGN(64 / 8);\n  _end = .; PROVIDE (end = .);\n  . = DATA_SEGMENT_END (.);\n  /* Stabs debugging sections.  */\n  .stab          0 : { *(.stab) }\n  .stabstr       0 : { *(.stabstr) }\n  .stab.excl     0 : { *(.stab.excl) }\n  .stab.exclstr  0 : { *(.stab.exclstr) }\n  .stab.index    0 : { *(.stab.index) }\n  .stab.indexstr 0 : { *(.stab.indexstr) }\n  .comment       0 : { *(.comment) }\n  .gnu.build.attributes : { *(.gnu.build.attributes .gnu.build.attributes.*) }\n  /* DWARF debug sections.\n     Symbols in the DWARF debugging sections are relative to the beginning\n     of the section so we begin them at 0.  */\n  /* DWARF 1.  */\n  .debug          0 : { *(.debug) }\n  .line           0 : { *(.line) }\n  /* GNU DWARF 1 extensions.  */\n  .debug_srcinfo  0 : { *(.debug_srcinfo) }\n  .debug_sfnames  0 : { *(.debug_sfnames) }\n  /* DWARF 1.1 and DWARF 2.  */\n  .debug_aranges  0 : { *(.debug_aranges) }\n  .debug_pubnames 0 : { *(.debug_pubnames) }\n  /* DWARF 2.  */\n  .debug_info     0 : { *(.debug_info .gnu.linkonce.wi.*) }\n  .debug_abbrev   0 : { *(.debug_abbrev) }\n  .debug_line     0 : { *(.debug_line .debug_line.* .debug_line_end) }\n  .debug_frame    0 : { *(.debug_frame) }\n  .debug_str      0 : { *(.debug_str) }\n  .debug_loc      0 : { *(.debug_loc) }\n  .debug_macinfo  0 : { *(.debug_macinfo) }\n  /* SGI/MIPS DWARF 2 extensions.  */\n  .debug_weaknames 0 : { *(.debug_weaknames) }\n  .debug_funcnames 0 : { *(.debug_funcnames) }\n  .debug_typenames 0 : { *(.debug_typenames) }\n  .debug_varnames  0 : { *(.debug_varnames) }\n  /* DWARF 3.  */\n  .debug_pubtypes 0 : { *(.debug_pubtypes) }\n  .debug_ranges   0 : { *(.debug_ranges) }\n  /* DWARF 5.  */\n  .debug_addr     0 : { *(.debug_addr) }\n  .debug_line_str 0 : { *(.debug_line_str) }\n  .debug_loclists 0 : { *(.debug_loclists) }\n  .debug_macro    0 : { *(.debug_macro) }\n  .debug_names    0 : { *(.debug_names) }\n  .debug_rnglists 0 : { *(.debug_rnglists) }\n  .debug_str_offsets 0 : { *(.debug_str_offsets) }\n  .debug_sup      0 : { *(.debug_sup) }\n  .gnu.attributes 0 : { KEEP (*(.gnu.attributes)) }\n  /DISCARD/ : { *(.note.GNU-stack) *(.gnu_debuglink) *(.gnu.lto_*) }\n}\n\n\0"
            as *const u8 as *const libc::c_char as *mut libc::c_char
    } else if link_info.type_0() as libc::c_int == type_dll as libc::c_int
        && link_info.combreloc() as libc::c_int != 0
        && link_info.separate_code() as libc::c_int != 0
        && link_info.flags & ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_ulong
            != 0
    {
        return b"/* Script for -shared -z combreloc -z separate-code -z relro -z now */\n/* Copyright (C) 2014-2021 Free Software Foundation, Inc.\n   Copying and distribution of this script, with or without modification,\n   are permitted in any medium without royalty provided the copyright\n   notice and this notice are preserved.  */\nOUTPUT_FORMAT(\"elf64-x86-64\", \"elf64-x86-64\",\n\t      \"elf64-x86-64\")\nOUTPUT_ARCH(i386:x86-64)\nENTRY(_start)\nSEARCH_DIR(\"=/usr/local/x86_64-pc-linux-gnu/lib64\"); SEARCH_DIR(\"=/usr/local/lib64\"); SEARCH_DIR(\"=/lib64\"); SEARCH_DIR(\"=/usr/lib64\"); SEARCH_DIR(\"=/usr/local/x86_64-pc-linux-gnu/lib\"); SEARCH_DIR(\"=/usr/local/lib\"); SEARCH_DIR(\"=/lib\"); SEARCH_DIR(\"=/usr/lib\");\nSECTIONS\n{\n  . = SEGMENT_START(\"text-segment\", 0) + SIZEOF_HEADERS;\n  .note.gnu.build-id  : { *(.note.gnu.build-id) }\n  .hash           : { *(.hash) }\n  .gnu.hash       : { *(.gnu.hash) }\n  .dynsym         : { *(.dynsym) }\n  .dynstr         : { *(.dynstr) }\n  .gnu.version    : { *(.gnu.version) }\n  .gnu.version_d  : { *(.gnu.version_d) }\n  .gnu.version_r  : { *(.gnu.version_r) }\n  .rela.dyn       :\n    {\n      *(.rela.init)\n      *(.rela.text .rela.text.* .rela.gnu.linkonce.t.*)\n      *(.rela.fini)\n      *(.rela.rodata .rela.rodata.* .rela.gnu.linkonce.r.*)\n      *(.rela.data .rela.data.* .rela.gnu.linkonce.d.*)\n      *(.rela.tdata .rela.tdata.* .rela.gnu.linkonce.td.*)\n      *(.rela.tbss .rela.tbss.* .rela.gnu.linkonce.tb.*)\n      *(.rela.ctors)\n      *(.rela.dtors)\n      *(.rela.got)\n      *(.rela.bss .rela.bss.* .rela.gnu.linkonce.b.*)\n      *(.rela.ldata .rela.ldata.* .rela.gnu.linkonce.l.*)\n      *(.rela.lbss .rela.lbss.* .rela.gnu.linkonce.lb.*)\n      *(.rela.lrodata .rela.lrodata.* .rela.gnu.linkonce.lr.*)\n      *(.rela.ifunc)\n    }\n  .rela.plt       :\n    {\n      *(.rela.plt)\n      *(.rela.iplt)\n    }\n  . = ALIGN(CONSTANT (MAXPAGESIZE));\n  .init           :\n  {\n    KEEP (*(SORT_NONE(.init)))\n  }\n  .plt            : { *(.plt) *(.iplt) }\n.plt.got        : { *(.plt.got) }\n.plt.sec        : { *(.plt.sec) }\n  .text           :\n  {\n    *(.text.unlikely .text.*_unlikely .text.unlikely.*)\n    *(.text.exit .text.exit.*)\n    *(.text.startup .text.startup.*)\n    *(.text.hot .text.hot.*)\n    *(SORT(.text.sorted.*))\n    *(.text .stub .text.* .gnu.linkonce.t.*)\n    /* .gnu.warning sections are handled specially by elf.em.  */\n    *(.gnu.warning)\n  }\n  .fini           :\n  {\n    KEEP (*(SORT_NONE(.fini)))\n  }\n  PROVIDE (__etext = .);\n  PROVIDE (_etext = .);\n  PROVIDE (etext = .);\n  . = ALIGN(CONSTANT (MAXPAGESIZE));\n  /* Adjust the address for the rodata segment.  We want to adjust up to\n     the same address within the page on the next page up.  */\n  . = SEGMENT_START(\"rodata-segment\", ALIGN(CONSTANT (MAXPAGESIZE)) + (. & (CONSTANT (MAXPAGESIZE) - 1)));\n  .rodata         : { *(.rodata .rodata.* .gnu.linkonce.r.*) }\n  .rodata1        : { *(.rodata1) }\n  .eh_frame_hdr   : { *(.eh_frame_hdr) *(.eh_frame_entry .eh_frame_entry.*) }\n  .eh_frame       : ONLY_IF_RO { KEEP (*(.eh_frame)) *(.eh_frame.*) }\n  .gcc_except_table   : ONLY_IF_RO { *(.gcc_except_table .gcc_except_table.*) }\n  .gnu_extab   : ONLY_IF_RO { *(.gnu_extab*) }\n  /* These sections are generated by the Sun/Oracle C++ compiler.  */\n  .exception_ranges   : ONLY_IF_RO { *(.exception_ranges*) }\n  /* Adjust the address for the data segment.  We want to adjust up to\n     the same address within the page on the next page up.  */\n  . = DATA_SEGMENT_ALIGN (CONSTANT (MAXPAGESIZE), CONSTANT (COMMONPAGESIZE));\n  /* Exception handling  */\n  .eh_frame       : ONLY_IF_RW { KEEP (*(.eh_frame)) *(.eh_frame.*) }\n  .gnu_extab      : ONLY_IF_RW { *(.gnu_extab) }\n  .gcc_except_table   : ONLY_IF_RW { *(.gcc_except_table .gcc_except_table.*) }\n  .exception_ranges   : ONLY_IF_RW { *(.exception_ranges*) }\n  /* Thread Local Storage sections  */\n  .tdata\t  :\n   {\n     *(.tdata .tdata.* .gnu.linkonce.td.*)\n   }\n  .tbss\t\t  : { *(.tbss .tbss.* .gnu.linkonce.tb.*) *(.tcommon) }\n  .preinit_array    :\n  {\n    KEEP (*(.preinit_array))\n  }\n  .init_array    :\n  {\n    KEEP (*(SORT_BY_INIT_PRIORITY(.init_array.*) SORT_BY_INIT_PRIORITY(.ctors.*)))\n    KEEP (*(.init_array EXCLUDE_FILE (*crtbegin.o *crtbegin?.o *crtend.o *crtend?.o ) .ctors))\n  }\n  .fini_array    :\n  {\n    KEEP (*(SORT_BY_INIT_PRIORITY(.fini_array.*) SORT_BY_INIT_PRIORITY(.dtors.*)))\n    KEEP (*(.fini_array EXCLUDE_FILE (*crtbegin.o *crtbegin?.o *crtend.o *crtend?.o ) .dtors))\n  }\n  .ctors          :\n  {\n    /* gcc uses crtbegin.o to find the start of\n       the constructors, so we make sure it is\n       first.  Because this is a wildcard, it\n       doesn't matter if the user does not\n       actually link against crtbegin.o; the\n       linker won't look for a file to match a\n       wildcard.  The wildcard also means that it\n       doesn't matter which directory crtbegin.o\n       is in.  */\n    KEEP (*crtbegin.o(.ctors))\n    KEEP (*crtbegin?.o(.ctors))\n    /* We don't want to include the .ctor section from\n       the crtend.o file until after the sorted ctors.\n       The .ctor section from the crtend file contains the\n       end of ctors marker and it must be last */\n    KEEP (*(EXCLUDE_FILE (*crtend.o *crtend?.o ) .ctors))\n    KEEP (*(SORT(.ctors.*)))\n    KEEP (*(.ctors))\n  }\n  .dtors          :\n  {\n    KEEP (*crtbegin.o(.dtors))\n    KEEP (*crtbegin?.o(.dtors))\n    KEEP (*(EXCLUDE_FILE (*crtend.o *crtend?.o ) .dtors))\n    KEEP (*(SORT(.dtors.*)))\n    KEEP (*(.dtors))\n  }\n  .jcr            : { KEEP (*(.jcr)) }\n  .data.rel.ro : { *(.data.rel.ro.local* .gnu.linkonce.d.rel.ro.local.*) *(.data.rel.ro .data.rel.ro.* .gnu.linkonce.d.rel.ro.*) }\n  .dynamic        : { *(.dynamic) }\n  .got            : { *(.got.plt) *(.igot.plt) *(.got) *(.igot) }\n  . = DATA_SEGMENT_RELRO_END (0, .);\n  .data           :\n  {\n    *(.data .data.* .gnu.linkonce.d.*)\n    SORT(CONSTRUCTORS)\n  }\n  .data1          : { *(.data1) }\n  PROVIDE (_edata = .); PROVIDE (edata = .);\n  . = .;\n  PROVIDE (__bss_start = .);\n  .bss            :\n  {\n   *(.dynbss)\n   *(.bss .bss.* .gnu.linkonce.b.*)\n   *(COMMON)\n   /* Align here to ensure that the .bss section occupies space up to\n      _end.  Align after .bss to ensure correct alignment even if the\n      .bss section disappears because there are no input sections.\n      FIXME: Why do we need it? When there is no .bss section, we do not\n      pad the .data section.  */\n   . = ALIGN(. != 0 ? 64 / 8 : 1);\n  }\n  .lbss   :\n  {\n    *(.dynlbss)\n    *(.lbss .lbss.* .gnu.linkonce.lb.*)\n    *(LARGE_COMMON)\n  }\n  . = ALIGN(64 / 8);\n  . = SEGMENT_START(\"ldata-segment\", .);\n  .lrodata   ALIGN(CONSTANT (MAXPAGESIZE)) + (. & (CONSTANT (MAXPAGESIZE) - 1)) :\n  {\n    *(.lrodata .lrodata.* .gnu.linkonce.lr.*)\n  }\n  .ldata   ALIGN(CONSTANT (MAXPAGESIZE)) + (. & (CONSTANT (MAXPAGESIZE) - 1)) :\n  {\n    *(.ldata .ldata.* .gnu.linkonce.l.*)\n    . = ALIGN(. != 0 ? 64 / 8 : 1);\n  }\n  . = ALIGN(64 / 8);\n  PROVIDE (_end = .); PROVIDE (end = .);\n  . = DATA_SEGMENT_END (.);\n  /* Stabs debugging sections.  */\n  .stab          0 : { *(.stab) }\n  .stabstr       0 : { *(.stabstr) }\n  .stab.excl     0 : { *(.stab.excl) }\n  .stab.exclstr  0 : { *(.stab.exclstr) }\n  .stab.index    0 : { *(.stab.index) }\n  .stab.indexstr 0 : { *(.stab.indexstr) }\n  .comment       0 : { *(.comment) }\n  .gnu.build.attributes : { *(.gnu.build.attributes .gnu.build.attributes.*) }\n  /* DWARF debug sections.\n     Symbols in the DWARF debugging sections are relative to the beginning\n     of the section so we begin them at 0.  */\n  /* DWARF 1.  */\n  .debug          0 : { *(.debug) }\n  .line           0 : { *(.line) }\n  /* GNU DWARF 1 extensions.  */\n  .debug_srcinfo  0 : { *(.debug_srcinfo) }\n  .debug_sfnames  0 : { *(.debug_sfnames) }\n  /* DWARF 1.1 and DWARF 2.  */\n  .debug_aranges  0 : { *(.debug_aranges) }\n  .debug_pubnames 0 : { *(.debug_pubnames) }\n  /* DWARF 2.  */\n  .debug_info     0 : { *(.debug_info .gnu.linkonce.wi.*) }\n  .debug_abbrev   0 : { *(.debug_abbrev) }\n  .debug_line     0 : { *(.debug_line .debug_line.* .debug_line_end) }\n  .debug_frame    0 : { *(.debug_frame) }\n  .debug_str      0 : { *(.debug_str) }\n  .debug_loc      0 : { *(.debug_loc) }\n  .debug_macinfo  0 : { *(.debug_macinfo) }\n  /* SGI/MIPS DWARF 2 extensions.  */\n  .debug_weaknames 0 : { *(.debug_weaknames) }\n  .debug_funcnames 0 : { *(.debug_funcnames) }\n  .debug_typenames 0 : { *(.debug_typenames) }\n  .debug_varnames  0 : { *(.debug_varnames) }\n  /* DWARF 3.  */\n  .debug_pubtypes 0 : { *(.debug_pubtypes) }\n  .debug_ranges   0 : { *(.debug_ranges) }\n  /* DWARF 5.  */\n  .debug_addr     0 : { *(.debug_addr) }\n  .debug_line_str 0 : { *(.debug_line_str) }\n  .debug_loclists 0 : { *(.debug_loclists) }\n  .debug_macro    0 : { *(.debug_macro) }\n  .debug_names    0 : { *(.debug_names) }\n  .debug_rnglists 0 : { *(.debug_rnglists) }\n  .debug_str_offsets 0 : { *(.debug_str_offsets) }\n  .debug_sup      0 : { *(.debug_sup) }\n  .gnu.attributes 0 : { KEEP (*(.gnu.attributes)) }\n  /DISCARD/ : { *(.note.GNU-stack) *(.gnu_debuglink) *(.gnu.lto_*) }\n}\n\n\0"
            as *const u8 as *const libc::c_char as *mut libc::c_char
    } else if link_info.type_0() as libc::c_int == type_dll as libc::c_int
        && link_info.combreloc() as libc::c_int != 0
        && link_info.relro() as libc::c_int != 0
        && link_info.flags & ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_ulong
            != 0
    {
        return b"/* Script for -shared -z combreloc -z relro -z now */\n/* Copyright (C) 2014-2021 Free Software Foundation, Inc.\n   Copying and distribution of this script, with or without modification,\n   are permitted in any medium without royalty provided the copyright\n   notice and this notice are preserved.  */\nOUTPUT_FORMAT(\"elf64-x86-64\", \"elf64-x86-64\",\n\t      \"elf64-x86-64\")\nOUTPUT_ARCH(i386:x86-64)\nENTRY(_start)\nSEARCH_DIR(\"=/usr/local/x86_64-pc-linux-gnu/lib64\"); SEARCH_DIR(\"=/usr/local/lib64\"); SEARCH_DIR(\"=/lib64\"); SEARCH_DIR(\"=/usr/lib64\"); SEARCH_DIR(\"=/usr/local/x86_64-pc-linux-gnu/lib\"); SEARCH_DIR(\"=/usr/local/lib\"); SEARCH_DIR(\"=/lib\"); SEARCH_DIR(\"=/usr/lib\");\nSECTIONS\n{\n  /* Read-only sections, merged into text segment: */\n  . = SEGMENT_START(\"text-segment\", 0) + SIZEOF_HEADERS;\n  .note.gnu.build-id  : { *(.note.gnu.build-id) }\n  .hash           : { *(.hash) }\n  .gnu.hash       : { *(.gnu.hash) }\n  .dynsym         : { *(.dynsym) }\n  .dynstr         : { *(.dynstr) }\n  .gnu.version    : { *(.gnu.version) }\n  .gnu.version_d  : { *(.gnu.version_d) }\n  .gnu.version_r  : { *(.gnu.version_r) }\n  .rela.dyn       :\n    {\n      *(.rela.init)\n      *(.rela.text .rela.text.* .rela.gnu.linkonce.t.*)\n      *(.rela.fini)\n      *(.rela.rodata .rela.rodata.* .rela.gnu.linkonce.r.*)\n      *(.rela.data .rela.data.* .rela.gnu.linkonce.d.*)\n      *(.rela.tdata .rela.tdata.* .rela.gnu.linkonce.td.*)\n      *(.rela.tbss .rela.tbss.* .rela.gnu.linkonce.tb.*)\n      *(.rela.ctors)\n      *(.rela.dtors)\n      *(.rela.got)\n      *(.rela.bss .rela.bss.* .rela.gnu.linkonce.b.*)\n      *(.rela.ldata .rela.ldata.* .rela.gnu.linkonce.l.*)\n      *(.rela.lbss .rela.lbss.* .rela.gnu.linkonce.lb.*)\n      *(.rela.lrodata .rela.lrodata.* .rela.gnu.linkonce.lr.*)\n      *(.rela.ifunc)\n    }\n  .rela.plt       :\n    {\n      *(.rela.plt)\n      *(.rela.iplt)\n    }\n  .init           :\n  {\n    KEEP (*(SORT_NONE(.init)))\n  }\n  .plt            : { *(.plt) *(.iplt) }\n.plt.got        : { *(.plt.got) }\n.plt.sec        : { *(.plt.sec) }\n  .text           :\n  {\n    *(.text.unlikely .text.*_unlikely .text.unlikely.*)\n    *(.text.exit .text.exit.*)\n    *(.text.startup .text.startup.*)\n    *(.text.hot .text.hot.*)\n    *(SORT(.text.sorted.*))\n    *(.text .stub .text.* .gnu.linkonce.t.*)\n    /* .gnu.warning sections are handled specially by elf.em.  */\n    *(.gnu.warning)\n  }\n  .fini           :\n  {\n    KEEP (*(SORT_NONE(.fini)))\n  }\n  PROVIDE (__etext = .);\n  PROVIDE (_etext = .);\n  PROVIDE (etext = .);\n  .rodata         : { *(.rodata .rodata.* .gnu.linkonce.r.*) }\n  .rodata1        : { *(.rodata1) }\n  .eh_frame_hdr   : { *(.eh_frame_hdr) *(.eh_frame_entry .eh_frame_entry.*) }\n  .eh_frame       : ONLY_IF_RO { KEEP (*(.eh_frame)) *(.eh_frame.*) }\n  .gcc_except_table   : ONLY_IF_RO { *(.gcc_except_table .gcc_except_table.*) }\n  .gnu_extab   : ONLY_IF_RO { *(.gnu_extab*) }\n  /* These sections are generated by the Sun/Oracle C++ compiler.  */\n  .exception_ranges   : ONLY_IF_RO { *(.exception_ranges*) }\n  /* Adjust the address for the data segment.  We want to adjust up to\n     the same address within the page on the next page up.  */\n  . = DATA_SEGMENT_ALIGN (CONSTANT (MAXPAGESIZE), CONSTANT (COMMONPAGESIZE));\n  /* Exception handling  */\n  .eh_frame       : ONLY_IF_RW { KEEP (*(.eh_frame)) *(.eh_frame.*) }\n  .gnu_extab      : ONLY_IF_RW { *(.gnu_extab) }\n  .gcc_except_table   : ONLY_IF_RW { *(.gcc_except_table .gcc_except_table.*) }\n  .exception_ranges   : ONLY_IF_RW { *(.exception_ranges*) }\n  /* Thread Local Storage sections  */\n  .tdata\t  :\n   {\n     *(.tdata .tdata.* .gnu.linkonce.td.*)\n   }\n  .tbss\t\t  : { *(.tbss .tbss.* .gnu.linkonce.tb.*) *(.tcommon) }\n  .preinit_array    :\n  {\n    KEEP (*(.preinit_array))\n  }\n  .init_array    :\n  {\n    KEEP (*(SORT_BY_INIT_PRIORITY(.init_array.*) SORT_BY_INIT_PRIORITY(.ctors.*)))\n    KEEP (*(.init_array EXCLUDE_FILE (*crtbegin.o *crtbegin?.o *crtend.o *crtend?.o ) .ctors))\n  }\n  .fini_array    :\n  {\n    KEEP (*(SORT_BY_INIT_PRIORITY(.fini_array.*) SORT_BY_INIT_PRIORITY(.dtors.*)))\n    KEEP (*(.fini_array EXCLUDE_FILE (*crtbegin.o *crtbegin?.o *crtend.o *crtend?.o ) .dtors))\n  }\n  .ctors          :\n  {\n    /* gcc uses crtbegin.o to find the start of\n       the constructors, so we make sure it is\n       first.  Because this is a wildcard, it\n       doesn't matter if the user does not\n       actually link against crtbegin.o; the\n       linker won't look for a file to match a\n       wildcard.  The wildcard also means that it\n       doesn't matter which directory crtbegin.o\n       is in.  */\n    KEEP (*crtbegin.o(.ctors))\n    KEEP (*crtbegin?.o(.ctors))\n    /* We don't want to include the .ctor section from\n       the crtend.o file until after the sorted ctors.\n       The .ctor section from the crtend file contains the\n       end of ctors marker and it must be last */\n    KEEP (*(EXCLUDE_FILE (*crtend.o *crtend?.o ) .ctors))\n    KEEP (*(SORT(.ctors.*)))\n    KEEP (*(.ctors))\n  }\n  .dtors          :\n  {\n    KEEP (*crtbegin.o(.dtors))\n    KEEP (*crtbegin?.o(.dtors))\n    KEEP (*(EXCLUDE_FILE (*crtend.o *crtend?.o ) .dtors))\n    KEEP (*(SORT(.dtors.*)))\n    KEEP (*(.dtors))\n  }\n  .jcr            : { KEEP (*(.jcr)) }\n  .data.rel.ro : { *(.data.rel.ro.local* .gnu.linkonce.d.rel.ro.local.*) *(.data.rel.ro .data.rel.ro.* .gnu.linkonce.d.rel.ro.*) }\n  .dynamic        : { *(.dynamic) }\n  .got            : { *(.got.plt) *(.igot.plt) *(.got) *(.igot) }\n  . = DATA_SEGMENT_RELRO_END (0, .);\n  .data           :\n  {\n    *(.data .data.* .gnu.linkonce.d.*)\n    SORT(CONSTRUCTORS)\n  }\n  .data1          : { *(.data1) }\n  PROVIDE (_edata = .); PROVIDE (edata = .);\n  . = .;\n  PROVIDE (__bss_start = .);\n  .bss            :\n  {\n   *(.dynbss)\n   *(.bss .bss.* .gnu.linkonce.b.*)\n   *(COMMON)\n   /* Align here to ensure that the .bss section occupies space up to\n      _end.  Align after .bss to ensure correct alignment even if the\n      .bss section disappears because there are no input sections.\n      FIXME: Why do we need it? When there is no .bss section, we do not\n      pad the .data section.  */\n   . = ALIGN(. != 0 ? 64 / 8 : 1);\n  }\n  .lbss   :\n  {\n    *(.dynlbss)\n    *(.lbss .lbss.* .gnu.linkonce.lb.*)\n    *(LARGE_COMMON)\n  }\n  . = ALIGN(64 / 8);\n  . = SEGMENT_START(\"ldata-segment\", .);\n  .lrodata   ALIGN(CONSTANT (MAXPAGESIZE)) + (. & (CONSTANT (MAXPAGESIZE) - 1)) :\n  {\n    *(.lrodata .lrodata.* .gnu.linkonce.lr.*)\n  }\n  .ldata   ALIGN(CONSTANT (MAXPAGESIZE)) + (. & (CONSTANT (MAXPAGESIZE) - 1)) :\n  {\n    *(.ldata .ldata.* .gnu.linkonce.l.*)\n    . = ALIGN(. != 0 ? 64 / 8 : 1);\n  }\n  . = ALIGN(64 / 8);\n  PROVIDE (_end = .); PROVIDE (end = .);\n  . = DATA_SEGMENT_END (.);\n  /* Stabs debugging sections.  */\n  .stab          0 : { *(.stab) }\n  .stabstr       0 : { *(.stabstr) }\n  .stab.excl     0 : { *(.stab.excl) }\n  .stab.exclstr  0 : { *(.stab.exclstr) }\n  .stab.index    0 : { *(.stab.index) }\n  .stab.indexstr 0 : { *(.stab.indexstr) }\n  .comment       0 : { *(.comment) }\n  .gnu.build.attributes : { *(.gnu.build.attributes .gnu.build.attributes.*) }\n  /* DWARF debug sections.\n     Symbols in the DWARF debugging sections are relative to the beginning\n     of the section so we begin them at 0.  */\n  /* DWARF 1.  */\n  .debug          0 : { *(.debug) }\n  .line           0 : { *(.line) }\n  /* GNU DWARF 1 extensions.  */\n  .debug_srcinfo  0 : { *(.debug_srcinfo) }\n  .debug_sfnames  0 : { *(.debug_sfnames) }\n  /* DWARF 1.1 and DWARF 2.  */\n  .debug_aranges  0 : { *(.debug_aranges) }\n  .debug_pubnames 0 : { *(.debug_pubnames) }\n  /* DWARF 2.  */\n  .debug_info     0 : { *(.debug_info .gnu.linkonce.wi.*) }\n  .debug_abbrev   0 : { *(.debug_abbrev) }\n  .debug_line     0 : { *(.debug_line .debug_line.* .debug_line_end) }\n  .debug_frame    0 : { *(.debug_frame) }\n  .debug_str      0 : { *(.debug_str) }\n  .debug_loc      0 : { *(.debug_loc) }\n  .debug_macinfo  0 : { *(.debug_macinfo) }\n  /* SGI/MIPS DWARF 2 extensions.  */\n  .debug_weaknames 0 : { *(.debug_weaknames) }\n  .debug_funcnames 0 : { *(.debug_funcnames) }\n  .debug_typenames 0 : { *(.debug_typenames) }\n  .debug_varnames  0 : { *(.debug_varnames) }\n  /* DWARF 3.  */\n  .debug_pubtypes 0 : { *(.debug_pubtypes) }\n  .debug_ranges   0 : { *(.debug_ranges) }\n  /* DWARF 5.  */\n  .debug_addr     0 : { *(.debug_addr) }\n  .debug_line_str 0 : { *(.debug_line_str) }\n  .debug_loclists 0 : { *(.debug_loclists) }\n  .debug_macro    0 : { *(.debug_macro) }\n  .debug_names    0 : { *(.debug_names) }\n  .debug_rnglists 0 : { *(.debug_rnglists) }\n  .debug_str_offsets 0 : { *(.debug_str_offsets) }\n  .debug_sup      0 : { *(.debug_sup) }\n  .gnu.attributes 0 : { KEEP (*(.gnu.attributes)) }\n  /DISCARD/ : { *(.note.GNU-stack) *(.gnu_debuglink) *(.gnu.lto_*) }\n}\n\n\0"
            as *const u8 as *const libc::c_char as *mut libc::c_char
    } else if link_info.type_0() as libc::c_int == type_dll as libc::c_int
        && link_info.combreloc() as libc::c_int != 0
        && link_info.separate_code() as libc::c_int != 0
    {
        return b"/* Script for -shared -z combreloc -z separate-code */\n/* Copyright (C) 2014-2021 Free Software Foundation, Inc.\n   Copying and distribution of this script, with or without modification,\n   are permitted in any medium without royalty provided the copyright\n   notice and this notice are preserved.  */\nOUTPUT_FORMAT(\"elf64-x86-64\", \"elf64-x86-64\",\n\t      \"elf64-x86-64\")\nOUTPUT_ARCH(i386:x86-64)\nENTRY(_start)\nSEARCH_DIR(\"=/usr/local/x86_64-pc-linux-gnu/lib64\"); SEARCH_DIR(\"=/usr/local/lib64\"); SEARCH_DIR(\"=/lib64\"); SEARCH_DIR(\"=/usr/lib64\"); SEARCH_DIR(\"=/usr/local/x86_64-pc-linux-gnu/lib\"); SEARCH_DIR(\"=/usr/local/lib\"); SEARCH_DIR(\"=/lib\"); SEARCH_DIR(\"=/usr/lib\");\nSECTIONS\n{\n  . = SEGMENT_START(\"text-segment\", 0) + SIZEOF_HEADERS;\n  .note.gnu.build-id  : { *(.note.gnu.build-id) }\n  .hash           : { *(.hash) }\n  .gnu.hash       : { *(.gnu.hash) }\n  .dynsym         : { *(.dynsym) }\n  .dynstr         : { *(.dynstr) }\n  .gnu.version    : { *(.gnu.version) }\n  .gnu.version_d  : { *(.gnu.version_d) }\n  .gnu.version_r  : { *(.gnu.version_r) }\n  .rela.dyn       :\n    {\n      *(.rela.init)\n      *(.rela.text .rela.text.* .rela.gnu.linkonce.t.*)\n      *(.rela.fini)\n      *(.rela.rodata .rela.rodata.* .rela.gnu.linkonce.r.*)\n      *(.rela.data .rela.data.* .rela.gnu.linkonce.d.*)\n      *(.rela.tdata .rela.tdata.* .rela.gnu.linkonce.td.*)\n      *(.rela.tbss .rela.tbss.* .rela.gnu.linkonce.tb.*)\n      *(.rela.ctors)\n      *(.rela.dtors)\n      *(.rela.got)\n      *(.rela.bss .rela.bss.* .rela.gnu.linkonce.b.*)\n      *(.rela.ldata .rela.ldata.* .rela.gnu.linkonce.l.*)\n      *(.rela.lbss .rela.lbss.* .rela.gnu.linkonce.lb.*)\n      *(.rela.lrodata .rela.lrodata.* .rela.gnu.linkonce.lr.*)\n      *(.rela.ifunc)\n    }\n  .rela.plt       :\n    {\n      *(.rela.plt)\n      *(.rela.iplt)\n    }\n  . = ALIGN(CONSTANT (MAXPAGESIZE));\n  .init           :\n  {\n    KEEP (*(SORT_NONE(.init)))\n  }\n  .plt            : { *(.plt) *(.iplt) }\n.plt.got        : { *(.plt.got) }\n.plt.sec        : { *(.plt.sec) }\n  .text           :\n  {\n    *(.text.unlikely .text.*_unlikely .text.unlikely.*)\n    *(.text.exit .text.exit.*)\n    *(.text.startup .text.startup.*)\n    *(.text.hot .text.hot.*)\n    *(SORT(.text.sorted.*))\n    *(.text .stub .text.* .gnu.linkonce.t.*)\n    /* .gnu.warning sections are handled specially by elf.em.  */\n    *(.gnu.warning)\n  }\n  .fini           :\n  {\n    KEEP (*(SORT_NONE(.fini)))\n  }\n  PROVIDE (__etext = .);\n  PROVIDE (_etext = .);\n  PROVIDE (etext = .);\n  . = ALIGN(CONSTANT (MAXPAGESIZE));\n  /* Adjust the address for the rodata segment.  We want to adjust up to\n     the same address within the page on the next page up.  */\n  . = SEGMENT_START(\"rodata-segment\", ALIGN(CONSTANT (MAXPAGESIZE)) + (. & (CONSTANT (MAXPAGESIZE) - 1)));\n  .rodata         : { *(.rodata .rodata.* .gnu.linkonce.r.*) }\n  .rodata1        : { *(.rodata1) }\n  .eh_frame_hdr   : { *(.eh_frame_hdr) *(.eh_frame_entry .eh_frame_entry.*) }\n  .eh_frame       : ONLY_IF_RO { KEEP (*(.eh_frame)) *(.eh_frame.*) }\n  .gcc_except_table   : ONLY_IF_RO { *(.gcc_except_table .gcc_except_table.*) }\n  .gnu_extab   : ONLY_IF_RO { *(.gnu_extab*) }\n  /* These sections are generated by the Sun/Oracle C++ compiler.  */\n  .exception_ranges   : ONLY_IF_RO { *(.exception_ranges*) }\n  /* Adjust the address for the data segment.  We want to adjust up to\n     the same address within the page on the next page up.  */\n  . = DATA_SEGMENT_ALIGN (CONSTANT (MAXPAGESIZE), CONSTANT (COMMONPAGESIZE));\n  /* Exception handling  */\n  .eh_frame       : ONLY_IF_RW { KEEP (*(.eh_frame)) *(.eh_frame.*) }\n  .gnu_extab      : ONLY_IF_RW { *(.gnu_extab) }\n  .gcc_except_table   : ONLY_IF_RW { *(.gcc_except_table .gcc_except_table.*) }\n  .exception_ranges   : ONLY_IF_RW { *(.exception_ranges*) }\n  /* Thread Local Storage sections  */\n  .tdata\t  :\n   {\n     *(.tdata .tdata.* .gnu.linkonce.td.*)\n   }\n  .tbss\t\t  : { *(.tbss .tbss.* .gnu.linkonce.tb.*) *(.tcommon) }\n  .preinit_array    :\n  {\n    KEEP (*(.preinit_array))\n  }\n  .init_array    :\n  {\n    KEEP (*(SORT_BY_INIT_PRIORITY(.init_array.*) SORT_BY_INIT_PRIORITY(.ctors.*)))\n    KEEP (*(.init_array EXCLUDE_FILE (*crtbegin.o *crtbegin?.o *crtend.o *crtend?.o ) .ctors))\n  }\n  .fini_array    :\n  {\n    KEEP (*(SORT_BY_INIT_PRIORITY(.fini_array.*) SORT_BY_INIT_PRIORITY(.dtors.*)))\n    KEEP (*(.fini_array EXCLUDE_FILE (*crtbegin.o *crtbegin?.o *crtend.o *crtend?.o ) .dtors))\n  }\n  .ctors          :\n  {\n    /* gcc uses crtbegin.o to find the start of\n       the constructors, so we make sure it is\n       first.  Because this is a wildcard, it\n       doesn't matter if the user does not\n       actually link against crtbegin.o; the\n       linker won't look for a file to match a\n       wildcard.  The wildcard also means that it\n       doesn't matter which directory crtbegin.o\n       is in.  */\n    KEEP (*crtbegin.o(.ctors))\n    KEEP (*crtbegin?.o(.ctors))\n    /* We don't want to include the .ctor section from\n       the crtend.o file until after the sorted ctors.\n       The .ctor section from the crtend file contains the\n       end of ctors marker and it must be last */\n    KEEP (*(EXCLUDE_FILE (*crtend.o *crtend?.o ) .ctors))\n    KEEP (*(SORT(.ctors.*)))\n    KEEP (*(.ctors))\n  }\n  .dtors          :\n  {\n    KEEP (*crtbegin.o(.dtors))\n    KEEP (*crtbegin?.o(.dtors))\n    KEEP (*(EXCLUDE_FILE (*crtend.o *crtend?.o ) .dtors))\n    KEEP (*(SORT(.dtors.*)))\n    KEEP (*(.dtors))\n  }\n  .jcr            : { KEEP (*(.jcr)) }\n  .data.rel.ro : { *(.data.rel.ro.local* .gnu.linkonce.d.rel.ro.local.*) *(.data.rel.ro .data.rel.ro.* .gnu.linkonce.d.rel.ro.*) }\n  .dynamic        : { *(.dynamic) }\n  .got            : { *(.got) *(.igot) }\n  . = DATA_SEGMENT_RELRO_END (SIZEOF (.got.plt) >= 24 ? 24 : 0, .);\n  .got.plt        : { *(.got.plt) *(.igot.plt) }\n  .data           :\n  {\n    *(.data .data.* .gnu.linkonce.d.*)\n    SORT(CONSTRUCTORS)\n  }\n  .data1          : { *(.data1) }\n  PROVIDE (_edata = .); PROVIDE (edata = .);\n  . = .;\n  PROVIDE (__bss_start = .);\n  .bss            :\n  {\n   *(.dynbss)\n   *(.bss .bss.* .gnu.linkonce.b.*)\n   *(COMMON)\n   /* Align here to ensure that the .bss section occupies space up to\n      _end.  Align after .bss to ensure correct alignment even if the\n      .bss section disappears because there are no input sections.\n      FIXME: Why do we need it? When there is no .bss section, we do not\n      pad the .data section.  */\n   . = ALIGN(. != 0 ? 64 / 8 : 1);\n  }\n  .lbss   :\n  {\n    *(.dynlbss)\n    *(.lbss .lbss.* .gnu.linkonce.lb.*)\n    *(LARGE_COMMON)\n  }\n  . = ALIGN(64 / 8);\n  . = SEGMENT_START(\"ldata-segment\", .);\n  .lrodata   ALIGN(CONSTANT (MAXPAGESIZE)) + (. & (CONSTANT (MAXPAGESIZE) - 1)) :\n  {\n    *(.lrodata .lrodata.* .gnu.linkonce.lr.*)\n  }\n  .ldata   ALIGN(CONSTANT (MAXPAGESIZE)) + (. & (CONSTANT (MAXPAGESIZE) - 1)) :\n  {\n    *(.ldata .ldata.* .gnu.linkonce.l.*)\n    . = ALIGN(. != 0 ? 64 / 8 : 1);\n  }\n  . = ALIGN(64 / 8);\n  PROVIDE (_end = .); PROVIDE (end = .);\n  . = DATA_SEGMENT_END (.);\n  /* Stabs debugging sections.  */\n  .stab          0 : { *(.stab) }\n  .stabstr       0 : { *(.stabstr) }\n  .stab.excl     0 : { *(.stab.excl) }\n  .stab.exclstr  0 : { *(.stab.exclstr) }\n  .stab.index    0 : { *(.stab.index) }\n  .stab.indexstr 0 : { *(.stab.indexstr) }\n  .comment       0 : { *(.comment) }\n  .gnu.build.attributes : { *(.gnu.build.attributes .gnu.build.attributes.*) }\n  /* DWARF debug sections.\n     Symbols in the DWARF debugging sections are relative to the beginning\n     of the section so we begin them at 0.  */\n  /* DWARF 1.  */\n  .debug          0 : { *(.debug) }\n  .line           0 : { *(.line) }\n  /* GNU DWARF 1 extensions.  */\n  .debug_srcinfo  0 : { *(.debug_srcinfo) }\n  .debug_sfnames  0 : { *(.debug_sfnames) }\n  /* DWARF 1.1 and DWARF 2.  */\n  .debug_aranges  0 : { *(.debug_aranges) }\n  .debug_pubnames 0 : { *(.debug_pubnames) }\n  /* DWARF 2.  */\n  .debug_info     0 : { *(.debug_info .gnu.linkonce.wi.*) }\n  .debug_abbrev   0 : { *(.debug_abbrev) }\n  .debug_line     0 : { *(.debug_line .debug_line.* .debug_line_end) }\n  .debug_frame    0 : { *(.debug_frame) }\n  .debug_str      0 : { *(.debug_str) }\n  .debug_loc      0 : { *(.debug_loc) }\n  .debug_macinfo  0 : { *(.debug_macinfo) }\n  /* SGI/MIPS DWARF 2 extensions.  */\n  .debug_weaknames 0 : { *(.debug_weaknames) }\n  .debug_funcnames 0 : { *(.debug_funcnames) }\n  .debug_typenames 0 : { *(.debug_typenames) }\n  .debug_varnames  0 : { *(.debug_varnames) }\n  /* DWARF 3.  */\n  .debug_pubtypes 0 : { *(.debug_pubtypes) }\n  .debug_ranges   0 : { *(.debug_ranges) }\n  /* DWARF 5.  */\n  .debug_addr     0 : { *(.debug_addr) }\n  .debug_line_str 0 : { *(.debug_line_str) }\n  .debug_loclists 0 : { *(.debug_loclists) }\n  .debug_macro    0 : { *(.debug_macro) }\n  .debug_names    0 : { *(.debug_names) }\n  .debug_rnglists 0 : { *(.debug_rnglists) }\n  .debug_str_offsets 0 : { *(.debug_str_offsets) }\n  .debug_sup      0 : { *(.debug_sup) }\n  .gnu.attributes 0 : { KEEP (*(.gnu.attributes)) }\n  /DISCARD/ : { *(.note.GNU-stack) *(.gnu_debuglink) *(.gnu.lto_*) }\n}\n\n\0"
            as *const u8 as *const libc::c_char as *mut libc::c_char
    } else if link_info.type_0() as libc::c_int == type_dll as libc::c_int
        && link_info.combreloc() as libc::c_int != 0
    {
        return b"/* Script for -shared -z combreloc */\n/* Copyright (C) 2014-2021 Free Software Foundation, Inc.\n   Copying and distribution of this script, with or without modification,\n   are permitted in any medium without royalty provided the copyright\n   notice and this notice are preserved.  */\nOUTPUT_FORMAT(\"elf64-x86-64\", \"elf64-x86-64\",\n\t      \"elf64-x86-64\")\nOUTPUT_ARCH(i386:x86-64)\nENTRY(_start)\nSEARCH_DIR(\"=/usr/local/x86_64-pc-linux-gnu/lib64\"); SEARCH_DIR(\"=/usr/local/lib64\"); SEARCH_DIR(\"=/lib64\"); SEARCH_DIR(\"=/usr/lib64\"); SEARCH_DIR(\"=/usr/local/x86_64-pc-linux-gnu/lib\"); SEARCH_DIR(\"=/usr/local/lib\"); SEARCH_DIR(\"=/lib\"); SEARCH_DIR(\"=/usr/lib\");\nSECTIONS\n{\n  /* Read-only sections, merged into text segment: */\n  . = SEGMENT_START(\"text-segment\", 0) + SIZEOF_HEADERS;\n  .note.gnu.build-id  : { *(.note.gnu.build-id) }\n  .hash           : { *(.hash) }\n  .gnu.hash       : { *(.gnu.hash) }\n  .dynsym         : { *(.dynsym) }\n  .dynstr         : { *(.dynstr) }\n  .gnu.version    : { *(.gnu.version) }\n  .gnu.version_d  : { *(.gnu.version_d) }\n  .gnu.version_r  : { *(.gnu.version_r) }\n  .rela.dyn       :\n    {\n      *(.rela.init)\n      *(.rela.text .rela.text.* .rela.gnu.linkonce.t.*)\n      *(.rela.fini)\n      *(.rela.rodata .rela.rodata.* .rela.gnu.linkonce.r.*)\n      *(.rela.data .rela.data.* .rela.gnu.linkonce.d.*)\n      *(.rela.tdata .rela.tdata.* .rela.gnu.linkonce.td.*)\n      *(.rela.tbss .rela.tbss.* .rela.gnu.linkonce.tb.*)\n      *(.rela.ctors)\n      *(.rela.dtors)\n      *(.rela.got)\n      *(.rela.bss .rela.bss.* .rela.gnu.linkonce.b.*)\n      *(.rela.ldata .rela.ldata.* .rela.gnu.linkonce.l.*)\n      *(.rela.lbss .rela.lbss.* .rela.gnu.linkonce.lb.*)\n      *(.rela.lrodata .rela.lrodata.* .rela.gnu.linkonce.lr.*)\n      *(.rela.ifunc)\n    }\n  .rela.plt       :\n    {\n      *(.rela.plt)\n      *(.rela.iplt)\n    }\n  .init           :\n  {\n    KEEP (*(SORT_NONE(.init)))\n  }\n  .plt            : { *(.plt) *(.iplt) }\n.plt.got        : { *(.plt.got) }\n.plt.sec        : { *(.plt.sec) }\n  .text           :\n  {\n    *(.text.unlikely .text.*_unlikely .text.unlikely.*)\n    *(.text.exit .text.exit.*)\n    *(.text.startup .text.startup.*)\n    *(.text.hot .text.hot.*)\n    *(SORT(.text.sorted.*))\n    *(.text .stub .text.* .gnu.linkonce.t.*)\n    /* .gnu.warning sections are handled specially by elf.em.  */\n    *(.gnu.warning)\n  }\n  .fini           :\n  {\n    KEEP (*(SORT_NONE(.fini)))\n  }\n  PROVIDE (__etext = .);\n  PROVIDE (_etext = .);\n  PROVIDE (etext = .);\n  .rodata         : { *(.rodata .rodata.* .gnu.linkonce.r.*) }\n  .rodata1        : { *(.rodata1) }\n  .eh_frame_hdr   : { *(.eh_frame_hdr) *(.eh_frame_entry .eh_frame_entry.*) }\n  .eh_frame       : ONLY_IF_RO { KEEP (*(.eh_frame)) *(.eh_frame.*) }\n  .gcc_except_table   : ONLY_IF_RO { *(.gcc_except_table .gcc_except_table.*) }\n  .gnu_extab   : ONLY_IF_RO { *(.gnu_extab*) }\n  /* These sections are generated by the Sun/Oracle C++ compiler.  */\n  .exception_ranges   : ONLY_IF_RO { *(.exception_ranges*) }\n  /* Adjust the address for the data segment.  We want to adjust up to\n     the same address within the page on the next page up.  */\n  . = DATA_SEGMENT_ALIGN (CONSTANT (MAXPAGESIZE), CONSTANT (COMMONPAGESIZE));\n  /* Exception handling  */\n  .eh_frame       : ONLY_IF_RW { KEEP (*(.eh_frame)) *(.eh_frame.*) }\n  .gnu_extab      : ONLY_IF_RW { *(.gnu_extab) }\n  .gcc_except_table   : ONLY_IF_RW { *(.gcc_except_table .gcc_except_table.*) }\n  .exception_ranges   : ONLY_IF_RW { *(.exception_ranges*) }\n  /* Thread Local Storage sections  */\n  .tdata\t  :\n   {\n     *(.tdata .tdata.* .gnu.linkonce.td.*)\n   }\n  .tbss\t\t  : { *(.tbss .tbss.* .gnu.linkonce.tb.*) *(.tcommon) }\n  .preinit_array    :\n  {\n    KEEP (*(.preinit_array))\n  }\n  .init_array    :\n  {\n    KEEP (*(SORT_BY_INIT_PRIORITY(.init_array.*) SORT_BY_INIT_PRIORITY(.ctors.*)))\n    KEEP (*(.init_array EXCLUDE_FILE (*crtbegin.o *crtbegin?.o *crtend.o *crtend?.o ) .ctors))\n  }\n  .fini_array    :\n  {\n    KEEP (*(SORT_BY_INIT_PRIORITY(.fini_array.*) SORT_BY_INIT_PRIORITY(.dtors.*)))\n    KEEP (*(.fini_array EXCLUDE_FILE (*crtbegin.o *crtbegin?.o *crtend.o *crtend?.o ) .dtors))\n  }\n  .ctors          :\n  {\n    /* gcc uses crtbegin.o to find the start of\n       the constructors, so we make sure it is\n       first.  Because this is a wildcard, it\n       doesn't matter if the user does not\n       actually link against crtbegin.o; the\n       linker won't look for a file to match a\n       wildcard.  The wildcard also means that it\n       doesn't matter which directory crtbegin.o\n       is in.  */\n    KEEP (*crtbegin.o(.ctors))\n    KEEP (*crtbegin?.o(.ctors))\n    /* We don't want to include the .ctor section from\n       the crtend.o file until after the sorted ctors.\n       The .ctor section from the crtend file contains the\n       end of ctors marker and it must be last */\n    KEEP (*(EXCLUDE_FILE (*crtend.o *crtend?.o ) .ctors))\n    KEEP (*(SORT(.ctors.*)))\n    KEEP (*(.ctors))\n  }\n  .dtors          :\n  {\n    KEEP (*crtbegin.o(.dtors))\n    KEEP (*crtbegin?.o(.dtors))\n    KEEP (*(EXCLUDE_FILE (*crtend.o *crtend?.o ) .dtors))\n    KEEP (*(SORT(.dtors.*)))\n    KEEP (*(.dtors))\n  }\n  .jcr            : { KEEP (*(.jcr)) }\n  .data.rel.ro : { *(.data.rel.ro.local* .gnu.linkonce.d.rel.ro.local.*) *(.data.rel.ro .data.rel.ro.* .gnu.linkonce.d.rel.ro.*) }\n  .dynamic        : { *(.dynamic) }\n  .got            : { *(.got) *(.igot) }\n  . = DATA_SEGMENT_RELRO_END (SIZEOF (.got.plt) >= 24 ? 24 : 0, .);\n  .got.plt        : { *(.got.plt) *(.igot.plt) }\n  .data           :\n  {\n    *(.data .data.* .gnu.linkonce.d.*)\n    SORT(CONSTRUCTORS)\n  }\n  .data1          : { *(.data1) }\n  PROVIDE (_edata = .); PROVIDE (edata = .);\n  . = .;\n  PROVIDE (__bss_start = .);\n  .bss            :\n  {\n   *(.dynbss)\n   *(.bss .bss.* .gnu.linkonce.b.*)\n   *(COMMON)\n   /* Align here to ensure that the .bss section occupies space up to\n      _end.  Align after .bss to ensure correct alignment even if the\n      .bss section disappears because there are no input sections.\n      FIXME: Why do we need it? When there is no .bss section, we do not\n      pad the .data section.  */\n   . = ALIGN(. != 0 ? 64 / 8 : 1);\n  }\n  .lbss   :\n  {\n    *(.dynlbss)\n    *(.lbss .lbss.* .gnu.linkonce.lb.*)\n    *(LARGE_COMMON)\n  }\n  . = ALIGN(64 / 8);\n  . = SEGMENT_START(\"ldata-segment\", .);\n  .lrodata   ALIGN(CONSTANT (MAXPAGESIZE)) + (. & (CONSTANT (MAXPAGESIZE) - 1)) :\n  {\n    *(.lrodata .lrodata.* .gnu.linkonce.lr.*)\n  }\n  .ldata   ALIGN(CONSTANT (MAXPAGESIZE)) + (. & (CONSTANT (MAXPAGESIZE) - 1)) :\n  {\n    *(.ldata .ldata.* .gnu.linkonce.l.*)\n    . = ALIGN(. != 0 ? 64 / 8 : 1);\n  }\n  . = ALIGN(64 / 8);\n  PROVIDE (_end = .); PROVIDE (end = .);\n  . = DATA_SEGMENT_END (.);\n  /* Stabs debugging sections.  */\n  .stab          0 : { *(.stab) }\n  .stabstr       0 : { *(.stabstr) }\n  .stab.excl     0 : { *(.stab.excl) }\n  .stab.exclstr  0 : { *(.stab.exclstr) }\n  .stab.index    0 : { *(.stab.index) }\n  .stab.indexstr 0 : { *(.stab.indexstr) }\n  .comment       0 : { *(.comment) }\n  .gnu.build.attributes : { *(.gnu.build.attributes .gnu.build.attributes.*) }\n  /* DWARF debug sections.\n     Symbols in the DWARF debugging sections are relative to the beginning\n     of the section so we begin them at 0.  */\n  /* DWARF 1.  */\n  .debug          0 : { *(.debug) }\n  .line           0 : { *(.line) }\n  /* GNU DWARF 1 extensions.  */\n  .debug_srcinfo  0 : { *(.debug_srcinfo) }\n  .debug_sfnames  0 : { *(.debug_sfnames) }\n  /* DWARF 1.1 and DWARF 2.  */\n  .debug_aranges  0 : { *(.debug_aranges) }\n  .debug_pubnames 0 : { *(.debug_pubnames) }\n  /* DWARF 2.  */\n  .debug_info     0 : { *(.debug_info .gnu.linkonce.wi.*) }\n  .debug_abbrev   0 : { *(.debug_abbrev) }\n  .debug_line     0 : { *(.debug_line .debug_line.* .debug_line_end) }\n  .debug_frame    0 : { *(.debug_frame) }\n  .debug_str      0 : { *(.debug_str) }\n  .debug_loc      0 : { *(.debug_loc) }\n  .debug_macinfo  0 : { *(.debug_macinfo) }\n  /* SGI/MIPS DWARF 2 extensions.  */\n  .debug_weaknames 0 : { *(.debug_weaknames) }\n  .debug_funcnames 0 : { *(.debug_funcnames) }\n  .debug_typenames 0 : { *(.debug_typenames) }\n  .debug_varnames  0 : { *(.debug_varnames) }\n  /* DWARF 3.  */\n  .debug_pubtypes 0 : { *(.debug_pubtypes) }\n  .debug_ranges   0 : { *(.debug_ranges) }\n  /* DWARF 5.  */\n  .debug_addr     0 : { *(.debug_addr) }\n  .debug_line_str 0 : { *(.debug_line_str) }\n  .debug_loclists 0 : { *(.debug_loclists) }\n  .debug_macro    0 : { *(.debug_macro) }\n  .debug_names    0 : { *(.debug_names) }\n  .debug_rnglists 0 : { *(.debug_rnglists) }\n  .debug_str_offsets 0 : { *(.debug_str_offsets) }\n  .debug_sup      0 : { *(.debug_sup) }\n  .gnu.attributes 0 : { KEEP (*(.gnu.attributes)) }\n  /DISCARD/ : { *(.note.GNU-stack) *(.gnu_debuglink) *(.gnu.lto_*) }\n}\n\n\0"
            as *const u8 as *const libc::c_char as *mut libc::c_char
    } else if link_info.type_0() as libc::c_int == type_dll as libc::c_int
        && link_info.separate_code() as libc::c_int != 0
    {
        return b"/* Script for -shared -z separate-code */\n/* Copyright (C) 2014-2021 Free Software Foundation, Inc.\n   Copying and distribution of this script, with or without modification,\n   are permitted in any medium without royalty provided the copyright\n   notice and this notice are preserved.  */\nOUTPUT_FORMAT(\"elf64-x86-64\", \"elf64-x86-64\",\n\t      \"elf64-x86-64\")\nOUTPUT_ARCH(i386:x86-64)\nENTRY(_start)\nSEARCH_DIR(\"=/usr/local/x86_64-pc-linux-gnu/lib64\"); SEARCH_DIR(\"=/usr/local/lib64\"); SEARCH_DIR(\"=/lib64\"); SEARCH_DIR(\"=/usr/lib64\"); SEARCH_DIR(\"=/usr/local/x86_64-pc-linux-gnu/lib\"); SEARCH_DIR(\"=/usr/local/lib\"); SEARCH_DIR(\"=/lib\"); SEARCH_DIR(\"=/usr/lib\");\nSECTIONS\n{\n  . = SEGMENT_START(\"text-segment\", 0) + SIZEOF_HEADERS;\n  .note.gnu.build-id  : { *(.note.gnu.build-id) }\n  .hash           : { *(.hash) }\n  .gnu.hash       : { *(.gnu.hash) }\n  .dynsym         : { *(.dynsym) }\n  .dynstr         : { *(.dynstr) }\n  .gnu.version    : { *(.gnu.version) }\n  .gnu.version_d  : { *(.gnu.version_d) }\n  .gnu.version_r  : { *(.gnu.version_r) }\n  .rela.init      : { *(.rela.init) }\n  .rela.text      : { *(.rela.text .rela.text.* .rela.gnu.linkonce.t.*) }\n  .rela.fini      : { *(.rela.fini) }\n  .rela.rodata    : { *(.rela.rodata .rela.rodata.* .rela.gnu.linkonce.r.*) }\n  .rela.data.rel.ro   : { *(.rela.data.rel.ro .rela.data.rel.ro.* .rela.gnu.linkonce.d.rel.ro.*) }\n  .rela.data      : { *(.rela.data .rela.data.* .rela.gnu.linkonce.d.*) }\n  .rela.tdata\t  : { *(.rela.tdata .rela.tdata.* .rela.gnu.linkonce.td.*) }\n  .rela.tbss\t  : { *(.rela.tbss .rela.tbss.* .rela.gnu.linkonce.tb.*) }\n  .rela.ctors     : { *(.rela.ctors) }\n  .rela.dtors     : { *(.rela.dtors) }\n  .rela.got       : { *(.rela.got) }\n  .rela.bss       : { *(.rela.bss .rela.bss.* .rela.gnu.linkonce.b.*) }\n  .rela.ldata     : { *(.rela.ldata .rela.ldata.* .rela.gnu.linkonce.l.*) }\n  .rela.lbss      : { *(.rela.lbss .rela.lbss.* .rela.gnu.linkonce.lb.*) }\n  .rela.lrodata   : { *(.rela.lrodata .rela.lrodata.* .rela.gnu.linkonce.lr.*) }\n  .rela.ifunc     : { *(.rela.ifunc) }\n  .rela.plt       :\n    {\n      *(.rela.plt)\n      *(.rela.iplt)\n    }\n  . = ALIGN(CONSTANT (MAXPAGESIZE));\n  .init           :\n  {\n    KEEP (*(SORT_NONE(.init)))\n  }\n  .plt            : { *(.plt) *(.iplt) }\n.plt.got        : { *(.plt.got) }\n.plt.sec        : { *(.plt.sec) }\n  .text           :\n  {\n    *(.text.unlikely .text.*_unlikely .text.unlikely.*)\n    *(.text.exit .text.exit.*)\n    *(.text.startup .text.startup.*)\n    *(.text.hot .text.hot.*)\n    *(SORT(.text.sorted.*))\n    *(.text .stub .text.* .gnu.linkonce.t.*)\n    /* .gnu.warning sections are handled specially by elf.em.  */\n    *(.gnu.warning)\n  }\n  .fini           :\n  {\n    KEEP (*(SORT_NONE(.fini)))\n  }\n  PROVIDE (__etext = .);\n  PROVIDE (_etext = .);\n  PROVIDE (etext = .);\n  . = ALIGN(CONSTANT (MAXPAGESIZE));\n  /* Adjust the address for the rodata segment.  We want to adjust up to\n     the same address within the page on the next page up.  */\n  . = SEGMENT_START(\"rodata-segment\", ALIGN(CONSTANT (MAXPAGESIZE)) + (. & (CONSTANT (MAXPAGESIZE) - 1)));\n  .rodata         : { *(.rodata .rodata.* .gnu.linkonce.r.*) }\n  .rodata1        : { *(.rodata1) }\n  .eh_frame_hdr   : { *(.eh_frame_hdr) *(.eh_frame_entry .eh_frame_entry.*) }\n  .eh_frame       : ONLY_IF_RO { KEEP (*(.eh_frame)) *(.eh_frame.*) }\n  .gcc_except_table   : ONLY_IF_RO { *(.gcc_except_table .gcc_except_table.*) }\n  .gnu_extab   : ONLY_IF_RO { *(.gnu_extab*) }\n  /* These sections are generated by the Sun/Oracle C++ compiler.  */\n  .exception_ranges   : ONLY_IF_RO { *(.exception_ranges*) }\n  /* Adjust the address for the data segment.  We want to adjust up to\n     the same address within the page on the next page up.  */\n  . = DATA_SEGMENT_ALIGN (CONSTANT (MAXPAGESIZE), CONSTANT (COMMONPAGESIZE));\n  /* Exception handling  */\n  .eh_frame       : ONLY_IF_RW { KEEP (*(.eh_frame)) *(.eh_frame.*) }\n  .gnu_extab      : ONLY_IF_RW { *(.gnu_extab) }\n  .gcc_except_table   : ONLY_IF_RW { *(.gcc_except_table .gcc_except_table.*) }\n  .exception_ranges   : ONLY_IF_RW { *(.exception_ranges*) }\n  /* Thread Local Storage sections  */\n  .tdata\t  :\n   {\n     *(.tdata .tdata.* .gnu.linkonce.td.*)\n   }\n  .tbss\t\t  : { *(.tbss .tbss.* .gnu.linkonce.tb.*) *(.tcommon) }\n  .preinit_array    :\n  {\n    KEEP (*(.preinit_array))\n  }\n  .init_array    :\n  {\n    KEEP (*(SORT_BY_INIT_PRIORITY(.init_array.*) SORT_BY_INIT_PRIORITY(.ctors.*)))\n    KEEP (*(.init_array EXCLUDE_FILE (*crtbegin.o *crtbegin?.o *crtend.o *crtend?.o ) .ctors))\n  }\n  .fini_array    :\n  {\n    KEEP (*(SORT_BY_INIT_PRIORITY(.fini_array.*) SORT_BY_INIT_PRIORITY(.dtors.*)))\n    KEEP (*(.fini_array EXCLUDE_FILE (*crtbegin.o *crtbegin?.o *crtend.o *crtend?.o ) .dtors))\n  }\n  .ctors          :\n  {\n    /* gcc uses crtbegin.o to find the start of\n       the constructors, so we make sure it is\n       first.  Because this is a wildcard, it\n       doesn't matter if the user does not\n       actually link against crtbegin.o; the\n       linker won't look for a file to match a\n       wildcard.  The wildcard also means that it\n       doesn't matter which directory crtbegin.o\n       is in.  */\n    KEEP (*crtbegin.o(.ctors))\n    KEEP (*crtbegin?.o(.ctors))\n    /* We don't want to include the .ctor section from\n       the crtend.o file until after the sorted ctors.\n       The .ctor section from the crtend file contains the\n       end of ctors marker and it must be last */\n    KEEP (*(EXCLUDE_FILE (*crtend.o *crtend?.o ) .ctors))\n    KEEP (*(SORT(.ctors.*)))\n    KEEP (*(.ctors))\n  }\n  .dtors          :\n  {\n    KEEP (*crtbegin.o(.dtors))\n    KEEP (*crtbegin?.o(.dtors))\n    KEEP (*(EXCLUDE_FILE (*crtend.o *crtend?.o ) .dtors))\n    KEEP (*(SORT(.dtors.*)))\n    KEEP (*(.dtors))\n  }\n  .jcr            : { KEEP (*(.jcr)) }\n  .data.rel.ro : { *(.data.rel.ro.local* .gnu.linkonce.d.rel.ro.local.*) *(.data.rel.ro .data.rel.ro.* .gnu.linkonce.d.rel.ro.*) }\n  .dynamic        : { *(.dynamic) }\n  .got            : { *(.got) *(.igot) }\n  . = DATA_SEGMENT_RELRO_END (SIZEOF (.got.plt) >= 24 ? 24 : 0, .);\n  .got.plt        : { *(.got.plt) *(.igot.plt) }\n  .data           :\n  {\n    *(.data .data.* .gnu.linkonce.d.*)\n    SORT(CONSTRUCTORS)\n  }\n  .data1          : { *(.data1) }\n  PROVIDE (_edata = .); PROVIDE (edata = .);\n  . = .;\n  PROVIDE (__bss_start = .);\n  .bss            :\n  {\n   *(.dynbss)\n   *(.bss .bss.* .gnu.linkonce.b.*)\n   *(COMMON)\n   /* Align here to ensure that the .bss section occupies space up to\n      _end.  Align after .bss to ensure correct alignment even if the\n      .bss section disappears because there are no input sections.\n      FIXME: Why do we need it? When there is no .bss section, we do not\n      pad the .data section.  */\n   . = ALIGN(. != 0 ? 64 / 8 : 1);\n  }\n  .lbss   :\n  {\n    *(.dynlbss)\n    *(.lbss .lbss.* .gnu.linkonce.lb.*)\n    *(LARGE_COMMON)\n  }\n  . = ALIGN(64 / 8);\n  . = SEGMENT_START(\"ldata-segment\", .);\n  .lrodata   ALIGN(CONSTANT (MAXPAGESIZE)) + (. & (CONSTANT (MAXPAGESIZE) - 1)) :\n  {\n    *(.lrodata .lrodata.* .gnu.linkonce.lr.*)\n  }\n  .ldata   ALIGN(CONSTANT (MAXPAGESIZE)) + (. & (CONSTANT (MAXPAGESIZE) - 1)) :\n  {\n    *(.ldata .ldata.* .gnu.linkonce.l.*)\n    . = ALIGN(. != 0 ? 64 / 8 : 1);\n  }\n  . = ALIGN(64 / 8);\n  PROVIDE (_end = .); PROVIDE (end = .);\n  . = DATA_SEGMENT_END (.);\n  /* Stabs debugging sections.  */\n  .stab          0 : { *(.stab) }\n  .stabstr       0 : { *(.stabstr) }\n  .stab.excl     0 : { *(.stab.excl) }\n  .stab.exclstr  0 : { *(.stab.exclstr) }\n  .stab.index    0 : { *(.stab.index) }\n  .stab.indexstr 0 : { *(.stab.indexstr) }\n  .comment       0 : { *(.comment) }\n  .gnu.build.attributes : { *(.gnu.build.attributes .gnu.build.attributes.*) }\n  /* DWARF debug sections.\n     Symbols in the DWARF debugging sections are relative to the beginning\n     of the section so we begin them at 0.  */\n  /* DWARF 1.  */\n  .debug          0 : { *(.debug) }\n  .line           0 : { *(.line) }\n  /* GNU DWARF 1 extensions.  */\n  .debug_srcinfo  0 : { *(.debug_srcinfo) }\n  .debug_sfnames  0 : { *(.debug_sfnames) }\n  /* DWARF 1.1 and DWARF 2.  */\n  .debug_aranges  0 : { *(.debug_aranges) }\n  .debug_pubnames 0 : { *(.debug_pubnames) }\n  /* DWARF 2.  */\n  .debug_info     0 : { *(.debug_info .gnu.linkonce.wi.*) }\n  .debug_abbrev   0 : { *(.debug_abbrev) }\n  .debug_line     0 : { *(.debug_line .debug_line.* .debug_line_end) }\n  .debug_frame    0 : { *(.debug_frame) }\n  .debug_str      0 : { *(.debug_str) }\n  .debug_loc      0 : { *(.debug_loc) }\n  .debug_macinfo  0 : { *(.debug_macinfo) }\n  /* SGI/MIPS DWARF 2 extensions.  */\n  .debug_weaknames 0 : { *(.debug_weaknames) }\n  .debug_funcnames 0 : { *(.debug_funcnames) }\n  .debug_typenames 0 : { *(.debug_typenames) }\n  .debug_varnames  0 : { *(.debug_varnames) }\n  /* DWARF 3.  */\n  .debug_pubtypes 0 : { *(.debug_pubtypes) }\n  .debug_ranges   0 : { *(.debug_ranges) }\n  /* DWARF 5.  */\n  .debug_addr     0 : { *(.debug_addr) }\n  .debug_line_str 0 : { *(.debug_line_str) }\n  .debug_loclists 0 : { *(.debug_loclists) }\n  .debug_macro    0 : { *(.debug_macro) }\n  .debug_names    0 : { *(.debug_names) }\n  .debug_rnglists 0 : { *(.debug_rnglists) }\n  .debug_str_offsets 0 : { *(.debug_str_offsets) }\n  .debug_sup      0 : { *(.debug_sup) }\n  .gnu.attributes 0 : { KEEP (*(.gnu.attributes)) }\n  /DISCARD/ : { *(.note.GNU-stack) *(.gnu_debuglink) *(.gnu.lto_*) }\n}\n\n\0"
            as *const u8 as *const libc::c_char as *mut libc::c_char
    } else if link_info.type_0() as libc::c_int == type_dll as libc::c_int {
        return b"/* Script for -shared */\n/* Copyright (C) 2014-2021 Free Software Foundation, Inc.\n   Copying and distribution of this script, with or without modification,\n   are permitted in any medium without royalty provided the copyright\n   notice and this notice are preserved.  */\nOUTPUT_FORMAT(\"elf64-x86-64\", \"elf64-x86-64\",\n\t      \"elf64-x86-64\")\nOUTPUT_ARCH(i386:x86-64)\nENTRY(_start)\nSEARCH_DIR(\"=/usr/local/x86_64-pc-linux-gnu/lib64\"); SEARCH_DIR(\"=/usr/local/lib64\"); SEARCH_DIR(\"=/lib64\"); SEARCH_DIR(\"=/usr/lib64\"); SEARCH_DIR(\"=/usr/local/x86_64-pc-linux-gnu/lib\"); SEARCH_DIR(\"=/usr/local/lib\"); SEARCH_DIR(\"=/lib\"); SEARCH_DIR(\"=/usr/lib\");\nSECTIONS\n{\n  /* Read-only sections, merged into text segment: */\n  . = SEGMENT_START(\"text-segment\", 0) + SIZEOF_HEADERS;\n  .note.gnu.build-id  : { *(.note.gnu.build-id) }\n  .hash           : { *(.hash) }\n  .gnu.hash       : { *(.gnu.hash) }\n  .dynsym         : { *(.dynsym) }\n  .dynstr         : { *(.dynstr) }\n  .gnu.version    : { *(.gnu.version) }\n  .gnu.version_d  : { *(.gnu.version_d) }\n  .gnu.version_r  : { *(.gnu.version_r) }\n  .rela.init      : { *(.rela.init) }\n  .rela.text      : { *(.rela.text .rela.text.* .rela.gnu.linkonce.t.*) }\n  .rela.fini      : { *(.rela.fini) }\n  .rela.rodata    : { *(.rela.rodata .rela.rodata.* .rela.gnu.linkonce.r.*) }\n  .rela.data.rel.ro   : { *(.rela.data.rel.ro .rela.data.rel.ro.* .rela.gnu.linkonce.d.rel.ro.*) }\n  .rela.data      : { *(.rela.data .rela.data.* .rela.gnu.linkonce.d.*) }\n  .rela.tdata\t  : { *(.rela.tdata .rela.tdata.* .rela.gnu.linkonce.td.*) }\n  .rela.tbss\t  : { *(.rela.tbss .rela.tbss.* .rela.gnu.linkonce.tb.*) }\n  .rela.ctors     : { *(.rela.ctors) }\n  .rela.dtors     : { *(.rela.dtors) }\n  .rela.got       : { *(.rela.got) }\n  .rela.bss       : { *(.rela.bss .rela.bss.* .rela.gnu.linkonce.b.*) }\n  .rela.ldata     : { *(.rela.ldata .rela.ldata.* .rela.gnu.linkonce.l.*) }\n  .rela.lbss      : { *(.rela.lbss .rela.lbss.* .rela.gnu.linkonce.lb.*) }\n  .rela.lrodata   : { *(.rela.lrodata .rela.lrodata.* .rela.gnu.linkonce.lr.*) }\n  .rela.ifunc     : { *(.rela.ifunc) }\n  .rela.plt       :\n    {\n      *(.rela.plt)\n      *(.rela.iplt)\n    }\n  .init           :\n  {\n    KEEP (*(SORT_NONE(.init)))\n  }\n  .plt            : { *(.plt) *(.iplt) }\n.plt.got        : { *(.plt.got) }\n.plt.sec        : { *(.plt.sec) }\n  .text           :\n  {\n    *(.text.unlikely .text.*_unlikely .text.unlikely.*)\n    *(.text.exit .text.exit.*)\n    *(.text.startup .text.startup.*)\n    *(.text.hot .text.hot.*)\n    *(SORT(.text.sorted.*))\n    *(.text .stub .text.* .gnu.linkonce.t.*)\n    /* .gnu.warning sections are handled specially by elf.em.  */\n    *(.gnu.warning)\n  }\n  .fini           :\n  {\n    KEEP (*(SORT_NONE(.fini)))\n  }\n  PROVIDE (__etext = .);\n  PROVIDE (_etext = .);\n  PROVIDE (etext = .);\n  .rodata         : { *(.rodata .rodata.* .gnu.linkonce.r.*) }\n  .rodata1        : { *(.rodata1) }\n  .eh_frame_hdr   : { *(.eh_frame_hdr) *(.eh_frame_entry .eh_frame_entry.*) }\n  .eh_frame       : ONLY_IF_RO { KEEP (*(.eh_frame)) *(.eh_frame.*) }\n  .gcc_except_table   : ONLY_IF_RO { *(.gcc_except_table .gcc_except_table.*) }\n  .gnu_extab   : ONLY_IF_RO { *(.gnu_extab*) }\n  /* These sections are generated by the Sun/Oracle C++ compiler.  */\n  .exception_ranges   : ONLY_IF_RO { *(.exception_ranges*) }\n  /* Adjust the address for the data segment.  We want to adjust up to\n     the same address within the page on the next page up.  */\n  . = DATA_SEGMENT_ALIGN (CONSTANT (MAXPAGESIZE), CONSTANT (COMMONPAGESIZE));\n  /* Exception handling  */\n  .eh_frame       : ONLY_IF_RW { KEEP (*(.eh_frame)) *(.eh_frame.*) }\n  .gnu_extab      : ONLY_IF_RW { *(.gnu_extab) }\n  .gcc_except_table   : ONLY_IF_RW { *(.gcc_except_table .gcc_except_table.*) }\n  .exception_ranges   : ONLY_IF_RW { *(.exception_ranges*) }\n  /* Thread Local Storage sections  */\n  .tdata\t  :\n   {\n     *(.tdata .tdata.* .gnu.linkonce.td.*)\n   }\n  .tbss\t\t  : { *(.tbss .tbss.* .gnu.linkonce.tb.*) *(.tcommon) }\n  .preinit_array    :\n  {\n    KEEP (*(.preinit_array))\n  }\n  .init_array    :\n  {\n    KEEP (*(SORT_BY_INIT_PRIORITY(.init_array.*) SORT_BY_INIT_PRIORITY(.ctors.*)))\n    KEEP (*(.init_array EXCLUDE_FILE (*crtbegin.o *crtbegin?.o *crtend.o *crtend?.o ) .ctors))\n  }\n  .fini_array    :\n  {\n    KEEP (*(SORT_BY_INIT_PRIORITY(.fini_array.*) SORT_BY_INIT_PRIORITY(.dtors.*)))\n    KEEP (*(.fini_array EXCLUDE_FILE (*crtbegin.o *crtbegin?.o *crtend.o *crtend?.o ) .dtors))\n  }\n  .ctors          :\n  {\n    /* gcc uses crtbegin.o to find the start of\n       the constructors, so we make sure it is\n       first.  Because this is a wildcard, it\n       doesn't matter if the user does not\n       actually link against crtbegin.o; the\n       linker won't look for a file to match a\n       wildcard.  The wildcard also means that it\n       doesn't matter which directory crtbegin.o\n       is in.  */\n    KEEP (*crtbegin.o(.ctors))\n    KEEP (*crtbegin?.o(.ctors))\n    /* We don't want to include the .ctor section from\n       the crtend.o file until after the sorted ctors.\n       The .ctor section from the crtend file contains the\n       end of ctors marker and it must be last */\n    KEEP (*(EXCLUDE_FILE (*crtend.o *crtend?.o ) .ctors))\n    KEEP (*(SORT(.ctors.*)))\n    KEEP (*(.ctors))\n  }\n  .dtors          :\n  {\n    KEEP (*crtbegin.o(.dtors))\n    KEEP (*crtbegin?.o(.dtors))\n    KEEP (*(EXCLUDE_FILE (*crtend.o *crtend?.o ) .dtors))\n    KEEP (*(SORT(.dtors.*)))\n    KEEP (*(.dtors))\n  }\n  .jcr            : { KEEP (*(.jcr)) }\n  .data.rel.ro : { *(.data.rel.ro.local* .gnu.linkonce.d.rel.ro.local.*) *(.data.rel.ro .data.rel.ro.* .gnu.linkonce.d.rel.ro.*) }\n  .dynamic        : { *(.dynamic) }\n  .got            : { *(.got) *(.igot) }\n  . = DATA_SEGMENT_RELRO_END (SIZEOF (.got.plt) >= 24 ? 24 : 0, .);\n  .got.plt        : { *(.got.plt) *(.igot.plt) }\n  .data           :\n  {\n    *(.data .data.* .gnu.linkonce.d.*)\n    SORT(CONSTRUCTORS)\n  }\n  .data1          : { *(.data1) }\n  PROVIDE (_edata = .); PROVIDE (edata = .);\n  . = .;\n  PROVIDE (__bss_start = .);\n  .bss            :\n  {\n   *(.dynbss)\n   *(.bss .bss.* .gnu.linkonce.b.*)\n   *(COMMON)\n   /* Align here to ensure that the .bss section occupies space up to\n      _end.  Align after .bss to ensure correct alignment even if the\n      .bss section disappears because there are no input sections.\n      FIXME: Why do we need it? When there is no .bss section, we do not\n      pad the .data section.  */\n   . = ALIGN(. != 0 ? 64 / 8 : 1);\n  }\n  .lbss   :\n  {\n    *(.dynlbss)\n    *(.lbss .lbss.* .gnu.linkonce.lb.*)\n    *(LARGE_COMMON)\n  }\n  . = ALIGN(64 / 8);\n  . = SEGMENT_START(\"ldata-segment\", .);\n  .lrodata   ALIGN(CONSTANT (MAXPAGESIZE)) + (. & (CONSTANT (MAXPAGESIZE) - 1)) :\n  {\n    *(.lrodata .lrodata.* .gnu.linkonce.lr.*)\n  }\n  .ldata   ALIGN(CONSTANT (MAXPAGESIZE)) + (. & (CONSTANT (MAXPAGESIZE) - 1)) :\n  {\n    *(.ldata .ldata.* .gnu.linkonce.l.*)\n    . = ALIGN(. != 0 ? 64 / 8 : 1);\n  }\n  . = ALIGN(64 / 8);\n  PROVIDE (_end = .); PROVIDE (end = .);\n  . = DATA_SEGMENT_END (.);\n  /* Stabs debugging sections.  */\n  .stab          0 : { *(.stab) }\n  .stabstr       0 : { *(.stabstr) }\n  .stab.excl     0 : { *(.stab.excl) }\n  .stab.exclstr  0 : { *(.stab.exclstr) }\n  .stab.index    0 : { *(.stab.index) }\n  .stab.indexstr 0 : { *(.stab.indexstr) }\n  .comment       0 : { *(.comment) }\n  .gnu.build.attributes : { *(.gnu.build.attributes .gnu.build.attributes.*) }\n  /* DWARF debug sections.\n     Symbols in the DWARF debugging sections are relative to the beginning\n     of the section so we begin them at 0.  */\n  /* DWARF 1.  */\n  .debug          0 : { *(.debug) }\n  .line           0 : { *(.line) }\n  /* GNU DWARF 1 extensions.  */\n  .debug_srcinfo  0 : { *(.debug_srcinfo) }\n  .debug_sfnames  0 : { *(.debug_sfnames) }\n  /* DWARF 1.1 and DWARF 2.  */\n  .debug_aranges  0 : { *(.debug_aranges) }\n  .debug_pubnames 0 : { *(.debug_pubnames) }\n  /* DWARF 2.  */\n  .debug_info     0 : { *(.debug_info .gnu.linkonce.wi.*) }\n  .debug_abbrev   0 : { *(.debug_abbrev) }\n  .debug_line     0 : { *(.debug_line .debug_line.* .debug_line_end) }\n  .debug_frame    0 : { *(.debug_frame) }\n  .debug_str      0 : { *(.debug_str) }\n  .debug_loc      0 : { *(.debug_loc) }\n  .debug_macinfo  0 : { *(.debug_macinfo) }\n  /* SGI/MIPS DWARF 2 extensions.  */\n  .debug_weaknames 0 : { *(.debug_weaknames) }\n  .debug_funcnames 0 : { *(.debug_funcnames) }\n  .debug_typenames 0 : { *(.debug_typenames) }\n  .debug_varnames  0 : { *(.debug_varnames) }\n  /* DWARF 3.  */\n  .debug_pubtypes 0 : { *(.debug_pubtypes) }\n  .debug_ranges   0 : { *(.debug_ranges) }\n  /* DWARF 5.  */\n  .debug_addr     0 : { *(.debug_addr) }\n  .debug_line_str 0 : { *(.debug_line_str) }\n  .debug_loclists 0 : { *(.debug_loclists) }\n  .debug_macro    0 : { *(.debug_macro) }\n  .debug_names    0 : { *(.debug_names) }\n  .debug_rnglists 0 : { *(.debug_rnglists) }\n  .debug_str_offsets 0 : { *(.debug_str_offsets) }\n  .debug_sup      0 : { *(.debug_sup) }\n  .gnu.attributes 0 : { KEEP (*(.gnu.attributes)) }\n  /DISCARD/ : { *(.note.GNU-stack) *(.gnu_debuglink) *(.gnu.lto_*) }\n}\n\n\0"
            as *const u8 as *const libc::c_char as *mut libc::c_char
    } else if link_info.combreloc() as libc::c_int != 0
        && link_info.separate_code() as libc::c_int != 0
        && link_info.flags & ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_ulong
            != 0
    {
        return b"/* Script for -z combreloc -z separate-code -z relro -z now */\n/* Copyright (C) 2014-2021 Free Software Foundation, Inc.\n   Copying and distribution of this script, with or without modification,\n   are permitted in any medium without royalty provided the copyright\n   notice and this notice are preserved.  */\nOUTPUT_FORMAT(\"elf64-x86-64\", \"elf64-x86-64\",\n\t      \"elf64-x86-64\")\nOUTPUT_ARCH(i386:x86-64)\nENTRY(_start)\nSEARCH_DIR(\"=/usr/local/x86_64-pc-linux-gnu/lib64\"); SEARCH_DIR(\"=/usr/local/lib64\"); SEARCH_DIR(\"=/lib64\"); SEARCH_DIR(\"=/usr/lib64\"); SEARCH_DIR(\"=/usr/local/x86_64-pc-linux-gnu/lib\"); SEARCH_DIR(\"=/usr/local/lib\"); SEARCH_DIR(\"=/lib\"); SEARCH_DIR(\"=/usr/lib\");\nSECTIONS\n{\n  PROVIDE (__executable_start = SEGMENT_START(\"text-segment\", 0x400000)); . = SEGMENT_START(\"text-segment\", 0x400000) + SIZEOF_HEADERS;\n  .interp         : { *(.interp) }\n  .note.gnu.build-id  : { *(.note.gnu.build-id) }\n  .hash           : { *(.hash) }\n  .gnu.hash       : { *(.gnu.hash) }\n  .dynsym         : { *(.dynsym) }\n  .dynstr         : { *(.dynstr) }\n  .gnu.version    : { *(.gnu.version) }\n  .gnu.version_d  : { *(.gnu.version_d) }\n  .gnu.version_r  : { *(.gnu.version_r) }\n  .rela.dyn       :\n    {\n      *(.rela.init)\n      *(.rela.text .rela.text.* .rela.gnu.linkonce.t.*)\n      *(.rela.fini)\n      *(.rela.rodata .rela.rodata.* .rela.gnu.linkonce.r.*)\n      *(.rela.data .rela.data.* .rela.gnu.linkonce.d.*)\n      *(.rela.tdata .rela.tdata.* .rela.gnu.linkonce.td.*)\n      *(.rela.tbss .rela.tbss.* .rela.gnu.linkonce.tb.*)\n      *(.rela.ctors)\n      *(.rela.dtors)\n      *(.rela.got)\n      *(.rela.bss .rela.bss.* .rela.gnu.linkonce.b.*)\n      *(.rela.ldata .rela.ldata.* .rela.gnu.linkonce.l.*)\n      *(.rela.lbss .rela.lbss.* .rela.gnu.linkonce.lb.*)\n      *(.rela.lrodata .rela.lrodata.* .rela.gnu.linkonce.lr.*)\n      *(.rela.ifunc)\n    }\n  .rela.plt       :\n    {\n      *(.rela.plt)\n      PROVIDE_HIDDEN (__rela_iplt_start = .);\n      *(.rela.iplt)\n      PROVIDE_HIDDEN (__rela_iplt_end = .);\n    }\n  . = ALIGN(CONSTANT (MAXPAGESIZE));\n  .init           :\n  {\n    KEEP (*(SORT_NONE(.init)))\n  }\n  .plt            : { *(.plt) *(.iplt) }\n.plt.got        : { *(.plt.got) }\n.plt.sec        : { *(.plt.sec) }\n  .text           :\n  {\n    *(.text.unlikely .text.*_unlikely .text.unlikely.*)\n    *(.text.exit .text.exit.*)\n    *(.text.startup .text.startup.*)\n    *(.text.hot .text.hot.*)\n    *(SORT(.text.sorted.*))\n    *(.text .stub .text.* .gnu.linkonce.t.*)\n    /* .gnu.warning sections are handled specially by elf.em.  */\n    *(.gnu.warning)\n  }\n  .fini           :\n  {\n    KEEP (*(SORT_NONE(.fini)))\n  }\n  PROVIDE (__etext = .);\n  PROVIDE (_etext = .);\n  PROVIDE (etext = .);\n  . = ALIGN(CONSTANT (MAXPAGESIZE));\n  /* Adjust the address for the rodata segment.  We want to adjust up to\n     the same address within the page on the next page up.  */\n  . = SEGMENT_START(\"rodata-segment\", ALIGN(CONSTANT (MAXPAGESIZE)) + (. & (CONSTANT (MAXPAGESIZE) - 1)));\n  .rodata         : { *(.rodata .rodata.* .gnu.linkonce.r.*) }\n  .rodata1        : { *(.rodata1) }\n  .eh_frame_hdr   : { *(.eh_frame_hdr) *(.eh_frame_entry .eh_frame_entry.*) }\n  .eh_frame       : ONLY_IF_RO { KEEP (*(.eh_frame)) *(.eh_frame.*) }\n  .gcc_except_table   : ONLY_IF_RO { *(.gcc_except_table .gcc_except_table.*) }\n  .gnu_extab   : ONLY_IF_RO { *(.gnu_extab*) }\n  /* These sections are generated by the Sun/Oracle C++ compiler.  */\n  .exception_ranges   : ONLY_IF_RO { *(.exception_ranges*) }\n  /* Adjust the address for the data segment.  We want to adjust up to\n     the same address within the page on the next page up.  */\n  . = DATA_SEGMENT_ALIGN (CONSTANT (MAXPAGESIZE), CONSTANT (COMMONPAGESIZE));\n  /* Exception handling  */\n  .eh_frame       : ONLY_IF_RW { KEEP (*(.eh_frame)) *(.eh_frame.*) }\n  .gnu_extab      : ONLY_IF_RW { *(.gnu_extab) }\n  .gcc_except_table   : ONLY_IF_RW { *(.gcc_except_table .gcc_except_table.*) }\n  .exception_ranges   : ONLY_IF_RW { *(.exception_ranges*) }\n  /* Thread Local Storage sections  */\n  .tdata\t  :\n   {\n     PROVIDE_HIDDEN (__tdata_start = .);\n     *(.tdata .tdata.* .gnu.linkonce.td.*)\n   }\n  .tbss\t\t  : { *(.tbss .tbss.* .gnu.linkonce.tb.*) *(.tcommon) }\n  .preinit_array    :\n  {\n    PROVIDE_HIDDEN (__preinit_array_start = .);\n    KEEP (*(.preinit_array))\n    PROVIDE_HIDDEN (__preinit_array_end = .);\n  }\n  .init_array    :\n  {\n    PROVIDE_HIDDEN (__init_array_start = .);\n    KEEP (*(SORT_BY_INIT_PRIORITY(.init_array.*) SORT_BY_INIT_PRIORITY(.ctors.*)))\n    KEEP (*(.init_array EXCLUDE_FILE (*crtbegin.o *crtbegin?.o *crtend.o *crtend?.o ) .ctors))\n    PROVIDE_HIDDEN (__init_array_end = .);\n  }\n  .fini_array    :\n  {\n    PROVIDE_HIDDEN (__fini_array_start = .);\n    KEEP (*(SORT_BY_INIT_PRIORITY(.fini_array.*) SORT_BY_INIT_PRIORITY(.dtors.*)))\n    KEEP (*(.fini_array EXCLUDE_FILE (*crtbegin.o *crtbegin?.o *crtend.o *crtend?.o ) .dtors))\n    PROVIDE_HIDDEN (__fini_array_end = .);\n  }\n  .ctors          :\n  {\n    /* gcc uses crtbegin.o to find the start of\n       the constructors, so we make sure it is\n       first.  Because this is a wildcard, it\n       doesn't matter if the user does not\n       actually link against crtbegin.o; the\n       linker won't look for a file to match a\n       wildcard.  The wildcard also means that it\n       doesn't matter which directory crtbegin.o\n       is in.  */\n    KEEP (*crtbegin.o(.ctors))\n    KEEP (*crtbegin?.o(.ctors))\n    /* We don't want to include the .ctor section from\n       the crtend.o file until after the sorted ctors.\n       The .ctor section from the crtend file contains the\n       end of ctors marker and it must be last */\n    KEEP (*(EXCLUDE_FILE (*crtend.o *crtend?.o ) .ctors))\n    KEEP (*(SORT(.ctors.*)))\n    KEEP (*(.ctors))\n  }\n  .dtors          :\n  {\n    KEEP (*crtbegin.o(.dtors))\n    KEEP (*crtbegin?.o(.dtors))\n    KEEP (*(EXCLUDE_FILE (*crtend.o *crtend?.o ) .dtors))\n    KEEP (*(SORT(.dtors.*)))\n    KEEP (*(.dtors))\n  }\n  .jcr            : { KEEP (*(.jcr)) }\n  .data.rel.ro : { *(.data.rel.ro.local* .gnu.linkonce.d.rel.ro.local.*) *(.data.rel.ro .data.rel.ro.* .gnu.linkonce.d.rel.ro.*) }\n  .dynamic        : { *(.dynamic) }\n  .got            : { *(.got.plt) *(.igot.plt) *(.got) *(.igot) }\n  . = DATA_SEGMENT_RELRO_END (0, .);\n  .data           :\n  {\n    *(.data .data.* .gnu.linkonce.d.*)\n    SORT(CONSTRUCTORS)\n  }\n  .data1          : { *(.data1) }\n  _edata = .; PROVIDE (edata = .);\n  . = .;\n  __bss_start = .;\n  .bss            :\n  {\n   *(.dynbss)\n   *(.bss .bss.* .gnu.linkonce.b.*)\n   *(COMMON)\n   /* Align here to ensure that the .bss section occupies space up to\n      _end.  Align after .bss to ensure correct alignment even if the\n      .bss section disappears because there are no input sections.\n      FIXME: Why do we need it? When there is no .bss section, we do not\n      pad the .data section.  */\n   . = ALIGN(. != 0 ? 64 / 8 : 1);\n  }\n  .lbss   :\n  {\n    *(.dynlbss)\n    *(.lbss .lbss.* .gnu.linkonce.lb.*)\n    *(LARGE_COMMON)\n  }\n  . = ALIGN(64 / 8);\n  . = SEGMENT_START(\"ldata-segment\", .);\n  .lrodata   ALIGN(CONSTANT (MAXPAGESIZE)) + (. & (CONSTANT (MAXPAGESIZE) - 1)) :\n  {\n    *(.lrodata .lrodata.* .gnu.linkonce.lr.*)\n  }\n  .ldata   ALIGN(CONSTANT (MAXPAGESIZE)) + (. & (CONSTANT (MAXPAGESIZE) - 1)) :\n  {\n    *(.ldata .ldata.* .gnu.linkonce.l.*)\n    . = ALIGN(. != 0 ? 64 / 8 : 1);\n  }\n  . = ALIGN(64 / 8);\n  _end = .; PROVIDE (end = .);\n  . = DATA_SEGMENT_END (.);\n  /* Stabs debugging sections.  */\n  .stab          0 : { *(.stab) }\n  .stabstr       0 : { *(.stabstr) }\n  .stab.excl     0 : { *(.stab.excl) }\n  .stab.exclstr  0 : { *(.stab.exclstr) }\n  .stab.index    0 : { *(.stab.index) }\n  .stab.indexstr 0 : { *(.stab.indexstr) }\n  .comment       0 : { *(.comment) }\n  .gnu.build.attributes : { *(.gnu.build.attributes .gnu.build.attributes.*) }\n  /* DWARF debug sections.\n     Symbols in the DWARF debugging sections are relative to the beginning\n     of the section so we begin them at 0.  */\n  /* DWARF 1.  */\n  .debug          0 : { *(.debug) }\n  .line           0 : { *(.line) }\n  /* GNU DWARF 1 extensions.  */\n  .debug_srcinfo  0 : { *(.debug_srcinfo) }\n  .debug_sfnames  0 : { *(.debug_sfnames) }\n  /* DWARF 1.1 and DWARF 2.  */\n  .debug_aranges  0 : { *(.debug_aranges) }\n  .debug_pubnames 0 : { *(.debug_pubnames) }\n  /* DWARF 2.  */\n  .debug_info     0 : { *(.debug_info .gnu.linkonce.wi.*) }\n  .debug_abbrev   0 : { *(.debug_abbrev) }\n  .debug_line     0 : { *(.debug_line .debug_line.* .debug_line_end) }\n  .debug_frame    0 : { *(.debug_frame) }\n  .debug_str      0 : { *(.debug_str) }\n  .debug_loc      0 : { *(.debug_loc) }\n  .debug_macinfo  0 : { *(.debug_macinfo) }\n  /* SGI/MIPS DWARF 2 extensions.  */\n  .debug_weaknames 0 : { *(.debug_weaknames) }\n  .debug_funcnames 0 : { *(.debug_funcnames) }\n  .debug_typenames 0 : { *(.debug_typenames) }\n  .debug_varnames  0 : { *(.debug_varnames) }\n  /* DWARF 3.  */\n  .debug_pubtypes 0 : { *(.debug_pubtypes) }\n  .debug_ranges   0 : { *(.debug_ranges) }\n  /* DWARF 5.  */\n  .debug_addr     0 : { *(.debug_addr) }\n  .debug_line_str 0 : { *(.debug_line_str) }\n  .debug_loclists 0 : { *(.debug_loclists) }\n  .debug_macro    0 : { *(.debug_macro) }\n  .debug_names    0 : { *(.debug_names) }\n  .debug_rnglists 0 : { *(.debug_rnglists) }\n  .debug_str_offsets 0 : { *(.debug_str_offsets) }\n  .debug_sup      0 : { *(.debug_sup) }\n  .gnu.attributes 0 : { KEEP (*(.gnu.attributes)) }\n  /DISCARD/ : { *(.note.GNU-stack) *(.gnu_debuglink) *(.gnu.lto_*) }\n}\n\n\0"
            as *const u8 as *const libc::c_char as *mut libc::c_char
    } else if link_info.combreloc() as libc::c_int != 0
        && link_info.relro() as libc::c_int != 0
        && link_info.flags & ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_ulong
            != 0
    {
        return b"/* Script for -z combreloc -z relro -z now */\n/* Copyright (C) 2014-2021 Free Software Foundation, Inc.\n   Copying and distribution of this script, with or without modification,\n   are permitted in any medium without royalty provided the copyright\n   notice and this notice are preserved.  */\nOUTPUT_FORMAT(\"elf64-x86-64\", \"elf64-x86-64\",\n\t      \"elf64-x86-64\")\nOUTPUT_ARCH(i386:x86-64)\nENTRY(_start)\nSEARCH_DIR(\"=/usr/local/x86_64-pc-linux-gnu/lib64\"); SEARCH_DIR(\"=/usr/local/lib64\"); SEARCH_DIR(\"=/lib64\"); SEARCH_DIR(\"=/usr/lib64\"); SEARCH_DIR(\"=/usr/local/x86_64-pc-linux-gnu/lib\"); SEARCH_DIR(\"=/usr/local/lib\"); SEARCH_DIR(\"=/lib\"); SEARCH_DIR(\"=/usr/lib\");\nSECTIONS\n{\n  /* Read-only sections, merged into text segment: */\n  PROVIDE (__executable_start = SEGMENT_START(\"text-segment\", 0x400000)); . = SEGMENT_START(\"text-segment\", 0x400000) + SIZEOF_HEADERS;\n  .interp         : { *(.interp) }\n  .note.gnu.build-id  : { *(.note.gnu.build-id) }\n  .hash           : { *(.hash) }\n  .gnu.hash       : { *(.gnu.hash) }\n  .dynsym         : { *(.dynsym) }\n  .dynstr         : { *(.dynstr) }\n  .gnu.version    : { *(.gnu.version) }\n  .gnu.version_d  : { *(.gnu.version_d) }\n  .gnu.version_r  : { *(.gnu.version_r) }\n  .rela.dyn       :\n    {\n      *(.rela.init)\n      *(.rela.text .rela.text.* .rela.gnu.linkonce.t.*)\n      *(.rela.fini)\n      *(.rela.rodata .rela.rodata.* .rela.gnu.linkonce.r.*)\n      *(.rela.data .rela.data.* .rela.gnu.linkonce.d.*)\n      *(.rela.tdata .rela.tdata.* .rela.gnu.linkonce.td.*)\n      *(.rela.tbss .rela.tbss.* .rela.gnu.linkonce.tb.*)\n      *(.rela.ctors)\n      *(.rela.dtors)\n      *(.rela.got)\n      *(.rela.bss .rela.bss.* .rela.gnu.linkonce.b.*)\n      *(.rela.ldata .rela.ldata.* .rela.gnu.linkonce.l.*)\n      *(.rela.lbss .rela.lbss.* .rela.gnu.linkonce.lb.*)\n      *(.rela.lrodata .rela.lrodata.* .rela.gnu.linkonce.lr.*)\n      *(.rela.ifunc)\n    }\n  .rela.plt       :\n    {\n      *(.rela.plt)\n      PROVIDE_HIDDEN (__rela_iplt_start = .);\n      *(.rela.iplt)\n      PROVIDE_HIDDEN (__rela_iplt_end = .);\n    }\n  .init           :\n  {\n    KEEP (*(SORT_NONE(.init)))\n  }\n  .plt            : { *(.plt) *(.iplt) }\n.plt.got        : { *(.plt.got) }\n.plt.sec        : { *(.plt.sec) }\n  .text           :\n  {\n    *(.text.unlikely .text.*_unlikely .text.unlikely.*)\n    *(.text.exit .text.exit.*)\n    *(.text.startup .text.startup.*)\n    *(.text.hot .text.hot.*)\n    *(SORT(.text.sorted.*))\n    *(.text .stub .text.* .gnu.linkonce.t.*)\n    /* .gnu.warning sections are handled specially by elf.em.  */\n    *(.gnu.warning)\n  }\n  .fini           :\n  {\n    KEEP (*(SORT_NONE(.fini)))\n  }\n  PROVIDE (__etext = .);\n  PROVIDE (_etext = .);\n  PROVIDE (etext = .);\n  .rodata         : { *(.rodata .rodata.* .gnu.linkonce.r.*) }\n  .rodata1        : { *(.rodata1) }\n  .eh_frame_hdr   : { *(.eh_frame_hdr) *(.eh_frame_entry .eh_frame_entry.*) }\n  .eh_frame       : ONLY_IF_RO { KEEP (*(.eh_frame)) *(.eh_frame.*) }\n  .gcc_except_table   : ONLY_IF_RO { *(.gcc_except_table .gcc_except_table.*) }\n  .gnu_extab   : ONLY_IF_RO { *(.gnu_extab*) }\n  /* These sections are generated by the Sun/Oracle C++ compiler.  */\n  .exception_ranges   : ONLY_IF_RO { *(.exception_ranges*) }\n  /* Adjust the address for the data segment.  We want to adjust up to\n     the same address within the page on the next page up.  */\n  . = DATA_SEGMENT_ALIGN (CONSTANT (MAXPAGESIZE), CONSTANT (COMMONPAGESIZE));\n  /* Exception handling  */\n  .eh_frame       : ONLY_IF_RW { KEEP (*(.eh_frame)) *(.eh_frame.*) }\n  .gnu_extab      : ONLY_IF_RW { *(.gnu_extab) }\n  .gcc_except_table   : ONLY_IF_RW { *(.gcc_except_table .gcc_except_table.*) }\n  .exception_ranges   : ONLY_IF_RW { *(.exception_ranges*) }\n  /* Thread Local Storage sections  */\n  .tdata\t  :\n   {\n     PROVIDE_HIDDEN (__tdata_start = .);\n     *(.tdata .tdata.* .gnu.linkonce.td.*)\n   }\n  .tbss\t\t  : { *(.tbss .tbss.* .gnu.linkonce.tb.*) *(.tcommon) }\n  .preinit_array    :\n  {\n    PROVIDE_HIDDEN (__preinit_array_start = .);\n    KEEP (*(.preinit_array))\n    PROVIDE_HIDDEN (__preinit_array_end = .);\n  }\n  .init_array    :\n  {\n    PROVIDE_HIDDEN (__init_array_start = .);\n    KEEP (*(SORT_BY_INIT_PRIORITY(.init_array.*) SORT_BY_INIT_PRIORITY(.ctors.*)))\n    KEEP (*(.init_array EXCLUDE_FILE (*crtbegin.o *crtbegin?.o *crtend.o *crtend?.o ) .ctors))\n    PROVIDE_HIDDEN (__init_array_end = .);\n  }\n  .fini_array    :\n  {\n    PROVIDE_HIDDEN (__fini_array_start = .);\n    KEEP (*(SORT_BY_INIT_PRIORITY(.fini_array.*) SORT_BY_INIT_PRIORITY(.dtors.*)))\n    KEEP (*(.fini_array EXCLUDE_FILE (*crtbegin.o *crtbegin?.o *crtend.o *crtend?.o ) .dtors))\n    PROVIDE_HIDDEN (__fini_array_end = .);\n  }\n  .ctors          :\n  {\n    /* gcc uses crtbegin.o to find the start of\n       the constructors, so we make sure it is\n       first.  Because this is a wildcard, it\n       doesn't matter if the user does not\n       actually link against crtbegin.o; the\n       linker won't look for a file to match a\n       wildcard.  The wildcard also means that it\n       doesn't matter which directory crtbegin.o\n       is in.  */\n    KEEP (*crtbegin.o(.ctors))\n    KEEP (*crtbegin?.o(.ctors))\n    /* We don't want to include the .ctor section from\n       the crtend.o file until after the sorted ctors.\n       The .ctor section from the crtend file contains the\n       end of ctors marker and it must be last */\n    KEEP (*(EXCLUDE_FILE (*crtend.o *crtend?.o ) .ctors))\n    KEEP (*(SORT(.ctors.*)))\n    KEEP (*(.ctors))\n  }\n  .dtors          :\n  {\n    KEEP (*crtbegin.o(.dtors))\n    KEEP (*crtbegin?.o(.dtors))\n    KEEP (*(EXCLUDE_FILE (*crtend.o *crtend?.o ) .dtors))\n    KEEP (*(SORT(.dtors.*)))\n    KEEP (*(.dtors))\n  }\n  .jcr            : { KEEP (*(.jcr)) }\n  .data.rel.ro : { *(.data.rel.ro.local* .gnu.linkonce.d.rel.ro.local.*) *(.data.rel.ro .data.rel.ro.* .gnu.linkonce.d.rel.ro.*) }\n  .dynamic        : { *(.dynamic) }\n  .got            : { *(.got.plt) *(.igot.plt) *(.got) *(.igot) }\n  . = DATA_SEGMENT_RELRO_END (0, .);\n  .data           :\n  {\n    *(.data .data.* .gnu.linkonce.d.*)\n    SORT(CONSTRUCTORS)\n  }\n  .data1          : { *(.data1) }\n  _edata = .; PROVIDE (edata = .);\n  . = .;\n  __bss_start = .;\n  .bss            :\n  {\n   *(.dynbss)\n   *(.bss .bss.* .gnu.linkonce.b.*)\n   *(COMMON)\n   /* Align here to ensure that the .bss section occupies space up to\n      _end.  Align after .bss to ensure correct alignment even if the\n      .bss section disappears because there are no input sections.\n      FIXME: Why do we need it? When there is no .bss section, we do not\n      pad the .data section.  */\n   . = ALIGN(. != 0 ? 64 / 8 : 1);\n  }\n  .lbss   :\n  {\n    *(.dynlbss)\n    *(.lbss .lbss.* .gnu.linkonce.lb.*)\n    *(LARGE_COMMON)\n  }\n  . = ALIGN(64 / 8);\n  . = SEGMENT_START(\"ldata-segment\", .);\n  .lrodata   ALIGN(CONSTANT (MAXPAGESIZE)) + (. & (CONSTANT (MAXPAGESIZE) - 1)) :\n  {\n    *(.lrodata .lrodata.* .gnu.linkonce.lr.*)\n  }\n  .ldata   ALIGN(CONSTANT (MAXPAGESIZE)) + (. & (CONSTANT (MAXPAGESIZE) - 1)) :\n  {\n    *(.ldata .ldata.* .gnu.linkonce.l.*)\n    . = ALIGN(. != 0 ? 64 / 8 : 1);\n  }\n  . = ALIGN(64 / 8);\n  _end = .; PROVIDE (end = .);\n  . = DATA_SEGMENT_END (.);\n  /* Stabs debugging sections.  */\n  .stab          0 : { *(.stab) }\n  .stabstr       0 : { *(.stabstr) }\n  .stab.excl     0 : { *(.stab.excl) }\n  .stab.exclstr  0 : { *(.stab.exclstr) }\n  .stab.index    0 : { *(.stab.index) }\n  .stab.indexstr 0 : { *(.stab.indexstr) }\n  .comment       0 : { *(.comment) }\n  .gnu.build.attributes : { *(.gnu.build.attributes .gnu.build.attributes.*) }\n  /* DWARF debug sections.\n     Symbols in the DWARF debugging sections are relative to the beginning\n     of the section so we begin them at 0.  */\n  /* DWARF 1.  */\n  .debug          0 : { *(.debug) }\n  .line           0 : { *(.line) }\n  /* GNU DWARF 1 extensions.  */\n  .debug_srcinfo  0 : { *(.debug_srcinfo) }\n  .debug_sfnames  0 : { *(.debug_sfnames) }\n  /* DWARF 1.1 and DWARF 2.  */\n  .debug_aranges  0 : { *(.debug_aranges) }\n  .debug_pubnames 0 : { *(.debug_pubnames) }\n  /* DWARF 2.  */\n  .debug_info     0 : { *(.debug_info .gnu.linkonce.wi.*) }\n  .debug_abbrev   0 : { *(.debug_abbrev) }\n  .debug_line     0 : { *(.debug_line .debug_line.* .debug_line_end) }\n  .debug_frame    0 : { *(.debug_frame) }\n  .debug_str      0 : { *(.debug_str) }\n  .debug_loc      0 : { *(.debug_loc) }\n  .debug_macinfo  0 : { *(.debug_macinfo) }\n  /* SGI/MIPS DWARF 2 extensions.  */\n  .debug_weaknames 0 : { *(.debug_weaknames) }\n  .debug_funcnames 0 : { *(.debug_funcnames) }\n  .debug_typenames 0 : { *(.debug_typenames) }\n  .debug_varnames  0 : { *(.debug_varnames) }\n  /* DWARF 3.  */\n  .debug_pubtypes 0 : { *(.debug_pubtypes) }\n  .debug_ranges   0 : { *(.debug_ranges) }\n  /* DWARF 5.  */\n  .debug_addr     0 : { *(.debug_addr) }\n  .debug_line_str 0 : { *(.debug_line_str) }\n  .debug_loclists 0 : { *(.debug_loclists) }\n  .debug_macro    0 : { *(.debug_macro) }\n  .debug_names    0 : { *(.debug_names) }\n  .debug_rnglists 0 : { *(.debug_rnglists) }\n  .debug_str_offsets 0 : { *(.debug_str_offsets) }\n  .debug_sup      0 : { *(.debug_sup) }\n  .gnu.attributes 0 : { KEEP (*(.gnu.attributes)) }\n  /DISCARD/ : { *(.note.GNU-stack) *(.gnu_debuglink) *(.gnu.lto_*) }\n}\n\n\0"
            as *const u8 as *const libc::c_char as *mut libc::c_char
    } else if link_info.combreloc() as libc::c_int != 0
        && link_info.separate_code() as libc::c_int != 0
    {
        return b"/* Script for -z combreloc -z separate-code */\n/* Copyright (C) 2014-2021 Free Software Foundation, Inc.\n   Copying and distribution of this script, with or without modification,\n   are permitted in any medium without royalty provided the copyright\n   notice and this notice are preserved.  */\nOUTPUT_FORMAT(\"elf64-x86-64\", \"elf64-x86-64\",\n\t      \"elf64-x86-64\")\nOUTPUT_ARCH(i386:x86-64)\nENTRY(_start)\nSEARCH_DIR(\"=/usr/local/x86_64-pc-linux-gnu/lib64\"); SEARCH_DIR(\"=/usr/local/lib64\"); SEARCH_DIR(\"=/lib64\"); SEARCH_DIR(\"=/usr/lib64\"); SEARCH_DIR(\"=/usr/local/x86_64-pc-linux-gnu/lib\"); SEARCH_DIR(\"=/usr/local/lib\"); SEARCH_DIR(\"=/lib\"); SEARCH_DIR(\"=/usr/lib\");\nSECTIONS\n{\n  PROVIDE (__executable_start = SEGMENT_START(\"text-segment\", 0x400000)); . = SEGMENT_START(\"text-segment\", 0x400000) + SIZEOF_HEADERS;\n  .interp         : { *(.interp) }\n  .note.gnu.build-id  : { *(.note.gnu.build-id) }\n  .hash           : { *(.hash) }\n  .gnu.hash       : { *(.gnu.hash) }\n  .dynsym         : { *(.dynsym) }\n  .dynstr         : { *(.dynstr) }\n  .gnu.version    : { *(.gnu.version) }\n  .gnu.version_d  : { *(.gnu.version_d) }\n  .gnu.version_r  : { *(.gnu.version_r) }\n  .rela.dyn       :\n    {\n      *(.rela.init)\n      *(.rela.text .rela.text.* .rela.gnu.linkonce.t.*)\n      *(.rela.fini)\n      *(.rela.rodata .rela.rodata.* .rela.gnu.linkonce.r.*)\n      *(.rela.data .rela.data.* .rela.gnu.linkonce.d.*)\n      *(.rela.tdata .rela.tdata.* .rela.gnu.linkonce.td.*)\n      *(.rela.tbss .rela.tbss.* .rela.gnu.linkonce.tb.*)\n      *(.rela.ctors)\n      *(.rela.dtors)\n      *(.rela.got)\n      *(.rela.bss .rela.bss.* .rela.gnu.linkonce.b.*)\n      *(.rela.ldata .rela.ldata.* .rela.gnu.linkonce.l.*)\n      *(.rela.lbss .rela.lbss.* .rela.gnu.linkonce.lb.*)\n      *(.rela.lrodata .rela.lrodata.* .rela.gnu.linkonce.lr.*)\n      *(.rela.ifunc)\n    }\n  .rela.plt       :\n    {\n      *(.rela.plt)\n      PROVIDE_HIDDEN (__rela_iplt_start = .);\n      *(.rela.iplt)\n      PROVIDE_HIDDEN (__rela_iplt_end = .);\n    }\n  . = ALIGN(CONSTANT (MAXPAGESIZE));\n  .init           :\n  {\n    KEEP (*(SORT_NONE(.init)))\n  }\n  .plt            : { *(.plt) *(.iplt) }\n.plt.got        : { *(.plt.got) }\n.plt.sec        : { *(.plt.sec) }\n  .text           :\n  {\n    *(.text.unlikely .text.*_unlikely .text.unlikely.*)\n    *(.text.exit .text.exit.*)\n    *(.text.startup .text.startup.*)\n    *(.text.hot .text.hot.*)\n    *(SORT(.text.sorted.*))\n    *(.text .stub .text.* .gnu.linkonce.t.*)\n    /* .gnu.warning sections are handled specially by elf.em.  */\n    *(.gnu.warning)\n  }\n  .fini           :\n  {\n    KEEP (*(SORT_NONE(.fini)))\n  }\n  PROVIDE (__etext = .);\n  PROVIDE (_etext = .);\n  PROVIDE (etext = .);\n  . = ALIGN(CONSTANT (MAXPAGESIZE));\n  /* Adjust the address for the rodata segment.  We want to adjust up to\n     the same address within the page on the next page up.  */\n  . = SEGMENT_START(\"rodata-segment\", ALIGN(CONSTANT (MAXPAGESIZE)) + (. & (CONSTANT (MAXPAGESIZE) - 1)));\n  .rodata         : { *(.rodata .rodata.* .gnu.linkonce.r.*) }\n  .rodata1        : { *(.rodata1) }\n  .eh_frame_hdr   : { *(.eh_frame_hdr) *(.eh_frame_entry .eh_frame_entry.*) }\n  .eh_frame       : ONLY_IF_RO { KEEP (*(.eh_frame)) *(.eh_frame.*) }\n  .gcc_except_table   : ONLY_IF_RO { *(.gcc_except_table .gcc_except_table.*) }\n  .gnu_extab   : ONLY_IF_RO { *(.gnu_extab*) }\n  /* These sections are generated by the Sun/Oracle C++ compiler.  */\n  .exception_ranges   : ONLY_IF_RO { *(.exception_ranges*) }\n  /* Adjust the address for the data segment.  We want to adjust up to\n     the same address within the page on the next page up.  */\n  . = DATA_SEGMENT_ALIGN (CONSTANT (MAXPAGESIZE), CONSTANT (COMMONPAGESIZE));\n  /* Exception handling  */\n  .eh_frame       : ONLY_IF_RW { KEEP (*(.eh_frame)) *(.eh_frame.*) }\n  .gnu_extab      : ONLY_IF_RW { *(.gnu_extab) }\n  .gcc_except_table   : ONLY_IF_RW { *(.gcc_except_table .gcc_except_table.*) }\n  .exception_ranges   : ONLY_IF_RW { *(.exception_ranges*) }\n  /* Thread Local Storage sections  */\n  .tdata\t  :\n   {\n     PROVIDE_HIDDEN (__tdata_start = .);\n     *(.tdata .tdata.* .gnu.linkonce.td.*)\n   }\n  .tbss\t\t  : { *(.tbss .tbss.* .gnu.linkonce.tb.*) *(.tcommon) }\n  .preinit_array    :\n  {\n    PROVIDE_HIDDEN (__preinit_array_start = .);\n    KEEP (*(.preinit_array))\n    PROVIDE_HIDDEN (__preinit_array_end = .);\n  }\n  .init_array    :\n  {\n    PROVIDE_HIDDEN (__init_array_start = .);\n    KEEP (*(SORT_BY_INIT_PRIORITY(.init_array.*) SORT_BY_INIT_PRIORITY(.ctors.*)))\n    KEEP (*(.init_array EXCLUDE_FILE (*crtbegin.o *crtbegin?.o *crtend.o *crtend?.o ) .ctors))\n    PROVIDE_HIDDEN (__init_array_end = .);\n  }\n  .fini_array    :\n  {\n    PROVIDE_HIDDEN (__fini_array_start = .);\n    KEEP (*(SORT_BY_INIT_PRIORITY(.fini_array.*) SORT_BY_INIT_PRIORITY(.dtors.*)))\n    KEEP (*(.fini_array EXCLUDE_FILE (*crtbegin.o *crtbegin?.o *crtend.o *crtend?.o ) .dtors))\n    PROVIDE_HIDDEN (__fini_array_end = .);\n  }\n  .ctors          :\n  {\n    /* gcc uses crtbegin.o to find the start of\n       the constructors, so we make sure it is\n       first.  Because this is a wildcard, it\n       doesn't matter if the user does not\n       actually link against crtbegin.o; the\n       linker won't look for a file to match a\n       wildcard.  The wildcard also means that it\n       doesn't matter which directory crtbegin.o\n       is in.  */\n    KEEP (*crtbegin.o(.ctors))\n    KEEP (*crtbegin?.o(.ctors))\n    /* We don't want to include the .ctor section from\n       the crtend.o file until after the sorted ctors.\n       The .ctor section from the crtend file contains the\n       end of ctors marker and it must be last */\n    KEEP (*(EXCLUDE_FILE (*crtend.o *crtend?.o ) .ctors))\n    KEEP (*(SORT(.ctors.*)))\n    KEEP (*(.ctors))\n  }\n  .dtors          :\n  {\n    KEEP (*crtbegin.o(.dtors))\n    KEEP (*crtbegin?.o(.dtors))\n    KEEP (*(EXCLUDE_FILE (*crtend.o *crtend?.o ) .dtors))\n    KEEP (*(SORT(.dtors.*)))\n    KEEP (*(.dtors))\n  }\n  .jcr            : { KEEP (*(.jcr)) }\n  .data.rel.ro : { *(.data.rel.ro.local* .gnu.linkonce.d.rel.ro.local.*) *(.data.rel.ro .data.rel.ro.* .gnu.linkonce.d.rel.ro.*) }\n  .dynamic        : { *(.dynamic) }\n  .got            : { *(.got) *(.igot) }\n  . = DATA_SEGMENT_RELRO_END (SIZEOF (.got.plt) >= 24 ? 24 : 0, .);\n  .got.plt        : { *(.got.plt) *(.igot.plt) }\n  .data           :\n  {\n    *(.data .data.* .gnu.linkonce.d.*)\n    SORT(CONSTRUCTORS)\n  }\n  .data1          : { *(.data1) }\n  _edata = .; PROVIDE (edata = .);\n  . = .;\n  __bss_start = .;\n  .bss            :\n  {\n   *(.dynbss)\n   *(.bss .bss.* .gnu.linkonce.b.*)\n   *(COMMON)\n   /* Align here to ensure that the .bss section occupies space up to\n      _end.  Align after .bss to ensure correct alignment even if the\n      .bss section disappears because there are no input sections.\n      FIXME: Why do we need it? When there is no .bss section, we do not\n      pad the .data section.  */\n   . = ALIGN(. != 0 ? 64 / 8 : 1);\n  }\n  .lbss   :\n  {\n    *(.dynlbss)\n    *(.lbss .lbss.* .gnu.linkonce.lb.*)\n    *(LARGE_COMMON)\n  }\n  . = ALIGN(64 / 8);\n  . = SEGMENT_START(\"ldata-segment\", .);\n  .lrodata   ALIGN(CONSTANT (MAXPAGESIZE)) + (. & (CONSTANT (MAXPAGESIZE) - 1)) :\n  {\n    *(.lrodata .lrodata.* .gnu.linkonce.lr.*)\n  }\n  .ldata   ALIGN(CONSTANT (MAXPAGESIZE)) + (. & (CONSTANT (MAXPAGESIZE) - 1)) :\n  {\n    *(.ldata .ldata.* .gnu.linkonce.l.*)\n    . = ALIGN(. != 0 ? 64 / 8 : 1);\n  }\n  . = ALIGN(64 / 8);\n  _end = .; PROVIDE (end = .);\n  . = DATA_SEGMENT_END (.);\n  /* Stabs debugging sections.  */\n  .stab          0 : { *(.stab) }\n  .stabstr       0 : { *(.stabstr) }\n  .stab.excl     0 : { *(.stab.excl) }\n  .stab.exclstr  0 : { *(.stab.exclstr) }\n  .stab.index    0 : { *(.stab.index) }\n  .stab.indexstr 0 : { *(.stab.indexstr) }\n  .comment       0 : { *(.comment) }\n  .gnu.build.attributes : { *(.gnu.build.attributes .gnu.build.attributes.*) }\n  /* DWARF debug sections.\n     Symbols in the DWARF debugging sections are relative to the beginning\n     of the section so we begin them at 0.  */\n  /* DWARF 1.  */\n  .debug          0 : { *(.debug) }\n  .line           0 : { *(.line) }\n  /* GNU DWARF 1 extensions.  */\n  .debug_srcinfo  0 : { *(.debug_srcinfo) }\n  .debug_sfnames  0 : { *(.debug_sfnames) }\n  /* DWARF 1.1 and DWARF 2.  */\n  .debug_aranges  0 : { *(.debug_aranges) }\n  .debug_pubnames 0 : { *(.debug_pubnames) }\n  /* DWARF 2.  */\n  .debug_info     0 : { *(.debug_info .gnu.linkonce.wi.*) }\n  .debug_abbrev   0 : { *(.debug_abbrev) }\n  .debug_line     0 : { *(.debug_line .debug_line.* .debug_line_end) }\n  .debug_frame    0 : { *(.debug_frame) }\n  .debug_str      0 : { *(.debug_str) }\n  .debug_loc      0 : { *(.debug_loc) }\n  .debug_macinfo  0 : { *(.debug_macinfo) }\n  /* SGI/MIPS DWARF 2 extensions.  */\n  .debug_weaknames 0 : { *(.debug_weaknames) }\n  .debug_funcnames 0 : { *(.debug_funcnames) }\n  .debug_typenames 0 : { *(.debug_typenames) }\n  .debug_varnames  0 : { *(.debug_varnames) }\n  /* DWARF 3.  */\n  .debug_pubtypes 0 : { *(.debug_pubtypes) }\n  .debug_ranges   0 : { *(.debug_ranges) }\n  /* DWARF 5.  */\n  .debug_addr     0 : { *(.debug_addr) }\n  .debug_line_str 0 : { *(.debug_line_str) }\n  .debug_loclists 0 : { *(.debug_loclists) }\n  .debug_macro    0 : { *(.debug_macro) }\n  .debug_names    0 : { *(.debug_names) }\n  .debug_rnglists 0 : { *(.debug_rnglists) }\n  .debug_str_offsets 0 : { *(.debug_str_offsets) }\n  .debug_sup      0 : { *(.debug_sup) }\n  .gnu.attributes 0 : { KEEP (*(.gnu.attributes)) }\n  /DISCARD/ : { *(.note.GNU-stack) *(.gnu_debuglink) *(.gnu.lto_*) }\n}\n\n\0"
            as *const u8 as *const libc::c_char as *mut libc::c_char
    } else if link_info.combreloc() != 0 {
        return b"/* Script for -z combreloc */\n/* Copyright (C) 2014-2021 Free Software Foundation, Inc.\n   Copying and distribution of this script, with or without modification,\n   are permitted in any medium without royalty provided the copyright\n   notice and this notice are preserved.  */\nOUTPUT_FORMAT(\"elf64-x86-64\", \"elf64-x86-64\",\n\t      \"elf64-x86-64\")\nOUTPUT_ARCH(i386:x86-64)\nENTRY(_start)\nSEARCH_DIR(\"=/usr/local/x86_64-pc-linux-gnu/lib64\"); SEARCH_DIR(\"=/usr/local/lib64\"); SEARCH_DIR(\"=/lib64\"); SEARCH_DIR(\"=/usr/lib64\"); SEARCH_DIR(\"=/usr/local/x86_64-pc-linux-gnu/lib\"); SEARCH_DIR(\"=/usr/local/lib\"); SEARCH_DIR(\"=/lib\"); SEARCH_DIR(\"=/usr/lib\");\nSECTIONS\n{\n  /* Read-only sections, merged into text segment: */\n  PROVIDE (__executable_start = SEGMENT_START(\"text-segment\", 0x400000)); . = SEGMENT_START(\"text-segment\", 0x400000) + SIZEOF_HEADERS;\n  .interp         : { *(.interp) }\n  .note.gnu.build-id  : { *(.note.gnu.build-id) }\n  .hash           : { *(.hash) }\n  .gnu.hash       : { *(.gnu.hash) }\n  .dynsym         : { *(.dynsym) }\n  .dynstr         : { *(.dynstr) }\n  .gnu.version    : { *(.gnu.version) }\n  .gnu.version_d  : { *(.gnu.version_d) }\n  .gnu.version_r  : { *(.gnu.version_r) }\n  .rela.dyn       :\n    {\n      *(.rela.init)\n      *(.rela.text .rela.text.* .rela.gnu.linkonce.t.*)\n      *(.rela.fini)\n      *(.rela.rodata .rela.rodata.* .rela.gnu.linkonce.r.*)\n      *(.rela.data .rela.data.* .rela.gnu.linkonce.d.*)\n      *(.rela.tdata .rela.tdata.* .rela.gnu.linkonce.td.*)\n      *(.rela.tbss .rela.tbss.* .rela.gnu.linkonce.tb.*)\n      *(.rela.ctors)\n      *(.rela.dtors)\n      *(.rela.got)\n      *(.rela.bss .rela.bss.* .rela.gnu.linkonce.b.*)\n      *(.rela.ldata .rela.ldata.* .rela.gnu.linkonce.l.*)\n      *(.rela.lbss .rela.lbss.* .rela.gnu.linkonce.lb.*)\n      *(.rela.lrodata .rela.lrodata.* .rela.gnu.linkonce.lr.*)\n      *(.rela.ifunc)\n    }\n  .rela.plt       :\n    {\n      *(.rela.plt)\n      PROVIDE_HIDDEN (__rela_iplt_start = .);\n      *(.rela.iplt)\n      PROVIDE_HIDDEN (__rela_iplt_end = .);\n    }\n  .init           :\n  {\n    KEEP (*(SORT_NONE(.init)))\n  }\n  .plt            : { *(.plt) *(.iplt) }\n.plt.got        : { *(.plt.got) }\n.plt.sec        : { *(.plt.sec) }\n  .text           :\n  {\n    *(.text.unlikely .text.*_unlikely .text.unlikely.*)\n    *(.text.exit .text.exit.*)\n    *(.text.startup .text.startup.*)\n    *(.text.hot .text.hot.*)\n    *(SORT(.text.sorted.*))\n    *(.text .stub .text.* .gnu.linkonce.t.*)\n    /* .gnu.warning sections are handled specially by elf.em.  */\n    *(.gnu.warning)\n  }\n  .fini           :\n  {\n    KEEP (*(SORT_NONE(.fini)))\n  }\n  PROVIDE (__etext = .);\n  PROVIDE (_etext = .);\n  PROVIDE (etext = .);\n  .rodata         : { *(.rodata .rodata.* .gnu.linkonce.r.*) }\n  .rodata1        : { *(.rodata1) }\n  .eh_frame_hdr   : { *(.eh_frame_hdr) *(.eh_frame_entry .eh_frame_entry.*) }\n  .eh_frame       : ONLY_IF_RO { KEEP (*(.eh_frame)) *(.eh_frame.*) }\n  .gcc_except_table   : ONLY_IF_RO { *(.gcc_except_table .gcc_except_table.*) }\n  .gnu_extab   : ONLY_IF_RO { *(.gnu_extab*) }\n  /* These sections are generated by the Sun/Oracle C++ compiler.  */\n  .exception_ranges   : ONLY_IF_RO { *(.exception_ranges*) }\n  /* Adjust the address for the data segment.  We want to adjust up to\n     the same address within the page on the next page up.  */\n  . = DATA_SEGMENT_ALIGN (CONSTANT (MAXPAGESIZE), CONSTANT (COMMONPAGESIZE));\n  /* Exception handling  */\n  .eh_frame       : ONLY_IF_RW { KEEP (*(.eh_frame)) *(.eh_frame.*) }\n  .gnu_extab      : ONLY_IF_RW { *(.gnu_extab) }\n  .gcc_except_table   : ONLY_IF_RW { *(.gcc_except_table .gcc_except_table.*) }\n  .exception_ranges   : ONLY_IF_RW { *(.exception_ranges*) }\n  /* Thread Local Storage sections  */\n  .tdata\t  :\n   {\n     PROVIDE_HIDDEN (__tdata_start = .);\n     *(.tdata .tdata.* .gnu.linkonce.td.*)\n   }\n  .tbss\t\t  : { *(.tbss .tbss.* .gnu.linkonce.tb.*) *(.tcommon) }\n  .preinit_array    :\n  {\n    PROVIDE_HIDDEN (__preinit_array_start = .);\n    KEEP (*(.preinit_array))\n    PROVIDE_HIDDEN (__preinit_array_end = .);\n  }\n  .init_array    :\n  {\n    PROVIDE_HIDDEN (__init_array_start = .);\n    KEEP (*(SORT_BY_INIT_PRIORITY(.init_array.*) SORT_BY_INIT_PRIORITY(.ctors.*)))\n    KEEP (*(.init_array EXCLUDE_FILE (*crtbegin.o *crtbegin?.o *crtend.o *crtend?.o ) .ctors))\n    PROVIDE_HIDDEN (__init_array_end = .);\n  }\n  .fini_array    :\n  {\n    PROVIDE_HIDDEN (__fini_array_start = .);\n    KEEP (*(SORT_BY_INIT_PRIORITY(.fini_array.*) SORT_BY_INIT_PRIORITY(.dtors.*)))\n    KEEP (*(.fini_array EXCLUDE_FILE (*crtbegin.o *crtbegin?.o *crtend.o *crtend?.o ) .dtors))\n    PROVIDE_HIDDEN (__fini_array_end = .);\n  }\n  .ctors          :\n  {\n    /* gcc uses crtbegin.o to find the start of\n       the constructors, so we make sure it is\n       first.  Because this is a wildcard, it\n       doesn't matter if the user does not\n       actually link against crtbegin.o; the\n       linker won't look for a file to match a\n       wildcard.  The wildcard also means that it\n       doesn't matter which directory crtbegin.o\n       is in.  */\n    KEEP (*crtbegin.o(.ctors))\n    KEEP (*crtbegin?.o(.ctors))\n    /* We don't want to include the .ctor section from\n       the crtend.o file until after the sorted ctors.\n       The .ctor section from the crtend file contains the\n       end of ctors marker and it must be last */\n    KEEP (*(EXCLUDE_FILE (*crtend.o *crtend?.o ) .ctors))\n    KEEP (*(SORT(.ctors.*)))\n    KEEP (*(.ctors))\n  }\n  .dtors          :\n  {\n    KEEP (*crtbegin.o(.dtors))\n    KEEP (*crtbegin?.o(.dtors))\n    KEEP (*(EXCLUDE_FILE (*crtend.o *crtend?.o ) .dtors))\n    KEEP (*(SORT(.dtors.*)))\n    KEEP (*(.dtors))\n  }\n  .jcr            : { KEEP (*(.jcr)) }\n  .data.rel.ro : { *(.data.rel.ro.local* .gnu.linkonce.d.rel.ro.local.*) *(.data.rel.ro .data.rel.ro.* .gnu.linkonce.d.rel.ro.*) }\n  .dynamic        : { *(.dynamic) }\n  .got            : { *(.got) *(.igot) }\n  . = DATA_SEGMENT_RELRO_END (SIZEOF (.got.plt) >= 24 ? 24 : 0, .);\n  .got.plt        : { *(.got.plt) *(.igot.plt) }\n  .data           :\n  {\n    *(.data .data.* .gnu.linkonce.d.*)\n    SORT(CONSTRUCTORS)\n  }\n  .data1          : { *(.data1) }\n  _edata = .; PROVIDE (edata = .);\n  . = .;\n  __bss_start = .;\n  .bss            :\n  {\n   *(.dynbss)\n   *(.bss .bss.* .gnu.linkonce.b.*)\n   *(COMMON)\n   /* Align here to ensure that the .bss section occupies space up to\n      _end.  Align after .bss to ensure correct alignment even if the\n      .bss section disappears because there are no input sections.\n      FIXME: Why do we need it? When there is no .bss section, we do not\n      pad the .data section.  */\n   . = ALIGN(. != 0 ? 64 / 8 : 1);\n  }\n  .lbss   :\n  {\n    *(.dynlbss)\n    *(.lbss .lbss.* .gnu.linkonce.lb.*)\n    *(LARGE_COMMON)\n  }\n  . = ALIGN(64 / 8);\n  . = SEGMENT_START(\"ldata-segment\", .);\n  .lrodata   ALIGN(CONSTANT (MAXPAGESIZE)) + (. & (CONSTANT (MAXPAGESIZE) - 1)) :\n  {\n    *(.lrodata .lrodata.* .gnu.linkonce.lr.*)\n  }\n  .ldata   ALIGN(CONSTANT (MAXPAGESIZE)) + (. & (CONSTANT (MAXPAGESIZE) - 1)) :\n  {\n    *(.ldata .ldata.* .gnu.linkonce.l.*)\n    . = ALIGN(. != 0 ? 64 / 8 : 1);\n  }\n  . = ALIGN(64 / 8);\n  _end = .; PROVIDE (end = .);\n  . = DATA_SEGMENT_END (.);\n  /* Stabs debugging sections.  */\n  .stab          0 : { *(.stab) }\n  .stabstr       0 : { *(.stabstr) }\n  .stab.excl     0 : { *(.stab.excl) }\n  .stab.exclstr  0 : { *(.stab.exclstr) }\n  .stab.index    0 : { *(.stab.index) }\n  .stab.indexstr 0 : { *(.stab.indexstr) }\n  .comment       0 : { *(.comment) }\n  .gnu.build.attributes : { *(.gnu.build.attributes .gnu.build.attributes.*) }\n  /* DWARF debug sections.\n     Symbols in the DWARF debugging sections are relative to the beginning\n     of the section so we begin them at 0.  */\n  /* DWARF 1.  */\n  .debug          0 : { *(.debug) }\n  .line           0 : { *(.line) }\n  /* GNU DWARF 1 extensions.  */\n  .debug_srcinfo  0 : { *(.debug_srcinfo) }\n  .debug_sfnames  0 : { *(.debug_sfnames) }\n  /* DWARF 1.1 and DWARF 2.  */\n  .debug_aranges  0 : { *(.debug_aranges) }\n  .debug_pubnames 0 : { *(.debug_pubnames) }\n  /* DWARF 2.  */\n  .debug_info     0 : { *(.debug_info .gnu.linkonce.wi.*) }\n  .debug_abbrev   0 : { *(.debug_abbrev) }\n  .debug_line     0 : { *(.debug_line .debug_line.* .debug_line_end) }\n  .debug_frame    0 : { *(.debug_frame) }\n  .debug_str      0 : { *(.debug_str) }\n  .debug_loc      0 : { *(.debug_loc) }\n  .debug_macinfo  0 : { *(.debug_macinfo) }\n  /* SGI/MIPS DWARF 2 extensions.  */\n  .debug_weaknames 0 : { *(.debug_weaknames) }\n  .debug_funcnames 0 : { *(.debug_funcnames) }\n  .debug_typenames 0 : { *(.debug_typenames) }\n  .debug_varnames  0 : { *(.debug_varnames) }\n  /* DWARF 3.  */\n  .debug_pubtypes 0 : { *(.debug_pubtypes) }\n  .debug_ranges   0 : { *(.debug_ranges) }\n  /* DWARF 5.  */\n  .debug_addr     0 : { *(.debug_addr) }\n  .debug_line_str 0 : { *(.debug_line_str) }\n  .debug_loclists 0 : { *(.debug_loclists) }\n  .debug_macro    0 : { *(.debug_macro) }\n  .debug_names    0 : { *(.debug_names) }\n  .debug_rnglists 0 : { *(.debug_rnglists) }\n  .debug_str_offsets 0 : { *(.debug_str_offsets) }\n  .debug_sup      0 : { *(.debug_sup) }\n  .gnu.attributes 0 : { KEEP (*(.gnu.attributes)) }\n  /DISCARD/ : { *(.note.GNU-stack) *(.gnu_debuglink) *(.gnu.lto_*) }\n}\n\n\0"
            as *const u8 as *const libc::c_char as *mut libc::c_char
    } else if link_info.separate_code() != 0 {
        return b"/* Script for -z separate-code */\n/* Copyright (C) 2014-2021 Free Software Foundation, Inc.\n   Copying and distribution of this script, with or without modification,\n   are permitted in any medium without royalty provided the copyright\n   notice and this notice are preserved.  */\nOUTPUT_FORMAT(\"elf64-x86-64\", \"elf64-x86-64\",\n\t      \"elf64-x86-64\")\nOUTPUT_ARCH(i386:x86-64)\nENTRY(_start)\nSEARCH_DIR(\"=/usr/local/x86_64-pc-linux-gnu/lib64\"); SEARCH_DIR(\"=/usr/local/lib64\"); SEARCH_DIR(\"=/lib64\"); SEARCH_DIR(\"=/usr/lib64\"); SEARCH_DIR(\"=/usr/local/x86_64-pc-linux-gnu/lib\"); SEARCH_DIR(\"=/usr/local/lib\"); SEARCH_DIR(\"=/lib\"); SEARCH_DIR(\"=/usr/lib\");\nSECTIONS\n{\n  PROVIDE (__executable_start = SEGMENT_START(\"text-segment\", 0x400000)); . = SEGMENT_START(\"text-segment\", 0x400000) + SIZEOF_HEADERS;\n  .interp         : { *(.interp) }\n  .note.gnu.build-id  : { *(.note.gnu.build-id) }\n  .hash           : { *(.hash) }\n  .gnu.hash       : { *(.gnu.hash) }\n  .dynsym         : { *(.dynsym) }\n  .dynstr         : { *(.dynstr) }\n  .gnu.version    : { *(.gnu.version) }\n  .gnu.version_d  : { *(.gnu.version_d) }\n  .gnu.version_r  : { *(.gnu.version_r) }\n  .rela.init      : { *(.rela.init) }\n  .rela.text      : { *(.rela.text .rela.text.* .rela.gnu.linkonce.t.*) }\n  .rela.fini      : { *(.rela.fini) }\n  .rela.rodata    : { *(.rela.rodata .rela.rodata.* .rela.gnu.linkonce.r.*) }\n  .rela.data.rel.ro   : { *(.rela.data.rel.ro .rela.data.rel.ro.* .rela.gnu.linkonce.d.rel.ro.*) }\n  .rela.data      : { *(.rela.data .rela.data.* .rela.gnu.linkonce.d.*) }\n  .rela.tdata\t  : { *(.rela.tdata .rela.tdata.* .rela.gnu.linkonce.td.*) }\n  .rela.tbss\t  : { *(.rela.tbss .rela.tbss.* .rela.gnu.linkonce.tb.*) }\n  .rela.ctors     : { *(.rela.ctors) }\n  .rela.dtors     : { *(.rela.dtors) }\n  .rela.got       : { *(.rela.got) }\n  .rela.bss       : { *(.rela.bss .rela.bss.* .rela.gnu.linkonce.b.*) }\n  .rela.ldata     : { *(.rela.ldata .rela.ldata.* .rela.gnu.linkonce.l.*) }\n  .rela.lbss      : { *(.rela.lbss .rela.lbss.* .rela.gnu.linkonce.lb.*) }\n  .rela.lrodata   : { *(.rela.lrodata .rela.lrodata.* .rela.gnu.linkonce.lr.*) }\n  .rela.ifunc     : { *(.rela.ifunc) }\n  .rela.plt       :\n    {\n      *(.rela.plt)\n      PROVIDE_HIDDEN (__rela_iplt_start = .);\n      *(.rela.iplt)\n      PROVIDE_HIDDEN (__rela_iplt_end = .);\n    }\n  . = ALIGN(CONSTANT (MAXPAGESIZE));\n  .init           :\n  {\n    KEEP (*(SORT_NONE(.init)))\n  }\n  .plt            : { *(.plt) *(.iplt) }\n.plt.got        : { *(.plt.got) }\n.plt.sec        : { *(.plt.sec) }\n  .text           :\n  {\n    *(.text.unlikely .text.*_unlikely .text.unlikely.*)\n    *(.text.exit .text.exit.*)\n    *(.text.startup .text.startup.*)\n    *(.text.hot .text.hot.*)\n    *(SORT(.text.sorted.*))\n    *(.text .stub .text.* .gnu.linkonce.t.*)\n    /* .gnu.warning sections are handled specially by elf.em.  */\n    *(.gnu.warning)\n  }\n  .fini           :\n  {\n    KEEP (*(SORT_NONE(.fini)))\n  }\n  PROVIDE (__etext = .);\n  PROVIDE (_etext = .);\n  PROVIDE (etext = .);\n  . = ALIGN(CONSTANT (MAXPAGESIZE));\n  /* Adjust the address for the rodata segment.  We want to adjust up to\n     the same address within the page on the next page up.  */\n  . = SEGMENT_START(\"rodata-segment\", ALIGN(CONSTANT (MAXPAGESIZE)) + (. & (CONSTANT (MAXPAGESIZE) - 1)));\n  .rodata         : { *(.rodata .rodata.* .gnu.linkonce.r.*) }\n  .rodata1        : { *(.rodata1) }\n  .eh_frame_hdr   : { *(.eh_frame_hdr) *(.eh_frame_entry .eh_frame_entry.*) }\n  .eh_frame       : ONLY_IF_RO { KEEP (*(.eh_frame)) *(.eh_frame.*) }\n  .gcc_except_table   : ONLY_IF_RO { *(.gcc_except_table .gcc_except_table.*) }\n  .gnu_extab   : ONLY_IF_RO { *(.gnu_extab*) }\n  /* These sections are generated by the Sun/Oracle C++ compiler.  */\n  .exception_ranges   : ONLY_IF_RO { *(.exception_ranges*) }\n  /* Adjust the address for the data segment.  We want to adjust up to\n     the same address within the page on the next page up.  */\n  . = DATA_SEGMENT_ALIGN (CONSTANT (MAXPAGESIZE), CONSTANT (COMMONPAGESIZE));\n  /* Exception handling  */\n  .eh_frame       : ONLY_IF_RW { KEEP (*(.eh_frame)) *(.eh_frame.*) }\n  .gnu_extab      : ONLY_IF_RW { *(.gnu_extab) }\n  .gcc_except_table   : ONLY_IF_RW { *(.gcc_except_table .gcc_except_table.*) }\n  .exception_ranges   : ONLY_IF_RW { *(.exception_ranges*) }\n  /* Thread Local Storage sections  */\n  .tdata\t  :\n   {\n     PROVIDE_HIDDEN (__tdata_start = .);\n     *(.tdata .tdata.* .gnu.linkonce.td.*)\n   }\n  .tbss\t\t  : { *(.tbss .tbss.* .gnu.linkonce.tb.*) *(.tcommon) }\n  .preinit_array    :\n  {\n    PROVIDE_HIDDEN (__preinit_array_start = .);\n    KEEP (*(.preinit_array))\n    PROVIDE_HIDDEN (__preinit_array_end = .);\n  }\n  .init_array    :\n  {\n    PROVIDE_HIDDEN (__init_array_start = .);\n    KEEP (*(SORT_BY_INIT_PRIORITY(.init_array.*) SORT_BY_INIT_PRIORITY(.ctors.*)))\n    KEEP (*(.init_array EXCLUDE_FILE (*crtbegin.o *crtbegin?.o *crtend.o *crtend?.o ) .ctors))\n    PROVIDE_HIDDEN (__init_array_end = .);\n  }\n  .fini_array    :\n  {\n    PROVIDE_HIDDEN (__fini_array_start = .);\n    KEEP (*(SORT_BY_INIT_PRIORITY(.fini_array.*) SORT_BY_INIT_PRIORITY(.dtors.*)))\n    KEEP (*(.fini_array EXCLUDE_FILE (*crtbegin.o *crtbegin?.o *crtend.o *crtend?.o ) .dtors))\n    PROVIDE_HIDDEN (__fini_array_end = .);\n  }\n  .ctors          :\n  {\n    /* gcc uses crtbegin.o to find the start of\n       the constructors, so we make sure it is\n       first.  Because this is a wildcard, it\n       doesn't matter if the user does not\n       actually link against crtbegin.o; the\n       linker won't look for a file to match a\n       wildcard.  The wildcard also means that it\n       doesn't matter which directory crtbegin.o\n       is in.  */\n    KEEP (*crtbegin.o(.ctors))\n    KEEP (*crtbegin?.o(.ctors))\n    /* We don't want to include the .ctor section from\n       the crtend.o file until after the sorted ctors.\n       The .ctor section from the crtend file contains the\n       end of ctors marker and it must be last */\n    KEEP (*(EXCLUDE_FILE (*crtend.o *crtend?.o ) .ctors))\n    KEEP (*(SORT(.ctors.*)))\n    KEEP (*(.ctors))\n  }\n  .dtors          :\n  {\n    KEEP (*crtbegin.o(.dtors))\n    KEEP (*crtbegin?.o(.dtors))\n    KEEP (*(EXCLUDE_FILE (*crtend.o *crtend?.o ) .dtors))\n    KEEP (*(SORT(.dtors.*)))\n    KEEP (*(.dtors))\n  }\n  .jcr            : { KEEP (*(.jcr)) }\n  .data.rel.ro : { *(.data.rel.ro.local* .gnu.linkonce.d.rel.ro.local.*) *(.data.rel.ro .data.rel.ro.* .gnu.linkonce.d.rel.ro.*) }\n  .dynamic        : { *(.dynamic) }\n  .got            : { *(.got) *(.igot) }\n  . = DATA_SEGMENT_RELRO_END (SIZEOF (.got.plt) >= 24 ? 24 : 0, .);\n  .got.plt        : { *(.got.plt) *(.igot.plt) }\n  .data           :\n  {\n    *(.data .data.* .gnu.linkonce.d.*)\n    SORT(CONSTRUCTORS)\n  }\n  .data1          : { *(.data1) }\n  _edata = .; PROVIDE (edata = .);\n  . = .;\n  __bss_start = .;\n  .bss            :\n  {\n   *(.dynbss)\n   *(.bss .bss.* .gnu.linkonce.b.*)\n   *(COMMON)\n   /* Align here to ensure that the .bss section occupies space up to\n      _end.  Align after .bss to ensure correct alignment even if the\n      .bss section disappears because there are no input sections.\n      FIXME: Why do we need it? When there is no .bss section, we do not\n      pad the .data section.  */\n   . = ALIGN(. != 0 ? 64 / 8 : 1);\n  }\n  .lbss   :\n  {\n    *(.dynlbss)\n    *(.lbss .lbss.* .gnu.linkonce.lb.*)\n    *(LARGE_COMMON)\n  }\n  . = ALIGN(64 / 8);\n  . = SEGMENT_START(\"ldata-segment\", .);\n  .lrodata   ALIGN(CONSTANT (MAXPAGESIZE)) + (. & (CONSTANT (MAXPAGESIZE) - 1)) :\n  {\n    *(.lrodata .lrodata.* .gnu.linkonce.lr.*)\n  }\n  .ldata   ALIGN(CONSTANT (MAXPAGESIZE)) + (. & (CONSTANT (MAXPAGESIZE) - 1)) :\n  {\n    *(.ldata .ldata.* .gnu.linkonce.l.*)\n    . = ALIGN(. != 0 ? 64 / 8 : 1);\n  }\n  . = ALIGN(64 / 8);\n  _end = .; PROVIDE (end = .);\n  . = DATA_SEGMENT_END (.);\n  /* Stabs debugging sections.  */\n  .stab          0 : { *(.stab) }\n  .stabstr       0 : { *(.stabstr) }\n  .stab.excl     0 : { *(.stab.excl) }\n  .stab.exclstr  0 : { *(.stab.exclstr) }\n  .stab.index    0 : { *(.stab.index) }\n  .stab.indexstr 0 : { *(.stab.indexstr) }\n  .comment       0 : { *(.comment) }\n  .gnu.build.attributes : { *(.gnu.build.attributes .gnu.build.attributes.*) }\n  /* DWARF debug sections.\n     Symbols in the DWARF debugging sections are relative to the beginning\n     of the section so we begin them at 0.  */\n  /* DWARF 1.  */\n  .debug          0 : { *(.debug) }\n  .line           0 : { *(.line) }\n  /* GNU DWARF 1 extensions.  */\n  .debug_srcinfo  0 : { *(.debug_srcinfo) }\n  .debug_sfnames  0 : { *(.debug_sfnames) }\n  /* DWARF 1.1 and DWARF 2.  */\n  .debug_aranges  0 : { *(.debug_aranges) }\n  .debug_pubnames 0 : { *(.debug_pubnames) }\n  /* DWARF 2.  */\n  .debug_info     0 : { *(.debug_info .gnu.linkonce.wi.*) }\n  .debug_abbrev   0 : { *(.debug_abbrev) }\n  .debug_line     0 : { *(.debug_line .debug_line.* .debug_line_end) }\n  .debug_frame    0 : { *(.debug_frame) }\n  .debug_str      0 : { *(.debug_str) }\n  .debug_loc      0 : { *(.debug_loc) }\n  .debug_macinfo  0 : { *(.debug_macinfo) }\n  /* SGI/MIPS DWARF 2 extensions.  */\n  .debug_weaknames 0 : { *(.debug_weaknames) }\n  .debug_funcnames 0 : { *(.debug_funcnames) }\n  .debug_typenames 0 : { *(.debug_typenames) }\n  .debug_varnames  0 : { *(.debug_varnames) }\n  /* DWARF 3.  */\n  .debug_pubtypes 0 : { *(.debug_pubtypes) }\n  .debug_ranges   0 : { *(.debug_ranges) }\n  /* DWARF 5.  */\n  .debug_addr     0 : { *(.debug_addr) }\n  .debug_line_str 0 : { *(.debug_line_str) }\n  .debug_loclists 0 : { *(.debug_loclists) }\n  .debug_macro    0 : { *(.debug_macro) }\n  .debug_names    0 : { *(.debug_names) }\n  .debug_rnglists 0 : { *(.debug_rnglists) }\n  .debug_str_offsets 0 : { *(.debug_str_offsets) }\n  .debug_sup      0 : { *(.debug_sup) }\n  .gnu.attributes 0 : { KEEP (*(.gnu.attributes)) }\n  /DISCARD/ : { *(.note.GNU-stack) *(.gnu_debuglink) *(.gnu.lto_*) }\n}\n\n\0"
            as *const u8 as *const libc::c_char as *mut libc::c_char
    } else {
        return b"/* Default linker script, for normal executables */\n/* Copyright (C) 2014-2021 Free Software Foundation, Inc.\n   Copying and distribution of this script, with or without modification,\n   are permitted in any medium without royalty provided the copyright\n   notice and this notice are preserved.  */\nOUTPUT_FORMAT(\"elf64-x86-64\", \"elf64-x86-64\",\n\t      \"elf64-x86-64\")\nOUTPUT_ARCH(i386:x86-64)\nENTRY(_start)\nSEARCH_DIR(\"=/usr/local/x86_64-pc-linux-gnu/lib64\"); SEARCH_DIR(\"=/usr/local/lib64\"); SEARCH_DIR(\"=/lib64\"); SEARCH_DIR(\"=/usr/lib64\"); SEARCH_DIR(\"=/usr/local/x86_64-pc-linux-gnu/lib\"); SEARCH_DIR(\"=/usr/local/lib\"); SEARCH_DIR(\"=/lib\"); SEARCH_DIR(\"=/usr/lib\");\nSECTIONS\n{\n  /* Read-only sections, merged into text segment: */\n  PROVIDE (__executable_start = SEGMENT_START(\"text-segment\", 0x400000)); . = SEGMENT_START(\"text-segment\", 0x400000) + SIZEOF_HEADERS;\n  .interp         : { *(.interp) }\n  .note.gnu.build-id  : { *(.note.gnu.build-id) }\n  .hash           : { *(.hash) }\n  .gnu.hash       : { *(.gnu.hash) }\n  .dynsym         : { *(.dynsym) }\n  .dynstr         : { *(.dynstr) }\n  .gnu.version    : { *(.gnu.version) }\n  .gnu.version_d  : { *(.gnu.version_d) }\n  .gnu.version_r  : { *(.gnu.version_r) }\n  .rela.init      : { *(.rela.init) }\n  .rela.text      : { *(.rela.text .rela.text.* .rela.gnu.linkonce.t.*) }\n  .rela.fini      : { *(.rela.fini) }\n  .rela.rodata    : { *(.rela.rodata .rela.rodata.* .rela.gnu.linkonce.r.*) }\n  .rela.data.rel.ro   : { *(.rela.data.rel.ro .rela.data.rel.ro.* .rela.gnu.linkonce.d.rel.ro.*) }\n  .rela.data      : { *(.rela.data .rela.data.* .rela.gnu.linkonce.d.*) }\n  .rela.tdata\t  : { *(.rela.tdata .rela.tdata.* .rela.gnu.linkonce.td.*) }\n  .rela.tbss\t  : { *(.rela.tbss .rela.tbss.* .rela.gnu.linkonce.tb.*) }\n  .rela.ctors     : { *(.rela.ctors) }\n  .rela.dtors     : { *(.rela.dtors) }\n  .rela.got       : { *(.rela.got) }\n  .rela.bss       : { *(.rela.bss .rela.bss.* .rela.gnu.linkonce.b.*) }\n  .rela.ldata     : { *(.rela.ldata .rela.ldata.* .rela.gnu.linkonce.l.*) }\n  .rela.lbss      : { *(.rela.lbss .rela.lbss.* .rela.gnu.linkonce.lb.*) }\n  .rela.lrodata   : { *(.rela.lrodata .rela.lrodata.* .rela.gnu.linkonce.lr.*) }\n  .rela.ifunc     : { *(.rela.ifunc) }\n  .rela.plt       :\n    {\n      *(.rela.plt)\n      PROVIDE_HIDDEN (__rela_iplt_start = .);\n      *(.rela.iplt)\n      PROVIDE_HIDDEN (__rela_iplt_end = .);\n    }\n  .init           :\n  {\n    KEEP (*(SORT_NONE(.init)))\n  }\n  .plt            : { *(.plt) *(.iplt) }\n.plt.got        : { *(.plt.got) }\n.plt.sec        : { *(.plt.sec) }\n  .text           :\n  {\n    *(.text.unlikely .text.*_unlikely .text.unlikely.*)\n    *(.text.exit .text.exit.*)\n    *(.text.startup .text.startup.*)\n    *(.text.hot .text.hot.*)\n    *(SORT(.text.sorted.*))\n    *(.text .stub .text.* .gnu.linkonce.t.*)\n    /* .gnu.warning sections are handled specially by elf.em.  */\n    *(.gnu.warning)\n  }\n  .fini           :\n  {\n    KEEP (*(SORT_NONE(.fini)))\n  }\n  PROVIDE (__etext = .);\n  PROVIDE (_etext = .);\n  PROVIDE (etext = .);\n  .rodata         : { *(.rodata .rodata.* .gnu.linkonce.r.*) }\n  .rodata1        : { *(.rodata1) }\n  .eh_frame_hdr   : { *(.eh_frame_hdr) *(.eh_frame_entry .eh_frame_entry.*) }\n  .eh_frame       : ONLY_IF_RO { KEEP (*(.eh_frame)) *(.eh_frame.*) }\n  .gcc_except_table   : ONLY_IF_RO { *(.gcc_except_table .gcc_except_table.*) }\n  .gnu_extab   : ONLY_IF_RO { *(.gnu_extab*) }\n  /* These sections are generated by the Sun/Oracle C++ compiler.  */\n  .exception_ranges   : ONLY_IF_RO { *(.exception_ranges*) }\n  /* Adjust the address for the data segment.  We want to adjust up to\n     the same address within the page on the next page up.  */\n  . = DATA_SEGMENT_ALIGN (CONSTANT (MAXPAGESIZE), CONSTANT (COMMONPAGESIZE));\n  /* Exception handling  */\n  .eh_frame       : ONLY_IF_RW { KEEP (*(.eh_frame)) *(.eh_frame.*) }\n  .gnu_extab      : ONLY_IF_RW { *(.gnu_extab) }\n  .gcc_except_table   : ONLY_IF_RW { *(.gcc_except_table .gcc_except_table.*) }\n  .exception_ranges   : ONLY_IF_RW { *(.exception_ranges*) }\n  /* Thread Local Storage sections  */\n  .tdata\t  :\n   {\n     PROVIDE_HIDDEN (__tdata_start = .);\n     *(.tdata .tdata.* .gnu.linkonce.td.*)\n   }\n  .tbss\t\t  : { *(.tbss .tbss.* .gnu.linkonce.tb.*) *(.tcommon) }\n  .preinit_array    :\n  {\n    PROVIDE_HIDDEN (__preinit_array_start = .);\n    KEEP (*(.preinit_array))\n    PROVIDE_HIDDEN (__preinit_array_end = .);\n  }\n  .init_array    :\n  {\n    PROVIDE_HIDDEN (__init_array_start = .);\n    KEEP (*(SORT_BY_INIT_PRIORITY(.init_array.*) SORT_BY_INIT_PRIORITY(.ctors.*)))\n    KEEP (*(.init_array EXCLUDE_FILE (*crtbegin.o *crtbegin?.o *crtend.o *crtend?.o ) .ctors))\n    PROVIDE_HIDDEN (__init_array_end = .);\n  }\n  .fini_array    :\n  {\n    PROVIDE_HIDDEN (__fini_array_start = .);\n    KEEP (*(SORT_BY_INIT_PRIORITY(.fini_array.*) SORT_BY_INIT_PRIORITY(.dtors.*)))\n    KEEP (*(.fini_array EXCLUDE_FILE (*crtbegin.o *crtbegin?.o *crtend.o *crtend?.o ) .dtors))\n    PROVIDE_HIDDEN (__fini_array_end = .);\n  }\n  .ctors          :\n  {\n    /* gcc uses crtbegin.o to find the start of\n       the constructors, so we make sure it is\n       first.  Because this is a wildcard, it\n       doesn't matter if the user does not\n       actually link against crtbegin.o; the\n       linker won't look for a file to match a\n       wildcard.  The wildcard also means that it\n       doesn't matter which directory crtbegin.o\n       is in.  */\n    KEEP (*crtbegin.o(.ctors))\n    KEEP (*crtbegin?.o(.ctors))\n    /* We don't want to include the .ctor section from\n       the crtend.o file until after the sorted ctors.\n       The .ctor section from the crtend file contains the\n       end of ctors marker and it must be last */\n    KEEP (*(EXCLUDE_FILE (*crtend.o *crtend?.o ) .ctors))\n    KEEP (*(SORT(.ctors.*)))\n    KEEP (*(.ctors))\n  }\n  .dtors          :\n  {\n    KEEP (*crtbegin.o(.dtors))\n    KEEP (*crtbegin?.o(.dtors))\n    KEEP (*(EXCLUDE_FILE (*crtend.o *crtend?.o ) .dtors))\n    KEEP (*(SORT(.dtors.*)))\n    KEEP (*(.dtors))\n  }\n  .jcr            : { KEEP (*(.jcr)) }\n  .data.rel.ro : { *(.data.rel.ro.local* .gnu.linkonce.d.rel.ro.local.*) *(.data.rel.ro .data.rel.ro.* .gnu.linkonce.d.rel.ro.*) }\n  .dynamic        : { *(.dynamic) }\n  .got            : { *(.got) *(.igot) }\n  . = DATA_SEGMENT_RELRO_END (SIZEOF (.got.plt) >= 24 ? 24 : 0, .);\n  .got.plt        : { *(.got.plt) *(.igot.plt) }\n  .data           :\n  {\n    *(.data .data.* .gnu.linkonce.d.*)\n    SORT(CONSTRUCTORS)\n  }\n  .data1          : { *(.data1) }\n  _edata = .; PROVIDE (edata = .);\n  . = .;\n  __bss_start = .;\n  .bss            :\n  {\n   *(.dynbss)\n   *(.bss .bss.* .gnu.linkonce.b.*)\n   *(COMMON)\n   /* Align here to ensure that the .bss section occupies space up to\n      _end.  Align after .bss to ensure correct alignment even if the\n      .bss section disappears because there are no input sections.\n      FIXME: Why do we need it? When there is no .bss section, we do not\n      pad the .data section.  */\n   . = ALIGN(. != 0 ? 64 / 8 : 1);\n  }\n  .lbss   :\n  {\n    *(.dynlbss)\n    *(.lbss .lbss.* .gnu.linkonce.lb.*)\n    *(LARGE_COMMON)\n  }\n  . = ALIGN(64 / 8);\n  . = SEGMENT_START(\"ldata-segment\", .);\n  .lrodata   ALIGN(CONSTANT (MAXPAGESIZE)) + (. & (CONSTANT (MAXPAGESIZE) - 1)) :\n  {\n    *(.lrodata .lrodata.* .gnu.linkonce.lr.*)\n  }\n  .ldata   ALIGN(CONSTANT (MAXPAGESIZE)) + (. & (CONSTANT (MAXPAGESIZE) - 1)) :\n  {\n    *(.ldata .ldata.* .gnu.linkonce.l.*)\n    . = ALIGN(. != 0 ? 64 / 8 : 1);\n  }\n  . = ALIGN(64 / 8);\n  _end = .; PROVIDE (end = .);\n  . = DATA_SEGMENT_END (.);\n  /* Stabs debugging sections.  */\n  .stab          0 : { *(.stab) }\n  .stabstr       0 : { *(.stabstr) }\n  .stab.excl     0 : { *(.stab.excl) }\n  .stab.exclstr  0 : { *(.stab.exclstr) }\n  .stab.index    0 : { *(.stab.index) }\n  .stab.indexstr 0 : { *(.stab.indexstr) }\n  .comment       0 : { *(.comment) }\n  .gnu.build.attributes : { *(.gnu.build.attributes .gnu.build.attributes.*) }\n  /* DWARF debug sections.\n     Symbols in the DWARF debugging sections are relative to the beginning\n     of the section so we begin them at 0.  */\n  /* DWARF 1.  */\n  .debug          0 : { *(.debug) }\n  .line           0 : { *(.line) }\n  /* GNU DWARF 1 extensions.  */\n  .debug_srcinfo  0 : { *(.debug_srcinfo) }\n  .debug_sfnames  0 : { *(.debug_sfnames) }\n  /* DWARF 1.1 and DWARF 2.  */\n  .debug_aranges  0 : { *(.debug_aranges) }\n  .debug_pubnames 0 : { *(.debug_pubnames) }\n  /* DWARF 2.  */\n  .debug_info     0 : { *(.debug_info .gnu.linkonce.wi.*) }\n  .debug_abbrev   0 : { *(.debug_abbrev) }\n  .debug_line     0 : { *(.debug_line .debug_line.* .debug_line_end) }\n  .debug_frame    0 : { *(.debug_frame) }\n  .debug_str      0 : { *(.debug_str) }\n  .debug_loc      0 : { *(.debug_loc) }\n  .debug_macinfo  0 : { *(.debug_macinfo) }\n  /* SGI/MIPS DWARF 2 extensions.  */\n  .debug_weaknames 0 : { *(.debug_weaknames) }\n  .debug_funcnames 0 : { *(.debug_funcnames) }\n  .debug_typenames 0 : { *(.debug_typenames) }\n  .debug_varnames  0 : { *(.debug_varnames) }\n  /* DWARF 3.  */\n  .debug_pubtypes 0 : { *(.debug_pubtypes) }\n  .debug_ranges   0 : { *(.debug_ranges) }\n  /* DWARF 5.  */\n  .debug_addr     0 : { *(.debug_addr) }\n  .debug_line_str 0 : { *(.debug_line_str) }\n  .debug_loclists 0 : { *(.debug_loclists) }\n  .debug_macro    0 : { *(.debug_macro) }\n  .debug_names    0 : { *(.debug_names) }\n  .debug_rnglists 0 : { *(.debug_rnglists) }\n  .debug_str_offsets 0 : { *(.debug_str_offsets) }\n  .debug_sup      0 : { *(.debug_sup) }\n  .gnu.attributes 0 : { KEEP (*(.gnu.attributes)) }\n  /DISCARD/ : { *(.note.GNU-stack) *(.gnu_debuglink) *(.gnu.lto_*) }\n}\n\n\0"
            as *const u8 as *const libc::c_char as *mut libc::c_char
    };
}
unsafe extern "C" fn gldelf_x86_64_add_options(
    mut ns: libc::c_int,
    mut shortopts: *mut *mut libc::c_char,
    mut nl: libc::c_int,
    mut longopts: *mut *mut option,
    mut nrl: libc::c_int,
    mut really_longopts: *mut *mut option,
) {
    static mut xtra_short: [libc::c_char; 5] = unsafe {
        *::core::mem::transmute::<&[u8; 5], &[libc::c_char; 5]>(b"z:P:\0")
    };
    static mut xtra_long: [option; 14] = [
        {
            let mut init = option {
                name: b"audit\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: OPTION_AUDIT as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"Bgroup\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: OPTION_GROUP as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"build-id\0" as *const u8 as *const libc::c_char,
                has_arg: 2 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: OPTION_BUILD_ID as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"compress-debug-sections\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: OPTION_COMPRESS_DEBUG as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"depaudit\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'P' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"disable-new-dtags\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: OPTION_DISABLE_NEW_DTAGS as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"enable-new-dtags\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: OPTION_ENABLE_NEW_DTAGS as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"eh-frame-hdr\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: OPTION_EH_FRAME_HDR as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"no-eh-frame-hdr\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: OPTION_NO_EH_FRAME_HDR as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"exclude-libs\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: OPTION_EXCLUDE_LIBS as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"hash-style\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: OPTION_HASH_STYLE as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"ld-generated-unwind-info\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 301 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"no-ld-generated-unwind-info\0" as *const u8
                    as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 302 as libc::c_int,
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
    *shortopts = xrealloc(
        *shortopts as *mut libc::c_void,
        (ns as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong),
    ) as *mut libc::c_char;
    memcpy(
        (*shortopts).offset(ns as isize) as *mut libc::c_void,
        &xtra_short as *const [libc::c_char; 5] as *const libc::c_void,
        ::core::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong,
    );
    *longopts = xrealloc(
        *longopts as *mut libc::c_void,
        (nl as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<option>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<[option; 14]>() as libc::c_ulong),
    ) as *mut option;
    memcpy(
        (*longopts).offset(nl as isize) as *mut libc::c_void,
        &xtra_long as *const [option; 14] as *const libc::c_void,
        ::core::mem::size_of::<[option; 14]>() as libc::c_ulong,
    );
}
unsafe extern "C" fn gldelf_x86_64_handle_option(mut optc: libc::c_int) -> bool {
    match optc {
        407 => {
            free(ldelf_emit_note_gnu_build_id as *mut libc::c_char as *mut libc::c_void);
            ldelf_emit_note_gnu_build_id = 0 as *const libc::c_char;
            if optarg.is_null() {
                optarg = b"sha1\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
            }
            if strcmp(optarg, b"none\0" as *const u8 as *const libc::c_char) != 0 {
                ldelf_emit_note_gnu_build_id = xstrdup(optarg);
            }
        }
        409 => {
            if strcasecmp(optarg, b"none\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                link_info.compress_debug = COMPRESS_DEBUG_NONE;
            } else if strcasecmp(optarg, b"zlib\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                link_info.compress_debug = COMPRESS_DEBUG_GABI_ZLIB;
            } else if strcasecmp(
                optarg,
                b"zlib-gnu\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                link_info.compress_debug = COMPRESS_DEBUG_GNU_ZLIB;
            } else if strcasecmp(
                optarg,
                b"zlib-gabi\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                link_info.compress_debug = COMPRESS_DEBUG_GABI_ZLIB;
            } else {
                einfo(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%F%P: invalid --compress-debug-sections option: `%s'\n\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    optarg,
                );
            }
        }
        408 => {
            ldelf_append_to_separated_string(&mut audit, optarg);
        }
        80 => {
            ldelf_append_to_separated_string(&mut depaudit, optarg);
        }
        400 => {
            link_info.set_new_dtags(0 as libc::c_int as libc::c_uint);
        }
        401 => {
            link_info.set_new_dtags(1 as libc::c_int as libc::c_uint);
        }
        403 => {
            link_info.set_eh_frame_hdr_type(1 as libc::c_int as libc::c_uint);
        }
        404 => {
            link_info.set_eh_frame_hdr_type(0 as libc::c_int as libc::c_uint);
        }
        402 => {
            link_info.flags_1 |= 0x4 as libc::c_int as bfd_vma;
            link_info.set_unresolved_syms_in_objects(RM_DIAGNOSE);
            link_info.set_unresolved_syms_in_shared_libs(RM_DIAGNOSE);
        }
        405 => {
            add_excluded_libs(optarg);
        }
        406 => {
            link_info.set_emit_hash(0 as libc::c_int as libc::c_uint);
            link_info.set_emit_gnu_hash(0 as libc::c_int as libc::c_uint);
            if strcmp(optarg, b"sysv\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                link_info.set_emit_hash(1 as libc::c_int as libc::c_uint);
            } else if strcmp(optarg, b"gnu\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                link_info.set_emit_gnu_hash(1 as libc::c_int as libc::c_uint);
            } else if strcmp(optarg, b"both\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                link_info.set_emit_hash(1 as libc::c_int as libc::c_uint);
                link_info.set_emit_gnu_hash(1 as libc::c_int as libc::c_uint);
            } else {
                einfo(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%F%P: invalid hash style `%s'\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    optarg,
                );
            }
        }
        122 => {
            if strcmp(optarg, b"defs\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                link_info.set_unresolved_syms_in_objects(RM_DIAGNOSE);
            } else if strcmp(optarg, b"undefs\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                link_info.set_unresolved_syms_in_objects(RM_IGNORE);
            } else if strcmp(optarg, b"muldefs\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                link_info
                    .set_allow_multiple_definition(1 as libc::c_int as libc::c_uint);
            } else if startswith(
                optarg,
                b"max-page-size=\0" as *const u8 as *const libc::c_char,
            ) {
                let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
                link_info
                    .maxpagesize = strtoul(
                    optarg.offset(14 as libc::c_int as isize),
                    &mut end,
                    0 as libc::c_int,
                );
                if *end as libc::c_int != 0
                    || link_info.maxpagesize
                        & (link_info.maxpagesize)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        != 0 as libc::c_int as libc::c_ulong
                {
                    einfo(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"%F%P: invalid maximum page size `%s'\n\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        optarg.offset(14 as libc::c_int as isize),
                    );
                }
            } else if startswith(
                optarg,
                b"common-page-size=\0" as *const u8 as *const libc::c_char,
            ) {
                let mut end_0: *mut libc::c_char = 0 as *mut libc::c_char;
                link_info
                    .commonpagesize = strtoul(
                    optarg.offset(17 as libc::c_int as isize),
                    &mut end_0,
                    0 as libc::c_int,
                );
                if *end_0 as libc::c_int != 0
                    || link_info.commonpagesize
                        & (link_info.commonpagesize)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        != 0 as libc::c_int as libc::c_ulong
                {
                    einfo(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"%F%P: invalid common page size `%s'\n\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        optarg.offset(17 as libc::c_int as isize),
                    );
                }
            } else if startswith(
                optarg,
                b"stack-size=\0" as *const u8 as *const libc::c_char,
            ) {
                let mut end_1: *mut libc::c_char = 0 as *mut libc::c_char;
                link_info
                    .stacksize = strtoul(
                    optarg.offset(11 as libc::c_int as isize),
                    &mut end_1,
                    0 as libc::c_int,
                ) as bfd_signed_vma;
                if *end_1 as libc::c_int != 0
                    || link_info.stacksize < 0 as libc::c_int as libc::c_long
                {
                    einfo(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"%F%P: invalid stack size `%s'\n\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        optarg.offset(11 as libc::c_int as isize),
                    );
                }
                if link_info.stacksize == 0 {
                    link_info.stacksize = -(1 as libc::c_int) as bfd_signed_vma;
                }
            } else if strcmp(optarg, b"execstack\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                link_info.set_execstack(1 as libc::c_int as libc::c_uint);
                link_info.set_noexecstack(0 as libc::c_int as libc::c_uint);
            } else if strcmp(
                optarg,
                b"noexecstack\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                link_info.set_noexecstack(1 as libc::c_int as libc::c_uint);
                link_info.set_execstack(0 as libc::c_int as libc::c_uint);
            } else if strcmp(
                optarg,
                b"unique-symbol\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                link_info.set_unique_symbol(1 as libc::c_int as libc::c_uint);
            } else if strcmp(
                optarg,
                b"nounique-symbol\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                link_info.set_unique_symbol(0 as libc::c_int as libc::c_uint);
            } else if strcmp(
                optarg,
                b"globalaudit\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                link_info.flags_1 |= 0x1000000 as libc::c_int as libc::c_ulong;
            } else if startswith(
                optarg,
                b"start-stop-gc\0" as *const u8 as *const libc::c_char,
            ) {
                link_info.start_stop_gc = 1 as libc::c_int;
            } else if startswith(
                optarg,
                b"nostart-stop-gc\0" as *const u8 as *const libc::c_char,
            ) {
                link_info.start_stop_gc = 0 as libc::c_int;
            } else if startswith(
                optarg,
                b"start-stop-visibility=\0" as *const u8 as *const libc::c_char,
            ) {
                if strcmp(
                    optarg,
                    b"start-stop-visibility=default\0" as *const u8
                        as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    link_info.start_stop_visibility = 0 as libc::c_int as libc::c_uint;
                } else if strcmp(
                    optarg,
                    b"start-stop-visibility=internal\0" as *const u8
                        as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    link_info.start_stop_visibility = 1 as libc::c_int as libc::c_uint;
                } else if strcmp(
                    optarg,
                    b"start-stop-visibility=hidden\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    link_info.start_stop_visibility = 2 as libc::c_int as libc::c_uint;
                } else if strcmp(
                    optarg,
                    b"start-stop-visibility=protected\0" as *const u8
                        as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    link_info.start_stop_visibility = 3 as libc::c_int as libc::c_uint;
                } else {
                    einfo(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"%F%P: invalid visibility in `-z %s'; must be default, internal, hidden, or protected\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        optarg,
                    );
                }
            } else if strcmp(optarg, b"global\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                link_info.flags_1 |= 0x2 as libc::c_int as bfd_vma;
            } else if strcmp(optarg, b"initfirst\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                link_info.flags_1 |= 0x20 as libc::c_int as bfd_vma;
            } else if strcmp(optarg, b"interpose\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                link_info.flags_1 |= 0x400 as libc::c_int as bfd_vma;
            } else if strcmp(optarg, b"loadfltr\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                link_info.flags_1 |= 0x10 as libc::c_int as bfd_vma;
            } else if strcmp(
                optarg,
                b"nodefaultlib\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                link_info.flags_1 |= 0x800 as libc::c_int as bfd_vma;
            } else if strcmp(optarg, b"nodelete\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                link_info.flags_1 |= 0x8 as libc::c_int as bfd_vma;
            } else if strcmp(optarg, b"nodlopen\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                link_info.flags_1 |= 0x40 as libc::c_int as bfd_vma;
            } else if strcmp(optarg, b"nodump\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                link_info.flags_1 |= 0x1000 as libc::c_int as bfd_vma;
            } else if strcmp(optarg, b"now\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                link_info.flags |= ((1 as libc::c_int) << 3 as libc::c_int) as bfd_vma;
                link_info.flags_1 |= 0x1 as libc::c_int as bfd_vma;
            } else if strcmp(optarg, b"lazy\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                link_info.flags
                    &= !(((1 as libc::c_int) << 3 as libc::c_int) as bfd_vma);
                link_info.flags_1 &= !(0x1 as libc::c_int as bfd_vma);
            } else if strcmp(optarg, b"origin\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                link_info.flags |= ((1 as libc::c_int) << 0 as libc::c_int) as bfd_vma;
                link_info.flags_1 |= 0x80 as libc::c_int as bfd_vma;
            } else if strcmp(optarg, b"unique\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                link_info.gnu_flags_1 |= 0x1 as libc::c_int as bfd_vma;
            } else if strcmp(optarg, b"nounique\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                link_info.gnu_flags_1 &= !(0x1 as libc::c_int as bfd_vma);
            } else if strcmp(optarg, b"combreloc\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                link_info.set_combreloc(1 as libc::c_int as libc::c_uint);
            } else if strcmp(
                optarg,
                b"nocombreloc\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                link_info.set_combreloc(0 as libc::c_int as libc::c_uint);
            } else if strcmp(
                optarg,
                b"nocopyreloc\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                link_info.set_nocopyreloc(1 as libc::c_int as libc::c_uint);
            } else if strcmp(optarg, b"relro\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                link_info.set_relro(1 as libc::c_int as libc::c_uint);
            } else if strcmp(optarg, b"norelro\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                link_info.set_relro(0 as libc::c_int as libc::c_uint);
            } else if strcmp(
                optarg,
                b"separate-code\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                link_info.set_separate_code(1 as libc::c_int as libc::c_uint);
            } else if strcmp(
                optarg,
                b"noseparate-code\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                link_info.set_separate_code(0 as libc::c_int as libc::c_uint);
            } else if strcmp(optarg, b"common\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                link_info.set_elf_stt_common(elf_stt_common);
            } else if strcmp(optarg, b"nocommon\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                link_info.set_elf_stt_common(no_elf_stt_common);
            } else if strcmp(optarg, b"text\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                link_info.set_textrel_check(textrel_check_error);
            } else if strcmp(optarg, b"notext\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                link_info.set_textrel_check(textrel_check_none);
            } else if strcmp(optarg, b"textoff\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                link_info.set_textrel_check(textrel_check_none);
            } else if strcmp(
                optarg,
                b"noextern-protected-data\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                link_info.extern_protected_data = 0 as libc::c_int;
            } else if strcmp(
                optarg,
                b"dynamic-undefined-weak\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                link_info.dynamic_undefined_weak = 1 as libc::c_int;
            } else if strcmp(
                optarg,
                b"nodynamic-undefined-weak\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                link_info.dynamic_undefined_weak = 0 as libc::c_int;
            } else if strcmp(
                optarg,
                b"noreloc-overflow\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                params.set_no_reloc_overflow_check(1 as libc::c_int as libc::c_uint);
            } else if strncmp(
                optarg,
                b"call-nop=\0" as *const u8 as *const libc::c_char,
                9 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int
            {
                if strcmp(
                    optarg.offset(9 as libc::c_int as isize),
                    b"prefix-addr\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    params.set_call_nop_as_suffix(0 as libc::c_int as libc::c_uint);
                    params.call_nop_byte = 0x67 as libc::c_int as libc::c_char;
                } else if strcmp(
                    optarg.offset(9 as libc::c_int as isize),
                    b"suffix-nop\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    params.set_call_nop_as_suffix(1 as libc::c_int as libc::c_uint);
                    params.call_nop_byte = 0x90 as libc::c_int as libc::c_char;
                } else if strncmp(
                    optarg.offset(9 as libc::c_int as isize),
                    b"prefix-\0" as *const u8 as *const libc::c_char,
                    7 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    let mut end_2: *mut libc::c_char = 0 as *mut libc::c_char;
                    params
                        .call_nop_byte = strtoul(
                        optarg.offset(16 as libc::c_int as isize),
                        &mut end_2,
                        0 as libc::c_int,
                    ) as libc::c_char;
                    if *end_2 != 0 {
                        einfo(
                            dcgettext(
                                0 as *const libc::c_char,
                                b"%F%P: invalid number for -z call-nop=prefix-: %s\n\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            optarg.offset(16 as libc::c_int as isize),
                        );
                    }
                    params.set_call_nop_as_suffix(0 as libc::c_int as libc::c_uint);
                } else if strncmp(
                    optarg.offset(9 as libc::c_int as isize),
                    b"suffix-\0" as *const u8 as *const libc::c_char,
                    7 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    let mut end_3: *mut libc::c_char = 0 as *mut libc::c_char;
                    params
                        .call_nop_byte = strtoul(
                        optarg.offset(16 as libc::c_int as isize),
                        &mut end_3,
                        0 as libc::c_int,
                    ) as libc::c_char;
                    if *end_3 != 0 {
                        einfo(
                            dcgettext(
                                0 as *const libc::c_char,
                                b"%F%P: invalid number for -z call-nop=suffix-: %s\n\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            optarg.offset(16 as libc::c_int as isize),
                        );
                    }
                    params.set_call_nop_as_suffix(1 as libc::c_int as libc::c_uint);
                } else {
                    einfo(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"%F%P: unsupported option: -z %s\n\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        optarg,
                    );
                }
            } else if strcmp(optarg, b"ibtplt\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                params.set_ibtplt(1 as libc::c_int as libc::c_uint);
            } else if strcmp(optarg, b"ibt\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                params.set_ibt(1 as libc::c_int as libc::c_uint);
            } else if strcmp(optarg, b"shstk\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                params.set_shstk(1 as libc::c_int as libc::c_uint);
            } else if strncmp(
                optarg,
                b"cet-report=\0" as *const u8 as *const libc::c_char,
                11 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int
            {
                if strcmp(
                    optarg.offset(11 as libc::c_int as isize),
                    b"none\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    params.cet_report = prop_report_none;
                } else if strcmp(
                    optarg.offset(11 as libc::c_int as isize),
                    b"warning\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    params
                        .cet_report = (prop_report_warning as libc::c_int
                        | prop_report_ibt as libc::c_int
                        | prop_report_shstk as libc::c_int) as elf_x86_prop_report;
                } else if strcmp(
                    optarg.offset(11 as libc::c_int as isize),
                    b"error\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    params
                        .cet_report = (prop_report_error as libc::c_int
                        | prop_report_ibt as libc::c_int
                        | prop_report_shstk as libc::c_int) as elf_x86_prop_report;
                } else {
                    einfo(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"%F%P: invalid option for -z cet-report=: %s\n\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        optarg.offset(11 as libc::c_int as isize),
                    );
                }
            } else if strcmp(
                optarg,
                b"report-relative-reloc\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                params.set_report_relative_reloc(1 as libc::c_int as libc::c_uint);
            } else if strcmp(
                optarg,
                b"x86-64-baseline\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                params.isa_level = 1 as libc::c_int as libc::c_uint;
            } else if strncmp(
                optarg,
                b"x86-64-v\0" as *const u8 as *const libc::c_char,
                8 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int
            {
                let mut end_4: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut level: libc::c_uint = strtoul(
                    optarg.offset(8 as libc::c_int as isize),
                    &mut end_4,
                    10 as libc::c_int,
                ) as libc::c_uint;
                if *end_4 as libc::c_int != 0 as libc::c_int
                    || level < 2 as libc::c_int as libc::c_uint
                    || level > 4 as libc::c_int as libc::c_uint
                {
                    einfo(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"%F%P: invalid x86-64 ISA level: %s\n\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        optarg,
                    );
                }
                params.isa_level = level;
            } else if strcmp(optarg, b"lam-u48\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                params.set_lam_u48(1 as libc::c_int as libc::c_uint);
            } else if strncmp(
                optarg,
                b"lam-u48-report=\0" as *const u8 as *const libc::c_char,
                15 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int
            {
                if strcmp(
                    optarg.offset(15 as libc::c_int as isize),
                    b"none\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    params.lam_u48_report = prop_report_none;
                } else if strcmp(
                    optarg.offset(15 as libc::c_int as isize),
                    b"warning\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    params.lam_u48_report = prop_report_warning;
                } else if strcmp(
                    optarg.offset(15 as libc::c_int as isize),
                    b"error\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    params.lam_u48_report = prop_report_error;
                } else {
                    einfo(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"%F%P: invalid option for -z lam-u48-report=: %s\n\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        optarg.offset(15 as libc::c_int as isize),
                    );
                }
            } else if strcmp(optarg, b"lam-u57\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                params.set_lam_u57(1 as libc::c_int as libc::c_uint);
            } else if strncmp(
                optarg,
                b"lam-u57-report=\0" as *const u8 as *const libc::c_char,
                15 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int
            {
                if strcmp(
                    optarg.offset(15 as libc::c_int as isize),
                    b"none\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    params.lam_u57_report = prop_report_none;
                } else if strcmp(
                    optarg.offset(15 as libc::c_int as isize),
                    b"warning\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    params.lam_u57_report = prop_report_warning;
                } else if strcmp(
                    optarg.offset(15 as libc::c_int as isize),
                    b"error\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    params.lam_u57_report = prop_report_error;
                } else {
                    einfo(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"%F%P: invalid option for -z lam-u57-report=: %s\n\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        optarg.offset(15 as libc::c_int as isize),
                    );
                }
            } else if strncmp(
                optarg,
                b"lam-report=\0" as *const u8 as *const libc::c_char,
                11 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int
            {
                if strcmp(
                    optarg.offset(11 as libc::c_int as isize),
                    b"none\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    params.lam_u48_report = prop_report_none;
                    params.lam_u57_report = prop_report_none;
                } else if strcmp(
                    optarg.offset(11 as libc::c_int as isize),
                    b"warning\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    params.lam_u48_report = prop_report_warning;
                    params.lam_u57_report = prop_report_warning;
                } else if strcmp(
                    optarg.offset(11 as libc::c_int as isize),
                    b"error\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    params.lam_u48_report = prop_report_error;
                    params.lam_u57_report = prop_report_error;
                } else {
                    einfo(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"%F%P: invalid option for -z lam-report=: %s\n\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        optarg.offset(11 as libc::c_int as isize),
                    );
                }
            } else if strcmp(optarg, b"bndplt\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                params.set_bndplt(1 as libc::c_int as libc::c_uint);
            } else {
                einfo(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%P: warning: -z %s ignored\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    optarg,
                );
            }
        }
        301 => {
            link_info.set_no_ld_generated_unwind_info(0 as libc::c_int as libc::c_uint);
        }
        302 => {
            link_info.set_no_ld_generated_unwind_info(1 as libc::c_int as libc::c_uint);
        }
        155 => {
            params.set_has_dynamic_linker(1 as libc::c_int as libc::c_uint);
            return 0 as libc::c_int != 0;
        }
        172 => {
            if !lang_has_input_file {
                params.set_static_before_all_inputs(1 as libc::c_int as libc::c_uint);
            }
            return 0 as libc::c_int != 0;
        }
        _ => return 0 as libc::c_int != 0,
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn gldelf_x86_64_list_options(mut file: *mut FILE) {
    fprintf(
        file,
        dcgettext(
            0 as *const libc::c_char,
            b"  -z noextern-protected-data  Do not treat protected data symbol as external\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    fprintf(
        file,
        dcgettext(
            0 as *const libc::c_char,
            b"  -z dynamic-undefined-weak   Make undefined weak symbols dynamic\n  -z nodynamic-undefined-weak Do not make undefined weak symbols dynamic\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    fprintf(
        file,
        dcgettext(
            0 as *const libc::c_char,
            b"  -z noreloc-overflow         Disable relocation overflow check\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    fprintf(
        file,
        dcgettext(
            0 as *const libc::c_char,
            b"  -z call-nop=PADDING         Use PADDING as 1-byte NOP for branch\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    fprintf(
        file,
        dcgettext(
            0 as *const libc::c_char,
            b"  -z ibtplt                   Generate IBT-enabled PLT entries\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    fprintf(
        file,
        dcgettext(
            0 as *const libc::c_char,
            b"  -z ibt                      Generate GNU_PROPERTY_X86_FEATURE_1_IBT\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    fprintf(
        file,
        dcgettext(
            0 as *const libc::c_char,
            b"  -z shstk                    Generate GNU_PROPERTY_X86_FEATURE_1_SHSTK\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    fprintf(
        file,
        dcgettext(
            0 as *const libc::c_char,
            b"  -z cet-report=[none|warning|error] (default: none)\n                              Report missing IBT and SHSTK properties\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    fprintf(
        file,
        dcgettext(
            0 as *const libc::c_char,
            b"  -z report-relative-reloc    Report relative relocations\n\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    fprintf(
        file,
        dcgettext(
            0 as *const libc::c_char,
            b"  -z x86-64-{baseline|v[234]} Mark x86-64-{baseline|v[234]} ISA level as needed\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    fprintf(
        file,
        dcgettext(
            0 as *const libc::c_char,
            b"  -z lam-u48                  Generate GNU_PROPERTY_X86_FEATURE_1_LAM_U48\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    fprintf(
        file,
        dcgettext(
            0 as *const libc::c_char,
            b"  -z lam-u48-report=[none|warning|error] (default: none)\n                              Report missing LAM_U48 property\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    fprintf(
        file,
        dcgettext(
            0 as *const libc::c_char,
            b"  -z lam-u57                  Generate GNU_PROPERTY_X86_FEATURE_1_LAM_U57\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    fprintf(
        file,
        dcgettext(
            0 as *const libc::c_char,
            b"  -z lam-u57-report=[none|warning|error] (default: none)\n                              Report missing LAM_U57 property\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    fprintf(
        file,
        dcgettext(
            0 as *const libc::c_char,
            b"  -z lam-report=[none|warning|error] (default: none)\n                              Report missing LAM_U48 and LAM_U57 properties\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    fprintf(
        file,
        dcgettext(
            0 as *const libc::c_char,
            b"  -z bndplt                   Always generate BND prefix in PLT entries\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
}
#[no_mangle]
pub static mut ld_elf_x86_64_emulation: ld_emulation_xfer_struct = unsafe {
    {
        let mut init = ld_emulation_xfer_struct {
            before_parse: Some(elf_x86_before_parse as unsafe extern "C" fn() -> ()),
            syslib: Some(
                syslib_default as unsafe extern "C" fn(*mut libc::c_char) -> (),
            ),
            hll: Some(hll_default as unsafe extern "C" fn(*mut libc::c_char) -> ()),
            after_parse: Some(ldelf_after_parse as unsafe extern "C" fn() -> ()),
            after_open: Some(gldelf_x86_64_after_open as unsafe extern "C" fn() -> ()),
            after_check_relocs: Some(
                after_check_relocs_default as unsafe extern "C" fn() -> (),
            ),
            before_place_orphans: Some(
                ldelf_before_place_orphans as unsafe extern "C" fn() -> (),
            ),
            after_allocation: Some(
                gldelf_x86_64_after_allocation as unsafe extern "C" fn() -> (),
            ),
            set_output_arch: Some(ldelf_set_output_arch as unsafe extern "C" fn() -> ()),
            choose_target: Some(
                ldemul_default_target
                    as unsafe extern "C" fn(
                        libc::c_int,
                        *mut *mut libc::c_char,
                    ) -> *mut libc::c_char,
            ),
            before_allocation: Some(
                gldelf_x86_64_before_allocation as unsafe extern "C" fn() -> (),
            ),
            get_script: Some(
                gldelf_x86_64_get_script
                    as unsafe extern "C" fn(*mut libc::c_int) -> *mut libc::c_char,
            ),
            emulation_name: b"elf_x86_64\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            target_name: b"elf64-x86-64\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            finish: Some(finish_default as unsafe extern "C" fn() -> ()),
            create_output_section_statements: Some(
                elf_x86_create_output_section_statements as unsafe extern "C" fn() -> (),
            ),
            open_dynamic_archive: Some(
                ldelf_open_dynamic_archive
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        *mut search_dirs_type,
                        *mut lang_input_statement_type,
                    ) -> bool,
            ),
            place_orphan: Some(
                ldelf_place_orphan
                    as unsafe extern "C" fn(
                        *mut asection,
                        *const libc::c_char,
                        libc::c_int,
                    ) -> *mut lang_output_section_statement_type,
            ),
            set_symbols: None,
            parse_args: None,
            add_options: Some(
                gldelf_x86_64_add_options
                    as unsafe extern "C" fn(
                        libc::c_int,
                        *mut *mut libc::c_char,
                        libc::c_int,
                        *mut *mut option,
                        libc::c_int,
                        *mut *mut option,
                    ) -> (),
            ),
            handle_option: Some(
                gldelf_x86_64_handle_option as unsafe extern "C" fn(libc::c_int) -> bool,
            ),
            unrecognized_file: None,
            list_options: Some(
                gldelf_x86_64_list_options as unsafe extern "C" fn(*mut FILE) -> (),
            ),
            recognized_file: Some(
                ldelf_load_symbols
                    as unsafe extern "C" fn(*mut lang_input_statement_type) -> bool,
            ),
            find_potential_libraries: None,
            new_vers_pattern: None,
            extra_map_file_text: None,
            emit_ctf_early: Some(
                ldelf_emit_ctf_early as unsafe extern "C" fn() -> libc::c_int,
            ),
            acquire_strings_for_ctf: Some(
                ldelf_acquire_strings_for_ctf
                    as unsafe extern "C" fn(*mut ctf_dict, *mut elf_strtab_hash) -> (),
            ),
            new_dynsym_for_ctf: Some(
                ldelf_new_dynsym_for_ctf
                    as unsafe extern "C" fn(
                        *mut ctf_dict,
                        libc::c_int,
                        *mut elf_internal_sym,
                    ) -> (),
            ),
            print_symbol: None,
        };
        init
    }
};