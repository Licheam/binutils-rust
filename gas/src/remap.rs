extern "C" {
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn concat(_: *const libc::c_char, _: ...) -> *mut libc::c_char;
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn as_fatal(format: *const libc::c_char, _: ...) -> !;
    fn filename_ncmp(
        s1: *const libc::c_char,
        s2: *const libc::c_char,
        n: size_t,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct debug_prefix_map {
    pub old_prefix: *const libc::c_char,
    pub new_prefix: *const libc::c_char,
    pub old_len: size_t,
    pub new_len: size_t,
    pub next: *mut debug_prefix_map,
}
#[no_mangle]
pub unsafe extern "C" fn add_debug_prefix_map(mut arg: *const libc::c_char) {
    let mut map: *mut debug_prefix_map = 0 as *mut debug_prefix_map;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut o: *mut libc::c_char = 0 as *mut libc::c_char;
    p = strchr(arg, '=' as i32);
    if p.is_null() {
        as_fatal(
            dcgettext(
                0 as *const libc::c_char,
                b"invalid argument '%s' to -fdebug-prefix-map\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            arg,
        );
    }
    map = xmalloc(::core::mem::size_of::<debug_prefix_map>() as libc::c_ulong)
        as *mut debug_prefix_map;
    o = xstrdup(arg);
    (*map).old_prefix = o;
    (*map).old_len = p.offset_from(arg) as libc::c_long as size_t;
    *o.offset((*map).old_len as isize) = 0 as libc::c_int as libc::c_char;
    p = p.offset(1);
    p;
    (*map).new_prefix = xstrdup(p);
    (*map).new_len = strlen(p);
    (*map).next = debug_prefix_maps;
    debug_prefix_maps = map;
}
#[no_mangle]
pub unsafe extern "C" fn remap_debug_filename(
    mut filename: *const libc::c_char,
) -> *const libc::c_char {
    let mut map: *mut debug_prefix_map = 0 as *mut debug_prefix_map;
    map = debug_prefix_maps;
    while !map.is_null() {
        if filename_ncmp(filename, (*map).old_prefix, (*map).old_len) == 0 as libc::c_int
        {
            let mut name: *const libc::c_char = filename.offset((*map).old_len as isize);
            return concat((*map).new_prefix, name, 0 as *mut libc::c_void);
        }
        map = (*map).next;
    }
    return xstrdup(filename);
}
#[no_mangle]
pub static mut debug_prefix_maps: *mut debug_prefix_map = 0 as *const debug_prefix_map
    as *mut debug_prefix_map;
