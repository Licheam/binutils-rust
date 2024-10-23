use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn remove(__filename: *const libc::c_char) -> libc::c_int;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn concat(_: *const libc::c_char, _: ...) -> *mut libc::c_char;
    fn make_temp_file(_: *const libc::c_char) -> *mut libc::c_char;
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    fn xrealloc(_: *mut libc::c_void, _: size_t) -> *mut libc::c_void;
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    fn free(_: *mut libc::c_void);
    fn mkstemps(__template: *mut libc::c_char, __suffixlen: libc::c_int) -> libc::c_int;
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
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn close(__fd: libc::c_int) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pex_obj {
    pub flags: libc::c_int,
    pub pname: *const libc::c_char,
    pub tempbase: *const libc::c_char,
    pub next_input: libc::c_int,
    pub next_input_name: *mut libc::c_char,
    pub next_input_name_allocated: libc::c_int,
    pub stderr_pipe: libc::c_int,
    pub count: libc::c_int,
    pub children: *mut pid_t,
    pub status: *mut libc::c_int,
    pub time: *mut pex_time,
    pub number_waited: libc::c_int,
    pub input_file: *mut FILE,
    pub read_output: *mut FILE,
    pub read_err: *mut FILE,
    pub remove_count: libc::c_int,
    pub remove: *mut *mut libc::c_char,
    pub funcs: *const pex_funcs,
    pub sysdep: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pex_funcs {
    pub open_read: Option::<
        unsafe extern "C" fn(
            *mut pex_obj,
            *const libc::c_char,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub open_write: Option::<
        unsafe extern "C" fn(
            *mut pex_obj,
            *const libc::c_char,
            libc::c_int,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub exec_child: Option::<
        unsafe extern "C" fn(
            *mut pex_obj,
            libc::c_int,
            *const libc::c_char,
            *const *mut libc::c_char,
            *const *mut libc::c_char,
            libc::c_int,
            libc::c_int,
            libc::c_int,
            libc::c_int,
            *mut *const libc::c_char,
            *mut libc::c_int,
        ) -> pid_t,
    >,
    pub close: Option::<unsafe extern "C" fn(*mut pex_obj, libc::c_int) -> libc::c_int>,
    pub wait: Option::<
        unsafe extern "C" fn(
            *mut pex_obj,
            pid_t,
            *mut libc::c_int,
            *mut pex_time,
            libc::c_int,
            *mut *const libc::c_char,
            *mut libc::c_int,
        ) -> pid_t,
    >,
    pub pipe: Option::<
        unsafe extern "C" fn(*mut pex_obj, *mut libc::c_int, libc::c_int) -> libc::c_int,
    >,
    pub fdopenr: Option::<
        unsafe extern "C" fn(*mut pex_obj, libc::c_int, libc::c_int) -> *mut FILE,
    >,
    pub fdopenw: Option::<
        unsafe extern "C" fn(*mut pex_obj, libc::c_int, libc::c_int) -> *mut FILE,
    >,
    pub cleanup: Option::<unsafe extern "C" fn(*mut pex_obj) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pex_time {
    pub user_seconds: libc::c_ulong,
    pub user_microseconds: libc::c_ulong,
    pub system_seconds: libc::c_ulong,
    pub system_microseconds: libc::c_ulong,
}
pub type pid_t = __pid_t;
#[no_mangle]
pub unsafe extern "C" fn pex_init_common(
    mut flags: libc::c_int,
    mut pname: *const libc::c_char,
    mut tempbase: *const libc::c_char,
    mut funcs: *const pex_funcs,
) -> *mut pex_obj {
    let mut obj: *mut pex_obj = 0 as *mut pex_obj;
    obj = xmalloc(::core::mem::size_of::<pex_obj>() as libc::c_ulong) as *mut pex_obj;
    (*obj).flags = flags;
    (*obj).pname = pname;
    (*obj).tempbase = tempbase;
    (*obj).next_input = 0 as libc::c_int;
    (*obj).next_input_name = 0 as *mut libc::c_char;
    (*obj).next_input_name_allocated = 0 as libc::c_int;
    (*obj).stderr_pipe = -(1 as libc::c_int);
    (*obj).count = 0 as libc::c_int;
    (*obj).children = 0 as *mut pid_t;
    (*obj).status = 0 as *mut libc::c_int;
    (*obj).time = 0 as *mut pex_time;
    (*obj).number_waited = 0 as libc::c_int;
    (*obj).input_file = 0 as *mut FILE;
    (*obj).read_output = 0 as *mut FILE;
    (*obj).read_err = 0 as *mut FILE;
    (*obj).remove_count = 0 as libc::c_int;
    (*obj).remove = 0 as *mut *mut libc::c_char;
    (*obj).funcs = funcs;
    (*obj).sysdep = 0 as *mut libc::c_void;
    return obj;
}
unsafe extern "C" fn pex_add_remove(
    mut obj: *mut pex_obj,
    mut name: *const libc::c_char,
    mut allocated: libc::c_int,
) {
    let mut add: *mut libc::c_char = 0 as *mut libc::c_char;
    (*obj).remove_count += 1;
    (*obj).remove_count;
    (*obj)
        .remove = xrealloc(
        (*obj).remove as *mut libc::c_void,
        (::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
            .wrapping_mul((*obj).remove_count as libc::c_ulong),
    ) as *mut *mut libc::c_char;
    if allocated != 0 {
        add = name as *mut libc::c_char;
    } else {
        add = xstrdup(name);
    }
    let ref mut fresh0 = *((*obj).remove)
        .offset(((*obj).remove_count - 1 as libc::c_int) as isize);
    *fresh0 = add;
}
unsafe extern "C" fn temp_file(
    mut obj: *mut pex_obj,
    mut flags: libc::c_int,
    mut name: *mut libc::c_char,
) -> *mut libc::c_char {
    if name.is_null() {
        if ((*obj).tempbase).is_null() {
            name = make_temp_file(0 as *const libc::c_char);
        } else {
            let mut len: libc::c_int = strlen((*obj).tempbase) as libc::c_int;
            let mut out: libc::c_int = 0;
            if len >= 6 as libc::c_int
                && strcmp(
                    ((*obj).tempbase)
                        .offset(len as isize)
                        .offset(-(6 as libc::c_int as isize)),
                    b"XXXXXX\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
            {
                name = xstrdup((*obj).tempbase);
            } else {
                name = concat(
                    (*obj).tempbase,
                    b"XXXXXX\0" as *const u8 as *const libc::c_char,
                    0 as *mut libc::c_void,
                );
            }
            out = mkstemps(name, 0 as libc::c_int);
            if out < 0 as libc::c_int {
                free(name as *mut libc::c_void);
                return 0 as *mut libc::c_char;
            }
            close(out);
        }
    } else if flags & 0x4 as libc::c_int != 0 as libc::c_int {
        if ((*obj).tempbase).is_null() {
            name = make_temp_file(name);
        } else {
            name = concat((*obj).tempbase, name, 0 as *mut libc::c_void);
        }
    }
    return name;
}
#[no_mangle]
pub unsafe extern "C" fn pex_run_in_environment(
    mut obj: *mut pex_obj,
    mut flags: libc::c_int,
    mut executable: *const libc::c_char,
    mut argv: *const *mut libc::c_char,
    mut env: *const *mut libc::c_char,
    mut orig_outname: *const libc::c_char,
    mut errname: *const libc::c_char,
    mut err: *mut libc::c_int,
) -> *const libc::c_char {
    let mut current_block: u64;
    let mut errmsg: *const libc::c_char = 0 as *const libc::c_char;
    let mut in_0: libc::c_int = 0;
    let mut out: libc::c_int = 0;
    let mut errdes: libc::c_int = 0;
    let mut outname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut outname_allocated: libc::c_int = 0;
    let mut p: [libc::c_int; 2] = [0; 2];
    let mut toclose: libc::c_int = 0;
    let mut pid: pid_t = 0;
    in_0 = -(1 as libc::c_int);
    out = -(1 as libc::c_int);
    errdes = -(1 as libc::c_int);
    outname = orig_outname as *mut libc::c_char;
    outname_allocated = 0 as libc::c_int;
    if !((*obj).input_file).is_null() {
        if fclose((*obj).input_file) == -(1 as libc::c_int) {
            errmsg = b"closing pipeline input file\0" as *const u8
                as *const libc::c_char;
            current_block = 15984047177975538432;
        } else {
            (*obj).input_file = 0 as *mut FILE;
            current_block = 1394248824506584008;
        }
    } else {
        current_block = 1394248824506584008;
    }
    match current_block {
        1394248824506584008 => {
            if !((*obj).next_input_name).is_null() {
                if pex_get_status_and_time(obj, 0 as libc::c_int, &mut errmsg, err) == 0
                {
                    current_block = 15984047177975538432;
                } else {
                    in_0 = ((*(*obj).funcs).open_read)
                        .expect(
                            "non-null function pointer",
                        )(
                        obj,
                        (*obj).next_input_name,
                        (flags & 0x10 as libc::c_int != 0 as libc::c_int) as libc::c_int,
                    );
                    if in_0 < 0 as libc::c_int {
                        *err = *__errno_location();
                        errmsg = b"open temporary file\0" as *const u8
                            as *const libc::c_char;
                        current_block = 15984047177975538432;
                    } else {
                        if (*obj).next_input_name_allocated != 0 {
                            free((*obj).next_input_name as *mut libc::c_void);
                            (*obj).next_input_name_allocated = 0 as libc::c_int;
                        }
                        (*obj).next_input_name = 0 as *mut libc::c_char;
                        current_block = 1109700713171191020;
                    }
                }
            } else {
                in_0 = (*obj).next_input;
                if in_0 < 0 as libc::c_int {
                    *err = 0 as libc::c_int;
                    errmsg = b"pipeline already complete\0" as *const u8
                        as *const libc::c_char;
                    current_block = 15984047177975538432;
                } else {
                    current_block = 1109700713171191020;
                }
            }
            match current_block {
                15984047177975538432 => {}
                _ => {
                    if flags & 0x1 as libc::c_int != 0 as libc::c_int {
                        if outname.is_null() {
                            out = 1 as libc::c_int;
                        } else if flags & 0x4 as libc::c_int != 0 as libc::c_int {
                            outname = concat(
                                (*obj).tempbase,
                                outname,
                                0 as *mut libc::c_void,
                            );
                            outname_allocated = 1 as libc::c_int;
                        }
                        (*obj).next_input = -(1 as libc::c_int);
                        current_block = 6717214610478484138;
                    } else if (*obj).flags & 0x2 as libc::c_int == 0 as libc::c_int {
                        outname = temp_file(obj, flags, outname);
                        if outname.is_null() {
                            *err = 0 as libc::c_int;
                            errmsg = b"could not create temporary file\0" as *const u8
                                as *const libc::c_char;
                            current_block = 15984047177975538432;
                        } else {
                            if outname != orig_outname as *mut libc::c_char {
                                outname_allocated = 1 as libc::c_int;
                            }
                            if (*obj).flags & 0x4 as libc::c_int == 0 as libc::c_int {
                                pex_add_remove(obj, outname, outname_allocated);
                                outname_allocated = 0 as libc::c_int;
                            }
                            (*obj).next_input_name = outname;
                            (*obj).next_input_name_allocated = outname_allocated;
                            outname_allocated = 0 as libc::c_int;
                            current_block = 6717214610478484138;
                        }
                    } else if ((*(*obj).funcs).pipe)
                        .expect(
                            "non-null function pointer",
                        )(
                        obj,
                        p.as_mut_ptr(),
                        (flags & 0x20 as libc::c_int != 0 as libc::c_int) as libc::c_int,
                    ) < 0 as libc::c_int
                    {
                        *err = *__errno_location();
                        errmsg = b"pipe\0" as *const u8 as *const libc::c_char;
                        current_block = 15984047177975538432;
                    } else {
                        out = p[1 as libc::c_int as usize];
                        (*obj).next_input = p[0 as libc::c_int as usize];
                        current_block = 6717214610478484138;
                    }
                    match current_block {
                        15984047177975538432 => {}
                        _ => {
                            if out < 0 as libc::c_int {
                                out = ((*(*obj).funcs).open_write)
                                    .expect(
                                        "non-null function pointer",
                                    )(
                                    obj,
                                    outname,
                                    (flags & 0x20 as libc::c_int != 0 as libc::c_int)
                                        as libc::c_int,
                                    (flags & 0x100 as libc::c_int != 0 as libc::c_int)
                                        as libc::c_int,
                                );
                                if out < 0 as libc::c_int {
                                    *err = *__errno_location();
                                    errmsg = b"open temporary output file\0" as *const u8
                                        as *const libc::c_char;
                                    current_block = 15984047177975538432;
                                } else {
                                    current_block = 1423531122933789233;
                                }
                            } else {
                                current_block = 1423531122933789233;
                            }
                            match current_block {
                                15984047177975538432 => {}
                                _ => {
                                    if outname_allocated != 0 {
                                        free(outname as *mut libc::c_void);
                                        outname_allocated = 0 as libc::c_int;
                                    }
                                    if !errname.is_null()
                                        && flags & 0x40 as libc::c_int != 0 as libc::c_int
                                    {
                                        *err = 0 as libc::c_int;
                                        errmsg = b"both ERRNAME and PEX_STDERR_TO_PIPE specified.\0"
                                            as *const u8 as *const libc::c_char;
                                    } else if (*obj).stderr_pipe != -(1 as libc::c_int) {
                                        *err = 0 as libc::c_int;
                                        errmsg = b"PEX_STDERR_TO_PIPE used in the middle of pipeline\0"
                                            as *const u8 as *const libc::c_char;
                                    } else {
                                        if errname.is_null() {
                                            if flags & 0x40 as libc::c_int != 0 {
                                                if ((*(*obj).funcs).pipe)
                                                    .expect(
                                                        "non-null function pointer",
                                                    )(
                                                    obj,
                                                    p.as_mut_ptr(),
                                                    (flags & 0x80 as libc::c_int != 0 as libc::c_int)
                                                        as libc::c_int,
                                                ) < 0 as libc::c_int
                                                {
                                                    *err = *__errno_location();
                                                    errmsg = b"pipe\0" as *const u8 as *const libc::c_char;
                                                    current_block = 15984047177975538432;
                                                } else {
                                                    errdes = p[1 as libc::c_int as usize];
                                                    (*obj).stderr_pipe = p[0 as libc::c_int as usize];
                                                    current_block = 13484060386966298149;
                                                }
                                            } else {
                                                errdes = 2 as libc::c_int;
                                                current_block = 13484060386966298149;
                                            }
                                        } else {
                                            errdes = ((*(*obj).funcs).open_write)
                                                .expect(
                                                    "non-null function pointer",
                                                )(
                                                obj,
                                                errname,
                                                (flags & 0x80 as libc::c_int != 0 as libc::c_int)
                                                    as libc::c_int,
                                                (flags & 0x200 as libc::c_int != 0 as libc::c_int)
                                                    as libc::c_int,
                                            );
                                            if errdes < 0 as libc::c_int {
                                                *err = *__errno_location();
                                                errmsg = b"open error file\0" as *const u8
                                                    as *const libc::c_char;
                                                current_block = 15984047177975538432;
                                            } else {
                                                current_block = 13484060386966298149;
                                            }
                                        }
                                        match current_block {
                                            15984047177975538432 => {}
                                            _ => {
                                                if (*obj).flags & 0x2 as libc::c_int == 0 as libc::c_int {
                                                    toclose = -(1 as libc::c_int);
                                                } else {
                                                    toclose = (*obj).next_input;
                                                }
                                                pid = ((*(*obj).funcs).exec_child)
                                                    .expect(
                                                        "non-null function pointer",
                                                    )(
                                                    obj,
                                                    flags,
                                                    executable,
                                                    argv,
                                                    env,
                                                    in_0,
                                                    out,
                                                    errdes,
                                                    toclose,
                                                    &mut errmsg,
                                                    err,
                                                );
                                                if !(pid < 0 as libc::c_int) {
                                                    (*obj).count += 1;
                                                    (*obj).count;
                                                    (*obj)
                                                        .children = xrealloc(
                                                        (*obj).children as *mut libc::c_void,
                                                        (::core::mem::size_of::<pid_t>() as libc::c_ulong)
                                                            .wrapping_mul((*obj).count as libc::c_ulong),
                                                    ) as *mut pid_t;
                                                    *((*obj).children)
                                                        .offset(((*obj).count - 1 as libc::c_int) as isize) = pid;
                                                    return 0 as *const libc::c_char;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    if in_0 >= 0 as libc::c_int && in_0 != 0 as libc::c_int {
        ((*(*obj).funcs).close).expect("non-null function pointer")(obj, in_0);
    }
    if out >= 0 as libc::c_int && out != 1 as libc::c_int {
        ((*(*obj).funcs).close).expect("non-null function pointer")(obj, out);
    }
    if errdes >= 0 as libc::c_int && errdes != 2 as libc::c_int {
        ((*(*obj).funcs).close).expect("non-null function pointer")(obj, errdes);
    }
    if outname_allocated != 0 {
        free(outname as *mut libc::c_void);
    }
    return errmsg;
}
#[no_mangle]
pub unsafe extern "C" fn pex_run(
    mut obj: *mut pex_obj,
    mut flags: libc::c_int,
    mut executable: *const libc::c_char,
    mut argv: *const *mut libc::c_char,
    mut orig_outname: *const libc::c_char,
    mut errname: *const libc::c_char,
    mut err: *mut libc::c_int,
) -> *const libc::c_char {
    return pex_run_in_environment(
        obj,
        flags,
        executable,
        argv,
        0 as *const *mut libc::c_char,
        orig_outname,
        errname,
        err,
    );
}
#[no_mangle]
pub unsafe extern "C" fn pex_input_file(
    mut obj: *mut pex_obj,
    mut flags: libc::c_int,
    mut in_name: *const libc::c_char,
) -> *mut FILE {
    let mut name: *mut libc::c_char = in_name as *mut libc::c_char;
    let mut f: *mut FILE = 0 as *mut FILE;
    if (*obj).count != 0 as libc::c_int
        || (*obj).next_input >= 0 as libc::c_int && (*obj).next_input != 0 as libc::c_int
        || !((*obj).next_input_name).is_null()
    {
        *__errno_location() = 22 as libc::c_int;
        return 0 as *mut FILE;
    }
    name = temp_file(obj, flags, name);
    if name.is_null() {
        return 0 as *mut FILE;
    }
    f = fopen(
        name,
        if flags & 0x20 as libc::c_int != 0 {
            b"wb\0" as *const u8 as *const libc::c_char
        } else {
            b"w\0" as *const u8 as *const libc::c_char
        },
    );
    if f.is_null() {
        free(name as *mut libc::c_void);
        return 0 as *mut FILE;
    }
    (*obj).input_file = f;
    (*obj).next_input_name = name;
    (*obj)
        .next_input_name_allocated = (name != in_name as *mut libc::c_char)
        as libc::c_int;
    return f;
}
#[no_mangle]
pub unsafe extern "C" fn pex_input_pipe(
    mut obj: *mut pex_obj,
    mut binary: libc::c_int,
) -> *mut FILE {
    let mut p: [libc::c_int; 2] = [0; 2];
    let mut f: *mut FILE = 0 as *mut FILE;
    if !((*obj).count > 0 as libc::c_int) {
        if !((*obj).flags & 0x2 as libc::c_int == 0) {
            if !((*obj).next_input >= 0 as libc::c_int
                && (*obj).next_input != 0 as libc::c_int
                || !((*obj).next_input_name).is_null())
            {
                if ((*(*obj).funcs).pipe)
                    .expect(
                        "non-null function pointer",
                    )(obj, p.as_mut_ptr(), (binary != 0 as libc::c_int) as libc::c_int)
                    < 0 as libc::c_int
                {
                    return 0 as *mut FILE;
                }
                f = ((*(*obj).funcs).fdopenw)
                    .expect(
                        "non-null function pointer",
                    )(
                    obj,
                    p[1 as libc::c_int as usize],
                    (binary != 0 as libc::c_int) as libc::c_int,
                );
                if f.is_null() {
                    let mut saved_errno: libc::c_int = *__errno_location();
                    ((*(*obj).funcs).close)
                        .expect(
                            "non-null function pointer",
                        )(obj, p[0 as libc::c_int as usize]);
                    ((*(*obj).funcs).close)
                        .expect(
                            "non-null function pointer",
                        )(obj, p[1 as libc::c_int as usize]);
                    *__errno_location() = saved_errno;
                    return 0 as *mut FILE;
                }
                (*obj).next_input = p[0 as libc::c_int as usize];
                return f;
            }
        }
    }
    *__errno_location() = 22 as libc::c_int;
    return 0 as *mut FILE;
}
#[no_mangle]
pub unsafe extern "C" fn pex_read_output(
    mut obj: *mut pex_obj,
    mut binary: libc::c_int,
) -> *mut FILE {
    if !((*obj).next_input_name).is_null() {
        let mut errmsg: *const libc::c_char = 0 as *const libc::c_char;
        let mut err: libc::c_int = 0;
        if pex_get_status_and_time(obj, 0 as libc::c_int, &mut errmsg, &mut err) == 0 {
            *__errno_location() = err;
            return 0 as *mut FILE;
        }
        (*obj)
            .read_output = fopen(
            (*obj).next_input_name,
            if binary != 0 {
                b"rb\0" as *const u8 as *const libc::c_char
            } else {
                b"r\0" as *const u8 as *const libc::c_char
            },
        );
        if (*obj).next_input_name_allocated != 0 {
            free((*obj).next_input_name as *mut libc::c_void);
            (*obj).next_input_name_allocated = 0 as libc::c_int;
        }
        (*obj).next_input_name = 0 as *mut libc::c_char;
    } else {
        let mut o: libc::c_int = 0;
        o = (*obj).next_input;
        if o < 0 as libc::c_int || o == 0 as libc::c_int {
            return 0 as *mut FILE;
        }
        (*obj)
            .read_output = ((*(*obj).funcs).fdopenr)
            .expect("non-null function pointer")(obj, o, binary);
        (*obj).next_input = -(1 as libc::c_int);
    }
    return (*obj).read_output;
}
#[no_mangle]
pub unsafe extern "C" fn pex_read_err(
    mut obj: *mut pex_obj,
    mut binary: libc::c_int,
) -> *mut FILE {
    let mut o: libc::c_int = 0;
    o = (*obj).stderr_pipe;
    if o < 0 as libc::c_int || o == 0 as libc::c_int {
        return 0 as *mut FILE;
    }
    (*obj)
        .read_err = ((*(*obj).funcs).fdopenr)
        .expect("non-null function pointer")(obj, o, binary);
    (*obj).stderr_pipe = -(1 as libc::c_int);
    return (*obj).read_err;
}
unsafe extern "C" fn pex_get_status_and_time(
    mut obj: *mut pex_obj,
    mut done: libc::c_int,
    mut errmsg: *mut *const libc::c_char,
    mut err: *mut libc::c_int,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    if (*obj).number_waited == (*obj).count {
        return 1 as libc::c_int;
    }
    (*obj)
        .status = xrealloc(
        (*obj).status as *mut libc::c_void,
        (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul((*obj).count as libc::c_ulong),
    ) as *mut libc::c_int;
    if (*obj).flags & 0x1 as libc::c_int != 0 as libc::c_int {
        (*obj)
            .time = xrealloc(
            (*obj).time as *mut libc::c_void,
            (::core::mem::size_of::<pex_time>() as libc::c_ulong)
                .wrapping_mul((*obj).count as libc::c_ulong),
        ) as *mut pex_time;
    }
    ret = 1 as libc::c_int;
    i = (*obj).number_waited;
    while i < (*obj).count {
        if ((*(*obj).funcs).wait)
            .expect(
                "non-null function pointer",
            )(
            obj,
            *((*obj).children).offset(i as isize),
            &mut *((*obj).status).offset(i as isize),
            (if ((*obj).time).is_null() {
                0 as *mut pex_time
            } else {
                &mut *((*obj).time).offset(i as isize)
            }),
            done,
            errmsg,
            err,
        ) < 0 as libc::c_int
        {
            ret = 0 as libc::c_int;
        }
        i += 1;
        i;
    }
    (*obj).number_waited = i;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn pex_get_status(
    mut obj: *mut pex_obj,
    mut count: libc::c_int,
    mut vector: *mut libc::c_int,
) -> libc::c_int {
    if ((*obj).status).is_null() {
        let mut errmsg: *const libc::c_char = 0 as *const libc::c_char;
        let mut err: libc::c_int = 0;
        if pex_get_status_and_time(obj, 0 as libc::c_int, &mut errmsg, &mut err) == 0 {
            return 0 as libc::c_int;
        }
    }
    if count > (*obj).count {
        memset(
            vector.offset((*obj).count as isize) as *mut libc::c_void,
            0 as libc::c_int,
            ((count - (*obj).count) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
        );
        count = (*obj).count;
    }
    memcpy(
        vector as *mut libc::c_void,
        (*obj).status as *const libc::c_void,
        (count as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn pex_get_times(
    mut obj: *mut pex_obj,
    mut count: libc::c_int,
    mut vector: *mut pex_time,
) -> libc::c_int {
    if ((*obj).status).is_null() {
        let mut errmsg: *const libc::c_char = 0 as *const libc::c_char;
        let mut err: libc::c_int = 0;
        if pex_get_status_and_time(obj, 0 as libc::c_int, &mut errmsg, &mut err) == 0 {
            return 0 as libc::c_int;
        }
    }
    if ((*obj).time).is_null() {
        return 0 as libc::c_int;
    }
    if count > (*obj).count {
        memset(
            vector.offset((*obj).count as isize) as *mut libc::c_void,
            0 as libc::c_int,
            ((count - (*obj).count) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<pex_time>() as libc::c_ulong),
        );
        count = (*obj).count;
    }
    memcpy(
        vector as *mut libc::c_void,
        (*obj).time as *const libc::c_void,
        (count as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<pex_time>() as libc::c_ulong),
    );
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn pex_free(mut obj: *mut pex_obj) {
    if (*obj).next_input >= 0 as libc::c_int && (*obj).next_input != 0 as libc::c_int {
        ((*(*obj).funcs).close)
            .expect("non-null function pointer")(obj, (*obj).next_input);
    }
    if (*obj).stderr_pipe >= 0 as libc::c_int && (*obj).stderr_pipe != 0 as libc::c_int {
        ((*(*obj).funcs).close)
            .expect("non-null function pointer")(obj, (*obj).stderr_pipe);
    }
    if !((*obj).read_output).is_null() {
        fclose((*obj).read_output);
    }
    if !((*obj).read_err).is_null() {
        fclose((*obj).read_err);
    }
    if ((*obj).status).is_null() {
        let mut errmsg: *const libc::c_char = 0 as *const libc::c_char;
        let mut err: libc::c_int = 0;
        (*obj).flags &= !(0x1 as libc::c_int);
        pex_get_status_and_time(obj, 1 as libc::c_int, &mut errmsg, &mut err);
    }
    if (*obj).next_input_name_allocated != 0 {
        free((*obj).next_input_name as *mut libc::c_void);
    }
    free((*obj).children as *mut libc::c_void);
    free((*obj).status as *mut libc::c_void);
    free((*obj).time as *mut libc::c_void);
    if (*obj).remove_count > 0 as libc::c_int {
        let mut i: libc::c_int = 0;
        i = 0 as libc::c_int;
        while i < (*obj).remove_count {
            remove(*((*obj).remove).offset(i as isize));
            free(*((*obj).remove).offset(i as isize) as *mut libc::c_void);
            i += 1;
            i;
        }
        free((*obj).remove as *mut libc::c_void);
    }
    if ((*(*obj).funcs).cleanup).is_some() {
        ((*(*obj).funcs).cleanup).expect("non-null function pointer")(obj);
    }
    free(obj as *mut libc::c_void);
}
