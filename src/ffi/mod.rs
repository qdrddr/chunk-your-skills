//! C FFI bindings for chunk-your-skills.
//!
//! All exported functions use `chunk_your_skills_*` naming and C-style return codes.
//! JSON outputs are written to `char**` out parameters; free with [`chunk_your_skills_free_string`].
#![allow(unsafe_op_in_unsafe_fn)]

mod cache;
mod error;
mod json_util;
mod memory;
mod pageindex;
mod paths;

pub use error::{
    ERR_ALLOC, ERR_INVALID_ARG, ERR_INVALID_HANDLE, ERR_INVALID_UTF8, ERR_IO, ERR_JSON,
    ERR_NULL_PTR, ERR_PANIC, OK, chunk_your_skills_clear_error, chunk_your_skills_get_last_error,
};
pub use memory::{chunk_your_skills_free_string, chunk_your_skills_get_version};

pub use cache::{
    chunk_your_skills_configure_memory_cache, chunk_your_skills_ensure_skills_registry,
};
pub use pageindex::{
    ChunkYourSkillsBuilder, chunk_your_skills_frontmatter_field,
    chunk_your_skills_parse_frontmatter_fields,
    chunk_your_skills_token_count_from_decomposed_frontmatter,
};
