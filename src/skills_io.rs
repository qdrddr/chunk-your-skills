use std::fs;
use std::path::Path;

use crate::pageindex::cache_layout::{NODES_DIR, nodes_dir, page_index_path, page_index_rel};
use crate::pageindex::document_json::{load_merged_document_from_entry, write_bytes_atomic};
use crate::pageindex::{SkillDocument, SkillsIndex};
use crate::paths::shorten_home_path;

/// Write node and page-index files from an in-memory index to `entry_dir`.
///
/// # Errors
///
/// Returns an error when directories or files cannot be created or written.
pub fn write_page_index_files(index: &SkillsIndex, entry_dir: &Path) -> Result<(), String> {
    fs::create_dir_all(entry_dir).map_err(|e| e.to_string())?;
    write_selected_files(index, entry_dir, |rel| {
        rel == page_index_rel() || rel.starts_with(&format!("{NODES_DIR}/"))
    })
}

/// Write all known index files (page index + node markdown).
///
/// # Errors
///
/// Returns an error when directories or files cannot be created or written.
pub fn write_skills_index(index: &SkillsIndex, output_dir: &Path) -> Result<(), String> {
    fs::create_dir_all(output_dir).map_err(|e| e.to_string())?;
    write_selected_files(index, output_dir, |_| true)
}

fn write_selected_files(
    index: &SkillsIndex,
    output_dir: &Path,
    include: impl Fn(&str) -> bool,
) -> Result<(), String> {
    for (rel, content) in &index.files {
        if !include(rel) {
            continue;
        }
        let path = output_dir.join(rel);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        write_bytes_atomic(&path, content.as_bytes())?;
    }
    Ok(())
}

/// Write node markdown files referenced in the index file map.
///
/// # Errors
///
/// Returns an error when files cannot be written.
pub fn write_node_files_from_index(index: &SkillsIndex, entry_dir: &Path) -> Result<(), String> {
    write_selected_files(index, entry_dir, |rel| {
        rel.starts_with(&format!("{NODES_DIR}/"))
    })
}

/// Load page-index-only skills index from an entry directory.
///
/// # Errors
///
/// Returns an error when the entry directory is invalid or files cannot be read.
pub fn load_page_index_from_entry(entry_dir: &Path, doc_id: &str) -> Result<SkillsIndex, String> {
    load_skills_index_from_entry(entry_dir, doc_id)
}

/// Load a skills index from an entry directory.
///
/// # Errors
///
/// Returns an error when the catalog directory is invalid or files cannot be read.
pub fn load_skills_index_from_entry(entry_dir: &Path, doc_id: &str) -> Result<SkillsIndex, String> {
    if let Some(cached) = crate::cache::get_skills_index(entry_dir, doc_id) {
        return Ok(cached);
    }
    let index = load_skills_index_from_entry_impl(entry_dir, doc_id)?;
    crate::cache::store_skills_index(entry_dir, doc_id, index.clone());
    Ok(index)
}

fn load_skills_index_from_entry_impl(
    entry_dir: &Path,
    doc_id: &str,
) -> Result<SkillsIndex, String> {
    let mut index = SkillsIndex::default();
    load_entry_files_into_index(entry_dir, &mut index)?;

    let merged = load_merged_document_from_entry(entry_dir)?;
    if let Some(doc) = SkillDocument::from_json(&merged) {
        index.documents.insert(doc_id.to_string(), doc);
    }

    if index.documents.is_empty() {
        return Err(format!(
            "no skill document found in entry directory: {}",
            entry_dir.display()
        ));
    }
    Ok(index)
}

/// Reload a skills index from disk and refresh the hot cache entry.
pub fn refresh_skills_index_cache(entry_dir: &Path, doc_id: &str) {
    if let Ok(index) = load_skills_index_from_entry_impl(entry_dir, doc_id) {
        crate::cache::store_skills_index(entry_dir, doc_id, index);
    }
}

