extern crate gcc;

use std::env;
use std::mem::size_of;
use std::os::raw;

fn main() {
    gcc::Config::new()
        .file("xdelta3/xdelta3.c")
        .flag("-Wall")
        .flag("-Wshadow")
        .flag("-fno-builtin")
        .flag("-Wextra")
        .flag("-Wsign-compare")
        .flag("-Wformat=2")
        .flag("-Wno-format-nonliteral")
        .flag("-Wno-unused-parameter")
        .flag("-Wno-unused-function")
        .flag("-pedantic")
        .flag("-std=c99")
        .flag(&format!("-DSIZEOF_SIZE_T={}", size_of::<usize>()))
        .flag(&format!("-DSIZEOF_UNSIGNED_INT={}", size_of::<raw::c_uint>()))
        .flag(&format!("-DSIZEOF_UNSIGNED_LONG={}", size_of::<raw::c_ulong>()))
        .flag(&format!("-DSIZEOF_UNSIGNED_LONG_LONG={}", size_of::<raw::c_ulonglong>()))
        .compile("libxdelta3.a");
    println!("cargo:root={}", env::var("OUT_DIR").unwrap());
}
