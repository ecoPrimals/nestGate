// SPDX-License-Identifier: AGPL-3.0-or-later
#![expect(
    unused,
    dead_code,
    deprecated,
    missing_docs,
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::restriction,
    clippy::cargo
)]

//! Simple Performance Validation Benchmark
//!
//! A lightweight benchmark suite that validates core performance characteristics
//! without requiring complex ZFS or security provider dependencies.

use criterion::{Criterion, Throughput, black_box, criterion_group, criterion_main};
use std::collections::HashMap;
use std::time::{Duration, SystemTime};

// ==================== CORE PERFORMANCE BENCHMARKS ====================

/// Benchmark string constant performance vs allocations
fn bench_string_constants(c: &mut Criterion) {
    let mut group = c.benchmark_group("string_operations");
    group.throughput(Throughput::Elements(1000));

    // Benchmark string constants (zero-cost)
    group.bench_function("constants", |b| {
        b.iter(|| {
            for _ in 0..1000 {
                black_box("hot");
                black_box("warm");
                black_box("cold");
                black_box("compression");
                black_box("lz4");
            }
        })
    });

    // Benchmark string allocations (baseline)
    group.bench_function("allocations", |b| {
        b.iter(|| {
            for _ in 0..1000 {
                black_box("hot".to_string());
                black_box("warm".to_string());
                black_box("cold".to_string());
                black_box("compression".to_string());
                black_box("lz4".to_string());
            }
        })
    });

    group.finish();
}

/// Benchmark HashMap operations
fn bench_hashmap_performance(c: &mut Criterion) {
    let mut group = c.benchmark_group("hashmap_operations");
    group.throughput(Throughput::Elements(100));

    let mut map: HashMap<String, String> = HashMap::new();
    for i in 0..100 {
        map.insert(format!("key_{}", i), format!("value_{}", i));
    }

    group.bench_function("lookup", |b| {
        b.iter(|| {
            for i in 0..100 {
                black_box(map.get(&format!("key_{}", i)));
            }
        })
    });

    group.bench_function("insert", |b| {
        b.iter(|| {
            let mut local_map = HashMap::new();
            for i in 0..100 {
                black_box(local_map.insert(format!("key_{}", i), format!("value_{}", i)));
            }
        })
    });

    group.finish();
}

/// Benchmark error handling patterns
fn bench_error_handling(c: &mut Criterion) {
    let mut group = c.benchmark_group("error_handling");
    group.throughput(Throughput::Elements(1000));

    fn result_operation(success: bool) -> Result<String, String> {
        if success {
            Ok("success".to_string())
        } else {
            Err("error".to_string())
        }
    }

    group.bench_function("result_success", |b| {
        b.iter(|| {
            for _ in 0..1000 {
                let _ = black_box(result_operation(true));
            }
        })
    });

    group.bench_function("result_error", |b| {
        b.iter(|| {
            for _ in 0..1000 {
                let _ = black_box(result_operation(false));
            }
        })
    });

    group.finish();
}

/// Benchmark time operations
fn bench_time_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("time_operations");
    group.throughput(Throughput::Elements(1000));

    group.bench_function("system_time_now", |b| {
        b.iter(|| {
            for _ in 0..1000 {
                black_box(SystemTime::now());
            }
        })
    });

    group.bench_function("duration_creation", |b| {
        b.iter(|| {
            for _ in 0..1000 {
                black_box(Duration::from_millis(100));
                black_box(Duration::from_secs(1));
                black_box(Duration::from_nanos(1000));
            }
        })
    });

    group.finish();
}

/// Benchmark memory allocation patterns
fn bench_memory_patterns(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_patterns");
    group.throughput(Throughput::Elements(100));

    group.bench_function("vec_creation", |b| {
        b.iter(|| {
            for _ in 0..100 {
                let mut vec = Vec::new();
                for i in 0..100 {
                    vec.push(i);
                }
                black_box(vec);
            }
        })
    });

    group.bench_function("vec_with_capacity", |b| {
        b.iter(|| {
            for _ in 0..100 {
                let mut vec = Vec::with_capacity(100);
                for i in 0..100 {
                    vec.push(i);
                }
                black_box(vec);
            }
        })
    });

    group.finish();
}

// ==================== BENCHMARK GROUPS ====================

criterion_group!(
    performance_benchmarks,
    bench_string_constants,
    bench_hashmap_performance,
    bench_error_handling,
    bench_time_operations,
    bench_memory_patterns
);

criterion_main!(performance_benchmarks);

fn result_operation(success: bool) -> Result<String, String> {
    if success {
        Ok("success".to_string())
    } else {
        Err("error".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_benchmark_functions() {
        // Smoke test to ensure benchmark functions don't panic
        let result = result_operation(true);
        assert!(result.is_ok());

        let result = result_operation(false);
        assert!(result.is_err());
    }
}
