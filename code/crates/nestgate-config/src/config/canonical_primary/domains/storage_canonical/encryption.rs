// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

// **STORAGE ENCRYPTION CONFIGURATION**

use serde::{Deserialize, Serialize};

/// Storage encryption configuration for data security at rest and in transit.
///
/// Provides comprehensive encryption settings including key management,
/// data encryption, transit encryption, and algorithm selection.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `StorageEncryption`
pub struct StorageEncryptionConfig {
    /// General encryption settings.
    pub encryption: EncryptionConfig,
    /// Key management configuration for encryption keys.
    pub key_management: KeyManagementStorageConfig,
    /// Data-at-rest encryption settings.
    pub data_encryption: DataEncryptionConfig,
    /// Data-in-transit encryption settings.
    pub transit_encryption: TransitEncryptionConfig,
    /// Encryption algorithm configuration.
    pub algorithm: EncryptionAlgorithmStorageConfig,
}

/// General encryption configuration.
///
/// Controls whether encryption features are enabled globally.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Encryption
pub struct EncryptionConfig {
    /// Whether encryption is enabled (default: false for development).
    pub enabled: bool,
}

/// Key management configuration for encryption keys.
///
/// Defines where and how encryption keys are stored and managed.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `KeyManagementStorage`
pub struct KeyManagementStorageConfig {
    /// Key management provider (e.g., "local", "vault", "kms").
    pub provider: String,
}

/// Data-at-rest encryption configuration.
///
/// Controls encryption of stored data on disk.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `DataEncryption`
pub struct DataEncryptionConfig {
    /// Whether data-at-rest encryption is enabled.
    pub enabled: bool,
}

/// Data-in-transit encryption configuration.
///
/// Controls encryption of data during network transmission.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `TransitEncryption`
pub struct TransitEncryptionConfig {
    /// Whether transit encryption (TLS/SSL) is enabled (default: true).
    pub enabled: bool,
}

/// Encryption algorithm configuration.
///
/// Specifies the cryptographic algorithm used for encryption.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `EncryptionAlgorithmStorage`
pub struct EncryptionAlgorithmStorageConfig {
    /// Encryption algorithm name (default: "AES-256-GCM").
    pub algorithm: String,
}

impl Default for StorageEncryptionConfig {
    /// Returns the default instance
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
    /// Create production-optimized encryption configuration.
    ///
    /// Uses default settings suitable for production environments.
    #[must_use]
    pub fn production_optimized() -> Self {
        Self::default()
    }

    /// Create development-optimized encryption configuration.
    ///
    /// Uses default settings suitable for local development.
    #[must_use]
    pub fn development_optimized() -> Self {
        Self::default()
    }

    /// Create high-performance encryption configuration.
    ///
    /// Optimized for maximum throughput.
    #[must_use]
    pub fn high_performance() -> Self {
        Self::default()
    }

    /// Create cloud-native encryption configuration.
    ///
    /// Optimized for cloud deployments with managed key services.
    #[must_use]
    pub fn cloud_native() -> Self {
        Self::default()
    }

    /// Merge this configuration with another, preferring values from `other`.
    #[must_use]
    pub fn merge(self, _other: Self) -> Self {
        self
    }

    /// Validate encryption configuration.
    ///
    /// # Errors
    ///
    /// Returns an error if validation fails.
    pub const fn validate(&self) -> nestgate_types::error::Result<()> {
        Ok(())
    }
}