/// Load a skills index from an entry directory.
///
/// # Errors
///
/// Returns an error when the catalog directory is invalid or files cannot be read.
pub fn load_skills_index_from_dir(catalog_dir: &Path) -> Result<SkillsIndex, String> {
    let doc_id = resolve_doc_id(catalog_dir, None, None)?;
    load_skills_index_from_entry(catalog_dir, &doc_id)
}

/// Resolve a catalog document id from `--doc-id`, `--path`, or the entry's page index.
///
/// # Errors
///
/// Returns an error when both selectors are provided, the path does not match any catalog
/// document, or the entry directory has no page index.
pub fn resolve_doc_id(
    catalog_dir: &Path,
    doc_id: Option<&str>,
    skill_path: Option<&Path>,
) -> Result<String, String> {
    match (doc_id, skill_path) {
        (Some(id), None) => Ok(id.to_string()),
        (None, Some(path)) => resolve_doc_id_from_skill_path(catalog_dir, path),
        (None, None) => infer_doc_id_from_entry(catalog_dir),
        (Some(_), Some(_)) => Err("provide either doc_id or path, not both".to_string()),
    }
}

/// Resolve a catalog document id from the original skill file path stored in `page_index.json`.
///
/// # Errors
///
/// Returns an error when the page index is missing or no document matches `skill_path`.
pub fn resolve_doc_id_from_skill_path(
    catalog_dir: &Path,
    skill_path: &Path,
) -> Result<String, String> {
    let page_path = page_index_path(catalog_dir);
    if !page_path.is_file() {
        return Err(format!("page index not found at {}", page_path.display()));
    }

    let raw = fs::read_to_string(&page_path).map_err(|e| e.to_string())?;
    let value: serde_json::Value = serde_json::from_str(&raw).map_err(|e| e.to_string())?;
    let stored_path = value.get("path").and_then(|v| v.as_str()).unwrap_or("");
    let doc_id = value
        .get("id")
        .and_then(|v| v.as_str())
        .ok_or_else(|| format!("page index missing id at {}", page_path.display()))?;

    let query = skill_path.to_string_lossy();
    let query_short = shorten_home_path(&query).unwrap_or_else(|_| normalize_skill_path(&query));

    if skill_paths_match(stored_path, &query_short) {
        return Ok(doc_id.to_string());
    }

    Err(format!(
        "no catalog document matches skill path {} (catalog path: {stored_path})",
        skill_path.display()
    ))
}

/// Reconstruct a skills index from an entry directory.
///
/// # Errors
///
/// Returns an error when the entry directory is missing or contains no documents.
pub fn skills_index_from_decomposed_dir(dir: &Path) -> Result<SkillsIndex, String> {
    load_skills_index_from_dir(dir)
}

impl SkillsIndex {
    /// Reconstruct a skills index from files under `catalog_dir`.
    ///
    /// # Errors
    ///
    /// Returns an error when the entry directory is missing or contains no documents.
    pub fn from_decomposed_dir(catalog_dir: &Path) -> Result<Self, String> {
        load_skills_index_from_dir(catalog_dir)
    }
}

/// Load entry files from disk into an existing index.
///
/// # Errors
///
/// Returns an error when files cannot be read.
pub fn load_decomposed_files_for_index(
    catalog_dir: &Path,
    index: &mut SkillsIndex,
) -> Result<(), String> {
    load_entry_files_into_index(catalog_dir, index)
}

fn load_entry_files_into_index(entry_dir: &Path, index: &mut SkillsIndex) -> Result<(), String> {
    let nodes = nodes_dir(entry_dir);
    if nodes.is_dir() {
        load_dir_files(entry_dir, &nodes, index)?;
    }
    Ok(())
}

