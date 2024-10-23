extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn htab_find_slot(
        _: htab_t,
        _: *const libc::c_void,
        _: insert_option,
    ) -> *mut *mut libc::c_void;
    fn htab_size(_: htab_t) -> size_t;
    fn htab_elements(_: htab_t) -> size_t;
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
pub struct htab {
    pub hash_f: htab_hash,
    pub eq_f: htab_eq,
    pub del_f: htab_del,
    pub entries: *mut *mut libc::c_void,
    pub size: size_t,
    pub n_elements: size_t,
    pub n_deleted: size_t,
    pub searches: libc::c_uint,
    pub collisions: libc::c_uint,
    pub alloc_f: htab_alloc,
    pub free_f: htab_free,
    pub alloc_arg: *mut libc::c_void,
    pub alloc_with_arg_f: htab_alloc_with_arg,
    pub free_with_arg_f: htab_free_with_arg,
    pub size_prime_index: libc::c_uint,
}
pub type htab_free_with_arg = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
>;
pub type htab_alloc_with_arg = Option::<
    unsafe extern "C" fn(*mut libc::c_void, size_t, size_t) -> *mut libc::c_void,
>;
pub type htab_free = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type htab_alloc = Option::<
    unsafe extern "C" fn(size_t, size_t) -> *mut libc::c_void,
>;
pub type htab_del = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type htab_eq = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
pub type htab_hash = Option::<unsafe extern "C" fn(*const libc::c_void) -> hashval_t>;
pub type hashval_t = libc::c_uint;
pub type htab_t = *mut htab;
pub type insert_option = libc::c_uint;
pub const INSERT: insert_option = 1;
pub const NO_INSERT: insert_option = 0;
#[no_mangle]
pub unsafe extern "C" fn htab_insert(
    mut htab: htab_t,
    mut element: *mut libc::c_void,
    mut replace: libc::c_int,
) -> *mut *mut libc::c_void {
    let mut slot: *mut *mut libc::c_void = htab_find_slot(htab, element, INSERT);
    if !(*slot).is_null() {
        if replace != 0 {
            if ((*htab).del_f).is_some() {
                (Some(((*htab).del_f).expect("non-null function pointer")))
                    .expect("non-null function pointer")(*slot);
            }
            *slot = element;
        }
        return slot;
    }
    *slot = element;
    return 0 as *mut *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn htab_print_statistics(
    mut f: *mut FILE,
    mut name: *const libc::c_char,
    mut table: htab_t,
) {
    fprintf(f, b"%s hash statistics:\n\0" as *const u8 as *const libc::c_char, name);
    fprintf(
        f,
        b"\t%u searches\n\0" as *const u8 as *const libc::c_char,
        (*table).searches,
    );
    fprintf(
        f,
        b"\t%u collisions\n\0" as *const u8 as *const libc::c_char,
        (*table).collisions,
    );
    fprintf(
        f,
        b"\t%lu elements\n\0" as *const u8 as *const libc::c_char,
        htab_elements(table),
    );
    fprintf(
        f,
        b"\t%lu table size\n\0" as *const u8 as *const libc::c_char,
        htab_size(table),
    );
}
