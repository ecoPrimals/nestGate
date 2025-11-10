//
// This module consolidates all ZFS-related types into a single, canonical location,
// replacing fragmented type definitions across multiple modules.

use crate::error::{create_zfs_error, ZfsOperation};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::SystemTime;

// Re-export core storage tier for convenience
pub use nestgate_core::canonical_types::StorageTier;

// ==================== SECTION: CAPACITY MONITORING TYPES ====================

/// Bottleneck detection report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BottleneckReport {
    pub dataset: String,
    pub bottleneck_type: String,
    pub severity: String,
    pub recommendations: Vec<String>,
}
/// Capacity monitoring report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapacityReport {
    pub dataset: String,
    pub current_usage: u64,
    pub projected_usage: u64,
    pub recommendations: Vec<String>,
}
/// Maintenance scheduling information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaintenanceSchedule {
    pub dataset: String,
    pub next_maintenance: SystemTime,
    pub tasks: Vec<String>,
}
/// System information for monitoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    pub timestamp: SystemTime,
    pub cpu_usage: f64,
    pub memory_usage: f64,
    pub disk_usage: f64,
}
/// Replication performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplicationPerformance {
    pub source_dataset: String,
    pub target_dataset: String,
    pub transfer_rate: f64,
    pub compression_ratio: f64,
    pub estimated_completion: SystemTime,
}
/// Snapshot retention policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetentionPolicy {
    pub name: String,
    pub keep_hourly: u32,
    pub keep_daily: u32,
    pub keep_weekly: u32,
    pub keep_monthly: u32,
}
// ==================== SECTION: CANONICAL ERROR TYPES ====================

// DEPRECATED: pub type ZfsResult<T> = Result<T, ZfsError>;
// USE INSTEAD: nestgate_core::error::Result<T> or nestgate_core::error::ZfsResult<T>
//
// MIGRATION PATH:
// ZfsResult<T> → nestgate_core::error::ZfsResult<T>
// This provides unified error handling across the entire ecosystem

pub use nestgate_core::error::Result;

// ==================== SECTION ====================

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
// ==================== SECTION ====================

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
// ==================== SECTION ====================

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
// ==================== SECTION ====================

/// ZFS snapshot information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotInfo {
    pub name: String,
    pub dataset: String,
    pub size: u64,
    pub properties: HashMap<String, String>,
    pub created_at: SystemTime,
}
// ==================== SECTION ====================

/// Zero-cost pool information for compile-time optimization
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
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
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
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
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ZeroCostSnapshotInfo {
    pub name: String,
    pub dataset: String,
    pub size: u64,
    pub properties: HashMap<String, String>,
    pub created_at: SystemTime,
}
// ==================== SECTION ====================

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
// ==================== SECTION ====================

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
// ==================== SECTION ====================

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

// ==================== SECTION ====================

impl From<nestgate_core::error::NestGateError> for ZfsError {
    fn from(err: nestgate_core::error::NestGateError) -> Self {
        match err {
            nestgate_core::error::NestGateError::Configuration(details) => ZfsError::ConfigError {
                message: details.message,
            },
            nestgate_core::error::NestGateError::Storage(details) => {
                ZfsError::Io(std::io::Error::other(details.message))
            }
            nestgate_core::error::NestGateError::Internal(details) => ZfsError::CommandError {
                message: details.message,
            },
            _ => ZfsError::CommandError {
                message: "ZFS operation failed".to_string().to_string(),
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
            ZfsError::PoolError { message } => create_zfs_error(message, ZfsOperation::PoolCreate),
            ZfsError::DatasetError { message } => {
                create_zfs_error(message, ZfsOperation::DatasetCreate)
            }
            ZfsError::SnapshotError { message } => {
                create_zfs_error(message, ZfsOperation::SnapshotCreate)
            }
            ZfsError::CommandError { message } => create_zfs_error(message, ZfsOperation::Command),
            ZfsError::ConfigError { message } => {
                nestgate_core::NestGateError::storage_error(&message)
            }
            ZfsError::Io(io_err) => {
                nestgate_core::NestGateError::storage_error(&format!("IO error: {io_err}"))
            }
        }
    }
}

// ==================== SECTION ====================

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

// ==================== SECTION ====================

/// Create a pool info from raw ZFS output
pub fn pool_info_from_zfs_output(name: &str, output: &str) -> Result<PoolInfo> {
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
pub fn dataset_info_from_zfs_output(output: &str) -> Result<DatasetInfo> {
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
            message: "Pool creation failed".to_string().to_string(),
        };
        let nestgate_err: nestgate_core::NestGateError = zfs_err.into();
        assert!(nestgate_err.to_string().contains("Pool creation failed"));
    }

