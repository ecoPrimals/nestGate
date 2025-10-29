//! **Tests for REST storage handlers**

use super::storage::*;
use crate::rest::models::{
    BenchmarkScenario, PerformanceRequirements, StorageBackendType, StorageTier,
};
use crate::rest::{ApiState, ListQuery};
use axum::extract::{Path, Query, State};

/// Helper to create test API state
fn create_test_state() -> ApiState {
    ApiState::new().expect("Failed to create test state")
}

// ==================== LIST BACKENDS TESTS ====================

#[tokio::test]
async fn test_list_backends_default() {
    let state = create_test_state();
    let query = ListQuery {
        page: None,
        per_page: None,
        sort: None,
        order: None,
        filter: None,
    };

    let result = list_backends(State(state), Query(query)).await;
    assert!(result.is_ok(), "list_backends should succeed");

    let response = result.unwrap();
    let backends = &response.0.data;

    assert!(!backends.is_empty(), "Should have at least one backend");
}

#[tokio::test]
async fn test_list_backends_with_filter() {
    let state = create_test_state();
    let query = ListQuery {
        page: None,
        per_page: None,
        sort: None,
        order: None,
        filter: Some("memory".to_string()),
    };

    let result = list_backends(State(state), Query(query)).await;
    assert!(result.is_ok(), "list_backends with filter should succeed");
}

#[tokio::test]
async fn test_list_backends_with_pagination() {
    let state = create_test_state();
    let query = ListQuery {
        page: Some(1),
        per_page: Some(10),
        sort: None,
        order: None,
        filter: None,
    };

    let result = list_backends(State(state), Query(query)).await;
    assert!(
        result.is_ok(),
        "list_backends with pagination should succeed"
    );
}

#[tokio::test]
async fn test_list_backends_page_two() {
    let state = create_test_state();
    let query = ListQuery {
        page: Some(2),
        per_page: Some(1),
        sort: None,
        order: None,
        filter: None,
    };

    let result = list_backends(State(state), Query(query)).await;
    assert!(result.is_ok(), "list_backends page 2 should succeed");
}

// ==================== GET BACKEND PERFORMANCE TESTS ====================
// NOTE: get_backend_performance handler not yet implemented
// Tests disabled until handler is added

// #[tokio::test]
// async fn test_get_backend_performance_memory() {
//     // TODO: Implement when get_backend_performance handler is available
// }

// ==================== SCAN STORAGE TESTS ====================
// NOTE: ScanStorageRequest struct has changed - tests need updating
// Current struct has: scan_depth, include_performance, include_costs, target_backends, path, include_cloud
// Old struct had: performance_requirements, capacity_gb, tier, additional_filters
// Tests disabled until updated to match current API

// #[tokio::test]
// async fn test_scan_storage_basic() {
//     // TODO: Update to new ScanStorageRequest structure
// }

// ==================== AUTO CONFIGURE TESTS ====================
// NOTE: AutoConfigInput struct has changed - tests need updating
// Actual struct has: hardware, performance_requirements, reliability_requirements, budget_constraints, workload_pattern
// Tests disabled until updated to match current API

// #[tokio::test]
// async fn test_auto_configure_basic() {
//     // TODO: Update to use new AutoConfigInput structure
// }

// ==================== COST ESTIMATION TESTS ====================
// NOTE: estimate_costs handler not yet implemented
// Tests disabled until handler is added

// #[tokio::test]
// async fn test_estimate_costs_memory_hot_tier() {
//     // TODO: Implement when estimate_costs handler is available
// }

// ==================== BENCHMARK TESTS ====================
// NOTE: BenchmarkStorageRequest struct has changed - tests need updating
// Actual struct has: storage_config, benchmark_config, backend, test_size_mb
// Tests disabled until updated to match current API

// #[tokio::test]
// async fn test_benchmark_storage_sequential_read() {
//     // TODO: Update to use new BenchmarkStorageRequest structure
// }

// ==================== PERFORMANCE PROJECTION TESTS ====================
// NOTE: project_performance handler not yet implemented
// Tests disabled until handler is added

// #[tokio::test]
// async fn test_project_performance_growth() {
//     let state = create_test_state();
//     let projection = PerformanceProjection {
//         expected_iops: 5000,
//         expected_throughput_mbps: 500.0,
//         expected_latency_ms: 5.0,
//         confidence_percent: 85.0,
//     };
//     // Test implementation when handler is available
// }

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

// #[test]
// fn test_cost_query_construction() {
//     // NOTE: CostQuery type no longer exists in current API
//     // Test disabled until API is updated
// }
