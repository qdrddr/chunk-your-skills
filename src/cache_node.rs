// N-API bindings for the Rust cache engine (included from `node.rs`).

use crate::cache::{
    ensure_skills_registry_from_specs, parse_skill_source_specs_json, parse_skill_sources,
    CachePolicy, CacheStatus,
};
use crate::pageindex::PageIndexConfig;
use napi::bindgen_prelude::*;
use napi_derive::napi;
use serde_json::{json, Value};
use std::path::PathBuf;

fn cache_policy_from_str(raw: Option<&str>) -> CachePolicy {
    match raw.map(str::trim) {
        Some(s) if s.eq_ignore_ascii_case("force_memory") || s.eq_ignore_ascii_case("memory") => {
            CachePolicy::ForceMemory
        }
        Some(s) if s.eq_ignore_ascii_case("force_disk") || s.eq_ignore_ascii_case("disk") => {
            CachePolicy::ForceDisk
        }
        _ => CachePolicy::Auto,
    }
}

const fn cache_status_str(status: CacheStatus) -> &'static str {
    match status {
        CacheStatus::Hit => "hit",
        CacheStatus::Miss => "miss",
        CacheStatus::MemoryFallback => "memory_fallback",
    }
}

fn page_index_config_from_value(config: Option<Value>) -> PageIndexConfig {
    config.map_or_else(PageIndexConfig::default, |val| PageIndexConfig::from_value(&val))
}

/// # Errors
///
/// Returns an error when skills registry ensure fails.
#[napi(js_name = "ensureSkillsRegistry")]
pub fn ensure_skills_registry_napi(
    source_paths: Vec<Value>,
    catalog_root: String,
    pageindex_config: Option<Value>,
    policy: Option<String>,
) -> Result<Vec<Value>> {
    let source_paths = Box::new(source_paths);
    let specs = if source_paths.iter().any(Value::is_object) {
        parse_skill_source_specs_json(source_paths.as_ref()).map_err(Error::from_reason)?
    } else {
        let paths: Vec<PathBuf> = source_paths
            .iter()
            .filter_map(|v| v.as_str().map(PathBuf::from))
            .collect();
        parse_skill_sources(&paths)
    };
    let cfg = page_index_config_from_value(pageindex_config);
    let cache_policy = cache_policy_from_str(policy.as_deref());
    drop(policy);
    let refs = ensure_skills_registry_from_specs(
        &specs,
        PathBuf::from(catalog_root).as_path(),
        &cfg,
        cache_policy,
    )
    .map_err(Error::from_reason)?;
    Ok(refs
        .into_iter()
        .map(|entry| {
            json!({
                "entry_dir": entry.entry_dir.display().to_string(),
                "doc_id": entry.doc_id,
                "content_sha256": entry.content_sha256,
                "disk_backed": entry.disk_backed,
                "cache_status": cache_status_str(entry.cache_status),
                "source_path": entry.source_path,
                "nodes_dir": entry.nodes_dir.as_ref().map(|p| p.display().to_string()),
                "document": entry.document,
                "lazy_pending": entry.lazy_pending,
            })
        })
        .collect())
}

/// Apply in-memory cache tuning from a config object.
///
/// # Errors
///
/// Returns an error when the config object cannot be parsed.
#[napi(js_name = "configureMemoryCache")]
#[allow(clippy::needless_pass_by_value)]
pub fn configure_memory_cache_napi(config: Value) -> Result<()> {
    crate::cache::configure_memory_cache(&config);
    Ok(())
}
