use ::libc;
extern "C" {
    pub type pex_obj;
    fn xmalloc(_: size_t) -> *mut libc::c_void;
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
    fn free(_: *mut libc::c_void);
}
pub type size_t = libc::c_ulong;
static mut pex: *mut pex_obj = 0 as *const pex_obj as *mut pex_obj;
static mut idx: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn pexecute(
    mut program: *const libc::c_char,
    mut argv: *const *mut libc::c_char,
    mut pname: *const libc::c_char,
    mut temp_base: *const libc::c_char,
    mut errmsg_fmt: *mut *mut libc::c_char,
    mut errmsg_arg: *mut *mut libc::c_char,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut errmsg: *const libc::c_char = 0 as *const libc::c_char;
    let mut err: libc::c_int = 0;
    if flags & 1 as libc::c_int != 0 as libc::c_int {
        if !pex.is_null() {
            *errmsg_fmt = b"pexecute already in progress\0" as *const u8
                as *const libc::c_char as *mut libc::c_char;
            *errmsg_arg = 0 as *mut libc::c_char;
            return -(1 as libc::c_int);
        }
        pex = pex_init(0x2 as libc::c_int, pname, temp_base);
        idx = 0 as libc::c_int;
    } else if pex.is_null() {
        *errmsg_fmt = b"pexecute not in progress\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
        *errmsg_arg = 0 as *mut libc::c_char;
        return -(1 as libc::c_int);
    }
    errmsg = pex_run(
        pex,
        (if flags & 2 as libc::c_int != 0 as libc::c_int {
            0x1 as libc::c_int
        } else {
            0 as libc::c_int
        })
            | (if flags & 4 as libc::c_int != 0 as libc::c_int {
                0x2 as libc::c_int
            } else {
                0 as libc::c_int
            }),
        program,
        argv,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        &mut err,
    );
    if !errmsg.is_null() {
        *errmsg_fmt = errmsg as *mut libc::c_char;
        *errmsg_arg = 0 as *mut libc::c_char;
        return -(1 as libc::c_int);
    }
    idx += 1;
    return idx;
}
#[no_mangle]
pub unsafe extern "C" fn pwait(
    mut pid: libc::c_int,
    mut status: *mut libc::c_int,
    mut flags: libc::c_int,
) -> libc::c_int {
    pid -= 1;
    pid;
    if pex.is_null() || pid < 0 as libc::c_int || pid >= idx {
        return -(1 as libc::c_int);
    }
    if pid == 0 as libc::c_int && idx == 1 as libc::c_int {
        if pex_get_status(pex, 1 as libc::c_int, status) == 0 {
            return -(1 as libc::c_int);
        }
    } else {
        let mut vector: *mut libc::c_int = 0 as *mut libc::c_int;
        vector = xmalloc(
            (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                .wrapping_mul(idx as libc::c_ulong),
        ) as *mut libc::c_int;
        if pex_get_status(pex, idx, vector) == 0 {
            free(vector as *mut libc::c_void);
            return -(1 as libc::c_int);
        }
        *status = *vector.offset(pid as isize);
        free(vector as *mut libc::c_void);
    }
    if pid + 1 as libc::c_int == idx {
        pex_free(pex);
        pex = 0 as *mut pex_obj;
        idx = 0 as libc::c_int;
    }
    return pid + 1 as libc::c_int;
}
