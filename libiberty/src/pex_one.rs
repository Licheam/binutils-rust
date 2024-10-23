use ::libc;
extern "C" {
    pub type pex_obj;
    fn pex_init(
        flags: libc::c_int,
        pname: *const libc::c_char,
        tempbase: *const libc::c_char,
    ) -> *mut pex_obj;
    fn pex_run(
        obj: *mut pex_obj,
        flags: libc::c_int,
        executable: *const libc::c_char,
        argv: *const *mut libc::c_char,
        outname: *const libc::c_char,
        errname: *const libc::c_char,
        err: *mut libc::c_int,
    ) -> *const libc::c_char;
    fn pex_get_status(
        _: *mut pex_obj,
        count: libc::c_int,
        vector: *mut libc::c_int,
    ) -> libc::c_int;
    fn pex_free(_: *mut pex_obj);
}
#[no_mangle]
pub unsafe extern "C" fn pex_one(
    mut flags: libc::c_int,
    mut executable: *const libc::c_char,
    mut argv: *const *mut libc::c_char,
    mut pname: *const libc::c_char,
    mut outname: *const libc::c_char,
    mut errname: *const libc::c_char,
    mut status: *mut libc::c_int,
    mut err: *mut libc::c_int,
) -> *const libc::c_char {
    let mut obj: *mut pex_obj = 0 as *mut pex_obj;
    let mut errmsg: *const libc::c_char = 0 as *const libc::c_char;
    obj = pex_init(0 as libc::c_int, pname, 0 as *const libc::c_char);
    errmsg = pex_run(obj, flags, executable, argv, outname, errname, err);
    if errmsg.is_null() {
        if pex_get_status(obj, 1 as libc::c_int, status) == 0 {
            *err = 0 as libc::c_int;
            errmsg = b"pex_get_status failed\0" as *const u8 as *const libc::c_char;
        }
    }
    pex_free(obj);
    return errmsg;
}
