// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! **CORE FUNCTIONALITY INTEGRATION TESTS**
//!
//! Tests that validate the actual NestGate core functionality

use crate::common::*;
use nestgate_core::{
    canonical_types::StorageTier,
    constants::{hardcoding::runtime_fallback_ports, network, security, storage},
    error::NestGateError,
    types::{AllocationStatus, HealthStatus, StorageTier, SystemInfo},
    canonical_modernization::unified_enums::{UnifiedAlertSeverity, UnifiedHealthStatus, UnifiedServiceState},
};
use std::time::Duration;

/// Test core constants are accessible and have expected values
#[test]
async fn test_core_constants_integration() -> Result<(), Box<dyn std::error::Error>> {
    // Test network constants (runtime fallbacks; API listen port is `HTTP`, internal API lane is `API`)
    assert_eq!(runtime_fallback_ports::HTTP, 8080);
    assert_eq!(runtime_fallback_ports::HEALTH, 8081);
    assert_eq!(network::LOCALHOST, "127.0.0.1");

    // Test storage constants
    assert_eq!(storage::sizes::KB, 1024);
    assert_eq!(storage::sizes::MB, 1024 * 1024);
    assert_eq!(storage::zfs::DEFAULT_POOL_NAME, "zfspool");

    // Test security constants
    assert_eq!(security::crypto::KEY_SIZE_BITS, 256);
    assert_eq!(security::auth::MAX_LOGIN_ATTEMPTS, 5);
    Ok(())
}

/// Test unified enum system functionality
#[test]
async fn test_unified_enums_integration() -> Result<(), Box<dyn std::error::Error>> {
    // Test health status enum
    let healthy = UnifiedHealthStatus::Healthy;
    let degraded = UnifiedHealthStatus::Degraded;
    assert_ne!(healthy, degraded);

    // Test service state enum
    let running = UnifiedServiceState::Running;
    let stopped = UnifiedServiceState::Stopped;
    assert_ne!(running, stopped);

    // Test alert severity enum
    let info = UnifiedAlertSeverity::Info;
    let critical = UnifiedAlertSeverity::Critical;
    assert_ne!(info, critical);
    Ok(())
}

/// Test core types functionality
#[test]
async fn test_core_types_integration() -> Result<(), Box<dyn std::error::Error>> {
    // Test storage tier
    let hot_tier = StorageTier::Hot;
    let cold_tier = StorageTier::Cold;
    assert_ne!(hot_tier, cold_tier);

    // Test allocation status
    let active = AllocationStatus::Active;
    let failed = AllocationStatus::Failed;
    assert_ne!(active, failed);

    // Test health status with defaults
    let health = HealthStatus::default();
    assert!(health.overall_healthy);
    assert_eq!(health.cpu_usage_percent, 0.0);

    // Test system info creation
    let system_info = SystemInfo {
        hostname: "test-system".to_string(),
        os_type: "Linux".to_string(),
        os_version: "5.15.0".to_string(),
        architecture: "x86_64".to_string(),
        total_memory: 8 * 1024 * 1024 * 1024,     // 8GB
        available_memory: 4 * 1024 * 1024 * 1024, // 4GB
        cpu_cores: 8,
        uptime_seconds: 86400, // 1 day
    };

    assert_eq!(system_info.hostname, "test-system");
    assert_eq!(system_info.cpu_cores, 8);
    assert_eq!(system_info.total_memory, 8 * 1024 * 1024 * 1024);
    Ok(())
}

/// Test error handling integration
#[test]
async fn test_error_handling_integration() -> Result<(), Box<dyn std::error::Error>> {
    // Test creating different error types
    let config_error = NestGateError::configuration_error("test_field", "Invalid configuration".to_string());

    // Test error display
    let error_display = format!("{}", config_error);
    assert!(error_display.contains("Invalid configuration"));

    // Test error debug
    let error_debug = format!("{:?}", config_error);
    assert!(error_debug.contains("Configuration"));
    Ok(())
}

