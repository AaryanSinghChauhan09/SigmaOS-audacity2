//! Configuration implementation

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudacityConfig {
    pub config_dir: String,
    pub data_dir: String,
    pub cache_dir: String,
    pub log_dir: String,
}

impl Default for AudacityConfig {
    fn default() -> Self {
        Self {
            config_dir: "/etc/sigmaos".to_string(),
            data_dir: "/var/lib/sigmaos".to_string(),
            cache_dir: "/var/cache/sigmaos".to_string(),
            log_dir: "/var/log/sigmaos".to_string(),
        }
    }
}

impl AudacityConfig {
    /// Load default configuration
    pub fn load_default() -> Result<Self> {
        Ok(Self::default())
    }

    /// Load from file
    pub fn load_from_file(path: &str) -> Result<Self> {
        if Path::new(path).exists() {
            let content = std::fs::read_to_string(path)?;
            let config: AudacityConfig = serde_json::from_str(&content)?;
            Ok(config)
        } else {
            Ok(Self::default())
        }
    }

    /// Save to file
    pub fn save_to_file(&self, path: &str) -> Result<()> {
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }
}
