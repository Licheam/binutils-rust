use ::libc;
extern "C" {
    fn free(_: *mut libc::c_void);
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    fn xrealloc(_: *mut libc::c_void, _: size_t) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub type uintptr_t = libc::c_ulong;
pub type splay_tree_key = uintptr_t;
pub type splay_tree_value = uintptr_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct splay_tree_node_s {
    pub key: splay_tree_key,
    pub value: splay_tree_value,
    pub left: splay_tree_node,
    pub right: splay_tree_node,
}
pub type splay_tree_node = *mut splay_tree_node_s;
pub type splay_tree_compare_fn = Option::<
    unsafe extern "C" fn(splay_tree_key, splay_tree_key) -> libc::c_int,
>;
pub type splay_tree_delete_key_fn = Option::<unsafe extern "C" fn(splay_tree_key) -> ()>;
pub type splay_tree_delete_value_fn = Option::<
    unsafe extern "C" fn(splay_tree_value) -> (),
>;
pub type splay_tree_foreach_fn = Option::<
    unsafe extern "C" fn(splay_tree_node, *mut libc::c_void) -> libc::c_int,
>;
pub type splay_tree_allocate_fn = Option::<
    unsafe extern "C" fn(libc::c_int, *mut libc::c_void) -> *mut libc::c_void,
>;
pub type splay_tree_deallocate_fn = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct splay_tree_s {
    pub root: splay_tree_node,
    pub comp: splay_tree_compare_fn,
    pub delete_key: splay_tree_delete_key_fn,
    pub delete_value: splay_tree_delete_value_fn,
    pub allocate: splay_tree_allocate_fn,
    pub deallocate: splay_tree_deallocate_fn,
    pub allocate_data: *mut libc::c_void,
}
pub type splay_tree = *mut splay_tree_s;
unsafe extern "C" fn splay_tree_delete_helper(
    mut sp: splay_tree,
    mut node: splay_tree_node,
) {
    let mut pending: splay_tree_node = 0 as splay_tree_node;
    let mut active: splay_tree_node = 0 as splay_tree_node;
    if node.is_null() {
        return;
    }
    if ((*sp).delete_key).is_some() {
        (Some(((*sp).delete_key).expect("non-null function pointer")))
            .expect("non-null function pointer")((*node).key);
    }
    if ((*sp).delete_value).is_some() {
        (Some(((*sp).delete_value).expect("non-null function pointer")))
            .expect("non-null function pointer")((*node).value);
    }
    (*node).key = pending as splay_tree_key;
    pending = node;
    while !pending.is_null() {
        active = pending;
        pending = 0 as splay_tree_node;
        while !active.is_null() {
            let mut temp: splay_tree_node = 0 as *mut splay_tree_node_s;
            if !((*active).left).is_null() {
                if ((*sp).delete_key).is_some() {
                    (Some(((*sp).delete_key).expect("non-null function pointer")))
                        .expect("non-null function pointer")((*(*active).left).key);
                }
                if ((*sp).delete_value).is_some() {
                    (Some(((*sp).delete_value).expect("non-null function pointer")))
                        .expect("non-null function pointer")((*(*active).left).value);
                }
                (*(*active).left).key = pending as splay_tree_key;
                pending = (*active).left;
            }
            if !((*active).right).is_null() {
                if ((*sp).delete_key).is_some() {
                    (Some(((*sp).delete_key).expect("non-null function pointer")))
                        .expect("non-null function pointer")((*(*active).right).key);
                }
                if ((*sp).delete_value).is_some() {
                    (Some(((*sp).delete_value).expect("non-null function pointer")))
                        .expect("non-null function pointer")((*(*active).right).value);
                }
                (*(*active).right).key = pending as splay_tree_key;
                pending = (*active).right;
            }
            temp = active;
            active = (*temp).key as splay_tree_node;
            (Some(((*sp).deallocate).expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(temp as *mut libc::c_char as *mut libc::c_void, (*sp).allocate_data);
        }
    }
}
#[inline]
unsafe extern "C" fn rotate_left(
    mut pp: *mut splay_tree_node,
    mut p: splay_tree_node,
    mut n: splay_tree_node,
) {
    let mut tmp: splay_tree_node = 0 as *mut splay_tree_node_s;
    tmp = (*n).right;
    (*n).right = p;
    (*p).left = tmp;
    *pp = n;
}
#[inline]
unsafe extern "C" fn rotate_right(
    mut pp: *mut splay_tree_node,
    mut p: splay_tree_node,
    mut n: splay_tree_node,
) {
    let mut tmp: splay_tree_node = 0 as *mut splay_tree_node_s;
    tmp = (*n).left;
    (*n).left = p;
    (*p).right = tmp;
    *pp = n;
}
unsafe extern "C" fn splay_tree_splay(mut sp: splay_tree, mut key: splay_tree_key) {
    if ((*sp).root).is_null() {
        return;
    }
    loop {
        let mut cmp1: libc::c_int = 0;
        let mut cmp2: libc::c_int = 0;
        let mut n: splay_tree_node = 0 as *mut splay_tree_node_s;
        let mut c: splay_tree_node = 0 as *mut splay_tree_node_s;
        n = (*sp).root;
        cmp1 = (Some(((*sp).comp).expect("non-null function pointer")))
            .expect("non-null function pointer")(key, (*n).key);
        if cmp1 == 0 as libc::c_int {
            return;
        }
        if cmp1 < 0 as libc::c_int {
            c = (*n).left;
        } else {
            c = (*n).right;
        }
        if c.is_null() {
            return;
        }
        cmp2 = (Some(((*sp).comp).expect("non-null function pointer")))
            .expect("non-null function pointer")(key, (*c).key);
        if cmp2 == 0 as libc::c_int || cmp2 < 0 as libc::c_int && ((*c).left).is_null()
            || cmp2 > 0 as libc::c_int && ((*c).right).is_null()
        {
            if cmp1 < 0 as libc::c_int {
                rotate_left(&mut (*sp).root, n, c);
            } else {
                rotate_right(&mut (*sp).root, n, c);
            }
            return;
        }
        if cmp1 < 0 as libc::c_int && cmp2 < 0 as libc::c_int {
            rotate_left(&mut (*n).left, c, (*c).left);
            rotate_left(&mut (*sp).root, n, (*n).left);
        } else if cmp1 > 0 as libc::c_int && cmp2 > 0 as libc::c_int {
            rotate_right(&mut (*n).right, c, (*c).right);
            rotate_right(&mut (*sp).root, n, (*n).right);
        } else if cmp1 < 0 as libc::c_int && cmp2 > 0 as libc::c_int {
            rotate_right(&mut (*n).left, c, (*c).right);
            rotate_left(&mut (*sp).root, n, (*n).left);
        } else if cmp1 > 0 as libc::c_int && cmp2 < 0 as libc::c_int {
            rotate_left(&mut (*n).right, c, (*c).left);
            rotate_right(&mut (*sp).root, n, (*n).right);
        }
    };
}
unsafe extern "C" fn splay_tree_foreach_helper(
    mut node: splay_tree_node,
    mut fn_0: splay_tree_foreach_fn,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut val: libc::c_int = 0;
    let mut stack: *mut splay_tree_node = 0 as *mut splay_tree_node;
    let mut stack_ptr: libc::c_int = 0;
    let mut stack_size: libc::c_int = 0;
    stack_size = 100 as libc::c_int;
    stack_ptr = 0 as libc::c_int;
    stack = xmalloc(
        (::core::mem::size_of::<splay_tree_node>() as libc::c_ulong)
            .wrapping_mul(stack_size as libc::c_ulong),
    ) as *mut splay_tree_node;
    val = 0 as libc::c_int;
    loop {
        while !node.is_null() {
            if stack_ptr == stack_size {
                stack_size *= 2 as libc::c_int;
                stack = xrealloc(
                    stack as *mut libc::c_void,
                    (::core::mem::size_of::<splay_tree_node>() as libc::c_ulong)
                        .wrapping_mul(stack_size as libc::c_ulong),
                ) as *mut splay_tree_node;
            }
            let fresh0 = stack_ptr;
            stack_ptr = stack_ptr + 1;
            let ref mut fresh1 = *stack.offset(fresh0 as isize);
            *fresh1 = node;
            node = (*node).left;
        }
        if stack_ptr == 0 as libc::c_int {
            break;
        }
        stack_ptr -= 1;
        node = *stack.offset(stack_ptr as isize);
        val = (Some(fn_0.expect("non-null function pointer")))
            .expect("non-null function pointer")(node, data);
        if val != 0 {
            break;
        }
        node = (*node).right;
    }
    free(stack as *mut libc::c_void);
    return val;
}
unsafe extern "C" fn splay_tree_xmalloc_allocate(
    mut size: libc::c_int,
    mut data: *mut libc::c_void,
) -> *mut libc::c_void {
    return xmalloc(size as size_t);
}
unsafe extern "C" fn splay_tree_xmalloc_deallocate(
    mut object: *mut libc::c_void,
    mut data: *mut libc::c_void,
) {
    free(object);
}
#[no_mangle]
pub unsafe extern "C" fn splay_tree_new(
    mut compare_fn: splay_tree_compare_fn,
    mut delete_key_fn: splay_tree_delete_key_fn,
    mut delete_value_fn: splay_tree_delete_value_fn,
) -> splay_tree {
    return splay_tree_new_with_allocator(
        compare_fn,
        delete_key_fn,
        delete_value_fn,
        Some(
            splay_tree_xmalloc_allocate
                as unsafe extern "C" fn(
                    libc::c_int,
                    *mut libc::c_void,
                ) -> *mut libc::c_void,
        ),
        Some(
            splay_tree_xmalloc_deallocate
                as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
        ),
        0 as *mut libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn splay_tree_new_with_allocator(
    mut compare_fn: splay_tree_compare_fn,
    mut delete_key_fn: splay_tree_delete_key_fn,
    mut delete_value_fn: splay_tree_delete_value_fn,
    mut allocate_fn: splay_tree_allocate_fn,
    mut deallocate_fn: splay_tree_deallocate_fn,
    mut allocate_data: *mut libc::c_void,
) -> splay_tree {
    return splay_tree_new_typed_alloc(
        compare_fn,
        delete_key_fn,
        delete_value_fn,
        allocate_fn,
        allocate_fn,
        deallocate_fn,
        allocate_data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn splay_tree_new_typed_alloc(
    mut compare_fn: splay_tree_compare_fn,
    mut delete_key_fn: splay_tree_delete_key_fn,
    mut delete_value_fn: splay_tree_delete_value_fn,
    mut tree_allocate_fn: splay_tree_allocate_fn,
    mut node_allocate_fn: splay_tree_allocate_fn,
    mut deallocate_fn: splay_tree_deallocate_fn,
    mut allocate_data: *mut libc::c_void,
) -> splay_tree {
    let mut sp: splay_tree = (Some(tree_allocate_fn.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        ::core::mem::size_of::<splay_tree_s>() as libc::c_ulong as libc::c_int,
        allocate_data,
    ) as splay_tree;
    (*sp).root = 0 as splay_tree_node;
    (*sp).comp = compare_fn;
    (*sp).delete_key = delete_key_fn;
    (*sp).delete_value = delete_value_fn;
    (*sp).allocate = node_allocate_fn;
    (*sp).deallocate = deallocate_fn;
    (*sp).allocate_data = allocate_data;
    return sp;
}
#[no_mangle]
pub unsafe extern "C" fn splay_tree_delete(mut sp: splay_tree) {
    splay_tree_delete_helper(sp, (*sp).root);
    (Some(((*sp).deallocate).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(sp as *mut libc::c_char as *mut libc::c_void, (*sp).allocate_data);
}
#[no_mangle]
pub unsafe extern "C" fn splay_tree_insert(
    mut sp: splay_tree,
    mut key: splay_tree_key,
    mut value: splay_tree_value,
) -> splay_tree_node {
    let mut comparison: libc::c_int = 0 as libc::c_int;
    splay_tree_splay(sp, key);
    if !((*sp).root).is_null() {
        comparison = (Some(((*sp).comp).expect("non-null function pointer")))
            .expect("non-null function pointer")((*(*sp).root).key, key);
    }
    if !((*sp).root).is_null() && comparison == 0 as libc::c_int {
        if ((*sp).delete_key).is_some() {
            (Some(((*sp).delete_key).expect("non-null function pointer")))
                .expect("non-null function pointer")((*(*sp).root).key);
        }
        if ((*sp).delete_value).is_some() {
            (Some(((*sp).delete_value).expect("non-null function pointer")))
                .expect("non-null function pointer")((*(*sp).root).value);
        }
        (*(*sp).root).key = key;
        (*(*sp).root).value = value;
    } else {
        let mut node: splay_tree_node = 0 as *mut splay_tree_node_s;
        node = (Some(((*sp).allocate).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            ::core::mem::size_of::<splay_tree_node_s>() as libc::c_ulong as libc::c_int,
            (*sp).allocate_data,
        ) as splay_tree_node;
        (*node).key = key;
        (*node).value = value;
        if ((*sp).root).is_null() {
            (*node).right = 0 as splay_tree_node;
            (*node).left = (*node).right;
        } else if comparison < 0 as libc::c_int {
            (*node).left = (*sp).root;
            (*node).right = (*(*node).left).right;
            (*(*node).left).right = 0 as splay_tree_node;
        } else {
            (*node).right = (*sp).root;
            (*node).left = (*(*node).right).left;
            (*(*node).right).left = 0 as splay_tree_node;
        }
        (*sp).root = node;
    }
    return (*sp).root;
}
#[no_mangle]
pub unsafe extern "C" fn splay_tree_remove(mut sp: splay_tree, mut key: splay_tree_key) {
    splay_tree_splay(sp, key);
    if !((*sp).root).is_null()
        && (Some(((*sp).comp).expect("non-null function pointer")))
            .expect("non-null function pointer")((*(*sp).root).key, key)
            == 0 as libc::c_int
    {
        let mut left: splay_tree_node = 0 as *mut splay_tree_node_s;
        let mut right: splay_tree_node = 0 as *mut splay_tree_node_s;
        left = (*(*sp).root).left;
        right = (*(*sp).root).right;
        if ((*sp).delete_key).is_some() {
            (Some(((*sp).delete_key).expect("non-null function pointer")))
                .expect("non-null function pointer")((*(*sp).root).key);
        }
        if ((*sp).delete_value).is_some() {
            (Some(((*sp).delete_value).expect("non-null function pointer")))
                .expect("non-null function pointer")((*(*sp).root).value);
        }
        (Some(((*sp).deallocate).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )((*sp).root as *mut libc::c_void, (*sp).allocate_data);
        if !left.is_null() {
            (*sp).root = left;
            if !right.is_null() {
                while !((*left).right).is_null() {
                    left = (*left).right;
                }
                (*left).right = right;
            }
        } else {
            (*sp).root = right;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn splay_tree_lookup(
    mut sp: splay_tree,
    mut key: splay_tree_key,
) -> splay_tree_node {
    splay_tree_splay(sp, key);
    if !((*sp).root).is_null()
        && (Some(((*sp).comp).expect("non-null function pointer")))
            .expect("non-null function pointer")((*(*sp).root).key, key)
            == 0 as libc::c_int
    {
        return (*sp).root
    } else {
        return 0 as splay_tree_node
    };
}
#[no_mangle]
pub unsafe extern "C" fn splay_tree_max(mut sp: splay_tree) -> splay_tree_node {
    let mut n: splay_tree_node = (*sp).root;
    if n.is_null() {
        return 0 as splay_tree_node;
    }
    while !((*n).right).is_null() {
        n = (*n).right;
    }
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn splay_tree_min(mut sp: splay_tree) -> splay_tree_node {
    let mut n: splay_tree_node = (*sp).root;
    if n.is_null() {
        return 0 as splay_tree_node;
    }
    while !((*n).left).is_null() {
        n = (*n).left;
    }
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn splay_tree_predecessor(
    mut sp: splay_tree,
    mut key: splay_tree_key,
) -> splay_tree_node {
    let mut comparison: libc::c_int = 0;
    let mut node: splay_tree_node = 0 as *mut splay_tree_node_s;
    if ((*sp).root).is_null() {
        return 0 as splay_tree_node;
    }
    splay_tree_splay(sp, key);
    comparison = (Some(((*sp).comp).expect("non-null function pointer")))
        .expect("non-null function pointer")((*(*sp).root).key, key);
    if comparison < 0 as libc::c_int {
        return (*sp).root;
    }
    node = (*(*sp).root).left;
    if !node.is_null() {
        while !((*node).right).is_null() {
            node = (*node).right;
        }
    }
    return node;
}
#[no_mangle]
pub unsafe extern "C" fn splay_tree_successor(
    mut sp: splay_tree,
    mut key: splay_tree_key,
) -> splay_tree_node {
    let mut comparison: libc::c_int = 0;
    let mut node: splay_tree_node = 0 as *mut splay_tree_node_s;
    if ((*sp).root).is_null() {
        return 0 as splay_tree_node;
    }
    splay_tree_splay(sp, key);
    comparison = (Some(((*sp).comp).expect("non-null function pointer")))
        .expect("non-null function pointer")((*(*sp).root).key, key);
    if comparison > 0 as libc::c_int {
        return (*sp).root;
    }
    node = (*(*sp).root).right;
    if !node.is_null() {
        while !((*node).left).is_null() {
            node = (*node).left;
        }
    }
    return node;
}
#[no_mangle]
pub unsafe extern "C" fn splay_tree_foreach(
    mut sp: splay_tree,
    mut fn_0: splay_tree_foreach_fn,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    return splay_tree_foreach_helper((*sp).root, fn_0, data);
}
#[no_mangle]
pub unsafe extern "C" fn splay_tree_compare_ints(
    mut k1: splay_tree_key,
    mut k2: splay_tree_key,
) -> libc::c_int {
    if (k1 as libc::c_int) < k2 as libc::c_int {
        return -(1 as libc::c_int)
    } else if k1 as libc::c_int > k2 as libc::c_int {
        return 1 as libc::c_int
    } else {
        return 0 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn splay_tree_compare_pointers(
    mut k1: splay_tree_key,
    mut k2: splay_tree_key,
) -> libc::c_int {
    if (k1 as *mut libc::c_char) < k2 as *mut libc::c_char {
        return -(1 as libc::c_int)
    } else if k1 as *mut libc::c_char > k2 as *mut libc::c_char {
        return 1 as libc::c_int
    } else {
        return 0 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn splay_tree_compare_strings(
    mut k1: splay_tree_key,
    mut k2: splay_tree_key,
) -> libc::c_int {
    return strcmp(k1 as *mut libc::c_char, k2 as *mut libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn splay_tree_delete_pointers(mut value: splay_tree_value) {
    free(value as *mut libc::c_void);
}
