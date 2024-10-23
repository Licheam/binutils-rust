use ::libc;
extern "C" {
    fn __errno_location() -> *mut libc::c_int;
    fn free(_: *mut libc::c_void);
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn getcwd(__buf: *mut libc::c_char, __size: size_t) -> *mut libc::c_char;
    fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    fn xmalloc(_: size_t) -> *mut libc::c_void;
}
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
pub type size_t = libc::c_ulong;
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
#[no_mangle]
pub unsafe extern "C" fn getpwd() -> *mut libc::c_char {
    static mut pwd: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
    static mut failure_errno: libc::c_int = 0;
    let mut p: *mut libc::c_char = pwd;
    let mut s: size_t = 0;
    let mut dotstat: stat = stat {
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
    let mut pwdstat: stat = stat {
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
    if p.is_null()
        && {
            let ref mut fresh0 = *__errno_location();
            *fresh0 = failure_errno;
            *fresh0 == 0
        }
    {
        p = getenv(b"PWD\0" as *const u8 as *const libc::c_char);
        if !(!p.is_null() && *p as libc::c_int == '/' as i32
            && stat(p, &mut pwdstat) == 0 as libc::c_int
            && stat(b".\0" as *const u8 as *const libc::c_char, &mut dotstat)
                == 0 as libc::c_int && dotstat.st_ino == pwdstat.st_ino
            && dotstat.st_dev == pwdstat.st_dev)
        {
            s = (4096 as libc::c_int + 1 as libc::c_int) as size_t;
            loop {
                p = xmalloc(
                    (::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
                        .wrapping_mul(s),
                ) as *mut libc::c_char;
                if !(getcwd(p, s)).is_null() {
                    break;
                }
                let mut e: libc::c_int = *__errno_location();
                free(p as *mut libc::c_void);
                if e != 34 as libc::c_int {
                    failure_errno = e;
                    *__errno_location() = failure_errno;
                    p = 0 as *mut libc::c_char;
                    break;
                } else {
                    s = (s as libc::c_ulong)
                        .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t
                        as size_t;
                }
            }
        }
        pwd = p;
    }
    return p;
}
