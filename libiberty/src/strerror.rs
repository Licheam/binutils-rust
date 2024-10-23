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
pub struct error_info {
    pub value: libc::c_int,
    pub name: *const libc::c_char,
    pub msg: *const libc::c_char,
}
static mut error_table: [error_info; 122] = [
    {
        let mut init = error_info {
            value: 1 as libc::c_int,
            name: b"EPERM\0" as *const u8 as *const libc::c_char,
            msg: b"Not owner\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 2 as libc::c_int,
            name: b"ENOENT\0" as *const u8 as *const libc::c_char,
            msg: b"No such file or directory\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 3 as libc::c_int,
            name: b"ESRCH\0" as *const u8 as *const libc::c_char,
            msg: b"No such process\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 4 as libc::c_int,
            name: b"EINTR\0" as *const u8 as *const libc::c_char,
            msg: b"Interrupted system call\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 5 as libc::c_int,
            name: b"EIO\0" as *const u8 as *const libc::c_char,
            msg: b"I/O error\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 6 as libc::c_int,
            name: b"ENXIO\0" as *const u8 as *const libc::c_char,
            msg: b"No such device or address\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 7 as libc::c_int,
            name: b"E2BIG\0" as *const u8 as *const libc::c_char,
            msg: b"Arg list too long\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 8 as libc::c_int,
            name: b"ENOEXEC\0" as *const u8 as *const libc::c_char,
            msg: b"Exec format error\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 9 as libc::c_int,
            name: b"EBADF\0" as *const u8 as *const libc::c_char,
            msg: b"Bad file number\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 10 as libc::c_int,
            name: b"ECHILD\0" as *const u8 as *const libc::c_char,
            msg: b"No child processes\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 11 as libc::c_int,
            name: b"EWOULDBLOCK\0" as *const u8 as *const libc::c_char,
            msg: b"Operation would block\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 11 as libc::c_int,
            name: b"EAGAIN\0" as *const u8 as *const libc::c_char,
            msg: b"No more processes\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 12 as libc::c_int,
            name: b"ENOMEM\0" as *const u8 as *const libc::c_char,
            msg: b"Not enough space\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 13 as libc::c_int,
            name: b"EACCES\0" as *const u8 as *const libc::c_char,
            msg: b"Permission denied\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 14 as libc::c_int,
            name: b"EFAULT\0" as *const u8 as *const libc::c_char,
            msg: b"Bad address\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 15 as libc::c_int,
            name: b"ENOTBLK\0" as *const u8 as *const libc::c_char,
            msg: b"Block device required\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 16 as libc::c_int,
            name: b"EBUSY\0" as *const u8 as *const libc::c_char,
            msg: b"Device busy\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 17 as libc::c_int,
            name: b"EEXIST\0" as *const u8 as *const libc::c_char,
            msg: b"File exists\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 18 as libc::c_int,
            name: b"EXDEV\0" as *const u8 as *const libc::c_char,
            msg: b"Cross-device link\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 19 as libc::c_int,
            name: b"ENODEV\0" as *const u8 as *const libc::c_char,
            msg: b"No such device\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 20 as libc::c_int,
            name: b"ENOTDIR\0" as *const u8 as *const libc::c_char,
            msg: b"Not a directory\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 21 as libc::c_int,
            name: b"EISDIR\0" as *const u8 as *const libc::c_char,
            msg: b"Is a directory\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 22 as libc::c_int,
            name: b"EINVAL\0" as *const u8 as *const libc::c_char,
            msg: b"Invalid argument\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 23 as libc::c_int,
            name: b"ENFILE\0" as *const u8 as *const libc::c_char,
            msg: b"File table overflow\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 24 as libc::c_int,
            name: b"EMFILE\0" as *const u8 as *const libc::c_char,
            msg: b"Too many open files\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 25 as libc::c_int,
            name: b"ENOTTY\0" as *const u8 as *const libc::c_char,
            msg: b"Not a typewriter\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 26 as libc::c_int,
            name: b"ETXTBSY\0" as *const u8 as *const libc::c_char,
            msg: b"Text file busy\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 27 as libc::c_int,
            name: b"EFBIG\0" as *const u8 as *const libc::c_char,
            msg: b"File too large\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 28 as libc::c_int,
            name: b"ENOSPC\0" as *const u8 as *const libc::c_char,
            msg: b"No space left on device\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 29 as libc::c_int,
            name: b"ESPIPE\0" as *const u8 as *const libc::c_char,
            msg: b"Illegal seek\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 30 as libc::c_int,
            name: b"EROFS\0" as *const u8 as *const libc::c_char,
            msg: b"Read-only file system\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 31 as libc::c_int,
            name: b"EMLINK\0" as *const u8 as *const libc::c_char,
            msg: b"Too many links\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 32 as libc::c_int,
            name: b"EPIPE\0" as *const u8 as *const libc::c_char,
            msg: b"Broken pipe\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 33 as libc::c_int,
            name: b"EDOM\0" as *const u8 as *const libc::c_char,
            msg: b"Math argument out of domain of func\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 34 as libc::c_int,
            name: b"ERANGE\0" as *const u8 as *const libc::c_char,
            msg: b"Math result not representable\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 42 as libc::c_int,
            name: b"ENOMSG\0" as *const u8 as *const libc::c_char,
            msg: b"No message of desired type\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 43 as libc::c_int,
            name: b"EIDRM\0" as *const u8 as *const libc::c_char,
            msg: b"Identifier removed\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 44 as libc::c_int,
            name: b"ECHRNG\0" as *const u8 as *const libc::c_char,
            msg: b"Channel number out of range\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 45 as libc::c_int,
            name: b"EL2NSYNC\0" as *const u8 as *const libc::c_char,
            msg: b"Level 2 not synchronized\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 46 as libc::c_int,
            name: b"EL3HLT\0" as *const u8 as *const libc::c_char,
            msg: b"Level 3 halted\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 47 as libc::c_int,
            name: b"EL3RST\0" as *const u8 as *const libc::c_char,
            msg: b"Level 3 reset\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 48 as libc::c_int,
            name: b"ELNRNG\0" as *const u8 as *const libc::c_char,
            msg: b"Link number out of range\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 49 as libc::c_int,
            name: b"EUNATCH\0" as *const u8 as *const libc::c_char,
            msg: b"Protocol driver not attached\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 50 as libc::c_int,
            name: b"ENOCSI\0" as *const u8 as *const libc::c_char,
            msg: b"No CSI structure available\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 51 as libc::c_int,
            name: b"EL2HLT\0" as *const u8 as *const libc::c_char,
            msg: b"Level 2 halted\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 35 as libc::c_int,
            name: b"EDEADLK\0" as *const u8 as *const libc::c_char,
            msg: b"Deadlock condition\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 37 as libc::c_int,
            name: b"ENOLCK\0" as *const u8 as *const libc::c_char,
            msg: b"No record locks available\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 52 as libc::c_int,
            name: b"EBADE\0" as *const u8 as *const libc::c_char,
            msg: b"Invalid exchange\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 53 as libc::c_int,
            name: b"EBADR\0" as *const u8 as *const libc::c_char,
            msg: b"Invalid request descriptor\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 54 as libc::c_int,
            name: b"EXFULL\0" as *const u8 as *const libc::c_char,
            msg: b"Exchange full\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 55 as libc::c_int,
            name: b"ENOANO\0" as *const u8 as *const libc::c_char,
            msg: b"No anode\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 56 as libc::c_int,
            name: b"EBADRQC\0" as *const u8 as *const libc::c_char,
            msg: b"Invalid request code\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 57 as libc::c_int,
            name: b"EBADSLT\0" as *const u8 as *const libc::c_char,
            msg: b"Invalid slot\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 35 as libc::c_int,
            name: b"EDEADLOCK\0" as *const u8 as *const libc::c_char,
            msg: b"File locking deadlock error\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 59 as libc::c_int,
            name: b"EBFONT\0" as *const u8 as *const libc::c_char,
            msg: b"Bad font file format\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 60 as libc::c_int,
            name: b"ENOSTR\0" as *const u8 as *const libc::c_char,
            msg: b"Device not a stream\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 61 as libc::c_int,
            name: b"ENODATA\0" as *const u8 as *const libc::c_char,
            msg: b"No data available\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 62 as libc::c_int,
            name: b"ETIME\0" as *const u8 as *const libc::c_char,
            msg: b"Timer expired\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 63 as libc::c_int,
            name: b"ENOSR\0" as *const u8 as *const libc::c_char,
            msg: b"Out of streams resources\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 64 as libc::c_int,
            name: b"ENONET\0" as *const u8 as *const libc::c_char,
            msg: b"Machine is not on the network\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 65 as libc::c_int,
            name: b"ENOPKG\0" as *const u8 as *const libc::c_char,
            msg: b"Package not installed\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 66 as libc::c_int,
            name: b"EREMOTE\0" as *const u8 as *const libc::c_char,
            msg: b"Object is remote\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 67 as libc::c_int,
            name: b"ENOLINK\0" as *const u8 as *const libc::c_char,
            msg: b"Link has been severed\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 68 as libc::c_int,
            name: b"EADV\0" as *const u8 as *const libc::c_char,
            msg: b"Advertise error\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 69 as libc::c_int,
            name: b"ESRMNT\0" as *const u8 as *const libc::c_char,
            msg: b"Srmount error\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 70 as libc::c_int,
            name: b"ECOMM\0" as *const u8 as *const libc::c_char,
            msg: b"Communication error on send\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 71 as libc::c_int,
            name: b"EPROTO\0" as *const u8 as *const libc::c_char,
            msg: b"Protocol error\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 72 as libc::c_int,
            name: b"EMULTIHOP\0" as *const u8 as *const libc::c_char,
            msg: b"Multihop attempted\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 73 as libc::c_int,
            name: b"EDOTDOT\0" as *const u8 as *const libc::c_char,
            msg: b"RFS specific error\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 74 as libc::c_int,
            name: b"EBADMSG\0" as *const u8 as *const libc::c_char,
            msg: b"Not a data message\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 36 as libc::c_int,
            name: b"ENAMETOOLONG\0" as *const u8 as *const libc::c_char,
            msg: b"File name too long\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 75 as libc::c_int,
            name: b"EOVERFLOW\0" as *const u8 as *const libc::c_char,
            msg: b"Value too large for defined data type\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 76 as libc::c_int,
            name: b"ENOTUNIQ\0" as *const u8 as *const libc::c_char,
            msg: b"Name not unique on network\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 77 as libc::c_int,
            name: b"EBADFD\0" as *const u8 as *const libc::c_char,
            msg: b"File descriptor in bad state\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 78 as libc::c_int,
            name: b"EREMCHG\0" as *const u8 as *const libc::c_char,
            msg: b"Remote address changed\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 79 as libc::c_int,
            name: b"ELIBACC\0" as *const u8 as *const libc::c_char,
            msg: b"Cannot access a needed shared library\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 80 as libc::c_int,
            name: b"ELIBBAD\0" as *const u8 as *const libc::c_char,
            msg: b"Accessing a corrupted shared library\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 81 as libc::c_int,
            name: b"ELIBSCN\0" as *const u8 as *const libc::c_char,
            msg: b".lib section in a.out corrupted\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 82 as libc::c_int,
            name: b"ELIBMAX\0" as *const u8 as *const libc::c_char,
            msg: b"Attempting to link in too many shared libraries\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 83 as libc::c_int,
            name: b"ELIBEXEC\0" as *const u8 as *const libc::c_char,
            msg: b"Cannot exec a shared library directly\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 84 as libc::c_int,
            name: b"EILSEQ\0" as *const u8 as *const libc::c_char,
            msg: b"Illegal byte sequence\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 38 as libc::c_int,
            name: b"ENOSYS\0" as *const u8 as *const libc::c_char,
            msg: b"Operation not applicable\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 40 as libc::c_int,
            name: b"ELOOP\0" as *const u8 as *const libc::c_char,
            msg: b"Too many symbolic links encountered\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 85 as libc::c_int,
            name: b"ERESTART\0" as *const u8 as *const libc::c_char,
            msg: b"Interrupted system call should be restarted\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 86 as libc::c_int,
            name: b"ESTRPIPE\0" as *const u8 as *const libc::c_char,
            msg: b"Streams pipe error\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 39 as libc::c_int,
            name: b"ENOTEMPTY\0" as *const u8 as *const libc::c_char,
            msg: b"Directory not empty\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 87 as libc::c_int,
            name: b"EUSERS\0" as *const u8 as *const libc::c_char,
            msg: b"Too many users\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 88 as libc::c_int,
            name: b"ENOTSOCK\0" as *const u8 as *const libc::c_char,
            msg: b"Socket operation on non-socket\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 89 as libc::c_int,
            name: b"EDESTADDRREQ\0" as *const u8 as *const libc::c_char,
            msg: b"Destination address required\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 90 as libc::c_int,
            name: b"EMSGSIZE\0" as *const u8 as *const libc::c_char,
            msg: b"Message too long\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 91 as libc::c_int,
            name: b"EPROTOTYPE\0" as *const u8 as *const libc::c_char,
            msg: b"Protocol wrong type for socket\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 92 as libc::c_int,
            name: b"ENOPROTOOPT\0" as *const u8 as *const libc::c_char,
            msg: b"Protocol not available\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 93 as libc::c_int,
            name: b"EPROTONOSUPPORT\0" as *const u8 as *const libc::c_char,
            msg: b"Protocol not supported\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 94 as libc::c_int,
            name: b"ESOCKTNOSUPPORT\0" as *const u8 as *const libc::c_char,
            msg: b"Socket type not supported\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 95 as libc::c_int,
            name: b"EOPNOTSUPP\0" as *const u8 as *const libc::c_char,
            msg: b"Operation not supported on transport endpoint\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 96 as libc::c_int,
            name: b"EPFNOSUPPORT\0" as *const u8 as *const libc::c_char,
            msg: b"Protocol family not supported\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 97 as libc::c_int,
            name: b"EAFNOSUPPORT\0" as *const u8 as *const libc::c_char,
            msg: b"Address family not supported by protocol\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 98 as libc::c_int,
            name: b"EADDRINUSE\0" as *const u8 as *const libc::c_char,
            msg: b"Address already in use\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 99 as libc::c_int,
            name: b"EADDRNOTAVAIL\0" as *const u8 as *const libc::c_char,
            msg: b"Cannot assign requested address\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 100 as libc::c_int,
            name: b"ENETDOWN\0" as *const u8 as *const libc::c_char,
            msg: b"Network is down\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 101 as libc::c_int,
            name: b"ENETUNREACH\0" as *const u8 as *const libc::c_char,
            msg: b"Network is unreachable\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 102 as libc::c_int,
            name: b"ENETRESET\0" as *const u8 as *const libc::c_char,
            msg: b"Network dropped connection because of reset\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 103 as libc::c_int,
            name: b"ECONNABORTED\0" as *const u8 as *const libc::c_char,
            msg: b"Software caused connection abort\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 104 as libc::c_int,
            name: b"ECONNRESET\0" as *const u8 as *const libc::c_char,
            msg: b"Connection reset by peer\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 105 as libc::c_int,
            name: b"ENOBUFS\0" as *const u8 as *const libc::c_char,
            msg: b"No buffer space available\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 106 as libc::c_int,
            name: b"EISCONN\0" as *const u8 as *const libc::c_char,
            msg: b"Transport endpoint is already connected\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 107 as libc::c_int,
            name: b"ENOTCONN\0" as *const u8 as *const libc::c_char,
            msg: b"Transport endpoint is not connected\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 108 as libc::c_int,
            name: b"ESHUTDOWN\0" as *const u8 as *const libc::c_char,
            msg: b"Cannot send after transport endpoint shutdown\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 109 as libc::c_int,
            name: b"ETOOMANYREFS\0" as *const u8 as *const libc::c_char,
            msg: b"Too many references: cannot splice\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 110 as libc::c_int,
            name: b"ETIMEDOUT\0" as *const u8 as *const libc::c_char,
            msg: b"Connection timed out\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 111 as libc::c_int,
            name: b"ECONNREFUSED\0" as *const u8 as *const libc::c_char,
            msg: b"Connection refused\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 112 as libc::c_int,
            name: b"EHOSTDOWN\0" as *const u8 as *const libc::c_char,
            msg: b"Host is down\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 113 as libc::c_int,
            name: b"EHOSTUNREACH\0" as *const u8 as *const libc::c_char,
            msg: b"No route to host\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 114 as libc::c_int,
            name: b"EALREADY\0" as *const u8 as *const libc::c_char,
            msg: b"Operation already in progress\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 115 as libc::c_int,
            name: b"EINPROGRESS\0" as *const u8 as *const libc::c_char,
            msg: b"Operation now in progress\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 116 as libc::c_int,
            name: b"ESTALE\0" as *const u8 as *const libc::c_char,
            msg: b"Stale NFS file handle\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 117 as libc::c_int,
            name: b"EUCLEAN\0" as *const u8 as *const libc::c_char,
            msg: b"Structure needs cleaning\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 118 as libc::c_int,
            name: b"ENOTNAM\0" as *const u8 as *const libc::c_char,
            msg: b"Not a XENIX named type file\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 119 as libc::c_int,
            name: b"ENAVAIL\0" as *const u8 as *const libc::c_char,
            msg: b"No XENIX semaphores available\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 120 as libc::c_int,
            name: b"EISNAM\0" as *const u8 as *const libc::c_char,
            msg: b"Is a named type file\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 121 as libc::c_int,
            name: b"EREMOTEIO\0" as *const u8 as *const libc::c_char,
            msg: b"Remote I/O error\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = error_info {
            value: 0 as libc::c_int,
            name: 0 as *const libc::c_char,
            msg: 0 as *const libc::c_char,
        };
        init
    },
];
static mut error_names: *mut *const libc::c_char = 0 as *const *const libc::c_char
    as *mut *const libc::c_char;
