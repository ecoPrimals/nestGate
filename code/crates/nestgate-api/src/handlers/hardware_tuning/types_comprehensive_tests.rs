// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

/// Comprehensive additional tests for hardware tuning types
/// Targets edge cases, error paths, and uncovered functionality
use super::*;
use serde_json;

#[test]
fn test_hardware_tuning_config_custom_values() {
    let config = HardwareTuningConfig {
        cpu_cores: 32,
        memory_gb: 128,
        cpu_tuning_enabled: false,
        memory_optimization_enabled: false,
        gpu_tuning_enabled: true,
        monitoring_interval: Duration::from_secs(1),
    };

    assert_eq!(config.cpu_cores, 32);
    assert_eq!(config.memory_gb, 128);
    assert!(!config.cpu_tuning_enabled);
    assert!(!config.memory_optimization_enabled);
    assert!(config.gpu_tuning_enabled);
    assert_eq!(config.monitoring_interval, Duration::from_secs(1));
}

#[test]
fn test_hardware_tuning_config_serialization() {
    let config = HardwareTuningConfig::default();
    let json = serde_json::to_string(&config).expect("Should serialize");

    assert!(json.contains("\"cpu_cores\":8"));
    assert!(json.contains("\"memory_gb\":16"));
    assert!(json.contains("\"cpu_tuning_enabled\":true"));
}

#[test]
fn test_hardware_tuning_config_deserialization() {
    let json = r#"{
        "cpu_cores": 16,
        "memory_gb": 64,
        "cpu_tuning_enabled": true,
        "memory_optimization_enabled": true,
        "gpu_tuning_enabled": false,
        "monitoring_interval": {"secs": 10, "nanos": 0}
    }"#;

    let config: HardwareTuningConfig = serde_json::from_str(json).expect("Should deserialize");
    assert_eq!(config.cpu_cores, 16);
    assert_eq!(config.memory_gb, 64);
}

#[test]
fn test_hardware_tuning_config_clone() {
    let config1 = HardwareTuningConfig::default();
    let config2 = config1.clone();

    assert_eq!(config1.cpu_cores, config2.cpu_cores);
    assert_eq!(config1.memory_gb, config2.memory_gb);
}

#[test]
fn test_compute_allocation_zero_resources() {
    let allocation = ComputeAllocation {
        cpu_cores: 0,
        memory_gb: 0,
        gpu_count: 0,
    };

    assert_eq!(allocation.cpu_cores, 0);
    assert_eq!(allocation.memory_gb, 0);
    assert_eq!(allocation.gpu_count, 0);
}

#[test]
fn test_compute_allocation_max_resources() {
    let allocation = ComputeAllocation {
        cpu_cores: 256,
        memory_gb: 1024,
        gpu_count: 16,
    };

    assert_eq!(allocation.cpu_cores, 256);
    assert_eq!(allocation.memory_gb, 1024);
    assert_eq!(allocation.gpu_count, 16);
}

#[test]
fn test_compute_allocation_serialization() {
    let allocation = ComputeAllocation {
        cpu_cores: 8,
        memory_gb: 32,
        gpu_count: 2,
    };

    let json = serde_json::to_string(&allocation).expect("Should serialize");
    let deserialized: ComputeAllocation = serde_json::from_str(&json).expect("Should deserialize");

    assert_eq!(allocation.cpu_cores, deserialized.cpu_cores);
    assert_eq!(allocation.memory_gb, deserialized.memory_gb);
    assert_eq!(allocation.gpu_count, deserialized.gpu_count);
}

#[test]
fn test_compute_resources_partial_availability() {
    let resources = ComputeResources {
        available_cpu: 4,
        available_memory_gb: 0,
        available_gpu: 8,
    };

    assert_eq!(resources.available_cpu, 4);
    assert_eq!(resources.available_memory_gb, 0);
    assert_eq!(resources.available_gpu, 8);
}

#[test]
fn test_compute_resource_request_minimal() {
    let request = ComputeResourceRequest {
        cpu_cores: 1,
        memory_gb: 1,
        gpu_count: 0,
    };

    assert_eq!(request.cpu_cores, 1);
    assert_eq!(request.memory_gb, 1);
    assert_eq!(request.gpu_count, 0);
}

#[test]
fn test_compute_resource_request_serialization() {
    let request = ComputeResourceRequest {
        cpu_cores: 16,
        memory_gb: 64,
        gpu_count: 4,
    };

    let json = serde_json::to_string(&request).expect("Should serialize");
    assert!(json.contains("\"cpu_cores\":16"));
    assert!(json.contains("\"gpu_count\":4"));
}