/// Test storage tier operations
#[test]
async fn test_storage_tier_operations() -> Result<(), Box<dyn std::error::Error>> {
    let hot = StorageTier::Hot;
    let warm = StorageTier::Warm;
    let cold = StorageTier::Cold;
    let cache = StorageTier::Cache;
    let archive = StorageTier::Archive;

    // Test priority ordering
    assert_eq!(cache.priority(), 0);
    assert_eq!(hot.priority(), 1);
    assert_eq!(warm.priority(), 2);
    assert_eq!(cold.priority(), 3);
    assert_eq!(archive.priority(), 4);

    // Test cache tier detection
    assert!(hot.is_cache_tier());
    assert!(cache.is_cache_tier());
    assert!(!cold.is_cache_tier());
    assert!(!archive.is_cache_tier());

    // Test string representation
    assert_eq!(hot.as_str(), "hot");
    assert_eq!(warm.as_str(), "warm");
    assert_eq!(cold.as_str(), "cold");

    // Test display
    assert_eq!(hot.to_string(), "Hot");
    assert_eq!(warm.to_string(), "Warm");
    assert_eq!(cold.to_string(), "Cold");
    Ok(())
}

/// Test all storage tiers are included in the complete list
#[test]
async fn test_storage_tier_completeness() -> Result<(), Box<dyn std::error::Error>> {
    let all_tiers = StorageTier::all();

    assert_eq!(all_tiers.len(), 5);
    assert!(all_tiers.contains(&StorageTier::Hot));
    assert!(all_tiers.contains(&StorageTier::Warm));
    assert!(all_tiers.contains(&StorageTier::Cold));
    assert!(all_tiers.contains(&StorageTier::Cache));
    assert!(all_tiers.contains(&StorageTier::Archive));
    Ok(())
}

/// Test health status comprehensive functionality
#[test]
async fn test_health_status_comprehensive() -> Result<(), Box<dyn std::error::Error>> {
    let mut health = HealthStatus::default();

    // Test default values
    assert!(health.overall_healthy);
    assert_eq!(health.cpu_usage_percent, 0.0);
    assert_eq!(health.memory_usage_percent, 0.0);
    assert_eq!(health.disk_usage_percent, 0.0);
    assert!(health.network_connected);
    assert_eq!(health.services_running, vec!["nestgate-core".to_string()]);

    // Test modification
    health.cpu_usage_percent = 75.5;
    health.memory_usage_percent = 60.2;
    health.overall_healthy = false;

    assert!(!health.overall_healthy);
    assert_eq!(health.cpu_usage_percent, 75.5);
    assert_eq!(health.memory_usage_percent, 60.2);
    Ok(())
}

/// Test system info comprehensive functionality
#[test]
async fn test_system_info_comprehensive() -> Result<(), Box<dyn std::error::Error>> {
    let system_info = SystemInfo {
        hostname: "production-server".to_string(),
        os_type: "Ubuntu".to_string(),
        os_version: "6.2.0".to_string(),
        architecture: "aarch64".to_string(),
        total_memory: 32 * 1024 * 1024 * 1024,     // 32GB
        available_memory: 16 * 1024 * 1024 * 1024, // 16GB
        cpu_cores: 16,
        uptime_seconds: 7 * 24 * 60 * 60, // 1 week
    };

    // Test all fields
    assert_eq!(system_info.hostname, "production-server");
    assert_eq!(system_info.os_type, "Ubuntu");
    assert_eq!(system_info.architecture, "aarch64");
    assert_eq!(system_info.os_version, "6.2.0");
    assert_eq!(system_info.total_memory, 32 * 1024 * 1024 * 1024);
    assert_eq!(system_info.available_memory, 16 * 1024 * 1024 * 1024);
    assert_eq!(system_info.cpu_cores, 16);
    assert_eq!(system_info.uptime_seconds, 7 * 24 * 60 * 60);

    // Test memory calculations
    let used_memory = system_info.total_memory - system_info.available_memory;
    assert_eq!(used_memory, 16 * 1024 * 1024 * 1024); // 16GB used

    let memory_usage_percent = (used_memory as f64 / system_info.total_memory as f64) * 100.0;
    assert_eq!(memory_usage_percent, 50.0); // 50% memory usage
    Ok(())
}

/// Async integration test for timeout functionality
#[tokio::test]
async fn test_async_integration_functionality() -> Result<(), Box<dyn std::error::Error>> {
    init_test_logging();

    let config = MockConfig::new();
    let storage = MockStorage::new();

    // Test async operations with timeout
    let timeout = config.get_timeout().await;
    assert!(timeout > Duration::from_millis(0));

    // Test storage operations
    let test_data = b"integration_test_data";
    storage.write("integration_key", test_data).await?;

    let retrieved_data = storage.read("integration_key").await?;
    assert_eq!(retrieved_data, test_data);

    // Test cleanup
    storage.delete("integration_key").await?;
    assert!(!storage.exists("integration_key").await);
    Ok(())
}

