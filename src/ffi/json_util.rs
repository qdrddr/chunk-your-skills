//! JSON parsing and FFI panic guards.

use crate::ffi::error::{ERR_INVALID_UTF8, ERR_JSON, ERR_PANIC, OK, clear_error, set_error};
use crate::ffi::memory::write_string_out;
use serde_json::Value;
use std::ffi::CStr;
use std::os::raw::{c_char, c_int};
use std::panic::catch_unwind;
use std::ptr;

pub unsafe fn c_str_to_str<'a>(ptr: *const c_char, name: &str) -> Result<&'a str, c_int> {
    if ptr.is_null() {
        set_error(&format!("null pointer: {name}"));
        return Err(crate::ffi::error::ERR_NULL_PTR);
    }
    match CStr::from_ptr(ptr).to_str() {
        Ok(s) => Ok(s),
        Err(e) => {
            set_error(&format!("invalid UTF-8 in {name}: {e}"));
            Err(ERR_INVALID_UTF8)
        }
    }
}

pub unsafe fn parse_json_cstr(ptr: *const c_char, name: &str) -> Result<Value, c_int> {
    let s = c_str_to_str(ptr, name)?;
    serde_json::from_str(s).map_err(|e| {
        set_error(&format!("JSON parse error in {name}: {e}"));
        ERR_JSON
    })
}

pub unsafe fn write_json_out(value: &Value, out: *mut *mut c_char) -> Result<(), c_int> {
    if out.is_null() {
        set_error("null pointer: out");
        return Err(crate::ffi::error::ERR_NULL_PTR);
    }
    match serde_json::to_string(value) {
        Ok(s) => {
            let code = write_string_out(s.as_str(), out);
            if code == OK { Ok(()) } else { Err(code) }
        }
        Err(e) => {
            set_error(&format!("JSON serialize error: {e}"));
            *out = ptr::null_mut();
            Err(ERR_JSON)
        }
    }
}

pub unsafe fn write_optional_string_out(
    value: Option<String>,
    out: *mut *mut c_char,
) -> Result<(), c_int> {
    if let Some(s) = value {
        let code = write_string_out(&s, out);
        if code == OK { Ok(()) } else { Err(code) }
    } else {
        *out = ptr::null_mut();
        clear_error();
        Ok(())
    }
}

pub unsafe fn write_string_result(s: &str, out: *mut *mut c_char) -> Result<(), c_int> {
    let code = write_string_out(s, out);
    if code == OK { Ok(()) } else { Err(code) }
}

pub fn run_ffi<F>(f: F) -> c_int
where
    F: FnOnce() -> Result<(), c_int> + std::panic::UnwindSafe,
{
    match ffi_guard(f) {
        Ok(()) => OK,
        Err(code) => code,
    }
}

pub fn json_array_or_empty(val: &Value) -> Vec<Value> {
    val.as_array().cloned().unwrap_or_default()
}

pub fn ffi_guard<F, T>(f: F) -> Result<T, c_int>
where
    F: FnOnce() -> Result<T, c_int> + std::panic::UnwindSafe,
{
    catch_unwind(f).unwrap_or_else(|_| {
        set_error("internal panic at FFI boundary");
        Err(ERR_PANIC)
    })
}
