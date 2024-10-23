use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fdopen(__fd: libc::c_int, __modes: *const libc::c_char) -> *mut FILE;
    fn xstrerror(_: libc::c_int) -> *mut libc::c_char;
    fn pex_init_common(
        _: libc::c_int,
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: *const pex_funcs,
    ) -> *mut pex_obj;
    static mut environ: *mut *mut libc::c_char;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn pipe(__pipedes: *mut libc::c_int) -> libc::c_int;
    fn pipe2(__pipedes: *mut libc::c_int, __flags: libc::c_int) -> libc::c_int;
    fn sleep(__seconds: libc::c_uint) -> libc::c_uint;
    fn dup2(__fd: libc::c_int, __fd2: libc::c_int) -> libc::c_int;
    fn execv(
        __path: *const libc::c_char,
        __argv: *const *mut libc::c_char,
    ) -> libc::c_int;
    fn execvp(
        __file: *const libc::c_char,
        __argv: *const *mut libc::c_char,
    ) -> libc::c_int;
    fn _exit(_: libc::c_int) -> !;
    fn vfork() -> libc::c_int;
    fn kill(__pid: __pid_t, __sig: libc::c_int) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn waitpid(
        __pid: __pid_t,
        __stat_loc: *mut libc::c_int,
        __options: libc::c_int,
    ) -> __pid_t;
    fn wait4(
        __pid: __pid_t,
        __stat_loc: *mut libc::c_int,
        __options: libc::c_int,
        __usage: *mut rusage,
    ) -> __pid_t;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
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
pub type ssize_t = __ssize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pex_obj {
    pub flags: libc::c_int,
    pub pname: *const libc::c_char,
    pub tempbase: *const libc::c_char,
    pub next_input: libc::c_int,
    pub next_input_name: *mut libc::c_char,
    pub next_input_name_allocated: libc::c_int,
    pub stderr_pipe: libc::c_int,
    pub count: libc::c_int,
    pub children: *mut pid_t,
    pub status: *mut libc::c_int,
    pub time: *mut pex_time,
    pub number_waited: libc::c_int,
    pub input_file: *mut FILE,
    pub read_output: *mut FILE,
    pub read_err: *mut FILE,
    pub remove_count: libc::c_int,
    pub remove: *mut *mut libc::c_char,
    pub funcs: *const pex_funcs,
    pub sysdep: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pex_funcs {
    pub open_read: Option::<
        unsafe extern "C" fn(
            *mut pex_obj,
            *const libc::c_char,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub open_write: Option::<
        unsafe extern "C" fn(
            *mut pex_obj,
            *const libc::c_char,
            libc::c_int,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub exec_child: Option::<
        unsafe extern "C" fn(
            *mut pex_obj,
            libc::c_int,
            *const libc::c_char,
            *const *mut libc::c_char,
            *const *mut libc::c_char,
            libc::c_int,
            libc::c_int,
            libc::c_int,
            libc::c_int,
            *mut *const libc::c_char,
            *mut libc::c_int,
        ) -> pid_t,
    >,
    pub close: Option::<unsafe extern "C" fn(*mut pex_obj, libc::c_int) -> libc::c_int>,
    pub wait: Option::<
        unsafe extern "C" fn(
            *mut pex_obj,
            pid_t,
            *mut libc::c_int,
            *mut pex_time,
            libc::c_int,
            *mut *const libc::c_char,
            *mut libc::c_int,
        ) -> pid_t,
    >,
    pub pipe: Option::<
        unsafe extern "C" fn(*mut pex_obj, *mut libc::c_int, libc::c_int) -> libc::c_int,
    >,
    pub fdopenr: Option::<
        unsafe extern "C" fn(*mut pex_obj, libc::c_int, libc::c_int) -> *mut FILE,
    >,
    pub fdopenw: Option::<
        unsafe extern "C" fn(*mut pex_obj, libc::c_int, libc::c_int) -> *mut FILE,
    >,
    pub cleanup: Option::<unsafe extern "C" fn(*mut pex_obj) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pex_time {
    pub user_seconds: libc::c_ulong,
    pub user_microseconds: libc::c_ulong,
    pub system_seconds: libc::c_ulong,
    pub system_microseconds: libc::c_ulong,
}
pub type pid_t = __pid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rusage {
    pub ru_utime: timeval,
    pub ru_stime: timeval,
    pub c2rust_unnamed: C2RustUnnamed_12,
    pub c2rust_unnamed_0: C2RustUnnamed_11,
    pub c2rust_unnamed_1: C2RustUnnamed_10,
    pub c2rust_unnamed_2: C2RustUnnamed_9,
    pub c2rust_unnamed_3: C2RustUnnamed_8,
    pub c2rust_unnamed_4: C2RustUnnamed_7,
    pub c2rust_unnamed_5: C2RustUnnamed_6,
    pub c2rust_unnamed_6: C2RustUnnamed_5,
    pub c2rust_unnamed_7: C2RustUnnamed_4,
    pub c2rust_unnamed_8: C2RustUnnamed_3,
    pub c2rust_unnamed_9: C2RustUnnamed_2,
    pub c2rust_unnamed_10: C2RustUnnamed_1,
    pub c2rust_unnamed_11: C2RustUnnamed_0,
    pub c2rust_unnamed_12: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub ru_nivcsw: libc::c_long,
    pub __ru_nivcsw_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub ru_nvcsw: libc::c_long,
    pub __ru_nvcsw_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub ru_nsignals: libc::c_long,
    pub __ru_nsignals_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub ru_msgrcv: libc::c_long,
    pub __ru_msgrcv_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub ru_msgsnd: libc::c_long,
    pub __ru_msgsnd_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub ru_oublock: libc::c_long,
    pub __ru_oublock_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
    pub ru_inblock: libc::c_long,
    pub __ru_inblock_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
    pub ru_nswap: libc::c_long,
    pub __ru_nswap_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
    pub ru_majflt: libc::c_long,
    pub __ru_majflt_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_8 {
    pub ru_minflt: libc::c_long,
    pub __ru_minflt_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_9 {
    pub ru_isrss: libc::c_long,
    pub __ru_isrss_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_10 {
    pub ru_idrss: libc::c_long,
    pub __ru_idrss_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_11 {
    pub ru_ixrss: libc::c_long,
    pub __ru_ixrss_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_12 {
    pub ru_maxrss: libc::c_long,
    pub __ru_maxrss_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fn_err {
    pub fn_0: *const libc::c_char,
    pub err: libc::c_int,
}
unsafe extern "C" fn pex_wait(
    mut obj: *mut pex_obj,
    mut pid: pid_t,
    mut status: *mut libc::c_int,
    mut time: *mut pex_time,
) -> pid_t {
    let mut ret: pid_t = 0;
    let mut r: rusage = rusage {
        ru_utime: timeval { tv_sec: 0, tv_usec: 0 },
        ru_stime: timeval { tv_sec: 0, tv_usec: 0 },
        c2rust_unnamed: C2RustUnnamed_12 { ru_maxrss: 0 },
        c2rust_unnamed_0: C2RustUnnamed_11 { ru_ixrss: 0 },
        c2rust_unnamed_1: C2RustUnnamed_10 { ru_idrss: 0 },
        c2rust_unnamed_2: C2RustUnnamed_9 { ru_isrss: 0 },
        c2rust_unnamed_3: C2RustUnnamed_8 { ru_minflt: 0 },
        c2rust_unnamed_4: C2RustUnnamed_7 { ru_majflt: 0 },
        c2rust_unnamed_5: C2RustUnnamed_6 { ru_nswap: 0 },
        c2rust_unnamed_6: C2RustUnnamed_5 { ru_inblock: 0 },
        c2rust_unnamed_7: C2RustUnnamed_4 { ru_oublock: 0 },
        c2rust_unnamed_8: C2RustUnnamed_3 { ru_msgsnd: 0 },
        c2rust_unnamed_9: C2RustUnnamed_2 { ru_msgrcv: 0 },
        c2rust_unnamed_10: C2RustUnnamed_1 { ru_nsignals: 0 },
        c2rust_unnamed_11: C2RustUnnamed_0 { ru_nvcsw: 0 },
        c2rust_unnamed_12: C2RustUnnamed { ru_nivcsw: 0 },
    };
    if time.is_null() {
        return waitpid(pid, status, 0 as libc::c_int);
    }
    ret = wait4(pid, status, 0 as libc::c_int, &mut r);
    if !time.is_null() {
        (*time).user_seconds = r.ru_utime.tv_sec as libc::c_ulong;
        (*time).user_microseconds = r.ru_utime.tv_usec as libc::c_ulong;
        (*time).system_seconds = r.ru_stime.tv_sec as libc::c_ulong;
        (*time).system_microseconds = r.ru_stime.tv_usec as libc::c_ulong;
    }
    return ret;
}
#[no_mangle]
pub static mut funcs: pex_funcs = unsafe {
    {
        let mut init = pex_funcs {
            open_read: Some(
                pex_unix_open_read
                    as unsafe extern "C" fn(
                        *mut pex_obj,
                        *const libc::c_char,
                        libc::c_int,
                    ) -> libc::c_int,
            ),
            open_write: Some(
                pex_unix_open_write
                    as unsafe extern "C" fn(
                        *mut pex_obj,
                        *const libc::c_char,
                        libc::c_int,
                        libc::c_int,
                    ) -> libc::c_int,
            ),
            exec_child: Some(
                pex_unix_exec_child
                    as unsafe extern "C" fn(
                        *mut pex_obj,
                        libc::c_int,
                        *const libc::c_char,
                        *const *mut libc::c_char,
                        *const *mut libc::c_char,
                        libc::c_int,
                        libc::c_int,
                        libc::c_int,
                        libc::c_int,
                        *mut *const libc::c_char,
                        *mut libc::c_int,
                    ) -> pid_t,
            ),
            close: Some(
                pex_unix_close
                    as unsafe extern "C" fn(*mut pex_obj, libc::c_int) -> libc::c_int,
            ),
            wait: Some(
                pex_unix_wait
                    as unsafe extern "C" fn(
                        *mut pex_obj,
                        pid_t,
                        *mut libc::c_int,
                        *mut pex_time,
                        libc::c_int,
                        *mut *const libc::c_char,
                        *mut libc::c_int,
                    ) -> libc::c_int,
            ),
            pipe: Some(
                pex_unix_pipe
                    as unsafe extern "C" fn(
                        *mut pex_obj,
                        *mut libc::c_int,
                        libc::c_int,
                    ) -> libc::c_int,
            ),
            fdopenr: Some(
                pex_unix_fdopenr
                    as unsafe extern "C" fn(
                        *mut pex_obj,
                        libc::c_int,
                        libc::c_int,
                    ) -> *mut FILE,
            ),
            fdopenw: Some(
                pex_unix_fdopenw
                    as unsafe extern "C" fn(
                        *mut pex_obj,
                        libc::c_int,
                        libc::c_int,
                    ) -> *mut FILE,
            ),
            cleanup: Some(pex_unix_cleanup as unsafe extern "C" fn(*mut pex_obj) -> ()),
        };
        init
    }
};
#[no_mangle]
pub unsafe extern "C" fn pex_init(
    mut flags: libc::c_int,
    mut pname: *const libc::c_char,
    mut tempbase: *const libc::c_char,
) -> *mut pex_obj {
    return pex_init_common(flags, pname, tempbase, &funcs);
}
unsafe extern "C" fn pex_unix_open_read(
    mut obj: *mut pex_obj,
    mut name: *const libc::c_char,
    mut binary: libc::c_int,
) -> libc::c_int {
    return open(name, 0 as libc::c_int);
}
unsafe extern "C" fn pex_unix_open_write(
    mut obj: *mut pex_obj,
    mut name: *const libc::c_char,
    mut binary: libc::c_int,
    mut append: libc::c_int,
) -> libc::c_int {
    return open(
        name,
        0o1 as libc::c_int | 0o100 as libc::c_int
            | (if append != 0 { 0o2000 as libc::c_int } else { 0o1000 as libc::c_int }),
        0o400 as libc::c_int | 0o200 as libc::c_int
            | 0o400 as libc::c_int >> 3 as libc::c_int
            | 0o200 as libc::c_int >> 3 as libc::c_int
            | 0o400 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
            | 0o200 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int,
    );
}
unsafe extern "C" fn pex_unix_close(
    mut obj: *mut pex_obj,
    mut fd: libc::c_int,
) -> libc::c_int {
    return close(fd);
}
unsafe extern "C" fn pex_unix_exec_child(
    mut obj: *mut pex_obj,
    mut flags: libc::c_int,
    mut executable: *const libc::c_char,
    mut argv: *const *mut libc::c_char,
    mut env: *const *mut libc::c_char,
    mut in_0: libc::c_int,
    mut out: libc::c_int,
    mut errdes: libc::c_int,
    mut toclose: libc::c_int,
    mut errmsg: *mut *const libc::c_char,
    mut err: *mut libc::c_int,
) -> pid_t {
    let mut pid: pid_t = -(1 as libc::c_int);
    let mut do_pipe: libc::c_int = 0 as libc::c_int;
    let mut pipes: [libc::c_int; 2] = [0; 2];
    ::core::ptr::write_volatile(&mut do_pipe as *mut libc::c_int, 1 as libc::c_int);
    if do_pipe != 0 {
        if pipe2(pipes.as_mut_ptr() as *mut libc::c_int, 0o2000000 as libc::c_int) != 0 {
            ::core::ptr::write_volatile(
                &mut do_pipe as *mut libc::c_int,
                0 as libc::c_int,
            );
        }
    }
    let mut sleep_interval: libc::c_int = 1 as libc::c_int;
    let mut retries: libc::c_int = 0;
    let mut save_environ: *mut *mut libc::c_char = environ;
    ::core::ptr::write_volatile(&mut retries as *mut libc::c_int, 0 as libc::c_int);
    while retries < 4 as libc::c_int {
        pid = vfork();
        if pid >= 0 as libc::c_int {
            break;
        }
        sleep(sleep_interval as libc::c_uint);
        ::core::ptr::write_volatile(
            &mut sleep_interval as *mut libc::c_int,
            ::core::ptr::read_volatile::<
                libc::c_int,
            >(&sleep_interval as *const libc::c_int) * 2 as libc::c_int,
        );
        ::core::ptr::write_volatile(
            &mut retries as *mut libc::c_int,
            ::core::ptr::read_volatile::<libc::c_int>(&retries as *const libc::c_int) + 1,
        );
        ::core::ptr::read_volatile::<libc::c_int>(&retries as *const libc::c_int);
    }
    match pid {
        -1 => {
            if do_pipe != 0 {
                close(pipes[0 as libc::c_int as usize]);
                close(pipes[1 as libc::c_int as usize]);
            }
            *err = *__errno_location();
            *errmsg = b"vfork\0" as *const u8 as *const libc::c_char;
            return -(1 as libc::c_int);
        }
        0 => {
            let mut failed: fn_err = fn_err {
                fn_0: 0 as *const libc::c_char,
                err: 0,
            };
            failed.fn_0 = 0 as *const libc::c_char;
            if do_pipe != 0 {
                close(pipes[0 as libc::c_int as usize]);
            }
            if (failed.fn_0).is_null() && in_0 != 0 as libc::c_int {
                if dup2(in_0, 0 as libc::c_int) < 0 as libc::c_int {
                    failed.fn_0 = b"dup2\0" as *const u8 as *const libc::c_char;
                    failed.err = *__errno_location();
                } else if close(in_0) < 0 as libc::c_int {
                    failed.fn_0 = b"close\0" as *const u8 as *const libc::c_char;
                    failed.err = *__errno_location();
                }
            }
            if (failed.fn_0).is_null() && out != 1 as libc::c_int {
                if dup2(out, 1 as libc::c_int) < 0 as libc::c_int {
                    failed.fn_0 = b"dup2\0" as *const u8 as *const libc::c_char;
                    failed.err = *__errno_location();
                } else if close(out) < 0 as libc::c_int {
                    failed.fn_0 = b"close\0" as *const u8 as *const libc::c_char;
                    failed.err = *__errno_location();
                }
            }
            if (failed.fn_0).is_null() && errdes != 2 as libc::c_int {
                if dup2(errdes, 2 as libc::c_int) < 0 as libc::c_int {
                    failed.fn_0 = b"dup2\0" as *const u8 as *const libc::c_char;
                    failed.err = *__errno_location();
                } else if close(errdes) < 0 as libc::c_int {
                    failed.fn_0 = b"close\0" as *const u8 as *const libc::c_char;
                    failed.err = *__errno_location();
                }
            }
            if (failed.fn_0).is_null() && toclose >= 0 as libc::c_int {
                if close(toclose) < 0 as libc::c_int {
                    failed.fn_0 = b"close\0" as *const u8 as *const libc::c_char;
                    failed.err = *__errno_location();
                }
            }
            if (failed.fn_0).is_null() && flags & 0x8 as libc::c_int != 0 as libc::c_int
            {
                if dup2(1 as libc::c_int, 2 as libc::c_int) < 0 as libc::c_int {
                    failed.fn_0 = b"dup2\0" as *const u8 as *const libc::c_char;
                    failed.err = *__errno_location();
                }
            }
            if (failed.fn_0).is_null() {
                if !env.is_null() {
                    environ = env as *mut *mut libc::c_char;
                }
                if flags & 0x2 as libc::c_int != 0 as libc::c_int {
                    execvp(executable, argv);
                    failed.fn_0 = b"execvp\0" as *const u8 as *const libc::c_char;
                    failed.err = *__errno_location();
                } else {
                    execv(executable, argv);
                    failed.fn_0 = b"execv\0" as *const u8 as *const libc::c_char;
                    failed.err = *__errno_location();
                }
            }
            let mut retval: ssize_t = 0 as libc::c_int as ssize_t;
            if do_pipe == 0
                || write(
                    pipes[1 as libc::c_int as usize],
                    &mut failed as *mut fn_err as *const libc::c_void,
                    ::core::mem::size_of::<fn_err>() as libc::c_ulong,
                ) as libc::c_ulong != ::core::mem::size_of::<fn_err>() as libc::c_ulong
            {
                retval
                    |= write(
                        2 as libc::c_int,
                        (*obj).pname as *const libc::c_void,
                        strlen((*obj).pname),
                    );
                retval
                    |= write(
                        2 as libc::c_int,
                        b": error trying to exec '\0" as *const u8 as *const libc::c_char
                            as *const libc::c_void,
                        strlen(
                            b": error trying to exec '\0" as *const u8
                                as *const libc::c_char,
                        ),
                    );
                retval
                    |= write(
                        2 as libc::c_int,
                        executable as *const libc::c_void,
                        strlen(executable),
                    );
                retval
                    |= write(
                        2 as libc::c_int,
                        b"': \0" as *const u8 as *const libc::c_char
                            as *const libc::c_void,
                        strlen(b"': \0" as *const u8 as *const libc::c_char),
                    );
                retval
                    |= write(
                        2 as libc::c_int,
                        failed.fn_0 as *const libc::c_void,
                        strlen(failed.fn_0),
                    );
                retval
                    |= write(
                        2 as libc::c_int,
                        b": \0" as *const u8 as *const libc::c_char
                            as *const libc::c_void,
                        strlen(b": \0" as *const u8 as *const libc::c_char),
                    );
                retval
                    |= write(
                        2 as libc::c_int,
                        xstrerror(failed.err) as *const libc::c_void,
                        strlen(xstrerror(failed.err)),
                    );
                retval
                    |= write(
                        2 as libc::c_int,
                        b"\n\0" as *const u8 as *const libc::c_char
                            as *const libc::c_void,
                        strlen(b"\n\0" as *const u8 as *const libc::c_char),
                    );
            }
            _exit(
                if retval < 0 as libc::c_int as libc::c_long {
                    -(2 as libc::c_int)
                } else {
                    -(1 as libc::c_int)
                },
            );
        }
        _ => {
            environ = save_environ;
            let mut failed_0: fn_err = fn_err {
                fn_0: 0 as *const libc::c_char,
                err: 0,
            };
            failed_0.fn_0 = 0 as *const libc::c_char;
            if do_pipe != 0 {
                close(pipes[1 as libc::c_int as usize]);
                let mut len: ssize_t = read(
                    pipes[0 as libc::c_int as usize],
                    &mut failed_0 as *mut fn_err as *mut libc::c_void,
                    ::core::mem::size_of::<fn_err>() as libc::c_ulong,
                );
                if len < 0 as libc::c_int as libc::c_long {
                    failed_0.fn_0 = 0 as *const libc::c_char;
                }
                close(pipes[0 as libc::c_int as usize]);
            }
            if (failed_0.fn_0).is_null() && in_0 != 0 as libc::c_int {
                if close(in_0) < 0 as libc::c_int {
                    failed_0.fn_0 = b"close\0" as *const u8 as *const libc::c_char;
                    failed_0.err = *__errno_location();
                }
            }
            if (failed_0.fn_0).is_null() && out != 1 as libc::c_int {
                if close(out) < 0 as libc::c_int {
                    failed_0.fn_0 = b"close\0" as *const u8 as *const libc::c_char;
                    failed_0.err = *__errno_location();
                }
            }
            if (failed_0.fn_0).is_null() && errdes != 2 as libc::c_int {
                if close(errdes) < 0 as libc::c_int {
                    failed_0.fn_0 = b"close\0" as *const u8 as *const libc::c_char;
                    failed_0.err = *__errno_location();
                }
            }
            if !(failed_0.fn_0).is_null() {
                *err = failed_0.err;
                *errmsg = failed_0.fn_0;
                return -(1 as libc::c_int);
            }
            return pid;
        }
    };
}
unsafe extern "C" fn pex_unix_wait(
    mut obj: *mut pex_obj,
    mut pid: pid_t,
    mut status: *mut libc::c_int,
    mut time: *mut pex_time,
    mut done: libc::c_int,
    mut errmsg: *mut *const libc::c_char,
    mut err: *mut libc::c_int,
) -> libc::c_int {
    if done != 0 {
        kill(pid, 15 as libc::c_int);
    }
    if pex_wait(obj, pid, status, time) < 0 as libc::c_int {
        *err = *__errno_location();
        *errmsg = b"wait\0" as *const u8 as *const libc::c_char;
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn pex_unix_pipe(
    mut obj: *mut pex_obj,
    mut p: *mut libc::c_int,
    mut binary: libc::c_int,
) -> libc::c_int {
    return pipe(p);
}
unsafe extern "C" fn pex_unix_fdopenr(
    mut obj: *mut pex_obj,
    mut fd: libc::c_int,
    mut binary: libc::c_int,
) -> *mut FILE {
    return fdopen(fd, b"r\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn pex_unix_fdopenw(
    mut obj: *mut pex_obj,
    mut fd: libc::c_int,
    mut binary: libc::c_int,
) -> *mut FILE {
    if fcntl(fd, 2 as libc::c_int, 1 as libc::c_int) < 0 as libc::c_int {
        return 0 as *mut FILE;
    }
    return fdopen(fd, b"w\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn pex_unix_cleanup(mut obj: *mut pex_obj) {}