#[test]
fn test_available_resources_full_capacity() {
    let resources = AvailableResources {
        available_cpu: 128,
        available_memory_gb: 512,
        available_gpu: 8,
    };

    assert_eq!(resources.available_cpu, 128);
    assert_eq!(resources.available_memory_gb, 512);
}

#[test]
fn test_available_resources_no_gpus() {
    let resources = AvailableResources {
        available_cpu: 16,
        available_memory_gb: 64,
        available_gpu: 0,
    };

    assert_eq!(resources.available_gpu, 0);
}

#[test]
fn test_gpu_allocation_multiple_gpus() {
    let gpu1 = GpuAllocation {
        gpu_id: "GPU-0".to_string(),
        memory_gb: 12,
    };

    let gpu2 = GpuAllocation {
        gpu_id: "GPU-1".to_string(),
        memory_gb: 24,
    };

    assert_ne!(gpu1.gpu_id, gpu2.gpu_id);
    assert_ne!(gpu1.memory_gb, gpu2.memory_gb);
}

#[test]
fn test_gpu_allocation_serialization() {
    let allocation = GpuAllocation {
        gpu_id: "GPU-3".to_string(),
        memory_gb: 16,
    };

    let json = serde_json::to_string(&allocation).expect("Should serialize");
    let deserialized: GpuAllocation = serde_json::from_str(&json).expect("Should deserialize");

    assert_eq!(allocation.gpu_id, deserialized.gpu_id);
    assert_eq!(allocation.memory_gb, deserialized.memory_gb);
}

#[test]
fn test_tuning_service_registration_https() {
    let registration = TuningServiceRegistration {
        service_name: "secure-tuning".to_string(),
        endpoint: format!(
            "https://secure.example.com:{}",
            nestgate_core::constants::network_hardcoded::ports::HTTPS_DEFAULT
        ),
    };

    assert!(registration.endpoint.starts_with("https://"));
    assert!(registration.endpoint.contains(&format!(
        ":{}",
        nestgate_core::constants::network_hardcoded::ports::HTTPS_DEFAULT
    )));
}

#[test]
fn test_tuning_service_registration_serialization() {
    let registration = TuningServiceRegistration {
        service_name: "test-service".to_string(),
        endpoint: "http://test.local:9090".to_string(),
    };

    let json = serde_json::to_string(&registration).expect("Should serialize");
    let deserialized: TuningServiceRegistration =
        serde_json::from_str(&json).expect("Should deserialize");

    assert_eq!(registration.service_name, deserialized.service_name);
    assert_eq!(registration.endpoint, deserialized.endpoint);
}

#[test]
fn test_compute_adapter_empty_name() {
    let adapter = ComputeAdapter::new(String::new());
    assert_eq!(adapter.service_name, "");
}

#[test]
fn test_compute_adapter_long_name() {
    let long_name = "a".repeat(1000);
    let adapter = ComputeAdapter::new(long_name.clone());
    assert_eq!(adapter.service_name, long_name);
}

#[test]
fn test_compute_adapter_serialization() {
    let adapter = ComputeAdapter::new("compute-service".to_string());
    let json = serde_json::to_string(&adapter).expect("Should serialize");
    let deserialized: ComputeAdapter = serde_json::from_str(&json).expect("Should deserialize");

    assert_eq!(adapter.service_name, deserialized.service_name);
}

#[test]
fn test_live_hardware_metrics_zero_values() {
    let metrics = LiveHardwareMetrics {
        cpu_usage: 0.0,
        memory_usage: 0.0,
        disk_io: 0.0,
        network_io: 0.0,
        power_consumption: 0.0,
        temperature: 0.0,
        gpu_usage: 0.0,
        disk_usage: 0.0,
        network_usage: 0.0,
        timestamp: Utc::now(),
    };

    assert_eq!(metrics.cpu_usage, 0.0);
    assert_eq!(metrics.power_consumption, 0.0);
}

#[test]
fn test_live_hardware_metrics_max_values() {
    let metrics = LiveHardwareMetrics {
        cpu_usage: 100.0,
        memory_usage: 100.0,
        disk_io: 10000.0,
        network_io: 10000.0,
        power_consumption: 1000.0,
        temperature: 100.0,
        gpu_usage: 100.0,
        disk_usage: 100.0,
        network_usage: 100.0,
        timestamp: Utc::now(),
    };

    assert_eq!(metrics.cpu_usage, 100.0);
    assert_eq!(metrics.memory_usage, 100.0);
}

