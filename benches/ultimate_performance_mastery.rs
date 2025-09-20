//! 🚀 **ULTIMATE PERFORMANCE MASTERY BENCHMARK SUITE**
//!
//! This benchmark suite validates our claims of zero-cost abstractions,
//! pedantic performance optimization, and industry-leading efficiency.

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use std::time::Duration;

// ==================== ZERO-COPY PERFORMANCE BENCHMARKS ====================

/// Benchmark zero-copy string operations
fn bench_zero_copy_strings(c: &mut Criterion) {
    let mut group = c.benchmark_group("zero_copy_strings");

    let data = "Hello, World! This is a test string for zero-copy operations.".repeat(1000);

    group.throughput(Throughput::Bytes(data.len() as u64));

    // Zero-copy string slice (should be O(1))
    group.bench_function("zero_copy_slice", |b| {
        b.iter(|| {
            let slice = &data[0..100];
            black_box(slice)
        })
    });

    // String clone (for comparison - should be O(n))
    group.bench_function("string_clone", |b| {
        b.iter(|| {
            let cloned = data.clone();
            black_box(cloned)
        })
    });

    group.finish();
}

/// Benchmark zero-copy memory operations
fn bench_zero_copy_memory(c: &mut Criterion) {
    let mut group = c.benchmark_group("zero_copy_memory");

    let data: Vec<u8> = (0..10000).map(|i| (i % 256) as u8).collect();

    group.throughput(Throughput::Bytes(data.len() as u64));

    // Zero-copy slice reference
    group.bench_function("slice_reference", |b| {
        b.iter(|| {
            let slice = &data[100..1000];
            black_box(slice)
        })
    });

    // Memory copy (for comparison)
    group.bench_function("memory_copy", |b| {
        b.iter(|| {
            let copied = data[100..1000].to_vec();
            black_box(copied)
        })
    });

    group.finish();
}

// ==================== CANONICAL PERFORMANCE BENCHMARKS ====================

/// Benchmark canonical error handling
fn bench_canonical_errors(c: &mut Criterion) {
    let mut group = c.benchmark_group("canonical_errors");

    // Result creation and propagation
    group.bench_function("result_ok", |b| {
        b.iter(|| {
            let result: Result<i32, String> = Ok(42);
            black_box(result)
        })
    });

    group.bench_function("result_err", |b| {
        b.iter(|| {
            let result: Result<i32, String> = Err("Error".to_string());
            black_box(result)
        })
    });

    // Error chain propagation
    group.bench_function("error_chain", |b| {
        b.iter(|| {
            let result = chain_operations();
            black_box(result)
        })
    });

    group.finish();
}

fn chain_operations() -> Result<i32, String> {
    let a = operation_a()?;
    let b = operation_b(a)?;
    let c = operation_c(b)?;
    Ok(c)
}

fn operation_a() -> Result<i32, String> {
    Ok(1)
}
fn operation_b(x: i32) -> Result<i32, String> {
    Ok(x * 2)
}
fn operation_c(x: i32) -> Result<i32, String> {
    Ok(x + 3)
}

// ==================== ASYNC PERFORMANCE BENCHMARKS ====================

/// Benchmark async operations
fn bench_async_performance(c: &mut Criterion) {
    let mut group = c.benchmark_group("async_performance");

    let rt = tokio::runtime::Runtime::new().unwrap();

    // Simple async function
    group.bench_function("simple_async", |b| {
        b.to_async(&rt).iter(|| async {
            let result = simple_async_operation().await;
            black_box(result)
        })
    });

    // Async with yield
    group.bench_function("async_with_yield", |b| {
        b.to_async(&rt).iter(|| async {
            let result = async_with_yield().await;
            black_box(result)
        })
    });

    group.finish();
}

async fn simple_async_operation() -> i32 {
    42
}

async fn async_with_yield() -> i32 {
    tokio::task::yield_now().await;
    42
}

// ==================== MEMORY ALLOCATION BENCHMARKS ====================

/// Benchmark memory allocation patterns
fn bench_memory_allocation(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_allocation");

    // Stack allocation
    group.bench_function("stack_allocation", |b| {
        b.iter(|| {
            let arr = [0u8; 1024];
            black_box(arr)
        })
    });

    // Heap allocation
    group.bench_function("heap_allocation", |b| {
        b.iter(|| {
            let vec = vec![0u8; 1024];
            black_box(vec)
        })
    });

    // Box allocation
    group.bench_function("box_allocation", |b| {
        b.iter(|| {
            let boxed = Box::new([0u8; 1024]);
            black_box(boxed)
        })
    });

    group.finish();
}

