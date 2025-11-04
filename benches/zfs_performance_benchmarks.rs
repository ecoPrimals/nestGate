//! Performance benchmarks for ZFS optimization algorithms
//! Measures the performance characteristics of bottleneck detection and optimization

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::SystemTime;
use tokio::runtime::Runtime;

use nestgate_zfs::performance_engine::{monitoring::RealTimePerformanceMonitor, types::*};
use nestgate_zfs::{config::ZfsConfig, dataset::ZfsDatasetManager, pool::ZfsPoolManager};

/// Benchmark performance monitoring creation and operations
fn bench_performance_monitoring(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    c.bench_function("performance_monitor_creation", |b| {
        b.iter(|| black_box(RealTimePerformanceMonitor::new()))
    });

    c.bench_function("performance_metrics_creation", |b| {
        b.iter(|| {
            black_box(ZfsPerformanceMetrics {
                timestamp: SystemTime::now(),
                pool_metrics: HashMap::new(),
                dataset_metrics: HashMap::new(),
                system_memory: SystemMemoryUsage {
                    total: 16 * 1024 * 1024 * 1024,
                    used: 8 * 1024 * 1024 * 1024,
                    available: 8 * 1024 * 1024 * 1024,
                },
                arc_stats: ArcStatistics {
                    size: 4 * 1024 * 1024 * 1024,
                    target_size: 8 * 1024 * 1024 * 1024,
                    hit_ratio: 0.85,
                    miss_ratio: 0.15,
                },
            })
        })
    });

    c.bench_function("trending_data_access", |b| {
        let monitor = RealTimePerformanceMonitor::new();

        // Pre-populate with test data
        rt.block_on(async {
            let metrics = ZfsPerformanceMetrics::default();
            let mut cache = monitor.get_metrics_cache().write().await;
            for i in 0..100 {
                cache.insert(format!("metrics_{}", i), metrics.clone());
            }
        });

        b.iter(|| rt.block_on(async { black_box(monitor.get_trending_data().await.unwrap()) }))
    });
}

/// Benchmark bottleneck detection algorithms
fn bench_bottleneck_detection(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    let mut group = c.benchmark_group("bottleneck_detection");

    for pool_count in [1, 5, 10, 50].iter() {
        group.bench_with_input(
            BenchmarkId::new("high_latency_detection", pool_count),
            pool_count,
            |b, &pool_count| {
                let monitor = RealTimePerformanceMonitor::new();

                // Create test metrics with multiple pools
                let mut pool_metrics = HashMap::new();
                for i in 0..pool_count {
                    pool_metrics.insert(
                        format!("pool_{}", i),
                        ZfsPoolMetrics {
                            pool_name: format!("pool_{}", i),
                            read_ops: 100.0 + i as f64,
                            write_ops: 50.0 + i as f64,
                            read_bandwidth: 1000.0 + i as f64,
                            write_bandwidth: 500.0 + i as f64,
                            latency: 120.0 + i as f64, // High latency
                            cache_hit_ratio: 0.85 - (i as f64 * 0.01),
                            fragmentation: 15.0 + i as f64,
                        },
                    );
                }

                let metrics = ZfsPerformanceMetrics {
                    timestamp: SystemTime::now(),
                    pool_metrics,
                    dataset_metrics: HashMap::new(),
                    system_memory: SystemMemoryUsage {
                        total: 16 * 1024 * 1024 * 1024,
                        used: 8 * 1024 * 1024 * 1024,
                        available: 8 * 1024 * 1024 * 1024,
                    },
                    arc_stats: ArcStatistics {
                        size: 4 * 1024 * 1024 * 1024,
                        target_size: 8 * 1024 * 1024 * 1024,
                        hit_ratio: 0.85,
                        miss_ratio: 0.15,
                    },
                };

                b.iter(|| {
                    rt.block_on(async {
                        // Store metrics for analysis
                        {
                            let mut cache = monitor.get_metrics_cache().write().await;
                            cache.insert("test_metrics".to_string(), metrics.clone());
                        }

                        // Benchmark the trending data retrieval and analysis
                        let trending_data = monitor.get_trending_data().await.unwrap();
                        black_box(trending_data)
                    })
                })
            },
        );
    }

    group.finish();
}

