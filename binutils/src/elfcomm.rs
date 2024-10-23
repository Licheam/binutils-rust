extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn vfprintf(
        _: *mut FILE,
        _: *const libc::c_char,
        _: ::core::ffi::VaList,
    ) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn getc(__stream: *mut FILE) -> libc::c_int;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fseek(
        __stream: *mut FILE,
        __off: libc::c_long,
        __whence: libc::c_int,
    ) -> libc::c_int;
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
    fn strtoul(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_ulong;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn abort() -> !;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn lbasename(_: *const libc::c_char) -> *const libc::c_char;
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    static mut program_name: *mut libc::c_char;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type size_t = libc::c_ulong;
pub type va_list = __builtin_va_list;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
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
pub type off_t = __off_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ar_hdr {
    pub ar_name: [libc::c_char; 16],
    pub ar_date: [libc::c_char; 12],
    pub ar_uid: [libc::c_char; 6],
    pub ar_gid: [libc::c_char; 6],
    pub ar_mode: [libc::c_char; 8],
    pub ar_size: [libc::c_char; 10],
    pub ar_fmag: [libc::c_char; 2],
}
pub type elf_vma = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct archive_info {
    pub file_name: *mut libc::c_char,
    pub file: *mut FILE,
    pub index_num: elf_vma,
    pub index_array: *mut elf_vma,
    pub sym_table: *mut libc::c_char,
    pub sym_size: libc::c_ulong,
    pub longnames: *mut libc::c_char,
    pub longnames_size: libc::c_ulong,
    pub nested_member_origin: libc::c_ulong,
    pub next_arhdr_offset: libc::c_ulong,
    pub is_thin_archive: libc::c_int,
    pub uses_64bit_indices: libc::c_int,
    pub arhdr: ar_hdr,
}
#[inline]
unsafe extern "C" fn startswith(
    mut str: *const libc::c_char,
    mut prefix: *const libc::c_char,
) -> bool {
    return strncmp(str, prefix, strlen(prefix)) == 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn error(mut message: *const libc::c_char, mut args: ...) {
    let mut args_0: ::core::ffi::VaListImpl;
    fflush(stdout);
    args_0 = args.clone();
    fprintf(
        stderr,
        dcgettext(
            0 as *const libc::c_char,
            b"%s: Error: \0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        program_name,
    );
    vfprintf(stderr, message, args_0.as_va_list());
}
#[no_mangle]
pub unsafe extern "C" fn warn(mut message: *const libc::c_char, mut args: ...) {
    let mut args_0: ::core::ffi::VaListImpl;
    fflush(stdout);
    args_0 = args.clone();
    fprintf(
        stderr,
        dcgettext(
            0 as *const libc::c_char,
            b"%s: Warning: \0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        program_name,
    );
    vfprintf(stderr, message, args_0.as_va_list());
}
#[no_mangle]
pub static mut byte_put: Option::<
    unsafe extern "C" fn(*mut libc::c_uchar, elf_vma, libc::c_uint) -> (),
> = None;
#[no_mangle]
pub unsafe extern "C" fn byte_put_little_endian(
    mut field: *mut libc::c_uchar,
    mut value: elf_vma,
    mut size: libc::c_uint,
) {
    if size as libc::c_ulong > ::core::mem::size_of::<elf_vma>() as libc::c_ulong {
        error(
            dcgettext(
                0 as *const libc::c_char,
                b"Unhandled data length: %d\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            size,
        );
        abort();
    }
    loop {
        let fresh0 = size;
        size = size.wrapping_sub(1);
        if !(fresh0 != 0) {
            break;
        }
        let fresh1 = field;
        field = field.offset(1);
        *fresh1 = (value & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
        value >>= 8 as libc::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn byte_put_big_endian(
    mut field: *mut libc::c_uchar,
    mut value: elf_vma,
    mut size: libc::c_uint,
) {
    if size as libc::c_ulong > ::core::mem::size_of::<elf_vma>() as libc::c_ulong {
        error(
            dcgettext(
                0 as *const libc::c_char,
                b"Unhandled data length: %d\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            size,
        );
        abort();
    }
    loop {
        let fresh2 = size;
        size = size.wrapping_sub(1);
        if !(fresh2 != 0) {
            break;
        }
        *field
            .offset(
                size as isize,
            ) = (value & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
        value >>= 8 as libc::c_int;
    };
}
#[no_mangle]
pub static mut byte_get: Option::<
    unsafe extern "C" fn(*const libc::c_uchar, libc::c_uint) -> elf_vma,
> = None;
#[no_mangle]
pub unsafe extern "C" fn byte_get_signed(
    mut field: *const libc::c_uchar,
    mut size: libc::c_uint,
) -> elf_vma {
    let mut x: elf_vma = byte_get.expect("non-null function pointer")(field, size);
    match size {
        1 => {
            return (x ^ 0x80 as libc::c_int as libc::c_ulong)
                .wrapping_sub(0x80 as libc::c_int as libc::c_ulong);
        }
        2 => {
            return (x ^ 0x8000 as libc::c_int as libc::c_ulong)
                .wrapping_sub(0x8000 as libc::c_int as libc::c_ulong);
        }
        3 => {
            return (x ^ 0x800000 as libc::c_int as libc::c_ulong)
                .wrapping_sub(0x800000 as libc::c_int as libc::c_ulong);
        }
        4 => {
            return (x ^ 0x80000000 as libc::c_uint as libc::c_ulong)
                .wrapping_sub(0x80000000 as libc::c_uint as libc::c_ulong);
        }
        5 | 6 | 7 | 8 => return x,
        _ => {
            abort();
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn byte_get_little_endian(
    mut field: *const libc::c_uchar,
    mut size: libc::c_uint,
) -> elf_vma {
    let mut current_block_13: u64;
    match size {
        1 => return *field as elf_vma,
        2 => {
            return (*field.offset(0 as libc::c_int as isize) as libc::c_uint
                | (*field.offset(1 as libc::c_int as isize) as libc::c_uint)
                    << 8 as libc::c_int) as elf_vma;
        }
        3 => {
            return *field.offset(0 as libc::c_int as isize) as libc::c_ulong
                | (*field.offset(1 as libc::c_int as isize) as libc::c_ulong)
                    << 8 as libc::c_int
                | (*field.offset(2 as libc::c_int as isize) as libc::c_ulong)
                    << 16 as libc::c_int;
        }
        4 => {
            return *field.offset(0 as libc::c_int as isize) as libc::c_ulong
                | (*field.offset(1 as libc::c_int as isize) as libc::c_ulong)
                    << 8 as libc::c_int
                | (*field.offset(2 as libc::c_int as isize) as libc::c_ulong)
                    << 16 as libc::c_int
                | (*field.offset(3 as libc::c_int as isize) as libc::c_ulong)
                    << 24 as libc::c_int;
        }
        5 => {
            if ::core::mem::size_of::<elf_vma>() as libc::c_ulong
                >= 8 as libc::c_int as libc::c_ulong
            {
                return *field.offset(0 as libc::c_int as isize) as elf_vma
                    | (*field.offset(1 as libc::c_int as isize) as elf_vma)
                        << 8 as libc::c_int
                    | (*field.offset(2 as libc::c_int as isize) as elf_vma)
                        << 16 as libc::c_int
                    | (*field.offset(3 as libc::c_int as isize) as elf_vma)
                        << 24 as libc::c_int
                    | (*field.offset(4 as libc::c_int as isize) as elf_vma)
                        << 32 as libc::c_int;
            }
            current_block_13 = 1972043240739276353;
        }
        6 => {
            current_block_13 = 1972043240739276353;
        }
        7 => {
            current_block_13 = 5374176796876528309;
        }
        8 => {
            current_block_13 = 15410123500013320168;
        }
        _ => {
            current_block_13 = 3003175661361588937;
        }
    }
    match current_block_13 {
        1972043240739276353 => {
            if ::core::mem::size_of::<elf_vma>() as libc::c_ulong
                >= 8 as libc::c_int as libc::c_ulong
            {
                return *field.offset(0 as libc::c_int as isize) as elf_vma
                    | (*field.offset(1 as libc::c_int as isize) as elf_vma)
                        << 8 as libc::c_int
                    | (*field.offset(2 as libc::c_int as isize) as elf_vma)
                        << 16 as libc::c_int
                    | (*field.offset(3 as libc::c_int as isize) as elf_vma)
                        << 24 as libc::c_int
                    | (*field.offset(4 as libc::c_int as isize) as elf_vma)
                        << 32 as libc::c_int
                    | (*field.offset(5 as libc::c_int as isize) as elf_vma)
                        << 40 as libc::c_int;
            }
            current_block_13 = 5374176796876528309;
        }
        _ => {}
    }
    match current_block_13 {
        5374176796876528309 => {
            if ::core::mem::size_of::<elf_vma>() as libc::c_ulong
                >= 8 as libc::c_int as libc::c_ulong
            {
                return *field.offset(0 as libc::c_int as isize) as elf_vma
                    | (*field.offset(1 as libc::c_int as isize) as elf_vma)
                        << 8 as libc::c_int
                    | (*field.offset(2 as libc::c_int as isize) as elf_vma)
                        << 16 as libc::c_int
                    | (*field.offset(3 as libc::c_int as isize) as elf_vma)
                        << 24 as libc::c_int
                    | (*field.offset(4 as libc::c_int as isize) as elf_vma)
                        << 32 as libc::c_int
                    | (*field.offset(5 as libc::c_int as isize) as elf_vma)
                        << 40 as libc::c_int
                    | (*field.offset(6 as libc::c_int as isize) as elf_vma)
                        << 48 as libc::c_int;
            }
            current_block_13 = 15410123500013320168;
        }
        _ => {}
    }
    match current_block_13 {
        15410123500013320168 => {
            if ::core::mem::size_of::<elf_vma>() as libc::c_ulong
                >= 8 as libc::c_int as libc::c_ulong
            {
                return *field.offset(0 as libc::c_int as isize) as elf_vma
                    | (*field.offset(1 as libc::c_int as isize) as elf_vma)
                        << 8 as libc::c_int
                    | (*field.offset(2 as libc::c_int as isize) as elf_vma)
                        << 16 as libc::c_int
                    | (*field.offset(3 as libc::c_int as isize) as elf_vma)
                        << 24 as libc::c_int
                    | (*field.offset(4 as libc::c_int as isize) as elf_vma)
                        << 32 as libc::c_int
                    | (*field.offset(5 as libc::c_int as isize) as elf_vma)
                        << 40 as libc::c_int
                    | (*field.offset(6 as libc::c_int as isize) as elf_vma)
                        << 48 as libc::c_int
                    | (*field.offset(7 as libc::c_int as isize) as elf_vma)
                        << 56 as libc::c_int;
            }
        }
        _ => {}
    }
    error(
        dcgettext(
            0 as *const libc::c_char,
            b"Unhandled data length: %d\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        size,
    );
    abort();
}
#[no_mangle]
pub unsafe extern "C" fn byte_get_big_endian(
    mut field: *const libc::c_uchar,
    mut size: libc::c_uint,
) -> elf_vma {
    let mut current_block_13: u64;
    match size {
        1 => return *field as elf_vma,
        2 => {
            return (*field.offset(1 as libc::c_int as isize) as libc::c_uint
                | ((*field.offset(0 as libc::c_int as isize) as libc::c_int)
                    << 8 as libc::c_int) as libc::c_uint) as elf_vma;
        }
        3 => {
            return *field.offset(2 as libc::c_int as isize) as libc::c_ulong
                | (*field.offset(1 as libc::c_int as isize) as libc::c_ulong)
                    << 8 as libc::c_int
                | (*field.offset(0 as libc::c_int as isize) as libc::c_ulong)
                    << 16 as libc::c_int;
        }
        4 => {
            return *field.offset(3 as libc::c_int as isize) as libc::c_ulong
                | (*field.offset(2 as libc::c_int as isize) as libc::c_ulong)
                    << 8 as libc::c_int
                | (*field.offset(1 as libc::c_int as isize) as libc::c_ulong)
                    << 16 as libc::c_int
                | (*field.offset(0 as libc::c_int as isize) as libc::c_ulong)
                    << 24 as libc::c_int;
        }
        5 => {
            if ::core::mem::size_of::<elf_vma>() as libc::c_ulong
                >= 8 as libc::c_int as libc::c_ulong
            {
                return *field.offset(4 as libc::c_int as isize) as elf_vma
                    | (*field.offset(3 as libc::c_int as isize) as elf_vma)
                        << 8 as libc::c_int
                    | (*field.offset(2 as libc::c_int as isize) as elf_vma)
                        << 16 as libc::c_int
                    | (*field.offset(1 as libc::c_int as isize) as elf_vma)
                        << 24 as libc::c_int
                    | (*field.offset(0 as libc::c_int as isize) as elf_vma)
                        << 32 as libc::c_int;
            }
            current_block_13 = 3572664042095076746;
        }
        6 => {
            current_block_13 = 3572664042095076746;
        }
        7 => {
            current_block_13 = 11854077921582779863;
        }
        8 => {
            current_block_13 = 2395149915345925908;
        }
        _ => {
            current_block_13 = 8307027339092315808;
        }
    }
    match current_block_13 {
        3572664042095076746 => {
            if ::core::mem::size_of::<elf_vma>() as libc::c_ulong
                >= 8 as libc::c_int as libc::c_ulong
            {
                return *field.offset(5 as libc::c_int as isize) as elf_vma
                    | (*field.offset(4 as libc::c_int as isize) as elf_vma)
                        << 8 as libc::c_int
                    | (*field.offset(3 as libc::c_int as isize) as elf_vma)
                        << 16 as libc::c_int
                    | (*field.offset(2 as libc::c_int as isize) as elf_vma)
                        << 24 as libc::c_int
                    | (*field.offset(1 as libc::c_int as isize) as elf_vma)
                        << 32 as libc::c_int
                    | (*field.offset(0 as libc::c_int as isize) as elf_vma)
                        << 40 as libc::c_int;
            }
            current_block_13 = 11854077921582779863;
        }
        _ => {}
    }
    match current_block_13 {
        11854077921582779863 => {
            if ::core::mem::size_of::<elf_vma>() as libc::c_ulong
                >= 8 as libc::c_int as libc::c_ulong
            {
                return *field.offset(6 as libc::c_int as isize) as elf_vma
                    | (*field.offset(5 as libc::c_int as isize) as elf_vma)
                        << 8 as libc::c_int
                    | (*field.offset(4 as libc::c_int as isize) as elf_vma)
                        << 16 as libc::c_int
                    | (*field.offset(3 as libc::c_int as isize) as elf_vma)
                        << 24 as libc::c_int
                    | (*field.offset(2 as libc::c_int as isize) as elf_vma)
                        << 32 as libc::c_int
                    | (*field.offset(1 as libc::c_int as isize) as elf_vma)
                        << 40 as libc::c_int
                    | (*field.offset(0 as libc::c_int as isize) as elf_vma)
                        << 48 as libc::c_int;
            }
            current_block_13 = 2395149915345925908;
        }
        _ => {}
    }
    match current_block_13 {
        2395149915345925908 => {
            if ::core::mem::size_of::<elf_vma>() as libc::c_ulong
                >= 8 as libc::c_int as libc::c_ulong
            {
                return *field.offset(7 as libc::c_int as isize) as elf_vma
                    | (*field.offset(6 as libc::c_int as isize) as elf_vma)
                        << 8 as libc::c_int
                    | (*field.offset(5 as libc::c_int as isize) as elf_vma)
                        << 16 as libc::c_int
                    | (*field.offset(4 as libc::c_int as isize) as elf_vma)
                        << 24 as libc::c_int
                    | (*field.offset(3 as libc::c_int as isize) as elf_vma)
                        << 32 as libc::c_int
                    | (*field.offset(2 as libc::c_int as isize) as elf_vma)
                        << 40 as libc::c_int
                    | (*field.offset(1 as libc::c_int as isize) as elf_vma)
                        << 48 as libc::c_int
                    | (*field.offset(0 as libc::c_int as isize) as elf_vma)
                        << 56 as libc::c_int;
            }
        }
        _ => {}
    }
    error(
        dcgettext(
            0 as *const libc::c_char,
            b"Unhandled data length: %d\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        size,
    );
    abort();
}
#[no_mangle]
pub unsafe extern "C" fn adjust_relative_path(
    mut file_name: *const libc::c_char,
    mut name: *const libc::c_char,
    mut name_len: libc::c_ulong,
) -> *mut libc::c_char {
    let mut member_file_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut base_name: *const libc::c_char = lbasename(file_name);
    let mut amt: size_t = 0;
    if *name.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32
        || *name.offset(0 as libc::c_int as isize) as libc::c_int == '\\' as i32
            && 0 as libc::c_int != 0
        || *name.offset(0 as libc::c_int as isize) as libc::c_int != 0
            && *name.offset(1 as libc::c_int as isize) as libc::c_int == ':' as i32
            && 0 as libc::c_int != 0 || base_name == file_name
    {
        amt = name_len.wrapping_add(1 as libc::c_int as libc::c_ulong);
        if amt == 0 as libc::c_int as libc::c_ulong {
            return 0 as *mut libc::c_char;
        }
        member_file_name = malloc(amt) as *mut libc::c_char;
        if member_file_name.is_null() {
            error(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Out of memory\n\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            return 0 as *mut libc::c_char;
        }
        memcpy(
            member_file_name as *mut libc::c_void,
            name as *const libc::c_void,
            name_len,
        );
        *member_file_name.offset(name_len as isize) = '\0' as i32 as libc::c_char;
    } else {
        let mut prefix_len: size_t = base_name.offset_from(file_name) as libc::c_long
            as size_t;
        amt = prefix_len
            .wrapping_add(name_len)
            .wrapping_add(1 as libc::c_int as libc::c_ulong);
        if amt < prefix_len || amt < name_len {
            error(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Abnormal length of thin archive member name: %lx\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                name_len,
            );
            return 0 as *mut libc::c_char;
        }
        member_file_name = malloc(amt) as *mut libc::c_char;
        if member_file_name.is_null() {
            error(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Out of memory\n\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            return 0 as *mut libc::c_char;
        }
        memcpy(
            member_file_name as *mut libc::c_void,
            file_name as *const libc::c_void,
            prefix_len,
        );
        memcpy(
            member_file_name.offset(prefix_len as isize) as *mut libc::c_void,
            name as *const libc::c_void,
            name_len,
        );
        *member_file_name
            .offset(
                prefix_len.wrapping_add(name_len) as isize,
            ) = '\0' as i32 as libc::c_char;
    }
    return member_file_name;
}
#[no_mangle]
pub unsafe extern "C" fn setup_archive(
    mut arch: *mut archive_info,
    mut file_name: *const libc::c_char,
    mut file: *mut FILE,
    mut file_size: off_t,
    mut is_thin_archive: libc::c_int,
    mut read_symbols: libc::c_int,
) -> libc::c_int {
    let mut got: size_t = 0;
    (*arch).file_name = strdup(file_name);
    (*arch).file = file;
    (*arch).index_num = 0 as libc::c_int as elf_vma;
    (*arch).index_array = 0 as *mut elf_vma;
    (*arch).sym_table = 0 as *mut libc::c_char;
    (*arch).sym_size = 0 as libc::c_int as libc::c_ulong;
    (*arch).longnames = 0 as *mut libc::c_char;
    (*arch).longnames_size = 0 as libc::c_int as libc::c_ulong;
    (*arch).nested_member_origin = 0 as libc::c_int as libc::c_ulong;
    (*arch).is_thin_archive = is_thin_archive;
    (*arch).uses_64bit_indices = 0 as libc::c_int;
    (*arch).next_arhdr_offset = 8 as libc::c_int as libc::c_ulong;
    if fseek(file, 8 as libc::c_int as libc::c_long, 0 as libc::c_int)
        != 0 as libc::c_int
    {
        error(
            dcgettext(
                0 as *const libc::c_char,
                b"%s: failed to seek to first archive header\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            file_name,
        );
        return 1 as libc::c_int;
    }
    got = fread(
        &mut (*arch).arhdr as *mut ar_hdr as *mut libc::c_void,
        1 as libc::c_int as libc::c_ulong,
        ::core::mem::size_of::<ar_hdr>() as libc::c_ulong,
        file,
    );
    if got != ::core::mem::size_of::<ar_hdr>() as libc::c_ulong {
        if got == 0 as libc::c_int as libc::c_ulong {
            return 0 as libc::c_int;
        }
        error(
            dcgettext(
                0 as *const libc::c_char,
                b"%s: failed to read archive header\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            file_name,
        );
        return 1 as libc::c_int;
    }
    if startswith(
        ((*arch).arhdr.ar_name).as_mut_ptr(),
        b"/               \0" as *const u8 as *const libc::c_char,
    ) {
        if process_archive_index_and_symbols(
            arch,
            4 as libc::c_int as libc::c_uint,
            read_symbols,
        ) == 0
        {
            return 1 as libc::c_int;
        }
    } else if startswith(
        ((*arch).arhdr.ar_name).as_mut_ptr(),
        b"/SYM64/         \0" as *const u8 as *const libc::c_char,
    ) {
        (*arch).uses_64bit_indices = 1 as libc::c_int;
        if process_archive_index_and_symbols(
            arch,
            8 as libc::c_int as libc::c_uint,
            read_symbols,
        ) == 0
        {
            return 1 as libc::c_int;
        }
    } else if read_symbols != 0 {
        printf(
            dcgettext(
                0 as *const libc::c_char,
                b"%s has no archive index\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            file_name,
        );
    }
    if startswith(
        ((*arch).arhdr.ar_name).as_mut_ptr(),
        b"//              \0" as *const u8 as *const libc::c_char,
    ) {
        let mut fmag_save: libc::c_char = (*arch)
            .arhdr
            .ar_fmag[0 as libc::c_int as usize];
        (*arch)
            .arhdr
            .ar_fmag[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        (*arch)
            .longnames_size = strtoul(
            ((*arch).arhdr.ar_size).as_mut_ptr(),
            0 as *mut *mut libc::c_char,
            10 as libc::c_int,
        );
        (*arch).arhdr.ar_fmag[0 as libc::c_int as usize] = fmag_save;
        if (*arch).longnames_size < 8 as libc::c_int as libc::c_ulong {
            error(
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: long name table is too small, (size = %ld)\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                file_name,
                (*arch).longnames_size,
            );
            return 1 as libc::c_int;
        }
        if (*arch).longnames_size as off_t > file_size
            || ((*arch).longnames_size as libc::c_long)
                < 0 as libc::c_int as libc::c_long
        {
            error(
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: long name table is too big, (size = 0x%lx)\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                file_name,
                (*arch).longnames_size,
            );
            return 1 as libc::c_int;
        }
        (*arch)
            .next_arhdr_offset = ((*arch).next_arhdr_offset)
            .wrapping_add(
                (::core::mem::size_of::<ar_hdr>() as libc::c_ulong)
                    .wrapping_add((*arch).longnames_size),
            );
        (*arch)
            .longnames = malloc(
            ((*arch).longnames_size).wrapping_add(1 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
        if ((*arch).longnames).is_null() {
            error(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Out of memory reading long symbol names in archive\n\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            return 1 as libc::c_int;
        }
        if fread(
            (*arch).longnames as *mut libc::c_void,
            (*arch).longnames_size,
            1 as libc::c_int as libc::c_ulong,
            file,
        ) != 1 as libc::c_int as libc::c_ulong
        {
            free((*arch).longnames as *mut libc::c_void);
            (*arch).longnames = 0 as *mut libc::c_char;
            error(
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: failed to read long symbol name string table\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                file_name,
            );
            return 1 as libc::c_int;
        }
        if (*arch).longnames_size & 1 as libc::c_int as libc::c_ulong
            != 0 as libc::c_int as libc::c_ulong
        {
            getc(file);
        }
        *((*arch).longnames)
            .offset((*arch).longnames_size as isize) = 0 as libc::c_int as libc::c_char;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn setup_nested_archive(
    mut nested_arch: *mut archive_info,
    mut member_file_name: *const libc::c_char,
) -> libc::c_int {
    let mut member_file: *mut FILE = 0 as *mut FILE;
    let mut statbuf: stat = stat {
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
    if !((*nested_arch).file_name).is_null()
        && strcmp((*nested_arch).file_name, member_file_name) == 0 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    if !((*nested_arch).file).is_null() {
        fclose((*nested_arch).file);
        (*nested_arch).file = 0 as *mut FILE;
    }
    release_archive(nested_arch);
    member_file = fopen(member_file_name, b"rb\0" as *const u8 as *const libc::c_char);
    if member_file.is_null() {
        return 1 as libc::c_int;
    }
    if fstat(fileno(member_file), &mut statbuf) < 0 as libc::c_int {
        return 1 as libc::c_int;
    }
    return setup_archive(
        nested_arch,
        member_file_name,
        member_file,
        statbuf.st_size,
        0 as libc::c_int,
        0 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn release_archive(mut arch: *mut archive_info) {
    free((*arch).file_name as *mut libc::c_void);
    free((*arch).index_array as *mut libc::c_void);
    free((*arch).sym_table as *mut libc::c_void);
    free((*arch).longnames as *mut libc::c_void);
    (*arch).file_name = 0 as *mut libc::c_char;
    (*arch).index_array = 0 as *mut elf_vma;
    (*arch).sym_table = 0 as *mut libc::c_char;
    (*arch).longnames = 0 as *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn get_archive_member_name(
    mut arch: *mut archive_info,
    mut nested_arch: *mut archive_info,
) -> *mut libc::c_char {
    let mut j: libc::c_ulong = 0;
    let mut k: libc::c_ulong = 0;
    if (*arch).arhdr.ar_name[0 as libc::c_int as usize] as libc::c_int == '/' as i32 {
        let mut endp: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut member_file_name: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut member_name: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut fmag_save: libc::c_char = 0;
        if ((*arch).longnames).is_null()
            || (*arch).longnames_size == 0 as libc::c_int as libc::c_ulong
        {
            error(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Archive member uses long names, but no longname table found\n\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            return 0 as *mut libc::c_char;
        }
        (*arch).nested_member_origin = 0 as libc::c_int as libc::c_ulong;
        fmag_save = (*arch).arhdr.ar_fmag[0 as libc::c_int as usize];
        (*arch)
            .arhdr
            .ar_fmag[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        j = strtoul(
            ((*arch).arhdr.ar_name).as_mut_ptr().offset(1 as libc::c_int as isize),
            &mut endp,
            10 as libc::c_int,
        );
        k = j;
        if (*arch).is_thin_archive != 0 && !endp.is_null()
            && *endp as libc::c_int == ':' as i32
        {
            (*arch)
                .nested_member_origin = strtoul(
                endp.offset(1 as libc::c_int as isize),
                0 as *mut *mut libc::c_char,
                10 as libc::c_int,
            );
        }
        (*arch).arhdr.ar_fmag[0 as libc::c_int as usize] = fmag_save;
        if j > (*arch).longnames_size {
            error(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Found long name index (%ld) beyond end of long name table\n\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                j,
            );
            return 0 as *mut libc::c_char;
        }
        while j < (*arch).longnames_size
            && *((*arch).longnames).offset(j as isize) as libc::c_int != '\n' as i32
            && *((*arch).longnames).offset(j as isize) as libc::c_int != '\0' as i32
        {
            j = j.wrapping_add(1);
            j;
        }
        if j > 0 as libc::c_int as libc::c_ulong
            && *((*arch).longnames)
                .offset(j.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                as libc::c_int == '/' as i32
        {
            j = j.wrapping_sub(1);
            j;
        }
        if j > (*arch).longnames_size {
            j = (*arch).longnames_size;
        }
        *((*arch).longnames).offset(j as isize) = '\0' as i32 as libc::c_char;
        if (*arch).is_thin_archive == 0
            || (*arch).nested_member_origin == 0 as libc::c_int as libc::c_ulong
        {
            return xstrdup(((*arch).longnames).offset(k as isize));
        }
        if k >= j {
            error(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Invalid Thin archive member name\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            return 0 as *mut libc::c_char;
        }
        member_file_name = adjust_relative_path(
            (*arch).file_name,
            ((*arch).longnames).offset(k as isize),
            j.wrapping_sub(k),
        );
        if !member_file_name.is_null()
            && setup_nested_archive(nested_arch, member_file_name) == 0 as libc::c_int
        {
            member_name = get_archive_member_name_at(
                nested_arch,
                (*arch).nested_member_origin,
                0 as *mut archive_info,
            );
            if !member_name.is_null() {
                free(member_file_name as *mut libc::c_void);
                return member_name;
            }
        }
        free(member_file_name as *mut libc::c_void);
        return xstrdup(((*arch).longnames).offset(k as isize));
    }
    j = 0 as libc::c_int as libc::c_ulong;
    while j < ::core::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong {
        if (*arch).arhdr.ar_name[j as usize] as libc::c_int == '/' as i32 {
            (*arch).arhdr.ar_name[j as usize] = '\0' as i32 as libc::c_char;
            return xstrdup(((*arch).arhdr.ar_name).as_mut_ptr());
        }
        j = j.wrapping_add(1);
        j;
    }
    let mut name: *mut libc::c_char = xmalloc(
        (::core::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong)
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    memcpy(
        name as *mut libc::c_void,
        ((*arch).arhdr.ar_name).as_mut_ptr() as *const libc::c_void,
        ::core::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
    );
    *name
        .offset(
            ::core::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong as isize,
        ) = '\0' as i32 as libc::c_char;
    return name;
}
#[no_mangle]
pub unsafe extern "C" fn get_archive_member_name_at(
    mut arch: *mut archive_info,
    mut offset: libc::c_ulong,
    mut nested_arch: *mut archive_info,
) -> *mut libc::c_char {
    let mut got: size_t = 0;
    if fseek((*arch).file, offset as libc::c_long, 0 as libc::c_int) != 0 as libc::c_int
    {
        error(
            dcgettext(
                0 as *const libc::c_char,
                b"%s: failed to seek to next file name\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            (*arch).file_name,
        );
        return 0 as *mut libc::c_char;
    }
    got = fread(
        &mut (*arch).arhdr as *mut ar_hdr as *mut libc::c_void,
        1 as libc::c_int as libc::c_ulong,
        ::core::mem::size_of::<ar_hdr>() as libc::c_ulong,
        (*arch).file,
    );
    if got != ::core::mem::size_of::<ar_hdr>() as libc::c_ulong {
        error(
            dcgettext(
                0 as *const libc::c_char,
                b"%s: failed to read archive header\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            (*arch).file_name,
        );
        return 0 as *mut libc::c_char;
    }
    if memcmp(
        ((*arch).arhdr.ar_fmag).as_mut_ptr() as *const libc::c_void,
        b"`\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        2 as libc::c_int as libc::c_ulong,
    ) != 0 as libc::c_int
    {
        error(
            dcgettext(
                0 as *const libc::c_char,
                b"%s: did not find a valid archive header\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            (*arch).file_name,
        );
        return 0 as *mut libc::c_char;
    }
    return get_archive_member_name(arch, nested_arch);
}
#[no_mangle]
pub unsafe extern "C" fn make_qualified_name(
    mut arch: *mut archive_info,
    mut nested_arch: *mut archive_info,
    mut member_name: *const libc::c_char,
) -> *mut libc::c_char {
    let mut error_name: *const libc::c_char = dcgettext(
        0 as *const libc::c_char,
        b"<corrupt>\0" as *const u8 as *const libc::c_char,
        5 as libc::c_int,
    );
    let mut len: size_t = 0;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    len = (strlen((*arch).file_name))
        .wrapping_add(strlen(member_name))
        .wrapping_add(3 as libc::c_int as libc::c_ulong);
    if (*arch).is_thin_archive != 0
        && (*arch).nested_member_origin != 0 as libc::c_int as libc::c_ulong
    {
        if !((*nested_arch).file_name).is_null() {
            len = (len as libc::c_ulong)
                .wrapping_add(
                    (strlen((*nested_arch).file_name))
                        .wrapping_add(2 as libc::c_int as libc::c_ulong),
                ) as size_t as size_t;
        } else {
            len = (len as libc::c_ulong)
                .wrapping_add(
                    (strlen(error_name)).wrapping_add(2 as libc::c_int as libc::c_ulong),
                ) as size_t as size_t;
        }
    }
    name = malloc(len) as *mut libc::c_char;
    if name.is_null() {
        error(
            dcgettext(
                0 as *const libc::c_char,
                b"Out of memory\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return 0 as *mut libc::c_char;
    }
    if (*arch).is_thin_archive != 0
        && (*arch).nested_member_origin != 0 as libc::c_int as libc::c_ulong
    {
        if !((*nested_arch).file_name).is_null() {
            snprintf(
                name,
                len,
                b"%s[%s(%s)]\0" as *const u8 as *const libc::c_char,
                (*arch).file_name,
                (*nested_arch).file_name,
                member_name,
            );
        } else {
            snprintf(
                name,
                len,
                b"%s[%s(%s)]\0" as *const u8 as *const libc::c_char,
                (*arch).file_name,
                error_name,
                member_name,
            );
        }
    } else if (*arch).is_thin_archive != 0 {
        snprintf(
            name,
            len,
            b"%s[%s]\0" as *const u8 as *const libc::c_char,
            (*arch).file_name,
            member_name,
        );
    } else {
        snprintf(
            name,
            len,
            b"%s(%s)\0" as *const u8 as *const libc::c_char,
            (*arch).file_name,
            member_name,
        );
    }
    return name;
}
unsafe extern "C" fn process_archive_index_and_symbols(
    mut arch: *mut archive_info,
    mut sizeof_ar_index: libc::c_uint,
    mut read_symbols: libc::c_int,
) -> libc::c_int {
    let mut got: size_t = 0;
    let mut size: libc::c_ulong = 0;
    let mut fmag_save: libc::c_char = 0;
    fmag_save = (*arch).arhdr.ar_fmag[0 as libc::c_int as usize];
    (*arch).arhdr.ar_fmag[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    size = strtoul(
        ((*arch).arhdr.ar_size).as_mut_ptr(),
        0 as *mut *mut libc::c_char,
        10 as libc::c_int,
    );
    (*arch).arhdr.ar_fmag[0 as libc::c_int as usize] = fmag_save;
    if (size as libc::c_long) < 0 as libc::c_int as libc::c_long {
        error(
            dcgettext(
                0 as *const libc::c_char,
                b"%s: invalid archive header size: %ld\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            (*arch).file_name,
            size,
        );
        return 0 as libc::c_int;
    }
    size = size.wrapping_add(size & 1 as libc::c_int as libc::c_ulong);
    (*arch)
        .next_arhdr_offset = ((*arch).next_arhdr_offset)
        .wrapping_add(
            (::core::mem::size_of::<ar_hdr>() as libc::c_ulong).wrapping_add(size),
        );
    if read_symbols == 0 {
        if fseek((*arch).file, size as libc::c_long, 1 as libc::c_int)
            != 0 as libc::c_int
        {
            error(
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: failed to skip archive symbol table\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*arch).file_name,
            );
            return 0 as libc::c_int;
        }
    } else {
        let mut i: libc::c_ulong = 0;
        let mut integer_buffer: [libc::c_uchar; 8] = [0; 8];
        let mut index_buffer: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        if sizeof_ar_index as libc::c_ulong
            <= ::core::mem::size_of::<[libc::c_uchar; 8]>() as libc::c_ulong
        {} else {
            __assert_fail(
                b"sizeof_ar_index <= sizeof integer_buffer\0" as *const u8
                    as *const libc::c_char,
                b"elfcomm.c\0" as *const u8 as *const libc::c_char,
                384 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 80],
                    &[libc::c_char; 80],
                >(
                    b"int process_archive_index_and_symbols(struct archive_info *, unsigned int, int)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_12247: {
            if sizeof_ar_index as libc::c_ulong
                <= ::core::mem::size_of::<[libc::c_uchar; 8]>() as libc::c_ulong
            {} else {
                __assert_fail(
                    b"sizeof_ar_index <= sizeof integer_buffer\0" as *const u8
                        as *const libc::c_char,
                    b"elfcomm.c\0" as *const u8 as *const libc::c_char,
                    384 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 80],
                        &[libc::c_char; 80],
                    >(
                        b"int process_archive_index_and_symbols(struct archive_info *, unsigned int, int)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        if size < sizeof_ar_index as libc::c_ulong {
            error(
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: the archive index is empty\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*arch).file_name,
            );
            return 0 as libc::c_int;
        }
        got = fread(
            integer_buffer.as_mut_ptr() as *mut libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            sizeof_ar_index as libc::c_ulong,
            (*arch).file,
        );
        if got != sizeof_ar_index as libc::c_ulong {
            error(
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: failed to read archive index\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*arch).file_name,
            );
            return 0 as libc::c_int;
        }
        (*arch)
            .index_num = byte_get_big_endian(
            integer_buffer.as_mut_ptr(),
            sizeof_ar_index,
        );
        size = size.wrapping_sub(sizeof_ar_index as libc::c_ulong);
        if size < ((*arch).index_num).wrapping_mul(sizeof_ar_index as libc::c_ulong)
            || size < (*arch).index_num
        {
            error(
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: the archive index is supposed to have 0x%lx entries of %d bytes, but the size is only 0x%lx\n\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*arch).file_name,
                (*arch).index_num as libc::c_long,
                sizeof_ar_index,
                size,
            );
            return 0 as libc::c_int;
        }
        index_buffer = malloc(
            ((*arch).index_num).wrapping_mul(sizeof_ar_index as libc::c_ulong),
        ) as *mut libc::c_uchar;
        if index_buffer.is_null() {
            error(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Out of memory whilst trying to read archive symbol index\n\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            return 0 as libc::c_int;
        }
        got = fread(
            index_buffer as *mut libc::c_void,
            sizeof_ar_index as libc::c_ulong,
            (*arch).index_num,
            (*arch).file,
        );
        if got != (*arch).index_num {
            free(index_buffer as *mut libc::c_void);
            error(
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: failed to read archive index\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*arch).file_name,
            );
            return 0 as libc::c_int;
        }
        size = size
            .wrapping_sub(
                ((*arch).index_num).wrapping_mul(sizeof_ar_index as libc::c_ulong),
            );
        (*arch)
            .index_array = malloc(
            ((*arch).index_num)
                .wrapping_mul(::core::mem::size_of::<elf_vma>() as libc::c_ulong),
        ) as *mut elf_vma;
        if ((*arch).index_array).is_null() {
            free(index_buffer as *mut libc::c_void);
            error(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Out of memory whilst trying to convert the archive symbol index\n\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            return 0 as libc::c_int;
        }
        i = 0 as libc::c_int as libc::c_ulong;
        while i < (*arch).index_num {
            *((*arch).index_array)
                .offset(
                    i as isize,
                ) = byte_get_big_endian(
                index_buffer
                    .offset(i.wrapping_mul(sizeof_ar_index as libc::c_ulong) as isize),
                sizeof_ar_index,
            );
            i = i.wrapping_add(1);
            i;
        }
        free(index_buffer as *mut libc::c_void);
        if size < 1 as libc::c_int as libc::c_ulong {
            error(
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: the archive has an index but no symbols\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*arch).file_name,
            );
            return 0 as libc::c_int;
        }
        (*arch).sym_table = malloc(size) as *mut libc::c_char;
        if ((*arch).sym_table).is_null() {
            error(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Out of memory whilst trying to read archive index symbol table\n\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            return 0 as libc::c_int;
        }
        (*arch).sym_size = size;
        got = fread(
            (*arch).sym_table as *mut libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            size,
            (*arch).file,
        );
        if got != size {
            error(
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: failed to read archive index symbol table\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*arch).file_name,
            );
            return 0 as libc::c_int;
        }
    }
    got = fread(
        &mut (*arch).arhdr as *mut ar_hdr as *mut libc::c_void,
        1 as libc::c_int as libc::c_ulong,
        ::core::mem::size_of::<ar_hdr>() as libc::c_ulong,
        (*arch).file,
    );
    if got != ::core::mem::size_of::<ar_hdr>() as libc::c_ulong
        && got != 0 as libc::c_int as libc::c_ulong
    {
        error(
            dcgettext(
                0 as *const libc::c_char,
                b"%s: failed to read archive header following archive index\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            (*arch).file_name,
        );
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
