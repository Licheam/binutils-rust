use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn xexit(status: libc::c_int) -> !;
    static mut environ: *mut *mut libc::c_char;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn sbrk(__delta: intptr_t) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __intptr_t = libc::c_long;
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
pub type intptr_t = __intptr_t;
static mut name: *const libc::c_char = b"\0" as *const u8 as *const libc::c_char;
static mut first_break: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub unsafe extern "C" fn xmalloc_set_program_name(mut s: *const libc::c_char) {
    name = s;
    if first_break.is_null() {
        first_break = sbrk(0 as libc::c_int as intptr_t) as *mut libc::c_char;
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmalloc_failed(mut size: size_t) -> ! {
    let mut allocated: size_t = 0;
    if !first_break.is_null() {
        allocated = (sbrk(0 as libc::c_int as intptr_t) as *mut libc::c_char)
            .offset_from(first_break) as libc::c_long as size_t;
    } else {
        allocated = (sbrk(0 as libc::c_int as intptr_t) as *mut libc::c_char)
            .offset_from(
                &mut environ as *mut *mut *mut libc::c_char as *mut libc::c_char,
            ) as libc::c_long as size_t;
    }
    fprintf(
        stderr,
        b"\n%s%sout of memory allocating %lu bytes after a total of %lu bytes\n\0"
            as *const u8 as *const libc::c_char,
        name,
        if *name as libc::c_int != 0 {
            b": \0" as *const u8 as *const libc::c_char
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
        size,
        allocated,
    );
    xexit(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn xmalloc(mut size: size_t) -> *mut libc::c_void {
    let mut newmem: *mut libc::c_void = 0 as *mut libc::c_void;
    if size == 0 as libc::c_int as libc::c_ulong {
        size = 1 as libc::c_int as size_t;
    }
    newmem = malloc(size);
    if newmem.is_null() {
        xmalloc_failed(size);
    }
    return newmem;
}
#[no_mangle]
pub unsafe extern "C" fn xcalloc(
    mut nelem: size_t,
    mut elsize: size_t,
) -> *mut libc::c_void {
    let mut newmem: *mut libc::c_void = 0 as *mut libc::c_void;
    if nelem == 0 as libc::c_int as libc::c_ulong
        || elsize == 0 as libc::c_int as libc::c_ulong
    {
        elsize = 1 as libc::c_int as size_t;
        nelem = elsize;
    }
    newmem = calloc(nelem, elsize);
    if newmem.is_null() {
        xmalloc_failed(nelem.wrapping_mul(elsize));
    }
    return newmem;
}
#[no_mangle]
pub unsafe extern "C" fn xrealloc(
    mut oldmem: *mut libc::c_void,
    mut size: size_t,
) -> *mut libc::c_void {
    let mut newmem: *mut libc::c_void = 0 as *mut libc::c_void;
    if size == 0 as libc::c_int as libc::c_ulong {
        size = 1 as libc::c_int as size_t;
    }
    if oldmem.is_null() {
        newmem = malloc(size);
    } else {
        newmem = realloc(oldmem, size);
    }
    if newmem.is_null() {
        xmalloc_failed(size);
    }
    return newmem;
}
