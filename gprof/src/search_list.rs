extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn xmalloc(_: size_t) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct search_list_elem {
    pub next: *mut search_list_elem,
    pub path: [libc::c_char; 1],
}
pub type Search_List_Elem = search_list_elem;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Search_List {
    pub head: *mut search_list_elem,
    pub tail: *mut search_list_elem,
}
#[no_mangle]
pub unsafe extern "C" fn search_list_append(
    mut list: *mut Search_List,
    mut paths: *const libc::c_char,
) {
    let mut new_el: *mut Search_List_Elem = 0 as *mut Search_List_Elem;
    let mut beg: *const libc::c_char = 0 as *const libc::c_char;
    let mut colon: *const libc::c_char = 0 as *const libc::c_char;
    let mut len: libc::c_uint = 0;
    colon = paths.offset(-(1 as libc::c_int as isize));
    loop {
        beg = colon.offset(1 as libc::c_int as isize);
        colon = strchr(beg, ':' as i32);
        if !colon.is_null() {
            len = colon.offset_from(beg) as libc::c_long as libc::c_uint;
        } else {
            len = strlen(beg) as libc::c_uint;
        }
        new_el = xmalloc(
            (::core::mem::size_of::<Search_List_Elem>() as libc::c_ulong)
                .wrapping_add(len as libc::c_ulong),
        ) as *mut Search_List_Elem;
        memcpy(
            ((*new_el).path).as_mut_ptr() as *mut libc::c_void,
            beg as *const libc::c_void,
            len as libc::c_ulong,
        );
        *((*new_el).path)
            .as_mut_ptr()
            .offset(len as isize) = '\0' as i32 as libc::c_char;
        (*new_el).next = 0 as *mut search_list_elem;
        if !((*list).tail).is_null() {
            (*(*list).tail).next = new_el;
        } else {
            (*list).head = new_el;
        }
        (*list).tail = new_el;
        if colon.is_null() {
            break;
        }
    };
}
