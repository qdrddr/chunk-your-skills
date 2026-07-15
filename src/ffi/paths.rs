//! Path configuration FFI exports.

use crate::ffi::error::{CYT_ERR_NULL_PTR, set_error};
use crate::ffi::json_util::{c_str_to_str, run_ffi, write_optional_string_out, write_string_result};
use crate::paths::{self, PathConfig};
use std::os::raw::{c_char, c_int};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cyt_configure_path_constants(
    md_ext: *const c_char,
    skills_decomposed_prefix: *const c_char,
    skills_decomposed_root: *const c_char,
    default_catalog_dir: *const c_char,
) -> c_int {
    run_ffi(|| {
        paths::configure(PathConfig {
            md_ext: c_str_to_str(md_ext, "md_ext")?.to_string(),
            skills_decomposed_prefix: c_str_to_str(
                skills_decomposed_prefix,
                "skills_decomposed_prefix",
            )?
            .to_string(),
            skills_decomposed_root: std::path::PathBuf::from(c_str_to_str(
                skills_decomposed_root,
                "skills_decomposed_root",
            )?),
            default_catalog_dir: std::path::PathBuf::from(c_str_to_str(
                default_catalog_dir,
                "default_catalog_dir",
            )?),
        });
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cyt_to_skills_decomposed_key(
    file_path: *const c_char,
    out: *mut *mut c_char,
) -> c_int {
    run_ffi(|| {
        if out.is_null() {
            set_error("null pointer: out");
            return Err(CYT_ERR_NULL_PTR);
        }
        let path = c_str_to_str(file_path, "file_path")?;
        unsafe { write_optional_string_out(paths::to_skills_decomposed_key(path), out)? };
        Ok(())
    })
}

macro_rules! path_getter {
    ($fn:ident, $body:expr) => {
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn $fn(out: *mut *mut c_char) -> c_int {
            crate::ffi::json_util::run_ffi(|| {
                if out.is_null() {
                    crate::ffi::error::set_error("null pointer: out");
                    return Err(crate::ffi::error::CYT_ERR_NULL_PTR);
                }
                unsafe {
                    crate::ffi::json_util::write_string_result(&$body, out)?;
                }
                Ok(())
            })
        }
    };
}

path_getter!(cyt_path_md_ext, paths::md_ext());
path_getter!(
    cyt_path_skills_decomposed_prefix,
    paths::skills_decomposed_prefix()
);
path_getter!(
    cyt_path_skills_decomposed_root,
    paths::skills_decomposed_root().to_string_lossy()
);
path_getter!(
    cyt_path_default_catalog_dir,
    paths::default_catalog_dir().to_string_lossy()
);
