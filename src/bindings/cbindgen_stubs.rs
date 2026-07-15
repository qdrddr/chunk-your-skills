//! C header stubs for path FFI symbols.
//!
//! Parsed by cbindgen only (`cbindgen.toml` `[parse] include`); not compiled into the library.
//! Implementations live in `ffi/paths.rs`.

use std::os::raw::{c_char, c_int};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cyt_path_md_ext(out: *mut *mut c_char) -> c_int {
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cyt_path_skills_decomposed_prefix(out: *mut *mut c_char) -> c_int {
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cyt_path_skills_decomposed_root(out: *mut *mut c_char) -> c_int {
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cyt_path_default_catalog_dir(out: *mut *mut c_char) -> c_int {
    0
}
