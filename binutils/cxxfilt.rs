#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(c_variadic)]
#![feature(extern_types)]
#![feature(label_break_value)]

#[macro_use]
extern crate c2rust_bitfields;
extern crate libc;
pub mod src {
pub mod bucomm;
pub mod filemode;
pub mod version;
} // mod src
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn getc(__stream: *mut FILE) -> libc::c_int;
    fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn free(_: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn bfd_set_error_program_name(_: *const libc::c_char);
    fn expandargv(_: *mut libc::c_int, _: *mut *mut *mut libc::c_char);
    fn xmalloc_set_program_name(_: *const libc::c_char);
    static mut current_demangling_style: demangling_styles;
    static libiberty_demanglers: [demangler_engine; 0];
    fn cplus_demangle(
        mangled: *const libc::c_char,
        options: libc::c_int,
    ) -> *mut libc::c_char;
    fn cplus_demangle_set_style(style: demangling_styles) -> demangling_styles;
    fn cplus_demangle_name_to_style(name: *const libc::c_char) -> demangling_styles;
    fn getopt_long(
        argc: libc::c_int,
        argv: *const *mut libc::c_char,
        shortopts: *const libc::c_char,
        longopts: *const option,
        longind: *mut libc::c_int,
    ) -> libc::c_int;
    static _sch_istable: [libc::c_ushort; 256];
    fn fatal(_: *const libc::c_char, _: ...) -> !;
    static mut program_name: *mut libc::c_char;
    fn print_version(_: *const libc::c_char);
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
pub type demangling_styles = libc::c_int;
pub const rust_demangling: demangling_styles = 131072;
pub const dlang_demangling: demangling_styles = 65536;
pub const gnat_demangling: demangling_styles = 32768;
pub const java_demangling: demangling_styles = 4;
pub const gnu_v3_demangling: demangling_styles = 16384;
pub const auto_demangling: demangling_styles = 256;
pub const unknown_demangling: demangling_styles = 0;
pub const no_demangling: demangling_styles = -1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct demangler_engine {
    pub demangling_style_name: *const libc::c_char,
    pub demangling_style: demangling_styles,
    pub demangling_style_doc: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
pub type C2RustUnnamed = libc::c_uint;
pub const _sch_isbasic: C2RustUnnamed = 3088;
pub const _sch_iscppsp: C2RustUnnamed = 3072;
pub const _sch_isgraph: C2RustUnnamed = 172;
pub const _sch_isidnum: C2RustUnnamed = 516;
pub const _sch_isalnum: C2RustUnnamed = 140;
pub const _sch_isalpha: C2RustUnnamed = 136;
pub const _sch_isnvsp: C2RustUnnamed = 2048;
pub const _sch_isvsp: C2RustUnnamed = 1024;
pub const _sch_isidst: C2RustUnnamed = 512;
pub const _sch_isxdigit: C2RustUnnamed = 256;
pub const _sch_isupper: C2RustUnnamed = 128;
pub const _sch_isspace: C2RustUnnamed = 64;
pub const _sch_ispunct: C2RustUnnamed = 32;
pub const _sch_isprint: C2RustUnnamed = 16;
pub const _sch_islower: C2RustUnnamed = 8;
pub const _sch_isdigit: C2RustUnnamed = 4;
pub const _sch_iscntrl: C2RustUnnamed = 2;
pub const _sch_isblank: C2RustUnnamed = 1;
#[inline]
unsafe extern "C" fn getchar() -> libc::c_int {
    return getc(stdin);
}
#[inline]
unsafe extern "C" fn putchar(mut __c: libc::c_int) -> libc::c_int {
    return putc(__c, stdout);
}
static mut flags: libc::c_int = (1 as libc::c_int) << 0 as libc::c_int
    | (1 as libc::c_int) << 1 as libc::c_int | (1 as libc::c_int) << 3 as libc::c_int;
static mut strip_underscore: libc::c_int = 0 as libc::c_int;
static mut long_options: [option; 13] = [
    {
        let mut init = option {
            name: b"strip-underscore\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: '_' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"format\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 's' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"help\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'h' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"no-params\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'p' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"no-strip-underscores\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'n' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"no-verbose\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'i' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"types\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 't' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"version\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'v' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"recurse-limit\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'R' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"recursion-limit\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'R' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"no-recurse-limit\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'r' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"no-recursion-limit\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'r' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: 0 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
];
unsafe extern "C" fn demangle_it(mut mangled_name: *mut libc::c_char) {
    let mut result: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut skip_first: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    if *mangled_name.offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32
        || *mangled_name.offset(0 as libc::c_int as isize) as libc::c_int == '$' as i32
    {
        skip_first = skip_first.wrapping_add(1);
        skip_first;
    }
    if strip_underscore != 0
        && *mangled_name.offset(skip_first as isize) as libc::c_int == '_' as i32
    {
        skip_first = skip_first.wrapping_add(1);
        skip_first;
    }
    result = cplus_demangle(mangled_name.offset(skip_first as isize), flags);
    if result.is_null() {
        printf(b"%s\0" as *const u8 as *const libc::c_char, mangled_name);
    } else {
        if *mangled_name.offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32 {
            putchar('.' as i32);
        }
        printf(b"%s\0" as *const u8 as *const libc::c_char, result);
        free(result as *mut libc::c_void);
    };
}
unsafe extern "C" fn print_demangler_list(mut stream: *mut FILE) {
    let mut demangler: *const demangler_engine = 0 as *const demangler_engine;
    fprintf(
        stream,
        b"{%s\0" as *const u8 as *const libc::c_char,
        (*libiberty_demanglers.as_ptr()).demangling_style_name,
    );
    demangler = libiberty_demanglers.as_ptr().offset(1 as libc::c_int as isize);
    while (*demangler).demangling_style as libc::c_int
        != unknown_demangling as libc::c_int
    {
        fprintf(
            stream,
            b",%s\0" as *const u8 as *const libc::c_char,
            (*demangler).demangling_style_name,
        );
        demangler = demangler.offset(1);
        demangler;
    }
    fprintf(stream, b"}\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn usage(mut stream: *mut FILE, mut status: libc::c_int) -> ! {
    fprintf(
        stream,
        b"Usage: %s [options] [mangled names]\n\0" as *const u8 as *const libc::c_char,
        program_name,
    );
    fprintf(
        stream,
        b"Options are:\n  [-_|--strip-underscore]     Ignore first leading underscore%s\n\0"
            as *const u8 as *const libc::c_char,
        if 0 as libc::c_int != 0 {
            b" (default)\0" as *const u8 as *const libc::c_char
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
    );
    fprintf(
        stream,
        b"  [-n|--no-strip-underscore]  Do not ignore a leading underscore%s\n\0"
            as *const u8 as *const libc::c_char,
        if 0 as libc::c_int != 0 {
            b"\0" as *const u8 as *const libc::c_char
        } else {
            b" (default)\0" as *const u8 as *const libc::c_char
        },
    );
    fprintf(
        stream,
        b"  [-p|--no-params]            Do not display function arguments\n  [-i|--no-verbose]           Do not show implementation details (if any)\n  [-R|--recurse-limit]        Enable a limit on recursion whilst demangling.  [Default]\n  ]-r|--no-recurse-limit]     Disable a limit on recursion whilst demangling\n  [-t|--types]                Also attempt to demangle type encodings\n  [-s|--format \0"
            as *const u8 as *const libc::c_char,
    );
    print_demangler_list(stream);
    fprintf(stream, b"]\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        stream,
        b"  [@<file>]                   Read extra options from <file>\n  [-h|--help]                 Display this information\n  [-v|--version]              Show the version information\nDemangled names are displayed to stdout.\nIf a name cannot be demangled it is just echoed to stdout.\nIf no names are provided on the command line, stdin is read.\n\0"
            as *const u8 as *const libc::c_char,
    );
    if (*::core::mem::transmute::<
        &[u8; 39],
        &[libc::c_char; 39],
    >(b"<https://www.sourceware.org/bugzilla/>\0"))[0 as libc::c_int as usize]
        as libc::c_int != 0 && status == 0 as libc::c_int
    {
        fprintf(
            stream,
            dcgettext(
                0 as *const libc::c_char,
                b"Report bugs to %s.\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            b"<https://www.sourceware.org/bugzilla/>\0" as *const u8
                as *const libc::c_char,
        );
    }
    exit(status);
}
unsafe extern "C" fn standard_symbol_characters() -> *const libc::c_char {
    return b"_$.\0" as *const u8 as *const libc::c_char;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut c: libc::c_int = 0;
    let mut valid_symbols: *const libc::c_char = 0 as *const libc::c_char;
    let mut style: demangling_styles = auto_demangling;
    program_name = *argv.offset(0 as libc::c_int as isize);
    xmalloc_set_program_name(program_name);
    bfd_set_error_program_name(program_name);
    expandargv(&mut argc, &mut argv);
    loop {
        c = getopt_long(
            argc,
            argv,
            b"_hinprRs:tv\0" as *const u8 as *const libc::c_char,
            long_options.as_ptr(),
            0 as *mut libc::c_int,
        );
        if !(c != -(1 as libc::c_int)) {
            break;
        }
        match c {
            63 => {
                usage(stderr, 1 as libc::c_int);
            }
            104 => {
                usage(stdout, 0 as libc::c_int);
            }
            110 => {
                strip_underscore = 0 as libc::c_int;
            }
            112 => {
                flags &= !((1 as libc::c_int) << 0 as libc::c_int);
            }
            114 => {
                flags |= (1 as libc::c_int) << 18 as libc::c_int;
            }
            82 => {
                flags &= !((1 as libc::c_int) << 18 as libc::c_int);
            }
            116 => {
                flags |= (1 as libc::c_int) << 4 as libc::c_int;
            }
            105 => {
                flags &= !((1 as libc::c_int) << 3 as libc::c_int);
            }
            118 => {
                print_version(b"c++filt\0" as *const u8 as *const libc::c_char);
                return 0 as libc::c_int;
            }
            95 => {
                strip_underscore = 1 as libc::c_int;
            }
            115 => {
                style = cplus_demangle_name_to_style(optarg);
                if style as libc::c_int == unknown_demangling as libc::c_int {
                    fprintf(
                        stderr,
                        b"%s: unknown demangling style `%s'\n\0" as *const u8
                            as *const libc::c_char,
                        program_name,
                        optarg,
                    );
                    return 1 as libc::c_int;
                }
                cplus_demangle_set_style(style);
            }
            _ => {}
        }
    }
    if optind < argc {
        while optind < argc {
            demangle_it(*argv.offset(optind as isize));
            putchar('\n' as i32);
            optind += 1;
            optind;
        }
        return 0 as libc::c_int;
    }
    match current_demangling_style as libc::c_int {
        256 | 16384 | 4 | 32768 | 65536 | 131072 => {
            valid_symbols = standard_symbol_characters();
        }
        _ => {
            fatal(
                b"Internal error: no symbol alphabet for current style\0" as *const u8
                    as *const libc::c_char,
            );
        }
    }
    loop {
        static mut mbuffer: [libc::c_char; 32767] = [0; 32767];
        let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
        c = getchar();
        while c != -(1 as libc::c_int)
            && (_sch_istable[(c & 0xff as libc::c_int) as usize] as libc::c_int
                & _sch_isalnum as libc::c_int as libc::c_ushort as libc::c_int != 0
                || !(strchr(valid_symbols, c)).is_null())
        {
            if i as libc::c_ulong
                >= (::core::mem::size_of::<[libc::c_char; 32767]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            {
                break;
            }
            let fresh0 = i;
            i = i.wrapping_add(1);
            mbuffer[fresh0 as usize] = c as libc::c_char;
            c = getchar();
        }
        if i > 0 as libc::c_int as libc::c_uint {
            mbuffer[i as usize] = 0 as libc::c_int as libc::c_char;
            demangle_it(mbuffer.as_mut_ptr());
        }
        if c == -(1 as libc::c_int) {
            break;
        }
        putchar(c);
        if c == '\n' as i32 {
            fflush(stdout);
        }
    }
    fflush(stdout);
    return 0 as libc::c_int;
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}
