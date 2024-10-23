use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn abort() -> !;
    fn free(_: *mut libc::c_void);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct objalloc {
    pub current_ptr: *mut libc::c_char,
    pub current_space: libc::c_uint,
    pub chunks: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct objalloc_chunk {
    pub next: *mut objalloc_chunk,
    pub current_ptr: *mut libc::c_char,
}
#[no_mangle]
pub unsafe extern "C" fn objalloc_create() -> *mut objalloc {
    let mut ret: *mut objalloc = 0 as *mut objalloc;
    let mut chunk: *mut objalloc_chunk = 0 as *mut objalloc_chunk;
    ret = malloc(::core::mem::size_of::<objalloc>() as libc::c_ulong) as *mut objalloc;
    if ret.is_null() {
        return 0 as *mut objalloc;
    }
    (*ret).chunks = malloc((4096 as libc::c_int - 32 as libc::c_int) as libc::c_ulong);
    if ((*ret).chunks).is_null() {
        free(ret as *mut libc::c_void);
        return 0 as *mut objalloc;
    }
    chunk = (*ret).chunks as *mut objalloc_chunk;
    (*chunk).next = 0 as *mut objalloc_chunk;
    (*chunk).current_ptr = 0 as *mut libc::c_char;
    (*ret)
        .current_ptr = (chunk as *mut libc::c_char)
        .offset(
            ((::core::mem::size_of::<objalloc_chunk>() as libc::c_ulong)
                .wrapping_add(8 as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                & !(8 as libc::c_ulong).wrapping_sub(1 as libc::c_int as libc::c_ulong))
                as isize,
        );
    (*ret)
        .current_space = ((4096 as libc::c_int - 32 as libc::c_int) as libc::c_ulong)
        .wrapping_sub(
            (::core::mem::size_of::<objalloc_chunk>() as libc::c_ulong)
                .wrapping_add(8 as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                & !(8 as libc::c_ulong).wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ) as libc::c_uint;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn _objalloc_alloc(
    mut o: *mut objalloc,
    mut original_len: libc::c_ulong,
) -> *mut libc::c_void {
    let mut len: libc::c_ulong = original_len;
    if len == 0 as libc::c_int as libc::c_ulong {
        len = 1 as libc::c_int as libc::c_ulong;
    }
    len = len
        .wrapping_add(8 as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        & !(8 as libc::c_ulong).wrapping_sub(1 as libc::c_int as libc::c_ulong);
    if len
        .wrapping_add(
            (::core::mem::size_of::<objalloc_chunk>() as libc::c_ulong)
                .wrapping_add(8 as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                & !(8 as libc::c_ulong).wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ) < original_len
    {
        return 0 as *mut libc::c_void;
    }
    if len <= (*o).current_space as libc::c_ulong {
        (*o).current_ptr = ((*o).current_ptr).offset(len as isize);
        (*o)
            .current_space = ((*o).current_space as libc::c_ulong).wrapping_sub(len)
            as libc::c_uint as libc::c_uint;
        return ((*o).current_ptr).offset(-(len as isize)) as *mut libc::c_void;
    }
    if len >= 512 as libc::c_int as libc::c_ulong {
        let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut chunk: *mut objalloc_chunk = 0 as *mut objalloc_chunk;
        ret = malloc(
            ((::core::mem::size_of::<objalloc_chunk>() as libc::c_ulong)
                .wrapping_add(8 as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                & !(8 as libc::c_ulong).wrapping_sub(1 as libc::c_int as libc::c_ulong))
                .wrapping_add(len),
        ) as *mut libc::c_char;
        if ret.is_null() {
            return 0 as *mut libc::c_void;
        }
        chunk = ret as *mut objalloc_chunk;
        (*chunk).next = (*o).chunks as *mut objalloc_chunk;
        (*chunk).current_ptr = (*o).current_ptr;
        (*o).chunks = chunk as *mut libc::c_void;
        return ret
            .offset(
                ((::core::mem::size_of::<objalloc_chunk>() as libc::c_ulong)
                    .wrapping_add(8 as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    & !(8 as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)) as isize,
            ) as *mut libc::c_void;
    } else {
        let mut chunk_0: *mut objalloc_chunk = 0 as *mut objalloc_chunk;
        chunk_0 = malloc((4096 as libc::c_int - 32 as libc::c_int) as libc::c_ulong)
            as *mut objalloc_chunk;
        if chunk_0.is_null() {
            return 0 as *mut libc::c_void;
        }
        (*chunk_0).next = (*o).chunks as *mut objalloc_chunk;
        (*chunk_0).current_ptr = 0 as *mut libc::c_char;
        (*o)
            .current_ptr = (chunk_0 as *mut libc::c_char)
            .offset(
                ((::core::mem::size_of::<objalloc_chunk>() as libc::c_ulong)
                    .wrapping_add(8 as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    & !(8 as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)) as isize,
            );
        (*o)
            .current_space = ((4096 as libc::c_int - 32 as libc::c_int) as libc::c_ulong)
            .wrapping_sub(
                (::core::mem::size_of::<objalloc_chunk>() as libc::c_ulong)
                    .wrapping_add(8 as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    & !(8 as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            ) as libc::c_uint;
        (*o).chunks = chunk_0 as *mut libc::c_void;
        return ({
            let mut __o: *mut objalloc = o;
            let mut __len: libc::c_ulong = len;
            if __len == 0 as libc::c_int as libc::c_ulong {
                __len = 1 as libc::c_int as libc::c_ulong;
            }
            __len = __len
                .wrapping_add(8 as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                & !(8 as libc::c_ulong).wrapping_sub(1 as libc::c_int as libc::c_ulong);
            if __len != 0 as libc::c_int as libc::c_ulong
                && __len <= (*__o).current_space as libc::c_ulong
            {
                (*__o).current_ptr = ((*__o).current_ptr).offset(__len as isize);
                (*__o)
                    .current_space = ((*__o).current_space as libc::c_ulong)
                    .wrapping_sub(__len) as libc::c_uint as libc::c_uint;
                ((*__o).current_ptr).offset(-(__len as isize)) as *mut libc::c_void
            } else {
                _objalloc_alloc(__o, __len)
            }
        });
    };
}
#[no_mangle]
pub unsafe extern "C" fn objalloc_free(mut o: *mut objalloc) {
    let mut l: *mut objalloc_chunk = 0 as *mut objalloc_chunk;
    l = (*o).chunks as *mut objalloc_chunk;
    while !l.is_null() {
        let mut next: *mut objalloc_chunk = 0 as *mut objalloc_chunk;
        next = (*l).next;
        free(l as *mut libc::c_void);
        l = next;
    }
    free(o as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn objalloc_free_block(
    mut o: *mut objalloc,
    mut block: *mut libc::c_void,
) {
    let mut p: *mut objalloc_chunk = 0 as *mut objalloc_chunk;
    let mut small: *mut objalloc_chunk = 0 as *mut objalloc_chunk;
    let mut b: *mut libc::c_char = block as *mut libc::c_char;
    small = 0 as *mut objalloc_chunk;
    p = (*o).chunks as *mut objalloc_chunk;
    while !p.is_null() {
        if ((*p).current_ptr).is_null() {
            if b > p as *mut libc::c_char
                && b
                    < (p as *mut libc::c_char)
                        .offset((4096 as libc::c_int - 32 as libc::c_int) as isize)
            {
                break;
            }
            small = p;
        } else if b
            == (p as *mut libc::c_char)
                .offset(
                    ((::core::mem::size_of::<objalloc_chunk>() as libc::c_ulong)
                        .wrapping_add(8 as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        & !(8 as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)) as isize,
                )
        {
            break;
        }
        p = (*p).next;
    }
    if p.is_null() {
        abort();
    }
    if ((*p).current_ptr).is_null() {
        let mut q: *mut objalloc_chunk = 0 as *mut objalloc_chunk;
        let mut first: *mut objalloc_chunk = 0 as *mut objalloc_chunk;
        first = 0 as *mut objalloc_chunk;
        q = (*o).chunks as *mut objalloc_chunk;
        while q != p {
            let mut next: *mut objalloc_chunk = 0 as *mut objalloc_chunk;
            next = (*q).next;
            if !small.is_null() {
                if small == q {
                    small = 0 as *mut objalloc_chunk;
                }
                free(q as *mut libc::c_void);
            } else if (*q).current_ptr > b {
                free(q as *mut libc::c_void);
            } else if first.is_null() {
                first = q;
            }
            q = next;
        }
        if first.is_null() {
            first = p;
        }
        (*o).chunks = first as *mut libc::c_void;
        (*o).current_ptr = b;
        (*o)
            .current_space = (p as *mut libc::c_char)
            .offset((4096 as libc::c_int - 32 as libc::c_int) as isize)
            .offset_from(b) as libc::c_long as libc::c_uint;
    } else {
        let mut q_0: *mut objalloc_chunk = 0 as *mut objalloc_chunk;
        let mut current_ptr: *mut libc::c_char = 0 as *mut libc::c_char;
        current_ptr = (*p).current_ptr;
        p = (*p).next;
        q_0 = (*o).chunks as *mut objalloc_chunk;
        while q_0 != p {
            let mut next_0: *mut objalloc_chunk = 0 as *mut objalloc_chunk;
            next_0 = (*q_0).next;
            free(q_0 as *mut libc::c_void);
            q_0 = next_0;
        }
        (*o).chunks = p as *mut libc::c_void;
        while !((*p).current_ptr).is_null() {
            p = (*p).next;
        }
        (*o).current_ptr = current_ptr;
        (*o)
            .current_space = (p as *mut libc::c_char)
            .offset((4096 as libc::c_int - 32 as libc::c_int) as isize)
            .offset_from(current_ptr) as libc::c_long as libc::c_uint;
    };
}
