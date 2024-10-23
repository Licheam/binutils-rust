use ::libc;
extern "C" {
    fn exit(_: libc::c_int) -> !;
}
#[no_mangle]
pub static mut _xexit_cleanup: Option::<unsafe extern "C" fn() -> ()> = None;
#[no_mangle]
pub unsafe extern "C" fn xexit(mut code: libc::c_int) -> ! {
    if _xexit_cleanup.is_some() {
        (Some(_xexit_cleanup.expect("non-null function pointer")))
            .expect("non-null function pointer")();
    }
    exit(code);
}