fn load_dir_files(entry_dir: &Path, dir: &Path, index: &mut SkillsIndex) -> Result<(), String> {
    for file_entry in fs::read_dir(dir).map_err(|e| e.to_string())? {
        let file_entry = file_entry.map_err(|e| e.to_string())?;
        let path = file_entry.path();
        if !path.is_file() {
            continue;
        }
        let rel = path
            .strip_prefix(entry_dir)
            .map_err(|e| e.to_string())?
            .to_string_lossy()
            .replace('\\', "/");
        let content = fs::read_to_string(&path).map_err(|e| e.to_string())?;
        index.files.insert(rel, content);
    }
    Ok(())
}

fn normalize_skill_path(path: &str) -> String {
    path.replace('\\', "/").trim_start_matches("./").to_string()
}

fn skill_paths_match(stored: &str, query: &str) -> bool {
    let stored = normalize_skill_path(stored);
    let query = normalize_skill_path(query);
    stored == query || stored.ends_with(&query) || query.ends_with(&stored)
}

fn infer_doc_id_from_entry(entry_dir: &Path) -> Result<String, String> {
    let page_path = entry_dir.join(page_index_rel());
    if page_path.is_file() {
        let raw = fs::read_to_string(&page_path).map_err(|e| e.to_string())?;
        let value: serde_json::Value = serde_json::from_str(&raw).map_err(|e| e.to_string())?;
        if let Some(id) = value.get("id").and_then(|v| v.as_str()) {
            return Ok(id.to_string());
        }
    }
    Err(format!(
        "could not infer doc_id from entry directory: {}",
        entry_dir.display()
    ))
}

pub fn merge_skills_index_files(index: &mut SkillsIndex, other: &SkillsIndex) {
    index.documents.extend(other.documents.clone());
    index.files.extend(other.files.clone());
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pageindex::{PageIndexConfig, build_skills_index};

    #[test]
    fn write_and_reconstruct_from_split_index_files() -> Result<(), String> {
        let dir = std::env::temp_dir().join(format!("cysk-skills-{}", std::process::id()));
        let skills_dir = dir.join("skills-src");
        let entry_dir = dir.join("entry");
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&skills_dir).map_err(|e| e.to_string())?;
        fs::write(
            skills_dir.join("skill.md"),
            "# Root\n\nBody\n\n## Child\n\nMore",
        )
        .map_err(|e| e.to_string())?;

        let index = build_skills_index(&[skills_dir], &PageIndexConfig::default())?;
        write_skills_index(&index, &entry_dir)?;

        assert!(entry_dir.join("nodes/page_index.json").is_file());
        assert!(entry_dir.join("nodes/n2.md").is_file());

        let rebuilt = load_skills_index_from_entry(&entry_dir, "skill")?;
        assert_eq!(rebuilt.documents.len(), index.documents.len());
        assert!(!rebuilt.files.is_empty());
        let _ = fs::remove_dir_all(&dir);
        Ok(())
    }

    #[test]
    fn resolve_doc_id_from_skill_path_matches_page_index() -> Result<(), String> {
        let dir = std::env::temp_dir().join(format!("cysk-resolve-{}", std::process::id()));
        let entry_dir = dir.join("entry");
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(entry_dir.join("nodes")).map_err(|e| e.to_string())?;
        fs::write(
            entry_dir.join("nodes/page_index.json"),
            r#"{
  "id": "skill",
  "path": "examples/context7/original/SKILL.md",
  "type": "md",
  "doc_name": "SKILL",
  "line_count": 1,
  "structure": []
}"#,
        )
        .map_err(|e| e.to_string())?;

        let doc_id = resolve_doc_id_from_skill_path(
            &entry_dir,
            Path::new("examples/context7/original/SKILL.md"),
        )?;
        assert_eq!(doc_id, "skill");

        let doc_id = resolve_doc_id(
            &entry_dir,
            None,
            Some(Path::new("examples/context7/original/SKILL.md")),
        )?;
        assert_eq!(doc_id, "skill");

        let doc_id = resolve_doc_id(&entry_dir, Some("skill"), None)?;
        assert_eq!(doc_id, "skill");

        let _ = fs::remove_dir_all(&dir);
        Ok(())
    }
}