#[test]
fn test_live_hardware_metrics_clone() {
    let metrics1 = LiveHardwareMetrics {
        cpu_usage: 45.5,
        memory_usage: 60.2,
        disk_io: 100.0,
        network_io: 80.5,
        power_consumption: 300.0,
        temperature: 65.0,
        gpu_usage: 40.0,
        disk_usage: 75.0,
        network_usage: 35.0,
        timestamp: Utc::now(),
    };

    let metrics2 = metrics1.clone();
    assert_eq!(metrics1.cpu_usage, metrics2.cpu_usage);
    assert_eq!(metrics1.temperature, metrics2.temperature);
}

#[tokio::test]
async fn test_performance_snapshot_ordering() {
    let snapshot1 = PerformanceSnapshot {
        timestamp: Utc::now(),
        cpu_usage: 50.0,
        memory_usage: 60.0,
        disk_io: 80.0,
        network_io: 40.0,
    };

    tokio::time::sleep(std::time::Duration::from_millis(10)).await;

    let snapshot2 = PerformanceSnapshot {
        timestamp: Utc::now(),
        cpu_usage: 55.0,
        memory_usage: 65.0,
        disk_io: 85.0,
        network_io: 45.0,
    };

    assert!(snapshot2.timestamp > snapshot1.timestamp);
}

#[test]
fn test_performance_snapshot_serialization() {
    let snapshot = PerformanceSnapshot {
        timestamp: Utc::now(),
        cpu_usage: 42.5,
        memory_usage: 55.3,
        disk_io: 70.2,
        network_io: 35.8,
    };

    let json = serde_json::to_string(&snapshot).expect("Should serialize");
    let deserialized: PerformanceSnapshot =
        serde_json::from_str(&json).expect("Should deserialize");

    assert_eq!(snapshot.cpu_usage, deserialized.cpu_usage);
    assert_eq!(snapshot.memory_usage, deserialized.memory_usage);
}

#[test]
fn test_system_profile_all_modes() {
    let profile = SystemProfile {
        cpu_profile: "power-save".to_string(),
        memory_profile: "conservative".to_string(),
        storage_profile: "balanced".to_string(),
        network_profile: "high-throughput".to_string(),
    };

    assert_eq!(profile.cpu_profile, "power-save");
    assert_eq!(profile.network_profile, "high-throughput");
}

#[test]
fn test_system_profile_serialization() {
    let profile = SystemProfile {
        cpu_profile: "performance".to_string(),
        memory_profile: "balanced".to_string(),
        storage_profile: "high-throughput".to_string(),
        network_profile: "low-latency".to_string(),
    };

    let json = serde_json::to_string(&profile).expect("Should serialize");
    let deserialized: SystemProfile = serde_json::from_str(&json).expect("Should deserialize");

    assert_eq!(profile.cpu_profile, deserialized.cpu_profile);
    assert_eq!(profile.storage_profile, deserialized.storage_profile);
}

#[test]
fn test_benchmark_result_zero_duration() {
    let benchmark = BenchmarkResult {
        benchmark_type: "quick-check".to_string(),
        score: 1000.0,
        duration_ms: 0,
        metrics: LiveHardwareMetrics {
            cpu_usage: 10.0,
            memory_usage: 20.0,
            disk_io: 5.0,
            network_io: 5.0,
            power_consumption: 100.0,
            temperature: 40.0,
            gpu_usage: 0.0,
            disk_usage: 50.0,
            network_usage: 10.0,
            timestamp: Utc::now(),
        },
    };

    assert_eq!(benchmark.duration_ms, 0);
}

#[test]
fn test_benchmark_result_long_duration() {
    let benchmark = BenchmarkResult {
        benchmark_type: "marathon".to_string(),
        score: 50000.0,
        duration_ms: 3600000, // 1 hour
        metrics: LiveHardwareMetrics {
            cpu_usage: 85.0,
            memory_usage: 90.0,
            disk_io: 200.0,
            network_io: 150.0,
            power_consumption: 400.0,
            temperature: 80.0,
            gpu_usage: 95.0,
            disk_usage: 80.0,
            network_usage: 60.0,
            timestamp: Utc::now(),
        },
    };

    assert_eq!(benchmark.duration_ms, 3600000);
    assert_eq!(benchmark.score, 50000.0);
}

