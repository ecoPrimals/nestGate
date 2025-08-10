/// **STORAGE MODULE**
/// Storage and persistence configuration - extracted from monolithic config
/// Handles storage backends, retention, compression, indexing, and backup settings
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Duration;

/// Storage and persistence settings
#[derive(Debug, Clone, Serialize, Deserialize)]
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

impl Default for FsMonitorStorageSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            backend: StorageBackendConfig::default(),
            retention: EventRetentionSettings::default(),
            compression: CompressionSettings::default(),
            indexing: IndexingSettings::default(),
            backup: BackupSettings::default(),
        }
    }
}

impl Default for StorageBackendConfig {
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
    fn default() -> Self {
        Self {
            enabled: true,
            default_retention: Duration::from_secs(86400 * 30), // 30 days
            policies: HashMap::new(),
        }
    }
}

impl Default for CompressionSettings {
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
