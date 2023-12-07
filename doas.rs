use ::libc;

extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type pam_handle;
    fn closefrom(lowfd: libc::c_int);
    fn errc(eval: libc::c_int, code: libc::c_int, fmt: *const libc::c_char, _: ...);
    fn execvpe(
        file: *const libc::c_char,
        argv: *const *mut libc::c_char,
        envp: *const *mut libc::c_char,
    ) -> libc::c_int;
    fn setprogname(progname: *const libc::c_char);
    fn setresuid(ruid: uid_t, euid: uid_t, suid: uid_t) -> libc::c_int;
    fn strlcat(
        dst: *mut libc::c_char,
        src: *const libc::c_char,
        dsize: size_t,
    ) -> size_t;
    fn strlcpy(
        dst: *mut libc::c_char,
        src: *const libc::c_char,
        dsize: size_t,
    ) -> size_t;
    fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn setenv(
        __name: *const libc::c_char,
        __value: *const libc::c_char,
        __replace: libc::c_int,
    ) -> libc::c_int;
    fn warnx(__format: *const libc::c_char, _: ...);
    fn err(__status: libc::c_int, __format: *const libc::c_char, _: ...) -> !;
    fn errx(__status: libc::c_int, __format: *const libc::c_char, _: ...) -> !;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn getcwd(__buf: *mut libc::c_char, __size: size_t) -> *mut libc::c_char;
    fn dup(__fd: libc::c_int) -> libc::c_int;
    fn dup2(__fd: libc::c_int, __fd2: libc::c_int) -> libc::c_int;
    fn getuid() -> __uid_t;
    fn geteuid() -> __uid_t;
    fn getgid() -> __gid_t;
    fn getgroups(__size: libc::c_int, __list: *mut __gid_t) -> libc::c_int;
    fn setresgid(__rgid: __gid_t, __egid: __gid_t, __sgid: __gid_t) -> libc::c_int;
    fn getopt(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
    ) -> libc::c_int;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    fn getpwuid(__uid: __uid_t) -> *mut passwd;
    fn getpwnam(__name: *const libc::c_char) -> *mut passwd;
    fn getgrnam(__name: *const libc::c_char) -> *mut group;
    fn initgroups(__user: *const libc::c_char, __group: __gid_t) -> libc::c_int;
    fn syslog(__pri: libc::c_int, __fmt: *const libc::c_char, _: ...);
    fn __errno_location() -> *mut libc::c_int;
    fn pam_strerror(pamh: *mut pam_handle_t, errnum: libc::c_int) -> *const libc::c_char;
    fn pam_start(
        service_name: *const libc::c_char,
        user: *const libc::c_char,
        pam_conversation: *const pam_conv,
        pamh: *mut *mut pam_handle_t,
    ) -> libc::c_int;
    fn pam_end(pamh: *mut pam_handle_t, pam_status: libc::c_int) -> libc::c_int;
    fn pam_authenticate(pamh: *mut pam_handle_t, flags: libc::c_int) -> libc::c_int;
    fn pam_acct_mgmt(pamh: *mut pam_handle_t, flags: libc::c_int) -> libc::c_int;
    fn pam_chauthtok(pamh: *mut pam_handle_t, flags: libc::c_int) -> libc::c_int;
    fn misc_conv(
        num_msg: libc::c_int,
        msgm: *mut *const pam_message,
        response: *mut *mut pam_response,
        appdata_ptr: *mut libc::c_void,
    ) -> libc::c_int;
    static mut rules: *mut *mut rule;
    static mut nrules: size_t;
    static mut parse_errors: libc::c_int;
    fn prepenv(
        _: *mut rule,
        original: *mut passwd,
        target: *mut passwd,
    ) -> *mut *mut libc::c_char;
    fn copyenvpw(original: *mut passwd) -> *mut passwd;
}
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type gid_t = __gid_t;
pub type uid_t = __uid_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct passwd {
    pub pw_name: *mut libc::c_char,
    pub pw_passwd: *mut libc::c_char,
    pub pw_uid: __uid_t,
    pub pw_gid: __gid_t,
    pub pw_gecos: *mut libc::c_char,
    pub pw_dir: *mut libc::c_char,
    pub pw_shell: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct group {
    pub gr_name: *mut libc::c_char,
    pub gr_passwd: *mut libc::c_char,
    pub gr_gid: __gid_t,
    pub gr_mem: *mut *mut libc::c_char,
}
pub type pam_handle_t = pam_handle;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pam_message {
    pub msg_style: libc::c_int,
    pub msg: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pam_response {
    pub resp: *mut libc::c_char,
    pub resp_retcode: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pam_conv {
    pub conv: Option::<
        unsafe extern "C" fn(
            libc::c_int,
            *mut *const pam_message,
            *mut *mut pam_response,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub appdata_ptr: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rule {
    pub action: libc::c_int,
    pub options: libc::c_int,
    pub ident: *const libc::c_char,
    pub target: *const libc::c_char,
    pub cmd: *const libc::c_char,
    pub cmdargs: *mut *const libc::c_char,
    pub envlist: *mut *const libc::c_char,
}
static mut pamc: pam_conv = unsafe {
    {
        let mut init = pam_conv {
            conv: Some(
                misc_conv
                    as unsafe extern "C" fn(
                        libc::c_int,
                        *mut *const pam_message,
                        *mut *mut pam_response,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            appdata_ptr: 0 as *const libc::c_void as *mut libc::c_void,
        };
        init
    }
};
unsafe extern "C" fn usage() {
    fprintf(
        stderr,
        b"usage: doas [-nSs] [-a style] [-C config] [-u user] command [args]\n\0"
            as *const u8 as *const libc::c_char,
    );
    exit(1 as libc::c_int);
}
unsafe extern "C" fn parseuid(
    mut s: *const libc::c_char,
    mut uid: *mut uid_t,
) -> libc::c_int {
    let mut pw: *mut passwd = 0 as *mut passwd;
    pw = getpwnam(s);
    if !pw.is_null() {
        *uid = (*pw).pw_uid;
        return 0 as libc::c_int;
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn uidcheck(
    mut s: *const libc::c_char,
    mut desired: uid_t,
) -> libc::c_int {
    let mut uid: uid_t = 0;
    if parseuid(s, &mut uid) != 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if uid != desired {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn parsegid(
    mut s: *const libc::c_char,
    mut gid: *mut gid_t,
) -> libc::c_int {
    let mut gr: *mut group = 0 as *mut group;
    gr = getgrnam(s);
    if !gr.is_null() {
        *gid = (*gr).gr_gid;
        return 0 as libc::c_int;
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn match_0(
    mut uid: uid_t,
    mut groups: *mut gid_t,
    mut ngroups: libc::c_int,
    mut target: uid_t,
    mut cmd: *const libc::c_char,
    mut cmdargs: *mut *const libc::c_char,
    mut r: *mut rule,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    if *((*r).ident).offset(0 as libc::c_int as isize) as libc::c_int == ':' as i32 {
        let mut rgid: gid_t = 0;
        if parsegid(((*r).ident).offset(1 as libc::c_int as isize), &mut rgid)
            == -(1 as libc::c_int)
        {
            return 0 as libc::c_int;
        }
        i = 0 as libc::c_int;
        while i < ngroups {
            if rgid == *groups.offset(i as isize) {
                break;
            }
            i += 1;
            i;
        }
        if i == ngroups {
            return 0 as libc::c_int;
        }
    } else if uidcheck((*r).ident, uid) != 0 as libc::c_int {
        return 0 as libc::c_int
    }
    if !((*r).target).is_null() && uidcheck((*r).target, target) != 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if !((*r).cmd).is_null() {
        if strcmp((*r).cmd, cmd) != 0 {
            return 0 as libc::c_int;
        }
        if !((*r).cmdargs).is_null() {
            i = 0 as libc::c_int;
            while !(*((*r).cmdargs).offset(i as isize)).is_null() {
                if (*cmdargs.offset(i as isize)).is_null() {
                    return 0 as libc::c_int;
                }
                if strcmp(
                    *((*r).cmdargs).offset(i as isize),
                    *cmdargs.offset(i as isize),
                ) != 0
                {
                    return 0 as libc::c_int;
                }
                i += 1;
                i;
            }
            if !(*cmdargs.offset(i as isize)).is_null() {
                return 0 as libc::c_int;
            }
        }
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn permit(
    mut uid: uid_t,
    mut groups: *mut gid_t,
    mut ngroups: libc::c_int,
    mut lastr: *mut *mut rule,
    mut target: uid_t,
    mut cmd: *const libc::c_char,
    mut cmdargs: *mut *const libc::c_char,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    *lastr = 0 as *mut rule;
    i = 0 as libc::c_int;
    while (i as libc::c_ulong) < nrules {
        if match_0(uid, groups, ngroups, target, cmd, cmdargs, *rules.offset(i as isize))
            != 0
        {
            *lastr = *rules.offset(i as isize);
        }
        i += 1;
        i;
    }
    if (*lastr).is_null() {
        return 0 as libc::c_int;
    }
    return ((**lastr).action == 1 as libc::c_int) as libc::c_int;
}
unsafe extern "C" fn parseconfig(
    mut filename: *const libc::c_char,
    mut checkperms: libc::c_int,
) {
    extern "C" {
        static mut yyfp: *mut FILE;
    }
    extern "C" {
        #[link_name = "yyparse"]
        fn yyparse_0() -> libc::c_int;
    }
    let mut sb: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    yyfp = fopen(filename, b"r\0" as *const u8 as *const libc::c_char);
    if yyfp.is_null() {
        err(
            1 as libc::c_int,
            if checkperms != 0 {
                b"doas is not enabled, %s\0" as *const u8 as *const libc::c_char
            } else {
                b"could not open config file %s\0" as *const u8 as *const libc::c_char
            },
            filename,
        );
    }
    if checkperms != 0 {
        if fstat(fileno(yyfp), &mut sb) != 0 as libc::c_int {
            err(
                1 as libc::c_int,
                b"fstat(\"%s\")\0" as *const u8 as *const libc::c_char,
                filename,
            );
        }
        if sb.st_mode
            & (0o200 as libc::c_int >> 3 as libc::c_int
                | 0o200 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int)
                as libc::c_uint != 0 as libc::c_int as libc::c_uint
        {
            errx(
                1 as libc::c_int,
                b"%s is writable by group or other\0" as *const u8
                    as *const libc::c_char,
                filename,
            );
        }
        if sb.st_uid != 0 as libc::c_int as libc::c_uint {
            errx(
                1 as libc::c_int,
                b"%s is not owned by root\0" as *const u8 as *const libc::c_char,
                filename,
            );
        }
    }
    yyparse_0();
    fclose(yyfp);
    if parse_errors != 0 {
        exit(1 as libc::c_int);
    }
}
unsafe extern "C" fn checkconfig(
    mut confpath: *const libc::c_char,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut uid: uid_t,
    mut groups: *mut gid_t,
    mut ngroups: libc::c_int,
    mut target: uid_t,
) {
    let mut rule: *mut rule = 0 as *mut rule;
    let mut status: libc::c_int = 0;
    status = setresuid(uid, uid, uid);
    if status == -(1 as libc::c_int) {
        errx(
            1 as libc::c_int,
            b"unable to set uid to %d\0" as *const u8 as *const libc::c_char,
            uid,
        );
    }
    parseconfig(confpath, 0 as libc::c_int);
    if argc == 0 {
        exit(0 as libc::c_int);
    }
    if permit(
        uid,
        groups,
        ngroups,
        &mut rule,
        target,
        *argv.offset(0 as libc::c_int as isize),
        (argv as *mut *const libc::c_char).offset(1 as libc::c_int as isize),
    ) != 0
    {
        printf(
            b"permit%s\n\0" as *const u8 as *const libc::c_char,
            if (*rule).options & 0x1 as libc::c_int != 0 {
                b" nopass\0" as *const u8 as *const libc::c_char
            } else {
                b"\0" as *const u8 as *const libc::c_char
            },
        );
        exit(0 as libc::c_int);
    } else {
        printf(b"deny\n\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    };
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut safepath: *const libc::c_char = b"/bin:/sbin:/usr/bin:/usr/sbin:/usr/local/bin:/usr/local/sbin\0"
        as *const u8 as *const libc::c_char;
    let mut confpath: *const libc::c_char = 0 as *const libc::c_char;
    let mut shargv: [*mut libc::c_char; 2] = [
        0 as *mut libc::c_char,
        0 as *mut libc::c_char,
    ];
    let mut sh: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cmd: *const libc::c_char = 0 as *const libc::c_char;
    let mut cmdline: [libc::c_char; 2048] = [0; 2048];
    let mut myname: [libc::c_char; 33] = [0; 33];
    let mut targetname: [libc::c_char; 33] = [0; 33];
    let mut original_pw: *mut passwd = 0 as *mut passwd;
    let mut target_pw: *mut passwd = 0 as *mut passwd;
    let mut temp_pw: *mut passwd = 0 as *mut passwd;
    let mut rule: *mut rule = 0 as *mut rule;
    let mut uid: uid_t = 0;
    let mut target: uid_t = 0 as libc::c_int as uid_t;
    let mut groups: [gid_t; 65537] = [0; 65537];
    let mut ngroups: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut ch: libc::c_int = 0;
    let mut Sflag: libc::c_int = 0 as libc::c_int;
    let mut sflag: libc::c_int = 0 as libc::c_int;
    let mut nflag: libc::c_int = 0 as libc::c_int;
    let mut cwdpath: [libc::c_char; 4096] = [0; 4096];
    let mut cwd: *const libc::c_char = 0 as *const libc::c_char;
    let mut envp: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    setprogname(b"doas\0" as *const u8 as *const libc::c_char);
    closefrom(2 as libc::c_int + 1 as libc::c_int);
    uid = getuid();
    targetname[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    loop {
        ch = getopt(argc, argv, b"+a:C:nSsu:\0" as *const u8 as *const libc::c_char);
        if !(ch != -(1 as libc::c_int)) {
            break;
        }
        let mut current_block_12: u64;
        match ch {
            67 => {
                confpath = optarg;
                current_block_12 = 2668756484064249700;
            }
            117 => {
                if strlcpy(
                    targetname.as_mut_ptr(),
                    optarg,
                    ::core::mem::size_of::<[libc::c_char; 33]>() as libc::c_ulong,
                ) >= ::core::mem::size_of::<[libc::c_char; 33]>() as libc::c_ulong
                {
                    errx(
                        1 as libc::c_int,
                        b"pw_name too long\0" as *const u8 as *const libc::c_char,
                    );
                }
                if parseuid(targetname.as_mut_ptr(), &mut target) != 0 as libc::c_int {
                    errx(
                        1 as libc::c_int,
                        b"unknown user\0" as *const u8 as *const libc::c_char,
                    );
                }
                current_block_12 = 2668756484064249700;
            }
            110 => {
                nflag = 1 as libc::c_int;
                current_block_12 = 2668756484064249700;
            }
            83 => {
                Sflag = 1 as libc::c_int;
                current_block_12 = 2926905216922942282;
            }
            115 => {
                current_block_12 = 2926905216922942282;
            }
            _ => {
                usage();
                current_block_12 = 2668756484064249700;
            }
        }
        match current_block_12 {
            2926905216922942282 => {
                sflag = 1 as libc::c_int;
            }
            _ => {}
        }
    }
    argv = argv.offset(optind as isize);
    argc -= optind;
    if !confpath.is_null() {
        if sflag != 0 {
            usage();
        }
    } else if sflag == 0 && argc == 0 || sflag != 0 && argc != 0 {
        usage();
    }
    temp_pw = getpwuid(uid);
    original_pw = copyenvpw(temp_pw);
    if original_pw.is_null() {
        err(1 as libc::c_int, b"getpwuid failed\0" as *const u8 as *const libc::c_char);
    }
    if strlcpy(
        myname.as_mut_ptr(),
        (*original_pw).pw_name,
        ::core::mem::size_of::<[libc::c_char; 33]>() as libc::c_ulong,
    ) >= ::core::mem::size_of::<[libc::c_char; 33]>() as libc::c_ulong
    {
        errx(
            1 as libc::c_int,
            b"pw_name too long\0" as *const u8 as *const libc::c_char,
        );
    }
    ngroups = getgroups(65536 as libc::c_int, groups.as_mut_ptr());
    if ngroups == -(1 as libc::c_int) {
        err(1 as libc::c_int, b"can't get groups\0" as *const u8 as *const libc::c_char);
    }
    let fresh0 = ngroups;
    ngroups = ngroups + 1;
    groups[fresh0 as usize] = getgid();
    if sflag != 0 {
        sh = getenv(b"SHELL\0" as *const u8 as *const libc::c_char);
        if sh.is_null() || *sh as libc::c_int == '\0' as i32 {
            shargv[0 as libc::c_int as usize] = strdup((*original_pw).pw_shell);
            if (shargv[0 as libc::c_int as usize]).is_null() {
                err(1 as libc::c_int, 0 as *const libc::c_char);
            }
        } else {
            shargv[0 as libc::c_int as usize] = sh;
        }
        argv = shargv.as_mut_ptr();
        argc = 1 as libc::c_int;
    }
    if !confpath.is_null() {
        checkconfig(confpath, argc, argv, uid, groups.as_mut_ptr(), ngroups, target);
        exit(1 as libc::c_int);
    }
    if geteuid() != 0 {
        errx(
            1 as libc::c_int,
            b"not installed setuid\0" as *const u8 as *const libc::c_char,
        );
    }
    parseconfig(
        b"/usr/local/etc/doas.conf\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
    );
    strlcpy(
        cmdline.as_mut_ptr(),
        *argv.offset(0 as libc::c_int as isize),
        ::core::mem::size_of::<[libc::c_char; 2048]>() as libc::c_ulong,
    );
    i = 1 as libc::c_int;
    while i < argc {
        if strlcat(
            cmdline.as_mut_ptr(),
            b" \0" as *const u8 as *const libc::c_char,
            ::core::mem::size_of::<[libc::c_char; 2048]>() as libc::c_ulong,
        ) >= ::core::mem::size_of::<[libc::c_char; 2048]>() as libc::c_ulong
        {
            break;
        }
        if strlcat(
            cmdline.as_mut_ptr(),
            *argv.offset(i as isize),
            ::core::mem::size_of::<[libc::c_char; 2048]>() as libc::c_ulong,
        ) >= ::core::mem::size_of::<[libc::c_char; 2048]>() as libc::c_ulong
        {
            break;
        }
        i += 1;
        i;
    }
    cmd = *argv.offset(0 as libc::c_int as isize);
    if permit(
        uid,
        groups.as_mut_ptr(),
        ngroups,
        &mut rule,
        target,
        cmd,
        (argv as *mut *const libc::c_char).offset(1 as libc::c_int as isize),
    ) == 0
    {
        syslog(
            (10 as libc::c_int) << 3 as libc::c_int | 5 as libc::c_int,
            b"failed command for %s: %s\0" as *const u8 as *const libc::c_char,
            myname.as_mut_ptr(),
            cmdline.as_mut_ptr(),
        );
        errc(1 as libc::c_int, 1 as libc::c_int, 0 as *const libc::c_char);
    }
    if Sflag != 0 {
        let ref mut fresh1 = *argv.offset(0 as libc::c_int as isize);
        *fresh1 = b"-doas\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if (*rule).options & 0x1 as libc::c_int == 0 {
        if nflag != 0 {
            errx(
                1 as libc::c_int,
                b"Authorization required\0" as *const u8 as *const libc::c_char,
            );
        }
        let mut pamh: *mut pam_handle_t = 0 as *mut pam_handle_t;
        let mut pam_err: libc::c_int = 0;
        let mut temp_stdin: libc::c_int = 0;
        temp_stdin = dup(0 as libc::c_int);
        if temp_stdin == -(1 as libc::c_int) {
            err(1 as libc::c_int, b"dup\0" as *const u8 as *const libc::c_char);
        }
        close(0 as libc::c_int);
        let mut temp_stdout: libc::c_int = dup(1 as libc::c_int);
        if temp_stdout == -(1 as libc::c_int) {
            err(1 as libc::c_int, b"dup\0" as *const u8 as *const libc::c_char);
        }
        close(1 as libc::c_int);
        if dup2(2 as libc::c_int, 1 as libc::c_int) == -(1 as libc::c_int) {
            err(1 as libc::c_int, b"dup2\0" as *const u8 as *const libc::c_char);
        }
        pam_err = pam_start(
            b"doas\0" as *const u8 as *const libc::c_char,
            myname.as_mut_ptr(),
            &mut pamc,
            &mut pamh,
        );
        if pam_err != 0 as libc::c_int {
            if !pamh.is_null() {
                syslog(
                    3 as libc::c_int,
                    b"%s: %s\0" as *const u8 as *const libc::c_char,
                    b"pam_start\0" as *const u8 as *const libc::c_char,
                    pam_strerror(pamh, pam_err),
                );
                warnx(
                    b"%s: %s\0" as *const u8 as *const libc::c_char,
                    b"pam_start\0" as *const u8 as *const libc::c_char,
                    pam_strerror(pamh, pam_err),
                );
                pam_end(pamh, pam_err);
                exit(1 as libc::c_int);
            }
            syslog(
                3 as libc::c_int,
                b"pam_start failed: %s\0" as *const u8 as *const libc::c_char,
                pam_strerror(pamh, pam_err),
            );
            errx(
                1 as libc::c_int,
                b"pam_start failed\0" as *const u8 as *const libc::c_char,
            );
        }
        pam_err = pam_authenticate(pamh, 0x8000 as libc::c_uint as libc::c_int);
        match pam_err {
            0 => {
                pam_err = pam_acct_mgmt(pamh, 0x8000 as libc::c_uint as libc::c_int);
                match pam_err {
                    0 => {}
                    12 => {
                        pam_err = pam_chauthtok(
                            pamh,
                            (0x8000 as libc::c_uint | 0x20 as libc::c_uint)
                                as libc::c_int,
                        );
                        if pam_err != 0 as libc::c_int {
                            syslog(
                                3 as libc::c_int,
                                b"%s: %s\0" as *const u8 as *const libc::c_char,
                                b"pam_chauthtok\0" as *const u8 as *const libc::c_char,
                                pam_strerror(pamh, pam_err),
                            );
                            warnx(
                                b"%s: %s\0" as *const u8 as *const libc::c_char,
                                b"pam_chauthtok\0" as *const u8 as *const libc::c_char,
                                pam_strerror(pamh, pam_err),
                            );
                            pam_end(pamh, pam_err);
                            exit(1 as libc::c_int);
                        }
                    }
                    7 | 10 | 11 => {
                        syslog(
                            (10 as libc::c_int) << 3 as libc::c_int | 5 as libc::c_int,
                            b"failed auth for %s\0" as *const u8 as *const libc::c_char,
                            myname.as_mut_ptr(),
                        );
                        errx(
                            1 as libc::c_int,
                            b"second authentication failed\0" as *const u8
                                as *const libc::c_char,
                        );
                    }
                    _ => {
                        syslog(
                            3 as libc::c_int,
                            b"%s: %s\0" as *const u8 as *const libc::c_char,
                            b"pam_acct_mgmt\0" as *const u8 as *const libc::c_char,
                            pam_strerror(pamh, pam_err),
                        );
                        warnx(
                            b"%s: %s\0" as *const u8 as *const libc::c_char,
                            b"pam_acct_mgmt\0" as *const u8 as *const libc::c_char,
                            pam_strerror(pamh, pam_err),
                        );
                        pam_end(pamh, pam_err);
                        exit(1 as libc::c_int);
                    }
                }
            }
            7 | 10 | 11 => {
                syslog(
                    (10 as libc::c_int) << 3 as libc::c_int | 5 as libc::c_int,
                    b"failed auth for %s\0" as *const u8 as *const libc::c_char,
                    myname.as_mut_ptr(),
                );
                errx(
                    1 as libc::c_int,
                    b"authentication failed\0" as *const u8 as *const libc::c_char,
                );
            }
            _ => {
                syslog(
                    3 as libc::c_int,
                    b"%s: %s\0" as *const u8 as *const libc::c_char,
                    b"pam_authenticate\0" as *const u8 as *const libc::c_char,
                    pam_strerror(pamh, pam_err),
                );
                warnx(
                    b"%s: %s\0" as *const u8 as *const libc::c_char,
                    b"pam_authenticate\0" as *const u8 as *const libc::c_char,
                    pam_strerror(pamh, pam_err),
                );
                pam_end(pamh, pam_err);
                exit(1 as libc::c_int);
            }
        }
        pam_end(pamh, pam_err);
        close(1 as libc::c_int);
        if dup2(temp_stdout, 1 as libc::c_int) == -(1 as libc::c_int) {
            err(1 as libc::c_int, b"dup2\0" as *const u8 as *const libc::c_char);
        }
    }
    if targetname[0 as libc::c_int as usize] as libc::c_int == '\0' as i32 {
        temp_pw = getpwuid(target);
    } else {
        temp_pw = getpwnam(targetname.as_mut_ptr());
    }
    if *((*temp_pw).pw_shell).offset(0 as libc::c_int as isize) as libc::c_int
        == '\0' as i32
    {
        (*temp_pw).pw_shell = strdup(b"/bin/sh\0" as *const u8 as *const libc::c_char);
    }
    target_pw = copyenvpw(temp_pw);
    if target_pw.is_null() {
        errx(
            1 as libc::c_int,
            b"no passwd entry for target\0" as *const u8 as *const libc::c_char,
        );
    }
    if setresgid((*target_pw).pw_gid, (*target_pw).pw_gid, (*target_pw).pw_gid)
        == -(1 as libc::c_int)
    {
        err(1 as libc::c_int, b"setresgid\0" as *const u8 as *const libc::c_char);
    }
    if initgroups((*target_pw).pw_name, (*target_pw).pw_gid) == -(1 as libc::c_int) {
        err(1 as libc::c_int, b"initgroups\0" as *const u8 as *const libc::c_char);
    }
    if setresuid(target, target, target) == -(1 as libc::c_int) {
        err(1 as libc::c_int, b"setresuid\0" as *const u8 as *const libc::c_char);
    }
    if (getcwd(
        cwdpath.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
    ))
        .is_null()
    {
        cwd = b"(failed)\0" as *const u8 as *const libc::c_char;
    } else {
        cwd = cwdpath.as_mut_ptr();
    }
    if (*rule).options & 0x8 as libc::c_int == 0 {
        syslog(
            (10 as libc::c_int) << 3 as libc::c_int | 6 as libc::c_int,
            b"%s ran command %s as %s from %s\0" as *const u8 as *const libc::c_char,
            myname.as_mut_ptr(),
            cmdline.as_mut_ptr(),
            (*target_pw).pw_name,
            cwd,
        );
    }
    envp = prepenv(rule, original_pw, target_pw);
    if !((*rule).cmd).is_null() {
        if setenv(
            b"PATH\0" as *const u8 as *const libc::c_char,
            safepath,
            1 as libc::c_int,
        ) == -(1 as libc::c_int)
        {
            err(
                1 as libc::c_int,
                b"failed to set PATH '%s'\0" as *const u8 as *const libc::c_char,
                safepath,
            );
        }
    }
    execvpe(cmd, argv, envp);
    if *__errno_location() == 2 as libc::c_int {
        errx(
            1 as libc::c_int,
            b"%s: command not found\0" as *const u8 as *const libc::c_char,
            cmd,
        );
    }
    err(1 as libc::c_int, b"%s\0" as *const u8 as *const libc::c_char, cmd);
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}