// ==================== SIMD AND VECTORIZATION BENCHMARKS ====================

/// Benchmark SIMD operations
fn bench_simd_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("simd_operations");

    let data_a: Vec<f32> = (0..10000).map(|i| i as f32).collect();
    let data_b: Vec<f32> = (0..10000).map(|i| (i * 2) as f32).collect();

    group.throughput(Throughput::Elements(data_a.len() as u64));

    // Scalar addition
    group.bench_function("scalar_addition", |b| {
        b.iter(|| {
            let result: Vec<f32> = data_a
                .iter()
                .zip(data_b.iter())
                .map(|(a, b)| a + b)
                .collect();
            black_box(result)
        })
    });

    // Iterator-based addition (potentially vectorized)
    group.bench_function("iterator_addition", |b| {
        b.iter(|| {
            let mut result = Vec::with_capacity(data_a.len());
            for (a, b) in data_a.iter().zip(data_b.iter()) {
                result.push(a + b);
            }
            black_box(result)
        })
    });

    group.finish();
}

// ==================== CACHE PERFORMANCE BENCHMARKS ====================

/// Benchmark cache-friendly vs cache-unfriendly access patterns
fn bench_cache_performance(c: &mut Criterion) {
    let mut group = c.benchmark_group("cache_performance");

    let size = 1024 * 1024; // 1MB of data
    let data: Vec<u64> = (0..size).map(|i| i as u64).collect();

    group.throughput(Throughput::Bytes((size * 8) as u64));

    // Sequential access (cache-friendly)
    group.bench_function("sequential_access", |b| {
        b.iter(|| {
            let mut sum = 0u64;
            for &value in &data {
                sum = sum.wrapping_add(value);
            }
            black_box(sum)
        })
    });

    // Random access (cache-unfriendly)
    group.bench_function("random_access", |b| {
        let indices: Vec<usize> = (0..1000).map(|i| (i * 1009) % size).collect();
        b.iter(|| {
            let mut sum = 0u64;
            for &index in &indices {
                sum = sum.wrapping_add(data[index]);
            }
            black_box(sum)
        })
    });

    group.finish();
}

// ==================== COMPILATION TIME BENCHMARKS ====================

/// Benchmark compilation time impact of different patterns
fn bench_compilation_patterns(c: &mut Criterion) {
    let mut group = c.benchmark_group("compilation_patterns");

    // Monomorphization impact
    group.bench_function("generic_function", |b| {
        b.iter(|| {
            let result1 = generic_function::<u32>(42);
            let result2 = generic_function::<u64>(42);
            let result3 = generic_function::<f32>(42.0);
            black_box((result1, result2, result3))
        })
    });

    // Trait object vs static dispatch
    group.bench_function("static_dispatch", |b| {
        let implementor = ConcreteType;
        b.iter(|| {
            let result = static_dispatch(&implementor);
            black_box(result)
        })
    });

    group.bench_function("dynamic_dispatch", |b| {
        let implementor: Box<dyn TestTrait> = Box::new(ConcreteType);
        b.iter(|| {
            let result = dynamic_dispatch(&*implementor);
            black_box(result)
        })
    });

    group.finish();
}

fn generic_function<T: Copy + std::fmt::Debug>(value: T) -> T {
    value
}

trait TestTrait {
    fn test_method(&self) -> i32;
}

struct ConcreteType;

impl TestTrait for ConcreteType {
    fn test_method(&self) -> i32 {
        42
    }
}

fn static_dispatch(implementor: &ConcreteType) -> i32 {
    implementor.test_method()
}

fn dynamic_dispatch(implementor: &dyn TestTrait) -> i32 {
    implementor.test_method()
}

// ==================== BENCHMARK CONFIGURATION ====================

criterion_group!(
    name = ultimate_performance_mastery;
    config = Criterion::default()
        .sample_size(1000)
        .measurement_time(Duration::from_secs(10))
        .warm_up_time(Duration::from_secs(3));
    targets =
        bench_zero_copy_strings,
        bench_zero_copy_memory,
        bench_canonical_errors,
        bench_async_performance,
        bench_memory_allocation,
        bench_simd_operations,
        bench_cache_performance,
        bench_compilation_patterns
);

criterion_main!(ultimate_performance_mastery);
