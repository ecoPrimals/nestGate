//! **HARDWARE TUNING PRODUCTION PLACEHOLDERS**
//!
//! Placeholder handlers for production builds where `dev-stubs` is not enabled.
//! These return helpful error messages directing users to implement real hardware detection.
//!
//! **⚠️ IMPORTANT ⚠️**
//!
//! These are NOT functional handlers - they exist solely to allow compilation
//! without `dev-stubs` feature. For production hardware tuning, implement using
//! `sysinfo` crate as documented in the stub handlers.

use axum::{http::StatusCode, response::Json};
use serde_json::json;

use super::types::{
    BenchmarkResult, ComputeAllocation, ComputeResources, CpuMonitor, GpuMonitor, HardwareMonitors,
    HardwareTuningConfig, LiveHardwareTuningSession, MemoryMonitor, SystemCapabilities,
    SystemMetricsCollector,
};

/// Placeholder response for disabled hardware tuning endpoints
fn hardware_tuning_disabled() -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(json!({
            "error": "Hardware tuning API is disabled in production builds",
            "message": "Hardware tuning stubs are development-only",
            "recommendation": "Implement real hardware detection using sysinfo crate",
            "implementation_guide": "See handlers.rs (dev-stubs) for implementation examples",
            "documentation": "https://docs.rs/sysinfo/latest/sysinfo/"
        })),
    )
}

/// Placeholder hardware tuning handler for production builds
#[derive(Debug, Clone)]
pub struct RealHardwareTuningHandler {
    #[allow(dead_code)] // Reserved for future hardware tuning implementation
    config: HardwareTuningConfig,
    #[allow(dead_code)] // Reserved for future metrics collection
    metrics_collector: SystemMetricsCollector,
    #[allow(dead_code)] // Reserved for future hardware monitoring
    monitors: HardwareMonitors,
}

impl Default for RealHardwareTuningHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl RealHardwareTuningHandler {
    /// Create a new placeholder handler
    #[must_use]
    pub fn new() -> Self {
        Self {
            config: HardwareTuningConfig::default(),
            metrics_collector: SystemMetricsCollector {
                cpu_monitor: CpuMonitor,
                memory_monitor: MemoryMonitor,
                gpu_monitor: Some(GpuMonitor),
            },
            monitors: HardwareMonitors {
                cpu: CpuMonitor,
                memory: MemoryMonitor,
                gpu: Some(GpuMonitor),
            },
        }
    }
}

// Export placeholder handler functions

/// Get hardware information
///
/// # Errors
///
/// Returns `StatusCode::NOT_IMPLEMENTED` as hardware tuning is disabled in production builds.
///
/// **Note**: This endpoint is disabled in production builds.
/// Returns `NOT_IMPLEMENTED` to indicate hardware tuning features are not available.
pub fn get_hardware_info() -> std::result::Result<Json<serde_json::Value>, StatusCode> {
    let (status, _response) = hardware_tuning_disabled();
    Err(status)
}

/// Optimize hardware performance
///
/// # Errors
///
/// Returns `StatusCode::NOT_IMPLEMENTED` as hardware tuning is disabled in production builds.
///
/// **Note**: This endpoint is disabled in production builds.
/// Returns `NOT_IMPLEMENTED` to indicate hardware tuning features are not available.
pub fn optimize_hardware_performance() -> std::result::Result<Json<serde_json::Value>, StatusCode> {
    let (status, _response) = hardware_tuning_disabled();
    Err(status)
}

/// Get system capabilities
///
/// # Errors
///
/// Returns `StatusCode::NOT_IMPLEMENTED` as hardware tuning is disabled in production builds.
///
/// **Note**: This endpoint is disabled in production builds.
/// Returns `NOT_IMPLEMENTED` to indicate hardware tuning features are not available.
pub const fn get_system_capabilities() -> std::result::Result<Json<SystemCapabilities>, StatusCode>
{
    Err(StatusCode::NOT_IMPLEMENTED)
}

/// Get compute resources
///
/// # Errors
///
/// Returns `StatusCode::NOT_IMPLEMENTED` as hardware tuning is disabled in production builds.
///
/// **Note**: This endpoint is disabled in production builds.
/// Returns `NOT_IMPLEMENTED` to indicate hardware tuning features are not available.
pub const fn get_compute_resources() -> std::result::Result<Json<ComputeResources>, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}

/// Register tuning service
///
/// # Errors
///
/// Returns `StatusCode::NOT_IMPLEMENTED` as hardware tuning is disabled in production builds.
///
/// **Note**: This endpoint is disabled in production builds.
/// Returns `NOT_IMPLEMENTED` to indicate hardware tuning features are not available.
pub fn register_tuning_service(
    _json: Json<serde_json::Value>,
) -> std::result::Result<Json<serde_json::Value>, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}

/// Run hardware benchmark
///
/// # Errors
///
/// Returns `StatusCode::NOT_IMPLEMENTED` as hardware tuning is disabled in production builds.
///
/// **Note**: This endpoint is disabled in production builds.
/// Returns `NOT_IMPLEMENTED` to indicate hardware tuning features are not available.
pub const fn run_hardware_benchmark() -> std::result::Result<Json<BenchmarkResult>, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}

/// Start hardware tuning session
///
/// **Note**: This endpoint is disabled in production builds.
/// Returns `NOT_IMPLEMENTED` to indicate hardware tuning features are not available.
pub const fn start_hardware_tuning_session(
) -> std::result::Result<Json<LiveHardwareTuningSession>, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}

/// Get allocation details
///
/// **Note**: This endpoint is disabled in production builds.
/// Returns `NOT_IMPLEMENTED` to indicate hardware tuning features are not available.
pub fn get_allocation_details(
    _path: axum::extract::Path<String>,
) -> std::result::Result<Json<ComputeAllocation>, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}
