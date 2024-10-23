extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn done(status: libc::c_int) -> !;
    static mut debug_level: libc::c_int;
    fn xrealloc(_: *mut libc::c_void, _: size_t) -> *mut libc::c_void;
    fn print_name(self_0: *mut Sym);
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
pub type Arc = arc;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DFN_Stack {
    pub sym: *mut Sym,
    pub cycle_top: libc::c_int,
}
#[no_mangle]
pub unsafe extern "C" fn cg_dfn(mut parent: *mut Sym) {
    let mut arc: *mut Arc = 0 as *mut Arc;
    if debug_level & (1 as libc::c_int) << 1 as libc::c_int != 0 {
        printf(b"[dfn] dfn( \0" as *const u8 as *const libc::c_char);
        print_name(parent);
        printf(b")\n\0" as *const u8 as *const libc::c_char);
    }
    if is_numbered(parent) {
        return;
    }
    if is_busy(parent) {
        find_cycle(parent);
        return;
    }
    pre_visit(parent);
    arc = (*parent).cg.children;
    while !arc.is_null() {
        cg_dfn((*arc).child);
        arc = (*arc).next_child;
    }
    post_visit(parent);
}
unsafe extern "C" fn is_numbered(mut child: *mut Sym) -> bool {
    return (*child).cg.top_order != 0 as libc::c_int
        && (*child).cg.top_order != -(1 as libc::c_int);
}
unsafe extern "C" fn is_busy(mut child: *mut Sym) -> bool {
    if (*child).cg.top_order == 0 as libc::c_int {
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn find_cycle(mut child: *mut Sym) {
    let mut head: *mut Sym = 0 as *mut Sym;
    let mut tail: *mut Sym = 0 as *mut Sym;
    let mut cycle_top: libc::c_int = 0;
    let mut cycle_index: libc::c_int = 0;
    cycle_top = dfn_depth;
    while cycle_top > 0 as libc::c_int {
        head = (*dfn_stack.offset(cycle_top as isize)).sym;
        if child == head {
            break;
        }
        if (*child).cg.cyc.head != child && (*child).cg.cyc.head == head {
            break;
        }
        cycle_top -= 1;
        cycle_top;
    }
    if cycle_top <= 0 as libc::c_int {
        fprintf(
            stderr,
            b"[find_cycle] couldn't find head of cycle\n\0" as *const u8
                as *const libc::c_char,
        );
        done(1 as libc::c_int);
    }
    if debug_level & (1 as libc::c_int) << 1 as libc::c_int != 0 {
        printf(
            b"[find_cycle] dfn_depth %d cycle_top %d \0" as *const u8
                as *const libc::c_char,
            dfn_depth,
            cycle_top,
        );
        if !head.is_null() {
            print_name(head);
        } else {
            printf(b"<unknown>\0" as *const u8 as *const libc::c_char);
        }
        printf(b"\n\0" as *const u8 as *const libc::c_char);
    }
    if cycle_top == dfn_depth {
        if debug_level & (1 as libc::c_int) << 1 as libc::c_int != 0 {
            printf(b"[find_cycle] \0" as *const u8 as *const libc::c_char);
            print_name(child);
            printf(b"\n\0" as *const u8 as *const libc::c_char);
        }
    } else {
        tail = head;
        while !((*tail).cg.cyc.next).is_null() {
            if debug_level & (1 as libc::c_int) << 1 as libc::c_int != 0 {
                printf(b"[find_cycle] tail \0" as *const u8 as *const libc::c_char);
                print_name(tail);
                printf(b"\n\0" as *const u8 as *const libc::c_char);
            }
            tail = (*tail).cg.cyc.next;
        }
        if (*head).cg.cyc.head != head {
            head = (*head).cg.cyc.head;
            if debug_level & (1 as libc::c_int) << 1 as libc::c_int != 0 {
                printf(
                    b"[find_cycle] new cyclehead \0" as *const u8 as *const libc::c_char,
                );
                print_name(head);
                printf(b"\n\0" as *const u8 as *const libc::c_char);
            }
        }
        cycle_index = cycle_top + 1 as libc::c_int;
        while cycle_index <= dfn_depth {
            child = (*dfn_stack.offset(cycle_index as isize)).sym;
            if (*child).cg.cyc.head == child {
                (*tail).cg.cyc.next = child;
                (*child).cg.cyc.head = head;
                if debug_level & (1 as libc::c_int) << 1 as libc::c_int != 0 {
                    printf(
                        b"[find_cycle] glomming \0" as *const u8 as *const libc::c_char,
                    );
                    print_name(child);
                    printf(b" onto \0" as *const u8 as *const libc::c_char);
                    print_name(head);
                    printf(b"\n\0" as *const u8 as *const libc::c_char);
                }
                tail = child;
                while !((*tail).cg.cyc.next).is_null() {
                    (*(*tail).cg.cyc.next).cg.cyc.head = head;
                    if debug_level & (1 as libc::c_int) << 1 as libc::c_int != 0 {
                        printf(
                            b"[find_cycle] and its tail \0" as *const u8
                                as *const libc::c_char,
                        );
                        print_name((*tail).cg.cyc.next);
                        printf(b" onto \0" as *const u8 as *const libc::c_char);
                        print_name(head);
                        printf(b"\n\0" as *const u8 as *const libc::c_char);
                    }
                    tail = (*tail).cg.cyc.next;
                }
            } else if (*child).cg.cyc.head != head {
                fprintf(
                    stderr,
                    b"[find_cycle] glommed, but not to head\n\0" as *const u8
                        as *const libc::c_char,
                );
                done(1 as libc::c_int);
            }
            cycle_index += 1;
            cycle_index;
        }
    };
}
unsafe extern "C" fn pre_visit(mut parent: *mut Sym) {
    dfn_depth += 1;
    dfn_depth;
    if dfn_depth >= dfn_maxdepth {
        dfn_maxdepth += 128 as libc::c_int;
        dfn_stack = xrealloc(
            dfn_stack as *mut libc::c_void,
            (dfn_maxdepth as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<DFN_Stack>() as libc::c_ulong),
        ) as *mut DFN_Stack;
    }
    let ref mut fresh0 = (*dfn_stack.offset(dfn_depth as isize)).sym;
    *fresh0 = parent;
    (*dfn_stack.offset(dfn_depth as isize)).cycle_top = dfn_depth;
    (*parent).cg.top_order = -(1 as libc::c_int);
    if debug_level & (1 as libc::c_int) << 1 as libc::c_int != 0 {
        printf(b"[pre_visit]\t\t%d:\0" as *const u8 as *const libc::c_char, dfn_depth);
        print_name(parent);
        printf(b"\n\0" as *const u8 as *const libc::c_char);
    }
}
unsafe extern "C" fn post_visit(mut parent: *mut Sym) {
    let mut member: *mut Sym = 0 as *mut Sym;
    if debug_level & (1 as libc::c_int) << 1 as libc::c_int != 0 {
        printf(b"[post_visit]\t%d: \0" as *const u8 as *const libc::c_char, dfn_depth);
        print_name(parent);
        printf(b"\n\0" as *const u8 as *const libc::c_char);
    }
    if (*parent).cg.cyc.head == parent {
        dfn_counter += 1;
        dfn_counter;
        member = parent;
        while !member.is_null() {
            (*member).cg.top_order = dfn_counter;
            if debug_level & (1 as libc::c_int) << 1 as libc::c_int != 0 {
                printf(b"[post_visit]\t\tmember \0" as *const u8 as *const libc::c_char);
                print_name(member);
                printf(
                    b"-> cg.top_order = %d\n\0" as *const u8 as *const libc::c_char,
                    dfn_counter,
                );
            }
            member = (*member).cg.cyc.next;
        }
    } else if debug_level & (1 as libc::c_int) << 1 as libc::c_int != 0 {
        printf(
            b"[post_visit]\t\tis part of a cycle\n\0" as *const u8 as *const libc::c_char,
        );
    }
    dfn_depth -= 1;
    dfn_depth;
}
#[no_mangle]
pub static mut dfn_stack: *mut DFN_Stack = 0 as *const DFN_Stack as *mut DFN_Stack;
#[no_mangle]
pub static mut dfn_maxdepth: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut dfn_depth: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut dfn_counter: libc::c_int = 0 as libc::c_int;
