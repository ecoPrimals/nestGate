use crate::universal_traits::PerformanceMetrics;
use uuid::Uuid;

/// Build mock resource allocation response
/// **PURE FUNCTION**: Mock resource allocation construction
/// **TESTABLE**: Can verify mock data field assignments
pub fn build_mock_resource_allocation(
    cpu_cores: u32,
    memory_gb: u32,
    storage_gb: u32,
    network_mbps: u32,
) -> crate::types::ResourceAllocation {
    crate::types::ResourceAllocation {
        id: Uuid::new_v4().to_string(),
        resource_type: format!("compute-{cpu_cores}-{memory_gb}-{storage_gb}-{network_mbps}"),
        status: "active".to_string(),
        amount: cpu_cores as u64,
        cpu_cores,
        memory_mb: (memory_gb * 1024) as u64,
        disk_gb: storage_gb as u64,
        network_bandwidth_mbps: network_mbps,
        allocated_at: std::time::SystemTime::now(),
        expires_at: Some(std::time::SystemTime::now() + std::time::Duration::from_secs(24 * 3600)),
        metadata: {
            let mut meta = std::collections::HashMap::new();
            meta.insert("cpu_cores".to_string(), cpu_cores.to_string());
            meta.insert("memory_gb".to_string(), memory_gb.to_string());
            meta.insert("storage_gb".to_string(), storage_gb.to_string());
            meta.insert("network_mbps".to_string(), network_mbps.to_string());
            meta
        },
    }
}

/// Build mock workload result
/// **PURE FUNCTION**: Mock workload result construction
/// **TESTABLE**: Can verify mock workload field assignments
pub fn build_mock_workload_result(
    workload_id: String,
    success: bool,
    processing_time_ms: u64,
) -> crate::types::WorkloadResult {
    crate::types::WorkloadResult {
        workload_id,
        success,
        status: if success {
            "completed".to_string()
        } else {
            "failed".to_string()
        },
        message: if success {
            "Mock workload completed successfully".to_string()
        } else {
            "Mock workload failed".to_string()
        },
        execution_time_ms: processing_time_ms,
        resources_used: crate::types::ResourceAllocation::default(),
        result_data: if success {
            serde_json::json!({"status": "completed", "output": "mock_result", "processing_time_ms": processing_time_ms})
        } else {
            serde_json::Value::Null
        },
        metrics: {
            let mut metrics = std::collections::HashMap::new();
            metrics.insert("processing_time_ms".to_string(), processing_time_ms as f64);
            metrics.insert("cpu_usage".to_string(), 0.75);
            metrics.insert("memory_usage".to_string(), 0.60);
            metrics
        },
        started_at: std::time::SystemTime::now()
            - std::time::Duration::from_millis(processing_time_ms),
        completed_at: Some(std::time::SystemTime::now()),
        error_message: if !success {
            Some("Mock error message".to_string())
        } else {
            None
        },
    }
}

/// Build mock performance metrics for testing
pub fn build_mock_performance_metrics() -> crate::Result<PerformanceMetrics> {
    Ok(PerformanceMetrics {
        timestamp: std::time::SystemTime::now(),
        _cpu_usage: 45.2,
        memory_usage: 67.8,
        disk_io: 1024.0 * 1024.0,   // 1MB/s
        network_io: 512.0 * 1024.0, // 512KB/s
    })
}
