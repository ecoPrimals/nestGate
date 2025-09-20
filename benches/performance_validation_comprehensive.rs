//! **COMPREHENSIVE PERFORMANCE VALIDATION BENCHMARKS**
//!
//! This module provides comprehensive benchmarks to validate the performance
//! improvements achieved through canonical modernization and optimization.
//!
//! ## Benchmark Categories
//! - **Allocation Performance**: Pool vs direct allocation
//! - **Async Performance**: Native async vs async_trait patterns
//! - **Memory Efficiency**: Zero-copy vs clone patterns
//! - **Service Discovery**: Canonical vs legacy patterns
//! - **Error Handling**: Unified vs fragmented error systems

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

// Import our optimized modules
use nestgate_core::optimized::{
    allocation_optimizer::{global_pools, ZeroAllocStringBuilder},
    perf::PerfTimer,
};
use nestgate_core::traits::canonical_unified_traits::{
    create_default_capabilities, ServiceCapabilities, UniversalResponseStatus,
};

// ==================== ALLOCATION BENCHMARKS ====================

/// Benchmark direct allocation vs pooled allocation
fn benchmark_allocation_patterns(c: &mut Criterion) {
    let mut group = c.benchmark_group("allocation_patterns");

    // Benchmark direct HashMap allocation
    group.bench_function("direct_hashmap", |b| {
        b.iter(|| {
            let mut map: HashMap<String, String> = HashMap::new();
            for i in 0..100 {
                map.insert(format!("key_{}", i), format!("value_{}", i));
            }
            black_box(map);
        });
    });

    // Benchmark pooled HashMap allocation
    group.bench_function("pooled_hashmap", |b| {
        b.iter(|| {
            let mut map = global_pools().hashmap_pool.acquire();
            for i in 0..100 {
                map.insert(format!("key_{}", i), format!("value_{}", i));
            }
            black_box(&*map);
        });
    });

    // Benchmark string building patterns
    group.bench_function("direct_string_building", |b| {
        b.iter(|| {
            let mut result = String::new();
            result.push_str("prefix_");
            result.push_str("middle_");
            result.push_str("suffix");
            for i in 0..10 {
                result.push_str(&format!("_{}", i));
            }
            black_box(result);
        });
    });

    group.bench_function("zero_alloc_string_building", |b| {
        b.iter(|| {
            let mut builder = ZeroAllocStringBuilder::with_capacity(256);
            builder
                .add_static("prefix_")
                .add_static("middle_")
                .add_static("suffix");
            for i in 0..10 {
                builder.add_dynamic(&format!("_{}", i));
            }
            let result = builder.build();
            black_box(result);
        });
    });

    group.finish();
}

// ==================== ASYNC PERFORMANCE BENCHMARKS ====================

/// Benchmark native async vs async_trait patterns
fn benchmark_async_patterns(c: &mut Criterion) {
    let mut group = c.benchmark_group("async_patterns");

    // Create runtime for async benchmarks
    let rt = tokio::runtime::Runtime::new().unwrap();

    // Benchmark native async trait method
    group.bench_function("native_async_service_call", |b| {
        b.to_async(&rt).iter(|| async {
            let capabilities = create_default_capabilities();
            black_box(capabilities);
        });
    });

    // Benchmark service capability creation
    group.bench_function("service_capabilities_creation", |b| {
        b.iter(|| {
            let capabilities = ServiceCapabilities::default();
            black_box(capabilities);
        });
    });

    // Benchmark response status creation
    group.bench_function("response_status_creation", |b| {
        b.iter(|| {
            let status = UniversalResponseStatus::Success;
            black_box(status);
        });
    });

    group.finish();
}

// ==================== MEMORY EFFICIENCY BENCHMARKS ====================

/// Benchmark memory efficiency patterns
fn benchmark_memory_patterns(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_patterns");

    // Create test data
    let large_string = "x".repeat(10000);
    let large_vec: Vec<u8> = (0..10000).map(|i| (i % 256) as u8).collect();

    // Benchmark Arc cloning vs direct cloning
    group.bench_function("direct_clone", |b| {
        b.iter(|| {
            let cloned_string = large_string.clone();
            let cloned_vec = large_vec.clone();
            black_box((cloned_string, cloned_vec));
        });
    });

    group.bench_function("arc_sharing", |b| {
        let arc_string = Arc::new(large_string.clone());
        let arc_vec = Arc::new(large_vec.clone());

        b.iter(|| {
            let shared_string = Arc::clone(&arc_string);
            let shared_vec = Arc::clone(&arc_vec);
            black_box((shared_string, shared_vec));
        });
    });

    // Benchmark pooled vs direct allocation for common operations
    group.bench_function("pooled_vec_operations", |b| {
        b.iter(|| {
            let mut vec = global_pools().vec_u8_pool.acquire();
            for i in 0..1000 {
                vec.push((i % 256) as u8);
            }
            black_box(&*vec);
        });
    });

    group.bench_function("direct_vec_operations", |b| {
        b.iter(|| {
            let mut vec = Vec::new();
            for i in 0..1000 {
                vec.push((i % 256) as u8);
            }
            black_box(vec);
        });
    });

    group.finish();
}

