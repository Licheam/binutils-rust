extern "C" {
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strncasecmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn free(_: *mut libc::c_void);
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    static flonum_positive_powers_of_ten: [FLONUM_TYPE; 0];
    static flonum_negative_powers_of_ten: [FLONUM_TYPE; 0];
    static table_size_of_flonum_powers_of_ten: libc::c_int;
    fn flonum_multip(
        a: *const FLONUM_TYPE,
        b: *const FLONUM_TYPE,
        product: *mut FLONUM_TYPE,
    );
    fn flonum_copy(in_0: *mut FLONUM_TYPE, out: *mut FLONUM_TYPE);
    fn as_fatal(format: *const libc::c_char, _: ...) -> !;
    fn as_abort(_: *const libc::c_char, _: libc::c_int, _: *const libc::c_char) -> !;
    static _sch_istable: [libc::c_ushort; 256];
}
pub type size_t = libc::c_ulong;
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
pub const _sch_isdigit: C2RustUnnamed = 4;
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
pub const _sch_iscntrl: C2RustUnnamed = 2;
pub const _sch_isblank: C2RustUnnamed = 1;
#[no_mangle]
pub unsafe extern "C" fn atof_generic(
    mut address_of_string_pointer: *mut *mut libc::c_char,
    mut string_of_decimal_marks: *const libc::c_char,
    mut string_of_decimal_exponent_marks: *const libc::c_char,
    mut address_of_generic_floating_point_number: *mut FLONUM_TYPE,
) -> libc::c_int {
    let mut return_value: libc::c_int = 0;
    let mut first_digit: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut number_of_digits_before_decimal: libc::c_uint = 0;
    let mut number_of_digits_after_decimal: libc::c_uint = 0;
    let mut decimal_exponent: libc::c_long = 0;
    let mut number_of_digits_available: libc::c_uint = 0;
    let mut digits_sign_char: libc::c_char = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut c: libc::c_char = 0;
    let mut seen_significant_digit: libc::c_int = 0;
    if *string_of_decimal_marks.offset(0 as libc::c_int as isize) as libc::c_int
        == '.' as i32
        && *string_of_decimal_marks.offset(1 as libc::c_int as isize) as libc::c_int
            == 0 as libc::c_int
    {} else {
        as_abort(
            b"atof-generic.c\0" as *const u8 as *const libc::c_char,
            97 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 69],
                &[libc::c_char; 69],
            >(b"int atof_generic(char **, const char *, const char *, FLONUM_TYPE *)\0"))
                .as_ptr(),
        );
    };
    first_digit = *address_of_string_pointer;
    c = *first_digit;
    if c as libc::c_int == '-' as i32 || c as libc::c_int == '+' as i32 {
        digits_sign_char = c;
        first_digit = first_digit.offset(1);
        first_digit;
    } else {
        digits_sign_char = '+' as i32 as libc::c_char;
    }
    match *first_digit.offset(0 as libc::c_int as isize) as libc::c_int {
        110 | 78 => {
            if strncasecmp(
                b"nan\0" as *const u8 as *const libc::c_char,
                first_digit,
                3 as libc::c_int as libc::c_ulong,
            ) == 0
            {
                (*address_of_generic_floating_point_number)
                    .sign = 0 as libc::c_int as libc::c_char;
                (*address_of_generic_floating_point_number)
                    .exponent = 0 as libc::c_int as libc::c_long;
                (*address_of_generic_floating_point_number)
                    .leader = (*address_of_generic_floating_point_number).low;
                *address_of_string_pointer = first_digit
                    .offset(3 as libc::c_int as isize);
                return 0 as libc::c_int;
            }
        }
        105 | 73 => {
            if strncasecmp(
                b"inf\0" as *const u8 as *const libc::c_char,
                first_digit,
                3 as libc::c_int as libc::c_ulong,
            ) == 0
            {
                (*address_of_generic_floating_point_number)
                    .sign = (if digits_sign_char as libc::c_int == '+' as i32 {
                    'P' as i32
                } else {
                    'N' as i32
                }) as libc::c_char;
                (*address_of_generic_floating_point_number)
                    .exponent = 0 as libc::c_int as libc::c_long;
                (*address_of_generic_floating_point_number)
                    .leader = (*address_of_generic_floating_point_number).low;
                first_digit = first_digit.offset(3 as libc::c_int as isize);
                if strncasecmp(
                    b"inity\0" as *const u8 as *const libc::c_char,
                    first_digit,
                    5 as libc::c_int as libc::c_ulong,
                ) == 0
                {
                    first_digit = first_digit.offset(5 as libc::c_int as isize);
                }
                *address_of_string_pointer = first_digit;
                return 0 as libc::c_int;
            }
        }
        _ => {}
    }
    number_of_digits_before_decimal = 0 as libc::c_int as libc::c_uint;
    number_of_digits_after_decimal = 0 as libc::c_int as libc::c_uint;
    decimal_exponent = 0 as libc::c_int as libc::c_long;
    seen_significant_digit = 0 as libc::c_int;
    p = first_digit;
    loop {
        c = *p;
        if !(c as libc::c_int != '\0' as i32
            && (c == 0 || !(c as libc::c_int == '.' as i32))
            && (c == 0
                || (strchr(string_of_decimal_exponent_marks, c as libc::c_int))
                    .is_null()))
        {
            break;
        }
        if !(_sch_istable[(c as libc::c_int & 0xff as libc::c_int) as usize]
            as libc::c_int & _sch_isdigit as libc::c_int as libc::c_ushort as libc::c_int
            != 0)
        {
            break;
        }
        if seen_significant_digit != 0 || c as libc::c_int > '0' as i32 {
            number_of_digits_before_decimal = number_of_digits_before_decimal
                .wrapping_add(1);
            number_of_digits_before_decimal;
            seen_significant_digit = 1 as libc::c_int;
        } else {
            first_digit = first_digit.offset(1);
            first_digit;
        }
        p = p.offset(1);
        p;
    }
    seen_significant_digit = 0 as libc::c_int;
    let mut subtract_decimal_exponent: libc::c_long = 0 as libc::c_int as libc::c_long;
    if c as libc::c_int != 0 && c as libc::c_int == '.' as i32 {
        let mut zeros: libc::c_uint = 0 as libc::c_int as libc::c_uint;
        if number_of_digits_before_decimal == 0 as libc::c_int as libc::c_uint {
            first_digit = first_digit.offset(1);
            first_digit;
        }
        p = p.offset(1);
        p;
        loop {
            c = *p;
            if !(c as libc::c_int != 0
                && _sch_istable[(c as libc::c_int & 0xff as libc::c_int) as usize]
                    as libc::c_int
                    & _sch_isdigit as libc::c_int as libc::c_ushort as libc::c_int != 0)
            {
                break;
            }
            if c as libc::c_int == '0' as i32 {
                if number_of_digits_before_decimal == 0 as libc::c_int as libc::c_uint
                    && seen_significant_digit == 0
                {
                    first_digit = first_digit.offset(1);
                    first_digit;
                    subtract_decimal_exponent -= 1;
                    subtract_decimal_exponent;
                } else {
                    zeros = zeros.wrapping_add(1);
                    zeros;
                }
            } else {
                seen_significant_digit = 1 as libc::c_int;
                number_of_digits_after_decimal = number_of_digits_after_decimal
                    .wrapping_add(
                        (1 as libc::c_int as libc::c_uint).wrapping_add(zeros),
                    );
                zeros = 0 as libc::c_int as libc::c_uint;
            }
            p = p.offset(1);
            p;
        }
    }
    if c as libc::c_int != 0
        && !(strchr(string_of_decimal_exponent_marks, c as libc::c_int)).is_null()
    {
        let mut digits_exponent_sign_char: libc::c_char = 0;
        p = p.offset(1);
        c = *p;
        if c as libc::c_int != 0
            && !(strchr(b"+-\0" as *const u8 as *const libc::c_char, c as libc::c_int))
                .is_null()
        {
            digits_exponent_sign_char = c;
            p = p.offset(1);
            c = *p;
        } else {
            digits_exponent_sign_char = '+' as i32 as libc::c_char;
        }
        while c != 0 {
            if !(_sch_istable[(c as libc::c_int & 0xff as libc::c_int) as usize]
                as libc::c_int
                & _sch_isdigit as libc::c_int as libc::c_ushort as libc::c_int != 0)
            {
                break;
            }
            decimal_exponent = decimal_exponent * 10 as libc::c_int as libc::c_long
                + c as libc::c_long - '0' as i32 as libc::c_long;
            p = p.offset(1);
            c = *p;
        }
        if digits_exponent_sign_char as libc::c_int == '-' as i32 {
            decimal_exponent = -decimal_exponent;
        }
    }
    decimal_exponent += subtract_decimal_exponent;
    *address_of_string_pointer = p;
    number_of_digits_available = number_of_digits_before_decimal
        .wrapping_add(number_of_digits_after_decimal);
    return_value = 0 as libc::c_int;
    if number_of_digits_available == 0 as libc::c_int as libc::c_uint {
        (*address_of_generic_floating_point_number)
            .exponent = 0 as libc::c_int as libc::c_long;
        (*address_of_generic_floating_point_number)
            .leader = ((*address_of_generic_floating_point_number).low)
            .offset(-(1 as libc::c_int) as isize);
        (*address_of_generic_floating_point_number).sign = digits_sign_char;
    } else {
        let mut count: libc::c_int = 0;
        let mut temporary_binary_low: *mut LITTLENUM_TYPE = 0 as *mut LITTLENUM_TYPE;
        let mut power_binary_low: *mut LITTLENUM_TYPE = 0 as *mut LITTLENUM_TYPE;
        let mut digits_binary_low: *mut LITTLENUM_TYPE = 0 as *mut LITTLENUM_TYPE;
        let mut precision: libc::c_uint = 0;
        let mut maximum_useful_digits: libc::c_uint = 0;
        let mut number_of_digits_to_use: libc::c_uint = 0;
        let mut more_than_enough_bits_for_digits: libc::c_uint = 0;
        let mut more_than_enough_littlenums_for_digits: libc::c_uint = 0;
        let mut size_of_digits_in_littlenums: libc::c_uint = 0;
        let mut size_of_digits_in_chars: libc::c_uint = 0;
        let mut power_of_10_flonum: FLONUM_TYPE = FLONUM_TYPE {
            low: 0 as *mut LITTLENUM_TYPE,
            high: 0 as *mut LITTLENUM_TYPE,
            leader: 0 as *mut LITTLENUM_TYPE,
            exponent: 0,
            sign: 0,
        };
        let mut digits_flonum: FLONUM_TYPE = FLONUM_TYPE {
            low: 0 as *mut LITTLENUM_TYPE,
            high: 0 as *mut LITTLENUM_TYPE,
            leader: 0 as *mut LITTLENUM_TYPE,
            exponent: 0,
            sign: 0,
        };
        precision = (((*address_of_generic_floating_point_number).high)
            .offset_from((*address_of_generic_floating_point_number).low) as libc::c_long
            + 1 as libc::c_int as libc::c_long) as libc::c_uint;
        maximum_useful_digits = precision
            .wrapping_mul(16 as libc::c_int as libc::c_uint)
            .wrapping_mul(1000000 as libc::c_int as libc::c_uint)
            .wrapping_div(3321928 as libc::c_int as libc::c_uint)
            .wrapping_add(1 as libc::c_int as libc::c_uint);
        if number_of_digits_available > maximum_useful_digits {
            number_of_digits_to_use = maximum_useful_digits;
        } else {
            number_of_digits_to_use = number_of_digits_available;
        }
        decimal_exponent
            += number_of_digits_before_decimal as libc::c_long
                - number_of_digits_to_use as libc::c_long;
        more_than_enough_bits_for_digits = number_of_digits_to_use
            .wrapping_mul(3321928 as libc::c_int as libc::c_uint)
            .wrapping_div(1000000 as libc::c_int as libc::c_uint)
            .wrapping_add(1 as libc::c_int as libc::c_uint);
        more_than_enough_littlenums_for_digits = more_than_enough_bits_for_digits
            .wrapping_div(16 as libc::c_int as libc::c_uint)
            .wrapping_add(2 as libc::c_int as libc::c_uint);
        size_of_digits_in_littlenums = more_than_enough_littlenums_for_digits;
        size_of_digits_in_chars = (size_of_digits_in_littlenums as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<LITTLENUM_TYPE>() as libc::c_ulong)
            as libc::c_uint;
        digits_binary_low = xmalloc(size_of_digits_in_chars as size_t)
            as *mut LITTLENUM_TYPE;
        memset(
            digits_binary_low as *mut libc::c_char as *mut libc::c_void,
            '\0' as i32,
            size_of_digits_in_chars as libc::c_ulong,
        );
        p = first_digit;
        count = number_of_digits_to_use as libc::c_int;
        while count != 0 {
            c = *p;
            if _sch_istable[(c as libc::c_int & 0xff as libc::c_int) as usize]
                as libc::c_int
                & _sch_isdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
            {
                let mut carry: libc::c_long = 0;
                let mut littlenum_pointer: *mut LITTLENUM_TYPE = 0
                    as *mut LITTLENUM_TYPE;
                let mut littlenum_limit: *mut LITTLENUM_TYPE = 0 as *mut LITTLENUM_TYPE;
                littlenum_limit = digits_binary_low
                    .offset(more_than_enough_littlenums_for_digits as isize)
                    .offset(-(1 as libc::c_int as isize));
                carry = (c as libc::c_int - '0' as i32) as libc::c_long;
                littlenum_pointer = digits_binary_low;
                while littlenum_pointer <= littlenum_limit {
                    let mut work: libc::c_long = 0;
                    work = carry
                        + 10 as libc::c_int as libc::c_long
                            * *littlenum_pointer as libc::c_long;
                    *littlenum_pointer = (work & 0xffff as libc::c_int as libc::c_long)
                        as LITTLENUM_TYPE;
                    carry = work >> 16 as libc::c_int;
                    littlenum_pointer = littlenum_pointer.offset(1);
                    littlenum_pointer;
                }
                if carry != 0 as libc::c_int as libc::c_long {
                    as_fatal(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"failed sanity check\0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                }
            } else {
                count += 1;
                count;
            }
            p = p.offset(1);
            p;
            count -= 1;
            count;
        }
        while *digits_binary_low
            .offset(
                size_of_digits_in_littlenums
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
            ) as libc::c_int == 0 as libc::c_int
            && size_of_digits_in_littlenums >= 2 as libc::c_int as libc::c_uint
        {
            size_of_digits_in_littlenums = size_of_digits_in_littlenums.wrapping_sub(1);
            size_of_digits_in_littlenums;
        }
        digits_flonum.low = digits_binary_low;
        digits_flonum
            .high = digits_binary_low
            .offset(size_of_digits_in_littlenums as isize)
            .offset(-(1 as libc::c_int as isize));
        digits_flonum.leader = digits_flonum.high;
        digits_flonum.exponent = 0 as libc::c_int as libc::c_long;
        digits_flonum.sign = '+' as i32 as libc::c_char;
        let mut decimal_exponent_is_negative: libc::c_int = 0;
        let mut temporary_flonum: FLONUM_TYPE = FLONUM_TYPE {
            low: 0 as *mut LITTLENUM_TYPE,
            high: 0 as *mut LITTLENUM_TYPE,
            leader: 0 as *mut LITTLENUM_TYPE,
            exponent: 0,
            sign: 0,
        };
        let mut size_of_power_in_littlenums: libc::c_uint = 0;
        let mut size_of_power_in_chars: libc::c_uint = 0;
        size_of_power_in_littlenums = precision;
        decimal_exponent_is_negative = (decimal_exponent
            < 0 as libc::c_int as libc::c_long) as libc::c_int;
        if decimal_exponent_is_negative != 0 {
            decimal_exponent = -decimal_exponent;
        }
        size_of_power_in_chars = (size_of_power_in_littlenums as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<LITTLENUM_TYPE>() as libc::c_ulong)
            .wrapping_add(2 as libc::c_int as libc::c_ulong) as libc::c_uint;
        power_binary_low = xmalloc(size_of_power_in_chars as size_t)
            as *mut LITTLENUM_TYPE;
        temporary_binary_low = xmalloc(size_of_power_in_chars as size_t)
            as *mut LITTLENUM_TYPE;
        memset(
            power_binary_low as *mut libc::c_char as *mut libc::c_void,
            '\0' as i32,
            size_of_power_in_chars as libc::c_ulong,
        );
        *power_binary_low = 1 as libc::c_int as LITTLENUM_TYPE;
        power_of_10_flonum.exponent = 0 as libc::c_int as libc::c_long;
        power_of_10_flonum.low = power_binary_low;
        power_of_10_flonum.leader = power_binary_low;
        power_of_10_flonum
            .high = power_binary_low
            .offset(size_of_power_in_littlenums as isize)
            .offset(-(1 as libc::c_int as isize));
        power_of_10_flonum.sign = '+' as i32 as libc::c_char;
        temporary_flonum.low = temporary_binary_low;
        temporary_flonum
            .high = temporary_binary_low
            .offset(size_of_power_in_littlenums as isize)
            .offset(-(1 as libc::c_int as isize));
        let mut place_number_limit: libc::c_int = 0;
        let mut place_number: libc::c_int = 0;
        let mut multiplicand: *const FLONUM_TYPE = 0 as *const FLONUM_TYPE;
        place_number_limit = table_size_of_flonum_powers_of_ten;
        multiplicand = if decimal_exponent_is_negative != 0 {
            flonum_negative_powers_of_ten.as_ptr()
        } else {
            flonum_positive_powers_of_ten.as_ptr()
        };
        place_number = 1 as libc::c_int;
        while decimal_exponent != 0 {
            if decimal_exponent & 1 as libc::c_int as libc::c_long != 0 {
                if place_number > place_number_limit {
                    return_value = 2 as libc::c_int;
                    decimal_exponent = 0 as libc::c_int as libc::c_long;
                } else {
                    flonum_multip(
                        multiplicand.offset(place_number as isize),
                        &mut power_of_10_flonum,
                        &mut temporary_flonum,
                    );
                    flonum_copy(&mut temporary_flonum, &mut power_of_10_flonum);
                }
            }
            decimal_exponent >>= 1 as libc::c_int;
            place_number += 1;
            place_number;
        }
        flonum_multip(
            &mut power_of_10_flonum,
            &mut digits_flonum,
            address_of_generic_floating_point_number,
        );
        (*address_of_generic_floating_point_number).sign = digits_sign_char;
        free(temporary_binary_low as *mut libc::c_void);
        free(power_binary_low as *mut libc::c_void);
        free(digits_binary_low as *mut libc::c_void);
    }
    return return_value;
}
