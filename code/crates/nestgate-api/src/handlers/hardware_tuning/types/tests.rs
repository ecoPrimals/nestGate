// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

use std::time::Duration;

use chrono::Utc;

use super::*;

#[test]
fn test_hardware_tuning_config_default() {
    let config = HardwareTuningConfig::default();
    assert_eq!(config.cpu_cores, 8);
    assert_eq!(config.memory_gb, 16);
    assert!(config.cpu_tuning_enabled);
    assert!(config.memory_optimization_enabled);
    assert!(!config.gpu_tuning_enabled);
    assert_eq!(config.monitoring_interval, Duration::from_secs(5));
}

#[test]
fn test_compute_allocation_creation() {
    let allocation = ComputeAllocation {
        cpu_cores: 4,
        memory_gb: 8,
        gpu_count: 2,
    };
    assert_eq!(allocation.cpu_cores, 4);
    assert_eq!(allocation.memory_gb, 8);
    assert_eq!(allocation.gpu_count, 2);
}

#[test]
fn test_compute_resources_creation() {
    let resources = ComputeResources {
        available_cpu: 16,
        available_memory_gb: 64,
        available_gpu: 4,
    };
    assert_eq!(resources.available_cpu, 16);
    assert_eq!(resources.available_memory_gb, 64);
    assert_eq!(resources.available_gpu, 4);
}

#[test]
fn test_compute_adapter_new() {
    let adapter = ComputeAdapter::new("test-service".to_string());
    assert_eq!(adapter.service_name, "test-service");
}

#[test]
fn test_live_hardware_tuning_session_new() {
    let session = LiveHardwareTuningSession::new().expect("Should create session");
    assert!(session.session_id.starts_with("session_"));
    assert_eq!(session.resource_allocation.cpu_cores, 8);
    assert_eq!(session.resource_allocation.memory_gb, 16);
    assert_eq!(session.current_metrics.cpu_usage, 25.0);
}

#[test]
fn test_live_hardware_tuning_session_collect_metrics() {
    let session = LiveHardwareTuningSession::new().expect("Should create session");
    let metrics = session
        .collect_current_metrics()
        .expect("Should collect metrics");
    assert_eq!(metrics.cpu_usage, 30.0);
    assert_eq!(metrics.memory_usage, 45.0);
}

#[test]
fn test_system_metrics_collector_new() {
    let collector = SystemMetricsCollector::new().expect("Should create collector");
    assert!(collector.gpu_monitor.is_none());
}

#[test]
fn test_system_capabilities_serialization() {
    let caps = SystemCapabilities {
        cpu_cores: 8,
        cpu_model: "Intel Core i7".to_string(),
        memory_gb: 32,
        gpu_available: true,
        gpu_info: Some(GpuInfo {
            name: "NVIDIA RTX 3080".to_string(),
            memory_mb: 10240,
        }),
    };

    let json = serde_json::to_string(&caps).expect("Should serialize");
    let deserialized: SystemCapabilities = serde_json::from_str(&json).expect("Should deserialize");
    assert_eq!(caps.cpu_cores, deserialized.cpu_cores);
    assert_eq!(caps.memory_gb, deserialized.memory_gb);
}

#[test]
fn test_gpu_allocation_creation() {
    let allocation = GpuAllocation {
        gpu_id: "GPU-0".to_string(),
        memory_gb: 8,
    };
    assert_eq!(allocation.gpu_id, "GPU-0");
    assert_eq!(allocation.memory_gb, 8);
}

#[test]
fn test_tuning_service_registration() {
    use nestgate_core::constants::hardcoding::{addresses, ports};
    let endpoint = format!(
        "http://{}:{}",
        addresses::LOCALHOST_NAME,
        ports::HTTP_DEFAULT
    );

    let registration = TuningServiceRegistration {
        service_name: "tuning-service".to_string(),
        endpoint,
    };
    assert_eq!(registration.service_name, "tuning-service");
    assert!(registration.endpoint.starts_with("http://"));
}

#[test]
fn test_performance_snapshot_creation() {
    let snapshot = PerformanceSnapshot {
        timestamp: Utc::now(),
        cpu_usage: 45.5,
        memory_usage: 60.2,
        disk_io: 80.0,
        network_io: 120.5,
    };
    assert_eq!(snapshot.cpu_usage, 45.5);
    assert_eq!(snapshot.memory_usage, 60.2);
}

#[test]
fn test_system_profile_creation() {
    let profile = SystemProfile {
        cpu_profile: "performance".to_string(),
        memory_profile: "balanced".to_string(),
        storage_profile: "high-throughput".to_string(),
        network_profile: "low-latency".to_string(),
    };
    assert_eq!(profile.cpu_profile, "performance");
    assert_eq!(profile.memory_profile, "balanced");
}

