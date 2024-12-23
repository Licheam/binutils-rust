#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(c_variadic)]
#![feature(extern_types)]


extern crate libc;
pub mod src {
pub mod adler32;
pub mod compress;
pub mod crc32;
pub mod deflate;
pub mod gzclose;
pub mod gzlib;
pub mod gzread;
pub mod gzwrite;
pub mod infback;
pub mod inffast;
pub mod inflate;
pub mod inftrees;
pub mod trees;
pub mod uncompr;
pub mod zutil;
} // mod src
