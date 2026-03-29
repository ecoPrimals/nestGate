// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective
#![allow(
    dead_code,
    missing_docs,
    unused_imports,
    unused_variables,
    clippy::all,
    clippy::cargo,
    clippy::nursery,
    clippy::pedantic,
    clippy::restriction
)]

//! Comprehensive Reality Check Benchmark
//!
//! This benchmark validates actual performance against documented claims
//! using basic Rust types to avoid module dependencies.

use criterion::{Criterion, black_box, criterion_group, criterion_main};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

/// Benchmark memory allocation patterns
fn benchmark_memory_allocation(c: &mut Criterion) {
    c.bench_function("heap_allocation", |b| {
        b.iter(|| {
            let data: Vec<u8> = vec![0; 1024];
            black_box(data);
        });
    });
}

/// Benchmark string operations
fn benchmark_string_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("string_operations");

    group.bench_function("string_creation", |b| {
        let strings = vec!["test1", "test2", "test1", "test2"];
        b.iter(|| {
            let mut owned_strings = Vec::new();
            for s in &strings {
                owned_strings.push(s.to_string());
            }
            black_box(owned_strings);
        });
    });

    group.bench_function("string_cloning", |b| {
        let test_string = "A".repeat(1000); // 1KB string
        b.iter(|| {
            let cloned = test_string.clone();
            black_box(cloned);
        });
    });

    group.finish();
}

/// Benchmark clone vs Arc sharing
fn benchmark_clone_vs_arc(c: &mut Criterion) {
    let data = "A".repeat(1000); // 1KB string

    let mut group = c.benchmark_group("clone_vs_arc");

    group.bench_function("traditional_clone", |b| {
        b.iter(|| {
            let cloned = data.clone();
            black_box(cloned);
        });
    });

    group.bench_function("arc_sharing", |b| {
        let arc_data = Arc::new(data.clone());
        b.iter(|| {
            let shared = arc_data.clone(); // Arc clone is cheap
            black_box(shared);
        });
    });

    group.finish();
}

/// Benchmark HashMap operations
fn benchmark_hashmap_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("hashmap_operations");

    group.bench_function("key_cloning", |b| {
        let mut map: HashMap<String, String> = HashMap::new();
        let keys = vec!["key1", "key2", "key3"];

        b.iter(|| {
            for key in &keys {
                map.insert(key.to_string(), "value".to_string());
            }
            black_box(&map);
            map.clear();
        });
    });

    group.bench_function("key_borrowing", |b| {
        let mut map: HashMap<&str, &str> = HashMap::new();
        let keys = vec!["key1", "key2", "key3"];

        b.iter(|| {
            for &key in &keys {
                map.insert(key, "value");
            }
            black_box(&map);
            map.clear();
        });
    });

    group.finish();
}

/// Benchmark data copying patterns
fn benchmark_data_copying(c: &mut Criterion) {
    let mut group = c.benchmark_group("data_copying");

    group.bench_function("vec_clone", |b| {
        let source = vec![1u8; 4096]; // 4KB
        b.iter(|| {
            let copied = source.clone();
            black_box(copied);
        });
    });

    group.bench_function("slice_copy", |b| {
        let source = vec![1u8; 4096]; // 4KB
        b.iter(|| {
            let mut dest = vec![0u8; 4096];
            dest.copy_from_slice(&source);
            black_box(dest);
        });
    });

    group.finish();
}

/// Reality check: baseline performance measurements
fn performance_baseline(c: &mut Criterion) {
    let mut group = c.benchmark_group("baseline_performance");
    group.measurement_time(Duration::from_secs(5));

    // Basic allocation
    group.bench_function("basic_allocation", |b| {
        b.iter(|| {
            let data: Vec<u8> = vec![0; 1024];
            black_box(data);
        });
    });

    // String formatting
    group.bench_function("string_formatting", |b| {
        let base = "test";
        b.iter(|| {
            let formatted = format!("{}_{}", base, 123);
            black_box(formatted);
        });
    });

    // HashMap access
    group.bench_function("hashmap_access", |b| {
        let mut map = HashMap::new();
        map.insert("key", "value");

        b.iter(|| {
            let value = map.get("key");
            black_box(value);
        });
    });

    group.finish();
}

/// Validate performance optimization claims
fn validate_optimization_claims(c: &mut Criterion) {
    let mut group = c.benchmark_group("optimization_validation");
    group.measurement_time(Duration::from_secs(10));

    // Test if Arc sharing is actually faster than cloning
    let large_data = "X".repeat(10000); // 10KB string

    group.bench_function("large_string_clone", |b| {
        b.iter(|| {
            let cloned = large_data.clone();
            black_box(cloned);
        });
    });

    group.bench_function("large_string_arc_clone", |b| {
        let arc_data = Arc::new(large_data.clone());
        b.iter(|| {
            let shared = arc_data.clone();
            black_box(shared);
        });
    });

    // Test memory reuse patterns
    group.bench_function("repeated_allocation", |b| {
        b.iter(|| {
            for _ in 0..100 {
                let data: Vec<u8> = vec![0; 100];
                black_box(data);
            }
        });
    });

    group.bench_function("reused_allocation", |b| {
        b.iter(|| {
            let mut data: Vec<u8> = Vec::with_capacity(100);
            for _ in 0..100 {
                data.clear();
                data.resize(100, 0);
                black_box(&data);
            }
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    benchmark_memory_allocation,
    benchmark_string_operations,
    benchmark_clone_vs_arc,
    benchmark_hashmap_operations,
    benchmark_data_copying,
    performance_baseline,
    validate_optimization_claims
);

criterion_main!(benches);
