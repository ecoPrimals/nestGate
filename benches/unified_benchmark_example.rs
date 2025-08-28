//! Unified Benchmark Configuration Example
//!
//! This benchmark demonstrates the use of the unified benchmark configuration system
//! that consolidates all benchmark testing configurations.

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::time::Duration;

// Use the canonical configuration system
use nestgate_core::config::unified::NestGateUnifiedConfig;

/// Demonstrate comprehensive benchmark configuration
fn bench_unified_config_creation(c: &mut Criterion) {
    c.bench_function("unified_config_creation", |b| {
        b.iter(|| {
            let config = CanonicalConfig::default();

            // Verify configuration is properly initialized using actual fields
            assert!(!config.system.service_name.is_empty());
            assert!(!config.system.version.is_empty());

            // Test performance settings using actual available fields
            assert!(config.performance.enable_caching);
            assert!(config.performance.concurrent_operations > 0);

            black_box(config);
        })
    });
}

/// Demonstrate performance benchmark configuration
fn bench_performance_config(c: &mut Criterion) {
    c.bench_function("performance_config", |b| {
        b.iter(|| {
            let mut config = CanonicalConfig::default();
            config.system.service_name = "performance-benchmark-test".to_string();

            // Verify performance-specific settings
            assert_eq!(config.system.service_name, "performance-benchmark-test");
            assert_eq!(config.performance.concurrent_operations, 100);
            assert!(config.performance.enable_caching);

            black_box(config);
        })
    });
}

/// Demonstrate zero-copy benchmark configuration
fn bench_zero_copy_config(c: &mut Criterion) {
    c.bench_function("zero_copy_config", |b| {
        b.iter(|| {
            let mut config = CanonicalConfig::default();
            config.system.service_name = "zero-copy-benchmark-test".to_string();

            // Verify zero-copy specific settings
            assert_eq!(config.system.service_name, "zero-copy-benchmark-test");
            // Using available fields - zero_copy is in performance domain
            assert!(config.performance.enable_caching);

            black_box(config);
        })
    });
}

/// Demonstrate stress testing configuration
fn bench_stress_config(c: &mut Criterion) {
    c.bench_function("stress_config", |b| {
        b.iter(|| {
            let mut config = CanonicalConfig::default();
            config.system.service_name = "stress-benchmark-test".to_string();

            // Verify stress testing specific settings
            assert_eq!(config.system.service_name, "stress-benchmark-test");
            // Using available fields from actual domain configs
            assert!(config.storage.compression_enabled);
            assert!(config.monitoring.metrics_enabled);

            black_box(config);
        })
    });
}

/// Demonstrate builder pattern usage
fn bench_builder_pattern(c: &mut Criterion) {
    c.bench_function("builder_pattern", |b| {
        b.iter(|| {
            let config = UnifiedBenchmarkConfig::builder()
                .test_name("custom-benchmark-test")
                .concurrent_threads(200)
                .target_ops_per_second(50000)
                .enable_memory_stress(true)
                .enable_cpu_stress(true)
                .enable_zero_copy_testing(true)
                .enable_profiling(true)
                .build();

            // Verify builder configuration
            assert_eq!(config.service.name, "custom-benchmark-test");
            assert_eq!(config.extensions.performance.concurrent_operations, 200);
            // Using available fields from domain configs
            assert!(config.extensions.storage.cache_size_mb > 0);
            assert!(config.extensions.monitoring.metrics_enabled);
            assert!(config.extensions.performance.enable_caching);

            black_box(config);
        })
    });
}

/// Demonstrate mock service configuration
fn bench_mock_service_config(c: &mut Criterion) {
    c.bench_function("mock_service_config", |b| {
        b.iter(|| {
            let mock_service = BenchmarkMockService::default();
            let mock_config = BenchmarkMockConfiguration::default();

            // Verify mock service configuration
            assert!(!mock_service.id.is_empty());
            assert_eq!(mock_service.name, "mock-benchmark-service");
            assert_eq!(mock_service.version, "1.0.0");
            assert!(!mock_service.endpoints.is_empty());

            // Verify mock configuration
            assert_eq!(mock_config.service_name, "nestgate-benchmark");
            assert_eq!(mock_config.port, 8080);
            assert_eq!(mock_config.environment, "benchmark");
            assert!(!mock_config.debug_mode);

            // Compute hash to use all fields
            let service_hash = mock_service.id.len() as u64
                + mock_service.name.len() as u64 * 2
                + mock_service.version.len() as u64 * 3
                + mock_service.endpoints.len() as u64 * 4
                + mock_service.metadata.len() as u64 * 5;

            let config_hash = mock_config.service_name.len() as u64
                + mock_config.database_url.len() as u64 * 2
                + mock_config.port as u64
                + if mock_config.debug_mode { 1000 } else { 0 }
                + mock_config.features.len() as u64 * 10
                + mock_config.environment.len() as u64 * 3;

            black_box((service_hash, config_hash));
        })
    });
}

/// Demonstrate comprehensive benchmark extensions
fn bench_extensions_validation(c: &mut Criterion) {
    c.bench_function("extensions_validation", |b| {
        b.iter(|| {
            let extensions = BenchmarkExtensions::default();

            // Validate all extension settings
            assert_eq!(extensions.performance.test_duration_seconds, 60);
            assert_eq!(extensions.performance.concurrent_threads, 50);
            assert_eq!(extensions.performance.target_ops_per_second, 1000);

            assert!(extensions.memory.memory_stress_enabled);
            assert_eq!(extensions.memory.initial_allocation_mb, 100);
            assert_eq!(extensions.memory.max_memory_mb, 2048);

            assert!(extensions.cpu.cpu_stress_enabled);
            assert_eq!(extensions.cpu.target_cpu_percent, 80.0);

            assert!(extensions.io.io_stress_enabled);
            assert_eq!(extensions.io.buffer_size, 8192);
            assert_eq!(extensions.io.target_iops, 10000);

            assert_eq!(extensions.network.target_bandwidth_mbps, 1000.0);
            assert_eq!(extensions.network.target_latency_ms, 10.0);

            assert_eq!(extensions.mocking.response_delay_ms, 10);
            assert_eq!(extensions.mocking.failure_rate, 0.01);

            assert!(extensions.zero_copy.enabled);
            assert!(extensions.zero_copy.arc_comparison_enabled);

            assert!(extensions.stress.enabled);
            assert_eq!(extensions.stress.duration, Duration::from_secs(300));

            black_box(extensions);
        })
    });
}

criterion_group!(
    unified_benchmark_tests,
    bench_unified_config_creation,
    bench_performance_config,
    bench_zero_copy_config,
    bench_stress_config,
    bench_builder_pattern,
    bench_mock_service_config,
    bench_extensions_validation
);
criterion_main!(unified_benchmark_tests);
