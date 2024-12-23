use ::libc;
extern "C" {
    fn bfd_default_compatible(
        a: *const bfd_arch_info_type,
        b: *const bfd_arch_info_type,
    ) -> *const bfd_arch_info_type;
    fn bfd_default_scan(info: *const bfd_arch_info, string: *const libc::c_char) -> bool;
    fn bfd_arch_i386_short_nop_fill(
        _: bfd_size_type,
        _: bool,
        _: bool,
    ) -> *mut libc::c_void;
}
pub type bfd_size_type = libc::c_ulong;
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
pub type bfd_arch_info_type = bfd_arch_info;
static mut bfd_iamcu_arch_intel_syntax: bfd_arch_info_type = unsafe {
    {
        let mut init = bfd_arch_info {
            bits_per_word: 32 as libc::c_int,
            bits_per_address: 32 as libc::c_int,
            bits_per_byte: 8 as libc::c_int,
            arch: bfd_arch_iamcu,
            mach: ((1 as libc::c_int) << 2 as libc::c_int
                | (1 as libc::c_int) << 8 as libc::c_int
                | (1 as libc::c_int) << 0 as libc::c_int) as libc::c_ulong,
            arch_name: b"iamcu:intel\0" as *const u8 as *const libc::c_char,
            printable_name: b"iamcu:intel\0" as *const u8 as *const libc::c_char,
            section_align_power: 3 as libc::c_int as libc::c_uint,
            the_default: 1 as libc::c_int != 0,
            compatible: Some(
                bfd_default_compatible
                    as unsafe extern "C" fn(
                        *const bfd_arch_info_type,
                        *const bfd_arch_info_type,
                    ) -> *const bfd_arch_info_type,
            ),
            scan: Some(
                bfd_default_scan
                    as unsafe extern "C" fn(
                        *const bfd_arch_info,
                        *const libc::c_char,
                    ) -> bool,
            ),
            fill: Some(
                bfd_arch_i386_short_nop_fill
                    as unsafe extern "C" fn(
                        bfd_size_type,
                        bool,
                        bool,
                    ) -> *mut libc::c_void,
            ),
            next: 0 as *const bfd_arch_info,
            max_reloc_offset_into_insn: 0 as libc::c_int,
        };
        init
    }
};
#[no_mangle]
pub static mut bfd_iamcu_arch: bfd_arch_info_type = unsafe {
    {
        let mut init = bfd_arch_info {
            bits_per_word: 32 as libc::c_int,
            bits_per_address: 32 as libc::c_int,
            bits_per_byte: 8 as libc::c_int,
            arch: bfd_arch_iamcu,
            mach: ((1 as libc::c_int) << 2 as libc::c_int
                | (1 as libc::c_int) << 8 as libc::c_int) as libc::c_ulong,
            arch_name: b"iamcu\0" as *const u8 as *const libc::c_char,
            printable_name: b"iamcu\0" as *const u8 as *const libc::c_char,
            section_align_power: 3 as libc::c_int as libc::c_uint,
            the_default: 1 as libc::c_int != 0,
            compatible: Some(
                bfd_default_compatible
                    as unsafe extern "C" fn(
                        *const bfd_arch_info_type,
                        *const bfd_arch_info_type,
                    ) -> *const bfd_arch_info_type,
            ),
            scan: Some(
                bfd_default_scan
                    as unsafe extern "C" fn(
                        *const bfd_arch_info,
                        *const libc::c_char,
                    ) -> bool,
            ),
            fill: Some(
                bfd_arch_i386_short_nop_fill
                    as unsafe extern "C" fn(
                        bfd_size_type,
                        bool,
                        bool,
                    ) -> *mut libc::c_void,
            ),
            next: &bfd_iamcu_arch_intel_syntax as *const bfd_arch_info_type,
            max_reloc_offset_into_insn: 0 as libc::c_int,
        };
        init
    }
};