/// Test comprehensive storage tier operations
#[test]
fn test_comprehensive_storage_tier_operations() -> Result<(), Box<dyn std::error::Error>> {
    let hot_tier = StorageTier::Hot;
    let warm_tier = StorageTier::Warm;
    let cold_tier = StorageTier::Cold;
    let cache_tier = StorageTier::Cache;
    let archive_tier = StorageTier::Archive;

    // Test all tier priorities
    assert_eq!(hot_tier.priority(), 1);
    assert_eq!(warm_tier.priority(), 2);
    assert_eq!(cold_tier.priority(), 3);
    assert_eq!(cache_tier.priority(), 0);
    assert_eq!(archive_tier.priority(), 4);

    // Test tier operations
    assert_ne!(hot_tier, warm_tier);
    assert_ne!(warm_tier, cold_tier);
    assert_ne!(cold_tier, archive_tier);
    assert_ne!(cache_tier, hot_tier);

    // Test tier serialization
    let hot_json = serde_json::to_string(&hot_tier)?;
    let deserialized_hot: StorageTier = serde_json::from_str(&hot_json)?;
    assert_eq!(hot_tier, deserialized_hot);
    Ok(())
}

/// Test comprehensive allocation status workflows
#[test]
fn test_comprehensive_allocation_status_workflows() -> Result<(), Box<dyn std::error::Error>> {
    // Test allocation lifecycle
    let pending = AllocationStatus::Pending;
    let active = AllocationStatus::Active;
    let failed = AllocationStatus::Failed;
    let inactive = AllocationStatus::Inactive;

    // Test status transitions
    assert_ne!(pending, active);
    assert_ne!(active, failed);
    assert_ne!(pending, failed);
    assert_ne!(active, inactive);

    // Test status serialization
    let statuses = vec![pending, active, failed, inactive];
    for status in statuses {
        let json = serde_json::to_string(&status)?;
        let deserialized: AllocationStatus = serde_json::from_str(&json)?;
        assert_eq!(status, deserialized);
    Ok(())
}
}

/// Test comprehensive error handling scenarios
#[test]
fn test_comprehensive_error_handling_scenarios() -> Result<(), Box<dyn std::error::Error>> {
    // Test different error types
    let internal_error = NestGateError::internal_error("Internal server error".to_string(), "test_component");

    let validation_error = NestGateError::validation_error("validation error");
    let validation_display = format!("{}", validation_error);
    let config_display = format!("{}", config_error);

    assert!(internal_display.contains("Internal server error"));
    assert!(validation_display.contains("Invalid input data"));
    assert!(config_display.contains("Missing configuration"));

    // Test error debugging
    let internal_debug = format!("{:?}", internal_error);
    let validation_debug = format!("{:?}", validation_error);
    let config_debug = format!("{:?}", config_error);

    assert!(internal_debug.contains("Internal"));
    assert!(validation_debug.contains("Validation"));
    assert!(config_debug.contains("Configuration"));
    Ok(())
}

/// Test comprehensive unified enum operations
#[test]
fn test_comprehensive_unified_enum_operations() -> Result<(), Box<dyn std::error::Error>> {
    // Test UnifiedHealthStatus
    let healthy = UnifiedHealthStatus::Healthy;
    let degraded = UnifiedHealthStatus::Degraded;
    let critical = UnifiedHealthStatus::Critical;
    let unknown = UnifiedHealthStatus::Unknown;

    // Test all variants exist and are different
    assert_ne!(healthy, degraded);
    assert_ne!(degraded, critical);
    assert_ne!(critical, unknown);
    assert_ne!(healthy, unknown);

    // Test UnifiedServiceState
    let running = UnifiedServiceState::Running;
    let stopped = UnifiedServiceState::Stopped;
    let starting = UnifiedServiceState::Starting;
    let stopping = UnifiedServiceState::Stopping;
    let error_state = UnifiedServiceState::Error;

    // Test all service states
    let all_states = vec![running, stopped, starting, stopping, error_state];
    for (i, state1) in all_states.iter().enumerate() {
        for (j, state2) in all_states.iter().enumerate() {
            if i != j {
                assert_ne!(
                    state1, state2,
                    "States {:?} and {:?} should be different",
                    state1, state2
                );
    Ok(())
            }
    Ok(())
        }
    Ok(())
}
}