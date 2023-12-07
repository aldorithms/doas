use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn reallocarray(
        optr: *mut libc::c_void,
        nmemb: size_t,
        size: size_t,
    ) -> *mut libc::c_void;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn vfprintf(
        _: *mut FILE,
        _: *const libc::c_char,
        _: ::core::ffi::VaList,
    ) -> libc::c_int;
    fn getc(__stream: *mut FILE) -> libc::c_int;
    fn ungetc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn err(__status: libc::c_int, __format: *const libc::c_char, _: ...) -> !;
    fn errx(__status: libc::c_int, __format: *const libc::c_char, _: ...) -> !;
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
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type va_list = __builtin_va_list;
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
pub struct rule {
    pub action: libc::c_int,
    pub options: libc::c_int,
    pub ident: *const libc::c_char,
    pub target: *const libc::c_char,
    pub cmd: *const libc::c_char,
    pub cmdargs: *mut *const libc::c_char,
    pub envlist: *mut *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yystype {
    pub c2rust_unnamed: C2RustUnnamed,
    pub lineno: libc::c_int,
    pub colno: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub c2rust_unnamed: C2RustUnnamed_0,
    pub strlist: *mut *const libc::c_char,
    pub str_0: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub action: libc::c_int,
    pub options: libc::c_int,
    pub cmd: *const libc::c_char,
    pub cmdargs: *mut *const libc::c_char,
    pub envlist: *mut *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct keyword {
    pub word: *const libc::c_char,
    pub token: libc::c_int,
}
pub type yy_state_t = yytype_int8;
pub type yytype_int8 = libc::c_schar;
pub type yysymbol_kind_t = libc::c_int;
pub const YYSYMBOL_args: yysymbol_kind_t = 27;
pub const YYSYMBOL_cmd: yysymbol_kind_t = 26;
pub const YYSYMBOL_target: yysymbol_kind_t = 25;
pub const YYSYMBOL_ident: yysymbol_kind_t = 24;
pub const YYSYMBOL_strlist: yysymbol_kind_t = 23;
pub const YYSYMBOL_option: yysymbol_kind_t = 22;
pub const YYSYMBOL_options: yysymbol_kind_t = 21;
pub const YYSYMBOL_action: yysymbol_kind_t = 20;
pub const YYSYMBOL_rule: yysymbol_kind_t = 19;
pub const YYSYMBOL_grammar: yysymbol_kind_t = 18;
pub const YYSYMBOL_YYACCEPT: yysymbol_kind_t = 17;
pub const YYSYMBOL_16_: yysymbol_kind_t = 16;
pub const YYSYMBOL_15_: yysymbol_kind_t = 15;
pub const YYSYMBOL_14_n_: yysymbol_kind_t = 14;
pub const YYSYMBOL_TSTRING: yysymbol_kind_t = 13;
pub const YYSYMBOL_TSETENV: yysymbol_kind_t = 12;
pub const YYSYMBOL_TKEEPENV: yysymbol_kind_t = 11;
pub const YYSYMBOL_TPERSIST: yysymbol_kind_t = 10;
pub const YYSYMBOL_TNOLOG: yysymbol_kind_t = 9;
pub const YYSYMBOL_TNOPASS: yysymbol_kind_t = 8;
pub const YYSYMBOL_TARGS: yysymbol_kind_t = 7;
pub const YYSYMBOL_TCMD: yysymbol_kind_t = 6;
pub const YYSYMBOL_TAS: yysymbol_kind_t = 5;
pub const YYSYMBOL_TDENY: yysymbol_kind_t = 4;
pub const YYSYMBOL_TPERMIT: yysymbol_kind_t = 3;
pub const YYSYMBOL_YYUNDEF: yysymbol_kind_t = 2;
pub const YYSYMBOL_YYerror: yysymbol_kind_t = 1;
pub const YYSYMBOL_YYEOF: yysymbol_kind_t = 0;
pub const YYSYMBOL_YYEMPTY: yysymbol_kind_t = -2;
pub type yy_state_fast_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub union yyalloc {
    pub yyss_alloc: yy_state_t,
    pub yyvs_alloc: yystype,
}
#[no_mangle]
pub static mut yyfp: *mut FILE = 0 as *const FILE as *mut FILE;
#[no_mangle]
pub static mut rules: *mut *mut rule = 0 as *const *mut rule as *mut *mut rule;
#[no_mangle]
pub static mut nrules: size_t = 0 as libc::c_int as size_t;
static mut maxrules: size_t = 0 as libc::c_int as size_t;
#[no_mangle]
pub static mut parse_errors: libc::c_int = 0 as libc::c_int;
unsafe extern "C" fn arraylen(mut arr: *mut *const libc::c_char) -> size_t {
    let mut cnt: size_t = 0 as libc::c_int as size_t;
    while !(*arr).is_null() {
        cnt = cnt.wrapping_add(1);
        cnt;
        arr = arr.offset(1);
        arr;
    }
    return cnt;
}
unsafe extern "C" fn yyerror(mut fmt: *const libc::c_char, mut args: ...) {
    let mut va: ::core::ffi::VaListImpl;
    fprintf(stderr, b"doas: \0" as *const u8 as *const libc::c_char);
    va = args.clone();
    vfprintf(stderr, fmt, va.as_va_list());
    fprintf(
        stderr,
        b" at line %d\n\0" as *const u8 as *const libc::c_char,
        yylval.lineno + 1 as libc::c_int,
    );
    parse_errors += 1;
    parse_errors;
}
static mut keywords: [keyword; 10] = [
    {
        let mut init = keyword {
            word: b"deny\0" as *const u8 as *const libc::c_char,
            token: 259 as libc::c_int,
        };
        init
    },
    {
        let mut init = keyword {
            word: b"permit\0" as *const u8 as *const libc::c_char,
            token: 258 as libc::c_int,
        };
        init
    },
    {
        let mut init = keyword {
            word: b"as\0" as *const u8 as *const libc::c_char,
            token: 260 as libc::c_int,
        };
        init
    },
    {
        let mut init = keyword {
            word: b"cmd\0" as *const u8 as *const libc::c_char,
            token: 261 as libc::c_int,
        };
        init
    },
    {
        let mut init = keyword {
            word: b"args\0" as *const u8 as *const libc::c_char,
            token: 262 as libc::c_int,
        };
        init
    },
    {
        let mut init = keyword {
            word: b"nopass\0" as *const u8 as *const libc::c_char,
            token: 263 as libc::c_int,
        };
        init
    },
    {
        let mut init = keyword {
            word: b"nolog\0" as *const u8 as *const libc::c_char,
            token: 264 as libc::c_int,
        };
        init
    },
    {
        let mut init = keyword {
            word: b"persist\0" as *const u8 as *const libc::c_char,
            token: 265 as libc::c_int,
        };
        init
    },
    {
        let mut init = keyword {
            word: b"keepenv\0" as *const u8 as *const libc::c_char,
            token: 266 as libc::c_int,
        };
        init
    },
    {
        let mut init = keyword {
            word: b"setenv\0" as *const u8 as *const libc::c_char,
            token: 267 as libc::c_int,
        };
        init
    },
];
unsafe extern "C" fn yylex() -> libc::c_int {
    let mut current_block: u64;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut ebuf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut quotes: libc::c_int = 0 as libc::c_int;
    let mut escape: libc::c_int = 0 as libc::c_int;
    let mut qpos: libc::c_int = -(1 as libc::c_int);
    let mut nonkw: libc::c_int = 0 as libc::c_int;
    p = buf.as_mut_ptr();
    ebuf = buf
        .as_mut_ptr()
        .offset(
            ::core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as isize,
        );
    loop {
        c = getc(yyfp);
        while c == ' ' as i32 || c == '\t' as i32 {
            yylval.colno += 1;
            yylval.colno;
            c = getc(yyfp);
        }
        match c {
            10 => {
                yylval.colno = 0 as libc::c_int;
                yylval.lineno += 1;
                yylval.lineno;
                current_block = 3511970825001807004;
                break;
            }
            123 | 125 => {
                current_block = 3511970825001807004;
                break;
            }
            35 => {
                current_block = 17216689946888361452;
                break;
            }
            -1 => {
                current_block = 13167264884375199442;
                break;
            }
            _ => {
                loop {
                    match c {
                        0 => {
                            yyerror(
                                b"unallowed character NUL in column %d\0" as *const u8
                                    as *const libc::c_char,
                                yylval.colno + 1 as libc::c_int,
                            );
                            escape = 0 as libc::c_int;
                            current_block = 12599329904712511516;
                        }
                        92 => {
                            escape = (escape == 0) as libc::c_int;
                            if escape != 0 {
                                current_block = 12599329904712511516;
                            } else {
                                current_block = 980989089337379490;
                            }
                        }
                        10 => {
                            if quotes != 0 {
                                yyerror(
                                    b"unterminated quotes in column %d\0" as *const u8
                                        as *const libc::c_char,
                                    qpos + 1 as libc::c_int,
                                );
                            }
                            if !(escape != 0) {
                                break;
                            }
                            nonkw = 1 as libc::c_int;
                            escape = 0 as libc::c_int;
                            yylval.colno = 0 as libc::c_int;
                            yylval.lineno += 1;
                            yylval.lineno;
                            current_block = 12599329904712511516;
                        }
                        -1 => {
                            if escape != 0 {
                                yyerror(
                                    b"unterminated escape in column %d\0" as *const u8
                                        as *const libc::c_char,
                                    yylval.colno,
                                );
                            }
                            if quotes != 0 {
                                yyerror(
                                    b"unterminated quotes in column %d\0" as *const u8
                                        as *const libc::c_char,
                                    qpos + 1 as libc::c_int,
                                );
                            }
                            break;
                        }
                        123 | 125 | 35 | 32 | 9 => {
                            if escape == 0 && quotes == 0 {
                                break;
                            }
                            current_block = 980989089337379490;
                        }
                        34 => {
                            if escape == 0 {
                                quotes = (quotes == 0) as libc::c_int;
                                if quotes != 0 {
                                    nonkw = 1 as libc::c_int;
                                    qpos = yylval.colno;
                                }
                                current_block = 12599329904712511516;
                            } else {
                                current_block = 980989089337379490;
                            }
                        }
                        _ => {
                            current_block = 980989089337379490;
                        }
                    }
                    match current_block {
                        980989089337379490 => {
                            let fresh0 = p;
                            p = p.offset(1);
                            *fresh0 = c as libc::c_char;
                            if p == ebuf {
                                yyerror(
                                    b"too long line\0" as *const u8 as *const libc::c_char,
                                );
                                p = buf.as_mut_ptr();
                            }
                            escape = 0 as libc::c_int;
                        }
                        _ => {}
                    }
                    c = getc(yyfp);
                    yylval.colno += 1;
                    yylval.colno;
                }
                *p = 0 as libc::c_int as libc::c_char;
                if c != -(1 as libc::c_int) {
                    ungetc(c, yyfp);
                }
                if !(p == buf.as_mut_ptr()) {
                    current_block = 5891011138178424807;
                    break;
                }
                if c == -(1 as libc::c_int) {
                    current_block = 13167264884375199442;
                    break;
                }
                if !(qpos == -(1 as libc::c_int)) {
                    current_block = 5891011138178424807;
                    break;
                }
            }
        }
    }
    match current_block {
        17216689946888361452 => {
            loop {
                c = getc(yyfp);
                if !(c != '\n' as i32) {
                    current_block = 2868539653012386629;
                    break;
                }
                if c == -(1 as libc::c_int) {
                    current_block = 13167264884375199442;
                    break;
                }
            }
            match current_block {
                13167264884375199442 => {}
                _ => {
                    yylval.colno = 0 as libc::c_int;
                    yylval.lineno += 1;
                    yylval.lineno;
                    return c;
                }
            }
        }
        5891011138178424807 => {
            if nonkw == 0 {
                i = 0 as libc::c_int;
                while (i as libc::c_ulong)
                    < (::core::mem::size_of::<[keyword; 10]>() as libc::c_ulong)
                        .wrapping_div(::core::mem::size_of::<keyword>() as libc::c_ulong)
                {
                    if strcmp(buf.as_mut_ptr(), keywords[i as usize].word)
                        == 0 as libc::c_int
                    {
                        return keywords[i as usize].token;
                    }
                    i += 1;
                    i;
                }
            }
            str = strdup(buf.as_mut_ptr());
            if str.is_null() {
                err(1 as libc::c_int, b"strdup\0" as *const u8 as *const libc::c_char);
            }
            yylval.c2rust_unnamed.str_0 = str;
            return 268 as libc::c_int;
        }
        3511970825001807004 => return c,
        _ => {}
    }
    if ferror(yyfp) != 0 {
        yyerror(b"input error reading config\0" as *const u8 as *const libc::c_char);
    }
    return 0 as libc::c_int;
}
static mut yytranslate: [yytype_int8; 269] = [
    0 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    15 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    16 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    9 as libc::c_int as yytype_int8,
    10 as libc::c_int as yytype_int8,
    11 as libc::c_int as yytype_int8,
    12 as libc::c_int as yytype_int8,
    13 as libc::c_int as yytype_int8,
];
static mut yypact: [yytype_int8; 33] = [
    0 as libc::c_int as yytype_int8,
    -(2 as libc::c_int) as yytype_int8,
    2 as libc::c_int as yytype_int8,
    -(5 as libc::c_int) as yytype_int8,
    -(5 as libc::c_int) as yytype_int8,
    -(5 as libc::c_int) as yytype_int8,
    -(5 as libc::c_int) as yytype_int8,
    -(5 as libc::c_int) as yytype_int8,
    1 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    -(5 as libc::c_int) as yytype_int8,
    -(5 as libc::c_int) as yytype_int8,
    8 as libc::c_int as yytype_int8,
    -(5 as libc::c_int) as yytype_int8,
    -(5 as libc::c_int) as yytype_int8,
    -(5 as libc::c_int) as yytype_int8,
    -(5 as libc::c_int) as yytype_int8,
    6 as libc::c_int as yytype_int8,
    -(5 as libc::c_int) as yytype_int8,
    9 as libc::c_int as yytype_int8,
    13 as libc::c_int as yytype_int8,
    -(5 as libc::c_int) as yytype_int8,
    -(5 as libc::c_int) as yytype_int8,
    10 as libc::c_int as yytype_int8,
    -(5 as libc::c_int) as yytype_int8,
    4 as libc::c_int as yytype_int8,
    17 as libc::c_int as yytype_int8,
    -(5 as libc::c_int) as yytype_int8,
    -(5 as libc::c_int) as yytype_int8,
    -(5 as libc::c_int) as yytype_int8,
    -(5 as libc::c_int) as yytype_int8,
    12 as libc::c_int as yytype_int8,
];
static mut yydefact: [yytype_int8; 33] = [
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    9 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    18 as libc::c_int as yytype_int8,
    19 as libc::c_int as yytype_int8,
    11 as libc::c_int as yytype_int8,
    12 as libc::c_int as yytype_int8,
    13 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    10 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    21 as libc::c_int as yytype_int8,
    16 as libc::c_int as yytype_int8,
    20 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    23 as libc::c_int as yytype_int8,
    17 as libc::c_int as yytype_int8,
    15 as libc::c_int as yytype_int8,
    16 as libc::c_int as yytype_int8,
    22 as libc::c_int as yytype_int8,
    24 as libc::c_int as yytype_int8,
];
static mut yypgoto: [yytype_int8; 11] = [
    -(5 as libc::c_int) as yytype_int8,
    -(5 as libc::c_int) as yytype_int8,
    -(5 as libc::c_int) as yytype_int8,
    -(5 as libc::c_int) as yytype_int8,
    -(5 as libc::c_int) as yytype_int8,
    -(5 as libc::c_int) as yytype_int8,
    -(4 as libc::c_int) as yytype_int8,
    -(5 as libc::c_int) as yytype_int8,
    -(5 as libc::c_int) as yytype_int8,
    -(5 as libc::c_int) as yytype_int8,
    -(5 as libc::c_int) as yytype_int8,
];
static mut yydefgoto: [yytype_int8; 11] = [
    0 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    9 as libc::c_int as yytype_int8,
    10 as libc::c_int as yytype_int8,
    19 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    13 as libc::c_int as yytype_int8,
    21 as libc::c_int as yytype_int8,
    25 as libc::c_int as yytype_int8,
    31 as libc::c_int as yytype_int8,
];
static mut yytable: [yytype_int8; 27] = [
    -(2 as libc::c_int) as yytype_int8,
    1 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    -(2 as libc::c_int) as yytype_int8,
    -(2 as libc::c_int) as yytype_int8,
    5 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    15 as libc::c_int as yytype_int8,
    16 as libc::c_int as yytype_int8,
    17 as libc::c_int as yytype_int8,
    18 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    20 as libc::c_int as yytype_int8,
    -(2 as libc::c_int) as yytype_int8,
    11 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    28 as libc::c_int as yytype_int8,
    12 as libc::c_int as yytype_int8,
    24 as libc::c_int as yytype_int8,
    29 as libc::c_int as yytype_int8,
    22 as libc::c_int as yytype_int8,
    23 as libc::c_int as yytype_int8,
    27 as libc::c_int as yytype_int8,
    30 as libc::c_int as yytype_int8,
    28 as libc::c_int as yytype_int8,
    32 as libc::c_int as yytype_int8,
];
static mut yycheck: [yytype_int8; 27] = [
    0 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    9 as libc::c_int as yytype_int8,
    10 as libc::c_int as yytype_int8,
    11 as libc::c_int as yytype_int8,
    12 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    13 as libc::c_int as yytype_int8,
    13 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    16 as libc::c_int as yytype_int8,
    15 as libc::c_int as yytype_int8,
    13 as libc::c_int as yytype_int8,
    13 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    13 as libc::c_int as yytype_int8,
    30 as libc::c_int as yytype_int8,
];
static mut yystos: [yytype_int8; 33] = [
    0 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    18 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    19 as libc::c_int as yytype_int8,
    20 as libc::c_int as yytype_int8,
    21 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    13 as libc::c_int as yytype_int8,
    24 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    9 as libc::c_int as yytype_int8,
    10 as libc::c_int as yytype_int8,
    11 as libc::c_int as yytype_int8,
    12 as libc::c_int as yytype_int8,
    22 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    25 as libc::c_int as yytype_int8,
    15 as libc::c_int as yytype_int8,
    13 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    23 as libc::c_int as yytype_int8,
    13 as libc::c_int as yytype_int8,
    13 as libc::c_int as yytype_int8,
    16 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    27 as libc::c_int as yytype_int8,
    23 as libc::c_int as yytype_int8,
];
static mut yyr1: [yytype_int8; 25] = [
    0 as libc::c_int as yytype_int8,
    17 as libc::c_int as yytype_int8,
    18 as libc::c_int as yytype_int8,
    18 as libc::c_int as yytype_int8,
    18 as libc::c_int as yytype_int8,
    18 as libc::c_int as yytype_int8,
    19 as libc::c_int as yytype_int8,
    20 as libc::c_int as yytype_int8,
    20 as libc::c_int as yytype_int8,
    21 as libc::c_int as yytype_int8,
    21 as libc::c_int as yytype_int8,
    22 as libc::c_int as yytype_int8,
    22 as libc::c_int as yytype_int8,
    22 as libc::c_int as yytype_int8,
    22 as libc::c_int as yytype_int8,
    22 as libc::c_int as yytype_int8,
    23 as libc::c_int as yytype_int8,
    23 as libc::c_int as yytype_int8,
    24 as libc::c_int as yytype_int8,
    25 as libc::c_int as yytype_int8,
    25 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    27 as libc::c_int as yytype_int8,
    27 as libc::c_int as yytype_int8,
];
static mut yyr2: [yytype_int8; 25] = [
    0 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
];
unsafe extern "C" fn yydestruct(
    mut yymsg: *const libc::c_char,
    mut yykind: yysymbol_kind_t,
    mut yyvaluep: *mut yystype,
) {
    if yymsg.is_null() {
        yymsg = b"Deleting\0" as *const u8 as *const libc::c_char;
    }
}
#[no_mangle]
pub static mut yychar: libc::c_int = 0;
#[no_mangle]
pub static mut yylval: yystype = yystype {
    c2rust_unnamed: C2RustUnnamed {
        c2rust_unnamed: C2RustUnnamed_0 {
            action: 0,
            options: 0,
            cmd: 0 as *const libc::c_char,
            cmdargs: 0 as *const *const libc::c_char as *mut *const libc::c_char,
            envlist: 0 as *const *const libc::c_char as *mut *const libc::c_char,
        },
    },
    lineno: 0,
    colno: 0,
};
#[no_mangle]
pub static mut yynerrs: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn yyparse() -> libc::c_int {
    let mut current_block: u64;
    let mut yystate: yy_state_fast_t = 0 as libc::c_int;
    let mut yyerrstatus: libc::c_int = 0 as libc::c_int;
    let mut yystacksize: libc::c_long = 200 as libc::c_int as libc::c_long;
    let mut yyssa: [yy_state_t; 200] = [0; 200];
    let mut yyss: *mut yy_state_t = yyssa.as_mut_ptr();
    let mut yyssp: *mut yy_state_t = yyss;
    let mut yyvsa: [yystype; 200] = [yystype {
        c2rust_unnamed: C2RustUnnamed {
            c2rust_unnamed: C2RustUnnamed_0 {
                action: 0,
                options: 0,
                cmd: 0 as *const libc::c_char,
                cmdargs: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                envlist: 0 as *const *const libc::c_char as *mut *const libc::c_char,
            },
        },
        lineno: 0,
        colno: 0,
    }; 200];
    let mut yyvs: *mut yystype = yyvsa.as_mut_ptr();
    let mut yyvsp: *mut yystype = yyvs;
    let mut yyn: libc::c_int = 0;
    let mut yyresult: libc::c_int = 0;
    let mut yytoken: yysymbol_kind_t = YYSYMBOL_YYEMPTY;
    let mut yyval: yystype = yystype {
        c2rust_unnamed: C2RustUnnamed {
            c2rust_unnamed: C2RustUnnamed_0 {
                action: 0,
                options: 0,
                cmd: 0 as *const libc::c_char,
                cmdargs: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                envlist: 0 as *const *const libc::c_char as *mut *const libc::c_char,
            },
        },
        lineno: 0,
        colno: 0,
    };
    let mut yylen: libc::c_int = 0 as libc::c_int;
    yychar = -(2 as libc::c_int);
    's_46: loop {
        (0 as libc::c_int != 0
            && (0 as libc::c_int <= yystate && yystate < 33 as libc::c_int))
            as libc::c_int;
        *yyssp = yystate as yy_state_t;
        if yyss.offset(yystacksize as isize).offset(-(1 as libc::c_int as isize))
            <= yyssp
        {
            let mut yysize: libc::c_long = yyssp.offset_from(yyss) as libc::c_long
                + 1 as libc::c_int as libc::c_long;
            if 10000 as libc::c_int as libc::c_long <= yystacksize {
                current_block = 15843522680380374007;
                break;
            }
            yystacksize *= 2 as libc::c_int as libc::c_long;
            if (10000 as libc::c_int as libc::c_long) < yystacksize {
                yystacksize = 10000 as libc::c_int as libc::c_long;
            }
            let mut yyss1: *mut yy_state_t = yyss;
            let mut yyptr: *mut yyalloc = malloc(
                (yystacksize
                    * (::core::mem::size_of::<yy_state_t>() as libc::c_ulong
                        as libc::c_long
                        + ::core::mem::size_of::<yystype>() as libc::c_ulong
                            as libc::c_long)
                    + (::core::mem::size_of::<yyalloc>() as libc::c_ulong as libc::c_long
                        - 1 as libc::c_int as libc::c_long)) as libc::c_ulong,
            ) as *mut yyalloc;
            if yyptr.is_null() {
                current_block = 15843522680380374007;
                break;
            }
            let mut yynewbytes: libc::c_long = 0;
            libc::memcpy(
                &mut (*yyptr).yyss_alloc as *mut yy_state_t as *mut libc::c_void,
                yyss as *const libc::c_void,
                (yysize as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<yy_state_t>() as libc::c_ulong)
                    as libc::size_t,
            );
            yyss = &mut (*yyptr).yyss_alloc;
            yynewbytes = yystacksize
                * ::core::mem::size_of::<yy_state_t>() as libc::c_ulong as libc::c_long
                + (::core::mem::size_of::<yyalloc>() as libc::c_ulong as libc::c_long
                    - 1 as libc::c_int as libc::c_long);
            yyptr = yyptr
                .offset(
                    (yynewbytes
                        / ::core::mem::size_of::<yyalloc>() as libc::c_ulong
                            as libc::c_long) as isize,
                );
            let mut yynewbytes_0: libc::c_long = 0;
            libc::memcpy(
                &mut (*yyptr).yyvs_alloc as *mut yystype as *mut libc::c_void,
                yyvs as *const libc::c_void,
                (yysize as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<yystype>() as libc::c_ulong)
                    as libc::size_t,
            );
            yyvs = &mut (*yyptr).yyvs_alloc;
            yynewbytes_0 = yystacksize
                * ::core::mem::size_of::<yystype>() as libc::c_ulong as libc::c_long
                + (::core::mem::size_of::<yyalloc>() as libc::c_ulong as libc::c_long
                    - 1 as libc::c_int as libc::c_long);
            yyptr = yyptr
                .offset(
                    (yynewbytes_0
                        / ::core::mem::size_of::<yyalloc>() as libc::c_ulong
                            as libc::c_long) as isize,
                );
            if yyss1 != yyssa.as_mut_ptr() {
                free(yyss1 as *mut libc::c_void);
            }
            yyssp = yyss.offset(yysize as isize).offset(-(1 as libc::c_int as isize));
            yyvsp = yyvs.offset(yysize as isize).offset(-(1 as libc::c_int as isize));
            if yyss.offset(yystacksize as isize).offset(-(1 as libc::c_int as isize))
                <= yyssp
            {
                current_block = 8552535143260915578;
                break;
            }
        }
        if yystate == 4 as libc::c_int {
            yyresult = 0 as libc::c_int;
            current_block = 11995758950903471652;
            break;
        } else {
            yyn = yypact[yystate as usize] as libc::c_int;
            if yyn == -(5 as libc::c_int) {
                current_block = 9900520664742359944;
            } else {
                if yychar == -(2 as libc::c_int) {
                    yychar = yylex();
                }
                if yychar <= 0 as libc::c_int {
                    yychar = 0 as libc::c_int;
                    yytoken = YYSYMBOL_YYEOF;
                    current_block = 6174974146017752131;
                } else if yychar == 256 as libc::c_int {
                    yychar = 257 as libc::c_int;
                    yytoken = YYSYMBOL_YYerror;
                    current_block = 8771130512950243840;
                } else {
                    yytoken = (if 0 as libc::c_int <= yychar
                        && yychar <= 268 as libc::c_int
                    {
                        yytranslate[yychar as usize] as yysymbol_kind_t as libc::c_int
                    } else {
                        YYSYMBOL_YYUNDEF as libc::c_int
                    }) as yysymbol_kind_t;
                    current_block = 6174974146017752131;
                }
                match current_block {
                    8771130512950243840 => {}
                    _ => {
                        yyn += yytoken as libc::c_int;
                        if yyn < 0 as libc::c_int || (26 as libc::c_int) < yyn
                            || yycheck[yyn as usize] as libc::c_int
                                != yytoken as libc::c_int
                        {
                            current_block = 9900520664742359944;
                        } else {
                            yyn = yytable[yyn as usize] as libc::c_int;
                            if yyn <= 0 as libc::c_int {
                                yyn = -yyn;
                                current_block = 12883057250296485142;
                            } else {
                                if yyerrstatus != 0 {
                                    yyerrstatus -= 1;
                                    yyerrstatus;
                                }
                                yystate = yyn;
                                yyvsp = yyvsp.offset(1);
                                *yyvsp = yylval;
                                yychar = -(2 as libc::c_int);
                                current_block = 4840081864679457396;
                            }
                        }
                    }
                }
            }
            match current_block {
                9900520664742359944 => {
                    yyn = yydefact[yystate as usize] as libc::c_int;
                    if yyn == 0 as libc::c_int {
                        yytoken = (if yychar == -(2 as libc::c_int) {
                            YYSYMBOL_YYEMPTY as libc::c_int
                        } else if 0 as libc::c_int <= yychar
                            && yychar <= 268 as libc::c_int
                        {
                            yytranslate[yychar as usize] as yysymbol_kind_t
                                as libc::c_int
                        } else {
                            YYSYMBOL_YYUNDEF as libc::c_int
                        }) as yysymbol_kind_t;
                        if yyerrstatus == 0 {
                            yynerrs += 1;
                            yynerrs;
                            yyerror(
                                b"syntax error\0" as *const u8 as *const libc::c_char,
                            );
                        }
                        if yyerrstatus == 3 as libc::c_int {
                            if yychar <= 0 as libc::c_int {
                                if yychar == 0 as libc::c_int {
                                    current_block = 8552535143260915578;
                                    break;
                                }
                            } else {
                                yydestruct(
                                    b"Error: discarding\0" as *const u8 as *const libc::c_char,
                                    yytoken,
                                    &mut yylval,
                                );
                                yychar = -(2 as libc::c_int);
                            }
                        }
                        current_block = 8771130512950243840;
                    } else {
                        current_block = 12883057250296485142;
                    }
                }
                _ => {}
            }
            match current_block {
                12883057250296485142 => {
                    yylen = yyr2[yyn as usize] as libc::c_int;
                    yyval = *yyvsp.offset((1 as libc::c_int - yylen) as isize);
                    match yyn {
                        6 => {
                            let mut r: *mut rule = 0 as *mut rule;
                            r = calloc(
                                1 as libc::c_int as libc::c_ulong,
                                ::core::mem::size_of::<rule>() as libc::c_ulong,
                            ) as *mut rule;
                            if r.is_null() {
                                errx(
                                    1 as libc::c_int,
                                    b"can't allocate rule\0" as *const u8 as *const libc::c_char,
                                );
                            }
                            (*r)
                                .action = (*yyvsp.offset(-(3 as libc::c_int) as isize))
                                .c2rust_unnamed
                                .c2rust_unnamed
                                .action;
                            (*r)
                                .options = (*yyvsp.offset(-(3 as libc::c_int) as isize))
                                .c2rust_unnamed
                                .c2rust_unnamed
                                .options;
                            (*r)
                                .envlist = (*yyvsp.offset(-(3 as libc::c_int) as isize))
                                .c2rust_unnamed
                                .c2rust_unnamed
                                .envlist;
                            (*r)
                                .ident = (*yyvsp.offset(-(2 as libc::c_int) as isize))
                                .c2rust_unnamed
                                .str_0;
                            (*r)
                                .target = (*yyvsp.offset(-(1 as libc::c_int) as isize))
                                .c2rust_unnamed
                                .str_0;
                            (*r)
                                .cmd = (*yyvsp.offset(0 as libc::c_int as isize))
                                .c2rust_unnamed
                                .c2rust_unnamed
                                .cmd;
                            (*r)
                                .cmdargs = (*yyvsp.offset(0 as libc::c_int as isize))
                                .c2rust_unnamed
                                .c2rust_unnamed
                                .cmdargs;
                            if nrules == maxrules {
                                if maxrules == 0 as libc::c_int as libc::c_ulong {
                                    maxrules = 32 as libc::c_int as size_t;
                                } else {
                                    maxrules = (maxrules as libc::c_ulong)
                                        .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t
                                        as size_t;
                                }
                                rules = reallocarray(
                                    rules as *mut libc::c_void,
                                    maxrules,
                                    ::core::mem::size_of::<*mut rule>() as libc::c_ulong,
                                ) as *mut *mut rule;
                                if rules.is_null() {
                                    errx(
                                        1 as libc::c_int,
                                        b"can't allocate rules\0" as *const u8
                                            as *const libc::c_char,
                                    );
                                }
                            }
                            let fresh1 = nrules;
                            nrules = nrules.wrapping_add(1);
                            let ref mut fresh2 = *rules.offset(fresh1 as isize);
                            *fresh2 = r;
                            current_block = 7370318721998929769;
                        }
                        7 => {
                            yyval
                                .c2rust_unnamed
                                .c2rust_unnamed
                                .action = 1 as libc::c_int;
                            yyval
                                .c2rust_unnamed
                                .c2rust_unnamed
                                .options = (*yyvsp.offset(0 as libc::c_int as isize))
                                .c2rust_unnamed
                                .c2rust_unnamed
                                .options;
                            yyval
                                .c2rust_unnamed
                                .c2rust_unnamed
                                .envlist = (*yyvsp.offset(0 as libc::c_int as isize))
                                .c2rust_unnamed
                                .c2rust_unnamed
                                .envlist;
                            current_block = 7370318721998929769;
                        }
                        8 => {
                            yyval
                                .c2rust_unnamed
                                .c2rust_unnamed
                                .action = 2 as libc::c_int;
                            yyval
                                .c2rust_unnamed
                                .c2rust_unnamed
                                .options = 0 as libc::c_int;
                            yyval
                                .c2rust_unnamed
                                .c2rust_unnamed
                                .envlist = 0 as *mut *const libc::c_char;
                            current_block = 7370318721998929769;
                        }
                        9 => {
                            yyval
                                .c2rust_unnamed
                                .c2rust_unnamed
                                .options = 0 as libc::c_int;
                            yyval
                                .c2rust_unnamed
                                .c2rust_unnamed
                                .envlist = 0 as *mut *const libc::c_char;
                            current_block = 7370318721998929769;
                        }
                        10 => {
                            yyval
                                .c2rust_unnamed
                                .c2rust_unnamed
                                .options = (*yyvsp.offset(-(1 as libc::c_int) as isize))
                                .c2rust_unnamed
                                .c2rust_unnamed
                                .options
                                | (*yyvsp.offset(0 as libc::c_int as isize))
                                    .c2rust_unnamed
                                    .c2rust_unnamed
                                    .options;
                            yyval
                                .c2rust_unnamed
                                .c2rust_unnamed
                                .envlist = (*yyvsp.offset(-(1 as libc::c_int) as isize))
                                .c2rust_unnamed
                                .c2rust_unnamed
                                .envlist;
                            if yyval.c2rust_unnamed.c2rust_unnamed.options
                                & (0x1 as libc::c_int | 0x4 as libc::c_int)
                                == 0x1 as libc::c_int | 0x4 as libc::c_int
                            {
                                yyerror(
                                    b"can't combine nopass and persist\0" as *const u8
                                        as *const libc::c_char,
                                );
                                current_block = 5832582820025303349;
                            } else {
                                if !((*yyvsp.offset(0 as libc::c_int as isize))
                                    .c2rust_unnamed
                                    .c2rust_unnamed
                                    .envlist)
                                    .is_null()
                                {
                                    if !(yyval.c2rust_unnamed.c2rust_unnamed.envlist).is_null()
                                    {
                                        yyerror(
                                            b"can't have two setenv sections\0" as *const u8
                                                as *const libc::c_char,
                                        );
                                        current_block = 5832582820025303349;
                                    } else {
                                        yyval
                                            .c2rust_unnamed
                                            .c2rust_unnamed
                                            .envlist = (*yyvsp.offset(0 as libc::c_int as isize))
                                            .c2rust_unnamed
                                            .c2rust_unnamed
                                            .envlist;
                                        current_block = 517042441694919077;
                                    }
                                } else {
                                    current_block = 517042441694919077;
                                }
                                match current_block {
                                    5832582820025303349 => {}
                                    _ => {
                                        current_block = 7370318721998929769;
                                    }
                                }
                            }
                            match current_block {
                                7370318721998929769 => {}
                                _ => {
                                    yynerrs += 1;
                                    yynerrs;
                                    yyvsp = yyvsp.offset(-(yylen as isize));
                                    yyssp = yyssp.offset(-(yylen as isize));
                                    yylen = 0 as libc::c_int;
                                    yystate = *yyssp as yy_state_fast_t;
                                    current_block = 8771130512950243840;
                                }
                            }
                        }
                        11 => {
                            yyval
                                .c2rust_unnamed
                                .c2rust_unnamed
                                .options = 0x1 as libc::c_int;
                            yyval
                                .c2rust_unnamed
                                .c2rust_unnamed
                                .envlist = 0 as *mut *const libc::c_char;
                            current_block = 7370318721998929769;
                        }
                        12 => {
                            yyval
                                .c2rust_unnamed
                                .c2rust_unnamed
                                .options = 0x8 as libc::c_int;
                            yyval
                                .c2rust_unnamed
                                .c2rust_unnamed
                                .envlist = 0 as *mut *const libc::c_char;
                            current_block = 7370318721998929769;
                        }
                        13 => {
                            yyval
                                .c2rust_unnamed
                                .c2rust_unnamed
                                .options = 0x4 as libc::c_int;
                            yyval
                                .c2rust_unnamed
                                .c2rust_unnamed
                                .envlist = 0 as *mut *const libc::c_char;
                            current_block = 7370318721998929769;
                        }
                        14 => {
                            yyval
                                .c2rust_unnamed
                                .c2rust_unnamed
                                .options = 0x2 as libc::c_int;
                            yyval
                                .c2rust_unnamed
                                .c2rust_unnamed
                                .envlist = 0 as *mut *const libc::c_char;
                            current_block = 7370318721998929769;
                        }
                        15 => {
                            yyval
                                .c2rust_unnamed
                                .c2rust_unnamed
                                .options = 0 as libc::c_int;
                            yyval
                                .c2rust_unnamed
                                .c2rust_unnamed
                                .envlist = (*yyvsp.offset(-(1 as libc::c_int) as isize))
                                .c2rust_unnamed
                                .strlist;
                            current_block = 7370318721998929769;
                        }
                        16 => {
                            yyval
                                .c2rust_unnamed
                                .strlist = calloc(
                                1 as libc::c_int as libc::c_ulong,
                                ::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                            ) as *mut *const libc::c_char;
                            if (yyval.c2rust_unnamed.strlist).is_null() {
                                errx(
                                    1 as libc::c_int,
                                    b"can't allocate strlist\0" as *const u8
                                        as *const libc::c_char,
                                );
                            }
                            current_block = 7370318721998929769;
                        }
                        17 => {
                            let mut nstr: libc::c_int = arraylen(
                                (*yyvsp.offset(-(1 as libc::c_int) as isize))
                                    .c2rust_unnamed
                                    .strlist,
                            ) as libc::c_int;
                            yyval
                                .c2rust_unnamed
                                .strlist = reallocarray(
                                (*yyvsp.offset(-(1 as libc::c_int) as isize))
                                    .c2rust_unnamed
                                    .strlist as *mut libc::c_void,
                                (nstr + 2 as libc::c_int) as size_t,
                                ::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                            ) as *mut *const libc::c_char;
                            if (yyval.c2rust_unnamed.strlist).is_null() {
                                errx(
                                    1 as libc::c_int,
                                    b"can't allocate strlist\0" as *const u8
                                        as *const libc::c_char,
                                );
                            }
                            let ref mut fresh3 = *(yyval.c2rust_unnamed.strlist)
                                .offset(nstr as isize);
                            *fresh3 = (*yyvsp.offset(0 as libc::c_int as isize))
                                .c2rust_unnamed
                                .str_0;
                            let ref mut fresh4 = *(yyval.c2rust_unnamed.strlist)
                                .offset((nstr + 1 as libc::c_int) as isize);
                            *fresh4 = 0 as *const libc::c_char;
                            current_block = 7370318721998929769;
                        }
                        18 => {
                            yyval
                                .c2rust_unnamed
                                .str_0 = (*yyvsp.offset(0 as libc::c_int as isize))
                                .c2rust_unnamed
                                .str_0;
                            current_block = 7370318721998929769;
                        }
                        19 => {
                            yyval.c2rust_unnamed.str_0 = 0 as *const libc::c_char;
                            current_block = 7370318721998929769;
                        }
                        20 => {
                            yyval
                                .c2rust_unnamed
                                .str_0 = (*yyvsp.offset(0 as libc::c_int as isize))
                                .c2rust_unnamed
                                .str_0;
                            current_block = 7370318721998929769;
                        }
                        21 => {
                            yyval
                                .c2rust_unnamed
                                .c2rust_unnamed
                                .cmd = 0 as *const libc::c_char;
                            yyval
                                .c2rust_unnamed
                                .c2rust_unnamed
                                .cmdargs = 0 as *mut *const libc::c_char;
                            current_block = 7370318721998929769;
                        }
                        22 => {
                            yyval
                                .c2rust_unnamed
                                .c2rust_unnamed
                                .cmd = (*yyvsp.offset(-(1 as libc::c_int) as isize))
                                .c2rust_unnamed
                                .str_0;
                            yyval
                                .c2rust_unnamed
                                .c2rust_unnamed
                                .cmdargs = (*yyvsp.offset(0 as libc::c_int as isize))
                                .c2rust_unnamed
                                .c2rust_unnamed
                                .cmdargs;
                            current_block = 7370318721998929769;
                        }
                        23 => {
                            yyval
                                .c2rust_unnamed
                                .c2rust_unnamed
                                .cmdargs = 0 as *mut *const libc::c_char;
                            current_block = 7370318721998929769;
                        }
                        24 => {
                            yyval
                                .c2rust_unnamed
                                .c2rust_unnamed
                                .cmdargs = (*yyvsp.offset(0 as libc::c_int as isize))
                                .c2rust_unnamed
                                .strlist;
                            current_block = 7370318721998929769;
                        }
                        _ => {
                            current_block = 7370318721998929769;
                        }
                    }
                    match current_block {
                        8771130512950243840 => {}
                        _ => {
                            yyvsp = yyvsp.offset(-(yylen as isize));
                            yyssp = yyssp.offset(-(yylen as isize));
                            yylen = 0 as libc::c_int;
                            yyvsp = yyvsp.offset(1);
                            *yyvsp = yyval;
                            let yylhs: libc::c_int = yyr1[yyn as usize] as libc::c_int
                                - 17 as libc::c_int;
                            let yyi: libc::c_int = yypgoto[yylhs as usize] as libc::c_int
                                + *yyssp as libc::c_int;
                            yystate = if 0 as libc::c_int <= yyi
                                && yyi <= 26 as libc::c_int
                                && yycheck[yyi as usize] as libc::c_int
                                    == *yyssp as libc::c_int
                            {
                                yytable[yyi as usize] as libc::c_int
                            } else {
                                yydefgoto[yylhs as usize] as libc::c_int
                            };
                            current_block = 4840081864679457396;
                        }
                    }
                }
                _ => {}
            }
            match current_block {
                8771130512950243840 => {
                    yyerrstatus = 3 as libc::c_int;
                    loop {
                        yyn = yypact[yystate as usize] as libc::c_int;
                        if !(yyn == -(5 as libc::c_int)) {
                            yyn += YYSYMBOL_YYerror as libc::c_int;
                            if 0 as libc::c_int <= yyn && yyn <= 26 as libc::c_int
                                && yycheck[yyn as usize] as libc::c_int
                                    == YYSYMBOL_YYerror as libc::c_int
                            {
                                yyn = yytable[yyn as usize] as libc::c_int;
                                if (0 as libc::c_int) < yyn {
                                    break;
                                }
                            }
                        }
                        if yyssp == yyss {
                            current_block = 8552535143260915578;
                            break 's_46;
                        }
                        yydestruct(
                            b"Error: popping\0" as *const u8 as *const libc::c_char,
                            yystos[yystate as usize] as yysymbol_kind_t,
                            yyvsp,
                        );
                        yyvsp = yyvsp.offset(-(1 as libc::c_int as isize));
                        yyssp = yyssp.offset(-(1 as libc::c_int as isize));
                        yystate = *yyssp as yy_state_fast_t;
                    }
                    yyvsp = yyvsp.offset(1);
                    *yyvsp = yylval;
                    yystate = yyn;
                }
                _ => {}
            }
            yyssp = yyssp.offset(1);
            yyssp;
        }
    }
    match current_block {
        15843522680380374007 => {
            yyerror(b"memory exhausted\0" as *const u8 as *const libc::c_char);
            yyresult = 2 as libc::c_int;
        }
        8552535143260915578 => {
            yyresult = 1 as libc::c_int;
        }
        _ => {}
    }
    if yychar != -(2 as libc::c_int) {
        yytoken = (if 0 as libc::c_int <= yychar && yychar <= 268 as libc::c_int {
            yytranslate[yychar as usize] as yysymbol_kind_t as libc::c_int
        } else {
            YYSYMBOL_YYUNDEF as libc::c_int
        }) as yysymbol_kind_t;
        yydestruct(
            b"Cleanup: discarding lookahead\0" as *const u8 as *const libc::c_char,
            yytoken,
            &mut yylval,
        );
    }
    yyvsp = yyvsp.offset(-(yylen as isize));
    yyssp = yyssp.offset(-(yylen as isize));
    while yyssp != yyss {
        yydestruct(
            b"Cleanup: popping\0" as *const u8 as *const libc::c_char,
            yystos[*yyssp as libc::c_int as usize] as yysymbol_kind_t,
            yyvsp,
        );
        yyvsp = yyvsp.offset(-(1 as libc::c_int as isize));
        yyssp = yyssp.offset(-(1 as libc::c_int as isize));
    }
    if yyss != yyssa.as_mut_ptr() {
        free(yyss as *mut libc::c_void);
    }
    return yyresult;
}
