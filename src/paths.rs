use std::path::{Path, PathBuf};

use serde_json::Value;
use std::sync::{OnceLock, RwLock};

/// SDK runtime path defaults; override from the host app via `configure`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PathConfig {
    pub md_ext: String,
    pub skills_decomposed_prefix: String,
    pub skills_decomposed_root: PathBuf,
    pub default_catalog_dir: PathBuf,
}

impl Default for PathConfig {
    fn default() -> Self {
        Self {
            md_ext: ".md".to_string(),
            skills_decomposed_prefix: "skills/decomposed/".to_string(),
            skills_decomposed_root: PathBuf::from("skills/decomposed"),
            default_catalog_dir: PathBuf::from(".catalog"),
        }
    }
}

fn config_lock() -> &'static RwLock<PathConfig> {
    static CONFIG: OnceLock<RwLock<PathConfig>> = OnceLock::new();
    CONFIG.get_or_init(|| RwLock::new(PathConfig::default()))
}

pub fn configure(cfg: PathConfig) {
    *config_lock()
        .write()
        .unwrap_or_else(std::sync::PoisonError::into_inner) = cfg;
}

#[must_use]
pub fn snapshot() -> PathConfig {
    config_lock()
        .read()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .clone()
}

#[must_use]
pub fn md_ext() -> String {
    snapshot().md_ext
}

#[must_use]
pub fn skills_decomposed_prefix() -> String {
    snapshot().skills_decomposed_prefix
}

#[must_use]
pub fn skills_decomposed_root() -> PathBuf {
    snapshot().skills_decomposed_root
}

#[must_use]
pub fn default_catalog_dir() -> PathBuf {
    snapshot().default_catalog_dir
}

#[must_use]
pub fn to_skills_decomposed_key(file_path: &str) -> Option<String> {
    let normalized = file_path.replace('\\', "/");
    let prefix = skills_decomposed_prefix();
    normalized
        .strip_prefix(&prefix)
        .map(str::to_string)
        .or_else(|| {
            let root = skills_decomposed_root();
            let root_str = root.to_string_lossy();
            normalized
                .strip_prefix(root_str.as_ref())
                .map(|s| s.trim_start_matches('/').to_string())
        })
}

/// Expand a leading `~/` to the user home directory.
///
/// # Errors
///
/// Returns an error when home cannot be resolved for a `~/` path.
pub fn expand_home_path(path: &Path) -> Result<PathBuf, String> {
    let s = path.to_string_lossy();
    if s.starts_with("~/") {
        return Ok(home_dir()?.join(s.trim_start_matches("~/")));
    }
    Ok(path.to_path_buf())
}

/// Return the user home directory.
///
/// # Errors
///
/// Returns an error when the home directory cannot be resolved.
pub fn home_dir() -> Result<PathBuf, String> {
    std::env::var_os("HOME")
        .map(PathBuf::from)
        .or_else(|| {
            #[cfg(windows)]
            {
                std::env::var_os("USERPROFILE").map(PathBuf::from)
            }
            #[cfg(not(windows))]
            {
                None
            }
        })
        .ok_or_else(|| "could not resolve home directory".to_string())
}

/// Shorten an absolute home path to `~/...` when possible.
///
/// # Errors
///
/// Returns an error when home cannot be resolved for shortening.
pub fn shorten_home_path(path: &str) -> Result<String, String> {
    let home = home_dir()?;
    let home_str = home.to_string_lossy();
    if let Some(rest) = path.strip_prefix(home_str.as_ref()) {
        let trimmed = rest.trim_start_matches('/');
        return Ok(format!("~/{trimmed}"));
    }
    Ok(path.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shorten_home_path_prefixes_tilde() -> Result<(), String> {
        let home = home_dir()?;
        let path = home.join("skills/foo/SKILL.md");
        let shortened = shorten_home_path(&path.to_string_lossy())?;
        assert!(shortened.starts_with("~/"));
        Ok(())
    }
}
