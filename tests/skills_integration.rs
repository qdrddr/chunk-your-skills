use chunk_your_skills::{
    PageIndexConfig, ReconstructOptions, SkillsBuilder, build_skills_index, get_skill_document,
    get_skill_line_content_from_spec, get_skill_structure, load_skills_index_from_dir,
    reconstruct_skill_markdown, repair_skill_nodes, skills_index_from_decomposed_dir,
};
use std::fs;
use std::path::Path;

fn fixture_skills_dir(base: &Path) -> Result<std::path::PathBuf, String> {
    let dir = base.join("skills-src");
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    fs::write(
        dir.join("create-hook.md"),
        "# Create Hook\n\nIntro\n\n## Usage\n\nRun the hook.\n\n## API\n\nDetails here.",
    )
    .map_err(|e| e.to_string())?;
    Ok(dir)
}

#[test]
fn in_memory_build_and_retrieve() -> Result<(), String> {
    let tmp = std::env::temp_dir().join(format!("cysk-skills-mem-{}", std::process::id()));
    let skills_dir = fixture_skills_dir(&tmp)?;
    let index = build_skills_index(&[skills_dir], &PageIndexConfig::default())?;
    assert_eq!(index.documents.len(), 1);
    let doc_id = "create-hook";
    let meta = get_skill_document(&index.documents, doc_id);
    assert_eq!(
        meta.get("doc_name").and_then(|v| v.as_str()),
        Some("create-hook")
    );
    let structure = get_skill_structure(&index.documents, doc_id);
    assert!(structure.is_array());
    let content = get_skill_line_content_from_spec(&index, doc_id, "1,5");
    let arr = content
        .as_array()
        .ok_or_else(|| "expected content array".to_string())?;
    assert!(!arr.is_empty());
    let _ = fs::remove_dir_all(&tmp);
    Ok(())
}

#[test]
fn write_reconstruct_and_retrieve_via_cli_flow() -> Result<(), String> {
    let tmp = std::env::temp_dir().join(format!("cysk-skills-disk-{}", std::process::id()));
    let _ = fs::remove_dir_all(&tmp);
    let skills_dir = fixture_skills_dir(&tmp)?;
    let catalog = tmp.join("catalog");

    let mut builder = SkillsBuilder::new(false, Some(catalog.clone()));
    builder.build_from_dirs(&[skills_dir], &PageIndexConfig::default())?;
    let index = builder.write_catalog()?;

    assert!(catalog.join("nodes/page_index.json").is_file());
    assert!(
        index.files.keys().any(|k| {
            k.starts_with("nodes/")
                && std::path::Path::new(k)
                    .extension()
                    .is_some_and(|ext| ext.eq_ignore_ascii_case("md"))
        }),
        "expected node markdown files"
    );

    let loaded = load_skills_index_from_dir(&catalog)?;
    let doc_id = "create-hook";
    let content = get_skill_line_content_from_spec(&loaded, doc_id, "5-10");
    let arr = content
        .as_array()
        .ok_or_else(|| "expected content array".to_string())?;
    assert!(!arr.is_empty());
    let _ = fs::remove_dir_all(&tmp);
    Ok(())
}

#[test]
fn decomposed_markdown_preserves_original_header() -> Result<(), String> {
    let tmp = std::env::temp_dir().join(format!("cysk-skills-header-{}", std::process::id()));
    let _ = fs::remove_dir_all(&tmp);
    let skills_dir = tmp.join("skills-src");
    fs::create_dir_all(&skills_dir).map_err(|e| e.to_string())?;
    fs::write(
        skills_dir.join("skill.md"),
        "## When to Use\n\nBody text\n\n### Child\n\nMore",
    )
    .map_err(|e| e.to_string())?;

    let index = build_skills_index(&[skills_dir], &PageIndexConfig::default())?;
    let content = index
        .files
        .get("nodes/n2.md")
        .ok_or_else(|| "missing decomposed node file".to_string())?;

    assert!(
        !content.contains("title:"),
        "frontmatter should not repeat the heading title"
    );
    assert!(
        !content.contains("# When to Use\n\n## When to Use"),
        "decomposed body should not duplicate the heading"
    );
    assert!(
        content.contains("## When to Use\n\nBody text"),
        "decomposed body should preserve the original heading level"
    );
    assert!(
        content.contains("token_count:"),
        "node frontmatter should include token_count"
    );

    let _ = fs::remove_dir_all(&tmp);
    Ok(())
}

#[test]
fn node_files_emitted_for_each_section() -> Result<(), String> {
    let tmp = std::env::temp_dir().join(format!("cysk-skills-nodes-{}", std::process::id()));
    let _ = fs::remove_dir_all(&tmp);
    let skills_dir = fixture_skills_dir(&tmp)?;

    let index = build_skills_index(&[skills_dir], &PageIndexConfig::default())?;
    assert_eq!(index.documents.len(), 1);
    assert!(
        index.files.keys().any(|k| {
            k.starts_with("nodes/")
                && std::path::Path::new(k)
                    .extension()
                    .is_some_and(|ext| ext.eq_ignore_ascii_case("md"))
        }),
        "expected node markdown files"
    );
    assert!(
        index.files.contains_key("nodes/page_index.json"),
        "expected page_index.json"
    );

    let _ = fs::remove_dir_all(&tmp);
    Ok(())
}

#[test]
fn reconstruct_by_node_id() -> Result<(), String> {
    let tmp = std::env::temp_dir().join(format!("cysk-reconstruct-{}", std::process::id()));
    let _ = fs::remove_dir_all(&tmp);
    let skills_dir = fixture_skills_dir(&tmp)?;
    let index = build_skills_index(&[skills_dir], &PageIndexConfig::default())?;
    let doc_id = "create-hook";

    let result =
        reconstruct_skill_markdown(&index, doc_id, &[], &["3"], &ReconstructOptions::default())?;
    assert!(result.markdown.contains("## Usage"));
    assert!(!result.markdown.contains("## API"));

    let _ = fs::remove_dir_all(&tmp);
    Ok(())
}

#[test]
fn catalog_roundtrip_from_decomposed_dir() -> Result<(), String> {
    let tmp = std::env::temp_dir().join(format!("cysk-catalog-{}", std::process::id()));
    let _ = fs::remove_dir_all(&tmp);
    let skills_dir = fixture_skills_dir(&tmp)?;
    let catalog = tmp.join("catalog");

    let mut builder = SkillsBuilder::new(false, Some(catalog.clone()));
    builder.build_from_dirs(&[skills_dir], &PageIndexConfig::default())?;
    builder.write_catalog()?;

    let index = skills_index_from_decomposed_dir(&catalog)?;
    assert!(index.documents.contains_key("create-hook"));

    let _ = fs::remove_dir_all(&tmp);
    Ok(())
}

#[test]
fn repair_skill_nodes_succeeds_for_valid_entry() -> Result<(), String> {
    let tmp = std::env::temp_dir().join(format!("cysk-repair-{}", std::process::id()));
    let _ = fs::remove_dir_all(&tmp);
    let skills_dir = fixture_skills_dir(&tmp)?;
    let catalog = tmp.join("catalog");

    let mut builder = SkillsBuilder::new(false, Some(catalog.clone()));
    builder.build_from_dirs(&[skills_dir], &PageIndexConfig::default())?;
    builder.write_catalog()?;

    repair_skill_nodes(&catalog, "create-hook", &PageIndexConfig::default())?;

    let _ = fs::remove_dir_all(&tmp);
    Ok(())
}
