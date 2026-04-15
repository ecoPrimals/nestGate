// SPDX-License-Identifier: AGPL-3.0-or-later
#![expect(
    missing_docs,
    clippy::all,
    clippy::pedantic,
    clippy::restriction,
    clippy::cargo
)]

use criterion::{BenchmarkId, Criterion, Throughput, black_box, criterion_group, criterion_main};
use std::sync::Arc;
use std::time::Duration;
use tokio::runtime::Runtime;
use tokio::sync::Semaphore;
use tokio::time::{Instant, sleep};

// Mock API endpoints for load testing
const API_ENDPOINTS: &[&str] = &[
    "/health",
    "/api/v1/storage/info",
    "/api/v1/zfs/pools",
    "/api/v1/storage/capacity",
    "/metrics",
];

/// Simulates a production HTTP request (latency model only; no failure path).
async fn simulate_api_request(endpoint: &str) -> Duration {
    let start = Instant::now();

    // Simulate network latency (1-5ms)
    let latency = Duration::from_micros(1000 + (endpoint.len() * 100) as u64);
    sleep(latency).await;

    // Simulate processing time based on endpoint complexity
    let processing_time = match endpoint {
        "/health" => Duration::from_micros(50),
        "/metrics" => Duration::from_micros(200),
        "/api/v1/storage/info" => Duration::from_micros(500),
        "/api/v1/zfs/pools" => Duration::from_millis(2),
        "/api/v1/storage/capacity" => Duration::from_millis(1),
        _ => Duration::from_micros(100),
    };
    sleep(processing_time).await;

    start.elapsed()
}

/// Production load test - concurrent requests
async fn concurrent_api_load_test(concurrent_requests: usize, total_requests: usize) -> Duration {
    let semaphore = Arc::new(Semaphore::new(concurrent_requests));
    let mut handles = Vec::new();
    let start_time = Instant::now();

    for i in 0..total_requests {
        let endpoint = API_ENDPOINTS[i % API_ENDPOINTS.len()];
        let semaphore = semaphore.clone();

        let handle = tokio::spawn(async move {
            let _permit = semaphore.acquire().await.unwrap();
            simulate_api_request(endpoint).await
        });

        handles.push(handle);
    }

    // Wait for all requests to complete
    for handle in handles {
        let _ = handle.await;
    }

    start_time.elapsed()
}

/// WebSocket connection simulation
async fn simulate_websocket_connections(
    concurrent_connections: usize,
    duration_seconds: u64,
) -> (usize, Duration) {
    let mut handles = Vec::new();
    let start_time = Instant::now();

    for _ in 0..concurrent_connections {
        let handle = tokio::spawn(async move {
            let connection_start = Instant::now();

            // Simulate WebSocket handshake
            sleep(Duration::from_millis(10)).await;

            // Simulate periodic messages during connection lifetime
            let duration = Duration::from_secs(duration_seconds);
            let mut messages_sent = 0;

            while connection_start.elapsed() < duration {
                // Simulate sending a message every 100ms
                sleep(Duration::from_millis(100)).await;
                messages_sent += 1;

                // Simulate occasional processing delay
                if messages_sent % 10 == 0 {
                    sleep(Duration::from_millis(5)).await;
                }
            }

            messages_sent
        });

        handles.push(handle);
    }

    let mut total_messages = 0;
    for handle in handles {
        if let Ok(messages) = handle.await {
            total_messages += messages;
        }
    }

    (total_messages, start_time.elapsed())
}

/// Memory-intensive operations simulation
fn simulate_memory_operations(operation_count: usize) -> Duration {
    let start = Instant::now();

    // Simulate large data processing
    let mut data_sets: Vec<Vec<u8>> = Vec::with_capacity(operation_count);

    for i in 0..operation_count {
        // Simulate variable-sized data processing (1KB to 100KB)
        let size = 1024 + (i * 100) % 100000;
        let mut data = vec![0u8; size];

        // Simulate data manipulation
        for (j, byte) in data.iter_mut().enumerate() {
            *byte = ((i + j) % 256) as u8;
        }

        // Simulate compression/decompression simulation
        let checksum: u32 = data.iter().map(|&x| x as u32).sum();

        // Keep some data in memory (simulate caching)
        if i % 10 == 0 {
            data_sets.push(data);
        }

        black_box(checksum);
    }

    // Simulate cleanup
    for data in &mut data_sets {
        data.clear();
    }

    start.elapsed()
}

