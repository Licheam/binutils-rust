extern "C" {
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn atof_generic(
        address_of_string_pointer: *mut *mut libc::c_char,
        string_of_decimal_marks: *const libc::c_char,
        string_of_decimal_exponent_marks: *const libc::c_char,
        address_of_generic_floating_point_number: *mut FLONUM_TYPE,
    ) -> libc::c_int;
    static mut generic_floating_point_number: FLONUM_TYPE;
    static EXP_CHARS: [libc::c_char; 0];
    fn number_to_chars_littleendian(_: *mut libc::c_char, _: valueT, _: libc::c_int);
    static mut input_line_pointer: *mut libc::c_char;
    static FLT_CHARS: [libc::c_char; 0];
    fn as_bad(format: *const libc::c_char, _: ...);
    fn as_warn(format: *const libc::c_char, _: ...);
    fn as_abort(_: *const libc::c_char, _: libc::c_int, _: *const libc::c_char) -> !;
}
pub type bfd_vma = libc::c_ulong;
pub type LITTLENUM_TYPE = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FLONUM_STRUCT {
    pub low: *mut LITTLENUM_TYPE,
    pub high: *mut LITTLENUM_TYPE,
    pub leader: *mut LITTLENUM_TYPE,
    pub exponent: libc::c_long,
    pub sign: libc::c_char,
}
pub type FLONUM_TYPE = FLONUM_STRUCT;
pub type addressT = bfd_vma;
pub type valueT = addressT;
#[no_mangle]
pub unsafe extern "C" fn ieee_md_atof(
    mut type_0: libc::c_int,
    mut litP: *mut libc::c_char,
    mut sizeP: *mut libc::c_int,
    mut big_wordian: bool,
) -> *const libc::c_char {
    let mut words: [LITTLENUM_TYPE; 6] = [0; 6];
    let mut wordP: *mut LITTLENUM_TYPE = 0 as *mut LITTLENUM_TYPE;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut prec: libc::c_int = 0 as libc::c_int;
    if !(strchr(FLT_CHARS.as_ptr(), type_0)).is_null() {
        match type_0 {
            72 | 104 => {
                prec = 1 as libc::c_int;
            }
            102 | 70 | 115 | 83 => {
                prec = 2 as libc::c_int;
            }
            100 | 68 | 114 | 82 => {
                prec = 4 as libc::c_int;
            }
            116 | 84 => {
                prec = 5 as libc::c_int;
                type_0 = 'x' as i32;
            }
            120 | 88 | 112 | 80 => {
                prec = 5 as libc::c_int;
            }
            _ => {}
        }
    } else if type_0 == 'f' as i32 {
        prec = 2 as libc::c_int;
    } else if type_0 == 'd' as i32 {
        prec = 4 as libc::c_int;
    }
    if prec == 0 as libc::c_int {
        *sizeP = 0 as libc::c_int;
        return dcgettext(
            0 as *const libc::c_char,
            b"Unrecognized or unsupported floating point constant\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        );
    }
    if prec <= 6 as libc::c_int {} else {
        as_abort(
            b"config/atof-ieee.c\0" as *const u8 as *const libc::c_char,
            832 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 52],
                &[libc::c_char; 52],
            >(b"const char *ieee_md_atof(int, char *, int *, _Bool)\0"))
                .as_ptr(),
        );
    };
    t = atof_ieee(input_line_pointer, type_0, words.as_mut_ptr());
    if !t.is_null() {
        input_line_pointer = t;
    }
    *sizeP = (prec as libc::c_ulong)
        .wrapping_mul(::core::mem::size_of::<LITTLENUM_TYPE>() as libc::c_ulong)
        as libc::c_int;
    if big_wordian {
        wordP = words.as_mut_ptr();
        loop {
            let fresh0 = prec;
            prec = prec - 1;
            if !(fresh0 != 0) {
                break;
            }
            let fresh1 = wordP;
            wordP = wordP.offset(1);
            number_to_chars_littleendian(
                litP,
                *fresh1 as valueT,
                ::core::mem::size_of::<LITTLENUM_TYPE>() as libc::c_ulong as libc::c_int,
            );
            litP = litP
                .offset(
                    ::core::mem::size_of::<LITTLENUM_TYPE>() as libc::c_ulong as isize,
                );
        }
    } else {
        wordP = words.as_mut_ptr().offset(prec as isize);
        loop {
            let fresh2 = prec;
            prec = prec - 1;
            if !(fresh2 != 0) {
                break;
            }
            wordP = wordP.offset(-1);
            number_to_chars_littleendian(
                litP,
                *wordP as valueT,
                ::core::mem::size_of::<LITTLENUM_TYPE>() as libc::c_ulong as libc::c_int,
            );
            litP = litP
                .offset(
                    ::core::mem::size_of::<LITTLENUM_TYPE>() as libc::c_ulong as isize,
                );
        }
    }
    return 0 as *const libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn atof_ieee_detail(
    mut str: *mut libc::c_char,
    mut precision: libc::c_int,
    mut exponent_bits: libc::c_int,
    mut words: *mut LITTLENUM_TYPE,
    mut generic_float_info: *mut FLONUM_TYPE,
) -> *mut libc::c_char {
    static mut bits: [LITTLENUM_TYPE; 12] = [0; 12];
    let mut return_value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut save_gen_flonum: FLONUM_TYPE = FLONUM_TYPE {
        low: 0 as *const LITTLENUM_TYPE as *mut LITTLENUM_TYPE,
        high: 0 as *const LITTLENUM_TYPE as *mut LITTLENUM_TYPE,
        leader: 0 as *const LITTLENUM_TYPE as *mut LITTLENUM_TYPE,
        exponent: 0,
        sign: 0,
    };
    save_gen_flonum = generic_floating_point_number;
    return_value = str;
    generic_floating_point_number
        .low = bits.as_mut_ptr().offset(5 as libc::c_int as isize);
    generic_floating_point_number.high = 0 as *mut LITTLENUM_TYPE;
    generic_floating_point_number.leader = 0 as *mut LITTLENUM_TYPE;
    generic_floating_point_number.exponent = 0 as libc::c_int as libc::c_long;
    generic_floating_point_number.sign = '\0' as i32 as libc::c_char;
    memset(
        bits.as_mut_ptr() as *mut libc::c_void,
        '\0' as i32,
        (::core::mem::size_of::<LITTLENUM_TYPE>() as libc::c_ulong)
            .wrapping_mul(5 as libc::c_int as libc::c_ulong),
    );
    generic_floating_point_number
        .high = (generic_floating_point_number.low)
        .offset(precision as isize)
        .offset(-(1 as libc::c_int as isize))
        .offset(2 as libc::c_int as isize);
    if atof_generic(
        &mut return_value,
        b".\0" as *const u8 as *const libc::c_char,
        EXP_CHARS.as_ptr(),
        &mut generic_floating_point_number,
    ) != 0
    {
        make_invalid_floating_point_number(words);
        return 0 as *mut libc::c_char;
    }
    if !generic_float_info.is_null() {
        *generic_float_info = generic_floating_point_number;
    }
    gen_to_words(words, precision, exponent_bits as libc::c_long);
    generic_floating_point_number = save_gen_flonum;
    return return_value;
}
#[no_mangle]
pub unsafe extern "C" fn atof_ieee(
    mut str: *mut libc::c_char,
    mut what_kind: libc::c_int,
    mut words: *mut LITTLENUM_TYPE,
) -> *mut libc::c_char {
    let mut precision: libc::c_int = 0;
    let mut exponent_bits: libc::c_long = 0;
    match what_kind {
        104 | 72 => {
            precision = 1 as libc::c_int;
            exponent_bits = 5 as libc::c_int as libc::c_long;
        }
        102 | 70 | 115 | 83 => {
            precision = 2 as libc::c_int;
            exponent_bits = 8 as libc::c_int as libc::c_long;
        }
        100 | 68 | 114 | 82 => {
            precision = 4 as libc::c_int;
            exponent_bits = 11 as libc::c_int as libc::c_long;
        }
        120 | 88 | 101 | 69 => {
            precision = 5 as libc::c_int;
            exponent_bits = 15 as libc::c_int as libc::c_long;
        }
        112 | 80 => {
            precision = 5 as libc::c_int;
            exponent_bits = -(1 as libc::c_int) as libc::c_long;
        }
        _ => {
            make_invalid_floating_point_number(words);
            return 0 as *mut libc::c_char;
        }
    }
    return atof_ieee_detail(
        str,
        precision,
        exponent_bits as libc::c_int,
        words,
        0 as *mut FLONUM_TYPE,
    );
}
#[no_mangle]
pub unsafe extern "C" fn gen_to_words(
    mut words: *mut LITTLENUM_TYPE,
    mut precision: libc::c_int,
    mut exponent_bits: libc::c_long,
) -> libc::c_int {
    let mut return_value: libc::c_int = 0 as libc::c_int;
    let mut exponent_1: libc::c_long = 0;
    let mut exponent_2: libc::c_long = 0;
    let mut exponent_3: libc::c_long = 0;
    let mut exponent_4: libc::c_long = 0;
    let mut exponent_skippage: libc::c_int = 0;
    let mut word1: LITTLENUM_TYPE = 0;
    let mut lp: *mut LITTLENUM_TYPE = 0 as *mut LITTLENUM_TYPE;
    let mut words_end: *mut LITTLENUM_TYPE = 0 as *mut LITTLENUM_TYPE;
    words_end = words.offset(precision as isize);
    if generic_floating_point_number.low > generic_floating_point_number.leader {
        if generic_floating_point_number.sign as libc::c_int == '+' as i32 {
            *words
                .offset(0 as libc::c_int as isize) = 0 as libc::c_int as LITTLENUM_TYPE;
        } else {
            *words
                .offset(
                    0 as libc::c_int as isize,
                ) = 0x8000 as libc::c_int as LITTLENUM_TYPE;
        }
        memset(
            &mut *words.offset(1 as libc::c_int as isize) as *mut LITTLENUM_TYPE
                as *mut libc::c_void,
            '\0' as i32,
            ((words_end.offset_from(words) as libc::c_long
                - 1 as libc::c_int as libc::c_long) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<LITTLENUM_TYPE>() as libc::c_ulong),
        );
        return return_value;
    }
    if generic_floating_point_number.sign as libc::c_int == 0 as libc::c_int {
        if precision == 1 as libc::c_int {
            *words
                .offset(
                    0 as libc::c_int as isize,
                ) = 0x7fff as libc::c_int as LITTLENUM_TYPE;
        } else if precision == 2 as libc::c_int {
            *words
                .offset(
                    0 as libc::c_int as isize,
                ) = 0x7fff as libc::c_int as LITTLENUM_TYPE;
            *words
                .offset(
                    1 as libc::c_int as isize,
                ) = 0xffff as libc::c_int as LITTLENUM_TYPE;
        } else if precision == 5 as libc::c_int {
            *words
                .offset(
                    0 as libc::c_int as isize,
                ) = 0xffff as libc::c_int as LITTLENUM_TYPE;
            *words
                .offset(
                    1 as libc::c_int as isize,
                ) = 0xc000 as libc::c_int as LITTLENUM_TYPE;
            *words
                .offset(2 as libc::c_int as isize) = 0 as libc::c_int as LITTLENUM_TYPE;
            *words
                .offset(3 as libc::c_int as isize) = 0 as libc::c_int as LITTLENUM_TYPE;
            *words
                .offset(4 as libc::c_int as isize) = 0 as libc::c_int as LITTLENUM_TYPE;
        } else {
            *words
                .offset(
                    0 as libc::c_int as isize,
                ) = 0x7fff as libc::c_int as LITTLENUM_TYPE;
            *words
                .offset(
                    1 as libc::c_int as isize,
                ) = 0xffff as libc::c_int as LITTLENUM_TYPE;
            *words
                .offset(
                    2 as libc::c_int as isize,
                ) = 0xffff as libc::c_int as LITTLENUM_TYPE;
            *words
                .offset(
                    3 as libc::c_int as isize,
                ) = 0xffff as libc::c_int as LITTLENUM_TYPE;
        }
        return return_value;
    } else if generic_floating_point_number.sign as libc::c_int == 'P' as i32 {
        if precision == 1 as libc::c_int {
            *words
                .offset(
                    0 as libc::c_int as isize,
                ) = 0x7c00 as libc::c_int as LITTLENUM_TYPE;
        } else if precision == 2 as libc::c_int {
            *words
                .offset(
                    0 as libc::c_int as isize,
                ) = 0x7f80 as libc::c_int as LITTLENUM_TYPE;
            *words
                .offset(1 as libc::c_int as isize) = 0 as libc::c_int as LITTLENUM_TYPE;
        } else if precision == 5 as libc::c_int {
            *words
                .offset(
                    0 as libc::c_int as isize,
                ) = 0x7fff as libc::c_int as LITTLENUM_TYPE;
            *words
                .offset(
                    1 as libc::c_int as isize,
                ) = 0x8000 as libc::c_int as LITTLENUM_TYPE;
            *words
                .offset(2 as libc::c_int as isize) = 0 as libc::c_int as LITTLENUM_TYPE;
            *words
                .offset(3 as libc::c_int as isize) = 0 as libc::c_int as LITTLENUM_TYPE;
            *words
                .offset(4 as libc::c_int as isize) = 0 as libc::c_int as LITTLENUM_TYPE;
        } else {
            *words
                .offset(
                    0 as libc::c_int as isize,
                ) = 0x7ff0 as libc::c_int as LITTLENUM_TYPE;
            *words
                .offset(1 as libc::c_int as isize) = 0 as libc::c_int as LITTLENUM_TYPE;
            *words
                .offset(2 as libc::c_int as isize) = 0 as libc::c_int as LITTLENUM_TYPE;
            *words
                .offset(3 as libc::c_int as isize) = 0 as libc::c_int as LITTLENUM_TYPE;
        }
        return return_value;
    } else if generic_floating_point_number.sign as libc::c_int == 'N' as i32 {
        if precision == 1 as libc::c_int {
            *words
                .offset(
                    0 as libc::c_int as isize,
                ) = 0xfc00 as libc::c_int as LITTLENUM_TYPE;
        } else if precision == 2 as libc::c_int {
            *words
                .offset(
                    0 as libc::c_int as isize,
                ) = 0xff80 as libc::c_int as LITTLENUM_TYPE;
            *words
                .offset(1 as libc::c_int as isize) = 0 as libc::c_int as LITTLENUM_TYPE;
        } else if precision == 5 as libc::c_int {
            *words
                .offset(
                    0 as libc::c_int as isize,
                ) = 0xffff as libc::c_int as LITTLENUM_TYPE;
            *words
                .offset(
                    1 as libc::c_int as isize,
                ) = 0x8000 as libc::c_int as LITTLENUM_TYPE;
            *words
                .offset(2 as libc::c_int as isize) = 0 as libc::c_int as LITTLENUM_TYPE;
            *words
                .offset(3 as libc::c_int as isize) = 0 as libc::c_int as LITTLENUM_TYPE;
            *words
                .offset(4 as libc::c_int as isize) = 0 as libc::c_int as LITTLENUM_TYPE;
        } else {
            *words
                .offset(
                    0 as libc::c_int as isize,
                ) = 0xfff0 as libc::c_int as LITTLENUM_TYPE;
            *words
                .offset(1 as libc::c_int as isize) = 0 as libc::c_int as LITTLENUM_TYPE;
            *words
                .offset(2 as libc::c_int as isize) = 0 as libc::c_int as LITTLENUM_TYPE;
            *words
                .offset(3 as libc::c_int as isize) = 0 as libc::c_int as LITTLENUM_TYPE;
        }
        return return_value;
    }
    bits_left_in_littlenum = 16 as libc::c_int;
    littlenum_pointer = generic_floating_point_number.leader;
    littlenums_left = (generic_floating_point_number.leader)
        .offset(1 as libc::c_int as isize)
        .offset_from(generic_floating_point_number.low) as libc::c_long as libc::c_int;
    exponent_skippage = 0 as libc::c_int;
    while next_bits(1 as libc::c_int) == 0 {
        exponent_skippage += 1;
        exponent_skippage;
    }
    exponent_1 = (generic_floating_point_number.leader)
        .offset(generic_floating_point_number.exponent as isize)
        .offset(1 as libc::c_int as isize)
        .offset_from(generic_floating_point_number.low) as libc::c_long;
    exponent_2 = exponent_1 * 16 as libc::c_int as libc::c_long;
    exponent_3 = exponent_2 - exponent_skippage as libc::c_long;
    exponent_4 = exponent_3
        + (((1 as libc::c_int) << exponent_bits - 1 as libc::c_int as libc::c_long)
            - 2 as libc::c_int) as libc::c_long;
    lp = words;
    word1 = (if generic_floating_point_number.sign as libc::c_int == '+' as i32 {
        0 as libc::c_int
    } else {
        (1 as libc::c_int) << 16 as libc::c_int - 1 as libc::c_int
    }) as LITTLENUM_TYPE;
    if exponent_4 <= 0 as libc::c_int as libc::c_long {
        let mut prec_bits: libc::c_int = 0;
        let mut num_bits: libc::c_int = 0;
        unget_bits(1 as libc::c_int);
        num_bits = -exponent_4 as libc::c_int;
        prec_bits = ((16 as libc::c_int * precision) as libc::c_long
            - (exponent_bits + 1 as libc::c_int as libc::c_long
                + num_bits as libc::c_long)) as libc::c_int;
        if precision == 5 as libc::c_int
            && exponent_bits == 15 as libc::c_int as libc::c_long
        {
            prec_bits -= 1 as libc::c_int;
            num_bits += 1 as libc::c_int;
        }
        if num_bits as libc::c_long >= 16 as libc::c_int as libc::c_long - exponent_bits
        {
            num_bits = (num_bits as libc::c_long
                - ((16 as libc::c_int - 1 as libc::c_int) as libc::c_long
                    - exponent_bits)) as libc::c_int;
            let fresh3 = lp;
            lp = lp.offset(1);
            *fresh3 = word1;
            if num_bits as libc::c_long + exponent_bits
                + 1 as libc::c_int as libc::c_long
                > (precision * 16 as libc::c_int) as libc::c_long
            {
                make_invalid_floating_point_number(words);
                return return_value;
            }
            while num_bits >= 16 as libc::c_int {
                num_bits -= 16 as libc::c_int;
                let fresh4 = lp;
                lp = lp.offset(1);
                *fresh4 = 0 as libc::c_int as LITTLENUM_TYPE;
            }
            if num_bits != 0 {
                let fresh5 = lp;
                lp = lp.offset(1);
                *fresh5 = next_bits(16 as libc::c_int - num_bits) as LITTLENUM_TYPE;
            }
        } else if precision == 5 as libc::c_int
            && exponent_bits == 15 as libc::c_int as libc::c_long
        {
            let fresh6 = lp;
            lp = lp.offset(1);
            *fresh6 = word1;
            let fresh7 = lp;
            lp = lp.offset(1);
            *fresh7 = next_bits(16 as libc::c_int - num_bits) as LITTLENUM_TYPE;
        } else {
            word1 = (word1 as libc::c_int
                | next_bits(
                    ((16 as libc::c_int - 1 as libc::c_int) as libc::c_long
                        - (exponent_bits + num_bits as libc::c_long)) as libc::c_int,
                )) as LITTLENUM_TYPE;
            let fresh8 = lp;
            lp = lp.offset(1);
            *fresh8 = word1;
        }
        while lp < words_end {
            let fresh9 = lp;
            lp = lp.offset(1);
            *fresh9 = next_bits(16 as libc::c_int) as LITTLENUM_TYPE;
        }
        if next_bits(1 as libc::c_int) != 0 {
            lp = lp.offset(-1);
            lp;
            if prec_bits >= 16 as libc::c_int {
                let mut n: libc::c_int = 0 as libc::c_int;
                let mut tmp_bits: libc::c_int = 0;
                n = 0 as libc::c_int;
                tmp_bits = prec_bits;
                while tmp_bits > 16 as libc::c_int {
                    if *lp.offset(n as isize) as libc::c_int
                        != -(1 as libc::c_int) as LITTLENUM_TYPE as libc::c_int
                    {
                        break;
                    }
                    n -= 1;
                    n;
                    tmp_bits -= 16 as libc::c_int;
                }
                if tmp_bits > 16 as libc::c_int
                    || *lp.offset(n as isize) as libc::c_ulong & mask[tmp_bits as usize]
                        != mask[tmp_bits as usize]
                    || prec_bits as libc::c_long
                        != (precision * 16 as libc::c_int) as libc::c_long
                            - exponent_bits - 1 as libc::c_int as libc::c_long
                        && !(precision == 5 as libc::c_int
                            && prec_bits as libc::c_long
                                == (precision * 16 as libc::c_int) as libc::c_long
                                    - exponent_bits - 2 as libc::c_int as libc::c_long)
                {
                    let mut carry: libc::c_ulong = 0;
                    carry = 1 as libc::c_int as libc::c_ulong;
                    while carry != 0 && lp >= words {
                        carry = (*lp as libc::c_ulong).wrapping_add(carry);
                        *lp = carry as LITTLENUM_TYPE;
                        carry >>= 16 as libc::c_int;
                        lp = lp.offset(-1);
                        lp;
                    }
                } else {
                    lp = words;
                    word1 = (if generic_floating_point_number.sign as libc::c_int
                        == '+' as i32
                    {
                        0 as libc::c_int
                    } else {
                        (1 as libc::c_int) << 16 as libc::c_int - 1 as libc::c_int
                    }) as LITTLENUM_TYPE;
                    word1 = (word1 as libc::c_int
                        | (1 as libc::c_int)
                            << (16 as libc::c_int - 1 as libc::c_int) as libc::c_long
                                - exponent_bits) as LITTLENUM_TYPE;
                    let fresh10 = lp;
                    lp = lp.offset(1);
                    *fresh10 = word1;
                    if precision == 5 as libc::c_int {
                        let fresh11 = lp;
                        lp = lp.offset(1);
                        *fresh11 = ((1 as libc::c_int)
                            << 16 as libc::c_int - 1 as libc::c_int) as LITTLENUM_TYPE;
                    }
                    while lp < words_end {
                        let fresh12 = lp;
                        lp = lp.offset(1);
                        *fresh12 = 0 as libc::c_int as LITTLENUM_TYPE;
                    }
                }
            } else {
                *lp = (*lp as libc::c_int + 1 as libc::c_int) as LITTLENUM_TYPE;
            }
        }
        return return_value;
    } else if exponent_4 as libc::c_ulong > mask[exponent_bits as usize]
        || 0 as libc::c_int == 0
            && exponent_4 as libc::c_ulong == mask[exponent_bits as usize]
    {
        make_invalid_floating_point_number(words);
        return return_value;
    } else {
        word1 = (word1 as libc::c_long
            | (exponent_4
                << (16 as libc::c_int - 1 as libc::c_int) as libc::c_long - exponent_bits
                | next_bits(
                    ((16 as libc::c_int - 1 as libc::c_int) as libc::c_long
                        - exponent_bits) as libc::c_int,
                ) as libc::c_long)) as LITTLENUM_TYPE;
    }
    let fresh13 = lp;
    lp = lp.offset(1);
    *fresh13 = word1;
    if exponent_bits == 15 as libc::c_int as libc::c_long
        && precision == 5 as libc::c_int
    {
        let fresh14 = lp;
        lp = lp.offset(1);
        *fresh14 = ((1 as libc::c_int) << 16 as libc::c_int - 1 as libc::c_int
            | next_bits(16 as libc::c_int - 1 as libc::c_int)) as LITTLENUM_TYPE;
    }
    while lp < words_end {
        let fresh15 = lp;
        lp = lp.offset(1);
        *fresh15 = next_bits(16 as libc::c_int) as LITTLENUM_TYPE;
    }
    if next_bits(1 as libc::c_int) != 0 {
        let mut carry_0: libc::c_ulong = 0;
        carry_0 = 1 as libc::c_int as libc::c_ulong;
        lp = lp.offset(-1);
        lp;
        while carry_0 != 0 {
            carry_0 = (*lp as libc::c_ulong).wrapping_add(carry_0);
            *lp = carry_0 as LITTLENUM_TYPE;
            carry_0 >>= 16 as libc::c_int;
            if lp == words {
                break;
            }
            lp = lp.offset(-1);
            lp;
        }
        if precision == 5 as libc::c_int
            && exponent_bits == 15 as libc::c_int as libc::c_long
        {
            if lp == words {
                let ref mut fresh16 = *lp.offset(1 as libc::c_int as isize);
                *fresh16 = (*fresh16 as libc::c_int
                    | (1 as libc::c_int) << 16 as libc::c_int - 1 as libc::c_int)
                    as LITTLENUM_TYPE;
            }
        }
        if (word1 as libc::c_int ^ *words as libc::c_int)
            & (1 as libc::c_int) << 16 as libc::c_int - 1 as libc::c_int != 0
        {
            *words = (*words as libc::c_int
                & !((1 as libc::c_int) << 16 as libc::c_int - 1 as libc::c_int))
                as LITTLENUM_TYPE;
        }
    }
    return return_value;
}
static mut mask: [libc::c_ulong; 33] = [
    0 as libc::c_int as libc::c_ulong,
    0x1 as libc::c_int as libc::c_ulong,
    0x3 as libc::c_int as libc::c_ulong,
    0x7 as libc::c_int as libc::c_ulong,
    0xf as libc::c_int as libc::c_ulong,
    0x1f as libc::c_int as libc::c_ulong,
    0x3f as libc::c_int as libc::c_ulong,
    0x7f as libc::c_int as libc::c_ulong,
    0xff as libc::c_int as libc::c_ulong,
    0x1ff as libc::c_int as libc::c_ulong,
    0x3ff as libc::c_int as libc::c_ulong,
    0x7ff as libc::c_int as libc::c_ulong,
    0xfff as libc::c_int as libc::c_ulong,
    0x1fff as libc::c_int as libc::c_ulong,
    0x3fff as libc::c_int as libc::c_ulong,
    0x7fff as libc::c_int as libc::c_ulong,
    0xffff as libc::c_int as libc::c_ulong,
    0x1ffff as libc::c_int as libc::c_ulong,
    0x3ffff as libc::c_int as libc::c_ulong,
    0x7ffff as libc::c_int as libc::c_ulong,
    0xfffff as libc::c_int as libc::c_ulong,
    0x1fffff as libc::c_int as libc::c_ulong,
    0x3fffff as libc::c_int as libc::c_ulong,
    0x7fffff as libc::c_int as libc::c_ulong,
    0xffffff as libc::c_int as libc::c_ulong,
    0x1ffffff as libc::c_int as libc::c_ulong,
    0x3ffffff as libc::c_int as libc::c_ulong,
    0x7ffffff as libc::c_int as libc::c_ulong,
    0xfffffff as libc::c_int as libc::c_ulong,
    0x1fffffff as libc::c_int as libc::c_ulong,
    0x3fffffff as libc::c_int as libc::c_ulong,
    0x7fffffff as libc::c_int as libc::c_ulong,
    0xffffffff as libc::c_uint as libc::c_ulong,
];
static mut bits_left_in_littlenum: libc::c_int = 0;
static mut littlenums_left: libc::c_int = 0;
static mut littlenum_pointer: *mut LITTLENUM_TYPE = 0 as *const LITTLENUM_TYPE
    as *mut LITTLENUM_TYPE;
