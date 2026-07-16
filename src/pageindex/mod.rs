pub mod build;
pub mod cache_layout;
pub mod config;
pub mod decompose;
pub mod document_json;
pub mod index;
pub mod node_id;
pub mod parse;
pub use parse::{
    extract_skill_prefix, frontmatter_field, frontmatter_yaml_body, parse_frontmatter_fields,
};
pub mod reconstruct;
pub mod retrieve;
pub mod skills_repair;
#[cfg(any(feature = "python", feature = "node", feature = "ffi"))]
pub(crate) mod spec_refs;
pub mod tree;
pub mod types;

pub use build::{
    build_page_index_for_content, build_page_index_for_file, build_page_index_only,
    build_skills_index, page_index_valid, write_page_index_entry,
};
pub use cache_layout::{
    entry_content_hash, metadata_path, node_md_path, nodes_dir, page_index_path, skill_entry_dir,
};
pub use config::PageIndexConfig;
pub use document_json::{
    EntryMetadata, SkillDocumentOnDisk, build_page_index_json_value, entry_hash_matches,
    finalize_entry_metadata, load_merged_document_json, parse_document_on_disk, read_document_json,
    read_entry_metadata, serialize_document_json, shorten_home_path, update_document_source_path,
    write_entry_metadata, write_page_index_files,
};
pub use index::md_to_tree;
pub use node_id::{node_id_from_value, node_id_key, node_id_value, parse_node_id_token};
pub use reconstruct::{
    RETRIEVE_DIR, ReconstructOptions, ReconstructResult, get_content_retrieve_result,
    reconstruct_skill_markdown, retrieve_output_rel_path, write_reconstructed_skill,
};
pub use retrieve::{
    get_document, get_document_structure, get_line_content, get_line_content_from_spec,
    parse_line_nums, parse_node_ids, token_count_from_decomposed_frontmatter,
};
pub use skills_repair::repair_skill_nodes;
pub use tree::{
    CONTENT_NODE_ID_START, NODE_ID_FRONTMATTER, NODE_ID_PREAMBLE, NODE_KIND_FRONTMATTER,
    NODE_KIND_PREAMBLE, finalize_skill_structure, is_frontmatter_node, is_preamble_node,
};
pub use types::{MdIndexResult, SkillDocument, SkillsIndex, node_md_rel, page_index_rel};
