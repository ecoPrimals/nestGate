//! Native Performance Tests
//!
//! Using Criterion benchmarking framework for stable performance testing
//! of key operations in the NestGate codebase.

use criterion::{Criterion, black_box, criterion_group, criterion_main};
use std::collections::HashMap;
use std::sync::Arc;

// Use the unified benchmark configuration system

// Simple mock config for demonstration
#[derive(Clone)]
struct MockConfig {
    name: String,
    value: u64,
    enabled: bool,
}

impl Default for MockConfig {
    fn default() -> Self {
        Self {
            name: "test_config".to_string(),
            value: 42,
            enabled: true,
        }
    }
}

fn bench_regular_clone(c: &mut Criterion) {
    let config = MockConfig::default();
    c.bench_function("regular_clone", |b| {
        b.iter(|| {
            let configs: Vec<_> = (0..1000).map(|_| config.clone()).collect();
            // Actually use the fields to eliminate dead code
            let total_value: u64 = configs
                .iter()
                .filter(|c| c.enabled)
                .map(|c| c.value + c.name.len() as u64)
                .sum();
            black_box(total_value);
        })
    });
}

fn bench_arc_clone(c: &mut Criterion) {
    let config = Arc::new(MockConfig::default());
    c.bench_function("arc_clone", |b| {
        b.iter(|| {
            let configs: Vec<_> = (0..1000).map(|_| Arc::clone(&config)).collect();
            // Actually use the fields to eliminate dead code
            let total_value: u64 = configs
                .iter()
                .filter(|c| c.enabled)
                .map(|c| c.value + c.name.len() as u64)
                .sum();
            black_box(total_value);
        })
    });
}

fn bench_hashmap_operations(c: &mut Criterion) {
    c.bench_function("hashmap_insert", |b| {
        b.iter(|| {
            let mut map = HashMap::new();
            for i in 0..1000 {
                map.insert(black_box(i), black_box(i * 2));
            }
            black_box(map);
        })
    });
}

fn bench_string_operations(c: &mut Criterion) {
    c.bench_function("string_formatting", |b| {
        b.iter(|| {
            let result: Vec<String> = (0..1000)
                .map(|i| format!("config_item_{}", black_box(i)))
                .collect();
            black_box(result);
        })
    });
}

criterion_group!(
    benches,
    bench_regular_clone,
    bench_arc_clone,
    bench_hashmap_operations,
    bench_string_operations
);
criterion_main!(benches);
