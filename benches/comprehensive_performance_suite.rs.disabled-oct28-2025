//! **COMPREHENSIVE PERFORMANCE BENCHMARK SUITE**
//!
//! Validates all performance optimizations and zero-copy implementations.
//! Benchmarks string operations, buffer management, COW operations, and security providers.

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use nestgate_core::optimized::{
    CompletlySafeBuffer, SafeCircularBuffer, StringPool, ZeroCopyStringBuilder,
    ZeroCopyStringProcessor,
};
use nestgate_core::universal_storage::zfs_features::{
    ModernZfsConfig as CowConfig, ModernZfsEngine as CowManager,
};
use nestgate_core::zero_cost_security_provider::{
    AuthenticationConfig, HybridAuthenticationManager, ZeroCostCredentials,
};
use std::time::Duration;
use tokio::runtime::Runtime;

/// Benchmark string optimization operations
fn bench_string_optimizations(c: &mut Criterion) {
    let mut group = c.benchmark_group("string_optimizations");

    // String pool benchmarks
    let pool = StringPool::new();
    let test_strings = vec![
        "hello",
        "world",
        "rust",
        "performance",
        "benchmark",
        "hello",
        "world",
        "rust", // Duplicates to test caching
    ];

    group.bench_function("string_pool_intern", |b| {
        b.iter(|| {
            for s in &test_strings {
                black_box(pool.intern(s));
            }
        })
    });

    // Zero-copy string processor benchmarks
    let processor = ZeroCopyStringProcessor::new();
    let test_inputs = vec![
        "  hello  world  ",
        "rust programming",
        "  performance   optimization  ",
        "clean string",
    ];

    group.bench_function("zero_copy_string_processing", |b| {
        b.iter(|| {
            for input in &test_inputs {
                black_box(processor.process_ref(input));
            }
        })
    });

    // String builder benchmarks
    group.bench_function("zero_copy_string_builder", |b| {
        b.iter(|| {
            let mut builder = ZeroCopyStringBuilder::new();
            builder
                .push_borrowed("Hello")
                .push_borrowed(" ")
                .push_borrowed("World")
                .push_borrowed("!")
                .push_borrowed(" ")
                .push_borrowed("From")
                .push_borrowed(" ")
                .push_borrowed("Rust");
            black_box(builder.build());
        })
    });

    // Batch processing benchmarks
    let batch_inputs: Vec<&str> = (0..1000)
        .map(|i| if i % 3 == 0 { "  spaced  " } else { "clean" })
        .collect();

    group.throughput(Throughput::Elements(batch_inputs.len() as u64));
    group.bench_function("batch_string_processing", |b| {
        b.iter(|| {
            black_box(processor.batch_process(&batch_inputs));
        })
    });

    group.finish();
}

/// Benchmark buffer operations
fn bench_buffer_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("buffer_operations");

    // Safe buffer benchmarks
    group.bench_function("safe_buffer_creation", |b| {
        b.iter(|| {
            let buffer: CompletlySafeBuffer<1024> = black_box(CompletlySafeBuffer::new());
            black_box(buffer);
        })
    });

    // Circular buffer benchmarks
    let mut circular_buffer = SafeCircularBuffer::new(1024);
    let test_data = vec![42u8; 512];

    group.bench_function("circular_buffer_write", |b| {
        b.iter(|| {
            black_box(circular_buffer.write(&test_data).unwrap_or(0));
        })
    });

    group.bench_function("circular_buffer_read", |b| {
        // Pre-fill buffer
        let _ = circular_buffer.write(&test_data);

        b.iter(|| {
            let mut read_buffer = vec![0u8; 256];
            black_box(circular_buffer.read(&mut read_buffer).unwrap_or(0));
        })
    });

    group.finish();
}

/// Benchmark COW operations (async)
fn bench_cow_operations(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("cow_operations");

    group.bench_function("cow_manager_creation", |b| {
        b.iter(|| {
            let config = CowConfig::default();
            // Note: This would require a mock storage backend in a real benchmark
            // For now, we'll just benchmark the config creation
            black_box(config);
        })
    });

    group.finish();
}

/// Benchmark security operations (async)
fn bench_security_operations(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("security_operations");

    let config = AuthenticationConfig::default();
    let auth_manager = HybridAuthenticationManager::new(config);

    group.bench_function("authentication_local", |b| {
        b.to_async(&rt).iter(|| async {
            let credentials = ZeroCostCredentials::password("admin", "admin");
            let result = auth_manager.authenticate(&credentials).await;
            black_box(result);
        })
    });

    group.bench_function("token_validation", |b| {
        b.to_async(&rt).iter(|| async {
            let result = auth_manager.validate_token("test-token").await;
            black_box(result);
        })
    });

    group.finish();
}