#[test]
fn test_tuning_result_no_optimizations() {
    let now = Utc::now();
    let metrics = LiveHardwareMetrics {
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

    let result = TuningResult {
        profile_name: "baseline".to_string(),
        optimizations_applied: vec![],
        estimated_power_increase: 0.0,
        performance_improvement: 0.0,
        before_metrics: metrics.clone(),
        after_metrics: metrics,
    };

    assert_eq!(result.optimizations_applied.len(), 0);
    assert_eq!(result.performance_improvement, 0.0);
}

#[test]
fn test_tuning_result_many_optimizations() {
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
        cpu_usage: 35.0,
        memory_usage: 45.0,
        disk_io: 120.0,
        network_io: 60.0,
        power_consumption: 350.0,
        temperature: 65.0,
        gpu_usage: 25.0,
        disk_usage: 65.0,
        network_usage: 30.0,
        timestamp: now,
    };

    let result = TuningResult {
        profile_name: "aggressive".to_string(),
        optimizations_applied: vec![
            "cpu-governor".to_string(),
            "memory-tuning".to_string(),
            "disk-scheduler".to_string(),
            "network-tuning".to_string(),
            "gpu-optimization".to_string(),
        ],
        estimated_power_increase: 70.0,
        performance_improvement: 50.0,
        before_metrics: before,
        after_metrics: after,
    };

    assert_eq!(result.optimizations_applied.len(), 5);
    assert_eq!(result.performance_improvement, 50.0);
}

#[test]
fn test_cpu_info_various_models() {
    let intel = CpuInfo {
        cores: 8,
        model: "Intel Core i9-12900K".to_string(),
    };

    let amd = CpuInfo {
        cores: 16,
        model: "AMD Ryzen 9 5950X".to_string(),
    };

    assert_ne!(intel.cores, amd.cores);
    assert_ne!(intel.model, amd.model);
}

#[test]
fn test_memory_info_various_capacities() {
    let small = MemoryInfo { total_gb: 8 };
    let medium = MemoryInfo { total_gb: 32 };
    let large = MemoryInfo { total_gb: 256 };

    assert!(small.total_gb < medium.total_gb);
    assert!(medium.total_gb < large.total_gb);
}

#[test]
fn test_gpu_info_various_models() {
    let nvidia_low = GpuInfo {
        name: "NVIDIA RTX 3060".to_string(),
        memory_mb: 12288,
    };

    let nvidia_high = GpuInfo {
        name: "NVIDIA RTX 4090".to_string(),
        memory_mb: 24576,
    };

    assert_ne!(nvidia_low.name, nvidia_high.name);
    assert!(nvidia_low.memory_mb < nvidia_high.memory_mb);
}

#[test]
fn test_system_capabilities_no_gpu() {
    let caps = SystemCapabilities {
        cpu_cores: 16,
        cpu_model: "AMD EPYC 7543".to_string(),
        memory_gb: 128,
        gpu_available: false,
        gpu_info: None,
    };

    assert!(!caps.gpu_available);
    assert!(caps.gpu_info.is_none());
}

#[test]
fn test_system_capabilities_with_gpu() {
    let caps = SystemCapabilities {
        cpu_cores: 32,
        cpu_model: "Intel Xeon Platinum 8380".to_string(),
        memory_gb: 512,
        gpu_available: true,
        gpu_info: Some(GpuInfo {
            name: "NVIDIA A100".to_string(),
            memory_mb: 40960,
        }),
    };

    assert!(caps.gpu_available);
    assert!(caps.gpu_info.is_some());
    assert_eq!(caps.gpu_info.unwrap().name, "NVIDIA A100");
}

// ✅ FIXED: Session ID generation now uses UUIDs for guaranteed uniqueness
#[test]
fn test_live_hardware_tuning_session_id_uniqueness() {
    let session1 = LiveHardwareTuningSession::new().expect("Should create session");
    let session2 = LiveHardwareTuningSession::new().expect("Should create session");

    assert_ne!(
        session1.session_id, session2.session_id,
        "Session IDs should be unique (UUID-based). session1={}, session2={}",
        session1.session_id, session2.session_id
    );
}

#[test]
fn test_system_metrics_collector_initialization() {
    let collector = SystemMetricsCollector::new().expect("Should create collector");

    // GPU monitor should be None by default (not detected)
    assert!(collector.gpu_monitor.is_none());
}
