use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn feof(__stream: *mut FILE) -> libc::c_int;
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
pub type uint32_t = __uint32_t;
pub type sha1_uint32 = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sha1_ctx {
    pub A: sha1_uint32,
    pub B: sha1_uint32,
    pub C: sha1_uint32,
    pub D: sha1_uint32,
    pub E: sha1_uint32,
    pub total: [sha1_uint32; 2],
    pub buflen: sha1_uint32,
    pub buffer: [sha1_uint32; 32],
}
static mut fillbuf: [libc::c_uchar; 64] = [
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
];
#[no_mangle]
pub unsafe extern "C" fn sha1_init_ctx(mut ctx: *mut sha1_ctx) {
    (*ctx).A = 0x67452301 as libc::c_int as sha1_uint32;
    (*ctx).B = 0xefcdab89 as libc::c_uint;
    (*ctx).C = 0x98badcfe as libc::c_uint;
    (*ctx).D = 0x10325476 as libc::c_int as sha1_uint32;
    (*ctx).E = 0xc3d2e1f0 as libc::c_uint;
    (*ctx).total[1 as libc::c_int as usize] = 0 as libc::c_int as sha1_uint32;
    (*ctx).total[0 as libc::c_int as usize] = (*ctx).total[1 as libc::c_int as usize];
    (*ctx).buflen = 0 as libc::c_int as sha1_uint32;
}
#[no_mangle]
pub unsafe extern "C" fn sha1_read_ctx(
    mut ctx: *const sha1_ctx,
    mut resbuf: *mut libc::c_void,
) -> *mut libc::c_void {
    *(resbuf as *mut sha1_uint32)
        .offset(
            0 as libc::c_int as isize,
        ) = (*ctx).A << 24 as libc::c_int
        | ((*ctx).A & 0xff00 as libc::c_int as libc::c_uint) << 8 as libc::c_int
        | (*ctx).A >> 8 as libc::c_int & 0xff00 as libc::c_int as libc::c_uint
        | (*ctx).A >> 24 as libc::c_int;
    *(resbuf as *mut sha1_uint32)
        .offset(
            1 as libc::c_int as isize,
        ) = (*ctx).B << 24 as libc::c_int
        | ((*ctx).B & 0xff00 as libc::c_int as libc::c_uint) << 8 as libc::c_int
        | (*ctx).B >> 8 as libc::c_int & 0xff00 as libc::c_int as libc::c_uint
        | (*ctx).B >> 24 as libc::c_int;
    *(resbuf as *mut sha1_uint32)
        .offset(
            2 as libc::c_int as isize,
        ) = (*ctx).C << 24 as libc::c_int
        | ((*ctx).C & 0xff00 as libc::c_int as libc::c_uint) << 8 as libc::c_int
        | (*ctx).C >> 8 as libc::c_int & 0xff00 as libc::c_int as libc::c_uint
        | (*ctx).C >> 24 as libc::c_int;
    *(resbuf as *mut sha1_uint32)
        .offset(
            3 as libc::c_int as isize,
        ) = (*ctx).D << 24 as libc::c_int
        | ((*ctx).D & 0xff00 as libc::c_int as libc::c_uint) << 8 as libc::c_int
        | (*ctx).D >> 8 as libc::c_int & 0xff00 as libc::c_int as libc::c_uint
        | (*ctx).D >> 24 as libc::c_int;
    *(resbuf as *mut sha1_uint32)
        .offset(
            4 as libc::c_int as isize,
        ) = (*ctx).E << 24 as libc::c_int
        | ((*ctx).E & 0xff00 as libc::c_int as libc::c_uint) << 8 as libc::c_int
        | (*ctx).E >> 8 as libc::c_int & 0xff00 as libc::c_int as libc::c_uint
        | (*ctx).E >> 24 as libc::c_int;
    return resbuf;
}
#[no_mangle]
pub unsafe extern "C" fn sha1_finish_ctx(
    mut ctx: *mut sha1_ctx,
    mut resbuf: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut bytes: sha1_uint32 = (*ctx).buflen;
    let mut size: size_t = (if bytes < 56 as libc::c_int as libc::c_uint {
        64 as libc::c_int / 4 as libc::c_int
    } else {
        64 as libc::c_int * 2 as libc::c_int / 4 as libc::c_int
    }) as size_t;
    (*ctx)
        .total[0 as libc::c_int
        as usize] = ((*ctx).total[0 as libc::c_int as usize] as libc::c_uint)
        .wrapping_add(bytes) as sha1_uint32 as sha1_uint32;
    if (*ctx).total[0 as libc::c_int as usize] < bytes {
        (*ctx)
            .total[1 as libc::c_int
            as usize] = ((*ctx).total[1 as libc::c_int as usize]).wrapping_add(1);
        (*ctx).total[1 as libc::c_int as usize];
    }
    (*ctx)
        .buffer[size.wrapping_sub(2 as libc::c_int as libc::c_ulong)
        as usize] = ((*ctx).total[1 as libc::c_int as usize] << 3 as libc::c_int
        | (*ctx).total[0 as libc::c_int as usize] >> 29 as libc::c_int)
        << 24 as libc::c_int
        | (((*ctx).total[1 as libc::c_int as usize] << 3 as libc::c_int
            | (*ctx).total[0 as libc::c_int as usize] >> 29 as libc::c_int)
            & 0xff00 as libc::c_int as libc::c_uint) << 8 as libc::c_int
        | ((*ctx).total[1 as libc::c_int as usize] << 3 as libc::c_int
            | (*ctx).total[0 as libc::c_int as usize] >> 29 as libc::c_int)
            >> 8 as libc::c_int & 0xff00 as libc::c_int as libc::c_uint
        | ((*ctx).total[1 as libc::c_int as usize] << 3 as libc::c_int
            | (*ctx).total[0 as libc::c_int as usize] >> 29 as libc::c_int)
            >> 24 as libc::c_int;
    (*ctx)
        .buffer[size.wrapping_sub(1 as libc::c_int as libc::c_ulong)
        as usize] = ((*ctx).total[0 as libc::c_int as usize] << 3 as libc::c_int)
        << 24 as libc::c_int
        | ((*ctx).total[0 as libc::c_int as usize] << 3 as libc::c_int
            & 0xff00 as libc::c_int as libc::c_uint) << 8 as libc::c_int
        | (*ctx).total[0 as libc::c_int as usize] << 3 as libc::c_int >> 8 as libc::c_int
            & 0xff00 as libc::c_int as libc::c_uint
        | (*ctx).total[0 as libc::c_int as usize] << 3 as libc::c_int
            >> 24 as libc::c_int;
    memcpy(
        &mut *(((*ctx).buffer).as_mut_ptr() as *mut libc::c_char).offset(bytes as isize)
            as *mut libc::c_char as *mut libc::c_void,
        fillbuf.as_ptr() as *const libc::c_void,
        size
            .wrapping_sub(2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(4 as libc::c_int as libc::c_ulong)
            .wrapping_sub(bytes as libc::c_ulong),
    );
    sha1_process_block(
        ((*ctx).buffer).as_mut_ptr() as *const libc::c_void,
        size.wrapping_mul(4 as libc::c_int as libc::c_ulong),
        ctx,
    );
    return sha1_read_ctx(ctx, resbuf);
}
#[no_mangle]
pub unsafe extern "C" fn sha1_stream(
    mut stream: *mut FILE,
    mut resblock: *mut libc::c_void,
) -> libc::c_int {
    let mut ctx: sha1_ctx = sha1_ctx {
        A: 0,
        B: 0,
        C: 0,
        D: 0,
        E: 0,
        total: [0; 2],
        buflen: 0,
        buffer: [0; 32],
    };
    let mut buffer: [libc::c_char; 4168] = [0; 4168];
    let mut sum: size_t = 0;
    sha1_init_ctx(&mut ctx);
    's_13: loop {
        let mut n: size_t = 0;
        sum = 0 as libc::c_int as size_t;
        loop {
            n = fread(
                buffer.as_mut_ptr().offset(sum as isize) as *mut libc::c_void,
                1 as libc::c_int as libc::c_ulong,
                (4096 as libc::c_int as libc::c_ulong).wrapping_sub(sum),
                stream,
            );
            sum = (sum as libc::c_ulong).wrapping_add(n) as size_t as size_t;
            if sum == 4096 as libc::c_int as libc::c_ulong {
                break;
            }
            if n == 0 as libc::c_int as libc::c_ulong {
                if ferror(stream) != 0 {
                    return 1 as libc::c_int;
                }
                break 's_13;
            } else if feof(stream) != 0 {
                break 's_13;
            }
        }
        sha1_process_block(
            buffer.as_mut_ptr() as *const libc::c_void,
            4096 as libc::c_int as size_t,
            &mut ctx,
        );
    }
    if sum > 0 as libc::c_int as libc::c_ulong {
        sha1_process_bytes(buffer.as_mut_ptr() as *const libc::c_void, sum, &mut ctx);
    }
    sha1_finish_ctx(&mut ctx, resblock);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn sha1_buffer(
    mut buffer: *const libc::c_char,
    mut len: size_t,
    mut resblock: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut ctx: sha1_ctx = sha1_ctx {
        A: 0,
        B: 0,
        C: 0,
        D: 0,
        E: 0,
        total: [0; 2],
        buflen: 0,
        buffer: [0; 32],
    };
    sha1_init_ctx(&mut ctx);
    sha1_process_bytes(buffer as *const libc::c_void, len, &mut ctx);
    return sha1_finish_ctx(&mut ctx, resblock);
}
#[no_mangle]
pub unsafe extern "C" fn sha1_process_bytes(
    mut buffer: *const libc::c_void,
    mut len: size_t,
    mut ctx: *mut sha1_ctx,
) {
    if (*ctx).buflen != 0 as libc::c_int as libc::c_uint {
        let mut left_over: size_t = (*ctx).buflen as size_t;
        let mut add: size_t = if (128 as libc::c_int as libc::c_ulong)
            .wrapping_sub(left_over) > len
        {
            len
        } else {
            (128 as libc::c_int as libc::c_ulong).wrapping_sub(left_over)
        };
        memcpy(
            &mut *(((*ctx).buffer).as_mut_ptr() as *mut libc::c_char)
                .offset(left_over as isize) as *mut libc::c_char as *mut libc::c_void,
            buffer,
            add,
        );
        (*ctx)
            .buflen = ((*ctx).buflen as libc::c_ulong).wrapping_add(add) as sha1_uint32
            as sha1_uint32;
        if (*ctx).buflen > 64 as libc::c_int as libc::c_uint {
            sha1_process_block(
                ((*ctx).buffer).as_mut_ptr() as *const libc::c_void,
                ((*ctx).buflen & !(63 as libc::c_int) as libc::c_uint) as size_t,
                ctx,
            );
            (*ctx).buflen &= 63 as libc::c_int as libc::c_uint;
            memcpy(
                ((*ctx).buffer).as_mut_ptr() as *mut libc::c_void,
                &mut *(((*ctx).buffer).as_mut_ptr() as *mut libc::c_char)
                    .offset(
                        (left_over.wrapping_add(add)
                            & !(63 as libc::c_int) as libc::c_ulong) as isize,
                    ) as *mut libc::c_char as *const libc::c_void,
                (*ctx).buflen as libc::c_ulong,
            );
        }
        buffer = (buffer as *const libc::c_char).offset(add as isize)
            as *const libc::c_void;
        len = (len as libc::c_ulong).wrapping_sub(add) as size_t as size_t;
    }
    if len >= 64 as libc::c_int as libc::c_ulong {
        if (buffer as size_t).wrapping_rem(4 as libc::c_ulong)
            != 0 as libc::c_int as libc::c_ulong
        {
            while len > 64 as libc::c_int as libc::c_ulong {
                sha1_process_block(
                    memcpy(
                        ((*ctx).buffer).as_mut_ptr() as *mut libc::c_void,
                        buffer,
                        64 as libc::c_int as libc::c_ulong,
                    ),
                    64 as libc::c_int as size_t,
                    ctx,
                );
                buffer = (buffer as *const libc::c_char)
                    .offset(64 as libc::c_int as isize) as *const libc::c_void;
                len = (len as libc::c_ulong)
                    .wrapping_sub(64 as libc::c_int as libc::c_ulong) as size_t
                    as size_t;
            }
        } else {
            sha1_process_block(buffer, len & !(63 as libc::c_int) as libc::c_ulong, ctx);
            buffer = (buffer as *const libc::c_char)
                .offset((len & !(63 as libc::c_int) as libc::c_ulong) as isize)
                as *const libc::c_void;
            len &= 63 as libc::c_int as libc::c_ulong;
        }
    }
    if len > 0 as libc::c_int as libc::c_ulong {
        let mut left_over_0: size_t = (*ctx).buflen as size_t;
        memcpy(
            &mut *(((*ctx).buffer).as_mut_ptr() as *mut libc::c_char)
                .offset(left_over_0 as isize) as *mut libc::c_char as *mut libc::c_void,
            buffer,
            len,
        );
        left_over_0 = (left_over_0 as libc::c_ulong).wrapping_add(len) as size_t
            as size_t;
        if left_over_0 >= 64 as libc::c_int as libc::c_ulong {
            sha1_process_block(
                ((*ctx).buffer).as_mut_ptr() as *const libc::c_void,
                64 as libc::c_int as size_t,
                ctx,
            );
            left_over_0 = (left_over_0 as libc::c_ulong)
                .wrapping_sub(64 as libc::c_int as libc::c_ulong) as size_t as size_t;
            memmove(
                ((*ctx).buffer).as_mut_ptr() as *mut libc::c_void,
                &mut *((*ctx).buffer).as_mut_ptr().offset(16 as libc::c_int as isize)
                    as *mut sha1_uint32 as *const libc::c_void,
                left_over_0,
            );
        }
        (*ctx).buflen = left_over_0 as sha1_uint32;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sha1_process_block(
    mut buffer: *const libc::c_void,
    mut len: size_t,
    mut ctx: *mut sha1_ctx,
) {
    let mut words: *const sha1_uint32 = buffer as *const sha1_uint32;
    let mut nwords: size_t = len
        .wrapping_div(::core::mem::size_of::<sha1_uint32>() as libc::c_ulong);
    let mut endp: *const sha1_uint32 = words.offset(nwords as isize);
    let mut x: [sha1_uint32; 16] = [0; 16];
    let mut a: sha1_uint32 = (*ctx).A;
    let mut b: sha1_uint32 = (*ctx).B;
    let mut c: sha1_uint32 = (*ctx).C;
    let mut d: sha1_uint32 = (*ctx).D;
    let mut e: sha1_uint32 = (*ctx).E;
    (*ctx)
        .total[0 as libc::c_int
        as usize] = ((*ctx).total[0 as libc::c_int as usize] as libc::c_ulong)
        .wrapping_add(len) as sha1_uint32 as sha1_uint32;
    (*ctx)
        .total[1 as libc::c_int
        as usize] = ((*ctx).total[1 as libc::c_int as usize] as libc::c_ulong)
        .wrapping_add(
            (len >> 31 as libc::c_int >> 1 as libc::c_int)
                .wrapping_add(
                    (((*ctx).total[0 as libc::c_int as usize] as libc::c_ulong) < len)
                        as libc::c_int as libc::c_ulong,
                ),
        ) as sha1_uint32 as sha1_uint32;
    while words < endp {
        let mut tm: sha1_uint32 = 0;
        let mut t: libc::c_int = 0;
        t = 0 as libc::c_int;
        while t < 16 as libc::c_int {
            x[t
                as usize] = *words << 24 as libc::c_int
                | (*words & 0xff00 as libc::c_int as libc::c_uint) << 8 as libc::c_int
                | *words >> 8 as libc::c_int & 0xff00 as libc::c_int as libc::c_uint
                | *words >> 24 as libc::c_int;
            words = words.offset(1);
            words;
            t += 1;
            t;
        }
        e = (e as libc::c_uint)
            .wrapping_add(
                (a << 5 as libc::c_int | a >> 32 as libc::c_int - 5 as libc::c_int)
                    .wrapping_add(d ^ b & (c ^ d))
                    .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                    .wrapping_add(x[0 as libc::c_int as usize]),
            ) as sha1_uint32 as sha1_uint32;
        b = b << 30 as libc::c_int | b >> 32 as libc::c_int - 30 as libc::c_int;
        d = (d as libc::c_uint)
            .wrapping_add(
                (e << 5 as libc::c_int | e >> 32 as libc::c_int - 5 as libc::c_int)
                    .wrapping_add(c ^ a & (b ^ c))
                    .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                    .wrapping_add(x[1 as libc::c_int as usize]),
            ) as sha1_uint32 as sha1_uint32;
        a = a << 30 as libc::c_int | a >> 32 as libc::c_int - 30 as libc::c_int;
        c = (c as libc::c_uint)
            .wrapping_add(
                (d << 5 as libc::c_int | d >> 32 as libc::c_int - 5 as libc::c_int)
                    .wrapping_add(b ^ e & (a ^ b))
                    .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                    .wrapping_add(x[2 as libc::c_int as usize]),
            ) as sha1_uint32 as sha1_uint32;
        e = e << 30 as libc::c_int | e >> 32 as libc::c_int - 30 as libc::c_int;
        b = (b as libc::c_uint)
            .wrapping_add(
                (c << 5 as libc::c_int | c >> 32 as libc::c_int - 5 as libc::c_int)
                    .wrapping_add(a ^ d & (e ^ a))
                    .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                    .wrapping_add(x[3 as libc::c_int as usize]),
            ) as sha1_uint32 as sha1_uint32;
        d = d << 30 as libc::c_int | d >> 32 as libc::c_int - 30 as libc::c_int;
        a = (a as libc::c_uint)
            .wrapping_add(
                (b << 5 as libc::c_int | b >> 32 as libc::c_int - 5 as libc::c_int)
                    .wrapping_add(e ^ c & (d ^ e))
                    .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                    .wrapping_add(x[4 as libc::c_int as usize]),
            ) as sha1_uint32 as sha1_uint32;
        c = c << 30 as libc::c_int | c >> 32 as libc::c_int - 30 as libc::c_int;
        e = (e as libc::c_uint)
            .wrapping_add(
                (a << 5 as libc::c_int | a >> 32 as libc::c_int - 5 as libc::c_int)
                    .wrapping_add(d ^ b & (c ^ d))
                    .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                    .wrapping_add(x[5 as libc::c_int as usize]),
            ) as sha1_uint32 as sha1_uint32;
        b = b << 30 as libc::c_int | b >> 32 as libc::c_int - 30 as libc::c_int;
        d = (d as libc::c_uint)
            .wrapping_add(
                (e << 5 as libc::c_int | e >> 32 as libc::c_int - 5 as libc::c_int)
                    .wrapping_add(c ^ a & (b ^ c))
                    .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                    .wrapping_add(x[6 as libc::c_int as usize]),
            ) as sha1_uint32 as sha1_uint32;
        a = a << 30 as libc::c_int | a >> 32 as libc::c_int - 30 as libc::c_int;
        c = (c as libc::c_uint)
            .wrapping_add(
                (d << 5 as libc::c_int | d >> 32 as libc::c_int - 5 as libc::c_int)
                    .wrapping_add(b ^ e & (a ^ b))
                    .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                    .wrapping_add(x[7 as libc::c_int as usize]),
            ) as sha1_uint32 as sha1_uint32;
        e = e << 30 as libc::c_int | e >> 32 as libc::c_int - 30 as libc::c_int;
        b = (b as libc::c_uint)
            .wrapping_add(
                (c << 5 as libc::c_int | c >> 32 as libc::c_int - 5 as libc::c_int)
                    .wrapping_add(a ^ d & (e ^ a))
                    .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                    .wrapping_add(x[8 as libc::c_int as usize]),
            ) as sha1_uint32 as sha1_uint32;
        d = d << 30 as libc::c_int | d >> 32 as libc::c_int - 30 as libc::c_int;
        a = (a as libc::c_uint)
            .wrapping_add(
                (b << 5 as libc::c_int | b >> 32 as libc::c_int - 5 as libc::c_int)
                    .wrapping_add(e ^ c & (d ^ e))
                    .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                    .wrapping_add(x[9 as libc::c_int as usize]),
            ) as sha1_uint32 as sha1_uint32;
        c = c << 30 as libc::c_int | c >> 32 as libc::c_int - 30 as libc::c_int;
        e = (e as libc::c_uint)
            .wrapping_add(
                (a << 5 as libc::c_int | a >> 32 as libc::c_int - 5 as libc::c_int)
                    .wrapping_add(d ^ b & (c ^ d))
                    .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                    .wrapping_add(x[10 as libc::c_int as usize]),
            ) as sha1_uint32 as sha1_uint32;
        b = b << 30 as libc::c_int | b >> 32 as libc::c_int - 30 as libc::c_int;
        d = (d as libc::c_uint)
            .wrapping_add(
                (e << 5 as libc::c_int | e >> 32 as libc::c_int - 5 as libc::c_int)
                    .wrapping_add(c ^ a & (b ^ c))
                    .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                    .wrapping_add(x[11 as libc::c_int as usize]),
            ) as sha1_uint32 as sha1_uint32;
        a = a << 30 as libc::c_int | a >> 32 as libc::c_int - 30 as libc::c_int;
        c = (c as libc::c_uint)
            .wrapping_add(
                (d << 5 as libc::c_int | d >> 32 as libc::c_int - 5 as libc::c_int)
                    .wrapping_add(b ^ e & (a ^ b))
                    .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                    .wrapping_add(x[12 as libc::c_int as usize]),
            ) as sha1_uint32 as sha1_uint32;
        e = e << 30 as libc::c_int | e >> 32 as libc::c_int - 30 as libc::c_int;
        b = (b as libc::c_uint)
            .wrapping_add(
                (c << 5 as libc::c_int | c >> 32 as libc::c_int - 5 as libc::c_int)
                    .wrapping_add(a ^ d & (e ^ a))
                    .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                    .wrapping_add(x[13 as libc::c_int as usize]),
            ) as sha1_uint32 as sha1_uint32;
        d = d << 30 as libc::c_int | d >> 32 as libc::c_int - 30 as libc::c_int;
        a = (a as libc::c_uint)
            .wrapping_add(
                (b << 5 as libc::c_int | b >> 32 as libc::c_int - 5 as libc::c_int)
                    .wrapping_add(e ^ c & (d ^ e))
                    .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                    .wrapping_add(x[14 as libc::c_int as usize]),
            ) as sha1_uint32 as sha1_uint32;
        c = c << 30 as libc::c_int | c >> 32 as libc::c_int - 30 as libc::c_int;
        e = (e as libc::c_uint)
            .wrapping_add(
                (a << 5 as libc::c_int | a >> 32 as libc::c_int - 5 as libc::c_int)
                    .wrapping_add(d ^ b & (c ^ d))
                    .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                    .wrapping_add(x[15 as libc::c_int as usize]),
            ) as sha1_uint32 as sha1_uint32;
        b = b << 30 as libc::c_int | b >> 32 as libc::c_int - 30 as libc::c_int;
        tm = x[(16 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(16 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(16 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(16 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(16 as libc::c_int & 0xf as libc::c_int)
            as usize] = tm << 1 as libc::c_int
            | tm >> 32 as libc::c_int - 1 as libc::c_int;
        d = (d as libc::c_uint)
            .wrapping_add(
                (e << 5 as libc::c_int | e >> 32 as libc::c_int - 5 as libc::c_int)
                    .wrapping_add(c ^ a & (b ^ c))
                    .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                    .wrapping_add(x[(16 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as sha1_uint32 as sha1_uint32;
        a = a << 30 as libc::c_int | a >> 32 as libc::c_int - 30 as libc::c_int;
        tm = x[(17 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(17 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(17 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(17 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(17 as libc::c_int & 0xf as libc::c_int)
            as usize] = tm << 1 as libc::c_int
            | tm >> 32 as libc::c_int - 1 as libc::c_int;
        c = (c as libc::c_uint)
            .wrapping_add(
                (d << 5 as libc::c_int | d >> 32 as libc::c_int - 5 as libc::c_int)
                    .wrapping_add(b ^ e & (a ^ b))
                    .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                    .wrapping_add(x[(17 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as sha1_uint32 as sha1_uint32;
        e = e << 30 as libc::c_int | e >> 32 as libc::c_int - 30 as libc::c_int;
        tm = x[(18 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(18 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(18 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(18 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(18 as libc::c_int & 0xf as libc::c_int)
            as usize] = tm << 1 as libc::c_int
            | tm >> 32 as libc::c_int - 1 as libc::c_int;
        b = (b as libc::c_uint)
            .wrapping_add(
                (c << 5 as libc::c_int | c >> 32 as libc::c_int - 5 as libc::c_int)
                    .wrapping_add(a ^ d & (e ^ a))
                    .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                    .wrapping_add(x[(18 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as sha1_uint32 as sha1_uint32;
        d = d << 30 as libc::c_int | d >> 32 as libc::c_int - 30 as libc::c_int;
        tm = x[(19 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(19 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(19 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(19 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(19 as libc::c_int & 0xf as libc::c_int)
            as usize] = tm << 1 as libc::c_int
            | tm >> 32 as libc::c_int - 1 as libc::c_int;
        a = (a as libc::c_uint)
            .wrapping_add(
                (b << 5 as libc::c_int | b >> 32 as libc::c_int - 5 as libc::c_int)
                    .wrapping_add(e ^ c & (d ^ e))
                    .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                    .wrapping_add(x[(19 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as sha1_uint32 as sha1_uint32;
        c = c << 30 as libc::c_int | c >> 32 as libc::c_int - 30 as libc::c_int;
        tm = x[(20 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(20 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(20 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(20 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(20 as libc::c_int & 0xf as libc::c_int)
            as usize] = tm << 1 as libc::c_int
            | tm >> 32 as libc::c_int - 1 as libc::c_int;
        e = (e as libc::c_uint)
            .wrapping_add(
                (a << 5 as libc::c_int | a >> 32 as libc::c_int - 5 as libc::c_int)
                    .wrapping_add(b ^ c ^ d)
                    .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
                    .wrapping_add(x[(20 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as sha1_uint32 as sha1_uint32;
        b = b << 30 as libc::c_int | b >> 32 as libc::c_int - 30 as libc::c_int;
        tm = x[(21 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(21 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(21 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(21 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(21 as libc::c_int & 0xf as libc::c_int)
            as usize] = tm << 1 as libc::c_int
            | tm >> 32 as libc::c_int - 1 as libc::c_int;
        d = (d as libc::c_uint)
            .wrapping_add(
                (e << 5 as libc::c_int | e >> 32 as libc::c_int - 5 as libc::c_int)
                    .wrapping_add(a ^ b ^ c)
                    .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
                    .wrapping_add(x[(21 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as sha1_uint32 as sha1_uint32;
        a = a << 30 as libc::c_int | a >> 32 as libc::c_int - 30 as libc::c_int;
        tm = x[(22 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(22 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(22 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(22 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(22 as libc::c_int & 0xf as libc::c_int)
            as usize] = tm << 1 as libc::c_int
            | tm >> 32 as libc::c_int - 1 as libc::c_int;
        c = (c as libc::c_uint)
            .wrapping_add(
                (d << 5 as libc::c_int | d >> 32 as libc::c_int - 5 as libc::c_int)
                    .wrapping_add(e ^ a ^ b)
                    .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
                    .wrapping_add(x[(22 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as sha1_uint32 as sha1_uint32;
        e = e << 30 as libc::c_int | e >> 32 as libc::c_int - 30 as libc::c_int;
        tm = x[(23 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(23 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(23 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(23 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(23 as libc::c_int & 0xf as libc::c_int)
            as usize] = tm << 1 as libc::c_int
            | tm >> 32 as libc::c_int - 1 as libc::c_int;
        b = (b as libc::c_uint)
            .wrapping_add(
                (c << 5 as libc::c_int | c >> 32 as libc::c_int - 5 as libc::c_int)
                    .wrapping_add(d ^ e ^ a)
                    .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
                    .wrapping_add(x[(23 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as sha1_uint32 as sha1_uint32;
        d = d << 30 as libc::c_int | d >> 32 as libc::c_int - 30 as libc::c_int;
        tm = x[(24 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(24 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(24 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(24 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(24 as libc::c_int & 0xf as libc::c_int)
            as usize] = tm << 1 as libc::c_int
            | tm >> 32 as libc::c_int - 1 as libc::c_int;
        a = (a as libc::c_uint)
            .wrapping_add(
                (b << 5 as libc::c_int | b >> 32 as libc::c_int - 5 as libc::c_int)
                    .wrapping_add(c ^ d ^ e)
                    .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
                    .wrapping_add(x[(24 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as sha1_uint32 as sha1_uint32;
        c = c << 30 as libc::c_int | c >> 32 as libc::c_int - 30 as libc::c_int;
        tm = x[(25 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(25 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(25 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(25 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(25 as libc::c_int & 0xf as libc::c_int)
            as usize] = tm << 1 as libc::c_int
            | tm >> 32 as libc::c_int - 1 as libc::c_int;
        e = (e as libc::c_uint)
            .wrapping_add(
                (a << 5 as libc::c_int | a >> 32 as libc::c_int - 5 as libc::c_int)
                    .wrapping_add(b ^ c ^ d)
                    .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
                    .wrapping_add(x[(25 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as sha1_uint32 as sha1_uint32;
        b = b << 30 as libc::c_int | b >> 32 as libc::c_int - 30 as libc::c_int;
        tm = x[(26 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(26 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(26 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(26 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(26 as libc::c_int & 0xf as libc::c_int)
            as usize] = tm << 1 as libc::c_int
            | tm >> 32 as libc::c_int - 1 as libc::c_int;
        d = (d as libc::c_uint)
            .wrapping_add(
                (e << 5 as libc::c_int | e >> 32 as libc::c_int - 5 as libc::c_int)
                    .wrapping_add(a ^ b ^ c)
                    .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
                    .wrapping_add(x[(26 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as sha1_uint32 as sha1_uint32;
        a = a << 30 as libc::c_int | a >> 32 as libc::c_int - 30 as libc::c_int;
        tm = x[(27 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(27 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(27 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(27 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(27 as libc::c_int & 0xf as libc::c_int)
            as usize] = tm << 1 as libc::c_int
            | tm >> 32 as libc::c_int - 1 as libc::c_int;
        c = (c as libc::c_uint)
            .wrapping_add(
                (d << 5 as libc::c_int | d >> 32 as libc::c_int - 5 as libc::c_int)
                    .wrapping_add(e ^ a ^ b)
                    .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
                    .wrapping_add(x[(27 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as sha1_uint32 as sha1_uint32;
        e = e << 30 as libc::c_int | e >> 32 as libc::c_int - 30 as libc::c_int;
        tm = x[(28 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(28 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(28 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(28 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(28 as libc::c_int & 0xf as libc::c_int)
            as usize] = tm << 1 as libc::c_int
            | tm >> 32 as libc::c_int - 1 as libc::c_int;
        b = (b as libc::c_uint)
            .wrapping_add(
                (c << 5 as libc::c_int | c >> 32 as libc::c_int - 5 as libc::c_int)
                    .wrapping_add(d ^ e ^ a)
                    .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
                    .wrapping_add(x[(28 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as sha1_uint32 as sha1_uint32;
        d = d << 30 as libc::c_int | d >> 32 as libc::c_int - 30 as libc::c_int;
        tm = x[(29 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(29 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(29 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(29 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(29 as libc::c_int & 0xf as libc::c_int)
            as usize] = tm << 1 as libc::c_int
            | tm >> 32 as libc::c_int - 1 as libc::c_int;
        a = (a as libc::c_uint)
            .wrapping_add(
                (b << 5 as libc::c_int | b >> 32 as libc::c_int - 5 as libc::c_int)
                    .wrapping_add(c ^ d ^ e)
                    .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
                    .wrapping_add(x[(29 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as sha1_uint32 as sha1_uint32;
        c = c << 30 as libc::c_int | c >> 32 as libc::c_int - 30 as libc::c_int;
        tm = x[(30 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(30 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(30 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(30 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(30 as libc::c_int & 0xf as libc::c_int)
            as usize] = tm << 1 as libc::c_int
            | tm >> 32 as libc::c_int - 1 as libc::c_int;
        e = (e as libc::c_uint)
            .wrapping_add(
                (a << 5 as libc::c_int | a >> 32 as libc::c_int - 5 as libc::c_int)
                    .wrapping_add(b ^ c ^ d)
                    .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
                    .wrapping_add(x[(30 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as sha1_uint32 as sha1_uint32;
        b = b << 30 as libc::c_int | b >> 32 as libc::c_int - 30 as libc::c_int;
        tm = x[(31 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(31 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(31 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(31 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(31 as libc::c_int & 0xf as libc::c_int)
            as usize] = tm << 1 as libc::c_int
            | tm >> 32 as libc::c_int - 1 as libc::c_int;
        d = (d as libc::c_uint)
            .wrapping_add(
                (e << 5 as libc::c_int | e >> 32 as libc::c_int - 5 as libc::c_int)
                    .wrapping_add(a ^ b ^ c)
                    .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
                    .wrapping_add(x[(31 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as sha1_uint32 as sha1_uint32;
        a = a << 30 as libc::c_int | a >> 32 as libc::c_int - 30 as libc::c_int;
        tm = x[(32 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(32 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(32 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(32 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(32 as libc::c_int & 0xf as libc::c_int)
            as usize] = tm << 1 as libc::c_int
            | tm >> 32 as libc::c_int - 1 as libc::c_int;
        c = (c as libc::c_uint)
            .wrapping_add(
                (d << 5 as libc::c_int | d >> 32 as libc::c_int - 5 as libc::c_int)
                    .wrapping_add(e ^ a ^ b)
                    .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
                    .wrapping_add(x[(32 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as sha1_uint32 as sha1_uint32;
        e = e << 30 as libc::c_int | e >> 32 as libc::c_int - 30 as libc::c_int;
        tm = x[(33 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(33 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(33 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(33 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(33 as libc::c_int & 0xf as libc::c_int)
            as usize] = tm << 1 as libc::c_int
            | tm >> 32 as libc::c_int - 1 as libc::c_int;
        b = (b as libc::c_uint)
            .wrapping_add(
                (c << 5 as libc::c_int | c >> 32 as libc::c_int - 5 as libc::c_int)
                    .wrapping_add(d ^ e ^ a)
                    .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
                    .wrapping_add(x[(33 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as sha1_uint32 as sha1_uint32;
        d = d << 30 as libc::c_int | d >> 32 as libc::c_int - 30 as libc::c_int;
        tm = x[(34 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(34 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(34 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(34 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(34 as libc::c_int & 0xf as libc::c_int)
            as usize] = tm << 1 as libc::c_int
            | tm >> 32 as libc::c_int - 1 as libc::c_int;
        a = (a as libc::c_uint)
            .wrapping_add(
                (b << 5 as libc::c_int | b >> 32 as libc::c_int - 5 as libc::c_int)
                    .wrapping_add(c ^ d ^ e)
                    .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
                    .wrapping_add(x[(34 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as sha1_uint32 as sha1_uint32;
        c = c << 30 as libc::c_int | c >> 32 as libc::c_int - 30 as libc::c_int;
        tm = x[(35 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(35 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(35 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(35 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(35 as libc::c_int & 0xf as libc::c_int)
            as usize] = tm << 1 as libc::c_int
            | tm >> 32 as libc::c_int - 1 as libc::c_int;
        e = (e as libc::c_uint)
            .wrapping_add(
                (a << 5 as libc::c_int | a >> 32 as libc::c_int - 5 as libc::c_int)
                    .wrapping_add(b ^ c ^ d)
                    .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
                    .wrapping_add(x[(35 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as sha1_uint32 as sha1_uint32;
        b = b << 30 as libc::c_int | b >> 32 as libc::c_int - 30 as libc::c_int;
        tm = x[(36 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(36 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(36 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(36 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(36 as libc::c_int & 0xf as libc::c_int)
            as usize] = tm << 1 as libc::c_int
            | tm >> 32 as libc::c_int - 1 as libc::c_int;
        d = (d as libc::c_uint)
            .wrapping_add(
                (e << 5 as libc::c_int | e >> 32 as libc::c_int - 5 as libc::c_int)
                    .wrapping_add(a ^ b ^ c)
                    .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
                    .wrapping_add(x[(36 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as sha1_uint32 as sha1_uint32;
        a = a << 30 as libc::c_int | a >> 32 as libc::c_int - 30 as libc::c_int;
        tm = x[(37 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(37 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(37 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(37 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(37 as libc::c_int & 0xf as libc::c_int)
            as usize] = tm << 1 as libc::c_int
            | tm >> 32 as libc::c_int - 1 as libc::c_int;
        c = (c as libc::c_uint)
            .wrapping_add(
                (d << 5 as libc::c_int | d >> 32 as libc::c_int - 5 as libc::c_int)
                    .wrapping_add(e ^ a ^ b)
                    .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
                    .wrapping_add(x[(37 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as sha1_uint32 as sha1_uint32;
        e = e << 30 as libc::c_int | e >> 32 as libc::c_int - 30 as libc::c_int;
        tm = x[(38 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(38 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(38 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(38 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(38 as libc::c_int & 0xf as libc::c_int)
            as usize] = tm << 1 as libc::c_int
            | tm >> 32 as libc::c_int - 1 as libc::c_int;
        b = (b as libc::c_uint)
            .wrapping_add(
                (c << 5 as libc::c_int | c >> 32 as libc::c_int - 5 as libc::c_int)
                    .wrapping_add(d ^ e ^ a)
                    .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
                    .wrapping_add(x[(38 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as sha1_uint32 as sha1_uint32;
        d = d << 30 as libc::c_int | d >> 32 as libc::c_int - 30 as libc::c_int;
        tm = x[(39 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(39 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(39 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(39 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(39 as libc::c_int & 0xf as libc::c_int)
            as usize] = tm << 1 as libc::c_int
            | tm >> 32 as libc::c_int - 1 as libc::c_int;
        a = (a as libc::c_uint)
            .wrapping_add(
                (b << 5 as libc::c_int | b >> 32 as libc::c_int - 5 as libc::c_int)
                    .wrapping_add(c ^ d ^ e)
                    .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
                    .wrapping_add(x[(39 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as sha1_uint32 as sha1_uint32;
        c = c << 30 as libc::c_int | c >> 32 as libc::c_int - 30 as libc::c_int;
        tm = x[(40 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(40 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(40 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(40 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(40 as libc::c_int & 0xf as libc::c_int)
            as usize] = tm << 1 as libc::c_int
            | tm >> 32 as libc::c_int - 1 as libc::c_int;
        e = (e as libc::c_uint)
            .wrapping_add(
                (a << 5 as libc::c_int | a >> 32 as libc::c_int - 5 as libc::c_int)
                    .wrapping_add(b & c | d & (b | c))
                    .wrapping_add(0x8f1bbcdc as libc::c_uint)
                    .wrapping_add(x[(40 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as sha1_uint32 as sha1_uint32;
        b = b << 30 as libc::c_int | b >> 32 as libc::c_int - 30 as libc::c_int;
        tm = x[(41 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(41 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(41 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(41 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(41 as libc::c_int & 0xf as libc::c_int)
            as usize] = tm << 1 as libc::c_int
            | tm >> 32 as libc::c_int - 1 as libc::c_int;
        d = (d as libc::c_uint)
            .wrapping_add(
                (e << 5 as libc::c_int | e >> 32 as libc::c_int - 5 as libc::c_int)
                    .wrapping_add(a & b | c & (a | b))
                    .wrapping_add(0x8f1bbcdc as libc::c_uint)
                    .wrapping_add(x[(41 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as sha1_uint32 as sha1_uint32;
        a = a << 30 as libc::c_int | a >> 32 as libc::c_int - 30 as libc::c_int;
        tm = x[(42 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(42 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(42 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(42 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(42 as libc::c_int & 0xf as libc::c_int)
            as usize] = tm << 1 as libc::c_int
            | tm >> 32 as libc::c_int - 1 as libc::c_int;
        c = (c as libc::c_uint)
            .wrapping_add(
                (d << 5 as libc::c_int | d >> 32 as libc::c_int - 5 as libc::c_int)
                    .wrapping_add(e & a | b & (e | a))
                    .wrapping_add(0x8f1bbcdc as libc::c_uint)
                    .wrapping_add(x[(42 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as sha1_uint32 as sha1_uint32;
        e = e << 30 as libc::c_int | e >> 32 as libc::c_int - 30 as libc::c_int;
        tm = x[(43 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(43 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(43 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(43 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(43 as libc::c_int & 0xf as libc::c_int)
            as usize] = tm << 1 as libc::c_int
            | tm >> 32 as libc::c_int - 1 as libc::c_int;
        b = (b as libc::c_uint)
            .wrapping_add(
                (c << 5 as libc::c_int | c >> 32 as libc::c_int - 5 as libc::c_int)
                    .wrapping_add(d & e | a & (d | e))
                    .wrapping_add(0x8f1bbcdc as libc::c_uint)
                    .wrapping_add(x[(43 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as sha1_uint32 as sha1_uint32;
        d = d << 30 as libc::c_int | d >> 32 as libc::c_int - 30 as libc::c_int;
        tm = x[(44 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(44 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(44 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(44 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(44 as libc::c_int & 0xf as libc::c_int)
            as usize] = tm << 1 as libc::c_int
            | tm >> 32 as libc::c_int - 1 as libc::c_int;
        a = (a as libc::c_uint)
            .wrapping_add(
                (b << 5 as libc::c_int | b >> 32 as libc::c_int - 5 as libc::c_int)
                    .wrapping_add(c & d | e & (c | d))
                    .wrapping_add(0x8f1bbcdc as libc::c_uint)
                    .wrapping_add(x[(44 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as sha1_uint32 as sha1_uint32;
        c = c << 30 as libc::c_int | c >> 32 as libc::c_int - 30 as libc::c_int;
        tm = x[(45 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(45 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(45 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(45 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(45 as libc::c_int & 0xf as libc::c_int)
            as usize] = tm << 1 as libc::c_int
            | tm >> 32 as libc::c_int - 1 as libc::c_int;
        e = (e as libc::c_uint)
            .wrapping_add(
                (a << 5 as libc::c_int | a >> 32 as libc::c_int - 5 as libc::c_int)
                    .wrapping_add(b & c | d & (b | c))
                    .wrapping_add(0x8f1bbcdc as libc::c_uint)
                    .wrapping_add(x[(45 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as sha1_uint32 as sha1_uint32;
        b = b << 30 as libc::c_int | b >> 32 as libc::c_int - 30 as libc::c_int;
        tm = x[(46 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(46 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(46 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(46 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(46 as libc::c_int & 0xf as libc::c_int)
            as usize] = tm << 1 as libc::c_int
            | tm >> 32 as libc::c_int - 1 as libc::c_int;
        d = (d as libc::c_uint)
            .wrapping_add(
                (e << 5 as libc::c_int | e >> 32 as libc::c_int - 5 as libc::c_int)
                    .wrapping_add(a & b | c & (a | b))
                    .wrapping_add(0x8f1bbcdc as libc::c_uint)
                    .wrapping_add(x[(46 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as sha1_uint32 as sha1_uint32;
        a = a << 30 as libc::c_int | a >> 32 as libc::c_int - 30 as libc::c_int;
        tm = x[(47 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(47 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(47 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(47 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(47 as libc::c_int & 0xf as libc::c_int)
            as usize] = tm << 1 as libc::c_int
            | tm >> 32 as libc::c_int - 1 as libc::c_int;
        c = (c as libc::c_uint)
            .wrapping_add(
                (d << 5 as libc::c_int | d >> 32 as libc::c_int - 5 as libc::c_int)
                    .wrapping_add(e & a | b & (e | a))
                    .wrapping_add(0x8f1bbcdc as libc::c_uint)
                    .wrapping_add(x[(47 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as sha1_uint32 as sha1_uint32;
        e = e << 30 as libc::c_int | e >> 32 as libc::c_int - 30 as libc::c_int;
        tm = x[(48 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(48 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(48 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(48 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(48 as libc::c_int & 0xf as libc::c_int)
            as usize] = tm << 1 as libc::c_int
            | tm >> 32 as libc::c_int - 1 as libc::c_int;
        b = (b as libc::c_uint)
            .wrapping_add(
                (c << 5 as libc::c_int | c >> 32 as libc::c_int - 5 as libc::c_int)
                    .wrapping_add(d & e | a & (d | e))
                    .wrapping_add(0x8f1bbcdc as libc::c_uint)
                    .wrapping_add(x[(48 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as sha1_uint32 as sha1_uint32;
        d = d << 30 as libc::c_int | d >> 32 as libc::c_int - 30 as libc::c_int;
        tm = x[(49 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(49 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(49 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(49 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(49 as libc::c_int & 0xf as libc::c_int)
            as usize] = tm << 1 as libc::c_int
            | tm >> 32 as libc::c_int - 1 as libc::c_int;
        a = (a as libc::c_uint)
            .wrapping_add(
                (b << 5 as libc::c_int | b >> 32 as libc::c_int - 5 as libc::c_int)
                    .wrapping_add(c & d | e & (c | d))
                    .wrapping_add(0x8f1bbcdc as libc::c_uint)
                    .wrapping_add(x[(49 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as sha1_uint32 as sha1_uint32;
        c = c << 30 as libc::c_int | c >> 32 as libc::c_int - 30 as libc::c_int;
        tm = x[(50 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(50 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(50 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(50 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(50 as libc::c_int & 0xf as libc::c_int)
            as usize] = tm << 1 as libc::c_int
            | tm >> 32 as libc::c_int - 1 as libc::c_int;
        e = (e as libc::c_uint)
            .wrapping_add(
                (a << 5 as libc::c_int | a >> 32 as libc::c_int - 5 as libc::c_int)
                    .wrapping_add(b & c | d & (b | c))
                    .wrapping_add(0x8f1bbcdc as libc::c_uint)
                    .wrapping_add(x[(50 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as sha1_uint32 as sha1_uint32;
        b = b << 30 as libc::c_int | b >> 32 as libc::c_int - 30 as libc::c_int;
        tm = x[(51 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(51 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(51 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(51 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(51 as libc::c_int & 0xf as libc::c_int)
            as usize] = tm << 1 as libc::c_int
            | tm >> 32 as libc::c_int - 1 as libc::c_int;
        d = (d as libc::c_uint)
            .wrapping_add(
                (e << 5 as libc::c_int | e >> 32 as libc::c_int - 5 as libc::c_int)
                    .wrapping_add(a & b | c & (a | b))
                    .wrapping_add(0x8f1bbcdc as libc::c_uint)
                    .wrapping_add(x[(51 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as sha1_uint32 as sha1_uint32;
        a = a << 30 as libc::c_int | a >> 32 as libc::c_int - 30 as libc::c_int;
        tm = x[(52 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(52 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(52 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(52 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(52 as libc::c_int & 0xf as libc::c_int)
            as usize] = tm << 1 as libc::c_int
            | tm >> 32 as libc::c_int - 1 as libc::c_int;
        c = (c as libc::c_uint)
            .wrapping_add(
                (d << 5 as libc::c_int | d >> 32 as libc::c_int - 5 as libc::c_int)
                    .wrapping_add(e & a | b & (e | a))
                    .wrapping_add(0x8f1bbcdc as libc::c_uint)
                    .wrapping_add(x[(52 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as sha1_uint32 as sha1_uint32;
        e = e << 30 as libc::c_int | e >> 32 as libc::c_int - 30 as libc::c_int;
        tm = x[(53 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(53 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(53 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(53 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(53 as libc::c_int & 0xf as libc::c_int)
            as usize] = tm << 1 as libc::c_int
            | tm >> 32 as libc::c_int - 1 as libc::c_int;
        b = (b as libc::c_uint)
            .wrapping_add(
                (c << 5 as libc::c_int | c >> 32 as libc::c_int - 5 as libc::c_int)
                    .wrapping_add(d & e | a & (d | e))
                    .wrapping_add(0x8f1bbcdc as libc::c_uint)
                    .wrapping_add(x[(53 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as sha1_uint32 as sha1_uint32;
        d = d << 30 as libc::c_int | d >> 32 as libc::c_int - 30 as libc::c_int;
        tm = x[(54 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(54 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(54 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(54 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(54 as libc::c_int & 0xf as libc::c_int)
            as usize] = tm << 1 as libc::c_int
            | tm >> 32 as libc::c_int - 1 as libc::c_int;
        a = (a as libc::c_uint)
            .wrapping_add(
                (b << 5 as libc::c_int | b >> 32 as libc::c_int - 5 as libc::c_int)
                    .wrapping_add(c & d | e & (c | d))
                    .wrapping_add(0x8f1bbcdc as libc::c_uint)
                    .wrapping_add(x[(54 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as sha1_uint32 as sha1_uint32;
        c = c << 30 as libc::c_int | c >> 32 as libc::c_int - 30 as libc::c_int;
        tm = x[(55 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(55 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(55 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(55 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(55 as libc::c_int & 0xf as libc::c_int)
            as usize] = tm << 1 as libc::c_int
            | tm >> 32 as libc::c_int - 1 as libc::c_int;
        e = (e as libc::c_uint)
            .wrapping_add(
                (a << 5 as libc::c_int | a >> 32 as libc::c_int - 5 as libc::c_int)
                    .wrapping_add(b & c | d & (b | c))
                    .wrapping_add(0x8f1bbcdc as libc::c_uint)
                    .wrapping_add(x[(55 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as sha1_uint32 as sha1_uint32;
        b = b << 30 as libc::c_int | b >> 32 as libc::c_int - 30 as libc::c_int;
        tm = x[(56 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(56 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(56 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(56 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(56 as libc::c_int & 0xf as libc::c_int)
            as usize] = tm << 1 as libc::c_int
            | tm >> 32 as libc::c_int - 1 as libc::c_int;
        d = (d as libc::c_uint)
            .wrapping_add(
                (e << 5 as libc::c_int | e >> 32 as libc::c_int - 5 as libc::c_int)
                    .wrapping_add(a & b | c & (a | b))
                    .wrapping_add(0x8f1bbcdc as libc::c_uint)
                    .wrapping_add(x[(56 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as sha1_uint32 as sha1_uint32;
        a = a << 30 as libc::c_int | a >> 32 as libc::c_int - 30 as libc::c_int;
        tm = x[(57 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(57 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(57 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(57 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(57 as libc::c_int & 0xf as libc::c_int)
            as usize] = tm << 1 as libc::c_int
            | tm >> 32 as libc::c_int - 1 as libc::c_int;
        c = (c as libc::c_uint)
            .wrapping_add(
                (d << 5 as libc::c_int | d >> 32 as libc::c_int - 5 as libc::c_int)
                    .wrapping_add(e & a | b & (e | a))
                    .wrapping_add(0x8f1bbcdc as libc::c_uint)
                    .wrapping_add(x[(57 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as sha1_uint32 as sha1_uint32;
        e = e << 30 as libc::c_int | e >> 32 as libc::c_int - 30 as libc::c_int;
        tm = x[(58 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(58 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(58 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(58 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(58 as libc::c_int & 0xf as libc::c_int)
            as usize] = tm << 1 as libc::c_int
            | tm >> 32 as libc::c_int - 1 as libc::c_int;
        b = (b as libc::c_uint)
            .wrapping_add(
                (c << 5 as libc::c_int | c >> 32 as libc::c_int - 5 as libc::c_int)
                    .wrapping_add(d & e | a & (d | e))
                    .wrapping_add(0x8f1bbcdc as libc::c_uint)
                    .wrapping_add(x[(58 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as sha1_uint32 as sha1_uint32;
        d = d << 30 as libc::c_int | d >> 32 as libc::c_int - 30 as libc::c_int;
        tm = x[(59 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(59 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(59 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(59 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(59 as libc::c_int & 0xf as libc::c_int)
            as usize] = tm << 1 as libc::c_int
            | tm >> 32 as libc::c_int - 1 as libc::c_int;
        a = (a as libc::c_uint)
            .wrapping_add(
                (b << 5 as libc::c_int | b >> 32 as libc::c_int - 5 as libc::c_int)
                    .wrapping_add(c & d | e & (c | d))
                    .wrapping_add(0x8f1bbcdc as libc::c_uint)
                    .wrapping_add(x[(59 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as sha1_uint32 as sha1_uint32;
        c = c << 30 as libc::c_int | c >> 32 as libc::c_int - 30 as libc::c_int;
        tm = x[(60 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(60 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(60 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(60 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(60 as libc::c_int & 0xf as libc::c_int)
            as usize] = tm << 1 as libc::c_int
            | tm >> 32 as libc::c_int - 1 as libc::c_int;
        e = (e as libc::c_uint)
            .wrapping_add(
                (a << 5 as libc::c_int | a >> 32 as libc::c_int - 5 as libc::c_int)
                    .wrapping_add(b ^ c ^ d)
                    .wrapping_add(0xca62c1d6 as libc::c_uint)
                    .wrapping_add(x[(60 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as sha1_uint32 as sha1_uint32;
        b = b << 30 as libc::c_int | b >> 32 as libc::c_int - 30 as libc::c_int;
        tm = x[(61 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(61 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(61 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(61 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(61 as libc::c_int & 0xf as libc::c_int)
            as usize] = tm << 1 as libc::c_int
            | tm >> 32 as libc::c_int - 1 as libc::c_int;
        d = (d as libc::c_uint)
            .wrapping_add(
                (e << 5 as libc::c_int | e >> 32 as libc::c_int - 5 as libc::c_int)
                    .wrapping_add(a ^ b ^ c)
                    .wrapping_add(0xca62c1d6 as libc::c_uint)
                    .wrapping_add(x[(61 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as sha1_uint32 as sha1_uint32;
        a = a << 30 as libc::c_int | a >> 32 as libc::c_int - 30 as libc::c_int;
        tm = x[(62 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(62 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(62 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(62 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(62 as libc::c_int & 0xf as libc::c_int)
            as usize] = tm << 1 as libc::c_int
            | tm >> 32 as libc::c_int - 1 as libc::c_int;
        c = (c as libc::c_uint)
            .wrapping_add(
                (d << 5 as libc::c_int | d >> 32 as libc::c_int - 5 as libc::c_int)
                    .wrapping_add(e ^ a ^ b)
                    .wrapping_add(0xca62c1d6 as libc::c_uint)
                    .wrapping_add(x[(62 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as sha1_uint32 as sha1_uint32;
        e = e << 30 as libc::c_int | e >> 32 as libc::c_int - 30 as libc::c_int;
        tm = x[(63 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(63 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(63 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(63 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(63 as libc::c_int & 0xf as libc::c_int)
            as usize] = tm << 1 as libc::c_int
            | tm >> 32 as libc::c_int - 1 as libc::c_int;
        b = (b as libc::c_uint)
            .wrapping_add(
                (c << 5 as libc::c_int | c >> 32 as libc::c_int - 5 as libc::c_int)
                    .wrapping_add(d ^ e ^ a)
                    .wrapping_add(0xca62c1d6 as libc::c_uint)
                    .wrapping_add(x[(63 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as sha1_uint32 as sha1_uint32;
        d = d << 30 as libc::c_int | d >> 32 as libc::c_int - 30 as libc::c_int;
        tm = x[(64 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(64 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(64 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(64 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(64 as libc::c_int & 0xf as libc::c_int)
            as usize] = tm << 1 as libc::c_int
            | tm >> 32 as libc::c_int - 1 as libc::c_int;
        a = (a as libc::c_uint)
            .wrapping_add(
                (b << 5 as libc::c_int | b >> 32 as libc::c_int - 5 as libc::c_int)
                    .wrapping_add(c ^ d ^ e)
                    .wrapping_add(0xca62c1d6 as libc::c_uint)
                    .wrapping_add(x[(64 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as sha1_uint32 as sha1_uint32;
        c = c << 30 as libc::c_int | c >> 32 as libc::c_int - 30 as libc::c_int;
        tm = x[(65 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(65 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(65 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(65 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(65 as libc::c_int & 0xf as libc::c_int)
            as usize] = tm << 1 as libc::c_int
            | tm >> 32 as libc::c_int - 1 as libc::c_int;
        e = (e as libc::c_uint)
            .wrapping_add(
                (a << 5 as libc::c_int | a >> 32 as libc::c_int - 5 as libc::c_int)
                    .wrapping_add(b ^ c ^ d)
                    .wrapping_add(0xca62c1d6 as libc::c_uint)
                    .wrapping_add(x[(65 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as sha1_uint32 as sha1_uint32;
        b = b << 30 as libc::c_int | b >> 32 as libc::c_int - 30 as libc::c_int;
        tm = x[(66 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(66 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(66 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(66 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(66 as libc::c_int & 0xf as libc::c_int)
            as usize] = tm << 1 as libc::c_int
            | tm >> 32 as libc::c_int - 1 as libc::c_int;
        d = (d as libc::c_uint)
            .wrapping_add(
                (e << 5 as libc::c_int | e >> 32 as libc::c_int - 5 as libc::c_int)
                    .wrapping_add(a ^ b ^ c)
                    .wrapping_add(0xca62c1d6 as libc::c_uint)
                    .wrapping_add(x[(66 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as sha1_uint32 as sha1_uint32;
        a = a << 30 as libc::c_int | a >> 32 as libc::c_int - 30 as libc::c_int;
        tm = x[(67 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(67 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(67 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(67 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(67 as libc::c_int & 0xf as libc::c_int)
            as usize] = tm << 1 as libc::c_int
            | tm >> 32 as libc::c_int - 1 as libc::c_int;
        c = (c as libc::c_uint)
            .wrapping_add(
                (d << 5 as libc::c_int | d >> 32 as libc::c_int - 5 as libc::c_int)
                    .wrapping_add(e ^ a ^ b)
                    .wrapping_add(0xca62c1d6 as libc::c_uint)
                    .wrapping_add(x[(67 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as sha1_uint32 as sha1_uint32;
        e = e << 30 as libc::c_int | e >> 32 as libc::c_int - 30 as libc::c_int;
        tm = x[(68 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(68 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(68 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(68 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(68 as libc::c_int & 0xf as libc::c_int)
            as usize] = tm << 1 as libc::c_int
            | tm >> 32 as libc::c_int - 1 as libc::c_int;
        b = (b as libc::c_uint)
            .wrapping_add(
                (c << 5 as libc::c_int | c >> 32 as libc::c_int - 5 as libc::c_int)
                    .wrapping_add(d ^ e ^ a)
                    .wrapping_add(0xca62c1d6 as libc::c_uint)
                    .wrapping_add(x[(68 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as sha1_uint32 as sha1_uint32;
        d = d << 30 as libc::c_int | d >> 32 as libc::c_int - 30 as libc::c_int;
        tm = x[(69 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(69 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(69 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(69 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(69 as libc::c_int & 0xf as libc::c_int)
            as usize] = tm << 1 as libc::c_int
            | tm >> 32 as libc::c_int - 1 as libc::c_int;
        a = (a as libc::c_uint)
            .wrapping_add(
                (b << 5 as libc::c_int | b >> 32 as libc::c_int - 5 as libc::c_int)
                    .wrapping_add(c ^ d ^ e)
                    .wrapping_add(0xca62c1d6 as libc::c_uint)
                    .wrapping_add(x[(69 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as sha1_uint32 as sha1_uint32;
        c = c << 30 as libc::c_int | c >> 32 as libc::c_int - 30 as libc::c_int;
        tm = x[(70 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(70 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(70 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(70 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(70 as libc::c_int & 0xf as libc::c_int)
            as usize] = tm << 1 as libc::c_int
            | tm >> 32 as libc::c_int - 1 as libc::c_int;
        e = (e as libc::c_uint)
            .wrapping_add(
                (a << 5 as libc::c_int | a >> 32 as libc::c_int - 5 as libc::c_int)
                    .wrapping_add(b ^ c ^ d)
                    .wrapping_add(0xca62c1d6 as libc::c_uint)
                    .wrapping_add(x[(70 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as sha1_uint32 as sha1_uint32;
        b = b << 30 as libc::c_int | b >> 32 as libc::c_int - 30 as libc::c_int;
        tm = x[(71 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(71 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(71 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(71 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(71 as libc::c_int & 0xf as libc::c_int)
            as usize] = tm << 1 as libc::c_int
            | tm >> 32 as libc::c_int - 1 as libc::c_int;
        d = (d as libc::c_uint)
            .wrapping_add(
                (e << 5 as libc::c_int | e >> 32 as libc::c_int - 5 as libc::c_int)
                    .wrapping_add(a ^ b ^ c)
                    .wrapping_add(0xca62c1d6 as libc::c_uint)
                    .wrapping_add(x[(71 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as sha1_uint32 as sha1_uint32;
        a = a << 30 as libc::c_int | a >> 32 as libc::c_int - 30 as libc::c_int;
        tm = x[(72 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(72 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(72 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(72 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(72 as libc::c_int & 0xf as libc::c_int)
            as usize] = tm << 1 as libc::c_int
            | tm >> 32 as libc::c_int - 1 as libc::c_int;
        c = (c as libc::c_uint)
            .wrapping_add(
                (d << 5 as libc::c_int | d >> 32 as libc::c_int - 5 as libc::c_int)
                    .wrapping_add(e ^ a ^ b)
                    .wrapping_add(0xca62c1d6 as libc::c_uint)
                    .wrapping_add(x[(72 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as sha1_uint32 as sha1_uint32;
        e = e << 30 as libc::c_int | e >> 32 as libc::c_int - 30 as libc::c_int;
        tm = x[(73 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(73 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(73 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(73 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(73 as libc::c_int & 0xf as libc::c_int)
            as usize] = tm << 1 as libc::c_int
            | tm >> 32 as libc::c_int - 1 as libc::c_int;
        b = (b as libc::c_uint)
            .wrapping_add(
                (c << 5 as libc::c_int | c >> 32 as libc::c_int - 5 as libc::c_int)
                    .wrapping_add(d ^ e ^ a)
                    .wrapping_add(0xca62c1d6 as libc::c_uint)
                    .wrapping_add(x[(73 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as sha1_uint32 as sha1_uint32;
        d = d << 30 as libc::c_int | d >> 32 as libc::c_int - 30 as libc::c_int;
        tm = x[(74 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(74 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(74 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(74 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(74 as libc::c_int & 0xf as libc::c_int)
            as usize] = tm << 1 as libc::c_int
            | tm >> 32 as libc::c_int - 1 as libc::c_int;
        a = (a as libc::c_uint)
            .wrapping_add(
                (b << 5 as libc::c_int | b >> 32 as libc::c_int - 5 as libc::c_int)
                    .wrapping_add(c ^ d ^ e)
                    .wrapping_add(0xca62c1d6 as libc::c_uint)
                    .wrapping_add(x[(74 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as sha1_uint32 as sha1_uint32;
        c = c << 30 as libc::c_int | c >> 32 as libc::c_int - 30 as libc::c_int;
        tm = x[(75 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(75 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(75 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(75 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(75 as libc::c_int & 0xf as libc::c_int)
            as usize] = tm << 1 as libc::c_int
            | tm >> 32 as libc::c_int - 1 as libc::c_int;
        e = (e as libc::c_uint)
            .wrapping_add(
                (a << 5 as libc::c_int | a >> 32 as libc::c_int - 5 as libc::c_int)
                    .wrapping_add(b ^ c ^ d)
                    .wrapping_add(0xca62c1d6 as libc::c_uint)
                    .wrapping_add(x[(75 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as sha1_uint32 as sha1_uint32;
        b = b << 30 as libc::c_int | b >> 32 as libc::c_int - 30 as libc::c_int;
        tm = x[(76 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(76 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(76 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(76 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(76 as libc::c_int & 0xf as libc::c_int)
            as usize] = tm << 1 as libc::c_int
            | tm >> 32 as libc::c_int - 1 as libc::c_int;
        d = (d as libc::c_uint)
            .wrapping_add(
                (e << 5 as libc::c_int | e >> 32 as libc::c_int - 5 as libc::c_int)
                    .wrapping_add(a ^ b ^ c)
                    .wrapping_add(0xca62c1d6 as libc::c_uint)
                    .wrapping_add(x[(76 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as sha1_uint32 as sha1_uint32;
        a = a << 30 as libc::c_int | a >> 32 as libc::c_int - 30 as libc::c_int;
        tm = x[(77 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(77 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(77 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(77 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(77 as libc::c_int & 0xf as libc::c_int)
            as usize] = tm << 1 as libc::c_int
            | tm >> 32 as libc::c_int - 1 as libc::c_int;
        c = (c as libc::c_uint)
            .wrapping_add(
                (d << 5 as libc::c_int | d >> 32 as libc::c_int - 5 as libc::c_int)
                    .wrapping_add(e ^ a ^ b)
                    .wrapping_add(0xca62c1d6 as libc::c_uint)
                    .wrapping_add(x[(77 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as sha1_uint32 as sha1_uint32;
        e = e << 30 as libc::c_int | e >> 32 as libc::c_int - 30 as libc::c_int;
        tm = x[(78 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(78 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(78 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(78 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(78 as libc::c_int & 0xf as libc::c_int)
            as usize] = tm << 1 as libc::c_int
            | tm >> 32 as libc::c_int - 1 as libc::c_int;
        b = (b as libc::c_uint)
            .wrapping_add(
                (c << 5 as libc::c_int | c >> 32 as libc::c_int - 5 as libc::c_int)
                    .wrapping_add(d ^ e ^ a)
                    .wrapping_add(0xca62c1d6 as libc::c_uint)
                    .wrapping_add(x[(78 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as sha1_uint32 as sha1_uint32;
        d = d << 30 as libc::c_int | d >> 32 as libc::c_int - 30 as libc::c_int;
        tm = x[(79 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(79 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(79 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(79 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(79 as libc::c_int & 0xf as libc::c_int)
            as usize] = tm << 1 as libc::c_int
            | tm >> 32 as libc::c_int - 1 as libc::c_int;
        a = (a as libc::c_uint)
            .wrapping_add(
                (b << 5 as libc::c_int | b >> 32 as libc::c_int - 5 as libc::c_int)
                    .wrapping_add(c ^ d ^ e)
                    .wrapping_add(0xca62c1d6 as libc::c_uint)
                    .wrapping_add(x[(79 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as sha1_uint32 as sha1_uint32;
        c = c << 30 as libc::c_int | c >> 32 as libc::c_int - 30 as libc::c_int;
        (*ctx)
            .A = ((*ctx).A as libc::c_uint).wrapping_add(a) as sha1_uint32
            as sha1_uint32;
        a = (*ctx).A;
        (*ctx)
            .B = ((*ctx).B as libc::c_uint).wrapping_add(b) as sha1_uint32
            as sha1_uint32;
        b = (*ctx).B;
        (*ctx)
            .C = ((*ctx).C as libc::c_uint).wrapping_add(c) as sha1_uint32
            as sha1_uint32;
        c = (*ctx).C;
        (*ctx)
            .D = ((*ctx).D as libc::c_uint).wrapping_add(d) as sha1_uint32
            as sha1_uint32;
        d = (*ctx).D;
        (*ctx)
            .E = ((*ctx).E as libc::c_uint).wrapping_add(e) as sha1_uint32
            as sha1_uint32;
        e = (*ctx).E;
    }
}
