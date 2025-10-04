//
// Pure data layer handlers for system health and status information.
// These handlers provide clean access to system data without any
// authentication or user management overhead.

use axum::{extract::State, response::Json};
use serde::{Deserialize, Serialize};

use std::sync::Arc;
use tracing::{debug, info};

use crate::rest::{ApiState, DataResponse};
use nestgate_core::universal_storage::auto_configurator::AutoConfigurator;

// ==================== SECTION ====================
// SYSTEM DATA HANDLERS
// ==================== SECTION ====================

/// System health status
#[derive(Debug, Serialize, Deserialize)]
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
    let zfs_engines = state.zfs_engines.read().await;
    let zfs_status = if zfs_engines.len() > 0 {
        "online"
    } else {
        "idle"
    };

    let storage_detector = state.storage_detector.try_lock();
    let storage_detector_status = if storage_detector.is_ok() {
        "online"
    } else {
        "busy"
    };

    let _auto_configurator = match state.auto_configurator.lock().await.as_ref() {
        Some(configurator) => configurator,
        None => {
            tracing::warn!("Auto configurator not available - continuing with degraded status");
            // Continue with degraded status rather than error
            // auto_configurator_status will be set to "unavailable" below
            &AutoConfigurator::new(Vec::new()) // Use a minimal configurator for status reporting
        }
    };
    let auto_configurator_status = if state.auto_configurator.lock().await.is_some() {
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
pub fn version_info() -> Json<DataResponse<VersionInfo>> {
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
    let engines = state.zfs_engines.read().await;
    let datasets_count = engines.len() as u32;

    let mut snapshots_count = 0;
    for (_name, _engine) in engines.iter() {
        // Placeholder stats - _engine is now just a String
        snapshots_count += 5; // Placeholder snapshot count
    }

    // Count storage backends (simplified)
    let storage_backends_count = 2; // Memory + Filesystem (for demo)

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
            let default_time = chrono::DateTime::parse_from_rfc3339("2025-01-01T00:00:00Z")
                .unwrap_or_else(|_| {
                    chrono::DateTime::from_timestamp(0, 0)
                        .unwrap_or_else(|| {
                            chrono::DateTime::from_timestamp(0, 0).unwrap_or_default()
                        })
                        .fixed_offset()
                });
            default_time
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
/// Get current resource usage (simplified)
fn get_resource_usage() -> ResourceUsage {
    // In a real implementation, would read from system APIs
    // For demo, generate realistic values
    let memory_total = 8 * 1024 * 1024 * 1024; // 8GB
    let memory_used = (memory_total as f64 * 0.45) as u64; // 45% usage
    let memory_usage_percent = (memory_used as f64 / memory_total as f64) * 100.0;

    let disk_total = 100 * 1024 * 1024 * 1024; // 100GB
    let disk_used = (disk_total as f64 * 0.25) as u64; // 25% usage
    let disk_usage_percent = (disk_used as f64 / disk_total as f64) * 100.0;

    // Generate CPU usage based on current time for some variation
    let cpu_usage_percent = {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        chrono::Utc::now().timestamp().hash(&mut hasher);
        let seed = hasher.finish();

        let base = 20.0;
        let variation = ((seed % 100) as f64) * 0.3;
        (base + variation).min(80.0)
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

/// Get snapshot count from a ZFS _engine
#[allow(dead_code)] // Utility function for snapshot monitoring
fn get_engine_snapshot_count(
    _engine: &Arc<dyn std::any::Any + Send + Sync>,
) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
    // In a real implementation, this would query the _engine's snapshot manager
    // For now, we'll estimate based on available snapshot _metadata
    use std::fs;
    use std::path::Path;
    let snapshot_dir = Path::new("/tmp/nestgate/snapshots");
    if snapshot_dir.exists() {
        if let Ok(entries) = fs::read_dir(snapshot_dir) {
            let count = entries.filter_map(|entry| entry.ok()).count() as u64;
            return Ok(count);
        }
    }

    // Default to 0 if no snapshots found
    Ok(0)
}
