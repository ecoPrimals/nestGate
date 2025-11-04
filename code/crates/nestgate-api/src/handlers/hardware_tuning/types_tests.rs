//! **COMPREHENSIVE TESTS FOR HARDWARE TUNING TYPES**
//!
//! Test coverage for types.rs - hardware tuning data structures.

#[cfg(test)]
mod tests {
    use super::super::types::*;
    use std::time::Duration;

    // ==================== HARDWARE TUNING CONFIG TESTS ====================

    #[test]
    fn test_hardware_tuning_config_default() {
        let config = HardwareTuningConfig::default();

        assert_eq!(config.cpu_cores, 8);
        assert_eq!(config.memory_gb, 16);
        assert!(config.cpu_tuning_enabled);
        assert!(config.memory_optimization_enabled);
        assert!(!config.gpu_tuning_enabled); // Disabled by default
        assert_eq!(config.monitoring_interval, Duration::from_secs(5));
    }

    #[test]
    fn test_hardware_tuning_config_custom() {
        let config = HardwareTuningConfig {
            cpu_cores: 16,
            memory_gb: 32,
            cpu_tuning_enabled: true,
            memory_optimization_enabled: true,
            gpu_tuning_enabled: true,
            monitoring_interval: Duration::from_secs(10),
        };

        assert_eq!(config.cpu_cores, 16);
        assert_eq!(config.memory_gb, 32);
        assert!(config.gpu_tuning_enabled);
        assert_eq!(config.monitoring_interval, Duration::from_secs(10));
    }

    #[test]
    fn test_hardware_tuning_config_serialization() {
        let config = HardwareTuningConfig::default();

        let json = serde_json::to_string(&config);
        assert!(json.is_ok());

        let json_str = json.expect("Test setup failed");
        assert!(json_str.contains("\"cpu_cores\""));
        assert!(json_str.contains("\"memory_gb\""));
    }

    #[test]
    fn test_hardware_tuning_config_deserialization() {
        let json = r#"{
            "cpu_cores": 12,
            "memory_gb": 24,
            "cpu_tuning_enabled": true,
            "memory_optimization_enabled": false,
            "gpu_tuning_enabled": true,
            "monitoring_interval": {"secs": 15, "nanos": 0}
        }"#;

        let config: Result<HardwareTuningConfig, _> = serde_json::from_str(json);
        assert!(config.is_ok());

