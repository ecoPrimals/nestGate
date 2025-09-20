// **STORAGE ENCRYPTION CONFIGURATION**

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageEncryptionConfig {
    pub encryption: EncryptionConfig,
    pub key_management: KeyManagementStorageConfig,
    pub data_encryption: DataEncryptionConfig,
    pub transit_encryption: TransitEncryptionConfig,
    pub algorithm: EncryptionAlgorithmStorageConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionConfig {
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyManagementStorageConfig {
    pub provider: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataEncryptionConfig {
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitEncryptionConfig {
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionAlgorithmStorageConfig {
    pub algorithm: String,
}

impl Default for StorageEncryptionConfig {
    fn default() -> Self {
        Self {
            encryption: EncryptionConfig { enabled: false },
            key_management: KeyManagementStorageConfig {
                provider: "local".to_string(),
            },
            data_encryption: DataEncryptionConfig { enabled: false },
            transit_encryption: TransitEncryptionConfig { enabled: true },
            algorithm: EncryptionAlgorithmStorageConfig {
                algorithm: "AES-256-GCM".to_string(),
            },
        }
    }
}

impl StorageEncryptionConfig {
    #[must_use]
    pub const fn production_optimized() -> Self {
        Self::default()
    }
    #[must_use]
    pub const fn development_optimized() -> Self {
        Self::default()
    }
    #[must_use]
    pub const fn high_performance() -> Self {
        Self::default()
    }
    #[must_use]
    pub const fn cloud_native() -> Self {
        Self::default()
    }
    #[must_use]
    pub const fn merge(self, _other: Self) -> Self {
        self
    }
    pub const fn validate(&self) -> crate::Result<()> {
        Ok(())
    }
}
