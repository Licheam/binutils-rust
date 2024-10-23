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
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
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
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn bfd_map_over_sections(
        abfd: *mut bfd,
        func: Option::<
            unsafe extern "C" fn(*mut bfd, *mut asection, *mut libc::c_void) -> (),
        >,
        obj: *mut libc::c_void,
    );
    fn bfd_scan_vma(
        string: *const libc::c_char,
        end: *mut *const libc::c_char,
        base: libc::c_int,
    ) -> bfd_vma;
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    fn xrealloc(_: *mut libc::c_void, _: size_t) -> *mut libc::c_void;
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn debug_write(
        _: *mut libc::c_void,
        _: *const debug_write_fns,
        _: *mut libc::c_void,
    ) -> bool;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct debug_write_fns {
    pub start_compilation_unit: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char) -> bool,
    >,
    pub start_source: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char) -> bool,
    >,
    pub empty_type: Option::<unsafe extern "C" fn(*mut libc::c_void) -> bool>,
    pub void_type: Option::<unsafe extern "C" fn(*mut libc::c_void) -> bool>,
    pub int_type: Option::<
        unsafe extern "C" fn(*mut libc::c_void, libc::c_uint, bool) -> bool,
    >,
    pub float_type: Option::<
        unsafe extern "C" fn(*mut libc::c_void, libc::c_uint) -> bool,
    >,
    pub complex_type: Option::<
        unsafe extern "C" fn(*mut libc::c_void, libc::c_uint) -> bool,
    >,
    pub bool_type: Option::<
        unsafe extern "C" fn(*mut libc::c_void, libc::c_uint) -> bool,
    >,
    pub enum_type: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const libc::c_char,
            *mut *const libc::c_char,
            *mut bfd_signed_vma,
        ) -> bool,
    >,
    pub pointer_type: Option::<unsafe extern "C" fn(*mut libc::c_void) -> bool>,
    pub function_type: Option::<
        unsafe extern "C" fn(*mut libc::c_void, libc::c_int, bool) -> bool,
    >,
    pub reference_type: Option::<unsafe extern "C" fn(*mut libc::c_void) -> bool>,
    pub range_type: Option::<
        unsafe extern "C" fn(*mut libc::c_void, bfd_signed_vma, bfd_signed_vma) -> bool,
    >,
    pub array_type: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            bfd_signed_vma,
            bfd_signed_vma,
            bool,
        ) -> bool,
    >,
    pub set_type: Option::<unsafe extern "C" fn(*mut libc::c_void, bool) -> bool>,
    pub offset_type: Option::<unsafe extern "C" fn(*mut libc::c_void) -> bool>,
    pub method_type: Option::<
        unsafe extern "C" fn(*mut libc::c_void, bool, libc::c_int, bool) -> bool,
    >,
    pub const_type: Option::<unsafe extern "C" fn(*mut libc::c_void) -> bool>,
    pub volatile_type: Option::<unsafe extern "C" fn(*mut libc::c_void) -> bool>,
    pub start_struct_type: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const libc::c_char,
            libc::c_uint,
            bool,
            libc::c_uint,
        ) -> bool,
    >,
    pub struct_field: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const libc::c_char,
            bfd_vma,
            bfd_vma,
            debug_visibility,
        ) -> bool,
    >,
    pub end_struct_type: Option::<unsafe extern "C" fn(*mut libc::c_void) -> bool>,
    pub start_class_type: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const libc::c_char,
            libc::c_uint,
            bool,
            libc::c_uint,
            bool,
            bool,
        ) -> bool,
    >,
    pub class_static_member: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const libc::c_char,
            *const libc::c_char,
            debug_visibility,
        ) -> bool,
    >,
    pub class_baseclass: Option::<
        unsafe extern "C" fn(*mut libc::c_void, bfd_vma, bool, debug_visibility) -> bool,
    >,
    pub class_start_method: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char) -> bool,
    >,
    pub class_method_variant: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const libc::c_char,
            debug_visibility,
            bool,
            bool,
            bfd_vma,
            bool,
        ) -> bool,
    >,
    pub class_static_method_variant: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const libc::c_char,
            debug_visibility,
            bool,
            bool,
        ) -> bool,
    >,
    pub class_end_method: Option::<unsafe extern "C" fn(*mut libc::c_void) -> bool>,
    pub end_class_type: Option::<unsafe extern "C" fn(*mut libc::c_void) -> bool>,
    pub typedef_type: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char) -> bool,
    >,
    pub tag_type: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const libc::c_char,
            libc::c_uint,
            debug_type_kind,
        ) -> bool,
    >,
    pub typdef: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char) -> bool,
    >,
    pub tag: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char) -> bool,
    >,
    pub int_constant: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char, bfd_vma) -> bool,
    >,
    pub float_constant: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const libc::c_char,
            libc::c_double,
        ) -> bool,
    >,
    pub typed_constant: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char, bfd_vma) -> bool,
    >,
    pub variable: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const libc::c_char,
            debug_var_kind,
            bfd_vma,
        ) -> bool,
    >,
    pub start_function: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char, bool) -> bool,
    >,
    pub function_parameter: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const libc::c_char,
            debug_parm_kind,
            bfd_vma,
        ) -> bool,
    >,
    pub start_block: Option::<unsafe extern "C" fn(*mut libc::c_void, bfd_vma) -> bool>,
    pub end_block: Option::<unsafe extern "C" fn(*mut libc::c_void, bfd_vma) -> bool>,
    pub end_function: Option::<unsafe extern "C" fn(*mut libc::c_void) -> bool>,
    pub lineno: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const libc::c_char,
            libc::c_ulong,
            bfd_vma,
        ) -> bool,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pr_handle {
    pub f: *mut FILE,
    pub indent: libc::c_uint,
    pub stack: *mut pr_stack,
    pub parameter: libc::c_int,
    pub filename: *mut libc::c_char,
    pub abfd: *mut bfd,
    pub syms: *mut *mut asymbol,
    pub demangler: Option::<
        unsafe extern "C" fn(
            *mut bfd,
            *const libc::c_char,
            libc::c_int,
        ) -> *mut libc::c_char,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pr_stack {
    pub next: *mut pr_stack,
    pub type_0: *mut libc::c_char,
    pub visibility: debug_visibility,
    pub method: *const libc::c_char,
    pub flavor: *const libc::c_char,
    pub parents: *mut libc::c_char,
    pub num_parents: libc::c_int,
}
#[inline]
unsafe extern "C" fn startswith(
    mut str: *const libc::c_char,
    mut prefix: *const libc::c_char,
) -> bool {
    return strncmp(str, prefix, strlen(prefix)) == 0 as libc::c_int;
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
unsafe extern "C" fn bfd_section_flags(mut sec: *const asection) -> flagword {
    return (*sec).flags;
}
#[no_mangle]
pub unsafe extern "C" fn print_debugging_info(
    mut f: *mut FILE,
    mut dhandle: *mut libc::c_void,
    mut abfd: *mut bfd,
    mut syms: *mut *mut asymbol,
    mut demangler: Option::<
        unsafe extern "C" fn(
            *mut bfd,
            *const libc::c_char,
            libc::c_int,
        ) -> *mut libc::c_char,
    >,
    mut as_tags: bool,
) -> bool {
    let mut info: pr_handle = pr_handle {
        f: 0 as *mut FILE,
        indent: 0,
        stack: 0 as *mut pr_stack,
        parameter: 0,
        filename: 0 as *mut libc::c_char,
        abfd: 0 as *mut bfd,
        syms: 0 as *mut *mut asymbol,
        demangler: None,
    };
    info.f = f;
    info.indent = 0 as libc::c_int as libc::c_uint;
    info.stack = 0 as *mut pr_stack;
    info.parameter = 0 as libc::c_int;
    info.filename = 0 as *mut libc::c_char;
    info.abfd = abfd;
    info.syms = syms;
    info.demangler = demangler;
    if as_tags {
        fputs(
            b"!_TAG_FILE_FORMAT\t2\t/extended format/\n\0" as *const u8
                as *const libc::c_char,
            f,
        );
        fputs(
            b"!_TAG_FILE_SORTED\t0\t/0=unsorted, 1=sorted/\n\0" as *const u8
                as *const libc::c_char,
            f,
        );
        fputs(
            b"!_TAG_PROGRAM_AUTHOR\tIan Lance Taylor, Salvador E. Tropea and others\t//\n\0"
                as *const u8 as *const libc::c_char,
            f,
        );
        fputs(
            b"!_TAG_PROGRAM_NAME\tobjdump\t/From GNU binutils/\n\0" as *const u8
                as *const libc::c_char,
            f,
        );
    }
    return if as_tags as libc::c_int != 0 {
        debug_write(dhandle, &tg_fns, &mut info as *mut pr_handle as *mut libc::c_void)
            as libc::c_int
    } else {
        debug_write(dhandle, &pr_fns, &mut info as *mut pr_handle as *mut libc::c_void)
            as libc::c_int
    } != 0;
}
unsafe extern "C" fn indent(mut info: *mut pr_handle) {
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*info).indent {
        putc(' ' as i32, (*info).f);
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn push_type(
    mut info: *mut pr_handle,
    mut type_0: *const libc::c_char,
) -> bool {
    let mut n: *mut pr_stack = 0 as *mut pr_stack;
    if type_0.is_null() {
        return 0 as libc::c_int != 0;
    }
    n = xmalloc(::core::mem::size_of::<pr_stack>() as libc::c_ulong) as *mut pr_stack;
    memset(
        n as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<pr_stack>() as libc::c_ulong,
    );
    (*n).type_0 = xstrdup(type_0);
    (*n).visibility = DEBUG_VISIBILITY_IGNORE;
    (*n).method = 0 as *const libc::c_char;
    (*n).next = (*info).stack;
    (*info).stack = n;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn prepend_type(
    mut info: *mut pr_handle,
    mut s: *const libc::c_char,
) -> bool {
    let mut n: *mut libc::c_char = 0 as *mut libc::c_char;
    if !((*info).stack).is_null() {} else {
        __assert_fail(
            b"info->stack != NULL\0" as *const u8 as *const libc::c_char,
            b"prdbg.c\0" as *const u8 as *const libc::c_char,
            355 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 53],
                &[libc::c_char; 53],
            >(b"_Bool prepend_type(struct pr_handle *, const char *)\0"))
                .as_ptr(),
        );
    }
    'c_11219: {
        if !((*info).stack).is_null() {} else {
            __assert_fail(
                b"info->stack != NULL\0" as *const u8 as *const libc::c_char,
                b"prdbg.c\0" as *const u8 as *const libc::c_char,
                355 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 53],
                    &[libc::c_char; 53],
                >(b"_Bool prepend_type(struct pr_handle *, const char *)\0"))
                    .as_ptr(),
            );
        }
    };
    n = xmalloc(
        (strlen(s))
            .wrapping_add(strlen((*(*info).stack).type_0))
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    sprintf(
        n,
        b"%s%s\0" as *const u8 as *const libc::c_char,
        s,
        (*(*info).stack).type_0,
    );
    free((*(*info).stack).type_0 as *mut libc::c_void);
    (*(*info).stack).type_0 = n;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn append_type(
    mut info: *mut pr_handle,
    mut s: *const libc::c_char,
) -> bool {
    let mut len: libc::c_uint = 0;
    if s.is_null() {
        return 0 as libc::c_int != 0;
    }
    if !((*info).stack).is_null() {} else {
        __assert_fail(
            b"info->stack != NULL\0" as *const u8 as *const libc::c_char,
            b"prdbg.c\0" as *const u8 as *const libc::c_char,
            375 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 52],
                &[libc::c_char; 52],
            >(b"_Bool append_type(struct pr_handle *, const char *)\0"))
                .as_ptr(),
        );
    }
    'c_11047: {
        if !((*info).stack).is_null() {} else {
            __assert_fail(
                b"info->stack != NULL\0" as *const u8 as *const libc::c_char,
                b"prdbg.c\0" as *const u8 as *const libc::c_char,
                375 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 52],
                    &[libc::c_char; 52],
                >(b"_Bool append_type(struct pr_handle *, const char *)\0"))
                    .as_ptr(),
            );
        }
    };
    len = strlen((*(*info).stack).type_0) as libc::c_uint;
    (*(*info).stack)
        .type_0 = xrealloc(
        (*(*info).stack).type_0 as *mut libc::c_void,
        (len as libc::c_ulong)
            .wrapping_add(strlen(s))
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    strcpy(((*(*info).stack).type_0).offset(len as isize), s);
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn substitute_type(
    mut info: *mut pr_handle,
    mut s: *const libc::c_char,
) -> bool {
    let mut u: *mut libc::c_char = 0 as *mut libc::c_char;
    if !((*info).stack).is_null() {} else {
        __assert_fail(
            b"info->stack != NULL\0" as *const u8 as *const libc::c_char,
            b"prdbg.c\0" as *const u8 as *const libc::c_char,
            414 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 56],
                &[libc::c_char; 56],
            >(b"_Bool substitute_type(struct pr_handle *, const char *)\0"))
                .as_ptr(),
        );
    }
    'c_11437: {
        if !((*info).stack).is_null() {} else {
            __assert_fail(
                b"info->stack != NULL\0" as *const u8 as *const libc::c_char,
                b"prdbg.c\0" as *const u8 as *const libc::c_char,
                414 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 56],
                    &[libc::c_char; 56],
                >(b"_Bool substitute_type(struct pr_handle *, const char *)\0"))
                    .as_ptr(),
            );
        }
    };
    u = strchr((*(*info).stack).type_0, '|' as i32);
    if !u.is_null() {
        let mut n: *mut libc::c_char = 0 as *mut libc::c_char;
        n = xmalloc((strlen((*(*info).stack).type_0)).wrapping_add(strlen(s)))
            as *mut libc::c_char;
        memcpy(
            n as *mut libc::c_void,
            (*(*info).stack).type_0 as *const libc::c_void,
            u.offset_from((*(*info).stack).type_0) as libc::c_long as libc::c_ulong,
        );
        strcpy(
            n.offset(u.offset_from((*(*info).stack).type_0) as libc::c_long as isize),
            s,
        );
        strcat(n, u.offset(1 as libc::c_int as isize));
        free((*(*info).stack).type_0 as *mut libc::c_void);
        (*(*info).stack).type_0 = n;
        return 1 as libc::c_int != 0;
    }
    if !(strchr(s, '|' as i32)).is_null()
        && (!(strchr((*(*info).stack).type_0, '{' as i32)).is_null()
            || !(strchr((*(*info).stack).type_0, '(' as i32)).is_null())
    {
        if !prepend_type(info, b"(\0" as *const u8 as *const libc::c_char)
            || !append_type(info, b")\0" as *const u8 as *const libc::c_char)
        {
            return 0 as libc::c_int != 0;
        }
    }
    if *s as libc::c_int == '\0' as i32 {
        return 1 as libc::c_int != 0;
    }
    return append_type(info, b" \0" as *const u8 as *const libc::c_char) as libc::c_int
        != 0 && append_type(info, s) as libc::c_int != 0;
}
unsafe extern "C" fn indent_type(mut info: *mut pr_handle) -> bool {
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*info).indent {
        if !append_type(info, b" \0" as *const u8 as *const libc::c_char) {
            return 0 as libc::c_int != 0;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn pop_type(mut info: *mut pr_handle) -> *mut libc::c_char {
    let mut o: *mut pr_stack = 0 as *mut pr_stack;
    let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
    if !((*info).stack).is_null() {} else {
        __assert_fail(
            b"info->stack != NULL\0" as *const u8 as *const libc::c_char,
            b"prdbg.c\0" as *const u8 as *const libc::c_char,
            473 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 35],
                &[libc::c_char; 35],
            >(b"char *pop_type(struct pr_handle *)\0"))
                .as_ptr(),
        );
    }
    'c_10888: {
        if !((*info).stack).is_null() {} else {
            __assert_fail(
                b"info->stack != NULL\0" as *const u8 as *const libc::c_char,
                b"prdbg.c\0" as *const u8 as *const libc::c_char,
                473 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 35],
                    &[libc::c_char; 35],
                >(b"char *pop_type(struct pr_handle *)\0"))
                    .as_ptr(),
            );
        }
    };
    o = (*info).stack;
    (*info).stack = (*o).next;
    ret = (*o).type_0;
    free(o as *mut libc::c_void);
    return ret;
}
unsafe extern "C" fn print_vma(
    mut vma: bfd_vma,
    mut buf: *mut libc::c_char,
    mut unsignedp: bool,
    mut hexp: bool,
) {
    if ::core::mem::size_of::<bfd_vma>() as libc::c_ulong
        <= ::core::mem::size_of::<libc::c_ulong>() as libc::c_ulong
    {
        if hexp {
            sprintf(buf, b"0x%lx\0" as *const u8 as *const libc::c_char, vma);
        } else if unsignedp {
            sprintf(buf, b"%lu\0" as *const u8 as *const libc::c_char, vma);
        } else {
            sprintf(
                buf,
                b"%ld\0" as *const u8 as *const libc::c_char,
                vma as libc::c_long,
            );
        }
    } else {
        *buf.offset(0 as libc::c_int as isize) = '0' as i32 as libc::c_char;
        *buf.offset(1 as libc::c_int as isize) = 'x' as i32 as libc::c_char;
        sprintf(
            buf.offset(2 as libc::c_int as isize),
            b"%016lx\0" as *const u8 as *const libc::c_char,
            vma,
        );
    };
}
unsafe extern "C" fn pr_fix_visibility(
    mut info: *mut pr_handle,
    mut visibility: debug_visibility,
) -> bool {
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: libc::c_uint = 0;
    if !((*info).stack).is_null() {} else {
        __assert_fail(
            b"info->stack != NULL\0" as *const u8 as *const libc::c_char,
            b"prdbg.c\0" as *const u8 as *const libc::c_char,
            1100 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 67],
                &[libc::c_char; 67],
            >(b"_Bool pr_fix_visibility(struct pr_handle *, enum debug_visibility)\0"))
                .as_ptr(),
        );
    }
    'c_12959: {
        if !((*info).stack).is_null() {} else {
            __assert_fail(
                b"info->stack != NULL\0" as *const u8 as *const libc::c_char,
                b"prdbg.c\0" as *const u8 as *const libc::c_char,
                1100 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 67],
                    &[libc::c_char; 67],
                >(
                    b"_Bool pr_fix_visibility(struct pr_handle *, enum debug_visibility)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if (*(*info).stack).visibility as libc::c_uint == visibility as libc::c_uint {
        return 1 as libc::c_int != 0;
    }
    match visibility as libc::c_uint {
        0 => {
            s = b"public\0" as *const u8 as *const libc::c_char;
        }
        2 => {
            s = b"private\0" as *const u8 as *const libc::c_char;
        }
        1 => {
            s = b"protected\0" as *const u8 as *const libc::c_char;
        }
        3 => {
            s = b"/* ignore */\0" as *const u8 as *const libc::c_char;
        }
        _ => {
            abort();
        }
    }
    t = (*(*info).stack).type_0;
    len = strlen(t) as libc::c_uint;
    if *t.offset(len.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize)
        as libc::c_int == ' ' as i32
    {} else {
        __assert_fail(
            b"t[len - 1] == ' '\0" as *const u8 as *const libc::c_char,
            b"prdbg.c\0" as *const u8 as *const libc::c_char,
            1129 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 67],
                &[libc::c_char; 67],
            >(b"_Bool pr_fix_visibility(struct pr_handle *, enum debug_visibility)\0"))
                .as_ptr(),
        );
    }
    'c_12822: {
        if *t.offset(len.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize)
            as libc::c_int == ' ' as i32
        {} else {
            __assert_fail(
                b"t[len - 1] == ' '\0" as *const u8 as *const libc::c_char,
                b"prdbg.c\0" as *const u8 as *const libc::c_char,
                1129 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 67],
                    &[libc::c_char; 67],
                >(
                    b"_Bool pr_fix_visibility(struct pr_handle *, enum debug_visibility)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    *t
        .offset(
            len.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
        ) = '\0' as i32 as libc::c_char;
    if !append_type(info, s)
        || !append_type(info, b":\n\0" as *const u8 as *const libc::c_char)
        || !indent_type(info)
    {
        return 0 as libc::c_int != 0;
    }
    (*(*info).stack).visibility = visibility;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn pr_start_compilation_unit(
    mut p: *mut libc::c_void,
    mut filename_0: *const libc::c_char,
) -> bool {
    let mut info: *mut pr_handle = p as *mut pr_handle;
    if (*info).indent == 0 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"info->indent == 0\0" as *const u8 as *const libc::c_char,
            b"prdbg.c\0" as *const u8 as *const libc::c_char,
            532 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 54],
                &[libc::c_char; 54],
            >(b"_Bool pr_start_compilation_unit(void *, const char *)\0"))
                .as_ptr(),
        );
    }
    'c_17009: {
        if (*info).indent == 0 as libc::c_int as libc::c_uint {} else {
            __assert_fail(
                b"info->indent == 0\0" as *const u8 as *const libc::c_char,
                b"prdbg.c\0" as *const u8 as *const libc::c_char,
                532 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 54],
                    &[libc::c_char; 54],
                >(b"_Bool pr_start_compilation_unit(void *, const char *)\0"))
                    .as_ptr(),
            );
        }
    };
    fprintf((*info).f, b"%s:\n\0" as *const u8 as *const libc::c_char, filename_0);
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn pr_start_source(
    mut p: *mut libc::c_void,
    mut filename_0: *const libc::c_char,
) -> bool {
    let mut info: *mut pr_handle = p as *mut pr_handle;
    if (*info).indent == 0 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"info->indent == 0\0" as *const u8 as *const libc::c_char,
            b"prdbg.c\0" as *const u8 as *const libc::c_char,
            546 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 44],
                &[libc::c_char; 44],
            >(b"_Bool pr_start_source(void *, const char *)\0"))
                .as_ptr(),
        );
    }
    'c_16940: {
        if (*info).indent == 0 as libc::c_int as libc::c_uint {} else {
            __assert_fail(
                b"info->indent == 0\0" as *const u8 as *const libc::c_char,
                b"prdbg.c\0" as *const u8 as *const libc::c_char,
                546 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 44],
                    &[libc::c_char; 44],
                >(b"_Bool pr_start_source(void *, const char *)\0"))
                    .as_ptr(),
            );
        }
    };
    fprintf((*info).f, b" %s:\n\0" as *const u8 as *const libc::c_char, filename_0);
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn pr_empty_type(mut p: *mut libc::c_void) -> bool {
    let mut info: *mut pr_handle = p as *mut pr_handle;
    return push_type(info, b"<undefined>\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn pr_void_type(mut p: *mut libc::c_void) -> bool {
    let mut info: *mut pr_handle = p as *mut pr_handle;
    return push_type(info, b"void\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn pr_int_type(
    mut p: *mut libc::c_void,
    mut size: libc::c_uint,
    mut unsignedp: bool,
) -> bool {
    let mut info: *mut pr_handle = p as *mut pr_handle;
    let mut ab: [libc::c_char; 40] = [0; 40];
    sprintf(
        ab.as_mut_ptr(),
        b"%sint%d\0" as *const u8 as *const libc::c_char,
        if unsignedp as libc::c_int != 0 {
            b"u\0" as *const u8 as *const libc::c_char
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
        size.wrapping_mul(8 as libc::c_int as libc::c_uint),
    );
    return push_type(info, ab.as_mut_ptr());
}
unsafe extern "C" fn pr_float_type(
    mut p: *mut libc::c_void,
    mut size: libc::c_uint,
) -> bool {
    let mut info: *mut pr_handle = p as *mut pr_handle;
    let mut ab: [libc::c_char; 40] = [0; 40];
    if size == 4 as libc::c_int as libc::c_uint {
        return push_type(info, b"float\0" as *const u8 as *const libc::c_char)
    } else if size == 8 as libc::c_int as libc::c_uint {
        return push_type(info, b"double\0" as *const u8 as *const libc::c_char)
    }
    sprintf(
        ab.as_mut_ptr(),
        b"float%d\0" as *const u8 as *const libc::c_char,
        size.wrapping_mul(8 as libc::c_int as libc::c_uint),
    );
    return push_type(info, ab.as_mut_ptr());
}
unsafe extern "C" fn pr_complex_type(
    mut p: *mut libc::c_void,
    mut size: libc::c_uint,
) -> bool {
    let mut info: *mut pr_handle = p as *mut pr_handle;
    if !pr_float_type(p, size) {
        return 0 as libc::c_int != 0;
    }
    return prepend_type(info, b"complex \0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn pr_bool_type(
    mut p: *mut libc::c_void,
    mut size: libc::c_uint,
) -> bool {
    let mut info: *mut pr_handle = p as *mut pr_handle;
    let mut ab: [libc::c_char; 40] = [0; 40];
    sprintf(
        ab.as_mut_ptr(),
        b"bool%d\0" as *const u8 as *const libc::c_char,
        size.wrapping_mul(8 as libc::c_int as libc::c_uint),
    );
    return push_type(info, ab.as_mut_ptr());
}
unsafe extern "C" fn pr_enum_type(
    mut p: *mut libc::c_void,
    mut tag: *const libc::c_char,
    mut names: *mut *const libc::c_char,
    mut values: *mut bfd_signed_vma,
) -> bool {
    let mut info: *mut pr_handle = p as *mut pr_handle;
    let mut i: libc::c_uint = 0;
    let mut val: bfd_signed_vma = 0;
    if !push_type(info, b"enum \0" as *const u8 as *const libc::c_char) {
        return 0 as libc::c_int != 0;
    }
    if !tag.is_null() {
        if !append_type(info, tag)
            || !append_type(info, b" \0" as *const u8 as *const libc::c_char)
        {
            return 0 as libc::c_int != 0;
        }
    }
    if !append_type(info, b"{ \0" as *const u8 as *const libc::c_char) {
        return 0 as libc::c_int != 0;
    }
    if names.is_null() {
        if !append_type(info, b"/* undefined */\0" as *const u8 as *const libc::c_char) {
            return 0 as libc::c_int != 0;
        }
    } else {
        val = 0 as libc::c_int as bfd_signed_vma;
        i = 0 as libc::c_int as libc::c_uint;
        while !(*names.offset(i as isize)).is_null() {
            if i > 0 as libc::c_int as libc::c_uint {
                if !append_type(info, b", \0" as *const u8 as *const libc::c_char) {
                    return 0 as libc::c_int != 0;
                }
            }
            if !append_type(info, *names.offset(i as isize)) {
                return 0 as libc::c_int != 0;
            }
            if *values.offset(i as isize) != val {
                let mut ab: [libc::c_char; 22] = [0; 22];
                print_vma(
                    *values.offset(i as isize) as bfd_vma,
                    ab.as_mut_ptr(),
                    0 as libc::c_int != 0,
                    0 as libc::c_int != 0,
                );
                if !append_type(info, b" = \0" as *const u8 as *const libc::c_char)
                    || !append_type(info, ab.as_mut_ptr())
                {
                    return 0 as libc::c_int != 0;
                }
                val = *values.offset(i as isize);
            }
            val += 1;
            val;
            i = i.wrapping_add(1);
            i;
        }
    }
    return append_type(info, b" }\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn pr_pointer_type(mut p: *mut libc::c_void) -> bool {
    let mut info: *mut pr_handle = p as *mut pr_handle;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    if !((*info).stack).is_null() {} else {
        __assert_fail(
            b"info->stack != NULL\0" as *const u8 as *const libc::c_char,
            b"prdbg.c\0" as *const u8 as *const libc::c_char,
            694 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 30],
                &[libc::c_char; 30],
            >(b"_Bool pr_pointer_type(void *)\0"))
                .as_ptr(),
        );
    }
    'c_16437: {
        if !((*info).stack).is_null() {} else {
            __assert_fail(
                b"info->stack != NULL\0" as *const u8 as *const libc::c_char,
                b"prdbg.c\0" as *const u8 as *const libc::c_char,
                694 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 30],
                    &[libc::c_char; 30],
                >(b"_Bool pr_pointer_type(void *)\0"))
                    .as_ptr(),
            );
        }
    };
    s = strchr((*(*info).stack).type_0, '|' as i32);
    if !s.is_null() && *s.offset(1 as libc::c_int as isize) as libc::c_int == '[' as i32
    {
        return substitute_type(info, b"(*|)\0" as *const u8 as *const libc::c_char);
    }
    return substitute_type(info, b"*|\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn pr_function_type(
    mut p: *mut libc::c_void,
    mut argcount: libc::c_int,
    mut varargs: bool,
) -> bool {
    let mut info: *mut pr_handle = p as *mut pr_handle;
    let mut arg_types: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut len: libc::c_uint = 0;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    if !((*info).stack).is_null() {} else {
        __assert_fail(
            b"info->stack != NULL\0" as *const u8 as *const libc::c_char,
            b"prdbg.c\0" as *const u8 as *const libc::c_char,
            712 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 43],
                &[libc::c_char; 43],
            >(b"_Bool pr_function_type(void *, int, _Bool)\0"))
                .as_ptr(),
        );
    }
    'c_16332: {
        if !((*info).stack).is_null() {} else {
            __assert_fail(
                b"info->stack != NULL\0" as *const u8 as *const libc::c_char,
                b"prdbg.c\0" as *const u8 as *const libc::c_char,
                712 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 43],
                    &[libc::c_char; 43],
                >(b"_Bool pr_function_type(void *, int, _Bool)\0"))
                    .as_ptr(),
            );
        }
    };
    len = 10 as libc::c_int as libc::c_uint;
    if argcount <= 0 as libc::c_int {
        arg_types = 0 as *mut *mut libc::c_char;
        len = len.wrapping_add(15 as libc::c_int as libc::c_uint);
    } else {
        let mut i: libc::c_int = 0;
        arg_types = xmalloc(
            (argcount as libc::c_ulong)
                .wrapping_mul(
                    ::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                ),
        ) as *mut *mut libc::c_char;
        i = argcount - 1 as libc::c_int;
        while i >= 0 as libc::c_int {
            if !substitute_type(info, b"\0" as *const u8 as *const libc::c_char) {
                free(arg_types as *mut libc::c_void);
                return 0 as libc::c_int != 0;
            }
            let ref mut fresh0 = *arg_types.offset(i as isize);
            *fresh0 = pop_type(info);
            if (*arg_types.offset(i as isize)).is_null() {
                free(arg_types as *mut libc::c_void);
                return 0 as libc::c_int != 0;
            }
            len = (len as libc::c_ulong)
                .wrapping_add(
                    (strlen(*arg_types.offset(i as isize)))
                        .wrapping_add(2 as libc::c_int as libc::c_ulong),
                ) as libc::c_uint as libc::c_uint;
            i -= 1;
            i;
        }
        if varargs {
            len = len.wrapping_add(5 as libc::c_int as libc::c_uint);
        }
    }
    s = xmalloc(len as size_t) as *mut libc::c_char;
    strcpy(s, b"(|) (\0" as *const u8 as *const libc::c_char);
    if argcount < 0 as libc::c_int {
        strcat(s, b"/* unknown */\0" as *const u8 as *const libc::c_char);
    } else {
        let mut i_0: libc::c_int = 0;
        i_0 = 0 as libc::c_int;
        while i_0 < argcount {
            if i_0 > 0 as libc::c_int {
                strcat(s, b", \0" as *const u8 as *const libc::c_char);
            }
            strcat(s, *arg_types.offset(i_0 as isize));
            i_0 += 1;
            i_0;
        }
        if varargs {
            if i_0 > 0 as libc::c_int {
                strcat(s, b", \0" as *const u8 as *const libc::c_char);
            }
            strcat(s, b"...\0" as *const u8 as *const libc::c_char);
        }
        if argcount > 0 as libc::c_int {
            free(arg_types as *mut libc::c_void);
        }
    }
    strcat(s, b")\0" as *const u8 as *const libc::c_char);
    if !substitute_type(info, s) {
        return 0 as libc::c_int != 0;
    }
    free(s as *mut libc::c_void);
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn pr_reference_type(mut p: *mut libc::c_void) -> bool {
    let mut info: *mut pr_handle = p as *mut pr_handle;
    if !((*info).stack).is_null() {} else {
        __assert_fail(
            b"info->stack != NULL\0" as *const u8 as *const libc::c_char,
            b"prdbg.c\0" as *const u8 as *const libc::c_char,
            789 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"_Bool pr_reference_type(void *)\0"))
                .as_ptr(),
        );
    }
    'c_11513: {
        if !((*info).stack).is_null() {} else {
            __assert_fail(
                b"info->stack != NULL\0" as *const u8 as *const libc::c_char,
                b"prdbg.c\0" as *const u8 as *const libc::c_char,
                789 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 32],
                    &[libc::c_char; 32],
                >(b"_Bool pr_reference_type(void *)\0"))
                    .as_ptr(),
            );
        }
    };
    return substitute_type(info, b"&|\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn pr_range_type(
    mut p: *mut libc::c_void,
    mut lower: bfd_signed_vma,
    mut upper: bfd_signed_vma,
) -> bool {
    let mut info: *mut pr_handle = p as *mut pr_handle;
    let mut abl: [libc::c_char; 22] = [0; 22];
    let mut abu: [libc::c_char; 22] = [0; 22];
    if !((*info).stack).is_null() {} else {
        __assert_fail(
            b"info->stack != NULL\0" as *const u8 as *const libc::c_char,
            b"prdbg.c\0" as *const u8 as *const libc::c_char,
            802 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 60],
                &[libc::c_char; 60],
            >(b"_Bool pr_range_type(void *, bfd_signed_vma, bfd_signed_vma)\0"))
                .as_ptr(),
        );
    }
    'c_15989: {
        if !((*info).stack).is_null() {} else {
            __assert_fail(
                b"info->stack != NULL\0" as *const u8 as *const libc::c_char,
                b"prdbg.c\0" as *const u8 as *const libc::c_char,
                802 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 60],
                    &[libc::c_char; 60],
                >(b"_Bool pr_range_type(void *, bfd_signed_vma, bfd_signed_vma)\0"))
                    .as_ptr(),
            );
        }
    };
    if !substitute_type(info, b"\0" as *const u8 as *const libc::c_char) {
        return 0 as libc::c_int != 0;
    }
    print_vma(
        lower as bfd_vma,
        abl.as_mut_ptr(),
        0 as libc::c_int != 0,
        0 as libc::c_int != 0,
    );
    print_vma(
        upper as bfd_vma,
        abu.as_mut_ptr(),
        0 as libc::c_int != 0,
        0 as libc::c_int != 0,
    );
    return prepend_type(info, b"range (\0" as *const u8 as *const libc::c_char)
        as libc::c_int != 0
        && append_type(info, b"):\0" as *const u8 as *const libc::c_char) as libc::c_int
            != 0 && append_type(info, abl.as_mut_ptr()) as libc::c_int != 0
        && append_type(info, b":\0" as *const u8 as *const libc::c_char) as libc::c_int
            != 0 && append_type(info, abu.as_mut_ptr()) as libc::c_int != 0;
}
unsafe extern "C" fn pr_array_type(
    mut p: *mut libc::c_void,
    mut lower: bfd_signed_vma,
    mut upper: bfd_signed_vma,
    mut stringp: bool,
) -> bool {
    let mut info: *mut pr_handle = p as *mut pr_handle;
    let mut range_type: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut abl: [libc::c_char; 22] = [0; 22];
    let mut abu: [libc::c_char; 22] = [0; 22];
    let mut ab: [libc::c_char; 50] = [0; 50];
    range_type = pop_type(info);
    if range_type.is_null() {
        return 0 as libc::c_int != 0;
    }
    if lower == 0 as libc::c_int as libc::c_long {
        if upper == -(1 as libc::c_int) as libc::c_long {
            sprintf(ab.as_mut_ptr(), b"|[]\0" as *const u8 as *const libc::c_char);
        } else {
            print_vma(
                (upper + 1 as libc::c_int as libc::c_long) as bfd_vma,
                abu.as_mut_ptr(),
                0 as libc::c_int != 0,
                0 as libc::c_int != 0,
            );
            sprintf(
                ab.as_mut_ptr(),
                b"|[%s]\0" as *const u8 as *const libc::c_char,
                abu.as_mut_ptr(),
            );
        }
    } else {
        print_vma(
            lower as bfd_vma,
            abl.as_mut_ptr(),
            0 as libc::c_int != 0,
            0 as libc::c_int != 0,
        );
        print_vma(
            upper as bfd_vma,
            abu.as_mut_ptr(),
            0 as libc::c_int != 0,
            0 as libc::c_int != 0,
        );
        sprintf(
            ab.as_mut_ptr(),
            b"|[%s:%s]\0" as *const u8 as *const libc::c_char,
            abl.as_mut_ptr(),
            abu.as_mut_ptr(),
        );
    }
    if !substitute_type(info, ab.as_mut_ptr()) {
        return 0 as libc::c_int != 0;
    }
    if strcmp(range_type, b"int\0" as *const u8 as *const libc::c_char)
        != 0 as libc::c_int
    {
        if !append_type(info, b":\0" as *const u8 as *const libc::c_char)
            || !append_type(info, range_type)
        {
            return 0 as libc::c_int != 0;
        }
    }
    if stringp {
        if !append_type(info, b" /* string */\0" as *const u8 as *const libc::c_char) {
            return 0 as libc::c_int != 0;
        }
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn pr_set_type(
    mut p: *mut libc::c_void,
    mut bitstringp: bool,
) -> bool {
    let mut info: *mut pr_handle = p as *mut pr_handle;
    if !substitute_type(info, b"\0" as *const u8 as *const libc::c_char) {
        return 0 as libc::c_int != 0;
    }
    if !prepend_type(info, b"set { \0" as *const u8 as *const libc::c_char)
        || !append_type(info, b" }\0" as *const u8 as *const libc::c_char)
    {
        return 0 as libc::c_int != 0;
    }
    if bitstringp {
        if !append_type(info, b"/* bitstring */\0" as *const u8 as *const libc::c_char) {
            return 0 as libc::c_int != 0;
        }
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn pr_offset_type(mut p: *mut libc::c_void) -> bool {
    let mut info: *mut pr_handle = p as *mut pr_handle;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    if !substitute_type(info, b"\0" as *const u8 as *const libc::c_char) {
        return 0 as libc::c_int != 0;
    }
    t = pop_type(info);
    if t.is_null() {
        return 0 as libc::c_int != 0;
    }
    return substitute_type(info, b"\0" as *const u8 as *const libc::c_char)
        as libc::c_int != 0
        && prepend_type(info, b" \0" as *const u8 as *const libc::c_char) as libc::c_int
            != 0 && prepend_type(info, t) as libc::c_int != 0
        && append_type(info, b"::|\0" as *const u8 as *const libc::c_char) as libc::c_int
            != 0;
}
unsafe extern "C" fn pr_method_type(
    mut p: *mut libc::c_void,
    mut domain: bool,
    mut argcount: libc::c_int,
    mut varargs: bool,
) -> bool {
    let mut info: *mut pr_handle = p as *mut pr_handle;
    let mut len: libc::c_uint = 0;
    let mut domain_type: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut arg_types: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    len = 10 as libc::c_int as libc::c_uint;
    if !domain {
        domain_type = 0 as *mut libc::c_char;
    } else {
        if !substitute_type(info, b"\0" as *const u8 as *const libc::c_char) {
            return 0 as libc::c_int != 0;
        }
        domain_type = pop_type(info);
        if domain_type.is_null() {
            return 0 as libc::c_int != 0;
        }
        if startswith(domain_type, b"class \0" as *const u8 as *const libc::c_char)
            as libc::c_int != 0
            && (strchr(
                domain_type
                    .offset(
                        ::core::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong
                            as isize,
                    )
                    .offset(-(1 as libc::c_int as isize)),
                ' ' as i32,
            ))
                .is_null()
        {
            domain_type = domain_type
                .offset(
                    (::core::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                );
        } else if startswith(
            domain_type,
            b"union class \0" as *const u8 as *const libc::c_char,
        ) as libc::c_int != 0
            && (strchr(
                domain_type
                    .offset(
                        ::core::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong
                            as isize,
                    )
                    .offset(-(1 as libc::c_int as isize)),
                ' ' as i32,
            ))
                .is_null()
        {
            domain_type = domain_type
                .offset(
                    (::core::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                );
        }
        len = (len as libc::c_ulong).wrapping_add(strlen(domain_type)) as libc::c_uint
            as libc::c_uint;
    }
    if argcount <= 0 as libc::c_int {
        arg_types = 0 as *mut *mut libc::c_char;
        len = len.wrapping_add(15 as libc::c_int as libc::c_uint);
    } else {
        let mut i: libc::c_int = 0;
        arg_types = xmalloc(
            (argcount as libc::c_ulong)
                .wrapping_mul(
                    ::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                ),
        ) as *mut *mut libc::c_char;
        i = argcount - 1 as libc::c_int;
        while i >= 0 as libc::c_int {
            if !substitute_type(info, b"\0" as *const u8 as *const libc::c_char) {
                free(arg_types as *mut libc::c_void);
                return 0 as libc::c_int != 0;
            }
            let ref mut fresh1 = *arg_types.offset(i as isize);
            *fresh1 = pop_type(info);
            if (*arg_types.offset(i as isize)).is_null() {
                free(arg_types as *mut libc::c_void);
                return 0 as libc::c_int != 0;
            }
            len = (len as libc::c_ulong)
                .wrapping_add(
                    (strlen(*arg_types.offset(i as isize)))
                        .wrapping_add(2 as libc::c_int as libc::c_ulong),
                ) as libc::c_uint as libc::c_uint;
            i -= 1;
            i;
        }
        if varargs {
            len = len.wrapping_add(5 as libc::c_int as libc::c_uint);
        }
    }
    s = xmalloc(len as size_t) as *mut libc::c_char;
    if !domain {
        *s = '\0' as i32 as libc::c_char;
    } else {
        strcpy(s, domain_type);
    }
    strcat(s, b"::| (\0" as *const u8 as *const libc::c_char);
    if argcount < 0 as libc::c_int {
        strcat(s, b"/* unknown */\0" as *const u8 as *const libc::c_char);
    } else {
        let mut i_0: libc::c_int = 0;
        i_0 = 0 as libc::c_int;
        while i_0 < argcount {
            if i_0 > 0 as libc::c_int {
                strcat(s, b", \0" as *const u8 as *const libc::c_char);
            }
            strcat(s, *arg_types.offset(i_0 as isize));
            i_0 += 1;
            i_0;
        }
        if varargs {
            if i_0 > 0 as libc::c_int {
                strcat(s, b", \0" as *const u8 as *const libc::c_char);
            }
            strcat(s, b"...\0" as *const u8 as *const libc::c_char);
        }
        if argcount > 0 as libc::c_int {
            free(arg_types as *mut libc::c_void);
        }
    }
    strcat(s, b")\0" as *const u8 as *const libc::c_char);
    if !substitute_type(info, s) {
        return 0 as libc::c_int != 0;
    }
    free(s as *mut libc::c_void);
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn pr_const_type(mut p: *mut libc::c_void) -> bool {
    let mut info: *mut pr_handle = p as *mut pr_handle;
    return substitute_type(info, b"const |\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn pr_volatile_type(mut p: *mut libc::c_void) -> bool {
    let mut info: *mut pr_handle = p as *mut pr_handle;
    return substitute_type(info, b"volatile |\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn pr_start_struct_type(
    mut p: *mut libc::c_void,
    mut tag: *const libc::c_char,
    mut id: libc::c_uint,
    mut structp: bool,
    mut size: libc::c_uint,
) -> bool {
    let mut info: *mut pr_handle = p as *mut pr_handle;
    (*info).indent = ((*info).indent).wrapping_add(2 as libc::c_int as libc::c_uint);
    if !push_type(
        info,
        if structp as libc::c_int != 0 {
            b"struct \0" as *const u8 as *const libc::c_char
        } else {
            b"union \0" as *const u8 as *const libc::c_char
        },
    ) {
        return 0 as libc::c_int != 0;
    }
    if !tag.is_null() {
        if !append_type(info, tag) {
            return 0 as libc::c_int != 0;
        }
    } else {
        let mut idbuf: [libc::c_char; 20] = [0; 20];
        sprintf(
            idbuf.as_mut_ptr(),
            b"%%anon%u\0" as *const u8 as *const libc::c_char,
            id,
        );
        if !append_type(info, idbuf.as_mut_ptr()) {
            return 0 as libc::c_int != 0;
        }
    }
    if !append_type(info, b" {\0" as *const u8 as *const libc::c_char) {
        return 0 as libc::c_int != 0;
    }
    if size != 0 as libc::c_int as libc::c_uint || !tag.is_null() {
        let mut ab: [libc::c_char; 30] = [0; 30];
        if !append_type(info, b" /*\0" as *const u8 as *const libc::c_char) {
            return 0 as libc::c_int != 0;
        }
        if size != 0 as libc::c_int as libc::c_uint {
            sprintf(
                ab.as_mut_ptr(),
                b" size %u\0" as *const u8 as *const libc::c_char,
                size,
            );
            if !append_type(info, ab.as_mut_ptr()) {
                return 0 as libc::c_int != 0;
            }
        }
        if !tag.is_null() {
            sprintf(
                ab.as_mut_ptr(),
                b" id %u\0" as *const u8 as *const libc::c_char,
                id,
            );
            if !append_type(info, ab.as_mut_ptr()) {
                return 0 as libc::c_int != 0;
            }
        }
        if !append_type(info, b" */\0" as *const u8 as *const libc::c_char) {
            return 0 as libc::c_int != 0;
        }
    }
    if !append_type(info, b"\n\0" as *const u8 as *const libc::c_char) {
        return 0 as libc::c_int != 0;
    }
    (*(*info).stack).visibility = DEBUG_VISIBILITY_PUBLIC;
    return indent_type(info);
}
unsafe extern "C" fn pr_struct_field(
    mut p: *mut libc::c_void,
    mut name: *const libc::c_char,
    mut bitpos: bfd_vma,
    mut bitsize: bfd_vma,
    mut visibility: debug_visibility,
) -> bool {
    let mut info: *mut pr_handle = p as *mut pr_handle;
    let mut ab: [libc::c_char; 22] = [0; 22];
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    if !substitute_type(info, name) {
        return 0 as libc::c_int != 0;
    }
    if !append_type(info, b"; /* \0" as *const u8 as *const libc::c_char) {
        return 0 as libc::c_int != 0;
    }
    if bitsize != 0 as libc::c_int as libc::c_ulong {
        print_vma(
            bitsize,
            ab.as_mut_ptr(),
            1 as libc::c_int != 0,
            0 as libc::c_int != 0,
        );
        if !append_type(info, b"bitsize \0" as *const u8 as *const libc::c_char)
            || !append_type(info, ab.as_mut_ptr())
            || !append_type(info, b", \0" as *const u8 as *const libc::c_char)
        {
            return 0 as libc::c_int != 0;
        }
    }
    print_vma(bitpos, ab.as_mut_ptr(), 1 as libc::c_int != 0, 0 as libc::c_int != 0);
    if !append_type(info, b"bitpos \0" as *const u8 as *const libc::c_char)
        || !append_type(info, ab.as_mut_ptr())
        || !append_type(info, b" */\n\0" as *const u8 as *const libc::c_char)
        || !indent_type(info)
    {
        return 0 as libc::c_int != 0;
    }
    t = pop_type(info);
    if t.is_null() {
        return 0 as libc::c_int != 0;
    }
    if !pr_fix_visibility(info, visibility) {
        return 0 as libc::c_int != 0;
    }
    return append_type(info, t);
}
unsafe extern "C" fn pr_end_struct_type(mut p: *mut libc::c_void) -> bool {
    let mut info: *mut pr_handle = p as *mut pr_handle;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    if !((*info).stack).is_null() {} else {
        __assert_fail(
            b"info->stack != NULL\0" as *const u8 as *const libc::c_char,
            b"prdbg.c\0" as *const u8 as *const libc::c_char,
            1192 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"_Bool pr_end_struct_type(void *)\0"))
                .as_ptr(),
        );
    }
    'c_12558: {
        if !((*info).stack).is_null() {} else {
            __assert_fail(
                b"info->stack != NULL\0" as *const u8 as *const libc::c_char,
                b"prdbg.c\0" as *const u8 as *const libc::c_char,
                1192 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 33],
                    &[libc::c_char; 33],
                >(b"_Bool pr_end_struct_type(void *)\0"))
                    .as_ptr(),
            );
        }
    };
    if (*info).indent >= 2 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"info->indent >= 2\0" as *const u8 as *const libc::c_char,
            b"prdbg.c\0" as *const u8 as *const libc::c_char,
            1193 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"_Bool pr_end_struct_type(void *)\0"))
                .as_ptr(),
        );
    }
    'c_12515: {
        if (*info).indent >= 2 as libc::c_int as libc::c_uint {} else {
            __assert_fail(
                b"info->indent >= 2\0" as *const u8 as *const libc::c_char,
                b"prdbg.c\0" as *const u8 as *const libc::c_char,
                1193 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 33],
                    &[libc::c_char; 33],
                >(b"_Bool pr_end_struct_type(void *)\0"))
                    .as_ptr(),
            );
        }
    };
    (*info).indent = ((*info).indent).wrapping_sub(2 as libc::c_int as libc::c_uint);
    s = ((*(*info).stack).type_0)
        .offset(strlen((*(*info).stack).type_0) as isize)
        .offset(-(2 as libc::c_int as isize));
    if *s.offset(0 as libc::c_int as isize) as libc::c_int == ' ' as i32
        && *s.offset(1 as libc::c_int as isize) as libc::c_int == ' ' as i32
        && *s.offset(2 as libc::c_int as isize) as libc::c_int == '\0' as i32
    {} else {
        __assert_fail(
            b"s[0] == ' ' && s[1] == ' ' && s[2] == '\\0'\0" as *const u8
                as *const libc::c_char,
            b"prdbg.c\0" as *const u8 as *const libc::c_char,
            1199 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"_Bool pr_end_struct_type(void *)\0"))
                .as_ptr(),
        );
    }
    'c_12402: {
        if *s.offset(0 as libc::c_int as isize) as libc::c_int == ' ' as i32
            && *s.offset(1 as libc::c_int as isize) as libc::c_int == ' ' as i32
            && *s.offset(2 as libc::c_int as isize) as libc::c_int == '\0' as i32
        {} else {
            __assert_fail(
                b"s[0] == ' ' && s[1] == ' ' && s[2] == '\\0'\0" as *const u8
                    as *const libc::c_char,
                b"prdbg.c\0" as *const u8 as *const libc::c_char,
                1199 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 33],
                    &[libc::c_char; 33],
                >(b"_Bool pr_end_struct_type(void *)\0"))
                    .as_ptr(),
            );
        }
    };
    let fresh2 = s;
    s = s.offset(1);
    *fresh2 = '}' as i32 as libc::c_char;
    *s = '\0' as i32 as libc::c_char;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn pr_start_class_type(
    mut p: *mut libc::c_void,
    mut tag: *const libc::c_char,
    mut id: libc::c_uint,
    mut structp: bool,
    mut size: libc::c_uint,
    mut vptr: bool,
    mut ownvptr: bool,
) -> bool {
    let mut info: *mut pr_handle = p as *mut pr_handle;
    let mut tv: *mut libc::c_char = 0 as *mut libc::c_char;
    (*info).indent = ((*info).indent).wrapping_add(2 as libc::c_int as libc::c_uint);
    if vptr as libc::c_int != 0 && !ownvptr {
        tv = pop_type(info);
        if tv.is_null() {
            return 0 as libc::c_int != 0;
        }
    }
    if !push_type(
        info,
        if structp as libc::c_int != 0 {
            b"class \0" as *const u8 as *const libc::c_char
        } else {
            b"union class \0" as *const u8 as *const libc::c_char
        },
    ) {
        return 0 as libc::c_int != 0;
    }
    if !tag.is_null() {
        if !append_type(info, tag) {
            return 0 as libc::c_int != 0;
        }
    } else {
        let mut idbuf: [libc::c_char; 20] = [0; 20];
        sprintf(
            idbuf.as_mut_ptr(),
            b"%%anon%u\0" as *const u8 as *const libc::c_char,
            id,
        );
        if !append_type(info, idbuf.as_mut_ptr()) {
            return 0 as libc::c_int != 0;
        }
    }
    if !append_type(info, b" {\0" as *const u8 as *const libc::c_char) {
        return 0 as libc::c_int != 0;
    }
    if size != 0 as libc::c_int as libc::c_uint || vptr as libc::c_int != 0
        || ownvptr as libc::c_int != 0 || !tag.is_null()
    {
        if !append_type(info, b" /*\0" as *const u8 as *const libc::c_char) {
            return 0 as libc::c_int != 0;
        }
        if size != 0 as libc::c_int as libc::c_uint {
            let mut ab: [libc::c_char; 20] = [0; 20];
            sprintf(ab.as_mut_ptr(), b"%u\0" as *const u8 as *const libc::c_char, size);
            if !append_type(info, b" size \0" as *const u8 as *const libc::c_char)
                || !append_type(info, ab.as_mut_ptr())
            {
                return 0 as libc::c_int != 0;
            }
        }
        if vptr {
            if !append_type(info, b" vtable \0" as *const u8 as *const libc::c_char) {
                return 0 as libc::c_int != 0;
            }
            if ownvptr {
                if !append_type(info, b"self \0" as *const u8 as *const libc::c_char) {
                    return 0 as libc::c_int != 0;
                }
            } else if !append_type(info, tv)
                || !append_type(info, b" \0" as *const u8 as *const libc::c_char)
            {
                return 0 as libc::c_int != 0
            }
        }
        if !tag.is_null() {
            let mut ab_0: [libc::c_char; 30] = [0; 30];
            sprintf(
                ab_0.as_mut_ptr(),
                b" id %u\0" as *const u8 as *const libc::c_char,
                id,
            );
            if !append_type(info, ab_0.as_mut_ptr()) {
                return 0 as libc::c_int != 0;
            }
        }
        if !append_type(info, b" */\0" as *const u8 as *const libc::c_char) {
            return 0 as libc::c_int != 0;
        }
    }
    (*(*info).stack).visibility = DEBUG_VISIBILITY_PRIVATE;
    return append_type(info, b"\n\0" as *const u8 as *const libc::c_char) as libc::c_int
        != 0 && indent_type(info) as libc::c_int != 0;
}
unsafe extern "C" fn pr_class_static_member(
    mut p: *mut libc::c_void,
    mut name: *const libc::c_char,
    mut physname: *const libc::c_char,
    mut visibility: debug_visibility,
) -> bool {
    let mut info: *mut pr_handle = p as *mut pr_handle;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    if !substitute_type(info, name) {
        return 0 as libc::c_int != 0;
    }
    if !prepend_type(info, b"static \0" as *const u8 as *const libc::c_char)
        || !append_type(info, b"; /* \0" as *const u8 as *const libc::c_char)
        || !append_type(info, physname)
        || !append_type(info, b" */\n\0" as *const u8 as *const libc::c_char)
        || !indent_type(info)
    {
        return 0 as libc::c_int != 0;
    }
    t = pop_type(info);
    if t.is_null() {
        return 0 as libc::c_int != 0;
    }
    if !pr_fix_visibility(info, visibility) {
        return 0 as libc::c_int != 0;
    }
    return append_type(info, t);
}
unsafe extern "C" fn pr_class_baseclass(
    mut p: *mut libc::c_void,
    mut bitpos: bfd_vma,
    mut is_virtual: bool,
    mut visibility: debug_visibility,
) -> bool {
    let mut info: *mut pr_handle = p as *mut pr_handle;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut prefix: *const libc::c_char = 0 as *const libc::c_char;
    let mut ab: [libc::c_char; 22] = [0; 22];
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut l: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n: *mut libc::c_char = 0 as *mut libc::c_char;
    if !((*info).stack).is_null() && !((*(*info).stack).next).is_null() {} else {
        __assert_fail(
            b"info->stack != NULL && info->stack->next != NULL\0" as *const u8
                as *const libc::c_char,
            b"prdbg.c\0" as *const u8 as *const libc::c_char,
            1336 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 72],
                &[libc::c_char; 72],
            >(
                b"_Bool pr_class_baseclass(void *, bfd_vma, _Bool, enum debug_visibility)\0",
            ))
                .as_ptr(),
        );
    }
    'c_14137: {
        if !((*info).stack).is_null() && !((*(*info).stack).next).is_null() {} else {
            __assert_fail(
                b"info->stack != NULL && info->stack->next != NULL\0" as *const u8
                    as *const libc::c_char,
                b"prdbg.c\0" as *const u8 as *const libc::c_char,
                1336 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 72],
                    &[libc::c_char; 72],
                >(
                    b"_Bool pr_class_baseclass(void *, bfd_vma, _Bool, enum debug_visibility)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if !substitute_type(info, b"\0" as *const u8 as *const libc::c_char) {
        return 0 as libc::c_int != 0;
    }
    t = pop_type(info);
    if t.is_null() {
        return 0 as libc::c_int != 0;
    }
    if startswith(t, b"class \0" as *const u8 as *const libc::c_char) {
        t = t
            .offset(
                (::core::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
            );
    }
    if !push_type(info, t) {
        return 0 as libc::c_int != 0;
    }
    if is_virtual {
        if !prepend_type(info, b"virtual \0" as *const u8 as *const libc::c_char) {
            return 0 as libc::c_int != 0;
        }
    }
    match visibility as libc::c_uint {
        0 => {
            prefix = b"public \0" as *const u8 as *const libc::c_char;
        }
        1 => {
            prefix = b"protected \0" as *const u8 as *const libc::c_char;
        }
        2 => {
            prefix = b"private \0" as *const u8 as *const libc::c_char;
        }
        _ => {
            prefix = b"/* unknown visibility */ \0" as *const u8 as *const libc::c_char;
        }
    }
    if !prepend_type(info, prefix) {
        return 0 as libc::c_int != 0;
    }
    if bitpos != 0 as libc::c_int as libc::c_ulong {
        print_vma(bitpos, ab.as_mut_ptr(), 1 as libc::c_int != 0, 0 as libc::c_int != 0);
        if !append_type(info, b" /* bitpos \0" as *const u8 as *const libc::c_char)
            || !append_type(info, ab.as_mut_ptr())
            || !append_type(info, b" */\0" as *const u8 as *const libc::c_char)
        {
            return 0 as libc::c_int != 0;
        }
    }
    s = strchr((*(*(*info).stack).next).type_0, '{' as i32);
    if !s.is_null() {} else {
        __assert_fail(
            b"s != NULL\0" as *const u8 as *const libc::c_char,
            b"prdbg.c\0" as *const u8 as *const libc::c_char,
            1392 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 72],
                &[libc::c_char; 72],
            >(
                b"_Bool pr_class_baseclass(void *, bfd_vma, _Bool, enum debug_visibility)\0",
            ))
                .as_ptr(),
        );
    }
    'c_13902: {
        if !s.is_null() {} else {
            __assert_fail(
                b"s != NULL\0" as *const u8 as *const libc::c_char,
                b"prdbg.c\0" as *const u8 as *const libc::c_char,
                1392 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 72],
                    &[libc::c_char; 72],
                >(
                    b"_Bool pr_class_baseclass(void *, bfd_vma, _Bool, enum debug_visibility)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    s = s.offset(-1);
    s;
    l = (*(*(*info).stack).next).type_0;
    while l != s {
        if *l as libc::c_int == ':' as i32 {
            break;
        }
        l = l.offset(1);
        l;
    }
    if !prepend_type(
        info,
        if l == s {
            b" : \0" as *const u8 as *const libc::c_char
        } else {
            b", \0" as *const u8 as *const libc::c_char
        },
    ) {
        return 0 as libc::c_int != 0;
    }
    t = pop_type(info);
    if t.is_null() {
        return 0 as libc::c_int != 0;
    }
    n = xmalloc(
        (strlen((*(*info).stack).type_0))
            .wrapping_add(strlen(t))
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    memcpy(
        n as *mut libc::c_void,
        (*(*info).stack).type_0 as *const libc::c_void,
        s.offset_from((*(*info).stack).type_0) as libc::c_long as libc::c_ulong,
    );
    strcpy(n.offset(s.offset_from((*(*info).stack).type_0) as libc::c_long as isize), t);
    strcat(n, s);
    free((*(*info).stack).type_0 as *mut libc::c_void);
    (*(*info).stack).type_0 = n;
    free(t as *mut libc::c_void);
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn pr_class_start_method(
    mut p: *mut libc::c_void,
    mut name: *const libc::c_char,
) -> bool {
    let mut info: *mut pr_handle = p as *mut pr_handle;
    if !((*info).stack).is_null() {} else {
        __assert_fail(
            b"info->stack != NULL\0" as *const u8 as *const libc::c_char,
            b"prdbg.c\0" as *const u8 as *const libc::c_char,
            1427 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 50],
                &[libc::c_char; 50],
            >(b"_Bool pr_class_start_method(void *, const char *)\0"))
                .as_ptr(),
        );
    }
    'c_13642: {
        if !((*info).stack).is_null() {} else {
            __assert_fail(
                b"info->stack != NULL\0" as *const u8 as *const libc::c_char,
                b"prdbg.c\0" as *const u8 as *const libc::c_char,
                1427 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 50],
                    &[libc::c_char; 50],
                >(b"_Bool pr_class_start_method(void *, const char *)\0"))
                    .as_ptr(),
            );
        }
    };
    (*(*info).stack).method = name;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn pr_class_method_variant(
    mut p: *mut libc::c_void,
    mut physname: *const libc::c_char,
    mut visibility: debug_visibility,
    mut constp: bool,
    mut volatilep: bool,
    mut voffset: bfd_vma,
    mut context: bool,
) -> bool {
    let mut info: *mut pr_handle = p as *mut pr_handle;
    let mut method_type: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut context_type: *mut libc::c_char = 0 as *mut libc::c_char;
    if !((*info).stack).is_null() {} else {
        __assert_fail(
            b"info->stack != NULL\0" as *const u8 as *const libc::c_char,
            b"prdbg.c\0" as *const u8 as *const libc::c_char,
            1444 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 105],
                &[libc::c_char; 105],
            >(
                b"_Bool pr_class_method_variant(void *, const char *, enum debug_visibility, _Bool, _Bool, bfd_vma, _Bool)\0",
            ))
                .as_ptr(),
        );
    }
    'c_13573: {
        if !((*info).stack).is_null() {} else {
            __assert_fail(
                b"info->stack != NULL\0" as *const u8 as *const libc::c_char,
                b"prdbg.c\0" as *const u8 as *const libc::c_char,
                1444 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 105],
                    &[libc::c_char; 105],
                >(
                    b"_Bool pr_class_method_variant(void *, const char *, enum debug_visibility, _Bool, _Bool, bfd_vma, _Bool)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if !((*(*info).stack).next).is_null() {} else {
        __assert_fail(
            b"info->stack->next != NULL\0" as *const u8 as *const libc::c_char,
            b"prdbg.c\0" as *const u8 as *const libc::c_char,
            1445 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 105],
                &[libc::c_char; 105],
            >(
                b"_Bool pr_class_method_variant(void *, const char *, enum debug_visibility, _Bool, _Bool, bfd_vma, _Bool)\0",
            ))
                .as_ptr(),
        );
    }
    'c_13522: {
        if !((*(*info).stack).next).is_null() {} else {
            __assert_fail(
                b"info->stack->next != NULL\0" as *const u8 as *const libc::c_char,
                b"prdbg.c\0" as *const u8 as *const libc::c_char,
                1445 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 105],
                    &[libc::c_char; 105],
                >(
                    b"_Bool pr_class_method_variant(void *, const char *, enum debug_visibility, _Bool, _Bool, bfd_vma, _Bool)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if volatilep {
        if !append_type(info, b" volatile\0" as *const u8 as *const libc::c_char) {
            return 0 as libc::c_int != 0;
        }
    }
    if constp {
        if !append_type(info, b" const\0" as *const u8 as *const libc::c_char) {
            return 0 as libc::c_int != 0;
        }
    }
    if !substitute_type(
        info,
        if context as libc::c_int != 0 {
            (*(*(*(*info).stack).next).next).method
        } else {
            (*(*(*info).stack).next).method
        },
    ) {
        return 0 as libc::c_int != 0;
    }
    method_type = pop_type(info);
    if method_type.is_null() {
        return 0 as libc::c_int != 0;
    }
    if !context {
        context_type = 0 as *mut libc::c_char;
    } else {
        context_type = pop_type(info);
        if context_type.is_null() {
            return 0 as libc::c_int != 0;
        }
    }
    if !pr_fix_visibility(info, visibility) {
        return 0 as libc::c_int != 0;
    }
    if !append_type(info, method_type)
        || !append_type(info, b" /* \0" as *const u8 as *const libc::c_char)
        || !append_type(info, physname)
        || !append_type(info, b" \0" as *const u8 as *const libc::c_char)
    {
        return 0 as libc::c_int != 0;
    }
    if context as libc::c_int != 0 || voffset != 0 as libc::c_int as libc::c_ulong {
        let mut ab: [libc::c_char; 22] = [0; 22];
        if context {
            if !append_type(info, b"context \0" as *const u8 as *const libc::c_char)
                || !append_type(info, context_type)
                || !append_type(info, b" \0" as *const u8 as *const libc::c_char)
            {
                return 0 as libc::c_int != 0;
            }
        }
        print_vma(
            voffset,
            ab.as_mut_ptr(),
            1 as libc::c_int != 0,
            0 as libc::c_int != 0,
        );
        if !append_type(info, b"voffset \0" as *const u8 as *const libc::c_char)
            || !append_type(info, ab.as_mut_ptr())
        {
            return 0 as libc::c_int != 0;
        }
    }
    return append_type(info, b" */;\n\0" as *const u8 as *const libc::c_char)
        as libc::c_int != 0 && indent_type(info) as libc::c_int != 0;
}
unsafe extern "C" fn pr_class_static_method_variant(
    mut p: *mut libc::c_void,
    mut physname: *const libc::c_char,
    mut visibility: debug_visibility,
    mut constp: bool,
    mut volatilep: bool,
) -> bool {
    let mut info: *mut pr_handle = p as *mut pr_handle;
    let mut method_type: *mut libc::c_char = 0 as *mut libc::c_char;
    if !((*info).stack).is_null() {} else {
        __assert_fail(
            b"info->stack != NULL\0" as *const u8 as *const libc::c_char,
            b"prdbg.c\0" as *const u8 as *const libc::c_char,
            1522 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 96],
                &[libc::c_char; 96],
            >(
                b"_Bool pr_class_static_method_variant(void *, const char *, enum debug_visibility, _Bool, _Bool)\0",
            ))
                .as_ptr(),
        );
    }
    'c_13187: {
        if !((*info).stack).is_null() {} else {
            __assert_fail(
                b"info->stack != NULL\0" as *const u8 as *const libc::c_char,
                b"prdbg.c\0" as *const u8 as *const libc::c_char,
                1522 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 96],
                    &[libc::c_char; 96],
                >(
                    b"_Bool pr_class_static_method_variant(void *, const char *, enum debug_visibility, _Bool, _Bool)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if !((*(*info).stack).next).is_null() {} else {
        __assert_fail(
            b"info->stack->next != NULL\0" as *const u8 as *const libc::c_char,
            b"prdbg.c\0" as *const u8 as *const libc::c_char,
            1523 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 96],
                &[libc::c_char; 96],
            >(
                b"_Bool pr_class_static_method_variant(void *, const char *, enum debug_visibility, _Bool, _Bool)\0",
            ))
                .as_ptr(),
        );
    }
    'c_13136: {
        if !((*(*info).stack).next).is_null() {} else {
            __assert_fail(
                b"info->stack->next != NULL\0" as *const u8 as *const libc::c_char,
                b"prdbg.c\0" as *const u8 as *const libc::c_char,
                1523 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 96],
                    &[libc::c_char; 96],
                >(
                    b"_Bool pr_class_static_method_variant(void *, const char *, enum debug_visibility, _Bool, _Bool)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if !((*(*(*info).stack).next).method).is_null() {} else {
        __assert_fail(
            b"info->stack->next->method != NULL\0" as *const u8 as *const libc::c_char,
            b"prdbg.c\0" as *const u8 as *const libc::c_char,
            1524 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 96],
                &[libc::c_char; 96],
            >(
                b"_Bool pr_class_static_method_variant(void *, const char *, enum debug_visibility, _Bool, _Bool)\0",
            ))
                .as_ptr(),
        );
    }
    'c_13080: {
        if !((*(*(*info).stack).next).method).is_null() {} else {
            __assert_fail(
                b"info->stack->next->method != NULL\0" as *const u8
                    as *const libc::c_char,
                b"prdbg.c\0" as *const u8 as *const libc::c_char,
                1524 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 96],
                    &[libc::c_char; 96],
                >(
                    b"_Bool pr_class_static_method_variant(void *, const char *, enum debug_visibility, _Bool, _Bool)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if volatilep {
        if !append_type(info, b" volatile\0" as *const u8 as *const libc::c_char) {
            return 0 as libc::c_int != 0;
        }
    }
    if constp {
        if !append_type(info, b" const\0" as *const u8 as *const libc::c_char) {
            return 0 as libc::c_int != 0;
        }
    }
    if !prepend_type(info, b"static \0" as *const u8 as *const libc::c_char) {
        return 0 as libc::c_int != 0;
    }
    if !substitute_type(info, (*(*(*info).stack).next).method) {
        return 0 as libc::c_int != 0;
    }
    method_type = pop_type(info);
    if method_type.is_null() {
        return 0 as libc::c_int != 0;
    }
    if !pr_fix_visibility(info, visibility) {
        return 0 as libc::c_int != 0;
    }
    return append_type(info, method_type) as libc::c_int != 0
        && append_type(info, b" /* \0" as *const u8 as *const libc::c_char)
            as libc::c_int != 0 && append_type(info, physname) as libc::c_int != 0
        && append_type(info, b" */;\n\0" as *const u8 as *const libc::c_char)
            as libc::c_int != 0 && indent_type(info) as libc::c_int != 0;
}
unsafe extern "C" fn pr_class_end_method(mut p: *mut libc::c_void) -> bool {
    let mut info: *mut pr_handle = p as *mut pr_handle;
    (*(*info).stack).method = 0 as *const libc::c_char;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn pr_end_class_type(mut p: *mut libc::c_void) -> bool {
    return pr_end_struct_type(p);
}
unsafe extern "C" fn pr_typedef_type(
    mut p: *mut libc::c_void,
    mut name: *const libc::c_char,
) -> bool {
    let mut info: *mut pr_handle = p as *mut pr_handle;
    return push_type(info, name);
}
unsafe extern "C" fn pr_tag_type(
    mut p: *mut libc::c_void,
    mut name: *const libc::c_char,
    mut id: libc::c_uint,
    mut kind: debug_type_kind,
) -> bool {
    let mut info: *mut pr_handle = p as *mut pr_handle;
    let mut t: *const libc::c_char = 0 as *const libc::c_char;
    let mut tag: *const libc::c_char = 0 as *const libc::c_char;
    let mut idbuf: [libc::c_char; 22] = [0; 22];
    match kind as libc::c_uint {
        7 => {
            t = b"struct \0" as *const u8 as *const libc::c_char;
        }
        8 => {
            t = b"union \0" as *const u8 as *const libc::c_char;
        }
        11 => {
            t = b"enum \0" as *const u8 as *const libc::c_char;
        }
        9 => {
            t = b"class \0" as *const u8 as *const libc::c_char;
        }
        10 => {
            t = b"union class \0" as *const u8 as *const libc::c_char;
        }
        _ => return 0 as libc::c_int != 0,
    }
    if !push_type(info, t) {
        return 0 as libc::c_int != 0;
    }
    if !name.is_null() {
        tag = name;
    } else {
        sprintf(
            idbuf.as_mut_ptr(),
            b"%%anon%u\0" as *const u8 as *const libc::c_char,
            id,
        );
        tag = idbuf.as_mut_ptr();
    }
    if !append_type(info, tag) {
        return 0 as libc::c_int != 0;
    }
    if !name.is_null()
        && kind as libc::c_uint != DEBUG_KIND_ENUM as libc::c_int as libc::c_uint
    {
        sprintf(
            idbuf.as_mut_ptr(),
            b" /* id %u */\0" as *const u8 as *const libc::c_char,
            id,
        );
        if !append_type(info, idbuf.as_mut_ptr()) {
            return 0 as libc::c_int != 0;
        }
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn pr_typdef(
    mut p: *mut libc::c_void,
    mut name: *const libc::c_char,
) -> bool {
    let mut info: *mut pr_handle = p as *mut pr_handle;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    if !substitute_type(info, name) {
        return 0 as libc::c_int != 0;
    }
    s = pop_type(info);
    if s.is_null() {
        return 0 as libc::c_int != 0;
    }
    indent(info);
    fprintf((*info).f, b"typedef %s;\n\0" as *const u8 as *const libc::c_char, s);
    free(s as *mut libc::c_void);
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn pr_tag(
    mut p: *mut libc::c_void,
    mut name: *const libc::c_char,
) -> bool {
    let mut info: *mut pr_handle = p as *mut pr_handle;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    t = pop_type(info);
    if t.is_null() {
        return 0 as libc::c_int != 0;
    }
    indent(info);
    fprintf((*info).f, b"%s;\n\0" as *const u8 as *const libc::c_char, t);
    free(t as *mut libc::c_void);
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn pr_int_constant(
    mut p: *mut libc::c_void,
    mut name: *const libc::c_char,
    mut val: bfd_vma,
) -> bool {
    let mut info: *mut pr_handle = p as *mut pr_handle;
    let mut ab: [libc::c_char; 22] = [0; 22];
    indent(info);
    print_vma(val, ab.as_mut_ptr(), 0 as libc::c_int != 0, 0 as libc::c_int != 0);
    fprintf(
        (*info).f,
        b"const int %s = %s;\n\0" as *const u8 as *const libc::c_char,
        name,
        ab.as_mut_ptr(),
    );
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn pr_float_constant(
    mut p: *mut libc::c_void,
    mut name: *const libc::c_char,
    mut val: libc::c_double,
) -> bool {
    let mut info: *mut pr_handle = p as *mut pr_handle;
    indent(info);
    fprintf(
        (*info).f,
        b"const double %s = %g;\n\0" as *const u8 as *const libc::c_char,
        name,
        val,
    );
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn pr_typed_constant(
    mut p: *mut libc::c_void,
    mut name: *const libc::c_char,
    mut val: bfd_vma,
) -> bool {
    let mut info: *mut pr_handle = p as *mut pr_handle;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ab: [libc::c_char; 22] = [0; 22];
    t = pop_type(info);
    if t.is_null() {
        return 0 as libc::c_int != 0;
    }
    indent(info);
    print_vma(val, ab.as_mut_ptr(), 0 as libc::c_int != 0, 0 as libc::c_int != 0);
    fprintf(
        (*info).f,
        b"const %s %s = %s;\n\0" as *const u8 as *const libc::c_char,
        t,
        name,
        ab.as_mut_ptr(),
    );
    free(t as *mut libc::c_void);
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn pr_variable(
    mut p: *mut libc::c_void,
    mut name: *const libc::c_char,
    mut kind: debug_var_kind,
    mut val: bfd_vma,
) -> bool {
    let mut info: *mut pr_handle = p as *mut pr_handle;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ab: [libc::c_char; 22] = [0; 22];
    if !substitute_type(info, name) {
        return 0 as libc::c_int != 0;
    }
    t = pop_type(info);
    if t.is_null() {
        return 0 as libc::c_int != 0;
    }
    indent(info);
    match kind as libc::c_uint {
        2 | 3 => {
            fprintf((*info).f, b"static \0" as *const u8 as *const libc::c_char);
        }
        5 => {
            fprintf((*info).f, b"register \0" as *const u8 as *const libc::c_char);
        }
        _ => {}
    }
    print_vma(val, ab.as_mut_ptr(), 1 as libc::c_int != 0, 1 as libc::c_int != 0);
    fprintf(
        (*info).f,
        b"%s /* %s */;\n\0" as *const u8 as *const libc::c_char,
        t,
        ab.as_mut_ptr(),
    );
    free(t as *mut libc::c_void);
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn pr_start_function(
    mut p: *mut libc::c_void,
    mut name: *const libc::c_char,
    mut global: bool,
) -> bool {
    let mut info: *mut pr_handle = p as *mut pr_handle;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    if !substitute_type(info, name) {
        return 0 as libc::c_int != 0;
    }
    t = pop_type(info);
    if t.is_null() {
        return 0 as libc::c_int != 0;
    }
    indent(info);
    if !global {
        fprintf((*info).f, b"static \0" as *const u8 as *const libc::c_char);
    }
    fprintf((*info).f, b"%s (\0" as *const u8 as *const libc::c_char, t);
    (*info).parameter = 1 as libc::c_int;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn pr_function_parameter(
    mut p: *mut libc::c_void,
    mut name: *const libc::c_char,
    mut kind: debug_parm_kind,
    mut val: bfd_vma,
) -> bool {
    let mut info: *mut pr_handle = p as *mut pr_handle;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ab: [libc::c_char; 22] = [0; 22];
    if kind as libc::c_uint == DEBUG_PARM_REFERENCE as libc::c_int as libc::c_uint
        || kind as libc::c_uint == DEBUG_PARM_REF_REG as libc::c_int as libc::c_uint
    {
        if !pr_reference_type(p) {
            return 0 as libc::c_int != 0;
        }
    }
    if !substitute_type(info, name) {
        return 0 as libc::c_int != 0;
    }
    t = pop_type(info);
    if t.is_null() {
        return 0 as libc::c_int != 0;
    }
    if (*info).parameter != 1 as libc::c_int {
        fprintf((*info).f, b", \0" as *const u8 as *const libc::c_char);
    }
    if kind as libc::c_uint == DEBUG_PARM_REG as libc::c_int as libc::c_uint
        || kind as libc::c_uint == DEBUG_PARM_REF_REG as libc::c_int as libc::c_uint
    {
        fprintf((*info).f, b"register \0" as *const u8 as *const libc::c_char);
    }
    print_vma(val, ab.as_mut_ptr(), 1 as libc::c_int != 0, 1 as libc::c_int != 0);
    fprintf(
        (*info).f,
        b"%s /* %s */\0" as *const u8 as *const libc::c_char,
        t,
        ab.as_mut_ptr(),
    );
    free(t as *mut libc::c_void);
    (*info).parameter += 1;
    (*info).parameter;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn pr_start_block(
    mut p: *mut libc::c_void,
    mut addr: bfd_vma,
) -> bool {
    let mut info: *mut pr_handle = p as *mut pr_handle;
    let mut ab: [libc::c_char; 22] = [0; 22];
    if (*info).parameter > 0 as libc::c_int {
        fprintf((*info).f, b")\n\0" as *const u8 as *const libc::c_char);
        (*info).parameter = 0 as libc::c_int;
    }
    indent(info);
    print_vma(addr, ab.as_mut_ptr(), 1 as libc::c_int != 0, 1 as libc::c_int != 0);
    fprintf(
        (*info).f,
        b"{ /* %s */\n\0" as *const u8 as *const libc::c_char,
        ab.as_mut_ptr(),
    );
    (*info).indent = ((*info).indent).wrapping_add(2 as libc::c_int as libc::c_uint);
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn pr_end_block(mut p: *mut libc::c_void, mut addr: bfd_vma) -> bool {
    let mut info: *mut pr_handle = p as *mut pr_handle;
    let mut ab: [libc::c_char; 22] = [0; 22];
    (*info).indent = ((*info).indent).wrapping_sub(2 as libc::c_int as libc::c_uint);
    indent(info);
    print_vma(addr, ab.as_mut_ptr(), 1 as libc::c_int != 0, 1 as libc::c_int != 0);
    fprintf(
        (*info).f,
        b"} /* %s */\n\0" as *const u8 as *const libc::c_char,
        ab.as_mut_ptr(),
    );
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn pr_end_function(mut p: *mut libc::c_void) -> bool {
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn pr_lineno(
    mut p: *mut libc::c_void,
    mut filename_0: *const libc::c_char,
    mut lineno: libc::c_ulong,
    mut addr: bfd_vma,
) -> bool {
    let mut info: *mut pr_handle = p as *mut pr_handle;
    let mut ab: [libc::c_char; 22] = [0; 22];
    indent(info);
    print_vma(addr, ab.as_mut_ptr(), 1 as libc::c_int != 0, 1 as libc::c_int != 0);
    fprintf(
        (*info).f,
        b"/* file %s line %lu addr %s */\n\0" as *const u8 as *const libc::c_char,
        filename_0,
        lineno,
        ab.as_mut_ptr(),
    );
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn append_parent(
    mut info: *mut pr_handle,
    mut s: *const libc::c_char,
) -> bool {
    let mut len: libc::c_uint = 0;
    if s.is_null() {
        return 0 as libc::c_int != 0;
    }
    if !((*info).stack).is_null() {} else {
        __assert_fail(
            b"info->stack != NULL\0" as *const u8 as *const libc::c_char,
            b"prdbg.c\0" as *const u8 as *const libc::c_char,
            395 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 54],
                &[libc::c_char; 54],
            >(b"_Bool append_parent(struct pr_handle *, const char *)\0"))
                .as_ptr(),
        );
    }
    'c_19771: {
        if !((*info).stack).is_null() {} else {
            __assert_fail(
                b"info->stack != NULL\0" as *const u8 as *const libc::c_char,
                b"prdbg.c\0" as *const u8 as *const libc::c_char,
                395 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 54],
                    &[libc::c_char; 54],
                >(b"_Bool append_parent(struct pr_handle *, const char *)\0"))
                    .as_ptr(),
            );
        }
    };
    len = (if !((*(*info).stack).parents).is_null() {
        strlen((*(*info).stack).parents)
    } else {
        0 as libc::c_int as libc::c_ulong
    }) as libc::c_uint;
    (*(*info).stack)
        .parents = xrealloc(
        (*(*info).stack).parents as *mut libc::c_void,
        (len as libc::c_ulong)
            .wrapping_add(strlen(s))
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    strcpy(((*(*info).stack).parents).offset(len as isize), s);
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn tg_fix_visibility(
    mut info: *mut pr_handle,
    mut visibility: debug_visibility,
) -> bool {
    if !((*info).stack).is_null() {} else {
        __assert_fail(
            b"info->stack != NULL\0" as *const u8 as *const libc::c_char,
            b"prdbg.c\0" as *const u8 as *const libc::c_char,
            2053 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 67],
                &[libc::c_char; 67],
            >(b"_Bool tg_fix_visibility(struct pr_handle *, enum debug_visibility)\0"))
                .as_ptr(),
        );
    }
    'c_18980: {
        if !((*info).stack).is_null() {} else {
            __assert_fail(
                b"info->stack != NULL\0" as *const u8 as *const libc::c_char,
                b"prdbg.c\0" as *const u8 as *const libc::c_char,
                2053 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 67],
                    &[libc::c_char; 67],
                >(
                    b"_Bool tg_fix_visibility(struct pr_handle *, enum debug_visibility)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if (*(*info).stack).visibility as libc::c_uint == visibility as libc::c_uint {
        return 1 as libc::c_int != 0;
    }
    if (*(*info).stack).visibility as libc::c_uint
        != DEBUG_VISIBILITY_IGNORE as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"info->stack->visibility != DEBUG_VISIBILITY_IGNORE\0" as *const u8
                as *const libc::c_char,
            b"prdbg.c\0" as *const u8 as *const libc::c_char,
            2058 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 67],
                &[libc::c_char; 67],
            >(b"_Bool tg_fix_visibility(struct pr_handle *, enum debug_visibility)\0"))
                .as_ptr(),
        );
    }
    'c_18917: {
        if (*(*info).stack).visibility as libc::c_uint
            != DEBUG_VISIBILITY_IGNORE as libc::c_int as libc::c_uint
        {} else {
            __assert_fail(
                b"info->stack->visibility != DEBUG_VISIBILITY_IGNORE\0" as *const u8
                    as *const libc::c_char,
                b"prdbg.c\0" as *const u8 as *const libc::c_char,
                2058 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 67],
                    &[libc::c_char; 67],
                >(
                    b"_Bool tg_fix_visibility(struct pr_handle *, enum debug_visibility)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    (*(*info).stack).visibility = visibility;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn find_address_in_section(
    mut abfd: *mut bfd,
    mut section: *mut asection,
    mut data: *mut libc::c_void,
) {
    let mut vma: bfd_vma = 0;
    let mut size: bfd_size_type = 0;
    let mut syms: *mut *mut asymbol = data as *mut *mut asymbol;
    if found {
        return;
    }
    if bfd_section_flags(section) & 0x1 as libc::c_int as libc::c_uint
        == 0 as libc::c_int as libc::c_uint
    {
        return;
    }
    vma = bfd_section_vma(section);
    if pc < vma {
        return;
    }
    size = bfd_section_size(section);
    if pc >= vma.wrapping_add(size) {
        return;
    }
    found = (Some(
        ((*(*abfd).xvec)._bfd_find_nearest_line).expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )(
        abfd,
        syms,
        section,
        pc.wrapping_sub(vma),
        &mut filename,
        &mut functionname,
        &mut line,
        0 as *mut libc::c_uint,
    );
}
unsafe extern "C" fn translate_addresses(
    mut abfd: *mut bfd,
    mut addr_hex: *mut libc::c_char,
    mut f: *mut FILE,
    mut syms: *mut *mut asymbol,
) {
    pc = bfd_scan_vma(addr_hex, 0 as *mut *const libc::c_char, 16 as libc::c_int);
    found = 0 as libc::c_int != 0;
    bfd_map_over_sections(
        abfd,
        Some(
            find_address_in_section
                as unsafe extern "C" fn(*mut bfd, *mut asection, *mut libc::c_void) -> (),
        ),
        syms as *mut libc::c_void,
    );
    if !found {
        fprintf(f, b"??\0" as *const u8 as *const libc::c_char);
    } else {
        fprintf(f, b"%u\0" as *const u8 as *const libc::c_char, line);
    };
}
unsafe extern "C" fn visibility_name(
    mut visibility: debug_visibility,
) -> *const libc::c_char {
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    match visibility as libc::c_uint {
        0 => {
            s = b"public\0" as *const u8 as *const libc::c_char;
        }
        2 => {
            s = b"private\0" as *const u8 as *const libc::c_char;
        }
        1 => {
            s = b"protected\0" as *const u8 as *const libc::c_char;
        }
        3 => {
            s = b"/* ignore */\0" as *const u8 as *const libc::c_char;
        }
        _ => {
            abort();
        }
    }
    return s;
}
unsafe extern "C" fn tg_start_compilation_unit(
    mut p: *mut libc::c_void,
    mut fname: *const libc::c_char,
) -> bool {
    let mut info: *mut pr_handle = p as *mut pr_handle;
    free((*info).filename as *mut libc::c_void);
    (*info).filename = strdup(fname);
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn tg_start_source(
    mut p: *mut libc::c_void,
    mut fname: *const libc::c_char,
) -> bool {
    let mut info: *mut pr_handle = p as *mut pr_handle;
    free((*info).filename as *mut libc::c_void);
    (*info).filename = strdup(fname);
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn tg_enum_type(
    mut p: *mut libc::c_void,
    mut tag: *const libc::c_char,
    mut names: *mut *const libc::c_char,
    mut values: *mut bfd_signed_vma,
) -> bool {
    let mut info: *mut pr_handle = p as *mut pr_handle;
    let mut i: libc::c_uint = 0;
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut ab: [libc::c_char; 22] = [0; 22];
    if !pr_enum_type(p, tag, names, values) {
        return 0 as libc::c_int != 0;
    }
    name = if !tag.is_null() {
        tag
    } else {
        b"unknown\0" as *const u8 as *const libc::c_char
    };
    if !tag.is_null() {
        fprintf(
            (*info).f,
            b"%s\t%s\t0;\"\tkind:e\ttype:%s\n\0" as *const u8 as *const libc::c_char,
            tag,
            (*info).filename,
            (*(*info).stack).type_0,
        );
    }
    if !names.is_null() {
        i = 0 as libc::c_int as libc::c_uint;
        while !(*names.offset(i as isize)).is_null() {
            print_vma(
                *values.offset(i as isize) as bfd_vma,
                ab.as_mut_ptr(),
                0 as libc::c_int != 0,
                0 as libc::c_int != 0,
            );
            fprintf(
                (*info).f,
                b"%s\t%s\t0;\"\tkind:g\tenum:%s\tvalue:%s\n\0" as *const u8
                    as *const libc::c_char,
                *names.offset(i as isize),
                (*info).filename,
                name,
                ab.as_mut_ptr(),
            );
            i = i.wrapping_add(1);
            i;
        }
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn tg_start_struct_type(
    mut p: *mut libc::c_void,
    mut tag: *const libc::c_char,
    mut id: libc::c_uint,
    mut structp: bool,
    mut size: libc::c_uint,
) -> bool {
    let mut info: *mut pr_handle = p as *mut pr_handle;
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut idbuf: [libc::c_char; 20] = [0; 20];
    if !tag.is_null() {
        name = tag;
    } else {
        name = idbuf.as_mut_ptr();
        sprintf(
            idbuf.as_mut_ptr(),
            b"%%anon%u\0" as *const u8 as *const libc::c_char,
            id,
        );
    }
    if !push_type(info, name) {
        return 0 as libc::c_int != 0;
    }
    (*(*info).stack)
        .flavor = if structp as libc::c_int != 0 {
        b"struct\0" as *const u8 as *const libc::c_char
    } else {
        b"union\0" as *const u8 as *const libc::c_char
    };
    fprintf(
        (*info).f,
        b"%s\t%s\t0;\"\tkind:%c\n\0" as *const u8 as *const libc::c_char,
        name,
        (*info).filename,
        *((*(*info).stack).flavor).offset(0 as libc::c_int as isize) as libc::c_int,
    );
    (*(*info).stack).visibility = DEBUG_VISIBILITY_PUBLIC;
    return indent_type(info);
}
unsafe extern "C" fn tg_struct_field(
    mut p: *mut libc::c_void,
    mut name: *const libc::c_char,
    mut bitpos: bfd_vma,
    mut bitsize: bfd_vma,
    mut visibility: debug_visibility,
) -> bool {
    let mut info: *mut pr_handle = p as *mut pr_handle;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    t = pop_type(info);
    if t.is_null() {
        return 0 as libc::c_int != 0;
    }
    if !tg_fix_visibility(info, visibility) {
        return 0 as libc::c_int != 0;
    }
    if *name.offset(0 as libc::c_int as isize) == 0 {
        return 1 as libc::c_int != 0;
    }
    fprintf(
        (*info).f,
        b"%s\t%s\t0;\"\tkind:m\ttype:%s\t%s:%s\taccess:%s\n\0" as *const u8
            as *const libc::c_char,
        name,
        (*info).filename,
        t,
        (*(*info).stack).flavor,
        (*(*info).stack).type_0,
        visibility_name(visibility),
    );
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn tg_end_struct_type(mut p: *mut libc::c_void) -> bool {
    if !((*(p as *mut pr_handle)).stack).is_null() {} else {
        __assert_fail(
            b"((struct pr_handle *) p)->stack != NULL\0" as *const u8
                as *const libc::c_char,
            b"prdbg.c\0" as *const u8 as *const libc::c_char,
            2098 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"_Bool tg_end_struct_type(void *)\0"))
                .as_ptr(),
        );
    }
    'c_18612: {
        if !((*(p as *mut pr_handle)).stack).is_null() {} else {
            __assert_fail(
                b"((struct pr_handle *) p)->stack != NULL\0" as *const u8
                    as *const libc::c_char,
                b"prdbg.c\0" as *const u8 as *const libc::c_char,
                2098 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 33],
                    &[libc::c_char; 33],
                >(b"_Bool tg_end_struct_type(void *)\0"))
                    .as_ptr(),
            );
        }
    };
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn tg_start_class_type(
    mut p: *mut libc::c_void,
    mut tag: *const libc::c_char,
    mut id: libc::c_uint,
    mut structp: bool,
    mut size: libc::c_uint,
    mut vptr: bool,
    mut ownvptr: bool,
) -> bool {
    let mut info: *mut pr_handle = p as *mut pr_handle;
    let mut tv: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut idbuf: [libc::c_char; 20] = [0; 20];
    (*info).indent = ((*info).indent).wrapping_add(2 as libc::c_int as libc::c_uint);
    if vptr as libc::c_int != 0 && !ownvptr {
        tv = pop_type(info);
        if tv.is_null() {
            return 0 as libc::c_int != 0;
        }
    }
    if !tag.is_null() {
        name = tag;
    } else {
        sprintf(
            idbuf.as_mut_ptr(),
            b"%%anon%u\0" as *const u8 as *const libc::c_char,
            id,
        );
        name = idbuf.as_mut_ptr();
    }
    if !push_type(info, name) {
        return 0 as libc::c_int != 0;
    }
    (*(*info).stack)
        .flavor = if structp as libc::c_int != 0 {
        b"class\0" as *const u8 as *const libc::c_char
    } else {
        b"union class\0" as *const u8 as *const libc::c_char
    };
    (*(*info).stack).parents = 0 as *mut libc::c_char;
    (*(*info).stack).num_parents = 0 as libc::c_int;
    if size != 0 as libc::c_int as libc::c_uint || vptr as libc::c_int != 0
        || ownvptr as libc::c_int != 0 || !tag.is_null()
    {
        if vptr {
            if !append_type(info, b" vtable \0" as *const u8 as *const libc::c_char) {
                return 0 as libc::c_int != 0;
            }
            if ownvptr {
                if !append_type(info, b"self \0" as *const u8 as *const libc::c_char) {
                    return 0 as libc::c_int != 0;
                }
            } else if !append_type(info, tv)
                || !append_type(info, b" \0" as *const u8 as *const libc::c_char)
            {
                return 0 as libc::c_int != 0
            }
        }
    }
    (*(*info).stack).visibility = DEBUG_VISIBILITY_PRIVATE;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn tg_class_static_member(
    mut p: *mut libc::c_void,
    mut name: *const libc::c_char,
    mut physname: *const libc::c_char,
    mut visibility: debug_visibility,
) -> bool {
    let mut info: *mut pr_handle = p as *mut pr_handle;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len_var: libc::c_int = 0;
    let mut len_class: libc::c_int = 0;
    let mut full_name: *mut libc::c_char = 0 as *mut libc::c_char;
    len_var = strlen(name) as libc::c_int;
    len_class = strlen((*(*(*info).stack).next).type_0) as libc::c_int;
    full_name = xmalloc((len_var + len_class + 3 as libc::c_int) as size_t)
        as *mut libc::c_char;
    if full_name.is_null() {
        return 0 as libc::c_int != 0;
    }
    sprintf(
        full_name,
        b"%s::%s\0" as *const u8 as *const libc::c_char,
        (*(*(*info).stack).next).type_0,
        name,
    );
    if !substitute_type(info, full_name) {
        free(full_name as *mut libc::c_void);
        return 0 as libc::c_int != 0;
    }
    if !prepend_type(info, b"static \0" as *const u8 as *const libc::c_char) {
        free(full_name as *mut libc::c_void);
        return 0 as libc::c_int != 0;
    }
    t = pop_type(info);
    if t.is_null() {
        free(full_name as *mut libc::c_void);
        return 0 as libc::c_int != 0;
    }
    if !tg_fix_visibility(info, visibility) {
        free(t as *mut libc::c_void);
        free(full_name as *mut libc::c_void);
        return 0 as libc::c_int != 0;
    }
    fprintf(
        (*info).f,
        b"%s\t%s\t0;\"\tkind:x\ttype:%s\tclass:%s\taccess:%s\n\0" as *const u8
            as *const libc::c_char,
        name,
        (*info).filename,
        t,
        (*(*info).stack).type_0,
        visibility_name(visibility),
    );
    free(t as *mut libc::c_void);
    free(full_name as *mut libc::c_void);
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn tg_class_baseclass(
    mut p: *mut libc::c_void,
    mut bitpos: bfd_vma,
    mut is_virtual: bool,
    mut visibility: debug_visibility,
) -> bool {
    let mut info: *mut pr_handle = p as *mut pr_handle;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut prefix: *const libc::c_char = 0 as *const libc::c_char;
    if !((*info).stack).is_null() && !((*(*info).stack).next).is_null() {} else {
        __assert_fail(
            b"info->stack != NULL && info->stack->next != NULL\0" as *const u8
                as *const libc::c_char,
            b"prdbg.c\0" as *const u8 as *const libc::c_char,
            2228 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 72],
                &[libc::c_char; 72],
            >(
                b"_Bool tg_class_baseclass(void *, bfd_vma, _Bool, enum debug_visibility)\0",
            ))
                .as_ptr(),
        );
    }
    'c_19975: {
        if !((*info).stack).is_null() && !((*(*info).stack).next).is_null() {} else {
            __assert_fail(
                b"info->stack != NULL && info->stack->next != NULL\0" as *const u8
                    as *const libc::c_char,
                b"prdbg.c\0" as *const u8 as *const libc::c_char,
                2228 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 72],
                    &[libc::c_char; 72],
                >(
                    b"_Bool tg_class_baseclass(void *, bfd_vma, _Bool, enum debug_visibility)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    t = pop_type(info);
    if t.is_null() {
        return 0 as libc::c_int != 0;
    }
    if startswith(t, b"class \0" as *const u8 as *const libc::c_char) {
        t = t
            .offset(
                (::core::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
            );
    }
    if !push_type(info, t) {
        return 0 as libc::c_int != 0;
    }
    if is_virtual {
        if !prepend_type(info, b"virtual \0" as *const u8 as *const libc::c_char) {
            return 0 as libc::c_int != 0;
        }
    }
    match visibility as libc::c_uint {
        0 => {
            prefix = b"public \0" as *const u8 as *const libc::c_char;
        }
        1 => {
            prefix = b"protected \0" as *const u8 as *const libc::c_char;
        }
        2 => {
            prefix = b"private \0" as *const u8 as *const libc::c_char;
        }
        _ => {
            prefix = b"/* unknown visibility */ \0" as *const u8 as *const libc::c_char;
        }
    }
    if !prepend_type(info, prefix) {
        return 0 as libc::c_int != 0;
    }
    t = pop_type(info);
    if t.is_null() {
        return 0 as libc::c_int != 0;
    }
    if (*(*info).stack).num_parents != 0
        && !append_parent(info, b", \0" as *const u8 as *const libc::c_char)
    {
        return 0 as libc::c_int != 0;
    }
    if !append_parent(info, t) {
        return 0 as libc::c_int != 0;
    }
    (*(*info).stack).num_parents += 1;
    (*(*info).stack).num_parents;
    free(t as *mut libc::c_void);
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn tg_class_method_variant(
    mut p: *mut libc::c_void,
    mut physname: *const libc::c_char,
    mut visibility: debug_visibility,
    mut constp: bool,
    mut volatilep: bool,
    mut voffset: bfd_vma,
    mut context: bool,
) -> bool {
    let mut info: *mut pr_handle = p as *mut pr_handle;
    let mut method_type: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut context_type: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut method_name: *mut libc::c_char = 0 as *mut libc::c_char;
    if !((*info).stack).is_null() {} else {
        __assert_fail(
            b"info->stack != NULL\0" as *const u8 as *const libc::c_char,
            b"prdbg.c\0" as *const u8 as *const libc::c_char,
            2297 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 105],
                &[libc::c_char; 105],
            >(
                b"_Bool tg_class_method_variant(void *, const char *, enum debug_visibility, _Bool, _Bool, bfd_vma, _Bool)\0",
            ))
                .as_ptr(),
        );
    }
    'c_19589: {
        if !((*info).stack).is_null() {} else {
            __assert_fail(
                b"info->stack != NULL\0" as *const u8 as *const libc::c_char,
                b"prdbg.c\0" as *const u8 as *const libc::c_char,
                2297 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 105],
                    &[libc::c_char; 105],
                >(
                    b"_Bool tg_class_method_variant(void *, const char *, enum debug_visibility, _Bool, _Bool, bfd_vma, _Bool)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if !((*(*info).stack).next).is_null() {} else {
        __assert_fail(
            b"info->stack->next != NULL\0" as *const u8 as *const libc::c_char,
            b"prdbg.c\0" as *const u8 as *const libc::c_char,
            2298 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 105],
                &[libc::c_char; 105],
            >(
                b"_Bool tg_class_method_variant(void *, const char *, enum debug_visibility, _Bool, _Bool, bfd_vma, _Bool)\0",
            ))
                .as_ptr(),
        );
    }
    'c_19539: {
        if !((*(*info).stack).next).is_null() {} else {
            __assert_fail(
                b"info->stack->next != NULL\0" as *const u8 as *const libc::c_char,
                b"prdbg.c\0" as *const u8 as *const libc::c_char,
                2298 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 105],
                    &[libc::c_char; 105],
                >(
                    b"_Bool tg_class_method_variant(void *, const char *, enum debug_visibility, _Bool, _Bool, bfd_vma, _Bool)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if volatilep {
        if !append_type(info, b" volatile\0" as *const u8 as *const libc::c_char) {
            return 0 as libc::c_int != 0;
        }
    }
    if constp {
        if !append_type(info, b" const\0" as *const u8 as *const libc::c_char) {
            return 0 as libc::c_int != 0;
        }
    }
    method_name = strdup(
        if context as libc::c_int != 0 {
            (*(*(*(*info).stack).next).next).method
        } else {
            (*(*(*info).stack).next).method
        },
    );
    if !substitute_type(info, method_name) {
        free(method_name as *mut libc::c_void);
        return 0 as libc::c_int != 0;
    }
    method_type = pop_type(info);
    if method_type.is_null() {
        free(method_name as *mut libc::c_void);
        return 0 as libc::c_int != 0;
    }
    if !context {
        context_type = 0 as *mut libc::c_char;
    } else {
        context_type = pop_type(info);
        if context_type.is_null() {
            free(method_type as *mut libc::c_void);
            free(method_name as *mut libc::c_void);
            return 0 as libc::c_int != 0;
        }
    }
    if !tg_fix_visibility(info, visibility) {
        free(method_type as *mut libc::c_void);
        free(method_name as *mut libc::c_void);
        free(context_type as *mut libc::c_void);
        return 0 as libc::c_int != 0;
    }
    fprintf(
        (*info).f,
        b"%s\t%s\t0;\"\tkind:p\ttype:%s\tclass:%s\n\0" as *const u8
            as *const libc::c_char,
        method_name,
        (*info).filename,
        method_type,
        (*(*info).stack).type_0,
    );
    free(method_type as *mut libc::c_void);
    free(method_name as *mut libc::c_void);
    free(context_type as *mut libc::c_void);
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn tg_class_static_method_variant(
    mut p: *mut libc::c_void,
    mut physname: *const libc::c_char,
    mut visibility: debug_visibility,
    mut constp: bool,
    mut volatilep: bool,
) -> bool {
    let mut info: *mut pr_handle = p as *mut pr_handle;
    let mut method_type: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut method_name: *mut libc::c_char = 0 as *mut libc::c_char;
    if !((*info).stack).is_null() {} else {
        __assert_fail(
            b"info->stack != NULL\0" as *const u8 as *const libc::c_char,
            b"prdbg.c\0" as *const u8 as *const libc::c_char,
            2374 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 96],
                &[libc::c_char; 96],
            >(
                b"_Bool tg_class_static_method_variant(void *, const char *, enum debug_visibility, _Bool, _Bool)\0",
            ))
                .as_ptr(),
        );
    }
    'c_19234: {
        if !((*info).stack).is_null() {} else {
            __assert_fail(
                b"info->stack != NULL\0" as *const u8 as *const libc::c_char,
                b"prdbg.c\0" as *const u8 as *const libc::c_char,
                2374 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 96],
                    &[libc::c_char; 96],
                >(
                    b"_Bool tg_class_static_method_variant(void *, const char *, enum debug_visibility, _Bool, _Bool)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if !((*(*info).stack).next).is_null() {} else {
        __assert_fail(
            b"info->stack->next != NULL\0" as *const u8 as *const libc::c_char,
            b"prdbg.c\0" as *const u8 as *const libc::c_char,
            2375 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 96],
                &[libc::c_char; 96],
            >(
                b"_Bool tg_class_static_method_variant(void *, const char *, enum debug_visibility, _Bool, _Bool)\0",
            ))
                .as_ptr(),
        );
    }
    'c_19184: {
        if !((*(*info).stack).next).is_null() {} else {
            __assert_fail(
                b"info->stack->next != NULL\0" as *const u8 as *const libc::c_char,
                b"prdbg.c\0" as *const u8 as *const libc::c_char,
                2375 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 96],
                    &[libc::c_char; 96],
                >(
                    b"_Bool tg_class_static_method_variant(void *, const char *, enum debug_visibility, _Bool, _Bool)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if !((*(*(*info).stack).next).method).is_null() {} else {
        __assert_fail(
            b"info->stack->next->method != NULL\0" as *const u8 as *const libc::c_char,
            b"prdbg.c\0" as *const u8 as *const libc::c_char,
            2376 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 96],
                &[libc::c_char; 96],
            >(
                b"_Bool tg_class_static_method_variant(void *, const char *, enum debug_visibility, _Bool, _Bool)\0",
            ))
                .as_ptr(),
        );
    }
    'c_19130: {
        if !((*(*(*info).stack).next).method).is_null() {} else {
            __assert_fail(
                b"info->stack->next->method != NULL\0" as *const u8
                    as *const libc::c_char,
                b"prdbg.c\0" as *const u8 as *const libc::c_char,
                2376 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 96],
                    &[libc::c_char; 96],
                >(
                    b"_Bool tg_class_static_method_variant(void *, const char *, enum debug_visibility, _Bool, _Bool)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if volatilep {
        if !append_type(info, b" volatile\0" as *const u8 as *const libc::c_char) {
            return 0 as libc::c_int != 0;
        }
    }
    if constp {
        if !append_type(info, b" const\0" as *const u8 as *const libc::c_char) {
            return 0 as libc::c_int != 0;
        }
    }
    if !prepend_type(info, b"static \0" as *const u8 as *const libc::c_char) {
        return 0 as libc::c_int != 0;
    }
    method_name = strdup((*(*(*info).stack).next).method);
    if !substitute_type(info, (*(*(*info).stack).next).method) {
        free(method_name as *mut libc::c_void);
        return 0 as libc::c_int != 0;
    }
    method_type = pop_type(info);
    if method_type.is_null() {
        free(method_name as *mut libc::c_void);
        return 0 as libc::c_int != 0;
    }
    if !tg_fix_visibility(info, visibility) {
        free(method_type as *mut libc::c_void);
        free(method_name as *mut libc::c_void);
        return 0 as libc::c_int != 0;
    }
    fprintf(
        (*info).f,
        b"%s\t%s\t0;\"\tkind:p\ttype:%s\tclass:%s\taccess:%s\n\0" as *const u8
            as *const libc::c_char,
        method_name,
        (*info).filename,
        method_type,
        (*(*info).stack).type_0,
        visibility_name(visibility),
    );
    free(method_type as *mut libc::c_void);
    free(method_name as *mut libc::c_void);
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn tg_end_class_type(mut p: *mut libc::c_void) -> bool {
    let mut info: *mut pr_handle = p as *mut pr_handle;
    fprintf(
        (*info).f,
        b"%s\t%s\t0;\"\tkind:c\ttype:%s\0" as *const u8 as *const libc::c_char,
        (*(*info).stack).type_0,
        (*info).filename,
        (*(*info).stack).flavor,
    );
    if (*(*info).stack).num_parents != 0 {
        fprintf(
            (*info).f,
            b"\tinherits:%s\0" as *const u8 as *const libc::c_char,
            (*(*info).stack).parents,
        );
        free((*(*info).stack).parents as *mut libc::c_void);
    }
    fputc('\n' as i32, (*info).f);
    return tg_end_struct_type(p);
}
unsafe extern "C" fn tg_tag_type(
    mut p: *mut libc::c_void,
    mut name: *const libc::c_char,
    mut id: libc::c_uint,
    mut kind: debug_type_kind,
) -> bool {
    let mut info: *mut pr_handle = p as *mut pr_handle;
    let mut t: *const libc::c_char = 0 as *const libc::c_char;
    let mut tag: *const libc::c_char = 0 as *const libc::c_char;
    let mut idbuf: [libc::c_char; 20] = [0; 20];
    match kind as libc::c_uint {
        7 => {
            t = b"struct \0" as *const u8 as *const libc::c_char;
        }
        8 => {
            t = b"union \0" as *const u8 as *const libc::c_char;
        }
        11 => {
            t = b"enum \0" as *const u8 as *const libc::c_char;
        }
        9 => {
            t = b"class \0" as *const u8 as *const libc::c_char;
        }
        10 => {
            t = b"union class \0" as *const u8 as *const libc::c_char;
        }
        _ => {
            abort();
        }
    }
    if !push_type(info, t) {
        return 0 as libc::c_int != 0;
    }
    if !name.is_null() {
        tag = name;
    } else {
        sprintf(
            idbuf.as_mut_ptr(),
            b"%%anon%u\0" as *const u8 as *const libc::c_char,
            id,
        );
        tag = idbuf.as_mut_ptr();
    }
    if !append_type(info, tag) {
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn tg_typdef(
    mut p: *mut libc::c_void,
    mut name: *const libc::c_char,
) -> bool {
    let mut info: *mut pr_handle = p as *mut pr_handle;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    s = pop_type(info);
    if s.is_null() {
        return 0 as libc::c_int != 0;
    }
    fprintf(
        (*info).f,
        b"%s\t%s\t0;\"\tkind:t\ttype:%s\n\0" as *const u8 as *const libc::c_char,
        name,
        (*info).filename,
        s,
    );
    free(s as *mut libc::c_void);
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn tg_tag(
    mut p: *mut libc::c_void,
    mut name: *const libc::c_char,
) -> bool {
    let mut info: *mut pr_handle = p as *mut pr_handle;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    t = pop_type(info);
    if t.is_null() {
        return 0 as libc::c_int != 0;
    }
    free(t as *mut libc::c_void);
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn tg_int_constant(
    mut p: *mut libc::c_void,
    mut name: *const libc::c_char,
    mut val: bfd_vma,
) -> bool {
    let mut info: *mut pr_handle = p as *mut pr_handle;
    let mut ab: [libc::c_char; 22] = [0; 22];
    indent(info);
    print_vma(val, ab.as_mut_ptr(), 0 as libc::c_int != 0, 0 as libc::c_int != 0);
    fprintf(
        (*info).f,
        b"%s\t%s\t0;\"\tkind:v\ttype:const int\tvalue:%s\n\0" as *const u8
            as *const libc::c_char,
        name,
        (*info).filename,
        ab.as_mut_ptr(),
    );
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn tg_float_constant(
    mut p: *mut libc::c_void,
    mut name: *const libc::c_char,
    mut val: libc::c_double,
) -> bool {
    let mut info: *mut pr_handle = p as *mut pr_handle;
    indent(info);
    fprintf(
        (*info).f,
        b"%s\t%s\t0;\"\tkind:v\ttype:const double\tvalue:%g\n\0" as *const u8
            as *const libc::c_char,
        name,
        (*info).filename,
        val,
    );
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn tg_typed_constant(
    mut p: *mut libc::c_void,
    mut name: *const libc::c_char,
    mut val: bfd_vma,
) -> bool {
    let mut info: *mut pr_handle = p as *mut pr_handle;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ab: [libc::c_char; 22] = [0; 22];
    t = pop_type(info);
    if t.is_null() {
        return 0 as libc::c_int != 0;
    }
    indent(info);
    print_vma(val, ab.as_mut_ptr(), 0 as libc::c_int != 0, 0 as libc::c_int != 0);
    fprintf(
        (*info).f,
        b"%s\t%s\t0;\"\tkind:v\ttype:const %s\tvalue:%s\n\0" as *const u8
            as *const libc::c_char,
        name,
        (*info).filename,
        t,
        ab.as_mut_ptr(),
    );
    free(t as *mut libc::c_void);
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn tg_variable(
    mut p: *mut libc::c_void,
    mut name: *const libc::c_char,
    mut kind: debug_var_kind,
    mut val: bfd_vma,
) -> bool {
    let mut info: *mut pr_handle = p as *mut pr_handle;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut from_class: *mut libc::c_char = 0 as *mut libc::c_char;
    t = pop_type(info);
    if t.is_null() {
        return 0 as libc::c_int != 0;
    }
    dname = 0 as *mut libc::c_char;
    if ((*info).demangler).is_some() {
        dname = ((*info).demangler)
            .expect("non-null function pointer")((*info).abfd, name, demangle_flags);
    }
    from_class = 0 as *mut libc::c_char;
    if !dname.is_null() {
        let mut sep: *mut libc::c_char = 0 as *mut libc::c_char;
        sep = strstr(dname, b"::\0" as *const u8 as *const libc::c_char);
        if !sep.is_null() {
            *sep = 0 as libc::c_int as libc::c_char;
            name = sep.offset(2 as libc::c_int as isize);
            from_class = dname;
        } else {
            name = dname;
        }
    }
    fprintf(
        (*info).f,
        b"%s\t%s\t0;\"\tkind:v\ttype:%s\0" as *const u8 as *const libc::c_char,
        name,
        (*info).filename,
        t,
    );
    match kind as libc::c_uint {
        2 | 3 => {
            fprintf((*info).f, b"\tfile:\0" as *const u8 as *const libc::c_char);
        }
        5 => {
            fprintf((*info).f, b"\tregister:\0" as *const u8 as *const libc::c_char);
        }
        _ => {}
    }
    if !from_class.is_null() {
        fprintf(
            (*info).f,
            b"\tclass:%s\0" as *const u8 as *const libc::c_char,
            from_class,
        );
    }
    if !dname.is_null() {
        free(dname as *mut libc::c_void);
    }
    fprintf((*info).f, b"\n\0" as *const u8 as *const libc::c_char);
    free(t as *mut libc::c_void);
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn tg_start_function(
    mut p: *mut libc::c_void,
    mut name: *const libc::c_char,
    mut global: bool,
) -> bool {
    let mut info: *mut pr_handle = p as *mut pr_handle;
    let mut dname: *mut libc::c_char = 0 as *mut libc::c_char;
    if !global {
        (*(*info).stack).flavor = b"static\0" as *const u8 as *const libc::c_char;
    } else {
        (*(*info).stack).flavor = 0 as *const libc::c_char;
    }
    dname = 0 as *mut libc::c_char;
    if ((*info).demangler).is_some() {
        dname = ((*info).demangler)
            .expect("non-null function pointer")((*info).abfd, name, demangle_flags);
    }
    if !substitute_type(
        info,
        if !dname.is_null() { dname as *const libc::c_char } else { name },
    ) {
        return 0 as libc::c_int != 0;
    }
    (*(*info).stack).method = 0 as *const libc::c_char;
    if !dname.is_null() {
        let mut sep: *mut libc::c_char = 0 as *mut libc::c_char;
        sep = strstr(dname, b"::\0" as *const u8 as *const libc::c_char);
        if !sep.is_null() {
            (*(*info).stack).method = dname;
            *sep = 0 as libc::c_int as libc::c_char;
            name = sep.offset(2 as libc::c_int as isize);
        } else {
            (*(*info).stack).method = b"\0" as *const u8 as *const libc::c_char;
            name = dname;
        }
        sep = strchr(name, '(' as i32);
        if !sep.is_null() {
            *sep = 0 as libc::c_int as libc::c_char;
        }
    }
    (*(*info).stack).parents = strdup(name);
    if ((*(*info).stack).method).is_null()
        && !append_type(info, b"(\0" as *const u8 as *const libc::c_char)
    {
        return 0 as libc::c_int != 0;
    }
    (*info).parameter = 1 as libc::c_int;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn tg_function_parameter(
    mut p: *mut libc::c_void,
    mut name: *const libc::c_char,
    mut kind: debug_parm_kind,
    mut val: bfd_vma,
) -> bool {
    let mut info: *mut pr_handle = p as *mut pr_handle;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    if kind as libc::c_uint == DEBUG_PARM_REFERENCE as libc::c_int as libc::c_uint
        || kind as libc::c_uint == DEBUG_PARM_REF_REG as libc::c_int as libc::c_uint
    {
        if !pr_reference_type(p) {
            return 0 as libc::c_int != 0;
        }
    }
    if !substitute_type(info, name) {
        return 0 as libc::c_int != 0;
    }
    t = pop_type(info);
    if t.is_null() {
        return 0 as libc::c_int != 0;
    }
    if ((*(*info).stack).method).is_null() {
        if (*info).parameter != 1 as libc::c_int
            && !append_type(info, b", \0" as *const u8 as *const libc::c_char)
        {
            return 0 as libc::c_int != 0;
        }
        if kind as libc::c_uint == DEBUG_PARM_REG as libc::c_int as libc::c_uint
            || kind as libc::c_uint == DEBUG_PARM_REF_REG as libc::c_int as libc::c_uint
        {
            if !append_type(info, b"register \0" as *const u8 as *const libc::c_char) {
                return 0 as libc::c_int != 0;
            }
        }
        if !append_type(info, t) {
            return 0 as libc::c_int != 0;
        }
    }
    free(t as *mut libc::c_void);
    (*info).parameter += 1;
    (*info).parameter;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn tg_start_block(
    mut p: *mut libc::c_void,
    mut addr: bfd_vma,
) -> bool {
    let mut info: *mut pr_handle = p as *mut pr_handle;
    let mut ab: [libc::c_char; 22] = [0; 22];
    let mut kind: libc::c_char = 0;
    let mut partof: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut local: bool = false;
    if (*info).parameter > 0 as libc::c_int {
        (*info).parameter = 0 as libc::c_int;
        fprintf(
            (*info).f,
            b"%s\t%s\t\0" as *const u8 as *const libc::c_char,
            (*(*info).stack).parents,
            (*info).filename,
        );
        free((*(*info).stack).parents as *mut libc::c_void);
        print_vma(addr, ab.as_mut_ptr(), 1 as libc::c_int != 0, 1 as libc::c_int != 0);
        translate_addresses((*info).abfd, ab.as_mut_ptr(), (*info).f, (*info).syms);
        local = !((*(*info).stack).flavor).is_null();
        if !((*(*info).stack).method).is_null()
            && *(*(*info).stack).method as libc::c_int != 0
        {
            kind = 'm' as i32 as libc::c_char;
            partof = (*(*info).stack).method as *mut libc::c_char;
        } else {
            kind = 'f' as i32 as libc::c_char;
            partof = 0 as *mut libc::c_char;
            if ((*(*info).stack).method).is_null()
                && !append_type(info, b")\0" as *const u8 as *const libc::c_char)
            {
                return 0 as libc::c_int != 0;
            }
        }
        t = pop_type(info);
        if t.is_null() {
            return 0 as libc::c_int != 0;
        }
        fprintf(
            (*info).f,
            b";\"\tkind:%c\ttype:%s\0" as *const u8 as *const libc::c_char,
            kind as libc::c_int,
            t,
        );
        if local {
            fputs(b"\tfile:\0" as *const u8 as *const libc::c_char, (*info).f);
        }
        if !partof.is_null() {
            fprintf(
                (*info).f,
                b"\tclass:%s\0" as *const u8 as *const libc::c_char,
                partof,
            );
            free(partof as *mut libc::c_void);
        }
        fputc('\n' as i32, (*info).f);
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn tg_end_block(mut p: *mut libc::c_void, mut addr: bfd_vma) -> bool {
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn tg_lineno(
    mut p: *mut libc::c_void,
    mut fname: *const libc::c_char,
    mut lineno: libc::c_ulong,
    mut addr: bfd_vma,
) -> bool {
    return 1 as libc::c_int != 0;
}
static mut pr_fns: debug_write_fns = unsafe {
    {
        let mut init = debug_write_fns {
            start_compilation_unit: Some(
                pr_start_compilation_unit
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_char,
                    ) -> bool,
            ),
            start_source: Some(
                pr_start_source
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_char,
                    ) -> bool,
            ),
            empty_type: Some(
                pr_empty_type as unsafe extern "C" fn(*mut libc::c_void) -> bool,
            ),
            void_type: Some(
                pr_void_type as unsafe extern "C" fn(*mut libc::c_void) -> bool,
            ),
            int_type: Some(
                pr_int_type
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        libc::c_uint,
                        bool,
                    ) -> bool,
            ),
            float_type: Some(
                pr_float_type
                    as unsafe extern "C" fn(*mut libc::c_void, libc::c_uint) -> bool,
            ),
            complex_type: Some(
                pr_complex_type
                    as unsafe extern "C" fn(*mut libc::c_void, libc::c_uint) -> bool,
            ),
            bool_type: Some(
                pr_bool_type
                    as unsafe extern "C" fn(*mut libc::c_void, libc::c_uint) -> bool,
            ),
            enum_type: Some(
                pr_enum_type
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_char,
                        *mut *const libc::c_char,
                        *mut bfd_signed_vma,
                    ) -> bool,
            ),
            pointer_type: Some(
                pr_pointer_type as unsafe extern "C" fn(*mut libc::c_void) -> bool,
            ),
            function_type: Some(
                pr_function_type
                    as unsafe extern "C" fn(*mut libc::c_void, libc::c_int, bool) -> bool,
            ),
            reference_type: Some(
                pr_reference_type as unsafe extern "C" fn(*mut libc::c_void) -> bool,
            ),
            range_type: Some(
                pr_range_type
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        bfd_signed_vma,
                        bfd_signed_vma,
                    ) -> bool,
            ),
            array_type: Some(
                pr_array_type
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        bfd_signed_vma,
                        bfd_signed_vma,
                        bool,
                    ) -> bool,
            ),
            set_type: Some(
                pr_set_type as unsafe extern "C" fn(*mut libc::c_void, bool) -> bool,
            ),
            offset_type: Some(
                pr_offset_type as unsafe extern "C" fn(*mut libc::c_void) -> bool,
            ),
            method_type: Some(
                pr_method_type
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        bool,
                        libc::c_int,
                        bool,
                    ) -> bool,
            ),
            const_type: Some(
                pr_const_type as unsafe extern "C" fn(*mut libc::c_void) -> bool,
            ),
            volatile_type: Some(
                pr_volatile_type as unsafe extern "C" fn(*mut libc::c_void) -> bool,
            ),
            start_struct_type: Some(
                pr_start_struct_type
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_char,
                        libc::c_uint,
                        bool,
                        libc::c_uint,
                    ) -> bool,
            ),
            struct_field: Some(
                pr_struct_field
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_char,
                        bfd_vma,
                        bfd_vma,
                        debug_visibility,
                    ) -> bool,
            ),
            end_struct_type: Some(
                pr_end_struct_type as unsafe extern "C" fn(*mut libc::c_void) -> bool,
            ),
            start_class_type: Some(
                pr_start_class_type
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_char,
                        libc::c_uint,
                        bool,
                        libc::c_uint,
                        bool,
                        bool,
                    ) -> bool,
            ),
            class_static_member: Some(
                pr_class_static_member
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_char,
                        *const libc::c_char,
                        debug_visibility,
                    ) -> bool,
            ),
            class_baseclass: Some(
                pr_class_baseclass
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        bfd_vma,
                        bool,
                        debug_visibility,
                    ) -> bool,
            ),
            class_start_method: Some(
                pr_class_start_method
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_char,
                    ) -> bool,
            ),
            class_method_variant: Some(
                pr_class_method_variant
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_char,
                        debug_visibility,
                        bool,
                        bool,
                        bfd_vma,
                        bool,
                    ) -> bool,
            ),
            class_static_method_variant: Some(
                pr_class_static_method_variant
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_char,
                        debug_visibility,
                        bool,
                        bool,
                    ) -> bool,
            ),
            class_end_method: Some(
                pr_class_end_method as unsafe extern "C" fn(*mut libc::c_void) -> bool,
            ),
            end_class_type: Some(
                pr_end_class_type as unsafe extern "C" fn(*mut libc::c_void) -> bool,
            ),
            typedef_type: Some(
                pr_typedef_type
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_char,
                    ) -> bool,
            ),
            tag_type: Some(
                pr_tag_type
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_char,
                        libc::c_uint,
                        debug_type_kind,
                    ) -> bool,
            ),
            typdef: Some(
                pr_typdef
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_char,
                    ) -> bool,
            ),
            tag: Some(
                pr_tag
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_char,
                    ) -> bool,
            ),
            int_constant: Some(
                pr_int_constant
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_char,
                        bfd_vma,
                    ) -> bool,
            ),
            float_constant: Some(
                pr_float_constant
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_char,
                        libc::c_double,
                    ) -> bool,
            ),
            typed_constant: Some(
                pr_typed_constant
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_char,
                        bfd_vma,
                    ) -> bool,
            ),
            variable: Some(
                pr_variable
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_char,
                        debug_var_kind,
                        bfd_vma,
                    ) -> bool,
            ),
            start_function: Some(
                pr_start_function
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_char,
                        bool,
                    ) -> bool,
            ),
            function_parameter: Some(
                pr_function_parameter
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_char,
                        debug_parm_kind,
                        bfd_vma,
                    ) -> bool,
            ),
            start_block: Some(
                pr_start_block
                    as unsafe extern "C" fn(*mut libc::c_void, bfd_vma) -> bool,
            ),
            end_block: Some(
                pr_end_block as unsafe extern "C" fn(*mut libc::c_void, bfd_vma) -> bool,
            ),
            end_function: Some(
                pr_end_function as unsafe extern "C" fn(*mut libc::c_void) -> bool,
            ),
            lineno: Some(
                pr_lineno
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_char,
                        libc::c_ulong,
                        bfd_vma,
                    ) -> bool,
            ),
        };
        init
    }
};
static mut tg_fns: debug_write_fns = unsafe {
    {
        let mut init = debug_write_fns {
            start_compilation_unit: Some(
                tg_start_compilation_unit
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_char,
                    ) -> bool,
            ),
            start_source: Some(
                tg_start_source
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_char,
                    ) -> bool,
            ),
            empty_type: Some(
                pr_empty_type as unsafe extern "C" fn(*mut libc::c_void) -> bool,
            ),
            void_type: Some(
                pr_void_type as unsafe extern "C" fn(*mut libc::c_void) -> bool,
            ),
            int_type: Some(
                pr_int_type
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        libc::c_uint,
                        bool,
                    ) -> bool,
            ),
            float_type: Some(
                pr_float_type
                    as unsafe extern "C" fn(*mut libc::c_void, libc::c_uint) -> bool,
            ),
            complex_type: Some(
                pr_complex_type
                    as unsafe extern "C" fn(*mut libc::c_void, libc::c_uint) -> bool,
            ),
            bool_type: Some(
                pr_bool_type
                    as unsafe extern "C" fn(*mut libc::c_void, libc::c_uint) -> bool,
            ),
            enum_type: Some(
                tg_enum_type
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_char,
                        *mut *const libc::c_char,
                        *mut bfd_signed_vma,
                    ) -> bool,
            ),
            pointer_type: Some(
                pr_pointer_type as unsafe extern "C" fn(*mut libc::c_void) -> bool,
            ),
            function_type: Some(
                pr_function_type
                    as unsafe extern "C" fn(*mut libc::c_void, libc::c_int, bool) -> bool,
            ),
            reference_type: Some(
                pr_reference_type as unsafe extern "C" fn(*mut libc::c_void) -> bool,
            ),
            range_type: Some(
                pr_range_type
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        bfd_signed_vma,
                        bfd_signed_vma,
                    ) -> bool,
            ),
            array_type: Some(
                pr_array_type
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        bfd_signed_vma,
                        bfd_signed_vma,
                        bool,
                    ) -> bool,
            ),
            set_type: Some(
                pr_set_type as unsafe extern "C" fn(*mut libc::c_void, bool) -> bool,
            ),
            offset_type: Some(
                pr_offset_type as unsafe extern "C" fn(*mut libc::c_void) -> bool,
            ),
            method_type: Some(
                pr_method_type
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        bool,
                        libc::c_int,
                        bool,
                    ) -> bool,
            ),
            const_type: Some(
                pr_const_type as unsafe extern "C" fn(*mut libc::c_void) -> bool,
            ),
            volatile_type: Some(
                pr_volatile_type as unsafe extern "C" fn(*mut libc::c_void) -> bool,
            ),
            start_struct_type: Some(
                tg_start_struct_type
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_char,
                        libc::c_uint,
                        bool,
                        libc::c_uint,
                    ) -> bool,
            ),
            struct_field: Some(
                tg_struct_field
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_char,
                        bfd_vma,
                        bfd_vma,
                        debug_visibility,
                    ) -> bool,
            ),
            end_struct_type: Some(
                tg_end_struct_type as unsafe extern "C" fn(*mut libc::c_void) -> bool,
            ),
            start_class_type: Some(
                tg_start_class_type
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_char,
                        libc::c_uint,
                        bool,
                        libc::c_uint,
                        bool,
                        bool,
                    ) -> bool,
            ),
            class_static_member: Some(
                tg_class_static_member
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_char,
                        *const libc::c_char,
                        debug_visibility,
                    ) -> bool,
            ),
            class_baseclass: Some(
                tg_class_baseclass
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        bfd_vma,
                        bool,
                        debug_visibility,
                    ) -> bool,
            ),
            class_start_method: Some(
                pr_class_start_method
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_char,
                    ) -> bool,
            ),
            class_method_variant: Some(
                tg_class_method_variant
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_char,
                        debug_visibility,
                        bool,
                        bool,
                        bfd_vma,
                        bool,
                    ) -> bool,
            ),
            class_static_method_variant: Some(
                tg_class_static_method_variant
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_char,
                        debug_visibility,
                        bool,
                        bool,
                    ) -> bool,
            ),
            class_end_method: Some(
                pr_class_end_method as unsafe extern "C" fn(*mut libc::c_void) -> bool,
            ),
            end_class_type: Some(
                tg_end_class_type as unsafe extern "C" fn(*mut libc::c_void) -> bool,
            ),
            typedef_type: Some(
                pr_typedef_type
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_char,
                    ) -> bool,
            ),
            tag_type: Some(
                tg_tag_type
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_char,
                        libc::c_uint,
                        debug_type_kind,
                    ) -> bool,
            ),
            typdef: Some(
                tg_typdef
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_char,
                    ) -> bool,
            ),
            tag: Some(
                tg_tag
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_char,
                    ) -> bool,
            ),
            int_constant: Some(
                tg_int_constant
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_char,
                        bfd_vma,
                    ) -> bool,
            ),
            float_constant: Some(
                tg_float_constant
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_char,
                        libc::c_double,
                    ) -> bool,
            ),
            typed_constant: Some(
                tg_typed_constant
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_char,
                        bfd_vma,
                    ) -> bool,
            ),
            variable: Some(
                tg_variable
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_char,
                        debug_var_kind,
                        bfd_vma,
                    ) -> bool,
            ),
            start_function: Some(
                tg_start_function
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_char,
                        bool,
                    ) -> bool,
            ),
            function_parameter: Some(
                tg_function_parameter
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_char,
                        debug_parm_kind,
                        bfd_vma,
                    ) -> bool,
            ),
            start_block: Some(
                tg_start_block
                    as unsafe extern "C" fn(*mut libc::c_void, bfd_vma) -> bool,
            ),
            end_block: Some(
                tg_end_block as unsafe extern "C" fn(*mut libc::c_void, bfd_vma) -> bool,
            ),
            end_function: Some(
                pr_end_function as unsafe extern "C" fn(*mut libc::c_void) -> bool,
            ),
            lineno: Some(
                tg_lineno
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_char,
                        libc::c_ulong,
                        bfd_vma,
                    ) -> bool,
            ),
        };
        init
    }
};
static mut demangle_flags: libc::c_int = (1 as libc::c_int) << 1 as libc::c_int
    | (1 as libc::c_int) << 0 as libc::c_int;
static mut pc: bfd_vma = 0;
static mut filename: *const libc::c_char = 0 as *const libc::c_char;
static mut functionname: *const libc::c_char = 0 as *const libc::c_char;
static mut line: libc::c_uint = 0;
static mut found: bool = false;