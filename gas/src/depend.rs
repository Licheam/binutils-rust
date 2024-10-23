extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    static mut out_file_name: *const libc::c_char;
    fn as_warn(format: *const libc::c_char, _: ...);
    fn filename_cmp(s1: *const libc::c_char, s2: *const libc::c_char) -> libc::c_int;
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
pub struct dependency {
    pub file: *mut libc::c_char,
    pub next: *mut dependency,
}
#[no_mangle]
pub unsafe extern "C" fn start_dependencies(mut filename: *mut libc::c_char) {
    dep_file = filename;
}
#[no_mangle]
pub unsafe extern "C" fn register_dependency(mut filename: *const libc::c_char) {
    let mut dep: *mut dependency = 0 as *mut dependency;
    if dep_file.is_null() {
        return;
    }
    dep = dep_chain;
    while !dep.is_null() {
        if filename_cmp(filename, (*dep).file) == 0 {
            return;
        }
        dep = (*dep).next;
    }
    dep = xmalloc(::core::mem::size_of::<dependency>() as libc::c_ulong)
        as *mut dependency;
    (*dep).file = xstrdup(filename);
    (*dep).next = dep_chain;
    dep_chain = dep;
}
#[no_mangle]
pub unsafe extern "C" fn print_dependencies() {
    let mut f: *mut FILE = 0 as *mut FILE;
    let mut dep: *mut dependency = 0 as *mut dependency;
    if dep_file.is_null() {
        return;
    }
    f = fopen(dep_file, b"w\0" as *const u8 as *const libc::c_char);
    if f.is_null() {
        as_warn(
            dcgettext(
                0 as *const libc::c_char,
                b"can't open `%s' for writing\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            dep_file,
        );
        return;
    }
    column = 0 as libc::c_int;
    wrap_output(f, out_file_name, ':' as i32);
    dep = dep_chain;
    while !dep.is_null() {
        wrap_output(f, (*dep).file, ' ' as i32);
        dep = (*dep).next;
    }
    putc('\n' as i32, f);
    if fclose(f) != 0 {
        as_warn(
            dcgettext(
                0 as *const libc::c_char,
                b"can't close `%s'\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            dep_file,
        );
    }
}
static mut dep_file: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut dep_chain: *mut dependency = 0 as *const dependency as *mut dependency;
static mut column: libc::c_int = 0 as libc::c_int;
unsafe extern "C" fn quote_string_for_make(
    mut file: *mut FILE,
    mut src: *const libc::c_char,
) -> libc::c_int {
    let mut p: *const libc::c_char = src;
    let mut i: libc::c_int = 0 as libc::c_int;
    loop {
        let fresh0 = p;
        p = p.offset(1);
        let mut c: libc::c_char = *fresh0;
        match c as libc::c_int {
            0 | 32 | 9 => {
                let mut q: *const libc::c_char = 0 as *const libc::c_char;
                q = p.offset(-(1 as libc::c_int as isize));
                while src < q
                    && *q.offset(-(1 as libc::c_int) as isize) as libc::c_int
                        == '\\' as i32
                {
                    if !file.is_null() {
                        putc('\\' as i32, file);
                    }
                    i += 1;
                    i;
                    q = q.offset(-1);
                    q;
                }
                if c == 0 {
                    return i;
                }
                if !file.is_null() {
                    putc('\\' as i32, file);
                }
                i += 1;
                i;
            }
            36 => {
                if !file.is_null() {
                    putc(c as libc::c_int, file);
                }
                i += 1;
                i;
            }
            _ => {}
        }
        if !file.is_null() {
            putc(c as libc::c_int, file);
        }
        i += 1;
        i;
    };
}
unsafe extern "C" fn wrap_output(
    mut f: *mut FILE,
    mut string: *const libc::c_char,
    mut spacer: libc::c_int,
) {
    let mut len: libc::c_int = quote_string_for_make(0 as *mut FILE, string);
    if len == 0 as libc::c_int {
        return;
    }
    if column != 0
        && (72 as libc::c_int - 1 as libc::c_int - 2 as libc::c_int) < column + len
    {
        fprintf(f, b" \\\n \0" as *const u8 as *const libc::c_char);
        column = 0 as libc::c_int;
        if spacer == ' ' as i32 {
            spacer = '\0' as i32;
        }
    }
    if spacer == ' ' as i32 {
        putc(spacer, f);
        column += 1;
        column;
    }
    quote_string_for_make(f, string);
    column += len;
    if spacer == ':' as i32 {
        putc(spacer, f);
        column += 1;
        column;
    }
}
