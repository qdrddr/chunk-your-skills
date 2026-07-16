//! Canonical manifest of chunk-your-skills C FFI exports.

/// One exported `cyt_*` C symbol.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FfiExport {
    pub name: &'static str,
    pub category: &'static str,
}

/// All exported FFI symbols grouped by module category.
pub const EXPORTS: &[FfiExport] = &[
    // core / memory
    export("cyt_clear_error", "core"),
    export("cyt_free_string", "core"),
    export("cyt_get_last_error", "core"),
    export("cyt_get_version", "core"),
    // tokens
    export("cyt_count_tokens", "tokens"),
    export("cyt_count_json_tokens", "tokens"),
    export("cyt_count_tokens_batch", "tokens"),
    export("cyt_configure_tokenizer_defaults", "tokens"),
    // cache
    export("cyt_configure_memory_cache", "cache"),
    export("cyt_ensure_skills_registry", "cache"),
    // paths
    export("cyt_configure_path_constants", "paths"),
    export("cyt_to_skills_decomposed_key", "paths"),
    export("cyt_path_md_ext", "paths"),
    export("cyt_path_skills_decomposed_prefix", "paths"),
    export("cyt_path_skills_decomposed_root", "paths"),
    export("cyt_path_default_catalog_dir", "paths"),
    // pageindex / skills
    export("cyt_build_skills_index", "pageindex"),
    export("cyt_write_skills_index", "pageindex"),
    export("cyt_load_skills_index_from_dir", "pageindex"),
    export("cyt_repair_skill_nodes", "pageindex"),
    export("cyt_skills_index_from_decomposed_dir", "pageindex"),
    export("cyt_md_to_tree", "pageindex"),
    export("cyt_get_skill_document", "pageindex"),
    export("cyt_get_skill_structure", "pageindex"),
    export("cyt_get_skill_line_content_from_spec", "pageindex"),
    export("cyt_get_skill_content_retrieve_result", "pageindex"),
    export("cyt_reconstruct_skill_markdown", "pageindex"),
    export("cyt_write_reconstructed_skill", "pageindex"),
    export("cyt_get_skill_line_content", "pageindex"),
    export("cyt_token_count_from_decomposed_frontmatter", "pageindex"),
    export("cyt_parse_frontmatter_fields", "pageindex"),
    export("cyt_frontmatter_field", "pageindex"),
    export("cyt_parse_skill_node_ids", "pageindex"),
    export("cyt_skills_builder_new", "pageindex"),
    export("cyt_skills_builder_free", "pageindex"),
    export("cyt_skills_builder_build_from_dirs", "pageindex"),
    export("cyt_skills_builder_write_catalog", "pageindex"),
    export("cyt_skills_builder_to_skills_index_json", "pageindex"),
    export("cyt_skills_builder_to_skills_dict", "pageindex"),
    export("cyt_reconstruct_options_default", "pageindex"),
    export("cyt_build_page_index_only", "pageindex"),
    export("cyt_page_index_valid", "pageindex"),
    export("cyt_load_skills_index_from_entry", "pageindex"),
    export("cyt_load_merged_skill_document_json", "pageindex"),
    export("cyt_finalize_skill_document_json", "pageindex"),
    export("cyt_update_skill_document_source_path", "pageindex"),
];

const fn export(name: &'static str, category: &'static str) -> FfiExport {
    FfiExport { name, category }
}

/// Macro-generated symbols documented via cbindgen stubs (not visible to cbindgen in macro expansion).
pub const CBINDGEN_STUB_SYMBOLS: &[&str] = &[
    "cyt_path_md_ext",
    "cyt_path_skills_decomposed_prefix",
    "cyt_path_skills_decomposed_root",
    "cyt_path_default_catalog_dir",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exports_are_unique() {
        let mut seen = std::collections::HashSet::new();
        for exp in EXPORTS {
            assert!(seen.insert(exp.name), "duplicate export: {}", exp.name);
        }
    }

    #[test]
    fn cbindgen_stubs_listed_in_exports() {
        for name in CBINDGEN_STUB_SYMBOLS {
            assert!(
                EXPORTS.iter().any(|e| e.name == *name),
                "stub symbol missing from EXPORTS: {name}"
            );
        }
    }
}
