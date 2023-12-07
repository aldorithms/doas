#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(c_variadic)]
#![feature(extern_types)]


extern crate libc;
pub mod compat {
pub mod closefrom;
pub mod errc;
pub mod execvpe;
pub mod getprogname;
pub mod reallocarray;
pub mod setprogname;
pub mod strlcat;
pub mod strlcpy;
pub mod strtonum;
pub mod verrc;
} // mod compat
pub mod doas;
pub mod env;
pub mod y_tab;
