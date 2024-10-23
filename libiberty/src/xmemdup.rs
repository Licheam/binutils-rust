use ::libc;
extern "C" {
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
#[no_mangle]
pub unsafe extern "C" fn xmemdup(
    mut input: *const libc::c_void,
    mut copy_size: size_t,
    mut alloc_size: size_t,
) -> *mut libc::c_void {
    let mut output: *mut libc::c_void = xmalloc(alloc_size);
    if alloc_size > copy_size {
        memset(
            (output as *mut libc::c_char).offset(copy_size as isize)
                as *mut libc::c_void,
            0 as libc::c_int,
            alloc_size.wrapping_sub(copy_size),
        );
    }
    return memcpy(output, input, copy_size);
}
