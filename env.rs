use ::libc;
extern "C" {
    fn reallocarray(
        optr: *mut libc::c_void,
        nmemb: size_t,
        size: size_t,
    ) -> *mut libc::c_void;
    fn asprintf(
        __ptr: *mut *mut libc::c_char,
        __fmt: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn err(__status: libc::c_int, __format: *const libc::c_char, _: ...) -> !;
    static mut environ: *mut *mut libc::c_char;
}
pub type __u_int = libc::c_uint;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type u_int = __u_int;
pub type size_t = libc::c_ulong;
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
pub struct env {
    pub root: envtree,
    pub count: u_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct envtree {
    pub rbh_root: *mut envnode,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct envnode {
    pub node: C2RustUnnamed,
    pub key: *const libc::c_char,
    pub value: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub rbe_left: *mut envnode,
    pub rbe_right: *mut envnode,
    pub rbe_parent: *mut envnode,
    pub rbe_color: libc::c_int,
}
#[no_mangle]
pub unsafe extern "C" fn envcmp(
    mut a: *mut envnode,
    mut b: *mut envnode,
) -> libc::c_int {
    return strcmp((*a).key, (*b).key);
}
unsafe extern "C" fn envtree_RB_MINMAX(
    mut head: *mut envtree,
    mut val: libc::c_int,
) -> *mut envnode {
    let mut tmp: *mut envnode = (*head).rbh_root;
    let mut parent: *mut envnode = 0 as *mut envnode;
    while !tmp.is_null() {
        parent = tmp;
        if val < 0 as libc::c_int {
            tmp = (*tmp).node.rbe_left;
        } else {
            tmp = (*tmp).node.rbe_right;
        }
    }
    return parent;
}
unsafe extern "C" fn envtree_RB_NEXT(mut elm: *mut envnode) -> *mut envnode {
    if !((*elm).node.rbe_right).is_null() {
        elm = (*elm).node.rbe_right;
        while !((*elm).node.rbe_left).is_null() {
            elm = (*elm).node.rbe_left;
        }
    } else if !((*elm).node.rbe_parent).is_null()
        && elm == (*(*elm).node.rbe_parent).node.rbe_left
    {
        elm = (*elm).node.rbe_parent;
    } else {
        while !((*elm).node.rbe_parent).is_null()
            && elm == (*(*elm).node.rbe_parent).node.rbe_right
        {
            elm = (*elm).node.rbe_parent;
        }
        elm = (*elm).node.rbe_parent;
    }
    return elm;
}
unsafe extern "C" fn envtree_RB_INSERT_COLOR(
    mut head: *mut envtree,
    mut elm: *mut envnode,
) {
    let mut parent: *mut envnode = 0 as *mut envnode;
    let mut gparent: *mut envnode = 0 as *mut envnode;
    let mut tmp: *mut envnode = 0 as *mut envnode;
    loop {
        parent = (*elm).node.rbe_parent;
        if !(!parent.is_null() && (*parent).node.rbe_color == 1 as libc::c_int) {
            break;
        }
        gparent = (*parent).node.rbe_parent;
        if parent == (*gparent).node.rbe_left {
            tmp = (*gparent).node.rbe_right;
            if !tmp.is_null() && (*tmp).node.rbe_color == 1 as libc::c_int {
                (*tmp).node.rbe_color = 0 as libc::c_int;
                (*parent).node.rbe_color = 0 as libc::c_int;
                (*gparent).node.rbe_color = 1 as libc::c_int;
                elm = gparent;
            } else {
                if (*parent).node.rbe_right == elm {
                    tmp = (*parent).node.rbe_right;
                    (*parent).node.rbe_right = (*tmp).node.rbe_left;
                    if !((*parent).node.rbe_right).is_null() {
                        (*(*tmp).node.rbe_left).node.rbe_parent = parent;
                    }
                    (*tmp).node.rbe_parent = (*parent).node.rbe_parent;
                    if !((*tmp).node.rbe_parent).is_null() {
                        if parent == (*(*parent).node.rbe_parent).node.rbe_left {
                            (*(*parent).node.rbe_parent).node.rbe_left = tmp;
                        } else {
                            (*(*parent).node.rbe_parent).node.rbe_right = tmp;
                        }
                    } else {
                        (*head).rbh_root = tmp;
                    }
                    (*tmp).node.rbe_left = parent;
                    (*parent).node.rbe_parent = tmp;
                    !((*tmp).node.rbe_parent).is_null();
                    tmp = parent;
                    parent = elm;
                    elm = tmp;
                }
                (*parent).node.rbe_color = 0 as libc::c_int;
                (*gparent).node.rbe_color = 1 as libc::c_int;
                tmp = (*gparent).node.rbe_left;
                (*gparent).node.rbe_left = (*tmp).node.rbe_right;
                if !((*gparent).node.rbe_left).is_null() {
                    (*(*tmp).node.rbe_right).node.rbe_parent = gparent;
                }
                (*tmp).node.rbe_parent = (*gparent).node.rbe_parent;
                if !((*tmp).node.rbe_parent).is_null() {
                    if gparent == (*(*gparent).node.rbe_parent).node.rbe_left {
                        (*(*gparent).node.rbe_parent).node.rbe_left = tmp;
                    } else {
                        (*(*gparent).node.rbe_parent).node.rbe_right = tmp;
                    }
                } else {
                    (*head).rbh_root = tmp;
                }
                (*tmp).node.rbe_right = gparent;
                (*gparent).node.rbe_parent = tmp;
                !((*tmp).node.rbe_parent).is_null();
            }
        } else {
            tmp = (*gparent).node.rbe_left;
            if !tmp.is_null() && (*tmp).node.rbe_color == 1 as libc::c_int {
                (*tmp).node.rbe_color = 0 as libc::c_int;
                (*parent).node.rbe_color = 0 as libc::c_int;
                (*gparent).node.rbe_color = 1 as libc::c_int;
                elm = gparent;
            } else {
                if (*parent).node.rbe_left == elm {
                    tmp = (*parent).node.rbe_left;
                    (*parent).node.rbe_left = (*tmp).node.rbe_right;
                    if !((*parent).node.rbe_left).is_null() {
                        (*(*tmp).node.rbe_right).node.rbe_parent = parent;
                    }
                    (*tmp).node.rbe_parent = (*parent).node.rbe_parent;
                    if !((*tmp).node.rbe_parent).is_null() {
                        if parent == (*(*parent).node.rbe_parent).node.rbe_left {
                            (*(*parent).node.rbe_parent).node.rbe_left = tmp;
                        } else {
                            (*(*parent).node.rbe_parent).node.rbe_right = tmp;
                        }
                    } else {
                        (*head).rbh_root = tmp;
                    }
                    (*tmp).node.rbe_right = parent;
                    (*parent).node.rbe_parent = tmp;
                    !((*tmp).node.rbe_parent).is_null();
                    tmp = parent;
                    parent = elm;
                    elm = tmp;
                }
                (*parent).node.rbe_color = 0 as libc::c_int;
                (*gparent).node.rbe_color = 1 as libc::c_int;
                tmp = (*gparent).node.rbe_right;
                (*gparent).node.rbe_right = (*tmp).node.rbe_left;
                if !((*gparent).node.rbe_right).is_null() {
                    (*(*tmp).node.rbe_left).node.rbe_parent = gparent;
                }
                (*tmp).node.rbe_parent = (*gparent).node.rbe_parent;
                if !((*tmp).node.rbe_parent).is_null() {
                    if gparent == (*(*gparent).node.rbe_parent).node.rbe_left {
                        (*(*gparent).node.rbe_parent).node.rbe_left = tmp;
                    } else {
                        (*(*gparent).node.rbe_parent).node.rbe_right = tmp;
                    }
                } else {
                    (*head).rbh_root = tmp;
                }
                (*tmp).node.rbe_left = gparent;
                (*gparent).node.rbe_parent = tmp;
                !((*tmp).node.rbe_parent).is_null();
            }
        }
    }
    (*(*head).rbh_root).node.rbe_color = 0 as libc::c_int;
}
unsafe extern "C" fn envtree_RB_REMOVE(
    mut head: *mut envtree,
    mut elm: *mut envnode,
) -> *mut envnode {
    let mut current_block: u64;
    let mut child: *mut envnode = 0 as *mut envnode;
    let mut parent: *mut envnode = 0 as *mut envnode;
    let mut old: *mut envnode = elm;
    let mut color: libc::c_int = 0;
    if ((*elm).node.rbe_left).is_null() {
        child = (*elm).node.rbe_right;
        current_block = 7245201122033322888;
    } else if ((*elm).node.rbe_right).is_null() {
        child = (*elm).node.rbe_left;
        current_block = 7245201122033322888;
    } else {
        let mut left: *mut envnode = 0 as *mut envnode;
        elm = (*elm).node.rbe_right;
        loop {
            left = (*elm).node.rbe_left;
            if left.is_null() {
                break;
            }
            elm = left;
        }
        child = (*elm).node.rbe_right;
        parent = (*elm).node.rbe_parent;
        color = (*elm).node.rbe_color;
        if !child.is_null() {
            (*child).node.rbe_parent = parent;
        }
        if !parent.is_null() {
            if (*parent).node.rbe_left == elm {
                (*parent).node.rbe_left = child;
            } else {
                (*parent).node.rbe_right = child;
            }
        } else {
            (*head).rbh_root = child;
        }
        if (*elm).node.rbe_parent == old {
            parent = elm;
        }
        (*elm).node = (*old).node;
        if !((*old).node.rbe_parent).is_null() {
            if (*(*old).node.rbe_parent).node.rbe_left == old {
                (*(*old).node.rbe_parent).node.rbe_left = elm;
            } else {
                (*(*old).node.rbe_parent).node.rbe_right = elm;
            }
        } else {
            (*head).rbh_root = elm;
        }
        (*(*old).node.rbe_left).node.rbe_parent = elm;
        if !((*old).node.rbe_right).is_null() {
            (*(*old).node.rbe_right).node.rbe_parent = elm;
        }
        if !parent.is_null() {
            left = parent;
            loop {
                left = (*left).node.rbe_parent;
                if left.is_null() {
                    break;
                }
            }
        }
        current_block = 1152310764220518792;
    }
    match current_block {
        7245201122033322888 => {
            parent = (*elm).node.rbe_parent;
            color = (*elm).node.rbe_color;
            if !child.is_null() {
                (*child).node.rbe_parent = parent;
            }
            if !parent.is_null() {
                if (*parent).node.rbe_left == elm {
                    (*parent).node.rbe_left = child;
                } else {
                    (*parent).node.rbe_right = child;
                }
            } else {
                (*head).rbh_root = child;
            }
        }
        _ => {}
    }
    if color == 0 as libc::c_int {
        envtree_RB_REMOVE_COLOR(head, parent, child);
    }
    return old;
}
unsafe extern "C" fn envtree_RB_REMOVE_COLOR(
    mut head: *mut envtree,
    mut parent: *mut envnode,
    mut elm: *mut envnode,
) {
    let mut tmp: *mut envnode = 0 as *mut envnode;
    while (elm.is_null() || (*elm).node.rbe_color == 0 as libc::c_int)
        && elm != (*head).rbh_root
    {
        if (*parent).node.rbe_left == elm {
            tmp = (*parent).node.rbe_right;
            if (*tmp).node.rbe_color == 1 as libc::c_int {
                (*tmp).node.rbe_color = 0 as libc::c_int;
                (*parent).node.rbe_color = 1 as libc::c_int;
                tmp = (*parent).node.rbe_right;
                (*parent).node.rbe_right = (*tmp).node.rbe_left;
                if !((*parent).node.rbe_right).is_null() {
                    (*(*tmp).node.rbe_left).node.rbe_parent = parent;
                }
                (*tmp).node.rbe_parent = (*parent).node.rbe_parent;
                if !((*tmp).node.rbe_parent).is_null() {
                    if parent == (*(*parent).node.rbe_parent).node.rbe_left {
                        (*(*parent).node.rbe_parent).node.rbe_left = tmp;
                    } else {
                        (*(*parent).node.rbe_parent).node.rbe_right = tmp;
                    }
                } else {
                    (*head).rbh_root = tmp;
                }
                (*tmp).node.rbe_left = parent;
                (*parent).node.rbe_parent = tmp;
                !((*tmp).node.rbe_parent).is_null();
                tmp = (*parent).node.rbe_right;
            }
            if (((*tmp).node.rbe_left).is_null()
                || (*(*tmp).node.rbe_left).node.rbe_color == 0 as libc::c_int)
                && (((*tmp).node.rbe_right).is_null()
                    || (*(*tmp).node.rbe_right).node.rbe_color == 0 as libc::c_int)
            {
                (*tmp).node.rbe_color = 1 as libc::c_int;
                elm = parent;
                parent = (*elm).node.rbe_parent;
            } else {
                if ((*tmp).node.rbe_right).is_null()
                    || (*(*tmp).node.rbe_right).node.rbe_color == 0 as libc::c_int
                {
                    let mut oleft: *mut envnode = 0 as *mut envnode;
                    oleft = (*tmp).node.rbe_left;
                    if !oleft.is_null() {
                        (*oleft).node.rbe_color = 0 as libc::c_int;
                    }
                    (*tmp).node.rbe_color = 1 as libc::c_int;
                    oleft = (*tmp).node.rbe_left;
                    (*tmp).node.rbe_left = (*oleft).node.rbe_right;
                    if !((*tmp).node.rbe_left).is_null() {
                        (*(*oleft).node.rbe_right).node.rbe_parent = tmp;
                    }
                    (*oleft).node.rbe_parent = (*tmp).node.rbe_parent;
                    if !((*oleft).node.rbe_parent).is_null() {
                        if tmp == (*(*tmp).node.rbe_parent).node.rbe_left {
                            (*(*tmp).node.rbe_parent).node.rbe_left = oleft;
                        } else {
                            (*(*tmp).node.rbe_parent).node.rbe_right = oleft;
                        }
                    } else {
                        (*head).rbh_root = oleft;
                    }
                    (*oleft).node.rbe_right = tmp;
                    (*tmp).node.rbe_parent = oleft;
                    !((*oleft).node.rbe_parent).is_null();
                    tmp = (*parent).node.rbe_right;
                }
                (*tmp).node.rbe_color = (*parent).node.rbe_color;
                (*parent).node.rbe_color = 0 as libc::c_int;
                if !((*tmp).node.rbe_right).is_null() {
                    (*(*tmp).node.rbe_right).node.rbe_color = 0 as libc::c_int;
                }
                tmp = (*parent).node.rbe_right;
                (*parent).node.rbe_right = (*tmp).node.rbe_left;
                if !((*parent).node.rbe_right).is_null() {
                    (*(*tmp).node.rbe_left).node.rbe_parent = parent;
                }
                (*tmp).node.rbe_parent = (*parent).node.rbe_parent;
                if !((*tmp).node.rbe_parent).is_null() {
                    if parent == (*(*parent).node.rbe_parent).node.rbe_left {
                        (*(*parent).node.rbe_parent).node.rbe_left = tmp;
                    } else {
                        (*(*parent).node.rbe_parent).node.rbe_right = tmp;
                    }
                } else {
                    (*head).rbh_root = tmp;
                }
                (*tmp).node.rbe_left = parent;
                (*parent).node.rbe_parent = tmp;
                !((*tmp).node.rbe_parent).is_null();
                elm = (*head).rbh_root;
                break;
            }
        } else {
            tmp = (*parent).node.rbe_left;
            if (*tmp).node.rbe_color == 1 as libc::c_int {
                (*tmp).node.rbe_color = 0 as libc::c_int;
                (*parent).node.rbe_color = 1 as libc::c_int;
                tmp = (*parent).node.rbe_left;
                (*parent).node.rbe_left = (*tmp).node.rbe_right;
                if !((*parent).node.rbe_left).is_null() {
                    (*(*tmp).node.rbe_right).node.rbe_parent = parent;
                }
                (*tmp).node.rbe_parent = (*parent).node.rbe_parent;
                if !((*tmp).node.rbe_parent).is_null() {
                    if parent == (*(*parent).node.rbe_parent).node.rbe_left {
                        (*(*parent).node.rbe_parent).node.rbe_left = tmp;
                    } else {
                        (*(*parent).node.rbe_parent).node.rbe_right = tmp;
                    }
                } else {
                    (*head).rbh_root = tmp;
                }
                (*tmp).node.rbe_right = parent;
                (*parent).node.rbe_parent = tmp;
                !((*tmp).node.rbe_parent).is_null();
                tmp = (*parent).node.rbe_left;
            }
            if (((*tmp).node.rbe_left).is_null()
                || (*(*tmp).node.rbe_left).node.rbe_color == 0 as libc::c_int)
                && (((*tmp).node.rbe_right).is_null()
                    || (*(*tmp).node.rbe_right).node.rbe_color == 0 as libc::c_int)
            {
                (*tmp).node.rbe_color = 1 as libc::c_int;
                elm = parent;
                parent = (*elm).node.rbe_parent;
            } else {
                if ((*tmp).node.rbe_left).is_null()
                    || (*(*tmp).node.rbe_left).node.rbe_color == 0 as libc::c_int
                {
                    let mut oright: *mut envnode = 0 as *mut envnode;
                    oright = (*tmp).node.rbe_right;
                    if !oright.is_null() {
                        (*oright).node.rbe_color = 0 as libc::c_int;
                    }
                    (*tmp).node.rbe_color = 1 as libc::c_int;
                    oright = (*tmp).node.rbe_right;
                    (*tmp).node.rbe_right = (*oright).node.rbe_left;
                    if !((*tmp).node.rbe_right).is_null() {
                        (*(*oright).node.rbe_left).node.rbe_parent = tmp;
                    }
                    (*oright).node.rbe_parent = (*tmp).node.rbe_parent;
                    if !((*oright).node.rbe_parent).is_null() {
                        if tmp == (*(*tmp).node.rbe_parent).node.rbe_left {
                            (*(*tmp).node.rbe_parent).node.rbe_left = oright;
                        } else {
                            (*(*tmp).node.rbe_parent).node.rbe_right = oright;
                        }
                    } else {
                        (*head).rbh_root = oright;
                    }
                    (*oright).node.rbe_left = tmp;
                    (*tmp).node.rbe_parent = oright;
                    !((*oright).node.rbe_parent).is_null();
                    tmp = (*parent).node.rbe_left;
                }
                (*tmp).node.rbe_color = (*parent).node.rbe_color;
                (*parent).node.rbe_color = 0 as libc::c_int;
                if !((*tmp).node.rbe_left).is_null() {
                    (*(*tmp).node.rbe_left).node.rbe_color = 0 as libc::c_int;
                }
                tmp = (*parent).node.rbe_left;
                (*parent).node.rbe_left = (*tmp).node.rbe_right;
                if !((*parent).node.rbe_left).is_null() {
                    (*(*tmp).node.rbe_right).node.rbe_parent = parent;
                }
                (*tmp).node.rbe_parent = (*parent).node.rbe_parent;
                if !((*tmp).node.rbe_parent).is_null() {
                    if parent == (*(*parent).node.rbe_parent).node.rbe_left {
                        (*(*parent).node.rbe_parent).node.rbe_left = tmp;
                    } else {
                        (*(*parent).node.rbe_parent).node.rbe_right = tmp;
                    }
                } else {
                    (*head).rbh_root = tmp;
                }
                (*tmp).node.rbe_right = parent;
                (*parent).node.rbe_parent = tmp;
                !((*tmp).node.rbe_parent).is_null();
                elm = (*head).rbh_root;
                break;
            }
        }
    }
    if !elm.is_null() {
        (*elm).node.rbe_color = 0 as libc::c_int;
    }
}
unsafe extern "C" fn envtree_RB_FIND(
    mut head: *mut envtree,
    mut elm: *mut envnode,
) -> *mut envnode {
    let mut tmp: *mut envnode = (*head).rbh_root;
    let mut comp: libc::c_int = 0;
    while !tmp.is_null() {
        comp = envcmp(elm, tmp);
        if comp < 0 as libc::c_int {
            tmp = (*tmp).node.rbe_left;
        } else if comp > 0 as libc::c_int {
            tmp = (*tmp).node.rbe_right;
        } else {
            return tmp
        }
    }
    return 0 as *mut envnode;
}
unsafe extern "C" fn envtree_RB_INSERT(
    mut head: *mut envtree,
    mut elm: *mut envnode,
) -> *mut envnode {
    let mut tmp: *mut envnode = 0 as *mut envnode;
    let mut parent: *mut envnode = 0 as *mut envnode;
    let mut comp: libc::c_int = 0 as libc::c_int;
    tmp = (*head).rbh_root;
    while !tmp.is_null() {
        parent = tmp;
        comp = envcmp(elm, parent);
        if comp < 0 as libc::c_int {
            tmp = (*tmp).node.rbe_left;
        } else if comp > 0 as libc::c_int {
            tmp = (*tmp).node.rbe_right;
        } else {
            return tmp
        }
    }
    (*elm).node.rbe_parent = parent;
    (*elm).node.rbe_right = 0 as *mut envnode;
    (*elm).node.rbe_left = (*elm).node.rbe_right;
    (*elm).node.rbe_color = 1 as libc::c_int;
    if !parent.is_null() {
        if comp < 0 as libc::c_int {
            (*parent).node.rbe_left = elm;
        } else {
            (*parent).node.rbe_right = elm;
        }
    } else {
        (*head).rbh_root = elm;
    }
    envtree_RB_INSERT_COLOR(head, elm);
    return 0 as *mut envnode;
}
unsafe extern "C" fn createnode(
    mut key: *const libc::c_char,
    mut value: *const libc::c_char,
) -> *mut envnode {
    let mut node: *mut envnode = 0 as *mut envnode;
    node = malloc(::core::mem::size_of::<envnode>() as libc::c_ulong) as *mut envnode;
    if node.is_null() {
        err(1 as libc::c_int, 0 as *const libc::c_char);
    }
    (*node).key = strdup(key);
    (*node).value = strdup(value);
    if ((*node).key).is_null() || ((*node).value).is_null() {
        err(1 as libc::c_int, 0 as *const libc::c_char);
    }
    return node;
}
unsafe extern "C" fn freenode(mut node: *mut envnode) {
    free((*node).key as *mut libc::c_char as *mut libc::c_void);
    free((*node).value as *mut libc::c_char as *mut libc::c_void);
    free(node as *mut libc::c_void);
}
unsafe extern "C" fn addnode(
    mut env: *mut env,
    mut key: *const libc::c_char,
    mut value: *const libc::c_char,
) {
    let mut node: *mut envnode = 0 as *mut envnode;
    node = createnode(key, value);
    envtree_RB_INSERT(&mut (*env).root, node);
    (*env).count = ((*env).count).wrapping_add(1);
    (*env).count;
}
#[no_mangle]
pub unsafe extern "C" fn copyenvpw(mut my_static: *mut passwd) -> *mut passwd {
    let mut new_pw: *mut passwd = 0 as *mut passwd;
    if my_static.is_null() {
        return 0 as *mut passwd;
    }
    new_pw = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::core::mem::size_of::<passwd>() as libc::c_ulong,
    ) as *mut passwd;
    if new_pw.is_null() {
        return 0 as *mut passwd;
    }
    (*new_pw).pw_name = strdup((*my_static).pw_name);
    (*new_pw).pw_passwd = strdup((*my_static).pw_passwd);
    (*new_pw).pw_uid = (*my_static).pw_uid;
    (*new_pw).pw_gid = (*my_static).pw_gid;
    (*new_pw).pw_gecos = strdup((*my_static).pw_gecos);
    (*new_pw).pw_dir = strdup((*my_static).pw_dir);
    (*new_pw).pw_shell = strdup((*my_static).pw_shell);
    return new_pw;
}
unsafe extern "C" fn createenv(
    mut rule: *mut rule,
    mut original: *mut passwd,
    mut target: *mut passwd,
) -> *mut env {
    let mut env: *mut env = 0 as *mut env;
    let mut i: u_int = 0;
    env = malloc(::core::mem::size_of::<env>() as libc::c_ulong) as *mut env;
    if env.is_null() {
        err(1 as libc::c_int, 0 as *const libc::c_char);
    }
    (*env).root.rbh_root = 0 as *mut envnode;
    (*env).count = 0 as libc::c_int as u_int;
    addnode(
        env,
        b"DOAS_USER\0" as *const u8 as *const libc::c_char,
        (*original).pw_name,
    );
    if (*rule).options & 0x2 as libc::c_int != 0 {
        addnode(env, b"HOME\0" as *const u8 as *const libc::c_char, (*original).pw_dir);
    } else {
        addnode(env, b"HOME\0" as *const u8 as *const libc::c_char, (*target).pw_dir);
    }
    addnode(env, b"LOGNAME\0" as *const u8 as *const libc::c_char, (*target).pw_name);
    addnode(
        env,
        b"PATH\0" as *const u8 as *const libc::c_char,
        b"/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin\0" as *const u8
            as *const libc::c_char,
    );
    addnode(env, b"SHELL\0" as *const u8 as *const libc::c_char, (*target).pw_shell);
    addnode(env, b"USER\0" as *const u8 as *const libc::c_char, (*target).pw_name);
    if (*rule).options & 0x2 as libc::c_int != 0 {
        i = 0 as libc::c_int as u_int;
        while !(*environ.offset(i as isize)).is_null() {
            let mut node: *mut envnode = 0 as *mut envnode;
            let mut e: *const libc::c_char = 0 as *const libc::c_char;
            let mut eq: *const libc::c_char = 0 as *const libc::c_char;
            let mut len: size_t = 0;
            let mut name: [libc::c_char; 1024] = [0; 1024];
            e = *environ.offset(i as isize);
            eq = strchr(e, '=' as i32);
            if !(eq.is_null() || eq == e) {
                len = eq.offset_from(e) as libc::c_long as size_t;
                if !(len
                    > (::core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong))
                {
                    memcpy(
                        name.as_mut_ptr() as *mut libc::c_void,
                        e as *const libc::c_void,
                        len,
                    );
                    name[len as usize] = '\0' as i32 as libc::c_char;
                    node = createnode(
                        name.as_mut_ptr(),
                        eq.offset(1 as libc::c_int as isize),
                    );
                    if !(envtree_RB_INSERT(&mut (*env).root, node)).is_null() {
                        freenode(node);
                    } else {
                        (*env).count = ((*env).count).wrapping_add(1);
                        (*env).count;
                    }
                }
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return env;
}
unsafe extern "C" fn flattenenv(mut env: *mut env) -> *mut *mut libc::c_char {
    let mut envp: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut node: *mut envnode = 0 as *mut envnode;
    let mut i: u_int = 0;
    envp = reallocarray(
        0 as *mut libc::c_void,
        ((*env).count).wrapping_add(1 as libc::c_int as libc::c_uint) as size_t,
        ::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
    ) as *mut *mut libc::c_char;
    if envp.is_null() {
        err(1 as libc::c_int, 0 as *const libc::c_char);
    }
    i = 0 as libc::c_int as u_int;
    node = envtree_RB_MINMAX(&mut (*env).root, -(1 as libc::c_int));
    while !node.is_null() {
        if asprintf(
            &mut *envp.offset(i as isize) as *mut *mut libc::c_char,
            b"%s=%s\0" as *const u8 as *const libc::c_char,
            (*node).key,
            (*node).value,
        ) == -(1 as libc::c_int)
        {
            err(1 as libc::c_int, 0 as *const libc::c_char);
        }
        i = i.wrapping_add(1);
        i;
        node = envtree_RB_NEXT(node);
    }
    let ref mut fresh0 = *envp.offset(i as isize);
    *fresh0 = 0 as *mut libc::c_char;
    return envp;
}
unsafe extern "C" fn fillenv(mut env: *mut env, mut envlist: *mut *const libc::c_char) {
    let mut node: *mut envnode = 0 as *mut envnode;
    let mut key: envnode = envnode {
        node: C2RustUnnamed {
            rbe_left: 0 as *mut envnode,
            rbe_right: 0 as *mut envnode,
            rbe_parent: 0 as *mut envnode,
            rbe_color: 0,
        },
        key: 0 as *const libc::c_char,
        value: 0 as *const libc::c_char,
    };
    let mut e: *const libc::c_char = 0 as *const libc::c_char;
    let mut eq: *const libc::c_char = 0 as *const libc::c_char;
    let mut val: *const libc::c_char = 0 as *const libc::c_char;
    let mut name: [libc::c_char; 1024] = [0; 1024];
    let mut i: u_int = 0;
    let mut len: size_t = 0;
    i = 0 as libc::c_int as u_int;
    while !(*envlist.offset(i as isize)).is_null() {
        e = *envlist.offset(i as isize);
        eq = strchr(e, '=' as i32);
        if eq.is_null() {
            len = strlen(e);
        } else {
            len = eq.offset_from(e) as libc::c_long as size_t;
        }
        if !(len
            > (::core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong))
        {
            memcpy(
                name.as_mut_ptr() as *mut libc::c_void,
                e as *const libc::c_void,
                len,
            );
            name[len as usize] = '\0' as i32 as libc::c_char;
            key.key = name.as_mut_ptr();
            if *name.as_mut_ptr() as libc::c_int == '-' as i32 {
                key.key = name.as_mut_ptr().offset(1 as libc::c_int as isize);
            }
            node = envtree_RB_FIND(&mut (*env).root, &mut key);
            if !node.is_null() {
                envtree_RB_REMOVE(&mut (*env).root, node);
                freenode(node);
                (*env).count = ((*env).count).wrapping_sub(1);
                (*env).count;
            }
            if !(*name.as_mut_ptr() as libc::c_int == '-' as i32) {
                if !eq.is_null() {
                    val = eq.offset(1 as libc::c_int as isize);
                    if *val as libc::c_int == '$' as i32 {
                        val = getenv(val.offset(1 as libc::c_int as isize));
                    }
                } else {
                    val = getenv(name.as_mut_ptr());
                }
                if !val.is_null() {
                    node = createnode(name.as_mut_ptr(), val);
                    envtree_RB_INSERT(&mut (*env).root, node);
                    (*env).count = ((*env).count).wrapping_add(1);
                    (*env).count;
                }
            }
        }
        i = i.wrapping_add(1);
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn prepenv(
    mut rule: *mut rule,
    mut original: *mut passwd,
    mut target: *mut passwd,
) -> *mut *mut libc::c_char {
    static mut safeset: [*const libc::c_char; 3] = [
        b"DISPLAY\0" as *const u8 as *const libc::c_char,
        b"TERM\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut env: *mut env = 0 as *mut env;
    env = createenv(rule, original, target);
    if (*rule).options & 0x2 as libc::c_int == 0 {
        fillenv(env, safeset.as_mut_ptr());
    }
    if !((*rule).envlist).is_null() {
        fillenv(env, (*rule).envlist);
    }
    return flattenenv(env);
}
