#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(c_variadic)]
#![feature(extern_types)]
#![feature(label_break_value)]

#[macro_use]
extern crate c2rust_bitfields;
extern crate libc;
pub mod src {
pub mod archive;
pub mod archive64;
pub mod archures;
pub mod bfd;
pub mod bfdio;
pub mod bfdwin;
pub mod binary;
pub mod cache;
pub mod coff_bfd;
pub mod coffgen;
pub mod cofflink;
pub mod compress;
pub mod corefile;
pub mod cpu_i386;
pub mod cpu_iamcu;
pub mod cpu_k1om;
pub mod cpu_l1om;
pub mod dwarf1;
pub mod dwarf2;
pub mod elf;
pub mod elf32;
pub mod elf32_gen;
pub mod elf32_i386;
pub mod elf64;
pub mod elf64_gen;
pub mod elf64_x86_64;
pub mod elf_attrs;
pub mod elf_eh_frame;
pub mod elf_ifunc;
pub mod elf_properties;
pub mod elf_strtab;
pub mod elf_vxworks;
pub mod elflink;
pub mod elfxx_x86;
pub mod format;
pub mod hash;
pub mod ihex;
pub mod init;
pub mod libbfd;
pub mod linker;
pub mod merge;
pub mod opncls;
pub mod pe_i386;
pub mod pe_x86_64;
pub mod pei_i386;
pub mod pei_x86_64;
pub mod peigen;
pub mod pex64igen;
pub mod plugin;
pub mod reloc;
pub mod section;
pub mod simple;
pub mod srec;
pub mod stab_syms;
pub mod stabs;
pub mod syms;
pub mod targets;
pub mod tekhex;
pub mod verilog;
} // mod src