#[test]
fn test_benchmark_result_creation() {
    let now = Utc::now();
    let benchmark = BenchmarkResult {
        benchmark_type: "cpu-stress".to_string(),
        score: 8500.0,
        duration_ms: 5000,
        metrics: LiveHardwareMetrics {
            cpu_usage: 95.0,
            memory_usage: 70.0,
            disk_io: 100.0,
            network_io: 50.0,
            power_consumption: 350.0,
            temperature: 75.0,
            gpu_usage: 0.0,
            disk_usage: 60.0,
            network_usage: 25.0,
            timestamp: now,
        },
    };
    assert_eq!(benchmark.benchmark_type, "cpu-stress");
    assert_eq!(benchmark.score, 8500.0);
}

#[test]
fn test_tuning_result_creation() {
    let now = Utc::now();
    let before = LiveHardwareMetrics {
        cpu_usage: 50.0,
        memory_usage: 60.0,
        disk_io: 80.0,
        network_io: 40.0,
        power_consumption: 280.0,
        temperature: 55.0,
        gpu_usage: 10.0,
        disk_usage: 65.0,
        network_usage: 20.0,
        timestamp: now,
    };

    let after = LiveHardwareMetrics {
        cpu_usage: 45.0,
        memory_usage: 55.0,
        disk_io: 90.0,
        network_io: 50.0,
        power_consumption: 300.0,
        temperature: 58.0,
        gpu_usage: 15.0,
        disk_usage: 65.0,
        network_usage: 25.0,
        timestamp: now,
    };

    let result = TuningResult {
        profile_name: "balanced".to_string(),
        optimizations_applied: vec!["cpu-governor".to_string(), "memory-tuning".to_string()],
        estimated_power_increase: 20.0,
        performance_improvement: 15.5,
        before_metrics: before,
        after_metrics: after,
    };

    assert_eq!(result.profile_name, "balanced");
    assert_eq!(result.optimizations_applied.len(), 2);
    assert_eq!(result.performance_improvement, 15.5);
}

#[test]
fn test_cpu_info_serialization() {
    let info = CpuInfo {
        cores: 8,
        model: "AMD Ryzen 7".to_string(),
    };

    let json = serde_json::to_string(&info).expect("Should serialize");
    let deserialized: CpuInfo = serde_json::from_str(&json).expect("Should deserialize");
    assert_eq!(info.cores, deserialized.cores);
    assert_eq!(info.model, deserialized.model);
}

#[test]
fn test_memory_info_serialization() {
    let info = MemoryInfo { total_gb: 64 };

    let json = serde_json::to_string(&info).expect("Should serialize");
    let deserialized: MemoryInfo = serde_json::from_str(&json).expect("Should deserialize");
    assert_eq!(info.total_gb, deserialized.total_gb);
}

#[test]
fn test_gpu_info_serialization() {
    let info = GpuInfo {
        name: "NVIDIA RTX 4090".to_string(),
        memory_mb: 24576,
    };

    let json = serde_json::to_string(&info).expect("Should serialize");
    let deserialized: GpuInfo = serde_json::from_str(&json).expect("Should deserialize");
    assert_eq!(info.name, deserialized.name);
    assert_eq!(info.memory_mb, deserialized.memory_mb);
}

#[test]
fn test_available_resources_creation() {
    let resources = AvailableResources {
        available_cpu: 12,
        available_memory_gb: 48,
        available_gpu: 2,
    };
    assert_eq!(resources.available_cpu, 12);
    assert_eq!(resources.available_memory_gb, 48);
}

#[test]
fn test_compute_resource_request() {
    let request = ComputeResourceRequest {
        cpu_cores: 4,
        memory_gb: 16,
        gpu_count: 1,
    };
    assert_eq!(request.cpu_cores, 4);
    assert_eq!(request.memory_gb, 16);
    assert_eq!(request.gpu_count, 1);
}

#[test]
fn test_live_hardware_metrics_serialization() {
    let metrics = LiveHardwareMetrics {
        cpu_usage: 55.5,
        memory_usage: 70.2,
        disk_io: 100.0,
        network_io: 80.5,
        power_consumption: 300.0,
        temperature: 65.0,
        gpu_usage: 40.0,
        disk_usage: 75.0,
        network_usage: 35.0,
        timestamp: Utc::now(),
    };

    let json = serde_json::to_string(&metrics).expect("Should serialize");
    let deserialized: LiveHardwareMetrics =
        serde_json::from_str(&json).expect("Should deserialize");
    assert_eq!(metrics.cpu_usage, deserialized.cpu_usage);
    assert_eq!(metrics.memory_usage, deserialized.memory_usage);
}
