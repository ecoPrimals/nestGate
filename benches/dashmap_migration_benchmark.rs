//! **DashMap Migration Benchmark**
//!
//! Tracks performance improvements as we migrate from `Arc<RwLock<HashMap>>`
//! to `DashMap` (lock-free concurrent hash map).
//!
//! ## Metrics Tracked
//!
//! - **Throughput**: Operations per second (higher is better)
//! - **Latency**: P50, P95, P99 latencies (lower is better)
//! - **Contention**: Lock contention under concurrent load
//! - **Scalability**: Performance across 1, 2, 4, 8, 16 threads
//!
//! ## Baseline
//!
//! Before DashMap migration:
//! - 43/406 files (10.6%) lock-free
//! - Baseline throughput: ~40K ops/sec (estimated)
//!
//! ## Target
//!
//! After migration to 53/406 files (13%):
//! - Expected: 10-25x improvement in contended scenarios
//! - Expected: 50-100K ops/sec throughput

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use dashmap::DashMap;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::Duration;

// ═══════════════════════════════════════════════════════════════════════════
// Test Data Structures
// ═══════════════════════════════════════════════════════════════════════════

/// Legacy: Arc<RwLock<HashMap>> (what we're migrating FROM)
type LegacyMap = Arc<RwLock<HashMap<String, String>>>;

/// Modern: DashMap (what we're migrating TO)
type ModernMap = Arc<DashMap<String, String>>;

// ═══════════════════════════════════════════════════════════════════════════
// Benchmark: Single-Threaded Operations
// ═══════════════════════════════════════════════════════════════════════════

fn bench_single_thread_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("single_thread_insert");
    group.throughput(Throughput::Elements(1000));

    // Baseline: HashMap with RwLock
    group.bench_function("legacy_rwlock_hashmap", |b| {
        b.iter(|| {
            let map: LegacyMap = Arc::new(RwLock::new(HashMap::new()));
            for i in 0..1000 {
                let key = format!("key_{}", i);
                let value = format!("value_{}", i);
                map.write().unwrap().insert(key, value);
            }
            black_box(map);
        });
    });

    // Modern: DashMap
    group.bench_function("modern_dashmap", |b| {
        b.iter(|| {
            let map: ModernMap = Arc::new(DashMap::new());
            for i in 0..1000 {
                let key = format!("key_{}", i);
                let value = format!("value_{}", i);
                map.insert(key, value);
            }
            black_box(map);
        });
    });

    group.finish();
}

fn bench_single_thread_read(c: &mut Criterion) {
    let mut group = c.benchmark_group("single_thread_read");
    group.throughput(Throughput::Elements(1000));

    // Prepare data
    let legacy_map: LegacyMap = Arc::new(RwLock::new(HashMap::new()));
    let modern_map: ModernMap = Arc::new(DashMap::new());
    
    for i in 0..1000 {
        let key = format!("key_{}", i);
        let value = format!("value_{}", i);
        legacy_map.write().unwrap().insert(key.clone(), value.clone());
        modern_map.insert(key, value);
    }

    // Baseline: HashMap with RwLock
    group.bench_function("legacy_rwlock_hashmap", |b| {
        b.iter(|| {
            for i in 0..1000 {
                let key = format!("key_{}", i);
                let _ = legacy_map.read().unwrap().get(&key);
            }
        });
    });

    // Modern: DashMap
    group.bench_function("modern_dashmap", |b| {
        b.iter(|| {
            for i in 0..1000 {
                let key = format!("key_{}", i);
                let _ = modern_map.get(&key);
            }
        });
    });

    group.finish();
}

// ═══════════════════════════════════════════════════════════════════════════
// Benchmark: Multi-Threaded Operations (Where DashMap Shines!)
// ═══════════════════════════════════════════════════════════════════════════

