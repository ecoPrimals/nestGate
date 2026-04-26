// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! **Tests for REST storage handlers**

use crate::rest::models::{
    BenchmarkScenario, PerformanceRequirements, StorageBackendType, StorageTier,
};
use crate::rest::{ApiState, ListQuery};

/// Helper to create test API state
fn create_test_state() -> ApiState {
    ApiState::new().expect("Failed to create test state")
}

// ==================== GET BACKEND PERFORMANCE TESTS ====================
// NOTE: get_backend_performance handler not yet implemented
// Tests disabled until handler is added

// ==================== SCAN STORAGE TESTS ====================
// NOTE: ScanStorageRequest struct has changed - tests need updating
// Current struct has: scan_depth, include_performance, include_costs, target_backends, path, include_cloud
// Old struct had: performance_requirements, capacity_gb, tier, additional_filters
// Tests disabled until updated to match current API

// ==================== AUTO CONFIGURE TESTS ====================
// NOTE: AutoConfigInput struct has changed - tests need updating
// Actual struct has: hardware, performance_requirements, reliability_requirements, budget_constraints, workload_pattern
// Tests disabled until updated to match current API

// ==================== COST ESTIMATION TESTS ====================
// NOTE: estimate_costs handler not yet implemented
// Tests disabled until handler is added

// ==================== BENCHMARK TESTS ====================
// NOTE: BenchmarkStorageRequest struct has changed - tests need updating
// Actual struct has: storage_config, benchmark_config, backend, test_size_mb
// Tests disabled until updated to match current API

// ==================== TYPE VALIDATION TESTS ====================

#[test]
fn test_backend_type_variants() {
    // Ensure all backend types can be created
    let _memory = StorageBackendType::Memory;
    let _filesystem = StorageBackendType::Filesystem;
    let _local = StorageBackendType::Local;
    let _remote = StorageBackendType::Remote;
    let _cloud = StorageBackendType::Cloud;
}

#[test]
fn test_storage_tier_variants() {
    // Ensure all storage tiers can be created
    let _hot = StorageTier::Hot;
    let _warm = StorageTier::Warm;
    let _cold = StorageTier::Cold;
}

#[test]
fn test_benchmark_scenario_variants() {
    // Ensure all benchmark scenarios can be created
    let _seq_read = BenchmarkScenario::SequentialRead;
    let _seq_write = BenchmarkScenario::SequentialWrite;
    let _rand_read = BenchmarkScenario::RandomRead;
    let _rand_write = BenchmarkScenario::RandomWrite;
    let _mixed = BenchmarkScenario::Mixed;
}

#[tokio::test]
async fn test_api_state_creation() {
    let result = ApiState::new();
    assert!(result.is_ok(), "ApiState creation should succeed");
}

#[test]
fn test_performance_requirements_validation() {
    let perf_reqs = PerformanceRequirements {
        min_iops: 1000,
        min_throughput_mbps: 100.0,
        max_latency_ms: 10.0,
        availability_percent: 99.9,
    };

    assert!(perf_reqs.min_iops > 0);
    assert!(perf_reqs.min_throughput_mbps > 0.0);
    assert!(perf_reqs.max_latency_ms > 0.0);
    assert!(perf_reqs.availability_percent > 0.0);
}

#[test]
fn test_list_query_construction() {
    let query = ListQuery {
        page: Some(1),
        per_page: Some(10),
        sort: Some("name".to_string()),
        order: Some("asc".to_string()),
        filter: Some("memory".to_string()),
    };

    assert_eq!(query.page, Some(1));
    assert_eq!(query.per_page, Some(10));
    assert!(query.filter.is_some());
}
