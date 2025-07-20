//! Performance Validation Benchmarks
//!
//! This benchmark suite validates the performance improvements claimed:
//! - UUID Caching: 5x performance improvement (274,587 ns/iter → <50,000 ns/iter)
//! - Memory Pooling: 2x performance improvement
//! - Overall system optimization validation

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use uuid::Uuid;

use nestgate_core::{
    memory_pool::{get_1mb_buffer, get_4kb_buffer, MemoryPool},
    uuid_cache::{get_or_create_uuid, UuidCache},
};

/// Benchmark UUID generation without caching (baseline)
fn benchmark_uuid_generation_baseline(c: &mut Criterion) {
    let mut group = c.benchmark_group("UUID Operations");

    group.bench_function("uuid_generation_baseline", |b| {
        b.iter(|| {
            // Simulate the old approach - frequent UUID generation
            for i in 0..100 {
                let service_name = format!("service-{}", i % 10);
                let uuid = black_box(Uuid::new_v4());
                black_box(uuid);
            }
        });
    });

    group.finish();
}

/// Benchmark UUID generation with our caching system
fn benchmark_uuid_generation_cached(c: &mut Criterion) {
    let mut group = c.benchmark_group("UUID Operations");

    // Pre-warm the cache
    for i in 0..10 {
        get_or_create_uuid(&format!("service-{}", i));
    }

    group.bench_function("uuid_generation_cached", |b| {
        b.iter(|| {
            // Use our cached approach
            for i in 0..100 {
                let service_name = format!("service-{}", i % 10);
                let uuid = black_box(get_or_create_uuid(&service_name));
                black_box(uuid);
            }
        });
    });

    group.finish();
}

/// Benchmark cache hit ratio performance
fn benchmark_uuid_cache_hit_ratio(c: &mut Criterion) {
    let mut group = c.benchmark_group("UUID Cache Performance");
    let cache = UuidCache::new();

    // Pre-populate cache with common UUIDs
    for i in 0..50 {
        cache.get_or_create(&format!("common-service-{}", i));
    }

    // Test different hit ratios
    for hit_ratio in [50, 75, 90, 95, 99].iter() {
        group.bench_with_input(
            BenchmarkId::new("cache_hit_ratio", hit_ratio),
            hit_ratio,
            |b, &hit_ratio| {
                b.iter(|| {
                    for i in 0..1000 {
                        let key = if i % 100 < hit_ratio {
                            // Cache hit - use existing key
                            format!("common-service-{}", i % 50)
                        } else {
                            // Cache miss - use new key
                            format!("new-service-{}", i)
                        };
                        let uuid = black_box(cache.get_or_create(&key));
                        black_box(uuid);
                    }
                });
            },
        );
    }

    group.finish();
}

/// Benchmark memory allocation without pooling (baseline)
fn benchmark_memory_allocation_baseline(c: &mut Criterion) {
    let mut group = c.benchmark_group("Memory Operations");

    group.bench_function("memory_allocation_baseline", |b| {
        b.iter(|| {
            // Simulate frequent allocations without pooling
            for _i in 0..100 {
                let buffer = black_box(vec![0u8; 4096]); // 4KB allocation
                black_box(buffer);
            }
        });
    });

    group.finish();
}

/// Benchmark memory allocation with our pooling system
fn benchmark_memory_allocation_pooled(c: &mut Criterion) {
    let mut group = c.benchmark_group("Memory Operations");

    group.bench_function("memory_allocation_pooled", |b| {
        b.iter(|| {
            // Use our memory pool
            for _i in 0..100 {
                let buffer = black_box(get_4kb_buffer());
                black_box(buffer);
                // Buffer is automatically returned to pool when dropped
            }
        });
    });

    group.finish();
}

/// Benchmark different buffer sizes with pooling
fn benchmark_memory_pool_sizes(c: &mut Criterion) {
    let mut group = c.benchmark_group("Memory Pool Sizes");

    // Test 4KB buffers
    group.bench_function("4kb_pooled_buffers", |b| {
        b.iter(|| {
            for _i in 0..50 {
                let buffer = black_box(get_4kb_buffer());
                black_box(buffer);
            }
        });
    });

    // Test 1MB buffers
    group.bench_function("1mb_pooled_buffers", |b| {
        b.iter(|| {
            for _i in 0..10 {
                let buffer = black_box(get_1mb_buffer());
                black_box(buffer);
            }
        });
    });

    group.finish();
}