unsafe extern "C" fn next_bits(mut number_of_bits: libc::c_int) -> libc::c_int {
    let mut return_value: libc::c_int = 0;
    if littlenums_left == 0 {
        return 0 as libc::c_int;
    }
    if number_of_bits >= bits_left_in_littlenum {
        return_value = (mask[bits_left_in_littlenum as usize]
            & *littlenum_pointer as libc::c_ulong) as libc::c_int;
        number_of_bits -= bits_left_in_littlenum;
        return_value <<= number_of_bits;
        littlenums_left -= 1;
        if littlenums_left != 0 {
            bits_left_in_littlenum = 16 as libc::c_int - number_of_bits;
            littlenum_pointer = littlenum_pointer.offset(-1);
            littlenum_pointer;
            return_value = (return_value as libc::c_ulong
                | (*littlenum_pointer as libc::c_int >> bits_left_in_littlenum)
                    as libc::c_ulong & mask[number_of_bits as usize]) as libc::c_int;
        }
    } else {
        bits_left_in_littlenum -= number_of_bits;
        return_value = (mask[number_of_bits as usize]
            & (*littlenum_pointer as libc::c_int >> bits_left_in_littlenum)
                as libc::c_ulong) as libc::c_int;
    }
    return return_value;
}
unsafe extern "C" fn unget_bits(mut num: libc::c_int) {
    if littlenums_left == 0 {
        littlenum_pointer = littlenum_pointer.offset(1);
        littlenum_pointer;
        littlenums_left += 1;
        littlenums_left;
        bits_left_in_littlenum = num;
    } else if bits_left_in_littlenum + num > 16 as libc::c_int {
        bits_left_in_littlenum = num - (16 as libc::c_int - bits_left_in_littlenum);
        littlenum_pointer = littlenum_pointer.offset(1);
        littlenum_pointer;
        littlenums_left += 1;
        littlenums_left;
    } else {
        bits_left_in_littlenum += num;
    };
}
unsafe extern "C" fn make_invalid_floating_point_number(mut words: *mut LITTLENUM_TYPE) {
    as_bad(
        dcgettext(
            0 as *const libc::c_char,
            b"cannot create floating-point number\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    *words
        .offset(
            0 as libc::c_int as isize,
        ) = (-(1 as libc::c_int) as libc::c_uint as LITTLENUM_TYPE as libc::c_int
        >> 1 as libc::c_int) as LITTLENUM_TYPE;
    *words.offset(1 as libc::c_int as isize) = -(1 as libc::c_int) as LITTLENUM_TYPE;
    *words.offset(2 as libc::c_int as isize) = -(1 as libc::c_int) as LITTLENUM_TYPE;
    *words.offset(3 as libc::c_int as isize) = -(1 as libc::c_int) as LITTLENUM_TYPE;
    *words.offset(4 as libc::c_int as isize) = -(1 as libc::c_int) as LITTLENUM_TYPE;
    *words.offset(5 as libc::c_int as isize) = -(1 as libc::c_int) as LITTLENUM_TYPE;
}
