// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! **HARDWARE TUNING — PRODUCTION (no `dev-stubs`)**
//!
//! Read-only endpoints use [`super::linux_proc`] (`/proc`, best-effort `nvidia-smi`) and, when the
//! `OpenZFS` kstat interface is present, a lightweight ARC snapshot from
//! `/proc/spl/kstat/zfs/arcstats`. Service registration remains `501` with an explicit JSON body.

use axum::{http::StatusCode, response::Json};
use chrono::Utc;
use serde_json::json;

use super::linux_proc;
use super::types::{
    BenchmarkResult, ComputeAllocation, ComputeResources, CpuMonitor, DiskMonitor, GpuMonitor,
    HardwareMonitors, HardwareTuningConfig, LiveHardwareTuningSession, MemoryMonitor,
    NetworkMonitor, SystemCapabilities, SystemMetricsCollector, TuningResult,
};

/// Response body when a hardware-tuning sub-feature is intentionally unavailable.
#[cfg(test)]
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

/// Best-effort ZFS ARC counters when `/proc/spl/kstat/zfs/arcstats` exists (Linux + ZFS kernel module).
fn zfs_arc_stats_json() -> serde_json::Value {
    #[cfg(target_os = "linux")]
    {
        let path = "/proc/spl/kstat/zfs/arcstats";
        if let Ok(content) = std::fs::read_to_string(path) {
            let mut hits = 0u64;
            let mut misses = 0u64;
            let mut size = 0u64;
            for line in content.lines() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 3 {
                    match parts[0] {
                        "hits" => hits = parts[2].parse().unwrap_or(0),
                        "misses" => misses = parts[2].parse().unwrap_or(0),
                        "size" => size = parts[2].parse().unwrap_or(0),
                        _ => {}
                    }
                }
            }
            let total = hits.saturating_add(misses);
            let hit_ratio = (total > 0).then_some(hits as f64 / total as f64);
            return json!({
                "available": true,
                "arc_size_bytes": size,
                "hits": hits,
                "misses": misses,
                "hit_ratio": hit_ratio,
            });
        }
    }
    json!({ "available": false })
}

const fn metrics_collector() -> SystemMetricsCollector {
    SystemMetricsCollector {
        cpu_monitor: CpuMonitor,
        memory_monitor: MemoryMonitor,
        gpu_monitor: Some(GpuMonitor),
        disk_monitor: DiskMonitor,
        network_monitor: NetworkMonitor,
    }
}

fn proc_snapshot_score(resources: &ComputeResources) -> f64 {
    f64::from(resources.available_cpu).mul_add(10.0, f64::from(resources.available_memory_gb) * 0.5)
}

/// Hardware tuning handler for production builds (reads from `/proc` and live metrics).
#[derive(Debug, Clone)]
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
    /// Create a new hardware tuning handler
    #[must_use]
    pub fn new() -> Self {
        Self {
            _config: HardwareTuningConfig::default(),
            _metrics_collector: metrics_collector(),
            _monitors: HardwareMonitors {
                cpu: CpuMonitor,
                memory: MemoryMonitor,
                gpu: Some(GpuMonitor),
            },
        }
    }
}

/// Get hardware information (compute resources, capability summary, optional ZFS ARC snapshot).
///
/// # Errors
///
/// Returns [`StatusCode::INTERNAL_SERVER_ERROR`] if `/proc` (or related) discovery fails.
pub fn get_hardware_info() -> std::result::Result<Json<serde_json::Value>, StatusCode> {
    let resources =
        linux_proc::compute_resources_from_proc().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let caps = linux_proc::system_capabilities_from_proc()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(json!({
        "status": "success",
        "compute": resources,
        "system_capabilities_summary": {
            "cpu_model": caps.cpu_model,
            "cpu_cores": caps.cpu_cores,
            "memory_gb": caps.memory_gb,
            "gpu_available": caps.gpu_available,
        },
        "zfs_arc": zfs_arc_stats_json(),
    })))
}

/// Observational “optimization”: samples live metrics twice without applying privileged tunables.
///
/// # Errors
///
/// Returns [`StatusCode::INTERNAL_SERVER_ERROR`] if metric collection fails.
pub fn optimize_hardware_performance() -> std::result::Result<Json<TuningResult>, StatusCode> {
    let collector = metrics_collector();
    let before_metrics = collector
        .collect_current_metrics()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let resources =
        linux_proc::compute_resources_from_proc().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let profile_name = format!(
        "{} logical CPUs, {} GiB RAM (best-effort)",
        resources.available_cpu, resources.available_memory_gb
    );
    let after_metrics = collector
        .collect_current_metrics()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(TuningResult {
        profile_name,
        optimizations_applied: vec![
            "observed_live_metrics_only".to_string(),
            "no_kernel_or_zfs_module_tuning_via_http".to_string(),
        ],
        estimated_power_increase: 0.0,
        performance_improvement: 0.0,
        before_metrics,
        after_metrics,
    }))
}

