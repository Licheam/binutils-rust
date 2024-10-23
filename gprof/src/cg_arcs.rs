extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
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
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    static mut debug_level: libc::c_int;
    static mut ignore_direct_calls: bool;
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    static mut symtab: Sym_Table;
    fn sym_init(_: *mut Sym);
    fn sym_lookup(_: *mut Sym_Table, _: bfd_vma) -> *mut Sym;
    fn find_call(_: *mut Sym, _: bfd_vma, _: bfd_vma);
    fn cg_dfn(root: *mut Sym);
    static mut print_time: libc::c_double;
    fn print_name(self_0: *mut Sym);
    static mut syms: [Sym_Table; 12];
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
pub const EXCL_TIME: C2RustUnnamed_3 = 7;
pub const INCL_TIME: C2RustUnnamed_3 = 6;
pub const EXCL_GRAPH: C2RustUnnamed_3 = 1;
pub const INCL_GRAPH: C2RustUnnamed_3 = 0;
pub type C2RustUnnamed_3 = libc::c_uint;
pub const NUM_TABLES: C2RustUnnamed_3 = 12;
pub const EXCL_EXEC: C2RustUnnamed_3 = 11;
pub const INCL_EXEC: C2RustUnnamed_3 = 10;
pub const EXCL_ANNO: C2RustUnnamed_3 = 9;
pub const INCL_ANNO: C2RustUnnamed_3 = 8;
pub const EXCL_FLAT: C2RustUnnamed_3 = 5;
pub const INCL_FLAT: C2RustUnnamed_3 = 4;
pub const EXCL_ARCS: C2RustUnnamed_3 = 3;
pub const INCL_ARCS: C2RustUnnamed_3 = 2;
#[no_mangle]
pub static mut num_cycles: libc::c_uint = 0;
#[no_mangle]
pub static mut cycle_header: *mut Sym = 0 as *const Sym as *mut Sym;
#[no_mangle]
pub unsafe extern "C" fn arc_add(
    mut parent: *mut Sym,
    mut child: *mut Sym,
    mut count: libc::c_ulong,
) {
    static mut maxarcs: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut arc: *mut Arc = 0 as *mut Arc;
    let mut newarcs: *mut *mut Arc = 0 as *mut *mut Arc;
    if debug_level & (1 as libc::c_int) << 4 as libc::c_int != 0 {
        printf(
            b"[arc_add] %lu arcs from %s to %s\n\0" as *const u8 as *const libc::c_char,
            count,
            (*parent).name,
            (*child).name,
        );
    }
    arc = arc_lookup(parent, child);
    if !arc.is_null() {
        if debug_level & (1 as libc::c_int) << 4 as libc::c_int != 0 {
            printf(
                b"[tally] hit %lu += %lu\n\0" as *const u8 as *const libc::c_char,
                (*arc).count,
                count,
            );
        }
        (*arc).count = ((*arc).count).wrapping_add(count);
        return;
    }
    arc = xmalloc(::core::mem::size_of::<Arc>() as libc::c_ulong) as *mut Arc;
    memset(
        arc as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<Arc>() as libc::c_ulong,
    );
    (*arc).parent = parent;
    (*arc).child = child;
    (*arc).count = count;
    if parent != child {
        if numarcs == maxarcs {
            if maxarcs == 0 as libc::c_int as libc::c_uint {
                maxarcs = 1 as libc::c_int as libc::c_uint;
            }
            maxarcs = maxarcs.wrapping_mul(2 as libc::c_int as libc::c_uint);
            newarcs = xmalloc(
                (::core::mem::size_of::<*mut Arc>() as libc::c_ulong)
                    .wrapping_mul(maxarcs as libc::c_ulong),
            ) as *mut *mut Arc;
            memcpy(
                newarcs as *mut libc::c_void,
                arcs as *const libc::c_void,
                (numarcs as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<*mut Arc>() as libc::c_ulong),
            );
            free(arcs as *mut libc::c_void);
            arcs = newarcs;
        }
        let fresh0 = numarcs;
        numarcs = numarcs.wrapping_add(1);
        let ref mut fresh1 = *arcs.offset(fresh0 as isize);
        *fresh1 = arc;
    }
    (*arc).next_child = (*parent).cg.children;
    (*parent).cg.children = arc;
    (*arc).next_parent = (*child).cg.parents;
    (*child).cg.parents = arc;
}
#[no_mangle]
pub unsafe extern "C" fn arc_lookup(
    mut parent: *mut Sym,
    mut child: *mut Sym,
) -> *mut Arc {
    let mut arc: *mut Arc = 0 as *mut Arc;
    if parent.is_null() || child.is_null() {
        printf(
            b"[arc_lookup] parent == 0 || child == 0\n\0" as *const u8
                as *const libc::c_char,
        );
        return 0 as *mut Arc;
    }
    if debug_level & (1 as libc::c_int) << 9 as libc::c_int != 0 {
        printf(
            b"[arc_lookup] parent %s child %s\n\0" as *const u8 as *const libc::c_char,
            (*parent).name,
            (*child).name,
        );
    }
    arc = (*parent).cg.children;
    while !arc.is_null() {
        if debug_level & (1 as libc::c_int) << 9 as libc::c_int != 0 {
            printf(
                b"[arc_lookup]\t parent %s child %s\n\0" as *const u8
                    as *const libc::c_char,
                (*(*arc).parent).name,
                (*(*arc).child).name,
            );
        }
        if (*child).addr >= (*(*arc).child).addr
            && (*child).end_addr <= (*(*arc).child).end_addr
        {
            return arc;
        }
        arc = (*arc).next_child;
    }
    return 0 as *mut Arc;
}
#[no_mangle]
pub unsafe extern "C" fn cg_assemble() -> *mut *mut Sym {
    let mut parent: *mut Sym = 0 as *mut Sym;
    let mut time_sorted_syms: *mut *mut Sym = 0 as *mut *mut Sym;
    let mut top_sorted_syms: *mut *mut Sym = 0 as *mut *mut Sym;
    let mut sym_index: libc::c_uint = 0;
    let mut arc: *mut Arc = 0 as *mut Arc;
    parent = symtab.base;
    while parent < symtab.limit {
        (*parent).cg.child_time = 0.0f64;
        arc = arc_lookup(parent, parent);
        if !arc.is_null() && parent == (*arc).child {
            (*parent).ncalls = ((*parent).ncalls).wrapping_sub((*arc).count);
            (*parent).cg.self_calls = (*arc).count;
        } else {
            (*parent).cg.self_calls = 0 as libc::c_int as libc::c_ulong;
        }
        (*parent).cg.prop.fract = 0.0f64;
        (*parent).cg.prop.self_0 = 0.0f64;
        (*parent).cg.prop.child = 0.0f64;
        (*parent).cg.print_flag = 0 as libc::c_int != 0;
        (*parent).cg.top_order = 0 as libc::c_int;
        (*parent).cg.cyc.num = 0 as libc::c_int;
        (*parent).cg.cyc.head = parent;
        (*parent).cg.cyc.next = 0 as *mut sym;
        if ignore_direct_calls {
            find_call(
                parent,
                (*parent).addr,
                (*parent.offset(1 as libc::c_int as isize)).addr,
            );
        }
        parent = parent.offset(1);
        parent;
    }
    parent = symtab.base;
    while parent < symtab.limit {
        if (*parent).cg.top_order == 0 as libc::c_int {
            cg_dfn(parent);
        }
        parent = parent.offset(1);
        parent;
    }
    cycle_link();
    top_sorted_syms = xmalloc(
        (symtab.len as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut Sym>() as libc::c_ulong),
    ) as *mut *mut Sym;
    sym_index = 0 as libc::c_int as libc::c_uint;
    while sym_index < symtab.len {
        let ref mut fresh2 = *top_sorted_syms.offset(sym_index as isize);
        *fresh2 = &mut *(symtab.base).offset(sym_index as isize) as *mut Sym;
        sym_index = sym_index.wrapping_add(1);
        sym_index;
    }
    qsort(
        top_sorted_syms as *mut libc::c_void,
        symtab.len as size_t,
        ::core::mem::size_of::<*mut Sym>() as libc::c_ulong,
        Some(
            cmp_topo
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    if debug_level & (1 as libc::c_int) << 1 as libc::c_int != 0 {
        printf(
            b"[cg_assemble] topological sort listing\n\0" as *const u8
                as *const libc::c_char,
        );
        sym_index = 0 as libc::c_int as libc::c_uint;
        while sym_index < symtab.len {
            printf(b"[cg_assemble] \0" as *const u8 as *const libc::c_char);
            printf(
                b"%d:\0" as *const u8 as *const libc::c_char,
                (**top_sorted_syms.offset(sym_index as isize)).cg.top_order,
            );
            print_name(*top_sorted_syms.offset(sym_index as isize));
            printf(b"\n\0" as *const u8 as *const libc::c_char);
            sym_index = sym_index.wrapping_add(1);
            sym_index;
        }
    }
    propagate_flags(top_sorted_syms);
    cycle_time();
    sym_index = 0 as libc::c_int as libc::c_uint;
    while sym_index < symtab.len {
        propagate_time(*top_sorted_syms.offset(sym_index as isize));
        sym_index = sym_index.wrapping_add(1);
        sym_index;
    }
    free(top_sorted_syms as *mut libc::c_void);
    time_sorted_syms = xmalloc(
        ((symtab.len).wrapping_add(num_cycles) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut Sym>() as libc::c_ulong),
    ) as *mut *mut Sym;
    sym_index = 0 as libc::c_int as libc::c_uint;
    while sym_index < symtab.len {
        let ref mut fresh3 = *time_sorted_syms.offset(sym_index as isize);
        *fresh3 = &mut *(symtab.base).offset(sym_index as isize) as *mut Sym;
        sym_index = sym_index.wrapping_add(1);
        sym_index;
    }
    sym_index = 1 as libc::c_int as libc::c_uint;
    while sym_index <= num_cycles {
        let ref mut fresh4 = *time_sorted_syms
            .offset(
                (symtab.len)
                    .wrapping_add(sym_index)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
            );
        *fresh4 = &mut *cycle_header.offset(sym_index as isize) as *mut Sym;
        sym_index = sym_index.wrapping_add(1);
        sym_index;
    }
    qsort(
        time_sorted_syms as *mut libc::c_void,
        (symtab.len).wrapping_add(num_cycles) as size_t,
        ::core::mem::size_of::<*mut Sym>() as libc::c_ulong,
        Some(
            cmp_total
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    sym_index = 0 as libc::c_int as libc::c_uint;
    while sym_index < (symtab.len).wrapping_add(num_cycles) {
        (**time_sorted_syms.offset(sym_index as isize))
            .cg
            .index = sym_index.wrapping_add(1 as libc::c_int as libc::c_uint)
            as libc::c_int;
        sym_index = sym_index.wrapping_add(1);
        sym_index;
    }
    return time_sorted_syms;
}
#[no_mangle]
pub static mut arcs: *mut *mut Arc = 0 as *const *mut Arc as *mut *mut Arc;
#[no_mangle]
pub static mut numarcs: libc::c_uint = 0;
unsafe extern "C" fn cmp_topo(
    mut lp: *const libc::c_void,
    mut rp: *const libc::c_void,
) -> libc::c_int {
    let mut left: *const Sym = *(lp as *mut *const Sym);
    let mut right: *const Sym = *(rp as *mut *const Sym);
    return (*left).cg.top_order - (*right).cg.top_order;
}
unsafe extern "C" fn propagate_time(mut parent: *mut Sym) {
    let mut arc: *mut Arc = 0 as *mut Arc;
    let mut child: *mut Sym = 0 as *mut Sym;
    let mut share: libc::c_double = 0.;
    let mut prop_share: libc::c_double = 0.;
    if (*parent).cg.prop.fract == 0.0f64 {
        return;
    }
    let mut current_block_30: u64;
    arc = (*parent).cg.children;
    while !arc.is_null() {
        child = (*arc).child;
        if !((*arc).count == 0 as libc::c_int as libc::c_ulong || child == parent
            || (*child).cg.prop.fract == 0 as libc::c_int as libc::c_double)
        {
            if (*child).cg.cyc.head != child {
                if (*parent).cg.cyc.num == (*child).cg.cyc.num {
                    current_block_30 = 4644295000439058019;
                } else {
                    if (*parent).cg.top_order <= (*child).cg.top_order {
                        fprintf(
                            stderr,
                            b"[propagate] toporder botches\n\0" as *const u8
                                as *const libc::c_char,
                        );
                    }
                    child = (*child).cg.cyc.head;
                    current_block_30 = 9606288038608642794;
                }
            } else if (*parent).cg.top_order <= (*child).cg.top_order {
                fprintf(
                    stderr,
                    b"[propagate] toporder botches\n\0" as *const u8
                        as *const libc::c_char,
                );
                current_block_30 = 4644295000439058019;
            } else {
                current_block_30 = 9606288038608642794;
            }
            match current_block_30 {
                4644295000439058019 => {}
                _ => {
                    if !((*child).ncalls == 0 as libc::c_int as libc::c_ulong) {
                        (*arc)
                            .time = (*child).hist.time
                            * ((*arc).count as libc::c_double
                                / (*child).ncalls as libc::c_double);
                        (*arc)
                            .child_time = (*child).cg.child_time
                            * ((*arc).count as libc::c_double
                                / (*child).ncalls as libc::c_double);
                        share = (*arc).time + (*arc).child_time;
                        (*parent).cg.child_time += share;
                        prop_share = (*parent).cg.prop.fract * share;
                        (*parent).cg.prop.child += prop_share;
                        (*arc).time *= (*parent).cg.prop.fract;
                        (*arc).child_time *= (*parent).cg.prop.fract;
                        if (*parent).cg.cyc.head != parent {
                            (*(*parent).cg.cyc.head).cg.child_time += share;
                            (*(*parent).cg.cyc.head).cg.prop.child += prop_share;
                        }
                        if debug_level & (1 as libc::c_int) << 10 as libc::c_int != 0 {
                            printf(
                                b"[prop_time] child \t\0" as *const u8
                                    as *const libc::c_char,
                            );
                            print_name(child);
                            printf(
                                b" with %f %f %lu/%lu\n\0" as *const u8
                                    as *const libc::c_char,
                                (*child).hist.time,
                                (*child).cg.child_time,
                                (*arc).count,
                                (*child).ncalls,
                            );
                            printf(
                                b"[prop_time] parent\t\0" as *const u8
                                    as *const libc::c_char,
                            );
                            print_name(parent);
                            printf(
                                b"\n[prop_time] share %f\n\0" as *const u8
                                    as *const libc::c_char,
                                share,
                            );
                        }
                    }
                }
            }
        }
        arc = (*arc).next_child;
    }
}
unsafe extern "C" fn cycle_time() {
    let mut member: *mut Sym = 0 as *mut Sym;
    let mut cyc: *mut Sym = 0 as *mut Sym;
    cyc = &mut *cycle_header.offset(1 as libc::c_int as isize) as *mut Sym;
    while cyc <= &mut *cycle_header.offset(num_cycles as isize) as *mut Sym {
        member = (*cyc).cg.cyc.next;
        while !member.is_null() {
            if !((*member).cg.prop.fract == 0.0f64) {
                (*cyc).hist.time += (*member).hist.time;
            }
            member = (*member).cg.cyc.next;
        }
        (*cyc).cg.prop.self_0 = (*cyc).cg.prop.fract * (*cyc).hist.time;
        cyc = cyc.offset(1);
        cyc;
    }
}
unsafe extern "C" fn cycle_link() {
    let mut sym: *mut Sym = 0 as *mut Sym;
    let mut cyc: *mut Sym = 0 as *mut Sym;
    let mut member: *mut Sym = 0 as *mut Sym;
    let mut arc: *mut Arc = 0 as *mut Arc;
    let mut num: libc::c_int = 0;
    num_cycles = 0 as libc::c_int as libc::c_uint;
    sym = symtab.base;
    while sym < symtab.limit {
        if (*sym).cg.cyc.head == sym && !((*sym).cg.cyc.next).is_null() {
            num_cycles = num_cycles.wrapping_add(1);
            num_cycles;
        }
        sym = sym.offset(1);
        sym;
    }
    cycle_header = xmalloc(
        (num_cycles.wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<Sym>() as libc::c_ulong),
    ) as *mut Sym;
    num = 0 as libc::c_int;
    cyc = cycle_header;
    sym = symtab.base;
    while sym < symtab.limit {
        if (*sym).cg.cyc.head == sym && !((*sym).cg.cyc.next).is_null() {
            num += 1;
            num;
            cyc = cyc.offset(1);
            cyc;
            sym_init(cyc);
            (*cyc).cg.print_flag = 1 as libc::c_int != 0;
            (*cyc).cg.top_order = 0 as libc::c_int;
            (*cyc).cg.cyc.num = num;
            (*cyc).cg.cyc.head = cyc;
            (*cyc).cg.cyc.next = sym;
            if debug_level & (1 as libc::c_int) << 2 as libc::c_int != 0 {
                printf(b"[cycle_link] \0" as *const u8 as *const libc::c_char);
                print_name(sym);
                printf(
                    b" is the head of cycle %d\n\0" as *const u8 as *const libc::c_char,
                    num,
                );
            }
            member = sym;
            while !member.is_null() {
                (*member).cg.cyc.num = num;
                (*member).cg.cyc.head = cyc;
                member = (*member).cg.cyc.next;
            }
            member = sym;
            while !member.is_null() {
                arc = (*member).cg.parents;
                while !arc.is_null() {
                    if !((*arc).parent == member) {
                        if (*(*arc).parent).cg.cyc.num == num {
                            (*cyc)
                                .cg
                                .self_calls = ((*cyc).cg.self_calls)
                                .wrapping_add((*arc).count);
                        } else {
                            (*cyc).ncalls = ((*cyc).ncalls).wrapping_add((*arc).count);
                        }
                    }
                    arc = (*arc).next_parent;
                }
                member = (*member).cg.cyc.next;
            }
        }
        sym = sym.offset(1);
        sym;
    }
}
unsafe extern "C" fn inherit_flags(mut child: *mut Sym) {
    let mut head: *mut Sym = 0 as *mut Sym;
    let mut parent: *mut Sym = 0 as *mut Sym;
    let mut member: *mut Sym = 0 as *mut Sym;
    let mut arc: *mut Arc = 0 as *mut Arc;
    head = (*child).cg.cyc.head;
    if child == head {
        (*child).cg.print_flag = 0 as libc::c_int != 0;
        (*child).cg.prop.fract = 0.0f64;
        arc = (*child).cg.parents;
        while !arc.is_null() {
            parent = (*arc).parent;
            if !(child == parent) {
                (*child)
                    .cg
                    .print_flag = ((*child).cg.print_flag as libc::c_int
                    | (*parent).cg.print_flag as libc::c_int) != 0;
                if (*child).ncalls != 0 as libc::c_int as libc::c_ulong {
                    (*child).cg.prop.fract
                        += (*parent).cg.prop.fract
                            * ((*arc).count as libc::c_double
                                / (*child).ncalls as libc::c_double);
                }
            }
            arc = (*arc).next_parent;
        }
    } else {
        (*head).cg.print_flag = 0 as libc::c_int != 0;
        (*head).cg.prop.fract = 0.0f64;
        member = (*head).cg.cyc.next;
        while !member.is_null() {
            arc = (*member).cg.parents;
            while !arc.is_null() {
                if !((*(*arc).parent).cg.cyc.head == head) {
                    parent = (*arc).parent;
                    (*head)
                        .cg
                        .print_flag = ((*head).cg.print_flag as libc::c_int
                        | (*parent).cg.print_flag as libc::c_int) != 0;
                    if (*head).ncalls != 0 as libc::c_int as libc::c_ulong {
                        (*head).cg.prop.fract
                            += (*parent).cg.prop.fract
                                * ((*arc).count as libc::c_double
                                    / (*head).ncalls as libc::c_double);
                    }
                }
                arc = (*arc).next_parent;
            }
            member = (*member).cg.cyc.next;
        }
        member = head;
        while !member.is_null() {
            (*member).cg.print_flag = (*head).cg.print_flag;
            (*member).cg.prop.fract = (*head).cg.prop.fract;
            member = (*member).cg.cyc.next;
        }
    };
}
unsafe extern "C" fn propagate_flags(mut symbols: *mut *mut Sym) {
    let mut sym_index: libc::c_int = 0;
    let mut old_head: *mut Sym = 0 as *mut Sym;
    let mut child: *mut Sym = 0 as *mut Sym;
    old_head = 0 as *mut Sym;
    sym_index = (symtab.len).wrapping_sub(1 as libc::c_int as libc::c_uint)
        as libc::c_int;
    while sym_index >= 0 as libc::c_int {
        child = *symbols.offset(sym_index as isize);
        if (*child).cg.cyc.head != old_head {
            old_head = (*child).cg.cyc.head;
            inherit_flags(child);
        }
        if debug_level & (1 as libc::c_int) << 10 as libc::c_int != 0 {
            printf(b"[prop_flags] \0" as *const u8 as *const libc::c_char);
            print_name(child);
            printf(
                b"inherits print-flag %d and prop-fract %f\n\0" as *const u8
                    as *const libc::c_char,
                (*child).cg.print_flag as libc::c_int,
                (*child).cg.prop.fract,
            );
        }
        if !(*child).cg.print_flag {
            if !(sym_lookup(
                &mut *syms.as_mut_ptr().offset(INCL_GRAPH as libc::c_int as isize),
                (*child).addr,
            ))
                .is_null()
                || syms[INCL_GRAPH as libc::c_int as usize].len
                    == 0 as libc::c_int as libc::c_uint
                    && (sym_lookup(
                        &mut *syms
                            .as_mut_ptr()
                            .offset(EXCL_GRAPH as libc::c_int as isize),
                        (*child).addr,
                    ))
                        .is_null()
            {
                (*child).cg.print_flag = 1 as libc::c_int != 0;
            }
        } else if (sym_lookup(
            &mut *syms.as_mut_ptr().offset(INCL_GRAPH as libc::c_int as isize),
            (*child).addr,
        ))
            .is_null()
            && !(sym_lookup(
                &mut *syms.as_mut_ptr().offset(EXCL_GRAPH as libc::c_int as isize),
                (*child).addr,
            ))
                .is_null()
        {
            (*child).cg.print_flag = 0 as libc::c_int != 0;
        }
        if (*child).cg.prop.fract == 0.0f64 {
            if !(sym_lookup(
                &mut *syms.as_mut_ptr().offset(INCL_TIME as libc::c_int as isize),
                (*child).addr,
            ))
                .is_null()
                || syms[INCL_TIME as libc::c_int as usize].len
                    == 0 as libc::c_int as libc::c_uint
                    && (sym_lookup(
                        &mut *syms
                            .as_mut_ptr()
                            .offset(EXCL_TIME as libc::c_int as isize),
                        (*child).addr,
                    ))
                        .is_null()
            {
                (*child).cg.prop.fract = 1.0f64;
            }
        } else if (sym_lookup(
            &mut *syms.as_mut_ptr().offset(INCL_TIME as libc::c_int as isize),
            (*child).addr,
        ))
            .is_null()
            && !(sym_lookup(
                &mut *syms.as_mut_ptr().offset(EXCL_TIME as libc::c_int as isize),
                (*child).addr,
            ))
                .is_null()
        {
            (*child).cg.prop.fract = 0.0f64;
        }
        (*child).cg.prop.self_0 = (*child).hist.time * (*child).cg.prop.fract;
        print_time += (*child).cg.prop.self_0;
        if debug_level & (1 as libc::c_int) << 10 as libc::c_int != 0 {
            printf(b"[prop_flags] \0" as *const u8 as *const libc::c_char);
            print_name(child);
            printf(
                b" ends up with printflag %d and prop-fract %f\n\0" as *const u8
                    as *const libc::c_char,
                (*child).cg.print_flag as libc::c_int,
                (*child).cg.prop.fract,
            );
            printf(
                b"[prop_flags] time %f propself %f print_time %f\n\0" as *const u8
                    as *const libc::c_char,
                (*child).hist.time,
                (*child).cg.prop.self_0,
                print_time,
            );
        }
        sym_index -= 1;
        sym_index;
    }
}
unsafe extern "C" fn cmp_total(
    mut lp: *const libc::c_void,
    mut rp: *const libc::c_void,
) -> libc::c_int {
    let mut left: *const Sym = *(lp as *mut *const Sym);
    let mut right: *const Sym = *(rp as *mut *const Sym);
    let mut diff: libc::c_double = 0.;
    diff = (*left).cg.prop.self_0 + (*left).cg.prop.child
        - ((*right).cg.prop.self_0 + (*right).cg.prop.child);
    if diff < 0.0f64 {
        return 1 as libc::c_int;
    }
    if diff > 0.0f64 {
        return -(1 as libc::c_int);
    }
    if ((*left).name).is_null() && (*left).cg.cyc.num != 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if ((*right).name).is_null() && (*right).cg.cyc.num != 0 as libc::c_int {
        return 1 as libc::c_int;
    }
    if ((*left).name).is_null() {
        return -(1 as libc::c_int);
    }
    if ((*right).name).is_null() {
        return 1 as libc::c_int;
    }
    if *((*left).name).offset(0 as libc::c_int as isize) as libc::c_int != '_' as i32
        && *((*right).name).offset(0 as libc::c_int as isize) as libc::c_int
            == '_' as i32
    {
        return -(1 as libc::c_int);
    }
    if *((*left).name).offset(0 as libc::c_int as isize) as libc::c_int == '_' as i32
        && *((*right).name).offset(0 as libc::c_int as isize) as libc::c_int
            != '_' as i32
    {
        return 1 as libc::c_int;
    }
    if (*left).ncalls > (*right).ncalls {
        return -(1 as libc::c_int);
    }
    if (*left).ncalls < (*right).ncalls {
        return 1 as libc::c_int;
    }
    return strcmp((*left).name, (*right).name);
}
