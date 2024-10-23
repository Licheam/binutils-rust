use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn free(_: *mut libc::c_void);
    fn abort() -> !;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn xcalloc(_: size_t, _: size_t) -> *mut libc::c_void;
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
pub type fibheapkey_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fibheap {
    pub nodes: size_t,
    pub min: *mut fibnode,
    pub root: *mut fibnode,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct fibnode {
    pub parent: *mut fibnode,
    pub child: *mut fibnode,
    pub left: *mut fibnode,
    pub right: *mut fibnode,
    pub key: fibheapkey_t,
    pub data: *mut libc::c_void,
    #[bitfield(name = "degree", ty = "libc::c_uint", bits = "0..=30")]
    #[bitfield(name = "mark", ty = "libc::c_uint", bits = "31..=31")]
    pub degree_mark: [u8; 4],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 4],
}
pub type fibheap_t = *mut fibheap;
pub type fibnode_t = *mut fibnode;
#[no_mangle]
pub unsafe extern "C" fn fibheap_new() -> fibheap_t {
    return xcalloc(
        1 as libc::c_int as size_t,
        ::core::mem::size_of::<fibheap>() as libc::c_ulong,
    ) as fibheap_t;
}
unsafe extern "C" fn fibnode_new() -> fibnode_t {
    let mut node: fibnode_t = 0 as *mut fibnode;
    node = xcalloc(
        1 as libc::c_int as size_t,
        ::core::mem::size_of::<fibnode>() as libc::c_ulong,
    ) as fibnode_t;
    (*node).left = node;
    (*node).right = node;
    return node;
}
#[inline]
unsafe extern "C" fn fibheap_compare(
    mut heap: fibheap_t,
    mut a: fibnode_t,
    mut b: fibnode_t,
) -> libc::c_int {
    if (*a).key < (*b).key {
        return -(1 as libc::c_int);
    }
    if (*a).key > (*b).key {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn fibheap_comp_data(
    mut heap: fibheap_t,
    mut key: fibheapkey_t,
    mut data: *mut libc::c_void,
    mut b: fibnode_t,
) -> libc::c_int {
    let mut a: fibnode = fibnode {
        parent: 0 as *mut fibnode,
        child: 0 as *mut fibnode,
        left: 0 as *mut fibnode,
        right: 0 as *mut fibnode,
        key: 0,
        data: 0 as *mut libc::c_void,
        degree_mark: [0; 4],
        c2rust_padding: [0; 4],
    };
    a.key = key;
    a.data = data;
    return fibheap_compare(heap, &mut a, b);
}
#[no_mangle]
pub unsafe extern "C" fn fibheap_insert(
    mut heap: fibheap_t,
    mut key: fibheapkey_t,
    mut data: *mut libc::c_void,
) -> fibnode_t {
    let mut node: fibnode_t = 0 as *mut fibnode;
    node = fibnode_new();
    (*node).data = data;
    (*node).key = key;
    fibheap_ins_root(heap, node);
    if ((*heap).min).is_null() || (*node).key < (*(*heap).min).key {
        (*heap).min = node;
    }
    (*heap).nodes = ((*heap).nodes).wrapping_add(1);
    (*heap).nodes;
    return node;
}
#[no_mangle]
pub unsafe extern "C" fn fibheap_min(mut heap: fibheap_t) -> *mut libc::c_void {
    if ((*heap).min).is_null() {
        return 0 as *mut libc::c_void;
    }
    return (*(*heap).min).data;
}
#[no_mangle]
pub unsafe extern "C" fn fibheap_min_key(mut heap: fibheap_t) -> fibheapkey_t {
    if ((*heap).min).is_null() {
        return 0 as libc::c_int as fibheapkey_t;
    }
    return (*(*heap).min).key;
}
#[no_mangle]
pub unsafe extern "C" fn fibheap_union(
    mut heapa: fibheap_t,
    mut heapb: fibheap_t,
) -> fibheap_t {
    let mut a_root: fibnode_t = 0 as *mut fibnode;
    let mut b_root: fibnode_t = 0 as *mut fibnode;
    let mut temp: fibnode_t = 0 as *mut fibnode;
    a_root = (*heapa).root;
    if a_root.is_null() {
        free(heapa as *mut libc::c_void);
        return heapb;
    }
    b_root = (*heapb).root;
    if b_root.is_null() {
        free(heapb as *mut libc::c_void);
        return heapa;
    }
    (*(*a_root).left).right = b_root;
    (*(*b_root).left).right = a_root;
    temp = (*a_root).left;
    (*a_root).left = (*b_root).left;
    (*b_root).left = temp;
    (*heapa)
        .nodes = ((*heapa).nodes as libc::c_ulong).wrapping_add((*heapb).nodes) as size_t
        as size_t;
    if fibheap_compare(heapa, (*heapb).min, (*heapa).min) < 0 as libc::c_int {
        (*heapa).min = (*heapb).min;
    }
    free(heapb as *mut libc::c_void);
    return heapa;
}
#[no_mangle]
pub unsafe extern "C" fn fibheap_extract_min(mut heap: fibheap_t) -> *mut libc::c_void {
    let mut z: fibnode_t = 0 as *mut fibnode;
    let mut ret: *mut libc::c_void = 0 as *mut libc::c_void;
    if !((*heap).min).is_null() {
        z = fibheap_extr_min_node(heap);
        ret = (*z).data;
        free(z as *mut libc::c_void);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn fibheap_replace_key_data(
    mut heap: fibheap_t,
    mut node: fibnode_t,
    mut key: fibheapkey_t,
    mut data: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut odata: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut okey: fibheapkey_t = 0;
    let mut y: fibnode_t = 0 as *mut fibnode;
    if fibheap_comp_data(heap, key, data, node) > 0 as libc::c_int {
        return 0 as *mut libc::c_void;
    }
    odata = (*node).data;
    okey = (*node).key;
    (*node).data = data;
    (*node).key = key;
    y = (*node).parent;
    if okey == key && okey != -(9223372036854775807 as libc::c_long) - 1 as libc::c_long
    {
        return odata;
    }
    if !y.is_null() && fibheap_compare(heap, node, y) <= 0 as libc::c_int {
        fibheap_cut(heap, node, y);
        fibheap_cascading_cut(heap, y);
    }
    if fibheap_compare(heap, node, (*heap).min) <= 0 as libc::c_int {
        (*heap).min = node;
    }
    return odata;
}
#[no_mangle]
pub unsafe extern "C" fn fibheap_replace_data(
    mut heap: fibheap_t,
    mut node: fibnode_t,
    mut data: *mut libc::c_void,
) -> *mut libc::c_void {
    return fibheap_replace_key_data(heap, node, (*node).key, data);
}
#[no_mangle]
pub unsafe extern "C" fn fibheap_replace_key(
    mut heap: fibheap_t,
    mut node: fibnode_t,
    mut key: fibheapkey_t,
) -> fibheapkey_t {
    let mut okey: libc::c_int = (*node).key as libc::c_int;
    fibheap_replace_key_data(heap, node, key, (*node).data);
    return okey as fibheapkey_t;
}
#[no_mangle]
pub unsafe extern "C" fn fibheap_delete_node(
    mut heap: fibheap_t,
    mut node: fibnode_t,
) -> *mut libc::c_void {
    let mut ret: *mut libc::c_void = (*node).data;
    fibheap_replace_key(
        heap,
        node,
        -(9223372036854775807 as libc::c_long) - 1 as libc::c_long,
    );
    if node != (*heap).min {
        fprintf(
            stderr,
            b"Can't force minimum on fibheap.\n\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    fibheap_extract_min(heap);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn fibheap_delete(mut heap: fibheap_t) {
    while !((*heap).min).is_null() {
        free(fibheap_extr_min_node(heap) as *mut libc::c_void);
    }
    free(heap as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn fibheap_empty(mut heap: fibheap_t) -> libc::c_int {
    return ((*heap).nodes == 0 as libc::c_int as libc::c_ulong) as libc::c_int;
}
unsafe extern "C" fn fibheap_extr_min_node(mut heap: fibheap_t) -> fibnode_t {
    let mut ret: fibnode_t = (*heap).min;
    let mut x: fibnode_t = 0 as *mut fibnode;
    let mut y: fibnode_t = 0 as *mut fibnode;
    let mut orig: fibnode_t = 0 as *mut fibnode;
    x = (*ret).child;
    orig = 0 as fibnode_t;
    while x != orig && !x.is_null() {
        if orig.is_null() {
            orig = x;
        }
        y = (*x).right;
        (*x).parent = 0 as *mut fibnode;
        fibheap_ins_root(heap, x);
        x = y;
    }
    fibheap_rem_root(heap, ret);
    (*heap).nodes = ((*heap).nodes).wrapping_sub(1);
    (*heap).nodes;
    if (*heap).nodes == 0 as libc::c_int as libc::c_ulong {
        (*heap).min = 0 as *mut fibnode;
    } else {
        (*heap).min = (*ret).right;
        fibheap_consolidate(heap);
    }
    return ret;
}
unsafe extern "C" fn fibheap_ins_root(mut heap: fibheap_t, mut node: fibnode_t) {
    if ((*heap).root).is_null() {
        (*heap).root = node;
        (*node).left = node;
        (*node).right = node;
        return;
    }
    fibnode_insert_after((*heap).root, node);
}
unsafe extern "C" fn fibheap_rem_root(mut heap: fibheap_t, mut node: fibnode_t) {
    if (*node).left == node {
        (*heap).root = 0 as *mut fibnode;
    } else {
        (*heap).root = fibnode_remove(node);
    };
}
unsafe extern "C" fn fibheap_consolidate(mut heap: fibheap_t) {
    let mut a: [fibnode_t; 65] = [0 as *mut fibnode; 65];
    let mut w: fibnode_t = 0 as *mut fibnode;
    let mut y: fibnode_t = 0 as *mut fibnode;
    let mut x: fibnode_t = 0 as *mut fibnode;
    let mut i: libc::c_int = 0;
    let mut d: libc::c_int = 0;
    let mut D: libc::c_int = 0;
    D = (1 as libc::c_int as libc::c_ulong)
        .wrapping_add(
            (8 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_long>() as libc::c_ulong),
        ) as libc::c_int;
    memset(
        a.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        (::core::mem::size_of::<fibnode_t>() as libc::c_ulong)
            .wrapping_mul(D as libc::c_ulong),
    );
    loop {
        w = (*heap).root;
        if w.is_null() {
            break;
        }
        x = w;
        fibheap_rem_root(heap, w);
        d = (*x).degree() as libc::c_int;
        while !(a[d as usize]).is_null() {
            y = a[d as usize];
            if fibheap_compare(heap, x, y) > 0 as libc::c_int {
                let mut temp: fibnode_t = 0 as *mut fibnode;
                temp = x;
                x = y;
                y = temp;
            }
            fibheap_link(heap, y, x);
            a[d as usize] = 0 as fibnode_t;
            d += 1;
            d;
        }
        a[d as usize] = x;
    }
    (*heap).min = 0 as *mut fibnode;
    i = 0 as libc::c_int;
    while i < D {
        if !(a[i as usize]).is_null() {
            fibheap_ins_root(heap, a[i as usize]);
            if ((*heap).min).is_null()
                || fibheap_compare(heap, a[i as usize], (*heap).min) < 0 as libc::c_int
            {
                (*heap).min = a[i as usize];
            }
        }
        i += 1;
        i;
    }
}
unsafe extern "C" fn fibheap_link(
    mut heap: fibheap_t,
    mut node: fibnode_t,
    mut parent: fibnode_t,
) {
    if ((*parent).child).is_null() {
        (*parent).child = node;
    } else {
        fibnode_insert_after((*(*parent).child).left, node);
    }
    (*node).parent = parent;
    (*parent).set_degree((*parent).degree() + 1);
    (*parent).degree();
    (*node).set_mark(0 as libc::c_int as libc::c_uint);
}
unsafe extern "C" fn fibheap_cut(
    mut heap: fibheap_t,
    mut node: fibnode_t,
    mut parent: fibnode_t,
) {
    fibnode_remove(node);
    (*parent).set_degree((*parent).degree() - 1);
    (*parent).degree();
    fibheap_ins_root(heap, node);
    (*node).parent = 0 as *mut fibnode;
    (*node).set_mark(0 as libc::c_int as libc::c_uint);
}
unsafe extern "C" fn fibheap_cascading_cut(mut heap: fibheap_t, mut y: fibnode_t) {
    let mut z: fibnode_t = 0 as *mut fibnode;
    loop {
        z = (*y).parent;
        if z.is_null() {
            break;
        }
        if (*y).mark() as libc::c_int == 0 as libc::c_int {
            (*y).set_mark(1 as libc::c_int as libc::c_uint);
            return;
        } else {
            fibheap_cut(heap, y, z);
            y = z;
        }
    };
}
unsafe extern "C" fn fibnode_insert_after(mut a: fibnode_t, mut b: fibnode_t) {
    if a == (*a).right {
        (*a).right = b;
        (*a).left = b;
        (*b).right = a;
        (*b).left = a;
    } else {
        (*b).right = (*a).right;
        (*(*a).right).left = b;
        (*a).right = b;
        (*b).left = a;
    };
}
unsafe extern "C" fn fibnode_remove(mut node: fibnode_t) -> fibnode_t {
    let mut ret: fibnode_t = 0 as *mut fibnode;
    if node == (*node).left {
        ret = 0 as fibnode_t;
    } else {
        ret = (*node).left;
    }
    if !((*node).parent).is_null() && (*(*node).parent).child == node {
        (*(*node).parent).child = ret;
    }
    (*(*node).right).left = (*node).left;
    (*(*node).left).right = (*node).right;
    (*node).parent = 0 as *mut fibnode;
    (*node).left = node;
    (*node).right = node;
    return ret;
}
