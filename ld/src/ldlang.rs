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
    pub type ctf_dict;
    pub type ctf_archive_internal;
    pub type ctf_next;
    pub type cie;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn strtoul(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_ulong;
    fn free(_: *mut libc::c_void);
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
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
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
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strpbrk(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn bfd_hash_table_init_n(
        _: *mut bfd_hash_table,
        _: Option::<
            unsafe extern "C" fn(
                *mut bfd_hash_entry,
                *mut bfd_hash_table,
                *const libc::c_char,
            ) -> *mut bfd_hash_entry,
        >,
        _: libc::c_uint,
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
    fn bfd_record_phdr(
        _: *mut bfd,
        _: libc::c_ulong,
        _: bool,
        _: flagword,
        _: bool,
        _: bfd_vma,
        _: bool,
        _: bool,
        _: libc::c_uint,
        _: *mut *mut bfd_section,
    ) -> bool;
    fn bfd_openw(filename: *const libc::c_char, target: *const libc::c_char) -> *mut bfd;
    fn bfd_close(abfd: *mut bfd) -> bool;
    static mut _bfd_std_section: [asection; 4];
    fn bfd_get_section_by_name(
        abfd: *mut bfd,
        name: *const libc::c_char,
    ) -> *mut asection;
    fn bfd_get_section_by_name_if(
        abfd: *mut bfd,
        name: *const libc::c_char,
        func: Option::<
            unsafe extern "C" fn(*mut bfd, *mut asection, *mut libc::c_void) -> bool,
        >,
        obj: *mut libc::c_void,
    ) -> *mut asection;
    fn bfd_make_section_anyway_with_flags(
        abfd: *mut bfd,
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
    fn bfd_get_section_contents(
        abfd: *mut bfd,
        section: *mut asection,
        location: *mut libc::c_void,
        offset: file_ptr,
        count: bfd_size_type,
    ) -> bool;
    fn bfd_printable_name(abfd: *mut bfd) -> *const libc::c_char;
    fn bfd_arch_get_compatible(
        abfd: *const bfd,
        bbfd: *const bfd,
        accept_unknowns: bool,
    ) -> *const bfd_arch_info_type;
    fn bfd_get_arch(abfd: *const bfd) -> bfd_architecture;
    fn bfd_arch_bits_per_address(abfd: *const bfd) -> libc::c_uint;
    fn bfd_arch_mach_octets_per_byte(
        arch: bfd_architecture,
        machine: libc::c_ulong,
    ) -> libc::c_uint;
    fn bfd_get_reloc_size(_: *const reloc_howto_type) -> libc::c_uint;
    fn bfd_get_error() -> bfd_error_type;
    fn bfd_set_error_handler(_: bfd_error_handler_type) -> bfd_error_handler_type;
    fn bfd_set_start_address(abfd: *mut bfd, vma: bfd_vma) -> bool;
    fn bfd_set_gp_size(abfd: *mut bfd, i: libc::c_uint);
    fn bfd_scan_vma(
        string: *const libc::c_char,
        end: *mut *const libc::c_char,
        base: libc::c_int,
    ) -> bfd_vma;
    fn bfd_demangle(
        _: *mut bfd,
        _: *const libc::c_char,
        _: libc::c_int,
    ) -> *mut libc::c_char;
    fn bfd_openr_next_archived_file(archive: *mut bfd, previous: *mut bfd) -> *mut bfd;
    fn bfd_iterate_over_targets(
        func: Option::<
            unsafe extern "C" fn(*const bfd_target, *mut libc::c_void) -> libc::c_int,
        >,
        data: *mut libc::c_void,
    ) -> *const bfd_target;
    fn bfd_check_format(abfd: *mut bfd, format: bfd_format) -> bool;
    fn bfd_check_format_matches(
        abfd: *mut bfd,
        format: bfd_format,
        matching: *mut *mut *mut libc::c_char,
    ) -> bool;
    fn bfd_set_format(abfd: *mut bfd, format: bfd_format) -> bool;
    fn bfd_link_check_relocs(abfd: *mut bfd, info: *mut bfd_link_info) -> bool;
    fn lbasename(_: *const libc::c_char) -> *const libc::c_char;
    fn concat(_: *const libc::c_char, _: ...) -> *mut libc::c_char;
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    fn xrealloc(_: *mut libc::c_void, _: size_t) -> *mut libc::c_void;
    fn xcalloc(_: size_t, _: size_t) -> *mut libc::c_void;
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn htab_create(_: size_t, _: htab_hash, _: htab_eq, _: htab_del) -> htab_t;
    fn htab_find(_: htab_t, _: *const libc::c_void) -> *mut libc::c_void;
    fn htab_find_slot(
        _: htab_t,
        _: *const libc::c_void,
        _: insert_option,
    ) -> *mut *mut libc::c_void;
    fn htab_hash_string(_: *const libc::c_void) -> hashval_t;
    fn filename_cmp(s1: *const libc::c_char, s2: *const libc::c_char) -> libc::c_int;
    fn filename_ncmp(
        s1: *const libc::c_char,
        s2: *const libc::c_char,
        n: size_t,
    ) -> libc::c_int;
    static _sch_istable: [libc::c_ushort; 256];
    static _sch_tolower: [libc::c_uchar; 256];
    fn _obstack_newchunk(_: *mut obstack, _: size_t);
    fn _obstack_free(_: *mut obstack, _: *mut libc::c_void);
    fn _obstack_begin(
        _: *mut obstack,
        _: size_t,
        _: size_t,
        _: Option::<unsafe extern "C" fn(size_t) -> *mut libc::c_void>,
        _: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    ) -> libc::c_int;
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
    fn bfd_link_add_undef(_: *mut bfd_link_hash_table, _: *mut bfd_link_hash_entry);
    fn bfd_section_already_linked_table_init() -> bool;
    fn bfd_section_already_linked_table_free();
    fn ctf_bfdopen(_: *mut bfd, _: *mut libc::c_int) -> *mut ctf_archive_t;
    fn ctf_close(_: *mut ctf_archive_t);
    fn ctf_dict_close(_: *mut ctf_dict_t);
    fn ctf_errwarning_next(
        _: *mut ctf_dict_t,
        _: *mut *mut ctf_next_t,
        is_warning: *mut libc::c_int,
        errp: *mut libc::c_int,
    ) -> *mut libc::c_char;
    fn ctf_errno(_: *mut ctf_dict_t) -> libc::c_int;
    fn ctf_errmsg(_: libc::c_int) -> *const libc::c_char;
    fn ctf_create(_: *mut libc::c_int) -> *mut ctf_dict_t;
    fn ctf_link_add_ctf(
        _: *mut ctf_dict_t,
        _: *mut ctf_archive_t,
        _: *const libc::c_char,
    ) -> libc::c_int;
    fn ctf_link(_: *mut ctf_dict_t, flags: libc::c_int) -> libc::c_int;
    fn ctf_link_write(
        _: *mut ctf_dict_t,
        size: *mut size_t,
        threshold: size_t,
    ) -> *mut libc::c_uchar;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn dcngettext(
        __domainname: *const libc::c_char,
        __msgid1: *const libc::c_char,
        __msgid2: *const libc::c_char,
        __n: libc::c_ulong,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    static mut command_line: args_type;
    static mut config: ld_config_type;
    static mut saved_script_handle: *mut FILE;
    static mut sort_section: sort_type;
    fn yyparse() -> libc::c_int;
    fn ld_abort(_: *const libc::c_char, _: libc::c_int, _: *const libc::c_char) -> !;
    static mut ld_sysroot: *const libc::c_char;
    static mut previous_script_handle: *mut FILE;
    static mut default_target: *mut libc::c_char;
    static mut trace_files: libc::c_uint;
    static mut verbose: bool;
    static mut g_switch_value: libc::c_int;
    static mut output_filename: *const libc::c_char;
    static mut link_info: bfd_link_info;
    static mut expld: ldexp_control;
    fn exp_intop(_: bfd_vma) -> *mut etree_type;
    fn exp_fold_tree(_: *mut etree_type, _: *mut asection, _: *mut bfd_vma);
    fn exp_fold_tree_no_dot(_: *mut etree_type);
    fn exp_binop(
        _: libc::c_int,
        _: *mut etree_type,
        _: *mut etree_type,
    ) -> *mut etree_type;
    fn exp_nameop(_: libc::c_int, _: *const libc::c_char) -> *mut etree_type;
    fn exp_assign(
        _: *const libc::c_char,
        _: *mut etree_type,
        _: bool,
    ) -> *mut etree_type;
    fn exp_provide(
        _: *const libc::c_char,
        _: *mut etree_type,
        _: bool,
    ) -> *mut etree_type;
    fn exp_print_tree(_: *mut etree_type);
    fn exp_get_vma(_: *mut etree_type, _: bfd_vma, _: *mut libc::c_char) -> bfd_vma;
    fn exp_get_power(_: *mut etree_type, _: *mut libc::c_char) -> libc::c_int;
    fn exp_get_abs_int(
        _: *mut etree_type,
        _: libc::c_int,
        _: *mut libc::c_char,
    ) -> bfd_vma;
    fn ldexp_finalize_syms();
    static mut parser_input: input_type;
    fn einfo(_: *const libc::c_char, _: ...);
    fn minfo(_: *const libc::c_char, _: ...);
    fn info_msg(_: *const libc::c_char, _: ...);
    fn info_assert(_: *const libc::c_char, _: libc::c_uint);
    fn print_space();
    fn print_nl();
    static mut constructor_list: lang_statement_list_type;
    static mut constructors_sorted: bool;
    fn ldctor_build_sets();
    static mut ldfile_assumed_script: bool;
    static mut ldfile_output_machine: libc::c_ulong;
    static mut ldfile_output_architecture: bfd_architecture;
    fn ldfile_open_command_file(name: *const libc::c_char);
    fn ldfile_open_file(_: *mut lang_input_statement_struct);
    fn ldemul_after_open();
    fn ldemul_after_check_relocs();
    fn ldemul_before_place_orphans();
    fn ldemul_after_allocation();
    fn ldemul_before_allocation();
    fn ldemul_set_output_arch();
    fn ldemul_finish();
    fn ldemul_create_output_section_statements();
    fn ldemul_place_orphan(
        _: *mut asection,
        _: *const libc::c_char,
        _: libc::c_int,
    ) -> *mut lang_output_section_statement_type;
    fn ldemul_unrecognized_file(_: *mut lang_input_statement_struct) -> bool;
    fn ldemul_recognized_file(_: *mut lang_input_statement_struct) -> bool;
    fn ldemul_new_vers_pattern(
        _: *mut bfd_elf_version_expr,
    ) -> *mut bfd_elf_version_expr;
    fn ldemul_extra_map_file_text(_: *mut bfd, _: *mut bfd_link_info, _: *mut FILE);
    fn ldemul_emit_ctf_early() -> libc::c_int;
    fn ldemul_acquire_strings_for_ctf(_: *mut ctf_dict, _: *mut elf_strtab_hash);
    fn ldemul_new_dynsym_for_ctf(
        _: *mut ctf_dict,
        symidx: libc::c_int,
        _: *mut elf_internal_sym,
    );
    fn ldemul_print_symbol(
        hash_entry: *mut bfd_link_hash_entry,
        ptr: *mut libc::c_void,
    ) -> bool;
    fn fnmatch(
        __pattern: *const libc::c_char,
        __string: *const libc::c_char,
        __flags: libc::c_int,
    ) -> libc::c_int;
    static mut current_demangling_style: demangling_styles;
    fn cplus_demangle_set_style(style: demangling_styles) -> demangling_styles;
    fn plugin_error_plugin() -> *const libc::c_char;
    fn plugin_call_all_symbols_read() -> libc::c_int;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
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
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
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
pub type bfd_error_handler_type = Option::<
    unsafe extern "C" fn(*const libc::c_char, *mut __va_list_tag) -> (),
>;
pub type ptrdiff_t = libc::c_long;
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
pub const _sch_iscntrl: C2RustUnnamed_24 = 2;
pub const _sch_isblank: C2RustUnnamed_24 = 1;
pub type notice_asneeded_action = libc::c_uint;
pub const notice_needed: notice_asneeded_action = 2;
pub const notice_not_needed: notice_asneeded_action = 1;
pub const notice_as_needed: notice_asneeded_action = 0;
pub type ctf_dict_t = ctf_dict;
pub type ctf_archive_t = ctf_archive_internal;
pub type C2RustUnnamed_25 = libc::c_uint;
pub const ECTF_NONAME: C2RustUnnamed_25 = 1058;
pub const ECTF_INCOMPLETE: C2RustUnnamed_25 = 1057;
pub const ECTF_NEEDSBFD: C2RustUnnamed_25 = 1056;
pub const ECTF_FLAGS: C2RustUnnamed_25 = 1055;
pub const ECTF_NEXT_WRONGFP: C2RustUnnamed_25 = 1054;
pub const ECTF_NEXT_WRONGFUN: C2RustUnnamed_25 = 1053;
pub const ECTF_NEXT_END: C2RustUnnamed_25 = 1052;
pub const ECTF_NONREPRESENTABLE: C2RustUnnamed_25 = 1051;
pub const ECTF_INTERNAL: C2RustUnnamed_25 = 1050;
pub const ECTF_NOTYET: C2RustUnnamed_25 = 1049;
pub const ECTF_DUMPSECTCHANGED: C2RustUnnamed_25 = 1048;
pub const ECTF_DUMPSECTUNKNOWN: C2RustUnnamed_25 = 1047;
pub const ECTF_SLICEOVERFLOW: C2RustUnnamed_25 = 1046;
pub const ECTF_ARNNAME: C2RustUnnamed_25 = 1045;
pub const ECTF_ARCREATE: C2RustUnnamed_25 = 1044;
pub const ECTF_COMPRESS: C2RustUnnamed_25 = 1043;
pub const ECTF_OVERROLLBACK: C2RustUnnamed_25 = 1042;
pub const ECTF_CONFLICT: C2RustUnnamed_25 = 1041;
pub const ECTF_DUPLICATE: C2RustUnnamed_25 = 1040;
pub const ECTF_FULL: C2RustUnnamed_25 = 1039;
pub const ECTF_DTFULL: C2RustUnnamed_25 = 1038;
pub const ECTF_RDONLY: C2RustUnnamed_25 = 1037;
pub const ECTF_NOMEMBNAM: C2RustUnnamed_25 = 1036;
pub const ECTF_NOENUMNAM: C2RustUnnamed_25 = 1035;
pub const ECTF_NOTSUP: C2RustUnnamed_25 = 1034;
pub const ECTF_NOLABELDATA: C2RustUnnamed_25 = 1033;
pub const ECTF_NOLABEL: C2RustUnnamed_25 = 1032;
pub const ECTF_NOTYPEDAT: C2RustUnnamed_25 = 1031;
pub const ECTF_NOTDATA: C2RustUnnamed_25 = 1030;
pub const ECTF_NOFUNCDAT: C2RustUnnamed_25 = 1029;
pub const ECTF_NOTFUNC: C2RustUnnamed_25 = 1028;
pub const ECTF_SYNTAX: C2RustUnnamed_25 = 1027;
pub const ECTF_NOTYPE: C2RustUnnamed_25 = 1026;
pub const ECTF_NAMELEN: C2RustUnnamed_25 = 1025;
pub const ECTF_NOTREF: C2RustUnnamed_25 = 1024;
pub const ECTF_NOTARRAY: C2RustUnnamed_25 = 1023;
pub const ECTF_NOTINTFP: C2RustUnnamed_25 = 1022;
pub const ECTF_NOTSUE: C2RustUnnamed_25 = 1021;
pub const ECTF_NOTENUM: C2RustUnnamed_25 = 1020;
pub const ECTF_NOTSOU: C2RustUnnamed_25 = 1019;
pub const ECTF_BADID: C2RustUnnamed_25 = 1018;
pub const ECTF_BADNAME: C2RustUnnamed_25 = 1017;
pub const ECTF_STRTAB: C2RustUnnamed_25 = 1016;
pub const ECTF_DECOMPRESS: C2RustUnnamed_25 = 1015;
pub const ECTF_ZALLOC: C2RustUnnamed_25 = 1014;
pub const ECTF_LINKADDEDLATE: C2RustUnnamed_25 = 1013;
pub const ECTF_DMODEL: C2RustUnnamed_25 = 1012;
pub const ECTF_NOPARENT: C2RustUnnamed_25 = 1011;
pub const ECTF_NOSYMTAB: C2RustUnnamed_25 = 1010;
pub const ECTF_NOCTFBUF: C2RustUnnamed_25 = 1009;
pub const ECTF_NOCTFDATA: C2RustUnnamed_25 = 1008;
pub const ECTF_CORRUPT: C2RustUnnamed_25 = 1007;
pub const ECTF_STRBAD: C2RustUnnamed_25 = 1006;
pub const ECTF_SYMBAD: C2RustUnnamed_25 = 1005;
pub const ECTF_SYMTAB: C2RustUnnamed_25 = 1004;
pub const ECTF_BFD_AMBIGUOUS: C2RustUnnamed_25 = 1003;
pub const ECTF_CTFVERS: C2RustUnnamed_25 = 1002;
pub const ECTF_BFDERR: C2RustUnnamed_25 = 1001;
pub const ECTF_FMT: C2RustUnnamed_25 = 1000;
pub type ctf_next_t = ctf_next;
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
pub type endian_enum = libc::c_uint;
pub const ENDIAN_LITTLE: endian_enum = 2;
pub const ENDIAN_BIG: endian_enum = 1;
pub const ENDIAN_UNSET: endian_enum = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct args_type {
    pub force_common_definition: bool,
    pub embedded_relocs: bool,
    pub force_exe_suffix: bool,
    pub cref: bool,
    pub warn_mismatch: bool,
    pub warn_search_mismatch: bool,
    pub check_section_addresses: libc::c_schar,
    pub accept_unknown_input_arch: bool,
    pub out_implib_filename: *mut libc::c_char,
    pub print_output_format: bool,
    pub print_memory_usage: bool,
    pub force_group_allocation: bool,
    pub endian: endian_enum,
    pub interpreter: *mut libc::c_char,
    pub soname: *mut libc::c_char,
    pub rpath: *mut libc::c_char,
    pub rpath_link: *mut libc::c_char,
    pub filter_shlib: *mut libc::c_char,
    pub auxiliary_filters: *mut *mut libc::c_char,
    pub version_exports_section: *mut libc::c_char,
    pub default_script: *mut libc::c_char,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct etree_value_type {
    pub value: bfd_vma,
    pub str_0: *mut libc::c_char,
    pub section: *mut asection,
    pub valid_p: bool,
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
    pub binary: C2RustUnnamed_33,
    pub trinary: C2RustUnnamed_32,
    pub assign: C2RustUnnamed_31,
    pub unary: C2RustUnnamed_30,
    pub name: C2RustUnnamed_29,
    pub value: C2RustUnnamed_28,
    pub rel: C2RustUnnamed_27,
    pub assert_s: C2RustUnnamed_26,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_26 {
    pub type_0: node_type,
    pub child: *mut etree_union,
    pub message: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_27 {
    pub type_0: node_type,
    pub section: *mut asection,
    pub value: bfd_vma,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_28 {
    pub type_0: node_type,
    pub value: bfd_vma,
    pub str_0: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_29 {
    pub type_0: node_type,
    pub name: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_30 {
    pub type_0: node_type,
    pub child: *mut etree_union,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_31 {
    pub type_0: node_type,
    pub dst: *const libc::c_char,
    pub src: *mut etree_union,
    pub hidden: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_32 {
    pub type_0: node_type,
    pub cond: *mut etree_union,
    pub lhs: *mut etree_union,
    pub rhs: *mut etree_union,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_33 {
    pub type_0: node_type,
    pub lhs: *mut etree_union,
    pub rhs: *mut etree_union,
}
pub type etree_type = etree_union;
pub type lang_phase_type = libc::c_uint;
pub const lang_fixed_phase_enum: lang_phase_type = 5;
pub const lang_final_phase_enum: lang_phase_type = 4;
pub const lang_assigning_phase_enum: lang_phase_type = 3;
pub const lang_allocating_phase_enum: lang_phase_type = 2;
pub const lang_mark_phase_enum: lang_phase_type = 1;
pub const lang_first_phase_enum: lang_phase_type = 0;
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
pub type phase_enum = libc::c_uint;
pub const exp_seg_done: phase_enum = 6;
pub const exp_seg_adjust: phase_enum = 5;
pub const exp_seg_relro_adjust: phase_enum = 4;
pub const exp_seg_end_seen: phase_enum = 3;
pub const exp_seg_relro_seen: phase_enum = 2;
pub const exp_seg_align_seen: phase_enum = 1;
pub const exp_seg_none: phase_enum = 0;
pub type relro_enum = libc::c_uint;
pub const exp_seg_relro_end: relro_enum = 2;
pub const exp_seg_relro_start: relro_enum = 1;
pub const exp_seg_relro_none: relro_enum = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct seg_align_type {
    pub phase: phase_enum,
    pub base: bfd_vma,
    pub relro_offset: bfd_vma,
    pub relro_end: bfd_vma,
    pub end: bfd_vma,
    pub pagesize: bfd_vma,
    pub maxpagesize: bfd_vma,
    pub relro: relro_enum,
    pub relro_start_stat: *mut lang_statement_union,
    pub relro_end_stat: *mut lang_statement_union,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ldexp_control {
    pub phase: lang_phase_type,
    pub assigning_to_dot: bool,
    pub rel_from_abs: bool,
    pub assign_name: *const libc::c_char,
    pub assign_src: *mut bfd_link_hash_entry,
    pub result: etree_value_type,
    pub dot: bfd_vma,
    pub dotp: *mut bfd_vma,
    pub section: *mut asection,
    pub dataseg: seg_align_type,
}
pub type lang_input_file_enum_type = libc::c_uint;
pub const lang_input_file_is_file_enum: lang_input_file_enum_type = 5;
pub const lang_input_file_is_search_file_enum: lang_input_file_enum_type = 4;
pub const lang_input_file_is_fake_enum: lang_input_file_enum_type = 3;
pub const lang_input_file_is_marker_enum: lang_input_file_enum_type = 2;
pub const lang_input_file_is_symbols_only_enum: lang_input_file_enum_type = 1;
pub const lang_input_file_is_l_enum: lang_input_file_enum_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct map_symbol_def {
    pub entry: *mut bfd_link_hash_entry,
    pub next: *mut map_symbol_def,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct input_section_userdata_struct {
    pub map_symbol_def_head: *mut map_symbol_def,
    pub map_symbol_def_tail: *mut *mut map_symbol_def,
    pub map_symbol_def_count: libc::c_ulong,
}
pub type input_section_userdata_type = input_section_userdata_struct;
pub type lang_match_sec_type_func = Option::<
    unsafe extern "C" fn(*mut bfd, *const asection, *mut bfd, *const asection) -> bool,
>;
pub type lang_statement_union_type = lang_statement_union;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lang_phdr {
    pub next: *mut lang_phdr,
    pub name: *const libc::c_char,
    pub type_0: libc::c_ulong,
    pub filehdr: bool,
    pub phdrs: bool,
    pub at: *mut etree_type,
    pub flags: *mut etree_type,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lang_nocrossref {
    pub next: *mut lang_nocrossref,
    pub name: *const libc::c_char,
}
pub type lang_nocrossref_type = lang_nocrossref;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lang_nocrossrefs {
    pub next: *mut lang_nocrossrefs,
    pub list: *mut lang_nocrossref_type,
    pub onlyfirst: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct unique_sections {
    pub next: *mut unique_sections,
    pub name: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct orphan_save {
    pub name: *const libc::c_char,
    pub flags: flagword,
    pub os: *mut lang_output_section_statement_type,
    pub section: *mut *mut asection,
    pub stmt: *mut *mut lang_statement_union_type,
    pub os_tail: *mut *mut lang_output_section_statement_type,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct asneeded_minfo {
    pub next: *mut asneeded_minfo,
    pub soname: *const libc::c_char,
    pub ref_0: *mut bfd,
    pub name: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct out_section_hash_entry {
    pub root: bfd_hash_entry,
    pub s: lang_statement_union_type,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct require_defined_symbol {
    pub name: *const libc::c_char,
    pub next: *mut require_defined_symbol,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct check_sec {
    pub sec: *mut asection,
    pub warned: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct excluded_lib {
    pub name: *mut libc::c_char,
    pub next: *mut excluded_lib,
}
pub type input_type = input_enum;
pub type input_enum = libc::c_uint;
pub const input_defsym: input_enum = 5;
pub const input_dynamic_list: input_enum = 4;
pub const input_version_script: input_enum = 3;
pub const input_mri_script: input_enum = 2;
pub const input_script: input_enum = 1;
pub const input_selected: input_enum = 0;
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
    pub tc_data: C2RustUnnamed_34,
    pub version: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_34 {
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
    pub d_un: C2RustUnnamed_35,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_35 {
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
pub type elf_target_os = libc::c_uint;
pub const is_nacl: elf_target_os = 3;
pub const is_vxworks: elf_target_os = 2;
pub const is_solaris: elf_target_os = 1;
pub const is_normal: elf_target_os = 0;
pub type ldlang_undef_chain_list_type = bfd_sym_chain;
pub type demangling_styles = libc::c_int;
pub const rust_demangling: demangling_styles = 131072;
pub const dlang_demangling: demangling_styles = 65536;
pub const gnat_demangling: demangling_styles = 32768;
pub const java_demangling: demangling_styles = 4;
pub const gnu_v3_demangling: demangling_styles = 16384;
pub const auto_demangling: demangling_styles = 256;
pub const unknown_demangling: demangling_styles = 0;
pub const no_demangling: demangling_styles = -1;
pub type open_bfd_mode = libc::c_uint;
pub const OPEN_BFD_RESCAN: open_bfd_mode = 2;
pub const OPEN_BFD_FORCE: open_bfd_mode = 1;
pub const OPEN_BFD_NORMAL: open_bfd_mode = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct section_iterator_callback_data {
    pub found_section: *mut asection,
    pub multiple_sections_found: bool,
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
    pub group: C2RustUnnamed_40,
    pub sec_group: *mut asection,
    pub next_in_group: *mut asection,
    pub fde_list: *mut eh_cie_fde,
    pub eh_frame_entry: *mut asection,
    pub has_secondary_relocs: bool,
    pub sec_info: *mut libc::c_void,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct eh_cie_fde {
    pub u: C2RustUnnamed_36,
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
pub union C2RustUnnamed_36 {
    pub fde: C2RustUnnamed_39,
    pub cie: C2RustUnnamed_37,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2RustUnnamed_37 {
    pub u: C2RustUnnamed_38,
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
pub union C2RustUnnamed_38 {
    pub full_cie: *mut cie,
    pub merged_with: *mut eh_cie_fde,
    pub sec: *mut asection,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_39 {
    pub cie_inf: *mut eh_cie_fde,
    pub next_for_section: *mut eh_cie_fde,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_40 {
    pub name: *const libc::c_char,
    pub id: *mut bfd_symbol,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct overlay_list {
    pub next: *mut overlay_list,
    pub os: *mut lang_output_section_statement_type,
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
unsafe extern "C" fn bfd_section_alignment(mut sec: *const asection) -> libc::c_uint {
    return (*sec).alignment_power;
}
#[inline]
unsafe extern "C" fn bfd_section_userdata(
    mut sec: *const asection,
) -> *mut libc::c_void {
    return (*sec).userdata;
}
#[inline]
unsafe extern "C" fn bfd_set_section_userdata(
    mut sec: *mut asection,
    mut val: *mut libc::c_void,
) -> bool {
    (*sec).userdata = val;
    return 1 as libc::c_int != 0;
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
unsafe extern "C" fn bfd_is_abs_section(mut sec: *const asection) -> bool {
    return sec
        == &mut *_bfd_std_section.as_mut_ptr().offset(2 as libc::c_int as isize)
            as *mut asection as *const asection;
}
#[inline]
unsafe extern "C" fn bfd_is_const_section(mut sec: *const asection) -> bool {
    return sec >= _bfd_std_section.as_mut_ptr() as *const asection
        && sec
            < _bfd_std_section
                .as_mut_ptr()
                .offset(
                    (::core::mem::size_of::<[asection; 4]>() as libc::c_ulong)
                        .wrapping_div(
                            ::core::mem::size_of::<asection>() as libc::c_ulong,
                        ) as isize,
                ) as *const asection;
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
unsafe extern "C" fn bfd_count_sections(mut abfd: *const bfd) -> libc::c_uint {
    return (*abfd).section_count;
}
#[inline]
unsafe extern "C" fn bfd_usrdata(mut abfd: *const bfd) -> *mut libc::c_void {
    return (*abfd).usrdata;
}
#[inline]
unsafe extern "C" fn bfd_set_usrdata(mut abfd: *mut bfd, mut val: *mut libc::c_void) {
    (*abfd).usrdata = val;
}
#[inline]
unsafe extern "C" fn bfd_section_list_remove(mut abfd: *mut bfd, mut s: *mut asection) {
    let mut next: *mut asection = (*s).next;
    let mut prev: *mut asection = (*s).prev;
    if !prev.is_null() {
        (*prev).next = next;
    } else {
        (*abfd).sections = next;
    }
    if !next.is_null() {
        (*next).prev = prev;
    } else {
        (*abfd).section_last = prev;
    };
}
#[inline]
unsafe extern "C" fn bfd_section_list_append(mut abfd: *mut bfd, mut s: *mut asection) {
    (*s).next = 0 as *mut bfd_section;
    if !((*abfd).section_last).is_null() {
        (*s).prev = (*abfd).section_last;
        (*(*abfd).section_last).next = s;
    } else {
        (*s).prev = 0 as *mut bfd_section;
        (*abfd).sections = s;
    }
    (*abfd).section_last = s;
}
#[inline]
unsafe extern "C" fn bfd_section_list_prepend(mut abfd: *mut bfd, mut s: *mut asection) {
    (*s).prev = 0 as *mut bfd_section;
    if !((*abfd).sections).is_null() {
        (*s).next = (*abfd).sections;
        (*(*abfd).sections).prev = s;
    } else {
        (*s).next = 0 as *mut bfd_section;
        (*abfd).section_last = s;
    }
    (*abfd).sections = s;
}
#[inline]
unsafe extern "C" fn bfd_section_list_insert_after(
    mut abfd: *mut bfd,
    mut a: *mut asection,
    mut s: *mut asection,
) {
    let mut next: *mut asection = (*a).next;
    (*s).next = next;
    (*s).prev = a;
    (*a).next = s;
    if !next.is_null() {
        (*next).prev = s;
    } else {
        (*abfd).section_last = s;
    };
}
#[inline]
unsafe extern "C" fn bfd_section_list_insert_before(
    mut abfd: *mut bfd,
    mut b: *mut asection,
    mut s: *mut asection,
) {
    let mut prev: *mut asection = (*b).prev;
    (*s).prev = prev;
    (*s).next = b;
    (*b).prev = s;
    if !prev.is_null() {
        (*prev).next = s;
    } else {
        (*abfd).sections = s;
    };
}
#[inline]
unsafe extern "C" fn bfd_section_removed_from_list(
    mut abfd: *const bfd,
    mut s: *const asection,
) -> bool {
    return if !((*s).next).is_null() {
        ((*(*s).next).prev != s as *mut bfd_section) as libc::c_int
    } else {
        ((*abfd).section_last != s as *mut bfd_section) as libc::c_int
    } != 0;
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
unsafe extern "C" fn bfd_get_symbol_leading_char(mut abfd: *const bfd) -> libc::c_char {
    return (*(*abfd).xvec).symbol_leading_char;
}
#[no_mangle]
pub static mut lang_phdr_list: *mut lang_phdr = 0 as *const lang_phdr as *mut lang_phdr;
#[no_mangle]
pub static mut nocrossref_list: *mut lang_nocrossrefs = 0 as *const lang_nocrossrefs
    as *mut lang_nocrossrefs;
#[no_mangle]
pub static mut output_target: *const libc::c_char = 0 as *const libc::c_char;
#[no_mangle]
pub static mut abs_output_section: *mut lang_output_section_statement_type = 0
    as *const lang_output_section_statement_type
    as *mut lang_output_section_statement_type;
#[no_mangle]
pub static mut lang_os_list: lang_statement_list_type = lang_statement_list_type {
    head: 0 as *const lang_statement_union as *mut lang_statement_union,
    tail: 0 as *const *mut lang_statement_union as *mut *mut lang_statement_union,
};
#[no_mangle]
pub static mut input_flags: lang_input_statement_flags = lang_input_statement_flags {
    maybe_archive_full_name_provided_search_dirs_sysrooted_just_syms_dynamic_add_DT_NEEDED_for_dynamic_add_DT_NEEDED_for_regular_whole_archive_loaded_real_missing_file_reload_claimed_claim_archive_lto_output: [0; 2],
    c2rust_padding: [0; 6],
    pushed: 0 as *const lang_input_statement_flags as *mut lang_input_statement_flags,
};
#[no_mangle]
pub static mut lang_has_input_file: bool = 0 as libc::c_int != 0;
#[no_mangle]
pub static mut statement_list: lang_statement_list_type = lang_statement_list_type {
    head: 0 as *const lang_statement_union as *mut lang_statement_union,
    tail: 0 as *const *mut lang_statement_union as *mut *mut lang_statement_union,
};
#[no_mangle]
pub static mut stat_ptr: *mut lang_statement_list_type = unsafe {
    &statement_list as *const lang_statement_list_type as *mut lang_statement_list_type
};
#[no_mangle]
pub static mut delete_output_file_on_failure: bool = 0 as libc::c_int != 0;
#[no_mangle]
pub static mut entry_symbol: bfd_sym_chain = {
    let mut init = bfd_sym_chain {
        next: 0 as *const bfd_sym_chain as *mut bfd_sym_chain,
        name: 0 as *const libc::c_char,
    };
    init
};
#[no_mangle]
pub static mut entry_section: *const libc::c_char = b".text\0" as *const u8
    as *const libc::c_char;
#[no_mangle]
pub static mut entry_from_cmdline: bool = false;
#[no_mangle]
pub static mut file_chain: lang_statement_list_type = {
    let mut init = statement_list {
        head: 0 as *const lang_statement_union as *mut lang_statement_union,
        tail: 0 as *const *mut lang_statement_union as *mut *mut lang_statement_union,
    };
    init
};
#[no_mangle]
pub static mut input_file_chain: lang_statement_list_type = lang_statement_list_type {
    head: 0 as *const lang_statement_union as *mut lang_statement_union,
    tail: 0 as *const *mut lang_statement_union as *mut *mut lang_statement_union,
};
#[no_mangle]
pub static mut current_dynamic_list_p: *mut *mut bfd_elf_dynamic_list = 0
    as *const *mut bfd_elf_dynamic_list as *mut *mut bfd_elf_dynamic_list;
#[no_mangle]
pub static mut lang_statement_iteration: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut asneeded_list_tail: *mut *mut asneeded_minfo = 0
    as *const *mut asneeded_minfo as *mut *mut asneeded_minfo;
#[no_mangle]
pub unsafe extern "C" fn lang_init() {
    _obstack_begin(
        &mut stat_obstack,
        1000 as libc::c_int as size_t,
        0 as libc::c_int as size_t,
        Some(xmalloc as unsafe extern "C" fn(size_t) -> *mut libc::c_void),
        Some(free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
    );
    stat_ptr = &mut statement_list;
    output_section_statement_table_init();
    lang_list_init(stat_ptr);
    lang_list_init(&mut input_file_chain);
    lang_list_init(&mut lang_os_list);
    lang_list_init(&mut file_chain);
    first_file = lang_add_input_file(
        0 as *const libc::c_char,
        lang_input_file_is_marker_enum,
        0 as *const libc::c_char,
    );
    abs_output_section = lang_output_section_statement_lookup(
        b"*ABS*\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        1 as libc::c_int,
    );
    (*abs_output_section)
        .bfd_section = &mut *_bfd_std_section
        .as_mut_ptr()
        .offset(2 as libc::c_int as isize) as *mut asection;
    asneeded_list_head = 0 as *mut asneeded_minfo;
    asneeded_list_tail = &mut asneeded_list_head;
}
#[no_mangle]
pub unsafe extern "C" fn lang_finish() {
    output_section_statement_table_free();
}
#[no_mangle]
pub unsafe extern "C" fn lang_memory_region_lookup(
    name: *const libc::c_char,
    mut create: bool,
) -> *mut lang_memory_region_type {
    let mut n: *mut lang_memory_region_name = 0 as *mut lang_memory_region_name;
    let mut r: *mut lang_memory_region_type = 0 as *mut lang_memory_region_type;
    let mut new_region: *mut lang_memory_region_type = 0 as *mut lang_memory_region_type;
    if name.is_null() {
        return 0 as *mut lang_memory_region_type;
    }
    r = lang_memory_region_list;
    while !r.is_null() {
        n = &mut (*r).name_list;
        while !n.is_null() {
            if strcmp((*n).name, name) == 0 as libc::c_int {
                if create {
                    einfo(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"%P:%pS: warning: redeclaration of memory region `%s'\n\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        0 as *mut libc::c_void,
                        name,
                    );
                }
                return r;
            }
            n = (*n).next;
        }
        r = (*r).next;
    }
    if !create && strcmp(name, b"*default*\0" as *const u8 as *const libc::c_char) != 0 {
        einfo(
            dcgettext(
                0 as *const libc::c_char,
                b"%P:%pS: warning: memory region `%s' not declared\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            0 as *mut libc::c_void,
            name,
        );
    }
    new_region = stat_alloc(
        ::core::mem::size_of::<lang_memory_region_type>() as libc::c_ulong,
    ) as *mut lang_memory_region_type;
    (*new_region).name_list.name = xstrdup(name);
    (*new_region).name_list.next = 0 as *mut memory_region_name_struct;
    (*new_region).next = 0 as *mut memory_region_struct;
    (*new_region).origin_exp = 0 as *mut etree_union;
    (*new_region).origin = 0 as libc::c_int as bfd_vma;
    (*new_region).length_exp = 0 as *mut etree_union;
    (*new_region).length = !(0 as libc::c_int as bfd_size_type);
    (*new_region).current = 0 as libc::c_int as bfd_vma;
    (*new_region).last_os = 0 as *mut lang_statement_union;
    (*new_region).flags = 0 as libc::c_int as flagword;
    (*new_region).not_flags = 0 as libc::c_int as flagword;
    (*new_region).had_full_message = 0 as libc::c_int != 0;
    *lang_memory_region_list_tail = new_region;
    lang_memory_region_list_tail = &mut (*new_region).next;
    return new_region;
}
#[no_mangle]
pub unsafe extern "C" fn lang_memory_region_alias(
    mut alias: *const libc::c_char,
    mut region_name: *const libc::c_char,
) {
    let mut n: *mut lang_memory_region_name = 0 as *mut lang_memory_region_name;
    let mut r: *mut lang_memory_region_type = 0 as *mut lang_memory_region_type;
    let mut region: *mut lang_memory_region_type = 0 as *mut lang_memory_region_type;
    if strcmp(region_name, b"*default*\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
        || strcmp(alias, b"*default*\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
    {
        einfo(
            dcgettext(
                0 as *const libc::c_char,
                b"%F%P:%pS: error: alias for default memory region\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            0 as *mut libc::c_void,
        );
    }
    region = 0 as *mut lang_memory_region_type;
    r = lang_memory_region_list;
    while !r.is_null() {
        n = &mut (*r).name_list;
        while !n.is_null() {
            if region.is_null() && strcmp((*n).name, region_name) == 0 as libc::c_int {
                region = r;
            }
            if strcmp((*n).name, alias) == 0 as libc::c_int {
                einfo(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%F%P:%pS: error: redefinition of memory region alias `%s'\n\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    0 as *mut libc::c_void,
                    alias,
                );
            }
            n = (*n).next;
        }
        r = (*r).next;
    }
    if region.is_null() {
        einfo(
            dcgettext(
                0 as *const libc::c_char,
                b"%F%P:%pS: error: memory region `%s' for alias `%s' does not exist\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            0 as *mut libc::c_void,
            region_name,
            alias,
        );
    }
    n = stat_alloc(::core::mem::size_of::<lang_memory_region_name>() as libc::c_ulong)
        as *mut lang_memory_region_name;
    (*n).name = xstrdup(alias);
    (*n).next = (*region).name_list.next;
    (*region).name_list.next = n;
}
#[no_mangle]
pub unsafe extern "C" fn lang_map() {
    let mut m: *mut lang_memory_region_type = 0 as *mut lang_memory_region_type;
    let mut dis_header_printed: bool = 0 as libc::c_int != 0;
    let mut file: *mut lang_input_statement_type = 0 as *mut lang_input_statement_type;
    file = file_chain.head as *mut lang_input_statement_type;
    while !file.is_null() {
        let mut s: *mut asection = 0 as *mut asection;
        if !((*(*file).the_bfd).flags
            & (0x1000 as libc::c_int | 0x40 as libc::c_int) as libc::c_uint
            != 0 as libc::c_int as libc::c_uint
            || ((*file).flags).just_syms() as libc::c_int != 0)
        {
            if config.print_map_discarded {
                s = (*(*file).the_bfd).sections;
                while !s.is_null() {
                    if (((*s).output_section).is_null()
                        || (*(*s).output_section).owner != link_info.output_bfd)
                        && (*s).flags
                            & (0x100000 as libc::c_int | 0x200000 as libc::c_int)
                                as libc::c_uint == 0 as libc::c_int as libc::c_uint
                    {
                        if !dis_header_printed {
                            fprintf(
                                config.map_file,
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"\nDiscarded input sections\n\n\0" as *const u8
                                        as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                            );
                            dis_header_printed = 1 as libc::c_int != 0;
                        }
                        print_input_section(s, 1 as libc::c_int != 0);
                    }
                    s = (*s).next;
                }
            }
        }
        file = (*file).next;
    }
    minfo(
        dcgettext(
            0 as *const libc::c_char,
            b"\nMemory Configuration\n\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    fprintf(
        config.map_file,
        b"%-16s %-18s %-18s %s\n\0" as *const u8 as *const libc::c_char,
        dcgettext(
            0 as *const libc::c_char,
            b"Name\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        dcgettext(
            0 as *const libc::c_char,
            b"Origin\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        dcgettext(
            0 as *const libc::c_char,
            b"Length\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        dcgettext(
            0 as *const libc::c_char,
            b"Attributes\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    m = lang_memory_region_list;
    while !m.is_null() {
        let mut buf: [libc::c_char; 100] = [0; 100];
        let mut len: libc::c_int = 0;
        fprintf(
            config.map_file,
            b"%-16s \0" as *const u8 as *const libc::c_char,
            (*m).name_list.name,
        );
        sprintf(
            buf.as_mut_ptr(),
            b"%016lx\0" as *const u8 as *const libc::c_char,
            (*m).origin,
        );
        minfo(b"0x%s \0" as *const u8 as *const libc::c_char, buf.as_mut_ptr());
        len = strlen(buf.as_mut_ptr()) as libc::c_int;
        while len < 16 as libc::c_int {
            print_space();
            len += 1;
            len;
        }
        minfo(b"0x%V\0" as *const u8 as *const libc::c_char, (*m).length);
        if (*m).flags != 0 || (*m).not_flags != 0 {
            if (*m).flags != 0 {
                print_space();
                lang_map_flags((*m).flags);
            }
            if (*m).not_flags != 0 {
                minfo(b" !\0" as *const u8 as *const libc::c_char);
                lang_map_flags((*m).not_flags);
            }
        }
        print_nl();
        m = (*m).next;
    }
    fprintf(
        config.map_file,
        dcgettext(
            0 as *const libc::c_char,
            b"\nLinker script and memory map\n\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    if link_info.reduce_memory_overheads() == 0 {
        _obstack_begin(
            &mut map_obstack,
            1000 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
            Some(xmalloc as unsafe extern "C" fn(size_t) -> *mut libc::c_void),
            Some(free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        );
        bfd_link_hash_traverse(
            link_info.hash,
            Some(
                sort_def_symbol
                    as unsafe extern "C" fn(
                        *mut bfd_link_hash_entry,
                        *mut libc::c_void,
                    ) -> bool,
            ),
            0 as *mut libc::c_void,
        );
    }
    expld.phase = lang_fixed_phase_enum;
    lang_statement_iteration += 1;
    lang_statement_iteration;
    print_statements();
    ldemul_extra_map_file_text(link_info.output_bfd, &mut link_info, config.map_file);
}
#[no_mangle]
pub unsafe extern "C" fn lang_set_flags(
    mut ptr: *mut lang_memory_region_type,
    mut flags: *const libc::c_char,
    mut invert: libc::c_int,
) {
    let mut ptr_flags: *mut flagword = 0 as *mut flagword;
    ptr_flags = if invert != 0 { &mut (*ptr).not_flags } else { &mut (*ptr).flags };
    while *flags != 0 {
        match *flags as libc::c_int {
            33 => {
                invert = (invert == 0) as libc::c_int;
                ptr_flags = if invert != 0 {
                    &mut (*ptr).not_flags
                } else {
                    &mut (*ptr).flags
                };
            }
            65 | 97 => {
                *ptr_flags |= 0x1 as libc::c_int as libc::c_uint;
            }
            82 | 114 => {
                *ptr_flags |= 0x8 as libc::c_int as libc::c_uint;
            }
            87 | 119 => {
                *ptr_flags |= 0x20 as libc::c_int as libc::c_uint;
            }
            88 | 120 => {
                *ptr_flags |= 0x10 as libc::c_int as libc::c_uint;
            }
            76 | 108 | 73 | 105 => {
                *ptr_flags |= 0x2 as libc::c_int as libc::c_uint;
            }
            _ => {
                einfo(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%F%P: invalid character %c (%d) in flags\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    *flags as libc::c_int,
                    *flags as libc::c_int,
                );
            }
        }
        flags = flags.offset(1);
        flags;
    }
}
#[no_mangle]
pub unsafe extern "C" fn lang_add_output(
    mut name: *const libc::c_char,
    mut from_script: libc::c_int,
) {
    if !had_output_filename || from_script == 0 {
        output_filename = name;
        had_output_filename = 1 as libc::c_int != 0;
    }
}
#[no_mangle]
pub unsafe extern "C" fn lang_enter_output_section_statement(
    mut output_section_statement_name: *const libc::c_char,
    mut address_exp: *mut etree_type,
    mut sectype: section_type,
    mut align: *mut etree_type,
    mut subalign: *mut etree_type,
    mut ebase: *mut etree_type,
    mut constraint: libc::c_int,
    mut align_with_input: libc::c_int,
) -> *mut lang_output_section_statement_type {
    let mut os: *mut lang_output_section_statement_type = 0
        as *mut lang_output_section_statement_type;
    os = lang_output_section_statement_lookup(
        output_section_statement_name,
        constraint,
        2 as libc::c_int,
    );
    current_section = os;
    if ((*os).addr_tree).is_null() {
        (*os).addr_tree = address_exp;
    }
    (*os).sectype = sectype;
    if sectype as libc::c_uint != noload_section as libc::c_int as libc::c_uint {
        (*os).flags = 0 as libc::c_int as flagword;
    } else {
        (*os).flags = 0x200 as libc::c_int as flagword;
    }
    (*os).block_value = 1 as libc::c_int as libc::c_uint;
    push_stat_ptr(&mut (*os).children);
    (*os)
        .set_align_lma_with_input(
            (align_with_input == 384 as libc::c_int) as libc::c_int as libc::c_uint,
        );
    if (*os).align_lma_with_input() as libc::c_int != 0 && !align.is_null() {
        einfo(
            dcgettext(
                0 as *const libc::c_char,
                b"%F%P:%pS: error: align with input and explicit align specified\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            0 as *mut libc::c_void,
        );
    }
    (*os).subsection_alignment = subalign;
    (*os).section_alignment = align;
    (*os).load_base = ebase;
    return os;
}
#[no_mangle]
pub unsafe extern "C" fn lang_final() {
    let mut new_stmt: *mut lang_output_statement_type = 0
        as *mut lang_output_statement_type;
    new_stmt = new_statement(
        lang_output_statement_enum,
        ::core::mem::size_of::<lang_output_statement_type>() as libc::c_ulong,
        stat_ptr,
    ) as *mut lang_output_statement_type;
    (*new_stmt).name = output_filename;
}
#[no_mangle]
pub unsafe extern "C" fn lang_relax_sections(mut need_layout: bool) {
    if link_info.disable_target_specific_optimizations == 0 as libc::c_int
        || link_info.disable_target_specific_optimizations == 1 as libc::c_int
    {
        let mut i: libc::c_int = link_info.relax_pass;
        link_info.relax_pass = 0 as libc::c_int;
        loop {
            let fresh0 = i;
            i = i - 1;
            if !(fresh0 != 0) {
                break;
            }
            let mut relax_again: bool = false;
            link_info.relax_trip = -(1 as libc::c_int);
            loop {
                link_info.relax_trip += 1;
                link_info.relax_trip;
                lang_do_assignments(lang_assigning_phase_enum);
                lang_reset_memory_regions();
                relax_again = 0 as libc::c_int != 0;
                lang_size_sections(&mut relax_again, 0 as libc::c_int != 0);
                if !relax_again {
                    break;
                }
            }
            link_info.relax_pass += 1;
            link_info.relax_pass;
        }
        need_layout = 1 as libc::c_int != 0;
    }
    if need_layout {
        lang_do_assignments(lang_assigning_phase_enum);
        lang_reset_memory_regions();
        lang_size_sections(0 as *mut bool, 1 as libc::c_int != 0);
    }
}
#[no_mangle]
pub unsafe extern "C" fn lang_process() {
    if !(link_info.dynamic_list).is_null() {
        lang_finalize_version_expr_head(&mut (*link_info.dynamic_list).head);
    }
    current_target = default_target;
    lang_for_each_statement(
        Some(
            ldlang_open_output
                as unsafe extern "C" fn(*mut lang_statement_union_type) -> (),
        ),
    );
    init_opb(0 as *mut asection);
    ldemul_create_output_section_statements();
    lang_place_undefineds();
    if !bfd_section_already_linked_table_init() {
        einfo(
            dcgettext(
                0 as *const libc::c_char,
                b"%F%P: can not create hash table: %E\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    lang_do_memory_regions(0 as libc::c_int != 0);
    current_target = default_target;
    lang_statement_iteration += 1;
    lang_statement_iteration;
    open_input_bfds(statement_list.head, OPEN_BFD_NORMAL);
    lang_do_memory_regions(1 as libc::c_int != 0);
    if link_info.lto_plugin_active() != 0 {
        let mut added: lang_statement_list_type = lang_statement_list_type {
            head: 0 as *const lang_statement_union as *mut lang_statement_union,
            tail: 0 as *const *mut lang_statement_union as *mut *mut lang_statement_union,
        };
        let mut files: lang_statement_list_type = lang_statement_list_type {
            head: 0 as *const lang_statement_union as *mut lang_statement_union,
            tail: 0 as *const *mut lang_statement_union as *mut *mut lang_statement_union,
        };
        let mut inputfiles: lang_statement_list_type = lang_statement_list_type {
            head: 0 as *const lang_statement_union as *mut lang_statement_union,
            tail: 0 as *const *mut lang_statement_union as *mut *mut lang_statement_union,
        };
        added = *stat_ptr;
        files = file_chain;
        inputfiles = input_file_chain;
        if plugin_call_all_symbols_read() != 0 {
            einfo(
                dcgettext(
                    0 as *const libc::c_char,
                    b"%F%P: %s: plugin reported error after all symbols read\n\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                plugin_error_plugin(),
            );
        }
        link_info.set_lto_all_symbols_read(1 as libc::c_int as libc::c_uint);
        plugin_undefs = (*link_info.hash).undefs_tail;
        open_input_bfds(*added.tail, OPEN_BFD_NORMAL);
        if plugin_undefs == (*link_info.hash).undefs_tail {
            plugin_undefs = 0 as *mut bfd_link_hash_entry;
        }
        lang_list_remove_tail(stat_ptr, &mut added);
        lang_list_remove_tail(&mut file_chain, &mut files);
        lang_list_remove_tail(&mut input_file_chain, &mut inputfiles);
        if !(added.head).is_null() {
            let mut before: bool = false;
            let mut prev: *mut *mut lang_statement_union_type = 0
                as *mut *mut lang_statement_union_type;
            plugin_insert = find_replacements_insert_point(&mut before);
            if plugin_insert.is_null() {
                info_assert(
                    b"ldlang.c\0" as *const u8 as *const libc::c_char,
                    8008 as libc::c_int as libc::c_uint,
                );
            }
            prev = &mut (*plugin_insert).header.next;
            if before {
                prev = find_next_input_statement(prev);
                if *prev
                    != (*plugin_insert).next_real_file as *mut libc::c_void
                        as *mut lang_statement_union_type
                {
                    prev = &mut (*plugin_insert).header.next;
                }
            }
            lang_list_insert_after(stat_ptr, &mut added, prev);
            lang_list_insert_after(
                &mut input_file_chain,
                &mut inputfiles,
                &mut (*plugin_insert).next_real_file
                    as *mut *mut lang_input_statement_struct as *mut libc::c_void
                    as *mut *mut lang_statement_union_type,
            );
            if !((*plugin_insert).filename).is_null() {
                lang_list_insert_after(
                    &mut file_chain,
                    &mut files,
                    &mut (*plugin_insert).next as *mut *mut lang_input_statement_struct
                        as *mut libc::c_void as *mut *mut lang_statement_union_type,
                );
            } else {
                lang_list_insert_after(
                    &mut file_chain,
                    &mut files,
                    &mut file_chain.head,
                );
            }
            files = file_chain;
            lang_statement_iteration += 1;
            lang_statement_iteration;
            open_input_bfds(statement_list.head, OPEN_BFD_RESCAN);
            lang_list_remove_tail(&mut file_chain, &mut files);
            while !(files.head).is_null() {
                let mut insert: *mut *mut lang_input_statement_type = 0
                    as *mut *mut lang_input_statement_type;
                let mut iter: *mut *mut lang_input_statement_type = 0
                    as *mut *mut lang_input_statement_type;
                let mut temp: *mut lang_input_statement_type = 0
                    as *mut lang_input_statement_type;
                let mut my_arch: *mut bfd = 0 as *mut bfd;
                insert = find_rescan_insertion(&mut (*files.head).input_statement);
                iter = &mut (*files.head).input_statement.next;
                my_arch = (*(*files.head).input_statement.the_bfd).my_archive;
                if !my_arch.is_null() {
                    while !(*iter).is_null() {
                        if (*(**iter).the_bfd).my_archive != my_arch {
                            break;
                        }
                        iter = &mut (**iter).next;
                    }
                }
                temp = *insert;
                *insert = &mut (*files.head).input_statement;
                files.head = *iter as *mut lang_statement_union_type;
                *iter = temp;
                if !my_arch.is_null() {
                    let mut parent: *mut lang_input_statement_type = bfd_usrdata(my_arch)
                        as *mut lang_input_statement_type;
                    if !parent.is_null() {
                        (*parent)
                            .next = (iter as *mut libc::c_char)
                            .offset(-(64 as libc::c_ulong as isize))
                            as *mut lang_input_statement_type;
                    }
                }
            }
        }
    }
    if !(link_info.gc_sym_list).is_null() {
        info_assert(
            b"ldlang.c\0" as *const u8 as *const libc::c_char,
            8072 as libc::c_int as libc::c_uint,
        );
    }
    link_info.gc_sym_list = &mut entry_symbol;
    if (entry_symbol.name).is_null() {
        link_info.gc_sym_list = entry_symbol.next;
        lang_add_gc_name(entry_symbol_default);
    }
    lang_add_gc_name(link_info.init_function);
    lang_add_gc_name(link_info.fini_function);
    ldemul_after_open();
    if !(config.map_file).is_null() {
        lang_print_asneeded();
    }
    ldlang_open_ctf();
    bfd_section_already_linked_table_free();
    lang_check();
    if !(command_line.version_exports_section).is_null() {
        lang_do_version_exports_section();
    }
    ldctor_build_sets();
    if config.build_constructors {
        lang_init_start_stop();
    }
    lang_do_assignments(lang_mark_phase_enum);
    expld.phase = lang_first_phase_enum;
    lang_common();
    lang_gc_sections();
    lang_mark_undefineds();
    lang_check_relocs();
    ldemul_after_check_relocs();
    update_wild_statements(statement_list.head);
    lang_statement_iteration += 1;
    lang_statement_iteration;
    map_input_to_output_sections(
        statement_list.head,
        0 as *const libc::c_char,
        0 as *mut lang_output_section_statement_type,
    );
    process_insert_statements(&mut (*lang_os_list.head).header.next);
    ldemul_before_place_orphans();
    lang_place_orphans();
    if !(link_info.type_0() as libc::c_int == type_relocatable as libc::c_int) {
        let mut found: *mut asection = 0 as *mut asection;
        (Some(
            ((*(*link_info.output_bfd).xvec)._bfd_merge_sections)
                .expect("non-null function pointer"),
        ))
            .expect("non-null function pointer")(link_info.output_bfd, &mut link_info);
        found = bfd_get_section_by_name(
            link_info.output_bfd,
            b".text\0" as *const u8 as *const libc::c_char,
        );
        if !found.is_null() {
            if config.text_read_only {
                (*found).flags |= 0x8 as libc::c_int as libc::c_uint;
            } else {
                (*found).flags &= !(0x8 as libc::c_int) as libc::c_uint;
            }
        }
    }
    lang_merge_ctf();
    lang_write_ctf(0 as libc::c_int);
    lang_propagate_lma_regions();
    if config.build_constructors {
        lang_undef_start_stop();
    }
    if !(link_info.type_0() as libc::c_int == type_relocatable as libc::c_int) {
        lang_init_startof_sizeof();
    }
    ldemul_before_allocation();
    lang_record_phdrs();
    if link_info.relro() as libc::c_int != 0
        && !(link_info.type_0() as libc::c_int == type_relocatable as libc::c_int)
    {
        lang_find_relro_sections();
    }
    lang_size_sections(
        0 as *mut bool,
        !(link_info.disable_target_specific_optimizations == 0 as libc::c_int
            || link_info.disable_target_specific_optimizations == 1 as libc::c_int),
    );
    ldemul_after_allocation();
    lang_finalize_start_stop();
    lang_do_assignments(lang_final_phase_enum);
    ldemul_finish();
    ldexp_finalize_syms();
    if command_line.check_section_addresses != 0 {
        lang_check_section_addresses();
    }
    ldlang_check_require_defined_symbols();
    lang_end();
}
#[no_mangle]
pub unsafe extern "C" fn lang_section_start(
    mut name: *const libc::c_char,
    mut address: *mut etree_type,
    mut segment: *const segment_type,
) {
    let mut ad: *mut lang_address_statement_type = 0 as *mut lang_address_statement_type;
    ad = new_statement(
        lang_address_statement_enum,
        ::core::mem::size_of::<lang_address_statement_type>() as libc::c_ulong,
        stat_ptr,
    ) as *mut lang_address_statement_type;
    (*ad).section_name = name;
    (*ad).address = address;
    (*ad).segment = segment;
}
#[no_mangle]
pub unsafe extern "C" fn lang_add_entry(
    mut name: *const libc::c_char,
    mut cmdline: bool,
) {
    if (entry_symbol.name).is_null() || cmdline as libc::c_int != 0
        || !entry_from_cmdline
    {
        entry_symbol.name = name;
        entry_from_cmdline = cmdline;
    }
}
#[no_mangle]
pub unsafe extern "C" fn lang_default_entry(mut name: *const libc::c_char) {
    entry_symbol_default = name;
}
#[no_mangle]
pub unsafe extern "C" fn lang_add_target(mut name: *const libc::c_char) {
    let mut new_stmt: *mut lang_target_statement_type = 0
        as *mut lang_target_statement_type;
    new_stmt = new_statement(
        lang_target_statement_enum,
        ::core::mem::size_of::<lang_target_statement_type>() as libc::c_ulong,
        stat_ptr,
    ) as *mut lang_target_statement_type;
    (*new_stmt).target = name;
}
#[no_mangle]
pub unsafe extern "C" fn lang_add_wild(
    mut filespec: *mut wildcard_spec,
    mut section_list: *mut wildcard_list,
    mut keep_sections: bool,
) {
    let mut curr: *mut wildcard_list = 0 as *mut wildcard_list;
    let mut next: *mut wildcard_list = 0 as *mut wildcard_list;
    let mut new_stmt: *mut lang_wild_statement_type = 0 as *mut lang_wild_statement_type;
    curr = section_list;
    section_list = 0 as *mut wildcard_list;
    while !curr.is_null() {
        next = (*curr).next;
        (*curr).next = section_list;
        section_list = curr;
        curr = next;
    }
    if !filespec.is_null() && !((*filespec).name).is_null() {
        if strcmp((*filespec).name, b"*\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            (*filespec).name = 0 as *const libc::c_char;
        } else if (strpbrk(
            (*filespec).name,
            b"?*[\0" as *const u8 as *const libc::c_char,
        ))
            .is_null()
        {
            lang_has_input_file = 1 as libc::c_int != 0;
        }
    }
    new_stmt = new_statement(
        lang_wild_statement_enum,
        ::core::mem::size_of::<lang_wild_statement_type>() as libc::c_ulong,
        stat_ptr,
    ) as *mut lang_wild_statement_type;
    (*new_stmt).filename = 0 as *const libc::c_char;
    (*new_stmt).filenames_sorted = 0 as libc::c_int != 0;
    (*new_stmt).section_flag_list = 0 as *mut flag_info;
    (*new_stmt).exclude_name_list = 0 as *mut name_list;
    if !filespec.is_null() {
        (*new_stmt).filename = (*filespec).name;
        (*new_stmt)
            .filenames_sorted = (*filespec).sorted as libc::c_uint
            == by_name as libc::c_int as libc::c_uint;
        (*new_stmt).section_flag_list = (*filespec).section_flag_list;
        (*new_stmt).exclude_name_list = (*filespec).exclude_name_list;
    }
    (*new_stmt).section_list = section_list;
    (*new_stmt).keep_sections = keep_sections;
    lang_list_init(&mut (*new_stmt).children);
    analyze_walk_wild_section_handler(new_stmt);
}
#[no_mangle]
pub unsafe extern "C" fn lang_add_map(mut name: *const libc::c_char) {
    while *name != 0 {
        match *name as libc::c_int {
            70 => {
                map_option_f = 1 as libc::c_int != 0;
            }
            _ => {}
        }
        name = name.offset(1);
        name;
    }
}
#[no_mangle]
pub unsafe extern "C" fn lang_add_fill(mut fill: *mut fill_type) {
    let mut new_stmt: *mut lang_fill_statement_type = 0 as *mut lang_fill_statement_type;
    new_stmt = new_statement(
        lang_fill_statement_enum,
        ::core::mem::size_of::<lang_fill_statement_type>() as libc::c_ulong,
        stat_ptr,
    ) as *mut lang_fill_statement_type;
    (*new_stmt).fill = fill;
}
#[no_mangle]
pub unsafe extern "C" fn lang_add_assignment(
    mut exp: *mut etree_type,
) -> *mut lang_assignment_statement_type {
    let mut new_stmt: *mut lang_assignment_statement_type = 0
        as *mut lang_assignment_statement_type;
    new_stmt = new_statement(
        lang_assignment_statement_enum,
        ::core::mem::size_of::<lang_assignment_statement_type>() as libc::c_ulong,
        stat_ptr,
    ) as *mut lang_assignment_statement_type;
    (*new_stmt).exp = exp;
    return new_stmt;
}
#[no_mangle]
pub unsafe extern "C" fn lang_add_attribute(mut attribute: statement_enum) {
    new_statement(
        attribute,
        ::core::mem::size_of::<lang_statement_header_type>() as libc::c_ulong,
        stat_ptr,
    );
}
#[no_mangle]
pub unsafe extern "C" fn lang_startup(mut name: *const libc::c_char) {
    if !((*first_file).filename).is_null() {
        einfo(
            dcgettext(
                0 as *const libc::c_char,
                b"%F%P: multiple STARTUP files\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    (*first_file).filename = name;
    (*first_file).local_sym_name = name;
    ((*first_file).flags).set_real(1 as libc::c_int as libc::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn lang_float(mut maybe: bool) {
    lang_float_flag = maybe;
}
#[no_mangle]
pub unsafe extern "C" fn lang_leave_output_section_statement(
    mut fill: *mut fill_type,
    mut memspec: *const libc::c_char,
    mut phdrs: *mut lang_output_section_phdr_list,
    mut lma_memspec: *const libc::c_char,
) {
    lang_get_regions(
        &mut (*current_section).region,
        &mut (*current_section).lma_region,
        memspec,
        lma_memspec,
        !((*current_section).load_base).is_null(),
        !((*current_section).addr_tree).is_null(),
    );
    (*current_section).fill = fill;
    (*current_section).phdrs = phdrs;
    pop_stat_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn lang_for_each_input_file(
    mut func: Option::<unsafe extern "C" fn(*mut lang_input_statement_type) -> ()>,
) {
    let mut f: *mut lang_input_statement_type = 0 as *mut lang_input_statement_type;
    f = input_file_chain.head as *mut libc::c_void as *mut lang_input_statement_type;
    while !f.is_null() {
        if ((*f).flags).real() != 0 {
            func.expect("non-null function pointer")(f);
        }
        f = (*f).next_real_file;
    }
}
#[no_mangle]
pub unsafe extern "C" fn lang_for_each_file(
    mut func: Option::<unsafe extern "C" fn(*mut lang_input_statement_type) -> ()>,
) {
    let mut f: *mut lang_input_statement_type = 0 as *mut lang_input_statement_type;
    f = file_chain.head as *mut lang_input_statement_type;
    while !f.is_null() {
        if ((*f).flags).real() != 0 {
            func.expect("non-null function pointer")(f);
        }
        f = (*f).next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn lang_reset_memory_regions() {
    let mut p: *mut lang_memory_region_type = lang_memory_region_list;
    let mut o: *mut asection = 0 as *mut asection;
    let mut os: *mut lang_output_section_statement_type = 0
        as *mut lang_output_section_statement_type;
    p = lang_memory_region_list;
    while !p.is_null() {
        (*p).current = (*p).origin;
        (*p).last_os = 0 as *mut lang_statement_union;
        p = (*p).next;
    }
    os = lang_os_list.head as *mut libc::c_void
        as *mut lang_output_section_statement_type;
    while !os.is_null() {
        (*os).set_processed_vma(0 as libc::c_int as libc::c_uint);
        (*os).set_processed_lma(0 as libc::c_int as libc::c_uint);
        os = (*os).next;
    }
    o = (*link_info.output_bfd).sections;
    while !o.is_null() {
        (*o).rawsize = (*o).size;
        if (*o).flags & 0x800 as libc::c_int as libc::c_uint == 0 {
            (*o).size = 0 as libc::c_int as bfd_size_type;
        }
        o = (*o).next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn lang_do_assignments(mut phase: lang_phase_type) {
    let mut found_end: bool = 0 as libc::c_int != 0;
    current_section = 0 as *mut lang_output_section_statement_type;
    prefer_next_section = 0 as libc::c_int != 0;
    expld.phase = phase;
    lang_statement_iteration += 1;
    lang_statement_iteration;
    lang_do_assignments_1(
        statement_list.head,
        abs_output_section,
        0 as *mut fill_type,
        0 as libc::c_int as bfd_vma,
        &mut found_end,
    );
}
#[no_mangle]
pub unsafe extern "C" fn section_for_dot() -> *mut asection {
    let mut s: *mut asection = 0 as *mut asection;
    if current_section.is_null() || prefer_next_section as libc::c_int != 0 {
        let mut stmt: *mut lang_statement_union_type = 0
            as *mut lang_statement_union_type;
        let mut os: *mut lang_output_section_statement_type = 0
            as *mut lang_output_section_statement_type;
        stmt = current_assign as *mut lang_statement_union_type;
        while !stmt.is_null() {
            if (*stmt).header.type_0 as libc::c_uint
                == lang_output_section_statement_enum as libc::c_int as libc::c_uint
            {
                break;
            }
            stmt = (*stmt).header.next;
        }
        os = &mut (*stmt).output_section_statement;
        while !os.is_null() && (*os).after_end() == 0
            && (((*os).bfd_section).is_null()
                || (*(*os).bfd_section).flags & 0x8000 as libc::c_int as libc::c_uint
                    != 0 as libc::c_int as libc::c_uint
                || bfd_section_removed_from_list(link_info.output_bfd, (*os).bfd_section)
                    as libc::c_int != 0)
        {
            os = (*os).next;
        }
        if current_section.is_null() || os.is_null() || (*os).after_end() == 0 {
            if !os.is_null() {
                s = (*os).bfd_section;
            } else {
                s = (*link_info.output_bfd).section_last;
            }
            while !s.is_null()
                && ((*s).flags & 0x1 as libc::c_int as libc::c_uint
                    == 0 as libc::c_int as libc::c_uint
                    || (*s).flags & 0x400 as libc::c_int as libc::c_uint
                        != 0 as libc::c_int as libc::c_uint)
            {
                s = (*s).prev;
            }
            if !s.is_null() {
                return s;
            }
            return &mut *_bfd_std_section.as_mut_ptr().offset(2 as libc::c_int as isize)
                as *mut asection;
        }
    }
    s = (*current_section).bfd_section;
    while !s.is_null()
        && ((*s).flags & 0x8000 as libc::c_int as libc::c_uint
            != 0 as libc::c_int as libc::c_uint
            || (*s).flags & 0x1 as libc::c_int as libc::c_uint
                == 0 as libc::c_int as libc::c_uint
            || (*s).flags & 0x400 as libc::c_int as libc::c_uint
                != 0 as libc::c_int as libc::c_uint
            || bfd_section_removed_from_list(link_info.output_bfd, s) as libc::c_int
                != 0)
    {
        s = (*s).prev;
    }
    if s.is_null() {
        s = (*link_info.output_bfd).sections;
    }
    while !s.is_null()
        && ((*s).flags & 0x1 as libc::c_int as libc::c_uint
            == 0 as libc::c_int as libc::c_uint
            || (*s).flags & 0x400 as libc::c_int as libc::c_uint
                != 0 as libc::c_int as libc::c_uint)
    {
        s = (*s).next;
    }
    if !s.is_null() {
        return s;
    }
    return &mut *_bfd_std_section.as_mut_ptr().offset(2 as libc::c_int as isize)
        as *mut asection;
}
#[no_mangle]
pub unsafe extern "C" fn ldlang_add_file(mut entry: *mut lang_input_statement_type) {
    lang_statement_append(
        &mut file_chain,
        entry as *mut libc::c_void,
        &mut (*entry).next as *mut *mut lang_input_statement_struct as *mut libc::c_void,
    );
    if !(link_info.input_bfds_tail != &mut (*(*entry).the_bfd).link.next as *mut *mut bfd
        && ((*(*entry).the_bfd).link.next).is_null())
    {
        info_assert(
            b"ldlang.c\0" as *const u8 as *const libc::c_char,
            7403 as libc::c_int as libc::c_uint,
        );
    }
    if !((*entry).the_bfd != link_info.output_bfd) {
        info_assert(
            b"ldlang.c\0" as *const u8 as *const libc::c_char,
            7404 as libc::c_int as libc::c_uint,
        );
    }
    *link_info.input_bfds_tail = (*entry).the_bfd;
    link_info.input_bfds_tail = &mut (*(*entry).the_bfd).link.next;
    bfd_set_usrdata((*entry).the_bfd, entry as *mut libc::c_void);
    bfd_set_gp_size((*entry).the_bfd, g_switch_value as libc::c_uint);
    bfd_map_over_sections(
        (*entry).the_bfd,
        Some(
            section_already_linked
                as unsafe extern "C" fn(*mut bfd, *mut asection, *mut libc::c_void) -> (),
        ),
        entry as *mut libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn lang_output_section_find_by_flags(
    mut sec: *const asection,
    mut sec_flags: flagword,
    mut exact: *mut *mut lang_output_section_statement_type,
    mut match_type: lang_match_sec_type_func,
) -> *mut lang_output_section_statement_type {
    let mut first: *mut lang_output_section_statement_type = 0
        as *mut lang_output_section_statement_type;
    let mut look: *mut lang_output_section_statement_type = 0
        as *mut lang_output_section_statement_type;
    let mut found: *mut lang_output_section_statement_type = 0
        as *mut lang_output_section_statement_type;
    let mut look_flags: flagword = 0;
    let mut differ: flagword = 0;
    first = lang_os_list.head as *mut libc::c_void
        as *mut lang_output_section_statement_type;
    first = (*first).next;
    found = 0 as *mut lang_output_section_statement_type;
    let mut current_block_8: u64;
    look = first;
    while !look.is_null() {
        look_flags = (*look).flags;
        if !((*look).bfd_section).is_null() {
            look_flags = (*(*look).bfd_section).flags;
            if match_type.is_some()
                && !match_type
                    .expect(
                        "non-null function pointer",
                    )(link_info.output_bfd, (*look).bfd_section, (*sec).owner, sec)
            {
                current_block_8 = 12675440807659640239;
            } else {
                current_block_8 = 6937071982253665452;
            }
        } else {
            current_block_8 = 6937071982253665452;
        }
        match current_block_8 {
            6937071982253665452 => {
                differ = look_flags ^ sec_flags;
                if differ
                    & (0x100 as libc::c_int | 0x1 as libc::c_int | 0x2 as libc::c_int
                        | 0x8 as libc::c_int | 0x10 as libc::c_int
                        | 0x400000 as libc::c_int | 0x400 as libc::c_int) as libc::c_uint
                    == 0
                {
                    found = look;
                }
            }
            _ => {}
        }
        look = (*look).next;
    }
    if !found.is_null() {
        if !exact.is_null() {
            *exact = found;
        }
        return found;
    }
    if sec_flags & 0x10 as libc::c_int as libc::c_uint
        != 0 as libc::c_int as libc::c_uint
        && sec_flags & 0x1 as libc::c_int as libc::c_uint
            != 0 as libc::c_int as libc::c_uint
    {
        let mut current_block_20: u64;
        look = first;
        while !look.is_null() {
            look_flags = (*look).flags;
            if !((*look).bfd_section).is_null() {
                look_flags = (*(*look).bfd_section).flags;
                if match_type.is_some()
                    && !match_type
                        .expect(
                            "non-null function pointer",
                        )(link_info.output_bfd, (*look).bfd_section, (*sec).owner, sec)
                {
                    current_block_20 = 8831408221741692167;
                } else {
                    current_block_20 = 12147880666119273379;
                }
            } else {
                current_block_20 = 12147880666119273379;
            }
            match current_block_20 {
                12147880666119273379 => {
                    differ = look_flags ^ sec_flags;
                    if differ
                        & (0x100 as libc::c_int | 0x1 as libc::c_int | 0x2 as libc::c_int
                            | 0x10 as libc::c_int | 0x400000 as libc::c_int
                            | 0x400 as libc::c_int) as libc::c_uint == 0
                    {
                        found = look;
                    }
                }
                _ => {}
            }
            look = (*look).next;
        }
    } else if sec_flags & 0x8 as libc::c_int as libc::c_uint
        != 0 as libc::c_int as libc::c_uint
        && sec_flags & 0x1 as libc::c_int as libc::c_uint
            != 0 as libc::c_int as libc::c_uint
    {
        let mut current_block_28: u64;
        look = first;
        while !look.is_null() {
            look_flags = (*look).flags;
            if !((*look).bfd_section).is_null() {
                look_flags = (*(*look).bfd_section).flags;
                if match_type.is_some()
                    && !match_type
                        .expect(
                            "non-null function pointer",
                        )(link_info.output_bfd, (*look).bfd_section, (*sec).owner, sec)
                {
                    current_block_28 = 2232869372362427478;
                } else {
                    current_block_28 = 11459959175219260272;
                }
            } else {
                current_block_28 = 11459959175219260272;
            }
            match current_block_28 {
                11459959175219260272 => {
                    differ = look_flags ^ sec_flags;
                    if differ
                        & (0x100 as libc::c_int | 0x1 as libc::c_int | 0x2 as libc::c_int
                            | 0x8 as libc::c_int | 0x400000 as libc::c_int)
                            as libc::c_uint == 0
                        || differ
                            & (0x100 as libc::c_int | 0x1 as libc::c_int
                                | 0x2 as libc::c_int | 0x8 as libc::c_int) as libc::c_uint
                            == 0
                            && look_flags & 0x400000 as libc::c_int as libc::c_uint == 0
                    {
                        found = look;
                    }
                }
                _ => {}
            }
            look = (*look).next;
        }
    } else if sec_flags & 0x400 as libc::c_int as libc::c_uint
        != 0 as libc::c_int as libc::c_uint
        && sec_flags & 0x1 as libc::c_int as libc::c_uint
            != 0 as libc::c_int as libc::c_uint
    {
        let mut seen_thread_local: bool = 0 as libc::c_int != 0;
        match_type = None;
        look = first;
        while !look.is_null() {
            look_flags = (*look).flags;
            if !((*look).bfd_section).is_null() {
                look_flags = (*(*look).bfd_section).flags;
            }
            differ = look_flags
                ^ (sec_flags | 0x2 as libc::c_int as libc::c_uint
                    | 0x100 as libc::c_int as libc::c_uint);
            if differ & (0x400 as libc::c_int | 0x1 as libc::c_int) as libc::c_uint == 0
            {
                if look_flags & 0x2 as libc::c_int as libc::c_uint == 0
                    && sec_flags & 0x2 as libc::c_int as libc::c_uint != 0
                {
                    break;
                }
                found = look;
                seen_thread_local = 1 as libc::c_int != 0;
            } else {
                if seen_thread_local {
                    break;
                }
                if differ
                    & (0x100 as libc::c_int | 0x1 as libc::c_int | 0x2 as libc::c_int)
                        as libc::c_uint == 0
                {
                    found = look;
                }
            }
            look = (*look).next;
        }
    } else if sec_flags & 0x400000 as libc::c_int as libc::c_uint
        != 0 as libc::c_int as libc::c_uint
        && sec_flags & 0x1 as libc::c_int as libc::c_uint
            != 0 as libc::c_int as libc::c_uint
    {
        let mut current_block_48: u64;
        look = first;
        while !look.is_null() {
            look_flags = (*look).flags;
            if !((*look).bfd_section).is_null() {
                look_flags = (*(*look).bfd_section).flags;
                if match_type.is_some()
                    && !match_type
                        .expect(
                            "non-null function pointer",
                        )(link_info.output_bfd, (*look).bfd_section, (*sec).owner, sec)
                {
                    current_block_48 = 11793792312832361944;
                } else {
                    current_block_48 = 2500484646272006982;
                }
            } else {
                current_block_48 = 2500484646272006982;
            }
            match current_block_48 {
                2500484646272006982 => {
                    differ = look_flags ^ sec_flags;
                    if differ
                        & (0x100 as libc::c_int | 0x1 as libc::c_int | 0x2 as libc::c_int
                            | 0x400 as libc::c_int) as libc::c_uint == 0
                        || look_flags & 0x400000 as libc::c_int as libc::c_uint != 0
                            && sec_flags & 0x100 as libc::c_int as libc::c_uint == 0
                    {
                        found = look;
                    }
                }
                _ => {}
            }
            look = (*look).next;
        }
    } else if sec_flags & 0x100 as libc::c_int as libc::c_uint
        != 0 as libc::c_int as libc::c_uint
        && sec_flags & 0x1 as libc::c_int as libc::c_uint
            != 0 as libc::c_int as libc::c_uint
    {
        let mut current_block_56: u64;
        look = first;
        while !look.is_null() {
            look_flags = (*look).flags;
            if !((*look).bfd_section).is_null() {
                look_flags = (*(*look).bfd_section).flags;
                if match_type.is_some()
                    && !match_type
                        .expect(
                            "non-null function pointer",
                        )(link_info.output_bfd, (*look).bfd_section, (*sec).owner, sec)
                {
                    current_block_56 = 8835654301469918283;
                } else {
                    current_block_56 = 317151059986244064;
                }
            } else {
                current_block_56 = 317151059986244064;
            }
            match current_block_56 {
                317151059986244064 => {
                    differ = look_flags ^ sec_flags;
                    if differ
                        & (0x100 as libc::c_int | 0x1 as libc::c_int | 0x2 as libc::c_int
                            | 0x400000 as libc::c_int | 0x400 as libc::c_int)
                            as libc::c_uint == 0
                    {
                        found = look;
                    }
                }
                _ => {}
            }
            look = (*look).next;
        }
    } else if sec_flags & 0x1 as libc::c_int as libc::c_uint
        != 0 as libc::c_int as libc::c_uint
    {
        let mut current_block_64: u64;
        look = first;
        while !look.is_null() {
            look_flags = (*look).flags;
            if !((*look).bfd_section).is_null() {
                look_flags = (*(*look).bfd_section).flags;
                if match_type.is_some()
                    && !match_type
                        .expect(
                            "non-null function pointer",
                        )(link_info.output_bfd, (*look).bfd_section, (*sec).owner, sec)
                {
                    current_block_64 = 10512632378975961025;
                } else {
                    current_block_64 = 3736434875406665187;
                }
            } else {
                current_block_64 = 3736434875406665187;
            }
            match current_block_64 {
                3736434875406665187 => {
                    differ = look_flags ^ sec_flags;
                    if differ & 0x1 as libc::c_int as libc::c_uint == 0 {
                        found = look;
                    }
                }
                _ => {}
            }
            look = (*look).next;
        }
    } else {
        look = first;
        while !look.is_null() {
            look_flags = (*look).flags;
            if !((*look).bfd_section).is_null() {
                look_flags = (*(*look).bfd_section).flags;
            }
            differ = look_flags ^ sec_flags;
            if differ & 0x2000 as libc::c_int as libc::c_uint == 0 {
                found = look;
            }
            look = (*look).next;
        }
        return found;
    }
    if !found.is_null() || match_type.is_none() {
        return found;
    }
    return lang_output_section_find_by_flags(
        sec,
        sec_flags,
        0 as *mut *mut lang_output_section_statement_type,
        None,
    );
}
#[no_mangle]
pub unsafe extern "C" fn lang_insert_orphan(
    mut s: *mut asection,
    mut secname: *const libc::c_char,
    mut constraint: libc::c_int,
    mut after: *mut lang_output_section_statement_type,
    mut place: *mut orphan_save,
    mut address: *mut etree_type,
    mut add_child: *mut lang_statement_list_type,
) -> *mut lang_output_section_statement_type {
    let mut add: lang_statement_list_type = lang_statement_list_type {
        head: 0 as *const lang_statement_union as *mut lang_statement_union,
        tail: 0 as *const *mut lang_statement_union as *mut *mut lang_statement_union,
    };
    let mut os: *mut lang_output_section_statement_type = 0
        as *mut lang_output_section_statement_type;
    let mut os_tail: *mut *mut lang_output_section_statement_type = 0
        as *mut *mut lang_output_section_statement_type;
    if !after.is_null() {
        lang_list_init(&mut add);
        push_stat_ptr(&mut add);
    }
    if link_info.type_0() as libc::c_int == type_relocatable as libc::c_int
        || (*s).flags & (0x2 as libc::c_int | 0x1 as libc::c_int) as libc::c_uint
            == 0 as libc::c_int as libc::c_uint
    {
        address = exp_intop(0 as libc::c_int as bfd_vma);
    }
    os_tail = lang_os_list.tail as *mut *mut lang_output_section_statement_type;
    os = lang_enter_output_section_statement(
        secname,
        address,
        normal_section,
        0 as *mut etree_type,
        0 as *mut etree_type,
        0 as *mut etree_type,
        constraint,
        0 as libc::c_int,
    );
    if add_child.is_null() {
        add_child = &mut (*os).children;
    }
    lang_add_section(add_child, s, 0 as *mut wildcard_list, 0 as *mut flag_info, os);
    if !after.is_null()
        && (*s).flags & (0x2 as libc::c_int | 0x1 as libc::c_int) as libc::c_uint
            != 0 as libc::c_int as libc::c_uint
    {
        let mut region: *const libc::c_char = if !((*after).region).is_null() {
            (*(*after).region).name_list.name
        } else {
            b"*default*\0" as *const u8 as *const libc::c_char
        };
        let mut lma_region: *const libc::c_char = if !((*after).lma_region).is_null() {
            (*(*after).lma_region).name_list.name
        } else {
            0 as *const libc::c_char
        };
        lang_leave_output_section_statement(
            0 as *mut fill_type,
            region,
            (*after).phdrs,
            lma_region,
        );
    } else {
        lang_leave_output_section_statement(
            0 as *mut fill_type,
            b"*default*\0" as *const u8 as *const libc::c_char,
            0 as *mut lang_output_section_phdr_list,
            0 as *const libc::c_char,
        );
    }
    if !after.is_null() {
        pop_stat_ptr();
    }
    if !after.is_null() && !((*os).bfd_section).is_null() {
        let mut snew: *mut asection = 0 as *mut asection;
        let mut as_0: *mut asection = 0 as *mut asection;
        let mut place_after: bool = ((*place).stmt).is_null();
        let mut insert_after: bool = 1 as libc::c_int != 0;
        snew = (*os).bfd_section;
        if ((*place).section).is_null()
            && after
                != lang_os_list.head as *mut libc::c_void
                    as *mut lang_output_section_statement_type
        {
            let mut bfd_section: *mut asection = (*after).bfd_section;
            if bfd_section.is_null() {
                bfd_section = output_prev_sec_find(after);
            }
            if !bfd_section.is_null() && bfd_section != snew {
                (*place).section = &mut (*bfd_section).next;
            }
        }
        if ((*place).section).is_null() {
            (*place).section = &mut (*link_info.output_bfd).sections;
        }
        as_0 = *(*place).section;
        if as_0.is_null() {
            bfd_section_list_remove(link_info.output_bfd, snew);
            bfd_section_list_append(link_info.output_bfd, snew);
        } else if bfd_get_flavour(link_info.output_bfd) as libc::c_uint
            == bfd_target_elf_flavour as libc::c_int as libc::c_uint
            && bfd_get_flavour((*s).owner) as libc::c_uint
                == bfd_target_elf_flavour as libc::c_int as libc::c_uint
            && ((*((*s).used_by_bfd as *mut bfd_elf_section_data)).this_hdr.sh_type
                == 7 as libc::c_int as libc::c_uint
                && (*s).flags & 0x2 as libc::c_int as libc::c_uint
                    != 0 as libc::c_int as libc::c_uint
                || (*((*as_0).used_by_bfd as *mut bfd_elf_section_data)).this_hdr.sh_type
                    == 7 as libc::c_int as libc::c_uint
                    && (*as_0).flags & 0x2 as libc::c_int as libc::c_uint
                        != 0 as libc::c_int as libc::c_uint)
        {
            let mut sec: *mut asection = 0 as *mut asection;
            let mut after_sec: *mut asection = 0 as *mut asection;
            let mut after_sec_note: bool = 0 as libc::c_int != 0;
            static mut first_orphan_note: *mut asection = 0 as *const asection
                as *mut asection;
            after_sec = 0 as *mut asection;
            if (*((*s).used_by_bfd as *mut bfd_elf_section_data)).this_hdr.sh_type
                == 7 as libc::c_int as libc::c_uint
                && (*s).flags & 0x2 as libc::c_int as libc::c_uint
                    != 0 as libc::c_int as libc::c_uint
            {
                first_orphan_note = 0 as *mut asection;
                sec = (*link_info.output_bfd).sections;
                while !sec.is_null() && !bfd_is_abs_section(sec) {
                    if sec != snew
                        && (*((*sec).used_by_bfd as *mut bfd_elf_section_data))
                            .this_hdr
                            .sh_type == 7 as libc::c_int as libc::c_uint
                        && (*sec).flags & 0x2 as libc::c_int as libc::c_uint
                            != 0 as libc::c_int as libc::c_uint
                    {
                        if first_orphan_note.is_null() {
                            first_orphan_note = sec;
                        }
                        if (*sec).alignment_power >= (*s).alignment_power {
                            after_sec = sec;
                        }
                    } else if !first_orphan_note.is_null() {
                        break;
                    }
                    sec = (*sec).next;
                }
                after_sec_note = !first_orphan_note.is_null();
                if after_sec.is_null() && after_sec_note as libc::c_int != 0 {
                    after_sec = first_orphan_note;
                    insert_after = 0 as libc::c_int != 0;
                }
            } else if !first_orphan_note.is_null() {
                after_sec_note = 1 as libc::c_int != 0;
                after_sec = as_0;
                sec = (*as_0).next;
                while !sec.is_null() && !bfd_is_abs_section(sec) {
                    if (*((*sec).used_by_bfd as *mut bfd_elf_section_data))
                        .this_hdr
                        .sh_type == 7 as libc::c_int as libc::c_uint
                        && (*sec).flags & 0x2 as libc::c_int as libc::c_uint
                            != 0 as libc::c_int as libc::c_uint
                    {
                        after_sec = sec;
                    }
                    sec = (*sec).next;
                }
            }
            if after_sec_note {
                if !after_sec.is_null() {
                    let mut stmt: *mut lang_output_section_statement_type = 0
                        as *mut lang_output_section_statement_type;
                    let mut next: *mut lang_output_section_statement_type = 0
                        as *mut lang_output_section_statement_type;
                    let mut found: bool = 0 as libc::c_int != 0;
                    stmt = after;
                    while !stmt.is_null() {
                        next = (*stmt).next;
                        if insert_after {
                            if (*stmt).bfd_section == after_sec {
                                place_after = 1 as libc::c_int != 0;
                                found = 1 as libc::c_int != 0;
                                after = stmt;
                                break;
                            }
                        } else if !next.is_null() && (*next).bfd_section == after_sec {
                            place_after = 1 as libc::c_int != 0;
                            found = 1 as libc::c_int != 0;
                            after = stmt;
                            break;
                        }
                        stmt = next;
                    }
                    if !found {
                        stmt = after;
                        while !stmt.is_null() {
                            if insert_after {
                                if (*stmt).bfd_section == after_sec {
                                    place_after = 1 as libc::c_int != 0;
                                    after = stmt;
                                    break;
                                }
                            } else if (*(*stmt).next).bfd_section == after_sec {
                                place_after = 1 as libc::c_int != 0;
                                after = stmt;
                                break;
                            }
                            stmt = (*stmt).prev;
                        }
                    }
                }
                if after_sec.is_null()
                    || insert_after as libc::c_int != 0 && (*after_sec).next != snew
                    || !insert_after && (*after_sec).prev != snew
                {
                    bfd_section_list_remove(link_info.output_bfd, snew);
                    if !after_sec.is_null() {
                        if insert_after {
                            bfd_section_list_insert_after(
                                link_info.output_bfd,
                                after_sec,
                                snew,
                            );
                        } else {
                            bfd_section_list_insert_before(
                                link_info.output_bfd,
                                after_sec,
                                snew,
                            );
                        }
                    } else {
                        bfd_section_list_prepend(link_info.output_bfd, snew);
                    }
                }
            } else if as_0 != snew && (*as_0).prev != snew {
                bfd_section_list_remove(link_info.output_bfd, snew);
                bfd_section_list_insert_before(link_info.output_bfd, as_0, snew);
            }
        } else if as_0 != snew && (*as_0).prev != snew {
            bfd_section_list_remove(link_info.output_bfd, snew);
            bfd_section_list_insert_before(link_info.output_bfd, as_0, snew);
        }
        (*place).section = &mut (*snew).next;
        if !(add.head).is_null() {
            let mut newly_added_os: *mut lang_output_section_statement_type = 0
                as *mut lang_output_section_statement_type;
            if place_after {
                let mut where_0: *mut *mut lang_statement_union_type = insert_os_after(
                    after,
                );
                *add.tail = *where_0;
                *where_0 = add.head;
                (*place).os_tail = &mut (*after).next;
            } else {
                *add.tail = *(*place).stmt;
                *(*place).stmt = add.head;
            }
            if *(*stat_ptr).tail == add.head {
                (*stat_ptr).tail = add.tail;
            }
            (*place).stmt = add.tail;
            newly_added_os = *os_tail;
            *os_tail = 0 as *mut lang_output_section_statement_type;
            (*newly_added_os)
                .prev = ((*place).os_tail as *mut libc::c_char)
                .offset(-(32 as libc::c_ulong as isize))
                as *mut lang_output_section_statement_type;
            (*newly_added_os).next = *(*place).os_tail;
            if !((*newly_added_os).next).is_null() {
                (*(*newly_added_os).next).prev = newly_added_os;
            }
            *(*place).os_tail = newly_added_os;
            (*place).os_tail = &mut (*newly_added_os).next;
            if (*os_tail).is_null() {
                lang_os_list.tail = os_tail as *mut *mut lang_statement_union_type;
            }
        }
    }
    return os;
}
#[no_mangle]
pub unsafe extern "C" fn lang_add_input_file(
    mut name: *const libc::c_char,
    mut file_type: lang_input_file_enum_type,
    mut target: *const libc::c_char,
) -> *mut lang_input_statement_type {
    if !name.is_null()
        && (*name as libc::c_int == '=' as i32
            || startswith(name, b"$SYSROOT\0" as *const u8 as *const libc::c_char)
                as libc::c_int != 0)
    {
        let mut ret: *mut lang_input_statement_type = 0
            as *mut lang_input_statement_type;
        let mut sysrooted_name: *mut libc::c_char = concat(
            ld_sysroot,
            name
                .offset(
                    (if *name as libc::c_int == '=' as i32 {
                        1 as libc::c_int as libc::c_ulong
                    } else {
                        strlen(b"$SYSROOT\0" as *const u8 as *const libc::c_char)
                    }) as isize,
                ),
            0 as *mut libc::c_void as *const libc::c_char,
        );
        let mut outer_sysrooted: libc::c_uint = input_flags.sysrooted();
        input_flags.set_sysrooted(0 as libc::c_int as libc::c_uint);
        ret = new_afile(sysrooted_name, file_type, target, 0 as *const libc::c_char);
        input_flags.set_sysrooted(outer_sysrooted);
        return ret;
    }
    return new_afile(name, file_type, target, current_input_file);
}
#[no_mangle]
pub unsafe extern "C" fn lang_output_section_get(
    mut output_section: *const asection,
) -> *mut lang_output_section_statement_type {
    return bfd_section_userdata(output_section)
        as *mut lang_output_section_statement_type;
}
#[no_mangle]
pub unsafe extern "C" fn lang_output_section_statement_lookup(
    mut name: *const libc::c_char,
    mut constraint: libc::c_int,
    mut create: libc::c_int,
) -> *mut lang_output_section_statement_type {
    let mut entry: *mut out_section_hash_entry = 0 as *mut out_section_hash_entry;
    entry = bfd_hash_lookup(
        &mut output_section_statement_table,
        name,
        create != 0 as libc::c_int,
        0 as libc::c_int != 0,
    ) as *mut out_section_hash_entry;
    if entry.is_null() {
        if create != 0 {
            einfo(
                dcgettext(
                    0 as *const libc::c_char,
                    b"%F%P: failed creating section `%s': %E\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                name,
            );
        }
        return 0 as *mut lang_output_section_statement_type;
    }
    if !((*entry).s.output_section_statement.name).is_null() {
        let mut last_ent: *mut out_section_hash_entry = 0 as *mut out_section_hash_entry;
        name = (*entry).s.output_section_statement.name;
        loop {
            if create != 2 as libc::c_int
                && !(create != 0 && constraint == 382 as libc::c_int)
                && (constraint == (*entry).s.output_section_statement.constraint
                    || constraint == 0 as libc::c_int
                        && (*entry).s.output_section_statement.constraint
                            >= 0 as libc::c_int)
            {
                return &mut (*entry).s.output_section_statement;
            }
            last_ent = entry;
            entry = (*entry).root.next as *mut out_section_hash_entry;
            if !(!entry.is_null() && name == (*entry).s.output_section_statement.name) {
                break;
            }
        }
        if create == 0 {
            return 0 as *mut lang_output_section_statement_type;
        }
        entry = output_section_statement_newfunc(
            0 as *mut bfd_hash_entry,
            &mut output_section_statement_table,
            name,
        ) as *mut out_section_hash_entry;
        if entry.is_null() {
            einfo(
                dcgettext(
                    0 as *const libc::c_char,
                    b"%F%P: failed creating section `%s': %E\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                name,
            );
            return 0 as *mut lang_output_section_statement_type;
        }
        (*entry).root = (*last_ent).root;
        (*last_ent).root.next = &mut (*entry).root;
    }
    (*entry).s.output_section_statement.name = name;
    (*entry).s.output_section_statement.constraint = constraint;
    ((*entry).s.output_section_statement)
        .set_dup_output(
            (create == 2 as libc::c_int || constraint == 382 as libc::c_int)
                as libc::c_int as libc::c_uint,
        );
    return &mut (*entry).s.output_section_statement;
}
#[no_mangle]
pub unsafe extern "C" fn next_matching_output_section_statement(
    mut os: *mut lang_output_section_statement_type,
    mut constraint: libc::c_int,
) -> *mut lang_output_section_statement_type {
    let mut entry: *mut out_section_hash_entry = (os as *mut libc::c_char)
        .offset(-(24 as libc::c_ulong as isize)) as *mut out_section_hash_entry;
    let mut name: *const libc::c_char = (*os).name;
    if !(name == (*entry).root.string) {
        info_assert(
            b"ldlang.c\0" as *const u8 as *const libc::c_char,
            1548 as libc::c_int as libc::c_uint,
        );
    }
    loop {
        entry = (*entry).root.next as *mut out_section_hash_entry;
        if entry.is_null() || name != (*entry).s.output_section_statement.name {
            return 0 as *mut lang_output_section_statement_type;
        }
        if !(constraint != (*entry).s.output_section_statement.constraint
            && (constraint != 0 as libc::c_int
                || (*entry).s.output_section_statement.constraint < 0 as libc::c_int))
        {
            break;
        }
    }
    return &mut (*entry).s.output_section_statement;
}
#[no_mangle]
pub unsafe extern "C" fn ldlang_add_undef(name: *const libc::c_char, mut cmdline: bool) {
    let mut new_undef: *mut ldlang_undef_chain_list_type = 0
        as *mut ldlang_undef_chain_list_type;
    new_undef = stat_alloc(
        ::core::mem::size_of::<ldlang_undef_chain_list_type>() as libc::c_ulong,
    ) as *mut ldlang_undef_chain_list_type;
    (*new_undef).next = entry_symbol.next;
    entry_symbol.next = new_undef;
    (*new_undef).name = xstrdup(name);
    if !(link_info.output_bfd).is_null() {
        insert_undefined((*new_undef).name);
    }
}
#[no_mangle]
pub unsafe extern "C" fn ldlang_add_require_defined(name: *const libc::c_char) {
    let mut ptr: *mut require_defined_symbol = 0 as *mut require_defined_symbol;
    ldlang_add_undef(name, 1 as libc::c_int != 0);
    ptr = stat_alloc(::core::mem::size_of::<require_defined_symbol>() as libc::c_ulong)
        as *mut require_defined_symbol;
    (*ptr).next = require_defined_symbol_list;
    (*ptr).name = strdup(name);
    require_defined_symbol_list = ptr;
}
#[no_mangle]
pub unsafe extern "C" fn lang_add_output_format(
    mut format: *const libc::c_char,
    mut big: *const libc::c_char,
    mut little: *const libc::c_char,
    mut from_script: libc::c_int,
) {
    if output_target.is_null() || from_script == 0 {
        if command_line.endian as libc::c_uint
            == ENDIAN_BIG as libc::c_int as libc::c_uint && !big.is_null()
        {
            format = big;
        } else if command_line.endian as libc::c_uint
            == ENDIAN_LITTLE as libc::c_int as libc::c_uint && !little.is_null()
        {
            format = little;
        }
        output_target = format;
    }
}
#[no_mangle]
pub unsafe extern "C" fn lang_list_init(mut list: *mut lang_statement_list_type) {
    (*list).head = 0 as *mut lang_statement_union;
    (*list).tail = &mut (*list).head;
}
#[no_mangle]
pub unsafe extern "C" fn push_stat_ptr(mut new_ptr: *mut lang_statement_list_type) {
    if stat_save_ptr
        >= stat_save
            .as_mut_ptr()
            .offset(
                (::core::mem::size_of::<[*mut lang_statement_list_type; 10]>()
                    as libc::c_ulong)
                    .wrapping_div(
                        ::core::mem::size_of::<*mut lang_statement_list_type>()
                            as libc::c_ulong,
                    ) as isize,
            )
    {
        ld_abort(
            b"ldlang.c\0" as *const u8 as *const libc::c_char,
            1070 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 47],
                &[libc::c_char; 47],
            >(b"void push_stat_ptr(lang_statement_list_type *)\0"))
                .as_ptr(),
        );
    }
    let fresh1 = stat_save_ptr;
    stat_save_ptr = stat_save_ptr.offset(1);
    *fresh1 = stat_ptr;
    stat_ptr = new_ptr;
}
#[no_mangle]
pub unsafe extern "C" fn pop_stat_ptr() {
    if stat_save_ptr <= stat_save.as_mut_ptr() {
        ld_abort(
            b"ldlang.c\0" as *const u8 as *const libc::c_char,
            1079 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 24],
                &[libc::c_char; 24],
            >(b"void pop_stat_ptr(void)\0"))
                .as_ptr(),
        );
    }
    stat_save_ptr = stat_save_ptr.offset(-1);
    stat_ptr = *stat_save_ptr;
}
#[no_mangle]
pub unsafe extern "C" fn lang_add_data(
    mut type_0: libc::c_int,
    mut exp: *mut etree_union,
) {
    let mut new_stmt: *mut lang_data_statement_type = 0 as *mut lang_data_statement_type;
    new_stmt = new_statement(
        lang_data_statement_enum,
        ::core::mem::size_of::<lang_data_statement_type>() as libc::c_ulong,
        stat_ptr,
    ) as *mut lang_data_statement_type;
    (*new_stmt).exp = exp;
    (*new_stmt).type_0 = type_0 as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn lang_add_reloc(
    mut reloc: bfd_reloc_code_real_type,
    mut howto: *const reloc_howto_type,
    mut section: *mut asection,
    mut name: *const libc::c_char,
    mut addend: *mut etree_union,
) {
    let mut p: *mut lang_reloc_statement_type = new_statement(
        lang_reloc_statement_enum,
        ::core::mem::size_of::<lang_reloc_statement_type>() as libc::c_ulong,
        stat_ptr,
    ) as *mut lang_reloc_statement_type;
    (*p).reloc = reloc;
    (*p).howto = howto;
    (*p).section = section;
    (*p).name = name;
    (*p).addend_exp = addend;
    (*p).addend_value = 0 as libc::c_int as bfd_vma;
    (*p).output_section = 0 as *mut asection;
    (*p).output_offset = 0 as libc::c_int as bfd_vma;
}
#[no_mangle]
pub unsafe extern "C" fn lang_for_each_statement(
    mut func: Option::<unsafe extern "C" fn(*mut lang_statement_union_type) -> ()>,
) {
    lang_for_each_statement_worker(func, statement_list.head);
}
#[no_mangle]
pub unsafe extern "C" fn lang_for_each_statement_worker(
    mut func: Option::<unsafe extern "C" fn(*mut lang_statement_union_type) -> ()>,
    mut s: *mut lang_statement_union_type,
) {
    while !s.is_null() {
        func.expect("non-null function pointer")(s);
        match (*s).header.type_0 as libc::c_uint {
            14 => {
                lang_for_each_statement_worker(func, constructor_list.head);
            }
            8 => {
                if (*s).output_section_statement.constraint != -(1 as libc::c_int) {
                    lang_for_each_statement_worker(
                        func,
                        (*s).output_section_statement.children.head,
                    );
                }
            }
            13 => {
                lang_for_each_statement_worker(func, (*s).wild_statement.children.head);
            }
            4 => {
                lang_for_each_statement_worker(func, (*s).group_statement.children.head);
            }
            2 | 11 | 15 | 9 | 12 | 5 | 6 | 1 | 10 | 0 | 3 | 7 => {}
            _ => {
                info_assert(
                    b"ldlang.c\0" as *const u8 as *const libc::c_char,
                    1036 as libc::c_int as libc::c_uint,
                );
            }
        }
        s = (*s).header.next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn stat_alloc(mut size: size_t) -> *mut libc::c_void {
    return ({
        let mut __h: *mut obstack = &mut stat_obstack;
        let mut __o: *mut obstack = __h;
        let mut __len: size_t = size;
        if ({
            let mut __o1: *const obstack = __o;
            ((*__o1).chunk_limit).offset_from((*__o1).next_free) as libc::c_long
                as size_t
        }) < __len
        {
            _obstack_newchunk(__o, __len);
        }
        (*__o).next_free = ((*__o).next_free).offset(__len as isize);
        ({
            let mut __o1: *mut obstack = __h;
            let mut __value: *mut libc::c_void = (*__o1).object_base
                as *mut libc::c_void;
            if (*__o1).next_free == __value as *mut libc::c_char {
                (*__o1).set_maybe_empty_object(1 as libc::c_int as libc::c_uint);
            }
            (*__o1)
                .next_free = (if (::core::mem::size_of::<ptrdiff_t>() as libc::c_ulong)
                < ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
            {
                (*__o1).object_base
            } else {
                0 as *mut libc::c_char
            })
                .offset(
                    ((((*__o1).next_free)
                        .offset_from(
                            (if (::core::mem::size_of::<ptrdiff_t>() as libc::c_ulong)
                                < ::core::mem::size_of::<*mut libc::c_void>()
                                    as libc::c_ulong
                            {
                                (*__o1).object_base
                            } else {
                                0 as *mut libc::c_char
                            }),
                        ) as libc::c_long as libc::c_ulong)
                        .wrapping_add((*__o1).alignment_mask) & !(*__o1).alignment_mask)
                        as isize,
                );
            if ((*__o1).next_free).offset_from((*__o1).chunk as *mut libc::c_char)
                as libc::c_long as size_t
                > ((*__o1).chunk_limit).offset_from((*__o1).chunk as *mut libc::c_char)
                    as libc::c_long as size_t
            {
                (*__o1).next_free = (*__o1).chunk_limit;
            }
            (*__o1).object_base = (*__o1).next_free;
            __value
        })
    });
}
#[no_mangle]
pub unsafe extern "C" fn strip_excluded_output_sections() {
    let mut os: *mut lang_output_section_statement_type = 0
        as *mut lang_output_section_statement_type;
    if expld.phase as libc::c_uint != lang_mark_phase_enum as libc::c_int as libc::c_uint
    {
        expld.phase = lang_mark_phase_enum;
        expld.dataseg.phase = exp_seg_none;
        one_lang_size_sections_pass(0 as *mut bool, 0 as libc::c_int != 0);
        lang_reset_memory_regions();
    }
    os = lang_os_list.head as *mut libc::c_void
        as *mut lang_output_section_statement_type;
    while !os.is_null() {
        let mut output_section: *mut asection = 0 as *mut asection;
        let mut exclude: bool = false;
        if !((*os).constraint < 0 as libc::c_int) {
            output_section = (*os).bfd_section;
            if !output_section.is_null() {
                exclude = (*output_section).rawsize == 0 as libc::c_int as libc::c_ulong
                    && (*output_section).flags & 0x200000 as libc::c_int as libc::c_uint
                        == 0 as libc::c_int as libc::c_uint
                    && !bfd_section_removed_from_list(
                        link_info.output_bfd,
                        output_section,
                    );
                if exclude as libc::c_int != 0
                    && !((*output_section).map_head.s).is_null()
                {
                    let mut s: *mut asection = 0 as *mut asection;
                    s = (*output_section).map_head.s;
                    while !s.is_null() {
                        if (*s).flags & 0x8000 as libc::c_int as libc::c_uint
                            == 0 as libc::c_int as libc::c_uint
                            && ((*s).flags & 0x100000 as libc::c_int as libc::c_uint
                                != 0 as libc::c_int as libc::c_uint
                                || link_info.emitrelocations() as libc::c_int != 0)
                        {
                            exclude = 0 as libc::c_int != 0;
                            break;
                        } else {
                            s = (*s).map_head.s;
                        }
                    }
                }
                if exclude {
                    if (*os).update_dot() == 0 {
                        (*os).set_ignored(1 as libc::c_int as libc::c_uint);
                    }
                    (*output_section).flags |= 0x8000 as libc::c_int as libc::c_uint;
                    bfd_section_list_remove(link_info.output_bfd, output_section);
                    (*link_info.output_bfd)
                        .section_count = ((*link_info.output_bfd).section_count)
                        .wrapping_sub(1);
                    (*link_info.output_bfd).section_count;
                }
            }
        }
        os = (*os).next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn lang_clear_os_map() {
    let mut os: *mut lang_output_section_statement_type = 0
        as *mut lang_output_section_statement_type;
    if map_head_is_link_order {
        return;
    }
    os = lang_os_list.head as *mut libc::c_void
        as *mut lang_output_section_statement_type;
    while !os.is_null() {
        let mut output_section: *mut asection = 0 as *mut asection;
        if !((*os).constraint < 0 as libc::c_int) {
            output_section = (*os).bfd_section;
            if !output_section.is_null() {
                (*output_section).map_head.link_order = 0 as *mut bfd_link_order;
                (*output_section).map_tail.link_order = 0 as *mut bfd_link_order;
            }
        }
        os = (*os).next;
    }
    map_head_is_link_order = 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn dprint_statement(
    mut s: *mut lang_statement_union_type,
    mut n: libc::c_int,
) {
    let mut map_save: *mut FILE = config.map_file;
    config.map_file = stderr;
    if n < 0 as libc::c_int {
        print_statement_list(s, abs_output_section);
    } else {
        while !s.is_null()
            && {
                n -= 1;
                n >= 0 as libc::c_int
            }
        {
            print_statement(s, abs_output_section);
            s = (*s).header.next;
        }
    }
    config.map_file = map_save;
}
#[no_mangle]
pub unsafe extern "C" fn lang_size_sections(
    mut relax: *mut bool,
    mut check_regions: bool,
) {
    expld.phase = lang_allocating_phase_enum;
    expld.dataseg.phase = exp_seg_none;
    one_lang_size_sections_pass(relax, check_regions);
    if expld.dataseg.phase as libc::c_uint
        != exp_seg_end_seen as libc::c_int as libc::c_uint
    {
        expld.dataseg.phase = exp_seg_done;
    }
    if expld.dataseg.phase as libc::c_uint
        == exp_seg_end_seen as libc::c_int as libc::c_uint
    {
        let mut do_reset: bool = lang_size_relro_segment(relax, check_regions);
        if do_reset {
            lang_reset_memory_regions();
            one_lang_size_sections_pass(relax, check_regions);
        }
        if link_info.relro() as libc::c_int != 0 && expld.dataseg.relro_end != 0 {
            link_info.relro_start = expld.dataseg.base;
            link_info.relro_end = expld.dataseg.relro_end;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn one_lang_size_sections_pass(
    mut relax: *mut bool,
    mut check_regions: bool,
) {
    lang_statement_iteration += 1;
    lang_statement_iteration;
    if expld.phase as libc::c_uint != lang_mark_phase_enum as libc::c_int as libc::c_uint
    {
        lang_sizing_iteration += 1;
        lang_sizing_iteration;
    }
    lang_size_sections_1(
        &mut statement_list.head,
        abs_output_section,
        0 as *mut fill_type,
        0 as libc::c_int as bfd_vma,
        relax,
        check_regions,
    );
}
#[no_mangle]
pub unsafe extern "C" fn lang_add_insert(
    mut where_0: *const libc::c_char,
    mut is_before: libc::c_int,
) {
    let mut new_stmt: *mut lang_insert_statement_type = 0
        as *mut lang_insert_statement_type;
    new_stmt = new_statement(
        lang_insert_statement_enum,
        ::core::mem::size_of::<lang_insert_statement_type>() as libc::c_ulong,
        stat_ptr,
    ) as *mut lang_insert_statement_type;
    (*new_stmt).where_0 = where_0;
    (*new_stmt).is_before = is_before != 0;
    saved_script_handle = previous_script_handle;
}
#[no_mangle]
pub unsafe extern "C" fn lang_enter_group() {
    let mut g: *mut lang_group_statement_type = 0 as *mut lang_group_statement_type;
    g = new_statement(
        lang_group_statement_enum,
        ::core::mem::size_of::<lang_group_statement_type>() as libc::c_ulong,
        stat_ptr,
    ) as *mut lang_group_statement_type;
    lang_list_init(&mut (*g).children);
    push_stat_ptr(&mut (*g).children);
}
#[no_mangle]
pub unsafe extern "C" fn lang_leave_group() {
    pop_stat_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn lang_add_section(
    mut ptr: *mut lang_statement_list_type,
    mut section: *mut asection,
    mut pattern: *mut wildcard_list,
    mut sflag_info: *mut flag_info,
    mut output: *mut lang_output_section_statement_type,
) {
    let mut flags: flagword = (*section).flags;
    let mut discard: bool = false;
    let mut new_section: *mut lang_input_section_type = 0
        as *mut lang_input_section_type;
    let mut abfd: *mut bfd = link_info.output_bfd;
    discard = lang_discard_section_p(section);
    if strcmp((*output).name, b"/DISCARD/\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        discard = 1 as libc::c_int != 0;
    }
    if discard {
        if ((*section).output_section).is_null() {
            (*section)
                .output_section = &mut *_bfd_std_section
                .as_mut_ptr()
                .offset(2 as libc::c_int as isize) as *mut asection;
        } else if link_info.non_contiguous_regions_warnings() != 0 {
            einfo(
                dcgettext(
                    0 as *const libc::c_char,
                    b"%P:%pS: warning: --enable-non-contiguous-regions makes section `%pA' from '%pB' match /DISCARD/ clause.\n\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                0 as *mut libc::c_void,
                section,
                (*section).owner,
            );
        }
        return;
    }
    if !sflag_info.is_null() {
        let mut keep: bool = false;
        keep = (Some(
            ((*(*abfd).xvec)._bfd_lookup_section_flags)
                .expect("non-null function pointer"),
        ))
            .expect("non-null function pointer")(&mut link_info, sflag_info, section);
        if !keep {
            return;
        }
    }
    if !((*section).output_section).is_null() {
        if link_info.non_contiguous_regions() == 0 {
            return;
        }
        if bfd_is_abs_section((*section).output_section) {
            return;
        }
        if (*section).output_section == (*output).bfd_section {
            return;
        }
        if link_info.non_contiguous_regions_warnings() as libc::c_int != 0
            && !((*output).bfd_section).is_null()
        {
            einfo(
                dcgettext(
                    0 as *const libc::c_char,
                    b"%P:%pS: warning: --enable-non-contiguous-regions may change behaviour for section `%pA' from '%pB' (assigned to %pA, but additional match: %pA)\n\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                0 as *mut libc::c_void,
                section,
                (*section).owner,
                (*section).output_section,
                (*output).bfd_section,
            );
        }
    }
    flags &= !(0x200 as libc::c_int) as libc::c_uint;
    if flags & (0x20000 as libc::c_int | 0x2000000 as libc::c_int) as libc::c_uint
        == (0x20000 as libc::c_int | 0x2000000 as libc::c_int) as libc::c_uint
    {
        if link_info.resolve_section_groups() != 0 {
            flags
                &= !(0x20000 as libc::c_int | 0xc0000 as libc::c_int
                    | 0x4 as libc::c_int) as libc::c_uint;
        } else {
            flags &= !(0xc0000 as libc::c_int | 0x4 as libc::c_int) as libc::c_uint;
        }
    } else if !(link_info.type_0() as libc::c_int == type_relocatable as libc::c_int) {
        flags
            &= !(0x20000 as libc::c_int | 0xc0000 as libc::c_int | 0x4 as libc::c_int)
                as libc::c_uint;
    }
    match (*output).sectype as libc::c_uint {
        4 => {
            flags &= !(0x1 as libc::c_int) as libc::c_uint;
        }
        3 => {
            flags &= !(0x2 as libc::c_int) as libc::c_uint;
            flags |= 0x200 as libc::c_int as libc::c_uint;
            if bfd_get_flavour(link_info.output_bfd) as libc::c_uint
                == bfd_target_elf_flavour as libc::c_int as libc::c_uint
            {
                flags &= !(0x100 as libc::c_int) as libc::c_uint;
            } else {
                flags &= !(0x1 as libc::c_int) as libc::c_uint;
            }
        }
        0 | 2 | 1 | _ => {}
    }
    if ((*output).bfd_section).is_null() {
        init_os(output, flags);
    }
    (*(*output).bfd_section).flags &= flags | !(0x8 as libc::c_int) as libc::c_uint;
    if (*(*output).bfd_section).linker_has_input() != 0 {
        flags &= !(0x8 as libc::c_int) as libc::c_uint;
        if (*(*output).bfd_section).flags
            & (0x800000 as libc::c_int | 0x1000000 as libc::c_int) as libc::c_uint
            != flags
                & (0x800000 as libc::c_int | 0x1000000 as libc::c_int) as libc::c_uint
            || flags & 0x800000 as libc::c_int as libc::c_uint
                != 0 as libc::c_int as libc::c_uint
                && (*(*output).bfd_section).entsize != (*section).entsize
        {
            (*(*output).bfd_section).flags
                &= !(0x800000 as libc::c_int | 0x1000000 as libc::c_int) as libc::c_uint;
            flags
                &= !(0x800000 as libc::c_int | 0x1000000 as libc::c_int) as libc::c_uint;
        }
    }
    (*(*output).bfd_section).flags |= flags;
    if (*(*output).bfd_section).linker_has_input() == 0 {
        (*(*output).bfd_section).set_linker_has_input(1 as libc::c_int as libc::c_uint);
        (Some(
            ((*(*link_info.output_bfd).xvec)._bfd_init_private_section_data)
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            (*section).owner,
            section,
            link_info.output_bfd,
            (*output).bfd_section,
            &mut link_info,
        );
        if flags & 0x800000 as libc::c_int as libc::c_uint
            != 0 as libc::c_int as libc::c_uint
        {
            (*(*output).bfd_section).entsize = (*section).entsize;
        }
    }
    if flags & 0x10000000 as libc::c_int as libc::c_uint
        != 0 as libc::c_int as libc::c_uint
        && bfd_get_arch((*section).owner) as libc::c_uint
            == bfd_arch_tic54x as libc::c_int as libc::c_uint
    {
        (*output).block_value = 128 as libc::c_int as libc::c_uint;
    }
    if (*section).alignment_power > (*(*output).bfd_section).alignment_power {
        (*(*output).bfd_section).alignment_power = (*section).alignment_power;
    }
    (*section).output_section = (*output).bfd_section;
    if !map_head_is_link_order {
        let mut s: *mut asection = (*(*output).bfd_section).map_tail.s;
        (*(*output).bfd_section).map_tail.s = section;
        (*section).map_head.s = 0 as *mut bfd_section;
        (*section).map_tail.s = s;
        if !s.is_null() {
            (*s).map_head.s = section;
        } else {
            (*(*output).bfd_section).map_head.s = section;
        }
    }
    new_section = new_statement(
        lang_input_section_enum,
        ::core::mem::size_of::<lang_input_section_type>() as libc::c_ulong,
        ptr,
    ) as *mut lang_input_section_type;
    (*new_section).section = section;
    (*new_section).pattern = pattern as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn lang_new_phdr(
    mut name: *const libc::c_char,
    mut type_0: *mut etree_type,
    mut filehdr: bool,
    mut phdrs: bool,
    mut at: *mut etree_type,
    mut flags: *mut etree_type,
) {
    let mut n: *mut lang_phdr = 0 as *mut lang_phdr;
    let mut pp: *mut *mut lang_phdr = 0 as *mut *mut lang_phdr;
    let mut hdrs: bool = false;
    n = stat_alloc(::core::mem::size_of::<lang_phdr>() as libc::c_ulong)
        as *mut lang_phdr;
    (*n).next = 0 as *mut lang_phdr;
    (*n).name = name;
    (*n)
        .type_0 = exp_get_vma(
        type_0,
        0 as libc::c_int as bfd_vma,
        b"program header type\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    (*n).filehdr = filehdr;
    (*n).phdrs = phdrs;
    (*n).at = at;
    (*n).flags = flags;
    hdrs = (*n).type_0 == 1 as libc::c_int as libc::c_ulong
        && (phdrs as libc::c_int != 0 || filehdr as libc::c_int != 0);
    pp = &mut lang_phdr_list;
    while !(*pp).is_null() {
        if hdrs as libc::c_int != 0 && (**pp).type_0 == 1 as libc::c_int as libc::c_ulong
            && !((**pp).filehdr as libc::c_int != 0 || (**pp).phdrs as libc::c_int != 0)
        {
            einfo(
                dcgettext(
                    0 as *const libc::c_char,
                    b"%X%P:%pS: PHDRS and FILEHDR are not supported when prior PT_LOAD headers lack them\n\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                0 as *mut libc::c_void,
            );
            hdrs = 0 as libc::c_int != 0;
        }
        pp = &mut (**pp).next;
    }
    *pp = n;
}
#[no_mangle]
pub unsafe extern "C" fn lang_add_nocrossref(mut l: *mut lang_nocrossref_type) {
    let mut n: *mut lang_nocrossrefs = 0 as *mut lang_nocrossrefs;
    n = xmalloc(::core::mem::size_of::<lang_nocrossrefs>() as libc::c_ulong)
        as *mut lang_nocrossrefs;
    (*n).next = nocrossref_list;
    (*n).list = l;
    (*n).onlyfirst = 0 as libc::c_int != 0;
    nocrossref_list = n;
    link_info.set_notice_all(1 as libc::c_int as libc::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn lang_add_nocrossref_to(mut l: *mut lang_nocrossref_type) {
    lang_add_nocrossref(l);
    (*nocrossref_list).onlyfirst = 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn lang_enter_overlay(
    mut vma_expr: *mut etree_type,
    mut subalign: *mut etree_type,
) {
    if !(overlay_vma.is_null() && overlay_subalign.is_null() && overlay_max.is_null()) {
        info_assert(
            b"ldlang.c\0" as *const u8 as *const libc::c_char,
            8762 as libc::c_int as libc::c_uint,
        );
    }
    overlay_vma = vma_expr;
    overlay_subalign = subalign;
}
#[no_mangle]
pub unsafe extern "C" fn lang_enter_overlay_section(mut name: *const libc::c_char) {
    let mut n: *mut overlay_list = 0 as *mut overlay_list;
    let mut size: *mut etree_type = 0 as *mut etree_type;
    lang_enter_output_section_statement(
        name,
        overlay_vma,
        overlay_section,
        0 as *mut etree_type,
        overlay_subalign,
        0 as *mut etree_type,
        0 as libc::c_int,
        0 as libc::c_int,
    );
    if overlay_list.is_null() {
        overlay_vma = exp_nameop(323 as libc::c_int, name);
    }
    n = xmalloc(::core::mem::size_of::<overlay_list>() as libc::c_ulong)
        as *mut overlay_list;
    (*n).os = current_section;
    (*n).next = overlay_list;
    overlay_list = n;
    size = exp_nameop(321 as libc::c_int, name);
    if overlay_max.is_null() {
        overlay_max = size;
    } else {
        overlay_max = exp_binop(325 as libc::c_int, overlay_max, size);
    };
}
#[no_mangle]
pub unsafe extern "C" fn lang_leave_overlay_section(
    mut fill: *mut fill_type,
    mut phdrs: *mut lang_output_section_phdr_list,
) {
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut clean: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s1: *const libc::c_char = 0 as *const libc::c_char;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    name = (*current_section).name;
    lang_leave_output_section_statement(
        fill,
        b"*default*\0" as *const u8 as *const libc::c_char,
        phdrs,
        0 as *const libc::c_char,
    );
    clean = xmalloc((strlen(name)).wrapping_add(1 as libc::c_int as libc::c_ulong))
        as *mut libc::c_char;
    s2 = clean;
    s1 = name;
    while *s1 as libc::c_int != '\0' as i32 {
        if _sch_istable[(*s1 as libc::c_int & 0xff as libc::c_int) as usize]
            as libc::c_int & _sch_isalnum as libc::c_int as libc::c_ushort as libc::c_int
            != 0 || *s1 as libc::c_int == '_' as i32
        {
            let fresh2 = s2;
            s2 = s2.offset(1);
            *fresh2 = *s1;
        }
        s1 = s1.offset(1);
        s1;
    }
    *s2 = '\0' as i32 as libc::c_char;
    buf = xmalloc(
        (strlen(clean))
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong),
    ) as *mut libc::c_char;
    sprintf(buf, b"__load_start_%s\0" as *const u8 as *const libc::c_char, clean);
    lang_add_assignment(
        exp_provide(buf, exp_nameop(324 as libc::c_int, name), 0 as libc::c_int != 0),
    );
    buf = xmalloc(
        (strlen(clean))
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong),
    ) as *mut libc::c_char;
    sprintf(buf, b"__load_stop_%s\0" as *const u8 as *const libc::c_char, clean);
    lang_add_assignment(
        exp_provide(
            buf,
            exp_binop(
                '+' as i32,
                exp_nameop(324 as libc::c_int, name),
                exp_nameop(321 as libc::c_int, name),
            ),
            0 as libc::c_int != 0,
        ),
    );
    free(clean as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn lang_leave_overlay(
    mut lma_expr: *mut etree_type,
    mut nocrossrefs: libc::c_int,
    mut fill: *mut fill_type,
    mut memspec: *const libc::c_char,
    mut phdrs: *mut lang_output_section_phdr_list,
    mut lma_memspec: *const libc::c_char,
) {
    let mut region: *mut lang_memory_region_type = 0 as *mut lang_memory_region_type;
    let mut lma_region: *mut lang_memory_region_type = 0 as *mut lang_memory_region_type;
    let mut l: *mut overlay_list = 0 as *mut overlay_list;
    let mut nocrossref: *mut lang_nocrossref_type = 0 as *mut lang_nocrossref_type;
    lang_get_regions(
        &mut region,
        &mut lma_region,
        memspec,
        lma_memspec,
        !lma_expr.is_null(),
        0 as libc::c_int != 0,
    );
    nocrossref = 0 as *mut lang_nocrossref_type;
    if !overlay_list.is_null() {
        (*(*overlay_list).os).set_update_dot(1 as libc::c_int as libc::c_uint);
        (*(*overlay_list).os)
            .update_dot_tree = exp_assign(
            b".\0" as *const u8 as *const libc::c_char,
            exp_binop('+' as i32, overlay_vma, overlay_max),
            0 as libc::c_int != 0,
        );
    }
    l = overlay_list;
    while !l.is_null() {
        let mut next: *mut overlay_list = 0 as *mut overlay_list;
        if !fill.is_null() && ((*(*l).os).fill).is_null() {
            (*(*l).os).fill = fill;
        }
        (*(*l).os).region = region;
        (*(*l).os).lma_region = lma_region;
        if ((*l).next).is_null() {
            (*(*l).os).load_base = lma_expr;
            (*(*l).os).sectype = first_overlay_section;
        }
        if !phdrs.is_null() && ((*(*l).os).phdrs).is_null() {
            (*(*l).os).phdrs = phdrs;
        }
        if nocrossrefs != 0 {
            let mut nc: *mut lang_nocrossref_type = 0 as *mut lang_nocrossref_type;
            nc = xmalloc(::core::mem::size_of::<lang_nocrossref_type>() as libc::c_ulong)
                as *mut lang_nocrossref_type;
            (*nc).name = (*(*l).os).name;
            (*nc).next = nocrossref;
            nocrossref = nc;
        }
        next = (*l).next;
        free(l as *mut libc::c_void);
        l = next;
    }
    if !nocrossref.is_null() {
        lang_add_nocrossref(nocrossref);
    }
    overlay_vma = 0 as *mut etree_type;
    overlay_list = 0 as *mut overlay_list;
    overlay_max = 0 as *mut etree_type;
    overlay_subalign = 0 as *mut etree_type;
}
#[no_mangle]
pub unsafe extern "C" fn lang_new_vers_pattern(
    mut orig: *mut bfd_elf_version_expr,
    mut new_name: *const libc::c_char,
    mut lang: *const libc::c_char,
    mut literal_p: bool,
) -> *mut bfd_elf_version_expr {
    let mut ret: *mut bfd_elf_version_expr = 0 as *mut bfd_elf_version_expr;
    ret = xmalloc(::core::mem::size_of::<bfd_elf_version_expr>() as libc::c_ulong)
        as *mut bfd_elf_version_expr;
    (*ret).next = orig;
    (*ret).set_symver(0 as libc::c_int as libc::c_uint);
    (*ret).set_script(0 as libc::c_int as libc::c_uint);
    (*ret).set_literal(1 as libc::c_int as libc::c_uint);
    (*ret)
        .pattern = if literal_p as libc::c_int != 0 {
        new_name
    } else {
        realsymbol(new_name)
    };
    if ((*ret).pattern).is_null() {
        (*ret).pattern = new_name;
        (*ret).set_literal(0 as libc::c_int as libc::c_uint);
    }
    if lang.is_null()
        || strcasecmp(lang, b"C\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
    {
        (*ret).set_mask(1 as libc::c_int as libc::c_uint);
    } else if strcasecmp(lang, b"C++\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        (*ret).set_mask(2 as libc::c_int as libc::c_uint);
    } else if strcasecmp(lang, b"Java\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        (*ret).set_mask(4 as libc::c_int as libc::c_uint);
    } else {
        einfo(
            dcgettext(
                0 as *const libc::c_char,
                b"%X%P: unknown language `%s' in version information\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            lang,
        );
        (*ret).set_mask(1 as libc::c_int as libc::c_uint);
    }
    return ldemul_new_vers_pattern(ret);
}
#[no_mangle]
pub unsafe extern "C" fn lang_new_vers_node(
    mut globals: *mut bfd_elf_version_expr,
    mut locals: *mut bfd_elf_version_expr,
) -> *mut bfd_elf_version_tree {
    let mut ret: *mut bfd_elf_version_tree = 0 as *mut bfd_elf_version_tree;
    ret = xcalloc(
        1 as libc::c_int as size_t,
        ::core::mem::size_of::<bfd_elf_version_tree>() as libc::c_ulong,
    ) as *mut bfd_elf_version_tree;
    (*ret).globals.list = globals;
    (*ret).locals.list = locals;
    (*ret)
        .match_0 = Some(
        lang_vers_match
            as unsafe extern "C" fn(
                *mut bfd_elf_version_expr_head,
                *mut bfd_elf_version_expr,
                *const libc::c_char,
            ) -> *mut bfd_elf_version_expr,
    );
    (*ret).name_indx = -(1 as libc::c_int) as libc::c_uint;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn lang_add_vers_depend(
    mut list: *mut bfd_elf_version_deps,
    mut name: *const libc::c_char,
) -> *mut bfd_elf_version_deps {
    let mut ret: *mut bfd_elf_version_deps = 0 as *mut bfd_elf_version_deps;
    let mut t: *mut bfd_elf_version_tree = 0 as *mut bfd_elf_version_tree;
    ret = xmalloc(::core::mem::size_of::<bfd_elf_version_deps>() as libc::c_ulong)
        as *mut bfd_elf_version_deps;
    (*ret).next = list;
    t = link_info.version_info;
    while !t.is_null() {
        if strcmp((*t).name, name) == 0 as libc::c_int {
            (*ret).version_needed = t;
            return ret;
        }
        t = (*t).next;
    }
    einfo(
        dcgettext(
            0 as *const libc::c_char,
            b"%X%P: unable to find version dependency `%s'\n\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
        name,
    );
    (*ret).version_needed = 0 as *mut bfd_elf_version_tree;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn lang_register_vers_node(
    mut name: *const libc::c_char,
    mut version: *mut bfd_elf_version_tree,
    mut deps: *mut bfd_elf_version_deps,
) {
    let mut t: *mut bfd_elf_version_tree = 0 as *mut bfd_elf_version_tree;
    let mut pp: *mut *mut bfd_elf_version_tree = 0 as *mut *mut bfd_elf_version_tree;
    let mut e1: *mut bfd_elf_version_expr = 0 as *mut bfd_elf_version_expr;
    if name.is_null() {
        name = b"\0" as *const u8 as *const libc::c_char;
    }
    if !(link_info.version_info).is_null()
        && (*name.offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32
            || *((*link_info.version_info).name).offset(0 as libc::c_int as isize)
                as libc::c_int == '\0' as i32)
    {
        einfo(
            dcgettext(
                0 as *const libc::c_char,
                b"%X%P: anonymous version tag cannot be combined with other version tags\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        free(version as *mut libc::c_void);
        return;
    }
    t = link_info.version_info;
    while !t.is_null() {
        if strcmp((*t).name, name) == 0 as libc::c_int {
            einfo(
                dcgettext(
                    0 as *const libc::c_char,
                    b"%X%P: duplicate version tag `%s'\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                name,
            );
        }
        t = (*t).next;
    }
    lang_finalize_version_expr_head(&mut (*version).globals);
    lang_finalize_version_expr_head(&mut (*version).locals);
    e1 = (*version).globals.list;
    while !e1.is_null() {
        t = link_info.version_info;
        while !t.is_null() {
            let mut e2: *mut bfd_elf_version_expr = 0 as *mut bfd_elf_version_expr;
            if !((*t).locals.htab).is_null() && (*e1).literal() as libc::c_int != 0 {
                e2 = htab_find((*t).locals.htab as htab_t, e1 as *const libc::c_void)
                    as *mut bfd_elf_version_expr;
                while !e2.is_null()
                    && strcmp((*e1).pattern, (*e2).pattern) == 0 as libc::c_int
                {
                    if (*e1).mask() as libc::c_int == (*e2).mask() as libc::c_int {
                        einfo(
                            dcgettext(
                                0 as *const libc::c_char,
                                b"%X%P: duplicate expression `%s' in version information\n\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            (*e1).pattern,
                        );
                    }
                    e2 = (*e2).next;
                }
            } else if (*e1).literal() == 0 {
                e2 = (*t).locals.remaining;
                while !e2.is_null() {
                    if strcmp((*e1).pattern, (*e2).pattern) == 0 as libc::c_int
                        && (*e1).mask() as libc::c_int == (*e2).mask() as libc::c_int
                    {
                        einfo(
                            dcgettext(
                                0 as *const libc::c_char,
                                b"%X%P: duplicate expression `%s' in version information\n\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            (*e1).pattern,
                        );
                    }
                    e2 = (*e2).next;
                }
            }
            t = (*t).next;
        }
        e1 = (*e1).next;
    }
    e1 = (*version).locals.list;
    while !e1.is_null() {
        t = link_info.version_info;
        while !t.is_null() {
            let mut e2_0: *mut bfd_elf_version_expr = 0 as *mut bfd_elf_version_expr;
            if !((*t).globals.htab).is_null() && (*e1).literal() as libc::c_int != 0 {
                e2_0 = htab_find((*t).globals.htab as htab_t, e1 as *const libc::c_void)
                    as *mut bfd_elf_version_expr;
                while !e2_0.is_null()
                    && strcmp((*e1).pattern, (*e2_0).pattern) == 0 as libc::c_int
                {
                    if (*e1).mask() as libc::c_int == (*e2_0).mask() as libc::c_int {
                        einfo(
                            dcgettext(
                                0 as *const libc::c_char,
                                b"%X%P: duplicate expression `%s' in version information\n\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            (*e1).pattern,
                        );
                    }
                    e2_0 = (*e2_0).next;
                }
            } else if (*e1).literal() == 0 {
                e2_0 = (*t).globals.remaining;
                while !e2_0.is_null() {
                    if strcmp((*e1).pattern, (*e2_0).pattern) == 0 as libc::c_int
                        && (*e1).mask() as libc::c_int == (*e2_0).mask() as libc::c_int
                    {
                        einfo(
                            dcgettext(
                                0 as *const libc::c_char,
                                b"%X%P: duplicate expression `%s' in version information\n\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            (*e1).pattern,
                        );
                    }
                    e2_0 = (*e2_0).next;
                }
            }
            t = (*t).next;
        }
        e1 = (*e1).next;
    }
    (*version).deps = deps;
    (*version).name = name;
    if *name.offset(0 as libc::c_int as isize) as libc::c_int != '\0' as i32 {
        version_index += 1;
        version_index;
        (*version).vernum = version_index as libc::c_uint;
    } else {
        (*version).vernum = 0 as libc::c_int as libc::c_uint;
    }
    pp = &mut link_info.version_info;
    while !(*pp).is_null() {
        pp = &mut (**pp).next;
    }
    *pp = version;
}
#[no_mangle]
pub unsafe extern "C" fn lang_append_dynamic_list(
    mut list_p: *mut *mut bfd_elf_dynamic_list,
    mut dynamic: *mut bfd_elf_version_expr,
) {
    if !(*list_p).is_null() {
        let mut tail: *mut bfd_elf_version_expr = 0 as *mut bfd_elf_version_expr;
        tail = dynamic;
        while !((*tail).next).is_null() {
            tail = (*tail).next;
        }
        (*tail).next = (**list_p).head.list;
        (**list_p).head.list = dynamic;
    } else {
        let mut d: *mut bfd_elf_dynamic_list = 0 as *mut bfd_elf_dynamic_list;
        d = xcalloc(
            1 as libc::c_int as size_t,
            ::core::mem::size_of::<bfd_elf_dynamic_list>() as libc::c_ulong,
        ) as *mut bfd_elf_dynamic_list;
        (*d).head.list = dynamic;
        (*d)
            .match_0 = Some(
            lang_vers_match
                as unsafe extern "C" fn(
                    *mut bfd_elf_version_expr_head,
                    *mut bfd_elf_version_expr,
                    *const libc::c_char,
                ) -> *mut bfd_elf_version_expr,
        );
        *list_p = d;
    };
}
#[no_mangle]
pub unsafe extern "C" fn lang_append_dynamic_list_cpp_typeinfo() {
    let mut symbols: [*const libc::c_char; 2] = [
        b"typeinfo name for*\0" as *const u8 as *const libc::c_char,
        b"typeinfo for*\0" as *const u8 as *const libc::c_char,
    ];
    let mut dynamic: *mut bfd_elf_version_expr = 0 as *mut bfd_elf_version_expr;
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong)
        < (::core::mem::size_of::<[*const libc::c_char; 2]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
    {
        dynamic = lang_new_vers_pattern(
            dynamic,
            symbols[i as usize],
            b"C++\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int != 0,
        );
        i = i.wrapping_add(1);
        i;
    }
    lang_append_dynamic_list(&mut link_info.dynamic_list, dynamic);
}
#[no_mangle]
pub unsafe extern "C" fn lang_append_dynamic_list_cpp_new() {
    let mut symbols: [*const libc::c_char; 2] = [
        b"operator new*\0" as *const u8 as *const libc::c_char,
        b"operator delete*\0" as *const u8 as *const libc::c_char,
    ];
    let mut dynamic: *mut bfd_elf_version_expr = 0 as *mut bfd_elf_version_expr;
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong)
        < (::core::mem::size_of::<[*const libc::c_char; 2]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
    {
        dynamic = lang_new_vers_pattern(
            dynamic,
            symbols[i as usize],
            b"C++\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int != 0,
        );
        i = i.wrapping_add(1);
        i;
    }
    lang_append_dynamic_list(&mut link_info.dynamic_list, dynamic);
}
#[no_mangle]
pub unsafe extern "C" fn lang_add_unique(mut name: *const libc::c_char) {
    let mut ent: *mut unique_sections = 0 as *mut unique_sections;
    ent = unique_section_list;
    while !ent.is_null() {
        if strcmp((*ent).name, name) == 0 as libc::c_int {
            return;
        }
        ent = (*ent).next;
    }
    ent = xmalloc(::core::mem::size_of::<unique_sections>() as libc::c_ulong)
        as *mut unique_sections;
    (*ent).name = xstrdup(name);
    (*ent).next = unique_section_list;
    unique_section_list = ent;
}
#[no_mangle]
pub unsafe extern "C" fn lang_get_output_target() -> *const libc::c_char {
    let mut target: *const libc::c_char = 0 as *const libc::c_char;
    if !output_target.is_null() {
        return output_target;
    }
    if current_target != default_target as *const libc::c_char
        && !current_target.is_null()
    {
        return current_target;
    }
    target = get_first_input_target();
    if !target.is_null() {
        return target;
    }
    return default_target;
}
#[no_mangle]
pub unsafe extern "C" fn add_excluded_libs(mut list: *const libc::c_char) {
    let mut p: *const libc::c_char = list;
    let mut end: *const libc::c_char = 0 as *const libc::c_char;
    while *p as libc::c_int != '\0' as i32 {
        let mut entry: *mut excluded_lib = 0 as *mut excluded_lib;
        end = strpbrk(p, b",:\0" as *const u8 as *const libc::c_char);
        if end.is_null() {
            end = p.offset(strlen(p) as isize);
        }
        entry = xmalloc(::core::mem::size_of::<excluded_lib>() as libc::c_ulong)
            as *mut excluded_lib;
        (*entry).next = excluded_libs;
        (*entry)
            .name = xmalloc(
            (end.offset_from(p) as libc::c_long + 1 as libc::c_int as libc::c_long)
                as size_t,
        ) as *mut libc::c_char;
        memcpy(
            (*entry).name as *mut libc::c_void,
            p as *const libc::c_void,
            end.offset_from(p) as libc::c_long as libc::c_ulong,
        );
        *((*entry).name)
            .offset(
                end.offset_from(p) as libc::c_long as isize,
            ) = '\0' as i32 as libc::c_char;
        excluded_libs = entry;
        if *end as libc::c_int == '\0' as i32 {
            break;
        }
        p = end.offset(1 as libc::c_int as isize);
    }
}
#[no_mangle]
pub unsafe extern "C" fn load_symbols(
    mut entry: *mut lang_input_statement_type,
    mut place: *mut lang_statement_list_type,
) -> bool {
    let mut matching: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    if ((*entry).flags).loaded() != 0 {
        return 1 as libc::c_int != 0;
    }
    ldfile_open_file(entry);
    if ((*entry).flags).missing_file() != 0 {
        return 1 as libc::c_int != 0;
    }
    if trace_files != 0 || verbose as libc::c_int != 0 {
        info_msg(b"%pI\n\0" as *const u8 as *const libc::c_char, entry);
    }
    if !bfd_check_format((*entry).the_bfd, bfd_archive)
        && !bfd_check_format_matches((*entry).the_bfd, bfd_object, &mut matching)
    {
        let mut err: bfd_error_type = bfd_error_no_error;
        let mut save_flags: lang_input_statement_flags = lang_input_statement_flags {
            maybe_archive_full_name_provided_search_dirs_sysrooted_just_syms_dynamic_add_DT_NEEDED_for_dynamic_add_DT_NEEDED_for_regular_whole_archive_loaded_real_missing_file_reload_claimed_claim_archive_lto_output: [0; 2],
            c2rust_padding: [0; 6],
            pushed: 0 as *const lang_input_statement_flags
                as *mut lang_input_statement_flags,
        };
        extern "C" {
            static mut yyin: *mut FILE;
        }
        err = bfd_get_error();
        if ldemul_unrecognized_file(entry) {
            return 1 as libc::c_int != 0;
        }
        if err as libc::c_uint
            == bfd_error_file_ambiguously_recognized as libc::c_int as libc::c_uint
        {
            let mut p: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
            einfo(
                dcgettext(
                    0 as *const libc::c_char,
                    b"%P: %pB: file not recognized: %E; matching formats:\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*entry).the_bfd,
            );
            p = matching;
            while !(*p).is_null() {
                einfo(b" %s\0" as *const u8 as *const libc::c_char, *p);
                p = p.offset(1);
                p;
            }
            einfo(b"%F\n\0" as *const u8 as *const libc::c_char);
        } else if err as libc::c_uint
            != bfd_error_file_not_recognized as libc::c_int as libc::c_uint
            || place.is_null()
        {
            einfo(
                dcgettext(
                    0 as *const libc::c_char,
                    b"%F%P: %pB: file not recognized: %E\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*entry).the_bfd,
            );
        }
        bfd_close((*entry).the_bfd);
        (*entry).the_bfd = 0 as *mut bfd;
        save_flags = input_flags;
        ldfile_open_command_file((*entry).filename);
        push_stat_ptr(place);
        input_flags
            .set_add_DT_NEEDED_for_regular(((*entry).flags).add_DT_NEEDED_for_regular());
        input_flags
            .set_add_DT_NEEDED_for_dynamic(((*entry).flags).add_DT_NEEDED_for_dynamic());
        input_flags.set_whole_archive(((*entry).flags).whole_archive());
        input_flags.set_dynamic(((*entry).flags).dynamic());
        ldfile_assumed_script = 1 as libc::c_int != 0;
        parser_input = input_script;
        current_input_file = (*entry).filename;
        yyparse();
        current_input_file = 0 as *const libc::c_char;
        ldfile_assumed_script = 0 as libc::c_int != 0;
        save_flags
            .set_missing_file(
                save_flags.missing_file()
                    | input_flags.missing_file() as libc::c_int as libc::c_uint,
            );
        input_flags = save_flags;
        pop_stat_ptr();
        fclose(yyin);
        yyin = 0 as *mut FILE;
        ((*entry).flags).set_loaded(1 as libc::c_int as libc::c_uint);
        return 1 as libc::c_int != 0;
    }
    if ldemul_recognized_file(entry) {
        return 1 as libc::c_int != 0;
    }
    match bfd_get_format((*entry).the_bfd) as libc::c_uint {
        1 => {
            if ((*entry).flags).reload() == 0 {
                ldlang_add_file(entry);
            }
        }
        2 => {
            check_excluded_libs((*entry).the_bfd);
            bfd_set_usrdata((*entry).the_bfd, entry as *mut libc::c_void);
            if ((*entry).flags).whole_archive() != 0 {
                let mut member: *mut bfd = 0 as *mut bfd;
                let mut loaded: bool = 1 as libc::c_int != 0;
                loop {
                    let mut subsbfd: *mut bfd = 0 as *mut bfd;
                    member = bfd_openr_next_archived_file((*entry).the_bfd, member);
                    if member.is_null() {
                        break;
                    }
                    if !bfd_check_format(member, bfd_object) {
                        einfo(
                            dcgettext(
                                0 as *const libc::c_char,
                                b"%F%P: %pB: member %pB in archive is not an object\n\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            (*entry).the_bfd,
                            member,
                        );
                        loaded = 0 as libc::c_int != 0;
                    }
                    subsbfd = member;
                    if !(Some(
                        ((*link_info.callbacks).add_archive_element)
                            .expect("non-null function pointer"),
                    ))
                        .expect(
                            "non-null function pointer",
                        )(
                        &mut link_info,
                        member,
                        b"--whole-archive\0" as *const u8 as *const libc::c_char,
                        &mut subsbfd,
                    )
                    {
                        ld_abort(
                            b"ldlang.c\0" as *const u8 as *const libc::c_char,
                            3140 as libc::c_int,
                            (*::core::mem::transmute::<
                                &[u8; 76],
                                &[libc::c_char; 76],
                            >(
                                b"_Bool load_symbols(lang_input_statement_type *, lang_statement_list_type *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    if !(Some(
                        ((*(*subsbfd).xvec)._bfd_link_add_symbols)
                            .expect("non-null function pointer"),
                    ))
                        .expect("non-null function pointer")(subsbfd, &mut link_info)
                    {
                        einfo(
                            dcgettext(
                                0 as *const libc::c_char,
                                b"%F%P: %pB: error adding symbols: %E\n\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            member,
                        );
                        loaded = 0 as libc::c_int != 0;
                    }
                }
                ((*entry).flags).set_loaded(loaded as libc::c_uint);
                return loaded;
            }
        }
        _ => {}
    }
    if (Some(
        ((*(*(*entry).the_bfd).xvec)._bfd_link_add_symbols)
            .expect("non-null function pointer"),
    ))
        .expect("non-null function pointer")((*entry).the_bfd, &mut link_info)
    {
        ((*entry).flags).set_loaded(1 as libc::c_int as libc::c_uint);
    } else {
        einfo(
            dcgettext(
                0 as *const libc::c_char,
                b"%F%P: %pB: error adding symbols: %E\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            (*entry).the_bfd,
        );
    }
    return ((*entry).flags).loaded() != 0;
}
#[no_mangle]
pub unsafe extern "C" fn ldlang_ctf_acquire_strings(
    mut dynstrtab: *mut elf_strtab_hash,
) {
    ldemul_acquire_strings_for_ctf(ctf_output, dynstrtab);
}
#[no_mangle]
pub unsafe extern "C" fn ldlang_ctf_new_dynsym(
    mut symidx: libc::c_int,
    mut sym: *mut elf_internal_sym,
) {
    ldemul_new_dynsym_for_ctf(ctf_output, symidx, sym);
}
#[no_mangle]
pub unsafe extern "C" fn ldlang_write_ctf_late() {
    lang_write_ctf(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn ldlang_override_segment_assignment(
    mut info: *mut bfd_link_info,
    mut abfd: *mut bfd,
    mut current_section_0: *mut asection,
    mut previous_section: *mut asection,
    mut new_segment: bool,
) -> bool {
    let mut cur: *mut lang_output_section_statement_type = 0
        as *mut lang_output_section_statement_type;
    let mut prev: *mut lang_output_section_statement_type = 0
        as *mut lang_output_section_statement_type;
    if new_segment {
        return 1 as libc::c_int != 0;
    }
    if current_section_0.is_null() || previous_section.is_null() {
        return new_segment;
    }
    if config.separate_code as libc::c_int != 0
        && ((*current_section_0).flags ^ (*previous_section).flags)
            & 0x10 as libc::c_int as libc::c_uint != 0
    {
        return 1 as libc::c_int != 0;
    }
    cur = lang_output_section_statement_lookup(
        (*current_section_0).name,
        0 as libc::c_int,
        0 as libc::c_int,
    );
    prev = lang_output_section_statement_lookup(
        (*previous_section).name,
        0 as libc::c_int,
        0 as libc::c_int,
    );
    if cur.is_null() || prev.is_null() {
        return new_segment;
    }
    return (*cur).region != (*prev).region;
}
#[no_mangle]
pub unsafe extern "C" fn lang_ld_feature(mut str: *mut libc::c_char) {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    p = str;
    while *p != 0 {
        let mut sep: libc::c_char = 0;
        while *p as libc::c_int == ',' as i32
            || _sch_istable[(*p as libc::c_int & 0xff as libc::c_int) as usize]
                as libc::c_int
                & _sch_isspace as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            p = p.offset(1);
            p;
        }
        if *p == 0 {
            break;
        }
        q = p.offset(1 as libc::c_int as isize);
        while *q as libc::c_int != 0 && *q as libc::c_int != ',' as i32
            && _sch_istable[(*q as libc::c_int & 0xff as libc::c_int) as usize]
                as libc::c_int
                & _sch_isspace as libc::c_int as libc::c_ushort as libc::c_int == 0
        {
            q = q.offset(1);
            q;
        }
        sep = *q;
        *q = 0 as libc::c_int as libc::c_char;
        if strcasecmp(p, b"SANE_EXPR\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            config.sane_expr = 1 as libc::c_int != 0;
        } else {
            einfo(
                dcgettext(
                    0 as *const libc::c_char,
                    b"%X%P: unknown feature `%s'\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                p,
            );
        }
        *q = sep;
        p = q;
    }
}
#[no_mangle]
pub unsafe extern "C" fn lang_print_memory_usage() {
    let mut r: *mut lang_memory_region_type = 0 as *mut lang_memory_region_type;
    printf(
        b"Memory region         Used Size  Region Size  %%age Used\n\0" as *const u8
            as *const libc::c_char,
    );
    r = lang_memory_region_list;
    while !((*r).next).is_null() {
        let mut used_length: bfd_vma = ((*r).current).wrapping_sub((*r).origin);
        printf(b"%16s: \0" as *const u8 as *const libc::c_char, (*r).name_list.name);
        lang_print_memory_size(used_length);
        lang_print_memory_size((*r).length);
        if (*r).length != 0 as libc::c_int as libc::c_ulong {
            let mut percent: libc::c_double = used_length as libc::c_double * 100.0f64
                / (*r).length as libc::c_double;
            printf(b"    %6.2f%%\0" as *const u8 as *const libc::c_char, percent);
        }
        printf(b"\n\0" as *const u8 as *const libc::c_char);
        r = (*r).next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn lang_add_gc_name(mut name: *const libc::c_char) {
    let mut sym: *mut bfd_sym_chain = 0 as *mut bfd_sym_chain;
    if name.is_null() {
        return;
    }
    sym = stat_alloc(::core::mem::size_of::<bfd_sym_chain>() as libc::c_ulong)
        as *mut bfd_sym_chain;
    (*sym).next = link_info.gc_sym_list;
    (*sym).name = name;
    link_info.gc_sym_list = sym;
}
#[no_mangle]
pub unsafe extern "C" fn print_one_symbol(
    mut hash_entry: *mut bfd_link_hash_entry,
    mut ptr: *mut libc::c_void,
) -> bool {
    let mut sec: *mut asection = ptr as *mut asection;
    if ((*hash_entry).type_0() as libc::c_int == bfd_link_hash_defined as libc::c_int
        || (*hash_entry).type_0() as libc::c_int == bfd_link_hash_defweak as libc::c_int)
        && sec == (*hash_entry).u.def.section
    {
        let mut i: libc::c_int = 0;
        i = 0 as libc::c_int;
        while i < 16 as libc::c_int {
            print_space();
            i += 1;
            i;
        }
        minfo(
            b"0x%V   \0" as *const u8 as *const libc::c_char,
            ((*hash_entry).u.def.value)
                .wrapping_add((*(*hash_entry).u.def.section).output_offset)
                .wrapping_add((*(*(*hash_entry).u.def.section).output_section).vma),
        );
        minfo(
            b"             %pT\n\0" as *const u8 as *const libc::c_char,
            (*hash_entry).root.string,
        );
    }
    return 1 as libc::c_int != 0;
}
#[inline]
unsafe extern "C" fn is_elf_hash_table(mut htab: *const bfd_link_hash_table) -> bool {
    return (*htab).type_0 as libc::c_uint
        == bfd_link_elf_hash_table as libc::c_int as libc::c_uint;
}
static mut stat_obstack: obstack = obstack {
    chunk_size: 0,
    chunk: 0 as *const _obstack_chunk as *mut _obstack_chunk,
    object_base: 0 as *const libc::c_char as *mut libc::c_char,
    next_free: 0 as *const libc::c_char as *mut libc::c_char,
    chunk_limit: 0 as *const libc::c_char as *mut libc::c_char,
    temp: C2RustUnnamed_1 { i: 0 },
    alignment_mask: 0,
    chunkfun: C2RustUnnamed_0 { plain: None },
    freefun: C2RustUnnamed { plain: None },
    extra_arg: 0 as *const libc::c_void as *mut libc::c_void,
    use_extra_arg_maybe_empty_object_alloc_failed: [0; 1],
    c2rust_padding: [0; 7],
};
static mut map_obstack: obstack = obstack {
    chunk_size: 0,
    chunk: 0 as *const _obstack_chunk as *mut _obstack_chunk,
    object_base: 0 as *const libc::c_char as *mut libc::c_char,
    next_free: 0 as *const libc::c_char as *mut libc::c_char,
    chunk_limit: 0 as *const libc::c_char as *mut libc::c_char,
    temp: C2RustUnnamed_1 { i: 0 },
    alignment_mask: 0,
    chunkfun: C2RustUnnamed_0 { plain: None },
    freefun: C2RustUnnamed { plain: None },
    extra_arg: 0 as *const libc::c_void as *mut libc::c_void,
    use_extra_arg_maybe_empty_object_alloc_failed: [0; 1],
    c2rust_padding: [0; 7],
};
static mut entry_symbol_default: *const libc::c_char = b"start\0" as *const u8
    as *const libc::c_char;
static mut map_head_is_link_order: bool = 0 as libc::c_int != 0;
static mut default_common_section: *mut lang_output_section_statement_type = 0
    as *const lang_output_section_statement_type
    as *mut lang_output_section_statement_type;
static mut map_option_f: bool = false;
static mut print_dot: bfd_vma = 0;
static mut first_file: *mut lang_input_statement_type = 0
    as *const lang_input_statement_type as *mut lang_input_statement_type;
static mut current_target: *const libc::c_char = 0 as *const libc::c_char;
static mut stat_save: [*mut lang_statement_list_type; 10] = [0
    as *const lang_statement_list_type as *mut lang_statement_list_type; 10];
static mut stat_save_ptr: *mut *mut lang_statement_list_type = 0
    as *const *mut lang_statement_list_type as *mut *mut lang_statement_list_type;
static mut unique_section_list: *mut unique_sections = 0 as *const unique_sections
    as *mut unique_sections;
static mut asneeded_list_head: *mut asneeded_minfo = 0 as *const asneeded_minfo
    as *mut asneeded_minfo;
static mut opb_shift: libc::c_uint = 0 as libc::c_int as libc::c_uint;
unsafe extern "C" fn exp_init_os(mut exp: *mut etree_type) {
    match (*exp).type_0.node_class as libc::c_uint {
        4 | 5 | 6 => {
            exp_init_os((*exp).assign.src);
        }
        0 => {
            exp_init_os((*exp).binary.lhs);
            exp_init_os((*exp).binary.rhs);
        }
        1 => {
            exp_init_os((*exp).trinary.cond);
            exp_init_os((*exp).trinary.lhs);
            exp_init_os((*exp).trinary.rhs);
        }
        8 => {
            exp_init_os((*exp).assert_s.child);
        }
        2 => {
            exp_init_os((*exp).unary.child);
        }
        3 => {
            match (*exp).type_0.node_code {
                323 | 324 | 321 => {
                    let mut os: *mut lang_output_section_statement_type = 0
                        as *mut lang_output_section_statement_type;
                    os = lang_output_section_statement_lookup(
                        (*exp).name.name,
                        0 as libc::c_int,
                        0 as libc::c_int,
                    );
                    if !os.is_null() && ((*os).bfd_section).is_null() {
                        init_os(os, 0 as libc::c_int as flagword);
                    }
                }
                _ => {}
            }
        }
        _ => {}
    };
}
unsafe extern "C" fn lookup_name(
    mut name: *const libc::c_char,
) -> *mut lang_input_statement_type {
    let mut search: *mut lang_input_statement_type = 0 as *mut lang_input_statement_type;
    search = input_file_chain.head as *mut libc::c_void
        as *mut lang_input_statement_type;
    while !search.is_null() {
        let mut filename: *const libc::c_char = (*search).local_sym_name;
        if !filename.is_null() && filename_cmp(filename, name) == 0 as libc::c_int {
            break;
        }
        search = (*search).next_real_file;
    }
    if search.is_null() {
        let mut tail: *mut *mut lang_statement_union_type = (*stat_ptr).tail;
        let mut after: *mut *mut lang_statement_union_type = (input_file_chain.tail
            as *mut libc::c_char)
            .offset(-(72 as libc::c_ulong as isize))
            .offset(0 as libc::c_ulong as isize) as *mut libc::c_void
            as *mut *mut lang_statement_union_type;
        let mut rest: *mut lang_statement_union_type = *after;
        (*stat_ptr).tail = after;
        search = new_afile(
            name,
            lang_input_file_is_search_file_enum,
            default_target,
            0 as *const libc::c_char,
        );
        *(*stat_ptr).tail = rest;
        if (*tail).is_null() {
            (*stat_ptr).tail = tail;
        }
    }
    if ((*search).flags).loaded() as libc::c_int != 0 || ((*search).flags).real() == 0 {
        return search;
    }
    if !load_symbols(search, 0 as *mut lang_statement_list_type) {
        return 0 as *mut lang_input_statement_type;
    }
    return search;
}
unsafe extern "C" fn insert_undefined(mut name: *const libc::c_char) {
    let mut h: *mut bfd_link_hash_entry = 0 as *mut bfd_link_hash_entry;
    h = bfd_link_hash_lookup(
        link_info.hash,
        name,
        1 as libc::c_int != 0,
        0 as libc::c_int != 0,
        1 as libc::c_int != 0,
    );
    if h.is_null() {
        einfo(
            dcgettext(
                0 as *const libc::c_char,
                b"%F%P: bfd_link_hash_lookup failed: %E\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    if (*h).type_0() as libc::c_int == bfd_link_hash_new as libc::c_int {
        (*h).set_type_0(bfd_link_hash_undefined);
        (*h).u.undef.abfd = 0 as *mut bfd;
        (*h).set_non_ir_ref_regular(1 as libc::c_int as libc::c_uint);
        bfd_link_add_undef(link_info.hash, h);
    }
}
unsafe extern "C" fn sort_def_symbol(
    mut hash_entry: *mut bfd_link_hash_entry,
    mut info: *mut libc::c_void,
) -> bool {
    if ((*hash_entry).type_0() as libc::c_int == bfd_link_hash_defined as libc::c_int
        || (*hash_entry).type_0() as libc::c_int == bfd_link_hash_defweak as libc::c_int)
        && (*(*hash_entry).u.def.section).owner != link_info.output_bfd
        && !((*(*hash_entry).u.def.section).owner).is_null()
    {
        let mut ud: *mut input_section_userdata_type = 0
            as *mut input_section_userdata_type;
        let mut def: *mut map_symbol_def = 0 as *mut map_symbol_def;
        ud = bfd_section_userdata((*hash_entry).u.def.section)
            as *mut input_section_userdata_type;
        if ud.is_null() {
            ud = stat_alloc(
                ::core::mem::size_of::<input_section_userdata_type>() as libc::c_ulong,
            ) as *mut input_section_userdata_type;
            bfd_set_section_userdata(
                (*hash_entry).u.def.section,
                ud as *mut libc::c_void,
            );
            (*ud).map_symbol_def_tail = &mut (*ud).map_symbol_def_head;
            (*ud).map_symbol_def_count = 0 as libc::c_int as libc::c_ulong;
        } else if ((*ud).map_symbol_def_tail).is_null() {
            (*ud).map_symbol_def_tail = &mut (*ud).map_symbol_def_head;
        }
        def = ({
            let mut __h: *mut obstack = &mut map_obstack as *mut obstack;
            let mut __o: *mut obstack = __h;
            let mut __len: size_t = ::core::mem::size_of::<map_symbol_def>()
                as libc::c_ulong;
            if ({
                let mut __o1: *const obstack = __o;
                ((*__o1).chunk_limit).offset_from((*__o1).next_free) as libc::c_long
                    as size_t
            }) < __len
            {
                _obstack_newchunk(__o, __len);
            }
            (*__o).next_free = ((*__o).next_free).offset(__len as isize);
            ({
                let mut __o1: *mut obstack = __h;
                let mut __value: *mut libc::c_void = (*__o1).object_base
                    as *mut libc::c_void;
                if (*__o1).next_free == __value as *mut libc::c_char {
                    (*__o1).set_maybe_empty_object(1 as libc::c_int as libc::c_uint);
                }
                (*__o1)
                    .next_free = (if (::core::mem::size_of::<ptrdiff_t>()
                    as libc::c_ulong)
                    < ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
                {
                    (*__o1).object_base
                } else {
                    0 as *mut libc::c_char
                })
                    .offset(
                        ((((*__o1).next_free)
                            .offset_from(
                                (if (::core::mem::size_of::<ptrdiff_t>() as libc::c_ulong)
                                    < ::core::mem::size_of::<*mut libc::c_void>()
                                        as libc::c_ulong
                                {
                                    (*__o1).object_base
                                } else {
                                    0 as *mut libc::c_char
                                }),
                            ) as libc::c_long as libc::c_ulong)
                            .wrapping_add((*__o1).alignment_mask)
                            & !(*__o1).alignment_mask) as isize,
                    );
                if ((*__o1).next_free).offset_from((*__o1).chunk as *mut libc::c_char)
                    as libc::c_long as size_t
                    > ((*__o1).chunk_limit)
                        .offset_from((*__o1).chunk as *mut libc::c_char) as libc::c_long
                        as size_t
                {
                    (*__o1).next_free = (*__o1).chunk_limit;
                }
                (*__o1).object_base = (*__o1).next_free;
                __value
            })
        }) as *mut map_symbol_def;
        (*def).entry = hash_entry;
        *(*ud).map_symbol_def_tail = def;
        (*ud).map_symbol_def_tail = &mut (*def).next;
        (*ud).map_symbol_def_count = ((*ud).map_symbol_def_count).wrapping_add(1);
        (*ud).map_symbol_def_count;
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn print_statement(
    mut s: *mut lang_statement_union_type,
    mut os: *mut lang_output_section_statement_type,
) {
    match (*s).header.type_0 as libc::c_uint {
        14 => {
            if !(constructor_list.head).is_null() {
                if constructors_sorted {
                    minfo(
                        b" SORT (CONSTRUCTORS)\n\0" as *const u8 as *const libc::c_char,
                    );
                } else {
                    minfo(b" CONSTRUCTORS\n\0" as *const u8 as *const libc::c_char);
                }
                print_statement_list(constructor_list.head, os);
            }
        }
        13 => {
            print_wild_statement(&mut (*s).wild_statement, os);
        }
        0 => {
            print_address_statement(&mut (*s).address_statement);
        }
        15 => {
            minfo(b" CREATE_OBJECT_SYMBOLS\n\0" as *const u8 as *const libc::c_char);
        }
        3 => {
            print_fill_statement(&mut (*s).fill_statement);
        }
        2 => {
            print_data_statement(&mut (*s).data_statement);
        }
        11 => {
            print_reloc_statement(&mut (*s).reloc_statement);
        }
        5 => {
            print_input_section((*s).input_section.section, 0 as libc::c_int != 0);
        }
        10 => {
            print_padding_statement(&mut (*s).padding_statement);
        }
        8 => {
            print_output_section_statement(&mut (*s).output_section_statement);
        }
        1 => {
            print_assignment(&mut (*s).assignment_statement, os);
        }
        12 => {
            fprintf(
                config.map_file,
                b"TARGET(%s)\n\0" as *const u8 as *const libc::c_char,
                (*s).target_statement.target,
            );
        }
        9 => {
            minfo(
                b"OUTPUT(%s\0" as *const u8 as *const libc::c_char,
                (*s).output_statement.name,
            );
            if !output_target.is_null() {
                minfo(b" %s\0" as *const u8 as *const libc::c_char, output_target);
            }
            minfo(b")\n\0" as *const u8 as *const libc::c_char);
        }
        6 => {
            print_input_statement(&mut (*s).input_statement);
        }
        4 => {
            print_group(&mut (*s).group_statement, os);
        }
        7 => {
            minfo(
                b"INSERT %s %s\n\0" as *const u8 as *const libc::c_char,
                if (*s).insert_statement.is_before as libc::c_int != 0 {
                    b"BEFORE\0" as *const u8 as *const libc::c_char
                } else {
                    b"AFTER\0" as *const u8 as *const libc::c_char
                },
                (*s).insert_statement.where_0,
            );
        }
        _ => {
            fprintf(
                config.map_file,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Fail with %d\n\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*s).header.type_0 as libc::c_uint,
            );
            info_assert(
                b"ldlang.c\0" as *const u8 as *const libc::c_char,
                5140 as libc::c_int as libc::c_uint,
            );
        }
    };
}
unsafe extern "C" fn print_statement_list(
    mut s: *mut lang_statement_union_type,
    mut os: *mut lang_output_section_statement_type,
) {
    while !s.is_null() {
        print_statement(s, os);
        s = (*s).header.next;
    }
}
unsafe extern "C" fn print_statements() {
    print_statement_list(statement_list.head, abs_output_section);
}
unsafe extern "C" fn print_input_section(mut i: *mut asection, mut is_discarded: bool) {
    let mut size: bfd_size_type = (*i).size;
    let mut len: libc::c_int = 0;
    let mut addr: bfd_vma = 0;
    init_opb(i);
    print_space();
    minfo(b"%s\0" as *const u8 as *const libc::c_char, (*i).name);
    len = (1 as libc::c_int as libc::c_ulong).wrapping_add(strlen((*i).name))
        as libc::c_int;
    if len >= 16 as libc::c_int - 1 as libc::c_int {
        print_nl();
        len = 0 as libc::c_int;
    }
    while len < 16 as libc::c_int {
        print_space();
        len += 1;
        len;
    }
    if !((*i).output_section).is_null()
        && (*(*i).output_section).owner == link_info.output_bfd
    {
        addr = ((*(*i).output_section).vma).wrapping_add((*i).output_offset);
    } else {
        addr = print_dot;
        if !is_discarded {
            size = 0 as libc::c_int as bfd_size_type;
        }
    }
    minfo(
        b"0x%V %W %pB\n\0" as *const u8 as *const libc::c_char,
        addr,
        size >> opb_shift,
        (*i).owner,
    );
    if size != (*i).rawsize && (*i).rawsize != 0 as libc::c_int as libc::c_ulong {
        len = 16 as libc::c_int + 3 as libc::c_int;
        len += 16 as libc::c_int;
        while len > 0 as libc::c_int {
            print_space();
            len -= 1;
            len;
        }
        minfo(
            dcgettext(
                0 as *const libc::c_char,
                b"%W (size before relaxing)\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            (*i).rawsize >> opb_shift,
        );
    }
    if !((*i).output_section).is_null()
        && (*(*i).output_section).owner == link_info.output_bfd
    {
        if link_info.reduce_memory_overheads() != 0 {
            bfd_link_hash_traverse(
                link_info.hash,
                Some(
                    ldemul_print_symbol
                        as unsafe extern "C" fn(
                            *mut bfd_link_hash_entry,
                            *mut libc::c_void,
                        ) -> bool,
                ),
                i as *mut libc::c_void,
            );
        } else {
            print_all_symbols(i);
        }
        if addr.wrapping_add(size >> opb_shift) > print_dot {
            print_dot = addr.wrapping_add(size >> opb_shift);
        }
    }
}
unsafe extern "C" fn lang_one_common(
    mut h: *mut bfd_link_hash_entry,
    mut info: *mut libc::c_void,
) -> bool {
    let mut power_of_two: libc::c_uint = 0;
    let mut size: bfd_vma = 0;
    let mut section: *mut asection = 0 as *mut asection;
    if (*h).type_0() as libc::c_int != bfd_link_hash_common as libc::c_int {
        return 1 as libc::c_int != 0;
    }
    size = (*h).u.c.size;
    power_of_two = (*(*h).u.c.p).alignment_power;
    if config.sort_common as libc::c_uint
        == sort_descending as libc::c_int as libc::c_uint
        && power_of_two < *(info as *mut libc::c_uint)
    {
        return 1 as libc::c_int != 0
    } else if config.sort_common as libc::c_uint
        == sort_ascending as libc::c_int as libc::c_uint
        && power_of_two > *(info as *mut libc::c_uint)
    {
        return 1 as libc::c_int != 0
    }
    section = (*(*h).u.c.p).section;
    if !(Some(
        ((*(*link_info.output_bfd).xvec)._bfd_define_common_symbol)
            .expect("non-null function pointer"),
    ))
        .expect("non-null function pointer")(link_info.output_bfd, &mut link_info, h)
    {
        einfo(
            dcgettext(
                0 as *const libc::c_char,
                b"%F%P: could not define common symbol `%pT': %E\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            (*h).root.string,
        );
    }
    if !(config.map_file).is_null() {
        static mut header_printed: bool = false;
        let mut len: libc::c_int = 0;
        let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut buf: [libc::c_char; 50] = [0; 50];
        if !header_printed {
            minfo(
                dcgettext(
                    0 as *const libc::c_char,
                    b"\nAllocating common symbols\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            minfo(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Common symbol       size              file\n\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            header_printed = 1 as libc::c_int != 0;
        }
        name = bfd_demangle(
            link_info.output_bfd,
            (*h).root.string,
            (1 as libc::c_int) << 1 as libc::c_int
                | (1 as libc::c_int) << 0 as libc::c_int,
        );
        if name.is_null() {
            minfo(b"%s\0" as *const u8 as *const libc::c_char, (*h).root.string);
            len = strlen((*h).root.string) as libc::c_int;
        } else {
            minfo(b"%s\0" as *const u8 as *const libc::c_char, name);
            len = strlen(name) as libc::c_int;
            free(name as *mut libc::c_void);
        }
        if len >= 19 as libc::c_int {
            print_nl();
            len = 0 as libc::c_int;
        }
        while len < 20 as libc::c_int {
            print_space();
            len += 1;
            len;
        }
        minfo(b"0x\0" as *const u8 as *const libc::c_char);
        if size <= 0xffffffff as libc::c_uint as libc::c_ulong {
            sprintf(
                buf.as_mut_ptr(),
                b"%lx\0" as *const u8 as *const libc::c_char,
                size,
            );
        } else {
            sprintf(
                buf.as_mut_ptr(),
                b"%016lx\0" as *const u8 as *const libc::c_char,
                size,
            );
        }
        minfo(b"%s\0" as *const u8 as *const libc::c_char, buf.as_mut_ptr());
        len = strlen(buf.as_mut_ptr()) as libc::c_int;
        while len < 16 as libc::c_int {
            print_space();
            len += 1;
            len;
        }
        minfo(b"%pB\n\0" as *const u8 as *const libc::c_char, (*section).owner);
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn lang_record_phdrs() {
    let mut alc: libc::c_uint = 0;
    let mut secs: *mut *mut asection = 0 as *mut *mut asection;
    let mut last: *mut lang_output_section_phdr_list = 0
        as *mut lang_output_section_phdr_list;
    let mut l: *mut lang_phdr = 0 as *mut lang_phdr;
    let mut os: *mut lang_output_section_statement_type = 0
        as *mut lang_output_section_statement_type;
    alc = 10 as libc::c_int as libc::c_uint;
    secs = xmalloc(
        (alc as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut asection>() as libc::c_ulong),
    ) as *mut *mut asection;
    last = 0 as *mut lang_output_section_phdr_list;
    l = lang_phdr_list;
    while !l.is_null() {
        let mut c: libc::c_uint = 0;
        let mut flags: flagword = 0;
        let mut at: bfd_vma = 0;
        c = 0 as libc::c_int as libc::c_uint;
        let mut current_block_26: u64;
        os = lang_os_list.head as *mut libc::c_void
            as *mut lang_output_section_statement_type;
        while !os.is_null() {
            let mut pl: *mut lang_output_section_phdr_list = 0
                as *mut lang_output_section_phdr_list;
            if !((*os).constraint < 0 as libc::c_int) {
                pl = (*os).phdrs;
                if !pl.is_null() {
                    last = pl;
                    current_block_26 = 10652014663920648156;
                } else if (*os).sectype as libc::c_uint
                    == noload_section as libc::c_int as libc::c_uint
                    || ((*os).bfd_section).is_null()
                    || (*(*os).bfd_section).flags & 0x1 as libc::c_int as libc::c_uint
                        == 0 as libc::c_int as libc::c_uint
                {
                    current_block_26 = 10886091980245723256;
                } else if (*l).type_0 == 3 as libc::c_int as libc::c_ulong {
                    current_block_26 = 10886091980245723256;
                } else {
                    if last.is_null() {
                        let mut tmp_os: *mut lang_output_section_statement_type = 0
                            as *mut lang_output_section_statement_type;
                        tmp_os = os;
                        while !tmp_os.is_null() {
                            if !((*tmp_os).phdrs).is_null() {
                                last = (*tmp_os).phdrs;
                                break;
                            } else {
                                tmp_os = (*tmp_os).next;
                            }
                        }
                        if last.is_null() {
                            einfo(
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"%F%P: no sections assigned to phdrs\n\0" as *const u8
                                        as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                            );
                        }
                    }
                    pl = last;
                    current_block_26 = 10652014663920648156;
                }
                match current_block_26 {
                    10886091980245723256 => {}
                    _ => {
                        if !((*os).bfd_section).is_null() {
                            while !pl.is_null() {
                                if strcmp((*pl).name, (*l).name) == 0 as libc::c_int {
                                    if c >= alc {
                                        alc = alc.wrapping_mul(2 as libc::c_int as libc::c_uint);
                                        secs = xrealloc(
                                            secs as *mut libc::c_void,
                                            (alc as libc::c_ulong)
                                                .wrapping_mul(
                                                    ::core::mem::size_of::<*mut asection>() as libc::c_ulong,
                                                ),
                                        ) as *mut *mut asection;
                                    }
                                    let ref mut fresh3 = *secs.offset(c as isize);
                                    *fresh3 = (*os).bfd_section;
                                    c = c.wrapping_add(1);
                                    c;
                                    (*pl).used = 1 as libc::c_int != 0;
                                }
                                pl = (*pl).next;
                            }
                        }
                    }
                }
            }
            os = (*os).next;
        }
        if ((*l).flags).is_null() {
            flags = 0 as libc::c_int as flagword;
        } else {
            flags = exp_get_vma(
                (*l).flags,
                0 as libc::c_int as bfd_vma,
                b"phdr flags\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) as flagword;
        }
        if ((*l).at).is_null() {
            at = 0 as libc::c_int as bfd_vma;
        } else {
            at = exp_get_vma(
                (*l).at,
                0 as libc::c_int as bfd_vma,
                b"phdr load address\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
        if !bfd_record_phdr(
            link_info.output_bfd,
            (*l).type_0,
            !((*l).flags).is_null(),
            flags,
            !((*l).at).is_null(),
            at,
            (*l).filehdr,
            (*l).phdrs,
            c,
            secs,
        ) {
            einfo(
                dcgettext(
                    0 as *const libc::c_char,
                    b"%F%P: bfd_record_phdr failed: %E\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
        l = (*l).next;
    }
    free(secs as *mut libc::c_void);
    os = lang_os_list.head as *mut libc::c_void
        as *mut lang_output_section_statement_type;
    while !os.is_null() {
        let mut pl_0: *mut lang_output_section_phdr_list = 0
            as *mut lang_output_section_phdr_list;
        if !((*os).constraint < 0 as libc::c_int || ((*os).bfd_section).is_null()) {
            pl_0 = (*os).phdrs;
            while !pl_0.is_null() {
                if !(*pl_0).used
                    && strcmp(
                        (*pl_0).name,
                        b"NONE\0" as *const u8 as *const libc::c_char,
                    ) != 0 as libc::c_int
                {
                    einfo(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"%X%P: section `%s' assigned to non-existent phdr `%s'\n\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        (*os).name,
                        (*pl_0).name,
                    );
                }
                pl_0 = (*pl_0).next;
            }
        }
        os = (*os).next;
    }
}
unsafe extern "C" fn lang_do_version_exports_section() {
    let mut greg: *mut bfd_elf_version_expr = 0 as *mut bfd_elf_version_expr;
    let mut lreg: *mut bfd_elf_version_expr = 0 as *mut bfd_elf_version_expr;
    let mut is: *mut lang_input_statement_type = 0 as *mut lang_input_statement_type;
    is = file_chain.head as *mut lang_input_statement_type;
    while !is.is_null() {
        let mut sec: *mut asection = bfd_get_section_by_name(
            (*is).the_bfd,
            b".exports\0" as *const u8 as *const libc::c_char,
        );
        let mut contents: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut len: bfd_size_type = 0;
        if !sec.is_null() {
            len = (*sec).size;
            contents = xmalloc(len) as *mut libc::c_char;
            if !bfd_get_section_contents(
                (*is).the_bfd,
                sec,
                contents as *mut libc::c_void,
                0 as libc::c_int as file_ptr,
                len,
            ) {
                einfo(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%X%P: unable to read .exports section contents\n\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    sec,
                );
            }
            p = contents;
            while p < contents.offset(len as isize) {
                greg = lang_new_vers_pattern(
                    greg,
                    p,
                    0 as *const libc::c_char,
                    0 as libc::c_int != 0,
                );
                p = (strchr(p, '\0' as i32)).offset(1 as libc::c_int as isize);
            }
            (*sec).flags
                |= (0x8000 as libc::c_int | 0x200000 as libc::c_int) as libc::c_uint;
        }
        is = (*is).next;
    }
    lreg = lang_new_vers_pattern(
        0 as *mut bfd_elf_version_expr,
        b"*\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as libc::c_int != 0,
    );
    lang_register_vers_node(
        command_line.version_exports_section,
        lang_new_vers_node(greg, lreg),
        0 as *mut bfd_elf_version_deps,
    );
}
unsafe extern "C" fn lang_finalize_version_expr_head(
    mut head: *mut bfd_elf_version_expr_head,
) {
    let mut count: size_t = 0 as libc::c_int as size_t;
    let mut e: *mut bfd_elf_version_expr = 0 as *mut bfd_elf_version_expr;
    let mut next: *mut bfd_elf_version_expr = 0 as *mut bfd_elf_version_expr;
    let mut list_loc: *mut *mut bfd_elf_version_expr = 0
        as *mut *mut bfd_elf_version_expr;
    let mut remaining_loc: *mut *mut bfd_elf_version_expr = 0
        as *mut *mut bfd_elf_version_expr;
    e = (*head).list;
    while !e.is_null() {
        if (*e).literal() != 0 {
            count = count.wrapping_add(1);
            count;
        }
        (*head).mask |= (*e).mask();
        e = (*e).next;
    }
    if count != 0 {
        (*head)
            .htab = htab_create(
            count.wrapping_mul(2 as libc::c_int as libc::c_ulong),
            Some(
                version_expr_head_hash
                    as unsafe extern "C" fn(*const libc::c_void) -> hashval_t,
            ),
            Some(
                version_expr_head_eq
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            None,
        ) as *mut libc::c_void;
        list_loc = &mut (*head).list;
        remaining_loc = &mut (*head).remaining;
        e = (*head).list;
        while !e.is_null() {
            next = (*e).next;
            if (*e).literal() == 0 {
                *remaining_loc = e;
                remaining_loc = &mut (*e).next;
            } else {
                let mut loc: *mut *mut libc::c_void = htab_find_slot(
                    (*head).htab as htab_t,
                    e as *const libc::c_void,
                    INSERT,
                );
                if !(*loc).is_null() {
                    let mut e1: *mut bfd_elf_version_expr = 0
                        as *mut bfd_elf_version_expr;
                    let mut last: *mut bfd_elf_version_expr = 0
                        as *mut bfd_elf_version_expr;
                    e1 = *loc as *mut bfd_elf_version_expr;
                    last = 0 as *mut bfd_elf_version_expr;
                    loop {
                        if (*e1).mask() as libc::c_int == (*e).mask() as libc::c_int {
                            last = 0 as *mut bfd_elf_version_expr;
                            break;
                        } else {
                            last = e1;
                            e1 = (*e1).next;
                            if !(!e1.is_null()
                                && strcmp((*e1).pattern, (*e).pattern) == 0 as libc::c_int)
                            {
                                break;
                            }
                        }
                    }
                    if last.is_null() {
                        free(e as *mut libc::c_void);
                    } else {
                        (*e).next = (*last).next;
                        (*last).next = e;
                    }
                } else {
                    *loc = e as *mut libc::c_void;
                    *list_loc = e;
                    list_loc = &mut (*e).next;
                }
            }
            e = next;
        }
        *remaining_loc = 0 as *mut bfd_elf_version_expr;
        *list_loc = (*head).remaining;
    } else {
        (*head).remaining = (*head).list;
    };
}
unsafe extern "C" fn lang_do_memory_regions(mut update_regions_p: bool) {
    let mut r: *mut lang_memory_region_type = lang_memory_region_list;
    while !r.is_null() {
        if !((*r).origin_exp).is_null() {
            exp_fold_tree_no_dot((*r).origin_exp);
            if update_regions_p {
                if expld.result.valid_p {
                    (*r).origin = expld.result.value;
                    (*r).current = (*r).origin;
                } else {
                    einfo(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"%P: invalid origin for memory region %s\n\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        (*r).name_list.name,
                    );
                }
            }
        }
        if !((*r).length_exp).is_null() {
            exp_fold_tree_no_dot((*r).length_exp);
            if update_regions_p {
                if expld.result.valid_p {
                    (*r).length = expld.result.value;
                } else {
                    einfo(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"%P: invalid length for memory region %s\n\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        (*r).name_list.name,
                    );
                }
            }
        }
        r = (*r).next;
    }
}
static mut current_input_file: *const libc::c_char = 0 as *const libc::c_char;
#[no_mangle]
pub static mut had_output_filename: bool = 0 as libc::c_int != 0;
#[no_mangle]
pub static mut lang_float_flag: bool = 0 as libc::c_int != 0;
static mut ctf_output: *mut ctf_dict_t = 0 as *const ctf_dict_t as *mut ctf_dict_t;
static mut lang_sizing_iteration: libc::c_int = 0 as libc::c_int;
unsafe extern "C" fn name_match(
    mut pattern: *const libc::c_char,
    mut name: *const libc::c_char,
) -> libc::c_int {
    if !(strpbrk(pattern, b"?*[\0" as *const u8 as *const libc::c_char)).is_null() {
        return fnmatch(pattern, name, 0 as libc::c_int);
    }
    return strcmp(pattern, name);
}
unsafe extern "C" fn ldirname(mut name: *const libc::c_char) -> *mut libc::c_char {
    let mut base: *const libc::c_char = lbasename(name);
    let mut dirname: *mut libc::c_char = 0 as *mut libc::c_char;
    while base > name
        && (*base.offset(-(1 as libc::c_int) as isize) as libc::c_int == '/' as i32
            || *base.offset(-(1 as libc::c_int) as isize) as libc::c_int == '\\' as i32
                && 0 as libc::c_int != 0)
    {
        base = base.offset(-1);
        base;
    }
    if base == name {
        return strdup(b".\0" as *const u8 as *const libc::c_char);
    }
    dirname = strdup(name);
    *dirname
        .offset(
            base.offset_from(name) as libc::c_long as isize,
        ) = '\0' as i32 as libc::c_char;
    return dirname;
}
unsafe extern "C" fn archive_path(
    mut pattern: *const libc::c_char,
) -> *mut libc::c_char {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    if link_info.path_separator as libc::c_int == 0 as libc::c_int {
        return p;
    }
    p = strchr(pattern, link_info.path_separator as libc::c_int);
    return p;
}
unsafe extern "C" fn input_statement_is_archive_path(
    mut file_spec: *const libc::c_char,
    mut sep: *mut libc::c_char,
    mut f: *mut lang_input_statement_type,
) -> bool {
    let mut match_0: bool = 0 as libc::c_int != 0;
    if (*sep.offset(1 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int
        || name_match(sep.offset(1 as libc::c_int as isize), (*f).filename)
            == 0 as libc::c_int)
        && (sep != file_spec as *mut libc::c_char) as libc::c_int
            == (!((*f).the_bfd).is_null() && !((*(*f).the_bfd).my_archive).is_null())
                as libc::c_int
    {
        match_0 = 1 as libc::c_int != 0;
        if sep != file_spec as *mut libc::c_char {
            let mut aname: *const libc::c_char = bfd_get_filename(
                (*(*f).the_bfd).my_archive,
            );
            *sep = 0 as libc::c_int as libc::c_char;
            match_0 = name_match(file_spec, aname) == 0 as libc::c_int;
            *sep = link_info.path_separator;
        }
    }
    return match_0;
}
unsafe extern "C" fn unique_section_p(
    mut sec: *const asection,
    mut os: *const lang_output_section_statement_type,
) -> bool {
    let mut unam: *mut unique_sections = 0 as *mut unique_sections;
    let mut secnam: *const libc::c_char = 0 as *const libc::c_char;
    if link_info.resolve_section_groups() == 0 && !((*sec).owner).is_null()
        && (Some(
            ((*(*(*sec).owner).xvec)._bfd_is_group_section)
                .expect("non-null function pointer"),
        ))
            .expect("non-null function pointer")((*sec).owner, sec) as libc::c_int != 0
    {
        return !(!os.is_null()
            && strcmp((*os).name, b"/DISCARD/\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int);
    }
    secnam = (*sec).name;
    unam = unique_section_list;
    while !unam.is_null() {
        if name_match((*unam).name, secnam) == 0 as libc::c_int {
            return 1 as libc::c_int != 0;
        }
        unam = (*unam).next;
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn walk_wild_file_in_exclude_list(
    mut exclude_list: *mut name_list,
    mut file: *mut lang_input_statement_type,
) -> bool {
    let mut list_tmp: *mut name_list = 0 as *mut name_list;
    list_tmp = exclude_list;
    while !list_tmp.is_null() {
        let mut p: *mut libc::c_char = archive_path((*list_tmp).name);
        if !p.is_null() {
            if input_statement_is_archive_path((*list_tmp).name, p, file) {
                return 1 as libc::c_int != 0;
            }
        } else if name_match((*list_tmp).name, (*file).filename) == 0 as libc::c_int {
            return 1 as libc::c_int != 0
        } else if !((*file).the_bfd).is_null()
            && !((*(*file).the_bfd).my_archive).is_null()
            && name_match(
                (*list_tmp).name,
                bfd_get_filename((*(*file).the_bfd).my_archive),
            ) == 0 as libc::c_int
        {
            return 1 as libc::c_int != 0
        }
        list_tmp = (*list_tmp).next;
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn walk_wild_consider_section(
    mut ptr: *mut lang_wild_statement_type,
    mut file: *mut lang_input_statement_type,
    mut s: *mut asection,
    mut sec: *mut wildcard_list,
    mut callback: callback_t,
    mut data: *mut libc::c_void,
) {
    if walk_wild_file_in_exclude_list((*sec).spec.exclude_name_list, file) {
        return;
    }
    (Some(callback.expect("non-null function pointer")))
        .expect("non-null function pointer")(ptr, sec, s, file, data);
}
unsafe extern "C" fn walk_wild_section_general(
    mut ptr: *mut lang_wild_statement_type,
    mut file: *mut lang_input_statement_type,
    mut callback: callback_t,
    mut data: *mut libc::c_void,
) {
    let mut s: *mut asection = 0 as *mut asection;
    let mut sec: *mut wildcard_list = 0 as *mut wildcard_list;
    s = (*(*file).the_bfd).sections;
    while !s.is_null() {
        sec = (*ptr).section_list;
        if sec.is_null() {
            (Some(callback.expect("non-null function pointer")))
                .expect("non-null function pointer")(ptr, sec, s, file, data);
        }
        while !sec.is_null() {
            let mut skip: bool = 0 as libc::c_int != 0;
            if !((*sec).spec.name).is_null() {
                let mut sname: *const libc::c_char = bfd_section_name(s);
                skip = name_match((*sec).spec.name, sname) != 0 as libc::c_int;
            }
            if !skip {
                walk_wild_consider_section(ptr, file, s, sec, callback, data);
            }
            sec = (*sec).next;
        }
        s = (*s).next;
    }
}
unsafe extern "C" fn section_iterator_callback(
    mut abfd: *mut bfd,
    mut s: *mut asection,
    mut data: *mut libc::c_void,
) -> bool {
    let mut d: *mut section_iterator_callback_data = data
        as *mut section_iterator_callback_data;
    if !((*d).found_section).is_null() {
        (*d).multiple_sections_found = 1 as libc::c_int != 0;
        return 1 as libc::c_int != 0;
    }
    (*d).found_section = s;
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn find_section(
    mut file: *mut lang_input_statement_type,
    mut sec: *mut wildcard_list,
    mut multiple_sections_found: *mut bool,
) -> *mut asection {
    let mut cb_data: section_iterator_callback_data = {
        let mut init = section_iterator_callback_data {
            found_section: 0 as *mut asection,
            multiple_sections_found: 0 as libc::c_int != 0,
        };
        init
    };
    bfd_get_section_by_name_if(
        (*file).the_bfd,
        (*sec).spec.name,
        Some(
            section_iterator_callback
                as unsafe extern "C" fn(
                    *mut bfd,
                    *mut asection,
                    *mut libc::c_void,
                ) -> bool,
        ),
        &mut cb_data as *mut section_iterator_callback_data as *mut libc::c_void,
    );
    *multiple_sections_found = cb_data.multiple_sections_found;
    return cb_data.found_section;
}
unsafe extern "C" fn is_simple_wild(mut name: *const libc::c_char) -> bool {
    let mut len: size_t = strcspn(name, b"*?[\0" as *const u8 as *const libc::c_char);
    return len >= 4 as libc::c_int as libc::c_ulong
        && *name.offset(len as isize) as libc::c_int == '*' as i32
        && *name.offset(len.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            as libc::c_int == '\0' as i32;
}
unsafe extern "C" fn match_simple_wild(
    mut pattern: *const libc::c_char,
    mut name: *const libc::c_char,
) -> bool {
    if *pattern.offset(0 as libc::c_int as isize) as libc::c_int
        != *name.offset(0 as libc::c_int as isize) as libc::c_int
        || *pattern.offset(1 as libc::c_int as isize) as libc::c_int
            != *name.offset(1 as libc::c_int as isize) as libc::c_int
        || *pattern.offset(2 as libc::c_int as isize) as libc::c_int
            != *name.offset(2 as libc::c_int as isize) as libc::c_int
        || *pattern.offset(3 as libc::c_int as isize) as libc::c_int
            != *name.offset(3 as libc::c_int as isize) as libc::c_int
    {
        return 0 as libc::c_int != 0;
    }
    pattern = pattern.offset(4 as libc::c_int as isize);
    name = name.offset(4 as libc::c_int as isize);
    while *pattern as libc::c_int != '*' as i32 {
        let fresh4 = name;
        name = name.offset(1);
        let fresh5 = pattern;
        pattern = pattern.offset(1);
        if *fresh4 as libc::c_int != *fresh5 as libc::c_int {
            return 0 as libc::c_int != 0;
        }
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn get_init_priority(mut sec: *const asection) -> libc::c_int {
    let mut name: *const libc::c_char = bfd_section_name(sec);
    let mut dot: *const libc::c_char = 0 as *const libc::c_char;
    dot = strrchr(name, '.' as i32);
    if !dot.is_null()
        && _sch_istable[(*dot.offset(1 as libc::c_int as isize) as libc::c_int
            & 0xff as libc::c_int) as usize] as libc::c_int
            & _sch_isdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut init_priority: libc::c_ulong = strtoul(
            dot.offset(1 as libc::c_int as isize),
            &mut end,
            10 as libc::c_int,
        );
        if *end as libc::c_int == 0 as libc::c_int {
            if dot == name.offset(6 as libc::c_int as isize)
                && (strncmp(
                    name,
                    b".ctors\0" as *const u8 as *const libc::c_char,
                    6 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                    || strncmp(
                        name,
                        b".dtors\0" as *const u8 as *const libc::c_char,
                        6 as libc::c_int as libc::c_ulong,
                    ) == 0 as libc::c_int)
            {
                init_priority = (65535 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(init_priority);
            }
            if init_priority <= 2147483647 as libc::c_int as libc::c_ulong {
                return init_priority as libc::c_int;
            }
        }
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn compare_section(
    mut sort: sort_type,
    mut asec: *mut asection,
    mut bsec: *mut asection,
) -> libc::c_int {
    let mut current_block: u64;
    let mut ret: libc::c_int = 0;
    let mut a_priority: libc::c_int = 0;
    let mut b_priority: libc::c_int = 0;
    match sort as libc::c_uint {
        6 => {
            a_priority = get_init_priority(asec);
            b_priority = get_init_priority(bsec);
            if a_priority < 0 as libc::c_int || b_priority < 0 as libc::c_int {
                current_block = 9213593603581973221;
            } else {
                ret = a_priority - b_priority;
                if ret != 0 {
                    current_block = 12599329904712511516;
                } else {
                    current_block = 9213593603581973221;
                }
            }
        }
        4 => {
            ret = (bfd_section_alignment(bsec)).wrapping_sub(bfd_section_alignment(asec))
                as libc::c_int;
            if ret != 0 {
                current_block = 12599329904712511516;
            } else {
                current_block = 9213593603581973221;
            }
        }
        1 => {
            current_block = 9213593603581973221;
        }
        3 => {
            ret = strcmp(bfd_section_name(asec), bfd_section_name(bsec));
            if ret != 0 {
                current_block = 12599329904712511516;
            } else {
                current_block = 15031764714545269691;
            }
        }
        2 => {
            current_block = 15031764714545269691;
        }
        _ => {
            ld_abort(
                b"ldlang.c\0" as *const u8 as *const libc::c_char,
                491 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 55],
                    &[libc::c_char; 55],
                >(b"int compare_section(sort_type, asection *, asection *)\0"))
                    .as_ptr(),
            );
        }
    }
    match current_block {
        9213593603581973221 => {
            ret = strcmp(bfd_section_name(asec), bfd_section_name(bsec));
        }
        15031764714545269691 => {
            ret = (bfd_section_alignment(bsec)).wrapping_sub(bfd_section_alignment(asec))
                as libc::c_int;
        }
        _ => {}
    }
    return ret;
}
unsafe extern "C" fn wild_sort_fast(
    mut wild_0: *mut lang_wild_statement_type,
    mut sec: *mut wildcard_list,
    mut file: *mut lang_input_statement_type,
    mut section: *mut asection,
) -> *mut *mut lang_section_bst_type {
    let mut tree: *mut *mut lang_section_bst_type = 0 as *mut *mut lang_section_bst_type;
    tree = &mut (*wild_0).tree;
    if !(*wild_0).filenames_sorted
        && (sec.is_null()
            || (*sec).spec.sorted as libc::c_uint == none as libc::c_int as libc::c_uint)
    {
        while !(*tree).is_null() {
            tree = &mut (**tree).right;
        }
        return tree;
    }
    while !(*tree).is_null() {
        if compare_section((*sec).spec.sorted, section, (**tree).section)
            < 0 as libc::c_int
        {
            tree = &mut (**tree).left;
        } else {
            tree = &mut (**tree).right;
        }
    }
    return tree;
}
unsafe extern "C" fn output_section_callback_fast(
    mut ptr: *mut lang_wild_statement_type,
    mut sec: *mut wildcard_list,
    mut section: *mut asection,
    mut file: *mut lang_input_statement_type,
    mut output: *mut libc::c_void,
) {
    let mut node: *mut lang_section_bst_type = 0 as *mut lang_section_bst_type;
    let mut tree: *mut *mut lang_section_bst_type = 0 as *mut *mut lang_section_bst_type;
    let mut os: *mut lang_output_section_statement_type = 0
        as *mut lang_output_section_statement_type;
    os = output as *mut lang_output_section_statement_type;
    if unique_section_p(section, os) {
        return;
    }
    node = xmalloc(::core::mem::size_of::<lang_section_bst_type>() as libc::c_ulong)
        as *mut lang_section_bst_type;
    (*node).left = 0 as *mut lang_section_bst;
    (*node).right = 0 as *mut lang_section_bst;
    (*node).section = section;
    (*node).pattern = (*ptr).section_list as *mut libc::c_void;
    tree = wild_sort_fast(ptr, sec, file, section);
    if !tree.is_null() {
        *tree = node;
    }
}
unsafe extern "C" fn output_section_callback_tree_to_list(
    mut ptr: *mut lang_wild_statement_type,
    mut tree: *mut lang_section_bst_type,
    mut output: *mut libc::c_void,
) {
    if !((*tree).left).is_null() {
        output_section_callback_tree_to_list(ptr, (*tree).left, output);
    }
    lang_add_section(
        &mut (*ptr).children,
        (*tree).section,
        (*tree).pattern as *mut wildcard_list,
        0 as *mut flag_info,
        output as *mut lang_output_section_statement_type,
    );
    if !((*tree).right).is_null() {
        output_section_callback_tree_to_list(ptr, (*tree).right, output);
    }
    free(tree as *mut libc::c_void);
}
unsafe extern "C" fn walk_wild_section_specs1_wild0(
    mut ptr: *mut lang_wild_statement_type,
    mut file: *mut lang_input_statement_type,
    mut callback: callback_t,
    mut data: *mut libc::c_void,
) {
    let mut multiple_sections_found: bool = false;
    let mut sec0: *mut wildcard_list = (*ptr).handler_data[0 as libc::c_int as usize];
    let mut s0: *mut asection = find_section(file, sec0, &mut multiple_sections_found);
    if multiple_sections_found {
        walk_wild_section_general(ptr, file, callback, data);
    } else if !s0.is_null() {
        walk_wild_consider_section(ptr, file, s0, sec0, callback, data);
    }
}
unsafe extern "C" fn walk_wild_section_specs1_wild1(
    mut ptr: *mut lang_wild_statement_type,
    mut file: *mut lang_input_statement_type,
    mut callback: callback_t,
    mut data: *mut libc::c_void,
) {
    let mut s: *mut asection = 0 as *mut asection;
    let mut wildsec0: *mut wildcard_list = (*ptr)
        .handler_data[0 as libc::c_int as usize];
    s = (*(*file).the_bfd).sections;
    while !s.is_null() {
        let mut sname: *const libc::c_char = bfd_section_name(s);
        let mut skip: bool = !match_simple_wild((*wildsec0).spec.name, sname);
        if !skip {
            walk_wild_consider_section(ptr, file, s, wildsec0, callback, data);
        }
        s = (*s).next;
    }
}
unsafe extern "C" fn walk_wild_section_specs2_wild1(
    mut ptr: *mut lang_wild_statement_type,
    mut file: *mut lang_input_statement_type,
    mut callback: callback_t,
    mut data: *mut libc::c_void,
) {
    let mut s: *mut asection = 0 as *mut asection;
    let mut sec0: *mut wildcard_list = (*ptr).handler_data[0 as libc::c_int as usize];
    let mut wildsec1: *mut wildcard_list = (*ptr)
        .handler_data[1 as libc::c_int as usize];
    let mut multiple_sections_found: bool = false;
    let mut s0: *mut asection = find_section(file, sec0, &mut multiple_sections_found);
    if multiple_sections_found {
        walk_wild_section_general(ptr, file, callback, data);
        return;
    }
    s = (*(*file).the_bfd).sections;
    while !s.is_null() {
        if s == s0 {
            walk_wild_consider_section(ptr, file, s, sec0, callback, data);
        } else {
            let mut sname: *const libc::c_char = bfd_section_name(s);
            let mut skip: bool = !match_simple_wild((*wildsec1).spec.name, sname);
            if !skip {
                walk_wild_consider_section(ptr, file, s, wildsec1, callback, data);
            }
        }
        s = (*s).next;
    }
}
unsafe extern "C" fn walk_wild_section_specs3_wild2(
    mut ptr: *mut lang_wild_statement_type,
    mut file: *mut lang_input_statement_type,
    mut callback: callback_t,
    mut data: *mut libc::c_void,
) {
    let mut s: *mut asection = 0 as *mut asection;
    let mut sec0: *mut wildcard_list = (*ptr).handler_data[0 as libc::c_int as usize];
    let mut wildsec1: *mut wildcard_list = (*ptr)
        .handler_data[1 as libc::c_int as usize];
    let mut wildsec2: *mut wildcard_list = (*ptr)
        .handler_data[2 as libc::c_int as usize];
    let mut multiple_sections_found: bool = false;
    let mut s0: *mut asection = find_section(file, sec0, &mut multiple_sections_found);
    if multiple_sections_found {
        walk_wild_section_general(ptr, file, callback, data);
        return;
    }
    s = (*(*file).the_bfd).sections;
    while !s.is_null() {
        if s == s0 {
            walk_wild_consider_section(ptr, file, s, sec0, callback, data);
        } else {
            let mut sname: *const libc::c_char = bfd_section_name(s);
            let mut skip: bool = !match_simple_wild((*wildsec1).spec.name, sname);
            if !skip {
                walk_wild_consider_section(ptr, file, s, wildsec1, callback, data);
            } else {
                skip = !match_simple_wild((*wildsec2).spec.name, sname);
                if !skip {
                    walk_wild_consider_section(ptr, file, s, wildsec2, callback, data);
                }
            }
        }
        s = (*s).next;
    }
}
unsafe extern "C" fn walk_wild_section_specs4_wild2(
    mut ptr: *mut lang_wild_statement_type,
    mut file: *mut lang_input_statement_type,
    mut callback: callback_t,
    mut data: *mut libc::c_void,
) {
    let mut s: *mut asection = 0 as *mut asection;
    let mut sec0: *mut wildcard_list = (*ptr).handler_data[0 as libc::c_int as usize];
    let mut sec1: *mut wildcard_list = (*ptr).handler_data[1 as libc::c_int as usize];
    let mut wildsec2: *mut wildcard_list = (*ptr)
        .handler_data[2 as libc::c_int as usize];
    let mut wildsec3: *mut wildcard_list = (*ptr)
        .handler_data[3 as libc::c_int as usize];
    let mut multiple_sections_found: bool = false;
    let mut s0: *mut asection = find_section(file, sec0, &mut multiple_sections_found);
    let mut s1: *mut asection = 0 as *mut asection;
    if multiple_sections_found {
        walk_wild_section_general(ptr, file, callback, data);
        return;
    }
    s1 = find_section(file, sec1, &mut multiple_sections_found);
    if multiple_sections_found {
        walk_wild_section_general(ptr, file, callback, data);
        return;
    }
    s = (*(*file).the_bfd).sections;
    while !s.is_null() {
        if s == s0 {
            walk_wild_consider_section(ptr, file, s, sec0, callback, data);
        } else if s == s1 {
            walk_wild_consider_section(ptr, file, s, sec1, callback, data);
        } else {
            let mut sname: *const libc::c_char = bfd_section_name(s);
            let mut skip: bool = !match_simple_wild((*wildsec2).spec.name, sname);
            if !skip {
                walk_wild_consider_section(ptr, file, s, wildsec2, callback, data);
            } else {
                skip = !match_simple_wild((*wildsec3).spec.name, sname);
                if !skip {
                    walk_wild_consider_section(ptr, file, s, wildsec3, callback, data);
                }
            }
        }
        s = (*s).next;
    }
}
unsafe extern "C" fn walk_wild_section(
    mut ptr: *mut lang_wild_statement_type,
    mut file: *mut lang_input_statement_type,
    mut callback: callback_t,
    mut data: *mut libc::c_void,
) {
    if ((*file).flags).just_syms() != 0 {
        return;
    }
    (Some(((*ptr).walk_wild_section_handler).expect("non-null function pointer")))
        .expect("non-null function pointer")(ptr, file, callback, data);
}
unsafe extern "C" fn wild_spec_can_overlap(
    mut name1: *const libc::c_char,
    mut name2: *const libc::c_char,
) -> bool {
    let mut prefix1_len: size_t = strcspn(
        name1,
        b"?*[\0" as *const u8 as *const libc::c_char,
    );
    let mut prefix2_len: size_t = strcspn(
        name2,
        b"?*[\0" as *const u8 as *const libc::c_char,
    );
    let mut min_prefix_len: size_t = 0;
    if *name1.offset(prefix1_len as isize) as libc::c_int == '\0' as i32 {
        prefix1_len = prefix1_len.wrapping_add(1);
        prefix1_len;
    }
    if *name2.offset(prefix2_len as isize) as libc::c_int == '\0' as i32 {
        prefix2_len = prefix2_len.wrapping_add(1);
        prefix2_len;
    }
    min_prefix_len = if prefix1_len < prefix2_len { prefix1_len } else { prefix2_len };
    return memcmp(
        name1 as *const libc::c_void,
        name2 as *const libc::c_void,
        min_prefix_len,
    ) == 0 as libc::c_int;
}
unsafe extern "C" fn analyze_walk_wild_section_handler(
    mut ptr: *mut lang_wild_statement_type,
) {
    let mut sec_count: libc::c_int = 0 as libc::c_int;
    let mut wild_name_count: libc::c_int = 0 as libc::c_int;
    let mut sec: *mut wildcard_list = 0 as *mut wildcard_list;
    let mut signature: libc::c_int = 0;
    let mut data_counter: libc::c_int = 0;
    (*ptr)
        .walk_wild_section_handler = Some(
        walk_wild_section_general
            as unsafe extern "C" fn(
                *mut lang_wild_statement_type,
                *mut lang_input_statement_type,
                callback_t,
                *mut libc::c_void,
            ) -> (),
    );
    (*ptr).handler_data[0 as libc::c_int as usize] = 0 as *mut wildcard_list;
    (*ptr).handler_data[1 as libc::c_int as usize] = 0 as *mut wildcard_list;
    (*ptr).handler_data[2 as libc::c_int as usize] = 0 as *mut wildcard_list;
    (*ptr).handler_data[3 as libc::c_int as usize] = 0 as *mut wildcard_list;
    (*ptr).tree = 0 as *mut lang_section_bst_type;
    sec = (*ptr).section_list;
    while !sec.is_null() {
        sec_count += 1;
        sec_count;
        if ((*sec).spec.name).is_null() {
            return;
        }
        if !(strpbrk((*sec).spec.name, b"?*[\0" as *const u8 as *const libc::c_char))
            .is_null()
        {
            wild_name_count += 1;
            wild_name_count;
            if !is_simple_wild((*sec).spec.name) {
                return;
            }
        }
        sec = (*sec).next;
    }
    if sec_count == 0 as libc::c_int || sec_count > 4 as libc::c_int {
        return;
    }
    sec = (*ptr).section_list;
    while !sec.is_null() {
        let mut sec2: *mut wildcard_list = 0 as *mut wildcard_list;
        sec2 = (*sec).next;
        while !sec2.is_null() {
            if wild_spec_can_overlap((*sec).spec.name, (*sec2).spec.name) {
                return;
            }
            sec2 = (*sec2).next;
        }
        sec = (*sec).next;
    }
    signature = (sec_count << 8 as libc::c_int) + wild_name_count;
    match signature {
        256 => {
            (*ptr)
                .walk_wild_section_handler = Some(
                walk_wild_section_specs1_wild0
                    as unsafe extern "C" fn(
                        *mut lang_wild_statement_type,
                        *mut lang_input_statement_type,
                        callback_t,
                        *mut libc::c_void,
                    ) -> (),
            );
        }
        257 => {
            (*ptr)
                .walk_wild_section_handler = Some(
                walk_wild_section_specs1_wild1
                    as unsafe extern "C" fn(
                        *mut lang_wild_statement_type,
                        *mut lang_input_statement_type,
                        callback_t,
                        *mut libc::c_void,
                    ) -> (),
            );
        }
        513 => {
            (*ptr)
                .walk_wild_section_handler = Some(
                walk_wild_section_specs2_wild1
                    as unsafe extern "C" fn(
                        *mut lang_wild_statement_type,
                        *mut lang_input_statement_type,
                        callback_t,
                        *mut libc::c_void,
                    ) -> (),
            );
        }
        770 => {
            (*ptr)
                .walk_wild_section_handler = Some(
                walk_wild_section_specs3_wild2
                    as unsafe extern "C" fn(
                        *mut lang_wild_statement_type,
                        *mut lang_input_statement_type,
                        callback_t,
                        *mut libc::c_void,
                    ) -> (),
            );
        }
        1026 => {
            (*ptr)
                .walk_wild_section_handler = Some(
                walk_wild_section_specs4_wild2
                    as unsafe extern "C" fn(
                        *mut lang_wild_statement_type,
                        *mut lang_input_statement_type,
                        callback_t,
                        *mut libc::c_void,
                    ) -> (),
            );
        }
        _ => return,
    }
    data_counter = 0 as libc::c_int;
    sec = (*ptr).section_list;
    while !sec.is_null() {
        if (strpbrk((*sec).spec.name, b"?*[\0" as *const u8 as *const libc::c_char))
            .is_null()
        {
            let fresh6 = data_counter;
            data_counter = data_counter + 1;
            (*ptr).handler_data[fresh6 as usize] = sec;
        }
        sec = (*sec).next;
    }
    sec = (*ptr).section_list;
    while !sec.is_null() {
        if !(strpbrk((*sec).spec.name, b"?*[\0" as *const u8 as *const libc::c_char))
            .is_null()
        {
            let fresh7 = data_counter;
            data_counter = data_counter + 1;
            (*ptr).handler_data[fresh7 as usize] = sec;
        }
        sec = (*sec).next;
    }
}
unsafe extern "C" fn walk_wild_file(
    mut s: *mut lang_wild_statement_type,
    mut f: *mut lang_input_statement_type,
    mut callback: callback_t,
    mut data: *mut libc::c_void,
) {
    if walk_wild_file_in_exclude_list((*s).exclude_name_list, f) {
        return;
    }
    if ((*f).the_bfd).is_null() || !bfd_check_format((*f).the_bfd, bfd_archive) {
        walk_wild_section(s, f, callback, data);
    } else {
        let mut member: *mut bfd = 0 as *mut bfd;
        member = bfd_openr_next_archived_file((*f).the_bfd, 0 as *mut bfd);
        while !member.is_null() {
            if !(bfd_usrdata(member)).is_null() {
                walk_wild_section(
                    s,
                    bfd_usrdata(member) as *mut lang_input_statement_type,
                    callback,
                    data,
                );
            }
            member = bfd_openr_next_archived_file((*f).the_bfd, member);
        }
    };
}
unsafe extern "C" fn walk_wild(
    mut s: *mut lang_wild_statement_type,
    mut callback: callback_t,
    mut data: *mut libc::c_void,
) {
    let mut file_spec: *const libc::c_char = (*s).filename;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    if file_spec.is_null() {
        let mut f: *mut lang_input_statement_type = 0 as *mut lang_input_statement_type;
        f = file_chain.head as *mut lang_input_statement_type;
        while !f.is_null() {
            walk_wild_file(s, f, callback, data);
            f = (*f).next;
        }
    } else {
        p = archive_path(file_spec);
        if !p.is_null() {
            let mut f_0: *mut lang_input_statement_type = 0
                as *mut lang_input_statement_type;
            f_0 = file_chain.head as *mut lang_input_statement_type;
            while !f_0.is_null() {
                if input_statement_is_archive_path(file_spec, p, f_0) {
                    walk_wild_file(s, f_0, callback, data);
                }
                f_0 = (*f_0).next;
            }
        } else if !(strpbrk(file_spec, b"?*[\0" as *const u8 as *const libc::c_char))
            .is_null()
        {
            let mut f_1: *mut lang_input_statement_type = 0
                as *mut lang_input_statement_type;
            f_1 = file_chain.head as *mut lang_input_statement_type;
            while !f_1.is_null() {
                if fnmatch(file_spec, (*f_1).filename, 0 as libc::c_int)
                    == 0 as libc::c_int
                {
                    walk_wild_file(s, f_1, callback, data);
                }
                f_1 = (*f_1).next;
            }
        } else {
            let mut f_2: *mut lang_input_statement_type = 0
                as *mut lang_input_statement_type;
            f_2 = lookup_name(file_spec);
            if !f_2.is_null() {
                walk_wild_file(s, f_2, callback, data);
            }
        }
    };
}
unsafe extern "C" fn lang_statement_append(
    mut list: *mut lang_statement_list_type,
    mut element: *mut libc::c_void,
    mut field: *mut libc::c_void,
) {
    *(*list).tail = element as *mut lang_statement_union;
    (*list).tail = field as *mut *mut lang_statement_union;
}
unsafe extern "C" fn new_statement(
    mut type_0: statement_enum,
    mut size: size_t,
    mut list: *mut lang_statement_list_type,
) -> *mut lang_statement_union_type {
    let mut new_stmt: *mut lang_statement_union_type = 0
        as *mut lang_statement_union_type;
    new_stmt = stat_alloc(size) as *mut lang_statement_union_type;
    (*new_stmt).header.type_0 = type_0;
    (*new_stmt).header.next = 0 as *mut lang_statement_union;
    lang_statement_append(
        list,
        new_stmt as *mut libc::c_void,
        &mut (*new_stmt).header.next as *mut *mut lang_statement_union
            as *mut libc::c_void,
    );
    return new_stmt;
}
unsafe extern "C" fn new_afile(
    mut name: *const libc::c_char,
    mut file_type: lang_input_file_enum_type,
    mut target: *const libc::c_char,
    mut from_filename: *const libc::c_char,
) -> *mut lang_input_statement_type {
    let mut p: *mut lang_input_statement_type = 0 as *mut lang_input_statement_type;
    lang_has_input_file = 1 as libc::c_int != 0;
    p = new_statement(
        lang_input_statement_enum,
        ::core::mem::size_of::<lang_input_statement_type>() as libc::c_ulong,
        stat_ptr,
    ) as *mut lang_input_statement_type;
    memset(
        &mut (*p).the_bfd as *mut *mut bfd as *mut libc::c_void,
        0 as libc::c_int,
        (::core::mem::size_of::<lang_input_statement_type>() as libc::c_ulong)
            .wrapping_sub(40 as libc::c_ulong),
    );
    (*p).extra_search_path = 0 as *const libc::c_char;
    (*p).target = target;
    ((*p).flags).set_dynamic(input_flags.dynamic());
    ((*p).flags).set_add_DT_NEEDED_for_dynamic(input_flags.add_DT_NEEDED_for_dynamic());
    ((*p).flags).set_add_DT_NEEDED_for_regular(input_flags.add_DT_NEEDED_for_regular());
    ((*p).flags).set_whole_archive(input_flags.whole_archive());
    ((*p).flags).set_sysrooted(input_flags.sysrooted());
    match file_type as libc::c_uint {
        1 => {
            (*p).filename = name;
            (*p).local_sym_name = name;
            ((*p).flags).set_real(1 as libc::c_int as libc::c_uint);
            ((*p).flags).set_just_syms(1 as libc::c_int as libc::c_uint);
        }
        3 => {
            (*p).filename = name;
            (*p).local_sym_name = name;
        }
        0 => {
            if *name.offset(0 as libc::c_int as isize) as libc::c_int == ':' as i32
                && *name.offset(1 as libc::c_int as isize) as libc::c_int != '\0' as i32
            {
                (*p).filename = name.offset(1 as libc::c_int as isize);
                ((*p).flags).set_full_name_provided(1 as libc::c_int as libc::c_uint);
            } else {
                (*p).filename = name;
            }
            (*p)
                .local_sym_name = concat(
                b"-l\0" as *const u8 as *const libc::c_char,
                name,
                0 as *mut libc::c_void as *const libc::c_char,
            );
            ((*p).flags).set_maybe_archive(1 as libc::c_int as libc::c_uint);
            ((*p).flags).set_real(1 as libc::c_int as libc::c_uint);
            ((*p).flags).set_search_dirs(1 as libc::c_int as libc::c_uint);
        }
        2 => {
            (*p).filename = name;
            (*p).local_sym_name = name;
            ((*p).flags).set_search_dirs(1 as libc::c_int as libc::c_uint);
        }
        4 => {
            (*p).filename = name;
            (*p).local_sym_name = name;
            if !from_filename.is_null()
                && !(*name.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32
                    || *name.offset(0 as libc::c_int as isize) as libc::c_int
                        == '\\' as i32 && 0 as libc::c_int != 0
                    || *name.offset(0 as libc::c_int as isize) as libc::c_int != 0
                        && *name.offset(1 as libc::c_int as isize) as libc::c_int
                            == ':' as i32 && 0 as libc::c_int != 0)
            {
                (*p).extra_search_path = ldirname(from_filename);
            }
            ((*p).flags).set_real(1 as libc::c_int as libc::c_uint);
            ((*p).flags).set_search_dirs(1 as libc::c_int as libc::c_uint);
        }
        5 => {
            (*p).filename = name;
            (*p).local_sym_name = name;
            ((*p).flags).set_real(1 as libc::c_int as libc::c_uint);
        }
        _ => {
            info_assert(
                b"ldlang.c\0" as *const u8 as *const libc::c_char,
                1175 as libc::c_int as libc::c_uint,
            );
        }
    }
    lang_statement_append(
        &mut input_file_chain,
        p as *mut libc::c_void,
        &mut (*p).next_real_file as *mut *mut lang_input_statement_struct
            as *mut libc::c_void,
    );
    return p;
}
static mut output_section_statement_table: bfd_hash_table = bfd_hash_table {
    table: 0 as *const *mut bfd_hash_entry as *mut *mut bfd_hash_entry,
    newfunc: None,
    memory: 0 as *const libc::c_void as *mut libc::c_void,
    size: 0,
    count: 0,
    entsize: 0,
    frozen: [0; 1],
    c2rust_padding: [0; 3],
};
unsafe extern "C" fn output_section_statement_newfunc(
    mut entry: *mut bfd_hash_entry,
    mut table: *mut bfd_hash_table,
    mut string: *const libc::c_char,
) -> *mut bfd_hash_entry {
    let mut nextp: *mut *mut lang_output_section_statement_type = 0
        as *mut *mut lang_output_section_statement_type;
    let mut ret: *mut out_section_hash_entry = 0 as *mut out_section_hash_entry;
    if entry.is_null() {
        entry = bfd_hash_allocate(
            table,
            ::core::mem::size_of::<out_section_hash_entry>() as libc::c_ulong
                as libc::c_uint,
        ) as *mut bfd_hash_entry;
        if entry.is_null() {
            return entry;
        }
    }
    entry = bfd_hash_newfunc(entry, table, string);
    if entry.is_null() {
        return entry;
    }
    ret = entry as *mut out_section_hash_entry;
    memset(
        &mut (*ret).s as *mut lang_statement_union_type as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<lang_statement_union_type>() as libc::c_ulong,
    );
    (*ret).s.header.type_0 = lang_output_section_statement_enum;
    (*ret).s.output_section_statement.subsection_alignment = 0 as *mut etree_union;
    (*ret).s.output_section_statement.section_alignment = 0 as *mut etree_union;
    (*ret).s.output_section_statement.block_value = 1 as libc::c_int as libc::c_uint;
    lang_list_init(&mut (*ret).s.output_section_statement.children);
    lang_statement_append(
        stat_ptr,
        &mut (*ret).s as *mut lang_statement_union_type as *mut libc::c_void,
        &mut (*ret).s.header.next as *mut *mut lang_statement_union as *mut libc::c_void,
    );
    if !(lang_os_list.head).is_null() {
        (*ret)
            .s
            .output_section_statement
            .prev = (lang_os_list.tail as *mut libc::c_char)
            .offset(-(32 as libc::c_ulong as isize))
            as *mut lang_output_section_statement_type;
    }
    nextp = &mut (*ret).s.output_section_statement.next;
    lang_statement_append(
        &mut lang_os_list,
        &mut (*ret).s as *mut lang_statement_union_type as *mut libc::c_void,
        nextp as *mut libc::c_void,
    );
    return &mut (*ret).root;
}
unsafe extern "C" fn output_section_statement_table_init() {
    if !bfd_hash_table_init_n(
        &mut output_section_statement_table,
        Some(
            output_section_statement_newfunc
                as unsafe extern "C" fn(
                    *mut bfd_hash_entry,
                    *mut bfd_hash_table,
                    *const libc::c_char,
                ) -> *mut bfd_hash_entry,
        ),
        ::core::mem::size_of::<out_section_hash_entry>() as libc::c_ulong
            as libc::c_uint,
        61 as libc::c_int as libc::c_uint,
    ) {
        einfo(
            dcgettext(
                0 as *const libc::c_char,
                b"%F%P: can not create hash table: %E\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
}
unsafe extern "C" fn output_section_statement_table_free() {
    bfd_hash_table_free(&mut output_section_statement_table);
}
static mut lang_memory_region_list: *mut lang_memory_region_type = 0
    as *const lang_memory_region_type as *mut lang_memory_region_type;
static mut lang_memory_region_list_tail: *mut *mut lang_memory_region_type = unsafe {
    &lang_memory_region_list as *const *mut lang_memory_region_type
        as *mut *mut lang_memory_region_type
};
unsafe extern "C" fn lang_memory_default(
    mut section: *mut asection,
) -> *mut lang_memory_region_type {
    let mut p: *mut lang_memory_region_type = 0 as *mut lang_memory_region_type;
    let mut sec_flags: flagword = (*section).flags;
    if sec_flags
        & (0x1 as libc::c_int | 0x8 as libc::c_int | 0x10 as libc::c_int) as libc::c_uint
        == 0x1 as libc::c_int as libc::c_uint
    {
        sec_flags |= 0x20 as libc::c_int as libc::c_uint;
    }
    p = lang_memory_region_list;
    while !p.is_null() {
        if (*p).flags & sec_flags != 0 as libc::c_int as libc::c_uint
            && (*p).not_flags & sec_flags == 0 as libc::c_int as libc::c_uint
        {
            return p;
        }
        p = (*p).next;
    }
    return lang_memory_region_lookup(
        b"*default*\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int != 0,
    );
}
unsafe extern "C" fn output_prev_sec_find(
    mut os: *mut lang_output_section_statement_type,
) -> *mut asection {
    let mut lookup: *mut lang_output_section_statement_type = 0
        as *mut lang_output_section_statement_type;
    lookup = (*os).prev;
    while !lookup.is_null() {
        if !((*lookup).constraint < 0 as libc::c_int) {
            if !((*lookup).bfd_section).is_null()
                && !((*(*lookup).bfd_section).owner).is_null()
            {
                return (*lookup).bfd_section;
            }
        }
        lookup = (*lookup).prev;
    }
    return 0 as *mut asection;
}
unsafe extern "C" fn insert_os_after(
    mut after: *mut lang_output_section_statement_type,
) -> *mut *mut lang_statement_union_type {
    let mut where_0: *mut *mut lang_statement_union_type = 0
        as *mut *mut lang_statement_union_type;
    let mut assign: *mut *mut lang_statement_union_type = 0
        as *mut *mut lang_statement_union_type;
    let mut ignore_first: bool = false;
    ignore_first = after
        == lang_os_list.head as *mut libc::c_void
            as *mut lang_output_section_statement_type;
    where_0 = &mut (*after).header.next;
    while !(*where_0).is_null() {
        match (**where_0).header.type_0 as libc::c_uint {
            1 => {
                if assign.is_null() {
                    let mut ass: *mut lang_assignment_statement_type = 0
                        as *mut lang_assignment_statement_type;
                    ass = &mut (**where_0).assignment_statement;
                    if (*(*ass).exp).type_0.node_class as libc::c_uint
                        != etree_assert as libc::c_int as libc::c_uint
                        && *((*(*ass).exp).assign.dst).offset(0 as libc::c_int as isize)
                            as libc::c_int == '.' as i32
                        && *((*(*ass).exp).assign.dst).offset(1 as libc::c_int as isize)
                            as libc::c_int == 0 as libc::c_int
                    {
                        if !ignore_first {
                            assign = where_0;
                        }
                        ignore_first = 0 as libc::c_int != 0;
                    }
                }
            }
            13 | 5 | 15 | 3 | 2 | 11 | 10 | 14 => {
                assign = 0 as *mut *mut lang_statement_union_type;
                ignore_first = 0 as libc::c_int != 0;
            }
            8 => {
                if !assign.is_null() {
                    let mut s: *mut asection = (**where_0)
                        .output_section_statement
                        .bfd_section;
                    if s.is_null() || ((*s).map_head.s).is_null()
                        || (*s).flags & 0x1 as libc::c_int as libc::c_uint
                            != 0 as libc::c_int as libc::c_uint
                    {
                        where_0 = assign;
                    }
                }
                break;
            }
            6 | 0 | 12 | 9 | 4 | 7 => {}
            _ => {
                break;
            }
        }
        where_0 = &mut (**where_0).header.next;
    }
    return where_0;
}
unsafe extern "C" fn lang_print_asneeded() {
    let mut m: *mut asneeded_minfo = 0 as *mut asneeded_minfo;
    if asneeded_list_head.is_null() {
        return;
    }
    minfo(
        dcgettext(
            0 as *const libc::c_char,
            b"\nAs-needed library included to satisfy reference by file (symbol)\n\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    m = asneeded_list_head;
    while !m.is_null() {
        let mut len: size_t = 0;
        minfo(b"%s\0" as *const u8 as *const libc::c_char, (*m).soname);
        len = strlen((*m).soname);
        if len >= 29 as libc::c_int as libc::c_ulong {
            print_nl();
            len = 0 as libc::c_int as size_t;
        }
        while len < 30 as libc::c_int as libc::c_ulong {
            print_space();
            len = len.wrapping_add(1);
            len;
        }
        if !((*m).ref_0).is_null() {
            minfo(b"%pB \0" as *const u8 as *const libc::c_char, (*m).ref_0);
        }
        minfo(b"(%pT)\n\0" as *const u8 as *const libc::c_char, (*m).name);
        m = (*m).next;
    }
}
unsafe extern "C" fn lang_map_flags(mut flag: flagword) {
    if flag & 0x1 as libc::c_int as libc::c_uint != 0 {
        minfo(b"a\0" as *const u8 as *const libc::c_char);
    }
    if flag & 0x10 as libc::c_int as libc::c_uint != 0 {
        minfo(b"x\0" as *const u8 as *const libc::c_char);
    }
    if flag & 0x8 as libc::c_int as libc::c_uint != 0 {
        minfo(b"r\0" as *const u8 as *const libc::c_char);
    }
    if flag & 0x20 as libc::c_int as libc::c_uint != 0 {
        minfo(b"w\0" as *const u8 as *const libc::c_char);
    }
    if flag & 0x2 as libc::c_int as libc::c_uint != 0 {
        minfo(b"l\0" as *const u8 as *const libc::c_char);
    }
}
unsafe extern "C" fn init_os(
    mut s: *mut lang_output_section_statement_type,
    mut flags: flagword,
) {
    if strcmp((*s).name, b"/DISCARD/\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        einfo(
            dcgettext(
                0 as *const libc::c_char,
                b"%F%P: illegal use of `%s' section\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            b"/DISCARD/\0" as *const u8 as *const libc::c_char,
        );
    }
    if (*s).dup_output() == 0 {
        (*s).bfd_section = bfd_get_section_by_name(link_info.output_bfd, (*s).name);
    }
    if ((*s).bfd_section).is_null() {
        (*s)
            .bfd_section = bfd_make_section_anyway_with_flags(
            link_info.output_bfd,
            (*s).name,
            flags,
        );
    }
    if ((*s).bfd_section).is_null() {
        einfo(
            dcgettext(
                0 as *const libc::c_char,
                b"%F%P: output format %s cannot represent section called %s: %E\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            (*(*link_info.output_bfd).xvec).name,
            (*s).name,
        );
    }
    (*(*s).bfd_section).output_section = (*s).bfd_section;
    (*(*s).bfd_section).output_offset = 0 as libc::c_int as bfd_vma;
    bfd_set_section_userdata((*s).bfd_section, s as *mut libc::c_void);
    if !((*s).addr_tree).is_null() {
        exp_init_os((*s).addr_tree);
    }
    if !((*s).load_base).is_null() {
        exp_init_os((*s).load_base);
    }
    if !((*s).section_alignment).is_null() {
        (*(*s).bfd_section)
            .alignment_power = exp_get_power(
            (*s).section_alignment,
            b"section alignment\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        ) as libc::c_uint;
    }
}
unsafe extern "C" fn section_already_linked(
    mut abfd: *mut bfd,
    mut sec: *mut asection,
    mut data: *mut libc::c_void,
) {
    let mut entry: *mut lang_input_statement_type = data
        as *mut lang_input_statement_type;
    if ((*entry).flags).just_syms() != 0 {
        (Some(((*(*abfd).xvec)._bfd_link_just_syms).expect("non-null function pointer")))
            .expect("non-null function pointer")(sec, &mut link_info);
        return;
    }
    if !(link_info.type_0() as libc::c_int == type_relocatable as libc::c_int)
        && (*abfd).flags & 0x10000 as libc::c_int as libc::c_uint
            == 0 as libc::c_int as libc::c_uint
        && (*sec).flags
            & (0x2000000 as libc::c_int | 0x200000 as libc::c_int
                | 0x8000 as libc::c_int) as libc::c_uint
            == 0x8000 as libc::c_int as libc::c_uint
    {
        (*sec)
            .output_section = &mut *_bfd_std_section
            .as_mut_ptr()
            .offset(2 as libc::c_int as isize) as *mut asection;
    }
    if (*abfd).flags & 0x40 as libc::c_int as libc::c_uint == 0 {
        (Some(
            ((*(*abfd).xvec)._section_already_linked).expect("non-null function pointer"),
        ))
            .expect("non-null function pointer")(abfd, sec, &mut link_info);
    }
}
unsafe extern "C" fn lang_discard_section_p(mut section: *mut asection) -> bool {
    let mut discard: bool = false;
    let mut flags: flagword = (*section).flags;
    discard = flags & 0x8000 as libc::c_int as libc::c_uint
        != 0 as libc::c_int as libc::c_uint;
    if flags & 0x2000000 as libc::c_int as libc::c_uint
        != 0 as libc::c_int as libc::c_uint
        && link_info.resolve_section_groups() as libc::c_int != 0
    {
        discard = 1 as libc::c_int != 0;
    }
    if (link_info.strip() as libc::c_int == strip_debugger as libc::c_int
        || link_info.strip() as libc::c_int == strip_all as libc::c_int)
        && flags & 0x2000 as libc::c_int as libc::c_uint
            != 0 as libc::c_int as libc::c_uint
    {
        discard = 1 as libc::c_int != 0;
    }
    return discard;
}
unsafe extern "C" fn wild_sort(
    mut wild_0: *mut lang_wild_statement_type,
    mut sec: *mut wildcard_list,
    mut file: *mut lang_input_statement_type,
    mut section: *mut asection,
) -> *mut lang_statement_union_type {
    let mut l: *mut lang_statement_union_type = 0 as *mut lang_statement_union_type;
    if !(*wild_0).filenames_sorted
        && (sec.is_null()
            || (*sec).spec.sorted as libc::c_uint == none as libc::c_int as libc::c_uint)
    {
        return 0 as *mut lang_statement_union_type;
    }
    let mut current_block_23: u64;
    l = (*wild_0).children.head;
    while !l.is_null() {
        let mut ls: *mut lang_input_section_type = 0 as *mut lang_input_section_type;
        if !((*l).header.type_0 as libc::c_uint
            != lang_input_section_enum as libc::c_int as libc::c_uint)
        {
            ls = &mut (*l).input_section;
            if (*wild_0).filenames_sorted {
                let mut fn_0: *const libc::c_char = 0 as *const libc::c_char;
                let mut ln: *const libc::c_char = 0 as *const libc::c_char;
                let mut fa: bool = false;
                let mut la: bool = false;
                let mut i: libc::c_int = 0;
                if !((*file).the_bfd).is_null()
                    && !((*(*file).the_bfd).my_archive).is_null()
                {
                    fn_0 = bfd_get_filename((*(*file).the_bfd).my_archive);
                    fa = 1 as libc::c_int != 0;
                } else {
                    fn_0 = (*file).filename;
                    fa = 0 as libc::c_int != 0;
                }
                if !((*(*(*ls).section).owner).my_archive).is_null() {
                    ln = bfd_get_filename((*(*(*ls).section).owner).my_archive);
                    la = 1 as libc::c_int != 0;
                } else {
                    ln = bfd_get_filename((*(*ls).section).owner);
                    la = 0 as libc::c_int != 0;
                }
                i = filename_cmp(fn_0, ln);
                if i > 0 as libc::c_int {
                    current_block_23 = 4988723283678924448;
                } else {
                    if i < 0 as libc::c_int {
                        break;
                    }
                    if fa as libc::c_int != 0 || la as libc::c_int != 0 {
                        if fa {
                            fn_0 = (*file).filename;
                        }
                        if la {
                            ln = bfd_get_filename((*(*ls).section).owner);
                        }
                        i = filename_cmp(fn_0, ln);
                        if i > 0 as libc::c_int {
                            current_block_23 = 4988723283678924448;
                        } else {
                            if i < 0 as libc::c_int {
                                break;
                            }
                            current_block_23 = 1608152415753874203;
                        }
                    } else {
                        current_block_23 = 1608152415753874203;
                    }
                }
            } else {
                current_block_23 = 1608152415753874203;
            }
            match current_block_23 {
                4988723283678924448 => {}
                _ => {
                    if !sec.is_null()
                        && (*sec).spec.sorted as libc::c_uint
                            != none as libc::c_int as libc::c_uint
                        && (*sec).spec.sorted as libc::c_uint
                            != by_none as libc::c_int as libc::c_uint
                    {
                        if compare_section((*sec).spec.sorted, section, (*ls).section)
                            < 0 as libc::c_int
                        {
                            break;
                        }
                    }
                }
            }
        }
        l = (*l).header.next;
    }
    return l;
}
unsafe extern "C" fn output_section_callback(
    mut ptr: *mut lang_wild_statement_type,
    mut sec: *mut wildcard_list,
    mut section: *mut asection,
    mut file: *mut lang_input_statement_type,
    mut output: *mut libc::c_void,
) {
    let mut before: *mut lang_statement_union_type = 0 as *mut lang_statement_union_type;
    let mut os: *mut lang_output_section_statement_type = 0
        as *mut lang_output_section_statement_type;
    os = output as *mut lang_output_section_statement_type;
    if unique_section_p(section, os) {
        return;
    }
    before = wild_sort(ptr, sec, file, section);
    if before.is_null() {
        lang_add_section(
            &mut (*ptr).children,
            section,
            (*ptr).section_list,
            (*ptr).section_flag_list,
            os,
        );
    } else {
        let mut list: lang_statement_list_type = lang_statement_list_type {
            head: 0 as *const lang_statement_union as *mut lang_statement_union,
            tail: 0 as *const *mut lang_statement_union as *mut *mut lang_statement_union,
        };
        let mut pp: *mut *mut lang_statement_union_type = 0
            as *mut *mut lang_statement_union_type;
        lang_list_init(&mut list);
        lang_add_section(
            &mut list,
            section,
            (*ptr).section_list,
            (*ptr).section_flag_list,
            os,
        );
        if !(list.head).is_null() {
            if !((*list.head).header.next).is_null() {
                info_assert(
                    b"ldlang.c\0" as *const u8 as *const libc::c_char,
                    2862 as libc::c_int as libc::c_uint,
                );
            }
            pp = &mut (*ptr).children.head;
            while *pp != before {
                if (*pp).is_null() {
                    info_assert(
                        b"ldlang.c\0" as *const u8 as *const libc::c_char,
                        2867 as libc::c_int as libc::c_uint,
                    );
                }
                pp = &mut (**pp).header.next;
            }
            (*list.head).header.next = *pp;
            *pp = list.head;
        }
    };
}
unsafe extern "C" fn check_section_callback(
    mut ptr: *mut lang_wild_statement_type,
    mut sec: *mut wildcard_list,
    mut section: *mut asection,
    mut file: *mut lang_input_statement_type,
    mut output: *mut libc::c_void,
) {
    let mut os: *mut lang_output_section_statement_type = 0
        as *mut lang_output_section_statement_type;
    os = output as *mut lang_output_section_statement_type;
    if unique_section_p(section, os) {
        return;
    }
    if ((*section).output_section).is_null()
        && (*section).flags & 0x8 as libc::c_int as libc::c_uint
            == 0 as libc::c_int as libc::c_uint
    {
        (*os).set_all_input_readonly(0 as libc::c_int as libc::c_uint);
    }
}
static mut excluded_libs: *mut excluded_lib = 0 as *const excluded_lib
    as *mut excluded_lib;
unsafe extern "C" fn check_excluded_libs(mut abfd: *mut bfd) {
    let mut lib: *mut excluded_lib = excluded_libs;
    while !lib.is_null() {
        let mut len: libc::c_int = strlen((*lib).name) as libc::c_int;
        let mut filename: *const libc::c_char = lbasename(bfd_get_filename(abfd));
        if strcmp((*lib).name, b"ALL\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            (*abfd).set_no_export(1 as libc::c_int as libc::c_uint);
            return;
        }
        if filename_ncmp((*lib).name, filename, len as size_t) == 0 as libc::c_int
            && (*filename.offset(len as isize) as libc::c_int == '\0' as i32
                || *filename.offset(len as isize) as libc::c_int == '.' as i32
                    && *filename.offset((len + 1 as libc::c_int) as isize) as libc::c_int
                        == 'a' as i32
                    && *filename.offset((len + 2 as libc::c_int) as isize) as libc::c_int
                        == '\0' as i32)
        {
            (*abfd).set_no_export(1 as libc::c_int as libc::c_uint);
            return;
        }
        lib = (*lib).next;
    }
}
unsafe extern "C" fn wild(
    mut s: *mut lang_wild_statement_type,
    mut target: *const libc::c_char,
    mut output: *mut lang_output_section_statement_type,
) {
    let mut sec: *mut wildcard_list = 0 as *mut wildcard_list;
    if !((*s).handler_data[0 as libc::c_int as usize]).is_null()
        && (*(*s).handler_data[0 as libc::c_int as usize]).spec.sorted as libc::c_uint
            == by_name as libc::c_int as libc::c_uint && !(*s).filenames_sorted
    {
        let mut tree: *mut lang_section_bst_type = 0 as *mut lang_section_bst_type;
        walk_wild(
            s,
            Some(
                output_section_callback_fast
                    as unsafe extern "C" fn(
                        *mut lang_wild_statement_type,
                        *mut wildcard_list,
                        *mut asection,
                        *mut lang_input_statement_type,
                        *mut libc::c_void,
                    ) -> (),
            ),
            output as *mut libc::c_void,
        );
        tree = (*s).tree;
        if !tree.is_null() {
            output_section_callback_tree_to_list(s, tree, output as *mut libc::c_void);
            (*s).tree = 0 as *mut lang_section_bst_type;
        }
    } else {
        walk_wild(
            s,
            Some(
                output_section_callback
                    as unsafe extern "C" fn(
                        *mut lang_wild_statement_type,
                        *mut wildcard_list,
                        *mut asection,
                        *mut lang_input_statement_type,
                        *mut libc::c_void,
                    ) -> (),
            ),
            output as *mut libc::c_void,
        );
    }
    if default_common_section.is_null() {
        sec = (*s).section_list;
        while !sec.is_null() {
            if !((*sec).spec.name).is_null()
                && strcmp(
                    (*sec).spec.name,
                    b"COMMON\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
            {
                default_common_section = output;
                break;
            } else {
                sec = (*sec).next;
            }
        }
    }
}
unsafe extern "C" fn get_target(
    mut target: *const bfd_target,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut sought: *const libc::c_char = data as *const libc::c_char;
    return (strcmp((*target).name, sought) == 0 as libc::c_int) as libc::c_int;
}
unsafe extern "C" fn stricpy(mut dest: *mut libc::c_char, mut src: *const libc::c_char) {
    let mut c: libc::c_char = 0;
    loop {
        let fresh8 = src;
        src = src.offset(1);
        c = *fresh8;
        if !(c as libc::c_int != 0 as libc::c_int) {
            break;
        }
        let fresh9 = dest;
        dest = dest.offset(1);
        *fresh9 = _sch_tolower[(c as libc::c_int & 0xff as libc::c_int) as usize]
            as libc::c_char;
    }
    *dest = 0 as libc::c_int as libc::c_char;
}
unsafe extern "C" fn strcut(
    mut haystack: *mut libc::c_char,
    mut needle: *const libc::c_char,
) {
    haystack = strstr(haystack, needle);
    if !haystack.is_null() {
        let mut src: *mut libc::c_char = 0 as *mut libc::c_char;
        src = haystack.offset(strlen(needle) as isize);
        while *src != 0 {
            let fresh10 = src;
            src = src.offset(1);
            let fresh11 = haystack;
            haystack = haystack.offset(1);
            *fresh11 = *fresh10;
        }
        *haystack = 0 as libc::c_int as libc::c_char;
    }
}
unsafe extern "C" fn name_compare(
    mut first: *const libc::c_char,
    mut second: *const libc::c_char,
) -> libc::c_int {
    let mut copy1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut copy2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut result: libc::c_int = 0;
    copy1 = xmalloc((strlen(first)).wrapping_add(1 as libc::c_int as libc::c_ulong))
        as *mut libc::c_char;
    copy2 = xmalloc((strlen(second)).wrapping_add(1 as libc::c_int as libc::c_ulong))
        as *mut libc::c_char;
    stricpy(copy1, first);
    stricpy(copy2, second);
    strcut(copy1, b"big\0" as *const u8 as *const libc::c_char);
    strcut(copy1, b"little\0" as *const u8 as *const libc::c_char);
    strcut(copy2, b"big\0" as *const u8 as *const libc::c_char);
    strcut(copy2, b"little\0" as *const u8 as *const libc::c_char);
    result = 0 as libc::c_int;
    while *copy1.offset(result as isize) as libc::c_int
        == *copy2.offset(result as isize) as libc::c_int
    {
        if *copy1.offset(result as isize) as libc::c_int == 0 as libc::c_int {
            result *= 10 as libc::c_int;
            break;
        } else {
            result += 1;
            result;
        }
    }
    free(copy1 as *mut libc::c_void);
    free(copy2 as *mut libc::c_void);
    return result;
}
static mut winner: *const bfd_target = 0 as *const bfd_target;
unsafe extern "C" fn closest_target_match(
    mut target: *const bfd_target,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut original: *const bfd_target = data as *const bfd_target;
    if command_line.endian as libc::c_uint == ENDIAN_BIG as libc::c_int as libc::c_uint
        && (*target).byteorder as libc::c_uint
            != BFD_ENDIAN_BIG as libc::c_int as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    if command_line.endian as libc::c_uint
        == ENDIAN_LITTLE as libc::c_int as libc::c_uint
        && (*target).byteorder as libc::c_uint
            != BFD_ENDIAN_LITTLE as libc::c_int as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    if (*target).flavour as libc::c_uint != (*original).flavour as libc::c_uint {
        return 0 as libc::c_int;
    }
    if strcmp((*target).name, b"elf32-big\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
        || strcmp((*target).name, b"elf64-big\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        || strcmp((*target).name, b"elf32-little\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        || strcmp((*target).name, b"elf64-little\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    if winner.is_null() {
        winner = target;
        return 0 as libc::c_int;
    }
    if name_compare((*target).name, (*original).name)
        > name_compare((*winner).name, (*original).name)
    {
        winner = target;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn get_first_input_target() -> *const libc::c_char {
    let mut target: *const libc::c_char = 0 as *const libc::c_char;
    let mut s: *mut lang_input_statement_type = 0 as *mut lang_input_statement_type;
    s = file_chain.head as *mut lang_input_statement_type;
    while !s.is_null() {
        if (*s).header.type_0 as libc::c_uint
            == lang_input_statement_enum as libc::c_int as libc::c_uint
            && ((*s).flags).real() as libc::c_int != 0
        {
            ldfile_open_file(s);
            if !((*s).the_bfd).is_null()
                && bfd_check_format((*s).the_bfd, bfd_object) as libc::c_int != 0
            {
                target = bfd_get_target((*s).the_bfd);
                if !target.is_null() {
                    break;
                }
            }
        }
        s = (*s).next;
    }
    return target;
}
unsafe extern "C" fn open_output(mut name: *const libc::c_char) {
    output_target = lang_get_output_target();
    if command_line.endian as libc::c_uint != ENDIAN_UNSET as libc::c_int as libc::c_uint
    {
        let mut target: *const bfd_target = bfd_iterate_over_targets(
            Some(
                get_target
                    as unsafe extern "C" fn(
                        *const bfd_target,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            output_target as *mut libc::c_void,
        );
        if !target.is_null() {
            let mut desired_endian: bfd_endian = BFD_ENDIAN_BIG;
            if command_line.endian as libc::c_uint
                == ENDIAN_BIG as libc::c_int as libc::c_uint
            {
                desired_endian = BFD_ENDIAN_BIG;
            } else {
                desired_endian = BFD_ENDIAN_LITTLE;
            }
            if (*target).byteorder as libc::c_uint != desired_endian as libc::c_uint {
                if !((*target).alternative_target).is_null()
                    && (*(*target).alternative_target).byteorder as libc::c_uint
                        == desired_endian as libc::c_uint
                {
                    output_target = (*(*target).alternative_target).name;
                } else {
                    bfd_iterate_over_targets(
                        Some(
                            closest_target_match
                                as unsafe extern "C" fn(
                                    *const bfd_target,
                                    *mut libc::c_void,
                                ) -> libc::c_int,
                        ),
                        target as *mut libc::c_void,
                    );
                    if winner.is_null() {
                        einfo(
                            dcgettext(
                                0 as *const libc::c_char,
                                b"%P: warning: could not find any targets that match endianness requirement\n\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                    } else {
                        output_target = (*winner).name;
                    }
                }
            }
        }
    }
    link_info.output_bfd = bfd_openw(name, output_target);
    if (link_info.output_bfd).is_null() {
        if bfd_get_error() as libc::c_uint
            == bfd_error_invalid_target as libc::c_int as libc::c_uint
        {
            einfo(
                dcgettext(
                    0 as *const libc::c_char,
                    b"%F%P: target %s not found\n\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                output_target,
            );
        }
        einfo(
            dcgettext(
                0 as *const libc::c_char,
                b"%F%P: cannot open output file %s: %E\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            name,
        );
    }
    delete_output_file_on_failure = 1 as libc::c_int != 0;
    if !bfd_set_format(link_info.output_bfd, bfd_object) {
        einfo(
            dcgettext(
                0 as *const libc::c_char,
                b"%F%P: %s: can not make object file: %E\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            name,
        );
    }
    if !(Some(
        ((*(*link_info.output_bfd).xvec)._bfd_set_arch_mach)
            .expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )(link_info.output_bfd, ldfile_output_architecture, ldfile_output_machine)
    {
        einfo(
            dcgettext(
                0 as *const libc::c_char,
                b"%F%P: %s: can not set architecture: %E\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            name,
        );
    }
    link_info
        .hash = (Some(
        ((*(*link_info.output_bfd).xvec)._bfd_link_hash_table_create)
            .expect("non-null function pointer"),
    ))
        .expect("non-null function pointer")(link_info.output_bfd);
    if (link_info.hash).is_null() {
        einfo(
            dcgettext(
                0 as *const libc::c_char,
                b"%F%P: can not create hash table: %E\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    bfd_set_gp_size(link_info.output_bfd, g_switch_value as libc::c_uint);
}
unsafe extern "C" fn ldlang_open_output(mut statement: *mut lang_statement_union_type) {
    match (*statement).header.type_0 as libc::c_uint {
        9 => {
            if !(link_info.output_bfd).is_null() {
                info_assert(
                    b"ldlang.c\0" as *const u8 as *const libc::c_char,
                    3475 as libc::c_int as libc::c_uint,
                );
            }
            open_output((*statement).output_statement.name);
            ldemul_set_output_arch();
            if config.magic_demand_paged as libc::c_int != 0
                && !(link_info.type_0() as libc::c_int
                    == type_relocatable as libc::c_int)
            {
                (*link_info.output_bfd).flags |= 0x100 as libc::c_int as libc::c_uint;
            } else {
                (*link_info.output_bfd).flags &= !(0x100 as libc::c_int) as libc::c_uint;
            }
            if config.text_read_only {
                (*link_info.output_bfd).flags |= 0x80 as libc::c_int as libc::c_uint;
            } else {
                (*link_info.output_bfd).flags &= !(0x80 as libc::c_int) as libc::c_uint;
            }
            if link_info.traditional_format() != 0 {
                (*link_info.output_bfd).flags |= 0x400 as libc::c_int as libc::c_uint;
            } else {
                (*link_info.output_bfd).flags &= !(0x400 as libc::c_int) as libc::c_uint;
            }
        }
        12 => {
            current_target = (*statement).target_statement.target;
        }
        _ => {}
    };
}
unsafe extern "C" fn init_opb(mut s: *mut asection) {
    let mut x: libc::c_uint = 0;
    opb_shift = 0 as libc::c_int as libc::c_uint;
    if bfd_get_flavour(link_info.output_bfd) as libc::c_uint
        == bfd_target_elf_flavour as libc::c_int as libc::c_uint && !s.is_null()
        && (*s).flags & 0x40000000 as libc::c_int as libc::c_uint
            != 0 as libc::c_int as libc::c_uint
    {
        return;
    }
    x = bfd_arch_mach_octets_per_byte(ldfile_output_architecture, ldfile_output_machine);
    if x > 1 as libc::c_int as libc::c_uint {
        while x & 1 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
            x >>= 1 as libc::c_int;
            opb_shift = opb_shift.wrapping_add(1);
            opb_shift;
        }
    }
    if !(x == 1 as libc::c_int as libc::c_uint) {
        info_assert(
            b"ldlang.c\0" as *const u8 as *const libc::c_char,
            3520 as libc::c_int as libc::c_uint,
        );
    }
}
static mut plugin_insert: *mut lang_input_statement_type = 0
    as *const lang_input_statement_type as *mut lang_input_statement_type;
static mut plugin_undefs: *mut bfd_link_hash_entry = 0 as *const bfd_link_hash_entry
    as *mut bfd_link_hash_entry;
unsafe extern "C" fn open_input_bfds(
    mut s: *mut lang_statement_union_type,
    mut mode: open_bfd_mode,
) {
    while !s.is_null() {
        match (*s).header.type_0 as libc::c_uint {
            14 => {
                open_input_bfds(constructor_list.head, mode);
            }
            8 => {
                open_input_bfds((*s).output_section_statement.children.head, mode);
            }
            13 => {
                if mode as libc::c_uint & OPEN_BFD_RESCAN as libc::c_int as libc::c_uint
                    == 0 as libc::c_int as libc::c_uint
                    && !((*s).wild_statement.filename).is_null()
                    && (strpbrk(
                        (*s).wild_statement.filename,
                        b"?*[\0" as *const u8 as *const libc::c_char,
                    ))
                        .is_null()
                    && (archive_path((*s).wild_statement.filename)).is_null()
                {
                    lookup_name((*s).wild_statement.filename);
                }
                open_input_bfds((*s).wild_statement.children.head, mode);
            }
            4 => {
                let mut undefs: *mut bfd_link_hash_entry = 0 as *mut bfd_link_hash_entry;
                let mut plugin_insert_save: *mut lang_input_statement_type = 0
                    as *mut lang_input_statement_type;
                loop {
                    plugin_insert_save = plugin_insert;
                    undefs = (*link_info.hash).undefs_tail;
                    open_input_bfds(
                        (*s).group_statement.children.head,
                        (mode as libc::c_uint
                            | OPEN_BFD_FORCE as libc::c_int as libc::c_uint)
                            as open_bfd_mode,
                    );
                    if !(undefs != (*link_info.hash).undefs_tail
                        || plugin_insert != plugin_insert_save
                            && !plugin_undefs.is_null())
                    {
                        break;
                    }
                }
            }
            12 => {
                current_target = (*s).target_statement.target;
            }
            6 => {
                if ((*s).input_statement.flags).real() != 0 {
                    let mut os_tail: *mut *mut lang_statement_union_type = 0
                        as *mut *mut lang_statement_union_type;
                    let mut add: lang_statement_list_type = lang_statement_list_type {
                        head: 0 as *const lang_statement_union
                            as *mut lang_statement_union,
                        tail: 0 as *const *mut lang_statement_union
                            as *mut *mut lang_statement_union,
                    };
                    let mut abfd: *mut bfd = 0 as *mut bfd;
                    (*s).input_statement.target = current_target;
                    if mode as libc::c_uint
                        != OPEN_BFD_NORMAL as libc::c_int as libc::c_uint
                        && (mode as libc::c_uint
                            & OPEN_BFD_RESCAN as libc::c_int as libc::c_uint
                            == 0 as libc::c_int as libc::c_uint
                            || plugin_insert.is_null())
                        && ((*s).input_statement.flags).loaded() as libc::c_int != 0
                        && {
                            abfd = (*s).input_statement.the_bfd;
                            !abfd.is_null()
                        }
                        && (bfd_get_format(abfd) as libc::c_uint
                            == bfd_archive as libc::c_int as libc::c_uint
                            && ((*s).input_statement.flags).whole_archive() == 0
                            || bfd_get_format(abfd) as libc::c_uint
                                == bfd_object as libc::c_int as libc::c_uint
                                && (*abfd).flags & 0x40 as libc::c_int as libc::c_uint
                                    != 0 as libc::c_int as libc::c_uint
                                && ((*s).input_statement.flags).add_DT_NEEDED_for_regular()
                                    as libc::c_int != 0
                                && bfd_get_flavour(abfd) as libc::c_uint
                                    == bfd_target_elf_flavour as libc::c_int as libc::c_uint
                                && (*(*abfd).tdata.elf_obj_data).dyn_lib_class()
                                    as libc::c_int & DYN_AS_NEEDED as libc::c_int
                                    != 0 as libc::c_int)
                    {
                        ((*s).input_statement.flags)
                            .set_loaded(0 as libc::c_int as libc::c_uint);
                        ((*s).input_statement.flags)
                            .set_reload(1 as libc::c_int as libc::c_uint);
                    }
                    os_tail = lang_os_list.tail;
                    lang_list_init(&mut add);
                    if !load_symbols(&mut (*s).input_statement, &mut add) {
                        config.make_executable = 0 as libc::c_int != 0;
                    }
                    if !(add.head).is_null() {
                        if os_tail != lang_os_list.tail {
                            einfo(
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"%P: warning: %s contains output sections; did you forget -T?\n\0"
                                        as *const u8 as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                                (*s).input_statement.filename,
                            );
                            *(*stat_ptr).tail = add.head;
                            (*stat_ptr).tail = add.tail;
                        } else {
                            *add.tail = (*s).header.next;
                            (*s).header.next = add.head;
                        }
                    }
                }
                if &mut (*s).input_statement as *mut lang_input_statement_type
                    == plugin_insert
                {
                    plugin_insert = 0 as *mut lang_input_statement_type;
                }
            }
            1 => {
                if (*(*s).assignment_statement.exp).type_0.node_class as libc::c_uint
                    != etree_assert as libc::c_int as libc::c_uint
                {
                    exp_fold_tree_no_dot((*s).assignment_statement.exp);
                }
            }
            _ => {}
        }
        s = (*s).header.next;
    }
    if input_flags.missing_file() != 0 {
        einfo(b"%F\0" as *const u8 as *const libc::c_char);
    }
}
unsafe extern "C" fn lang_ctf_errs_warnings(mut fp: *mut ctf_dict_t) {
    let mut i: *mut ctf_next_t = 0 as *mut ctf_next_t;
    let mut text: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut is_warning: libc::c_int = 0;
    let mut err: libc::c_int = 0;
    loop {
        text = ctf_errwarning_next(fp, &mut i, &mut is_warning, &mut err);
        if text.is_null() {
            break;
        }
        einfo(
            dcgettext(
                0 as *const libc::c_char,
                b"%s: %s\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            if is_warning != 0 {
                dcgettext(
                    0 as *const libc::c_char,
                    b"CTF warning\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                )
            } else {
                dcgettext(
                    0 as *const libc::c_char,
                    b"CTF error\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                )
            },
            text,
        );
        free(text as *mut libc::c_void);
    }
    if err != ECTF_NEXT_END as libc::c_int {
        einfo(
            dcgettext(
                0 as *const libc::c_char,
                b"CTF error: cannot get CTF errors: `%s'\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            ctf_errmsg(err),
        );
    }
    if !(fp.is_null() || ctf_errno(fp) != ECTF_INTERNAL as libc::c_int) {
        info_assert(
            b"ldlang.c\0" as *const u8 as *const libc::c_char,
            3700 as libc::c_int as libc::c_uint,
        );
    }
}
unsafe extern "C" fn ldlang_open_ctf() {
    let mut any_ctf: libc::c_int = 0 as libc::c_int;
    let mut err: libc::c_int = 0;
    let mut file: *mut lang_input_statement_type = 0 as *mut lang_input_statement_type;
    file = file_chain.head as *mut lang_input_statement_type;
    while !file.is_null() {
        let mut sect: *mut asection = 0 as *mut asection;
        (*file).the_ctf = ctf_bfdopen((*file).the_bfd, &mut err);
        if ((*file).the_ctf).is_null() {
            if err != ECTF_NOCTFDATA as libc::c_int {
                lang_ctf_errs_warnings(0 as *mut ctf_dict_t);
                einfo(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%P: warning: CTF section in %pB not loaded; its types will be discarded: %s\n\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    (*file).the_bfd,
                    ctf_errmsg(err),
                );
            }
        } else {
            sect = bfd_get_section_by_name(
                (*file).the_bfd,
                b".ctf\0" as *const u8 as *const libc::c_char,
            );
            (*sect).size = 0 as libc::c_int as bfd_size_type;
            (*sect).flags
                |= (0x200 as libc::c_int | 0x100 as libc::c_int
                    | 0x100000 as libc::c_int) as libc::c_uint;
            if any_ctf != 0 {
                (*sect).flags |= 0x8000 as libc::c_int as libc::c_uint;
            }
            any_ctf = 1 as libc::c_int;
        }
        file = (*file).next;
    }
    if any_ctf == 0 {
        ctf_output = 0 as *mut ctf_dict_t;
        return;
    }
    ctf_output = ctf_create(&mut err);
    if !ctf_output.is_null() {
        return;
    }
    einfo(
        dcgettext(
            0 as *const libc::c_char,
            b"%P: warning: CTF output not created: `%s'\n\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
        ctf_errmsg(err),
    );
    let mut errfile: *mut lang_input_statement_type = 0
        as *mut lang_input_statement_type;
    errfile = file_chain.head as *mut lang_input_statement_type;
    while !errfile.is_null() {
        ctf_close((*errfile).the_ctf);
        errfile = (*errfile).next;
    }
}
unsafe extern "C" fn lang_merge_ctf() {
    let mut output_sect: *mut asection = 0 as *mut asection;
    let mut flags: libc::c_int = 0 as libc::c_int;
    if ctf_output.is_null() {
        return;
    }
    output_sect = bfd_get_section_by_name(
        link_info.output_bfd,
        b".ctf\0" as *const u8 as *const libc::c_char,
    );
    if output_sect.is_null() {
        ctf_dict_close(ctf_output);
        ctf_output = 0 as *mut ctf_dict_t;
        let mut file: *mut lang_input_statement_type = 0
            as *mut lang_input_statement_type;
        file = file_chain.head as *mut lang_input_statement_type;
        while !file.is_null() {
            ctf_close((*file).the_ctf);
            (*file).the_ctf = 0 as *mut ctf_archive_t;
            file = (*file).next;
        }
        return;
    }
    let mut file_0: *mut lang_input_statement_type = 0 as *mut lang_input_statement_type;
    file_0 = file_chain.head as *mut lang_input_statement_type;
    while !file_0.is_null() {
        if !((*file_0).the_ctf).is_null() {
            if ctf_link_add_ctf(ctf_output, (*file_0).the_ctf, (*file_0).filename)
                < 0 as libc::c_int
            {
                einfo(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%P: warning: CTF section in %pB cannot be linked: `%s'\n\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    (*file_0).the_bfd,
                    ctf_errmsg(ctf_errno(ctf_output)),
                );
                ctf_close((*file_0).the_ctf);
                (*file_0).the_ctf = 0 as *mut ctf_archive_t;
            }
        }
        file_0 = (*file_0).next;
    }
    if !config.ctf_share_duplicated {
        flags = 0 as libc::c_int;
    } else {
        flags = 0x1 as libc::c_int;
    }
    if !config.ctf_variables {
        flags |= 0x8 as libc::c_int;
    }
    if link_info.type_0() as libc::c_int == type_relocatable as libc::c_int {
        flags |= 0x10 as libc::c_int;
    }
    if ctf_link(ctf_output, flags) < 0 as libc::c_int {
        lang_ctf_errs_warnings(ctf_output);
        einfo(
            dcgettext(
                0 as *const libc::c_char,
                b"%P: warning: CTF linking failed; output will have no CTF section: %s\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            ctf_errmsg(ctf_errno(ctf_output)),
        );
        if !output_sect.is_null() {
            (*output_sect).size = 0 as libc::c_int as bfd_size_type;
            (*output_sect).flags |= 0x8000 as libc::c_int as libc::c_uint;
        }
    }
    lang_ctf_errs_warnings(ctf_output);
}
unsafe extern "C" fn lang_write_ctf(mut late: libc::c_int) {
    let mut output_size: size_t = 0;
    let mut output_sect: *mut asection = 0 as *mut asection;
    if ctf_output.is_null() {
        return;
    }
    if late != 0 {
        if ldemul_emit_ctf_early() != 0 {
            return;
        }
    } else if ldemul_emit_ctf_early() == 0 {
        return
    }
    ldemul_new_dynsym_for_ctf(ctf_output, 0 as libc::c_int, 0 as *mut elf_internal_sym);
    output_sect = bfd_get_section_by_name(
        link_info.output_bfd,
        b".ctf\0" as *const u8 as *const libc::c_char,
    );
    if !output_sect.is_null() {
        (*output_sect)
            .contents = ctf_link_write(
            ctf_output,
            &mut output_size,
            4096 as libc::c_int as size_t,
        );
        (*output_sect).size = output_size;
        (*output_sect).flags
            |= (0x4000 as libc::c_int | 0x200000 as libc::c_int) as libc::c_uint;
        lang_ctf_errs_warnings(ctf_output);
        if ((*output_sect).contents).is_null() {
            einfo(
                dcgettext(
                    0 as *const libc::c_char,
                    b"%P: warning: CTF section emission failed; output will have no CTF section: %s\n\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                ctf_errmsg(ctf_errno(ctf_output)),
            );
            (*output_sect).size = 0 as libc::c_int as bfd_size_type;
            (*output_sect).flags |= 0x8000 as libc::c_int as libc::c_uint;
        }
    }
    ctf_dict_close(ctf_output);
    ctf_output = 0 as *mut ctf_dict_t;
    let mut file: *mut lang_input_statement_type = 0 as *mut lang_input_statement_type;
    file = file_chain.head as *mut lang_input_statement_type;
    while !file.is_null() {
        (*file).the_ctf = 0 as *mut ctf_archive_t;
        file = (*file).next;
    }
}
unsafe extern "C" fn lang_place_undefineds() {
    let mut ptr: *mut ldlang_undef_chain_list_type = 0
        as *mut ldlang_undef_chain_list_type;
    ptr = entry_symbol.next;
    while !ptr.is_null() {
        insert_undefined((*ptr).name);
        ptr = (*ptr).next;
    }
}
unsafe extern "C" fn lang_mark_undefineds() {
    let mut ptr: *mut ldlang_undef_chain_list_type = 0
        as *mut ldlang_undef_chain_list_type;
    if is_elf_hash_table(link_info.hash) {
        ptr = entry_symbol.next;
        while !ptr.is_null() {
            let mut h: *mut elf_link_hash_entry = bfd_link_hash_lookup(
                link_info.hash,
                (*ptr).name,
                0 as libc::c_int != 0,
                0 as libc::c_int != 0,
                1 as libc::c_int != 0,
            ) as *mut elf_link_hash_entry;
            if !h.is_null() {
                (*h).set_mark(1 as libc::c_int as libc::c_uint);
            }
            ptr = (*ptr).next;
        }
    }
}
static mut require_defined_symbol_list: *mut require_defined_symbol = 0
    as *const require_defined_symbol as *mut require_defined_symbol;
unsafe extern "C" fn ldlang_check_require_defined_symbols() {
    let mut ptr: *mut require_defined_symbol = 0 as *mut require_defined_symbol;
    ptr = require_defined_symbol_list;
    while !ptr.is_null() {
        let mut h: *mut bfd_link_hash_entry = 0 as *mut bfd_link_hash_entry;
        h = bfd_link_hash_lookup(
            link_info.hash,
            (*ptr).name,
            0 as libc::c_int != 0,
            0 as libc::c_int != 0,
            1 as libc::c_int != 0,
        );
        if h.is_null()
            || (*h).type_0() as libc::c_int != bfd_link_hash_defined as libc::c_int
                && (*h).type_0() as libc::c_int != bfd_link_hash_defweak as libc::c_int
        {
            einfo(
                dcgettext(
                    0 as *const libc::c_char,
                    b"%X%P: required symbol `%s' not defined\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*ptr).name,
            );
        }
        ptr = (*ptr).next;
    }
}
unsafe extern "C" fn check_input_sections(
    mut s: *mut lang_statement_union_type,
    mut output_section_statement: *mut lang_output_section_statement_type,
) {
    while !s.is_null() {
        match (*s).header.type_0 as libc::c_uint {
            13 => {
                walk_wild(
                    &mut (*s).wild_statement,
                    Some(
                        check_section_callback
                            as unsafe extern "C" fn(
                                *mut lang_wild_statement_type,
                                *mut wildcard_list,
                                *mut asection,
                                *mut lang_input_statement_type,
                                *mut libc::c_void,
                            ) -> (),
                    ),
                    output_section_statement as *mut libc::c_void,
                );
                if (*output_section_statement).all_input_readonly() == 0 {
                    return;
                }
            }
            14 => {
                check_input_sections(constructor_list.head, output_section_statement);
                if (*output_section_statement).all_input_readonly() == 0 {
                    return;
                }
            }
            4 => {
                check_input_sections(
                    (*s).group_statement.children.head,
                    output_section_statement,
                );
                if (*output_section_statement).all_input_readonly() == 0 {
                    return;
                }
            }
            _ => {}
        }
        s = (*s).header.next;
    }
}
unsafe extern "C" fn update_wild_statements(mut s: *mut lang_statement_union_type) {
    let mut sec: *mut wildcard_list = 0 as *mut wildcard_list;
    match sort_section as libc::c_uint {
        0 => {}
        1 | 2 => {
            while !s.is_null() {
                match (*s).header.type_0 as libc::c_uint {
                    13 => {
                        sec = (*s).wild_statement.section_list;
                        while !sec.is_null() {
                            if strcmp(
                                (*sec).spec.name,
                                b".init\0" as *const u8 as *const libc::c_char,
                            ) != 0 as libc::c_int
                                && strcmp(
                                    (*sec).spec.name,
                                    b".fini\0" as *const u8 as *const libc::c_char,
                                ) != 0 as libc::c_int
                            {
                                match (*sec).spec.sorted as libc::c_uint {
                                    0 => {
                                        (*sec).spec.sorted = sort_section;
                                    }
                                    1 => {
                                        if sort_section as libc::c_uint
                                            == by_alignment as libc::c_int as libc::c_uint
                                        {
                                            (*sec).spec.sorted = by_name_alignment;
                                        }
                                    }
                                    2 => {
                                        if sort_section as libc::c_uint
                                            == by_name as libc::c_int as libc::c_uint
                                        {
                                            (*sec).spec.sorted = by_alignment_name;
                                        }
                                    }
                                    _ => {}
                                }
                            }
                            sec = (*sec).next;
                        }
                    }
                    14 => {
                        update_wild_statements(constructor_list.head);
                    }
                    8 => {
                        update_wild_statements(
                            (*s).output_section_statement.children.head,
                        );
                    }
                    4 => {
                        update_wild_statements((*s).group_statement.children.head);
                    }
                    _ => {}
                }
                s = (*s).header.next;
            }
        }
        _ => {
            info_assert(
                b"ldlang.c\0" as *const u8 as *const libc::c_char,
                4116 as libc::c_int as libc::c_uint,
            );
        }
    };
}
unsafe extern "C" fn map_input_to_output_sections(
    mut s: *mut lang_statement_union_type,
    mut target: *const libc::c_char,
    mut os: *mut lang_output_section_statement_type,
) {
    while !s.is_null() {
        let mut tos: *mut lang_output_section_statement_type = 0
            as *mut lang_output_section_statement_type;
        let mut flags: flagword = 0;
        match (*s).header.type_0 as libc::c_uint {
            13 => {
                wild(&mut (*s).wild_statement, target, os);
            }
            14 => {
                map_input_to_output_sections(constructor_list.head, target, os);
            }
            8 => {
                tos = &mut (*s).output_section_statement;
                if (*tos).constraint == 381 as libc::c_int
                    || (*tos).constraint == 380 as libc::c_int
                {
                    (*tos).set_all_input_readonly(1 as libc::c_int as libc::c_uint);
                    check_input_sections((*tos).children.head, tos);
                    if (*tos).all_input_readonly() as libc::c_int
                        != ((*tos).constraint == 380 as libc::c_int) as libc::c_int
                    {
                        (*tos).constraint = -(1 as libc::c_int);
                    }
                }
                if (*tos).constraint >= 0 as libc::c_int {
                    map_input_to_output_sections((*tos).children.head, target, tos);
                }
            }
            12 => {
                target = (*s).target_statement.target;
            }
            4 => {
                map_input_to_output_sections(
                    (*s).group_statement.children.head,
                    target,
                    os,
                );
            }
            2 => {
                exp_init_os((*s).data_statement.exp);
                flags = (0x100 as libc::c_int | 0x1 as libc::c_int | 0x2 as libc::c_int)
                    as flagword;
                match (*os).sectype as libc::c_uint {
                    4 => {
                        flags = 0x100 as libc::c_int as flagword;
                    }
                    3 => {
                        if bfd_get_flavour(link_info.output_bfd) as libc::c_uint
                            == bfd_target_elf_flavour as libc::c_int as libc::c_uint
                        {
                            flags = (0x200 as libc::c_int | 0x1 as libc::c_int)
                                as flagword;
                        } else {
                            flags = (0x200 as libc::c_int | 0x100 as libc::c_int)
                                as flagword;
                        }
                    }
                    0 | 2 | 1 | _ => {}
                }
                if ((*os).bfd_section).is_null() {
                    init_os(os, flags | 0x8 as libc::c_int as libc::c_uint);
                } else {
                    (*(*os).bfd_section).flags |= flags;
                }
            }
            3 | 15 | 11 | 10 | 6 => {
                if !os.is_null() && ((*os).bfd_section).is_null() {
                    init_os(os, 0 as libc::c_int as flagword);
                }
            }
            1 => {
                if !os.is_null() && ((*os).bfd_section).is_null() {
                    init_os(os, 0 as libc::c_int as flagword);
                }
                exp_init_os((*s).assignment_statement.exp);
            }
            0 => {
                if ((*s).address_statement.segment).is_null()
                    || !(*(*s).address_statement.segment).used
                {
                    let mut name: *const libc::c_char = (*s)
                        .address_statement
                        .section_name;
                    tos = lang_output_section_statement_lookup(
                        name,
                        0 as libc::c_int,
                        1 as libc::c_int,
                    );
                    (*tos).addr_tree = (*s).address_statement.address;
                    if ((*tos).bfd_section).is_null() {
                        init_os(tos, 0 as libc::c_int as flagword);
                    }
                }
            }
            9 | 5 | 7 | _ => {}
        }
        s = (*s).header.next;
    }
}
unsafe extern "C" fn process_insert_statements(
    mut start: *mut *mut lang_statement_union_type,
) {
    let mut s: *mut *mut lang_statement_union_type = 0
        as *mut *mut lang_statement_union_type;
    let mut first_os: *mut lang_output_section_statement_type = 0
        as *mut lang_output_section_statement_type;
    let mut last_os: *mut lang_output_section_statement_type = 0
        as *mut lang_output_section_statement_type;
    let mut os: *mut lang_output_section_statement_type = 0
        as *mut lang_output_section_statement_type;
    s = start;
    while !(*s).is_null() {
        if (**s).header.type_0 as libc::c_uint
            == lang_output_section_statement_enum as libc::c_int as libc::c_uint
        {
            os = &mut (**s).output_section_statement;
            if !(last_os.is_null() || (*last_os).next == os) {
                info_assert(
                    b"ldlang.c\0" as *const u8 as *const libc::c_char,
                    4330 as libc::c_int as libc::c_uint,
                );
            }
            last_os = os;
            (*last_os).constraint = -(2 as libc::c_int) - (*last_os).constraint;
            if first_os.is_null() {
                first_os = last_os;
            }
        } else if (**s).header.type_0 as libc::c_uint
            == lang_group_statement_enum as libc::c_int as libc::c_uint
        {
            process_insert_statements(&mut (**s).group_statement.children.head);
        } else if (**s).header.type_0 as libc::c_uint
            == lang_insert_statement_enum as libc::c_int as libc::c_uint
        {
            let mut i: *mut lang_insert_statement_type = &mut (**s).insert_statement;
            let mut where_0: *mut lang_output_section_statement_type = 0
                as *mut lang_output_section_statement_type;
            let mut ptr: *mut *mut lang_statement_union_type = 0
                as *mut *mut lang_statement_union_type;
            let mut first: *mut lang_statement_union_type = 0
                as *mut lang_statement_union_type;
            if link_info.non_contiguous_regions() != 0 {
                einfo(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"warning: INSERT statement in linker script is incompatible with --enable-non-contiguous-regions.\n\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
            }
            where_0 = lang_output_section_statement_lookup(
                (*i).where_0,
                0 as libc::c_int,
                0 as libc::c_int,
            );
            if !where_0.is_null() && (*i).is_before as libc::c_int != 0 {
                loop {
                    where_0 = (*where_0).prev;
                    if !(!where_0.is_null() && (*where_0).constraint < 0 as libc::c_int)
                    {
                        break;
                    }
                }
            }
            if where_0.is_null() {
                einfo(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%F%P: %s not found for insert\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    (*i).where_0,
                );
                return;
            }
            if !last_os.is_null() {
                let mut first_sec: *mut asection = 0 as *mut asection;
                let mut last_sec: *mut asection = 0 as *mut asection;
                let mut next: *mut *mut lang_output_section_statement_struct = 0
                    as *mut *mut lang_output_section_statement_struct;
                (*(*first_os).prev).next = (*last_os).next;
                if ((*last_os).next).is_null() {
                    next = &mut (*(*first_os).prev).next;
                    lang_os_list.tail = next as *mut *mut lang_statement_union_type;
                } else {
                    (*(*last_os).next).prev = (*first_os).prev;
                }
                (*last_os).next = (*where_0).next;
                if ((*where_0).next).is_null() {
                    next = &mut (*last_os).next;
                    lang_os_list.tail = next as *mut *mut lang_statement_union_type;
                } else {
                    (*(*where_0).next).prev = last_os;
                }
                (*first_os).prev = where_0;
                (*where_0).next = first_os;
                first_sec = 0 as *mut asection;
                last_sec = 0 as *mut asection;
                os = first_os;
                while !os.is_null() {
                    (*os).constraint = -(2 as libc::c_int) - (*os).constraint;
                    if !((*os).bfd_section).is_null()
                        && !((*(*os).bfd_section).owner).is_null()
                    {
                        last_sec = (*os).bfd_section;
                        if first_sec.is_null() {
                            first_sec = last_sec;
                        }
                    }
                    if os == last_os {
                        break;
                    }
                    os = (*os).next;
                }
                if !last_sec.is_null() {
                    let mut sec: *mut asection = (*where_0).bfd_section;
                    if sec.is_null() {
                        sec = output_prev_sec_find(where_0);
                    }
                    if !sec.is_null() && sec != last_sec {
                        if !((*first_sec).prev).is_null() {
                            (*(*first_sec).prev).next = (*last_sec).next;
                        } else {
                            (*link_info.output_bfd).sections = (*last_sec).next;
                        }
                        if !((*last_sec).next).is_null() {
                            (*(*last_sec).next).prev = (*first_sec).prev;
                        } else {
                            (*link_info.output_bfd).section_last = (*first_sec).prev;
                        }
                        (*last_sec).next = (*sec).next;
                        if !((*sec).next).is_null() {
                            (*(*sec).next).prev = last_sec;
                        } else {
                            (*link_info.output_bfd).section_last = last_sec;
                        }
                        (*first_sec).prev = sec;
                        (*sec).next = first_sec;
                    }
                }
                first_os = 0 as *mut lang_output_section_statement_type;
                last_os = 0 as *mut lang_output_section_statement_type;
            }
            ptr = insert_os_after(where_0);
            first = *start;
            *start = (**s).header.next;
            *s = *ptr;
            if (*s).is_null() {
                statement_list.tail = s;
            }
            *ptr = first;
            s = start;
            continue;
        }
        s = &mut (**s).header.next;
    }
    os = first_os;
    while !os.is_null() {
        (*os).constraint = -(2 as libc::c_int) - (*os).constraint;
        if os == last_os {
            break;
        }
        os = (*os).next;
    }
}
unsafe extern "C" fn print_output_section_statement(
    mut output_section_statement: *mut lang_output_section_statement_type,
) {
    let mut section: *mut asection = (*output_section_statement).bfd_section;
    let mut len: libc::c_int = 0;
    if output_section_statement != abs_output_section {
        minfo(
            b"\n%s\0" as *const u8 as *const libc::c_char,
            (*output_section_statement).name,
        );
        if !section.is_null() {
            print_dot = (*section).vma;
            len = strlen((*output_section_statement).name) as libc::c_int;
            if len >= 16 as libc::c_int - 1 as libc::c_int {
                print_nl();
                len = 0 as libc::c_int;
            }
            while len < 16 as libc::c_int {
                print_space();
                len += 1;
                len;
            }
            minfo(
                b"0x%V %W\0" as *const u8 as *const libc::c_char,
                (*section).vma,
                (*section).size >> opb_shift,
            );
            if (*section).vma != (*section).lma {
                minfo(
                    dcgettext(
                        0 as *const libc::c_char,
                        b" load address 0x%V\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    (*section).lma,
                );
            }
            if !((*output_section_statement).update_dot_tree).is_null() {
                exp_fold_tree(
                    (*output_section_statement).update_dot_tree,
                    &mut *_bfd_std_section
                        .as_mut_ptr()
                        .offset(2 as libc::c_int as isize),
                    &mut print_dot,
                );
            }
        }
        print_nl();
    }
    print_statement_list(
        (*output_section_statement).children.head,
        output_section_statement,
    );
}
unsafe extern "C" fn print_assignment(
    mut assignment: *mut lang_assignment_statement_type,
    mut output_section: *mut lang_output_section_statement_type,
) {
    let mut i: libc::c_uint = 0;
    let mut is_dot: bool = false;
    let mut tree: *mut etree_type = 0 as *mut etree_type;
    let mut osec: *mut asection = 0 as *mut asection;
    i = 0 as libc::c_int as libc::c_uint;
    while i < 16 as libc::c_int as libc::c_uint {
        print_space();
        i = i.wrapping_add(1);
        i;
    }
    if (*(*assignment).exp).type_0.node_class as libc::c_uint
        == etree_assert as libc::c_int as libc::c_uint
    {
        is_dot = 0 as libc::c_int != 0;
        tree = (*(*assignment).exp).assert_s.child;
    } else {
        let mut dst: *const libc::c_char = (*(*assignment).exp).assign.dst;
        is_dot = *dst.offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32
            && *dst.offset(1 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int;
        tree = (*assignment).exp;
    }
    osec = (*output_section).bfd_section;
    if osec.is_null() {
        osec = &mut *_bfd_std_section.as_mut_ptr().offset(2 as libc::c_int as isize)
            as *mut asection;
    }
    if (*(*assignment).exp).type_0.node_class as libc::c_uint
        != etree_provide as libc::c_int as libc::c_uint
    {
        exp_fold_tree(tree, osec, &mut print_dot);
    } else {
        expld.result.valid_p = 0 as libc::c_int != 0;
    }
    if expld.result.valid_p {
        let mut value: bfd_vma = 0;
        if (*(*assignment).exp).type_0.node_class as libc::c_uint
            == etree_assert as libc::c_int as libc::c_uint || is_dot as libc::c_int != 0
            || !(expld.assign_name).is_null()
        {
            value = expld.result.value;
            if !(expld.result.section).is_null() {
                value = (value as libc::c_ulong)
                    .wrapping_add((*expld.result.section).vma) as bfd_vma as bfd_vma;
            }
            minfo(b"0x%V\0" as *const u8 as *const libc::c_char, value);
            if is_dot {
                print_dot = value;
            }
        } else {
            let mut h: *mut bfd_link_hash_entry = 0 as *mut bfd_link_hash_entry;
            h = bfd_link_hash_lookup(
                link_info.hash,
                (*(*assignment).exp).assign.dst,
                0 as libc::c_int != 0,
                0 as libc::c_int != 0,
                1 as libc::c_int != 0,
            );
            if !h.is_null()
                && ((*h).type_0() as libc::c_int == bfd_link_hash_defined as libc::c_int
                    || (*h).type_0() as libc::c_int
                        == bfd_link_hash_defweak as libc::c_int)
            {
                value = (*h).u.def.value;
                value = (value as libc::c_ulong)
                    .wrapping_add((*(*(*h).u.def.section).output_section).vma) as bfd_vma
                    as bfd_vma;
                value = (value as libc::c_ulong)
                    .wrapping_add((*(*h).u.def.section).output_offset) as bfd_vma
                    as bfd_vma;
                minfo(b"[0x%V]\0" as *const u8 as *const libc::c_char, value);
            } else {
                minfo(b"[unresolved]\0" as *const u8 as *const libc::c_char);
            }
        }
    } else {
        if (*(*assignment).exp).type_0.node_class as libc::c_uint
            == etree_provide as libc::c_int as libc::c_uint
        {
            minfo(b"[!provide]\0" as *const u8 as *const libc::c_char);
        } else {
            minfo(b"*undef*   \0" as *const u8 as *const libc::c_char);
        }
        minfo(b"        \0" as *const u8 as *const libc::c_char);
    }
    expld.assign_name = 0 as *const libc::c_char;
    minfo(b"                \0" as *const u8 as *const libc::c_char);
    exp_print_tree((*assignment).exp);
    print_nl();
}
unsafe extern "C" fn print_input_statement(mut statm: *mut lang_input_statement_type) {
    if !((*statm).filename).is_null() {
        fprintf(
            config.map_file,
            b"LOAD %s\n\0" as *const u8 as *const libc::c_char,
            (*statm).filename,
        );
    }
}
unsafe extern "C" fn hash_entry_addr_cmp(
    mut a: *const libc::c_void,
    mut b: *const libc::c_void,
) -> libc::c_int {
    let mut l: *const bfd_link_hash_entry = *(a as *mut *const bfd_link_hash_entry);
    let mut r: *const bfd_link_hash_entry = *(b as *mut *const bfd_link_hash_entry);
    if (*l).u.def.value < (*r).u.def.value {
        return -(1 as libc::c_int)
    } else if (*l).u.def.value > (*r).u.def.value {
        return 1 as libc::c_int
    } else {
        return 0 as libc::c_int
    };
}
unsafe extern "C" fn print_all_symbols(mut sec: *mut asection) {
    let mut ud: *mut input_section_userdata_type = bfd_section_userdata(sec)
        as *mut input_section_userdata_type;
    let mut def: *mut map_symbol_def = 0 as *mut map_symbol_def;
    let mut entries: *mut *mut bfd_link_hash_entry = 0 as *mut *mut bfd_link_hash_entry;
    let mut i: libc::c_uint = 0;
    if ud.is_null() {
        return;
    }
    *(*ud).map_symbol_def_tail = 0 as *mut map_symbol_def;
    entries = ({
        let mut __h: *mut obstack = &mut map_obstack as *mut obstack;
        let mut __o: *mut obstack = __h;
        let mut __len: size_t = ((*ud).map_symbol_def_count)
            .wrapping_mul(
                ::core::mem::size_of::<*mut bfd_link_hash_entry>() as libc::c_ulong,
            );
        if ({
            let mut __o1: *const obstack = __o;
            ((*__o1).chunk_limit).offset_from((*__o1).next_free) as libc::c_long
                as size_t
        }) < __len
        {
            _obstack_newchunk(__o, __len);
        }
        (*__o).next_free = ((*__o).next_free).offset(__len as isize);
        ({
            let mut __o1: *mut obstack = __h;
            let mut __value: *mut libc::c_void = (*__o1).object_base
                as *mut libc::c_void;
            if (*__o1).next_free == __value as *mut libc::c_char {
                (*__o1).set_maybe_empty_object(1 as libc::c_int as libc::c_uint);
            }
            (*__o1)
                .next_free = (if (::core::mem::size_of::<ptrdiff_t>() as libc::c_ulong)
                < ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
            {
                (*__o1).object_base
            } else {
                0 as *mut libc::c_char
            })
                .offset(
                    ((((*__o1).next_free)
                        .offset_from(
                            (if (::core::mem::size_of::<ptrdiff_t>() as libc::c_ulong)
                                < ::core::mem::size_of::<*mut libc::c_void>()
                                    as libc::c_ulong
                            {
                                (*__o1).object_base
                            } else {
                                0 as *mut libc::c_char
                            }),
                        ) as libc::c_long as libc::c_ulong)
                        .wrapping_add((*__o1).alignment_mask) & !(*__o1).alignment_mask)
                        as isize,
                );
            if ((*__o1).next_free).offset_from((*__o1).chunk as *mut libc::c_char)
                as libc::c_long as size_t
                > ((*__o1).chunk_limit).offset_from((*__o1).chunk as *mut libc::c_char)
                    as libc::c_long as size_t
            {
                (*__o1).next_free = (*__o1).chunk_limit;
            }
            (*__o1).object_base = (*__o1).next_free;
            __value
        })
    }) as *mut *mut bfd_link_hash_entry;
    i = 0 as libc::c_int as libc::c_uint;
    def = (*ud).map_symbol_def_head;
    while !def.is_null() {
        let ref mut fresh12 = *entries.offset(i as isize);
        *fresh12 = (*def).entry;
        def = (*def).next;
        i = i.wrapping_add(1);
        i;
    }
    qsort(
        entries as *mut libc::c_void,
        (*ud).map_symbol_def_count,
        ::core::mem::size_of::<*mut bfd_link_hash_entry>() as libc::c_ulong,
        Some(
            hash_entry_addr_cmp
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) < (*ud).map_symbol_def_count {
        ldemul_print_symbol(*entries.offset(i as isize), sec as *mut libc::c_void);
        i = i.wrapping_add(1);
        i;
    }
    let mut __o: *mut obstack = &mut map_obstack;
    let mut __obj: *mut libc::c_void = entries as *mut libc::c_void;
    if __obj > (*__o).chunk as *mut libc::c_void
        && __obj < (*__o).chunk_limit as *mut libc::c_void
    {
        (*__o).object_base = __obj as *mut libc::c_char;
        (*__o).next_free = (*__o).object_base;
    } else {
        _obstack_free(__o, __obj);
    };
}
unsafe extern "C" fn print_fill_statement(mut fill: *mut lang_fill_statement_type) {
    let mut size: size_t = 0;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    fputs(b" FILL mask 0x\0" as *const u8 as *const libc::c_char, config.map_file);
    p = ((*(*fill).fill).data).as_mut_ptr();
    size = (*(*fill).fill).size;
    while size != 0 as libc::c_int as libc::c_ulong {
        fprintf(
            config.map_file,
            b"%02x\0" as *const u8 as *const libc::c_char,
            *p as libc::c_int,
        );
        p = p.offset(1);
        p;
        size = size.wrapping_sub(1);
        size;
    }
    fputs(b"\n\0" as *const u8 as *const libc::c_char, config.map_file);
}
unsafe extern "C" fn print_data_statement(mut data: *mut lang_data_statement_type) {
    let mut i: libc::c_int = 0;
    let mut addr: bfd_vma = 0;
    let mut size: bfd_size_type = 0;
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    init_opb((*data).output_section);
    i = 0 as libc::c_int;
    while i < 16 as libc::c_int {
        print_space();
        i += 1;
        i;
    }
    addr = (*data).output_offset;
    if !((*data).output_section).is_null() {
        addr = (addr as libc::c_ulong).wrapping_add((*(*data).output_section).vma)
            as bfd_vma as bfd_vma;
    }
    match (*data).type_0 {
        286 => {
            size = 1 as libc::c_int as bfd_size_type;
            name = b"BYTE\0" as *const u8 as *const libc::c_char;
        }
        285 => {
            size = 2 as libc::c_int as bfd_size_type;
            name = b"SHORT\0" as *const u8 as *const libc::c_char;
        }
        284 => {
            size = 4 as libc::c_int as bfd_size_type;
            name = b"LONG\0" as *const u8 as *const libc::c_char;
        }
        282 => {
            size = 8 as libc::c_int as bfd_size_type;
            name = b"QUAD\0" as *const u8 as *const libc::c_char;
        }
        283 => {
            size = 8 as libc::c_int as bfd_size_type;
            name = b"SQUAD\0" as *const u8 as *const libc::c_char;
        }
        _ => {
            ld_abort(
                b"ldlang.c\0" as *const u8 as *const libc::c_char,
                4897 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 54],
                    &[libc::c_char; 54],
                >(b"void print_data_statement(lang_data_statement_type *)\0"))
                    .as_ptr(),
            );
        }
    }
    if size < ((1 as libc::c_int as libc::c_uint) << opb_shift) as libc::c_ulong {
        size = ((1 as libc::c_int as libc::c_uint) << opb_shift) as bfd_size_type;
    }
    minfo(
        b"0x%V %W %s 0x%v\0" as *const u8 as *const libc::c_char,
        addr,
        size >> opb_shift,
        name,
        (*data).value,
    );
    if (*(*data).exp).type_0.node_class as libc::c_uint
        != etree_value as libc::c_int as libc::c_uint
    {
        print_space();
        exp_print_tree((*data).exp);
    }
    print_nl();
    print_dot = addr.wrapping_add(size >> opb_shift);
}
unsafe extern "C" fn print_address_statement(
    mut address: *mut lang_address_statement_type,
) {
    minfo(
        dcgettext(
            0 as *const libc::c_char,
            b"Address of section %s set to \0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        (*address).section_name,
    );
    exp_print_tree((*address).address);
    print_nl();
}
unsafe extern "C" fn print_reloc_statement(mut reloc: *mut lang_reloc_statement_type) {
    let mut i: libc::c_int = 0;
    let mut addr: bfd_vma = 0;
    let mut size: bfd_size_type = 0;
    init_opb((*reloc).output_section);
    i = 0 as libc::c_int;
    while i < 16 as libc::c_int {
        print_space();
        i += 1;
        i;
    }
    addr = (*reloc).output_offset;
    if !((*reloc).output_section).is_null() {
        addr = (addr as libc::c_ulong).wrapping_add((*(*reloc).output_section).vma)
            as bfd_vma as bfd_vma;
    }
    size = bfd_get_reloc_size((*reloc).howto) as bfd_size_type;
    minfo(
        b"0x%V %W RELOC %s \0" as *const u8 as *const libc::c_char,
        addr,
        size >> opb_shift,
        (*(*reloc).howto).name,
    );
    if !((*reloc).name).is_null() {
        minfo(b"%s+\0" as *const u8 as *const libc::c_char, (*reloc).name);
    } else {
        minfo(b"%s+\0" as *const u8 as *const libc::c_char, (*(*reloc).section).name);
    }
    exp_print_tree((*reloc).addend_exp);
    print_nl();
    print_dot = addr.wrapping_add(size >> opb_shift);
}
unsafe extern "C" fn print_padding_statement(mut s: *mut lang_padding_statement_type) {
    let mut len: libc::c_int = 0;
    let mut addr: bfd_vma = 0;
    init_opb((*s).output_section);
    minfo(b" *fill*\0" as *const u8 as *const libc::c_char);
    len = (::core::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int;
    while len < 16 as libc::c_int {
        print_space();
        len += 1;
        len;
    }
    addr = (*s).output_offset;
    if !((*s).output_section).is_null() {
        addr = (addr as libc::c_ulong).wrapping_add((*(*s).output_section).vma)
            as bfd_vma as bfd_vma;
    }
    minfo(
        b"0x%V %W \0" as *const u8 as *const libc::c_char,
        addr,
        (*s).size >> opb_shift,
    );
    if (*(*s).fill).size != 0 as libc::c_int as libc::c_ulong {
        let mut size: size_t = 0;
        let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        p = ((*(*s).fill).data).as_mut_ptr();
        size = (*(*s).fill).size;
        while size != 0 as libc::c_int as libc::c_ulong {
            fprintf(
                config.map_file,
                b"%02x\0" as *const u8 as *const libc::c_char,
                *p as libc::c_int,
            );
            p = p.offset(1);
            p;
            size = size.wrapping_sub(1);
            size;
        }
    }
    print_nl();
    print_dot = addr.wrapping_add((*s).size >> opb_shift);
}
unsafe extern "C" fn print_wild_statement(
    mut w: *mut lang_wild_statement_type,
    mut os: *mut lang_output_section_statement_type,
) {
    let mut sec: *mut wildcard_list = 0 as *mut wildcard_list;
    print_space();
    if !((*w).exclude_name_list).is_null() {
        let mut tmp: *mut name_list = 0 as *mut name_list;
        minfo(
            b"EXCLUDE_FILE(%s\0" as *const u8 as *const libc::c_char,
            (*(*w).exclude_name_list).name,
        );
        tmp = (*(*w).exclude_name_list).next;
        while !tmp.is_null() {
            minfo(b" %s\0" as *const u8 as *const libc::c_char, (*tmp).name);
            tmp = (*tmp).next;
        }
        minfo(b") \0" as *const u8 as *const libc::c_char);
    }
    if (*w).filenames_sorted {
        minfo(b"SORT_BY_NAME(\0" as *const u8 as *const libc::c_char);
    }
    if !((*w).filename).is_null() {
        minfo(b"%s\0" as *const u8 as *const libc::c_char, (*w).filename);
    } else {
        minfo(b"*\0" as *const u8 as *const libc::c_char);
    }
    if (*w).filenames_sorted {
        minfo(b")\0" as *const u8 as *const libc::c_char);
    }
    minfo(b"(\0" as *const u8 as *const libc::c_char);
    sec = (*w).section_list;
    while !sec.is_null() {
        let mut closing_paren: libc::c_int = 0 as libc::c_int;
        match (*sec).spec.sorted as libc::c_uint {
            1 => {
                minfo(b"SORT_BY_NAME(\0" as *const u8 as *const libc::c_char);
                closing_paren = 1 as libc::c_int;
            }
            2 => {
                minfo(b"SORT_BY_ALIGNMENT(\0" as *const u8 as *const libc::c_char);
                closing_paren = 1 as libc::c_int;
            }
            3 => {
                minfo(
                    b"SORT_BY_NAME(SORT_BY_ALIGNMENT(\0" as *const u8
                        as *const libc::c_char,
                );
                closing_paren = 2 as libc::c_int;
            }
            4 => {
                minfo(
                    b"SORT_BY_ALIGNMENT(SORT_BY_NAME(\0" as *const u8
                        as *const libc::c_char,
                );
                closing_paren = 2 as libc::c_int;
            }
            5 => {
                minfo(b"SORT_NONE(\0" as *const u8 as *const libc::c_char);
                closing_paren = 1 as libc::c_int;
            }
            6 => {
                minfo(b"SORT_BY_INIT_PRIORITY(\0" as *const u8 as *const libc::c_char);
                closing_paren = 1 as libc::c_int;
            }
            0 | _ => {}
        }
        if !((*sec).spec.exclude_name_list).is_null() {
            let mut tmp_0: *mut name_list = 0 as *mut name_list;
            minfo(
                b"EXCLUDE_FILE(%s\0" as *const u8 as *const libc::c_char,
                (*(*sec).spec.exclude_name_list).name,
            );
            tmp_0 = (*(*sec).spec.exclude_name_list).next;
            while !tmp_0.is_null() {
                minfo(b" %s\0" as *const u8 as *const libc::c_char, (*tmp_0).name);
                tmp_0 = (*tmp_0).next;
            }
            minfo(b") \0" as *const u8 as *const libc::c_char);
        }
        if !((*sec).spec.name).is_null() {
            minfo(b"%s\0" as *const u8 as *const libc::c_char, (*sec).spec.name);
        } else {
            minfo(b"*\0" as *const u8 as *const libc::c_char);
        }
        while closing_paren > 0 as libc::c_int {
            minfo(b")\0" as *const u8 as *const libc::c_char);
            closing_paren -= 1;
            closing_paren;
        }
        if !((*sec).next).is_null() {
            minfo(b" \0" as *const u8 as *const libc::c_char);
        }
        sec = (*sec).next;
    }
    minfo(b")\0" as *const u8 as *const libc::c_char);
    print_nl();
    print_statement_list((*w).children.head, os);
}
unsafe extern "C" fn print_group(
    mut s: *mut lang_group_statement_type,
    mut os: *mut lang_output_section_statement_type,
) {
    fprintf(config.map_file, b"START GROUP\n\0" as *const u8 as *const libc::c_char);
    print_statement_list((*s).children.head, os);
    fprintf(config.map_file, b"END GROUP\n\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn insert_pad(
    mut ptr: *mut *mut lang_statement_union_type,
    mut fill: *mut fill_type,
    mut alignment_needed: bfd_size_type,
    mut output_section: *mut asection,
    mut dot: bfd_vma,
) {
    static mut zero_fill: fill_type = fill_type { size: 0, data: [0; 1] };
    let mut pad: *mut lang_statement_union_type = 0 as *mut lang_statement_union_type;
    if ptr != &mut statement_list.head as *mut *mut lang_statement_union {
        pad = (ptr as *mut libc::c_char).offset(-(0 as libc::c_ulong as isize))
            as *mut lang_statement_union_type;
    }
    if !(!pad.is_null()
        && (*pad).header.type_0 as libc::c_uint
            == lang_padding_statement_enum as libc::c_int as libc::c_uint
        && (*pad).padding_statement.output_section == output_section)
    {
        pad = *ptr;
        if !(!pad.is_null()
            && (*pad).header.type_0 as libc::c_uint
                == lang_padding_statement_enum as libc::c_int as libc::c_uint
            && (*pad).padding_statement.output_section == output_section)
        {
            pad = stat_alloc(
                ::core::mem::size_of::<lang_padding_statement_type>() as libc::c_ulong,
            ) as *mut lang_statement_union_type;
            (*pad).header.next = *ptr;
            *ptr = pad;
            (*pad).header.type_0 = lang_padding_statement_enum;
            (*pad).padding_statement.output_section = output_section;
            if fill.is_null() {
                fill = &mut zero_fill;
            }
            (*pad).padding_statement.fill = fill;
        }
    }
    (*pad).padding_statement.output_offset = dot.wrapping_sub((*output_section).vma);
    (*pad).padding_statement.size = alignment_needed;
    if (*output_section).flags & 0x800 as libc::c_int as libc::c_uint == 0 {
        (*output_section)
            .size = dot
            .wrapping_add(alignment_needed >> opb_shift)
            .wrapping_sub((*output_section).vma) << opb_shift;
    }
}
unsafe extern "C" fn size_input_section(
    mut this_ptr: *mut *mut lang_statement_union_type,
    mut output_section_statement: *mut lang_output_section_statement_type,
    mut fill: *mut fill_type,
    mut removed: *mut bool,
    mut dot: bfd_vma,
) -> bfd_vma {
    let mut is: *mut lang_input_section_type = &mut (**this_ptr).input_section;
    let mut i: *mut asection = (*is).section;
    let mut o: *mut asection = (*output_section_statement).bfd_section;
    *removed = 0 as libc::c_int != 0;
    if link_info.non_contiguous_regions() != 0 {
        if !((*i).already_assigned).is_null() && (*i).already_assigned != o {
            *removed = 1 as libc::c_int != 0;
            return dot;
        }
    }
    if (*i).sec_info_type() as libc::c_int == 4 as libc::c_int {
        (*i).output_offset = ((*i).vma).wrapping_sub((*o).vma);
    } else if (*i).flags & 0x8000 as libc::c_int as libc::c_uint
        != 0 as libc::c_int as libc::c_uint
        || (*output_section_statement).ignored() as libc::c_int != 0
    {
        (*i).output_offset = dot.wrapping_sub((*o).vma);
    } else {
        let mut alignment_needed: bfd_size_type = 0;
        if !((*output_section_statement).subsection_alignment).is_null() {
            (*i)
                .alignment_power = exp_get_power(
                (*output_section_statement).subsection_alignment,
                b"subsection alignment\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            ) as libc::c_uint;
        }
        if (*o).alignment_power < (*i).alignment_power {
            (*o).alignment_power = (*i).alignment_power;
        }
        alignment_needed = (dot
            .wrapping_add((1 as libc::c_int as bfd_vma) << (*i).alignment_power)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            & ((1 as libc::c_int as bfd_vma) << (*i).alignment_power).wrapping_neg())
            .wrapping_sub(dot);
        if alignment_needed != 0 as libc::c_int as libc::c_ulong {
            insert_pad(this_ptr, fill, alignment_needed << opb_shift, o, dot);
            dot = (dot as libc::c_ulong).wrapping_add(alignment_needed) as bfd_vma
                as bfd_vma;
        }
        if link_info.non_contiguous_regions() != 0 {
            if !((*output_section_statement).region).is_null() {
                let mut end: bfd_vma = ((*(*output_section_statement).region).origin)
                    .wrapping_add((*(*output_section_statement).region).length);
                if dot.wrapping_add((*i).size >> opb_shift) > end {
                    if (*i).flags & 0x100000 as libc::c_int as libc::c_uint != 0 {
                        einfo(
                            dcgettext(
                                0 as *const libc::c_char,
                                b"%F%P: Output section '%s' not large enough for the linker-created stubs section '%s'.\n\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            (*(*i).output_section).name,
                            (*i).name,
                        );
                    }
                    if (*i).rawsize != 0 && (*i).rawsize != (*i).size {
                        einfo(
                            dcgettext(
                                0 as *const libc::c_char,
                                b"%F%P: Relaxation not supported with --enable-non-contiguous-regions (section '%s' would overflow '%s' after it changed size).\n\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            (*i).name,
                            (*(*i).output_section).name,
                        );
                    }
                    *removed = 1 as libc::c_int != 0;
                    dot = end;
                    (*i).output_section = 0 as *mut bfd_section;
                    return dot;
                }
            }
        }
        (*i).output_offset = dot.wrapping_sub((*o).vma);
        dot = (dot as libc::c_ulong).wrapping_add((*i).size >> opb_shift) as bfd_vma
            as bfd_vma;
        if (*o).flags & 0x800 as libc::c_int as libc::c_uint == 0 {
            (*o).size = dot.wrapping_sub((*o).vma) << opb_shift;
        }
        if link_info.non_contiguous_regions() != 0 {
            (*i).already_assigned = o;
            (*i).output_section = o;
        }
    }
    return dot;
}
unsafe extern "C" fn sort_sections_by_lma(
    mut arg1: *const libc::c_void,
    mut arg2: *const libc::c_void,
) -> libc::c_int {
    let mut sec1: *const asection = (*(arg1 as *const check_sec)).sec;
    let mut sec2: *const asection = (*(arg2 as *const check_sec)).sec;
    if (*sec1).lma < (*sec2).lma {
        return -(1 as libc::c_int)
    } else if (*sec1).lma > (*sec2).lma {
        return 1 as libc::c_int
    } else if (*sec1).id < (*sec2).id {
        return -(1 as libc::c_int)
    } else if (*sec1).id > (*sec2).id {
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn sort_sections_by_vma(
    mut arg1: *const libc::c_void,
    mut arg2: *const libc::c_void,
) -> libc::c_int {
    let mut sec1: *const asection = (*(arg1 as *const check_sec)).sec;
    let mut sec2: *const asection = (*(arg2 as *const check_sec)).sec;
    if (*sec1).vma < (*sec2).vma {
        return -(1 as libc::c_int)
    } else if (*sec1).vma > (*sec2).vma {
        return 1 as libc::c_int
    } else if (*sec1).id < (*sec2).id {
        return -(1 as libc::c_int)
    } else if (*sec1).id > (*sec2).id {
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn lang_check_section_addresses() {
    let mut s: *mut asection = 0 as *mut asection;
    let mut p: *mut asection = 0 as *mut asection;
    let mut sections: *mut check_sec = 0 as *mut check_sec;
    let mut i: size_t = 0;
    let mut count: size_t = 0;
    let mut addr_mask: bfd_vma = 0;
    let mut s_start: bfd_vma = 0;
    let mut s_end: bfd_vma = 0;
    let mut p_start: bfd_vma = 0 as libc::c_int as bfd_vma;
    let mut p_end: bfd_vma = 0 as libc::c_int as bfd_vma;
    let mut m: *mut lang_memory_region_type = 0 as *mut lang_memory_region_type;
    let mut overlays: bool = false;
    addr_mask = ((1 as libc::c_int as bfd_vma)
        << (bfd_arch_bits_per_address(link_info.output_bfd))
            .wrapping_sub(1 as libc::c_int as libc::c_uint))
        .wrapping_sub(1 as libc::c_int as libc::c_ulong);
    addr_mask = (addr_mask << 1 as libc::c_int)
        .wrapping_add(1 as libc::c_int as libc::c_ulong);
    s = (*link_info.output_bfd).sections;
    while !s.is_null() {
        if (*s).flags & 0x1 as libc::c_int as libc::c_uint
            != 0 as libc::c_int as libc::c_uint
        {
            s_end = ((*s).vma).wrapping_add((*s).size) & addr_mask;
            if s_end != 0 as libc::c_int as libc::c_ulong && s_end < (*s).vma & addr_mask
            {
                einfo(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%X%P: section %s VMA wraps around address space\n\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    (*s).name,
                );
            } else {
                s_end = ((*s).lma).wrapping_add((*s).size) & addr_mask;
                if s_end != 0 as libc::c_int as libc::c_ulong
                    && s_end < (*s).lma & addr_mask
                {
                    einfo(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"%X%P: section %s LMA wraps around address space\n\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        (*s).name,
                    );
                }
            }
        }
        s = (*s).next;
    }
    if bfd_count_sections(link_info.output_bfd) <= 1 as libc::c_int as libc::c_uint {
        return;
    }
    count = bfd_count_sections(link_info.output_bfd) as size_t;
    sections = xmalloc(
        (::core::mem::size_of::<check_sec>() as libc::c_ulong).wrapping_mul(count),
    ) as *mut check_sec;
    count = 0 as libc::c_int as size_t;
    s = (*link_info.output_bfd).sections;
    while !s.is_null() {
        if !((*s).flags & 0x1 as libc::c_int as libc::c_uint
            == 0 as libc::c_int as libc::c_uint
            || (*s).flags & (0x2 as libc::c_int | 0x400 as libc::c_int) as libc::c_uint
                == 0x400 as libc::c_int as libc::c_uint
            || (*s).size == 0 as libc::c_int as libc::c_ulong)
        {
            let ref mut fresh13 = (*sections.offset(count as isize)).sec;
            *fresh13 = s;
            (*sections.offset(count as isize)).warned = 0 as libc::c_int != 0;
            count = count.wrapping_add(1);
            count;
        }
        s = (*s).next;
    }
    if count <= 1 as libc::c_int as libc::c_ulong {
        free(sections as *mut libc::c_void);
        return;
    }
    qsort(
        sections as *mut libc::c_void,
        count,
        ::core::mem::size_of::<check_sec>() as libc::c_ulong,
        Some(
            sort_sections_by_lma
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    p = 0 as *mut asection;
    i = 0 as libc::c_int as size_t;
    while i < count {
        s = (*sections.offset(i as isize)).sec;
        init_opb(s);
        if (*s).flags & 0x2 as libc::c_int as libc::c_uint
            != 0 as libc::c_int as libc::c_uint
        {
            s_start = (*s).lma;
            s_end = s_start
                .wrapping_add((*s).size >> opb_shift)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong);
            if !p.is_null() && (s_start <= p_end || p_end < p_start) {
                einfo(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%X%P: section %s LMA [%V,%V] overlaps section %s LMA [%V,%V]\n\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    (*s).name,
                    s_start,
                    s_end,
                    (*p).name,
                    p_start,
                    p_end,
                );
                (*sections.offset(i as isize)).warned = 1 as libc::c_int != 0;
            }
            p = s;
            p_start = s_start;
            p_end = s_end;
        }
        i = i.wrapping_add(1);
        i;
    }
    qsort(
        sections as *mut libc::c_void,
        count,
        ::core::mem::size_of::<check_sec>() as libc::c_ulong,
        Some(
            sort_sections_by_vma
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    overlays = 0 as libc::c_int != 0;
    p_start = (*(*sections.offset(0 as libc::c_int as isize)).sec).vma;
    i = 1 as libc::c_int as size_t;
    while i < count {
        s_start = (*(*sections.offset(i as isize)).sec).vma;
        if p_start == s_start {
            overlays = 1 as libc::c_int != 0;
            break;
        } else {
            p_start = s_start;
            i = i.wrapping_add(1);
            i;
        }
    }
    if !overlays {
        p = 0 as *mut asection;
        i = 0 as libc::c_int as size_t;
        while i < count {
            s = (*sections.offset(i as isize)).sec;
            init_opb(s);
            s_start = (*s).vma;
            s_end = s_start
                .wrapping_add((*s).size >> opb_shift)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong);
            if !p.is_null() && !(*sections.offset(i as isize)).warned
                && (s_start <= p_end || p_end < p_start)
            {
                einfo(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%X%P: section %s VMA [%V,%V] overlaps section %s VMA [%V,%V]\n\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    (*s).name,
                    s_start,
                    s_end,
                    (*p).name,
                    p_start,
                    p_end,
                );
            }
            p = s;
            p_start = s_start;
            p_end = s_end;
            i = i.wrapping_add(1);
            i;
        }
    }
    free(sections as *mut libc::c_void);
    m = lang_memory_region_list;
    while !m.is_null() {
        if (*m).had_full_message {
            let mut over: libc::c_ulong = ((*m).current)
                .wrapping_sub(((*m).origin).wrapping_add((*m).length));
            einfo(
                dcngettext(
                    0 as *const libc::c_char,
                    b"%X%P: region `%s' overflowed by %lu byte\n\0" as *const u8
                        as *const libc::c_char,
                    b"%X%P: region `%s' overflowed by %lu bytes\n\0" as *const u8
                        as *const libc::c_char,
                    over,
                    5 as libc::c_int,
                ),
                (*m).name_list.name,
                over,
            );
        }
        m = (*m).next;
    }
}
unsafe extern "C" fn os_region_check(
    mut os: *mut lang_output_section_statement_type,
    mut region: *mut lang_memory_region_type,
    mut tree: *mut etree_type,
    mut rbase: bfd_vma,
) {
    if ((*region).current < (*region).origin
        || ((*region).current).wrapping_sub((*region).origin) > (*region).length)
        && ((*region).current != ((*region).origin).wrapping_add((*region).length)
            || rbase == 0 as libc::c_int as libc::c_ulong)
    {
        if !tree.is_null() {
            einfo(
                dcgettext(
                    0 as *const libc::c_char,
                    b"%X%P: address 0x%v of %pB section `%s' is not within region `%s'\n\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*region).current,
                (*(*os).bfd_section).owner,
                (*(*os).bfd_section).name,
                (*region).name_list.name,
            );
        } else if !(*region).had_full_message {
            (*region).had_full_message = 1 as libc::c_int != 0;
            einfo(
                dcgettext(
                    0 as *const libc::c_char,
                    b"%X%P: %pB section `%s' will not fit in region `%s'\n\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*(*os).bfd_section).owner,
                (*(*os).bfd_section).name,
                (*region).name_list.name,
            );
        }
    }
}
unsafe extern "C" fn ldlang_check_relro_region(
    mut s: *mut lang_statement_union_type,
    mut seg: *mut seg_align_type,
) {
    if (*seg).relro as libc::c_uint == exp_seg_relro_start as libc::c_int as libc::c_uint
    {
        if ((*seg).relro_start_stat).is_null() {
            (*seg).relro_start_stat = s;
        } else if !((*seg).relro_start_stat == s) {
            info_assert(
                b"ldlang.c\0" as *const u8 as *const libc::c_char,
                5645 as libc::c_int as libc::c_uint,
            );
        }
    } else if (*seg).relro as libc::c_uint
        == exp_seg_relro_end as libc::c_int as libc::c_uint
    {
        if ((*seg).relro_end_stat).is_null() {
            (*seg).relro_end_stat = s;
        } else if !((*seg).relro_end_stat == s) {
            info_assert(
                b"ldlang.c\0" as *const u8 as *const libc::c_char,
                5654 as libc::c_int as libc::c_uint,
            );
        }
    }
}
unsafe extern "C" fn lang_size_sections_1(
    mut prev: *mut *mut lang_statement_union_type,
    mut output_section_statement: *mut lang_output_section_statement_type,
    mut fill: *mut fill_type,
    mut dot: bfd_vma,
    mut relax: *mut bool,
    mut check_regions: bool,
) -> bfd_vma {
    let mut s: *mut lang_statement_union_type = 0 as *mut lang_statement_union_type;
    let mut prev_s: *mut lang_statement_union_type = 0 as *mut lang_statement_union_type;
    let mut removed_prev_s: bool = 0 as libc::c_int != 0;
    s = *prev;
    while !s.is_null() {
        let mut removed: bool = 0 as libc::c_int != 0;
        match (*s).header.type_0 as libc::c_uint {
            8 => {
                let mut newdot: bfd_vma = 0;
                let mut after: bfd_vma = 0;
                let mut dotdelta: bfd_vma = 0;
                let mut os: *mut lang_output_section_statement_type = 0
                    as *mut lang_output_section_statement_type;
                let mut r: *mut lang_memory_region_type = 0
                    as *mut lang_memory_region_type;
                let mut section_alignment: libc::c_int = 0 as libc::c_int;
                os = &mut (*s).output_section_statement;
                init_opb((*os).bfd_section);
                if !((*os).constraint == -(1 as libc::c_int)) {
                    if ((*os).addr_tree).is_null()
                        && link_info.type_0() as libc::c_int
                            == type_relocatable as libc::c_int
                        && bfd_get_flavour(link_info.output_bfd) as libc::c_uint
                            == bfd_target_coff_flavour as libc::c_int as libc::c_uint
                    {
                        (*os).addr_tree = exp_intop(0 as libc::c_int as bfd_vma);
                    }
                    if !((*os).addr_tree).is_null() {
                        (*os).set_processed_vma(0 as libc::c_int as libc::c_uint);
                        exp_fold_tree(
                            (*os).addr_tree,
                            &mut *_bfd_std_section
                                .as_mut_ptr()
                                .offset(2 as libc::c_int as isize),
                            &mut dot,
                        );
                        if expld.result.valid_p {
                            dot = expld.result.value;
                            if !(expld.result.section).is_null() {
                                dot = (dot as libc::c_ulong)
                                    .wrapping_add((*expld.result.section).vma) as bfd_vma
                                    as bfd_vma;
                            }
                        } else if expld.phase as libc::c_uint
                            != lang_mark_phase_enum as libc::c_int as libc::c_uint
                        {
                            einfo(
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"%F%P:%pS: non constant or forward reference address expression for section %s\n\0"
                                        as *const u8 as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                                (*os).addr_tree,
                                (*os).name,
                            );
                        }
                    }
                    if !((*os).bfd_section).is_null() {
                        if (bfd_get_flavour(link_info.output_bfd) as libc::c_uint
                            == bfd_target_ecoff_flavour as libc::c_int as libc::c_uint
                            || bfd_get_flavour(link_info.output_bfd) as libc::c_uint
                                == bfd_target_coff_flavour as libc::c_int as libc::c_uint)
                            && (*(*os).bfd_section).flags
                                & 0x4000000 as libc::c_int as libc::c_uint
                                != 0 as libc::c_int as libc::c_uint
                        {
                            let mut input: *mut asection = 0 as *mut asection;
                            if ((*os).children.head).is_null()
                                || !((*(*os).children.head).header.next).is_null()
                                || (*(*os).children.head).header.type_0 as libc::c_uint
                                    != lang_input_section_enum as libc::c_int as libc::c_uint
                            {
                                einfo(
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"%X%P: internal error on COFF shared library section %s\n\0"
                                            as *const u8 as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                    (*os).name,
                                );
                            }
                            input = (*(*os).children.head).input_section.section;
                            bfd_set_section_vma(
                                (*os).bfd_section,
                                bfd_section_vma(input),
                            );
                            if (*(*os).bfd_section).flags
                                & 0x800 as libc::c_int as libc::c_uint == 0
                            {
                                (*(*os).bfd_section).size = (*input).size;
                            }
                        } else {
                            newdot = dot;
                            dotdelta = 0 as libc::c_int as bfd_vma;
                            if bfd_is_abs_section((*os).bfd_section) {
                                if !((*(*os).bfd_section).vma
                                    == 0 as libc::c_int as libc::c_ulong)
                                {
                                    info_assert(
                                        b"ldlang.c\0" as *const u8 as *const libc::c_char,
                                        5754 as libc::c_int as libc::c_uint,
                                    );
                                }
                            } else {
                                if ((*os).addr_tree).is_null() {
                                    if ((*os).region).is_null()
                                        || (*(*os).bfd_section).flags
                                            & (0x1 as libc::c_int | 0x2 as libc::c_int) as libc::c_uint
                                            != 0
                                            && *((*(*os).region).name_list.name)
                                                .offset(0 as libc::c_int as isize) as libc::c_int
                                                == '*' as i32
                                            && strcmp(
                                                (*(*os).region).name_list.name,
                                                b"*default*\0" as *const u8 as *const libc::c_char,
                                            ) == 0 as libc::c_int
                                    {
                                        (*os).region = lang_memory_default((*os).bfd_section);
                                    }
                                    if (*os).ignored() == 0
                                        && !((*(*os).bfd_section).flags
                                            & 0x1 as libc::c_int as libc::c_uint
                                            == 0 as libc::c_int as libc::c_uint
                                            || (*(*os).bfd_section).flags
                                                & (0x2 as libc::c_int | 0x400 as libc::c_int)
                                                    as libc::c_uint == 0x400 as libc::c_int as libc::c_uint)
                                        && !(link_info.type_0() as libc::c_int
                                            == type_relocatable as libc::c_int)
                                        && check_regions as libc::c_int != 0
                                        && strcmp(
                                            (*(*os).region).name_list.name,
                                            b"*default*\0" as *const u8 as *const libc::c_char,
                                        ) == 0 as libc::c_int && !lang_memory_region_list.is_null()
                                        && (strcmp(
                                            (*lang_memory_region_list).name_list.name,
                                            b"*default*\0" as *const u8 as *const libc::c_char,
                                        ) != 0 as libc::c_int
                                            || !((*lang_memory_region_list).next).is_null())
                                        && lang_sizing_iteration == 1 as libc::c_int
                                    {
                                        if command_line.check_section_addresses != 0 {
                                            einfo(
                                                dcgettext(
                                                    0 as *const libc::c_char,
                                                    b"%F%P: error: no memory region specified for loadable section `%s'\n\0"
                                                        as *const u8 as *const libc::c_char,
                                                    5 as libc::c_int,
                                                ),
                                                bfd_section_name((*os).bfd_section),
                                            );
                                        } else {
                                            einfo(
                                                dcgettext(
                                                    0 as *const libc::c_char,
                                                    b"%P: warning: no memory region specified for loadable section `%s'\n\0"
                                                        as *const u8 as *const libc::c_char,
                                                    5 as libc::c_int,
                                                ),
                                                bfd_section_name((*os).bfd_section),
                                            );
                                        }
                                    }
                                    newdot = (*(*os).region).current;
                                    section_alignment = (*(*os).bfd_section).alignment_power
                                        as libc::c_int;
                                } else {
                                    section_alignment = exp_get_power(
                                        (*os).section_alignment,
                                        b"section alignment\0" as *const u8 as *const libc::c_char
                                            as *mut libc::c_char,
                                    );
                                }
                                if section_alignment > 0 as libc::c_int {
                                    let mut savedot: bfd_vma = newdot;
                                    let mut diff: bfd_vma = 0 as libc::c_int as bfd_vma;
                                    newdot = newdot
                                        .wrapping_add(
                                            (1 as libc::c_int as bfd_vma) << section_alignment,
                                        )
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        & ((1 as libc::c_int as bfd_vma) << section_alignment)
                                            .wrapping_neg();
                                    dotdelta = newdot.wrapping_sub(savedot);
                                    if lang_sizing_iteration == 1 as libc::c_int {
                                        diff = dotdelta;
                                    } else if lang_sizing_iteration > 1 as libc::c_int {
                                        diff = newdot.wrapping_sub((*(*os).bfd_section).vma);
                                        if diff
                                            & ((1 as libc::c_int as bfd_vma) << section_alignment)
                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong) == 0
                                        {
                                            diff = 0 as libc::c_int as bfd_vma;
                                        }
                                    }
                                    if diff != 0 as libc::c_int as libc::c_ulong
                                        && (config.warn_section_align as libc::c_int != 0
                                            || !((*os).addr_tree).is_null())
                                    {
                                        einfo(
                                            dcgettext(
                                                0 as *const libc::c_char,
                                                b"%P: warning: start of section %s changed by %ld\n\0"
                                                    as *const u8 as *const libc::c_char,
                                                5 as libc::c_int,
                                            ),
                                            (*os).name,
                                            diff as libc::c_long,
                                        );
                                    }
                                }
                                bfd_set_section_vma((*os).bfd_section, newdot);
                                (*(*os).bfd_section)
                                    .output_offset = 0 as libc::c_int as bfd_vma;
                            }
                            lang_size_sections_1(
                                &mut (*os).children.head,
                                os,
                                (*os).fill,
                                newdot,
                                relax,
                                check_regions,
                            );
                            (*os).set_processed_vma(1 as libc::c_int as libc::c_uint);
                            if bfd_is_abs_section((*os).bfd_section) as libc::c_int != 0
                                || (*os).ignored() as libc::c_int != 0
                            {
                                if !((*(*os).bfd_section).size
                                    == 0 as libc::c_int as libc::c_ulong)
                                {
                                    info_assert(
                                        b"ldlang.c\0" as *const u8 as *const libc::c_char,
                                        5855 as libc::c_int as libc::c_uint,
                                    );
                                }
                            } else {
                                dot = (*(*os).bfd_section).vma;
                                after = dot
                                    .wrapping_add((*(*os).bfd_section).size >> opb_shift)
                                    .wrapping_add((*os).block_value as libc::c_ulong)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    & ((*os).block_value as bfd_vma).wrapping_neg();
                                if (*(*os).bfd_section).flags
                                    & 0x800 as libc::c_int as libc::c_uint == 0
                                {
                                    (*(*os).bfd_section)
                                        .size = after.wrapping_sub((*(*os).bfd_section).vma)
                                        << opb_shift;
                                }
                            }
                            r = (*os).region;
                            if r.is_null() {
                                r = lang_memory_region_lookup(
                                    b"*default*\0" as *const u8 as *const libc::c_char,
                                    0 as libc::c_int != 0,
                                );
                            }
                            if !((*os).load_base).is_null() {
                                let mut lma: bfd_vma = exp_get_abs_int(
                                    (*os).load_base,
                                    0 as libc::c_int,
                                    b"load base\0" as *const u8 as *const libc::c_char
                                        as *mut libc::c_char,
                                );
                                (*(*os).bfd_section).lma = lma;
                            } else if !((*os).lma_region).is_null() {
                                let mut lma_0: bfd_vma = (*(*os).lma_region).current;
                                if (*os).align_lma_with_input() != 0 {
                                    lma_0 = (lma_0 as libc::c_ulong).wrapping_add(dotdelta)
                                        as bfd_vma as bfd_vma;
                                } else {
                                    if (*os).lma_region != (*os).region {
                                        section_alignment = exp_get_power(
                                            (*os).section_alignment,
                                            b"section alignment\0" as *const u8 as *const libc::c_char
                                                as *mut libc::c_char,
                                        );
                                    }
                                    if section_alignment > 0 as libc::c_int {
                                        lma_0 = lma_0
                                            .wrapping_add(
                                                (1 as libc::c_int as bfd_vma) << section_alignment,
                                            )
                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            & ((1 as libc::c_int as bfd_vma) << section_alignment)
                                                .wrapping_neg();
                                    }
                                }
                                (*(*os).bfd_section).lma = lma_0;
                            } else if !((*r).last_os).is_null()
                                && (*(*os).bfd_section).flags
                                    & 0x1 as libc::c_int as libc::c_uint
                                    != 0 as libc::c_int as libc::c_uint
                            {
                                let mut lma_1: bfd_vma = 0;
                                let mut last: *mut asection = 0 as *mut asection;
                                last = (*(*r).last_os).output_section_statement.bfd_section;
                                if dot < (*last).vma
                                    && (*(*os).bfd_section).size
                                        != 0 as libc::c_int as libc::c_ulong
                                    && dot.wrapping_add((*(*os).bfd_section).size >> opb_shift)
                                        <= (*last).vma
                                {
                                    if (*last).vma != (*last).lma {
                                        einfo(
                                            dcgettext(
                                                0 as *const libc::c_char,
                                                b"%P: warning: dot moved backwards before `%s'\n\0"
                                                    as *const u8 as *const libc::c_char,
                                                5 as libc::c_int,
                                            ),
                                            (*os).name,
                                        );
                                    }
                                } else {
                                    if (*os).sectype as libc::c_uint
                                        == overlay_section as libc::c_int as libc::c_uint
                                    {
                                        lma_1 = ((*last).lma)
                                            .wrapping_add((*last).size >> opb_shift);
                                    } else {
                                        lma_1 = ((*(*os).bfd_section).vma)
                                            .wrapping_add((*last).lma)
                                            .wrapping_sub((*last).vma);
                                    }
                                    if section_alignment > 0 as libc::c_int {
                                        lma_1 = lma_1
                                            .wrapping_add(
                                                (1 as libc::c_int as bfd_vma) << section_alignment,
                                            )
                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            & ((1 as libc::c_int as bfd_vma) << section_alignment)
                                                .wrapping_neg();
                                    }
                                    (*(*os).bfd_section).lma = lma_1;
                                }
                            }
                            (*os).set_processed_lma(1 as libc::c_int as libc::c_uint);
                            if (!((*(*os).bfd_section).flags
                                & 0x1 as libc::c_int as libc::c_uint
                                == 0 as libc::c_int as libc::c_uint
                                || (*(*os).bfd_section).flags
                                    & (0x2 as libc::c_int | 0x400 as libc::c_int)
                                        as libc::c_uint == 0x400 as libc::c_int as libc::c_uint)
                                && ((*(*os).bfd_section).size
                                    != 0 as libc::c_int as libc::c_ulong
                                    || ((*r).last_os).is_null()
                                        && (*(*os).bfd_section).vma != (*(*os).bfd_section).lma
                                    || !((*r).last_os).is_null()
                                        && dot
                                            >= (*(*(*r).last_os).output_section_statement.bfd_section)
                                                .vma)
                                || (*os).sectype as libc::c_uint
                                    == first_overlay_section as libc::c_int as libc::c_uint)
                                && ((*os).lma_region).is_null()
                                && !(link_info.type_0() as libc::c_int
                                    == type_relocatable as libc::c_int)
                            {
                                (*r).last_os = s;
                            }
                            if !(bfd_is_abs_section((*os).bfd_section) as libc::c_int
                                != 0 || (*os).ignored() as libc::c_int != 0)
                            {
                                if !((*(*os).bfd_section).flags
                                    & (0x2 as libc::c_int | 0x400 as libc::c_int)
                                        as libc::c_uint == 0x400 as libc::c_int as libc::c_uint)
                                    || link_info.type_0() as libc::c_int
                                        == type_relocatable as libc::c_int
                                {
                                    dotdelta = (*(*os).bfd_section).size >> opb_shift;
                                } else {
                                    dotdelta = 0 as libc::c_int as bfd_vma;
                                }
                                dot = (dot as libc::c_ulong).wrapping_add(dotdelta)
                                    as bfd_vma as bfd_vma;
                                if !((*os).update_dot_tree).is_null() {
                                    exp_fold_tree(
                                        (*os).update_dot_tree,
                                        &mut *_bfd_std_section
                                            .as_mut_ptr()
                                            .offset(2 as libc::c_int as isize),
                                        &mut dot,
                                    );
                                }
                                if !((*os).region).is_null()
                                    && (*(*os).bfd_section).flags
                                        & (0x1 as libc::c_int | 0x2 as libc::c_int) as libc::c_uint
                                        != 0
                                {
                                    (*(*os).region).current = dot;
                                    if check_regions {
                                        os_region_check(
                                            os,
                                            (*os).region,
                                            (*os).addr_tree,
                                            (*(*os).bfd_section).vma,
                                        );
                                    }
                                    if !((*os).lma_region).is_null()
                                        && (*os).lma_region != (*os).region
                                        && ((*(*os).bfd_section).flags
                                            & 0x2 as libc::c_int as libc::c_uint != 0
                                            || (*os).align_lma_with_input() as libc::c_int != 0)
                                    {
                                        (*(*os).lma_region)
                                            .current = ((*(*os).bfd_section).lma)
                                            .wrapping_add(dotdelta);
                                        if check_regions {
                                            os_region_check(
                                                os,
                                                (*os).lma_region,
                                                0 as *mut etree_type,
                                                (*(*os).bfd_section).lma,
                                            );
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            14 => {
                dot = lang_size_sections_1(
                    &mut constructor_list.head,
                    output_section_statement,
                    fill,
                    dot,
                    relax,
                    check_regions,
                );
            }
            2 => {
                let mut size: libc::c_uint = 0 as libc::c_int as libc::c_uint;
                (*s)
                    .data_statement
                    .output_offset = dot
                    .wrapping_sub((*(*output_section_statement).bfd_section).vma);
                (*s)
                    .data_statement
                    .output_section = (*output_section_statement).bfd_section;
                exp_fold_tree(
                    (*s).data_statement.exp,
                    &mut *_bfd_std_section
                        .as_mut_ptr()
                        .offset(2 as libc::c_int as isize),
                    &mut dot,
                );
                match (*s).data_statement.type_0 {
                    282 | 283 => {
                        size = 8 as libc::c_int as libc::c_uint;
                    }
                    284 => {
                        size = 4 as libc::c_int as libc::c_uint;
                    }
                    285 => {
                        size = 2 as libc::c_int as libc::c_uint;
                    }
                    286 => {
                        size = 1 as libc::c_int as libc::c_uint;
                    }
                    _ => {
                        ld_abort(
                            b"ldlang.c\0" as *const u8 as *const libc::c_char,
                            6038 as libc::c_int,
                            (*::core::mem::transmute::<
                                &[u8; 135],
                                &[libc::c_char; 135],
                            >(
                                b"bfd_vma lang_size_sections_1(lang_statement_union_type **, lang_output_section_statement_type *, fill_type *, bfd_vma, _Bool *, _Bool)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                }
                if size < (1 as libc::c_int as libc::c_uint) << opb_shift {
                    size = (1 as libc::c_int as libc::c_uint) << opb_shift;
                }
                dot = (dot as libc::c_ulong)
                    .wrapping_add((size >> opb_shift) as libc::c_ulong) as bfd_vma
                    as bfd_vma;
                if (*(*output_section_statement).bfd_section).flags
                    & 0x800 as libc::c_int as libc::c_uint == 0
                {
                    (*(*output_section_statement).bfd_section)
                        .size = dot
                        .wrapping_sub((*(*output_section_statement).bfd_section).vma)
                        << opb_shift;
                }
            }
            11 => {
                let mut size_0: libc::c_int = 0;
                (*s)
                    .reloc_statement
                    .output_offset = dot
                    .wrapping_sub((*(*output_section_statement).bfd_section).vma);
                (*s)
                    .reloc_statement
                    .output_section = (*output_section_statement).bfd_section;
                size_0 = bfd_get_reloc_size((*s).reloc_statement.howto) as libc::c_int;
                dot = (dot as libc::c_ulong)
                    .wrapping_add((size_0 >> opb_shift) as libc::c_ulong) as bfd_vma
                    as bfd_vma;
                if (*(*output_section_statement).bfd_section).flags
                    & 0x800 as libc::c_int as libc::c_uint == 0
                {
                    (*(*output_section_statement).bfd_section)
                        .size = dot
                        .wrapping_sub((*(*output_section_statement).bfd_section).vma)
                        << opb_shift;
                }
            }
            13 => {
                dot = lang_size_sections_1(
                    &mut (*s).wild_statement.children.head,
                    output_section_statement,
                    fill,
                    dot,
                    relax,
                    check_regions,
                );
            }
            15 => {
                link_info
                    .create_object_symbols_section = (*output_section_statement)
                    .bfd_section;
                (*(*output_section_statement).bfd_section).flags
                    |= 0x200000 as libc::c_int as libc::c_uint;
            }
            5 => {
                let mut i: *mut asection = 0 as *mut asection;
                i = (*s).input_section.section;
                if !relax.is_null() {
                    let mut again: bool = false;
                    if !(Some(
                        ((*(*(*i).owner).xvec)._bfd_relax_section)
                            .expect("non-null function pointer"),
                    ))
                        .expect(
                            "non-null function pointer",
                        )((*i).owner, i, &mut link_info, &mut again)
                    {
                        einfo(
                            dcgettext(
                                0 as *const libc::c_char,
                                b"%F%P: can't relax section: %E\n\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                    }
                    if again {
                        *relax = 1 as libc::c_int != 0;
                    }
                }
                dot = size_input_section(
                    prev,
                    output_section_statement,
                    fill,
                    &mut removed,
                    dot,
                );
            }
            3 => {
                (*s)
                    .fill_statement
                    .output_section = (*output_section_statement).bfd_section;
                fill = (*s).fill_statement.fill;
            }
            1 => {
                let mut newdot_0: bfd_vma = dot;
                let mut tree: *mut etree_type = (*s).assignment_statement.exp;
                expld.dataseg.relro = exp_seg_relro_none;
                exp_fold_tree(
                    tree,
                    (*output_section_statement).bfd_section,
                    &mut newdot_0,
                );
                ldlang_check_relro_region(s, &mut expld.dataseg);
                expld.dataseg.relro = exp_seg_relro_none;
                if ((*tree).type_0.node_class as libc::c_uint
                    == etree_provided as libc::c_int as libc::c_uint
                    || (*tree).type_0.node_class as libc::c_uint
                        == etree_assign as libc::c_int as libc::c_uint)
                    && (*((*tree).assign.dst).offset(0 as libc::c_int as isize)
                        as libc::c_int != '.' as i32
                        || *((*tree).assign.dst).offset(1 as libc::c_int as isize)
                            as libc::c_int != '\0' as i32)
                {
                    (*output_section_statement)
                        .set_update_dot(1 as libc::c_int as libc::c_uint);
                }
                if (*output_section_statement).ignored() == 0 {
                    if output_section_statement == abs_output_section {
                        (*lang_memory_region_lookup(
                            b"*default*\0" as *const u8 as *const libc::c_char,
                            0 as libc::c_int != 0,
                        ))
                            .current = newdot_0;
                    } else if newdot_0 != dot {
                        insert_pad(
                            &mut (*s).header.next,
                            fill,
                            newdot_0.wrapping_sub(dot) << opb_shift,
                            (*output_section_statement).bfd_section,
                            dot,
                        );
                        s = (*s).header.next;
                        if (*output_section_statement).sectype as libc::c_uint
                            != noalloc_section as libc::c_int as libc::c_uint
                            && ((*output_section_statement).sectype as libc::c_uint
                                != noload_section as libc::c_int as libc::c_uint
                                || bfd_get_flavour(link_info.output_bfd) as libc::c_uint
                                    == bfd_target_elf_flavour as libc::c_int as libc::c_uint)
                        {
                            (*(*output_section_statement).bfd_section).flags
                                |= 0x1 as libc::c_int as libc::c_uint;
                        }
                    }
                    dot = newdot_0;
                }
            }
            10 => {
                (*s).padding_statement.size = 0 as libc::c_int as bfd_size_type;
                (*s)
                    .padding_statement
                    .output_offset = dot
                    .wrapping_sub((*(*output_section_statement).bfd_section).vma);
            }
            4 => {
                dot = lang_size_sections_1(
                    &mut (*s).group_statement.children.head,
                    output_section_statement,
                    fill,
                    dot,
                    relax,
                    check_regions,
                );
            }
            9 | 12 | 6 | 7 | 0 => {}
            _ => {
                info_assert(
                    b"ldlang.c\0" as *const u8 as *const libc::c_char,
                    6214 as libc::c_int as libc::c_uint,
                );
            }
        }
        if link_info.non_contiguous_regions() as libc::c_int != 0
            && removed as libc::c_int != 0
        {
            if removed_prev_s {
                prev_s = 0 as *mut lang_statement_union_type;
            }
            if !prev_s.is_null() {
                (*prev_s).header.next = (*s).header.next;
                s = prev_s;
                removed_prev_s = 0 as libc::c_int != 0;
            } else {
                *prev = (*s).header.next;
                removed_prev_s = 1 as libc::c_int != 0;
            }
            if !removed_prev_s {
                prev = &mut (*s).header.next;
            }
        } else {
            prev = &mut (*s).header.next;
            removed_prev_s = 0 as libc::c_int != 0;
        }
        prev_s = s;
        s = (*s).header.next;
    }
    return dot;
}
unsafe extern "C" fn lang_size_segment(mut seg: *mut seg_align_type) -> bool {
    let mut first: bfd_vma = 0;
    let mut last: bfd_vma = 0;
    first = ((*seg).base).wrapping_neg()
        & ((*seg).pagesize).wrapping_sub(1 as libc::c_int as libc::c_ulong);
    last = (*seg).end
        & ((*seg).pagesize).wrapping_sub(1 as libc::c_int as libc::c_ulong);
    if first != 0 && last != 0
        && (*seg).base
            & !((*seg).pagesize).wrapping_sub(1 as libc::c_int as libc::c_ulong)
            != (*seg).end
                & !((*seg).pagesize).wrapping_sub(1 as libc::c_int as libc::c_ulong)
        && first.wrapping_add(last) <= (*seg).pagesize
    {
        (*seg).phase = exp_seg_adjust;
        return 1 as libc::c_int != 0;
    }
    (*seg).phase = exp_seg_done;
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn lang_size_relro_segment_1(mut seg: *mut seg_align_type) -> bfd_vma {
    let mut relro_end: bfd_vma = 0;
    let mut desired_end: bfd_vma = 0;
    let mut sec: *mut asection = 0 as *mut asection;
    relro_end = ((*seg).relro_end)
        .wrapping_add((*seg).pagesize)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        & !((*seg).pagesize).wrapping_sub(1 as libc::c_int as libc::c_ulong);
    desired_end = relro_end.wrapping_sub((*seg).relro_offset);
    sec = (*link_info.output_bfd).section_last;
    while !sec.is_null() {
        if (*sec).flags & 0x1 as libc::c_int as libc::c_uint
            != 0 as libc::c_int as libc::c_uint && (*sec).vma >= (*seg).base
            && (*sec).vma < ((*seg).relro_end).wrapping_sub((*seg).relro_offset)
        {
            let mut start: bfd_vma = 0;
            let mut end: bfd_vma = 0;
            let mut bump: bfd_vma = 0;
            start = (*sec).vma;
            end = start;
            if !((*sec).flags
                & (0x2 as libc::c_int | 0x400 as libc::c_int) as libc::c_uint
                == 0x400 as libc::c_int as libc::c_uint)
            {
                end = (end as libc::c_ulong).wrapping_add((*sec).size >> opb_shift)
                    as bfd_vma as bfd_vma;
            }
            bump = desired_end.wrapping_sub(end);
            start = (start as libc::c_ulong).wrapping_add(bump) as bfd_vma as bfd_vma;
            start
                &= !((1 as libc::c_int as bfd_vma) << (*sec).alignment_power)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong);
            desired_end = start;
        }
        sec = (*sec).prev;
    }
    (*seg).phase = exp_seg_relro_adjust;
    if !(desired_end >= (*seg).base) {
        info_assert(
            b"ldlang.c\0" as *const u8 as *const libc::c_char,
            6374 as libc::c_int as libc::c_uint,
        );
    }
    (*seg).base = desired_end;
    return relro_end;
}
unsafe extern "C" fn lang_size_relro_segment(
    mut relax: *mut bool,
    mut check_regions: bool,
) -> bool {
    let mut do_reset: bool = 0 as libc::c_int != 0;
    let mut do_data_relro: bool = false;
    let mut data_initial_base: bfd_vma = 0;
    let mut data_relro_end: bfd_vma = 0;
    if link_info.relro() as libc::c_int != 0 && expld.dataseg.relro_end != 0 {
        do_data_relro = 1 as libc::c_int != 0;
        data_initial_base = expld.dataseg.base;
        data_relro_end = lang_size_relro_segment_1(&mut expld.dataseg);
    } else {
        do_data_relro = 0 as libc::c_int != 0;
        data_relro_end = 0 as libc::c_int as bfd_vma;
        data_initial_base = data_relro_end;
    }
    if do_data_relro {
        lang_reset_memory_regions();
        one_lang_size_sections_pass(relax, check_regions);
        if do_data_relro as libc::c_int != 0 && expld.dataseg.relro_end > data_relro_end
        {
            expld.dataseg.base = data_initial_base;
            do_reset = 1 as libc::c_int != 0;
        }
    }
    if !do_data_relro && lang_size_segment(&mut expld.dataseg) as libc::c_int != 0 {
        do_reset = 1 as libc::c_int != 0;
    }
    return do_reset;
}
static mut current_section: *mut lang_output_section_statement_type = 0
    as *const lang_output_section_statement_type
    as *mut lang_output_section_statement_type;
static mut current_assign: *mut lang_assignment_statement_type = 0
    as *const lang_assignment_statement_type as *mut lang_assignment_statement_type;
static mut prefer_next_section: bool = false;
unsafe extern "C" fn lang_do_assignments_1(
    mut s: *mut lang_statement_union_type,
    mut current_os: *mut lang_output_section_statement_type,
    mut fill: *mut fill_type,
    mut dot: bfd_vma,
    mut found_end: *mut bool,
) -> bfd_vma {
    while !s.is_null() {
        match (*s).header.type_0 as libc::c_uint {
            14 => {
                dot = lang_do_assignments_1(
                    constructor_list.head,
                    current_os,
                    fill,
                    dot,
                    found_end,
                );
            }
            8 => {
                let mut os: *mut lang_output_section_statement_type = 0
                    as *mut lang_output_section_statement_type;
                let mut newdot: bfd_vma = 0;
                os = &mut (*s).output_section_statement;
                (*os).set_after_end(*found_end as libc::c_uint);
                init_opb((*os).bfd_section);
                if !((*os).bfd_section).is_null() && (*os).ignored() == 0 {
                    if (*(*os).bfd_section).flags & 0x1 as libc::c_int as libc::c_uint
                        != 0 as libc::c_int as libc::c_uint
                    {
                        current_section = os;
                        prefer_next_section = 0 as libc::c_int != 0;
                    }
                    dot = (*(*os).bfd_section).vma;
                }
                newdot = lang_do_assignments_1(
                    (*os).children.head,
                    os,
                    (*os).fill,
                    dot,
                    found_end,
                );
                if (*os).ignored() == 0 {
                    if !((*os).bfd_section).is_null() {
                        if !((*(*os).bfd_section).flags
                            & (0x2 as libc::c_int | 0x400 as libc::c_int) as libc::c_uint
                            == 0x400 as libc::c_int as libc::c_uint)
                            || link_info.type_0() as libc::c_int
                                == type_relocatable as libc::c_int
                        {
                            dot = (dot as libc::c_ulong)
                                .wrapping_add((*(*os).bfd_section).size >> opb_shift)
                                as bfd_vma as bfd_vma;
                        }
                        if !((*os).update_dot_tree).is_null() {
                            exp_fold_tree(
                                (*os).update_dot_tree,
                                &mut *_bfd_std_section
                                    .as_mut_ptr()
                                    .offset(2 as libc::c_int as isize),
                                &mut dot,
                            );
                        }
                    } else {
                        dot = newdot;
                    }
                }
            }
            13 => {
                dot = lang_do_assignments_1(
                    (*s).wild_statement.children.head,
                    current_os,
                    fill,
                    dot,
                    found_end,
                );
            }
            2 => {
                exp_fold_tree(
                    (*s).data_statement.exp,
                    &mut *_bfd_std_section
                        .as_mut_ptr()
                        .offset(2 as libc::c_int as isize),
                    &mut dot,
                );
                if expld.result.valid_p {
                    (*s).data_statement.value = expld.result.value;
                    if !(expld.result.section).is_null() {
                        (*s)
                            .data_statement
                            .value = ((*s).data_statement.value as libc::c_ulong)
                            .wrapping_add((*expld.result.section).vma) as bfd_vma
                            as bfd_vma;
                    }
                } else if expld.phase as libc::c_uint
                    == lang_final_phase_enum as libc::c_int as libc::c_uint
                {
                    einfo(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"%F%P: invalid data statement\n\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                }
                let mut size: libc::c_uint = 0;
                match (*s).data_statement.type_0 {
                    282 | 283 => {
                        size = 8 as libc::c_int as libc::c_uint;
                    }
                    284 => {
                        size = 4 as libc::c_int as libc::c_uint;
                    }
                    285 => {
                        size = 2 as libc::c_int as libc::c_uint;
                    }
                    286 => {
                        size = 1 as libc::c_int as libc::c_uint;
                    }
                    _ => {
                        ld_abort(
                            b"ldlang.c\0" as *const u8 as *const libc::c_char,
                            6534 as libc::c_int,
                            (*::core::mem::transmute::<
                                &[u8; 128],
                                &[libc::c_char; 128],
                            >(
                                b"bfd_vma lang_do_assignments_1(lang_statement_union_type *, lang_output_section_statement_type *, fill_type *, bfd_vma, _Bool *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                }
                if size < (1 as libc::c_int as libc::c_uint) << opb_shift {
                    size = (1 as libc::c_int as libc::c_uint) << opb_shift;
                }
                dot = (dot as libc::c_ulong)
                    .wrapping_add((size >> opb_shift) as libc::c_ulong) as bfd_vma
                    as bfd_vma;
            }
            11 => {
                exp_fold_tree(
                    (*s).reloc_statement.addend_exp,
                    &mut *_bfd_std_section
                        .as_mut_ptr()
                        .offset(2 as libc::c_int as isize),
                    &mut dot,
                );
                if expld.result.valid_p {
                    (*s).reloc_statement.addend_value = expld.result.value;
                } else if expld.phase as libc::c_uint
                    == lang_final_phase_enum as libc::c_int as libc::c_uint
                {
                    einfo(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"%F%P: invalid reloc statement\n\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                }
                dot = (dot as libc::c_ulong)
                    .wrapping_add(
                        (bfd_get_reloc_size((*s).reloc_statement.howto) >> opb_shift)
                            as libc::c_ulong,
                    ) as bfd_vma as bfd_vma;
            }
            5 => {
                let mut in_0: *mut asection = (*s).input_section.section;
                if (*in_0).flags & 0x8000 as libc::c_int as libc::c_uint
                    == 0 as libc::c_int as libc::c_uint
                {
                    dot = (dot as libc::c_ulong).wrapping_add((*in_0).size >> opb_shift)
                        as bfd_vma as bfd_vma;
                }
            }
            3 => {
                fill = (*s).fill_statement.fill;
            }
            1 => {
                current_assign = &mut (*s).assignment_statement;
                if (*(*current_assign).exp).type_0.node_class as libc::c_uint
                    != etree_assert as libc::c_int as libc::c_uint
                {
                    let mut p: *const libc::c_char = (*(*current_assign).exp).assign.dst;
                    if current_os == abs_output_section
                        && *p.offset(0 as libc::c_int as isize) as libc::c_int
                            == '.' as i32
                        && *p.offset(1 as libc::c_int as isize) as libc::c_int
                            == 0 as libc::c_int
                    {
                        prefer_next_section = 1 as libc::c_int != 0;
                    }
                    while *p as libc::c_int == '_' as i32 {
                        p = p.offset(1);
                        p;
                    }
                    if strcmp(p, b"end\0" as *const u8 as *const libc::c_char)
                        == 0 as libc::c_int
                    {
                        *found_end = 1 as libc::c_int != 0;
                    }
                }
                exp_fold_tree(
                    (*s).assignment_statement.exp,
                    if !((*current_os).bfd_section).is_null() {
                        (*current_os).bfd_section
                    } else {
                        &mut *_bfd_std_section
                            .as_mut_ptr()
                            .offset(1 as libc::c_int as isize)
                    },
                    &mut dot,
                );
            }
            10 => {
                dot = (dot as libc::c_ulong)
                    .wrapping_add((*s).padding_statement.size >> opb_shift) as bfd_vma
                    as bfd_vma;
            }
            4 => {
                dot = lang_do_assignments_1(
                    (*s).group_statement.children.head,
                    current_os,
                    fill,
                    dot,
                    found_end,
                );
            }
            15 | 9 | 12 | 6 | 7 | 0 => {}
            _ => {
                info_assert(
                    b"ldlang.c\0" as *const u8 as *const libc::c_char,
                    6617 as libc::c_int as libc::c_uint,
                );
            }
        }
        s = (*s).header.next;
    }
    return dot;
}
static mut start_stop_syms: *mut *mut bfd_link_hash_entry = 0
    as *const *mut bfd_link_hash_entry as *mut *mut bfd_link_hash_entry;
static mut start_stop_count: size_t = 0 as libc::c_int as size_t;
static mut start_stop_alloc: size_t = 0 as libc::c_int as size_t;
unsafe extern "C" fn lang_define_start_stop(
    mut symbol: *const libc::c_char,
    mut sec: *mut asection,
) {
    let mut h: *mut bfd_link_hash_entry = 0 as *mut bfd_link_hash_entry;
    h = (Some(
        ((*(*link_info.output_bfd).xvec)._bfd_define_start_stop)
            .expect("non-null function pointer"),
    ))
        .expect("non-null function pointer")(&mut link_info, symbol, sec);
    if !h.is_null() {
        if start_stop_count == start_stop_alloc {
            start_stop_alloc = (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(start_stop_alloc)
                .wrapping_add(10 as libc::c_int as libc::c_ulong);
            start_stop_syms = xrealloc(
                start_stop_syms as *mut libc::c_void,
                start_stop_alloc
                    .wrapping_mul(
                        ::core::mem::size_of::<*mut bfd_link_hash_entry>()
                            as libc::c_ulong,
                    ),
            ) as *mut *mut bfd_link_hash_entry;
        }
        let fresh14 = start_stop_count;
        start_stop_count = start_stop_count.wrapping_add(1);
        let ref mut fresh15 = *start_stop_syms.offset(fresh14 as isize);
        *fresh15 = h;
    }
}
unsafe extern "C" fn lang_init_start_stop() {
    let mut abfd: *mut bfd = 0 as *mut bfd;
    let mut s: *mut asection = 0 as *mut asection;
    let mut leading_char: libc::c_char = bfd_get_symbol_leading_char(
        link_info.output_bfd,
    );
    abfd = link_info.input_bfds;
    while !abfd.is_null() {
        s = (*abfd).sections;
        while !s.is_null() {
            let mut ps: *const libc::c_char = 0 as *const libc::c_char;
            let mut secname: *const libc::c_char = (*s).name;
            ps = secname;
            while *ps as libc::c_int != '\0' as i32 {
                if _sch_istable[(*ps as libc::c_uchar as libc::c_int
                    & 0xff as libc::c_int) as usize] as libc::c_int
                    & _sch_isalnum as libc::c_int as libc::c_ushort as libc::c_int == 0
                    && *ps as libc::c_int != '_' as i32
                {
                    break;
                }
                ps = ps.offset(1);
                ps;
            }
            if *ps as libc::c_int == '\0' as i32 {
                let mut symbol: *mut libc::c_char = xmalloc(
                    (10 as libc::c_int as libc::c_ulong).wrapping_add(strlen(secname)),
                ) as *mut libc::c_char;
                *symbol.offset(0 as libc::c_int as isize) = leading_char;
                sprintf(
                    symbol
                        .offset(
                            (leading_char as libc::c_int != 0 as libc::c_int)
                                as libc::c_int as isize,
                        ),
                    b"__start_%s\0" as *const u8 as *const libc::c_char,
                    secname,
                );
                lang_define_start_stop(symbol, s);
                *symbol.offset(1 as libc::c_int as isize) = leading_char;
                memcpy(
                    symbol
                        .offset(1 as libc::c_int as isize)
                        .offset(
                            (leading_char as libc::c_int != 0 as libc::c_int)
                                as libc::c_int as isize,
                        ) as *mut libc::c_void,
                    b"__stop\0" as *const u8 as *const libc::c_char
                        as *const libc::c_void,
                    6 as libc::c_int as libc::c_ulong,
                );
                lang_define_start_stop(symbol.offset(1 as libc::c_int as isize), s);
                free(symbol as *mut libc::c_void);
            }
            s = (*s).next;
        }
        abfd = (*abfd).link.next;
    }
}
unsafe extern "C" fn foreach_start_stop(
    mut func: Option::<unsafe extern "C" fn(*mut bfd_link_hash_entry) -> ()>,
) {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < start_stop_count {
        func.expect("non-null function pointer")(*start_stop_syms.offset(i as isize));
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn undef_start_stop(mut h: *mut bfd_link_hash_entry) {
    if (*h).ldscript_def() != 0 {
        return;
    }
    if ((*(*h).u.def.section).output_section).is_null()
        || (*(*(*h).u.def.section).output_section).owner != link_info.output_bfd
        || strcmp(
            (*(*h).u.def.section).name,
            (*(*(*h).u.def.section).output_section).name,
        ) != 0 as libc::c_int
    {
        let mut sec: *mut asection = bfd_get_section_by_name(
            link_info.output_bfd,
            (*(*h).u.def.section).name,
        );
        if !sec.is_null() {
            let mut i: *mut asection = 0 as *mut asection;
            i = (*sec).map_head.s;
            while !i.is_null() {
                if strcmp((*(*h).u.def.section).name, (*i).name) == 0 as libc::c_int {
                    (*h).u.def.section = i;
                    return;
                }
                i = (*i).map_head.s;
            }
        }
        (*h).set_type_0(bfd_link_hash_undefined);
        (*h).u.undef.abfd = 0 as *mut bfd;
        if is_elf_hash_table(link_info.hash) {
            let mut bed: *const elf_backend_data = 0 as *const elf_backend_data;
            let mut eh: *mut elf_link_hash_entry = h as *mut elf_link_hash_entry;
            let mut was_forced: libc::c_uint = (*eh).forced_local();
            bed = (*(*link_info.output_bfd).xvec).backend_data
                as *const elf_backend_data;
            (Some(((*bed).elf_backend_hide_symbol).expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(&mut link_info, eh, 1 as libc::c_int != 0);
            if (*eh).ref_regular_nonweak() == 0 {
                (*h).set_type_0(bfd_link_hash_undefweak);
            }
            (*eh).set_def_regular(0 as libc::c_int as libc::c_uint);
            (*eh).set_forced_local(was_forced);
        }
    }
}
unsafe extern "C" fn lang_undef_start_stop() {
    foreach_start_stop(
        Some(undef_start_stop as unsafe extern "C" fn(*mut bfd_link_hash_entry) -> ()),
    );
}
unsafe extern "C" fn lang_init_startof_sizeof() {
    let mut s: *mut asection = 0 as *mut asection;
    s = (*link_info.output_bfd).sections;
    while !s.is_null() {
        let mut secname: *const libc::c_char = (*s).name;
        let mut symbol: *mut libc::c_char = xmalloc(
            (10 as libc::c_int as libc::c_ulong).wrapping_add(strlen(secname)),
        ) as *mut libc::c_char;
        sprintf(symbol, b".startof.%s\0" as *const u8 as *const libc::c_char, secname);
        lang_define_start_stop(symbol, s);
        memcpy(
            symbol.offset(1 as libc::c_int as isize) as *mut libc::c_void,
            b".size\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            5 as libc::c_int as libc::c_ulong,
        );
        lang_define_start_stop(symbol.offset(1 as libc::c_int as isize), s);
        free(symbol as *mut libc::c_void);
        s = (*s).next;
    }
}
unsafe extern "C" fn set_start_stop(mut h: *mut bfd_link_hash_entry) {
    if (*h).ldscript_def() as libc::c_int != 0
        || (*h).type_0() as libc::c_int != bfd_link_hash_defined as libc::c_int
    {
        return;
    }
    if *((*h).root.string).offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32
    {
        if *((*h).root.string).offset(2 as libc::c_int as isize) as libc::c_int
            == 'i' as i32
        {
            (*h).u.def.value = (*(*h).u.def.section).size >> opb_shift;
            (*h)
                .u
                .def
                .section = &mut *_bfd_std_section
                .as_mut_ptr()
                .offset(2 as libc::c_int as isize) as *mut asection;
        }
    } else {
        let mut has_lead: libc::c_int = (bfd_get_symbol_leading_char(
            link_info.output_bfd,
        ) as libc::c_int != 0 as libc::c_int) as libc::c_int;
        (*h).u.def.section = (*(*h).u.def.section).output_section;
        if *((*h).root.string).offset((4 as libc::c_int + has_lead) as isize)
            as libc::c_int == 'o' as i32
        {
            (*h).u.def.value = (*(*h).u.def.section).size >> opb_shift;
        }
    };
}
unsafe extern "C" fn lang_finalize_start_stop() {
    foreach_start_stop(
        Some(set_start_stop as unsafe extern "C" fn(*mut bfd_link_hash_entry) -> ()),
    );
}
unsafe extern "C" fn lang_end() {
    let mut h: *mut bfd_link_hash_entry = 0 as *mut bfd_link_hash_entry;
    let mut warn: bool = false;
    if link_info.type_0() as libc::c_int == type_relocatable as libc::c_int
        && link_info.gc_sections() == 0
        || link_info.type_0() as libc::c_int == type_dll as libc::c_int
    {
        warn = entry_from_cmdline;
    } else {
        warn = 1 as libc::c_int != 0;
    }
    if link_info.type_0() as libc::c_int == type_relocatable as libc::c_int
        && link_info.gc_sections() as libc::c_int != 0
        && link_info.gc_keep_exported() == 0
    {
        let mut sym: *mut bfd_sym_chain = 0 as *mut bfd_sym_chain;
        sym = link_info.gc_sym_list;
        while !sym.is_null() {
            h = bfd_link_hash_lookup(
                link_info.hash,
                (*sym).name,
                0 as libc::c_int != 0,
                0 as libc::c_int != 0,
                0 as libc::c_int != 0,
            );
            if !h.is_null()
                && ((*h).type_0() as libc::c_int == bfd_link_hash_defined as libc::c_int
                    || (*h).type_0() as libc::c_int
                        == bfd_link_hash_defweak as libc::c_int)
                && !bfd_is_const_section((*h).u.def.section)
            {
                break;
            }
            sym = (*sym).next;
        }
        if sym.is_null() {
            einfo(
                dcgettext(
                    0 as *const libc::c_char,
                    b"%F%P: --gc-sections requires a defined symbol root specified by -e or -u\n\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
    }
    if (entry_symbol.name).is_null() {
        entry_symbol.name = entry_symbol_default;
        warn = 0 as libc::c_int != 0;
    }
    h = bfd_link_hash_lookup(
        link_info.hash,
        entry_symbol.name,
        0 as libc::c_int != 0,
        0 as libc::c_int != 0,
        1 as libc::c_int != 0,
    );
    if !h.is_null()
        && ((*h).type_0() as libc::c_int == bfd_link_hash_defined as libc::c_int
            || (*h).type_0() as libc::c_int == bfd_link_hash_defweak as libc::c_int)
        && !((*(*h).u.def.section).output_section).is_null()
    {
        let mut val: bfd_vma = 0;
        val = ((*h).u.def.value)
            .wrapping_add(bfd_section_vma((*(*h).u.def.section).output_section))
            .wrapping_add((*(*h).u.def.section).output_offset);
        if !bfd_set_start_address(link_info.output_bfd, val) {
            einfo(
                dcgettext(
                    0 as *const libc::c_char,
                    b"%F%P: %s: can't set start address\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                entry_symbol.name,
            );
        }
    } else {
        let mut val_0: bfd_vma = 0;
        let mut send: *const libc::c_char = 0 as *const libc::c_char;
        val_0 = bfd_scan_vma(entry_symbol.name, &mut send, 0 as libc::c_int);
        if *send as libc::c_int == '\0' as i32 {
            if !bfd_set_start_address(link_info.output_bfd, val_0) {
                einfo(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%F%P: can't set start address\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
            }
        } else {
            let mut ts: *mut asection = 0 as *mut asection;
            ts = bfd_get_section_by_name(link_info.output_bfd, entry_section);
            if !ts.is_null() {
                if warn {
                    einfo(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"%P: warning: cannot find entry symbol %s; defaulting to %V\n\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        entry_symbol.name,
                        bfd_section_vma(ts),
                    );
                }
                if !bfd_set_start_address(link_info.output_bfd, bfd_section_vma(ts)) {
                    einfo(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"%F%P: can't set start address\n\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                }
            } else if warn {
                einfo(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%P: warning: cannot find entry symbol %s; not setting start address\n\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    entry_symbol.name,
                );
            }
        }
    };
}
unsafe extern "C" fn ignore_bfd_errors(
    mut fmt: *const libc::c_char,
    mut ap: ::core::ffi::VaList,
) {}
unsafe extern "C" fn lang_check() {
    let mut file: *mut lang_input_statement_type = 0 as *mut lang_input_statement_type;
    let mut input_bfd: *mut bfd = 0 as *mut bfd;
    let mut compatible: *const bfd_arch_info_type = 0 as *const bfd_arch_info_type;
    file = file_chain.head as *mut libc::c_void as *mut lang_input_statement_type;
    while !file.is_null() {
        if !(((*file).flags).claimed() != 0) {
            input_bfd = (*file).the_bfd;
            compatible = bfd_arch_get_compatible(
                input_bfd,
                link_info.output_bfd,
                command_line.accept_unknown_input_arch,
            );
            if ((*file).flags).just_syms() == 0
                && (link_info.type_0() as libc::c_int == type_relocatable as libc::c_int
                    || link_info.emitrelocations() as libc::c_int != 0)
                && (compatible.is_null()
                    || bfd_get_flavour(input_bfd) as libc::c_uint
                        != bfd_get_flavour(link_info.output_bfd) as libc::c_uint)
                && bfd_get_file_flags(input_bfd) & 0x1 as libc::c_int as libc::c_uint
                    != 0 as libc::c_int as libc::c_uint
            {
                einfo(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%F%P: relocatable linking with relocations from format %s (%pB) to format %s (%pB) is not supported\n\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    bfd_get_target(input_bfd),
                    input_bfd,
                    bfd_get_target(link_info.output_bfd),
                    link_info.output_bfd,
                );
            }
            if compatible.is_null() {
                if command_line.warn_mismatch {
                    einfo(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"%X%P: %s architecture of input file `%pB' is incompatible with %s output\n\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        bfd_printable_name(input_bfd),
                        input_bfd,
                        bfd_printable_name(link_info.output_bfd),
                    );
                }
            } else if ((*file).flags).just_syms() == 0
                && ((*input_bfd).flags & 0x40 as libc::c_int as libc::c_uint
                    != 0 as libc::c_int as libc::c_uint
                    || bfd_count_sections(input_bfd) != 0 as libc::c_int as libc::c_uint)
            {
                let mut pfn: bfd_error_handler_type = None;
                if !command_line.warn_mismatch {
                    pfn = bfd_set_error_handler(
                        Some(
                            ignore_bfd_errors
                                as unsafe extern "C" fn(
                                    *const libc::c_char,
                                    ::core::ffi::VaList,
                                ) -> (),
                        ),
                    );
                }
                if !(Some(
                    ((*(*link_info.output_bfd).xvec)._bfd_merge_private_bfd_data)
                        .expect("non-null function pointer"),
                ))
                    .expect("non-null function pointer")(input_bfd, &mut link_info)
                {
                    if command_line.warn_mismatch {
                        einfo(
                            dcgettext(
                                0 as *const libc::c_char,
                                b"%X%P: failed to merge target specific data of file %pB\n\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            input_bfd,
                        );
                    }
                }
                if !command_line.warn_mismatch {
                    bfd_set_error_handler(pfn);
                }
            }
        }
        file = (*file).next;
    }
}
unsafe extern "C" fn lang_common() {
    if link_info.inhibit_common_definition() != 0 {
        return;
    }
    if link_info.type_0() as libc::c_int == type_relocatable as libc::c_int
        && !command_line.force_common_definition
    {
        return;
    }
    if config.sort_common as u64 == 0 {
        bfd_link_hash_traverse(
            link_info.hash,
            Some(
                lang_one_common
                    as unsafe extern "C" fn(
                        *mut bfd_link_hash_entry,
                        *mut libc::c_void,
                    ) -> bool,
            ),
            0 as *mut libc::c_void,
        );
    } else {
        let mut power: libc::c_uint = 0;
        if config.sort_common as libc::c_uint
            == sort_descending as libc::c_int as libc::c_uint
        {
            power = 4 as libc::c_int as libc::c_uint;
            while power > 0 as libc::c_int as libc::c_uint {
                bfd_link_hash_traverse(
                    link_info.hash,
                    Some(
                        lang_one_common
                            as unsafe extern "C" fn(
                                *mut bfd_link_hash_entry,
                                *mut libc::c_void,
                            ) -> bool,
                    ),
                    &mut power as *mut libc::c_uint as *mut libc::c_void,
                );
                power = power.wrapping_sub(1);
                power;
            }
            power = 0 as libc::c_int as libc::c_uint;
            bfd_link_hash_traverse(
                link_info.hash,
                Some(
                    lang_one_common
                        as unsafe extern "C" fn(
                            *mut bfd_link_hash_entry,
                            *mut libc::c_void,
                        ) -> bool,
                ),
                &mut power as *mut libc::c_uint as *mut libc::c_void,
            );
        } else {
            power = 0 as libc::c_int as libc::c_uint;
            while power <= 4 as libc::c_int as libc::c_uint {
                bfd_link_hash_traverse(
                    link_info.hash,
                    Some(
                        lang_one_common
                            as unsafe extern "C" fn(
                                *mut bfd_link_hash_entry,
                                *mut libc::c_void,
                            ) -> bool,
                    ),
                    &mut power as *mut libc::c_uint as *mut libc::c_void,
                );
                power = power.wrapping_add(1);
                power;
            }
            power = -(1 as libc::c_int) as libc::c_uint;
            bfd_link_hash_traverse(
                link_info.hash,
                Some(
                    lang_one_common
                        as unsafe extern "C" fn(
                            *mut bfd_link_hash_entry,
                            *mut libc::c_void,
                        ) -> bool,
                ),
                &mut power as *mut libc::c_uint as *mut libc::c_void,
            );
        }
    };
}
unsafe extern "C" fn ldlang_place_orphan(mut s: *mut asection) {
    if config.orphan_handling as libc::c_uint
        == orphan_handling_discard as libc::c_int as libc::c_uint
    {
        let mut os: *mut lang_output_section_statement_type = 0
            as *mut lang_output_section_statement_type;
        os = lang_output_section_statement_lookup(
            b"/DISCARD/\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
            1 as libc::c_int,
        );
        if ((*os).addr_tree).is_null()
            && (link_info.type_0() as libc::c_int == type_relocatable as libc::c_int
                || (*s).flags & (0x2 as libc::c_int | 0x1 as libc::c_int) as libc::c_uint
                    == 0 as libc::c_int as libc::c_uint)
        {
            (*os).addr_tree = exp_intop(0 as libc::c_int as bfd_vma);
        }
        lang_add_section(
            &mut (*os).children,
            s,
            0 as *mut wildcard_list,
            0 as *mut flag_info,
            os,
        );
    } else {
        let mut os_0: *mut lang_output_section_statement_type = 0
            as *mut lang_output_section_statement_type;
        let mut name: *const libc::c_char = (*s).name;
        let mut constraint: libc::c_int = 0 as libc::c_int;
        if config.orphan_handling as libc::c_uint
            == orphan_handling_error as libc::c_int as libc::c_uint
        {
            einfo(
                dcgettext(
                    0 as *const libc::c_char,
                    b"%X%P: error: unplaced orphan section `%pA' from `%pB'\n\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                s,
                (*s).owner,
            );
        }
        if config.unique_orphan_sections as libc::c_int != 0
            || unique_section_p(s, 0 as *const lang_output_section_statement_type)
                as libc::c_int != 0
        {
            constraint = 382 as libc::c_int;
        }
        os_0 = ldemul_place_orphan(s, name, constraint);
        if os_0.is_null() {
            os_0 = lang_output_section_statement_lookup(
                name,
                constraint,
                1 as libc::c_int,
            );
            if ((*os_0).addr_tree).is_null()
                && (link_info.type_0() as libc::c_int == type_relocatable as libc::c_int
                    || (*s).flags
                        & (0x2 as libc::c_int | 0x1 as libc::c_int) as libc::c_uint
                        == 0 as libc::c_int as libc::c_uint)
            {
                (*os_0).addr_tree = exp_intop(0 as libc::c_int as bfd_vma);
            }
            lang_add_section(
                &mut (*os_0).children,
                s,
                0 as *mut wildcard_list,
                0 as *mut flag_info,
                os_0,
            );
        }
        if config.orphan_handling as libc::c_uint
            == orphan_handling_warn as libc::c_int as libc::c_uint
        {
            einfo(
                dcgettext(
                    0 as *const libc::c_char,
                    b"%P: warning: orphan section `%pA' from `%pB' being placed in section `%s'\n\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                s,
                (*s).owner,
                (*os_0).name,
            );
        }
    };
}
unsafe extern "C" fn lang_place_orphans() {
    let mut file: *mut lang_input_statement_type = 0 as *mut lang_input_statement_type;
    file = file_chain.head as *mut lang_input_statement_type;
    while !file.is_null() {
        let mut s: *mut asection = 0 as *mut asection;
        s = (*(*file).the_bfd).sections;
        while !s.is_null() {
            if ((*s).output_section).is_null() {
                if ((*file).flags).just_syms() != 0 {
                    (Some(
                        ((*(*(*file).the_bfd).xvec)._bfd_link_just_syms)
                            .expect("non-null function pointer"),
                    ))
                        .expect("non-null function pointer")(s, &mut link_info);
                } else if lang_discard_section_p(s) {
                    (*s)
                        .output_section = &mut *_bfd_std_section
                        .as_mut_ptr()
                        .offset(2 as libc::c_int as isize) as *mut asection;
                } else if strcmp(
                    (*s).name,
                    b"COMMON\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    if !(link_info.type_0() as libc::c_int
                        == type_relocatable as libc::c_int)
                        || command_line.force_common_definition as libc::c_int != 0
                    {
                        if default_common_section.is_null() {
                            default_common_section = lang_output_section_statement_lookup(
                                b".bss\0" as *const u8 as *const libc::c_char,
                                0 as libc::c_int,
                                1 as libc::c_int,
                            );
                        }
                        lang_add_section(
                            &mut (*default_common_section).children,
                            s,
                            0 as *mut wildcard_list,
                            0 as *mut flag_info,
                            default_common_section,
                        );
                    }
                } else {
                    ldlang_place_orphan(s);
                }
            }
            s = (*s).next;
        }
        file = (*file).next;
    }
}
unsafe extern "C" fn gc_section_callback(
    mut ptr: *mut lang_wild_statement_type,
    mut sec: *mut wildcard_list,
    mut section: *mut asection,
    mut file: *mut lang_input_statement_type,
    mut data: *mut libc::c_void,
) {
    if (*ptr).keep_sections {
        (*section).flags |= 0x200000 as libc::c_int as libc::c_uint;
    }
}
unsafe extern "C" fn lang_gc_sections_1(mut s: *mut lang_statement_union_type) {
    while !s.is_null() {
        match (*s).header.type_0 as libc::c_uint {
            13 => {
                walk_wild(
                    &mut (*s).wild_statement,
                    Some(
                        gc_section_callback
                            as unsafe extern "C" fn(
                                *mut lang_wild_statement_type,
                                *mut wildcard_list,
                                *mut asection,
                                *mut lang_input_statement_type,
                                *mut libc::c_void,
                            ) -> (),
                    ),
                    0 as *mut libc::c_void,
                );
            }
            14 => {
                lang_gc_sections_1(constructor_list.head);
            }
            8 => {
                lang_gc_sections_1((*s).output_section_statement.children.head);
            }
            4 => {
                lang_gc_sections_1((*s).group_statement.children.head);
            }
            _ => {}
        }
        s = (*s).header.next;
    }
}
unsafe extern "C" fn lang_gc_sections() {
    lang_gc_sections_1(statement_list.head);
    if link_info.type_0() as libc::c_int == type_relocatable as libc::c_int {
        let mut f: *mut lang_input_statement_type = 0 as *mut lang_input_statement_type;
        f = file_chain.head as *mut lang_input_statement_type;
        while !f.is_null() {
            let mut sec: *mut asection = 0 as *mut asection;
            if !(((*f).flags).claimed() != 0) {
                sec = (*(*f).the_bfd).sections;
                while !sec.is_null() {
                    if (*sec).flags & 0x2000 as libc::c_int as libc::c_uint
                        == 0 as libc::c_int as libc::c_uint
                        || strcmp(
                            (*sec).name,
                            b".stabstr\0" as *const u8 as *const libc::c_char,
                        ) != 0 as libc::c_int
                    {
                        (*sec).flags &= !(0x8000 as libc::c_int) as libc::c_uint;
                    }
                    sec = (*sec).next;
                }
            }
            f = (*f).next;
        }
    }
    if link_info.gc_sections() != 0 {
        (Some(
            ((*(*link_info.output_bfd).xvec)._bfd_gc_sections)
                .expect("non-null function pointer"),
        ))
            .expect("non-null function pointer")(link_info.output_bfd, &mut link_info);
    }
}
unsafe extern "C" fn find_relro_section_callback(
    mut ptr: *mut lang_wild_statement_type,
    mut sec: *mut wildcard_list,
    mut section: *mut asection,
    mut file: *mut lang_input_statement_type,
    mut data: *mut libc::c_void,
) {
    if !((*section).output_section).is_null()
        && (*(*section).output_section).owner == link_info.output_bfd
        && (*(*section).output_section).flags & 0x8000 as libc::c_int as libc::c_uint
            == 0 as libc::c_int as libc::c_uint
        && !((*section).flags & 0x1 as libc::c_int as libc::c_uint
            == 0 as libc::c_int as libc::c_uint
            || (*section).flags
                & (0x2 as libc::c_int | 0x400 as libc::c_int) as libc::c_uint
                == 0x400 as libc::c_int as libc::c_uint)
        && (*section).size != 0 as libc::c_int as libc::c_ulong
    {
        let mut has_relro_section: *mut bool = data as *mut bool;
        *has_relro_section = 1 as libc::c_int != 0;
    }
}
unsafe extern "C" fn lang_find_relro_sections_1(
    mut s: *mut lang_statement_union_type,
    mut seg: *mut seg_align_type,
    mut has_relro_section: *mut bool,
) {
    if *has_relro_section {
        return;
    }
    while !s.is_null() {
        if s == (*seg).relro_end_stat {
            break;
        }
        match (*s).header.type_0 as libc::c_uint {
            13 => {
                walk_wild(
                    &mut (*s).wild_statement,
                    Some(
                        find_relro_section_callback
                            as unsafe extern "C" fn(
                                *mut lang_wild_statement_type,
                                *mut wildcard_list,
                                *mut asection,
                                *mut lang_input_statement_type,
                                *mut libc::c_void,
                            ) -> (),
                    ),
                    has_relro_section as *mut libc::c_void,
                );
            }
            14 => {
                lang_find_relro_sections_1(
                    constructor_list.head,
                    seg,
                    has_relro_section,
                );
            }
            8 => {
                lang_find_relro_sections_1(
                    (*s).output_section_statement.children.head,
                    seg,
                    has_relro_section,
                );
            }
            4 => {
                lang_find_relro_sections_1(
                    (*s).group_statement.children.head,
                    seg,
                    has_relro_section,
                );
            }
            _ => {}
        }
        s = (*s).header.next;
    }
}
unsafe extern "C" fn lang_find_relro_sections() {
    let mut has_relro_section: bool = 0 as libc::c_int != 0;
    lang_find_relro_sections_1(
        expld.dataseg.relro_start_stat,
        &mut expld.dataseg,
        &mut has_relro_section,
    );
    if !has_relro_section {
        link_info.set_relro(0 as libc::c_int as libc::c_uint);
    }
}
unsafe extern "C" fn find_replacements_insert_point(
    mut before: *mut bool,
) -> *mut lang_input_statement_type {
    let mut claim1: *mut lang_input_statement_type = 0 as *mut lang_input_statement_type;
    let mut lastobject: *mut lang_input_statement_type = 0
        as *mut lang_input_statement_type;
    lastobject = input_file_chain.head as *mut libc::c_void
        as *mut lang_input_statement_type;
    claim1 = file_chain.head as *mut libc::c_void as *mut lang_input_statement_type;
    while !claim1.is_null() {
        if ((*claim1).flags).claimed() != 0 {
            *before = ((*claim1).flags).claim_archive() != 0;
            return if ((*claim1).flags).claim_archive() as libc::c_int != 0 {
                lastobject
            } else {
                claim1
            };
        }
        if !((*claim1).the_bfd).is_null() && ((*(*claim1).the_bfd).my_archive).is_null()
        {
            lastobject = claim1;
        }
        claim1 = (*claim1).next;
    }
    *before = 0 as libc::c_int != 0;
    return lastobject;
}
unsafe extern "C" fn find_rescan_insertion(
    mut add: *mut lang_input_statement_type,
) -> *mut *mut lang_input_statement_type {
    let mut add_bfd: *mut bfd = (*add).the_bfd;
    let mut f: *mut lang_input_statement_type = 0 as *mut lang_input_statement_type;
    let mut last_loaded: *mut lang_input_statement_type = 0
        as *mut lang_input_statement_type;
    let mut before: *mut lang_input_statement_type = 0 as *mut lang_input_statement_type;
    let mut iter: *mut *mut lang_input_statement_type = 0
        as *mut *mut lang_input_statement_type;
    if !((*add_bfd).my_archive).is_null() {
        add_bfd = (*add_bfd).my_archive;
    }
    f = input_file_chain.head as *mut libc::c_void as *mut lang_input_statement_type;
    while !f.is_null() {
        if (*f).the_bfd == add_bfd {
            before = last_loaded;
            if !((*f).next).is_null() {
                return &mut (*(*f).next).next;
            }
        }
        if !((*f).the_bfd).is_null() && !((*f).next).is_null() {
            last_loaded = f;
        }
        f = (*f).next_real_file;
    }
    iter = if !before.is_null() {
        &mut (*before).next
    } else {
        &mut (*file_chain.head).input_statement.next
    };
    while !(*iter).is_null() {
        if ((**iter).flags).claim_archive() == 0
            && ((*(**iter).the_bfd).my_archive).is_null()
        {
            break;
        }
        iter = &mut (**iter).next;
    }
    return iter;
}
unsafe extern "C" fn lang_list_insert_after(
    mut destlist: *mut lang_statement_list_type,
    mut srclist: *mut lang_statement_list_type,
    mut field: *mut *mut lang_statement_union_type,
) {
    *(*srclist).tail = *field;
    *field = (*srclist).head;
    if (*destlist).tail == field {
        (*destlist).tail = (*srclist).tail;
    }
}
unsafe extern "C" fn lang_list_remove_tail(
    mut destlist: *mut lang_statement_list_type,
    mut origlist: *mut lang_statement_list_type,
) {
    let mut savetail: *mut *mut lang_statement_union = 0
        as *mut *mut lang_statement_union;
    if !((*origlist).head == (*destlist).head) {
        info_assert(
            b"ldlang.c\0" as *const u8 as *const libc::c_char,
            7828 as libc::c_int as libc::c_uint,
        );
    }
    savetail = (*origlist).tail;
    (*origlist).head = *savetail;
    (*origlist).tail = (*destlist).tail;
    (*destlist).tail = savetail;
    *savetail = 0 as *mut lang_statement_union;
}
unsafe extern "C" fn find_next_input_statement(
    mut s: *mut *mut lang_statement_union_type,
) -> *mut *mut lang_statement_union_type {
    let mut current_block_6: u64;
    while !(*s).is_null() {
        let mut t: *mut *mut lang_statement_union_type = 0
            as *mut *mut lang_statement_union_type;
        match (**s).header.type_0 as libc::c_uint {
            6 => return s,
            13 => {
                t = &mut (**s).wild_statement.children.head;
                current_block_6 = 10886091980245723256;
            }
            4 => {
                t = &mut (**s).group_statement.children.head;
                current_block_6 = 10886091980245723256;
            }
            8 => {
                t = &mut (**s).output_section_statement.children.head;
                current_block_6 = 10886091980245723256;
            }
            _ => {
                current_block_6 = 792017965103506125;
            }
        }
        match current_block_6 {
            10886091980245723256 => {
                t = find_next_input_statement(t);
                if !(*t).is_null() {
                    return t;
                }
            }
            _ => {}
        }
        s = &mut (**s).header.next;
    }
    return s;
}
unsafe extern "C" fn lang_check_relocs() {
    if link_info.check_relocs_after_open_input() != 0 {
        let mut abfd: *mut bfd = 0 as *mut bfd;
        abfd = link_info.input_bfds;
        while !abfd.is_null() {
            if !bfd_link_check_relocs(abfd, &mut link_info) {
                config.make_executable = 0 as libc::c_int != 0;
            }
            abfd = (*abfd).link.next;
        }
    }
}
unsafe extern "C" fn lang_propagate_lma_regions() {
    let mut os: *mut lang_output_section_statement_type = 0
        as *mut lang_output_section_statement_type;
    os = lang_os_list.head as *mut libc::c_void
        as *mut lang_output_section_statement_type;
    while !os.is_null() {
        if !((*os).prev).is_null() && ((*os).lma_region).is_null()
            && ((*os).load_base).is_null() && ((*os).addr_tree).is_null()
            && (*os).region == (*(*os).prev).region
        {
            (*os).lma_region = (*(*os).prev).lma_region;
        }
        os = (*os).next;
    }
}
unsafe extern "C" fn lang_get_regions(
    mut region: *mut *mut lang_memory_region_type,
    mut lma_region: *mut *mut lang_memory_region_type,
    mut memspec: *const libc::c_char,
    mut lma_memspec: *const libc::c_char,
    mut have_lma: bool,
    mut have_vma: bool,
) {
    *lma_region = lang_memory_region_lookup(lma_memspec, 0 as libc::c_int != 0);
    if !lma_memspec.is_null() && !have_vma
        && strcmp(memspec, b"*default*\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
    {
        *region = *lma_region;
    } else {
        *region = lang_memory_region_lookup(memspec, 0 as libc::c_int != 0);
    }
    if have_lma as libc::c_int != 0 && !lma_memspec.is_null() {
        einfo(
            dcgettext(
                0 as *const libc::c_char,
                b"%X%P:%pS: section has both a load address and a load region\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            0 as *mut libc::c_void,
        );
    }
}
static mut overlay_vma: *mut etree_type = 0 as *const etree_type as *mut etree_type;
static mut overlay_subalign: *mut etree_type = 0 as *const etree_type as *mut etree_type;
static mut overlay_max: *mut etree_type = 0 as *const etree_type as *mut etree_type;
static mut overlay_list: *mut overlay_list = 0 as *const overlay_list
    as *mut overlay_list;
unsafe extern "C" fn lang_vers_match(
    mut head: *mut bfd_elf_version_expr_head,
    mut prev: *mut bfd_elf_version_expr,
    mut sym: *const libc::c_char,
) -> *mut bfd_elf_version_expr {
    let mut current_block: u64;
    let mut c_sym: *const libc::c_char = 0 as *const libc::c_char;
    let mut cxx_sym: *const libc::c_char = sym;
    let mut java_sym: *const libc::c_char = sym;
    let mut expr: *mut bfd_elf_version_expr = 0 as *mut bfd_elf_version_expr;
    let mut curr_style: demangling_styles = unknown_demangling;
    curr_style = current_demangling_style;
    cplus_demangle_set_style(no_demangling);
    c_sym = bfd_demangle(link_info.output_bfd, sym, 0 as libc::c_int);
    if c_sym.is_null() {
        c_sym = sym;
    }
    cplus_demangle_set_style(curr_style);
    if (*head).mask & 2 as libc::c_int as libc::c_uint != 0 {
        cxx_sym = bfd_demangle(
            link_info.output_bfd,
            sym,
            (1 as libc::c_int) << 0 as libc::c_int
                | (1 as libc::c_int) << 1 as libc::c_int,
        );
        if cxx_sym.is_null() {
            cxx_sym = sym;
        }
    }
    if (*head).mask & 4 as libc::c_int as libc::c_uint != 0 {
        java_sym = bfd_demangle(
            link_info.output_bfd,
            sym,
            (1 as libc::c_int) << 2 as libc::c_int,
        );
        if java_sym.is_null() {
            java_sym = sym;
        }
    }
    if !((*head).htab).is_null()
        && (prev.is_null() || (*prev).literal() as libc::c_int != 0)
    {
        let mut e: bfd_elf_version_expr = bfd_elf_version_expr {
            next: 0 as *mut bfd_elf_version_expr,
            pattern: 0 as *const libc::c_char,
            literal_symver_script_mask: [0; 1],
            c2rust_padding: [0; 7],
        };
        match if !prev.is_null() {
            (*prev).mask() as libc::c_int
        } else {
            0 as libc::c_int
        } {
            0 => {
                current_block = 17537801594644156102;
                match current_block {
                    17537801594644156102 => {
                        if (*head).mask & 1 as libc::c_int as libc::c_uint != 0 {
                            e.pattern = c_sym;
                            expr = htab_find(
                                (*head).htab as htab_t,
                                &mut e as *mut bfd_elf_version_expr as *const libc::c_void,
                            ) as *mut bfd_elf_version_expr;
                            loop {
                                if !(!expr.is_null()
                                    && strcmp((*expr).pattern, c_sym) == 0 as libc::c_int)
                                {
                                    current_block = 16889230658170523591;
                                    break;
                                }
                                if (*expr).mask() as libc::c_int == 1 as libc::c_int {
                                    current_block = 11848385186333083207;
                                    break;
                                }
                                expr = (*expr).next;
                            }
                        } else {
                            current_block = 16889230658170523591;
                        }
                    }
                    _ => {}
                }
                match current_block {
                    11848385186333083207 => {}
                    _ => {
                        match current_block {
                            16889230658170523591 => {
                                if (*head).mask & 2 as libc::c_int as libc::c_uint != 0 {
                                    e.pattern = cxx_sym;
                                    expr = htab_find(
                                        (*head).htab as htab_t,
                                        &mut e as *mut bfd_elf_version_expr as *const libc::c_void,
                                    ) as *mut bfd_elf_version_expr;
                                    loop {
                                        if !(!expr.is_null()
                                            && strcmp((*expr).pattern, cxx_sym) == 0 as libc::c_int)
                                        {
                                            current_block = 858360100726142910;
                                            break;
                                        }
                                        if (*expr).mask() as libc::c_int == 2 as libc::c_int {
                                            current_block = 11848385186333083207;
                                            break;
                                        }
                                        expr = (*expr).next;
                                    }
                                } else {
                                    current_block = 858360100726142910;
                                }
                            }
                            _ => {}
                        }
                        match current_block {
                            11848385186333083207 => {}
                            _ => {
                                if (*head).mask & 4 as libc::c_int as libc::c_uint != 0 {
                                    e.pattern = java_sym;
                                    expr = htab_find(
                                        (*head).htab as htab_t,
                                        &mut e as *mut bfd_elf_version_expr as *const libc::c_void,
                                    ) as *mut bfd_elf_version_expr;
                                    loop {
                                        if !(!expr.is_null()
                                            && strcmp((*expr).pattern, java_sym) == 0 as libc::c_int)
                                        {
                                            current_block = 11385396242402735691;
                                            break;
                                        }
                                        if (*expr).mask() as libc::c_int == 4 as libc::c_int {
                                            current_block = 11848385186333083207;
                                            break;
                                        }
                                        expr = (*expr).next;
                                    }
                                } else {
                                    current_block = 11385396242402735691;
                                }
                            }
                        }
                    }
                }
            }
            1 => {
                current_block = 16889230658170523591;
                match current_block {
                    17537801594644156102 => {
                        if (*head).mask & 1 as libc::c_int as libc::c_uint != 0 {
                            e.pattern = c_sym;
                            expr = htab_find(
                                (*head).htab as htab_t,
                                &mut e as *mut bfd_elf_version_expr as *const libc::c_void,
                            ) as *mut bfd_elf_version_expr;
                            loop {
                                if !(!expr.is_null()
                                    && strcmp((*expr).pattern, c_sym) == 0 as libc::c_int)
                                {
                                    current_block = 16889230658170523591;
                                    break;
                                }
                                if (*expr).mask() as libc::c_int == 1 as libc::c_int {
                                    current_block = 11848385186333083207;
                                    break;
                                }
                                expr = (*expr).next;
                            }
                        } else {
                            current_block = 16889230658170523591;
                        }
                    }
                    _ => {}
                }
                match current_block {
                    11848385186333083207 => {}
                    _ => {
                        match current_block {
                            16889230658170523591 => {
                                if (*head).mask & 2 as libc::c_int as libc::c_uint != 0 {
                                    e.pattern = cxx_sym;
                                    expr = htab_find(
                                        (*head).htab as htab_t,
                                        &mut e as *mut bfd_elf_version_expr as *const libc::c_void,
                                    ) as *mut bfd_elf_version_expr;
                                    loop {
                                        if !(!expr.is_null()
                                            && strcmp((*expr).pattern, cxx_sym) == 0 as libc::c_int)
                                        {
                                            current_block = 858360100726142910;
                                            break;
                                        }
                                        if (*expr).mask() as libc::c_int == 2 as libc::c_int {
                                            current_block = 11848385186333083207;
                                            break;
                                        }
                                        expr = (*expr).next;
                                    }
                                } else {
                                    current_block = 858360100726142910;
                                }
                            }
                            _ => {}
                        }
                        match current_block {
                            11848385186333083207 => {}
                            _ => {
                                if (*head).mask & 4 as libc::c_int as libc::c_uint != 0 {
                                    e.pattern = java_sym;
                                    expr = htab_find(
                                        (*head).htab as htab_t,
                                        &mut e as *mut bfd_elf_version_expr as *const libc::c_void,
                                    ) as *mut bfd_elf_version_expr;
                                    loop {
                                        if !(!expr.is_null()
                                            && strcmp((*expr).pattern, java_sym) == 0 as libc::c_int)
                                        {
                                            current_block = 11385396242402735691;
                                            break;
                                        }
                                        if (*expr).mask() as libc::c_int == 4 as libc::c_int {
                                            current_block = 11848385186333083207;
                                            break;
                                        }
                                        expr = (*expr).next;
                                    }
                                } else {
                                    current_block = 11385396242402735691;
                                }
                            }
                        }
                    }
                }
            }
            2 => {
                current_block = 858360100726142910;
                match current_block {
                    17537801594644156102 => {
                        if (*head).mask & 1 as libc::c_int as libc::c_uint != 0 {
                            e.pattern = c_sym;
                            expr = htab_find(
                                (*head).htab as htab_t,
                                &mut e as *mut bfd_elf_version_expr as *const libc::c_void,
                            ) as *mut bfd_elf_version_expr;
                            loop {
                                if !(!expr.is_null()
                                    && strcmp((*expr).pattern, c_sym) == 0 as libc::c_int)
                                {
                                    current_block = 16889230658170523591;
                                    break;
                                }
                                if (*expr).mask() as libc::c_int == 1 as libc::c_int {
                                    current_block = 11848385186333083207;
                                    break;
                                }
                                expr = (*expr).next;
                            }
                        } else {
                            current_block = 16889230658170523591;
                        }
                    }
                    _ => {}
                }
                match current_block {
                    11848385186333083207 => {}
                    _ => {
                        match current_block {
                            16889230658170523591 => {
                                if (*head).mask & 2 as libc::c_int as libc::c_uint != 0 {
                                    e.pattern = cxx_sym;
                                    expr = htab_find(
                                        (*head).htab as htab_t,
                                        &mut e as *mut bfd_elf_version_expr as *const libc::c_void,
                                    ) as *mut bfd_elf_version_expr;
                                    loop {
                                        if !(!expr.is_null()
                                            && strcmp((*expr).pattern, cxx_sym) == 0 as libc::c_int)
                                        {
                                            current_block = 858360100726142910;
                                            break;
                                        }
                                        if (*expr).mask() as libc::c_int == 2 as libc::c_int {
                                            current_block = 11848385186333083207;
                                            break;
                                        }
                                        expr = (*expr).next;
                                    }
                                } else {
                                    current_block = 858360100726142910;
                                }
                            }
                            _ => {}
                        }
                        match current_block {
                            11848385186333083207 => {}
                            _ => {
                                if (*head).mask & 4 as libc::c_int as libc::c_uint != 0 {
                                    e.pattern = java_sym;
                                    expr = htab_find(
                                        (*head).htab as htab_t,
                                        &mut e as *mut bfd_elf_version_expr as *const libc::c_void,
                                    ) as *mut bfd_elf_version_expr;
                                    loop {
                                        if !(!expr.is_null()
                                            && strcmp((*expr).pattern, java_sym) == 0 as libc::c_int)
                                        {
                                            current_block = 11385396242402735691;
                                            break;
                                        }
                                        if (*expr).mask() as libc::c_int == 4 as libc::c_int {
                                            current_block = 11848385186333083207;
                                            break;
                                        }
                                        expr = (*expr).next;
                                    }
                                } else {
                                    current_block = 11385396242402735691;
                                }
                            }
                        }
                    }
                }
            }
            _ => {
                current_block = 11385396242402735691;
            }
        }
    } else {
        current_block = 11385396242402735691;
    }
    match current_block {
        11385396242402735691 => {
            if prev.is_null() || (*prev).literal() as libc::c_int != 0 {
                expr = (*head).remaining;
            } else {
                expr = (*prev).next;
            }
            while !expr.is_null() {
                let mut s: *const libc::c_char = 0 as *const libc::c_char;
                if !((*expr).pattern).is_null() {
                    if *((*expr).pattern).offset(0 as libc::c_int as isize)
                        as libc::c_int == '*' as i32
                        && *((*expr).pattern).offset(1 as libc::c_int as isize)
                            as libc::c_int == '\0' as i32
                    {
                        break;
                    }
                    if (*expr).mask() as libc::c_int == 4 as libc::c_int {
                        s = java_sym;
                    } else if (*expr).mask() as libc::c_int == 2 as libc::c_int {
                        s = cxx_sym;
                    } else {
                        s = c_sym;
                    }
                    if fnmatch((*expr).pattern, s, 0 as libc::c_int) == 0 as libc::c_int
                    {
                        break;
                    }
                }
                expr = (*expr).next;
            }
        }
        _ => {}
    }
    if c_sym != sym {
        free(c_sym as *mut libc::c_char as *mut libc::c_void);
    }
    if cxx_sym != sym {
        free(cxx_sym as *mut libc::c_char as *mut libc::c_void);
    }
    if java_sym != sym {
        free(java_sym as *mut libc::c_char as *mut libc::c_void);
    }
    return expr;
}
unsafe extern "C" fn realsymbol(
    mut pattern: *const libc::c_char,
) -> *const libc::c_char {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut changed: bool = 0 as libc::c_int != 0;
    let mut backslash: bool = 0 as libc::c_int != 0;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut symbol: *mut libc::c_char = xmalloc(
        (strlen(pattern)).wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    p = pattern;
    s = symbol;
    while *p as libc::c_int != '\0' as i32 {
        if backslash {
            *s.offset(-(1 as libc::c_int as isize)) = *p;
            backslash = 0 as libc::c_int != 0;
            changed = 1 as libc::c_int != 0;
        } else {
            if *p as libc::c_int == '?' as i32 || *p as libc::c_int == '*' as i32
                || *p as libc::c_int == '[' as i32
            {
                free(symbol as *mut libc::c_void);
                return 0 as *const libc::c_char;
            }
            let fresh16 = s;
            s = s.offset(1);
            *fresh16 = *p;
            backslash = *p as libc::c_int == '\\' as i32;
        }
        p = p.offset(1);
        p;
    }
    if changed {
        *s = '\0' as i32 as libc::c_char;
        return symbol;
    } else {
        free(symbol as *mut libc::c_void);
        return pattern;
    };
}
static mut version_index: libc::c_int = 0;
unsafe extern "C" fn version_expr_head_hash(mut p: *const libc::c_void) -> hashval_t {
    let mut e: *const bfd_elf_version_expr = p as *const bfd_elf_version_expr;
    return htab_hash_string((*e).pattern as *const libc::c_void);
}
unsafe extern "C" fn version_expr_head_eq(
    mut p1: *const libc::c_void,
    mut p2: *const libc::c_void,
) -> libc::c_int {
    let mut e1: *const bfd_elf_version_expr = p1 as *const bfd_elf_version_expr;
    let mut e2: *const bfd_elf_version_expr = p2 as *const bfd_elf_version_expr;
    return (strcmp((*e1).pattern, (*e2).pattern) == 0 as libc::c_int) as libc::c_int;
}
unsafe extern "C" fn lang_print_memory_size(mut sz: bfd_vma) {
    if sz & 0x3fffffff as libc::c_int as libc::c_ulong
        == 0 as libc::c_int as libc::c_ulong
    {
        printf(
            b"%10lu GB\0" as *const u8 as *const libc::c_char,
            sz >> 30 as libc::c_int,
        );
    } else if sz & 0xfffff as libc::c_int as libc::c_ulong
        == 0 as libc::c_int as libc::c_ulong
    {
        printf(
            b"%10lu MB\0" as *const u8 as *const libc::c_char,
            sz >> 20 as libc::c_int,
        );
    } else if sz & 0x3ff as libc::c_int as libc::c_ulong
        == 0 as libc::c_int as libc::c_ulong
    {
        printf(
            b"%10lu KB\0" as *const u8 as *const libc::c_char,
            sz >> 10 as libc::c_int,
        );
    } else {
        printf(b" %10lu B\0" as *const u8 as *const libc::c_char, sz);
    };
}
unsafe extern "C" fn run_static_initializers() {
    stat_save_ptr = &mut *stat_save.as_mut_ptr().offset(0 as libc::c_int as isize)
        as *mut *mut lang_statement_list_type;
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
