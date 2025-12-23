//! Basic Tests module

use crate::error::NestGateError;
use std::collections::HashMap;
//
// These tests provide basic coverage for core types and functions.

use crate::canonical_types::{AllocationStatus, HealthStatus, StorageTier, SystemInfo};
use crate::canonical_modernization::{UnifiedHealthStatus, UnifiedServiceState};

use crate::error::NestGateError;
use crate::canonical_types::StorageTier;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_storage_tier_variants() -> std::result::Result<(), Box<dyn std::error::Error>> {
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
    Ok(())
    }

    #[test]
    fn test_allocation_status() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let active = AllocationStatus::Active;
        let inactive = AllocationStatus::Inactive;
        let pending = AllocationStatus::Pending;
        let failed = AllocationStatus::Failed;

        // Test enum variants can be constructed
        assert!(matches!(active, AllocationStatus::Active));
        assert!(matches!(inactive, AllocationStatus::Inactive));
    Ok(())
    }

    #[test]
    fn test_unified_health_status() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let healthy = UnifiedHealthStatus::Healthy;
        let degraded = UnifiedHealthStatus::Degraded;
        let unhealthy = UnifiedHealthStatus::Unhealthy;
        let unknown = UnifiedHealthStatus::Unknown;

        // Test enum variants exist
        assert!(matches!(healthy, UnifiedHealthStatus::Healthy));
        assert!(matches!(degraded, UnifiedHealthStatus::Degraded));
    Ok(())
    }

    #[test]
    fn test_unified_service_state() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let starting = UnifiedServiceState::Starting;
        let running = UnifiedServiceState::Running;
        let stopping = UnifiedServiceState::Stopping;
        let stopped = UnifiedServiceState::Stopped;

        // Test enum variants
        assert!(matches!(starting, UnifiedServiceState::Starting));
        assert!(matches!(running, UnifiedServiceState::Running));
    Ok(())
    }

    #[test]
    fn test_health_status_default() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let health = HealthStatus::default();

        assert!(health.overall_healthy);
        assert_eq!(health.cpu_usage_percent, 0.0);
        assert_eq!(health.memory_usage_percent, 0.0);
        assert_eq!(health.disk_usage_percent, 0.0);
        assert!(health.network_connected);
        assert_eq!(health.services_running, vec!["nestgate-core".to_string()]);
    Ok(())
    }

    #[test]
    fn test_system_info_construction() -> std::result::Result<(), Box<dyn std::error::Error>> {
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
    Ok(())
    }

    #[test]
    fn test_nestgate_error_creation() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let config_error = NestGateError::configuration(
            context: "test_context".to_string(),
            source: None,
        );

        match config_error {
            NestGateError::configuration(
                message, context, ..
            ) => {
                assert_eq!(message, "Test configuration error");
                assert_eq!(context, "test_context");
    Ok(())
            }
            _ => return Err(std::io::Error::new(
    std::io::ErrorKind::Other,
    "Expected Configuration error variant".to_string()
).into()),
    Ok(())
        }
    Ok(())
    }

    #[test]
    fn test_nestgate_error_validation() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let validation_error = NestGateError::validation(
            currentvalue: Some("invalid".to_string()));

        match validation_error {
            NestGateError::validation(
                field,
                message,
                user_error,
                ..
            ) => {
                assert_eq!(field, "test_field");
                assert_eq!(message, "Invalid value");
                assert!(user_error);
    Ok(())
            }
            _ => return Err(std::io::Error::new(
    std::io::ErrorKind::Other,
    "Expected Validation error variant".to_string()
).into()),
    Ok(())
        }
    Ok(())
    }

    #[test]
    fn test_error_display() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let error = NestGateError::internal_error(

        let error_string = format!("{error}");
        assert!(error_string.contains("Test internal error"));
    Ok(())
    }

    #[test]
    fn test_basic_string_operations() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let test_string = "nestgate".to_string();
        assert_eq!(test_string.len(), 8);
        assert!(test_string.contains("gate"));
        assert!(test_string.starts_with("nest"));
    Ok(())
    }

    #[test]
    fn test_basic_collections() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let mut map = HashMap::new();
        map.insert("key1".to_string(), "value1".to_string());
        map.insert("key2".to_string(), "value2".to_string());

        assert_eq!(map.len(), 2);
        assert_eq!(map.get("key1"), Some(&"value1"));
        assert_eq!(map.get("key2"), Some(&"value2"));
        assert_eq!(map.get("key3"), None);
    Ok(())
    }
    Ok(())
}
