extern crate gcc;
extern crate libc;

use std::env;
use std::mem::size_of;

fn main() {
    gcc::Config::new()
        .file("xdelta3.c")
        .flag("-std=c99")
        .flag(&format!("-DSIZEOF_SIZE_T={}", size_of::<libc::size_t>()))
        .flag(&format!("-DSIZEOF_UNSIGNED_INT={}", size_of::<libc::c_uint>()))
        .flag(&format!("-DSIZEOF_UNSIGNED_LONG={}", size_of::<libc::c_ulong>()))
        .flag(&format!("-DSIZEOF_UNSIGNED_LONG_LONG={}", size_of::<libc::c_ulonglong>()))
        .compile("libxdelta3.a");
    println!("cargo:root={}", env::var("OUT_DIR").unwrap());
}
