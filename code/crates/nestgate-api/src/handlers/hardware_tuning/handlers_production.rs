//! **HARDWARE TUNING — PRODUCTION-STYLE AXUM HANDLERS**
//!
//! JSON endpoints that share `/proc` discovery with [`super::handlers::RealHardwareTuningHandler`]
//! via [`super::linux_proc`] (no `sysinfo`).

use axum::response::Json;
use chrono::Utc;
use tracing::info;

use super::handlers::RealHardwareTuningHandler;
use super::linux_proc;
use super::types::{
    BenchmarkResult, ComputeAllocation, ComputeResourceRequest, ComputeResources,
    LiveHardwareMetrics, LiveHardwareTuningSession, SystemCapabilities, SystemProfile,
    TuningResult, TuningServiceRegistration,
};
use crate::error::Result;

/// Live metrics via the same collector as [`RealHardwareTuningHandler`].
async fn collect_live_metrics(handler: &RealHardwareTuningHandler) -> Result<LiveHardwareMetrics> {
    handler.get_live_hardware_metrics().await
}

/// Expose current compute resources (from `/proc`).
pub async fn get_hardware_info() -> Result<Json<ComputeResources>> {
    let resources = linux_proc::compute_resources_from_proc()?;
    Ok(Json(resources))
}

/// Same as [`get_system_capabilities`] (alias for route naming).
pub async fn get_hardware_capabilities() -> Result<Json<SystemCapabilities>> {
    get_system_capabilities().await
}

/// Derived profile from `/proc` and live metrics.
pub async fn get_system_profile() -> Result<Json<SystemProfile>> {
    let handler = RealHardwareTuningHandler::new();
    let profile = handler.get_derived_system_profile().await?;
    Ok(Json(profile))
}

/// Optimize hardware performance (observational; no privileged tuning).
pub async fn optimize_hardware_performance() -> Result<Json<TuningResult>> {
    let handler = RealHardwareTuningHandler::new();
    let tuning_result = handler.auto_tune().await?;
    Ok(Json(tuning_result))
}

/// System capabilities from `/proc` and optional GPU tooling.
pub async fn get_system_capabilities() -> Result<Json<SystemCapabilities>> {
    let handler = RealHardwareTuningHandler::new();
    let caps = handler.detect_system_capabilities().await?;
    Ok(Json(caps))
}

/// Same as [`get_hardware_info`].
pub async fn get_compute_resources() -> Result<Json<ComputeResources>> {
    get_hardware_info().await
}

/// Register tuning service (metadata only; no host service manager integration).
pub async fn register_tuning_service(
    Json(registration): Json<TuningServiceRegistration>,
) -> Result<Json<serde_json::Value>> {
    info!("Registering tuning service: {}", registration.service_name);

    Ok(Json(serde_json::json!({
        "status": "registered",
        "service_name": registration.service_name,
        "registered_at": Utc::now(),
    })))
}

/// Lightweight benchmark derived from `/proc` visibility and live metrics (not a stress test).
pub async fn run_hardware_benchmark() -> Result<Json<BenchmarkResult>> {
    let handler = RealHardwareTuningHandler::new();
    let start_time = std::time::Instant::now();
    let resources = linux_proc::compute_resources_from_proc()?;
    let metrics = collect_live_metrics(&handler).await?;
    let duration = start_time.elapsed();
    let score = f64::from(resources.available_cpu)
        .mul_add(10.0, f64::from(resources.available_memory_gb) * 0.5);

    Ok(Json(BenchmarkResult {
        benchmark_type: "proc_snapshot".to_string(),
        score,
        duration_ms: duration.as_millis() as u64,
        metrics,
    }))
}

/// Start a tuning session backed by live metrics and `/proc` resource snapshot.
pub async fn start_hardware_tuning_session() -> Result<Json<LiveHardwareTuningSession>> {
    let handler = RealHardwareTuningHandler::new();
    let metrics = collect_live_metrics(&handler).await?;
    let resources = linux_proc::compute_resources_from_proc()?;

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

/// Allocation details from current `/proc` snapshot (no persistent allocator state).
pub async fn get_allocation_details(
    axum::extract::Path(_allocation_id): axum::extract::Path<String>,
) -> Result<Json<ComputeAllocation>> {
    let resources = linux_proc::compute_resources_from_proc()?;

    Ok(Json(ComputeAllocation {
        cpu_cores: resources.available_cpu,
        memory_gb: resources.available_memory_gb,
        gpu_count: resources.available_gpu,
    }))
}

/// Request compute resources with validation against `/proc` limits.
pub async fn request_compute_resources(
    Json(request): Json<ComputeResourceRequest>,
) -> Result<Json<ComputeAllocation>> {
    let handler = RealHardwareTuningHandler::new();
    let allocation = handler.request_compute_resources(&request).await?;
    Ok(Json(allocation))
}
