use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn abort() -> !;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn xcalloc(_: size_t, _: size_t) -> *mut libc::c_void;
}
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type size_t = libc::c_ulong;
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
pub type uint64_t = __uint64_t;
pub type intptr_t = libc::c_long;
pub type hashval_t = libc::c_uint;
pub type htab_hash = Option::<unsafe extern "C" fn(*const libc::c_void) -> hashval_t>;
pub type htab_eq = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
pub type htab_del = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type htab_trav = Option::<
    unsafe extern "C" fn(*mut *mut libc::c_void, *mut libc::c_void) -> libc::c_int,
>;
pub type htab_alloc = Option::<
    unsafe extern "C" fn(size_t, size_t) -> *mut libc::c_void,
>;
pub type htab_free = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type htab_alloc_with_arg = Option::<
    unsafe extern "C" fn(*mut libc::c_void, size_t, size_t) -> *mut libc::c_void,
>;
pub type htab_free_with_arg = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
>;
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
pub type htab_t = *mut htab;
pub type insert_option = libc::c_uint;
pub const INSERT: insert_option = 1;
pub const NO_INSERT: insert_option = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct prime_ent {
    pub prime: hashval_t,
    pub inv: hashval_t,
    pub inv_m2: hashval_t,
    pub shift: hashval_t,
}
pub type ull = uint64_t;
#[no_mangle]
pub static mut htab_hash_pointer: htab_hash = unsafe {
    Some(hash_pointer as unsafe extern "C" fn(*const libc::c_void) -> hashval_t)
};
#[no_mangle]
pub static mut htab_eq_pointer: htab_eq = unsafe {
    Some(
        eq_pointer
            as unsafe extern "C" fn(
                *const libc::c_void,
                *const libc::c_void,
            ) -> libc::c_int,
    )
};
static mut prime_tab: [prime_ent; 30] = [
    {
        let mut init = prime_ent {
            prime: 7 as libc::c_int as hashval_t,
            inv: 0x24924925 as libc::c_int as hashval_t,
            inv_m2: 0x9999999b as libc::c_uint,
            shift: 2 as libc::c_int as hashval_t,
        };
        init
    },
    {
        let mut init = prime_ent {
            prime: 13 as libc::c_int as hashval_t,
            inv: 0x3b13b13c as libc::c_int as hashval_t,
            inv_m2: 0x745d1747 as libc::c_int as hashval_t,
            shift: 3 as libc::c_int as hashval_t,
        };
        init
    },
    {
        let mut init = prime_ent {
            prime: 31 as libc::c_int as hashval_t,
            inv: 0x8421085 as libc::c_int as hashval_t,
            inv_m2: 0x1a7b9612 as libc::c_int as hashval_t,
            shift: 4 as libc::c_int as hashval_t,
        };
        init
    },
    {
        let mut init = prime_ent {
            prime: 61 as libc::c_int as hashval_t,
            inv: 0xc9714fc as libc::c_int as hashval_t,
            inv_m2: 0x15b1e5f8 as libc::c_int as hashval_t,
            shift: 5 as libc::c_int as hashval_t,
        };
        init
    },
    {
        let mut init = prime_ent {
            prime: 127 as libc::c_int as hashval_t,
            inv: 0x2040811 as libc::c_int as hashval_t,
            inv_m2: 0x624dd30 as libc::c_int as hashval_t,
            shift: 6 as libc::c_int as hashval_t,
        };
        init
    },
    {
        let mut init = prime_ent {
            prime: 251 as libc::c_int as hashval_t,
            inv: 0x5197f7e as libc::c_int as hashval_t,
            inv_m2: 0x73260a5 as libc::c_int as hashval_t,
            shift: 7 as libc::c_int as hashval_t,
        };
        init
    },
    {
        let mut init = prime_ent {
            prime: 509 as libc::c_int as hashval_t,
            inv: 0x1824366 as libc::c_int as hashval_t,
            inv_m2: 0x2864fc8 as libc::c_int as hashval_t,
            shift: 8 as libc::c_int as hashval_t,
        };
        init
    },
    {
        let mut init = prime_ent {
            prime: 1021 as libc::c_int as hashval_t,
            inv: 0xc0906d as libc::c_int as hashval_t,
            inv_m2: 0x14191f7 as libc::c_int as hashval_t,
            shift: 9 as libc::c_int as hashval_t,
        };
        init
    },
    {
        let mut init = prime_ent {
            prime: 2039 as libc::c_int as hashval_t,
            inv: 0x121456f as libc::c_int as hashval_t,
            inv_m2: 0x161e69e as libc::c_int as hashval_t,
            shift: 10 as libc::c_int as hashval_t,
        };
        init
    },
    {
        let mut init = prime_ent {
            prime: 4093 as libc::c_int as hashval_t,
            inv: 0x300902 as libc::c_int as hashval_t,
            inv_m2: 0x501908 as libc::c_int as hashval_t,
            shift: 11 as libc::c_int as hashval_t,
        };
        init
    },
    {
        let mut init = prime_ent {
            prime: 8191 as libc::c_int as hashval_t,
            inv: 0x80041 as libc::c_int as hashval_t,
            inv_m2: 0x180241 as libc::c_int as hashval_t,
            shift: 12 as libc::c_int as hashval_t,
        };
        init
    },
    {
        let mut init = prime_ent {
            prime: 16381 as libc::c_int as hashval_t,
            inv: 0xc0091 as libc::c_int as hashval_t,
            inv_m2: 0x140191 as libc::c_int as hashval_t,
            shift: 13 as libc::c_int as hashval_t,
        };
        init
    },
    {
        let mut init = prime_ent {
            prime: 32749 as libc::c_int as hashval_t,
            inv: 0x2605a5 as libc::c_int as hashval_t,
            inv_m2: 0x2a06e6 as libc::c_int as hashval_t,
            shift: 14 as libc::c_int as hashval_t,
        };
        init
    },
    {
        let mut init = prime_ent {
            prime: 65521 as libc::c_int as hashval_t,
            inv: 0xf00e2 as libc::c_int as hashval_t,
            inv_m2: 0x110122 as libc::c_int as hashval_t,
            shift: 15 as libc::c_int as hashval_t,
        };
        init
    },
    {
        let mut init = prime_ent {
            prime: 131071 as libc::c_int as hashval_t,
            inv: 0x8001 as libc::c_int as hashval_t,
            inv_m2: 0x18003 as libc::c_int as hashval_t,
            shift: 16 as libc::c_int as hashval_t,
        };
        init
    },
    {
        let mut init = prime_ent {
            prime: 262139 as libc::c_int as hashval_t,
            inv: 0x14002 as libc::c_int as hashval_t,
            inv_m2: 0x1c004 as libc::c_int as hashval_t,
            shift: 17 as libc::c_int as hashval_t,
        };
        init
    },
    {
        let mut init = prime_ent {
            prime: 524287 as libc::c_int as hashval_t,
            inv: 0x2001 as libc::c_int as hashval_t,
            inv_m2: 0x6001 as libc::c_int as hashval_t,
            shift: 18 as libc::c_int as hashval_t,
        };
        init
    },
    {
        let mut init = prime_ent {
            prime: 1048573 as libc::c_int as hashval_t,
            inv: 0x3001 as libc::c_int as hashval_t,
            inv_m2: 0x5001 as libc::c_int as hashval_t,
            shift: 19 as libc::c_int as hashval_t,
        };
        init
    },
    {
        let mut init = prime_ent {
            prime: 2097143 as libc::c_int as hashval_t,
            inv: 0x4801 as libc::c_int as hashval_t,
            inv_m2: 0x5801 as libc::c_int as hashval_t,
            shift: 20 as libc::c_int as hashval_t,
        };
        init
    },
    {
        let mut init = prime_ent {
            prime: 4194301 as libc::c_int as hashval_t,
            inv: 0xc01 as libc::c_int as hashval_t,
            inv_m2: 0x1401 as libc::c_int as hashval_t,
            shift: 21 as libc::c_int as hashval_t,
        };
        init
    },
    {
        let mut init = prime_ent {
            prime: 8388593 as libc::c_int as hashval_t,
            inv: 0x1e01 as libc::c_int as hashval_t,
            inv_m2: 0x2201 as libc::c_int as hashval_t,
            shift: 22 as libc::c_int as hashval_t,
        };
        init
    },
    {
        let mut init = prime_ent {
            prime: 16777213 as libc::c_int as hashval_t,
            inv: 0x301 as libc::c_int as hashval_t,
            inv_m2: 0x501 as libc::c_int as hashval_t,
            shift: 23 as libc::c_int as hashval_t,
        };
        init
    },
    {
        let mut init = prime_ent {
            prime: 33554393 as libc::c_int as hashval_t,
            inv: 0x1381 as libc::c_int as hashval_t,
            inv_m2: 0x1481 as libc::c_int as hashval_t,
            shift: 24 as libc::c_int as hashval_t,
        };
        init
    },
    {
        let mut init = prime_ent {
            prime: 67108859 as libc::c_int as hashval_t,
            inv: 0x141 as libc::c_int as hashval_t,
            inv_m2: 0x1c1 as libc::c_int as hashval_t,
            shift: 25 as libc::c_int as hashval_t,
        };
        init
    },
    {
        let mut init = prime_ent {
            prime: 134217689 as libc::c_int as hashval_t,
            inv: 0x4e1 as libc::c_int as hashval_t,
            inv_m2: 0x521 as libc::c_int as hashval_t,
            shift: 26 as libc::c_int as hashval_t,
        };
        init
    },
    {
        let mut init = prime_ent {
            prime: 268435399 as libc::c_int as hashval_t,
            inv: 0x391 as libc::c_int as hashval_t,
            inv_m2: 0x3b1 as libc::c_int as hashval_t,
            shift: 27 as libc::c_int as hashval_t,
        };
        init
    },
    {
        let mut init = prime_ent {
            prime: 536870909 as libc::c_int as hashval_t,
            inv: 0x19 as libc::c_int as hashval_t,
            inv_m2: 0x29 as libc::c_int as hashval_t,
            shift: 28 as libc::c_int as hashval_t,
        };
        init
    },
    {
        let mut init = prime_ent {
            prime: 1073741789 as libc::c_int as hashval_t,
            inv: 0x8d as libc::c_int as hashval_t,
            inv_m2: 0x95 as libc::c_int as hashval_t,
            shift: 29 as libc::c_int as hashval_t,
        };
        init
    },
    {
        let mut init = prime_ent {
            prime: 2147483647 as libc::c_int as hashval_t,
            inv: 0x3 as libc::c_int as hashval_t,
            inv_m2: 0x7 as libc::c_int as hashval_t,
            shift: 30 as libc::c_int as hashval_t,
        };
        init
    },
    {
        let mut init = prime_ent {
            prime: 0xfffffffb as libc::c_uint,
            inv: 0x6 as libc::c_int as hashval_t,
            inv_m2: 0x8 as libc::c_int as hashval_t,
            shift: 31 as libc::c_int as hashval_t,
        };
        init
    },
];
unsafe extern "C" fn higher_prime_index(mut n: libc::c_ulong) -> libc::c_uint {
    let mut low: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut high: libc::c_uint = (::core::mem::size_of::<[prime_ent; 30]>()
        as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<prime_ent>() as libc::c_ulong)
        as libc::c_uint;
    while low != high {
        let mut mid: libc::c_uint = low
            .wrapping_add(
                high.wrapping_sub(low).wrapping_div(2 as libc::c_int as libc::c_uint),
            );
        if n > prime_tab[mid as usize].prime as libc::c_ulong {
            low = mid.wrapping_add(1 as libc::c_int as libc::c_uint);
        } else {
            high = mid;
        }
    }
    if n > prime_tab[low as usize].prime as libc::c_ulong {
        fprintf(
            stderr,
            b"Cannot find prime bigger than %lu\n\0" as *const u8 as *const libc::c_char,
            n,
        );
        abort();
    }
    return low;
}
unsafe extern "C" fn eq_pointer(
    mut p1: *const libc::c_void,
    mut p2: *const libc::c_void,
) -> libc::c_int {
    return (p1 == p2) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn htab_size(mut htab: htab_t) -> size_t {
    return (*htab).size;
}
#[no_mangle]
pub unsafe extern "C" fn htab_elements(mut htab: htab_t) -> size_t {
    return ((*htab).n_elements).wrapping_sub((*htab).n_deleted);
}
#[inline]
unsafe extern "C" fn htab_mod_1(
    mut x: hashval_t,
    mut y: hashval_t,
    mut inv: hashval_t,
    mut shift: libc::c_int,
) -> hashval_t {
    if (::core::mem::size_of::<hashval_t>() as libc::c_ulong)
        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
        <= 32 as libc::c_int as libc::c_ulong
    {
        let mut t1: hashval_t = 0;
        let mut t2: hashval_t = 0;
        let mut t3: hashval_t = 0;
        let mut t4: hashval_t = 0;
        let mut q: hashval_t = 0;
        let mut r: hashval_t = 0;
        t1 = ((x as ull).wrapping_mul(inv as libc::c_ulong) >> 32 as libc::c_int)
            as hashval_t;
        t2 = x.wrapping_sub(t1);
        t3 = t2 >> 1 as libc::c_int;
        t4 = t1.wrapping_add(t3);
        q = t4 >> shift;
        r = x.wrapping_sub(q.wrapping_mul(y));
        return r;
    }
    return x.wrapping_rem(y);
}
#[inline]
unsafe extern "C" fn htab_mod(mut hash: hashval_t, mut htab: htab_t) -> hashval_t {
    let mut p: *const prime_ent = &*prime_tab
        .as_ptr()
        .offset((*htab).size_prime_index as isize) as *const prime_ent;
    return htab_mod_1(hash, (*p).prime, (*p).inv, (*p).shift as libc::c_int);
}
#[inline]
unsafe extern "C" fn htab_mod_m2(mut hash: hashval_t, mut htab: htab_t) -> hashval_t {
    let mut p: *const prime_ent = &*prime_tab
        .as_ptr()
        .offset((*htab).size_prime_index as isize) as *const prime_ent;
    return (1 as libc::c_int as libc::c_uint)
        .wrapping_add(
            htab_mod_1(
                hash,
                ((*p).prime).wrapping_sub(2 as libc::c_int as libc::c_uint),
                (*p).inv_m2,
                (*p).shift as libc::c_int,
            ),
        );
}
#[no_mangle]
pub unsafe extern "C" fn htab_create_alloc(
    mut size: size_t,
    mut hash_f: htab_hash,
    mut eq_f: htab_eq,
    mut del_f: htab_del,
    mut alloc_f: htab_alloc,
    mut free_f: htab_free,
) -> htab_t {
    return htab_create_typed_alloc(size, hash_f, eq_f, del_f, alloc_f, alloc_f, free_f);
}
#[no_mangle]
pub unsafe extern "C" fn htab_create_alloc_ex(
    mut size: size_t,
    mut hash_f: htab_hash,
    mut eq_f: htab_eq,
    mut del_f: htab_del,
    mut alloc_arg: *mut libc::c_void,
    mut alloc_f: htab_alloc_with_arg,
    mut free_f: htab_free_with_arg,
) -> htab_t {
    let mut result: htab_t = 0 as *mut htab;
    let mut size_prime_index: libc::c_uint = 0;
    size_prime_index = higher_prime_index(size);
    size = prime_tab[size_prime_index as usize].prime as size_t;
    result = (Some(alloc_f.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        alloc_arg,
        1 as libc::c_int as size_t,
        ::core::mem::size_of::<htab>() as libc::c_ulong,
    ) as htab_t;
    if result.is_null() {
        return 0 as htab_t;
    }
    (*result)
        .entries = (Some(alloc_f.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(alloc_arg, size, ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
        as *mut *mut libc::c_void;
    if ((*result).entries).is_null() {
        if free_f.is_some() {
            (Some(free_f.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(alloc_arg, result as *mut libc::c_void);
        }
        return 0 as htab_t;
    }
    (*result).size = size;
    (*result).size_prime_index = size_prime_index;
    (*result).hash_f = hash_f;
    (*result).eq_f = eq_f;
    (*result).del_f = del_f;
    (*result).alloc_arg = alloc_arg;
    (*result).alloc_with_arg_f = alloc_f;
    (*result).free_with_arg_f = free_f;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn htab_create_typed_alloc(
    mut size: size_t,
    mut hash_f: htab_hash,
    mut eq_f: htab_eq,
    mut del_f: htab_del,
    mut alloc_tab_f: htab_alloc,
    mut alloc_f: htab_alloc,
    mut free_f: htab_free,
) -> htab_t {
    let mut result: htab_t = 0 as *mut htab;
    let mut size_prime_index: libc::c_uint = 0;
    size_prime_index = higher_prime_index(size);
    size = prime_tab[size_prime_index as usize].prime as size_t;
    result = (Some(alloc_tab_f.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(1 as libc::c_int as size_t, ::core::mem::size_of::<htab>() as libc::c_ulong)
        as htab_t;
    if result.is_null() {
        return 0 as htab_t;
    }
    (*result)
        .entries = (Some(alloc_f.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(size, ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
        as *mut *mut libc::c_void;
    if ((*result).entries).is_null() {
        if free_f.is_some() {
            (Some(free_f.expect("non-null function pointer")))
                .expect("non-null function pointer")(result as *mut libc::c_void);
        }
        return 0 as htab_t;
    }
    (*result).size = size;
    (*result).size_prime_index = size_prime_index;
    (*result).hash_f = hash_f;
    (*result).eq_f = eq_f;
    (*result).del_f = del_f;
    (*result).alloc_f = alloc_f;
    (*result).free_f = free_f;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn htab_set_functions_ex(
    mut htab: htab_t,
    mut hash_f: htab_hash,
    mut eq_f: htab_eq,
    mut del_f: htab_del,
    mut alloc_arg: *mut libc::c_void,
    mut alloc_f: htab_alloc_with_arg,
    mut free_f: htab_free_with_arg,
) {
    (*htab).hash_f = hash_f;
    (*htab).eq_f = eq_f;
    (*htab).del_f = del_f;
    (*htab).alloc_arg = alloc_arg;
    (*htab).alloc_with_arg_f = alloc_f;
    (*htab).free_with_arg_f = free_f;
}
#[no_mangle]
pub unsafe extern "C" fn htab_create(
    mut size: size_t,
    mut hash_f: htab_hash,
    mut eq_f: htab_eq,
    mut del_f: htab_del,
) -> htab_t {
    return htab_create_alloc(
        size,
        hash_f,
        eq_f,
        del_f,
        Some(xcalloc as unsafe extern "C" fn(size_t, size_t) -> *mut libc::c_void),
        Some(free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
    );
}
#[no_mangle]
pub unsafe extern "C" fn htab_try_create(
    mut size: size_t,
    mut hash_f: htab_hash,
    mut eq_f: htab_eq,
    mut del_f: htab_del,
) -> htab_t {
    return htab_create_alloc(
        size,
        hash_f,
        eq_f,
        del_f,
        Some(
            calloc
                as unsafe extern "C" fn(
                    libc::c_ulong,
                    libc::c_ulong,
                ) -> *mut libc::c_void,
        ),
        Some(free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
    );
}
#[no_mangle]
pub unsafe extern "C" fn htab_delete(mut htab: htab_t) {
    let mut size: size_t = (*htab).size;
    let mut entries: *mut *mut libc::c_void = (*htab).entries;
    let mut i: libc::c_int = 0;
    if ((*htab).del_f).is_some() {
        i = size.wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int;
        while i >= 0 as libc::c_int {
            if !(*entries.offset(i as isize)).is_null()
                && *entries.offset(i as isize) != 1 as libc::c_int as *mut libc::c_void
            {
                (Some(((*htab).del_f).expect("non-null function pointer")))
                    .expect("non-null function pointer")(*entries.offset(i as isize));
            }
            i -= 1;
            i;
        }
    }
    if ((*htab).free_f).is_some() {
        (Some(((*htab).free_f).expect("non-null function pointer")))
            .expect("non-null function pointer")(entries as *mut libc::c_void);
        (Some(((*htab).free_f).expect("non-null function pointer")))
            .expect("non-null function pointer")(htab as *mut libc::c_void);
    } else if ((*htab).free_with_arg_f).is_some() {
        (Some(((*htab).free_with_arg_f).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )((*htab).alloc_arg, entries as *mut libc::c_void);
        (Some(((*htab).free_with_arg_f).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )((*htab).alloc_arg, htab as *mut libc::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn htab_empty(mut htab: htab_t) {
    let mut size: size_t = (*htab).size;
    let mut entries: *mut *mut libc::c_void = (*htab).entries;
    let mut i: libc::c_int = 0;
    if ((*htab).del_f).is_some() {
        i = size.wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int;
        while i >= 0 as libc::c_int {
            if !(*entries.offset(i as isize)).is_null()
                && *entries.offset(i as isize) != 1 as libc::c_int as *mut libc::c_void
            {
                (Some(((*htab).del_f).expect("non-null function pointer")))
                    .expect("non-null function pointer")(*entries.offset(i as isize));
            }
            i -= 1;
            i;
        }
    }
    if size
        > ((1024 as libc::c_int * 1024 as libc::c_int) as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
    {
        let mut nindex: libc::c_int = higher_prime_index(
            (1024 as libc::c_int as libc::c_ulong)
                .wrapping_div(
                    ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                ),
        ) as libc::c_int;
        let mut nsize: libc::c_int = prime_tab[nindex as usize].prime as libc::c_int;
        if ((*htab).free_f).is_some() {
            (Some(((*htab).free_f).expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )((*htab).entries as *mut libc::c_void);
        } else if ((*htab).free_with_arg_f).is_some() {
            (Some(((*htab).free_with_arg_f).expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )((*htab).alloc_arg, (*htab).entries as *mut libc::c_void);
        }
        if ((*htab).alloc_with_arg_f).is_some() {
            (*htab)
                .entries = (Some(
                ((*htab).alloc_with_arg_f).expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(
                (*htab).alloc_arg,
                nsize as size_t,
                ::core::mem::size_of::<*mut *mut libc::c_void>() as libc::c_ulong,
            ) as *mut *mut libc::c_void;
        } else {
            (*htab)
                .entries = (Some(((*htab).alloc_f).expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                nsize as size_t,
                ::core::mem::size_of::<*mut *mut libc::c_void>() as libc::c_ulong,
            ) as *mut *mut libc::c_void;
        }
        (*htab).size = nsize as size_t;
        (*htab).size_prime_index = nindex as libc::c_uint;
    } else {
        memset(
            entries as *mut libc::c_void,
            0 as libc::c_int,
            size
                .wrapping_mul(
                    ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                ),
        );
    }
    (*htab).n_deleted = 0 as libc::c_int as size_t;
    (*htab).n_elements = 0 as libc::c_int as size_t;
}
unsafe extern "C" fn find_empty_slot_for_expand(
    mut htab: htab_t,
    mut hash: hashval_t,
) -> *mut *mut libc::c_void {
    let mut index: hashval_t = htab_mod(hash, htab);
    let mut size: size_t = (*htab).size;
    let mut slot: *mut *mut libc::c_void = ((*htab).entries).offset(index as isize);
    let mut hash2: hashval_t = 0;
    if (*slot).is_null() {
        return slot
    } else if *slot == 1 as libc::c_int as *mut libc::c_void {
        abort();
    }
    hash2 = htab_mod_m2(hash, htab);
    loop {
        index = (index as libc::c_uint).wrapping_add(hash2) as hashval_t as hashval_t;
        if index as libc::c_ulong >= size {
            index = (index as libc::c_ulong).wrapping_sub(size) as hashval_t
                as hashval_t;
        }
        slot = ((*htab).entries).offset(index as isize);
        if (*slot).is_null() {
            return slot
        } else if *slot == 1 as libc::c_int as *mut libc::c_void {
            abort();
        }
    };
}
unsafe extern "C" fn htab_expand(mut htab: htab_t) -> libc::c_int {
    let mut oentries: *mut *mut libc::c_void = 0 as *mut *mut libc::c_void;
    let mut olimit: *mut *mut libc::c_void = 0 as *mut *mut libc::c_void;
    let mut p: *mut *mut libc::c_void = 0 as *mut *mut libc::c_void;
    let mut nentries: *mut *mut libc::c_void = 0 as *mut *mut libc::c_void;
    let mut nsize: size_t = 0;
    let mut osize: size_t = 0;
    let mut elts: size_t = 0;
    let mut oindex: libc::c_uint = 0;
    let mut nindex: libc::c_uint = 0;
    oentries = (*htab).entries;
    oindex = (*htab).size_prime_index;
    osize = (*htab).size;
    olimit = oentries.offset(osize as isize);
    elts = ((*htab).n_elements).wrapping_sub((*htab).n_deleted);
    if elts.wrapping_mul(2 as libc::c_int as libc::c_ulong) > osize
        || elts.wrapping_mul(8 as libc::c_int as libc::c_ulong) < osize
            && osize > 32 as libc::c_int as libc::c_ulong
    {
        nindex = higher_prime_index(
            elts.wrapping_mul(2 as libc::c_int as libc::c_ulong),
        );
        nsize = prime_tab[nindex as usize].prime as size_t;
    } else {
        nindex = oindex;
        nsize = osize;
    }
    if ((*htab).alloc_with_arg_f).is_some() {
        nentries = (Some(((*htab).alloc_with_arg_f).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            (*htab).alloc_arg,
            nsize,
            ::core::mem::size_of::<*mut *mut libc::c_void>() as libc::c_ulong,
        ) as *mut *mut libc::c_void;
    } else {
        nentries = (Some(((*htab).alloc_f).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(nsize, ::core::mem::size_of::<*mut *mut libc::c_void>() as libc::c_ulong)
            as *mut *mut libc::c_void;
    }
    if nentries.is_null() {
        return 0 as libc::c_int;
    }
    (*htab).entries = nentries;
    (*htab).size = nsize;
    (*htab).size_prime_index = nindex;
    (*htab)
        .n_elements = ((*htab).n_elements as libc::c_ulong)
        .wrapping_sub((*htab).n_deleted) as size_t as size_t;
    (*htab).n_deleted = 0 as libc::c_int as size_t;
    p = oentries;
    loop {
        let mut x: *mut libc::c_void = *p;
        if !x.is_null() && x != 1 as libc::c_int as *mut libc::c_void {
            let mut q: *mut *mut libc::c_void = find_empty_slot_for_expand(
                htab,
                (Some(((*htab).hash_f).expect("non-null function pointer")))
                    .expect("non-null function pointer")(x),
            );
            *q = x;
        }
        p = p.offset(1);
        p;
        if !(p < olimit) {
            break;
        }
    }
    if ((*htab).free_f).is_some() {
        (Some(((*htab).free_f).expect("non-null function pointer")))
            .expect("non-null function pointer")(oentries as *mut libc::c_void);
    } else if ((*htab).free_with_arg_f).is_some() {
        (Some(((*htab).free_with_arg_f).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )((*htab).alloc_arg, oentries as *mut libc::c_void);
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn htab_find_with_hash(
    mut htab: htab_t,
    mut element: *const libc::c_void,
    mut hash: hashval_t,
) -> *mut libc::c_void {
    let mut index: hashval_t = 0;
    let mut hash2: hashval_t = 0;
    let mut size: size_t = 0;
    let mut entry: *mut libc::c_void = 0 as *mut libc::c_void;
    (*htab).searches = ((*htab).searches).wrapping_add(1);
    (*htab).searches;
    size = (*htab).size;
    index = htab_mod(hash, htab);
    entry = *((*htab).entries).offset(index as isize);
    if entry.is_null()
        || entry != 1 as libc::c_int as *mut libc::c_void
            && (Some(((*htab).eq_f).expect("non-null function pointer")))
                .expect("non-null function pointer")(entry, element) != 0
    {
        return entry;
    }
    hash2 = htab_mod_m2(hash, htab);
    loop {
        (*htab).collisions = ((*htab).collisions).wrapping_add(1);
        (*htab).collisions;
        index = (index as libc::c_uint).wrapping_add(hash2) as hashval_t as hashval_t;
        if index as libc::c_ulong >= size {
            index = (index as libc::c_ulong).wrapping_sub(size) as hashval_t
                as hashval_t;
        }
        entry = *((*htab).entries).offset(index as isize);
        if entry.is_null()
            || entry != 1 as libc::c_int as *mut libc::c_void
                && (Some(((*htab).eq_f).expect("non-null function pointer")))
                    .expect("non-null function pointer")(entry, element) != 0
        {
            return entry;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn htab_find(
    mut htab: htab_t,
    mut element: *const libc::c_void,
) -> *mut libc::c_void {
    return htab_find_with_hash(
        htab,
        element,
        (Some(((*htab).hash_f).expect("non-null function pointer")))
            .expect("non-null function pointer")(element),
    );
}
#[no_mangle]
pub unsafe extern "C" fn htab_find_slot_with_hash(
    mut htab: htab_t,
    mut element: *const libc::c_void,
    mut hash: hashval_t,
    mut insert: insert_option,
) -> *mut *mut libc::c_void {
    let mut first_deleted_slot: *mut *mut libc::c_void = 0 as *mut *mut libc::c_void;
    let mut index: hashval_t = 0;
    let mut hash2: hashval_t = 0;
    let mut size: size_t = 0;
    let mut entry: *mut libc::c_void = 0 as *mut libc::c_void;
    size = (*htab).size;
    if insert as libc::c_uint == INSERT as libc::c_int as libc::c_uint
        && size.wrapping_mul(3 as libc::c_int as libc::c_ulong)
            <= ((*htab).n_elements).wrapping_mul(4 as libc::c_int as libc::c_ulong)
    {
        if htab_expand(htab) == 0 as libc::c_int {
            return 0 as *mut *mut libc::c_void;
        }
        size = (*htab).size;
    }
    index = htab_mod(hash, htab);
    (*htab).searches = ((*htab).searches).wrapping_add(1);
    (*htab).searches;
    first_deleted_slot = 0 as *mut *mut libc::c_void;
    entry = *((*htab).entries).offset(index as isize);
    if !entry.is_null() {
        if entry == 1 as libc::c_int as *mut libc::c_void {
            first_deleted_slot = &mut *((*htab).entries).offset(index as isize)
                as *mut *mut libc::c_void;
        } else if (Some(((*htab).eq_f).expect("non-null function pointer")))
            .expect("non-null function pointer")(entry, element) != 0
        {
            return &mut *((*htab).entries).offset(index as isize)
                as *mut *mut libc::c_void
        }
        hash2 = htab_mod_m2(hash, htab);
        loop {
            (*htab).collisions = ((*htab).collisions).wrapping_add(1);
            (*htab).collisions;
            index = (index as libc::c_uint).wrapping_add(hash2) as hashval_t
                as hashval_t;
            if index as libc::c_ulong >= size {
                index = (index as libc::c_ulong).wrapping_sub(size) as hashval_t
                    as hashval_t;
            }
            entry = *((*htab).entries).offset(index as isize);
            if entry.is_null() {
                break;
            }
            if entry == 1 as libc::c_int as *mut libc::c_void {
                if first_deleted_slot.is_null() {
                    first_deleted_slot = &mut *((*htab).entries).offset(index as isize)
                        as *mut *mut libc::c_void;
                }
            } else if (Some(((*htab).eq_f).expect("non-null function pointer")))
                .expect("non-null function pointer")(entry, element) != 0
            {
                return &mut *((*htab).entries).offset(index as isize)
                    as *mut *mut libc::c_void
            }
        }
    }
    if insert as libc::c_uint == NO_INSERT as libc::c_int as libc::c_uint {
        return 0 as *mut *mut libc::c_void;
    }
    if !first_deleted_slot.is_null() {
        (*htab).n_deleted = ((*htab).n_deleted).wrapping_sub(1);
        (*htab).n_deleted;
        *first_deleted_slot = 0 as *mut libc::c_void;
        return first_deleted_slot;
    }
    (*htab).n_elements = ((*htab).n_elements).wrapping_add(1);
    (*htab).n_elements;
    return &mut *((*htab).entries).offset(index as isize) as *mut *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn htab_find_slot(
    mut htab: htab_t,
    mut element: *const libc::c_void,
    mut insert: insert_option,
) -> *mut *mut libc::c_void {
    return htab_find_slot_with_hash(
        htab,
        element,
        (Some(((*htab).hash_f).expect("non-null function pointer")))
            .expect("non-null function pointer")(element),
        insert,
    );
}
#[no_mangle]
pub unsafe extern "C" fn htab_remove_elt(
    mut htab: htab_t,
    mut element: *const libc::c_void,
) {
    htab_remove_elt_with_hash(
        htab,
        element,
        (Some(((*htab).hash_f).expect("non-null function pointer")))
            .expect("non-null function pointer")(element),
    );
}
#[no_mangle]
pub unsafe extern "C" fn htab_remove_elt_with_hash(
    mut htab: htab_t,
    mut element: *const libc::c_void,
    mut hash: hashval_t,
) {
    let mut slot: *mut *mut libc::c_void = 0 as *mut *mut libc::c_void;
    slot = htab_find_slot_with_hash(htab, element, hash, NO_INSERT);
    if slot.is_null() {
        return;
    }
    if ((*htab).del_f).is_some() {
        (Some(((*htab).del_f).expect("non-null function pointer")))
            .expect("non-null function pointer")(*slot);
    }
    *slot = 1 as libc::c_int as *mut libc::c_void;
    (*htab).n_deleted = ((*htab).n_deleted).wrapping_add(1);
    (*htab).n_deleted;
}
#[no_mangle]
pub unsafe extern "C" fn htab_clear_slot(
    mut htab: htab_t,
    mut slot: *mut *mut libc::c_void,
) {
    if slot < (*htab).entries || slot >= ((*htab).entries).offset((*htab).size as isize)
        || (*slot).is_null() || *slot == 1 as libc::c_int as *mut libc::c_void
    {
        abort();
    }
    if ((*htab).del_f).is_some() {
        (Some(((*htab).del_f).expect("non-null function pointer")))
            .expect("non-null function pointer")(*slot);
    }
    *slot = 1 as libc::c_int as *mut libc::c_void;
    (*htab).n_deleted = ((*htab).n_deleted).wrapping_add(1);
    (*htab).n_deleted;
}
#[no_mangle]
pub unsafe extern "C" fn htab_traverse_noresize(
    mut htab: htab_t,
    mut callback: htab_trav,
    mut info: *mut libc::c_void,
) {
    let mut slot: *mut *mut libc::c_void = 0 as *mut *mut libc::c_void;
    let mut limit: *mut *mut libc::c_void = 0 as *mut *mut libc::c_void;
    slot = (*htab).entries;
    limit = slot.offset((*htab).size as isize);
    loop {
        let mut x: *mut libc::c_void = *slot;
        if !x.is_null() && x != 1 as libc::c_int as *mut libc::c_void {
            if (Some(callback.expect("non-null function pointer")))
                .expect("non-null function pointer")(slot, info) == 0
            {
                break;
            }
        }
        slot = slot.offset(1);
        if !(slot < limit) {
            break;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn htab_traverse(
    mut htab: htab_t,
    mut callback: htab_trav,
    mut info: *mut libc::c_void,
) {
    let mut size: size_t = (*htab).size;
    if ((*htab).n_elements)
        .wrapping_sub((*htab).n_deleted)
        .wrapping_mul(8 as libc::c_int as libc::c_ulong) < size
        && size > 32 as libc::c_int as libc::c_ulong
    {
        htab_expand(htab);
    }
    htab_traverse_noresize(htab, callback, info);
}
#[no_mangle]
pub unsafe extern "C" fn htab_collisions(mut htab: htab_t) -> libc::c_double {
    if (*htab).searches == 0 as libc::c_int as libc::c_uint {
        return 0.0f64;
    }
    return (*htab).collisions as libc::c_double / (*htab).searches as libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn htab_hash_string(mut p: *const libc::c_void) -> hashval_t {
    let mut str: *const libc::c_uchar = p as *const libc::c_uchar;
    let mut r: hashval_t = 0 as libc::c_int as hashval_t;
    let mut c: libc::c_uchar = 0;
    loop {
        let fresh0 = str;
        str = str.offset(1);
        c = *fresh0;
        if !(c as libc::c_int != 0 as libc::c_int) {
            break;
        }
        r = r
            .wrapping_mul(67 as libc::c_int as libc::c_uint)
            .wrapping_add(c as libc::c_uint)
            .wrapping_sub(113 as libc::c_int as libc::c_uint);
    }
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn htab_eq_string(
    mut a: *const libc::c_void,
    mut b: *const libc::c_void,
) -> libc::c_int {
    return (strcmp(a as *const libc::c_char, b as *const libc::c_char)
        == 0 as libc::c_int) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn iterative_hash(
    mut k_in: *const libc::c_void,
    mut length: size_t,
    mut initval: hashval_t,
) -> hashval_t {
    let mut k: *const libc::c_uchar = k_in as *const libc::c_uchar;
    let mut a: hashval_t = 0;
    let mut b: hashval_t = 0;
    let mut c: hashval_t = 0;
    let mut len: hashval_t = 0;
    len = length as hashval_t;
    b = 0x9e3779b9 as libc::c_uint;
    a = b;
    c = initval;
    if ::core::mem::size_of::<hashval_t>() as libc::c_ulong
        == 4 as libc::c_int as libc::c_ulong
        && k as size_t & 3 as libc::c_int as libc::c_ulong
            == 0 as libc::c_int as libc::c_ulong
    {
        while len >= 12 as libc::c_int as libc::c_uint {
            a = (a as libc::c_uint)
                .wrapping_add(*(k.offset(0 as libc::c_int as isize) as *mut hashval_t))
                as hashval_t as hashval_t;
            b = (b as libc::c_uint)
                .wrapping_add(*(k.offset(4 as libc::c_int as isize) as *mut hashval_t))
                as hashval_t as hashval_t;
            c = (c as libc::c_uint)
                .wrapping_add(*(k.offset(8 as libc::c_int as isize) as *mut hashval_t))
                as hashval_t as hashval_t;
            a = (a as libc::c_uint).wrapping_sub(b) as hashval_t as hashval_t;
            a = (a as libc::c_uint).wrapping_sub(c) as hashval_t as hashval_t;
            a ^= c >> 13 as libc::c_int;
            b = (b as libc::c_uint).wrapping_sub(c) as hashval_t as hashval_t;
            b = (b as libc::c_uint).wrapping_sub(a) as hashval_t as hashval_t;
            b ^= a << 8 as libc::c_int;
            c = (c as libc::c_uint).wrapping_sub(a) as hashval_t as hashval_t;
            c = (c as libc::c_uint).wrapping_sub(b) as hashval_t as hashval_t;
            c ^= (b & 0xffffffff as libc::c_uint) >> 13 as libc::c_int;
            a = (a as libc::c_uint).wrapping_sub(b) as hashval_t as hashval_t;
            a = (a as libc::c_uint).wrapping_sub(c) as hashval_t as hashval_t;
            a ^= (c & 0xffffffff as libc::c_uint) >> 12 as libc::c_int;
            b = (b as libc::c_uint).wrapping_sub(c) as hashval_t as hashval_t;
            b = (b as libc::c_uint).wrapping_sub(a) as hashval_t as hashval_t;
            b = (b ^ a << 16 as libc::c_int) & 0xffffffff as libc::c_uint;
            c = (c as libc::c_uint).wrapping_sub(a) as hashval_t as hashval_t;
            c = (c as libc::c_uint).wrapping_sub(b) as hashval_t as hashval_t;
            c = (c ^ b >> 5 as libc::c_int) & 0xffffffff as libc::c_uint;
            a = (a as libc::c_uint).wrapping_sub(b) as hashval_t as hashval_t;
            a = (a as libc::c_uint).wrapping_sub(c) as hashval_t as hashval_t;
            a = (a ^ c >> 3 as libc::c_int) & 0xffffffff as libc::c_uint;
            b = (b as libc::c_uint).wrapping_sub(c) as hashval_t as hashval_t;
            b = (b as libc::c_uint).wrapping_sub(a) as hashval_t as hashval_t;
            b = (b ^ a << 10 as libc::c_int) & 0xffffffff as libc::c_uint;
            c = (c as libc::c_uint).wrapping_sub(a) as hashval_t as hashval_t;
            c = (c as libc::c_uint).wrapping_sub(b) as hashval_t as hashval_t;
            c = (c ^ b >> 15 as libc::c_int) & 0xffffffff as libc::c_uint;
            k = k.offset(12 as libc::c_int as isize);
            len = (len as libc::c_uint).wrapping_sub(12 as libc::c_int as libc::c_uint)
                as hashval_t as hashval_t;
        }
    } else {
        while len >= 12 as libc::c_int as libc::c_uint {
            a = (a as libc::c_uint)
                .wrapping_add(
                    (*k.offset(0 as libc::c_int as isize) as libc::c_uint)
                        .wrapping_add(
                            (*k.offset(1 as libc::c_int as isize) as hashval_t)
                                << 8 as libc::c_int,
                        )
                        .wrapping_add(
                            (*k.offset(2 as libc::c_int as isize) as hashval_t)
                                << 16 as libc::c_int,
                        )
                        .wrapping_add(
                            (*k.offset(3 as libc::c_int as isize) as hashval_t)
                                << 24 as libc::c_int,
                        ),
                ) as hashval_t as hashval_t;
            b = (b as libc::c_uint)
                .wrapping_add(
                    (*k.offset(4 as libc::c_int as isize) as libc::c_uint)
                        .wrapping_add(
                            (*k.offset(5 as libc::c_int as isize) as hashval_t)
                                << 8 as libc::c_int,
                        )
                        .wrapping_add(
                            (*k.offset(6 as libc::c_int as isize) as hashval_t)
                                << 16 as libc::c_int,
                        )
                        .wrapping_add(
                            (*k.offset(7 as libc::c_int as isize) as hashval_t)
                                << 24 as libc::c_int,
                        ),
                ) as hashval_t as hashval_t;
            c = (c as libc::c_uint)
                .wrapping_add(
                    (*k.offset(8 as libc::c_int as isize) as libc::c_uint)
                        .wrapping_add(
                            (*k.offset(9 as libc::c_int as isize) as hashval_t)
                                << 8 as libc::c_int,
                        )
                        .wrapping_add(
                            (*k.offset(10 as libc::c_int as isize) as hashval_t)
                                << 16 as libc::c_int,
                        )
                        .wrapping_add(
                            (*k.offset(11 as libc::c_int as isize) as hashval_t)
                                << 24 as libc::c_int,
                        ),
                ) as hashval_t as hashval_t;
            a = (a as libc::c_uint).wrapping_sub(b) as hashval_t as hashval_t;
            a = (a as libc::c_uint).wrapping_sub(c) as hashval_t as hashval_t;
            a ^= c >> 13 as libc::c_int;
            b = (b as libc::c_uint).wrapping_sub(c) as hashval_t as hashval_t;
            b = (b as libc::c_uint).wrapping_sub(a) as hashval_t as hashval_t;
            b ^= a << 8 as libc::c_int;
            c = (c as libc::c_uint).wrapping_sub(a) as hashval_t as hashval_t;
            c = (c as libc::c_uint).wrapping_sub(b) as hashval_t as hashval_t;
            c ^= (b & 0xffffffff as libc::c_uint) >> 13 as libc::c_int;
            a = (a as libc::c_uint).wrapping_sub(b) as hashval_t as hashval_t;
            a = (a as libc::c_uint).wrapping_sub(c) as hashval_t as hashval_t;
            a ^= (c & 0xffffffff as libc::c_uint) >> 12 as libc::c_int;
            b = (b as libc::c_uint).wrapping_sub(c) as hashval_t as hashval_t;
            b = (b as libc::c_uint).wrapping_sub(a) as hashval_t as hashval_t;
            b = (b ^ a << 16 as libc::c_int) & 0xffffffff as libc::c_uint;
            c = (c as libc::c_uint).wrapping_sub(a) as hashval_t as hashval_t;
            c = (c as libc::c_uint).wrapping_sub(b) as hashval_t as hashval_t;
            c = (c ^ b >> 5 as libc::c_int) & 0xffffffff as libc::c_uint;
            a = (a as libc::c_uint).wrapping_sub(b) as hashval_t as hashval_t;
            a = (a as libc::c_uint).wrapping_sub(c) as hashval_t as hashval_t;
            a = (a ^ c >> 3 as libc::c_int) & 0xffffffff as libc::c_uint;
            b = (b as libc::c_uint).wrapping_sub(c) as hashval_t as hashval_t;
            b = (b as libc::c_uint).wrapping_sub(a) as hashval_t as hashval_t;
            b = (b ^ a << 10 as libc::c_int) & 0xffffffff as libc::c_uint;
            c = (c as libc::c_uint).wrapping_sub(a) as hashval_t as hashval_t;
            c = (c as libc::c_uint).wrapping_sub(b) as hashval_t as hashval_t;
            c = (c ^ b >> 15 as libc::c_int) & 0xffffffff as libc::c_uint;
            k = k.offset(12 as libc::c_int as isize);
            len = (len as libc::c_uint).wrapping_sub(12 as libc::c_int as libc::c_uint)
                as hashval_t as hashval_t;
        }
    }
    c = (c as libc::c_ulong).wrapping_add(length) as hashval_t as hashval_t;
    let mut current_block_87: u64;
    match len {
        11 => {
            c = (c as libc::c_uint)
                .wrapping_add(
                    (*k.offset(10 as libc::c_int as isize) as hashval_t)
                        << 24 as libc::c_int,
                ) as hashval_t as hashval_t;
            current_block_87 = 13909856113418196919;
        }
        10 => {
            current_block_87 = 13909856113418196919;
        }
        9 => {
            current_block_87 = 18219932089923136765;
        }
        8 => {
            current_block_87 = 7379239181021633694;
        }
        7 => {
            current_block_87 = 6801053210060364487;
        }
        6 => {
            current_block_87 = 2769525310156437215;
        }
        5 => {
            current_block_87 = 5156032796029747150;
        }
        4 => {
            current_block_87 = 14111752540665342400;
        }
        3 => {
            current_block_87 = 2318216883367804037;
        }
        2 => {
            current_block_87 = 4095228106372614377;
        }
        1 => {
            current_block_87 = 18097817891508164753;
        }
        _ => {
            current_block_87 = 2706659501864706830;
        }
    }
    match current_block_87 {
        13909856113418196919 => {
            c = (c as libc::c_uint)
                .wrapping_add(
                    (*k.offset(9 as libc::c_int as isize) as hashval_t)
                        << 16 as libc::c_int,
                ) as hashval_t as hashval_t;
            current_block_87 = 18219932089923136765;
        }
        _ => {}
    }
    match current_block_87 {
        18219932089923136765 => {
            c = (c as libc::c_uint)
                .wrapping_add(
                    (*k.offset(8 as libc::c_int as isize) as hashval_t)
                        << 8 as libc::c_int,
                ) as hashval_t as hashval_t;
            current_block_87 = 7379239181021633694;
        }
        _ => {}
    }
    match current_block_87 {
        7379239181021633694 => {
            b = (b as libc::c_uint)
                .wrapping_add(
                    (*k.offset(7 as libc::c_int as isize) as hashval_t)
                        << 24 as libc::c_int,
                ) as hashval_t as hashval_t;
            current_block_87 = 6801053210060364487;
        }
        _ => {}
    }
    match current_block_87 {
        6801053210060364487 => {
            b = (b as libc::c_uint)
                .wrapping_add(
                    (*k.offset(6 as libc::c_int as isize) as hashval_t)
                        << 16 as libc::c_int,
                ) as hashval_t as hashval_t;
            current_block_87 = 2769525310156437215;
        }
        _ => {}
    }
    match current_block_87 {
        2769525310156437215 => {
            b = (b as libc::c_uint)
                .wrapping_add(
                    (*k.offset(5 as libc::c_int as isize) as hashval_t)
                        << 8 as libc::c_int,
                ) as hashval_t as hashval_t;
            current_block_87 = 5156032796029747150;
        }
        _ => {}
    }
    match current_block_87 {
        5156032796029747150 => {
            b = (b as libc::c_uint)
                .wrapping_add(*k.offset(4 as libc::c_int as isize) as libc::c_uint)
                as hashval_t as hashval_t;
            current_block_87 = 14111752540665342400;
        }
        _ => {}
    }
    match current_block_87 {
        14111752540665342400 => {
            a = (a as libc::c_uint)
                .wrapping_add(
                    (*k.offset(3 as libc::c_int as isize) as hashval_t)
                        << 24 as libc::c_int,
                ) as hashval_t as hashval_t;
            current_block_87 = 2318216883367804037;
        }
        _ => {}
    }
    match current_block_87 {
        2318216883367804037 => {
            a = (a as libc::c_uint)
                .wrapping_add(
                    (*k.offset(2 as libc::c_int as isize) as hashval_t)
                        << 16 as libc::c_int,
                ) as hashval_t as hashval_t;
            current_block_87 = 4095228106372614377;
        }
        _ => {}
    }
    match current_block_87 {
        4095228106372614377 => {
            a = (a as libc::c_uint)
                .wrapping_add(
                    (*k.offset(1 as libc::c_int as isize) as hashval_t)
                        << 8 as libc::c_int,
                ) as hashval_t as hashval_t;
            current_block_87 = 18097817891508164753;
        }
        _ => {}
    }
    match current_block_87 {
        18097817891508164753 => {
            a = (a as libc::c_uint)
                .wrapping_add(*k.offset(0 as libc::c_int as isize) as libc::c_uint)
                as hashval_t as hashval_t;
        }
        _ => {}
    }
    a = (a as libc::c_uint).wrapping_sub(b) as hashval_t as hashval_t;
    a = (a as libc::c_uint).wrapping_sub(c) as hashval_t as hashval_t;
    a ^= c >> 13 as libc::c_int;
    b = (b as libc::c_uint).wrapping_sub(c) as hashval_t as hashval_t;
    b = (b as libc::c_uint).wrapping_sub(a) as hashval_t as hashval_t;
    b ^= a << 8 as libc::c_int;
    c = (c as libc::c_uint).wrapping_sub(a) as hashval_t as hashval_t;
    c = (c as libc::c_uint).wrapping_sub(b) as hashval_t as hashval_t;
    c ^= (b & 0xffffffff as libc::c_uint) >> 13 as libc::c_int;
    a = (a as libc::c_uint).wrapping_sub(b) as hashval_t as hashval_t;
    a = (a as libc::c_uint).wrapping_sub(c) as hashval_t as hashval_t;
    a ^= (c & 0xffffffff as libc::c_uint) >> 12 as libc::c_int;
    b = (b as libc::c_uint).wrapping_sub(c) as hashval_t as hashval_t;
    b = (b as libc::c_uint).wrapping_sub(a) as hashval_t as hashval_t;
    b = (b ^ a << 16 as libc::c_int) & 0xffffffff as libc::c_uint;
    c = (c as libc::c_uint).wrapping_sub(a) as hashval_t as hashval_t;
    c = (c as libc::c_uint).wrapping_sub(b) as hashval_t as hashval_t;
    c = (c ^ b >> 5 as libc::c_int) & 0xffffffff as libc::c_uint;
    a = (a as libc::c_uint).wrapping_sub(b) as hashval_t as hashval_t;
    a = (a as libc::c_uint).wrapping_sub(c) as hashval_t as hashval_t;
    a = (a ^ c >> 3 as libc::c_int) & 0xffffffff as libc::c_uint;
    b = (b as libc::c_uint).wrapping_sub(c) as hashval_t as hashval_t;
    b = (b as libc::c_uint).wrapping_sub(a) as hashval_t as hashval_t;
    b = (b ^ a << 10 as libc::c_int) & 0xffffffff as libc::c_uint;
    c = (c as libc::c_uint).wrapping_sub(a) as hashval_t as hashval_t;
    c = (c as libc::c_uint).wrapping_sub(b) as hashval_t as hashval_t;
    c = (c ^ b >> 15 as libc::c_int) & 0xffffffff as libc::c_uint;
    return c;
}
unsafe extern "C" fn hash_pointer(mut p: *const libc::c_void) -> hashval_t {
    let mut v: intptr_t = p as intptr_t;
    let mut a: libc::c_uint = 0;
    let mut b: libc::c_uint = 0;
    let mut c: libc::c_uint = 0;
    b = 0x9e3779b9 as libc::c_uint;
    a = b;
    a = (a as libc::c_long
        + (v
            >> (::core::mem::size_of::<intptr_t>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                .wrapping_div(2 as libc::c_int as libc::c_ulong))) as libc::c_uint;
    b = (b as libc::c_long
        + (v
            & ((1 as libc::c_int as intptr_t)
                << (::core::mem::size_of::<intptr_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_div(2 as libc::c_int as libc::c_ulong))
                - 1 as libc::c_int as libc::c_long)) as libc::c_uint;
    c = 0x42135234 as libc::c_int as libc::c_uint;
    a = a.wrapping_sub(b);
    a = a.wrapping_sub(c);
    a ^= c >> 13 as libc::c_int;
    b = b.wrapping_sub(c);
    b = b.wrapping_sub(a);
    b ^= a << 8 as libc::c_int;
    c = c.wrapping_sub(a);
    c = c.wrapping_sub(b);
    c ^= (b & 0xffffffff as libc::c_uint) >> 13 as libc::c_int;
    a = a.wrapping_sub(b);
    a = a.wrapping_sub(c);
    a ^= (c & 0xffffffff as libc::c_uint) >> 12 as libc::c_int;
    b = b.wrapping_sub(c);
    b = b.wrapping_sub(a);
    b = (b ^ a << 16 as libc::c_int) & 0xffffffff as libc::c_uint;
    c = c.wrapping_sub(a);
    c = c.wrapping_sub(b);
    c = (c ^ b >> 5 as libc::c_int) & 0xffffffff as libc::c_uint;
    a = a.wrapping_sub(b);
    a = a.wrapping_sub(c);
    a = (a ^ c >> 3 as libc::c_int) & 0xffffffff as libc::c_uint;
    b = b.wrapping_sub(c);
    b = b.wrapping_sub(a);
    b = (b ^ a << 10 as libc::c_int) & 0xffffffff as libc::c_uint;
    c = c.wrapping_sub(a);
    c = c.wrapping_sub(b);
    c = (c ^ b >> 15 as libc::c_int) & 0xffffffff as libc::c_uint;
    return c;
}
