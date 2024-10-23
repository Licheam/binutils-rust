use ::libc;
extern "C" {
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct signal_info {
    pub value: libc::c_int,
    pub name: *const libc::c_char,
    pub msg: *const libc::c_char,
}
static mut signal_table: [signal_info; 34] = [
    {
        let mut init = signal_info {
            value: 1 as libc::c_int,
            name: b"SIGHUP\0" as *const u8 as *const libc::c_char,
            msg: b"Hangup\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = signal_info {
            value: 2 as libc::c_int,
            name: b"SIGINT\0" as *const u8 as *const libc::c_char,
            msg: b"Interrupt\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = signal_info {
            value: 3 as libc::c_int,
            name: b"SIGQUIT\0" as *const u8 as *const libc::c_char,
            msg: b"Quit\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = signal_info {
            value: 4 as libc::c_int,
            name: b"SIGILL\0" as *const u8 as *const libc::c_char,
            msg: b"Illegal instruction\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = signal_info {
            value: 5 as libc::c_int,
            name: b"SIGTRAP\0" as *const u8 as *const libc::c_char,
            msg: b"Trace/breakpoint trap\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = signal_info {
            value: 6 as libc::c_int,
            name: b"SIGIOT\0" as *const u8 as *const libc::c_char,
            msg: b"IOT trap\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = signal_info {
            value: 6 as libc::c_int,
            name: b"SIGABRT\0" as *const u8 as *const libc::c_char,
            msg: b"Aborted\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = signal_info {
            value: 8 as libc::c_int,
            name: b"SIGFPE\0" as *const u8 as *const libc::c_char,
            msg: b"Arithmetic exception\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = signal_info {
            value: 9 as libc::c_int,
            name: b"SIGKILL\0" as *const u8 as *const libc::c_char,
            msg: b"Killed\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = signal_info {
            value: 7 as libc::c_int,
            name: b"SIGBUS\0" as *const u8 as *const libc::c_char,
            msg: b"Bus error\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = signal_info {
            value: 11 as libc::c_int,
            name: b"SIGSEGV\0" as *const u8 as *const libc::c_char,
            msg: b"Segmentation fault\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = signal_info {
            value: 31 as libc::c_int,
            name: b"SIGSYS\0" as *const u8 as *const libc::c_char,
            msg: b"Bad system call\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = signal_info {
            value: 13 as libc::c_int,
            name: b"SIGPIPE\0" as *const u8 as *const libc::c_char,
            msg: b"Broken pipe\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = signal_info {
            value: 14 as libc::c_int,
            name: b"SIGALRM\0" as *const u8 as *const libc::c_char,
            msg: b"Alarm clock\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = signal_info {
            value: 15 as libc::c_int,
            name: b"SIGTERM\0" as *const u8 as *const libc::c_char,
            msg: b"Terminated\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = signal_info {
            value: 10 as libc::c_int,
            name: b"SIGUSR1\0" as *const u8 as *const libc::c_char,
            msg: b"User defined signal 1\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = signal_info {
            value: 12 as libc::c_int,
            name: b"SIGUSR2\0" as *const u8 as *const libc::c_char,
            msg: b"User defined signal 2\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = signal_info {
            value: 17 as libc::c_int,
            name: b"SIGCLD\0" as *const u8 as *const libc::c_char,
            msg: b"Child status changed\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = signal_info {
            value: 17 as libc::c_int,
            name: b"SIGCHLD\0" as *const u8 as *const libc::c_char,
            msg: b"Child status changed\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = signal_info {
            value: 30 as libc::c_int,
            name: b"SIGPWR\0" as *const u8 as *const libc::c_char,
            msg: b"Power fail/restart\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = signal_info {
            value: 28 as libc::c_int,
            name: b"SIGWINCH\0" as *const u8 as *const libc::c_char,
            msg: b"Window size changed\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = signal_info {
            value: 23 as libc::c_int,
            name: b"SIGURG\0" as *const u8 as *const libc::c_char,
            msg: b"Urgent I/O condition\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = signal_info {
            value: 29 as libc::c_int,
            name: b"SIGIO\0" as *const u8 as *const libc::c_char,
            msg: b"I/O possible\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = signal_info {
            value: 29 as libc::c_int,
            name: b"SIGPOLL\0" as *const u8 as *const libc::c_char,
            msg: b"Pollable event occurred\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = signal_info {
            value: 19 as libc::c_int,
            name: b"SIGSTOP\0" as *const u8 as *const libc::c_char,
            msg: b"Stopped (signal)\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = signal_info {
            value: 20 as libc::c_int,
            name: b"SIGTSTP\0" as *const u8 as *const libc::c_char,
            msg: b"Stopped (user)\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = signal_info {
            value: 18 as libc::c_int,
            name: b"SIGCONT\0" as *const u8 as *const libc::c_char,
            msg: b"Continued\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = signal_info {
            value: 21 as libc::c_int,
            name: b"SIGTTIN\0" as *const u8 as *const libc::c_char,
            msg: b"Stopped (tty input)\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = signal_info {
            value: 22 as libc::c_int,
            name: b"SIGTTOU\0" as *const u8 as *const libc::c_char,
            msg: b"Stopped (tty output)\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = signal_info {
            value: 26 as libc::c_int,
            name: b"SIGVTALRM\0" as *const u8 as *const libc::c_char,
            msg: b"Virtual timer expired\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = signal_info {
            value: 27 as libc::c_int,
            name: b"SIGPROF\0" as *const u8 as *const libc::c_char,
            msg: b"Profiling timer expired\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = signal_info {
            value: 24 as libc::c_int,
            name: b"SIGXCPU\0" as *const u8 as *const libc::c_char,
            msg: b"CPU time limit exceeded\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = signal_info {
            value: 25 as libc::c_int,
            name: b"SIGXFSZ\0" as *const u8 as *const libc::c_char,
            msg: b"File size limit exceeded\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = signal_info {
            value: 0 as libc::c_int,
            name: 0 as *const libc::c_char,
            msg: 0 as *const libc::c_char,
        };
        init
    },
];
static mut signal_names: *mut *const libc::c_char = 0 as *const *const libc::c_char
    as *mut *const libc::c_char;
static mut num_signal_names: libc::c_int = 0 as libc::c_int;
static mut sys_nsig: libc::c_int = 0;
static mut sys_siglist: *mut *const libc::c_char = 0 as *const *const libc::c_char
    as *mut *const libc::c_char;
unsafe extern "C" fn init_signal_tables() {
    let mut eip: *const signal_info = 0 as *const signal_info;
    let mut nbytes: libc::c_int = 0;
    if num_signal_names == 0 as libc::c_int {
        eip = signal_table.as_ptr();
        while !((*eip).name).is_null() {
            if (*eip).value >= num_signal_names {
                num_signal_names = (*eip).value + 1 as libc::c_int;
            }
            eip = eip.offset(1);
            eip;
        }
    }
    if signal_names.is_null() {
        nbytes = (num_signal_names as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
            as libc::c_int;
        signal_names = malloc(nbytes as libc::c_ulong) as *mut *const libc::c_char;
        if !signal_names.is_null() {
            memset(
                signal_names as *mut libc::c_void,
                0 as libc::c_int,
                nbytes as libc::c_ulong,
            );
            eip = signal_table.as_ptr();
            while !((*eip).name).is_null() {
                let ref mut fresh0 = *signal_names.offset((*eip).value as isize);
                *fresh0 = (*eip).name;
                eip = eip.offset(1);
                eip;
            }
        }
    }
    if sys_siglist.is_null() {
        nbytes = (num_signal_names as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
            as libc::c_int;
        sys_siglist = malloc(nbytes as libc::c_ulong) as *mut *const libc::c_char;
        if !sys_siglist.is_null() {
            memset(
                sys_siglist as *mut libc::c_void,
                0 as libc::c_int,
                nbytes as libc::c_ulong,
            );
            sys_nsig = num_signal_names;
            eip = signal_table.as_ptr();
            while !((*eip).name).is_null() {
                let ref mut fresh1 = *sys_siglist.offset((*eip).value as isize);
                *fresh1 = (*eip).msg;
                eip = eip.offset(1);
                eip;
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn signo_max() -> libc::c_int {
    let mut maxsize: libc::c_int = 0;
    if signal_names.is_null() {
        init_signal_tables();
    }
    maxsize = if sys_nsig > num_signal_names { sys_nsig } else { num_signal_names };
    return maxsize - 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn strsigno(mut signo: libc::c_int) -> *const libc::c_char {
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    static mut buf: [libc::c_char; 32] = [0; 32];
    if signal_names.is_null() {
        init_signal_tables();
    }
    if signo < 0 as libc::c_int || signo >= num_signal_names {
        name = 0 as *const libc::c_char;
    } else if signal_names.is_null() || (*signal_names.offset(signo as isize)).is_null()
    {
        sprintf(
            buf.as_mut_ptr(),
            b"Signal %d\0" as *const u8 as *const libc::c_char,
            signo,
        );
        name = buf.as_mut_ptr() as *const libc::c_char;
    } else {
        name = *signal_names.offset(signo as isize);
    }
    return name;
}
#[no_mangle]
pub unsafe extern "C" fn strtosigno(mut name: *const libc::c_char) -> libc::c_int {
    let mut signo: libc::c_int = 0 as libc::c_int;
    if !name.is_null() {
        if signal_names.is_null() {
            init_signal_tables();
        }
        signo = 0 as libc::c_int;
        while signo < num_signal_names {
            if !(*signal_names.offset(signo as isize)).is_null()
                && strcmp(name, *signal_names.offset(signo as isize)) == 0 as libc::c_int
            {
                break;
            }
            signo += 1;
            signo;
        }
        if signo == num_signal_names {
            signo = 0 as libc::c_int;
        }
    }
    return signo;
}
