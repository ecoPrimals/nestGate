// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Comprehensive tests for hardware tuning types
//!
//! This test suite provides extensive coverage for all hardware tuning data structures,
//! focusing on validation, serialization, and default behaviors.

use super::types::*;
use chrono::Utc;
use serde_json;
use std::time::Duration;

// ==================== HARDWARE TUNING CONFIG TESTS ====================

#[test]
fn test_hardware_tuning_config_default() {
    let config = HardwareTuningConfig::default();
    assert_eq!(config.cpu_cores, 8);
    assert_eq!(config.memory_gb, 16);
    assert!(config.cpu_tuning_enabled);
    assert!(config.memory_optimization_enabled);
    assert!(!config.gpu_tuning_enabled); // GPU disabled by default
    assert_eq!(config.monitoring_interval, Duration::from_secs(5));
}

#[test]
fn test_hardware_tuning_config_custom() {
    let config = HardwareTuningConfig {
        cpu_cores: 16,
        memory_gb: 32,
        cpu_tuning_enabled: false,
        memory_optimization_enabled: true,
        gpu_tuning_enabled: true,
        monitoring_interval: Duration::from_secs(10),
    };
    assert_eq!(config.cpu_cores, 16);
    assert_eq!(config.memory_gb, 32);
    assert!(!config.cpu_tuning_enabled);
    assert!(config.memory_optimization_enabled);
    assert!(config.gpu_tuning_enabled);
    assert_eq!(config.monitoring_interval, Duration::from_secs(10));
}

#[test]
fn test_hardware_tuning_config_serialization() {
    let config = HardwareTuningConfig::default();
    let json = serde_json::to_string(&config).expect("Failed to serialize");
    assert!(json.contains("cpu_cores"));
    assert!(json.contains("memory_gb"));
    assert!(json.contains("cpu_tuning_enabled"));
}

#[test]
fn test_hardware_tuning_config_deserialization() {
    let json = r#"{
        "cpu_cores": 4,
        "memory_gb": 8,
        "cpu_tuning_enabled": false,
        "memory_optimization_enabled": false,
        "gpu_tuning_enabled": true,
        "monitoring_interval": {"secs": 15, "nanos": 0}
    }"#;
    let config: HardwareTuningConfig = serde_json::from_str(json).expect("Failed to deserialize");
    assert_eq!(config.cpu_cores, 4);
    assert_eq!(config.memory_gb, 8);
    assert!(!config.cpu_tuning_enabled);
    assert!(!config.memory_optimization_enabled);
    assert!(config.gpu_tuning_enabled);
}

#[test]
fn test_hardware_tuning_config_clone() {
    let config = HardwareTuningConfig::default();
    let cloned = config.clone();
    assert_eq!(config.cpu_cores, cloned.cpu_cores);
    assert_eq!(config.memory_gb, cloned.memory_gb);
}

// ==================== COMPUTE ALLOCATION TESTS ====================

#[test]
fn test_compute_allocation_creation() {
    let allocation = ComputeAllocation {
        cpu_cores: 4,
        memory_gb: 8,
        gpu_count: 1,
    };
    assert_eq!(allocation.cpu_cores, 4);
    assert_eq!(allocation.memory_gb, 8);
    assert_eq!(allocation.gpu_count, 1);
}

#[test]
fn test_compute_allocation_zero_gpu() {
    let allocation = ComputeAllocation {
        cpu_cores: 8,
        memory_gb: 16,
        gpu_count: 0,
    };
    assert_eq!(allocation.gpu_count, 0);
}

#[test]
fn test_compute_allocation_serialization() {
    let allocation = ComputeAllocation {
        cpu_cores: 2,
        memory_gb: 4,
        gpu_count: 0,
    };
    let json = serde_json::to_string(&allocation).expect("Failed to serialize");
    assert!(json.contains("cpu_cores"));
    assert!(json.contains("memory_gb"));
    assert!(json.contains("gpu_count"));
}

// ==================== COMPUTE RESOURCES TESTS ====================

#[test]
fn test_compute_resources_creation() {
    let resources = ComputeResources {
        available_cpu: 16,
        available_memory_gb: 32,
        available_gpu: 2,
    };
    assert_eq!(resources.available_cpu, 16);
    assert_eq!(resources.available_memory_gb, 32);
    assert_eq!(resources.available_gpu, 2);
}

