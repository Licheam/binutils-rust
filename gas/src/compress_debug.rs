extern "C" {
    pub type internal_state;
    fn deflateInit_(
        strm: z_streamp,
        level: libc::c_int,
        version: *const libc::c_char,
        stream_size: libc::c_int,
    ) -> libc::c_int;
    fn deflate(strm: z_streamp, flush: libc::c_int) -> libc::c_int;
    fn deflateEnd(strm: z_streamp) -> libc::c_int;
}
pub type Byte = libc::c_uchar;
pub type uInt = libc::c_uint;
pub type uLong = libc::c_ulong;
pub type Bytef = Byte;
pub type voidpf = *mut libc::c_void;
pub type alloc_func = Option::<unsafe extern "C" fn(voidpf, uInt, uInt) -> voidpf>;
pub type free_func = Option::<unsafe extern "C" fn(voidpf, voidpf) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct z_stream_s {
    pub next_in: *mut Bytef,
    pub avail_in: uInt,
    pub total_in: uLong,
    pub next_out: *mut Bytef,
    pub avail_out: uInt,
    pub total_out: uLong,
    pub msg: *mut libc::c_char,
    pub state: *mut internal_state,
    pub zalloc: alloc_func,
    pub zfree: free_func,
    pub opaque: voidpf,
    pub data_type: libc::c_int,
    pub adler: uLong,
    pub reserved: uLong,
}
pub type z_stream = z_stream_s;
pub type z_streamp = *mut z_stream;
#[no_mangle]
pub unsafe extern "C" fn compress_init() -> *mut z_stream_s {
    static mut strm: z_stream_s = z_stream_s {
        next_in: 0 as *const Bytef as *mut Bytef,
        avail_in: 0,
        total_in: 0,
        next_out: 0 as *const Bytef as *mut Bytef,
        avail_out: 0,
        total_out: 0,
        msg: 0 as *const libc::c_char as *mut libc::c_char,
        state: 0 as *const internal_state as *mut internal_state,
        zalloc: None,
        zfree: None,
        opaque: 0 as *const libc::c_void as *mut libc::c_void,
        data_type: 0,
        adler: 0,
        reserved: 0,
    };
    strm.zalloc = None;
    strm.zfree = None;
    strm.opaque = 0 as *mut libc::c_void;
    deflateInit_(
        &mut strm,
        -(1 as libc::c_int),
        b"1.2.11\0" as *const u8 as *const libc::c_char,
        ::core::mem::size_of::<z_stream>() as libc::c_ulong as libc::c_int,
    );
    return &mut strm;
}
#[no_mangle]
pub unsafe extern "C" fn compress_data(
    mut strm: *mut z_stream_s,
    mut next_in: *mut *const libc::c_char,
    mut avail_in: *mut libc::c_int,
    mut next_out: *mut *mut libc::c_char,
    mut avail_out: *mut libc::c_int,
) -> libc::c_int {
    let mut out_size: libc::c_int = 0 as libc::c_int;
    let mut x: libc::c_int = 0;
    (*strm).next_in = *next_in as *mut Bytef;
    (*strm).avail_in = *avail_in as uInt;
    (*strm).next_out = *next_out as *mut Bytef;
    (*strm).avail_out = *avail_out as uInt;
    x = deflate(strm, 0 as libc::c_int);
    if x != 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    out_size = (*avail_out as libc::c_uint).wrapping_sub((*strm).avail_out)
        as libc::c_int;
    *next_in = (*strm).next_in as *mut libc::c_char;
    *avail_in = (*strm).avail_in as libc::c_int;
    *next_out = (*strm).next_out as *mut libc::c_char;
    *avail_out = (*strm).avail_out as libc::c_int;
    return out_size;
}
#[no_mangle]
pub unsafe extern "C" fn compress_finish(
    mut strm: *mut z_stream_s,
    mut next_out: *mut *mut libc::c_char,
    mut avail_out: *mut libc::c_int,
    mut out_size: *mut libc::c_int,
) -> libc::c_int {
    let mut x: libc::c_int = 0;
    (*strm).avail_in = 0 as libc::c_int as uInt;
    (*strm).next_out = *next_out as *mut Bytef;
    (*strm).avail_out = *avail_out as uInt;
    x = deflate(strm, 4 as libc::c_int);
    *out_size = (*avail_out as libc::c_uint).wrapping_sub((*strm).avail_out)
        as libc::c_int;
    *next_out = (*strm).next_out as *mut libc::c_char;
    *avail_out = (*strm).avail_out as libc::c_int;
    if x == 1 as libc::c_int {
        deflateEnd(strm);
        return 0 as libc::c_int;
    }
    if (*strm).avail_out != 0 as libc::c_int as libc::c_uint {
        return -(1 as libc::c_int);
    }
    return 1 as libc::c_int;
}
