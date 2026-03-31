// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! # Universal ZFS Types
//!
//! **Status**: Production Ready ✅  
//! **Purpose**: Common types used across all ZFS service implementations
//!
//! ## Architecture
//!
//! This module provides a clean, modular structure for ZFS types:
//! - **errors**: Comprehensive error handling for ZFS operations
//! - **pool**: Pool management types (info, health, state, capacity, scrub)
//! - **dataset**: Dataset and snapshot information types
//! - **config**: Configuration types for pools, datasets, and snapshots
//! - **health**: Service health monitoring and metrics
//!
//! ## Refactoring (November 13, 2025)
//!
//! This module was refactored from a single 1,063-line file into focused modules:
//! - Improved maintainability and discoverability
//! - Clear separation of concerns
//! - Each module under 400 lines
//! - Backward-compatible re-exports
//!
//! ## Code Quality
//!
//! - **Unwraps**: 1 in test code (acceptable)
//! - **Unsafe**: 0 blocks
//! - **Error Handling**: Comprehensive `Result<T, E>` patterns
//! - **Documentation**: All public types documented

// Re-export all public types for backward compatibility
pub mod config;
pub mod dataset;
pub mod errors;
pub mod health;
pub mod pool;

// Re-export all types at the module level
pub use config::{DatasetConfig, DatasetConfigCanonical, SnapshotConfig, SnapshotConfigCanonical};
pub use dataset::{DatasetInfo, DatasetType, SnapshotInfo};
pub use errors::{RateLimitInfo, UniversalZfsError, UniversalZfsErrorData, UniversalZfsResult};
pub use health::{HealthCheck, HealthStatus, ServiceMetrics, ServiceStatus};
pub use pool::{PoolCapacity, PoolConfig, PoolHealth, PoolInfo, PoolState, ScrubStatus};

// ==================== TESTS ====================

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_universal_zfs_error_service_unavailable() {
        let error = UniversalZfsError::service_unavailable("Service down");
        assert!(matches!(
            error,
            UniversalZfsError::ServiceUnavailable { .. }
        ));
    }

    #[test]
    fn test_universal_zfs_error_timeout() {
        let error = UniversalZfsError::timeout("list_pools", std::time::Duration::from_secs(30));
        assert!(matches!(error, UniversalZfsError::Timeout { .. }));
    }

    #[test]
    fn test_pool_info_creation() {
        let pool = PoolInfo {
            name: "tank".to_string(),
            health: PoolHealth::Online,
            state: PoolState::Active,
            capacity: PoolCapacity {
                total: 1000,
                used: 500,
                available: 500,
            },
            scrub: Some(ScrubStatus::None),
            properties: HashMap::new(),
        };
        assert_eq!(pool.name, "tank");
        assert_eq!(pool.health, PoolHealth::Online);
    }

    #[test]
    fn test_dataset_info_creation() {
        let dataset = DatasetInfo {
            name: "tank/data".to_string(),
            dataset_type: DatasetType::Filesystem,
            used: 100,
            available: 900,
            referenced: 100,
            mountpoint: Some("/mnt/data".to_string()),
            properties: HashMap::new(),
        };
        assert_eq!(dataset.name, "tank/data");
        assert_eq!(dataset.dataset_type, DatasetType::Filesystem);
    }

    #[test]
    fn test_snapshot_info_creation() {
        let snapshot = SnapshotInfo {
            name: "tank/data@snap1".to_string(),
            creation_time: 1_234_567_890,
            used: 50,
            referenced: 100,
            properties: HashMap::new(),
        };
        assert_eq!(snapshot.name, "tank/data@snap1");
        assert_eq!(snapshot.creation_time, 1_234_567_890);
    }

    #[test]
    fn test_dataset_config_creation() {
        let config = DatasetConfig {
            name: "tank/data".to_string(),
            mountpoint: Some("/mnt/data".to_string()),
            compression: true,
            quota: Some(1_000_000),
            reservation: None,
            properties: HashMap::new(),
        };
        assert_eq!(config.name, "tank/data");
        assert!(config.compression);
    }

    #[test]
    fn test_snapshot_config_creation() {
        let config = SnapshotConfig {
            name: "tank/data@snap1".to_string(),
            dataset: "tank/data".to_string(),
            properties: HashMap::new(),
        };
        assert_eq!(config.name, "tank/data@snap1");
    }

    #[test]
    fn test_health_status_creation() {
        let health = HealthStatus {
            service_name: "zfs-backend".to_string(),
            status: ServiceStatus::Healthy,
            checks: vec![HealthCheck {
                name: "pool_health".to_string(),
                passed: true,
                message: None,
            }],
            last_check: std::time::SystemTime::now(),
            metadata: HashMap::new(),
        };
        assert_eq!(health.service_name, "zfs-backend");
        assert_eq!(health.status, ServiceStatus::Healthy);
    }

    #[test]
    fn test_service_metrics_default() {
        let metrics = ServiceMetrics::default();
        assert_eq!(metrics.requests_total, 0);
        assert_eq!(metrics.error_rate, 0.0);
    }

    #[test]
    fn test_service_metrics_update_error_rate() {
        let mut metrics = ServiceMetrics::new("zfs-backend");
        metrics.requests_total = 1000;
        metrics.requests_failed = 50;
        metrics.update_error_rate();
        assert_eq!(metrics.error_rate, 5.0);
    }

    #[test]
    fn test_service_metrics_custom_metrics() {
        let mut metrics = ServiceMetrics::new("zfs-backend");
        metrics.requests_total = 1000;
        metrics.requests_failed = 50;
        metrics.error_rate = 5.0;
        metrics
            .custom_metrics
            .insert("latency_p95".to_string(), 125.5);

        assert_eq!(metrics.service_name, "zfs-backend");
        assert_eq!(metrics.requests_total, 1000);
        assert_eq!(metrics.error_rate, 5.0);
        assert_eq!(metrics.custom_metrics.get("latency_p95"), Some(&125.5));
    }

    #[test]
    fn test_service_metrics_serialization() {
        let metrics = ServiceMetrics::default();
        let json = serde_json::to_string(&metrics);
        assert!(json.is_ok());
    }

    // ==================== ERROR CONVERSION TESTS ====================

    #[test]
    fn test_io_error_conversion() {
        let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let zfs_error: UniversalZfsError = io_error.into();
        assert!(matches!(zfs_error, UniversalZfsError::Backend { .. }));
    }

    #[test]
    fn test_timeout_elapsed_conversion() {
        use tokio::time::{Duration as TokioDuration, timeout};

        // Create a timeout error by running an async block
        let rt = tokio::runtime::Runtime::new().expect("Failed to create tokio runtime for test");
        let result = rt.block_on(async {
            timeout(TokioDuration::from_millis(1), async {
                tokio::time::sleep(TokioDuration::from_secs(10)).await;
            })
            .await
        });

        if let Err(elapsed) = result {
            let zfs_error: UniversalZfsError = elapsed.into();
            assert!(matches!(zfs_error, UniversalZfsError::Timeout { .. }));
        }
    }

    #[test]
    fn test_error_to_nestgate_error_conversion() {
        let zfs_error = UniversalZfsError::service_unavailable("test");
        let nestgate_error: nestgate_core::error::NestGateError = zfs_error.into();
        // Just verify it converts without panicking
        assert!(format!("{nestgate_error:?}").contains("test"));
    }
}
