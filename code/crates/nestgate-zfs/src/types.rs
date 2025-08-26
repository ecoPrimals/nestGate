//
// This module consolidates all ZFS-related types into a single, canonical location,
// replacing fragmented type definitions across multiple modules.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::SystemTime;
use nestgate_core::error::conversions::create_zfs_error;
use nestgate_core::error::domain_errors::ZfsOperation;

// Re-export core storage tier for convenience
pub use nestgate_core::canonical_types::StorageTier;

// ==================== ERROR TYPES ====================

/// ZFS-specific error type for backward compatibility
#[derive(Debug, thiserror::Error)]
pub enum ZfsError {
    #[error("Pool operation failed: {message}")]
    PoolError { message: String },
    #[error("Dataset operation failed: {message}")]
    DatasetError { message: String },
    #[error("Snapshot operation failed: {message}")]
    SnapshotError { message: String },
    #[error("Command execution failed: {message}")]
    CommandError { message: String },
    #[error("Configuration error: {message}")]
    ConfigError { message: String },
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

/// Result type for ZFS operations
/// **CANONICAL**: ZFS-specific Result type using IdioResult
/// This follows the canonical Result<T,E> pattern with domain-specific error type
pub type ZfsResult<T> = Result<T, ZfsError>;

// ==================== POOL TYPES ====================

/// ZFS pool information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolInfo {
    pub name: String,
    pub size: u64,
    pub used: u64,
    pub available: u64,
    pub health: PoolHealth,
    pub state: PoolState,
    pub capacity: PoolCapacity,
    pub properties: HashMap<String, String>,
    pub created_at: SystemTime,
}

/// ZFS pool health status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PoolHealth {
    Healthy,
    Warning,
    Critical,
    Unknown,
}

/// ZFS pool state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PoolState {
    Online,
    Offline,
    Degraded,
    Faulted,
    Removed,
    Unavailable,
}

/// ZFS pool capacity information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolCapacity {
    pub total_bytes: u64,
    pub used_bytes: u64,
    pub available_bytes: u64,
    pub fragmentation_percent: f64,
    pub deduplication_ratio: f64,
}

// ==================== DATASET TYPES ====================

/// ZFS dataset information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetInfo {
    pub name: String,
    pub full_name: String,
    pub pool: String,
    pub size: u64,
    pub used: u64,
    pub available: u64,
    pub mount_point: Option<PathBuf>,
    pub compression: String,
    pub checksum: String,
    pub tier: StorageTier,
    pub properties: HashMap<String, String>,
    pub created_at: SystemTime,
}

// ==================== SNAPSHOT TYPES ====================

/// ZFS snapshot information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotInfo {
    pub name: String,
    pub dataset: String,
    pub size: u64,
    pub properties: HashMap<String, String>,
    pub created_at: SystemTime,
}

// ==================== ZERO-COST TYPES ====================

/// Zero-cost pool information for compile-time optimization
#[derive(Debug, Clone)]
pub struct ZeroCostPoolInfo {
    pub name: String,
    pub size: u64,
    pub used: u64,
    pub available: u64,
    pub health: String,
    pub properties: HashMap<String, String>,
    pub created_at: SystemTime,
}

/// Zero-cost dataset information
#[derive(Debug, Clone)]
pub struct ZeroCostDatasetInfo {
    pub name: String,
    pub full_name: String,
    pub pool: String,
    pub size: u64,
    pub used: u64,
    pub available: u64,
    pub mount_point: Option<PathBuf>,
    pub tier: StorageTier,
    pub properties: HashMap<String, String>,
    pub created_at: SystemTime,
}

/// Zero-cost snapshot information
#[derive(Debug, Clone)]
pub struct ZeroCostSnapshotInfo {
    pub name: String,
    pub dataset: String,
    pub size: u64,
    pub properties: HashMap<String, String>,
    pub created_at: SystemTime,
}

// ==================== COMMAND TYPES ====================

/// ZFS command execution result
#[derive(Debug, Clone)]
pub struct CommandResult {
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
    pub exit_code: Option<i32>,
}

