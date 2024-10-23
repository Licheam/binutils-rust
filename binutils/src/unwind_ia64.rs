extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    static mut stdout: *mut FILE;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
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
pub type unw_decoder = Option::<
    unsafe extern "C" fn(
        *const libc::c_uchar,
        libc::c_uint,
        *mut libc::c_void,
        *const libc::c_uchar,
    ) -> *const libc::c_uchar,
>;
pub type unw_word = bfd_vma;
pub type bfd_vma = libc::c_ulong;
#[inline]
unsafe extern "C" fn putchar(mut __c: libc::c_int) -> libc::c_int {
    return putc(__c, stdout);
}
#[no_mangle]
pub unsafe extern "C" fn unw_decode(
    mut dp: *const libc::c_uchar,
    mut inside_body: libc::c_int,
    mut ptr_inside_body: *mut libc::c_void,
    mut end: *const libc::c_uchar,
) -> *const libc::c_uchar {
    let mut decoder: unw_decoder = None;
    let mut code: libc::c_uchar = 0;
    if (end.offset_from(dp) as libc::c_long) < 1 as libc::c_int as libc::c_long {
        printf(
            dcgettext(
                0 as *const libc::c_char,
                b"\t<corrupt IA64 descriptor>\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return end;
    }
    let fresh0 = dp;
    dp = dp.offset(1);
    code = *fresh0;
    decoder = unw_decode_table[inside_body
        as usize][(code as libc::c_int >> 5 as libc::c_int) as usize];
    return (Some(decoder.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(dp, code as libc::c_uint, ptr_inside_body, end);
}
static mut unw_rlen: bfd_vma = 0 as libc::c_int as bfd_vma;
unsafe extern "C" fn unw_print_brmask(
    mut cp: *mut libc::c_char,
    mut mask: libc::c_uint,
) {
    let mut sep: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while mask != 0 && i < 5 as libc::c_int {
        if mask & 1 as libc::c_int as libc::c_uint != 0 {
            if sep != 0 {
                let fresh1 = cp;
                cp = cp.offset(1);
                *fresh1 = ',' as i32 as libc::c_char;
            }
            let fresh2 = cp;
            cp = cp.offset(1);
            *fresh2 = 'b' as i32 as libc::c_char;
            let fresh3 = cp;
            cp = cp.offset(1);
            *fresh3 = (i + 1 as libc::c_int + '0' as i32) as libc::c_char;
            sep = 1 as libc::c_int;
        }
        mask >>= 1 as libc::c_int;
        i += 1;
        i;
    }
    *cp = '\0' as i32 as libc::c_char;
}
unsafe extern "C" fn unw_print_grmask(
    mut cp: *mut libc::c_char,
    mut mask: libc::c_uint,
) {
    let mut sep: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        if mask & 1 as libc::c_int as libc::c_uint != 0 {
            if sep != 0 {
                let fresh4 = cp;
                cp = cp.offset(1);
                *fresh4 = ',' as i32 as libc::c_char;
            }
            let fresh5 = cp;
            cp = cp.offset(1);
            *fresh5 = 'r' as i32 as libc::c_char;
            let fresh6 = cp;
            cp = cp.offset(1);
            *fresh6 = (i + 4 as libc::c_int + '0' as i32) as libc::c_char;
            sep = 1 as libc::c_int;
        }
        mask >>= 1 as libc::c_int;
        i += 1;
        i;
    }
    *cp = '\0' as i32 as libc::c_char;
}
unsafe extern "C" fn unw_print_frmask(
    mut cp: *mut libc::c_char,
    mut mask: libc::c_uint,
) {
    let mut sep: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 20 as libc::c_int {
        if mask & 1 as libc::c_int as libc::c_uint != 0 {
            if sep != 0 {
                let fresh7 = cp;
                cp = cp.offset(1);
                *fresh7 = ',' as i32 as libc::c_char;
            }
            let fresh8 = cp;
            cp = cp.offset(1);
            *fresh8 = 'f' as i32 as libc::c_char;
            if i < 4 as libc::c_int {
                let fresh9 = cp;
                cp = cp.offset(1);
                *fresh9 = (i + 2 as libc::c_int + '0' as i32) as libc::c_char;
            } else {
                let fresh10 = cp;
                cp = cp.offset(1);
                *fresh10 = ((i + 2 as libc::c_int) / 10 as libc::c_int + 1 as libc::c_int
                    + '0' as i32) as libc::c_char;
                let fresh11 = cp;
                cp = cp.offset(1);
                *fresh11 = ((i + 2 as libc::c_int) % 10 as libc::c_int + '0' as i32)
                    as libc::c_char;
            }
            sep = 1 as libc::c_int;
        }
        mask >>= 1 as libc::c_int;
        i += 1;
        i;
    }
    *cp = '\0' as i32 as libc::c_char;
}
unsafe extern "C" fn unw_print_abreg(
    mut cp: *mut libc::c_char,
    mut abreg: libc::c_uint,
) {
    static mut special_reg: [*const libc::c_char; 16] = [
        b"pr\0" as *const u8 as *const libc::c_char,
        b"psp\0" as *const u8 as *const libc::c_char,
        b"@priunat\0" as *const u8 as *const libc::c_char,
        b"rp\0" as *const u8 as *const libc::c_char,
        b"ar.bsp\0" as *const u8 as *const libc::c_char,
        b"ar.bspstore\0" as *const u8 as *const libc::c_char,
        b"ar.rnat\0" as *const u8 as *const libc::c_char,
        b"ar.unat\0" as *const u8 as *const libc::c_char,
        b"ar.fpsr\0" as *const u8 as *const libc::c_char,
        b"ar.pfs\0" as *const u8 as *const libc::c_char,
        b"ar.lc\0" as *const u8 as *const libc::c_char,
        b"Unknown11\0" as *const u8 as *const libc::c_char,
        b"Unknown12\0" as *const u8 as *const libc::c_char,
        b"Unknown13\0" as *const u8 as *const libc::c_char,
        b"Unknown14\0" as *const u8 as *const libc::c_char,
        b"Unknown15\0" as *const u8 as *const libc::c_char,
    ];
    match abreg >> 5 as libc::c_int & 0x3 as libc::c_int as libc::c_uint {
        0 => {
            sprintf(
                cp,
                b"r%u\0" as *const u8 as *const libc::c_char,
                abreg & 0x1f as libc::c_int as libc::c_uint,
            );
        }
        1 => {
            sprintf(
                cp,
                b"f%u\0" as *const u8 as *const libc::c_char,
                abreg & 0x1f as libc::c_int as libc::c_uint,
            );
        }
        2 => {
            sprintf(
                cp,
                b"b%u\0" as *const u8 as *const libc::c_char,
                abreg & 0x1f as libc::c_int as libc::c_uint,
            );
        }
        3 => {
            strcpy(
                cp,
                special_reg[(abreg & 0xf as libc::c_int as libc::c_uint) as usize],
            );
        }
        _ => {}
    };
}
unsafe extern "C" fn unw_print_xyreg(
    mut cp: *mut libc::c_char,
    mut x: libc::c_uint,
    mut ytreg: libc::c_uint,
) {
    match x << 1 as libc::c_int
        | ytreg >> 7 as libc::c_int & 1 as libc::c_int as libc::c_uint
    {
        0 => {
            sprintf(
                cp,
                b"r%u\0" as *const u8 as *const libc::c_char,
                ytreg & 0x1f as libc::c_int as libc::c_uint,
            );
        }
        1 => {
            sprintf(
                cp,
                b"f%u\0" as *const u8 as *const libc::c_char,
                ytreg & 0x1f as libc::c_int as libc::c_uint,
            );
        }
        2 => {
            sprintf(
                cp,
                b"b%u\0" as *const u8 as *const libc::c_char,
                ytreg & 0x1f as libc::c_int as libc::c_uint,
            );
        }
        _ => {
            strcpy(cp, b"invalid\0" as *const u8 as *const libc::c_char);
        }
    };
}
unsafe extern "C" fn unw_decode_uleb128(
    mut dpp: *mut *const libc::c_uchar,
    mut end: *const libc::c_uchar,
) -> unw_word {
    let mut shift: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut status: libc::c_int = 1 as libc::c_int;
    let mut byte: unw_word = 0;
    let mut result: unw_word = 0 as libc::c_int as unw_word;
    let mut bp: *const libc::c_uchar = *dpp;
    while bp < end {
        let fresh12 = bp;
        bp = bp.offset(1);
        byte = *fresh12 as unw_word;
        if (shift as libc::c_ulong)
            < (::core::mem::size_of::<unw_word>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
        {
            result |= (byte & 0x7f as libc::c_int as libc::c_ulong) << shift;
            if result >> shift != byte & 0x7f as libc::c_int as libc::c_ulong {
                status |= 2 as libc::c_int;
            }
            shift = shift.wrapping_add(7 as libc::c_int as libc::c_uint);
        } else if byte & 0x7f as libc::c_int as libc::c_ulong
            != 0 as libc::c_int as libc::c_ulong
        {
            status |= 2 as libc::c_int;
        }
        if !(byte & 0x80 as libc::c_int as libc::c_ulong
            == 0 as libc::c_int as libc::c_ulong)
        {
            continue;
        }
        status &= !(1 as libc::c_int);
        break;
    }
    *dpp = bp;
    if status != 0 as libc::c_int {
        printf(
            dcgettext(
                0 as *const libc::c_char,
                b"Bad uleb128\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    return result;
}
unsafe extern "C" fn unw_decode_x1(
    mut dp: *const libc::c_uchar,
    mut code: libc::c_uint,
    mut arg: *mut libc::c_void,
    mut end: *const libc::c_uchar,
) -> *const libc::c_uchar {
    let mut byte1: libc::c_uchar = 0;
    let mut abreg: libc::c_uchar = 0;
    let mut t: unw_word = 0;
    let mut off: unw_word = 0;
    if (end.offset_from(dp) as libc::c_long) < 3 as libc::c_int as libc::c_long {
        printf(
            dcgettext(
                0 as *const libc::c_char,
                b"\t<corrupt X1>\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return end;
    }
    let fresh13 = dp;
    dp = dp.offset(1);
    byte1 = *fresh13;
    t = unw_decode_uleb128(&mut dp, end);
    off = unw_decode_uleb128(&mut dp, end);
    abreg = (byte1 as libc::c_int & 0x7f as libc::c_int) as libc::c_uchar;
    if byte1 as libc::c_int & 0x80 as libc::c_int != 0 {
        let mut regname: [libc::c_char; 20] = [0; 20];
        unw_print_abreg(regname.as_mut_ptr(), abreg as libc::c_uint);
        printf(
            b"\t%s:spill_sprel(reg=%s,t=%lu,spoff=0x%lx)\n\0" as *const u8
                as *const libc::c_char,
            b"X1\0" as *const u8 as *const libc::c_char,
            regname.as_mut_ptr(),
            t,
            (4 as libc::c_int as libc::c_ulong).wrapping_mul(off),
        );
    } else {
        let mut regname_0: [libc::c_char; 20] = [0; 20];
        unw_print_abreg(regname_0.as_mut_ptr(), abreg as libc::c_uint);
        printf(
            b"\t%s:spill_psprel(reg=%s,t=%lu,pspoff=0x10-0x%lx)\n\0" as *const u8
                as *const libc::c_char,
            b"X1\0" as *const u8 as *const libc::c_char,
            regname_0.as_mut_ptr(),
            t,
            (4 as libc::c_int as libc::c_ulong).wrapping_mul(off),
        );
    }
    return dp;
}
unsafe extern "C" fn unw_decode_x2(
    mut dp: *const libc::c_uchar,
    mut code: libc::c_uint,
    mut arg: *mut libc::c_void,
    mut end: *const libc::c_uchar,
) -> *const libc::c_uchar {
    let mut byte1: libc::c_uchar = 0;
    let mut byte2: libc::c_uchar = 0;
    let mut abreg: libc::c_uchar = 0;
    let mut x: libc::c_uchar = 0;
    let mut ytreg: libc::c_uchar = 0;
    let mut t: unw_word = 0;
    if (end.offset_from(dp) as libc::c_long) < 3 as libc::c_int as libc::c_long {
        printf(
            dcgettext(
                0 as *const libc::c_char,
                b"\t<corrupt X2>\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return end;
    }
    let fresh14 = dp;
    dp = dp.offset(1);
    byte1 = *fresh14;
    let fresh15 = dp;
    dp = dp.offset(1);
    byte2 = *fresh15;
    t = unw_decode_uleb128(&mut dp, end);
    abreg = (byte1 as libc::c_int & 0x7f as libc::c_int) as libc::c_uchar;
    ytreg = byte2;
    x = (byte1 as libc::c_int >> 7 as libc::c_int & 1 as libc::c_int) as libc::c_uchar;
    if byte1 as libc::c_int & 0x80 as libc::c_int == 0 as libc::c_int
        && ytreg as libc::c_int == 0 as libc::c_int
    {
        let mut regname: [libc::c_char; 20] = [0; 20];
        unw_print_abreg(regname.as_mut_ptr(), abreg as libc::c_uint);
        printf(
            b"\t%s:restore(t=%lu,reg=%s)\n\0" as *const u8 as *const libc::c_char,
            b"X2\0" as *const u8 as *const libc::c_char,
            t,
            regname.as_mut_ptr(),
        );
    } else {
        let mut abregname: [libc::c_char; 20] = [0; 20];
        let mut tregname: [libc::c_char; 20] = [0; 20];
        unw_print_abreg(abregname.as_mut_ptr(), abreg as libc::c_uint);
        unw_print_xyreg(tregname.as_mut_ptr(), x as libc::c_uint, ytreg as libc::c_uint);
        printf(
            b"\t%s:spill_reg(t=%lu,reg=%s,treg=%s)\n\0" as *const u8
                as *const libc::c_char,
            b"X2\0" as *const u8 as *const libc::c_char,
            t,
            abregname.as_mut_ptr(),
            tregname.as_mut_ptr(),
        );
    }
    return dp;
}
unsafe extern "C" fn unw_decode_x3(
    mut dp: *const libc::c_uchar,
    mut code: libc::c_uint,
    mut arg: *mut libc::c_void,
    mut end: *const libc::c_uchar,
) -> *const libc::c_uchar {
    let mut byte1: libc::c_uchar = 0;
    let mut byte2: libc::c_uchar = 0;
    let mut abreg: libc::c_uchar = 0;
    let mut qp: libc::c_uchar = 0;
    let mut t: unw_word = 0;
    let mut off: unw_word = 0;
    if (end.offset_from(dp) as libc::c_long) < 4 as libc::c_int as libc::c_long {
        printf(
            dcgettext(
                0 as *const libc::c_char,
                b"\t<corrupt X3>\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return end;
    }
    let fresh16 = dp;
    dp = dp.offset(1);
    byte1 = *fresh16;
    let fresh17 = dp;
    dp = dp.offset(1);
    byte2 = *fresh17;
    t = unw_decode_uleb128(&mut dp, end);
    off = unw_decode_uleb128(&mut dp, end);
    qp = (byte1 as libc::c_int & 0x3f as libc::c_int) as libc::c_uchar;
    abreg = (byte2 as libc::c_int & 0x7f as libc::c_int) as libc::c_uchar;
    if byte1 as libc::c_int & 0x80 as libc::c_int != 0 {
        let mut regname: [libc::c_char; 20] = [0; 20];
        unw_print_abreg(regname.as_mut_ptr(), abreg as libc::c_uint);
        printf(
            b"\t%s:spill_sprel_p(qp=p%u,t=%lu,reg=%s,spoff=0x%lx)\n\0" as *const u8
                as *const libc::c_char,
            b"X3\0" as *const u8 as *const libc::c_char,
            qp as libc::c_int,
            t,
            regname.as_mut_ptr(),
            (4 as libc::c_int as libc::c_ulong).wrapping_mul(off),
        );
    } else {
        let mut regname_0: [libc::c_char; 20] = [0; 20];
        unw_print_abreg(regname_0.as_mut_ptr(), abreg as libc::c_uint);
        printf(
            b"\t%s:spill_psprel_p(qp=p%u,t=%lu,reg=%s,pspoff=0x10-0x%lx)\n\0"
                as *const u8 as *const libc::c_char,
            b"X3\0" as *const u8 as *const libc::c_char,
            qp as libc::c_int,
            t,
            regname_0.as_mut_ptr(),
            (4 as libc::c_int as libc::c_ulong).wrapping_mul(off),
        );
    }
    return dp;
}
unsafe extern "C" fn unw_decode_x4(
    mut dp: *const libc::c_uchar,
    mut code: libc::c_uint,
    mut arg: *mut libc::c_void,
    mut end: *const libc::c_uchar,
) -> *const libc::c_uchar {
    let mut byte1: libc::c_uchar = 0;
    let mut byte2: libc::c_uchar = 0;
    let mut byte3: libc::c_uchar = 0;
    let mut qp: libc::c_uchar = 0;
    let mut abreg: libc::c_uchar = 0;
    let mut x: libc::c_uchar = 0;
    let mut ytreg: libc::c_uchar = 0;
    let mut t: unw_word = 0;
    if (end.offset_from(dp) as libc::c_long) < 4 as libc::c_int as libc::c_long {
        printf(
            dcgettext(
                0 as *const libc::c_char,
                b"\t<corrupt X4>\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return end;
    }
    let fresh18 = dp;
    dp = dp.offset(1);
    byte1 = *fresh18;
    let fresh19 = dp;
    dp = dp.offset(1);
    byte2 = *fresh19;
    let fresh20 = dp;
    dp = dp.offset(1);
    byte3 = *fresh20;
    t = unw_decode_uleb128(&mut dp, end);
    qp = (byte1 as libc::c_int & 0x3f as libc::c_int) as libc::c_uchar;
    abreg = (byte2 as libc::c_int & 0x7f as libc::c_int) as libc::c_uchar;
    x = (byte2 as libc::c_int >> 7 as libc::c_int & 1 as libc::c_int) as libc::c_uchar;
    ytreg = byte3;
    if byte2 as libc::c_int & 0x80 as libc::c_int == 0 as libc::c_int
        && byte3 as libc::c_int == 0 as libc::c_int
    {
        let mut regname: [libc::c_char; 20] = [0; 20];
        unw_print_abreg(regname.as_mut_ptr(), abreg as libc::c_uint);
        printf(
            b"\t%s:restore_p(qp=p%u,t=%lu,reg=%s)\n\0" as *const u8
                as *const libc::c_char,
            b"X4\0" as *const u8 as *const libc::c_char,
            qp as libc::c_int,
            t,
            regname.as_mut_ptr(),
        );
    } else {
        let mut regname_0: [libc::c_char; 20] = [0; 20];
        let mut tregname: [libc::c_char; 20] = [0; 20];
        unw_print_abreg(regname_0.as_mut_ptr(), abreg as libc::c_uint);
        unw_print_xyreg(tregname.as_mut_ptr(), x as libc::c_uint, ytreg as libc::c_uint);
        printf(
            b"\t%s:spill_reg_p(qp=p%u,t=%lu,reg=%s,treg=%s)\n\0" as *const u8
                as *const libc::c_char,
            b"X4\0" as *const u8 as *const libc::c_char,
            qp as libc::c_int,
            t,
            regname_0.as_mut_ptr(),
            tregname.as_mut_ptr(),
        );
    }
    return dp;
}
unsafe extern "C" fn unw_decode_r1(
    mut dp: *const libc::c_uchar,
    mut code: libc::c_uint,
    mut arg: *mut libc::c_void,
    mut end: *const libc::c_uchar,
) -> *const libc::c_uchar {
    let mut body: libc::c_int = (code & 0x20 as libc::c_int as libc::c_uint
        != 0 as libc::c_int as libc::c_uint) as libc::c_int;
    let mut rlen: unw_word = 0;
    rlen = (code & 0x1f as libc::c_int as libc::c_uint) as unw_word;
    unw_rlen = rlen;
    *(arg as *mut libc::c_int) = body;
    printf(
        b"    %s:%s(rlen=%lu)\n\0" as *const u8 as *const libc::c_char,
        b"R1\0" as *const u8 as *const libc::c_char,
        if body != 0 {
            b"body\0" as *const u8 as *const libc::c_char
        } else {
            b"prologue\0" as *const u8 as *const libc::c_char
        },
        rlen,
    );
    return dp;
}
unsafe extern "C" fn unw_decode_r2(
    mut dp: *const libc::c_uchar,
    mut code: libc::c_uint,
    mut arg: *mut libc::c_void,
    mut end: *const libc::c_uchar,
) -> *const libc::c_uchar {
    let mut byte1: libc::c_uchar = 0;
    let mut mask: libc::c_uchar = 0;
    let mut grsave: libc::c_uchar = 0;
    let mut rlen: unw_word = 0;
    if (end.offset_from(dp) as libc::c_long) < 2 as libc::c_int as libc::c_long {
        printf(
            dcgettext(
                0 as *const libc::c_char,
                b"\t<corrupt R2>\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return end;
    }
    let fresh21 = dp;
    dp = dp.offset(1);
    byte1 = *fresh21;
    mask = ((code & 0x7 as libc::c_int as libc::c_uint) << 1 as libc::c_int
        | (byte1 as libc::c_int >> 7 as libc::c_int & 1 as libc::c_int) as libc::c_uint)
        as libc::c_uchar;
    grsave = (byte1 as libc::c_int & 0x7f as libc::c_int) as libc::c_uchar;
    rlen = unw_decode_uleb128(&mut dp, end);
    let mut regname: [libc::c_char; 16] = [0; 16];
    let mut maskstr: [libc::c_char; 64] = [0; 64];
    let mut sep: *mut libc::c_char = 0 as *mut libc::c_char;
    unw_rlen = rlen;
    *(arg as *mut libc::c_int) = 0 as libc::c_int;
    maskstr[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    sep = b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    if mask as libc::c_int & 0x8 as libc::c_int != 0 {
        strcat(maskstr.as_mut_ptr(), b"rp\0" as *const u8 as *const libc::c_char);
        sep = b",\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if mask as libc::c_int & 0x4 as libc::c_int != 0 {
        strcat(maskstr.as_mut_ptr(), sep);
        strcat(maskstr.as_mut_ptr(), b"ar.pfs\0" as *const u8 as *const libc::c_char);
        sep = b",\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if mask as libc::c_int & 0x2 as libc::c_int != 0 {
        strcat(maskstr.as_mut_ptr(), sep);
        strcat(maskstr.as_mut_ptr(), b"psp\0" as *const u8 as *const libc::c_char);
        sep = b",\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if mask as libc::c_int & 0x1 as libc::c_int != 0 {
        strcat(maskstr.as_mut_ptr(), sep);
        strcat(maskstr.as_mut_ptr(), b"pr\0" as *const u8 as *const libc::c_char);
    }
    sprintf(
        regname.as_mut_ptr(),
        b"r%u\0" as *const u8 as *const libc::c_char,
        grsave as libc::c_int,
    );
    printf(
        b"    %s:prologue_gr(mask=[%s],grsave=%s,rlen=%lu)\n\0" as *const u8
            as *const libc::c_char,
        b"R2\0" as *const u8 as *const libc::c_char,
        maskstr.as_mut_ptr(),
        regname.as_mut_ptr(),
        rlen,
    );
    return dp;
}
unsafe extern "C" fn unw_decode_r3(
    mut dp: *const libc::c_uchar,
    mut code: libc::c_uint,
    mut arg: *mut libc::c_void,
    mut end: *const libc::c_uchar,
) -> *const libc::c_uchar {
    let mut rlen: unw_word = 0;
    rlen = unw_decode_uleb128(&mut dp, end);
    unw_rlen = rlen;
    *(arg
        as *mut libc::c_int) = (code & 0x3 as libc::c_int as libc::c_uint
        == 1 as libc::c_int as libc::c_uint) as libc::c_int;
    printf(
        b"    %s:%s(rlen=%lu)\n\0" as *const u8 as *const libc::c_char,
        b"R3\0" as *const u8 as *const libc::c_char,
        if code & 0x3 as libc::c_int as libc::c_uint == 1 as libc::c_int as libc::c_uint
        {
            b"body\0" as *const u8 as *const libc::c_char
        } else {
            b"prologue\0" as *const u8 as *const libc::c_char
        },
        rlen,
    );
    return dp;
}
unsafe extern "C" fn unw_decode_p1(
    mut dp: *const libc::c_uchar,
    mut code: libc::c_uint,
    mut arg: *mut libc::c_void,
    mut end: *const libc::c_uchar,
) -> *const libc::c_uchar {
    let mut brmask: libc::c_uchar = (code & 0x1f as libc::c_int as libc::c_uint)
        as libc::c_uchar;
    let mut brstr: [libc::c_char; 20] = [0; 20];
    unw_print_brmask(brstr.as_mut_ptr(), brmask as libc::c_uint);
    printf(
        b"\t%s:br_mem(brmask=[%s])\n\0" as *const u8 as *const libc::c_char,
        b"P1\0" as *const u8 as *const libc::c_char,
        brstr.as_mut_ptr(),
    );
    return dp;
}
unsafe extern "C" fn unw_decode_p2_p5(
    mut dp: *const libc::c_uchar,
    mut code: libc::c_uint,
    mut arg: *mut libc::c_void,
    mut end: *const libc::c_uchar,
) -> *const libc::c_uchar {
    if code & 0x10 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
        let mut byte1: libc::c_uchar = 0;
        if (end.offset_from(dp) as libc::c_long) < 1 as libc::c_int as libc::c_long {
            printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"\t<corrupt P2>\n\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            return end;
        }
        let fresh22 = dp;
        dp = dp.offset(1);
        byte1 = *fresh22;
        let mut brstr: [libc::c_char; 20] = [0; 20];
        unw_print_brmask(
            brstr.as_mut_ptr(),
            (code & 0xf as libc::c_int as libc::c_uint) << 1 as libc::c_int
                | (byte1 as libc::c_int >> 7 as libc::c_int & 1 as libc::c_int)
                    as libc::c_uint,
        );
        printf(
            b"\t%s:br_gr(brmask=[%s],gr=r%u)\n\0" as *const u8 as *const libc::c_char,
            b"P2\0" as *const u8 as *const libc::c_char,
            brstr.as_mut_ptr(),
            byte1 as libc::c_int & 0x7f as libc::c_int,
        );
    } else if code & 0x8 as libc::c_int as libc::c_uint
        == 0 as libc::c_int as libc::c_uint
    {
        let mut byte1_0: libc::c_uchar = 0;
        let mut r: libc::c_uchar = 0;
        let mut dst: libc::c_uchar = 0;
        if (end.offset_from(dp) as libc::c_long) < 1 as libc::c_int as libc::c_long {
            printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"\t<corrupt P3>\n\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            return end;
        }
        let fresh23 = dp;
        dp = dp.offset(1);
        byte1_0 = *fresh23;
        r = ((code & 0x7 as libc::c_int as libc::c_uint) << 1 as libc::c_int
            | (byte1_0 as libc::c_int >> 7 as libc::c_int & 1 as libc::c_int)
                as libc::c_uint) as libc::c_uchar;
        dst = (byte1_0 as libc::c_int & 0x7f as libc::c_int) as libc::c_uchar;
        match r as libc::c_int {
            0 => {
                printf(
                    b"\t%s:%s_gr(reg=r%u)\n\0" as *const u8 as *const libc::c_char,
                    b"P3\0" as *const u8 as *const libc::c_char,
                    b"psp\0" as *const u8 as *const libc::c_char,
                    dst as libc::c_int,
                );
            }
            1 => {
                printf(
                    b"\t%s:%s_gr(reg=r%u)\n\0" as *const u8 as *const libc::c_char,
                    b"P3\0" as *const u8 as *const libc::c_char,
                    b"rp\0" as *const u8 as *const libc::c_char,
                    dst as libc::c_int,
                );
            }
            2 => {
                printf(
                    b"\t%s:%s_gr(reg=r%u)\n\0" as *const u8 as *const libc::c_char,
                    b"P3\0" as *const u8 as *const libc::c_char,
                    b"pfs\0" as *const u8 as *const libc::c_char,
                    dst as libc::c_int,
                );
            }
            3 => {
                printf(
                    b"\t%s:%s_gr(reg=r%u)\n\0" as *const u8 as *const libc::c_char,
                    b"P3\0" as *const u8 as *const libc::c_char,
                    b"pr\0" as *const u8 as *const libc::c_char,
                    dst as libc::c_int,
                );
            }
            4 => {
                printf(
                    b"\t%s:%s_gr(reg=r%u)\n\0" as *const u8 as *const libc::c_char,
                    b"P3\0" as *const u8 as *const libc::c_char,
                    b"unat\0" as *const u8 as *const libc::c_char,
                    dst as libc::c_int,
                );
            }
            5 => {
                printf(
                    b"\t%s:%s_gr(reg=r%u)\n\0" as *const u8 as *const libc::c_char,
                    b"P3\0" as *const u8 as *const libc::c_char,
                    b"lc\0" as *const u8 as *const libc::c_char,
                    dst as libc::c_int,
                );
            }
            6 => {
                printf(
                    b"\t%s:rp_br(reg=b%u)\n\0" as *const u8 as *const libc::c_char,
                    b"P3\0" as *const u8 as *const libc::c_char,
                    dst as libc::c_int,
                );
            }
            7 => {
                printf(
                    b"\t%s:%s_gr(reg=r%u)\n\0" as *const u8 as *const libc::c_char,
                    b"P3\0" as *const u8 as *const libc::c_char,
                    b"rnat\0" as *const u8 as *const libc::c_char,
                    dst as libc::c_int,
                );
            }
            8 => {
                printf(
                    b"\t%s:%s_gr(reg=r%u)\n\0" as *const u8 as *const libc::c_char,
                    b"P3\0" as *const u8 as *const libc::c_char,
                    b"bsp\0" as *const u8 as *const libc::c_char,
                    dst as libc::c_int,
                );
            }
            9 => {
                printf(
                    b"\t%s:%s_gr(reg=r%u)\n\0" as *const u8 as *const libc::c_char,
                    b"P3\0" as *const u8 as *const libc::c_char,
                    b"bspstore\0" as *const u8 as *const libc::c_char,
                    dst as libc::c_int,
                );
            }
            10 => {
                printf(
                    b"\t%s:%s_gr(reg=r%u)\n\0" as *const u8 as *const libc::c_char,
                    b"P3\0" as *const u8 as *const libc::c_char,
                    b"fpsr\0" as *const u8 as *const libc::c_char,
                    dst as libc::c_int,
                );
            }
            11 => {
                printf(
                    b"\t%s:priunat_gr(reg=r%u)\n\0" as *const u8 as *const libc::c_char,
                    b"P3\0" as *const u8 as *const libc::c_char,
                    dst as libc::c_int,
                );
            }
            _ => {
                printf(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Unknown code 0x%02x\n\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    r as libc::c_int,
                );
            }
        }
    } else if code & 0x7 as libc::c_int as libc::c_uint
        == 0 as libc::c_int as libc::c_uint
    {
        static mut spill_type: *const libc::c_char = b"-frb\0" as *const u8
            as *const libc::c_char;
        let mut imaskp: *const libc::c_uchar = dp;
        let mut mask: libc::c_uchar = 0 as libc::c_int as libc::c_uchar;
        let mut insn: bfd_vma = 0 as libc::c_int as bfd_vma;
        if dp.offset(unw_rlen.wrapping_div(4 as libc::c_int as libc::c_ulong) as isize)
            > end
        {
            printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"\nERROR: unwind length too long (0x%lx > 0x%lx)\n\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                unw_rlen.wrapping_div(4 as libc::c_int as libc::c_ulong) as libc::c_long,
                end.offset_from(dp) as libc::c_long,
            );
        } else {
            printf(
                b"\t%s:spill_mask(imask=[\0" as *const u8 as *const libc::c_char,
                b"P4\0" as *const u8 as *const libc::c_char,
            );
            insn = 0 as libc::c_int as bfd_vma;
            while insn < unw_rlen {
                if insn.wrapping_rem(4 as libc::c_int as libc::c_ulong)
                    == 0 as libc::c_int as libc::c_ulong
                {
                    let fresh24 = imaskp;
                    imaskp = imaskp.offset(1);
                    mask = *fresh24;
                }
                if insn > 0 as libc::c_int as libc::c_ulong
                    && insn.wrapping_rem(3 as libc::c_int as libc::c_ulong)
                        == 0 as libc::c_int as libc::c_ulong
                {
                    putchar(',' as i32);
                }
                putchar(
                    *spill_type
                        .offset(
                            (mask as libc::c_int
                                >> (2 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(
                                        (3 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(insn & 0x3 as libc::c_int as libc::c_ulong),
                                    ) & 0x3 as libc::c_int) as isize,
                        ) as libc::c_int,
                );
                insn = insn.wrapping_add(1);
                insn;
            }
            printf(b"])\n\0" as *const u8 as *const libc::c_char);
            dp = imaskp;
        }
    } else if code & 0x7 as libc::c_int as libc::c_uint
        == 1 as libc::c_int as libc::c_uint
    {
        let mut grmask: unw_word = 0;
        let mut frmask: unw_word = 0;
        let mut byte1_1: unw_word = 0;
        let mut byte2: unw_word = 0;
        let mut byte3: unw_word = 0;
        if (end.offset_from(dp) as libc::c_long) < 3 as libc::c_int as libc::c_long {
            printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"\t<corrupt P5>\n\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            return end;
        }
        let fresh25 = dp;
        dp = dp.offset(1);
        byte1_1 = *fresh25 as unw_word;
        let fresh26 = dp;
        dp = dp.offset(1);
        byte2 = *fresh26 as unw_word;
        let fresh27 = dp;
        dp = dp.offset(1);
        byte3 = *fresh27 as unw_word;
        grmask = byte1_1 >> 4 as libc::c_int & 0xf as libc::c_int as libc::c_ulong;
        frmask = (byte1_1 & 0xf as libc::c_int as libc::c_ulong) << 16 as libc::c_int
            | byte2 << 8 as libc::c_int | byte3;
        let mut frstr: [libc::c_char; 200] = [0; 200];
        let mut grstr: [libc::c_char; 20] = [0; 20];
        unw_print_grmask(grstr.as_mut_ptr(), grmask as libc::c_uint);
        unw_print_frmask(frstr.as_mut_ptr(), frmask as libc::c_uint);
        printf(
            b"\t%s:frgr_mem(grmask=[%s],frmask=[%s])\n\0" as *const u8
                as *const libc::c_char,
            b"P5\0" as *const u8 as *const libc::c_char,
            grstr.as_mut_ptr(),
            frstr.as_mut_ptr(),
        );
    } else {
        printf(
            dcgettext(
                0 as *const libc::c_char,
                b"Unknown code 0x%02x\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            code,
        );
    }
    return dp;
}
unsafe extern "C" fn unw_decode_p6(
    mut dp: *const libc::c_uchar,
    mut code: libc::c_uint,
    mut arg: *mut libc::c_void,
    mut end: *const libc::c_uchar,
) -> *const libc::c_uchar {
    let mut gregs: libc::c_int = (code & 0x10 as libc::c_int as libc::c_uint
        != 0 as libc::c_int as libc::c_uint) as libc::c_int;
    let mut mask: libc::c_uchar = (code & 0xf as libc::c_int as libc::c_uint)
        as libc::c_uchar;
    if gregs != 0 {
        let mut grstr: [libc::c_char; 200] = [0; 200];
        unw_print_grmask(grstr.as_mut_ptr(), mask as libc::c_uint);
        printf(
            b"\t%s:gr_mem(grmask=[%s])\n\0" as *const u8 as *const libc::c_char,
            b"P6\0" as *const u8 as *const libc::c_char,
            grstr.as_mut_ptr(),
        );
    } else {
        let mut frstr: [libc::c_char; 200] = [0; 200];
        unw_print_frmask(frstr.as_mut_ptr(), mask as libc::c_uint);
        printf(
            b"\t%s:fr_mem(frmask=[%s])\n\0" as *const u8 as *const libc::c_char,
            b"P6\0" as *const u8 as *const libc::c_char,
            frstr.as_mut_ptr(),
        );
    }
    return dp;
}
unsafe extern "C" fn unw_decode_p7_p10(
    mut dp: *const libc::c_uchar,
    mut code: libc::c_uint,
    mut arg: *mut libc::c_void,
    mut end: *const libc::c_uchar,
) -> *const libc::c_uchar {
    let mut r: libc::c_uchar = 0;
    let mut byte1: libc::c_uchar = 0;
    let mut byte2: libc::c_uchar = 0;
    let mut t: unw_word = 0;
    let mut size: unw_word = 0;
    if code & 0x10 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
        r = (code & 0xf as libc::c_int as libc::c_uint) as libc::c_uchar;
        t = unw_decode_uleb128(&mut dp, end);
        match r as libc::c_int {
            0 => {
                size = unw_decode_uleb128(&mut dp, end);
                printf(
                    b"\t%s:mem_stack_f(t=%lu,size=%lu)\n\0" as *const u8
                        as *const libc::c_char,
                    b"P7\0" as *const u8 as *const libc::c_char,
                    t,
                    (16 as libc::c_int as libc::c_ulong).wrapping_mul(size),
                );
            }
            1 => {
                printf(
                    b"\t%s:mem_stack_v(t=%lu)\n\0" as *const u8 as *const libc::c_char,
                    b"P7\0" as *const u8 as *const libc::c_char,
                    t,
                );
            }
            2 => {
                printf(
                    b"\t%s:spill_base(pspoff=0x10-0x%lx)\n\0" as *const u8
                        as *const libc::c_char,
                    b"P7\0" as *const u8 as *const libc::c_char,
                    (4 as libc::c_int as libc::c_ulong).wrapping_mul(t),
                );
            }
            3 => {
                printf(
                    b"\t%s:%s_sprel(spoff=0x%lx)\n\0" as *const u8
                        as *const libc::c_char,
                    b"P7\0" as *const u8 as *const libc::c_char,
                    b"psp\0" as *const u8 as *const libc::c_char,
                    (4 as libc::c_int as libc::c_ulong).wrapping_mul(t),
                );
            }
            4 => {
                printf(
                    b"\t%s:%s_when(t=%lu)\n\0" as *const u8 as *const libc::c_char,
                    b"P7\0" as *const u8 as *const libc::c_char,
                    b"rp\0" as *const u8 as *const libc::c_char,
                    t,
                );
            }
            5 => {
                printf(
                    b"\t%s:%s_psprel(pspoff=0x10-0x%lx)\n\0" as *const u8
                        as *const libc::c_char,
                    b"P7\0" as *const u8 as *const libc::c_char,
                    b"rp\0" as *const u8 as *const libc::c_char,
                    (4 as libc::c_int as libc::c_ulong).wrapping_mul(t),
                );
            }
            6 => {
                printf(
                    b"\t%s:%s_when(t=%lu)\n\0" as *const u8 as *const libc::c_char,
                    b"P7\0" as *const u8 as *const libc::c_char,
                    b"pfs\0" as *const u8 as *const libc::c_char,
                    t,
                );
            }
            7 => {
                printf(
                    b"\t%s:%s_psprel(pspoff=0x10-0x%lx)\n\0" as *const u8
                        as *const libc::c_char,
                    b"P7\0" as *const u8 as *const libc::c_char,
                    b"pfs\0" as *const u8 as *const libc::c_char,
                    (4 as libc::c_int as libc::c_ulong).wrapping_mul(t),
                );
            }
            8 => {
                printf(
                    b"\t%s:%s_when(t=%lu)\n\0" as *const u8 as *const libc::c_char,
                    b"P7\0" as *const u8 as *const libc::c_char,
                    b"pr\0" as *const u8 as *const libc::c_char,
                    t,
                );
            }
            9 => {
                printf(
                    b"\t%s:%s_psprel(pspoff=0x10-0x%lx)\n\0" as *const u8
                        as *const libc::c_char,
                    b"P7\0" as *const u8 as *const libc::c_char,
                    b"pr\0" as *const u8 as *const libc::c_char,
                    (4 as libc::c_int as libc::c_ulong).wrapping_mul(t),
                );
            }
            10 => {
                printf(
                    b"\t%s:%s_when(t=%lu)\n\0" as *const u8 as *const libc::c_char,
                    b"P7\0" as *const u8 as *const libc::c_char,
                    b"lc\0" as *const u8 as *const libc::c_char,
                    t,
                );
            }
            11 => {
                printf(
                    b"\t%s:%s_psprel(pspoff=0x10-0x%lx)\n\0" as *const u8
                        as *const libc::c_char,
                    b"P7\0" as *const u8 as *const libc::c_char,
                    b"lc\0" as *const u8 as *const libc::c_char,
                    (4 as libc::c_int as libc::c_ulong).wrapping_mul(t),
                );
            }
            12 => {
                printf(
                    b"\t%s:%s_when(t=%lu)\n\0" as *const u8 as *const libc::c_char,
                    b"P7\0" as *const u8 as *const libc::c_char,
                    b"unat\0" as *const u8 as *const libc::c_char,
                    t,
                );
            }
            13 => {
                printf(
                    b"\t%s:%s_psprel(pspoff=0x10-0x%lx)\n\0" as *const u8
                        as *const libc::c_char,
                    b"P7\0" as *const u8 as *const libc::c_char,
                    b"unat\0" as *const u8 as *const libc::c_char,
                    (4 as libc::c_int as libc::c_ulong).wrapping_mul(t),
                );
            }
            14 => {
                printf(
                    b"\t%s:%s_when(t=%lu)\n\0" as *const u8 as *const libc::c_char,
                    b"P7\0" as *const u8 as *const libc::c_char,
                    b"fpsr\0" as *const u8 as *const libc::c_char,
                    t,
                );
            }
            15 => {
                printf(
                    b"\t%s:%s_psprel(pspoff=0x10-0x%lx)\n\0" as *const u8
                        as *const libc::c_char,
                    b"P7\0" as *const u8 as *const libc::c_char,
                    b"fpsr\0" as *const u8 as *const libc::c_char,
                    (4 as libc::c_int as libc::c_ulong).wrapping_mul(t),
                );
            }
            _ => {
                printf(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Unknown code 0x%02x\n\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    r as libc::c_int,
                );
            }
        }
    } else {
        match code & 0xf as libc::c_int as libc::c_uint {
            0 => {
                if (end.offset_from(dp) as libc::c_long)
                    < 2 as libc::c_int as libc::c_long
                {
                    printf(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"\t<corrupt P8>\n\0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    return end;
                }
                let fresh28 = dp;
                dp = dp.offset(1);
                r = *fresh28;
                t = unw_decode_uleb128(&mut dp, end);
                match r as libc::c_int {
                    1 => {
                        printf(
                            b"\t%s:%s_sprel(spoff=0x%lx)\n\0" as *const u8
                                as *const libc::c_char,
                            b"P8\0" as *const u8 as *const libc::c_char,
                            b"rp\0" as *const u8 as *const libc::c_char,
                            (4 as libc::c_int as libc::c_ulong).wrapping_mul(t),
                        );
                    }
                    2 => {
                        printf(
                            b"\t%s:%s_sprel(spoff=0x%lx)\n\0" as *const u8
                                as *const libc::c_char,
                            b"P8\0" as *const u8 as *const libc::c_char,
                            b"pfs\0" as *const u8 as *const libc::c_char,
                            (4 as libc::c_int as libc::c_ulong).wrapping_mul(t),
                        );
                    }
                    3 => {
                        printf(
                            b"\t%s:%s_sprel(spoff=0x%lx)\n\0" as *const u8
                                as *const libc::c_char,
                            b"P8\0" as *const u8 as *const libc::c_char,
                            b"pr\0" as *const u8 as *const libc::c_char,
                            (4 as libc::c_int as libc::c_ulong).wrapping_mul(t),
                        );
                    }
                    4 => {
                        printf(
                            b"\t%s:%s_sprel(spoff=0x%lx)\n\0" as *const u8
                                as *const libc::c_char,
                            b"P8\0" as *const u8 as *const libc::c_char,
                            b"lc\0" as *const u8 as *const libc::c_char,
                            (4 as libc::c_int as libc::c_ulong).wrapping_mul(t),
                        );
                    }
                    5 => {
                        printf(
                            b"\t%s:%s_sprel(spoff=0x%lx)\n\0" as *const u8
                                as *const libc::c_char,
                            b"P8\0" as *const u8 as *const libc::c_char,
                            b"unat\0" as *const u8 as *const libc::c_char,
                            (4 as libc::c_int as libc::c_ulong).wrapping_mul(t),
                        );
                    }
                    6 => {
                        printf(
                            b"\t%s:%s_sprel(spoff=0x%lx)\n\0" as *const u8
                                as *const libc::c_char,
                            b"P8\0" as *const u8 as *const libc::c_char,
                            b"fpsr\0" as *const u8 as *const libc::c_char,
                            (4 as libc::c_int as libc::c_ulong).wrapping_mul(t),
                        );
                    }
                    7 => {
                        printf(
                            b"\t%s:%s_when(t=%lu)\n\0" as *const u8
                                as *const libc::c_char,
                            b"P8\0" as *const u8 as *const libc::c_char,
                            b"bsp\0" as *const u8 as *const libc::c_char,
                            t,
                        );
                    }
                    8 => {
                        printf(
                            b"\t%s:%s_psprel(pspoff=0x10-0x%lx)\n\0" as *const u8
                                as *const libc::c_char,
                            b"P8\0" as *const u8 as *const libc::c_char,
                            b"bsp\0" as *const u8 as *const libc::c_char,
                            (4 as libc::c_int as libc::c_ulong).wrapping_mul(t),
                        );
                    }
                    9 => {
                        printf(
                            b"\t%s:%s_sprel(spoff=0x%lx)\n\0" as *const u8
                                as *const libc::c_char,
                            b"P8\0" as *const u8 as *const libc::c_char,
                            b"bsp\0" as *const u8 as *const libc::c_char,
                            (4 as libc::c_int as libc::c_ulong).wrapping_mul(t),
                        );
                    }
                    10 => {
                        printf(
                            b"\t%s:%s_when(t=%lu)\n\0" as *const u8
                                as *const libc::c_char,
                            b"P8\0" as *const u8 as *const libc::c_char,
                            b"bspstore\0" as *const u8 as *const libc::c_char,
                            t,
                        );
                    }
                    11 => {
                        printf(
                            b"\t%s:%s_psprel(pspoff=0x10-0x%lx)\n\0" as *const u8
                                as *const libc::c_char,
                            b"P8\0" as *const u8 as *const libc::c_char,
                            b"bspstore\0" as *const u8 as *const libc::c_char,
                            (4 as libc::c_int as libc::c_ulong).wrapping_mul(t),
                        );
                    }
                    12 => {
                        printf(
                            b"\t%s:%s_sprel(spoff=0x%lx)\n\0" as *const u8
                                as *const libc::c_char,
                            b"P8\0" as *const u8 as *const libc::c_char,
                            b"bspstore\0" as *const u8 as *const libc::c_char,
                            (4 as libc::c_int as libc::c_ulong).wrapping_mul(t),
                        );
                    }
                    13 => {
                        printf(
                            b"\t%s:%s_when(t=%lu)\n\0" as *const u8
                                as *const libc::c_char,
                            b"P8\0" as *const u8 as *const libc::c_char,
                            b"rnat\0" as *const u8 as *const libc::c_char,
                            t,
                        );
                    }
                    14 => {
                        printf(
                            b"\t%s:%s_psprel(pspoff=0x10-0x%lx)\n\0" as *const u8
                                as *const libc::c_char,
                            b"P8\0" as *const u8 as *const libc::c_char,
                            b"rnat\0" as *const u8 as *const libc::c_char,
                            (4 as libc::c_int as libc::c_ulong).wrapping_mul(t),
                        );
                    }
                    15 => {
                        printf(
                            b"\t%s:%s_sprel(spoff=0x%lx)\n\0" as *const u8
                                as *const libc::c_char,
                            b"P8\0" as *const u8 as *const libc::c_char,
                            b"rnat\0" as *const u8 as *const libc::c_char,
                            (4 as libc::c_int as libc::c_ulong).wrapping_mul(t),
                        );
                    }
                    16 => {
                        printf(
                            b"\t%s:priunat_when_gr(t=%lu)\n\0" as *const u8
                                as *const libc::c_char,
                            b"P8\0" as *const u8 as *const libc::c_char,
                            t,
                        );
                    }
                    17 => {
                        printf(
                            b"\t%s:priunat_psprel(pspoff=0x10-0x%lx)\n\0" as *const u8
                                as *const libc::c_char,
                            b"P8\0" as *const u8 as *const libc::c_char,
                            (4 as libc::c_int as libc::c_ulong).wrapping_mul(t),
                        );
                    }
                    18 => {
                        printf(
                            b"\t%s:priunat_sprel(spoff=0x%lx)\n\0" as *const u8
                                as *const libc::c_char,
                            b"P8\0" as *const u8 as *const libc::c_char,
                            (4 as libc::c_int as libc::c_ulong).wrapping_mul(t),
                        );
                    }
                    19 => {
                        printf(
                            b"\t%s:priunat_when_mem(t=%lu)\n\0" as *const u8
                                as *const libc::c_char,
                            b"P8\0" as *const u8 as *const libc::c_char,
                            t,
                        );
                    }
                    _ => {
                        printf(
                            dcgettext(
                                0 as *const libc::c_char,
                                b"Unknown code 0x%02x\n\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            r as libc::c_int,
                        );
                    }
                }
            }
            1 => {
                if (end.offset_from(dp) as libc::c_long)
                    < 2 as libc::c_int as libc::c_long
                {
                    printf(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"\t<corrupt P9>\n\0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    return end;
                }
                let fresh29 = dp;
                dp = dp.offset(1);
                byte1 = *fresh29;
                let fresh30 = dp;
                dp = dp.offset(1);
                byte2 = *fresh30;
                let mut grstr: [libc::c_char; 20] = [0; 20];
                unw_print_grmask(
                    grstr.as_mut_ptr(),
                    (byte1 as libc::c_int & 0xf as libc::c_int) as libc::c_uint,
                );
                printf(
                    b"\t%s:gr_gr(grmask=[%s],r%u)\n\0" as *const u8
                        as *const libc::c_char,
                    b"P9\0" as *const u8 as *const libc::c_char,
                    grstr.as_mut_ptr(),
                    byte2 as libc::c_int & 0x7f as libc::c_int,
                );
            }
            15 => {
                if (end.offset_from(dp) as libc::c_long)
                    < 2 as libc::c_int as libc::c_long
                {
                    printf(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"\t<corrupt P10>\n\0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    return end;
                }
                let fresh31 = dp;
                dp = dp.offset(1);
                byte1 = *fresh31;
                let fresh32 = dp;
                dp = dp.offset(1);
                byte2 = *fresh32;
                static mut abiname: [*const libc::c_char; 3] = [
                    b"@svr4\0" as *const u8 as *const libc::c_char,
                    b"@hpux\0" as *const u8 as *const libc::c_char,
                    b"@nt\0" as *const u8 as *const libc::c_char,
                ];
                let mut buf: [libc::c_char; 20] = [0; 20];
                let mut abistr: *const libc::c_char = buf.as_mut_ptr();
                if (byte1 as libc::c_int) < 3 as libc::c_int {
                    abistr = abiname[byte1 as usize];
                } else {
                    sprintf(
                        buf.as_mut_ptr(),
                        b"0x%x\0" as *const u8 as *const libc::c_char,
                        byte1 as libc::c_int,
                    );
                }
                printf(
                    b"\t%s:unwabi(abi=%s,context=0x%02x)\n\0" as *const u8
                        as *const libc::c_char,
                    b"P10\0" as *const u8 as *const libc::c_char,
                    abistr,
                    byte2 as libc::c_int,
                );
            }
            9 => return unw_decode_x1(dp, code, arg, end),
            10 => return unw_decode_x2(dp, code, arg, end),
            11 => return unw_decode_x3(dp, code, arg, end),
            12 => return unw_decode_x4(dp, code, arg, end),
            _ => {
                printf(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Unknown code 0x%02x\n\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    code,
                );
            }
        }
    }
    return dp;
}
unsafe extern "C" fn unw_decode_b1(
    mut dp: *const libc::c_uchar,
    mut code: libc::c_uint,
    mut arg: *mut libc::c_void,
    mut end: *const libc::c_uchar,
) -> *const libc::c_uchar {
    let mut label: unw_word = (code & 0x1f as libc::c_int as libc::c_uint) as unw_word;
    if code & 0x20 as libc::c_int as libc::c_uint != 0 as libc::c_int as libc::c_uint {
        printf(
            b"\t%s:copy_state(label=%lu)\n\0" as *const u8 as *const libc::c_char,
            b"B1\0" as *const u8 as *const libc::c_char,
            label,
        );
    } else {
        printf(
            b"\t%s:label_state(label=%lu)\n\0" as *const u8 as *const libc::c_char,
            b"B1\0" as *const u8 as *const libc::c_char,
            label,
        );
    }
    return dp;
}
unsafe extern "C" fn unw_decode_b2(
    mut dp: *const libc::c_uchar,
    mut code: libc::c_uint,
    mut arg: *mut libc::c_void,
    mut end: *const libc::c_uchar,
) -> *const libc::c_uchar {
    let mut t: unw_word = 0;
    t = unw_decode_uleb128(&mut dp, end);
    printf(
        b"\t%s:epilogue(t=%lu,ecount=%lu)\n\0" as *const u8 as *const libc::c_char,
        b"B2\0" as *const u8 as *const libc::c_char,
        t,
        (code & 0x1f as libc::c_int as libc::c_uint) as libc::c_ulong,
    );
    return dp;
}
unsafe extern "C" fn unw_decode_b3_x4(
    mut dp: *const libc::c_uchar,
    mut code: libc::c_uint,
    mut arg: *mut libc::c_void,
    mut end: *const libc::c_uchar,
) -> *const libc::c_uchar {
    let mut t: unw_word = 0;
    let mut ecount: unw_word = 0;
    let mut label: unw_word = 0;
    if code & 0x10 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
        t = unw_decode_uleb128(&mut dp, end);
        ecount = unw_decode_uleb128(&mut dp, end);
        printf(
            b"\t%s:epilogue(t=%lu,ecount=%lu)\n\0" as *const u8 as *const libc::c_char,
            b"B3\0" as *const u8 as *const libc::c_char,
            t,
            ecount,
        );
    } else if code & 0x7 as libc::c_int as libc::c_uint
        == 0 as libc::c_int as libc::c_uint
    {
        label = unw_decode_uleb128(&mut dp, end);
        if code & 0x8 as libc::c_int as libc::c_uint != 0 as libc::c_int as libc::c_uint
        {
            printf(
                b"\t%s:copy_state(label=%lu)\n\0" as *const u8 as *const libc::c_char,
                b"B4\0" as *const u8 as *const libc::c_char,
                label,
            );
        } else {
            printf(
                b"\t%s:label_state(label=%lu)\n\0" as *const u8 as *const libc::c_char,
                b"B4\0" as *const u8 as *const libc::c_char,
                label,
            );
        }
    } else {
        match code & 0x7 as libc::c_int as libc::c_uint {
            1 => return unw_decode_x1(dp, code, arg, end),
            2 => return unw_decode_x2(dp, code, arg, end),
            3 => return unw_decode_x3(dp, code, arg, end),
            4 => return unw_decode_x4(dp, code, arg, end),
            _ => {
                printf(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Unknown code 0x%02x\n\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    code,
                );
            }
        }
    }
    return dp;
}
static mut unw_decode_table: [[unw_decoder; 8]; 2] = unsafe {
    [
        [
            Some(
                unw_decode_r1
                    as unsafe extern "C" fn(
                        *const libc::c_uchar,
                        libc::c_uint,
                        *mut libc::c_void,
                        *const libc::c_uchar,
                    ) -> *const libc::c_uchar,
            ),
            Some(
                unw_decode_r1
                    as unsafe extern "C" fn(
                        *const libc::c_uchar,
                        libc::c_uint,
                        *mut libc::c_void,
                        *const libc::c_uchar,
                    ) -> *const libc::c_uchar,
            ),
            Some(
                unw_decode_r2
                    as unsafe extern "C" fn(
                        *const libc::c_uchar,
                        libc::c_uint,
                        *mut libc::c_void,
                        *const libc::c_uchar,
                    ) -> *const libc::c_uchar,
            ),
            Some(
                unw_decode_r3
                    as unsafe extern "C" fn(
                        *const libc::c_uchar,
                        libc::c_uint,
                        *mut libc::c_void,
                        *const libc::c_uchar,
                    ) -> *const libc::c_uchar,
            ),
            Some(
                unw_decode_p1
                    as unsafe extern "C" fn(
                        *const libc::c_uchar,
                        libc::c_uint,
                        *mut libc::c_void,
                        *const libc::c_uchar,
                    ) -> *const libc::c_uchar,
            ),
            Some(
                unw_decode_p2_p5
                    as unsafe extern "C" fn(
                        *const libc::c_uchar,
                        libc::c_uint,
                        *mut libc::c_void,
                        *const libc::c_uchar,
                    ) -> *const libc::c_uchar,
            ),
            Some(
                unw_decode_p6
                    as unsafe extern "C" fn(
                        *const libc::c_uchar,
                        libc::c_uint,
                        *mut libc::c_void,
                        *const libc::c_uchar,
                    ) -> *const libc::c_uchar,
            ),
            Some(
                unw_decode_p7_p10
                    as unsafe extern "C" fn(
                        *const libc::c_uchar,
                        libc::c_uint,
                        *mut libc::c_void,
                        *const libc::c_uchar,
                    ) -> *const libc::c_uchar,
            ),
        ],
        [
            Some(
                unw_decode_r1
                    as unsafe extern "C" fn(
                        *const libc::c_uchar,
                        libc::c_uint,
                        *mut libc::c_void,
                        *const libc::c_uchar,
                    ) -> *const libc::c_uchar,
            ),
            Some(
                unw_decode_r1
                    as unsafe extern "C" fn(
                        *const libc::c_uchar,
                        libc::c_uint,
                        *mut libc::c_void,
                        *const libc::c_uchar,
                    ) -> *const libc::c_uchar,
            ),
            Some(
                unw_decode_r2
                    as unsafe extern "C" fn(
                        *const libc::c_uchar,
                        libc::c_uint,
                        *mut libc::c_void,
                        *const libc::c_uchar,
                    ) -> *const libc::c_uchar,
            ),
            Some(
                unw_decode_r3
                    as unsafe extern "C" fn(
                        *const libc::c_uchar,
                        libc::c_uint,
                        *mut libc::c_void,
                        *const libc::c_uchar,
                    ) -> *const libc::c_uchar,
            ),
            Some(
                unw_decode_b1
                    as unsafe extern "C" fn(
                        *const libc::c_uchar,
                        libc::c_uint,
                        *mut libc::c_void,
                        *const libc::c_uchar,
                    ) -> *const libc::c_uchar,
            ),
            Some(
                unw_decode_b1
                    as unsafe extern "C" fn(
                        *const libc::c_uchar,
                        libc::c_uint,
                        *mut libc::c_void,
                        *const libc::c_uchar,
                    ) -> *const libc::c_uchar,
            ),
            Some(
                unw_decode_b2
                    as unsafe extern "C" fn(
                        *const libc::c_uchar,
                        libc::c_uint,
                        *mut libc::c_void,
                        *const libc::c_uchar,
                    ) -> *const libc::c_uchar,
            ),
            Some(
                unw_decode_b3_x4
                    as unsafe extern "C" fn(
                        *const libc::c_uchar,
                        libc::c_uint,
                        *mut libc::c_void,
                        *const libc::c_uchar,
                    ) -> *const libc::c_uchar,
            ),
        ],
    ]
};
