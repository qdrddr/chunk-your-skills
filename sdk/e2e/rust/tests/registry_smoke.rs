use chunk_your_skills::{PageIndexConfig, build_skills_index, count_tokens};
use std::fs;
use std::path::PathBuf;

fn fixture_skills_dir(base: &PathBuf) -> std::io::Result<()> {
    let dir = base.join("skills-src");
    fs::create_dir_all(&dir)?;
    fs::write(
        dir.join("create-hook.md"),
        "# Create Hook\n\nIntro\n\n## Usage\n\nRun the hook.\n",
    )?;
    Ok(())
}

#[test]
fn build_skills_index_from_registry_crate() {
    let tmp = std::env::temp_dir().join(format!("cysk-e2e-rust-{}", std::process::id()));
    let _ = fs::remove_dir_all(&tmp);
    fixture_skills_dir(&tmp).expect("fixture skills dir");
    let skills_dir = tmp.join("skills-src");

    let index = build_skills_index(&[skills_dir], &PageIndexConfig::default())
        .expect("build_skills_index");
    assert_eq!(index.documents.len(), 1);
    assert!(index
        .files
        .keys()
        .any(|k| k.starts_with("nodes/") && k.ends_with(".md")));
    let _ = fs::remove_dir_all(&tmp);
}

#[test]
fn count_tokens_smoke() {
    let count = count_tokens("hello world").expect("count_tokens");
    assert!(count >= 1);
}
