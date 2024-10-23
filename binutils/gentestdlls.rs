#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(extern_types)]


extern crate bfd_sys;
extern crate libctf_sys;
extern crate libiberty_sys;
extern crate zlib_sys;
extern crate libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn exit(_: libc::c_int) -> !;
    fn chdir(__path: *const libc::c_char) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __uint16_t = libc::c_ushort;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type uint16_t = __uint16_t;
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
unsafe extern "C" fn write_dos_header_and_stub(mut file: *mut FILE) {
    let mut buffer: [libc::c_char; 128] = [
        0x4d as libc::c_int as libc::c_char,
        0x5a as libc::c_int as libc::c_char,
        0x90 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0x3 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0x4 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0xff as libc::c_int as libc::c_char,
        0xff as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0xb8 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0x40 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0x80 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0xe as libc::c_int as libc::c_char,
        0x1f as libc::c_int as libc::c_char,
        0xba as libc::c_int as libc::c_char,
        0xe as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0xb4 as libc::c_int as libc::c_char,
        0x9 as libc::c_int as libc::c_char,
        0xcd as libc::c_int as libc::c_char,
        0x21 as libc::c_int as libc::c_char,
        0xb8 as libc::c_int as libc::c_char,
        0x1 as libc::c_int as libc::c_char,
        0x4c as libc::c_int as libc::c_char,
        0xcd as libc::c_int as libc::c_char,
        0x21 as libc::c_int as libc::c_char,
        0x54 as libc::c_int as libc::c_char,
        0x68 as libc::c_int as libc::c_char,
        0x69 as libc::c_int as libc::c_char,
        0x73 as libc::c_int as libc::c_char,
        0x20 as libc::c_int as libc::c_char,
        0x70 as libc::c_int as libc::c_char,
        0x72 as libc::c_int as libc::c_char,
        0x6f as libc::c_int as libc::c_char,
        0x67 as libc::c_int as libc::c_char,
        0x72 as libc::c_int as libc::c_char,
        0x61 as libc::c_int as libc::c_char,
        0x6d as libc::c_int as libc::c_char,
        0x20 as libc::c_int as libc::c_char,
        0x63 as libc::c_int as libc::c_char,
        0x61 as libc::c_int as libc::c_char,
        0x6e as libc::c_int as libc::c_char,
        0x6e as libc::c_int as libc::c_char,
        0x6f as libc::c_int as libc::c_char,
        0x74 as libc::c_int as libc::c_char,
        0x20 as libc::c_int as libc::c_char,
        0x62 as libc::c_int as libc::c_char,
        0x65 as libc::c_int as libc::c_char,
        0x20 as libc::c_int as libc::c_char,
        0x72 as libc::c_int as libc::c_char,
        0x75 as libc::c_int as libc::c_char,
        0x6e as libc::c_int as libc::c_char,
        0x20 as libc::c_int as libc::c_char,
        0x69 as libc::c_int as libc::c_char,
        0x6e as libc::c_int as libc::c_char,
        0x20 as libc::c_int as libc::c_char,
        0x44 as libc::c_int as libc::c_char,
        0x4f as libc::c_int as libc::c_char,
        0x53 as libc::c_int as libc::c_char,
        0x20 as libc::c_int as libc::c_char,
        0x6d as libc::c_int as libc::c_char,
        0x6f as libc::c_int as libc::c_char,
        0x64 as libc::c_int as libc::c_char,
        0x65 as libc::c_int as libc::c_char,
        0x2e as libc::c_int as libc::c_char,
        0xd as libc::c_int as libc::c_char,
        0xd as libc::c_int as libc::c_char,
        0xa as libc::c_int as libc::c_char,
        0x24 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
    ];
    fwrite(
        buffer.as_mut_ptr() as *const libc::c_void,
        1 as libc::c_int as libc::c_ulong,
        128 as libc::c_int as libc::c_ulong,
        file,
    );
}
unsafe extern "C" fn write_pe_signature(mut file: *mut FILE) {
    let mut buffer: [libc::c_char; 4] = [0; 4];
    buffer[0 as libc::c_int as usize] = 'P' as i32 as libc::c_char;
    buffer[1 as libc::c_int as usize] = 'E' as i32 as libc::c_char;
    buffer[2 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    buffer[3 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    fwrite(
        buffer.as_mut_ptr() as *const libc::c_void,
        1 as libc::c_int as libc::c_ulong,
        4 as libc::c_int as libc::c_ulong,
        file,
    );
}
unsafe extern "C" fn write_coff_header(mut file: *mut FILE, mut machine: uint16_t) {
    let mut buffer: [libc::c_char; 128] = [0; 128];
    memset(
        buffer.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
    );
    buffer[0 as libc::c_int
        as usize] = (machine as libc::c_int & 0xff as libc::c_int) as libc::c_char;
    buffer[1 as libc::c_int
        as usize] = (machine as libc::c_int >> 0x8 as libc::c_int) as libc::c_char;
    fwrite(
        buffer.as_mut_ptr() as *const libc::c_void,
        2 as libc::c_int as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        file,
    );
    memset(
        buffer.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
    );
    fwrite(
        buffer.as_mut_ptr() as *const libc::c_void,
        2 as libc::c_int as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        file,
    );
    fwrite(
        buffer.as_mut_ptr() as *const libc::c_void,
        4 as libc::c_int as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        file,
    );
    fwrite(
        buffer.as_mut_ptr() as *const libc::c_void,
        4 as libc::c_int as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        file,
    );
    fwrite(
        buffer.as_mut_ptr() as *const libc::c_void,
        4 as libc::c_int as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        file,
    );
    fwrite(
        buffer.as_mut_ptr() as *const libc::c_void,
        2 as libc::c_int as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        file,
    );
    buffer[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    buffer[1 as libc::c_int as usize] = 0x20 as libc::c_int as libc::c_char;
    fwrite(
        buffer.as_mut_ptr() as *const libc::c_void,
        2 as libc::c_int as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        file,
    );
    memset(
        buffer.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
    );
}
unsafe extern "C" fn write_simple_dll(
    mut name: *const libc::c_char,
    mut machine: uint16_t,
) {
    let mut file: *mut FILE = fopen(name, b"w\0" as *const u8 as *const libc::c_char);
    if file.is_null() {
        fprintf(
            stderr,
            b"error: unable to open file for writing\n\0" as *const u8
                as *const libc::c_char,
        );
        exit(3 as libc::c_int);
    }
    write_dos_header_and_stub(file);
    write_pe_signature(file);
    write_coff_header(file, machine);
    fclose(file);
    file = 0 as *mut FILE;
    printf(b"wrote %s\n\0" as *const u8 as *const libc::c_char, name);
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut program_name: *mut libc::c_char = *argv.offset(0 as libc::c_int as isize);
    let mut output_directory: *mut libc::c_char = *argv
        .offset(1 as libc::c_int as isize);
    let mut i: libc::c_int = 0;
    if argc < 3 as libc::c_int {
        fprintf(
            stderr,
            b"usage: %s output-directory format [format ...] \n\n\0" as *const u8
                as *const libc::c_char,
            program_name,
        );
        fprintf(
            stderr,
            b"format is an objdump-style format string, like pei-i386\n\0" as *const u8
                as *const libc::c_char,
        );
        exit(2 as libc::c_int);
    }
    if chdir(output_directory) != 0 as libc::c_int {
        fprintf(
            stderr,
            b"error: unable to change directory to %s\n\0" as *const u8
                as *const libc::c_char,
            output_directory,
        );
        exit(2 as libc::c_int);
    }
    i = 2 as libc::c_int;
    while i < argc {
        let mut wanted_format: *mut libc::c_char = *argv.offset(i as isize);
        if strcmp(b"pei-i386\0" as *const u8 as *const libc::c_char, wanted_format)
            == 0 as libc::c_int
        {
            write_simple_dll(
                b"simple-pei-i386.dll\0" as *const u8 as *const libc::c_char,
                0x14c as libc::c_int as uint16_t,
            );
            write_simple_dll(
                b"linux-pei-i386.dll\0" as *const u8 as *const libc::c_char,
                (0x14c as libc::c_int ^ 0x7b79 as libc::c_int) as uint16_t,
            );
        } else if strcmp(
            b"pei-x86-64\0" as *const u8 as *const libc::c_char,
            wanted_format,
        ) == 0 as libc::c_int
        {
            write_simple_dll(
                b"simple-pei-x86-64.dll\0" as *const u8 as *const libc::c_char,
                0x8664 as libc::c_int as uint16_t,
            );
            write_simple_dll(
                b"linux-pei-x86-64.dll\0" as *const u8 as *const libc::c_char,
                (0x8664 as libc::c_int ^ 0x7b79 as libc::c_int) as uint16_t,
            );
        } else {
            fprintf(
                stderr,
                b"error: can't handle format %s\n\0" as *const u8 as *const libc::c_char,
                wanted_format,
            );
            exit(2 as libc::c_int);
        }
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}
