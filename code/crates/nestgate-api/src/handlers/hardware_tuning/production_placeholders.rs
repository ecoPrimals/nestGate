// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! **HARDWARE TUNING PRODUCTION PLACEHOLDERS**
//!
//! Placeholder handlers for production builds where `dev-stubs` is not enabled.
//! These return helpful error messages directing users to implement real hardware detection.
//!
//! **⚠️ IMPORTANT ⚠️**
//!
//! These are NOT functional handlers - they exist solely to allow compilation
//! without `dev-stubs` feature. For production hardware tuning, implement using
//! `nestgate_core::linux_proc` and `hardware_tuning::linux_proc` (ecoBin v3.0; `sysinfo` is legacy).

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
            "recommendation": "Implement real hardware detection using /proc + rustix (ecoBin v3.0); see linux_proc in nestgate-core",
            "implementation_guide": "See handlers.rs (dev-stubs) and hardware_tuning::linux_proc for implementation examples",
            "documentation": "https://docs.rs/rustix/latest/rustix/"
        })),
    )
}

/// Placeholder hardware tuning handler for production builds
#[derive(Debug, Clone)]
/// Handler for RealHardwareTuning requests
pub struct RealHardwareTuningHandler {
    #[allow(dead_code)] // Reserved for future hardware tuning implementation
    config: HardwareTuningConfig,
    #[allow(dead_code)] // Reserved for future metrics collection
    metrics_collector: SystemMetricsCollector,
    #[allow(dead_code)] // Reserved for future hardware monitoring
    monitors: HardwareMonitors,
}

impl Default for RealHardwareTuningHandler {
    /// Returns the default instance
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
pub const fn start_hardware_tuning_session()
-> std::result::Result<Json<LiveHardwareTuningSession>, StatusCode> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hardware_tuning_disabled_response() {
        let (status, response) = hardware_tuning_disabled();
        assert_eq!(status, StatusCode::NOT_IMPLEMENTED);

        let value = response.0;
        assert!(value.get("error").is_some());
        assert!(value.get("message").is_some());
        assert!(value.get("recommendation").is_some());
    }

    #[test]
    fn test_real_hardware_tuning_handler_new() {
        let handler = RealHardwareTuningHandler::new();
        assert_eq!(handler.config.cpu_cores, 8);
        assert_eq!(handler.config.memory_gb, 16);
    }

    #[test]
    fn test_real_hardware_tuning_handler_default() {
        let handler = RealHardwareTuningHandler::default();
        assert_eq!(handler.config.cpu_cores, 8);
    }

    #[test]
    fn test_get_hardware_info_returns_not_implemented() {
        let result = get_hardware_info();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), StatusCode::NOT_IMPLEMENTED);
    }

    #[test]
    fn test_optimize_hardware_performance_returns_not_implemented() {
        let result = optimize_hardware_performance();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), StatusCode::NOT_IMPLEMENTED);
    }

    #[test]
    fn test_get_system_capabilities_returns_not_implemented() {
        let result = get_system_capabilities();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), StatusCode::NOT_IMPLEMENTED);
    }

    #[test]
    fn test_get_compute_resources_returns_not_implemented() {
        let result = get_compute_resources();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), StatusCode::NOT_IMPLEMENTED);
    }

    #[test]
    fn test_register_tuning_service_returns_not_implemented() {
        let json_data = Json(serde_json::json!({"test": "data"}));
        let result = register_tuning_service(json_data);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), StatusCode::NOT_IMPLEMENTED);
    }

    #[test]
    fn test_run_hardware_benchmark_returns_not_implemented() {
        let result = run_hardware_benchmark();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), StatusCode::NOT_IMPLEMENTED);
    }

    #[test]
    fn test_start_hardware_tuning_session_returns_not_implemented() {
        let result = start_hardware_tuning_session();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), StatusCode::NOT_IMPLEMENTED);
    }

    #[test]
    fn test_get_allocation_details_returns_not_implemented() {
        let path = axum::extract::Path("test-allocation".to_string());
        let result = get_allocation_details(path);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), StatusCode::NOT_IMPLEMENTED);
    }

    #[test]
    fn test_handler_has_correct_monitors() {
        let handler = RealHardwareTuningHandler::new();
        assert!(handler.monitors.gpu.is_some());
    }

    #[test]
    fn test_handler_has_metrics_collector() {
        let handler = RealHardwareTuningHandler::new();
        assert!(handler.metrics_collector.gpu_monitor.is_some());
    }
}
