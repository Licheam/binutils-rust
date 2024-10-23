use ::libc;
extern "C" {
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    fn xcalloc(_: size_t, _: size_t) -> *mut libc::c_void;
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn lseek(__fd: libc::c_int, __offset: __off_t, __whence: libc::c_int) -> __off_t;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn __errno_location() -> *mut libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn free(_: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    static simple_object_coff_functions: simple_object_functions;
    static simple_object_elf_functions: simple_object_functions;
    static simple_object_mach_o_functions: simple_object_functions;
    static simple_object_xcoff_functions: simple_object_functions;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type off_t = __off_t;
pub type ssize_t = __ssize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct simple_object_read_struct {
    pub descriptor: libc::c_int,
    pub offset: off_t,
    pub functions: *const simple_object_functions,
    pub data: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct simple_object_functions {
    pub match_0: Option::<
        unsafe extern "C" fn(
            *mut libc::c_uchar,
            libc::c_int,
            off_t,
            *const libc::c_char,
            *mut *const libc::c_char,
            *mut libc::c_int,
        ) -> *mut libc::c_void,
    >,
    pub find_sections: Option::<
        unsafe extern "C" fn(
            *mut simple_object_read,
            Option::<
                unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const libc::c_char,
                    off_t,
                    off_t,
                ) -> libc::c_int,
            >,
            *mut libc::c_void,
            *mut libc::c_int,
        ) -> *const libc::c_char,
    >,
    pub fetch_attributes: Option::<
        unsafe extern "C" fn(
            *mut simple_object_read,
            *mut *const libc::c_char,
            *mut libc::c_int,
        ) -> *mut libc::c_void,
    >,
    pub release_read: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub attributes_merge: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut libc::c_void,
            *mut libc::c_int,
        ) -> *const libc::c_char,
    >,
    pub release_attributes: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub start_write: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut *const libc::c_char,
            *mut libc::c_int,
        ) -> *mut libc::c_void,
    >,
    pub write_to_file: Option::<
        unsafe extern "C" fn(
            *mut simple_object_write,
            libc::c_int,
            *mut libc::c_int,
        ) -> *const libc::c_char,
    >,
    pub release_write: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub copy_lto_debug_sections: Option::<
        unsafe extern "C" fn(
            *mut simple_object_read,
            *mut simple_object_write,
            Option::<unsafe extern "C" fn(*const libc::c_char) -> *mut libc::c_char>,
            *mut libc::c_int,
        ) -> *const libc::c_char,
    >,
}
pub type simple_object_write = simple_object_write_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct simple_object_write_struct {
    pub functions: *const simple_object_functions,
    pub segment_name: *mut libc::c_char,
    pub sections: *mut simple_object_write_section,
    pub last_section: *mut simple_object_write_section,
    pub data: *mut libc::c_void,
}
pub type simple_object_write_section = simple_object_write_section_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct simple_object_write_section_struct {
    pub next: *mut simple_object_write_section,
    pub name: *mut libc::c_char,
    pub align: libc::c_uint,
    pub buffers: *mut simple_object_write_section_buffer,
    pub last_buffer: *mut simple_object_write_section_buffer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct simple_object_write_section_buffer {
    pub next: *mut simple_object_write_section_buffer,
    pub size: size_t,
    pub buffer: *const libc::c_void,
    pub free_buffer: *mut libc::c_void,
}
pub type simple_object_read = simple_object_read_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct find_one_section_data {
    pub name: *const libc::c_char,
    pub offset: *mut off_t,
    pub length: *mut off_t,
    pub found: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct simple_object_attributes_struct {
    pub functions: *const simple_object_functions,
    pub data: *mut libc::c_void,
}
pub type simple_object_attributes = simple_object_attributes_struct;
static mut format_functions: [*const simple_object_functions; 4] = unsafe {
    [
        &simple_object_elf_functions as *const simple_object_functions,
        &simple_object_mach_o_functions as *const simple_object_functions,
        &simple_object_coff_functions as *const simple_object_functions,
        &simple_object_xcoff_functions as *const simple_object_functions,
    ]
};
#[no_mangle]
pub unsafe extern "C" fn simple_object_internal_read(
    mut descriptor: libc::c_int,
    mut offset: off_t,
    mut buffer: *mut libc::c_uchar,
    mut size: size_t,
    mut errmsg: *mut *const libc::c_char,
    mut err: *mut libc::c_int,
) -> libc::c_int {
    if lseek(descriptor, offset, 0 as libc::c_int) < 0 as libc::c_int as libc::c_long {
        *errmsg = b"lseek\0" as *const u8 as *const libc::c_char;
        *err = *__errno_location();
        return 0 as libc::c_int;
    }
    loop {
        let mut got: ssize_t = read(descriptor, buffer as *mut libc::c_void, size);
        if got == 0 as libc::c_int as libc::c_long {
            break;
        }
        if got > 0 as libc::c_int as libc::c_long {
            buffer = buffer.offset(got as isize);
            size = (size as libc::c_ulong).wrapping_sub(got as libc::c_ulong) as size_t
                as size_t;
        } else if *__errno_location() != 4 as libc::c_int {
            *errmsg = b"read\0" as *const u8 as *const libc::c_char;
            *err = *__errno_location();
            return 0 as libc::c_int;
        }
        if !(size > 0 as libc::c_int as libc::c_ulong) {
            break;
        }
    }
    if size > 0 as libc::c_int as libc::c_ulong {
        *errmsg = b"file too short\0" as *const u8 as *const libc::c_char;
        *err = 0 as libc::c_int;
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn simple_object_internal_write(
    mut descriptor: libc::c_int,
    mut offset: off_t,
    mut buffer: *const libc::c_uchar,
    mut size: size_t,
    mut errmsg: *mut *const libc::c_char,
    mut err: *mut libc::c_int,
) -> libc::c_int {
    if lseek(descriptor, offset, 0 as libc::c_int) < 0 as libc::c_int as libc::c_long {
        *errmsg = b"lseek\0" as *const u8 as *const libc::c_char;
        *err = *__errno_location();
        return 0 as libc::c_int;
    }
    loop {
        let mut wrote: ssize_t = write(descriptor, buffer as *const libc::c_void, size);
        if wrote == 0 as libc::c_int as libc::c_long {
            break;
        }
        if wrote > 0 as libc::c_int as libc::c_long {
            buffer = buffer.offset(wrote as isize);
            size = (size as libc::c_ulong).wrapping_sub(wrote as libc::c_ulong) as size_t
                as size_t;
        } else if *__errno_location() != 4 as libc::c_int {
            *errmsg = b"write\0" as *const u8 as *const libc::c_char;
            *err = *__errno_location();
            return 0 as libc::c_int;
        }
        if !(size > 0 as libc::c_int as libc::c_ulong) {
            break;
        }
    }
    if size > 0 as libc::c_int as libc::c_ulong {
        *errmsg = b"short write\0" as *const u8 as *const libc::c_char;
        *err = 0 as libc::c_int;
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn simple_object_start_read(
    mut descriptor: libc::c_int,
    mut offset: off_t,
    mut segment_name: *const libc::c_char,
    mut errmsg: *mut *const libc::c_char,
    mut err: *mut libc::c_int,
) -> *mut simple_object_read {
    let mut header: [libc::c_uchar; 16] = [0; 16];
    let mut len: size_t = 0;
    let mut i: size_t = 0;
    if simple_object_internal_read(
        descriptor,
        offset,
        header.as_mut_ptr(),
        16 as libc::c_int as size_t,
        errmsg,
        err,
    ) == 0
    {
        return 0 as *mut simple_object_read;
    }
    len = (::core::mem::size_of::<[*const simple_object_functions; 4]>()
        as libc::c_ulong)
        .wrapping_div(
            ::core::mem::size_of::<*const simple_object_functions>() as libc::c_ulong,
        );
    i = 0 as libc::c_int as size_t;
    while i < len {
        let mut data: *mut libc::c_void = 0 as *mut libc::c_void;
        data = ((*format_functions[i as usize]).match_0)
            .expect(
                "non-null function pointer",
            )(header.as_mut_ptr(), descriptor, offset, segment_name, errmsg, err);
        if !data.is_null() {
            let mut ret: *mut simple_object_read = 0 as *mut simple_object_read;
            ret = xmalloc(::core::mem::size_of::<simple_object_read>() as libc::c_ulong)
                as *mut simple_object_read;
            (*ret).descriptor = descriptor;
            (*ret).offset = offset;
            (*ret).functions = format_functions[i as usize];
            (*ret).data = data;
            return ret;
        }
        i = i.wrapping_add(1);
        i;
    }
    *errmsg = b"file not recognized\0" as *const u8 as *const libc::c_char;
    *err = 0 as libc::c_int;
    return 0 as *mut simple_object_read;
}
#[no_mangle]
pub unsafe extern "C" fn simple_object_find_sections(
    mut sobj: *mut simple_object_read,
    mut pfn: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const libc::c_char,
            off_t,
            off_t,
        ) -> libc::c_int,
    >,
    mut data: *mut libc::c_void,
    mut err: *mut libc::c_int,
) -> *const libc::c_char {
    return ((*(*sobj).functions).find_sections)
        .expect("non-null function pointer")(sobj, pfn, data, err);
}
unsafe extern "C" fn find_one_section(
    mut data: *mut libc::c_void,
    mut name: *const libc::c_char,
    mut offset: off_t,
    mut length: off_t,
) -> libc::c_int {
    let mut fosd: *mut find_one_section_data = data as *mut find_one_section_data;
    if strcmp(name, (*fosd).name) != 0 as libc::c_int {
        return 1 as libc::c_int;
    }
    *(*fosd).offset = offset;
    *(*fosd).length = length;
    (*fosd).found = 1 as libc::c_int;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn simple_object_find_section(
    mut sobj: *mut simple_object_read,
    mut name: *const libc::c_char,
    mut offset: *mut off_t,
    mut length: *mut off_t,
    mut errmsg: *mut *const libc::c_char,
    mut err: *mut libc::c_int,
) -> libc::c_int {
    let mut fosd: find_one_section_data = find_one_section_data {
        name: 0 as *const libc::c_char,
        offset: 0 as *mut off_t,
        length: 0 as *mut off_t,
        found: 0,
    };
    fosd.name = name;
    fosd.offset = offset;
    fosd.length = length;
    fosd.found = 0 as libc::c_int;
    *errmsg = simple_object_find_sections(
        sobj,
        Some(
            find_one_section
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const libc::c_char,
                    off_t,
                    off_t,
                ) -> libc::c_int,
        ),
        &mut fosd as *mut find_one_section_data as *mut libc::c_void,
        err,
    );
    if !(*errmsg).is_null() {
        return 0 as libc::c_int;
    }
    if fosd.found == 0 {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn handle_lto_debug_sections(
    mut name: *const libc::c_char,
    mut rename: libc::c_int,
) -> *mut libc::c_char {
    let mut newname: *mut libc::c_char = if rename != 0 {
        xcalloc(
            (strlen(name)).wrapping_add(1 as libc::c_int as libc::c_ulong),
            ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
        ) as *mut libc::c_char
    } else {
        xstrdup(name)
    };
    if strncmp(
        name,
        b".rela\0" as *const u8 as *const libc::c_char,
        (::core::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    ) == 0 as libc::c_int
    {
        if rename != 0 {
            strncpy(
                newname,
                name,
                (::core::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            );
        }
        name = name
            .offset(
                (::core::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
            );
    } else if strncmp(
        name,
        b".rel\0" as *const u8 as *const libc::c_char,
        (::core::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    ) == 0 as libc::c_int
    {
        if rename != 0 {
            strncpy(
                newname,
                name,
                (::core::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            );
        }
        name = name
            .offset(
                (::core::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
            );
    }
    if strncmp(
        name,
        b".gnu.debuglto_\0" as *const u8 as *const libc::c_char,
        (::core::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    ) == 0 as libc::c_int
    {
        return if rename != 0 {
            strcat(
                newname,
                name
                    .offset(
                        ::core::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                            as isize,
                    )
                    .offset(-(1 as libc::c_int as isize)),
            )
        } else {
            newname
        }
    } else if strncmp(
        name,
        b".gnu.lto_.debug_\0" as *const u8 as *const libc::c_char,
        (::core::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    ) == 0 as libc::c_int
    {
        return if rename != 0 {
            strcat(
                newname,
                name
                    .offset(
                        ::core::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong
                            as isize,
                    )
                    .offset(-(1 as libc::c_int as isize)),
            )
        } else {
            newname
        }
    } else if strcmp(name, b".note.GNU-stack\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        return strcpy(newname, name)
    } else if strcmp(name, b".note.gnu.property\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        return strcpy(newname, name)
    } else if strcmp(name, b".comment\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        return strcpy(newname, name)
    } else if strcmp(name, b".GCC.command.line\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        return strcpy(newname, name)
    } else if strcmp(name, b".ctf\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        return strcpy(newname, name)
    } else if strcmp(name, b".BTF\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        return strcpy(newname, name)
    }
    free(newname as *mut libc::c_void);
    return 0 as *mut libc::c_char;
}
unsafe extern "C" fn handle_lto_debug_sections_rename(
    mut name: *const libc::c_char,
) -> *mut libc::c_char {
    return handle_lto_debug_sections(name, 1 as libc::c_int);
}
unsafe extern "C" fn handle_lto_debug_sections_norename(
    mut name: *const libc::c_char,
) -> *mut libc::c_char {
    return handle_lto_debug_sections(name, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn simple_object_copy_lto_debug_sections(
    mut sobj: *mut simple_object_read,
    mut dest: *const libc::c_char,
    mut err: *mut libc::c_int,
    mut rename: libc::c_int,
) -> *const libc::c_char {
    let mut errmsg: *const libc::c_char = 0 as *const libc::c_char;
    let mut dest_sobj: *mut simple_object_write = 0 as *mut simple_object_write;
    let mut attrs: *mut simple_object_attributes = 0 as *mut simple_object_attributes;
    let mut outfd: libc::c_int = 0;
    if ((*(*sobj).functions).copy_lto_debug_sections).is_none() {
        *err = 22 as libc::c_int;
        return b"simple_object_copy_lto_debug_sections not implemented\0" as *const u8
            as *const libc::c_char;
    }
    attrs = simple_object_fetch_attributes(sobj, &mut errmsg, err);
    if attrs.is_null() {
        return errmsg;
    }
    dest_sobj = simple_object_start_write(
        attrs,
        0 as *const libc::c_char,
        &mut errmsg,
        err,
    );
    simple_object_release_attributes(attrs);
    if dest_sobj.is_null() {
        return errmsg;
    }
    errmsg = ((*(*sobj).functions).copy_lto_debug_sections)
        .expect(
            "non-null function pointer",
        )(
        sobj,
        dest_sobj,
        if rename != 0 {
            Some(
                handle_lto_debug_sections_rename
                    as unsafe extern "C" fn(*const libc::c_char) -> *mut libc::c_char,
            )
        } else {
            Some(
                handle_lto_debug_sections_norename
                    as unsafe extern "C" fn(*const libc::c_char) -> *mut libc::c_char,
            )
        },
        err,
    );
    if !errmsg.is_null() {
        simple_object_release_write(dest_sobj);
        return errmsg;
    }
    outfd = open(
        dest,
        0o100 as libc::c_int | 0o1 as libc::c_int | 0o1000 as libc::c_int
            | 0 as libc::c_int,
        0o777 as libc::c_int,
    );
    if outfd == -(1 as libc::c_int) {
        *err = *__errno_location();
        simple_object_release_write(dest_sobj);
        return b"open failed\0" as *const u8 as *const libc::c_char;
    }
    errmsg = simple_object_write_to_file(dest_sobj, outfd, err);
    close(outfd);
    if !errmsg.is_null() {
        simple_object_release_write(dest_sobj);
        return errmsg;
    }
    simple_object_release_write(dest_sobj);
    return 0 as *const libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn simple_object_fetch_attributes(
    mut sobj: *mut simple_object_read,
    mut errmsg: *mut *const libc::c_char,
    mut err: *mut libc::c_int,
) -> *mut simple_object_attributes {
    let mut data: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut ret: *mut simple_object_attributes = 0 as *mut simple_object_attributes;
    data = ((*(*sobj).functions).fetch_attributes)
        .expect("non-null function pointer")(sobj, errmsg, err);
    if data.is_null() {
        return 0 as *mut simple_object_attributes;
    }
    ret = xmalloc(::core::mem::size_of::<simple_object_attributes>() as libc::c_ulong)
        as *mut simple_object_attributes;
    (*ret).functions = (*sobj).functions;
    (*ret).data = data;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn simple_object_release_read(mut sobj: *mut simple_object_read) {
    ((*(*sobj).functions).release_read)
        .expect("non-null function pointer")((*sobj).data);
    free(sobj as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn simple_object_attributes_merge(
    mut to: *mut simple_object_attributes,
    mut from: *mut simple_object_attributes,
    mut err: *mut libc::c_int,
) -> *const libc::c_char {
    if (*to).functions != (*from).functions {
        *err = 0 as libc::c_int;
        return b"different object file format\0" as *const u8 as *const libc::c_char;
    }
    return ((*(*to).functions).attributes_merge)
        .expect("non-null function pointer")((*to).data, (*from).data, err);
}
#[no_mangle]
pub unsafe extern "C" fn simple_object_release_attributes(
    mut attrs: *mut simple_object_attributes,
) {
    ((*(*attrs).functions).release_attributes)
        .expect("non-null function pointer")((*attrs).data);
    free(attrs as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn simple_object_start_write(
    mut attrs: *mut simple_object_attributes,
    mut segment_name: *const libc::c_char,
    mut errmsg: *mut *const libc::c_char,
    mut err: *mut libc::c_int,
) -> *mut simple_object_write {
    let mut data: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut ret: *mut simple_object_write = 0 as *mut simple_object_write;
    data = ((*(*attrs).functions).start_write)
        .expect("non-null function pointer")((*attrs).data, errmsg, err);
    if data.is_null() {
        return 0 as *mut simple_object_write;
    }
    ret = xmalloc(::core::mem::size_of::<simple_object_write>() as libc::c_ulong)
        as *mut simple_object_write;
    (*ret).functions = (*attrs).functions;
    (*ret)
        .segment_name = if !segment_name.is_null() {
        xstrdup(segment_name)
    } else {
        0 as *mut libc::c_char
    };
    (*ret).sections = 0 as *mut simple_object_write_section;
    (*ret).last_section = 0 as *mut simple_object_write_section;
    (*ret).data = data;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn simple_object_write_create_section(
    mut sobj: *mut simple_object_write,
    mut name: *const libc::c_char,
    mut align: libc::c_uint,
    mut errmsg: *mut *const libc::c_char,
    mut err: *mut libc::c_int,
) -> *mut simple_object_write_section {
    let mut ret: *mut simple_object_write_section = 0
        as *mut simple_object_write_section;
    ret = xmalloc(::core::mem::size_of::<simple_object_write_section>() as libc::c_ulong)
        as *mut simple_object_write_section;
    (*ret).next = 0 as *mut simple_object_write_section;
    (*ret).name = xstrdup(name);
    (*ret).align = align;
    (*ret).buffers = 0 as *mut simple_object_write_section_buffer;
    (*ret).last_buffer = 0 as *mut simple_object_write_section_buffer;
    if ((*sobj).last_section).is_null() {
        (*sobj).sections = ret;
        (*sobj).last_section = ret;
    } else {
        (*(*sobj).last_section).next = ret;
        (*sobj).last_section = ret;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn simple_object_write_add_data(
    mut sobj: *mut simple_object_write,
    mut section: *mut simple_object_write_section,
    mut buffer: *const libc::c_void,
    mut size: size_t,
    mut copy: libc::c_int,
    mut err: *mut libc::c_int,
) -> *const libc::c_char {
    let mut wsb: *mut simple_object_write_section_buffer = 0
        as *mut simple_object_write_section_buffer;
    wsb = xmalloc(
        ::core::mem::size_of::<simple_object_write_section_buffer>() as libc::c_ulong,
    ) as *mut simple_object_write_section_buffer;
    (*wsb).next = 0 as *mut simple_object_write_section_buffer;
    (*wsb).size = size;
    if copy == 0 {
        (*wsb).buffer = buffer;
        (*wsb).free_buffer = 0 as *mut libc::c_void;
    } else {
        (*wsb)
            .free_buffer = xmalloc(
            (::core::mem::size_of::<libc::c_char>() as libc::c_ulong).wrapping_mul(size),
        ) as *mut libc::c_char as *mut libc::c_void;
        memcpy((*wsb).free_buffer, buffer, size);
        (*wsb).buffer = (*wsb).free_buffer;
    }
    if ((*section).last_buffer).is_null() {
        (*section).buffers = wsb;
        (*section).last_buffer = wsb;
    } else {
        (*(*section).last_buffer).next = wsb;
        (*section).last_buffer = wsb;
    }
    return 0 as *const libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn simple_object_write_to_file(
    mut sobj: *mut simple_object_write,
    mut descriptor: libc::c_int,
    mut err: *mut libc::c_int,
) -> *const libc::c_char {
    return ((*(*sobj).functions).write_to_file)
        .expect("non-null function pointer")(sobj, descriptor, err);
}
#[no_mangle]
pub unsafe extern "C" fn simple_object_release_write(
    mut sobj: *mut simple_object_write,
) {
    let mut section: *mut simple_object_write_section = 0
        as *mut simple_object_write_section;
    free((*sobj).segment_name as *mut libc::c_void);
    section = (*sobj).sections;
    while !section.is_null() {
        let mut buffer: *mut simple_object_write_section_buffer = 0
            as *mut simple_object_write_section_buffer;
        let mut next_section: *mut simple_object_write_section = 0
            as *mut simple_object_write_section;
        buffer = (*section).buffers;
        while !buffer.is_null() {
            let mut next_buffer: *mut simple_object_write_section_buffer = 0
                as *mut simple_object_write_section_buffer;
            if !((*buffer).free_buffer).is_null() {
                free((*buffer).free_buffer);
            }
            next_buffer = (*buffer).next;
            free(buffer as *mut libc::c_void);
            buffer = next_buffer;
        }
        next_section = (*section).next;
        free((*section).name as *mut libc::c_void);
        free(section as *mut libc::c_void);
        section = next_section;
    }
    ((*(*sobj).functions).release_write)
        .expect("non-null function pointer")((*sobj).data);
    free(sobj as *mut libc::c_void);
}
