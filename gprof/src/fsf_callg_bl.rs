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
pub unsafe extern "C" fn fsf_callg_blurb(mut file: *mut FILE) {
    fputs(b"\n\0" as *const u8 as *const libc::c_char, file);
    fputs(
        b" This table describes the call tree of the program, and was sorted by\n\0"
            as *const u8 as *const libc::c_char,
        file,
    );
    fputs(
        b" the total amount of time spent in each function and its children.\n\0"
            as *const u8 as *const libc::c_char,
        file,
    );
    fputs(b"\n\0" as *const u8 as *const libc::c_char, file);
    fputs(
        b" Each entry in this table consists of several lines.  The line with the\n\0"
            as *const u8 as *const libc::c_char,
        file,
    );
    fputs(
        b" index number at the left hand margin lists the current function.\n\0"
            as *const u8 as *const libc::c_char,
        file,
    );
    fputs(
        b" The lines above it list the functions that called this function,\n\0"
            as *const u8 as *const libc::c_char,
        file,
    );
    fputs(
        b" and the lines below it list the functions this one called.\n\0" as *const u8
            as *const libc::c_char,
        file,
    );
    fputs(b" This line lists:\n\0" as *const u8 as *const libc::c_char, file);
    fputs(
        b"     index\tA unique number given to each element of the table.\n\0"
            as *const u8 as *const libc::c_char,
        file,
    );
    fputs(
        b"\t\tIndex numbers are sorted numerically.\n\0" as *const u8
            as *const libc::c_char,
        file,
    );
    fputs(
        b"\t\tThe index number is printed next to every function name so\n\0"
            as *const u8 as *const libc::c_char,
        file,
    );
    fputs(
        b"\t\tit is easier to look up where the function is in the table.\n\0"
            as *const u8 as *const libc::c_char,
        file,
    );
    fputs(b"\n\0" as *const u8 as *const libc::c_char, file);
    fputs(
        b"     % time\tThis is the percentage of the `total' time that was spent\n\0"
            as *const u8 as *const libc::c_char,
        file,
    );
    fputs(
        b"\t\tin this function and its children.  Note that due to\n\0" as *const u8
            as *const libc::c_char,
        file,
    );
    fputs(
        b"\t\tdifferent viewpoints, functions excluded by options, etc,\n\0" as *const u8
            as *const libc::c_char,
        file,
    );
    fputs(
        b"\t\tthese numbers will NOT add up to 100%.\n\0" as *const u8
            as *const libc::c_char,
        file,
    );
    fputs(b"\n\0" as *const u8 as *const libc::c_char, file);
    fputs(
        b"     self\tThis is the total amount of time spent in this function.\n\0"
            as *const u8 as *const libc::c_char,
        file,
    );
    fputs(b"\n\0" as *const u8 as *const libc::c_char, file);
    fputs(
        b"     children\tThis is the total amount of time propagated into this\n\0"
            as *const u8 as *const libc::c_char,
        file,
    );
    fputs(
        b"\t\tfunction by its children.\n\0" as *const u8 as *const libc::c_char,
        file,
    );
    fputs(b"\n\0" as *const u8 as *const libc::c_char, file);
    fputs(
        b"     called\tThis is the number of times the function was called.\n\0"
            as *const u8 as *const libc::c_char,
        file,
    );
    fputs(
        b"\t\tIf the function called itself recursively, the number\n\0" as *const u8
            as *const libc::c_char,
        file,
    );
    fputs(
        b"\t\tonly includes non-recursive calls, and is followed by\n\0" as *const u8
            as *const libc::c_char,
        file,
    );
    fputs(
        b"\t\ta `+' and the number of recursive calls.\n\0" as *const u8
            as *const libc::c_char,
        file,
    );
    fputs(b"\n\0" as *const u8 as *const libc::c_char, file);
    fputs(
        b"     name\tThe name of the current function.  The index number is\n\0"
            as *const u8 as *const libc::c_char,
        file,
    );
    fputs(
        b"\t\tprinted after it.  If the function is a member of a\n\0" as *const u8
            as *const libc::c_char,
        file,
    );
    fputs(
        b"\t\tcycle, the cycle number is printed between the\n\0" as *const u8
            as *const libc::c_char,
        file,
    );
    fputs(
        b"\t\tfunction's name and the index number.\n\0" as *const u8
            as *const libc::c_char,
        file,
    );
    fputs(b"\n\0" as *const u8 as *const libc::c_char, file);
    fputs(b"\n\0" as *const u8 as *const libc::c_char, file);
    fputs(
        b" For the function's parents, the fields have the following meanings:\n\0"
            as *const u8 as *const libc::c_char,
        file,
    );
    fputs(b"\n\0" as *const u8 as *const libc::c_char, file);
    fputs(
        b"     self\tThis is the amount of time that was propagated directly\n\0"
            as *const u8 as *const libc::c_char,
        file,
    );
    fputs(
        b"\t\tfrom the function into this parent.\n\0" as *const u8
            as *const libc::c_char,
        file,
    );
    fputs(b"\n\0" as *const u8 as *const libc::c_char, file);
    fputs(
        b"     children\tThis is the amount of time that was propagated from\n\0"
            as *const u8 as *const libc::c_char,
        file,
    );
    fputs(
        b"\t\tthe function's children into this parent.\n\0" as *const u8
            as *const libc::c_char,
        file,
    );
    fputs(b"\n\0" as *const u8 as *const libc::c_char, file);
    fputs(
        b"     called\tThis is the number of times this parent called the\n\0"
            as *const u8 as *const libc::c_char,
        file,
    );
    fputs(
        b"\t\tfunction `/' the total number of times the function\n\0" as *const u8
            as *const libc::c_char,
        file,
    );
    fputs(
        b"\t\twas called.  Recursive calls to the function are not\n\0" as *const u8
            as *const libc::c_char,
        file,
    );
    fputs(
        b"\t\tincluded in the number after the `/'.\n\0" as *const u8
            as *const libc::c_char,
        file,
    );
    fputs(b"\n\0" as *const u8 as *const libc::c_char, file);
    fputs(
        b"     name\tThis is the name of the parent.  The parent's index\n\0"
            as *const u8 as *const libc::c_char,
        file,
    );
    fputs(
        b"\t\tnumber is printed after it.  If the parent is a\n\0" as *const u8
            as *const libc::c_char,
        file,
    );
    fputs(
        b"\t\tmember of a cycle, the cycle number is printed between\n\0" as *const u8
            as *const libc::c_char,
        file,
    );
    fputs(
        b"\t\tthe name and the index number.\n\0" as *const u8 as *const libc::c_char,
        file,
    );
    fputs(b"\n\0" as *const u8 as *const libc::c_char, file);
    fputs(
        b" If the parents of the function cannot be determined, the word\n\0"
            as *const u8 as *const libc::c_char,
        file,
    );
    fputs(
        b" `<spontaneous>' is printed in the `name' field, and all the other\n\0"
            as *const u8 as *const libc::c_char,
        file,
    );
    fputs(b" fields are blank.\n\0" as *const u8 as *const libc::c_char, file);
    fputs(b"\n\0" as *const u8 as *const libc::c_char, file);
    fputs(
        b" For the function's children, the fields have the following meanings:\n\0"
            as *const u8 as *const libc::c_char,
        file,
    );
    fputs(b"\n\0" as *const u8 as *const libc::c_char, file);
    fputs(
        b"     self\tThis is the amount of time that was propagated directly\n\0"
            as *const u8 as *const libc::c_char,
        file,
    );
    fputs(
        b"\t\tfrom the child into the function.\n\0" as *const u8 as *const libc::c_char,
        file,
    );
    fputs(b"\n\0" as *const u8 as *const libc::c_char, file);
    fputs(
        b"     children\tThis is the amount of time that was propagated from the\n\0"
            as *const u8 as *const libc::c_char,
        file,
    );
    fputs(
        b"\t\tchild's children to the function.\n\0" as *const u8 as *const libc::c_char,
        file,
    );
    fputs(b"\n\0" as *const u8 as *const libc::c_char, file);
    fputs(
        b"     called\tThis is the number of times the function called\n\0" as *const u8
            as *const libc::c_char,
        file,
    );
    fputs(
        b"\t\tthis child `/' the total number of times the child\n\0" as *const u8
            as *const libc::c_char,
        file,
    );
    fputs(
        b"\t\twas called.  Recursive calls by the child are not\n\0" as *const u8
            as *const libc::c_char,
        file,
    );
    fputs(
        b"\t\tlisted in the number after the `/'.\n\0" as *const u8
            as *const libc::c_char,
        file,
    );
    fputs(b"\n\0" as *const u8 as *const libc::c_char, file);
    fputs(
        b"     name\tThis is the name of the child.  The child's index\n\0" as *const u8
            as *const libc::c_char,
        file,
    );
    fputs(
        b"\t\tnumber is printed after it.  If the child is a\n\0" as *const u8
            as *const libc::c_char,
        file,
    );
    fputs(
        b"\t\tmember of a cycle, the cycle number is printed\n\0" as *const u8
            as *const libc::c_char,
        file,
    );
    fputs(
        b"\t\tbetween the name and the index number.\n\0" as *const u8
            as *const libc::c_char,
        file,
    );
    fputs(b"\n\0" as *const u8 as *const libc::c_char, file);
    fputs(
        b" If there are any cycles (circles) in the call graph, there is an\n\0"
            as *const u8 as *const libc::c_char,
        file,
    );
    fputs(
        b" entry for the cycle-as-a-whole.  This entry shows who called the\n\0"
            as *const u8 as *const libc::c_char,
        file,
    );
    fputs(
        b" cycle (as parents) and the members of the cycle (as children.)\n\0"
            as *const u8 as *const libc::c_char,
        file,
    );
    fputs(
        b" The `+' recursive calls entry shows the number of function calls that\n\0"
            as *const u8 as *const libc::c_char,
        file,
    );
    fputs(
        b" were internal to the cycle, and the calls entry for each member shows,\n\0"
            as *const u8 as *const libc::c_char,
        file,
    );
    fputs(
        b" for that member, how many times it was called from other members of\n\0"
            as *const u8 as *const libc::c_char,
        file,
    );
    fputs(b" the cycle.\n\0" as *const u8 as *const libc::c_char, file);
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