/// ZFS command operations
#[derive(Debug, Clone)]
pub enum ZfsCommand {
    CreatePool {
        name: String,
        devices: Vec<String>,
    },
    CreateDataset {
        name: String,
        properties: HashMap<String, String>,
    },
    CreateSnapshot {
        dataset: String,
        name: String,
    },
    ListPools,
    ListDatasets {
        pool: Option<String>,
    },
    ListSnapshots {
        dataset: Option<String>,
    },
    GetProperties {
        target: String,
    },
    SetProperty {
        target: String,
        property: String,
        value: String,
    },
}

// ==================== NATIVE ZFS TYPES ====================

/// Native ZFS pool status from zpool command
#[derive(Debug, Clone)]
pub enum PoolStatus {
    Online,
    Degraded,
    Faulted,
    Offline,
    Removed,
    Unavailable,
}

// ==================== CONVERSION IMPLEMENTATIONS ====================

impl From<PoolStatus> for PoolHealth {
    fn from(status: PoolStatus) -> Self {
        match status {
            PoolStatus::Online => PoolHealth::Healthy,
            PoolStatus::Degraded => PoolHealth::Warning,
            PoolStatus::Faulted => PoolHealth::Critical,
            PoolStatus::Offline | PoolStatus::Removed | PoolStatus::Unavailable => {
                PoolHealth::Unknown
            }
        }
    }
}

// ==================== ERROR CONVERSION IMPLEMENTATIONS ====================

impl From<nestgate_core::error::NestGateError> for ZfsError {
    fn from(err: nestgate_core::error::NestGateError) -> Self {
        match err {
            nestgate_core::error::NestGateError::Configuration { message, .. } => {
                ZfsError::ConfigError { message }
            }
            nestgate_core::error::NestGateError::Io { error_message, .. } => ZfsError::Io(
                std::io::Error::new(std::io::ErrorKind::Other, error_message),
            ),
            _ => ZfsError::CommandError {
                message: format!("ZFS operation failed: {}", err),
            },
        }
    }
}

impl From<PoolStatus> for PoolState {
    fn from(status: PoolStatus) -> Self {
        match status {
            PoolStatus::Online => PoolState::Online,
            PoolStatus::Degraded => PoolState::Degraded,
            PoolStatus::Faulted => PoolState::Faulted,
            PoolStatus::Offline => PoolState::Offline,
            PoolStatus::Removed => PoolState::Removed,
            PoolStatus::Unavailable => PoolState::Unavailable,
        }
    }
}

impl From<ZfsError> for nestgate_core::NestGateError {
    fn from(err: ZfsError) -> Self {
        match err {
            ZfsError::PoolError { message } => {
                create_zfs_error(message, ZfsOperation::PoolCreate)
            }
            ZfsError::DatasetError { message } => {
                create_zfs_error(message, ZfsOperation::DatasetCreate)
            }
            ZfsError::SnapshotError { message } => {
                create_zfs_error(message, ZfsOperation::SnapshotCreate)
            }
            ZfsError::CommandError { message } => {
                create_zfs_error(message, ZfsOperation::Command)
            }
            ZfsError::ConfigError { message } => nestgate_core::NestGateError::storage_error("zfs_config", message),
            ZfsError::Io(io_err) => {
                nestgate_core::NestGateError::storage_error("zfs_io", &format!("IO error: {io_err}"), None)
            }
        }
    }
}

// ==================== DEFAULT IMPLEMENTATIONS ====================

impl Default for PoolInfo {
    fn default() -> Self {
        Self {
            name: String::new(),
            size: 0,
            used: 0,
            available: 0,
            health: PoolHealth::Unknown,
            state: PoolState::Unavailable,
            capacity: PoolCapacity::default(),
            properties: HashMap::new(),
            created_at: SystemTime::now(),
        }
    }
}

impl Default for PoolCapacity {
    fn default() -> Self {
        Self {
            total_bytes: 0,
            used_bytes: 0,
            available_bytes: 0,
            fragmentation_percent: 0.0,
            deduplication_ratio: 1.0,
        }
    }
}

impl Default for DatasetInfo {
    fn default() -> Self {
        Self {
            name: String::new(),
            full_name: String::new(),
            pool: String::new(),
            size: 0,
            used: 0,
            available: 0,
            mount_point: None,
            compression: "lz4".to_string(),
            checksum: "sha256".to_string(),
            tier: StorageTier::Warm,
            properties: HashMap::new(),
            created_at: SystemTime::now(),
        }
    }
}

