///! Mock Builders for Return Types
///!
///! **⚠️ DEVELOPMENT/TEST ONLY**: This module is only available with `dev-stubs` feature
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Simple performance metrics for mock builders
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub cpu_usage: f64,
    pub memory_usage: f64,
    pub disk_io: f64,
    pub network_io: f64,
}
/// Resource allocation structure for mock builders
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResourceAllocation {
    pub id: String,
    pub resource_type: String,
    pub status: String,
    pub allocated_at: String,
    pub expires_at: String,
    pub metadata: serde_json::Value,
}
/// Workload result structure for mock builders
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkloadResult {
    pub performance_metrics: PerformanceMetrics,
    pub workload_id: String,
    pub success: bool,
    pub execution_time_ms: u64,
    pub resources_used: ResourceAllocation,
    pub result_data: serde_json::Value,
}
/// Build mock resource allocation response
/// **PURE FUNCTION**: Mock resource allocation construction
/// **TESTABLE**: Can verify mock data field assignments
#[must_use]
pub fn build_mock_resource_allocation(
    cpu_cores: u32,
    memory_gb: u32,
    storage_gb: u32,
    network_mbps: u32,
) -> ResourceAllocation {
    ResourceAllocation {
        id: Uuid::new_v4().to_string(),
        resource_type: format!("compute-{cpu_cores}-{memory_gb}-{storage_gb}-{network_mbps}"),
        status: "active".to_string(),
        allocated_at: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
            .to_string(),
        expires_at: (std::time::SystemTime::now() + std::time::Duration::from_secs(24 * 3600))
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
            .to_string(),
        metadata: serde_json::json!({
            "cpu_cores": cpu_cores,
            "memory_gb": memory_gb,
            "storage_gb": storage_gb,
            "network_mbps": network_mbps
        }),
    }
}
/// Build mock workload result
/// **PURE FUNCTION**: Mock workload result construction
/// **TESTABLE**: Can verify mock workload field assignments
#[must_use]
pub fn build_mock_workload_result(
    workload_id: String,
    success: bool,
    processing_time_ms: u64,
) -> WorkloadResult {
    WorkloadResult {
        performance_metrics: PerformanceMetrics {
            cpu_usage: 0.75,
            memory_usage: 0.60,
            disk_io: processing_time_ms as f64,
            network_io: 0.50,
        },
        workload_id,
        success,
        execution_time_ms: processing_time_ms,
        resources_used: ResourceAllocation::default(),
        result_data: if success {
            serde_json::json!({"status": "completed", "output": "mock_result", "processing_time_ms": processing_time_ms})
        } else {
            serde_json::Value::Null
        },
    }
}
/// Build mock performance metrics for testing
pub fn build_mock_performance_metrics() -> crate::Result<PerformanceMetrics> {
    Ok(PerformanceMetrics {
        cpu_usage: 45.2,
        memory_usage: 67.8,
        disk_io: 1024.0 * 1024.0,   // 1MB/s
        network_io: 512.0 * 1024.0, // 512KB/s
    })
}

/// Access grant structure for testing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessGrant {
    pub permissions: Vec<String>,
    pub valid_until: u64,
    pub proof_data: String,
    pub consensus_nodes: Vec<String>,
    pub confidence_score: f64,
}

/// Build mock access grant for testing
#[must_use]
pub fn build_access_grant(
    permissions: &[String],
    valid_until: u64,
    proof_data: &str,
    consensus_nodes: &[String],
    confidence_score: f64,
) -> AccessGrant {
    AccessGrant {
        permissions: permissions.to_vec(),
        valid_until,
        proof_data: proof_data.to_string(),
        consensus_nodes: consensus_nodes.to_vec(),
        confidence_score,
    }
}
