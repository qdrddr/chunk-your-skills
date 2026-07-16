//! Canonical manifest of chunk-your-skills C FFI exports.

/// One exported `chunk_your_skills_*` C symbol.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FfiExport {
    pub name: &'static str,
    pub category: &'static str,
}

/// All exported FFI symbols grouped by module category.
pub const EXPORTS: &[FfiExport] = &[
    // core / memory
    export("chunk_your_skills_clear_error", "core"),
    export("chunk_your_skills_free_string", "core"),
    export("chunk_your_skills_get_last_error", "core"),
    export("chunk_your_skills_get_version", "core"),
    // cache
    export("chunk_your_skills_configure_memory_cache", "cache"),
    export("chunk_your_skills_ensure_skills_registry", "cache"),
    // paths
    export("chunk_your_skills_configure_path_constants", "paths"),
    export("chunk_your_skills_to_skills_decomposed_key", "paths"),
    export("chunk_your_skills_path_md_ext", "paths"),
    export("chunk_your_skills_path_skills_decomposed_prefix", "paths"),
    export("chunk_your_skills_path_skills_decomposed_root", "paths"),
    export("chunk_your_skills_path_default_catalog_dir", "paths"),
    // pageindex / skills
    export("chunk_your_skills_build_skills_index", "pageindex"),
    export("chunk_your_skills_write_skills_index", "pageindex"),
    export("chunk_your_skills_load_skills_index_from_dir", "pageindex"),
    export("chunk_your_skills_repair_skill_nodes", "pageindex"),
    export(
        "chunk_your_skills_skills_index_from_decomposed_dir",
        "pageindex",
    ),
    export("chunk_your_skills_md_to_tree", "pageindex"),
    export("chunk_your_skills_get_skill_document", "pageindex"),
    export("chunk_your_skills_get_skill_structure", "pageindex"),
    export(
        "chunk_your_skills_get_skill_line_content_from_spec",
        "pageindex",
    ),
    export(
        "chunk_your_skills_get_skill_content_retrieve_result",
        "pageindex",
    ),
    export("chunk_your_skills_reconstruct_skill_markdown", "pageindex"),
    export("chunk_your_skills_write_reconstructed_skill", "pageindex"),
    export("chunk_your_skills_get_skill_line_content", "pageindex"),
    export(
        "chunk_your_skills_token_count_from_decomposed_frontmatter",
        "pageindex",
    ),
    export("chunk_your_skills_parse_frontmatter_fields", "pageindex"),
    export("chunk_your_skills_frontmatter_field", "pageindex"),
    export("chunk_your_skills_parse_skill_node_ids", "pageindex"),
    export("chunk_your_skills_skills_builder_new", "pageindex"),
    export("chunk_your_skills_skills_builder_free", "pageindex"),
    export(
        "chunk_your_skills_skills_builder_build_from_dirs",
        "pageindex",
    ),
    export(
        "chunk_your_skills_skills_builder_write_catalog",
        "pageindex",
    ),
    export(
        "chunk_your_skills_skills_builder_to_skills_index_json",
        "pageindex",
    ),
    export(
        "chunk_your_skills_skills_builder_to_skills_dict",
        "pageindex",
    ),
    export("chunk_your_skills_reconstruct_options_default", "pageindex"),
    export("chunk_your_skills_build_page_index_only", "pageindex"),
    export("chunk_your_skills_page_index_valid", "pageindex"),
    export(
        "chunk_your_skills_load_skills_index_from_entry",
        "pageindex",
    ),
    export(
        "chunk_your_skills_load_merged_skill_document_json",
        "pageindex",
    ),
    export(
        "chunk_your_skills_finalize_skill_document_json",
        "pageindex",
    ),
    export(
        "chunk_your_skills_update_skill_document_source_path",
        "pageindex",
    ),
];

const fn export(name: &'static str, category: &'static str) -> FfiExport {
    FfiExport { name, category }
}

/// Macro-generated symbols documented via cbindgen stubs (not visible to cbindgen in macro expansion).
pub const CBINDGEN_STUB_SYMBOLS: &[&str] = &[
    "chunk_your_skills_path_md_ext",
    "chunk_your_skills_path_skills_decomposed_prefix",
    "chunk_your_skills_path_skills_decomposed_root",
    "chunk_your_skills_path_default_catalog_dir",
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

    #[test]
    fn header_contains_all_exports() {
        let header = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/chunk_your_skills.h"));
        for exp in EXPORTS {
            assert!(
                header.contains(exp.name),
                "generated header missing export: {}",
                exp.name
            );
        }
    }

    #[test]
    fn no_legacy_prefix_in_generated_header() {
        let header = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/chunk_your_skills.h"));
        let legacy_fn = ['c', 'y', 't', '_'].iter().collect::<String>();
        let legacy_macro = ['C', 'Y', 'T', '_'].iter().collect::<String>();
        let legacy_ns = format!("namespace {}", ['c', 'y', 't'].iter().collect::<String>());
        assert!(
            !header.contains(&legacy_fn),
            "generated header must not contain legacy fn prefix"
        );
        assert!(
            !header.contains(&legacy_macro),
            "generated header must not contain legacy macro prefix"
        );
        assert!(
            !header.contains(&legacy_ns),
            "generated header must not use legacy namespace"
        );
    }
}
