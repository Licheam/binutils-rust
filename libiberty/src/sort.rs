use ::libc;
extern "C" {
    fn abort() -> !;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub type digit_t = libc::c_uchar;
#[no_mangle]
pub unsafe extern "C" fn sort_pointers(
    mut n: size_t,
    mut pointers: *mut *mut libc::c_void,
    mut work: *mut *mut libc::c_void,
) {
    let mut count: [libc::c_uint; 256] = [0; 256];
    let mut big_endian_p: libc::c_int = 0;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    if (::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<digit_t>() as libc::c_ulong)
        .wrapping_rem(2 as libc::c_int as libc::c_ulong)
        != 0 as libc::c_int as libc::c_ulong
    {
        abort();
    }
    i = 0 as libc::c_int as size_t;
    j = 0 as libc::c_int as size_t;
    while i < ::core::mem::size_of::<size_t>() as libc::c_ulong {
        j = (j as libc::c_ulong)
            .wrapping_mul(
                (127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int) as libc::c_ulong,
            ) as size_t as size_t;
        j = (j as libc::c_ulong).wrapping_add(i) as size_t as size_t;
        i = i.wrapping_add(1);
        i;
    }
    big_endian_p = (*(&mut j as *mut size_t as *mut libc::c_char)
        .offset(0 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int)
        as libc::c_int;
    i = 0 as libc::c_int as size_t;
    while i
        < (::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<digit_t>() as libc::c_ulong)
    {
        let mut digit: *mut digit_t = 0 as *mut digit_t;
        let mut bias: *mut digit_t = 0 as *mut digit_t;
        let mut top: *mut digit_t = 0 as *mut digit_t;
        let mut countp: *mut libc::c_uint = 0 as *mut libc::c_uint;
        let mut pointerp: *mut *mut libc::c_void = 0 as *mut *mut libc::c_void;
        if big_endian_p != 0 {
            j = (::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<digit_t>() as libc::c_ulong)
                .wrapping_sub(i);
        } else {
            j = i;
        }
        memset(
            count.as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            ((127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_uint>() as libc::c_ulong),
        );
        bias = (pointers as *mut digit_t).offset(j as isize);
        top = (pointers.offset(n as isize) as *mut digit_t).offset(j as isize);
        digit = bias;
        while digit < top {
            count[*digit as usize] = (count[*digit as usize]).wrapping_add(1);
            count[*digit as usize];
            digit = digit
                .offset(
                    (::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                        .wrapping_div(::core::mem::size_of::<digit_t>() as libc::c_ulong)
                        as isize,
                );
        }
        countp = count.as_mut_ptr().offset(1 as libc::c_int as isize);
        while countp
            < count
                .as_mut_ptr()
                .offset(
                    (127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int
                        + 1 as libc::c_int) as isize,
                )
        {
            *countp = (*countp)
                .wrapping_add(*countp.offset(-(1 as libc::c_int) as isize));
            countp = countp.offset(1);
            countp;
        }
        pointerp = pointers.offset(n as isize).offset(-(1 as libc::c_int as isize));
        while pointerp >= pointers {
            count[*(pointerp as *mut digit_t).offset(j as isize)
                as usize] = (count[*(pointerp as *mut digit_t).offset(j as isize)
                as usize])
                .wrapping_sub(1);
            let ref mut fresh0 = *work
                .offset(
                    count[*(pointerp as *mut digit_t).offset(j as isize) as usize]
                        as isize,
                );
            *fresh0 = *pointerp;
            pointerp = pointerp.offset(-1);
            pointerp;
        }
        pointerp = pointers;
        pointers = work;
        work = pointerp;
        i = i.wrapping_add(1);
        i;
    }
}
