use ::libc;
#[no_mangle]
pub unsafe extern "C" fn bfd_get_stab_name(
    mut code: libc::c_int,
) -> *const libc::c_char {
    match code {
        32 => return b"GSYM\0" as *const u8 as *const libc::c_char,
        34 => return b"FNAME\0" as *const u8 as *const libc::c_char,
        36 => return b"FUN\0" as *const u8 as *const libc::c_char,
        38 => return b"STSYM\0" as *const u8 as *const libc::c_char,
        40 => return b"LCSYM\0" as *const u8 as *const libc::c_char,
        42 => return b"MAIN\0" as *const u8 as *const libc::c_char,
        44 => return b"ROSYM\0" as *const u8 as *const libc::c_char,
        46 => return b"BNSYM\0" as *const u8 as *const libc::c_char,
        48 => return b"PC\0" as *const u8 as *const libc::c_char,
        50 => return b"NSYMS\0" as *const u8 as *const libc::c_char,
        52 => return b"NOMAP\0" as *const u8 as *const libc::c_char,
        56 => return b"OBJ\0" as *const u8 as *const libc::c_char,
        60 => return b"OPT\0" as *const u8 as *const libc::c_char,
        64 => return b"RSYM\0" as *const u8 as *const libc::c_char,
        66 => return b"M2C\0" as *const u8 as *const libc::c_char,
        68 => return b"SLINE\0" as *const u8 as *const libc::c_char,
        70 => return b"DSLINE\0" as *const u8 as *const libc::c_char,
        72 => return b"BSLINE\0" as *const u8 as *const libc::c_char,
        74 => return b"DEFD\0" as *const u8 as *const libc::c_char,
        76 => return b"FLINE\0" as *const u8 as *const libc::c_char,
        78 => return b"ENSYM\0" as *const u8 as *const libc::c_char,
        80 => return b"EHDECL\0" as *const u8 as *const libc::c_char,
        84 => return b"CATCH\0" as *const u8 as *const libc::c_char,
        96 => return b"SSYM\0" as *const u8 as *const libc::c_char,
        98 => return b"ENDM\0" as *const u8 as *const libc::c_char,
        100 => return b"SO\0" as *const u8 as *const libc::c_char,
        102 => return b"OSO\0" as *const u8 as *const libc::c_char,
        108 => return b"ALIAS\0" as *const u8 as *const libc::c_char,
        128 => return b"LSYM\0" as *const u8 as *const libc::c_char,
        130 => return b"BINCL\0" as *const u8 as *const libc::c_char,
        132 => return b"SOL\0" as *const u8 as *const libc::c_char,
        160 => return b"PSYM\0" as *const u8 as *const libc::c_char,
        162 => return b"EINCL\0" as *const u8 as *const libc::c_char,
        164 => return b"ENTRY\0" as *const u8 as *const libc::c_char,
        192 => return b"LBRAC\0" as *const u8 as *const libc::c_char,
        194 => return b"EXCL\0" as *const u8 as *const libc::c_char,
        196 => return b"SCOPE\0" as *const u8 as *const libc::c_char,
        208 => return b"PATCH\0" as *const u8 as *const libc::c_char,
        224 => return b"RBRAC\0" as *const u8 as *const libc::c_char,
        226 => return b"BCOMM\0" as *const u8 as *const libc::c_char,
        228 => return b"ECOMM\0" as *const u8 as *const libc::c_char,
        232 => return b"ECOML\0" as *const u8 as *const libc::c_char,
        234 => return b"WITH\0" as *const u8 as *const libc::c_char,
        240 => return b"NBTEXT\0" as *const u8 as *const libc::c_char,
        242 => return b"NBDATA\0" as *const u8 as *const libc::c_char,
        244 => return b"NBBSS\0" as *const u8 as *const libc::c_char,
        246 => return b"NBSTS\0" as *const u8 as *const libc::c_char,
        248 => return b"NBLCS\0" as *const u8 as *const libc::c_char,
        254 => return b"LENG\0" as *const u8 as *const libc::c_char,
        20 => return b"SETA\0" as *const u8 as *const libc::c_char,
        22 => return b"SETT\0" as *const u8 as *const libc::c_char,
        24 => return b"SETD\0" as *const u8 as *const libc::c_char,
        26 => return b"SETB\0" as *const u8 as *const libc::c_char,
        28 => return b"SETV\0" as *const u8 as *const libc::c_char,
        10 => return b"INDR\0" as *const u8 as *const libc::c_char,
        30 => return b"WARNING\0" as *const u8 as *const libc::c_char,
        _ => {}
    }
    return 0 as *const libc::c_char;
}
