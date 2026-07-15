//! Process-wide hot LRU caches for frequently read cache artifacts.

use std::path::Path;
use std::sync::{LazyLock, Mutex};

use serde_json::Value;

use crate::pageindex::SkillsIndex;

use super::config::memory_cache_config;
use super::lru::LruCache;

static SKILLS_HOT: LazyLock<Mutex<SkillsHotCaches>> =
    LazyLock::new(|| Mutex::new(SkillsHotCaches::new()));

struct SkillsHotCaches {
    merged_documents: LruCache<String, Value>,
    skills_indices: LruCache<String, SkillsIndex>,
}

impl SkillsHotCaches {
    fn new() -> Self {
        let cfg = memory_cache_config();
        Self {
            merged_documents: LruCache::new(cfg.lru_merged_documents),
            skills_indices: LruCache::new(cfg.lru_skills_index),
        }
    }

    fn refresh_capacities(&mut self) {
        let cfg = memory_cache_config();
        self.merged_documents = LruCache::new(cfg.lru_merged_documents);
        self.skills_indices = LruCache::new(cfg.lru_skills_index);
    }
}

fn with_skills_hot<F, R>(f: F) -> R
where
    F: FnOnce(&mut SkillsHotCaches) -> R,
{
    let mut guard = SKILLS_HOT
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    f(&mut guard)
}

/// Rebuild LRU stores after config changes.
pub fn reset_hot_caches() {
    with_skills_hot(SkillsHotCaches::refresh_capacities);
}

fn merged_doc_key(entry_dir: &Path) -> String {
    entry_dir.display().to_string()
}

fn skills_index_key(entry_dir: &Path, doc_id: &str) -> String {
    format!("{}|{doc_id}", entry_dir.display())
}

/// Store or fetch a merged skill document JSON value.
#[must_use]
pub fn store_merged_document(entry_dir: &Path, document: Value) -> Value {
    let key = merged_doc_key(entry_dir);
    with_skills_hot(|hot| {
        hot.merged_documents.insert(key, document.clone());
    });
    document
}

#[must_use]
pub fn get_merged_document(entry_dir: &Path) -> Option<Value> {
    let key = merged_doc_key(entry_dir);
    with_skills_hot(|hot| hot.merged_documents.get_cloned(&key))
}

/// Store a fully loaded skills index for later reconstruct calls.
pub fn store_skills_index(entry_dir: &Path, doc_id: &str, index: SkillsIndex) {
    let key = skills_index_key(entry_dir, doc_id);
    with_skills_hot(|hot| {
        hot.skills_indices.insert(key, index);
    });
}

#[must_use]
pub fn get_skills_index(entry_dir: &Path, doc_id: &str) -> Option<SkillsIndex> {
    let key = skills_index_key(entry_dir, doc_id);
    with_skills_hot(|hot| hot.skills_indices.get_cloned(&key))
}
