use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn free(_: *mut libc::c_void);
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn xmalloc(_: size_t) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
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
pub struct partition_elem {
    pub next: *mut partition_elem,
    pub class_element: libc::c_int,
    pub class_count: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct partition_def {
    pub num_elements: libc::c_int,
    pub elements: [partition_elem; 1],
}
pub type partition = *mut partition_def;
#[no_mangle]
pub unsafe extern "C" fn partition_new(mut num_elements: libc::c_int) -> partition {
    let mut e: libc::c_int = 0;
    let mut part: partition = xmalloc(
        (::core::mem::size_of::<partition_def>() as libc::c_ulong)
            .wrapping_add(
                ((num_elements - 1 as libc::c_int) as libc::c_ulong)
                    .wrapping_mul(
                        ::core::mem::size_of::<partition_elem>() as libc::c_ulong,
                    ),
            ),
    ) as partition;
    (*part).num_elements = num_elements;
    e = 0 as libc::c_int;
    while e < num_elements {
        (*((*part).elements).as_mut_ptr().offset(e as isize)).class_element = e;
        let ref mut fresh0 = (*((*part).elements).as_mut_ptr().offset(e as isize)).next;
        *fresh0 = &mut *((*part).elements).as_mut_ptr().offset(e as isize)
            as *mut partition_elem;
        (*((*part).elements).as_mut_ptr().offset(e as isize))
            .class_count = 1 as libc::c_int as libc::c_uint;
        e += 1;
        e;
    }
    return part;
}
#[no_mangle]
pub unsafe extern "C" fn partition_delete(mut part: partition) {
    free(part as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn partition_union(
    mut part: partition,
    mut elem1: libc::c_int,
    mut elem2: libc::c_int,
) -> libc::c_int {
    let mut elements: *mut partition_elem = ((*part).elements).as_mut_ptr();
    let mut e1: *mut partition_elem = 0 as *mut partition_elem;
    let mut e2: *mut partition_elem = 0 as *mut partition_elem;
    let mut p: *mut partition_elem = 0 as *mut partition_elem;
    let mut old_next: *mut partition_elem = 0 as *mut partition_elem;
    let mut class_element: libc::c_int = (*elements.offset(elem1 as isize))
        .class_element;
    if class_element == (*elements.offset(elem2 as isize)).class_element {
        return class_element;
    }
    if (*elements.offset(elem1 as isize)).class_count
        < (*elements.offset(elem2 as isize)).class_count
    {
        let mut temp: libc::c_int = elem1;
        elem1 = elem2;
        elem2 = temp;
        class_element = (*elements.offset(elem1 as isize)).class_element;
    }
    e1 = &mut *elements.offset(elem1 as isize) as *mut partition_elem;
    e2 = &mut *elements.offset(elem2 as isize) as *mut partition_elem;
    let ref mut fresh1 = (*elements.offset(class_element as isize)).class_count;
    *fresh1 = (*fresh1)
        .wrapping_add((*elements.offset((*e2).class_element as isize)).class_count);
    (*e2).class_element = class_element;
    p = (*e2).next;
    while p != e2 {
        (*p).class_element = class_element;
        p = (*p).next;
    }
    old_next = (*e1).next;
    (*e1).next = (*e2).next;
    (*e2).next = old_next;
    return class_element;
}
unsafe extern "C" fn elem_compare(
    mut elem1: *const libc::c_void,
    mut elem2: *const libc::c_void,
) -> libc::c_int {
    let mut e1: libc::c_int = *(elem1 as *const libc::c_int);
    let mut e2: libc::c_int = *(elem2 as *const libc::c_int);
    if e1 < e2 {
        return -(1 as libc::c_int)
    } else if e1 > e2 {
        return 1 as libc::c_int
    } else {
        return 0 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn partition_print(mut part: partition, mut fp: *mut FILE) {
    let mut done: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut num_elements: libc::c_int = (*part).num_elements;
    let mut elements: *mut partition_elem = ((*part).elements).as_mut_ptr();
    let mut class_elements: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut e: libc::c_int = 0;
    done = xmalloc(num_elements as size_t) as *mut libc::c_char;
    memset(done as *mut libc::c_void, 0 as libc::c_int, num_elements as libc::c_ulong);
    class_elements = xmalloc(
        (num_elements as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    fputc('[' as i32, fp);
    e = 0 as libc::c_int;
    while e < num_elements {
        if *done.offset(e as isize) == 0 {
            let mut c: libc::c_int = e;
            let mut count: libc::c_int = (*elements
                .offset((*elements.offset(e as isize)).class_element as isize))
                .class_count as libc::c_int;
            let mut i: libc::c_int = 0;
            i = 0 as libc::c_int;
            while i < count {
                *class_elements.offset(i as isize) = c;
                *done.offset(c as isize) = 1 as libc::c_int as libc::c_char;
                c = ((*elements.offset(c as isize)).next).offset_from(elements)
                    as libc::c_long as libc::c_int;
                i += 1;
                i;
            }
            qsort(
                class_elements as *mut libc::c_void,
                count as size_t,
                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                Some(
                    elem_compare
                        as unsafe extern "C" fn(
                            *const libc::c_void,
                            *const libc::c_void,
                        ) -> libc::c_int,
                ),
            );
            fputc('(' as i32, fp);
            i = 0 as libc::c_int;
            while i < count {
                fprintf(
                    fp,
                    if i == 0 as libc::c_int {
                        b"%d\0" as *const u8 as *const libc::c_char
                    } else {
                        b" %d\0" as *const u8 as *const libc::c_char
                    },
                    *class_elements.offset(i as isize),
                );
                i += 1;
                i;
            }
            fputc(')' as i32, fp);
        }
        e += 1;
        e;
    }
    fputc(']' as i32, fp);
    free(class_elements as *mut libc::c_void);
    free(done as *mut libc::c_void);
}