/// Storage I/O simulation
async fn simulate_storage_operations(operation_count: usize) -> Duration {
    let start = Instant::now();

    for i in 0..operation_count {
        // Simulate disk I/O latency based on operation type
        let io_time = match i % 4 {
            0 => Duration::from_micros(100), // Fast cache hit
            1 => Duration::from_millis(1),   // SSD read
            2 => Duration::from_millis(5),   // HDD read
            3 => Duration::from_millis(10),  // Network storage
            _ => Duration::from_micros(500),
        };

        sleep(io_time).await;

        // Simulate data processing after I/O
        black_box(i * i);
    }

    start.elapsed()
}

fn bench_api_load_tests(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    let mut group = c.benchmark_group("API Load Testing");
    group.measurement_time(Duration::from_secs(15));

    // Test various concurrency levels
    for concurrent_requests in [1, 10, 50, 100].iter() {
        group.throughput(Throughput::Elements(*concurrent_requests as u64));
        group.bench_with_input(
            BenchmarkId::new("concurrent_requests", concurrent_requests),
            concurrent_requests,
            |b, &concurrent_requests| {
                b.iter(|| {
                    rt.block_on(concurrent_api_load_test(
                        concurrent_requests,
                        concurrent_requests * 10,
                    ))
                })
            },
        );
    }

    group.finish();
}

fn bench_websocket_performance(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    let mut group = c.benchmark_group("WebSocket Performance");
    group.measurement_time(Duration::from_secs(10));

    for connections in [10, 25, 50, 100].iter() {
        group.throughput(Throughput::Elements(*connections as u64));
        group.bench_with_input(
            BenchmarkId::new("concurrent_connections", connections),
            connections,
            |b, &connections| {
                b.iter(|| rt.block_on(simulate_websocket_connections(connections, 1)));
            },
        );
    }

    group.finish();
}

fn bench_memory_performance(c: &mut Criterion) {
    let mut group = c.benchmark_group("Memory Performance");
    group.measurement_time(Duration::from_secs(15));

    for operations in [100, 500, 1000, 2000].iter() {
        group.throughput(Throughput::Elements(*operations as u64));
        group.bench_with_input(
            BenchmarkId::new("memory_operations", operations),
            operations,
            |b, &operations| {
                b.iter(|| simulate_memory_operations(black_box(operations)));
            },
        );
    }

    group.finish();
}

fn bench_storage_performance(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    let mut group = c.benchmark_group("Storage I/O Performance");
    group.measurement_time(Duration::from_secs(10));

    for operations in [25, 50, 100, 200].iter() {
        group.throughput(Throughput::Elements(*operations as u64));
        group.bench_with_input(
            BenchmarkId::new("storage_operations", operations),
            operations,
            |b, &operations| {
                b.iter(|| rt.block_on(simulate_storage_operations(black_box(operations))));
            },
        );
    }

    group.finish();
}

fn bench_mixed_production_workload(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    c.bench_function("production_mixed_workload", |b| {
        b.iter(|| {
            rt.block_on(async {
                // Simulate realistic production load
                let api_task = tokio::spawn(concurrent_api_load_test(25, 100));
                let websocket_task = tokio::spawn(simulate_websocket_connections(10, 1));
                let storage_task = tokio::spawn(simulate_storage_operations(50));

                let (api_result, websocket_result, storage_result) =
                    tokio::join!(api_task, websocket_task, storage_task);

                // Combine results for validation
                let total_time =
                    api_result.unwrap() + websocket_result.unwrap().1 + storage_result.unwrap();

                black_box(total_time)
            })
        })
    });
}

criterion_group!(
    benches,
    bench_api_load_tests,
    bench_websocket_performance,
    bench_memory_performance,
    bench_storage_performance,
    bench_mixed_production_workload
);

criterion_main!(benches);
