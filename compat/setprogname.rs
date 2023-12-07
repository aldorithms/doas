use ::libc;
extern "C" {
    static mut __progname: *const libc::c_char;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn setprogname(mut progname: *const libc::c_char) {
    let mut tmpn: *mut libc::c_char = 0 as *mut libc::c_char;
    tmpn = strrchr(progname, '/' as i32);
    if tmpn.is_null() {
        __progname = progname as *mut libc::c_char;
    } else {
        __progname = tmpn.offset(1 as libc::c_int as isize);
    };
}
