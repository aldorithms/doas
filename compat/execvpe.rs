use ::libc;
extern "C" {
    fn strlcpy(
        dst: *mut libc::c_char,
        src: *const libc::c_char,
        dsize: size_t,
    ) -> size_t;
    fn writev(__fd: libc::c_int, __iovec: *const iovec, __count: libc::c_int) -> ssize_t;
    fn __errno_location() -> *mut libc::c_int;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn bcopy(__src: *const libc::c_void, __dest: *mut libc::c_void, __n: size_t);
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strsep(
        __stringp: *mut *mut libc::c_char,
        __delim: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn execve(
        __path: *const libc::c_char,
        __argv: *const *mut libc::c_char,
        __envp: *const *mut libc::c_char,
    ) -> libc::c_int;
}
pub type __ssize_t = libc::c_long;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iovec {
    pub iov_base: *mut libc::c_void,
    pub iov_len: size_t,
}
#[no_mangle]
pub unsafe extern "C" fn execvpe(
    mut name: *const libc::c_char,
    mut argv: *const *mut libc::c_char,
    mut envp: *const *mut libc::c_char,
) -> libc::c_int {
    let mut current_block: u64;
    let mut memp: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut cnt: libc::c_int = 0;
    let mut lp: size_t = 0;
    let mut ln: size_t = 0;
    let mut len: size_t = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut eacces: libc::c_int = 0 as libc::c_int;
    let mut bp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cur: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut path: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buf: [libc::c_char; 4096] = [0; 4096];
    if name.is_null() || *name as libc::c_int == '\0' as i32 {
        *__errno_location() = 2 as libc::c_int;
        return -(1 as libc::c_int);
    }
    if !(strchr(name, '/' as i32)).is_null() {
        bp = name as *mut libc::c_char;
        path = 0 as *mut libc::c_char;
        cur = path;
        current_block = 1832559870555093816;
    } else {
        bp = buf.as_mut_ptr();
        path = getenv(b"PATH\0" as *const u8 as *const libc::c_char);
        if path.is_null() {
            path = b"/usr/bin:/bin\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
        }
        len = (strlen(path)).wrapping_add(1 as libc::c_int as libc::c_ulong);
        let mut fresh0 = ::std::vec::from_elem(0, len as usize);
        cur = fresh0.as_mut_ptr() as *mut libc::c_char;
        if cur.is_null() {
            *__errno_location() = 12 as libc::c_int;
            return -(1 as libc::c_int);
        }
        strlcpy(cur, path, len);
        current_block = 12599329904712511516;
    }
    loop {
        match current_block {
            12599329904712511516 => {
                p = strsep(&mut cur, b":\0" as *const u8 as *const libc::c_char);
                if p.is_null() {
                    current_block = 790185930182612747;
                    break;
                }
                if *p == 0 {
                    p = b".\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
                    lp = 1 as libc::c_int as size_t;
                } else {
                    lp = strlen(p);
                }
                ln = strlen(name);
                if lp.wrapping_add(ln).wrapping_add(2 as libc::c_int as libc::c_ulong)
                    > ::core::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong
                {
                    let mut iov: [iovec; 3] = [iovec {
                        iov_base: 0 as *mut libc::c_void,
                        iov_len: 0,
                    }; 3];
                    iov[0 as libc::c_int as usize]
                        .iov_base = b"execvp: \0" as *const u8 as *const libc::c_char
                        as *mut libc::c_void;
                    iov[0 as libc::c_int as usize].iov_len = 8 as libc::c_int as size_t;
                    iov[1 as libc::c_int as usize].iov_base = p as *mut libc::c_void;
                    iov[1 as libc::c_int as usize].iov_len = lp;
                    iov[2 as libc::c_int as usize]
                        .iov_base = b": path too long\n\0" as *const u8
                        as *const libc::c_char as *mut libc::c_void;
                    iov[2 as libc::c_int as usize].iov_len = 16 as libc::c_int as size_t;
                    writev(2 as libc::c_int, iov.as_mut_ptr(), 3 as libc::c_int);
                    current_block = 12599329904712511516;
                } else {
                    bcopy(
                        p as *const libc::c_void,
                        buf.as_mut_ptr() as *mut libc::c_void,
                        lp,
                    );
                    buf[lp as usize] = '/' as i32 as libc::c_char;
                    bcopy(
                        name as *const libc::c_void,
                        buf
                            .as_mut_ptr()
                            .offset(lp as isize)
                            .offset(1 as libc::c_int as isize) as *mut libc::c_void,
                        ln,
                    );
                    buf[lp
                        .wrapping_add(ln)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                        as usize] = '\0' as i32 as libc::c_char;
                    current_block = 1832559870555093816;
                }
            }
            _ => {
                execve(bp, argv, envp);
                match *__errno_location() {
                    8 => {
                        cnt = 0 as libc::c_int;
                        while !(*argv.offset(cnt as isize)).is_null() {
                            cnt += 1;
                            cnt;
                        }
                        let mut fresh1 = ::std::vec::from_elem(
                            0,
                            ((cnt + 2 as libc::c_int) as libc::c_ulong)
                                .wrapping_mul(
                                    ::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                                ) as usize,
                        );
                        memp = fresh1.as_mut_ptr() as *mut *mut libc::c_char;
                        if memp.is_null() {
                            current_block = 11600023080462952803;
                            break;
                        }
                        let ref mut fresh2 = *memp.offset(0 as libc::c_int as isize);
                        *fresh2 = b"sh\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char;
                        let ref mut fresh3 = *memp.offset(1 as libc::c_int as isize);
                        *fresh3 = bp;
                        bcopy(
                            argv.offset(1 as libc::c_int as isize)
                                as *const libc::c_void,
                            memp.offset(2 as libc::c_int as isize) as *mut libc::c_void,
                            (cnt as libc::c_ulong)
                                .wrapping_mul(
                                    ::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                                ),
                        );
                        execve(
                            b"/bin/sh\0" as *const u8 as *const libc::c_char,
                            memp as *const *mut libc::c_char,
                            envp,
                        );
                        current_block = 11600023080462952803;
                        break;
                    }
                    21 | 40 | 36 | 2 | 20 => {
                        current_block = 12599329904712511516;
                    }
                    13 => {
                        eacces = 1 as libc::c_int;
                        current_block = 12599329904712511516;
                    }
                    7 | 12 | 26 | _ => {
                        current_block = 11600023080462952803;
                        break;
                    }
                }
            }
        }
    }
    match current_block {
        790185930182612747 => {
            if eacces != 0 {
                *__errno_location() = 13 as libc::c_int;
            } else if *__errno_location() == 0 {
                *__errno_location() = 2 as libc::c_int;
            }
        }
        _ => {}
    }
    return -(1 as libc::c_int);
}
