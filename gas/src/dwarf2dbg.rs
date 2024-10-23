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
    pub type artdata;
    pub type aout_data_struct;
    pub type bfd_iovec;
    pub type _bfd_window_internal;
    pub type symbol;
    pub type list_info_struct;
    pub type lineno_list;
    pub type frch_cfi_data;
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
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn ffs(__i: libc::c_int) -> libc::c_int;
    fn free(_: *mut libc::c_void);
    static mut _bfd_std_section: [asection; 4];
    fn bfd_get_section_by_name(
        abfd: *mut bfd,
        name: *const libc::c_char,
    ) -> *mut asection;
    fn bfd_set_section_flags(sec: *mut asection, flags: flagword) -> bool;
    fn lbasename(_: *const libc::c_char) -> *const libc::c_char;
    fn concat(_: *const libc::c_char, _: ...) -> *mut libc::c_char;
    fn getpwd() -> *mut libc::c_char;
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    fn xrealloc(_: *mut libc::c_void, _: size_t) -> *mut libc::c_void;
    fn xcalloc(_: size_t, _: size_t) -> *mut libc::c_void;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    static mut stdoutput: *mut bfd;
    static mut now_subseg: subsegT;
    static mut now_seg: segT;
    static mut reg_section: segT;
    static mut expr_section: segT;
    static mut linkrelax: libc::c_int;
    static mut listing: libc::c_int;
    static mut debug_type: debug_info_type;
    static mut flag_dwarf_sections: bool;
    static mut flag_dwarf_cie_version: libc::c_int;
    static mut dwarf_level: libc::c_uint;
    fn as_bad(format: *const libc::c_char, _: ...);
    fn as_fatal(format: *const libc::c_char, _: ...) -> !;
    fn as_warn(format: *const libc::c_char, _: ...);
    fn as_abort(_: *const libc::c_char, _: libc::c_int, _: *const libc::c_char) -> !;
    fn seen_at_least_1_file() -> libc::c_int;
    fn as_where(_: *mut libc::c_uint) -> *const libc::c_char;
    fn subseg_new(_: *const libc::c_char, _: subsegT) -> segT;
    fn subseg_set(_: segT, _: subsegT);
    fn seg_not_empty_p(_: segT) -> libc::c_int;
    fn subseg_get(_: *const libc::c_char, _: libc::c_int) -> segT;
    fn remap_debug_filename(_: *const libc::c_char) -> *const libc::c_char;
    static mut generic_bignum: [LITTLENUM_TYPE; 0];
    fn get_symbol_name(_: *mut *mut libc::c_char) -> libc::c_char;
    fn restore_line_pointer(_: libc::c_char) -> libc::c_char;
    fn expr(_: libc::c_int, _: *mut expressionS, _: expr_mode) -> segT;
    fn make_expr_symbol(expressionP: *mut expressionS) -> *mut symbolS;
    fn resolve_expression(_: *mut expressionS) -> libc::c_int;
    fn x86_dwarf2_addr_size() -> libc::c_int;
    static mut target_big_endian: libc::c_int;
    fn demand_empty_rest_of_line();
    static mut input_line_pointer: *mut libc::c_char;
    fn demand_copy_C_string(len_pointer: *mut libc::c_int) -> *mut libc::c_char;
    fn ignore_rest_of_line();
    fn get_absolute_expression() -> offsetT;
    fn s_app_file(_: libc::c_int);
    fn frag_now_fix_octets() -> addressT;
    fn symbol_get_value_expression(_: *mut symbolS) -> *mut expressionS;
    fn S_IS_DEFINED(_: *mut symbolS) -> libc::c_int;
    static mut zero_address_frag: fragS;
    fn symbol_set_frag(_: *mut symbolS, _: *mut fragS);
    fn S_SET_SEGMENT(_: *mut symbolS, _: segT);
    fn symbol_set_value_expression(_: *mut symbolS, _: *const expressionS);
    fn symbol_temp_make() -> *mut symbolS;
    fn symbol_constant_p(_: *mut symbolS) -> libc::c_int;
    static mut frag_now: *mut fragS;
    fn symbol_temp_new(_: segT, _: *mut fragS, _: valueT) -> *mut symbolS;
    fn symbol_new(
        _: *const libc::c_char,
        _: segT,
        _: *mut fragS,
        _: valueT,
    ) -> *mut symbolS;
    fn S_SET_VALUE(_: *mut symbolS, _: valueT);
    fn read_symbol_name() -> *mut libc::c_char;
    fn S_CAN_BE_REDEFINED(_: *const symbolS) -> libc::c_int;
    fn symbol_clone(_: *mut symbolS, _: libc::c_int) -> *mut symbolS;
    fn S_IS_VOLATILE(_: *const symbolS) -> libc::c_int;
    fn symbol_equated_p(_: *mut symbolS) -> libc::c_int;
    fn symbol_find_or_make(name: *const libc::c_char) -> *mut symbolS;
    fn listing_source_line(_: libc::c_uint);
    fn listing_source_file(_: *const libc::c_char);
    fn S_GET_VALUE(_: *mut symbolS) -> valueT;
    fn frag_now_fix() -> addressT;
    fn S_GET_SEGMENT(_: *mut symbolS) -> segT;
    fn symbol_set_value_now(_: *mut symbolS);
    fn frag_more(nchars: size_t) -> *mut libc::c_char;
    fn number_to_chars_littleendian(_: *mut libc::c_char, _: valueT, _: libc::c_int);
    fn emit_expr(exp: *mut expressionS, nbytes: libc::c_uint);
    fn emit_leb128_expr(_: *mut expressionS, _: libc::c_int);
    fn sizeof_leb128(_: valueT, _: libc::c_int) -> libc::c_uint;
    fn output_leb128(_: *mut libc::c_char, _: valueT, _: libc::c_int) -> libc::c_uint;
    fn frag_append_1_char(_: libc::c_int);
    fn symbol_temp_new_now_octets() -> *mut symbolS;
    fn symbol_make(name: *const libc::c_char) -> *mut symbolS;
    fn record_alignment(_: segT, _: libc::c_uint);
    fn frag_var(
        type_0: relax_stateT,
        max_chars: size_t,
        var: size_t,
        subtype: relax_substateT,
        symbol: *mut symbolS,
        offset: offsetT,
        opcode: *mut libc::c_char,
    ) -> *mut libc::c_char;
    fn symbol_get_frag(_: *mut symbolS) -> *mut fragS;
    fn resolve_symbol_value(_: *mut symbolS) -> valueT;
    fn emit_expr_fix(
        _: *mut expressionS,
        _: libc::c_uint,
        _: *mut fragS,
        _: *mut libc::c_char,
        _: bfd_reloc_code_real_type,
    );
    static mut finalize_syms: libc::c_int;
    fn symbol_resolved_p(_: *mut symbolS) -> libc::c_int;
    static _sch_istable: [libc::c_ushort; 256];
    fn filename_cmp(s1: *const libc::c_char, s2: *const libc::c_char) -> libc::c_int;
    fn filename_ncmp(
        s1: *const libc::c_char,
        s2: *const libc::c_char,
        n: size_t,
    ) -> libc::c_int;
    fn section_symbol(_: segT) -> *mut symbolS;
}
pub type size_t = libc::c_ulong;
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
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct obstack {
    pub chunk_size: size_t,
    pub chunk: *mut _obstack_chunk,
    pub object_base: *mut libc::c_char,
    pub next_free: *mut libc::c_char,
    pub chunk_limit: *mut libc::c_char,
    pub temp: C2RustUnnamed_1,
    pub alignment_mask: size_t,
    pub chunkfun: C2RustUnnamed_0,
    pub freefun: C2RustUnnamed,
    pub extra_arg: *mut libc::c_void,
    #[bitfield(name = "use_extra_arg", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "maybe_empty_object", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "alloc_failed", ty = "libc::c_uint", bits = "2..=2")]
    pub use_extra_arg_maybe_empty_object_alloc_failed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub plain: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub extra: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub plain: Option::<unsafe extern "C" fn(size_t) -> *mut libc::c_void>,
    pub extra: Option::<
        unsafe extern "C" fn(*mut libc::c_void, size_t) -> *mut libc::c_void,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub i: size_t,
    pub p: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _obstack_chunk {
    pub limit: *mut libc::c_char,
    pub prev: *mut _obstack_chunk,
    pub contents: [libc::c_char; 4],
}
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
    pub link: C2RustUnnamed_22,
    pub tdata: C2RustUnnamed_2,
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
pub union C2RustUnnamed_2 {
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
    pub local_got: C2RustUnnamed_18,
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
    pub build_id: C2RustUnnamed_16,
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
    pub map_head: C2RustUnnamed_3,
    pub map_tail: C2RustUnnamed_3,
    pub already_assigned: *mut bfd_section,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
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
    pub u: C2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub indirect: C2RustUnnamed_8,
    pub data: C2RustUnnamed_7,
    pub reloc: C2RustUnnamed_5,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub p: *mut bfd_link_order_reloc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bfd_link_order_reloc {
    pub reloc: bfd_reloc_code_real_type,
    pub u: C2RustUnnamed_6,
    pub addend: bfd_vma,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
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
pub struct C2RustUnnamed_7 {
    pub size: libc::c_uint,
    pub contents: *mut bfd_byte,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
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
    pub udata: C2RustUnnamed_9,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_9 {
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
    pub u: C2RustUnnamed_10,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_10 {
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
    pub u: C2RustUnnamed_11,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_11 {
    pub undef: C2RustUnnamed_15,
    pub def: C2RustUnnamed_14,
    pub i: C2RustUnnamed_13,
    pub c: C2RustUnnamed_12,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_12 {
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
pub struct C2RustUnnamed_13 {
    pub next: *mut bfd_link_hash_entry,
    pub link: *mut bfd_link_hash_entry,
    pub warning: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_14 {
    pub next: *mut bfd_link_hash_entry,
    pub section: *mut asection,
    pub value: bfd_vma,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_15 {
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
pub struct C2RustUnnamed_16 {
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
    pub u: C2RustUnnamed_17,
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
pub union C2RustUnnamed_17 {
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
pub union C2RustUnnamed_18 {
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
    pub u: C2RustUnnamed_21,
    pub verinfo: C2RustUnnamed_20,
    pub u2: C2RustUnnamed_19,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_19 {
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
pub union C2RustUnnamed_20 {
    pub verdef: *mut Elf_Internal_Verdef,
    pub vertree: *mut bfd_elf_version_tree,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_21 {
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
pub union C2RustUnnamed_22 {
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
    pub u: C2RustUnnamed_23,
    pub namidx: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_23 {
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
pub type LITTLENUM_TYPE = libc::c_ushort;
pub type addressT = bfd_vma;
pub type offsetT = bfd_signed_vma;
pub type valueT = addressT;
pub type segT = *mut asection;
pub type subsegT = libc::c_int;
pub type _relax_state = libc::c_uint;
pub const rs_dwarf2dbg: _relax_state = 13;
pub const rs_cfa: _relax_state = 12;
pub const rs_leb128: _relax_state = 11;
pub const rs_fill_nop: _relax_state = 10;
pub const rs_space_nop: _relax_state = 9;
pub const rs_space: _relax_state = 8;
pub const rs_machine_dependent: _relax_state = 7;
pub const rs_broken_word: _relax_state = 6;
pub const rs_org: _relax_state = 5;
pub const rs_align_test: _relax_state = 4;
pub const rs_align_code: _relax_state = 3;
pub const rs_align: _relax_state = 2;
pub const rs_fill: _relax_state = 1;
pub const rs_dummy: _relax_state = 0;
pub type relax_stateT = _relax_state;
pub type relax_substateT = libc::c_uint;
pub type debug_info_type = libc::c_uint;
pub const DEBUG_DWARF2: debug_info_type = 5;
pub const DEBUG_DWARF: debug_info_type = 4;
pub const DEBUG_ECOFF: debug_info_type = 3;
pub const DEBUG_STABS: debug_info_type = 2;
pub const DEBUG_NONE: debug_info_type = 1;
pub const DEBUG_UNSPECIFIED: debug_info_type = 0;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct expressionS {
    pub X_add_symbol: *mut symbolS,
    pub X_op_symbol: *mut symbolS,
    pub X_add_number: offsetT,
    #[bitfield(name = "X_op", ty = "operatorT", bits = "0..=7")]
    #[bitfield(name = "X_unsigned", ty = "libc::c_uint", bits = "8..=8")]
    #[bitfield(name = "X_extrabit", ty = "libc::c_uint", bits = "9..=9")]
    pub X_op_X_unsigned_X_extrabit: [u8; 2],
    pub X_md: libc::c_ushort,
}
pub type operatorT = libc::c_uint;
pub const O_max: operatorT = 62;
pub const O_md32: operatorT = 61;
pub const O_md31: operatorT = 60;
pub const O_md30: operatorT = 59;
pub const O_md29: operatorT = 58;
pub const O_md28: operatorT = 57;
pub const O_md27: operatorT = 56;
pub const O_md26: operatorT = 55;
pub const O_md25: operatorT = 54;
pub const O_md24: operatorT = 53;
pub const O_md23: operatorT = 52;
pub const O_md22: operatorT = 51;
pub const O_md21: operatorT = 50;
pub const O_md20: operatorT = 49;
pub const O_md19: operatorT = 48;
pub const O_md18: operatorT = 47;
pub const O_md17: operatorT = 46;
pub const O_md16: operatorT = 45;
pub const O_md15: operatorT = 44;
pub const O_md14: operatorT = 43;
pub const O_md13: operatorT = 42;
pub const O_md12: operatorT = 41;
pub const O_md11: operatorT = 40;
pub const O_md10: operatorT = 39;
pub const O_md9: operatorT = 38;
pub const O_md8: operatorT = 37;
pub const O_md7: operatorT = 36;
pub const O_md6: operatorT = 35;
pub const O_md5: operatorT = 34;
pub const O_md4: operatorT = 33;
pub const O_md3: operatorT = 32;
pub const O_md2: operatorT = 31;
pub const O_md1: operatorT = 30;
pub const O_index: operatorT = 29;
pub const O_logical_or: operatorT = 28;
pub const O_logical_and: operatorT = 27;
pub const O_gt: operatorT = 26;
pub const O_ge: operatorT = 25;
pub const O_le: operatorT = 24;
pub const O_lt: operatorT = 23;
pub const O_ne: operatorT = 22;
pub const O_eq: operatorT = 21;
pub const O_subtract: operatorT = 20;
pub const O_add: operatorT = 19;
pub const O_bit_and: operatorT = 18;
pub const O_bit_exclusive_or: operatorT = 17;
pub const O_bit_or_not: operatorT = 16;
pub const O_bit_inclusive_or: operatorT = 15;
pub const O_right_shift: operatorT = 14;
pub const O_left_shift: operatorT = 13;
pub const O_modulus: operatorT = 12;
pub const O_divide: operatorT = 11;
pub const O_multiply: operatorT = 10;
pub const O_logical_not: operatorT = 9;
pub const O_bit_not: operatorT = 8;
pub const O_uminus: operatorT = 7;
pub const O_big: operatorT = 6;
pub const O_register: operatorT = 5;
pub const O_symbol_rva: operatorT = 4;
pub const O_symbol: operatorT = 3;
pub const O_constant: operatorT = 2;
pub const O_absent: operatorT = 1;
pub const O_illegal: operatorT = 0;
pub type symbolS = symbol;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct fix {
    pub fx_next: *mut fix,
    #[bitfield(name = "fx_pcrel", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "fx_done", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "fx_no_overflow", ty = "libc::c_uint", bits = "2..=2")]
    #[bitfield(name = "fx_signed", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "fx_tcbit", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "fx_tcbit2", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "fx_unused", ty = "libc::c_uint", bits = "6..=15")]
    #[bitfield(name = "fx_pcrel_adjust", ty = "libc::c_int", bits = "16..=23")]
    #[bitfield(name = "fx_size", ty = "libc::c_uint", bits = "24..=31")]
    pub fx_pcrel_fx_done_fx_no_overflow_fx_signed_fx_tcbit_fx_tcbit2_fx_unused_fx_pcrel_adjust_fx_size: [u8; 4],
    pub fx_r_type: bfd_reloc_code_real_type,
    pub fx_frag: *mut fragS,
    pub fx_where: libc::c_ulong,
    pub fx_addsy: *mut symbolS,
    pub fx_subsy: *mut symbolS,
    pub fx_offset: valueT,
    pub fx_dot_value: addressT,
    pub fx_dot_frag: *mut fragS,
    pub fx_addnumber: valueT,
    pub fx_file: *const libc::c_char,
    pub fx_line: libc::c_uint,
}
pub type fragS = frag;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct frag {
    pub fr_address: addressT,
    pub last_fr_address: addressT,
    pub fr_fix: valueT,
    pub fr_var: offsetT,
    pub fr_offset: offsetT,
    pub fr_symbol: *mut symbolS,
    pub fr_opcode: *mut libc::c_char,
    pub fr_next: *mut frag,
    pub fr_file: *const libc::c_char,
    pub fr_line: libc::c_uint,
    pub line: *mut list_info_struct,
    #[bitfield(name = "region", ty = "libc::c_uint", bits = "0..=15")]
    #[bitfield(name = "relax_marker", ty = "libc::c_uint", bits = "16..=16")]
    #[bitfield(name = "has_code", ty = "libc::c_uint", bits = "17..=17")]
    #[bitfield(name = "insn_addr", ty = "libc::c_uint", bits = "18..=23")]
    pub region_relax_marker_has_code_insn_addr: [u8; 3],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
    pub fr_type: relax_stateT,
    pub fr_subtype: relax_substateT,
    pub tc_frag_data: i386_tc_frag_data,
    pub fr_literal: [libc::c_char; 1],
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct i386_tc_frag_data {
    pub u: C2RustUnnamed_25,
    pub padding_address: addressT,
    pub isa: processor_type,
    pub isa_flags: i386_cpu_flags,
    pub tune: processor_type,
    pub max_bytes: libc::c_uint,
    pub length: libc::c_uchar,
    pub last_length: libc::c_uchar,
    pub max_prefix_length: libc::c_uchar,
    pub prefix_length: libc::c_uchar,
    pub default_prefix: libc::c_uchar,
    pub cmp_size: libc::c_uchar,
    #[bitfield(name = "mf_type", ty = "libc::c_uint", bits = "0..=2")]
    #[bitfield(name = "classified", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "branch_type", ty = "libc::c_uint", bits = "4..=6")]
    pub mf_type_classified_branch_type: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 5],
}
pub type processor_type = libc::c_uint;
pub const PROCESSOR_BT: processor_type = 21;
pub const PROCESSOR_ZNVER: processor_type = 20;
pub const PROCESSOR_BD: processor_type = 19;
pub const PROCESSOR_AMDFAM10: processor_type = 18;
pub const PROCESSOR_GENERIC64: processor_type = 17;
pub const PROCESSOR_GENERIC32: processor_type = 16;
pub const PROCESSOR_K8: processor_type = 15;
pub const PROCESSOR_ATHLON: processor_type = 14;
pub const PROCESSOR_K6: processor_type = 13;
pub const PROCESSOR_IAMCU: processor_type = 12;
pub const PROCESSOR_K1OM: processor_type = 11;
pub const PROCESSOR_L1OM: processor_type = 10;
pub const PROCESSOR_COREI7: processor_type = 9;
pub const PROCESSOR_CORE2: processor_type = 8;
pub const PROCESSOR_CORE: processor_type = 7;
pub const PROCESSOR_NOCONA: processor_type = 6;
pub const PROCESSOR_PENTIUM4: processor_type = 5;
pub const PROCESSOR_PENTIUMPRO: processor_type = 4;
pub const PROCESSOR_PENTIUM: processor_type = 3;
pub const PROCESSOR_I486: processor_type = 2;
pub const PROCESSOR_I386: processor_type = 1;
pub const PROCESSOR_UNKNOWN: processor_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union i386_cpu_flags {
    pub bitfield: C2RustUnnamed_24,
    pub array: [libc::c_uint; 4],
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2RustUnnamed_24 {
    #[bitfield(name = "cpui186", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "cpui286", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "cpui386", ty = "libc::c_uint", bits = "2..=2")]
    #[bitfield(name = "cpui486", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "cpui586", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "cpui686", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "cpucmov", ty = "libc::c_uint", bits = "6..=6")]
    #[bitfield(name = "cpufxsr", ty = "libc::c_uint", bits = "7..=7")]
    #[bitfield(name = "cpuclflush", ty = "libc::c_uint", bits = "8..=8")]
    #[bitfield(name = "cpunop", ty = "libc::c_uint", bits = "9..=9")]
    #[bitfield(name = "cpusyscall", ty = "libc::c_uint", bits = "10..=10")]
    #[bitfield(name = "cpu8087", ty = "libc::c_uint", bits = "11..=11")]
    #[bitfield(name = "cpu287", ty = "libc::c_uint", bits = "12..=12")]
    #[bitfield(name = "cpu387", ty = "libc::c_uint", bits = "13..=13")]
    #[bitfield(name = "cpu687", ty = "libc::c_uint", bits = "14..=14")]
    #[bitfield(name = "cpufisttp", ty = "libc::c_uint", bits = "15..=15")]
    #[bitfield(name = "cpummx", ty = "libc::c_uint", bits = "16..=16")]
    #[bitfield(name = "cpusse", ty = "libc::c_uint", bits = "17..=17")]
    #[bitfield(name = "cpusse2", ty = "libc::c_uint", bits = "18..=18")]
    #[bitfield(name = "cpua3dnow", ty = "libc::c_uint", bits = "19..=19")]
    #[bitfield(name = "cpua3dnowa", ty = "libc::c_uint", bits = "20..=20")]
    #[bitfield(name = "cpusse3", ty = "libc::c_uint", bits = "21..=21")]
    #[bitfield(name = "cpupadlock", ty = "libc::c_uint", bits = "22..=22")]
    #[bitfield(name = "cpusvme", ty = "libc::c_uint", bits = "23..=23")]
    #[bitfield(name = "cpuvmx", ty = "libc::c_uint", bits = "24..=24")]
    #[bitfield(name = "cpusmx", ty = "libc::c_uint", bits = "25..=25")]
    #[bitfield(name = "cpussse3", ty = "libc::c_uint", bits = "26..=26")]
    #[bitfield(name = "cpusse4a", ty = "libc::c_uint", bits = "27..=27")]
    #[bitfield(name = "cpulzcnt", ty = "libc::c_uint", bits = "28..=28")]
    #[bitfield(name = "cpupopcnt", ty = "libc::c_uint", bits = "29..=29")]
    #[bitfield(name = "cpusse4_1", ty = "libc::c_uint", bits = "30..=30")]
    #[bitfield(name = "cpusse4_2", ty = "libc::c_uint", bits = "31..=31")]
    #[bitfield(name = "cpuavx", ty = "libc::c_uint", bits = "32..=32")]
    #[bitfield(name = "cpuavx2", ty = "libc::c_uint", bits = "33..=33")]
    #[bitfield(name = "cpuavx512f", ty = "libc::c_uint", bits = "34..=34")]
    #[bitfield(name = "cpuavx512cd", ty = "libc::c_uint", bits = "35..=35")]
    #[bitfield(name = "cpuavx512er", ty = "libc::c_uint", bits = "36..=36")]
    #[bitfield(name = "cpuavx512pf", ty = "libc::c_uint", bits = "37..=37")]
    #[bitfield(name = "cpuavx512vl", ty = "libc::c_uint", bits = "38..=38")]
    #[bitfield(name = "cpuavx512dq", ty = "libc::c_uint", bits = "39..=39")]
    #[bitfield(name = "cpuavx512bw", ty = "libc::c_uint", bits = "40..=40")]
    #[bitfield(name = "cpul1om", ty = "libc::c_uint", bits = "41..=41")]
    #[bitfield(name = "cpuk1om", ty = "libc::c_uint", bits = "42..=42")]
    #[bitfield(name = "cpuiamcu", ty = "libc::c_uint", bits = "43..=43")]
    #[bitfield(name = "cpuxsave", ty = "libc::c_uint", bits = "44..=44")]
    #[bitfield(name = "cpuxsaveopt", ty = "libc::c_uint", bits = "45..=45")]
    #[bitfield(name = "cpuaes", ty = "libc::c_uint", bits = "46..=46")]
    #[bitfield(name = "cpupclmul", ty = "libc::c_uint", bits = "47..=47")]
    #[bitfield(name = "cpufma", ty = "libc::c_uint", bits = "48..=48")]
    #[bitfield(name = "cpufma4", ty = "libc::c_uint", bits = "49..=49")]
    #[bitfield(name = "cpuxop", ty = "libc::c_uint", bits = "50..=50")]
    #[bitfield(name = "cpulwp", ty = "libc::c_uint", bits = "51..=51")]
    #[bitfield(name = "cpubmi", ty = "libc::c_uint", bits = "52..=52")]
    #[bitfield(name = "cputbm", ty = "libc::c_uint", bits = "53..=53")]
    #[bitfield(name = "cpumovbe", ty = "libc::c_uint", bits = "54..=54")]
    #[bitfield(name = "cpucx16", ty = "libc::c_uint", bits = "55..=55")]
    #[bitfield(name = "cpuept", ty = "libc::c_uint", bits = "56..=56")]
    #[bitfield(name = "cpurdtscp", ty = "libc::c_uint", bits = "57..=57")]
    #[bitfield(name = "cpufsgsbase", ty = "libc::c_uint", bits = "58..=58")]
    #[bitfield(name = "cpurdrnd", ty = "libc::c_uint", bits = "59..=59")]
    #[bitfield(name = "cpuf16c", ty = "libc::c_uint", bits = "60..=60")]
    #[bitfield(name = "cpubmi2", ty = "libc::c_uint", bits = "61..=61")]
    #[bitfield(name = "cpuhle", ty = "libc::c_uint", bits = "62..=62")]
    #[bitfield(name = "cpurtm", ty = "libc::c_uint", bits = "63..=63")]
    #[bitfield(name = "cpuinvpcid", ty = "libc::c_uint", bits = "64..=64")]
    #[bitfield(name = "cpuvmfunc", ty = "libc::c_uint", bits = "65..=65")]
    #[bitfield(name = "cpumpx", ty = "libc::c_uint", bits = "66..=66")]
    #[bitfield(name = "cpulm", ty = "libc::c_uint", bits = "67..=67")]
    #[bitfield(name = "cpurdseed", ty = "libc::c_uint", bits = "68..=68")]
    #[bitfield(name = "cpuadx", ty = "libc::c_uint", bits = "69..=69")]
    #[bitfield(name = "cpuprfchw", ty = "libc::c_uint", bits = "70..=70")]
    #[bitfield(name = "cpusmap", ty = "libc::c_uint", bits = "71..=71")]
    #[bitfield(name = "cpusha", ty = "libc::c_uint", bits = "72..=72")]
    #[bitfield(name = "cpuclflushopt", ty = "libc::c_uint", bits = "73..=73")]
    #[bitfield(name = "cpuxsaves", ty = "libc::c_uint", bits = "74..=74")]
    #[bitfield(name = "cpuxsavec", ty = "libc::c_uint", bits = "75..=75")]
    #[bitfield(name = "cpuprefetchwt1", ty = "libc::c_uint", bits = "76..=76")]
    #[bitfield(name = "cpuse1", ty = "libc::c_uint", bits = "77..=77")]
    #[bitfield(name = "cpuclwb", ty = "libc::c_uint", bits = "78..=78")]
    #[bitfield(name = "cpuavx512ifma", ty = "libc::c_uint", bits = "79..=79")]
    #[bitfield(name = "cpuavx512vbmi", ty = "libc::c_uint", bits = "80..=80")]
    #[bitfield(name = "cpuavx512_4fmaps", ty = "libc::c_uint", bits = "81..=81")]
    #[bitfield(name = "cpuavx512_4vnniw", ty = "libc::c_uint", bits = "82..=82")]
    #[bitfield(name = "cpuavx512_vpopcntdq", ty = "libc::c_uint", bits = "83..=83")]
    #[bitfield(name = "cpuavx512_vbmi2", ty = "libc::c_uint", bits = "84..=84")]
    #[bitfield(name = "cpuavx512_vnni", ty = "libc::c_uint", bits = "85..=85")]
    #[bitfield(name = "cpuavx512_bitalg", ty = "libc::c_uint", bits = "86..=86")]
    #[bitfield(name = "cpuavx512_bf16", ty = "libc::c_uint", bits = "87..=87")]
    #[bitfield(name = "cpuavx512_vp2intersect", ty = "libc::c_uint", bits = "88..=88")]
    #[bitfield(name = "cputdx", ty = "libc::c_uint", bits = "89..=89")]
    #[bitfield(name = "cpuavx_vnni", ty = "libc::c_uint", bits = "90..=90")]
    #[bitfield(name = "cpumwaitx", ty = "libc::c_uint", bits = "91..=91")]
    #[bitfield(name = "cpuclzero", ty = "libc::c_uint", bits = "92..=92")]
    #[bitfield(name = "cpuospke", ty = "libc::c_uint", bits = "93..=93")]
    #[bitfield(name = "cpurdpid", ty = "libc::c_uint", bits = "94..=94")]
    #[bitfield(name = "cpuptwrite", ty = "libc::c_uint", bits = "95..=95")]
    #[bitfield(name = "cpuibt", ty = "libc::c_uint", bits = "96..=96")]
    #[bitfield(name = "cpushstk", ty = "libc::c_uint", bits = "97..=97")]
    #[bitfield(name = "cpuamx_int8", ty = "libc::c_uint", bits = "98..=98")]
    #[bitfield(name = "cpuamx_bf16", ty = "libc::c_uint", bits = "99..=99")]
    #[bitfield(name = "cpuamx_tile", ty = "libc::c_uint", bits = "100..=100")]
    #[bitfield(name = "cpugfni", ty = "libc::c_uint", bits = "101..=101")]
    #[bitfield(name = "cpuvaes", ty = "libc::c_uint", bits = "102..=102")]
    #[bitfield(name = "cpuvpclmulqdq", ty = "libc::c_uint", bits = "103..=103")]
    #[bitfield(name = "cpuwbnoinvd", ty = "libc::c_uint", bits = "104..=104")]
    #[bitfield(name = "cpupconfig", ty = "libc::c_uint", bits = "105..=105")]
    #[bitfield(name = "cpuwaitpkg", ty = "libc::c_uint", bits = "106..=106")]
    #[bitfield(name = "cpuuintr", ty = "libc::c_uint", bits = "107..=107")]
    #[bitfield(name = "cpucldemote", ty = "libc::c_uint", bits = "108..=108")]
    #[bitfield(name = "cpumovdiri", ty = "libc::c_uint", bits = "109..=109")]
    #[bitfield(name = "cpumovdir64b", ty = "libc::c_uint", bits = "110..=110")]
    #[bitfield(name = "cpuenqcmd", ty = "libc::c_uint", bits = "111..=111")]
    #[bitfield(name = "cpuserialize", ty = "libc::c_uint", bits = "112..=112")]
    #[bitfield(name = "cpurdpru", ty = "libc::c_uint", bits = "113..=113")]
    #[bitfield(name = "cpumcommit", ty = "libc::c_uint", bits = "114..=114")]
    #[bitfield(name = "cpusev_es", ty = "libc::c_uint", bits = "115..=115")]
    #[bitfield(name = "cputsxldtrk", ty = "libc::c_uint", bits = "116..=116")]
    #[bitfield(name = "cpukl", ty = "libc::c_uint", bits = "117..=117")]
    #[bitfield(name = "cpuwidekl", ty = "libc::c_uint", bits = "118..=118")]
    #[bitfield(name = "cpuhreset", ty = "libc::c_uint", bits = "119..=119")]
    #[bitfield(name = "cpuinvlpgb", ty = "libc::c_uint", bits = "120..=120")]
    #[bitfield(name = "cputlbsync", ty = "libc::c_uint", bits = "121..=121")]
    #[bitfield(name = "cpusnp", ty = "libc::c_uint", bits = "122..=122")]
    #[bitfield(name = "cpu64", ty = "libc::c_uint", bits = "123..=123")]
    #[bitfield(name = "cpuno64", ty = "libc::c_uint", bits = "124..=124")]
    #[bitfield(name = "unused", ty = "libc::c_uint", bits = "125..=127")]
    pub cpui186_cpui286_cpui386_cpui486_cpui586_cpui686_cpucmov_cpufxsr_cpuclflush_cpunop_cpusyscall_cpu8087_cpu287_cpu387_cpu687_cpufisttp_cpummx_cpusse_cpusse2_cpua3dnow_cpua3dnowa_cpusse3_cpupadlock_cpusvme_cpuvmx_cpusmx_cpussse3_cpusse4a_cpulzcnt_cpupopcnt_cpusse4_1_cpusse4_2_cpuavx_cpuavx2_cpuavx512f_cpuavx512cd_cpuavx512er_cpuavx512pf_cpuavx512vl_cpuavx512dq_cpuavx512bw_cpul1om_cpuk1om_cpuiamcu_cpuxsave_cpuxsaveopt_cpuaes_cpupclmul_cpufma_cpufma4_cpuxop_cpulwp_cpubmi_cputbm_cpumovbe_cpucx16_cpuept_cpurdtscp_cpufsgsbase_cpurdrnd_cpuf16c_cpubmi2_cpuhle_cpurtm_cpuinvpcid_cpuvmfunc_cpumpx_cpulm_cpurdseed_cpuadx_cpuprfchw_cpusmap_cpusha_cpuclflushopt_cpuxsaves_cpuxsavec_cpuprefetchwt1_cpuse1_cpuclwb_cpuavx512ifma_cpuavx512vbmi_cpuavx512_4fmaps_cpuavx512_4vnniw_cpuavx512_vpopcntdq_cpuavx512_vbmi2_cpuavx512_vnni_cpuavx512_bitalg_cpuavx512_bf16_cpuavx512_vp2intersect_cputdx_cpuavx_vnni_cpumwaitx_cpuclzero_cpuospke_cpurdpid_cpuptwrite_cpuibt_cpushstk_cpuamx_int8_cpuamx_bf16_cpuamx_tile_cpugfni_cpuvaes_cpuvpclmulqdq_cpuwbnoinvd_cpupconfig_cpuwaitpkg_cpuuintr_cpucldemote_cpumovdiri_cpumovdir64b_cpuenqcmd_cpuserialize_cpurdpru_cpumcommit_cpusev_es_cputsxldtrk_cpukl_cpuwidekl_cpuhreset_cpuinvlpgb_cputlbsync_cpusnp_cpu64_cpuno64_unused: [u8; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_25 {
    pub padding_fragP: *mut fragS,
    pub branch_fragP: *mut fragS,
}
pub type expr_mode = libc::c_uint;
pub const expr_defer: expr_mode = 2;
pub const expr_normal: expr_mode = 1;
pub const expr_evaluate: expr_mode = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dwarf2_line_info {
    pub filenum: libc::c_uint,
    pub line: libc::c_uint,
    pub column: libc::c_uint,
    pub isa: libc::c_uint,
    pub flags: libc::c_uint,
    pub discriminator: libc::c_uint,
    pub view: *mut symbolS,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct file_entry {
    pub filename: *const libc::c_char,
    pub dir: libc::c_uint,
    pub md5: [libc::c_uchar; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct line_entry {
    pub next: *mut line_entry,
    pub label: *mut symbolS,
    pub loc: dwarf2_line_info,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct line_subseg {
    pub next: *mut line_subseg,
    pub subseg: subsegT,
    pub head: *mut line_entry,
    pub ptail: *mut *mut line_entry,
    pub pmove_tail: *mut *mut line_entry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct line_seg {
    pub next: *mut line_seg,
    pub seg: segT,
    pub head: *mut line_subseg,
    pub text_start: *mut symbolS,
    pub text_end: *mut symbolS,
}
pub type segment_info_type = segment_info_struct;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct segment_info_struct {
    pub frchainP: *mut frchainS,
    #[bitfield(name = "hadone", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "bss", ty = "libc::c_uint", bits = "1..=1")]
    pub hadone_bss: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub user_stuff: libc::c_int,
    pub fix_root: *mut fixS,
    pub fix_tail: *mut fixS,
    pub dot: *mut symbolS,
    pub lineno_list_head: *mut lineno_list,
    pub lineno_list_tail: *mut lineno_list,
    pub bfd_section: *mut asection,
    pub sym: *mut symbolS,
    pub dwarf2_line_seg: *mut libc::c_void,
    pub stabu: C2RustUnnamed_26,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_26 {
    pub stab_string_size: libc::c_ulong,
    pub p: *mut libc::c_char,
}
pub type fixS = fix;
pub type frchainS = frchain;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct frchain {
    pub frch_root: *mut frag,
    pub frch_last: *mut frag,
    pub frch_next: *mut frchain,
    pub frch_subseg: subsegT,
    pub fix_root: *mut fixS,
    pub fix_tail: *mut fixS,
    pub frch_obstack: obstack,
    pub frch_frag_now: *mut fragS,
    pub frch_cfi_data: *mut frch_cfi_data,
}
pub const _sch_isdigit: C2RustUnnamed_27 = 4;
pub const _sch_isalpha: C2RustUnnamed_27 = 136;
pub const DW_LANG_Mips_Assembler: dwarf_source_language = 32769;
pub const dwarf2_format_32bit: dwarf2_format = 0;
pub const DW_UT_compile: dwarf_unit_type = 1;
pub const dwarf2_format_64bit_irix: dwarf2_format = 2;
pub const dwarf2_format_64bit: dwarf2_format = 1;
pub const DW_FORM_data2: dwarf_form = 5;
pub const DW_AT_language: dwarf_attribute = 19;
pub const DW_FORM_strp: dwarf_form = 14;
pub const DW_AT_producer: dwarf_attribute = 37;
pub const DW_AT_comp_dir: dwarf_attribute = 27;
pub const DW_AT_name: dwarf_attribute = 3;
pub const DW_AT_ranges: dwarf_attribute = 85;
pub const DW_FORM_udata: dwarf_form = 15;
pub const DW_AT_high_pc: dwarf_attribute = 18;
pub const DW_FORM_addr: dwarf_form = 1;
pub const DW_AT_low_pc: dwarf_attribute = 17;
pub const DW_AT_stmt_list: dwarf_attribute = 16;
pub const DW_FORM_sec_offset: dwarf_form = 23;
pub const DW_FORM_data8: dwarf_form = 7;
pub const DW_FORM_data4: dwarf_form = 6;
pub const DW_TAG_compile_unit: dwarf_tag = 17;
pub const DW_RLE_end_of_list: dwarf_range_list_entry = 0;
pub const DW_RLE_start_length: dwarf_range_list_entry = 7;
pub const DW_LNS_copy: dwarf_line_number_ops = 1;
pub const DW_LNS_advance_pc: dwarf_line_number_ops = 2;
pub const DW_LNS_const_add_pc: dwarf_line_number_ops = 8;
pub const DW_LNS_advance_line: dwarf_line_number_ops = 3;
pub const DW_LNE_end_sequence: dwarf_line_number_x_ops = 1;
pub const DW_LNS_extended_op: dwarf_line_number_ops = 0;
pub const DW_LNE_set_address: dwarf_line_number_x_ops = 2;
pub const DW_LNS_set_epilogue_begin: dwarf_line_number_ops = 11;
pub const DW_LNS_set_prologue_end: dwarf_line_number_ops = 10;
pub const DW_LNS_set_basic_block: dwarf_line_number_ops = 7;
pub const DW_LNS_negate_stmt: dwarf_line_number_ops = 6;
pub const DW_LNS_set_isa: dwarf_line_number_ops = 12;
pub const DW_LNE_set_discriminator: dwarf_line_number_x_ops = 4;
pub const DW_LNS_set_column: dwarf_line_number_ops = 5;
pub const DW_LNS_set_file: dwarf_line_number_ops = 4;
pub const DW_FORM_data16: dwarf_form = 30;
pub const DW_LNCT_MD5: dwarf_line_number_content_type = 5;
pub const DW_LNCT_size: dwarf_line_number_content_type = 4;
pub const DW_LNCT_timestamp: dwarf_line_number_content_type = 3;
pub const DW_LNCT_directory_index: dwarf_line_number_content_type = 2;
pub const DW_FORM_line_strp: dwarf_form = 31;
pub const DW_LNCT_path: dwarf_line_number_content_type = 1;
pub const DW_LNS_fixed_advance_pc: dwarf_line_number_ops = 9;
pub type dwarf2_format = libc::c_uint;
pub type C2RustUnnamed_27 = libc::c_uint;
pub const _sch_isbasic: C2RustUnnamed_27 = 3088;
pub const _sch_iscppsp: C2RustUnnamed_27 = 3072;
pub const _sch_isgraph: C2RustUnnamed_27 = 172;
pub const _sch_isidnum: C2RustUnnamed_27 = 516;
pub const _sch_isalnum: C2RustUnnamed_27 = 140;
pub const _sch_isnvsp: C2RustUnnamed_27 = 2048;
pub const _sch_isvsp: C2RustUnnamed_27 = 1024;
pub const _sch_isidst: C2RustUnnamed_27 = 512;
pub const _sch_isxdigit: C2RustUnnamed_27 = 256;
pub const _sch_isupper: C2RustUnnamed_27 = 128;
pub const _sch_isspace: C2RustUnnamed_27 = 64;
pub const _sch_ispunct: C2RustUnnamed_27 = 32;
pub const _sch_isprint: C2RustUnnamed_27 = 16;
pub const _sch_islower: C2RustUnnamed_27 = 8;
pub const _sch_iscntrl: C2RustUnnamed_27 = 2;
pub const _sch_isblank: C2RustUnnamed_27 = 1;
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
pub const DW_FORM_strp_sup: dwarf_form = 29;
pub const DW_FORM_ref_sup4: dwarf_form = 28;
pub const DW_FORM_addrx: dwarf_form = 27;
pub const DW_FORM_strx: dwarf_form = 26;
pub const DW_FORM_ref_sig8: dwarf_form = 32;
pub const DW_FORM_flag_present: dwarf_form = 25;
pub const DW_FORM_exprloc: dwarf_form = 24;
pub const DW_FORM_indirect: dwarf_form = 22;
pub const DW_FORM_ref_udata: dwarf_form = 21;
pub const DW_FORM_ref8: dwarf_form = 20;
pub const DW_FORM_ref4: dwarf_form = 19;
pub const DW_FORM_ref2: dwarf_form = 18;
pub const DW_FORM_ref1: dwarf_form = 17;
pub const DW_FORM_ref_addr: dwarf_form = 16;
pub const DW_FORM_sdata: dwarf_form = 13;
pub const DW_FORM_flag: dwarf_form = 12;
pub const DW_FORM_data1: dwarf_form = 11;
pub const DW_FORM_block1: dwarf_form = 10;
pub const DW_FORM_block: dwarf_form = 9;
pub const DW_FORM_string: dwarf_form = 8;
pub const DW_FORM_block4: dwarf_form = 4;
pub const DW_FORM_block2: dwarf_form = 3;
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
pub const DW_AT_lower_bound: dwarf_attribute = 34;
pub const DW_AT_is_optional: dwarf_attribute = 33;
pub const DW_AT_inline: dwarf_attribute = 32;
pub const DW_AT_default_value: dwarf_attribute = 30;
pub const DW_AT_containing_type: dwarf_attribute = 29;
pub const DW_AT_const_value: dwarf_attribute = 28;
pub const DW_AT_common_reference: dwarf_attribute = 26;
pub const DW_AT_string_length: dwarf_attribute = 25;
pub const DW_AT_import: dwarf_attribute = 24;
pub const DW_AT_visibility: dwarf_attribute = 23;
pub const DW_AT_discr_value: dwarf_attribute = 22;
pub const DW_AT_discr: dwarf_attribute = 21;
pub const DW_AT_member: dwarf_attribute = 20;
pub const DW_AT_element_list: dwarf_attribute = 15;
pub const DW_AT_bit_size: dwarf_attribute = 13;
pub const DW_AT_bit_offset: dwarf_attribute = 12;
pub const DW_AT_byte_size: dwarf_attribute = 11;
pub const DW_AT_subscr_data: dwarf_attribute = 10;
pub const DW_AT_ordering: dwarf_attribute = 9;
pub const DW_AT_location: dwarf_attribute = 2;
pub const DW_AT_sibling: dwarf_attribute = 1;
pub type dwarf_unit_type = libc::c_uint;
pub const DW_UT_hi_user: dwarf_unit_type = 255;
pub const DW_UT_lo_user: dwarf_unit_type = 128;
pub const DW_UT_split_type: dwarf_unit_type = 6;
pub const DW_UT_split_compile: dwarf_unit_type = 5;
pub const DW_UT_skeleton: dwarf_unit_type = 4;
pub const DW_UT_partial: dwarf_unit_type = 3;
pub const DW_UT_type: dwarf_unit_type = 2;
pub type dwarf_line_number_ops = libc::c_uint;
pub type dwarf_line_number_x_ops = libc::c_uint;
pub const DW_LNE_hi_user: dwarf_line_number_x_ops = 255;
pub const DW_LNE_lo_user: dwarf_line_number_x_ops = 128;
pub const DW_LNE_HP_source_file_correlation: dwarf_line_number_x_ops = 128;
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
pub const DW_LNE_define_file: dwarf_line_number_x_ops = 3;
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
pub const DW_LANG_hi_user: dwarf_source_language = 65535;
pub const DW_LANG_lo_user: dwarf_source_language = 32768;
pub const DW_LANG_RenderScript: dwarf_source_language = 36;
pub const DW_LANG_Fortran08: dwarf_source_language = 35;
pub const DW_LANG_Fortran03: dwarf_source_language = 34;
pub const DW_LANG_C_plus_plus_14: dwarf_source_language = 33;
pub const DW_LANG_Dylan: dwarf_source_language = 32;
pub const DW_LANG_Julia: dwarf_source_language = 31;
pub const DW_LANG_Swift: dwarf_source_language = 30;
pub const DW_LANG_C11: dwarf_source_language = 29;
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
pub const DW_LANG_UPC: dwarf_source_language = 18;
pub const DW_LANG_ObjC_plus_plus: dwarf_source_language = 17;
pub const DW_LANG_ObjC: dwarf_source_language = 16;
pub const DW_LANG_PLI: dwarf_source_language = 15;
pub const DW_LANG_Fortran95: dwarf_source_language = 14;
pub const DW_LANG_Ada95: dwarf_source_language = 13;
pub const DW_LANG_C99: dwarf_source_language = 12;
pub const DW_LANG_Java: dwarf_source_language = 11;
pub const DW_LANG_Modula2: dwarf_source_language = 10;
pub const DW_LANG_Pascal83: dwarf_source_language = 9;
pub const DW_LANG_Fortran90: dwarf_source_language = 8;
pub const DW_LANG_Fortran77: dwarf_source_language = 7;
pub const DW_LANG_Cobol85: dwarf_source_language = 6;
pub const DW_LANG_Cobol74: dwarf_source_language = 5;
pub const DW_LANG_C_plus_plus: dwarf_source_language = 4;
pub const DW_LANG_Ada83: dwarf_source_language = 3;
pub const DW_LANG_C: dwarf_source_language = 2;
pub const DW_LANG_C89: dwarf_source_language = 1;
pub type dwarf_range_list_entry = libc::c_uint;
pub const DW_RLE_start_end: dwarf_range_list_entry = 6;
pub const DW_RLE_base_address: dwarf_range_list_entry = 5;
pub const DW_RLE_offset_pair: dwarf_range_list_entry = 4;
pub const DW_RLE_startx_length: dwarf_range_list_entry = 3;
pub const DW_RLE_startx_endx: dwarf_range_list_entry = 2;
pub const DW_RLE_base_addressx: dwarf_range_list_entry = 1;
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
unsafe extern "C" fn bfd_section_flags(mut sec: *const asection) -> flagword {
    return (*sec).flags;
}
#[inline]
unsafe extern "C" fn bfd_section_userdata(
    mut sec: *const asection,
) -> *mut libc::c_void {
    return (*sec).userdata;
}
#[inline]
unsafe extern "C" fn xmemdup0(
    mut in_0: *const libc::c_char,
    mut len: size_t,
) -> *mut libc::c_char {
    let mut out: *mut libc::c_char = xmalloc(
        len.wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    *out.offset(len as isize) = 0 as libc::c_int as libc::c_char;
    return memcpy(out as *mut libc::c_void, in_0 as *const libc::c_void, len)
        as *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn dwarf2_directive_file(mut dummy: libc::c_int) {
    dwarf2_directive_filename();
}
#[no_mangle]
pub unsafe extern "C" fn dwarf2_directive_filename() -> *mut libc::c_char {
    let mut with_md5: bool = 0 as libc::c_int != 0;
    let mut num: valueT = 0;
    let mut filename: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dirname: *const libc::c_char = 0 as *const libc::c_char;
    let mut filename_len: libc::c_int = 0;
    let mut i: libc::c_uint = 0;
    if *input_line_pointer as libc::c_int == ' ' as i32 {
        input_line_pointer = input_line_pointer.offset(1);
        input_line_pointer;
    } else {};
    if *input_line_pointer as libc::c_int == '"' as i32 {
        s_app_file(0 as libc::c_int);
        return 0 as *mut libc::c_char;
    }
    num = get_absolute_expression() as valueT;
    if (num as offsetT) < 1 as libc::c_int as libc::c_long {
        if num == 0 as libc::c_int as libc::c_ulong
            && dwarf_level < 5 as libc::c_int as libc::c_uint
        {
            dwarf_level = 5 as libc::c_int as libc::c_uint;
        }
        if (num as offsetT) < 0 as libc::c_int as libc::c_long
            || (if dwarf_level > 3 as libc::c_int as libc::c_uint {
                dwarf_level
            } else {
                3 as libc::c_int as libc::c_uint
            }) < 5 as libc::c_int as libc::c_uint
        {
            as_bad(
                dcgettext(
                    0 as *const libc::c_char,
                    b"file number less than one\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            ignore_rest_of_line();
            return 0 as *mut libc::c_char;
        }
    }
    filename = demand_copy_C_string(&mut filename_len);
    if filename.is_null() {
        return 0 as *mut libc::c_char;
    }
    if (if dwarf_level > 3 as libc::c_int as libc::c_uint {
        dwarf_level
    } else {
        3 as libc::c_int as libc::c_uint
    }) > 4 as libc::c_int as libc::c_uint
    {
        if *input_line_pointer as libc::c_int == ' ' as i32 {
            input_line_pointer = input_line_pointer.offset(1);
            input_line_pointer;
        } else {};
        if *input_line_pointer as libc::c_int == '"' as i32 {
            dirname = filename;
            filename = demand_copy_C_string(&mut filename_len);
            if *input_line_pointer as libc::c_int == ' ' as i32 {
                input_line_pointer = input_line_pointer.offset(1);
                input_line_pointer;
            } else {};
        }
        if startswith(input_line_pointer, b"md5\0" as *const u8 as *const libc::c_char) {
            input_line_pointer = input_line_pointer.offset(3 as libc::c_int as isize);
            if *input_line_pointer as libc::c_int == ' ' as i32 {
                input_line_pointer = input_line_pointer.offset(1);
                input_line_pointer;
            } else {};
            let mut exp: expressionS = expressionS {
                X_add_symbol: 0 as *mut symbolS,
                X_op_symbol: 0 as *mut symbolS,
                X_add_number: 0,
                X_op_X_unsigned_X_extrabit: [0; 2],
                X_md: 0,
            };
            expr(0 as libc::c_int, &mut exp, expr_evaluate);
            if exp.X_op() as libc::c_int != O_big as libc::c_int {
                as_bad(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"md5 value too small or not a constant\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
            } else {
                with_md5 = 1 as libc::c_int != 0;
            }
        }
    }
    demand_empty_rest_of_line();
    debug_type = DEBUG_NONE;
    if num != num as libc::c_uint as libc::c_ulong
        || num
            >= (-(1 as libc::c_int) as size_t)
                .wrapping_div(::core::mem::size_of::<file_entry>() as libc::c_ulong)
                .wrapping_sub(32 as libc::c_int as libc::c_ulong)
    {
        as_bad(
            dcgettext(
                0 as *const libc::c_char,
                b"file number %lu is too big\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            num,
        );
        return 0 as *mut libc::c_char;
    }
    if num_of_auto_assigned != 0 {
        if files_in_use
            != num_of_auto_assigned.wrapping_add(1 as libc::c_int as libc::c_uint)
        {
            as_abort(
                b"dwarf2dbg.c\0" as *const u8 as *const libc::c_char,
                1121 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 38],
                    &[libc::c_char; 38],
                >(b"char *dwarf2_directive_filename(void)\0"))
                    .as_ptr(),
            );
        }
        i = 1 as libc::c_int as libc::c_uint;
        while i < files_in_use {
            let ref mut fresh0 = (*files.offset(i as isize)).filename;
            *fresh0 = 0 as *const libc::c_char;
            i = i.wrapping_add(1);
            i;
        }
        files_in_use = 0 as libc::c_int as libc::c_uint;
        num_of_auto_assigned = 0 as libc::c_int as libc::c_uint;
    }
    if !allocate_filename_to_slot(dirname, filename, num as libc::c_uint, with_md5) {
        return 0 as *mut libc::c_char;
    }
    return filename;
}
#[no_mangle]
pub unsafe extern "C" fn dwarf2_directive_loc(mut dummy: libc::c_int) {
    let mut filenum: offsetT = 0;
    let mut line: offsetT = 0;
    if dwarf2_loc_directive_seen {
        dwarf2_emit_insn(0 as libc::c_int);
    }
    filenum = get_absolute_expression();
    if *input_line_pointer as libc::c_int == ' ' as i32 {
        input_line_pointer = input_line_pointer.offset(1);
        input_line_pointer;
    } else {};
    line = get_absolute_expression();
    if filenum < 1 as libc::c_int as libc::c_long {
        if filenum == 0 as libc::c_int as libc::c_long
            && dwarf_level < 5 as libc::c_int as libc::c_uint
        {
            dwarf_level = 5 as libc::c_int as libc::c_uint;
        }
        if filenum < 0 as libc::c_int as libc::c_long
            || (if dwarf_level > 3 as libc::c_int as libc::c_uint {
                dwarf_level
            } else {
                3 as libc::c_int as libc::c_uint
            }) < 5 as libc::c_int as libc::c_uint
        {
            as_bad(
                dcgettext(
                    0 as *const libc::c_char,
                    b"file number less than one\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            return;
        }
    }
    if filenum as valueT >= files_in_use as libc::c_ulong
        || ((*files.offset(filenum as isize)).filename).is_null()
    {
        as_bad(
            dcgettext(
                0 as *const libc::c_char,
                b"unassigned file number %ld\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            filenum,
        );
        return;
    }
    current.filenum = filenum as libc::c_uint;
    current.line = line as libc::c_uint;
    current.discriminator = 0 as libc::c_int as libc::c_uint;
    if listing != 0 {
        if (*files.offset(filenum as isize)).dir != 0 {
            let mut dir_len: size_t = strlen(
                *dirs.offset((*files.offset(filenum as isize)).dir as isize),
            );
            let mut file_len: size_t = strlen(
                (*files.offset(filenum as isize)).filename,
            );
            let mut cp: *mut libc::c_char = xmalloc(
                (::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
                    .wrapping_mul(
                        dir_len
                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                            .wrapping_add(file_len)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong),
                    ),
            ) as *mut libc::c_char;
            memcpy(
                cp as *mut libc::c_void,
                *dirs.offset((*files.offset(filenum as isize)).dir as isize)
                    as *const libc::c_void,
                dir_len,
            );
            *cp.offset(dir_len as isize) = '/' as i32 as libc::c_char;
            memcpy(
                cp.offset(dir_len as isize).offset(1 as libc::c_int as isize)
                    as *mut libc::c_void,
                (*files.offset(filenum as isize)).filename as *const libc::c_void,
                file_len,
            );
            *cp
                .offset(
                    dir_len
                        .wrapping_add(file_len)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = '\0' as i32 as libc::c_char;
            listing_source_file(cp);
            free(cp as *mut libc::c_void);
        } else {
            listing_source_file((*files.offset(filenum as isize)).filename);
        }
        listing_source_line(line as libc::c_uint);
    }
    if *input_line_pointer as libc::c_int == ' ' as i32 {
        input_line_pointer = input_line_pointer.offset(1);
        input_line_pointer;
    } else {};
    if _sch_istable[(*input_line_pointer as libc::c_int & 0xff as libc::c_int) as usize]
        as libc::c_int & _sch_isdigit as libc::c_int as libc::c_ushort as libc::c_int
        != 0
    {
        current.column = get_absolute_expression() as libc::c_uint;
        if *input_line_pointer as libc::c_int == ' ' as i32 {
            input_line_pointer = input_line_pointer.offset(1);
            input_line_pointer;
        } else {};
    }
    while _sch_istable[(*input_line_pointer as libc::c_int & 0xff as libc::c_int)
        as usize] as libc::c_int
        & _sch_isalpha as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut c: libc::c_char = 0;
        let mut value: offsetT = 0;
        c = get_symbol_name(&mut p);
        if strcmp(p, b"basic_block\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            current.flags |= ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uint;
            *input_line_pointer = c;
        } else if strcmp(p, b"prologue_end\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            current.flags |= ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uint;
            *input_line_pointer = c;
        } else if strcmp(p, b"epilogue_begin\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            current.flags |= ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_uint;
            *input_line_pointer = c;
        } else if strcmp(p, b"is_stmt\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            restore_line_pointer(c);
            value = get_absolute_expression();
            if value == 0 as libc::c_int as libc::c_long {
                current.flags
                    &= !((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint;
            } else if value == 1 as libc::c_int as libc::c_long {
                current.flags
                    |= ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint;
            } else {
                as_bad(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"is_stmt value not 0 or 1\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                return;
            }
        } else if strcmp(p, b"isa\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            restore_line_pointer(c);
            value = get_absolute_expression();
            if value >= 0 as libc::c_int as libc::c_long {
                current.isa = value as libc::c_uint;
            } else {
                as_bad(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"isa number less than zero\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                return;
            }
        } else if strcmp(p, b"discriminator\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            restore_line_pointer(c);
            value = get_absolute_expression();
            if value >= 0 as libc::c_int as libc::c_long {
                current.discriminator = value as libc::c_uint;
            } else {
                as_bad(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"discriminator less than zero\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                return;
            }
        } else if strcmp(p, b"view\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            let mut sym: *mut symbolS = 0 as *mut symbolS;
            restore_line_pointer(c);
            if *input_line_pointer as libc::c_int == ' ' as i32 {
                input_line_pointer = input_line_pointer.offset(1);
                input_line_pointer;
            } else {};
            if _sch_istable[(*input_line_pointer as libc::c_int & 0xff as libc::c_int)
                as usize] as libc::c_int
                & _sch_isdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
                || *input_line_pointer as libc::c_int == '-' as i32
            {
                let mut force_reset: bool = *input_line_pointer as libc::c_int
                    == '-' as i32;
                value = get_absolute_expression();
                if value != 0 as libc::c_int as libc::c_long {
                    as_bad(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"numeric view can only be asserted to zero\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    return;
                }
                if force_reset as libc::c_int != 0 && !force_reset_view.is_null() {
                    sym = force_reset_view;
                } else {
                    sym = symbol_temp_new(
                        &mut *_bfd_std_section
                            .as_mut_ptr()
                            .offset(2 as libc::c_int as isize),
                        &mut zero_address_frag,
                        value as valueT,
                    );
                    if force_reset {
                        force_reset_view = sym;
                    }
                }
            } else {
                let mut name: *mut libc::c_char = read_symbol_name();
                if name.is_null() {
                    return;
                }
                sym = symbol_find_or_make(name);
                if S_IS_DEFINED(sym) != 0 || symbol_equated_p(sym) != 0 {
                    if S_IS_VOLATILE(sym) != 0 {
                        sym = symbol_clone(sym, 1 as libc::c_int);
                    } else if S_CAN_BE_REDEFINED(sym) == 0 {
                        as_bad(
                            dcgettext(
                                0 as *const libc::c_char,
                                b"symbol `%s' is already defined\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            name,
                        );
                        return;
                    }
                }
                S_SET_SEGMENT(
                    sym,
                    &mut *_bfd_std_section.as_mut_ptr().offset(1 as libc::c_int as isize),
                );
                S_SET_VALUE(sym, 0 as libc::c_int as valueT);
                symbol_set_frag(sym, &mut zero_address_frag);
            }
            current.view = sym;
        } else {
            as_bad(
                dcgettext(
                    0 as *const libc::c_char,
                    b"unknown .loc sub-directive `%s'\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                p,
            );
            restore_line_pointer(c);
            return;
        }
        if *input_line_pointer as libc::c_int == '"' as i32 {
            input_line_pointer = input_line_pointer.offset(1);
            input_line_pointer;
        }
        if *input_line_pointer as libc::c_int == ' ' as i32 {
            input_line_pointer = input_line_pointer.offset(1);
            input_line_pointer;
        }
    }
    demand_empty_rest_of_line();
    dwarf2_loc_directive_seen = 1 as libc::c_int != 0;
    dwarf2_any_loc_directive_seen = dwarf2_loc_directive_seen;
    debug_type = DEBUG_NONE;
    if !(current.view).is_null() {
        dwarf2_emit_insn(0 as libc::c_int);
    }
}
#[no_mangle]
pub unsafe extern "C" fn dwarf2_emit_insn(mut size: libc::c_int) {
    let mut loc: dwarf2_line_info = dwarf2_line_info {
        filenum: 0,
        line: 0,
        column: 0,
        isa: 0,
        flags: 0,
        discriminator: 0,
        view: 0 as *mut symbolS,
    };
    if if debug_type as libc::c_uint != DEBUG_DWARF2 as libc::c_int as libc::c_uint {
        !dwarf2_loc_directive_seen as libc::c_int
    } else {
        (seen_at_least_1_file() == 0) as libc::c_int
    } != 0
    {
        return;
    }
    dwarf2_where(&mut loc);
    dwarf2_gen_line_info(
        (frag_now_fix_octets())
            .wrapping_sub(size as libc::c_ulong)
            .wrapping_div(((1 as libc::c_int) << 0 as libc::c_int) as libc::c_ulong),
        &mut loc,
    );
    dwarf2_consume_line_info();
}
#[no_mangle]
pub unsafe extern "C" fn dwarf2_consume_line_info() {
    dwarf2_loc_directive_seen = 0 as libc::c_int != 0;
    current.flags
        &= !((1 as libc::c_int) << 1 as libc::c_int
            | (1 as libc::c_int) << 2 as libc::c_int
            | (1 as libc::c_int) << 3 as libc::c_int) as libc::c_uint;
    current.discriminator = 0 as libc::c_int as libc::c_uint;
    current.view = 0 as *mut symbolS;
}
#[no_mangle]
pub static mut dwarf2_loc_directive_seen: bool = false;
#[no_mangle]
pub unsafe extern "C" fn dwarf2_gen_line_info(
    mut ofs: addressT,
    mut loc: *mut dwarf2_line_info,
) {
    static mut line: libc::c_uint = -(1 as libc::c_int) as libc::c_uint;
    static mut filenum: libc::c_uint = -(1 as libc::c_int) as libc::c_uint;
    let mut sym: *mut symbolS = 0 as *mut symbolS;
    if (*loc).line == 0 as libc::c_int as libc::c_uint {
        return;
    }
    if (*loc).filenum == 0 as libc::c_int as libc::c_uint {
        if dwarf_level < 5 as libc::c_int as libc::c_uint {
            dwarf_level = 5 as libc::c_int as libc::c_uint;
        }
        if (if dwarf_level > 3 as libc::c_int as libc::c_uint {
            dwarf_level
        } else {
            3 as libc::c_int as libc::c_uint
        }) < 5 as libc::c_int as libc::c_uint
        {
            return;
        }
    }
    if debug_type as libc::c_uint == DEBUG_DWARF2 as libc::c_int as libc::c_uint
        && line == (*loc).line && filenum == (*loc).filenum
    {
        return;
    }
    line = (*loc).line;
    filenum = (*loc).filenum;
    if linkrelax != 0 {
        let mut name: [libc::c_char; 120] = [0; 120];
        sprintf(
            name.as_mut_ptr(),
            b".Loc.%u.%u\0" as *const u8 as *const libc::c_char,
            line,
            filenum,
        );
        sym = symbol_new(name.as_mut_ptr(), now_seg, frag_now, ofs);
    } else {
        sym = symbol_temp_new(now_seg, frag_now, ofs);
    }
    dwarf2_gen_line_info_1(sym, loc);
}
#[no_mangle]
pub unsafe extern "C" fn dwarf2_where(mut line: *mut dwarf2_line_info) {
    if debug_type as libc::c_uint == DEBUG_DWARF2 as libc::c_int as libc::c_uint {
        let mut filename: *const libc::c_char = 0 as *const libc::c_char;
        memset(
            line as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<dwarf2_line_info>() as libc::c_ulong,
        );
        filename = as_where(&mut (*line).line);
        (*line).filenum = allocate_filenum(filename) as libc::c_uint;
        (*line).column = 0 as libc::c_int as libc::c_uint;
        (*line).flags = ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint;
        (*line).isa = current.isa;
        (*line).discriminator = current.discriminator;
        (*line).view = 0 as *mut symbolS;
    } else {
        *line = current;
    };
}
#[no_mangle]
pub unsafe extern "C" fn dwarf2_directive_loc_mark_labels(mut dummy: libc::c_int) {
    let mut value: offsetT = get_absolute_expression();
    if value != 0 as libc::c_int as libc::c_long
        && value != 1 as libc::c_int as libc::c_long
    {
        as_bad(
            dcgettext(
                0 as *const libc::c_char,
                b"expected 0 or 1\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        ignore_rest_of_line();
    } else {
        dwarf2_loc_mark_labels = value != 0 as libc::c_int as libc::c_long;
        demand_empty_rest_of_line();
    };
}
#[no_mangle]
pub static mut dwarf2_loc_mark_labels: bool = false;
#[no_mangle]
pub unsafe extern "C" fn dwarf2_set_isa(mut isa: libc::c_uint) {
    current.isa = isa;
}
#[no_mangle]
pub unsafe extern "C" fn dwarf2_move_insn(mut delta: libc::c_int) {
    let mut lss: *mut line_subseg = 0 as *mut line_subseg;
    let mut e: *mut line_entry = 0 as *mut line_entry;
    let mut now: valueT = 0;
    if delta == 0 as libc::c_int {
        return;
    }
    lss = get_line_subseg(now_seg, now_subseg, 0 as libc::c_int != 0);
    if lss.is_null() {
        return;
    }
    now = frag_now_fix();
    loop {
        e = *(*lss).pmove_tail;
        if e.is_null() {
            break;
        }
        if S_GET_VALUE((*e).label) == now {
            S_SET_VALUE((*e).label, now.wrapping_add(delta as libc::c_ulong));
        }
        (*lss).pmove_tail = &mut (*e).next;
    };
}
#[no_mangle]
pub unsafe extern "C" fn dwarf2_emit_label(mut label: *mut symbolS) {
    let mut loc: dwarf2_line_info = dwarf2_line_info {
        filenum: 0,
        line: 0,
        column: 0,
        isa: 0,
        flags: 0,
        discriminator: 0,
        view: 0 as *mut symbolS,
    };
    if !dwarf2_loc_mark_labels {
        return;
    }
    if S_GET_SEGMENT(label) != now_seg {
        return;
    }
    if bfd_section_flags(now_seg as *const asection)
        & 0x10 as libc::c_int as libc::c_uint == 0
    {
        return;
    }
    if files_in_use == 0 as libc::c_int as libc::c_uint
        && debug_type as libc::c_uint != DEBUG_DWARF2 as libc::c_int as libc::c_uint
    {
        return;
    }
    dwarf2_where(&mut loc);
    loc.flags |= ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uint;
    dwarf2_gen_line_info_1(label, &mut loc);
    dwarf2_consume_line_info();
}
#[no_mangle]
pub unsafe extern "C" fn dwarf2_init() {
    last_seg_ptr = &mut all_segs;
    if flag_dwarf_cie_version == -(1 as libc::c_int) {
        flag_dwarf_cie_version = 1 as libc::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn dwarf2_finish() {
    let mut line_seg: segT = 0 as *mut asection;
    let mut s: *mut line_seg = 0 as *mut line_seg;
    let mut info_seg: segT = 0 as *mut asection;
    let mut emit_other_sections: libc::c_int = 0 as libc::c_int;
    let mut empty_debug_line: libc::c_int = 0 as libc::c_int;
    info_seg = bfd_get_section_by_name(
        stdoutput,
        b".debug_info\0" as *const u8 as *const libc::c_char,
    );
    emit_other_sections = (info_seg.is_null() || seg_not_empty_p(info_seg) == 0)
        as libc::c_int;
    line_seg = bfd_get_section_by_name(
        stdoutput,
        b".debug_line\0" as *const u8 as *const libc::c_char,
    );
    empty_debug_line = (line_seg.is_null() || seg_not_empty_p(line_seg) == 0)
        as libc::c_int;
    if !all_segs.is_null() && empty_debug_line == 0
        && dwarf2_any_loc_directive_seen as libc::c_int != 0
    {
        as_fatal(
            b"duplicate .debug_line sections\0" as *const u8 as *const libc::c_char,
        );
    }
    if all_segs.is_null() && emit_other_sections != 0
        || emit_other_sections == 0 && empty_debug_line == 0
    {
        return;
    }
    sizeof_address = x86_dwarf2_addr_size() as libc::c_uint;
    if empty_debug_line != 0 {
        line_seg = subseg_new(
            b".debug_line\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
        bfd_set_section_flags(
            line_seg,
            (0x8 as libc::c_int | 0x2000 as libc::c_int
                | (if 1 as libc::c_int != 0 {
                    0x40000000 as libc::c_int
                } else {
                    0 as libc::c_int
                })) as flagword,
        );
    }
    s = all_segs;
    while !s.is_null() {
        let mut lss: *mut line_subseg = (*s).head;
        let mut ptail: *mut *mut line_entry = (*lss).ptail;
        if !((*lss).head).is_null() && !((*(*lss).head).loc.view).is_null() {
            set_or_check_view((*lss).head, 0 as *mut line_entry, 0 as *mut line_entry);
        }
        loop {
            lss = (*lss).next;
            if lss.is_null() {
                break;
            }
            if !((*lss).head).is_null() && !((*(*lss).head).loc.view).is_null() {
                set_or_check_view(
                    (*lss).head,
                    if ((*s).head).is_null() {
                        0 as *mut line_entry
                    } else {
                        ptail as *mut line_entry
                    },
                    if !((*s).head).is_null() {
                        (*(*s).head).head
                    } else {
                        0 as *mut line_entry
                    },
                );
            }
            *ptail = (*lss).head;
            ptail = (*lss).ptail;
        }
        s = (*s).next;
    }
    if empty_debug_line != 0 {
        out_debug_line(line_seg);
    }
    if emit_other_sections != 0 {
        let mut abbrev_seg: segT = 0 as *mut asection;
        let mut aranges_seg: segT = 0 as *mut asection;
        let mut str_seg: segT = 0 as *mut asection;
        let mut name_sym: *mut symbolS = 0 as *mut symbolS;
        let mut comp_dir_sym: *mut symbolS = 0 as *mut symbolS;
        let mut producer_sym: *mut symbolS = 0 as *mut symbolS;
        let mut ranges_sym: *mut symbolS = 0 as *mut symbolS;
        if !all_segs.is_null() {} else {
            as_abort(
                b"dwarf2dbg.c\0" as *const u8 as *const libc::c_char,
                2860 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 25],
                    &[libc::c_char; 25],
                >(b"void dwarf2_finish(void)\0"))
                    .as_ptr(),
            );
        };
        info_seg = subseg_new(
            b".debug_info\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
        abbrev_seg = subseg_new(
            b".debug_abbrev\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
        aranges_seg = subseg_new(
            b".debug_aranges\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
        str_seg = subseg_new(
            b".debug_str\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
        bfd_set_section_flags(
            info_seg,
            (0x8 as libc::c_int | 0x2000 as libc::c_int
                | (if 1 as libc::c_int != 0 {
                    0x40000000 as libc::c_int
                } else {
                    0 as libc::c_int
                })) as flagword,
        );
        bfd_set_section_flags(
            abbrev_seg,
            (0x8 as libc::c_int | 0x2000 as libc::c_int
                | (if 1 as libc::c_int != 0 {
                    0x40000000 as libc::c_int
                } else {
                    0 as libc::c_int
                })) as flagword,
        );
        bfd_set_section_flags(
            aranges_seg,
            (0x8 as libc::c_int | 0x2000 as libc::c_int
                | (if 1 as libc::c_int != 0 {
                    0x40000000 as libc::c_int
                } else {
                    0 as libc::c_int
                })) as flagword,
        );
        bfd_set_section_flags(
            str_seg,
            (0x8 as libc::c_int | 0x2000 as libc::c_int
                | (if 1 as libc::c_int != 0 {
                    0x40000000 as libc::c_int
                } else {
                    0 as libc::c_int
                }) | 0x800000 as libc::c_int | 0x1000000 as libc::c_int) as flagword,
        );
        (*str_seg).entsize = 1 as libc::c_int as libc::c_uint;
        record_alignment(
            aranges_seg,
            (ffs(
                (2 as libc::c_int as libc::c_uint).wrapping_mul(sizeof_address)
                    as libc::c_int,
            ) - 1 as libc::c_int) as libc::c_uint,
        );
        if ((*all_segs).next).is_null() {
            ranges_sym = 0 as *mut symbolS;
        } else if dwarf_level < 5 as libc::c_int as libc::c_uint {
            let mut ranges_seg: segT = subseg_new(
                b".debug_ranges\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int,
            );
            bfd_set_section_flags(
                ranges_seg,
                (0x8 as libc::c_int | 0x2000 as libc::c_int
                    | (if 1 as libc::c_int != 0 {
                        0x40000000 as libc::c_int
                    } else {
                        0 as libc::c_int
                    })) as flagword,
            );
            record_alignment(
                ranges_seg,
                (ffs(
                    (2 as libc::c_int as libc::c_uint).wrapping_mul(sizeof_address)
                        as libc::c_int,
                ) - 1 as libc::c_int) as libc::c_uint,
            );
            out_debug_ranges(ranges_seg, &mut ranges_sym);
        } else {
            let mut rnglists_seg: segT = subseg_new(
                b".debug_rnglists\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int,
            );
            bfd_set_section_flags(
                rnglists_seg,
                (0x8 as libc::c_int | 0x2000 as libc::c_int
                    | (if 1 as libc::c_int != 0 {
                        0x40000000 as libc::c_int
                    } else {
                        0 as libc::c_int
                    })) as flagword,
            );
            out_debug_rnglists(rnglists_seg, &mut ranges_sym);
        }
        out_debug_aranges(aranges_seg, info_seg);
        out_debug_abbrev(abbrev_seg, info_seg, line_seg);
        out_debug_str(str_seg, &mut name_sym, &mut comp_dir_sym, &mut producer_sym);
        out_debug_info(
            info_seg,
            abbrev_seg,
            line_seg,
            ranges_sym,
            name_sym,
            comp_dir_sym,
            producer_sym,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn dwarf2dbg_estimate_size_before_relax(
    mut frag: *mut fragS,
) -> libc::c_int {
    let mut addr_delta: offsetT = 0;
    let mut size: libc::c_int = 0;
    addr_delta = resolve_symbol_value((*frag).fr_symbol) as offsetT;
    if linkrelax != 0 {
        size = size_fixed_inc_line_addr(
            (*frag).fr_offset as libc::c_int,
            addr_delta as addressT,
        );
    } else {
        size = size_inc_line_addr(
            (*frag).fr_offset as libc::c_int,
            addr_delta as addressT,
        );
    }
    (*frag).fr_subtype = size as relax_substateT;
    return size;
}
#[no_mangle]
pub unsafe extern "C" fn dwarf2dbg_relax_frag(mut frag: *mut fragS) -> libc::c_int {
    let mut old_size: libc::c_int = 0;
    let mut new_size: libc::c_int = 0;
    old_size = (*frag).fr_subtype as libc::c_int;
    new_size = dwarf2dbg_estimate_size_before_relax(frag);
    return new_size - old_size;
}
#[no_mangle]
pub unsafe extern "C" fn dwarf2dbg_convert_frag(mut frag: *mut fragS) {
    let mut addr_diff: offsetT = 0;
    if linkrelax != 0 {
        let mut saved_finalize_syms: libc::c_int = finalize_syms;
        finalize_syms = 0 as libc::c_int;
        addr_diff = resolve_symbol_value((*frag).fr_symbol) as offsetT;
        finalize_syms = saved_finalize_syms;
    } else {
        addr_diff = resolve_symbol_value((*frag).fr_symbol) as offsetT;
    }
    if (*frag).fr_var >= (*frag).fr_subtype as libc::c_int as libc::c_long {} else {
        as_abort(
            b"dwarf2dbg.c\0" as *const u8 as *const libc::c_char,
            1848 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 37],
                &[libc::c_char; 37],
            >(b"void dwarf2dbg_convert_frag(fragS *)\0"))
                .as_ptr(),
        );
    };
    if linkrelax != 0 {
        emit_fixed_inc_line_addr(
            (*frag).fr_offset as libc::c_int,
            addr_diff as addressT,
            frag,
            ((*frag).fr_literal).as_mut_ptr().offset((*frag).fr_fix as isize),
            (*frag).fr_subtype as libc::c_int,
        );
    } else {
        emit_inc_line_addr(
            (*frag).fr_offset as libc::c_int,
            addr_diff as addressT,
            ((*frag).fr_literal).as_mut_ptr().offset((*frag).fr_fix as isize),
            (*frag).fr_subtype as libc::c_int,
        );
    }
    (*frag)
        .fr_fix = ((*frag).fr_fix as libc::c_ulong)
        .wrapping_add((*frag).fr_subtype as libc::c_ulong) as valueT as valueT;
    (*frag).fr_type = rs_fill;
    (*frag).fr_var = 0 as libc::c_int as offsetT;
    (*frag).fr_offset = 0 as libc::c_int as offsetT;
}
#[no_mangle]
pub unsafe extern "C" fn dwarf2dbg_final_check() {
    while !view_assert_failed.is_null() {
        let mut exp: *mut expressionS = 0 as *mut expressionS;
        let mut sym: *mut symbolS = 0 as *mut symbolS;
        let mut failed: offsetT = 0;
        if symbol_resolved_p(view_assert_failed) == 0 {} else {
            as_abort(
                b"dwarf2dbg.c\0" as *const u8 as *const libc::c_char,
                2926 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 33],
                    &[libc::c_char; 33],
                >(b"void dwarf2dbg_final_check(void)\0"))
                    .as_ptr(),
            );
        };
        exp = symbol_get_value_expression(view_assert_failed);
        sym = view_assert_failed;
        if (*exp).X_op() as libc::c_int == O_add as libc::c_int
            && (*exp).X_add_number == 0 as libc::c_int as libc::c_long
            && (*exp).X_unsigned() as libc::c_int != 0
        {
            view_assert_failed = (*exp).X_add_symbol;
            sym = (*exp).X_op_symbol;
        } else {
            view_assert_failed = 0 as *mut symbolS;
        }
        failed = resolve_symbol_value(sym) as offsetT;
        if !(symbol_resolved_p(sym) == 0 || failed != 0) {
            continue;
        }
        as_bad(
            dcgettext(
                0 as *const libc::c_char,
                b"view number mismatch\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        break;
    }
}
static mut all_segs: *mut line_seg = 0 as *const line_seg as *mut line_seg;
static mut last_seg_ptr: *mut *mut line_seg = 0 as *const *mut line_seg
    as *mut *mut line_seg;
static mut files: *mut file_entry = 0 as *const file_entry as *mut file_entry;
static mut files_in_use: libc::c_uint = 0;
static mut files_allocated: libc::c_uint = 0;
static mut num_of_auto_assigned: libc::c_uint = 0;
static mut dirs: *mut *mut libc::c_char = 0 as *const *mut libc::c_char
    as *mut *mut libc::c_char;
static mut dirs_in_use: libc::c_uint = 0 as libc::c_int as libc::c_uint;
static mut dirs_allocated: libc::c_uint = 0 as libc::c_int as libc::c_uint;
static mut dwarf2_any_loc_directive_seen: bool = false;
static mut current: dwarf2_line_info = dwarf2_line_info {
    filenum: 0,
    line: 0,
    column: 0,
    isa: 0,
    flags: 0,
    discriminator: 0,
    view: 0 as *mut symbolS,
};
static mut force_reset_view: *mut symbolS = 0 as *const symbolS as *mut symbolS;
static mut view_assert_failed: *mut symbolS = 0 as *const symbolS as *mut symbolS;
static mut sizeof_address: libc::c_uint = 0;
unsafe extern "C" fn generic_dwarf2_emit_offset(
    mut symbol: *mut symbolS,
    mut size: libc::c_uint,
) {
    let mut exp: expressionS = expressionS {
        X_add_symbol: 0 as *mut symbolS,
        X_op_symbol: 0 as *mut symbolS,
        X_add_number: 0,
        X_op_X_unsigned_X_extrabit: [0; 2],
        X_md: 0,
    };
    memset(
        &mut exp as *mut expressionS as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<expressionS>() as libc::c_ulong,
    );
    exp.set_X_op(O_symbol);
    exp.X_add_symbol = symbol;
    exp.X_add_number = 0 as libc::c_int as offsetT;
    emit_expr(&mut exp, size);
}
unsafe extern "C" fn get_line_subseg(
    mut seg: segT,
    mut subseg: subsegT,
    mut create_p: bool,
) -> *mut line_subseg {
    let mut current_block: u64;
    let mut s: *mut line_seg = (*(bfd_section_userdata(seg as *const asection)
        as *mut segment_info_type))
        .dwarf2_line_seg as *mut line_seg;
    let mut pss: *mut *mut line_subseg = 0 as *mut *mut line_subseg;
    let mut lss: *mut line_subseg = 0 as *mut line_subseg;
    if s.is_null() {
        if !create_p {
            return 0 as *mut line_subseg;
        }
        s = xmalloc(::core::mem::size_of::<line_seg>() as libc::c_ulong)
            as *mut line_seg;
        (*s).next = 0 as *mut line_seg;
        (*s).seg = seg;
        (*s).head = 0 as *mut line_subseg;
        *last_seg_ptr = s;
        last_seg_ptr = &mut (*s).next;
        let ref mut fresh1 = (*(bfd_section_userdata(seg as *const asection)
            as *mut segment_info_type))
            .dwarf2_line_seg;
        *fresh1 = s as *mut libc::c_void;
    }
    if seg == (*s).seg {} else {
        as_abort(
            b"dwarf2dbg.c\0" as *const u8 as *const libc::c_char,
            290 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 58],
                &[libc::c_char; 58],
            >(b"struct line_subseg *get_line_subseg(segT, subsegT, _Bool)\0"))
                .as_ptr(),
        );
    };
    pss = &mut (*s).head;
    loop {
        lss = *pss;
        if lss.is_null() {
            current_block = 13586036798005543211;
            break;
        }
        if (*lss).subseg == subseg {
            current_block = 13588463733889387893;
            break;
        }
        if (*lss).subseg > subseg {
            current_block = 13586036798005543211;
            break;
        }
        pss = &mut (*lss).next;
    }
    match current_block {
        13586036798005543211 => {
            lss = xmalloc(::core::mem::size_of::<line_subseg>() as libc::c_ulong)
                as *mut line_subseg;
            (*lss).next = *pss;
            (*lss).subseg = subseg;
            (*lss).head = 0 as *mut line_entry;
            (*lss).ptail = &mut (*lss).head;
            (*lss).pmove_tail = &mut (*lss).head;
            *pss = lss;
        }
        _ => {}
    }
    return lss;
}
unsafe extern "C" fn reverse_line_entry_list(mut h: *mut line_entry) -> *mut line_entry {
    let mut p: *mut line_entry = 0 as *mut line_entry;
    let mut e: *mut line_entry = 0 as *mut line_entry;
    let mut n: *mut line_entry = 0 as *mut line_entry;
    e = h;
    while !e.is_null() {
        n = (*e).next;
        (*e).next = p;
        p = e;
        e = n;
    }
    return p;
}
unsafe extern "C" fn set_or_check_view(
    mut e: *mut line_entry,
    mut p: *mut line_entry,
    mut h: *mut line_entry,
) {
    let mut viewx: expressionS = expressionS {
        X_add_symbol: 0 as *mut symbolS,
        X_op_symbol: 0 as *mut symbolS,
        X_add_number: 0,
        X_op_X_unsigned_X_extrabit: [0; 2],
        X_md: 0,
    };
    memset(
        &mut viewx as *mut expressionS as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<expressionS>() as libc::c_ulong,
    );
    viewx.set_X_unsigned(1 as libc::c_int as libc::c_uint);
    if p.is_null() || (*e).loc.view == force_reset_view && !force_reset_view.is_null() {
        viewx.set_X_op(O_constant);
        viewx.X_add_number = 0 as libc::c_int as offsetT;
        viewx.X_add_symbol = 0 as *mut symbolS;
        viewx.X_op_symbol = 0 as *mut symbolS;
    } else {
        viewx.set_X_op(O_gt);
        viewx.X_add_number = 0 as libc::c_int as offsetT;
        viewx.X_add_symbol = (*e).label;
        viewx.X_op_symbol = (*p).label;
        resolve_expression(&mut viewx);
        if viewx.X_op() as libc::c_int == O_constant as libc::c_int {
            viewx.X_add_number = (viewx.X_add_number == 0) as libc::c_int as offsetT;
        } else {
            viewx.X_add_symbol = make_expr_symbol(&mut viewx);
            viewx.X_add_number = 0 as libc::c_int as offsetT;
            viewx.X_op_symbol = 0 as *mut symbolS;
            viewx.set_X_op(O_logical_not);
        }
    }
    if S_IS_DEFINED((*e).loc.view) != 0 && symbol_constant_p((*e).loc.view) != 0 {
        let mut value: *mut expressionS = symbol_get_value_expression((*e).loc.view);
        if viewx.X_op() as libc::c_int == O_constant as libc::c_int {
            if ((*value).X_add_number == 0) as libc::c_int
                != (viewx.X_add_number == 0) as libc::c_int
            {
                as_bad(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"view number mismatch\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
            }
        } else if (*value).X_add_number == 0 {
            let mut deferred: *mut symbolS = make_expr_symbol(&mut viewx);
            if !view_assert_failed.is_null() {
                let mut chk: expressionS = expressionS {
                    X_add_symbol: 0 as *mut symbolS,
                    X_op_symbol: 0 as *mut symbolS,
                    X_add_number: 0,
                    X_op_X_unsigned_X_extrabit: [0; 2],
                    X_md: 0,
                };
                memset(
                    &mut chk as *mut expressionS as *mut libc::c_void,
                    0 as libc::c_int,
                    ::core::mem::size_of::<expressionS>() as libc::c_ulong,
                );
                chk.set_X_unsigned(1 as libc::c_int as libc::c_uint);
                chk.set_X_op(O_add);
                chk.X_add_number = 0 as libc::c_int as offsetT;
                chk.X_add_symbol = view_assert_failed;
                chk.X_op_symbol = deferred;
                deferred = make_expr_symbol(&mut chk);
            }
            view_assert_failed = deferred;
        }
    }
    if viewx.X_op() as libc::c_int != O_constant as libc::c_int
        || viewx.X_add_number != 0
    {
        let mut incv: expressionS = expressionS {
            X_add_symbol: 0 as *mut symbolS,
            X_op_symbol: 0 as *mut symbolS,
            X_add_number: 0,
            X_op_X_unsigned_X_extrabit: [0; 2],
            X_md: 0,
        };
        if ((*p).loc.view).is_null() {
            (*p).loc.view = symbol_temp_make();
            if S_IS_DEFINED((*p).loc.view) == 0 {} else {
                as_abort(
                    b"dwarf2dbg.c\0" as *const u8 as *const libc::c_char,
                    410 as libc::c_int,
                    (*::core::mem::transmute::<
                        &[u8; 86],
                        &[libc::c_char; 86],
                    >(
                        b"void set_or_check_view(struct line_entry *, struct line_entry *, struct line_entry *)\0",
                    ))
                        .as_ptr(),
                );
            };
        }
        memset(
            &mut incv as *mut expressionS as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<expressionS>() as libc::c_ulong,
        );
        incv.set_X_unsigned(1 as libc::c_int as libc::c_uint);
        incv.set_X_op(O_symbol);
        incv.X_add_symbol = (*p).loc.view;
        incv.X_add_number = 1 as libc::c_int as offsetT;
        if viewx.X_op() as libc::c_int == O_constant as libc::c_int {
            if viewx.X_add_number == 1 as libc::c_int as libc::c_long {} else {
                as_abort(
                    b"dwarf2dbg.c\0" as *const u8 as *const libc::c_char,
                    421 as libc::c_int,
                    (*::core::mem::transmute::<
                        &[u8; 86],
                        &[libc::c_char; 86],
                    >(
                        b"void set_or_check_view(struct line_entry *, struct line_entry *, struct line_entry *)\0",
                    ))
                        .as_ptr(),
                );
            };
            viewx = incv;
        } else {
            viewx.X_add_symbol = make_expr_symbol(&mut viewx);
            viewx.X_add_number = 0 as libc::c_int as offsetT;
            viewx.X_op_symbol = make_expr_symbol(&mut incv);
            viewx.set_X_op(O_multiply);
        }
    }
    if S_IS_DEFINED((*e).loc.view) == 0 {
        symbol_set_value_expression((*e).loc.view, &mut viewx);
        S_SET_SEGMENT((*e).loc.view, expr_section);
        symbol_set_frag((*e).loc.view, &mut zero_address_frag);
    }
    if !h.is_null() && !p.is_null() && !((*p).loc.view).is_null()
        && S_IS_DEFINED((*p).loc.view) == 0
    {
        let mut h2: *mut line_entry = 0 as *mut line_entry;
        let mut r: *mut line_entry = reverse_line_entry_list(h);
        if r == p {} else {
            as_abort(
                b"dwarf2dbg.c\0" as *const u8 as *const libc::c_char,
                449 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 86],
                    &[libc::c_char; 86],
                >(
                    b"void set_or_check_view(struct line_entry *, struct line_entry *, struct line_entry *)\0",
                ))
                    .as_ptr(),
            );
        };
        while !(r == h) {
            set_or_check_view(r, (*r).next, 0 as *mut line_entry);
            if !(!((*r).next).is_null() && !((*(*r).next).loc.view).is_null()
                && S_IS_DEFINED((*(*r).next).loc.view) == 0
                && {
                    r = (*r).next;
                    !r.is_null()
                })
            {
                break;
            }
        }
        h2 = reverse_line_entry_list(p);
        if h2 == h {} else {
            as_abort(
                b"dwarf2dbg.c\0" as *const u8 as *const libc::c_char,
                467 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 86],
                    &[libc::c_char; 86],
                >(
                    b"void set_or_check_view(struct line_entry *, struct line_entry *, struct line_entry *)\0",
                ))
                    .as_ptr(),
            );
        };
        loop {
            if !(r == h) {
                if S_IS_DEFINED((*r).loc.view) != 0 {} else {
                    as_abort(
                        b"dwarf2dbg.c\0" as *const u8 as *const libc::c_char,
                        478 as libc::c_int,
                        (*::core::mem::transmute::<
                            &[u8; 86],
                            &[libc::c_char; 86],
                        >(
                            b"void set_or_check_view(struct line_entry *, struct line_entry *, struct line_entry *)\0",
                        ))
                            .as_ptr(),
                    );
                };
                resolve_expression(symbol_get_value_expression((*r).loc.view));
            }
            if !(r != p
                && {
                    r = (*r).next;
                    !r.is_null()
                })
            {
                break;
            }
        }
        resolve_expression(symbol_get_value_expression((*e).loc.view));
    }
}
unsafe extern "C" fn dwarf2_gen_line_info_1(
    mut label: *mut symbolS,
    mut loc: *mut dwarf2_line_info,
) {
    let mut lss: *mut line_subseg = 0 as *mut line_subseg;
    let mut e: *mut line_entry = 0 as *mut line_entry;
    let mut need_flags: flagword = (0x2 as libc::c_int | 0x10 as libc::c_int)
        as flagword;
    need_flags |= 0x1 as libc::c_int as libc::c_uint;
    if (*now_seg).flags & need_flags != need_flags {
        if debug_type as libc::c_uint != DEBUG_DWARF2 as libc::c_int as libc::c_uint {
            as_warn(
                b"dwarf line number information for %s ignored\0" as *const u8
                    as *const libc::c_char,
                bfd_section_name(now_seg as *const asection),
            );
        }
        return;
    }
    e = xmalloc(::core::mem::size_of::<line_entry>() as libc::c_ulong)
        as *mut line_entry;
    (*e).next = 0 as *mut line_entry;
    (*e).label = label;
    (*e).loc = *loc;
    lss = get_line_subseg(now_seg, now_subseg, 1 as libc::c_int != 0);
    if !((*loc).view).is_null() && !((*lss).head).is_null() {
        set_or_check_view(e, (*lss).ptail as *mut line_entry, (*lss).head);
    }
    *(*lss).ptail = e;
    (*lss).ptail = &mut (*e).next;
}
unsafe extern "C" fn get_basename(
    mut pathname: *const libc::c_char,
) -> *const libc::c_char {
    let mut file: *const libc::c_char = 0 as *const libc::c_char;
    file = lbasename(pathname);
    if file == pathname.offset(1 as libc::c_int as isize) {
        file = pathname;
    }
    return file;
}
unsafe extern "C" fn get_directory_table_entry(
    mut dirname: *const libc::c_char,
    mut dirlen: size_t,
    mut can_use_zero: bool,
) -> libc::c_uint {
    let mut d: libc::c_uint = 0;
    if dirlen == 0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int as libc::c_uint;
    }
    if *dirname.offset(dirlen.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
        as libc::c_int == '/' as i32
        || *dirname
            .offset(dirlen.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
            as libc::c_int == '\\' as i32 && 0 as libc::c_int != 0
    {
        dirlen = dirlen.wrapping_sub(1);
        dirlen;
        if dirlen == 0 as libc::c_int as libc::c_ulong {
            return 0 as libc::c_int as libc::c_uint;
        }
    }
    d = 0 as libc::c_int as libc::c_uint;
    while d < dirs_in_use {
        if !(*dirs.offset(d as isize)).is_null()
            && filename_ncmp(dirname, *dirs.offset(d as isize), dirlen)
                == 0 as libc::c_int
            && *(*dirs.offset(d as isize)).offset(dirlen as isize) as libc::c_int
                == '\0' as i32
        {
            return d;
        }
        d = d.wrapping_add(1);
        d;
    }
    if can_use_zero {
        if dirs.is_null() || (*dirs.offset(0 as libc::c_int as isize)).is_null() {
            d = 0 as libc::c_int as libc::c_uint;
        }
    } else if d == 0 as libc::c_int as libc::c_uint {
        d = 1 as libc::c_int as libc::c_uint;
    }
    if d >= dirs_allocated {
        let mut old: libc::c_uint = dirs_allocated;
        dirs_allocated = d.wrapping_add(32 as libc::c_int as libc::c_uint);
        dirs = xrealloc(
            dirs as *mut libc::c_void,
            (::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
                .wrapping_mul(dirs_allocated as libc::c_ulong),
        ) as *mut *mut libc::c_char;
        memset(
            dirs.offset(old as isize) as *mut libc::c_void,
            0 as libc::c_int,
            (dirs_allocated.wrapping_sub(old) as libc::c_ulong)
                .wrapping_mul(
                    ::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                ),
        );
    }
    let ref mut fresh2 = *dirs.offset(d as isize);
    *fresh2 = xmemdup0(dirname, dirlen);
    if dirs_in_use <= d {
        dirs_in_use = d.wrapping_add(1 as libc::c_int as libc::c_uint);
    }
    return d;
}
unsafe extern "C" fn assign_file_to_slot(
    mut i: libc::c_ulong,
    mut file: *const libc::c_char,
    mut dir: libc::c_uint,
) -> bool {
    if i >= files_allocated as libc::c_ulong {
        let mut old: libc::c_uint = files_allocated;
        files_allocated = i.wrapping_add(32 as libc::c_int as libc::c_ulong)
            as libc::c_uint;
        if files_allocated <= old {
            as_bad(
                dcgettext(
                    0 as *const libc::c_char,
                    b"file number %lu is too big\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                i,
            );
            return 0 as libc::c_int != 0;
        }
        files = xrealloc(
            files as *mut libc::c_void,
            (::core::mem::size_of::<file_entry>() as libc::c_ulong)
                .wrapping_mul(files_allocated as libc::c_ulong),
        ) as *mut file_entry;
        memset(
            files.offset(old as isize) as *mut libc::c_void,
            0 as libc::c_int,
            i
                .wrapping_add(32 as libc::c_int as libc::c_ulong)
                .wrapping_sub(old as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<file_entry>() as libc::c_ulong),
        );
    }
    let ref mut fresh3 = (*files.offset(i as isize)).filename;
    *fresh3 = file;
    (*files.offset(i as isize)).dir = dir;
    memset(
        ((*files.offset(i as isize)).md5).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        16 as libc::c_int as libc::c_ulong,
    );
    if (files_in_use as libc::c_ulong)
        < i.wrapping_add(1 as libc::c_int as libc::c_ulong)
    {
        files_in_use = i.wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_uint;
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn allocate_filenum(mut pathname: *const libc::c_char) -> libc::c_int {
    static mut last_used: libc::c_int = -(1 as libc::c_int);
    static mut last_used_dir_len: libc::c_int = 0 as libc::c_int;
    let mut file: *const libc::c_char = 0 as *const libc::c_char;
    let mut dir_len: size_t = 0;
    let mut i: libc::c_uint = 0;
    let mut dir: libc::c_uint = 0;
    if last_used != -(1 as libc::c_int) {
        let mut dirname: *const libc::c_char = 0 as *const libc::c_char;
        if !dirs.is_null() {
            dirname = *dirs.offset((*files.offset(last_used as isize)).dir as isize);
        }
        if dirname.is_null() {
            if filename_cmp(pathname, (*files.offset(last_used as isize)).filename)
                == 0 as libc::c_int
            {
                return last_used;
            }
        } else if filename_ncmp(pathname, dirname, last_used_dir_len as size_t)
            == 0 as libc::c_int
            && (*pathname.offset(last_used_dir_len as isize) as libc::c_int == '/' as i32
                || *pathname.offset(last_used_dir_len as isize) as libc::c_int
                    == '\\' as i32 && 0 as libc::c_int != 0)
            && filename_cmp(
                pathname
                    .offset(last_used_dir_len as isize)
                    .offset(1 as libc::c_int as isize),
                (*files.offset(last_used as isize)).filename,
            ) == 0 as libc::c_int
        {
            return last_used
        }
    }
    file = get_basename(pathname);
    dir_len = file.offset_from(pathname) as libc::c_long as size_t;
    dir = get_directory_table_entry(pathname, dir_len, 0 as libc::c_int != 0);
    i = 1 as libc::c_int as libc::c_uint;
    while i < files_in_use {
        if (*files.offset(i as isize)).dir == dir
            && !((*files.offset(i as isize)).filename).is_null()
            && filename_cmp(file, (*files.offset(i as isize)).filename)
                == 0 as libc::c_int
        {
            last_used = i as libc::c_int;
            last_used_dir_len = dir_len as libc::c_int;
            return i as libc::c_int;
        }
        i = i.wrapping_add(1);
        i;
    }
    if !assign_file_to_slot(i as libc::c_ulong, file, dir) {
        return -(1 as libc::c_int);
    }
    num_of_auto_assigned = num_of_auto_assigned.wrapping_add(1);
    num_of_auto_assigned;
    last_used = i as libc::c_int;
    last_used_dir_len = dir_len as libc::c_int;
    return i as libc::c_int;
}
unsafe extern "C" fn allocate_filename_to_slot(
    mut dirname: *const libc::c_char,
    mut filename: *const libc::c_char,
    mut num: libc::c_uint,
    mut with_md5: bool,
) -> bool {
    let mut file: *const libc::c_char = 0 as *const libc::c_char;
    let mut dirlen: size_t = 0;
    let mut i: libc::c_uint = 0;
    let mut d: libc::c_uint = 0;
    if num < files_allocated && !((*files.offset(num as isize)).filename).is_null() {
        let mut dir: *const libc::c_char = 0 as *const libc::c_char;
        if !dirs.is_null() {
            dir = *dirs.offset((*files.offset(num as isize)).dir as isize);
        }
        if !(with_md5 as libc::c_int != 0
            && memcmp(
                generic_bignum.as_mut_ptr() as *const libc::c_void,
                ((*files.offset(num as isize)).md5).as_mut_ptr() as *const libc::c_void,
                16 as libc::c_int as libc::c_ulong,
            ) != 0 as libc::c_int)
        {
            if !dirname.is_null() {
                if !(!dir.is_null() && filename_cmp(dir, dirname) != 0 as libc::c_int) {
                    if !(filename_cmp(filename, (*files.offset(num as isize)).filename)
                        != 0 as libc::c_int)
                    {
                        if dir.is_null() {
                            if dirs.is_null() {
                                dirs_allocated = ((*files.offset(num as isize)).dir)
                                    .wrapping_add(32 as libc::c_int as libc::c_uint);
                                dirs = xcalloc(
                                    dirs_allocated as size_t,
                                    ::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                                ) as *mut *mut libc::c_char;
                            }
                            let ref mut fresh4 = *dirs
                                .offset((*files.offset(num as isize)).dir as isize);
                            *fresh4 = xmemdup0(dirname, strlen(dirname));
                        }
                        return 1 as libc::c_int != 0;
                    }
                }
            } else if !dir.is_null() {
                dirlen = strlen(dir);
                if filename_ncmp(filename, dir, dirlen) == 0 as libc::c_int
                    && (*filename.offset(dirlen as isize) as libc::c_int == '/' as i32
                        || *filename.offset(dirlen as isize) as libc::c_int
                            == '\\' as i32 && 0 as libc::c_int != 0)
                    && filename_cmp(
                        filename
                            .offset(dirlen as isize)
                            .offset(1 as libc::c_int as isize),
                        (*files.offset(num as isize)).filename,
                    ) == 0 as libc::c_int
                {
                    return 1 as libc::c_int != 0;
                }
            } else {
                file = get_basename(filename);
                if filename_cmp(file, (*files.offset(num as isize)).filename)
                    == 0 as libc::c_int
                {
                    if file > filename {
                        if dirs.is_null() {
                            dirs_allocated = ((*files.offset(num as isize)).dir)
                                .wrapping_add(32 as libc::c_int as libc::c_uint);
                            dirs = xcalloc(
                                dirs_allocated as size_t,
                                ::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                            ) as *mut *mut libc::c_char;
                        }
                        let ref mut fresh5 = *dirs
                            .offset((*files.offset(num as isize)).dir as isize);
                        *fresh5 = xmemdup0(
                            filename,
                            file.offset_from(filename) as libc::c_long as size_t,
                        );
                    }
                    return 1 as libc::c_int != 0;
                }
            }
        }
        as_bad(
            dcgettext(
                0 as *const libc::c_char,
                b"file table slot %u is already occupied by a different file (%s%s%s vs %s%s%s)\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            num,
            if dir.is_null() { b"\0" as *const u8 as *const libc::c_char } else { dir },
            if dir.is_null() {
                b"\0" as *const u8 as *const libc::c_char
            } else {
                b"/\0" as *const u8 as *const libc::c_char
            },
            (*files.offset(num as isize)).filename,
            if dirname.is_null() {
                b"\0" as *const u8 as *const libc::c_char
            } else {
                dirname
            },
            if dirname.is_null() {
                b"\0" as *const u8 as *const libc::c_char
            } else {
                b"/\0" as *const u8 as *const libc::c_char
            },
            filename,
        );
        return 0 as libc::c_int != 0;
    } else {
        if dirname.is_null() {
            dirname = filename;
            file = get_basename(filename);
            dirlen = file.offset_from(filename) as libc::c_long as size_t;
        } else {
            dirlen = strlen(dirname);
            file = filename;
        }
        d = get_directory_table_entry(
            dirname,
            dirlen,
            num == 0 as libc::c_int as libc::c_uint,
        );
        i = num;
        if !assign_file_to_slot(i as libc::c_ulong, file, d) {
            return 0 as libc::c_int != 0;
        }
        if with_md5 {
            if target_big_endian != 0 {
                let mut bits_remaining: libc::c_uint = (16 as libc::c_int
                    * 8 as libc::c_int) as libc::c_uint;
                let mut byte: libc::c_uint = 16 as libc::c_int as libc::c_uint;
                let mut bignum_index: libc::c_uint = 0 as libc::c_int as libc::c_uint;
                while bits_remaining != 0 {
                    let mut bignum_bits_remaining: libc::c_uint = 16 as libc::c_int
                        as libc::c_uint;
                    let mut bignum_value: valueT = *generic_bignum
                        .as_mut_ptr()
                        .offset(bignum_index as isize) as valueT;
                    bignum_index = bignum_index.wrapping_add(1);
                    bignum_index;
                    while bignum_bits_remaining != 0 {
                        byte = byte.wrapping_sub(1);
                        (*files.offset(i as isize))
                            .md5[byte
                            as usize] = (bignum_value
                            & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                        bignum_value >>= 8 as libc::c_int;
                        bignum_bits_remaining = bignum_bits_remaining
                            .wrapping_sub(8 as libc::c_int as libc::c_uint);
                        bits_remaining = bits_remaining
                            .wrapping_sub(8 as libc::c_int as libc::c_uint);
                    }
                }
            } else {
                let mut bits_remaining_0: libc::c_uint = (16 as libc::c_int
                    * 8 as libc::c_int) as libc::c_uint;
                let mut byte_0: libc::c_uint = 0 as libc::c_int as libc::c_uint;
                let mut bignum_index_0: libc::c_uint = 0 as libc::c_int as libc::c_uint;
                while bits_remaining_0 != 0 {
                    let mut bignum_bits_remaining_0: libc::c_uint = 16 as libc::c_int
                        as libc::c_uint;
                    let mut bignum_value_0: valueT = *generic_bignum
                        .as_mut_ptr()
                        .offset(bignum_index_0 as isize) as valueT;
                    bignum_index_0 = bignum_index_0.wrapping_add(1);
                    bignum_index_0;
                    while bignum_bits_remaining_0 != 0 {
                        let fresh6 = byte_0;
                        byte_0 = byte_0.wrapping_add(1);
                        (*files.offset(i as isize))
                            .md5[fresh6
                            as usize] = (bignum_value_0
                            & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                        bignum_value_0 >>= 8 as libc::c_int;
                        bignum_bits_remaining_0 = bignum_bits_remaining_0
                            .wrapping_sub(8 as libc::c_int as libc::c_uint);
                        bits_remaining_0 = bits_remaining_0
                            .wrapping_sub(8 as libc::c_int as libc::c_uint);
                    }
                }
            }
        } else {
            memset(
                ((*files.offset(i as isize)).md5).as_mut_ptr() as *mut libc::c_void,
                0 as libc::c_int,
                16 as libc::c_int as libc::c_ulong,
            );
        }
        return 1 as libc::c_int != 0;
    };
}
unsafe extern "C" fn first_frag_for_seg(mut seg: segT) -> *mut frag {
    return (*(*(bfd_section_userdata(seg as *const asection) as *mut segment_info_type))
        .frchainP)
        .frch_root;
}
unsafe extern "C" fn last_frag_for_seg(mut seg: segT) -> *mut frag {
    let mut f: *mut frchainS = (*(bfd_section_userdata(seg as *const asection)
        as *mut segment_info_type))
        .frchainP;
    while !((*f).frch_next).is_null() {
        f = (*f).frch_next;
    }
    return (*f).frch_last;
}
#[inline]
unsafe extern "C" fn out_byte(mut byte: libc::c_int) {
    frag_append_1_char(byte);
}
#[inline]
unsafe extern "C" fn out_opcode(mut opc: libc::c_int) {
    out_byte(opc);
}
#[inline]
unsafe extern "C" fn out_two(mut data: libc::c_int) {
    number_to_chars_littleendian(
        frag_more(2 as libc::c_int as size_t),
        data as valueT,
        2 as libc::c_int,
    );
}
#[inline]
unsafe extern "C" fn out_four(mut data: libc::c_int) {
    number_to_chars_littleendian(
        frag_more(4 as libc::c_int as size_t),
        data as valueT,
        4 as libc::c_int,
    );
}
unsafe extern "C" fn out_uleb128(mut value: addressT) {
    output_leb128(
        frag_more(sizeof_leb128(value, 0 as libc::c_int) as size_t),
        value,
        0 as libc::c_int,
    );
}
unsafe extern "C" fn out_leb128(mut value: addressT) {
    output_leb128(
        frag_more(sizeof_leb128(value, 1 as libc::c_int) as size_t),
        value,
        1 as libc::c_int,
    );
}
#[inline]
unsafe extern "C" fn out_abbrev(mut name: libc::c_int, mut form: libc::c_int) {
    out_uleb128(name as addressT);
    out_uleb128(form as addressT);
}
unsafe extern "C" fn get_frag_fix(mut frag: *mut fragS, mut seg: segT) -> offsetT {
    let mut fr: *mut frchainS = 0 as *mut frchainS;
    if !((*frag).fr_next).is_null() {
        return (*frag).fr_fix as offsetT;
    }
    fr = (*(bfd_section_userdata(seg as *const asection) as *mut segment_info_type))
        .frchainP;
    while !fr.is_null() {
        if (*fr).frch_last == frag {
            return ((*fr).frch_obstack.next_free as *mut libc::c_void
                as *mut libc::c_char)
                .offset_from(((*frag).fr_literal).as_mut_ptr()) as libc::c_long;
        }
        fr = (*fr).frch_next;
    }
    as_abort(
        b"dwarf2dbg.c\0" as *const u8 as *const libc::c_char,
        1446 as libc::c_int,
        (*::core::mem::transmute::<
            &[u8; 36],
            &[libc::c_char; 36],
        >(b"offsetT get_frag_fix(fragS *, segT)\0"))
            .as_ptr(),
    );
}
unsafe extern "C" fn out_set_addr(mut sym: *mut symbolS) {
    let mut exp: expressionS = expressionS {
        X_add_symbol: 0 as *mut symbolS,
        X_op_symbol: 0 as *mut symbolS,
        X_add_number: 0,
        X_op_X_unsigned_X_extrabit: [0; 2],
        X_md: 0,
    };
    memset(
        &mut exp as *mut expressionS as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<expressionS>() as libc::c_ulong,
    );
    out_opcode(DW_LNS_extended_op as libc::c_int);
    out_uleb128(
        sizeof_address.wrapping_add(1 as libc::c_int as libc::c_uint) as addressT,
    );
    out_opcode(DW_LNE_set_address as libc::c_int);
    exp.set_X_op(O_symbol);
    exp.X_add_symbol = sym;
    exp.X_add_number = 0 as libc::c_int as offsetT;
    emit_expr(&mut exp, sizeof_address);
}
unsafe extern "C" fn scale_addr_delta(mut addr_delta: *mut addressT) {
    static mut printed_this: libc::c_int = 0 as libc::c_int;
    if 1 as libc::c_int > 1 as libc::c_int {
        if (*addr_delta).wrapping_rem(1 as libc::c_int as libc::c_ulong)
            != 0 as libc::c_int as libc::c_ulong && printed_this == 0
        {
            as_bad(
                b"unaligned opcodes detected in executable segment\0" as *const u8
                    as *const libc::c_char,
            );
            printed_this = 1 as libc::c_int;
        }
        *addr_delta = (*addr_delta as libc::c_ulong)
            .wrapping_div(1 as libc::c_int as libc::c_ulong) as addressT as addressT;
    }
}
unsafe extern "C" fn size_inc_line_addr(
    mut line_delta: libc::c_int,
    mut addr_delta: addressT,
) -> libc::c_int {
    let mut tmp: libc::c_uint = 0;
    let mut opcode: libc::c_uint = 0;
    let mut len: libc::c_int = 0 as libc::c_int;
    scale_addr_delta(&mut addr_delta);
    if line_delta == 2147483647 as libc::c_int {
        if addr_delta
            == ((255 as libc::c_int - 13 as libc::c_int) / 14 as libc::c_int)
                as libc::c_ulong
        {
            len = 1 as libc::c_int;
        } else if addr_delta != 0 {
            len = (1 as libc::c_int as libc::c_uint)
                .wrapping_add(sizeof_leb128(addr_delta, 0 as libc::c_int))
                as libc::c_int;
        }
        return len + 3 as libc::c_int;
    }
    tmp = (line_delta - -(5 as libc::c_int)) as libc::c_uint;
    if tmp >= 14 as libc::c_int as libc::c_uint {
        len = (1 as libc::c_int as libc::c_uint)
            .wrapping_add(sizeof_leb128(line_delta as valueT, 1 as libc::c_int))
            as libc::c_int;
        line_delta = 0 as libc::c_int;
        tmp = (0 as libc::c_int - -(5 as libc::c_int)) as libc::c_uint;
    }
    tmp = tmp.wrapping_add(13 as libc::c_int as libc::c_uint);
    if addr_delta
        < (256 as libc::c_int
            + (255 as libc::c_int - 13 as libc::c_int) / 14 as libc::c_int)
            as libc::c_ulong
    {
        opcode = (tmp as libc::c_ulong)
            .wrapping_add(addr_delta.wrapping_mul(14 as libc::c_int as libc::c_ulong))
            as libc::c_uint;
        if opcode <= 255 as libc::c_int as libc::c_uint {
            return len + 1 as libc::c_int;
        }
        opcode = (tmp as libc::c_ulong)
            .wrapping_add(
                addr_delta
                    .wrapping_sub(
                        ((255 as libc::c_int - 13 as libc::c_int) / 14 as libc::c_int)
                            as libc::c_ulong,
                    )
                    .wrapping_mul(14 as libc::c_int as libc::c_ulong),
            ) as libc::c_uint;
        if opcode <= 255 as libc::c_int as libc::c_uint {
            return len + 2 as libc::c_int;
        }
    }
    len = (len as libc::c_uint)
        .wrapping_add(
            (1 as libc::c_int as libc::c_uint)
                .wrapping_add(sizeof_leb128(addr_delta, 0 as libc::c_int)),
        ) as libc::c_int as libc::c_int;
    len += 1 as libc::c_int;
    return len;
}
unsafe extern "C" fn emit_inc_line_addr(
    mut line_delta: libc::c_int,
    mut addr_delta: addressT,
    mut p: *mut libc::c_char,
    mut len: libc::c_int,
) {
    let mut current_block: u64;
    let mut tmp: libc::c_uint = 0;
    let mut opcode: libc::c_uint = 0;
    let mut need_copy: libc::c_int = 0 as libc::c_int;
    let mut end: *mut libc::c_char = p.offset(len as isize);
    if addr_delta as offsetT >= 0 as libc::c_int as libc::c_long {} else {
        as_abort(
            b"dwarf2dbg.c\0" as *const u8 as *const libc::c_char,
            1559 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 52],
                &[libc::c_char; 52],
            >(b"void emit_inc_line_addr(int, addressT, char *, int)\0"))
                .as_ptr(),
        );
    };
    scale_addr_delta(&mut addr_delta);
    if line_delta == 2147483647 as libc::c_int {
        if addr_delta
            == ((255 as libc::c_int - 13 as libc::c_int) / 14 as libc::c_int)
                as libc::c_ulong
        {
            let fresh7 = p;
            p = p.offset(1);
            *fresh7 = DW_LNS_const_add_pc as libc::c_int as libc::c_char;
        } else if addr_delta != 0 {
            let fresh8 = p;
            p = p.offset(1);
            *fresh8 = DW_LNS_advance_pc as libc::c_int as libc::c_char;
            p = p.offset(output_leb128(p, addr_delta, 0 as libc::c_int) as isize);
        }
        let fresh9 = p;
        p = p.offset(1);
        *fresh9 = DW_LNS_extended_op as libc::c_int as libc::c_char;
        let fresh10 = p;
        p = p.offset(1);
        *fresh10 = 1 as libc::c_int as libc::c_char;
        let fresh11 = p;
        p = p.offset(1);
        *fresh11 = DW_LNE_end_sequence as libc::c_int as libc::c_char;
    } else {
        tmp = (line_delta - -(5 as libc::c_int)) as libc::c_uint;
        if tmp >= 14 as libc::c_int as libc::c_uint {
            let fresh12 = p;
            p = p.offset(1);
            *fresh12 = DW_LNS_advance_line as libc::c_int as libc::c_char;
            p = p
                .offset(
                    output_leb128(p, line_delta as valueT, 1 as libc::c_int) as isize,
                );
            line_delta = 0 as libc::c_int;
            tmp = (0 as libc::c_int - -(5 as libc::c_int)) as libc::c_uint;
            need_copy = 1 as libc::c_int;
        }
        if line_delta == 0 as libc::c_int
            && addr_delta == 0 as libc::c_int as libc::c_ulong
        {
            let fresh13 = p;
            p = p.offset(1);
            *fresh13 = DW_LNS_copy as libc::c_int as libc::c_char;
        } else {
            tmp = tmp.wrapping_add(13 as libc::c_int as libc::c_uint);
            if addr_delta
                < (256 as libc::c_int
                    + (255 as libc::c_int - 13 as libc::c_int) / 14 as libc::c_int)
                    as libc::c_ulong
            {
                opcode = (tmp as libc::c_ulong)
                    .wrapping_add(
                        addr_delta.wrapping_mul(14 as libc::c_int as libc::c_ulong),
                    ) as libc::c_uint;
                if opcode <= 255 as libc::c_int as libc::c_uint {
                    let fresh14 = p;
                    p = p.offset(1);
                    *fresh14 = opcode as libc::c_char;
                    current_block = 16337538214244615919;
                } else {
                    opcode = (tmp as libc::c_ulong)
                        .wrapping_add(
                            addr_delta
                                .wrapping_sub(
                                    ((255 as libc::c_int - 13 as libc::c_int)
                                        / 14 as libc::c_int) as libc::c_ulong,
                                )
                                .wrapping_mul(14 as libc::c_int as libc::c_ulong),
                        ) as libc::c_uint;
                    if opcode <= 255 as libc::c_int as libc::c_uint {
                        let fresh15 = p;
                        p = p.offset(1);
                        *fresh15 = DW_LNS_const_add_pc as libc::c_int as libc::c_char;
                        let fresh16 = p;
                        p = p.offset(1);
                        *fresh16 = opcode as libc::c_char;
                        current_block = 16337538214244615919;
                    } else {
                        current_block = 16203760046146113240;
                    }
                }
            } else {
                current_block = 16203760046146113240;
            }
            match current_block {
                16337538214244615919 => {}
                _ => {
                    let fresh17 = p;
                    p = p.offset(1);
                    *fresh17 = DW_LNS_advance_pc as libc::c_int as libc::c_char;
                    p = p
                        .offset(output_leb128(p, addr_delta, 0 as libc::c_int) as isize);
                    if need_copy != 0 {
                        let fresh18 = p;
                        p = p.offset(1);
                        *fresh18 = DW_LNS_copy as libc::c_int as libc::c_char;
                    } else {
                        let fresh19 = p;
                        p = p.offset(1);
                        *fresh19 = tmp as libc::c_char;
                    }
                }
            }
        }
    }
    if p == end {} else {
        as_abort(
            b"dwarf2dbg.c\0" as *const u8 as *const libc::c_char,
            1640 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 52],
                &[libc::c_char; 52],
            >(b"void emit_inc_line_addr(int, addressT, char *, int)\0"))
                .as_ptr(),
        );
    };
}
unsafe extern "C" fn out_inc_line_addr(
    mut line_delta: libc::c_int,
    mut addr_delta: addressT,
) {
    let mut len: libc::c_int = size_inc_line_addr(line_delta, addr_delta);
    emit_inc_line_addr(line_delta, addr_delta, frag_more(len as size_t), len);
}
unsafe extern "C" fn size_fixed_inc_line_addr(
    mut line_delta: libc::c_int,
    mut addr_delta: addressT,
) -> libc::c_int {
    let mut len: libc::c_int = 0 as libc::c_int;
    if line_delta != 2147483647 as libc::c_int {
        len = (1 as libc::c_int as libc::c_uint)
            .wrapping_add(sizeof_leb128(line_delta as valueT, 1 as libc::c_int))
            as libc::c_int;
    }
    if addr_delta > 50000 as libc::c_int as libc::c_ulong {
        len = (len as libc::c_uint)
            .wrapping_add(
                (1 as libc::c_int as libc::c_uint)
                    .wrapping_add(
                        sizeof_leb128(
                            sizeof_address.wrapping_add(1 as libc::c_int as libc::c_uint)
                                as valueT,
                            0 as libc::c_int,
                        ),
                    ),
            ) as libc::c_int as libc::c_int;
        len = (len as libc::c_uint)
            .wrapping_add(
                (1 as libc::c_int as libc::c_uint).wrapping_add(sizeof_address),
            ) as libc::c_int as libc::c_int;
    } else {
        len += 3 as libc::c_int;
    }
    if line_delta == 2147483647 as libc::c_int {
        len += 3 as libc::c_int;
    } else {
        len += 1 as libc::c_int;
    }
    return len;
}
unsafe extern "C" fn emit_fixed_inc_line_addr(
    mut line_delta: libc::c_int,
    mut addr_delta: addressT,
    mut frag: *mut fragS,
    mut p: *mut libc::c_char,
    mut len: libc::c_int,
) {
    let mut pexp: *mut expressionS = 0 as *mut expressionS;
    let mut end: *mut libc::c_char = p.offset(len as isize);
    if addr_delta as offsetT >= 0 as libc::c_int as libc::c_long {} else {
        as_abort(
            b"dwarf2dbg.c\0" as *const u8 as *const libc::c_char,
            1698 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 67],
                &[libc::c_char; 67],
            >(b"void emit_fixed_inc_line_addr(int, addressT, fragS *, char *, int)\0"))
                .as_ptr(),
        );
    };
    if len == size_fixed_inc_line_addr(line_delta, addr_delta) {} else {
        as_abort(
            b"dwarf2dbg.c\0" as *const u8 as *const libc::c_char,
            1701 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 67],
                &[libc::c_char; 67],
            >(b"void emit_fixed_inc_line_addr(int, addressT, fragS *, char *, int)\0"))
                .as_ptr(),
        );
    };
    if line_delta != 2147483647 as libc::c_int {
        let fresh20 = p;
        p = p.offset(1);
        *fresh20 = DW_LNS_advance_line as libc::c_int as libc::c_char;
        p = p.offset(output_leb128(p, line_delta as valueT, 1 as libc::c_int) as isize);
    }
    pexp = symbol_get_value_expression((*frag).fr_symbol);
    if addr_delta > 50000 as libc::c_int as libc::c_ulong {
        let mut to_sym: *mut symbolS = 0 as *mut symbolS;
        let mut exp: expressionS = expressionS {
            X_add_symbol: 0 as *mut symbolS,
            X_op_symbol: 0 as *mut symbolS,
            X_add_number: 0,
            X_op_X_unsigned_X_extrabit: [0; 2],
            X_md: 0,
        };
        memset(
            &mut exp as *mut expressionS as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<expressionS>() as libc::c_ulong,
        );
        if (*pexp).X_op() as libc::c_int == O_subtract as libc::c_int {} else {
            as_abort(
                b"dwarf2dbg.c\0" as *const u8 as *const libc::c_char,
                1723 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 67],
                    &[libc::c_char; 67],
                >(
                    b"void emit_fixed_inc_line_addr(int, addressT, fragS *, char *, int)\0",
                ))
                    .as_ptr(),
            );
        };
        to_sym = (*pexp).X_add_symbol;
        let fresh21 = p;
        p = p.offset(1);
        *fresh21 = DW_LNS_extended_op as libc::c_int as libc::c_char;
        p = p
            .offset(
                output_leb128(
                    p,
                    sizeof_address.wrapping_add(1 as libc::c_int as libc::c_uint)
                        as valueT,
                    0 as libc::c_int,
                ) as isize,
            );
        let fresh22 = p;
        p = p.offset(1);
        *fresh22 = DW_LNE_set_address as libc::c_int as libc::c_char;
        exp.set_X_op(O_symbol);
        exp.X_add_symbol = to_sym;
        exp.X_add_number = 0 as libc::c_int as offsetT;
        emit_expr_fix(&mut exp, sizeof_address, frag, p, BFD_RELOC_NONE);
        p = p.offset(sizeof_address as isize);
    } else {
        let fresh23 = p;
        p = p.offset(1);
        *fresh23 = DW_LNS_fixed_advance_pc as libc::c_int as libc::c_char;
        emit_expr_fix(pexp, 2 as libc::c_int as libc::c_uint, frag, p, BFD_RELOC_NONE);
        p = p.offset(2 as libc::c_int as isize);
    }
    if line_delta == 2147483647 as libc::c_int {
        let fresh24 = p;
        p = p.offset(1);
        *fresh24 = DW_LNS_extended_op as libc::c_int as libc::c_char;
        let fresh25 = p;
        p = p.offset(1);
        *fresh25 = 1 as libc::c_int as libc::c_char;
        let fresh26 = p;
        p = p.offset(1);
        *fresh26 = DW_LNE_end_sequence as libc::c_int as libc::c_char;
    } else {
        let fresh27 = p;
        p = p.offset(1);
        *fresh27 = DW_LNS_copy as libc::c_int as libc::c_char;
    }
    if p == end {} else {
        as_abort(
            b"dwarf2dbg.c\0" as *const u8 as *const libc::c_char,
            1751 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 67],
                &[libc::c_char; 67],
            >(b"void emit_fixed_inc_line_addr(int, addressT, fragS *, char *, int)\0"))
                .as_ptr(),
        );
    };
}
unsafe extern "C" fn relax_inc_line_addr(
    mut line_delta: libc::c_int,
    mut to_sym: *mut symbolS,
    mut from_sym: *mut symbolS,
) {
    let mut exp: expressionS = expressionS {
        X_add_symbol: 0 as *mut symbolS,
        X_op_symbol: 0 as *mut symbolS,
        X_add_number: 0,
        X_op_X_unsigned_X_extrabit: [0; 2],
        X_md: 0,
    };
    let mut max_chars: libc::c_int = 0;
    memset(
        &mut exp as *mut expressionS as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<expressionS>() as libc::c_ulong,
    );
    exp.set_X_op(O_subtract);
    exp.X_add_symbol = to_sym;
    exp.X_op_symbol = from_sym;
    exp.X_add_number = 0 as libc::c_int as offsetT;
    if linkrelax != 0 {
        max_chars = size_fixed_inc_line_addr(
            line_delta,
            -(1 as libc::c_int) as addressT,
        );
    } else {
        max_chars = size_inc_line_addr(line_delta, -(1 as libc::c_int) as addressT);
    }
    frag_var(
        rs_dwarf2dbg,
        max_chars as size_t,
        max_chars as size_t,
        1 as libc::c_int as relax_substateT,
        make_expr_symbol(&mut exp),
        line_delta as offsetT,
        0 as *mut libc::c_char,
    );
}
unsafe extern "C" fn process_entries(mut seg: segT, mut e: *mut line_entry) {
    let mut filenum: libc::c_uint = 1 as libc::c_int as libc::c_uint;
    let mut line: libc::c_uint = 1 as libc::c_int as libc::c_uint;
    let mut column: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut isa: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut flags: libc::c_uint = (if 1 as libc::c_int != 0 {
        (1 as libc::c_int) << 0 as libc::c_int
    } else {
        0 as libc::c_int
    }) as libc::c_uint;
    let mut last_frag: *mut fragS = 0 as *mut fragS;
    let mut frag: *mut fragS = 0 as *mut fragS;
    let mut last_frag_ofs: addressT = 0 as libc::c_int as addressT;
    let mut frag_ofs: addressT = 0;
    let mut last_lab: *mut symbolS = 0 as *mut symbolS;
    let mut lab: *mut symbolS = 0 as *mut symbolS;
    let mut next: *mut line_entry = 0 as *mut line_entry;
    if flag_dwarf_sections {
        let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut sec_name: *const libc::c_char = 0 as *const libc::c_char;
        sec_name = bfd_section_name(seg as *const asection);
        if strcmp(sec_name, b".text\0" as *const u8 as *const libc::c_char)
            != 0 as libc::c_int
        {
            name = concat(
                b".debug_line\0" as *const u8 as *const libc::c_char,
                sec_name,
                0 as *mut libc::c_void as *mut libc::c_char,
            );
            subseg_set(subseg_get(name, 0 as libc::c_int), 0 as libc::c_int);
        } else {
            subseg_set(
                subseg_get(
                    b".debug_line\0" as *const u8 as *const libc::c_char,
                    0 as libc::c_int,
                ),
                0 as libc::c_int,
            );
        }
    }
    loop {
        let mut line_delta: libc::c_int = 0;
        if filenum != (*e).loc.filenum {
            filenum = (*e).loc.filenum;
            out_opcode(DW_LNS_set_file as libc::c_int);
            out_uleb128(filenum as addressT);
        }
        if column != (*e).loc.column {
            column = (*e).loc.column;
            out_opcode(DW_LNS_set_column as libc::c_int);
            out_uleb128(column as addressT);
        }
        if (*e).loc.discriminator != 0 as libc::c_int as libc::c_uint {
            out_opcode(DW_LNS_extended_op as libc::c_int);
            out_leb128(
                (1 as libc::c_int as libc::c_uint)
                    .wrapping_add(
                        sizeof_leb128((*e).loc.discriminator as valueT, 0 as libc::c_int),
                    ) as addressT,
            );
            out_opcode(DW_LNE_set_discriminator as libc::c_int);
            out_uleb128((*e).loc.discriminator as addressT);
        }
        if isa != (*e).loc.isa {
            isa = (*e).loc.isa;
            out_opcode(DW_LNS_set_isa as libc::c_int);
            out_uleb128(isa as addressT);
        }
        if ((*e).loc.flags ^ flags)
            & ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint != 0
        {
            flags = (*e).loc.flags;
            out_opcode(DW_LNS_negate_stmt as libc::c_int);
        }
        if (*e).loc.flags & ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uint != 0
        {
            out_opcode(DW_LNS_set_basic_block as libc::c_int);
        }
        if (*e).loc.flags & ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uint != 0
        {
            out_opcode(DW_LNS_set_prologue_end as libc::c_int);
        }
        if (*e).loc.flags & ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_uint != 0
        {
            out_opcode(DW_LNS_set_epilogue_begin as libc::c_int);
        }
        line_delta = ((*e).loc.line).wrapping_sub(line) as libc::c_int;
        lab = (*e).label;
        frag = symbol_get_frag(lab);
        frag_ofs = S_GET_VALUE(lab);
        if last_frag.is_null()
            || (*e).loc.view == force_reset_view && !force_reset_view.is_null()
                && (if frag == last_frag {
                    (last_frag_ofs == frag_ofs) as libc::c_int
                } else {
                    (frag_ofs == 0 as libc::c_int as libc::c_ulong
                        && last_frag_ofs as offsetT >= get_frag_fix(last_frag, seg))
                        as libc::c_int
                }) != 0
        {
            out_set_addr(lab);
            out_inc_line_addr(line_delta, 0 as libc::c_int as addressT);
        } else if frag == last_frag && linkrelax == 0 {
            out_inc_line_addr(line_delta, frag_ofs.wrapping_sub(last_frag_ofs));
        } else {
            relax_inc_line_addr(line_delta, lab, last_lab);
        }
        line = (*e).loc.line;
        last_lab = lab;
        last_frag = frag;
        last_frag_ofs = frag_ofs;
        next = (*e).next;
        free(e as *mut libc::c_void);
        e = next;
        if e.is_null() {
            break;
        }
    }
    frag = last_frag_for_seg(seg);
    frag_ofs = get_frag_fix(frag, seg) as addressT;
    if frag == last_frag && linkrelax == 0 {
        out_inc_line_addr(
            2147483647 as libc::c_int,
            frag_ofs.wrapping_sub(last_frag_ofs),
        );
    } else {
        lab = symbol_temp_new(seg, frag, frag_ofs);
        relax_inc_line_addr(2147483647 as libc::c_int, lab, last_lab);
    };
}
unsafe extern "C" fn add_line_strp(
    mut line_str_seg: segT,
    mut str: *const libc::c_char,
) -> *mut symbolS {
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut size: size_t = 0;
    let mut sym: *mut symbolS = 0 as *mut symbolS;
    subseg_set(line_str_seg, 0 as libc::c_int);
    sym = symbol_temp_new_now_octets();
    size = (strlen(str)).wrapping_add(1 as libc::c_int as libc::c_ulong);
    cp = frag_more(size);
    memcpy(cp as *mut libc::c_void, str as *const libc::c_void, size);
    return sym;
}
unsafe extern "C" fn out_dir_and_file_list(
    mut line_seg: segT,
    mut sizeof_offset: libc::c_int,
) {
    let mut size: size_t = 0;
    let mut dir: *const libc::c_char = 0 as *const libc::c_char;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_uint = 0;
    let mut emit_md5: bool = 0 as libc::c_int != 0;
    let mut emit_timestamps: bool = 1 as libc::c_int != 0;
    let mut emit_filesize: bool = 1 as libc::c_int != 0;
    let mut line_str_seg: segT = 0 as segT;
    let mut line_strp: *mut symbolS = 0 as *mut symbolS;
    if (if dwarf_level > 3 as libc::c_int as libc::c_uint {
        dwarf_level
    } else {
        3 as libc::c_int as libc::c_uint
    }) >= 5 as libc::c_int as libc::c_uint
    {
        out_byte(1 as libc::c_int);
        out_uleb128(DW_LNCT_path as libc::c_int as addressT);
        out_uleb128(DW_FORM_line_strp as libc::c_int as addressT);
        if dirs_in_use == 0 as libc::c_int as libc::c_uint
            && files_in_use > 0 as libc::c_int as libc::c_uint
        {
            out_uleb128(1 as libc::c_int as addressT);
        } else {
            out_uleb128(dirs_in_use as addressT);
        }
    }
    if (if dwarf_level > 3 as libc::c_int as libc::c_uint {
        dwarf_level
    } else {
        3 as libc::c_int as libc::c_uint
    }) >= 5 as libc::c_int as libc::c_uint
        && (dirs_in_use > 0 as libc::c_int as libc::c_uint
            || files_in_use > 0 as libc::c_int as libc::c_uint)
    {
        line_str_seg = subseg_new(
            b".debug_line_str\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
        bfd_set_section_flags(
            line_str_seg,
            (0x8 as libc::c_int | 0x2000 as libc::c_int
                | (if 1 as libc::c_int != 0 {
                    0x40000000 as libc::c_int
                } else {
                    0 as libc::c_int
                }) | 0x800000 as libc::c_int | 0x1000000 as libc::c_int) as flagword,
        );
        (*line_str_seg).entsize = 1 as libc::c_int as libc::c_uint;
        if dirs_in_use > 0 as libc::c_int as libc::c_uint && !dirs.is_null()
            && !(*dirs.offset(0 as libc::c_int as isize)).is_null()
        {
            dir = remap_debug_filename(*dirs.offset(0 as libc::c_int as isize));
        } else if dirs_in_use > 1 as libc::c_int as libc::c_uint && !dirs.is_null()
            && !(*dirs.offset(1 as libc::c_int as isize)).is_null()
        {
            dir = remap_debug_filename(*dirs.offset(1 as libc::c_int as isize));
        } else {
            dir = remap_debug_filename(getpwd());
        }
        line_strp = add_line_strp(line_str_seg, dir);
        subseg_set(line_seg, 0 as libc::c_int);
        generic_dwarf2_emit_offset(line_strp, sizeof_offset as libc::c_uint);
    }
    i = 1 as libc::c_int as libc::c_uint;
    while i < dirs_in_use {
        dir = remap_debug_filename(*dirs.offset(i as isize));
        if (if dwarf_level > 3 as libc::c_int as libc::c_uint {
            dwarf_level
        } else {
            3 as libc::c_int as libc::c_uint
        }) < 5 as libc::c_int as libc::c_uint
        {
            size = (strlen(dir)).wrapping_add(1 as libc::c_int as libc::c_ulong);
            cp = frag_more(size);
            memcpy(cp as *mut libc::c_void, dir as *const libc::c_void, size);
        } else {
            line_strp = add_line_strp(line_str_seg, dir);
            subseg_set(line_seg, 0 as libc::c_int);
            generic_dwarf2_emit_offset(line_strp, sizeof_offset as libc::c_uint);
        }
        i = i.wrapping_add(1);
        i;
    }
    if (if dwarf_level > 3 as libc::c_int as libc::c_uint {
        dwarf_level
    } else {
        3 as libc::c_int as libc::c_uint
    }) < 5 as libc::c_int as libc::c_uint
    {
        out_byte('\0' as i32);
    }
    if (if dwarf_level > 3 as libc::c_int as libc::c_uint {
        dwarf_level
    } else {
        3 as libc::c_int as libc::c_uint
    }) >= 5 as libc::c_int as libc::c_uint
    {
        let mut columns: libc::c_uint = 4 as libc::c_int as libc::c_uint;
        if -(1 as libc::c_int) as libc::c_ulong == (1 as libc::c_ulong).wrapping_neg() {
            emit_timestamps = 0 as libc::c_int != 0;
            columns = columns.wrapping_sub(1);
            columns;
        }
        if -(1 as libc::c_int) == -(1 as libc::c_int) {
            emit_filesize = 0 as libc::c_int != 0;
            columns = columns.wrapping_sub(1);
            columns;
        }
        i = 0 as libc::c_int as libc::c_uint;
        while i < files_in_use {
            if (*files.offset(i as isize)).md5[0 as libc::c_int as usize] as libc::c_int
                != 0 as libc::c_int
            {
                break;
            }
            i = i.wrapping_add(1);
            i;
        }
        if i < files_in_use {
            emit_md5 = 1 as libc::c_int != 0;
            columns = columns.wrapping_add(1);
            columns;
        }
        out_byte(columns as libc::c_int);
        out_uleb128(DW_LNCT_path as libc::c_int as addressT);
        out_uleb128(DW_FORM_line_strp as libc::c_int as addressT);
        out_uleb128(DW_LNCT_directory_index as libc::c_int as addressT);
        out_uleb128(DW_FORM_udata as libc::c_int as addressT);
        if emit_timestamps {
            out_uleb128(DW_LNCT_timestamp as libc::c_int as addressT);
            out_uleb128(DW_FORM_udata as libc::c_int as addressT);
        }
        if emit_filesize {
            out_uleb128(DW_LNCT_size as libc::c_int as addressT);
            out_uleb128(DW_FORM_udata as libc::c_int as addressT);
        }
        if emit_md5 {
            out_uleb128(DW_LNCT_MD5 as libc::c_int as addressT);
            out_uleb128(DW_FORM_data16 as libc::c_int as addressT);
        }
        out_uleb128(files_in_use as addressT);
    }
    let mut current_block_102: u64;
    i = (if (if dwarf_level > 3 as libc::c_int as libc::c_uint {
        dwarf_level
    } else {
        3 as libc::c_int as libc::c_uint
    }) > 4 as libc::c_int as libc::c_uint
    {
        0 as libc::c_int
    } else {
        1 as libc::c_int
    }) as libc::c_uint;
    while i < files_in_use {
        let mut fullfilename: *const libc::c_char = 0 as *const libc::c_char;
        if ((*files.offset(i as isize)).filename).is_null() {
            if (if dwarf_level > 3 as libc::c_int as libc::c_uint {
                dwarf_level
            } else {
                3 as libc::c_int as libc::c_uint
            }) >= 5 as libc::c_int as libc::c_uint
                && i == 0 as libc::c_int as libc::c_uint
                && files_in_use >= 1 as libc::c_int as libc::c_uint
            {
                let ref mut fresh28 = (*files.offset(0 as libc::c_int as isize))
                    .filename;
                *fresh28 = (*files.offset(1 as libc::c_int as isize)).filename;
            } else {
                let ref mut fresh29 = (*files.offset(i as isize)).filename;
                *fresh29 = b"\0" as *const u8 as *const libc::c_char;
            }
            if (if dwarf_level > 3 as libc::c_int as libc::c_uint {
                dwarf_level
            } else {
                3 as libc::c_int as libc::c_uint
            }) < 5 as libc::c_int as libc::c_uint
                || i != 0 as libc::c_int as libc::c_uint
            {
                as_bad(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"unassigned file number %ld\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    i as libc::c_long,
                );
                current_block_102 = 6560072651652764009;
            } else {
                current_block_102 = 981995395831942902;
            }
        } else {
            current_block_102 = 981995395831942902;
        }
        match current_block_102 {
            981995395831942902 => {
                fullfilename = (*files.offset(i as isize)).filename;
                if (if dwarf_level > 3 as libc::c_int as libc::c_uint {
                    dwarf_level
                } else {
                    3 as libc::c_int as libc::c_uint
                }) < 5 as libc::c_int as libc::c_uint
                {
                    size = (strlen(fullfilename))
                        .wrapping_add(1 as libc::c_int as libc::c_ulong);
                    cp = frag_more(size);
                    memcpy(
                        cp as *mut libc::c_void,
                        fullfilename as *const libc::c_void,
                        size,
                    );
                } else {
                    line_strp = add_line_strp(line_str_seg, fullfilename);
                    subseg_set(line_seg, 0 as libc::c_int);
                    generic_dwarf2_emit_offset(line_strp, sizeof_offset as libc::c_uint);
                }
                out_uleb128((*files.offset(i as isize)).dir as addressT);
                if emit_timestamps {
                    let mut timestamp: offsetT = 0;
                    timestamp = -(1 as libc::c_int) as offsetT;
                    if timestamp == -(1 as libc::c_int) as libc::c_long {
                        timestamp = 0 as libc::c_int as offsetT;
                    }
                    out_uleb128(timestamp as addressT);
                }
                if emit_filesize {
                    let mut filesize: offsetT = 0;
                    filesize = -(1 as libc::c_int) as offsetT;
                    if filesize == -(1 as libc::c_int) as libc::c_long {
                        filesize = 0 as libc::c_int as offsetT;
                    }
                    out_uleb128(filesize as addressT);
                }
                if emit_md5 {
                    let mut b: libc::c_int = 0;
                    b = 0 as libc::c_int;
                    while b < 16 as libc::c_int {
                        out_byte(
                            (*files.offset(i as isize)).md5[b as usize] as libc::c_int,
                        );
                        b += 1;
                        b;
                    }
                }
            }
            _ => {}
        }
        i = i.wrapping_add(1);
        i;
    }
    if (if dwarf_level > 3 as libc::c_int as libc::c_uint {
        dwarf_level
    } else {
        3 as libc::c_int as libc::c_uint
    }) < 5 as libc::c_int as libc::c_uint
    {
        out_byte(0 as libc::c_int);
    }
}
unsafe extern "C" fn out_header(
    mut sec: *mut asection,
    mut exp: *mut expressionS,
) -> libc::c_int {
    let mut start_sym: *mut symbolS = 0 as *mut symbolS;
    let mut end_sym: *mut symbolS = 0 as *mut symbolS;
    subseg_set(sec, 0 as libc::c_int);
    if flag_dwarf_sections {
        frag_now_fix();
        start_sym = symbol_make(
            b".Ldebug_line_start\0" as *const u8 as *const libc::c_char,
        );
        end_sym = symbol_make(b".Ldebug_line_end\0" as *const u8 as *const libc::c_char);
        symbol_set_value_now(start_sym);
    } else {
        start_sym = symbol_temp_new_now_octets();
        end_sym = symbol_temp_make();
    }
    (*exp).set_X_op(O_subtract);
    (*exp).X_add_symbol = end_sym;
    (*exp).X_op_symbol = start_sym;
    match dwarf2_format_32bit as libc::c_int {
        0 => {
            (*exp).X_add_number = -(4 as libc::c_int) as offsetT;
            emit_expr(exp, 4 as libc::c_int as libc::c_uint);
            return 4 as libc::c_int;
        }
        1 => {
            (*exp).X_add_number = -(12 as libc::c_int) as offsetT;
            out_four(-(1 as libc::c_int));
            emit_expr(exp, 8 as libc::c_int as libc::c_uint);
            return 8 as libc::c_int;
        }
        2 => {
            (*exp).X_add_number = -(8 as libc::c_int) as offsetT;
            emit_expr(exp, 8 as libc::c_int as libc::c_uint);
            return 8 as libc::c_int;
        }
        _ => {}
    }
    as_fatal(
        dcgettext(
            0 as *const libc::c_char,
            b"internal error: unknown dwarf2 format\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
}
unsafe extern "C" fn out_debug_line(mut line_seg: segT) {
    let mut exp: expressionS = expressionS {
        X_add_symbol: 0 as *mut symbolS,
        X_op_symbol: 0 as *mut symbolS,
        X_add_number: 0,
        X_op_X_unsigned_X_extrabit: [0; 2],
        X_md: 0,
    };
    let mut prologue_start: *mut symbolS = 0 as *mut symbolS;
    let mut prologue_end: *mut symbolS = 0 as *mut symbolS;
    let mut line_end: *mut symbolS = 0 as *mut symbolS;
    let mut s: *mut line_seg = 0 as *mut line_seg;
    let mut sizeof_offset: libc::c_int = 0;
    memset(
        &mut exp as *mut expressionS as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<expressionS>() as libc::c_ulong,
    );
    sizeof_offset = out_header(line_seg, &mut exp);
    line_end = exp.X_add_symbol;
    out_two(
        (if dwarf_level > 3 as libc::c_int as libc::c_uint {
            dwarf_level
        } else {
            3 as libc::c_int as libc::c_uint
        }) as libc::c_int,
    );
    if (if dwarf_level > 3 as libc::c_int as libc::c_uint {
        dwarf_level
    } else {
        3 as libc::c_int as libc::c_uint
    }) >= 5 as libc::c_int as libc::c_uint
    {
        out_byte(sizeof_address as libc::c_int);
        out_byte(0 as libc::c_int);
    }
    prologue_start = symbol_temp_make();
    prologue_end = symbol_temp_make();
    exp.set_X_op(O_subtract);
    exp.X_add_symbol = prologue_end;
    exp.X_op_symbol = prologue_start;
    exp.X_add_number = 0 as libc::c_int as offsetT;
    emit_expr(&mut exp, sizeof_offset as libc::c_uint);
    symbol_set_value_now(prologue_start);
    out_byte(1 as libc::c_int);
    if (if dwarf_level > 3 as libc::c_int as libc::c_uint {
        dwarf_level
    } else {
        3 as libc::c_int as libc::c_uint
    }) >= 4 as libc::c_int as libc::c_uint
    {
        out_byte(1 as libc::c_int);
    }
    out_byte(1 as libc::c_int);
    out_byte(-(5 as libc::c_int));
    out_byte(14 as libc::c_int);
    out_byte(13 as libc::c_int);
    out_byte(0 as libc::c_int);
    out_byte(1 as libc::c_int);
    out_byte(1 as libc::c_int);
    out_byte(1 as libc::c_int);
    out_byte(1 as libc::c_int);
    out_byte(0 as libc::c_int);
    out_byte(0 as libc::c_int);
    out_byte(0 as libc::c_int);
    out_byte(1 as libc::c_int);
    out_byte(0 as libc::c_int);
    out_byte(0 as libc::c_int);
    out_byte(1 as libc::c_int);
    if 13 as libc::c_int == 13 as libc::c_int {} else {
        as_abort(
            b"dwarf2dbg.c\0" as *const u8 as *const libc::c_char,
            2368 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 26],
                &[libc::c_char; 26],
            >(b"void out_debug_line(segT)\0"))
                .as_ptr(),
        );
    };
    out_dir_and_file_list(line_seg, sizeof_offset);
    symbol_set_value_now(prologue_end);
    s = all_segs;
    while !s.is_null() {
        if (*s).seg
            != &mut *_bfd_std_section.as_mut_ptr().offset(2 as libc::c_int as isize)
                as *mut asection
            && (*s).seg
                != &mut *_bfd_std_section.as_mut_ptr().offset(1 as libc::c_int as isize)
                    as *mut asection && (*s).seg != reg_section
            && (*s).seg != expr_section
        {
            process_entries((*s).seg, (*(*s).head).head);
        }
        s = (*s).next;
    }
    if flag_dwarf_sections {
        subseg_set(
            subseg_get(
                b".debug_line_end\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int,
            ),
            0 as libc::c_int,
        );
    }
    symbol_set_value_now(line_end);
}
unsafe extern "C" fn out_debug_ranges(
    mut ranges_seg: segT,
    mut ranges_sym: *mut *mut symbolS,
) {
    let mut addr_size: libc::c_uint = sizeof_address;
    let mut s: *mut line_seg = 0 as *mut line_seg;
    let mut exp: expressionS = expressionS {
        X_add_symbol: 0 as *mut symbolS,
        X_op_symbol: 0 as *mut symbolS,
        X_add_number: 0,
        X_op_X_unsigned_X_extrabit: [0; 2],
        X_md: 0,
    };
    let mut i: libc::c_uint = 0;
    memset(
        &mut exp as *mut expressionS as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<expressionS>() as libc::c_ulong,
    );
    subseg_set(ranges_seg, 0 as libc::c_int);
    *ranges_sym = symbol_temp_new_now_octets();
    i = 0 as libc::c_int as libc::c_uint;
    while i < addr_size {
        out_byte(0xff as libc::c_int);
        i = i.wrapping_add(1);
        i;
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < addr_size {
        out_byte(0 as libc::c_int);
        i = i.wrapping_add(1);
        i;
    }
    s = all_segs;
    while !s.is_null() {
        let mut frag: *mut fragS = 0 as *mut fragS;
        let mut beg: *mut symbolS = 0 as *mut symbolS;
        let mut end: *mut symbolS = 0 as *mut symbolS;
        frag = first_frag_for_seg((*s).seg);
        beg = symbol_temp_new((*s).seg, frag, 0 as libc::c_int as valueT);
        (*s).text_start = beg;
        frag = last_frag_for_seg((*s).seg);
        end = symbol_temp_new((*s).seg, frag, get_frag_fix(frag, (*s).seg) as valueT);
        (*s).text_end = end;
        exp.set_X_op(O_symbol);
        exp.X_add_symbol = beg;
        exp.X_add_number = 0 as libc::c_int as offsetT;
        emit_expr(&mut exp, addr_size);
        exp.set_X_op(O_symbol);
        exp.X_add_symbol = end;
        exp.X_add_number = 0 as libc::c_int as offsetT;
        emit_expr(&mut exp, addr_size);
        s = (*s).next;
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < addr_size {
        out_byte(0 as libc::c_int);
        i = i.wrapping_add(1);
        i;
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < addr_size {
        out_byte(0 as libc::c_int);
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn out_debug_rnglists(
    mut ranges_seg: segT,
    mut ranges_sym: *mut *mut symbolS,
) {
    let mut exp: expressionS = expressionS {
        X_add_symbol: 0 as *mut symbolS,
        X_op_symbol: 0 as *mut symbolS,
        X_add_number: 0,
        X_op_X_unsigned_X_extrabit: [0; 2],
        X_md: 0,
    };
    let mut ranges_end: *mut symbolS = 0 as *mut symbolS;
    let mut s: *mut line_seg = 0 as *mut line_seg;
    memset(
        &mut exp as *mut expressionS as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<expressionS>() as libc::c_ulong,
    );
    out_header(ranges_seg, &mut exp);
    ranges_end = exp.X_add_symbol;
    out_two(5 as libc::c_int);
    out_byte(sizeof_address as libc::c_int);
    out_byte(0 as libc::c_int);
    out_four(0 as libc::c_int);
    *ranges_sym = symbol_temp_new_now_octets();
    s = all_segs;
    while !s.is_null() {
        let mut frag: *mut fragS = 0 as *mut fragS;
        let mut beg: *mut symbolS = 0 as *mut symbolS;
        let mut end: *mut symbolS = 0 as *mut symbolS;
        out_byte(DW_RLE_start_length as libc::c_int);
        frag = first_frag_for_seg((*s).seg);
        beg = symbol_temp_new((*s).seg, frag, 0 as libc::c_int as valueT);
        (*s).text_start = beg;
        frag = last_frag_for_seg((*s).seg);
        end = symbol_temp_new((*s).seg, frag, get_frag_fix(frag, (*s).seg) as valueT);
        (*s).text_end = end;
        exp.set_X_op(O_symbol);
        exp.X_add_symbol = beg;
        exp.X_add_number = 0 as libc::c_int as offsetT;
        emit_expr(&mut exp, sizeof_address);
        exp.set_X_op(O_symbol);
        exp.X_add_symbol = end;
        exp.X_add_number = 0 as libc::c_int as offsetT;
        emit_leb128_expr(&mut exp, 0 as libc::c_int);
        s = (*s).next;
    }
    out_byte(DW_RLE_end_of_list as libc::c_int);
    symbol_set_value_now(ranges_end);
}
unsafe extern "C" fn out_debug_aranges(mut aranges_seg: segT, mut info_seg: segT) {
    let mut addr_size: libc::c_uint = sizeof_address;
    let mut size: offsetT = 0;
    let mut s: *mut line_seg = 0 as *mut line_seg;
    let mut exp: expressionS = expressionS {
        X_add_symbol: 0 as *mut symbolS,
        X_op_symbol: 0 as *mut symbolS,
        X_add_number: 0,
        X_op_X_unsigned_X_extrabit: [0; 2],
        X_md: 0,
    };
    let mut aranges_end: *mut symbolS = 0 as *mut symbolS;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sizeof_offset: libc::c_int = 0;
    memset(
        &mut exp as *mut expressionS as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<expressionS>() as libc::c_ulong,
    );
    sizeof_offset = out_header(aranges_seg, &mut exp);
    aranges_end = exp.X_add_symbol;
    size = -exp.X_add_number;
    out_two(2 as libc::c_int);
    size += 2 as libc::c_int as libc::c_long;
    generic_dwarf2_emit_offset(section_symbol(info_seg), sizeof_offset as libc::c_uint);
    size += sizeof_offset as libc::c_long;
    out_byte(addr_size as libc::c_int);
    size += 1;
    size;
    out_byte(0 as libc::c_int);
    size += 1;
    size;
    loop {
        let fresh30 = size;
        size = size + 1;
        if !(fresh30
            % (2 as libc::c_int as libc::c_uint).wrapping_mul(addr_size) as libc::c_long
            > 0 as libc::c_int as libc::c_long)
        {
            break;
        }
        out_byte(0 as libc::c_int);
    }
    s = all_segs;
    while !s.is_null() {
        let mut frag: *mut fragS = 0 as *mut fragS;
        let mut beg: *mut symbolS = 0 as *mut symbolS;
        let mut end: *mut symbolS = 0 as *mut symbolS;
        frag = first_frag_for_seg((*s).seg);
        beg = symbol_temp_new((*s).seg, frag, 0 as libc::c_int as valueT);
        (*s).text_start = beg;
        frag = last_frag_for_seg((*s).seg);
        end = symbol_temp_new((*s).seg, frag, get_frag_fix(frag, (*s).seg) as valueT);
        (*s).text_end = end;
        exp.set_X_op(O_symbol);
        exp.X_add_symbol = beg;
        exp.X_add_number = 0 as libc::c_int as offsetT;
        emit_expr(&mut exp, addr_size);
        exp.set_X_op(O_subtract);
        exp.X_add_symbol = end;
        exp.X_op_symbol = beg;
        exp.X_add_number = 0 as libc::c_int as offsetT;
        emit_expr(&mut exp, addr_size);
        s = (*s).next;
    }
    p = frag_more((2 as libc::c_int as libc::c_uint).wrapping_mul(addr_size) as size_t);
    number_to_chars_littleendian(
        p,
        0 as libc::c_int as valueT,
        addr_size as libc::c_int,
    );
    number_to_chars_littleendian(
        p.offset(addr_size as isize),
        0 as libc::c_int as valueT,
        addr_size as libc::c_int,
    );
    symbol_set_value_now(aranges_end);
}
unsafe extern "C" fn out_debug_abbrev(
    mut abbrev_seg: segT,
    mut info_seg: segT,
    mut line_seg: segT,
) {
    let mut secoff_form: libc::c_int = 0;
    subseg_set(abbrev_seg, 0 as libc::c_int);
    out_uleb128(1 as libc::c_int as addressT);
    out_uleb128(DW_TAG_compile_unit as libc::c_int as addressT);
    out_byte(0 as libc::c_int);
    if dwarf_level < 4 as libc::c_int as libc::c_uint {
        if dwarf2_format_32bit as libc::c_int == dwarf2_format_32bit as libc::c_int {
            secoff_form = DW_FORM_data4 as libc::c_int;
        } else {
            secoff_form = DW_FORM_data8 as libc::c_int;
        }
    } else {
        secoff_form = DW_FORM_sec_offset as libc::c_int;
    }
    out_abbrev(DW_AT_stmt_list as libc::c_int, secoff_form);
    if ((*all_segs).next).is_null() {
        out_abbrev(DW_AT_low_pc as libc::c_int, DW_FORM_addr as libc::c_int);
        if dwarf_level < 4 as libc::c_int as libc::c_uint {
            out_abbrev(DW_AT_high_pc as libc::c_int, DW_FORM_addr as libc::c_int);
        } else {
            out_abbrev(DW_AT_high_pc as libc::c_int, DW_FORM_udata as libc::c_int);
        }
    } else {
        out_abbrev(DW_AT_ranges as libc::c_int, secoff_form);
    }
    out_abbrev(DW_AT_name as libc::c_int, DW_FORM_strp as libc::c_int);
    out_abbrev(DW_AT_comp_dir as libc::c_int, DW_FORM_strp as libc::c_int);
    out_abbrev(DW_AT_producer as libc::c_int, DW_FORM_strp as libc::c_int);
    out_abbrev(DW_AT_language as libc::c_int, DW_FORM_data2 as libc::c_int);
    out_abbrev(0 as libc::c_int, 0 as libc::c_int);
    out_byte(0 as libc::c_int);
}
unsafe extern "C" fn out_debug_info(
    mut info_seg: segT,
    mut abbrev_seg: segT,
    mut line_seg: segT,
    mut ranges_sym: *mut symbolS,
    mut name_sym: *mut symbolS,
    mut comp_dir_sym: *mut symbolS,
    mut producer_sym: *mut symbolS,
) {
    let mut exp: expressionS = expressionS {
        X_add_symbol: 0 as *mut symbolS,
        X_op_symbol: 0 as *mut symbolS,
        X_add_number: 0,
        X_op_X_unsigned_X_extrabit: [0; 2],
        X_md: 0,
    };
    let mut info_end: *mut symbolS = 0 as *mut symbolS;
    let mut sizeof_offset: libc::c_int = 0;
    memset(
        &mut exp as *mut expressionS as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<expressionS>() as libc::c_ulong,
    );
    sizeof_offset = out_header(info_seg, &mut exp);
    info_end = exp.X_add_symbol;
    out_two(dwarf_level as libc::c_int);
    if dwarf_level < 5 as libc::c_int as libc::c_uint {
        generic_dwarf2_emit_offset(
            section_symbol(abbrev_seg),
            sizeof_offset as libc::c_uint,
        );
    } else {
        out_byte(DW_UT_compile as libc::c_int);
    }
    out_byte(sizeof_address as libc::c_int);
    if dwarf_level >= 5 as libc::c_int as libc::c_uint {
        generic_dwarf2_emit_offset(
            section_symbol(abbrev_seg),
            sizeof_offset as libc::c_uint,
        );
    }
    out_uleb128(1 as libc::c_int as addressT);
    generic_dwarf2_emit_offset(
        section_symbol(line_seg),
        (if dwarf2_format_32bit as libc::c_int == dwarf2_format_32bit as libc::c_int {
            4 as libc::c_int
        } else {
            8 as libc::c_int
        }) as libc::c_uint,
    );
    if ((*all_segs).next).is_null() {
        exp.set_X_op(O_symbol);
        exp.X_add_symbol = (*all_segs).text_start;
        exp.X_add_number = 0 as libc::c_int as offsetT;
        emit_expr(&mut exp, sizeof_address);
        if dwarf_level < 4 as libc::c_int as libc::c_uint {
            exp.set_X_op(O_symbol);
        } else {
            exp.set_X_op(O_subtract);
            exp.X_op_symbol = (*all_segs).text_start;
        }
        exp.X_add_symbol = (*all_segs).text_end;
        exp.X_add_number = 0 as libc::c_int as offsetT;
        if dwarf_level < 4 as libc::c_int as libc::c_uint {
            emit_expr(&mut exp, sizeof_address);
        } else {
            emit_leb128_expr(&mut exp, 0 as libc::c_int);
        }
    } else {
        generic_dwarf2_emit_offset(ranges_sym, sizeof_offset as libc::c_uint);
    }
    generic_dwarf2_emit_offset(name_sym, sizeof_offset as libc::c_uint);
    generic_dwarf2_emit_offset(comp_dir_sym, sizeof_offset as libc::c_uint);
    generic_dwarf2_emit_offset(producer_sym, sizeof_offset as libc::c_uint);
    out_two(DW_LANG_Mips_Assembler as libc::c_int);
    symbol_set_value_now(info_end);
}
unsafe extern "C" fn out_debug_str(
    mut str_seg: segT,
    mut name_sym: *mut *mut symbolS,
    mut comp_dir_sym: *mut *mut symbolS,
    mut producer_sym: *mut *mut symbolS,
) {
    let mut producer: [libc::c_char; 128] = [0; 128];
    let mut comp_dir: *const libc::c_char = 0 as *const libc::c_char;
    let mut dirname: *const libc::c_char = 0 as *const libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: libc::c_int = 0;
    let mut first_file: libc::c_int = if (if dwarf_level
        > 3 as libc::c_int as libc::c_uint
    {
        dwarf_level
    } else {
        3 as libc::c_int as libc::c_uint
    }) > 4 as libc::c_int as libc::c_uint
    {
        0 as libc::c_int
    } else {
        1 as libc::c_int
    };
    subseg_set(str_seg, 0 as libc::c_int);
    *name_sym = symbol_temp_new_now_octets();
    if files_in_use == 0 as libc::c_int as libc::c_uint {
        as_abort(
            b"dwarf2dbg.c\0" as *const u8 as *const libc::c_char,
            2723 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 61],
                &[libc::c_char; 61],
            >(b"void out_debug_str(segT, symbolS **, symbolS **, symbolS **)\0"))
                .as_ptr(),
        );
    }
    if (*files.offset(first_file as isize)).dir != 0 {
        dirname = remap_debug_filename(
            *dirs.offset((*files.offset(first_file as isize)).dir as isize),
        );
        len = strlen(dirname) as libc::c_int;
        p = frag_more((len + 1 as libc::c_int) as size_t);
        memcpy(
            p as *mut libc::c_void,
            dirname as *const libc::c_void,
            len as libc::c_ulong,
        );
        *p.offset(len as isize) = '/' as i32 as libc::c_char;
    }
    len = (strlen((*files.offset(first_file as isize)).filename))
        .wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int;
    p = frag_more(len as size_t);
    memcpy(
        p as *mut libc::c_void,
        (*files.offset(first_file as isize)).filename as *const libc::c_void,
        len as libc::c_ulong,
    );
    *comp_dir_sym = symbol_temp_new_now_octets();
    comp_dir = remap_debug_filename(getpwd());
    len = (strlen(comp_dir)).wrapping_add(1 as libc::c_int as libc::c_ulong)
        as libc::c_int;
    p = frag_more(len as size_t);
    memcpy(
        p as *mut libc::c_void,
        comp_dir as *const libc::c_void,
        len as libc::c_ulong,
    );
    *producer_sym = symbol_temp_new_now_octets();
    sprintf(
        producer.as_mut_ptr(),
        b"GNU AS %s\0" as *const u8 as *const libc::c_char,
        b"2.37\0" as *const u8 as *const libc::c_char,
    );
    len = (strlen(producer.as_mut_ptr())).wrapping_add(1 as libc::c_int as libc::c_ulong)
        as libc::c_int;
    p = frag_more(len as size_t);
    memcpy(
        p as *mut libc::c_void,
        producer.as_mut_ptr() as *const libc::c_void,
        len as libc::c_ulong,
    );
}
unsafe extern "C" fn run_static_initializers() {
    current = {
        let mut init = dwarf2_line_info {
            filenum: 1 as libc::c_int as libc::c_uint,
            line: 1 as libc::c_int as libc::c_uint,
            column: 0 as libc::c_int as libc::c_uint,
            isa: 0 as libc::c_int as libc::c_uint,
            flags: (if 1 as libc::c_int != 0 {
                (1 as libc::c_int) << 0 as libc::c_int
            } else {
                0 as libc::c_int
            }) as libc::c_uint,
            discriminator: 0 as libc::c_int as libc::c_uint,
            view: 0 as *mut symbolS,
        };
        init
    };
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