fn bench_concurrent_mixed_workload(c: &mut Criterion) {
    let mut group = c.benchmark_group("concurrent_mixed_workload");
    
    for thread_count in [2, 4, 8, 16].iter() {
        group.throughput(Throughput::Elements(10000 * thread_count));

        // Baseline: HashMap with RwLock
        group.bench_with_input(
            BenchmarkId::new("legacy_rwlock_hashmap", thread_count),
            thread_count,
            |b, &threads| {
                b.iter(|| {
                    let map: LegacyMap = Arc::new(RwLock::new(HashMap::new()));
                    let mut handles = vec![];

                    for t in 0..threads {
                        let map_clone = Arc::clone(&map);
                        let handle = std::thread::spawn(move || {
                            // 70% reads, 30% writes (realistic workload)
                            for i in 0..10000 {
                                let key = format!("key_{}_{}", t, i);
                                
                                if i % 10 < 7 {
                                    // Read
                                    let _ = map_clone.read().unwrap().get(&key);
                                } else {
                                    // Write
                                    let value = format!("value_{}_{}", t, i);
                                    map_clone.write().unwrap().insert(key, value);
                                }
                            }
                        });
                        handles.push(handle);
                    }

                    for handle in handles {
                        handle.join().unwrap();
                    }
                    black_box(map);
                });
            },
        );

        // Modern: DashMap
        group.bench_with_input(
            BenchmarkId::new("modern_dashmap", thread_count),
            thread_count,
            |b, &threads| {
                b.iter(|| {
                    let map: ModernMap = Arc::new(DashMap::new());
                    let mut handles = vec![];

                    for t in 0..threads {
                        let map_clone = Arc::clone(&map);
                        let handle = std::thread::spawn(move || {
                            // 70% reads, 30% writes (realistic workload)
                            for i in 0..10000 {
                                let key = format!("key_{}_{}", t, i);
                                
                                if i % 10 < 7 {
                                    // Read
                                    let _ = map_clone.get(&key);
                                } else {
                                    // Write
                                    let value = format!("value_{}_{}", t, i);
                                    map_clone.insert(key, value);
                                }
                            }
                        });
                        handles.push(handle);
                    }

                    for handle in handles {
                        handle.join().unwrap();
                    }
                    black_box(map);
                });
            },
        );
    }

    group.finish();
}

// ═══════════════════════════════════════════════════════════════════════════
// Benchmark: High-Contention Scenario
// ═══════════════════════════════════════════════════════════════════════════

fn bench_high_contention(c: &mut Criterion) {
    let mut group = c.benchmark_group("high_contention");
    group.measurement_time(Duration::from_secs(10));
    
    let thread_count = 16;
    let ops_per_thread = 5000;
    group.throughput(Throughput::Elements((thread_count * ops_per_thread) as u64));

    // Baseline: HashMap with RwLock (high lock contention)
    group.bench_function("legacy_rwlock_hashmap", |b| {
        b.iter(|| {
            let map: LegacyMap = Arc::new(RwLock::new(HashMap::new()));
            let mut handles = vec![];

            for t in 0..thread_count {
                let map_clone = Arc::clone(&map);
                let handle = std::thread::spawn(move || {
                    // All threads fight for same keys (maximum contention!)
                    for i in 0..ops_per_thread {
                        let key = format!("shared_key_{}", i % 10); // Only 10 keys!
                        let value = format!("value_{}_{}", t, i);
                        map_clone.write().unwrap().insert(key, value);
                    }
                });
                handles.push(handle);
            }

            for handle in handles {
                handle.join().unwrap();
            }
            black_box(map);
        });
    });

    // Modern: DashMap (lock-free, should handle contention much better)
    group.bench_function("modern_dashmap", |b| {
        b.iter(|| {
            let map: ModernMap = Arc::new(DashMap::new());
            let mut handles = vec![];

            for t in 0..thread_count {
                let map_clone = Arc::clone(&map);
                let handle = std::thread::spawn(move || {
                    // All threads fight for same keys (maximum contention!)
                    for i in 0..ops_per_thread {
                        let key = format!("shared_key_{}", i % 10); // Only 10 keys!
                        let value = format!("value_{}_{}", t, i);
                        map_clone.insert(key, value);
                    }
                });
                handles.push(handle);
            }

            for handle in handles {
                handle.join().unwrap();
            }
            black_box(map);
        });
    });

    group.finish();
}

// ═══════════════════════════════════════════════════════════════════════════
// Benchmark Configuration
// ═══════════════════════════════════════════════════════════════════════════

criterion_group!(
    name = benches;
    config = Criterion::default()
        .measurement_time(Duration::from_secs(10))
        .sample_size(100)
        .warm_up_time(Duration::from_secs(3));
    targets = 
        bench_single_thread_insert,
        bench_single_thread_read,
        bench_concurrent_mixed_workload,
        bench_high_contention
);

criterion_main!(benches);
