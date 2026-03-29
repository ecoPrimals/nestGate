// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

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
/// Performancemetrics
pub struct PerformanceMetrics {
    /// Cpu Usage
    pub cpu_usage: f64,
    /// Memory Usage
    pub memory_usage: f64,
    /// Disk Io
    pub disk_io: f64,
    /// Network Io
    pub network_io: f64,
}

/// Resource allocation structure for mock builders
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Resourceallocation
pub struct ResourceAllocation {
    /// Unique identifier
    pub id: String,
    /// Resource Type
    pub resource_type: String,
    /// Status
    pub status: String,
    /// Allocated At
    pub allocated_at: String,
    /// Expires At
    pub expires_at: String,
    /// Additional metadata key-value pairs
    pub metadata: serde_json::Value,
}

/// Workload result structure for mock builders
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Workloadresult
pub struct WorkloadResult {
    /// Performance Metrics
    pub performance_metrics: PerformanceMetrics,
    /// Workload identifier
    pub workload_id: String,
    /// Success
    pub success: bool,
    /// Execution Time Ms
    pub execution_time_ms: u64,
    /// Resources Used
    pub resources_used: ResourceAllocation,
    /// Result Data
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
/// Configuration for Mocking
pub struct MockingConfig {
    /// Services
    pub services: MockServiceConfig,
    /// Doubles
    pub doubles: TestDoubleConfig,
    /// Stubs
    pub stubs: StubConfig,
}

/// Mock service configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `MockService`
pub struct MockServiceConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
}

/// Test double configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `TestDouble`
pub struct TestDoubleConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
}

/// Stub configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Stub
pub struct StubConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
}

impl Default for MockServiceConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self { enabled: true }
    }
}

impl Default for TestDoubleConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self { enabled: true }
    }
}

impl Default for StubConfig {
    /// Returns the default instance
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
