use nestgate_core::*;
use std::time::{Duration, Instant};
use tokio;

/// Performance benchmarking tests for NestGate
/// These tests verify that performance requirements are met

const PERFORMANCE_THRESHOLD_MS: u128 = 100; // 100ms max for operations
const THROUGHPUT_THRESHOLD: usize = 1000; // 1000 ops/sec minimum

#[tokio::test]
async fn benchmark_service_discovery() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("🏃 Benchmarking service discovery performance...");

    let registry = service_discovery::InMemoryServiceRegistry::new();
    let service_count = 100;

    // Benchmark service registration
    let start = Instant::now();
    for i in 0..service_count {
        let service = service_discovery::ServiceInfo {
            name: format!("service-{}", i),
            version: "1.0.0".to_string(),
            endpoints: vec![format!("http://localhost:{}", 8000 + i)],
            health_check_url: Some(format!("http://localhost:{}/health", 8000 + i)),
            metadata: std::collections::HashMap::new(),
        };
        registry.register_service(service).await?;
    }
    let registration_time = start.elapsed();

    // Benchmark service discovery
    let start = Instant::now();
    for i in 0..service_count {
        let _ = registry.discover_service(&format!("service-{}", i)).await?;
    }
    let discovery_time = start.elapsed();

    // Calculate performance metrics
    let reg_ops_per_sec = (service_count as f64 / registration_time.as_secs_f64()) as usize;
    let disc_ops_per_sec = (service_count as f64 / discovery_time.as_secs_f64()) as usize;

    println!("📊 Service Discovery Benchmark Results:");
    println!("   Registration: {} ops/sec", reg_ops_per_sec);
    println!("   Discovery: {} ops/sec", disc_ops_per_sec);
    println!("   Registration time: {:?}", registration_time);
    println!("   Discovery time: {:?}", discovery_time);

    // Assert performance requirements
    assert!(
        reg_ops_per_sec >= THROUGHPUT_THRESHOLD,
        "Registration throughput too low"
    );
    assert!(
        disc_ops_per_sec >= THROUGHPUT_THRESHOLD,
        "Discovery throughput too low"
    );

    println!("✅ Service discovery performance benchmark passed");
    Ok(())
}

#[tokio::test]
async fn benchmark_error_handling() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("🏃 Benchmarking error handling performance...");

    let iterations = 10000;

    // Benchmark error creation
    let start = Instant::now();
    for i in 0..iterations {
        let _err =
            error::NestGateError::internal_error(&format!("Test error {}", i), "benchmark_test");
    }
    let creation_time = start.elapsed();

    // Benchmark error formatting
    let test_error = error::NestGateError::internal_error("Benchmark error", "test");
    let start = Instant::now();
    for _ in 0..iterations {
        let _formatted = format!("{}", test_error);
    }
    let formatting_time = start.elapsed();

    let creation_ops_per_sec = (iterations as f64 / creation_time.as_secs_f64()) as usize;
    let formatting_ops_per_sec = (iterations as f64 / formatting_time.as_secs_f64()) as usize;

    println!("📊 Error Handling Benchmark Results:");
    println!("   Error creation: {} ops/sec", creation_ops_per_sec);
    println!("   Error formatting: {} ops/sec", formatting_ops_per_sec);

    assert!(
        creation_ops_per_sec >= THROUGHPUT_THRESHOLD * 10,
        "Error creation too slow"
    );
    assert!(
        formatting_ops_per_sec >= THROUGHPUT_THRESHOLD * 5,
        "Error formatting too slow"
    );

    println!("✅ Error handling performance benchmark passed");
    Ok(())
}

#[tokio::test]
async fn benchmark_zero_cost_abstractions() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("🏃 Benchmarking zero-cost abstractions...");

    let iterations = 100000;

    // Benchmark zero-cost config access
    let start = Instant::now();
    for _ in 0..iterations {
        let _optimized = zero_cost::ZeroCostConfig::ENABLE_SIMD;
        let _zero_copy = zero_cost::ZeroCostConfig::ENABLE_ZERO_COPY;
    }
    let config_time = start.elapsed();

    // Benchmark zero-cost type conversions
    let start = Instant::now();
    for i in 0..iterations {
        let state = if i % 2 == 0 {
            canonical_types::UnifiedServiceState::Running
        } else {
            canonical_types::UnifiedServiceState::Stopped
        };
        let _is_active = state.is_active();
    }
    let conversion_time = start.elapsed();

    let config_ops_per_sec = (iterations as f64 / config_time.as_secs_f64()) as usize;
    let conversion_ops_per_sec = (iterations as f64 / conversion_time.as_secs_f64()) as usize;

    println!("📊 Zero-Cost Abstractions Benchmark Results:");
    println!("   Config access: {} ops/sec", config_ops_per_sec);
    println!("   Type conversions: {} ops/sec", conversion_ops_per_sec);

    // Zero-cost abstractions should be extremely fast
    assert!(
        config_ops_per_sec >= THROUGHPUT_THRESHOLD * 100,
        "Config access not zero-cost"
    );
    assert!(
        conversion_ops_per_sec >= THROUGHPUT_THRESHOLD * 50,
        "Type conversions not zero-cost"
    );

    println!("✅ Zero-cost abstractions benchmark passed");
    Ok(())
}

