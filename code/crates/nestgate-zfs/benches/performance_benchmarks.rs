/// Performance Benchmarks for NestGate ZFS
/// Benchmarks to validate performance characteristics and identify bottlenecks
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Duration;
use tokio::runtime::Runtime;

use nestgate_zfs::*;

// Temporary definitions for missing types
#[derive(Debug, Clone)]
pub enum OptimizationComplexity {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone)]
pub enum OptimizationType {
    TierMigration,
    Compression,
    Deduplication,
    Caching,
}

#[derive(Debug, Clone)]
pub struct OptimizationOpportunity {
    pub optimization_type: OptimizationType,
    pub description: String,
    pub expected_impact: f64,
    pub confidence: f64,
    pub complexity: OptimizationComplexity,
    pub implementation_time: Duration,
}

/// Benchmark configuration creation
fn bench_config_creation(c: &mut Criterion) {
    c.bench_function("config_creation", |b| {
        b.iter(|| black_box(ZfsUnifiedMcpConfig::default()))
    });
}

/// Benchmark configuration validation
fn bench_config_validation(c: &mut Criterion) {
    let config = ZfsUnifiedMcpConfig::default();

    c.bench_function("config_validation", |b| {
        b.iter(|| {
            black_box(config.validate()).unwrap_or_else(|e| {
                tracing::error!("Unwrap failed: {:?}", e);
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Operation failed: {:?}", e),
                )
                .into());
            })
        })
    });
}

/// Benchmark tier configuration access
fn bench_tier_config_access(c: &mut Criterion) {
    let config = ZfsUnifiedMcpConfig::default();
    let tiers = [StorageTier::Hot, StorageTier::Warm, StorageTier::Cold];

    c.bench_function("tier_config_access", |b| {
        b.iter(|| {
            for tier in &tiers {
                black_box(config.get_tier_config(tier));
            }
        })
    });
}

/// Benchmark performance metrics creation
fn bench_performance_metrics(c: &mut Criterion) {
    c.bench_function("performance_metrics_creation", |b| {
        b.iter(|| black_box(crate::performance::CurrentPerformanceMetrics::default()))
    });
}

/// Benchmark tier metrics generation
fn bench_tier_metrics_generation(c: &mut Criterion) {
    let tiers = [
        StorageTier::Hot,
        StorageTier::Warm,
        StorageTier::Cold,
        StorageTier::Cache,
    ];

    for tier in &tiers {
        c.bench_with_input(
            BenchmarkId::new("tier_metrics_generation", format!("{tier:?}")),
            tier,
            |b, &tier| {
                b.iter(|| {
                    black_box(crate::performance::TierMetrics::default_for_tier(
                        tier.into(),
                    ))
                })
            },
        );
    }
}

/// Benchmark AI optimization opportunity sorting
fn bench_ai_optimization_sorting(c: &mut Criterion) {
    let mut group = c.benchmark_group("ai_optimization");

    for size in [10, 100, 1000].iter() {
        group.bench_with_input(
            BenchmarkId::new("opportunity_sorting", size),
            size,
            |b, &size| {
                let opportunities = create_test_opportunities(size);
                b.iter(|| {
                    let mut ops = opportunities.clone();
                    ops.sort_by(|a, b| {
                        b.expected_impact
                            .partial_cmp(&a.expected_impact)
                            .unwrap_or_else(|e| {
                                tracing::error!("Unwrap failed: {:?}", e);
                                return Err(std::io::Error::new(
                                    std::io::ErrorKind::Other,
                                    format!("Operation failed: {:?}", e),
                                )
                                .into());
                            })
                    });
                    black_box(ops)
                })
            },
        );
    }
    group.finish();
}

/// Benchmark migration job creation
fn bench_migration_job_creation(c: &mut Criterion) {
    c.bench_function("migration_job_creation", |b| {
        b.iter(|| {
            black_box(crate::migration::MigrationJob::new(
                PathBuf::from("/test/file.txt"),
                StorageTier::Hot,
                StorageTier::Warm,
                crate::migration::MigrationPriority::Normal,
                1024 * 1024,
            ))
        })
    });
}

/// Benchmark snapshot policy validation
fn bench_snapshot_policy_validation(c: &mut Criterion) {
    let policy = crate::snapshot::SnapshotPolicy::default();

    c.bench_function("snapshot_policy_validation", |b| {
        b.iter(|| {
            // Simulate policy validation logic
            black_box(policy.enabled && !policy.name.is_empty() && policy.priority > 0)
        })
    });
}

/// Benchmark concurrent metrics collection
fn bench_concurrent_metrics(c: &mut Criterion) {
    let rt = Runtime::new().unwrap_or_else(|e| {
        tracing::error!("Unwrap failed: {:?}", e);
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Operation failed: {:?}", e),
        )
        .into());
    });

    c.bench_function("concurrent_metrics_collection", |b| {
        b.to_async(&rt).iter(|| async {
            let handles: Vec<_> = (0..10)
                .map(|_| {
                    tokio::spawn(async {
                        black_box(crate::performance::CurrentPerformanceMetrics::default())
                    })
                })
                .collect();

            for handle in handles {
                black_box(handle.await.unwrap_or_else(|e| {
                    tracing::error!("Unwrap failed: {:?}", e);
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("Operation failed: {:?}", e),
                    )
                    .into());
                }));
            }
        })
    });
}

