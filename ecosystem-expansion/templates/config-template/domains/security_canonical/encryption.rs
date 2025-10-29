//! **ENCRYPTION CONFIGURATION**

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionSecurityConfig {
    pub enabled: bool,
    pub algorithms: Vec<String>,
    pub key_management: KeyManagementConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptographicConfig {
    pub algorithm: String,
    pub key_size: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyManagementConfig {
    pub auto_rotation: bool,
    pub key_store: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionAlgorithmConfig {
    pub symmetric: Vec<String>,
    pub asymmetric: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HashingConfig {
    pub algorithm: String,
    pub salt_length: u32,
}

impl Default for EncryptionSecurityConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            algorithms: vec!["AES-256-GCM".to_string()],
            key_management: KeyManagementConfig::default(),
        }
    }
}

impl Default for KeyManagementConfig {
    fn default() -> Self {
        Self {
            auto_rotation: true,
            key_store: "vault".to_string(),
        }
    }
}

impl EncryptionSecurityConfig {
    pub fn production_hardened() -> Self { Self::default() }
    pub fn development_optimized() -> Self { Self::default() }
    pub fn compliance_focused() -> Self { Self::default() }
    pub fn merge(self, _other: Self) -> Self { self }
    pub fn validate(&self) -> crate::Result<()> { Ok(()) }
} 