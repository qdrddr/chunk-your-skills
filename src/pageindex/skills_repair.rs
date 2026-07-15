use std::fs;
use std::path::Path;

use super::cache_layout::node_md_rel;
use super::config::PageIndexConfig;
use super::node_id::node_id_from_value;
use super::retrieve::strip_decomposed_frontmatter;
use super::tree::structure_to_list;
use super::types::SkillsIndex;
use crate::skills_io::load_skills_index_from_entry;

pub(crate) fn populate_structure_text_from_node_files(
    structure: &mut serde_json::Value,
    index: &SkillsIndex,
    _doc_id: &str,
) {
    populate_node_text(structure, index);
}

fn populate_node_text(structure: &mut serde_json::Value, index: &SkillsIndex) {
    match structure {
        serde_json::Value::Object(map) => {
            if map.contains_key("node_id") {
                let node_id = node_id_from_value(map.get("node_id"));
                let rel = node_md_rel(node_id);
                if let Some(raw) = index.files.get(&rel) {
                    let text = strip_decomposed_frontmatter(raw);
                    if !text.is_empty() {
                        map.insert("text".to_string(), serde_json::Value::String(text));
                    }
                }
            }
            if let Some(serde_json::Value::Array(children)) = map.get_mut("nodes") {
                for child in children {
                    populate_node_text(child, index);
                }
            }
        }
        serde_json::Value::Array(items) => {
            for item in items {
                populate_node_text(item, index);
            }
        }
        _ => {}
    }
}

fn node_file_exists(entry_dir: &Path, node_id: u32) -> bool {
    entry_dir.join(node_md_rel(node_id)).is_file()
}

fn attach_missing_nodes_to_structure(
    structure: &mut serde_json::Value,
    config: &PageIndexConfig,
    index: &mut SkillsIndex,
    doc_id: &str,
    entry_dir: &Path,
) -> Result<bool, String> {
    let mut changed = false;
    let nodes = structure_to_list(structure);
    for node in nodes {
        let Some(obj) = node.as_object() else {
            continue;
        };
        let node_id = node_id_from_value(obj.get("node_id"));
        if node_file_exists(entry_dir, node_id) {
            continue;
        }
        let line_num = obj
            .get("line_num")
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0);
        let title = obj.get("title").and_then(|v| v.as_str()).unwrap_or("");
        let text = obj.get("text").and_then(|v| v.as_str()).unwrap_or("");
        let body = if text.is_empty() {
            format!("# {title}\n")
        } else {
            text.to_string()
        };
        let token_count = crate::tiktoken::count_tokens_or_min(&body);
        let md_content = format!(
            "---\ndoc_id: {doc_id}\nnode_id: {node_id}\nline_num: {line_num}\ntoken_count: {token_count}\n---\n{body}",
        );
        let rel = node_md_rel(node_id);
        index.files.insert(rel.clone(), md_content.clone());
        let path = entry_dir.join(&rel);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        fs::write(&path, md_content).map_err(|e| e.to_string())?;
        changed = true;
    }
    let _ = config;
    Ok(changed)
}

/// Repair missing node markdown files for a cached skill entry.
///
/// # Errors
///
/// Returns an error when the entry directory is invalid or node files cannot be written.
pub fn repair_skill_nodes(
    entry_dir: &Path,
    doc_id: &str,
    config: &PageIndexConfig,
) -> Result<(), String> {
    let mut index = load_skills_index_from_entry(entry_dir, doc_id)?;
    let doc = index
        .documents
        .get(doc_id)
        .cloned()
        .ok_or_else(|| format!("skill document not found: {doc_id}"))?;
    let mut structure = doc.structure.clone();

    populate_structure_text_from_node_files(&mut structure, &index, doc_id);
    let changed = attach_missing_nodes_to_structure(
        &mut structure,
        config,
        &mut index,
        doc_id,
        entry_dir,
    )?;

    if !changed {
        return Ok(());
    }

    let mut updated = doc;
    updated.structure = structure;
    index.documents.insert(doc_id.to_string(), updated);
    Ok(())
}
