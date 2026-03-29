#![allow(
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

//! Zero-Copy Optimization Benchmarks
//!
//! Benchmarks comparing zero-copy vs traditional copy operations
//! to measure the effectiveness of our optimization strategies.

use criterion::{Criterion, black_box, criterion_group, criterion_main};
use std::borrow::Cow;
use std::sync::Arc;

// Use the unified benchmark configuration system

// Zero-copy string operations using Cow
fn process_string_zerocopy(input: &str) -> Cow<'_, str> {
    if let Some(stripped) = input.strip_prefix("prefix_") {
        Cow::Borrowed(stripped) // Zero-copy slice
    } else {
        Cow::Owned(format!("processed_{input}")) // Only allocate when needed
    }
}

fn process_string_traditional(input: &str) -> String {
    if let Some(stripped) = input.strip_prefix("prefix_") {
        stripped.to_string() // Always allocate
    } else {
        format!("processed_{input}") // Always allocate
    }
}

fn bench_string_processing_zerocopy(c: &mut Criterion) {
    let test_strings = vec![
        "prefix_hello",
        "prefix_world",
        "prefix_test",
        "no_prefix_1",
        "no_prefix_2",
        "prefix_another",
    ];

    c.bench_function("string_processing_zerocopy", |b| {
        b.iter(|| {
            for s in &test_strings {
                black_box(process_string_zerocopy(s));
            }
        })
    });
}

fn bench_string_processing_traditional(c: &mut Criterion) {
    let test_strings = vec![
        "prefix_hello",
        "prefix_world",
        "prefix_test",
        "no_prefix_1",
        "no_prefix_2",
        "prefix_another",
    ];

    c.bench_function("string_processing_traditional", |b| {
        b.iter(|| {
            for s in &test_strings {
                black_box(process_string_traditional(s));
            }
        })
    });
}

// Arc vs clone for shared data
#[derive(Clone)]
struct LargeData {
    data: Vec<u8>,
    metadata: std::collections::HashMap<String, String>,
}

impl LargeData {
    fn new(size: usize) -> Self {
        let mut metadata = std::collections::HashMap::new();
        for i in 0..100 {
            metadata.insert(format!("key_{i}"), format!("value_{i}"));
        }

        Self {
            data: vec![0u8; size],
            metadata,
        }
    }

    // Method to use the fields and eliminate dead code
    fn compute_signature(&self) -> u64 {
        let data_sum: u64 = self.data.iter().map(|&b| b as u64).sum();
        let metadata_sum: u64 = self.metadata.len() as u64 * 42;
        data_sum + metadata_sum
    }
}

fn bench_large_data_clone(c: &mut Criterion) {
    let large_data = LargeData::new(10_000);

    c.bench_function("large_data_clone", |b| {
        b.iter(|| {
            let copies: Vec<_> = (0..10).map(|_| large_data.clone()).collect();
            // Actually use the data to eliminate dead code
            let total_signature: u64 = copies.iter().map(|d| d.compute_signature()).sum();
            black_box(total_signature);
        })
    });
}

fn bench_large_data_arc(c: &mut Criterion) {
    let large_data = Arc::new(LargeData::new(10_000));

    c.bench_function("large_data_arc", |b| {
        b.iter(|| {
            let copies: Vec<_> = (0..10).map(|_| Arc::clone(&large_data)).collect();
            // Actually use the data to eliminate dead code
            let total_signature: u64 = copies.iter().map(|d| d.compute_signature()).sum();
            black_box(total_signature);
        })
    });
}

// Buffer reuse vs allocation
fn bench_buffer_allocation(c: &mut Criterion) {
    c.bench_function("buffer_allocation", |b| {
        b.iter(|| {
            let buffers: Vec<Vec<u8>> = (0..1000).map(|i| vec![0u8; i % 100 + 1]).collect();
            black_box(buffers);
        })
    });
}

fn bench_buffer_reuse(c: &mut Criterion) {
    let mut buffer = Vec::with_capacity(1000);

    c.bench_function("buffer_reuse", |b| {
        b.iter(|| {
            let results: Vec<_> = (0..1000)
                .map(|i| {
                    buffer.clear();
                    buffer.resize(i % 100 + 1, 0u8);
                    buffer.len()
                })
                .collect();
            black_box(results);
        })
    });
}

// Slice vs owned data
fn process_slice(data: &[u8]) -> usize {
    data.iter().map(|&b| b as usize).sum()
}

fn process_owned(data: Vec<u8>) -> usize {
    data.iter().map(|&b| b as usize).sum()
}

fn bench_slice_processing(c: &mut Criterion) {
    let data = vec![1u8; 10000];

    c.bench_function("slice_processing", |b| {
        b.iter(|| {
            black_box(process_slice(&data));
        })
    });
}

fn bench_owned_processing(c: &mut Criterion) {
    let data = vec![1u8; 10000];

    c.bench_function("owned_processing", |b| {
        b.iter(|| {
            black_box(process_owned(data.clone()));
        })
    });
}

criterion_group!(
    zerocopy_benches,
    bench_string_processing_zerocopy,
    bench_string_processing_traditional,
    bench_large_data_clone,
    bench_large_data_arc,
    bench_buffer_allocation,
    bench_buffer_reuse,
    bench_slice_processing,
    bench_owned_processing
);
criterion_main!(zerocopy_benches);
