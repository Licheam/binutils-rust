use ::libc;
extern "C" {
    fn getrlimit(__resource: __rlimit_resource_t, __rlimits: *mut rlimit) -> libc::c_int;
    fn setrlimit(
        __resource: __rlimit_resource_t,
        __rlimits: *const rlimit,
    ) -> libc::c_int;
}
pub type __rlim_t = libc::c_ulong;
pub type __rlimit_resource = libc::c_uint;
pub const __RLIM_NLIMITS: __rlimit_resource = 16;
pub const __RLIMIT_NLIMITS: __rlimit_resource = 16;
pub const __RLIMIT_RTTIME: __rlimit_resource = 15;
pub const __RLIMIT_RTPRIO: __rlimit_resource = 14;
pub const __RLIMIT_NICE: __rlimit_resource = 13;
pub const __RLIMIT_MSGQUEUE: __rlimit_resource = 12;
pub const __RLIMIT_SIGPENDING: __rlimit_resource = 11;
pub const __RLIMIT_LOCKS: __rlimit_resource = 10;
pub const __RLIMIT_MEMLOCK: __rlimit_resource = 8;
pub const __RLIMIT_NPROC: __rlimit_resource = 6;
pub const RLIMIT_AS: __rlimit_resource = 9;
pub const __RLIMIT_OFILE: __rlimit_resource = 7;
pub const RLIMIT_NOFILE: __rlimit_resource = 7;
pub const __RLIMIT_RSS: __rlimit_resource = 5;
pub const RLIMIT_CORE: __rlimit_resource = 4;
pub const RLIMIT_STACK: __rlimit_resource = 3;
pub const RLIMIT_DATA: __rlimit_resource = 2;
pub const RLIMIT_FSIZE: __rlimit_resource = 1;
pub const RLIMIT_CPU: __rlimit_resource = 0;
pub type rlim_t = __rlim_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rlimit {
    pub rlim_cur: rlim_t,
    pub rlim_max: rlim_t,
}
pub type __rlimit_resource_t = __rlimit_resource;
#[no_mangle]
pub unsafe extern "C" fn stack_limit_increase(mut pref: libc::c_ulong) {
    let mut rlim: rlimit = rlimit { rlim_cur: 0, rlim_max: 0 };
    if getrlimit(RLIMIT_STACK, &mut rlim) == 0 as libc::c_int
        && rlim.rlim_cur != -(1 as libc::c_int) as __rlim_t && rlim.rlim_cur < pref
        && (rlim.rlim_max == -(1 as libc::c_int) as __rlim_t
            || rlim.rlim_cur < rlim.rlim_max)
    {
        rlim.rlim_cur = pref;
        if rlim.rlim_max != -(1 as libc::c_int) as __rlim_t
            && rlim.rlim_cur > rlim.rlim_max
        {
            rlim.rlim_cur = rlim.rlim_max;
        }
        setrlimit(RLIMIT_STACK, &mut rlim);
    }
}
