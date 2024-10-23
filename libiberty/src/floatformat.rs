use ::libc;
extern "C" {
    fn frexp(_: libc::c_double, _: *mut libc::c_int) -> libc::c_double;
    fn ldexp(_: libc::c_double, _: libc::c_int) -> libc::c_double;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
pub type floatformat_byteorders = libc::c_uint;
pub const floatformat_vax: floatformat_byteorders = 3;
pub const floatformat_littlebyte_bigword: floatformat_byteorders = 2;
pub const floatformat_big: floatformat_byteorders = 1;
pub const floatformat_little: floatformat_byteorders = 0;
pub type floatformat_intbit = libc::c_uint;
pub const floatformat_intbit_no: floatformat_intbit = 1;
pub const floatformat_intbit_yes: floatformat_intbit = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct floatformat {
    pub byteorder: floatformat_byteorders,
    pub totalsize: libc::c_uint,
    pub sign_start: libc::c_uint,
    pub exp_start: libc::c_uint,
    pub exp_len: libc::c_uint,
    pub exp_bias: libc::c_int,
    pub exp_nan: libc::c_uint,
    pub man_start: libc::c_uint,
    pub man_len: libc::c_uint,
    pub intbit: floatformat_intbit,
    pub name: *const libc::c_char,
    pub is_valid: Option::<
        unsafe extern "C" fn(*const floatformat, *const libc::c_void) -> libc::c_int,
    >,
    pub split_half: *const floatformat,
}
unsafe extern "C" fn floatformat_always_valid(
    mut fmt: *const floatformat,
    mut from: *const libc::c_void,
) -> libc::c_int {
    return 1 as libc::c_int;
}
#[no_mangle]
pub static mut floatformat_ieee_half_big: floatformat = unsafe {
    {
        let mut init = floatformat {
            byteorder: floatformat_big,
            totalsize: 16 as libc::c_int as libc::c_uint,
            sign_start: 0 as libc::c_int as libc::c_uint,
            exp_start: 1 as libc::c_int as libc::c_uint,
            exp_len: 5 as libc::c_int as libc::c_uint,
            exp_bias: 15 as libc::c_int,
            exp_nan: 31 as libc::c_int as libc::c_uint,
            man_start: 6 as libc::c_int as libc::c_uint,
            man_len: 10 as libc::c_int as libc::c_uint,
            intbit: floatformat_intbit_no,
            name: b"floatformat_ieee_half_big\0" as *const u8 as *const libc::c_char,
            is_valid: Some(
                floatformat_always_valid
                    as unsafe extern "C" fn(
                        *const floatformat,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            split_half: 0 as *const floatformat,
        };
        init
    }
};
#[no_mangle]
pub static mut floatformat_ieee_half_little: floatformat = unsafe {
    {
        let mut init = floatformat {
            byteorder: floatformat_little,
            totalsize: 16 as libc::c_int as libc::c_uint,
            sign_start: 0 as libc::c_int as libc::c_uint,
            exp_start: 1 as libc::c_int as libc::c_uint,
            exp_len: 5 as libc::c_int as libc::c_uint,
            exp_bias: 15 as libc::c_int,
            exp_nan: 31 as libc::c_int as libc::c_uint,
            man_start: 6 as libc::c_int as libc::c_uint,
            man_len: 10 as libc::c_int as libc::c_uint,
            intbit: floatformat_intbit_no,
            name: b"floatformat_ieee_half_little\0" as *const u8 as *const libc::c_char,
            is_valid: Some(
                floatformat_always_valid
                    as unsafe extern "C" fn(
                        *const floatformat,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            split_half: 0 as *const floatformat,
        };
        init
    }
};
#[no_mangle]
pub static mut floatformat_ieee_single_big: floatformat = unsafe {
    {
        let mut init = floatformat {
            byteorder: floatformat_big,
            totalsize: 32 as libc::c_int as libc::c_uint,
            sign_start: 0 as libc::c_int as libc::c_uint,
            exp_start: 1 as libc::c_int as libc::c_uint,
            exp_len: 8 as libc::c_int as libc::c_uint,
            exp_bias: 127 as libc::c_int,
            exp_nan: 255 as libc::c_int as libc::c_uint,
            man_start: 9 as libc::c_int as libc::c_uint,
            man_len: 23 as libc::c_int as libc::c_uint,
            intbit: floatformat_intbit_no,
            name: b"floatformat_ieee_single_big\0" as *const u8 as *const libc::c_char,
            is_valid: Some(
                floatformat_always_valid
                    as unsafe extern "C" fn(
                        *const floatformat,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            split_half: 0 as *const floatformat,
        };
        init
    }
};
#[no_mangle]
pub static mut floatformat_ieee_single_little: floatformat = unsafe {
    {
        let mut init = floatformat {
            byteorder: floatformat_little,
            totalsize: 32 as libc::c_int as libc::c_uint,
            sign_start: 0 as libc::c_int as libc::c_uint,
            exp_start: 1 as libc::c_int as libc::c_uint,
            exp_len: 8 as libc::c_int as libc::c_uint,
            exp_bias: 127 as libc::c_int,
            exp_nan: 255 as libc::c_int as libc::c_uint,
            man_start: 9 as libc::c_int as libc::c_uint,
            man_len: 23 as libc::c_int as libc::c_uint,
            intbit: floatformat_intbit_no,
            name: b"floatformat_ieee_single_little\0" as *const u8
                as *const libc::c_char,
            is_valid: Some(
                floatformat_always_valid
                    as unsafe extern "C" fn(
                        *const floatformat,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            split_half: 0 as *const floatformat,
        };
        init
    }
};
#[no_mangle]
pub static mut floatformat_ieee_double_big: floatformat = unsafe {
    {
        let mut init = floatformat {
            byteorder: floatformat_big,
            totalsize: 64 as libc::c_int as libc::c_uint,
            sign_start: 0 as libc::c_int as libc::c_uint,
            exp_start: 1 as libc::c_int as libc::c_uint,
            exp_len: 11 as libc::c_int as libc::c_uint,
            exp_bias: 1023 as libc::c_int,
            exp_nan: 2047 as libc::c_int as libc::c_uint,
            man_start: 12 as libc::c_int as libc::c_uint,
            man_len: 52 as libc::c_int as libc::c_uint,
            intbit: floatformat_intbit_no,
            name: b"floatformat_ieee_double_big\0" as *const u8 as *const libc::c_char,
            is_valid: Some(
                floatformat_always_valid
                    as unsafe extern "C" fn(
                        *const floatformat,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            split_half: 0 as *const floatformat,
        };
        init
    }
};
#[no_mangle]
pub static mut floatformat_ieee_double_little: floatformat = unsafe {
    {
        let mut init = floatformat {
            byteorder: floatformat_little,
            totalsize: 64 as libc::c_int as libc::c_uint,
            sign_start: 0 as libc::c_int as libc::c_uint,
            exp_start: 1 as libc::c_int as libc::c_uint,
            exp_len: 11 as libc::c_int as libc::c_uint,
            exp_bias: 1023 as libc::c_int,
            exp_nan: 2047 as libc::c_int as libc::c_uint,
            man_start: 12 as libc::c_int as libc::c_uint,
            man_len: 52 as libc::c_int as libc::c_uint,
            intbit: floatformat_intbit_no,
            name: b"floatformat_ieee_double_little\0" as *const u8
                as *const libc::c_char,
            is_valid: Some(
                floatformat_always_valid
                    as unsafe extern "C" fn(
                        *const floatformat,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            split_half: 0 as *const floatformat,
        };
        init
    }
};
#[no_mangle]
pub static mut floatformat_ieee_double_littlebyte_bigword: floatformat = unsafe {
    {
        let mut init = floatformat {
            byteorder: floatformat_littlebyte_bigword,
            totalsize: 64 as libc::c_int as libc::c_uint,
            sign_start: 0 as libc::c_int as libc::c_uint,
            exp_start: 1 as libc::c_int as libc::c_uint,
            exp_len: 11 as libc::c_int as libc::c_uint,
            exp_bias: 1023 as libc::c_int,
            exp_nan: 2047 as libc::c_int as libc::c_uint,
            man_start: 12 as libc::c_int as libc::c_uint,
            man_len: 52 as libc::c_int as libc::c_uint,
            intbit: floatformat_intbit_no,
            name: b"floatformat_ieee_double_littlebyte_bigword\0" as *const u8
                as *const libc::c_char,
            is_valid: Some(
                floatformat_always_valid
                    as unsafe extern "C" fn(
                        *const floatformat,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            split_half: 0 as *const floatformat,
        };
        init
    }
};
#[no_mangle]
pub static mut floatformat_vax_f: floatformat = unsafe {
    {
        let mut init = floatformat {
            byteorder: floatformat_vax,
            totalsize: 32 as libc::c_int as libc::c_uint,
            sign_start: 0 as libc::c_int as libc::c_uint,
            exp_start: 1 as libc::c_int as libc::c_uint,
            exp_len: 8 as libc::c_int as libc::c_uint,
            exp_bias: 129 as libc::c_int,
            exp_nan: 0 as libc::c_int as libc::c_uint,
            man_start: 9 as libc::c_int as libc::c_uint,
            man_len: 23 as libc::c_int as libc::c_uint,
            intbit: floatformat_intbit_no,
            name: b"floatformat_vax_f\0" as *const u8 as *const libc::c_char,
            is_valid: Some(
                floatformat_always_valid
                    as unsafe extern "C" fn(
                        *const floatformat,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            split_half: 0 as *const floatformat,
        };
        init
    }
};
#[no_mangle]
pub static mut floatformat_vax_d: floatformat = unsafe {
    {
        let mut init = floatformat {
            byteorder: floatformat_vax,
            totalsize: 64 as libc::c_int as libc::c_uint,
            sign_start: 0 as libc::c_int as libc::c_uint,
            exp_start: 1 as libc::c_int as libc::c_uint,
            exp_len: 8 as libc::c_int as libc::c_uint,
            exp_bias: 129 as libc::c_int,
            exp_nan: 0 as libc::c_int as libc::c_uint,
            man_start: 9 as libc::c_int as libc::c_uint,
            man_len: 55 as libc::c_int as libc::c_uint,
            intbit: floatformat_intbit_no,
            name: b"floatformat_vax_d\0" as *const u8 as *const libc::c_char,
            is_valid: Some(
                floatformat_always_valid
                    as unsafe extern "C" fn(
                        *const floatformat,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            split_half: 0 as *const floatformat,
        };
        init
    }
};
#[no_mangle]
pub static mut floatformat_vax_g: floatformat = unsafe {
    {
        let mut init = floatformat {
            byteorder: floatformat_vax,
            totalsize: 64 as libc::c_int as libc::c_uint,
            sign_start: 0 as libc::c_int as libc::c_uint,
            exp_start: 1 as libc::c_int as libc::c_uint,
            exp_len: 11 as libc::c_int as libc::c_uint,
            exp_bias: 1025 as libc::c_int,
            exp_nan: 0 as libc::c_int as libc::c_uint,
            man_start: 12 as libc::c_int as libc::c_uint,
            man_len: 52 as libc::c_int as libc::c_uint,
            intbit: floatformat_intbit_no,
            name: b"floatformat_vax_g\0" as *const u8 as *const libc::c_char,
            is_valid: Some(
                floatformat_always_valid
                    as unsafe extern "C" fn(
                        *const floatformat,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            split_half: 0 as *const floatformat,
        };
        init
    }
};
unsafe extern "C" fn floatformat_i387_ext_is_valid(
    mut fmt: *const floatformat,
    mut from: *const libc::c_void,
) -> libc::c_int {
    let mut exponent: libc::c_ulong = 0;
    let mut int_bit: libc::c_ulong = 0;
    let mut ufrom: *const libc::c_uchar = from as *const libc::c_uchar;
    exponent = get_field(
        ufrom,
        (*fmt).byteorder,
        (*fmt).totalsize,
        (*fmt).exp_start,
        (*fmt).exp_len,
    );
    int_bit = get_field(
        ufrom,
        (*fmt).byteorder,
        (*fmt).totalsize,
        (*fmt).man_start,
        1 as libc::c_int as libc::c_uint,
    );
    if (exponent == 0 as libc::c_int as libc::c_ulong) as libc::c_int
        != (int_bit == 0 as libc::c_int as libc::c_ulong) as libc::c_int
    {
        return 0 as libc::c_int
    } else {
        return 1 as libc::c_int
    };
}
#[no_mangle]
pub static mut floatformat_i387_ext: floatformat = unsafe {
    {
        let mut init = floatformat {
            byteorder: floatformat_little,
            totalsize: 80 as libc::c_int as libc::c_uint,
            sign_start: 0 as libc::c_int as libc::c_uint,
            exp_start: 1 as libc::c_int as libc::c_uint,
            exp_len: 15 as libc::c_int as libc::c_uint,
            exp_bias: 0x3fff as libc::c_int,
            exp_nan: 0x7fff as libc::c_int as libc::c_uint,
            man_start: 16 as libc::c_int as libc::c_uint,
            man_len: 64 as libc::c_int as libc::c_uint,
            intbit: floatformat_intbit_yes,
            name: b"floatformat_i387_ext\0" as *const u8 as *const libc::c_char,
            is_valid: Some(
                floatformat_i387_ext_is_valid
                    as unsafe extern "C" fn(
                        *const floatformat,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            split_half: 0 as *const floatformat,
        };
        init
    }
};
#[no_mangle]
pub static mut floatformat_m68881_ext: floatformat = unsafe {
    {
        let mut init = floatformat {
            byteorder: floatformat_big,
            totalsize: 96 as libc::c_int as libc::c_uint,
            sign_start: 0 as libc::c_int as libc::c_uint,
            exp_start: 1 as libc::c_int as libc::c_uint,
            exp_len: 15 as libc::c_int as libc::c_uint,
            exp_bias: 0x3fff as libc::c_int,
            exp_nan: 0x7fff as libc::c_int as libc::c_uint,
            man_start: 32 as libc::c_int as libc::c_uint,
            man_len: 64 as libc::c_int as libc::c_uint,
            intbit: floatformat_intbit_yes,
            name: b"floatformat_m68881_ext\0" as *const u8 as *const libc::c_char,
            is_valid: Some(
                floatformat_always_valid
                    as unsafe extern "C" fn(
                        *const floatformat,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            split_half: 0 as *const floatformat,
        };
        init
    }
};
#[no_mangle]
pub static mut floatformat_i960_ext: floatformat = unsafe {
    {
        let mut init = floatformat {
            byteorder: floatformat_little,
            totalsize: 96 as libc::c_int as libc::c_uint,
            sign_start: 16 as libc::c_int as libc::c_uint,
            exp_start: 17 as libc::c_int as libc::c_uint,
            exp_len: 15 as libc::c_int as libc::c_uint,
            exp_bias: 0x3fff as libc::c_int,
            exp_nan: 0x7fff as libc::c_int as libc::c_uint,
            man_start: 32 as libc::c_int as libc::c_uint,
            man_len: 64 as libc::c_int as libc::c_uint,
            intbit: floatformat_intbit_yes,
            name: b"floatformat_i960_ext\0" as *const u8 as *const libc::c_char,
            is_valid: Some(
                floatformat_always_valid
                    as unsafe extern "C" fn(
                        *const floatformat,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            split_half: 0 as *const floatformat,
        };
        init
    }
};
#[no_mangle]
pub static mut floatformat_m88110_ext: floatformat = unsafe {
    {
        let mut init = floatformat {
            byteorder: floatformat_big,
            totalsize: 80 as libc::c_int as libc::c_uint,
            sign_start: 0 as libc::c_int as libc::c_uint,
            exp_start: 1 as libc::c_int as libc::c_uint,
            exp_len: 15 as libc::c_int as libc::c_uint,
            exp_bias: 0x3fff as libc::c_int,
            exp_nan: 0x7fff as libc::c_int as libc::c_uint,
            man_start: 16 as libc::c_int as libc::c_uint,
            man_len: 64 as libc::c_int as libc::c_uint,
            intbit: floatformat_intbit_yes,
            name: b"floatformat_m88110_ext\0" as *const u8 as *const libc::c_char,
            is_valid: Some(
                floatformat_always_valid
                    as unsafe extern "C" fn(
                        *const floatformat,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            split_half: 0 as *const floatformat,
        };
        init
    }
};
#[no_mangle]
pub static mut floatformat_m88110_harris_ext: floatformat = unsafe {
    {
        let mut init = floatformat {
            byteorder: floatformat_big,
            totalsize: 128 as libc::c_int as libc::c_uint,
            sign_start: 0 as libc::c_int as libc::c_uint,
            exp_start: 1 as libc::c_int as libc::c_uint,
            exp_len: 11 as libc::c_int as libc::c_uint,
            exp_bias: 0x3ff as libc::c_int,
            exp_nan: 0x7ff as libc::c_int as libc::c_uint,
            man_start: 12 as libc::c_int as libc::c_uint,
            man_len: 52 as libc::c_int as libc::c_uint,
            intbit: floatformat_intbit_no,
            name: b"floatformat_m88110_ext_harris\0" as *const u8 as *const libc::c_char,
            is_valid: Some(
                floatformat_always_valid
                    as unsafe extern "C" fn(
                        *const floatformat,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            split_half: 0 as *const floatformat,
        };
        init
    }
};
#[no_mangle]
pub static mut floatformat_arm_ext_big: floatformat = unsafe {
    {
        let mut init = floatformat {
            byteorder: floatformat_big,
            totalsize: 96 as libc::c_int as libc::c_uint,
            sign_start: 0 as libc::c_int as libc::c_uint,
            exp_start: 17 as libc::c_int as libc::c_uint,
            exp_len: 15 as libc::c_int as libc::c_uint,
            exp_bias: 0x3fff as libc::c_int,
            exp_nan: 0x7fff as libc::c_int as libc::c_uint,
            man_start: 32 as libc::c_int as libc::c_uint,
            man_len: 64 as libc::c_int as libc::c_uint,
            intbit: floatformat_intbit_yes,
            name: b"floatformat_arm_ext_big\0" as *const u8 as *const libc::c_char,
            is_valid: Some(
                floatformat_always_valid
                    as unsafe extern "C" fn(
                        *const floatformat,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            split_half: 0 as *const floatformat,
        };
        init
    }
};
#[no_mangle]
pub static mut floatformat_arm_ext_littlebyte_bigword: floatformat = unsafe {
    {
        let mut init = floatformat {
            byteorder: floatformat_littlebyte_bigword,
            totalsize: 96 as libc::c_int as libc::c_uint,
            sign_start: 0 as libc::c_int as libc::c_uint,
            exp_start: 17 as libc::c_int as libc::c_uint,
            exp_len: 15 as libc::c_int as libc::c_uint,
            exp_bias: 0x3fff as libc::c_int,
            exp_nan: 0x7fff as libc::c_int as libc::c_uint,
            man_start: 32 as libc::c_int as libc::c_uint,
            man_len: 64 as libc::c_int as libc::c_uint,
            intbit: floatformat_intbit_yes,
            name: b"floatformat_arm_ext_littlebyte_bigword\0" as *const u8
                as *const libc::c_char,
            is_valid: Some(
                floatformat_always_valid
                    as unsafe extern "C" fn(
                        *const floatformat,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            split_half: 0 as *const floatformat,
        };
        init
    }
};
#[no_mangle]
pub static mut floatformat_ia64_spill_big: floatformat = unsafe {
    {
        let mut init = floatformat {
            byteorder: floatformat_big,
            totalsize: 128 as libc::c_int as libc::c_uint,
            sign_start: 0 as libc::c_int as libc::c_uint,
            exp_start: 1 as libc::c_int as libc::c_uint,
            exp_len: 17 as libc::c_int as libc::c_uint,
            exp_bias: 65535 as libc::c_int,
            exp_nan: 0x1ffff as libc::c_int as libc::c_uint,
            man_start: 18 as libc::c_int as libc::c_uint,
            man_len: 64 as libc::c_int as libc::c_uint,
            intbit: floatformat_intbit_yes,
            name: b"floatformat_ia64_spill_big\0" as *const u8 as *const libc::c_char,
            is_valid: Some(
                floatformat_always_valid
                    as unsafe extern "C" fn(
                        *const floatformat,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            split_half: 0 as *const floatformat,
        };
        init
    }
};
#[no_mangle]
pub static mut floatformat_ia64_spill_little: floatformat = unsafe {
    {
        let mut init = floatformat {
            byteorder: floatformat_little,
            totalsize: 128 as libc::c_int as libc::c_uint,
            sign_start: 0 as libc::c_int as libc::c_uint,
            exp_start: 1 as libc::c_int as libc::c_uint,
            exp_len: 17 as libc::c_int as libc::c_uint,
            exp_bias: 65535 as libc::c_int,
            exp_nan: 0x1ffff as libc::c_int as libc::c_uint,
            man_start: 18 as libc::c_int as libc::c_uint,
            man_len: 64 as libc::c_int as libc::c_uint,
            intbit: floatformat_intbit_yes,
            name: b"floatformat_ia64_spill_little\0" as *const u8 as *const libc::c_char,
            is_valid: Some(
                floatformat_always_valid
                    as unsafe extern "C" fn(
                        *const floatformat,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            split_half: 0 as *const floatformat,
        };
        init
    }
};
#[no_mangle]
pub static mut floatformat_ia64_quad_big: floatformat = unsafe {
    {
        let mut init = floatformat {
            byteorder: floatformat_big,
            totalsize: 128 as libc::c_int as libc::c_uint,
            sign_start: 0 as libc::c_int as libc::c_uint,
            exp_start: 1 as libc::c_int as libc::c_uint,
            exp_len: 15 as libc::c_int as libc::c_uint,
            exp_bias: 16383 as libc::c_int,
            exp_nan: 0x7fff as libc::c_int as libc::c_uint,
            man_start: 16 as libc::c_int as libc::c_uint,
            man_len: 112 as libc::c_int as libc::c_uint,
            intbit: floatformat_intbit_no,
            name: b"floatformat_ia64_quad_big\0" as *const u8 as *const libc::c_char,
            is_valid: Some(
                floatformat_always_valid
                    as unsafe extern "C" fn(
                        *const floatformat,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            split_half: 0 as *const floatformat,
        };
        init
    }
};
#[no_mangle]
pub static mut floatformat_ia64_quad_little: floatformat = unsafe {
    {
        let mut init = floatformat {
            byteorder: floatformat_little,
            totalsize: 128 as libc::c_int as libc::c_uint,
            sign_start: 0 as libc::c_int as libc::c_uint,
            exp_start: 1 as libc::c_int as libc::c_uint,
            exp_len: 15 as libc::c_int as libc::c_uint,
            exp_bias: 16383 as libc::c_int,
            exp_nan: 0x7fff as libc::c_int as libc::c_uint,
            man_start: 16 as libc::c_int as libc::c_uint,
            man_len: 112 as libc::c_int as libc::c_uint,
            intbit: floatformat_intbit_no,
            name: b"floatformat_ia64_quad_little\0" as *const u8 as *const libc::c_char,
            is_valid: Some(
                floatformat_always_valid
                    as unsafe extern "C" fn(
                        *const floatformat,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            split_half: 0 as *const floatformat,
        };
        init
    }
};
unsafe extern "C" fn floatformat_ibm_long_double_is_valid(
    mut fmt: *const floatformat,
    mut from: *const libc::c_void,
) -> libc::c_int {
    let mut ufrom: *const libc::c_uchar = from as *const libc::c_uchar;
    let mut hfmt: *const floatformat = (*fmt).split_half;
    let mut top_exp: libc::c_long = 0;
    let mut bot_exp: libc::c_long = 0;
    let mut top_nan: libc::c_int = 0 as libc::c_int;
    top_exp = get_field(
        ufrom,
        (*hfmt).byteorder,
        (*hfmt).totalsize,
        (*hfmt).exp_start,
        (*hfmt).exp_len,
    ) as libc::c_long;
    bot_exp = get_field(
        ufrom.offset(8 as libc::c_int as isize),
        (*hfmt).byteorder,
        (*hfmt).totalsize,
        (*hfmt).exp_start,
        (*hfmt).exp_len,
    ) as libc::c_long;
    if top_exp as libc::c_ulong == (*hfmt).exp_nan as libc::c_ulong {
        top_nan = mant_bits_set(hfmt, ufrom);
    }
    if top_nan != 0 {
        return 1 as libc::c_int;
    }
    if top_exp as libc::c_ulong == (*hfmt).exp_nan as libc::c_ulong
        || top_exp == 0 as libc::c_int as libc::c_long
    {
        if bot_exp != 0 as libc::c_int as libc::c_long {
            return 0 as libc::c_int;
        }
        return (mant_bits_set(hfmt, ufrom.offset(8 as libc::c_int as isize)) == 0)
            as libc::c_int;
    }
    if bot_exp < top_exp - 53 as libc::c_int as libc::c_long {
        return 1 as libc::c_int;
    }
    if bot_exp > top_exp - 53 as libc::c_int as libc::c_long
        && bot_exp != 0 as libc::c_int as libc::c_long
    {
        return 0 as libc::c_int;
    }
    if bot_exp == 0 as libc::c_int as libc::c_long {
        let mut first_bit: libc::c_int = -(1 as libc::c_int);
        let mut second_bit: libc::c_int = -(1 as libc::c_int);
        let mut cur_bit: libc::c_int = 0;
        cur_bit = 0 as libc::c_int;
        while (cur_bit as libc::c_uint) < (*hfmt).man_len {
            if get_field(
                ufrom.offset(8 as libc::c_int as isize),
                (*hfmt).byteorder,
                (*hfmt).totalsize,
                ((*hfmt).man_start).wrapping_add(cur_bit as libc::c_uint),
                1 as libc::c_int as libc::c_uint,
            ) != 0
            {
                if first_bit == -(1 as libc::c_int) {
                    first_bit = cur_bit;
                } else {
                    second_bit = cur_bit;
                    break;
                }
            }
            cur_bit += 1;
            cur_bit;
        }
        if first_bit == -(1 as libc::c_int) {
            return 1 as libc::c_int;
        }
        if (-first_bit as libc::c_long) < top_exp - 53 as libc::c_int as libc::c_long {
            return 1 as libc::c_int;
        }
        if -first_bit as libc::c_long > top_exp - 53 as libc::c_int as libc::c_long {
            return 0 as libc::c_int;
        }
        if second_bit != -(1 as libc::c_int) {
            return 0 as libc::c_int;
        }
        return (get_field(
            ufrom,
            (*hfmt).byteorder,
            (*hfmt).totalsize,
            ((*hfmt).man_start)
                .wrapping_add((*hfmt).man_len)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
            1 as libc::c_int as libc::c_uint,
        ) == 0) as libc::c_int;
    } else {
        if get_field(
            ufrom,
            (*hfmt).byteorder,
            (*hfmt).totalsize,
            ((*hfmt).man_start)
                .wrapping_add((*hfmt).man_len)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
            1 as libc::c_int as libc::c_uint,
        ) != 0
        {
            return 0 as libc::c_int;
        }
        return (mant_bits_set(hfmt, ufrom.offset(8 as libc::c_int as isize)) == 0)
            as libc::c_int;
    };
}
#[no_mangle]
pub static mut floatformat_ibm_long_double_big: floatformat = unsafe {
    {
        let mut init = floatformat {
            byteorder: floatformat_big,
            totalsize: 128 as libc::c_int as libc::c_uint,
            sign_start: 0 as libc::c_int as libc::c_uint,
            exp_start: 1 as libc::c_int as libc::c_uint,
            exp_len: 11 as libc::c_int as libc::c_uint,
            exp_bias: 1023 as libc::c_int,
            exp_nan: 2047 as libc::c_int as libc::c_uint,
            man_start: 12 as libc::c_int as libc::c_uint,
            man_len: 52 as libc::c_int as libc::c_uint,
            intbit: floatformat_intbit_no,
            name: b"floatformat_ibm_long_double_big\0" as *const u8
                as *const libc::c_char,
            is_valid: Some(
                floatformat_ibm_long_double_is_valid
                    as unsafe extern "C" fn(
                        *const floatformat,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            split_half: &floatformat_ieee_double_big as *const floatformat,
        };
        init
    }
};
#[no_mangle]
pub static mut floatformat_ibm_long_double_little: floatformat = unsafe {
    {
        let mut init = floatformat {
            byteorder: floatformat_little,
            totalsize: 128 as libc::c_int as libc::c_uint,
            sign_start: 0 as libc::c_int as libc::c_uint,
            exp_start: 1 as libc::c_int as libc::c_uint,
            exp_len: 11 as libc::c_int as libc::c_uint,
            exp_bias: 1023 as libc::c_int,
            exp_nan: 2047 as libc::c_int as libc::c_uint,
            man_start: 12 as libc::c_int as libc::c_uint,
            man_len: 52 as libc::c_int as libc::c_uint,
            intbit: floatformat_intbit_no,
            name: b"floatformat_ibm_long_double_little\0" as *const u8
                as *const libc::c_char,
            is_valid: Some(
                floatformat_ibm_long_double_is_valid
                    as unsafe extern "C" fn(
                        *const floatformat,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            split_half: &floatformat_ieee_double_little as *const floatformat,
        };
        init
    }
};
#[no_mangle]
pub static mut floatformat_bfloat16_big: floatformat = unsafe {
    {
        let mut init = floatformat {
            byteorder: floatformat_big,
            totalsize: 16 as libc::c_int as libc::c_uint,
            sign_start: 0 as libc::c_int as libc::c_uint,
            exp_start: 1 as libc::c_int as libc::c_uint,
            exp_len: 8 as libc::c_int as libc::c_uint,
            exp_bias: 127 as libc::c_int,
            exp_nan: 255 as libc::c_int as libc::c_uint,
            man_start: 9 as libc::c_int as libc::c_uint,
            man_len: 7 as libc::c_int as libc::c_uint,
            intbit: floatformat_intbit_no,
            name: b"floatformat_bfloat16_big\0" as *const u8 as *const libc::c_char,
            is_valid: Some(
                floatformat_always_valid
                    as unsafe extern "C" fn(
                        *const floatformat,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            split_half: 0 as *const floatformat,
        };
        init
    }
};
#[no_mangle]
pub static mut floatformat_bfloat16_little: floatformat = unsafe {
    {
        let mut init = floatformat {
            byteorder: floatformat_little,
            totalsize: 16 as libc::c_int as libc::c_uint,
            sign_start: 0 as libc::c_int as libc::c_uint,
            exp_start: 1 as libc::c_int as libc::c_uint,
            exp_len: 8 as libc::c_int as libc::c_uint,
            exp_bias: 127 as libc::c_int,
            exp_nan: 255 as libc::c_int as libc::c_uint,
            man_start: 9 as libc::c_int as libc::c_uint,
            man_len: 7 as libc::c_int as libc::c_uint,
            intbit: floatformat_intbit_no,
            name: b"floatformat_bfloat16_little\0" as *const u8 as *const libc::c_char,
            is_valid: Some(
                floatformat_always_valid
                    as unsafe extern "C" fn(
                        *const floatformat,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            split_half: 0 as *const floatformat,
        };
        init
    }
};
unsafe extern "C" fn mant_bits_set(
    mut fmt: *const floatformat,
    mut ufrom: *const libc::c_uchar,
) -> libc::c_int {
    let mut mant_bits: libc::c_uint = 0;
    let mut mant_off: libc::c_uint = 0;
    let mut mant_bits_left: libc::c_int = 0;
    mant_off = (*fmt).man_start;
    mant_bits_left = (*fmt).man_len as libc::c_int;
    while mant_bits_left > 0 as libc::c_int {
        mant_bits = (if mant_bits_left < 32 as libc::c_int {
            mant_bits_left
        } else {
            32 as libc::c_int
        }) as libc::c_uint;
        if get_field(ufrom, (*fmt).byteorder, (*fmt).totalsize, mant_off, mant_bits)
            != 0 as libc::c_int as libc::c_ulong
        {
            return 1 as libc::c_int;
        }
        mant_off = mant_off.wrapping_add(mant_bits);
        mant_bits_left = (mant_bits_left as libc::c_uint).wrapping_sub(mant_bits)
            as libc::c_int as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn get_field(
    mut data: *const libc::c_uchar,
    mut order: floatformat_byteorders,
    mut total_len: libc::c_uint,
    mut start: libc::c_uint,
    mut len: libc::c_uint,
) -> libc::c_ulong {
    let mut result: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut cur_byte: libc::c_uint = 0;
    let mut lo_bit: libc::c_int = 0;
    let mut hi_bit: libc::c_int = 0;
    let mut cur_bitshift: libc::c_int = 0 as libc::c_int;
    let mut nextbyte: libc::c_int = if order as libc::c_uint
        == floatformat_little as libc::c_int as libc::c_uint
    {
        1 as libc::c_int
    } else {
        -(1 as libc::c_int)
    };
    start = total_len.wrapping_sub(start.wrapping_add(len));
    if order as libc::c_uint == floatformat_little as libc::c_int as libc::c_uint {
        cur_byte = start.wrapping_div(8 as libc::c_int as libc::c_uint);
    } else {
        cur_byte = total_len
            .wrapping_sub(start)
            .wrapping_sub(1 as libc::c_int as libc::c_uint)
            .wrapping_div(8 as libc::c_int as libc::c_uint);
    }
    lo_bit = start.wrapping_rem(8 as libc::c_int as libc::c_uint) as libc::c_int;
    hi_bit = (if (lo_bit as libc::c_uint).wrapping_add(len)
        < 8 as libc::c_int as libc::c_uint
    {
        (lo_bit as libc::c_uint).wrapping_add(len)
    } else {
        8 as libc::c_int as libc::c_uint
    }) as libc::c_int;
    loop {
        let mut shifted: libc::c_uint = (*data.offset(cur_byte as isize) as libc::c_int
            >> lo_bit) as libc::c_uint;
        let mut bits: libc::c_uint = (hi_bit - lo_bit) as libc::c_uint;
        let mut mask: libc::c_uint = (((1 as libc::c_int) << bits) - 1 as libc::c_int)
            as libc::c_uint;
        result |= ((shifted & mask) << cur_bitshift) as libc::c_ulong;
        len = len.wrapping_sub(bits);
        cur_bitshift = (cur_bitshift as libc::c_uint).wrapping_add(bits) as libc::c_int
            as libc::c_int;
        cur_byte = cur_byte.wrapping_add(nextbyte as libc::c_uint);
        lo_bit = 0 as libc::c_int;
        hi_bit = (if len < 8 as libc::c_int as libc::c_uint {
            len
        } else {
            8 as libc::c_int as libc::c_uint
        }) as libc::c_int;
        if !(len != 0 as libc::c_int as libc::c_uint) {
            break;
        }
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn floatformat_to_double(
    mut fmt: *const floatformat,
    mut from: *const libc::c_void,
    mut to: *mut libc::c_double,
) {
    let mut ufrom: *const libc::c_uchar = from as *const libc::c_uchar;
    let mut dto: libc::c_double = 0.;
    let mut exponent: libc::c_long = 0;
    let mut mant: libc::c_ulong = 0;
    let mut mant_bits: libc::c_uint = 0;
    let mut mant_off: libc::c_uint = 0;
    let mut mant_bits_left: libc::c_int = 0;
    exponent = get_field(
        ufrom,
        (*fmt).byteorder,
        (*fmt).totalsize,
        (*fmt).exp_start,
        (*fmt).exp_len,
    ) as libc::c_long;
    if exponent as libc::c_ulong == (*fmt).exp_nan as libc::c_ulong {
        let mut nan: libc::c_int = mant_bits_set(fmt, ufrom);
        if nan != 0 {
            dto = ::core::f32::NAN as libc::c_double;
        } else {
            dto = ::core::f32::INFINITY as libc::c_double;
        }
        if get_field(
            ufrom,
            (*fmt).byteorder,
            (*fmt).totalsize,
            (*fmt).sign_start,
            1 as libc::c_int as libc::c_uint,
        ) != 0
        {
            dto = -dto;
        }
        *to = dto;
        return;
    }
    mant_bits_left = (*fmt).man_len as libc::c_int;
    mant_off = (*fmt).man_start;
    dto = 0.0f64;
    if exponent == 0 as libc::c_int as libc::c_long {
        exponent = (1 as libc::c_int - (*fmt).exp_bias) as libc::c_long;
    } else {
        exponent -= (*fmt).exp_bias as libc::c_long;
        if (*fmt).intbit as libc::c_uint
            == floatformat_intbit_no as libc::c_int as libc::c_uint
        {
            dto = ldexp(1.0f64, exponent as libc::c_int);
        } else {
            exponent += 1;
            exponent;
        }
    }
    while mant_bits_left > 0 as libc::c_int {
        mant_bits = (if mant_bits_left < 32 as libc::c_int {
            mant_bits_left
        } else {
            32 as libc::c_int
        }) as libc::c_uint;
        mant = get_field(ufrom, (*fmt).byteorder, (*fmt).totalsize, mant_off, mant_bits);
        dto
            += ldexp(
                mant as libc::c_double,
                (exponent - mant_bits as libc::c_long) as libc::c_int,
            );
        exponent -= mant_bits as libc::c_long;
        mant_off = mant_off.wrapping_add(mant_bits);
        mant_bits_left = (mant_bits_left as libc::c_uint).wrapping_sub(mant_bits)
            as libc::c_int as libc::c_int;
    }
    if get_field(
        ufrom,
        (*fmt).byteorder,
        (*fmt).totalsize,
        (*fmt).sign_start,
        1 as libc::c_int as libc::c_uint,
    ) != 0
    {
        dto = -dto;
    }
    *to = dto;
}
unsafe extern "C" fn put_field(
    mut data: *mut libc::c_uchar,
    mut order: floatformat_byteorders,
    mut total_len: libc::c_uint,
    mut start: libc::c_uint,
    mut len: libc::c_uint,
    mut stuff_to_put: libc::c_ulong,
) {
    let mut cur_byte: libc::c_uint = 0;
    let mut lo_bit: libc::c_int = 0;
    let mut hi_bit: libc::c_int = 0;
    let mut nextbyte: libc::c_int = if order as libc::c_uint
        == floatformat_little as libc::c_int as libc::c_uint
    {
        1 as libc::c_int
    } else {
        -(1 as libc::c_int)
    };
    start = total_len.wrapping_sub(start.wrapping_add(len));
    if order as libc::c_uint == floatformat_little as libc::c_int as libc::c_uint {
        cur_byte = start.wrapping_div(8 as libc::c_int as libc::c_uint);
    } else {
        cur_byte = total_len
            .wrapping_sub(start)
            .wrapping_sub(1 as libc::c_int as libc::c_uint)
            .wrapping_div(8 as libc::c_int as libc::c_uint);
    }
    lo_bit = start.wrapping_rem(8 as libc::c_int as libc::c_uint) as libc::c_int;
    hi_bit = (if (lo_bit as libc::c_uint).wrapping_add(len)
        < 8 as libc::c_int as libc::c_uint
    {
        (lo_bit as libc::c_uint).wrapping_add(len)
    } else {
        8 as libc::c_int as libc::c_uint
    }) as libc::c_int;
    loop {
        let mut byte_ptr: *mut libc::c_uchar = data.offset(cur_byte as isize);
        let mut bits: libc::c_uint = (hi_bit - lo_bit) as libc::c_uint;
        let mut mask: libc::c_uint = ((((1 as libc::c_int) << bits) - 1 as libc::c_int)
            << lo_bit) as libc::c_uint;
        *byte_ptr = ((*byte_ptr as libc::c_uint & !mask) as libc::c_ulong
            | stuff_to_put << lo_bit & mask as libc::c_ulong) as libc::c_uchar;
        stuff_to_put >>= bits;
        len = len.wrapping_sub(bits);
        cur_byte = cur_byte.wrapping_add(nextbyte as libc::c_uint);
        lo_bit = 0 as libc::c_int;
        hi_bit = (if len < 8 as libc::c_int as libc::c_uint {
            len
        } else {
            8 as libc::c_int as libc::c_uint
        }) as libc::c_int;
        if !(len != 0 as libc::c_int as libc::c_uint) {
            break;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn floatformat_from_double(
    mut fmt: *const floatformat,
    mut from: *const libc::c_double,
    mut to: *mut libc::c_void,
) {
    let mut dfrom: libc::c_double = 0.;
    let mut exponent: libc::c_int = 0;
    let mut mant: libc::c_double = 0.;
    let mut mant_bits: libc::c_uint = 0;
    let mut mant_off: libc::c_uint = 0;
    let mut mant_bits_left: libc::c_int = 0;
    let mut uto: *mut libc::c_uchar = to as *mut libc::c_uchar;
    dfrom = *from;
    memset(
        uto as *mut libc::c_void,
        0 as libc::c_int,
        ((*fmt).totalsize).wrapping_div(8 as libc::c_int as libc::c_uint)
            as libc::c_ulong,
    );
    if dfrom < 0 as libc::c_int as libc::c_double {
        put_field(
            uto,
            (*fmt).byteorder,
            (*fmt).totalsize,
            (*fmt).sign_start,
            1 as libc::c_int as libc::c_uint,
            1 as libc::c_int as libc::c_ulong,
        );
        dfrom = -dfrom;
    }
    if dfrom == 0 as libc::c_int as libc::c_double {
        return;
    }
    if dfrom != dfrom {
        put_field(
            uto,
            (*fmt).byteorder,
            (*fmt).totalsize,
            (*fmt).exp_start,
            (*fmt).exp_len,
            (*fmt).exp_nan as libc::c_ulong,
        );
        put_field(
            uto,
            (*fmt).byteorder,
            (*fmt).totalsize,
            (*fmt).man_start,
            32 as libc::c_int as libc::c_uint,
            1 as libc::c_int as libc::c_ulong,
        );
        return;
    }
    if dfrom + dfrom == dfrom {
        put_field(
            uto,
            (*fmt).byteorder,
            (*fmt).totalsize,
            (*fmt).exp_start,
            (*fmt).exp_len,
            (*fmt).exp_nan as libc::c_ulong,
        );
        return;
    }
    mant = frexp(dfrom, &mut exponent);
    if exponent + (*fmt).exp_bias - 1 as libc::c_int > 0 as libc::c_int {
        put_field(
            uto,
            (*fmt).byteorder,
            (*fmt).totalsize,
            (*fmt).exp_start,
            (*fmt).exp_len,
            (exponent + (*fmt).exp_bias - 1 as libc::c_int) as libc::c_ulong,
        );
    } else {
        put_field(
            uto,
            (*fmt).byteorder,
            (*fmt).totalsize,
            (*fmt).exp_start,
            (*fmt).exp_len,
            0 as libc::c_int as libc::c_ulong,
        );
        mant = ldexp(mant, exponent + (*fmt).exp_bias - 1 as libc::c_int);
    }
    mant_bits_left = (*fmt).man_len as libc::c_int;
    mant_off = (*fmt).man_start;
    while mant_bits_left > 0 as libc::c_int {
        let mut mant_long: libc::c_ulong = 0;
        mant_bits = (if mant_bits_left < 32 as libc::c_int {
            mant_bits_left
        } else {
            32 as libc::c_int
        }) as libc::c_uint;
        mant *= 4294967296.0f64;
        mant_long = mant as libc::c_ulong;
        mant -= mant_long as libc::c_double;
        if mant_bits_left as libc::c_uint == (*fmt).man_len
            && (*fmt).intbit as libc::c_uint
                == floatformat_intbit_no as libc::c_int as libc::c_uint
            && exponent + (*fmt).exp_bias - 1 as libc::c_int > 0 as libc::c_int
        {
            mant_long &= 0x7fffffff as libc::c_int as libc::c_ulong;
            mant_bits = mant_bits.wrapping_sub(1 as libc::c_int as libc::c_uint);
        } else if mant_bits < 32 as libc::c_int as libc::c_uint {
            mant_long >>= (32 as libc::c_int as libc::c_uint).wrapping_sub(mant_bits);
        }
        put_field(
            uto,
            (*fmt).byteorder,
            (*fmt).totalsize,
            mant_off,
            mant_bits,
            mant_long,
        );
        mant_off = mant_off.wrapping_add(mant_bits);
        mant_bits_left = (mant_bits_left as libc::c_uint).wrapping_sub(mant_bits)
            as libc::c_int as libc::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn floatformat_is_valid(
    mut fmt: *const floatformat,
    mut from: *const libc::c_void,
) -> libc::c_int {
    return ((*fmt).is_valid).expect("non-null function pointer")(fmt, from);
}
