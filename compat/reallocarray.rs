use ::libc;
extern "C" {
    fn __errno_location() -> *mut libc::c_int;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
#[no_mangle]
pub unsafe extern "C" fn reallocarray(
    mut optr: *mut libc::c_void,
    mut nmemb: size_t,
    mut size: size_t,
) -> *mut libc::c_void {
    if (nmemb
        >= (1 as libc::c_int as size_t)
            << (::core::mem::size_of::<size_t>() as libc::c_ulong)
                .wrapping_mul(4 as libc::c_int as libc::c_ulong)
        || size
            >= (1 as libc::c_int as size_t)
                << (::core::mem::size_of::<size_t>() as libc::c_ulong)
                    .wrapping_mul(4 as libc::c_int as libc::c_ulong))
        && nmemb > 0 as libc::c_int as libc::c_ulong
        && (18446744073709551615 as libc::c_ulong).wrapping_div(nmemb) < size
    {
        *__errno_location() = 12 as libc::c_int;
        return 0 as *mut libc::c_void;
    }
    return realloc(optr, size.wrapping_mul(nmemb));
}