static mut num_error_names: libc::c_int = 0 as libc::c_int;
static mut sys_nerr__: libc::c_int = 0;
static mut sys_errlist__: *mut *const libc::c_char = 0 as *const *const libc::c_char
    as *mut *const libc::c_char;
unsafe extern "C" fn init_error_tables() {
    let mut eip: *const error_info = 0 as *const error_info;
    let mut nbytes: libc::c_int = 0;
    if num_error_names == 0 as libc::c_int {
        eip = error_table.as_ptr();
        while !((*eip).name).is_null() {
            if (*eip).value >= num_error_names {
                num_error_names = (*eip).value + 1 as libc::c_int;
            }
            eip = eip.offset(1);
            eip;
        }
    }
    if error_names.is_null() {
        nbytes = (num_error_names as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
            as libc::c_int;
        error_names = malloc(nbytes as libc::c_ulong) as *mut *const libc::c_char;
        if !error_names.is_null() {
            memset(
                error_names as *mut libc::c_void,
                0 as libc::c_int,
                nbytes as libc::c_ulong,
            );
            eip = error_table.as_ptr();
            while !((*eip).name).is_null() {
                let ref mut fresh0 = *error_names.offset((*eip).value as isize);
                *fresh0 = (*eip).name;
                eip = eip.offset(1);
                eip;
            }
        }
    }
    if sys_errlist__.is_null() {
        nbytes = (num_error_names as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
            as libc::c_int;
        sys_errlist__ = malloc(nbytes as libc::c_ulong) as *mut *const libc::c_char;
        if !sys_errlist__.is_null() {
            memset(
                sys_errlist__ as *mut libc::c_void,
                0 as libc::c_int,
                nbytes as libc::c_ulong,
            );
            sys_nerr__ = num_error_names;
            eip = error_table.as_ptr();
            while !((*eip).name).is_null() {
                let ref mut fresh1 = *sys_errlist__.offset((*eip).value as isize);
                *fresh1 = (*eip).msg;
                eip = eip.offset(1);
                eip;
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn errno_max() -> libc::c_int {
    let mut maxsize: libc::c_int = 0;
    if error_names.is_null() {
        init_error_tables();
    }
    maxsize = if sys_nerr__ > num_error_names { sys_nerr__ } else { num_error_names };
    return maxsize - 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn strerrno(mut errnoval: libc::c_int) -> *const libc::c_char {
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    static mut buf: [libc::c_char; 32] = [0; 32];
    if error_names.is_null() {
        init_error_tables();
    }
    if errnoval < 0 as libc::c_int || errnoval >= num_error_names {
        name = 0 as *const libc::c_char;
    } else if error_names.is_null() || (*error_names.offset(errnoval as isize)).is_null()
    {
        sprintf(
            buf.as_mut_ptr(),
            b"Error %d\0" as *const u8 as *const libc::c_char,
            errnoval,
        );
        name = buf.as_mut_ptr() as *const libc::c_char;
    } else {
        name = *error_names.offset(errnoval as isize);
    }
    return name;
}
#[no_mangle]
pub unsafe extern "C" fn strtoerrno(mut name: *const libc::c_char) -> libc::c_int {
    let mut errnoval: libc::c_int = 0 as libc::c_int;
    if !name.is_null() {
        if error_names.is_null() {
            init_error_tables();
        }
        errnoval = 0 as libc::c_int;
        while errnoval < num_error_names {
            if !(*error_names.offset(errnoval as isize)).is_null()
                && strcmp(name, *error_names.offset(errnoval as isize))
                    == 0 as libc::c_int
            {
                break;
            }
            errnoval += 1;
            errnoval;
        }
        if errnoval == num_error_names {
            errnoval = 0 as libc::c_int;
        }
    }
    return errnoval;
}
