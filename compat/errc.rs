use ::libc;
extern "C" {
    fn verrc(
        eval: libc::c_int,
        code: libc::c_int,
        fmt: *const libc::c_char,
        ap: ::core::ffi::VaList,
    );
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type va_list = __builtin_va_list;
#[no_mangle]
pub unsafe extern "C" fn errc(
    mut eval: libc::c_int,
    mut code: libc::c_int,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    verrc(eval, code, fmt, ap.as_va_list());
}
