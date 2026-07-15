use std::fs;
use std::path::{Path, PathBuf};

use super::cache_layout::page_index_path;
use super::config::PageIndexConfig;
use super::decompose::decompose_page_index;
use super::document_json::{
    EntryMetadata, entry_hash_matches, page_index_files_complete, read_document_json,
    write_entry_metadata, write_page_index_files,
};
use super::index::md_to_tree;
use super::parse::{extract_node_text_content, extract_nodes_from_markdown, extract_skill_prefix};
use super::tree::{build_tree_from_nodes, finalize_skill_structure};
use super::types::{MdIndexResult, SkillsIndex, build_skill_document, doc_id_from_rel_path};

/// Build an in-memory skills index from one or more skill directories.
///
/// # Errors
///
/// Returns an error when a directory is missing or a markdown file cannot be read.
pub fn build_skills_index(
    skill_dirs: &[PathBuf],
    config: &PageIndexConfig,
) -> Result<SkillsIndex, String> {
    build_skills_index_with_options(skill_dirs, config)
}

/// Build a page-index-only skills index (nodes without chunk variants).
///
/// # Errors
///
/// Returns an error when a directory is missing or a markdown file cannot be read.
pub fn build_page_index_only(
    skill_dirs: &[PathBuf],
    config: &PageIndexConfig,
) -> Result<SkillsIndex, String> {
    build_skills_index_with_options(skill_dirs, config)
}

/// Build a page-index-only skills index from a single markdown file in place.
///
/// # Errors
///
/// Returns an error when the file is missing or cannot be read.
pub fn build_page_index_for_file(
    source: &Path,
    config: &PageIndexConfig,
) -> Result<SkillsIndex, String> {
    let content = fs::read_to_string(source).map_err(|e| e.to_string())?;
    build_page_index_for_content(source, &content, config)
}

/// Build a page-index-only skills index from markdown content in memory.
///
/// `source` is the canonical skill path stored in document metadata (not a temp copy).
///
/// # Errors
///
/// Returns an error when indexing fails.
pub fn build_page_index_for_content(
    source: &Path,
    content: &str,
    config: &PageIndexConfig,
) -> Result<SkillsIndex, String> {
    let mut index = SkillsIndex::default();
    let doc_id = doc_id_from_source_path(source);
    index_skill_md_content(SkillMdIndexRequest {
        path: source,
        doc_id: &doc_id,
        content,
        config,
        index: &mut index,
    })?;
    Ok(index)
}

fn build_skills_index_with_options(
    skill_dirs: &[PathBuf],
    config: &PageIndexConfig,
) -> Result<SkillsIndex, String> {
    let mut index = SkillsIndex::default();

    for dir in skill_dirs {
        let expanded = crate::paths::expand_home_path(dir)?;
        if !expanded.is_dir() {
            return Err(format!(
                "skills directory not found: {}",
                expanded.display()
            ));
        }
        walk_skill_md_files(&expanded, &expanded, config, &mut index)?;
    }

    Ok(index)
}

fn walk_skill_md_files(
    root: &Path,
    current: &Path,
    config: &PageIndexConfig,
    index: &mut SkillsIndex,
) -> Result<(), String> {
    for entry in fs::read_dir(current).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if path.is_dir() {
            walk_skill_md_files(root, &path, config, index)?;
            continue;
        }
        if path.extension().is_none_or(|e| e != "md") {
            continue;
        }
        let rel = path
            .strip_prefix(root)
            .map_err(|e| e.to_string())?
            .to_string_lossy()
            .replace('\\', "/");
        let doc_id = doc_id_from_rel_path(&rel);
        if index.documents.contains_key(&doc_id) {
            continue;
        }
        index_skill_md_file(path.as_path(), &doc_id, config, index)?;
    }
    Ok(())
}

fn doc_id_from_source_path(path: &Path) -> String {
    path.file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("skill")
        .to_string()
        .replace('/', "__")
        .to_lowercase()
}

fn index_skill_md_file(
    path: &Path,
    doc_id: &str,
    config: &PageIndexConfig,
    index: &mut SkillsIndex,
) -> Result<(), String> {
    let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
    index_skill_md_content(SkillMdIndexRequest {
        path,
        doc_id,
        content: &content,
        config,
        index,
    })
}

struct SkillMdIndexRequest<'a> {
    path: &'a Path,
    doc_id: &'a str,
    content: &'a str,
    config: &'a PageIndexConfig,
    index: &'a mut SkillsIndex,
}

fn index_skill_md_content(req: SkillMdIndexRequest<'_>) -> Result<(), String> {
    let SkillMdIndexRequest {
        path,
        doc_id,
        content,
        config,
        index,
    } = req;
    let prefix = extract_skill_prefix(content);
    let source_path = super::document_json::shorten_home_path(path.to_string_lossy().as_ref())?;
    if index.documents.contains_key(doc_id) {
        return Ok(());
    }

    let result = md_to_tree(content, &source_path, config);
    let preamble = prefix.preamble.clone();
    let flat_for_decompose = build_flat_structure(
        content,
        prefix.frontmatter.as_deref(),
        prefix.frontmatter_line_num,
        preamble.as_deref(),
        prefix.preamble_line_num,
        config,
    );
    let doc = build_skill_document(
        doc_id.to_string(),
        &source_path,
        &MdIndexResult {
            doc_name: result.doc_name,
            line_count: result.line_count,
            structure: flat_for_decompose.clone(),
        },
        config,
        prefix.frontmatter,
        preamble,
    );
    decompose_page_index(index, &doc, &flat_for_decompose, config);
    index.documents.insert(doc_id.to_string(), doc);
    Ok(())
}

