#![cfg(feature = "ffi")]

use chunk_your_skills::ffi::{CYT_OK, cyt_count_tokens, cyt_free_string, cyt_get_version};
use std::ffi::CStr;
use std::os::raw::c_char;
use std::ptr;

const fn cstr(bytes: &'static [u8]) -> &'static CStr {
    // SAFETY: every `cstr(...)` literal in this module is nul-terminated.
    unsafe { CStr::from_bytes_with_nul_unchecked(bytes) }
}

unsafe fn read_out(out: *mut c_char) -> String {
    let s = unsafe { CStr::from_ptr(out).to_string_lossy().into_owned() };
    unsafe { cyt_free_string(out) };
    s
}

#[test]
fn count_tokens_smoke() {
    let text = cstr(b"hello world\0");
    let count = unsafe { cyt_count_tokens(text.as_ptr()) };
    assert!(count >= 1);
}

#[test]
fn get_version_smoke() {
    let mut out: *mut c_char = ptr::null_mut();
    let code = unsafe { cyt_get_version(ptr::addr_of_mut!(out)) };
    assert_eq!(code, CYT_OK);
    assert!(!out.is_null());
    let version = unsafe { read_out(out) };
    assert!(!version.is_empty());
}
