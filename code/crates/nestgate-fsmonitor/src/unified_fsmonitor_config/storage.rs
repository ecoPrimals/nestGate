// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

/// Storage and persistence configuration - extracted from monolithic config
/// Handles storage backends, retention, compression, indexing, and backup settings
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Duration;
/// Storage and persistence settings
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FsMonitorStorageSettings {
    /// Enable event storage
    pub enabled: bool,
    /// Storage backend configuration
    pub backend: StorageBackendConfig,
    /// Event retention settings
    pub retention: EventRetentionSettings,
    /// Compression settings
    pub compression: CompressionSettings,
    /// Indexing settings
    pub indexing: IndexingSettings,
    /// Backup and recovery settings
    pub backup: BackupSettings,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
/// 
/// **Migration Path**:
/// ```rust
/// // OLD (deprecated):
/// use crate::config::StorageBackendConfig;
/// 
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::config::StorageBackendConfig; // Now aliases to CanonicalNetworkConfig
/// ```
/// 
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(since = "0.11.0", note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead")]
pub struct StorageBackendConfig {
    /// Backend type (file, database, memory, etc.)
    pub backend_type: String,
    /// Connection configuration
    pub connection: HashMap<String, serde_json::Value>,
    /// Connection pool settings
    pub pool: ConnectionPoolSettings,
    /// Enable encryption at rest
    pub encryption_enabled: bool,
    /// Encryption key identifier
    pub encryption_key_id: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionPoolSettings {
    /// Minimum connections
    pub min_connections: u32,
    /// Maximum connections
    pub max_connections: u32,
    /// Connection timeout
    pub connection_timeout: Duration,
    /// Idle timeout
    pub idle_timeout: Duration,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventRetentionSettings {
    /// Enable retention policies
    pub enabled: bool,
    /// Default retention period
    pub default_retention: Duration,
    /// Retention policies by event type
    pub policies: HashMap<String, Duration>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressionSettings {
    /// Enable compression
    pub enabled: bool,
    /// Compression algorithm
    pub algorithm: String,
    /// Compression level (1-9)
    pub level: u8,
    /// Minimum size for compression
    pub min_size: u64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexingSettings {
    /// Enable indexing
    pub enabled: bool,
    /// Index fields
    pub fields: Vec<String>,
    /// Index refresh interval
    pub refresh_interval: Duration,
    /// Enable full-text search
    pub full_text_search: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupSettings {
    /// Enable backups
    pub enabled: bool,
    /// Backup interval
    pub interval: Duration,
    /// Backup location
    pub location: PathBuf,
    /// Number of backups to keep
    pub retention_count: u32,
    /// Enable compression for backups
    pub compress: bool,
}
impl Default for StorageBackendConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            backend_type: "memory".to_string(),
            connection: HashMap::new(),
            pool: ConnectionPoolSettings::default(),
            encryption_enabled: false,
            encryption_key_id: None,
        }
    }
}

impl Default for ConnectionPoolSettings {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            min_connections: 1,
            max_connections: 10,
            connection_timeout: Duration::from_secs(30),
            idle_timeout: Duration::from_secs(600),
        }
    }
}

impl Default for EventRetentionSettings {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            enabled: true,
            default_retention: Duration::from_secs(86400 * 30), // 30 days
            policies: HashMap::new(),
        }
    }
}

impl Default for CompressionSettings {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            enabled: false,
            algorithm: "gzip".to_string(),
            level: 6,
            min_size: 1024, // 1KB
        }
    }
}

impl Default for IndexingSettings {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            enabled: false,
            fields: vec![
                "path".to_string(),
                "event_type".to_string(),
                "timestamp".to_string(),
            ],
            refresh_interval: Duration::from_secs(60),
            full_text_search: false,
        }
    }
}

impl Default for BackupSettings {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            enabled: false,
            interval: Duration::from_secs(86400), // Daily
            location: PathBuf::from("/tmp/fsmonitor_backup"),
            retention_count: 7,
            compress: true,
        }
    }
}

// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
/// 
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
#[allow(deprecated)]
pub type StorageBackendConfigCanonical = nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using StorageBackendConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.