#[test]
fn test_compute_resources_no_gpu() {
    let resources = ComputeResources {
        available_cpu: 8,
        available_memory_gb: 16,
        available_gpu: 0,
    };
    assert_eq!(resources.available_gpu, 0);
}

#[test]
fn test_compute_resources_serialization() {
    let resources = ComputeResources {
        available_cpu: 4,
        available_memory_gb: 8,
        available_gpu: 1,
    };
    let json = serde_json::to_string(&resources).expect("Failed to serialize");
    let deserialized: ComputeResources = serde_json::from_str(&json).expect("Failed to deserialize");
    assert_eq!(deserialized.available_cpu, 4);
    assert_eq!(deserialized.available_memory_gb, 8);
    assert_eq!(deserialized.available_gpu, 1);
}

// ==================== COMPUTE RESOURCE REQUEST TESTS ====================

#[test]
fn test_compute_resource_request_creation() {
    let request = ComputeResourceRequest {
        cpu_cores: 4,
        memory_gb: 8,
        gpu_count: 1,
    };
    assert_eq!(request.cpu_cores, 4);
    assert_eq!(request.memory_gb, 8);
    assert_eq!(request.gpu_count, 1);
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
fn test_compute_resource_request_high_demand() {
    let request = ComputeResourceRequest {
        cpu_cores: 64,
        memory_gb: 256,
        gpu_count: 8,
    };
    assert_eq!(request.cpu_cores, 64);
    assert_eq!(request.memory_gb, 256);
    assert_eq!(request.gpu_count, 8);
}

// ==================== AVAILABLE RESOURCES TESTS ====================

#[test]
fn test_available_resources_creation() {
    let resources = AvailableResources {
        available_cpu: 32,
        available_memory_gb: 64,
        available_gpu: 4,
    };
    assert_eq!(resources.available_cpu, 32);
    assert_eq!(resources.available_memory_gb, 64);
    assert_eq!(resources.available_gpu, 4);
}

#[test]
fn test_available_resources_exhausted() {
    let resources = AvailableResources {
        available_cpu: 0,
        available_memory_gb: 0,
        available_gpu: 0,
    };
    assert_eq!(resources.available_cpu, 0);
    assert_eq!(resources.available_memory_gb, 0);
    assert_eq!(resources.available_gpu, 0);
}

// ==================== GPU ALLOCATION TESTS ====================

#[test]
fn test_gpu_allocation_creation() {
    let gpu = GpuAllocation {
        gpu_id: "GPU-0".to_string(),
        gpu_memory_gb: 16,
        compute_capability: "8.0".to_string(),
    };
    assert_eq!(gpu.gpu_id, "GPU-0");
    assert_eq!(gpu.gpu_memory_gb, 16);
    assert_eq!(gpu.compute_capability, "8.0");
}

#[test]
fn test_gpu_allocation_serialization() {
    let gpu = GpuAllocation {
        gpu_id: "GPU-1".to_string(),
        gpu_memory_gb: 24,
        compute_capability: "9.0".to_string(),
    };
    let json = serde_json::to_string(&gpu).expect("Failed to serialize");
    let deserialized: GpuAllocation = serde_json::from_str(&json).expect("Failed to deserialize");
    assert_eq!(deserialized.gpu_id, "GPU-1");
    assert_eq!(deserialized.gpu_memory_gb, 24);
}

// ==================== INTEGRATION TESTS ====================

#[test]
fn test_config_with_max_resources() {
    let config = HardwareTuningConfig {
        cpu_cores: 128,
        memory_gb: 1024,
        cpu_tuning_enabled: true,
        memory_optimization_enabled: true,
        gpu_tuning_enabled: true,
        monitoring_interval: Duration::from_secs(1),
    };
    assert_eq!(config.cpu_cores, 128);
    assert_eq!(config.memory_gb, 1024);
}

#[test]
fn test_resource_request_allocation_match() {
    let request = ComputeResourceRequest {
        cpu_cores: 8,
        memory_gb: 16,
        gpu_count: 2,
    };
    
    let allocation = ComputeAllocation {
        cpu_cores: request.cpu_cores,
        memory_gb: request.memory_gb,
        gpu_count: request.gpu_count,
    };
    
    assert_eq!(request.cpu_cores, allocation.cpu_cores);
    assert_eq!(request.memory_gb, allocation.memory_gb);
    assert_eq!(request.gpu_count, allocation.gpu_count);
}

#[test]
fn test_monitoring_interval_variations() {
    let intervals = vec![
        Duration::from_secs(1),
        Duration::from_secs(5),
        Duration::from_secs(10),
        Duration::from_secs(60),
    ];
    
    for interval in intervals {
        let config = HardwareTuningConfig {
            cpu_cores: 8,
            memory_gb: 16,
            cpu_tuning_enabled: true,
            memory_optimization_enabled: true,
            gpu_tuning_enabled: false,
            monitoring_interval: interval,
        };
        assert_eq!(config.monitoring_interval, interval);
    }
}

// ==================== EDGE CASES TESTS ====================

#[test]
fn test_config_all_disabled() {
    let config = HardwareTuningConfig {
        cpu_cores: 1,
        memory_gb: 1,
        cpu_tuning_enabled: false,
        memory_optimization_enabled: false,
        gpu_tuning_enabled: false,
        monitoring_interval: Duration::from_secs(60),
    };
    assert!(!config.cpu_tuning_enabled);
    assert!(!config.memory_optimization_enabled);
    assert!(!config.gpu_tuning_enabled);
}

#[test]
fn test_zero_interval() {
    let config = HardwareTuningConfig {
        cpu_cores: 8,
        memory_gb: 16,
        cpu_tuning_enabled: true,
        memory_optimization_enabled: true,
        gpu_tuning_enabled: false,
        monitoring_interval: Duration::from_secs(0),
    };
    assert_eq!(config.monitoring_interval, Duration::from_secs(0));
}

// ==================== ROUNDTRIP TESTS ====================

#[test]
fn test_hardware_tuning_config_roundtrip() {
    let original = HardwareTuningConfig {
        cpu_cores: 12,
        memory_gb: 24,
        cpu_tuning_enabled: true,
        memory_optimization_enabled: false,
        gpu_tuning_enabled: true,
        monitoring_interval: Duration::from_secs(7),
    };
    
    let json = serde_json::to_string(&original).expect("Serialization failed");
    let deserialized: HardwareTuningConfig = serde_json::from_str(&json).expect("Deserialization failed");
    
    assert_eq!(original.cpu_cores, deserialized.cpu_cores);
    assert_eq!(original.memory_gb, deserialized.memory_gb);
    assert_eq!(original.cpu_tuning_enabled, deserialized.cpu_tuning_enabled);
    assert_eq!(original.memory_optimization_enabled, deserialized.memory_optimization_enabled);
    assert_eq!(original.gpu_tuning_enabled, deserialized.gpu_tuning_enabled);
}

#[test]
fn test_all_types_roundtrip() {
    let allocation = ComputeAllocation {
        cpu_cores: 4,
        memory_gb: 8,
        gpu_count: 1,
    };
    let json = serde_json::to_string(&allocation).unwrap();
    let recovered: ComputeAllocation = serde_json::from_str(&json).unwrap();
    assert_eq!(allocation.cpu_cores, recovered.cpu_cores);
    
    let resources = ComputeResources {
        available_cpu: 16,
        available_memory_gb: 32,
        available_gpu: 2,
    };
    let json = serde_json::to_string(&resources).unwrap();
    let recovered: ComputeResources = serde_json::from_str(&json).unwrap();
    assert_eq!(resources.available_cpu, recovered.available_cpu);
    
    let request = ComputeResourceRequest {
        cpu_cores: 8,
        memory_gb: 16,
        gpu_count: 1,
    };
    let json = serde_json::to_string(&request).unwrap();
    let recovered: ComputeResourceRequest = serde_json::from_str(&json).unwrap();
    assert_eq!(request.cpu_cores, recovered.cpu_cores);
}

// ==================== PERFORMANCE TESTS ====================

#[test]
fn test_config_clone_performance() {
    let config = HardwareTuningConfig::default();
    for _ in 0..1000 {
        let _cloned = config.clone();
    }
    // Test passes if no panic occurs
}

#[test]
fn test_serialization_performance() {
    let config = HardwareTuningConfig::default();
    for _ in 0..100 {
        let json = serde_json::to_string(&config).expect("Serialization failed");
        assert!(!json.is_empty());
    }
}
