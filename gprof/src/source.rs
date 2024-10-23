extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn perror(__s: *const libc::c_char);
    fn free(_: *mut libc::c_void);
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    static mut stderr: *mut FILE;
    static mut stdout: *mut FILE;
    fn __errno_location() -> *mut libc::c_int;
    fn filename_cmp(s1: *const libc::c_char, s2: *const libc::c_char) -> libc::c_int;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    static mut whoami: *const libc::c_char;
    static mut debug_level: libc::c_int;
    static mut first_output: bool;
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct search_list_elem {
    pub next: *mut search_list_elem,
    pub path: [libc::c_char; 1],
}
pub type Search_List_Elem = search_list_elem;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Search_List {
    pub head: *mut search_list_elem,
    pub tail: *mut search_list_elem,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct source_file {
    pub next: *mut source_file,
    pub name: *const libc::c_char,
    pub ncalls: libc::c_ulong,
    pub num_lines: libc::c_int,
    pub nalloced: libc::c_int,
    pub line: *mut *mut libc::c_void,
}
pub type Source_File = source_file;
#[no_mangle]
pub static mut create_annotation_files: bool = 0 as libc::c_int != 0;
#[no_mangle]
pub static mut src_search_list: Search_List = {
    let mut init = Search_List {
        head: 0 as *const search_list_elem as *mut search_list_elem,
        tail: 0 as *const search_list_elem as *mut search_list_elem,
    };
    init
};
#[no_mangle]
pub static mut first_src_file: *mut Source_File = 0 as *const Source_File
    as *mut Source_File;
#[no_mangle]
pub unsafe extern "C" fn source_file_lookup_path(
    mut path: *const libc::c_char,
) -> *mut Source_File {
    let mut sf: *mut Source_File = 0 as *mut Source_File;
    sf = first_src_file;
    while !sf.is_null() {
        if filename_cmp(path, (*sf).name) == 0 as libc::c_int {
            break;
        }
        sf = (*sf).next;
    }
    if sf.is_null() {
        sf = xmalloc(::core::mem::size_of::<Source_File>() as libc::c_ulong)
            as *mut Source_File;
        memset(
            sf as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<Source_File>() as libc::c_ulong,
        );
        (*sf).name = xstrdup(path);
        (*sf).next = first_src_file;
        first_src_file = sf;
    }
    return sf;
}
#[no_mangle]
pub unsafe extern "C" fn source_file_lookup_name(
    mut filename: *const libc::c_char,
) -> *mut Source_File {
    let mut fname: *const libc::c_char = 0 as *const libc::c_char;
    let mut sf: *mut Source_File = 0 as *mut Source_File;
    sf = first_src_file;
    while !sf.is_null() {
        fname = strrchr((*sf).name, '/' as i32);
        if !fname.is_null() {
            fname = fname.offset(1);
            fname;
        } else {
            fname = (*sf).name;
        }
        if filename_cmp(filename, fname) == 0 as libc::c_int {
            break;
        }
        sf = (*sf).next;
    }
    return sf;
}
#[no_mangle]
pub unsafe extern "C" fn annotate_source(
    mut sf: *mut Source_File,
    mut max_width: libc::c_uint,
    mut annote: Option::<
        unsafe extern "C" fn(
            *mut libc::c_char,
            libc::c_uint,
            libc::c_int,
            *mut libc::c_void,
        ) -> (),
    >,
    mut arg: *mut libc::c_void,
) -> *mut FILE {
    static mut first_file: bool = 1 as libc::c_int != 0;
    let mut i: libc::c_int = 0;
    let mut line_num: libc::c_int = 0;
    let mut nread: libc::c_int = 0;
    let mut new_line: bool = false;
    let mut buf: [libc::c_char; 8192] = [0; 8192];
    let mut fname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut annotation: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut name_only: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ifp: *mut FILE = 0 as *mut FILE;
    let mut ofp: *mut FILE = 0 as *mut FILE;
    let mut sle: *mut Search_List_Elem = src_search_list.head;
    fname = (*sf).name as *mut libc::c_char;
    if *((*sf).name).offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32
        || *((*sf).name).offset(0 as libc::c_int as isize) as libc::c_int == '\\' as i32
            && 0 as libc::c_int != 0
        || *((*sf).name).offset(0 as libc::c_int as isize) as libc::c_int != 0
            && *((*sf).name).offset(1 as libc::c_int as isize) as libc::c_int
                == ':' as i32 && 0 as libc::c_int != 0
    {
        sle = 0 as *mut Search_List_Elem;
    }
    name_only = 0 as *mut libc::c_char;
    loop {
        if debug_level & (1 as libc::c_int) << 13 as libc::c_int != 0 {
            printf(
                b"[annotate_source]: looking for %s, trying %s\n\0" as *const u8
                    as *const libc::c_char,
                (*sf).name,
                fname,
            );
        }
        ifp = fopen(fname, b"r\0" as *const u8 as *const libc::c_char);
        if fname != (*sf).name as *mut libc::c_char {
            free(fname as *mut libc::c_void);
        }
        if !ifp.is_null() {
            break;
        }
        if sle.is_null() && name_only.is_null() {
            name_only = strrchr((*sf).name, '/' as i32);
            if !name_only.is_null() {
                name_only = name_only.offset(1);
                name_only;
                sle = src_search_list.head;
            }
        }
        if !sle.is_null() {
            fname = xmalloc(
                (strlen(((*sle).path).as_mut_ptr()))
                    .wrapping_add(3 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        strlen(
                            (if !name_only.is_null() {
                                name_only as *const libc::c_char
                            } else {
                                (*sf).name
                            }),
                        ),
                    ),
            ) as *mut libc::c_char;
            strcpy(fname, ((*sle).path).as_mut_ptr());
            strcat(fname, b"/\0" as *const u8 as *const libc::c_char);
            if !name_only.is_null() {
                strcat(fname, name_only);
            } else {
                strcat(fname, (*sf).name);
            }
            sle = (*sle).next;
        } else {
            if *__errno_location() == 2 as libc::c_int {
                fprintf(
                    stderr,
                    dcgettext(
                        b"gprof\0" as *const u8 as *const libc::c_char,
                        b"%s: could not locate `%s'\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    whoami,
                    (*sf).name,
                );
            } else {
                perror((*sf).name);
            }
            return 0 as *mut FILE;
        }
    }
    ofp = stdout;
    if create_annotation_files {
        let mut filename: *const libc::c_char = 0 as *const libc::c_char;
        filename = strrchr((*sf).name, '/' as i32);
        if !filename.is_null() {
            filename = filename.offset(1);
            filename;
        } else {
            filename = (*sf).name;
        }
        fname = xmalloc(
            (strlen(filename))
                .wrapping_add(strlen(b"-ann\0" as *const u8 as *const libc::c_char))
                .wrapping_add(1 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
        strcpy(fname, filename);
        strcat(fname, b"-ann\0" as *const u8 as *const libc::c_char);
        ofp = fopen(fname, b"w\0" as *const u8 as *const libc::c_char);
        if ofp.is_null() {
            perror(fname);
            free(fname as *mut libc::c_void);
            return 0 as *mut FILE;
        }
        free(fname as *mut libc::c_void);
    }
    if ofp == stdout {
        if first_file {
            first_file = 0 as libc::c_int != 0;
        } else {
            fputc('\n' as i32, ofp);
        }
        if first_output {
            first_output = 0 as libc::c_int != 0;
        } else {
            fprintf(ofp, b"\x0C\n\0" as *const u8 as *const libc::c_char);
        }
        fprintf(
            ofp,
            dcgettext(
                b"gprof\0" as *const u8 as *const libc::c_char,
                b"*** File %s:\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            (*sf).name,
        );
    }
    annotation = xmalloc(
        max_width.wrapping_add(1 as libc::c_int as libc::c_uint) as size_t,
    ) as *mut libc::c_char;
    line_num = 1 as libc::c_int;
    new_line = 1 as libc::c_int != 0;
    loop {
        nread = fread(
            buf.as_mut_ptr() as *mut libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            ::core::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong,
            ifp,
        ) as libc::c_int;
        if !(nread > 0 as libc::c_int) {
            break;
        }
        i = 0 as libc::c_int;
        while i < nread {
            if new_line {
                (Some(annote.expect("non-null function pointer")))
                    .expect(
                        "non-null function pointer",
                    )(annotation, max_width, line_num, arg);
                fputs(annotation, ofp);
                line_num += 1;
                line_num;
            }
            new_line = buf[i as usize] as libc::c_int == '\n' as i32;
            fputc(buf[i as usize] as libc::c_int, ofp);
            i += 1;
            i;
        }
    }
    free(annotation as *mut libc::c_void);
    fclose(ifp);
    return ofp;
}
