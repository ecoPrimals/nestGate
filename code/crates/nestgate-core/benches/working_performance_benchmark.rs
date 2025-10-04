//! Working Performance Benchmark
//!
//! This benchmark uses only standard Rust types to measure baseline performance
//! and validate optimization claims without relying on problematic modules.

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

/// Benchmark clone vs Arc performance with different data sizes
fn benchmark_clone_vs_arc_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("clone_vs_arc_scaling");

    // Test different data sizes
    for size in [100, 1000, 10000, 100000].iter() {
        let data = "X".repeat(*size);

        group.bench_with_input(BenchmarkId::new("string_clone", size), size, |b, _| {
            b.iter(|| {
                let cloned = data.clone();
                black_box(cloned);
            });
        });

        group.bench_with_input(BenchmarkId::new("arc_clone", size), size, |b, _| {
            let arc_data = Arc::new(data.clone());
            b.iter(|| {
                let shared = arc_data.clone();
                black_box(shared);
            });
        });
    }

    group.finish();
}

/// Benchmark HashMap operations with different key strategies
fn benchmark_hashmap_key_strategies(c: &mut Criterion) {
    let mut group = c.benchmark_group("hashmap_key_strategies");

    // Test string key cloning vs borrowing
    group.bench_function("owned_string_keys", |b| {
        b.iter(|| {
            let mut map: HashMap<String, i32> = HashMap::new();
            for i in 0..100 {
                map.insert(format!("key_{}", i), i);
            }
            black_box(map);
        });
    });

    group.bench_function("borrowed_str_keys", |b| {
        let keys: Vec<String> = (0..100).map(|i| format!("key_{}", i)).collect();
        b.iter(|| {
            let mut map: HashMap<&str, i32> = HashMap::new();
            for (i, key) in keys.iter().enumerate() {
                map.insert(key, i as i32);
            }
            black_box(map);
        });
    });

    group.finish();
}

/// Benchmark memory allocation patterns
fn benchmark_allocation_patterns(c: &mut Criterion) {
    let mut group = c.benchmark_group("allocation_patterns");

    // Repeated allocation vs reuse
    group.bench_function("repeated_allocation", |b| {
        b.iter(|| {
            for _ in 0..100 {
                let data: Vec<u8> = vec![0; 1024];
                black_box(data);
            }
        });
    });

    group.bench_function("allocation_reuse", |b| {
        b.iter(|| {
            let mut data: Vec<u8> = Vec::with_capacity(1024);
            for _ in 0..100 {
                data.clear();
                data.resize(1024, 0);
                black_box(&data);
            }
        });
    });

    // Small vs large allocations
    group.bench_function("small_allocations", |b| {
        b.iter(|| {
            for _ in 0..1000 {
                let data: Vec<u8> = vec![0; 64];
                black_box(data);
            }
        });
    });

    group.bench_function("large_allocations", |b| {
        b.iter(|| {
            for _ in 0..10 {
                let data: Vec<u8> = vec![0; 64 * 1000];
                black_box(data);
            }
        });
    });

    group.finish();
}

/// Benchmark string operations
fn benchmark_string_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("string_operations");

    // String creation strategies
    group.bench_function("string_format", |b| {
        b.iter(|| {
            for i in 0..100 {
                let s = format!("item_{}", i);
                black_box(s);
            }
        });
    });

    group.bench_function("string_push", |b| {
        b.iter(|| {
            for i in 0..100 {
                let mut s = String::from("item_");
                s.push_str(&i.to_string());
                black_box(s);
            }
        });
    });

    group.bench_function("string_concat", |b| {
        b.iter(|| {
            for i in 0..100 {
                let s = "item_".to_string() + &i.to_string();
                black_box(s);
            }
        });
    });

    group.finish();
}

/// Benchmark data copying strategies
fn benchmark_data_copying(c: &mut Criterion) {
    let mut group = c.benchmark_group("data_copying");

    let source_data = vec![1u8; 4096];

    group.bench_function("vec_clone", |b| {
        b.iter(|| {
            let copied = source_data.clone();
            black_box(copied);
        });
    });

    group.bench_function("slice_copy", |b| {
        b.iter(|| {
            let mut dest = vec![0u8; 4096];
            dest.copy_from_slice(&source_data);
            black_box(dest);
        });
    });

    group.bench_function("manual_copy", |b| {
        b.iter(|| {
            let mut dest = Vec::with_capacity(4096);
            dest.extend_from_slice(&source_data);
            black_box(dest);
        });
    });

    group.finish();
}

/// Benchmark concurrency patterns
fn benchmark_concurrency_patterns(c: &mut Criterion) {
    let mut group = c.benchmark_group("concurrency_patterns");

    // Arc sharing vs cloning
    group.bench_function("arc_sharing_threads", |b| {
        let data = Arc::new(vec![1u8; 1024]);
        b.iter(|| {
            let handles: Vec<_> = (0..4)
                .map(|_| {
                    let data_clone = data.clone();
                    std::thread::spawn(move || {
                        let sum: u32 = data_clone.iter().map(|&x| x as u32).sum();
                        black_box(sum);
                    })
                })
                .collect();

            for handle in handles {
                handle.join().unwrap();
            }
        });
    });

    group.finish();
}

/// Performance reality check - measure actual system capabilities
fn performance_reality_check(c: &mut Criterion) {
    let mut group = c.benchmark_group("reality_check");
    group.measurement_time(Duration::from_secs(10));

    // Measure actual system performance characteristics
    group.bench_function("system_time_overhead", |b| {
        b.iter(|| {
            let start = Instant::now();
            black_box(start);
            let elapsed = start.elapsed();
            black_box(elapsed);
        });
    });

    group.bench_function("memory_bandwidth_test", |b| {
        let data = vec![1u8; 1024 * 1024]; // 1MB
        b.iter(|| {
            let sum: u64 = data.iter().map(|&x| x as u64).sum();
            black_box(sum);
        });
    });

    group.bench_function("cpu_intensive_task", |b| {
        b.iter(|| {
            let mut result = 0u64;
            for i in 0..10000 {
                result = result.wrapping_add(i * i);
            }
            black_box(result);
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    benchmark_clone_vs_arc_scaling,
    benchmark_hashmap_key_strategies,
    benchmark_allocation_patterns,
    benchmark_string_operations,
    benchmark_data_copying,
    benchmark_concurrency_patterns,
    performance_reality_check
);

criterion_main!(benches);
