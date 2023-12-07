use ::libc;
extern "C" {
    static mut __progname: *const libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn getprogname() -> *const libc::c_char {
    return __progname;
}