/// Benchmark optimization algorithm performance
fn bench_optimization_algorithms(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    let mut group = c.benchmark_group("optimization_algorithms");

    for optimization_count in [1, 5, 10, 25].iter() {
        group.bench_with_input(
            BenchmarkId::new("applied_optimization_creation", optimization_count),
            optimization_count,
            |b, &optimization_count| {
                b.iter(|| {
                    let mut optimizations = Vec::new();
                    for i in 0..optimization_count {
                        optimizations.push(AppliedOptimization {
                            optimization_type: match i % 7 {
                                0 => OptimizationType::LatencyOptimization,
                                1 => OptimizationType::CacheOptimization,
                                2 => OptimizationType::ThroughputOptimization,
                                3 => OptimizationType::FragmentationDefrag,
                                4 => OptimizationType::ArcTuning,
                                5 => OptimizationType::RecordSizeOptimization,
                                _ => OptimizationType::CompressionOptimization,
                            },
                            description: format!("Optimization {} applied", i),
                            performance_impact: 10.0 + (i as f64 * 2.5),
                            applied_at: SystemTime::now(),
                        });
                    }
                    black_box(optimizations)
                })
            },
        );
    }

    group.bench_with_input(
        BenchmarkId::new("optimization_result_creation", &25),
        &25,
        |b, &optimization_count| {
            b.iter(|| {
                let mut result = PerformanceOptimizationResult::default();

                for i in 0..optimization_count {
                    result.applied_optimizations.push(AppliedOptimization {
                        optimization_type: OptimizationType::LatencyOptimization,
                        description: format!("Test optimization {}", i),
                        performance_impact: 5.0 + i as f64,
                        applied_at: SystemTime::now(),
                    });
                }

                result.performance_improvement = optimization_count as f64 * 5.0;
                result.recommendations = vec![
                    "Monitor performance after optimization".to_string(),
                    "Schedule regular performance reviews".to_string(),
                ];

                black_box(result)
            })
        },
    );

    group.finish();
}

/// Benchmark bottleneck structure creation and manipulation
fn bench_bottleneck_structures(c: &mut Criterion) {
    let mut group = c.benchmark_group("bottleneck_structures");

    for bottleneck_count in [1, 10, 50, 100].iter() {
        group.bench_with_input(
            BenchmarkId::new("bottleneck_creation", bottleneck_count),
            bottleneck_count,
            |b, &bottleneck_count| {
                b.iter(|| {
                    let mut bottlenecks = Vec::new();
                    for i in 0..bottleneck_count {
                        bottlenecks.push(ZfsBottleneck {
                            bottleneck_type: match i % 8 {
                                0 => ZfsBottleneckType::HighLatency,
                                1 => ZfsBottleneckType::LowThroughput,
                                2 => ZfsBottleneckType::CacheMiss,
                                3 => ZfsBottleneckType::Fragmentation,
                                4 => ZfsBottleneckType::MemoryPressure,
                                5 => ZfsBottleneckType::CpuUtilization,
                                6 => ZfsBottleneckType::NetworkBandwidth,
                                _ => ZfsBottleneckType::DiskIo,
                            },
                            severity: match i % 4 {
                                0 => BottleneckSeverity::Low,
                                1 => BottleneckSeverity::Medium,
                                2 => BottleneckSeverity::High,
                                _ => BottleneckSeverity::Critical,
                            },
                            pool_name: format!("pool_{}", i),
                            dataset_name: if i % 2 == 0 {
                                Some(format!("dataset_{}", i))
                            } else {
                                None
                            },
                            description: format!("Bottleneck {} detected", i),
                            impact_score: 50.0 + (i as f64 * 2.0) % 50.0,
                        });
                    }
                    black_box(bottlenecks)
                })
            },
        );
    }

    group.finish();
}

