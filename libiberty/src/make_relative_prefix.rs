use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn access(__name: *const libc::c_char, __type: libc::c_int) -> libc::c_int;
    fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn lbasename(_: *const libc::c_char) -> *const libc::c_char;
    fn lrealpath(_: *const libc::c_char) -> *mut libc::c_char;
}
pub type size_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
unsafe extern "C" fn save_string(
    mut s: *const libc::c_char,
    mut len: libc::c_int,
) -> *mut libc::c_char {
    let mut result: *mut libc::c_char = malloc((len + 1 as libc::c_int) as libc::c_ulong)
        as *mut libc::c_char;
    memcpy(result as *mut libc::c_void, s as *const libc::c_void, len as libc::c_ulong);
    *result.offset(len as isize) = 0 as libc::c_int as libc::c_char;
    return result;
}
unsafe extern "C" fn split_directories(
    mut name: *const libc::c_char,
    mut ptr_num_dirs: *mut libc::c_int,
) -> *mut *mut libc::c_char {
    let mut num_dirs: libc::c_int = 0 as libc::c_int;
    let mut dirs: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut q: *const libc::c_char = 0 as *const libc::c_char;
    let mut ch: libc::c_int = 0;
    if *name == 0 {
        return 0 as *mut *mut libc::c_char;
    }
    p = name;
    loop {
        let fresh0 = p;
        p = p.offset(1);
        ch = *fresh0 as libc::c_int;
        if !(ch != '\0' as i32) {
            break;
        }
        if ch == '/' as i32 {
            num_dirs += 1;
            num_dirs;
            while *p as libc::c_int == '/' as i32 {
                p = p.offset(1);
                p;
            }
        }
    }
    dirs = malloc(
        (::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
            .wrapping_mul((num_dirs + 2 as libc::c_int) as libc::c_ulong),
    ) as *mut *mut libc::c_char;
    if dirs.is_null() {
        return 0 as *mut *mut libc::c_char;
    }
    num_dirs = 0 as libc::c_int;
    p = name;
    q = p;
    loop {
        let fresh1 = p;
        p = p.offset(1);
        ch = *fresh1 as libc::c_int;
        if !(ch != '\0' as i32) {
            break;
        }
        if ch == '/' as i32 {
            while *p as libc::c_int == '/' as i32 {
                p = p.offset(1);
                p;
            }
            let fresh2 = num_dirs;
            num_dirs = num_dirs + 1;
            let ref mut fresh3 = *dirs.offset(fresh2 as isize);
            *fresh3 = save_string(q, p.offset_from(q) as libc::c_long as libc::c_int);
            if (*dirs.offset((num_dirs - 1 as libc::c_int) as isize)).is_null() {
                let ref mut fresh4 = *dirs.offset(num_dirs as isize);
                *fresh4 = 0 as *mut libc::c_char;
                free_split_directories(dirs);
                return 0 as *mut *mut libc::c_char;
            }
            q = p;
        }
    }
    if p.offset(-(1 as libc::c_int as isize)).offset_from(q) as libc::c_long
        > 0 as libc::c_int as libc::c_long
    {
        let fresh5 = num_dirs;
        num_dirs = num_dirs + 1;
        let ref mut fresh6 = *dirs.offset(fresh5 as isize);
        *fresh6 = save_string(
            q,
            p.offset(-(1 as libc::c_int as isize)).offset_from(q) as libc::c_long
                as libc::c_int,
        );
    }
    let ref mut fresh7 = *dirs.offset(num_dirs as isize);
    *fresh7 = 0 as *mut libc::c_char;
    if (*dirs.offset((num_dirs - 1 as libc::c_int) as isize)).is_null() {
        free_split_directories(dirs);
        return 0 as *mut *mut libc::c_char;
    }
    if !ptr_num_dirs.is_null() {
        *ptr_num_dirs = num_dirs;
    }
    return dirs;
}
unsafe extern "C" fn free_split_directories(mut dirs: *mut *mut libc::c_char) {
    let mut i: libc::c_int = 0 as libc::c_int;
    if !dirs.is_null() {
        while !(*dirs.offset(i as isize)).is_null() {
            let fresh8 = i;
            i = i + 1;
            free(*dirs.offset(fresh8 as isize) as *mut libc::c_void);
        }
        free(dirs as *mut libc::c_char as *mut libc::c_void);
    }
}
unsafe extern "C" fn make_relative_prefix_1(
    mut progname: *const libc::c_char,
    mut bin_prefix: *const libc::c_char,
    mut prefix: *const libc::c_char,
    resolve_links: libc::c_int,
) -> *mut libc::c_char {
    let mut current_block: u64;
    let mut prog_dirs: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut bin_dirs: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut prefix_dirs: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut prog_num: libc::c_int = 0;
    let mut bin_num: libc::c_int = 0;
    let mut prefix_num: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut common: libc::c_int = 0;
    let mut needed_len: libc::c_int = 0;
    let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut full_progname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut alloc_ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    if progname.is_null() || bin_prefix.is_null() || prefix.is_null() {
        return 0 as *mut libc::c_char;
    }
    if lbasename(progname) == progname {
        let mut temp: *mut libc::c_char = 0 as *mut libc::c_char;
        temp = getenv(b"PATH\0" as *const u8 as *const libc::c_char);
        if !temp.is_null() {
            let mut startp: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut endp: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut nstore: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut prefixlen: size_t = (strlen(temp))
                .wrapping_add(1 as libc::c_int as libc::c_ulong);
            let mut len: size_t = 0;
            if prefixlen < 2 as libc::c_int as libc::c_ulong {
                prefixlen = 2 as libc::c_int as size_t;
            }
            len = prefixlen
                .wrapping_add(strlen(progname))
                .wrapping_add(1 as libc::c_int as libc::c_ulong);
            if len < 4032 as libc::c_int as libc::c_ulong {
                let mut fresh9 = ::std::vec::from_elem(0, len as usize);
                nstore = fresh9.as_mut_ptr() as *mut libc::c_char;
            } else {
                nstore = malloc(len) as *mut libc::c_char;
                alloc_ptr = nstore;
            }
            endp = temp;
            startp = endp;
            loop {
                if *endp as libc::c_int == ':' as i32
                    || *endp as libc::c_int == 0 as libc::c_int
                {
                    if endp == startp {
                        *nstore
                            .offset(
                                0 as libc::c_int as isize,
                            ) = '.' as i32 as libc::c_char;
                        *nstore
                            .offset(
                                1 as libc::c_int as isize,
                            ) = '/' as i32 as libc::c_char;
                        *nstore
                            .offset(
                                2 as libc::c_int as isize,
                            ) = '\0' as i32 as libc::c_char;
                    } else {
                        memcpy(
                            nstore as *mut libc::c_void,
                            startp as *const libc::c_void,
                            endp.offset_from(startp) as libc::c_long as libc::c_ulong,
                        );
                        if !(*endp.offset(-(1 as libc::c_int) as isize) as libc::c_int
                            == '/' as i32)
                        {
                            *nstore
                                .offset(
                                    endp.offset_from(startp) as libc::c_long as isize,
                                ) = '/' as i32 as libc::c_char;
                            *nstore
                                .offset(
                                    (endp.offset_from(startp) as libc::c_long
                                        + 1 as libc::c_int as libc::c_long) as isize,
                                ) = 0 as libc::c_int as libc::c_char;
                        } else {
                            *nstore
                                .offset(
                                    endp.offset_from(startp) as libc::c_long as isize,
                                ) = 0 as libc::c_int as libc::c_char;
                        }
                    }
                    strcat(nstore, progname);
                    if access(nstore, 1 as libc::c_int) == 0 {
                        let mut st: stat = stat {
                            st_dev: 0,
                            st_ino: 0,
                            st_nlink: 0,
                            st_mode: 0,
                            st_uid: 0,
                            st_gid: 0,
                            __pad0: 0,
                            st_rdev: 0,
                            st_size: 0,
                            st_blksize: 0,
                            st_blocks: 0,
                            st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
                            st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
                            st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
                            __glibc_reserved: [0; 3],
                        };
                        if stat(nstore, &mut st) >= 0 as libc::c_int
                            && st.st_mode & 0o170000 as libc::c_int as libc::c_uint
                                == 0o100000 as libc::c_int as libc::c_uint
                        {
                            progname = nstore;
                            break;
                        }
                    }
                    if *endp as libc::c_int == 0 as libc::c_int {
                        break;
                    }
                    startp = endp.offset(1 as libc::c_int as isize);
                    endp = startp;
                } else {
                    endp = endp.offset(1);
                    endp;
                }
            }
        }
    }
    if resolve_links != 0 {
        full_progname = lrealpath(progname);
    } else {
        full_progname = strdup(progname);
    }
    if !full_progname.is_null() {
        prog_dirs = split_directories(full_progname, &mut prog_num);
        free(full_progname as *mut libc::c_void);
        if !prog_dirs.is_null() {
            bin_dirs = split_directories(bin_prefix, &mut bin_num);
            if !bin_dirs.is_null() {
                prog_num -= 1;
                prog_num;
                if prog_num == bin_num {
                    i = 0 as libc::c_int;
                    while i < bin_num {
                        if strcmp(
                            *prog_dirs.offset(i as isize),
                            *bin_dirs.offset(i as isize),
                        ) != 0 as libc::c_int
                        {
                            break;
                        }
                        i += 1;
                        i;
                    }
                    if prog_num <= 0 as libc::c_int || i == bin_num {
                        current_block = 12740058906064627462;
                    } else {
                        current_block = 10150597327160359210;
                    }
                } else {
                    current_block = 10150597327160359210;
                }
                match current_block {
                    12740058906064627462 => {}
                    _ => {
                        prefix_dirs = split_directories(prefix, &mut prefix_num);
                        if !prefix_dirs.is_null() {
                            n = if prefix_num < bin_num { prefix_num } else { bin_num };
                            common = 0 as libc::c_int;
                            while common < n {
                                if strcmp(
                                    *bin_dirs.offset(common as isize),
                                    *prefix_dirs.offset(common as isize),
                                ) != 0 as libc::c_int
                                {
                                    break;
                                }
                                common += 1;
                                common;
                            }
                            if !(common == 0 as libc::c_int) {
                                needed_len = 0 as libc::c_int;
                                i = 0 as libc::c_int;
                                while i < prog_num {
                                    needed_len = (needed_len as libc::c_ulong)
                                        .wrapping_add(strlen(*prog_dirs.offset(i as isize)))
                                        as libc::c_int as libc::c_int;
                                    i += 1;
                                    i;
                                }
                                needed_len = (needed_len as libc::c_ulong)
                                    .wrapping_add(
                                        (::core::mem::size_of::<[libc::c_char; 3]>()
                                            as libc::c_ulong)
                                            .wrapping_mul((bin_num - common) as libc::c_ulong),
                                    ) as libc::c_int as libc::c_int;
                                i = common;
                                while i < prefix_num {
                                    needed_len = (needed_len as libc::c_ulong)
                                        .wrapping_add(strlen(*prefix_dirs.offset(i as isize)))
                                        as libc::c_int as libc::c_int;
                                    i += 1;
                                    i;
                                }
                                needed_len += 1 as libc::c_int;
                                ret = malloc(needed_len as libc::c_ulong)
                                    as *mut libc::c_char;
                                if !ret.is_null() {
                                    *ret = '\0' as i32 as libc::c_char;
                                    i = 0 as libc::c_int;
                                    while i < prog_num {
                                        strcat(ret, *prog_dirs.offset(i as isize));
                                        i += 1;
                                        i;
                                    }
                                    ptr = ret.offset(strlen(ret) as isize);
                                    i = common;
                                    while i < bin_num {
                                        strcpy(ptr, b"..\0" as *const u8 as *const libc::c_char);
                                        ptr = ptr
                                            .offset(
                                                (::core::mem::size_of::<[libc::c_char; 3]>()
                                                    as libc::c_ulong)
                                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                                            );
                                        let fresh10 = ptr;
                                        ptr = ptr.offset(1);
                                        *fresh10 = '/' as i32 as libc::c_char;
                                        i += 1;
                                        i;
                                    }
                                    *ptr = '\0' as i32 as libc::c_char;
                                    i = common;
                                    while i < prefix_num {
                                        strcat(ret, *prefix_dirs.offset(i as isize));
                                        i += 1;
                                        i;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    free_split_directories(prog_dirs);
    free_split_directories(bin_dirs);
    free_split_directories(prefix_dirs);
    free(alloc_ptr as *mut libc::c_void);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn make_relative_prefix(
    mut progname: *const libc::c_char,
    mut bin_prefix: *const libc::c_char,
    mut prefix: *const libc::c_char,
) -> *mut libc::c_char {
    return make_relative_prefix_1(progname, bin_prefix, prefix, 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn make_relative_prefix_ignore_links(
    mut progname: *const libc::c_char,
    mut bin_prefix: *const libc::c_char,
    mut prefix: *const libc::c_char,
) -> *mut libc::c_char {
    return make_relative_prefix_1(progname, bin_prefix, prefix, 0 as libc::c_int);
}
