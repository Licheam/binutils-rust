#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(extern_types)]


extern crate bfd_sys;
extern crate libctf_sys;
extern crate libiberty_sys;
extern crate zlib_sys;
extern crate libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn abort() -> !;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn yylex() -> libc::c_int;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union YYSTYPE {
    pub i: libc::c_int,
    pub s: *mut libc::c_char,
}
pub type yy_state_t = yytype_int8;
pub type yytype_int8 = libc::c_schar;
pub type yy_state_fast_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub union yyalloc {
    pub yyss_alloc: yy_state_t,
    pub yyvs_alloc: YYSTYPE,
}
static mut writecode: libc::c_char = 0;
static mut it: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut code: libc::c_int = 0;
static mut repeat: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut oldrepeat: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut name: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut rdepth: libc::c_int = 0;
static mut names: [*mut libc::c_char; 3] = [
    b" \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"[n]\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"[n][m]\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
];
static mut pnames: [*mut libc::c_char; 3] = [
    b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"*\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"**\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
];
unsafe extern "C" fn yyerror(mut s: *mut libc::c_char) -> libc::c_int {
    fprintf(stderr, b"%s\n\0" as *const u8 as *const libc::c_char, s);
    return 0 as libc::c_int;
}
#[no_mangle]
pub static mut yylval: YYSTYPE = YYSTYPE { i: 0 };
#[no_mangle]
pub unsafe extern "C" fn yyparse() -> libc::c_int {
    let mut current_block: u64;
    let mut yystate: yy_state_fast_t = 0;
    let mut yyerrstatus: libc::c_int = 0;
    let mut yyssa: [yy_state_t; 200] = [0; 200];
    let mut yyss: *mut yy_state_t = 0 as *mut yy_state_t;
    let mut yyssp: *mut yy_state_t = 0 as *mut yy_state_t;
    let mut yyvsa: [YYSTYPE; 200] = [YYSTYPE { i: 0 }; 200];
    let mut yyvs: *mut YYSTYPE = 0 as *mut YYSTYPE;
    let mut yyvsp: *mut YYSTYPE = 0 as *mut YYSTYPE;
    let mut yystacksize: libc::c_long = 0;
    let mut yyn: libc::c_int = 0;
    let mut yyresult: libc::c_int = 0;
    let mut yytoken: libc::c_int = 0 as libc::c_int;
    let mut yyval: YYSTYPE = YYSTYPE { i: 0 };
    let mut yylen: libc::c_int = 0 as libc::c_int;
    yyss = yyssa.as_mut_ptr();
    yyssp = yyss;
    yyvs = yyvsa.as_mut_ptr();
    yyvsp = yyvs;
    yystacksize = 200 as libc::c_int as libc::c_long;
    yystate = 0 as libc::c_int;
    yyerrstatus = 0 as libc::c_int;
    yynerrs = 0 as libc::c_int;
    yychar = -(2 as libc::c_int);
    's_64: loop {
        (0 as libc::c_int != 0
            && (0 as libc::c_int <= yystate && yystate < 55 as libc::c_int))
            as libc::c_int;
        *yyssp = yystate as yy_state_t;
        if yyss.offset(yystacksize as isize).offset(-(1 as libc::c_int as isize))
            <= yyssp
        {
            let mut yysize: libc::c_long = yyssp.offset_from(yyss) as libc::c_long
                + 1 as libc::c_int as libc::c_long;
            if 10000 as libc::c_int as libc::c_long <= yystacksize {
                current_block = 12565593370437119333;
                break;
            }
            yystacksize *= 2 as libc::c_int as libc::c_long;
            if (10000 as libc::c_int as libc::c_long) < yystacksize {
                yystacksize = 10000 as libc::c_int as libc::c_long;
            }
            let mut yyss1: *mut yy_state_t = yyss;
            let mut yyptr: *mut yyalloc = malloc(
                (yystacksize
                    * (::core::mem::size_of::<yy_state_t>() as libc::c_ulong
                        as libc::c_long
                        + ::core::mem::size_of::<YYSTYPE>() as libc::c_ulong
                            as libc::c_long)
                    + (::core::mem::size_of::<yyalloc>() as libc::c_ulong as libc::c_long
                        - 1 as libc::c_int as libc::c_long)) as libc::c_ulong,
            ) as *mut yyalloc;
            if yyptr.is_null() {
                current_block = 12565593370437119333;
                break;
            }
            let mut yynewbytes: libc::c_long = 0;
            libc::memcpy(
                &mut (*yyptr).yyss_alloc as *mut yy_state_t as *mut libc::c_void,
                yyss as *const libc::c_void,
                (yysize as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<yy_state_t>() as libc::c_ulong)
                    as libc::size_t,
            );
            yyss = &mut (*yyptr).yyss_alloc;
            yynewbytes = yystacksize
                * ::core::mem::size_of::<yy_state_t>() as libc::c_ulong as libc::c_long
                + (::core::mem::size_of::<yyalloc>() as libc::c_ulong as libc::c_long
                    - 1 as libc::c_int as libc::c_long);
            yyptr = yyptr
                .offset(
                    (yynewbytes
                        / ::core::mem::size_of::<yyalloc>() as libc::c_ulong
                            as libc::c_long) as isize,
                );
            let mut yynewbytes_0: libc::c_long = 0;
            libc::memcpy(
                &mut (*yyptr).yyvs_alloc as *mut YYSTYPE as *mut libc::c_void,
                yyvs as *const libc::c_void,
                (yysize as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<YYSTYPE>() as libc::c_ulong)
                    as libc::size_t,
            );
            yyvs = &mut (*yyptr).yyvs_alloc;
            yynewbytes_0 = yystacksize
                * ::core::mem::size_of::<YYSTYPE>() as libc::c_ulong as libc::c_long
                + (::core::mem::size_of::<yyalloc>() as libc::c_ulong as libc::c_long
                    - 1 as libc::c_int as libc::c_long);
            yyptr = yyptr
                .offset(
                    (yynewbytes_0
                        / ::core::mem::size_of::<yyalloc>() as libc::c_ulong
                            as libc::c_long) as isize,
                );
            if yyss1 != yyssa.as_mut_ptr() {
                free(yyss1 as *mut libc::c_void);
            }
            yyssp = yyss.offset(yysize as isize).offset(-(1 as libc::c_int as isize));
            yyvsp = yyvs.offset(yysize as isize).offset(-(1 as libc::c_int as isize));
            if yyss.offset(yystacksize as isize).offset(-(1 as libc::c_int as isize))
                <= yyssp
            {
                current_block = 12607448202022854059;
                break;
            }
        }
        if yystate == 3 as libc::c_int {
            yyresult = 0 as libc::c_int;
            current_block = 17482076912623907234;
            break;
        } else {
            yyn = yypact[yystate as usize] as libc::c_int;
            if yyn == -(14 as libc::c_int) {
                current_block = 11976493425328827790;
            } else {
                if yychar == -(2 as libc::c_int) {
                    yychar = yylex();
                }
                if yychar <= 0 as libc::c_int {
                    yytoken = 0 as libc::c_int;
                    yychar = yytoken;
                } else {
                    yytoken = if 0 as libc::c_int <= yychar
                        && yychar <= 263 as libc::c_int
                    {
                        yytranslate[yychar as usize] as libc::c_int
                    } else {
                        2 as libc::c_int
                    };
                }
                yyn += yytoken;
                if yyn < 0 as libc::c_int || (38 as libc::c_int) < yyn
                    || yycheck[yyn as usize] as libc::c_int != yytoken
                {
                    current_block = 11976493425328827790;
                } else {
                    yyn = yytable[yyn as usize] as libc::c_int;
                    if yyn <= 0 as libc::c_int {
                        yyn = -yyn;
                        current_block = 5474218235931721333;
                    } else {
                        if yyerrstatus != 0 {
                            yyerrstatus -= 1;
                            yyerrstatus;
                        }
                        yystate = yyn;
                        yyvsp = yyvsp.offset(1);
                        *yyvsp = yylval;
                        yychar = -(2 as libc::c_int);
                        current_block = 9232923187868260496;
                    }
                }
            }
            match current_block {
                11976493425328827790 => {
                    yyn = yydefact[yystate as usize] as libc::c_int;
                    if yyn == 0 as libc::c_int {
                        yytoken = if yychar == -(2 as libc::c_int) {
                            -(2 as libc::c_int)
                        } else if 0 as libc::c_int <= yychar
                            && yychar <= 263 as libc::c_int
                        {
                            yytranslate[yychar as usize] as libc::c_int
                        } else {
                            2 as libc::c_int
                        };
                        if yyerrstatus == 0 {
                            yynerrs += 1;
                            yynerrs;
                            yyerror(
                                b"syntax error\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                            );
                        }
                        if yyerrstatus == 3 as libc::c_int {
                            if yychar <= 0 as libc::c_int {
                                if yychar == 0 as libc::c_int {
                                    current_block = 12607448202022854059;
                                    break;
                                }
                            } else {
                                yydestruct(
                                    b"Error: discarding\0" as *const u8 as *const libc::c_char,
                                    yytoken,
                                    &mut yylval,
                                );
                                yychar = -(2 as libc::c_int);
                            }
                        }
                        yyerrstatus = 3 as libc::c_int;
                        loop {
                            yyn = yypact[yystate as usize] as libc::c_int;
                            if !(yyn == -(14 as libc::c_int)) {
                                yyn += 1 as libc::c_int;
                                if 0 as libc::c_int <= yyn && yyn <= 38 as libc::c_int
                                    && yycheck[yyn as usize] as libc::c_int == 1 as libc::c_int
                                {
                                    yyn = yytable[yyn as usize] as libc::c_int;
                                    if (0 as libc::c_int) < yyn {
                                        break;
                                    }
                                }
                            }
                            if yyssp == yyss {
                                current_block = 12607448202022854059;
                                break 's_64;
                            }
                            yydestruct(
                                b"Error: popping\0" as *const u8 as *const libc::c_char,
                                yystos[yystate as usize] as libc::c_int,
                                yyvsp,
                            );
                            yyvsp = yyvsp.offset(-(1 as libc::c_int as isize));
                            yyssp = yyssp.offset(-(1 as libc::c_int as isize));
                            yystate = *yyssp as yy_state_fast_t;
                        }
                        yyvsp = yyvsp.offset(1);
                        *yyvsp = yylval;
                        yystate = yyn;
                        current_block = 9232923187868260496;
                    } else {
                        current_block = 5474218235931721333;
                    }
                }
                _ => {}
            }
            match current_block {
                5474218235931721333 => {
                    yylen = yyr2[yyn as usize] as libc::c_int;
                    yyval = *yyvsp.offset((1 as libc::c_int - yylen) as isize);
                    match yyn {
                        2 => {
                            match writecode as libc::c_int {
                                105 => {
                                    printf(
                                        b"#ifdef SYSROFF_SWAP_IN\n\0" as *const u8
                                            as *const libc::c_char,
                                    );
                                }
                                112 => {
                                    printf(
                                        b"#ifdef SYSROFF_p\n\0" as *const u8 as *const libc::c_char,
                                    );
                                }
                                103 => {
                                    printf(
                                        b"#ifdef SYSROFF_SWAP_OUT\n\0" as *const u8
                                            as *const libc::c_char,
                                    );
                                }
                                99 => {
                                    printf(
                                        b"#ifdef SYSROFF_PRINT\n\0" as *const u8
                                            as *const libc::c_char,
                                    );
                                    printf(
                                        b"#include <stdio.h>\n\0" as *const u8
                                            as *const libc::c_char,
                                    );
                                    printf(
                                        b"#include <stdlib.h>\n\0" as *const u8
                                            as *const libc::c_char,
                                    );
                                    printf(
                                        b"#include <ansidecl.h>\n\0" as *const u8
                                            as *const libc::c_char,
                                    );
                                }
                                100 | _ => {}
                            }
                        }
                        3 => {
                            match writecode as libc::c_int {
                                105 | 112 | 103 | 99 => {
                                    printf(b"#endif\n\0" as *const u8 as *const libc::c_char);
                                }
                                100 | _ => {}
                            }
                        }
                        6 => {
                            it = (*yyvsp.offset(-(1 as libc::c_int) as isize)).s;
                            code = (*yyvsp.offset(0 as libc::c_int as isize)).i;
                            match writecode as libc::c_int {
                                100 => {
                                    printf(
                                        b"\n\n\n#define IT_%s_CODE 0x%x\n\0" as *const u8
                                            as *const libc::c_char,
                                        it,
                                        code,
                                    );
                                    printf(
                                        b"struct IT_%s;\n\0" as *const u8 as *const libc::c_char,
                                        it,
                                    );
                                    printf(
                                        b"extern void sysroff_swap_%s_in (struct IT_%s *);\n\0"
                                            as *const u8 as *const libc::c_char,
                                        (*yyvsp.offset(-(1 as libc::c_int) as isize)).s,
                                        it,
                                    );
                                    printf(
                                        b"extern void sysroff_swap_%s_out (FILE *, struct IT_%s *);\n\0"
                                            as *const u8 as *const libc::c_char,
                                        (*yyvsp.offset(-(1 as libc::c_int) as isize)).s,
                                        it,
                                    );
                                    printf(
                                        b"extern void sysroff_print_%s_out (struct IT_%s *);\n\0"
                                            as *const u8 as *const libc::c_char,
                                        (*yyvsp.offset(-(1 as libc::c_int) as isize)).s,
                                        it,
                                    );
                                    printf(
                                        b"struct IT_%s { \n\0" as *const u8 as *const libc::c_char,
                                        it,
                                    );
                                }
                                105 => {
                                    printf(
                                        b"void sysroff_swap_%s_in (struct IT_%s * ptr)\n\0"
                                            as *const u8 as *const libc::c_char,
                                        (*yyvsp.offset(-(1 as libc::c_int) as isize)).s,
                                        it,
                                    );
                                    printf(b"{\n\0" as *const u8 as *const libc::c_char);
                                    printf(
                                        b"\tunsigned char raw[255];\n\0" as *const u8
                                            as *const libc::c_char,
                                    );
                                    printf(
                                        b"\tint idx = 0;\n\0" as *const u8 as *const libc::c_char,
                                    );
                                    printf(
                                        b"\tint size;\n\0" as *const u8 as *const libc::c_char,
                                    );
                                    printf(
                                        b"\tmemset(raw,0,255);\n\0" as *const u8
                                            as *const libc::c_char,
                                    );
                                    printf(
                                        b"\tmemset(ptr,0,sizeof(*ptr));\n\0" as *const u8
                                            as *const libc::c_char,
                                    );
                                    printf(
                                        b"\tsize = fillup(raw);\n\0" as *const u8
                                            as *const libc::c_char,
                                    );
                                }
                                103 => {
                                    printf(
                                        b"void sysroff_swap_%s_out (FILE * ffile, struct IT_%s * ptr)\n\0"
                                            as *const u8 as *const libc::c_char,
                                        (*yyvsp.offset(-(1 as libc::c_int) as isize)).s,
                                        it,
                                    );
                                    printf(b"{\n\0" as *const u8 as *const libc::c_char);
                                    printf(
                                        b"\tunsigned char raw[255];\n\0" as *const u8
                                            as *const libc::c_char,
                                    );
                                    printf(
                                        b"\tint idx = 16;\n\0" as *const u8 as *const libc::c_char,
                                    );
                                    printf(
                                        b"\tmemset (raw, 0, 255);\n\0" as *const u8
                                            as *const libc::c_char,
                                    );
                                    printf(
                                        b"\tcode = IT_%s_CODE;\n\0" as *const u8
                                            as *const libc::c_char,
                                        it,
                                    );
                                }
                                111 => {
                                    printf(
                                        b"void sysroff_swap_%s_out (bfd * abfd, struct IT_%s * ptr)\n\0"
                                            as *const u8 as *const libc::c_char,
                                        (*yyvsp.offset(-(1 as libc::c_int) as isize)).s,
                                        it,
                                    );
                                    printf(b"{\n\0" as *const u8 as *const libc::c_char);
                                    printf(
                                        b"\tint idx = 0;\n\0" as *const u8 as *const libc::c_char,
                                    );
                                }
                                99 => {
                                    printf(
                                        b"void sysroff_print_%s_out (struct IT_%s *ptr)\n\0"
                                            as *const u8 as *const libc::c_char,
                                        (*yyvsp.offset(-(1 as libc::c_int) as isize)).s,
                                        it,
                                    );
                                    printf(b"{\n\0" as *const u8 as *const libc::c_char);
                                    printf(
                                        b"itheader(\"%s\", IT_%s_CODE);\n\0" as *const u8
                                            as *const libc::c_char,
                                        (*yyvsp.offset(-(1 as libc::c_int) as isize)).s,
                                        (*yyvsp.offset(-(1 as libc::c_int) as isize)).s,
                                    );
                                }
                                116 | _ => {}
                            }
                        }
                        7 => {
                            let mut current_block_101: u64;
                            match writecode as libc::c_int {
                                100 => {
                                    printf(b"};\n\0" as *const u8 as *const libc::c_char);
                                    current_block_101 = 4983594971376015098;
                                }
                                103 => {
                                    printf(
                                        b"\tchecksum(ffile,raw, idx, IT_%s_CODE);\n\0" as *const u8
                                            as *const libc::c_char,
                                        it,
                                    );
                                    current_block_101 = 15480363249811881210;
                                }
                                105 | 111 | 99 => {
                                    current_block_101 = 15480363249811881210;
                                }
                                _ => {
                                    current_block_101 = 4983594971376015098;
                                }
                            }
                            match current_block_101 {
                                15480363249811881210 => {
                                    printf(b"}\n\0" as *const u8 as *const libc::c_char);
                                }
                                _ => {}
                            }
                            free(it as *mut libc::c_void);
                        }
                        12 => {
                            rdepth += 1;
                            rdepth;
                            let mut current_block_115: u64;
                            match writecode as libc::c_int {
                                99 => {
                                    if rdepth == 1 as libc::c_int {
                                        printf(
                                            b"\tprintf(\"repeat %%d\\n\", %s);\n\0" as *const u8
                                                as *const libc::c_char,
                                            (*yyvsp.offset(0 as libc::c_int as isize)).s,
                                        );
                                    }
                                    if rdepth == 2 as libc::c_int {
                                        printf(
                                            b"\tprintf(\"repeat %%d\\n\", %s[n]);\n\0" as *const u8
                                                as *const libc::c_char,
                                            (*yyvsp.offset(0 as libc::c_int as isize)).s,
                                        );
                                    }
                                    current_block_115 = 1784308579802614031;
                                }
                                105 | 103 | 111 => {
                                    current_block_115 = 1784308579802614031;
                                }
                                _ => {
                                    current_block_115 = 5636883459695696059;
                                }
                            }
                            match current_block_115 {
                                1784308579802614031 => {
                                    if rdepth == 1 as libc::c_int {
                                        printf(
                                            b"\t{ int n; for (n = 0; n < %s; n++) {\n\0" as *const u8
                                                as *const libc::c_char,
                                            (*yyvsp.offset(0 as libc::c_int as isize)).s,
                                        );
                                    }
                                    if rdepth == 2 as libc::c_int {
                                        printf(
                                            b"\t{ int m; for (m = 0; m < %s[n]; m++) {\n\0" as *const u8
                                                as *const libc::c_char,
                                            (*yyvsp.offset(0 as libc::c_int as isize)).s,
                                        );
                                    }
                                }
                                _ => {}
                            }
                            oldrepeat = repeat;
                            repeat = (*yyvsp.offset(0 as libc::c_int as isize)).s;
                        }
                        13 => {
                            free(repeat as *mut libc::c_void);
                            repeat = oldrepeat;
                            oldrepeat = 0 as *mut libc::c_char;
                            rdepth -= 1;
                            rdepth;
                            match writecode as libc::c_int {
                                105 | 103 | 111 | 99 => {
                                    printf(b"\t}}\n\0" as *const u8 as *const libc::c_char);
                                }
                                _ => {}
                            }
                        }
                        14 => {
                            match writecode as libc::c_int {
                                105 | 103 | 111 | 99 => {
                                    printf(
                                        b"\tif (%s) {\n\0" as *const u8 as *const libc::c_char,
                                        (*yyvsp.offset(0 as libc::c_int as isize)).s,
                                    );
                                }
                                _ => {}
                            }
                            free(
                                (*yyvsp.offset(0 as libc::c_int as isize)).s
                                    as *mut libc::c_void,
                            );
                        }
                        15 => {
                            match writecode as libc::c_int {
                                105 | 103 | 111 | 99 => {
                                    printf(b"\t}\n\0" as *const u8 as *const libc::c_char);
                                }
                                _ => {}
                            }
                        }
                        16 => {
                            name = (*yyvsp.offset(0 as libc::c_int as isize)).s;
                        }
                        17 => {
                            let mut desc: *mut libc::c_char = (*yyvsp
                                .offset(-(8 as libc::c_int) as isize))
                                .s;
                            let mut type_0: *mut libc::c_char = (*yyvsp
                                .offset(-(6 as libc::c_int) as isize))
                                .s;
                            let mut size: libc::c_int = (*yyvsp
                                .offset(-(5 as libc::c_int) as isize))
                                .i;
                            let mut id: *mut libc::c_char = (*yyvsp
                                .offset(-(3 as libc::c_int) as isize))
                                .s;
                            let mut p: *mut libc::c_char = names[rdepth as usize];
                            let mut ptr: *mut libc::c_char = pnames[rdepth as usize];
                            match writecode as libc::c_int {
                                103 => {
                                    if size % 8 as libc::c_int != 0 {
                                        printf(
                                            b"\twriteBITS(ptr->%s%s,raw,&idx,%d);\n\0" as *const u8
                                                as *const libc::c_char,
                                            id,
                                            names[rdepth as usize],
                                            size,
                                        );
                                    } else {
                                        printf(
                                            b"\twrite%s(ptr->%s%s,raw,&idx,%d,ffile);\n\0" as *const u8
                                                as *const libc::c_char,
                                            type_0,
                                            id,
                                            names[rdepth as usize],
                                            size / 8 as libc::c_int,
                                        );
                                    }
                                }
                                105 => {
                                    if rdepth >= 1 as libc::c_int {
                                        printf(
                                            b"if (!ptr->%s) ptr->%s = (%s*)xcalloc(%s, sizeof(ptr->%s[0]));\n\0"
                                                as *const u8 as *const libc::c_char,
                                            id,
                                            id,
                                            type_0,
                                            repeat,
                                            id,
                                        );
                                    }
                                    if rdepth == 2 as libc::c_int {
                                        printf(
                                            b"if (!ptr->%s[n]) ptr->%s[n] = (%s**)xcalloc(%s[n], sizeof(ptr->%s[n][0]));\n\0"
                                                as *const u8 as *const libc::c_char,
                                            id,
                                            id,
                                            type_0,
                                            repeat,
                                            id,
                                        );
                                    }
                                    if size % 8 as libc::c_int != 0 {
                                        printf(
                                            b"\tptr->%s%s = getBITS(raw,&idx, %d,size);\n\0"
                                                as *const u8 as *const libc::c_char,
                                            id,
                                            names[rdepth as usize],
                                            size,
                                        );
                                    } else {
                                        printf(
                                            b"\tptr->%s%s = get%s(raw,&idx, %d,size);\n\0" as *const u8
                                                as *const libc::c_char,
                                            id,
                                            names[rdepth as usize],
                                            type_0,
                                            size / 8 as libc::c_int,
                                        );
                                    }
                                }
                                111 => {
                                    printf(
                                        b"\tput%s(raw,%d,%d,&idx,ptr->%s%s);\n\0" as *const u8
                                            as *const libc::c_char,
                                        type_0,
                                        size / 8 as libc::c_int,
                                        size % 8 as libc::c_int,
                                        id,
                                        names[rdepth as usize],
                                    );
                                }
                                100 => {
                                    if !repeat.is_null() {
                                        printf(
                                            b"\t/* repeat %s */\n\0" as *const u8
                                                as *const libc::c_char,
                                            repeat,
                                        );
                                    }
                                    if *type_0.offset(0 as libc::c_int as isize) as libc::c_int
                                        == 'I' as i32
                                    {
                                        printf(
                                            b"\tint %s%s; \t/* %s */\n\0" as *const u8
                                                as *const libc::c_char,
                                            ptr,
                                            id,
                                            desc,
                                        );
                                    } else if *type_0.offset(0 as libc::c_int as isize)
                                        as libc::c_int == 'C' as i32
                                    {
                                        printf(
                                            b"\tchar %s*%s;\t /* %s */\n\0" as *const u8
                                                as *const libc::c_char,
                                            ptr,
                                            id,
                                            desc,
                                        );
                                    } else {
                                        printf(
                                            b"\tbarray %s%s;\t /* %s */\n\0" as *const u8
                                                as *const libc::c_char,
                                            ptr,
                                            id,
                                            desc,
                                        );
                                    }
                                }
                                99 => {
                                    printf(
                                        b"tabout();\n\0" as *const u8 as *const libc::c_char,
                                    );
                                    printf(
                                        b"\tprintf(\"/*%-30s*/ ptr->%s = \");\n\0" as *const u8
                                            as *const libc::c_char,
                                        desc,
                                        id,
                                    );
                                    if *type_0.offset(0 as libc::c_int as isize) as libc::c_int
                                        == 'I' as i32
                                    {
                                        printf(
                                            b"\tprintf(\"%%d\\n\",ptr->%s%s);\n\0" as *const u8
                                                as *const libc::c_char,
                                            id,
                                            p,
                                        );
                                    } else if *type_0.offset(0 as libc::c_int as isize)
                                        as libc::c_int == 'C' as i32
                                    {
                                        printf(
                                            b"\tprintf(\"%%s\\n\",ptr->%s%s);\n\0" as *const u8
                                                as *const libc::c_char,
                                            id,
                                            p,
                                        );
                                    } else if *type_0.offset(0 as libc::c_int as isize)
                                        as libc::c_int == 'B' as i32
                                    {
                                        printf(
                                            b"\tpbarray(&ptr->%s%s);\n\0" as *const u8
                                                as *const libc::c_char,
                                            id,
                                            p,
                                        );
                                    } else {
                                        abort();
                                    }
                                }
                                _ => {}
                            }
                            free(desc as *mut libc::c_void);
                            free(id as *mut libc::c_void);
                        }
                        18 => {
                            yyval.s = (*yyvsp.offset(0 as libc::c_int as isize)).s;
                        }
                        19 => {
                            yyval
                                .s = b"INT\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char;
                        }
                        20 => {
                            yyval.s = (*yyvsp.offset(-(1 as libc::c_int) as isize)).s;
                        }
                        21 => {
                            yyval
                                .i = (*yyvsp.offset(-(1 as libc::c_int) as isize)).i
                                * (*yyvsp.offset(0 as libc::c_int as isize)).i;
                        }
                        22 => {
                            yyval.s = (*yyvsp.offset(-(1 as libc::c_int) as isize)).s;
                        }
                        23 => {
                            yyval
                                .s = strdup(b"dummy\0" as *const u8 as *const libc::c_char);
                        }
                        27 => {
                            match writecode as libc::c_int {
                                100 => {
                                    printf(
                                        b"#define %s %s\n\0" as *const u8 as *const libc::c_char,
                                        (*yyvsp.offset(-(2 as libc::c_int) as isize)).s,
                                        (*yyvsp.offset(-(1 as libc::c_int) as isize)).s,
                                    );
                                }
                                99 => {
                                    printf(
                                        b"if (ptr->%s%s == %s) { tabout(); printf(\"%s\\n\");}\n\0"
                                            as *const u8 as *const libc::c_char,
                                        name,
                                        names[rdepth as usize],
                                        (*yyvsp.offset(-(1 as libc::c_int) as isize)).s,
                                        (*yyvsp.offset(-(2 as libc::c_int) as isize)).s,
                                    );
                                }
                                _ => {}
                            }
                            free(
                                (*yyvsp.offset(-(2 as libc::c_int) as isize)).s
                                    as *mut libc::c_void,
                            );
                            free(
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).s
                                    as *mut libc::c_void,
                            );
                        }
                        _ => {}
                    }
                    yyvsp = yyvsp.offset(-(yylen as isize));
                    yyssp = yyssp.offset(-(yylen as isize));
                    yylen = 0 as libc::c_int;
                    yyvsp = yyvsp.offset(1);
                    *yyvsp = yyval;
                    let yylhs: libc::c_int = yyr1[yyn as usize] as libc::c_int
                        - 11 as libc::c_int;
                    let yyi: libc::c_int = yypgoto[yylhs as usize] as libc::c_int
                        + *yyssp as libc::c_int;
                    yystate = if 0 as libc::c_int <= yyi && yyi <= 38 as libc::c_int
                        && yycheck[yyi as usize] as libc::c_int == *yyssp as libc::c_int
                    {
                        yytable[yyi as usize] as libc::c_int
                    } else {
                        yydefgoto[yylhs as usize] as libc::c_int
                    };
                }
                _ => {}
            }
            yyssp = yyssp.offset(1);
            yyssp;
        }
    }
    match current_block {
        12565593370437119333 => {
            yyerror(
                b"memory exhausted\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            yyresult = 2 as libc::c_int;
        }
        12607448202022854059 => {
            yyresult = 1 as libc::c_int;
        }
        _ => {}
    }
    if yychar != -(2 as libc::c_int) {
        yytoken = if 0 as libc::c_int <= yychar && yychar <= 263 as libc::c_int {
            yytranslate[yychar as usize] as libc::c_int
        } else {
            2 as libc::c_int
        };
        yydestruct(
            b"Cleanup: discarding lookahead\0" as *const u8 as *const libc::c_char,
            yytoken,
            &mut yylval,
        );
    }
    yyvsp = yyvsp.offset(-(yylen as isize));
    yyssp = yyssp.offset(-(yylen as isize));
    while yyssp != yyss {
        yydestruct(
            b"Cleanup: popping\0" as *const u8 as *const libc::c_char,
            yystos[*yyssp as usize] as libc::c_int,
            yyvsp,
        );
        yyvsp = yyvsp.offset(-(1 as libc::c_int as isize));
        yyssp = yyssp.offset(-(1 as libc::c_int as isize));
    }
    if yyss != yyssa.as_mut_ptr() {
        free(yyss as *mut libc::c_void);
    }
    return yyresult;
}
#[no_mangle]
pub static mut yydebug: libc::c_int = 0;
unsafe fn main_0(mut ac: libc::c_int, mut av: *mut *mut libc::c_char) -> libc::c_int {
    yydebug = 0 as libc::c_int;
    if ac > 1 as libc::c_int {
        writecode = *(*av.offset(1 as libc::c_int as isize))
            .offset(1 as libc::c_int as isize);
    }
    if writecode as libc::c_int == 'd' as i32 {
        printf(
            b"typedef struct { unsigned char *data; int len; } barray; \n\0" as *const u8
                as *const libc::c_char,
        );
        printf(b"typedef  int INT;\n\0" as *const u8 as *const libc::c_char);
        printf(b"typedef  char * CHARS;\n\0" as *const u8 as *const libc::c_char);
    }
    yyparse();
    return 0 as libc::c_int;
}
static mut yytranslate: [yytype_int8; 264] = [
    0 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    9 as libc::c_int as yytype_int8,
    10 as libc::c_int as yytype_int8,
];
static mut yypact: [yytype_int8; 55] = [
    -(14 as libc::c_int) as yytype_int8,
    8 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    -(14 as libc::c_int) as yytype_int8,
    2 as libc::c_int as yytype_int8,
    -(14 as libc::c_int) as yytype_int8,
    4 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    -(14 as libc::c_int) as yytype_int8,
    -(14 as libc::c_int) as yytype_int8,
    6 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    9 as libc::c_int as yytype_int8,
    10 as libc::c_int as yytype_int8,
    11 as libc::c_int as yytype_int8,
    15 as libc::c_int as yytype_int8,
    -(14 as libc::c_int) as yytype_int8,
    -(14 as libc::c_int) as yytype_int8,
    -(14 as libc::c_int) as yytype_int8,
    -(14 as libc::c_int) as yytype_int8,
    -(14 as libc::c_int) as yytype_int8,
    -(14 as libc::c_int) as yytype_int8,
    16 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    -(14 as libc::c_int) as yytype_int8,
    -(14 as libc::c_int) as yytype_int8,
    5 as libc::c_int as yytype_int8,
    17 as libc::c_int as yytype_int8,
    18 as libc::c_int as yytype_int8,
    19 as libc::c_int as yytype_int8,
    20 as libc::c_int as yytype_int8,
    -(14 as libc::c_int) as yytype_int8,
    -(14 as libc::c_int) as yytype_int8,
    -(14 as libc::c_int) as yytype_int8,
    22 as libc::c_int as yytype_int8,
    23 as libc::c_int as yytype_int8,
    -(14 as libc::c_int) as yytype_int8,
    24 as libc::c_int as yytype_int8,
    27 as libc::c_int as yytype_int8,
    -(14 as libc::c_int) as yytype_int8,
    -(14 as libc::c_int) as yytype_int8,
    28 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    -(14 as libc::c_int) as yytype_int8,
    25 as libc::c_int as yytype_int8,
    -(14 as libc::c_int) as yytype_int8,
    29 as libc::c_int as yytype_int8,
    30 as libc::c_int as yytype_int8,
    -(14 as libc::c_int) as yytype_int8,
];
static mut yydefact: [yytype_int8; 55] = [
    2 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    11 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    11 as libc::c_int as yytype_int8,
    11 as libc::c_int as yytype_int8,
    11 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    10 as libc::c_int as yytype_int8,
    9 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    12 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    19 as libc::c_int as yytype_int8,
    11 as libc::c_int as yytype_int8,
    11 as libc::c_int as yytype_int8,
    20 as libc::c_int as yytype_int8,
    18 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    15 as libc::c_int as yytype_int8,
    13 as libc::c_int as yytype_int8,
    21 as libc::c_int as yytype_int8,
    23 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    16 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    24 as libc::c_int as yytype_int8,
    22 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    17 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    25 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    27 as libc::c_int as yytype_int8,
];
static mut yypgoto: [yytype_int8; 19] = [
    -(14 as libc::c_int) as yytype_int8,
    -(14 as libc::c_int) as yytype_int8,
    -(14 as libc::c_int) as yytype_int8,
    32 as libc::c_int as yytype_int8,
    -(14 as libc::c_int) as yytype_int8,
    -(14 as libc::c_int) as yytype_int8,
    -(13 as libc::c_int) as yytype_int8,
    -(14 as libc::c_int) as yytype_int8,
    -(14 as libc::c_int) as yytype_int8,
    -(14 as libc::c_int) as yytype_int8,
    -(14 as libc::c_int) as yytype_int8,
    -(14 as libc::c_int) as yytype_int8,
    -(14 as libc::c_int) as yytype_int8,
    -(14 as libc::c_int) as yytype_int8,
    -(14 as libc::c_int) as yytype_int8,
    -(14 as libc::c_int) as yytype_int8,
    -(14 as libc::c_int) as yytype_int8,
    -(14 as libc::c_int) as yytype_int8,
    -(14 as libc::c_int) as yytype_int8,
];
static mut yydefgoto: [yytype_int8; 19] = [
    -(1 as libc::c_int) as yytype_int8,
    1 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    10 as libc::c_int as yytype_int8,
    12 as libc::c_int as yytype_int8,
    13 as libc::c_int as yytype_int8,
    29 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    28 as libc::c_int as yytype_int8,
    15 as libc::c_int as yytype_int8,
    44 as libc::c_int as yytype_int8,
    32 as libc::c_int as yytype_int8,
    19 as libc::c_int as yytype_int8,
    36 as libc::c_int as yytype_int8,
    42 as libc::c_int as yytype_int8,
    47 as libc::c_int as yytype_int8,
    48 as libc::c_int as yytype_int8,
];
static mut yytable: [yytype_int8; 39] = [
    21 as libc::c_int as yytype_int8,
    22 as libc::c_int as yytype_int8,
    23 as libc::c_int as yytype_int8,
    16 as libc::c_int as yytype_int8,
    17 as libc::c_int as yytype_int8,
    18 as libc::c_int as yytype_int8,
    50 as libc::c_int as yytype_int8,
    51 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    11 as libc::c_int as yytype_int8,
    9 as libc::c_int as yytype_int8,
    20 as libc::c_int as yytype_int8,
    35 as libc::c_int as yytype_int8,
    33 as libc::c_int as yytype_int8,
    34 as libc::c_int as yytype_int8,
    24 as libc::c_int as yytype_int8,
    25 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    27 as libc::c_int as yytype_int8,
    31 as libc::c_int as yytype_int8,
    30 as libc::c_int as yytype_int8,
    37 as libc::c_int as yytype_int8,
    38 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    40 as libc::c_int as yytype_int8,
    41 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    39 as libc::c_int as yytype_int8,
    45 as libc::c_int as yytype_int8,
    43 as libc::c_int as yytype_int8,
    46 as libc::c_int as yytype_int8,
    52 as libc::c_int as yytype_int8,
    49 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    54 as libc::c_int as yytype_int8,
    53 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
];
static mut yycheck: [yytype_int8; 39] = [
    13 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    15 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    9 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    9 as libc::c_int as yytype_int8,
    28 as libc::c_int as yytype_int8,
    29 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    6 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    10 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    6 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
];
static mut yystos: [yytype_int8; 55] = [
    0 as libc::c_int as yytype_int8,
    12 as libc::c_int as yytype_int8,
    13 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    15 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    9 as libc::c_int as yytype_int8,
    16 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    17 as libc::c_int as yytype_int8,
    18 as libc::c_int as yytype_int8,
    20 as libc::c_int as yytype_int8,
    22 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    25 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    17 as libc::c_int as yytype_int8,
    17 as libc::c_int as yytype_int8,
    17 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    21 as libc::c_int as yytype_int8,
    19 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    24 as libc::c_int as yytype_int8,
    17 as libc::c_int as yytype_int8,
    17 as libc::c_int as yytype_int8,
    9 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    10 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    27 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    23 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    28 as libc::c_int as yytype_int8,
    29 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
];
static mut yyr1: [yytype_int8; 28] = [
    0 as libc::c_int as yytype_int8,
    11 as libc::c_int as yytype_int8,
    13 as libc::c_int as yytype_int8,
    12 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    16 as libc::c_int as yytype_int8,
    15 as libc::c_int as yytype_int8,
    17 as libc::c_int as yytype_int8,
    17 as libc::c_int as yytype_int8,
    17 as libc::c_int as yytype_int8,
    17 as libc::c_int as yytype_int8,
    19 as libc::c_int as yytype_int8,
    18 as libc::c_int as yytype_int8,
    21 as libc::c_int as yytype_int8,
    20 as libc::c_int as yytype_int8,
    23 as libc::c_int as yytype_int8,
    22 as libc::c_int as yytype_int8,
    24 as libc::c_int as yytype_int8,
    24 as libc::c_int as yytype_int8,
    25 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    27 as libc::c_int as yytype_int8,
    27 as libc::c_int as yytype_int8,
    28 as libc::c_int as yytype_int8,
    28 as libc::c_int as yytype_int8,
    29 as libc::c_int as yytype_int8,
    29 as libc::c_int as yytype_int8,
];
static mut yyr2: [yytype_int8; 28] = [
    0 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    10 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
];
unsafe extern "C" fn yydestruct(
    mut yymsg: *const libc::c_char,
    mut yytype: libc::c_int,
    mut yyvaluep: *mut YYSTYPE,
) {
    if yymsg.is_null() {
        yymsg = b"Deleting\0" as *const u8 as *const libc::c_char;
    }
}
#[no_mangle]
pub static mut yychar: libc::c_int = 0;
#[no_mangle]
pub static mut yynerrs: libc::c_int = 0;
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
