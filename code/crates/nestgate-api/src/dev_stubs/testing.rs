//! **TEST UTILITIES AND MOCK BUILDERS**
//!
//! ⚠️ **DEVELOPMENT/TEST ONLY**: This module is only available with `dev-stubs` feature
//!
//! Provides utilities for building mock data and configuring test doubles.
//!
//! # Modules
//!
//! - Mock builders for return types
//! - Test double configuration
//! - Resource allocation mocks
//! - Workload result mocks
//!
//! # Migration Note
//!
//! **Consolidated**: November 10, 2025
//! - From: `nestgate-core/src/return_builders/mock_builders.rs`
//! - From: `nestgate-core/src/config/canonical_primary/domains/test_canonical/mocking.rs`

#![cfg(feature = "dev-stubs")]

use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ==================== MOCK DATA STRUCTURES ====================

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

// ==================== MOCK BUILDERS ====================

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
        expires_at: (std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
            + 3600)
            .to_string(),
        metadata: serde_json::json!({
            "cpu_cores": cpu_cores,
            "memory_gb": memory_gb,
            "storage_gb": storage_gb,
            "network_mbps": network_mbps,
        }),
    }
}

/// Build mock workload result with configurable success and timing
/// **PURE FUNCTION**: Mock workload result construction
/// **TESTABLE**: Can verify all fields and nested structures
#[must_use]
pub fn build_mock_workload_result(
    workload_id: &str,
    success: bool,
    execution_time_ms: u64,
    cpu_usage: f64,
    memory_usage: f64,
) -> WorkloadResult {
    WorkloadResult {
        workload_id: workload_id.to_string(),
        success,
        execution_time_ms,
        performance_metrics: PerformanceMetrics {
            cpu_usage,
            memory_usage,
            disk_io: 0.0,
            network_io: 0.0,
        },
        resources_used: build_mock_resource_allocation(4, 8, 100, 1000),
        result_data: serde_json::json!({
            "status": if success { "completed" } else { "failed" },
            "timestamp": std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        }),
    }
}

// ==================== TEST MOCKING CONFIGURATION ====================

/// Test mocking configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MockingConfig {
    pub services: MockServiceConfig,
    pub doubles: TestDoubleConfig,
    pub stubs: StubConfig,
}

/// Mock service configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MockServiceConfig {
    pub enabled: bool,
}

/// Test double configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestDoubleConfig {
    pub enabled: bool,
}

/// Stub configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StubConfig {
    pub enabled: bool,
}

impl Default for MockServiceConfig {
    fn default() -> Self {
        Self { enabled: true }
    }
}

impl Default for TestDoubleConfig {
    fn default() -> Self {
        Self { enabled: true }
    }
}

impl Default for StubConfig {
    fn default() -> Self {
        Self { enabled: true }
    }
}

impl MockingConfig {
    /// Create a new mocking configuration with all features enabled
    #[must_use]
    pub const fn new() -> Self {
        Self {
            services: MockServiceConfig { enabled: true },
            doubles: TestDoubleConfig { enabled: true },
            stubs: StubConfig { enabled: true },
        }
    }

    /// Create a configuration with all mocking disabled
    #[must_use]
    pub const fn disabled() -> Self {
        Self {
            services: MockServiceConfig { enabled: false },
            doubles: TestDoubleConfig { enabled: false },
            stubs: StubConfig { enabled: false },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_mock_resource_allocation() {
        let allocation = build_mock_resource_allocation(8, 16, 500, 10000);

        assert_eq!(allocation.status, "active");
        assert_eq!(allocation.resource_type, "compute-8-16-500-10000");
        assert!(!allocation.id.is_empty());
    }

    #[test]
    fn test_build_mock_workload_result() {
        let result = build_mock_workload_result("test-workload", true, 1000, 0.5, 0.7);

        assert_eq!(result.workload_id, "test-workload");
        assert!(result.success);
        assert_eq!(result.execution_time_ms, 1000);
        assert_eq!(result.performance_metrics.cpu_usage, 0.5);
        assert_eq!(result.performance_metrics.memory_usage, 0.7);
    }

    #[test]
    fn test_mocking_config_defaults() {
        let config = MockingConfig::default();

        assert!(config.services.enabled);
        assert!(config.doubles.enabled);
        assert!(config.stubs.enabled);
    }

    #[test]
    fn test_mocking_config_disabled() {
        let config = MockingConfig::disabled();

        assert!(!config.services.enabled);
        assert!(!config.doubles.enabled);
        assert!(!config.stubs.enabled);
    }
}