    // ==================== EXTENDED TEST COVERAGE ====================

    #[test]
    fn test_pool_health_all_variants() {
        let healthy = PoolHealth::Healthy;
        let warning = PoolHealth::Warning;
        let critical = PoolHealth::Critical;
        let unknown = PoolHealth::Unknown;

        // Verify all variants can be created and cloned
        assert!(matches!(healthy.clone(), PoolHealth::Healthy));
        assert!(matches!(warning.clone(), PoolHealth::Warning));
        assert!(matches!(critical.clone(), PoolHealth::Critical));
        assert!(matches!(unknown.clone(), PoolHealth::Unknown));
    }

    #[test]
    fn test_pool_state_all_variants() {
        let states = vec![
            PoolState::Online,
            PoolState::Offline,
            PoolState::Degraded,
            PoolState::Faulted,
            PoolState::Removed,
            PoolState::Unavailable,
        ];

        // Verify all states can be created
        assert_eq!(states.len(), 6);
        for state in states {
            let _cloned = state.clone();
        }
    }

    #[test]
    fn test_pool_capacity_calculations() {
        let capacity = PoolCapacity {
            total_bytes: 1_000_000,
            used_bytes: 600_000,
            available_bytes: 400_000,
            fragmentation_percent: 15.5,
            deduplication_ratio: 1.2,
        };

        assert_eq!(capacity.total_bytes, 1_000_000);
        assert_eq!(
            capacity.used_bytes + capacity.available_bytes,
            capacity.total_bytes
        );
        assert!(capacity.fragmentation_percent > 0.0);
        assert!(capacity.deduplication_ratio >= 1.0);
    }

    #[test]
    fn test_pool_info_with_properties() {
        let mut properties = HashMap::new();
        properties.insert("compression".to_string(), "lz4".to_string());
        properties.insert("atime".to_string(), "off".to_string());

        let pool = PoolInfo {
            name: "test-pool".to_string(),
            size: 1_000_000,
            used: 500_000,
            available: 500_000,
            health: PoolHealth::Healthy,
            state: PoolState::Online,
            capacity: PoolCapacity {
                total_bytes: 1_000_000,
                used_bytes: 500_000,
                available_bytes: 500_000,
                fragmentation_percent: 10.0,
                deduplication_ratio: 1.0,
            },
            properties: properties.clone(),
            created_at: SystemTime::now(),
        };

        assert_eq!(pool.name, "test-pool");
        assert_eq!(pool.properties.get("compression").unwrap(), "lz4");
        assert!(pool.properties.contains_key("atime"));
    }

    #[test]
    fn test_dataset_info_with_mount_point() {
        let dataset = DatasetInfo {
            name: "test-dataset".to_string(),
            full_name: "pool/test-dataset".to_string(),
            pool: "pool".to_string(),
            size: 100_000,
            used: 50_000,
            available: 50_000,
            mount_point: Some(PathBuf::from("/mnt/test")),
            compression: "lz4".to_string(),
            checksum: "sha256".to_string(),
            tier: StorageTier::Hot,
            properties: HashMap::new(),
            created_at: SystemTime::now(),
        };

        assert_eq!(dataset.name, "test-dataset");
        assert!(dataset.mount_point.is_some());
        assert_eq!(dataset.mount_point.unwrap(), PathBuf::from("/mnt/test"));
        assert!(matches!(dataset.tier, StorageTier::Hot));
    }

    #[test]
    fn test_snapshot_info_creation() {
        let mut props = HashMap::new();
        props.insert("used".to_string(), "1024".to_string());

        let snapshot = SnapshotInfo {
            name: "snap1".to_string(),
            dataset: "pool/dataset".to_string(),
            size: 1024,
            properties: props,
            created_at: SystemTime::now(),
        };

        assert_eq!(snapshot.name, "snap1");
        assert_eq!(snapshot.dataset, "pool/dataset");
        assert_eq!(snapshot.size, 1024);
        assert!(snapshot.properties.contains_key("used"));
    }

    #[test]
    fn test_command_result_success() {
        let result = CommandResult {
            success: true,
            stdout: "Operation completed".to_string(),
            stderr: String::new(),
            exit_code: Some(0),
        };

        assert!(result.success);
        assert_eq!(result.exit_code, Some(0));
        assert!(result.stderr.is_empty());
        assert!(!result.stdout.is_empty());
    }