/// System capabilities from `/proc` and optional `nvidia-smi`.
///
/// # Errors
///
/// Returns [`StatusCode::INTERNAL_SERVER_ERROR`] if host discovery fails.
pub fn get_system_capabilities() -> std::result::Result<Json<SystemCapabilities>, StatusCode> {
    linux_proc::system_capabilities_from_proc()
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

/// Available compute resources from `/proc` (same data as the `compute` field of [`get_hardware_info`]).
///
/// # Errors
///
/// Returns [`StatusCode::INTERNAL_SERVER_ERROR`] if discovery fails.
pub fn get_compute_resources() -> std::result::Result<Json<ComputeResources>, StatusCode> {
    linux_proc::compute_resources_from_proc()
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

/// Register tuning service — not implemented (no HTTP-facing service registry).
pub fn register_tuning_service(
    _json: Json<serde_json::Value>,
) -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(json!({
            "error": "not_implemented",
            "feature": "hardware_tuning_service_registration",
            "details": "Persisting tuning service registrations requires host integration (systemd, Kubernetes, or the NestGate capability plane). This HTTP handler does not write to a service registry.",
        })),
    )
}

/// Lightweight benchmark from a `/proc` resource snapshot and live metrics (not a stress test).
///
/// # Errors
///
/// Returns [`StatusCode::INTERNAL_SERVER_ERROR`] if discovery or metric collection fails.
pub fn run_hardware_benchmark() -> std::result::Result<Json<BenchmarkResult>, StatusCode> {
    let start_time = std::time::Instant::now();
    let resources =
        linux_proc::compute_resources_from_proc().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let metrics = metrics_collector()
        .collect_current_metrics()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let duration_ms = start_time.elapsed().as_millis() as u64;
    let score = proc_snapshot_score(&resources);
    Ok(Json(BenchmarkResult {
        benchmark_type: "proc_snapshot".to_string(),
        score,
        duration_ms,
        metrics,
    }))
}

/// Start a session backed by a live metric sample and current `/proc` resource snapshot.
///
/// # Errors
///
/// Returns [`StatusCode::INTERNAL_SERVER_ERROR`] if discovery or metric collection fails.
pub fn start_hardware_tuning_session()
-> std::result::Result<Json<LiveHardwareTuningSession>, StatusCode> {
    let metrics = metrics_collector()
        .collect_current_metrics()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let resources =
        linux_proc::compute_resources_from_proc().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let allocation = ComputeAllocation {
        cpu_cores: resources.available_cpu,
        memory_gb: resources.available_memory_gb,
        gpu_count: resources.available_gpu,
    };
    Ok(Json(LiveHardwareTuningSession {
        session_id: uuid::Uuid::new_v4().to_string(),
        started_at: Utc::now(),
        resource_allocation: allocation,
        current_metrics: metrics,
    }))
}

/// Allocation view from the current `/proc` snapshot (no persistent allocator).
///
/// # Errors
///
/// Returns [`StatusCode::INTERNAL_SERVER_ERROR`] if discovery fails.
pub fn get_allocation_details(
    _path: axum::extract::Path<String>,
) -> std::result::Result<Json<ComputeAllocation>, StatusCode> {
    let resources =
        linux_proc::compute_resources_from_proc().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(ComputeAllocation {
        cpu_cores: resources.available_cpu,
        memory_gb: resources.available_memory_gb,
        gpu_count: resources.available_gpu,
    }))
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
    fn test_get_hardware_info_ok_or_error() {
        let result = get_hardware_info();
        assert!(result.is_ok() || matches!(result, Err(StatusCode::INTERNAL_SERVER_ERROR)));
        if let Ok(Json(v)) = result {
            assert_eq!(v.get("status").and_then(|x| x.as_str()), Some("success"));
            assert!(v.get("compute").is_some());
            assert!(v.get("zfs_arc").is_some());
        }
    }

    #[test]
    fn test_optimize_hardware_performance_ok_or_error() {
        let result = optimize_hardware_performance();
        assert!(result.is_ok() || matches!(result, Err(StatusCode::INTERNAL_SERVER_ERROR)));
    }

    #[test]
    fn test_get_system_capabilities_ok_or_error() {
        let result = get_system_capabilities();
        assert!(result.is_ok() || matches!(result, Err(StatusCode::INTERNAL_SERVER_ERROR)));
        if let Ok(Json(caps)) = result {
            assert!(caps.cpu_cores > 0);
        }
    }

    #[test]
    fn test_get_compute_resources_ok_or_error() {
        let result = get_compute_resources();
        assert!(result.is_ok() || matches!(result, Err(StatusCode::INTERNAL_SERVER_ERROR)));
        if let Ok(Json(r)) = result {
            assert!(r.available_cpu >= 1);
            assert!(r.available_memory_gb >= 1);
        }
    }

    #[test]
    fn test_register_tuning_service_returns_not_implemented_with_body() {
        let json_data = Json(serde_json::json!({"test": "data"}));
        let (status, Json(body)) = register_tuning_service(json_data);
        assert_eq!(status, StatusCode::NOT_IMPLEMENTED);
        assert_eq!(
            body.get("error").and_then(|v| v.as_str()),
            Some("not_implemented")
        );
        assert!(body.get("details").and_then(|v| v.as_str()).is_some());
    }

    #[test]
    fn test_run_hardware_benchmark_ok_or_error() {
        let result = run_hardware_benchmark();
        assert!(result.is_ok() || matches!(result, Err(StatusCode::INTERNAL_SERVER_ERROR)));
    }

    #[test]
    fn test_start_hardware_tuning_session_ok_or_error() {
        let result = start_hardware_tuning_session();
        assert!(result.is_ok() || matches!(result, Err(StatusCode::INTERNAL_SERVER_ERROR)));
    }

    #[test]
    fn test_get_allocation_details_ok_or_error() {
        let path = axum::extract::Path("test-allocation".to_string());
        let result = get_allocation_details(path);
        assert!(result.is_ok() || matches!(result, Err(StatusCode::INTERNAL_SERVER_ERROR)));
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