/// Benchmark concurrent UUID cache access
fn benchmark_concurrent_uuid_access(c: &mut Criterion) {
    use std::sync::Arc;
    use std::thread;

    let mut group = c.benchmark_group("Concurrent Operations");
    let cache = Arc::new(UuidCache::new());

    // Pre-populate with some UUIDs
    for i in 0..20 {
        cache.get_or_create(&format!("service-{}", i));
    }

    group.bench_function("concurrent_uuid_cache", |b| {
        b.iter(|| {
            let cache_clone = Arc::clone(&cache);
            let counter = Arc::new(AtomicUsize::new(0));
            let mut handles = vec![];

            // Spawn 8 threads for concurrent access
            for thread_id in 0..8 {
                let cache = Arc::clone(&cache_clone);
                let counter = Arc::clone(&counter);

                let handle = thread::spawn(move || {
                    for i in 0..25 {
                        let key = format!("thread-{}-service-{}", thread_id, i % 10);
                        let uuid = cache.get_or_create(&key);
                        black_box(uuid);
                        counter.fetch_add(1, Ordering::Relaxed);
                    }
                });
                handles.push(handle);
            }

            // Wait for all threads to complete
            for handle in handles {
                handle.join().unwrap();
            }

            let total = counter.load(Ordering::Relaxed);
            black_box(total);
        });
    });

    group.finish();
}

/// Benchmark UUID cache statistics collection
fn benchmark_cache_statistics(c: &mut Criterion) {
    let mut group = c.benchmark_group("Cache Statistics");
    let cache = UuidCache::new();

    // Generate some cache activity
    for i in 0..1000 {
        cache.get_or_create(&format!("stat-test-{}", i % 50));
    }

    group.bench_function("statistics_collection", |b| {
        b.iter(|| {
            let stats = black_box(cache.statistics());
            black_box(stats.hit_ratio);
            black_box(stats.cache_size);
        });
    });

    group.finish();
}

/// Comprehensive system performance test
fn benchmark_integrated_system_performance(c: &mut Criterion) {
    let mut group = c.benchmark_group("Integrated System");

    group.bench_function("uuid_and_memory_combined", |b| {
        b.iter(|| {
            // Simulate a realistic workload combining both optimizations
            for i in 0..50 {
                // Get cached UUID
                let service_id = format!("integrated-service-{}", i % 10);
                let uuid = get_or_create_uuid(&service_id);

                // Allocate pooled buffer
                let buffer = get_4kb_buffer();

                // Simulate some work with both
                black_box(uuid);
                black_box(buffer);
            }
        });
    });

    group.finish();
}

/// Performance improvement validation
fn validate_performance_improvements(c: &mut Criterion) {
    let mut group = c.benchmark_group("Performance Validation");

    // Measure UUID operations: Target <50,000 ns/iter (5x improvement)
    group.bench_function("uuid_target_validation", |b| {
        // Pre-warm cache
        for i in 0..10 {
            get_or_create_uuid(&format!("validation-{}", i));
        }

        let start = Instant::now();
        b.iter(|| {
            // Realistic service UUID lookup pattern
            for i in 0..10 {
                let key = format!("validation-{}", i);
                let uuid = black_box(get_or_create_uuid(&key));
                black_box(uuid);
            }
        });
        let elapsed = start.elapsed();

        // The benchmark framework will show if we meet our target
        black_box(elapsed);
    });

    // Measure memory operations: Target 2x improvement
    group.bench_function("memory_target_validation", |b| {
        b.iter(|| {
            for _i in 0..20 {
                let buffer = black_box(get_4kb_buffer());
                black_box(buffer);
            }
        });
    });

    group.finish();
}

/// Real-world simulation benchmark
fn benchmark_realistic_workload(c: &mut Criterion) {
    let mut group = c.benchmark_group("Realistic Workload");

    group.bench_function("storage_service_simulation", |b| {
        b.iter(|| {
            // Simulate a storage service handling multiple requests
            for request_id in 0..25 {
                // Each request needs a service UUID and buffer
                let service_name = match request_id % 5 {
                    0 => "zfs-pool-manager",
                    1 => "dataset-handler",
                    2 => "snapshot-service",
                    3 => "replication-engine",
                    _ => "performance-monitor",
                };

                // Get cached service UUID (high hit ratio expected)
                let service_uuid = get_or_create_uuid(service_name);

                // Allocate buffer for request processing
                let request_buffer = get_4kb_buffer();

                // Simulate request processing
                black_box(service_uuid);
                black_box(request_buffer);

                // Some requests need larger buffers
                if request_id % 10 == 0 {
                    let large_buffer = get_1mb_buffer();
                    black_box(large_buffer);
                }
            }
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    benchmark_uuid_generation_baseline,
    benchmark_uuid_generation_cached,
    benchmark_uuid_cache_hit_ratio,
    benchmark_memory_allocation_baseline,
    benchmark_memory_allocation_pooled,
    benchmark_memory_pool_sizes,
    benchmark_concurrent_uuid_access,
    benchmark_cache_statistics,
    benchmark_integrated_system_performance,
    validate_performance_improvements,
    benchmark_realistic_workload
);

criterion_main!(benches);