    #[test]
    fn test_command_result_failure() {
        let result = CommandResult {
            success: false,
            stdout: String::new(),
            stderr: "Command failed: pool not found".to_string(),
            exit_code: Some(1),
        };

        assert!(!result.success);
        assert_eq!(result.exit_code, Some(1));
        assert!(!result.stderr.is_empty());
        assert!(result.stderr.contains("pool not found"));
    }

    #[test]
    fn test_zfs_command_variants() {
        let create_pool = ZfsCommand::CreatePool {
            name: "test-pool".to_string(),
            devices: vec!["/dev/disk1".to_string(), "/dev/disk2".to_string()],
        };

        let create_dataset = ZfsCommand::CreateDataset {
            name: "pool/dataset".to_string(),
            properties: HashMap::new(),
        };

        let create_snapshot = ZfsCommand::CreateSnapshot {
            dataset: "pool/dataset".to_string(),
            name: "snap1".to_string(),
        };

        // Verify all command types can be created
        assert!(matches!(create_pool, ZfsCommand::CreatePool { .. }));
        assert!(matches!(create_dataset, ZfsCommand::CreateDataset { .. }));
        assert!(matches!(create_snapshot, ZfsCommand::CreateSnapshot { .. }));
    }

    #[test]
    fn test_zfs_error_types() {
        let pool_err = ZfsError::PoolError {
            message: "Pool error".to_string(),
        };
        let dataset_err = ZfsError::DatasetError {
            message: "Dataset error".to_string(),
        };
        let snapshot_err = ZfsError::SnapshotError {
            message: "Snapshot error".to_string(),
        };
        let command_err = ZfsError::CommandError {
            message: "Command error".to_string(),
        };
        let config_err = ZfsError::ConfigError {
            message: "Config error".to_string(),
        };

        // Verify error messages
        assert!(pool_err.to_string().contains("Pool error"));
        assert!(dataset_err.to_string().contains("Dataset error"));
        assert!(snapshot_err.to_string().contains("Snapshot error"));
        assert!(command_err.to_string().contains("Command error"));
        assert!(config_err.to_string().contains("Config error"));
    }

    #[test]
    fn test_bottleneck_report() {
        let report = BottleneckReport {
            dataset: "pool/dataset".to_string(),
            bottleneck_type: "IO".to_string(),
            severity: "High".to_string(),
            recommendations: vec!["Increase cache".to_string(), "Add more disks".to_string()],
        };

        assert_eq!(report.dataset, "pool/dataset");
        assert_eq!(report.bottleneck_type, "IO");
        assert_eq!(report.recommendations.len(), 2);
    }

    #[test]
    fn test_capacity_report() {
        let report = CapacityReport {
            dataset: "pool/dataset".to_string(),
            current_usage: 800_000,
            projected_usage: 950_000,
            recommendations: vec!["Add storage".to_string()],
        };

        assert!(report.projected_usage > report.current_usage);
        assert!(!report.recommendations.is_empty());
    }

    #[test]
    fn test_retention_policy() {
        let policy = RetentionPolicy {
            name: "standard".to_string(),
            keep_hourly: 24,
            keep_daily: 7,
            keep_weekly: 4,
            keep_monthly: 12,
        };

        assert_eq!(policy.name, "standard");
        assert_eq!(policy.keep_hourly, 24);
        assert_eq!(policy.keep_daily, 7);
        assert_eq!(policy.keep_weekly, 4);
        assert_eq!(policy.keep_monthly, 12);
    }

    #[test]
    fn test_zero_cost_pool_info() {
        let pool = ZeroCostPoolInfo {
            name: "zero-pool".to_string(),
            size: 1_000_000,
            used: 400_000,
            available: 600_000,
            health: "healthy".to_string(),
            properties: HashMap::new(),
            created_at: SystemTime::now(),
        };

        assert_eq!(pool.name, "zero-pool");
        assert_eq!(pool.health, "healthy");
        assert!(pool.used + pool.available <= pool.size);
    }

    #[test]
    fn test_zero_cost_dataset_info() {
        let dataset = ZeroCostDatasetInfo {
            name: "zero-dataset".to_string(),
            full_name: "pool/zero-dataset".to_string(),
            pool: "pool".to_string(),
            size: 50_000,
            used: 25_000,
            available: 25_000,
            mount_point: Some(PathBuf::from("/mnt/zero")),
            tier: StorageTier::Cold,
            properties: HashMap::new(),
            created_at: SystemTime::now(),
        };

        assert_eq!(dataset.name, "zero-dataset");
        assert!(matches!(dataset.tier, StorageTier::Cold));
    }

