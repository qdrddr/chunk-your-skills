//! `PyO3` bindings for the Rust cache engine.

use std::path::PathBuf;

use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};

use crate::cache::{
    CachePolicy, CacheStatus, configure_memory_cache, ensure_skills_registry_from_specs,
    parse_skill_source_specs_json, parse_skill_sources,
};
use crate::pageindex::PageIndexConfig;

use super::{py_to_value, value_to_py};

fn cache_policy_from_str(raw: Option<&str>) -> CachePolicy {
    match raw.map(str::trim).map(str::to_ascii_lowercase) {
        Some(s) if s == "force_memory" || s == "memory" => CachePolicy::ForceMemory,
        Some(s) if s == "force_disk" || s == "disk" => CachePolicy::ForceDisk,
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

fn page_index_config_from_py(config: Option<Bound<'_, PyAny>>) -> PyResult<PageIndexConfig> {
    match config {
        Some(obj) => Ok(PageIndexConfig::from_value(&py_to_value(obj)?)),
        None => Ok(PageIndexConfig::default()),
    }
}

#[pyfunction(name = "ensure_skills_registry")]
fn ensure_skills_registry_py(
    py: Python<'_>,
    source_paths: Bound<'_, PyAny>,
    catalog_root: &str,
    pageindex_config: Option<Bound<'_, PyAny>>,
    policy: Option<&str>,
) -> PyResult<Py<PyAny>> {
    let sources_val = py_to_value(source_paths)?;
    let sources_arr = sources_val.as_array().cloned().unwrap_or_default();
    let specs = if sources_arr.iter().any(serde_json::Value::is_object) {
        parse_skill_source_specs_json(&sources_arr)
    } else {
        let paths: Vec<PathBuf> = sources_arr
            .iter()
            .filter_map(|v| v.as_str().map(PathBuf::from))
            .collect();
        Ok(parse_skill_sources(&paths))
    }
    .map_err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>)?;
    let cfg = page_index_config_from_py(pageindex_config)?;
    let refs = ensure_skills_registry_from_specs(
        &specs,
        PathBuf::from(catalog_root).as_path(),
        &cfg,
        cache_policy_from_str(policy),
    )
    .map_err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>)?;

    let list = PyList::empty(py);
    for entry in refs {
        let dict = PyDict::new(py);
        dict.set_item("entry_dir", entry.entry_dir.display().to_string())?;
        dict.set_item("doc_id", entry.doc_id)?;
        dict.set_item("content_sha256", entry.content_sha256)?;
        dict.set_item("disk_backed", entry.disk_backed)?;
        dict.set_item("cache_status", cache_status_str(entry.cache_status))?;
        dict.set_item("source_path", entry.source_path)?;
        dict.set_item(
            "nodes_dir",
            entry.nodes_dir.as_ref().map(|p| p.display().to_string()),
        )?;
        if let Some(document) = &entry.document {
            dict.set_item("document", value_to_py(py, document)?)?;
        }
        dict.set_item("lazy_pending", entry.lazy_pending)?;
        list.append(dict)?;
    }
    Ok(list.into())
}

#[pyfunction(name = "configure_memory_cache")]
fn configure_memory_cache_py(config: Bound<'_, PyAny>) -> PyResult<()> {
    configure_memory_cache(&py_to_value(config)?);
    Ok(())
}

pub fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(ensure_skills_registry_py, m)?)?;
    m.add_function(wrap_pyfunction!(configure_memory_cache_py, m)?)?;
    Ok(())
}
