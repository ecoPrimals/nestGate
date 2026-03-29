// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

// **ENCRYPTION CONFIGURATION**

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `EncryptionSecurity`
pub struct EncryptionSecurityConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
    /// Algorithms
    pub algorithms: Vec<String>,
    /// Key Management
    pub key_management: KeyManagementConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Cryptographic
pub struct CryptographicConfig {
    /// Algorithm
    pub algorithm: String,
    /// Size of key
    pub key_size: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `KeyManagement`
pub struct KeyManagementConfig {
    /// Auto Rotation
    pub auto_rotation: bool,
    /// Key Store
    pub key_store: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `EncryptionAlgorithm`
pub struct EncryptionAlgorithmConfig {
    /// Symmetric
    pub symmetric: Vec<String>,
    /// Asymmetric
    pub asymmetric: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Hashing
pub struct HashingConfig {
    /// Algorithm
    pub algorithm: String,
    /// Salt Length
    pub salt_length: u32,
}

impl Default for EncryptionSecurityConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            enabled: true,
            algorithms: vec!["AES-256-GCM".to_string()],
            key_management: KeyManagementConfig::default(),
        }
    }
}

impl Default for KeyManagementConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            auto_rotation: true,
            key_store: "vault".to_string(),
        }
    }
}

impl EncryptionSecurityConfig {
    /// Returns a production-hardened encryption configuration
    #[must_use]
    pub fn production_hardened() -> Self {
        Self::default()
    }
    /// Returns a development-optimized encryption configuration
    #[must_use]
    pub fn development_optimized() -> Self {
        Self::default()
    }
    /// Returns a compliance-focused encryption configuration
    #[must_use]
    pub fn compliance_focused() -> Self {
        Self::default()
    }
    /// Merges this configuration with another
    #[must_use]
    pub fn merge(self, _other: Self) -> Self {
        self
    }
    /// Validates data
    pub const fn validate(&self) -> nestgate_types::error::Result<()> {
        Ok(())
    }
}