#[tokio::test]
async fn benchmark_memory_operations() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("🏃 Benchmarking memory operations...");

    let iterations = 50000;
    let data_size = 1024; // 1KB chunks

    // Benchmark safe memory operations
    let start = Instant::now();
    for i in 0..iterations {
        let data = vec![i as u8; data_size];
        let _result = safe_operations::safe_memory_operation(data, "benchmark_test")?;
    }
    let memory_time = start.elapsed();

    let memory_ops_per_sec = (iterations as f64 / memory_time.as_secs_f64()) as usize;
    let throughput_mb_per_sec = (memory_ops_per_sec * data_size) / (1024 * 1024);

    println!("📊 Memory Operations Benchmark Results:");
    println!("   Memory operations: {} ops/sec", memory_ops_per_sec);
    println!("   Memory throughput: {} MB/sec", throughput_mb_per_sec);

    assert!(
        memory_ops_per_sec >= THROUGHPUT_THRESHOLD,
        "Memory operations too slow"
    );

    println!("✅ Memory operations benchmark passed");
    Ok(())
}

#[tokio::test]
async fn benchmark_concurrent_operations() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("🏃 Benchmarking concurrent operations...");

    let concurrent_tasks = 100;
    let operations_per_task = 100;

    let start = Instant::now();

    // Spawn concurrent tasks
    let mut handles = Vec::new();
    for task_id in 0..concurrent_tasks {
        let handle = tokio::spawn(async move {
            for i in 0..operations_per_task {
                // Simulate concurrent work
                let _result = safe_operations::safe_string_operation(
                    &format!("task-{}-op-{}", task_id, i),
                    "concurrent_benchmark",
                );
                tokio::task::yield_now().await;
            }
        });
        handles.push(handle);
    }

    // Wait for all tasks to complete
    for handle in handles {
        handle.await?;
    }

    let total_time = start.elapsed();
    let total_operations = concurrent_tasks * operations_per_task;
    let ops_per_sec = (total_operations as f64 / total_time.as_secs_f64()) as usize;

    println!("📊 Concurrent Operations Benchmark Results:");
    println!("   Total operations: {}", total_operations);
    println!("   Concurrent tasks: {}", concurrent_tasks);
    println!("   Operations per second: {}", ops_per_sec);
    println!("   Total time: {:?}", total_time);

    assert!(
        ops_per_sec >= THROUGHPUT_THRESHOLD * 5,
        "Concurrent operations too slow"
    );

    println!("✅ Concurrent operations benchmark passed");
    Ok(())
}

#[tokio::test]
async fn benchmark_full_system_load() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("🏃 Benchmarking full system under load...");

    // Initialize system components
    let config = config::canonical_master::NestGateNestGateCanonicalConfig::default();
    let registry = service_discovery::InMemoryServiceRegistry::new();
    let security_provider = security::SecurityProvider::new_test();
    let metrics = observability::PerformanceMetrics::new();

    let load_duration = Duration::from_secs(5);
    let concurrent_clients = 50;

    let start = Instant::now();
    let mut handles = Vec::new();

    // Simulate concurrent load
    for client_id in 0..concurrent_clients {
        let registry_clone = registry.clone();
        let security_clone = security_provider.clone();

        let handle = tokio::spawn(async move {
            let mut operations = 0;
            let client_start = Instant::now();

            while client_start.elapsed() < load_duration {
                // Register a service
                let service = service_discovery::ServiceInfo {
                    name: format!("load-test-service-{}-{}", client_id, operations),
                    version: "1.0.0".to_string(),
                    endpoints: vec![format!("http://localhost:{}", 10000 + client_id)],
                    health_check_url: None,
                    metadata: std::collections::HashMap::new(),
                };

                if let Ok(_) = registry_clone.register_service(service).await {
                    operations += 1;
                }

                // Generate and validate token
                if let Ok(token) =
                    security_clone.generate_test_token(&format!("user-{}", client_id))
                {
                    let _ = security_clone.validate_token(&token);
                }

                tokio::task::yield_now().await;
            }

            operations
        });

        handles.push(handle);
    }

    // Collect results
    let mut total_operations = 0;
    for handle in handles {
        total_operations += handle.await?;
    }

    let total_time = start.elapsed();
    let ops_per_sec = (total_operations as f64 / total_time.as_secs_f64()) as usize;

    println!("📊 Full System Load Benchmark Results:");
    println!("   Total operations: {}", total_operations);
    println!("   Concurrent clients: {}", concurrent_clients);
    println!("   Test duration: {:?}", total_time);
    println!("   Operations per second: {}", ops_per_sec);
    println!(
        "   Average ops per client: {}",
        total_operations / concurrent_clients
    );

    // System should handle reasonable load
    assert!(
        ops_per_sec >= THROUGHPUT_THRESHOLD,
        "System performance under load too low"
    );
    assert!(
        total_operations > concurrent_clients * 10,
        "Not enough operations completed"
    );

    println!("✅ Full system load benchmark passed");
    Ok(())
}
