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
    pub type bfd_strtab_hash;
    pub type ecoff_debug_swap;
    pub type cie;
    fn bfd_sprintf_vma(_: *mut bfd, _: *mut libc::c_char, _: bfd_vma);
    fn bfd_hash_allocate(_: *mut bfd_hash_table, _: libc::c_uint) -> *mut libc::c_void;
    fn bfd_zalloc(abfd: *mut bfd, wanted: bfd_size_type) -> *mut libc::c_void;
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
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn htab_try_create(_: size_t, _: htab_hash, _: htab_eq, _: htab_del) -> htab_t;
    fn htab_delete(_: htab_t);
    fn htab_find_slot_with_hash(
        _: htab_t,
        _: *const libc::c_void,
        _: hashval_t,
        _: insert_option,
    ) -> *mut *mut libc::c_void;
    fn htab_traverse(_: htab_t, _: htab_trav, _: *mut libc::c_void);
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    static mut _bfd_std_section: [asection; 4];
    fn bfd_get_section_by_name(
        abfd: *mut bfd,
        name: *const libc::c_char,
    ) -> *mut asection;
    fn bfd_get_linker_section(
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
    fn bfd_set_error(error_tag: bfd_error_type);
    fn _bfd_error_handler(fmt: *const libc::c_char, _: ...);
    fn bfd_link_hash_lookup(
        _: *mut bfd_link_hash_table,
        _: *const libc::c_char,
        create: bool,
        copy: bool,
        follow: bool,
    ) -> *mut bfd_link_hash_entry;
    fn bfd_link_hash_traverse(
        _: *mut bfd_link_hash_table,
        _: Option::<
            unsafe extern "C" fn(*mut bfd_link_hash_entry, *mut libc::c_void) -> bool,
        >,
        _: *mut libc::c_void,
    );
    fn bfd_malloc(_: bfd_size_type) -> *mut libc::c_void;
    fn _bfd_link_hash_newfunc(
        entry: *mut bfd_hash_entry,
        table: *mut bfd_hash_table,
        string: *const libc::c_char,
    ) -> *mut bfd_hash_entry;
    fn _bfd_generic_link_add_one_symbol(
        _: *mut bfd_link_info,
        _: *mut bfd,
        name: *const libc::c_char,
        _: flagword,
        _: *mut asection,
        _: bfd_vma,
        _: *const libc::c_char,
        copy: bool,
        constructor: bool,
        _: *mut *mut bfd_link_hash_entry,
    ) -> bool;
    fn bfd_assert(_: *const libc::c_char, _: libc::c_int);
    fn _bfd_abort(_: *const libc::c_char, _: libc::c_int, _: *const libc::c_char) -> !;
    fn bfd_zmalloc(SIZE: bfd_size_type) -> *mut libc::c_void;
    fn bfd_log2(x: bfd_vma) -> libc::c_uint;
    fn _bfd_elf_section_from_bfd_section(_: *mut bfd, _: *mut asection) -> libc::c_uint;
    fn bfd_elf_sym_name(
        _: *mut bfd,
        _: *mut Elf_Internal_Shdr,
        _: *mut Elf_Internal_Sym,
        _: *mut asection,
    ) -> *const libc::c_char;
    fn bfd_elf_allocate_object(_: *mut bfd, _: size_t, _: elf_target_id) -> bool;
    fn _bfd_elf_link_hash_table_free(_: *mut bfd);
    fn _bfd_elf_link_hash_copy_indirect(
        _: *mut bfd_link_info,
        _: *mut elf_link_hash_entry,
        _: *mut elf_link_hash_entry,
    );
    fn _bfd_elf_link_hash_hide_symbol(
        _: *mut bfd_link_info,
        _: *mut elf_link_hash_entry,
        _: bool,
    );
    fn _bfd_elf_link_hash_table_init(
        _: *mut elf_link_hash_table,
        _: *mut bfd,
        _: Option::<
            unsafe extern "C" fn(
                *mut bfd_hash_entry,
                *mut bfd_hash_table,
                *const libc::c_char,
            ) -> *mut bfd_hash_entry,
        >,
        _: libc::c_uint,
        _: elf_target_id,
    ) -> bool;
    fn _bfd_elf_link_hide_sym_by_version(
        _: *mut bfd_link_info,
        _: *mut elf_link_hash_entry,
    ) -> bool;
    fn _bfd_elf_strtab_delref(_: *mut elf_strtab_hash, _: size_t);
    fn _bfd_elf_write_section_eh_frame(
        _: *mut bfd,
        _: *mut bfd_link_info,
        _: *mut asection,
        _: *mut bfd_byte,
    ) -> bool;
    fn _bfd_elf_eh_frame_present(_: *mut bfd_link_info) -> bool;
    fn _bfd_elf_hash_symbol(_: *mut elf_link_hash_entry) -> bool;
    fn _bfd_elf_create_got_section(_: *mut bfd, _: *mut bfd_link_info) -> bool;
    fn _bfd_elf_adjust_dynamic_copy(
        _: *mut bfd_link_info,
        _: *mut elf_link_hash_entry,
        _: *mut asection,
    ) -> bool;
    fn _bfd_elf_symbol_refs_local_p(
        _: *mut elf_link_hash_entry,
        _: *mut bfd_link_info,
        _: bool,
    ) -> bool;
    fn _bfd_elf_link_check_relocs(_: *mut bfd, _: *mut bfd_link_info) -> bool;
    fn bfd_elf_link_record_dynamic_symbol(
        _: *mut bfd_link_info,
        _: *mut elf_link_hash_entry,
    ) -> bool;
    fn _bfd_elf_gc_mark_hook(
        _: *mut asection,
        _: *mut bfd_link_info,
        _: *mut Elf_Internal_Rela,
        _: *mut elf_link_hash_entry,
        _: *mut Elf_Internal_Sym,
    ) -> *mut asection;
    fn _bfd_elf_get_property(
        _: *mut bfd,
        _: libc::c_uint,
        _: libc::c_uint,
    ) -> *mut elf_property;
    fn _bfd_elf_link_setup_gnu_properties(_: *mut bfd_link_info) -> *mut bfd;
    fn _bfd_elf_create_ifunc_sections(_: *mut bfd, _: *mut bfd_link_info) -> bool;
    fn _bfd_elf_allocate_ifunc_dyn_relocs(
        _: *mut bfd_link_info,
        _: *mut elf_link_hash_entry,
        _: *mut *mut elf_dyn_relocs,
        _: libc::c_uint,
        _: libc::c_uint,
        _: libc::c_uint,
        _: bool,
    ) -> bool;
    fn _bfd_elf_readonly_dynrelocs(_: *mut elf_link_hash_entry) -> *mut asection;
    fn free(_: *mut libc::c_void);
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn elf_vxworks_create_dynamic_sections(
        _: *mut bfd,
        _: *mut bfd_link_info,
        _: *mut *mut asection,
    ) -> bool;
    fn elf_vxworks_finish_dynamic_entry(_: *mut bfd, _: *mut Elf_Internal_Dyn) -> bool;
    fn _bfd_elf_maybe_vxworks_add_dynamic_tags(
        _: *mut bfd,
        _: *mut bfd_link_info,
        _: bool,
    ) -> bool;
    fn objalloc_create() -> *mut objalloc;
    fn _objalloc_alloc(_: *mut objalloc, _: libc::c_ulong) -> *mut libc::c_void;
    fn objalloc_free(_: *mut objalloc);
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
pub type notice_asneeded_action = libc::c_uint;
pub const notice_needed: notice_asneeded_action = 2;
pub const notice_not_needed: notice_asneeded_action = 1;
pub const notice_as_needed: notice_asneeded_action = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Elf32_External_Rel {
    pub r_offset: [libc::c_uchar; 4],
    pub r_info: [libc::c_uchar; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Elf32_External_Rela {
    pub r_offset: [libc::c_uchar; 4],
    pub r_info: [libc::c_uchar; 4],
    pub r_addend: [libc::c_uchar; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Elf64_External_Rela {
    pub r_offset: [libc::c_uchar; 8],
    pub r_info: [libc::c_uchar; 8],
    pub r_addend: [libc::c_uchar; 8],
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct elf_link_local_dynamic_entry {
    pub next: *mut elf_link_local_dynamic_entry,
    pub input_bfd: *mut bfd,
    pub input_indx: libc::c_long,
    pub dynindx: libc::c_long,
    pub isym: Elf_Internal_Sym,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct elf_link_loaded_list {
    pub next: *mut elf_link_loaded_list,
    pub abfd: *mut bfd,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct eh_cie_fde {
    pub u: C2RustUnnamed_23,
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
pub union C2RustUnnamed_23 {
    pub fde: C2RustUnnamed_26,
    pub cie: C2RustUnnamed_24,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2RustUnnamed_24 {
    pub u: C2RustUnnamed_25,
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
pub union C2RustUnnamed_25 {
    pub full_cie: *mut cie,
    pub merged_with: *mut eh_cie_fde,
    pub sec: *mut asection,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_26 {
    pub cie_inf: *mut eh_cie_fde,
    pub next_for_section: *mut eh_cie_fde,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct eh_frame_array_ent {
    pub initial_loc: bfd_vma,
    pub range: bfd_size_type,
    pub fde: bfd_vma,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dwarf_eh_frame_hdr_info {
    pub cies: *mut htab,
    pub fde_count: libc::c_uint,
    pub table: bool,
    pub array: *mut eh_frame_array_ent,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct compact_eh_frame_hdr_info {
    pub allocated_entries: libc::c_uint,
    pub entries: *mut *mut asection,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct eh_frame_hdr_info {
    pub hdr_sec: *mut asection,
    pub array_count: libc::c_uint,
    pub frame_hdr_is_compact: bool,
    pub u: C2RustUnnamed_27,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_27 {
    pub dwarf: dwarf_eh_frame_hdr_info,
    pub compact: compact_eh_frame_hdr_info,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct elf_sym_strtab {
    pub sym: Elf_Internal_Sym,
    pub dest_index: libc::c_ulong,
    pub destshndx_index: libc::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bfd_link_needed_list {
    pub next: *mut bfd_link_needed_list,
    pub by: *mut bfd,
    pub name: *const libc::c_char,
}
pub type elf_target_os = libc::c_uint;
pub const is_nacl: elf_target_os = 3;
pub const is_vxworks: elf_target_os = 2;
pub const is_solaris: elf_target_os = 1;
pub const is_normal: elf_target_os = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sym_cache {
    pub abfd: *mut bfd,
    pub indx: [libc::c_ulong; 32],
    pub sym: [Elf_Internal_Sym; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct elf_link_hash_table {
    pub root: bfd_link_hash_table,
    pub hash_table_id: elf_target_id,
    pub dynamic_sections_created: bool,
    pub dynamic_relocs: bool,
    pub is_relocatable_executable: bool,
    pub ifunc_resolvers: bool,
    pub dt_pltgot_required: bool,
    pub dt_jmprel_required: bool,
    pub dynobj: *mut bfd,
    pub init_got_refcount: gotplt_union,
    pub init_plt_refcount: gotplt_union,
    pub init_got_offset: gotplt_union,
    pub init_plt_offset: gotplt_union,
    pub dynsymcount: bfd_size_type,
    pub local_dynsymcount: bfd_size_type,
    pub dynstr: *mut elf_strtab_hash,
    pub strtabcount: bfd_size_type,
    pub strtabsize: bfd_size_type,
    pub strtab: *mut elf_sym_strtab,
    pub bucketcount: bfd_size_type,
    pub needed: *mut bfd_link_needed_list,
    pub text_index_section: *mut asection,
    pub data_index_section: *mut asection,
    pub hgot: *mut elf_link_hash_entry,
    pub hplt: *mut elf_link_hash_entry,
    pub hdynamic: *mut elf_link_hash_entry,
    pub merge_info: *mut libc::c_void,
    pub stab_info: stab_info,
    pub eh_info: eh_frame_hdr_info,
    pub dynlocal: *mut elf_link_local_dynamic_entry,
    pub runpath: *mut bfd_link_needed_list,
    pub tls_sec: *mut asection,
    pub tls_size: bfd_size_type,
    pub tlsdesc_plt: bfd_vma,
    pub tlsdesc_got: bfd_vma,
    pub target_os: elf_target_os,
    pub dyn_loaded: *mut elf_link_loaded_list,
    pub sym_cache: sym_cache,
    pub sgot: *mut asection,
    pub sgotplt: *mut asection,
    pub srelgot: *mut asection,
    pub splt: *mut asection,
    pub srelplt: *mut asection,
    pub sdynbss: *mut asection,
    pub srelbss: *mut asection,
    pub sdynrelro: *mut asection,
    pub sreldynrelro: *mut asection,
    pub igotplt: *mut asection,
    pub iplt: *mut asection,
    pub irelplt: *mut asection,
    pub irelifunc: *mut asection,
    pub dynsym: *mut asection,
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
    pub group: C2RustUnnamed_28,
    pub sec_group: *mut asection,
    pub next_in_group: *mut asection,
    pub fde_list: *mut eh_cie_fde,
    pub eh_frame_entry: *mut asection,
    pub has_secondary_relocs: bool,
    pub sec_info: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_28 {
    pub name: *const libc::c_char,
    pub id: *mut bfd_symbol,
}
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
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct elf_x86_link_hash_table {
    pub elf: elf_link_hash_table,
    pub interp: *mut asection,
    pub plt_eh_frame: *mut asection,
    pub plt_second: *mut asection,
    pub plt_second_eh_frame: *mut asection,
    pub plt_got: *mut asection,
    pub plt_got_eh_frame: *mut asection,
    pub plt: elf_x86_plt_layout,
    pub lazy_plt: *const elf_x86_lazy_plt_layout,
    pub non_lazy_plt: *const elf_x86_non_lazy_plt_layout,
    pub tls_ld_or_ldm_got: C2RustUnnamed_29,
    pub sgotplt_jump_table_size: bfd_vma,
    pub tls_module_base: *mut bfd_link_hash_entry,
    pub loc_hash_table: htab_t,
    pub loc_hash_memory: *mut libc::c_void,
    pub next_jump_slot_index: bfd_vma,
    pub next_irelative_index: bfd_vma,
    pub srelplt2: *mut asection,
    pub next_tls_desc_index: bfd_vma,
    pub plt0_pad_byte: bfd_byte,
    #[bitfield(name = "got_referenced", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "pcrel_plt", ty = "libc::c_uint", bits = "1..=1")]
    pub got_referenced_pcrel_plt: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 6],
    pub r_info: Option::<unsafe extern "C" fn(bfd_vma, bfd_vma) -> bfd_vma>,
    pub r_sym: Option::<unsafe extern "C" fn(bfd_vma) -> bfd_vma>,
    pub is_reloc_section: Option::<unsafe extern "C" fn(*const libc::c_char) -> bool>,
    pub sizeof_reloc: libc::c_uint,
    pub got_entry_size: libc::c_uint,
    pub pointer_r_type: libc::c_uint,
    pub dynamic_interpreter_size: libc::c_int,
    pub dynamic_interpreter: *const libc::c_char,
    pub tls_get_addr: *const libc::c_char,
    pub params: *mut elf_linker_x86_params,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_29 {
    pub refcount: bfd_signed_vma,
    pub offset: bfd_vma,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct elf_x86_non_lazy_plt_layout {
    pub plt_entry: *const bfd_byte,
    pub pic_plt_entry: *const bfd_byte,
    pub plt_entry_size: libc::c_uint,
    pub plt_got_offset: libc::c_uint,
    pub plt_got_insn_size: libc::c_uint,
    pub eh_frame_plt: *const bfd_byte,
    pub eh_frame_plt_size: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct elf_x86_lazy_plt_layout {
    pub plt0_entry: *const bfd_byte,
    pub plt0_entry_size: libc::c_uint,
    pub plt_entry: *const bfd_byte,
    pub plt_entry_size: libc::c_uint,
    pub plt_tlsdesc_entry: *const bfd_byte,
    pub plt_tlsdesc_entry_size: libc::c_uint,
    pub plt_tlsdesc_got1_offset: libc::c_uint,
    pub plt_tlsdesc_got2_offset: libc::c_uint,
    pub plt_tlsdesc_got1_insn_end: libc::c_uint,
    pub plt_tlsdesc_got2_insn_end: libc::c_uint,
    pub plt0_got1_offset: libc::c_uint,
    pub plt0_got2_offset: libc::c_uint,
    pub plt0_got2_insn_end: libc::c_uint,
    pub plt_got_offset: libc::c_uint,
    pub plt_reloc_offset: libc::c_uint,
    pub plt_plt_offset: libc::c_uint,
    pub plt_got_insn_size: libc::c_uint,
    pub plt_plt_insn_end: libc::c_uint,
    pub plt_lazy_offset: libc::c_uint,
    pub pic_plt0_entry: *const bfd_byte,
    pub pic_plt_entry: *const bfd_byte,
    pub eh_frame_plt: *const bfd_byte,
    pub eh_frame_plt_size: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct elf_x86_plt_layout {
    pub plt0_entry: *const bfd_byte,
    pub plt_entry: *const bfd_byte,
    pub plt_entry_size: libc::c_uint,
    pub has_plt0: libc::c_uint,
    pub plt_got_offset: libc::c_uint,
    pub plt_got_insn_size: libc::c_uint,
    pub iplt_alignment: libc::c_uint,
    pub eh_frame_plt: *const bfd_byte,
    pub eh_frame_plt_size: libc::c_uint,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct elf_x86_link_hash_entry {
    pub elf: elf_link_hash_entry,
    pub tls_type: libc::c_uchar,
    #[bitfield(name = "zero_undefweak", ty = "libc::c_uint", bits = "0..=1")]
    #[bitfield(name = "no_finish_dynamic_symbol", ty = "libc::c_uint", bits = "2..=2")]
    #[bitfield(name = "tls_get_addr", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "def_protected", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "local_ref", ty = "libc::c_uint", bits = "5..=6")]
    #[bitfield(name = "linker_def", ty = "libc::c_uint", bits = "7..=7")]
    #[bitfield(name = "gotoff_ref", ty = "libc::c_uint", bits = "8..=8")]
    #[bitfield(name = "needs_copy", ty = "libc::c_uint", bits = "9..=9")]
    pub zero_undefweak_no_finish_dynamic_symbol_tls_get_addr_def_protected_local_ref_linker_def_gotoff_ref_needs_copy: [u8; 2],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 5],
    pub plt_got: gotplt_union,
    pub plt_second: gotplt_union,
    pub tlsdesc_got: bfd_vma,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct elf_x86_init_table {
    pub lazy_plt: *const elf_x86_lazy_plt_layout,
    pub non_lazy_plt: *const elf_x86_non_lazy_plt_layout,
    pub lazy_ibt_plt: *const elf_x86_lazy_plt_layout,
    pub non_lazy_ibt_plt: *const elf_x86_non_lazy_plt_layout,
    pub plt0_pad_byte: bfd_byte,
    pub r_info: Option::<unsafe extern "C" fn(bfd_vma, bfd_vma) -> bfd_vma>,
    pub r_sym: Option::<unsafe extern "C" fn(bfd_vma) -> bfd_vma>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct elf_x86_obj_tdata {
    pub root: elf_obj_tdata,
    pub local_got_tls_type: *mut libc::c_char,
    pub local_tlsdesc_gotent: *mut bfd_vma,
}
pub type elf_x86_plt_type = libc::c_int;
pub const plt_unknown: elf_x86_plt_type = -1;
pub const plt_second: elf_x86_plt_type = 4;
pub const plt_pic: elf_x86_plt_type = 2;
pub const plt_lazy: elf_x86_plt_type = 1;
pub const plt_non_lazy: elf_x86_plt_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct elf_x86_plt {
    pub name: *const libc::c_char,
    pub sec: *mut asection,
    pub contents: *mut bfd_byte,
    pub type_0: elf_x86_plt_type,
    pub plt_got_offset: libc::c_uint,
    pub plt_entry_size: libc::c_uint,
    pub plt_got_insn_size: libc::c_uint,
    pub count: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct objalloc {
    pub current_ptr: *mut libc::c_char,
    pub current_space: libc::c_uint,
    pub chunks: *mut libc::c_void,
}
pub const R_386_32: elf_i386_reloc_type = 1;
pub const R_X86_64_32: elf_x86_64_reloc_type = 10;
pub const R_X86_64_64: elf_x86_64_reloc_type = 1;
pub const R_386_8: elf_i386_reloc_type = 22;
pub const R_386_16: elf_i386_reloc_type = 20;
pub const R_X86_64_REX_GOTPCRELX: elf_x86_64_reloc_type = 42;
pub const R_X86_64_GOTPCRELX: elf_x86_64_reloc_type = 41;
pub const R_X86_64_GOTPCREL: elf_x86_64_reloc_type = 9;
pub const R_X86_64_8: elf_x86_64_reloc_type = 14;
pub const R_X86_64_16: elf_x86_64_reloc_type = 12;
pub const R_X86_64_32S: elf_x86_64_reloc_type = 11;
pub const R_X86_64_GNU_VTENTRY: elf_x86_64_reloc_type = 251;
pub const R_X86_64_GNU_VTINHERIT: elf_x86_64_reloc_type = 250;
pub const R_386_GNU_VTENTRY: elf_i386_reloc_type = 251;
pub const R_386_GNU_VTINHERIT: elf_i386_reloc_type = 250;
pub const R_386_IRELATIVE: elf_i386_reloc_type = 42;
pub const R_386_GLOB_DAT: elf_i386_reloc_type = 6;
pub const R_386_JUMP_SLOT: elf_i386_reloc_type = 7;
pub const R_X86_64_IRELATIVE: elf_x86_64_reloc_type = 37;
pub const R_X86_64_GLOB_DAT: elf_x86_64_reloc_type = 6;
pub const R_X86_64_JUMP_SLOT: elf_x86_64_reloc_type = 7;
pub type elf_i386_reloc_type = libc::c_uint;
pub const R_386_max: elf_i386_reloc_type = 252;
pub const R_386_USED_BY_INTEL_200: elf_i386_reloc_type = 200;
pub const R_386_GOT32X: elf_i386_reloc_type = 43;
pub const R_386_TLS_DESC: elf_i386_reloc_type = 41;
pub const R_386_TLS_DESC_CALL: elf_i386_reloc_type = 40;
pub const R_386_TLS_GOTDESC: elf_i386_reloc_type = 39;
pub const R_386_SIZE32: elf_i386_reloc_type = 38;
pub const R_386_TLS_TPOFF32: elf_i386_reloc_type = 37;
pub const R_386_TLS_DTPOFF32: elf_i386_reloc_type = 36;
pub const R_386_TLS_DTPMOD32: elf_i386_reloc_type = 35;
pub const R_386_TLS_LE_32: elf_i386_reloc_type = 34;
pub const R_386_TLS_IE_32: elf_i386_reloc_type = 33;
pub const R_386_TLS_LDO_32: elf_i386_reloc_type = 32;
pub const R_386_TLS_LDM_POP: elf_i386_reloc_type = 31;
pub const R_386_TLS_LDM_CALL: elf_i386_reloc_type = 30;
pub const R_386_TLS_LDM_PUSH: elf_i386_reloc_type = 29;
pub const R_386_TLS_LDM_32: elf_i386_reloc_type = 28;
pub const R_386_TLS_GD_POP: elf_i386_reloc_type = 27;
pub const R_386_TLS_GD_CALL: elf_i386_reloc_type = 26;
pub const R_386_TLS_GD_PUSH: elf_i386_reloc_type = 25;
pub const R_386_TLS_GD_32: elf_i386_reloc_type = 24;
pub const R_386_PC8: elf_i386_reloc_type = 23;
pub const R_386_PC16: elf_i386_reloc_type = 21;
pub const R_386_TLS_LDM: elf_i386_reloc_type = 19;
pub const R_386_TLS_GD: elf_i386_reloc_type = 18;
pub const R_386_TLS_LE: elf_i386_reloc_type = 17;
pub const R_386_TLS_GOTIE: elf_i386_reloc_type = 16;
pub const R_386_TLS_IE: elf_i386_reloc_type = 15;
pub const R_386_TLS_TPOFF: elf_i386_reloc_type = 14;
pub const LAST_INVALID_RELOC: elf_i386_reloc_type = 13;
pub const FIRST_INVALID_RELOC: elf_i386_reloc_type = 12;
pub const R_386_32PLT: elf_i386_reloc_type = 11;
pub const R_386_GOTPC: elf_i386_reloc_type = 10;
pub const R_386_GOTOFF: elf_i386_reloc_type = 9;
pub const R_386_RELATIVE: elf_i386_reloc_type = 8;
pub const R_386_COPY: elf_i386_reloc_type = 5;
pub const R_386_PLT32: elf_i386_reloc_type = 4;
pub const R_386_GOT32: elf_i386_reloc_type = 3;
pub const R_386_PC32: elf_i386_reloc_type = 2;
pub const R_386_NONE: elf_i386_reloc_type = 0;
pub type elf_x86_64_reloc_type = libc::c_uint;
pub const R_X86_64_max: elf_x86_64_reloc_type = 252;
pub const R_X86_64_PLT32_BND: elf_x86_64_reloc_type = 40;
pub const R_X86_64_PC32_BND: elf_x86_64_reloc_type = 39;
pub const R_X86_64_RELATIVE64: elf_x86_64_reloc_type = 38;
pub const R_X86_64_TLSDESC: elf_x86_64_reloc_type = 36;
pub const R_X86_64_TLSDESC_CALL: elf_x86_64_reloc_type = 35;
pub const R_X86_64_GOTPC32_TLSDESC: elf_x86_64_reloc_type = 34;
pub const R_X86_64_SIZE64: elf_x86_64_reloc_type = 33;
pub const R_X86_64_SIZE32: elf_x86_64_reloc_type = 32;
pub const R_X86_64_PLTOFF64: elf_x86_64_reloc_type = 31;
pub const R_X86_64_GOTPLT64: elf_x86_64_reloc_type = 30;
pub const R_X86_64_GOTPC64: elf_x86_64_reloc_type = 29;
pub const R_X86_64_GOTPCREL64: elf_x86_64_reloc_type = 28;
pub const R_X86_64_GOT64: elf_x86_64_reloc_type = 27;
pub const R_X86_64_GOTPC32: elf_x86_64_reloc_type = 26;
pub const R_X86_64_GOTOFF64: elf_x86_64_reloc_type = 25;
pub const R_X86_64_PC64: elf_x86_64_reloc_type = 24;
pub const R_X86_64_TPOFF32: elf_x86_64_reloc_type = 23;
pub const R_X86_64_GOTTPOFF: elf_x86_64_reloc_type = 22;
pub const R_X86_64_DTPOFF32: elf_x86_64_reloc_type = 21;
pub const R_X86_64_TLSLD: elf_x86_64_reloc_type = 20;
pub const R_X86_64_TLSGD: elf_x86_64_reloc_type = 19;
pub const R_X86_64_TPOFF64: elf_x86_64_reloc_type = 18;
pub const R_X86_64_DTPOFF64: elf_x86_64_reloc_type = 17;
pub const R_X86_64_DTPMOD64: elf_x86_64_reloc_type = 16;
pub const R_X86_64_PC8: elf_x86_64_reloc_type = 15;
pub const R_X86_64_PC16: elf_x86_64_reloc_type = 13;
pub const R_X86_64_RELATIVE: elf_x86_64_reloc_type = 8;
pub const R_X86_64_COPY: elf_x86_64_reloc_type = 5;
pub const R_X86_64_PLT32: elf_x86_64_reloc_type = 4;
pub const R_X86_64_GOT32: elf_x86_64_reloc_type = 3;
pub const R_X86_64_PC32: elf_x86_64_reloc_type = 2;
pub const R_X86_64_NONE: elf_x86_64_reloc_type = 0;
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
unsafe extern "C" fn bfd_set_section_alignment(
    mut sec: *mut asection,
    mut val: libc::c_uint,
) -> bool {
    (*sec).alignment_power = val;
    return 1 as libc::c_int != 0;
}
#[inline]
unsafe extern "C" fn bfd_is_abs_section(mut sec: *const asection) -> bool {
    return sec
        == &mut *_bfd_std_section.as_mut_ptr().offset(2 as libc::c_int as isize)
            as *mut asection as *const asection;
}
#[inline]
unsafe extern "C" fn bfd_count_sections(mut abfd: *const bfd) -> libc::c_uint {
    return (*abfd).section_count;
}
#[inline]
unsafe extern "C" fn bfd_get_flavour(mut abfd: *const bfd) -> bfd_flavour {
    return (*(*abfd).xvec).flavour;
}
#[inline]
unsafe extern "C" fn weakdef(
    mut h: *mut elf_link_hash_entry,
) -> *mut elf_link_hash_entry {
    while (*h).is_weakalias() != 0 {
        h = (*h).u.alias;
    }
    return h;
}
#[inline]
unsafe extern "C" fn is_elf_hash_table(mut htab: *const bfd_link_hash_table) -> bool {
    return (*htab).type_0 as libc::c_uint
        == bfd_link_elf_hash_table as libc::c_int as libc::c_uint;
}
#[inline]
unsafe extern "C" fn elf_link_hash_lookup(
    mut table: *mut elf_link_hash_table,
    mut string: *const libc::c_char,
    mut create: bool,
    mut copy: bool,
    mut follow: bool,
) -> *mut elf_link_hash_entry {
    if 0 as libc::c_int != 0 && !is_elf_hash_table(&mut (*table).root) {
        _bfd_abort(
            b"./elf-bfd.h\0" as *const u8 as *const libc::c_char,
            731 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 114],
                &[libc::c_char; 114],
            >(
                b"struct elf_link_hash_entry *elf_link_hash_lookup(struct elf_link_hash_table *, const char *, _Bool, _Bool, _Bool)\0",
            ))
                .as_ptr(),
        );
    }
    return bfd_link_hash_lookup(&mut (*table).root, string, create, copy, follow)
        as *mut elf_link_hash_entry;
}
#[inline]
unsafe extern "C" fn elf_link_hash_traverse(
    mut table: *mut elf_link_hash_table,
    mut f: Option::<
        unsafe extern "C" fn(*mut elf_link_hash_entry, *mut libc::c_void) -> bool,
    >,
    mut info: *mut libc::c_void,
) {
    if 0 as libc::c_int != 0 && !is_elf_hash_table(&mut (*table).root) {
        _bfd_abort(
            b"./elf-bfd.h\0" as *const u8 as *const libc::c_char,
            744 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 115],
                &[libc::c_char; 115],
            >(
                b"void elf_link_hash_traverse(struct elf_link_hash_table *, _Bool (*)(struct elf_link_hash_entry *, void *), void *)\0",
            ))
                .as_ptr(),
        );
    }
    bfd_link_hash_traverse(
        &mut (*table).root,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut elf_link_hash_entry, *mut libc::c_void) -> bool,
            >,
            Option::<
                unsafe extern "C" fn(*mut bfd_link_hash_entry, *mut libc::c_void) -> bool,
            >,
        >(f),
        info,
    );
}
#[inline]
unsafe extern "C" fn elf_hash_table(
    mut info: *const bfd_link_info,
) -> *mut elf_link_hash_table {
    return (*info).hash as *mut elf_link_hash_table;
}
#[inline]
unsafe extern "C" fn elf_hash_table_id(
    mut table: *const elf_link_hash_table,
) -> elf_target_id {
    return (*table).hash_table_id;
}
#[no_mangle]
pub unsafe extern "C" fn _bfd_x86_elf_mkobject(mut abfd: *mut bfd) -> bool {
    return bfd_elf_allocate_object(
        abfd,
        ::core::mem::size_of::<elf_x86_obj_tdata>() as libc::c_ulong,
        (*((*(*abfd).xvec).backend_data as *const elf_backend_data)).target_id,
    );
}
#[no_mangle]
pub unsafe extern "C" fn _bfd_x86_elf_set_tls_module_base(mut info: *mut bfd_link_info) {
    let mut htab: *mut elf_x86_link_hash_table = 0 as *mut elf_x86_link_hash_table;
    let mut base: *mut bfd_link_hash_entry = 0 as *mut bfd_link_hash_entry;
    let mut bed: *const elf_backend_data = 0 as *const elf_backend_data;
    if !((*info).type_0() as libc::c_int == type_pde as libc::c_int
        || (*info).type_0() as libc::c_int == type_pie as libc::c_int)
    {
        return;
    }
    bed = (*(*(*info).output_bfd).xvec).backend_data as *const elf_backend_data;
    htab = if is_elf_hash_table((*info).hash) as libc::c_int != 0
        && elf_hash_table_id((*info).hash as *mut elf_link_hash_table) as libc::c_uint
            == (*bed).target_id as libc::c_uint
    {
        (*info).hash as *mut elf_x86_link_hash_table
    } else {
        0 as *mut elf_x86_link_hash_table
    };
    if htab.is_null() {
        return;
    }
    base = (*htab).tls_module_base;
    if base.is_null() {
        return;
    }
    (*base).u.def.value = (*htab).elf.tls_size;
}
#[no_mangle]
pub unsafe extern "C" fn _bfd_x86_elf_dtpoff_base(
    mut info: *mut bfd_link_info,
) -> bfd_vma {
    if ((*elf_hash_table(info)).tls_sec).is_null() {
        return 0 as libc::c_int as bfd_vma;
    }
    return (*(*elf_hash_table(info)).tls_sec).vma;
}
unsafe extern "C" fn elf_x86_allocate_dynrelocs(
    mut h: *mut elf_link_hash_entry,
    mut inf: *mut libc::c_void,
) -> bool {
    let mut info: *mut bfd_link_info = 0 as *mut bfd_link_info;
    let mut htab: *mut elf_x86_link_hash_table = 0 as *mut elf_x86_link_hash_table;
    let mut eh: *mut elf_x86_link_hash_entry = 0 as *mut elf_x86_link_hash_entry;
    let mut p: *mut elf_dyn_relocs = 0 as *mut elf_dyn_relocs;
    let mut plt_entry_size: libc::c_uint = 0;
    let mut resolved_to_zero: bool = false;
    let mut bed: *const elf_backend_data = 0 as *const elf_backend_data;
    if ((*h).root).type_0() as libc::c_int == bfd_link_hash_indirect as libc::c_int {
        return 1 as libc::c_int != 0;
    }
    eh = h as *mut elf_x86_link_hash_entry;
    info = inf as *mut bfd_link_info;
    bed = (*(*(*info).output_bfd).xvec).backend_data as *const elf_backend_data;
    htab = if is_elf_hash_table((*info).hash) as libc::c_int != 0
        && elf_hash_table_id((*info).hash as *mut elf_link_hash_table) as libc::c_uint
            == (*bed).target_id as libc::c_uint
    {
        (*info).hash as *mut elf_x86_link_hash_table
    } else {
        0 as *mut elf_x86_link_hash_table
    };
    if htab.is_null() {
        return 0 as libc::c_int != 0;
    }
    plt_entry_size = (*htab).plt.plt_entry_size;
    resolved_to_zero = ((*eh).elf.root).type_0() as libc::c_int
        == bfd_link_hash_undefweak as libc::c_int
        && (_bfd_x86_elf_link_symbol_references_local(info, &mut (*eh).elf)
            as libc::c_int != 0
            || ((*info).type_0() as libc::c_int == type_pde as libc::c_int
                || (*info).type_0() as libc::c_int == type_pie as libc::c_int)
                && (*eh).zero_undefweak() as libc::c_int > 0 as libc::c_int);
    if !((*htab).plt_got).is_null() && (*h).type_0() as libc::c_int != 10 as libc::c_int
        && (*h).pointer_equality_needed() == 0
        && (*h).plt.refcount > 0 as libc::c_int as libc::c_long
        && (*h).got.refcount > 0 as libc::c_int as libc::c_long
    {
        (*h).plt.offset = -(1 as libc::c_int) as bfd_vma;
        (*eh).plt_got.refcount = 1 as libc::c_int as bfd_signed_vma;
    }
    if (*h).type_0() as libc::c_int == 10 as libc::c_int
        && (*h).def_regular() as libc::c_int != 0
    {
        if (*eh).gotoff_ref() != 0 {
            (*h).plt.refcount = 1 as libc::c_int as bfd_signed_vma;
        }
        if _bfd_elf_allocate_ifunc_dyn_relocs(
            info,
            h,
            &mut (*h).dyn_relocs,
            plt_entry_size,
            ((*htab).plt.has_plt0).wrapping_mul(plt_entry_size),
            (*htab).got_entry_size,
            1 as libc::c_int != 0,
        ) {
            let mut s: *mut asection = (*htab).plt_second;
            if (*h).plt.offset != -(1 as libc::c_int) as bfd_vma && !s.is_null() {
                (*eh).plt_second.offset = (*s).size;
                (*s)
                    .size = ((*s).size as libc::c_ulong)
                    .wrapping_add(
                        (*(*htab).non_lazy_plt).plt_entry_size as libc::c_ulong,
                    ) as bfd_size_type as bfd_size_type;
            }
            return 1 as libc::c_int != 0;
        } else {
            return 0 as libc::c_int != 0
        }
    } else if (*htab).elf.dynamic_sections_created as libc::c_int != 0
        && ((*h).plt.refcount > 0 as libc::c_int as libc::c_long
            || (*eh).plt_got.refcount > 0 as libc::c_int as libc::c_long)
    {
        let mut use_plt_got: bool = (*eh).plt_got.refcount
            > 0 as libc::c_int as libc::c_long;
        if (*h).dynindx == -(1 as libc::c_int) as libc::c_long
            && (*h).forced_local() == 0 && !resolved_to_zero
            && ((*h).root).type_0() as libc::c_int
                == bfd_link_hash_undefweak as libc::c_int
        {
            if !bfd_elf_link_record_dynamic_symbol(info, h) {
                return 0 as libc::c_int != 0;
            }
        }
        if (*info).type_0() as libc::c_int == type_dll as libc::c_int
            || (*info).type_0() as libc::c_int == type_pie as libc::c_int
            || 1 as libc::c_int != 0
                && (0 as libc::c_int != 0 || (*h).forced_local() == 0)
                && ((*h).dynindx != -(1 as libc::c_int) as libc::c_long
                    || (*h).forced_local() as libc::c_int != 0)
        {
            let mut s_0: *mut asection = (*htab).elf.splt;
            let mut second_s: *mut asection = (*htab).plt_second;
            let mut got_s: *mut asection = (*htab).plt_got;
            let mut use_plt: bool = false;
            if (*s_0).size == 0 as libc::c_int as libc::c_ulong {
                (*s_0)
                    .size = ((*htab).plt.has_plt0).wrapping_mul(plt_entry_size)
                    as bfd_size_type;
            }
            if use_plt_got {
                (*eh).plt_got.offset = (*got_s).size;
            } else {
                (*h).plt.offset = (*s_0).size;
                if !second_s.is_null() {
                    (*eh).plt_second.offset = (*second_s).size;
                }
            }
            if (*h).def_regular() != 0 {
                use_plt = 0 as libc::c_int != 0;
            } else if (*htab).pcrel_plt() != 0 {
                use_plt = !((*info).type_0() as libc::c_int == type_dll as libc::c_int);
            } else {
                use_plt = (*info).type_0() as libc::c_int == type_pde as libc::c_int;
            }
            if use_plt {
                if use_plt_got {
                    (*h).root.u.def.section = got_s;
                    (*h).root.u.def.value = (*eh).plt_got.offset;
                } else if !second_s.is_null() {
                    (*h).root.u.def.section = second_s;
                    (*h).root.u.def.value = (*eh).plt_second.offset;
                } else {
                    (*h).root.u.def.section = s_0;
                    (*h).root.u.def.value = (*h).plt.offset;
                }
            }
            if use_plt_got {
                (*got_s)
                    .size = ((*got_s).size as libc::c_ulong)
                    .wrapping_add(
                        (*(*htab).non_lazy_plt).plt_entry_size as libc::c_ulong,
                    ) as bfd_size_type as bfd_size_type;
            } else {
                (*s_0)
                    .size = ((*s_0).size as libc::c_ulong)
                    .wrapping_add(plt_entry_size as libc::c_ulong) as bfd_size_type
                    as bfd_size_type;
                if !second_s.is_null() {
                    (*second_s)
                        .size = ((*second_s).size as libc::c_ulong)
                        .wrapping_add(
                            (*(*htab).non_lazy_plt).plt_entry_size as libc::c_ulong,
                        ) as bfd_size_type as bfd_size_type;
                }
                (*(*htab).elf.sgotplt)
                    .size = ((*(*htab).elf.sgotplt).size as libc::c_ulong)
                    .wrapping_add((*htab).got_entry_size as libc::c_ulong)
                    as bfd_size_type as bfd_size_type;
                if !resolved_to_zero {
                    (*(*htab).elf.srelplt)
                        .size = ((*(*htab).elf.srelplt).size as libc::c_ulong)
                        .wrapping_add((*htab).sizeof_reloc as libc::c_ulong)
                        as bfd_size_type as bfd_size_type;
                    (*(*htab).elf.srelplt)
                        .reloc_count = ((*(*htab).elf.srelplt).reloc_count)
                        .wrapping_add(1);
                    (*(*htab).elf.srelplt).reloc_count;
                }
            }
            if (*htab).elf.target_os as libc::c_uint
                == is_vxworks as libc::c_int as libc::c_uint
                && !((*info).type_0() as libc::c_int == type_dll as libc::c_int
                    || (*info).type_0() as libc::c_int == type_pie as libc::c_int)
            {
                let mut srelplt2: *mut asection = (*htab).srelplt2;
                if (*h).plt.offset == plt_entry_size as libc::c_ulong {
                    (*srelplt2)
                        .size = ((*srelplt2).size as libc::c_ulong)
                        .wrapping_add(
                            ((*htab).sizeof_reloc)
                                .wrapping_mul(2 as libc::c_int as libc::c_uint)
                                as libc::c_ulong,
                        ) as bfd_size_type as bfd_size_type;
                }
                (*srelplt2)
                    .size = ((*srelplt2).size as libc::c_ulong)
                    .wrapping_add(
                        ((*htab).sizeof_reloc)
                            .wrapping_mul(2 as libc::c_int as libc::c_uint)
                            as libc::c_ulong,
                    ) as bfd_size_type as bfd_size_type;
            }
        } else {
            (*eh).plt_got.offset = -(1 as libc::c_int) as bfd_vma;
            (*h).plt.offset = -(1 as libc::c_int) as bfd_vma;
            (*h).set_needs_plt(0 as libc::c_int as libc::c_uint);
        }
    } else {
        (*eh).plt_got.offset = -(1 as libc::c_int) as bfd_vma;
        (*h).plt.offset = -(1 as libc::c_int) as bfd_vma;
        (*h).set_needs_plt(0 as libc::c_int as libc::c_uint);
    }
    (*eh).tlsdesc_got = -(1 as libc::c_int) as bfd_vma;
    if (*h).got.refcount > 0 as libc::c_int as libc::c_long
        && ((*info).type_0() as libc::c_int == type_pde as libc::c_int
            || (*info).type_0() as libc::c_int == type_pie as libc::c_int)
        && (*h).dynindx == -(1 as libc::c_int) as libc::c_long
        && (*(h as *mut elf_x86_link_hash_entry)).tls_type as libc::c_int
            & 4 as libc::c_int != 0
    {
        (*h).got.offset = -(1 as libc::c_int) as bfd_vma;
    } else if (*h).got.refcount > 0 as libc::c_int as libc::c_long {
        let mut s_1: *mut asection = 0 as *mut asection;
        let mut dyn_0: bool = false;
        let mut tls_type: libc::c_int = (*(h as *mut elf_x86_link_hash_entry)).tls_type
            as libc::c_int;
        if (*h).dynindx == -(1 as libc::c_int) as libc::c_long
            && (*h).forced_local() == 0 && !resolved_to_zero
            && ((*h).root).type_0() as libc::c_int
                == bfd_link_hash_undefweak as libc::c_int
        {
            if !bfd_elf_link_record_dynamic_symbol(info, h) {
                return 0 as libc::c_int != 0;
            }
        }
        s_1 = (*htab).elf.sgot;
        if tls_type == 8 as libc::c_int
            || tls_type == 2 as libc::c_int | 8 as libc::c_int
        {
            (*eh)
                .tlsdesc_got = ((*(*htab).elf.sgotplt).size)
                .wrapping_sub(
                    ((*(*htab).elf.srelplt).reloc_count)
                        .wrapping_mul((*htab).got_entry_size) as libc::c_ulong,
                );
            (*(*htab).elf.sgotplt)
                .size = ((*(*htab).elf.sgotplt).size as libc::c_ulong)
                .wrapping_add(
                    (2 as libc::c_int as libc::c_uint)
                        .wrapping_mul((*htab).got_entry_size) as libc::c_ulong,
                ) as bfd_size_type as bfd_size_type;
            (*h).got.offset = -(2 as libc::c_int) as bfd_vma;
        }
        if !(tls_type == 8 as libc::c_int
            || tls_type == 2 as libc::c_int | 8 as libc::c_int)
            || (tls_type == 2 as libc::c_int
                || tls_type == 2 as libc::c_int | 8 as libc::c_int)
        {
            (*h).got.offset = (*s_1).size;
            (*s_1)
                .size = ((*s_1).size as libc::c_ulong)
                .wrapping_add((*htab).got_entry_size as libc::c_ulong) as bfd_size_type
                as bfd_size_type;
            if tls_type == 2 as libc::c_int
                || tls_type == 2 as libc::c_int | 8 as libc::c_int
                || tls_type == 7 as libc::c_int
            {
                (*s_1)
                    .size = ((*s_1).size as libc::c_ulong)
                    .wrapping_add((*htab).got_entry_size as libc::c_ulong)
                    as bfd_size_type as bfd_size_type;
            }
        }
        dyn_0 = (*htab).elf.dynamic_sections_created;
        if tls_type == 7 as libc::c_int {
            (*(*htab).elf.srelgot)
                .size = ((*(*htab).elf.srelgot).size as libc::c_ulong)
                .wrapping_add(
                    (2 as libc::c_int as libc::c_uint).wrapping_mul((*htab).sizeof_reloc)
                        as libc::c_ulong,
                ) as bfd_size_type as bfd_size_type;
        } else if (tls_type == 2 as libc::c_int
            || tls_type == 2 as libc::c_int | 8 as libc::c_int)
            && (*h).dynindx == -(1 as libc::c_int) as libc::c_long
            || tls_type & 4 as libc::c_int != 0
        {
            (*(*htab).elf.srelgot)
                .size = ((*(*htab).elf.srelgot).size as libc::c_ulong)
                .wrapping_add((*htab).sizeof_reloc as libc::c_ulong) as bfd_size_type
                as bfd_size_type;
        } else if tls_type == 2 as libc::c_int
            || tls_type == 2 as libc::c_int | 8 as libc::c_int
        {
            (*(*htab).elf.srelgot)
                .size = ((*(*htab).elf.srelgot).size as libc::c_ulong)
                .wrapping_add(
                    (2 as libc::c_int as libc::c_uint).wrapping_mul((*htab).sizeof_reloc)
                        as libc::c_ulong,
                ) as bfd_size_type as bfd_size_type;
        } else if !(tls_type == 8 as libc::c_int
            || tls_type == 2 as libc::c_int | 8 as libc::c_int)
            && ((*h).other() as libc::c_int & 0x3 as libc::c_int == 0 as libc::c_int
                && !resolved_to_zero
                || ((*h).root).type_0() as libc::c_int
                    != bfd_link_hash_undefweak as libc::c_int)
            && (((*info).type_0() as libc::c_int == type_dll as libc::c_int
                || (*info).type_0() as libc::c_int == type_pie as libc::c_int)
                && !((*h).dynindx == -(1 as libc::c_int) as libc::c_long
                    && ((((*h).root).type_0() as libc::c_int
                        == bfd_link_hash_defined as libc::c_int
                        || ((*h).root).type_0() as libc::c_int
                            == bfd_link_hash_defweak as libc::c_int)
                        && bfd_is_abs_section((*h).root.u.def.section) as libc::c_int
                            != 0 && ((*h).root).rel_from_abs() == 0
                        && ((*h).root).ldscript_def() == 0))
                || dyn_0 as libc::c_int != 0
                    && (0 as libc::c_int != 0 || (*h).forced_local() == 0)
                    && ((*h).dynindx != -(1 as libc::c_int) as libc::c_long
                        || (*h).forced_local() as libc::c_int != 0))
        {
            (*(*htab).elf.srelgot)
                .size = ((*(*htab).elf.srelgot).size as libc::c_ulong)
                .wrapping_add((*htab).sizeof_reloc as libc::c_ulong) as bfd_size_type
                as bfd_size_type;
        }
        if tls_type == 8 as libc::c_int
            || tls_type == 2 as libc::c_int | 8 as libc::c_int
        {
            (*(*htab).elf.srelplt)
                .size = ((*(*htab).elf.srelplt).size as libc::c_ulong)
                .wrapping_add((*htab).sizeof_reloc as libc::c_ulong) as bfd_size_type
                as bfd_size_type;
            if (*bed).target_id as libc::c_uint
                == X86_64_ELF_DATA as libc::c_int as libc::c_uint
            {
                (*htab).elf.tlsdesc_plt = -(1 as libc::c_int) as bfd_vma;
            }
        }
    } else {
        (*h).got.offset = -(1 as libc::c_int) as bfd_vma;
    }
    if ((*h).dyn_relocs).is_null() {
        return 1 as libc::c_int != 0;
    }
    if (*info).type_0() as libc::c_int == type_dll as libc::c_int
        || (*info).type_0() as libc::c_int == type_pie as libc::c_int
    {
        if _bfd_elf_symbol_refs_local_p(h, info, 1 as libc::c_int != 0) {
            let mut pp: *mut *mut elf_dyn_relocs = 0 as *mut *mut elf_dyn_relocs;
            pp = &mut (*h).dyn_relocs;
            loop {
                p = *pp;
                if p.is_null() {
                    break;
                }
                (*p)
                    .count = ((*p).count as libc::c_ulong).wrapping_sub((*p).pc_count)
                    as bfd_size_type as bfd_size_type;
                (*p).pc_count = 0 as libc::c_int as bfd_size_type;
                if (*p).count == 0 as libc::c_int as libc::c_ulong {
                    *pp = (*p).next;
                } else {
                    pp = &mut (*p).next;
                }
            }
        }
        if (*htab).elf.target_os as libc::c_uint
            == is_vxworks as libc::c_int as libc::c_uint
        {
            let mut pp_0: *mut *mut elf_dyn_relocs = 0 as *mut *mut elf_dyn_relocs;
            pp_0 = &mut (*h).dyn_relocs;
            loop {
                p = *pp_0;
                if p.is_null() {
                    break;
                }
                if strcmp(
                    (*(*(*p).sec).output_section).name,
                    b".tls_vars\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    *pp_0 = (*p).next;
                } else {
                    pp_0 = &mut (*p).next;
                }
            }
        }
        if !((*h).dyn_relocs).is_null() {
            if ((*h).root).type_0() as libc::c_int
                == bfd_link_hash_undefweak as libc::c_int
            {
                if (*h).other() as libc::c_int & 0x3 as libc::c_int != 0 as libc::c_int
                    || resolved_to_zero as libc::c_int != 0
                {
                    if (*bed).target_id as libc::c_uint
                        == I386_ELF_DATA as libc::c_int as libc::c_uint
                        && (*h).non_got_ref() as libc::c_int != 0
                    {
                        let mut pp_1: *mut *mut elf_dyn_relocs = 0
                            as *mut *mut elf_dyn_relocs;
                        pp_1 = &mut (*h).dyn_relocs;
                        loop {
                            p = *pp_1;
                            if p.is_null() {
                                break;
                            }
                            if (*p).pc_count == 0 as libc::c_int as libc::c_ulong {
                                *pp_1 = (*p).next;
                            } else {
                                (*p).count = (*p).pc_count;
                                pp_1 = &mut (*p).next;
                            }
                        }
                        if !((*h).dyn_relocs).is_null()
                            && !bfd_elf_link_record_dynamic_symbol(info, h)
                        {
                            return 0 as libc::c_int != 0;
                        }
                    } else {
                        (*h).dyn_relocs = 0 as *mut elf_dyn_relocs;
                    }
                } else if (*h).dynindx == -(1 as libc::c_int) as libc::c_long
                    && (*h).forced_local() == 0
                    && !bfd_elf_link_record_dynamic_symbol(info, h)
                {
                    return 0 as libc::c_int != 0
                }
            } else if ((*info).type_0() as libc::c_int == type_pde as libc::c_int
                || (*info).type_0() as libc::c_int == type_pie as libc::c_int)
                && ((*h).needs_copy() as libc::c_int != 0
                    || (*eh).needs_copy() as libc::c_int != 0)
                && (*h).def_dynamic() as libc::c_int != 0 && (*h).def_regular() == 0
            {
                let mut pp_2: *mut *mut elf_dyn_relocs = 0 as *mut *mut elf_dyn_relocs;
                pp_2 = &mut (*h).dyn_relocs;
                loop {
                    p = *pp_2;
                    if p.is_null() {
                        break;
                    }
                    if (*p).pc_count != 0 as libc::c_int as libc::c_ulong {
                        *pp_2 = (*p).next;
                    } else {
                        pp_2 = &mut (*p).next;
                    }
                }
            }
        }
    } else {
        let mut current_block_173: u64;
        if ((*h).non_got_ref() == 0
            || ((*h).root).type_0() as libc::c_int
                == bfd_link_hash_undefweak as libc::c_int && !resolved_to_zero)
            && ((*h).def_dynamic() as libc::c_int != 0 && (*h).def_regular() == 0
                || (*htab).elf.dynamic_sections_created as libc::c_int != 0
                    && (((*h).root).type_0() as libc::c_int
                        == bfd_link_hash_undefweak as libc::c_int
                        || ((*h).root).type_0() as libc::c_int
                            == bfd_link_hash_undefined as libc::c_int))
        {
            if (*h).dynindx == -(1 as libc::c_int) as libc::c_long
                && (*h).forced_local() == 0 && !resolved_to_zero
                && ((*h).root).type_0() as libc::c_int
                    == bfd_link_hash_undefweak as libc::c_int
                && !bfd_elf_link_record_dynamic_symbol(info, h)
            {
                return 0 as libc::c_int != 0;
            }
            if (*h).dynindx != -(1 as libc::c_int) as libc::c_long {
                current_block_173 = 4330759529560430365;
            } else {
                current_block_173 = 13895078145312174667;
            }
        } else {
            current_block_173 = 13895078145312174667;
        }
        match current_block_173 {
            13895078145312174667 => {
                (*h).dyn_relocs = 0 as *mut elf_dyn_relocs;
            }
            _ => {}
        }
    }
    p = (*h).dyn_relocs;
    while !p.is_null() {
        let mut sreloc: *mut asection = 0 as *mut asection;
        sreloc = (*((*(*p).sec).used_by_bfd as *mut bfd_elf_section_data)).sreloc;
        if sreloc.is_null() {
            bfd_assert(
                b"elfxx-x86.c\0" as *const u8 as *const libc::c_char,
                531 as libc::c_int,
            );
        }
        (*sreloc)
            .size = ((*sreloc).size as libc::c_ulong)
            .wrapping_add(
                ((*p).count).wrapping_mul((*htab).sizeof_reloc as libc::c_ulong),
            ) as bfd_size_type as bfd_size_type;
        p = (*p).next;
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn elf_x86_allocate_local_dynreloc(
    mut slot: *mut *mut libc::c_void,
    mut inf: *mut libc::c_void,
) -> libc::c_int {
    let mut h: *mut elf_link_hash_entry = *slot as *mut elf_link_hash_entry;
    if (*h).type_0() as libc::c_int != 10 as libc::c_int || (*h).def_regular() == 0
        || (*h).ref_regular() == 0 || (*h).forced_local() == 0
        || ((*h).root).type_0() as libc::c_int != bfd_link_hash_defined as libc::c_int
    {
        _bfd_abort(
            b"elfxx-x86.c\0" as *const u8 as *const libc::c_char,
            552 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 53],
                &[libc::c_char; 53],
            >(b"int elf_x86_allocate_local_dynreloc(void **, void *)\0"))
                .as_ptr(),
        );
    }
    return elf_x86_allocate_dynrelocs(h, inf) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _bfd_elf_x86_get_local_sym_hash(
    mut htab: *mut elf_x86_link_hash_table,
    mut abfd: *mut bfd,
    mut rel: *const Elf_Internal_Rela,
    mut create: bool,
) -> *mut elf_link_hash_entry {
    let mut e: elf_x86_link_hash_entry = elf_x86_link_hash_entry {
        elf: elf_link_hash_entry {
            root: bfd_link_hash_entry {
                root: bfd_hash_entry {
                    next: 0 as *mut bfd_hash_entry,
                    string: 0 as *const libc::c_char,
                    hash: 0,
                },
                type_0_non_ir_ref_regular_non_ir_ref_dynamic_linker_def_ldscript_def_rel_from_abs: [0; 2],
                c2rust_padding: [0; 6],
                u: C2RustUnnamed_8 {
                    undef: C2RustUnnamed_12 {
                        next: 0 as *mut bfd_link_hash_entry,
                        abfd: 0 as *mut bfd,
                    },
                },
            },
            indx: 0,
            dynindx: 0,
            got: gotplt_union { refcount: 0 },
            plt: gotplt_union { refcount: 0 },
            size: 0,
            dyn_relocs: 0 as *mut elf_dyn_relocs,
            type_0_other_target_internal_ref_regular_def_regular_ref_dynamic_def_dynamic_ref_regular_nonweak_ref_ir_nonweak_dynamic_adjusted_needs_copy_needs_plt_non_elf_versioned_forced_local_dynamic_mark_non_got_ref_dynamic_def_ref_dynamic_nonweak_pointer_equality_needed_unique_global_protected_def_start_stop_is_weakalias: [0; 6],
            c2rust_padding: [0; 2],
            dynstr_index: 0,
            u: C2RustUnnamed_18 {
                alias: 0 as *mut elf_link_hash_entry,
            },
            verinfo: C2RustUnnamed_17 {
                verdef: 0 as *mut Elf_Internal_Verdef,
            },
            u2: C2RustUnnamed_16 {
                start_stop_section: 0 as *mut asection,
            },
        },
        tls_type: 0,
        zero_undefweak_no_finish_dynamic_symbol_tls_get_addr_def_protected_local_ref_linker_def_gotoff_ref_needs_copy: [0; 2],
        c2rust_padding: [0; 5],
        plt_got: gotplt_union { refcount: 0 },
        plt_second: gotplt_union { refcount: 0 },
        tlsdesc_got: 0,
    };
    let mut ret: *mut elf_x86_link_hash_entry = 0 as *mut elf_x86_link_hash_entry;
    let mut sec: *mut asection = (*abfd).sections;
    let mut h: hashval_t = ((((*sec).id & 0xff as libc::c_uint) << 24 as libc::c_int
        | ((*sec).id & 0xff00 as libc::c_int as libc::c_uint) << 8 as libc::c_int)
        as libc::c_ulong
        ^ ((*htab).r_sym).expect("non-null function pointer")((*rel).r_info)
        ^ (((*sec).id & 0xffff0000 as libc::c_uint) >> 16 as libc::c_int)
            as libc::c_ulong) as hashval_t;
    let mut slot: *mut *mut libc::c_void = 0 as *mut *mut libc::c_void;
    e.elf.indx = (*sec).id as libc::c_long;
    e
        .elf
        .dynstr_index = ((*htab).r_sym)
        .expect("non-null function pointer")((*rel).r_info);
    slot = htab_find_slot_with_hash(
        (*htab).loc_hash_table,
        &mut e as *mut elf_x86_link_hash_entry as *const libc::c_void,
        h,
        (if create as libc::c_int != 0 {
            INSERT as libc::c_int
        } else {
            NO_INSERT as libc::c_int
        }) as insert_option,
    );
    if slot.is_null() {
        return 0 as *mut elf_link_hash_entry;
    }
    if !(*slot).is_null() {
        ret = *slot as *mut elf_x86_link_hash_entry;
        return &mut (*ret).elf;
    }
    ret = ({
        let mut __o: *mut objalloc = (*htab).loc_hash_memory as *mut objalloc;
        let mut __len: libc::c_ulong = ::core::mem::size_of::<elf_x86_link_hash_entry>()
            as libc::c_ulong;
        if __len == 0 as libc::c_int as libc::c_ulong {
            __len = 1 as libc::c_int as libc::c_ulong;
        }
        __len = __len
            .wrapping_add(8 as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            & !(8 as libc::c_ulong).wrapping_sub(1 as libc::c_int as libc::c_ulong);
        if __len != 0 as libc::c_int as libc::c_ulong
            && __len <= (*__o).current_space as libc::c_ulong
        {
            (*__o).current_ptr = ((*__o).current_ptr).offset(__len as isize);
            (*__o)
                .current_space = ((*__o).current_space as libc::c_ulong)
                .wrapping_sub(__len) as libc::c_uint as libc::c_uint;
            ((*__o).current_ptr).offset(-(__len as isize)) as *mut libc::c_void
        } else {
            _objalloc_alloc(__o, __len)
        }
    }) as *mut elf_x86_link_hash_entry;
    if !ret.is_null() {
        memset(
            ret as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<elf_x86_link_hash_entry>() as libc::c_ulong,
        );
        (*ret).elf.indx = (*sec).id as libc::c_long;
        (*ret)
            .elf
            .dynstr_index = ((*htab).r_sym)
            .expect("non-null function pointer")((*rel).r_info);
        (*ret).elf.dynindx = -(1 as libc::c_int) as libc::c_long;
        (*ret).plt_got.offset = -(1 as libc::c_int) as bfd_vma;
        *slot = ret as *mut libc::c_void;
    }
    return &mut (*ret).elf;
}
#[no_mangle]
pub unsafe extern "C" fn _bfd_x86_elf_link_hash_newfunc(
    mut entry: *mut bfd_hash_entry,
    mut table: *mut bfd_hash_table,
    mut string: *const libc::c_char,
) -> *mut bfd_hash_entry {
    if entry.is_null() {
        entry = bfd_hash_allocate(
            table,
            ::core::mem::size_of::<elf_x86_link_hash_entry>() as libc::c_ulong
                as libc::c_uint,
        ) as *mut bfd_hash_entry;
        if entry.is_null() {
            return entry;
        }
    }
    entry = _bfd_link_hash_newfunc(entry, table, string);
    if !entry.is_null() {
        let mut eh: *mut elf_x86_link_hash_entry = entry as *mut elf_x86_link_hash_entry;
        let mut htab: *mut elf_link_hash_table = table as *mut elf_link_hash_table;
        memset(
            &mut (*eh).elf.size as *mut bfd_size_type as *mut libc::c_void,
            0 as libc::c_int,
            (::core::mem::size_of::<elf_x86_link_hash_entry>() as libc::c_ulong)
                .wrapping_sub(88 as libc::c_ulong),
        );
        (*eh).elf.indx = -(1 as libc::c_int) as libc::c_long;
        (*eh).elf.dynindx = -(1 as libc::c_int) as libc::c_long;
        (*eh).elf.got = (*htab).init_got_refcount;
        (*eh).elf.plt = (*htab).init_plt_refcount;
        ((*eh).elf).set_non_elf(1 as libc::c_int as libc::c_uint);
        (*eh).plt_second.offset = -(1 as libc::c_int) as bfd_vma;
        (*eh).plt_got.offset = -(1 as libc::c_int) as bfd_vma;
        (*eh).tlsdesc_got = -(1 as libc::c_int) as bfd_vma;
        (*eh).set_zero_undefweak(1 as libc::c_int as libc::c_uint);
    }
    return entry;
}
#[no_mangle]
pub unsafe extern "C" fn _bfd_x86_elf_local_htab_hash(
    mut ptr: *const libc::c_void,
) -> hashval_t {
    let mut h: *mut elf_link_hash_entry = ptr as *mut elf_link_hash_entry;
    return ((((*h).indx & 0xff as libc::c_uint as libc::c_long) << 24 as libc::c_int
        | ((*h).indx & 0xff00 as libc::c_int as libc::c_long) << 8 as libc::c_int)
        as libc::c_ulong ^ (*h).dynstr_index
        ^ (((*h).indx & 0xffff0000 as libc::c_uint as libc::c_long) >> 16 as libc::c_int)
            as libc::c_ulong) as hashval_t;
}
#[no_mangle]
pub unsafe extern "C" fn _bfd_x86_elf_local_htab_eq(
    mut ptr1: *const libc::c_void,
    mut ptr2: *const libc::c_void,
) -> libc::c_int {
    let mut h1: *mut elf_link_hash_entry = ptr1 as *mut elf_link_hash_entry;
    let mut h2: *mut elf_link_hash_entry = ptr2 as *mut elf_link_hash_entry;
    return ((*h1).indx == (*h2).indx && (*h1).dynstr_index == (*h2).dynstr_index)
        as libc::c_int;
}
unsafe extern "C" fn elf_x86_link_hash_table_free(mut obfd: *mut bfd) {
    let mut htab: *mut elf_x86_link_hash_table = (*obfd).link.hash
        as *mut elf_x86_link_hash_table;
    if !((*htab).loc_hash_table).is_null() {
        htab_delete((*htab).loc_hash_table);
    }
    if !((*htab).loc_hash_memory).is_null() {
        objalloc_free((*htab).loc_hash_memory as *mut objalloc);
    }
    _bfd_elf_link_hash_table_free(obfd);
}
unsafe extern "C" fn elf_i386_is_reloc_section(
    mut secname: *const libc::c_char,
) -> bool {
    return startswith(secname, b".rel\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn elf_x86_64_is_reloc_section(
    mut secname: *const libc::c_char,
) -> bool {
    return startswith(secname, b".rela\0" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn _bfd_x86_elf_link_hash_table_create(
    mut abfd: *mut bfd,
) -> *mut bfd_link_hash_table {
    let mut ret: *mut elf_x86_link_hash_table = 0 as *mut elf_x86_link_hash_table;
    let mut bed: *const elf_backend_data = 0 as *const elf_backend_data;
    let mut amt: size_t = ::core::mem::size_of::<elf_x86_link_hash_table>()
        as libc::c_ulong;
    ret = bfd_zmalloc(amt) as *mut elf_x86_link_hash_table;
    if ret.is_null() {
        return 0 as *mut bfd_link_hash_table;
    }
    bed = (*(*abfd).xvec).backend_data as *const elf_backend_data;
    if !_bfd_elf_link_hash_table_init(
        &mut (*ret).elf,
        abfd,
        Some(
            _bfd_x86_elf_link_hash_newfunc
                as unsafe extern "C" fn(
                    *mut bfd_hash_entry,
                    *mut bfd_hash_table,
                    *const libc::c_char,
                ) -> *mut bfd_hash_entry,
        ),
        ::core::mem::size_of::<elf_x86_link_hash_entry>() as libc::c_ulong
            as libc::c_uint,
        (*bed).target_id,
    ) {
        free(ret as *mut libc::c_void);
        return 0 as *mut bfd_link_hash_table;
    }
    if (*bed).target_id as libc::c_uint == X86_64_ELF_DATA as libc::c_int as libc::c_uint
    {
        (*ret)
            .is_reloc_section = Some(
            elf_x86_64_is_reloc_section
                as unsafe extern "C" fn(*const libc::c_char) -> bool,
        );
        (*ret).got_entry_size = 8 as libc::c_int as libc::c_uint;
        (*ret).set_pcrel_plt(1 as libc::c_int as libc::c_uint);
        (*ret).tls_get_addr = b"__tls_get_addr\0" as *const u8 as *const libc::c_char;
    }
    if (*(*((*(*abfd).xvec).backend_data as *const elf_backend_data)).s).elfclass
        as libc::c_int == 2 as libc::c_int
    {
        (*ret)
            .sizeof_reloc = ::core::mem::size_of::<Elf64_External_Rela>()
            as libc::c_ulong as libc::c_uint;
        (*ret).pointer_r_type = R_X86_64_64 as libc::c_int as libc::c_uint;
        (*ret)
            .dynamic_interpreter = b"/lib/ld64.so.1\0" as *const u8
            as *const libc::c_char;
        (*ret)
            .dynamic_interpreter_size = ::core::mem::size_of::<[libc::c_char; 15]>()
            as libc::c_ulong as libc::c_int;
    } else if (*bed).target_id as libc::c_uint
        == X86_64_ELF_DATA as libc::c_int as libc::c_uint
    {
        (*ret)
            .sizeof_reloc = ::core::mem::size_of::<Elf32_External_Rela>()
            as libc::c_ulong as libc::c_uint;
        (*ret).pointer_r_type = R_X86_64_32 as libc::c_int as libc::c_uint;
        (*ret)
            .dynamic_interpreter = b"/lib/ldx32.so.1\0" as *const u8
            as *const libc::c_char;
        (*ret)
            .dynamic_interpreter_size = ::core::mem::size_of::<[libc::c_char; 16]>()
            as libc::c_ulong as libc::c_int;
    } else {
        (*ret)
            .is_reloc_section = Some(
            elf_i386_is_reloc_section
                as unsafe extern "C" fn(*const libc::c_char) -> bool,
        );
        (*ret)
            .sizeof_reloc = ::core::mem::size_of::<Elf32_External_Rel>() as libc::c_ulong
            as libc::c_uint;
        (*ret).got_entry_size = 4 as libc::c_int as libc::c_uint;
        (*ret).set_pcrel_plt(0 as libc::c_int as libc::c_uint);
        (*ret).pointer_r_type = R_386_32 as libc::c_int as libc::c_uint;
        (*ret)
            .dynamic_interpreter = b"/usr/lib/libc.so.1\0" as *const u8
            as *const libc::c_char;
        (*ret)
            .dynamic_interpreter_size = ::core::mem::size_of::<[libc::c_char; 19]>()
            as libc::c_ulong as libc::c_int;
        (*ret).tls_get_addr = b"___tls_get_addr\0" as *const u8 as *const libc::c_char;
    }
    (*ret)
        .loc_hash_table = htab_try_create(
        1024 as libc::c_int as size_t,
        Some(
            _bfd_x86_elf_local_htab_hash
                as unsafe extern "C" fn(*const libc::c_void) -> hashval_t,
        ),
        Some(
            _bfd_x86_elf_local_htab_eq
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
        None,
    );
    (*ret).loc_hash_memory = objalloc_create() as *mut libc::c_void;
    if ((*ret).loc_hash_table).is_null() || ((*ret).loc_hash_memory).is_null() {
        elf_x86_link_hash_table_free(abfd);
        return 0 as *mut bfd_link_hash_table;
    }
    (*ret)
        .elf
        .root
        .hash_table_free = Some(
        elf_x86_link_hash_table_free as unsafe extern "C" fn(*mut bfd) -> (),
    );
    return &mut (*ret).elf.root;
}
#[no_mangle]
pub unsafe extern "C" fn _bfd_x86_elf_compare_relocs(
    mut ap: *const libc::c_void,
    mut bp: *const libc::c_void,
) -> libc::c_int {
    let mut a: *const arelent = *(ap as *mut *const arelent);
    let mut b: *const arelent = *(bp as *mut *const arelent);
    if (*a).address > (*b).address {
        return 1 as libc::c_int
    } else if (*a).address < (*b).address {
        return -(1 as libc::c_int)
    } else {
        return 0 as libc::c_int
    };
}
unsafe extern "C" fn elf_x86_linker_defined(
    mut info: *mut bfd_link_info,
    mut name: *const libc::c_char,
) {
    let mut h: *mut elf_link_hash_entry = 0 as *mut elf_link_hash_entry;
    h = elf_link_hash_lookup(
        elf_hash_table(info),
        name,
        0 as libc::c_int != 0,
        0 as libc::c_int != 0,
        0 as libc::c_int != 0,
    );
    if h.is_null() {
        return;
    }
    while ((*h).root).type_0() as libc::c_int == bfd_link_hash_indirect as libc::c_int {
        h = (*h).root.u.i.link as *mut elf_link_hash_entry;
    }
    if ((*h).root).type_0() as libc::c_int == bfd_link_hash_new as libc::c_int
        || ((*h).root).type_0() as libc::c_int == bfd_link_hash_undefined as libc::c_int
        || ((*h).root).type_0() as libc::c_int == bfd_link_hash_undefweak as libc::c_int
        || ((*h).root).type_0() as libc::c_int == bfd_link_hash_common as libc::c_int
        || (*h).def_regular() == 0 && (*h).def_dynamic() as libc::c_int != 0
    {
        let ref mut fresh0 = *(h as *mut elf_x86_link_hash_entry);
        (*fresh0).set_local_ref(2 as libc::c_int as libc::c_uint);
        let ref mut fresh1 = *(h as *mut elf_x86_link_hash_entry);
        (*fresh1).set_linker_def(1 as libc::c_int as libc::c_uint);
    }
}
unsafe extern "C" fn elf_x86_hide_linker_defined(
    mut info: *mut bfd_link_info,
    mut name: *const libc::c_char,
) {
    let mut h: *mut elf_link_hash_entry = 0 as *mut elf_link_hash_entry;
    h = elf_link_hash_lookup(
        elf_hash_table(info),
        name,
        0 as libc::c_int != 0,
        0 as libc::c_int != 0,
        0 as libc::c_int != 0,
    );
    if h.is_null() {
        return;
    }
    while ((*h).root).type_0() as libc::c_int == bfd_link_hash_indirect as libc::c_int {
        h = (*h).root.u.i.link as *mut elf_link_hash_entry;
    }
    if (*h).other() as libc::c_int & 0x3 as libc::c_int == 1 as libc::c_int
        || (*h).other() as libc::c_int & 0x3 as libc::c_int == 2 as libc::c_int
    {
        _bfd_elf_link_hash_hide_symbol(info, h, 1 as libc::c_int != 0);
    }
}
#[no_mangle]
pub unsafe extern "C" fn _bfd_x86_elf_link_check_relocs(
    mut abfd: *mut bfd,
    mut info: *mut bfd_link_info,
) -> bool {
    if !((*info).type_0() as libc::c_int == type_relocatable as libc::c_int) {
        let mut htab: *mut elf_x86_link_hash_table = 0 as *mut elf_x86_link_hash_table;
        let mut bed: *const elf_backend_data = (*(*abfd).xvec).backend_data
            as *const elf_backend_data;
        htab = if is_elf_hash_table((*info).hash) as libc::c_int != 0
            && elf_hash_table_id((*info).hash as *mut elf_link_hash_table)
                as libc::c_uint == (*bed).target_id as libc::c_uint
        {
            (*info).hash as *mut elf_x86_link_hash_table
        } else {
            0 as *mut elf_x86_link_hash_table
        };
        if !htab.is_null() {
            let mut h: *mut elf_link_hash_entry = 0 as *mut elf_link_hash_entry;
            h = elf_link_hash_lookup(
                elf_hash_table(info),
                (*htab).tls_get_addr,
                0 as libc::c_int != 0,
                0 as libc::c_int != 0,
                0 as libc::c_int != 0,
            );
            if !h.is_null() {
                let ref mut fresh2 = *(h as *mut elf_x86_link_hash_entry);
                (*fresh2).set_tls_get_addr(1 as libc::c_int as libc::c_uint);
                while ((*h).root).type_0() as libc::c_int
                    == bfd_link_hash_indirect as libc::c_int
                {
                    h = (*h).root.u.i.link as *mut elf_link_hash_entry;
                    let ref mut fresh3 = *(h as *mut elf_x86_link_hash_entry);
                    (*fresh3).set_tls_get_addr(1 as libc::c_int as libc::c_uint);
                }
            }
            elf_x86_linker_defined(
                info,
                b"__ehdr_start\0" as *const u8 as *const libc::c_char,
            );
            if (*info).type_0() as libc::c_int == type_pde as libc::c_int
                || (*info).type_0() as libc::c_int == type_pie as libc::c_int
            {
                elf_x86_linker_defined(
                    info,
                    b"__bss_start\0" as *const u8 as *const libc::c_char,
                );
                elf_x86_linker_defined(
                    info,
                    b"_end\0" as *const u8 as *const libc::c_char,
                );
                elf_x86_linker_defined(
                    info,
                    b"_edata\0" as *const u8 as *const libc::c_char,
                );
            } else {
                elf_x86_hide_linker_defined(
                    info,
                    b"__bss_start\0" as *const u8 as *const libc::c_char,
                );
                elf_x86_hide_linker_defined(
                    info,
                    b"_end\0" as *const u8 as *const libc::c_char,
                );
                elf_x86_hide_linker_defined(
                    info,
                    b"_edata\0" as *const u8 as *const libc::c_char,
                );
            }
        }
    }
    return _bfd_elf_link_check_relocs(abfd, info);
}
#[no_mangle]
pub unsafe extern "C" fn _bfd_elf_x86_valid_reloc_p(
    mut input_section: *mut asection,
    mut info: *mut bfd_link_info,
    mut htab: *mut elf_x86_link_hash_table,
    mut rel: *const Elf_Internal_Rela,
    mut h: *mut elf_link_hash_entry,
    mut sym: *mut Elf_Internal_Sym,
    mut symtab_hdr: *mut Elf_Internal_Shdr,
    mut no_dynreloc_p: *mut bool,
) -> bool {
    let mut valid_p: bool = 1 as libc::c_int != 0;
    *no_dynreloc_p = 0 as libc::c_int != 0;
    if ((*info).type_0() as libc::c_int == type_dll as libc::c_int
        || (*info).type_0() as libc::c_int == type_pie as libc::c_int)
        && (h.is_null()
            || _bfd_elf_symbol_refs_local_p(h, info, 0 as libc::c_int != 0)
                as libc::c_int != 0)
    {
        let mut bed: *const elf_backend_data = 0 as *const elf_backend_data;
        let mut r_type: libc::c_uint = 0;
        let mut irel: Elf_Internal_Rela = Elf_Internal_Rela {
            r_offset: 0,
            r_info: 0,
            r_addend: 0,
        };
        if !h.is_null() {
            if !((((*h).root).type_0() as libc::c_int
                == bfd_link_hash_defined as libc::c_int
                || ((*h).root).type_0() as libc::c_int
                    == bfd_link_hash_defweak as libc::c_int)
                && bfd_is_abs_section((*h).root.u.def.section) as libc::c_int != 0
                && ((*h).root).rel_from_abs() == 0 && ((*h).root).ldscript_def() == 0)
            {
                return valid_p;
            }
        } else if (*sym).st_shndx != (0xf as libc::c_uint).wrapping_neg() {
            return valid_p
        }
        bed = (*(*(*input_section).owner).xvec).backend_data as *const elf_backend_data;
        r_type = ((*rel).r_info & 0xff as libc::c_int as libc::c_ulong) as libc::c_uint;
        irel = *rel;
        if (*bed).target_id as libc::c_uint
            == X86_64_ELF_DATA as libc::c_int as libc::c_uint
        {
            r_type &= !((1 as libc::c_int) << 7 as libc::c_int) as libc::c_uint;
            valid_p = r_type == R_X86_64_64 as libc::c_int as libc::c_uint
                || r_type == R_X86_64_32 as libc::c_int as libc::c_uint
                || r_type == R_X86_64_32S as libc::c_int as libc::c_uint
                || r_type == R_X86_64_16 as libc::c_int as libc::c_uint
                || r_type == R_X86_64_8 as libc::c_int as libc::c_uint
                || r_type == R_X86_64_GOTPCREL as libc::c_int as libc::c_uint
                || r_type == R_X86_64_GOTPCRELX as libc::c_int as libc::c_uint
                || r_type == R_X86_64_REX_GOTPCRELX as libc::c_int as libc::c_uint;
            if !valid_p {
                let mut r_symndx: libc::c_uint = ((*htab).r_sym)
                    .expect("non-null function pointer")((*rel).r_info) as libc::c_uint;
                irel
                    .r_info = ((*htab).r_info)
                    .expect(
                        "non-null function pointer",
                    )(r_symndx as bfd_vma, r_type as bfd_vma);
            }
        } else {
            valid_p = r_type == R_386_32 as libc::c_int as libc::c_uint
                || r_type == R_386_16 as libc::c_int as libc::c_uint
                || r_type == R_386_8 as libc::c_int as libc::c_uint;
        }
        if valid_p {
            *no_dynreloc_p = 1 as libc::c_int != 0;
        } else {
            let mut name: *const libc::c_char = 0 as *const libc::c_char;
            let mut internal_reloc: arelent = arelent {
                sym_ptr_ptr: 0 as *mut *mut bfd_symbol,
                address: 0,
                addend: 0,
                howto: 0 as *const reloc_howto_type,
            };
            if !((*bed).elf_info_to_howto)
                .expect(
                    "non-null function pointer",
                )((*input_section).owner, &mut internal_reloc, &mut irel)
                || (internal_reloc.howto).is_null()
            {
                _bfd_abort(
                    b"elfxx-x86.c\0" as *const u8 as *const libc::c_char,
                    971 as libc::c_int,
                    (*::core::mem::transmute::<
                        &[u8; 210],
                        &[libc::c_char; 210],
                    >(
                        b"_Bool _bfd_elf_x86_valid_reloc_p(asection *, struct bfd_link_info *, struct elf_x86_link_hash_table *, const Elf_Internal_Rela *, struct elf_link_hash_entry *, Elf_Internal_Sym *, Elf_Internal_Shdr *, _Bool *)\0",
                    ))
                        .as_ptr(),
                );
            }
            if !h.is_null() {
                name = (*h).root.root.string;
            } else {
                name = bfd_elf_sym_name(
                    (*input_section).owner,
                    symtab_hdr,
                    sym,
                    0 as *mut asection,
                );
            }
            ((*(*info).callbacks).einfo)
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    b"bfd\0" as *const u8 as *const libc::c_char,
                    b"%F%P: %pB: relocation %s against absolute symbol `%s' in section `%pA' is disallowed\n\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*input_section).owner,
                (*internal_reloc.howto).name,
                name,
                input_section,
            );
            bfd_set_error(bfd_error_bad_value);
        }
    }
    return valid_p;
}
#[no_mangle]
pub unsafe extern "C" fn _bfd_x86_elf_size_dynamic_sections(
    mut output_bfd: *mut bfd,
    mut info: *mut bfd_link_info,
) -> bool {
    let mut htab: *mut elf_x86_link_hash_table = 0 as *mut elf_x86_link_hash_table;
    let mut dynobj: *mut bfd = 0 as *mut bfd;
    let mut s: *mut asection = 0 as *mut asection;
    let mut relocs: bool = false;
    let mut ibfd: *mut bfd = 0 as *mut bfd;
    let mut bed: *const elf_backend_data = (*(*output_bfd).xvec).backend_data
        as *const elf_backend_data;
    htab = if is_elf_hash_table((*info).hash) as libc::c_int != 0
        && elf_hash_table_id((*info).hash as *mut elf_link_hash_table) as libc::c_uint
            == (*bed).target_id as libc::c_uint
    {
        (*info).hash as *mut elf_x86_link_hash_table
    } else {
        0 as *mut elf_x86_link_hash_table
    };
    if htab.is_null() {
        return 0 as libc::c_int != 0;
    }
    dynobj = (*htab).elf.dynobj;
    if dynobj.is_null() {
        _bfd_abort(
            b"elfxx-x86.c\0" as *const u8 as *const libc::c_char,
            1010 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 72],
                &[libc::c_char; 72],
            >(
                b"_Bool _bfd_x86_elf_size_dynamic_sections(bfd *, struct bfd_link_info *)\0",
            ))
                .as_ptr(),
        );
    }
    ibfd = (*info).input_bfds;
    while !ibfd.is_null() {
        let mut local_got: *mut bfd_signed_vma = 0 as *mut bfd_signed_vma;
        let mut end_local_got: *mut bfd_signed_vma = 0 as *mut bfd_signed_vma;
        let mut local_tls_type: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut local_tlsdesc_gotent: *mut bfd_vma = 0 as *mut bfd_vma;
        let mut locsymcount: bfd_size_type = 0;
        let mut symtab_hdr: *mut Elf_Internal_Shdr = 0 as *mut Elf_Internal_Shdr;
        let mut srel: *mut asection = 0 as *mut asection;
        if bfd_get_flavour(ibfd) as libc::c_uint
            == bfd_target_elf_flavour as libc::c_int as libc::c_uint
            && !((*ibfd).tdata.elf_obj_data).is_null()
            && (*(*ibfd).tdata.elf_obj_data).object_id() as libc::c_uint
                == (*htab).elf.hash_table_id as libc::c_uint
        {
            s = (*ibfd).sections;
            while !s.is_null() {
                let mut p: *mut elf_dyn_relocs = 0 as *mut elf_dyn_relocs;
                p = (*((*s).used_by_bfd as *mut bfd_elf_section_data)).local_dynrel
                    as *mut elf_dyn_relocs;
                while !p.is_null() {
                    if !(!bfd_is_abs_section((*p).sec)
                        && bfd_is_abs_section((*(*p).sec).output_section) as libc::c_int
                            != 0)
                    {
                        if !((*htab).elf.target_os as libc::c_uint
                            == is_vxworks as libc::c_int as libc::c_uint
                            && strcmp(
                                (*(*(*p).sec).output_section).name,
                                b".tls_vars\0" as *const u8 as *const libc::c_char,
                            ) == 0 as libc::c_int)
                        {
                            if (*p).count != 0 as libc::c_int as libc::c_ulong {
                                srel = (*((*(*p).sec).used_by_bfd
                                    as *mut bfd_elf_section_data))
                                    .sreloc;
                                (*srel)
                                    .size = ((*srel).size as libc::c_ulong)
                                    .wrapping_add(
                                        ((*p).count)
                                            .wrapping_mul((*htab).sizeof_reloc as libc::c_ulong),
                                    ) as bfd_size_type as bfd_size_type;
                                if (*(*(*p).sec).output_section).flags
                                    & 0x8 as libc::c_int as libc::c_uint
                                    != 0 as libc::c_int as libc::c_uint
                                    && (*info).flags
                                        & ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_ulong
                                        == 0 as libc::c_int as libc::c_ulong
                                {
                                    (*info).flags
                                        |= ((1 as libc::c_int) << 2 as libc::c_int)
                                            as libc::c_ulong;
                                    if (*info).textrel_check() as libc::c_int
                                        != textrel_check_none as libc::c_int
                                    {
                                        ((*(*info).callbacks).einfo)
                                            .expect(
                                                "non-null function pointer",
                                            )(
                                            dcgettext(
                                                b"bfd\0" as *const u8 as *const libc::c_char,
                                                b"%P: %pB: warning: relocation in read-only section `%pA'\n\0"
                                                    as *const u8 as *const libc::c_char,
                                                5 as libc::c_int,
                                            ),
                                            (*(*p).sec).owner,
                                            (*p).sec,
                                        );
                                    }
                                }
                            }
                        }
                    }
                    p = (*p).next;
                }
                s = (*s).next;
            }
            local_got = (*(*ibfd).tdata.elf_obj_data).local_got.refcounts;
            if !local_got.is_null() {
                symtab_hdr = &mut (*(*ibfd).tdata.elf_obj_data).symtab_hdr;
                locsymcount = (*symtab_hdr).sh_info as bfd_size_type;
                end_local_got = local_got.offset(locsymcount as isize);
                local_tls_type = (*((*ibfd).tdata.any as *mut elf_x86_obj_tdata))
                    .local_got_tls_type;
                local_tlsdesc_gotent = (*((*ibfd).tdata.any as *mut elf_x86_obj_tdata))
                    .local_tlsdesc_gotent;
                s = (*htab).elf.sgot;
                srel = (*htab).elf.srelgot;
                while local_got < end_local_got {
                    *local_tlsdesc_gotent = -(1 as libc::c_int) as bfd_vma;
                    if *local_got > 0 as libc::c_int as libc::c_long {
                        if *local_tls_type as libc::c_int == 8 as libc::c_int
                            || *local_tls_type as libc::c_int
                                == 2 as libc::c_int | 8 as libc::c_int
                        {
                            *local_tlsdesc_gotent = ((*(*htab).elf.sgotplt).size)
                                .wrapping_sub(
                                    ((*(*htab).elf.srelplt).reloc_count)
                                        .wrapping_mul((*htab).got_entry_size) as libc::c_ulong,
                                );
                            (*(*htab).elf.sgotplt)
                                .size = ((*(*htab).elf.sgotplt).size as libc::c_ulong)
                                .wrapping_add(
                                    (2 as libc::c_int as libc::c_uint)
                                        .wrapping_mul((*htab).got_entry_size) as libc::c_ulong,
                                ) as bfd_size_type as bfd_size_type;
                            *local_got = -(2 as libc::c_int) as bfd_vma
                                as bfd_signed_vma;
                        }
                        if !(*local_tls_type as libc::c_int == 8 as libc::c_int
                            || *local_tls_type as libc::c_int
                                == 2 as libc::c_int | 8 as libc::c_int)
                            || (*local_tls_type as libc::c_int == 2 as libc::c_int
                                || *local_tls_type as libc::c_int
                                    == 2 as libc::c_int | 8 as libc::c_int)
                        {
                            *local_got = (*s).size as bfd_signed_vma;
                            (*s)
                                .size = ((*s).size as libc::c_ulong)
                                .wrapping_add((*htab).got_entry_size as libc::c_ulong)
                                as bfd_size_type as bfd_size_type;
                            if *local_tls_type as libc::c_int == 2 as libc::c_int
                                || *local_tls_type as libc::c_int
                                    == 2 as libc::c_int | 8 as libc::c_int
                                || *local_tls_type as libc::c_int == 7 as libc::c_int
                            {
                                (*s)
                                    .size = ((*s).size as libc::c_ulong)
                                    .wrapping_add((*htab).got_entry_size as libc::c_ulong)
                                    as bfd_size_type as bfd_size_type;
                            }
                        }
                        if ((*info).type_0() as libc::c_int == type_dll as libc::c_int
                            || (*info).type_0() as libc::c_int
                                == type_pie as libc::c_int)
                            && *local_tls_type as libc::c_int != 9 as libc::c_int
                            || (*local_tls_type as libc::c_int == 2 as libc::c_int
                                || *local_tls_type as libc::c_int
                                    == 2 as libc::c_int | 8 as libc::c_int
                                || (*local_tls_type as libc::c_int == 8 as libc::c_int
                                    || *local_tls_type as libc::c_int
                                        == 2 as libc::c_int | 8 as libc::c_int))
                            || *local_tls_type as libc::c_int & 4 as libc::c_int != 0
                        {
                            if *local_tls_type as libc::c_int == 7 as libc::c_int {
                                (*srel)
                                    .size = ((*srel).size as libc::c_ulong)
                                    .wrapping_add(
                                        (2 as libc::c_int as libc::c_uint)
                                            .wrapping_mul((*htab).sizeof_reloc) as libc::c_ulong,
                                    ) as bfd_size_type as bfd_size_type;
                            } else if *local_tls_type as libc::c_int == 2 as libc::c_int
                                || *local_tls_type as libc::c_int
                                    == 2 as libc::c_int | 8 as libc::c_int
                                || !(*local_tls_type as libc::c_int == 8 as libc::c_int
                                    || *local_tls_type as libc::c_int
                                        == 2 as libc::c_int | 8 as libc::c_int)
                            {
                                (*srel)
                                    .size = ((*srel).size as libc::c_ulong)
                                    .wrapping_add((*htab).sizeof_reloc as libc::c_ulong)
                                    as bfd_size_type as bfd_size_type;
                            }
                            if *local_tls_type as libc::c_int == 8 as libc::c_int
                                || *local_tls_type as libc::c_int
                                    == 2 as libc::c_int | 8 as libc::c_int
                            {
                                (*(*htab).elf.srelplt)
                                    .size = ((*(*htab).elf.srelplt).size as libc::c_ulong)
                                    .wrapping_add((*htab).sizeof_reloc as libc::c_ulong)
                                    as bfd_size_type as bfd_size_type;
                                if (*bed).target_id as libc::c_uint
                                    == X86_64_ELF_DATA as libc::c_int as libc::c_uint
                                {
                                    (*htab).elf.tlsdesc_plt = -(1 as libc::c_int) as bfd_vma;
                                }
                            }
                        }
                    } else {
                        *local_got = -(1 as libc::c_int) as bfd_vma as bfd_signed_vma;
                    }
                    local_got = local_got.offset(1);
                    local_got;
                    local_tls_type = local_tls_type.offset(1);
                    local_tls_type;
                    local_tlsdesc_gotent = local_tlsdesc_gotent.offset(1);
                    local_tlsdesc_gotent;
                }
            }
        }
        ibfd = (*ibfd).link.next;
    }
    if (*htab).tls_ld_or_ldm_got.refcount > 0 as libc::c_int as libc::c_long {
        (*htab).tls_ld_or_ldm_got.offset = (*(*htab).elf.sgot).size;
        (*(*htab).elf.sgot)
            .size = ((*(*htab).elf.sgot).size as libc::c_ulong)
            .wrapping_add(
                (2 as libc::c_int as libc::c_uint).wrapping_mul((*htab).got_entry_size)
                    as libc::c_ulong,
            ) as bfd_size_type as bfd_size_type;
        (*(*htab).elf.srelgot)
            .size = ((*(*htab).elf.srelgot).size as libc::c_ulong)
            .wrapping_add((*htab).sizeof_reloc as libc::c_ulong) as bfd_size_type
            as bfd_size_type;
    } else {
        (*htab).tls_ld_or_ldm_got.offset = -(1 as libc::c_int) as bfd_vma;
    }
    elf_link_hash_traverse(
        &mut (*htab).elf,
        Some(
            elf_x86_allocate_dynrelocs
                as unsafe extern "C" fn(
                    *mut elf_link_hash_entry,
                    *mut libc::c_void,
                ) -> bool,
        ),
        info as *mut libc::c_void,
    );
    htab_traverse(
        (*htab).loc_hash_table,
        Some(
            elf_x86_allocate_local_dynreloc
                as unsafe extern "C" fn(
                    *mut *mut libc::c_void,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
        info as *mut libc::c_void,
    );
    if !((*htab).elf.srelplt).is_null() {
        (*htab).next_tls_desc_index = (*(*htab).elf.srelplt).reloc_count as bfd_vma;
        (*htab)
            .sgotplt_jump_table_size = ((*(*htab).elf.srelplt).reloc_count)
            .wrapping_mul((*htab).got_entry_size) as bfd_vma;
        (*htab)
            .next_irelative_index = ((*(*htab).elf.srelplt).reloc_count)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as bfd_vma;
    } else if !((*htab).elf.irelplt).is_null() {
        (*htab)
            .next_irelative_index = ((*(*htab).elf.irelplt).reloc_count)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as bfd_vma;
    }
    if (*htab).elf.tlsdesc_plt != 0 {
        if (*info).flags & ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_ulong != 0
        {
            (*htab).elf.tlsdesc_plt = 0 as libc::c_int as bfd_vma;
        } else {
            (*htab).elf.tlsdesc_got = (*(*htab).elf.sgot).size;
            (*(*htab).elf.sgot)
                .size = ((*(*htab).elf.sgot).size as libc::c_ulong)
                .wrapping_add((*htab).got_entry_size as libc::c_ulong) as bfd_size_type
                as bfd_size_type;
            if (*(*htab).elf.splt).size == 0 as libc::c_int as libc::c_ulong {
                (*(*htab).elf.splt).size = (*htab).plt.plt_entry_size as bfd_size_type;
            }
            (*htab).elf.tlsdesc_plt = (*(*htab).elf.splt).size;
            (*(*htab).elf.splt)
                .size = ((*(*htab).elf.splt).size as libc::c_ulong)
                .wrapping_add((*htab).plt.plt_entry_size as libc::c_ulong)
                as bfd_size_type as bfd_size_type;
        }
    }
    if !((*htab).elf.sgotplt).is_null() {
        if (((*htab).elf.hgot).is_null() || (*htab).got_referenced() == 0)
            && (*(*htab).elf.sgotplt).size == (*bed).got_header_size
            && (((*htab).elf.splt).is_null()
                || (*(*htab).elf.splt).size == 0 as libc::c_int as libc::c_ulong)
            && (((*htab).elf.sgot).is_null()
                || (*(*htab).elf.sgot).size == 0 as libc::c_int as libc::c_ulong)
            && (((*htab).elf.iplt).is_null()
                || (*(*htab).elf.iplt).size == 0 as libc::c_int as libc::c_ulong)
            && (((*htab).elf.igotplt).is_null()
                || (*(*htab).elf.igotplt).size == 0 as libc::c_int as libc::c_ulong)
        {
            (*(*htab).elf.sgotplt).size = 0 as libc::c_int as bfd_size_type;
            if !((*htab).elf.hgot).is_null()
                && (*htab).elf.target_os as libc::c_uint
                    != is_solaris as libc::c_int as libc::c_uint
            {
                ((*(*htab).elf.hgot).root).set_type_0(bfd_link_hash_undefined);
                (*(*htab).elf.hgot)
                    .root
                    .u
                    .undef
                    .abfd = (*(*(*htab).elf.hgot).root.u.def.section).owner;
                ((*(*htab).elf.hgot).root)
                    .set_linker_def(0 as libc::c_int as libc::c_uint);
                (*(*htab).elf.hgot).set_ref_regular(0 as libc::c_int as libc::c_uint);
                (*(*htab).elf.hgot).set_def_regular(0 as libc::c_int as libc::c_uint);
            }
        }
    }
    if _bfd_elf_eh_frame_present(info) {
        if !((*htab).plt_eh_frame).is_null() && !((*htab).elf.splt).is_null()
            && (*(*htab).elf.splt).size != 0 as libc::c_int as libc::c_ulong
            && !bfd_is_abs_section((*(*htab).elf.splt).output_section)
        {
            (*(*htab).plt_eh_frame)
                .size = (*htab).plt.eh_frame_plt_size as bfd_size_type;
        }
        if !((*htab).plt_got_eh_frame).is_null() && !((*htab).plt_got).is_null()
            && (*(*htab).plt_got).size != 0 as libc::c_int as libc::c_ulong
            && !bfd_is_abs_section((*(*htab).plt_got).output_section)
        {
            (*(*htab).plt_got_eh_frame)
                .size = (*(*htab).non_lazy_plt).eh_frame_plt_size as bfd_size_type;
        }
        if !((*htab).plt_second_eh_frame).is_null() && !((*htab).plt_second).is_null()
            && (*(*htab).plt_second).size != 0 as libc::c_int as libc::c_ulong
            && !bfd_is_abs_section((*(*htab).plt_second).output_section)
        {
            (*(*htab).plt_second_eh_frame)
                .size = (*(*htab).non_lazy_plt).eh_frame_plt_size as bfd_size_type;
        }
    }
    relocs = 0 as libc::c_int != 0;
    let mut current_block_126: u64;
    s = (*dynobj).sections;
    while !s.is_null() {
        let mut strip_section: bool = 1 as libc::c_int != 0;
        if !((*s).flags & 0x100000 as libc::c_int as libc::c_uint
            == 0 as libc::c_int as libc::c_uint)
        {
            if s == (*htab).elf.splt || s == (*htab).elf.sgot {
                if !((*htab).elf.hplt).is_null() {
                    strip_section = 0 as libc::c_int != 0;
                }
                current_block_126 = 16593409533420678784;
            } else if s == (*htab).elf.sgotplt || s == (*htab).elf.iplt
                || s == (*htab).elf.igotplt || s == (*htab).plt_second
                || s == (*htab).plt_got || s == (*htab).plt_eh_frame
                || s == (*htab).plt_got_eh_frame || s == (*htab).plt_second_eh_frame
                || s == (*htab).elf.sdynbss || s == (*htab).elf.sdynrelro
            {
                current_block_126 = 16593409533420678784;
            } else if ((*htab).is_reloc_section)
                .expect("non-null function pointer")(bfd_section_name(s))
            {
                if (*s).size != 0 as libc::c_int as libc::c_ulong
                    && s != (*htab).elf.srelplt && s != (*htab).srelplt2
                {
                    relocs = 1 as libc::c_int != 0;
                }
                if s != (*htab).elf.srelplt {
                    (*s).reloc_count = 0 as libc::c_int as libc::c_uint;
                }
                current_block_126 = 16593409533420678784;
            } else {
                current_block_126 = 17995254032144898061;
            }
            match current_block_126 {
                17995254032144898061 => {}
                _ => {
                    if (*s).size == 0 as libc::c_int as libc::c_ulong {
                        if strip_section {
                            (*s).flags |= 0x8000 as libc::c_int as libc::c_uint;
                        }
                    } else if !((*s).flags & 0x100 as libc::c_int as libc::c_uint
                        == 0 as libc::c_int as libc::c_uint)
                    {
                        if s == (*htab).elf.iplt {
                            bfd_set_section_alignment(s, (*htab).plt.iplt_alignment);
                        }
                        (*s)
                            .contents = bfd_zalloc(dynobj, (*s).size)
                            as *mut libc::c_uchar;
                        if ((*s).contents).is_null() {
                            return 0 as libc::c_int != 0;
                        }
                    }
                }
            }
        }
        s = (*s).next;
    }
    if !((*htab).plt_eh_frame).is_null() && !((*(*htab).plt_eh_frame).contents).is_null()
    {
        memcpy(
            (*(*htab).plt_eh_frame).contents as *mut libc::c_void,
            (*htab).plt.eh_frame_plt as *const libc::c_void,
            (*(*htab).plt_eh_frame).size,
        );
        (Some(((*(*dynobj).xvec).bfd_putx32).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            (*(*htab).elf.splt).size,
            ((*(*htab).plt_eh_frame).contents)
                .offset(4 as libc::c_int as isize)
                .offset(20 as libc::c_int as isize)
                .offset(12 as libc::c_int as isize) as *mut libc::c_void,
        );
    }
    if !((*htab).plt_got_eh_frame).is_null()
        && !((*(*htab).plt_got_eh_frame).contents).is_null()
    {
        memcpy(
            (*(*htab).plt_got_eh_frame).contents as *mut libc::c_void,
            (*(*htab).non_lazy_plt).eh_frame_plt as *const libc::c_void,
            (*(*htab).plt_got_eh_frame).size,
        );
        (Some(((*(*dynobj).xvec).bfd_putx32).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            (*(*htab).plt_got).size,
            ((*(*htab).plt_got_eh_frame).contents)
                .offset(4 as libc::c_int as isize)
                .offset(20 as libc::c_int as isize)
                .offset(12 as libc::c_int as isize) as *mut libc::c_void,
        );
    }
    if !((*htab).plt_second_eh_frame).is_null()
        && !((*(*htab).plt_second_eh_frame).contents).is_null()
    {
        memcpy(
            (*(*htab).plt_second_eh_frame).contents as *mut libc::c_void,
            (*(*htab).non_lazy_plt).eh_frame_plt as *const libc::c_void,
            (*(*htab).plt_second_eh_frame).size,
        );
        (Some(((*(*dynobj).xvec).bfd_putx32).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            (*(*htab).plt_second).size,
            ((*(*htab).plt_second_eh_frame).contents)
                .offset(4 as libc::c_int as isize)
                .offset(20 as libc::c_int as isize)
                .offset(12 as libc::c_int as isize) as *mut libc::c_void,
        );
    }
    return _bfd_elf_maybe_vxworks_add_dynamic_tags(output_bfd, info, relocs);
}
#[no_mangle]
pub unsafe extern "C" fn _bfd_x86_elf_finish_dynamic_sections(
    mut output_bfd: *mut bfd,
    mut info: *mut bfd_link_info,
) -> *mut elf_x86_link_hash_table {
    let mut htab: *mut elf_x86_link_hash_table = 0 as *mut elf_x86_link_hash_table;
    let mut bed: *const elf_backend_data = 0 as *const elf_backend_data;
    let mut dynobj: *mut bfd = 0 as *mut bfd;
    let mut sdyn: *mut asection = 0 as *mut asection;
    let mut dyncon: *mut bfd_byte = 0 as *mut bfd_byte;
    let mut dynconend: *mut bfd_byte = 0 as *mut bfd_byte;
    let mut sizeof_dyn: bfd_size_type = 0;
    bed = (*(*output_bfd).xvec).backend_data as *const elf_backend_data;
    htab = if is_elf_hash_table((*info).hash) as libc::c_int != 0
        && elf_hash_table_id((*info).hash as *mut elf_link_hash_table) as libc::c_uint
            == (*bed).target_id as libc::c_uint
    {
        (*info).hash as *mut elf_x86_link_hash_table
    } else {
        0 as *mut elf_x86_link_hash_table
    };
    if htab.is_null() {
        return htab;
    }
    dynobj = (*htab).elf.dynobj;
    sdyn = bfd_get_linker_section(
        dynobj,
        b".dynamic\0" as *const u8 as *const libc::c_char,
    );
    if !((*htab).elf.sgotplt).is_null()
        && (*(*htab).elf.sgotplt).size > 0 as libc::c_int as libc::c_ulong
    {
        let mut dynamic_addr: bfd_vma = 0;
        if bfd_is_abs_section((*(*htab).elf.sgotplt).output_section) {
            _bfd_error_handler(
                dcgettext(
                    b"bfd\0" as *const u8 as *const libc::c_char,
                    b"discarded output section: `%pA'\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*htab).elf.sgotplt,
            );
            return 0 as *mut elf_x86_link_hash_table;
        }
        (*((*(*(*htab).elf.sgotplt).output_section).used_by_bfd
            as *mut bfd_elf_section_data))
            .this_hdr
            .sh_entsize = (*htab).got_entry_size as bfd_size_type;
        dynamic_addr = if sdyn.is_null() {
            0 as libc::c_int as bfd_vma
        } else {
            ((*(*sdyn).output_section).vma).wrapping_add((*sdyn).output_offset)
        };
        if (*htab).got_entry_size == 8 as libc::c_int as libc::c_uint {
            (Some(
                ((*(*output_bfd).xvec).bfd_putx64).expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(dynamic_addr, (*(*htab).elf.sgotplt).contents as *mut libc::c_void);
            (Some(
                ((*(*output_bfd).xvec).bfd_putx64).expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(
                0 as libc::c_int as bfd_vma,
                ((*(*htab).elf.sgotplt).contents).offset(8 as libc::c_int as isize)
                    as *mut libc::c_void,
            );
            (Some(
                ((*(*output_bfd).xvec).bfd_putx64).expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(
                0 as libc::c_int as bfd_vma,
                ((*(*htab).elf.sgotplt).contents)
                    .offset((8 as libc::c_int * 2 as libc::c_int) as isize)
                    as *mut libc::c_void,
            );
        } else {
            (Some(
                ((*(*output_bfd).xvec).bfd_putx32).expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(dynamic_addr, (*(*htab).elf.sgotplt).contents as *mut libc::c_void);
            (Some(
                ((*(*output_bfd).xvec).bfd_putx32).expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(
                0 as libc::c_int as bfd_vma,
                ((*(*htab).elf.sgotplt).contents).offset(4 as libc::c_int as isize)
                    as *mut libc::c_void,
            );
            (Some(
                ((*(*output_bfd).xvec).bfd_putx32).expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(
                0 as libc::c_int as bfd_vma,
                ((*(*htab).elf.sgotplt).contents)
                    .offset((4 as libc::c_int * 2 as libc::c_int) as isize)
                    as *mut libc::c_void,
            );
        }
    }
    if !(*htab).elf.dynamic_sections_created {
        return htab;
    }
    if sdyn.is_null() || ((*htab).elf.sgot).is_null() {
        _bfd_abort(
            b"elfxx-x86.c\0" as *const u8 as *const libc::c_char,
            1435 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 100],
                &[libc::c_char; 100],
            >(
                b"struct elf_x86_link_hash_table *_bfd_x86_elf_finish_dynamic_sections(bfd *, struct bfd_link_info *)\0",
            ))
                .as_ptr(),
        );
    }
    sizeof_dyn = (*(*bed).s).sizeof_dyn as bfd_size_type;
    dyncon = (*sdyn).contents;
    dynconend = ((*sdyn).contents).offset((*sdyn).size as isize);
    let mut current_block_40: u64;
    while dyncon < dynconend {
        let mut dyn_0: Elf_Internal_Dyn = Elf_Internal_Dyn {
            d_tag: 0,
            d_un: C2RustUnnamed_21 { d_val: 0 },
        };
        let mut s: *mut asection = 0 as *mut asection;
        (Some(((*(*bed).s).swap_dyn_in).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(dynobj, dyncon as *const libc::c_void, &mut dyn_0);
        match dyn_0.d_tag {
            3 => {
                s = (*htab).elf.sgotplt;
                dyn_0
                    .d_un
                    .d_ptr = ((*(*s).output_section).vma)
                    .wrapping_add((*s).output_offset);
                current_block_40 = 14832935472441733737;
            }
            23 => {
                dyn_0.d_un.d_ptr = (*(*(*htab).elf.srelplt).output_section).vma;
                current_block_40 = 14832935472441733737;
            }
            2 => {
                s = (*(*htab).elf.srelplt).output_section;
                dyn_0.d_un.d_val = (*s).size;
                current_block_40 = 14832935472441733737;
            }
            1879047926 => {
                s = (*htab).elf.splt;
                dyn_0
                    .d_un
                    .d_ptr = ((*(*s).output_section).vma)
                    .wrapping_add((*s).output_offset)
                    .wrapping_add((*htab).elf.tlsdesc_plt);
                current_block_40 = 14832935472441733737;
            }
            1879047927 => {
                s = (*htab).elf.sgot;
                dyn_0
                    .d_un
                    .d_ptr = ((*(*s).output_section).vma)
                    .wrapping_add((*s).output_offset)
                    .wrapping_add((*htab).elf.tlsdesc_got);
                current_block_40 = 14832935472441733737;
            }
            _ => {
                if (*htab).elf.target_os as libc::c_uint
                    == is_vxworks as libc::c_int as libc::c_uint
                    && elf_vxworks_finish_dynamic_entry(output_bfd, &mut dyn_0)
                        as libc::c_int != 0
                {
                    current_block_40 = 14832935472441733737;
                } else {
                    current_block_40 = 10652014663920648156;
                }
            }
        }
        match current_block_40 {
            14832935472441733737 => {
                (Some(((*(*bed).s).swap_dyn_out).expect("non-null function pointer")))
                    .expect(
                        "non-null function pointer",
                    )(output_bfd, &mut dyn_0, dyncon as *mut libc::c_void);
            }
            _ => {}
        }
        dyncon = dyncon.offset(sizeof_dyn as isize);
    }
    if !((*htab).plt_got).is_null()
        && (*(*htab).plt_got).size > 0 as libc::c_int as libc::c_ulong
    {
        (*((*(*(*htab).plt_got).output_section).used_by_bfd
            as *mut bfd_elf_section_data))
            .this_hdr
            .sh_entsize = (*(*htab).non_lazy_plt).plt_entry_size as bfd_size_type;
    }
    if !((*htab).plt_second).is_null()
        && (*(*htab).plt_second).size > 0 as libc::c_int as libc::c_ulong
    {
        (*((*(*(*htab).plt_second).output_section).used_by_bfd
            as *mut bfd_elf_section_data))
            .this_hdr
            .sh_entsize = (*(*htab).non_lazy_plt).plt_entry_size as bfd_size_type;
    }
    if !((*htab).plt_eh_frame).is_null() && !((*(*htab).plt_eh_frame).contents).is_null()
    {
        if !((*htab).elf.splt).is_null()
            && (*(*htab).elf.splt).size != 0 as libc::c_int as libc::c_ulong
            && (*(*htab).elf.splt).flags & 0x8000 as libc::c_int as libc::c_uint
                == 0 as libc::c_int as libc::c_uint
            && !((*(*htab).elf.splt).output_section).is_null()
            && !((*(*htab).plt_eh_frame).output_section).is_null()
        {
            let mut plt_start: bfd_vma = (*(*(*htab).elf.splt).output_section).vma;
            let mut eh_frame_start: bfd_vma = ((*(*(*htab).plt_eh_frame).output_section)
                .vma)
                .wrapping_add((*(*htab).plt_eh_frame).output_offset)
                .wrapping_add(4 as libc::c_int as libc::c_ulong)
                .wrapping_add(20 as libc::c_int as libc::c_ulong)
                .wrapping_add(8 as libc::c_int as libc::c_ulong);
            (Some(((*(*dynobj).xvec).bfd_putx32).expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                plt_start.wrapping_sub(eh_frame_start),
                ((*(*htab).plt_eh_frame).contents)
                    .offset(4 as libc::c_int as isize)
                    .offset(20 as libc::c_int as isize)
                    .offset(8 as libc::c_int as isize) as *mut libc::c_void,
            );
        }
        if (*(*htab).plt_eh_frame).sec_info_type() as libc::c_int == 3 as libc::c_int {
            if !_bfd_elf_write_section_eh_frame(
                output_bfd,
                info,
                (*htab).plt_eh_frame,
                (*(*htab).plt_eh_frame).contents,
            ) {
                return 0 as *mut elf_x86_link_hash_table;
            }
        }
    }
    if !((*htab).plt_got_eh_frame).is_null()
        && !((*(*htab).plt_got_eh_frame).contents).is_null()
    {
        if !((*htab).plt_got).is_null()
            && (*(*htab).plt_got).size != 0 as libc::c_int as libc::c_ulong
            && (*(*htab).plt_got).flags & 0x8000 as libc::c_int as libc::c_uint
                == 0 as libc::c_int as libc::c_uint
            && !((*(*htab).plt_got).output_section).is_null()
            && !((*(*htab).plt_got_eh_frame).output_section).is_null()
        {
            let mut plt_start_0: bfd_vma = (*(*(*htab).plt_got).output_section).vma;
            let mut eh_frame_start_0: bfd_vma = ((*(*(*htab).plt_got_eh_frame)
                .output_section)
                .vma)
                .wrapping_add((*(*htab).plt_got_eh_frame).output_offset)
                .wrapping_add(4 as libc::c_int as libc::c_ulong)
                .wrapping_add(20 as libc::c_int as libc::c_ulong)
                .wrapping_add(8 as libc::c_int as libc::c_ulong);
            (Some(((*(*dynobj).xvec).bfd_putx32).expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                plt_start_0.wrapping_sub(eh_frame_start_0),
                ((*(*htab).plt_got_eh_frame).contents)
                    .offset(4 as libc::c_int as isize)
                    .offset(20 as libc::c_int as isize)
                    .offset(8 as libc::c_int as isize) as *mut libc::c_void,
            );
        }
        if (*(*htab).plt_got_eh_frame).sec_info_type() as libc::c_int == 3 as libc::c_int
        {
            if !_bfd_elf_write_section_eh_frame(
                output_bfd,
                info,
                (*htab).plt_got_eh_frame,
                (*(*htab).plt_got_eh_frame).contents,
            ) {
                return 0 as *mut elf_x86_link_hash_table;
            }
        }
    }
    if !((*htab).plt_second_eh_frame).is_null()
        && !((*(*htab).plt_second_eh_frame).contents).is_null()
    {
        if !((*htab).plt_second).is_null()
            && (*(*htab).plt_second).size != 0 as libc::c_int as libc::c_ulong
            && (*(*htab).plt_second).flags & 0x8000 as libc::c_int as libc::c_uint
                == 0 as libc::c_int as libc::c_uint
            && !((*(*htab).plt_second).output_section).is_null()
            && !((*(*htab).plt_second_eh_frame).output_section).is_null()
        {
            let mut plt_start_1: bfd_vma = (*(*(*htab).plt_second).output_section).vma;
            let mut eh_frame_start_1: bfd_vma = ((*(*(*htab).plt_second_eh_frame)
                .output_section)
                .vma)
                .wrapping_add((*(*htab).plt_second_eh_frame).output_offset)
                .wrapping_add(4 as libc::c_int as libc::c_ulong)
                .wrapping_add(20 as libc::c_int as libc::c_ulong)
                .wrapping_add(8 as libc::c_int as libc::c_ulong);
            (Some(((*(*dynobj).xvec).bfd_putx32).expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                plt_start_1.wrapping_sub(eh_frame_start_1),
                ((*(*htab).plt_second_eh_frame).contents)
                    .offset(4 as libc::c_int as isize)
                    .offset(20 as libc::c_int as isize)
                    .offset(8 as libc::c_int as isize) as *mut libc::c_void,
            );
        }
        if (*(*htab).plt_second_eh_frame).sec_info_type() as libc::c_int
            == 3 as libc::c_int
        {
            if !_bfd_elf_write_section_eh_frame(
                output_bfd,
                info,
                (*htab).plt_second_eh_frame,
                (*(*htab).plt_second_eh_frame).contents,
            ) {
                return 0 as *mut elf_x86_link_hash_table;
            }
        }
    }
    if !((*htab).elf.sgot).is_null()
        && (*(*htab).elf.sgot).size > 0 as libc::c_int as libc::c_ulong
    {
        (*((*(*(*htab).elf.sgot).output_section).used_by_bfd
            as *mut bfd_elf_section_data))
            .this_hdr
            .sh_entsize = (*htab).got_entry_size as bfd_size_type;
    }
    return htab;
}
#[no_mangle]
pub unsafe extern "C" fn _bfd_x86_elf_always_size_sections(
    mut output_bfd: *mut bfd,
    mut info: *mut bfd_link_info,
) -> bool {
    let mut tls_sec: *mut asection = (*elf_hash_table(info)).tls_sec;
    if !tls_sec.is_null() {
        let mut tlsbase: *mut elf_link_hash_entry = 0 as *mut elf_link_hash_entry;
        tlsbase = elf_link_hash_lookup(
            elf_hash_table(info),
            b"_TLS_MODULE_BASE_\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int != 0,
            0 as libc::c_int != 0,
            0 as libc::c_int != 0,
        );
        if !tlsbase.is_null() && (*tlsbase).type_0() as libc::c_int == 6 as libc::c_int {
            let mut htab: *mut elf_x86_link_hash_table = 0
                as *mut elf_x86_link_hash_table;
            let mut bh: *mut bfd_link_hash_entry = 0 as *mut bfd_link_hash_entry;
            let mut bed: *const elf_backend_data = (*(*output_bfd).xvec).backend_data
                as *const elf_backend_data;
            htab = if is_elf_hash_table((*info).hash) as libc::c_int != 0
                && elf_hash_table_id((*info).hash as *mut elf_link_hash_table)
                    as libc::c_uint == (*bed).target_id as libc::c_uint
            {
                (*info).hash as *mut elf_x86_link_hash_table
            } else {
                0 as *mut elf_x86_link_hash_table
            };
            if htab.is_null() {
                return 0 as libc::c_int != 0;
            }
            if !_bfd_generic_link_add_one_symbol(
                info,
                output_bfd,
                b"_TLS_MODULE_BASE_\0" as *const u8 as *const libc::c_char,
                ((1 as libc::c_int) << 0 as libc::c_int) as flagword,
                tls_sec,
                0 as libc::c_int as bfd_vma,
                0 as *const libc::c_char,
                0 as libc::c_int != 0,
                (*bed).collect() != 0,
                &mut bh,
            ) {
                return 0 as libc::c_int != 0;
            }
            (*htab).tls_module_base = bh;
            tlsbase = bh as *mut elf_link_hash_entry;
            (*tlsbase).set_def_regular(1 as libc::c_int as libc::c_uint);
            (*tlsbase).set_other(2 as libc::c_int as libc::c_uint);
            ((*tlsbase).root).set_linker_def(1 as libc::c_int as libc::c_uint);
            (Some(((*bed).elf_backend_hide_symbol).expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(info, tlsbase, 1 as libc::c_int != 0);
        }
    }
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn _bfd_x86_elf_merge_symbol_attribute(
    mut h: *mut elf_link_hash_entry,
    mut st_other: libc::c_uint,
    mut definition: bool,
    mut _dynamic: bool,
) {
    if definition {
        let mut eh: *mut elf_x86_link_hash_entry = h as *mut elf_x86_link_hash_entry;
        (*eh)
            .set_def_protected(
                (st_other & 0x3 as libc::c_int as libc::c_uint
                    == 3 as libc::c_int as libc::c_uint) as libc::c_int as libc::c_uint,
            );
    }
}
#[no_mangle]
pub unsafe extern "C" fn _bfd_x86_elf_copy_indirect_symbol(
    mut info: *mut bfd_link_info,
    mut dir: *mut elf_link_hash_entry,
    mut ind: *mut elf_link_hash_entry,
) {
    let mut edir: *mut elf_x86_link_hash_entry = 0 as *mut elf_x86_link_hash_entry;
    let mut eind: *mut elf_x86_link_hash_entry = 0 as *mut elf_x86_link_hash_entry;
    edir = dir as *mut elf_x86_link_hash_entry;
    eind = ind as *mut elf_x86_link_hash_entry;
    if ((*ind).root).type_0() as libc::c_int == bfd_link_hash_indirect as libc::c_int
        && (*dir).got.refcount <= 0 as libc::c_int as libc::c_long
    {
        (*edir).tls_type = (*eind).tls_type;
        (*eind).tls_type = 0 as libc::c_int as libc::c_uchar;
    }
    (*edir)
        .set_gotoff_ref(
            (*edir).gotoff_ref() | (*eind).gotoff_ref() as libc::c_int as libc::c_uint,
        );
    (*edir)
        .set_zero_undefweak(
            (*edir).zero_undefweak()
                | (*eind).zero_undefweak() as libc::c_int as libc::c_uint,
        );
    if 1 as libc::c_int != 0
        && ((*ind).root).type_0() as libc::c_int != bfd_link_hash_indirect as libc::c_int
        && (*dir).dynamic_adjusted() as libc::c_int != 0
    {
        if (*dir).versioned() as libc::c_int != versioned_hidden as libc::c_int {
            (*dir)
                .set_ref_dynamic(
                    (*dir).ref_dynamic()
                        | (*ind).ref_dynamic() as libc::c_int as libc::c_uint,
                );
        }
        (*dir)
            .set_ref_regular(
                (*dir).ref_regular()
                    | (*ind).ref_regular() as libc::c_int as libc::c_uint,
            );
        (*dir)
            .set_ref_regular_nonweak(
                (*dir).ref_regular_nonweak()
                    | (*ind).ref_regular_nonweak() as libc::c_int as libc::c_uint,
            );
        (*dir)
            .set_needs_plt(
                (*dir).needs_plt() | (*ind).needs_plt() as libc::c_int as libc::c_uint,
            );
        (*dir)
            .set_pointer_equality_needed(
                (*dir).pointer_equality_needed()
                    | (*ind).pointer_equality_needed() as libc::c_int as libc::c_uint,
            );
    } else {
        _bfd_elf_link_hash_copy_indirect(info, dir, ind);
    };
}
#[no_mangle]
pub unsafe extern "C" fn _bfd_x86_elf_fixup_symbol(
    mut info: *mut bfd_link_info,
    mut h: *mut elf_link_hash_entry,
) -> bool {
    if (*h).dynindx != -(1 as libc::c_int) as libc::c_long
        && (((*(h as *mut elf_x86_link_hash_entry)).elf.root).type_0() as libc::c_int
            == bfd_link_hash_undefweak as libc::c_int
            && (_bfd_x86_elf_link_symbol_references_local(
                info,
                &mut (*(h as *mut elf_x86_link_hash_entry)).elf,
            ) as libc::c_int != 0
                || ((*info).type_0() as libc::c_int == type_pde as libc::c_int
                    || (*info).type_0() as libc::c_int == type_pie as libc::c_int)
                    && (*(h as *mut elf_x86_link_hash_entry)).zero_undefweak()
                        as libc::c_int > 0 as libc::c_int))
    {
        (*h).dynindx = -(1 as libc::c_int) as libc::c_long;
        _bfd_elf_strtab_delref((*elf_hash_table(info)).dynstr, (*h).dynstr_index);
    }
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn _bfd_x86_elf_link_fixup_ifunc_symbol(
    mut info: *mut bfd_link_info,
    mut htab: *mut elf_x86_link_hash_table,
    mut h: *mut elf_link_hash_entry,
    mut sym: *mut Elf_Internal_Sym,
) {
    if (*info).type_0() as libc::c_int == type_pde as libc::c_int
        && (*h).def_regular() as libc::c_int != 0
        && (*h).dynindx != -(1 as libc::c_int) as libc::c_long
        && (*h).plt.offset != -(1 as libc::c_int) as bfd_vma
        && (*h).type_0() as libc::c_int == 10 as libc::c_int
    {
        let mut plt_s: *mut asection = 0 as *mut asection;
        let mut plt_offset: bfd_vma = 0;
        let mut output_bfd: *mut bfd = (*info).output_bfd;
        if !((*htab).plt_second).is_null() {
            let mut eh: *mut elf_x86_link_hash_entry = h as *mut elf_x86_link_hash_entry;
            plt_s = (*htab).plt_second;
            plt_offset = (*eh).plt_second.offset;
        } else {
            plt_s = (*htab).elf.splt;
            plt_offset = (*h).plt.offset;
        }
        (*sym).st_size = 0 as libc::c_int as bfd_vma;
        (*sym)
            .st_info = (((*sym).st_info as libc::c_uint >> 4 as libc::c_int)
            << 4 as libc::c_int)
            .wrapping_add((2 as libc::c_int & 0xf as libc::c_int) as libc::c_uint)
            as libc::c_uchar;
        (*sym)
            .st_shndx = _bfd_elf_section_from_bfd_section(
            output_bfd,
            (*plt_s).output_section,
        );
        (*sym)
            .st_value = ((*(*plt_s).output_section).vma)
            .wrapping_add((*plt_s).output_offset)
            .wrapping_add(plt_offset);
    }
}
#[no_mangle]
pub unsafe extern "C" fn _bfd_x86_elf_link_report_relative_reloc(
    mut info: *mut bfd_link_info,
    mut asect: *mut asection,
    mut h: *mut elf_link_hash_entry,
    mut sym: *mut Elf_Internal_Sym,
    mut reloc_name: *const libc::c_char,
    mut reloc: *const libc::c_void,
) {
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut abfd: *mut bfd = 0 as *mut bfd;
    let mut rel: *const Elf_Internal_Rela = reloc as *const Elf_Internal_Rela;
    let mut r_offset: [libc::c_char; 30] = [0; 30];
    let mut r_info: [libc::c_char; 30] = [0; 30];
    if (*asect).flags & 0x100000 as libc::c_int as libc::c_uint
        != 0 as libc::c_int as libc::c_uint
    {
        abfd = (*info).output_bfd;
    } else {
        abfd = (*asect).owner;
    }
    if !h.is_null() && !((*h).root.root.string).is_null() {
        name = (*h).root.root.string;
    } else {
        name = bfd_elf_sym_name(
            abfd,
            &mut (*(*abfd).tdata.elf_obj_data).symtab_hdr,
            sym,
            0 as *mut asection,
        );
    }
    bfd_sprintf_vma(abfd, r_offset.as_mut_ptr(), (*rel).r_offset);
    bfd_sprintf_vma(abfd, r_info.as_mut_ptr(), (*rel).r_info);
    if (*asect).use_rela_p() != 0 {
        let mut r_addend: [libc::c_char; 30] = [0; 30];
        bfd_sprintf_vma(abfd, r_addend.as_mut_ptr(), (*rel).r_addend);
        ((*(*info).callbacks).einfo)
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                b"bfd\0" as *const u8 as *const libc::c_char,
                b"%pB: %s (offset: 0x%s, info: 0x%s, addend: 0x%s) against '%s' for section '%pA' in %pB\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            (*info).output_bfd,
            reloc_name,
            r_offset.as_mut_ptr(),
            r_info.as_mut_ptr(),
            r_addend.as_mut_ptr(),
            name,
            asect,
            abfd,
        );
    } else {
        ((*(*info).callbacks).einfo)
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                b"bfd\0" as *const u8 as *const libc::c_char,
                b"%pB: %s (offset: 0x%s, info: 0x%s) against '%s' for section '%pA' in %pB\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            (*info).output_bfd,
            reloc_name,
            r_offset.as_mut_ptr(),
            r_info.as_mut_ptr(),
            name,
            asect,
            abfd,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn _bfd_x86_elf_hash_symbol(
    mut h: *mut elf_link_hash_entry,
) -> bool {
    if (*h).plt.offset != -(1 as libc::c_int) as bfd_vma && (*h).def_regular() == 0
        && (*h).pointer_equality_needed() == 0
    {
        return 0 as libc::c_int != 0;
    }
    return _bfd_elf_hash_symbol(h);
}
#[no_mangle]
pub unsafe extern "C" fn _bfd_x86_elf_adjust_dynamic_symbol(
    mut info: *mut bfd_link_info,
    mut h: *mut elf_link_hash_entry,
) -> bool {
    let mut htab: *mut elf_x86_link_hash_table = 0 as *mut elf_x86_link_hash_table;
    let mut s: *mut asection = 0 as *mut asection;
    let mut srel: *mut asection = 0 as *mut asection;
    let mut eh: *mut elf_x86_link_hash_entry = 0 as *mut elf_x86_link_hash_entry;
    let mut p: *mut elf_dyn_relocs = 0 as *mut elf_dyn_relocs;
    let mut bed: *const elf_backend_data = (*(*(*info).output_bfd).xvec).backend_data
        as *const elf_backend_data;
    eh = h as *mut elf_x86_link_hash_entry;
    if (*h).type_0() as libc::c_int == 10 as libc::c_int {
        if (*h).ref_regular() as libc::c_int != 0
            && _bfd_elf_symbol_refs_local_p(h, info, 1 as libc::c_int != 0)
                as libc::c_int != 0
        {
            let mut pc_count: bfd_size_type = 0 as libc::c_int as bfd_size_type;
            let mut count: bfd_size_type = 0 as libc::c_int as bfd_size_type;
            let mut pp: *mut *mut elf_dyn_relocs = 0 as *mut *mut elf_dyn_relocs;
            eh = h as *mut elf_x86_link_hash_entry;
            pp = &mut (*h).dyn_relocs;
            loop {
                p = *pp;
                if p.is_null() {
                    break;
                }
                pc_count = (pc_count as libc::c_ulong).wrapping_add((*p).pc_count)
                    as bfd_size_type as bfd_size_type;
                (*p)
                    .count = ((*p).count as libc::c_ulong).wrapping_sub((*p).pc_count)
                    as bfd_size_type as bfd_size_type;
                (*p).pc_count = 0 as libc::c_int as bfd_size_type;
                count = (count as libc::c_ulong).wrapping_add((*p).count)
                    as bfd_size_type as bfd_size_type;
                if (*p).count == 0 as libc::c_int as libc::c_ulong {
                    *pp = (*p).next;
                } else {
                    pp = &mut (*p).next;
                }
            }
            if pc_count != 0 || count != 0 {
                (*h).set_non_got_ref(1 as libc::c_int as libc::c_uint);
                if pc_count != 0 {
                    (*h).set_needs_plt(1 as libc::c_int as libc::c_uint);
                    if (*h).plt.refcount <= 0 as libc::c_int as libc::c_long {
                        (*h).plt.refcount = 1 as libc::c_int as bfd_signed_vma;
                    } else {
                        (*h).plt.refcount += 1 as libc::c_int as libc::c_long;
                    }
                }
            }
            if (*eh).gotoff_ref() != 0 {
                (*h).plt.refcount = 1 as libc::c_int as bfd_signed_vma;
            }
        }
        if (*h).plt.refcount <= 0 as libc::c_int as libc::c_long {
            (*h).plt.offset = -(1 as libc::c_int) as bfd_vma;
            (*h).set_needs_plt(0 as libc::c_int as libc::c_uint);
        }
        return 1 as libc::c_int != 0;
    }
    if (*h).type_0() as libc::c_int == 2 as libc::c_int
        || (*h).needs_plt() as libc::c_int != 0
    {
        if (*h).plt.refcount <= 0 as libc::c_int as libc::c_long
            || _bfd_elf_symbol_refs_local_p(h, info, 1 as libc::c_int != 0)
                as libc::c_int != 0
            || (*h).other() as libc::c_int & 0x3 as libc::c_int != 0 as libc::c_int
                && ((*h).root).type_0() as libc::c_int
                    == bfd_link_hash_undefweak as libc::c_int
        {
            (*h).plt.offset = -(1 as libc::c_int) as bfd_vma;
            (*h).set_needs_plt(0 as libc::c_int as libc::c_uint);
        }
        return 1 as libc::c_int != 0;
    } else {
        (*h).plt.offset = -(1 as libc::c_int) as bfd_vma;
    }
    if (*h).is_weakalias() != 0 {
        let mut def: *mut elf_link_hash_entry = weakdef(h);
        if !(((*def).root).type_0() as libc::c_int
            == bfd_link_hash_defined as libc::c_int)
        {
            bfd_assert(
                b"elfxx-x86.c\0" as *const u8 as *const libc::c_char,
                1915 as libc::c_int,
            );
        }
        (*h).root.u.def.section = (*def).root.u.def.section;
        (*h).root.u.def.value = (*def).root.u.def.value;
        if 1 as libc::c_int != 0 || (*info).nocopyreloc() as libc::c_int != 0
            || (*eh).def_protected() as libc::c_int != 0
                && (((*eh).elf.root).type_0() as libc::c_int
                    == bfd_link_hash_defined as libc::c_int
                    || ((*eh).elf.root).type_0() as libc::c_int
                        == bfd_link_hash_defweak as libc::c_int)
                && (*(*(*(*eh).elf.root.u.def.section).owner).tdata.elf_obj_data)
                    .has_no_copy_on_protected() as libc::c_int != 0
                && (*(*(*eh).elf.root.u.def.section).owner).flags
                    & 0x40 as libc::c_int as libc::c_uint
                    != 0 as libc::c_int as libc::c_uint
                && (*(*eh).elf.root.u.def.section).flags
                    & 0x10 as libc::c_int as libc::c_uint
                    == 0 as libc::c_int as libc::c_uint
        {
            (*h).set_non_got_ref((*def).non_got_ref());
            (*eh).set_needs_copy((*def).needs_copy());
        }
        return 1 as libc::c_int != 0;
    }
    if !((*info).type_0() as libc::c_int == type_pde as libc::c_int
        || (*info).type_0() as libc::c_int == type_pie as libc::c_int)
    {
        return 1 as libc::c_int != 0;
    }
    if (*h).non_got_ref() == 0 && (*eh).gotoff_ref() == 0 {
        return 1 as libc::c_int != 0;
    }
    if (*info).nocopyreloc() as libc::c_int != 0
        || (*eh).def_protected() as libc::c_int != 0
            && (((*eh).elf.root).type_0() as libc::c_int
                == bfd_link_hash_defined as libc::c_int
                || ((*eh).elf.root).type_0() as libc::c_int
                    == bfd_link_hash_defweak as libc::c_int)
            && (*(*(*(*eh).elf.root.u.def.section).owner).tdata.elf_obj_data)
                .has_no_copy_on_protected() as libc::c_int != 0
            && (*(*(*eh).elf.root.u.def.section).owner).flags
                & 0x40 as libc::c_int as libc::c_uint != 0 as libc::c_int as libc::c_uint
            && (*(*eh).elf.root.u.def.section).flags
                & 0x10 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint
    {
        (*h).set_non_got_ref(0 as libc::c_int as libc::c_uint);
        return 1 as libc::c_int != 0;
    }
    htab = if is_elf_hash_table((*info).hash) as libc::c_int != 0
        && elf_hash_table_id((*info).hash as *mut elf_link_hash_table) as libc::c_uint
            == (*bed).target_id as libc::c_uint
    {
        (*info).hash as *mut elf_x86_link_hash_table
    } else {
        0 as *mut elf_x86_link_hash_table
    };
    if htab.is_null() {
        return 0 as libc::c_int != 0;
    }
    if 1 as libc::c_int != 0
        && ((*bed).target_id as libc::c_uint
            == X86_64_ELF_DATA as libc::c_int as libc::c_uint
            || (*eh).gotoff_ref() == 0
                && (*htab).elf.target_os as libc::c_uint
                    != is_vxworks as libc::c_int as libc::c_uint)
    {
        if (_bfd_elf_readonly_dynrelocs(h)).is_null() {
            (*h).set_non_got_ref(0 as libc::c_int as libc::c_uint);
            return 1 as libc::c_int != 0;
        }
    }
    if (*(*h).root.u.def.section).flags & 0x8 as libc::c_int as libc::c_uint
        != 0 as libc::c_int as libc::c_uint
    {
        s = (*htab).elf.sdynrelro;
        srel = (*htab).elf.sreldynrelro;
    } else {
        s = (*htab).elf.sdynbss;
        srel = (*htab).elf.srelbss;
    }
    if (*(*h).root.u.def.section).flags & 0x1 as libc::c_int as libc::c_uint
        != 0 as libc::c_int as libc::c_uint
        && (*h).size != 0 as libc::c_int as libc::c_ulong
    {
        (*srel)
            .size = ((*srel).size as libc::c_ulong)
            .wrapping_add((*htab).sizeof_reloc as libc::c_ulong) as bfd_size_type
            as bfd_size_type;
        (*h).set_needs_copy(1 as libc::c_int as libc::c_uint);
    }
    return _bfd_elf_adjust_dynamic_copy(info, h, s);
}
#[no_mangle]
pub unsafe extern "C" fn _bfd_x86_elf_hide_symbol(
    mut info: *mut bfd_link_info,
    mut h: *mut elf_link_hash_entry,
    mut force_local: bool,
) {
    if ((*h).root).type_0() as libc::c_int == bfd_link_hash_undefweak as libc::c_int
        && (*info).nointerp() as libc::c_int != 0
        && (*info).type_0() as libc::c_int == type_pie as libc::c_int
    {
        let mut eh: *mut elf_x86_link_hash_entry = h as *mut elf_x86_link_hash_entry;
        if (*h).plt.refcount > 0 as libc::c_int as libc::c_long
            || (*eh).plt_got.refcount > 0 as libc::c_int as libc::c_long
        {
            return;
        }
    }
    _bfd_elf_link_hash_hide_symbol(info, h, force_local);
}
#[no_mangle]
pub unsafe extern "C" fn _bfd_x86_elf_link_symbol_references_local(
    mut info: *mut bfd_link_info,
    mut h: *mut elf_link_hash_entry,
) -> bool {
    let mut eh: *mut elf_x86_link_hash_entry = h as *mut elf_x86_link_hash_entry;
    let mut htab: *mut elf_x86_link_hash_table = (*info).hash
        as *mut elf_x86_link_hash_table;
    if (*eh).local_ref() as libc::c_int > 1 as libc::c_int {
        return 1 as libc::c_int != 0;
    }
    if (*eh).local_ref() as libc::c_int == 1 as libc::c_int {
        return 0 as libc::c_int != 0;
    }
    if _bfd_elf_symbol_refs_local_p(h, info, 0 as libc::c_int != 0) as libc::c_int != 0
        || ((*h).root).type_0() as libc::c_int == bfd_link_hash_undefweak as libc::c_int
            && ((*h).other() as libc::c_int & 0x3 as libc::c_int != 0 as libc::c_int
                || ((*info).type_0() as libc::c_int == type_pde as libc::c_int
                    || (*info).type_0() as libc::c_int == type_pie as libc::c_int)
                    && ((*htab).interp).is_null()
                || (*info).dynamic_undefined_weak == 0 as libc::c_int)
        || ((*h).def_regular() as libc::c_int != 0
            || (*h).def_regular() == 0 && (*h).def_dynamic() == 0
                && ((*h).root).type_0() as libc::c_int
                    == bfd_link_hash_defined as libc::c_int)
            && !((*info).version_info).is_null()
            && _bfd_elf_link_hide_sym_by_version(info, h) as libc::c_int != 0
    {
        (*eh).set_local_ref(2 as libc::c_int as libc::c_uint);
        return 1 as libc::c_int != 0;
    }
    (*eh).set_local_ref(1 as libc::c_int as libc::c_uint);
    return 0 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn _bfd_x86_elf_gc_mark_hook(
    mut sec: *mut asection,
    mut info: *mut bfd_link_info,
    mut rel: *mut Elf_Internal_Rela,
    mut h: *mut elf_link_hash_entry,
    mut sym: *mut Elf_Internal_Sym,
) -> *mut asection {
    if R_X86_64_GNU_VTINHERIT as libc::c_int as libc::c_uint
        != R_386_GNU_VTINHERIT as libc::c_int as libc::c_uint
        || R_X86_64_GNU_VTENTRY as libc::c_int as libc::c_uint
            != R_386_GNU_VTENTRY as libc::c_int as libc::c_uint
    {
        _bfd_abort(
            b"elfxx-x86.c\0" as *const u8 as *const libc::c_char,
            2087 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 143],
                &[libc::c_char; 143],
            >(
                b"asection *_bfd_x86_elf_gc_mark_hook(asection *, struct bfd_link_info *, Elf_Internal_Rela *, struct elf_link_hash_entry *, Elf_Internal_Sym *)\0",
            ))
                .as_ptr(),
        );
    }
    if !h.is_null() {
        match (*rel).r_info & 0xff as libc::c_int as libc::c_ulong {
            250 | 251 => return 0 as *mut asection,
            _ => {}
        }
    }
    return _bfd_elf_gc_mark_hook(sec, info, rel, h, sym);
}
unsafe extern "C" fn elf_i386_get_plt_got_vma(
    mut _plt_p: *mut elf_x86_plt,
    mut off: bfd_vma,
    mut _offset: bfd_vma,
    mut got_addr: bfd_vma,
) -> bfd_vma {
    return got_addr.wrapping_add(off);
}
unsafe extern "C" fn elf_x86_64_get_plt_got_vma(
    mut plt_p: *mut elf_x86_plt,
    mut off: bfd_vma,
    mut offset: bfd_vma,
    mut _got_addr: bfd_vma,
) -> bfd_vma {
    return ((*(*plt_p).sec).vma)
        .wrapping_add(offset)
        .wrapping_add(off)
        .wrapping_add((*plt_p).plt_got_insn_size as libc::c_ulong);
}
unsafe extern "C" fn elf_i386_valid_plt_reloc_p(mut type_0: libc::c_uint) -> bool {
    return type_0 == R_386_JUMP_SLOT as libc::c_int as libc::c_uint
        || type_0 == R_386_GLOB_DAT as libc::c_int as libc::c_uint
        || type_0 == R_386_IRELATIVE as libc::c_int as libc::c_uint;
}
unsafe extern "C" fn elf_x86_64_valid_plt_reloc_p(mut type_0: libc::c_uint) -> bool {
    return type_0 == R_X86_64_JUMP_SLOT as libc::c_int as libc::c_uint
        || type_0 == R_X86_64_GLOB_DAT as libc::c_int as libc::c_uint
        || type_0 == R_X86_64_IRELATIVE as libc::c_int as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn _bfd_x86_elf_get_synthetic_symtab(
    mut abfd: *mut bfd,
    mut count: libc::c_long,
    mut relsize: libc::c_long,
    mut got_addr: bfd_vma,
    mut plts: *mut elf_x86_plt,
    mut dynsyms: *mut *mut asymbol,
    mut ret: *mut *mut asymbol,
) -> libc::c_long {
    let mut current_block: u64;
    let mut size: libc::c_long = 0;
    let mut i: libc::c_long = 0;
    let mut n: libc::c_long = 0;
    let mut len: libc::c_long = 0;
    let mut j: libc::c_int = 0;
    let mut plt_got_offset: libc::c_uint = 0;
    let mut plt_entry_size: libc::c_uint = 0;
    let mut s: *mut asymbol = 0 as *mut asymbol;
    let mut plt_contents: *mut bfd_byte = 0 as *mut bfd_byte;
    let mut dynrelcount: libc::c_long = 0;
    let mut dynrelbuf: *mut *mut arelent = 0 as *mut *mut arelent;
    let mut p: *mut arelent = 0 as *mut arelent;
    let mut names: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut bed: *const elf_backend_data = 0 as *const elf_backend_data;
    let mut get_plt_got_vma: Option::<
        unsafe extern "C" fn(*mut elf_x86_plt, bfd_vma, bfd_vma, bfd_vma) -> bfd_vma,
    > = None;
    let mut valid_plt_reloc_p: Option::<unsafe extern "C" fn(libc::c_uint) -> bool> = None;
    dynrelbuf = 0 as *mut *mut arelent;
    if count == 0 as libc::c_int as libc::c_long {
        current_block = 9307817154712295799;
    } else {
        dynrelbuf = bfd_malloc(relsize as bfd_size_type) as *mut *mut arelent;
        if dynrelbuf.is_null() {
            current_block = 9307817154712295799;
        } else {
            dynrelcount = (Some(
                ((*(*abfd).xvec)._bfd_canonicalize_dynamic_reloc)
                    .expect("non-null function pointer"),
            ))
                .expect("non-null function pointer")(abfd, dynrelbuf, dynsyms);
            if dynrelcount <= 0 as libc::c_int as libc::c_long {
                current_block = 9307817154712295799;
            } else {
                qsort(
                    dynrelbuf as *mut libc::c_void,
                    dynrelcount as size_t,
                    ::core::mem::size_of::<*mut arelent>() as libc::c_ulong,
                    Some(
                        _bfd_x86_elf_compare_relocs
                            as unsafe extern "C" fn(
                                *const libc::c_void,
                                *const libc::c_void,
                            ) -> libc::c_int,
                    ),
                );
                size = (count as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<asymbol>() as libc::c_ulong)
                    as libc::c_long;
                n = 0 as libc::c_int as libc::c_long;
                i = 0 as libc::c_int as libc::c_long;
                while i < dynrelcount {
                    p = *dynrelbuf.offset(i as isize);
                    size = (size as libc::c_ulong)
                        .wrapping_add(
                            (strlen((**(*p).sym_ptr_ptr).name))
                                .wrapping_add(
                                    ::core::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong,
                                ),
                        ) as libc::c_long as libc::c_long;
                    if (*p).addend != 0 as libc::c_int as libc::c_ulong {
                        size = (size as libc::c_ulong)
                            .wrapping_add(
                                (::core::mem::size_of::<[libc::c_char; 4]>()
                                    as libc::c_ulong)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(
                                        (8 as libc::c_int
                                            * ((*(*((*(*abfd).xvec).backend_data
                                                as *const elf_backend_data))
                                                .s)
                                                .elfclass as libc::c_int == 2 as libc::c_int)
                                                as libc::c_int) as libc::c_ulong,
                                    ),
                            ) as libc::c_long as libc::c_long;
                    }
                    i += 1;
                    i;
                }
                *ret = bfd_zmalloc(size as bfd_size_type) as *mut asymbol;
                s = *ret;
                if s.is_null() {
                    current_block = 9307817154712295799;
                } else {
                    bed = (*(*abfd).xvec).backend_data as *const elf_backend_data;
                    if (*bed).target_id as libc::c_uint
                        == X86_64_ELF_DATA as libc::c_int as libc::c_uint
                    {
                        get_plt_got_vma = Some(
                            elf_x86_64_get_plt_got_vma
                                as unsafe extern "C" fn(
                                    *mut elf_x86_plt,
                                    bfd_vma,
                                    bfd_vma,
                                    bfd_vma,
                                ) -> bfd_vma,
                        );
                        valid_plt_reloc_p = Some(
                            elf_x86_64_valid_plt_reloc_p
                                as unsafe extern "C" fn(libc::c_uint) -> bool,
                        );
                        current_block = 2569451025026770673;
                    } else {
                        get_plt_got_vma = Some(
                            elf_i386_get_plt_got_vma
                                as unsafe extern "C" fn(
                                    *mut elf_x86_plt,
                                    bfd_vma,
                                    bfd_vma,
                                    bfd_vma,
                                ) -> bfd_vma,
                        );
                        valid_plt_reloc_p = Some(
                            elf_i386_valid_plt_reloc_p
                                as unsafe extern "C" fn(libc::c_uint) -> bool,
                        );
                        if got_addr != 0 {
                            let mut sec: *mut asection = bfd_get_section_by_name(
                                abfd,
                                b".got.plt\0" as *const u8 as *const libc::c_char,
                            );
                            if !sec.is_null() {
                                got_addr = (*sec).vma;
                            } else {
                                sec = bfd_get_section_by_name(
                                    abfd,
                                    b".got\0" as *const u8 as *const libc::c_char,
                                );
                                if !sec.is_null() {
                                    got_addr = (*sec).vma;
                                }
                            }
                            if got_addr == -(1 as libc::c_int) as bfd_vma {
                                current_block = 9307817154712295799;
                            } else {
                                current_block = 2569451025026770673;
                            }
                        } else {
                            current_block = 2569451025026770673;
                        }
                    }
                    match current_block {
                        9307817154712295799 => {}
                        _ => {
                            names = s.offset(count as isize) as *mut libc::c_char;
                            size = 0 as libc::c_int as libc::c_long;
                            n = 0 as libc::c_int as libc::c_long;
                            j = 0 as libc::c_int;
                            while !((*plts.offset(j as isize)).name).is_null() {
                                plt_contents = (*plts.offset(j as isize)).contents;
                                if !plt_contents.is_null() {
                                    let mut k: libc::c_long = 0;
                                    let mut offset: bfd_vma = 0;
                                    let mut plt: *mut asection = 0 as *mut asection;
                                    let mut plt_p: *mut elf_x86_plt = &mut *plts
                                        .offset(j as isize) as *mut elf_x86_plt;
                                    plt_got_offset = (*plt_p).plt_got_offset;
                                    plt_entry_size = (*plt_p).plt_entry_size;
                                    plt = (*plt_p).sec;
                                    if (*plt_p).type_0 as libc::c_int & plt_lazy as libc::c_int
                                        != 0
                                    {
                                        k = 1 as libc::c_int as libc::c_long;
                                        offset = plt_entry_size as bfd_vma;
                                    } else {
                                        k = 0 as libc::c_int as libc::c_long;
                                        offset = 0 as libc::c_int as bfd_vma;
                                    }
                                    while k < (*plt_p).count {
                                        let mut off: libc::c_int = 0;
                                        let mut got_vma: bfd_vma = 0;
                                        let mut min: libc::c_long = 0;
                                        let mut max: libc::c_long = 0;
                                        let mut mid: libc::c_long = 0;
                                        off = (Some(
                                            ((*(*abfd).xvec).bfd_h_getx32)
                                                .expect("non-null function pointer"),
                                        ))
                                            .expect(
                                                "non-null function pointer",
                                            )(
                                            plt_contents
                                                .offset(offset as isize)
                                                .offset(plt_got_offset as isize) as *const libc::c_void,
                                        ) as libc::c_int;
                                        got_vma = get_plt_got_vma
                                            .expect(
                                                "non-null function pointer",
                                            )(plt_p, off as bfd_vma, offset, got_addr);
                                        p = *dynrelbuf.offset(0 as libc::c_int as isize);
                                        min = 0 as libc::c_int as libc::c_long;
                                        max = dynrelcount;
                                        while (min + 1 as libc::c_int as libc::c_long) < max {
                                            let mut r: *mut arelent = 0 as *mut arelent;
                                            mid = (min + max) / 2 as libc::c_int as libc::c_long;
                                            r = *dynrelbuf.offset(mid as isize);
                                            if got_vma > (*r).address {
                                                min = mid;
                                            } else if got_vma < (*r).address {
                                                max = mid;
                                            } else {
                                                p = r;
                                                break;
                                            }
                                        }
                                        if got_vma == (*p).address && !((*p).howto).is_null()
                                            && valid_plt_reloc_p
                                                .expect("non-null function pointer")((*(*p).howto).type_0)
                                                as libc::c_int != 0
                                        {
                                            *s = **(*p).sym_ptr_ptr;
                                            if (*s).flags
                                                & ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint
                                                == 0 as libc::c_int as libc::c_uint
                                            {
                                                (*s).flags
                                                    |= ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uint;
                                            }
                                            (*s).flags
                                                |= ((1 as libc::c_int) << 21 as libc::c_int)
                                                    as libc::c_uint;
                                            (*s).flags
                                                &= !((1 as libc::c_int) << 8 as libc::c_int)
                                                    as libc::c_uint;
                                            (*s).section = plt;
                                            (*s).the_bfd = (*plt).owner;
                                            (*s).value = offset;
                                            (*s).udata.p = 0 as *mut libc::c_void;
                                            (*s).name = names;
                                            len = strlen((**(*p).sym_ptr_ptr).name) as libc::c_long;
                                            memcpy(
                                                names as *mut libc::c_void,
                                                (**(*p).sym_ptr_ptr).name as *const libc::c_void,
                                                len as libc::c_ulong,
                                            );
                                            names = names.offset(len as isize);
                                            if (*p).addend != 0 as libc::c_int as libc::c_ulong {
                                                let mut buf: [libc::c_char; 30] = [0; 30];
                                                let mut a: *mut libc::c_char = 0 as *mut libc::c_char;
                                                memcpy(
                                                    names as *mut libc::c_void,
                                                    b"+0x\0" as *const u8 as *const libc::c_char
                                                        as *const libc::c_void,
                                                    (::core::mem::size_of::<[libc::c_char; 4]>()
                                                        as libc::c_ulong)
                                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                                );
                                                names = names
                                                    .offset(
                                                        (::core::mem::size_of::<[libc::c_char; 4]>()
                                                            as libc::c_ulong)
                                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                                                    );
                                                bfd_sprintf_vma(abfd, buf.as_mut_ptr(), (*p).addend);
                                                a = buf.as_mut_ptr();
                                                while *a as libc::c_int == '0' as i32 {
                                                    a = a.offset(1);
                                                    a;
                                                }
                                                size = strlen(a) as libc::c_long;
                                                memcpy(
                                                    names as *mut libc::c_void,
                                                    a as *const libc::c_void,
                                                    size as libc::c_ulong,
                                                );
                                                names = names.offset(size as isize);
                                            }
                                            memcpy(
                                                names as *mut libc::c_void,
                                                b"@plt\0" as *const u8 as *const libc::c_char
                                                    as *const libc::c_void,
                                                ::core::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong,
                                            );
                                            names = names
                                                .offset(
                                                    ::core::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong
                                                        as isize,
                                                );
                                            n += 1;
                                            n;
                                            s = s.offset(1);
                                            s;
                                            (*p).howto = 0 as *const reloc_howto_type;
                                        }
                                        offset = (offset as libc::c_ulong)
                                            .wrapping_add(plt_entry_size as libc::c_ulong) as bfd_vma
                                            as bfd_vma;
                                        k += 1;
                                        k;
                                    }
                                }
                                j += 1;
                                j;
                            }
                            if n == 0 as libc::c_int as libc::c_long {
                                current_block = 9307817154712295799;
                            } else {
                                count = n;
                                current_block = 5722677567366458307;
                            }
                        }
                    }
                }
            }
        }
    }
    match current_block {
        9307817154712295799 => {
            count = -(1 as libc::c_int) as libc::c_long;
        }
        _ => {}
    }
    j = 0 as libc::c_int;
    while !((*plts.offset(j as isize)).name).is_null() {
        free((*plts.offset(j as isize)).contents as *mut libc::c_void);
        j += 1;
        j;
    }
    free(dynrelbuf as *mut libc::c_void);
    return count;
}
#[no_mangle]
pub unsafe extern "C" fn _bfd_x86_elf_parse_gnu_properties(
    mut abfd: *mut bfd,
    mut type_0: libc::c_uint,
    mut ptr: *mut bfd_byte,
    mut datasz: libc::c_uint,
) -> elf_property_kind {
    let mut prop: *mut elf_property = 0 as *mut elf_property;
    if type_0 == 0xc0000000 as libc::c_uint || type_0 == 0xc0000001 as libc::c_uint
        || type_0 >= 0xc0000002 as libc::c_uint && type_0 <= 0xc0007fff as libc::c_uint
        || type_0 >= 0xc0008000 as libc::c_uint && type_0 <= 0xc000ffff as libc::c_uint
        || type_0 >= 0xc0010000 as libc::c_uint && type_0 <= 0xc0017fff as libc::c_uint
    {
        if datasz != 4 as libc::c_int as libc::c_uint {
            _bfd_error_handler(
                dcgettext(
                    b"bfd\0" as *const u8 as *const libc::c_char,
                    b"error: %pB: <corrupt x86 property (0x%x) size: 0x%x>\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                abfd,
                type_0,
                datasz,
            );
            return property_corrupt;
        }
        prop = _bfd_elf_get_property(abfd, type_0, datasz);
        (*prop).u.number
            |= (Some(((*(*abfd).xvec).bfd_h_getx32).expect("non-null function pointer")))
                .expect("non-null function pointer")(ptr as *const libc::c_void);
        (*prop).pr_kind = property_number;
        return property_number;
    }
    return property_ignored;
}
#[no_mangle]
pub unsafe extern "C" fn _bfd_x86_elf_merge_gnu_properties(
    mut info: *mut bfd_link_info,
    mut _abfd: *mut bfd,
    mut _bbfd: *mut bfd,
    mut aprop: *mut elf_property,
    mut bprop: *mut elf_property,
) -> bool {
    let mut number: libc::c_uint = 0;
    let mut features: libc::c_uint = 0;
    let mut updated: bool = 0 as libc::c_int != 0;
    let mut bed: *const elf_backend_data = 0 as *const elf_backend_data;
    let mut htab: *mut elf_x86_link_hash_table = 0 as *mut elf_x86_link_hash_table;
    let mut pr_type: libc::c_uint = if !aprop.is_null() {
        (*aprop).pr_type
    } else {
        (*bprop).pr_type
    };
    if pr_type == 0xc0000000 as libc::c_uint
        || pr_type >= 0xc0010000 as libc::c_uint && pr_type <= 0xc0017fff as libc::c_uint
    {
        if aprop.is_null() || bprop.is_null() {
            if !aprop.is_null() {
                (*aprop).pr_kind = property_remove;
                updated = 1 as libc::c_int != 0;
            }
        } else {
            number = (*aprop).u.number as libc::c_uint;
            (*aprop).u.number = number as libc::c_ulong | (*bprop).u.number;
            updated = number != (*aprop).u.number as libc::c_uint;
        }
        return updated;
    } else if pr_type == 0xc0000001 as libc::c_uint
        || pr_type >= 0xc0008000 as libc::c_uint && pr_type <= 0xc000ffff as libc::c_uint
    {
        features = 0 as libc::c_int as libc::c_uint;
        if pr_type
            == (0xc0008000 as libc::c_uint)
                .wrapping_add(2 as libc::c_int as libc::c_uint)
        {
            bed = (*(*(*info).output_bfd).xvec).backend_data as *const elf_backend_data;
            htab = if is_elf_hash_table((*info).hash) as libc::c_int != 0
                && elf_hash_table_id((*info).hash as *mut elf_link_hash_table)
                    as libc::c_uint == (*bed).target_id as libc::c_uint
            {
                (*info).hash as *mut elf_x86_link_hash_table
            } else {
                0 as *mut elf_x86_link_hash_table
            };
            match (*(*htab).params).isa_level {
                0 => {}
                2 => {
                    features = (1 as libc::c_uint) << 1 as libc::c_int;
                }
                3 => {
                    features = (1 as libc::c_uint) << 2 as libc::c_int;
                }
                4 => {
                    features = (1 as libc::c_uint) << 3 as libc::c_int;
                }
                _ => {
                    _bfd_abort(
                        b"elfxx-x86.c\0" as *const u8 as *const libc::c_char,
                        2443 as libc::c_int,
                        (*::core::mem::transmute::<
                            &[u8; 110],
                            &[libc::c_char; 110],
                        >(
                            b"_Bool _bfd_x86_elf_merge_gnu_properties(struct bfd_link_info *, bfd *, bfd *, elf_property *, elf_property *)\0",
                        ))
                            .as_ptr(),
                    );
                }
            }
        }
        if !aprop.is_null() && !bprop.is_null() {
            number = (*aprop).u.number as libc::c_uint;
            (*aprop)
                .u
                .number = number as libc::c_ulong | (*bprop).u.number
                | features as libc::c_ulong;
            if (*aprop).u.number == 0 as libc::c_int as libc::c_ulong {
                (*aprop).pr_kind = property_remove;
                updated = 1 as libc::c_int != 0;
            } else {
                updated = number != (*aprop).u.number as libc::c_uint;
            }
        } else if !aprop.is_null() {
            (*aprop).u.number |= features as libc::c_ulong;
            if (*aprop).u.number == 0 as libc::c_int as libc::c_ulong {
                (*aprop).pr_kind = property_remove;
                updated = 1 as libc::c_int != 0;
            }
        } else {
            (*bprop).u.number |= features as libc::c_ulong;
            updated = (*bprop).u.number != 0 as libc::c_int as libc::c_ulong;
        }
        return updated;
    } else if pr_type >= 0xc0000002 as libc::c_uint
        && pr_type <= 0xc0007fff as libc::c_uint
    {
        bed = (*(*(*info).output_bfd).xvec).backend_data as *const elf_backend_data;
        htab = if is_elf_hash_table((*info).hash) as libc::c_int != 0
            && elf_hash_table_id((*info).hash as *mut elf_link_hash_table)
                as libc::c_uint == (*bed).target_id as libc::c_uint
        {
            (*info).hash as *mut elf_x86_link_hash_table
        } else {
            0 as *mut elf_x86_link_hash_table
        };
        if htab.is_null() {
            _bfd_abort(
                b"elfxx-x86.c\0" as *const u8 as *const libc::c_char,
                2494 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 110],
                    &[libc::c_char; 110],
                >(
                    b"_Bool _bfd_x86_elf_merge_gnu_properties(struct bfd_link_info *, bfd *, bfd *, elf_property *, elf_property *)\0",
                ))
                    .as_ptr(),
            );
        }
        if !aprop.is_null() && !bprop.is_null() {
            number = (*aprop).u.number as libc::c_uint;
            (*aprop).u.number = number as libc::c_ulong & (*bprop).u.number;
            if pr_type
                == (0xc0000002 as libc::c_uint)
                    .wrapping_add(0 as libc::c_int as libc::c_uint)
            {
                features = 0 as libc::c_int as libc::c_uint;
                if (*(*htab).params).ibt() != 0 {
                    features = (1 as libc::c_uint) << 0 as libc::c_int;
                }
                if (*(*htab).params).shstk() != 0 {
                    features |= (1 as libc::c_uint) << 1 as libc::c_int;
                }
                if (*(*htab).params).lam_u48() != 0 {
                    features
                        |= (1 as libc::c_uint) << 2 as libc::c_int
                            | (1 as libc::c_uint) << 3 as libc::c_int;
                } else if (*(*htab).params).lam_u57() != 0 {
                    features |= (1 as libc::c_uint) << 3 as libc::c_int;
                }
                (*aprop).u.number |= features as libc::c_ulong;
            }
            updated = number != (*aprop).u.number as libc::c_uint;
            if (*aprop).u.number == 0 as libc::c_int as libc::c_ulong {
                (*aprop).pr_kind = property_remove;
            }
        } else {
            features = 0 as libc::c_int as libc::c_uint;
            if pr_type
                == (0xc0000002 as libc::c_uint)
                    .wrapping_add(0 as libc::c_int as libc::c_uint)
            {
                if (*(*htab).params).ibt() != 0 {
                    features = (1 as libc::c_uint) << 0 as libc::c_int;
                }
                if (*(*htab).params).shstk() != 0 {
                    features |= (1 as libc::c_uint) << 1 as libc::c_int;
                }
                if (*(*htab).params).lam_u48() != 0 {
                    features
                        |= (1 as libc::c_uint) << 2 as libc::c_int
                            | (1 as libc::c_uint) << 3 as libc::c_int;
                } else if (*(*htab).params).lam_u57() != 0 {
                    features |= (1 as libc::c_uint) << 3 as libc::c_int;
                }
            }
            if features != 0 {
                if !aprop.is_null() {
                    updated = features != (*aprop).u.number as libc::c_uint;
                    (*aprop).u.number = features as bfd_vma;
                } else {
                    updated = 1 as libc::c_int != 0;
                    (*bprop).u.number = features as bfd_vma;
                }
            } else if !aprop.is_null() {
                (*aprop).pr_kind = property_remove;
                updated = 1 as libc::c_int != 0;
            }
        }
        return updated;
    } else {
        _bfd_abort(
            b"elfxx-x86.c\0" as *const u8 as *const libc::c_char,
            2564 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 110],
                &[libc::c_char; 110],
            >(
                b"_Bool _bfd_x86_elf_merge_gnu_properties(struct bfd_link_info *, bfd *, bfd *, elf_property *, elf_property *)\0",
            ))
                .as_ptr(),
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn _bfd_x86_elf_link_setup_gnu_properties(
    mut info: *mut bfd_link_info,
    mut init_table: *mut elf_x86_init_table,
) -> *mut bfd {
    let mut current_block: u64;
    let mut normal_target: bool = false;
    let mut lazy_plt: bool = false;
    let mut sec: *mut asection = 0 as *mut asection;
    let mut pltsec: *mut asection = 0 as *mut asection;
    let mut dynobj: *mut bfd = 0 as *mut bfd;
    let mut use_ibt_plt: bool = false;
    let mut plt_alignment: libc::c_uint = 0;
    let mut features: libc::c_uint = 0;
    let mut isa_level: libc::c_uint = 0;
    let mut htab: *mut elf_x86_link_hash_table = 0 as *mut elf_x86_link_hash_table;
    let mut pbfd: *mut bfd = 0 as *mut bfd;
    let mut ebfd: *mut bfd = 0 as *mut bfd;
    let mut prop: *mut elf_property = 0 as *mut elf_property;
    let mut bed: *const elf_backend_data = 0 as *const elf_backend_data;
    let mut class_align: libc::c_uint = (if (*(*((*(*(*info).output_bfd).xvec)
        .backend_data as *const elf_backend_data))
        .s)
        .elfclass as libc::c_int == 2 as libc::c_int
    {
        3 as libc::c_int
    } else {
        2 as libc::c_int
    }) as libc::c_uint;
    let mut got_align: libc::c_uint = 0;
    pbfd = (*info).input_bfds;
    while !pbfd.is_null() {
        if bfd_get_flavour(pbfd) as libc::c_uint
            == bfd_target_elf_flavour as libc::c_int as libc::c_uint
            && bfd_count_sections(pbfd) != 0 as libc::c_int as libc::c_uint
        {
            ebfd = pbfd;
            if !((*(*pbfd).tdata.elf_obj_data).properties).is_null() {
                break;
            }
        }
        pbfd = (*pbfd).link.next;
    }
    bed = (*(*(*info).output_bfd).xvec).backend_data as *const elf_backend_data;
    htab = if is_elf_hash_table((*info).hash) as libc::c_int != 0
        && elf_hash_table_id((*info).hash as *mut elf_link_hash_table) as libc::c_uint
            == (*bed).target_id as libc::c_uint
    {
        (*info).hash as *mut elf_x86_link_hash_table
    } else {
        0 as *mut elf_x86_link_hash_table
    };
    if htab.is_null() {
        return pbfd;
    }
    features = 0 as libc::c_int as libc::c_uint;
    if (*(*htab).params).ibt() != 0 {
        features = (1 as libc::c_uint) << 0 as libc::c_int;
        (*(*htab).params)
            .cet_report = ::core::mem::transmute::<
            libc::c_uint,
            elf_x86_prop_report,
        >(
            (*(*htab).params).cet_report as libc::c_uint
                & !(prop_report_ibt as libc::c_int) as libc::c_uint,
        );
    }
    if (*(*htab).params).shstk() != 0 {
        features |= (1 as libc::c_uint) << 1 as libc::c_int;
        (*(*htab).params)
            .cet_report = ::core::mem::transmute::<
            libc::c_uint,
            elf_x86_prop_report,
        >(
            (*(*htab).params).cet_report as libc::c_uint
                & !(prop_report_shstk as libc::c_int) as libc::c_uint,
        );
    }
    if (*(*htab).params).cet_report as libc::c_uint
        & (prop_report_ibt as libc::c_int | prop_report_shstk as libc::c_int)
            as libc::c_uint == 0
    {
        (*(*htab).params).cet_report = prop_report_none;
    }
    if (*(*htab).params).lam_u48() != 0 {
        features
            |= (1 as libc::c_uint) << 2 as libc::c_int
                | (1 as libc::c_uint) << 3 as libc::c_int;
        (*(*htab).params).lam_u48_report = prop_report_none;
        (*(*htab).params).lam_u57_report = prop_report_none;
    } else if (*(*htab).params).lam_u57() != 0 {
        features |= (1 as libc::c_uint) << 3 as libc::c_int;
        (*(*htab).params).lam_u57_report = prop_report_none;
    }
    match (*(*htab).params).isa_level {
        0 => {
            isa_level = 0 as libc::c_int as libc::c_uint;
        }
        1 => {
            isa_level = (1 as libc::c_uint) << 0 as libc::c_int;
        }
        2 => {
            isa_level = (1 as libc::c_uint) << 1 as libc::c_int;
        }
        3 => {
            isa_level = (1 as libc::c_uint) << 2 as libc::c_int;
        }
        4 => {
            isa_level = (1 as libc::c_uint) << 3 as libc::c_int;
        }
        _ => {
            _bfd_abort(
                b"elfxx-x86.c\0" as *const u8 as *const libc::c_char,
                2654 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 97],
                    &[libc::c_char; 97],
                >(
                    b"bfd *_bfd_x86_elf_link_setup_gnu_properties(struct bfd_link_info *, struct elf_x86_init_table *)\0",
                ))
                    .as_ptr(),
            );
        }
    }
    if !ebfd.is_null() {
        prop = 0 as *mut elf_property;
        if features != 0 {
            prop = _bfd_elf_get_property(
                ebfd,
                (0xc0000002 as libc::c_uint)
                    .wrapping_add(0 as libc::c_int as libc::c_uint),
                4 as libc::c_int as libc::c_uint,
            );
            (*prop).u.number |= features as libc::c_ulong;
            (*prop).pr_kind = property_number;
        }
        if isa_level != 0 {
            prop = _bfd_elf_get_property(
                ebfd,
                (0xc0008000 as libc::c_uint)
                    .wrapping_add(2 as libc::c_int as libc::c_uint),
                4 as libc::c_int as libc::c_uint,
            );
            (*prop).u.number |= isa_level as libc::c_ulong;
            (*prop).pr_kind = property_number;
        }
        if !prop.is_null() && pbfd.is_null() {
            sec = bfd_make_section_with_flags(
                ebfd,
                b".note.gnu.property\0" as *const u8 as *const libc::c_char,
                (0x1 as libc::c_int | 0x2 as libc::c_int | 0x4000 as libc::c_int
                    | 0x8 as libc::c_int | 0x100 as libc::c_int | 0x20 as libc::c_int)
                    as flagword,
            );
            if sec.is_null() {
                ((*(*info).callbacks).einfo)
                    .expect(
                        "non-null function pointer",
                    )(
                    dcgettext(
                        b"bfd\0" as *const u8 as *const libc::c_char,
                        b"%F%P: failed to create GNU property section\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
            }
            if !bfd_set_section_alignment(sec, class_align) {
                current_block = 6041833672043070190;
            } else {
                current_block = 14945149239039849694;
            }
        } else {
            current_block = 5892776923941496671;
        }
    } else {
        current_block = 5892776923941496671;
    }
    loop {
        match current_block {
            14945149239039849694 => {
                (*((*sec).used_by_bfd as *mut bfd_elf_section_data))
                    .this_hdr
                    .sh_type = 7 as libc::c_int as libc::c_uint;
                current_block = 5892776923941496671;
            }
            6041833672043070190 => {
                ((*(*info).callbacks).einfo)
                    .expect(
                        "non-null function pointer",
                    )(
                    dcgettext(
                        b"bfd\0" as *const u8 as *const libc::c_char,
                        b"%F%pA: failed to align section\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    sec,
                );
                current_block = 14945149239039849694;
            }
            _ => {
                if (*(*htab).params).cet_report as libc::c_uint != 0
                    || (*(*htab).params).lam_u48_report as libc::c_uint != 0
                    || (*(*htab).params).lam_u57_report as libc::c_uint != 0
                {
                    let mut abfd: *mut bfd = 0 as *mut bfd;
                    let mut warning_msg: *const libc::c_char = dcgettext(
                        b"bfd\0" as *const u8 as *const libc::c_char,
                        b"%P: %pB: warning: missing %s\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    );
                    let mut error_msg: *const libc::c_char = dcgettext(
                        b"bfd\0" as *const u8 as *const libc::c_char,
                        b"%X%P: %pB: error: missing %s\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    );
                    let mut cet_msg: *const libc::c_char = 0 as *const libc::c_char;
                    let mut lam_u48_msg: *const libc::c_char = 0 as *const libc::c_char;
                    let mut lam_u57_msg: *const libc::c_char = 0 as *const libc::c_char;
                    let mut missing: *const libc::c_char = 0 as *const libc::c_char;
                    let mut p: *mut elf_property_list = 0 as *mut elf_property_list;
                    let mut missing_ibt: bool = false;
                    let mut missing_shstk: bool = false;
                    let mut missing_lam_u48: bool = false;
                    let mut missing_lam_u57: bool = false;
                    let mut check_ibt: bool = (*(*htab).params).cet_report
                        as libc::c_uint != 0
                        && (*(*htab).params).cet_report as libc::c_uint
                            & prop_report_ibt as libc::c_int as libc::c_uint != 0;
                    let mut check_shstk: bool = (*(*htab).params).cet_report
                        as libc::c_uint != 0
                        && (*(*htab).params).cet_report as libc::c_uint
                            & prop_report_shstk as libc::c_int as libc::c_uint != 0;
                    if (*(*htab).params).cet_report as u64 != 0 {
                        if (*(*htab).params).cet_report as libc::c_uint
                            & prop_report_warning as libc::c_int as libc::c_uint != 0
                        {
                            cet_msg = warning_msg;
                        } else {
                            cet_msg = error_msg;
                        }
                    }
                    if (*(*htab).params).lam_u48_report as u64 != 0 {
                        if (*(*htab).params).lam_u48_report as libc::c_uint
                            & prop_report_warning as libc::c_int as libc::c_uint != 0
                        {
                            lam_u48_msg = warning_msg;
                        } else {
                            lam_u48_msg = error_msg;
                        }
                    }
                    if (*(*htab).params).lam_u57_report as u64 != 0 {
                        if (*(*htab).params).lam_u57_report as libc::c_uint
                            & prop_report_warning as libc::c_int as libc::c_uint != 0
                        {
                            lam_u57_msg = warning_msg;
                        } else {
                            lam_u57_msg = error_msg;
                        }
                    }
                    abfd = (*info).input_bfds;
                    while !abfd.is_null() {
                        if (*abfd).flags
                            & (0x40 as libc::c_int | 0x10000 as libc::c_int
                                | 0x1000 as libc::c_int) as libc::c_uint == 0
                            && bfd_get_flavour(abfd) as libc::c_uint
                                == bfd_target_elf_flavour as libc::c_int as libc::c_uint
                        {
                            p = (*(*abfd).tdata.elf_obj_data).properties;
                            while !p.is_null() {
                                if (*p).property.pr_type
                                    == (0xc0000002 as libc::c_uint)
                                        .wrapping_add(0 as libc::c_int as libc::c_uint)
                                {
                                    break;
                                }
                                p = (*p).next;
                            }
                            missing_ibt = check_ibt;
                            missing_shstk = check_shstk;
                            missing_lam_u48 = !lam_u48_msg.is_null();
                            missing_lam_u57 = !lam_u57_msg.is_null();
                            if !p.is_null() {
                                missing_ibt = (missing_ibt as libc::c_int
                                    & ((*p).property.u.number
                                        & ((1 as libc::c_uint) << 0 as libc::c_int) as libc::c_ulong
                                        == 0) as libc::c_int) != 0;
                                missing_shstk = (missing_shstk as libc::c_int
                                    & ((*p).property.u.number
                                        & ((1 as libc::c_uint) << 1 as libc::c_int) as libc::c_ulong
                                        == 0) as libc::c_int) != 0;
                                missing_lam_u48 = (missing_lam_u48 as libc::c_int
                                    & ((*p).property.u.number
                                        & ((1 as libc::c_uint) << 2 as libc::c_int) as libc::c_ulong
                                        == 0) as libc::c_int) != 0;
                                missing_lam_u57 = (missing_lam_u57 as libc::c_int
                                    & ((*p).property.u.number
                                        & ((1 as libc::c_uint) << 3 as libc::c_int) as libc::c_ulong
                                        == 0) as libc::c_int) != 0;
                            }
                            if missing_ibt as libc::c_int != 0
                                || missing_shstk as libc::c_int != 0
                            {
                                if missing_ibt as libc::c_int != 0
                                    && missing_shstk as libc::c_int != 0
                                {
                                    missing = dcgettext(
                                        b"bfd\0" as *const u8 as *const libc::c_char,
                                        b"IBT and SHSTK properties\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    );
                                } else if missing_ibt {
                                    missing = dcgettext(
                                        b"bfd\0" as *const u8 as *const libc::c_char,
                                        b"IBT property\0" as *const u8 as *const libc::c_char,
                                        5 as libc::c_int,
                                    );
                                } else {
                                    missing = dcgettext(
                                        b"bfd\0" as *const u8 as *const libc::c_char,
                                        b"SHSTK property\0" as *const u8 as *const libc::c_char,
                                        5 as libc::c_int,
                                    );
                                }
                                ((*(*info).callbacks).einfo)
                                    .expect(
                                        "non-null function pointer",
                                    )(cet_msg, abfd, missing);
                            }
                            if missing_lam_u48 {
                                missing = dcgettext(
                                    b"bfd\0" as *const u8 as *const libc::c_char,
                                    b"LAM_U48 property\0" as *const u8 as *const libc::c_char,
                                    5 as libc::c_int,
                                );
                                ((*(*info).callbacks).einfo)
                                    .expect(
                                        "non-null function pointer",
                                    )(lam_u48_msg, abfd, missing);
                            }
                            if missing_lam_u57 {
                                missing = dcgettext(
                                    b"bfd\0" as *const u8 as *const libc::c_char,
                                    b"LAM_U57 property\0" as *const u8 as *const libc::c_char,
                                    5 as libc::c_int,
                                );
                                ((*(*info).callbacks).einfo)
                                    .expect(
                                        "non-null function pointer",
                                    )(lam_u57_msg, abfd, missing);
                            }
                        }
                        abfd = (*abfd).link.next;
                    }
                }
                pbfd = _bfd_elf_link_setup_gnu_properties(info);
                (*htab).r_info = (*init_table).r_info;
                (*htab).r_sym = (*init_table).r_sym;
                if (*info).type_0() as libc::c_int == type_relocatable as libc::c_int {
                    return pbfd;
                }
                (*htab).plt0_pad_byte = (*init_table).plt0_pad_byte;
                use_ibt_plt = (*(*htab).params).ibtplt() as libc::c_int != 0
                    || (*(*htab).params).ibt() as libc::c_int != 0;
                if !use_ibt_plt && !pbfd.is_null() {
                    let mut p_0: *mut elf_property_list = 0 as *mut elf_property_list;
                    p_0 = (*(*pbfd).tdata.elf_obj_data).properties;
                    while !p_0.is_null() {
                        if (0xc0000002 as libc::c_uint)
                            .wrapping_add(0 as libc::c_int as libc::c_uint)
                            == (*p_0).property.pr_type
                        {
                            use_ibt_plt = (*p_0).property.u.number
                                & ((1 as libc::c_uint) << 0 as libc::c_int) as libc::c_ulong
                                != 0;
                            break;
                        } else {
                            if (0xc0000002 as libc::c_uint)
                                .wrapping_add(0 as libc::c_int as libc::c_uint)
                                < (*p_0).property.pr_type
                            {
                                break;
                            }
                            p_0 = (*p_0).next;
                        }
                    }
                }
                dynobj = (*htab).elf.dynobj;
                if dynobj.is_null() {
                    if !pbfd.is_null() {
                        (*htab).elf.dynobj = pbfd;
                        dynobj = pbfd;
                    } else {
                        let mut abfd_0: *mut bfd = 0 as *mut bfd;
                        abfd_0 = (*info).input_bfds;
                        while !abfd_0.is_null() {
                            if bfd_get_flavour(abfd_0) as libc::c_uint
                                == bfd_target_elf_flavour as libc::c_int as libc::c_uint
                                && (*abfd_0).flags
                                    & (0x40 as libc::c_int | 0x1000 as libc::c_int
                                        | 0x10000 as libc::c_int) as libc::c_uint
                                    == 0 as libc::c_int as libc::c_uint
                                && ((*bed).relocs_compatible)
                                    .expect(
                                        "non-null function pointer",
                                    )((*abfd_0).xvec, (*(*info).output_bfd).xvec) as libc::c_int
                                    != 0
                            {
                                (*htab).elf.dynobj = abfd_0;
                                dynobj = abfd_0;
                                break;
                            } else {
                                abfd_0 = (*abfd_0).link.next;
                            }
                        }
                    }
                }
                if dynobj.is_null() {
                    return pbfd;
                }
                (*htab).plt.has_plt0 = 1 as libc::c_int as libc::c_uint;
                normal_target = (*htab).elf.target_os as libc::c_uint
                    == is_normal as libc::c_int as libc::c_uint;
                if normal_target {
                    if use_ibt_plt {
                        (*htab).lazy_plt = (*init_table).lazy_ibt_plt;
                        (*htab).non_lazy_plt = (*init_table).non_lazy_ibt_plt;
                    } else {
                        (*htab).lazy_plt = (*init_table).lazy_plt;
                        (*htab).non_lazy_plt = (*init_table).non_lazy_plt;
                    }
                } else {
                    (*htab).lazy_plt = (*init_table).lazy_plt;
                    (*htab).non_lazy_plt = 0 as *const elf_x86_non_lazy_plt_layout;
                }
                pltsec = (*htab).elf.splt;
                if !((*htab).non_lazy_plt).is_null()
                    && ((*htab).plt.has_plt0 == 0 || pltsec.is_null())
                {
                    lazy_plt = 0 as libc::c_int != 0;
                    if (*info).type_0() as libc::c_int == type_dll as libc::c_int
                        || (*info).type_0() as libc::c_int == type_pie as libc::c_int
                    {
                        (*htab).plt.plt_entry = (*(*htab).non_lazy_plt).pic_plt_entry;
                    } else {
                        (*htab).plt.plt_entry = (*(*htab).non_lazy_plt).plt_entry;
                    }
                    (*htab).plt.plt_entry_size = (*(*htab).non_lazy_plt).plt_entry_size;
                    (*htab).plt.plt_got_offset = (*(*htab).non_lazy_plt).plt_got_offset;
                    (*htab)
                        .plt
                        .plt_got_insn_size = (*(*htab).non_lazy_plt).plt_got_insn_size;
                    (*htab)
                        .plt
                        .eh_frame_plt_size = (*(*htab).non_lazy_plt).eh_frame_plt_size;
                    (*htab).plt.eh_frame_plt = (*(*htab).non_lazy_plt).eh_frame_plt;
                } else {
                    lazy_plt = 1 as libc::c_int != 0;
                    if (*info).type_0() as libc::c_int == type_dll as libc::c_int
                        || (*info).type_0() as libc::c_int == type_pie as libc::c_int
                    {
                        (*htab).plt.plt0_entry = (*(*htab).lazy_plt).pic_plt0_entry;
                        (*htab).plt.plt_entry = (*(*htab).lazy_plt).pic_plt_entry;
                    } else {
                        (*htab).plt.plt0_entry = (*(*htab).lazy_plt).plt0_entry;
                        (*htab).plt.plt_entry = (*(*htab).lazy_plt).plt_entry;
                    }
                    (*htab).plt.plt_entry_size = (*(*htab).lazy_plt).plt_entry_size;
                    (*htab).plt.plt_got_offset = (*(*htab).lazy_plt).plt_got_offset;
                    (*htab)
                        .plt
                        .plt_got_insn_size = (*(*htab).lazy_plt).plt_got_insn_size;
                    (*htab)
                        .plt
                        .eh_frame_plt_size = (*(*htab).lazy_plt).eh_frame_plt_size;
                    (*htab).plt.eh_frame_plt = (*(*htab).lazy_plt).eh_frame_plt;
                }
                if (*htab).elf.target_os as libc::c_uint
                    == is_vxworks as libc::c_int as libc::c_uint
                    && !elf_vxworks_create_dynamic_sections(
                        dynobj,
                        info,
                        &mut (*htab).srelplt2,
                    )
                {
                    ((*(*info).callbacks).einfo)
                        .expect(
                            "non-null function pointer",
                        )(
                        dcgettext(
                            b"bfd\0" as *const u8 as *const libc::c_char,
                            b"%F%P: failed to create VxWorks dynamic sections\n\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    return pbfd;
                }
                if ((*htab).elf.sgot).is_null()
                    && !_bfd_elf_create_got_section(dynobj, info)
                {
                    ((*(*info).callbacks).einfo)
                        .expect(
                            "non-null function pointer",
                        )(
                        dcgettext(
                            b"bfd\0" as *const u8 as *const libc::c_char,
                            b"%F%P: failed to create GOT sections\n\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                }
                got_align = (if (*bed).target_id as libc::c_uint
                    == X86_64_ELF_DATA as libc::c_int as libc::c_uint
                {
                    3 as libc::c_int
                } else {
                    2 as libc::c_int
                }) as libc::c_uint;
                sec = (*htab).elf.sgot;
                if !bfd_set_section_alignment(sec, got_align) {
                    current_block = 6041833672043070190;
                    continue;
                }
                sec = (*htab).elf.sgotplt;
                if !bfd_set_section_alignment(sec, got_align) {
                    current_block = 6041833672043070190;
                    continue;
                }
                if !_bfd_elf_create_ifunc_sections(dynobj, info) {
                    ((*(*info).callbacks).einfo)
                        .expect(
                            "non-null function pointer",
                        )(
                        dcgettext(
                            b"bfd\0" as *const u8 as *const libc::c_char,
                            b"%F%P: failed to create ifunc sections\n\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                }
                plt_alignment = bfd_log2((*htab).plt.plt_entry_size as bfd_vma);
                if !pltsec.is_null() {
                    if ((*info).type_0() as libc::c_int == type_pde as libc::c_int
                        || (*info).type_0() as libc::c_int == type_pie as libc::c_int)
                        && (*info).nointerp() == 0
                    {
                        let mut s: *mut asection = bfd_get_linker_section(
                            dynobj,
                            b".interp\0" as *const u8 as *const libc::c_char,
                        );
                        if s.is_null() {
                            _bfd_abort(
                                b"elfxx-x86.c\0" as *const u8 as *const libc::c_char,
                                2975 as libc::c_int,
                                (*::core::mem::transmute::<
                                    &[u8; 97],
                                    &[libc::c_char; 97],
                                >(
                                    b"bfd *_bfd_x86_elf_link_setup_gnu_properties(struct bfd_link_info *, struct elf_x86_init_table *)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                        (*s).size = (*htab).dynamic_interpreter_size as bfd_size_type;
                        (*s)
                            .contents = (*htab).dynamic_interpreter
                            as *mut libc::c_uchar;
                        (*htab).interp = s;
                    }
                    if normal_target {
                        let mut pltflags: flagword = (*bed).dynamic_sec_flags
                            | 0x1 as libc::c_int as libc::c_uint
                            | 0x10 as libc::c_int as libc::c_uint
                            | 0x2 as libc::c_int as libc::c_uint
                            | 0x8 as libc::c_int as libc::c_uint;
                        let mut non_lazy_plt_alignment: libc::c_uint = bfd_log2(
                            (*(*htab).non_lazy_plt).plt_entry_size as bfd_vma,
                        );
                        sec = pltsec;
                        if !bfd_set_section_alignment(sec, plt_alignment) {
                            current_block = 6041833672043070190;
                            continue;
                        }
                        sec = bfd_make_section_anyway_with_flags(
                            dynobj,
                            b".plt.got\0" as *const u8 as *const libc::c_char,
                            pltflags,
                        );
                        if sec.is_null() {
                            ((*(*info).callbacks).einfo)
                                .expect(
                                    "non-null function pointer",
                                )(
                                dcgettext(
                                    b"bfd\0" as *const u8 as *const libc::c_char,
                                    b"%F%P: failed to create GOT PLT section\n\0" as *const u8
                                        as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                            );
                        }
                        if !bfd_set_section_alignment(sec, non_lazy_plt_alignment) {
                            current_block = 6041833672043070190;
                            continue;
                        }
                        (*htab).plt_got = sec;
                        if lazy_plt {
                            sec = 0 as *mut asection;
                            if use_ibt_plt {
                                sec = bfd_make_section_anyway_with_flags(
                                    dynobj,
                                    b".plt.sec\0" as *const u8 as *const libc::c_char,
                                    pltflags,
                                );
                                if sec.is_null() {
                                    ((*(*info).callbacks).einfo)
                                        .expect(
                                            "non-null function pointer",
                                        )(
                                        dcgettext(
                                            b"bfd\0" as *const u8 as *const libc::c_char,
                                            b"%F%P: failed to create IBT-enabled PLT section\n\0"
                                                as *const u8 as *const libc::c_char,
                                            5 as libc::c_int,
                                        ),
                                    );
                                }
                                if !bfd_set_section_alignment(sec, plt_alignment) {
                                    current_block = 6041833672043070190;
                                    continue;
                                }
                            } else if (*(*htab).params).bndplt() as libc::c_int != 0
                                && (*(*((*(*dynobj).xvec).backend_data
                                    as *const elf_backend_data))
                                    .s)
                                    .elfclass as libc::c_int == 2 as libc::c_int
                            {
                                sec = bfd_make_section_anyway_with_flags(
                                    dynobj,
                                    b".plt.sec\0" as *const u8 as *const libc::c_char,
                                    pltflags,
                                );
                                if sec.is_null() {
                                    ((*(*info).callbacks).einfo)
                                        .expect(
                                            "non-null function pointer",
                                        )(
                                        dcgettext(
                                            b"bfd\0" as *const u8 as *const libc::c_char,
                                            b"%F%P: failed to create BND PLT section\n\0" as *const u8
                                                as *const libc::c_char,
                                            5 as libc::c_int,
                                        ),
                                    );
                                }
                                if !bfd_set_section_alignment(sec, non_lazy_plt_alignment) {
                                    current_block = 6041833672043070190;
                                    continue;
                                }
                            }
                            (*htab).plt_second = sec;
                        }
                    }
                    if (*info).no_ld_generated_unwind_info() == 0 {
                        let mut flags: flagword = (0x1 as libc::c_int
                            | 0x2 as libc::c_int | 0x8 as libc::c_int
                            | 0x100 as libc::c_int | 0x4000 as libc::c_int
                            | 0x100000 as libc::c_int) as flagword;
                        sec = bfd_make_section_anyway_with_flags(
                            dynobj,
                            b".eh_frame\0" as *const u8 as *const libc::c_char,
                            flags,
                        );
                        if sec.is_null() {
                            ((*(*info).callbacks).einfo)
                                .expect(
                                    "non-null function pointer",
                                )(
                                dcgettext(
                                    b"bfd\0" as *const u8 as *const libc::c_char,
                                    b"%F%P: failed to create PLT .eh_frame section\n\0"
                                        as *const u8 as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                            );
                        }
                        if !bfd_set_section_alignment(sec, class_align) {
                            current_block = 6041833672043070190;
                            continue;
                        }
                        (*htab).plt_eh_frame = sec;
                        if !((*htab).plt_got).is_null() {
                            sec = bfd_make_section_anyway_with_flags(
                                dynobj,
                                b".eh_frame\0" as *const u8 as *const libc::c_char,
                                flags,
                            );
                            if sec.is_null() {
                                ((*(*info).callbacks).einfo)
                                    .expect(
                                        "non-null function pointer",
                                    )(
                                    dcgettext(
                                        b"bfd\0" as *const u8 as *const libc::c_char,
                                        b"%F%P: failed to create GOT PLT .eh_frame section\n\0"
                                            as *const u8 as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                            }
                            if !bfd_set_section_alignment(sec, class_align) {
                                current_block = 6041833672043070190;
                                continue;
                            }
                            (*htab).plt_got_eh_frame = sec;
                        }
                        if !((*htab).plt_second).is_null() {
                            sec = bfd_make_section_anyway_with_flags(
                                dynobj,
                                b".eh_frame\0" as *const u8 as *const libc::c_char,
                                flags,
                            );
                            if sec.is_null() {
                                ((*(*info).callbacks).einfo)
                                    .expect(
                                        "non-null function pointer",
                                    )(
                                    dcgettext(
                                        b"bfd\0" as *const u8 as *const libc::c_char,
                                        b"%F%P: failed to create the second PLT .eh_frame section\n\0"
                                            as *const u8 as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                            }
                            if !bfd_set_section_alignment(sec, class_align) {
                                current_block = 6041833672043070190;
                                continue;
                            }
                            (*htab).plt_second_eh_frame = sec;
                        }
                    }
                }
                sec = (*htab).elf.iplt;
                if sec.is_null() {
                    break;
                }
                if !bfd_set_section_alignment(sec, 0 as libc::c_int as libc::c_uint) {
                    current_block = 6041833672043070190;
                    continue;
                }
                (*htab)
                    .plt
                    .iplt_alignment = if normal_target as libc::c_int != 0 {
                    plt_alignment
                } else {
                    (*bed).plt_alignment()
                };
                break;
            }
        }
    }
    if ((*info).type_0() as libc::c_int == type_pde as libc::c_int
        || (*info).type_0() as libc::c_int == type_pie as libc::c_int)
        && (*info).nointerp() == 0 && (*(*htab).params).has_dynamic_linker() == 0
        && (*(*htab).params).static_before_all_inputs() as libc::c_int != 0
    {
        let mut abfd_1: *mut bfd = 0 as *mut bfd;
        abfd_1 = (*info).input_bfds;
        while !abfd_1.is_null() {
            if (*abfd_1).flags & 0x40 as libc::c_int as libc::c_uint != 0 {
                ((*(*info).callbacks).einfo)
                    .expect(
                        "non-null function pointer",
                    )(
                    dcgettext(
                        b"bfd\0" as *const u8 as *const libc::c_char,
                        b"%X%P: attempted static link of dynamic object `%pB'\n\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    abfd_1,
                );
            }
            abfd_1 = (*abfd_1).link.next;
        }
    }
    return pbfd;
}
#[no_mangle]
pub unsafe extern "C" fn _bfd_x86_elf_link_fixup_gnu_properties(
    mut info: *mut bfd_link_info,
    mut listp: *mut *mut elf_property_list,
) {
    let mut p: *mut elf_property_list = 0 as *mut elf_property_list;
    p = *listp;
    while !p.is_null() {
        let mut type_0: libc::c_uint = (*p).property.pr_type;
        if type_0 == 0xc0000000 as libc::c_uint || type_0 == 0xc0000001 as libc::c_uint
            || type_0 >= 0xc0000002 as libc::c_uint
                && type_0 <= 0xc0007fff as libc::c_uint
            || type_0 >= 0xc0008000 as libc::c_uint
                && type_0 <= 0xc000ffff as libc::c_uint
            || type_0 >= 0xc0010000 as libc::c_uint
                && type_0 <= 0xc0017fff as libc::c_uint
        {
            if (*p).property.u.number == 0 as libc::c_int as libc::c_ulong
                && (type_0 == 0xc0000001 as libc::c_uint
                    || type_0 >= 0xc0000002 as libc::c_uint
                        && type_0 <= 0xc0007fff as libc::c_uint
                    || type_0 >= 0xc0008000 as libc::c_uint
                        && type_0 <= 0xc000ffff as libc::c_uint)
            {
                *listp = (*p).next;
            } else {
                if type_0
                    == (0xc0000002 as libc::c_uint)
                        .wrapping_add(0 as libc::c_int as libc::c_uint)
                    && !((*(*((*(*(*info).output_bfd).xvec).backend_data
                        as *const elf_backend_data))
                        .s)
                        .elfclass as libc::c_int == 2 as libc::c_int)
                {
                    (*p).property.u.number
                        &= !((1 as libc::c_uint) << 2 as libc::c_int
                            | (1 as libc::c_uint) << 3 as libc::c_int) as libc::c_ulong;
                }
                listp = &mut (*p).next;
            }
        } else if type_0 > 0xdfffffff as libc::c_uint {
            break;
        }
        p = (*p).next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn _bfd_elf_linker_x86_set_options(
    mut info: *mut bfd_link_info,
    mut params: *mut elf_linker_x86_params,
) {
    let mut bed: *const elf_backend_data = (*(*(*info).output_bfd).xvec).backend_data
        as *const elf_backend_data;
    let mut htab: *mut elf_x86_link_hash_table = if is_elf_hash_table((*info).hash)
        as libc::c_int != 0
        && elf_hash_table_id((*info).hash as *mut elf_link_hash_table) as libc::c_uint
            == (*bed).target_id as libc::c_uint
    {
        (*info).hash as *mut elf_x86_link_hash_table
    } else {
        0 as *mut elf_x86_link_hash_table
    };
    if !htab.is_null() {
        (*htab).params = params;
    }
}
