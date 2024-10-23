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
    pub type symbol;
    pub type list_info_struct;
    pub type cie;
    pub type frch_cfi_data;
    pub type lineno_list;
    pub type z_stream_s;
    fn concat(_: *const libc::c_char, _: ...) -> *mut libc::c_char;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
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
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn free(_: *mut libc::c_void);
    fn bfd_sprintf_vma(_: *mut bfd, _: *mut libc::c_char, _: bfd_vma);
    fn bfd_alloc(abfd: *mut bfd, wanted: bfd_size_type) -> *mut libc::c_void;
    static mut _bfd_std_section: [asection; 4];
    fn bfd_get_section_by_name(
        abfd: *mut bfd,
        name: *const libc::c_char,
    ) -> *mut asection;
    fn bfd_set_section_flags(sec: *mut asection, flags: flagword) -> bool;
    fn bfd_rename_section(sec: *mut asection, newname: *const libc::c_char);
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
    fn bfd_arch_bits_per_address(abfd: *const bfd) -> libc::c_uint;
    fn bfd_install_relocation(
        abfd: *mut bfd,
        reloc_entry: *mut arelent,
        data: *mut libc::c_void,
        data_start: bfd_vma,
        input_section: *mut asection,
        error_message: *mut *mut libc::c_char,
    ) -> bfd_reloc_status_type;
    fn bfd_reloc_type_lookup(
        abfd: *mut bfd,
        code: bfd_reloc_code_real_type,
    ) -> *const reloc_howto_type;
    fn bfd_get_reloc_code_name(code: bfd_reloc_code_real_type) -> *const libc::c_char;
    fn bfd_set_symtab(
        abfd: *mut bfd,
        location: *mut *mut asymbol,
        count: libc::c_uint,
    ) -> bool;
    fn bfd_get_error() -> bfd_error_type;
    fn bfd_errmsg(error_tag: bfd_error_type) -> *const libc::c_char;
    fn bfd_update_compression_header(
        abfd: *mut bfd,
        contents: *mut bfd_byte,
        sec: *mut asection,
    );
    fn bfd_get_compression_header_size(
        abfd: *mut bfd,
        sec: *mut asection,
    ) -> libc::c_int;
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    fn xcalloc(_: size_t, _: size_t) -> *mut libc::c_void;
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
    static mut stdoutput: *mut bfd;
    static mut now_seg: segT;
    static mut reg_section: segT;
    static mut expr_section: segT;
    static mut text_section: segT;
    static mut data_section: segT;
    static mut flag_readonly_data_in_text: libc::c_uchar;
    static mut flag_always_generate_output: libc::c_uchar;
    static mut flag_compress_debug: compressed_debug_section_type;
    static mut linkrelax: libc::c_int;
    static mut do_not_pad_sections_to_alignment: libc::c_int;
    fn as_bad(format: *const libc::c_char, _: ...);
    fn as_fatal(format: *const libc::c_char, _: ...) -> !;
    fn as_bad_where(
        file: *const libc::c_char,
        line: libc::c_uint,
        format: *const libc::c_char,
        _: ...
    );
    fn as_warn_where(
        file: *const libc::c_char,
        line: libc::c_uint,
        format: *const libc::c_char,
        _: ...
    );
    fn as_abort(_: *const libc::c_char, _: libc::c_int, _: *const libc::c_char) -> !;
    fn had_errors() -> libc::c_int;
    fn as_where(_: *mut libc::c_uint) -> *const libc::c_char;
    fn subseg_change(_: segT, _: libc::c_int);
    fn subseg_new(_: *const libc::c_char, _: subsegT) -> segT;
    fn subseg_set(_: segT, _: subsegT);
    fn subseg_text_p(_: segT) -> libc::c_int;
    fn eh_frame_estimate_size_before_relax(_: *mut fragS) -> libc::c_int;
    fn eh_frame_relax_frag(_: *mut fragS) -> libc::c_int;
    fn eh_frame_convert_frag(_: *mut fragS);
    fn S_FORCE_RELOC(_: *mut symbolS, _: libc::c_int) -> libc::c_int;
    fn make_expr_symbol(expressionP: *mut expressionS) -> *mut symbolS;
    fn bfd_elf_obj_attr_size(_: *mut bfd) -> bfd_vma;
    fn bfd_elf_set_obj_attr_contents(_: *mut bfd, _: *mut bfd_byte, _: bfd_vma);
    fn i386_validate_fix(_: *mut fix) -> libc::c_int;
    fn tc_i386_fix_adjustable(_: *mut fix) -> libc::c_int;
    static md_relax_table: [relax_type; 0];
    fn i386_generic_table_relax_frag(
        _: segT,
        _: *mut fragS,
        _: libc::c_long,
    ) -> libc::c_long;
    fn i386_generate_nops(
        _: *mut fragS,
        _: *mut libc::c_char,
        _: offsetT,
        _: libc::c_int,
    );
    fn elf_frob_file();
    fn elf_frob_file_before_adjust();
    fn elf_frob_file_after_relocs();
    fn dwarf2dbg_estimate_size_before_relax(_: *mut fragS) -> libc::c_int;
    fn dwarf2dbg_relax_frag(_: *mut fragS) -> libc::c_int;
    fn dwarf2dbg_convert_frag(_: *mut fragS);
    fn dwarf2dbg_final_check();
    fn elf_adjust_symtab();
    fn elf_frob_symbol(_: *mut symbolS, _: *mut libc::c_int);
    fn frag_alloc(_: *mut obstack) -> *mut fragS;
    static mut flag_use_elf_stt_common: libc::c_int;
    fn tc_gen_reloc(_: *mut asection, _: *mut fixS) -> *mut arelent;
    fn symbol_section_p(_: *mut symbolS) -> libc::c_int;
    fn symbol_mark_written(_: *mut symbolS);
    fn symbol_get_bfdsym(_: *mut symbolS) -> *mut asymbol;
    fn symbol_used_in_reloc_p(_: *mut symbolS) -> libc::c_int;
    fn symbol_next(_: *mut symbolS) -> *mut symbolS;
    static mut symbol_rootP: *mut symbolS;
    fn S_GET_VALUE(_: *mut symbolS) -> valueT;
    fn symbol_mark_resolved(_: *mut symbolS);
    fn S_GET_NAME(_: *mut symbolS) -> *const libc::c_char;
    fn symbol_resolved_p(_: *mut symbolS) -> libc::c_int;
    static mut symbol_lastP: *mut symbolS;
    fn symbol_remove(
        symbolP: *mut symbolS,
        rootP: *mut *mut symbolS,
        lastP: *mut *mut symbolS,
    );
    fn symbol_used_p(_: *mut symbolS) -> libc::c_int;
    fn S_IS_WEAKREFD(_: *mut symbolS) -> libc::c_int;
    fn S_IS_LOCAL(_: *mut symbolS) -> libc::c_int;
    fn S_IS_EXTERNAL(_: *mut symbolS) -> libc::c_int;
    fn S_GET_SEGMENT(_: *mut symbolS) -> segT;
    fn symbol_get_value_expression(_: *mut symbolS) -> *mut expressionS;
    fn S_IS_WEAKREFR(_: *mut symbolS) -> libc::c_int;
    fn S_IS_COMMON(_: *mut symbolS) -> libc::c_int;
    fn symbol_equated_reloc_p(_: *mut symbolS) -> libc::c_int;
    fn resolve_symbol_value(_: *mut symbolS) -> valueT;
    fn decode_local_label_name(s: *mut libc::c_char) -> *mut libc::c_char;
    fn S_IS_DEFINED(_: *mut symbolS) -> libc::c_int;
    fn symbol_mri_common_p(_: *mut symbolS) -> libc::c_int;
    fn symbol_mark_used_in_reloc(_: *mut symbolS);
    fn md_apply_fix(_: *mut fixS, _: *mut valueT, _: segT);
    fn md_pcrel_from(_: *mut fixS) -> libc::c_long;
    fn md_section_align(_: segT, _: valueT) -> valueT;
    fn frag_wane(fragP: *mut fragS);
    fn md_convert_frag(_: *mut bfd, _: segT, _: *mut fragS);
    fn output_leb128(_: *mut libc::c_char, _: valueT, _: libc::c_int) -> libc::c_uint;
    fn sizeof_leb128(_: valueT, _: libc::c_int) -> libc::c_uint;
    static mut zero_address_frag: fragS;
    fn md_estimate_size_before_relax(fragP: *mut fragS, _: segT) -> libc::c_int;
    static mut frag_now: *mut fragS;
    fn frag_align(alignment: libc::c_int, fill_character: libc::c_int, max: libc::c_int);
    fn frag_align_code(alignment: libc::c_int, max: libc::c_int);
    static mut target_big_endian: libc::c_int;
    fn frag_more(nchars: size_t) -> *mut libc::c_char;
    fn frag_now_fix() -> addressT;
    static mut flag_generate_build_notes: bool;
    fn resolve_local_symbol_values();
    fn symbol_get_frag(_: *mut symbolS) -> *mut fragS;
    static mut notes: obstack;
    fn print_symbol_value_1(_: *mut FILE, _: *mut symbolS);
    static mut frchain_now: *mut frchainS;
    fn section_symbol(_: segT) -> *mut symbolS;
    fn _obstack_newchunk(_: *mut obstack, _: size_t);
    fn compress_init() -> *mut z_stream_s;
    fn compress_data(
        _: *mut z_stream_s,
        _: *mut *const libc::c_char,
        _: *mut libc::c_int,
        _: *mut *mut libc::c_char,
        _: *mut libc::c_int,
    ) -> libc::c_int;
    fn compress_finish(
        _: *mut z_stream_s,
        _: *mut *mut libc::c_char,
        _: *mut libc::c_int,
        _: *mut libc::c_int,
    ) -> libc::c_int;
    static mut indent_level: libc::c_int;
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
pub type ptrdiff_t = libc::c_long;
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
pub type relax_addressT = addressT;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct relax_type {
    pub rlx_forward: offsetT,
    pub rlx_backward: offsetT,
    pub rlx_length: libc::c_uchar,
    pub rlx_more: relax_substateT,
}
pub type relax_typeS = relax_type;
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
pub type fixS = fix;
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
    pub d_un: C2RustUnnamed_26,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_26 {
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
    pub tc_data: C2RustUnnamed_27,
    pub version: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_27 {
    pub hppa_arg_reloc: libc::c_uint,
    pub mips_extr: *mut libc::c_void,
    pub any: *mut libc::c_void,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct eh_cie_fde {
    pub u: C2RustUnnamed_28,
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
pub union C2RustUnnamed_28 {
    pub fde: C2RustUnnamed_31,
    pub cie: C2RustUnnamed_29,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2RustUnnamed_29 {
    pub u: C2RustUnnamed_30,
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
pub union C2RustUnnamed_30 {
    pub full_cie: *mut cie,
    pub merged_with: *mut eh_cie_fde,
    pub sec: *mut asection,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_31 {
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
    pub group: C2RustUnnamed_32,
    pub sec_group: *mut asection,
    pub next_in_group: *mut asection,
    pub fde_list: *mut eh_cie_fde,
    pub eh_frame_entry: *mut asection,
    pub has_secondary_relocs: bool,
    pub sec_info: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_32 {
    pub name: *const libc::c_char,
    pub id: *mut bfd_symbol,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct reloc_list {
    pub next: *mut reloc_list,
    pub u: C2RustUnnamed_33,
    pub file: *const libc::c_char,
    pub line: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_33 {
    pub a: C2RustUnnamed_35,
    pub b: C2RustUnnamed_34,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_34 {
    pub sec: *mut asection,
    pub s: *mut asymbol,
    pub r: arelent,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_35 {
    pub offset_sym: *mut symbolS,
    pub howto: *const reloc_howto_type,
    pub sym: *mut symbolS,
    pub addend: bfd_vma,
}
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
    pub stabu: C2RustUnnamed_36,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_36 {
    pub stab_string_size: libc::c_ulong,
    pub p: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct relax_seg_info {
    pub pass: libc::c_int,
    pub changed: libc::c_int,
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
unsafe extern "C" fn bfd_section_alignment(mut sec: *const asection) -> libc::c_uint {
    return (*sec).alignment_power;
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
unsafe extern "C" fn bfd_is_com_section(mut sec: *const asection) -> bool {
    return (*sec).flags & 0x1000 as libc::c_int as libc::c_uint
        != 0 as libc::c_int as libc::c_uint;
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
unsafe extern "C" fn bfd_get_target(mut abfd: *const bfd) -> *const libc::c_char {
    return (*(*abfd).xvec).name;
}
#[inline]
unsafe extern "C" fn bfd_keep_unused_section_symbols(mut abfd: *const bfd) -> bool {
    return (*(*abfd).xvec).keep_unused_section_symbols;
}
#[no_mangle]
pub unsafe extern "C" fn generic_force_reloc(mut fix: *mut fixS) -> libc::c_int {
    if (*fix).fx_r_type as libc::c_uint
        == BFD_RELOC_VTABLE_INHERIT as libc::c_int as libc::c_uint
        || (*fix).fx_r_type as libc::c_uint
            == BFD_RELOC_VTABLE_ENTRY as libc::c_int as libc::c_uint
    {
        return 1 as libc::c_int;
    }
    if ((*fix).fx_addsy).is_null() {
        return 0 as libc::c_int;
    }
    return S_FORCE_RELOC(
        (*fix).fx_addsy,
        ((*fix).fx_subsy == 0 as *mut libc::c_void as *mut symbolS) as libc::c_int,
    );
}
#[no_mangle]
pub static mut finalize_syms: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut abs_section_sym: *mut symbolS = 0 as *const symbolS as *mut symbolS;
#[no_mangle]
pub static mut dot_value: addressT = 0;
#[no_mangle]
pub static mut dot_frag: *mut fragS = 0 as *const fragS as *mut fragS;
#[no_mangle]
pub static mut reloc_list: *mut reloc_list = 0 as *const reloc_list as *mut reloc_list;
#[no_mangle]
pub unsafe extern "C" fn append(
    mut charPP: *mut *mut libc::c_char,
    mut fromP: *mut libc::c_char,
    mut length: libc::c_ulong,
) {
    if length == 0 as libc::c_int as libc::c_ulong {
        return;
    }
    memcpy(*charPP as *mut libc::c_void, fromP as *const libc::c_void, length);
    *charPP = (*charPP).offset(length as isize);
}
#[no_mangle]
pub unsafe extern "C" fn record_alignment(mut seg: segT, mut align: libc::c_uint) {
    if seg
        == &mut *_bfd_std_section.as_mut_ptr().offset(2 as libc::c_int as isize)
            as *mut asection
    {
        return;
    }
    if align > bfd_section_alignment(seg as *const asection) {
        bfd_set_section_alignment(seg, align);
    }
}
#[no_mangle]
pub unsafe extern "C" fn get_recorded_alignment(mut seg: segT) -> libc::c_int {
    if seg
        == &mut *_bfd_std_section.as_mut_ptr().offset(2 as libc::c_int as isize)
            as *mut asection
    {
        return 0 as libc::c_int;
    }
    return bfd_section_alignment(seg as *const asection) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn write_object_file() {
    let mut rsi: relax_seg_info = relax_seg_info {
        pass: 0,
        changed: 0,
    };
    subsegs_finish();
    let mut i: libc::c_int = 0;
    bfd_section_list_remove(stdoutput, reg_section);
    bfd_section_list_remove(stdoutput, expr_section);
    (*stdoutput)
        .section_count = ((*stdoutput).section_count)
        .wrapping_sub(2 as libc::c_int as libc::c_uint);
    i = 0 as libc::c_int;
    bfd_map_over_sections(
        stdoutput,
        Some(
            renumber_sections
                as unsafe extern "C" fn(*mut bfd, *mut asection, *mut libc::c_void) -> (),
        ),
        &mut i as *mut libc::c_int as *mut libc::c_void,
    );
    bfd_map_over_sections(
        stdoutput,
        Some(
            chain_frchains_together
                as unsafe extern "C" fn(*mut bfd, segT, *mut libc::c_void) -> (),
        ),
        0 as *mut libc::c_char as *mut libc::c_void,
    );
    if flag_readonly_data_in_text != 0 {
        merge_data_into_text();
    }
    rsi.pass = 0 as libc::c_int;
    loop {
        rsi.changed = 0 as libc::c_int;
        bfd_map_over_sections(
            stdoutput,
            Some(
                relax_seg
                    as unsafe extern "C" fn(
                        *mut bfd,
                        *mut asection,
                        *mut libc::c_void,
                    ) -> (),
            ),
            &mut rsi as *mut relax_seg_info as *mut libc::c_void,
        );
        rsi.pass += 1;
        rsi.pass;
        if rsi.changed == 0 {
            break;
        }
    }
    finalize_syms = 1 as libc::c_int;
    bfd_map_over_sections(
        stdoutput,
        Some(
            size_seg
                as unsafe extern "C" fn(*mut bfd, *mut asection, *mut libc::c_void) -> (),
        ),
        0 as *mut libc::c_char as *mut libc::c_void,
    );
    finalize_syms = 1 as libc::c_int;
    dwarf2dbg_final_check();
    create_obj_attrs_section();
    if !symbol_rootP.is_null() {
        let mut symp: *mut symbolS = 0 as *mut symbolS;
        symp = symbol_rootP;
        while !symp.is_null() {
            resolve_symbol_value(symp);
            symp = symbol_next(symp);
        }
    }
    resolve_local_symbol_values();
    resolve_reloc_expr_symbols();
    maybe_generate_build_notes();
    elf_frob_file_before_adjust();
    bfd_map_over_sections(
        stdoutput,
        Some(
            adjust_reloc_syms
                as unsafe extern "C" fn(*mut bfd, *mut asection, *mut libc::c_void) -> (),
        ),
        0 as *mut libc::c_char as *mut libc::c_void,
    );
    bfd_map_over_sections(
        stdoutput,
        Some(
            fix_segment
                as unsafe extern "C" fn(*mut bfd, *mut asection, *mut libc::c_void) -> (),
        ),
        0 as *mut libc::c_char as *mut libc::c_void,
    );
    if !symbol_rootP.is_null() {
        let mut symp_0: *mut symbolS = 0 as *mut symbolS;
        let mut skip_next_symbol: bool = 0 as libc::c_int != 0;
        symp_0 = symbol_rootP;
        while !symp_0.is_null() {
            let mut punt: libc::c_int = 0 as libc::c_int;
            let mut name: *const libc::c_char = 0 as *const libc::c_char;
            if skip_next_symbol {
                (*symbol_get_bfdsym(symp_0)).value = S_GET_VALUE(symp_0);
                skip_next_symbol = 0 as libc::c_int != 0;
            } else if symbol_mri_common_p(symp_0) != 0 {
                if S_IS_EXTERNAL(symp_0) != 0 {
                    as_bad(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"%s: global symbols not supported in common sections\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        S_GET_NAME(symp_0),
                    );
                }
                symbol_remove(symp_0, &mut symbol_rootP, &mut symbol_lastP);
            } else {
                name = S_GET_NAME(symp_0);
                if !name.is_null() {
                    let mut name2: *const libc::c_char = decode_local_label_name(
                        S_GET_NAME(symp_0) as *mut libc::c_char,
                    );
                    if name2 != name && S_IS_DEFINED(symp_0) == 0 {
                        as_bad(
                            dcgettext(
                                0 as *const libc::c_char,
                                b"local label `%s' is not defined\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            name2,
                        );
                    }
                }
                resolve_symbol_value(symp_0);
                if symbol_equated_reloc_p(symp_0) != 0 || S_IS_WEAKREFR(symp_0) != 0 {
                    let mut sname: *const libc::c_char = S_GET_NAME(symp_0);
                    if S_IS_COMMON(symp_0) != 0
                        && !(strcmp(
                            sname,
                            b"L0\x01\0" as *const u8 as *const libc::c_char,
                        ) == 0 as libc::c_int) && S_IS_WEAKREFR(symp_0) == 0
                    {
                        let mut e: *mut expressionS = symbol_get_value_expression(
                            symp_0,
                        );
                        as_bad(
                            dcgettext(
                                0 as *const libc::c_char,
                                b"`%s' can't be equated to common symbol `%s'\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            sname,
                            S_GET_NAME((*e).X_add_symbol),
                        );
                    }
                    if S_GET_SEGMENT(symp_0) == reg_section {
                        if S_GET_NAME(symp_0) != (*reg_section).name {
                            as_bad(
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"can't make global register symbol `%s'\0" as *const u8
                                        as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                                sname,
                            );
                        }
                    }
                    symbol_remove(symp_0, &mut symbol_rootP, &mut symbol_lastP);
                } else {
                    elf_frob_symbol(symp_0, &mut punt);
                    if symp_0 == abs_section_sym
                        || 1 as libc::c_int == 0 && symbol_section_p(symp_0) != 0
                        || S_IS_EXTERNAL(symp_0) == 0
                            && (punt != 0 || S_IS_LOCAL(symp_0) != 0
                                || S_IS_WEAKREFD(symp_0) != 0 && symbol_used_p(symp_0) == 0)
                            && symbol_used_in_reloc_p(symp_0) == 0
                    {
                        symbol_remove(symp_0, &mut symbol_rootP, &mut symbol_lastP);
                    } else {
                        if symbol_resolved_p(symp_0) == 0 {
                            as_bad(
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"can't resolve value for symbol `%s'\0" as *const u8
                                        as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                                S_GET_NAME(symp_0),
                            );
                            symbol_mark_resolved(symp_0);
                        }
                        (*symbol_get_bfdsym(symp_0)).value = S_GET_VALUE(symp_0);
                        if (*symbol_get_bfdsym(symp_0)).flags
                            & ((1 as libc::c_int) << 12 as libc::c_int) as libc::c_uint
                            != 0
                        {
                            skip_next_symbol = 1 as libc::c_int != 0;
                        }
                    }
                }
            }
            symp_0 = symbol_next(symp_0);
        }
    }
    elf_adjust_symtab();
    if flag_always_generate_output == 0 && had_errors() != 0 {
        return;
    }
    set_symtab();
    elf_frob_file();
    bfd_map_over_sections(
        stdoutput,
        Some(
            write_relocs
                as unsafe extern "C" fn(*mut bfd, *mut asection, *mut libc::c_void) -> (),
        ),
        0 as *mut libc::c_char as *mut libc::c_void,
    );
    elf_frob_file_after_relocs();
    if 1 as libc::c_int != 0 && flag_use_elf_stt_common != 0 {
        (*stdoutput).flags
            |= (0x40000 as libc::c_int | 0x80000 as libc::c_int) as libc::c_uint;
    }
    if flag_compress_debug as u64 != 0 {
        if flag_compress_debug as libc::c_uint
            == COMPRESS_DEBUG_GABI_ZLIB as libc::c_int as libc::c_uint
        {
            (*stdoutput).flags
                |= (0x4000 as libc::c_int | 0x20000 as libc::c_int) as libc::c_uint;
        } else {
            (*stdoutput).flags |= 0x4000 as libc::c_int as libc::c_uint;
        }
        bfd_map_over_sections(
            stdoutput,
            Some(
                compress_debug
                    as unsafe extern "C" fn(
                        *mut bfd,
                        *mut asection,
                        *mut libc::c_void,
                    ) -> (),
            ),
            0 as *mut libc::c_char as *mut libc::c_void,
        );
    }
    bfd_map_over_sections(
        stdoutput,
        Some(
            write_contents
                as unsafe extern "C" fn(*mut bfd, *mut asection, *mut libc::c_void) -> (),
        ),
        0 as *mut libc::c_char as *mut libc::c_void,
    );
}
#[no_mangle]
pub static mut symbol_table_frozen: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn relax_segment(
    mut segment_frag_root: *mut frag,
    mut segment: segT,
    mut pass: libc::c_int,
) -> libc::c_int {
    let mut frag_count: libc::c_ulong = 0;
    let mut fragP: *mut frag = 0 as *mut frag;
    let mut address: relax_addressT = 0;
    let mut region: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    subseg_change(segment, 0 as libc::c_int);
    address = 0 as libc::c_int as relax_addressT;
    region = 0 as libc::c_int;
    frag_count = 0 as libc::c_int as libc::c_ulong;
    fragP = segment_frag_root;
    while !fragP.is_null() {
        (*fragP).set_region(region as libc::c_uint);
        (*fragP).set_relax_marker(0 as libc::c_int as libc::c_uint);
        (*fragP).fr_address = address;
        address = (address as libc::c_ulong).wrapping_add((*fragP).fr_fix)
            as relax_addressT as relax_addressT;
        match (*fragP).fr_type as libc::c_uint {
            1 => {
                address = (address as libc::c_ulong)
                    .wrapping_add(
                        ((*fragP).fr_offset * (*fragP).fr_var) as libc::c_ulong,
                    ) as relax_addressT as relax_addressT;
            }
            2 | 3 | 4 => {
                let mut offset: addressT = relax_align(
                    address,
                    (*fragP).fr_offset as libc::c_int,
                );
                if (*fragP).fr_subtype != 0 as libc::c_int as libc::c_uint
                    && offset > (*fragP).fr_subtype as libc::c_ulong
                {
                    offset = 0 as libc::c_int as addressT;
                }
                if offset.wrapping_rem((*fragP).fr_var as libc::c_ulong)
                    != 0 as libc::c_int as libc::c_ulong
                {
                    as_bad_where(
                        (*fragP).fr_file,
                        (*fragP).fr_line,
                        dcngettext(
                            0 as *const libc::c_char,
                            b"alignment padding (%lu byte) not a multiple of %ld\0"
                                as *const u8 as *const libc::c_char,
                            b"alignment padding (%lu bytes) not a multiple of %ld\0"
                                as *const u8 as *const libc::c_char,
                            offset,
                            5 as libc::c_int,
                        ),
                        offset,
                        (*fragP).fr_var,
                    );
                    offset = (offset as libc::c_ulong)
                        .wrapping_sub(
                            offset.wrapping_rem((*fragP).fr_var as libc::c_ulong),
                        ) as addressT as addressT;
                }
                address = (address as libc::c_ulong).wrapping_add(offset)
                    as relax_addressT as relax_addressT;
                region += 1 as libc::c_int;
            }
            5 => {
                region += 1 as libc::c_int;
            }
            8 | 9 => {}
            7 => {
                if !((*fragP).fr_symbol).is_null() {
                    resolve_symbol_value((*fragP).fr_symbol);
                }
                address = (address as libc::c_ulong)
                    .wrapping_add(
                        md_estimate_size_before_relax(fragP, segment) as libc::c_ulong,
                    ) as relax_addressT as relax_addressT;
            }
            11 => {
                (*fragP).fr_offset = 1 as libc::c_int as offsetT;
                address = (address as libc::c_ulong)
                    .wrapping_add((*fragP).fr_offset as libc::c_ulong) as relax_addressT
                    as relax_addressT;
            }
            12 => {
                address = (address as libc::c_ulong)
                    .wrapping_add(
                        eh_frame_estimate_size_before_relax(fragP) as libc::c_ulong,
                    ) as relax_addressT as relax_addressT;
            }
            13 => {
                address = (address as libc::c_ulong)
                    .wrapping_add(
                        dwarf2dbg_estimate_size_before_relax(fragP) as libc::c_ulong,
                    ) as relax_addressT as relax_addressT;
            }
            _ => {
                as_fatal(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Case value %ld unexpected at line %d of file \"%s\"\n\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    (*fragP).fr_type as libc::c_long,
                    2761 as libc::c_int,
                    b"write.c\0" as *const u8 as *const libc::c_char,
                );
            }
        }
        fragP = (*fragP).fr_next;
        frag_count = frag_count.wrapping_add(1);
        frag_count;
    }
    let mut max_iterations: libc::c_ulong = 0;
    let mut stretch: offsetT = 0;
    let mut stretched: libc::c_int = 0;
    let mut rs_leb128_fudge: libc::c_int = 0 as libc::c_int;
    max_iterations = frag_count.wrapping_mul(frag_count);
    if max_iterations < frag_count {
        max_iterations = frag_count;
    }
    ret = 0 as libc::c_int;
    loop {
        stretch = 0 as libc::c_int as offsetT;
        stretched = 0 as libc::c_int;
        fragP = segment_frag_root;
        while !fragP.is_null() {
            let mut growth: offsetT = 0 as libc::c_int as offsetT;
            let mut was_address: addressT = 0;
            let mut offset_0: offsetT = 0;
            let mut symbolP: *mut symbolS = 0 as *mut symbolS;
            (*fragP)
                .set_relax_marker(
                    (*fragP).relax_marker() ^ 1 as libc::c_int as libc::c_uint,
                );
            was_address = (*fragP).fr_address;
            (*fragP)
                .fr_address = ((*fragP).fr_address as libc::c_ulong)
                .wrapping_add(stretch as libc::c_ulong) as addressT as addressT;
            address = (*fragP).fr_address;
            symbolP = (*fragP).fr_symbol;
            offset_0 = (*fragP).fr_offset;
            match (*fragP).fr_type as libc::c_uint {
                1 => {
                    growth = 0 as libc::c_int as offsetT;
                }
                2 | 3 | 4 => {
                    let mut oldoff: addressT = 0;
                    let mut newoff: addressT = 0;
                    oldoff = relax_align(
                        was_address.wrapping_add((*fragP).fr_fix),
                        offset_0 as libc::c_int,
                    );
                    newoff = relax_align(
                        address.wrapping_add((*fragP).fr_fix),
                        offset_0 as libc::c_int,
                    );
                    if (*fragP).fr_subtype != 0 as libc::c_int as libc::c_uint {
                        if oldoff > (*fragP).fr_subtype as libc::c_ulong {
                            oldoff = 0 as libc::c_int as addressT;
                        }
                        if newoff > (*fragP).fr_subtype as libc::c_ulong {
                            newoff = 0 as libc::c_int as addressT;
                        }
                    }
                    growth = newoff.wrapping_sub(oldoff) as offsetT;
                    if growth < 0 as libc::c_int as libc::c_long
                        && rs_leb128_fudge & 16 as libc::c_int != 0 as libc::c_int
                        && rs_leb128_fudge & 15 as libc::c_int >= 2 as libc::c_int
                    {
                        let mut seginfo: *mut segment_info_type = bfd_section_userdata(
                            segment as *const asection,
                        ) as *mut segment_info_type;
                        let mut ob: *mut obstack = &mut (*(*seginfo).frchainP)
                            .frch_obstack;
                        let mut newf: *mut frag = 0 as *mut frag;
                        newf = frag_alloc(ob);
                        (*ob)
                            .next_free = ((*ob).next_free)
                            .offset((*fragP).fr_var as isize);
                        let mut __o1: *mut obstack = ob;
                        let mut __value: *mut libc::c_void = (*__o1).object_base
                            as *mut libc::c_void;
                        if (*__o1).next_free == __value as *mut libc::c_char {
                            (*__o1)
                                .set_maybe_empty_object(1 as libc::c_int as libc::c_uint);
                        }
                        (*__o1)
                            .next_free = (if (::core::mem::size_of::<ptrdiff_t>()
                            as libc::c_ulong)
                            < ::core::mem::size_of::<*mut libc::c_void>()
                                as libc::c_ulong
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
                        if ((*__o1).next_free)
                            .offset_from((*__o1).chunk as *mut libc::c_char)
                            as libc::c_long as size_t
                            > ((*__o1).chunk_limit)
                                .offset_from((*__o1).chunk as *mut libc::c_char)
                                as libc::c_long as size_t
                        {
                            (*__o1).next_free = (*__o1).chunk_limit;
                        }
                        (*__o1).object_base = (*__o1).next_free;
                        memcpy(
                            newf as *mut libc::c_void,
                            fragP as *const libc::c_void,
                            (zero_address_frag.fr_literal)
                                .as_mut_ptr()
                                .offset_from(
                                    &mut zero_address_frag as *mut fragS as *mut libc::c_char,
                                ) as libc::c_long as libc::c_ulong,
                        );
                        memcpy(
                            ((*newf).fr_literal).as_mut_ptr() as *mut libc::c_void,
                            ((*fragP).fr_literal)
                                .as_mut_ptr()
                                .offset((*fragP).fr_fix as isize) as *const libc::c_void,
                            (*fragP).fr_var as libc::c_ulong,
                        );
                        (*newf).fr_type = rs_fill;
                        (*newf)
                            .fr_address = address
                            .wrapping_add((*fragP).fr_fix)
                            .wrapping_add(newoff);
                        (*newf).fr_fix = 0 as libc::c_int as valueT;
                        (*newf)
                            .fr_offset = ((1 as libc::c_int as offsetT)
                            << (*fragP).fr_offset) / (*fragP).fr_var;
                        if (*newf).fr_offset * (*newf).fr_var
                            != (1 as libc::c_int as offsetT) << (*fragP).fr_offset
                        {
                            (*newf)
                                .fr_offset = (1 as libc::c_int as offsetT)
                                << (*fragP).fr_offset;
                            (*newf).fr_var = 1 as libc::c_int as offsetT;
                        }
                        growth += (*newf).fr_offset * (*newf).fr_var;
                        (*newf)
                            .fr_address = ((*newf).fr_address as libc::c_ulong)
                            .wrapping_sub((stretch + growth) as libc::c_ulong)
                            as addressT as addressT;
                        (*newf)
                            .set_relax_marker(
                                (*newf).relax_marker() ^ 1 as libc::c_int as libc::c_uint,
                            );
                        (*fragP).fr_next = newf;
                    }
                }
                5 => {
                    let mut target: offsetT = offset_0;
                    let mut after: addressT = 0;
                    if !symbolP.is_null() {
                        target = (target as libc::c_ulong)
                            .wrapping_add(
                                (S_GET_VALUE(symbolP))
                                    .wrapping_mul(
                                        ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_ulong,
                                    ),
                            ) as offsetT as offsetT;
                    }
                    after = ((*(*fragP).fr_next).fr_address)
                        .wrapping_add(stretch as libc::c_ulong);
                    growth = (target as libc::c_ulong).wrapping_sub(after) as offsetT;
                    if address.wrapping_add((*fragP).fr_fix) as offsetT > target {
                        growth = 0 as libc::c_int as offsetT;
                        if pass < 2 as libc::c_int {
                            ret = 1 as libc::c_int;
                        } else {
                            as_bad_where(
                                (*fragP).fr_file,
                                (*fragP).fr_line,
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"attempt to move .org backwards\0" as *const u8
                                        as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                            );
                            (*fragP).fr_type = rs_align;
                            (*fragP).fr_subtype = 0 as libc::c_int as relax_substateT;
                            (*fragP).fr_offset = 0 as libc::c_int as offsetT;
                            (*fragP).fr_fix = after.wrapping_sub(address);
                        }
                    }
                }
                8 | 9 => {
                    growth = 0 as libc::c_int as offsetT;
                    if !symbolP.is_null() {
                        let mut amount: offsetT = 0;
                        amount = S_GET_VALUE(symbolP) as offsetT;
                        if S_GET_SEGMENT(symbolP)
                            != &mut *_bfd_std_section
                                .as_mut_ptr()
                                .offset(2 as libc::c_int as isize) as *mut asection
                            || S_IS_COMMON(symbolP) != 0 || S_IS_DEFINED(symbolP) == 0
                        {
                            as_bad_where(
                                (*fragP).fr_file,
                                (*fragP).fr_line,
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b".space, .nops or .fill specifies non-absolute value\0"
                                        as *const u8 as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                            );
                            (*fragP).fr_symbol = 0 as *mut symbolS;
                        } else if amount < 0 as libc::c_int as libc::c_long {
                            if pass < 2 as libc::c_int {
                                ret = 1 as libc::c_int;
                            } else {
                                as_warn_where(
                                    (*fragP).fr_file,
                                    (*fragP).fr_line,
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b".space, .nops or .fill with negative value, ignored\0"
                                            as *const u8 as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                                (*fragP).fr_symbol = 0 as *mut symbolS;
                            }
                        } else {
                            growth = was_address
                                .wrapping_add((*fragP).fr_fix)
                                .wrapping_add(amount as libc::c_ulong)
                                .wrapping_sub((*(*fragP).fr_next).fr_address) as offsetT;
                        }
                    }
                }
                7 => {
                    growth = i386_generic_table_relax_frag(segment, fragP, stretch);
                }
                11 => {
                    let mut value: valueT = 0;
                    let mut size: offsetT = 0;
                    value = resolve_symbol_value((*fragP).fr_symbol);
                    size = sizeof_leb128(value, (*fragP).fr_subtype as libc::c_int)
                        as offsetT;
                    growth = size - (*fragP).fr_offset;
                    (*fragP).fr_offset = size;
                }
                12 => {
                    growth = eh_frame_relax_frag(fragP) as offsetT;
                }
                13 => {
                    growth = dwarf2dbg_relax_frag(fragP) as offsetT;
                }
                _ => {
                    as_fatal(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Case value %ld unexpected at line %d of file \"%s\"\n\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        (*fragP).fr_type as libc::c_long,
                        3104 as libc::c_int,
                        b"write.c\0" as *const u8 as *const libc::c_char,
                    );
                }
            }
            if growth != 0 {
                stretch += growth;
                stretched = 1 as libc::c_int;
                if (*fragP).fr_type as libc::c_uint
                    == rs_leb128 as libc::c_int as libc::c_uint
                {
                    rs_leb128_fudge += 16 as libc::c_int;
                } else if (*fragP).fr_type as libc::c_uint
                    == rs_align as libc::c_int as libc::c_uint
                    && rs_leb128_fudge & 16 as libc::c_int != 0 as libc::c_int
                    && stretch == 0 as libc::c_int as libc::c_long
                {
                    rs_leb128_fudge += 16 as libc::c_int;
                } else {
                    rs_leb128_fudge = 0 as libc::c_int;
                }
            }
            fragP = (*fragP).fr_next;
        }
        if stretch == 0 as libc::c_int as libc::c_long
            && rs_leb128_fudge & 16 as libc::c_int == 0 as libc::c_int
            && rs_leb128_fudge & -(16 as libc::c_int) != 0 as libc::c_int
        {
            rs_leb128_fudge += 1 as libc::c_int;
        } else {
            rs_leb128_fudge = 0 as libc::c_int;
        }
        if !(stretched != 0
            && {
                max_iterations = max_iterations.wrapping_sub(1);
                max_iterations != 0
            })
        {
            break;
        }
    }
    if stretched != 0 {
        as_fatal(
            dcgettext(
                0 as *const libc::c_char,
                b"Infinite loop encountered whilst attempting to compute the addresses of symbols in section %s\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            bfd_section_name(segment as *const asection),
        );
    }
    fragP = segment_frag_root;
    while !fragP.is_null() {
        if (*fragP).last_fr_address != (*fragP).fr_address {
            (*fragP).last_fr_address = (*fragP).fr_address;
            ret = 1 as libc::c_int;
        }
        fragP = (*fragP).fr_next;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn relax_frag(
    mut segment: segT,
    mut fragP: *mut fragS,
    mut stretch: libc::c_long,
) -> libc::c_long {
    let mut this_type: *const relax_typeS = 0 as *const relax_typeS;
    let mut start_type: *const relax_typeS = 0 as *const relax_typeS;
    let mut next_state: relax_substateT = 0;
    let mut this_state: relax_substateT = 0;
    let mut growth: offsetT = 0;
    let mut aim: offsetT = 0;
    let mut target: addressT = 0;
    let mut address: addressT = 0;
    let mut symbolP: *mut symbolS = 0 as *mut symbolS;
    let mut table: *const relax_typeS = 0 as *const relax_typeS;
    target = (*fragP).fr_offset as addressT;
    address = ((*fragP).fr_address).wrapping_add((*fragP).fr_fix);
    table = md_relax_table.as_ptr();
    this_state = (*fragP).fr_subtype;
    this_type = table.offset(this_state as isize);
    start_type = this_type;
    symbolP = (*fragP).fr_symbol;
    if !symbolP.is_null() {
        let mut sym_frag: *mut fragS = 0 as *mut fragS;
        sym_frag = symbol_get_frag(symbolP);
        target = (target as libc::c_ulong).wrapping_add(S_GET_VALUE(symbolP)) as addressT
            as addressT;
        if stretch != 0 as libc::c_int as libc::c_long
            && (*sym_frag).relax_marker() as libc::c_int
                != (*fragP).relax_marker() as libc::c_int
            && S_GET_SEGMENT(symbolP) == segment
        {
            if stretch < 0 as libc::c_int as libc::c_long
                || (*sym_frag).region() as libc::c_int
                    == (*fragP).region() as libc::c_int
            {
                target = (target as libc::c_ulong).wrapping_add(stretch as libc::c_ulong)
                    as addressT as addressT;
            } else if target < address {
                return 0 as libc::c_int as libc::c_long
            }
        }
    }
    aim = target.wrapping_sub(address) as offsetT;
    if aim < 0 as libc::c_int as libc::c_long {
        next_state = (*this_type).rlx_more;
        while next_state != 0 {
            if aim >= (*this_type).rlx_backward {
                next_state = 0 as libc::c_int as relax_substateT;
            } else {
                this_state = next_state;
                this_type = table.offset(this_state as isize);
                next_state = (*this_type).rlx_more;
            }
        }
    } else {
        next_state = (*this_type).rlx_more;
        while next_state != 0 {
            if aim <= (*this_type).rlx_forward {
                next_state = 0 as libc::c_int as relax_substateT;
            } else {
                this_state = next_state;
                this_type = table.offset(this_state as isize);
                next_state = (*this_type).rlx_more;
            }
        }
    }
    growth = ((*this_type).rlx_length as libc::c_int
        - (*start_type).rlx_length as libc::c_int) as offsetT;
    if growth != 0 as libc::c_int as libc::c_long {
        (*fragP).fr_subtype = this_state;
    }
    return growth;
}
#[no_mangle]
pub unsafe extern "C" fn number_to_chars_littleendian(
    mut buf: *mut libc::c_char,
    mut val: valueT,
    mut n: libc::c_int,
) {
    if n <= 0 as libc::c_int {
        as_abort(
            b"write.c\0" as *const u8 as *const libc::c_char,
            3162 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 55],
                &[libc::c_char; 55],
            >(b"void number_to_chars_littleendian(char *, valueT, int)\0"))
                .as_ptr(),
        );
    }
    loop {
        let fresh0 = n;
        n = n - 1;
        if !(fresh0 != 0) {
            break;
        }
        let fresh1 = buf;
        buf = buf.offset(1);
        *fresh1 = (val & 0xff as libc::c_int as libc::c_ulong) as libc::c_char;
        val >>= 8 as libc::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn number_to_chars_bigendian(
    mut buf: *mut libc::c_char,
    mut val: valueT,
    mut n: libc::c_int,
) {
    if n <= 0 as libc::c_int {
        as_abort(
            b"write.c\0" as *const u8 as *const libc::c_char,
            3150 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 52],
                &[libc::c_char; 52],
            >(b"void number_to_chars_bigendian(char *, valueT, int)\0"))
                .as_ptr(),
        );
    }
    loop {
        let fresh2 = n;
        n = n - 1;
        if !(fresh2 != 0) {
            break;
        }
        *buf
            .offset(
                n as isize,
            ) = (val & 0xff as libc::c_int as libc::c_ulong) as libc::c_char;
        val >>= 8 as libc::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn fix_new(
    mut frag: *mut fragS,
    mut where_0: libc::c_ulong,
    mut size: libc::c_ulong,
    mut add_symbol: *mut symbolS,
    mut offset: offsetT,
    mut pcrel: libc::c_int,
    mut r_type: bfd_reloc_code_real,
) -> *mut fixS {
    return fix_new_internal(
        frag,
        where_0,
        size,
        add_symbol,
        0 as *mut libc::c_void as *mut symbolS,
        offset,
        pcrel,
        r_type,
        0 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn fix_at_start(
    mut frag: *mut fragS,
    mut size: libc::c_ulong,
    mut add_symbol: *mut symbolS,
    mut offset: offsetT,
    mut pcrel: libc::c_int,
    mut r_type: bfd_reloc_code_real,
) -> *mut fixS {
    return fix_new_internal(
        frag,
        0 as libc::c_int as libc::c_ulong,
        size,
        add_symbol,
        0 as *mut libc::c_void as *mut symbolS,
        offset,
        pcrel,
        r_type,
        1 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn fix_new_exp(
    mut frag: *mut fragS,
    mut where_0: libc::c_ulong,
    mut size: libc::c_ulong,
    mut exp: *mut expressionS,
    mut pcrel: libc::c_int,
    mut r_type: bfd_reloc_code_real,
) -> *mut fixS {
    let mut add: *mut symbolS = 0 as *mut symbolS;
    let mut sub: *mut symbolS = 0 as *mut symbolS;
    let mut off: offsetT = 0 as libc::c_int as offsetT;
    let mut current_block_15: u64;
    match (*exp).X_op() as libc::c_int {
        1 => {
            current_block_15 = 15652330335145281839;
        }
        5 => {
            as_bad(
                dcgettext(
                    0 as *const libc::c_char,
                    b"register value used as expression\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            current_block_15 = 15652330335145281839;
        }
        19 => {
            let mut stmp: *mut symbolS = make_expr_symbol(exp);
            (*exp).set_X_op(O_symbol);
            (*exp).X_op_symbol = 0 as *mut symbolS;
            (*exp).X_add_symbol = stmp;
            (*exp).X_add_number = 0 as libc::c_int as offsetT;
            return fix_new_exp(frag, where_0, size, exp, pcrel, r_type);
        }
        4 => {
            add = (*exp).X_add_symbol;
            off = (*exp).X_add_number;
            r_type = BFD_RELOC_RVA;
            current_block_15 = 15652330335145281839;
        }
        7 => {
            sub = (*exp).X_add_symbol;
            off = (*exp).X_add_number;
            current_block_15 = 15652330335145281839;
        }
        20 => {
            sub = (*exp).X_op_symbol;
            current_block_15 = 15348904380173152460;
        }
        3 => {
            current_block_15 = 15348904380173152460;
        }
        2 => {
            current_block_15 = 5358154283696459044;
        }
        _ => {
            add = make_expr_symbol(exp);
            current_block_15 = 15652330335145281839;
        }
    }
    match current_block_15 {
        15348904380173152460 => {
            add = (*exp).X_add_symbol;
            current_block_15 = 5358154283696459044;
        }
        _ => {}
    }
    match current_block_15 {
        5358154283696459044 => {
            off = (*exp).X_add_number;
        }
        _ => {}
    }
    return fix_new_internal(
        frag,
        where_0,
        size,
        add,
        sub,
        off,
        pcrel,
        r_type,
        0 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn write_print_statistics(mut file: *mut FILE) {
    fprintf(file, b"fixups: %d\n\0" as *const u8 as *const libc::c_char, n_fixups);
}
#[no_mangle]
pub unsafe extern "C" fn print_fixup(mut fixp: *mut fixS) {
    indent_level = 1 as libc::c_int;
    fprintf(stderr, b"fix \0" as *const u8 as *const libc::c_char);
    fprintf(
        stderr,
        b"%016lx\0" as *const u8 as *const libc::c_char,
        fixp as bfd_hostptr_t,
    );
    fprintf(
        stderr,
        b" %s:%d\0" as *const u8 as *const libc::c_char,
        (*fixp).fx_file,
        (*fixp).fx_line,
    );
    if (*fixp).fx_pcrel() != 0 {
        fprintf(stderr, b" pcrel\0" as *const u8 as *const libc::c_char);
    }
    if (*fixp).fx_pcrel_adjust() != 0 {
        fprintf(
            stderr,
            b" pcrel_adjust=%d\0" as *const u8 as *const libc::c_char,
            (*fixp).fx_pcrel_adjust(),
        );
    }
    if (*fixp).fx_tcbit() != 0 {
        fprintf(stderr, b" tcbit\0" as *const u8 as *const libc::c_char);
    }
    if (*fixp).fx_done() != 0 {
        fprintf(stderr, b" done\0" as *const u8 as *const libc::c_char);
    }
    fprintf(
        stderr,
        b"\n    size=%d frag=\0" as *const u8 as *const libc::c_char,
        (*fixp).fx_size() as libc::c_int,
    );
    fprintf(
        stderr,
        b"%016lx\0" as *const u8 as *const libc::c_char,
        (*fixp).fx_frag as bfd_hostptr_t,
    );
    fprintf(
        stderr,
        b" where=%ld offset=%lx addnumber=%lx\0" as *const u8 as *const libc::c_char,
        (*fixp).fx_where as libc::c_long,
        (*fixp).fx_offset,
        (*fixp).fx_addnumber,
    );
    fprintf(
        stderr,
        b"\n    %s (%d)\0" as *const u8 as *const libc::c_char,
        bfd_get_reloc_code_name((*fixp).fx_r_type),
        (*fixp).fx_r_type as libc::c_uint,
    );
    if !((*fixp).fx_addsy).is_null() {
        fprintf(stderr, b"\n   +<\0" as *const u8 as *const libc::c_char);
        print_symbol_value_1(stderr, (*fixp).fx_addsy);
        fprintf(stderr, b">\0" as *const u8 as *const libc::c_char);
    }
    if !((*fixp).fx_subsy).is_null() {
        fprintf(stderr, b"\n   -<\0" as *const u8 as *const libc::c_char);
        print_symbol_value_1(stderr, (*fixp).fx_subsy);
        fprintf(stderr, b">\0" as *const u8 as *const libc::c_char);
    }
    fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
}
static mut frags_chained: libc::c_int = 0 as libc::c_int;
static mut n_fixups: libc::c_int = 0;
unsafe extern "C" fn fix_new_internal(
    mut frag: *mut fragS,
    mut where_0: libc::c_ulong,
    mut size: libc::c_ulong,
    mut add_symbol: *mut symbolS,
    mut sub_symbol: *mut symbolS,
    mut offset: offsetT,
    mut pcrel: libc::c_int,
    mut r_type: bfd_reloc_code_real,
    mut at_beginning: libc::c_int,
) -> *mut fixS {
    let mut fixP: *mut fixS = 0 as *mut fixS;
    n_fixups += 1;
    n_fixups;
    fixP = ({
        let mut __h: *mut obstack = &mut notes as *mut obstack;
        let mut __o: *mut obstack = __h;
        let mut __len: size_t = ::core::mem::size_of::<fixS>() as libc::c_ulong;
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
    }) as *mut fixS;
    (*fixP).fx_frag = frag;
    (*fixP).fx_where = where_0;
    (*fixP).set_fx_size(size as libc::c_uint);
    if (*fixP).fx_size() as libc::c_ulong != size {
        as_bad(
            dcgettext(
                0 as *const libc::c_char,
                b"field fx_size too small to hold %lu\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            size,
        );
        as_abort(
            b"write.c\0" as *const u8 as *const libc::c_char,
            168 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 129],
                &[libc::c_char; 129],
            >(
                b"fixS *fix_new_internal(fragS *, unsigned long, unsigned long, symbolS *, symbolS *, offsetT, int, enum bfd_reloc_code_real, int)\0",
            ))
                .as_ptr(),
        );
    }
    (*fixP).fx_addsy = add_symbol;
    (*fixP).fx_subsy = sub_symbol;
    (*fixP).fx_offset = offset as valueT;
    (*fixP).fx_dot_value = dot_value;
    (*fixP).fx_dot_frag = dot_frag;
    (*fixP).set_fx_pcrel(pcrel as libc::c_uint);
    (*fixP).fx_r_type = r_type;
    (*fixP).set_fx_pcrel_adjust(0 as libc::c_int);
    (*fixP).fx_addnumber = 0 as libc::c_int as valueT;
    (*fixP).set_fx_tcbit(0 as libc::c_int as libc::c_uint);
    (*fixP).set_fx_tcbit2(0 as libc::c_int as libc::c_uint);
    (*fixP).set_fx_done(0 as libc::c_int as libc::c_uint);
    (*fixP).set_fx_no_overflow(0 as libc::c_int as libc::c_uint);
    (*fixP).set_fx_signed(0 as libc::c_int as libc::c_uint);
    (*fixP).fx_file = as_where(&mut (*fixP).fx_line);
    let mut seg_fix_rootP: *mut *mut fixS = if frags_chained != 0 {
        &mut (*((bfd_section_userdata
            as unsafe extern "C" fn(
                *const asection,
            ) -> *mut libc::c_void)(now_seg as *const asection)
            as *mut segment_info_type))
            .fix_root
    } else {
        &mut (*frchain_now).fix_root
    };
    let mut seg_fix_tailP: *mut *mut fixS = if frags_chained != 0 {
        &mut (*((bfd_section_userdata
            as unsafe extern "C" fn(
                *const asection,
            ) -> *mut libc::c_void)(now_seg as *const asection)
            as *mut segment_info_type))
            .fix_tail
    } else {
        &mut (*frchain_now).fix_tail
    };
    if at_beginning != 0 {
        (*fixP).fx_next = *seg_fix_rootP;
        *seg_fix_rootP = fixP;
        if ((*fixP).fx_next).is_null() {
            *seg_fix_tailP = fixP;
        }
    } else {
        (*fixP).fx_next = 0 as *mut fix;
        if !(*seg_fix_tailP).is_null() {
            (**seg_fix_tailP).fx_next = fixP;
        } else {
            *seg_fix_rootP = fixP;
        }
        *seg_fix_tailP = fixP;
    }
    return fixP;
}
unsafe extern "C" fn renumber_sections(
    mut abfd: *mut bfd,
    mut sec: *mut asection,
    mut countparg: *mut libc::c_void,
) {
    let mut countp: *mut libc::c_int = countparg as *mut libc::c_int;
    (*sec).index = *countp as libc::c_uint;
    *countp += 1;
    *countp;
}
unsafe extern "C" fn chain_frchains_together_1(
    mut section: segT,
    mut frchp: *mut frchain,
) -> *mut fragS {
    let mut dummy: fragS = fragS {
        fr_address: 0,
        last_fr_address: 0,
        fr_fix: 0,
        fr_var: 0,
        fr_offset: 0,
        fr_symbol: 0 as *const symbolS as *mut symbolS,
        fr_opcode: 0 as *const libc::c_char as *mut libc::c_char,
        fr_next: 0 as *const frag as *mut frag,
        fr_file: 0 as *const libc::c_char,
        fr_line: 0,
        line: 0 as *const list_info_struct as *mut list_info_struct,
        region_relax_marker_has_code_insn_addr: [0; 3],
        c2rust_padding: [0; 1],
        fr_type: rs_dummy,
        fr_subtype: 0,
        tc_frag_data: i386_tc_frag_data {
            u: C2RustUnnamed_25 {
                padding_fragP: 0 as *const fragS as *mut fragS,
            },
            padding_address: 0,
            isa: PROCESSOR_UNKNOWN,
            isa_flags: i386_cpu_flags {
                bitfield: C2RustUnnamed_24 {
                    cpui186_cpui286_cpui386_cpui486_cpui586_cpui686_cpucmov_cpufxsr_cpuclflush_cpunop_cpusyscall_cpu8087_cpu287_cpu387_cpu687_cpufisttp_cpummx_cpusse_cpusse2_cpua3dnow_cpua3dnowa_cpusse3_cpupadlock_cpusvme_cpuvmx_cpusmx_cpussse3_cpusse4a_cpulzcnt_cpupopcnt_cpusse4_1_cpusse4_2_cpuavx_cpuavx2_cpuavx512f_cpuavx512cd_cpuavx512er_cpuavx512pf_cpuavx512vl_cpuavx512dq_cpuavx512bw_cpul1om_cpuk1om_cpuiamcu_cpuxsave_cpuxsaveopt_cpuaes_cpupclmul_cpufma_cpufma4_cpuxop_cpulwp_cpubmi_cputbm_cpumovbe_cpucx16_cpuept_cpurdtscp_cpufsgsbase_cpurdrnd_cpuf16c_cpubmi2_cpuhle_cpurtm_cpuinvpcid_cpuvmfunc_cpumpx_cpulm_cpurdseed_cpuadx_cpuprfchw_cpusmap_cpusha_cpuclflushopt_cpuxsaves_cpuxsavec_cpuprefetchwt1_cpuse1_cpuclwb_cpuavx512ifma_cpuavx512vbmi_cpuavx512_4fmaps_cpuavx512_4vnniw_cpuavx512_vpopcntdq_cpuavx512_vbmi2_cpuavx512_vnni_cpuavx512_bitalg_cpuavx512_bf16_cpuavx512_vp2intersect_cputdx_cpuavx_vnni_cpumwaitx_cpuclzero_cpuospke_cpurdpid_cpuptwrite_cpuibt_cpushstk_cpuamx_int8_cpuamx_bf16_cpuamx_tile_cpugfni_cpuvaes_cpuvpclmulqdq_cpuwbnoinvd_cpupconfig_cpuwaitpkg_cpuuintr_cpucldemote_cpumovdiri_cpumovdir64b_cpuenqcmd_cpuserialize_cpurdpru_cpumcommit_cpusev_es_cputsxldtrk_cpukl_cpuwidekl_cpuhreset_cpuinvlpgb_cputlbsync_cpusnp_cpu64_cpuno64_unused: [0; 16],
                },
            },
            tune: PROCESSOR_UNKNOWN,
            max_bytes: 0,
            length: 0,
            last_length: 0,
            max_prefix_length: 0,
            prefix_length: 0,
            default_prefix: 0,
            cmp_size: 0,
            mf_type_classified_branch_type: [0; 1],
            c2rust_padding: [0; 5],
        },
        fr_literal: [0; 1],
    };
    let mut prev_frag: *mut fragS = &mut dummy;
    let mut fix_dummy: fixS = fixS {
        fx_next: 0 as *mut fix,
        fx_pcrel_fx_done_fx_no_overflow_fx_signed_fx_tcbit_fx_tcbit2_fx_unused_fx_pcrel_adjust_fx_size: [0; 4],
        fx_r_type: _dummy_first_bfd_reloc_code_real,
        fx_frag: 0 as *mut fragS,
        fx_where: 0,
        fx_addsy: 0 as *mut symbolS,
        fx_subsy: 0 as *mut symbolS,
        fx_offset: 0,
        fx_dot_value: 0,
        fx_dot_frag: 0 as *mut fragS,
        fx_addnumber: 0,
        fx_file: 0 as *const libc::c_char,
        fx_line: 0,
    };
    let mut prev_fix: *mut fixS = &mut fix_dummy;
    while !frchp.is_null() {
        (*prev_frag).fr_next = (*frchp).frch_root;
        prev_frag = (*frchp).frch_last;
        if (*prev_frag).fr_type as libc::c_uint != 0 as libc::c_int as libc::c_uint
        {} else {
            as_abort(
                b"write.c\0" as *const u8 as *const libc::c_char,
                396 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 57],
                    &[libc::c_char; 57],
                >(b"fragS *chain_frchains_together_1(segT, struct frchain *)\0"))
                    .as_ptr(),
            );
        };
        if !((*frchp).fix_root).is_null() {
            if ((*(bfd_section_userdata(section as *const asection)
                as *mut segment_info_type))
                .fix_root)
                .is_null()
            {
                let ref mut fresh3 = (*(bfd_section_userdata(section as *const asection)
                    as *mut segment_info_type))
                    .fix_root;
                *fresh3 = (*frchp).fix_root;
            }
            (*prev_fix).fx_next = (*frchp).fix_root;
            let ref mut fresh4 = (*(bfd_section_userdata(section as *const asection)
                as *mut segment_info_type))
                .fix_tail;
            *fresh4 = (*frchp).fix_tail;
            prev_fix = (*frchp).fix_tail;
        }
        frchp = (*frchp).frch_next;
    }
    if prev_frag != &mut dummy as *mut fragS
        && (*prev_frag).fr_type as libc::c_uint != 0 as libc::c_int as libc::c_uint
    {} else {
        as_abort(
            b"write.c\0" as *const u8 as *const libc::c_char,
            407 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 57],
                &[libc::c_char; 57],
            >(b"fragS *chain_frchains_together_1(segT, struct frchain *)\0"))
                .as_ptr(),
        );
    };
    (*prev_frag).fr_next = 0 as *mut frag;
    return prev_frag;
}
unsafe extern "C" fn chain_frchains_together(
    mut abfd: *mut bfd,
    mut section: segT,
    mut xxx: *mut libc::c_void,
) {
    let mut info: *mut segment_info_type = 0 as *mut segment_info_type;
    info = bfd_section_userdata(section as *const asection) as *mut segment_info_type;
    if !info.is_null() {
        (*(*info).frchainP)
            .frch_last = chain_frchains_together_1(section, (*info).frchainP);
    }
    frags_chained = 1 as libc::c_int;
}
unsafe extern "C" fn cvt_frag_to_fill(mut sec: segT, mut fragP: *mut fragS) {
    let mut current_block_32: u64;
    match (*fragP).fr_type as libc::c_uint {
        9 => {
            current_block_32 = 3640593987805443782;
        }
        2 | 3 | 4 | 5 | 8 => {
            if (*fragP).fr_type as libc::c_uint
                == rs_align_code as libc::c_int as libc::c_uint
            {
                let mut __count: offsetT = ((*(*fragP).fr_next).fr_address)
                    .wrapping_sub((*fragP).fr_address)
                    .wrapping_sub((*fragP).fr_fix) as offsetT;
                if __count > 0 as libc::c_int as libc::c_long
                    && __count as libc::c_uint <= (*fragP).tc_frag_data.max_bytes
                {
                    i386_generate_nops(
                        fragP,
                        ((*fragP).fr_literal)
                            .as_mut_ptr()
                            .offset((*fragP).fr_fix as isize),
                        __count,
                        0 as libc::c_int,
                    );
                }
            }
            current_block_32 = 3640593987805443782;
        }
        1 | 10 => {
            current_block_32 = 8704759739624374314;
        }
        11 => {
            let mut value: valueT = S_GET_VALUE((*fragP).fr_symbol);
            let mut size: libc::c_int = 0;
            if S_IS_DEFINED((*fragP).fr_symbol) == 0 {
                as_bad_where(
                    (*fragP).fr_file,
                    (*fragP).fr_line,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"leb128 operand is an undefined symbol: %s\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    S_GET_NAME((*fragP).fr_symbol),
                );
            }
            size = output_leb128(
                ((*fragP).fr_literal).as_mut_ptr().offset((*fragP).fr_fix as isize),
                value,
                (*fragP).fr_subtype as libc::c_int,
            ) as libc::c_int;
            (*fragP)
                .fr_fix = ((*fragP).fr_fix as libc::c_ulong)
                .wrapping_add(size as libc::c_ulong) as valueT as valueT;
            (*fragP).fr_type = rs_fill;
            (*fragP).fr_var = 0 as libc::c_int as offsetT;
            (*fragP).fr_offset = 0 as libc::c_int as offsetT;
            (*fragP).fr_symbol = 0 as *mut symbolS;
            current_block_32 = 8704759739624374314;
        }
        12 => {
            eh_frame_convert_frag(fragP);
            current_block_32 = 8704759739624374314;
        }
        13 => {
            dwarf2dbg_convert_frag(fragP);
            current_block_32 = 8704759739624374314;
        }
        7 => {
            md_convert_frag(stdoutput, sec, fragP);
            if ((*fragP).fr_next).is_null()
                || ((*(*fragP).fr_next).fr_address).wrapping_sub((*fragP).fr_address)
                    == (*fragP).fr_fix
            {} else {
                as_abort(
                    b"write.c\0" as *const u8 as *const libc::c_char,
                    504 as libc::c_int,
                    (*::core::mem::transmute::<
                        &[u8; 37],
                        &[libc::c_char; 37],
                    >(b"void cvt_frag_to_fill(segT, fragS *)\0"))
                        .as_ptr(),
                );
            };
            frag_wane(fragP);
            current_block_32 = 8704759739624374314;
        }
        _ => {
            as_fatal(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Case value %ld unexpected at line %d of file \"%s\"\n\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*fragP).fr_type as libc::c_long,
                532 as libc::c_int,
                b"write.c\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    match current_block_32 {
        3640593987805443782 => {
            (*fragP)
                .fr_offset = ((*(*fragP).fr_next).fr_address)
                .wrapping_sub((*fragP).fr_address)
                .wrapping_sub((*fragP).fr_fix)
                .wrapping_div((*fragP).fr_var as libc::c_ulong) as offsetT;
            if (*fragP).fr_offset < 0 as libc::c_int as libc::c_long {
                as_bad_where(
                    (*fragP).fr_file,
                    (*fragP).fr_line,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"attempt to .org/.space/.nops backwards? (%ld)\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    (*fragP).fr_offset,
                );
                (*fragP).fr_offset = 0 as libc::c_int as offsetT;
            }
            if (*fragP).fr_type as libc::c_uint
                == rs_space_nop as libc::c_int as libc::c_uint
            {
                (*fragP).fr_type = rs_fill_nop;
            } else {
                (*fragP).fr_type = rs_fill;
            }
        }
        _ => {}
    };
}
unsafe extern "C" fn relax_seg(
    mut abfd: *mut bfd,
    mut sec: *mut asection,
    mut xxx: *mut libc::c_void,
) {
    let mut seginfo: *mut segment_info_type = bfd_section_userdata(sec)
        as *mut segment_info_type;
    let mut info: *mut relax_seg_info = xxx as *mut relax_seg_info;
    if !seginfo.is_null() && !((*seginfo).frchainP).is_null()
        && relax_segment((*(*seginfo).frchainP).frch_root, sec, (*info).pass) != 0
    {
        (*info).changed = 1 as libc::c_int;
    }
}
unsafe extern "C" fn size_seg(
    mut abfd: *mut bfd,
    mut sec: *mut asection,
    mut xxx: *mut libc::c_void,
) {
    let mut flags: flagword = 0;
    let mut fragp: *mut fragS = 0 as *mut fragS;
    let mut seginfo: *mut segment_info_type = 0 as *mut segment_info_type;
    let mut x: libc::c_int = 0;
    let mut size: valueT = 0;
    let mut newsize: valueT = 0;
    subseg_change(sec, 0 as libc::c_int);
    seginfo = bfd_section_userdata(sec) as *mut segment_info_type;
    if !seginfo.is_null() && !((*seginfo).frchainP).is_null() {
        fragp = (*(*seginfo).frchainP).frch_root;
        while !fragp.is_null() {
            cvt_frag_to_fill(sec, fragp);
            fragp = (*fragp).fr_next;
        }
        fragp = (*(*seginfo).frchainP).frch_root;
        while !((*fragp).fr_next).is_null() {
            fragp = (*fragp).fr_next;
        }
        size = ((*fragp).fr_address).wrapping_add((*fragp).fr_fix);
    } else {
        size = 0 as libc::c_int as valueT;
    }
    flags = bfd_section_flags(sec);
    if size == 0 as libc::c_int as libc::c_ulong
        && bfd_section_size(sec) != 0 as libc::c_int as libc::c_ulong
        && flags & 0x100 as libc::c_int as libc::c_uint
            != 0 as libc::c_int as libc::c_uint
    {
        return;
    }
    if size > 0 as libc::c_int as libc::c_ulong && (*seginfo).bss() == 0 {
        flags |= 0x100 as libc::c_int as libc::c_uint;
    }
    flags &= !(0x4 as libc::c_int) as libc::c_uint;
    x = bfd_set_section_flags(sec, flags) as libc::c_int;
    if x != 0 {} else {
        as_abort(
            b"write.c\0" as *const u8 as *const libc::c_char,
            593 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 41],
                &[libc::c_char; 41],
            >(b"void size_seg(bfd *, asection *, void *)\0"))
                .as_ptr(),
        );
    };
    if do_not_pad_sections_to_alignment != 0 {
        newsize = size;
    } else {
        newsize = md_section_align(sec, size);
    }
    x = bfd_set_section_size(sec, newsize) as libc::c_int;
    if x != 0 {} else {
        as_abort(
            b"write.c\0" as *const u8 as *const libc::c_char,
            602 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 41],
                &[libc::c_char; 41],
            >(b"void size_seg(bfd *, asection *, void *)\0"))
                .as_ptr(),
        );
    };
    if newsize >= size {} else {
        as_abort(
            b"write.c\0" as *const u8 as *const libc::c_char,
            606 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 41],
                &[libc::c_char; 41],
            >(b"void size_seg(bfd *, asection *, void *)\0"))
                .as_ptr(),
        );
    };
    if size != newsize {
        let mut last: *mut fragS = (*(*seginfo).frchainP).frch_last;
        fragp = (*(*seginfo).frchainP).frch_root;
        while (*fragp).fr_next != last {
            fragp = (*fragp).fr_next;
        }
        (*last).fr_address = size;
        if newsize.wrapping_sub(size).wrapping_rem((*fragp).fr_var as libc::c_ulong)
            == 0 as libc::c_int as libc::c_ulong
        {
            (*fragp)
                .fr_offset = ((*fragp).fr_offset as libc::c_ulong)
                .wrapping_add(
                    newsize
                        .wrapping_sub(size)
                        .wrapping_div((*fragp).fr_var as libc::c_ulong),
                ) as offsetT as offsetT;
        } else {
            as_abort(
                b"write.c\0" as *const u8 as *const libc::c_char,
                621 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 41],
                    &[libc::c_char; 41],
                >(b"void size_seg(bfd *, asection *, void *)\0"))
                    .as_ptr(),
            );
        }
    }
}
unsafe extern "C" fn resolve_reloc_expr_symbols() {
    let mut addr_mask: bfd_vma = 1 as libc::c_int as bfd_vma;
    let mut r: *mut reloc_list = 0 as *mut reloc_list;
    addr_mask
        <<= (bfd_arch_bits_per_address(stdoutput))
            .wrapping_sub(1 as libc::c_int as libc::c_uint);
    addr_mask <<= 1 as libc::c_int;
    addr_mask = (addr_mask as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as bfd_vma as bfd_vma;
    r = reloc_list;
    while !r.is_null() {
        let mut howto: *const reloc_howto_type = (*r).u.a.howto;
        let mut symval: *mut expressionS = 0 as *mut expressionS;
        let mut sym: *mut symbolS = 0 as *mut symbolS;
        let mut offset: bfd_vma = 0;
        let mut addend: bfd_vma = 0;
        let mut sec: *mut asection = 0 as *mut asection;
        resolve_symbol_value((*r).u.a.offset_sym);
        symval = symbol_get_value_expression((*r).u.a.offset_sym);
        offset = 0 as libc::c_int as bfd_vma;
        sym = 0 as *mut symbolS;
        if (*symval).X_op() as libc::c_int == O_constant as libc::c_int {
            sym = (*r).u.a.offset_sym;
        } else if (*symval).X_op() as libc::c_int == O_symbol as libc::c_int {
            sym = (*symval).X_add_symbol;
            offset = (*symval).X_add_number as bfd_vma;
            symval = symbol_get_value_expression((*symval).X_add_symbol);
        }
        if sym.is_null() || (*symval).X_op() as libc::c_int != O_constant as libc::c_int
            || {
                sec = S_GET_SEGMENT(sym);
                sec.is_null()
            }
            || !(sec
                != &mut *_bfd_std_section.as_mut_ptr().offset(2 as libc::c_int as isize)
                    as *mut asection
                && sec
                    != &mut *_bfd_std_section
                        .as_mut_ptr()
                        .offset(1 as libc::c_int as isize) as *mut asection
                && sec != reg_section && sec != expr_section)
        {
            as_bad_where(
                (*r).file,
                (*r).line,
                dcgettext(
                    0 as *const libc::c_char,
                    b"invalid offset expression\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            sec = 0 as *mut asection;
        } else {
            offset = (offset as libc::c_ulong).wrapping_add(S_GET_VALUE(sym)) as bfd_vma
                as bfd_vma;
        }
        sym = 0 as *mut symbolS;
        addend = (*r).u.a.addend;
        if !((*r).u.a.sym).is_null() {
            resolve_symbol_value((*r).u.a.sym);
            symval = symbol_get_value_expression((*r).u.a.sym);
            if (*symval).X_op() as libc::c_int == O_constant as libc::c_int {
                sym = (*r).u.a.sym;
            } else if (*symval).X_op() as libc::c_int == O_symbol as libc::c_int {
                sym = (*symval).X_add_symbol;
                addend = (addend as libc::c_ulong)
                    .wrapping_add((*symval).X_add_number as libc::c_ulong) as bfd_vma
                    as bfd_vma;
                symval = symbol_get_value_expression((*symval).X_add_symbol);
            }
            if (*symval).X_op() as libc::c_int != O_constant as libc::c_int {
                as_bad_where(
                    (*r).file,
                    (*r).line,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"invalid reloc expression\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                sec = 0 as *mut asection;
            } else if !sym.is_null() && !sec.is_null() {
                if S_IS_LOCAL(sym) != 0 && S_IS_DEFINED(sym) != 0
                    && symbol_section_p(sym) == 0
                    && ((*sec).use_rela_p() as libc::c_int != 0
                        || (*howto).partial_inplace() as libc::c_int != 0
                            && ((*howto).pc_relative() == 0
                                || (*howto).src_mask == addr_mask))
                {
                    let mut symsec: *mut asection = S_GET_SEGMENT(sym);
                    if !((*symsec).flags & 0x800000 as libc::c_int as libc::c_uint
                        != 0 as libc::c_int as libc::c_uint
                        && addend != 0 as libc::c_int as libc::c_ulong
                        || (*symsec).flags & 0x400 as libc::c_int as libc::c_uint
                            != 0 as libc::c_int as libc::c_uint)
                    {
                        addend = (addend as libc::c_ulong).wrapping_add(S_GET_VALUE(sym))
                            as bfd_vma as bfd_vma;
                        sym = section_symbol(symsec);
                    }
                }
                symbol_mark_used_in_reloc(sym);
            }
        }
        if sym.is_null() {
            if abs_section_sym.is_null() {
                abs_section_sym = section_symbol(
                    &mut *_bfd_std_section.as_mut_ptr().offset(2 as libc::c_int as isize),
                );
            }
            sym = abs_section_sym;
        }
        (*r).u.b.sec = sec;
        (*r).u.b.s = symbol_get_bfdsym(sym);
        (*r).u.b.r.sym_ptr_ptr = &mut (*r).u.b.s;
        (*r).u.b.r.address = offset;
        (*r).u.b.r.addend = addend;
        (*r).u.b.r.howto = howto;
        r = (*r).next;
    }
}
unsafe extern "C" fn adjust_reloc_syms(
    mut abfd: *mut bfd,
    mut sec: *mut asection,
    mut xxx: *mut libc::c_void,
) {
    let mut seginfo: *mut segment_info_type = bfd_section_userdata(sec)
        as *mut segment_info_type;
    let mut fixp: *mut fixS = 0 as *mut fixS;
    if seginfo.is_null() {
        return;
    }
    let mut current_block_20: u64;
    fixp = (*seginfo).fix_root;
    while !fixp.is_null() {
        if !((*fixp).fx_done() != 0) {
            if !((*fixp).fx_addsy).is_null() {
                let mut sym: *mut symbolS = 0 as *mut symbolS;
                let mut symsec: *mut asection = 0 as *mut asection;
                sym = (*fixp).fx_addsy;
                resolve_symbol_value(sym);
                if !((*fixp).fx_subsy).is_null() {
                    resolve_symbol_value((*fixp).fx_subsy);
                }
                while symbol_equated_reloc_p(sym) != 0 || S_IS_WEAKREFR(sym) != 0 {
                    let mut newsym: *mut symbolS = (*symbol_get_value_expression(sym))
                        .X_add_symbol;
                    if sym == newsym {
                        break;
                    }
                    (*fixp)
                        .fx_offset = ((*fixp).fx_offset as libc::c_ulong)
                        .wrapping_add(
                            (*symbol_get_value_expression(sym)).X_add_number
                                as libc::c_ulong,
                        ) as valueT as valueT;
                    (*fixp).fx_addsy = newsym;
                    sym = newsym;
                }
                if symbol_mri_common_p(sym) != 0 {
                    (*fixp)
                        .fx_offset = ((*fixp).fx_offset as libc::c_ulong)
                        .wrapping_add(S_GET_VALUE(sym)) as valueT as valueT;
                    (*fixp).fx_addsy = (*symbol_get_value_expression(sym)).X_add_symbol;
                } else if !(S_FORCE_RELOC((*fixp).fx_addsy, 1 as libc::c_int) != 0) {
                    if !(tc_i386_fix_adjustable(fixp) == 0) {
                        if symbol_section_p(sym) != 0 {
                            symbol_mark_used_in_reloc(sym);
                        } else {
                            symsec = S_GET_SEGMENT(sym);
                            if symsec.is_null() {
                                as_abort(
                                    b"write.c\0" as *const u8 as *const libc::c_char,
                                    862 as libc::c_int,
                                    (*::core::mem::transmute::<
                                        &[u8; 50],
                                        &[libc::c_char; 50],
                                    >(b"void adjust_reloc_syms(bfd *, asection *, void *)\0"))
                                        .as_ptr(),
                                );
                            }
                            if !(bfd_is_abs_section(symsec) as libc::c_int != 0
                                || symsec == reg_section)
                            {
                                if symsec != sec && S_IS_LOCAL(sym) == 0 {
                                    if (*symsec).flags & 0x20000 as libc::c_int as libc::c_uint
                                        != 0 as libc::c_int as libc::c_uint
                                        || 1 as libc::c_int != 0
                                            && startswith(
                                                bfd_section_name(symsec),
                                                b".gnu.linkonce\0" as *const u8 as *const libc::c_char,
                                            ) as libc::c_int != 0
                                    {
                                        current_block_20 = 16658872821858055392;
                                    } else {
                                        current_block_20 = 15125582407903384992;
                                    }
                                } else {
                                    current_block_20 = 15125582407903384992;
                                }
                                match current_block_20 {
                                    16658872821858055392 => {}
                                    _ => {
                                        if !((*symsec).flags
                                            & 0x800000 as libc::c_int as libc::c_uint
                                            != 0 as libc::c_int as libc::c_uint
                                            && ((*fixp).fx_offset != 0 as libc::c_int as libc::c_ulong
                                                || !((*fixp).fx_subsy).is_null()))
                                        {
                                            if !((*symsec).flags & 0x400 as libc::c_int as libc::c_uint
                                                != 0 as libc::c_int as libc::c_uint)
                                            {
                                                (*fixp)
                                                    .fx_offset = ((*fixp).fx_offset as libc::c_ulong)
                                                    .wrapping_add(S_GET_VALUE(sym)) as valueT as valueT;
                                                (*fixp).fx_addsy = section_symbol(S_GET_SEGMENT(sym));
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        fixp = (*fixp).fx_next;
    }
}
unsafe extern "C" fn fixup_segment(mut fixP: *mut fixS, mut this_segment: segT) {
    let mut add_number: valueT = 0;
    let mut fragP: *mut fragS = 0 as *mut fragS;
    if !fixP.is_null() && abs_section_sym.is_null() {
        abs_section_sym = section_symbol(
            &mut *_bfd_std_section.as_mut_ptr().offset(2 as libc::c_int as isize),
        );
    }
    if linkrelax != 0 && 1 as libc::c_int != 0 {
        while !fixP.is_null() {
            if (*fixP).fx_done() == 0 {
                if ((*fixP).fx_addsy).is_null() {
                    (*fixP).fx_addsy = abs_section_sym;
                }
                symbol_mark_used_in_reloc((*fixP).fx_addsy);
                if !((*fixP).fx_subsy).is_null() {
                    symbol_mark_used_in_reloc((*fixP).fx_subsy);
                }
            }
            fixP = (*fixP).fx_next;
        }
        return;
    }
    while !fixP.is_null() {
        let mut add_symbol_segment: segT = &mut *_bfd_std_section
            .as_mut_ptr()
            .offset(2 as libc::c_int as isize) as *mut asection;
        fragP = (*fixP).fx_frag;
        if !(i386_validate_fix(fixP) == 0) {
            add_number = (*fixP).fx_offset;
            if !((*fixP).fx_addsy).is_null() {
                add_symbol_segment = S_GET_SEGMENT((*fixP).fx_addsy);
            }
            if !((*fixP).fx_subsy).is_null() {
                let mut sub_symbol_segment: segT = 0 as *mut asection;
                resolve_symbol_value((*fixP).fx_subsy);
                sub_symbol_segment = S_GET_SEGMENT((*fixP).fx_subsy);
                if !((*fixP).fx_addsy).is_null()
                    && sub_symbol_segment == add_symbol_segment
                    && S_FORCE_RELOC((*fixP).fx_addsy, 0 as libc::c_int) == 0
                    && S_FORCE_RELOC((*fixP).fx_subsy, 0 as libc::c_int) == 0
                    && (add_symbol_segment
                        != &mut *_bfd_std_section
                            .as_mut_ptr()
                            .offset(2 as libc::c_int as isize) as *mut asection
                        && add_symbol_segment
                            != &mut *_bfd_std_section
                                .as_mut_ptr()
                                .offset(1 as libc::c_int as isize) as *mut asection
                        && add_symbol_segment != reg_section
                        && add_symbol_segment != expr_section)
                {
                    add_number = (add_number as libc::c_ulong)
                        .wrapping_add(S_GET_VALUE((*fixP).fx_addsy)) as valueT as valueT;
                    add_number = (add_number as libc::c_ulong)
                        .wrapping_sub(S_GET_VALUE((*fixP).fx_subsy)) as valueT as valueT;
                    (*fixP).fx_offset = add_number;
                    (*fixP).fx_addsy = 0 as *mut symbolS;
                    (*fixP).fx_subsy = 0 as *mut symbolS;
                } else if sub_symbol_segment
                    == &mut *_bfd_std_section
                        .as_mut_ptr()
                        .offset(2 as libc::c_int as isize) as *mut asection
                    && S_FORCE_RELOC((*fixP).fx_subsy, 0 as libc::c_int) == 0
                    && !(0 as libc::c_int == 0 && add_symbol_segment == reg_section)
                {
                    add_number = (add_number as libc::c_ulong)
                        .wrapping_sub(S_GET_VALUE((*fixP).fx_subsy)) as valueT as valueT;
                    (*fixP).fx_offset = add_number;
                    (*fixP).fx_subsy = 0 as *mut symbolS;
                } else if sub_symbol_segment == this_segment
                    && S_FORCE_RELOC((*fixP).fx_subsy, 0 as libc::c_int) == 0
                    && !(0 as libc::c_int == 0 && add_symbol_segment == reg_section)
                {
                    add_number = (add_number as libc::c_ulong)
                        .wrapping_sub(S_GET_VALUE((*fixP).fx_subsy)) as valueT as valueT;
                    (*fixP)
                        .fx_offset = add_number
                        .wrapping_add((*fixP).fx_dot_value)
                        .wrapping_add((*(*fixP).fx_dot_frag).fr_address);
                    if 0 as libc::c_int != 0 || (*fixP).fx_pcrel() == 0 {
                        add_number = (add_number as libc::c_ulong)
                            .wrapping_add(md_pcrel_from(fixP) as libc::c_ulong) as valueT
                            as valueT;
                    }
                    (*fixP).fx_subsy = 0 as *mut symbolS;
                    (*fixP).set_fx_pcrel(1 as libc::c_int as libc::c_uint);
                } else if !((0 as libc::c_int != 0 || add_symbol_segment != reg_section)
                    && ((*fixP).fx_r_type as libc::c_uint
                        == BFD_RELOC_GPREL32 as libc::c_int as libc::c_uint
                        || (*fixP).fx_r_type as libc::c_uint
                            == BFD_RELOC_GPREL16 as libc::c_int as libc::c_uint))
                {
                    if 0 as libc::c_int == 0
                        && (add_symbol_segment == reg_section
                            || sub_symbol_segment == reg_section)
                    {
                        as_bad_where(
                            (*fixP).fx_file,
                            (*fixP).fx_line,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"register value used as expression\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                    } else {
                        as_bad_where(
                            (*fixP).fx_file,
                            (*fixP).fx_line,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"can't resolve `%s' {%s section} - `%s' {%s section}\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            if !((*fixP).fx_addsy).is_null() {
                                S_GET_NAME((*fixP).fx_addsy)
                            } else {
                                b"0\0" as *const u8 as *const libc::c_char
                            },
                            bfd_section_name(add_symbol_segment as *const asection),
                            S_GET_NAME((*fixP).fx_subsy),
                            bfd_section_name(sub_symbol_segment as *const asection),
                        );
                    }
                } else if sub_symbol_segment
                    != &mut *_bfd_std_section
                        .as_mut_ptr()
                        .offset(1 as libc::c_int as isize) as *mut asection
                    && !bfd_is_com_section(sub_symbol_segment as *const asection)
                    && 0 as libc::c_int != 0
                {
                    add_number = (add_number as libc::c_ulong)
                        .wrapping_sub(S_GET_VALUE((*fixP).fx_subsy)) as valueT as valueT;
                }
            }
            if !((*fixP).fx_addsy).is_null() {
                if add_symbol_segment == this_segment
                    && S_FORCE_RELOC((*fixP).fx_addsy, 0 as libc::c_int) == 0
                    && !((*fixP).fx_pcrel() == 0 || generic_force_reloc(fixP) != 0
                        || (*fixP).fx_r_type as libc::c_uint
                            == BFD_RELOC_386_PLT32 as libc::c_int as libc::c_uint
                        || (*fixP).fx_r_type as libc::c_uint
                            == BFD_RELOC_386_GOTPC as libc::c_int as libc::c_uint
                        || (*fixP).fx_r_type as libc::c_uint
                            == BFD_RELOC_X86_64_GOTPCREL as libc::c_int as libc::c_uint
                        || (*fixP).fx_r_type as libc::c_uint
                            == BFD_RELOC_X86_64_GOTPCRELX as libc::c_int as libc::c_uint
                        || (*fixP).fx_r_type as libc::c_uint
                            == BFD_RELOC_X86_64_REX_GOTPCRELX as libc::c_int
                                as libc::c_uint)
                {
                    add_number = (add_number as libc::c_ulong)
                        .wrapping_add(S_GET_VALUE((*fixP).fx_addsy)) as valueT as valueT;
                    (*fixP).fx_offset = add_number;
                    if (*fixP).fx_pcrel() != 0 {
                        add_number = (add_number as libc::c_ulong)
                            .wrapping_sub(md_pcrel_from(fixP) as libc::c_ulong) as valueT
                            as valueT;
                    }
                    (*fixP).fx_addsy = 0 as *mut symbolS;
                    (*fixP).set_fx_pcrel(0 as libc::c_int as libc::c_uint);
                } else if add_symbol_segment
                    == &mut *_bfd_std_section
                        .as_mut_ptr()
                        .offset(2 as libc::c_int as isize) as *mut asection
                    && S_FORCE_RELOC((*fixP).fx_addsy, 0 as libc::c_int) == 0
                    && !(generic_force_reloc(fixP) != 0
                        || (*fixP).fx_r_type as libc::c_uint
                            == BFD_RELOC_386_GOT32 as libc::c_int as libc::c_uint
                        || (*fixP).fx_r_type as libc::c_uint
                            == BFD_RELOC_386_GOT32X as libc::c_int as libc::c_uint
                        || (*fixP).fx_r_type as libc::c_uint
                            == BFD_RELOC_X86_64_GOTPCREL as libc::c_int as libc::c_uint
                        || (*fixP).fx_r_type as libc::c_uint
                            == BFD_RELOC_X86_64_GOTPCRELX as libc::c_int as libc::c_uint
                        || (*fixP).fx_r_type as libc::c_uint
                            == BFD_RELOC_X86_64_REX_GOTPCRELX as libc::c_int
                                as libc::c_uint)
                {
                    add_number = (add_number as libc::c_ulong)
                        .wrapping_add(S_GET_VALUE((*fixP).fx_addsy)) as valueT as valueT;
                    (*fixP).fx_offset = add_number;
                    (*fixP).fx_addsy = 0 as *mut symbolS;
                } else if add_symbol_segment
                    != &mut *_bfd_std_section
                        .as_mut_ptr()
                        .offset(1 as libc::c_int as isize) as *mut asection
                    && !bfd_is_com_section(add_symbol_segment as *const asection)
                    && 0 as libc::c_int != 0
                {
                    add_number = (add_number as libc::c_ulong)
                        .wrapping_add(S_GET_VALUE((*fixP).fx_addsy)) as valueT as valueT;
                }
            }
            if (*fixP).fx_pcrel() != 0 {
                add_number = (add_number as libc::c_ulong)
                    .wrapping_sub(md_pcrel_from(fixP) as libc::c_ulong) as valueT
                    as valueT;
                if (*fixP).fx_done() == 0 && ((*fixP).fx_addsy).is_null() {
                    (*fixP).fx_addsy = abs_section_sym;
                }
            }
            if (*fixP).fx_done() == 0 {
                md_apply_fix(fixP, &mut add_number, this_segment);
            }
            if (*fixP).fx_done() == 0 {
                if ((*fixP).fx_addsy).is_null() {
                    (*fixP).fx_addsy = abs_section_sym;
                }
                symbol_mark_used_in_reloc((*fixP).fx_addsy);
                if !((*fixP).fx_subsy).is_null() {
                    symbol_mark_used_in_reloc((*fixP).fx_subsy);
                }
            }
            if (*fixP).fx_no_overflow() == 0
                && (*fixP).fx_size() as libc::c_int != 0 as libc::c_int
            {
                if ((*fixP).fx_size() as libc::c_ulong)
                    < ::core::mem::size_of::<valueT>() as libc::c_ulong
                {
                    let mut mask: valueT = 0;
                    mask = 0 as libc::c_int as valueT;
                    mask = mask.wrapping_sub(1);
                    mask;
                    mask
                        <<= (*fixP).fx_size() as libc::c_int * 8 as libc::c_int
                            - (if (*fixP).fx_signed() as libc::c_int != 0 {
                                1 as libc::c_int
                            } else {
                                0 as libc::c_int
                            });
                    if add_number & mask != 0 as libc::c_int as libc::c_ulong
                        && (if (*fixP).fx_signed() as libc::c_int != 0 {
                            (add_number & mask != mask) as libc::c_int
                        } else {
                            (add_number.wrapping_neg() & mask
                                != 0 as libc::c_int as libc::c_ulong) as libc::c_int
                        }) != 0
                    {
                        let mut buf: [libc::c_char; 50] = [0; 50];
                        let mut buf2: [libc::c_char; 50] = [0; 50];
                        bfd_sprintf_vma(
                            stdoutput,
                            buf.as_mut_ptr(),
                            ((*fragP).fr_address).wrapping_add((*fixP).fx_where),
                        );
                        if add_number > 1000 as libc::c_int as libc::c_ulong {
                            bfd_sprintf_vma(stdoutput, buf2.as_mut_ptr(), add_number);
                        } else {
                            sprintf(
                                buf2.as_mut_ptr(),
                                b"%ld\0" as *const u8 as *const libc::c_char,
                                add_number as libc::c_long,
                            );
                        }
                        as_bad_where(
                            (*fixP).fx_file,
                            (*fixP).fx_line,
                            dcngettext(
                                0 as *const libc::c_char,
                                b"value of %s too large for field of %d byte at %s\0"
                                    as *const u8 as *const libc::c_char,
                                b"value of %s too large for field of %d bytes at %s\0"
                                    as *const u8 as *const libc::c_char,
                                (*fixP).fx_size() as libc::c_ulong,
                                5 as libc::c_int,
                            ),
                            buf2.as_mut_ptr(),
                            (*fixP).fx_size() as libc::c_int,
                            buf.as_mut_ptr(),
                        );
                    }
                }
            }
        }
        fixP = (*fixP).fx_next;
    }
}
unsafe extern "C" fn fix_segment(
    mut abfd: *mut bfd,
    mut sec: *mut asection,
    mut xxx: *mut libc::c_void,
) {
    let mut seginfo: *mut segment_info_type = bfd_section_userdata(sec)
        as *mut segment_info_type;
    fixup_segment((*seginfo).fix_root, sec);
}
unsafe extern "C" fn install_reloc(
    mut sec: *mut asection,
    mut reloc: *mut arelent,
    mut fragp: *mut fragS,
    mut file: *const libc::c_char,
    mut line: libc::c_uint,
) {
    let mut err: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: bfd_reloc_status_type = 0 as bfd_reloc_status_type;
    let mut sym: *mut asymbol = 0 as *mut asymbol;
    if !((*reloc).sym_ptr_ptr).is_null()
        && {
            sym = *(*reloc).sym_ptr_ptr;
            !sym.is_null()
        }
        && (*sym).flags & ((1 as libc::c_int) << 5 as libc::c_int) as libc::c_uint
            == 0 as libc::c_int as libc::c_uint
        && ((*sym).flags & ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_uint
            == 0 as libc::c_int as libc::c_uint
            || 1 as libc::c_int != 0 && !bfd_is_abs_section((*sym).section))
    {
        as_bad_where(
            file,
            line,
            dcgettext(
                0 as *const libc::c_char,
                b"redefined symbol cannot be used on reloc\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    s = bfd_install_relocation(
        stdoutput,
        reloc,
        ((*fragp).fr_literal).as_mut_ptr() as *mut libc::c_void,
        (*fragp).fr_address,
        sec,
        &mut err,
    );
    match s as libc::c_uint {
        2 => {}
        3 => {
            as_bad_where(
                file,
                line,
                dcgettext(
                    0 as *const libc::c_char,
                    b"relocation overflow\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
        4 => {
            as_bad_where(
                file,
                line,
                dcgettext(
                    0 as *const libc::c_char,
                    b"relocation out of range\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
        _ => {
            as_fatal(
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s:%u: bad return from bfd_install_relocation: %x\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                file,
                line,
                s as libc::c_uint,
            );
        }
    };
}
unsafe extern "C" fn get_frag_for_reloc(
    mut last_frag: *mut fragS,
    mut seginfo: *const segment_info_type,
    mut r: *const reloc_list,
) -> *mut fragS {
    let mut f: *mut fragS = 0 as *mut fragS;
    f = last_frag;
    while !f.is_null() {
        if (*f).fr_address <= (*r).u.b.r.address
            && (*r).u.b.r.address < ((*f).fr_address).wrapping_add((*f).fr_fix)
        {
            return f;
        }
        f = (*f).fr_next;
    }
    f = (*(*seginfo).frchainP).frch_root;
    while !f.is_null() {
        if (*f).fr_address <= (*r).u.b.r.address
            && (*r).u.b.r.address < ((*f).fr_address).wrapping_add((*f).fr_fix)
        {
            return f;
        }
        f = (*f).fr_next;
    }
    f = (*(*seginfo).frchainP).frch_root;
    while !f.is_null() {
        if (*f).fr_address <= (*r).u.b.r.address
            && (*r).u.b.r.address <= ((*f).fr_address).wrapping_add((*f).fr_fix)
        {
            return f;
        }
        f = (*f).fr_next;
    }
    as_bad_where(
        (*r).file,
        (*r).line,
        dcgettext(
            0 as *const libc::c_char,
            b"reloc not within (fixed part of) section\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    return 0 as *mut fragS;
}
unsafe extern "C" fn write_relocs(
    mut abfd: *mut bfd,
    mut sec: *mut asection,
    mut xxx: *mut libc::c_void,
) {
    let mut seginfo: *mut segment_info_type = bfd_section_userdata(sec)
        as *mut segment_info_type;
    let mut n: libc::c_uint = 0;
    let mut my_reloc_list: *mut reloc_list = 0 as *mut reloc_list;
    let mut rp: *mut *mut reloc_list = 0 as *mut *mut reloc_list;
    let mut r: *mut reloc_list = 0 as *mut reloc_list;
    let mut relocs: *mut *mut arelent = 0 as *mut *mut arelent;
    let mut fixp: *mut fixS = 0 as *mut fixS;
    let mut last_frag: *mut fragS = 0 as *mut fragS;
    if seginfo.is_null() {
        return;
    }
    n = 0 as libc::c_int as libc::c_uint;
    fixp = (*seginfo).fix_root;
    while !fixp.is_null() {
        if (*fixp).fx_done() == 0 {
            n = n.wrapping_add(1);
            n;
        }
        fixp = (*fixp).fx_next;
    }
    rp = &mut reloc_list;
    my_reloc_list = 0 as *mut reloc_list;
    loop {
        r = *rp;
        if r.is_null() {
            break;
        }
        if (*r).u.b.sec == sec {
            *rp = (*r).next;
            (*r).next = my_reloc_list;
            my_reloc_list = r;
            n = n.wrapping_add(1);
            n;
        } else {
            rp = &mut (*r).next;
        }
    }
    relocs = xcalloc(
        n as size_t,
        ::core::mem::size_of::<*mut arelent>() as libc::c_ulong,
    ) as *mut *mut arelent;
    n = 0 as libc::c_int as libc::c_uint;
    r = my_reloc_list;
    last_frag = 0 as *mut fragS;
    fixp = (*seginfo).fix_root;
    while !fixp.is_null() {
        let mut fx_size: libc::c_int = 0;
        let mut slack: libc::c_int = 0;
        let mut loc: valueT = 0;
        let mut reloc: *mut *mut arelent = 0 as *mut *mut arelent;
        let mut rel: *mut arelent = 0 as *mut arelent;
        reloc = &mut rel;
        if !((*fixp).fx_done() != 0) {
            fx_size = (*fixp).fx_size() as libc::c_int;
            slack = 0 as libc::c_int;
            if slack > 0 as libc::c_int {
                fx_size = if fx_size > slack {
                    fx_size - slack
                } else {
                    0 as libc::c_int
                };
            }
            loc = ((*fixp).fx_where).wrapping_add(fx_size as libc::c_ulong);
            if slack >= 0 as libc::c_int && loc > (*(*fixp).fx_frag).fr_fix {
                as_bad_where(
                    (*fixp).fx_file,
                    (*fixp).fx_line,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"internal error: fixup not contained within frag\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
            }
            *reloc = tc_gen_reloc(sec, fixp);
            if !(*reloc).is_null() {
                while !r.is_null() && (*r).u.b.r.address < (**reloc).address {
                    let mut f: *mut fragS = get_frag_for_reloc(last_frag, seginfo, r);
                    if !f.is_null() {
                        last_frag = f;
                        let fresh5 = n;
                        n = n.wrapping_add(1);
                        let ref mut fresh6 = *relocs.offset(fresh5 as isize);
                        *fresh6 = &mut (*r).u.b.r;
                        install_reloc(sec, &mut (*r).u.b.r, f, (*r).file, (*r).line);
                    }
                    r = (*r).next;
                }
                let fresh7 = n;
                n = n.wrapping_add(1);
                let ref mut fresh8 = *relocs.offset(fresh7 as isize);
                *fresh8 = *reloc;
                install_reloc(
                    sec,
                    *reloc,
                    (*fixp).fx_frag,
                    (*fixp).fx_file,
                    (*fixp).fx_line,
                );
            }
        }
        fixp = (*fixp).fx_next;
    }
    while !r.is_null() {
        let mut f_0: *mut fragS = get_frag_for_reloc(last_frag, seginfo, r);
        if !f_0.is_null() {
            last_frag = f_0;
            let fresh9 = n;
            n = n.wrapping_add(1);
            let ref mut fresh10 = *relocs.offset(fresh9 as isize);
            *fresh10 = &mut (*r).u.b.r;
            install_reloc(sec, &mut (*r).u.b.r, f_0, (*r).file, (*r).line);
        }
        r = (*r).next;
    }
    if n != 0 {
        let mut flags: flagword = bfd_section_flags(sec);
        flags |= 0x4 as libc::c_int as libc::c_uint;
        bfd_set_section_flags(sec, flags);
        (Some(((*(*stdoutput).xvec)._bfd_set_reloc).expect("non-null function pointer")))
            .expect("non-null function pointer")(stdoutput, sec, relocs, n);
    }
}
unsafe extern "C" fn compress_frag(
    mut strm: *mut z_stream_s,
    mut contents: *const libc::c_char,
    mut in_size: libc::c_int,
    mut last_newf: *mut *mut fragS,
    mut ob: *mut obstack,
) -> libc::c_int {
    let mut out_size: libc::c_int = 0;
    let mut total_out_size: libc::c_int = 0 as libc::c_int;
    let mut f: *mut fragS = *last_newf;
    let mut next_out: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut avail_out: libc::c_int = 0;
    while in_size > 0 as libc::c_int {
        avail_out = ({
            let mut __o1: *const obstack = ob;
            ((*__o1).chunk_limit).offset_from((*__o1).next_free) as libc::c_long
                as size_t
        }) as libc::c_int;
        if avail_out <= 0 as libc::c_int {
            let mut __o1: *mut obstack = ob;
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
            f = frag_alloc(ob);
            (*f).fr_type = rs_fill;
            (**last_newf).fr_next = f;
            *last_newf = f;
            avail_out = ({
                let mut __o1_0: *const obstack = ob;
                ((*__o1_0).chunk_limit).offset_from((*__o1_0).next_free) as libc::c_long
                    as size_t
            }) as libc::c_int;
        }
        if avail_out <= 0 as libc::c_int {
            as_fatal(
                dcgettext(
                    0 as *const libc::c_char,
                    b"can't extend frag\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
        next_out = (*ob).next_free as *mut libc::c_void as *mut libc::c_char;
        (*ob).next_free = ((*ob).next_free).offset(avail_out as isize);
        out_size = compress_data(
            strm,
            &mut contents,
            &mut in_size,
            &mut next_out,
            &mut avail_out,
        );
        if out_size < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
        (*f)
            .fr_fix = ((*f).fr_fix as libc::c_ulong)
            .wrapping_add(out_size as libc::c_ulong) as valueT as valueT;
        total_out_size += out_size;
        if avail_out > 0 as libc::c_int {
            (*ob).next_free = ((*ob).next_free).offset(-avail_out as isize);
        }
    }
    return total_out_size;
}
unsafe extern "C" fn compress_debug(
    mut abfd: *mut bfd,
    mut sec: *mut asection,
    mut xxx: *mut libc::c_void,
) {
    let mut seginfo: *mut segment_info_type = bfd_section_userdata(sec)
        as *mut segment_info_type;
    let mut f: *mut fragS = 0 as *mut fragS;
    let mut first_newf: *mut fragS = 0 as *mut fragS;
    let mut last_newf: *mut fragS = 0 as *mut fragS;
    let mut ob: *mut obstack = &mut (*(*seginfo).frchainP).frch_obstack;
    let mut uncompressed_size: bfd_size_type = (*sec).size;
    let mut compressed_size: bfd_size_type = 0;
    let mut section_name: *const libc::c_char = 0 as *const libc::c_char;
    let mut compressed_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut header: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut strm: *mut z_stream_s = 0 as *mut z_stream_s;
    let mut x: libc::c_int = 0;
    let mut flags: flagword = bfd_section_flags(sec);
    let mut header_size: libc::c_uint = 0;
    let mut compression_header_size: libc::c_uint = 0;
    if seginfo.is_null() || (*sec).size < 32 as libc::c_int as libc::c_ulong
        || flags & (0x1 as libc::c_int | 0x100 as libc::c_int) as libc::c_uint
            == 0x1 as libc::c_int as libc::c_uint
    {
        return;
    }
    section_name = bfd_section_name(sec);
    if !startswith(section_name, b".debug_\0" as *const u8 as *const libc::c_char) {
        return;
    }
    strm = compress_init();
    if strm.is_null() {
        return;
    }
    if flag_compress_debug as libc::c_uint
        == COMPRESS_DEBUG_GABI_ZLIB as libc::c_int as libc::c_uint
    {
        compression_header_size = bfd_get_compression_header_size(
            stdoutput,
            0 as *mut asection,
        ) as libc::c_uint;
        header_size = compression_header_size;
    } else {
        compression_header_size = 0 as libc::c_int as libc::c_uint;
        header_size = 12 as libc::c_int as libc::c_uint;
    }
    first_newf = frag_alloc(ob);
    if ({
        let mut __o1: *const obstack = ob;
        ((*__o1).chunk_limit).offset_from((*__o1).next_free) as libc::c_long as size_t
    }) < header_size as libc::c_ulong
    {
        first_newf = frag_alloc(ob);
    }
    if ({
        let mut __o1: *const obstack = ob;
        ((*__o1).chunk_limit).offset_from((*__o1).next_free) as libc::c_long as size_t
    }) < header_size as libc::c_ulong
    {
        as_fatal(
            dcngettext(
                0 as *const libc::c_char,
                b"can't extend frag %lu char\0" as *const u8 as *const libc::c_char,
                b"can't extend frag %lu chars\0" as *const u8 as *const libc::c_char,
                header_size as libc::c_ulong,
                5 as libc::c_int,
            ),
            header_size as libc::c_ulong,
        );
    }
    last_newf = first_newf;
    (*ob).next_free = ((*ob).next_free).offset(header_size as isize);
    (*last_newf).fr_type = rs_fill;
    (*last_newf).fr_fix = header_size as valueT;
    header = ((*last_newf).fr_literal).as_mut_ptr();
    compressed_size = header_size as bfd_size_type;
    f = (*(*seginfo).frchainP).frch_root;
    while !f.is_null() {
        let mut fill_size: offsetT = 0;
        let mut fill_literal: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut count: offsetT = 0;
        let mut out_size: libc::c_int = 0;
        if (*f).fr_type as libc::c_uint == rs_fill as libc::c_int as libc::c_uint
        {} else {
            as_abort(
                b"write.c\0" as *const u8 as *const libc::c_char,
                1502 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 47],
                    &[libc::c_char; 47],
                >(b"void compress_debug(bfd *, asection *, void *)\0"))
                    .as_ptr(),
            );
        };
        if (*f).fr_fix != 0 {
            out_size = compress_frag(
                strm,
                ((*f).fr_literal).as_mut_ptr(),
                (*f).fr_fix as libc::c_int,
                &mut last_newf,
                ob,
            );
            if out_size < 0 as libc::c_int {
                return;
            }
            compressed_size = (compressed_size as libc::c_ulong)
                .wrapping_add(out_size as libc::c_ulong) as bfd_size_type
                as bfd_size_type;
        }
        fill_literal = ((*f).fr_literal).as_mut_ptr().offset((*f).fr_fix as isize);
        fill_size = (*f).fr_var;
        count = (*f).fr_offset;
        if count >= 0 as libc::c_int as libc::c_long {} else {
            as_abort(
                b"write.c\0" as *const u8 as *const libc::c_char,
                1514 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 47],
                    &[libc::c_char; 47],
                >(b"void compress_debug(bfd *, asection *, void *)\0"))
                    .as_ptr(),
            );
        };
        if fill_size != 0 && count != 0 {
            loop {
                let fresh11 = count;
                count = count - 1;
                if !(fresh11 != 0) {
                    break;
                }
                out_size = compress_frag(
                    strm,
                    fill_literal,
                    fill_size as libc::c_int,
                    &mut last_newf,
                    ob,
                );
                if out_size < 0 as libc::c_int {
                    return;
                }
                compressed_size = (compressed_size as libc::c_ulong)
                    .wrapping_add(out_size as libc::c_ulong) as bfd_size_type
                    as bfd_size_type;
            }
        }
        f = (*f).fr_next;
    }
    loop {
        let mut avail_out: libc::c_int = 0;
        let mut next_out: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut out_size_0: libc::c_int = 0;
        avail_out = ({
            let mut __o1: *const obstack = ob;
            ((*__o1).chunk_limit).offset_from((*__o1).next_free) as libc::c_long
                as size_t
        }) as libc::c_int;
        if avail_out <= 0 as libc::c_int {
            let mut newf: *mut fragS = 0 as *mut fragS;
            let mut __o1: *mut obstack = ob;
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
            newf = frag_alloc(ob);
            (*newf).fr_type = rs_fill;
            (*last_newf).fr_next = newf;
            last_newf = newf;
            avail_out = ({
                let mut __o1_0: *const obstack = ob;
                ((*__o1_0).chunk_limit).offset_from((*__o1_0).next_free) as libc::c_long
                    as size_t
            }) as libc::c_int;
        }
        if avail_out <= 0 as libc::c_int {
            as_fatal(
                dcgettext(
                    0 as *const libc::c_char,
                    b"can't extend frag\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
        next_out = (*ob).next_free as *mut libc::c_void as *mut libc::c_char;
        (*ob).next_free = ((*ob).next_free).offset(avail_out as isize);
        x = compress_finish(strm, &mut next_out, &mut avail_out, &mut out_size_0);
        if x < 0 as libc::c_int {
            return;
        }
        (*last_newf)
            .fr_fix = ((*last_newf).fr_fix as libc::c_ulong)
            .wrapping_add(out_size_0 as libc::c_ulong) as valueT as valueT;
        compressed_size = (compressed_size as libc::c_ulong)
            .wrapping_add(out_size_0 as libc::c_ulong) as bfd_size_type as bfd_size_type;
        if avail_out > 0 as libc::c_int {
            (*ob).next_free = ((*ob).next_free).offset(-avail_out as isize);
        }
        if x == 0 as libc::c_int {
            break;
        }
    }
    if compressed_size >= uncompressed_size {
        return;
    }
    (*(*seginfo).frchainP).frch_root = first_newf;
    (*(*seginfo).frchainP).frch_last = last_newf;
    bfd_update_compression_header(abfd, header as *mut bfd_byte, sec);
    x = bfd_set_section_size(sec, compressed_size) as libc::c_int;
    if x != 0 {} else {
        as_abort(
            b"write.c\0" as *const u8 as *const libc::c_char,
            1580 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 47],
                &[libc::c_char; 47],
            >(b"void compress_debug(bfd *, asection *, void *)\0"))
                .as_ptr(),
        );
    };
    if compression_header_size == 0 {
        compressed_name = concat(
            b".z\0" as *const u8 as *const libc::c_char,
            section_name.offset(1 as libc::c_int as isize),
            0 as *mut libc::c_void as *mut libc::c_char,
        );
        bfd_rename_section(sec, compressed_name);
    }
}
unsafe extern "C" fn write_contents(
    mut abfd: *mut bfd,
    mut sec: *mut asection,
    mut xxx: *mut libc::c_void,
) {
    let mut seginfo: *mut segment_info_type = bfd_section_userdata(sec)
        as *mut segment_info_type;
    let mut offset: addressT = 0 as libc::c_int as addressT;
    let mut f: *mut fragS = 0 as *mut fragS;
    if seginfo.is_null()
        || bfd_section_flags(sec) & 0x100 as libc::c_int as libc::c_uint == 0
    {
        return;
    }
    f = (*(*seginfo).frchainP).frch_root;
    while !f.is_null() {
        let mut x: libc::c_int = 0;
        let mut fill_size: addressT = 0;
        let mut fill_literal: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut count: offsetT = 0;
        if (*f).fr_type as libc::c_uint == rs_fill as libc::c_int as libc::c_uint
            || (*f).fr_type as libc::c_uint == rs_fill_nop as libc::c_int as libc::c_uint
        {} else {
            as_abort(
                b"write.c\0" as *const u8 as *const libc::c_char,
                1625 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 47],
                    &[libc::c_char; 47],
                >(b"void write_contents(bfd *, asection *, void *)\0"))
                    .as_ptr(),
            );
        };
        if (*f).fr_fix != 0 {
            x = bfd_set_section_contents(
                stdoutput,
                sec,
                ((*f).fr_literal).as_mut_ptr() as *const libc::c_void,
                offset as file_ptr,
                (*f).fr_fix,
            ) as libc::c_int;
            if x == 0 {
                as_fatal(
                    dcngettext(
                        0 as *const libc::c_char,
                        b"can't write %ld byte to section %s of %s: '%s'\0" as *const u8
                            as *const libc::c_char,
                        b"can't write %ld bytes to section %s of %s: '%s'\0" as *const u8
                            as *const libc::c_char,
                        (*f).fr_fix as libc::c_long as libc::c_ulong,
                        5 as libc::c_int,
                    ),
                    (*f).fr_fix as libc::c_long,
                    bfd_section_name(sec),
                    bfd_get_filename(stdoutput),
                    bfd_errmsg(bfd_get_error()),
                );
            }
            offset = (offset as libc::c_ulong).wrapping_add((*f).fr_fix) as addressT
                as addressT;
        }
        fill_size = (*f).fr_var as addressT;
        count = (*f).fr_offset;
        fill_literal = ((*f).fr_literal).as_mut_ptr().offset((*f).fr_fix as isize);
        if (*f).fr_type as libc::c_uint == rs_fill_nop as libc::c_int as libc::c_uint {
            if count >= 0 as libc::c_int as libc::c_long
                && fill_size == 1 as libc::c_int as libc::c_ulong
            {} else {
                as_abort(
                    b"write.c\0" as *const u8 as *const libc::c_char,
                    1649 as libc::c_int,
                    (*::core::mem::transmute::<
                        &[u8; 47],
                        &[libc::c_char; 47],
                    >(b"void write_contents(bfd *, asection *, void *)\0"))
                        .as_ptr(),
                );
            };
            if count > 0 as libc::c_int as libc::c_long {
                let mut buf: *mut libc::c_char = xmalloc(count as size_t)
                    as *mut libc::c_char;
                i386_generate_nops(f, buf, count, *fill_literal as libc::c_int);
                x = bfd_set_section_contents(
                    stdoutput,
                    sec,
                    buf as *const libc::c_void,
                    offset as file_ptr,
                    count as bfd_size_type,
                ) as libc::c_int;
                if x == 0 {
                    as_fatal(
                        dcngettext(
                            0 as *const libc::c_char,
                            b"can't fill %ld byte in section %s of %s: '%s'\0"
                                as *const u8 as *const libc::c_char,
                            b"can't fill %ld bytes in section %s of %s: '%s'\0"
                                as *const u8 as *const libc::c_char,
                            count as libc::c_ulong,
                            5 as libc::c_int,
                        ),
                        count,
                        bfd_section_name(sec),
                        bfd_get_filename(stdoutput),
                        bfd_errmsg(bfd_get_error()),
                    );
                }
                offset = (offset as libc::c_ulong).wrapping_add(count as libc::c_ulong)
                    as addressT as addressT;
                free(buf as *mut libc::c_void);
            }
        } else {
            if count >= 0 as libc::c_int as libc::c_long {} else {
                as_abort(
                    b"write.c\0" as *const u8 as *const libc::c_char,
                    1673 as libc::c_int,
                    (*::core::mem::transmute::<
                        &[u8; 47],
                        &[libc::c_char; 47],
                    >(b"void write_contents(bfd *, asection *, void *)\0"))
                        .as_ptr(),
                );
            };
            if fill_size != 0 && count != 0 {
                let mut buf_0: [libc::c_char; 256] = [0; 256];
                if fill_size
                    > ::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong
                {
                    loop {
                        let fresh12 = count;
                        count = count - 1;
                        if !(fresh12 != 0) {
                            break;
                        }
                        x = bfd_set_section_contents(
                            stdoutput,
                            sec,
                            fill_literal as *const libc::c_void,
                            offset as file_ptr,
                            fill_size,
                        ) as libc::c_int;
                        if x == 0 {
                            as_fatal(
                                dcngettext(
                                    0 as *const libc::c_char,
                                    b"can't fill %ld byte in section %s of %s: '%s'\0"
                                        as *const u8 as *const libc::c_char,
                                    b"can't fill %ld bytes in section %s of %s: '%s'\0"
                                        as *const u8 as *const libc::c_char,
                                    fill_size as libc::c_long as libc::c_ulong,
                                    5 as libc::c_int,
                                ),
                                fill_size as libc::c_long,
                                bfd_section_name(sec),
                                bfd_get_filename(stdoutput),
                                bfd_errmsg(bfd_get_error()),
                            );
                        }
                        offset = (offset as libc::c_ulong).wrapping_add(fill_size)
                            as addressT as addressT;
                    }
                } else {
                    let mut n_per_buf: libc::c_int = 0;
                    let mut i: libc::c_int = 0;
                    if fill_size == 1 as libc::c_int as libc::c_ulong {
                        n_per_buf = ::core::mem::size_of::<[libc::c_char; 256]>()
                            as libc::c_ulong as libc::c_int;
                        memset(
                            buf_0.as_mut_ptr() as *mut libc::c_void,
                            *fill_literal as libc::c_int,
                            n_per_buf as libc::c_ulong,
                        );
                    } else {
                        let mut bufp: *mut libc::c_char = 0 as *mut libc::c_char;
                        n_per_buf = (::core::mem::size_of::<[libc::c_char; 256]>()
                            as libc::c_ulong)
                            .wrapping_div(fill_size) as libc::c_int;
                        i = n_per_buf;
                        bufp = buf_0.as_mut_ptr();
                        while i != 0 {
                            memcpy(
                                bufp as *mut libc::c_void,
                                fill_literal as *const libc::c_void,
                                fill_size,
                            );
                            i -= 1;
                            i;
                            bufp = bufp.offset(fill_size as isize);
                        }
                    }
                    while count > 0 as libc::c_int as libc::c_long {
                        n_per_buf = (if n_per_buf as libc::c_long > count {
                            count
                        } else {
                            n_per_buf as libc::c_long
                        }) as libc::c_int;
                        x = bfd_set_section_contents(
                            stdoutput,
                            sec,
                            buf_0.as_mut_ptr() as *const libc::c_void,
                            offset as file_ptr,
                            (n_per_buf as bfd_size_type).wrapping_mul(fill_size),
                        ) as libc::c_int;
                        if x == 0 {
                            as_fatal(
                                dcngettext(
                                    0 as *const libc::c_char,
                                    b"can't fill %ld byte in section %s of %s: '%s'\0"
                                        as *const u8 as *const libc::c_char,
                                    b"can't fill %ld bytes in section %s of %s: '%s'\0"
                                        as *const u8 as *const libc::c_char,
                                    (n_per_buf as libc::c_ulong).wrapping_mul(fill_size)
                                        as libc::c_long as libc::c_ulong,
                                    5 as libc::c_int,
                                ),
                                (n_per_buf as libc::c_ulong).wrapping_mul(fill_size)
                                    as libc::c_long,
                                bfd_section_name(sec),
                                bfd_get_filename(stdoutput),
                                bfd_errmsg(bfd_get_error()),
                            );
                        }
                        offset = (offset as libc::c_ulong)
                            .wrapping_add(
                                (n_per_buf as libc::c_ulong).wrapping_mul(fill_size),
                            ) as addressT as addressT;
                        count -= n_per_buf as libc::c_long;
                    }
                }
            }
        }
        f = (*f).fr_next;
    }
}
unsafe extern "C" fn merge_data_into_text() {
    let ref mut fresh13 = (*(*(*(bfd_section_userdata(text_section as *const asection)
        as *mut segment_info_type))
        .frchainP)
        .frch_last)
        .fr_next;
    *fresh13 = (*(*(bfd_section_userdata(data_section as *const asection)
        as *mut segment_info_type))
        .frchainP)
        .frch_root;
    let ref mut fresh14 = (*(*(bfd_section_userdata(text_section as *const asection)
        as *mut segment_info_type))
        .frchainP)
        .frch_last;
    *fresh14 = (*(*(bfd_section_userdata(data_section as *const asection)
        as *mut segment_info_type))
        .frchainP)
        .frch_last;
    let ref mut fresh15 = (*(bfd_section_userdata(data_section as *const asection)
        as *mut segment_info_type))
        .frchainP;
    *fresh15 = 0 as *mut frchainS;
}
unsafe extern "C" fn set_symtab() {
    let mut nsyms: libc::c_int = 0;
    let mut asympp: *mut *mut asymbol = 0 as *mut *mut asymbol;
    let mut symp: *mut symbolS = 0 as *mut symbolS;
    let mut result: bool = false;
    nsyms = 0 as libc::c_int;
    symp = symbol_rootP;
    while !symp.is_null() {
        if bfd_keep_unused_section_symbols(stdoutput) as libc::c_int != 0
            || symbol_section_p(symp) == 0 || symbol_used_in_reloc_p(symp) != 0
        {
            nsyms += 1;
            nsyms;
        }
        symp = symbol_next(symp);
    }
    if nsyms != 0 {
        let mut i: libc::c_int = 0;
        let mut amt: bfd_size_type = (nsyms as bfd_size_type)
            .wrapping_mul(::core::mem::size_of::<*mut asymbol>() as libc::c_ulong);
        asympp = bfd_alloc(stdoutput, amt) as *mut *mut asymbol;
        symp = symbol_rootP;
        i = 0 as libc::c_int;
        while i < nsyms {
            if bfd_keep_unused_section_symbols(stdoutput) as libc::c_int != 0
                || symbol_section_p(symp) == 0 || symbol_used_in_reloc_p(symp) != 0
            {
                let ref mut fresh16 = *asympp.offset(i as isize);
                *fresh16 = symbol_get_bfdsym(symp);
                if (**asympp.offset(i as isize)).flags
                    != ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_uint
                    || !(bfd_is_const_section((**asympp.offset(i as isize)).section)
                        as libc::c_int != 0
                        && (*(**asympp.offset(i as isize)).section).symbol
                            == *asympp.offset(i as isize))
                {
                    let ref mut fresh17 = (**asympp.offset(i as isize)).flags;
                    *fresh17 |= ((1 as libc::c_int) << 5 as libc::c_int) as libc::c_uint;
                }
                symbol_mark_written(symp);
                if symbol_section_p(symp) != 0 {
                    let ref mut fresh18 = (**asympp.offset(i as isize)).flags;
                    *fresh18
                        |= ((1 as libc::c_int) << 24 as libc::c_int) as libc::c_uint;
                }
                i += 1;
                i;
            }
            symp = symbol_next(symp);
        }
    } else {
        asympp = 0 as *mut *mut asymbol;
    }
    result = bfd_set_symtab(stdoutput, asympp, nsyms as libc::c_uint);
    if result as libc::c_int != 0 {} else {
        as_abort(
            b"write.c\0" as *const u8 as *const libc::c_char,
            1795 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 22],
                &[libc::c_char; 22],
            >(b"void set_symtab(void)\0"))
                .as_ptr(),
        );
    };
    symbol_table_frozen = 1 as libc::c_int;
}
unsafe extern "C" fn subsegs_finish_section(mut s: *mut asection) {
    let mut frchainP: *mut frchain = 0 as *mut frchain;
    let mut seginfo: *mut segment_info_type = bfd_section_userdata(s)
        as *mut segment_info_type;
    if seginfo.is_null() {
        return;
    }
    frchainP = (*seginfo).frchainP;
    while !frchainP.is_null() {
        let mut alignment: libc::c_int = 0;
        subseg_set(s, (*frchainP).frch_subseg);
        if had_errors() != 0 {
            do_not_pad_sections_to_alignment = 1 as libc::c_int;
        }
        alignment = 0 as libc::c_int;
        if bfd_section_flags(now_seg as *const asection)
            & 0x800000 as libc::c_int as libc::c_uint != 0 && (*now_seg).entsize != 0
        {
            let mut entsize: libc::c_uint = (*now_seg).entsize;
            let mut entalign: libc::c_int = 0 as libc::c_int;
            while entsize & 1 as libc::c_int as libc::c_uint
                == 0 as libc::c_int as libc::c_uint
            {
                entalign += 1;
                entalign;
                entsize >>= 1 as libc::c_int;
            }
            if entalign > alignment {
                alignment = entalign;
            }
        }
        if subseg_text_p(now_seg) != 0 {
            frag_align_code(alignment, 0 as libc::c_int);
        } else {
            frag_align(alignment, 0 as libc::c_int, 0 as libc::c_int);
        }
        frag_wane(frag_now);
        (*frag_now).fr_fix = 0 as libc::c_int as valueT;
        frchainP = (*frchainP).frch_next;
    }
}
unsafe extern "C" fn subsegs_finish() {
    let mut s: *mut asection = 0 as *mut asection;
    s = (*stdoutput).sections;
    while !s.is_null() {
        subsegs_finish_section(s);
        s = (*s).next;
    }
}
unsafe extern "C" fn create_obj_attrs_section() {
    let mut s: segT = 0 as *mut asection;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut size: offsetT = 0;
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    size = bfd_elf_obj_attr_size(stdoutput) as offsetT;
    if size == 0 as libc::c_int as libc::c_long {
        return;
    }
    name = (*((*(*stdoutput).xvec).backend_data as *const elf_backend_data))
        .obj_attrs_section;
    if name.is_null() {
        name = b".gnu.attributes\0" as *const u8 as *const libc::c_char;
    }
    s = subseg_new(name, 0 as libc::c_int);
    (*((*s).used_by_bfd as *mut bfd_elf_section_data))
        .this_hdr
        .sh_type = (*((*(*stdoutput).xvec).backend_data as *const elf_backend_data))
        .obj_attrs_section_type;
    bfd_set_section_flags(s, (0x8 as libc::c_int | 0x20 as libc::c_int) as flagword);
    frag_now_fix();
    p = frag_more(size as size_t);
    bfd_elf_set_obj_attr_contents(stdoutput, p as *mut bfd_byte, size as bfd_vma);
    subsegs_finish_section(s);
    relax_segment(
        (*(*(bfd_section_userdata(s as *const asection) as *mut segment_info_type))
            .frchainP)
            .frch_root,
        s,
        0 as libc::c_int,
    );
    size_seg(stdoutput, s, 0 as *mut libc::c_void);
}
unsafe extern "C" fn create_note_reloc(
    mut sec: segT,
    mut sym: *mut symbolS,
    mut note_offset: bfd_size_type,
    mut desc2_offset: bfd_size_type,
    mut desc2_size: offsetT,
    mut reloc_type: libc::c_int,
    mut addend: bfd_vma,
    mut note: *mut libc::c_char,
) {
    let mut reloc: *mut reloc_list = 0 as *mut reloc_list;
    reloc = xmalloc(::core::mem::size_of::<reloc_list>() as libc::c_ulong)
        as *mut reloc_list;
    (*reloc).u.b.sec = sec;
    (*reloc).u.b.s = symbol_get_bfdsym(sym);
    (*reloc).u.b.r.sym_ptr_ptr = &mut (*reloc).u.b.s;
    (*reloc).u.b.r.address = note_offset.wrapping_add(desc2_offset);
    (*reloc).u.b.r.addend = addend;
    (*reloc)
        .u
        .b
        .r
        .howto = bfd_reloc_type_lookup(
        stdoutput,
        reloc_type as bfd_reloc_code_real_type,
    );
    if ((*reloc).u.b.r.howto).is_null() {
        as_bad(
            dcgettext(
                0 as *const libc::c_char,
                b"unable to create reloc for build note\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return;
    }
    (*reloc).file = b"<gnu build note>\0" as *const u8 as *const libc::c_char;
    (*reloc).line = 0 as libc::c_int as libc::c_uint;
    (*reloc).next = reloc_list;
    reloc_list = reloc;
    if (*sec).use_rela_p() == 0
        || !(strstr(
            bfd_get_target(stdoutput),
            b"-sh\0" as *const u8 as *const libc::c_char,
        ))
            .is_null()
    {
        let mut i: offsetT = 0;
        (*reloc).u.b.r.addend = 0 as libc::c_int as bfd_vma;
        if target_big_endian != 0 {
            i = desc2_size;
            while addend != 0 as libc::c_int as libc::c_ulong
                && i > 0 as libc::c_int as libc::c_long
            {
                *note
                    .offset(
                        desc2_offset
                            .wrapping_add(i as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                    ) = (addend & 0xff as libc::c_int as libc::c_ulong) as libc::c_char;
                addend >>= 8 as libc::c_int;
                i -= 1;
                i;
            }
        } else {
            i = 0 as libc::c_int as offsetT;
            while addend != 0 as libc::c_int as libc::c_ulong && i < desc2_size {
                *note
                    .offset(
                        desc2_offset.wrapping_add(i as libc::c_ulong) as isize,
                    ) = (addend & 0xff as libc::c_int as libc::c_ulong) as libc::c_char;
                addend >>= 8 as libc::c_int;
                i += 1;
                i;
            }
        }
    }
}
unsafe extern "C" fn maybe_generate_build_notes() {
    let mut sec: segT = 0 as *mut asection;
    let mut note: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut note_size: offsetT = 0;
    let mut total_size: offsetT = 0;
    let mut desc_size: offsetT = 0;
    let mut desc2_offset: offsetT = 0;
    let mut desc_reloc: libc::c_int = 0;
    let mut sym: *mut symbolS = 0 as *mut symbolS;
    let mut bsym: *mut asymbol = 0 as *mut asymbol;
    if !flag_generate_build_notes
        || !(bfd_get_section_by_name(
            stdoutput,
            b".gnu.build.attributes\0" as *const u8 as *const libc::c_char,
        ))
            .is_null()
    {
        return;
    }
    sec = subseg_new(
        b".gnu.build.attributes\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
    );
    (*((*sec).used_by_bfd as *mut bfd_elf_section_data))
        .this_hdr
        .sh_type = 7 as libc::c_int as libc::c_uint;
    bfd_set_section_flags(
        sec,
        (0x8 as libc::c_int | 0x100 as libc::c_int | 0x20 as libc::c_int
            | (if 1 as libc::c_int != 0 {
                0x40000000 as libc::c_int
            } else {
                0 as libc::c_int
            })) as flagword,
    );
    bfd_set_section_alignment(sec, 2 as libc::c_int as libc::c_uint);
    if bfd_arch_bits_per_address(stdoutput) <= 32 as libc::c_int as libc::c_uint {
        note_size = 28 as libc::c_int as offsetT;
        desc_size = 8 as libc::c_int as offsetT;
        desc2_offset = 24 as libc::c_int as offsetT;
        if !(strstr(
            bfd_get_target(stdoutput),
            b"-crx\0" as *const u8 as *const libc::c_char,
        ))
            .is_null()
        {
            desc_reloc = BFD_RELOC_CRX_NUM32 as libc::c_int;
        } else if !(strstr(
            bfd_get_target(stdoutput),
            b"-cr16\0" as *const u8 as *const libc::c_char,
        ))
            .is_null()
        {
            desc_reloc = BFD_RELOC_CR16_NUM32 as libc::c_int;
        } else {
            desc_reloc = BFD_RELOC_32 as libc::c_int;
        }
    } else {
        note_size = 36 as libc::c_int as offsetT;
        desc_size = 16 as libc::c_int as offsetT;
        desc2_offset = 28 as libc::c_int as offsetT;
        if !(strstr(
            bfd_get_target(stdoutput),
            b"-ia64\0" as *const u8 as *const libc::c_char,
        ))
            .is_null()
        {
            desc_reloc = if target_big_endian != 0 {
                BFD_RELOC_IA64_DIR32MSB as libc::c_int
            } else {
                BFD_RELOC_IA64_DIR32LSB as libc::c_int
            };
        } else if !(strstr(
            bfd_get_target(stdoutput),
            b"-hppa\0" as *const u8 as *const libc::c_char,
        ))
            .is_null()
        {
            desc_reloc = 80 as libc::c_int;
        } else {
            desc_reloc = BFD_RELOC_64 as libc::c_int;
        }
    }
    total_size = 0 as libc::c_int as offsetT;
    note = 0 as *mut libc::c_char;
    sym = symbol_rootP;
    while !sym.is_null() {
        bsym = symbol_get_bfdsym(sym);
        if !bsym.is_null()
            && (*bsym).flags & ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_uint
                != 0 && !((*bsym).section).is_null()
            && (*(*bsym).section).flags
                & (0x10 as libc::c_int | 0x20000 as libc::c_int) as libc::c_uint
                == 0x10 as libc::c_int as libc::c_uint
            && !startswith(
                S_GET_NAME(sym),
                b".gnu.linkonce\0" as *const u8 as *const libc::c_char,
            )
        {
            frag_now_fix();
            note = frag_more(note_size as size_t);
            memset(
                note as *mut libc::c_void,
                0 as libc::c_int,
                note_size as libc::c_ulong,
            );
            if target_big_endian != 0 {
                *note
                    .offset(
                        3 as libc::c_int as isize,
                    ) = 8 as libc::c_int as libc::c_char;
                *note.offset(7 as libc::c_int as isize) = desc_size as libc::c_char;
                *note
                    .offset(
                        10 as libc::c_int as isize,
                    ) = (0x100 as libc::c_int >> 8 as libc::c_int) as libc::c_char;
                *note
                    .offset(
                        11 as libc::c_int as isize,
                    ) = (0x100 as libc::c_int & 0xff as libc::c_int) as libc::c_char;
            } else {
                *note
                    .offset(
                        0 as libc::c_int as isize,
                    ) = 8 as libc::c_int as libc::c_char;
                *note.offset(4 as libc::c_int as isize) = desc_size as libc::c_char;
                *note
                    .offset(
                        8 as libc::c_int as isize,
                    ) = (0x100 as libc::c_int & 0xff as libc::c_int) as libc::c_char;
                *note
                    .offset(
                        9 as libc::c_int as isize,
                    ) = (0x100 as libc::c_int >> 8 as libc::c_int) as libc::c_char;
            }
            memcpy(
                note.offset(12 as libc::c_int as isize) as *mut libc::c_void,
                b"GA$\x013a1\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                8 as libc::c_int as libc::c_ulong,
            );
            create_note_reloc(
                sec,
                sym,
                total_size as bfd_size_type,
                20 as libc::c_int as bfd_size_type,
                desc_size / 2 as libc::c_int as libc::c_long,
                desc_reloc,
                0 as libc::c_int as bfd_vma,
                note,
            );
            create_note_reloc(
                sec,
                sym,
                total_size as bfd_size_type,
                desc2_offset as bfd_size_type,
                desc_size / 2 as libc::c_int as libc::c_long,
                desc_reloc,
                bfd_section_size((*bsym).section),
                note,
            );
            symbol_mark_used_in_reloc(sym);
            total_size += note_size;
        }
        sym = symbol_next(sym);
    }
    if total_size != 0 {
        bfd_set_section_contents(
            stdoutput,
            sec,
            note as *mut bfd_byte as *const libc::c_void,
            0 as libc::c_int as file_ptr,
            total_size as bfd_size_type,
        );
    }
    subsegs_finish_section(sec);
    relax_segment(
        (*(*(bfd_section_userdata(sec as *const asection) as *mut segment_info_type))
            .frchainP)
            .frch_root,
        sec,
        0 as libc::c_int,
    );
    size_seg(stdoutput, sec, 0 as *mut libc::c_void);
}
unsafe extern "C" fn relax_align(
    mut address: relax_addressT,
    mut alignment: libc::c_int,
) -> relax_addressT {
    let mut mask: relax_addressT = 0;
    let mut new_address: relax_addressT = 0;
    mask = !((!(0 as libc::c_int) as relax_addressT) << alignment);
    new_address = address.wrapping_add(mask) & !mask;
    return new_address.wrapping_sub(address);
}
