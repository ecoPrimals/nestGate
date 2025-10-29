//! NestGate Operations Performance Benchmarks
//!
//! Performance benchmarks for core NestGate operations using stable Criterion framework

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

// Use the unified benchmark configuration system
use nestgate_core::config::unified::{
    BenchmarkMockConfiguration, BenchmarkMockService, UnifiedBenchmarkConfig,
};

fn bench_service_creation(c: &mut Criterion) {
    c.bench_function("service_creation", |b| {
        b.iter(|| {
            let services: Vec<_> = (0..100)
                .map(|i| BenchmarkMockService {
                    id: format!("service_{i}"),
                    name: format!("NestGate Service {i}"),
                    ..BenchmarkMockService::default()
                })
                .collect();
            // Actually use all fields to eliminate dead code
            let total_hash: u64 = services
                .iter()
                .map(|s| {
                    let id_hash = s.id.len() as u64;
                    let name_hash = s.name.len() as u64 * 2;
                    let version_hash = s.version.len() as u64 * 3;
                    let endpoints_hash = s.endpoints.len() as u64 * 4;
                    let metadata_hash = s.metadata.len() as u64 * 5;
                    id_hash + name_hash + version_hash + endpoints_hash + metadata_hash
                })
                .sum();
            black_box(total_hash);
        })
    });
}

fn bench_config_serialization(c: &mut Criterion) {
    c.bench_function("config_serialization", |b| {
        b.iter(|| {
            let configs: Vec<_> = (0..100)
                .map(|i| BenchmarkMockConfiguration {
                    service_name: format!("service_{i}"),
                    port: 8080 + i as u16,
                    ..BenchmarkMockConfiguration::default()
                })
                .collect();
            // Actually use all fields to eliminate dead code
            let total_score: u64 = configs
                .iter()
                .map(|c| {
                    let name_score = c.service_name.len() as u64;
                    let url_score = c.database_url.len() as u64 * 2;
                    let port_score = c.port as u64;
                    let debug_score = if c.debug_mode { 1000 } else { 0 };
                    let features_score = c.features.len() as u64 * 10;
                    let env_score = c.environment.len() as u64 * 3;
                    name_score + url_score + port_score + debug_score + features_score + env_score
                })
                .sum();
            black_box(total_score);
        })
    });
}

fn bench_pool_management(c: &mut Criterion) {
    c.bench_function("pool_management", |b| {
        b.iter(|| {
            // Simulate pool status checking and utilization calculation
            let pools: Vec<_> = (0..50).map(|i| format!("pool_{i}")).collect();

            let utilization: f64 = (0..50)
                .map(|i| (i as f64 * 0.02) % 1.0) // Mock utilization percentages
                .sum::<f64>()
                / 50.0;

            black_box((pools, utilization));
        })
    });
}

fn bench_dataset_operations(c: &mut Criterion) {
    c.bench_function("dataset_operations", |b| {
        b.iter(|| {
            // Simulate dataset management operations
            let datasets: Vec<_> = (0..100).map(|i| format!("dataset_{i}")).collect();

            let pool_usage: HashMap<String, f64> = (0..10)
                .map(|i| (format!("pool_{i}"), (i as f64 * 10.0) % 100.0))
                .collect();

            black_box((datasets, pool_usage));
        })
    });
}

fn bench_config_parsing(c: &mut Criterion) {
    let configs: Vec<MockConfiguration> = (0..100)
        .map(|i| MockConfiguration {
            service_name: format!("service_{i}"),
            database_url: "postgresql://localhost/nestgate".to_string(),
            port: 8080 + i as u16,
            debug_mode: false,
            features: vec!["api".to_string(), "web".to_string()],
            environment: "development".to_string(),
        })
        .collect();

    c.bench_function("config_parsing", |b| {
        b.iter(|| {
            let serialized: Vec<_> = configs
                .iter()
                .map(|config| format!("{:?}", black_box(config)))
                .collect();
            black_box(serialized);
        })
    });
}

fn bench_uuid_generation(c: &mut Criterion) {
    c.bench_function("uuid_generation", |b| {
        b.iter(|| {
            let uuids: Vec<_> = (0..1000).map(|_| Uuid::new_v4()).collect();

            // Clone to avoid borrow checker issues
            let unique_uuids: std::collections::HashSet<_> = uuids.iter().cloned().collect();
            black_box((uuids.len(), unique_uuids.len()));
        })
    });
}

fn bench_data_processing(c: &mut Criterion) {
    c.bench_function("data_processing", |b| {
        b.iter(|| {
            // Simulate processing data chunks with checksum calculation
            let data_chunks: Vec<Vec<u8>> = (0..100)
                .map(|i| format!("data_chunk_{i}").into_bytes())
                .collect();

            let checksums: Vec<u32> = data_chunks
                .iter()
                .map(|chunk| chunk.iter().map(|&b| b as u32).sum::<u32>())
                .collect();

            black_box((data_chunks, checksums));
        })
    });
}

fn bench_concurrent_operations(c: &mut Criterion) {
    let shared_data = Arc::new(Mutex::new(HashMap::<String, i32>::new()));

    c.bench_function("concurrent_operations", |b| {
        b.iter(|| {
            let handles: Vec<_> = (0..10)
                .map(|thread_id| {
                    let data = Arc::clone(&shared_data);
                    std::thread::spawn(
                        move || -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
                            for i in 0..10 {
                                let key = format!("key_{thread_id}_{i}");
                                let value = thread_id * 10 + i;

                                {
                                    let mut map =
                                        nestgate_core::safe_operations::safe_mutex_lock(&data)?;
                                    map.insert(key.clone(), value);
                                }

                                {
                                    let map =
                                        nestgate_core::safe_operations::safe_mutex_lock(&data)?;
                                    black_box(map.get(&key).copied());
                                }
                            }
                            Ok(())
                        },
                    )
                })
                .collect();

            for handle in handles {
                handle.join().unwrap();
            }
        })
    });
}

fn bench_memory_allocation_patterns(c: &mut Criterion) {
    c.bench_function("memory_allocation_patterns", |b| {
        b.iter(|| {
            // Test different allocation patterns

            // String allocations
            let strings: Vec<String> = (0..1000).map(|i| format!("string_{i}")).collect();

            // Vector allocations
            let vectors: Vec<Vec<i32>> = (0..100).map(|i| (0..i).collect()).collect();

            // HashMap allocations
            let maps: Vec<HashMap<i32, String>> = (0..50)
                .map(|i| (0..i).map(|j| (j, format!("value_{j}"))).collect())
                .collect();

            black_box((strings, vectors, maps));
        })
    });
}

fn bench_string_operations(c: &mut Criterion) {
    c.bench_function("string_operations", |b| {
        b.iter(|| {
            // Various string operations common in NestGate
            let base_strings: Vec<_> = (0..100).map(|i| format!("base_string_{i}")).collect();

            let concatenated: Vec<_> = base_strings
                .iter()
                .map(|s| format!("{s}_processed"))
                .collect();

            // Split and count instead of storing the split results to avoid borrowing issues
            let split_count: usize = concatenated.iter().map(|s| s.split('_').count()).sum();

            black_box((base_strings.len(), concatenated.len(), split_count));
        })
    });
}

criterion_group!(
    operations_benches,
    bench_service_creation,
    bench_config_serialization,
    bench_pool_management,
    bench_dataset_operations,
    bench_config_parsing,
    bench_uuid_generation,
    bench_data_processing,
    bench_concurrent_operations,
    bench_memory_allocation_patterns,
    bench_string_operations
);
criterion_main!(operations_benches);
