extern "C" {
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn fchmod(__fd: libc::c_int, __mode: __mode_t) -> libc::c_int;
    fn utimensat(
        __fd: libc::c_int,
        __path: *const libc::c_char,
        __times: *const timespec,
        __flags: libc::c_int,
    ) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn lseek(__fd: libc::c_int, __offset: __off_t, __whence: libc::c_int) -> __off_t;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn non_fatal(_: *const libc::c_char, _: ...);
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
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type ssize_t = __ssize_t;
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
pub unsafe extern "C" fn set_times(
    mut destination: *const libc::c_char,
    mut statbuf: *const stat,
) {
    let mut result: libc::c_int = 0;
    let mut times: [timespec; 2] = [timespec { tv_sec: 0, tv_nsec: 0 }; 2];
    times[0 as libc::c_int as usize] = get_stat_atime(statbuf);
    times[1 as libc::c_int as usize] = get_stat_mtime(statbuf);
    result = utimensat(
        -(100 as libc::c_int),
        destination,
        times.as_mut_ptr() as *const timespec,
        0 as libc::c_int,
    );
    if result != 0 as libc::c_int {
        non_fatal(
            dcgettext(
                0 as *const libc::c_char,
                b"%s: cannot set time: %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            destination,
            strerror(*__errno_location()),
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn smart_rename(
    mut from: *const libc::c_char,
    mut to: *const libc::c_char,
    mut fromfd: libc::c_int,
    mut target_stat: *mut stat,
    mut preserve_dates: bool,
) -> libc::c_int {
    let mut ret: libc::c_int = 0 as libc::c_int;
    if to != from {
        ret = simple_copy(fromfd, to, target_stat);
        if ret != 0 as libc::c_int {
            non_fatal(
                dcgettext(
                    0 as *const libc::c_char,
                    b"unable to copy file '%s'; reason: %s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                to,
                strerror(*__errno_location()),
            );
        }
        unlink(from);
    }
    if preserve_dates {
        set_times(to, target_stat);
    }
    return ret;
}
unsafe extern "C" fn simple_copy(
    mut fromfd: libc::c_int,
    mut to: *const libc::c_char,
    mut target_stat: *mut stat,
) -> libc::c_int {
    let mut tofd: libc::c_int = 0;
    let mut nread: libc::c_int = 0;
    let mut saved: libc::c_int = 0;
    let mut buf: [libc::c_char; 8192] = [0; 8192];
    if fromfd < 0 as libc::c_int
        || lseek(fromfd, 0 as libc::c_int as __off_t, 0 as libc::c_int)
            != 0 as libc::c_int as libc::c_long
    {
        return -(1 as libc::c_int);
    }
    tofd = open(to, 0o1 as libc::c_int | 0o1000 as libc::c_int | 0 as libc::c_int);
    if tofd < 0 as libc::c_int {
        saved = *__errno_location();
        close(fromfd);
        *__errno_location() = saved;
        return -(1 as libc::c_int);
    }
    loop {
        nread = read(
            fromfd,
            buf.as_mut_ptr() as *mut libc::c_void,
            ::core::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong,
        ) as libc::c_int;
        if !(nread > 0 as libc::c_int) {
            break;
        }
        if write(tofd, buf.as_mut_ptr() as *const libc::c_void, nread as size_t)
            != nread as libc::c_long
        {
            saved = *__errno_location();
            close(fromfd);
            close(tofd);
            *__errno_location() = saved;
            return -(1 as libc::c_int);
        }
    }
    saved = *__errno_location();
    if !target_stat.is_null() {
        fchmod(tofd, (*target_stat).st_mode);
    }
    close(fromfd);
    close(tofd);
    if nread < 0 as libc::c_int {
        *__errno_location() = saved;
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn get_stat_atime(mut st: *const stat) -> timespec {
    return (*st).st_atim;
}
#[inline]
unsafe extern "C" fn get_stat_mtime(mut st: *const stat) -> timespec {
    return (*st).st_mtim;
}
