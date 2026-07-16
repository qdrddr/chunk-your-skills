#![cfg(feature = "ffi")]

use chunk_your_skills::ffi::{
    CYT_OK, cyt_count_tokens, cyt_free_string, cyt_frontmatter_field, cyt_get_version,
    cyt_parse_frontmatter_fields,
};
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

#[test]
fn parse_frontmatter_fields_smoke() {
    let frontmatter = cstr(b"---\nname: demo\ndescription: >-\n  hello world\n---\0");
    let mut out: *mut c_char = ptr::null_mut();
    let code =
        unsafe { cyt_parse_frontmatter_fields(frontmatter.as_ptr(), ptr::addr_of_mut!(out)) };
    assert_eq!(code, CYT_OK);
    assert!(!out.is_null());
    let json = unsafe { read_out(out) };
    assert!(json.contains("\"name\":\"demo\"") || json.contains("\"name\": \"demo\""));
    assert!(json.contains("hello world"));

    let key = cstr(b"description\0");
    let mut field_out: *mut c_char = ptr::null_mut();
    let code = unsafe {
        cyt_frontmatter_field(
            frontmatter.as_ptr(),
            key.as_ptr(),
            ptr::addr_of_mut!(field_out),
        )
    };
    assert_eq!(code, CYT_OK);
    assert!(!field_out.is_null());
    let field = unsafe { read_out(field_out) };
    assert!(field.contains("hello world"));
}