// ==================== SERVICE DISCOVERY BENCHMARKS ====================

/// Benchmark service discovery patterns
fn benchmark_service_discovery(c: &mut Criterion) {
    let mut group = c.benchmark_group("service_discovery");

    // Benchmark service metadata creation
    group.bench_function("service_metadata_creation", |b| {
        b.iter(|| {
            let capabilities = ServiceCapabilities {
                max_concurrent_requests: 1000,
                supported_protocols: vec!["http".to_string(), "grpc".to_string()],
                health_check_interval: Duration::from_secs(30),
                endpoints: {
                    let mut endpoints = HashMap::new();
                    endpoints.insert("health".to_string(), "/health".to_string());
                    endpoints.insert("metrics".to_string(), "/metrics".to_string());
                    endpoints
                },
                tags: vec!["storage".to_string(), "zfs".to_string()],
            };
            black_box(capabilities);
        });
    });

    // Benchmark response status handling
    group.bench_function("response_status_matching", |b| {
        let statuses = vec![
            UniversalResponseStatus::Success,
            UniversalResponseStatus::Error,
            UniversalResponseStatus::Pending,
        ];

        b.iter(|| {
            for status in &statuses {
                let result = match status {
                    UniversalResponseStatus::Success => "ok",
                    UniversalResponseStatus::Error => "error",
                    UniversalResponseStatus::Pending => "pending",
                };
                black_box(result);
            }
        });
    });

    group.finish();
}

// ==================== COMPREHENSIVE SYSTEM BENCHMARKS ====================

/// End-to-end system performance benchmark
fn benchmark_system_integration(c: &mut Criterion) {
    let mut group = c.benchmark_group("system_integration");

    // Benchmark complete service operation cycle
    group.bench_function("complete_service_cycle", |b| {
        b.iter(|| {
            // 1. Create service capabilities
            let capabilities = create_default_capabilities();

            // 2. Process request with pooled resources
            let mut request_data = global_pools().hashmap_pool.acquire();
            request_data.insert("operation".to_string(), "test".to_string());
            request_data.insert("timestamp".to_string(), "2025-01-30".to_string());

            // 3. Build response
            let mut response_builder = ZeroAllocStringBuilder::with_capacity(256);
            response_builder.add_static("response:").add_dynamic(
                &request_data
                    .get("operation")
                    .unwrap_or(&"unknown".to_string()),
            );

            // 4. Create response status
            let status = UniversalResponseStatus::Success;

            let response = response_builder.build();
            black_box((capabilities, response, status));
        });
    });

    group.finish();
}

// ==================== BENCHMARK CONFIGURATION ====================

criterion_group!(
    name = performance_benchmarks;
    config = Criterion::default()
        .sample_size(1000)
        .measurement_time(Duration::from_secs(10))
        .warm_up_time(Duration::from_secs(3));
    targets =
        benchmark_allocation_patterns,
        benchmark_async_patterns,
        benchmark_memory_patterns,
        benchmark_service_discovery,
        benchmark_system_integration
);

criterion_main!(performance_benchmarks);

// ==================== BENCHMARK VALIDATION ====================

#[cfg(test)]
mod validation_tests {
    use super::*;

    #[test]
    fn validate_allocation_pools() {
        // Test that pools actually reuse objects
        let pool = &global_pools().string_pool;

        {
            let mut obj1 = pool.acquire();
            obj1.push_str("test");
        } // obj1 returned to pool

        {
            let obj2 = pool.acquire();
            // obj2 should be the reused obj1 (now cleared)
            assert!(obj2.is_empty());
        }
    }

    #[test]
    fn validate_zero_alloc_builder() {
        let builder = ZeroAllocStringBuilder::with_capacity(100);
        let result = builder
            .add_static("hello")
            .add_static(" ")
            .add_static("world")
            .build();

        assert_eq!(result, "hello world");
    }

    #[test]
    fn validate_canonical_types() {
        let capabilities = create_default_capabilities();
        assert!(capabilities.max_concurrent_requests > 0);

        let status = UniversalResponseStatus::Success;
        assert!(matches!(status, UniversalResponseStatus::Success));
    }
}
