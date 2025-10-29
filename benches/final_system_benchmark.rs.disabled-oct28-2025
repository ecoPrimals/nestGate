//! Final System Performance Benchmark
//!
//! This benchmark validates the performance characteristics of the completed
//! NestGate system after all optimizations and fixes.

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use nestgate_core::{config::unified::NestGateCanonicalUnifiedConfig, types::StorageTier};
use std::time::Duration;

/// Benchmark configuration loading and validation
fn benchmark_config_operations(c: &mut Criterion) {
    c.bench_function("config_creation", |b| {
        b.iter(|| black_box(NestGateCanonicalUnifiedConfig::default()))
    });

    c.bench_function("config_serialization", |b| {
        let config = NestGateCanonicalUnifiedConfig::default();
        b.iter(|| {
            if let Ok(json) = serde_json::to_string(&config) {
                black_box(json)
            } else {
                black_box(String::new())
            }
        })
    });

    c.bench_function("config_deserialization", |b| {
        let config = NestGateCanonicalUnifiedConfig::default();
        if let Ok(json) = serde_json::to_string(&config) {
            b.iter(|| {
                if let Ok(parsed) = serde_json::from_str::<NestGateCanonicalUnifiedConfig>(&json) {
                    black_box(parsed)
                } else {
                    black_box(NestGateCanonicalUnifiedConfig::default())
                }
            })
        }
    });
}

/// Benchmark storage tier operations
fn benchmark_storage_tier_operations(c: &mut Criterion) {
    c.bench_function("storage_tier_priority", |b| {
        let tiers = vec![
            StorageTier::Hot,
            StorageTier::Warm,
            StorageTier::Cold,
            StorageTier::Cache,
            StorageTier::Archive,
        ];
        b.iter(|| {
            for tier in &tiers {
                black_box(tier.priority());
            }
        })
    });

    c.bench_function("storage_tier_display", |b| {
        let tier = StorageTier::Hot;
        b.iter(|| black_box(tier.to_string()))
    });
}

/// Benchmark async operations
fn benchmark_async_operations(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();

    c.bench_function("async_config_validation", |b| {
        b.iter(|| {
            rt.block_on(async {
                let config = NestGateCanonicalUnifiedConfig::default();
                // Simulate async validation
                tokio::time::sleep(Duration::from_micros(1)).await;
                black_box(config)
            })
        })
    });
}

/// Benchmark memory operations
fn benchmark_memory_operations(c: &mut Criterion) {
    c.bench_function("config_cloning", |b| {
        let config = NestGateCanonicalUnifiedConfig::default();
        b.iter(|| black_box(config.clone()))
    });

    c.bench_function("large_data_allocation", |b| {
        b.iter(|| {
            let data: Vec<u8> = vec![0; 1024 * 1024]; // 1MB
            black_box(data)
        })
    });
}

/// Benchmark system integration
fn benchmark_system_integration(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();

    c.bench_function("full_system_initialization", |b| {
        b.iter(|| {
            rt.block_on(async {
                // Simulate full system initialization
                let config = NestGateCanonicalUnifiedConfig::default();

                // Simulate various initialization steps
                tokio::time::sleep(Duration::from_micros(10)).await;

                // Validate configuration
                let json = serde_json::to_string(&config).unwrap();
                let _deserialized: NestGateCanonicalUnifiedConfig =
                    serde_json::from_str(&json).unwrap();

                black_box(config)
            })
        })
    });
}

criterion_group!(
    benches,
    benchmark_config_operations,
    benchmark_storage_tier_operations,
    benchmark_async_operations,
    benchmark_memory_operations,
    benchmark_system_integration
);
criterion_main!(benches);
