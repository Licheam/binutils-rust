extern "C" {
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
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
pub unsafe extern "C" fn flonum_copy(
    mut in_0: *mut FLONUM_TYPE,
    mut out: *mut FLONUM_TYPE,
) {
    let mut in_length: libc::c_uint = 0;
    let mut out_length: libc::c_uint = 0;
    (*out).sign = (*in_0).sign;
    in_length = ((*in_0).leader).offset_from((*in_0).low) as libc::c_long
        as libc::c_uint;
    if (*in_0).leader < (*in_0).low {
        (*out).leader = ((*out).low).offset(-(1 as libc::c_int as isize));
    } else {
        out_length = ((*out).high).offset_from((*out).low) as libc::c_long
            as libc::c_uint;
        if in_length <= out_length {
            if in_length < out_length {
                memset(
                    ((*out).low)
                        .offset(in_length as isize)
                        .offset(1 as libc::c_int as isize) as *mut libc::c_char
                        as *mut libc::c_void,
                    '\0' as i32,
                    out_length.wrapping_sub(in_length) as libc::c_ulong,
                );
            }
            memcpy(
                (*out).low as *mut libc::c_void,
                (*in_0).low as *mut libc::c_void,
                (in_length.wrapping_add(1 as libc::c_int as libc::c_uint)
                    as libc::c_ulong)
                    .wrapping_mul(
                        ::core::mem::size_of::<LITTLENUM_TYPE>() as libc::c_ulong,
                    ),
            );
            (*out).exponent = (*in_0).exponent;
            (*out)
                .leader = ((*out).low)
                .offset(
                    ((*in_0).leader).offset_from((*in_0).low) as libc::c_long as isize,
                );
        } else {
            let mut shorten: libc::c_int = 0;
            shorten = in_length.wrapping_sub(out_length) as libc::c_int;
            memcpy(
                (*out).low as *mut libc::c_void,
                ((*in_0).low).offset(shorten as isize) as *mut libc::c_void,
                (out_length.wrapping_add(1 as libc::c_int as libc::c_uint)
                    as libc::c_ulong)
                    .wrapping_mul(
                        ::core::mem::size_of::<LITTLENUM_TYPE>() as libc::c_ulong,
                    ),
            );
            (*out).leader = (*out).high;
            (*out).exponent = (*in_0).exponent + shorten as libc::c_long;
        }
    };
}
