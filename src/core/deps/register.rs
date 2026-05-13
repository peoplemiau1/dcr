use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::fs;
use std::path::{Path, PathBuf};
use toml::Value as TomlValue;

#[derive(Debug, Serialize, Deserialize)]
pub struct Registry {
    pub url: String,
    pub priority: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DcrConfig {
    pub registry: std::collections::HashMap<String, Registry>,
}

pub fn get_registry_config() -> Option<DcrConfig> {
    let home = std::env::var("HOME").ok()?;
    let config_path = Path::new(&home).join(".dcr/config.toml");
    if !config_path.exists() {
        return None;
    }

    let content = fs::read_to_string(config_path).ok()?;
    toml::from_str(&content).ok()
}

pub fn get_index_path() -> PathBuf {
    std::env::var("DCR_INDEX_PATH")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
            Path::new(&home).join(".dcr").join("index.json")
        })
}

pub fn resolve_package_from_registry(name: &str) -> Result<JsonValue, String> {
    let config = get_registry_config().ok_or("No registry config found")?;
    let mut registries: Vec<(&String, &Registry)> = config.registry.iter().collect();
    registries.sort_by(|a, b| b.1.priority.cmp(&a.1.priority));

    for (_name_reg, _reg) in registries {
        let index_path = get_index_path();
        if index_path.exists() {
            let index_content = fs::read_to_string(&index_path).map_err(|e| e.to_string())?;
            let index: JsonValue =
                serde_json::from_str(&index_content).map_err(|e| e.to_string())?;
            if let Some(pkgs) = index.get("packages").and_then(|v| v.as_array()) {
                for pkg in pkgs {
                    if pkg.get("name").and_then(|v| v.as_str()) == Some(name) {
                        return Ok(pkg.clone());
                    }
                }
            }
        }
    }
    Err(format!(
        "Package {} not found in registry (checked: {:?})",
        name,
        get_index_path()
    ))
}

pub fn is_registry_dep(value: &TomlValue) -> bool {
    if value.is_str() {
        return true;
    }
    if let Some(table) = value.as_table() {
        return !table.contains_key("git")
            && !table.contains_key("path")
            && !table.contains_key("url")
            && !table.contains_key("version");
    }
    false
}