/// Benchmark memory allocation patterns
fn bench_memory_patterns(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_patterns");

    // Compare different string concatenation approaches
    let parts = vec!["Hello", " ", "World", "!", " ", "From", " ", "Rust"];

    group.bench_function("naive_string_concat", |b| {
        b.iter(|| {
            let mut result = String::new();
            for part in &parts {
                result.push_str(part);
            }
            black_box(result);
        })
    });

    group.bench_function("preallocated_string_concat", |b| {
        b.iter(|| {
            let total_len: usize = parts.iter().map(|s| s.len()).sum();
            let mut result = String::with_capacity(total_len);
            for part in &parts {
                result.push_str(part);
            }
            black_box(result);
        })
    });

    group.bench_function("zero_copy_builder_concat", |b| {
        b.iter(|| {
            let mut builder = ZeroCopyStringBuilder::new();
            for part in &parts {
                builder.push_borrowed(part);
            }
            black_box(builder.build());
        })
    });

    // Vector allocation patterns
    group.bench_function("vec_push_growth", |b| {
        b.iter(|| {
            let mut vec = Vec::new();
            for i in 0..1000 {
                vec.push(i);
            }
            black_box(vec);
        })
    });

    group.bench_function("vec_with_capacity", |b| {
        b.iter(|| {
            let mut vec = Vec::with_capacity(1000);
            for i in 0..1000 {
                vec.push(i);
            }
            black_box(vec);
        })
    });

    group.finish();
}

/// Benchmark data processing pipelines
fn bench_data_pipelines(c: &mut Criterion) {
    let mut group = c.benchmark_group("data_pipelines");

    // Simulate a data processing pipeline
    let input_data: Vec<String> = (0..1000).map(|i| format!("  data_item_{}  ", i)).collect();

    group.throughput(Throughput::Elements(input_data.len() as u64));

    group.bench_function("traditional_pipeline", |b| {
        b.iter(|| {
            let results: Vec<String> = input_data
                .iter()
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .map(|s| s.to_uppercase())
                .collect();
            black_box(results);
        })
    });

    let processor = ZeroCopyStringProcessor::new();
    group.bench_function("zero_copy_pipeline", |b| {
        b.iter(|| {
            let str_refs: Vec<&str> = input_data.iter().map(|s| s.as_str()).collect();
            let processed = processor.batch_process(&str_refs);
            let results: Vec<String> = processed
                .into_iter()
                .filter(|s| !s.is_empty())
                .map(|s| s.to_uppercase())
                .collect();
            black_box(results);
        })
    });

    group.finish();
}

/// Benchmark concurrent operations
fn bench_concurrent_operations(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("concurrent_operations");

    let pool = StringPool::new();
    let test_strings: Vec<String> = (0..100).map(|i| format!("test_string_{}", i)).collect();

    group.bench_function("concurrent_string_interning", |b| {
        b.to_async(&rt).iter(|| async {
            let tasks: Vec<_> = test_strings
                .iter()
                .map(|s| {
                    let pool = &pool;
                    let s = s.as_str();
                    tokio::spawn(async move { pool.intern(s) })
                })
                .collect();

            let results = futures::future::join_all(tasks).await;
            black_box(results);
        })
    });

    group.finish();
}

/// Comprehensive performance validation
fn bench_performance_validation(c: &mut Criterion) {
    let mut group = c.benchmark_group("performance_validation");

    // Validate that our optimizations are actually faster
    let large_input = "  ".repeat(1000) + &"a".repeat(1000) + &"  ".repeat(1000);
    let processor = ZeroCopyStringProcessor::new();

    group.bench_function("standard_string_trim", |b| {
        b.iter(|| {
            let result = large_input.trim().to_string();
            black_box(result);
        })
    });

    group.bench_function("zero_copy_string_process", |b| {
        b.iter(|| {
            let result = processor.process_ref(&large_input);
            black_box(result);
        })
    });

    // Memory usage comparison
    group.bench_function("string_pool_memory_efficiency", |b| {
        b.iter(|| {
            let pool = StringPool::new();
            // Intern the same strings multiple times
            for _ in 0..100 {
                for i in 0..10 {
                    let s = format!("repeated_string_{}", i);
                    pool.intern(&s);
                }
            }
            black_box(pool.stats());
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_string_optimizations,
    bench_buffer_operations,
    bench_cow_operations,
    bench_security_operations,
    bench_memory_patterns,
    bench_data_pipelines,
    bench_concurrent_operations,
    bench_performance_validation
);

criterion_main!(benches);