        let config = config.expect("Test setup failed");
        assert_eq!(config.cpu_cores, 12);
        assert_eq!(config.memory_gb, 24);
        assert!(!config.memory_optimization_enabled);
    }

    #[test]
    fn test_hardware_tuning_config_clone() {
        let config = HardwareTuningConfig::default();
        let cloned = config.clone();

        assert_eq!(config.cpu_cores, cloned.cpu_cores);
        assert_eq!(config.memory_gb, cloned.memory_gb);
        assert_eq!(config.cpu_tuning_enabled, cloned.cpu_tuning_enabled);
    }

    // ==================== COMPUTE ALLOCATION TESTS ====================

    #[test]
    fn test_compute_allocation_creation() {
        let allocation = ComputeAllocation {
            cpu_cores: 8,
            memory_gb: 16,
            gpu_count: 2,
        };

        assert_eq!(allocation.cpu_cores, 8);
        assert_eq!(allocation.memory_gb, 16);
        assert_eq!(allocation.gpu_count, 2);
    }

    #[test]
    fn test_compute_allocation_serialization() {
        let allocation = ComputeAllocation {
            cpu_cores: 4,
            memory_gb: 8,
            gpu_count: 1,
        };

        let json = serde_json::to_string(&allocation).expect("Should serialize");
        assert!(json.contains("\"cpu_cores\":4"));
        assert!(json.contains("\"memory_gb\":8"));
        assert!(json.contains("\"gpu_count\":1"));
    }

    #[test]
    fn test_compute_allocation_deserialization() {
        let json = r#"{"cpu_cores": 16, "memory_gb": 32, "gpu_count": 4}"#;
        let allocation: ComputeAllocation = serde_json::from_str(json).expect("Should deserialize");

        assert_eq!(allocation.cpu_cores, 16);
        assert_eq!(allocation.memory_gb, 32);
        assert_eq!(allocation.gpu_count, 4);
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

    // ==================== COMPUTE RESOURCES TESTS ====================

    #[test]
    fn test_compute_resources_creation() {
        let resources = ComputeResources {
            available_cpu: 16,
            available_memory_gb: 64,
            available_gpu: 2,
        };

        assert_eq!(resources.available_cpu, 16);
        assert_eq!(resources.available_memory_gb, 64);
        assert_eq!(resources.available_gpu, 2);
    }

    #[test]
    fn test_compute_resources_serialization() {
        let resources = ComputeResources {
            available_cpu: 8,
            available_memory_gb: 32,
            available_gpu: 1,
        };

        let json = serde_json::to_string(&resources).expect("Should serialize");
        let deserialized: ComputeResources =
            serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(deserialized.available_cpu, resources.available_cpu);
        assert_eq!(
            deserialized.available_memory_gb,
            resources.available_memory_gb
        );
        assert_eq!(deserialized.available_gpu, resources.available_gpu);
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
    fn test_compute_resource_request_serialization() {
        let request = ComputeResourceRequest {
            cpu_cores: 2,
            memory_gb: 4,
            gpu_count: 0,
        };

        let json = serde_json::to_string(&request).expect("Should serialize");
        assert!(json.contains("\"cpu_cores\":2"));
        assert!(json.contains("\"gpu_count\":0"));
    }

    #[test]
    fn test_compute_resource_request_large_values() {
        let request = ComputeResourceRequest {
            cpu_cores: 128,
            memory_gb: 1024,
            gpu_count: 8,
        };

        assert_eq!(request.cpu_cores, 128);
        assert_eq!(request.memory_gb, 1024);
        assert_eq!(request.gpu_count, 8);
    }

    // ==================== AVAILABLE RESOURCES TESTS ====================

    #[test]
    fn test_available_resources_creation() {
        let resources = AvailableResources {
            available_cpu: 12,
            available_memory_gb: 48,
            available_gpu: 3,
        };

        assert_eq!(resources.available_cpu, 12);
        assert_eq!(resources.available_memory_gb, 48);
        assert_eq!(resources.available_gpu, 3);
    }

    #[test]
    fn test_available_resources_full_utilization() {
        let resources = AvailableResources {
            available_cpu: 0,
            available_memory_gb: 0,
            available_gpu: 0,
        };

        assert_eq!(resources.available_cpu, 0);
        assert_eq!(resources.available_memory_gb, 0);
        assert_eq!(resources.available_gpu, 0);
    }

    // ==================== ADDITIONAL STRUCTURE TESTS ====================

    #[test]
    fn test_multiple_allocations() {
        let alloc1 = ComputeAllocation {
            cpu_cores: 4,
            memory_gb: 8,
            gpu_count: 1,
        };

        let alloc2 = ComputeAllocation {
            cpu_cores: 8,
            memory_gb: 16,
            gpu_count: 2,
        };

        // Simulate resource tracking
        let total_cpu = alloc1.cpu_cores + alloc2.cpu_cores;
        let total_memory = alloc1.memory_gb + alloc2.memory_gb;
        let total_gpus = alloc1.gpu_count + alloc2.gpu_count;

        assert_eq!(total_cpu, 12);
        assert_eq!(total_memory, 24);
        assert_eq!(total_gpus, 3);
    }

    #[test]
    fn test_resource_request_validation_logic() {
        let request = ComputeResourceRequest {
            cpu_cores: 16,
            memory_gb: 32,
            gpu_count: 2,
        };

        let available = AvailableResources {
            available_cpu: 20,
            available_memory_gb: 64,
            available_gpu: 4,
        };

        // Simulate validation
        let can_fulfill = request.cpu_cores <= available.available_cpu
            && request.memory_gb <= available.available_memory_gb
            && request.gpu_count <= available.available_gpu;

        assert!(can_fulfill, "Resources should be sufficient");
    }

    #[test]
    fn test_resource_request_insufficient_resources() {
        let request = ComputeResourceRequest {
            cpu_cores: 32,
            memory_gb: 128,
            gpu_count: 8,
        };

        let available = AvailableResources {
            available_cpu: 16,
            available_memory_gb: 64,
            available_gpu: 2,
        };

        let can_fulfill = request.cpu_cores <= available.available_cpu
            && request.memory_gb <= available.available_memory_gb
            && request.gpu_count <= available.available_gpu;

        assert!(!can_fulfill, "Resources should be insufficient");
    }

    #[test]
    fn test_config_with_disabled_features() {
        let config = HardwareTuningConfig {
            cpu_cores: 4,
            memory_gb: 8,
            cpu_tuning_enabled: false,
            memory_optimization_enabled: false,
            gpu_tuning_enabled: false,
            monitoring_interval: Duration::from_secs(60),
        };

        assert!(!config.cpu_tuning_enabled);
        assert!(!config.memory_optimization_enabled);
        assert!(!config.gpu_tuning_enabled);
        assert_eq!(config.monitoring_interval, Duration::from_secs(60));
    }

    #[test]
    fn test_config_high_performance_profile() {
        let config = HardwareTuningConfig {
            cpu_cores: 64,
            memory_gb: 256,
            cpu_tuning_enabled: true,
            memory_optimization_enabled: true,
            gpu_tuning_enabled: true,
            monitoring_interval: Duration::from_secs(1), // Frequent monitoring
        };

        assert_eq!(config.cpu_cores, 64);
        assert_eq!(config.memory_gb, 256);
        assert_eq!(config.monitoring_interval, Duration::from_secs(1));
    }

    #[test]
    fn test_serialization_round_trip() {
        let original = HardwareTuningConfig {
            cpu_cores: 10,
            memory_gb: 20,
            cpu_tuning_enabled: true,
            memory_optimization_enabled: false,
            gpu_tuning_enabled: true,
            monitoring_interval: Duration::from_secs(7),
        };

        let _serialized = serde_json::to_string(&original)
            .map_err(|e| NestGateError::Configuration { message: "Should serialize".to_string(), source: Some(Box::new(e)) })
            .expect("Serialization should succeed");
        let deserialized: HardwareTuningConfig = serde_json::from_str(&_serialized)
            .map_err(|e| NestGateError::Configuration { message: "Should deserialize".to_string(), source: Some(Box::new(e)) })
            .expect("Deserialization should succeed");

        assert_eq!(original.cpu_cores, deserialized.cpu_cores);
        assert_eq!(original.memory_gb, deserialized.memory_gb);
        assert_eq!(original.cpu_tuning_enabled, deserialized.cpu_tuning_enabled);
        assert_eq!(
            original.memory_optimization_enabled,
            deserialized.memory_optimization_enabled
        );
        assert_eq!(original.gpu_tuning_enabled, deserialized.gpu_tuning_enabled);
    }

    #[test]
    fn test_compute_resources_clone() {
        let resources = ComputeResources {
            available_cpu: 24,
            available_memory_gb: 96,
            available_gpu: 4,
        };

        let cloned = resources.clone();
        assert_eq!(resources.available_cpu, cloned.available_cpu);
        assert_eq!(resources.available_memory_gb, cloned.available_memory_gb);
        assert_eq!(resources.available_gpu, cloned.available_gpu);
    }

    #[test]
    fn test_compute_allocation_debug_format() {
        let allocation = ComputeAllocation {
            cpu_cores: 8,
            memory_gb: 16,
            gpu_count: 2,
        };

        let debug_str = format!("{allocation:?}");
        assert!(debug_str.contains("ComputeAllocation"));
        assert!(debug_str.contains("cpu_cores"));
    }
}