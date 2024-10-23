use ::libc;
extern "C" {
    pub type internal_state;
    fn inflate(strm: z_streamp, flush: libc::c_int) -> libc::c_int;
    fn inflateEnd(strm: z_streamp) -> libc::c_int;
    fn inflateInit_(
        strm: z_streamp,
        version: *const libc::c_char,
        stream_size: libc::c_int,
    ) -> libc::c_int;
}
pub type Byte = libc::c_uchar;
pub type uInt = libc::c_uint;
pub type uLong = libc::c_ulong;
pub type Bytef = Byte;
pub type uLongf = uLong;
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
pub unsafe extern "C" fn uncompress2(
    mut dest: *mut Bytef,
    mut destLen: *mut uLongf,
    mut source: *const Bytef,
    mut sourceLen: *mut uLong,
) -> libc::c_int {
    let mut stream: z_stream = z_stream {
        next_in: 0 as *mut Bytef,
        avail_in: 0,
        total_in: 0,
        next_out: 0 as *mut Bytef,
        avail_out: 0,
        total_out: 0,
        msg: 0 as *mut libc::c_char,
        state: 0 as *mut internal_state,
        zalloc: None,
        zfree: None,
        opaque: 0 as *mut libc::c_void,
        data_type: 0,
        adler: 0,
        reserved: 0,
    };
    let mut err: libc::c_int = 0;
    let max: uInt = -(1 as libc::c_int) as uInt;
    let mut len: uLong = 0;
    let mut left: uLong = 0;
    let mut buf: [Byte; 1] = [0; 1];
    len = *sourceLen;
    if *destLen != 0 {
        left = *destLen;
        *destLen = 0 as libc::c_int as uLongf;
    } else {
        left = 1 as libc::c_int as uLong;
        dest = buf.as_mut_ptr();
    }
    stream.next_in = source as *mut Bytef;
    stream.avail_in = 0 as libc::c_int as uInt;
    stream.zalloc = None;
    stream.zfree = None;
    stream.opaque = 0 as voidpf;
    err = inflateInit_(
        &mut stream,
        b"1.2.11\0" as *const u8 as *const libc::c_char,
        ::core::mem::size_of::<z_stream>() as libc::c_ulong as libc::c_int,
    );
    if err != 0 as libc::c_int {
        return err;
    }
    stream.next_out = dest;
    stream.avail_out = 0 as libc::c_int as uInt;
    loop {
        if stream.avail_out == 0 as libc::c_int as libc::c_uint {
            stream.avail_out = if left > max as uLong { max } else { left as uInt };
            left = (left as libc::c_ulong)
                .wrapping_sub(stream.avail_out as libc::c_ulong) as uLong as uLong;
        }
        if stream.avail_in == 0 as libc::c_int as libc::c_uint {
            stream.avail_in = if len > max as uLong { max } else { len as uInt };
            len = (len as libc::c_ulong).wrapping_sub(stream.avail_in as libc::c_ulong)
                as uLong as uLong;
        }
        err = inflate(&mut stream, 0 as libc::c_int);
        if !(err == 0 as libc::c_int) {
            break;
        }
    }
    *sourceLen = (*sourceLen as libc::c_ulong)
        .wrapping_sub(len.wrapping_add(stream.avail_in as libc::c_ulong)) as uLong
        as uLong;
    if dest != buf.as_mut_ptr() {
        *destLen = stream.total_out;
    } else if stream.total_out != 0 && err == -(5 as libc::c_int) {
        left = 1 as libc::c_int as uLong;
    }
    inflateEnd(&mut stream);
    return if err == 1 as libc::c_int {
        0 as libc::c_int
    } else if err == 2 as libc::c_int {
        -(3 as libc::c_int)
    } else if err == -(5 as libc::c_int)
        && left.wrapping_add(stream.avail_out as libc::c_ulong) != 0
    {
        -(3 as libc::c_int)
    } else {
        err
    };
}
#[no_mangle]
pub unsafe extern "C" fn uncompress(
    mut dest: *mut Bytef,
    mut destLen: *mut uLongf,
    mut source: *const Bytef,
    mut sourceLen: uLong,
) -> libc::c_int {
    return uncompress2(dest, destLen, source, &mut sourceLen);
}
