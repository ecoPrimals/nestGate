//! **HARDWARE TUNING TEST HELPERS**
//!
//! Stub implementations for hardware tuning tests.
//! These provide minimal implementations to satisfy test compilation.

use axum::Json;
use nestgate_api::{error::Result, handlers::hardware_tuning::types::*};
use uuid::Uuid;

/// Stub: Get system capabilities
pub async fn get_system_capabilities() -> Result<Json<SystemCapabilities>> {
    Ok(Json(SystemCapabilities {
        max_cpu_cores: num_cpus::get() as u32,
        max_memory_gb: 16, // Stub value
        gpu_available: false,
        simd_support: true,
    }))
}

/// Stub: Run hardware benchmarks
pub async fn run_hardware_benchmarks() -> Result<Json<Vec<BenchmarkResult>>> {
    Ok(Json(vec![
        BenchmarkResult {
            benchmark_type: "cpu".to_string(),
            score: 1000.0,
            duration_ms: 100,
            details: "Stub CPU benchmark".to_string(),
        },
        BenchmarkResult {
            benchmark_type: "memory".to_string(),
            score: 2000.0,
            duration_ms: 150,
            details: "Stub memory benchmark".to_string(),
        },
    ]))
}

/// Stub: Allocate compute resources
pub async fn allocate_compute_resources(
    Json(request): Json<ComputeResourceRequest>,
) -> Result<Json<ComputeAllocation>> {
    // Basic validation
    if request.cpu_cores == 0 {
        return Err(nestgate_api::error::ApiError::BadRequest(
            "CPU cores must be greater than 0".to_string(),
        ));
    }
    if request.memory_gb == 0 {
        return Err(nestgate_api::error::ApiError::BadRequest(
            "Memory must be greater than 0".to_string(),
        ));
    }
    if request.memory_gb > 1024 {
        return Err(nestgate_api::error::ApiError::BadRequest(
            "Memory request exceeds limits".to_string(),
        ));
    }

    Ok(Json(ComputeAllocation {
        allocation_id: Uuid::new_v4().to_string(),
        allocated_cores: request.cpu_cores,
        allocated_memory_gb: request.memory_gb,
        status: "allocated".to_string(),
    }))
}

/// Stub: Get live hardware metrics
pub async fn get_live_metrics() -> Result<Json<LiveHardwareMetrics>> {
    Ok(Json(LiveHardwareMetrics {
        cpu_usage: 45.0,
        memory_usage: 60.0,
        gpu_usage: 0.0,
        disk_io: 1024,
        network_io: 2048,
        timestamp: chrono::Utc::now().timestamp(),
    }))
}

/// Stub: Get system profile
pub async fn get_system_profile() -> Result<Json<SystemProfile>> {
    Ok(Json(SystemProfile {
        total_cores: num_cpus::get() as u32,
        total_memory_gb: 16,
        cpu_model: "Stub CPU".to_string(),
        os_type: std::env::consts::OS.to_string(),
        performance_tier: "medium".to_string(),
        capabilities: vec!["simd".to_string(), "threading".to_string()],
    }))
}

/// Stub: Start tuning session
pub async fn start_tuning_session() -> Result<Json<LiveHardwareTuningSession>> {
    Ok(Json(LiveHardwareTuningSession {
        session_id: Uuid::new_v4().to_string(),
        status: "active".to_string(),
        recommendations: vec!["Optimize CPU usage".to_string()],
        started_at: chrono::Utc::now().timestamp(),
    }))
}
