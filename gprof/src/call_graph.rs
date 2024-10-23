extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
    fn done(status: libc::c_int) -> !;
    static mut stderr: *mut FILE;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    static mut whoami: *const libc::c_char;
    static mut debug_level: libc::c_int;
    static mut symtab: Sym_Table;
    fn sym_lookup(_: *mut Sym_Table, _: bfd_vma) -> *mut Sym;
    fn arc_add(parent: *mut Sym, child: *mut Sym, count: libc::c_ulong);
    fn gmon_io_read_vma(ifp: *mut FILE, valp: *mut bfd_vma) -> libc::c_int;
    fn gmon_io_read_32(ifp: *mut FILE, valp: *mut libc::c_uint) -> libc::c_int;
    fn gmon_io_write_vma(ifp: *mut FILE, val: bfd_vma) -> libc::c_int;
    fn gmon_io_write_32(ifp: *mut FILE, val: libc::c_uint) -> libc::c_int;
    fn gmon_io_write_8(ifp: *mut FILE, val: libc::c_uint) -> libc::c_int;
    static mut syms: [Sym_Table; 12];
    fn sym_id_arc_is_present(_: *mut Sym_Table, _: *mut Sym, _: *mut Sym) -> bool;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Sym_Table {
    pub len: libc::c_uint,
    pub base: *mut Sym,
    pub limit: *mut Sym,
}
pub type Arc = arc;
pub const EXCL_ARCS: C2RustUnnamed_4 = 3;
pub const INCL_ARCS: C2RustUnnamed_4 = 2;
pub const GMON_TAG_CG_ARC: C2RustUnnamed_3 = 1;
pub type C2RustUnnamed_3 = libc::c_uint;
pub const GMON_TAG_BB_COUNT: C2RustUnnamed_3 = 2;
pub const GMON_TAG_TIME_HIST: C2RustUnnamed_3 = 0;
pub type C2RustUnnamed_4 = libc::c_uint;
pub const NUM_TABLES: C2RustUnnamed_4 = 12;
pub const EXCL_EXEC: C2RustUnnamed_4 = 11;
pub const INCL_EXEC: C2RustUnnamed_4 = 10;
pub const EXCL_ANNO: C2RustUnnamed_4 = 9;
pub const INCL_ANNO: C2RustUnnamed_4 = 8;
pub const EXCL_TIME: C2RustUnnamed_4 = 7;
pub const INCL_TIME: C2RustUnnamed_4 = 6;
pub const EXCL_FLAT: C2RustUnnamed_4 = 5;
pub const INCL_FLAT: C2RustUnnamed_4 = 4;
pub const EXCL_GRAPH: C2RustUnnamed_4 = 1;
pub const INCL_GRAPH: C2RustUnnamed_4 = 0;
#[no_mangle]
pub unsafe extern "C" fn cg_tally(
    mut from_pc: bfd_vma,
    mut self_pc: bfd_vma,
    mut count: libc::c_ulong,
) {
    let mut parent: *mut Sym = 0 as *mut Sym;
    let mut child: *mut Sym = 0 as *mut Sym;
    parent = sym_lookup(&mut symtab, from_pc);
    child = sym_lookup(&mut symtab, self_pc);
    if child.is_null() || parent.is_null() {
        return;
    }
    while child >= symtab.base && (*child).is_func() == 0 {
        child = child.offset(-1);
        child;
    }
    if child < symtab.base {
        return;
    }
    if sym_id_arc_is_present(
        &mut *syms.as_mut_ptr().offset(INCL_ARCS as libc::c_int as isize),
        parent,
        child,
    ) as libc::c_int != 0
        || syms[INCL_ARCS as libc::c_int as usize].len
            == 0 as libc::c_int as libc::c_uint
            && !sym_id_arc_is_present(
                &mut *syms.as_mut_ptr().offset(EXCL_ARCS as libc::c_int as isize),
                parent,
                child,
            )
    {
        (*child).ncalls = ((*child).ncalls).wrapping_add(count);
        if debug_level & (1 as libc::c_int) << 4 as libc::c_int != 0 {
            printf(
                dcgettext(
                    b"gprof\0" as *const u8 as *const libc::c_char,
                    b"[cg_tally] arc from %s to %s traversed %lu times\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*parent).name,
                (*child).name,
                count,
            );
        }
        arc_add(parent, child, count);
    }
}
#[no_mangle]
pub unsafe extern "C" fn cg_read_rec(
    mut ifp: *mut FILE,
    mut filename: *const libc::c_char,
) {
    let mut from_pc: bfd_vma = 0;
    let mut self_pc: bfd_vma = 0;
    let mut count: libc::c_uint = 0;
    if gmon_io_read_vma(ifp, &mut from_pc) != 0
        || gmon_io_read_vma(ifp, &mut self_pc) != 0
        || gmon_io_read_32(ifp, &mut count) != 0
    {
        fprintf(
            stderr,
            dcgettext(
                b"gprof\0" as *const u8 as *const libc::c_char,
                b"%s: %s: unexpected end of file\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            whoami,
            filename,
        );
        done(1 as libc::c_int);
    }
    if debug_level & (1 as libc::c_int) << 6 as libc::c_int != 0 {
        printf(
            b"[cg_read_rec] frompc 0x%lx selfpc 0x%lx count %lu\n\0" as *const u8
                as *const libc::c_char,
            from_pc,
            self_pc,
            count as libc::c_ulong,
        );
    }
    cg_tally(from_pc, self_pc, count as libc::c_ulong);
}
#[no_mangle]
pub unsafe extern "C" fn cg_write_arcs(
    mut ofp: *mut FILE,
    mut filename: *const libc::c_char,
) {
    let mut arc: *mut Arc = 0 as *mut Arc;
    let mut sym: *mut Sym = 0 as *mut Sym;
    sym = symtab.base;
    while sym < symtab.limit {
        arc = (*sym).cg.children;
        while !arc.is_null() {
            if gmon_io_write_8(ofp, GMON_TAG_CG_ARC as libc::c_int as libc::c_uint) != 0
                || gmon_io_write_vma(ofp, (*(*arc).parent).addr) != 0
                || gmon_io_write_vma(ofp, (*(*arc).child).addr) != 0
                || gmon_io_write_32(ofp, (*arc).count as libc::c_uint) != 0
            {
                perror(filename);
                done(1 as libc::c_int);
            }
            if debug_level & (1 as libc::c_int) << 6 as libc::c_int != 0 {
                printf(
                    b"[cg_write_arcs] frompc 0x%lx selfpc 0x%lx count %lu\n\0"
                        as *const u8 as *const libc::c_char,
                    (*(*arc).parent).addr,
                    (*(*arc).child).addr,
                    (*arc).count,
                );
            }
            arc = (*arc).next_child;
        }
        sym = sym.offset(1);
        sym;
    }
}