impl Default for SnapshotInfo {
    fn default() -> Self {
        Self {
            name: String::new(),
            dataset: String::new(),
            size: 0,
            properties: HashMap::new(),
            created_at: SystemTime::now(),
        }
    }
}

// ==================== UTILITY FUNCTIONS ====================

/// Create a pool info from raw ZFS output
pub fn pool_info_from_zfs_output(name: &str, output: &str) -> ZfsResult<PoolInfo> {
    let mut properties = HashMap::new();

    for line in output.lines() {
        if let Some((key, value)) = line.split_once('\t') {
            properties.insert(key.trim().to_string(), value.trim().to_string());
        }
    }

    let size: u64 = properties
        .get("size")
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);
    let used: u64 = properties
        .get("allocated")
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);
    let available = size.saturating_sub(used);

    let health_str = properties.get("health").map_or("UNKNOWN", |h| h.as_str());
    let health = match health_str {
        "ONLINE" => PoolHealth::Healthy,
        "DEGRADED" => PoolHealth::Warning,
        "FAULTED" | "UNAVAIL" => PoolHealth::Critical,
        _ => PoolHealth::Unknown,
    };

    let state = match health_str {
        "ONLINE" => PoolState::Online,
        "DEGRADED" => PoolState::Degraded,
        "FAULTED" => PoolState::Faulted,
        "OFFLINE" => PoolState::Offline,
        "UNAVAIL" => PoolState::Unavailable,
        _ => PoolState::Unavailable,
    };

    Ok(PoolInfo {
        name: name.to_string(),
        size,
        used,
        available,
        health,
        state,
        capacity: PoolCapacity {
            total_bytes: size,
            used_bytes: used,
            available_bytes: available,
            fragmentation_percent: 0.0, // Would be parsed from detailed output
            deduplication_ratio: 1.0,   // Would be parsed from detailed output
        },
        properties,
        created_at: SystemTime::now(),
    })
}

/// Create dataset info from ZFS output
pub fn dataset_info_from_zfs_output(output: &str) -> ZfsResult<DatasetInfo> {
    let mut properties = HashMap::new();

    for line in output.lines() {
        if let Some((key, value)) = line.split_once('\t') {
            properties.insert(key.trim().to_string(), value.trim().to_string());
        }
    }

    let name = properties.get("name").cloned().unwrap_or_default();
    let pool = name.split('/').next().unwrap_or("").to_string();
    let used: u64 = properties
        .get("used")
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);
    let available: u64 = properties
        .get("available")
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);
    let size = used + available;

    let mount_point = properties
        .get("mountpoint")
        .filter(|mp| *mp != "none" && *mp != "-")
        .map(PathBuf::from);

    Ok(DatasetInfo {
        name: name.split('/').skip(1).collect::<Vec<_>>().join("/"),
        full_name: name,
        pool,
        size,
        used,
        available,
        mount_point,
        compression: properties
            .get("compression")
            .cloned()
            .unwrap_or_else(|| "lz4".to_string()),
        checksum: properties
            .get("checksum")
            .cloned()
            .unwrap_or_else(|| "sha256".to_string()),
        tier: StorageTier::Warm, // Default, would be determined by tier analysis
        properties,
        created_at: SystemTime::now(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pool_info_creation() {
        let pool_info = PoolInfo::default();
        assert_eq!(pool_info.name, "");
        assert_eq!(pool_info.size, 0);
        assert!(matches!(pool_info.health, PoolHealth::Unknown));
    }

    #[test]
    fn test_dataset_info_creation() {
        let dataset_info = DatasetInfo::default();
        assert_eq!(dataset_info.compression, "lz4");
        assert_eq!(dataset_info.checksum, "sha256");
        assert!(matches!(dataset_info.tier, StorageTier::Warm));
    }

    #[test]
    fn test_pool_status_conversion() {
        let health: PoolHealth = PoolStatus::Online.into();
        assert!(matches!(health, PoolHealth::Healthy));

        let state: PoolState = PoolStatus::Degraded.into();
        assert!(matches!(state, PoolState::Degraded));
    }

    #[test]
    fn test_zfs_error_conversion() {
        let zfs_err = ZfsError::PoolError {
            message: "Pool creation failed".to_string(),
        };
        let nestgate_err: nestgate_core::NestGateError = zfs_err.into();
        assert!(nestgate_err.to_string().contains("Pool creation failed"));
    }
}
