//! Skill pageindex: decompose SKILL.md into nodes and recompose skinny skills.
#![allow(
    clippy::pub_use,
    clippy::module_name_repetitions,
    clippy::multiple_crate_versions
)]

pub mod cache;
pub mod json_util;
pub mod pageindex;
pub mod paths;
pub mod skills_builder;
pub mod skills_io;

#[cfg(feature = "python")]
pub mod python;

#[cfg(feature = "node")]
pub mod node;

#[cfg(feature = "ffi")]
pub mod bindings;

#[cfg(feature = "ffi")]
pub mod ffi;

pub use pageindex::{
    EntryMetadata, MdIndexResult, PageIndexConfig, RETRIEVE_DIR, ReconstructOptions,
    ReconstructResult, SkillDocument, SkillsIndex, build_page_index_for_file, build_skills_index,
    finalize_entry_metadata as finalize_skill_document_json, frontmatter_field,
    get_content_retrieve_result as get_skill_content_retrieve_result,
    get_document as get_skill_document, get_document_structure as get_skill_structure,
    get_line_content as get_skill_line_content,
    get_line_content_from_spec as get_skill_line_content_from_spec,
    load_merged_document_json as load_merged_skill_document_json, md_to_tree,
    parse_frontmatter_fields, parse_line_nums as parse_skill_line_nums,
    parse_node_ids as parse_skill_node_ids, reconstruct_skill_markdown, repair_skill_nodes,
    retrieve_output_rel_path, token_count_from_decomposed_frontmatter, update_document_source_path,
    write_entry_metadata, write_reconstructed_skill,
};
pub use paths::{
    PathConfig, configure as configure_paths, skills_decomposed_prefix, snapshot as path_snapshot,
    to_skills_decomposed_key,
};
pub use skills_builder::SkillsBuilder;
pub use skills_io::{
    load_decomposed_files_for_index, load_skills_index_from_dir, resolve_doc_id,
    resolve_doc_id_from_skill_path, skills_index_from_decomposed_dir, write_skills_index,
};
