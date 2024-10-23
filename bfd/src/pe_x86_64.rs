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
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
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
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn bfd_bread(_: *mut libc::c_void, _: bfd_size_type, _: *mut bfd) -> bfd_size_type;
    fn bfd_bwrite(
        _: *const libc::c_void,
        _: bfd_size_type,
        _: *mut bfd,
    ) -> bfd_size_type;
    fn bfd_seek(_: *mut bfd, _: file_ptr, _: libc::c_int) -> libc::c_int;
    fn bfd_tell(_: *mut bfd) -> file_ptr;
    fn bfd_getl64(_: *const libc::c_void) -> bfd_uint64_t;
    fn bfd_getl_signed_64(_: *const libc::c_void) -> bfd_int64_t;
    fn bfd_getl32(_: *const libc::c_void) -> bfd_vma;
    fn bfd_getl_signed_32(_: *const libc::c_void) -> bfd_signed_vma;
    fn bfd_getl16(_: *const libc::c_void) -> bfd_vma;
    fn bfd_getl_signed_16(_: *const libc::c_void) -> bfd_signed_vma;
    fn bfd_putl64(_: bfd_uint64_t, _: *mut libc::c_void);
    fn bfd_putl32(_: bfd_vma, _: *mut libc::c_void);
    fn bfd_putl16(_: bfd_vma, _: *mut libc::c_void);
    fn bfd_alloc(abfd: *mut bfd, wanted: bfd_size_type) -> *mut libc::c_void;
    fn bfd_zalloc(abfd: *mut bfd, wanted: bfd_size_type) -> *mut libc::c_void;
    fn bfd_get_file_size(abfd: *mut bfd) -> ufile_ptr;
    static mut _bfd_std_section: [asection; 4];
    fn bfd_generic_is_group_section(_: *mut bfd, sec: *const asection) -> bool;
    fn bfd_generic_discard_group(abfd: *mut bfd, group: *mut asection) -> bool;
    fn bfd_default_set_arch_mach(
        abfd: *mut bfd,
        arch: bfd_architecture,
        mach: libc::c_ulong,
    ) -> bool;
    fn bfd_get_arch(abfd: *const bfd) -> bfd_architecture;
    fn bfd_get_reloc_size(_: *const reloc_howto_type) -> libc::c_uint;
    fn bfd_reloc_offset_in_range(
        howto: *const reloc_howto_type,
        abfd: *mut bfd,
        section: *mut asection,
        offset: bfd_size_type,
    ) -> bool;
    fn bfd_set_error(error_tag: bfd_error_type);
    fn _bfd_error_handler(fmt: *const libc::c_char, _: ...);
    fn bfd_generic_define_common_symbol(
        output_bfd: *mut bfd,
        info: *mut bfd_link_info,
        h: *mut bfd_link_hash_entry,
    ) -> bool;
    fn _bfd_generic_link_hide_symbol(
        output_bfd: *mut bfd,
        info: *mut bfd_link_info,
        h: *mut bfd_link_hash_entry,
    );
    fn bfd_generic_define_start_stop(
        info: *mut bfd_link_info,
        symbol: *const libc::c_char,
        sec: *mut asection,
    ) -> *mut bfd_link_hash_entry;
    fn _bfd_generic_link_check_relocs(abfd: *mut bfd, info: *mut bfd_link_info) -> bool;
    fn bfd_link_hash_lookup(
        _: *mut bfd_link_hash_table,
        _: *const libc::c_char,
        create: bool,
        copy: bool,
        follow: bool,
    ) -> *mut bfd_link_hash_entry;
    fn coff_mangle_symbols(_: *mut bfd);
    fn coff_write_symbols(_: *mut bfd) -> bool;
    fn coff_count_linenumbers(_: *mut bfd) -> libc::c_int;
    fn coff_canonicalize_symtab(_: *mut bfd, _: *mut *mut asymbol) -> libc::c_long;
    fn coff_get_symtab_upper_bound(_: *mut bfd) -> libc::c_long;
    fn coff_section_from_bfd_index(_: *mut bfd, _: libc::c_int) -> *mut bfd_section;
    fn coff_object_p(_: *mut bfd) -> bfd_cleanup;
    fn coff_write_linenumbers(_: *mut bfd) -> bool;
    fn coff_get_lineno(_: *mut bfd, _: *mut asymbol) -> *mut alent;
    fn coff_renumber_symbols(_: *mut bfd, _: *mut libc::c_int) -> bool;
    fn _bfd_coff_close_and_cleanup(_: *mut bfd) -> bool;
    fn coff_sizeof_headers(_: *mut bfd, _: *mut bfd_link_info) -> libc::c_int;
    fn coff_find_inliner_info(
        _: *mut bfd,
        _: *mut *const libc::c_char,
        _: *mut *const libc::c_char,
        _: *mut libc::c_uint,
    ) -> bool;
    fn coff_find_nearest_line(
        _: *mut bfd,
        _: *mut *mut asymbol,
        _: *mut asection,
        _: bfd_vma,
        _: *mut *const libc::c_char,
        _: *mut *const libc::c_char,
        _: *mut libc::c_uint,
        _: *mut libc::c_uint,
    ) -> bool;
    fn coff_bfd_make_debug_symbol(
        _: *mut bfd,
        _: *mut libc::c_void,
        _: libc::c_ulong,
    ) -> *mut asymbol;
    fn coff_get_reloc_upper_bound(_: *mut bfd, _: sec_ptr) -> libc::c_long;
    fn coff_print_symbol(
        _: *mut bfd,
        filep: *mut libc::c_void,
        _: *mut asymbol,
        _: bfd_print_symbol_type,
    );
    fn coff_make_empty_symbol(_: *mut bfd) -> *mut asymbol;
    fn coff_get_normalized_symtab(_: *mut bfd) -> *mut coff_ptr_struct;
    fn _bfd_coff_generic_relocate_section(
        _: *mut bfd,
        _: *mut bfd_link_info,
        _: *mut bfd,
        _: *mut asection,
        _: *mut bfd_byte,
        _: *mut internal_reloc,
        _: *mut internal_syment,
        _: *mut *mut asection,
    ) -> bool;
    fn _bfd_coff_get_external_symbols(_: *mut bfd) -> bool;
    fn _bfd_coff_link_hash_table_create(_: *mut bfd) -> *mut bfd_link_hash_table;
    fn _bfd_coff_final_link(_: *mut bfd, _: *mut bfd_link_info) -> bool;
    fn _bfd_coff_internal_syment_name(
        _: *mut bfd,
        _: *const internal_syment,
        _: *mut libc::c_char,
    ) -> *const libc::c_char;
    fn _bfd_coff_section_already_linked(
        _: *mut bfd,
        _: *mut asection,
        _: *mut bfd_link_info,
    ) -> bool;
    fn _bfd_coff_link_add_symbols(_: *mut bfd, _: *mut bfd_link_info) -> bool;
    fn _bfd_coff_is_local_label_name(_: *mut bfd, _: *const libc::c_char) -> bool;
    fn bfd_coff_gc_sections(_: *mut bfd, _: *mut bfd_link_info) -> bool;
    fn bfd_coff_group_name(_: *mut bfd, _: *const asection) -> *const libc::c_char;
    fn pex64_bfd_print_pdata(_: *mut bfd, _: *mut libc::c_void) -> bool;
    fn bfd_malloc(_: bfd_size_type) -> *mut libc::c_void;
    fn bfd_release(_: *mut bfd, _: *mut libc::c_void);
    fn _bfd_generic_mkarchive(_: *mut bfd) -> bool;
    fn bfd_generic_archive_p(_: *mut bfd) -> bfd_cleanup;
    fn bfd_slurp_armap(_: *mut bfd) -> bool;
    fn _bfd_slurp_extended_name_table(_: *mut bfd) -> bool;
    fn _bfd_write_archive_contents(_: *mut bfd) -> bool;
    fn _bfd_generic_get_elt_at_index(_: *mut bfd, _: symindex) -> *mut bfd;
    fn _bfd_bool_bfd_asymbol_false(_: *mut bfd, _: *mut asymbol) -> bool;
    fn _bfd_bool_bfd_false_error(_: *mut bfd) -> bool;
    fn _bfd_bool_bfd_true(_: *mut bfd) -> bool;
    fn _bfd_bool_bfd_link_true(_: *mut bfd, _: *mut bfd_link_info) -> bool;
    fn _bfd_bool_bfd_bfd_true(_: *mut bfd, _: *mut bfd) -> bool;
    fn _bfd_bool_bfd_uint_true(_: *mut bfd, _: libc::c_uint) -> bool;
    fn _bfd_bool_bfd_asymbol_bfd_asymbol_true(
        _: *mut bfd,
        _: *mut asymbol,
        _: *mut bfd,
        _: *mut asymbol,
    ) -> bool;
    fn _bfd_long_bfd_n1_error(_: *mut bfd) -> libc::c_long;
    fn _bfd_dummy_target(_: *mut bfd) -> bfd_cleanup;
    fn bfd_dont_truncate_arname(
        _: *mut bfd,
        _: *const libc::c_char,
        _: *mut libc::c_char,
    );
    fn _bfd_coff_write_armap(
        _: *mut bfd,
        _: libc::c_uint,
        _: *mut orl,
        _: libc::c_uint,
        _: libc::c_int,
    ) -> bool;
    fn _bfd_generic_read_ar_hdr(_: *mut bfd) -> *mut libc::c_void;
    fn _bfd_generic_write_ar_hdr(_: *mut bfd, _: *mut bfd) -> bool;
    fn bfd_generic_openr_next_archived_file(_: *mut bfd, _: *mut bfd) -> *mut bfd;
    fn bfd_generic_stat_arch_elt(_: *mut bfd, _: *mut stat) -> libc::c_int;
    fn _bfd_generic_new_section_hook(_: *mut bfd, _: *mut asection) -> bool;
    fn _bfd_generic_get_section_contents(
        _: *mut bfd,
        _: *mut asection,
        _: *mut libc::c_void,
        _: file_ptr,
        _: bfd_size_type,
    ) -> bool;
    fn _bfd_generic_get_section_contents_in_window(
        _: *mut bfd,
        _: *mut asection,
        _: *mut bfd_window,
        _: file_ptr,
        _: bfd_size_type,
    ) -> bool;
    fn _bfd_generic_init_private_section_data(
        _: *mut bfd,
        _: *mut asection,
        _: *mut bfd,
        _: *mut asection,
        _: *mut bfd_link_info,
    ) -> bool;
    fn _bfd_nocore_core_file_failing_command(_: *mut bfd) -> *mut libc::c_char;
    fn _bfd_nocore_core_file_failing_signal(_: *mut bfd) -> libc::c_int;
    fn _bfd_nocore_core_file_matches_executable_p(_: *mut bfd, _: *mut bfd) -> bool;
    fn _bfd_nocore_core_file_pid(_: *mut bfd) -> libc::c_int;
    fn _bfd_archive_coff_construct_extended_name_table(
        _: *mut bfd,
        _: *mut *mut libc::c_char,
        _: *mut bfd_size_type,
        _: *mut *const libc::c_char,
    ) -> bool;
    fn _bfd_nosymbols_canonicalize_symtab(
        _: *mut bfd,
        _: *mut *mut asymbol,
    ) -> libc::c_long;
    fn _bfd_nosymbols_get_symbol_version_string(
        _: *mut bfd,
        _: *mut asymbol,
        _: bool,
        _: *mut bool,
    ) -> *const libc::c_char;
    fn _bfd_nosymbols_find_line(
        _: *mut bfd,
        _: *mut *mut asymbol,
        _: *mut asymbol,
        _: *mut *const libc::c_char,
        _: *mut libc::c_uint,
    ) -> bool;
    fn _bfd_nodynamic_get_synthetic_symtab(
        _: *mut bfd,
        _: libc::c_long,
        _: *mut *mut asymbol,
        _: libc::c_long,
        _: *mut *mut asymbol,
        _: *mut *mut asymbol,
    ) -> libc::c_long;
    fn _bfd_nodynamic_canonicalize_dynamic_reloc(
        _: *mut bfd,
        _: *mut *mut arelent,
        _: *mut *mut asymbol,
    ) -> libc::c_long;
    fn _bfd_generic_read_minisymbols(
        _: *mut bfd,
        _: bool,
        _: *mut *mut libc::c_void,
        _: *mut libc::c_uint,
    ) -> libc::c_long;
    fn _bfd_generic_minisymbol_to_symbol(
        _: *mut bfd,
        _: bool,
        _: *const libc::c_void,
        _: *mut asymbol,
    ) -> *mut asymbol;
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
    fn _bfd_generic_link_just_syms(_: *mut asection, _: *mut bfd_link_info);
    fn _bfd_generic_copy_link_hash_symbol_type(
        _: *mut bfd,
        _: *mut bfd_link_hash_entry,
        _: *mut bfd_link_hash_entry,
    );
    fn _bfd_generic_link_split_section(_: *mut bfd, _: *mut bfd_section) -> bool;
    fn bfd_assert(_: *const libc::c_char, _: libc::c_int);
    fn _bfd_abort(_: *const libc::c_char, _: libc::c_int, _: *const libc::c_char) -> !;
    fn _bfd_pex64i_swap_sym_in(_: *mut bfd, _: *mut libc::c_void, _: *mut libc::c_void);
    fn _bfd_pex64i_swap_sym_out(
        _: *mut bfd,
        _: *mut libc::c_void,
        _: *mut libc::c_void,
    ) -> libc::c_uint;
    fn _bfd_pex64i_swap_aux_in(
        _: *mut bfd,
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: *mut libc::c_void,
    );
    fn _bfd_pex64i_swap_aux_out(
        _: *mut bfd,
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: *mut libc::c_void,
    ) -> libc::c_uint;
    fn _bfd_pex64i_swap_lineno_in(
        _: *mut bfd,
        _: *mut libc::c_void,
        _: *mut libc::c_void,
    );
    fn _bfd_pex64i_swap_lineno_out(
        _: *mut bfd,
        _: *mut libc::c_void,
        _: *mut libc::c_void,
    ) -> libc::c_uint;
    fn _bfd_pex64i_swap_aouthdr_in(
        _: *mut bfd,
        _: *mut libc::c_void,
        _: *mut libc::c_void,
    );
    fn _bfd_pex64i_swap_aouthdr_out(
        _: *mut bfd,
        _: *mut libc::c_void,
        _: *mut libc::c_void,
    ) -> libc::c_uint;
    fn _bfd_pex64i_swap_scnhdr_out(
        _: *mut bfd,
        _: *mut libc::c_void,
        _: *mut libc::c_void,
    ) -> libc::c_uint;
    fn _bfd_pex64_print_private_bfd_data_common(
        _: *mut bfd,
        _: *mut libc::c_void,
    ) -> bool;
    fn _bfd_pex64_bfd_copy_private_bfd_data_common(_: *mut bfd, _: *mut bfd) -> bool;
    fn _bfd_pex64_get_symbol_info(_: *mut bfd, _: *mut asymbol, _: *mut symbol_info);
    fn _bfd_pex64_only_swap_filehdr_out(
        _: *mut bfd,
        _: *mut libc::c_void,
        _: *mut libc::c_void,
    ) -> libc::c_uint;
    fn _bfd_pex64_bfd_copy_private_section_data(
        _: *mut bfd,
        _: *mut asection,
        _: *mut bfd,
        _: *mut asection,
    ) -> bool;
    fn _bfd_generic_set_reloc(
        abfd: *mut bfd,
        section: sec_ptr,
        relptr: *mut *mut arelent,
        count: libc::c_uint,
    );
    fn bfd_generic_get_relocated_section_contents(
        abfd: *mut bfd,
        link_info: *mut bfd_link_info,
        link_order: *mut bfd_link_order,
        data: *mut bfd_byte,
        relocatable: bool,
        symbols: *mut *mut asymbol,
    ) -> *mut bfd_byte;
    fn bfd_generic_merge_sections(_: *mut bfd, _: *mut bfd_link_info) -> bool;
    fn bfd_generic_lookup_section_flags(
        _: *mut bfd_link_info,
        _: *mut flag_info,
        _: *mut asection,
    ) -> bool;
    fn bfd_generic_relax_section(
        abfd: *mut bfd,
        section: *mut asection,
        _: *mut bfd_link_info,
        _: *mut bool,
    ) -> bool;
    fn _bfd_get_link_info(_: *mut bfd) -> *mut bfd_link_info;
    fn _bfd_pex64i_final_link_postscript(
        _: *mut bfd,
        _: *mut coff_final_link_info,
    ) -> bool;
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
pub type arelent_chain = relent_chain;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct coff_section_alignment_entry {
    pub name: *const libc::c_char,
    pub comparison_length: libc::c_uint,
    pub default_alignment_min: libc::c_uint,
    pub default_alignment_max: libc::c_uint,
    pub alignment_power: libc::c_uint,
}
pub type coff_symbol_type = coff_symbol_struct;
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
#[repr(C)]
pub struct external_reloc {
    pub r_vaddr: [libc::c_char; 4],
    pub r_symndx: [libc::c_char; 4],
    pub r_type: [libc::c_char; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct external_ANON_OBJECT_HEADER_BIGOBJ {
    pub Sig1: [libc::c_char; 2],
    pub Sig2: [libc::c_char; 2],
    pub Version: [libc::c_char; 2],
    pub Machine: [libc::c_char; 2],
    pub TimeDateStamp: [libc::c_char; 4],
    pub ClassID: [libc::c_char; 16],
    pub SizeOfData: [libc::c_char; 4],
    pub Flags: [libc::c_char; 4],
    pub MetaDataSize: [libc::c_char; 4],
    pub MetaDataOffset: [libc::c_char; 4],
    pub NumberOfSections: [libc::c_char; 4],
    pub PointerToSymbolTable: [libc::c_char; 4],
    pub NumberOfSymbols: [libc::c_char; 4],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct external_SYMBOL_EX {
    pub e: C2RustUnnamed_36,
    pub e_value: [libc::c_char; 4],
    pub e_scnum: [libc::c_char; 4],
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
pub union external_AUX_SYMBOL_EX {
    pub Sym: C2RustUnnamed_40,
    pub File: C2RustUnnamed_39,
    pub Section: C2RustUnnamed_38,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_38 {
    pub Length: [libc::c_char; 4],
    pub NumberOfRelocations: [libc::c_char; 2],
    pub NumberOfLinenumbers: [libc::c_char; 2],
    pub Checksum: [libc::c_char; 4],
    pub Number: [libc::c_char; 2],
    pub Selection: [libc::c_char; 1],
    pub bReserved: [libc::c_char; 1],
    pub HighNumber: [libc::c_char; 2],
    pub rgbReserved: [libc::c_char; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_39 {
    pub Name: [libc::c_char; 20],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_40 {
    pub WeakDefaultSymIndex: [libc::c_char; 4],
    pub WeakSearchType: [libc::c_char; 4],
    pub rgbReserved: [libc::c_char; 12],
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
unsafe extern "C" fn bfd_section_vma(mut sec: *const asection) -> bfd_vma {
    return (*sec).vma;
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
unsafe extern "C" fn bfd_get_start_address(mut abfd: *const bfd) -> bfd_vma {
    return (*abfd).start_address;
}
#[inline]
unsafe extern "C" fn bfd_get_symcount(mut abfd: *const bfd) -> libc::c_uint {
    return (*abfd).symcount;
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
unsafe extern "C" fn bfd_family_coff(mut abfd: *const bfd) -> bool {
    return bfd_get_flavour(abfd) as libc::c_uint
        == bfd_target_coff_flavour as libc::c_int as libc::c_uint
        || bfd_get_flavour(abfd) as libc::c_uint
            == bfd_target_xcoff_flavour as libc::c_int as libc::c_uint;
}
#[inline]
unsafe extern "C" fn bfd_applicable_section_flags(mut abfd: *const bfd) -> flagword {
    return (*(*abfd).xvec).section_flags;
}
unsafe extern "C" fn pex64_link_add_symbols(
    mut abfd: *mut bfd,
    mut info: *mut bfd_link_info,
) -> bool {
    if (*info).type_0() as libc::c_int == type_pde as libc::c_int
        && bfd_get_flavour((*info).output_bfd) as libc::c_uint
            == bfd_target_elf_flavour as libc::c_int as libc::c_uint
    {
        let mut h: *mut bfd_link_hash_entry = 0 as *mut bfd_link_hash_entry;
        let mut hi: *mut bfd_link_hash_entry = 0 as *mut bfd_link_hash_entry;
        hi = bfd_link_hash_lookup(
            (*info).hash,
            b"__ImageBase\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int != 0,
            0 as libc::c_int != 0,
            0 as libc::c_int != 0,
        );
        if (*hi).type_0() as libc::c_int == bfd_link_hash_new as libc::c_int
            || (*hi).type_0() as libc::c_int == bfd_link_hash_undefined as libc::c_int
            || (*hi).type_0() as libc::c_int == bfd_link_hash_undefweak as libc::c_int
        {
            h = bfd_link_hash_lookup(
                (*info).hash,
                b"__executable_start\0" as *const u8 as *const libc::c_char,
                1 as libc::c_int != 0,
                0 as libc::c_int != 0,
                1 as libc::c_int != 0,
            );
            (*hi).set_type_0(bfd_link_hash_indirect);
            (*hi).u.i.link = h;
        }
    }
    return _bfd_coff_link_add_symbols(abfd, info);
}
unsafe extern "C" fn coff_amd64_reloc_type_lookup(
    mut _abfd: *mut bfd,
    mut code: bfd_reloc_code_real_type,
) -> *const reloc_howto_type {
    match code as libc::c_uint {
        62 => return howto_table.as_ptr().offset(3 as libc::c_int as isize),
        2 => return howto_table.as_ptr().offset(2 as libc::c_int as isize),
        1 => return howto_table.as_ptr().offset(1 as libc::c_int as isize),
        8 => return howto_table.as_ptr().offset(14 as libc::c_int as isize),
        9 => return howto_table.as_ptr().offset(4 as libc::c_int as isize),
        388 => return howto_table.as_ptr().offset(17 as libc::c_int as isize),
        5 => return howto_table.as_ptr().offset(16 as libc::c_int as isize),
        11 => return howto_table.as_ptr().offset(19 as libc::c_int as isize),
        7 => return howto_table.as_ptr().offset(15 as libc::c_int as isize),
        13 => return howto_table.as_ptr().offset(18 as libc::c_int as isize),
        14 => return howto_table.as_ptr().offset(11 as libc::c_int as isize),
        _ => {
            bfd_assert(
                b"./coff-x86_64.c\0" as *const u8 as *const libc::c_char,
                726 as libc::c_int,
            );
            return 0 as *const reloc_howto_type;
        }
    };
}
unsafe extern "C" fn coff_amd64_reloc_name_lookup(
    mut _abfd: *mut bfd,
    mut r_name: *const libc::c_char,
) -> *const reloc_howto_type {
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong)
        < (::core::mem::size_of::<[reloc_howto_type; 21]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<reloc_howto_type>() as libc::c_ulong)
    {
        if !(howto_table[i as usize].name).is_null()
            && strcasecmp(howto_table[i as usize].name, r_name) == 0 as libc::c_int
        {
            return &*howto_table.as_ptr().offset(i as isize) as *const reloc_howto_type;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as *const reloc_howto_type;
}
unsafe extern "C" fn coff_amd64_is_local_label_name(
    mut abfd: *mut bfd,
    mut name: *const libc::c_char,
) -> bool {
    if *name.offset(0 as libc::c_int as isize) as libc::c_int == 'L' as i32 {
        return 1 as libc::c_int != 0;
    }
    return _bfd_coff_is_local_label_name(abfd, name);
}
static mut pe_saved_coff_bfd_print_private_bfd_data: Option::<
    unsafe extern "C" fn(*mut bfd, *mut libc::c_void) -> bool,
> = None;
unsafe extern "C" fn pe_print_private_bfd_data(
    mut abfd: *mut bfd,
    mut vfile: *mut libc::c_void,
) -> bool {
    let mut file: *mut FILE = vfile as *mut FILE;
    if !_bfd_pex64_print_private_bfd_data_common(abfd, vfile) {
        return 0 as libc::c_int != 0;
    }
    if pe_saved_coff_bfd_print_private_bfd_data.is_none() {
        return 1 as libc::c_int != 0;
    }
    fputc('\n' as i32, file);
    return pe_saved_coff_bfd_print_private_bfd_data
        .expect("non-null function pointer")(abfd, vfile);
}
static mut pe_saved_coff_bfd_copy_private_bfd_data: Option::<
    unsafe extern "C" fn(*mut bfd, *mut bfd) -> bool,
> = None;
unsafe extern "C" fn pe_bfd_copy_private_bfd_data(
    mut ibfd: *mut bfd,
    mut obfd: *mut bfd,
) -> bool {
    if !((*obfd).tdata.pe_obj_data).is_null() && !((*ibfd).tdata.pe_obj_data).is_null()
        && (*(*ibfd).tdata.pe_obj_data).real_flags & 0x20 as libc::c_int as libc::c_uint
            != 0
    {
        (*(*obfd).tdata.pe_obj_data).real_flags |= 0x20 as libc::c_int as libc::c_uint;
    }
    if !_bfd_pex64_bfd_copy_private_bfd_data_common(ibfd, obfd) {
        return 0 as libc::c_int != 0;
    }
    if pe_saved_coff_bfd_copy_private_bfd_data.is_some() {
        return pe_saved_coff_bfd_copy_private_bfd_data
            .expect("non-null function pointer")(ibfd, obfd);
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn coff_swap_reloc_in(
    mut abfd: *mut bfd,
    mut src: *mut libc::c_void,
    mut dst: *mut libc::c_void,
) {
    let mut reloc_src: *mut external_reloc = src as *mut external_reloc;
    let mut reloc_dst: *mut internal_reloc = dst as *mut internal_reloc;
    (*reloc_dst)
        .r_vaddr = (Some(
        ((*(*abfd).xvec).bfd_h_getx32).expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )(((*reloc_src).r_vaddr).as_mut_ptr() as *const libc::c_void);
    (*reloc_dst)
        .r_symndx = (Some(
        ((*(*abfd).xvec).bfd_h_getx_signed_32).expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )(((*reloc_src).r_symndx).as_mut_ptr() as *const libc::c_void);
    (*reloc_dst)
        .r_type = (Some(
        ((*(*abfd).xvec).bfd_h_getx16).expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )(((*reloc_src).r_type).as_mut_ptr() as *const libc::c_void) as libc::c_ushort;
}
unsafe extern "C" fn coff_swap_reloc_out(
    mut abfd: *mut bfd,
    mut src: *mut libc::c_void,
    mut dst: *mut libc::c_void,
) -> libc::c_uint {
    let mut reloc_src: *mut internal_reloc = src as *mut internal_reloc;
    let mut reloc_dst: *mut external_reloc = dst as *mut external_reloc;
    (Some(((*(*abfd).xvec).bfd_h_putx32).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (*reloc_src).r_vaddr,
        ((*reloc_dst).r_vaddr).as_mut_ptr() as *mut libc::c_void,
    );
    (Some(((*(*abfd).xvec).bfd_h_putx32).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (*reloc_src).r_symndx as bfd_vma,
        ((*reloc_dst).r_symndx).as_mut_ptr() as *mut libc::c_void,
    );
    (Some(((*(*abfd).xvec).bfd_h_putx16).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (*reloc_src).r_type as bfd_vma,
        ((*reloc_dst).r_type).as_mut_ptr() as *mut libc::c_void,
    );
    return 10 as libc::c_int as libc::c_uint;
}
unsafe extern "C" fn coff_swap_filehdr_in(
    mut abfd: *mut bfd,
    mut src: *mut libc::c_void,
    mut dst: *mut libc::c_void,
) {
    let mut filehdr_src: *mut external_filehdr = src as *mut external_filehdr;
    let mut filehdr_dst: *mut internal_filehdr = dst as *mut internal_filehdr;
    (*filehdr_dst)
        .f_magic = (Some(
        ((*(*abfd).xvec).bfd_h_getx16).expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )(((*filehdr_src).f_magic).as_mut_ptr() as *const libc::c_void)
        as libc::c_ushort;
    (*filehdr_dst)
        .f_nscns = (Some(
        ((*(*abfd).xvec).bfd_h_getx16).expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )(((*filehdr_src).f_nscns).as_mut_ptr() as *const libc::c_void) as libc::c_uint;
    (*filehdr_dst)
        .f_timdat = (Some(
        ((*(*abfd).xvec).bfd_h_getx32).expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )(((*filehdr_src).f_timdat).as_mut_ptr() as *const libc::c_void) as libc::c_long;
    (*filehdr_dst)
        .f_nsyms = (Some(
        ((*(*abfd).xvec).bfd_h_getx32).expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )(((*filehdr_src).f_nsyms).as_mut_ptr() as *const libc::c_void) as libc::c_long;
    (*filehdr_dst)
        .f_flags = (Some(
        ((*(*abfd).xvec).bfd_h_getx16).expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )(((*filehdr_src).f_flags).as_mut_ptr() as *const libc::c_void)
        as libc::c_ushort;
    (*filehdr_dst)
        .f_symptr = (Some(
        ((*(*abfd).xvec).bfd_h_getx32).expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )(((*filehdr_src).f_symptr).as_mut_ptr() as *const libc::c_void);
    if (*filehdr_dst).f_nsyms != 0 as libc::c_int as libc::c_long
        && (*filehdr_dst).f_symptr == 0 as libc::c_int as libc::c_ulong
    {
        (*filehdr_dst).f_nsyms = 0 as libc::c_int as libc::c_long;
        (*filehdr_dst)
            .f_flags = ((*filehdr_dst).f_flags as libc::c_int | 0x8 as libc::c_int)
            as libc::c_ushort;
    }
    (*filehdr_dst)
        .f_opthdr = (Some(
        ((*(*abfd).xvec).bfd_h_getx16).expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )(((*filehdr_src).f_opthdr).as_mut_ptr() as *const libc::c_void)
        as libc::c_ushort;
}
unsafe extern "C" fn coff_swap_scnhdr_in(
    mut abfd: *mut bfd,
    mut ext: *mut libc::c_void,
    mut in_0: *mut libc::c_void,
) {
    let mut scnhdr_ext: *mut external_scnhdr = ext as *mut external_scnhdr;
    let mut scnhdr_int: *mut internal_scnhdr = in_0 as *mut internal_scnhdr;
    memcpy(
        ((*scnhdr_int).s_name).as_mut_ptr() as *mut libc::c_void,
        ((*scnhdr_ext).s_name).as_mut_ptr() as *const libc::c_void,
        ::core::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong,
    );
    (*scnhdr_int)
        .s_vaddr = (Some(
        ((*(*abfd).xvec).bfd_h_getx32).expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )(((*scnhdr_ext).s_vaddr).as_mut_ptr() as *const libc::c_void);
    (*scnhdr_int)
        .s_paddr = (Some(
        ((*(*abfd).xvec).bfd_h_getx32).expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )(((*scnhdr_ext).s_paddr).as_mut_ptr() as *const libc::c_void);
    (*scnhdr_int)
        .s_size = (Some(
        ((*(*abfd).xvec).bfd_h_getx32).expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )(((*scnhdr_ext).s_size).as_mut_ptr() as *const libc::c_void);
    (*scnhdr_int)
        .s_scnptr = (Some(
        ((*(*abfd).xvec).bfd_h_getx32).expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )(((*scnhdr_ext).s_scnptr).as_mut_ptr() as *const libc::c_void);
    (*scnhdr_int)
        .s_relptr = (Some(
        ((*(*abfd).xvec).bfd_h_getx32).expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )(((*scnhdr_ext).s_relptr).as_mut_ptr() as *const libc::c_void);
    (*scnhdr_int)
        .s_lnnoptr = (Some(
        ((*(*abfd).xvec).bfd_h_getx32).expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )(((*scnhdr_ext).s_lnnoptr).as_mut_ptr() as *const libc::c_void);
    (*scnhdr_int)
        .s_flags = (Some(
        ((*(*abfd).xvec).bfd_h_getx32).expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )(((*scnhdr_ext).s_flags).as_mut_ptr() as *const libc::c_void);
    (*scnhdr_int)
        .s_nreloc = (Some(
        ((*(*abfd).xvec).bfd_h_getx16).expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )(((*scnhdr_ext).s_nreloc).as_mut_ptr() as *const libc::c_void);
    (*scnhdr_int)
        .s_nlnno = (Some(
        ((*(*abfd).xvec).bfd_h_getx16).expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )(((*scnhdr_ext).s_nlnno).as_mut_ptr() as *const libc::c_void);
    if (*scnhdr_int).s_vaddr != 0 as libc::c_int as libc::c_ulong {
        (*scnhdr_int)
            .s_vaddr = ((*scnhdr_int).s_vaddr as libc::c_ulong)
            .wrapping_add((*(*abfd).tdata.pe_obj_data).pe_opthdr.ImageBase) as bfd_vma
            as bfd_vma;
    }
    if (*scnhdr_int).s_paddr > 0 as libc::c_int as libc::c_ulong
        && ((*scnhdr_int).s_flags & 0x80 as libc::c_int as libc::c_ulong
            != 0 as libc::c_int as libc::c_ulong
            && (!startswith(
                (*(*abfd).xvec).name,
                b"pei-\0" as *const u8 as *const libc::c_char,
            ) || (*scnhdr_int).s_size == 0 as libc::c_int as libc::c_ulong)
            || startswith(
                (*(*abfd).xvec).name,
                b"pei-\0" as *const u8 as *const libc::c_char,
            ) as libc::c_int != 0 && (*scnhdr_int).s_size > (*scnhdr_int).s_paddr)
    {
        (*scnhdr_int).s_size = (*scnhdr_int).s_paddr;
    }
}
unsafe extern "C" fn pe_mkobject(mut abfd: *mut bfd) -> bool {
    let mut pe: *mut pe_data_type = 0 as *mut pe_data_type;
    let mut amt: size_t = ::core::mem::size_of::<pe_data_type>() as libc::c_ulong;
    (*abfd).tdata.pe_obj_data = bfd_zalloc(abfd, amt) as *mut pe_tdata;
    if ((*abfd).tdata.pe_obj_data).is_null() {
        return 0 as libc::c_int != 0;
    }
    pe = (*abfd).tdata.pe_obj_data;
    (*pe).coff.pe = 1 as libc::c_int;
    (*pe)
        .in_reloc_p = Some(
        in_reloc_p as unsafe extern "C" fn(*mut bfd, *const reloc_howto_type) -> bool,
    );
    (*pe).dos_message[0 as libc::c_int as usize] = 0xeba1f0e as libc::c_int;
    (*pe)
        .dos_message[1 as libc::c_int
        as usize] = 0xcd09b400 as libc::c_uint as libc::c_int;
    (*pe).dos_message[2 as libc::c_int as usize] = 0x4c01b821 as libc::c_int;
    (*pe).dos_message[3 as libc::c_int as usize] = 0x685421cd as libc::c_int;
    (*pe).dos_message[4 as libc::c_int as usize] = 0x70207369 as libc::c_int;
    (*pe).dos_message[5 as libc::c_int as usize] = 0x72676f72 as libc::c_int;
    (*pe).dos_message[6 as libc::c_int as usize] = 0x63206d61 as libc::c_int;
    (*pe).dos_message[7 as libc::c_int as usize] = 0x6f6e6e61 as libc::c_int;
    (*pe).dos_message[8 as libc::c_int as usize] = 0x65622074 as libc::c_int;
    (*pe).dos_message[9 as libc::c_int as usize] = 0x6e757220 as libc::c_int;
    (*pe).dos_message[10 as libc::c_int as usize] = 0x206e6920 as libc::c_int;
    (*pe).dos_message[11 as libc::c_int as usize] = 0x20534f44 as libc::c_int;
    (*pe).dos_message[12 as libc::c_int as usize] = 0x65646f6d as libc::c_int;
    (*pe).dos_message[13 as libc::c_int as usize] = 0xa0d0d2e as libc::c_int;
    (*pe).dos_message[14 as libc::c_int as usize] = 0x24 as libc::c_int;
    (*pe).dos_message[15 as libc::c_int as usize] = 0 as libc::c_int;
    memset(
        &mut (*pe).pe_opthdr as *mut internal_extra_pe_aouthdr as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<internal_extra_pe_aouthdr>() as libc::c_ulong,
    );
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn pe_mkobject_hook(
    mut abfd: *mut bfd,
    mut filehdr: *mut libc::c_void,
    mut _aouthdr: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut internal_f: *mut internal_filehdr = filehdr as *mut internal_filehdr;
    let mut pe: *mut pe_data_type = 0 as *mut pe_data_type;
    if !pe_mkobject(abfd) {
        return 0 as *mut libc::c_void;
    }
    pe = (*abfd).tdata.pe_obj_data;
    (*pe).coff.sym_filepos = (*internal_f).f_symptr as file_ptr;
    (*pe).coff.local_n_btmask = 0xf as libc::c_int as libc::c_uint;
    (*pe).coff.local_n_btshft = 4 as libc::c_int as libc::c_uint;
    (*pe).coff.local_n_tmask = 0x30 as libc::c_int as libc::c_uint;
    (*pe).coff.local_n_tshift = 2 as libc::c_int as libc::c_uint;
    (*pe).coff.local_symesz = 18 as libc::c_int as libc::c_uint;
    (*pe).coff.local_auxesz = 18 as libc::c_int as libc::c_uint;
    (*pe).coff.local_linesz = (4 as libc::c_int + 2 as libc::c_int) as libc::c_uint;
    (*pe).coff.timestamp = (*internal_f).f_timdat;
    (*(*abfd).tdata.coff_obj_data)
        .conv_table_size = (*internal_f).f_nsyms as libc::c_int;
    (*(*abfd).tdata.coff_obj_data)
        .raw_syment_count = (*(*abfd).tdata.coff_obj_data).conv_table_size
        as libc::c_ulong;
    (*pe).real_flags = (*internal_f).f_flags as flagword;
    if (*internal_f).f_flags as libc::c_int & 0x2000 as libc::c_int != 0 as libc::c_int {
        (*pe).dll = 1 as libc::c_int;
    }
    if (*internal_f).f_flags as libc::c_int & 0x200 as libc::c_int == 0 as libc::c_int {
        (*abfd).flags |= 0x8 as libc::c_int as libc::c_uint;
    }
    memcpy(
        ((*pe).dos_message).as_mut_ptr() as *mut libc::c_void,
        ((*internal_f).pe.dos_message).as_mut_ptr() as *const libc::c_void,
        ::core::mem::size_of::<[libc::c_int; 16]>() as libc::c_ulong,
    );
    return pe as *mut libc::c_void;
}
unsafe extern "C" fn bfd_coff_set_long_section_names_allowed(
    mut abfd: *mut bfd,
    mut enable: libc::c_int,
) -> bool {
    (*((*(*abfd).xvec).backend_data as *mut bfd_coff_backend_data))
        ._bfd_coff_long_section_names = enable != 0;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn sec_to_styp_flags(
    mut sec_name: *const libc::c_char,
    mut sec_flags: flagword,
) -> libc::c_long {
    let mut styp_flags: libc::c_long = 0 as libc::c_int as libc::c_long;
    let mut is_dbg: bool = 0 as libc::c_int != 0;
    if startswith(sec_name, b".debug\0" as *const u8 as *const libc::c_char)
        as libc::c_int != 0
        || startswith(sec_name, b".zdebug\0" as *const u8 as *const libc::c_char)
            as libc::c_int != 0
        || startswith(
            sec_name,
            b".gnu.linkonce.wi.\0" as *const u8 as *const libc::c_char,
        ) as libc::c_int != 0
        || startswith(
            sec_name,
            b".gnu.linkonce.wt.\0" as *const u8 as *const libc::c_char,
        ) as libc::c_int != 0
        || startswith(sec_name, b".stab\0" as *const u8 as *const libc::c_char)
            as libc::c_int != 0
    {
        is_dbg = 1 as libc::c_int != 0;
    }
    if is_dbg {
        sec_flags
            &= (0x20000 as libc::c_int | 0 as libc::c_int
                | (0x40000 as libc::c_int | 0x80000 as libc::c_int)
                | 0x80000 as libc::c_int) as libc::c_uint;
        sec_flags |= (0x2000 as libc::c_int | 0x8 as libc::c_int) as libc::c_uint;
    }
    if sec_flags & 0x10 as libc::c_int as libc::c_uint
        != 0 as libc::c_int as libc::c_uint
    {
        styp_flags |= 0x20 as libc::c_int as libc::c_long;
    }
    if sec_flags & (0x20 as libc::c_int | 0x2000 as libc::c_int) as libc::c_uint
        != 0 as libc::c_int as libc::c_uint
    {
        styp_flags |= 0x40 as libc::c_int as libc::c_long;
    }
    if sec_flags & 0x1 as libc::c_int as libc::c_uint != 0 as libc::c_int as libc::c_uint
        && sec_flags & 0x2 as libc::c_int as libc::c_uint
            == 0 as libc::c_int as libc::c_uint
    {
        styp_flags |= 0x80 as libc::c_int as libc::c_long;
    }
    if sec_flags & 0x1000 as libc::c_int as libc::c_uint
        != 0 as libc::c_int as libc::c_uint
    {
        styp_flags |= 0x1000 as libc::c_int as libc::c_long;
    }
    if sec_flags & 0x2000 as libc::c_int as libc::c_uint
        != 0 as libc::c_int as libc::c_uint
    {
        styp_flags |= 0x2000000 as libc::c_int as libc::c_long;
    }
    if sec_flags & (0x8000 as libc::c_int | 0x200 as libc::c_int) as libc::c_uint
        != 0 as libc::c_int as libc::c_uint && !is_dbg
    {
        styp_flags |= 0x800 as libc::c_int as libc::c_long;
    }
    if sec_flags & 0x20000 as libc::c_int as libc::c_uint != 0 {
        styp_flags |= 0x1000 as libc::c_int as libc::c_long;
    }
    if sec_flags
        & (0 as libc::c_int | (0x40000 as libc::c_int | 0x80000 as libc::c_int)
            | 0x80000 as libc::c_int) as libc::c_uint != 0 as libc::c_int as libc::c_uint
    {
        styp_flags |= 0x1000 as libc::c_int as libc::c_long;
    }
    if sec_flags & 0x40000000 as libc::c_int as libc::c_uint
        == 0 as libc::c_int as libc::c_uint
    {
        styp_flags |= 0x40000000 as libc::c_int as libc::c_long;
    }
    if sec_flags & 0x8 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint
    {
        styp_flags |= 0x80000000 as libc::c_uint as libc::c_long;
    }
    if sec_flags & 0x10 as libc::c_int as libc::c_uint != 0 {
        styp_flags |= 0x20000000 as libc::c_int as libc::c_long;
    }
    if sec_flags & 0x8000000 as libc::c_int as libc::c_uint != 0 {
        styp_flags |= 0x10000000 as libc::c_int as libc::c_long;
    }
    return styp_flags;
}
unsafe extern "C" fn styp_to_sec_flags(
    mut abfd: *mut bfd,
    mut hdr: *mut libc::c_void,
    mut name: *const libc::c_char,
    mut section: *mut asection,
    mut flags_ptr: *mut flagword,
) -> bool {
    let mut internal_s: *mut internal_scnhdr = hdr as *mut internal_scnhdr;
    let mut styp_flags: libc::c_ulong = (*internal_s).s_flags;
    let mut sec_flags: flagword = 0;
    let mut result: bool = 1 as libc::c_int != 0;
    let mut is_dbg: bool = 0 as libc::c_int != 0;
    if startswith(name, b".debug\0" as *const u8 as *const libc::c_char) as libc::c_int
        != 0
        || startswith(name, b".zdebug\0" as *const u8 as *const libc::c_char)
            as libc::c_int != 0
        || startswith(name, b".gnu.linkonce.wi.\0" as *const u8 as *const libc::c_char)
            as libc::c_int != 0
        || startswith(name, b".gnu.linkonce.wt.\0" as *const u8 as *const libc::c_char)
            as libc::c_int != 0
        || startswith(name, b".gnu_debuglink\0" as *const u8 as *const libc::c_char)
            as libc::c_int != 0
        || startswith(name, b".gnu_debugaltlink\0" as *const u8 as *const libc::c_char)
            as libc::c_int != 0
        || startswith(name, b".stab\0" as *const u8 as *const libc::c_char)
            as libc::c_int != 0
    {
        is_dbg = 1 as libc::c_int != 0;
    }
    sec_flags = 0x8 as libc::c_int as flagword;
    if styp_flags & 0x40000000 as libc::c_int as libc::c_ulong
        == 0 as libc::c_int as libc::c_ulong
    {
        sec_flags |= 0x40000000 as libc::c_int as libc::c_uint;
    }
    while styp_flags != 0 {
        let mut flag: libc::c_ulong = styp_flags & styp_flags.wrapping_neg();
        let mut unhandled: *mut libc::c_char = 0 as *mut libc::c_char;
        styp_flags &= !flag;
        match flag {
            1 => {
                unhandled = b"STYP_DSECT\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
            }
            4 => {
                unhandled = b"STYP_GROUP\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
            }
            16 => {
                unhandled = b"STYP_COPY\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
            }
            1024 => {
                unhandled = b"STYP_OVER\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
            }
            2 => {
                sec_flags |= 0x200 as libc::c_int as libc::c_uint;
            }
            1073741824 => {
                sec_flags &= !(0x40000000 as libc::c_int) as libc::c_uint;
            }
            256 => {
                unhandled = b"IMAGE_SCN_LNK_OTHER\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
            }
            67108864 => {
                unhandled = b"IMAGE_SCN_MEM_NOT_CACHED\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char;
            }
            134217728 => {
                _bfd_error_handler(
                    dcgettext(
                        b"bfd\0" as *const u8 as *const libc::c_char,
                        b"%pB: warning: ignoring section flag %s in section %s\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    abfd,
                    b"IMAGE_SCN_MEM_NOT_PAGED\0" as *const u8 as *const libc::c_char,
                    name,
                );
            }
            536870912 => {
                sec_flags |= 0x10 as libc::c_int as libc::c_uint;
            }
            2147483648 => {
                sec_flags &= !(0x8 as libc::c_int) as libc::c_uint;
            }
            33554432 => {
                if is_dbg as libc::c_int != 0
                    || strcmp(name, b".comment\0" as *const u8 as *const libc::c_char)
                        == 0 as libc::c_int
                {
                    sec_flags
                        |= (0x2000 as libc::c_int | 0x8 as libc::c_int) as libc::c_uint;
                }
            }
            268435456 => {
                sec_flags |= 0x8000000 as libc::c_int as libc::c_uint;
            }
            2048 => {
                if !is_dbg {
                    sec_flags |= 0x8000 as libc::c_int as libc::c_uint;
                }
            }
            32 => {
                sec_flags
                    |= (0x10 as libc::c_int | 0x1 as libc::c_int | 0x2 as libc::c_int)
                        as libc::c_uint;
            }
            64 => {
                if is_dbg {
                    sec_flags |= 0x2000 as libc::c_int as libc::c_uint;
                } else {
                    sec_flags
                        |= (0x20 as libc::c_int | 0x1 as libc::c_int
                            | 0x2 as libc::c_int) as libc::c_uint;
                }
            }
            128 => {
                sec_flags |= 0x1 as libc::c_int as libc::c_uint;
            }
            512 => {
                sec_flags |= 0x2000 as libc::c_int as libc::c_uint;
            }
            4096 => {
                sec_flags = handle_COMDAT(abfd, sec_flags, hdr, name, section);
            }
            8 | _ => {}
        }
        if !unhandled.is_null() {
            _bfd_error_handler(
                dcgettext(
                    b"bfd\0" as *const u8 as *const libc::c_char,
                    b"%pB (%s): section flag %s (%#lx) ignored\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                abfd,
                name,
                unhandled,
                flag,
            );
            result = 0 as libc::c_int != 0;
        }
    }
    if bfd_applicable_section_flags(abfd) & 0x400000 as libc::c_int as libc::c_uint
        != 0 as libc::c_int as libc::c_uint
        && (startswith(name, b".sbss\0" as *const u8 as *const libc::c_char)
            as libc::c_int != 0
            || startswith(name, b".sdata\0" as *const u8 as *const libc::c_char)
                as libc::c_int != 0)
    {
        sec_flags |= 0x400000 as libc::c_int as libc::c_uint;
    }
    if startswith(name, b".gnu.linkonce\0" as *const u8 as *const libc::c_char) {
        sec_flags |= (0x20000 as libc::c_int | 0 as libc::c_int) as libc::c_uint;
    }
    if !flags_ptr.is_null() {
        *flags_ptr = sec_flags;
    }
    return result;
}
unsafe extern "C" fn handle_COMDAT(
    mut abfd: *mut bfd,
    mut sec_flags: flagword,
    mut hdr: *mut libc::c_void,
    mut name: *const libc::c_char,
    mut section: *mut asection,
) -> flagword {
    let mut current_block: u64;
    let mut _internal_s: *mut internal_scnhdr = hdr as *mut internal_scnhdr;
    let mut esymstart: *mut bfd_byte = 0 as *mut bfd_byte;
    let mut esym: *mut bfd_byte = 0 as *mut bfd_byte;
    let mut esymend: *mut bfd_byte = 0 as *mut bfd_byte;
    let mut seen_state: libc::c_int = 0 as libc::c_int;
    let mut target_name: *mut libc::c_char = 0 as *mut libc::c_char;
    sec_flags |= 0x20000 as libc::c_int as libc::c_uint;
    if !_bfd_coff_get_external_symbols(abfd) {
        return sec_flags;
    }
    esym = (*(*abfd).tdata.coff_obj_data).external_syms as *mut bfd_byte;
    esymstart = esym;
    esymend = esym
        .offset(
            ((*(*abfd).tdata.coff_obj_data).raw_syment_count)
                .wrapping_mul(
                    (*((*(*abfd).xvec).backend_data as *mut bfd_coff_backend_data))
                        ._bfd_symesz as libc::c_ulong,
                ) as isize,
        );
    while esym < esymend {
        let mut isym: internal_syment = internal_syment {
            _n: C2RustUnnamed_17 {
                _n_name: [0; 8],
            },
            n_value: 0,
            n_scnum: 0,
            n_flags: 0,
            n_type: 0,
            n_sclass: 0,
            n_numaux: 0,
        };
        let mut buf: [libc::c_char; 9] = [0; 9];
        let mut symname: *const libc::c_char = 0 as *const libc::c_char;
        ((*((*(*abfd).xvec).backend_data as *mut bfd_coff_backend_data))
            ._bfd_coff_swap_sym_in)
            .expect(
                "non-null function pointer",
            )(
            abfd,
            esym as *mut libc::c_void,
            &mut isym as *mut internal_syment as *mut libc::c_void,
        );
        if !(::core::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong
            <= 8 as libc::c_int as libc::c_ulong)
        {
            bfd_assert(
                b"./coffcode.h\0" as *const u8 as *const libc::c_char,
                952 as libc::c_int,
            );
        }
        if isym.n_scnum == (*section).target_index {
            symname = _bfd_coff_internal_syment_name(abfd, &mut isym, buf.as_mut_ptr());
            if symname.is_null() {
                _bfd_error_handler(
                    dcgettext(
                        b"bfd\0" as *const u8 as *const libc::c_char,
                        b"%pB: unable to load COMDAT section name\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    abfd,
                );
                break;
            } else {
                match seen_state {
                    0 => {
                        current_block = 8236137900636309791;
                        match current_block {
                            8236137900636309791 => {
                                let mut aux: internal_auxent = internal_auxent {
                                    x_sym: C2RustUnnamed_23 {
                                        x_tagndx: C2RustUnnamed_30 { l: 0 },
                                        x_misc: C2RustUnnamed_28 {
                                            x_lnsz: C2RustUnnamed_29 {
                                                x_lnno: 0,
                                                x_size: 0,
                                            },
                                        },
                                        x_fcnary: C2RustUnnamed_24 {
                                            x_fcn: C2RustUnnamed_26 {
                                                x_lnnoptr: 0,
                                                x_endndx: C2RustUnnamed_27 { l: 0 },
                                            },
                                        },
                                        x_tvndx: 0,
                                    },
                                };
                                if !((isym.n_sclass as libc::c_int == 3 as libc::c_int
                                    || isym.n_sclass as libc::c_int == 2 as libc::c_int)
                                    && isym.n_type as libc::c_int & 0xf as libc::c_int
                                        == 0 as libc::c_int
                                    && isym.n_value == 0 as libc::c_int as libc::c_ulong)
                                {
                                    _bfd_error_handler(
                                        dcgettext(
                                            b"bfd\0" as *const u8 as *const libc::c_char,
                                            b"%pB: error: unexpected symbol '%s' in COMDAT section\0"
                                                as *const u8 as *const libc::c_char,
                                            5 as libc::c_int,
                                        ),
                                        abfd,
                                        symname,
                                    );
                                    break;
                                } else {
                                    if isym.n_sclass as libc::c_int == 3 as libc::c_int
                                        && strcmp(name, symname) != 0 as libc::c_int
                                    {
                                        _bfd_error_handler(
                                            dcgettext(
                                                b"bfd\0" as *const u8 as *const libc::c_char,
                                                b"%pB: warning: COMDAT symbol '%s' does not match section name '%s'\0"
                                                    as *const u8 as *const libc::c_char,
                                                5 as libc::c_int,
                                            ),
                                            abfd,
                                            symname,
                                            name,
                                        );
                                    }
                                    seen_state = 1 as libc::c_int;
                                    if esym
                                        .offset(
                                            (*((*(*abfd).xvec).backend_data
                                                as *mut bfd_coff_backend_data))
                                                ._bfd_symesz as isize,
                                        ) >= esymend
                                    {
                                        _bfd_error_handler(
                                            dcgettext(
                                                b"bfd\0" as *const u8 as *const libc::c_char,
                                                b"%pB: warning: no symbol for section '%s' found\0"
                                                    as *const u8 as *const libc::c_char,
                                                5 as libc::c_int,
                                            ),
                                            abfd,
                                            symname,
                                        );
                                    } else {
                                        ((*((*(*abfd).xvec).backend_data
                                            as *mut bfd_coff_backend_data))
                                            ._bfd_coff_swap_aux_in)
                                            .expect(
                                                "non-null function pointer",
                                            )(
                                            abfd,
                                            esym
                                                .offset(
                                                    (*((*(*abfd).xvec).backend_data
                                                        as *mut bfd_coff_backend_data))
                                                        ._bfd_symesz as isize,
                                                ) as *mut libc::c_void,
                                            isym.n_type as libc::c_int,
                                            isym.n_sclass as libc::c_int,
                                            0 as libc::c_int,
                                            isym.n_numaux as libc::c_int,
                                            &mut aux as *mut internal_auxent as *mut libc::c_void,
                                        );
                                        target_name = strchr(name, '$' as i32);
                                        if !target_name.is_null() {
                                            seen_state = 2 as libc::c_int;
                                            target_name = target_name.offset(1 as libc::c_int as isize);
                                        }
                                        match aux.x_scn.x_comdat as libc::c_int {
                                            1 => {
                                                sec_flags &= !(0x20000 as libc::c_int) as libc::c_uint;
                                            }
                                            2 => {
                                                sec_flags |= 0 as libc::c_int as libc::c_uint;
                                            }
                                            3 => {
                                                sec_flags |= 0x80000 as libc::c_int as libc::c_uint;
                                            }
                                            4 => {
                                                sec_flags
                                                    |= (0x40000 as libc::c_int | 0x80000 as libc::c_int)
                                                        as libc::c_uint;
                                            }
                                            5 => {
                                                sec_flags &= !(0x20000 as libc::c_int) as libc::c_uint;
                                            }
                                            _ => {
                                                sec_flags |= 0 as libc::c_int as libc::c_uint;
                                            }
                                        }
                                    }
                                }
                                current_block = 14220266465818359136;
                            }
                            12534241576630939481 => {
                                if strcmp(
                                    target_name,
                                    symname
                                        .offset(
                                            (if 0 as libc::c_int != 0 {
                                                1 as libc::c_int
                                            } else {
                                                0 as libc::c_int
                                            }) as isize,
                                        ),
                                ) != 0 as libc::c_int
                                {
                                    esym = esym
                                        .offset(
                                            ((isym.n_numaux as libc::c_int + 1 as libc::c_int)
                                                as libc::c_uint)
                                                .wrapping_mul(
                                                    (*((*(*abfd).xvec).backend_data
                                                        as *mut bfd_coff_backend_data))
                                                        ._bfd_symesz,
                                                ) as isize,
                                        );
                                    continue;
                                } else {
                                    current_block = 14389858507636960298;
                                }
                            }
                            _ => {}
                        }
                        match current_block {
                            14220266465818359136 => {}
                            _ => {
                                let mut newname: *mut libc::c_char = 0 as *mut libc::c_char;
                                let mut amt: size_t = 0;
                                amt = ::core::mem::size_of::<coff_comdat_info>()
                                    as libc::c_ulong;
                                let ref mut fresh0 = (*((*section).used_by_bfd
                                    as *mut coff_section_tdata))
                                    .comdat;
                                *fresh0 = bfd_alloc(abfd, amt) as *mut coff_comdat_info;
                                if ((*((*section).used_by_bfd as *mut coff_section_tdata))
                                    .comdat)
                                    .is_null()
                                {
                                    _bfd_abort(
                                        b"./coffcode.h\0" as *const u8 as *const libc::c_char,
                                        1157 as libc::c_int,
                                        (*::core::mem::transmute::<
                                            &[u8; 74],
                                            &[libc::c_char; 74],
                                        >(
                                            b"flagword handle_COMDAT(bfd *, flagword, void *, const char *, asection *)\0",
                                        ))
                                            .as_ptr(),
                                    );
                                }
                                (*(*((*section).used_by_bfd as *mut coff_section_tdata))
                                    .comdat)
                                    .symbol = esym.offset_from(esymstart) as libc::c_long
                                    / (*((*(*abfd).xvec).backend_data
                                        as *mut bfd_coff_backend_data))
                                        ._bfd_symesz as libc::c_long;
                                amt = (strlen(symname))
                                    .wrapping_add(1 as libc::c_int as libc::c_ulong);
                                newname = bfd_alloc(abfd, amt) as *mut libc::c_char;
                                if newname.is_null() {
                                    _bfd_abort(
                                        b"./coffcode.h\0" as *const u8 as *const libc::c_char,
                                        1165 as libc::c_int,
                                        (*::core::mem::transmute::<
                                            &[u8; 74],
                                            &[libc::c_char; 74],
                                        >(
                                            b"flagword handle_COMDAT(bfd *, flagword, void *, const char *, asection *)\0",
                                        ))
                                            .as_ptr(),
                                    );
                                }
                                strcpy(newname, symname);
                                let ref mut fresh1 = (*(*((*section).used_by_bfd
                                    as *mut coff_section_tdata))
                                    .comdat)
                                    .name;
                                *fresh1 = newname;
                                break;
                            }
                        }
                    }
                    2 => {
                        current_block = 12534241576630939481;
                        match current_block {
                            8236137900636309791 => {
                                let mut aux: internal_auxent = internal_auxent {
                                    x_sym: C2RustUnnamed_23 {
                                        x_tagndx: C2RustUnnamed_30 { l: 0 },
                                        x_misc: C2RustUnnamed_28 {
                                            x_lnsz: C2RustUnnamed_29 {
                                                x_lnno: 0,
                                                x_size: 0,
                                            },
                                        },
                                        x_fcnary: C2RustUnnamed_24 {
                                            x_fcn: C2RustUnnamed_26 {
                                                x_lnnoptr: 0,
                                                x_endndx: C2RustUnnamed_27 { l: 0 },
                                            },
                                        },
                                        x_tvndx: 0,
                                    },
                                };
                                if !((isym.n_sclass as libc::c_int == 3 as libc::c_int
                                    || isym.n_sclass as libc::c_int == 2 as libc::c_int)
                                    && isym.n_type as libc::c_int & 0xf as libc::c_int
                                        == 0 as libc::c_int
                                    && isym.n_value == 0 as libc::c_int as libc::c_ulong)
                                {
                                    _bfd_error_handler(
                                        dcgettext(
                                            b"bfd\0" as *const u8 as *const libc::c_char,
                                            b"%pB: error: unexpected symbol '%s' in COMDAT section\0"
                                                as *const u8 as *const libc::c_char,
                                            5 as libc::c_int,
                                        ),
                                        abfd,
                                        symname,
                                    );
                                    break;
                                } else {
                                    if isym.n_sclass as libc::c_int == 3 as libc::c_int
                                        && strcmp(name, symname) != 0 as libc::c_int
                                    {
                                        _bfd_error_handler(
                                            dcgettext(
                                                b"bfd\0" as *const u8 as *const libc::c_char,
                                                b"%pB: warning: COMDAT symbol '%s' does not match section name '%s'\0"
                                                    as *const u8 as *const libc::c_char,
                                                5 as libc::c_int,
                                            ),
                                            abfd,
                                            symname,
                                            name,
                                        );
                                    }
                                    seen_state = 1 as libc::c_int;
                                    if esym
                                        .offset(
                                            (*((*(*abfd).xvec).backend_data
                                                as *mut bfd_coff_backend_data))
                                                ._bfd_symesz as isize,
                                        ) >= esymend
                                    {
                                        _bfd_error_handler(
                                            dcgettext(
                                                b"bfd\0" as *const u8 as *const libc::c_char,
                                                b"%pB: warning: no symbol for section '%s' found\0"
                                                    as *const u8 as *const libc::c_char,
                                                5 as libc::c_int,
                                            ),
                                            abfd,
                                            symname,
                                        );
                                    } else {
                                        ((*((*(*abfd).xvec).backend_data
                                            as *mut bfd_coff_backend_data))
                                            ._bfd_coff_swap_aux_in)
                                            .expect(
                                                "non-null function pointer",
                                            )(
                                            abfd,
                                            esym
                                                .offset(
                                                    (*((*(*abfd).xvec).backend_data
                                                        as *mut bfd_coff_backend_data))
                                                        ._bfd_symesz as isize,
                                                ) as *mut libc::c_void,
                                            isym.n_type as libc::c_int,
                                            isym.n_sclass as libc::c_int,
                                            0 as libc::c_int,
                                            isym.n_numaux as libc::c_int,
                                            &mut aux as *mut internal_auxent as *mut libc::c_void,
                                        );
                                        target_name = strchr(name, '$' as i32);
                                        if !target_name.is_null() {
                                            seen_state = 2 as libc::c_int;
                                            target_name = target_name.offset(1 as libc::c_int as isize);
                                        }
                                        match aux.x_scn.x_comdat as libc::c_int {
                                            1 => {
                                                sec_flags &= !(0x20000 as libc::c_int) as libc::c_uint;
                                            }
                                            2 => {
                                                sec_flags |= 0 as libc::c_int as libc::c_uint;
                                            }
                                            3 => {
                                                sec_flags |= 0x80000 as libc::c_int as libc::c_uint;
                                            }
                                            4 => {
                                                sec_flags
                                                    |= (0x40000 as libc::c_int | 0x80000 as libc::c_int)
                                                        as libc::c_uint;
                                            }
                                            5 => {
                                                sec_flags &= !(0x20000 as libc::c_int) as libc::c_uint;
                                            }
                                            _ => {
                                                sec_flags |= 0 as libc::c_int as libc::c_uint;
                                            }
                                        }
                                    }
                                }
                                current_block = 14220266465818359136;
                            }
                            12534241576630939481 => {
                                if strcmp(
                                    target_name,
                                    symname
                                        .offset(
                                            (if 0 as libc::c_int != 0 {
                                                1 as libc::c_int
                                            } else {
                                                0 as libc::c_int
                                            }) as isize,
                                        ),
                                ) != 0 as libc::c_int
                                {
                                    esym = esym
                                        .offset(
                                            ((isym.n_numaux as libc::c_int + 1 as libc::c_int)
                                                as libc::c_uint)
                                                .wrapping_mul(
                                                    (*((*(*abfd).xvec).backend_data
                                                        as *mut bfd_coff_backend_data))
                                                        ._bfd_symesz,
                                                ) as isize,
                                        );
                                    continue;
                                } else {
                                    current_block = 14389858507636960298;
                                }
                            }
                            _ => {}
                        }
                        match current_block {
                            14220266465818359136 => {}
                            _ => {
                                let mut newname: *mut libc::c_char = 0 as *mut libc::c_char;
                                let mut amt: size_t = 0;
                                amt = ::core::mem::size_of::<coff_comdat_info>()
                                    as libc::c_ulong;
                                let ref mut fresh0 = (*((*section).used_by_bfd
                                    as *mut coff_section_tdata))
                                    .comdat;
                                *fresh0 = bfd_alloc(abfd, amt) as *mut coff_comdat_info;
                                if ((*((*section).used_by_bfd as *mut coff_section_tdata))
                                    .comdat)
                                    .is_null()
                                {
                                    _bfd_abort(
                                        b"./coffcode.h\0" as *const u8 as *const libc::c_char,
                                        1157 as libc::c_int,
                                        (*::core::mem::transmute::<
                                            &[u8; 74],
                                            &[libc::c_char; 74],
                                        >(
                                            b"flagword handle_COMDAT(bfd *, flagword, void *, const char *, asection *)\0",
                                        ))
                                            .as_ptr(),
                                    );
                                }
                                (*(*((*section).used_by_bfd as *mut coff_section_tdata))
                                    .comdat)
                                    .symbol = esym.offset_from(esymstart) as libc::c_long
                                    / (*((*(*abfd).xvec).backend_data
                                        as *mut bfd_coff_backend_data))
                                        ._bfd_symesz as libc::c_long;
                                amt = (strlen(symname))
                                    .wrapping_add(1 as libc::c_int as libc::c_ulong);
                                newname = bfd_alloc(abfd, amt) as *mut libc::c_char;
                                if newname.is_null() {
                                    _bfd_abort(
                                        b"./coffcode.h\0" as *const u8 as *const libc::c_char,
                                        1165 as libc::c_int,
                                        (*::core::mem::transmute::<
                                            &[u8; 74],
                                            &[libc::c_char; 74],
                                        >(
                                            b"flagword handle_COMDAT(bfd *, flagword, void *, const char *, asection *)\0",
                                        ))
                                            .as_ptr(),
                                    );
                                }
                                strcpy(newname, symname);
                                let ref mut fresh1 = (*(*((*section).used_by_bfd
                                    as *mut coff_section_tdata))
                                    .comdat)
                                    .name;
                                *fresh1 = newname;
                                break;
                            }
                        }
                    }
                    1 => {
                        current_block = 14389858507636960298;
                        match current_block {
                            8236137900636309791 => {
                                let mut aux: internal_auxent = internal_auxent {
                                    x_sym: C2RustUnnamed_23 {
                                        x_tagndx: C2RustUnnamed_30 { l: 0 },
                                        x_misc: C2RustUnnamed_28 {
                                            x_lnsz: C2RustUnnamed_29 {
                                                x_lnno: 0,
                                                x_size: 0,
                                            },
                                        },
                                        x_fcnary: C2RustUnnamed_24 {
                                            x_fcn: C2RustUnnamed_26 {
                                                x_lnnoptr: 0,
                                                x_endndx: C2RustUnnamed_27 { l: 0 },
                                            },
                                        },
                                        x_tvndx: 0,
                                    },
                                };
                                if !((isym.n_sclass as libc::c_int == 3 as libc::c_int
                                    || isym.n_sclass as libc::c_int == 2 as libc::c_int)
                                    && isym.n_type as libc::c_int & 0xf as libc::c_int
                                        == 0 as libc::c_int
                                    && isym.n_value == 0 as libc::c_int as libc::c_ulong)
                                {
                                    _bfd_error_handler(
                                        dcgettext(
                                            b"bfd\0" as *const u8 as *const libc::c_char,
                                            b"%pB: error: unexpected symbol '%s' in COMDAT section\0"
                                                as *const u8 as *const libc::c_char,
                                            5 as libc::c_int,
                                        ),
                                        abfd,
                                        symname,
                                    );
                                    break;
                                } else {
                                    if isym.n_sclass as libc::c_int == 3 as libc::c_int
                                        && strcmp(name, symname) != 0 as libc::c_int
                                    {
                                        _bfd_error_handler(
                                            dcgettext(
                                                b"bfd\0" as *const u8 as *const libc::c_char,
                                                b"%pB: warning: COMDAT symbol '%s' does not match section name '%s'\0"
                                                    as *const u8 as *const libc::c_char,
                                                5 as libc::c_int,
                                            ),
                                            abfd,
                                            symname,
                                            name,
                                        );
                                    }
                                    seen_state = 1 as libc::c_int;
                                    if esym
                                        .offset(
                                            (*((*(*abfd).xvec).backend_data
                                                as *mut bfd_coff_backend_data))
                                                ._bfd_symesz as isize,
                                        ) >= esymend
                                    {
                                        _bfd_error_handler(
                                            dcgettext(
                                                b"bfd\0" as *const u8 as *const libc::c_char,
                                                b"%pB: warning: no symbol for section '%s' found\0"
                                                    as *const u8 as *const libc::c_char,
                                                5 as libc::c_int,
                                            ),
                                            abfd,
                                            symname,
                                        );
                                    } else {
                                        ((*((*(*abfd).xvec).backend_data
                                            as *mut bfd_coff_backend_data))
                                            ._bfd_coff_swap_aux_in)
                                            .expect(
                                                "non-null function pointer",
                                            )(
                                            abfd,
                                            esym
                                                .offset(
                                                    (*((*(*abfd).xvec).backend_data
                                                        as *mut bfd_coff_backend_data))
                                                        ._bfd_symesz as isize,
                                                ) as *mut libc::c_void,
                                            isym.n_type as libc::c_int,
                                            isym.n_sclass as libc::c_int,
                                            0 as libc::c_int,
                                            isym.n_numaux as libc::c_int,
                                            &mut aux as *mut internal_auxent as *mut libc::c_void,
                                        );
                                        target_name = strchr(name, '$' as i32);
                                        if !target_name.is_null() {
                                            seen_state = 2 as libc::c_int;
                                            target_name = target_name.offset(1 as libc::c_int as isize);
                                        }
                                        match aux.x_scn.x_comdat as libc::c_int {
                                            1 => {
                                                sec_flags &= !(0x20000 as libc::c_int) as libc::c_uint;
                                            }
                                            2 => {
                                                sec_flags |= 0 as libc::c_int as libc::c_uint;
                                            }
                                            3 => {
                                                sec_flags |= 0x80000 as libc::c_int as libc::c_uint;
                                            }
                                            4 => {
                                                sec_flags
                                                    |= (0x40000 as libc::c_int | 0x80000 as libc::c_int)
                                                        as libc::c_uint;
                                            }
                                            5 => {
                                                sec_flags &= !(0x20000 as libc::c_int) as libc::c_uint;
                                            }
                                            _ => {
                                                sec_flags |= 0 as libc::c_int as libc::c_uint;
                                            }
                                        }
                                    }
                                }
                                current_block = 14220266465818359136;
                            }
                            12534241576630939481 => {
                                if strcmp(
                                    target_name,
                                    symname
                                        .offset(
                                            (if 0 as libc::c_int != 0 {
                                                1 as libc::c_int
                                            } else {
                                                0 as libc::c_int
                                            }) as isize,
                                        ),
                                ) != 0 as libc::c_int
                                {
                                    esym = esym
                                        .offset(
                                            ((isym.n_numaux as libc::c_int + 1 as libc::c_int)
                                                as libc::c_uint)
                                                .wrapping_mul(
                                                    (*((*(*abfd).xvec).backend_data
                                                        as *mut bfd_coff_backend_data))
                                                        ._bfd_symesz,
                                                ) as isize,
                                        );
                                    continue;
                                } else {
                                    current_block = 14389858507636960298;
                                }
                            }
                            _ => {}
                        }
                        match current_block {
                            14220266465818359136 => {}
                            _ => {
                                let mut newname: *mut libc::c_char = 0 as *mut libc::c_char;
                                let mut amt: size_t = 0;
                                amt = ::core::mem::size_of::<coff_comdat_info>()
                                    as libc::c_ulong;
                                let ref mut fresh0 = (*((*section).used_by_bfd
                                    as *mut coff_section_tdata))
                                    .comdat;
                                *fresh0 = bfd_alloc(abfd, amt) as *mut coff_comdat_info;
                                if ((*((*section).used_by_bfd as *mut coff_section_tdata))
                                    .comdat)
                                    .is_null()
                                {
                                    _bfd_abort(
                                        b"./coffcode.h\0" as *const u8 as *const libc::c_char,
                                        1157 as libc::c_int,
                                        (*::core::mem::transmute::<
                                            &[u8; 74],
                                            &[libc::c_char; 74],
                                        >(
                                            b"flagword handle_COMDAT(bfd *, flagword, void *, const char *, asection *)\0",
                                        ))
                                            .as_ptr(),
                                    );
                                }
                                (*(*((*section).used_by_bfd as *mut coff_section_tdata))
                                    .comdat)
                                    .symbol = esym.offset_from(esymstart) as libc::c_long
                                    / (*((*(*abfd).xvec).backend_data
                                        as *mut bfd_coff_backend_data))
                                        ._bfd_symesz as libc::c_long;
                                amt = (strlen(symname))
                                    .wrapping_add(1 as libc::c_int as libc::c_ulong);
                                newname = bfd_alloc(abfd, amt) as *mut libc::c_char;
                                if newname.is_null() {
                                    _bfd_abort(
                                        b"./coffcode.h\0" as *const u8 as *const libc::c_char,
                                        1165 as libc::c_int,
                                        (*::core::mem::transmute::<
                                            &[u8; 74],
                                            &[libc::c_char; 74],
                                        >(
                                            b"flagword handle_COMDAT(bfd *, flagword, void *, const char *, asection *)\0",
                                        ))
                                            .as_ptr(),
                                    );
                                }
                                strcpy(newname, symname);
                                let ref mut fresh1 = (*(*((*section).used_by_bfd
                                    as *mut coff_section_tdata))
                                    .comdat)
                                    .name;
                                *fresh1 = newname;
                                break;
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        esym = esym
            .offset(
                ((isym.n_numaux as libc::c_int + 1 as libc::c_int) as libc::c_uint)
                    .wrapping_mul(
                        (*((*(*abfd).xvec).backend_data as *mut bfd_coff_backend_data))
                            ._bfd_symesz,
                    ) as isize,
            );
    }
    return sec_flags;
}
unsafe extern "C" fn coff_bad_format_hook(
    mut _abfd: *mut bfd,
    mut filehdr: *mut libc::c_void,
) -> bool {
    let mut internal_f: *mut internal_filehdr = filehdr as *mut internal_filehdr;
    if (*internal_f).f_magic as libc::c_int != 0x8664 as libc::c_int
        && (*internal_f).f_magic as libc::c_int
            != 0x8664 as libc::c_int ^ 0x4644 as libc::c_int
        && (*internal_f).f_magic as libc::c_int
            != 0x8664 as libc::c_int ^ 0xadc4 as libc::c_int
        && (*internal_f).f_magic as libc::c_int
            != 0x8664 as libc::c_int ^ 0x7b79 as libc::c_int
        && (*internal_f).f_magic as libc::c_int
            != 0x8664 as libc::c_int ^ 0x1993 as libc::c_int
    {
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn coff_set_custom_section_alignment(
    mut _abfd: *mut bfd,
    mut section: *mut asection,
    mut alignment_table: *const coff_section_alignment_entry,
    table_size: libc::c_uint,
) {
    let default_alignment: libc::c_uint = 2 as libc::c_int as libc::c_uint;
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while i < table_size {
        let mut secname: *const libc::c_char = bfd_section_name(section);
        if if (*alignment_table.offset(i as isize)).comparison_length
            == -(1 as libc::c_int) as libc::c_uint
        {
            (strcmp((*alignment_table.offset(i as isize)).name, secname)
                == 0 as libc::c_int) as libc::c_int
        } else {
            (strncmp(
                (*alignment_table.offset(i as isize)).name,
                secname,
                (*alignment_table.offset(i as isize)).comparison_length as libc::c_ulong,
            ) == 0 as libc::c_int) as libc::c_int
        } != 0
        {
            break;
        }
        i = i.wrapping_add(1);
        i;
    }
    if i >= table_size {
        return;
    }
    if (*alignment_table.offset(i as isize)).default_alignment_min
        != -(1 as libc::c_int) as libc::c_uint
        && default_alignment
            < (*alignment_table.offset(i as isize)).default_alignment_min
    {
        return;
    }
    if (*alignment_table.offset(i as isize)).default_alignment_max
        != -(1 as libc::c_int) as libc::c_uint
        && default_alignment
            > (*alignment_table.offset(i as isize)).default_alignment_max
    {
        return;
    }
    (*section).alignment_power = (*alignment_table.offset(i as isize)).alignment_power;
}
unsafe extern "C" fn coff_new_section_hook(
    mut abfd: *mut bfd,
    mut section: *mut asection,
) -> bool {
    let mut native: *mut combined_entry_type = 0 as *mut combined_entry_type;
    let mut amt: size_t = 0;
    let mut sclass: libc::c_uchar = 3 as libc::c_int as libc::c_uchar;
    (*section).alignment_power = 2 as libc::c_int as libc::c_uint;
    if !_bfd_generic_new_section_hook(abfd, section) {
        return 0 as libc::c_int != 0;
    }
    amt = (::core::mem::size_of::<combined_entry_type>() as libc::c_ulong)
        .wrapping_mul(10 as libc::c_int as libc::c_ulong);
    native = bfd_zalloc(abfd, amt) as *mut combined_entry_type;
    if native.is_null() {
        return 0 as libc::c_int != 0;
    }
    (*native).is_sym = 1 as libc::c_int != 0;
    (*native).u.syment.n_type = 0 as libc::c_int as libc::c_ushort;
    (*native).u.syment.n_sclass = sclass;
    let ref mut fresh2 = (*(&mut (*(*section).symbol).the_bfd as *mut *mut bfd
        as *mut coff_symbol_type))
        .native;
    *fresh2 = native;
    coff_set_custom_section_alignment(
        abfd,
        section,
        coff_section_alignment_table.as_ptr(),
        coff_section_alignment_table_size,
    );
    return 1 as libc::c_int != 0;
}
static mut coff_section_alignment_table_size: libc::c_uint = 0;
static mut coff_section_alignment_table: [coff_section_alignment_entry; 13] = [coff_section_alignment_entry {
    name: 0 as *const libc::c_char,
    comparison_length: 0,
    default_alignment_min: 0,
    default_alignment_max: 0,
    alignment_power: 0,
}; 13];
unsafe extern "C" fn coff_set_arch_mach_hook(
    mut abfd: *mut bfd,
    mut filehdr: *mut libc::c_void,
) -> bool {
    let mut machine: libc::c_ulong = 0;
    let mut arch: bfd_architecture = bfd_arch_unknown;
    let mut internal_f: *mut internal_filehdr = filehdr as *mut internal_filehdr;
    machine = 0 as libc::c_int as libc::c_ulong;
    match (*internal_f).f_magic as libc::c_int {
        34404 | 49184 | 11168 | 64797 | 40951 => {
            arch = bfd_arch_i386;
            machine = ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_ulong;
        }
        _ => {
            arch = bfd_arch_obscure;
        }
    }
    bfd_default_set_arch_mach(abfd, arch, machine);
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn coff_write_relocs(
    mut abfd: *mut bfd,
    mut first_undef: libc::c_int,
) -> bool {
    let mut s: *mut asection = 0 as *mut asection;
    s = (*abfd).sections;
    while !s.is_null() {
        let mut i: libc::c_uint = 0;
        let mut dst: external_reloc = external_reloc {
            r_vaddr: [0; 4],
            r_symndx: [0; 4],
            r_type: [0; 2],
        };
        let mut p: *mut *mut arelent = 0 as *mut *mut arelent;
        p = (*s).orelocation;
        if bfd_seek(abfd, (*s).rel_filepos, 0 as libc::c_int) != 0 as libc::c_int {
            return 0 as libc::c_int != 0;
        }
        if ((*(*abfd).tdata.coff_obj_data).pe != 0
            || (*(*abfd).tdata.coff_obj_data).go32 as libc::c_int != 0)
            && (*s).reloc_count >= 0xffff as libc::c_int as libc::c_uint
        {
            let mut n: internal_reloc = internal_reloc {
                r_vaddr: 0,
                r_symndx: 0,
                r_type: 0,
                r_size: 0,
                r_extern: 0,
                r_offset: 0,
            };
            memset(
                &mut n as *mut internal_reloc as *mut libc::c_void,
                0 as libc::c_int,
                ::core::mem::size_of::<internal_reloc>() as libc::c_ulong,
            );
            n
                .r_vaddr = ((*s).reloc_count)
                .wrapping_add(1 as libc::c_int as libc::c_uint) as bfd_vma;
            coff_swap_reloc_out(
                abfd,
                &mut n as *mut internal_reloc as *mut libc::c_void,
                &mut dst as *mut external_reloc as *mut libc::c_void,
            );
            if bfd_bwrite(
                &mut dst as *mut external_reloc as *const libc::c_void,
                (*((*(*abfd).xvec).backend_data as *mut bfd_coff_backend_data))
                    ._bfd_relsz as bfd_size_type,
                abfd,
            )
                != (*((*(*abfd).xvec).backend_data as *mut bfd_coff_backend_data))
                    ._bfd_relsz as libc::c_ulong
            {
                return 0 as libc::c_int != 0;
            }
        }
        i = 0 as libc::c_int as libc::c_uint;
        while i < (*s).reloc_count {
            let mut n_0: internal_reloc = internal_reloc {
                r_vaddr: 0,
                r_symndx: 0,
                r_type: 0,
                r_size: 0,
                r_extern: 0,
                r_offset: 0,
            };
            let mut q: *mut arelent = *p.offset(i as isize);
            memset(
                &mut n_0 as *mut internal_reloc as *mut libc::c_void,
                0 as libc::c_int,
                ::core::mem::size_of::<internal_reloc>() as libc::c_ulong,
            );
            if !(*((*q).sym_ptr_ptr).offset(0 as libc::c_int as isize)).is_null()
                && (**((*q).sym_ptr_ptr).offset(0 as libc::c_int as isize)).the_bfd
                    != abfd
            {
                let mut j: libc::c_int = 0;
                let mut sname: *const libc::c_char = (**((*q).sym_ptr_ptr)
                    .offset(0 as libc::c_int as isize))
                    .name;
                let mut outsyms: *mut *mut asymbol = (*abfd).outsymbols;
                j = first_undef;
                while !(*outsyms.offset(j as isize)).is_null() {
                    let mut intable: *const libc::c_char = (**outsyms.offset(j as isize))
                        .name;
                    if strcmp(intable, sname) == 0 as libc::c_int {
                        (*q).sym_ptr_ptr = outsyms.offset(j as isize);
                        break;
                    } else {
                        j += 1;
                        j;
                    }
                }
            }
            n_0.r_vaddr = ((*q).address).wrapping_add((*s).vma);
            if !((*q).sym_ptr_ptr).is_null()
                && !(*((*q).sym_ptr_ptr).offset(0 as libc::c_int as isize)).is_null()
            {
                if (**(*q).sym_ptr_ptr).section
                    == &mut *_bfd_std_section
                        .as_mut_ptr()
                        .offset(2 as libc::c_int as isize) as *mut asection
                    && (**(*q).sym_ptr_ptr).flags
                        & ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_uint
                        != 0 as libc::c_int as libc::c_uint
                {
                    n_0.r_symndx = -(1 as libc::c_int) as libc::c_long;
                } else {
                    n_0.r_symndx = (**(*q).sym_ptr_ptr).udata.i as libc::c_long;
                    if n_0.r_symndx
                        > (*(*abfd).tdata.coff_obj_data).conv_table_size as libc::c_long
                    {
                        bfd_set_error(bfd_error_bad_value);
                        _bfd_error_handler(
                            dcgettext(
                                b"bfd\0" as *const u8 as *const libc::c_char,
                                b"%pB: reloc against a non-existent symbol index: %ld\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            abfd,
                            n_0.r_symndx,
                        );
                        return 0 as libc::c_int != 0;
                    }
                }
            }
            n_0.r_type = (*(*q).howto).type_0 as libc::c_ushort;
            coff_swap_reloc_out(
                abfd,
                &mut n_0 as *mut internal_reloc as *mut libc::c_void,
                &mut dst as *mut external_reloc as *mut libc::c_void,
            );
            if bfd_bwrite(
                &mut dst as *mut external_reloc as *const libc::c_void,
                (*((*(*abfd).xvec).backend_data as *mut bfd_coff_backend_data))
                    ._bfd_relsz as bfd_size_type,
                abfd,
            )
                != (*((*(*abfd).xvec).backend_data as *mut bfd_coff_backend_data))
                    ._bfd_relsz as libc::c_ulong
            {
                return 0 as libc::c_int != 0;
            }
            i = i.wrapping_add(1);
            i;
        }
        s = (*s).next;
    }
    return 1 as libc::c_int != 0;
}
#[inline]
unsafe extern "C" fn _bfd_alloc_and_read(
    mut abfd: *mut bfd,
    mut asize: bfd_size_type,
    mut rsize: bfd_size_type,
) -> *mut bfd_byte {
    let mut mem: *mut bfd_byte = 0 as *mut bfd_byte;
    if 0 == 0 {
        let mut filesize: ufile_ptr = bfd_get_file_size(abfd);
        if filesize != 0 as libc::c_int as libc::c_ulong && rsize > filesize {
            bfd_set_error(bfd_error_file_truncated);
            return 0 as *mut bfd_byte;
        }
    }
    mem = bfd_alloc(abfd, asize) as *mut bfd_byte;
    if !mem.is_null() {
        if bfd_bread(mem as *mut libc::c_void, rsize, abfd) == rsize {
            return mem;
        }
        bfd_release(abfd, mem as *mut libc::c_void);
    }
    return 0 as *mut bfd_byte;
}
unsafe extern "C" fn coff_set_flags(
    mut abfd: *mut bfd,
    mut magicp: *mut libc::c_uint,
    mut _flagsp: *mut libc::c_ushort,
) -> bool {
    match bfd_get_arch(abfd) as libc::c_uint {
        8 => {
            *magicp = 0x8664 as libc::c_int as libc::c_uint;
            return 1 as libc::c_int != 0;
        }
        _ => {}
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn coff_set_arch_mach(
    mut abfd: *mut bfd,
    mut arch: bfd_architecture,
    mut machine: libc::c_ulong,
) -> bool {
    let mut dummy1: libc::c_uint = 0;
    let mut dummy2: libc::c_ushort = 0;
    if !bfd_default_set_arch_mach(abfd, arch, machine) {
        return 0 as libc::c_int != 0;
    }
    if arch as libc::c_uint != bfd_arch_unknown as libc::c_int as libc::c_uint
        && !coff_set_flags(abfd, &mut dummy1, &mut dummy2)
    {
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn coff_compute_section_file_positions(mut abfd: *mut bfd) -> bool {
    let mut current: *mut asection = 0 as *mut asection;
    let mut sofar: file_ptr = (*((*(*abfd).xvec).backend_data
        as *mut bfd_coff_backend_data))
        ._bfd_filhsz as file_ptr;
    let mut align_adjust: bool = false;
    let mut target_index: libc::c_uint = 0;
    let mut previous: *mut asection = 0 as *mut asection;
    let mut old_sofar: file_ptr = 0;
    let mut page_size: libc::c_int = 0x1000 as libc::c_int;
    if bfd_get_start_address(abfd) != 0 {
        (*abfd).flags |= 0x2 as libc::c_int as libc::c_uint;
    }
    if (*abfd).flags & 0x2 as libc::c_int as libc::c_uint != 0 {
        sofar
            += (*((*(*abfd).xvec).backend_data as *mut bfd_coff_backend_data))
                ._bfd_aoutsz as libc::c_long;
    }
    sofar
        += ((*abfd).section_count)
            .wrapping_mul(
                (*((*(*abfd).xvec).backend_data as *mut bfd_coff_backend_data))
                    ._bfd_scnhsz,
            ) as libc::c_long;
    target_index = 1 as libc::c_int as libc::c_uint;
    current = (*abfd).sections;
    while !current.is_null() {
        let fresh3 = target_index;
        target_index = target_index.wrapping_add(1);
        (*current).target_index = fresh3 as libc::c_int;
        current = (*current).next;
    }
    if target_index
        >= (*((*(*abfd).xvec).backend_data as *mut bfd_coff_backend_data))
            ._bfd_coff_max_nscns
    {
        bfd_set_error(bfd_error_file_too_big);
        _bfd_error_handler(
            dcgettext(
                b"bfd\0" as *const u8 as *const libc::c_char,
                b"%pB: too many sections (%d)\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            abfd,
            target_index,
        );
        return 0 as libc::c_int != 0;
    }
    align_adjust = 0 as libc::c_int != 0;
    current = (*abfd).sections;
    while !current.is_null() {
        if !((*current).flags & 0x100 as libc::c_int as libc::c_uint == 0) {
            (*current).rawsize = (*current).size;
            if (*abfd).flags & 0x2 as libc::c_int as libc::c_uint
                != 0 as libc::c_int as libc::c_uint
            {
                old_sofar = sofar;
                sofar = (if (sofar as bfd_vma)
                    .wrapping_add(
                        ((1 as libc::c_int) << (*current).alignment_power)
                            as libc::c_ulong,
                    )
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) >= sofar as bfd_vma
                {
                    (sofar as bfd_vma)
                        .wrapping_add(
                            (((1 as libc::c_int) << (*current).alignment_power)
                                - 1 as libc::c_int) as libc::c_ulong,
                        )
                        & !((((1 as libc::c_int) << (*current).alignment_power)
                            - 1 as libc::c_int) as bfd_vma)
                } else {
                    !(0 as libc::c_int as bfd_vma)
                }) as file_ptr;
                if !previous.is_null() {
                    (*previous)
                        .size = ((*previous).size as libc::c_ulong)
                        .wrapping_add((sofar - old_sofar) as libc::c_ulong)
                        as bfd_size_type as bfd_size_type;
                }
            }
            if (*abfd).flags & 0x100 as libc::c_int as libc::c_uint
                != 0 as libc::c_int as libc::c_uint
                && (*current).flags & 0x1 as libc::c_int as libc::c_uint
                    != 0 as libc::c_int as libc::c_uint
            {
                sofar = (sofar as libc::c_ulong)
                    .wrapping_add(
                        ((*current).vma)
                            .wrapping_sub(sofar as bfd_vma)
                            .wrapping_rem(page_size as libc::c_ulong),
                    ) as file_ptr as file_ptr;
            }
            (*current).filepos = sofar;
            sofar = (sofar as libc::c_ulong).wrapping_add((*current).size) as file_ptr
                as file_ptr;
            if (*abfd).flags & 0x2 as libc::c_int as libc::c_uint
                == 0 as libc::c_int as libc::c_uint
            {
                let mut old_size: bfd_size_type = 0;
                old_size = (*current).size;
                (*current)
                    .size = if ((*current).size)
                    .wrapping_add(
                        ((1 as libc::c_int) << (*current).alignment_power)
                            as libc::c_ulong,
                    )
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) >= (*current).size
                {
                    ((*current).size)
                        .wrapping_add(
                            (((1 as libc::c_int) << (*current).alignment_power)
                                - 1 as libc::c_int) as libc::c_ulong,
                        )
                        & !((((1 as libc::c_int) << (*current).alignment_power)
                            - 1 as libc::c_int) as bfd_vma)
                } else {
                    !(0 as libc::c_int as bfd_vma)
                };
                align_adjust = (*current).size != old_size;
                sofar = (sofar as libc::c_ulong)
                    .wrapping_add(((*current).size).wrapping_sub(old_size)) as file_ptr
                    as file_ptr;
            } else {
                old_sofar = sofar;
                sofar = (if (sofar as bfd_vma)
                    .wrapping_add(
                        ((1 as libc::c_int) << (*current).alignment_power)
                            as libc::c_ulong,
                    )
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) >= sofar as bfd_vma
                {
                    (sofar as bfd_vma)
                        .wrapping_add(
                            (((1 as libc::c_int) << (*current).alignment_power)
                                - 1 as libc::c_int) as libc::c_ulong,
                        )
                        & !((((1 as libc::c_int) << (*current).alignment_power)
                            - 1 as libc::c_int) as bfd_vma)
                } else {
                    !(0 as libc::c_int as bfd_vma)
                }) as file_ptr;
                align_adjust = sofar != old_sofar;
                (*current)
                    .size = ((*current).size as libc::c_ulong)
                    .wrapping_add((sofar - old_sofar) as libc::c_ulong) as bfd_size_type
                    as bfd_size_type;
            }
            if strcmp((*current).name, b".lib\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                bfd_set_section_vma(current, 0 as libc::c_int as bfd_vma);
            }
            previous = current;
        }
        current = (*current).next;
    }
    if align_adjust {
        let mut b: bfd_byte = 0;
        b = 0 as libc::c_int as bfd_byte;
        if bfd_seek(abfd, sofar - 1 as libc::c_int as libc::c_long, 0 as libc::c_int)
            != 0 as libc::c_int
            || bfd_bwrite(
                &mut b as *mut bfd_byte as *const libc::c_void,
                1 as libc::c_int as bfd_size_type,
                abfd,
            ) != 1 as libc::c_int as libc::c_ulong
        {
            return 0 as libc::c_int != 0;
        }
    }
    sofar = (if (sofar as bfd_vma)
        .wrapping_add(((1 as libc::c_int) << 2 as libc::c_int) as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong) >= sofar as bfd_vma
    {
        (sofar as bfd_vma)
            .wrapping_add(
                (((1 as libc::c_int) << 2 as libc::c_int) - 1 as libc::c_int)
                    as libc::c_ulong,
            )
            & !((((1 as libc::c_int) << 2 as libc::c_int) - 1 as libc::c_int) as bfd_vma)
    } else {
        !(0 as libc::c_int as bfd_vma)
    }) as file_ptr;
    (*(*abfd).tdata.coff_obj_data).relocbase = sofar as libc::c_ulong;
    (*abfd).set_output_has_begun(1 as libc::c_int as libc::c_uint);
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn coff_amd64_rtype_to_howto(
    mut abfd: *mut bfd,
    mut sec: *mut asection,
    mut rel: *mut internal_reloc,
    mut h: *mut coff_link_hash_entry,
    mut sym: *mut internal_syment,
    mut addendp: *mut bfd_vma,
) -> *const reloc_howto_type {
    let mut howto: *const reloc_howto_type = 0 as *const reloc_howto_type;
    if (*rel).r_type as libc::c_ulong
        >= (::core::mem::size_of::<[reloc_howto_type; 21]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<reloc_howto_type>() as libc::c_ulong)
    {
        bfd_set_error(bfd_error_bad_value);
        return 0 as *const reloc_howto_type;
    }
    howto = howto_table.as_ptr().offset((*rel).r_type as libc::c_int as isize);
    *addendp = 0 as libc::c_int as bfd_vma;
    if (*rel).r_type as libc::c_int >= 5 as libc::c_int
        && (*rel).r_type as libc::c_int <= 9 as libc::c_int
    {
        *addendp = (*addendp as libc::c_ulong)
            .wrapping_sub(((*rel).r_type as libc::c_int - 4 as libc::c_int) as bfd_vma)
            as bfd_vma as bfd_vma;
        (*rel).r_type = 4 as libc::c_int as libc::c_ushort;
    }
    if (*howto).pc_relative() != 0 {
        *addendp = (*addendp as libc::c_ulong).wrapping_add((*sec).vma) as bfd_vma
            as bfd_vma;
    }
    if !sym.is_null() && (*sym).n_scnum == 0 as libc::c_int
        && (*sym).n_value != 0 as libc::c_int as libc::c_ulong
    {
        if h.is_null() {
            bfd_assert(
                b"./coff-x86_64.c\0" as *const u8 as *const libc::c_char,
                616 as libc::c_int,
            );
        }
    }
    if (*howto).pc_relative() != 0 {
        if (*rel).r_type as libc::c_int == 14 as libc::c_int {
            *addendp = (*addendp as libc::c_ulong)
                .wrapping_sub(8 as libc::c_int as libc::c_ulong) as bfd_vma as bfd_vma;
        } else {
            *addendp = (*addendp as libc::c_ulong)
                .wrapping_sub(4 as libc::c_int as libc::c_ulong) as bfd_vma as bfd_vma;
        }
        if !sym.is_null() && (*sym).n_scnum != 0 as libc::c_int {
            *addendp = (*addendp as libc::c_ulong).wrapping_sub((*sym).n_value)
                as bfd_vma as bfd_vma;
        }
    }
    if (*rel).r_type as libc::c_int == 3 as libc::c_int
        && bfd_get_flavour((*(*sec).output_section).owner) as libc::c_uint
            == bfd_target_coff_flavour as libc::c_int as libc::c_uint
    {
        *addendp = (*addendp as libc::c_ulong)
            .wrapping_sub(
                (*(*(*(*sec).output_section).owner).tdata.pe_obj_data)
                    .pe_opthdr
                    .ImageBase,
            ) as bfd_vma as bfd_vma;
    }
    if (*rel).r_type as libc::c_int == 11 as libc::c_int {
        let mut osect_vma: bfd_vma = 0;
        if !h.is_null()
            && (((*h).root).type_0() as libc::c_int
                == bfd_link_hash_defined as libc::c_int
                || ((*h).root).type_0() as libc::c_int
                    == bfd_link_hash_defweak as libc::c_int)
        {
            osect_vma = (*(*(*h).root.u.def.section).output_section).vma;
        } else {
            let mut s: *mut asection = 0 as *mut asection;
            let mut i: libc::c_int = 0;
            s = (*abfd).sections;
            i = 1 as libc::c_int;
            while i < (*sym).n_scnum {
                s = (*s).next;
                i += 1;
                i;
            }
            osect_vma = (*(*s).output_section).vma;
        }
        *addendp = (*addendp as libc::c_ulong).wrapping_sub(osect_vma) as bfd_vma
            as bfd_vma;
    }
    return howto;
}
unsafe extern "C" fn coff_pe_amd64_relocate_section(
    mut output_bfd: *mut bfd,
    mut info: *mut bfd_link_info,
    mut input_bfd: *mut bfd,
    mut input_section: *mut asection,
    mut contents: *mut bfd_byte,
    mut relocs: *mut internal_reloc,
    mut syms: *mut internal_syment,
    mut sections: *mut *mut asection,
) -> bool {
    if (*info).type_0() as libc::c_int == type_relocatable as libc::c_int {
        return 1 as libc::c_int != 0;
    }
    return _bfd_coff_generic_relocate_section(
        output_bfd,
        info,
        input_bfd,
        input_section,
        contents,
        relocs,
        syms,
        sections,
    );
}
static mut howto_table: [reloc_howto_type; 21] = [reloc_howto_type {
    type_0: 0,
    size_bitsize_rightshift_bitpos_complain_on_overflow_negate_pc_relative_partial_inplace_pcrel_offset: [0; 4],
    src_mask: 0,
    dst_mask: 0,
    special_function: None,
    name: 0 as *const libc::c_char,
}; 21];
unsafe extern "C" fn coff_write_object_contents(mut abfd: *mut bfd) -> bool {
    let mut current: *mut asection = 0 as *mut asection;
    let mut hasrelocs: bool = 0 as libc::c_int != 0;
    let mut haslinno: bool = 0 as libc::c_int != 0;
    let mut scn_base: file_ptr = 0;
    let mut reloc_base: file_ptr = 0;
    let mut lineno_base: file_ptr = 0;
    let mut sym_base: file_ptr = 0;
    let mut reloc_size: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut reloc_count: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut lnno_size: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut long_section_names: bool = false;
    let mut text_sec: *mut asection = 0 as *mut asection;
    let mut data_sec: *mut asection = 0 as *mut asection;
    let mut bss_sec: *mut asection = 0 as *mut asection;
    let mut internal_f: internal_filehdr = internal_filehdr {
        pe: internal_extra_pe_filehdr {
            e_magic: 0,
            e_cblp: 0,
            e_cp: 0,
            e_crlc: 0,
            e_cparhdr: 0,
            e_minalloc: 0,
            e_maxalloc: 0,
            e_ss: 0,
            e_sp: 0,
            e_csum: 0,
            e_ip: 0,
            e_cs: 0,
            e_lfarlc: 0,
            e_ovno: 0,
            e_res: [0; 4],
            e_oemid: 0,
            e_oeminfo: 0,
            e_res2: [0; 10],
            e_lfanew: 0,
            dos_message: [0; 16],
            nt_signature: 0,
        },
        f_magic: 0,
        f_nscns: 0,
        f_timdat: 0,
        f_symptr: 0,
        f_nsyms: 0,
        f_opthdr: 0,
        f_flags: 0,
        f_target_id: 0,
    };
    let mut internal_a: internal_aouthdr = internal_aouthdr {
        magic: 0,
        vstamp: 0,
        tsize: 0,
        dsize: 0,
        bsize: 0,
        entry: 0,
        text_start: 0,
        data_start: 0,
        o_toc: 0,
        o_snentry: 0,
        o_sntext: 0,
        o_sndata: 0,
        o_sntoc: 0,
        o_snloader: 0,
        o_snbss: 0,
        o_algntext: 0,
        o_algndata: 0,
        o_modtype: 0,
        o_cputype: 0,
        o_maxstack: 0,
        o_maxdata: 0,
        o_flags: 0,
        o_sntdata: 0,
        o_sntbss: 0,
        o_x64flags: 0,
        bss_start: 0,
        gp_value: 0,
        gprmask: 0,
        cprmask: [0; 4],
        fprmask: 0,
        o_inlib: 0,
        o_sri: 0,
        vid: [0; 2],
        pe: internal_extra_pe_aouthdr {
            Magic: 0,
            MajorLinkerVersion: 0,
            MinorLinkerVersion: 0,
            SizeOfCode: 0,
            SizeOfInitializedData: 0,
            SizeOfUninitializedData: 0,
            AddressOfEntryPoint: 0,
            BaseOfCode: 0,
            BaseOfData: 0,
            ImageBase: 0,
            SectionAlignment: 0,
            FileAlignment: 0,
            MajorOperatingSystemVersion: 0,
            MinorOperatingSystemVersion: 0,
            MajorImageVersion: 0,
            MinorImageVersion: 0,
            MajorSubsystemVersion: 0,
            MinorSubsystemVersion: 0,
            Reserved1: 0,
            SizeOfImage: 0,
            SizeOfHeaders: 0,
            CheckSum: 0,
            Subsystem: 0,
            DllCharacteristics: 0,
            SizeOfStackReserve: 0,
            SizeOfStackCommit: 0,
            SizeOfHeapReserve: 0,
            SizeOfHeapCommit: 0,
            LoaderFlags: 0,
            NumberOfRvaAndSizes: 0,
            DataDirectory: [IMAGE_DATA_DIRECTORY {
                VirtualAddress: 0,
                Size: 0,
            }; 16],
        },
    };
    let mut string_size: size_t = 4 as libc::c_int as size_t;
    bfd_set_error(bfd_error_system_call);
    lnno_size = (coff_count_linenumbers(abfd) as libc::c_uint)
        .wrapping_mul(
            (*((*(*abfd).xvec).backend_data as *mut bfd_coff_backend_data))._bfd_linesz,
        ) as libc::c_ulong;
    if (*abfd).output_has_begun() == 0 {
        if !coff_compute_section_file_positions(abfd) {
            return 0 as libc::c_int != 0;
        }
    }
    reloc_base = (*(*abfd).tdata.coff_obj_data).relocbase as file_ptr;
    current = (*abfd).sections;
    while !current.is_null() {
        if ((*(*abfd).tdata.coff_obj_data).pe != 0
            || (*(*abfd).tdata.coff_obj_data).go32 as libc::c_int != 0)
            && (*current).reloc_count >= 0xffff as libc::c_int as libc::c_uint
        {
            reloc_count = reloc_count.wrapping_add(1);
            reloc_count;
        }
        reloc_count = reloc_count.wrapping_add((*current).reloc_count as libc::c_ulong);
        current = (*current).next;
    }
    reloc_size = reloc_count
        .wrapping_mul(
            (*((*(*abfd).xvec).backend_data as *mut bfd_coff_backend_data))._bfd_relsz
                as libc::c_ulong,
        );
    lineno_base = (reloc_base as libc::c_ulong).wrapping_add(reloc_size) as file_ptr;
    sym_base = (lineno_base as libc::c_ulong).wrapping_add(lnno_size) as file_ptr;
    current = (*abfd).sections;
    while !current.is_null() {
        if (*current).lineno_count != 0 {
            (*current).line_filepos = lineno_base;
            (*current).moving_line_filepos = lineno_base;
            lineno_base
                += ((*current).lineno_count)
                    .wrapping_mul(
                        (*((*(*abfd).xvec).backend_data as *mut bfd_coff_backend_data))
                            ._bfd_linesz,
                    ) as libc::c_long;
        } else {
            (*current).line_filepos = 0 as libc::c_int as file_ptr;
        }
        if (*current).reloc_count != 0 {
            (*current).rel_filepos = reloc_base;
            reloc_base
                += ((*current).reloc_count)
                    .wrapping_mul(
                        (*((*(*abfd).xvec).backend_data as *mut bfd_coff_backend_data))
                            ._bfd_relsz,
                    ) as libc::c_long;
            if ((*(*abfd).tdata.coff_obj_data).pe != 0
                || (*(*abfd).tdata.coff_obj_data).go32 as libc::c_int != 0)
                && (*current).reloc_count >= 0xffff as libc::c_int as libc::c_uint
            {
                reloc_base
                    += (*((*(*abfd).xvec).backend_data as *mut bfd_coff_backend_data))
                        ._bfd_relsz as libc::c_long;
            }
        } else {
            (*current).rel_filepos = 0 as libc::c_int as file_ptr;
        }
        current = (*current).next;
    }
    internal_f.f_nscns = 0 as libc::c_int as libc::c_uint;
    if (*abfd).flags & 0x2 as libc::c_int as libc::c_uint
        != 0 as libc::c_int as libc::c_uint
    {
        scn_base = ((*((*(*abfd).xvec).backend_data as *mut bfd_coff_backend_data))
            ._bfd_filhsz)
            .wrapping_add(
                (*((*(*abfd).xvec).backend_data as *mut bfd_coff_backend_data))
                    ._bfd_aoutsz,
            ) as file_ptr;
    } else {
        scn_base = (*((*(*abfd).xvec).backend_data as *mut bfd_coff_backend_data))
            ._bfd_filhsz as file_ptr;
    }
    if bfd_seek(abfd, scn_base, 0 as libc::c_int) != 0 as libc::c_int {
        return 0 as libc::c_int != 0;
    }
    long_section_names = 0 as libc::c_int != 0;
    current = (*abfd).sections;
    while !current.is_null() {
        let mut section: internal_scnhdr = internal_scnhdr {
            s_name: [0; 8],
            s_paddr: 0,
            s_vaddr: 0,
            s_size: 0,
            s_scnptr: 0,
            s_relptr: 0,
            s_lnnoptr: 0,
            s_nreloc: 0,
            s_nlnno: 0,
            s_flags: 0,
            s_page: 0,
        };
        internal_f.f_nscns = (internal_f.f_nscns).wrapping_add(1);
        internal_f.f_nscns;
        strncpy(
            (section.s_name).as_mut_ptr(),
            (*current).name,
            8 as libc::c_int as libc::c_ulong,
        );
        if (*((*(*abfd).xvec).backend_data as *mut bfd_coff_backend_data))
            ._bfd_coff_long_section_names
        {
            let mut len: size_t = 0;
            len = strlen((*current).name);
            if len > 8 as libc::c_int as libc::c_ulong {
                let mut s_name_buf: [libc::c_char; 29] = [0; 29];
                if string_size >= 10000000 as libc::c_int as libc::c_ulong {
                    bfd_set_error(bfd_error_file_too_big);
                    _bfd_error_handler(
                        dcgettext(
                            b"bfd\0" as *const u8 as *const libc::c_char,
                            b"%pB: section %pA: string table overflow at offset %ld\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        abfd,
                        current,
                        string_size,
                    );
                    return 0 as libc::c_int != 0;
                }
                sprintf(
                    s_name_buf.as_mut_ptr(),
                    b"/%lu\0" as *const u8 as *const libc::c_char,
                    string_size,
                );
                strncpy(
                    (section.s_name).as_mut_ptr(),
                    s_name_buf.as_mut_ptr(),
                    8 as libc::c_int as libc::c_ulong,
                );
                string_size = (string_size as libc::c_ulong)
                    .wrapping_add(len.wrapping_add(1 as libc::c_int as libc::c_ulong))
                    as size_t as size_t;
                long_section_names = 1 as libc::c_int != 0;
            }
        }
        if strcmp((*current).name, b".lib\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            section.s_vaddr = 0 as libc::c_int as bfd_vma;
        } else {
            section.s_vaddr = (*current).vma;
        }
        section.s_paddr = (*current).lma;
        section.s_size = (*current).size;
        section.s_page = 0 as libc::c_int as libc::c_uchar;
        section.s_paddr = 0 as libc::c_int as bfd_vma;
        if (*current).size == 0 as libc::c_int as libc::c_ulong
            || (*current).flags
                & (0x2 as libc::c_int | 0x100 as libc::c_int) as libc::c_uint
                == 0 as libc::c_int as libc::c_uint
        {
            section.s_scnptr = 0 as libc::c_int as bfd_vma;
        } else {
            section.s_scnptr = (*current).filepos as bfd_vma;
        }
        section.s_relptr = (*current).rel_filepos as bfd_vma;
        section.s_lnnoptr = (*current).line_filepos as bfd_vma;
        section.s_nreloc = (*current).reloc_count as libc::c_ulong;
        section.s_nlnno = (*current).lineno_count as libc::c_ulong;
        if (*current).reloc_count != 0 as libc::c_int as libc::c_uint {
            hasrelocs = 1 as libc::c_int != 0;
        }
        if (*current).lineno_count != 0 as libc::c_int as libc::c_uint {
            haslinno = 1 as libc::c_int != 0;
        }
        section
            .s_flags = sec_to_styp_flags((*current).name, (*current).flags)
            as libc::c_ulong;
        if strcmp((*current).name, b".text\0" as *const u8 as *const libc::c_char) == 0 {
            text_sec = current;
        } else if strcmp((*current).name, b".data\0" as *const u8 as *const libc::c_char)
            == 0
        {
            data_sec = current;
        } else if strcmp((*current).name, b".bss\0" as *const u8 as *const libc::c_char)
            == 0
        {
            bss_sec = current;
        }
        if (if (*abfd).flags & (0x2 as libc::c_int | 0x40 as libc::c_int) as libc::c_uint
            != 0 as libc::c_int as libc::c_uint
        {
            0 as libc::c_int
        } else {
            section.s_flags
                |= ((if (*current).alignment_power < 13 as libc::c_int as libc::c_uint {
                    (*current).alignment_power
                } else {
                    13 as libc::c_int as libc::c_uint
                })
                    .wrapping_add(1 as libc::c_int as libc::c_uint) << 20 as libc::c_int)
                    as libc::c_ulong;
            1 as libc::c_int
        }) != 0
            && ((section.s_flags & 0xf00000 as libc::c_int as libc::c_ulong)
                >> 20 as libc::c_int)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                != (*current).alignment_power as libc::c_ulong
        {
            let mut warn: bool = !((*(*abfd).tdata.coff_obj_data).link_info).is_null()
                && !((*(*(*abfd).tdata.coff_obj_data).link_info).type_0() as libc::c_int
                    == type_relocatable as libc::c_int);
            _bfd_error_handler(
                dcgettext(
                    b"bfd\0" as *const u8 as *const libc::c_char,
                    b"%pB:%s section %s: alignment 2**%u not representable\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                abfd,
                if warn as libc::c_int != 0 {
                    b" warning:\0" as *const u8 as *const libc::c_char
                } else {
                    b"\0" as *const u8 as *const libc::c_char
                },
                (*current).name,
                (*current).alignment_power,
            );
            if !warn {
                bfd_set_error(bfd_error_nonrepresentable_section);
                return 0 as libc::c_int != 0;
            }
        }
        let mut buff: external_scnhdr = external_scnhdr {
            s_name: [0; 8],
            s_paddr: [0; 4],
            s_vaddr: [0; 4],
            s_size: [0; 4],
            s_scnptr: [0; 4],
            s_relptr: [0; 4],
            s_lnnoptr: [0; 4],
            s_nreloc: [0; 2],
            s_nlnno: [0; 2],
            s_flags: [0; 4],
        };
        let mut amt: bfd_size_type = (*((*(*abfd).xvec).backend_data
            as *mut bfd_coff_backend_data))
            ._bfd_scnhsz as bfd_size_type;
        if ((*((*(*abfd).xvec).backend_data as *mut bfd_coff_backend_data))
            ._bfd_coff_swap_scnhdr_out)
            .expect(
                "non-null function pointer",
            )(
            abfd,
            &mut section as *mut internal_scnhdr as *mut libc::c_void,
            &mut buff as *mut external_scnhdr as *mut libc::c_void,
        ) == 0 as libc::c_int as libc::c_uint
            || bfd_bwrite(
                &mut buff as *mut external_scnhdr as *const libc::c_void,
                amt,
                abfd,
            ) != amt
        {
            return 0 as libc::c_int != 0;
        }
        if (*current).flags & 0x20000 as libc::c_int as libc::c_uint
            != 0 as libc::c_int as libc::c_uint
        {
            let mut i: libc::c_uint = 0;
            let mut count: libc::c_uint = 0;
            let mut psym: *mut *mut asymbol = 0 as *mut *mut asymbol;
            let mut csym: *mut coff_symbol_type = 0 as *mut coff_symbol_type;
            let mut psymsec: *mut *mut asymbol = 0 as *mut *mut asymbol;
            psymsec = 0 as *mut *mut asymbol;
            count = bfd_get_symcount(abfd);
            i = 0 as libc::c_int as libc::c_uint;
            psym = (*abfd).outsymbols;
            while i < count {
                if !((**psym).section != current) {
                    if psymsec.is_null() {
                        psymsec = psym;
                    }
                    if strcmp((**psym).name, (*current).name) == 0 as libc::c_int {
                        csym = if bfd_family_coff(bfd_asymbol_bfd(*psym)) as libc::c_int
                            != 0
                            && !((*bfd_asymbol_bfd(*psym)).tdata.coff_obj_data).is_null()
                        {
                            *psym as *mut coff_symbol_type
                        } else {
                            0 as *mut coff_symbol_type
                        };
                        if !(csym.is_null() || ((*csym).native).is_null()
                            || !(*(*csym).native).is_sym
                            || ((*(*csym).native).u.syment.n_numaux as libc::c_int)
                                < 1 as libc::c_int
                            || (*(*csym).native).u.syment.n_sclass as libc::c_int
                                != 3 as libc::c_int
                            || (*(*csym).native).u.syment.n_type as libc::c_int
                                != 0 as libc::c_int)
                        {
                            break;
                        }
                    }
                }
                i = i.wrapping_add(1);
                i;
                psym = psym.offset(1);
                psym;
            }
            if i < count {
                let mut aux: *mut combined_entry_type = 0 as *mut combined_entry_type;
                aux = ((*csym).native).offset(1 as libc::c_int as isize);
                if (*aux).is_sym {
                    bfd_assert(
                        b"./coffcode.h\0" as *const u8 as *const libc::c_char,
                        3737 as libc::c_int,
                    );
                }
                match (*current).flags & 0xc0000 as libc::c_int as libc::c_uint {
                    0 => {
                        (*aux)
                            .u
                            .auxent
                            .x_scn
                            .x_comdat = 2 as libc::c_int as libc::c_uchar;
                    }
                    262144 => {
                        (*aux)
                            .u
                            .auxent
                            .x_scn
                            .x_comdat = 1 as libc::c_int as libc::c_uchar;
                    }
                    524288 => {
                        (*aux)
                            .u
                            .auxent
                            .x_scn
                            .x_comdat = 3 as libc::c_int as libc::c_uchar;
                    }
                    786432 => {
                        (*aux)
                            .u
                            .auxent
                            .x_scn
                            .x_comdat = 4 as libc::c_int as libc::c_uchar;
                    }
                    _ => {}
                }
                if psym != psymsec {
                    let mut hold: *mut asymbol = 0 as *mut asymbol;
                    let mut pcopy: *mut *mut asymbol = 0 as *mut *mut asymbol;
                    hold = *psym;
                    pcopy = psym;
                    while pcopy > psymsec {
                        let ref mut fresh4 = *pcopy.offset(0 as libc::c_int as isize);
                        *fresh4 = *pcopy.offset(-(1 as libc::c_int) as isize);
                        pcopy = pcopy.offset(-1);
                        pcopy;
                    }
                    *psymsec = hold;
                }
            }
        }
        current = (*current).next;
    }
    internal_f.f_timdat = 0 as libc::c_int as libc::c_long;
    internal_f.f_flags = 0 as libc::c_int as libc::c_ushort;
    if (*abfd).flags & 0x2 as libc::c_int as libc::c_uint != 0 {
        internal_f
            .f_opthdr = (*((*(*abfd).xvec).backend_data as *mut bfd_coff_backend_data))
            ._bfd_aoutsz as libc::c_ushort;
    } else {
        internal_f.f_opthdr = 0 as libc::c_int as libc::c_ushort;
    }
    if !hasrelocs {
        internal_f
            .f_flags = (internal_f.f_flags as libc::c_int | 0x1 as libc::c_int)
            as libc::c_ushort;
    }
    if !haslinno {
        internal_f
            .f_flags = (internal_f.f_flags as libc::c_int | 0x4 as libc::c_int)
            as libc::c_ushort;
    }
    if (*abfd).flags & 0x2 as libc::c_int as libc::c_uint != 0 {
        internal_f
            .f_flags = (internal_f.f_flags as libc::c_int | 0x2 as libc::c_int)
            as libc::c_ushort;
    }
    memset(
        &mut internal_a as *mut internal_aouthdr as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<internal_aouthdr>() as libc::c_ulong,
    );
    let mut magic: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut flags: libc::c_ushort = 0 as libc::c_int as libc::c_ushort;
    coff_set_flags(abfd, &mut magic, &mut flags);
    internal_f.f_magic = magic as libc::c_ushort;
    internal_f
        .f_flags = (internal_f.f_flags as libc::c_int | flags as libc::c_int)
        as libc::c_ushort;
    internal_a.magic = 0x20b as libc::c_int as libc::c_short;
    internal_a.vstamp = 0 as libc::c_int as libc::c_short;
    (*(*abfd).tdata.coff_obj_data).sym_filepos = sym_base;
    if bfd_get_symcount(abfd) != 0 as libc::c_int as libc::c_uint {
        let mut firstundef: libc::c_int = 0;
        if !coff_renumber_symbols(abfd, &mut firstundef) {
            return 0 as libc::c_int != 0;
        }
        coff_mangle_symbols(abfd);
        if !coff_write_symbols(abfd) {
            return 0 as libc::c_int != 0;
        }
        if !coff_write_linenumbers(abfd) {
            return 0 as libc::c_int != 0;
        }
        if !coff_write_relocs(abfd, firstundef) {
            return 0 as libc::c_int != 0;
        }
    } else if long_section_names as libc::c_int != 0
        && !(*(*abfd).tdata.coff_obj_data).strings_written
    {
        if !coff_write_symbols(abfd) {
            return 0 as libc::c_int != 0;
        }
    }
    if (*(*abfd).tdata.coff_obj_data).raw_syment_count
        != 0 as libc::c_int as libc::c_ulong
    {
        internal_f.f_symptr = sym_base as bfd_vma;
    } else {
        if long_section_names {
            internal_f.f_symptr = sym_base as bfd_vma;
        } else {
            internal_f.f_symptr = 0 as libc::c_int as bfd_vma;
        }
        internal_f
            .f_flags = (internal_f.f_flags as libc::c_int | 0x8 as libc::c_int)
            as libc::c_ushort;
    }
    if !text_sec.is_null() {
        internal_a.tsize = (*text_sec).size;
        internal_a
            .text_start = if internal_a.tsize != 0 {
            (*text_sec).vma
        } else {
            0 as libc::c_int as libc::c_ulong
        };
    }
    if !data_sec.is_null() {
        internal_a.dsize = (*data_sec).size;
        internal_a
            .data_start = if internal_a.dsize != 0 {
            (*data_sec).vma
        } else {
            0 as libc::c_int as libc::c_ulong
        };
    }
    if !bss_sec.is_null() {
        internal_a.bsize = (*bss_sec).size;
        if internal_a.bsize != 0 && (*bss_sec).vma < internal_a.data_start {
            internal_a.data_start = (*bss_sec).vma;
        }
    }
    internal_a.entry = bfd_get_start_address(abfd);
    internal_f.f_nsyms = (*(*abfd).tdata.coff_obj_data).raw_syment_count as libc::c_long;
    let mut pe: *mut pe_tdata = (*abfd).tdata.pe_obj_data;
    if ((*pe).build_id.after_write_object_contents).is_some() {
        (Some(
            ((*pe).build_id.after_write_object_contents)
                .expect("non-null function pointer"),
        ))
            .expect("non-null function pointer")(abfd);
    }
    if bfd_seek(abfd, 0 as libc::c_int as file_ptr, 0 as libc::c_int) != 0 as libc::c_int
    {
        return 0 as libc::c_int != 0;
    }
    let mut buff_0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut amount: bfd_size_type = (*((*(*abfd).xvec).backend_data
        as *mut bfd_coff_backend_data))
        ._bfd_filhsz as bfd_size_type;
    buff_0 = bfd_malloc(amount) as *mut libc::c_char;
    if buff_0.is_null() {
        return 0 as libc::c_int != 0;
    }
    ((*((*(*abfd).xvec).backend_data as *mut bfd_coff_backend_data))
        ._bfd_coff_swap_filehdr_out)
        .expect(
            "non-null function pointer",
        )(
        abfd,
        &mut internal_f as *mut internal_filehdr as *mut libc::c_void,
        buff_0 as *mut libc::c_void,
    );
    amount = bfd_bwrite(buff_0 as *const libc::c_void, amount, abfd);
    free(buff_0 as *mut libc::c_void);
    if amount
        != (*((*(*abfd).xvec).backend_data as *mut bfd_coff_backend_data))._bfd_filhsz
            as libc::c_ulong
    {
        return 0 as libc::c_int != 0;
    }
    if (*abfd).flags & 0x2 as libc::c_int as libc::c_uint != 0 {
        let mut buff_1: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut amount_0: bfd_size_type = (*((*(*abfd).xvec).backend_data
            as *mut bfd_coff_backend_data))
            ._bfd_aoutsz as bfd_size_type;
        buff_1 = bfd_malloc(amount_0) as *mut libc::c_char;
        if buff_1.is_null() {
            return 0 as libc::c_int != 0;
        }
        _bfd_pex64i_swap_aouthdr_out(
            abfd,
            &mut internal_a as *mut internal_aouthdr as *mut libc::c_void,
            buff_1 as *mut libc::c_void,
        );
        amount_0 = bfd_bwrite(buff_1 as *const libc::c_void, amount_0, abfd);
        free(buff_1 as *mut libc::c_void);
        if amount_0
            != (*((*(*abfd).xvec).backend_data as *mut bfd_coff_backend_data))
                ._bfd_aoutsz as libc::c_ulong
        {
            return 0 as libc::c_int != 0;
        }
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn in_reloc_p(
    mut _abfd: *mut bfd,
    mut howto: *const reloc_howto_type,
) -> bool {
    return (*howto).pc_relative() == 0
        && (*howto).type_0 != 3 as libc::c_int as libc::c_uint
        && (*howto).type_0 != 11 as libc::c_int as libc::c_uint;
}
unsafe extern "C" fn coff_set_section_contents(
    mut abfd: *mut bfd,
    mut section: sec_ptr,
    mut location: *const libc::c_void,
    mut offset: file_ptr,
    mut count: bfd_size_type,
) -> bool {
    if (*abfd).output_has_begun() == 0 {
        if !coff_compute_section_file_positions(abfd) {
            return 0 as libc::c_int != 0;
        }
    }
    if strcmp((*section).name, b".lib\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        let mut rec: *mut bfd_byte = 0 as *mut bfd_byte;
        let mut recend: *mut bfd_byte = 0 as *mut bfd_byte;
        rec = location as *mut bfd_byte;
        recend = rec.offset(count as isize);
        while rec < recend {
            (*section).lma = ((*section).lma).wrapping_add(1);
            (*section).lma;
            rec = rec
                .offset(
                    ((Some(
                        ((*(*abfd).xvec).bfd_getx32).expect("non-null function pointer"),
                    ))
                        .expect("non-null function pointer")(rec as *const libc::c_void))
                        .wrapping_mul(4 as libc::c_int as libc::c_ulong) as isize,
                );
        }
        if !(rec == recend) {
            bfd_assert(
                b"./coffcode.h\0" as *const u8 as *const libc::c_char,
                4260 as libc::c_int,
            );
        }
    }
    if (*section).filepos == 0 as libc::c_int as libc::c_long {
        return 1 as libc::c_int != 0;
    }
    if bfd_seek(abfd, (*section).filepos + offset, 0 as libc::c_int) != 0 as libc::c_int
    {
        return 0 as libc::c_int != 0;
    }
    if count == 0 as libc::c_int as libc::c_ulong {
        return 1 as libc::c_int != 0;
    }
    return bfd_bwrite(location, count, abfd) == count;
}
unsafe extern "C" fn coff_slurp_line_table(
    mut abfd: *mut bfd,
    mut asect: *mut asection,
) -> bool {
    let mut native_lineno: *mut external_lineno = 0 as *mut external_lineno;
    let mut lineno_cache: *mut alent = 0 as *mut alent;
    let mut counter: libc::c_uint = 0;
    let mut cache_ptr: *mut alent = 0 as *mut alent;
    let mut prev_offset: bfd_vma = 0 as libc::c_int as bfd_vma;
    let mut ordered: bool = 1 as libc::c_int != 0;
    let mut nbr_func: libc::c_uint = 0;
    let mut src: *mut external_lineno = 0 as *mut external_lineno;
    let mut have_func: bool = false;
    let mut ret: bool = 1 as libc::c_int != 0;
    let mut amt: size_t = 0;
    if (*asect).lineno_count == 0 as libc::c_int as libc::c_uint {
        return 1 as libc::c_int != 0;
    }
    if !((*asect).lineno).is_null() {
        bfd_assert(
            b"./coffcode.h\0" as *const u8 as *const libc::c_char,
            4353 as libc::c_int,
        );
    }
    if (*asect).lineno_count as libc::c_ulong > (*asect).size {
        _bfd_error_handler(
            dcgettext(
                b"bfd\0" as *const u8 as *const libc::c_char,
                b"%pB: warning: line number count (%#lx) exceeds section size (%#lx)\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            abfd,
            (*asect).lineno_count as libc::c_ulong,
            (*asect).size,
        );
        return 0 as libc::c_int != 0;
    }
    amt = ((*asect).lineno_count).wrapping_add(1 as libc::c_int as libc::c_uint)
        as size_t;
    amt = (amt as libc::c_ulong)
        .wrapping_mul(::core::mem::size_of::<alent>() as libc::c_ulong) as size_t
        as size_t;
    if ::core::mem::size_of::<alent>() as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
        && amt.wrapping_div(::core::mem::size_of::<alent>() as libc::c_ulong)
            != ((*asect).lineno_count).wrapping_add(1 as libc::c_int as libc::c_uint)
                as libc::c_ulong
    {
        bfd_set_error(bfd_error_file_too_big);
        return 0 as libc::c_int != 0;
    }
    lineno_cache = bfd_alloc(abfd, amt) as *mut alent;
    if lineno_cache.is_null() {
        return 0 as libc::c_int != 0;
    }
    native_lineno = buy_and_read(
        abfd,
        (*asect).line_filepos,
        (*asect).lineno_count as bfd_size_type,
        (*((*(*abfd).xvec).backend_data as *mut bfd_coff_backend_data))._bfd_linesz
            as bfd_size_type,
    ) as *mut external_lineno;
    if native_lineno.is_null() {
        _bfd_error_handler(
            dcgettext(
                b"bfd\0" as *const u8 as *const libc::c_char,
                b"%pB: warning: line number table read failed\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            abfd,
        );
        bfd_release(abfd, lineno_cache as *mut libc::c_void);
        return 0 as libc::c_int != 0;
    }
    cache_ptr = lineno_cache;
    (*asect).lineno = lineno_cache;
    src = native_lineno;
    nbr_func = 0 as libc::c_int as libc::c_uint;
    have_func = 0 as libc::c_int != 0;
    let mut current_block_55: u64;
    counter = 0 as libc::c_int as libc::c_uint;
    while counter < (*asect).lineno_count {
        let mut dst: internal_lineno = internal_lineno {
            l_addr: C2RustUnnamed_34 { l_symndx: 0 },
            l_lnno: 0,
        };
        ((*((*(*abfd).xvec).backend_data as *mut bfd_coff_backend_data))
            ._bfd_coff_swap_lineno_in)
            .expect(
                "non-null function pointer",
            )(
            abfd,
            src as *mut libc::c_void,
            &mut dst as *mut internal_lineno as *mut libc::c_void,
        );
        (*cache_ptr).line_number = dst.l_lnno as libc::c_uint;
        memset(
            &mut (*cache_ptr).u as *mut C2RustUnnamed_7 as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<C2RustUnnamed_7>() as libc::c_ulong,
        );
        if (*cache_ptr).line_number == 0 as libc::c_int as libc::c_uint {
            let mut ent: *mut combined_entry_type = 0 as *mut combined_entry_type;
            let mut symndx: libc::c_ulong = 0;
            let mut sym: *mut coff_symbol_type = 0 as *mut coff_symbol_type;
            have_func = 0 as libc::c_int != 0;
            symndx = dst.l_addr.l_symndx as libc::c_ulong;
            if symndx >= (*(*abfd).tdata.coff_obj_data).raw_syment_count {
                _bfd_error_handler(
                    dcgettext(
                        b"bfd\0" as *const u8 as *const libc::c_char,
                        b"%pB: warning: illegal symbol index 0x%lx in line number entry %d\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    abfd,
                    symndx,
                    counter,
                );
                (*cache_ptr).line_number = -(1 as libc::c_int) as libc::c_uint;
                ret = 0 as libc::c_int != 0;
                current_block_55 = 11194104282611034094;
            } else {
                ent = ((*(*abfd).tdata.coff_obj_data).raw_syments)
                    .offset(symndx as isize);
                if !(*ent).is_sym {
                    _bfd_error_handler(
                        dcgettext(
                            b"bfd\0" as *const u8 as *const libc::c_char,
                            b"%pB: warning: illegal symbol index 0x%lx in line number entry %d\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        abfd,
                        symndx,
                        counter,
                    );
                    (*cache_ptr).line_number = -(1 as libc::c_int) as libc::c_uint;
                    ret = 0 as libc::c_int != 0;
                    current_block_55 = 11194104282611034094;
                } else {
                    sym = (*ent).u.syment._n._n_n._n_zeroes as *mut coff_symbol_type;
                    if sym < (*(*abfd).tdata.coff_obj_data).symbols
                        || sym
                            >= ((*(*abfd).tdata.coff_obj_data).symbols)
                                .offset(bfd_get_symcount(abfd) as isize)
                    {
                        _bfd_error_handler(
                            dcgettext(
                                b"bfd\0" as *const u8 as *const libc::c_char,
                                b"%pB: warning: illegal symbol in line number entry %d\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            abfd,
                            counter,
                        );
                        (*cache_ptr).line_number = -(1 as libc::c_int) as libc::c_uint;
                        ret = 0 as libc::c_int != 0;
                        current_block_55 = 11194104282611034094;
                    } else {
                        have_func = 1 as libc::c_int != 0;
                        nbr_func = nbr_func.wrapping_add(1);
                        nbr_func;
                        (*cache_ptr).u.sym = sym as *mut asymbol;
                        if !((*sym).lineno).is_null() {
                            _bfd_error_handler(
                                dcgettext(
                                    b"bfd\0" as *const u8 as *const libc::c_char,
                                    b"%pB: warning: duplicate line number information for `%s'\0"
                                        as *const u8 as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                                abfd,
                                bfd_asymbol_name(&mut (*sym).symbol),
                            );
                        }
                        (*sym).lineno = cache_ptr;
                        if (*sym).symbol.value < prev_offset {
                            ordered = 0 as libc::c_int != 0;
                        }
                        prev_offset = (*sym).symbol.value;
                        current_block_55 = 13678349939556791712;
                    }
                }
            }
        } else if !have_func {
            current_block_55 = 11194104282611034094;
        } else {
            (*cache_ptr)
                .u
                .offset = (dst.l_addr.l_paddr as libc::c_ulong)
                .wrapping_sub(bfd_section_vma(asect));
            current_block_55 = 13678349939556791712;
        }
        match current_block_55 {
            13678349939556791712 => {
                cache_ptr = cache_ptr.offset(1);
                cache_ptr;
            }
            _ => {}
        }
        counter = counter.wrapping_add(1);
        counter;
        src = src.offset(1);
        src;
    }
    (*asect)
        .lineno_count = cache_ptr.offset_from(lineno_cache) as libc::c_long
        as libc::c_uint;
    memset(
        cache_ptr as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<alent>() as libc::c_ulong,
    );
    bfd_release(abfd, native_lineno as *mut libc::c_void);
    if !ordered {
        let mut func_table: *mut *mut alent = 0 as *mut *mut alent;
        let mut n_lineno_cache: *mut alent = 0 as *mut alent;
        amt = nbr_func as size_t;
        amt = (amt as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut alent>() as libc::c_ulong)
            as size_t as size_t;
        if ::core::mem::size_of::<*mut alent>() as libc::c_ulong
            != 0 as libc::c_int as libc::c_ulong
            && amt.wrapping_div(::core::mem::size_of::<*mut alent>() as libc::c_ulong)
                != nbr_func as libc::c_ulong
        {
            bfd_set_error(bfd_error_file_too_big);
            ret = 0 as libc::c_int != 0;
        } else {
            func_table = bfd_alloc(abfd, amt) as *mut *mut alent;
            if !func_table.is_null() {
                let mut p: *mut *mut alent = func_table;
                let mut i: libc::c_uint = 0;
                i = 0 as libc::c_int as libc::c_uint;
                while i < (*asect).lineno_count {
                    if (*lineno_cache.offset(i as isize)).line_number
                        == 0 as libc::c_int as libc::c_uint
                    {
                        let fresh5 = p;
                        p = p.offset(1);
                        *fresh5 = &mut *lineno_cache.offset(i as isize) as *mut alent;
                    }
                    i = i.wrapping_add(1);
                    i;
                }
                if !(p.offset_from(func_table) as libc::c_long as libc::c_uint
                    == nbr_func)
                {
                    bfd_assert(
                        b"./coffcode.h\0" as *const u8 as *const libc::c_char,
                        4496 as libc::c_int,
                    );
                }
                qsort(
                    func_table as *mut libc::c_void,
                    nbr_func as size_t,
                    ::core::mem::size_of::<*mut alent>() as libc::c_ulong,
                    Some(
                        coff_sort_func_alent
                            as unsafe extern "C" fn(
                                *const libc::c_void,
                                *const libc::c_void,
                            ) -> libc::c_int,
                    ),
                );
                amt = (*asect).lineno_count as size_t;
                amt = (amt as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<alent>() as libc::c_ulong)
                    as size_t as size_t;
                if ::core::mem::size_of::<alent>() as libc::c_ulong
                    != 0 as libc::c_int as libc::c_ulong
                    && amt.wrapping_div(::core::mem::size_of::<alent>() as libc::c_ulong)
                        != (*asect).lineno_count as libc::c_ulong
                {
                    bfd_set_error(bfd_error_file_too_big);
                    ret = 0 as libc::c_int != 0;
                } else {
                    n_lineno_cache = bfd_alloc(abfd, amt) as *mut alent;
                    if !n_lineno_cache.is_null() {
                        let mut n_cache_ptr: *mut alent = n_lineno_cache;
                        i = 0 as libc::c_int as libc::c_uint;
                        while i < nbr_func {
                            let mut sym_0: *mut coff_symbol_type = 0
                                as *mut coff_symbol_type;
                            let mut old_ptr: *mut alent = *func_table.offset(i as isize);
                            sym_0 = (*old_ptr).u.sym as *mut coff_symbol_type;
                            (*sym_0)
                                .lineno = lineno_cache
                                .offset(
                                    n_cache_ptr.offset_from(n_lineno_cache) as libc::c_long
                                        as isize,
                                );
                            loop {
                                let fresh6 = old_ptr;
                                old_ptr = old_ptr.offset(1);
                                let fresh7 = n_cache_ptr;
                                n_cache_ptr = n_cache_ptr.offset(1);
                                *fresh7 = *fresh6;
                                if !((*old_ptr).line_number
                                    != 0 as libc::c_int as libc::c_uint)
                                {
                                    break;
                                }
                            }
                            i = i.wrapping_add(1);
                            i;
                        }
                        memcpy(
                            lineno_cache as *mut libc::c_void,
                            n_lineno_cache as *const libc::c_void,
                            ((*asect).lineno_count as libc::c_ulong)
                                .wrapping_mul(
                                    ::core::mem::size_of::<alent>() as libc::c_ulong,
                                ),
                        );
                    } else {
                        ret = 0 as libc::c_int != 0;
                    }
                }
                bfd_release(abfd, func_table as *mut libc::c_void);
            } else {
                ret = 0 as libc::c_int != 0;
            }
        }
    }
    return ret;
}
unsafe extern "C" fn coff_sort_func_alent(
    mut arg1: *const libc::c_void,
    mut arg2: *const libc::c_void,
) -> libc::c_int {
    let mut al1: *const alent = *(arg1 as *mut *const alent);
    let mut al2: *const alent = *(arg2 as *mut *const alent);
    let mut s1: *const coff_symbol_type = (*al1).u.sym as *const coff_symbol_type;
    let mut s2: *const coff_symbol_type = (*al2).u.sym as *const coff_symbol_type;
    if s1.is_null() || s2.is_null() {
        return 0 as libc::c_int;
    }
    if (*s1).symbol.value < (*s2).symbol.value {
        return -(1 as libc::c_int)
    } else if (*s1).symbol.value > (*s2).symbol.value {
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn coff_amd64_reloc(
    mut abfd: *mut bfd,
    mut reloc_entry: *mut arelent,
    mut symbol: *mut asymbol,
    mut data: *mut libc::c_void,
    mut input_section: *mut asection,
    mut output_bfd: *mut bfd,
    mut _error_message: *mut *mut libc::c_char,
) -> bfd_reloc_status_type {
    let mut diff: symvalue = 0;
    if bfd_is_com_section((*symbol).section) {
        diff = (*reloc_entry).addend;
    } else if output_bfd.is_null() {
        if (*symbol).flags & ((1 as libc::c_int) << 7 as libc::c_int) as libc::c_uint
            != 0
        {
            diff = ((*reloc_entry).addend).wrapping_sub((*symbol).value);
        } else {
            diff = ((*reloc_entry).addend).wrapping_neg();
        }
    } else {
        diff = (*reloc_entry).addend;
    }
    if output_bfd.is_null() {
        if (*(*reloc_entry).howto).pc_relative() != 0 {
            diff = (diff as libc::c_ulong)
                .wrapping_sub(bfd_get_reloc_size((*reloc_entry).howto) as libc::c_ulong)
                as symvalue as symvalue;
        }
        if (*(*reloc_entry).howto).type_0 >= 5 as libc::c_int as libc::c_uint
            && (*(*reloc_entry).howto).type_0 <= 9 as libc::c_int as libc::c_uint
        {
            diff = (diff as libc::c_ulong)
                .wrapping_sub(
                    ((*(*reloc_entry).howto).type_0)
                        .wrapping_sub(4 as libc::c_int as libc::c_uint) as libc::c_ulong,
                ) as symvalue as symvalue;
        }
    }
    if (*(*reloc_entry).howto).type_0 == 3 as libc::c_int as libc::c_uint
        && output_bfd.is_null()
    {
        let mut obfd: *mut bfd = (*(*input_section).output_section).owner;
        let mut link_info: *mut bfd_link_info = 0 as *mut bfd_link_info;
        let mut h: *mut bfd_link_hash_entry = 0 as *mut bfd_link_hash_entry;
        match bfd_get_flavour(obfd) as libc::c_uint {
            2 => {
                diff = (diff as libc::c_ulong)
                    .wrapping_sub((*(*obfd).tdata.pe_obj_data).pe_opthdr.ImageBase)
                    as symvalue as symvalue;
            }
            5 => {
                link_info = _bfd_get_link_info(obfd);
                if link_info.is_null() {
                    return bfd_reloc_dangerous;
                }
                h = bfd_link_hash_lookup(
                    (*link_info).hash,
                    b"__ImageBase\0" as *const u8 as *const libc::c_char,
                    0 as libc::c_int != 0,
                    0 as libc::c_int != 0,
                    0 as libc::c_int != 0,
                );
                if h.is_null() {
                    return bfd_reloc_dangerous;
                }
                while (*h).type_0() as libc::c_int
                    == bfd_link_hash_indirect as libc::c_int
                {
                    h = (*h).u.i.link;
                }
                diff = (diff as libc::c_ulong)
                    .wrapping_sub(
                        ((*h).u.def.value)
                            .wrapping_add((*(*h).u.def.section).output_offset)
                            .wrapping_add((*(*(*h).u.def.section).output_section).vma),
                    ) as symvalue as symvalue;
            }
            _ => {}
        }
    }
    if diff != 0 as libc::c_int as libc::c_ulong {
        let mut howto: *const reloc_howto_type = (*reloc_entry).howto;
        let mut octets: bfd_size_type = ((*reloc_entry).address)
            .wrapping_mul(1 as libc::c_int as libc::c_ulong);
        let mut addr: *mut libc::c_uchar = (data as *mut libc::c_uchar)
            .offset(octets as isize);
        if !bfd_reloc_offset_in_range(howto, abfd, input_section, octets) {
            return bfd_reloc_outofrange;
        }
        match (*howto).size() as libc::c_int {
            0 => {
                let mut x: libc::c_char = (*(addr as *const libc::c_uchar) as bfd_vma
                    & 0xff as libc::c_int as libc::c_ulong) as libc::c_char;
                x = (x as libc::c_ulong & !(*howto).dst_mask
                    | (x as libc::c_ulong & (*howto).src_mask).wrapping_add(diff)
                        & (*howto).dst_mask) as libc::c_char;
                *addr = (x as libc::c_int & 0xff as libc::c_int) as libc::c_uchar;
            }
            1 => {
                let mut x_0: libc::c_short = (Some(
                    ((*(*abfd).xvec).bfd_getx16).expect("non-null function pointer"),
                ))
                    .expect("non-null function pointer")(addr as *const libc::c_void)
                    as libc::c_short;
                x_0 = (x_0 as libc::c_ulong & !(*howto).dst_mask
                    | (x_0 as libc::c_ulong & (*howto).src_mask).wrapping_add(diff)
                        & (*howto).dst_mask) as libc::c_short;
                (Some(((*(*abfd).xvec).bfd_putx16).expect("non-null function pointer")))
                    .expect(
                        "non-null function pointer",
                    )(x_0 as bfd_vma, addr as *mut libc::c_void);
            }
            2 => {
                let mut x_1: libc::c_long = (Some(
                    ((*(*abfd).xvec).bfd_getx32).expect("non-null function pointer"),
                ))
                    .expect("non-null function pointer")(addr as *const libc::c_void)
                    as libc::c_long;
                x_1 = (x_1 as libc::c_ulong & !(*howto).dst_mask
                    | (x_1 as libc::c_ulong & (*howto).src_mask).wrapping_add(diff)
                        & (*howto).dst_mask) as libc::c_long;
                (Some(((*(*abfd).xvec).bfd_putx32).expect("non-null function pointer")))
                    .expect(
                        "non-null function pointer",
                    )(x_1 as bfd_vma, addr as *mut libc::c_void);
            }
            4 => {
                let mut x_2: bfd_uint64_t = (Some(
                    ((*(*abfd).xvec).bfd_getx64).expect("non-null function pointer"),
                ))
                    .expect("non-null function pointer")(addr as *const libc::c_void);
                x_2 = x_2 & !(*howto).dst_mask
                    | (x_2 & (*howto).src_mask).wrapping_add(diff) & (*howto).dst_mask;
                (Some(((*(*abfd).xvec).bfd_putx64).expect("non-null function pointer")))
                    .expect("non-null function pointer")(x_2, addr as *mut libc::c_void);
            }
            _ => {
                bfd_set_error(bfd_error_bad_value);
                return bfd_reloc_notsupported;
            }
        }
    }
    return bfd_reloc_continue;
}
unsafe extern "C" fn buy_and_read(
    mut abfd: *mut bfd,
    mut where_0: file_ptr,
    mut nmemb: bfd_size_type,
    mut size: bfd_size_type,
) -> *mut libc::c_void {
    let mut amt: size_t = 0;
    amt = nmemb;
    amt = (amt as libc::c_ulong).wrapping_mul(size) as size_t as size_t;
    if size != 0 as libc::c_int as libc::c_ulong && amt.wrapping_div(size) != nmemb {
        bfd_set_error(bfd_error_file_too_big);
        return 0 as *mut libc::c_void;
    }
    if bfd_seek(abfd, where_0, 0 as libc::c_int) != 0 as libc::c_int {
        return 0 as *mut libc::c_void;
    }
    return _bfd_alloc_and_read(abfd, amt, amt) as *mut libc::c_void;
}
unsafe extern "C" fn coff_slurp_symbol_table(mut abfd: *mut bfd) -> bool {
    let mut native_symbols: *mut combined_entry_type = 0 as *mut combined_entry_type;
    let mut cached_area: *mut coff_symbol_type = 0 as *mut coff_symbol_type;
    let mut table_ptr: *mut libc::c_uint = 0 as *mut libc::c_uint;
    let mut number_of_symbols: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut ret: bool = 1 as libc::c_int != 0;
    let mut amt: size_t = 0;
    if !((*(*abfd).tdata.coff_obj_data).symbols).is_null() {
        return 1 as libc::c_int != 0;
    }
    native_symbols = coff_get_normalized_symtab(abfd);
    if native_symbols.is_null() {
        return 0 as libc::c_int != 0;
    }
    amt = (*(*abfd).tdata.coff_obj_data).raw_syment_count;
    amt = (amt as libc::c_ulong)
        .wrapping_mul(::core::mem::size_of::<coff_symbol_type>() as libc::c_ulong)
        as size_t as size_t;
    if ::core::mem::size_of::<coff_symbol_type>() as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
        && amt.wrapping_div(::core::mem::size_of::<coff_symbol_type>() as libc::c_ulong)
            != (*(*abfd).tdata.coff_obj_data).raw_syment_count
    {
        bfd_set_error(bfd_error_file_too_big);
        return 0 as libc::c_int != 0;
    }
    cached_area = bfd_alloc(abfd, amt) as *mut coff_symbol_type;
    if cached_area.is_null() {
        return 0 as libc::c_int != 0;
    }
    amt = (*(*abfd).tdata.coff_obj_data).raw_syment_count;
    amt = (amt as libc::c_ulong)
        .wrapping_mul(::core::mem::size_of::<libc::c_uint>() as libc::c_ulong) as size_t
        as size_t;
    if ::core::mem::size_of::<libc::c_uint>() as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
        && amt.wrapping_div(::core::mem::size_of::<libc::c_uint>() as libc::c_ulong)
            != (*(*abfd).tdata.coff_obj_data).raw_syment_count
    {
        bfd_set_error(bfd_error_file_too_big);
        return 0 as libc::c_int != 0;
    }
    table_ptr = bfd_zalloc(abfd, amt) as *mut libc::c_uint;
    if table_ptr.is_null() {
        return 0 as libc::c_int != 0
    } else {
        let mut dst: *mut coff_symbol_type = cached_area;
        let mut last_native_index: libc::c_uint = (*(*abfd).tdata.coff_obj_data)
            .raw_syment_count as libc::c_uint;
        let mut this_index: libc::c_uint = 0 as libc::c_int as libc::c_uint;
        while this_index < last_native_index {
            let mut src: *mut combined_entry_type = native_symbols
                .offset(this_index as isize);
            *table_ptr.offset(this_index as isize) = number_of_symbols;
            (*dst).symbol.the_bfd = abfd;
            if !(*src).is_sym {
                bfd_assert(
                    b"./coffcode.h\0" as *const u8 as *const libc::c_char,
                    4594 as libc::c_int,
                );
            }
            (*dst).symbol.name = (*src).u.syment._n._n_n._n_offset as *mut libc::c_char;
            (*src).u.syment._n._n_n._n_zeroes = dst as bfd_hostptr_t;
            (*dst)
                .symbol
                .section = coff_section_from_bfd_index(abfd, (*src).u.syment.n_scnum);
            (*dst).symbol.flags = 0 as libc::c_int as flagword;
            (*dst).symbol.value = 0 as libc::c_int as symvalue;
            (*dst).done_lineno = 0 as libc::c_int != 0;
            let mut current_block_71: u64;
            match (*src).u.syment.n_sclass as libc::c_int {
                2 | 127 | 23 | 104 | 105 => {
                    match coff_classify_symbol(abfd, &mut (*src).u.syment)
                        as libc::c_uint
                    {
                        0 => {
                            (*dst)
                                .symbol
                                .flags = ((1 as libc::c_int) << 1 as libc::c_int
                                | (1 as libc::c_int) << 1 as libc::c_int) as flagword;
                            (*dst).symbol.value = (*src).u.syment.n_value;
                            if (*src).u.syment.n_type as libc::c_ulong
                                & 0x30 as libc::c_int as libc::c_ulong
                                == (2 as libc::c_int as libc::c_ulong) << 4 as libc::c_int
                            {
                                (*dst).symbol.flags
                                    |= ((1 as libc::c_int) << 10 as libc::c_int
                                        | (1 as libc::c_int) << 3 as libc::c_int) as libc::c_uint;
                            }
                        }
                        1 => {
                            (*dst)
                                .symbol
                                .section = &mut *_bfd_std_section
                                .as_mut_ptr()
                                .offset(0 as libc::c_int as isize) as *mut asection;
                            (*dst).symbol.value = (*src).u.syment.n_value;
                        }
                        2 => {
                            (*dst)
                                .symbol
                                .section = &mut *_bfd_std_section
                                .as_mut_ptr()
                                .offset(1 as libc::c_int as isize) as *mut asection;
                            (*dst).symbol.value = 0 as libc::c_int as symvalue;
                        }
                        4 => {
                            (*dst).symbol.flags
                                |= ((1 as libc::c_int) << 1 as libc::c_int
                                    | (1 as libc::c_int) << 8 as libc::c_int) as libc::c_uint;
                            (*dst).symbol.value = 0 as libc::c_int as symvalue;
                        }
                        3 => {
                            (*dst)
                                .symbol
                                .flags = ((1 as libc::c_int) << 0 as libc::c_int)
                                as flagword;
                            (*dst).symbol.value = (*src).u.syment.n_value;
                            if (*src).u.syment.n_type as libc::c_ulong
                                & 0x30 as libc::c_int as libc::c_ulong
                                == (2 as libc::c_int as libc::c_ulong) << 4 as libc::c_int
                            {
                                (*dst).symbol.flags
                                    |= ((1 as libc::c_int) << 10 as libc::c_int
                                        | (1 as libc::c_int) << 3 as libc::c_int) as libc::c_uint;
                            }
                        }
                        _ => {}
                    }
                    if (*src).u.syment.n_sclass as libc::c_int == 105 as libc::c_int {
                        (*dst).symbol.flags
                            |= ((1 as libc::c_int) << 7 as libc::c_int) as libc::c_uint;
                    }
                    if (*src).u.syment.n_sclass as libc::c_int == 104 as libc::c_int
                        && (*src).u.syment.n_scnum > 0 as libc::c_int
                    {
                        (*dst)
                            .symbol
                            .flags = ((1 as libc::c_int) << 0 as libc::c_int)
                            as flagword;
                    }
                    if (*src).u.syment.n_sclass as libc::c_int == 127 as libc::c_int {
                        (*dst).symbol.flags
                            |= ((1 as libc::c_int) << 7 as libc::c_int) as libc::c_uint;
                    }
                    current_block_71 = 10809827304263610514;
                }
                3 | 6 => {
                    if (*src).u.syment.n_scnum == -(2 as libc::c_int) {
                        (*dst)
                            .symbol
                            .flags = ((1 as libc::c_int) << 2 as libc::c_int)
                            as flagword;
                    } else {
                        (*dst)
                            .symbol
                            .flags = ((1 as libc::c_int) << 0 as libc::c_int)
                            as flagword;
                    }
                    if !((*dst).symbol.section).is_null() {
                        (*dst).symbol.value = (*src).u.syment.n_value;
                    } else {
                        (*dst).symbol.value = (*src).u.syment.n_value;
                    }
                    current_block_71 = 10809827304263610514;
                }
                8 | 102 | 17 | 4 | 13 | 9 | 1 | 18 | 15 | 16 | 11 | 12 => {
                    (*dst)
                        .symbol
                        .flags = ((1 as libc::c_int) << 2 as libc::c_int) as flagword;
                    (*dst).symbol.value = (*src).u.syment.n_value;
                    current_block_71 = 10809827304263610514;
                }
                103 | 10 => {
                    (*dst)
                        .symbol
                        .flags = ((1 as libc::c_int) << 2 as libc::c_int) as flagword;
                    (*dst).symbol.value = (*src).u.syment.n_value;
                    current_block_71 = 10809827304263610514;
                }
                100 | 101 | 255 => {
                    (*dst).symbol.value = (*src).u.syment.n_value;
                    if strcmp(
                        (*dst).symbol.name,
                        b".bf\0" as *const u8 as *const libc::c_char,
                    ) != 0 as libc::c_int
                    {
                        (*dst)
                            .symbol
                            .flags = ((1 as libc::c_int) << 2 as libc::c_int)
                            as flagword;
                    } else {
                        (*dst)
                            .symbol
                            .flags = ((1 as libc::c_int) << 2 as libc::c_int
                            | (1 as libc::c_int) << 17 as libc::c_int) as flagword;
                    }
                    current_block_71 = 10809827304263610514;
                }
                20 => {
                    (*dst).symbol.value = (*src).u.syment.n_value;
                    (*dst)
                        .symbol
                        .flags = ((1 as libc::c_int) << 1 as libc::c_int) as flagword;
                    current_block_71 = 10809827304263610514;
                }
                0 => {
                    if (*src).u.syment.n_type as libc::c_int == 0 as libc::c_int
                        && (*src).u.syment.n_value == 0 as libc::c_int as libc::c_ulong
                        && (*src).u.syment.n_scnum == 0 as libc::c_int
                    {
                        current_block_71 = 10809827304263610514;
                    } else {
                        current_block_71 = 6355853790832952044;
                    }
                }
                106 => {
                    current_block_71 = 9041441036995890747;
                }
                5 | 7 | 14 | 21 | _ => {
                    current_block_71 = 6355853790832952044;
                }
            }
            match current_block_71 {
                6355853790832952044 => {
                    _bfd_error_handler(
                        dcgettext(
                            b"bfd\0" as *const u8 as *const libc::c_char,
                            b"%pB: unrecognized storage class %d for %s symbol `%s'\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        abfd,
                        (*src).u.syment.n_sclass as libc::c_int,
                        (*(*dst).symbol.section).name,
                        (*dst).symbol.name,
                    );
                    ret = 0 as libc::c_int != 0;
                    current_block_71 = 9041441036995890747;
                }
                _ => {}
            }
            match current_block_71 {
                9041441036995890747 => {
                    (*dst)
                        .symbol
                        .flags = ((1 as libc::c_int) << 2 as libc::c_int) as flagword;
                    (*dst).symbol.value = (*src).u.syment.n_value;
                }
                _ => {}
            }
            (*dst).native = src;
            (*dst).symbol.udata.i = 0 as libc::c_int as bfd_vma;
            (*dst).lineno = 0 as *mut lineno_cache_entry;
            this_index = this_index
                .wrapping_add(
                    ((*src).u.syment.n_numaux as libc::c_int + 1 as libc::c_int)
                        as libc::c_uint,
                );
            dst = dst.offset(1);
            dst;
            number_of_symbols = number_of_symbols.wrapping_add(1);
            number_of_symbols;
        }
    }
    (*(*abfd).tdata.coff_obj_data).symbols = cached_area;
    (*(*abfd).tdata.coff_obj_data).raw_syments = native_symbols;
    (*abfd).symcount = number_of_symbols;
    (*(*abfd).tdata.coff_obj_data).conversion_table = table_ptr;
    let mut p: *mut asection = 0 as *mut asection;
    p = (*abfd).sections;
    while !p.is_null() {
        if !coff_slurp_line_table(abfd, p) {
            return 0 as libc::c_int != 0;
        }
        p = (*p).next;
    }
    return ret;
}
#[no_mangle]
pub static mut x86_64_pe_big_vec: bfd_target = unsafe {
    {
        let mut init = bfd_target {
            name: b"pe-bigobj-x86-64\0" as *const u8 as *const libc::c_char,
            flavour: bfd_target_coff_flavour,
            byteorder: BFD_ENDIAN_LITTLE,
            header_byteorder: BFD_ENDIAN_LITTLE,
            object_flags: (0x1 as libc::c_int | 0x2 as libc::c_int | 0x4 as libc::c_int
                | 0x8 as libc::c_int | 0x10 as libc::c_int | 0x20 as libc::c_int
                | 0x80 as libc::c_int | 0x100 as libc::c_int | 0x4000 as libc::c_int
                | 0x8000 as libc::c_int) as flagword,
            section_flags: (0x100 as libc::c_int | 0x1 as libc::c_int
                | 0x2 as libc::c_int | 0x4 as libc::c_int | 0x20000 as libc::c_int
                | 0xc0000 as libc::c_int | 0x8 as libc::c_int | 0x2000 as libc::c_int
                | 0x10 as libc::c_int | 0x20 as libc::c_int | 0x8000 as libc::c_int)
                as flagword,
            symbol_leading_char: 0 as libc::c_int as libc::c_char,
            ar_pad_char: '/' as i32 as libc::c_char,
            ar_max_namelen: 15 as libc::c_int as libc::c_uchar,
            match_priority: 0 as libc::c_int as libc::c_uchar,
            keep_unused_section_symbols: 1 as libc::c_int != 0,
            bfd_getx64: Some(
                bfd_getl64 as unsafe extern "C" fn(*const libc::c_void) -> bfd_uint64_t,
            ),
            bfd_getx_signed_64: Some(
                bfd_getl_signed_64
                    as unsafe extern "C" fn(*const libc::c_void) -> bfd_int64_t,
            ),
            bfd_putx64: Some(
                bfd_putl64 as unsafe extern "C" fn(bfd_uint64_t, *mut libc::c_void) -> (),
            ),
            bfd_getx32: Some(
                bfd_getl32 as unsafe extern "C" fn(*const libc::c_void) -> bfd_vma,
            ),
            bfd_getx_signed_32: Some(
                bfd_getl_signed_32
                    as unsafe extern "C" fn(*const libc::c_void) -> bfd_signed_vma,
            ),
            bfd_putx32: Some(
                bfd_putl32 as unsafe extern "C" fn(bfd_vma, *mut libc::c_void) -> (),
            ),
            bfd_getx16: Some(
                bfd_getl16 as unsafe extern "C" fn(*const libc::c_void) -> bfd_vma,
            ),
            bfd_getx_signed_16: Some(
                bfd_getl_signed_16
                    as unsafe extern "C" fn(*const libc::c_void) -> bfd_signed_vma,
            ),
            bfd_putx16: Some(
                bfd_putl16 as unsafe extern "C" fn(bfd_vma, *mut libc::c_void) -> (),
            ),
            bfd_h_getx64: Some(
                bfd_getl64 as unsafe extern "C" fn(*const libc::c_void) -> bfd_uint64_t,
            ),
            bfd_h_getx_signed_64: Some(
                bfd_getl_signed_64
                    as unsafe extern "C" fn(*const libc::c_void) -> bfd_int64_t,
            ),
            bfd_h_putx64: Some(
                bfd_putl64 as unsafe extern "C" fn(bfd_uint64_t, *mut libc::c_void) -> (),
            ),
            bfd_h_getx32: Some(
                bfd_getl32 as unsafe extern "C" fn(*const libc::c_void) -> bfd_vma,
            ),
            bfd_h_getx_signed_32: Some(
                bfd_getl_signed_32
                    as unsafe extern "C" fn(*const libc::c_void) -> bfd_signed_vma,
            ),
            bfd_h_putx32: Some(
                bfd_putl32 as unsafe extern "C" fn(bfd_vma, *mut libc::c_void) -> (),
            ),
            bfd_h_getx16: Some(
                bfd_getl16 as unsafe extern "C" fn(*const libc::c_void) -> bfd_vma,
            ),
            bfd_h_getx_signed_16: Some(
                bfd_getl_signed_16
                    as unsafe extern "C" fn(*const libc::c_void) -> bfd_signed_vma,
            ),
            bfd_h_putx16: Some(
                bfd_putl16 as unsafe extern "C" fn(bfd_vma, *mut libc::c_void) -> (),
            ),
            _bfd_check_format: [
                Some(_bfd_dummy_target as unsafe extern "C" fn(*mut bfd) -> bfd_cleanup),
                Some(coff_object_p as unsafe extern "C" fn(*mut bfd) -> bfd_cleanup),
                Some(
                    bfd_generic_archive_p
                        as unsafe extern "C" fn(*mut bfd) -> bfd_cleanup,
                ),
                Some(coff_object_p as unsafe extern "C" fn(*mut bfd) -> bfd_cleanup),
            ],
            _bfd_set_format: [
                Some(
                    _bfd_bool_bfd_false_error as unsafe extern "C" fn(*mut bfd) -> bool,
                ),
                Some(pe_mkobject as unsafe extern "C" fn(*mut bfd) -> bool),
                Some(_bfd_generic_mkarchive as unsafe extern "C" fn(*mut bfd) -> bool),
                Some(_bfd_bool_bfd_false_error as unsafe extern "C" fn(*mut bfd) -> bool),
            ],
            _bfd_write_contents: [
                Some(
                    _bfd_bool_bfd_false_error as unsafe extern "C" fn(*mut bfd) -> bool,
                ),
                Some(
                    coff_write_object_contents as unsafe extern "C" fn(*mut bfd) -> bool,
                ),
                Some(
                    _bfd_write_archive_contents as unsafe extern "C" fn(*mut bfd) -> bool,
                ),
                Some(_bfd_bool_bfd_false_error as unsafe extern "C" fn(*mut bfd) -> bool),
            ],
            _close_and_cleanup: Some(
                _bfd_coff_close_and_cleanup as unsafe extern "C" fn(*mut bfd) -> bool,
            ),
            _bfd_free_cached_info: Some(
                _bfd_bool_bfd_true as unsafe extern "C" fn(*mut bfd) -> bool,
            ),
            _new_section_hook: Some(
                coff_new_section_hook
                    as unsafe extern "C" fn(*mut bfd, *mut asection) -> bool,
            ),
            _bfd_get_section_contents: Some(
                _bfd_generic_get_section_contents
                    as unsafe extern "C" fn(
                        *mut bfd,
                        *mut asection,
                        *mut libc::c_void,
                        file_ptr,
                        bfd_size_type,
                    ) -> bool,
            ),
            _bfd_get_section_contents_in_window: Some(
                _bfd_generic_get_section_contents_in_window
                    as unsafe extern "C" fn(
                        *mut bfd,
                        *mut asection,
                        *mut bfd_window,
                        file_ptr,
                        bfd_size_type,
                    ) -> bool,
            ),
            _bfd_copy_private_bfd_data: Some(
                pe_bfd_copy_private_bfd_data
                    as unsafe extern "C" fn(*mut bfd, *mut bfd) -> bool,
            ),
            _bfd_merge_private_bfd_data: Some(
                _bfd_bool_bfd_link_true
                    as unsafe extern "C" fn(*mut bfd, *mut bfd_link_info) -> bool,
            ),
            _bfd_init_private_section_data: Some(
                _bfd_generic_init_private_section_data
                    as unsafe extern "C" fn(
                        *mut bfd,
                        *mut asection,
                        *mut bfd,
                        *mut asection,
                        *mut bfd_link_info,
                    ) -> bool,
            ),
            _bfd_copy_private_section_data: Some(
                _bfd_pex64_bfd_copy_private_section_data
                    as unsafe extern "C" fn(
                        *mut bfd,
                        *mut asection,
                        *mut bfd,
                        *mut asection,
                    ) -> bool,
            ),
            _bfd_copy_private_symbol_data: Some(
                _bfd_bool_bfd_asymbol_bfd_asymbol_true
                    as unsafe extern "C" fn(
                        *mut bfd,
                        *mut asymbol,
                        *mut bfd,
                        *mut asymbol,
                    ) -> bool,
            ),
            _bfd_copy_private_header_data: Some(
                _bfd_bool_bfd_bfd_true
                    as unsafe extern "C" fn(*mut bfd, *mut bfd) -> bool,
            ),
            _bfd_set_private_flags: Some(
                _bfd_bool_bfd_uint_true
                    as unsafe extern "C" fn(*mut bfd, libc::c_uint) -> bool,
            ),
            _bfd_print_private_bfd_data: Some(
                pe_print_private_bfd_data
                    as unsafe extern "C" fn(*mut bfd, *mut libc::c_void) -> bool,
            ),
            _core_file_failing_command: Some(
                _bfd_nocore_core_file_failing_command
                    as unsafe extern "C" fn(*mut bfd) -> *mut libc::c_char,
            ),
            _core_file_failing_signal: Some(
                _bfd_nocore_core_file_failing_signal
                    as unsafe extern "C" fn(*mut bfd) -> libc::c_int,
            ),
            _core_file_matches_executable_p: Some(
                _bfd_nocore_core_file_matches_executable_p
                    as unsafe extern "C" fn(*mut bfd, *mut bfd) -> bool,
            ),
            _core_file_pid: Some(
                _bfd_nocore_core_file_pid
                    as unsafe extern "C" fn(*mut bfd) -> libc::c_int,
            ),
            _bfd_slurp_armap: Some(
                bfd_slurp_armap as unsafe extern "C" fn(*mut bfd) -> bool,
            ),
            _bfd_slurp_extended_name_table: Some(
                _bfd_slurp_extended_name_table as unsafe extern "C" fn(*mut bfd) -> bool,
            ),
            _bfd_construct_extended_name_table: Some(
                _bfd_archive_coff_construct_extended_name_table
                    as unsafe extern "C" fn(
                        *mut bfd,
                        *mut *mut libc::c_char,
                        *mut bfd_size_type,
                        *mut *const libc::c_char,
                    ) -> bool,
            ),
            _bfd_truncate_arname: Some(
                bfd_dont_truncate_arname
                    as unsafe extern "C" fn(
                        *mut bfd,
                        *const libc::c_char,
                        *mut libc::c_char,
                    ) -> (),
            ),
            write_armap: Some(
                _bfd_coff_write_armap
                    as unsafe extern "C" fn(
                        *mut bfd,
                        libc::c_uint,
                        *mut orl,
                        libc::c_uint,
                        libc::c_int,
                    ) -> bool,
            ),
            _bfd_read_ar_hdr_fn: Some(
                _bfd_generic_read_ar_hdr
                    as unsafe extern "C" fn(*mut bfd) -> *mut libc::c_void,
            ),
            _bfd_write_ar_hdr_fn: Some(
                _bfd_generic_write_ar_hdr
                    as unsafe extern "C" fn(*mut bfd, *mut bfd) -> bool,
            ),
            openr_next_archived_file: Some(
                bfd_generic_openr_next_archived_file
                    as unsafe extern "C" fn(*mut bfd, *mut bfd) -> *mut bfd,
            ),
            _bfd_get_elt_at_index: Some(
                _bfd_generic_get_elt_at_index
                    as unsafe extern "C" fn(*mut bfd, symindex) -> *mut bfd,
            ),
            _bfd_stat_arch_elt: Some(
                bfd_generic_stat_arch_elt
                    as unsafe extern "C" fn(*mut bfd, *mut stat) -> libc::c_int,
            ),
            _bfd_update_armap_timestamp: Some(
                _bfd_bool_bfd_true as unsafe extern "C" fn(*mut bfd) -> bool,
            ),
            _bfd_get_symtab_upper_bound: Some(
                coff_get_symtab_upper_bound
                    as unsafe extern "C" fn(*mut bfd) -> libc::c_long,
            ),
            _bfd_canonicalize_symtab: Some(
                coff_canonicalize_symtab
                    as unsafe extern "C" fn(*mut bfd, *mut *mut asymbol) -> libc::c_long,
            ),
            _bfd_make_empty_symbol: Some(
                coff_make_empty_symbol as unsafe extern "C" fn(*mut bfd) -> *mut asymbol,
            ),
            _bfd_print_symbol: Some(
                coff_print_symbol
                    as unsafe extern "C" fn(
                        *mut bfd,
                        *mut libc::c_void,
                        *mut asymbol,
                        bfd_print_symbol_type,
                    ) -> (),
            ),
            _bfd_get_symbol_info: Some(
                _bfd_pex64_get_symbol_info
                    as unsafe extern "C" fn(
                        *mut bfd,
                        *mut asymbol,
                        *mut symbol_info,
                    ) -> (),
            ),
            _bfd_get_symbol_version_string: Some(
                _bfd_nosymbols_get_symbol_version_string
                    as unsafe extern "C" fn(
                        *mut bfd,
                        *mut asymbol,
                        bool,
                        *mut bool,
                    ) -> *const libc::c_char,
            ),
            _bfd_is_local_label_name: Some(
                coff_amd64_is_local_label_name
                    as unsafe extern "C" fn(*mut bfd, *const libc::c_char) -> bool,
            ),
            _bfd_is_target_special_symbol: Some(
                _bfd_bool_bfd_asymbol_false
                    as unsafe extern "C" fn(*mut bfd, *mut asymbol) -> bool,
            ),
            _get_lineno: Some(
                coff_get_lineno
                    as unsafe extern "C" fn(*mut bfd, *mut asymbol) -> *mut alent,
            ),
            _bfd_find_nearest_line: Some(
                coff_find_nearest_line
                    as unsafe extern "C" fn(
                        *mut bfd,
                        *mut *mut asymbol,
                        *mut asection,
                        bfd_vma,
                        *mut *const libc::c_char,
                        *mut *const libc::c_char,
                        *mut libc::c_uint,
                        *mut libc::c_uint,
                    ) -> bool,
            ),
            _bfd_find_line: Some(
                _bfd_nosymbols_find_line
                    as unsafe extern "C" fn(
                        *mut bfd,
                        *mut *mut asymbol,
                        *mut asymbol,
                        *mut *const libc::c_char,
                        *mut libc::c_uint,
                    ) -> bool,
            ),
            _bfd_find_inliner_info: Some(
                coff_find_inliner_info
                    as unsafe extern "C" fn(
                        *mut bfd,
                        *mut *const libc::c_char,
                        *mut *const libc::c_char,
                        *mut libc::c_uint,
                    ) -> bool,
            ),
            _bfd_make_debug_symbol: Some(
                coff_bfd_make_debug_symbol
                    as unsafe extern "C" fn(
                        *mut bfd,
                        *mut libc::c_void,
                        libc::c_ulong,
                    ) -> *mut asymbol,
            ),
            _read_minisymbols: Some(
                _bfd_generic_read_minisymbols
                    as unsafe extern "C" fn(
                        *mut bfd,
                        bool,
                        *mut *mut libc::c_void,
                        *mut libc::c_uint,
                    ) -> libc::c_long,
            ),
            _minisymbol_to_symbol: Some(
                _bfd_generic_minisymbol_to_symbol
                    as unsafe extern "C" fn(
                        *mut bfd,
                        bool,
                        *const libc::c_void,
                        *mut asymbol,
                    ) -> *mut asymbol,
            ),
            _get_reloc_upper_bound: Some(
                coff_get_reloc_upper_bound
                    as unsafe extern "C" fn(*mut bfd, sec_ptr) -> libc::c_long,
            ),
            _bfd_canonicalize_reloc: Some(
                coff_canonicalize_reloc
                    as unsafe extern "C" fn(
                        *mut bfd,
                        *mut asection,
                        *mut *mut arelent,
                        *mut *mut asymbol,
                    ) -> libc::c_long,
            ),
            _bfd_set_reloc: Some(
                _bfd_generic_set_reloc
                    as unsafe extern "C" fn(
                        *mut bfd,
                        sec_ptr,
                        *mut *mut arelent,
                        libc::c_uint,
                    ) -> (),
            ),
            reloc_type_lookup: Some(
                coff_amd64_reloc_type_lookup
                    as unsafe extern "C" fn(
                        *mut bfd,
                        bfd_reloc_code_real_type,
                    ) -> *const reloc_howto_type,
            ),
            reloc_name_lookup: Some(
                coff_amd64_reloc_name_lookup
                    as unsafe extern "C" fn(
                        *mut bfd,
                        *const libc::c_char,
                    ) -> *const reloc_howto_type,
            ),
            _bfd_set_arch_mach: Some(
                coff_set_arch_mach
                    as unsafe extern "C" fn(
                        *mut bfd,
                        bfd_architecture,
                        libc::c_ulong,
                    ) -> bool,
            ),
            _bfd_set_section_contents: Some(
                coff_set_section_contents
                    as unsafe extern "C" fn(
                        *mut bfd,
                        *mut asection,
                        *const libc::c_void,
                        file_ptr,
                        bfd_size_type,
                    ) -> bool,
            ),
            _bfd_sizeof_headers: Some(
                coff_sizeof_headers
                    as unsafe extern "C" fn(*mut bfd, *mut bfd_link_info) -> libc::c_int,
            ),
            _bfd_get_relocated_section_contents: Some(
                bfd_generic_get_relocated_section_contents
                    as unsafe extern "C" fn(
                        *mut bfd,
                        *mut bfd_link_info,
                        *mut bfd_link_order,
                        *mut bfd_byte,
                        bool,
                        *mut *mut asymbol,
                    ) -> *mut bfd_byte,
            ),
            _bfd_relax_section: Some(
                bfd_generic_relax_section
                    as unsafe extern "C" fn(
                        *mut bfd,
                        *mut asection,
                        *mut bfd_link_info,
                        *mut bool,
                    ) -> bool,
            ),
            _bfd_link_hash_table_create: Some(
                _bfd_coff_link_hash_table_create
                    as unsafe extern "C" fn(*mut bfd) -> *mut bfd_link_hash_table,
            ),
            _bfd_link_add_symbols: Some(
                pex64_link_add_symbols
                    as unsafe extern "C" fn(*mut bfd, *mut bfd_link_info) -> bool,
            ),
            _bfd_link_just_syms: Some(
                _bfd_generic_link_just_syms
                    as unsafe extern "C" fn(*mut asection, *mut bfd_link_info) -> (),
            ),
            _bfd_copy_link_hash_symbol_type: Some(
                _bfd_generic_copy_link_hash_symbol_type
                    as unsafe extern "C" fn(
                        *mut bfd,
                        *mut bfd_link_hash_entry,
                        *mut bfd_link_hash_entry,
                    ) -> (),
            ),
            _bfd_final_link: Some(
                _bfd_coff_final_link
                    as unsafe extern "C" fn(*mut bfd, *mut bfd_link_info) -> bool,
            ),
            _bfd_link_split_section: Some(
                _bfd_generic_link_split_section
                    as unsafe extern "C" fn(*mut bfd, *mut bfd_section) -> bool,
            ),
            _bfd_link_check_relocs: Some(
                _bfd_generic_link_check_relocs
                    as unsafe extern "C" fn(*mut bfd, *mut bfd_link_info) -> bool,
            ),
            _bfd_gc_sections: Some(
                bfd_coff_gc_sections
                    as unsafe extern "C" fn(*mut bfd, *mut bfd_link_info) -> bool,
            ),
            _bfd_lookup_section_flags: Some(
                bfd_generic_lookup_section_flags
                    as unsafe extern "C" fn(
                        *mut bfd_link_info,
                        *mut flag_info,
                        *mut asection,
                    ) -> bool,
            ),
            _bfd_merge_sections: Some(
                bfd_generic_merge_sections
                    as unsafe extern "C" fn(*mut bfd, *mut bfd_link_info) -> bool,
            ),
            _bfd_is_group_section: Some(
                bfd_generic_is_group_section
                    as unsafe extern "C" fn(*mut bfd, *const asection) -> bool,
            ),
            _bfd_group_name: Some(
                bfd_coff_group_name
                    as unsafe extern "C" fn(
                        *mut bfd,
                        *const asection,
                    ) -> *const libc::c_char,
            ),
            _bfd_discard_group: Some(
                bfd_generic_discard_group
                    as unsafe extern "C" fn(*mut bfd, *mut asection) -> bool,
            ),
            _section_already_linked: Some(
                _bfd_coff_section_already_linked
                    as unsafe extern "C" fn(
                        *mut bfd,
                        *mut asection,
                        *mut bfd_link_info,
                    ) -> bool,
            ),
            _bfd_define_common_symbol: Some(
                bfd_generic_define_common_symbol
                    as unsafe extern "C" fn(
                        *mut bfd,
                        *mut bfd_link_info,
                        *mut bfd_link_hash_entry,
                    ) -> bool,
            ),
            _bfd_link_hide_symbol: Some(
                _bfd_generic_link_hide_symbol
                    as unsafe extern "C" fn(
                        *mut bfd,
                        *mut bfd_link_info,
                        *mut bfd_link_hash_entry,
                    ) -> (),
            ),
            _bfd_define_start_stop: Some(
                bfd_generic_define_start_stop
                    as unsafe extern "C" fn(
                        *mut bfd_link_info,
                        *const libc::c_char,
                        *mut asection,
                    ) -> *mut bfd_link_hash_entry,
            ),
            _bfd_get_dynamic_symtab_upper_bound: Some(
                _bfd_long_bfd_n1_error as unsafe extern "C" fn(*mut bfd) -> libc::c_long,
            ),
            _bfd_canonicalize_dynamic_symtab: Some(
                _bfd_nosymbols_canonicalize_symtab
                    as unsafe extern "C" fn(*mut bfd, *mut *mut asymbol) -> libc::c_long,
            ),
            _bfd_get_synthetic_symtab: Some(
                _bfd_nodynamic_get_synthetic_symtab
                    as unsafe extern "C" fn(
                        *mut bfd,
                        libc::c_long,
                        *mut *mut asymbol,
                        libc::c_long,
                        *mut *mut asymbol,
                        *mut *mut asymbol,
                    ) -> libc::c_long,
            ),
            _bfd_get_dynamic_reloc_upper_bound: Some(
                _bfd_long_bfd_n1_error as unsafe extern "C" fn(*mut bfd) -> libc::c_long,
            ),
            _bfd_canonicalize_dynamic_reloc: Some(
                _bfd_nodynamic_canonicalize_dynamic_reloc
                    as unsafe extern "C" fn(
                        *mut bfd,
                        *mut *mut arelent,
                        *mut *mut asymbol,
                    ) -> libc::c_long,
            ),
            alternative_target: 0 as *const bfd_target,
            backend_data: &bigobj_swap_table as *const bfd_coff_backend_data
                as *mut bfd_coff_backend_data as *const libc::c_void,
        };
        init
    }
};
unsafe extern "C" fn coff_classify_symbol(
    mut abfd: *mut bfd,
    mut syment: *mut internal_syment,
) -> coff_symbol_classification {
    match (*syment).n_sclass as libc::c_int {
        2 | 127 | 23 | 105 => {
            if (*syment).n_scnum == 0 as libc::c_int {
                if (*syment).n_value == 0 as libc::c_int as libc::c_ulong {
                    return COFF_SYMBOL_UNDEFINED
                } else {
                    return COFF_SYMBOL_COMMON
                }
            }
            return COFF_SYMBOL_GLOBAL;
        }
        _ => {}
    }
    if (*syment).n_sclass as libc::c_int == 3 as libc::c_int {
        if (*syment).n_scnum == 0 as libc::c_int {
            return COFF_SYMBOL_LOCAL;
        }
        return COFF_SYMBOL_LOCAL;
    }
    if (*syment).n_sclass as libc::c_int == 104 as libc::c_int {
        (*syment).n_value = 0 as libc::c_int as bfd_vma;
        if (*syment).n_scnum == 0 as libc::c_int {
            return COFF_SYMBOL_UNDEFINED;
        }
        return COFF_SYMBOL_PE_SECTION;
    }
    if (*syment).n_scnum == 0 as libc::c_int {
        let mut buf: [libc::c_char; 9] = [0; 9];
        _bfd_error_handler(
            dcgettext(
                b"bfd\0" as *const u8 as *const libc::c_char,
                b"warning: %pB: local symbol `%s' has no section\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            abfd,
            _bfd_coff_internal_syment_name(abfd, syment, buf.as_mut_ptr()),
        );
    }
    return COFF_SYMBOL_LOCAL;
}
unsafe extern "C" fn coff_slurp_reloc_table(
    mut abfd: *mut bfd,
    mut asect: sec_ptr,
    mut symbols: *mut *mut asymbol,
) -> bool {
    let mut native_relocs: *mut external_reloc = 0 as *mut external_reloc;
    let mut reloc_cache: *mut arelent = 0 as *mut arelent;
    let mut cache_ptr: *mut arelent = 0 as *mut arelent;
    let mut idx: libc::c_uint = 0;
    let mut amt: size_t = 0;
    if !((*asect).relocation).is_null() {
        return 1 as libc::c_int != 0;
    }
    if (*asect).reloc_count == 0 as libc::c_int as libc::c_uint {
        return 1 as libc::c_int != 0;
    }
    if (*asect).flags & 0x80 as libc::c_int as libc::c_uint != 0 {
        return 1 as libc::c_int != 0;
    }
    if !coff_slurp_symbol_table(abfd) {
        return 0 as libc::c_int != 0;
    }
    native_relocs = buy_and_read(
        abfd,
        (*asect).rel_filepos,
        (*asect).reloc_count as bfd_size_type,
        (*((*(*abfd).xvec).backend_data as *mut bfd_coff_backend_data))._bfd_relsz
            as bfd_size_type,
    ) as *mut external_reloc;
    amt = (*asect).reloc_count as size_t;
    amt = (amt as libc::c_ulong)
        .wrapping_mul(::core::mem::size_of::<arelent>() as libc::c_ulong) as size_t
        as size_t;
    if ::core::mem::size_of::<arelent>() as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
        && amt.wrapping_div(::core::mem::size_of::<arelent>() as libc::c_ulong)
            != (*asect).reloc_count as libc::c_ulong
    {
        bfd_set_error(bfd_error_file_too_big);
        return 0 as libc::c_int != 0;
    }
    reloc_cache = bfd_alloc(abfd, amt) as *mut arelent;
    if reloc_cache.is_null() || native_relocs.is_null() {
        return 0 as libc::c_int != 0;
    }
    idx = 0 as libc::c_int as libc::c_uint;
    while idx < (*asect).reloc_count {
        let mut dst: internal_reloc = internal_reloc {
            r_vaddr: 0,
            r_symndx: 0,
            r_type: 0,
            r_size: 0,
            r_extern: 0,
            r_offset: 0,
        };
        let mut src: *mut external_reloc = 0 as *mut external_reloc;
        let mut ptr: *mut asymbol = 0 as *mut asymbol;
        cache_ptr = reloc_cache.offset(idx as isize);
        src = native_relocs.offset(idx as isize);
        dst.r_offset = 0 as libc::c_int as libc::c_ulong;
        coff_swap_reloc_in(
            abfd,
            src as *mut libc::c_void,
            &mut dst as *mut internal_reloc as *mut libc::c_void,
        );
        (*cache_ptr).address = dst.r_vaddr;
        if dst.r_symndx != -(1 as libc::c_int) as libc::c_long && !symbols.is_null() {
            if dst.r_symndx < 0 as libc::c_int as libc::c_long
                || dst.r_symndx
                    >= (*(*abfd).tdata.coff_obj_data).conv_table_size as libc::c_long
            {
                _bfd_error_handler(
                    dcgettext(
                        b"bfd\0" as *const u8 as *const libc::c_char,
                        b"%pB: warning: illegal symbol index %ld in relocs\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    abfd,
                    dst.r_symndx,
                );
                (*cache_ptr)
                    .sym_ptr_ptr = _bfd_std_section[2 as libc::c_int as usize]
                    .symbol_ptr_ptr;
                ptr = 0 as *mut asymbol;
            } else {
                (*cache_ptr)
                    .sym_ptr_ptr = symbols
                    .offset(
                        *((*(*abfd).tdata.coff_obj_data).conversion_table)
                            .offset(dst.r_symndx as isize) as isize,
                    );
                ptr = *(*cache_ptr).sym_ptr_ptr;
            }
        } else {
            (*cache_ptr)
                .sym_ptr_ptr = _bfd_std_section[2 as libc::c_int as usize]
                .symbol_ptr_ptr;
            ptr = 0 as *mut asymbol;
        }
        let mut coffsym: *mut coff_symbol_type = 0 as *mut coff_symbol_type;
        if !ptr.is_null() && bfd_asymbol_bfd(ptr) != abfd {
            coffsym = ((*(*abfd).tdata.coff_obj_data).symbols)
                .offset(
                    ((*cache_ptr).sym_ptr_ptr).offset_from(symbols) as libc::c_long
                        as isize,
                );
        } else if !ptr.is_null() {
            coffsym = if bfd_family_coff(bfd_asymbol_bfd(ptr)) as libc::c_int != 0
                && !((*bfd_asymbol_bfd(ptr)).tdata.coff_obj_data).is_null()
            {
                ptr as *mut coff_symbol_type
            } else {
                0 as *mut coff_symbol_type
            };
        }
        if !coffsym.is_null()
            && (*(*coffsym).native).u.syment.n_scnum == 0 as libc::c_int
        {
            (*cache_ptr).addend = ((*(*coffsym).native).u.syment.n_value).wrapping_neg();
        } else if !ptr.is_null() && bfd_asymbol_bfd(ptr) == abfd
            && !((*ptr).section).is_null()
        {
            (*cache_ptr)
                .addend = ((*(*ptr).section).vma)
                .wrapping_add((*ptr).value)
                .wrapping_neg();
        } else {
            (*cache_ptr).addend = 0 as libc::c_int as bfd_vma;
        }
        if !ptr.is_null()
            && (dst.r_type as libc::c_ulong)
                < (::core::mem::size_of::<[reloc_howto_type; 21]>() as libc::c_ulong)
                    .wrapping_div(
                        ::core::mem::size_of::<reloc_howto_type>() as libc::c_ulong,
                    )
            && (howto_table[dst.r_type as usize]).pc_relative() as libc::c_int != 0
        {
            (*cache_ptr)
                .addend = ((*cache_ptr).addend as libc::c_ulong)
                .wrapping_add((*asect).vma) as bfd_vma as bfd_vma;
        }
        (*cache_ptr)
            .address = ((*cache_ptr).address as libc::c_ulong).wrapping_sub((*asect).vma)
            as bfd_size_type as bfd_size_type;
        (*cache_ptr)
            .howto = if (dst.r_type as libc::c_ulong)
            < (::core::mem::size_of::<[reloc_howto_type; 21]>() as libc::c_ulong)
                .wrapping_div(
                    ::core::mem::size_of::<reloc_howto_type>() as libc::c_ulong,
                )
        {
            howto_table.as_ptr().offset(dst.r_type as libc::c_int as isize)
        } else {
            0 as *const reloc_howto_type
        };
        if ((*cache_ptr).howto).is_null() {
            _bfd_error_handler(
                dcgettext(
                    b"bfd\0" as *const u8 as *const libc::c_char,
                    b"%pB: illegal relocation type %d at address %#lx\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                abfd,
                dst.r_type as libc::c_int,
                dst.r_vaddr,
            );
            bfd_set_error(bfd_error_bad_value);
            return 0 as libc::c_int != 0;
        }
        idx = idx.wrapping_add(1);
        idx;
    }
    (*asect).relocation = reloc_cache;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn coff_canonicalize_reloc(
    mut abfd: *mut bfd,
    mut section: sec_ptr,
    mut relptr: *mut *mut arelent,
    mut symbols: *mut *mut asymbol,
) -> libc::c_long {
    let mut tblptr: *mut arelent = (*section).relocation;
    let mut count: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    if (*section).flags & 0x80 as libc::c_int as libc::c_uint != 0 {
        let mut chain: *mut arelent_chain = (*section).constructor_chain;
        count = 0 as libc::c_int as libc::c_uint;
        while count < (*section).reloc_count {
            let fresh8 = relptr;
            relptr = relptr.offset(1);
            *fresh8 = &mut (*chain).relent;
            chain = (*chain).next;
            count = count.wrapping_add(1);
            count;
        }
    } else {
        if !coff_slurp_reloc_table(abfd, section, symbols) {
            return -(1 as libc::c_int) as libc::c_long;
        }
        tblptr = (*section).relocation;
        loop {
            let fresh9 = count;
            count = count.wrapping_add(1);
            if !(fresh9 < (*section).reloc_count) {
                break;
            }
            let fresh10 = tblptr;
            tblptr = tblptr.offset(1);
            let fresh11 = relptr;
            relptr = relptr.offset(1);
            *fresh11 = fresh10;
        }
    }
    *relptr = 0 as *mut arelent;
    return (*section).reloc_count as libc::c_long;
}
unsafe extern "C" fn coff_set_alignment_hook(
    mut abfd: *mut bfd,
    mut section: *mut asection,
    mut scnhdr: *mut libc::c_void,
) {
    let mut hdr: *mut internal_scnhdr = scnhdr as *mut internal_scnhdr;
    let mut amt: size_t = 0;
    let mut alignment_power_const: libc::c_uint = ((*hdr).s_flags
        & 0xf00000 as libc::c_int as libc::c_ulong) as libc::c_uint;
    match alignment_power_const {
        14680064 | 13631488 | 12582912 | 11534336 | 10485760 | 9437184 | 8388608
        | 7340032 | 6291456 | 5242880 | 4194304 | 3145728 | 2097152 | 1048576 => {
            (*section)
                .alignment_power = (alignment_power_const >> 20 as libc::c_int)
                .wrapping_sub(1 as libc::c_int as libc::c_uint);
        }
        _ => {}
    }
    if ((*section).used_by_bfd as *mut coff_section_tdata).is_null() {
        amt = ::core::mem::size_of::<coff_section_tdata>() as libc::c_ulong;
        (*section).used_by_bfd = bfd_zalloc(abfd, amt);
        if ((*section).used_by_bfd).is_null() {
            _bfd_abort(
                b"./coffcode.h\0" as *const u8 as *const libc::c_char,
                1922 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 56],
                    &[libc::c_char; 56],
                >(b"void coff_set_alignment_hook(bfd *, asection *, void *)\0"))
                    .as_ptr(),
            );
        }
    }
    if ((*((*section).used_by_bfd as *mut coff_section_tdata)).tdata
        as *mut pei_section_tdata)
        .is_null()
    {
        amt = ::core::mem::size_of::<pei_section_tdata>() as libc::c_ulong;
        let ref mut fresh12 = (*((*section).used_by_bfd as *mut coff_section_tdata))
            .tdata;
        *fresh12 = bfd_zalloc(abfd, amt);
        if ((*((*section).used_by_bfd as *mut coff_section_tdata)).tdata).is_null() {
            _bfd_abort(
                b"./coffcode.h\0" as *const u8 as *const libc::c_char,
                1931 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 56],
                    &[libc::c_char; 56],
                >(b"void coff_set_alignment_hook(bfd *, asection *, void *)\0"))
                    .as_ptr(),
            );
        }
    }
    (*((*((*section).used_by_bfd as *mut coff_section_tdata)).tdata
        as *mut pei_section_tdata))
        .virt_size = (*hdr).s_paddr;
    (*((*((*section).used_by_bfd as *mut coff_section_tdata)).tdata
        as *mut pei_section_tdata))
        .pe_flags = (*hdr).s_flags as libc::c_long;
    (*section).lma = (*hdr).s_vaddr;
    if (*hdr).s_flags & 0x1000000 as libc::c_int as libc::c_ulong != 0 {
        let mut dst: external_reloc = external_reloc {
            r_vaddr: [0; 4],
            r_symndx: [0; 4],
            r_type: [0; 2],
        };
        let mut n: internal_reloc = internal_reloc {
            r_vaddr: 0,
            r_symndx: 0,
            r_type: 0,
            r_size: 0,
            r_extern: 0,
            r_offset: 0,
        };
        let mut oldpos: file_ptr = bfd_tell(abfd);
        let mut relsz: bfd_size_type = (*((*(*abfd).xvec).backend_data
            as *mut bfd_coff_backend_data))
            ._bfd_relsz as bfd_size_type;
        if bfd_seek(abfd, (*hdr).s_relptr as file_ptr, 0 as libc::c_int)
            != 0 as libc::c_int
        {
            return;
        }
        if bfd_bread(&mut dst as *mut external_reloc as *mut libc::c_void, relsz, abfd)
            != relsz
        {
            return;
        }
        coff_swap_reloc_in(
            abfd,
            &mut dst as *mut external_reloc as *mut libc::c_void,
            &mut n as *mut internal_reloc as *mut libc::c_void,
        );
        if bfd_seek(abfd, oldpos, 0 as libc::c_int) != 0 as libc::c_int {
            return;
        }
        (*hdr).s_nreloc = (n.r_vaddr).wrapping_sub(1 as libc::c_int as libc::c_ulong);
        (*section).reloc_count = (*hdr).s_nreloc as libc::c_uint;
        (*section)
            .rel_filepos = ((*section).rel_filepos as libc::c_ulong).wrapping_add(relsz)
            as file_ptr as file_ptr;
    } else if (*hdr).s_nreloc == 0xffff as libc::c_int as libc::c_ulong {
        _bfd_error_handler(
            dcgettext(
                b"bfd\0" as *const u8 as *const libc::c_char,
                b"%pB: warning: claims to have 0xffff relocs, without overflow\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            abfd,
        );
    }
}
unsafe extern "C" fn symname_in_debug_hook(
    mut _abfd: *mut bfd,
    mut _sym: *mut internal_syment,
) -> bool {
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn coff_print_aux(
    mut _abfd: *mut bfd,
    mut _file: *mut FILE,
    mut _table_base: *mut combined_entry_type,
    mut symbol: *mut combined_entry_type,
    mut aux: *mut combined_entry_type,
    mut _indaux: libc::c_uint,
) -> bool {
    if !(*symbol).is_sym {
        bfd_assert(
            b"./coffcode.h\0" as *const u8 as *const libc::c_char,
            2471 as libc::c_int,
        );
    }
    if (*aux).is_sym {
        bfd_assert(
            b"./coffcode.h\0" as *const u8 as *const libc::c_char,
            2472 as libc::c_int,
        );
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn dummy_reloc16_estimate(
    mut _abfd: *mut bfd,
    mut _input_section: *mut asection,
    mut _reloc: *mut arelent,
    mut _shrink: libc::c_uint,
    mut _link_info: *mut bfd_link_info,
) -> libc::c_int {
    _bfd_abort(
        b"./coffcode.h\0" as *const u8 as *const libc::c_char,
        5258 as libc::c_int,
        (*::core::mem::transmute::<
            &[u8; 95],
            &[libc::c_char; 95],
        >(
            b"int dummy_reloc16_estimate(bfd *, asection *, arelent *, unsigned int, struct bfd_link_info *)\0",
        ))
            .as_ptr(),
    );
}
unsafe extern "C" fn dummy_reloc16_extra_cases(
    mut _abfd: *mut bfd,
    mut _link_info: *mut bfd_link_info,
    mut _link_order: *mut bfd_link_order,
    mut _reloc: *mut arelent,
    mut _data: *mut bfd_byte,
    mut _src_ptr: *mut libc::c_uint,
    mut _dst_ptr: *mut libc::c_uint,
) {
    _bfd_abort(
        b"./coffcode.h\0" as *const u8 as *const libc::c_char,
        5279 as libc::c_int,
        (*::core::mem::transmute::<
            &[u8; 142],
            &[libc::c_char; 142],
        >(
            b"void dummy_reloc16_extra_cases(bfd *, struct bfd_link_info *, struct bfd_link_order *, arelent *, bfd_byte *, unsigned int *, unsigned int *)\0",
        ))
            .as_ptr(),
    );
}
unsafe extern "C" fn coff_link_output_has_begun(
    mut abfd: *mut bfd,
    mut _info: *mut coff_final_link_info,
) -> bool {
    return (*abfd).output_has_begun() != 0;
}
static mut bfd_coff_std_swap_table: bfd_coff_backend_data = unsafe {
    {
        let mut init = bfd_coff_backend_data {
            _bfd_coff_swap_aux_in: Some(
                _bfd_pex64i_swap_aux_in
                    as unsafe extern "C" fn(
                        *mut bfd,
                        *mut libc::c_void,
                        libc::c_int,
                        libc::c_int,
                        libc::c_int,
                        libc::c_int,
                        *mut libc::c_void,
                    ) -> (),
            ),
            _bfd_coff_swap_sym_in: Some(
                _bfd_pex64i_swap_sym_in
                    as unsafe extern "C" fn(
                        *mut bfd,
                        *mut libc::c_void,
                        *mut libc::c_void,
                    ) -> (),
            ),
            _bfd_coff_swap_lineno_in: Some(
                _bfd_pex64i_swap_lineno_in
                    as unsafe extern "C" fn(
                        *mut bfd,
                        *mut libc::c_void,
                        *mut libc::c_void,
                    ) -> (),
            ),
            _bfd_coff_swap_aux_out: Some(
                _bfd_pex64i_swap_aux_out
                    as unsafe extern "C" fn(
                        *mut bfd,
                        *mut libc::c_void,
                        libc::c_int,
                        libc::c_int,
                        libc::c_int,
                        libc::c_int,
                        *mut libc::c_void,
                    ) -> libc::c_uint,
            ),
            _bfd_coff_swap_sym_out: Some(
                _bfd_pex64i_swap_sym_out
                    as unsafe extern "C" fn(
                        *mut bfd,
                        *mut libc::c_void,
                        *mut libc::c_void,
                    ) -> libc::c_uint,
            ),
            _bfd_coff_swap_lineno_out: Some(
                _bfd_pex64i_swap_lineno_out
                    as unsafe extern "C" fn(
                        *mut bfd,
                        *mut libc::c_void,
                        *mut libc::c_void,
                    ) -> libc::c_uint,
            ),
            _bfd_coff_swap_reloc_out: Some(
                coff_swap_reloc_out
                    as unsafe extern "C" fn(
                        *mut bfd,
                        *mut libc::c_void,
                        *mut libc::c_void,
                    ) -> libc::c_uint,
            ),
            _bfd_coff_swap_filehdr_out: Some(
                _bfd_pex64_only_swap_filehdr_out
                    as unsafe extern "C" fn(
                        *mut bfd,
                        *mut libc::c_void,
                        *mut libc::c_void,
                    ) -> libc::c_uint,
            ),
            _bfd_coff_swap_aouthdr_out: Some(
                _bfd_pex64i_swap_aouthdr_out
                    as unsafe extern "C" fn(
                        *mut bfd,
                        *mut libc::c_void,
                        *mut libc::c_void,
                    ) -> libc::c_uint,
            ),
            _bfd_coff_swap_scnhdr_out: Some(
                _bfd_pex64i_swap_scnhdr_out
                    as unsafe extern "C" fn(
                        *mut bfd,
                        *mut libc::c_void,
                        *mut libc::c_void,
                    ) -> libc::c_uint,
            ),
            _bfd_filhsz: 20 as libc::c_int as libc::c_uint,
            _bfd_aoutsz: (24 as libc::c_int + 196 as libc::c_int
                + 5 as libc::c_int * 4 as libc::c_int) as libc::c_uint,
            _bfd_scnhsz: 40 as libc::c_int as libc::c_uint,
            _bfd_symesz: 18 as libc::c_int as libc::c_uint,
            _bfd_auxesz: 18 as libc::c_int as libc::c_uint,
            _bfd_relsz: 10 as libc::c_int as libc::c_uint,
            _bfd_linesz: (4 as libc::c_int + 2 as libc::c_int) as libc::c_uint,
            _bfd_filnmlen: 18 as libc::c_int as libc::c_uint,
            _bfd_coff_long_filenames: 1 as libc::c_int != 0,
            _bfd_coff_long_section_names: 1 as libc::c_int != 0,
            _bfd_coff_set_long_section_names: Some(
                bfd_coff_set_long_section_names_allowed
                    as unsafe extern "C" fn(*mut bfd, libc::c_int) -> bool,
            ),
            _bfd_coff_default_section_alignment_power: 2 as libc::c_int as libc::c_uint,
            _bfd_coff_force_symnames_in_strings: 0 as libc::c_int != 0,
            _bfd_coff_debug_string_prefix_length: 2 as libc::c_int as libc::c_uint,
            _bfd_coff_max_nscns: 32768 as libc::c_int as libc::c_uint,
            _bfd_coff_swap_filehdr_in: Some(
                coff_swap_filehdr_in
                    as unsafe extern "C" fn(
                        *mut bfd,
                        *mut libc::c_void,
                        *mut libc::c_void,
                    ) -> (),
            ),
            _bfd_coff_swap_aouthdr_in: Some(
                _bfd_pex64i_swap_aouthdr_in
                    as unsafe extern "C" fn(
                        *mut bfd,
                        *mut libc::c_void,
                        *mut libc::c_void,
                    ) -> (),
            ),
            _bfd_coff_swap_scnhdr_in: Some(
                coff_swap_scnhdr_in
                    as unsafe extern "C" fn(
                        *mut bfd,
                        *mut libc::c_void,
                        *mut libc::c_void,
                    ) -> (),
            ),
            _bfd_coff_swap_reloc_in: Some(
                coff_swap_reloc_in
                    as unsafe extern "C" fn(
                        *mut bfd,
                        *mut libc::c_void,
                        *mut libc::c_void,
                    ) -> (),
            ),
            _bfd_coff_bad_format_hook: Some(
                coff_bad_format_hook
                    as unsafe extern "C" fn(*mut bfd, *mut libc::c_void) -> bool,
            ),
            _bfd_coff_set_arch_mach_hook: Some(
                coff_set_arch_mach_hook
                    as unsafe extern "C" fn(*mut bfd, *mut libc::c_void) -> bool,
            ),
            _bfd_coff_mkobject_hook: Some(
                pe_mkobject_hook
                    as unsafe extern "C" fn(
                        *mut bfd,
                        *mut libc::c_void,
                        *mut libc::c_void,
                    ) -> *mut libc::c_void,
            ),
            _bfd_styp_to_sec_flags_hook: Some(
                styp_to_sec_flags
                    as unsafe extern "C" fn(
                        *mut bfd,
                        *mut libc::c_void,
                        *const libc::c_char,
                        *mut asection,
                        *mut flagword,
                    ) -> bool,
            ),
            _bfd_set_alignment_hook: Some(
                coff_set_alignment_hook
                    as unsafe extern "C" fn(
                        *mut bfd,
                        *mut asection,
                        *mut libc::c_void,
                    ) -> (),
            ),
            _bfd_coff_slurp_symbol_table: Some(
                coff_slurp_symbol_table as unsafe extern "C" fn(*mut bfd) -> bool,
            ),
            _bfd_coff_symname_in_debug: Some(
                symname_in_debug_hook
                    as unsafe extern "C" fn(*mut bfd, *mut internal_syment) -> bool,
            ),
            _bfd_coff_pointerize_aux_hook: None,
            _bfd_coff_print_aux: Some(
                coff_print_aux
                    as unsafe extern "C" fn(
                        *mut bfd,
                        *mut FILE,
                        *mut combined_entry_type,
                        *mut combined_entry_type,
                        *mut combined_entry_type,
                        libc::c_uint,
                    ) -> bool,
            ),
            _bfd_coff_reloc16_extra_cases: Some(
                dummy_reloc16_extra_cases
                    as unsafe extern "C" fn(
                        *mut bfd,
                        *mut bfd_link_info,
                        *mut bfd_link_order,
                        *mut arelent,
                        *mut bfd_byte,
                        *mut libc::c_uint,
                        *mut libc::c_uint,
                    ) -> (),
            ),
            _bfd_coff_reloc16_estimate: Some(
                dummy_reloc16_estimate
                    as unsafe extern "C" fn(
                        *mut bfd,
                        *mut asection,
                        *mut arelent,
                        libc::c_uint,
                        *mut bfd_link_info,
                    ) -> libc::c_int,
            ),
            _bfd_coff_classify_symbol: Some(
                coff_classify_symbol
                    as unsafe extern "C" fn(
                        *mut bfd,
                        *mut internal_syment,
                    ) -> coff_symbol_classification,
            ),
            _bfd_coff_compute_section_file_positions: Some(
                coff_compute_section_file_positions
                    as unsafe extern "C" fn(*mut bfd) -> bool,
            ),
            _bfd_coff_start_final_link: None,
            _bfd_coff_relocate_section: Some(
                coff_pe_amd64_relocate_section
                    as unsafe extern "C" fn(
                        *mut bfd,
                        *mut bfd_link_info,
                        *mut bfd,
                        *mut asection,
                        *mut bfd_byte,
                        *mut internal_reloc,
                        *mut internal_syment,
                        *mut *mut asection,
                    ) -> bool,
            ),
            _bfd_coff_rtype_to_howto: Some(
                coff_amd64_rtype_to_howto
                    as unsafe extern "C" fn(
                        *mut bfd,
                        *mut asection,
                        *mut internal_reloc,
                        *mut coff_link_hash_entry,
                        *mut internal_syment,
                        *mut bfd_vma,
                    ) -> *const reloc_howto_type,
            ),
            _bfd_coff_adjust_symndx: None,
            _bfd_coff_link_add_one_symbol: Some(
                _bfd_generic_link_add_one_symbol
                    as unsafe extern "C" fn(
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
            ),
            _bfd_coff_link_output_has_begun: Some(
                coff_link_output_has_begun
                    as unsafe extern "C" fn(*mut bfd, *mut coff_final_link_info) -> bool,
            ),
            _bfd_coff_final_link_postscript: Some(
                _bfd_pex64i_final_link_postscript
                    as unsafe extern "C" fn(*mut bfd, *mut coff_final_link_info) -> bool,
            ),
            _bfd_coff_print_pdata: Some(
                pex64_bfd_print_pdata
                    as unsafe extern "C" fn(*mut bfd, *mut libc::c_void) -> bool,
            ),
        };
        init
    }
};
static mut header_bigobj_classid: [libc::c_char; 16] = [
    0xc7 as libc::c_int as libc::c_char,
    0xa1 as libc::c_int as libc::c_char,
    0xba as libc::c_int as libc::c_char,
    0xd1 as libc::c_int as libc::c_char,
    0xee as libc::c_int as libc::c_char,
    0xba as libc::c_int as libc::c_char,
    0xa9 as libc::c_int as libc::c_char,
    0x4b as libc::c_int as libc::c_char,
    0xaf as libc::c_int as libc::c_char,
    0x20 as libc::c_int as libc::c_char,
    0xfa as libc::c_int as libc::c_char,
    0xf6 as libc::c_int as libc::c_char,
    0x6a as libc::c_int as libc::c_char,
    0xa4 as libc::c_int as libc::c_char,
    0xdc as libc::c_int as libc::c_char,
    0xb8 as libc::c_int as libc::c_char,
];
unsafe extern "C" fn coff_bigobj_swap_filehdr_in(
    mut abfd: *mut bfd,
    mut src: *mut libc::c_void,
    mut dst: *mut libc::c_void,
) {
    let mut filehdr_src: *mut external_ANON_OBJECT_HEADER_BIGOBJ = src
        as *mut external_ANON_OBJECT_HEADER_BIGOBJ;
    let mut filehdr_dst: *mut internal_filehdr = dst as *mut internal_filehdr;
    (*filehdr_dst)
        .f_magic = (Some(
        ((*(*abfd).xvec).bfd_h_getx16).expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )(((*filehdr_src).Machine).as_mut_ptr() as *const libc::c_void)
        as libc::c_ushort;
    (*filehdr_dst)
        .f_nscns = (Some(
        ((*(*abfd).xvec).bfd_h_getx32).expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )(((*filehdr_src).NumberOfSections).as_mut_ptr() as *const libc::c_void)
        as libc::c_uint;
    (*filehdr_dst)
        .f_timdat = (Some(
        ((*(*abfd).xvec).bfd_h_getx32).expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )(((*filehdr_src).TimeDateStamp).as_mut_ptr() as *const libc::c_void)
        as libc::c_long;
    (*filehdr_dst)
        .f_symptr = (Some(
        ((*(*abfd).xvec).bfd_h_getx32).expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )(((*filehdr_src).PointerToSymbolTable).as_mut_ptr() as *const libc::c_void);
    (*filehdr_dst)
        .f_nsyms = (Some(
        ((*(*abfd).xvec).bfd_h_getx32).expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )(((*filehdr_src).NumberOfSymbols).as_mut_ptr() as *const libc::c_void)
        as libc::c_long;
    (*filehdr_dst).f_opthdr = 0 as libc::c_int as libc::c_ushort;
    (*filehdr_dst).f_flags = 0 as libc::c_int as libc::c_ushort;
    if (Some(((*(*abfd).xvec).bfd_h_getx16).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(((*filehdr_src).Sig1).as_mut_ptr() as *const libc::c_void)
        != 0 as libc::c_int as libc::c_ulong
        || (Some(((*(*abfd).xvec).bfd_h_getx16).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(((*filehdr_src).Sig2).as_mut_ptr() as *const libc::c_void)
            != 0xffff as libc::c_int as libc::c_ulong
        || (Some(((*(*abfd).xvec).bfd_h_getx16).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(((*filehdr_src).Version).as_mut_ptr() as *const libc::c_void)
            != 2 as libc::c_int as libc::c_ulong
        || memcmp(
            ((*filehdr_src).ClassID).as_mut_ptr() as *const libc::c_void,
            header_bigobj_classid.as_ptr() as *const libc::c_void,
            16 as libc::c_int as libc::c_ulong,
        ) != 0 as libc::c_int
    {
        (*filehdr_dst).f_opthdr = 0xffff as libc::c_int as libc::c_ushort;
    }
}
unsafe extern "C" fn coff_bigobj_swap_filehdr_out(
    mut abfd: *mut bfd,
    mut in_0: *mut libc::c_void,
    mut out: *mut libc::c_void,
) -> libc::c_uint {
    let mut filehdr_in: *mut internal_filehdr = in_0 as *mut internal_filehdr;
    let mut filehdr_out: *mut external_ANON_OBJECT_HEADER_BIGOBJ = out
        as *mut external_ANON_OBJECT_HEADER_BIGOBJ;
    memset(
        filehdr_out as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<external_ANON_OBJECT_HEADER_BIGOBJ>() as libc::c_ulong,
    );
    (Some(((*(*abfd).xvec).bfd_h_putx16).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        0 as libc::c_int as bfd_vma,
        ((*filehdr_out).Sig1).as_mut_ptr() as *mut libc::c_void,
    );
    (Some(((*(*abfd).xvec).bfd_h_putx16).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        0xffff as libc::c_int as bfd_vma,
        ((*filehdr_out).Sig2).as_mut_ptr() as *mut libc::c_void,
    );
    (Some(((*(*abfd).xvec).bfd_h_putx16).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        2 as libc::c_int as bfd_vma,
        ((*filehdr_out).Version).as_mut_ptr() as *mut libc::c_void,
    );
    memcpy(
        ((*filehdr_out).ClassID).as_mut_ptr() as *mut libc::c_void,
        header_bigobj_classid.as_ptr() as *const libc::c_void,
        16 as libc::c_int as libc::c_ulong,
    );
    (Some(((*(*abfd).xvec).bfd_h_putx16).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (*filehdr_in).f_magic as bfd_vma,
        ((*filehdr_out).Machine).as_mut_ptr() as *mut libc::c_void,
    );
    (Some(((*(*abfd).xvec).bfd_h_putx32).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (*filehdr_in).f_nscns as bfd_vma,
        ((*filehdr_out).NumberOfSections).as_mut_ptr() as *mut libc::c_void,
    );
    (Some(((*(*abfd).xvec).bfd_h_putx32).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (*filehdr_in).f_timdat as bfd_vma,
        ((*filehdr_out).TimeDateStamp).as_mut_ptr() as *mut libc::c_void,
    );
    (Some(((*(*abfd).xvec).bfd_h_putx32).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (*filehdr_in).f_symptr,
        ((*filehdr_out).PointerToSymbolTable).as_mut_ptr() as *mut libc::c_void,
    );
    (Some(((*(*abfd).xvec).bfd_h_putx32).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (*filehdr_in).f_nsyms as bfd_vma,
        ((*filehdr_out).NumberOfSymbols).as_mut_ptr() as *mut libc::c_void,
    );
    return (*((*(*abfd).xvec).backend_data as *mut bfd_coff_backend_data))._bfd_filhsz;
}
unsafe extern "C" fn coff_bigobj_swap_sym_in(
    mut abfd: *mut bfd,
    mut ext1: *mut libc::c_void,
    mut in1: *mut libc::c_void,
) {
    let mut ext: *mut external_SYMBOL_EX = ext1 as *mut external_SYMBOL_EX;
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
    if !(::core::mem::size_of::<libc::c_int>() as libc::c_ulong
        >= 4 as libc::c_int as libc::c_ulong)
    {
        bfd_assert(
            b"./coffcode.h\0" as *const u8 as *const libc::c_char,
            5599 as libc::c_int,
        );
    }
    (*in_0)
        .n_scnum = (Some(
        ((*(*abfd).xvec).bfd_h_getx32).expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )(((*ext).e_scnum).as_mut_ptr() as *const libc::c_void) as libc::c_int;
    (*in_0)
        .n_type = (Some(
        ((*(*abfd).xvec).bfd_h_getx16).expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )(((*ext).e_type).as_mut_ptr() as *const libc::c_void) as libc::c_ushort;
    (*in_0)
        .n_sclass = (*(((*ext).e_sclass).as_mut_ptr() as *const libc::c_uchar) as bfd_vma
        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
    (*in_0)
        .n_numaux = (*(((*ext).e_numaux).as_mut_ptr() as *const libc::c_uchar) as bfd_vma
        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
}
unsafe extern "C" fn coff_bigobj_swap_sym_out(
    mut abfd: *mut bfd,
    mut inp: *mut libc::c_void,
    mut extp: *mut libc::c_void,
) -> libc::c_uint {
    let mut in_0: *mut internal_syment = inp as *mut internal_syment;
    let mut ext: *mut external_SYMBOL_EX = extp as *mut external_SYMBOL_EX;
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
    (Some(((*(*abfd).xvec).bfd_h_putx32).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )((*in_0).n_value, ((*ext).e_value).as_mut_ptr() as *mut libc::c_void);
    (Some(((*(*abfd).xvec).bfd_h_putx32).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (*in_0).n_scnum as bfd_vma,
        ((*ext).e_scnum).as_mut_ptr() as *mut libc::c_void,
    );
    (Some(((*(*abfd).xvec).bfd_h_putx16).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )((*in_0).n_type as bfd_vma, ((*ext).e_type).as_mut_ptr() as *mut libc::c_void);
    *(((*ext).e_sclass).as_mut_ptr()
        as *mut libc::c_uchar) = ((*in_0).n_sclass as libc::c_int & 0xff as libc::c_int)
        as libc::c_uchar;
    *(((*ext).e_numaux).as_mut_ptr()
        as *mut libc::c_uchar) = ((*in_0).n_numaux as libc::c_int & 0xff as libc::c_int)
        as libc::c_uchar;
    return 20 as libc::c_int as libc::c_uint;
}
unsafe extern "C" fn coff_bigobj_swap_aux_in(
    mut abfd: *mut bfd,
    mut ext1: *mut libc::c_void,
    mut type_0: libc::c_int,
    mut in_class: libc::c_int,
    mut indx: libc::c_int,
    mut numaux: libc::c_int,
    mut in1: *mut libc::c_void,
) {
    let mut ext: *mut external_AUX_SYMBOL_EX = ext1 as *mut external_AUX_SYMBOL_EX;
    let mut in_0: *mut internal_auxent = in1 as *mut internal_auxent;
    memset(
        in_0 as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<internal_auxent>() as libc::c_ulong,
    );
    match in_class {
        103 => {
            if numaux > 1 as libc::c_int {
                if indx == 0 as libc::c_int {
                    memcpy(
                        ((*in_0).x_file.x_fname).as_mut_ptr() as *mut libc::c_void,
                        ((*ext).File.Name).as_mut_ptr() as *const libc::c_void,
                        (numaux as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<external_AUX_SYMBOL_EX>()
                                    as libc::c_ulong,
                            ),
                    );
                }
            } else {
                memcpy(
                    ((*in_0).x_file.x_fname).as_mut_ptr() as *mut libc::c_void,
                    ((*ext).File.Name).as_mut_ptr() as *const libc::c_void,
                    ::core::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong,
                );
            }
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
                    )(((*ext).Section.Length).as_mut_ptr() as *const libc::c_void)
                    as libc::c_long;
                (*in_0)
                    .x_scn
                    .x_nreloc = (Some(
                    ((*(*abfd).xvec).bfd_h_getx16).expect("non-null function pointer"),
                ))
                    .expect(
                        "non-null function pointer",
                    )(
                    ((*ext).Section.NumberOfRelocations).as_mut_ptr()
                        as *const libc::c_void,
                ) as libc::c_ushort;
                (*in_0)
                    .x_scn
                    .x_nlinno = (Some(
                    ((*(*abfd).xvec).bfd_h_getx16).expect("non-null function pointer"),
                ))
                    .expect(
                        "non-null function pointer",
                    )(
                    ((*ext).Section.NumberOfLinenumbers).as_mut_ptr()
                        as *const libc::c_void,
                ) as libc::c_ushort;
                (*in_0)
                    .x_scn
                    .x_checksum = (Some(
                    ((*(*abfd).xvec).bfd_h_getx32).expect("non-null function pointer"),
                ))
                    .expect(
                        "non-null function pointer",
                    )(((*ext).Section.Checksum).as_mut_ptr() as *const libc::c_void);
                (*in_0)
                    .x_scn
                    .x_associated = ((Some(
                    ((*(*abfd).xvec).bfd_h_getx16).expect("non-null function pointer"),
                ))
                    .expect(
                        "non-null function pointer",
                    )(((*ext).Section.Number).as_mut_ptr() as *const libc::c_void)
                    | (Some(
                        ((*(*abfd).xvec).bfd_h_getx16)
                            .expect("non-null function pointer"),
                    ))
                        .expect(
                            "non-null function pointer",
                        )(
                        ((*ext).Section.HighNumber).as_mut_ptr() as *const libc::c_void,
                    ) << 16 as libc::c_int) as libc::c_ushort;
                (*in_0)
                    .x_scn
                    .x_comdat = (*(((*ext).Section.Selection).as_mut_ptr()
                    as *const libc::c_uchar) as bfd_vma
                    & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                return;
            }
        }
        _ => {
            (*in_0)
                .x_sym
                .x_tagndx
                .l = (Some(
                ((*(*abfd).xvec).bfd_h_getx32).expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(((*ext).Sym.WeakDefaultSymIndex).as_mut_ptr() as *const libc::c_void)
                as libc::c_long;
        }
    };
}
unsafe extern "C" fn coff_bigobj_swap_aux_out(
    mut abfd: *mut bfd,
    mut inp: *mut libc::c_void,
    mut type_0: libc::c_int,
    mut in_class: libc::c_int,
    mut _indx: libc::c_int,
    mut _numaux: libc::c_int,
    mut extp: *mut libc::c_void,
) -> libc::c_uint {
    let mut in_0: *mut internal_auxent = inp as *mut internal_auxent;
    let mut ext: *mut external_AUX_SYMBOL_EX = extp as *mut external_AUX_SYMBOL_EX;
    memset(
        ext as *mut libc::c_void,
        0 as libc::c_int,
        18 as libc::c_int as libc::c_ulong,
    );
    match in_class {
        103 => {
            memcpy(
                ((*ext).File.Name).as_mut_ptr() as *mut libc::c_void,
                ((*in_0).x_file.x_fname).as_mut_ptr() as *const libc::c_void,
                ::core::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong,
            );
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
                    ((*ext).Section.Length).as_mut_ptr() as *mut libc::c_void,
                );
                (Some(
                    ((*(*abfd).xvec).bfd_h_putx16).expect("non-null function pointer"),
                ))
                    .expect(
                        "non-null function pointer",
                    )(
                    (*in_0).x_scn.x_nreloc as bfd_vma,
                    ((*ext).Section.NumberOfRelocations).as_mut_ptr()
                        as *mut libc::c_void,
                );
                (Some(
                    ((*(*abfd).xvec).bfd_h_putx16).expect("non-null function pointer"),
                ))
                    .expect(
                        "non-null function pointer",
                    )(
                    (*in_0).x_scn.x_nlinno as bfd_vma,
                    ((*ext).Section.NumberOfLinenumbers).as_mut_ptr()
                        as *mut libc::c_void,
                );
                (Some(
                    ((*(*abfd).xvec).bfd_h_putx32).expect("non-null function pointer"),
                ))
                    .expect(
                        "non-null function pointer",
                    )(
                    (*in_0).x_scn.x_checksum,
                    ((*ext).Section.Checksum).as_mut_ptr() as *mut libc::c_void,
                );
                (Some(
                    ((*(*abfd).xvec).bfd_h_putx16).expect("non-null function pointer"),
                ))
                    .expect(
                        "non-null function pointer",
                    )(
                    ((*in_0).x_scn.x_associated as libc::c_int & 0xffff as libc::c_int)
                        as bfd_vma,
                    ((*ext).Section.Number).as_mut_ptr() as *mut libc::c_void,
                );
                (Some(
                    ((*(*abfd).xvec).bfd_h_putx16).expect("non-null function pointer"),
                ))
                    .expect(
                        "non-null function pointer",
                    )(
                    ((*in_0).x_scn.x_associated as libc::c_int >> 16 as libc::c_int)
                        as bfd_vma,
                    ((*ext).Section.HighNumber).as_mut_ptr() as *mut libc::c_void,
                );
                *(((*ext).Section.Selection).as_mut_ptr()
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
        ((*ext).Sym.WeakDefaultSymIndex).as_mut_ptr() as *mut libc::c_void,
    );
    (Some(((*(*abfd).xvec).bfd_h_putx32).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        1 as libc::c_int as bfd_vma,
        ((*ext).Sym.WeakSearchType).as_mut_ptr() as *mut libc::c_void,
    );
    return 18 as libc::c_int as libc::c_uint;
}
static mut bigobj_swap_table: bfd_coff_backend_data = unsafe {
    {
        let mut init = bfd_coff_backend_data {
            _bfd_coff_swap_aux_in: Some(
                coff_bigobj_swap_aux_in
                    as unsafe extern "C" fn(
                        *mut bfd,
                        *mut libc::c_void,
                        libc::c_int,
                        libc::c_int,
                        libc::c_int,
                        libc::c_int,
                        *mut libc::c_void,
                    ) -> (),
            ),
            _bfd_coff_swap_sym_in: Some(
                coff_bigobj_swap_sym_in
                    as unsafe extern "C" fn(
                        *mut bfd,
                        *mut libc::c_void,
                        *mut libc::c_void,
                    ) -> (),
            ),
            _bfd_coff_swap_lineno_in: Some(
                _bfd_pex64i_swap_lineno_in
                    as unsafe extern "C" fn(
                        *mut bfd,
                        *mut libc::c_void,
                        *mut libc::c_void,
                    ) -> (),
            ),
            _bfd_coff_swap_aux_out: Some(
                coff_bigobj_swap_aux_out
                    as unsafe extern "C" fn(
                        *mut bfd,
                        *mut libc::c_void,
                        libc::c_int,
                        libc::c_int,
                        libc::c_int,
                        libc::c_int,
                        *mut libc::c_void,
                    ) -> libc::c_uint,
            ),
            _bfd_coff_swap_sym_out: Some(
                coff_bigobj_swap_sym_out
                    as unsafe extern "C" fn(
                        *mut bfd,
                        *mut libc::c_void,
                        *mut libc::c_void,
                    ) -> libc::c_uint,
            ),
            _bfd_coff_swap_lineno_out: Some(
                _bfd_pex64i_swap_lineno_out
                    as unsafe extern "C" fn(
                        *mut bfd,
                        *mut libc::c_void,
                        *mut libc::c_void,
                    ) -> libc::c_uint,
            ),
            _bfd_coff_swap_reloc_out: Some(
                coff_swap_reloc_out
                    as unsafe extern "C" fn(
                        *mut bfd,
                        *mut libc::c_void,
                        *mut libc::c_void,
                    ) -> libc::c_uint,
            ),
            _bfd_coff_swap_filehdr_out: Some(
                coff_bigobj_swap_filehdr_out
                    as unsafe extern "C" fn(
                        *mut bfd,
                        *mut libc::c_void,
                        *mut libc::c_void,
                    ) -> libc::c_uint,
            ),
            _bfd_coff_swap_aouthdr_out: Some(
                _bfd_pex64i_swap_aouthdr_out
                    as unsafe extern "C" fn(
                        *mut bfd,
                        *mut libc::c_void,
                        *mut libc::c_void,
                    ) -> libc::c_uint,
            ),
            _bfd_coff_swap_scnhdr_out: Some(
                _bfd_pex64i_swap_scnhdr_out
                    as unsafe extern "C" fn(
                        *mut bfd,
                        *mut libc::c_void,
                        *mut libc::c_void,
                    ) -> libc::c_uint,
            ),
            _bfd_filhsz: (14 as libc::c_int * 4 as libc::c_int) as libc::c_uint,
            _bfd_aoutsz: (24 as libc::c_int + 196 as libc::c_int
                + 5 as libc::c_int * 4 as libc::c_int) as libc::c_uint,
            _bfd_scnhsz: 40 as libc::c_int as libc::c_uint,
            _bfd_symesz: 20 as libc::c_int as libc::c_uint,
            _bfd_auxesz: 20 as libc::c_int as libc::c_uint,
            _bfd_relsz: 10 as libc::c_int as libc::c_uint,
            _bfd_linesz: (4 as libc::c_int + 2 as libc::c_int) as libc::c_uint,
            _bfd_filnmlen: 20 as libc::c_int as libc::c_uint,
            _bfd_coff_long_filenames: 1 as libc::c_int != 0,
            _bfd_coff_long_section_names: 1 as libc::c_int != 0,
            _bfd_coff_set_long_section_names: Some(
                bfd_coff_set_long_section_names_allowed
                    as unsafe extern "C" fn(*mut bfd, libc::c_int) -> bool,
            ),
            _bfd_coff_default_section_alignment_power: 2 as libc::c_int as libc::c_uint,
            _bfd_coff_force_symnames_in_strings: 0 as libc::c_int != 0,
            _bfd_coff_debug_string_prefix_length: 2 as libc::c_int as libc::c_uint,
            _bfd_coff_max_nscns: (1 as libc::c_uint) << 31 as libc::c_int,
            _bfd_coff_swap_filehdr_in: Some(
                coff_bigobj_swap_filehdr_in
                    as unsafe extern "C" fn(
                        *mut bfd,
                        *mut libc::c_void,
                        *mut libc::c_void,
                    ) -> (),
            ),
            _bfd_coff_swap_aouthdr_in: Some(
                _bfd_pex64i_swap_aouthdr_in
                    as unsafe extern "C" fn(
                        *mut bfd,
                        *mut libc::c_void,
                        *mut libc::c_void,
                    ) -> (),
            ),
            _bfd_coff_swap_scnhdr_in: Some(
                coff_swap_scnhdr_in
                    as unsafe extern "C" fn(
                        *mut bfd,
                        *mut libc::c_void,
                        *mut libc::c_void,
                    ) -> (),
            ),
            _bfd_coff_swap_reloc_in: Some(
                coff_swap_reloc_in
                    as unsafe extern "C" fn(
                        *mut bfd,
                        *mut libc::c_void,
                        *mut libc::c_void,
                    ) -> (),
            ),
            _bfd_coff_bad_format_hook: Some(
                coff_bad_format_hook
                    as unsafe extern "C" fn(*mut bfd, *mut libc::c_void) -> bool,
            ),
            _bfd_coff_set_arch_mach_hook: Some(
                coff_set_arch_mach_hook
                    as unsafe extern "C" fn(*mut bfd, *mut libc::c_void) -> bool,
            ),
            _bfd_coff_mkobject_hook: Some(
                pe_mkobject_hook
                    as unsafe extern "C" fn(
                        *mut bfd,
                        *mut libc::c_void,
                        *mut libc::c_void,
                    ) -> *mut libc::c_void,
            ),
            _bfd_styp_to_sec_flags_hook: Some(
                styp_to_sec_flags
                    as unsafe extern "C" fn(
                        *mut bfd,
                        *mut libc::c_void,
                        *const libc::c_char,
                        *mut asection,
                        *mut flagword,
                    ) -> bool,
            ),
            _bfd_set_alignment_hook: Some(
                coff_set_alignment_hook
                    as unsafe extern "C" fn(
                        *mut bfd,
                        *mut asection,
                        *mut libc::c_void,
                    ) -> (),
            ),
            _bfd_coff_slurp_symbol_table: Some(
                coff_slurp_symbol_table as unsafe extern "C" fn(*mut bfd) -> bool,
            ),
            _bfd_coff_symname_in_debug: Some(
                symname_in_debug_hook
                    as unsafe extern "C" fn(*mut bfd, *mut internal_syment) -> bool,
            ),
            _bfd_coff_pointerize_aux_hook: None,
            _bfd_coff_print_aux: Some(
                coff_print_aux
                    as unsafe extern "C" fn(
                        *mut bfd,
                        *mut FILE,
                        *mut combined_entry_type,
                        *mut combined_entry_type,
                        *mut combined_entry_type,
                        libc::c_uint,
                    ) -> bool,
            ),
            _bfd_coff_reloc16_extra_cases: Some(
                dummy_reloc16_extra_cases
                    as unsafe extern "C" fn(
                        *mut bfd,
                        *mut bfd_link_info,
                        *mut bfd_link_order,
                        *mut arelent,
                        *mut bfd_byte,
                        *mut libc::c_uint,
                        *mut libc::c_uint,
                    ) -> (),
            ),
            _bfd_coff_reloc16_estimate: Some(
                dummy_reloc16_estimate
                    as unsafe extern "C" fn(
                        *mut bfd,
                        *mut asection,
                        *mut arelent,
                        libc::c_uint,
                        *mut bfd_link_info,
                    ) -> libc::c_int,
            ),
            _bfd_coff_classify_symbol: Some(
                coff_classify_symbol
                    as unsafe extern "C" fn(
                        *mut bfd,
                        *mut internal_syment,
                    ) -> coff_symbol_classification,
            ),
            _bfd_coff_compute_section_file_positions: Some(
                coff_compute_section_file_positions
                    as unsafe extern "C" fn(*mut bfd) -> bool,
            ),
            _bfd_coff_start_final_link: None,
            _bfd_coff_relocate_section: Some(
                coff_pe_amd64_relocate_section
                    as unsafe extern "C" fn(
                        *mut bfd,
                        *mut bfd_link_info,
                        *mut bfd,
                        *mut asection,
                        *mut bfd_byte,
                        *mut internal_reloc,
                        *mut internal_syment,
                        *mut *mut asection,
                    ) -> bool,
            ),
            _bfd_coff_rtype_to_howto: Some(
                coff_amd64_rtype_to_howto
                    as unsafe extern "C" fn(
                        *mut bfd,
                        *mut asection,
                        *mut internal_reloc,
                        *mut coff_link_hash_entry,
                        *mut internal_syment,
                        *mut bfd_vma,
                    ) -> *const reloc_howto_type,
            ),
            _bfd_coff_adjust_symndx: None,
            _bfd_coff_link_add_one_symbol: Some(
                _bfd_generic_link_add_one_symbol
                    as unsafe extern "C" fn(
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
            ),
            _bfd_coff_link_output_has_begun: Some(
                coff_link_output_has_begun
                    as unsafe extern "C" fn(*mut bfd, *mut coff_final_link_info) -> bool,
            ),
            _bfd_coff_final_link_postscript: Some(
                _bfd_pex64i_final_link_postscript
                    as unsafe extern "C" fn(*mut bfd, *mut coff_final_link_info) -> bool,
            ),
            _bfd_coff_print_pdata: Some(
                pex64_bfd_print_pdata
                    as unsafe extern "C" fn(*mut bfd, *mut libc::c_void) -> bool,
            ),
        };
        init
    }
};
#[no_mangle]
pub static mut x86_64_pe_vec: bfd_target = unsafe {
    {
        let mut init = bfd_target {
            name: b"pe-x86-64\0" as *const u8 as *const libc::c_char,
            flavour: bfd_target_coff_flavour,
            byteorder: BFD_ENDIAN_LITTLE,
            header_byteorder: BFD_ENDIAN_LITTLE,
            object_flags: (0x1 as libc::c_int | 0x2 as libc::c_int | 0x4 as libc::c_int
                | 0x8 as libc::c_int | 0x10 as libc::c_int | 0x20 as libc::c_int
                | 0x80 as libc::c_int | 0x100 as libc::c_int | 0x4000 as libc::c_int
                | 0x8000 as libc::c_int) as flagword,
            section_flags: (0x100 as libc::c_int | 0x1 as libc::c_int
                | 0x2 as libc::c_int | 0x4 as libc::c_int | 0x20000 as libc::c_int
                | 0xc0000 as libc::c_int | 0x8 as libc::c_int | 0x2000 as libc::c_int
                | 0x10 as libc::c_int | 0x20 as libc::c_int | 0x8000 as libc::c_int)
                as flagword,
            symbol_leading_char: 0 as libc::c_int as libc::c_char,
            ar_pad_char: '/' as i32 as libc::c_char,
            ar_max_namelen: 15 as libc::c_int as libc::c_uchar,
            match_priority: 0 as libc::c_int as libc::c_uchar,
            keep_unused_section_symbols: 1 as libc::c_int != 0,
            bfd_getx64: Some(
                bfd_getl64 as unsafe extern "C" fn(*const libc::c_void) -> bfd_uint64_t,
            ),
            bfd_getx_signed_64: Some(
                bfd_getl_signed_64
                    as unsafe extern "C" fn(*const libc::c_void) -> bfd_int64_t,
            ),
            bfd_putx64: Some(
                bfd_putl64 as unsafe extern "C" fn(bfd_uint64_t, *mut libc::c_void) -> (),
            ),
            bfd_getx32: Some(
                bfd_getl32 as unsafe extern "C" fn(*const libc::c_void) -> bfd_vma,
            ),
            bfd_getx_signed_32: Some(
                bfd_getl_signed_32
                    as unsafe extern "C" fn(*const libc::c_void) -> bfd_signed_vma,
            ),
            bfd_putx32: Some(
                bfd_putl32 as unsafe extern "C" fn(bfd_vma, *mut libc::c_void) -> (),
            ),
            bfd_getx16: Some(
                bfd_getl16 as unsafe extern "C" fn(*const libc::c_void) -> bfd_vma,
            ),
            bfd_getx_signed_16: Some(
                bfd_getl_signed_16
                    as unsafe extern "C" fn(*const libc::c_void) -> bfd_signed_vma,
            ),
            bfd_putx16: Some(
                bfd_putl16 as unsafe extern "C" fn(bfd_vma, *mut libc::c_void) -> (),
            ),
            bfd_h_getx64: Some(
                bfd_getl64 as unsafe extern "C" fn(*const libc::c_void) -> bfd_uint64_t,
            ),
            bfd_h_getx_signed_64: Some(
                bfd_getl_signed_64
                    as unsafe extern "C" fn(*const libc::c_void) -> bfd_int64_t,
            ),
            bfd_h_putx64: Some(
                bfd_putl64 as unsafe extern "C" fn(bfd_uint64_t, *mut libc::c_void) -> (),
            ),
            bfd_h_getx32: Some(
                bfd_getl32 as unsafe extern "C" fn(*const libc::c_void) -> bfd_vma,
            ),
            bfd_h_getx_signed_32: Some(
                bfd_getl_signed_32
                    as unsafe extern "C" fn(*const libc::c_void) -> bfd_signed_vma,
            ),
            bfd_h_putx32: Some(
                bfd_putl32 as unsafe extern "C" fn(bfd_vma, *mut libc::c_void) -> (),
            ),
            bfd_h_getx16: Some(
                bfd_getl16 as unsafe extern "C" fn(*const libc::c_void) -> bfd_vma,
            ),
            bfd_h_getx_signed_16: Some(
                bfd_getl_signed_16
                    as unsafe extern "C" fn(*const libc::c_void) -> bfd_signed_vma,
            ),
            bfd_h_putx16: Some(
                bfd_putl16 as unsafe extern "C" fn(bfd_vma, *mut libc::c_void) -> (),
            ),
            _bfd_check_format: [
                Some(_bfd_dummy_target as unsafe extern "C" fn(*mut bfd) -> bfd_cleanup),
                Some(coff_object_p as unsafe extern "C" fn(*mut bfd) -> bfd_cleanup),
                Some(
                    bfd_generic_archive_p
                        as unsafe extern "C" fn(*mut bfd) -> bfd_cleanup,
                ),
                Some(coff_object_p as unsafe extern "C" fn(*mut bfd) -> bfd_cleanup),
            ],
            _bfd_set_format: [
                Some(
                    _bfd_bool_bfd_false_error as unsafe extern "C" fn(*mut bfd) -> bool,
                ),
                Some(pe_mkobject as unsafe extern "C" fn(*mut bfd) -> bool),
                Some(_bfd_generic_mkarchive as unsafe extern "C" fn(*mut bfd) -> bool),
                Some(_bfd_bool_bfd_false_error as unsafe extern "C" fn(*mut bfd) -> bool),
            ],
            _bfd_write_contents: [
                Some(
                    _bfd_bool_bfd_false_error as unsafe extern "C" fn(*mut bfd) -> bool,
                ),
                Some(
                    coff_write_object_contents as unsafe extern "C" fn(*mut bfd) -> bool,
                ),
                Some(
                    _bfd_write_archive_contents as unsafe extern "C" fn(*mut bfd) -> bool,
                ),
                Some(_bfd_bool_bfd_false_error as unsafe extern "C" fn(*mut bfd) -> bool),
            ],
            _close_and_cleanup: Some(
                _bfd_coff_close_and_cleanup as unsafe extern "C" fn(*mut bfd) -> bool,
            ),
            _bfd_free_cached_info: Some(
                _bfd_bool_bfd_true as unsafe extern "C" fn(*mut bfd) -> bool,
            ),
            _new_section_hook: Some(
                coff_new_section_hook
                    as unsafe extern "C" fn(*mut bfd, *mut asection) -> bool,
            ),
            _bfd_get_section_contents: Some(
                _bfd_generic_get_section_contents
                    as unsafe extern "C" fn(
                        *mut bfd,
                        *mut asection,
                        *mut libc::c_void,
                        file_ptr,
                        bfd_size_type,
                    ) -> bool,
            ),
            _bfd_get_section_contents_in_window: Some(
                _bfd_generic_get_section_contents_in_window
                    as unsafe extern "C" fn(
                        *mut bfd,
                        *mut asection,
                        *mut bfd_window,
                        file_ptr,
                        bfd_size_type,
                    ) -> bool,
            ),
            _bfd_copy_private_bfd_data: Some(
                pe_bfd_copy_private_bfd_data
                    as unsafe extern "C" fn(*mut bfd, *mut bfd) -> bool,
            ),
            _bfd_merge_private_bfd_data: Some(
                _bfd_bool_bfd_link_true
                    as unsafe extern "C" fn(*mut bfd, *mut bfd_link_info) -> bool,
            ),
            _bfd_init_private_section_data: Some(
                _bfd_generic_init_private_section_data
                    as unsafe extern "C" fn(
                        *mut bfd,
                        *mut asection,
                        *mut bfd,
                        *mut asection,
                        *mut bfd_link_info,
                    ) -> bool,
            ),
            _bfd_copy_private_section_data: Some(
                _bfd_pex64_bfd_copy_private_section_data
                    as unsafe extern "C" fn(
                        *mut bfd,
                        *mut asection,
                        *mut bfd,
                        *mut asection,
                    ) -> bool,
            ),
            _bfd_copy_private_symbol_data: Some(
                _bfd_bool_bfd_asymbol_bfd_asymbol_true
                    as unsafe extern "C" fn(
                        *mut bfd,
                        *mut asymbol,
                        *mut bfd,
                        *mut asymbol,
                    ) -> bool,
            ),
            _bfd_copy_private_header_data: Some(
                _bfd_bool_bfd_bfd_true
                    as unsafe extern "C" fn(*mut bfd, *mut bfd) -> bool,
            ),
            _bfd_set_private_flags: Some(
                _bfd_bool_bfd_uint_true
                    as unsafe extern "C" fn(*mut bfd, libc::c_uint) -> bool,
            ),
            _bfd_print_private_bfd_data: Some(
                pe_print_private_bfd_data
                    as unsafe extern "C" fn(*mut bfd, *mut libc::c_void) -> bool,
            ),
            _core_file_failing_command: Some(
                _bfd_nocore_core_file_failing_command
                    as unsafe extern "C" fn(*mut bfd) -> *mut libc::c_char,
            ),
            _core_file_failing_signal: Some(
                _bfd_nocore_core_file_failing_signal
                    as unsafe extern "C" fn(*mut bfd) -> libc::c_int,
            ),
            _core_file_matches_executable_p: Some(
                _bfd_nocore_core_file_matches_executable_p
                    as unsafe extern "C" fn(*mut bfd, *mut bfd) -> bool,
            ),
            _core_file_pid: Some(
                _bfd_nocore_core_file_pid
                    as unsafe extern "C" fn(*mut bfd) -> libc::c_int,
            ),
            _bfd_slurp_armap: Some(
                bfd_slurp_armap as unsafe extern "C" fn(*mut bfd) -> bool,
            ),
            _bfd_slurp_extended_name_table: Some(
                _bfd_slurp_extended_name_table as unsafe extern "C" fn(*mut bfd) -> bool,
            ),
            _bfd_construct_extended_name_table: Some(
                _bfd_archive_coff_construct_extended_name_table
                    as unsafe extern "C" fn(
                        *mut bfd,
                        *mut *mut libc::c_char,
                        *mut bfd_size_type,
                        *mut *const libc::c_char,
                    ) -> bool,
            ),
            _bfd_truncate_arname: Some(
                bfd_dont_truncate_arname
                    as unsafe extern "C" fn(
                        *mut bfd,
                        *const libc::c_char,
                        *mut libc::c_char,
                    ) -> (),
            ),
            write_armap: Some(
                _bfd_coff_write_armap
                    as unsafe extern "C" fn(
                        *mut bfd,
                        libc::c_uint,
                        *mut orl,
                        libc::c_uint,
                        libc::c_int,
                    ) -> bool,
            ),
            _bfd_read_ar_hdr_fn: Some(
                _bfd_generic_read_ar_hdr
                    as unsafe extern "C" fn(*mut bfd) -> *mut libc::c_void,
            ),
            _bfd_write_ar_hdr_fn: Some(
                _bfd_generic_write_ar_hdr
                    as unsafe extern "C" fn(*mut bfd, *mut bfd) -> bool,
            ),
            openr_next_archived_file: Some(
                bfd_generic_openr_next_archived_file
                    as unsafe extern "C" fn(*mut bfd, *mut bfd) -> *mut bfd,
            ),
            _bfd_get_elt_at_index: Some(
                _bfd_generic_get_elt_at_index
                    as unsafe extern "C" fn(*mut bfd, symindex) -> *mut bfd,
            ),
            _bfd_stat_arch_elt: Some(
                bfd_generic_stat_arch_elt
                    as unsafe extern "C" fn(*mut bfd, *mut stat) -> libc::c_int,
            ),
            _bfd_update_armap_timestamp: Some(
                _bfd_bool_bfd_true as unsafe extern "C" fn(*mut bfd) -> bool,
            ),
            _bfd_get_symtab_upper_bound: Some(
                coff_get_symtab_upper_bound
                    as unsafe extern "C" fn(*mut bfd) -> libc::c_long,
            ),
            _bfd_canonicalize_symtab: Some(
                coff_canonicalize_symtab
                    as unsafe extern "C" fn(*mut bfd, *mut *mut asymbol) -> libc::c_long,
            ),
            _bfd_make_empty_symbol: Some(
                coff_make_empty_symbol as unsafe extern "C" fn(*mut bfd) -> *mut asymbol,
            ),
            _bfd_print_symbol: Some(
                coff_print_symbol
                    as unsafe extern "C" fn(
                        *mut bfd,
                        *mut libc::c_void,
                        *mut asymbol,
                        bfd_print_symbol_type,
                    ) -> (),
            ),
            _bfd_get_symbol_info: Some(
                _bfd_pex64_get_symbol_info
                    as unsafe extern "C" fn(
                        *mut bfd,
                        *mut asymbol,
                        *mut symbol_info,
                    ) -> (),
            ),
            _bfd_get_symbol_version_string: Some(
                _bfd_nosymbols_get_symbol_version_string
                    as unsafe extern "C" fn(
                        *mut bfd,
                        *mut asymbol,
                        bool,
                        *mut bool,
                    ) -> *const libc::c_char,
            ),
            _bfd_is_local_label_name: Some(
                coff_amd64_is_local_label_name
                    as unsafe extern "C" fn(*mut bfd, *const libc::c_char) -> bool,
            ),
            _bfd_is_target_special_symbol: Some(
                _bfd_bool_bfd_asymbol_false
                    as unsafe extern "C" fn(*mut bfd, *mut asymbol) -> bool,
            ),
            _get_lineno: Some(
                coff_get_lineno
                    as unsafe extern "C" fn(*mut bfd, *mut asymbol) -> *mut alent,
            ),
            _bfd_find_nearest_line: Some(
                coff_find_nearest_line
                    as unsafe extern "C" fn(
                        *mut bfd,
                        *mut *mut asymbol,
                        *mut asection,
                        bfd_vma,
                        *mut *const libc::c_char,
                        *mut *const libc::c_char,
                        *mut libc::c_uint,
                        *mut libc::c_uint,
                    ) -> bool,
            ),
            _bfd_find_line: Some(
                _bfd_nosymbols_find_line
                    as unsafe extern "C" fn(
                        *mut bfd,
                        *mut *mut asymbol,
                        *mut asymbol,
                        *mut *const libc::c_char,
                        *mut libc::c_uint,
                    ) -> bool,
            ),
            _bfd_find_inliner_info: Some(
                coff_find_inliner_info
                    as unsafe extern "C" fn(
                        *mut bfd,
                        *mut *const libc::c_char,
                        *mut *const libc::c_char,
                        *mut libc::c_uint,
                    ) -> bool,
            ),
            _bfd_make_debug_symbol: Some(
                coff_bfd_make_debug_symbol
                    as unsafe extern "C" fn(
                        *mut bfd,
                        *mut libc::c_void,
                        libc::c_ulong,
                    ) -> *mut asymbol,
            ),
            _read_minisymbols: Some(
                _bfd_generic_read_minisymbols
                    as unsafe extern "C" fn(
                        *mut bfd,
                        bool,
                        *mut *mut libc::c_void,
                        *mut libc::c_uint,
                    ) -> libc::c_long,
            ),
            _minisymbol_to_symbol: Some(
                _bfd_generic_minisymbol_to_symbol
                    as unsafe extern "C" fn(
                        *mut bfd,
                        bool,
                        *const libc::c_void,
                        *mut asymbol,
                    ) -> *mut asymbol,
            ),
            _get_reloc_upper_bound: Some(
                coff_get_reloc_upper_bound
                    as unsafe extern "C" fn(*mut bfd, sec_ptr) -> libc::c_long,
            ),
            _bfd_canonicalize_reloc: Some(
                coff_canonicalize_reloc
                    as unsafe extern "C" fn(
                        *mut bfd,
                        *mut asection,
                        *mut *mut arelent,
                        *mut *mut asymbol,
                    ) -> libc::c_long,
            ),
            _bfd_set_reloc: Some(
                _bfd_generic_set_reloc
                    as unsafe extern "C" fn(
                        *mut bfd,
                        sec_ptr,
                        *mut *mut arelent,
                        libc::c_uint,
                    ) -> (),
            ),
            reloc_type_lookup: Some(
                coff_amd64_reloc_type_lookup
                    as unsafe extern "C" fn(
                        *mut bfd,
                        bfd_reloc_code_real_type,
                    ) -> *const reloc_howto_type,
            ),
            reloc_name_lookup: Some(
                coff_amd64_reloc_name_lookup
                    as unsafe extern "C" fn(
                        *mut bfd,
                        *const libc::c_char,
                    ) -> *const reloc_howto_type,
            ),
            _bfd_set_arch_mach: Some(
                coff_set_arch_mach
                    as unsafe extern "C" fn(
                        *mut bfd,
                        bfd_architecture,
                        libc::c_ulong,
                    ) -> bool,
            ),
            _bfd_set_section_contents: Some(
                coff_set_section_contents
                    as unsafe extern "C" fn(
                        *mut bfd,
                        *mut asection,
                        *const libc::c_void,
                        file_ptr,
                        bfd_size_type,
                    ) -> bool,
            ),
            _bfd_sizeof_headers: Some(
                coff_sizeof_headers
                    as unsafe extern "C" fn(*mut bfd, *mut bfd_link_info) -> libc::c_int,
            ),
            _bfd_get_relocated_section_contents: Some(
                bfd_generic_get_relocated_section_contents
                    as unsafe extern "C" fn(
                        *mut bfd,
                        *mut bfd_link_info,
                        *mut bfd_link_order,
                        *mut bfd_byte,
                        bool,
                        *mut *mut asymbol,
                    ) -> *mut bfd_byte,
            ),
            _bfd_relax_section: Some(
                bfd_generic_relax_section
                    as unsafe extern "C" fn(
                        *mut bfd,
                        *mut asection,
                        *mut bfd_link_info,
                        *mut bool,
                    ) -> bool,
            ),
            _bfd_link_hash_table_create: Some(
                _bfd_coff_link_hash_table_create
                    as unsafe extern "C" fn(*mut bfd) -> *mut bfd_link_hash_table,
            ),
            _bfd_link_add_symbols: Some(
                pex64_link_add_symbols
                    as unsafe extern "C" fn(*mut bfd, *mut bfd_link_info) -> bool,
            ),
            _bfd_link_just_syms: Some(
                _bfd_generic_link_just_syms
                    as unsafe extern "C" fn(*mut asection, *mut bfd_link_info) -> (),
            ),
            _bfd_copy_link_hash_symbol_type: Some(
                _bfd_generic_copy_link_hash_symbol_type
                    as unsafe extern "C" fn(
                        *mut bfd,
                        *mut bfd_link_hash_entry,
                        *mut bfd_link_hash_entry,
                    ) -> (),
            ),
            _bfd_final_link: Some(
                _bfd_coff_final_link
                    as unsafe extern "C" fn(*mut bfd, *mut bfd_link_info) -> bool,
            ),
            _bfd_link_split_section: Some(
                _bfd_generic_link_split_section
                    as unsafe extern "C" fn(*mut bfd, *mut bfd_section) -> bool,
            ),
            _bfd_link_check_relocs: Some(
                _bfd_generic_link_check_relocs
                    as unsafe extern "C" fn(*mut bfd, *mut bfd_link_info) -> bool,
            ),
            _bfd_gc_sections: Some(
                bfd_coff_gc_sections
                    as unsafe extern "C" fn(*mut bfd, *mut bfd_link_info) -> bool,
            ),
            _bfd_lookup_section_flags: Some(
                bfd_generic_lookup_section_flags
                    as unsafe extern "C" fn(
                        *mut bfd_link_info,
                        *mut flag_info,
                        *mut asection,
                    ) -> bool,
            ),
            _bfd_merge_sections: Some(
                bfd_generic_merge_sections
                    as unsafe extern "C" fn(*mut bfd, *mut bfd_link_info) -> bool,
            ),
            _bfd_is_group_section: Some(
                bfd_generic_is_group_section
                    as unsafe extern "C" fn(*mut bfd, *const asection) -> bool,
            ),
            _bfd_group_name: Some(
                bfd_coff_group_name
                    as unsafe extern "C" fn(
                        *mut bfd,
                        *const asection,
                    ) -> *const libc::c_char,
            ),
            _bfd_discard_group: Some(
                bfd_generic_discard_group
                    as unsafe extern "C" fn(*mut bfd, *mut asection) -> bool,
            ),
            _section_already_linked: Some(
                _bfd_coff_section_already_linked
                    as unsafe extern "C" fn(
                        *mut bfd,
                        *mut asection,
                        *mut bfd_link_info,
                    ) -> bool,
            ),
            _bfd_define_common_symbol: Some(
                bfd_generic_define_common_symbol
                    as unsafe extern "C" fn(
                        *mut bfd,
                        *mut bfd_link_info,
                        *mut bfd_link_hash_entry,
                    ) -> bool,
            ),
            _bfd_link_hide_symbol: Some(
                _bfd_generic_link_hide_symbol
                    as unsafe extern "C" fn(
                        *mut bfd,
                        *mut bfd_link_info,
                        *mut bfd_link_hash_entry,
                    ) -> (),
            ),
            _bfd_define_start_stop: Some(
                bfd_generic_define_start_stop
                    as unsafe extern "C" fn(
                        *mut bfd_link_info,
                        *const libc::c_char,
                        *mut asection,
                    ) -> *mut bfd_link_hash_entry,
            ),
            _bfd_get_dynamic_symtab_upper_bound: Some(
                _bfd_long_bfd_n1_error as unsafe extern "C" fn(*mut bfd) -> libc::c_long,
            ),
            _bfd_canonicalize_dynamic_symtab: Some(
                _bfd_nosymbols_canonicalize_symtab
                    as unsafe extern "C" fn(*mut bfd, *mut *mut asymbol) -> libc::c_long,
            ),
            _bfd_get_synthetic_symtab: Some(
                _bfd_nodynamic_get_synthetic_symtab
                    as unsafe extern "C" fn(
                        *mut bfd,
                        libc::c_long,
                        *mut *mut asymbol,
                        libc::c_long,
                        *mut *mut asymbol,
                        *mut *mut asymbol,
                    ) -> libc::c_long,
            ),
            _bfd_get_dynamic_reloc_upper_bound: Some(
                _bfd_long_bfd_n1_error as unsafe extern "C" fn(*mut bfd) -> libc::c_long,
            ),
            _bfd_canonicalize_dynamic_reloc: Some(
                _bfd_nodynamic_canonicalize_dynamic_reloc
                    as unsafe extern "C" fn(
                        *mut bfd,
                        *mut *mut arelent,
                        *mut *mut asymbol,
                    ) -> libc::c_long,
            ),
            alternative_target: 0 as *const bfd_target,
            backend_data: &bfd_coff_std_swap_table as *const bfd_coff_backend_data
                as *mut bfd_coff_backend_data as *mut libc::c_void,
        };
        init
    }
};
unsafe extern "C" fn run_static_initializers() {
    coff_section_alignment_table_size = (::core::mem::size_of::<
        [coff_section_alignment_entry; 13],
    >() as libc::c_ulong)
        .wrapping_div(
            ::core::mem::size_of::<coff_section_alignment_entry>() as libc::c_ulong,
        ) as libc::c_uint;
    coff_section_alignment_table = [
        {
            let mut init = coff_section_alignment_entry {
                name: b".bss\0" as *const u8 as *const libc::c_char,
                comparison_length: -(1 as libc::c_int) as libc::c_uint,
                default_alignment_min: -(1 as libc::c_int) as libc::c_uint,
                default_alignment_max: -(1 as libc::c_int) as libc::c_uint,
                alignment_power: 4 as libc::c_int as libc::c_uint,
            };
            init
        },
        {
            let mut init = coff_section_alignment_entry {
                name: b".data\0" as *const u8 as *const libc::c_char,
                comparison_length: (::core::mem::size_of::<[libc::c_char; 6]>()
                    as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_uint,
                default_alignment_min: -(1 as libc::c_int) as libc::c_uint,
                default_alignment_max: -(1 as libc::c_int) as libc::c_uint,
                alignment_power: 4 as libc::c_int as libc::c_uint,
            };
            init
        },
        {
            let mut init = coff_section_alignment_entry {
                name: b".rdata\0" as *const u8 as *const libc::c_char,
                comparison_length: (::core::mem::size_of::<[libc::c_char; 7]>()
                    as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_uint,
                default_alignment_min: -(1 as libc::c_int) as libc::c_uint,
                default_alignment_max: -(1 as libc::c_int) as libc::c_uint,
                alignment_power: 4 as libc::c_int as libc::c_uint,
            };
            init
        },
        {
            let mut init = coff_section_alignment_entry {
                name: b".text\0" as *const u8 as *const libc::c_char,
                comparison_length: (::core::mem::size_of::<[libc::c_char; 6]>()
                    as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_uint,
                default_alignment_min: -(1 as libc::c_int) as libc::c_uint,
                default_alignment_max: -(1 as libc::c_int) as libc::c_uint,
                alignment_power: 4 as libc::c_int as libc::c_uint,
            };
            init
        },
        {
            let mut init = coff_section_alignment_entry {
                name: b".idata\0" as *const u8 as *const libc::c_char,
                comparison_length: (::core::mem::size_of::<[libc::c_char; 7]>()
                    as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_uint,
                default_alignment_min: -(1 as libc::c_int) as libc::c_uint,
                default_alignment_max: -(1 as libc::c_int) as libc::c_uint,
                alignment_power: 2 as libc::c_int as libc::c_uint,
            };
            init
        },
        {
            let mut init = coff_section_alignment_entry {
                name: b".pdata\0" as *const u8 as *const libc::c_char,
                comparison_length: -(1 as libc::c_int) as libc::c_uint,
                default_alignment_min: -(1 as libc::c_int) as libc::c_uint,
                default_alignment_max: -(1 as libc::c_int) as libc::c_uint,
                alignment_power: 2 as libc::c_int as libc::c_uint,
            };
            init
        },
        {
            let mut init = coff_section_alignment_entry {
                name: b".debug\0" as *const u8 as *const libc::c_char,
                comparison_length: (::core::mem::size_of::<[libc::c_char; 7]>()
                    as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_uint,
                default_alignment_min: -(1 as libc::c_int) as libc::c_uint,
                default_alignment_max: -(1 as libc::c_int) as libc::c_uint,
                alignment_power: 0 as libc::c_int as libc::c_uint,
            };
            init
        },
        {
            let mut init = coff_section_alignment_entry {
                name: b".zdebug\0" as *const u8 as *const libc::c_char,
                comparison_length: (::core::mem::size_of::<[libc::c_char; 8]>()
                    as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_uint,
                default_alignment_min: -(1 as libc::c_int) as libc::c_uint,
                default_alignment_max: -(1 as libc::c_int) as libc::c_uint,
                alignment_power: 0 as libc::c_int as libc::c_uint,
            };
            init
        },
        {
            let mut init = coff_section_alignment_entry {
                name: b".gnu.linkonce.wi.\0" as *const u8 as *const libc::c_char,
                comparison_length: (::core::mem::size_of::<[libc::c_char; 18]>()
                    as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_uint,
                default_alignment_min: -(1 as libc::c_int) as libc::c_uint,
                default_alignment_max: -(1 as libc::c_int) as libc::c_uint,
                alignment_power: 0 as libc::c_int as libc::c_uint,
            };
            init
        },
        {
            let mut init = coff_section_alignment_entry {
                name: b".stabstr\0" as *const u8 as *const libc::c_char,
                comparison_length: (::core::mem::size_of::<[libc::c_char; 9]>()
                    as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_uint,
                default_alignment_min: 1 as libc::c_int as libc::c_uint,
                default_alignment_max: -(1 as libc::c_int) as libc::c_uint,
                alignment_power: 0 as libc::c_int as libc::c_uint,
            };
            init
        },
        {
            let mut init = coff_section_alignment_entry {
                name: b".stab\0" as *const u8 as *const libc::c_char,
                comparison_length: (::core::mem::size_of::<[libc::c_char; 6]>()
                    as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_uint,
                default_alignment_min: 3 as libc::c_int as libc::c_uint,
                default_alignment_max: -(1 as libc::c_int) as libc::c_uint,
                alignment_power: 2 as libc::c_int as libc::c_uint,
            };
            init
        },
        {
            let mut init = coff_section_alignment_entry {
                name: b".ctors\0" as *const u8 as *const libc::c_char,
                comparison_length: -(1 as libc::c_int) as libc::c_uint,
                default_alignment_min: 3 as libc::c_int as libc::c_uint,
                default_alignment_max: -(1 as libc::c_int) as libc::c_uint,
                alignment_power: 2 as libc::c_int as libc::c_uint,
            };
            init
        },
        {
            let mut init = coff_section_alignment_entry {
                name: b".dtors\0" as *const u8 as *const libc::c_char,
                comparison_length: -(1 as libc::c_int) as libc::c_uint,
                default_alignment_min: 3 as libc::c_int as libc::c_uint,
                default_alignment_max: -(1 as libc::c_int) as libc::c_uint,
                alignment_power: 2 as libc::c_int as libc::c_uint,
            };
            init
        },
    ];
    howto_table = [
        {
            let mut init = reloc_howto_struct {
                size_bitsize_rightshift_bitpos_complain_on_overflow_negate_pc_relative_partial_inplace_pcrel_offset: [0; 4],
                type_0: 0 as libc::c_int as libc::c_uint,
                src_mask: 0 as libc::c_int as bfd_vma,
                dst_mask: 0 as libc::c_int as bfd_vma,
                special_function: None,
                name: 0 as *const libc::c_char,
            };
            init.set_size(
                (if (0 as libc::c_int) < 0 as libc::c_int {
                    -(0 as libc::c_int)
                } else {
                    0 as libc::c_int
                }) as libc::c_uint,
            );
            init.set_bitsize(0 as libc::c_int as libc::c_uint);
            init.set_rightshift(0 as libc::c_int as libc::c_uint);
            init.set_bitpos(0 as libc::c_int as libc::c_uint);
            init.set_complain_on_overflow(complain_overflow_dont);
            init.set_negate(
                ((0 as libc::c_int) < 0 as libc::c_int) as libc::c_int as libc::c_uint,
            );
            init.set_pc_relative(0 as libc::c_int as libc::c_uint);
            init.set_partial_inplace(0 as libc::c_int as libc::c_uint);
            init.set_pcrel_offset(0 as libc::c_int as libc::c_uint);
            init
        },
        {
            let mut init = reloc_howto_struct {
                size_bitsize_rightshift_bitpos_complain_on_overflow_negate_pc_relative_partial_inplace_pcrel_offset: [0; 4],
                type_0: 1 as libc::c_int as libc::c_uint,
                src_mask: 0xffffffffffffffff as libc::c_ulonglong as bfd_vma,
                dst_mask: 0xffffffffffffffff as libc::c_ulonglong as bfd_vma,
                special_function: Some(
                    coff_amd64_reloc
                        as unsafe extern "C" fn(
                            *mut bfd,
                            *mut arelent,
                            *mut asymbol,
                            *mut libc::c_void,
                            *mut asection,
                            *mut bfd,
                            *mut *mut libc::c_char,
                        ) -> bfd_reloc_status_type,
                ),
                name: b"IMAGE_REL_AMD64_ADDR64\0" as *const u8 as *const libc::c_char,
            };
            init.set_size(
                (if (4 as libc::c_int) < 0 as libc::c_int {
                    -(4 as libc::c_int)
                } else {
                    4 as libc::c_int
                }) as libc::c_uint,
            );
            init.set_bitsize(64 as libc::c_int as libc::c_uint);
            init.set_rightshift(0 as libc::c_int as libc::c_uint);
            init.set_bitpos(0 as libc::c_int as libc::c_uint);
            init.set_complain_on_overflow(complain_overflow_bitfield);
            init.set_negate(
                ((4 as libc::c_int) < 0 as libc::c_int) as libc::c_int as libc::c_uint,
            );
            init.set_pc_relative(0 as libc::c_int as libc::c_uint);
            init.set_partial_inplace(1 as libc::c_int as libc::c_uint);
            init.set_pcrel_offset(1 as libc::c_int as libc::c_uint);
            init
        },
        {
            let mut init = reloc_howto_struct {
                size_bitsize_rightshift_bitpos_complain_on_overflow_negate_pc_relative_partial_inplace_pcrel_offset: [0; 4],
                type_0: 2 as libc::c_int as libc::c_uint,
                src_mask: 0xffffffff as libc::c_uint as bfd_vma,
                dst_mask: 0xffffffff as libc::c_uint as bfd_vma,
                special_function: Some(
                    coff_amd64_reloc
                        as unsafe extern "C" fn(
                            *mut bfd,
                            *mut arelent,
                            *mut asymbol,
                            *mut libc::c_void,
                            *mut asection,
                            *mut bfd,
                            *mut *mut libc::c_char,
                        ) -> bfd_reloc_status_type,
                ),
                name: b"IMAGE_REL_AMD64_ADDR32\0" as *const u8 as *const libc::c_char,
            };
            init.set_size(
                (if (2 as libc::c_int) < 0 as libc::c_int {
                    -(2 as libc::c_int)
                } else {
                    2 as libc::c_int
                }) as libc::c_uint,
            );
            init.set_bitsize(32 as libc::c_int as libc::c_uint);
            init.set_rightshift(0 as libc::c_int as libc::c_uint);
            init.set_bitpos(0 as libc::c_int as libc::c_uint);
            init.set_complain_on_overflow(complain_overflow_bitfield);
            init.set_negate(
                ((2 as libc::c_int) < 0 as libc::c_int) as libc::c_int as libc::c_uint,
            );
            init.set_pc_relative(0 as libc::c_int as libc::c_uint);
            init.set_partial_inplace(1 as libc::c_int as libc::c_uint);
            init.set_pcrel_offset(1 as libc::c_int as libc::c_uint);
            init
        },
        {
            let mut init = reloc_howto_struct {
                size_bitsize_rightshift_bitpos_complain_on_overflow_negate_pc_relative_partial_inplace_pcrel_offset: [0; 4],
                type_0: 3 as libc::c_int as libc::c_uint,
                src_mask: 0xffffffff as libc::c_uint as bfd_vma,
                dst_mask: 0xffffffff as libc::c_uint as bfd_vma,
                special_function: Some(
                    coff_amd64_reloc
                        as unsafe extern "C" fn(
                            *mut bfd,
                            *mut arelent,
                            *mut asymbol,
                            *mut libc::c_void,
                            *mut asection,
                            *mut bfd,
                            *mut *mut libc::c_char,
                        ) -> bfd_reloc_status_type,
                ),
                name: b"IMAGE_REL_AMD64_ADDR32NB\0" as *const u8 as *const libc::c_char,
            };
            init.set_size(
                (if (2 as libc::c_int) < 0 as libc::c_int {
                    -(2 as libc::c_int)
                } else {
                    2 as libc::c_int
                }) as libc::c_uint,
            );
            init.set_bitsize(32 as libc::c_int as libc::c_uint);
            init.set_rightshift(0 as libc::c_int as libc::c_uint);
            init.set_bitpos(0 as libc::c_int as libc::c_uint);
            init.set_complain_on_overflow(complain_overflow_bitfield);
            init.set_negate(
                ((2 as libc::c_int) < 0 as libc::c_int) as libc::c_int as libc::c_uint,
            );
            init.set_pc_relative(0 as libc::c_int as libc::c_uint);
            init.set_partial_inplace(1 as libc::c_int as libc::c_uint);
            init.set_pcrel_offset(0 as libc::c_int as libc::c_uint);
            init
        },
        {
            let mut init = reloc_howto_struct {
                size_bitsize_rightshift_bitpos_complain_on_overflow_negate_pc_relative_partial_inplace_pcrel_offset: [0; 4],
                type_0: 4 as libc::c_int as libc::c_uint,
                src_mask: 0xffffffff as libc::c_uint as bfd_vma,
                dst_mask: 0xffffffff as libc::c_uint as bfd_vma,
                special_function: Some(
                    coff_amd64_reloc
                        as unsafe extern "C" fn(
                            *mut bfd,
                            *mut arelent,
                            *mut asymbol,
                            *mut libc::c_void,
                            *mut asection,
                            *mut bfd,
                            *mut *mut libc::c_char,
                        ) -> bfd_reloc_status_type,
                ),
                name: b"IMAGE_REL_AMD64_REL32\0" as *const u8 as *const libc::c_char,
            };
            init.set_size(
                (if (2 as libc::c_int) < 0 as libc::c_int {
                    -(2 as libc::c_int)
                } else {
                    2 as libc::c_int
                }) as libc::c_uint,
            );
            init.set_bitsize(32 as libc::c_int as libc::c_uint);
            init.set_rightshift(0 as libc::c_int as libc::c_uint);
            init.set_bitpos(0 as libc::c_int as libc::c_uint);
            init.set_complain_on_overflow(complain_overflow_signed);
            init.set_negate(
                ((2 as libc::c_int) < 0 as libc::c_int) as libc::c_int as libc::c_uint,
            );
            init.set_pc_relative(1 as libc::c_int as libc::c_uint);
            init.set_partial_inplace(1 as libc::c_int as libc::c_uint);
            init.set_pcrel_offset(1 as libc::c_int as libc::c_uint);
            init
        },
        {
            let mut init = reloc_howto_struct {
                size_bitsize_rightshift_bitpos_complain_on_overflow_negate_pc_relative_partial_inplace_pcrel_offset: [0; 4],
                type_0: 5 as libc::c_int as libc::c_uint,
                src_mask: 0xffffffff as libc::c_uint as bfd_vma,
                dst_mask: 0xffffffff as libc::c_uint as bfd_vma,
                special_function: Some(
                    coff_amd64_reloc
                        as unsafe extern "C" fn(
                            *mut bfd,
                            *mut arelent,
                            *mut asymbol,
                            *mut libc::c_void,
                            *mut asection,
                            *mut bfd,
                            *mut *mut libc::c_char,
                        ) -> bfd_reloc_status_type,
                ),
                name: b"IMAGE_REL_AMD64_REL32_1\0" as *const u8 as *const libc::c_char,
            };
            init.set_size(
                (if (2 as libc::c_int) < 0 as libc::c_int {
                    -(2 as libc::c_int)
                } else {
                    2 as libc::c_int
                }) as libc::c_uint,
            );
            init.set_bitsize(32 as libc::c_int as libc::c_uint);
            init.set_rightshift(0 as libc::c_int as libc::c_uint);
            init.set_bitpos(0 as libc::c_int as libc::c_uint);
            init.set_complain_on_overflow(complain_overflow_signed);
            init.set_negate(
                ((2 as libc::c_int) < 0 as libc::c_int) as libc::c_int as libc::c_uint,
            );
            init.set_pc_relative(1 as libc::c_int as libc::c_uint);
            init.set_partial_inplace(1 as libc::c_int as libc::c_uint);
            init.set_pcrel_offset(1 as libc::c_int as libc::c_uint);
            init
        },
        {
            let mut init = reloc_howto_struct {
                size_bitsize_rightshift_bitpos_complain_on_overflow_negate_pc_relative_partial_inplace_pcrel_offset: [0; 4],
                type_0: 6 as libc::c_int as libc::c_uint,
                src_mask: 0xffffffff as libc::c_uint as bfd_vma,
                dst_mask: 0xffffffff as libc::c_uint as bfd_vma,
                special_function: Some(
                    coff_amd64_reloc
                        as unsafe extern "C" fn(
                            *mut bfd,
                            *mut arelent,
                            *mut asymbol,
                            *mut libc::c_void,
                            *mut asection,
                            *mut bfd,
                            *mut *mut libc::c_char,
                        ) -> bfd_reloc_status_type,
                ),
                name: b"IMAGE_REL_AMD64_REL32_2\0" as *const u8 as *const libc::c_char,
            };
            init.set_size(
                (if (2 as libc::c_int) < 0 as libc::c_int {
                    -(2 as libc::c_int)
                } else {
                    2 as libc::c_int
                }) as libc::c_uint,
            );
            init.set_bitsize(32 as libc::c_int as libc::c_uint);
            init.set_rightshift(0 as libc::c_int as libc::c_uint);
            init.set_bitpos(0 as libc::c_int as libc::c_uint);
            init.set_complain_on_overflow(complain_overflow_signed);
            init.set_negate(
                ((2 as libc::c_int) < 0 as libc::c_int) as libc::c_int as libc::c_uint,
            );
            init.set_pc_relative(1 as libc::c_int as libc::c_uint);
            init.set_partial_inplace(1 as libc::c_int as libc::c_uint);
            init.set_pcrel_offset(1 as libc::c_int as libc::c_uint);
            init
        },
        {
            let mut init = reloc_howto_struct {
                size_bitsize_rightshift_bitpos_complain_on_overflow_negate_pc_relative_partial_inplace_pcrel_offset: [0; 4],
                type_0: 7 as libc::c_int as libc::c_uint,
                src_mask: 0xffffffff as libc::c_uint as bfd_vma,
                dst_mask: 0xffffffff as libc::c_uint as bfd_vma,
                special_function: Some(
                    coff_amd64_reloc
                        as unsafe extern "C" fn(
                            *mut bfd,
                            *mut arelent,
                            *mut asymbol,
                            *mut libc::c_void,
                            *mut asection,
                            *mut bfd,
                            *mut *mut libc::c_char,
                        ) -> bfd_reloc_status_type,
                ),
                name: b"IMAGE_REL_AMD64_REL32_3\0" as *const u8 as *const libc::c_char,
            };
            init.set_size(
                (if (2 as libc::c_int) < 0 as libc::c_int {
                    -(2 as libc::c_int)
                } else {
                    2 as libc::c_int
                }) as libc::c_uint,
            );
            init.set_bitsize(32 as libc::c_int as libc::c_uint);
            init.set_rightshift(0 as libc::c_int as libc::c_uint);
            init.set_bitpos(0 as libc::c_int as libc::c_uint);
            init.set_complain_on_overflow(complain_overflow_signed);
            init.set_negate(
                ((2 as libc::c_int) < 0 as libc::c_int) as libc::c_int as libc::c_uint,
            );
            init.set_pc_relative(1 as libc::c_int as libc::c_uint);
            init.set_partial_inplace(1 as libc::c_int as libc::c_uint);
            init.set_pcrel_offset(1 as libc::c_int as libc::c_uint);
            init
        },
        {
            let mut init = reloc_howto_struct {
                size_bitsize_rightshift_bitpos_complain_on_overflow_negate_pc_relative_partial_inplace_pcrel_offset: [0; 4],
                type_0: 8 as libc::c_int as libc::c_uint,
                src_mask: 0xffffffff as libc::c_uint as bfd_vma,
                dst_mask: 0xffffffff as libc::c_uint as bfd_vma,
                special_function: Some(
                    coff_amd64_reloc
                        as unsafe extern "C" fn(
                            *mut bfd,
                            *mut arelent,
                            *mut asymbol,
                            *mut libc::c_void,
                            *mut asection,
                            *mut bfd,
                            *mut *mut libc::c_char,
                        ) -> bfd_reloc_status_type,
                ),
                name: b"IMAGE_REL_AMD64_REL32_4\0" as *const u8 as *const libc::c_char,
            };
            init.set_size(
                (if (2 as libc::c_int) < 0 as libc::c_int {
                    -(2 as libc::c_int)
                } else {
                    2 as libc::c_int
                }) as libc::c_uint,
            );
            init.set_bitsize(32 as libc::c_int as libc::c_uint);
            init.set_rightshift(0 as libc::c_int as libc::c_uint);
            init.set_bitpos(0 as libc::c_int as libc::c_uint);
            init.set_complain_on_overflow(complain_overflow_signed);
            init.set_negate(
                ((2 as libc::c_int) < 0 as libc::c_int) as libc::c_int as libc::c_uint,
            );
            init.set_pc_relative(1 as libc::c_int as libc::c_uint);
            init.set_partial_inplace(1 as libc::c_int as libc::c_uint);
            init.set_pcrel_offset(1 as libc::c_int as libc::c_uint);
            init
        },
        {
            let mut init = reloc_howto_struct {
                size_bitsize_rightshift_bitpos_complain_on_overflow_negate_pc_relative_partial_inplace_pcrel_offset: [0; 4],
                type_0: 9 as libc::c_int as libc::c_uint,
                src_mask: 0xffffffff as libc::c_uint as bfd_vma,
                dst_mask: 0xffffffff as libc::c_uint as bfd_vma,
                special_function: Some(
                    coff_amd64_reloc
                        as unsafe extern "C" fn(
                            *mut bfd,
                            *mut arelent,
                            *mut asymbol,
                            *mut libc::c_void,
                            *mut asection,
                            *mut bfd,
                            *mut *mut libc::c_char,
                        ) -> bfd_reloc_status_type,
                ),
                name: b"IMAGE_REL_AMD64_REL32_5\0" as *const u8 as *const libc::c_char,
            };
            init.set_size(
                (if (2 as libc::c_int) < 0 as libc::c_int {
                    -(2 as libc::c_int)
                } else {
                    2 as libc::c_int
                }) as libc::c_uint,
            );
            init.set_bitsize(32 as libc::c_int as libc::c_uint);
            init.set_rightshift(0 as libc::c_int as libc::c_uint);
            init.set_bitpos(0 as libc::c_int as libc::c_uint);
            init.set_complain_on_overflow(complain_overflow_signed);
            init.set_negate(
                ((2 as libc::c_int) < 0 as libc::c_int) as libc::c_int as libc::c_uint,
            );
            init.set_pc_relative(1 as libc::c_int as libc::c_uint);
            init.set_partial_inplace(1 as libc::c_int as libc::c_uint);
            init.set_pcrel_offset(1 as libc::c_int as libc::c_uint);
            init
        },
        {
            let mut init = reloc_howto_struct {
                size_bitsize_rightshift_bitpos_complain_on_overflow_negate_pc_relative_partial_inplace_pcrel_offset: [0; 4],
                type_0: 10 as libc::c_int as libc::c_uint,
                src_mask: 0 as libc::c_int as bfd_vma,
                dst_mask: 0 as libc::c_int as bfd_vma,
                special_function: None,
                name: 0 as *const libc::c_char,
            };
            init.set_size(
                (if (0 as libc::c_int) < 0 as libc::c_int {
                    -(0 as libc::c_int)
                } else {
                    0 as libc::c_int
                }) as libc::c_uint,
            );
            init.set_bitsize(0 as libc::c_int as libc::c_uint);
            init.set_rightshift(0 as libc::c_int as libc::c_uint);
            init.set_bitpos(0 as libc::c_int as libc::c_uint);
            init.set_complain_on_overflow(complain_overflow_dont);
            init.set_negate(
                ((0 as libc::c_int) < 0 as libc::c_int) as libc::c_int as libc::c_uint,
            );
            init.set_pc_relative(0 as libc::c_int as libc::c_uint);
            init.set_partial_inplace(0 as libc::c_int as libc::c_uint);
            init.set_pcrel_offset(0 as libc::c_int as libc::c_uint);
            init
        },
        {
            let mut init = reloc_howto_struct {
                size_bitsize_rightshift_bitpos_complain_on_overflow_negate_pc_relative_partial_inplace_pcrel_offset: [0; 4],
                type_0: 11 as libc::c_int as libc::c_uint,
                src_mask: 0xffffffff as libc::c_uint as bfd_vma,
                dst_mask: 0xffffffff as libc::c_uint as bfd_vma,
                special_function: Some(
                    coff_amd64_reloc
                        as unsafe extern "C" fn(
                            *mut bfd,
                            *mut arelent,
                            *mut asymbol,
                            *mut libc::c_void,
                            *mut asection,
                            *mut bfd,
                            *mut *mut libc::c_char,
                        ) -> bfd_reloc_status_type,
                ),
                name: b"IMAGE_REL_AMD64_SECREL\0" as *const u8 as *const libc::c_char,
            };
            init.set_size(
                (if (2 as libc::c_int) < 0 as libc::c_int {
                    -(2 as libc::c_int)
                } else {
                    2 as libc::c_int
                }) as libc::c_uint,
            );
            init.set_bitsize(32 as libc::c_int as libc::c_uint);
            init.set_rightshift(0 as libc::c_int as libc::c_uint);
            init.set_bitpos(0 as libc::c_int as libc::c_uint);
            init.set_complain_on_overflow(complain_overflow_bitfield);
            init.set_negate(
                ((2 as libc::c_int) < 0 as libc::c_int) as libc::c_int as libc::c_uint,
            );
            init.set_pc_relative(0 as libc::c_int as libc::c_uint);
            init.set_partial_inplace(1 as libc::c_int as libc::c_uint);
            init.set_pcrel_offset(1 as libc::c_int as libc::c_uint);
            init
        },
        {
            let mut init = reloc_howto_struct {
                size_bitsize_rightshift_bitpos_complain_on_overflow_negate_pc_relative_partial_inplace_pcrel_offset: [0; 4],
                type_0: 12 as libc::c_int as libc::c_uint,
                src_mask: 0 as libc::c_int as bfd_vma,
                dst_mask: 0 as libc::c_int as bfd_vma,
                special_function: None,
                name: 0 as *const libc::c_char,
            };
            init.set_size(
                (if (0 as libc::c_int) < 0 as libc::c_int {
                    -(0 as libc::c_int)
                } else {
                    0 as libc::c_int
                }) as libc::c_uint,
            );
            init.set_bitsize(0 as libc::c_int as libc::c_uint);
            init.set_rightshift(0 as libc::c_int as libc::c_uint);
            init.set_bitpos(0 as libc::c_int as libc::c_uint);
            init.set_complain_on_overflow(complain_overflow_dont);
            init.set_negate(
                ((0 as libc::c_int) < 0 as libc::c_int) as libc::c_int as libc::c_uint,
            );
            init.set_pc_relative(0 as libc::c_int as libc::c_uint);
            init.set_partial_inplace(0 as libc::c_int as libc::c_uint);
            init.set_pcrel_offset(0 as libc::c_int as libc::c_uint);
            init
        },
        {
            let mut init = reloc_howto_struct {
                size_bitsize_rightshift_bitpos_complain_on_overflow_negate_pc_relative_partial_inplace_pcrel_offset: [0; 4],
                type_0: 13 as libc::c_int as libc::c_uint,
                src_mask: 0 as libc::c_int as bfd_vma,
                dst_mask: 0 as libc::c_int as bfd_vma,
                special_function: None,
                name: 0 as *const libc::c_char,
            };
            init.set_size(
                (if (0 as libc::c_int) < 0 as libc::c_int {
                    -(0 as libc::c_int)
                } else {
                    0 as libc::c_int
                }) as libc::c_uint,
            );
            init.set_bitsize(0 as libc::c_int as libc::c_uint);
            init.set_rightshift(0 as libc::c_int as libc::c_uint);
            init.set_bitpos(0 as libc::c_int as libc::c_uint);
            init.set_complain_on_overflow(complain_overflow_dont);
            init.set_negate(
                ((0 as libc::c_int) < 0 as libc::c_int) as libc::c_int as libc::c_uint,
            );
            init.set_pc_relative(0 as libc::c_int as libc::c_uint);
            init.set_partial_inplace(0 as libc::c_int as libc::c_uint);
            init.set_pcrel_offset(0 as libc::c_int as libc::c_uint);
            init
        },
        {
            let mut init = reloc_howto_struct {
                size_bitsize_rightshift_bitpos_complain_on_overflow_negate_pc_relative_partial_inplace_pcrel_offset: [0; 4],
                type_0: 14 as libc::c_int as libc::c_uint,
                src_mask: 0xffffffffffffffff as libc::c_ulonglong as bfd_vma,
                dst_mask: 0xffffffffffffffff as libc::c_ulonglong as bfd_vma,
                special_function: Some(
                    coff_amd64_reloc
                        as unsafe extern "C" fn(
                            *mut bfd,
                            *mut arelent,
                            *mut asymbol,
                            *mut libc::c_void,
                            *mut asection,
                            *mut bfd,
                            *mut *mut libc::c_char,
                        ) -> bfd_reloc_status_type,
                ),
                name: b"R_X86_64_PC64\0" as *const u8 as *const libc::c_char,
            };
            init.set_size(
                (if (4 as libc::c_int) < 0 as libc::c_int {
                    -(4 as libc::c_int)
                } else {
                    4 as libc::c_int
                }) as libc::c_uint,
            );
            init.set_bitsize(64 as libc::c_int as libc::c_uint);
            init.set_rightshift(0 as libc::c_int as libc::c_uint);
            init.set_bitpos(0 as libc::c_int as libc::c_uint);
            init.set_complain_on_overflow(complain_overflow_signed);
            init.set_negate(
                ((4 as libc::c_int) < 0 as libc::c_int) as libc::c_int as libc::c_uint,
            );
            init.set_pc_relative(1 as libc::c_int as libc::c_uint);
            init.set_partial_inplace(1 as libc::c_int as libc::c_uint);
            init.set_pcrel_offset(1 as libc::c_int as libc::c_uint);
            init
        },
        {
            let mut init = reloc_howto_struct {
                size_bitsize_rightshift_bitpos_complain_on_overflow_negate_pc_relative_partial_inplace_pcrel_offset: [0; 4],
                type_0: 15 as libc::c_int as libc::c_uint,
                src_mask: 0xff as libc::c_int as bfd_vma,
                dst_mask: 0xff as libc::c_int as bfd_vma,
                special_function: Some(
                    coff_amd64_reloc
                        as unsafe extern "C" fn(
                            *mut bfd,
                            *mut arelent,
                            *mut asymbol,
                            *mut libc::c_void,
                            *mut asection,
                            *mut bfd,
                            *mut *mut libc::c_char,
                        ) -> bfd_reloc_status_type,
                ),
                name: b"R_X86_64_8\0" as *const u8 as *const libc::c_char,
            };
            init.set_size(
                (if (0 as libc::c_int) < 0 as libc::c_int {
                    -(0 as libc::c_int)
                } else {
                    0 as libc::c_int
                }) as libc::c_uint,
            );
            init.set_bitsize(8 as libc::c_int as libc::c_uint);
            init.set_rightshift(0 as libc::c_int as libc::c_uint);
            init.set_bitpos(0 as libc::c_int as libc::c_uint);
            init.set_complain_on_overflow(complain_overflow_bitfield);
            init.set_negate(
                ((0 as libc::c_int) < 0 as libc::c_int) as libc::c_int as libc::c_uint,
            );
            init.set_pc_relative(0 as libc::c_int as libc::c_uint);
            init.set_partial_inplace(1 as libc::c_int as libc::c_uint);
            init.set_pcrel_offset(1 as libc::c_int as libc::c_uint);
            init
        },
        {
            let mut init = reloc_howto_struct {
                size_bitsize_rightshift_bitpos_complain_on_overflow_negate_pc_relative_partial_inplace_pcrel_offset: [0; 4],
                type_0: 16 as libc::c_int as libc::c_uint,
                src_mask: 0xffff as libc::c_int as bfd_vma,
                dst_mask: 0xffff as libc::c_int as bfd_vma,
                special_function: Some(
                    coff_amd64_reloc
                        as unsafe extern "C" fn(
                            *mut bfd,
                            *mut arelent,
                            *mut asymbol,
                            *mut libc::c_void,
                            *mut asection,
                            *mut bfd,
                            *mut *mut libc::c_char,
                        ) -> bfd_reloc_status_type,
                ),
                name: b"R_X86_64_16\0" as *const u8 as *const libc::c_char,
            };
            init.set_size(
                (if (1 as libc::c_int) < 0 as libc::c_int {
                    -(1 as libc::c_int)
                } else {
                    1 as libc::c_int
                }) as libc::c_uint,
            );
            init.set_bitsize(16 as libc::c_int as libc::c_uint);
            init.set_rightshift(0 as libc::c_int as libc::c_uint);
            init.set_bitpos(0 as libc::c_int as libc::c_uint);
            init.set_complain_on_overflow(complain_overflow_bitfield);
            init.set_negate(
                ((1 as libc::c_int) < 0 as libc::c_int) as libc::c_int as libc::c_uint,
            );
            init.set_pc_relative(0 as libc::c_int as libc::c_uint);
            init.set_partial_inplace(1 as libc::c_int as libc::c_uint);
            init.set_pcrel_offset(1 as libc::c_int as libc::c_uint);
            init
        },
        {
            let mut init = reloc_howto_struct {
                size_bitsize_rightshift_bitpos_complain_on_overflow_negate_pc_relative_partial_inplace_pcrel_offset: [0; 4],
                type_0: 17 as libc::c_int as libc::c_uint,
                src_mask: 0xffffffff as libc::c_uint as bfd_vma,
                dst_mask: 0xffffffff as libc::c_uint as bfd_vma,
                special_function: Some(
                    coff_amd64_reloc
                        as unsafe extern "C" fn(
                            *mut bfd,
                            *mut arelent,
                            *mut asymbol,
                            *mut libc::c_void,
                            *mut asection,
                            *mut bfd,
                            *mut *mut libc::c_char,
                        ) -> bfd_reloc_status_type,
                ),
                name: b"R_X86_64_32S\0" as *const u8 as *const libc::c_char,
            };
            init.set_size(
                (if (2 as libc::c_int) < 0 as libc::c_int {
                    -(2 as libc::c_int)
                } else {
                    2 as libc::c_int
                }) as libc::c_uint,
            );
            init.set_bitsize(32 as libc::c_int as libc::c_uint);
            init.set_rightshift(0 as libc::c_int as libc::c_uint);
            init.set_bitpos(0 as libc::c_int as libc::c_uint);
            init.set_complain_on_overflow(complain_overflow_bitfield);
            init.set_negate(
                ((2 as libc::c_int) < 0 as libc::c_int) as libc::c_int as libc::c_uint,
            );
            init.set_pc_relative(0 as libc::c_int as libc::c_uint);
            init.set_partial_inplace(1 as libc::c_int as libc::c_uint);
            init.set_pcrel_offset(1 as libc::c_int as libc::c_uint);
            init
        },
        {
            let mut init = reloc_howto_struct {
                size_bitsize_rightshift_bitpos_complain_on_overflow_negate_pc_relative_partial_inplace_pcrel_offset: [0; 4],
                type_0: 18 as libc::c_int as libc::c_uint,
                src_mask: 0xff as libc::c_int as bfd_vma,
                dst_mask: 0xff as libc::c_int as bfd_vma,
                special_function: Some(
                    coff_amd64_reloc
                        as unsafe extern "C" fn(
                            *mut bfd,
                            *mut arelent,
                            *mut asymbol,
                            *mut libc::c_void,
                            *mut asection,
                            *mut bfd,
                            *mut *mut libc::c_char,
                        ) -> bfd_reloc_status_type,
                ),
                name: b"R_X86_64_PC8\0" as *const u8 as *const libc::c_char,
            };
            init.set_size(
                (if (0 as libc::c_int) < 0 as libc::c_int {
                    -(0 as libc::c_int)
                } else {
                    0 as libc::c_int
                }) as libc::c_uint,
            );
            init.set_bitsize(8 as libc::c_int as libc::c_uint);
            init.set_rightshift(0 as libc::c_int as libc::c_uint);
            init.set_bitpos(0 as libc::c_int as libc::c_uint);
            init.set_complain_on_overflow(complain_overflow_signed);
            init.set_negate(
                ((0 as libc::c_int) < 0 as libc::c_int) as libc::c_int as libc::c_uint,
            );
            init.set_pc_relative(1 as libc::c_int as libc::c_uint);
            init.set_partial_inplace(1 as libc::c_int as libc::c_uint);
            init.set_pcrel_offset(1 as libc::c_int as libc::c_uint);
            init
        },
        {
            let mut init = reloc_howto_struct {
                size_bitsize_rightshift_bitpos_complain_on_overflow_negate_pc_relative_partial_inplace_pcrel_offset: [0; 4],
                type_0: 19 as libc::c_int as libc::c_uint,
                src_mask: 0xffff as libc::c_int as bfd_vma,
                dst_mask: 0xffff as libc::c_int as bfd_vma,
                special_function: Some(
                    coff_amd64_reloc
                        as unsafe extern "C" fn(
                            *mut bfd,
                            *mut arelent,
                            *mut asymbol,
                            *mut libc::c_void,
                            *mut asection,
                            *mut bfd,
                            *mut *mut libc::c_char,
                        ) -> bfd_reloc_status_type,
                ),
                name: b"R_X86_64_PC16\0" as *const u8 as *const libc::c_char,
            };
            init.set_size(
                (if (1 as libc::c_int) < 0 as libc::c_int {
                    -(1 as libc::c_int)
                } else {
                    1 as libc::c_int
                }) as libc::c_uint,
            );
            init.set_bitsize(16 as libc::c_int as libc::c_uint);
            init.set_rightshift(0 as libc::c_int as libc::c_uint);
            init.set_bitpos(0 as libc::c_int as libc::c_uint);
            init.set_complain_on_overflow(complain_overflow_signed);
            init.set_negate(
                ((1 as libc::c_int) < 0 as libc::c_int) as libc::c_int as libc::c_uint,
            );
            init.set_pc_relative(1 as libc::c_int as libc::c_uint);
            init.set_partial_inplace(1 as libc::c_int as libc::c_uint);
            init.set_pcrel_offset(1 as libc::c_int as libc::c_uint);
            init
        },
        {
            let mut init = reloc_howto_struct {
                size_bitsize_rightshift_bitpos_complain_on_overflow_negate_pc_relative_partial_inplace_pcrel_offset: [0; 4],
                type_0: 20 as libc::c_int as libc::c_uint,
                src_mask: 0xffffffff as libc::c_uint as bfd_vma,
                dst_mask: 0xffffffff as libc::c_uint as bfd_vma,
                special_function: Some(
                    coff_amd64_reloc
                        as unsafe extern "C" fn(
                            *mut bfd,
                            *mut arelent,
                            *mut asymbol,
                            *mut libc::c_void,
                            *mut asection,
                            *mut bfd,
                            *mut *mut libc::c_char,
                        ) -> bfd_reloc_status_type,
                ),
                name: b"R_X86_64_PC32\0" as *const u8 as *const libc::c_char,
            };
            init.set_size(
                (if (2 as libc::c_int) < 0 as libc::c_int {
                    -(2 as libc::c_int)
                } else {
                    2 as libc::c_int
                }) as libc::c_uint,
            );
            init.set_bitsize(32 as libc::c_int as libc::c_uint);
            init.set_rightshift(0 as libc::c_int as libc::c_uint);
            init.set_bitpos(0 as libc::c_int as libc::c_uint);
            init.set_complain_on_overflow(complain_overflow_signed);
            init.set_negate(
                ((2 as libc::c_int) < 0 as libc::c_int) as libc::c_int as libc::c_uint,
            );
            init.set_pc_relative(1 as libc::c_int as libc::c_uint);
            init.set_partial_inplace(1 as libc::c_int as libc::c_uint);
            init.set_pcrel_offset(1 as libc::c_int as libc::c_uint);
            init
        },
    ];
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
