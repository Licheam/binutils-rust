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
pub unsafe extern "C" fn bsd_callg_blurb(mut file: *mut FILE) {
    fputs(b"\n\0" as *const u8 as *const libc::c_char, file);
    fputs(b"\n\0" as *const u8 as *const libc::c_char, file);
    fputs(b"\n\0" as *const u8 as *const libc::c_char, file);
    fputs(b"call graph profile:\n\0" as *const u8 as *const libc::c_char, file);
    fputs(
        b"          The sum of self and descendents is the major sort\n\0" as *const u8
            as *const libc::c_char,
        file,
    );
    fputs(b"          for this listing.\n\0" as *const u8 as *const libc::c_char, file);
    fputs(b"\n\0" as *const u8 as *const libc::c_char, file);
    fputs(b"          function entries:\n\0" as *const u8 as *const libc::c_char, file);
    fputs(b"\n\0" as *const u8 as *const libc::c_char, file);
    fputs(
        b"index     the index of the function in the call graph\n\0" as *const u8
            as *const libc::c_char,
        file,
    );
    fputs(
        b"          listing, as an aid to locating it (see below).\n\0" as *const u8
            as *const libc::c_char,
        file,
    );
    fputs(b"\n\0" as *const u8 as *const libc::c_char, file);
    fputs(
        b"%time     the percentage of the total time of the program\n\0" as *const u8
            as *const libc::c_char,
        file,
    );
    fputs(
        b"          accounted for by this function and its\n\0" as *const u8
            as *const libc::c_char,
        file,
    );
    fputs(b"          descendents.\n\0" as *const u8 as *const libc::c_char, file);
    fputs(b"\n\0" as *const u8 as *const libc::c_char, file);
    fputs(
        b"self      the number of seconds spent in this function\n\0" as *const u8
            as *const libc::c_char,
        file,
    );
    fputs(b"          itself.\n\0" as *const u8 as *const libc::c_char, file);
    fputs(b"\n\0" as *const u8 as *const libc::c_char, file);
    fputs(b"descendents\n\0" as *const u8 as *const libc::c_char, file);
    fputs(
        b"          the number of seconds spent in the descendents of\n\0" as *const u8
            as *const libc::c_char,
        file,
    );
    fputs(
        b"          this function on behalf of this function.\n\0" as *const u8
            as *const libc::c_char,
        file,
    );
    fputs(b"\n\0" as *const u8 as *const libc::c_char, file);
    fputs(
        b"called    the number of times this function is called (other\n\0" as *const u8
            as *const libc::c_char,
        file,
    );
    fputs(
        b"          than recursive calls).\n\0" as *const u8 as *const libc::c_char,
        file,
    );
    fputs(b"\n\0" as *const u8 as *const libc::c_char, file);
    fputs(
        b"self      the number of times this function calls itself\n\0" as *const u8
            as *const libc::c_char,
        file,
    );
    fputs(b"          recursively.\n\0" as *const u8 as *const libc::c_char, file);
    fputs(b"\n\0" as *const u8 as *const libc::c_char, file);
    fputs(
        b"name      the name of the function, with an indication of\n\0" as *const u8
            as *const libc::c_char,
        file,
    );
    fputs(
        b"          its membership in a cycle, if any.\n\0" as *const u8
            as *const libc::c_char,
        file,
    );
    fputs(b"\n\0" as *const u8 as *const libc::c_char, file);
    fputs(
        b"index     the index of the function in the call graph\n\0" as *const u8
            as *const libc::c_char,
        file,
    );
    fputs(
        b"          listing, as an aid to locating it.\n\0" as *const u8
            as *const libc::c_char,
        file,
    );
    fputs(b"\n\0" as *const u8 as *const libc::c_char, file);
    fputs(b"\n\0" as *const u8 as *const libc::c_char, file);
    fputs(b"\n\0" as *const u8 as *const libc::c_char, file);
    fputs(b"          parent listings:\n\0" as *const u8 as *const libc::c_char, file);
    fputs(b"\n\0" as *const u8 as *const libc::c_char, file);
    fputs(
        b"self*     the number of seconds of this function's self time\n\0" as *const u8
            as *const libc::c_char,
        file,
    );
    fputs(
        b"          which is due to calls from this parent.\n\0" as *const u8
            as *const libc::c_char,
        file,
    );
    fputs(b"\n\0" as *const u8 as *const libc::c_char, file);
    fputs(b"descendents*\n\0" as *const u8 as *const libc::c_char, file);
    fputs(
        b"          the number of seconds of this function's\n\0" as *const u8
            as *const libc::c_char,
        file,
    );
    fputs(
        b"          descendent time which is due to calls from this\n\0" as *const u8
            as *const libc::c_char,
        file,
    );
    fputs(b"          parent.\n\0" as *const u8 as *const libc::c_char, file);
    fputs(b"\n\0" as *const u8 as *const libc::c_char, file);
    fputs(
        b"called**  the number of times this function is called by\n\0" as *const u8
            as *const libc::c_char,
        file,
    );
    fputs(
        b"          this parent.  This is the numerator of the\n\0" as *const u8
            as *const libc::c_char,
        file,
    );
    fputs(
        b"          fraction which divides up the function's time to\n\0" as *const u8
            as *const libc::c_char,
        file,
    );
    fputs(b"          its parents.\n\0" as *const u8 as *const libc::c_char, file);
    fputs(b"\n\0" as *const u8 as *const libc::c_char, file);
    fputs(
        b"total*    the number of times this function was called by\n\0" as *const u8
            as *const libc::c_char,
        file,
    );
    fputs(
        b"          all of its parents.  This is the denominator of\n\0" as *const u8
            as *const libc::c_char,
        file,
    );
    fputs(
        b"          the propagation fraction.\n\0" as *const u8 as *const libc::c_char,
        file,
    );
    fputs(b"\n\0" as *const u8 as *const libc::c_char, file);
    fputs(
        b"parents   the name of this parent, with an indication of the\n\0" as *const u8
            as *const libc::c_char,
        file,
    );
    fputs(
        b"          parent's membership in a cycle, if any.\n\0" as *const u8
            as *const libc::c_char,
        file,
    );
    fputs(b"\n\0" as *const u8 as *const libc::c_char, file);
    fputs(
        b"index     the index of this parent in the call graph\n\0" as *const u8
            as *const libc::c_char,
        file,
    );
    fputs(
        b"          listing, as an aid in locating it.\n\0" as *const u8
            as *const libc::c_char,
        file,
    );
    fputs(b"\n\0" as *const u8 as *const libc::c_char, file);
    fputs(b"\n\0" as *const u8 as *const libc::c_char, file);
    fputs(b"\n\0" as *const u8 as *const libc::c_char, file);
    fputs(b"          children listings:\n\0" as *const u8 as *const libc::c_char, file);
    fputs(b"\n\0" as *const u8 as *const libc::c_char, file);
    fputs(
        b"self*     the number of seconds of this child's self time\n\0" as *const u8
            as *const libc::c_char,
        file,
    );
    fputs(
        b"          which is due to being called by this function.\n\0" as *const u8
            as *const libc::c_char,
        file,
    );
    fputs(b"\n\0" as *const u8 as *const libc::c_char, file);
    fputs(b"descendent*\n\0" as *const u8 as *const libc::c_char, file);
    fputs(
        b"          the number of seconds of this child's descendent's\n\0" as *const u8
            as *const libc::c_char,
        file,
    );
    fputs(
        b"          time which is due to being called by this\n\0" as *const u8
            as *const libc::c_char,
        file,
    );
    fputs(b"          function.\n\0" as *const u8 as *const libc::c_char, file);
    fputs(b"\n\0" as *const u8 as *const libc::c_char, file);
    fputs(
        b"called**  the number of times this child is called by this\n\0" as *const u8
            as *const libc::c_char,
        file,
    );
    fputs(
        b"          function.  This is the numerator of the\n\0" as *const u8
            as *const libc::c_char,
        file,
    );
    fputs(
        b"          propagation fraction for this child.\n\0" as *const u8
            as *const libc::c_char,
        file,
    );
    fputs(b"\n\0" as *const u8 as *const libc::c_char, file);
    fputs(
        b"total*    the number of times this child is called by all\n\0" as *const u8
            as *const libc::c_char,
        file,
    );
    fputs(
        b"          functions.  This is the denominator of the\n\0" as *const u8
            as *const libc::c_char,
        file,
    );
    fputs(
        b"          propagation fraction.\n\0" as *const u8 as *const libc::c_char,
        file,
    );
    fputs(b"\n\0" as *const u8 as *const libc::c_char, file);
    fputs(
        b"children  the name of this child, and an indication of its\n\0" as *const u8
            as *const libc::c_char,
        file,
    );
    fputs(
        b"          membership in a cycle, if any.\n\0" as *const u8
            as *const libc::c_char,
        file,
    );
    fputs(b"\n\0" as *const u8 as *const libc::c_char, file);
    fputs(
        b"index     the index of this child in the call graph listing,\n\0" as *const u8
            as *const libc::c_char,
        file,
    );
    fputs(
        b"          as an aid to locating it.\n\0" as *const u8 as *const libc::c_char,
        file,
    );
    fputs(b"\n\0" as *const u8 as *const libc::c_char, file);
    fputs(b"\n\0" as *const u8 as *const libc::c_char, file);
    fputs(b"\n\0" as *const u8 as *const libc::c_char, file);
    fputs(
        b"          * these fields are omitted for parents (or\n\0" as *const u8
            as *const libc::c_char,
        file,
    );
    fputs(
        b"          children) in the same cycle as the function.  If\n\0" as *const u8
            as *const libc::c_char,
        file,
    );
    fputs(
        b"          the function (or child) is a member of a cycle,\n\0" as *const u8
            as *const libc::c_char,
        file,
    );
    fputs(
        b"          the propagated times and propagation denominator\n\0" as *const u8
            as *const libc::c_char,
        file,
    );
    fputs(
        b"          represent the self time and descendent time of the\n\0" as *const u8
            as *const libc::c_char,
        file,
    );
    fputs(b"          cycle as a whole.\n\0" as *const u8 as *const libc::c_char, file);
    fputs(b"\n\0" as *const u8 as *const libc::c_char, file);
    fputs(
        b"          ** static-only parents and children are indicated\n\0" as *const u8
            as *const libc::c_char,
        file,
    );
    fputs(
        b"          by a call count of 0.\n\0" as *const u8 as *const libc::c_char,
        file,
    );
    fputs(b"\n\0" as *const u8 as *const libc::c_char, file);
    fputs(b"\n\0" as *const u8 as *const libc::c_char, file);
    fputs(b"\n\0" as *const u8 as *const libc::c_char, file);
    fputs(b"          cycle listings:\n\0" as *const u8 as *const libc::c_char, file);
    fputs(
        b"          the cycle as a whole is listed with the same\n\0" as *const u8
            as *const libc::c_char,
        file,
    );
    fputs(
        b"          fields as a function entry.  Below it are listed\n\0" as *const u8
            as *const libc::c_char,
        file,
    );
    fputs(
        b"          the members of the cycle, and their contributions\n\0" as *const u8
            as *const libc::c_char,
        file,
    );
    fputs(
        b"          to the time and call counts of the cycle.\n\0" as *const u8
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
