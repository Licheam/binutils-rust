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
#[no_mangle]
pub unsafe extern "C" fn flonum_multip(
    mut a: *const FLONUM_TYPE,
    mut b: *const FLONUM_TYPE,
    mut product: *mut FLONUM_TYPE,
) {
    let mut size_of_a: libc::c_int = 0;
    let mut size_of_b: libc::c_int = 0;
    let mut size_of_product: libc::c_int = 0;
    let mut size_of_sum: libc::c_int = 0;
    let mut extra_product_positions: libc::c_int = 0;
    let mut work: libc::c_ulong = 0;
    let mut carry: libc::c_ulong = 0;
    let mut exponent: libc::c_long = 0;
    let mut q: *mut LITTLENUM_TYPE = 0 as *mut LITTLENUM_TYPE;
    let mut significant: libc::c_long = 0;
    let mut P: libc::c_int = 0;
    let mut N: libc::c_int = 0;
    let mut A: libc::c_int = 0;
    let mut B: libc::c_int = 0;
    if (*a).sign as libc::c_int != '-' as i32 && (*a).sign as libc::c_int != '+' as i32
        || (*b).sign as libc::c_int != '-' as i32
            && (*b).sign as libc::c_int != '+' as i32
    {
        (*product).sign = 0 as libc::c_int as libc::c_char;
        return;
    }
    (*product)
        .sign = (if (*a).sign as libc::c_int == (*b).sign as libc::c_int {
        '+' as i32
    } else {
        '-' as i32
    }) as libc::c_char;
    size_of_a = ((*a).leader).offset_from((*a).low) as libc::c_long as libc::c_int;
    size_of_b = ((*b).leader).offset_from((*b).low) as libc::c_long as libc::c_int;
    exponent = (*a).exponent + (*b).exponent;
    size_of_product = ((*product).high).offset_from((*product).low) as libc::c_long
        as libc::c_int;
    size_of_sum = size_of_a + size_of_b;
    extra_product_positions = size_of_product - size_of_sum;
    if extra_product_positions < 0 as libc::c_int {
        P = extra_product_positions;
        exponent -= extra_product_positions as libc::c_long;
    } else {
        P = 0 as libc::c_int;
    }
    carry = 0 as libc::c_int as libc::c_ulong;
    significant = 0 as libc::c_int as libc::c_long;
    N = 0 as libc::c_int;
    while N <= size_of_sum {
        work = carry;
        carry = 0 as libc::c_int as libc::c_ulong;
        A = 0 as libc::c_int;
        while A <= N {
            B = N - A;
            if A <= size_of_a && B <= size_of_b && B >= 0 as libc::c_int {
                work = work
                    .wrapping_add(
                        (*((*a).low).offset(A as isize) as libc::c_ulong)
                            .wrapping_mul(
                                *((*b).low).offset(B as isize) as libc::c_ulong,
                            ),
                    );
                carry = carry.wrapping_add(work >> 16 as libc::c_int);
                work &= 0xffff as libc::c_int as libc::c_ulong;
            }
            A += 1;
            A;
        }
        significant = (significant as libc::c_ulong | work) as libc::c_long;
        if significant != 0 || P < 0 as libc::c_int {
            if P >= 0 as libc::c_int {
                *((*product).low).offset(P as isize) = work as LITTLENUM_TYPE;
            }
            P += 1;
            P;
        } else {
            extra_product_positions += 1;
            extra_product_positions;
            exponent += 1;
            exponent;
        }
        N += 1;
        N;
    }
    if carry != 0 {
        if extra_product_positions > 0 as libc::c_int {
            *((*product).low).offset(P as isize) = carry as LITTLENUM_TYPE;
        } else {
            exponent += 1;
            exponent;
            P -= 1;
            P;
            q = ((*product).low).offset(P as isize);
            while q >= (*product).low {
                work = *q as libc::c_ulong;
                *q = carry as LITTLENUM_TYPE;
                carry = work;
                q = q.offset(-1);
                q;
            }
        }
    } else {
        P -= 1;
        P;
    }
    (*product).leader = ((*product).low).offset(P as isize);
    (*product).exponent = exponent;
}
