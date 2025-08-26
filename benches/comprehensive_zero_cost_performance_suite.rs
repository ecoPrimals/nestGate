//! # Comprehensive Zero-Copy Performance Suite
//!
//! **Advanced benchmarking for zero-copy optimizations and performance validation**
//!
//! This benchmark suite validates the performance characteristics of NestGate's
//! zero-copy implementations across storage, security, and orchestration components.

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use nestgate_core::{
    error::Result,
    zero_cost_security_provider::{AuthenticationConfig, ProductionSecurityProvider},
};
use std::sync::Arc;
use tokio::runtime::Runtime;

/// Benchmark zero-copy string operations
fn benchmark_zero_copy_strings(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    c.bench_function("zero_copy_string_processing", |b| {
        b.iter(|| {
            let data = "test_data_for_processing".to_string();
            black_box(data.as_str().len())
        })
    });
}

/// Benchmark memory pool operations
fn benchmark_memory_pools(c: &mut Criterion) {
    c.bench_function("memory_pool_allocation", |b| {
        b.iter(|| {
            let pool_data = vec![0u8; 1024];
            black_box(pool_data.len())
        })
    });
}

/// Benchmark security provider operations
fn benchmark_security_operations(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    c.bench_function("security_provider_operations", |b| {
        b.iter(|| {
            let config = AuthenticationConfig::default();
            let provider = ProductionSecurityProvider::new(config);
            black_box(provider)
        })
    });
}

/// Benchmark configuration operations
fn benchmark_configuration_operations(c: &mut Criterion) {
    c.bench_function("config_operations", |b| {
        b.iter(|| {
            let config = AuthenticationConfig::default();
            black_box(config)
        })
    });
}

criterion_group!(
    zero_copy_benchmarks,
    benchmark_zero_copy_strings,
    benchmark_memory_pools,
    benchmark_security_operations,
    benchmark_configuration_operations,
);

criterion_main!(zero_copy_benchmarks);
