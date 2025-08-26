//! Simple Performance Benchmark
//!
//! Basic benchmark to establish performance baseline and verify benchmark infrastructure

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::collections::HashMap;

// Simple function benchmarks for core operations
fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

// Benchmark basic CPU-intensive operation
fn bench_fibonacci(c: &mut Criterion) {
    c.bench_function("fibonacci_20", |b| b.iter(|| fibonacci(black_box(20))));
}

// Benchmark HashMap operations (common in our codebase)
fn bench_hashmap_operations(c: &mut Criterion) {
    c.bench_function("hashmap_1000_inserts", |b| {
        b.iter(|| {
            let mut map = HashMap::new();
            for i in 0..1000 {
                map.insert(black_box(i), black_box(format!("value_{i}")));
            }
            black_box(map)
        })
    });
}

// Benchmark string operations (common for configurations)
fn bench_string_operations(c: &mut Criterion) {
    c.bench_function("string_concat_100", |b| {
        b.iter(|| {
            let mut result = String::new();
            for i in 0..100 {
                result.push_str(&format!("item_{}", black_box(i)));
            }
            black_box(result);
        })
    });
}

criterion_group!(
    benches,
    bench_fibonacci,
    bench_hashmap_operations,
    bench_string_operations
);

criterion_main!(benches);