fn build_flat_structure(
    markdown_content: &str,
    frontmatter: Option<&str>,
    frontmatter_line_num: Option<u32>,
    preamble: Option<&str>,
    preamble_line_num: Option<u32>,
    config: &PageIndexConfig,
) -> serde_json::Value {
    let (node_list, markdown_lines) = extract_nodes_from_markdown(markdown_content);
    let nodes_with_content = extract_node_text_content(&node_list, &markdown_lines);
    let tree = build_tree_from_nodes(&nodes_with_content);
    finalize_skill_structure(
        tree,
        frontmatter,
        frontmatter_line_num,
        preamble,
        preamble_line_num,
        config,
    )
}

/// Persist page-index files for one skill entry.
///
/// # Errors
///
/// Returns an error when files cannot be written.
pub fn write_page_index_entry(
    index: &SkillsIndex,
    entry_dir: &Path,
    doc_id: &str,
    metadata: Option<&EntryMetadata>,
) -> Result<(), String> {
    let doc = index
        .documents
        .get(doc_id)
        .ok_or_else(|| format!("skill document not found: {doc_id}"))?;
    write_page_index_files(entry_dir, doc)?;
    if let Some(metadata) = metadata {
        write_entry_metadata(entry_dir, metadata)?;
    }
    crate::skills_io::write_node_files_from_index(index, entry_dir)?;
    Ok(())
}

/// Return whether the on-disk page index is complete for the entry hash.
#[must_use]
pub fn page_index_valid(entry_dir: &Path, content_sha256: &str) -> bool {
    if !entry_hash_matches(entry_dir, content_sha256) {
        return false;
    }
    let page_path = page_index_path(entry_dir);
    if !page_path.is_file() {
        return false;
    }
    let Ok(page) = read_document_json(&page_path) else {
        return false;
    };
    let Some(structure) = page.get("structure") else {
        return false;
    };
    page_index_files_complete(entry_dir, structure)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stores_home_paths_with_tilde_prefix() -> Result<(), String> {
        let home = crate::paths::home_dir()?;
        let skills_dir = home.join(format!(".cysk-skills-home-{}", std::process::id()));
        fs::create_dir_all(&skills_dir).map_err(|e| e.to_string())?;
        fs::write(skills_dir.join("example.md"), "# Example\n\nBody").map_err(|e| e.to_string())?;

        let index = build_skills_index(
            std::slice::from_ref(&skills_dir),
            &PageIndexConfig::default(),
        )?;
        let doc = index
            .documents
            .get("example")
            .ok_or_else(|| "missing example document".to_string())?;
        assert_eq!(
            doc.path,
            format!("~/.cysk-skills-home-{}/example.md", std::process::id())
        );

        let _ = fs::remove_dir_all(&skills_dir);
        Ok(())
    }

    #[test]
    fn page_index_only_skips_chunk_files() -> Result<(), String> {
        let home = crate::paths::home_dir()?;
        let skills_dir = home.join(format!(".cysk-skills-page-only-{}", std::process::id()));
        fs::create_dir_all(&skills_dir).map_err(|e| e.to_string())?;
        fs::write(
            skills_dir.join("skill.md"),
            "# Root\n\nBody\n\n## Child\n\nMore",
        )
        .map_err(|e| e.to_string())?;

        let index = build_page_index_only(
            std::slice::from_ref(&skills_dir),
            &PageIndexConfig::default(),
        )?;
        assert!(index.files.keys().all(|k| !k.starts_with("chunks/")));

        let _ = fs::remove_dir_all(&skills_dir);
        Ok(())
    }

    #[test]
    fn build_page_index_for_file_indexes_in_place() -> Result<(), String> {
        let home = crate::paths::home_dir()?;
        let skills_dir = home.join(format!(".cysk-skills-single-{}", std::process::id()));
        fs::create_dir_all(&skills_dir).map_err(|e| e.to_string())?;
        let skill_path = skills_dir.join("create-hook.md");
        fs::write(&skill_path, "# Create Hook\n\nBody").map_err(|e| e.to_string())?;
        fs::write(skills_dir.join("other.md"), "# Other\n\nMore").map_err(|e| e.to_string())?;

        let index = build_page_index_for_file(&skill_path, &PageIndexConfig::default())?;
        assert_eq!(index.documents.len(), 1);
        let doc = index
            .documents
            .get("create-hook")
            .ok_or_else(|| "missing create-hook document".to_string())?;
        assert!(doc.path.ends_with("create-hook.md"));

        let _ = fs::remove_dir_all(&skills_dir);
        Ok(())
    }
}
