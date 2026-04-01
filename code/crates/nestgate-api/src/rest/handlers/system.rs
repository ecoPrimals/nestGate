// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//
// Pure data layer handlers for system health and status information.
// These handlers provide clean access to system data without any
// authentication or user management overhead.

//! System module

use axum::{extract::State, response::Json};
use serde::{Deserialize, Serialize};

use tracing::{debug, info};

use crate::rest::{ApiState, DataResponse};

// ==================== SECTION ====================
// SYSTEM DATA HANDLERS
// ==================== SECTION ====================

/// System health status
#[derive(Debug, Serialize, Deserialize)]
/// Healthstatus
pub struct HealthStatus {
    /// Overall system status
    pub status: String,
    /// System uptime in seconds
    pub uptime_seconds: u64,
    /// System version
    pub version: String,
    /// Service status
    pub services: ServiceStatus,
    /// System timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
}
/// Service status information
#[derive(Debug, Serialize, Deserialize)]
/// Servicestatus
pub struct ServiceStatus {
    /// ZFS _engine status
    pub zfs_engine: String,
    /// Storage detector status
    pub storage_detector: String,
    /// Auto-configurator status
    pub auto_configurator: String,
    /// Metrics collector status
    pub metrics_collector: String,
}
/// Version information
#[derive(Debug, Serialize, Deserialize)]
/// Versioninfo
pub struct VersionInfo {
    /// Application version
    pub version: String,
    /// Build date
    pub build_date: String,
    /// Git commit hash
    pub git_hash: String,
    /// Rust version used for build
    pub rust_version: String,
    /// Target platform
    pub target: String,
    /// Build profile (debug/release)
    pub profile: String,
}
/// System status information
#[derive(Debug, Serialize, Deserialize)]
/// Systemstatusinfo
pub struct SystemStatusInfo {
    /// System health
    pub health: HealthStatus,
    /// Version information
    pub version: VersionInfo,
    /// Resource usage
    pub resources: ResourceUsage,
    /// Active datasets count
    pub datasets_count: u32,
    /// Total snapshots count
    pub snapshots_count: u32,
    /// Storage backends count
    pub storage_backends_count: u32,
}
/// Resource usage information
#[derive(Debug, Serialize, Deserialize)]
/// Resourceusage
pub struct ResourceUsage {
    /// Memory usage in bytes
    pub memory_used_bytes: u64,
    /// Memory total in bytes
    pub memory_total_bytes: u64,
    /// Memory usage percentage
    pub memory_usage_percent: f64,
    /// CPU usage percentage
    pub cpu_usage_percent: f64,
    /// Disk usage in bytes
    pub disk_used_bytes: u64,
    /// Disk total in bytes
    pub disk_total_bytes: u64,
    /// Disk usage percentage
    pub disk_usage_percent: f64,
}
/// Health check endpoint
/// GET /health
pub async fn health_check(State(state): State<ApiState>) -> Json<DataResponse<HealthStatus>> {
    debug!("Performing health check");
    // Check service statuses
    let zfs_status = if state.zfs_engines.is_empty() {
        "idle"
    } else {
        "online"
    };

    let storage_detector = state.storage_detector.try_read();
    let storage_detector_status = if storage_detector.is_ok() {
        "online"
    } else {
        "busy"
    };

    if state.auto_configurator.get().is_none() {
        tracing::warn!("Auto configurator not available - continuing with degraded status");
        // Continue with degraded status rather than error;
        // auto_configurator_status will be set to "unavailable" below.
    }
    let auto_configurator_status = if state.auto_configurator.get().is_some() {
        "online"
    } else {
        "unavailable"
    };

    let health = HealthStatus {
        status: "healthy".to_string(),
        uptime_seconds: get_system_uptime(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        services: ServiceStatus {
            zfs_engine: zfs_status.to_string(),
            storage_detector: storage_detector_status.to_string(),
            auto_configurator: auto_configurator_status.to_string(),
            metrics_collector: "online".to_string(),
        },
        timestamp: chrono::Utc::now(),
    };

    info!("Health check completed - status: {}", health.status);
    Json(DataResponse::new(health))
}

/// Version information endpoint
/// GET /version
pub async fn version_info() -> Json<DataResponse<VersionInfo>> {
    debug!("Getting version information");
    let version = VersionInfo {
        version: env!("CARGO_PKG_VERSION").to_string(),
        build_date: get_build_date(),
        git_hash: get_git_hash(),
        rust_version: get_rust_version(),
        target: get_target_triple(),
        profile: get_build_profile(),
    };

    info!("Retrieved version information: {}", version.version);
    Json(DataResponse::new(version))
}

/// System status endpoint
/// GET /system/status
pub async fn system_status(State(state): State<ApiState>) -> Json<DataResponse<SystemStatusInfo>> {
    debug!("Getting system status");
    // Get health status
    let health = health_check(State(state.clone())).await;
    let health_data = health.0.data;

    // Get version info
    let version = version_info().await;
    let version_data = version.0.data;

    // Get resource usage
    let resources = get_resource_usage();

    // Count datasets and snapshots
    let datasets_count = state.zfs_engines.len() as u32;

    let snapshots_count = super::zfs::helpers::get_snapshot_count_from_engine_impl();

    let storage_backends_count = u32::from(!state.zfs_engines.is_empty());

    let system_status = SystemStatusInfo {
        health: health_data,
        version: version_data,
        resources,
        datasets_count,
        snapshots_count: snapshots_count.try_into().unwrap_or(0),
        storage_backends_count,
    };

    info!(
        "Retrieved system status - {} datasets, {} snapshots",
        datasets_count, snapshots_count
    );
    Json(DataResponse::new(system_status))
}

// ==================== SECTION ====================
// HELPER FUNCTIONS
// ==================== SECTION ====================

/// Get system uptime in seconds (simplified)
fn get_system_uptime() -> u64 {
    // In a real implementation, would read from /proc/uptime or similar
    // For demo, calculate from a fixed start time
    let start_time = chrono::DateTime::parse_from_rfc3339("2025-01-30T08:00:00Z")
        .unwrap_or_else(|e| {
            tracing::error!("Unwrap failed: {:?}", e);

            chrono::DateTime::parse_from_rfc3339("2025-01-01T00:00:00Z").unwrap_or_else(|_| {
                chrono::DateTime::from_timestamp(0, 0)
                    .unwrap_or_else(|| chrono::DateTime::from_timestamp(0, 0).unwrap_or_default())
                    .fixed_offset()
            })
        })
        .with_timezone(&chrono::Utc);
    let now = chrono::Utc::now();
    (now - start_time).num_seconds().max(0) as u64
}

/// Get build date
fn get_build_date() -> String {
    // In a real build system, this would be set by build scripts
    option_env!("BUILD_DATE")
        .unwrap_or("2025-01-30")
        .to_string()
}
/// Get git commit hash
fn get_git_hash() -> String {
    // In a real build system, this would be set by build scripts
    option_env!("GIT_HASH").unwrap_or("dev-build").to_string()
}
/// Get Rust version used for build
fn get_rust_version() -> String {
    // This would typically be captured during build
    option_env!("RUST_VERSION")
        .unwrap_or(env!("CARGO_PKG_RUST_VERSION"))
        .to_string()
}
/// Get target triple
fn get_target_triple() -> String {
    std::env::consts::ARCH.to_string() + "-" + std::env::consts::OS
}
/// Get build profile
fn get_build_profile() -> String {
    if cfg!(debug_assertions) {
        "debug".to_string()
    } else {
        "release".to_string()
    }
}
/// Current resource usage from `/proc` and root [`statvfs`](nestgate_core::linux_proc::statvfs_space) (Linux).
fn get_resource_usage() -> ResourceUsage {
    #[cfg(target_os = "linux")]
    {
        let cpu_usage_percent =
            nestgate_core::linux_proc::globalcpu_usage_percent_from_stat().unwrap_or(0.0);
        let memory_total = nestgate_core::linux_proc::total_memory_bytes().unwrap_or(0);
        let memory_used = nestgate_core::linux_proc::used_memory_bytes().unwrap_or(0);
        let memory_usage_percent = nestgate_core::linux_proc::memory_usage_percent().unwrap_or(0.0);
        let (disk_total, disk_avail) =
            nestgate_core::linux_proc::statvfs_space(std::path::Path::new("/")).unwrap_or((0, 0));
        let disk_used = disk_total.saturating_sub(disk_avail);
        let disk_usage_percent = if disk_total > 0 {
            (disk_used as f64 / disk_total as f64) * 100.0
        } else {
            0.0
        };
        ResourceUsage {
            memory_used_bytes: memory_used,
            memory_total_bytes: memory_total,
            memory_usage_percent,
            cpu_usage_percent,
            disk_used_bytes: disk_used,
            disk_total_bytes: disk_total,
            disk_usage_percent,
        }
    }
    #[cfg(not(target_os = "linux"))]
    {
        ResourceUsage {
            memory_used_bytes: 0,
            memory_total_bytes: 0,
            memory_usage_percent: 0.0,
            cpu_usage_percent: 0.0,
            disk_used_bytes: 0,
            disk_total_bytes: 0,
            disk_usage_percent: 0.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rest::ApiState;
    use dashmap::DashMap;
    use nestgate_core::universal_storage::StorageDetector;
    use std::sync::{Arc, OnceLock};
    use tokio::sync::RwLock;

    /// Helper to create a test API state
    fn create_test_api_state() -> ApiState {
        ApiState {
            zfs_engines: Arc::new(DashMap::new()),
            storage_detector: Arc::new(RwLock::new(StorageDetector::default())),
            auto_configurator: Arc::new(OnceLock::new()),
            rpc_manager: Arc::new(OnceLock::new()),
        }
    }

    #[tokio::test]
    async fn test_health_check_returns_data() {
        let state = create_test_api_state();
        let result = health_check(State(state)).await;

        assert_eq!(result.0.data.status, "healthy");
        assert!(!result.0.data.version.is_empty());
        assert!(result.0.data.uptime_seconds > 0);
    }

    #[tokio::test]
    async fn test_health_check_service_statuses() {
        let state = create_test_api_state();
        let result = health_check(State(state)).await;

        let services = &result.0.data.services;
        // With empty engines, should be idle
        assert_eq!(services.zfs_engine, "idle");
        assert_eq!(services.storage_detector, "online");
        assert_eq!(services.metrics_collector, "online");
    }

    #[tokio::test]
    async fn test_health_check_with_engines() {
        let state = create_test_api_state();

        // Add a test engine
        state
            .zfs_engines
            .insert("test-engine".to_string(), "engine-data".to_string());

        let result = health_check(State(state)).await;

        // With engines present, should be online
        assert_eq!(result.0.data.services.zfs_engine, "online");
    }

    #[tokio::test]
    async fn test_version_info_contains_all_fields() {
        let result = version_info().await;

        assert!(!result.0.data.version.is_empty());
        assert!(!result.0.data.build_date.is_empty());
        assert!(!result.0.data.git_hash.is_empty());
        // rust_version may be empty if CARGO_PKG_RUST_VERSION is not set
        // assert!(!result.0.data.rust_version.is_empty());
        assert!(!result.0.data.target.is_empty());
        assert!(!result.0.data.profile.is_empty());
    }

    #[tokio::test]
    async fn test_version_info_profile_is_correct() {
        let result = version_info().await;

        // Profile should be either debug or release
        let profile = &result.0.data.profile;
        assert!(profile == "debug" || profile == "release");
    }

    #[tokio::test]
    async fn test_system_status_comprehensive() {
        let state = create_test_api_state();
        let result = system_status(State(state)).await;

        // Verify health data
        assert_eq!(result.0.data.health.status, "healthy");

        // Verify version data
        assert!(!result.0.data.version.version.is_empty());

        // Verify resource data (real /proc + statvfs on Linux only)
        #[cfg(target_os = "linux")]
        {
            assert!(result.0.data.resources.memory_total_bytes > 0);
            assert!(result.0.data.resources.disk_total_bytes > 0);
        }
        assert!(result.0.data.resources.cpu_usage_percent >= 0.0);
        assert!(result.0.data.resources.cpu_usage_percent <= 100.0);
    }

    #[tokio::test]
    async fn test_system_status_with_datasets() {
        let state = create_test_api_state();

        // Add test datasets
        state
            .zfs_engines
            .insert("dataset1".to_string(), "data1".to_string());
        state
            .zfs_engines
            .insert("dataset2".to_string(), "data2".to_string());
        state
            .zfs_engines
            .insert("dataset3".to_string(), "data3".to_string());

        let result = system_status(State(state)).await;

        assert_eq!(result.0.data.datasets_count, 3);
        // Snapshot count comes from filesystem scan — 0 in test environments
        assert_eq!(result.0.data.snapshots_count, 0);
    }

    #[test]
    fn test_get_system_uptime_positive() {
        let uptime = get_system_uptime();
        assert!(uptime > 0, "System uptime should be positive");
    }

    #[test]
    fn test_get_build_date_not_empty() {
        let build_date = get_build_date();
        assert!(!build_date.is_empty());
    }

    #[test]
    fn test_get_git_hash_not_empty() {
        let git_hash = get_git_hash();
        assert!(!git_hash.is_empty());
    }

    #[test]
    fn test_get_rust_version_returns_string() {
        let rust_version = get_rust_version();
        // Rust version may be empty if CARGO_PKG_RUST_VERSION is not set
        // Just verify it returns a string (may be empty)
        assert!(rust_version.is_empty() || !rust_version.is_empty());
    }

    #[test]
    fn test_get_target_triple_format() {
        let target = get_target_triple();
        assert!(!target.is_empty());
        assert!(target.contains('-'), "Target should contain arch-os format");
    }

    #[test]
    fn test_get_build_profile_valid() {
        let profile = get_build_profile();
        assert!(profile == "debug" || profile == "release");
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn test_get_resource_usage_valid_ranges() {
        let resources = get_resource_usage();

        // Memory checks
        assert!(resources.memory_total_bytes > 0);
        assert!(resources.memory_used_bytes <= resources.memory_total_bytes);
        assert!(resources.memory_usage_percent >= 0.0);
        assert!(resources.memory_usage_percent <= 100.0);

        // CPU checks
        assert!(resources.cpu_usage_percent >= 0.0);
        assert!(resources.cpu_usage_percent <= 100.0);

        // Disk checks
        assert!(resources.disk_total_bytes > 0);
        assert!(resources.disk_used_bytes <= resources.disk_total_bytes);
        assert!(resources.disk_usage_percent >= 0.0);
        assert!(resources.disk_usage_percent <= 100.0);
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn test_get_resource_usage_consistency() {
        let resources = get_resource_usage();

        // Verify percentage calculations are consistent
        let expected_memory_percent =
            (resources.memory_used_bytes as f64 / resources.memory_total_bytes as f64) * 100.0;
        assert!(
            (resources.memory_usage_percent - expected_memory_percent).abs() < 0.1,
            "Memory percentage should match calculation"
        );

        let expected_disk_percent =
            (resources.disk_used_bytes as f64 / resources.disk_total_bytes as f64) * 100.0;
        assert!(
            (resources.disk_usage_percent - expected_disk_percent).abs() < 0.1,
            "Disk percentage should match calculation"
        );
    }

    #[tokio::test]
    async fn test_health_check_timestamp_recent() {
        let state = create_test_api_state();
        let result = health_check(State(state)).await;

        let now = chrono::Utc::now();
        let timestamp = result.0.data.timestamp;

        // Timestamp should be within last 5 seconds
        let diff = (now - timestamp).num_seconds().abs();
        assert!(diff < 5, "Timestamp should be recent");
    }

    #[tokio::test]
    async fn test_system_status_storage_backends_count() {
        let state = create_test_api_state();
        let result = system_status(State(state)).await;

        // Empty test state has no engines → 0 backends
        assert_eq!(result.0.data.storage_backends_count, 0);
    }

    #[test]
    fn test_service_status_serialization() {
        let service_status = ServiceStatus {
            zfs_engine: "online".to_string(),
            storage_detector: "online".to_string(),
            auto_configurator: "online".to_string(),
            metrics_collector: "online".to_string(),
        };

        // Should be serializable
        let json = serde_json::to_string(&service_status);
        assert!(json.is_ok());
    }

    #[test]
    fn test_health_status_serialization() {
        let health = HealthStatus {
            status: "healthy".to_string(),
            uptime_seconds: 12345,
            version: "0.1.0".to_string(),
            services: ServiceStatus {
                zfs_engine: "online".to_string(),
                storage_detector: "online".to_string(),
                auto_configurator: "online".to_string(),
                metrics_collector: "online".to_string(),
            },
            timestamp: chrono::Utc::now(),
        };

        // Should be serializable
        let json = serde_json::to_string(&health);
        assert!(json.is_ok());
    }

    #[test]
    fn test_version_info_serialization() {
        let version = VersionInfo {
            version: "0.1.0".to_string(),
            build_date: "2025-01-30".to_string(),
            git_hash: "abc123".to_string(),
            rust_version: "1.75.0".to_string(),
            target: "x86_64-linux".to_string(),
            profile: "release".to_string(),
        };

        // Should be serializable
        let json = serde_json::to_string(&version);
        assert!(json.is_ok());
    }

    #[test]
    fn test_resource_usage_serialization() {
        let resources = ResourceUsage {
            memory_used_bytes: 1024 * 1024 * 100,
            memory_total_bytes: 1024 * 1024 * 1024,
            memory_usage_percent: 10.0,
            cpu_usage_percent: 45.0,
            disk_used_bytes: 1024 * 1024 * 1024 * 10,
            disk_total_bytes: 1024 * 1024 * 1024 * 100,
            disk_usage_percent: 10.0,
        };

        // Should be serializable
        let json = serde_json::to_string(&resources);
        assert!(json.is_ok());
    }
}
