//! Cache fallback tests.

#![allow(clippy::unwrap_used)]

use std::path::PathBuf;

use crate::cache::{disk_available, CachePolicy, CacheStatus};
use crate::cache::test_guard::CacheConfigTestGuard;

fn unavailable_disk_cache_path() -> PathBuf {
    if cfg!(windows) {
        PathBuf::from(r"Z:\__cysk_nonexistent_drive\cysk-cache-test")
    } else {
        PathBuf::from("/nonexistent-root/cysk-cache-test")
    }
}

#[test]
fn disk_available_false_for_missing_home_subpath() {
    assert!(!disk_available(&unavailable_disk_cache_path()));
}

#[test]
fn force_memory_env_disables_disk() {
    let _guard = CacheConfigTestGuard::with_patch(&serde_json::json!({ "async_disk_writes": false }));
    unsafe {
        std::env::set_var("CYSK_CACHE_FORCE_MEMORY", "1");
    }
    let tmp = std::env::temp_dir().join(format!("cysk-cache-{}", std::process::id()));
    let _ = std::fs::create_dir_all(&tmp);
    assert!(!disk_available(&tmp));
    let _ = std::fs::remove_dir_all(&tmp);
    unsafe {
        std::env::remove_var("CYSK_CACHE_FORCE_MEMORY");
    }
}

#[test]
fn force_memory_policy_skips_disk() {
    let _guard = CacheConfigTestGuard::with_patch(&serde_json::json!({ "async_disk_writes": false }));
    let tmp = std::env::temp_dir().join(format!("cysk-policy-{}", std::process::id()));
    let skills = tmp.join("skills");
    let catalog = tmp.join("catalog");
    let _ = std::fs::remove_dir_all(&tmp);
    std::fs::create_dir_all(&skills).unwrap();
    std::fs::write(
        skills.join("demo.md"),
        "---\nname: demo\n---\n# Title\n\nBody",
    )
    .unwrap();

    let refs = crate::cache::ensure_skills_registry(
        &[skills.join("demo.md")],
        &catalog,
        &crate::pageindex::PageIndexConfig::default(),
        CachePolicy::ForceMemory,
    )
    .unwrap();
    assert_eq!(refs.len(), 1);
    assert!(!refs[0].disk_backed);
    assert_eq!(refs[0].cache_status, CacheStatus::MemoryFallback);
    let _ = std::fs::remove_dir_all(&tmp);
}
