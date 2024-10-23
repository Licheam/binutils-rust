extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn prompt();
    fn ar_clear();
    fn ar_replace(_: *mut list);
    fn ar_delete(_: *mut list);
    fn ar_save();
    fn ar_list();
    fn ar_open(_: *mut libc::c_char, _: libc::c_int);
    fn ar_directory(_: *mut libc::c_char, _: *mut list, _: *mut libc::c_char);
    fn ar_addmod(_: *mut list);
    fn ar_addlib(_: *mut libc::c_char, _: *mut list);
    fn ar_end();
    fn ar_extract(_: *mut list);
    fn yylex() -> libc::c_int;
    static mut verbose: libc::c_int;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct list {
    pub name: *mut libc::c_char,
    pub next: *mut list,
}
pub type yy_state_t = yytype_int8;
pub type yytype_int8 = libc::c_schar;
#[derive(Copy, Clone)]
#[repr(C)]
pub union YYSTYPE {
    pub name: *mut libc::c_char,
    pub list: *mut list,
}
pub type yy_state_fast_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub union yyalloc {
    pub yyss_alloc: yy_state_t,
    pub yyvs_alloc: YYSTYPE,
}
#[no_mangle]
pub unsafe extern "C" fn yyparse() -> libc::c_int {
    let mut current_block: u64;
    let mut yystate: yy_state_fast_t = 0;
    let mut yyerrstatus: libc::c_int = 0;
    let mut yyssa: [yy_state_t; 200] = [0; 200];
    let mut yyss: *mut yy_state_t = 0 as *mut yy_state_t;
    let mut yyssp: *mut yy_state_t = 0 as *mut yy_state_t;
    let mut yyvsa: [YYSTYPE; 200] = [YYSTYPE {
        name: 0 as *mut libc::c_char,
    }; 200];
    let mut yyvs: *mut YYSTYPE = 0 as *mut YYSTYPE;
    let mut yyvsp: *mut YYSTYPE = 0 as *mut YYSTYPE;
    let mut yystacksize: libc::c_long = 0;
    let mut yyn: libc::c_int = 0;
    let mut yyresult: libc::c_int = 0;
    let mut yytoken: libc::c_int = 0 as libc::c_int;
    let mut yyval: YYSTYPE = YYSTYPE {
        name: 0 as *mut libc::c_char,
    };
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
            && (0 as libc::c_int <= yystate && yystate < 53 as libc::c_int))
            as libc::c_int;
        *yyssp = yystate as yy_state_t;
        if yyss.offset(yystacksize as isize).offset(-(1 as libc::c_int as isize))
            <= yyssp
        {
            let mut yysize: libc::c_long = yyssp.offset_from(yyss) as libc::c_long
                + 1 as libc::c_int as libc::c_long;
            if 10000 as libc::c_int as libc::c_long <= yystacksize {
                current_block = 5362389748661360472;
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
                current_block = 5362389748661360472;
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
                current_block = 17407811139759298593;
                break;
            }
        }
        if yystate == 3 as libc::c_int {
            yyresult = 0 as libc::c_int;
            current_block = 8078507066274755648;
            break;
        } else {
            yyn = yypact[yystate as usize] as libc::c_int;
            if yyn == -(14 as libc::c_int) {
                current_block = 3948188629359026727;
            } else {
                if yychar == -(2 as libc::c_int) {
                    yychar = yylex();
                }
                if yychar <= 0 as libc::c_int {
                    yytoken = 0 as libc::c_int;
                    yychar = yytoken;
                } else {
                    yytoken = if 0 as libc::c_int <= yychar
                        && yychar <= 275 as libc::c_int
                    {
                        yytranslate[yychar as usize] as libc::c_int
                    } else {
                        2 as libc::c_int
                    };
                }
                yyn += yytoken;
                if yyn < 0 as libc::c_int || (34 as libc::c_int) < yyn
                    || yycheck[yyn as usize] as libc::c_int != yytoken
                {
                    current_block = 3948188629359026727;
                } else {
                    yyn = yytable[yyn as usize] as libc::c_int;
                    if yyn <= 0 as libc::c_int {
                        yyn = -yyn;
                        current_block = 5852485875983108082;
                    } else {
                        if yyerrstatus != 0 {
                            yyerrstatus -= 1;
                            yyerrstatus;
                        }
                        yystate = yyn;
                        yyvsp = yyvsp.offset(1);
                        *yyvsp = yylval;
                        yychar = -(2 as libc::c_int);
                        current_block = 15415490723644612855;
                    }
                }
            }
            match current_block {
                3948188629359026727 => {
                    yyn = yydefact[yystate as usize] as libc::c_int;
                    if yyn == 0 as libc::c_int {
                        yytoken = if yychar == -(2 as libc::c_int) {
                            -(2 as libc::c_int)
                        } else if 0 as libc::c_int <= yychar
                            && yychar <= 275 as libc::c_int
                        {
                            yytranslate[yychar as usize] as libc::c_int
                        } else {
                            2 as libc::c_int
                        };
                        if yyerrstatus == 0 {
                            yynerrs += 1;
                            yynerrs;
                            yyerror(
                                b"syntax error\0" as *const u8 as *const libc::c_char,
                            );
                        }
                        if yyerrstatus == 3 as libc::c_int {
                            if yychar <= 0 as libc::c_int {
                                if yychar == 0 as libc::c_int {
                                    current_block = 17407811139759298593;
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
                                if 0 as libc::c_int <= yyn && yyn <= 34 as libc::c_int
                                    && yycheck[yyn as usize] as libc::c_int == 1 as libc::c_int
                                {
                                    yyn = yytable[yyn as usize] as libc::c_int;
                                    if (0 as libc::c_int) < yyn {
                                        break;
                                    }
                                }
                            }
                            if yyssp == yyss {
                                current_block = 17407811139759298593;
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
                        current_block = 15415490723644612855;
                    } else {
                        current_block = 5852485875983108082;
                    }
                }
                _ => {}
            }
            match current_block {
                5852485875983108082 => {
                    yylen = yyr2[yyn as usize] as libc::c_int;
                    yyval = *yyvsp.offset((1 as libc::c_int - yylen) as isize);
                    match yyn {
                        2 => {
                            prompt();
                        }
                        6 => {
                            prompt();
                        }
                        19 => {
                            ar_end();
                            return 0 as libc::c_int;
                        }
                        21 => {
                            yyerror(b"foo\0" as *const u8 as *const libc::c_char);
                        }
                        23 => {
                            ar_extract((*yyvsp.offset(0 as libc::c_int as isize)).list);
                        }
                        24 => {
                            ar_replace((*yyvsp.offset(0 as libc::c_int as isize)).list);
                        }
                        25 => {
                            ar_clear();
                        }
                        26 => {
                            ar_delete((*yyvsp.offset(0 as libc::c_int as isize)).list);
                        }
                        27 => {
                            ar_addmod((*yyvsp.offset(0 as libc::c_int as isize)).list);
                        }
                        28 => {
                            ar_list();
                        }
                        29 => {
                            ar_save();
                        }
                        30 => {
                            ar_open(
                                (*yyvsp.offset(0 as libc::c_int as isize)).name,
                                0 as libc::c_int,
                            );
                        }
                        31 => {
                            ar_open(
                                (*yyvsp.offset(0 as libc::c_int as isize)).name,
                                1 as libc::c_int,
                            );
                        }
                        32 => {
                            ar_addlib(
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).name,
                                (*yyvsp.offset(0 as libc::c_int as isize)).list,
                            );
                        }
                        33 => {
                            ar_directory(
                                (*yyvsp.offset(-(2 as libc::c_int) as isize)).name,
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).list,
                                (*yyvsp.offset(0 as libc::c_int as isize)).name,
                            );
                        }
                        34 => {
                            yyval.name = (*yyvsp.offset(0 as libc::c_int as isize)).name;
                        }
                        35 => {
                            yyval.name = 0 as *mut libc::c_char;
                        }
                        36 => {
                            yyval
                                .list = (*yyvsp.offset(-(1 as libc::c_int) as isize)).list;
                        }
                        37 => {
                            yyval.list = 0 as *mut list;
                        }
                        38 => {
                            let mut n: *mut list = malloc(
                                ::core::mem::size_of::<list>() as libc::c_ulong,
                            ) as *mut list;
                            (*n)
                                .next = (*yyvsp.offset(-(2 as libc::c_int) as isize)).list;
                            (*n).name = (*yyvsp.offset(0 as libc::c_int as isize)).name;
                            yyval.list = n;
                        }
                        39 => {
                            yyval.list = 0 as *mut list;
                        }
                        42 => {
                            verbose = (verbose == 0) as libc::c_int;
                        }
                        _ => {}
                    }
                    yyvsp = yyvsp.offset(-(yylen as isize));
                    yyssp = yyssp.offset(-(yylen as isize));
                    yylen = 0 as libc::c_int;
                    yyvsp = yyvsp.offset(1);
                    *yyvsp = yyval;
                    let yylhs: libc::c_int = yyr1[yyn as usize] as libc::c_int
                        - 24 as libc::c_int;
                    let yyi: libc::c_int = yypgoto[yylhs as usize] as libc::c_int
                        + *yyssp as libc::c_int;
                    yystate = if 0 as libc::c_int <= yyi && yyi <= 34 as libc::c_int
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
        5362389748661360472 => {
            yyerror(b"memory exhausted\0" as *const u8 as *const libc::c_char);
            yyresult = 2 as libc::c_int;
        }
        17407811139759298593 => {
            yyresult = 1 as libc::c_int;
        }
        _ => {}
    }
    if yychar != -(2 as libc::c_int) {
        yytoken = if 0 as libc::c_int <= yychar && yychar <= 275 as libc::c_int {
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
unsafe extern "C" fn yyerror(mut _x: *const libc::c_char) -> libc::c_int {
    extern "C" {
        static mut linenumber: libc::c_int;
    }
    printf(
        dcgettext(
            0 as *const libc::c_char,
            b"Syntax error in archive script, line %d\n\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
        linenumber + 1 as libc::c_int,
    );
    return 0 as libc::c_int;
}
#[no_mangle]
pub static mut yylval: YYSTYPE = YYSTYPE {
    name: 0 as *mut libc::c_char,
};
static mut yytranslate: [yytype_int8; 276] = [
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
    21 as libc::c_int as yytype_int8,
    22 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    23 as libc::c_int as yytype_int8,
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
    5 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    9 as libc::c_int as yytype_int8,
    10 as libc::c_int as yytype_int8,
    11 as libc::c_int as yytype_int8,
    12 as libc::c_int as yytype_int8,
    13 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    15 as libc::c_int as yytype_int8,
    16 as libc::c_int as yytype_int8,
    17 as libc::c_int as yytype_int8,
    18 as libc::c_int as yytype_int8,
    19 as libc::c_int as yytype_int8,
    20 as libc::c_int as yytype_int8,
];
static mut yypact: [yytype_int8; 53] = [
    -(14 as libc::c_int) as yytype_int8,
    1 as libc::c_int as yytype_int8,
    -(14 as libc::c_int) as yytype_int8,
    -(14 as libc::c_int) as yytype_int8,
    5 as libc::c_int as yytype_int8,
    -(14 as libc::c_int) as yytype_int8,
    -(14 as libc::c_int) as yytype_int8,
    -(14 as libc::c_int) as yytype_int8,
    2 as libc::c_int as yytype_int8,
    -(14 as libc::c_int) as yytype_int8,
    -(14 as libc::c_int) as yytype_int8,
    -(14 as libc::c_int) as yytype_int8,
    21 as libc::c_int as yytype_int8,
    -(14 as libc::c_int) as yytype_int8,
    22 as libc::c_int as yytype_int8,
    -(14 as libc::c_int) as yytype_int8,
    -(14 as libc::c_int) as yytype_int8,
    -(14 as libc::c_int) as yytype_int8,
    -(14 as libc::c_int) as yytype_int8,
    23 as libc::c_int as yytype_int8,
    -(14 as libc::c_int) as yytype_int8,
    26 as libc::c_int as yytype_int8,
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
    10 as libc::c_int as yytype_int8,
    -(3 as libc::c_int) as yytype_int8,
    -(14 as libc::c_int) as yytype_int8,
    -(3 as libc::c_int) as yytype_int8,
    10 as libc::c_int as yytype_int8,
    -(3 as libc::c_int) as yytype_int8,
    -(3 as libc::c_int) as yytype_int8,
    -(14 as libc::c_int) as yytype_int8,
    -(14 as libc::c_int) as yytype_int8,
    -(14 as libc::c_int) as yytype_int8,
    -(14 as libc::c_int) as yytype_int8,
    -(14 as libc::c_int) as yytype_int8,
    27 as libc::c_int as yytype_int8,
    28 as libc::c_int as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    -(14 as libc::c_int) as yytype_int8,
    -(14 as libc::c_int) as yytype_int8,
    -(14 as libc::c_int) as yytype_int8,
    -(14 as libc::c_int) as yytype_int8,
];
static mut yydefact: [yytype_int8; 53] = [
    2 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    20 as libc::c_int as yytype_int8,
    42 as libc::c_int as yytype_int8,
    21 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    28 as libc::c_int as yytype_int8,
    39 as libc::c_int as yytype_int8,
    25 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    39 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    19 as libc::c_int as yytype_int8,
    39 as libc::c_int as yytype_int8,
    39 as libc::c_int as yytype_int8,
    29 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    15 as libc::c_int as yytype_int8,
    16 as libc::c_int as yytype_int8,
    12 as libc::c_int as yytype_int8,
    17 as libc::c_int as yytype_int8,
    13 as libc::c_int as yytype_int8,
    18 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    11 as libc::c_int as yytype_int8,
    10 as libc::c_int as yytype_int8,
    9 as libc::c_int as yytype_int8,
    37 as libc::c_int as yytype_int8,
    27 as libc::c_int as yytype_int8,
    31 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    37 as libc::c_int as yytype_int8,
    23 as libc::c_int as yytype_int8,
    24 as libc::c_int as yytype_int8,
    30 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    39 as libc::c_int as yytype_int8,
    32 as libc::c_int as yytype_int8,
    40 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    35 as libc::c_int as yytype_int8,
    41 as libc::c_int as yytype_int8,
    38 as libc::c_int as yytype_int8,
    34 as libc::c_int as yytype_int8,
    33 as libc::c_int as yytype_int8,
    36 as libc::c_int as yytype_int8,
];
static mut yypgoto: [yytype_int8; 22] = [
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
    -(14 as libc::c_int) as yytype_int8,
    -(14 as libc::c_int) as yytype_int8,
    -(14 as libc::c_int) as yytype_int8,
    -(14 as libc::c_int) as yytype_int8,
    -(14 as libc::c_int) as yytype_int8,
    -(14 as libc::c_int) as yytype_int8,
    -(4 as libc::c_int) as yytype_int8,
    -(13 as libc::c_int) as yytype_int8,
    -(14 as libc::c_int) as yytype_int8,
    -(14 as libc::c_int) as yytype_int8,
];
static mut yydefgoto: [yytype_int8; 22] = [
    -(1 as libc::c_int) as yytype_int8,
    1 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    20 as libc::c_int as yytype_int8,
    21 as libc::c_int as yytype_int8,
    22 as libc::c_int as yytype_int8,
    23 as libc::c_int as yytype_int8,
    24 as libc::c_int as yytype_int8,
    25 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    27 as libc::c_int as yytype_int8,
    28 as libc::c_int as yytype_int8,
    29 as libc::c_int as yytype_int8,
    30 as libc::c_int as yytype_int8,
    31 as libc::c_int as yytype_int8,
    32 as libc::c_int as yytype_int8,
    51 as libc::c_int as yytype_int8,
    44 as libc::c_int as yytype_int8,
    35 as libc::c_int as yytype_int8,
    46 as libc::c_int as yytype_int8,
    33 as libc::c_int as yytype_int8,
];
static mut yytable: [yytype_int8; 35] = [
    37 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    -(41 as libc::c_int) as yytype_int8,
    39 as libc::c_int as yytype_int8,
    40 as libc::c_int as yytype_int8,
    -(3 as libc::c_int) as yytype_int8,
    5 as libc::c_int as yytype_int8,
    34 as libc::c_int as yytype_int8,
    -(22 as libc::c_int) as yytype_int8,
    6 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    9 as libc::c_int as yytype_int8,
    10 as libc::c_int as yytype_int8,
    11 as libc::c_int as yytype_int8,
    12 as libc::c_int as yytype_int8,
    13 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    15 as libc::c_int as yytype_int8,
    16 as libc::c_int as yytype_int8,
    45 as libc::c_int as yytype_int8,
    52 as libc::c_int as yytype_int8,
    45 as libc::c_int as yytype_int8,
    17 as libc::c_int as yytype_int8,
    18 as libc::c_int as yytype_int8,
    19 as libc::c_int as yytype_int8,
    36 as libc::c_int as yytype_int8,
    38 as libc::c_int as yytype_int8,
    41 as libc::c_int as yytype_int8,
    42 as libc::c_int as yytype_int8,
    48 as libc::c_int as yytype_int8,
    43 as libc::c_int as yytype_int8,
    49 as libc::c_int as yytype_int8,
    50 as libc::c_int as yytype_int8,
    47 as libc::c_int as yytype_int8,
];
static mut yycheck: [yytype_int8; 35] = [
    13 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    16 as libc::c_int as yytype_int8,
    17 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    9 as libc::c_int as yytype_int8,
    10 as libc::c_int as yytype_int8,
    11 as libc::c_int as yytype_int8,
    12 as libc::c_int as yytype_int8,
    13 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    23 as libc::c_int as yytype_int8,
    22 as libc::c_int as yytype_int8,
    23 as libc::c_int as yytype_int8,
    18 as libc::c_int as yytype_int8,
    19 as libc::c_int as yytype_int8,
    20 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    43 as libc::c_int as yytype_int8,
    21 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    38 as libc::c_int as yytype_int8,
];
static mut yystos: [yytype_int8; 53] = [
    0 as libc::c_int as yytype_int8,
    25 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    27 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    9 as libc::c_int as yytype_int8,
    10 as libc::c_int as yytype_int8,
    11 as libc::c_int as yytype_int8,
    12 as libc::c_int as yytype_int8,
    13 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    18 as libc::c_int as yytype_int8,
    19 as libc::c_int as yytype_int8,
    20 as libc::c_int as yytype_int8,
    28 as libc::c_int as yytype_int8,
    29 as libc::c_int as yytype_int8,
    30 as libc::c_int as yytype_int8,
    31 as libc::c_int as yytype_int8,
    32 as libc::c_int as yytype_int8,
    33 as libc::c_int as yytype_int8,
    34 as libc::c_int as yytype_int8,
    35 as libc::c_int as yytype_int8,
    36 as libc::c_int as yytype_int8,
    37 as libc::c_int as yytype_int8,
    38 as libc::c_int as yytype_int8,
    39 as libc::c_int as yytype_int8,
    40 as libc::c_int as yytype_int8,
    45 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    43 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    43 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    43 as libc::c_int as yytype_int8,
    43 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    21 as libc::c_int as yytype_int8,
    42 as libc::c_int as yytype_int8,
    23 as libc::c_int as yytype_int8,
    44 as libc::c_int as yytype_int8,
    42 as libc::c_int as yytype_int8,
    43 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    41 as libc::c_int as yytype_int8,
    22 as libc::c_int as yytype_int8,
];
static mut yyr1: [yytype_int8; 43] = [
    0 as libc::c_int as yytype_int8,
    24 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    25 as libc::c_int as yytype_int8,
    27 as libc::c_int as yytype_int8,
    27 as libc::c_int as yytype_int8,
    28 as libc::c_int as yytype_int8,
    29 as libc::c_int as yytype_int8,
    29 as libc::c_int as yytype_int8,
    29 as libc::c_int as yytype_int8,
    29 as libc::c_int as yytype_int8,
    29 as libc::c_int as yytype_int8,
    29 as libc::c_int as yytype_int8,
    29 as libc::c_int as yytype_int8,
    29 as libc::c_int as yytype_int8,
    29 as libc::c_int as yytype_int8,
    29 as libc::c_int as yytype_int8,
    29 as libc::c_int as yytype_int8,
    29 as libc::c_int as yytype_int8,
    29 as libc::c_int as yytype_int8,
    29 as libc::c_int as yytype_int8,
    29 as libc::c_int as yytype_int8,
    29 as libc::c_int as yytype_int8,
    30 as libc::c_int as yytype_int8,
    31 as libc::c_int as yytype_int8,
    32 as libc::c_int as yytype_int8,
    33 as libc::c_int as yytype_int8,
    34 as libc::c_int as yytype_int8,
    35 as libc::c_int as yytype_int8,
    36 as libc::c_int as yytype_int8,
    37 as libc::c_int as yytype_int8,
    38 as libc::c_int as yytype_int8,
    39 as libc::c_int as yytype_int8,
    40 as libc::c_int as yytype_int8,
    41 as libc::c_int as yytype_int8,
    41 as libc::c_int as yytype_int8,
    42 as libc::c_int as yytype_int8,
    42 as libc::c_int as yytype_int8,
    43 as libc::c_int as yytype_int8,
    43 as libc::c_int as yytype_int8,
    44 as libc::c_int as yytype_int8,
    44 as libc::c_int as yytype_int8,
    45 as libc::c_int as yytype_int8,
];
static mut yyr2: [yytype_int8; 43] = [
    0 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
];
unsafe extern "C" fn yydestruct(
    mut yymsg: *const libc::c_char,
    mut _yytype: libc::c_int,
    mut _yyvaluep: *mut YYSTYPE,
) {
    if yymsg.is_null() {
        yymsg = b"Deleting\0" as *const u8 as *const libc::c_char;
    }
}
#[no_mangle]
pub static mut yychar: libc::c_int = 0;
#[no_mangle]
pub static mut yynerrs: libc::c_int = 0;
