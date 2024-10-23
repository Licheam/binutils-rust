extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdout: *mut FILE;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn free(_: *mut libc::c_void);
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn filename_cmp(s1: *const libc::c_char, s2: *const libc::c_char) -> libc::c_int;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    static mut first_output: bool;
    static mut print_path: bool;
    static mut print_descriptions: bool;
    static mut line_granularity: bool;
    static mut ignore_zeros: bool;
    static mut bsd_style_output: bool;
    static mut output_width: libc::c_int;
    static mut debug_level: libc::c_int;
    static mut hz: libc::c_long;
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    static mut symtab: Sym_Table;
    static mut num_cycles: libc::c_uint;
    static mut cycle_header: *mut Sym;
    static mut arcs: *mut *mut Arc;
    static mut numarcs: libc::c_uint;
    static mut hist_scale: libc::c_double;
    fn print_name_only(self_0: *mut Sym) -> libc::c_int;
    fn print_name(self_0: *mut Sym);
    static mut symbol_map: *mut function_map;
    static mut symbol_map_count: libc::c_uint;
    fn bsd_callg_blurb(fp: *mut FILE);
    fn fsf_callg_blurb(fp: *mut FILE);
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
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
pub type bfd_vma = libc::c_ulong;
pub type UNIT = [libc::c_uchar; 2];
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
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct sym {
    pub addr: bfd_vma,
    pub end_addr: bfd_vma,
    pub name: *const libc::c_char,
    pub file: *mut Source_File,
    pub line_num: libc::c_int,
    #[bitfield(name = "is_func", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "is_static", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "is_bb_head", ty = "libc::c_uint", bits = "2..=2")]
    #[bitfield(name = "mapped", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "has_been_placed", ty = "libc::c_uint", bits = "4..=4")]
    pub is_func_is_static_is_bb_head_mapped_has_been_placed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub ncalls: libc::c_ulong,
    pub nuses: libc::c_int,
    pub bb_addr: [bfd_vma; 10],
    pub bb_calls: [libc::c_ulong; 10],
    pub next: *mut sym,
    pub prev: *mut sym,
    pub hist: C2RustUnnamed_2,
    pub cg: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub self_calls: libc::c_ulong,
    pub child_time: libc::c_double,
    pub index: libc::c_int,
    pub top_order: libc::c_int,
    pub print_flag: bool,
    pub prop: C2RustUnnamed_1,
    pub cyc: C2RustUnnamed_0,
    pub parents: *mut arc,
    pub children: *mut arc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct arc {
    pub parent: *mut Sym,
    pub child: *mut Sym,
    pub count: libc::c_ulong,
    pub time: libc::c_double,
    pub child_time: libc::c_double,
    pub next_parent: *mut arc,
    pub next_child: *mut arc,
    pub has_been_placed: libc::c_int,
}
pub type Sym = sym;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub num: libc::c_int,
    pub head: *mut sym,
    pub next: *mut sym,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub fract: libc::c_double,
    pub self_0: libc::c_double,
    pub child: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub time: libc::c_double,
    pub scaled_addr: bfd_vma,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Sym_Table {
    pub len: libc::c_uint,
    pub base: *mut Sym,
    pub limit: *mut Sym,
}
pub type Arc = arc;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct function_map {
    pub function_name: *mut libc::c_char,
    pub file_name: *mut libc::c_char,
    #[bitfield(name = "is_first", ty = "libc::c_uint", bits = "0..=0")]
    pub is_first: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
#[inline]
unsafe extern "C" fn putchar(mut __c: libc::c_int) -> libc::c_int {
    return putc(__c, stdout);
}
#[no_mangle]
pub static mut print_time: libc::c_double = 0.0f64;
#[no_mangle]
pub unsafe extern "C" fn cg_print(mut timesortsym: *mut *mut Sym) {
    let mut sym_index: libc::c_uint = 0;
    let mut parent: *mut Sym = 0 as *mut Sym;
    if print_descriptions as libc::c_int != 0 && bsd_style_output as libc::c_int != 0 {
        bsd_callg_blurb(stdout);
    }
    print_header();
    sym_index = 0 as libc::c_int as libc::c_uint;
    while sym_index < (symtab.len).wrapping_add(num_cycles) {
        parent = *timesortsym.offset(sym_index as isize);
        if !(ignore_zeros as libc::c_int != 0
            && (*parent).ncalls == 0 as libc::c_int as libc::c_ulong
            && (*parent).cg.self_calls == 0 as libc::c_int as libc::c_ulong
            && (*parent).cg.prop.self_0 == 0 as libc::c_int as libc::c_double
            && (*parent).cg.prop.child == 0 as libc::c_int as libc::c_double
            || !(*parent).cg.print_flag
            || line_granularity as libc::c_int != 0 && (*parent).is_func() == 0)
        {
            if ((*parent).name).is_null() && (*parent).cg.cyc.num != 0 as libc::c_int {
                print_cycle(parent);
                print_members(parent);
            } else {
                print_parents(parent);
                print_line(parent);
                print_children(parent);
            }
            if bsd_style_output {
                printf(b"\n\0" as *const u8 as *const libc::c_char);
            }
            printf(
                b"-----------------------------------------------\n\0" as *const u8
                    as *const libc::c_char,
            );
            if bsd_style_output {
                printf(b"\n\0" as *const u8 as *const libc::c_char);
            }
        }
        sym_index = sym_index.wrapping_add(1);
        sym_index;
    }
    free(timesortsym as *mut libc::c_void);
    if print_descriptions as libc::c_int != 0 && !bsd_style_output {
        fsf_callg_blurb(stdout);
    }
}
#[no_mangle]
pub unsafe extern "C" fn cg_print_index() {
    let mut sym_index: libc::c_uint = 0;
    let mut nnames: libc::c_uint = 0;
    let mut todo: libc::c_uint = 0;
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_uint = 0;
    let mut col: libc::c_int = 0;
    let mut starting_col: libc::c_int = 0;
    let mut name_sorted_syms: *mut *mut Sym = 0 as *mut *mut Sym;
    let mut sym: *mut Sym = 0 as *mut Sym;
    let mut filename: *const libc::c_char = 0 as *const libc::c_char;
    let mut buf: [libc::c_char; 20] = [0; 20];
    let mut column_width: libc::c_int = (output_width - 1 as libc::c_int)
        / 3 as libc::c_int;
    name_sorted_syms = xmalloc(
        ((symtab.len).wrapping_add(num_cycles) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut Sym>() as libc::c_ulong),
    ) as *mut *mut Sym;
    sym_index = 0 as libc::c_int as libc::c_uint;
    nnames = 0 as libc::c_int as libc::c_uint;
    while sym_index < symtab.len {
        if !(ignore_zeros as libc::c_int != 0
            && (*(symtab.base).offset(sym_index as isize)).ncalls
                == 0 as libc::c_int as libc::c_ulong
            && (*(symtab.base).offset(sym_index as isize)).hist.time
                == 0 as libc::c_int as libc::c_double)
        {
            let fresh0 = nnames;
            nnames = nnames.wrapping_add(1);
            let ref mut fresh1 = *name_sorted_syms.offset(fresh0 as isize);
            *fresh1 = &mut *(symtab.base).offset(sym_index as isize) as *mut Sym;
        }
        sym_index = sym_index.wrapping_add(1);
        sym_index;
    }
    qsort(
        name_sorted_syms as *mut libc::c_void,
        nnames as size_t,
        ::core::mem::size_of::<*mut Sym>() as libc::c_ulong,
        Some(
            cmp_name
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    sym_index = 1 as libc::c_int as libc::c_uint;
    todo = nnames;
    while sym_index <= num_cycles {
        let fresh2 = todo;
        todo = todo.wrapping_add(1);
        let ref mut fresh3 = *name_sorted_syms.offset(fresh2 as isize);
        *fresh3 = &mut *cycle_header.offset(sym_index as isize) as *mut Sym;
        sym_index = sym_index.wrapping_add(1);
        sym_index;
    }
    printf(b"\x0C\n\0" as *const u8 as *const libc::c_char);
    printf(
        dcgettext(
            b"gprof\0" as *const u8 as *const libc::c_char,
            b"Index by function name\n\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    sym_index = todo
        .wrapping_add(2 as libc::c_int as libc::c_uint)
        .wrapping_div(3 as libc::c_int as libc::c_uint);
    i = 0 as libc::c_int as libc::c_uint;
    while i < sym_index {
        col = 0 as libc::c_int;
        starting_col = 0 as libc::c_int;
        j = i;
        while j < todo {
            sym = *name_sorted_syms.offset(j as isize);
            if (*sym).cg.print_flag {
                sprintf(
                    buf.as_mut_ptr(),
                    b"[%d]\0" as *const u8 as *const libc::c_char,
                    (*sym).cg.index,
                );
            } else {
                sprintf(
                    buf.as_mut_ptr(),
                    b"(%d)\0" as *const u8 as *const libc::c_char,
                    (*sym).cg.index,
                );
            }
            if j < nnames {
                if bsd_style_output {
                    printf(
                        b"%6.6s %-19.19s\0" as *const u8 as *const libc::c_char,
                        buf.as_mut_ptr(),
                        (*sym).name,
                    );
                } else {
                    col = (col as libc::c_ulong).wrapping_add(strlen(buf.as_mut_ptr()))
                        as libc::c_int as libc::c_int;
                    while col < starting_col + 5 as libc::c_int {
                        putchar(' ' as i32);
                        col += 1;
                        col;
                    }
                    printf(
                        b" %s \0" as *const u8 as *const libc::c_char,
                        buf.as_mut_ptr(),
                    );
                    col += print_name_only(sym);
                    if !line_granularity && (*sym).is_static() as libc::c_int != 0
                        && !((*sym).file).is_null()
                    {
                        filename = (*(*sym).file).name;
                        if !print_path {
                            filename = strrchr(filename, '/' as i32);
                            if !filename.is_null() {
                                filename = filename.offset(1);
                                filename;
                            } else {
                                filename = (*(*sym).file).name;
                            }
                        }
                        printf(b" (%s)\0" as *const u8 as *const libc::c_char, filename);
                        col = (col as libc::c_ulong)
                            .wrapping_add(
                                (strlen(filename))
                                    .wrapping_add(3 as libc::c_int as libc::c_ulong),
                            ) as libc::c_int as libc::c_int;
                    }
                }
            } else if bsd_style_output {
                printf(
                    b"%6.6s \0" as *const u8 as *const libc::c_char,
                    buf.as_mut_ptr(),
                );
                sprintf(
                    buf.as_mut_ptr(),
                    dcgettext(
                        b"gprof\0" as *const u8 as *const libc::c_char,
                        b"<cycle %d>\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    (*sym).cg.cyc.num,
                );
                printf(
                    b"%-19.19s\0" as *const u8 as *const libc::c_char,
                    buf.as_mut_ptr(),
                );
            } else {
                col = (col as libc::c_ulong).wrapping_add(strlen(buf.as_mut_ptr()))
                    as libc::c_int as libc::c_int;
                while col < starting_col + 5 as libc::c_int {
                    putchar(' ' as i32);
                    col += 1;
                    col;
                }
                printf(b" %s \0" as *const u8 as *const libc::c_char, buf.as_mut_ptr());
                sprintf(
                    buf.as_mut_ptr(),
                    dcgettext(
                        b"gprof\0" as *const u8 as *const libc::c_char,
                        b"<cycle %d>\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    (*sym).cg.cyc.num,
                );
                printf(b"%s\0" as *const u8 as *const libc::c_char, buf.as_mut_ptr());
                col = (col as libc::c_ulong).wrapping_add(strlen(buf.as_mut_ptr()))
                    as libc::c_int as libc::c_int;
            }
            starting_col += column_width;
            j = j.wrapping_add(sym_index);
        }
        printf(b"\n\0" as *const u8 as *const libc::c_char);
        i = i.wrapping_add(1);
        i;
    }
    free(name_sorted_syms as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn cg_print_file_ordering() {
    let mut scratch_arc_count: libc::c_ulong = 0;
    let mut arc_index: libc::c_ulong = 0;
    let mut sym_index: libc::c_ulong = 0;
    let mut scratch_arcs: *mut *mut Arc = 0 as *mut *mut Arc;
    let mut last: *mut libc::c_char = 0 as *mut libc::c_char;
    scratch_arc_count = 0 as libc::c_int as libc::c_ulong;
    scratch_arcs = xmalloc(
        (numarcs as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut Arc>() as libc::c_ulong),
    ) as *mut *mut Arc;
    arc_index = 0 as libc::c_int as libc::c_ulong;
    while arc_index < numarcs as libc::c_ulong {
        if (*(**arcs.offset(arc_index as isize)).parent).mapped() == 0
            || (*(**arcs.offset(arc_index as isize)).child).mapped() == 0
        {
            (**arcs.offset(arc_index as isize)).has_been_placed = 1 as libc::c_int;
        }
        arc_index = arc_index.wrapping_add(1);
        arc_index;
    }
    order_and_dump_functions_by_arcs(
        arcs,
        numarcs as libc::c_ulong,
        0 as libc::c_int,
        scratch_arcs,
        &mut scratch_arc_count,
    );
    sym_index = 0 as libc::c_int as libc::c_ulong;
    while sym_index < symtab.len as libc::c_ulong {
        if (*(symtab.base).offset(sym_index as isize)).mapped() as libc::c_int != 0
            && (*(symtab.base).offset(sym_index as isize)).has_been_placed() == 0
        {
            printf(
                b"%s\n\0" as *const u8 as *const libc::c_char,
                (*(symtab.base).offset(sym_index as isize)).name,
            );
        }
        sym_index = sym_index.wrapping_add(1);
        sym_index;
    }
    qsort(
        symbol_map as *mut libc::c_void,
        symbol_map_count as size_t,
        ::core::mem::size_of::<function_map>() as libc::c_ulong,
        Some(
            cmp_symbol_map
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    last = 0 as *mut libc::c_char;
    sym_index = 0 as libc::c_int as libc::c_ulong;
    while sym_index < symbol_map_count as libc::c_ulong {
        let mut index2: libc::c_uint = 0;
        if !(!last.is_null()
            && filename_cmp(last, (*symbol_map.offset(sym_index as isize)).file_name)
                == 0)
        {
            index2 = 0 as libc::c_int as libc::c_uint;
            while index2 < symtab.len {
                if !((*(symtab.base).offset(index2 as isize)).mapped() == 0) {
                    if filename_cmp(
                        (*(symtab.base).offset(index2 as isize)).name,
                        (*symbol_map.offset(sym_index as isize)).file_name,
                    ) == 0
                    {
                        break;
                    }
                }
                index2 = index2.wrapping_add(1);
                index2;
            }
            if index2 == symtab.len {
                printf(
                    b"%s\n\0" as *const u8 as *const libc::c_char,
                    (*symbol_map.offset(sym_index as isize)).file_name,
                );
            }
            last = (*symbol_map.offset(sym_index as isize)).file_name;
        }
        sym_index = sym_index.wrapping_add(1);
        sym_index;
    }
}
#[no_mangle]
pub unsafe extern "C" fn cg_print_function_ordering() {
    let mut sym_index: libc::c_ulong = 0;
    let mut arc_index: libc::c_ulong = 0;
    let mut used: libc::c_ulong = 0;
    let mut unused: libc::c_ulong = 0;
    let mut scratch_index: libc::c_ulong = 0;
    let mut unplaced_arc_count: libc::c_ulong = 0;
    let mut high_arc_count: libc::c_ulong = 0;
    let mut scratch_arc_count: libc::c_ulong = 0;
    let mut total_arcs: libc::c_ulonglong = 0;
    let mut tmp_arcs_count: libc::c_ulonglong = 0;
    let mut unused_syms: *mut *mut Sym = 0 as *mut *mut Sym;
    let mut used_syms: *mut *mut Sym = 0 as *mut *mut Sym;
    let mut scratch_syms: *mut *mut Sym = 0 as *mut *mut Sym;
    let mut unplaced_arcs: *mut *mut Arc = 0 as *mut *mut Arc;
    let mut high_arcs: *mut *mut Arc = 0 as *mut *mut Arc;
    let mut scratch_arcs: *mut *mut Arc = 0 as *mut *mut Arc;
    sym_index = 0 as libc::c_int as libc::c_ulong;
    used = 0 as libc::c_int as libc::c_ulong;
    unused = 0 as libc::c_int as libc::c_ulong;
    scratch_index = 0 as libc::c_int as libc::c_ulong;
    unplaced_arc_count = 0 as libc::c_int as libc::c_ulong;
    high_arc_count = 0 as libc::c_int as libc::c_ulong;
    scratch_arc_count = 0 as libc::c_int as libc::c_ulong;
    unused_syms = xmalloc(
        (symtab.len as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut Sym>() as libc::c_ulong),
    ) as *mut *mut Sym;
    used_syms = xmalloc(
        (symtab.len as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut Sym>() as libc::c_ulong),
    ) as *mut *mut Sym;
    scratch_syms = xmalloc(
        (symtab.len as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut Sym>() as libc::c_ulong),
    ) as *mut *mut Sym;
    high_arcs = xmalloc(
        (numarcs as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut Arc>() as libc::c_ulong),
    ) as *mut *mut Arc;
    scratch_arcs = xmalloc(
        (numarcs as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut Arc>() as libc::c_ulong),
    ) as *mut *mut Arc;
    unplaced_arcs = xmalloc(
        (numarcs as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut Arc>() as libc::c_ulong),
    ) as *mut *mut Arc;
    sym_index = 0 as libc::c_int as libc::c_ulong;
    used = 0 as libc::c_int as libc::c_ulong;
    unused = 0 as libc::c_int as libc::c_ulong;
    while sym_index < symtab.len as libc::c_ulong {
        if (*(symtab.base).offset(sym_index as isize)).ncalls
            == 0 as libc::c_int as libc::c_ulong
        {
            let fresh4 = unused;
            unused = unused.wrapping_add(1);
            let ref mut fresh5 = *unused_syms.offset(fresh4 as isize);
            *fresh5 = &mut *(symtab.base).offset(sym_index as isize) as *mut Sym;
            let ref mut fresh6 = *(symtab.base).offset(sym_index as isize);
            (*fresh6).set_has_been_placed(1 as libc::c_int as libc::c_uint);
        } else {
            let fresh7 = used;
            used = used.wrapping_add(1);
            let ref mut fresh8 = *used_syms.offset(fresh7 as isize);
            *fresh8 = &mut *(symtab.base).offset(sym_index as isize) as *mut Sym;
            let ref mut fresh9 = *(symtab.base).offset(sym_index as isize);
            (*fresh9).set_has_been_placed(0 as libc::c_int as libc::c_uint);
            let ref mut fresh10 = (*(symtab.base).offset(sym_index as isize)).next;
            *fresh10 = 0 as *mut sym;
            let ref mut fresh11 = (*(symtab.base).offset(sym_index as isize)).prev;
            *fresh11 = 0 as *mut sym;
            (*(symtab.base).offset(sym_index as isize)).nuses = 0 as libc::c_int;
        }
        sym_index = sym_index.wrapping_add(1);
        sym_index;
    }
    qsort(
        arcs as *mut libc::c_void,
        numarcs as size_t,
        ::core::mem::size_of::<*mut Arc>() as libc::c_ulong,
        Some(
            cmp_arc_count
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    total_arcs = 0 as libc::c_int as libc::c_ulonglong;
    arc_index = 0 as libc::c_int as libc::c_ulong;
    while arc_index < numarcs as libc::c_ulong {
        total_arcs = total_arcs
            .wrapping_add(
                (**arcs.offset(arc_index as isize)).count as libc::c_ulonglong,
            );
        (**arcs.offset(arc_index as isize)).has_been_placed = 0 as libc::c_int;
        arc_index = arc_index.wrapping_add(1);
        arc_index;
    }
    tmp_arcs_count = 0 as libc::c_int as libc::c_ulonglong;
    arc_index = 0 as libc::c_int as libc::c_ulong;
    while arc_index < numarcs as libc::c_ulong {
        tmp_arcs_count = tmp_arcs_count
            .wrapping_add(
                (**arcs.offset(arc_index as isize)).count as libc::c_ulonglong,
            );
        if tmp_arcs_count as libc::c_double / total_arcs as libc::c_double > 0.90f64 {
            break;
        }
        let ref mut fresh12 = (*(**arcs.offset(arc_index as isize)).child).nuses;
        *fresh12 += 1;
        *fresh12;
        arc_index = arc_index.wrapping_add(1);
        arc_index;
    }
    memcpy(
        scratch_syms as *mut libc::c_void,
        used_syms as *const libc::c_void,
        used.wrapping_mul(::core::mem::size_of::<*mut Sym>() as libc::c_ulong),
    );
    qsort(
        scratch_syms as *mut libc::c_void,
        used,
        ::core::mem::size_of::<*mut Sym>() as libc::c_ulong,
        Some(
            cmp_fun_nuses
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    sym_index = 0 as libc::c_int as libc::c_ulong;
    while sym_index < used.wrapping_div(80 as libc::c_int as libc::c_ulong) {
        let mut sym: *mut Sym = *scratch_syms.offset(sym_index as isize);
        let mut arc: *mut Arc = 0 as *mut Arc;
        if (*sym).nuses == 5 as libc::c_int {
            break;
        }
        arc = (*sym).cg.children;
        while !arc.is_null() {
            if (*arc).parent != (*arc).child {
                let fresh13 = scratch_arc_count;
                scratch_arc_count = scratch_arc_count.wrapping_add(1);
                let ref mut fresh14 = *scratch_arcs.offset(fresh13 as isize);
                *fresh14 = arc;
            }
            (*arc).has_been_placed = 1 as libc::c_int;
            arc = (*arc).next_child;
        }
        arc = (*sym).cg.parents;
        while !arc.is_null() {
            if (*arc).parent != (*arc).child {
                let fresh15 = scratch_arc_count;
                scratch_arc_count = scratch_arc_count.wrapping_add(1);
                let ref mut fresh16 = *scratch_arcs.offset(fresh15 as isize);
                *fresh16 = arc;
            }
            (*arc).has_been_placed = 1 as libc::c_int;
            arc = (*arc).next_parent;
        }
        scratch_index = sym_index;
        (*sym).set_has_been_placed(1 as libc::c_int as libc::c_uint);
        sym_index = sym_index.wrapping_add(1);
        sym_index;
    }
    arc_index = 0 as libc::c_int as libc::c_ulong;
    while arc_index < scratch_arc_count {
        let mut arc_0: *mut Arc = *scratch_arcs.offset(arc_index as isize);
        if (*(*arc_0).child).has_been_placed() as libc::c_int != 0
            && (*(*arc_0).parent).has_been_placed() as libc::c_int != 0
        {
            let fresh17 = high_arc_count;
            high_arc_count = high_arc_count.wrapping_add(1);
            let ref mut fresh18 = *high_arcs.offset(fresh17 as isize);
            *fresh18 = *scratch_arcs.offset(arc_index as isize);
            (*(*arc_0).child).set_has_been_placed(0 as libc::c_int as libc::c_uint);
            (*(*arc_0).parent).set_has_been_placed(0 as libc::c_int as libc::c_uint);
        }
        arc_index = arc_index.wrapping_add(1);
        arc_index;
    }
    sym_index = 0 as libc::c_int as libc::c_ulong;
    while sym_index < scratch_index {
        if (**scratch_syms.offset(sym_index as isize)).has_been_placed() != 0 {
            printf(
                b"%s\n\0" as *const u8 as *const libc::c_char,
                (**scratch_syms.offset(sym_index as isize)).name,
            );
        }
        sym_index = sym_index.wrapping_add(1);
        sym_index;
    }
    qsort(
        high_arcs as *mut libc::c_void,
        high_arc_count,
        ::core::mem::size_of::<*mut Arc>() as libc::c_ulong,
        Some(
            cmp_arc_count
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    order_and_dump_functions_by_arcs(
        high_arcs,
        high_arc_count,
        1 as libc::c_int,
        unplaced_arcs,
        &mut unplaced_arc_count,
    );
    order_and_dump_functions_by_arcs(
        arcs,
        numarcs as libc::c_ulong,
        0 as libc::c_int,
        unplaced_arcs,
        &mut unplaced_arc_count,
    );
    order_and_dump_functions_by_arcs(
        unplaced_arcs,
        unplaced_arc_count,
        1 as libc::c_int,
        scratch_arcs,
        &mut scratch_arc_count,
    );
    sym_index = 0 as libc::c_int as libc::c_ulong;
    while sym_index < used {
        if (**used_syms.offset(sym_index as isize)).has_been_placed() as libc::c_int
            == 0 as libc::c_int
        {
            printf(
                b"%s\n\0" as *const u8 as *const libc::c_char,
                (**used_syms.offset(sym_index as isize)).name,
            );
        }
        sym_index = sym_index.wrapping_add(1);
        sym_index;
    }
    sym_index = 0 as libc::c_int as libc::c_ulong;
    while sym_index < unused {
        printf(
            b"%s\n\0" as *const u8 as *const libc::c_char,
            (**unused_syms.offset(sym_index as isize)).name,
        );
        sym_index = sym_index.wrapping_add(1);
        sym_index;
    }
    unused_syms = xmalloc(
        (symtab.len as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut Sym>() as libc::c_ulong),
    ) as *mut *mut Sym;
    used_syms = xmalloc(
        (symtab.len as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut Sym>() as libc::c_ulong),
    ) as *mut *mut Sym;
    scratch_syms = xmalloc(
        (symtab.len as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut Sym>() as libc::c_ulong),
    ) as *mut *mut Sym;
    high_arcs = xmalloc(
        (numarcs as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut Arc>() as libc::c_ulong),
    ) as *mut *mut Arc;
    scratch_arcs = xmalloc(
        (numarcs as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut Arc>() as libc::c_ulong),
    ) as *mut *mut Arc;
    unplaced_arcs = xmalloc(
        (numarcs as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut Arc>() as libc::c_ulong),
    ) as *mut *mut Arc;
    free(unused_syms as *mut libc::c_void);
    free(used_syms as *mut libc::c_void);
    free(scratch_syms as *mut libc::c_void);
    free(high_arcs as *mut libc::c_void);
    free(scratch_arcs as *mut libc::c_void);
    free(unplaced_arcs as *mut libc::c_void);
}
unsafe extern "C" fn print_header() {
    if first_output {
        first_output = 0 as libc::c_int != 0;
    } else {
        printf(b"\x0C\n\0" as *const u8 as *const libc::c_char);
    }
    if !bsd_style_output {
        if print_descriptions {
            printf(
                dcgettext(
                    b"gprof\0" as *const u8 as *const libc::c_char,
                    b"\t\t     Call graph (explanation follows)\n\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        } else {
            printf(
                dcgettext(
                    b"gprof\0" as *const u8 as *const libc::c_char,
                    b"\t\t\tCall graph\n\n\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
    }
    printf(
        dcgettext(
            b"gprof\0" as *const u8 as *const libc::c_char,
            b"\ngranularity: each sample hit covers %ld byte(s)\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
        hist_scale as libc::c_long
            * ::core::mem::size_of::<UNIT>() as libc::c_ulong as libc::c_long,
    );
    if print_time > 0.0f64 {
        printf(
            dcgettext(
                b"gprof\0" as *const u8 as *const libc::c_char,
                b" for %.2f%% of %.2f seconds\n\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            100.0f64 / print_time,
            print_time / hz as libc::c_double,
        );
    } else {
        printf(
            dcgettext(
                b"gprof\0" as *const u8 as *const libc::c_char,
                b" no time propagated\n\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        print_time = 1.0f64;
    }
    if bsd_style_output {
        printf(
            b"%6.6s %5.5s %7.7s %11.11s %7.7s/%-7.7s     %-8.8s\n\0" as *const u8
                as *const libc::c_char,
            b"\0" as *const u8 as *const libc::c_char,
            b"\0" as *const u8 as *const libc::c_char,
            b"\0" as *const u8 as *const libc::c_char,
            b"\0" as *const u8 as *const libc::c_char,
            dcgettext(
                b"gprof\0" as *const u8 as *const libc::c_char,
                b"called\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            dcgettext(
                b"gprof\0" as *const u8 as *const libc::c_char,
                b"total\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            dcgettext(
                b"gprof\0" as *const u8 as *const libc::c_char,
                b"parents\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        printf(
            b"%-6.6s %5.5s %7.7s %11.11s %7.7s+%-7.7s %-8.8s\t%5.5s\n\0" as *const u8
                as *const libc::c_char,
            dcgettext(
                b"gprof\0" as *const u8 as *const libc::c_char,
                b"index\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            dcgettext(
                b"gprof\0" as *const u8 as *const libc::c_char,
                b"%time\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            dcgettext(
                b"gprof\0" as *const u8 as *const libc::c_char,
                b"self\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            dcgettext(
                b"gprof\0" as *const u8 as *const libc::c_char,
                b"descendants\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            dcgettext(
                b"gprof\0" as *const u8 as *const libc::c_char,
                b"called\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            dcgettext(
                b"gprof\0" as *const u8 as *const libc::c_char,
                b"self\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            dcgettext(
                b"gprof\0" as *const u8 as *const libc::c_char,
                b"name\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            dcgettext(
                b"gprof\0" as *const u8 as *const libc::c_char,
                b"index\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        printf(
            b"%6.6s %5.5s %7.7s %11.11s %7.7s/%-7.7s     %-8.8s\n\0" as *const u8
                as *const libc::c_char,
            b"\0" as *const u8 as *const libc::c_char,
            b"\0" as *const u8 as *const libc::c_char,
            b"\0" as *const u8 as *const libc::c_char,
            b"\0" as *const u8 as *const libc::c_char,
            dcgettext(
                b"gprof\0" as *const u8 as *const libc::c_char,
                b"called\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            dcgettext(
                b"gprof\0" as *const u8 as *const libc::c_char,
                b"total\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            dcgettext(
                b"gprof\0" as *const u8 as *const libc::c_char,
                b"children\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        printf(b"\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(
            dcgettext(
                b"gprof\0" as *const u8 as *const libc::c_char,
                b"index %% time    self  children    called     name\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    };
}
unsafe extern "C" fn print_cycle(mut cyc: *mut Sym) {
    let mut buf: [libc::c_char; 8192] = [0; 8192];
    sprintf(
        buf.as_mut_ptr(),
        b"[%d]\0" as *const u8 as *const libc::c_char,
        (*cyc).cg.index,
    );
    printf(
        if bsd_style_output as libc::c_int != 0 {
            b"%-6.6s %5.1f %7.2f %11.2f %7lu\0" as *const u8 as *const libc::c_char
        } else {
            b"%-6.6s %5.1f %7.2f %7.2f %7lu\0" as *const u8 as *const libc::c_char
        },
        buf.as_mut_ptr(),
        100 as libc::c_int as libc::c_double
            * ((*cyc).cg.prop.self_0 + (*cyc).cg.prop.child) / print_time,
        (*cyc).cg.prop.self_0 / hz as libc::c_double,
        (*cyc).cg.prop.child / hz as libc::c_double,
        (*cyc).ncalls,
    );
    if (*cyc).cg.self_calls != 0 as libc::c_int as libc::c_ulong {
        printf(b"+%-7lu\0" as *const u8 as *const libc::c_char, (*cyc).cg.self_calls);
    } else {
        printf(
            b" %7.7s\0" as *const u8 as *const libc::c_char,
            b"\0" as *const u8 as *const libc::c_char,
        );
    }
    printf(
        dcgettext(
            b"gprof\0" as *const u8 as *const libc::c_char,
            b" <cycle %d as a whole> [%d]\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        (*cyc).cg.cyc.num,
        (*cyc).cg.index,
    );
}
unsafe extern "C" fn cmp_member(mut left: *mut Sym, mut right: *mut Sym) -> libc::c_int {
    let mut left_time: libc::c_double = (*left).cg.prop.self_0 + (*left).cg.prop.child;
    let mut right_time: libc::c_double = (*right).cg.prop.self_0
        + (*right).cg.prop.child;
    let mut left_calls: libc::c_ulong = ((*left).ncalls)
        .wrapping_add((*left).cg.self_calls);
    let mut right_calls: libc::c_ulong = ((*right).ncalls)
        .wrapping_add((*right).cg.self_calls);
    if left_time > right_time {
        return 1 as libc::c_int;
    }
    if left_time < right_time {
        return -(1 as libc::c_int);
    }
    if left_calls > right_calls {
        return 1 as libc::c_int;
    }
    if left_calls < right_calls {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn sort_members(mut cyc: *mut Sym) {
    let mut todo: *mut Sym = 0 as *mut Sym;
    let mut doing: *mut Sym = 0 as *mut Sym;
    let mut prev: *mut Sym = 0 as *mut Sym;
    todo = (*cyc).cg.cyc.next;
    (*cyc).cg.cyc.next = 0 as *mut sym;
    doing = todo;
    while !doing.is_null() {
        todo = (*doing).cg.cyc.next;
        prev = cyc;
        while !((*prev).cg.cyc.next).is_null() {
            if cmp_member(doing, (*prev).cg.cyc.next) == 1 as libc::c_int {
                break;
            }
            prev = (*prev).cg.cyc.next;
        }
        (*doing).cg.cyc.next = (*prev).cg.cyc.next;
        (*prev).cg.cyc.next = doing;
        doing = todo;
    }
}
unsafe extern "C" fn print_members(mut cyc: *mut Sym) {
    let mut member: *mut Sym = 0 as *mut Sym;
    sort_members(cyc);
    member = (*cyc).cg.cyc.next;
    while !member.is_null() {
        printf(
            if bsd_style_output as libc::c_int != 0 {
                b"%6.6s %5.5s %7.2f %11.2f %7lu\0" as *const u8 as *const libc::c_char
            } else {
                b"%6.6s %5.5s %7.2f %7.2f %7lu\0" as *const u8 as *const libc::c_char
            },
            b"\0" as *const u8 as *const libc::c_char,
            b"\0" as *const u8 as *const libc::c_char,
            (*member).cg.prop.self_0 / hz as libc::c_double,
            (*member).cg.prop.child / hz as libc::c_double,
            (*member).ncalls,
        );
        if (*member).cg.self_calls != 0 as libc::c_int as libc::c_ulong {
            printf(
                b"+%-7lu\0" as *const u8 as *const libc::c_char,
                (*member).cg.self_calls,
            );
        } else {
            printf(
                b" %7.7s\0" as *const u8 as *const libc::c_char,
                b"\0" as *const u8 as *const libc::c_char,
            );
        }
        printf(b"     \0" as *const u8 as *const libc::c_char);
        print_name(member);
        printf(b"\n\0" as *const u8 as *const libc::c_char);
        member = (*member).cg.cyc.next;
    }
}
unsafe extern "C" fn cmp_arc(mut left: *mut Arc, mut right: *mut Arc) -> libc::c_int {
    let mut left_parent: *mut Sym = (*left).parent;
    let mut left_child: *mut Sym = (*left).child;
    let mut right_parent: *mut Sym = (*right).parent;
    let mut right_child: *mut Sym = (*right).child;
    let mut left_time: libc::c_double = 0.;
    let mut right_time: libc::c_double = 0.;
    if debug_level & (1 as libc::c_int) << 5 as libc::c_int != 0 {
        printf(b"[cmp_arc] \0" as *const u8 as *const libc::c_char);
        print_name(left_parent);
        printf(b" calls \0" as *const u8 as *const libc::c_char);
        print_name(left_child);
        printf(
            b" %f + %f %lu/%lu\n\0" as *const u8 as *const libc::c_char,
            (*left).time,
            (*left).child_time,
            (*left).count,
            (*left_child).ncalls,
        );
        printf(b"[cmp_arc] \0" as *const u8 as *const libc::c_char);
        print_name(right_parent);
        printf(b" calls \0" as *const u8 as *const libc::c_char);
        print_name(right_child);
        printf(
            b" %f + %f %lu/%lu\n\0" as *const u8 as *const libc::c_char,
            (*right).time,
            (*right).child_time,
            (*right).count,
            (*right_child).ncalls,
        );
        printf(b"\n\0" as *const u8 as *const libc::c_char);
    }
    if left_parent == left_child {
        return -(1 as libc::c_int);
    }
    if right_parent == right_child {
        return 1 as libc::c_int;
    }
    if (*left_parent).cg.cyc.num != 0 as libc::c_int
        && (*left_child).cg.cyc.num != 0 as libc::c_int
        && (*left_parent).cg.cyc.num == (*left_child).cg.cyc.num
    {
        if (*right_parent).cg.cyc.num != 0 as libc::c_int
            && (*right_child).cg.cyc.num != 0 as libc::c_int
            && (*right_parent).cg.cyc.num == (*right_child).cg.cyc.num
        {
            if (*left).count < (*right).count {
                return -(1 as libc::c_int);
            }
            if (*left).count > (*right).count {
                return 1 as libc::c_int;
            }
            return 0 as libc::c_int;
        } else {
            return -(1 as libc::c_int)
        }
    } else if (*right_parent).cg.cyc.num != 0 as libc::c_int
        && (*right_child).cg.cyc.num != 0 as libc::c_int
        && (*right_parent).cg.cyc.num == (*right_child).cg.cyc.num
    {
        return 1 as libc::c_int
    } else {
        left_time = (*left).time + (*left).child_time;
        right_time = (*right).time + (*right).child_time;
        if left_time < right_time {
            return -(1 as libc::c_int);
        }
        if left_time > right_time {
            return 1 as libc::c_int;
        }
        if (*left).count < (*right).count {
            return -(1 as libc::c_int);
        }
        if (*left).count > (*right).count {
            return 1 as libc::c_int;
        }
        return 0 as libc::c_int;
    };
}
unsafe extern "C" fn sort_parents(mut child: *mut Sym) {
    let mut arc: *mut Arc = 0 as *mut Arc;
    let mut detached: *mut Arc = 0 as *mut Arc;
    let mut sorted: Arc = Arc {
        parent: 0 as *mut Sym,
        child: 0 as *mut Sym,
        count: 0,
        time: 0.,
        child_time: 0.,
        next_parent: 0 as *mut arc,
        next_child: 0 as *mut arc,
        has_been_placed: 0,
    };
    let mut prev: *mut Arc = 0 as *mut Arc;
    sorted.next_parent = 0 as *mut arc;
    arc = (*child).cg.parents;
    while !arc.is_null() {
        detached = (*arc).next_parent;
        prev = &mut sorted;
        while !((*prev).next_parent).is_null() {
            if cmp_arc(arc, (*prev).next_parent) != 1 as libc::c_int {
                break;
            }
            prev = (*prev).next_parent;
        }
        (*arc).next_parent = (*prev).next_parent;
        (*prev).next_parent = arc;
        arc = detached;
    }
    (*child).cg.parents = sorted.next_parent;
}
unsafe extern "C" fn print_parents(mut child: *mut Sym) {
    let mut parent: *mut Sym = 0 as *mut Sym;
    let mut arc: *mut Arc = 0 as *mut Arc;
    let mut cycle_head: *mut Sym = 0 as *mut Sym;
    if !((*child).cg.cyc.head).is_null() {
        cycle_head = (*child).cg.cyc.head;
    } else {
        cycle_head = child;
    }
    if ((*child).cg.parents).is_null() {
        printf(
            if bsd_style_output as libc::c_int != 0 {
                dcgettext(
                    b"gprof\0" as *const u8 as *const libc::c_char,
                    b"%6.6s %5.5s %7.7s %11.11s %7.7s %7.7s     <spontaneous>\n\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                )
            } else {
                dcgettext(
                    b"gprof\0" as *const u8 as *const libc::c_char,
                    b"%6.6s %5.5s %7.7s %7.7s %7.7s %7.7s     <spontaneous>\n\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                )
            },
            b"\0" as *const u8 as *const libc::c_char,
            b"\0" as *const u8 as *const libc::c_char,
            b"\0" as *const u8 as *const libc::c_char,
            b"\0" as *const u8 as *const libc::c_char,
            b"\0" as *const u8 as *const libc::c_char,
            b"\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    sort_parents(child);
    arc = (*child).cg.parents;
    while !arc.is_null() {
        parent = (*arc).parent;
        if child == parent
            || (*child).cg.cyc.num != 0 as libc::c_int
                && (*parent).cg.cyc.num == (*child).cg.cyc.num
        {
            printf(
                if bsd_style_output as libc::c_int != 0 {
                    b"%6.6s %5.5s %7.7s %11.11s %7lu %7.7s     \0" as *const u8
                        as *const libc::c_char
                } else {
                    b"%6.6s %5.5s %7.7s %7.7s %7lu %7.7s     \0" as *const u8
                        as *const libc::c_char
                },
                b"\0" as *const u8 as *const libc::c_char,
                b"\0" as *const u8 as *const libc::c_char,
                b"\0" as *const u8 as *const libc::c_char,
                b"\0" as *const u8 as *const libc::c_char,
                (*arc).count,
                b"\0" as *const u8 as *const libc::c_char,
            );
            print_name(parent);
            printf(b"\n\0" as *const u8 as *const libc::c_char);
        } else {
            printf(
                if bsd_style_output as libc::c_int != 0 {
                    b"%6.6s %5.5s %7.2f %11.2f %7lu/%-7lu     \0" as *const u8
                        as *const libc::c_char
                } else {
                    b"%6.6s %5.5s %7.2f %7.2f %7lu/%-7lu     \0" as *const u8
                        as *const libc::c_char
                },
                b"\0" as *const u8 as *const libc::c_char,
                b"\0" as *const u8 as *const libc::c_char,
                (*arc).time / hz as libc::c_double,
                (*arc).child_time / hz as libc::c_double,
                (*arc).count,
                (*cycle_head).ncalls,
            );
            print_name(parent);
            printf(b"\n\0" as *const u8 as *const libc::c_char);
        }
        arc = (*arc).next_parent;
    }
}
unsafe extern "C" fn sort_children(mut parent: *mut Sym) {
    let mut arc: *mut Arc = 0 as *mut Arc;
    let mut detached: *mut Arc = 0 as *mut Arc;
    let mut sorted: Arc = Arc {
        parent: 0 as *mut Sym,
        child: 0 as *mut Sym,
        count: 0,
        time: 0.,
        child_time: 0.,
        next_parent: 0 as *mut arc,
        next_child: 0 as *mut arc,
        has_been_placed: 0,
    };
    let mut prev: *mut Arc = 0 as *mut Arc;
    sorted.next_child = 0 as *mut arc;
    arc = (*parent).cg.children;
    while !arc.is_null() {
        detached = (*arc).next_child;
        prev = &mut sorted;
        while !((*prev).next_child).is_null() {
            if cmp_arc(arc, (*prev).next_child) != -(1 as libc::c_int) {
                break;
            }
            prev = (*prev).next_child;
        }
        (*arc).next_child = (*prev).next_child;
        (*prev).next_child = arc;
        arc = detached;
    }
    (*parent).cg.children = sorted.next_child;
}
unsafe extern "C" fn print_children(mut parent: *mut Sym) {
    let mut child: *mut Sym = 0 as *mut Sym;
    let mut arc: *mut Arc = 0 as *mut Arc;
    sort_children(parent);
    arc = (*parent).cg.children;
    arc = (*parent).cg.children;
    while !arc.is_null() {
        child = (*arc).child;
        if child == parent
            || (*child).cg.cyc.num != 0 as libc::c_int
                && (*child).cg.cyc.num == (*parent).cg.cyc.num
        {
            printf(
                if bsd_style_output as libc::c_int != 0 {
                    b"%6.6s %5.5s %7.7s %11.11s %7lu %7.7s     \0" as *const u8
                        as *const libc::c_char
                } else {
                    b"%6.6s %5.5s %7.7s %7.7s %7lu %7.7s     \0" as *const u8
                        as *const libc::c_char
                },
                b"\0" as *const u8 as *const libc::c_char,
                b"\0" as *const u8 as *const libc::c_char,
                b"\0" as *const u8 as *const libc::c_char,
                b"\0" as *const u8 as *const libc::c_char,
                (*arc).count,
                b"\0" as *const u8 as *const libc::c_char,
            );
            print_name(child);
            printf(b"\n\0" as *const u8 as *const libc::c_char);
        } else {
            printf(
                if bsd_style_output as libc::c_int != 0 {
                    b"%6.6s %5.5s %7.2f %11.2f %7lu/%-7lu     \0" as *const u8
                        as *const libc::c_char
                } else {
                    b"%6.6s %5.5s %7.2f %7.2f %7lu/%-7lu     \0" as *const u8
                        as *const libc::c_char
                },
                b"\0" as *const u8 as *const libc::c_char,
                b"\0" as *const u8 as *const libc::c_char,
                (*arc).time / hz as libc::c_double,
                (*arc).child_time / hz as libc::c_double,
                (*arc).count,
                (*(*child).cg.cyc.head).ncalls,
            );
            print_name(child);
            printf(b"\n\0" as *const u8 as *const libc::c_char);
        }
        arc = (*arc).next_child;
    }
}
unsafe extern "C" fn print_line(mut np: *mut Sym) {
    let mut buf: [libc::c_char; 8192] = [0; 8192];
    sprintf(
        buf.as_mut_ptr(),
        b"[%d]\0" as *const u8 as *const libc::c_char,
        (*np).cg.index,
    );
    printf(
        if bsd_style_output as libc::c_int != 0 {
            b"%-6.6s %5.1f %7.2f %11.2f\0" as *const u8 as *const libc::c_char
        } else {
            b"%-6.6s %5.1f %7.2f %7.2f\0" as *const u8 as *const libc::c_char
        },
        buf.as_mut_ptr(),
        100 as libc::c_int as libc::c_double
            * ((*np).cg.prop.self_0 + (*np).cg.prop.child) / print_time,
        (*np).cg.prop.self_0 / hz as libc::c_double,
        (*np).cg.prop.child / hz as libc::c_double,
    );
    if ((*np).ncalls).wrapping_add((*np).cg.self_calls)
        != 0 as libc::c_int as libc::c_ulong
    {
        printf(b" %7lu\0" as *const u8 as *const libc::c_char, (*np).ncalls);
        if (*np).cg.self_calls != 0 as libc::c_int as libc::c_ulong {
            printf(
                b"+%-7lu \0" as *const u8 as *const libc::c_char,
                (*np).cg.self_calls,
            );
        } else {
            printf(
                b" %7.7s \0" as *const u8 as *const libc::c_char,
                b"\0" as *const u8 as *const libc::c_char,
            );
        }
    } else {
        printf(
            b" %7.7s %7.7s \0" as *const u8 as *const libc::c_char,
            b"\0" as *const u8 as *const libc::c_char,
            b"\0" as *const u8 as *const libc::c_char,
        );
    }
    print_name(np);
    printf(b"\n\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn cmp_name(
    mut left: *const libc::c_void,
    mut right: *const libc::c_void,
) -> libc::c_int {
    let mut npp1: *mut *const Sym = left as *mut *const Sym;
    let mut npp2: *mut *const Sym = right as *mut *const Sym;
    return strcmp((**npp1).name, (**npp2).name);
}
unsafe extern "C" fn cmp_arc_count(
    mut left: *const libc::c_void,
    mut right: *const libc::c_void,
) -> libc::c_int {
    let mut npp1: *mut *const Arc = left as *mut *const Arc;
    let mut npp2: *mut *const Arc = right as *mut *const Arc;
    if (**npp1).count > (**npp2).count {
        return -(1 as libc::c_int)
    } else if (**npp1).count < (**npp2).count {
        return 1 as libc::c_int
    } else {
        return 0 as libc::c_int
    };
}
unsafe extern "C" fn cmp_fun_nuses(
    mut left: *const libc::c_void,
    mut right: *const libc::c_void,
) -> libc::c_int {
    let mut npp1: *mut *const Sym = left as *mut *const Sym;
    let mut npp2: *mut *const Sym = right as *mut *const Sym;
    if (**npp1).nuses > (**npp2).nuses {
        return -(1 as libc::c_int)
    } else if (**npp1).nuses < (**npp2).nuses {
        return 1 as libc::c_int
    } else {
        return 0 as libc::c_int
    };
}
unsafe extern "C" fn order_and_dump_functions_by_arcs(
    mut the_arcs: *mut *mut Arc,
    mut arc_count: libc::c_ulong,
    mut all: libc::c_int,
    mut unplaced_arcs: *mut *mut Arc,
    mut unplaced_arc_count: *mut libc::c_ulong,
) {
    let mut tmp_arcs: libc::c_ulonglong = 0;
    let mut total_arcs: libc::c_ulonglong = 0;
    let mut arc_index: libc::c_uint = 0;
    if all == 0 {
        total_arcs = 0 as libc::c_int as libc::c_ulonglong;
        arc_index = 0 as libc::c_int as libc::c_uint;
        while (arc_index as libc::c_ulong) < arc_count {
            total_arcs = total_arcs
                .wrapping_add(
                    (**the_arcs.offset(arc_index as isize)).count as libc::c_ulonglong,
                );
            arc_index = arc_index.wrapping_add(1);
            arc_index;
        }
    } else {
        total_arcs = 0 as libc::c_int as libc::c_ulonglong;
    }
    tmp_arcs = 0 as libc::c_int as libc::c_ulonglong;
    let mut current_block_71: u64;
    arc_index = 0 as libc::c_int as libc::c_uint;
    while (arc_index as libc::c_ulong) < arc_count {
        let mut sym1: *mut Sym = 0 as *mut Sym;
        let mut sym2: *mut Sym = 0 as *mut Sym;
        let mut child: *mut Sym = 0 as *mut Sym;
        let mut parent: *mut Sym = 0 as *mut Sym;
        tmp_arcs = tmp_arcs
            .wrapping_add(
                (**the_arcs.offset(arc_index as isize)).count as libc::c_ulonglong,
            );
        if !((**the_arcs.offset(arc_index as isize)).has_been_placed != 0) {
            child = (**the_arcs.offset(arc_index as isize)).child;
            parent = (**the_arcs.offset(arc_index as isize)).parent;
            if all == 0
                && tmp_arcs as libc::c_double / total_arcs as libc::c_double > 0.99f64
                || (*child).has_been_placed() as libc::c_int != 0
                || (*parent).has_been_placed() as libc::c_int != 0
            {
                let fresh19 = *unplaced_arc_count;
                *unplaced_arc_count = (*unplaced_arc_count).wrapping_add(1);
                let ref mut fresh20 = *unplaced_arcs.offset(fresh19 as isize);
                *fresh20 = *the_arcs.offset(arc_index as isize);
            } else if !((*parent).next).is_null() && !((*parent).prev).is_null()
                && !((*child).next).is_null() && !((*child).prev).is_null()
            {
                let fresh21 = *unplaced_arc_count;
                *unplaced_arc_count = (*unplaced_arc_count).wrapping_add(1);
                let ref mut fresh22 = *unplaced_arcs.offset(fresh21 as isize);
                *fresh22 = *the_arcs.offset(arc_index as isize);
            } else {
                if ((*parent).next).is_null() && ((*parent).prev).is_null() {
                    let mut next_count: libc::c_int = 0 as libc::c_int;
                    let mut prev_count: libc::c_int = 0 as libc::c_int;
                    let mut prev: *mut Sym = child;
                    let mut next: *mut Sym = child;
                    while !((*next).next).is_null() {
                        next = (*next).next;
                        next_count += 1;
                        next_count;
                    }
                    while !((*prev).prev).is_null() {
                        prev = (*prev).prev;
                        prev_count += 1;
                        prev_count;
                    }
                    child = if next_count < prev_count { next } else { prev };
                    current_block_71 = 3123434771885419771;
                } else if ((*child).next).is_null() && ((*child).prev).is_null() {
                    let mut next_count_0: libc::c_int = 0 as libc::c_int;
                    let mut prev_count_0: libc::c_int = 0 as libc::c_int;
                    let mut prev_0: *mut Sym = parent;
                    let mut next_0: *mut Sym = parent;
                    while !((*next_0).next).is_null() {
                        next_0 = (*next_0).next;
                        next_count_0 += 1;
                        next_count_0;
                    }
                    while !((*prev_0).prev).is_null() {
                        prev_0 = (*prev_0).prev;
                        prev_count_0 += 1;
                        prev_count_0;
                    }
                    parent = if prev_count_0 < next_count_0 { prev_0 } else { next_0 };
                    current_block_71 = 3123434771885419771;
                } else {
                    let fresh23 = *unplaced_arc_count;
                    *unplaced_arc_count = (*unplaced_arc_count).wrapping_add(1);
                    let ref mut fresh24 = *unplaced_arcs.offset(fresh23 as isize);
                    *fresh24 = *the_arcs.offset(arc_index as isize);
                    current_block_71 = 10886091980245723256;
                }
                match current_block_71 {
                    10886091980245723256 => {}
                    _ => {
                        sym1 = parent;
                        if !((*sym1).next).is_null() {
                            while !((*sym1).next).is_null() {
                                sym1 = (*sym1).next;
                            }
                        } else {
                            while !((*sym1).prev).is_null() {
                                sym1 = (*sym1).prev;
                            }
                        }
                        sym2 = child;
                        if !((*sym2).next).is_null() {
                            while !((*sym2).next).is_null() {
                                sym2 = (*sym2).next;
                            }
                        } else {
                            while !((*sym2).prev).is_null() {
                                sym2 = (*sym2).prev;
                            }
                        }
                        if sym1 == child && sym2 == parent {
                            let fresh25 = *unplaced_arc_count;
                            *unplaced_arc_count = (*unplaced_arc_count).wrapping_add(1);
                            let ref mut fresh26 = *unplaced_arcs
                                .offset(fresh25 as isize);
                            *fresh26 = *the_arcs.offset(arc_index as isize);
                        } else if !((*parent).next).is_null() {
                            if ((*child).next).is_null() {
                                (*parent).prev = child;
                                (*child).next = parent;
                                (**the_arcs.offset(arc_index as isize))
                                    .has_been_placed = 1 as libc::c_int;
                            }
                        } else if !((*parent).prev).is_null() {
                            if ((*child).prev).is_null() {
                                (*parent).next = child;
                                (*child).prev = parent;
                                (**the_arcs.offset(arc_index as isize))
                                    .has_been_placed = 1 as libc::c_int;
                            }
                        } else if !((*child).prev).is_null() {
                            (*parent).prev = child;
                            (*child).next = parent;
                            (**the_arcs.offset(arc_index as isize))
                                .has_been_placed = 1 as libc::c_int;
                        } else {
                            (*parent).next = child;
                            (*child).prev = parent;
                            (**the_arcs.offset(arc_index as isize))
                                .has_been_placed = 1 as libc::c_int;
                        }
                    }
                }
            }
        }
        arc_index = arc_index.wrapping_add(1);
        arc_index;
    }
    arc_index = 0 as libc::c_int as libc::c_uint;
    while (arc_index as libc::c_ulong) < arc_count {
        let mut sym: *mut Sym = 0 as *mut Sym;
        if !((*(**the_arcs.offset(arc_index as isize)).parent).has_been_placed()
            as libc::c_int != 0
            || (*(**the_arcs.offset(arc_index as isize)).child).has_been_placed()
                as libc::c_int != 0)
        {
            sym = (**the_arcs.offset(arc_index as isize)).parent;
            if !(((*sym).next).is_null() && ((*sym).prev).is_null()) {
                while !((*sym).prev).is_null() {
                    sym = (*sym).prev;
                }
                while !sym.is_null() {
                    (*sym).set_has_been_placed(1 as libc::c_int as libc::c_uint);
                    printf(b"%s\n\0" as *const u8 as *const libc::c_char, (*sym).name);
                    sym = (*sym).next;
                }
            }
        }
        arc_index = arc_index.wrapping_add(1);
        arc_index;
    }
    if all != 0 {
        arc_index = 0 as libc::c_int as libc::c_uint;
        while (arc_index as libc::c_ulong) < arc_count {
            let mut sym_0: *mut Sym = 0 as *mut Sym;
            if !((*(**the_arcs.offset(arc_index as isize)).parent).has_been_placed()
                as libc::c_int != 0
                || (*(**the_arcs.offset(arc_index as isize)).child).has_been_placed()
                    as libc::c_int != 0)
            {
                sym_0 = (**the_arcs.offset(arc_index as isize)).parent;
                (*sym_0).set_has_been_placed(1 as libc::c_int as libc::c_uint);
                printf(b"%s\n\0" as *const u8 as *const libc::c_char, (*sym_0).name);
            }
            arc_index = arc_index.wrapping_add(1);
            arc_index;
        }
    }
}
unsafe extern "C" fn cmp_symbol_map(
    mut l: *const libc::c_void,
    mut r: *const libc::c_void,
) -> libc::c_int {
    return filename_cmp(
        (*(l as *mut function_map)).file_name,
        (*(r as *mut function_map)).file_name,
    );
}
