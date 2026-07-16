use serde_json::Value;

use super::config::PageIndexConfig;
use super::document_json::{build_page_index_json_value, serialize_document_json};
use super::node_id::node_id_from_value;
use super::tree::structure_to_list;
use super::types::{SkillDocument, SkillsIndex, node_md_rel};

/// Write node-only page index and node markdown files into the index file map.
pub fn decompose_page_index(
    index: &mut SkillsIndex,
    doc: &SkillDocument,
    flat_structure: &Value,
    config: &PageIndexConfig,
) {
    let page_json = serialize_document_json(&build_page_index_json_value(doc)).unwrap_or_default();
    index
        .files
        .insert(super::cache_layout::page_index_rel().to_string(), page_json);

    write_node_markdown_files(index, doc, flat_structure, config);
}

fn write_node_markdown_files(
    index: &mut SkillsIndex,
    doc: &SkillDocument,
    flat_structure: &Value,
    _config: &PageIndexConfig,
) {
    let nodes = structure_to_list(flat_structure);
    for node in nodes {
        let Some(obj) = node.as_object() else {
            continue;
        };
        let node_id = node_id_from_value(obj.get("node_id"));
        let title = obj.get("title").and_then(|v| v.as_str()).unwrap_or("");
        let line_num = obj.get("line_num").and_then(Value::as_u64).unwrap_or(0);
        let text = obj.get("text").and_then(|v| v.as_str()).unwrap_or("");

        let body = if super::tree::is_frontmatter_node(obj) || super::tree::is_preamble_node(obj) {
            text.to_string()
        } else if text.is_empty() {
            format!("# {title}\n")
        } else {
            text.to_string()
        };

        let md_content = format!(
            "---\ndoc_id: {}\nnode_id: {node_id}\nline_num: {line_num}\ntoken_count:\n---\n{body}",
            doc.id
        );

        index.files.insert(node_md_rel(node_id), md_content);
    }
}