/// Benchmark memory allocation patterns
fn bench_memory_allocation(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_allocation");

    group.bench_function("hashmap_creation", |b| {
        b.iter(|| {
            let mut map: HashMap<String, String> = HashMap::new();
            for i in 0..100 {
                map.insert(format!("key_{i}"), format!("value_{i}"));
            }
            black_box(map)
        })
    });

    group.bench_function("vec_creation", |b| {
        b.iter(|| {
            let mut vec = Vec::new();
            for i in 0..100 {
                vec.push(format!("item_{i}"));
            }
            black_box(vec)
        })
    });

    group.finish();
}

/// Benchmark error handling overhead
fn bench_error_handling(c: &mut Criterion) {
    use crate::error::*;

    c.bench_function("error_creation", |b| {
        b.iter(|| {
            black_box(ZfsError::PoolError(PoolError::NotFound {
                pool_name: "test".to_string(),
            }))
        })
    });

    c.bench_function("error_retryability_check", |b| {
        let errors = vec![
            ZfsError::Timeout("timeout".to_string()),
            ZfsError::SystemUnavailable("unavailable".to_string()),
            ZfsError::PoolError(PoolError::NotFound {
                pool_name: "test".to_string(),
            }),
        ];

        b.iter(|| {
            for error in &errors {
                black_box(ZfsError::is_retryable(error));
            }
        })
    });
}

/// Benchmark serialization performance
fn bench_serialization(c: &mut Criterion) {
    let config = ZfsUnifiedMcpConfig::default();
    let metrics = crate::performance::CurrentPerformanceMetrics::default();

    let mut group = c.benchmark_group("serialization");

    group.bench_function("config_json_serialize", |b| {
        b.iter(|| {
            black_box(serde_json::to_string(&config).map_err(|e| {
                tracing::error!("JSON serialization failed: {}", e);
                std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("JSON serialization error: {}", e),
                )
            })?)
        })
    });

    group.bench_function("metrics_json_serialize", |b| {
        b.iter(|| {
            black_box(serde_json::to_string(&metrics).map_err(|e| {
                tracing::error!("JSON serialization failed: {}", e);
                std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("JSON serialization error: {}", e),
                )
            })?)
        })
    });

    let config_json = serde_json::to_string(&config).map_err(|e| {
        tracing::error!("JSON serialization failed: {}", e);
        std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("JSON serialization error: {}", e),
        )
    })?;
    group.bench_function("config_json_deserialize", |b| {
        b.iter(|| {
            black_box(
                serde_json::from_str::<ZfsMcpConfig>(&config_json).unwrap_or_else(|e| {
                    tracing::error!("Unwrap failed: {:?}", e);
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("Operation failed: {:?}", e),
                    )
                    .into());
                }),
            )
        })
    });

    group.finish();
}

/// Benchmark async operations
fn bench_async_operations(c: &mut Criterion) {
    let rt = Runtime::new().unwrap_or_else(|e| {
        tracing::error!("Unwrap failed: {:?}", e);
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Operation failed: {:?}", e),
        )
        .into());
    });

    c.bench_function("async_task_spawning", |b| {
        b.to_async(&rt).iter(|| async {
            let handles: Vec<_> = (0..100)
                .map(|i| {
                    tokio::spawn(async move {
                        tokio::time::sleep(Duration::from_nanos(1)).await;
                        black_box(i)
                    })
                })
                .collect();

            for handle in handles {
                black_box(handle.await.unwrap_or_else(|e| {
                    tracing::error!("Unwrap failed: {:?}", e);
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("Operation failed: {:?}", e),
                    )
                    .into());
                }));
            }
        })
    });
}

/// Helper function to create test optimization opportunities
fn create_test_opportunities(count: usize) -> Vec<OptimizationOpportunity> {
    (0..count)
        .map(|i| {
            OptimizationOpportunity {
                optimization_type: OptimizationType::TierMigration,
                description: format!("Optimization {i}"),
                expected_impact: (i as f64 * 3.7) % 100.0, // Pseudo-random impact
                confidence: 0.5 + (i as f64 * 0.1) % 0.5,
                complexity: OptimizationComplexity::Medium,
                implementation_time: Duration::from_secs(60 + (i as u64 * 13) % 300),
            }
        })
        .collect()
}

criterion_group!(
    benches,
    bench_config_creation,
    bench_config_validation,
    bench_tier_config_access,
    bench_performance_metrics,
    bench_tier_metrics_generation,
    bench_ai_optimization_sorting,
    bench_migration_job_creation,
    bench_snapshot_policy_validation,
    bench_concurrent_metrics,
    bench_memory_allocation,
    bench_error_handling,
    bench_serialization,
    bench_async_operations
);

criterion_main!(benches);
