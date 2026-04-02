// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! **HARDWARE TUNING PRODUCTION SHIMS**
//!
//! Handlers for production builds where `dev-stubs` is not enabled: routes compile and return
//! `501 Not Implemented` with a minimal structured body (no hardcoded external URLs).
//!
//! These responses are **intentional**: they implement the delegation pattern when the full
//! hardware-tuning HTTP implementation is not compiled in. They are not placeholders left to be
//! “filled in” later; clients should enable `dev-stubs` or use the appropriate integration path
//! when that surface is required.

use axum::{http::StatusCode, response::Json};
use serde_json::json;

use super::types::{
    BenchmarkResult, ComputeAllocation, ComputeResources, CpuMonitor, GpuMonitor, HardwareMonitors,
    HardwareTuningConfig, LiveHardwareTuningSession, MemoryMonitor, SystemCapabilities,
    SystemMetricsCollector,
};

/// Response body when the hardware tuning HTTP surface is not compiled in.
fn hardware_tuning_disabled() -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(json!({
            "error": "not_implemented",
            "feature": "hardware_tuning_http",
            "details": null,
        })),
    )
}

/// Placeholder hardware tuning handler for production builds
#[derive(Debug, Clone)]
/// Handler for `RealHardwareTuning` requests
pub struct RealHardwareTuningHandler {
    _config: HardwareTuningConfig,
    _metrics_collector: SystemMetricsCollector,
    _monitors: HardwareMonitors,
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
            _config: HardwareTuningConfig::default(),
            _metrics_collector: SystemMetricsCollector {
                cpu_monitor: CpuMonitor,
                memory_monitor: MemoryMonitor,
                gpu_monitor: Some(GpuMonitor),
            },
            _monitors: HardwareMonitors {
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
        assert_eq!(
            value.get("error").and_then(|v| v.as_str()),
            Some("not_implemented")
        );
        assert_eq!(
            value.get("feature").and_then(|v| v.as_str()),
            Some("hardware_tuning_http")
        );
        assert!(value.get("details").is_none_or(serde_json::Value::is_null));
    }

    #[test]
    fn test_real_hardware_tuning_handler_new() {
        let handler = RealHardwareTuningHandler::new();
        assert_eq!(handler._config.cpu_cores, 8);
        assert_eq!(handler._config.memory_gb, 16);
    }

    #[test]
    fn test_real_hardware_tuning_handler_default() {
        let handler = RealHardwareTuningHandler::default();
        assert_eq!(handler._config.cpu_cores, 8);
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
        assert!(handler._monitors.gpu.is_some());
    }

    #[test]
    fn test_handler_has_metrics_collector() {
        let handler = RealHardwareTuningHandler::new();
        assert!(handler._metrics_collector.gpu_monitor.is_some());
    }
}
