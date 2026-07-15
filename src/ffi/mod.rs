//! C FFI bindings for chunk-your-skills.
//!
//! All exported functions use `cyt_*` naming and C-style return codes.
//! JSON outputs are written to `char**` out parameters; free with [`cyt_free_string`].
#![allow(unsafe_op_in_unsafe_fn)]

mod cache;
mod error;
mod json_util;
mod memory;
mod pageindex;
mod paths;
mod tokens;

pub use error::{
    CYT_ERR_ALLOC, CYT_ERR_INVALID_ARG, CYT_ERR_INVALID_HANDLE, CYT_ERR_INVALID_UTF8, CYT_ERR_IO,
    CYT_ERR_JSON, CYT_ERR_NULL_PTR, CYT_ERR_PANIC, CYT_OK, cyt_clear_error, cyt_get_last_error,
};
pub use memory::{cyt_free_string, cyt_get_version};

pub use cache::{cyt_configure_memory_cache, cyt_ensure_skills_registry};
pub use pageindex::CytSkillsBuilder;
pub use tokens::{
    cyt_configure_tokenizer_defaults, cyt_count_json_tokens, cyt_count_tokens,
    cyt_count_tokens_batch,
};
