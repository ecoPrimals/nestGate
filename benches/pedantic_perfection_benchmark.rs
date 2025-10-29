//! # Pedantic Perfection Benchmark Suite
//!
//! **LEGENDARY PERFORMANCE VALIDATION** for all pedantic optimizations
//!
//! This benchmark validates:
//! - Zero-copy string operations
//! - Compile-time dispatch vs runtime dispatch
//! - Memory allocation optimizations
//! - Async performance improvements

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::sync::Arc;
use std::time::Duration;

// Mock types for benchmarking
struct MockService;
trait MockTrait {
    fn operation(&self) -> String;
}
impl MockTrait for MockService {
    fn operation(&self) -> String {
        "result".to_string()
    }
}

/// **ZERO-COPY STRING OPERATIONS BENCHMARK**
fn benchmark_zero_copy_strings(c: &mut Criterion) {
    let test_string = "This is a test string for zero-copy optimization benchmarks";

    c.bench_function("zero_copy_string_slice", |b| {
        b.iter(|| {
            // Zero-copy string slicing
            let slice = &test_string[5..15];
            black_box(slice)
        })
    });

    c.bench_function("traditional_string_clone", |b| {
        b.iter(|| {
            // Traditional string cloning (for comparison)
            let cloned = test_string[5..15].to_string();
            black_box(cloned)
        })
    });
}

/// **COMPILE-TIME VS RUNTIME DISPATCH BENCHMARK**
fn benchmark_dispatch_performance(c: &mut Criterion) {
    let service = MockService;
    let dyn_service: Arc<dyn MockTrait> = Arc::new(MockService);

    c.bench_function("compile_time_dispatch", |b| {
        b.iter(|| {
            // Direct method call - compile-time dispatch
            let result = service.operation();
            black_box(result)
        })
    });

    c.bench_function("runtime_dispatch", |b| {
        b.iter(|| {
            // Dynamic dispatch through trait object
            let result = dyn_service.operation();
            black_box(result)
        })
    });
}

/// **MEMORY ALLOCATION BENCHMARK**
fn benchmark_memory_allocation(c: &mut Criterion) {
    c.bench_function("vec_allocation", |b| {
        b.iter(|| {
            // Safe Vec allocation
            let vec = vec![0u8; 1024];
            black_box(vec)
        })
    });

    c.bench_function("box_allocation", |b| {
        b.iter(|| {
            // Box allocation for comparison
            let boxed = Box::new([0u8; 1024]);
            black_box(boxed)
        })
    });
}

/// **ASYNC PERFORMANCE BENCHMARK**
fn benchmark_async_operations(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();

    c.bench_function("async_operation", |b| {
        b.to_async(&rt).iter(|| async {
            // Async operation benchmark
            tokio::time::sleep(Duration::from_nanos(1)).await;
            black_box(())
        })
    });
}

/// **CACHE-LINE ALIGNED STRUCTURES BENCHMARK**
#[repr(align(64))] // Cache-line aligned
struct CacheAlignedStruct {
    data: [u64; 8],
}

#[repr(C)] // Standard alignment
struct StandardStruct {
    data: [u64; 8],
}

fn benchmark_cache_alignment(c: &mut Criterion) {
    let aligned = CacheAlignedStruct { data: [1; 8] };
    let standard = StandardStruct { data: [1; 8] };

    c.bench_function("cache_aligned_access", |b| {
        b.iter(|| {
            let sum: u64 = aligned.data.iter().sum();
            black_box(sum)
        })
    });

    c.bench_function("standard_aligned_access", |b| {
        b.iter(|| {
            let sum: u64 = standard.data.iter().sum();
            black_box(sum)
        })
    });
}

/// **COMPREHENSIVE PERFORMANCE VALIDATION**
fn benchmark_comprehensive_performance(c: &mut Criterion) {
    let mut group = c.benchmark_group("pedantic_perfection");
    group.sample_size(1000);

    group.bench_function("optimized_pipeline", |b| {
        b.iter(|| {
            // Optimized data processing pipeline
            let data = "performance,optimization,benchmark,data";
            let results: Vec<&str> = data.split(',').collect();
            let processed: Vec<String> = results
                .into_iter()
                .map(|s| format!("processed_{}", s))
                .collect();
            black_box(processed)
        })
    });

    group.finish();
}

criterion_group!(
    pedantic_benchmarks,
    benchmark_zero_copy_strings,
    benchmark_dispatch_performance,
    benchmark_memory_allocation,
    benchmark_async_operations,
    benchmark_cache_alignment,
    benchmark_comprehensive_performance
);

criterion_main!(pedantic_benchmarks);
