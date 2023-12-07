use ::libc;
extern "C" {
    pub type __dirstream;
    fn strtonum(
        numstr: *const libc::c_char,
        minval: libc::c_longlong,
        maxval: libc::c_longlong,
        errstrp: *mut *const libc::c_char,
    ) -> libc::c_longlong;
    fn reallocarray(
        optr: *mut libc::c_void,
        nmemb: size_t,
        size: size_t,
    ) -> *mut libc::c_void;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn free(_: *mut libc::c_void);
    fn opendir(__name: *const libc::c_char) -> *mut DIR;
    fn closedir(__dirp: *mut DIR) -> libc::c_int;
    fn readdir(__dirp: *mut DIR) -> *mut dirent;
    fn dirfd(__dirp: *mut DIR) -> libc::c_int;
}
pub type __ino_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type DIR = __dirstream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dirent {
    pub d_ino: __ino_t,
    pub d_off: __off_t,
    pub d_reclen: libc::c_ushort,
    pub d_type: libc::c_uchar,
    pub d_name: [libc::c_char; 256],
}
#[inline]
unsafe extern "C" fn closefrom_close(mut fd: libc::c_int) {
    close(fd);
}
#[no_mangle]
pub unsafe extern "C" fn closefrom(mut lowfd: libc::c_int) {
    let mut path: *const libc::c_char = 0 as *const libc::c_char;
    let mut dirp: *mut DIR = 0 as *mut DIR;
    let mut dent: *mut dirent = 0 as *mut dirent;
    let mut fd_array: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut fd_array_used: libc::c_int = 0 as libc::c_int;
    let mut fd_array_size: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    path = b"/proc/self/fd\0" as *const u8 as *const libc::c_char;
    dirp = opendir(path);
    if dirp.is_null() {
        return;
    }
    loop {
        dent = readdir(dirp);
        if dent.is_null() {
            break;
        }
        let mut errstr: *const libc::c_char = 0 as *const libc::c_char;
        let mut fd: libc::c_int = 0;
        fd = strtonum(
            ((*dent).d_name).as_mut_ptr(),
            lowfd as libc::c_longlong,
            2147483647 as libc::c_int as libc::c_longlong,
            &mut errstr,
        ) as libc::c_int;
        if !errstr.is_null() || fd == dirfd(dirp) {
            continue;
        }
        if fd_array_used >= fd_array_size {
            let mut ptr: *mut libc::c_int = 0 as *mut libc::c_int;
            if fd_array_size > 0 as libc::c_int {
                fd_array_size *= 2 as libc::c_int;
            } else {
                fd_array_size = 32 as libc::c_int;
            }
            ptr = reallocarray(
                fd_array as *mut libc::c_void,
                fd_array_size as size_t,
                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
            ) as *mut libc::c_int;
            if ptr.is_null() {
                break;
            }
            fd_array = ptr;
        }
        let fresh0 = fd_array_used;
        fd_array_used = fd_array_used + 1;
        *fd_array.offset(fresh0 as isize) = fd;
    }
    i = 0 as libc::c_int;
    while i < fd_array_used {
        closefrom_close(*fd_array.offset(i as isize));
        i += 1;
        i;
    }
    free(fd_array as *mut libc::c_void);
    closedir(dirp);
}
