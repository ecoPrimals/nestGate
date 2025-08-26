//! # Advanced Performance Benchmark Suite
//!
//! Comprehensive benchmarks for NestGate's zero-copy operations,
//! memory management, and overall system performance.

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use nestgate_core::{
    error::Result,
    optimized::{
        CloneMetrics, CompletlySafeBuffer, LargeBuffer, MediumBuffer, SmallBuffer, StringPool,
        ZeroCopyResults,
    },
    universal_storage::zero_copy::ZeroCopyBuffer,
    zero_copy::{BufferManager, StringUtils},
};
use std::sync::Arc;
use tokio::runtime::Runtime;

/// Benchmark zero-copy buffer operations
fn bench_zero_copy_buffers(c: &mut Criterion) {
    let mut group = c.benchmark_group("zero_copy_buffers");

    // Test different buffer sizes
    let sizes = [64, 256, 1024, 4096, 65536];

    for size in sizes.iter() {
        group.bench_with_input(
            BenchmarkId::new("safe_buffer_create", size),
            size,
            |b, &size| {
                b.iter(|| {
                    let buffer = CompletlySafeBuffer::<1024>::new();
                    black_box(buffer);
                });
            },
        );

        group.bench_with_input(
            BenchmarkId::new("zero_copy_borrowed", size),
            size,
            |b, &size| {
                let data = vec![0u8; size];
                b.iter(|| {
                    let buffer = ZeroCopyBuffer::Borrowed(&data);
                    black_box(buffer);
                });
            },
        );

        group.bench_with_input(
            BenchmarkId::new("zero_copy_shared", size),
            size,
            |b, &size| {
                let data = bytes::Bytes::from(vec![0u8; size]);
                b.iter(|| {
                    let buffer = ZeroCopyBuffer::Shared(data.clone());
                    black_box(buffer);
                });
            },
        );
    }

    group.finish();
}

/// Benchmark string operations and optimizations
fn bench_string_optimizations(c: &mut Criterion) {
    let mut group = c.benchmark_group("string_optimizations");

    let test_strings = [
        "short",
        "medium length string for testing",
        "very long string that would benefit from zero-copy optimizations and efficient memory management patterns",
    ];

    for test_str in test_strings.iter() {
        group.bench_with_input(
            BenchmarkId::new("static_cow", test_str.len()),
            test_str,
            |b, &test_str| {
                b.iter(|| {
                    let cow = StringUtils::static_cow(test_str);
                    black_box(cow);
                });
            },
        );

        group.bench_with_input(
            BenchmarkId::new("string_pool", test_str.len()),
            test_str,
            |b, &test_str| {
                let pool = StringPool::new();
                b.iter(|| {
                    let interned = pool.get_or_intern(test_str).unwrap();
                    black_box(interned);
                });
            },
        );
    }

    group.finish();
}

/// Benchmark memory allocation patterns
fn bench_memory_patterns(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_patterns");

    // Test Arc vs cloning for shared data
    let large_data = vec![0u8; 65536];

    group.bench_function("arc_clone", |b| {
        let arc_data = Arc::new(large_data.clone());
        b.iter(|| {
            let cloned = Arc::clone(&arc_data);
            black_box(cloned);
        });
    });

    group.bench_function("vec_clone", |b| {
        b.iter(|| {
            let cloned = large_data.clone();
            black_box(cloned);
        });
    });

    // Test buffer reuse patterns
    group.bench_function("buffer_reuse", |b| {
        let mut manager = BufferManager::new(1024);
        b.iter(|| {
            let buffer = manager.get_buffer();
            black_box(buffer);
        });
    });

    group.finish();
}

/// Benchmark async operations performance
fn bench_async_operations(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("async_operations");

    group.bench_function("async_buffer_operations", |b| {
        b.to_async(&rt).iter(|| async {
            let buffer = SmallBuffer::new();
            let result = simulate_async_operation(buffer).await;
            black_box(result);
        });
    });

    group.bench_function("concurrent_zero_copy", |b| {
        b.to_async(&rt).iter(|| async {
            let data = bytes::Bytes::from_static(b"test data for concurrent processing");
            let tasks: Vec<_> = (0..10)
                .map(|_| {
                    let data_clone = data.clone(); // Zero-copy clone
                    tokio::spawn(async move { process_data_zero_copy(data_clone).await })
                })
                .collect();

            let results = futures::future::join_all(tasks).await;
            black_box(results);
        });
    });

    group.finish();
}

/// Benchmark system integration performance
fn bench_system_integration(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("system_integration");

    group.bench_function("full_pipeline", |b| {
        b.to_async(&rt).iter(|| async {
            // Simulate a full request pipeline with zero-copy optimizations
            let input_data = bytes::Bytes::from_static(b"pipeline test data");

            // Stage 1: Parse (zero-copy)
            let parsed = parse_zero_copy(&input_data).await.unwrap();

            // Stage 2: Process (zero-copy)
            let processed = process_data_zero_copy(parsed).await.unwrap();

            // Stage 3: Serialize (minimal copy)
            let output = serialize_efficient(processed).await.unwrap();

            black_box(output);
        });
    });

    group.finish();
}

/// Helper functions for benchmarks

async fn simulate_async_operation(buffer: SmallBuffer) -> Result<()> {
    tokio::task::yield_now().await;
    black_box(buffer);
    Ok(())
}

async fn parse_zero_copy(data: &bytes::Bytes) -> Result<bytes::Bytes> {
    // Simulate parsing without copying
    Ok(data.clone()) // This is a zero-copy operation with Bytes
}

async fn process_data_zero_copy(data: bytes::Bytes) -> Result<bytes::Bytes> {
    // Simulate processing that maintains zero-copy semantics
    tokio::task::yield_now().await;
    Ok(data)
}

async fn serialize_efficient(data: bytes::Bytes) -> Result<Vec<u8>> {
    // Simulate serialization with minimal copying
    Ok(data.to_vec())
}

impl BufferManager {
    fn new(buffer_size: usize) -> Self {
        Self {
            buffers: Vec::new(),
            buffer_size,
        }
    }

    fn get_buffer(&mut self) -> Vec<u8> {
        self.buffers
            .pop()
            .unwrap_or_else(|| vec![0u8; self.buffer_size])
    }
}

criterion_group!(
    advanced_benches,
    bench_zero_copy_buffers,
    bench_string_optimizations,
    bench_memory_patterns,
    bench_async_operations,
    bench_system_integration
);

criterion_main!(advanced_benches);
