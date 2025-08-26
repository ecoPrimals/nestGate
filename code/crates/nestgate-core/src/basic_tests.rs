use crate::NestGateError;
use std::collections::HashMap;
//
// These tests provide basic coverage for core types and functions.

use crate::error::NestGateError;
use crate::types::{AllocationStatus, HealthStatus, StorageTier, SystemInfo};
use crate::canonical_modernization::{UnifiedHealthStatus, UnifiedServiceState};
use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_storage_tier_variants() {
        let hot = StorageTier::Hot;
        let warm = StorageTier::Warm;
        let cold = StorageTier::Cold;
        let cache = StorageTier::Cache;
        let archive = StorageTier::Archive;

        // Test that we can create all variants
        assert_eq!(hot.to_string(), "Hot");
        assert_eq!(warm.to_string(), "Warm");
        assert_eq!(cold.to_string(), "Cold");
        assert_eq!(cache.to_string(), "Cache");
        assert_eq!(archive.to_string(), "Archive");
    }

    #[test]
    fn test_allocation_status() {
        let active = AllocationStatus::Active;
        let inactive = AllocationStatus::Inactive;
        let pending = AllocationStatus::Pending;
        let failed = AllocationStatus::Failed;

        // Basic enum variant tests
        match active {
            AllocationStatus::Active => assert!(true),
            _ => assert!(false, "Expected Active variant"),
        }

        match inactive {
            AllocationStatus::Inactive => assert!(true),
            _ => assert!(false, "Expected Inactive variant"),
        }
    }

    #[test]
    fn test_unified_health_status() {
        let healthy = UnifiedHealthStatus::Healthy;
        let degraded = UnifiedHealthStatus::Degraded;
        let unhealthy = UnifiedHealthStatus::Unhealthy;
        let unknown = UnifiedHealthStatus::Unknown;

        // Test enum variants exist
        match healthy {
            UnifiedHealthStatus::Healthy => assert!(true),
            _ => assert!(false, "Expected Healthy variant"),
        }

        match degraded {
            UnifiedHealthStatus::Degraded => assert!(true),
            _ => assert!(false, "Expected Degraded variant"),
        }
    }

    #[test]
    fn test_unified_service_state() {
        let starting = UnifiedServiceState::Starting;
        let running = UnifiedServiceState::Running;
        let stopping = UnifiedServiceState::Stopping;
        let stopped = UnifiedServiceState::Stopped;

        // Test enum variants
        match starting {
            UnifiedServiceState::Starting => assert!(true),
            _ => assert!(false, "Expected Starting variant"),
        }

        match running {
            UnifiedServiceState::Running => assert!(true),
            _ => assert!(false, "Expected Running variant"),
        }
    }

    #[test]
    fn test_health_status_default() {
        let health = HealthStatus::default();

        assert!(health.overall_healthy);
        assert_eq!(health.cpu_usage_percent, 0.0);
        assert_eq!(health.memory_usage_percent, 0.0);
        assert_eq!(health.disk_usage_percent, 0.0);
        assert!(health.network_connected);
        assert_eq!(health.services_running, vec!["nestgate-core".to_string()]);
    }

    #[test]
    fn test_system_info_construction() {
        let system_info = SystemInfo {
            hostname: "test-host".to_string(),
            os_type: "Linux".to_string(),
            architecture: "x86_64".to_string(),
            os_version: "5.15.0".to_string(),
            total_memory: 8_589_934_592,     // 8GB
            available_memory: 4_294_967_296, // 4GB
            cpu_count: 8,
            uptime_seconds: 86400, // 1 day
        };

        assert_eq!(system_info.hostname, "test-host");
        assert_eq!(system_info.os_type, "Linux");
        assert_eq!(system_info.architecture, "x86_64");
        assert_eq!(system_info.cpu_count, 8);
        assert_eq!(system_info.total_memory, 8_589_934_592);
    }

    #[test]
    fn test_nestgate_error_creation() {
        let config_error = NestGateError::Configuration {
            message: "Test configuration error".to_string(),
            context: "test_context".to_string(),
            source: None,
        };

        match config_error {
            NestGateError::Configuration {
                message, context, ..
            } => {
                assert_eq!(message, "Test configuration error");
                assert_eq!(context, "test_context");
            }
            _ => return Err(std::io::Error::new(
    std::io::ErrorKind::Other,
    "Expected Configuration error variant".to_string()
).into()),
        }
    }

    #[test]
    fn test_nestgate_error_validation() {
        let validation_error = NestGateError::Validation {
            field: "test_field".to_string(),
            message: "Invalid value".to_string(),
            current_value: Some("invalid".to_string()),
            expected: Some("valid".to_string()),
            user_error: true,
        };

        match validation_error {
            NestGateError::Validation {
                field,
                message,
                user_error,
                ..
            } => {
                assert_eq!(field, "test_field");
                assert_eq!(message, "Invalid value");
                assert!(user_error);
            }
            _ => return Err(std::io::Error::new(
    std::io::ErrorKind::Other,
    "Expected Validation error variant".to_string()
).into()),
        }
    }

    #[test]
    fn test_error_display() {
        let error = NestGateError::Internal {
            message: "Test internal error".to_string(),
            location: Some("test_module".to_string()),
            debug_info: None,
            is_bug: false,
        };

        let error_string = format!("{}", error);
        assert!(error_string.contains("Test internal error"));
    }

    #[test]
    fn test_basic_string_operations() {
        let test_string = "nestgate".to_string();
        assert_eq!(test_string.len(), 8);
        assert!(test_string.contains("gate"));
        assert!(test_string.starts_with("nest"));
    }

    #[test]
    fn test_basic_collections() {
        let mut map = HashMap::new();
        map.insert("key1".to_string(), "value1".to_string());
        map.insert("key2".to_string(), "value2".to_string());

        assert_eq!(map.len(), 2);
        assert_eq!(map.get("key1"), Some(&"value1".to_string()));
        assert_eq!(map.get("key2"), Some(&"value2".to_string()));
        assert_eq!(map.get("key3"), None);
    }
}
