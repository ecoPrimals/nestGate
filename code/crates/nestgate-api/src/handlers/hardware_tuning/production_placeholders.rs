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
    config: HardwareTuningConfig,
    metrics_collector: SystemMetricsCollector,
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
pub fn get_hardware_info() -> std::result::Result<Json<serde_json::Value>, StatusCode> {
    let (status, response) = hardware_tuning_disabled();
    Err(status)
}

pub fn optimize_hardware_performance() -> std::result::Result<Json<serde_json::Value>, StatusCode> {
    let (status, response) = hardware_tuning_disabled();
    Err(status)
}

pub const fn get_system_capabilities() -> std::result::Result<Json<SystemCapabilities>, StatusCode>
{
    Err(StatusCode::NOT_IMPLEMENTED)
}

pub const fn get_compute_resources() -> std::result::Result<Json<ComputeResources>, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}

pub fn register_tuning_service(
    _Json: Json<serde_json::Value>,
) -> std::result::Result<Json<serde_json::Value>, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}

pub const fn run_hardware_benchmark() -> std::result::Result<Json<BenchmarkResult>, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}

pub const fn start_hardware_tuning_session(
) -> std::result::Result<Json<LiveHardwareTuningSession>, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}

pub fn get_allocation_details(
    _Path: axum::extract::Path<String>,
) -> std::result::Result<Json<ComputeAllocation>, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}