/// Benchmark memory usage patterns
fn bench_memory_usage(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    c.bench_function("memory_intensive_metrics_processing", |b| {
        let monitor = RealTimePerformanceMonitor::new();

        b.iter(|| {
            rt.block_on(async {
                // Create large metrics dataset
                let mut large_pool_metrics = HashMap::new();
                for i in 0..1000 {
                    large_pool_metrics.insert(
                        format!("pool_{}", i),
                        ZfsPoolMetrics {
                            pool_name: format!("pool_{}", i),
                            read_ops: 100.0 + i as f64,
                            write_ops: 50.0 + i as f64,
                            read_bandwidth: 1000.0 + i as f64,
                            write_bandwidth: 500.0 + i as f64,
                            latency: 25.0 + (i as f64 * 0.1),
                            cache_hit_ratio: 0.85 - (i as f64 * 0.0001),
                            fragmentation: 15.0 + (i as f64 * 0.05),
                        },
                    );
                }

                let large_metrics = ZfsPerformanceMetrics {
                    timestamp: SystemTime::now(),
                    pool_metrics: large_pool_metrics,
                    dataset_metrics: HashMap::new(),
                    system_memory: SystemMemoryUsage {
                        total: 16 * 1024 * 1024 * 1024,
                        used: 8 * 1024 * 1024 * 1024,
                        available: 8 * 1024 * 1024 * 1024,
                    },
                    arc_stats: ArcStatistics {
                        size: 4 * 1024 * 1024 * 1024,
                        target_size: 8 * 1024 * 1024 * 1024,
                        hit_ratio: 0.85,
                        miss_ratio: 0.15,
                    },
                };

                // Store and retrieve large metrics
                {
                    let mut cache = monitor.get_metrics_cache().write().await;
                    cache.insert("large_metrics".to_string(), large_metrics);
                }

                let trending_data = monitor.get_trending_data().await.unwrap();
                black_box(trending_data)
            })
        })
    });
}

/// Benchmark concurrent access patterns
fn bench_concurrent_access(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    c.bench_function("concurrent_metrics_access", |b| {
        let monitor = Arc::new(RealTimePerformanceMonitor::new());

        // Pre-populate with test data
        rt.block_on(async {
            let metrics = ZfsPerformanceMetrics::default();
            let mut cache = monitor.get_metrics_cache().write().await;
            for i in 0..50 {
                cache.insert(format!("metrics_{}", i), metrics.clone());
            }
        });

        b.iter(|| {
            rt.block_on(async {
                // Simulate concurrent access
                let mut handles = Vec::new();

                for _ in 0..10 {
                    let monitor_clone = Arc::clone(&monitor);
                    let handle =
                        tokio::spawn(
                            async move { monitor_clone.get_trending_data().await.unwrap() },
                        );
                    handles.push(handle);
                }

                let results = futures::future::join_all(handles).await;
                black_box(results)
            })
        })
    });
}

/// Benchmark trend calculation algorithms
fn bench_trend_calculation(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    let mut group = c.benchmark_group("trend_calculation");

    for data_points in [10, 50, 100, 500].iter() {
        group.bench_with_input(
            BenchmarkId::new("trend_analysis", data_points),
            data_points,
            |b, &data_points| {
                // Create test data
                let test_values: Vec<f64> = (0..data_points)
                    .map(|i| 100.0 + (i as f64 * 0.5) + (i as f64).sin() * 10.0)
                    .collect();

                b.iter(|| {
                    // Simulate trend calculation (simple linear regression)
                    let n = test_values.len() as f64;
                    let x_sum: f64 = (0..test_values.len()).map(|i| i as f64).sum();
                    let y_sum: f64 = test_values.iter().sum();
                    let xy_sum: f64 = test_values
                        .iter()
                        .enumerate()
                        .map(|(i, &y)| i as f64 * y)
                        .sum();
                    let x_squared_sum: f64 =
                        (0..test_values.len()).map(|i| (i as f64).powi(2)).sum();

                    let slope = (n * xy_sum - x_sum * y_sum) / (n * x_squared_sum - x_sum.powi(2));
                    black_box(slope)
                })
            },
        );
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_performance_monitoring,
    bench_bottleneck_detection,
    bench_optimization_algorithms,
    bench_bottleneck_structures,
    bench_memory_usage,
    bench_concurrent_access,
    bench_trend_calculation
);
criterion_main!(benches);