    #[test]
    fn test_zero_cost_snapshot_info() {
        let snapshot = ZeroCostSnapshotInfo {
            name: "zero-snap".to_string(),
            dataset: "pool/dataset".to_string(),
            size: 2048,
            properties: HashMap::new(),
            created_at: SystemTime::now(),
        };

        assert_eq!(snapshot.name, "zero-snap");
        assert_eq!(snapshot.size, 2048);
    }

    #[test]
    fn test_replication_performance() {
        let perf = ReplicationPerformance {
            source_dataset: "source/dataset".to_string(),
            target_dataset: "target/dataset".to_string(),
            transfer_rate: 1024.0 * 1024.0, // 1 MB/s
            compression_ratio: 2.5,
            estimated_completion: SystemTime::now(),
        };

        assert!(perf.transfer_rate > 0.0);
        assert!(perf.compression_ratio > 1.0);
    }

    #[test]
    fn test_maintenance_schedule() {
        let schedule = MaintenanceSchedule {
            dataset: "pool/dataset".to_string(),
            next_maintenance: SystemTime::now(),
            tasks: vec!["scrub".to_string(), "backup".to_string()],
        };

        assert_eq!(schedule.tasks.len(), 2);
        assert!(schedule.tasks.contains(&"scrub".to_string()));
    }

    #[test]
    fn test_retention_policy_serialization() {
        let policy = RetentionPolicy {
            name: "standard".to_string(),
            keep_hourly: 24,
            keep_daily: 7,
            keep_weekly: 4,
            keep_monthly: 12,
        };

        // Test JSON serialization
        let json = serde_json::to_string(&policy).expect("Failed to serialize");
        let deserialized: RetentionPolicy =
            serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(policy.name, deserialized.name);
        assert_eq!(policy.keep_hourly, deserialized.keep_hourly);
    }

    #[test]
    fn test_replication_performance_metrics() {
        let perf = ReplicationPerformance {
            source_dataset: "source/dataset".to_string(),
            target_dataset: "target/dataset".to_string(),
            transfer_rate: 1024.0 * 1024.0 * 10.0, // 10 MB/s
            compression_ratio: 3.0,
            estimated_completion: SystemTime::now(),
        };

        assert_eq!(perf.source_dataset, "source/dataset");
        assert_eq!(perf.target_dataset, "target/dataset");
        assert!(perf.transfer_rate > 0.0);
        assert!(perf.compression_ratio > 1.0);
    }

    #[test]
    fn test_maintenance_schedule_clone() {
        let schedule = MaintenanceSchedule {
            dataset: "pool/dataset".to_string(),
            next_maintenance: SystemTime::now(),
            tasks: vec!["scrub".to_string(), "backup".to_string()],
        };

        let cloned = schedule.clone();
        assert_eq!(schedule.dataset, cloned.dataset);
        assert_eq!(schedule.tasks.len(), cloned.tasks.len());
    }
}

// ==================== SECTION: CANONICAL ZFS CONFIGURATION ====================

/// **CANONICAL ZFS CONFIGURATION RE-EXPORTS**
///
/// Re-export canonical ZFS configuration types from nestgate-core for convenience.
/// These replace the old fragmented configuration types.
///
/// **Migration Date**: November 7, 2025
/// **Pattern**: Following proven NetworkConfig migration success
// Re-export canonical ZFS configuration types
pub use nestgate_core::config::canonical_primary::domains::storage_canonical::{
    AlertThresholds,
    // Supporting configuration types
    // Note: RetentionPolicy is already defined in this file (line 57)
    ArcCacheConfig,
    L2ArcConfig,
    PrefetchConfig,
    // Configuration enums
    ZfsCompression,
    ZfsDatasetConfig,
    ZfsMaintenanceConfig,
    ZfsMigrationConfig,

    ZfsMonitoringConfig,
    ZfsPerformanceConfig,
    // Core configuration structs
    ZfsPoolConfig,
    ZfsPoolSettings,

    ZfsRedundancy,
    ZfsSecurityConfig,
    ZfsSnapshotConfig,
    // Main ZFS configuration
    ZfsStorageConfig,

    ZilConfig,
};

// Type alias for backward compatibility with code expecting "ZfsConfig"
pub use ZfsStorageConfig as CanonicalZfsConfig;
