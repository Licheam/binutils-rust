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
    pub type xcoff_tdata;
    pub type pe_tdata;
    pub type coff_tdata;
    pub type artdata;
    pub type aout_data_struct;
    pub type bfd_link_hash_table;
    pub type bfd_link_order;
    pub type bfd_iovec;
    pub type bfd_link_info;
    pub type bfd_link_hash_entry;
    pub type flag_info;
    pub type _bfd_window_internal;
    pub type demangle_operator_info;
    pub type demangle_builtin_type_info;
    pub type debug_type_s;
    pub type debug_field_s;
    pub type debug_baseclass_s;
    pub type debug_method_s;
    pub type debug_method_variant_s;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
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
    fn abort() -> !;
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
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __errno_location() -> *mut libc::c_int;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn concat(_: *const libc::c_char, _: ...) -> *mut libc::c_char;
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    fn xrealloc(_: *mut libc::c_void, _: size_t) -> *mut libc::c_void;
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
    static _sch_istable: [libc::c_ushort; 256];
    fn cplus_demangle(
        mangled: *const libc::c_char,
        options: libc::c_int,
    ) -> *mut libc::c_char;
    fn cplus_demangle_v3_components(
        mangled: *const libc::c_char,
        options: libc::c_int,
        mem: *mut *mut libc::c_void,
    ) -> *mut demangle_component;
    fn cplus_demangle_print(
        options: libc::c_int,
        tree: *mut demangle_component,
        estimated_length: libc::c_int,
        p_allocated_size: *mut size_t,
    ) -> *mut libc::c_char;
    fn debug_set_filename(_: *mut libc::c_void, _: *const libc::c_char) -> bool;
    fn debug_start_source(_: *mut libc::c_void, _: *const libc::c_char) -> bool;
    fn debug_record_function(
        _: *mut libc::c_void,
        _: *const libc::c_char,
        _: debug_type,
        _: bool,
        _: bfd_vma,
    ) -> bool;
    fn debug_record_parameter(
        _: *mut libc::c_void,
        _: *const libc::c_char,
        _: debug_type,
        _: debug_parm_kind,
        _: bfd_vma,
    ) -> bool;
    fn debug_end_function(_: *mut libc::c_void, _: bfd_vma) -> bool;
    fn debug_start_block(_: *mut libc::c_void, _: bfd_vma) -> bool;
    fn debug_end_block(_: *mut libc::c_void, _: bfd_vma) -> bool;
    fn debug_record_line(_: *mut libc::c_void, _: libc::c_ulong, _: bfd_vma) -> bool;
    fn debug_start_common_block(_: *mut libc::c_void, _: *const libc::c_char) -> bool;
    fn debug_end_common_block(_: *mut libc::c_void, _: *const libc::c_char) -> bool;
    fn debug_record_int_const(
        _: *mut libc::c_void,
        _: *const libc::c_char,
        _: bfd_vma,
    ) -> bool;
    fn debug_record_float_const(
        _: *mut libc::c_void,
        _: *const libc::c_char,
        _: libc::c_double,
    ) -> bool;
    fn debug_record_typed_const(
        _: *mut libc::c_void,
        _: *const libc::c_char,
        _: debug_type,
        _: bfd_vma,
    ) -> bool;
    fn debug_record_label(
        _: *mut libc::c_void,
        _: *const libc::c_char,
        _: debug_type,
        _: bfd_vma,
    ) -> bool;
    fn debug_record_variable(
        _: *mut libc::c_void,
        _: *const libc::c_char,
        _: debug_type,
        _: debug_var_kind,
        _: bfd_vma,
    ) -> bool;
    fn debug_make_indirect_type(
        _: *mut libc::c_void,
        _: *mut debug_type,
        _: *const libc::c_char,
    ) -> debug_type;
    fn debug_make_void_type(_: *mut libc::c_void) -> debug_type;
    fn debug_make_int_type(_: *mut libc::c_void, _: libc::c_uint, _: bool) -> debug_type;
    fn debug_make_float_type(_: *mut libc::c_void, _: libc::c_uint) -> debug_type;
    fn debug_make_bool_type(_: *mut libc::c_void, _: libc::c_uint) -> debug_type;
    fn debug_make_complex_type(_: *mut libc::c_void, _: libc::c_uint) -> debug_type;
    fn debug_make_struct_type(
        _: *mut libc::c_void,
        _: bool,
        _: bfd_vma,
        _: *mut debug_field,
    ) -> debug_type;
    fn debug_make_object_type(
        _: *mut libc::c_void,
        _: bool,
        _: bfd_vma,
        _: *mut debug_field,
        _: *mut debug_baseclass,
        _: *mut debug_method,
        _: debug_type,
        _: bool,
    ) -> debug_type;
    fn debug_make_enum_type(
        _: *mut libc::c_void,
        _: *mut *const libc::c_char,
        _: *mut bfd_signed_vma,
    ) -> debug_type;
    fn debug_make_pointer_type(_: *mut libc::c_void, _: debug_type) -> debug_type;
    fn debug_make_function_type(
        _: *mut libc::c_void,
        _: debug_type,
        _: *mut debug_type,
        _: bool,
    ) -> debug_type;
    fn debug_make_reference_type(_: *mut libc::c_void, _: debug_type) -> debug_type;
    fn debug_make_range_type(
        _: *mut libc::c_void,
        _: debug_type,
        _: bfd_signed_vma,
        _: bfd_signed_vma,
    ) -> debug_type;
    fn debug_make_array_type(
        _: *mut libc::c_void,
        _: debug_type,
        _: debug_type,
        _: bfd_signed_vma,
        _: bfd_signed_vma,
        _: bool,
    ) -> debug_type;
    fn debug_make_set_type(_: *mut libc::c_void, _: debug_type, _: bool) -> debug_type;
    fn debug_make_offset_type(
        _: *mut libc::c_void,
        _: debug_type,
        _: debug_type,
    ) -> debug_type;
    fn debug_make_method_type(
        _: *mut libc::c_void,
        _: debug_type,
        _: debug_type,
        _: *mut debug_type,
        _: bool,
    ) -> debug_type;
    fn debug_make_const_type(_: *mut libc::c_void, _: debug_type) -> debug_type;
    fn debug_make_volatile_type(_: *mut libc::c_void, _: debug_type) -> debug_type;
    fn debug_make_undefined_tagged_type(
        _: *mut libc::c_void,
        _: *const libc::c_char,
        _: debug_type_kind,
    ) -> debug_type;
    fn debug_make_baseclass(
        _: *mut libc::c_void,
        _: debug_type,
        _: bfd_vma,
        _: bool,
        _: debug_visibility,
    ) -> debug_baseclass;
    fn debug_make_field(
        _: *mut libc::c_void,
        _: *const libc::c_char,
        _: debug_type,
        _: bfd_vma,
        _: bfd_vma,
        _: debug_visibility,
    ) -> debug_field;
    fn debug_make_static_member(
        _: *mut libc::c_void,
        _: *const libc::c_char,
        _: debug_type,
        _: *const libc::c_char,
        _: debug_visibility,
    ) -> debug_field;
    fn debug_make_method(
        _: *mut libc::c_void,
        _: *const libc::c_char,
        _: *mut debug_method_variant,
    ) -> debug_method;
    fn debug_make_method_variant(
        _: *mut libc::c_void,
        _: *const libc::c_char,
        _: debug_type,
        _: debug_visibility,
        _: bool,
        _: bool,
        _: bfd_vma,
        _: debug_type,
    ) -> debug_method_variant;
    fn debug_make_static_method_variant(
        _: *mut libc::c_void,
        _: *const libc::c_char,
        _: debug_type,
        _: debug_visibility,
        _: bool,
        _: bool,
    ) -> debug_method_variant;
    fn debug_name_type(
        _: *mut libc::c_void,
        _: *const libc::c_char,
        _: debug_type,
    ) -> debug_type;
    fn debug_tag_type(
        _: *mut libc::c_void,
        _: *const libc::c_char,
        _: debug_type,
    ) -> debug_type;
    fn debug_record_type_size(
        _: *mut libc::c_void,
        _: debug_type,
        _: libc::c_uint,
    ) -> bool;
    fn debug_find_named_type(_: *mut libc::c_void, _: *const libc::c_char) -> debug_type;
    fn debug_find_tagged_type(
        _: *mut libc::c_void,
        _: *const libc::c_char,
        _: debug_type_kind,
    ) -> debug_type;
    fn debug_get_type_kind(_: *mut libc::c_void, _: debug_type) -> debug_type_kind;
    fn debug_get_type_name(_: *mut libc::c_void, _: debug_type) -> *const libc::c_char;
    fn debug_get_return_type(_: *mut libc::c_void, _: debug_type) -> debug_type;
    fn debug_get_parameter_types(
        _: *mut libc::c_void,
        _: debug_type,
        _: *mut bool,
    ) -> *const debug_type;
    fn debug_get_fields(_: *mut libc::c_void, _: debug_type) -> *const debug_field;
    fn debug_get_field_type(_: *mut libc::c_void, _: debug_field) -> debug_type;
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
    pub link: C2RustUnnamed_0,
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
pub union C2RustUnnamed_0 {
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bfd_symbol {
    pub the_bfd: *mut bfd,
    pub name: *const libc::c_char,
    pub value: symvalue,
    pub flags: flagword,
    pub section: *mut bfd_section,
    pub udata: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub p: *mut libc::c_void,
    pub i: bfd_vma,
}
pub type bfd_vma = libc::c_ulong;
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
    pub map_head: C2RustUnnamed_2,
    pub map_tail: C2RustUnnamed_2,
    pub already_assigned: *mut bfd_section,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub link_order: *mut bfd_link_order,
    pub s: *mut bfd_section,
    pub linked_to_symbol_name: *const libc::c_char,
}
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
pub type asection = bfd_section;
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
    pub u: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub sym: *mut bfd_symbol,
    pub offset: bfd_vma,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct relax_table {
    pub addr: bfd_vma,
    pub size: libc::c_int,
}
pub type flagword = libc::c_uint;
pub type symvalue = libc::c_ulong;
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
pub type sec_ptr = *mut bfd_section;
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
pub type asymbol = bfd_symbol;
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
    pub u: C2RustUnnamed_4,
    pub namidx: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
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
pub type bfd_signed_vma = libc::c_long;
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
pub type C2RustUnnamed_5 = libc::c_uint;
pub const _sch_isbasic: C2RustUnnamed_5 = 3088;
pub const _sch_iscppsp: C2RustUnnamed_5 = 3072;
pub const _sch_isgraph: C2RustUnnamed_5 = 172;
pub const _sch_isidnum: C2RustUnnamed_5 = 516;
pub const _sch_isalnum: C2RustUnnamed_5 = 140;
pub const _sch_isalpha: C2RustUnnamed_5 = 136;
pub const _sch_isnvsp: C2RustUnnamed_5 = 2048;
pub const _sch_isvsp: C2RustUnnamed_5 = 1024;
pub const _sch_isidst: C2RustUnnamed_5 = 512;
pub const _sch_isxdigit: C2RustUnnamed_5 = 256;
pub const _sch_isupper: C2RustUnnamed_5 = 128;
pub const _sch_isspace: C2RustUnnamed_5 = 64;
pub const _sch_ispunct: C2RustUnnamed_5 = 32;
pub const _sch_isprint: C2RustUnnamed_5 = 16;
pub const _sch_islower: C2RustUnnamed_5 = 8;
pub const _sch_isdigit: C2RustUnnamed_5 = 4;
pub const _sch_iscntrl: C2RustUnnamed_5 = 2;
pub const _sch_isblank: C2RustUnnamed_5 = 1;
pub type gnu_v3_ctor_kinds = libc::c_uint;
pub const gnu_v3_object_ctor_group: gnu_v3_ctor_kinds = 5;
pub const gnu_v3_unified_ctor: gnu_v3_ctor_kinds = 4;
pub const gnu_v3_complete_object_allocating_ctor: gnu_v3_ctor_kinds = 3;
pub const gnu_v3_base_object_ctor: gnu_v3_ctor_kinds = 2;
pub const gnu_v3_complete_object_ctor: gnu_v3_ctor_kinds = 1;
pub type gnu_v3_dtor_kinds = libc::c_uint;
pub const gnu_v3_object_dtor_group: gnu_v3_dtor_kinds = 5;
pub const gnu_v3_unified_dtor: gnu_v3_dtor_kinds = 4;
pub const gnu_v3_base_object_dtor: gnu_v3_dtor_kinds = 3;
pub const gnu_v3_complete_object_dtor: gnu_v3_dtor_kinds = 2;
pub const gnu_v3_deleting_dtor: gnu_v3_dtor_kinds = 1;
pub type demangle_component_type = libc::c_uint;
pub const DEMANGLE_COMPONENT_THROW_SPEC: demangle_component_type = 81;
pub const DEMANGLE_COMPONENT_NOEXCEPT: demangle_component_type = 80;
pub const DEMANGLE_COMPONENT_CLONE: demangle_component_type = 79;
pub const DEMANGLE_COMPONENT_TRANSACTION_SAFE: demangle_component_type = 78;
pub const DEMANGLE_COMPONENT_TAGGED_NAME: demangle_component_type = 77;
pub const DEMANGLE_COMPONENT_PACK_EXPANSION: demangle_component_type = 76;
pub const DEMANGLE_COMPONENT_NONTRANSACTION_CLONE: demangle_component_type = 75;
pub const DEMANGLE_COMPONENT_TRANSACTION_CLONE: demangle_component_type = 74;
pub const DEMANGLE_COMPONENT_UNNAMED_TYPE: demangle_component_type = 73;
pub const DEMANGLE_COMPONENT_DEFAULT_ARG: demangle_component_type = 72;
pub const DEMANGLE_COMPONENT_LAMBDA: demangle_component_type = 71;
pub const DEMANGLE_COMPONENT_GLOBAL_DESTRUCTORS: demangle_component_type = 70;
pub const DEMANGLE_COMPONENT_GLOBAL_CONSTRUCTORS: demangle_component_type = 69;
pub const DEMANGLE_COMPONENT_DECLTYPE: demangle_component_type = 68;
pub const DEMANGLE_COMPONENT_NUMBER: demangle_component_type = 67;
pub const DEMANGLE_COMPONENT_CHARACTER: demangle_component_type = 66;
pub const DEMANGLE_COMPONENT_COMPOUND_NAME: demangle_component_type = 65;
pub const DEMANGLE_COMPONENT_JAVA_RESOURCE: demangle_component_type = 64;
pub const DEMANGLE_COMPONENT_VENDOR_EXPR: demangle_component_type = 63;
pub const DEMANGLE_COMPONENT_LITERAL_NEG: demangle_component_type = 62;
pub const DEMANGLE_COMPONENT_LITERAL: demangle_component_type = 61;
pub const DEMANGLE_COMPONENT_TRINARY_ARG2: demangle_component_type = 60;
pub const DEMANGLE_COMPONENT_TRINARY_ARG1: demangle_component_type = 59;
pub const DEMANGLE_COMPONENT_TRINARY: demangle_component_type = 58;
pub const DEMANGLE_COMPONENT_BINARY_ARGS: demangle_component_type = 57;
pub const DEMANGLE_COMPONENT_BINARY: demangle_component_type = 56;
pub const DEMANGLE_COMPONENT_UNARY: demangle_component_type = 55;
pub const DEMANGLE_COMPONENT_NULLARY: demangle_component_type = 54;
pub const DEMANGLE_COMPONENT_CONVERSION: demangle_component_type = 53;
pub const DEMANGLE_COMPONENT_CAST: demangle_component_type = 52;
pub const DEMANGLE_COMPONENT_EXTENDED_OPERATOR: demangle_component_type = 51;
pub const DEMANGLE_COMPONENT_OPERATOR: demangle_component_type = 50;
pub const DEMANGLE_COMPONENT_INITIALIZER_LIST: demangle_component_type = 49;
pub const DEMANGLE_COMPONENT_TPARM_OBJ: demangle_component_type = 48;
pub const DEMANGLE_COMPONENT_TEMPLATE_ARGLIST: demangle_component_type = 47;
pub const DEMANGLE_COMPONENT_ARGLIST: demangle_component_type = 46;
pub const DEMANGLE_COMPONENT_VECTOR_TYPE: demangle_component_type = 45;
pub const DEMANGLE_COMPONENT_FIXED_TYPE: demangle_component_type = 44;
pub const DEMANGLE_COMPONENT_PTRMEM_TYPE: demangle_component_type = 43;
pub const DEMANGLE_COMPONENT_ARRAY_TYPE: demangle_component_type = 42;
pub const DEMANGLE_COMPONENT_FUNCTION_TYPE: demangle_component_type = 41;
pub const DEMANGLE_COMPONENT_VENDOR_TYPE: demangle_component_type = 40;
pub const DEMANGLE_COMPONENT_BUILTIN_TYPE: demangle_component_type = 39;
pub const DEMANGLE_COMPONENT_IMAGINARY: demangle_component_type = 38;
pub const DEMANGLE_COMPONENT_COMPLEX: demangle_component_type = 37;
pub const DEMANGLE_COMPONENT_RVALUE_REFERENCE: demangle_component_type = 36;
pub const DEMANGLE_COMPONENT_REFERENCE: demangle_component_type = 35;
pub const DEMANGLE_COMPONENT_POINTER: demangle_component_type = 34;
pub const DEMANGLE_COMPONENT_VENDOR_TYPE_QUAL: demangle_component_type = 33;
pub const DEMANGLE_COMPONENT_RVALUE_REFERENCE_THIS: demangle_component_type = 32;
pub const DEMANGLE_COMPONENT_REFERENCE_THIS: demangle_component_type = 31;
pub const DEMANGLE_COMPONENT_CONST_THIS: demangle_component_type = 30;
pub const DEMANGLE_COMPONENT_VOLATILE_THIS: demangle_component_type = 29;
pub const DEMANGLE_COMPONENT_RESTRICT_THIS: demangle_component_type = 28;
pub const DEMANGLE_COMPONENT_CONST: demangle_component_type = 27;
pub const DEMANGLE_COMPONENT_VOLATILE: demangle_component_type = 26;
pub const DEMANGLE_COMPONENT_RESTRICT: demangle_component_type = 25;
pub const DEMANGLE_COMPONENT_SUB_STD: demangle_component_type = 24;
pub const DEMANGLE_COMPONENT_HIDDEN_ALIAS: demangle_component_type = 23;
pub const DEMANGLE_COMPONENT_REFTEMP: demangle_component_type = 22;
pub const DEMANGLE_COMPONENT_TLS_WRAPPER: demangle_component_type = 21;
pub const DEMANGLE_COMPONENT_TLS_INIT: demangle_component_type = 20;
pub const DEMANGLE_COMPONENT_GUARD: demangle_component_type = 19;
pub const DEMANGLE_COMPONENT_JAVA_CLASS: demangle_component_type = 18;
pub const DEMANGLE_COMPONENT_COVARIANT_THUNK: demangle_component_type = 17;
pub const DEMANGLE_COMPONENT_VIRTUAL_THUNK: demangle_component_type = 16;
pub const DEMANGLE_COMPONENT_THUNK: demangle_component_type = 15;
pub const DEMANGLE_COMPONENT_TYPEINFO_FN: demangle_component_type = 14;
pub const DEMANGLE_COMPONENT_TYPEINFO_NAME: demangle_component_type = 13;
pub const DEMANGLE_COMPONENT_TYPEINFO: demangle_component_type = 12;
pub const DEMANGLE_COMPONENT_CONSTRUCTION_VTABLE: demangle_component_type = 11;
pub const DEMANGLE_COMPONENT_VTT: demangle_component_type = 10;
pub const DEMANGLE_COMPONENT_VTABLE: demangle_component_type = 9;
pub const DEMANGLE_COMPONENT_DTOR: demangle_component_type = 8;
pub const DEMANGLE_COMPONENT_CTOR: demangle_component_type = 7;
pub const DEMANGLE_COMPONENT_FUNCTION_PARAM: demangle_component_type = 6;
pub const DEMANGLE_COMPONENT_TEMPLATE_PARAM: demangle_component_type = 5;
pub const DEMANGLE_COMPONENT_TEMPLATE: demangle_component_type = 4;
pub const DEMANGLE_COMPONENT_TYPED_NAME: demangle_component_type = 3;
pub const DEMANGLE_COMPONENT_LOCAL_NAME: demangle_component_type = 2;
pub const DEMANGLE_COMPONENT_QUAL_NAME: demangle_component_type = 1;
pub const DEMANGLE_COMPONENT_NAME: demangle_component_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct demangle_component {
    pub type_0: demangle_component_type,
    pub d_printing: libc::c_int,
    pub d_counting: libc::c_int,
    pub u: C2RustUnnamed_6,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
    pub s_name: C2RustUnnamed_18,
    pub s_operator: C2RustUnnamed_17,
    pub s_extended_operator: C2RustUnnamed_16,
    pub s_fixed: C2RustUnnamed_15,
    pub s_ctor: C2RustUnnamed_14,
    pub s_dtor: C2RustUnnamed_13,
    pub s_builtin: C2RustUnnamed_12,
    pub s_string: C2RustUnnamed_11,
    pub s_number: C2RustUnnamed_10,
    pub s_character: C2RustUnnamed_9,
    pub s_binary: C2RustUnnamed_8,
    pub s_unary_num: C2RustUnnamed_7,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub sub: *mut demangle_component,
    pub num: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub left: *mut demangle_component,
    pub right: *mut demangle_component,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub character: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub number: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub string: *const libc::c_char,
    pub len: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_12 {
    pub type_0: *const demangle_builtin_type_info,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_13 {
    pub kind: gnu_v3_dtor_kinds,
    pub name: *mut demangle_component,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_14 {
    pub kind: gnu_v3_ctor_kinds,
    pub name: *mut demangle_component,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_15 {
    pub length: *mut demangle_component,
    pub accum: libc::c_short,
    pub sat: libc::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_16 {
    pub args: libc::c_int,
    pub name: *mut demangle_component,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_17 {
    pub op: *const demangle_operator_info,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_18 {
    pub s: *const libc::c_char,
    pub len: libc::c_int,
}
pub type debug_type_kind = libc::c_uint;
pub const DEBUG_KIND_TAGGED: debug_type_kind = 23;
pub const DEBUG_KIND_NAMED: debug_type_kind = 22;
pub const DEBUG_KIND_VOLATILE: debug_type_kind = 21;
pub const DEBUG_KIND_CONST: debug_type_kind = 20;
pub const DEBUG_KIND_METHOD: debug_type_kind = 19;
pub const DEBUG_KIND_OFFSET: debug_type_kind = 18;
pub const DEBUG_KIND_SET: debug_type_kind = 17;
pub const DEBUG_KIND_ARRAY: debug_type_kind = 16;
pub const DEBUG_KIND_RANGE: debug_type_kind = 15;
pub const DEBUG_KIND_REFERENCE: debug_type_kind = 14;
pub const DEBUG_KIND_FUNCTION: debug_type_kind = 13;
pub const DEBUG_KIND_POINTER: debug_type_kind = 12;
pub const DEBUG_KIND_ENUM: debug_type_kind = 11;
pub const DEBUG_KIND_UNION_CLASS: debug_type_kind = 10;
pub const DEBUG_KIND_CLASS: debug_type_kind = 9;
pub const DEBUG_KIND_UNION: debug_type_kind = 8;
pub const DEBUG_KIND_STRUCT: debug_type_kind = 7;
pub const DEBUG_KIND_BOOL: debug_type_kind = 6;
pub const DEBUG_KIND_COMPLEX: debug_type_kind = 5;
pub const DEBUG_KIND_FLOAT: debug_type_kind = 4;
pub const DEBUG_KIND_INT: debug_type_kind = 3;
pub const DEBUG_KIND_VOID: debug_type_kind = 2;
pub const DEBUG_KIND_INDIRECT: debug_type_kind = 1;
pub const DEBUG_KIND_ILLEGAL: debug_type_kind = 0;
pub type debug_var_kind = libc::c_uint;
pub const DEBUG_REGISTER: debug_var_kind = 5;
pub const DEBUG_LOCAL: debug_var_kind = 4;
pub const DEBUG_LOCAL_STATIC: debug_var_kind = 3;
pub const DEBUG_STATIC: debug_var_kind = 2;
pub const DEBUG_GLOBAL: debug_var_kind = 1;
pub const DEBUG_VAR_ILLEGAL: debug_var_kind = 0;
pub type debug_parm_kind = libc::c_uint;
pub const DEBUG_PARM_REF_REG: debug_parm_kind = 4;
pub const DEBUG_PARM_REFERENCE: debug_parm_kind = 3;
pub const DEBUG_PARM_REG: debug_parm_kind = 2;
pub const DEBUG_PARM_STACK: debug_parm_kind = 1;
pub const DEBUG_PARM_ILLEGAL: debug_parm_kind = 0;
pub type debug_visibility = libc::c_uint;
pub const DEBUG_VISIBILITY_IGNORE: debug_visibility = 3;
pub const DEBUG_VISIBILITY_PRIVATE: debug_visibility = 2;
pub const DEBUG_VISIBILITY_PROTECTED: debug_visibility = 1;
pub const DEBUG_VISIBILITY_PUBLIC: debug_visibility = 0;
pub type debug_type = *mut debug_type_s;
pub type debug_field = *mut debug_field_s;
pub type debug_baseclass = *mut debug_baseclass_s;
pub type debug_method = *mut debug_method_s;
pub type debug_method_variant = *mut debug_method_variant_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stab_handle {
    pub abfd: *mut bfd,
    pub sections: bool,
    pub syms: *mut *mut asymbol,
    pub symcount: libc::c_long,
    pub so_string: *mut libc::c_char,
    pub so_value: bfd_vma,
    pub file_start_offset: bfd_vma,
    pub function_start_offset: bfd_vma,
    pub gcc_compiled: libc::c_int,
    pub n_opt_found: bool,
    pub main_filename: *mut libc::c_char,
    pub bincl_stack: *mut bincl_file,
    pub bincl_list: *mut bincl_file,
    pub within_function: bool,
    pub function_end: bfd_vma,
    pub block_depth: libc::c_int,
    pub pending: *mut stab_pending_var,
    pub files: libc::c_uint,
    pub file_types: *mut *mut stab_types,
    pub xcoff_types: [debug_type; 34],
    pub tags: *mut stab_tag,
    pub self_crossref: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stab_tag {
    pub next: *mut stab_tag,
    pub name: *const libc::c_char,
    pub kind: debug_type_kind,
    pub slot: debug_type,
    pub type_0: debug_type,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stab_types {
    pub next: *mut stab_types,
    pub types: [debug_type; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stab_pending_var {
    pub next: *mut stab_pending_var,
    pub name: *const libc::c_char,
    pub type_0: debug_type,
    pub kind: debug_var_kind,
    pub val: bfd_vma,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bincl_file {
    pub next: *mut bincl_file,
    pub next_stack: *mut bincl_file,
    pub name: *const libc::c_char,
    pub hash: bfd_vma,
    pub file: libc::c_uint,
    pub file_types: *mut stab_types,
}
pub const N_MAIN: __stab_debug_code = 42;
pub const N_ENDM: __stab_debug_code = 98;
pub const N_OBJ: __stab_debug_code = 56;
pub const N_OPT: __stab_debug_code = 60;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stab_demangle_typestring {
    pub typestring: *const libc::c_char,
    pub len: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stab_demangle_info {
    pub dhandle: *mut libc::c_void,
    pub info: *mut stab_handle,
    pub args: *mut debug_type,
    pub varargs: bool,
    pub typestrings: *mut stab_demangle_typestring,
    pub typestring_count: libc::c_uint,
    pub typestring_alloc: libc::c_uint,
}
pub const N_FUN: __stab_debug_code = 36;
pub const N_ECOMM: __stab_debug_code = 228;
pub const N_BCOMM: __stab_debug_code = 226;
pub const N_SLINE: __stab_debug_code = 68;
pub const N_EXCL: __stab_debug_code = 194;
pub const N_EINCL: __stab_debug_code = 162;
pub const N_BINCL: __stab_debug_code = 130;
pub const N_SOL: __stab_debug_code = 132;
pub const N_SO: __stab_debug_code = 100;
pub const N_RBRAC: __stab_debug_code = 224;
pub const N_LBRAC: __stab_debug_code = 192;
pub type __stab_debug_code = libc::c_uint;
pub const LAST_UNUSED_STAB_CODE: __stab_debug_code = 255;
pub const N_LENG: __stab_debug_code = 254;
pub const N_NBLCS: __stab_debug_code = 248;
pub const N_NBSTS: __stab_debug_code = 246;
pub const N_NBBSS: __stab_debug_code = 244;
pub const N_NBDATA: __stab_debug_code = 242;
pub const N_NBTEXT: __stab_debug_code = 240;
pub const N_WITH: __stab_debug_code = 234;
pub const N_ECOML: __stab_debug_code = 232;
pub const N_PATCH: __stab_debug_code = 208;
pub const N_SCOPE: __stab_debug_code = 196;
pub const N_ENTRY: __stab_debug_code = 164;
pub const N_PSYM: __stab_debug_code = 160;
pub const N_LSYM: __stab_debug_code = 128;
pub const N_ALIAS: __stab_debug_code = 108;
pub const N_OSO: __stab_debug_code = 102;
pub const N_SSYM: __stab_debug_code = 96;
pub const N_CATCH: __stab_debug_code = 84;
pub const N_MOD2: __stab_debug_code = 80;
pub const N_EHDECL: __stab_debug_code = 80;
pub const N_ENSYM: __stab_debug_code = 78;
pub const N_FLINE: __stab_debug_code = 76;
pub const N_DEFD: __stab_debug_code = 74;
pub const N_BROWS: __stab_debug_code = 72;
pub const N_BSLINE: __stab_debug_code = 72;
pub const N_DSLINE: __stab_debug_code = 70;
pub const N_M2C: __stab_debug_code = 66;
pub const N_RSYM: __stab_debug_code = 64;
pub const N_NOMAP: __stab_debug_code = 52;
pub const N_NSYMS: __stab_debug_code = 50;
pub const N_PC: __stab_debug_code = 48;
pub const N_BNSYM: __stab_debug_code = 46;
pub const N_ROSYM: __stab_debug_code = 44;
pub const N_LCSYM: __stab_debug_code = 40;
pub const N_STSYM: __stab_debug_code = 38;
pub const N_FNAME: __stab_debug_code = 34;
pub const N_GSYM: __stab_debug_code = 32;
#[inline]
unsafe extern "C" fn atof(mut __nptr: *const libc::c_char) -> libc::c_double {
    return strtod(__nptr, 0 as *mut libc::c_void as *mut *mut libc::c_char);
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
unsafe extern "C" fn bfd_asymbol_value(mut sy: *const asymbol) -> bfd_vma {
    return ((*(*sy).section).vma).wrapping_add((*sy).value);
}
#[inline]
unsafe extern "C" fn bfd_asymbol_name(mut sy: *const asymbol) -> *const libc::c_char {
    return (*sy).name;
}
#[inline]
unsafe extern "C" fn bfd_get_symbol_leading_char(mut abfd: *const bfd) -> libc::c_char {
    return (*(*abfd).xvec).symbol_leading_char;
}
#[no_mangle]
pub unsafe extern "C" fn start_stab(
    mut _dhandle: *mut libc::c_void,
    mut abfd: *mut bfd,
    mut sections: bool,
    mut syms: *mut *mut asymbol,
    mut symcount: libc::c_long,
) -> *mut libc::c_void {
    let mut ret: *mut stab_handle = 0 as *mut stab_handle;
    ret = xmalloc(::core::mem::size_of::<stab_handle>() as libc::c_ulong)
        as *mut stab_handle;
    memset(
        ret as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<stab_handle>() as libc::c_ulong,
    );
    (*ret).abfd = abfd;
    (*ret).sections = sections;
    (*ret).syms = syms;
    (*ret).symcount = symcount;
    (*ret).files = 1 as libc::c_int as libc::c_uint;
    (*ret)
        .file_types = xmalloc(::core::mem::size_of::<*mut stab_types>() as libc::c_ulong)
        as *mut *mut stab_types;
    let ref mut fresh0 = *((*ret).file_types).offset(0 as libc::c_int as isize);
    *fresh0 = 0 as *mut stab_types;
    (*ret).function_end = -(1 as libc::c_int) as bfd_vma;
    return ret as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn finish_stab(
    mut dhandle: *mut libc::c_void,
    mut handle: *mut libc::c_void,
) -> bool {
    let mut info: *mut stab_handle = handle as *mut stab_handle;
    let mut st: *mut stab_tag = 0 as *mut stab_tag;
    if (*info).within_function {
        if !stab_emit_pending_vars(dhandle, info)
            || !debug_end_function(dhandle, (*info).function_end)
        {
            return 0 as libc::c_int != 0;
        }
        (*info).within_function = 0 as libc::c_int != 0;
        (*info).function_end = -(1 as libc::c_int) as bfd_vma;
    }
    st = (*info).tags;
    while !st.is_null() {
        let mut kind: debug_type_kind = DEBUG_KIND_ILLEGAL;
        kind = (*st).kind;
        if kind as libc::c_uint == DEBUG_KIND_ILLEGAL as libc::c_int as libc::c_uint {
            kind = DEBUG_KIND_STRUCT;
        }
        (*st).slot = debug_make_undefined_tagged_type(dhandle, (*st).name, kind);
        if ((*st).slot).is_null() {
            return 0 as libc::c_int != 0;
        }
        st = (*st).next;
    }
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn parse_stab(
    mut dhandle: *mut libc::c_void,
    mut handle: *mut libc::c_void,
    mut type_0: libc::c_int,
    mut desc: libc::c_int,
    mut value: bfd_vma,
    mut string: *const libc::c_char,
) -> bool {
    let mut string_end: *const libc::c_char = 0 as *const libc::c_char;
    let mut info: *mut stab_handle = handle as *mut stab_handle;
    if !((*info).so_string).is_null()
        && (type_0 != N_SO as libc::c_int || *string as libc::c_int == '\0' as i32
            || value != (*info).so_value)
    {
        if !debug_set_filename(dhandle, (*info).so_string) {
            return 0 as libc::c_int != 0;
        }
        (*info).main_filename = (*info).so_string;
        (*info).gcc_compiled = 0 as libc::c_int;
        (*info).n_opt_found = 0 as libc::c_int != 0;
        if !(*info).sections {
            (*info).file_start_offset = (*info).so_value;
        }
        (*info).files = 1 as libc::c_int as libc::c_uint;
        (*info)
            .file_types = xmalloc(
            ::core::mem::size_of::<*mut stab_types>() as libc::c_ulong,
        ) as *mut *mut stab_types;
        let ref mut fresh1 = *((*info).file_types).offset(0 as libc::c_int as isize);
        *fresh1 = 0 as *mut stab_types;
        (*info).so_string = 0 as *mut libc::c_char;
    }
    string_end = string.offset(strlen(string) as isize);
    let mut current_block_98: u64;
    match type_0 {
        192 => {
            if (*info).n_opt_found as libc::c_int != 0 && desc == 1 as libc::c_int {
                current_block_98 = 11869735117417356968;
            } else {
                if !(*info).within_function {
                    fprintf(
                        stderr,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"N_LBRAC not within function\n\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    return 0 as libc::c_int != 0;
                }
                if !debug_start_block(
                    dhandle,
                    value
                        .wrapping_add((*info).file_start_offset)
                        .wrapping_add((*info).function_start_offset),
                ) {
                    return 0 as libc::c_int != 0;
                }
                if !stab_emit_pending_vars(dhandle, info) {
                    return 0 as libc::c_int != 0;
                }
                (*info).block_depth += 1;
                (*info).block_depth;
                current_block_98 = 11869735117417356968;
            }
        }
        224 => {
            if (*info).n_opt_found as libc::c_int != 0 && desc == 1 as libc::c_int {
                current_block_98 = 11869735117417356968;
            } else {
                if !stab_emit_pending_vars(dhandle, info) {
                    return 0 as libc::c_int != 0;
                }
                if !debug_end_block(
                    dhandle,
                    value
                        .wrapping_add((*info).file_start_offset)
                        .wrapping_add((*info).function_start_offset),
                ) {
                    return 0 as libc::c_int != 0;
                }
                (*info).block_depth -= 1;
                (*info).block_depth;
                if (*info).block_depth < 0 as libc::c_int {
                    fprintf(
                        stderr,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Too many N_RBRACs\n\0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    return 0 as libc::c_int != 0;
                }
                current_block_98 = 11869735117417356968;
            }
        }
        100 => {
            if (*info).within_function {
                let mut endval: bfd_vma = 0;
                endval = value;
                if *string as libc::c_int != '\0' as i32
                    && (*info).function_end != -(1 as libc::c_int) as bfd_vma
                    && (*info).function_end < endval
                {
                    endval = (*info).function_end;
                }
                if !stab_emit_pending_vars(dhandle, info)
                    || !debug_end_function(dhandle, endval)
                {
                    return 0 as libc::c_int != 0;
                }
                (*info).within_function = 0 as libc::c_int != 0;
                (*info).function_end = -(1 as libc::c_int) as bfd_vma;
            }
            if *string as libc::c_int == '\0' as i32 {
                return 1 as libc::c_int != 0;
            }
            if ((*info).so_string).is_null() {
                (*info).so_string = xstrdup(string);
            } else {
                let mut f: *mut libc::c_char = 0 as *mut libc::c_char;
                f = (*info).so_string;
                if *string.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32
                    || *string.offset(0 as libc::c_int as isize) as libc::c_int
                        == '\\' as i32 && 0 as libc::c_int != 0
                    || *string.offset(0 as libc::c_int as isize) as libc::c_int != 0
                        && *string.offset(1 as libc::c_int as isize) as libc::c_int
                            == ':' as i32 && 0 as libc::c_int != 0
                {
                    (*info).so_string = xstrdup(string);
                } else {
                    (*info)
                        .so_string = concat(
                        (*info).so_string,
                        string,
                        0 as *mut libc::c_void as *const libc::c_char,
                    );
                }
                free(f as *mut libc::c_void);
            }
            (*info).so_value = value;
            current_block_98 = 11869735117417356968;
        }
        132 => {
            if !debug_start_source(dhandle, string) {
                return 0 as libc::c_int != 0;
            }
            current_block_98 = 11869735117417356968;
        }
        130 => {
            push_bincl(info, string, value);
            if !debug_start_source(dhandle, string) {
                return 0 as libc::c_int != 0;
            }
            current_block_98 = 11869735117417356968;
        }
        162 => {
            if !debug_start_source(dhandle, pop_bincl(info)) {
                return 0 as libc::c_int != 0;
            }
            current_block_98 = 11869735117417356968;
        }
        194 => {
            if !find_excl(info, string, value) {
                return 0 as libc::c_int != 0;
            }
            current_block_98 = 11869735117417356968;
        }
        68 => {
            if !debug_record_line(
                dhandle,
                desc as libc::c_ulong,
                value
                    .wrapping_add(
                        if (*info).within_function as libc::c_int != 0 {
                            (*info).function_start_offset
                        } else {
                            0 as libc::c_int as libc::c_ulong
                        },
                    ),
            ) {
                return 0 as libc::c_int != 0;
            }
            current_block_98 = 11869735117417356968;
        }
        226 => {
            if !debug_start_common_block(dhandle, string) {
                return 0 as libc::c_int != 0;
            }
            current_block_98 = 11869735117417356968;
        }
        228 => {
            if !debug_end_common_block(dhandle, string) {
                return 0 as libc::c_int != 0;
            }
            current_block_98 = 11869735117417356968;
        }
        36 => {
            if *string as libc::c_int == '\0' as i32 {
                if (*info).within_function {
                    if (*info).sections {
                        value = (value as libc::c_ulong)
                            .wrapping_add((*info).function_start_offset) as bfd_vma
                            as bfd_vma;
                    }
                    if !stab_emit_pending_vars(dhandle, info)
                        || !debug_end_function(dhandle, value)
                    {
                        return 0 as libc::c_int != 0;
                    }
                    (*info).within_function = 0 as libc::c_int != 0;
                    (*info).function_end = -(1 as libc::c_int) as bfd_vma;
                }
                current_block_98 = 11869735117417356968;
            } else {
                if (*info).within_function as libc::c_int != 0
                    && ((*info).function_end == -(1 as libc::c_int) as bfd_vma
                        || value < (*info).function_end)
                {
                    (*info).function_end = value;
                }
                current_block_98 = 984170810754156115;
            }
        }
        60 => {
            if !string.is_null()
                && strcmp(
                    string,
                    b"gcc2_compiled.\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
            {
                (*info).gcc_compiled = 2 as libc::c_int;
            } else if !string.is_null()
                && strcmp(string, b"gcc_compiled.\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
            {
                (*info).gcc_compiled = 1 as libc::c_int;
            } else {
                (*info).n_opt_found = 1 as libc::c_int != 0;
            }
            current_block_98 = 11869735117417356968;
        }
        31 | 12 | 56 | 98 | 42 | 30 => {
            current_block_98 = 11869735117417356968;
        }
        _ => {
            current_block_98 = 984170810754156115;
        }
    }
    match current_block_98 {
        984170810754156115 => {
            let mut colon: *const libc::c_char = 0 as *const libc::c_char;
            colon = strchr(string, ':' as i32);
            if !colon.is_null()
                && (*colon.offset(1 as libc::c_int as isize) as libc::c_int == 'f' as i32
                    || *colon.offset(1 as libc::c_int as isize) as libc::c_int
                        == 'F' as i32)
            {
                if (*info).within_function {
                    let mut endval_0: bfd_vma = 0;
                    endval_0 = value;
                    if (*info).function_end != -(1 as libc::c_int) as bfd_vma
                        && (*info).function_end < endval_0
                    {
                        endval_0 = (*info).function_end;
                    }
                    if !stab_emit_pending_vars(dhandle, info)
                        || !debug_end_function(dhandle, endval_0)
                    {
                        return 0 as libc::c_int != 0;
                    }
                    (*info).function_end = -(1 as libc::c_int) as bfd_vma;
                }
                if (*info).sections {
                    (*info).function_start_offset = value;
                }
                (*info).within_function = 1 as libc::c_int != 0;
            }
            if !parse_stab_string(
                dhandle,
                info,
                type_0,
                desc,
                value,
                string,
                string_end,
            ) {
                return 0 as libc::c_int != 0;
            }
        }
        _ => {}
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn savestring(
    mut start: *const libc::c_char,
    mut len: libc::c_int,
) -> *mut libc::c_char {
    let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
    ret = xmalloc((len + 1 as libc::c_int) as size_t) as *mut libc::c_char;
    memcpy(ret as *mut libc::c_void, start as *const libc::c_void, len as libc::c_ulong);
    *ret.offset(len as isize) = '\0' as i32 as libc::c_char;
    return ret;
}
unsafe extern "C" fn bad_stab(mut p: *const libc::c_char) {
    fprintf(
        stderr,
        dcgettext(
            0 as *const libc::c_char,
            b"Bad stab: %s\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        p,
    );
}
unsafe extern "C" fn warn_stab(
    mut p: *const libc::c_char,
    mut err: *const libc::c_char,
) {
    fprintf(
        stderr,
        dcgettext(
            0 as *const libc::c_char,
            b"Warning: %s: %s\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        err,
        p,
    );
}
unsafe extern "C" fn parse_stab_string(
    mut dhandle: *mut libc::c_void,
    mut info: *mut stab_handle,
    mut stabtype: libc::c_int,
    mut _desc: libc::c_int,
    mut value: bfd_vma,
    mut string: *const libc::c_char,
    mut string_end: *const libc::c_char,
) -> bool {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut type_0: libc::c_int = 0;
    let mut dtype: debug_type = 0 as *mut debug_type_s;
    let mut synonym: bool = false;
    let mut self_crossref: bool = false;
    let mut slot: *mut debug_type = 0 as *mut debug_type;
    p = strchr(string, ':' as i32);
    if p.is_null() {
        return 1 as libc::c_int != 0;
    }
    while *p.offset(1 as libc::c_int as isize) as libc::c_int == ':' as i32 {
        p = p.offset(2 as libc::c_int as isize);
        p = strchr(p, ':' as i32);
        if p.is_null() {
            bad_stab(string);
            return 0 as libc::c_int != 0;
        }
    }
    name = 0 as *mut libc::c_char;
    if *string.offset(0 as libc::c_int as isize) as libc::c_int == '$' as i32 {
        match *string.offset(1 as libc::c_int as isize) as libc::c_int {
            116 => {
                name = b"this\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
            }
            118 => {}
            101 => {
                name = b"eh_throw\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
            }
            95 => {}
            88 => {}
            _ => {
                warn_stab(
                    string,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"unknown C++ encoded name\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
            }
        }
    }
    if name.is_null() {
        if p == string
            || *string.offset(0 as libc::c_int as isize) as libc::c_int == ' ' as i32
                && p == string.offset(1 as libc::c_int as isize)
        {
            name = 0 as *mut libc::c_char;
        } else {
            name = savestring(
                string,
                p.offset_from(string) as libc::c_long as libc::c_int,
            );
        }
    }
    p = p.offset(1);
    p;
    if _sch_istable[(*p as libc::c_int & 0xff as libc::c_int) as usize] as libc::c_int
        & _sch_isdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
        || *p as libc::c_int == '(' as i32 || *p as libc::c_int == '-' as i32
    {
        type_0 = 'l' as i32;
    } else if *p as libc::c_int == 0 as libc::c_int {
        bad_stab(string);
        return 0 as libc::c_int != 0;
    } else {
        let fresh2 = p;
        p = p.offset(1);
        type_0 = *fresh2 as libc::c_int;
    }
    let mut current_block_189: u64;
    match type_0 {
        99 => {
            if *p as libc::c_int != '=' as i32 {
                bad_stab(string);
                return 0 as libc::c_int != 0;
            }
            p = p.offset(1);
            p;
            let fresh3 = p;
            p = p.offset(1);
            match *fresh3 as libc::c_int {
                114 => {
                    if !debug_record_float_const(dhandle, name, atof(p)) {
                        return 0 as libc::c_int != 0;
                    }
                }
                105 => {
                    if !debug_record_int_const(dhandle, name, atoi(p) as bfd_vma) {
                        return 0 as libc::c_int != 0;
                    }
                }
                101 => {
                    dtype = parse_stab_type(
                        dhandle,
                        info,
                        0 as *mut libc::c_void as *const libc::c_char,
                        &mut p,
                        0 as *mut libc::c_void as *mut *mut debug_type,
                        string_end,
                    );
                    if dtype.is_null() {
                        return 0 as libc::c_int != 0;
                    }
                    if *p as libc::c_int != ',' as i32 {
                        bad_stab(string);
                        return 0 as libc::c_int != 0;
                    }
                    if !debug_record_typed_const(
                        dhandle,
                        name,
                        dtype,
                        atoi(p) as bfd_vma,
                    ) {
                        return 0 as libc::c_int != 0;
                    }
                }
                _ => {
                    bad_stab(string);
                    return 0 as libc::c_int != 0;
                }
            }
            current_block_189 = 13161952823003036500;
        }
        67 => {
            dtype = parse_stab_type(
                dhandle,
                info,
                0 as *mut libc::c_void as *const libc::c_char,
                &mut p,
                0 as *mut libc::c_void as *mut *mut debug_type,
                string_end,
            );
            if dtype.is_null() {
                return 0 as libc::c_int != 0;
            }
            if !debug_record_label(dhandle, name, dtype, value) {
                return 0 as libc::c_int != 0;
            }
            current_block_189 = 13161952823003036500;
        }
        102 | 70 => {
            dtype = parse_stab_type(
                dhandle,
                info,
                0 as *mut libc::c_void as *const libc::c_char,
                &mut p,
                0 as *mut libc::c_void as *mut *mut debug_type,
                string_end,
            );
            if dtype.is_null() {
                return 0 as libc::c_int != 0;
            }
            if !debug_record_function(
                dhandle,
                name,
                dtype,
                type_0 == 'F' as i32,
                value,
            ) {
                return 0 as libc::c_int != 0;
            }
            while *p as libc::c_int == ';' as i32 {
                p = p.offset(1);
                p;
                if (parse_stab_type(
                    dhandle,
                    info,
                    0 as *mut libc::c_void as *const libc::c_char,
                    &mut p,
                    0 as *mut libc::c_void as *mut *mut debug_type,
                    string_end,
                ))
                    .is_null()
                {
                    return 0 as libc::c_int != 0;
                }
            }
            current_block_189 = 13161952823003036500;
        }
        71 => {
            let mut ps: *mut *mut asymbol = 0 as *mut *mut asymbol;
            dtype = parse_stab_type(
                dhandle,
                info,
                0 as *mut libc::c_void as *const libc::c_char,
                &mut p,
                0 as *mut libc::c_void as *mut *mut debug_type,
                string_end,
            );
            if dtype.is_null() {
                return 0 as libc::c_int != 0;
            }
            if !name.is_null() {
                let mut leading: libc::c_char = 0;
                let mut c: libc::c_long = 0;
                leading = bfd_get_symbol_leading_char((*info).abfd);
                c = (*info).symcount;
                ps = (*info).syms;
                while c > 0 as libc::c_int as libc::c_long {
                    let mut n: *const libc::c_char = 0 as *const libc::c_char;
                    n = bfd_asymbol_name(*ps);
                    if leading as libc::c_int != '\0' as i32
                        && *n as libc::c_int == leading as libc::c_int
                    {
                        n = n.offset(1);
                        n;
                    }
                    if *n as libc::c_int == *name as libc::c_int
                        && strcmp(n, name) == 0 as libc::c_int
                    {
                        break;
                    }
                    c -= 1;
                    c;
                    ps = ps.offset(1);
                    ps;
                }
                if c > 0 as libc::c_int as libc::c_long {
                    value = bfd_asymbol_value(*ps);
                }
            }
            if !stab_record_variable(dhandle, info, name, dtype, DEBUG_GLOBAL, value) {
                return 0 as libc::c_int != 0;
            }
            current_block_189 = 13161952823003036500;
        }
        108 | 115 => {
            dtype = parse_stab_type(
                dhandle,
                info,
                0 as *mut libc::c_void as *const libc::c_char,
                &mut p,
                0 as *mut libc::c_void as *mut *mut debug_type,
                string_end,
            );
            if dtype.is_null() {
                return 0 as libc::c_int != 0;
            }
            if !stab_record_variable(dhandle, info, name, dtype, DEBUG_LOCAL, value) {
                return 0 as libc::c_int != 0;
            }
            current_block_189 = 13161952823003036500;
        }
        112 => {
            if *p as libc::c_int != 'F' as i32 {
                dtype = parse_stab_type(
                    dhandle,
                    info,
                    0 as *mut libc::c_void as *const libc::c_char,
                    &mut p,
                    0 as *mut libc::c_void as *mut *mut debug_type,
                    string_end,
                );
            } else {
                p = p.offset(1);
                p;
                dtype = parse_stab_type(
                    dhandle,
                    info,
                    0 as *mut libc::c_void as *const libc::c_char,
                    &mut p,
                    0 as *mut libc::c_void as *mut *mut debug_type,
                    string_end,
                );
                if !dtype.is_null() {
                    let mut ftype: debug_type = 0 as *mut debug_type_s;
                    ftype = debug_make_function_type(
                        dhandle,
                        dtype,
                        0 as *mut libc::c_void as *mut debug_type,
                        0 as libc::c_int != 0,
                    );
                    dtype = debug_make_pointer_type(dhandle, ftype);
                }
            }
            if dtype.is_null() {
                return 0 as libc::c_int != 0;
            }
            if !debug_record_parameter(dhandle, name, dtype, DEBUG_PARM_STACK, value) {
                return 0 as libc::c_int != 0;
            }
            current_block_189 = 13161952823003036500;
        }
        80 => {
            if stabtype == N_FUN as libc::c_int {
                while *p as libc::c_int == ';' as i32 {
                    p = p.offset(1);
                    p;
                    if (parse_stab_type(
                        dhandle,
                        info,
                        0 as *mut libc::c_void as *const libc::c_char,
                        &mut p,
                        0 as *mut libc::c_void as *mut *mut debug_type,
                        string_end,
                    ))
                        .is_null()
                    {
                        return 0 as libc::c_int != 0;
                    }
                }
                current_block_189 = 13161952823003036500;
            } else {
                current_block_189 = 17988377589706326989;
            }
        }
        82 => {
            current_block_189 = 17988377589706326989;
        }
        114 => {
            dtype = parse_stab_type(
                dhandle,
                info,
                0 as *mut libc::c_void as *const libc::c_char,
                &mut p,
                0 as *mut libc::c_void as *mut *mut debug_type,
                string_end,
            );
            if dtype.is_null() {
                return 0 as libc::c_int != 0;
            }
            if !stab_record_variable(dhandle, info, name, dtype, DEBUG_REGISTER, value) {
                return 0 as libc::c_int != 0;
            }
            current_block_189 = 13161952823003036500;
        }
        83 => {
            dtype = parse_stab_type(
                dhandle,
                info,
                0 as *mut libc::c_void as *const libc::c_char,
                &mut p,
                0 as *mut libc::c_void as *mut *mut debug_type,
                string_end,
            );
            if dtype.is_null() {
                return 0 as libc::c_int != 0;
            }
            if !stab_record_variable(dhandle, info, name, dtype, DEBUG_STATIC, value) {
                return 0 as libc::c_int != 0;
            }
            current_block_189 = 13161952823003036500;
        }
        116 => {
            dtype = parse_stab_type(dhandle, info, name, &mut p, &mut slot, string_end);
            if dtype.is_null() {
                return 0 as libc::c_int != 0;
            }
            if name.is_null() {
                return 1 as libc::c_int != 0;
            }
            dtype = debug_name_type(dhandle, name, dtype);
            if dtype.is_null() {
                return 0 as libc::c_int != 0;
            }
            if !slot.is_null() {
                *slot = dtype;
            }
            current_block_189 = 13161952823003036500;
        }
        84 => {
            if *p as libc::c_int != 't' as i32 {
                synonym = 0 as libc::c_int != 0;
            } else {
                synonym = 1 as libc::c_int != 0;
                p = p.offset(1);
                p;
            }
            dtype = parse_stab_type(dhandle, info, name, &mut p, &mut slot, string_end);
            if dtype.is_null() {
                return 0 as libc::c_int != 0;
            }
            if name.is_null() {
                return 1 as libc::c_int != 0;
            }
            self_crossref = (*info).self_crossref;
            dtype = debug_tag_type(dhandle, name, dtype);
            if dtype.is_null() {
                return 0 as libc::c_int != 0;
            }
            if !slot.is_null() {
                *slot = dtype;
            }
            if !self_crossref {
                let mut pst: *mut *mut stab_tag = 0 as *mut *mut stab_tag;
                pst = &mut (*info).tags;
                while !(*pst).is_null() {
                    if *((**pst).name).offset(0 as libc::c_int as isize) as libc::c_int
                        == *name.offset(0 as libc::c_int as isize) as libc::c_int
                        && strcmp((**pst).name, name) == 0 as libc::c_int
                    {
                        (**pst).slot = dtype;
                        *pst = (**pst).next;
                        break;
                    } else {
                        pst = &mut (**pst).next;
                    }
                }
            }
            if synonym {
                dtype = debug_name_type(dhandle, name, dtype);
                if dtype.is_null() {
                    return 0 as libc::c_int != 0;
                }
                if !slot.is_null() {
                    *slot = dtype;
                }
            }
            current_block_189 = 13161952823003036500;
        }
        86 => {
            dtype = parse_stab_type(
                dhandle,
                info,
                0 as *mut libc::c_void as *const libc::c_char,
                &mut p,
                0 as *mut libc::c_void as *mut *mut debug_type,
                string_end,
            );
            if dtype.is_null() {
                return 0 as libc::c_int != 0;
            }
            if !stab_record_variable(
                dhandle,
                info,
                name,
                dtype,
                DEBUG_LOCAL_STATIC,
                value,
            ) {
                return 0 as libc::c_int != 0;
            }
            current_block_189 = 13161952823003036500;
        }
        118 => {
            dtype = parse_stab_type(
                dhandle,
                info,
                0 as *mut libc::c_void as *const libc::c_char,
                &mut p,
                0 as *mut libc::c_void as *mut *mut debug_type,
                string_end,
            );
            if dtype.is_null() {
                return 0 as libc::c_int != 0;
            }
            if !debug_record_parameter(
                dhandle,
                name,
                dtype,
                DEBUG_PARM_REFERENCE,
                value,
            ) {
                return 0 as libc::c_int != 0;
            }
            current_block_189 = 13161952823003036500;
        }
        97 => {
            dtype = parse_stab_type(
                dhandle,
                info,
                0 as *mut libc::c_void as *const libc::c_char,
                &mut p,
                0 as *mut libc::c_void as *mut *mut debug_type,
                string_end,
            );
            if dtype.is_null() {
                return 0 as libc::c_int != 0;
            }
            if !debug_record_parameter(dhandle, name, dtype, DEBUG_PARM_REF_REG, value) {
                return 0 as libc::c_int != 0;
            }
            current_block_189 = 13161952823003036500;
        }
        88 => {
            dtype = parse_stab_type(
                dhandle,
                info,
                0 as *mut libc::c_void as *const libc::c_char,
                &mut p,
                0 as *mut libc::c_void as *mut *mut debug_type,
                string_end,
            );
            if dtype.is_null() {
                return 0 as libc::c_int != 0;
            }
            if !stab_record_variable(dhandle, info, name, dtype, DEBUG_LOCAL, value) {
                return 0 as libc::c_int != 0;
            }
            current_block_189 = 13161952823003036500;
        }
        89 => {
            p = p.offset(1);
            if *p as libc::c_int == 'n' as i32
                && {
                    p = p.offset(1);
                    *p as libc::c_int == '0' as i32
                }
            {
                while *p as libc::c_int != ';' as i32 {
                    p = p.offset(1);
                    p;
                }
                p = p.offset(1);
                p;
                return 1 as libc::c_int != 0;
            }
            current_block_189 = 4996350435295267853;
        }
        _ => {
            current_block_189 = 4996350435295267853;
        }
    }
    match current_block_189 {
        4996350435295267853 => {
            bad_stab(string);
            return 0 as libc::c_int != 0;
        }
        17988377589706326989 => {
            dtype = parse_stab_type(
                dhandle,
                info,
                0 as *mut libc::c_void as *const libc::c_char,
                &mut p,
                0 as *mut libc::c_void as *mut *mut debug_type,
                string_end,
            );
            if dtype.is_null() {
                return 0 as libc::c_int != 0;
            }
            if !debug_record_parameter(dhandle, name, dtype, DEBUG_PARM_REG, value) {
                return 0 as libc::c_int != 0;
            }
        }
        _ => {}
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn parse_stab_type(
    mut dhandle: *mut libc::c_void,
    mut info: *mut stab_handle,
    mut type_name: *const libc::c_char,
    mut pp: *mut *const libc::c_char,
    mut slotp: *mut *mut debug_type,
    mut p_end: *const libc::c_char,
) -> debug_type {
    let mut orig: *const libc::c_char = 0 as *const libc::c_char;
    let mut typenums: [libc::c_int; 2] = [0; 2];
    let mut size: libc::c_int = 0;
    let mut stringp: bool = false;
    let mut descriptor: libc::c_int = 0;
    let mut dtype: debug_type = 0 as *mut debug_type_s;
    if !slotp.is_null() {
        *slotp = 0 as *mut debug_type;
    }
    orig = *pp;
    if orig >= p_end {
        return 0 as *mut libc::c_void as debug_type;
    }
    size = -(1 as libc::c_int);
    stringp = 0 as libc::c_int != 0;
    (*info).self_crossref = 0 as libc::c_int != 0;
    if _sch_istable[(**pp as libc::c_int & 0xff as libc::c_int) as usize] as libc::c_int
        & _sch_isdigit as libc::c_int as libc::c_ushort as libc::c_int == 0
        && **pp as libc::c_int != '(' as i32 && **pp as libc::c_int != '-' as i32
    {
        typenums[1 as libc::c_int as usize] = -(1 as libc::c_int);
        typenums[0 as libc::c_int as usize] = typenums[1 as libc::c_int as usize];
    } else {
        if !parse_stab_type_number(pp, typenums.as_mut_ptr(), p_end) {
            return 0 as *mut libc::c_void as debug_type;
        }
        if **pp as libc::c_int != '=' as i32 {
            return stab_find_type(dhandle, info, typenums.as_mut_ptr());
        }
        if !slotp.is_null() && typenums[0 as libc::c_int as usize] >= 0 as libc::c_int
            && typenums[1 as libc::c_int as usize] >= 0 as libc::c_int
        {
            *slotp = stab_find_slot(info, typenums.as_mut_ptr());
        }
        *pp = (*pp).offset(1);
        let _ = *pp;
        while **pp as libc::c_int == '@' as i32 {
            let mut p: *const libc::c_char = (*pp).offset(1 as libc::c_int as isize);
            let mut attr: *const libc::c_char = 0 as *const libc::c_char;
            if _sch_istable[(*p as libc::c_int & 0xff as libc::c_int) as usize]
                as libc::c_int
                & _sch_isdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
                || *p as libc::c_int == '(' as i32 || *p as libc::c_int == '-' as i32
            {
                break;
            }
            attr = p;
            while *p as libc::c_int != ';' as i32 {
                if *p as libc::c_int == '\0' as i32 {
                    bad_stab(orig);
                    return 0 as *mut libc::c_void as debug_type;
                }
                p = p.offset(1);
                p;
            }
            *pp = p.offset(1 as libc::c_int as isize);
            match *attr as libc::c_int {
                115 => {
                    size = atoi(attr.offset(1 as libc::c_int as isize));
                    size /= 8 as libc::c_int;
                    if size <= 0 as libc::c_int {
                        size = -(1 as libc::c_int);
                    }
                }
                83 => {
                    stringp = 1 as libc::c_int != 0;
                }
                0 => {
                    bad_stab(orig);
                    return 0 as *mut libc::c_void as debug_type;
                }
                _ => {}
            }
        }
    }
    descriptor = **pp as libc::c_int;
    *pp = (*pp).offset(1);
    let _ = *pp;
    match descriptor {
        120 => {
            let mut code: debug_type_kind = DEBUG_KIND_ILLEGAL;
            let mut q1: *const libc::c_char = 0 as *const libc::c_char;
            let mut q2: *const libc::c_char = 0 as *const libc::c_char;
            let mut p_0: *const libc::c_char = 0 as *const libc::c_char;
            match **pp as libc::c_int {
                115 => {
                    code = DEBUG_KIND_STRUCT;
                }
                117 => {
                    code = DEBUG_KIND_UNION;
                }
                101 => {
                    code = DEBUG_KIND_ENUM;
                }
                0 => {
                    bad_stab(orig);
                    return 0 as *mut libc::c_void as debug_type;
                }
                _ => {
                    warn_stab(
                        orig,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"unrecognized cross reference type\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    code = DEBUG_KIND_STRUCT;
                }
            }
            *pp = (*pp).offset(1);
            let _ = *pp;
            q1 = strchr(*pp, '<' as i32);
            p_0 = strchr(*pp, ':' as i32);
            if p_0.is_null() {
                bad_stab(orig);
                return 0 as *mut libc::c_void as debug_type;
            }
            if !q1.is_null() && p_0 > q1
                && *p_0.offset(1 as libc::c_int as isize) as libc::c_int == ':' as i32
            {
                let mut nest: libc::c_int = 0 as libc::c_int;
                q2 = q1;
                while *q2 as libc::c_int != '\0' as i32 {
                    if *q2 as libc::c_int == '<' as i32 {
                        nest += 1;
                        nest;
                    } else if *q2 as libc::c_int == '>' as i32 {
                        nest -= 1;
                        nest;
                    } else if *q2 as libc::c_int == ':' as i32
                        && nest == 0 as libc::c_int
                    {
                        break;
                    }
                    q2 = q2.offset(1);
                    q2;
                }
                p_0 = q2;
                if *p_0 as libc::c_int != ':' as i32 {
                    bad_stab(orig);
                    return 0 as *mut libc::c_void as debug_type;
                }
            }
            if !type_name.is_null()
                && strncmp(
                    type_name,
                    *pp,
                    p_0.offset_from(*pp) as libc::c_long as libc::c_ulong,
                ) == 0 as libc::c_int
                && *type_name.offset(p_0.offset_from(*pp) as libc::c_long as isize)
                    as libc::c_int == '\0' as i32
            {
                (*info).self_crossref = 1 as libc::c_int != 0;
            }
            dtype = stab_find_tagged_type(
                dhandle,
                info,
                *pp,
                p_0.offset_from(*pp) as libc::c_long as libc::c_int,
                code,
            );
            *pp = p_0.offset(1 as libc::c_int as isize);
        }
        45 | 48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 | 40 => {
            let mut hold: *const libc::c_char = 0 as *const libc::c_char;
            let mut xtypenums: [libc::c_int; 2] = [0; 2];
            *pp = (*pp).offset(-1);
            let _ = *pp;
            hold = *pp;
            if !parse_stab_type_number(pp, xtypenums.as_mut_ptr(), p_end) {
                return 0 as *mut libc::c_void as debug_type;
            }
            if typenums[0 as libc::c_int as usize]
                == xtypenums[0 as libc::c_int as usize]
                && typenums[1 as libc::c_int as usize]
                    == xtypenums[1 as libc::c_int as usize]
            {
                dtype = debug_make_void_type(dhandle);
            } else {
                *pp = hold;
                dtype = parse_stab_type(
                    dhandle,
                    info,
                    0 as *mut libc::c_void as *const libc::c_char,
                    pp,
                    0 as *mut libc::c_void as *mut *mut debug_type,
                    p_end,
                );
                if dtype.is_null() {
                    return 0 as *mut libc::c_void as debug_type;
                }
            }
            if typenums[0 as libc::c_int as usize] != -(1 as libc::c_int) {
                if !stab_record_type(dhandle, info, typenums.as_mut_ptr(), dtype) {
                    return 0 as *mut libc::c_void as debug_type;
                }
            }
        }
        42 => {
            dtype = debug_make_pointer_type(
                dhandle,
                parse_stab_type(
                    dhandle,
                    info,
                    0 as *mut libc::c_void as *const libc::c_char,
                    pp,
                    0 as *mut libc::c_void as *mut *mut debug_type,
                    p_end,
                ),
            );
        }
        38 => {
            dtype = debug_make_reference_type(
                dhandle,
                parse_stab_type(
                    dhandle,
                    info,
                    0 as *mut libc::c_void as *const libc::c_char,
                    pp,
                    0 as *mut libc::c_void as *mut *mut debug_type,
                    p_end,
                ),
            );
        }
        102 => {
            dtype = debug_make_function_type(
                dhandle,
                parse_stab_type(
                    dhandle,
                    info,
                    0 as *mut libc::c_void as *const libc::c_char,
                    pp,
                    0 as *mut libc::c_void as *mut *mut debug_type,
                    p_end,
                ),
                0 as *mut libc::c_void as *mut debug_type,
                0 as libc::c_int != 0,
            );
        }
        107 => {
            dtype = debug_make_const_type(
                dhandle,
                parse_stab_type(
                    dhandle,
                    info,
                    0 as *mut libc::c_void as *const libc::c_char,
                    pp,
                    0 as *mut libc::c_void as *mut *mut debug_type,
                    p_end,
                ),
            );
        }
        66 => {
            dtype = debug_make_volatile_type(
                dhandle,
                parse_stab_type(
                    dhandle,
                    info,
                    0 as *mut libc::c_void as *const libc::c_char,
                    pp,
                    0 as *mut libc::c_void as *mut *mut debug_type,
                    p_end,
                ),
            );
        }
        64 => {
            let mut domain: debug_type = 0 as *mut debug_type_s;
            let mut memtype: debug_type = 0 as *mut debug_type_s;
            domain = parse_stab_type(
                dhandle,
                info,
                0 as *mut libc::c_void as *const libc::c_char,
                pp,
                0 as *mut libc::c_void as *mut *mut debug_type,
                p_end,
            );
            if domain.is_null() {
                return 0 as *mut libc::c_void as debug_type;
            }
            if **pp as libc::c_int != ',' as i32 {
                bad_stab(orig);
                return 0 as *mut libc::c_void as debug_type;
            }
            *pp = (*pp).offset(1);
            let _ = *pp;
            memtype = parse_stab_type(
                dhandle,
                info,
                0 as *mut libc::c_void as *const libc::c_char,
                pp,
                0 as *mut libc::c_void as *mut *mut debug_type,
                p_end,
            );
            if memtype.is_null() {
                return 0 as *mut libc::c_void as debug_type;
            }
            dtype = debug_make_offset_type(dhandle, domain, memtype);
        }
        35 => {
            if **pp as libc::c_int == '#' as i32 {
                let mut return_type: debug_type = 0 as *mut debug_type_s;
                *pp = (*pp).offset(1);
                let _ = *pp;
                return_type = parse_stab_type(
                    dhandle,
                    info,
                    0 as *mut libc::c_void as *const libc::c_char,
                    pp,
                    0 as *mut libc::c_void as *mut *mut debug_type,
                    p_end,
                );
                if return_type.is_null() {
                    return 0 as *mut libc::c_void as debug_type;
                }
                if **pp as libc::c_int != ';' as i32 {
                    bad_stab(orig);
                    return 0 as *mut libc::c_void as debug_type;
                }
                *pp = (*pp).offset(1);
                let _ = *pp;
                dtype = debug_make_method_type(
                    dhandle,
                    return_type,
                    0 as *mut libc::c_void as debug_type,
                    0 as *mut libc::c_void as *mut debug_type,
                    0 as libc::c_int != 0,
                );
            } else {
                let mut domain_0: debug_type = 0 as *mut debug_type_s;
                let mut return_type_0: debug_type = 0 as *mut debug_type_s;
                let mut args: *mut debug_type = 0 as *mut debug_type;
                let mut n: libc::c_uint = 0;
                let mut alloc: libc::c_uint = 0;
                let mut varargs: bool = false;
                domain_0 = parse_stab_type(
                    dhandle,
                    info,
                    0 as *mut libc::c_void as *const libc::c_char,
                    pp,
                    0 as *mut libc::c_void as *mut *mut debug_type,
                    p_end,
                );
                if domain_0.is_null() {
                    return 0 as *mut libc::c_void as debug_type;
                }
                if **pp as libc::c_int != ',' as i32 {
                    bad_stab(orig);
                    return 0 as *mut libc::c_void as debug_type;
                }
                *pp = (*pp).offset(1);
                let _ = *pp;
                return_type_0 = parse_stab_type(
                    dhandle,
                    info,
                    0 as *mut libc::c_void as *const libc::c_char,
                    pp,
                    0 as *mut libc::c_void as *mut *mut debug_type,
                    p_end,
                );
                if return_type_0.is_null() {
                    return 0 as *mut libc::c_void as debug_type;
                }
                alloc = 10 as libc::c_int as libc::c_uint;
                args = xmalloc(
                    (alloc as libc::c_ulong)
                        .wrapping_mul(
                            ::core::mem::size_of::<debug_type>() as libc::c_ulong,
                        ),
                ) as *mut debug_type;
                n = 0 as libc::c_int as libc::c_uint;
                while **pp as libc::c_int != ';' as i32 {
                    if **pp as libc::c_int != ',' as i32 {
                        bad_stab(orig);
                        return 0 as *mut libc::c_void as debug_type;
                    }
                    *pp = (*pp).offset(1);
                    let _ = *pp;
                    if n.wrapping_add(1 as libc::c_int as libc::c_uint) >= alloc {
                        alloc = alloc.wrapping_add(10 as libc::c_int as libc::c_uint);
                        args = xrealloc(
                            args as *mut libc::c_void,
                            (alloc as libc::c_ulong)
                                .wrapping_mul(
                                    ::core::mem::size_of::<debug_type>() as libc::c_ulong,
                                ),
                        ) as *mut debug_type;
                    }
                    let ref mut fresh4 = *args.offset(n as isize);
                    *fresh4 = parse_stab_type(
                        dhandle,
                        info,
                        0 as *mut libc::c_void as *const libc::c_char,
                        pp,
                        0 as *mut libc::c_void as *mut *mut debug_type,
                        p_end,
                    );
                    if (*args.offset(n as isize)).is_null() {
                        return 0 as *mut libc::c_void as debug_type;
                    }
                    n = n.wrapping_add(1);
                    n;
                }
                *pp = (*pp).offset(1);
                let _ = *pp;
                if n == 0 as libc::c_int as libc::c_uint
                    || debug_get_type_kind(
                        dhandle,
                        *args
                            .offset(
                                n.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
                            ),
                    ) as libc::c_uint != DEBUG_KIND_VOID as libc::c_int as libc::c_uint
                {
                    varargs = 1 as libc::c_int != 0;
                } else {
                    n = n.wrapping_sub(1);
                    n;
                    varargs = 0 as libc::c_int != 0;
                }
                let ref mut fresh5 = *args.offset(n as isize);
                *fresh5 = 0 as *mut libc::c_void as debug_type;
                dtype = debug_make_method_type(
                    dhandle,
                    return_type_0,
                    domain_0,
                    args,
                    varargs,
                );
            }
        }
        114 => {
            dtype = parse_stab_range_type(
                dhandle,
                info,
                type_name,
                pp,
                typenums.as_mut_ptr(),
                p_end,
            );
        }
        98 => {
            dtype = parse_stab_sun_builtin_type(dhandle, pp, p_end);
        }
        82 => {
            dtype = parse_stab_sun_floating_type(dhandle, pp, p_end);
        }
        101 => {
            dtype = parse_stab_enum_type(dhandle, pp, p_end);
        }
        115 | 117 => {
            dtype = parse_stab_struct_type(
                dhandle,
                info,
                type_name,
                pp,
                descriptor == 's' as i32,
                typenums.as_mut_ptr(),
                p_end,
            );
        }
        97 => {
            if **pp as libc::c_int != 'r' as i32 {
                bad_stab(orig);
                return 0 as *mut libc::c_void as debug_type;
            }
            *pp = (*pp).offset(1);
            let _ = *pp;
            dtype = parse_stab_array_type(dhandle, info, pp, stringp, p_end);
        }
        83 => {
            dtype = debug_make_set_type(
                dhandle,
                parse_stab_type(
                    dhandle,
                    info,
                    0 as *mut libc::c_void as *const libc::c_char,
                    pp,
                    0 as *mut libc::c_void as *mut *mut debug_type,
                    p_end,
                ),
                stringp,
            );
        }
        _ => {
            bad_stab(orig);
            return 0 as *mut libc::c_void as debug_type;
        }
    }
    if dtype.is_null() {
        return 0 as *mut libc::c_void as debug_type;
    }
    if typenums[0 as libc::c_int as usize] != -(1 as libc::c_int) {
        if !stab_record_type(dhandle, info, typenums.as_mut_ptr(), dtype) {
            return 0 as *mut libc::c_void as debug_type;
        }
    }
    if size != -(1 as libc::c_int) {
        if !debug_record_type_size(dhandle, dtype, size as libc::c_uint) {
            return 0 as *mut libc::c_void as debug_type;
        }
    }
    return dtype;
}
unsafe extern "C" fn parse_stab_type_number(
    mut pp: *mut *const libc::c_char,
    mut typenums: *mut libc::c_int,
    mut p_end: *const libc::c_char,
) -> bool {
    let mut orig: *const libc::c_char = 0 as *const libc::c_char;
    orig = *pp;
    if **pp as libc::c_int != '(' as i32 {
        *typenums.offset(0 as libc::c_int as isize) = 0 as libc::c_int;
        *typenums
            .offset(
                1 as libc::c_int as isize,
            ) = parse_number(pp, 0 as *mut libc::c_void as *mut bool, p_end)
            as libc::c_int;
        return 1 as libc::c_int != 0;
    }
    *pp = (*pp).offset(1);
    let _ = *pp;
    *typenums
        .offset(
            0 as libc::c_int as isize,
        ) = parse_number(pp, 0 as *mut libc::c_void as *mut bool, p_end) as libc::c_int;
    if **pp as libc::c_int != ',' as i32 {
        bad_stab(orig);
        return 0 as libc::c_int != 0;
    }
    *pp = (*pp).offset(1);
    let _ = *pp;
    *typenums
        .offset(
            1 as libc::c_int as isize,
        ) = parse_number(pp, 0 as *mut libc::c_void as *mut bool, p_end) as libc::c_int;
    if **pp as libc::c_int != ')' as i32 {
        bad_stab(orig);
        return 0 as libc::c_int != 0;
    }
    *pp = (*pp).offset(1);
    let _ = *pp;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn parse_stab_range_type(
    mut dhandle: *mut libc::c_void,
    mut info: *mut stab_handle,
    mut type_name: *const libc::c_char,
    mut pp: *mut *const libc::c_char,
    mut typenums: *const libc::c_int,
    mut p_end: *const libc::c_char,
) -> debug_type {
    let mut orig: *const libc::c_char = 0 as *const libc::c_char;
    let mut rangenums: [libc::c_int; 2] = [0; 2];
    let mut self_subrange: bool = false;
    let mut index_type: debug_type = 0 as *mut debug_type_s;
    let mut s2: *const libc::c_char = 0 as *const libc::c_char;
    let mut s3: *const libc::c_char = 0 as *const libc::c_char;
    let mut n2: bfd_signed_vma = 0;
    let mut n3: bfd_signed_vma = 0;
    let mut ov2: bool = false;
    let mut ov3: bool = false;
    orig = *pp;
    if orig >= p_end {
        return 0 as *mut libc::c_void as debug_type;
    }
    index_type = 0 as *mut libc::c_void as debug_type;
    if !parse_stab_type_number(pp, rangenums.as_mut_ptr(), p_end) {
        return 0 as *mut libc::c_void as debug_type;
    }
    self_subrange = rangenums[0 as libc::c_int as usize]
        == *typenums.offset(0 as libc::c_int as isize)
        && rangenums[1 as libc::c_int as usize]
            == *typenums.offset(1 as libc::c_int as isize);
    if **pp as libc::c_int == '=' as i32 {
        *pp = orig;
        index_type = parse_stab_type(
            dhandle,
            info,
            0 as *mut libc::c_void as *const libc::c_char,
            pp,
            0 as *mut libc::c_void as *mut *mut debug_type,
            p_end,
        );
        if index_type.is_null() {
            return 0 as *mut libc::c_void as debug_type;
        }
    }
    if **pp as libc::c_int == ';' as i32 {
        *pp = (*pp).offset(1);
        let _ = *pp;
    }
    s2 = *pp;
    n2 = parse_number(pp, &mut ov2, p_end) as bfd_signed_vma;
    if **pp as libc::c_int != ';' as i32 {
        bad_stab(orig);
        return 0 as *mut libc::c_void as debug_type;
    }
    *pp = (*pp).offset(1);
    let _ = *pp;
    s3 = *pp;
    n3 = parse_number(pp, &mut ov3, p_end) as bfd_signed_vma;
    if **pp as libc::c_int != ';' as i32 {
        bad_stab(orig);
        return 0 as *mut libc::c_void as debug_type;
    }
    *pp = (*pp).offset(1);
    let _ = *pp;
    if ov2 as libc::c_int != 0 || ov3 as libc::c_int != 0 {
        if index_type.is_null() {
            if startswith(
                s2,
                b"01000000000000000000000;\0" as *const u8 as *const libc::c_char,
            ) as libc::c_int != 0
                && startswith(
                    s3,
                    b"0777777777777777777777;\0" as *const u8 as *const libc::c_char,
                ) as libc::c_int != 0
            {
                return debug_make_int_type(
                    dhandle,
                    8 as libc::c_int as libc::c_uint,
                    0 as libc::c_int != 0,
                );
            }
            if !ov2 && n2 == 0 as libc::c_int as libc::c_long
                && startswith(
                    s3,
                    b"01777777777777777777777;\0" as *const u8 as *const libc::c_char,
                ) as libc::c_int != 0
            {
                return debug_make_int_type(
                    dhandle,
                    8 as libc::c_int as libc::c_uint,
                    1 as libc::c_int != 0,
                );
            }
        }
        warn_stab(
            orig,
            dcgettext(
                0 as *const libc::c_char,
                b"numeric overflow\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    if index_type.is_null() {
        if self_subrange as libc::c_int != 0 && n2 == 0 as libc::c_int as libc::c_long
            && n3 == 0 as libc::c_int as libc::c_long
        {
            return debug_make_void_type(dhandle);
        }
        if self_subrange as libc::c_int != 0 && n3 == 0 as libc::c_int as libc::c_long
            && n2 > 0 as libc::c_int as libc::c_long
        {
            return debug_make_complex_type(dhandle, n2 as libc::c_uint);
        }
        if n3 == 0 as libc::c_int as libc::c_long
            && n2 > 0 as libc::c_int as libc::c_long
        {
            return debug_make_float_type(dhandle, n2 as libc::c_uint);
        }
        if n2 == 0 as libc::c_int as libc::c_long
            && n3 == -(1 as libc::c_int) as libc::c_long
        {
            if !type_name.is_null() {
                if strcmp(
                    type_name,
                    b"long long int\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    return debug_make_int_type(
                        dhandle,
                        8 as libc::c_int as libc::c_uint,
                        0 as libc::c_int != 0,
                    )
                } else if strcmp(
                    type_name,
                    b"long long unsigned int\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    return debug_make_int_type(
                        dhandle,
                        8 as libc::c_int as libc::c_uint,
                        1 as libc::c_int != 0,
                    )
                }
            }
            return debug_make_int_type(
                dhandle,
                4 as libc::c_int as libc::c_uint,
                1 as libc::c_int != 0,
            );
        }
        if self_subrange as libc::c_int != 0 && n2 == 0 as libc::c_int as libc::c_long
            && n3 == 127 as libc::c_int as libc::c_long
        {
            return debug_make_int_type(
                dhandle,
                1 as libc::c_int as libc::c_uint,
                0 as libc::c_int != 0,
            );
        }
        if n2 == 0 as libc::c_int as libc::c_long {
            if n3 < 0 as libc::c_int as libc::c_long {
                return debug_make_int_type(
                    dhandle,
                    -n3 as libc::c_uint,
                    1 as libc::c_int != 0,
                )
            } else if n3 == 0xff as libc::c_int as libc::c_long {
                return debug_make_int_type(
                    dhandle,
                    1 as libc::c_int as libc::c_uint,
                    1 as libc::c_int != 0,
                )
            } else if n3 == 0xffff as libc::c_int as libc::c_long {
                return debug_make_int_type(
                    dhandle,
                    2 as libc::c_int as libc::c_uint,
                    1 as libc::c_int != 0,
                )
            } else if n3 == 0xffffffff as libc::c_uint as bfd_signed_vma {
                return debug_make_int_type(
                    dhandle,
                    4 as libc::c_int as libc::c_uint,
                    1 as libc::c_int != 0,
                )
            } else if n3 == 0xffffffffffffffff as libc::c_ulonglong as bfd_signed_vma {
                return debug_make_int_type(
                    dhandle,
                    8 as libc::c_int as libc::c_uint,
                    1 as libc::c_int != 0,
                )
            }
        } else if n3 == 0 as libc::c_int as libc::c_long
            && n2 < 0 as libc::c_int as libc::c_long
            && (self_subrange as libc::c_int != 0
                || n2 == -(8 as libc::c_int) as libc::c_long)
        {
            return debug_make_int_type(
                dhandle,
                -n2 as libc::c_uint,
                1 as libc::c_int != 0,
            )
        } else if n2 == -n3 - 1 as libc::c_int as libc::c_long
            || n2 == n3 + 1 as libc::c_int as libc::c_long
        {
            if n3 == 0x7f as libc::c_int as libc::c_long {
                return debug_make_int_type(
                    dhandle,
                    1 as libc::c_int as libc::c_uint,
                    0 as libc::c_int != 0,
                )
            } else if n3 == 0x7fff as libc::c_int as libc::c_long {
                return debug_make_int_type(
                    dhandle,
                    2 as libc::c_int as libc::c_uint,
                    0 as libc::c_int != 0,
                )
            } else if n3 == 0x7fffffff as libc::c_int as libc::c_long {
                return debug_make_int_type(
                    dhandle,
                    4 as libc::c_int as libc::c_uint,
                    0 as libc::c_int != 0,
                )
            } else if n3 as libc::c_ulong
                == (0x7fffffff as libc::c_int as bfd_vma) << 32 as libc::c_int
                    | 0xffffffff as libc::c_uint as libc::c_ulong
            {
                return debug_make_int_type(
                    dhandle,
                    8 as libc::c_int as libc::c_uint,
                    0 as libc::c_int != 0,
                )
            }
        }
    }
    if self_subrange {
        bad_stab(orig);
        return 0 as *mut libc::c_void as debug_type;
    }
    index_type = stab_find_type(dhandle, info, rangenums.as_mut_ptr());
    if index_type.is_null() {
        warn_stab(
            orig,
            dcgettext(
                0 as *const libc::c_char,
                b"missing index type\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        index_type = debug_make_int_type(
            dhandle,
            4 as libc::c_int as libc::c_uint,
            0 as libc::c_int != 0,
        );
    }
    return debug_make_range_type(dhandle, index_type, n2, n3);
}
unsafe extern "C" fn parse_stab_sun_builtin_type(
    mut dhandle: *mut libc::c_void,
    mut pp: *mut *const libc::c_char,
    mut p_end: *const libc::c_char,
) -> debug_type {
    let mut orig: *const libc::c_char = 0 as *const libc::c_char;
    let mut unsignedp: bool = false;
    let mut bits: bfd_vma = 0;
    orig = *pp;
    if orig >= p_end {
        return 0 as *mut libc::c_void as debug_type;
    }
    match **pp as libc::c_int {
        115 => {
            unsignedp = 0 as libc::c_int != 0;
        }
        117 => {
            unsignedp = 1 as libc::c_int != 0;
        }
        _ => {
            bad_stab(orig);
            return 0 as *mut libc::c_void as debug_type;
        }
    }
    *pp = (*pp).offset(1);
    let _ = *pp;
    if **pp as libc::c_int == 'c' as i32 || **pp as libc::c_int == 'b' as i32
        || **pp as libc::c_int == 'v' as i32
    {
        *pp = (*pp).offset(1);
        let _ = *pp;
    }
    parse_number(pp, 0 as *mut libc::c_void as *mut bool, p_end);
    if **pp as libc::c_int != ';' as i32 {
        bad_stab(orig);
        return 0 as *mut libc::c_void as debug_type;
    }
    *pp = (*pp).offset(1);
    let _ = *pp;
    parse_number(pp, 0 as *mut libc::c_void as *mut bool, p_end);
    if **pp as libc::c_int != ';' as i32 {
        bad_stab(orig);
        return 0 as *mut libc::c_void as debug_type;
    }
    *pp = (*pp).offset(1);
    let _ = *pp;
    bits = parse_number(pp, 0 as *mut libc::c_void as *mut bool, p_end);
    if **pp as libc::c_int == ';' as i32 {
        *pp = (*pp).offset(1);
        let _ = *pp;
    }
    if bits == 0 as libc::c_int as libc::c_ulong {
        return debug_make_void_type(dhandle);
    }
    return debug_make_int_type(
        dhandle,
        bits.wrapping_div(8 as libc::c_int as libc::c_ulong) as libc::c_uint,
        unsignedp,
    );
}
unsafe extern "C" fn parse_stab_sun_floating_type(
    mut dhandle: *mut libc::c_void,
    mut pp: *mut *const libc::c_char,
    mut p_end: *const libc::c_char,
) -> debug_type {
    let mut orig: *const libc::c_char = 0 as *const libc::c_char;
    let mut details: bfd_vma = 0;
    let mut bytes: bfd_vma = 0;
    orig = *pp;
    if orig >= p_end {
        return 0 as *mut libc::c_void as debug_type;
    }
    details = parse_number(pp, 0 as *mut libc::c_void as *mut bool, p_end);
    if **pp as libc::c_int != ';' as i32 {
        bad_stab(orig);
        return 0 as *mut libc::c_void as debug_type;
    }
    bytes = parse_number(pp, 0 as *mut libc::c_void as *mut bool, p_end);
    if **pp as libc::c_int != ';' as i32 {
        bad_stab(orig);
        return 0 as *mut libc::c_void as debug_type;
    }
    if details == 3 as libc::c_int as libc::c_ulong
        || details == 4 as libc::c_int as libc::c_ulong
        || details == 5 as libc::c_int as libc::c_ulong
    {
        return debug_make_complex_type(dhandle, bytes as libc::c_uint);
    }
    return debug_make_float_type(dhandle, bytes as libc::c_uint);
}
unsafe extern "C" fn parse_stab_enum_type(
    mut dhandle: *mut libc::c_void,
    mut pp: *mut *const libc::c_char,
    mut p_end: *const libc::c_char,
) -> debug_type {
    let mut orig: *const libc::c_char = 0 as *const libc::c_char;
    let mut names: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
    let mut values: *mut bfd_signed_vma = 0 as *mut bfd_signed_vma;
    let mut n: libc::c_uint = 0;
    let mut alloc: libc::c_uint = 0;
    orig = *pp;
    if orig >= p_end {
        return 0 as *mut libc::c_void as debug_type;
    }
    if **pp as libc::c_int == '-' as i32 {
        while **pp as libc::c_int != ':' as i32
            && **pp as libc::c_int != 0 as libc::c_int
        {
            *pp = (*pp).offset(1);
            let _ = *pp;
        }
        if **pp as libc::c_int == 0 as libc::c_int {
            bad_stab(orig);
            return 0 as *mut libc::c_void as debug_type;
        }
        *pp = (*pp).offset(1);
        let _ = *pp;
    }
    alloc = 10 as libc::c_int as libc::c_uint;
    names = xmalloc(
        (alloc as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*const libc::c_char>() as libc::c_ulong),
    ) as *mut *const libc::c_char;
    values = xmalloc(
        (alloc as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<bfd_signed_vma>() as libc::c_ulong),
    ) as *mut bfd_signed_vma;
    n = 0 as libc::c_int as libc::c_uint;
    while **pp as libc::c_int != '\0' as i32 && **pp as libc::c_int != ';' as i32
        && **pp as libc::c_int != ',' as i32
    {
        let mut p: *const libc::c_char = 0 as *const libc::c_char;
        let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut val: bfd_signed_vma = 0;
        p = *pp;
        while *p as libc::c_int != ':' as i32 && *p as libc::c_int != 0 as libc::c_int {
            p = p.offset(1);
            p;
        }
        if *p as libc::c_int == 0 as libc::c_int {
            bad_stab(orig);
            free(names as *mut libc::c_void);
            free(values as *mut libc::c_void);
            return 0 as *mut libc::c_void as debug_type;
        }
        name = savestring(*pp, p.offset_from(*pp) as libc::c_long as libc::c_int);
        *pp = p.offset(1 as libc::c_int as isize);
        val = parse_number(pp, 0 as *mut libc::c_void as *mut bool, p_end)
            as bfd_signed_vma;
        if **pp as libc::c_int != ',' as i32 {
            bad_stab(orig);
            free(name as *mut libc::c_void);
            free(names as *mut libc::c_void);
            free(values as *mut libc::c_void);
            return 0 as *mut libc::c_void as debug_type;
        }
        *pp = (*pp).offset(1);
        let _ = *pp;
        if n.wrapping_add(1 as libc::c_int as libc::c_uint) >= alloc {
            alloc = alloc.wrapping_add(10 as libc::c_int as libc::c_uint);
            names = xrealloc(
                names as *mut libc::c_void,
                (alloc as libc::c_ulong)
                    .wrapping_mul(
                        ::core::mem::size_of::<*const libc::c_char>() as libc::c_ulong,
                    ),
            ) as *mut *const libc::c_char;
            values = xrealloc(
                values as *mut libc::c_void,
                (alloc as libc::c_ulong)
                    .wrapping_mul(
                        ::core::mem::size_of::<bfd_signed_vma>() as libc::c_ulong,
                    ),
            ) as *mut bfd_signed_vma;
        }
        let ref mut fresh6 = *names.offset(n as isize);
        *fresh6 = name;
        *values.offset(n as isize) = val;
        n = n.wrapping_add(1);
        n;
    }
    let ref mut fresh7 = *names.offset(n as isize);
    *fresh7 = 0 as *const libc::c_char;
    *values.offset(n as isize) = 0 as libc::c_int as bfd_signed_vma;
    if **pp as libc::c_int == ';' as i32 {
        *pp = (*pp).offset(1);
        let _ = *pp;
    }
    return debug_make_enum_type(dhandle, names, values);
}
unsafe extern "C" fn parse_stab_struct_type(
    mut dhandle: *mut libc::c_void,
    mut info: *mut stab_handle,
    mut tagname: *const libc::c_char,
    mut pp: *mut *const libc::c_char,
    mut structp: bool,
    mut typenums: *const libc::c_int,
    mut p_end: *const libc::c_char,
) -> debug_type {
    let mut size: bfd_vma = 0;
    let mut baseclasses: *mut debug_baseclass = 0 as *mut debug_baseclass;
    let mut fields: *mut debug_field = 0 as *mut debug_field;
    let mut statics: bool = false;
    let mut methods: *mut debug_method = 0 as *mut debug_method;
    let mut vptrbase: debug_type = 0 as *mut debug_type_s;
    let mut ownvptr: bool = false;
    size = parse_number(pp, 0 as *mut libc::c_void as *mut bool, p_end);
    if !parse_stab_baseclasses(dhandle, info, pp, &mut baseclasses, p_end)
        || !parse_stab_struct_fields(dhandle, info, pp, &mut fields, &mut statics, p_end)
        || !parse_stab_members(dhandle, info, tagname, pp, typenums, &mut methods, p_end)
        || !parse_stab_tilde_field(
            dhandle,
            info,
            pp,
            typenums,
            &mut vptrbase,
            &mut ownvptr,
            p_end,
        )
    {
        free(fields as *mut libc::c_void);
        return 0 as *mut libc::c_void as debug_type;
    }
    if !statics && baseclasses.is_null() && methods.is_null() && vptrbase.is_null()
        && !ownvptr
    {
        return debug_make_struct_type(dhandle, structp, size, fields);
    }
    return debug_make_object_type(
        dhandle,
        structp,
        size,
        fields,
        baseclasses,
        methods,
        vptrbase,
        ownvptr,
    );
}
unsafe extern "C" fn parse_stab_baseclasses(
    mut dhandle: *mut libc::c_void,
    mut info: *mut stab_handle,
    mut pp: *mut *const libc::c_char,
    mut retp: *mut *mut debug_baseclass,
    mut p_end: *const libc::c_char,
) -> bool {
    let mut orig: *const libc::c_char = 0 as *const libc::c_char;
    let mut c: libc::c_uint = 0;
    let mut i: libc::c_uint = 0;
    let mut classes: *mut debug_baseclass = 0 as *mut debug_baseclass;
    *retp = 0 as *mut debug_baseclass;
    orig = *pp;
    if orig >= p_end {
        return 0 as libc::c_int != 0;
    }
    if **pp as libc::c_int != '!' as i32 {
        return 1 as libc::c_int != 0;
    }
    *pp = (*pp).offset(1);
    let _ = *pp;
    c = parse_number(pp, 0 as *mut libc::c_void as *mut bool, p_end) as libc::c_uint;
    if **pp as libc::c_int != ',' as i32 {
        bad_stab(orig);
        return 0 as libc::c_int != 0;
    }
    *pp = (*pp).offset(1);
    let _ = *pp;
    classes = xmalloc(
        (c.wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<debug_baseclass>() as libc::c_ulong),
    ) as *mut debug_baseclass;
    i = 0 as libc::c_int as libc::c_uint;
    while i < c {
        let mut is_virtual: bool = false;
        let mut visibility: debug_visibility = DEBUG_VISIBILITY_PUBLIC;
        let mut bitpos: bfd_vma = 0;
        let mut type_0: debug_type = 0 as *mut debug_type_s;
        match **pp as libc::c_int {
            48 => {
                is_virtual = 0 as libc::c_int != 0;
            }
            49 => {
                is_virtual = 1 as libc::c_int != 0;
            }
            0 => {
                bad_stab(orig);
                return 0 as libc::c_int != 0;
            }
            _ => {
                warn_stab(
                    orig,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"unknown virtual character for baseclass\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                is_virtual = 0 as libc::c_int != 0;
            }
        }
        *pp = (*pp).offset(1);
        let _ = *pp;
        match **pp as libc::c_int {
            48 => {
                visibility = DEBUG_VISIBILITY_PRIVATE;
            }
            49 => {
                visibility = DEBUG_VISIBILITY_PROTECTED;
            }
            50 => {
                visibility = DEBUG_VISIBILITY_PUBLIC;
            }
            0 => {
                bad_stab(orig);
                return 0 as libc::c_int != 0;
            }
            _ => {
                warn_stab(
                    orig,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"unknown visibility character for baseclass\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                visibility = DEBUG_VISIBILITY_PUBLIC;
            }
        }
        *pp = (*pp).offset(1);
        let _ = *pp;
        bitpos = parse_number(pp, 0 as *mut libc::c_void as *mut bool, p_end);
        if **pp as libc::c_int != ',' as i32 {
            bad_stab(orig);
            return 0 as libc::c_int != 0;
        }
        *pp = (*pp).offset(1);
        let _ = *pp;
        type_0 = parse_stab_type(
            dhandle,
            info,
            0 as *mut libc::c_void as *const libc::c_char,
            pp,
            0 as *mut libc::c_void as *mut *mut debug_type,
            p_end,
        );
        if type_0.is_null() {
            return 0 as libc::c_int != 0;
        }
        let ref mut fresh8 = *classes.offset(i as isize);
        *fresh8 = debug_make_baseclass(dhandle, type_0, bitpos, is_virtual, visibility);
        if (*classes.offset(i as isize)).is_null() {
            return 0 as libc::c_int != 0;
        }
        if **pp as libc::c_int != ';' as i32 {
            return 0 as libc::c_int != 0;
        }
        *pp = (*pp).offset(1);
        let _ = *pp;
        i = i.wrapping_add(1);
        i;
    }
    let ref mut fresh9 = *classes.offset(i as isize);
    *fresh9 = 0 as *mut libc::c_void as debug_baseclass;
    *retp = classes;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn parse_stab_struct_fields(
    mut dhandle: *mut libc::c_void,
    mut info: *mut stab_handle,
    mut pp: *mut *const libc::c_char,
    mut retp: *mut *mut debug_field,
    mut staticsp: *mut bool,
    mut p_end: *const libc::c_char,
) -> bool {
    let mut orig: *const libc::c_char = 0 as *const libc::c_char;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut fields: *mut debug_field = 0 as *mut debug_field;
    let mut c: libc::c_uint = 0;
    let mut alloc: libc::c_uint = 0;
    *retp = 0 as *mut debug_field;
    *staticsp = 0 as libc::c_int != 0;
    orig = *pp;
    if orig >= p_end {
        return 0 as libc::c_int != 0;
    }
    c = 0 as libc::c_int as libc::c_uint;
    alloc = 10 as libc::c_int as libc::c_uint;
    fields = xmalloc(
        (alloc as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<debug_field>() as libc::c_ulong),
    ) as *mut debug_field;
    while **pp as libc::c_int != ';' as i32 {
        p = *pp;
        if c.wrapping_add(1 as libc::c_int as libc::c_uint) >= alloc {
            alloc = alloc.wrapping_add(10 as libc::c_int as libc::c_uint);
            fields = xrealloc(
                fields as *mut libc::c_void,
                (alloc as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<debug_field>() as libc::c_ulong),
            ) as *mut debug_field;
        }
        if (*p as libc::c_int == '$' as i32 || *p as libc::c_int == '.' as i32)
            && *p.offset(1 as libc::c_int as isize) as libc::c_int != '_' as i32
        {
            *pp = (*pp).offset(1);
            let _ = *pp;
            if !parse_stab_cpp_abbrev(
                dhandle,
                info,
                pp,
                fields.offset(c as isize),
                p_end,
            ) {
                free(fields as *mut libc::c_void);
                return 0 as libc::c_int != 0;
            }
            c = c.wrapping_add(1);
            c;
        } else {
            p = strchr(p, ':' as i32);
            if p.is_null() {
                bad_stab(orig);
                free(fields as *mut libc::c_void);
                return 0 as libc::c_int != 0;
            }
            if *p.offset(1 as libc::c_int as isize) as libc::c_int == ':' as i32 {
                break;
            }
            if !parse_stab_one_struct_field(
                dhandle,
                info,
                pp,
                p,
                fields.offset(c as isize),
                staticsp,
                p_end,
            ) {
                return 0 as libc::c_int != 0;
            }
            c = c.wrapping_add(1);
            c;
        }
    }
    let ref mut fresh10 = *fields.offset(c as isize);
    *fresh10 = 0 as *mut libc::c_void as debug_field;
    *retp = fields;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn parse_stab_cpp_abbrev(
    mut dhandle: *mut libc::c_void,
    mut info: *mut stab_handle,
    mut pp: *mut *const libc::c_char,
    mut retp: *mut debug_field,
    mut p_end: *const libc::c_char,
) -> bool {
    let mut orig: *const libc::c_char = 0 as *const libc::c_char;
    let mut cpp_abbrev: libc::c_int = 0;
    let mut context: debug_type = 0 as *mut debug_type_s;
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut type_name: *const libc::c_char = 0 as *const libc::c_char;
    let mut type_0: debug_type = 0 as *mut debug_type_s;
    let mut bitpos: bfd_vma = 0;
    *retp = 0 as *mut libc::c_void as debug_field;
    orig = *pp;
    if orig >= p_end {
        return 0 as libc::c_int != 0;
    }
    if **pp as libc::c_int != 'v' as i32 {
        bad_stab(*pp);
        return 0 as libc::c_int != 0;
    }
    *pp = (*pp).offset(1);
    let _ = *pp;
    cpp_abbrev = **pp as libc::c_int;
    if cpp_abbrev == 0 as libc::c_int {
        bad_stab(orig);
        return 0 as libc::c_int != 0;
    }
    *pp = (*pp).offset(1);
    let _ = *pp;
    context = parse_stab_type(
        dhandle,
        info,
        0 as *mut libc::c_void as *const libc::c_char,
        pp,
        0 as *mut libc::c_void as *mut *mut debug_type,
        p_end,
    );
    if context.is_null() {
        return 0 as libc::c_int != 0;
    }
    match cpp_abbrev {
        102 => {
            name = b"_vptr$\0" as *const u8 as *const libc::c_char;
        }
        98 => {
            type_name = debug_get_type_name(dhandle, context);
            if type_name.is_null() {
                warn_stab(
                    orig,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"unnamed $vb type\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                type_name = b"FOO\0" as *const u8 as *const libc::c_char;
            }
            name = concat(
                b"_vb$\0" as *const u8 as *const libc::c_char,
                type_name,
                0 as *mut libc::c_void as *const libc::c_char,
            );
        }
        _ => {
            warn_stab(
                orig,
                dcgettext(
                    0 as *const libc::c_char,
                    b"unrecognized C++ abbreviation\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            name = b"INVALID_CPLUSPLUS_ABBREV\0" as *const u8 as *const libc::c_char;
        }
    }
    if **pp as libc::c_int != ':' as i32 {
        bad_stab(orig);
        return 0 as libc::c_int != 0;
    }
    *pp = (*pp).offset(1);
    let _ = *pp;
    type_0 = parse_stab_type(
        dhandle,
        info,
        0 as *mut libc::c_void as *const libc::c_char,
        pp,
        0 as *mut libc::c_void as *mut *mut debug_type,
        p_end,
    );
    if **pp as libc::c_int != ',' as i32 {
        bad_stab(orig);
        return 0 as libc::c_int != 0;
    }
    *pp = (*pp).offset(1);
    let _ = *pp;
    bitpos = parse_number(pp, 0 as *mut libc::c_void as *mut bool, p_end);
    if **pp as libc::c_int != ';' as i32 {
        bad_stab(orig);
        return 0 as libc::c_int != 0;
    }
    *pp = (*pp).offset(1);
    let _ = *pp;
    *retp = debug_make_field(
        dhandle,
        name,
        type_0,
        bitpos,
        0 as libc::c_int as bfd_vma,
        DEBUG_VISIBILITY_PRIVATE,
    );
    if (*retp).is_null() {
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn parse_stab_one_struct_field(
    mut dhandle: *mut libc::c_void,
    mut info: *mut stab_handle,
    mut pp: *mut *const libc::c_char,
    mut p: *const libc::c_char,
    mut retp: *mut debug_field,
    mut staticsp: *mut bool,
    mut p_end: *const libc::c_char,
) -> bool {
    let mut orig: *const libc::c_char = 0 as *const libc::c_char;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut visibility: debug_visibility = DEBUG_VISIBILITY_PUBLIC;
    let mut type_0: debug_type = 0 as *mut debug_type_s;
    let mut bitpos: bfd_vma = 0;
    let mut bitsize: bfd_vma = 0;
    orig = *pp;
    if orig >= p_end {
        return 0 as libc::c_int != 0;
    }
    name = savestring(*pp, p.offset_from(*pp) as libc::c_long as libc::c_int);
    *pp = p.offset(1 as libc::c_int as isize);
    if **pp as libc::c_int != '/' as i32 {
        visibility = DEBUG_VISIBILITY_PUBLIC;
    } else {
        *pp = (*pp).offset(1);
        let _ = *pp;
        match **pp as libc::c_int {
            48 => {
                visibility = DEBUG_VISIBILITY_PRIVATE;
            }
            49 => {
                visibility = DEBUG_VISIBILITY_PROTECTED;
            }
            50 => {
                visibility = DEBUG_VISIBILITY_PUBLIC;
            }
            0 => {
                bad_stab(orig);
                return 0 as libc::c_int != 0;
            }
            _ => {
                warn_stab(
                    orig,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"unknown visibility character for field\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                visibility = DEBUG_VISIBILITY_PUBLIC;
            }
        }
        *pp = (*pp).offset(1);
        let _ = *pp;
    }
    type_0 = parse_stab_type(
        dhandle,
        info,
        0 as *mut libc::c_void as *const libc::c_char,
        pp,
        0 as *mut libc::c_void as *mut *mut debug_type,
        p_end,
    );
    if type_0.is_null() {
        free(name as *mut libc::c_void);
        return 0 as libc::c_int != 0;
    }
    if **pp as libc::c_int == ':' as i32 {
        let mut varname: *mut libc::c_char = 0 as *mut libc::c_char;
        *pp = (*pp).offset(1);
        let _ = *pp;
        p = strchr(*pp, ';' as i32);
        if p.is_null() {
            bad_stab(orig);
            free(name as *mut libc::c_void);
            return 0 as libc::c_int != 0;
        }
        varname = savestring(*pp, p.offset_from(*pp) as libc::c_long as libc::c_int);
        *pp = p.offset(1 as libc::c_int as isize);
        *retp = debug_make_static_member(dhandle, name, type_0, varname, visibility);
        *staticsp = 1 as libc::c_int != 0;
        return 1 as libc::c_int != 0;
    }
    if **pp as libc::c_int != ',' as i32 {
        bad_stab(orig);
        free(name as *mut libc::c_void);
        return 0 as libc::c_int != 0;
    }
    *pp = (*pp).offset(1);
    let _ = *pp;
    bitpos = parse_number(pp, 0 as *mut libc::c_void as *mut bool, p_end);
    if **pp as libc::c_int != ',' as i32 {
        bad_stab(orig);
        free(name as *mut libc::c_void);
        return 0 as libc::c_int != 0;
    }
    *pp = (*pp).offset(1);
    let _ = *pp;
    bitsize = parse_number(pp, 0 as *mut libc::c_void as *mut bool, p_end);
    if **pp as libc::c_int != ';' as i32 {
        bad_stab(orig);
        free(name as *mut libc::c_void);
        return 0 as libc::c_int != 0;
    }
    *pp = (*pp).offset(1);
    let _ = *pp;
    if bitpos == 0 as libc::c_int as libc::c_ulong
        && bitsize == 0 as libc::c_int as libc::c_ulong
    {
        visibility = DEBUG_VISIBILITY_IGNORE;
    }
    *retp = debug_make_field(dhandle, name, type_0, bitpos, bitsize, visibility);
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn parse_stab_members(
    mut dhandle: *mut libc::c_void,
    mut info: *mut stab_handle,
    mut tagname: *const libc::c_char,
    mut pp: *mut *const libc::c_char,
    mut typenums: *const libc::c_int,
    mut retp: *mut *mut debug_method,
    mut p_end: *const libc::c_char,
) -> bool {
    let mut current_block: u64;
    let mut orig: *const libc::c_char = 0 as *const libc::c_char;
    let mut methods: *mut debug_method = 0 as *mut debug_method;
    let mut c: libc::c_uint = 0;
    let mut alloc: libc::c_uint = 0;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut variants: *mut debug_method_variant = 0 as *mut debug_method_variant;
    let mut argtypes: *mut libc::c_char = 0 as *mut libc::c_char;
    *retp = 0 as *mut debug_method;
    orig = *pp;
    if orig >= p_end {
        return 0 as libc::c_int != 0;
    }
    alloc = 0 as libc::c_int as libc::c_uint;
    methods = 0 as *mut debug_method;
    c = 0 as libc::c_int as libc::c_uint;
    's_37: loop {
        if !(**pp as libc::c_int != ';' as i32) {
            current_block = 5404178929002277151;
            break;
        }
        let mut p: *const libc::c_char = 0 as *const libc::c_char;
        let mut cvars: libc::c_uint = 0;
        let mut allocvars: libc::c_uint = 0;
        let mut look_ahead_type: debug_type = 0 as *mut debug_type_s;
        p = strchr(*pp, ':' as i32);
        if p.is_null()
            || *p.offset(1 as libc::c_int as isize) as libc::c_int != ':' as i32
        {
            current_block = 5404178929002277151;
            break;
        }
        if *(*pp).offset(0 as libc::c_int as isize) as libc::c_int != 'o' as i32
            || *(*pp).offset(1 as libc::c_int as isize) as libc::c_int != 'p' as i32
            || *(*pp).offset(2 as libc::c_int as isize) as libc::c_int != '$' as i32
        {
            name = savestring(*pp, p.offset_from(*pp) as libc::c_long as libc::c_int);
            *pp = p.offset(2 as libc::c_int as isize);
        } else {
            *pp = p.offset(2 as libc::c_int as isize);
            p = *pp;
            while *p as libc::c_int != '.' as i32 && *p as libc::c_int != '\0' as i32 {
                p = p.offset(1);
                p;
            }
            if *p as libc::c_int != '.' as i32 {
                bad_stab(orig);
                current_block = 2243640431624444939;
                break;
            } else {
                name = savestring(
                    *pp,
                    p.offset_from(*pp) as libc::c_long as libc::c_int,
                );
                *pp = p.offset(1 as libc::c_int as isize);
            }
        }
        allocvars = 10 as libc::c_int as libc::c_uint;
        variants = xmalloc(
            (allocvars as libc::c_ulong)
                .wrapping_mul(
                    ::core::mem::size_of::<debug_method_variant>() as libc::c_ulong,
                ),
        ) as *mut debug_method_variant;
        cvars = 0 as libc::c_int as libc::c_uint;
        look_ahead_type = 0 as *mut libc::c_void as debug_type;
        loop {
            let mut type_0: debug_type = 0 as *mut debug_type_s;
            let mut stub: bool = false;
            let mut visibility: debug_visibility = DEBUG_VISIBILITY_PUBLIC;
            let mut constp: bool = false;
            let mut volatilep: bool = false;
            let mut staticp: bool = false;
            let mut voffset: bfd_vma = 0;
            let mut context: debug_type = 0 as *mut debug_type_s;
            let mut physname: *const libc::c_char = 0 as *const libc::c_char;
            let mut varargs: bool = false;
            if !look_ahead_type.is_null() {
                type_0 = look_ahead_type;
                look_ahead_type = 0 as *mut libc::c_void as debug_type;
            } else {
                type_0 = parse_stab_type(
                    dhandle,
                    info,
                    0 as *mut libc::c_void as *const libc::c_char,
                    pp,
                    0 as *mut libc::c_void as *mut *mut debug_type,
                    p_end,
                );
                if type_0.is_null() {
                    current_block = 2243640431624444939;
                    break 's_37;
                }
                if **pp as libc::c_int != ':' as i32 {
                    bad_stab(orig);
                    current_block = 2243640431624444939;
                    break 's_37;
                }
            }
            *pp = (*pp).offset(1);
            let _ = *pp;
            p = strchr(*pp, ';' as i32);
            if p.is_null() {
                bad_stab(orig);
                current_block = 2243640431624444939;
                break 's_37;
            } else {
                stub = 0 as libc::c_int != 0;
                if debug_get_type_kind(dhandle, type_0) as libc::c_uint
                    == DEBUG_KIND_METHOD as libc::c_int as libc::c_uint
                    && (debug_get_parameter_types(dhandle, type_0, &mut varargs))
                        .is_null()
                {
                    stub = 1 as libc::c_int != 0;
                }
                argtypes = savestring(
                    *pp,
                    p.offset_from(*pp) as libc::c_long as libc::c_int,
                );
                *pp = p.offset(1 as libc::c_int as isize);
                match **pp as libc::c_int {
                    48 => {
                        visibility = DEBUG_VISIBILITY_PRIVATE;
                    }
                    49 => {
                        visibility = DEBUG_VISIBILITY_PROTECTED;
                    }
                    0 => {
                        bad_stab(orig);
                        current_block = 2243640431624444939;
                        break 's_37;
                    }
                    _ => {
                        visibility = DEBUG_VISIBILITY_PUBLIC;
                    }
                }
                *pp = (*pp).offset(1);
                let _ = *pp;
                constp = 0 as libc::c_int != 0;
                volatilep = 0 as libc::c_int != 0;
                match **pp as libc::c_int {
                    65 => {
                        *pp = (*pp).offset(1);
                        let _ = *pp;
                    }
                    66 => {
                        constp = 1 as libc::c_int != 0;
                        *pp = (*pp).offset(1);
                        let _ = *pp;
                    }
                    67 => {
                        volatilep = 1 as libc::c_int != 0;
                        *pp = (*pp).offset(1);
                        let _ = *pp;
                    }
                    68 => {
                        constp = 1 as libc::c_int != 0;
                        volatilep = 1 as libc::c_int != 0;
                        *pp = (*pp).offset(1);
                        let _ = *pp;
                    }
                    42 | 63 | 46 => {}
                    _ => {
                        warn_stab(
                            orig,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"const/volatile indicator missing\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                    }
                }
                staticp = 0 as libc::c_int != 0;
                match **pp as libc::c_int {
                    42 => {
                        *pp = (*pp).offset(1);
                        let _ = *pp;
                        voffset = parse_number(
                            pp,
                            0 as *mut libc::c_void as *mut bool,
                            p_end,
                        );
                        if **pp as libc::c_int != ';' as i32 {
                            bad_stab(orig);
                            current_block = 2243640431624444939;
                            break 's_37;
                        } else {
                            *pp = (*pp).offset(1);
                            let _ = *pp;
                            voffset &= 0x7fffffff as libc::c_int as libc::c_ulong;
                            if **pp as libc::c_int == ';' as i32
                                || **pp as libc::c_int == '\0' as i32
                            {
                                context = 0 as *mut libc::c_void as debug_type;
                            } else {
                                look_ahead_type = parse_stab_type(
                                    dhandle,
                                    info,
                                    0 as *mut libc::c_void as *const libc::c_char,
                                    pp,
                                    0 as *mut libc::c_void as *mut *mut debug_type,
                                    p_end,
                                );
                                if **pp as libc::c_int == ':' as i32 {
                                    context = 0 as *mut libc::c_void as debug_type;
                                } else {
                                    context = look_ahead_type;
                                    look_ahead_type = 0 as *mut libc::c_void as debug_type;
                                    if **pp as libc::c_int != ';' as i32 {
                                        bad_stab(orig);
                                        current_block = 2243640431624444939;
                                        break 's_37;
                                    } else {
                                        *pp = (*pp).offset(1);
                                        let _ = *pp;
                                    }
                                }
                            }
                        }
                    }
                    63 => {
                        *pp = (*pp).offset(1);
                        let _ = *pp;
                        staticp = 1 as libc::c_int != 0;
                        voffset = 0 as libc::c_int as bfd_vma;
                        context = 0 as *mut libc::c_void as debug_type;
                        if strncmp(argtypes, name, strlen(name)) != 0 as libc::c_int {
                            stub = 1 as libc::c_int != 0;
                        }
                    }
                    46 => {
                        *pp = (*pp).offset(1);
                        let _ = *pp;
                        voffset = 0 as libc::c_int as bfd_vma;
                        context = 0 as *mut libc::c_void as debug_type;
                    }
                    _ => {
                        warn_stab(
                            orig,
                            b"member function type missing\0" as *const u8
                                as *const libc::c_char,
                        );
                        voffset = 0 as libc::c_int as bfd_vma;
                        context = 0 as *mut libc::c_void as debug_type;
                    }
                }
                physname = argtypes;
                if stub {
                    let mut class_type: debug_type = 0 as *mut debug_type_s;
                    let mut return_type: debug_type = 0 as *mut debug_type_s;
                    class_type = stab_find_type(dhandle, info, typenums);
                    if class_type.is_null() {
                        current_block = 2243640431624444939;
                        break 's_37;
                    }
                    return_type = debug_get_return_type(dhandle, type_0);
                    if return_type.is_null() {
                        bad_stab(orig);
                        current_block = 2243640431624444939;
                        break 's_37;
                    } else {
                        type_0 = parse_stab_argtypes(
                            dhandle,
                            info,
                            class_type,
                            name,
                            tagname,
                            return_type,
                            argtypes,
                            constp,
                            volatilep,
                            &mut physname,
                        );
                        if type_0.is_null() {
                            current_block = 2243640431624444939;
                            break 's_37;
                        }
                    }
                }
                if cvars.wrapping_add(1 as libc::c_int as libc::c_uint) >= allocvars {
                    allocvars = allocvars
                        .wrapping_add(10 as libc::c_int as libc::c_uint);
                    variants = xrealloc(
                        variants as *mut libc::c_void,
                        (allocvars as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<debug_method_variant>()
                                    as libc::c_ulong,
                            ),
                    ) as *mut debug_method_variant;
                }
                if !staticp {
                    let ref mut fresh11 = *variants.offset(cvars as isize);
                    *fresh11 = debug_make_method_variant(
                        dhandle,
                        physname,
                        type_0,
                        visibility,
                        constp,
                        volatilep,
                        voffset,
                        context,
                    );
                } else {
                    let ref mut fresh12 = *variants.offset(cvars as isize);
                    *fresh12 = debug_make_static_method_variant(
                        dhandle,
                        physname,
                        type_0,
                        visibility,
                        constp,
                        volatilep,
                    );
                }
                if (*variants.offset(cvars as isize)).is_null() {
                    current_block = 2243640431624444939;
                    break 's_37;
                }
                cvars = cvars.wrapping_add(1);
                cvars;
                if !(**pp as libc::c_int != ';' as i32
                    && **pp as libc::c_int != '\0' as i32)
                {
                    break;
                }
            }
        }
        let ref mut fresh13 = *variants.offset(cvars as isize);
        *fresh13 = 0 as *mut libc::c_void as debug_method_variant;
        if **pp as libc::c_int != '\0' as i32 {
            *pp = (*pp).offset(1);
            let _ = *pp;
        }
        if c.wrapping_add(1 as libc::c_int as libc::c_uint) >= alloc {
            alloc = alloc.wrapping_add(10 as libc::c_int as libc::c_uint);
            methods = xrealloc(
                methods as *mut libc::c_void,
                (alloc as libc::c_ulong)
                    .wrapping_mul(
                        ::core::mem::size_of::<debug_method>() as libc::c_ulong,
                    ),
            ) as *mut debug_method;
        }
        let ref mut fresh14 = *methods.offset(c as isize);
        *fresh14 = debug_make_method(dhandle, name, variants);
        c = c.wrapping_add(1);
        c;
    }
    match current_block {
        2243640431624444939 => {
            free(name as *mut libc::c_void);
            free(variants as *mut libc::c_void);
            free(argtypes as *mut libc::c_void);
            return 0 as libc::c_int != 0;
        }
        _ => {
            if !methods.is_null() {
                let ref mut fresh15 = *methods.offset(c as isize);
                *fresh15 = 0 as *mut libc::c_void as debug_method;
            }
            *retp = methods;
            return 1 as libc::c_int != 0;
        }
    };
}
unsafe extern "C" fn parse_stab_argtypes(
    mut dhandle: *mut libc::c_void,
    mut info: *mut stab_handle,
    mut class_type: debug_type,
    mut fieldname: *const libc::c_char,
    mut tagname: *const libc::c_char,
    mut return_type: debug_type,
    mut argtypes: *const libc::c_char,
    mut constp: bool,
    mut volatilep: bool,
    mut pphysname: *mut *const libc::c_char,
) -> debug_type {
    let mut is_full_physname_constructor: bool = false;
    let mut is_constructor: bool = false;
    let mut is_destructor: bool = false;
    let mut is_v3: bool = false;
    let mut args: *mut debug_type = 0 as *mut debug_type;
    let mut varargs: bool = false;
    let mut physname_len: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    is_full_physname_constructor = *argtypes.offset(0 as libc::c_int as isize)
        as libc::c_int == '_' as i32
        && *argtypes.offset(1 as libc::c_int as isize) as libc::c_int == '_' as i32
        && (_sch_istable[(*argtypes.offset(2 as libc::c_int as isize) as libc::c_int
            & 0xff as libc::c_int) as usize] as libc::c_int
            & _sch_isdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
            || *argtypes.offset(2 as libc::c_int as isize) as libc::c_int == 'Q' as i32
            || *argtypes.offset(2 as libc::c_int as isize) as libc::c_int == 't' as i32)
        || startswith(argtypes, b"__ct\0" as *const u8 as *const libc::c_char)
            as libc::c_int != 0;
    is_constructor = is_full_physname_constructor as libc::c_int != 0
        || !tagname.is_null() && strcmp(fieldname, tagname) == 0 as libc::c_int;
    is_destructor = *argtypes.offset(0 as libc::c_int as isize) as libc::c_int
        == '_' as i32
        && (*argtypes.offset(1 as libc::c_int as isize) as libc::c_int == '$' as i32
            || *argtypes.offset(1 as libc::c_int as isize) as libc::c_int == '.' as i32)
        && *argtypes.offset(2 as libc::c_int as isize) as libc::c_int == '_' as i32
        || startswith(argtypes, b"__dt\0" as *const u8 as *const libc::c_char)
            as libc::c_int != 0;
    is_v3 = *argtypes.offset(0 as libc::c_int as isize) as libc::c_int == '_' as i32
        && *argtypes.offset(1 as libc::c_int as isize) as libc::c_int == 'Z' as i32;
    if !(is_destructor as libc::c_int != 0
        || is_full_physname_constructor as libc::c_int != 0 || is_v3 as libc::c_int != 0)
    {
        let mut len: libc::c_uint = 0;
        let mut const_prefix: *const libc::c_char = 0 as *const libc::c_char;
        let mut volatile_prefix: *const libc::c_char = 0 as *const libc::c_char;
        let mut buf: [libc::c_char; 20] = [0; 20];
        let mut mangled_name_len: libc::c_uint = 0;
        let mut physname: *mut libc::c_char = 0 as *mut libc::c_char;
        len = (if tagname.is_null() {
            0 as libc::c_int as libc::c_ulong
        } else {
            strlen(tagname)
        }) as libc::c_uint;
        const_prefix = if constp as libc::c_int != 0 {
            b"C\0" as *const u8 as *const libc::c_char
        } else {
            b"\0" as *const u8 as *const libc::c_char
        };
        volatile_prefix = if volatilep as libc::c_int != 0 {
            b"V\0" as *const u8 as *const libc::c_char
        } else {
            b"\0" as *const u8 as *const libc::c_char
        };
        if len == 0 as libc::c_int as libc::c_uint {
            sprintf(
                buf.as_mut_ptr(),
                b"__%s%s\0" as *const u8 as *const libc::c_char,
                const_prefix,
                volatile_prefix,
            );
        } else if !tagname.is_null() && !(strchr(tagname, '<' as i32)).is_null() {
            sprintf(
                buf.as_mut_ptr(),
                b"__%s%s\0" as *const u8 as *const libc::c_char,
                const_prefix,
                volatile_prefix,
            );
            tagname = 0 as *const libc::c_char;
            len = 0 as libc::c_int as libc::c_uint;
        } else {
            sprintf(
                buf.as_mut_ptr(),
                b"__%s%s%d\0" as *const u8 as *const libc::c_char,
                const_prefix,
                volatile_prefix,
                len,
            );
        }
        mangled_name_len = (if is_constructor as libc::c_int != 0 {
            0 as libc::c_int as libc::c_ulong
        } else {
            strlen(fieldname)
        })
            .wrapping_add(strlen(buf.as_mut_ptr()))
            .wrapping_add(len as libc::c_ulong)
            .wrapping_add(strlen(argtypes))
            .wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_uint;
        if *fieldname.offset(0 as libc::c_int as isize) as libc::c_int == 'o' as i32
            && *fieldname.offset(1 as libc::c_int as isize) as libc::c_int == 'p' as i32
            && (*fieldname.offset(2 as libc::c_int as isize) as libc::c_int == '$' as i32
                || *fieldname.offset(2 as libc::c_int as isize) as libc::c_int
                    == '.' as i32)
        {
            return 0 as *mut libc::c_void as debug_type;
        }
        physname = xmalloc(mangled_name_len as size_t) as *mut libc::c_char;
        if is_constructor {
            *physname.offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
        } else {
            strcpy(physname, fieldname);
        }
        physname_len = strlen(physname) as libc::c_uint;
        strcat(physname, buf.as_mut_ptr());
        if !tagname.is_null() {
            strcat(physname, tagname);
        }
        strcat(physname, argtypes);
        *pphysname = physname;
    }
    if *argtypes as libc::c_int == '\0' as i32 || is_destructor as libc::c_int != 0 {
        args = xmalloc(::core::mem::size_of::<debug_type>() as libc::c_ulong)
            as *mut debug_type;
        *args = 0 as debug_type;
        return debug_make_method_type(
            dhandle,
            return_type,
            class_type,
            args,
            0 as libc::c_int != 0,
        );
    }
    args = stab_demangle_argtypes(dhandle, info, *pphysname, &mut varargs, physname_len);
    if args.is_null() {
        return 0 as *mut libc::c_void as debug_type;
    }
    return debug_make_method_type(dhandle, return_type, class_type, args, varargs);
}
unsafe extern "C" fn parse_stab_tilde_field(
    mut dhandle: *mut libc::c_void,
    mut info: *mut stab_handle,
    mut pp: *mut *const libc::c_char,
    mut typenums: *const libc::c_int,
    mut retvptrbase: *mut debug_type,
    mut retownvptr: *mut bool,
    mut p_end: *const libc::c_char,
) -> bool {
    let mut orig: *const libc::c_char = 0 as *const libc::c_char;
    let mut hold: *const libc::c_char = 0 as *const libc::c_char;
    let mut vtypenums: [libc::c_int; 2] = [0; 2];
    *retvptrbase = 0 as *mut libc::c_void as debug_type;
    *retownvptr = 0 as libc::c_int != 0;
    orig = *pp;
    if orig >= p_end {
        return 0 as libc::c_int != 0;
    }
    if **pp as libc::c_int == ';' as i32 {
        *pp = (*pp).offset(1);
        let _ = *pp;
    }
    if **pp as libc::c_int != '~' as i32 {
        return 1 as libc::c_int != 0;
    }
    *pp = (*pp).offset(1);
    let _ = *pp;
    if **pp as libc::c_int == '=' as i32 || **pp as libc::c_int == '+' as i32
        || **pp as libc::c_int == '-' as i32
    {
        *pp = (*pp).offset(1);
        let _ = *pp;
    }
    if **pp as libc::c_int != '%' as i32 {
        return 1 as libc::c_int != 0;
    }
    *pp = (*pp).offset(1);
    let _ = *pp;
    hold = *pp;
    if !parse_stab_type_number(pp, vtypenums.as_mut_ptr(), p_end) {
        return 0 as libc::c_int != 0;
    }
    if vtypenums[0 as libc::c_int as usize]
        == *typenums.offset(0 as libc::c_int as isize)
        && vtypenums[1 as libc::c_int as usize]
            == *typenums.offset(1 as libc::c_int as isize)
    {
        *retownvptr = 1 as libc::c_int != 0;
    } else {
        let mut vtype: debug_type = 0 as *mut debug_type_s;
        let mut p: *const libc::c_char = 0 as *const libc::c_char;
        *pp = hold;
        vtype = parse_stab_type(
            dhandle,
            info,
            0 as *mut libc::c_void as *const libc::c_char,
            pp,
            0 as *mut libc::c_void as *mut *mut debug_type,
            p_end,
        );
        p = *pp;
        while *p as libc::c_int != ';' as i32 && *p as libc::c_int != '\0' as i32 {
            p = p.offset(1);
            p;
        }
        if *p as libc::c_int != ';' as i32 {
            bad_stab(orig);
            return 0 as libc::c_int != 0;
        }
        *retvptrbase = vtype;
        *pp = p.offset(1 as libc::c_int as isize);
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn parse_stab_array_type(
    mut dhandle: *mut libc::c_void,
    mut info: *mut stab_handle,
    mut pp: *mut *const libc::c_char,
    mut stringp: bool,
    mut p_end: *const libc::c_char,
) -> debug_type {
    let mut orig: *const libc::c_char = 0 as *const libc::c_char;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut typenums: [libc::c_int; 2] = [0; 2];
    let mut index_type: debug_type = 0 as *mut debug_type_s;
    let mut adjustable: bool = false;
    let mut lower: bfd_signed_vma = 0;
    let mut upper: bfd_signed_vma = 0;
    let mut element_type: debug_type = 0 as *mut debug_type_s;
    orig = *pp;
    if orig >= p_end {
        return 0 as *mut libc::c_void as debug_type;
    }
    p = *pp;
    if !parse_stab_type_number(&mut p, typenums.as_mut_ptr(), p_end) {
        return 0 as *mut libc::c_void as debug_type;
    }
    if typenums[0 as libc::c_int as usize] == 0 as libc::c_int
        && typenums[1 as libc::c_int as usize] == 0 as libc::c_int
        && **pp as libc::c_int != '=' as i32
    {
        index_type = debug_find_named_type(
            dhandle,
            b"int\0" as *const u8 as *const libc::c_char,
        );
        if index_type.is_null() {
            index_type = debug_make_int_type(
                dhandle,
                4 as libc::c_int as libc::c_uint,
                0 as libc::c_int != 0,
            );
            if index_type.is_null() {
                return 0 as *mut libc::c_void as debug_type;
            }
        }
        *pp = p;
    } else {
        index_type = parse_stab_type(
            dhandle,
            info,
            0 as *mut libc::c_void as *const libc::c_char,
            pp,
            0 as *mut libc::c_void as *mut *mut debug_type,
            p_end,
        );
    }
    if **pp as libc::c_int != ';' as i32 {
        bad_stab(orig);
        return 0 as *mut libc::c_void as debug_type;
    }
    *pp = (*pp).offset(1);
    let _ = *pp;
    adjustable = 0 as libc::c_int != 0;
    if _sch_istable[(**pp as libc::c_int & 0xff as libc::c_int) as usize] as libc::c_int
        & _sch_isdigit as libc::c_int as libc::c_ushort as libc::c_int == 0
        && **pp as libc::c_int != '-' as i32 && **pp as libc::c_int != 0 as libc::c_int
    {
        *pp = (*pp).offset(1);
        let _ = *pp;
        adjustable = 1 as libc::c_int != 0;
    }
    lower = parse_number(pp, 0 as *mut libc::c_void as *mut bool, p_end)
        as bfd_signed_vma;
    if **pp as libc::c_int != ';' as i32 {
        bad_stab(orig);
        return 0 as *mut libc::c_void as debug_type;
    }
    *pp = (*pp).offset(1);
    let _ = *pp;
    if _sch_istable[(**pp as libc::c_int & 0xff as libc::c_int) as usize] as libc::c_int
        & _sch_isdigit as libc::c_int as libc::c_ushort as libc::c_int == 0
        && **pp as libc::c_int != '-' as i32 && **pp as libc::c_int != 0 as libc::c_int
    {
        *pp = (*pp).offset(1);
        let _ = *pp;
        adjustable = 1 as libc::c_int != 0;
    }
    upper = parse_number(pp, 0 as *mut libc::c_void as *mut bool, p_end)
        as bfd_signed_vma;
    if **pp as libc::c_int != ';' as i32 {
        bad_stab(orig);
        return 0 as *mut libc::c_void as debug_type;
    }
    *pp = (*pp).offset(1);
    let _ = *pp;
    element_type = parse_stab_type(
        dhandle,
        info,
        0 as *mut libc::c_void as *const libc::c_char,
        pp,
        0 as *mut libc::c_void as *mut *mut debug_type,
        p_end,
    );
    if element_type.is_null() {
        return 0 as *mut libc::c_void as debug_type;
    }
    if adjustable {
        lower = 0 as libc::c_int as bfd_signed_vma;
        upper = -(1 as libc::c_int) as bfd_signed_vma;
    }
    return debug_make_array_type(
        dhandle,
        element_type,
        index_type,
        lower,
        upper,
        stringp,
    );
}
unsafe extern "C" fn push_bincl(
    mut info: *mut stab_handle,
    mut name: *const libc::c_char,
    mut hash: bfd_vma,
) {
    let mut n: *mut bincl_file = 0 as *mut bincl_file;
    n = xmalloc(::core::mem::size_of::<bincl_file>() as libc::c_ulong)
        as *mut bincl_file;
    (*n).next = (*info).bincl_list;
    (*n).next_stack = (*info).bincl_stack;
    (*n).name = name;
    (*n).hash = hash;
    (*n).file = (*info).files;
    (*n).file_types = 0 as *mut stab_types;
    (*info).bincl_list = n;
    (*info).bincl_stack = n;
    (*info).files = ((*info).files).wrapping_add(1);
    (*info).files;
    (*info)
        .file_types = xrealloc(
        (*info).file_types as *mut libc::c_void,
        ((*info).files as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut stab_types>() as libc::c_ulong),
    ) as *mut *mut stab_types;
    let ref mut fresh16 = *((*info).file_types).offset((*n).file as isize);
    *fresh16 = 0 as *mut stab_types;
}
unsafe extern "C" fn pop_bincl(mut info: *mut stab_handle) -> *const libc::c_char {
    let mut o: *mut bincl_file = 0 as *mut bincl_file;
    o = (*info).bincl_stack;
    if o.is_null() {
        return (*info).main_filename;
    }
    (*info).bincl_stack = (*o).next_stack;
    if (*o).file >= (*info).files {
        return (*info).main_filename;
    }
    (*o).file_types = *((*info).file_types).offset((*o).file as isize);
    if ((*info).bincl_stack).is_null() {
        return (*info).main_filename;
    }
    return (*(*info).bincl_stack).name;
}
unsafe extern "C" fn find_excl(
    mut info: *mut stab_handle,
    mut name: *const libc::c_char,
    mut hash: bfd_vma,
) -> bool {
    let mut l: *mut bincl_file = 0 as *mut bincl_file;
    (*info).files = ((*info).files).wrapping_add(1);
    (*info).files;
    (*info)
        .file_types = xrealloc(
        (*info).file_types as *mut libc::c_void,
        ((*info).files as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut stab_types>() as libc::c_ulong),
    ) as *mut *mut stab_types;
    l = (*info).bincl_list;
    while !l.is_null() {
        if (*l).hash == hash && strcmp((*l).name, name) == 0 as libc::c_int {
            break;
        }
        l = (*l).next;
    }
    if l.is_null() {
        warn_stab(
            name,
            dcgettext(
                0 as *const libc::c_char,
                b"Undefined N_EXCL\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        let ref mut fresh17 = *((*info).file_types)
            .offset(
                ((*info).files).wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
            );
        *fresh17 = 0 as *mut stab_types;
        return 1 as libc::c_int != 0;
    }
    let ref mut fresh18 = *((*info).file_types)
        .offset(((*info).files).wrapping_sub(1 as libc::c_int as libc::c_uint) as isize);
    *fresh18 = (*l).file_types;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn stab_record_variable(
    mut dhandle: *mut libc::c_void,
    mut info: *mut stab_handle,
    mut name: *const libc::c_char,
    mut type_0: debug_type,
    mut kind: debug_var_kind,
    mut val: bfd_vma,
) -> bool {
    let mut v: *mut stab_pending_var = 0 as *mut stab_pending_var;
    if kind as libc::c_uint == DEBUG_GLOBAL as libc::c_int as libc::c_uint
        || kind as libc::c_uint == DEBUG_STATIC as libc::c_int as libc::c_uint
        || !(*info).within_function
        || (*info).gcc_compiled == 0 as libc::c_int
            && (*info).n_opt_found as libc::c_int != 0
    {
        return debug_record_variable(dhandle, name, type_0, kind, val);
    }
    v = xmalloc(::core::mem::size_of::<stab_pending_var>() as libc::c_ulong)
        as *mut stab_pending_var;
    memset(
        v as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<stab_pending_var>() as libc::c_ulong,
    );
    (*v).next = (*info).pending;
    (*v).name = name;
    (*v).type_0 = type_0;
    (*v).kind = kind;
    (*v).val = val;
    (*info).pending = v;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn stab_emit_pending_vars(
    mut dhandle: *mut libc::c_void,
    mut info: *mut stab_handle,
) -> bool {
    let mut v: *mut stab_pending_var = 0 as *mut stab_pending_var;
    v = (*info).pending;
    while !v.is_null() {
        let mut next: *mut stab_pending_var = 0 as *mut stab_pending_var;
        if !debug_record_variable(dhandle, (*v).name, (*v).type_0, (*v).kind, (*v).val) {
            return 0 as libc::c_int != 0;
        }
        next = (*v).next;
        free(v as *mut libc::c_void);
        v = next;
    }
    (*info).pending = 0 as *mut stab_pending_var;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn stab_find_slot(
    mut info: *mut stab_handle,
    mut typenums: *const libc::c_int,
) -> *mut debug_type {
    let mut filenum: libc::c_int = 0;
    let mut tindex: libc::c_int = 0;
    let mut ps: *mut *mut stab_types = 0 as *mut *mut stab_types;
    filenum = *typenums.offset(0 as libc::c_int as isize);
    tindex = *typenums.offset(1 as libc::c_int as isize);
    if filenum < 0 as libc::c_int || filenum as libc::c_uint >= (*info).files {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"Type file number %d out of range\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            filenum,
        );
        return 0 as *mut debug_type;
    }
    if tindex < 0 as libc::c_int {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"Type index number %d out of range\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            tindex,
        );
        return 0 as *mut debug_type;
    }
    ps = ((*info).file_types).offset(filenum as isize);
    while tindex >= 16 as libc::c_int {
        if (*ps).is_null() {
            *ps = xmalloc(::core::mem::size_of::<stab_types>() as libc::c_ulong)
                as *mut stab_types;
            memset(
                *ps as *mut libc::c_void,
                0 as libc::c_int,
                ::core::mem::size_of::<stab_types>() as libc::c_ulong,
            );
        }
        ps = &mut (**ps).next;
        tindex -= 16 as libc::c_int;
    }
    if (*ps).is_null() {
        *ps = xmalloc(::core::mem::size_of::<stab_types>() as libc::c_ulong)
            as *mut stab_types;
        memset(
            *ps as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<stab_types>() as libc::c_ulong,
        );
    }
    return ((**ps).types).as_mut_ptr().offset(tindex as isize);
}
unsafe extern "C" fn stab_find_type(
    mut dhandle: *mut libc::c_void,
    mut info: *mut stab_handle,
    mut typenums: *const libc::c_int,
) -> debug_type {
    let mut slot: *mut debug_type = 0 as *mut debug_type;
    if *typenums.offset(0 as libc::c_int as isize) == 0 as libc::c_int
        && *typenums.offset(1 as libc::c_int as isize) < 0 as libc::c_int
    {
        return stab_xcoff_builtin_type(
            dhandle,
            info,
            *typenums.offset(1 as libc::c_int as isize),
        );
    }
    slot = stab_find_slot(info, typenums);
    if slot.is_null() {
        return 0 as *mut libc::c_void as debug_type;
    }
    if (*slot).is_null() {
        return debug_make_indirect_type(
            dhandle,
            slot,
            0 as *mut libc::c_void as *const libc::c_char,
        );
    }
    return *slot;
}
unsafe extern "C" fn stab_record_type(
    mut _dhandle: *mut libc::c_void,
    mut info: *mut stab_handle,
    mut typenums: *const libc::c_int,
    mut type_0: debug_type,
) -> bool {
    let mut slot: *mut debug_type = 0 as *mut debug_type;
    slot = stab_find_slot(info, typenums);
    if slot.is_null() {
        return 0 as libc::c_int != 0;
    }
    *slot = type_0;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn stab_xcoff_builtin_type(
    mut dhandle: *mut libc::c_void,
    mut info: *mut stab_handle,
    mut typenum: libc::c_int,
) -> debug_type {
    let mut rettype: debug_type = 0 as *mut debug_type_s;
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    if typenum >= 0 as libc::c_int || typenum < -(34 as libc::c_int) {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"Unrecognized XCOFF type %d\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            typenum,
        );
        return 0 as *mut libc::c_void as debug_type;
    }
    if !((*info).xcoff_types[-typenum as usize]).is_null() {
        return (*info).xcoff_types[-typenum as usize];
    }
    match -typenum {
        1 => {
            name = b"int\0" as *const u8 as *const libc::c_char;
            rettype = debug_make_int_type(
                dhandle,
                4 as libc::c_int as libc::c_uint,
                0 as libc::c_int != 0,
            );
        }
        2 => {
            name = b"char\0" as *const u8 as *const libc::c_char;
            rettype = debug_make_int_type(
                dhandle,
                1 as libc::c_int as libc::c_uint,
                0 as libc::c_int != 0,
            );
        }
        3 => {
            name = b"short\0" as *const u8 as *const libc::c_char;
            rettype = debug_make_int_type(
                dhandle,
                2 as libc::c_int as libc::c_uint,
                0 as libc::c_int != 0,
            );
        }
        4 => {
            name = b"long\0" as *const u8 as *const libc::c_char;
            rettype = debug_make_int_type(
                dhandle,
                4 as libc::c_int as libc::c_uint,
                0 as libc::c_int != 0,
            );
        }
        5 => {
            name = b"unsigned char\0" as *const u8 as *const libc::c_char;
            rettype = debug_make_int_type(
                dhandle,
                1 as libc::c_int as libc::c_uint,
                1 as libc::c_int != 0,
            );
        }
        6 => {
            name = b"signed char\0" as *const u8 as *const libc::c_char;
            rettype = debug_make_int_type(
                dhandle,
                1 as libc::c_int as libc::c_uint,
                0 as libc::c_int != 0,
            );
        }
        7 => {
            name = b"unsigned short\0" as *const u8 as *const libc::c_char;
            rettype = debug_make_int_type(
                dhandle,
                2 as libc::c_int as libc::c_uint,
                1 as libc::c_int != 0,
            );
        }
        8 => {
            name = b"unsigned int\0" as *const u8 as *const libc::c_char;
            rettype = debug_make_int_type(
                dhandle,
                4 as libc::c_int as libc::c_uint,
                1 as libc::c_int != 0,
            );
        }
        9 => {
            name = b"unsigned\0" as *const u8 as *const libc::c_char;
            rettype = debug_make_int_type(
                dhandle,
                4 as libc::c_int as libc::c_uint,
                1 as libc::c_int != 0,
            );
        }
        10 => {
            name = b"unsigned long\0" as *const u8 as *const libc::c_char;
            rettype = debug_make_int_type(
                dhandle,
                4 as libc::c_int as libc::c_uint,
                1 as libc::c_int != 0,
            );
        }
        11 => {
            name = b"void\0" as *const u8 as *const libc::c_char;
            rettype = debug_make_void_type(dhandle);
        }
        12 => {
            name = b"float\0" as *const u8 as *const libc::c_char;
            rettype = debug_make_float_type(dhandle, 4 as libc::c_int as libc::c_uint);
        }
        13 => {
            name = b"double\0" as *const u8 as *const libc::c_char;
            rettype = debug_make_float_type(dhandle, 8 as libc::c_int as libc::c_uint);
        }
        14 => {
            name = b"long double\0" as *const u8 as *const libc::c_char;
            rettype = debug_make_float_type(dhandle, 8 as libc::c_int as libc::c_uint);
        }
        15 => {
            name = b"integer\0" as *const u8 as *const libc::c_char;
            rettype = debug_make_int_type(
                dhandle,
                4 as libc::c_int as libc::c_uint,
                0 as libc::c_int != 0,
            );
        }
        16 => {
            name = b"boolean\0" as *const u8 as *const libc::c_char;
            rettype = debug_make_bool_type(dhandle, 4 as libc::c_int as libc::c_uint);
        }
        17 => {
            name = b"short real\0" as *const u8 as *const libc::c_char;
            rettype = debug_make_float_type(dhandle, 4 as libc::c_int as libc::c_uint);
        }
        18 => {
            name = b"real\0" as *const u8 as *const libc::c_char;
            rettype = debug_make_float_type(dhandle, 8 as libc::c_int as libc::c_uint);
        }
        19 => {
            name = b"stringptr\0" as *const u8 as *const libc::c_char;
            rettype = 0 as debug_type;
        }
        20 => {
            name = b"character\0" as *const u8 as *const libc::c_char;
            rettype = debug_make_int_type(
                dhandle,
                1 as libc::c_int as libc::c_uint,
                1 as libc::c_int != 0,
            );
        }
        21 => {
            name = b"logical*1\0" as *const u8 as *const libc::c_char;
            rettype = debug_make_bool_type(dhandle, 1 as libc::c_int as libc::c_uint);
        }
        22 => {
            name = b"logical*2\0" as *const u8 as *const libc::c_char;
            rettype = debug_make_bool_type(dhandle, 2 as libc::c_int as libc::c_uint);
        }
        23 => {
            name = b"logical*4\0" as *const u8 as *const libc::c_char;
            rettype = debug_make_bool_type(dhandle, 4 as libc::c_int as libc::c_uint);
        }
        24 => {
            name = b"logical\0" as *const u8 as *const libc::c_char;
            rettype = debug_make_bool_type(dhandle, 4 as libc::c_int as libc::c_uint);
        }
        25 => {
            name = b"complex\0" as *const u8 as *const libc::c_char;
            rettype = debug_make_complex_type(dhandle, 8 as libc::c_int as libc::c_uint);
        }
        26 => {
            name = b"double complex\0" as *const u8 as *const libc::c_char;
            rettype = debug_make_complex_type(
                dhandle,
                16 as libc::c_int as libc::c_uint,
            );
        }
        27 => {
            name = b"integer*1\0" as *const u8 as *const libc::c_char;
            rettype = debug_make_int_type(
                dhandle,
                1 as libc::c_int as libc::c_uint,
                0 as libc::c_int != 0,
            );
        }
        28 => {
            name = b"integer*2\0" as *const u8 as *const libc::c_char;
            rettype = debug_make_int_type(
                dhandle,
                2 as libc::c_int as libc::c_uint,
                0 as libc::c_int != 0,
            );
        }
        29 => {
            name = b"integer*4\0" as *const u8 as *const libc::c_char;
            rettype = debug_make_int_type(
                dhandle,
                4 as libc::c_int as libc::c_uint,
                0 as libc::c_int != 0,
            );
        }
        30 => {
            name = b"wchar\0" as *const u8 as *const libc::c_char;
            rettype = debug_make_int_type(
                dhandle,
                2 as libc::c_int as libc::c_uint,
                0 as libc::c_int != 0,
            );
        }
        31 => {
            name = b"long long\0" as *const u8 as *const libc::c_char;
            rettype = debug_make_int_type(
                dhandle,
                8 as libc::c_int as libc::c_uint,
                0 as libc::c_int != 0,
            );
        }
        32 => {
            name = b"unsigned long long\0" as *const u8 as *const libc::c_char;
            rettype = debug_make_int_type(
                dhandle,
                8 as libc::c_int as libc::c_uint,
                1 as libc::c_int != 0,
            );
        }
        33 => {
            name = b"logical*8\0" as *const u8 as *const libc::c_char;
            rettype = debug_make_bool_type(dhandle, 8 as libc::c_int as libc::c_uint);
        }
        34 => {
            name = b"integer*8\0" as *const u8 as *const libc::c_char;
            rettype = debug_make_int_type(
                dhandle,
                8 as libc::c_int as libc::c_uint,
                0 as libc::c_int != 0,
            );
        }
        _ => {
            abort();
        }
    }
    rettype = debug_name_type(dhandle, name, rettype);
    (*info).xcoff_types[-typenum as usize] = rettype;
    return rettype;
}
unsafe extern "C" fn stab_find_tagged_type(
    mut dhandle: *mut libc::c_void,
    mut info: *mut stab_handle,
    mut p: *const libc::c_char,
    mut len: libc::c_int,
    mut kind: debug_type_kind,
) -> debug_type {
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dtype: debug_type = 0 as *mut debug_type_s;
    let mut st: *mut stab_tag = 0 as *mut stab_tag;
    name = savestring(p, len);
    dtype = debug_find_tagged_type(dhandle, name, DEBUG_KIND_ILLEGAL);
    if !dtype.is_null() {
        free(name as *mut libc::c_void);
        return dtype;
    }
    st = (*info).tags;
    while !st.is_null() {
        if *((*st).name).offset(0 as libc::c_int as isize) as libc::c_int
            == *name.offset(0 as libc::c_int as isize) as libc::c_int
            && strcmp((*st).name, name) == 0 as libc::c_int
        {
            if (*st).kind as libc::c_uint
                == DEBUG_KIND_ILLEGAL as libc::c_int as libc::c_uint
            {
                (*st).kind = kind;
            }
            free(name as *mut libc::c_void);
            break;
        } else {
            st = (*st).next;
        }
    }
    if st.is_null() {
        st = xmalloc(::core::mem::size_of::<stab_tag>() as libc::c_ulong)
            as *mut stab_tag;
        memset(
            st as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<stab_tag>() as libc::c_ulong,
        );
        (*st).next = (*info).tags;
        (*st).name = name;
        (*st).kind = kind;
        (*st).slot = 0 as *mut libc::c_void as debug_type;
        (*st).type_0 = debug_make_indirect_type(dhandle, &mut (*st).slot, name);
        (*info).tags = st;
    }
    return (*st).type_0;
}
unsafe extern "C" fn stab_demangle_argtypes(
    mut dhandle: *mut libc::c_void,
    mut info: *mut stab_handle,
    mut physname: *const libc::c_char,
    mut pvarargs: *mut bool,
    mut physname_len: libc::c_uint,
) -> *mut debug_type {
    let mut current_block: u64;
    let mut minfo: stab_demangle_info = stab_demangle_info {
        dhandle: 0 as *mut libc::c_void,
        info: 0 as *mut stab_handle,
        args: 0 as *mut debug_type,
        varargs: false,
        typestrings: 0 as *mut stab_demangle_typestring,
        typestring_count: 0,
        typestring_alloc: 0,
    };
    if *physname.offset(0 as libc::c_int as isize) as libc::c_int == '_' as i32
        && *physname.offset(1 as libc::c_int as isize) as libc::c_int == 'Z' as i32
    {
        return stab_demangle_v3_argtypes(dhandle, info, physname, pvarargs);
    }
    minfo.dhandle = dhandle;
    minfo.info = info;
    minfo.args = 0 as *mut debug_type;
    minfo.varargs = 0 as libc::c_int != 0;
    minfo.typestring_alloc = 10 as libc::c_int as libc::c_uint;
    minfo
        .typestrings = xmalloc(
        (minfo.typestring_alloc as libc::c_ulong)
            .wrapping_mul(
                ::core::mem::size_of::<stab_demangle_typestring>() as libc::c_ulong,
            ),
    ) as *mut stab_demangle_typestring;
    minfo.typestring_count = 0 as libc::c_int as libc::c_uint;
    if stab_demangle_prefix(&mut minfo, &mut physname, physname_len) {
        if *physname as libc::c_int != '\0' as i32 {
            if !stab_demangle_signature(&mut minfo, &mut physname) {
                current_block = 15589034125590538558;
            } else {
                current_block = 3276175668257526147;
            }
        } else {
            current_block = 3276175668257526147;
        }
        match current_block {
            15589034125590538558 => {}
            _ => {
                free(minfo.typestrings as *mut libc::c_void);
                minfo.typestrings = 0 as *mut stab_demangle_typestring;
                if (minfo.args).is_null() {
                    fprintf(
                        stderr,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"no argument types in mangled string\n\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                }
                *pvarargs = minfo.varargs;
                return minfo.args;
            }
        }
    }
    free(minfo.typestrings as *mut libc::c_void);
    return 0 as *mut debug_type;
}
unsafe extern "C" fn stab_demangle_v3_argtypes(
    mut dhandle: *mut libc::c_void,
    mut info: *mut stab_handle,
    mut physname: *const libc::c_char,
    mut pvarargs: *mut bool,
) -> *mut debug_type {
    let mut dc: *mut demangle_component = 0 as *mut demangle_component;
    let mut mem: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut pargs: *mut debug_type = 0 as *mut debug_type;
    dc = cplus_demangle_v3_components(
        physname,
        (1 as libc::c_int) << 0 as libc::c_int | demangle_flags,
        &mut mem,
    );
    if dc.is_null() {
        stab_bad_demangle(physname);
        return 0 as *mut debug_type;
    }
    if (*dc).type_0 as libc::c_uint
        != DEMANGLE_COMPONENT_TYPED_NAME as libc::c_int as libc::c_uint
        || (*(*dc).u.s_binary.right).type_0 as libc::c_uint
            != DEMANGLE_COMPONENT_FUNCTION_TYPE as libc::c_int as libc::c_uint
    {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"Demangled name is not a function\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        free(mem);
        return 0 as *mut debug_type;
    }
    pargs = stab_demangle_v3_arglist(
        dhandle,
        info,
        (*(*dc).u.s_binary.right).u.s_binary.right,
        pvarargs,
    );
    free(mem);
    return pargs;
}
unsafe extern "C" fn stab_demangle_v3_arglist(
    mut dhandle: *mut libc::c_void,
    mut info: *mut stab_handle,
    mut arglist: *mut demangle_component,
    mut pvarargs: *mut bool,
) -> *mut debug_type {
    let mut dc: *mut demangle_component = 0 as *mut demangle_component;
    let mut alloc: libc::c_uint = 0;
    let mut count: libc::c_uint = 0;
    let mut pargs: *mut debug_type = 0 as *mut debug_type;
    alloc = 10 as libc::c_int as libc::c_uint;
    pargs = xmalloc(
        (alloc as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<debug_type>() as libc::c_ulong),
    ) as *mut debug_type;
    *pvarargs = 0 as libc::c_int != 0;
    count = 0 as libc::c_int as libc::c_uint;
    dc = arglist;
    while !dc.is_null() {
        let mut arg: debug_type = 0 as *mut debug_type_s;
        let mut varargs: bool = false;
        if (*dc).type_0 as libc::c_uint
            != DEMANGLE_COMPONENT_ARGLIST as libc::c_int as libc::c_uint
        {
            fprintf(
                stderr,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Unexpected type in v3 arglist demangling\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            free(pargs as *mut libc::c_void);
            return 0 as *mut debug_type;
        }
        if ((*dc).u.s_binary.left).is_null() {
            break;
        }
        arg = stab_demangle_v3_arg(
            dhandle,
            info,
            (*dc).u.s_binary.left,
            0 as debug_type,
            &mut varargs,
        );
        if arg.is_null() {
            if varargs {
                *pvarargs = 1 as libc::c_int != 0;
            } else {
                free(pargs as *mut libc::c_void);
                return 0 as *mut debug_type;
            }
        } else {
            if count.wrapping_add(1 as libc::c_int as libc::c_uint) >= alloc {
                alloc = alloc.wrapping_add(10 as libc::c_int as libc::c_uint);
                pargs = xrealloc(
                    pargs as *mut libc::c_void,
                    (alloc as libc::c_ulong)
                        .wrapping_mul(
                            ::core::mem::size_of::<debug_type>() as libc::c_ulong,
                        ),
                ) as *mut debug_type;
            }
            let ref mut fresh19 = *pargs.offset(count as isize);
            *fresh19 = arg;
            count = count.wrapping_add(1);
            count;
        }
        dc = (*dc).u.s_binary.right;
    }
    let ref mut fresh20 = *pargs.offset(count as isize);
    *fresh20 = 0 as *mut libc::c_void as debug_type;
    return pargs;
}
unsafe extern "C" fn stab_demangle_v3_arg(
    mut dhandle: *mut libc::c_void,
    mut info: *mut stab_handle,
    mut dc: *mut demangle_component,
    mut context: debug_type,
    mut pvarargs: *mut bool,
) -> debug_type {
    let mut dt: debug_type = 0 as *mut debug_type_s;
    if !pvarargs.is_null() {
        *pvarargs = 0 as libc::c_int != 0;
    }
    match (*dc).type_0 as libc::c_uint {
        0 => {
            if !context.is_null() {
                let mut fields: *const debug_field = 0 as *const debug_field;
                fields = debug_get_fields(dhandle, context);
                if !fields.is_null() {
                    while !(*fields).is_null() {
                        let mut ft: debug_type = 0 as *mut debug_type_s;
                        let mut dn: *const libc::c_char = 0 as *const libc::c_char;
                        ft = debug_get_field_type(dhandle, *fields);
                        if ft.is_null() {
                            return 0 as debug_type;
                        }
                        dn = debug_get_type_name(dhandle, ft);
                        if !dn.is_null()
                            && strlen(dn) as libc::c_int == (*dc).u.s_name.len
                            && strncmp(
                                dn,
                                (*dc).u.s_name.s,
                                (*dc).u.s_name.len as libc::c_ulong,
                            ) == 0 as libc::c_int
                        {
                            return ft;
                        }
                        fields = fields.offset(1);
                        fields;
                    }
                }
            }
            return stab_find_tagged_type(
                dhandle,
                info,
                (*dc).u.s_name.s,
                (*dc).u.s_name.len,
                DEBUG_KIND_ILLEGAL,
            );
        }
        1 => {
            context = stab_demangle_v3_arg(
                dhandle,
                info,
                (*dc).u.s_binary.left,
                context,
                0 as *mut bool,
            );
            if context.is_null() {
                return 0 as debug_type;
            }
            return stab_demangle_v3_arg(
                dhandle,
                info,
                (*dc).u.s_binary.right,
                context,
                0 as *mut bool,
            );
        }
        4 => {
            let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut alc: size_t = 0;
            p = cplus_demangle_print(
                (1 as libc::c_int) << 0 as libc::c_int | demangle_flags,
                dc,
                20 as libc::c_int,
                &mut alc,
            );
            if p.is_null() {
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Failed to print demangled template\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                return 0 as debug_type;
            }
            dt = stab_find_tagged_type(
                dhandle,
                info,
                p,
                strlen(p) as libc::c_int,
                DEBUG_KIND_CLASS,
            );
            free(p as *mut libc::c_void);
            return dt;
        }
        24 => {
            return stab_find_tagged_type(
                dhandle,
                info,
                (*dc).u.s_string.string,
                (*dc).u.s_string.len,
                DEBUG_KIND_ILLEGAL,
            );
        }
        25 | 26 | 27 | 34 | 35 => {
            dt = stab_demangle_v3_arg(
                dhandle,
                info,
                (*dc).u.s_binary.left,
                0 as debug_type,
                0 as *mut bool,
            );
            if dt.is_null() {
                return 0 as debug_type;
            }
            match (*dc).type_0 as libc::c_uint {
                25 => return dt,
                26 => return debug_make_volatile_type(dhandle, dt),
                27 => return debug_make_const_type(dhandle, dt),
                34 => return debug_make_pointer_type(dhandle, dt),
                35 => return debug_make_reference_type(dhandle, dt),
                _ => {
                    abort();
                }
            }
        }
        41 => {
            let mut pargs: *mut debug_type = 0 as *mut debug_type;
            let mut varargs: bool = false;
            if ((*dc).u.s_binary.left).is_null() {
                dt = debug_make_void_type(dhandle);
            } else {
                dt = stab_demangle_v3_arg(
                    dhandle,
                    info,
                    (*dc).u.s_binary.left,
                    0 as debug_type,
                    0 as *mut bool,
                );
            }
            if dt.is_null() {
                return 0 as debug_type;
            }
            pargs = stab_demangle_v3_arglist(
                dhandle,
                info,
                (*dc).u.s_binary.right,
                &mut varargs,
            );
            if pargs.is_null() {
                return 0 as debug_type;
            }
            return debug_make_function_type(dhandle, dt, pargs, varargs);
        }
        39 => {
            let mut p_0: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut alc_0: size_t = 0;
            let mut ret: debug_type = 0 as *mut debug_type_s;
            p_0 = cplus_demangle_print(
                (1 as libc::c_int) << 0 as libc::c_int | demangle_flags,
                dc,
                20 as libc::c_int,
                &mut alc_0,
            );
            if p_0.is_null() {
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Couldn't get demangled builtin type\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                return 0 as debug_type;
            }
            if strcmp(p_0, b"signed char\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                ret = debug_make_int_type(
                    dhandle,
                    1 as libc::c_int as libc::c_uint,
                    0 as libc::c_int != 0,
                );
            } else if strcmp(p_0, b"bool\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                ret = debug_make_bool_type(dhandle, 1 as libc::c_int as libc::c_uint);
            } else if strcmp(p_0, b"char\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                ret = debug_make_int_type(
                    dhandle,
                    1 as libc::c_int as libc::c_uint,
                    0 as libc::c_int != 0,
                );
            } else if strcmp(p_0, b"double\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                ret = debug_make_float_type(dhandle, 8 as libc::c_int as libc::c_uint);
            } else if strcmp(p_0, b"long double\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                ret = debug_make_float_type(dhandle, 8 as libc::c_int as libc::c_uint);
            } else if strcmp(p_0, b"float\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                ret = debug_make_float_type(dhandle, 4 as libc::c_int as libc::c_uint);
            } else if strcmp(p_0, b"__float128\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                ret = debug_make_float_type(dhandle, 16 as libc::c_int as libc::c_uint);
            } else if strcmp(p_0, b"unsigned char\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                ret = debug_make_int_type(
                    dhandle,
                    1 as libc::c_int as libc::c_uint,
                    1 as libc::c_int != 0,
                );
            } else if strcmp(p_0, b"int\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                ret = debug_make_int_type(
                    dhandle,
                    4 as libc::c_int as libc::c_uint,
                    0 as libc::c_int != 0,
                );
            } else if strcmp(p_0, b"unsigned int\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                ret = debug_make_int_type(
                    dhandle,
                    4 as libc::c_int as libc::c_uint,
                    1 as libc::c_int != 0,
                );
            } else if strcmp(p_0, b"long\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                ret = debug_make_int_type(
                    dhandle,
                    4 as libc::c_int as libc::c_uint,
                    0 as libc::c_int != 0,
                );
            } else if strcmp(p_0, b"unsigned long\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                ret = debug_make_int_type(
                    dhandle,
                    4 as libc::c_int as libc::c_uint,
                    1 as libc::c_int != 0,
                );
            } else if strcmp(p_0, b"__int128\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                ret = debug_make_int_type(
                    dhandle,
                    16 as libc::c_int as libc::c_uint,
                    0 as libc::c_int != 0,
                );
            } else if strcmp(
                p_0,
                b"unsigned __int128\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                ret = debug_make_int_type(
                    dhandle,
                    16 as libc::c_int as libc::c_uint,
                    1 as libc::c_int != 0,
                );
            } else if strcmp(p_0, b"short\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                ret = debug_make_int_type(
                    dhandle,
                    2 as libc::c_int as libc::c_uint,
                    0 as libc::c_int != 0,
                );
            } else if strcmp(
                p_0,
                b"unsigned short\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                ret = debug_make_int_type(
                    dhandle,
                    2 as libc::c_int as libc::c_uint,
                    1 as libc::c_int != 0,
                );
            } else if strcmp(p_0, b"void\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                ret = debug_make_void_type(dhandle);
            } else if strcmp(p_0, b"wchar_t\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                ret = debug_make_int_type(
                    dhandle,
                    4 as libc::c_int as libc::c_uint,
                    1 as libc::c_int != 0,
                );
            } else if strcmp(p_0, b"long long\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                ret = debug_make_int_type(
                    dhandle,
                    8 as libc::c_int as libc::c_uint,
                    0 as libc::c_int != 0,
                );
            } else if strcmp(
                p_0,
                b"unsigned long long\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                ret = debug_make_int_type(
                    dhandle,
                    8 as libc::c_int as libc::c_uint,
                    1 as libc::c_int != 0,
                );
            } else if strcmp(p_0, b"...\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                if pvarargs.is_null() {
                    fprintf(
                        stderr,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Unexpected demangled varargs\n\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                } else {
                    *pvarargs = 1 as libc::c_int != 0;
                }
                ret = 0 as debug_type;
            } else {
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Unrecognized demangled builtin type\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                ret = 0 as debug_type;
            }
            free(p_0 as *mut libc::c_void);
            return ret;
        }
        2 | 3 | 5 | 7 | 8 | 18 | 28 | 29 | 30 | 33 | 37 | 38 | 40 | 42 | 43 | 46 | _ => {
            fprintf(
                stderr,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Unrecognized demangle component %d\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*dc).type_0 as libc::c_int,
            );
            return 0 as debug_type;
        }
    };
}
static mut demangle_flags: libc::c_int = (1 as libc::c_int) << 1 as libc::c_int;
unsafe extern "C" fn parse_number(
    mut pp: *mut *const libc::c_char,
    mut poverflow: *mut bool,
    mut p_end: *const libc::c_char,
) -> bfd_vma {
    let mut ul: libc::c_ulong = 0;
    let mut orig: *const libc::c_char = 0 as *const libc::c_char;
    if !poverflow.is_null() {
        *poverflow = 0 as libc::c_int != 0;
    }
    orig = *pp;
    if orig >= p_end {
        return 0 as libc::c_int as bfd_vma;
    }
    if *orig as libc::c_int == 0 as libc::c_int {
        return 0 as libc::c_int as bfd_vma;
    }
    *__errno_location() = 0 as libc::c_int;
    ul = strtoul(*pp, pp as *mut *mut libc::c_char, 0 as libc::c_int);
    if ul.wrapping_add(1 as libc::c_int as libc::c_ulong)
        != 0 as libc::c_int as libc::c_ulong || *__errno_location() == 0 as libc::c_int
    {
        if *orig as libc::c_int == '-' as i32 {
            return ul as libc::c_long as bfd_vma;
        }
        return ul;
    }
    if ::core::mem::size_of::<bfd_vma>() as libc::c_ulong
        > ::core::mem::size_of::<libc::c_ulong>() as libc::c_ulong
    {
        let mut p: *const libc::c_char = 0 as *const libc::c_char;
        let mut neg: bool = false;
        let mut base: libc::c_int = 0;
        let mut over: bfd_vma = 0;
        let mut lastdig: bfd_vma = 0;
        let mut overflow: bool = false;
        let mut v: bfd_vma = 0;
        p = orig;
        neg = 0 as libc::c_int != 0;
        if *p as libc::c_int == '+' as i32 {
            p = p.offset(1);
            p;
        } else if *p as libc::c_int == '-' as i32 {
            neg = 1 as libc::c_int != 0;
            p = p.offset(1);
            p;
        }
        base = 10 as libc::c_int;
        if *p as libc::c_int == '0' as i32 {
            if *p.offset(1 as libc::c_int as isize) as libc::c_int == 'x' as i32
                || *p.offset(1 as libc::c_int as isize) as libc::c_int == 'X' as i32
            {
                base = 16 as libc::c_int;
                p = p.offset(2 as libc::c_int as isize);
            } else {
                base = 8 as libc::c_int;
                p = p.offset(1);
                p;
            }
        }
        over = (-(1 as libc::c_int) as bfd_signed_vma as bfd_vma)
            .wrapping_div(base as bfd_vma);
        lastdig = (-(1 as libc::c_int) as bfd_signed_vma as bfd_vma)
            .wrapping_rem(base as bfd_vma);
        overflow = 0 as libc::c_int != 0;
        v = 0 as libc::c_int as bfd_vma;
        loop {
            let mut d: libc::c_int = 0;
            let fresh21 = p;
            p = p.offset(1);
            d = *fresh21 as libc::c_int;
            if _sch_istable[(d & 0xff as libc::c_int) as usize] as libc::c_int
                & _sch_isdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
            {
                d -= '0' as i32;
            } else if _sch_istable[(d & 0xff as libc::c_int) as usize] as libc::c_int
                & _sch_isupper as libc::c_int as libc::c_ushort as libc::c_int != 0
            {
                d -= 'A' as i32;
            } else {
                if !(_sch_istable[(d & 0xff as libc::c_int) as usize] as libc::c_int
                    & _sch_islower as libc::c_int as libc::c_ushort as libc::c_int != 0)
                {
                    break;
                }
                d -= 'a' as i32;
            }
            if d >= base {
                break;
            }
            if !(v > over || v == over && d as bfd_vma > lastdig) {
                continue;
            }
            overflow = 1 as libc::c_int != 0;
            break;
        }
        if !overflow {
            if neg {
                v = v.wrapping_neg();
            }
            return v;
        }
    }
    if !poverflow.is_null() {
        *poverflow = 1 as libc::c_int != 0;
    } else {
        warn_stab(
            orig,
            dcgettext(
                0 as *const libc::c_char,
                b"numeric overflow\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    return 0 as libc::c_int as bfd_vma;
}
unsafe extern "C" fn stab_bad_demangle(mut s: *const libc::c_char) {
    fprintf(
        stderr,
        dcgettext(
            0 as *const libc::c_char,
            b"bad mangled name `%s'\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        s,
    );
}
unsafe extern "C" fn stab_demangle_count(
    mut pp: *mut *const libc::c_char,
) -> libc::c_uint {
    let mut count: libc::c_uint = 0;
    count = 0 as libc::c_int as libc::c_uint;
    while _sch_istable[(**pp as libc::c_int & 0xff as libc::c_int) as usize]
        as libc::c_int & _sch_isdigit as libc::c_int as libc::c_ushort as libc::c_int
        != 0
    {
        count = count.wrapping_mul(10 as libc::c_int as libc::c_uint);
        count = count.wrapping_add((**pp as libc::c_int - '0' as i32) as libc::c_uint);
        *pp = (*pp).offset(1);
        let _ = *pp;
    }
    return count;
}
unsafe extern "C" fn stab_demangle_get_count(
    mut pp: *mut *const libc::c_char,
    mut pi: *mut libc::c_uint,
) -> bool {
    if _sch_istable[(**pp as libc::c_int & 0xff as libc::c_int) as usize] as libc::c_int
        & _sch_isdigit as libc::c_int as libc::c_ushort as libc::c_int == 0
    {
        return 0 as libc::c_int != 0;
    }
    *pi = (**pp as libc::c_int - '0' as i32) as libc::c_uint;
    *pp = (*pp).offset(1);
    let _ = *pp;
    if _sch_istable[(**pp as libc::c_int & 0xff as libc::c_int) as usize] as libc::c_int
        & _sch_isdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        let mut count: libc::c_uint = 0;
        let mut p: *const libc::c_char = 0 as *const libc::c_char;
        count = *pi;
        p = *pp;
        loop {
            count = count.wrapping_mul(10 as libc::c_int as libc::c_uint);
            count = count.wrapping_add((*p as libc::c_int - '0' as i32) as libc::c_uint);
            p = p.offset(1);
            p;
            if !(_sch_istable[(*p as libc::c_int & 0xff as libc::c_int) as usize]
                as libc::c_int
                & _sch_isdigit as libc::c_int as libc::c_ushort as libc::c_int != 0)
            {
                break;
            }
        }
        if *p as libc::c_int == '_' as i32 {
            *pp = p.offset(1 as libc::c_int as isize);
            *pi = count;
        }
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn stab_demangle_prefix(
    mut minfo: *mut stab_demangle_info,
    mut pp: *mut *const libc::c_char,
    mut physname_len: libc::c_uint,
) -> bool {
    let mut scan: *const libc::c_char = 0 as *const libc::c_char;
    let mut i: libc::c_uint = 0;
    if physname_len != 0 {
        scan = (*pp).offset(physname_len as isize);
    } else {
        scan = *pp;
        loop {
            scan = strchr(scan, '_' as i32);
            if !(!scan.is_null()
                && {
                    scan = scan.offset(1);
                    *scan as libc::c_int != '_' as i32
                })
            {
                break;
            }
        }
        if scan.is_null() {
            stab_bad_demangle(*pp);
            return 0 as libc::c_int != 0;
        }
        scan = scan.offset(-1);
        scan;
        i = strspn(scan, b"_\0" as *const u8 as *const libc::c_char) as libc::c_uint;
        if i > 2 as libc::c_int as libc::c_uint {
            scan = scan
                .offset(i.wrapping_sub(2 as libc::c_int as libc::c_uint) as isize);
        }
    }
    if scan == *pp
        && (_sch_istable[(*scan.offset(2 as libc::c_int as isize) as libc::c_int
            & 0xff as libc::c_int) as usize] as libc::c_int
            & _sch_isdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
            || *scan.offset(2 as libc::c_int as isize) as libc::c_int == 'Q' as i32
            || *scan.offset(2 as libc::c_int as isize) as libc::c_int == 't' as i32)
    {
        *pp = scan.offset(2 as libc::c_int as isize);
        return 1 as libc::c_int != 0;
    } else if scan == *pp
        && _sch_istable[(*scan.offset(2 as libc::c_int as isize) as libc::c_int
            & 0xff as libc::c_int) as usize] as libc::c_int
            & _sch_isdigit as libc::c_int as libc::c_ushort as libc::c_int == 0
        && *scan.offset(2 as libc::c_int as isize) as libc::c_int != 't' as i32
    {
        while *scan as libc::c_int == '_' as i32 {
            scan = scan.offset(1);
            scan;
        }
        scan = strstr(scan, b"__\0" as *const u8 as *const libc::c_char);
        if scan.is_null()
            || *scan.offset(2 as libc::c_int as isize) as libc::c_int == '\0' as i32
        {
            stab_bad_demangle(*pp);
            return 0 as libc::c_int != 0;
        }
        return stab_demangle_function_name(minfo, pp, scan);
    } else if *scan.offset(2 as libc::c_int as isize) as libc::c_int != '\0' as i32 {
        return stab_demangle_function_name(minfo, pp, scan)
    } else {
        stab_bad_demangle(*pp);
        return 0 as libc::c_int != 0;
    };
}
unsafe extern "C" fn stab_demangle_function_name(
    mut minfo: *mut stab_demangle_info,
    mut pp: *mut *const libc::c_char,
    mut scan: *const libc::c_char,
) -> bool {
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    name = *pp;
    *pp = scan.offset(2 as libc::c_int as isize);
    if (*pp).offset_from(name) as libc::c_long >= 5 as libc::c_int as libc::c_long
        && startswith(name, b"type\0" as *const u8 as *const libc::c_char) as libc::c_int
            != 0
        && (*name.offset(4 as libc::c_int as isize) as libc::c_int == '$' as i32
            || *name.offset(4 as libc::c_int as isize) as libc::c_int == '.' as i32)
    {
        let mut tem: *const libc::c_char = 0 as *const libc::c_char;
        tem = name.offset(5 as libc::c_int as isize);
        if !stab_demangle_type(
            minfo,
            &mut tem,
            0 as *mut libc::c_void as *mut debug_type,
        ) {
            return 0 as libc::c_int != 0;
        }
    } else if *name.offset(0 as libc::c_int as isize) as libc::c_int == '_' as i32
        && *name.offset(1 as libc::c_int as isize) as libc::c_int == '_' as i32
        && *name.offset(2 as libc::c_int as isize) as libc::c_int == 'o' as i32
        && *name.offset(3 as libc::c_int as isize) as libc::c_int == 'p' as i32
    {
        let mut tem_0: *const libc::c_char = 0 as *const libc::c_char;
        tem_0 = name.offset(4 as libc::c_int as isize);
        if !stab_demangle_type(
            minfo,
            &mut tem_0,
            0 as *mut libc::c_void as *mut debug_type,
        ) {
            return 0 as libc::c_int != 0;
        }
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn stab_demangle_signature(
    mut minfo: *mut stab_demangle_info,
    mut pp: *mut *const libc::c_char,
) -> bool {
    let mut orig: *const libc::c_char = 0 as *const libc::c_char;
    let mut expect_func: bool = false;
    let mut func_done: bool = false;
    let mut hold: *const libc::c_char = 0 as *const libc::c_char;
    orig = *pp;
    expect_func = 0 as libc::c_int != 0;
    func_done = 0 as libc::c_int != 0;
    hold = 0 as *const libc::c_char;
    while **pp as libc::c_int != '\0' as i32 {
        match **pp as libc::c_int {
            81 => {
                hold = *pp;
                if !stab_demangle_qualified(
                    minfo,
                    pp,
                    0 as *mut libc::c_void as *mut debug_type,
                )
                    || !stab_demangle_remember_type(
                        minfo,
                        hold,
                        (*pp).offset_from(hold) as libc::c_long as libc::c_int,
                    )
                {
                    return 0 as libc::c_int != 0;
                }
                expect_func = 1 as libc::c_int != 0;
                hold = 0 as *const libc::c_char;
            }
            83 => {
                if hold.is_null() {
                    hold = *pp;
                }
                *pp = (*pp).offset(1);
                let _ = *pp;
            }
            67 => {
                if hold.is_null() {
                    hold = *pp;
                }
                *pp = (*pp).offset(1);
                let _ = *pp;
            }
            48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
                if hold.is_null() {
                    hold = *pp;
                }
                if !stab_demangle_class(
                    minfo,
                    pp,
                    0 as *mut libc::c_void as *mut *const libc::c_char,
                )
                    || !stab_demangle_remember_type(
                        minfo,
                        hold,
                        (*pp).offset_from(hold) as libc::c_long as libc::c_int,
                    )
                {
                    return 0 as libc::c_int != 0;
                }
                expect_func = 1 as libc::c_int != 0;
                hold = 0 as *const libc::c_char;
            }
            70 => {
                hold = 0 as *const libc::c_char;
                func_done = 1 as libc::c_int != 0;
                *pp = (*pp).offset(1);
                let _ = *pp;
                if !stab_demangle_args(
                    minfo,
                    pp,
                    &mut (*minfo).args,
                    &mut (*minfo).varargs,
                ) {
                    return 0 as libc::c_int != 0;
                }
            }
            116 => {
                if hold.is_null() {
                    hold = *pp;
                }
                if !stab_demangle_template(
                    minfo,
                    pp,
                    0 as *mut libc::c_void as *mut *mut libc::c_char,
                )
                    || !stab_demangle_remember_type(
                        minfo,
                        hold,
                        (*pp).offset_from(hold) as libc::c_long as libc::c_int,
                    )
                {
                    return 0 as libc::c_int != 0;
                }
                hold = 0 as *const libc::c_char;
                expect_func = 1 as libc::c_int != 0;
            }
            95 => {
                stab_bad_demangle(orig);
                return 0 as libc::c_int != 0;
            }
            _ => {
                func_done = 1 as libc::c_int != 0;
                if !stab_demangle_args(
                    minfo,
                    pp,
                    &mut (*minfo).args,
                    &mut (*minfo).varargs,
                ) {
                    return 0 as libc::c_int != 0;
                }
            }
        }
        if expect_func {
            func_done = 1 as libc::c_int != 0;
            if !stab_demangle_args(
                minfo,
                pp,
                &mut (*minfo).args,
                &mut (*minfo).varargs,
            ) {
                return 0 as libc::c_int != 0;
            }
        }
    }
    if !func_done {
        if !stab_demangle_args(minfo, pp, &mut (*minfo).args, &mut (*minfo).varargs) {
            return 0 as libc::c_int != 0;
        }
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn stab_demangle_qualified(
    mut minfo: *mut stab_demangle_info,
    mut pp: *mut *const libc::c_char,
    mut ptype: *mut debug_type,
) -> bool {
    let mut orig: *const libc::c_char = 0 as *const libc::c_char;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut qualifiers: libc::c_uint = 0;
    let mut context: debug_type = 0 as *mut debug_type_s;
    orig = *pp;
    match *(*pp).offset(1 as libc::c_int as isize) as libc::c_int {
        95 => {
            p = (*pp).offset(2 as libc::c_int as isize);
            if _sch_istable[(*p as libc::c_int & 0xff as libc::c_int) as usize]
                as libc::c_int
                & _sch_isdigit as libc::c_int as libc::c_ushort as libc::c_int == 0
                || *p as libc::c_int == '0' as i32
            {
                stab_bad_demangle(orig);
                return 0 as libc::c_int != 0;
            }
            qualifiers = atoi(p) as libc::c_uint;
            while _sch_istable[(*p as libc::c_int & 0xff as libc::c_int) as usize]
                as libc::c_int
                & _sch_isdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
            {
                p = p.offset(1);
                p;
            }
            if *p as libc::c_int != '_' as i32 {
                stab_bad_demangle(orig);
                return 0 as libc::c_int != 0;
            }
            *pp = p.offset(1 as libc::c_int as isize);
        }
        49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
            qualifiers = (*(*pp).offset(1 as libc::c_int as isize) as libc::c_int
                - '0' as i32) as libc::c_uint;
            if *(*pp).offset(2 as libc::c_int as isize) as libc::c_int == '_' as i32 {
                *pp = (*pp).offset(1);
                let _ = *pp;
            }
            *pp = (*pp).offset(2 as libc::c_int as isize);
        }
        48 | _ => {
            stab_bad_demangle(orig);
            return 0 as libc::c_int != 0;
        }
    }
    context = 0 as *mut libc::c_void as debug_type;
    loop {
        let fresh22 = qualifiers;
        qualifiers = qualifiers.wrapping_sub(1);
        if !(fresh22 > 0 as libc::c_int as libc::c_uint) {
            break;
        }
        if **pp as libc::c_int == '_' as i32 {
            *pp = (*pp).offset(1);
            let _ = *pp;
        }
        if **pp as libc::c_int == 't' as i32 {
            let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
            if !stab_demangle_template(
                minfo,
                pp,
                if !ptype.is_null() { &mut name } else { 0 as *mut *mut libc::c_char },
            ) {
                return 0 as libc::c_int != 0;
            }
            if !ptype.is_null() {
                context = stab_find_tagged_type(
                    (*minfo).dhandle,
                    (*minfo).info,
                    name,
                    strlen(name) as libc::c_int,
                    DEBUG_KIND_CLASS,
                );
                free(name as *mut libc::c_void);
                if context.is_null() {
                    return 0 as libc::c_int != 0;
                }
            }
        } else {
            let mut len: libc::c_uint = 0;
            len = stab_demangle_count(pp);
            if strlen(*pp) < len as libc::c_ulong {
                stab_bad_demangle(orig);
                return 0 as libc::c_int != 0;
            }
            if !ptype.is_null() {
                let mut fields: *const debug_field = 0 as *const debug_field;
                fields = 0 as *const debug_field;
                if !context.is_null() {
                    fields = debug_get_fields((*minfo).dhandle, context);
                }
                context = 0 as *mut libc::c_void as debug_type;
                if !fields.is_null() {
                    let mut name_0: *mut libc::c_char = 0 as *mut libc::c_char;
                    name_0 = savestring(*pp, len as libc::c_int);
                    while !(*fields).is_null() {
                        let mut ft: debug_type = 0 as *mut debug_type_s;
                        let mut dn: *const libc::c_char = 0 as *const libc::c_char;
                        ft = debug_get_field_type((*minfo).dhandle, *fields);
                        if ft.is_null() {
                            free(name_0 as *mut libc::c_void);
                            return 0 as libc::c_int != 0;
                        }
                        dn = debug_get_type_name((*minfo).dhandle, ft);
                        if !dn.is_null() && strcmp(dn, name_0) == 0 as libc::c_int {
                            context = ft;
                            break;
                        } else {
                            fields = fields.offset(1);
                            fields;
                        }
                    }
                    free(name_0 as *mut libc::c_void);
                }
                if context.is_null() {
                    if qualifiers == 0 as libc::c_int as libc::c_uint {
                        let mut name_1: *mut libc::c_char = 0 as *mut libc::c_char;
                        name_1 = savestring(*pp, len as libc::c_int);
                        context = debug_find_named_type((*minfo).dhandle, name_1);
                        free(name_1 as *mut libc::c_void);
                    }
                    if context.is_null() {
                        context = stab_find_tagged_type(
                            (*minfo).dhandle,
                            (*minfo).info,
                            *pp,
                            len as libc::c_int,
                            (if qualifiers == 0 as libc::c_int as libc::c_uint {
                                DEBUG_KIND_ILLEGAL as libc::c_int
                            } else {
                                DEBUG_KIND_CLASS as libc::c_int
                            }) as debug_type_kind,
                        );
                        if context.is_null() {
                            return 0 as libc::c_int != 0;
                        }
                    }
                }
            }
            *pp = (*pp).offset(len as isize);
        }
    }
    if !ptype.is_null() {
        *ptype = context;
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn stab_demangle_template(
    mut minfo: *mut stab_demangle_info,
    mut pp: *mut *const libc::c_char,
    mut pname: *mut *mut libc::c_char,
) -> bool {
    let mut orig: *const libc::c_char = 0 as *const libc::c_char;
    let mut r: libc::c_uint = 0;
    let mut i: libc::c_uint = 0;
    orig = *pp;
    *pp = (*pp).offset(1);
    let _ = *pp;
    r = stab_demangle_count(pp);
    if r == 0 as libc::c_int as libc::c_uint || strlen(*pp) < r as libc::c_ulong {
        stab_bad_demangle(orig);
        return 0 as libc::c_int != 0;
    }
    *pp = (*pp).offset(r as isize);
    if stab_demangle_get_count(pp, &mut r) as libc::c_int == 0 as libc::c_int {
        stab_bad_demangle(orig);
        return 0 as libc::c_int != 0;
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < r {
        if **pp as libc::c_int == 'Z' as i32 {
            *pp = (*pp).offset(1);
            let _ = *pp;
            if !stab_demangle_type(
                minfo,
                pp,
                0 as *mut libc::c_void as *mut debug_type,
            ) {
                return 0 as libc::c_int != 0;
            }
        } else {
            let mut old_p: *const libc::c_char = 0 as *const libc::c_char;
            let mut pointerp: bool = false;
            let mut realp: bool = false;
            let mut integralp: bool = false;
            let mut charp: bool = false;
            let mut boolp: bool = false;
            let mut done: bool = false;
            old_p = *pp;
            pointerp = 0 as libc::c_int != 0;
            realp = 0 as libc::c_int != 0;
            integralp = 0 as libc::c_int != 0;
            charp = 0 as libc::c_int != 0;
            boolp = 0 as libc::c_int != 0;
            done = 0 as libc::c_int != 0;
            if !stab_demangle_type(
                minfo,
                pp,
                0 as *mut libc::c_void as *mut debug_type,
            ) {
                return 0 as libc::c_int != 0;
            }
            while *old_p as libc::c_int != '\0' as i32 && !done {
                match *old_p as libc::c_int {
                    80 | 112 | 82 => {
                        pointerp = 1 as libc::c_int != 0;
                        done = 1 as libc::c_int != 0;
                    }
                    67 | 83 | 85 | 86 | 70 | 77 | 79 => {
                        old_p = old_p.offset(1);
                        old_p;
                    }
                    81 => {
                        integralp = 1 as libc::c_int != 0;
                        done = 1 as libc::c_int != 0;
                    }
                    84 => {
                        abort();
                    }
                    118 => {
                        abort();
                    }
                    120 | 108 | 105 | 115 | 119 => {
                        integralp = 1 as libc::c_int != 0;
                        done = 1 as libc::c_int != 0;
                    }
                    98 => {
                        boolp = 1 as libc::c_int != 0;
                        done = 1 as libc::c_int != 0;
                    }
                    99 => {
                        charp = 1 as libc::c_int != 0;
                        done = 1 as libc::c_int != 0;
                    }
                    114 | 100 | 102 => {
                        realp = 1 as libc::c_int != 0;
                        done = 1 as libc::c_int != 0;
                    }
                    _ => {
                        integralp = 1 as libc::c_int != 0;
                        done = 1 as libc::c_int != 0;
                    }
                }
            }
            if integralp {
                if **pp as libc::c_int == 'm' as i32 {
                    *pp = (*pp).offset(1);
                    let _ = *pp;
                }
                while _sch_istable[(**pp as libc::c_int & 0xff as libc::c_int) as usize]
                    as libc::c_int
                    & _sch_isdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
                {
                    *pp = (*pp).offset(1);
                    let _ = *pp;
                }
            } else if charp {
                let mut val: libc::c_uint = 0;
                if **pp as libc::c_int == 'm' as i32 {
                    *pp = (*pp).offset(1);
                    let _ = *pp;
                }
                val = stab_demangle_count(pp);
                if val == 0 as libc::c_int as libc::c_uint {
                    stab_bad_demangle(orig);
                    return 0 as libc::c_int != 0;
                }
            } else if boolp {
                let mut val_0: libc::c_uint = 0;
                val_0 = stab_demangle_count(pp);
                if val_0 != 0 as libc::c_int as libc::c_uint
                    && val_0 != 1 as libc::c_int as libc::c_uint
                {
                    stab_bad_demangle(orig);
                    return 0 as libc::c_int != 0;
                }
            } else if realp {
                if **pp as libc::c_int == 'm' as i32 {
                    *pp = (*pp).offset(1);
                    let _ = *pp;
                }
                while _sch_istable[(**pp as libc::c_int & 0xff as libc::c_int) as usize]
                    as libc::c_int
                    & _sch_isdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
                {
                    *pp = (*pp).offset(1);
                    let _ = *pp;
                }
                if **pp as libc::c_int == '.' as i32 {
                    *pp = (*pp).offset(1);
                    let _ = *pp;
                    while _sch_istable[(**pp as libc::c_int & 0xff as libc::c_int)
                        as usize] as libc::c_int
                        & _sch_isdigit as libc::c_int as libc::c_ushort as libc::c_int
                        != 0
                    {
                        *pp = (*pp).offset(1);
                        let _ = *pp;
                    }
                }
                if **pp as libc::c_int == 'e' as i32 {
                    *pp = (*pp).offset(1);
                    let _ = *pp;
                    while _sch_istable[(**pp as libc::c_int & 0xff as libc::c_int)
                        as usize] as libc::c_int
                        & _sch_isdigit as libc::c_int as libc::c_ushort as libc::c_int
                        != 0
                    {
                        *pp = (*pp).offset(1);
                        let _ = *pp;
                    }
                }
            } else if pointerp {
                let mut len: libc::c_uint = 0;
                len = stab_demangle_count(pp);
                if len == 0 as libc::c_int as libc::c_uint {
                    stab_bad_demangle(orig);
                    return 0 as libc::c_int != 0;
                }
                *pp = (*pp).offset(len as isize);
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    if !pname.is_null() {
        let mut s1: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut s2: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut s3: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut s4: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut from: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut to: *mut libc::c_char = 0 as *mut libc::c_char;
        s1 = savestring(orig, (*pp).offset_from(orig) as libc::c_long as libc::c_int);
        s2 = concat(
            b"NoSuchStrinG__\0" as *const u8 as *const libc::c_char,
            s1,
            0 as *mut libc::c_void as *const libc::c_char,
        );
        free(s1 as *mut libc::c_void);
        s3 = cplus_demangle(s2, demangle_flags);
        free(s2 as *mut libc::c_void);
        if !s3.is_null() {
            s4 = strstr(s3, b"::NoSuchStrinG\0" as *const u8 as *const libc::c_char);
        }
        if s3.is_null() || s4.is_null() {
            stab_bad_demangle(orig);
            free(s3 as *mut libc::c_void);
            return 0 as libc::c_int != 0;
        }
        to = s3;
        from = to;
        while from != s4 {
            if *from as libc::c_int != ' ' as i32
                || *from.offset(1 as libc::c_int as isize) as libc::c_int == '>' as i32
                    && from > s3
                    && *from.offset(-(1 as libc::c_int) as isize) as libc::c_int
                        == '>' as i32
            {
                let fresh23 = to;
                to = to.offset(1);
                *fresh23 = *from;
            }
            from = from.offset(1);
            from;
        }
        *pname = savestring(s3, to.offset_from(s3) as libc::c_long as libc::c_int);
        free(s3 as *mut libc::c_void);
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn stab_demangle_class(
    mut _minfo: *mut stab_demangle_info,
    mut pp: *mut *const libc::c_char,
    mut pstart: *mut *const libc::c_char,
) -> bool {
    let mut orig: *const libc::c_char = 0 as *const libc::c_char;
    let mut n: libc::c_uint = 0;
    orig = *pp;
    n = stab_demangle_count(pp);
    if strlen(*pp) < n as libc::c_ulong {
        stab_bad_demangle(orig);
        return 0 as libc::c_int != 0;
    }
    if !pstart.is_null() {
        *pstart = *pp;
    }
    *pp = (*pp).offset(n as isize);
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn stab_demangle_args(
    mut minfo: *mut stab_demangle_info,
    mut pp: *mut *const libc::c_char,
    mut pargs: *mut *mut debug_type,
    mut pvarargs: *mut bool,
) -> bool {
    let mut orig: *const libc::c_char = 0 as *const libc::c_char;
    let mut alloc: libc::c_uint = 0;
    let mut count: libc::c_uint = 0;
    orig = *pp;
    alloc = 10 as libc::c_int as libc::c_uint;
    if !pargs.is_null() {
        *pargs = xmalloc(
            (alloc as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<debug_type>() as libc::c_ulong),
        ) as *mut debug_type;
        *pvarargs = 0 as libc::c_int != 0;
    }
    count = 0 as libc::c_int as libc::c_uint;
    while **pp as libc::c_int != '_' as i32 && **pp as libc::c_int != '\0' as i32
        && **pp as libc::c_int != 'e' as i32
    {
        if **pp as libc::c_int == 'N' as i32 || **pp as libc::c_int == 'T' as i32 {
            let mut temptype: libc::c_char = 0;
            let mut r: libc::c_uint = 0;
            let mut t: libc::c_uint = 0;
            temptype = **pp;
            *pp = (*pp).offset(1);
            let _ = *pp;
            if temptype as libc::c_int == 'T' as i32 {
                r = 1 as libc::c_int as libc::c_uint;
            } else if !stab_demangle_get_count(pp, &mut r) {
                stab_bad_demangle(orig);
                return 0 as libc::c_int != 0;
            }
            if !stab_demangle_get_count(pp, &mut t) {
                stab_bad_demangle(orig);
                return 0 as libc::c_int != 0;
            }
            if t >= (*minfo).typestring_count {
                stab_bad_demangle(orig);
                return 0 as libc::c_int != 0;
            }
            loop {
                let fresh24 = r;
                r = r.wrapping_sub(1);
                if !(fresh24 > 0 as libc::c_int as libc::c_uint) {
                    break;
                }
                let mut tem: *const libc::c_char = 0 as *const libc::c_char;
                tem = (*((*minfo).typestrings).offset(t as isize)).typestring;
                if !stab_demangle_arg(minfo, &mut tem, pargs, &mut count, &mut alloc) {
                    return 0 as libc::c_int != 0;
                }
            }
        } else if !stab_demangle_arg(minfo, pp, pargs, &mut count, &mut alloc) {
            return 0 as libc::c_int != 0
        }
    }
    if !pargs.is_null() {
        let ref mut fresh25 = *(*pargs).offset(count as isize);
        *fresh25 = 0 as *mut libc::c_void as debug_type;
    }
    if **pp as libc::c_int == 'e' as i32 {
        if !pargs.is_null() {
            *pvarargs = 1 as libc::c_int != 0;
        }
        *pp = (*pp).offset(1);
        let _ = *pp;
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn stab_demangle_arg(
    mut minfo: *mut stab_demangle_info,
    mut pp: *mut *const libc::c_char,
    mut pargs: *mut *mut debug_type,
    mut pcount: *mut libc::c_uint,
    mut palloc: *mut libc::c_uint,
) -> bool {
    let mut start: *const libc::c_char = 0 as *const libc::c_char;
    let mut type_0: debug_type = 0 as *mut debug_type_s;
    start = *pp;
    if !stab_demangle_type(
        minfo,
        pp,
        if pargs.is_null() {
            0 as *mut libc::c_void as *mut debug_type
        } else {
            &mut type_0
        },
    )
        || !stab_demangle_remember_type(
            minfo,
            start,
            (*pp).offset_from(start) as libc::c_long as libc::c_int,
        )
    {
        return 0 as libc::c_int != 0;
    }
    if !pargs.is_null() {
        if type_0.is_null() {
            return 0 as libc::c_int != 0;
        }
        if (*pcount).wrapping_add(1 as libc::c_int as libc::c_uint) >= *palloc {
            *palloc = (*palloc).wrapping_add(10 as libc::c_int as libc::c_uint);
            *pargs = xrealloc(
                *pargs as *mut libc::c_void,
                (*palloc as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<debug_type>() as libc::c_ulong),
            ) as *mut debug_type;
        }
        let ref mut fresh26 = *(*pargs).offset(*pcount as isize);
        *fresh26 = type_0;
        *pcount = (*pcount).wrapping_add(1);
        let _ = *pcount;
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn stab_demangle_type(
    mut minfo: *mut stab_demangle_info,
    mut pp: *mut *const libc::c_char,
    mut ptype: *mut debug_type,
) -> bool {
    let mut orig: *const libc::c_char = 0 as *const libc::c_char;
    orig = *pp;
    match **pp as libc::c_int {
        80 | 112 => {
            *pp = (*pp).offset(1);
            let _ = *pp;
            if !stab_demangle_type(minfo, pp, ptype) {
                return 0 as libc::c_int != 0;
            }
            if !ptype.is_null() {
                *ptype = debug_make_pointer_type((*minfo).dhandle, *ptype);
            }
        }
        82 => {
            *pp = (*pp).offset(1);
            let _ = *pp;
            if !stab_demangle_type(minfo, pp, ptype) {
                return 0 as libc::c_int != 0;
            }
            if !ptype.is_null() {
                *ptype = debug_make_reference_type((*minfo).dhandle, *ptype);
            }
        }
        65 => {
            let mut high: libc::c_ulong = 0;
            *pp = (*pp).offset(1);
            let _ = *pp;
            high = 0 as libc::c_int as libc::c_ulong;
            while **pp as libc::c_int != '\0' as i32 && **pp as libc::c_int != '_' as i32
            {
                if _sch_istable[(**pp as libc::c_int & 0xff as libc::c_int) as usize]
                    as libc::c_int
                    & _sch_isdigit as libc::c_int as libc::c_ushort as libc::c_int == 0
                {
                    stab_bad_demangle(orig);
                    return 0 as libc::c_int != 0;
                }
                high = high.wrapping_mul(10 as libc::c_int as libc::c_ulong);
                high = high
                    .wrapping_add((**pp as libc::c_int - '0' as i32) as libc::c_ulong);
                *pp = (*pp).offset(1);
                let _ = *pp;
            }
            if **pp as libc::c_int != '_' as i32 {
                stab_bad_demangle(orig);
                return 0 as libc::c_int != 0;
            }
            *pp = (*pp).offset(1);
            let _ = *pp;
            if !stab_demangle_type(minfo, pp, ptype) {
                return 0 as libc::c_int != 0;
            }
            if !ptype.is_null() {
                let mut int_type: debug_type = 0 as *mut debug_type_s;
                int_type = debug_find_named_type(
                    (*minfo).dhandle,
                    b"int\0" as *const u8 as *const libc::c_char,
                );
                if int_type.is_null() {
                    int_type = debug_make_int_type(
                        (*minfo).dhandle,
                        4 as libc::c_int as libc::c_uint,
                        0 as libc::c_int != 0,
                    );
                }
                *ptype = debug_make_array_type(
                    (*minfo).dhandle,
                    *ptype,
                    int_type,
                    0 as libc::c_int as bfd_signed_vma,
                    high as bfd_signed_vma,
                    0 as libc::c_int != 0,
                );
            }
        }
        84 => {
            let mut i: libc::c_uint = 0;
            let mut p: *const libc::c_char = 0 as *const libc::c_char;
            *pp = (*pp).offset(1);
            let _ = *pp;
            if !stab_demangle_get_count(pp, &mut i) {
                stab_bad_demangle(orig);
                return 0 as libc::c_int != 0;
            }
            if i >= (*minfo).typestring_count {
                stab_bad_demangle(orig);
                return 0 as libc::c_int != 0;
            }
            p = (*((*minfo).typestrings).offset(i as isize)).typestring;
            if !stab_demangle_type(minfo, &mut p, ptype) {
                return 0 as libc::c_int != 0;
            }
        }
        70 => {
            let mut args: *mut debug_type = 0 as *mut debug_type;
            let mut varargs: bool = false;
            *pp = (*pp).offset(1);
            let _ = *pp;
            if !stab_demangle_args(
                minfo,
                pp,
                if ptype.is_null() {
                    0 as *mut libc::c_void as *mut *mut debug_type
                } else {
                    &mut args
                },
                if ptype.is_null() {
                    0 as *mut libc::c_void as *mut bool
                } else {
                    &mut varargs
                },
            ) {
                return 0 as libc::c_int != 0;
            }
            if **pp as libc::c_int != '_' as i32 {
                stab_bad_demangle(orig);
                return 0 as libc::c_int != 0;
            }
            *pp = (*pp).offset(1);
            let _ = *pp;
            if !stab_demangle_type(minfo, pp, ptype) {
                return 0 as libc::c_int != 0;
            }
            if !ptype.is_null() {
                *ptype = debug_make_function_type(
                    (*minfo).dhandle,
                    *ptype,
                    args,
                    varargs,
                );
            }
        }
        77 | 79 => {
            let mut memberp: bool = false;
            let mut class_type: debug_type = 0 as *mut libc::c_void as debug_type;
            let mut args_0: *mut debug_type = 0 as *mut debug_type;
            let mut varargs_0: bool = false;
            let mut n: libc::c_uint = 0;
            let mut name: *const libc::c_char = 0 as *const libc::c_char;
            memberp = **pp as libc::c_int == 'M' as i32;
            args_0 = 0 as *mut debug_type;
            varargs_0 = 0 as libc::c_int != 0;
            *pp = (*pp).offset(1);
            let _ = *pp;
            if _sch_istable[(**pp as libc::c_int & 0xff as libc::c_int) as usize]
                as libc::c_int
                & _sch_isdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
            {
                n = stab_demangle_count(pp);
                if strlen(*pp) < n as libc::c_ulong {
                    stab_bad_demangle(orig);
                    return 0 as libc::c_int != 0;
                }
                name = *pp;
                *pp = (*pp).offset(n as isize);
                if !ptype.is_null() {
                    class_type = stab_find_tagged_type(
                        (*minfo).dhandle,
                        (*minfo).info,
                        name,
                        n as libc::c_int,
                        DEBUG_KIND_CLASS,
                    );
                    if class_type.is_null() {
                        return 0 as libc::c_int != 0;
                    }
                }
            } else if **pp as libc::c_int == 'Q' as i32 {
                if !stab_demangle_qualified(
                    minfo,
                    pp,
                    if ptype.is_null() {
                        0 as *mut libc::c_void as *mut debug_type
                    } else {
                        &mut class_type
                    },
                ) {
                    return 0 as libc::c_int != 0;
                }
            } else {
                stab_bad_demangle(orig);
                return 0 as libc::c_int != 0;
            }
            if memberp {
                if **pp as libc::c_int == 'C' as i32 {
                    *pp = (*pp).offset(1);
                    let _ = *pp;
                } else if **pp as libc::c_int == 'V' as i32 {
                    *pp = (*pp).offset(1);
                    let _ = *pp;
                }
                if **pp as libc::c_int != 'F' as i32 {
                    stab_bad_demangle(orig);
                    return 0 as libc::c_int != 0;
                }
                *pp = (*pp).offset(1);
                let _ = *pp;
                if !stab_demangle_args(
                    minfo,
                    pp,
                    if ptype.is_null() {
                        0 as *mut libc::c_void as *mut *mut debug_type
                    } else {
                        &mut args_0
                    },
                    if ptype.is_null() {
                        0 as *mut libc::c_void as *mut bool
                    } else {
                        &mut varargs_0
                    },
                ) {
                    return 0 as libc::c_int != 0;
                }
            }
            if **pp as libc::c_int != '_' as i32 {
                stab_bad_demangle(orig);
                return 0 as libc::c_int != 0;
            }
            *pp = (*pp).offset(1);
            let _ = *pp;
            if !stab_demangle_type(minfo, pp, ptype) {
                return 0 as libc::c_int != 0;
            }
            if !ptype.is_null() {
                if !memberp {
                    *ptype = debug_make_offset_type(
                        (*minfo).dhandle,
                        class_type,
                        *ptype,
                    );
                } else {
                    *ptype = debug_make_method_type(
                        (*minfo).dhandle,
                        *ptype,
                        class_type,
                        args_0,
                        varargs_0,
                    );
                }
            }
        }
        71 => {
            *pp = (*pp).offset(1);
            let _ = *pp;
            if !stab_demangle_type(minfo, pp, ptype) {
                return 0 as libc::c_int != 0;
            }
        }
        67 => {
            *pp = (*pp).offset(1);
            let _ = *pp;
            if !stab_demangle_type(minfo, pp, ptype) {
                return 0 as libc::c_int != 0;
            }
            if !ptype.is_null() {
                *ptype = debug_make_const_type((*minfo).dhandle, *ptype);
            }
        }
        81 => {
            if !stab_demangle_qualified(minfo, pp, ptype) {
                return 0 as libc::c_int != 0;
            }
        }
        _ => {
            if !stab_demangle_fund_type(minfo, pp, ptype) {
                return 0 as libc::c_int != 0;
            }
        }
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn stab_demangle_fund_type(
    mut minfo: *mut stab_demangle_info,
    mut pp: *mut *const libc::c_char,
    mut ptype: *mut debug_type,
) -> bool {
    let mut orig: *const libc::c_char = 0 as *const libc::c_char;
    let mut constp: bool = false;
    let mut volatilep: bool = false;
    let mut unsignedp: bool = false;
    let mut signedp: bool = false;
    let mut done: bool = false;
    orig = *pp;
    constp = 0 as libc::c_int != 0;
    volatilep = 0 as libc::c_int != 0;
    unsignedp = 0 as libc::c_int != 0;
    signedp = 0 as libc::c_int != 0;
    done = 0 as libc::c_int != 0;
    while !done {
        match **pp as libc::c_int {
            67 => {
                constp = 1 as libc::c_int != 0;
                *pp = (*pp).offset(1);
                let _ = *pp;
            }
            85 => {
                unsignedp = 1 as libc::c_int != 0;
                *pp = (*pp).offset(1);
                let _ = *pp;
            }
            83 => {
                signedp = 1 as libc::c_int != 0;
                *pp = (*pp).offset(1);
                let _ = *pp;
            }
            86 => {
                volatilep = 1 as libc::c_int != 0;
                *pp = (*pp).offset(1);
                let _ = *pp;
            }
            _ => {
                done = 1 as libc::c_int != 0;
            }
        }
    }
    let mut current_block_113: u64;
    match **pp as libc::c_int {
        0 | 95 => {
            stab_bad_demangle(orig);
            current_block_113 = 11865390570819897086;
        }
        118 => {
            if !ptype.is_null() {
                *ptype = debug_find_named_type(
                    (*minfo).dhandle,
                    b"void\0" as *const u8 as *const libc::c_char,
                );
                if (*ptype).is_null() {
                    *ptype = debug_make_void_type((*minfo).dhandle);
                }
            }
            *pp = (*pp).offset(1);
            let _ = *pp;
            current_block_113 = 11865390570819897086;
        }
        120 => {
            if !ptype.is_null() {
                *ptype = debug_find_named_type(
                    (*minfo).dhandle,
                    if unsignedp as libc::c_int != 0 {
                        b"long long unsigned int\0" as *const u8 as *const libc::c_char
                    } else {
                        b"long long int\0" as *const u8 as *const libc::c_char
                    },
                );
                if (*ptype).is_null() {
                    *ptype = debug_make_int_type(
                        (*minfo).dhandle,
                        8 as libc::c_int as libc::c_uint,
                        unsignedp,
                    );
                }
            }
            *pp = (*pp).offset(1);
            let _ = *pp;
            current_block_113 = 11865390570819897086;
        }
        108 => {
            if !ptype.is_null() {
                *ptype = debug_find_named_type(
                    (*minfo).dhandle,
                    if unsignedp as libc::c_int != 0 {
                        b"long unsigned int\0" as *const u8 as *const libc::c_char
                    } else {
                        b"long int\0" as *const u8 as *const libc::c_char
                    },
                );
                if (*ptype).is_null() {
                    *ptype = debug_make_int_type(
                        (*minfo).dhandle,
                        4 as libc::c_int as libc::c_uint,
                        unsignedp,
                    );
                }
            }
            *pp = (*pp).offset(1);
            let _ = *pp;
            current_block_113 = 11865390570819897086;
        }
        105 => {
            if !ptype.is_null() {
                *ptype = debug_find_named_type(
                    (*minfo).dhandle,
                    if unsignedp as libc::c_int != 0 {
                        b"unsigned int\0" as *const u8 as *const libc::c_char
                    } else {
                        b"int\0" as *const u8 as *const libc::c_char
                    },
                );
                if (*ptype).is_null() {
                    *ptype = debug_make_int_type(
                        (*minfo).dhandle,
                        4 as libc::c_int as libc::c_uint,
                        unsignedp,
                    );
                }
            }
            *pp = (*pp).offset(1);
            let _ = *pp;
            current_block_113 = 11865390570819897086;
        }
        115 => {
            if !ptype.is_null() {
                *ptype = debug_find_named_type(
                    (*minfo).dhandle,
                    if unsignedp as libc::c_int != 0 {
                        b"short unsigned int\0" as *const u8 as *const libc::c_char
                    } else {
                        b"short int\0" as *const u8 as *const libc::c_char
                    },
                );
                if (*ptype).is_null() {
                    *ptype = debug_make_int_type(
                        (*minfo).dhandle,
                        2 as libc::c_int as libc::c_uint,
                        unsignedp,
                    );
                }
            }
            *pp = (*pp).offset(1);
            let _ = *pp;
            current_block_113 = 11865390570819897086;
        }
        98 => {
            if !ptype.is_null() {
                *ptype = debug_find_named_type(
                    (*minfo).dhandle,
                    b"bool\0" as *const u8 as *const libc::c_char,
                );
                if (*ptype).is_null() {
                    *ptype = debug_make_bool_type(
                        (*minfo).dhandle,
                        4 as libc::c_int as libc::c_uint,
                    );
                }
            }
            *pp = (*pp).offset(1);
            let _ = *pp;
            current_block_113 = 11865390570819897086;
        }
        99 => {
            if !ptype.is_null() {
                *ptype = debug_find_named_type(
                    (*minfo).dhandle,
                    if unsignedp as libc::c_int != 0 {
                        b"unsigned char\0" as *const u8 as *const libc::c_char
                    } else if signedp as libc::c_int != 0 {
                        b"signed char\0" as *const u8 as *const libc::c_char
                    } else {
                        b"char\0" as *const u8 as *const libc::c_char
                    },
                );
                if (*ptype).is_null() {
                    *ptype = debug_make_int_type(
                        (*minfo).dhandle,
                        1 as libc::c_int as libc::c_uint,
                        unsignedp,
                    );
                }
            }
            *pp = (*pp).offset(1);
            let _ = *pp;
            current_block_113 = 11865390570819897086;
        }
        119 => {
            if !ptype.is_null() {
                *ptype = debug_find_named_type(
                    (*minfo).dhandle,
                    b"__wchar_t\0" as *const u8 as *const libc::c_char,
                );
                if (*ptype).is_null() {
                    *ptype = debug_make_int_type(
                        (*minfo).dhandle,
                        2 as libc::c_int as libc::c_uint,
                        1 as libc::c_int != 0,
                    );
                }
            }
            *pp = (*pp).offset(1);
            let _ = *pp;
            current_block_113 = 11865390570819897086;
        }
        114 => {
            if !ptype.is_null() {
                *ptype = debug_find_named_type(
                    (*minfo).dhandle,
                    b"long double\0" as *const u8 as *const libc::c_char,
                );
                if (*ptype).is_null() {
                    *ptype = debug_make_float_type(
                        (*minfo).dhandle,
                        8 as libc::c_int as libc::c_uint,
                    );
                }
            }
            *pp = (*pp).offset(1);
            let _ = *pp;
            current_block_113 = 11865390570819897086;
        }
        100 => {
            if !ptype.is_null() {
                *ptype = debug_find_named_type(
                    (*minfo).dhandle,
                    b"double\0" as *const u8 as *const libc::c_char,
                );
                if (*ptype).is_null() {
                    *ptype = debug_make_float_type(
                        (*minfo).dhandle,
                        8 as libc::c_int as libc::c_uint,
                    );
                }
            }
            *pp = (*pp).offset(1);
            let _ = *pp;
            current_block_113 = 11865390570819897086;
        }
        102 => {
            if !ptype.is_null() {
                *ptype = debug_find_named_type(
                    (*minfo).dhandle,
                    b"float\0" as *const u8 as *const libc::c_char,
                );
                if (*ptype).is_null() {
                    *ptype = debug_make_float_type(
                        (*minfo).dhandle,
                        4 as libc::c_int as libc::c_uint,
                    );
                }
            }
            *pp = (*pp).offset(1);
            let _ = *pp;
            current_block_113 = 11865390570819897086;
        }
        71 => {
            *pp = (*pp).offset(1);
            let _ = *pp;
            if _sch_istable[(**pp as libc::c_int & 0xff as libc::c_int) as usize]
                as libc::c_int
                & _sch_isdigit as libc::c_int as libc::c_ushort as libc::c_int == 0
            {
                stab_bad_demangle(orig);
                return 0 as libc::c_int != 0;
            }
            current_block_113 = 3737796672379282751;
        }
        48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
            current_block_113 = 3737796672379282751;
        }
        116 => {
            let mut name_0: *mut libc::c_char = 0 as *mut libc::c_char;
            if !stab_demangle_template(
                minfo,
                pp,
                if !ptype.is_null() { &mut name_0 } else { 0 as *mut *mut libc::c_char },
            ) {
                return 0 as libc::c_int != 0;
            }
            if !ptype.is_null() {
                *ptype = stab_find_tagged_type(
                    (*minfo).dhandle,
                    (*minfo).info,
                    name_0,
                    strlen(name_0) as libc::c_int,
                    DEBUG_KIND_CLASS,
                );
                free(name_0 as *mut libc::c_void);
                if (*ptype).is_null() {
                    return 0 as libc::c_int != 0;
                }
            }
            current_block_113 = 11865390570819897086;
        }
        _ => {
            stab_bad_demangle(orig);
            return 0 as libc::c_int != 0;
        }
    }
    match current_block_113 {
        3737796672379282751 => {
            let mut hold: *const libc::c_char = 0 as *const libc::c_char;
            if !stab_demangle_class(minfo, pp, &mut hold) {
                return 0 as libc::c_int != 0;
            }
            if !ptype.is_null() {
                let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
                name = savestring(
                    hold,
                    (*pp).offset_from(hold) as libc::c_long as libc::c_int,
                );
                *ptype = debug_find_named_type((*minfo).dhandle, name);
                free(name as *mut libc::c_void);
                if (*ptype).is_null() {
                    *ptype = stab_find_tagged_type(
                        (*minfo).dhandle,
                        (*minfo).info,
                        hold,
                        (*pp).offset_from(hold) as libc::c_long as libc::c_int,
                        DEBUG_KIND_ILLEGAL,
                    );
                    if (*ptype).is_null() {
                        return 0 as libc::c_int != 0;
                    }
                }
            }
        }
        _ => {}
    }
    if !ptype.is_null() {
        if constp {
            *ptype = debug_make_const_type((*minfo).dhandle, *ptype);
        }
        if volatilep {
            *ptype = debug_make_volatile_type((*minfo).dhandle, *ptype);
        }
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn stab_demangle_remember_type(
    mut minfo: *mut stab_demangle_info,
    mut p: *const libc::c_char,
    mut len: libc::c_int,
) -> bool {
    if (*minfo).typestring_count >= (*minfo).typestring_alloc {
        (*minfo)
            .typestring_alloc = ((*minfo).typestring_alloc)
            .wrapping_add(10 as libc::c_int as libc::c_uint);
        (*minfo)
            .typestrings = xrealloc(
            (*minfo).typestrings as *mut libc::c_void,
            ((*minfo).typestring_alloc as libc::c_ulong)
                .wrapping_mul(
                    ::core::mem::size_of::<stab_demangle_typestring>() as libc::c_ulong,
                ),
        ) as *mut stab_demangle_typestring;
    }
    let ref mut fresh27 = (*((*minfo).typestrings)
        .offset((*minfo).typestring_count as isize))
        .typestring;
    *fresh27 = p;
    (*((*minfo).typestrings).offset((*minfo).typestring_count as isize))
        .len = len as libc::c_uint;
    (*minfo).typestring_count = ((*minfo).typestring_count).wrapping_add(1);
    (*minfo).typestring_count;
    return 1 as libc::c_int != 0;
}
