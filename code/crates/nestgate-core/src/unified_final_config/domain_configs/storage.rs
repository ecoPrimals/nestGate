//
// Storage-specific configuration structures extracted from the monolithic NestGateCanonicalConfig.rs
// for better maintainability and focused responsibility.

use serde::{Deserialize, Serialize};

/// Storage domain configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageDomainConfig {
    pub backend_type: String,
    pub data_directory: PathBuf,
    pub cache_size_mb: u64,
    pub compression_enabled: bool,
    pub encryption_enabled: bool,
    pub backup_enabled: bool,
}
impl Default for StorageDomainConfig {
    fn default() -> Self {
        Self {
            backend_type: "filesystem".to_string(),
            data_directory: PathBuf::from("/var/lib/nestgate/data"),
            cache_size_mb: 512,
            compression_enabled: true,
            encryption_enabled: true,
            backup_enabled: true,
        }
    }
}
