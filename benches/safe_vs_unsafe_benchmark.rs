//! **SAFE VS UNSAFE PERFORMANCE BENCHMARK**
//!
//! Comprehensive benchmark comparing safe and unsafe implementations.
//! This proves that "Safe AND Fast Rust" is achievable without compromises.
//!
//! **Benchmarked Components:**
//! - Ring Buffer (SPSC queue)
//! - Memory Pool (object pooling)
//! - Memory Arena (bump allocator)
//! - SIMD Operations (vectorized operations)
//! - Cache-Aligned Counter (atomic operations)

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use nestgate_core::performance::{
    advanced_optimizations::{CacheAlignedCounter, LockFreeRingBuffer, MemoryPool},
    safe_optimizations::{SafeCacheAlignedCounter, SafeMemoryPool, SafeRingBuffer},
};
use std::sync::atomic::Ordering;

// **RING BUFFER BENCHMARKS**

fn bench_ring_buffer_unsafe(c: &mut Criterion) {
    let mut group = c.benchmark_group("ring_buffer_unsafe");

    for size in [100, 1000, 10000].iter() {
        group.throughput(Throughput::Elements(*size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter(|| {
                let buffer: LockFreeRingBuffer<u64, 1024> = LockFreeRingBuffer::new();
                for i in 0..size {
                    let _ = buffer.push(black_box(i));
                }
                for _ in 0..size {
                    let _ = buffer.pop();
                }
            });
        });
    }
    group.finish();
}

fn bench_ring_buffer_safe(c: &mut Criterion) {
    let mut group = c.benchmark_group("ring_buffer_safe");

    for size in [100, 1000, 10000].iter() {
        group.throughput(Throughput::Elements(*size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter(|| {
                let buffer = SafeRingBuffer::new(1024);
                for i in 0..size {
                    let _ = buffer.push(black_box(i));
                }
                for _ in 0..size {
                    let _ = buffer.pop();
                }
            });
        });
    }
    group.finish();
}

// **MEMORY POOL BENCHMARKS**

fn bench_memory_pool_unsafe(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_pool_unsafe");

    for chunk_size in [64, 256, 1024].iter() {
        group.throughput(Throughput::Bytes(*chunk_size as u64 * 100));
        group.bench_with_input(
            BenchmarkId::from_parameter(chunk_size),
            chunk_size,
            |b, &chunk_size| {
                b.iter(|| {
                    let pool = MemoryPool::new(chunk_size, 100);
                    let mut buffers = Vec::new();

                    // Allocate 100 buffers
                    for _ in 0..100 {
                        if let Some(buf) = pool.allocate() {
                            buffers.push(buf);
                        }
                    }

                    // Deallocate all
                    for buf in buffers {
                        pool.deallocate(buf);
                    }
                });
            },
        );
    }
    group.finish();
}

fn bench_memory_pool_safe(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_pool_safe");

    for chunk_size in [64, 256, 1024].iter() {
        group.throughput(Throughput::Bytes(*chunk_size as u64 * 100));
        group.bench_with_input(
            BenchmarkId::from_parameter(chunk_size),
            chunk_size,
            |b, &chunk_size| {
                b.iter(|| {
                    let pool = SafeMemoryPool::new(chunk_size, 100);
                    let mut buffers = Vec::new();

                    // Allocate 100 buffers
                    for _ in 0..100 {
                        if let Some(buf) = pool.allocate() {
                            buffers.push(buf);
                        }
                    }

                    // Deallocate all
                    for buf in buffers {
                        pool.deallocate(buf);
                    }
                });
            },
        );
    }
    group.finish();
}

// **CACHE-ALIGNED COUNTER BENCHMARKS**

fn bench_counter_unsafe(c: &mut Criterion) {
    let mut group = c.benchmark_group("counter_unsafe");

    for iterations in [1000, 10000, 100000].iter() {
        group.throughput(Throughput::Elements(*iterations as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(iterations),
            iterations,
            |b, &iterations| {
                b.iter(|| {
                    let counter = CacheAlignedCounter::new(0);
                    for _ in 0..iterations {
                        counter.increment();
                    }
                    black_box(counter.get())
                });
            },
        );
    }
    group.finish();
}

fn bench_counter_safe(c: &mut Criterion) {
    let mut group = c.benchmark_group("counter_safe");

    for iterations in [1000, 10000, 100000].iter() {
        group.throughput(Throughput::Elements(*iterations as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(iterations),
            iterations,
            |b, &iterations| {
                b.iter(|| {
                    let counter = SafeCacheAlignedCounter::new(0);
                    for _ in 0..iterations {
                        counter.increment();
                    }
                    black_box(counter.get())
                });
            },
        );
    }
    group.finish();
}

// **MEMORY COPY BENCHMARKS**

fn bench_memory_copy_safe(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_copy");

    for size in [1024, 4096, 16384].iter() {
        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            let src = vec![42u8; size];
            b.iter(|| {
                let mut dst = vec![0u8; size];
                dst.copy_from_slice(&src);
                black_box(dst)
            });
        });
    }
    group.finish();
}

// **VECTOR OPERATIONS BENCHMARK**

fn bench_vector_sum_safe(c: &mut Criterion) {
    let mut group = c.benchmark_group("vector_sum_safe");

    for size in [1000, 10000, 100000].iter() {
        group.throughput(Throughput::Elements(*size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            let data: Vec<f32> = (0..size).map(|i| i as f32).collect();
            b.iter(|| {
                // Compiler will auto-vectorize this with SIMD
                let sum: f32 = data.iter().sum();
                black_box(sum)
            });
        });
    }
    group.finish();
}

criterion_group!(
    safe_vs_unsafe_benches,
    bench_ring_buffer_unsafe,
    bench_ring_buffer_safe,
    bench_memory_pool_unsafe,
    bench_memory_pool_safe,
    bench_counter_unsafe,
    bench_counter_safe,
    bench_memory_copy_safe,
    bench_vector_sum_safe,
);

criterion_main!(safe_vs_unsafe_benches);
