#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(c_variadic)]
#![feature(extern_types)]

#[macro_use]
extern crate c2rust_bitfields;
extern crate libc;
pub mod src {
pub mod alloca;
pub mod argv;
pub mod bsearch_r;
pub mod choose_temp;
pub mod concat;
pub mod cp_demangle;
pub mod cp_demint;
pub mod cplus_dem;
pub mod crc32;
pub mod d_demangle;
pub mod dwarfnames;
pub mod dyn_string;
pub mod fdmatch;
pub mod fibheap;
pub mod filedescriptor;
pub mod filename_cmp;
pub mod floatformat;
pub mod fnmatch;
pub mod fopen_unlocked;
pub mod getopt;
pub mod getopt1;
pub mod getpwd;
pub mod getruntime;
pub mod hashtab;
pub mod hex;
pub mod lbasename;
pub mod lrealpath;
pub mod make_relative_prefix;
pub mod make_temp_file;
pub mod md5;
pub mod objalloc;
pub mod obstack;
pub mod partition;
pub mod pex_common;
pub mod pex_one;
pub mod pex_unix;
pub mod pexecute;
pub mod physmem;
pub mod regex;
pub mod rust_demangle;
pub mod safe_ctype;
pub mod setproctitle;
pub mod sha1;
pub mod simple_object;
pub mod simple_object_coff;
pub mod simple_object_elf;
pub mod simple_object_mach_o;
pub mod simple_object_xcoff;
pub mod sort;
pub mod spaces;
pub mod splay_tree;
pub mod stack_limit;
pub mod strerror;
pub mod strsignal;
pub mod timeval_utils;
pub mod unlink_if_ordinary;
pub mod vprintf_support;
pub mod xasprintf;
pub mod xatexit;
pub mod xexit;
pub mod xmalloc;
pub mod xmemdup;
pub mod xstrdup;
pub mod xstrerror;
pub mod xstrndup;
pub mod xvasprintf;
} // mod src
