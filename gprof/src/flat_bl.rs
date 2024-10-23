extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
#[no_mangle]
pub unsafe extern "C" fn flat_blurb(mut file: *mut FILE) {
    fputs(b"\n\0" as *const u8 as *const libc::c_char, file);
    fputs(
        b" %         the percentage of the total running time of the\n\0" as *const u8
            as *const libc::c_char,
        file,
    );
    fputs(
        b"time       program used by this function.\n\0" as *const u8
            as *const libc::c_char,
        file,
    );
    fputs(b"\n\0" as *const u8 as *const libc::c_char, file);
    fputs(
        b"cumulative a running sum of the number of seconds accounted\n\0" as *const u8
            as *const libc::c_char,
        file,
    );
    fputs(
        b" seconds   for by this function and those listed above it.\n\0" as *const u8
            as *const libc::c_char,
        file,
    );
    fputs(b"\n\0" as *const u8 as *const libc::c_char, file);
    fputs(
        b" self      the number of seconds accounted for by this\n\0" as *const u8
            as *const libc::c_char,
        file,
    );
    fputs(
        b"seconds    function alone.  This is the major sort for this\n\0" as *const u8
            as *const libc::c_char,
        file,
    );
    fputs(b"           listing.\n\0" as *const u8 as *const libc::c_char, file);
    fputs(b"\n\0" as *const u8 as *const libc::c_char, file);
    fputs(
        b"calls      the number of times this function was invoked, if\n\0" as *const u8
            as *const libc::c_char,
        file,
    );
    fputs(
        b"           this function is profiled, else blank.\n\0" as *const u8
            as *const libc::c_char,
        file,
    );
    fputs(b"\n\0" as *const u8 as *const libc::c_char, file);
    fputs(
        b" self      the average number of milliseconds spent in this\n\0" as *const u8
            as *const libc::c_char,
        file,
    );
    fputs(
        b"ms/call    function per call, if this function is profiled,\n\0" as *const u8
            as *const libc::c_char,
        file,
    );
    fputs(b"\t   else blank.\n\0" as *const u8 as *const libc::c_char, file);
    fputs(b"\n\0" as *const u8 as *const libc::c_char, file);
    fputs(
        b" total     the average number of milliseconds spent in this\n\0" as *const u8
            as *const libc::c_char,
        file,
    );
    fputs(
        b"ms/call    function and its descendents per call, if this\n\0" as *const u8
            as *const libc::c_char,
        file,
    );
    fputs(
        b"\t   function is profiled, else blank.\n\0" as *const u8
            as *const libc::c_char,
        file,
    );
    fputs(b"\n\0" as *const u8 as *const libc::c_char, file);
    fputs(
        b"name       the name of the function.  This is the minor sort\n\0" as *const u8
            as *const libc::c_char,
        file,
    );
    fputs(
        b"           for this listing. The index shows the location of\n\0" as *const u8
            as *const libc::c_char,
        file,
    );
    fputs(
        b"\t   the function in the gprof listing. If the index is\n\0" as *const u8
            as *const libc::c_char,
        file,
    );
    fputs(
        b"\t   in parenthesis it shows where it would appear in\n\0" as *const u8
            as *const libc::c_char,
        file,
    );
    fputs(
        b"\t   the gprof listing if it were to be printed.\n\0" as *const u8
            as *const libc::c_char,
        file,
    );
    fputs(b"\x0C\n\0" as *const u8 as *const libc::c_char, file);
    fputs(
        b"Copyright (C) 2012-2021 Free Software Foundation, Inc.\n\0" as *const u8
            as *const libc::c_char,
        file,
    );
    fputs(b"\n\0" as *const u8 as *const libc::c_char, file);
    fputs(
        b"Copying and distribution of this file, with or without modification,\n\0"
            as *const u8 as *const libc::c_char,
        file,
    );
    fputs(
        b"are permitted in any medium without royalty provided the copyright\n\0"
            as *const u8 as *const libc::c_char,
        file,
    );
    fputs(
        b"notice and this notice are preserved.\n\0" as *const u8 as *const libc::c_char,
        file,
    );
}
