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
pub mod ctf_archive;
pub mod ctf_create;
pub mod ctf_decl;
pub mod ctf_dedup;
pub mod ctf_dump;
pub mod ctf_error;
pub mod ctf_hash;
pub mod ctf_labels;
pub mod ctf_link;
pub mod ctf_lookup;
pub mod ctf_open;
pub mod ctf_open_bfd;
pub mod ctf_serialize;
pub mod ctf_sha1;
pub mod ctf_string;
pub mod ctf_subr;
pub mod ctf_types;
pub mod ctf_util;
} // mod src
