//! Storage configuration types.

use serde::{Deserialize, Serialize};

/// Storage configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    pub data_dir: String,
    pub cache_size_mb: u64,
    pub compression_enabled: bool,
}

impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            data_dir: "/var/lib/nestgate".to_string(),
            cache_size_mb: 1024,
            compression_enabled: true,
        }
    }
} 