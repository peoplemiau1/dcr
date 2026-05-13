use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Registry {
    pub url: String,
    pub priority: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub registry: HashMap<String, Registry>,
}

pub struct RegistryManager {
    pub config: Config,
    #[allow(dead_code)]
    pub path: PathBuf,
}

impl RegistryManager {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let home = std::env::var("HOME")?;
        let dcr_dir = PathBuf::from(home).join(".dcr");
        let config_path = dcr_dir.join("config.toml");

        if !config_path.exists() {
            return Err("Config file ~/.dcr/config.toml not found".into());
        }

        let content = fs::read_to_string(&config_path)?;
        let config: Config = toml::from_str(&content)?;

        Ok(Self {
            config,
            path: dcr_dir,
        })
    }
}
